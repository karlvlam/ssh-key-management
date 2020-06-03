# gen-user-file

### Usage

```
USAGE:
    gen-user-file --userdir <user_dir> --out <user_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --userdir <user_dir>    User directory
    -o, --out <user_file>       User file name

```


### Example

Prepare a user directory, with system user files in it:

```
sample_role/
├── ec2_user
└── ubuntu
```

Run `gen-user-file` to generate the user file:
```
$ gen-user-file  -u sample_role -o sample-role.json 

5c15249a17c12483616943e37ef34ff1f498397bd6a4e621aafe7a62bd5c2c6f
{
  "ec2_user": [
    "paul",
    "mary",
    "peter"
  ],
  "ubuntu": [
    "peter",
    "paul",
    "mary"
  ]
}
User file created: sample-role.json
```






