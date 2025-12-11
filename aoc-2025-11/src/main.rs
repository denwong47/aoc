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
