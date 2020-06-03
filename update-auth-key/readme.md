# update-auth-key

### Usage

```
USAGE:
    update-auth-key --keyurl <key_file_url> --userurl <user_file_url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --keyurl <key_file_url>      Key file URL
    -u, --userurl <user_file_url>    User file URL
```

### Example 
```bash
$ update-auth-key -k https://s3-ap-southeast-1.amazonaws.com/crypto-karl-test/keylist -u https://s3-ap-southeast-1.amazonaws.com/crypto-karl-test/sample_role.json

2019-03-11T09:03:18.366 [WARN] USER_KEY_NOT_FOUND: USER => "ubuntu", KEY => "mary",
2019-03-11T09:03:18.366 [WARN] USER_KEY_NOT_FOUND: USER => "ec2_user", KEY => "paul",
2019-03-11T09:03:18.373 [WARN] USER_OR_HOME_NOT_FOUND: "ubuntu"
2019-03-11T09:03:18.376 Skipped unchanged key file: "/home/ec2_user/.ssh/authorized_keys"
```

### File format

Byte 0-63: sha256sum of byte [65-n]
Byte 64  : "\n" 
Byte 65-n: file content

### File Example

Key file:

```
8684ff31e792b0adc4713cd32ab7099c8b3932600505627ee093016bc9b3cf89
{
  "karl": "ssh-rsa AA/UXidw/NZS+IyDmkPtZQlN3BvAORu7UvlqYHzU/Jx1ESl/kTsNmBwdxYpNZMXXBtTayLzGIVHUYhZUWimXKLMTRNsLmuwIkJ15CBt3He8MnrkHJf8QGhwWe8PXkWDrMJPoQhWGOMAJkY1cvKweq/PMslLNXE6AChFPuHamL5v6otoYWYfhMhl1RCDD8nd5gCHVuBO5etMguFKdjyAzJ+thORJ7aW8nZKY/7pI/9W8kYmWr0CvHyrgF5a1VPSWENV1SO4sXihhpX4IC56UVU/CktgfqdOvJ7/PVzngvVJc/io2FuiYqdGBNQPoQdDzqoV6ib+A1JCmYtbWe4dP3hCF4kQoIKTG11czYl2PCH9o1vAXKDrCEwIyGoR4ApnaspntZtD5U6DE1b5wiaOCBaqQAhastqNbAE5rr+CVp3m+nZBD6eTC/7Am3/2wchqwrfLllqq5Nw== karl@test.local\n",
  "peter": "ssh-rsa Ab+Mf1eiXWIqhVJkXQTU/Q0Mg48j65cVCYsNcW8pZwWODsPIkxamOSHTAsKadXGlZlZovQ0fvvg+vKcE1Fh+sxMAiQDFGffubEqqJVgi1tFZ/7pI/9W8kYmWr0CvHyrgF5a1VPSWENV1SO4sXihhpX4IC56UVU/CktgfqdOvJ7/PVzngvVJcj7ih5tnBkwKM68UluryP1hdFTuZ9Os3AOIOPzscK9mz0uxUXIx0x3yWqBKikeb4P1saeLpLqDxq/io2FuiYqdGBNQPoQdDzqoV6ib+A1JCmYtbWe4dP3hCF4kQoIKTG11czYl2PCH9o1vAXKDrCEwIyGoR4ApnaspntZtD5U6DE1b5wiaOCBaqQAhastqNbAE5rr+CVp3m+nZBD6eTC/7Am3/2wchqwrfLllqq5Nw== peter@test.local\n"
}
```

User file:

```
4d146252a788aaf98500f720e5129b173d993c9066df3464ab605fcd40c27eff
{
  "ubuntu": [
    "peter",
    "paul",
    "mary"
  ],
  "ec2_user": [
    "peter",
    "paul",
    "mary"
  ]
}
```

### How it works

1. Download key file and user file
2. Check file integrity
3. Locate system user's home directories (from user file) 
4. Generate authorized_keys content, then compare with `~/.ssh/authorized_keys`
5. Update `~/.ssh/authorized_keys` for any changes.

