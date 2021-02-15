md-numbered-headers
===================

Insert numbered-headers to a markdown file.

Convert from:

```markdown
# Title
## Section one
Text here.

### Sub section one
Text.

## Section two
Another text.
```

To:

```markdown
# Title
## 1. Section one
Text here.

### 1.1 Sub section one
Text.

## 2. Section two
Another text.
```

See more option with `--help`.

## Usage
Single file:

```
$ cat my-file.md | md-numbered-headers | tee my-file.md > /dev/null
```

In the git repogitory:

```
$ for f in `git ls-files |grep .md`; do echo $f; cat $f | md-numbered-headers | tee $f > /dev/null; done
```

### Cleanup only mode
```
$ cat my-file.md | md-numbered-headers -c | tee my-file.md > /dev/null
```

### The `reset_with_higher_depth` option
If true and the `start_depth` is default value `2`, it resets the header number counting with depth 1 header (`#`).

Given:

```markdown
# Title
## Topic a
## Topic b
# Another title
## Topic c
```

Result:

```markdown
# Title
## 1. Topic a
## 2. Topic b
# Another title
## 1. Topic c
```
