pub mod mode;
pub mod file;
pub mod runner;

#[cfg(test)]
pub mod tests {
    use tempdir::TempDir;
    pub fn temp_file_path_string<'a>(tmp_dir: &'a TempDir) -> String {
        tmp_dir.path()
            .join("bds_kv_file")
            .to_str()
            .unwrap()
            .to_string()
    }
}
