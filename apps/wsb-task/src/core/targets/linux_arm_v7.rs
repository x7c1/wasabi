use crate::core::targets::BuildTarget;

pub struct LinuxArmV7;

impl BuildTarget for LinuxArmV7 {
    fn as_triple(&self) -> &str {
        "armv7-unknown-linux-musleabihf"
    }
}