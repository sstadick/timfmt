# 📐 timfmt

A small utility for formatting things the way Tim likes.

## Install

```
cargo install timfmt
```

## Usage

Simply copy a block of code from you editor and run it through `timfmt`:

```
pbpaste | timfmt | pbcopy
```

Then paste it back over your ugly and weirdly indented code.

**Note** This can only handle things on the same level of indentation.
Multiple levels of indentation will be stripped of leading indents.

## Example

Before

```
cpus = { check_max_cpus(1) }
memory = { check_max_mem(4.GB) }
```

After

```
cpus   = { check_max_cpus(1)   } 
memory = { check_max_mem(4.GB) } 
```
