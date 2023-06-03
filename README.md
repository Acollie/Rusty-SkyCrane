# Rusty SkyCrane

Sky crane is a rust cli utility to upload and update AWS lambda code.
Then a lambda with the same name is detected the code will be updated instead.

## Usage

```bash cargo run <filename> <function_name> <role arn>```
If the function exists the program will update the code.

## Supported filetypes
- Python
- Go


## Working in progress 
- [ ] Allow for role to be created on the fly
- [ ] Detect program dependencies 
- [ ] Auto zip dependencies
- [ ] Add Nodejs support