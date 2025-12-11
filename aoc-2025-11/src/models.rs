use simple_graph::traits;
use std::collections::HashMap;

pub type DeviceId = [char; 3];
pub type Distance = u32;

pub type DeviceMap = HashMap<DeviceId, Device>;

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

    pub fn new_empty(id: DeviceId) -> Self {
        Self {
            id,
            connected_devices: Vec::new(),
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

pub fn text_to_devices(input: &str) -> anyhow::Result<DeviceMap> {
    let devices = HashMap::from_iter(
        input
            .lines()
            .map(|line| {
                let device = line_to_device(line)?;
                Ok((device.id.clone(), device))
            })
            .collect::<anyhow::Result<DeviceMap>>()?,
    );
    Ok(devices)
}

pub fn invert_device_map(map: &DeviceMap) -> HashMap<DeviceId, Device> {
    let mut inverted: HashMap<DeviceId, Device> = HashMap::new();

    for (device_id, device) in map.iter() {
        inverted
                .entry(*device_id)
                .or_insert_with(|| Device::new_empty(*device_id));
        for neighbour_id in device.connected_devices.iter() {
            inverted
                .entry(*neighbour_id)
                .or_insert_with(|| Device::new_empty(*neighbour_id))
                .connected_devices
                .push(*device_id);
        }
    }

    inverted
}

#[cfg(test)]
mod test_invert_device_map {
    use super::*;

    const INPUT: &str = "svr: aaa bbb
                         aaa: fft
                         fft: ccc
                         bbb: tty
                         tty: ccc
                         ccc: ddd eee
                         ddd: hub
                         hub: fff
                         eee: dac
                         dac: fff
                         fff: ggg hhh
                         ggg: out
                         hhh: out";

    #[test]
    fn test_invert_device_map() {
        let devices = text_to_devices(INPUT).expect("Failed to parse devices from input");
        let inverted = invert_device_map(&devices);

        assert_eq!(inverted[&['s', 'v', 'r']].connected_devices.len(), 0);
        assert_eq!(inverted[&['a', 'a', 'a']].connected_devices, vec![['s', 'v', 'r']]);
        assert_eq!(inverted[&['b', 'b', 'b']].connected_devices, vec![['s', 'v', 'r']]);
        assert_eq!(inverted[&['f', 'f', 't']].connected_devices, vec![['a', 'a', 'a']]);
        assert_eq!(inverted[&['t', 't', 'y']].connected_devices, vec![['b', 'b', 'b']]);
        assert_eq!(inverted[&['c', 'c', 'c']].connected_devices, vec![['t', 't', 'y'], ['f', 'f', 't']]);
        assert_eq!(inverted[&['d', 'd', 'd']].connected_devices, vec![['c', 'c', 'c']]);
        assert_eq!(inverted[&['e', 'e', 'e']].connected_devices, vec![['c', 'c', 'c']]);
        assert_eq!(inverted[&['h', 'u', 'b']].connected_devices, vec![['d', 'd', 'd']]);
        assert_eq!(inverted[&['d', 'a', 'c']].connected_devices, vec![['e', 'e', 'e']]);
        assert_eq!(inverted[&['f', 'f', 'f']].connected_devices, vec![['h', 'u', 'b'], ['d', 'a', 'c']]);
        assert_eq!(inverted[&['g', 'g', 'g']].connected_devices, vec![['f', 'f', 'f']]);
        assert_eq!(inverted[&['h', 'h', 'h']].connected_devices, vec![['f', 'f', 'f']]);
        assert_eq!(inverted[&['o', 'u', 't']].connected_devices, vec![['g', 'g', 'g'], ['h', 'h', 'h']]);
    }       
}