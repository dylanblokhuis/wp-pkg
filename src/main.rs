use std::{fs::File, io};
use std::{io::prelude::*, path::Path};

use mysql::{prelude::Queryable, Pool};
use walkdir::WalkDir;
use zip::write::FileOptions;

fn main() -> Result<(), io::Error> {
    let dir_where_cmd_is_run = "C:\\dev\\web\\wordpress".to_owned();
    let wp_content_dir = dir_where_cmd_is_run + "\\wp-content";

    zip(wp_content_dir.as_str(), "./wp3.zip");
    dump_db();

    Ok(())
}

fn zip(src_path: &str, zip_dest: &str) -> Result<(), io::Error> {
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

        println!("zipping: {}", name);

        if path.is_file() {
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if path_stripped.as_os_str().len() != 0 {
            zip.add_directory(name, options)?;
        }
    }

    println!("Zipped {} to {}", src_path, zip_dest);
    Ok(())
}

fn dump_db() -> Result<(), mysql::Error> {
    let url = "mysql://root:password@localhost:3306/db_name";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let res = conn.query_drop("SHOW TABLES")?;

    Ok(())
}
