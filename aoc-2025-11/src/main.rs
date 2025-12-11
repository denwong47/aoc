pub mod models;
use std::collections::HashMap;

mod input;
use input::INPUT;

const START: &'static models::DeviceId = &['y', 'o', 'u'];
const DESTINATION: &'static models::DeviceId = &['o', 'u', 't'];

const SERVER_RACK: &'static models::DeviceId = &['s', 'v', 'r'];
const DAC: &'static models::DeviceId = &['d', 'a', 'c'];
const FFT: &'static models::DeviceId = &['f', 'f', 't'];

fn build_devices(input: &str) -> anyhow::Result<HashMap<models::DeviceId, models::Device>> {
    let mut map = models::text_to_devices(input)?;

    map.insert(
        *DESTINATION,
        models::Device::new(*DESTINATION, std::iter::empty()),
    );
    Ok(map)
}

fn count_number_of_solutions(
    devices: &HashMap<models::DeviceId, models::Device>,
    start_id: &models::DeviceId,
    destination_id: &models::DeviceId,
    avoid: &[&models::DeviceId],
) -> anyhow::Result<usize> {
    let mut private_devices = devices.clone();
    for avoid_id in avoid {
        private_devices.remove(*avoid_id);
    }

    let get_node_by_key = |key: &models::DeviceId| private_devices.get(key);

    let mut dfs = simple_graph::Dfs::<models::DeviceId, models::Distance, models::Device>::new(
        private_devices
            .get(start_id)
            .ok_or_else(|| anyhow::anyhow!("Start node not found"))?,
        private_devices
            .get(destination_id)
            .ok_or_else(|| anyhow::anyhow!("Start node not found"))?,
        get_node_by_key,
    )?;

    let mut solution_count: usize = 0;
    while let Some(solution) = dfs.next_solution(get_node_by_key.clone()) {
        solution_count += 1;
        println!("Found solution #{:?}", solution.0.into_iter().map(|k| k.iter().collect::<String>()).collect::<Vec<String>>());
    }

    Ok(solution_count)
}

fn part_2_solutions_count(
    devices: &HashMap<models::DeviceId, models::Device>,
) -> anyhow::Result<usize> {
    let inverted_devices = models::invert_device_map(devices);

    let svr_to_dac_count =
        count_number_of_solutions(&inverted_devices, DAC, SERVER_RACK, &[&DESTINATION, FFT])
            .expect("Failed to count number of solutions from SVR to DAC");
    println!("Number of paths from SVR to DAC: {}", svr_to_dac_count);

    let svr_to_fft_count =
        count_number_of_solutions(&inverted_devices, FFT, SERVER_RACK, &[&DESTINATION, DAC])
            .expect("Failed to count number of solutions from SVR to FFT");
    println!("Number of paths from SVR to FFT: {}", svr_to_fft_count);

    let dac_to_fft_count =
        count_number_of_solutions(&inverted_devices, FFT, DAC, &[&DESTINATION, SERVER_RACK])
            .expect("Failed to count number of solutions from DAC to FFT");
    println!("Number of paths from DAC to FFT: {}", dac_to_fft_count);

    let fft_to_dac_count =
        count_number_of_solutions(&inverted_devices, DAC, FFT, &[&DESTINATION, SERVER_RACK])
            .expect("Failed to count number of solutions from FFT to DAC");
    println!("Number of paths from FFT to DAC: {}", fft_to_dac_count);

    let dac_to_out_count =
        count_number_of_solutions(&inverted_devices, DESTINATION, DAC, &[SERVER_RACK, FFT])
            .expect("Failed to count number of solutions from DAC to OUT");
    println!("Number of paths from DAC to OUT: {}", dac_to_out_count);

    let fft_to_out_count =
        count_number_of_solutions(&inverted_devices, DESTINATION, FFT, &[SERVER_RACK, DAC])
            .expect("Failed to count number of solutions from FFT to OUT");
    println!("Number of paths from FFT to OUT: {}", fft_to_out_count);

    let svr_to_out_through_dac_count = 
        fft_to_dac_count
            .checked_mul(svr_to_fft_count)
            .expect("Overflow when calculating FFT to DAC through SVR")
        .checked_mul(dac_to_out_count)
        .expect("Overflow when calculating SVR to OUT through DAC");

    let svr_to_out_through_fft_count = 
        dac_to_fft_count
            .checked_mul(svr_to_dac_count)
            .expect("Overflow when calculating DAC to FFT through SVR")
        .checked_mul(fft_to_out_count)
        .expect("Overflow when calculating SVR to OUT through FFT");

    let solution_count = svr_to_out_through_dac_count
        .checked_add(svr_to_out_through_fft_count)
        .expect("Overflow when calculating total paths from SVR to OUT");

    Ok(solution_count)
}

fn main() {
    let devices = build_devices(INPUT).expect("Failed to build devices from input");

    let destination_id = *DESTINATION;

    '_part1: {
        let solution_count = count_number_of_solutions(&devices, START, &destination_id, &[])
            .expect("Failed to count number of solutions for Part 1");

        println!("Part 1: Total number of distinct paths: {}", solution_count);
    }

    '_part2: {
        let solution_count =
            part_2_solutions_count(&devices)
                .expect("Failed to count number of solutions for Part 2");
        println!("Part 2: Total number of valid paths: {}", solution_count);
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
        assert!(devices.contains_key(&['a', 'a', 'a']));
        assert!(devices.contains_key(&['y', 'o', 'u']));
        assert!(devices.contains_key(&['b', 'b', 'b']));
        assert!(devices.contains_key(&['c', 'c', 'c']));
        assert!(devices.contains_key(&['d', 'd', 'd']));
        assert!(devices.contains_key(&['e', 'e', 'e']));
        assert!(devices.contains_key(&['f', 'f', 'f']));
        assert!(devices.contains_key(&['g', 'g', 'g']));
        assert!(devices.contains_key(&['h', 'h', 'h']));
        assert!(devices.contains_key(&['i', 'i', 'i']));
        assert!(devices.contains_key(&['o', 'u', 't']));
    }

    #[test]
    fn test_part1() {
        let devices = build_devices(PART1_INPUT).expect("Failed to build devices from test input");

        let start_device = devices.get(START).expect("Start device not found");
        let destination_id = *DESTINATION;

        let get_node_by_key = |key: &models::DeviceId| devices.get(key);
        let mut dfs = simple_graph::Dfs::new(
            start_device,
            devices
                .get(&destination_id)
                .expect("Destination device not found"),
            get_node_by_key,
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
