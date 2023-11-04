fn main() -> std::io::Result<()> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../media/icon.ico");
        res.compile()?;
    }

    Ok(())
}
