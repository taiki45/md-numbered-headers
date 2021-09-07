use md_numbered_headers::*;

const DEFAULT_OPT: Opt = Opt {
    cleanup_only: false,
    start_depth: 2,
    end_depth: 6,
    reset_with_higher_depth: false,
};

#[test]
fn test_increment_only() {
    let basic_md = "
## heading
### heading
### heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading
### 1.2. heading
"
    );
}

#[test]
fn test_increment_with_reset() {
    let basic_md = "
## heading
### heading
### heading
## heading
### heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading
### 1.2. heading
## 2. heading
### 2.1. heading
"
    );
}

#[test]
fn test_with_heading1() {
    let basic_md = "
# heading
## heading
### heading
# heading
## heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
# heading
## 1. heading
### 1.1. heading
# heading
## 2. heading
"
    );
}

#[test]
fn test_depth5() {
    let basic_md = "
## heading
### heading
#### heading
##### heading
###### heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading
#### 1.1.1. heading
##### 1.1.1.1. heading
###### heading
"
    );
}

#[test]
fn test_non_incremental() {
    let basic_md = "
### heading
### heading
## heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
### 1.1. heading
### 1.2. heading
## 2. heading
"
    );
}

#[test]
fn test_existing_numbers() {
    let basic_md = "
## 1. heading
### 1.5. heading (1.2) test
## 1.3. heading
### heading 3.2 test
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading (1.2) test
## 2. heading
### 2.1. heading 3.2 test
"
    );
}

#[test]
fn test_last_newline() {
    let basic_md = "
## 1. heading
### 1.5. heading
## 1.3. heading
```
```

"
    .to_string();

    let out = process(&process(&basic_md, DEFAULT_OPT), DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading
## 2. heading
```
```

"
    );
}

#[test]
fn test_ignore_code_block() {
    let basic_md = "
## heading
## heading
```
## comment
$ echo
```
"
    .to_string();

    let out = process(&process(&basic_md, DEFAULT_OPT), DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
## 2. heading
```
## comment
$ echo
```
"
    );
}

#[test]
fn test_clean_up() {
    let basic_md = "
### 1.1. heading
### 1.2. heading
```
## comment 1.2
$ echo
```
"
    .to_string();

    let opt = Opt {
        cleanup_only: true,
        ..DEFAULT_OPT
    };
    let out = process(&basic_md, opt);
    assert_eq!(
        out,
        "
### heading
### heading
```
## comment 1.2
$ echo
```
"
    );
}

#[test]
fn test_no_space() {
    let basic_md = "
## 1.heading
## 1.heading
"
    .to_string();

    let out = process(&basic_md, DEFAULT_OPT);
    assert_eq!(
        out,
        "
## 1. heading
## 2. heading
"
    );
}

#[test]
fn test_start_end_depth() {
    let basic_md = "
## heading
```
# comment 1.2
$ echo
```
### heading
#### heading
# heading
"
    .to_string();

    let opt = Opt {
        start_depth: 1,
        end_depth: 4,
        ..DEFAULT_OPT
    };
    let out = process(&basic_md, opt);
    assert_eq!(
        out,
        "
## 1.1. heading
```
# comment 1.2
$ echo
```
### 1.1.1. heading
#### heading
# 2. heading
"
    );
}

#[test]
fn test_reset() {
    let basic_md = "
## heading
## heading
# heading
## heading
"
    .to_string();

    let opt = Opt {
        reset_with_higher_depth: true,
        ..DEFAULT_OPT
    };
    let out = process(&basic_md, opt);
    assert_eq!(
        out,
        "
## 1. heading
## 2. heading
# heading
## 1. heading
"
    );
}
