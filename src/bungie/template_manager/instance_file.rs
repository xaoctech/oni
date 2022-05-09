
use binread::BinRead;
use bytemuck::{Zeroable, Pod};
use crate::bungie::template_manager::four_cc::FourCC;

#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
#[repr(transparent)]
pub struct DataPtr(u32);

// impl DataPtr {
//     pub fn get(self, inst: &InstanceFile, size: usize) -> &[u8] {
//         &inst.data_block()[(self.0 as usize)..(self.0 as usize + size)]
//     }
//
//     pub fn get_with_offset(self, inst: &InstanceFile, offset: isize, size: usize) -> &[u8] {
//         let offset = (self.0 as usize).wrapping_add(offset as usize);
//         &inst.data_block()[offset..(offset + size)]
//     }
// }
//
#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
#[repr(transparent)]
pub struct NamePtr(u32);
//
// impl NamePtr {
//     pub fn get(self, inst: &InstanceFile) -> anyhow::Result<&str> {
//         let s = &inst.name_block()[(self.0 as usize)..];
//         match memchr::memchr(0, s) {
//             Some(n) => Ok(from_utf8(&s[..n])?),
//             None => Err(anyhow::anyhow!("invalid name offset {}", self.0))
//         }
//     }
// }

bitflags::bitflags! {
    #[derive(BinRead)]
    pub struct DescriptorFlags: u32 {
		const None            = 0;
		const Unique          = (1 << 0);
		const PlaceHolder     = (1 << 1);
		const Duplicate       = (1 << 2);			// This instance does not point to its own data - it points to its duplicates
		const DuplicatedSrc   = (1 << 3);			// This instance is being used by duplicate instances as a source

		// These are not saved
		const Touched         = (1 << 20);
		const InBatchFile     = (1 << 21);
		const DeleteMe        = (1 << 22);

        const PersistentMask  = 0xFFFF;
    }
}
unsafe impl Pod for DescriptorFlags {}

unsafe impl Zeroable for DescriptorFlags {}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
pub struct InstanceDescriptor {
    template_ptr: u32,
    data_ptr: DataPtr,
    name_ptr: NamePtr,
    pub size: u32,
    // This is the total size including entire var array that is written to disk
    pub flags: DescriptorFlags,
}

impl InstanceDescriptor {
    pub fn template(&self) -> FourCC {
        FourCC(self.template_ptr)
    }

    // pub fn name<'inst>(&self, instance_file: &'inst InstanceFile) -> Option<&'inst str> {
    //     if !self.flags.contains(DescriptorFlags::Unique) {
    //         self.name_ptr.get(&instance_file).ok()
    //     } else {
    //         None
    //     }
    // }
    //
    // pub fn data<'inst, T: Pod>(&self, instance_file: &'inst InstanceFile) -> Option<&'inst T> {
    //     if !self.flags.contains(DescriptorFlags::PlaceHolder) {
    //         try_from_bytes(self.data_ptr.get(&instance_file, size_of::<T>())).ok()
    //     } else {
    //         None
    //     }
    // }
    //
    // pub fn file_id(&self, instance_file: &InstanceFile) -> Option<u32> {
    //     if !self.flags.contains(DescriptorFlags::PlaceHolder) {
    //         try_from_bytes(self.data_ptr.get_with_offset(&instance_file, -4, size_of::<u32>())).ok().cloned()
    //     } else {
    //         None
    //     }
    // }
    //
    // pub fn place_holder(&self, instance_file: &InstanceFile) -> u32 {
    //     (if !self.flags.contains(DescriptorFlags::PlaceHolder) {
    //         try_from_bytes(self.data_ptr.get_with_offset(&instance_file, -8, size_of::<u32>())).ok().cloned().unwrap()
    //     } else {
    //         self.data_ptr.0
    //     }) >> 8
    // }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
pub struct NameDescriptor {
    pub instance_desc_index: u32,
    pub name_ptr: NamePtr,
}

// impl NameDescriptor {
//     pub fn instance_desc<'inst>(&self, instance_file: &'inst InstanceFile) -> &'inst InstanceDescriptor {
//         &instance_file.instance_descriptors()[self.instance_desc_index as usize]
//     }
// }

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
pub struct TemplateDescriptor {
    pub checksum: u64,
    pub tag: FourCC,
    pub _num_used: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, BinRead)]
pub struct InstanceFileHeader {
    pub total_template_checksum: u64,

    pub version: u32,
    pub size_of_header: u16,
    pub size_of_instance_descriptor: u16,
    pub size_of_template_descriptor: u16,
    pub size_of_name_descriptor: u16,

    pub num_instance_descriptors: u32,
    pub num_name_descriptors: u32,
    pub num_template_descriptors: u32,

    pub data_block_offset: u32,
    pub data_block_length: u32,

    pub name_block_offset: u32,
    pub name_block_length: u32,

    pub pad2: [u32; 4],
}
