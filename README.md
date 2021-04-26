# wp-pkg

Simple tool for packaging your WordPress project and database. Ignores non versioned files like node_modules and .git

## Installation
```bash
$ brew install dylanblokhuis/wp-pkg/wp-pkg
```

or [download](https://github.com/dylanblokhuis/wp-pkg/releases) the binaries and put them in your $PATH variable

## Usage

```bash
$ wp-pkg <path>
```

## How to use with Local by Flywheel

Local uses a different mysql socket for each website, to make wp-pkg work you need to specify the path to your mysql socket.


```php
// wp-config.php
define( 'DB_SOCKET', '/Users/<user>/Library/Application Support/Local/run/<site_id>/mysql/mysqld.sock' );
```

<img src="https://i.imgur.com/b3Gpu6b.png" alt="Screenshot showcasing the Local by Flywheel database socket location">

# Roadmap

- zips the wp-content folder with ignoring things like node_modules/.git etc ✔️
- uses the wp-config file to export the database ✔️
- search and replace the serialized stuff
- config file? e.g. reading .wp-pkg file for settings like old url and new url
