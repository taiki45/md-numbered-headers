md-numbered-headers
===================

Addiing numbered-headers to a markdown file.

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

Spec:

- Only supports header-depth=4.

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
