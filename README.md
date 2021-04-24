# wp-pkg

cli for migrating your wp sites

- zips the wp-content folder with ignoring things like node_modules/.git etc
- uses the wp-config file to export the database and uses serde_php for deserializing stuff
- reads .wp-pkg file for settings like old url and new url
