use simple_graph::traits;


pub type DeviceId = u32;
pub type Distance = u32;

pub type DeviceMap = fxhash::FxHashMap<DeviceId, Device>;

#[cfg(feature = "trace")]
pub trait DeviceIdToStr {
    fn to_str(&self) -> String;
}

#[cfg(feature = "trace")]
impl DeviceIdToStr for DeviceId {
    fn to_str(&self) -> String {
        let c1 = ((*self >> 16) & 0xFF) as u8 as char;
        let c2 = ((*self >> 8) & 0xFF) as u8 as char;
        let c3 = (*self & 0xFF) as u8 as char;
        format!("{}{}{}", c1, c2, c3)
    }
}

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

    pub fn id(&self) -> DeviceId {
        self.id
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

impl<'s> traits::IsNodeWithIndexedNeighbours<'s, DeviceId, u32> for Device {
    fn get_neighbour(
        &'s self,
        index: usize,
        get_node_by_key: impl Fn(&DeviceId) -> Option<&'s Self>,
    ) -> Option<(&'s Self, u32)> {
        self.connected_devices.get(index).and_then(|neighbour_id| {
            get_node_by_key(neighbour_id).map(|node| (node, 1))
        })
    }
}

pub fn invert_device_map(map: &DeviceMap) -> DeviceMap {
    let mut inverted: DeviceMap = DeviceMap::default();

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
    use crate::parse;

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
        let devices = parse::text_to_devices(INPUT).expect("Failed to parse devices from input");
        let inverted = invert_device_map(&devices);

        assert_eq!(inverted[&parse::str_to_device_id("svr")].connected_devices.len(), 0);
        assert_eq!(inverted[&parse::str_to_device_id("aaa")].connected_devices, vec![parse::str_to_device_id("svr")]);
        assert_eq!(inverted[&parse::str_to_device_id("bbb")].connected_devices, vec![parse::str_to_device_id("svr")]);
        assert_eq!(inverted[&parse::str_to_device_id("fft")].connected_devices, vec![parse::str_to_device_id("aaa")]);
        assert_eq!(inverted[&parse::str_to_device_id("tty")].connected_devices, vec![parse::str_to_device_id("bbb")]);
        assert_eq!(inverted[&parse::str_to_device_id("ccc")].connected_devices, vec![parse::str_to_device_id("fft"), parse::str_to_device_id("tty")]);
        assert_eq!(inverted[&parse::str_to_device_id("ddd")].connected_devices, vec![parse::str_to_device_id("ccc")]);
        assert_eq!(inverted[&parse::str_to_device_id("eee")].connected_devices, vec![parse::str_to_device_id("ccc")]);
        assert_eq!(inverted[&parse::str_to_device_id("hub")].connected_devices, vec![parse::str_to_device_id("ddd")]);
        assert_eq!(inverted[&parse::str_to_device_id("dac")].connected_devices, vec![parse::str_to_device_id("eee")]);
        assert_eq!(inverted[&parse::str_to_device_id("fff")].connected_devices, vec![parse::str_to_device_id("hub"), parse::str_to_device_id("dac")]);
        assert_eq!(inverted[&parse::str_to_device_id("ggg")].connected_devices, vec![parse::str_to_device_id("fff")]);
        assert_eq!(inverted[&parse::str_to_device_id("hhh")].connected_devices, vec![parse::str_to_device_id("fff")]);
        assert_eq!(inverted[&parse::str_to_device_id("out")].connected_devices, vec![parse::str_to_device_id("ggg"), parse::str_to_device_id("hhh")]);
    }       
}