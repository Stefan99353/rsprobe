fn download(url: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(".test_output");
    if !dir.is_dir() {
        std::fs::create_dir(&dir).unwrap();
    }

    let filename = url
        .replace("http:", ":")
        .replace("https", "")
        .replace('/', "__")
        .replace(':', "__");
    let path = dir.join(filename);

    if !path.is_file() {
        let status = std::process::Command::new("curl")
            .args(&[
                "-H",
                "user-agent: Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0",
                "--insecure",
                "-L",
                "-o",
            ])
            .arg(&path)
            .arg(url)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !status.success() {
            panic!("Download failed");
        }
    }

    path
}

async fn probe(url: &str) {
    let path = download(url);

    let builder = rsprobe::ProbeBuilder::new(path.to_str().unwrap())
        .show_program_version()
        .show_library_versions()
        .show_pixel_formats()
        .show_packets()
        .show_frames()
        .show_programs()
        .show_streams()
        .show_chapters()
        .show_format();

    builder.execute()
        .await
        .unwrap();
}

#[tokio::test]
async fn download_and_probe() {
    let items = vec![
        // Images.
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/images/BigBuckBunny.jpg",
        // Audios.
        "365.flac",
        // Videos.
        "taj.mkv",
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-avi-file.avi",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-avi-file.avi",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mov-file.mov",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mpg-file.mpg",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-wmv-file.wmv",
    ];

    for url in items {
        probe(url).await;
    }
}