use anyhow::Result;
use libc::{c_char, c_int, c_ulong, ioctl, O_RDWR, open, write as libc_write};
use log::{info, error};
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Write};

// Zygisk JNI绑定
#[link(name = "zygisk")]
extern "C" {
    fn zygisk_hook() -> c_int;
    fn zygisk_get_command() -> *const c_char;
}

pub struct ZygiskController {}

impl ZygiskController {
    pub fn new() -> Result<Self> {
        unsafe {
            let ret = zygisk_hook();
            if ret != 0 {
                return Err(anyhow::anyhow!("Zygisk hook failed: {}", ret));
            }
            info!("Zygisk hook success");
        }
        Ok(ZygiskController {})
    }
}

impl super::Controller for ZygiskController {
    fn start(&self) -> Result<()> {
        unsafe {
            let cmd = CStr::from_ptr(zygisk_get_command())
                .to_str()
                .map_err(|e| anyhow::anyhow!("Invalid command encoding: {}", e))?;
            info!("Zygisk command: {}", cmd);

            // 实现核心调度逻辑
            match cmd {
                "start" => {
                    // 设置大核在线
                    Self::set_cpu_online(4, true)?;
                    Self::set_cpu_online(5, true)?;
                    
                    // 设置最大频率
                    Self::set_cpu_freq(4, 1512000)?;
                    Self::set_cpu_freq(5, 1512000)?;

                    info!("Zygisk调度已启动");
                },
                _ => return Err(anyhow::anyhow!("未知命令: {}", cmd)),
            }
        }
        Ok(())
    }
}

impl ZygiskController {
    fn set_cpu_online(cpu: i32, online: bool) -> io::Result<()> {
        let path = format!("/sys/devices/system/cpu/cpu{}/online", cpu);
        let mut file = File::create(&path)?;
        file.write_all(if online { b"1" } else { b"0" })?;
        Ok(())
    }

    fn set_cpu_freq(cpu: i32, freq: c_ulong) -> io::Result<()> {
        let fd = unsafe { open(format!("/dev/cpu/{}/cpufreq", cpu).as_ptr() as *const i8, O_RDWR) };
        unsafe {
            ioctl(fd, 0x40045432, freq); // _IOC(_IOC_READ|_IOC_WRITE, 'P', 1, sizeof(unsigned long))
        }
        Ok(())
    }
}