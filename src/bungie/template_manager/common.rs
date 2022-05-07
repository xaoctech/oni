use anyhow::anyhow;
use std::path::Path;

#[repr(transparent)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceFileId(pub u32);

impl InstanceFileId {
    pub fn new(level: u32, checksum: u32) -> Self {
        Self((level << 25) | ((checksum & 0xFFFFFF) << 1))
    }

    pub fn level(self) -> u32 {
        self.0 >> 25
    }

    pub fn checksum(self) -> u32 {
        self.0 & 0xFFFFFF
    }
}

impl std::fmt::Debug for InstanceFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:06x})", self.level(), self.checksum())
    }
}

#[derive(Debug)]
pub struct LevelInfo {
    pub levelNumber: u32,
    pub is_final: bool,
    pub instanceFileID: InstanceFileId,
}

pub fn level_info(path: &Path) -> anyhow::Result<LevelInfo> {
    let name = path.file_name().unwrap().to_str().unwrap();
    let pos = name
        .find('_')
        .ok_or(anyhow!("invalid level file {:?} - '_' expected", path))?;
    let level = (&name[5..pos]).parse::<u32>()?;
    let name = &name[pos + 1..];
    let pos = name
        .find('.')
        .ok_or(anyhow!("invalid level file {:?} - '.' expected", path))?;
    let cp = &name[..pos];
    let (is_final, checksum) = if cp == "Final" {
        (true, 0)
    } else {
        (
            false,
            cp.bytes()
                .fold((0u32, 1u32), |(checksum, factor), c| {
                    (
                        checksum + (factor * (c.to_ascii_uppercase() - b'A') as u32 + 1),
                        factor + 1,
                    )
                })
                .0,
        )
    };
    Ok(LevelInfo{
        levelNumber: level,
        is_final,
        instanceFileID: InstanceFileId::new(level, checksum)
    })
}
