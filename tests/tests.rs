extern crate same_content;

use std::fs::File;
use std::path::Path;

use same_content::*;

#[test]
fn same_file() {
    let data_folder = Path::new("tests").join("data");

    let mut file_1 = File::open(data_folder.join("P1140310.jpg")).unwrap();
    let mut file_2 = File::open(data_folder.join("P1140558.jpg")).unwrap();

    assert!(!same_content_from_files(&mut file_1, &mut file_2).unwrap());
    assert!(same_content_from_files(
        &mut File::open(data_folder.join("P1140310.jpg")).unwrap(),
        &mut file_1
    )
    .unwrap());
    assert!(same_content_from_files(
        &mut File::open(data_folder.join("P1140558.jpg")).unwrap(),
        &mut file_2
    )
    .unwrap());
}
