use simple_graph::traits;
use std::collections::HashMap;

pub type DeviceId = [char; 3];
pub type Distance = u32;

#[derive(Debug, Clone)]
pub struct Device {
    id: DeviceId,
    connected_devices: Vec<DeviceId>,
}

impl Device {
    pub fn new(id: DeviceId, connected_devices: impl Iterator<Item = DeviceId>) -> Self {
        Self {
            id,
            connected_devices: connected_devices.collect(),
        }
    }
}

impl<'s> traits::IsNode<'s, DeviceId, u32> for Device {
    fn id(&self) -> &DeviceId {
        &self.id
    }

    fn neighbours(
        &'s self,
        get_node_by_key: impl Fn(&DeviceId) -> Option<&'s Self>,
    ) -> impl Iterator<Item = (&'s Self, u32)> {
        self.connected_devices
            .iter()
            .filter_map(move |neighbour_id| {
                // Currently hardcoding distance as 1 between connected devices,
                // don't know what Part 2 entails yet
                // Ignore nodes that no longer exists
                get_node_by_key(neighbour_id).map(|node| (node, 1))
            })
    }
}

pub fn str_to_device_id(s: &str) -> anyhow::Result<DeviceId> {
    let chars: Vec<char> = s.trim().chars().collect();
    if chars.len() != 3 {
        return Err(anyhow::anyhow!("Invalid device ID length: {}", s));
    }
    Ok([chars[0], chars[1], chars[2]])
}

/// Breaks down lines of ``ccc: ddd eee fff`` into [`Device`] objects
pub fn line_to_device(line: &str) -> anyhow::Result<Device> {
    line.split_once(": ")
        .ok_or_else(|| anyhow::anyhow!("Invalid line format: {}", line))
        .and_then(|(id_str, neighbours_str)| {
            let id = str_to_device_id(id_str)?;
            let neighbours = neighbours_str
                .trim()
                .split_whitespace()
                .map(str_to_device_id)
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(Device::new(id, neighbours.into_iter()))
        })
}

pub fn text_to_devices(input: &str) -> anyhow::Result<HashMap<DeviceId, Device>> {
    let devices = HashMap::from_iter(
        input
            .lines()
            .map(|line| {
                let device = line_to_device(line)?;
                Ok((device.id.clone(), device))
            })
            .collect::<anyhow::Result<HashMap<DeviceId, Device>>>()?,
    );
    Ok(devices)
}
