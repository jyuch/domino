use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../media/icon.ico");
        res.compile()?;
    }

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(PathBuf::from(env::var("OUT_DIR")?).join("descriptor.bin"))
        .compile(
            &["../proto/hello_world/hello_world.proto"],
            &["../proto/hello_world"],
        )?;

    Ok(())
}
