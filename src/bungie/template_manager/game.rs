use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::mem::{size_of};
use std::path::{Path, PathBuf};
use anyhow::anyhow;
use binread::{BinRead, BinReaderExt, ReadOptions};
use log::{info};
use crate::bungie::template_manager::common::{InstanceFileId, level_info};
use crate::bungie::template_manager::instance_file::{InstanceDescriptor, InstanceFileHeader, NameDescriptor, TemplateDescriptor};

pub struct InstanceFileRef  {
    instanceFileRef: PathBuf,
    fileIndex: InstanceFileId,
}

pub struct InstanceFile;
pub struct PrivateData;

pub struct Game {
    dataFolderRef: PathBuf,
    validLevels: HashSet<u32>,
    instanceFileRefs: Vec<InstanceFileRef>,
    loadedInstanceFiles: Vec<InstanceFile>,
    dynamicInstanceFile: Option<Box<InstanceFile>>,
    privateData: Vec<PrivateData>,
}

impl Game {
    pub fn new(dataFolderRef: PathBuf) -> Self {
        Self {
            dataFolderRef,
            validLevels: Default::default(),
            instanceFileRefs: vec![],
            loadedInstanceFiles: vec![],
            dynamicInstanceFile: None,
            privateData: vec![],
        }
    }

    pub fn validate_level(&self, path: &Path, level: u32) -> anyhow::Result<()> {
        let mut file = BufReader::new(File::open(path).map_err(|e| anyhow!("level {}: Can't open file {:?}: {}", level, path, e))?);
        let header: InstanceFileHeader = file.read_ne().map_err(|e| anyhow!("level {}: Can't load header from file {:?}: {}", level, path, e))?;
        if header.num_template_descriptors == 0 {
            return Err(anyhow!("level {}: No templates in file {:?}", level, path))
        }
        let offset = size_of::<InstanceFileHeader>() + header.num_instance_descriptors as usize * size_of::<InstanceDescriptor>() +
            header.num_name_descriptors as usize * size_of::<NameDescriptor>();

        file.seek(SeekFrom::Start(offset as u64))?;
        let mut opt = ReadOptions::default();
        opt.count = Some(header.num_template_descriptors as usize);
        let descriptors = Vec::<TemplateDescriptor>::read_options(&mut file, &opt, ())?;
        for descriptor in descriptors {
            info!("found template {}", descriptor.tag)
        }
        Ok(())
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        let paths = std::fs::read_dir(&self.dataFolderRef)?;
        for path in paths.filter_map(|e| e.ok()).map(|e| e.path()) {
            if path
                .file_name()
                .and_then(|n| {
                    n.to_str()
                        .filter(|f| f.starts_with("level") && f.ends_with(".dat"))
                })
                .is_some()
            {
                match level_info(&path) {
                    Ok(info) => {
                        log::info!("fond new level: {:?}", info);
                        match self.validate_level(&path, info.levelNumber) {
                            Ok(_) => {
                                if info.is_final {
                                    self.validLevels.insert(info.levelNumber);
                                }
                                for instanceFileRef in &self.instanceFileRefs {
                                    if instanceFileRef.fileIndex == info.instanceFileID {
                                        return Err(anyhow!("conflicting file indices"));
                                    }
                                }
                                self.instanceFileRefs.push(InstanceFileRef {
                                    instanceFileRef: path,
                                    fileIndex: info.instanceFileID,
                                })
                            }
                            Err(_) => {
                                log::warn!("invalid level {:?}", path.file_name().unwrap())
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("invalid level file {:?}: {}", path, err)
                    }
                }
            }
        }
        Ok(())
    }
}
