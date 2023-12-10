use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, Read},
    vec,
};

#[derive(Debug)]
enum MapTypes {
    SeedToSoil,
    SoilToFertilizer,
    FeterilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Entry {
    source_start: u64,
    dest_start: u64,
    range: u64,
    optimal: Option<u64>,
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        Entry { ..*self }
    }
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<Entry>,
    soil_to_fertilizer: Vec<Entry>,
    feterilizer_to_water: Vec<Entry>,
    water_to_light: Vec<Entry>,
    light_to_temperature: Vec<Entry>,
    temperature_to_humidity: Vec<Entry>,
    humidity_to_location: Vec<Entry>,
}

impl Maps {
    fn get_next_map(&self, map_type: &MapTypes) -> Option<MapTypes> {
        match map_type {
            MapTypes::SeedToSoil => Some(MapTypes::SoilToFertilizer),
            MapTypes::SoilToFertilizer => Some(MapTypes::FeterilizerToWater),
            MapTypes::FeterilizerToWater => Some(MapTypes::WaterToLight),
            MapTypes::WaterToLight => Some(MapTypes::LightToTemperature),
            MapTypes::LightToTemperature => Some(MapTypes::TemperatureToHumidity),
            MapTypes::TemperatureToHumidity => Some(MapTypes::HumidityToLocation),
            MapTypes::HumidityToLocation => None,
        }
    }

    fn get_map(&self, map_type: &MapTypes) -> &Vec<Entry> {
        match map_type {
            MapTypes::SeedToSoil => &self.seed_to_soil,
            MapTypes::SoilToFertilizer => &self.soil_to_fertilizer,
            MapTypes::FeterilizerToWater => &self.feterilizer_to_water,
            MapTypes::WaterToLight => &self.water_to_light,
            MapTypes::LightToTemperature => &self.light_to_temperature,
            MapTypes::TemperatureToHumidity => &self.temperature_to_humidity,
            MapTypes::HumidityToLocation => &self.humidity_to_location,
        }
    }

    fn get_next(&self, map_type: &MapTypes, value: u64) -> (Option<MapTypes>, u64) {
        let map = self.get_map(map_type);

        let next_map = self.get_next_map(map_type);

        for entry in map {
            if entry.source_start <= value && value < entry.source_start + entry.range {
                return (next_map, entry.dest_start + value - entry.source_start);
            }
        }

        (next_map, value)
    }

    fn split_intervals(&self, map_type: &MapTypes, start: u64, range: u64) -> Vec<Entry> {
        let mut new_arr = vec![];
        let map = self.get_map(&map_type);
        for entry in map {
            if entry.source_start < start && start + range < entry.source_start + entry.range {
                // this is the case where we have an interval contained within an interval
                // it looks something like this
                //
                // |--------------| real
                //     |-----| input
                //
                // in this case we'll split it like this
                //
                //   a    b     c
                // |---|-----|----| real
                //     |-----| input

                let a = Entry {
                    source_start: entry.source_start,
                    dest_start: entry.dest_start,
                    range: start - entry.source_start,
                    optimal: None,
                };

                let b = Entry {
                    source_start: start,
                    dest_start: entry.dest_start + a.range,
                    range,
                    optimal: None,
                };

                let c = Entry {
                    source_start: start + range,
                    dest_start: entry.dest_start + a.range + b.range,
                    range: entry.source_start + entry.range - (a.source_start + a.range + b.range),
                    optimal: None,
                };

                new_arr.push(a);
                new_arr.push(b);
                new_arr.push(c);
            } else if entry.source_start < start && start < entry.source_start + entry.range {
                // this is the case where our starting point ins contained in the other interval
                // but our end is not
                //
                // |---------| real
                //       |--------| input
                //
                // we will partition it like this
                //
                //    a    b
                // |-----|---| real
                //       |--------| input

                let a = Entry {
                    source_start: entry.source_start,
                    dest_start: entry.dest_start,
                    range: start - entry.source_start,
                    optimal: None,
                };

                let b = Entry {
                    source_start: start,
                    dest_start: entry.dest_start + a.range,
                    range: entry.source_start + entry.range - (a.source_start + a.range),
                    optimal: None,
                };

                new_arr.push(a);
                new_arr.push(b);
            } else if entry.source_start < start + range
                && start + range < entry.source_start + entry.range
            {
                // this is the case where our end point is contained in the other interval
                // but our start is not
                //
                //     |----------------| real
                // |----------| input
                //
                // we will partition it like this
                //         a       b
                //     |------|---------| real
                // |----------| input

                let a = Entry {
                    source_start: entry.source_start,
                    dest_start: entry.dest_start,
                    range: start + range - entry.source_start,
                    optimal: None,
                };
                let b = Entry {
                    source_start: start + range,
                    dest_start: entry.dest_start + a.range,
                    range: entry.source_start + entry.range - (a.source_start + a.range),
                    optimal: None,
                };

                new_arr.push(a);
                new_arr.push(b);
            } else {
                new_arr.push(entry.clone());
            }
        }

        // assert that we can get from start to end with all the
        // new intervals we added/split
        // this means that we have no missing intervals
        //
        // if this was a graphing problem, which it probably is
        // and i was too stupid to figure it out, then this would mean that
        // there exists a path from start to end
        let mut required = BTreeSet::new();
        for entry in &new_arr {
            if start <= entry.source_start && entry.source_start + entry.range <= start + range {
                required.insert((entry.source_start, entry.range));
            }
        }

        // if we do find something missing, just add it
        let mut prev_start = start + range;
        for entry in required.clone().iter().rev() {
            if prev_start != entry.0 + entry.1 {
                let e = Entry {
                    source_start: entry.0 + entry.1,
                    dest_start: entry.0 + entry.1,
                    range: prev_start - entry.0 - entry.1,
                    optimal: None,
                };

                required.insert((e.source_start, e.range));

                new_arr.push(e);
            }
            prev_start = entry.0;
        }

        // at the end, if we are missing the first interval, just add it individually
        if prev_start != start {
            let e = Entry {
                source_start: start,
                dest_start: start,
                range: prev_start - start,
                optimal: None,
            };

            required.insert((e.source_start, e.range));

            new_arr.push(e);
        }

        let mut prev_start = start + range;
        for entry in required.iter().rev() {
            assert!(prev_start == entry.0 + entry.1);
            prev_start = entry.0;
        }
        assert!(prev_start == start);

        new_arr
    }

    fn build_optimal_values(&mut self) {
        for entry in &mut self.humidity_to_location {
            entry.optimal = Some(entry.dest_start);
        }

        for entry in &mut self.temperature_to_humidity {
            entry.optimal = get_optimal(&self.humidity_to_location, entry.dest_start, entry.range);
        }

        for entry in &mut self.light_to_temperature {
            entry.optimal =
                get_optimal(&self.temperature_to_humidity, entry.dest_start, entry.range);
        }

        for entry in &mut self.water_to_light {
            entry.optimal = get_optimal(&self.light_to_temperature, entry.dest_start, entry.range);
        }

        for entry in &mut self.feterilizer_to_water {
            entry.optimal = get_optimal(&self.water_to_light, entry.dest_start, entry.range);
        }

        for entry in &mut self.soil_to_fertilizer {
            entry.optimal = get_optimal(&self.feterilizer_to_water, entry.dest_start, entry.range);
        }

        for entry in &mut self.seed_to_soil {
            entry.optimal = get_optimal(&self.soil_to_fertilizer, entry.dest_start, entry.range);
        }
    }

    fn insert(&mut self, map_type: &MapTypes, entry: Entry) {
        match map_type {
            MapTypes::SeedToSoil => self.seed_to_soil.push(entry),
            MapTypes::SoilToFertilizer => self.soil_to_fertilizer.push(entry),
            MapTypes::FeterilizerToWater => self.feterilizer_to_water.push(entry),
            MapTypes::WaterToLight => self.water_to_light.push(entry),
            MapTypes::LightToTemperature => self.light_to_temperature.push(entry),
            MapTypes::TemperatureToHumidity => self.temperature_to_humidity.push(entry),
            MapTypes::HumidityToLocation => self.humidity_to_location.push(entry),
        };
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        let mut next_map = Some(MapTypes::SeedToSoil);

        while let Some(ref map) = next_map {
            (next_map, value) = self.get_next(map, value);
        }

        value
    }
}

fn get_optimal(next_map: &Vec<Entry>, start: u64, range: u64) -> Option<u64> {
    let mut optimal: Option<u64> = None;

    for entry in next_map {
        if start <= entry.source_start && entry.source_start + entry.range <= start + range {
            optimal = match optimal {
                Some(val) => Some(val.min(entry.optimal.unwrap())),
                None => entry.optimal,
            };
        }
    }

    optimal
}

fn main() -> io::Result<()> {
    let mut file = File::open("./input")?;
    let mut s = String::new();

    _ = file.read_to_string(&mut s);
    let lines: Vec<&str> = s.split("\n").collect();

    let mut maps = Maps {
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        feterilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let seeds: Vec<u64> = lines
        .first()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map_type = MapTypes::SeedToSoil;
    for line in &lines[2..] {
        match *line {
            "seed-to-soil map:" => map_type = MapTypes::SeedToSoil,
            "soil-to-fertilizer map:" => map_type = MapTypes::SoilToFertilizer,
            "fertilizer-to-water map:" => map_type = MapTypes::FeterilizerToWater,
            "water-to-light map:" => map_type = MapTypes::WaterToLight,
            "light-to-temperature map:" => map_type = MapTypes::LightToTemperature,
            "temperature-to-humidity map:" => map_type = MapTypes::TemperatureToHumidity,
            "humidity-to-location map:" => map_type = MapTypes::HumidityToLocation,
            "" => {}
            _ => {
                let parts: Vec<u64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
                maps.insert(
                    &map_type,
                    Entry {
                        dest_start: parts[0],
                        source_start: parts[1],
                        range: parts[2],
                        optimal: None,
                    },
                );
            }
        };
    }

    let mut part1: u64 = u64::MAX;
    for seed in seeds.iter() {
        let location = maps.seed_to_location(*seed);
        part1 = if location < part1 { location } else { part1 }
    }
    println!("part1: {:?}", part1);

    // this was as slow as i expected
    // let mut part2_naive: u64 = u64::MAX;
    // for i in (0..seeds.len()).step_by(2) {
    //     for j in seeds[i]..seeds[i] + seeds[i + 1] {
    //         let location = maps.seed_to_location(j);
    //         part2_naive = if location < part2_naive {
    //             location
    //         } else {
    //             part2_naive
    //         }
    //     }
    // }
    // println!("{:?}", part2_naive);

    // maybe use a loop idiot
    {
        for i in (0..seeds.len()).step_by(2) {
            maps.seed_to_soil = maps.split_intervals(&MapTypes::SeedToSoil, seeds[i], seeds[i + 1]);
        }

        for entry in &maps.seed_to_soil {
            maps.soil_to_fertilizer =
                maps.split_intervals(&MapTypes::SoilToFertilizer, entry.dest_start, entry.range);
        }

        for entry in &maps.soil_to_fertilizer {
            maps.feterilizer_to_water =
                maps.split_intervals(&MapTypes::FeterilizerToWater, entry.dest_start, entry.range);
        }

        for entry in &maps.feterilizer_to_water {
            maps.water_to_light =
                maps.split_intervals(&MapTypes::WaterToLight, entry.dest_start, entry.range);
        }

        for entry in &maps.water_to_light {
            maps.light_to_temperature =
                maps.split_intervals(&MapTypes::LightToTemperature, entry.dest_start, entry.range);
        }

        for entry in &maps.light_to_temperature {
            maps.temperature_to_humidity = maps.split_intervals(
                &MapTypes::TemperatureToHumidity,
                entry.dest_start,
                entry.range,
            );
        }

        for entry in &maps.temperature_to_humidity {
            maps.humidity_to_location =
                maps.split_intervals(&MapTypes::HumidityToLocation, entry.dest_start, entry.range);
        }
    }

    maps.build_optimal_values();
    let mut part2: u64 = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        part2 = part2.min(get_optimal(&maps.seed_to_soil, seeds[i], seeds[i + 1]).unwrap());
    }

    println!("part2: {:?}", part2);

    Ok(())
}
