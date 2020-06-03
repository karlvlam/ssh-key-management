# gen-key-file

### Usage

```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --keydir <key_dir>    Key directory
    -o, --out <key_file>      Key file name
```


### Example

Prepare a key directory, with ssh public key files in it:

```
my_keys/
├── paul
└── peter
```

Run `gen-key-file` to generate the key file:
```
$ gen-key-file -k my_keys -o my_key_file.json
99dcbfc6de3510a51bc2732e102ae28f7a320d45b31433a0d2256096179e9671
{
  "peter": "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDSpcb4gqdMp9qBDJx5NCCX4zzyUKvNY0+ud70yAmFWmXc9YHvdNpOhyIq0p2uAKYxTzwJCkG0jCXnHS+NmH+byapSyZuvKHCOcaIfbRFK2lOtrYVjpCDsyvqWUVsAL9II0hh1B67F5tdhqEiKrQBTIhvwkG4dzwfsp+HVZrvPo0rPTftPsKEAyXcHkXWhlFy/RWGjyKVHH+0Dhx50RXrOuQwHKsUi+AxV9QH/fHOKEEjbxv5XDWI0k9Bh3GThptzbYe3tCZuPcD7SnLWhHVy/AW/7mdNS2E+dDyxIKwIDMLLWzcsw4owd/eSZ5SXEiNH4lQNQpE3jsy/zQKh9APzWn peter@test.local\n",
  "paul": "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQD44XpAiDY8YTxumidy9FwdHXKyCNGVfiIVAujc6jP3SwJz1jUfW+ffqcxlko8lbeYb7iNj8sUVDkD83d1f72zAgqe6Z0pLN2tt7aUpqUJZdN7nQGyJPkdc2/JUkrogTH12Rwla3gDsW+zxCLITJhrVyRk2WW3m2U2Xfb5WIumKKOmqQglcXSEPqU87ap8dY1zXDxGhVwt5HIjcllqWghqI2QkFlowPQ6SMQVfmEAIQEo1qZWMG+HlKJVpRVgmaAdWnZ6ucx1VuO4Mg0honn9He6QuDW+cEgGFW7FbQlgfudMfrslUlQ82UwCXNyjEE37/EU7dg4o2TiDuxJ3NdKKWf paul@test.local\n"
}
Key file created: my_key_file.json
```






