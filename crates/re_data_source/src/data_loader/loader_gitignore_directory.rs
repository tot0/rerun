use ignore::WalkBuilder;

/// Recursively loads entire directories which contain .gitignore file, using the appropriate [`crate::DataLoader`]:s for each
/// files within which aren't ignored.
pub struct GitignoreDirectoryLoader;

impl crate::DataLoader for GitignoreDirectoryLoader {
    #[inline]
    fn name(&self) -> String {
        "rerun.data_loaders.GitignoreDirectory".into()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_from_path(
        &self,
        store_id: re_log_types::StoreId,
        dirpath: std::path::PathBuf,
        tx: std::sync::mpsc::Sender<crate::LoadedData>,
    ) -> Result<(), crate::DataLoaderError> {
        if dirpath.is_file() {
            return Err(crate::DataLoaderError::Incompatible(dirpath.clone()));
        }

        re_tracing::profile_function!(dirpath.display().to_string());

        re_log::debug!(?dirpath, loader = self.name(), "Loading directory…",);

        for entry in WalkBuilder::new(dirpath.as_path()).build() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    re_log::error!(loader = self.name(), ?dirpath, %err, "Failed to open filesystem entry");
                    continue;
                }
            };

            let filepath = entry.path();
            if filepath.is_file() {
                let store_id = store_id.clone();
                let filepath = filepath.to_owned();
                let tx = tx.clone();

                // NOTE(1): `spawn` is fine, this whole function is native-only.
                // NOTE(2): this must spawned on a dedicated thread to avoid a deadlock!
                // `load` will spawn a bunch of loaders on the common rayon thread pool and wait for
                // their response via channels: we cannot be waiting for these responses on the
                // common rayon thread pool.
                _ = std::thread::Builder::new()
                    .name(format!("load_dir_entry({filepath:?})"))
                    .spawn(move || {
                        let data = match crate::load_file::load(&store_id, &filepath, None) {
                            Ok(data) => data,
                            Err(err) => {
                                re_log::error!(?filepath, %err, "Failed to load directory entry");
                                return;
                            }
                        };

                        for datum in data {
                            if tx.send(datum).is_err() {
                                break;
                            }
                        }
                    });
            }
        }

        Ok(())
    }

    #[inline]
    fn load_from_file_contents(
        &self,
        _store_id: re_log_types::StoreId,
        path: std::path::PathBuf,
        _contents: std::borrow::Cow<'_, [u8]>,
        _tx: std::sync::mpsc::Sender<crate::LoadedData>,
    ) -> Result<(), crate::DataLoaderError> {
        // TODO(cmc): This could make sense to implement for e.g. archive formats (zip, tar, …)
        Err(crate::DataLoaderError::Incompatible(path))
    }
}
