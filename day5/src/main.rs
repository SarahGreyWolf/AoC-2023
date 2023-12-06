use std::env;
use std::fs::File;
use std::io::Read;
use std::thread;

#[derive(Debug, Default, Clone)]
struct Set {
    map_id: usize,
    destination_start: usize,
    source_start: usize,
    range: usize,
}

impl Set {
    pub fn read_line(line: &str, map_id: usize) -> Option<Set> {
        let destination_start_string: String =
            line.chars().take_while(|c| c.is_digit(10)).collect();
        // If the first numbers is empty we can assume there are no more numbers on this line
        if destination_start_string.is_empty() {
            return None;
        }
        let source_start_string: String = line
            .chars()
            .skip(destination_start_string.len() + 1)
            .take_while(|c| c.is_digit(10))
            .collect();
        let range_string: String = line
            .chars()
            .skip(destination_start_string.len() + 1 + source_start_string.len() + 1)
            .take_while(|c| c.is_digit(10))
            .collect();
        let destination_start = usize::from_str_radix(&destination_start_string, 10).unwrap();
        let source_start = usize::from_str_radix(&source_start_string, 10).unwrap();
        let range = usize::from_str_radix(&range_string, 10).unwrap();

        Some(Set {
            map_id,
            destination_start,
            source_start,
            range,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("Usage: {} <filepath>", args.next().unwrap());
    }
    // Skip first arg as that is the executable
    args.next().unwrap();
    let mut file = File::open(args.next().unwrap())?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut lines = input.lines();
    // Get first line
    let seeds_line = lines.next().unwrap();
    let seeds = get_seeds(seeds_line);

    println!("Seeds: {seeds:?}");

    let mut map_id = 0;
    let mut sets = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some(set) = Set::read_line(line, map_id) {
            sets.push(set);
        } else {
            map_id += 1;
        }
    }
    let mut maps: Vec<Vec<Set>> = vec![];
    let mut current_map = 0;
    for set in sets {
        if current_map != set.map_id {
            current_map += 1;
            maps.push(vec![]);
        }
        maps[set.map_id - 1].push(set);
    }

    let mut largest = 0;
    for set_map in &maps {
        for set in set_map {
            let destination_range = set.destination_start..set.destination_start + set.range;
            let source_range = set.source_start..set.source_start + set.range;
            let largest_in_set = if destination_range.clone().last().unwrap()
                > source_range.clone().last().unwrap()
            {
                destination_range.clone().last().unwrap()
            } else {
                source_range.clone().last().unwrap()
            };
            if largest_in_set > largest {
                largest = largest_in_set;
            }
        }
    }
    // PART 1
    let mut closest_location = largest;
    for seed in &seeds {
        let seed = *seed as usize;
        let soil = get_at_index(&maps[0], seed);
        let fertilizer = get_at_index(&maps[1], soil);
        let water = get_at_index(&maps[2], fertilizer);
        let light = get_at_index(&maps[3], water);
        let temperature = get_at_index(&maps[4], light);
        let humidity = get_at_index(&maps[5], temperature);
        let location = get_at_index(&maps[6], humidity);
        // println!("For seed {seed}, soil {soil}, fertilizer {fertilizer}, water {water}, light {light}, temperature {temperature}, humidity {humidity}, location {location}");
        if location < closest_location {
            closest_location = location;
        }
    }
    println!("The closest seed location is {}", closest_location);

    // PART 2
    let mut seed_ranges: Vec<(u32, u32)> = vec![];
    let mut index = 0;
    for seed in seeds {
        if index == 0 {
            seed_ranges.push((seed, 0));
            index += 1;
        } else {
            seed_ranges.last_mut().unwrap().1 = seed;
            index = 0;
        }
    }
    let mut closest_location = largest * 1000;

    let mut threads = vec![];

    for range in seed_ranges {
        let maps_clone = maps.clone();
        let thread = thread::spawn(move || {
            let mut our_closest = largest;
            println!(
                "Created thread for range {} to {}",
                range.0,
                range.0 + range.1
            );
            for seed in range.0..range.0 + range.1 {
                let seed = seed as usize;
                let soil = get_at_index(&maps_clone[0], seed);
                let fertilizer = get_at_index(&maps_clone[1], soil);
                let water = get_at_index(&maps_clone[2], fertilizer);
                let light = get_at_index(&maps_clone[3], water);
                let temperature = get_at_index(&maps_clone[4], light);
                let humidity = get_at_index(&maps_clone[5], temperature);
                let location = get_at_index(&maps_clone[6], humidity);
                // println!("For seed {seed}, soil {soil}, fertilizer {fertilizer}, water {water}, light {light}, temperature {temperature}, humidity {humidity}, location {location}");
                if location < our_closest {
                    our_closest = location;
                }
            }
            println!(
                "Finished thread for range {} to {} with {our_closest}",
                range.0,
                range.0 + range.1
            );
            our_closest
        });
        threads.push(thread);
    }

    for thread in threads {
        let thread_closest = thread.join().unwrap();
        if thread_closest < closest_location {
            closest_location = thread_closest;
        }
    }
    println!("The closest seed location is {}", closest_location);

    Ok(())
}

fn get_seeds(line: &str) -> Vec<u32> {
    let mut seeds = vec![];
    let list_start = line.find(": ").unwrap();
    let seed_numbers_string = line.split_at(list_start + 2).1;
    let split_seed_numbers = seed_numbers_string.split(' ');
    for numbers in split_seed_numbers {
        let value_string: String = numbers.chars().take_while(|c| c.is_digit(10)).collect();
        let value = u32::from_str_radix(&value_string, 10).unwrap();
        seeds.push(value);
    }
    seeds
}

fn get_at_index(sets: &[Set], index: usize) -> usize {
    for set in sets {
        if index >= set.source_start && index < set.source_start + set.range {
            let depth_into_source = index - set.source_start;
            return set.destination_start + depth_into_source;
        }
    }
    index
}
