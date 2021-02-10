md-numbered-headers
===================

Addiing numbered-headers to a markdown file.

## Usage
Single file:

```
$ cat my-file.md | md-numbered-headers | tee my-file.md > /dev/null
```

In the git repogitory:

```
$ for f in `git ls-files |grep .md`; do echo $f; cat $f | md-numbered-headers | tee $f > /dev/null; done
```
