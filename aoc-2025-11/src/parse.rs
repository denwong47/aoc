use crate::models::*;

pub fn str_to_device_id(s: &str) -> DeviceId {
    let chars: Vec<char> = s.trim().chars().collect();
    ((chars[0] as u32) << 16) | ((chars[1] as u32) << 8) | (chars[2] as u32)
}

/// Breaks down lines of ``ccc: ddd eee fff`` into [`Device`] objects
pub fn line_to_device(line: &str) -> anyhow::Result<Device> {
    line.split_once(": ")
        .ok_or_else(|| anyhow::anyhow!("Invalid line format: {}", line))
        .and_then(|(id_str, neighbours_str)| {
            let id = str_to_device_id(id_str);
            let neighbours = neighbours_str
                .trim()
                .split_whitespace()
                .map(str_to_device_id)
                .collect::<Vec<_>>();
            Ok(Device::new(id, neighbours.into_iter()))
        })
}

pub fn text_to_devices(input: &str) -> anyhow::Result<DeviceMap> {
    let devices = fxhash::FxHashMap::from_iter(
        input
            .lines()
            .map(|line| {
                let device = line_to_device(line)?;
                Ok((device.id(), device))
            })
            .collect::<anyhow::Result<DeviceMap>>()?,
    );
    Ok(devices)
}
