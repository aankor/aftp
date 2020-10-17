use aftp::{types::Result, FtpStream};
use async_std::io::Cursor;
use claim::{assert_ok, assert_ok_eq};

#[async_std::test]
async fn test_ftp() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").await.unwrap();
    assert_ok!(ftp_stream.login("Doe", "mumble").await);

    assert_ok!(ftp_stream.mkdir("test_dir").await);
    assert_ok!(ftp_stream.cwd("test_dir").await);
    assert_ok!(ftp_stream
        .pwd()
        .await
        .map(|pwd| assert!(pwd.ends_with("/test_dir"))));

    // store a file
    let file_data = "test data\n";
    let mut reader = Cursor::new(file_data.as_bytes());
    assert_ok!(ftp_stream.put("test_file.txt", &mut reader).await);

    // retrieve file
    assert_ok_eq!(
        ftp_stream
            .simple_retr("test_file.txt")
            .await
            .map(std::io::Cursor::into_inner),
        file_data.as_bytes()
    );

    // remove file
    assert_ok!(ftp_stream.rm("test_file.txt").await);

    // cleanup: go up, remove folder, and quit
    assert_ok!(ftp_stream.cdup().await);

    assert_ok!(ftp_stream.rmdir("test_dir").await);
    assert_ok!(ftp_stream.quit().await);
}
