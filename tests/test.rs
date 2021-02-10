use md_numbered_headers::*;

#[test]
fn test_increment_only() {
    let basic_md = "
## heading
### heading
### heading
"
    .to_string();

    let out = process(&basic_md);
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

    let out = process(&basic_md);
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

    let out = process(&basic_md);
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

    let out = process(&basic_md);
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

    let out = process(&basic_md);
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
"
    .to_string();

    let out = process(&basic_md);
    assert_eq!(
        out,
        "
## 1. heading
### 1.1. heading (1.2) test
## 2. heading
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

    let out = process(&process(&basic_md));
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

    let out = process(&process(&basic_md));
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

    let out = process_cleanup(&basic_md);
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

    let out = process(&basic_md);
    assert_eq!(
        out,
        "
## 1. heading
## 2. heading
"
    );
}
