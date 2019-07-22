# YAML Merge

Just a little rust based script to merge yaml files (i.e. stack their content into one file) recursively.

Run it:
```bash
cargo install cargo-script
./yamlmerge.rs
```

It's a legacy thing, a very similar effect can be achieved by
```bash
yq -y -s  'reduce .[] as $x ({}; . * $x)' original.yaml patch1.yaml patch2.yaml
```
but I'm nostalgic towards my own code and do not want to throw it away.
