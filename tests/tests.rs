use std::fs::File;
use std::path::Path;

#[cfg(feature = "tokio")]
use tokio::fs::File as AsyncFile;

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

#[cfg(feature = "tokio")]
#[tokio::test]
async fn same_file_async() {
    let data_folder = Path::new("tests").join("data");

    let mut file_1 = AsyncFile::open(data_folder.join("P1140310.jpg")).await.unwrap();
    let mut file_2 = AsyncFile::open(data_folder.join("P1140558.jpg")).await.unwrap();

    assert!(!same_content_from_files_async(&mut file_1, &mut file_2).await.unwrap());
    assert!(same_content_from_files_async(
        &mut AsyncFile::open(data_folder.join("P1140310.jpg")).await.unwrap(),
        &mut file_1
    )
    .await
    .unwrap());
    assert!(same_content_from_files_async(
        &mut AsyncFile::open(data_folder.join("P1140558.jpg")).await.unwrap(),
        &mut file_2
    )
    .await
    .unwrap());
}
