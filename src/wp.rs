use regex::Regex;
use std::{
    fs::{self, File},
    io,
};
use std::{io::prelude::*, path::Path};
use walkdir::WalkDir;
use zip::write::FileOptions;

pub fn zip(src_path: &str, zip_dest: &str) -> Result<(), io::Error> {
    let blacklisted_names = ["node_modules", ".git", ".DS_Store"];

    let walked_src_dir = WalkDir::new(src_path);
    let it = walked_src_dir.into_iter().filter_map(|e| e.ok());

    let zip_file = File::create(zip_dest).unwrap();

    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();

        if blacklisted_names
            .iter()
            .any(|name| path.display().to_string().contains(name))
        {
            continue;
        }

        let path_stripped = path.strip_prefix(Path::new(src_path)).unwrap();
        let name = path_stripped.display().to_string();

        if path.is_file() {
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !path_stripped.as_os_str().is_empty() {
            zip.add_directory(name, options)?;
        }
    }

    Ok(())
}

pub struct Config {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
}

pub fn read_config(wp_config_path: &str) -> Result<Config, io::Error> {
    let raw_config = &mut fs::read_to_string(wp_config_path)?;
    remove_whitespace(raw_config);

    let matches = Regex::new(r"define\('(?P<key>(.*?))','(?P<value>(.*?))'\);").unwrap();

    let mut config: Config = Config {
        db_name: String::new(),
        db_user: String::new(),
        db_password: String::new(),
        db_host: String::new(),
    };

    for caps in matches.captures_iter(raw_config.as_str()) {
        match &caps["key"] {
            "DB_NAME" => config.db_name = caps["value"].to_string(),
            "DB_USER" => config.db_user = caps["value"].to_string(),
            "DB_PASSWORD" => config.db_password = caps["value"].to_string(),
            "DB_HOST" => config.db_host = caps["value"].to_string(),
            _ => continue,
        }
    }

    Ok(config)
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
