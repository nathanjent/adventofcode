use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

pub fn obsecurity_1(file: &str) -> usize {
    let (sum, _) = parse_rooms(file);
    sum
}

pub fn obsecurity_2(file: &str, room_search_term: &str) -> usize {
    let (_, rooms) = parse_rooms(file);
    let mut names = Vec::new();
    for room in rooms {
        let mut words = Vec::new();
        for word in room.encrypted {
            let mut shifted = Vec::new();
            for c in word.chars() {
                let mut c = c as u8;
                let shift = (room.id % 26) as u8;
                c = c + shift;
                if c > 'z' as u8 {
                    c = 'a' as u8 - 1 + (c - 'z' as u8);
                }
                shifted.push(c as char);
            }
            words.push(shifted.iter().cloned().collect::<String>());
        }
        let name = words.into_iter()
            .map(|mut s| {
                s.push(' ');
                s
            })
            .collect::<String>();
        if name.find(room_search_term).is_some() {
            return room.id;
        }
        names.push(name);
    }
    println!("{:?}", names);
    42
}

fn parse_rooms(file: &str) -> (usize, Vec<Room>) {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut rooms = Vec::new();
    let mut sum = 0;
    for line in lines {
        let mut words = line.split("-").collect::<Vec<&str>>();

        if let Some(id_checksum) = words.pop() {
            if let Some(i) = id_checksum.find("[") {
                let (id, checksum) = id_checksum.split_at(i);
                let checksum = checksum.trim_matches(|c| c == '[' || c == ']')
                    .split("")
                    .flat_map(str::chars)
                    .collect::<Vec<char>>();
                // println!("{}", id);
                // println!("{:?}", checksum);

                let letters = words.iter().flat_map(|s| s.chars()).collect::<Vec<char>>();
                // println!("{:?}", letters);

                let mut counts = BTreeMap::new();
                for c in letters.iter() {
                    *counts.entry(*c).or_insert(0) += 1;
                }
                // println!("{:?}", counts);

                let mut stack = counts.iter().collect::<Vec<(&char, &usize)>>();
                // sort high to low
                stack.sort_by(|a, b| b.1.cmp(a.1));
                // println!("{:?}", stack);

                let (checkedsum_full, _): (Vec<&char>, Vec<&usize>) = stack.iter().cloned().unzip();
                let checkedsum = checkedsum_full[..5]
                    .iter()
                    .cloned()
                    .cloned()
                    .collect::<Vec<char>>();
                // println!("{:?}", checkedsum);

                if let Ok(id) = id.parse::<usize>() {
                    if checksum == checkedsum {
                        // Collect verified rooms
                        rooms.push(Room {
                            id: id,
                            encrypted: words.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                        });
                        sum += id;
                    }
                }
            }
        };
        // println!("");
    }
    (sum, rooms)
}

struct Room {
    encrypted: Vec<String>,
    id: usize,
}
