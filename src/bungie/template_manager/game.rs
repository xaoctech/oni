use std::collections::HashSet;
use std::path::PathBuf;
use anyhow::anyhow;
use crate::bungie::template_manager::common::{InstanceFileId, level_info};

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
                        if info.is_final {
                            self.validLevels.insert(info.levelNumber);
                        }
                        for instanceFileRef in &self.instanceFileRefs {
                            if instanceFileRef.fileIndex == info.instanceFileID {
                                return Err(anyhow!("conflicting file indices"))
                            }
                        }
                        self.instanceFileRefs.push(InstanceFileRef{
                            instanceFileRef: path,
                            fileIndex: info.instanceFileID
                        })
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
