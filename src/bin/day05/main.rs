struct MapSegment {
    source: u64,
    dest: u64,
    len: u64,
}

impl MapSegment {
    fn new(source: u64, dest: u64, len: u64) -> Self {
        Self { source, dest, len }
    }

    // convert value from source category to destination category
    // returns None if value not within map
    fn convert(&self, val: u64) -> Option<u64> {
        // if value is in source category
        if (self.source..self.source + self.len).contains(&val) {
            // add the difference of the value and source start, and add that to destination start
            return Some(self.dest + val - self.source);
        }
        None
    }
}

struct SeedMap {
    segments: Vec<MapSegment>,
}

impl SeedMap {
    fn new(segments: Vec<MapSegment>) -> Self {
        Self { segments }
    }

    fn convert(&self, val: u64) -> u64 {
        // check all segments to see if value is mapped by any, if not return original value
        for r in &self.segments {
            if let Some(dest) = r.convert(val) {
                return dest;
            }
        }
        val
    }
}

struct Almanac {
    maps: Vec<SeedMap>,
}

impl Almanac {
    fn new(maps: Vec<SeedMap>) -> Self {
        Self { maps }
    }

    // continually map value through all categories from source category to destination
    fn convert(&self, mut val: u64) -> u64 {
        for m in &self.maps {
            val = m.convert(val);
        }
        val
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    // read seed values from first line
    let seeds: Vec<u64> = {
        let seed_line = lines.next().unwrap();
        let start = seed_line.find(':').unwrap();
        seed_line[start + 2..]
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect()
    };
    let seed_ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|c| (c[0], c[1])).collect();
    lines.next().unwrap(); //skip blank line
    let mut maps = vec![];
    loop {
        let mut segments = vec![];
        if lines.next().is_none() {
            //skip map title line, or break if no lines left
            break;
        }
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // stop reading once we hit blank line between maps (or iter ends)
            }
            // read 3 nums from line and build MapSegment
            let nums: Vec<_> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            segments.push(MapSegment::new(nums[1], nums[0], nums[2]));
        }
        // build map from segments
        maps.push(SeedMap::new(segments));
    }
    // run all seeds through every map, and return lowest final value
    let almanac = Almanac::new(maps);
    let min_location = seeds.into_iter().map(|s| almanac.convert(s)).min().unwrap();
    println!("part 1: {min_location}");

    // takes a computer less time to do this than it takes for me to write a smarter algorithm
    let mut min_location = u64::MAX;
    for (start, len) in seed_ranges {
        for seed in start..start + len {
            let seed_location = almanac.convert(seed);
            min_location = min_location.min(seed_location);
        }
    }
    println!("part 2: {min_location}");
}
