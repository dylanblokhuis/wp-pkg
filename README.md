# wp-pkg

Simple tool for packaging your WordPress project. Ignores non versioned files like node_modules and .git

## Usage

```bash
$ wp-pkg <path>
```

# Roadmap

- zips the wp-content folder with ignoring things like node_modules/.git etc ✔️
- uses the wp-config file to export the database ✔️
- search and replace the serialized stuff
- config file? e.g. reading .wp-pkg file for settings like old url and new url
