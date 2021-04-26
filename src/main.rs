use clap::{App, Arg};
use simple_stopwatch::Stopwatch;
use std::env;
use std::path::Path;
use std::path::PathBuf;

mod db;
mod wp;

fn main() {
    let sw = Stopwatch::start_new();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::from_usage("<path> 'Path to your WordPress project'"))
        .get_matches();

    let path_arg;
    match matches.value_of("path") {
        Some(val) => path_arg = val,
        None => {
            println!("No path argument supplied");
            std::process::exit(1);
        }
    }

    let path = PathBuf::from(path_arg);

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

    let db_url;
    match wp_config.db_socket {
        Some(socket) => {
            db_url = format!(
                "mysql://{user}:{password}@{host}:3306/{name}?socket={socket}",
                user = wp_config.db_user,
                password = wp_config.db_password,
                host = wp_config.db_host,
                name = wp_config.db_name,
                socket = socket,
            )
        }
        None => {
            db_url = format!(
                "mysql://{user}:{password}@{host}:3306/{name}",
                user = wp_config.db_user,
                password = wp_config.db_password,
                host = wp_config.db_host,
                name = wp_config.db_name,
            )
        }
    }

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
