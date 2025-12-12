//! # Day 11: **Reactor**
//! 
//! You hear some loud beeping coming from a hatch in the floor of the factory, so you decide to check it out. Inside, you find several large electrical conduits and a ladder.
//! 
//! Climbing down the ladder, you discover the source of the beeping: a large, toroidal reactor which powers the factory above. Some Elves here are hurriedly running between the reactor and a nearby server rack, apparently trying to fix something.
//! 
//! One of the Elves notices you and rushes over. "It's a good thing you're here! We just installed a new server rack, but we aren't having any luck getting the reactor to communicate with it!" You glance around the room and see a tangle of cables and devices running from the server rack to the reactor. She rushes off, returning a moment later with a list of the devices and their outputs (your puzzle input).
//! 
//! For example:
//! 
//! ```text
//! aaa: you hhh
//! you: bbb ccc
//! bbb: ddd eee
//! ccc: ddd eee fff
//! ddd: ggg
//! eee: out
//! fff: out
//! ggg: out
//! hhh: ccc fff iii
//! iii: out
//! ```
//! 
//! Each line gives the name of a device followed by a list of the devices to which its outputs are attached. So, bbb: ddd eee means that device bbb has two outputs, one leading to device ddd and the other leading to device eee.
//! 
//! The Elves are pretty sure that the issue isn't due to any specific device, but rather that the issue is triggered by data following some specific path through the devices. Data only ever flows from a device through its outputs; it can't flow backwards.
//! 
//! After dividing up the work, the Elves would like you to focus on the devices starting with the one next to you (an Elf hastily attaches a label which just says you) and ending with the main output to the reactor (which is the device with the label out).
//! 
//! To help the Elves figure out which path is causing the issue, they need you to find every path from you to out.
//! 
//! In this example, these are all of the paths from you to out:
//! 
//! ```text
//! 
//!     Data could take the connection from you to bbb, then from bbb to ddd, then from ddd to ggg, then from ggg to out.
//!     Data could take the connection to bbb, then to eee, then to out.
//!     Data could go to ccc, then ddd, then ggg, then out.
//!     Data could go to ccc, then eee, then out.
//!     Data could go to ccc, then fff, then out.
//! 
//! ```
//! 
//! In total, there are 5 different paths leading from you to out.
//! 
//! How many different paths lead from you to out?
//! 
//! Your puzzle answer was 796.
//! 
//! ## Part Two
//! 
//! Thanks in part to your analysis, the Elves have figured out a little bit about the issue. They now know that the problematic data path passes through both dac (a digital-to-analog converter) and fft (a device which performs a fast Fourier transform).
//! 
//! They're still not sure which specific path is the problem, and so they now need you to find every path from svr (the server rack) to out. However, the paths you find must all also visit both dac and fft (in any order).
//! 
//! For example:
//! 
//! ```text
//! svr: aaa bbb
//! aaa: fft
//! fft: ccc
//! bbb: tty
//! tty: ccc
//! ccc: ddd eee
//! ddd: hub
//! hub: fff
//! eee: dac
//! dac: fff
//! fff: ggg hhh
//! ggg: out
//! hhh: out
//! ```
//! 
//! This new list of devices contains many paths from svr to out:
//! 
//! ```text
//! svr,aaa,fft,ccc,ddd,hub,fff,ggg,out
//! svr,aaa,fft,ccc,ddd,hub,fff,hhh,out
//! svr,aaa,fft,ccc,eee,dac,fff,ggg,out
//! svr,aaa,fft,ccc,eee,dac,fff,hhh,out
//! svr,bbb,tty,ccc,ddd,hub,fff,ggg,out
//! svr,bbb,tty,ccc,ddd,hub,fff,hhh,out
//! svr,bbb,tty,ccc,eee,dac,fff,ggg,out
//! svr,bbb,tty,ccc,eee,dac,fff,hhh,out
//! ```
//! 
//! However, only 2 paths from svr to out visit both dac and fft.
//! 
//! Find all of the paths that lead from svr to out. How many of those paths visit both dac and fft?
//! 
//! Your puzzle answer was 294053029111296.
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod models;
pub mod parse;

mod input;
use input::INPUT;

use crate::models::DeviceMap;

#[cfg(feature = "profile")]
use std::time::Instant;

const START: &str = "you";
const DESTINATION: &str = "out";

const SERVER_RACK: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn build_devices(input: &str) -> anyhow::Result<models::DeviceMap> {
    let mut map = parse::text_to_devices(input)?;

    let destination_id = parse::str_to_device_id(DESTINATION);
    map.insert(
        destination_id,
        models::Device::new(destination_id, std::iter::empty()),
    );
    Ok(map)
}

fn count_number_of_solutions(
    devices: &DeviceMap,
    start_id: models::DeviceId,
    destination_id: models::DeviceId,
    avoid: &[&models::DeviceId],
) -> anyhow::Result<usize> {
    let mut private_devices = devices.clone();
    for avoid_id in avoid {
        private_devices.remove(*avoid_id);
    }

    let solution_count = simple_graph::dfs_count::<models::DeviceId, models::Distance, models::Device>(
        private_devices
            .get(&start_id)
            .ok_or_else(|| anyhow::anyhow!("Start node not found"))?,
        &destination_id,
        private_devices.len(),
        |key| private_devices.get(key)
    );

    Ok(solution_count)
}

fn part_2_solutions_count(
    devices: &models::DeviceMap,
) -> anyhow::Result<usize> {
    let inverted_devices = models::invert_device_map(devices);

    let server_rack_id = parse::str_to_device_id(SERVER_RACK);
    let destination_id = parse::str_to_device_id(DESTINATION);
    let dac_id = parse::str_to_device_id(DAC);
    let fft_id = parse::str_to_device_id(FFT);

    let svr_to_dac_count =
        count_number_of_solutions(&devices, server_rack_id, dac_id, &[])
            .expect("Failed to count number of solutions from SVR to DAC");
    println!("Number of paths from SVR to DAC: {}", svr_to_dac_count);

    let svr_to_fft_count: usize =
        count_number_of_solutions(&inverted_devices, fft_id, server_rack_id, &[])
            .expect("Failed to count number of solutions from SVR to FFT");
    println!("Number of paths from SVR to FFT: {}", svr_to_fft_count);

    let dac_to_fft_count =
        count_number_of_solutions(&inverted_devices, fft_id, dac_id, &[])
            .expect("Failed to count number of solutions from DAC to FFT");
    println!("Number of paths from DAC to FFT: {}", dac_to_fft_count);

    let fft_to_dac_count =
        count_number_of_solutions(&inverted_devices, dac_id, fft_id, &[])
            .expect("Failed to count number of solutions from FFT to DAC");
    println!("Number of paths from FFT to DAC: {}", fft_to_dac_count);

    let dac_to_out_count =
        count_number_of_solutions(&devices, dac_id, destination_id, &[])
            .expect("Failed to count number of solutions from DAC to OUT");
    println!("Number of paths from DAC to OUT: {}", dac_to_out_count);

    let fft_to_out_count =
        count_number_of_solutions(&devices, fft_id, destination_id, &[])
            .expect("Failed to count number of solutions from FFT to OUT");
    println!("Number of paths from FFT to OUT: {}", fft_to_out_count);

    let svr_to_out_through_dac_count = 
        svr_to_fft_count
            .checked_mul(fft_to_dac_count)
            .expect("Overflow when calculating FFT to DAC through SVR")
        .checked_mul(dac_to_out_count)
        .expect("Overflow when calculating SVR to OUT through DAC");

    let svr_to_out_through_fft_count = 
        svr_to_dac_count
            .checked_mul(dac_to_fft_count)
            .expect("Overflow when calculating DAC to FFT through SVR")
        .checked_mul(fft_to_out_count)
        .expect("Overflow when calculating SVR to OUT through FFT");

    let solution_count = svr_to_out_through_dac_count
        .checked_add(svr_to_out_through_fft_count)
        .expect("Overflow when calculating total paths from SVR to OUT");

    #[cfg(feature = "assert-truth")]
    {
        assert_eq!(svr_to_dac_count, 1040248093572);
        assert_eq!(svr_to_fft_count, 5418);
        assert_eq!(dac_to_fft_count, 0);
        assert_eq!(fft_to_dac_count, 13733136);
        assert_eq!(dac_to_out_count, 3952);
        assert_eq!(fft_to_out_count, 3822779890610);
    }

    Ok(solution_count)
}

fn main() {
    let devices = build_devices(INPUT).expect("Failed to build devices from input");

    let start_id = parse::str_to_device_id(START);
    let destination_id = parse::str_to_device_id(DESTINATION);

    #[cfg(feature = "profile")]
    let start = Instant::now();
    '_part1: {
        let solution_count = count_number_of_solutions(&devices, start_id, destination_id, &[])
            .expect("Failed to count number of solutions for Part 1");

        println!("Part 1: Total number of distinct paths: {}", solution_count);
    }
    #[cfg(feature = "profile")]
    {
        let duration = start.elapsed();
        println!("Part 1 completed in: {:?}", duration);
    }

    #[cfg(feature = "profile")]
    let start = Instant::now();
    '_part2: {
        let solution_count =
            part_2_solutions_count(&devices)
                .expect("Failed to count number of solutions for Part 2");
        println!("Part 2: Total number of valid paths: {}", solution_count);
    }
    #[cfg(feature = "profile")]
    {
        let duration = start.elapsed();
        println!("Part 2 completed in: {:?}", duration);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    const PART1_INPUT: &'static str = "aaa: you hhh
                                      you: bbb ccc
                                      bbb: ddd eee
                                      ccc: ddd eee fff
                                      ddd: ggg
                                      eee: out
                                      fff: out
                                      ggg: out
                                      hhh: ccc fff iii
                                      iii: out";

    const PART2_INPUT: &'static str = "svr: aaa bbb
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
    fn test_parsing() {
        let devices = build_devices(PART1_INPUT).expect("Failed to build devices from test input");
        assert_eq!(devices.len(), 11);
        assert!(devices.contains_key(&parse::str_to_device_id("aaa")));
        assert!(devices.contains_key(&parse::str_to_device_id("you")));
        assert!(devices.contains_key(&parse::str_to_device_id("bbb")));
        assert!(devices.contains_key(&parse::str_to_device_id("ccc")));
        assert!(devices.contains_key(&parse::str_to_device_id("ddd")));
        assert!(devices.contains_key(&parse::str_to_device_id("eee")));
        assert!(devices.contains_key(&parse::str_to_device_id("fff")));
        assert!(devices.contains_key(&parse::str_to_device_id("ggg")));
        assert!(devices.contains_key(&parse::str_to_device_id("hhh")));
        assert!(devices.contains_key(&parse::str_to_device_id("iii")));
        assert!(devices.contains_key(&parse::str_to_device_id("out")));
    }

    #[test]
    fn test_part1() {
        let devices = build_devices(PART1_INPUT).expect("Failed to build devices from test input");

        let start_id = parse::str_to_device_id(START);
        let destination_id = parse::str_to_device_id(DESTINATION);
        let start_device = devices.get(&start_id).expect("Start device not found");
        let destination_id = destination_id;

        let get_node_by_key = |key: &models::DeviceId| devices.get(key);
        let mut dfs = simple_graph::Dfs::new(
            start_device,
            devices
                .get(&destination_id)
                .expect("Destination device not found"),
                devices.len()
        )
        .expect("Failed to create DFS instance");

        let solutions = {
            let mut sols = HashSet::new();
            while let Some(solution) = dfs.next_solution(get_node_by_key) {
                sols.insert((
                    solution
                        .0
                        .into_iter()
                        .map(|k| *k)
                        .collect::<Vec<models::DeviceId>>(),
                    solution.1,
                ));
            }
            sols
        };

        assert_eq!(solutions.len(), 5);
    }

    #[test]
    fn test_part2() {
        let devices = build_devices(PART2_INPUT).expect("Failed to build devices from test input");
        let solution_count =
            part_2_solutions_count(&devices)
                .expect("Failed to count number of solutions for Part 2");
        assert_eq!(solution_count, 2);
    }
}
