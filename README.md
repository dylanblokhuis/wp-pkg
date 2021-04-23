# wp-pkg

cli for migrating your wp sites

- zips the wp-content folder with ignoring things like node_modules/.git etc
- uses the wp-config file to export the database
- reads .wp-pkg file for settings like old url and new url

```
// overwrites website on url
$ wp-pkg deploy
```

```
// overwrites current wp-content and database
$ wp-pkg pull
```
