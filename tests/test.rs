use md_numbered_headers::process;

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
