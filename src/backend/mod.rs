pub mod zygisk;
pub mod ebpf;

pub fn initialize() -> Result<Box<dyn Controller>, anyhow::Error> {
    if cfg!(target_os = "android") {
        if let Ok(backend) = std::env::var("FAS_BACKEND") {
            match backend.as_str() {
                "zygisk" => {
                    info!("Initializing Zygisk backend");
                    Ok(Box::new(zygisk::ZygiskController::new()?))
                },
                "ebpf" => {
                    info!("Initializing eBPF backend");
                    Ok(Box::new(ebpf::EBPFController::new()?))
                },
                _ => Err(anyhow!("Invalid backend: {}", backend))
            }
        } else {
            Err(anyhow!("FAS_BACKEND not set"))
        }
    } else {
        Err(anyhow!("Unsupported platform"))
    }
}