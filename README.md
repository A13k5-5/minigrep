A simple command-line tool for searching text in files, inspired by the Unix `grep` utility. It allows you to search for a substring in files and display the matching lines.

### How to use
Run the project with the following syntax:

```
cargo run -- <query> <file_path>
```

additionally, you can set case sensitivity by setting `IGNORE_CASE` environment variable to any value.

On Linux, the command would look like this:

```
IGNORE_CASE=1 cargo run -- <query> <file_path>
```
