# Rusty SkyCrane

A simple cli to upload bits of code to lambda

## Usage

```bash cargo run <filename> <function_name> <role arn>```
If the function exists the program will update the code.

## Supported filetypes
- Python
- Go
- Node


## TODO
- [ ] Allow for role to be created on the fly
- [ ] Auto zip dependencies