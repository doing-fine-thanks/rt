# rt
### _A half-baked `tree` clone._

`rt` is a shameless, less useful, and untested clone
of the beloved program `tree`. 

It will display files in a directory like so:

```text
$ rt
.
├─ Cargo.toml
├─ Cargo.lock
├─ README.md
├─ .gitignore
├─ .idea
│  ├─ workspace.xml
│  ├─ modules.xml
│  ├─ vcs.xml
│  └─ rt.iml
└─ src
   └─ main.rs
```

It also allows for pattern matching (`-p`):
```text
$ rt -p "./s*"                 

.
└─ src
   └─ main.rs
```


And exclusion patterns (`-e`):
```text
$ rt
.
├─ Cargo.toml
├─ Cargo.lock
├─ README.md
├─ .gitignore
└─ src
   └─ main.rs
```

This tool is built using [ptree](https://docs.rs/ptree/latest/ptree/), 
so you can leverage the great [style options](https://docs.rs/ptree/latest/ptree/print_config/struct.PrintConfig.html) 
that those folks whipped up.

## Contributions:
Currently `rt` isn't taking contributions. If you want to suggest 
something, drop me an email or something (or feel free to fork).
