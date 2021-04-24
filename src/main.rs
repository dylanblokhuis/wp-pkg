mod db;
mod wp;

fn main() {
    let dir_where_cmd_is_run = "C:\\dev\\web\\wordpress".to_owned();
    let wp_content_dir = dir_where_cmd_is_run + "\\wp-content";

    match wp::zip(wp_content_dir.as_str(), "./wp3.zip") {
        Ok(res) => res,
        Err(error) => {
            println!("Failed to zip wp-content: {:?}", error);
            std::process::exit(1);
        }
    }
    match db::dump("./dump.sql") {
        Ok(res) => res,
        Err(error) => {
            println!("Failed to dump database: {:?}", error);
            std::process::exit(1);
        }
    }
}
