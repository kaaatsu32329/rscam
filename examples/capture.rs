fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 300),
            resolution: (256, 144),
            format: b"Z16 ",
            ..Default::default()
        })
        .unwrap();

    for i in 1.. {
        let frame = camera.capture().unwrap();
        println!("Frame #{} of length {}", i, frame.len());
    }
}
