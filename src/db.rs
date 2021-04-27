use mysql::{prelude::Queryable, Pool, PooledConn, Row};
use regex::{Captures, Regex};
use std::fs;

struct Table {
    sql: String,
    name: String,
    insert: String,
}
pub fn dump(
    dump_destination: &str,
    db_url: &str,
    current_domain: Option<&String>,
    new_domain: Option<&String>,
) -> Result<(), mysql::Error> {
    let pool = Pool::new(db_url)?;
    let mut conn = pool.get_conn()?;

    let mut tables: Vec<Table> = Vec::new();

    let show_tables_result: Vec<String> = conn.query("SHOW TABLES")?;

    for table_name in show_tables_result {
        let table_create_script: Option<(String, String)> =
            conn.query_first("SHOW CREATE TABLE ".to_owned() + table_name.as_str())?;

        let insert_sql = get_insert_sql(&mut conn, &table_name)?;

        tables.push(Table {
            name: table_name,
            insert: insert_sql,
            sql: format!("{}\n\n", table_create_script.unwrap().1),
        });
    }

    let mut dump_script = create_dump_script(&tables, get_server_version(&mut conn)?.as_ref());

    if let Some(i) = current_domain {
        if let Some(j) = new_domain {
            let matches = Regex::new(r#"s:(?P<length>(.*?)):\\"(?P<value>(.*?))\\""#).unwrap();

            let replace_result = matches.replace(dump_script.as_str(), |caps: &Captures| {
                let value = &caps["value"];
                let new_value = value.replace(i, j);

                format!(r#"s:{}:{}"#, new_value.len(), new_value)
            });

            // replace the serialized values
            dump_script = replace_result.to_string();

            // replace the regular records
            dump_script = dump_script.replace(i, j);
        }
    }

    fs::write(dump_destination, dump_script)?;

    Ok(())
}

fn get_insert_sql(conn: &mut PooledConn, table_name: &str) -> Result<String, mysql::Error> {
    let mut sql = String::new();
    let query_rows: Vec<Row> = conn.query(format!("SELECT * FROM {}", table_name))?;

    if query_rows.is_empty() {
        return Ok(String::new());
    }

    let mut columns: Vec<String> = Vec::new();
    let mut rows: Vec<String> = Vec::new();

    // get all the column names for the insert query
    for column in query_rows[0].columns_ref() {
        columns.push(column.name_str().to_string())
    }

    // get all the values and format them like so: ('1', '2', 'data', 'content'. ..etc)
    for row in query_rows {
        let mut values: Vec<String> = Vec::new();
        for column in row.columns_ref() {
            let column_value = &row[column.name_str().as_ref()];
            let sql = column_value.as_sql(false);

            values.push(sql);
        }

        rows.push(format!("({})", values.join(", ")));
    }

    sql.push_str(
        format!(
            "INSERT INTO {name} ({columns}) VALUES \n {values};",
            name = table_name,
            columns = columns.join(", "),
            values = rows.join(",\n")
        )
        .as_str(),
    );

    Ok(sql)
}

fn get_server_version(conn: &mut PooledConn) -> Result<String, mysql::Error> {
    let version_query: Option<String> = conn.query_first("SELECT version()")?;

    Ok(version_query.unwrap_or("Unknown".to_string()))
}

fn create_dump_script(tables: &[Table], server_version: &str) -> String {
    let mut table_sql = String::new();

    for table in tables {
        let sql = format!(
            "
--
-- Table structure for table {name}
--
DROP TABLE IF EXISTS {name};
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
{sql};
/*!40101 SET character_set_client = @saved_cs_client */;
--
-- Dumping data for table {name}
--
LOCK TABLES {name} WRITE;
/*!40000 ALTER TABLE {name} DISABLE KEYS */;
{insert}
/*!40000 ALTER TABLE {name} ENABLE KEYS */;
UNLOCK TABLES;
        ",
            name = table.name,
            sql = table.sql,
            insert = table.insert
        );

        table_sql.push_str(sql.as_str());
    }

    format!(
        "
-- SQL Dump by wp-pkg on version: {dump_version}
--
-- ------------------------------------------------------
-- Database server version {server_version}
/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

{table_sql}",
        dump_version = env!("CARGO_PKG_VERSION"),
        server_version = server_version,
        table_sql = table_sql
    )
}
