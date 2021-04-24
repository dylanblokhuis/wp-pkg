use simple_stopwatch::Stopwatch;
use std::env;
use std::path::Path;
use std::path::PathBuf;

mod db;
mod wp;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);
    let sw = Stopwatch::start_new();
    let path = PathBuf::from(&args[1]);

    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(path);

    println!("{}", absolute_path.display());

    let wp_content_dir = absolute_path.join("wp-content").display().to_string();
    let wp_config_path = absolute_path.join("wp-config.php").display().to_string();

    if !Path::new(wp_content_dir.as_str()).exists() {
        println!("Directory cannot be found: {:?}", wp_content_dir);
        std::process::exit(1);
    }

    let wp_config;

    println!("Reading wp-config from {}", wp_config_path);
    match wp::read_config(wp_config_path.as_str()) {
        Ok(res) => {
            println!("Read wp-config from {}\n", wp_config_path);
            wp_config = res;
        }
        Err(error) => {
            println!("Failed to read wp-config: {:?}", error);
            std::process::exit(1);
        }
    }

    println!("Zipping {}", wp_content_dir);
    match wp::zip(wp_content_dir.as_str(), "./wp.zip") {
        Ok(_) => println!("Zipped {}\n", wp_content_dir),
        Err(error) => {
            println!("Failed to zip wp-content: {:?}", error);
            std::process::exit(1);
        }
    }

    let db_url = format!(
        "mysql://{user}:{password}@{host}:3306/{name}",
        user = wp_config.db_user,
        password = wp_config.db_password,
        host = wp_config.db_host,
        name = wp_config.db_name
    );

    println!("Dumping database with credentials: {}", db_url.as_str());
    match db::dump("./dump.sql", db_url.as_str()) {
        Ok(_) => {
            println!("Dumped database with credentials: {}\n", db_url.as_str());
        }
        Err(error) => {
            println!("Failed to dump database: {:?}", error);
            std::process::exit(1);
        }
    }

    let seconds = sw.s();
    println!("✨ Finished in {:.2} seconds", seconds);
}
