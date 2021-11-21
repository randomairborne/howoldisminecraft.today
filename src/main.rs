use howoldisminecraft::{router, update_manifest};

fn main() {
    std::thread::spawn(|| {
        update_manifest().expect("failed to update manifest");
        std::thread::sleep(std::time::Duration::from_secs(3600));
    });

    let addr = "127.0.0.1:7878";
    gotham::start(addr, router()).expect("failed to start server")
}
