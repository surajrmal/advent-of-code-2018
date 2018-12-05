#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use failure::Error;
use nom::types::CompleteStr as Input;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventType {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

impl PartialOrd for EventType {
    fn partial_cmp(&self, _other: &EventType) -> Option<Ordering> {
	Some(Ordering::Equal)
    }
}

impl Ord for EventType {
    fn cmp(&self, _other: &EventType) -> Ordering {
	Ordering::Equal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    year : usize,
    month : usize,
    day : usize,
    hour : usize,
    minute : usize,
    event_type: EventType,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	named!(integer(Input) -> usize, map_res!(nom::digit, |d: Input| d.parse()));

	named!(event_type_parser(Input) -> EventType,
	  alt!( do_parse!(tag!("Guard #") >> guard: ws!(integer) >> tag!("begins shift") >> (EventType::BeginShift(guard)))
	      | do_parse!(tag!("wakes up") >> (EventType::WakeUp))
	      | do_parse!(tag!("falls asleep") >> (EventType::FallAsleep))

	  )
	);

	named!(event_parser(Input) -> Event,
	  do_parse!(
	                            tag!("[") >>
	    year:   ws!(integer) >> tag!("-") >>
	    month:  ws!(integer) >> tag!("-") >>
	    day:    ws!(integer) >>
	    hour:   ws!(integer) >> tag!(":") >>
	    minute: ws!(integer) >> tag!("]") >>
	    event_type: ws!(event_type_parser) >>
	    (Event { year, month, day, hour, minute, event_type })
	  )
	);

	event_parser(Input(s)).map(|(_s, c)| c).map_err(|e| format_err!("{}", e))
    }
}

fn parse_input(input: &str) -> Result<Vec<Event>, Error> {
    input.split('\n')
	 .filter(|line| !line.is_empty())
	 .map(str::parse::<Event>)
 	 .collect::<Result<Vec<_>, Error>>()
}

fn part1(mut events: Vec<Event>) -> usize {
    events.sort();
    let mut map = HashMap::new();
    let mut current_guard = None;
    let mut fall_asleep_minute = None;
    for event in events {
	match event.event_type {
	    EventType::BeginShift(guard) => { current_guard = Some(guard); },
	    EventType::FallAsleep => { fall_asleep_minute = Some(event.minute); },
	    EventType::WakeUp => {
		assert!(fall_asleep_minute.is_some());
		assert!(current_guard.is_some());
		assert!(fall_asleep_minute.unwrap() < event.minute);
		let guard = map.entry(current_guard.unwrap()).or_insert([0; 60]);
		for min in fall_asleep_minute.unwrap()..event.minute {
		    (*guard)[min as usize] += 1;
		}
		fall_asleep_minute = None;
	    },
	}
    }
    let (guard, _) = map.iter().fold((0, 0), |(max_guard, max_minutes), (guard, minutes)| {
	let minutes = minutes.iter().sum();
	if minutes > max_minutes {
	    (*guard, minutes)
	} else {
	    (max_guard, max_minutes)
	}
    });
    let (minute, _) = map[&guard].into_iter().enumerate().fold((0, 0), |(max_minute, max_times), (minute, times)| {
	if times > &max_times {
	    (minute, *times)
	} else {
	    (max_minute, max_times)
	}
    });
    guard * minute
}

fn part2(mut events: Vec<Event>) -> usize {
    events.sort();
    let mut map = HashMap::new();
    let mut current_guard = None;
    let mut fall_asleep_minute = None;
    for event in events {
	match event.event_type {
	    EventType::BeginShift(guard) => { current_guard = Some(guard); },
	    EventType::FallAsleep => { fall_asleep_minute = Some(event.minute); },
	    EventType::WakeUp => {
		assert!(fall_asleep_minute.is_some());
		assert!(current_guard.is_some());
		assert!(fall_asleep_minute.unwrap() < event.minute);
		let guard = map.entry(current_guard.unwrap()).or_insert([0; 60]);
		for min in fall_asleep_minute.unwrap()..event.minute {
		    (*guard)[min as usize] += 1;
		}
	    },
	}
    }
    let (guard, minute, _) = map.into_iter().fold((0, 0, 0), |(max_guard, max_minute, max_times), (guard, minutes)| {
	let (minute, times) = minutes.into_iter().enumerate().fold((0, 0), |(max_minute, max_times), (minute, times)| {
	    if times > &max_times {
		(minute, *times)
	    } else {
		(max_minute, max_times)
	    }
	});
	if times > max_times {
	    (guard, minute, times)
	} else {
	    (max_guard, max_minute, max_times)
	}
    });
    guard * minute
}


fn main() {
    let input = include_str!("input.txt");
    let events = parse_input(input).unwrap();
    println!("part1: {}", part1(events.clone()));
    println!("part2: {}", part2(events));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
	let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up";
        assert_eq!(
	    vec![
		Event{year: 1518, month: 11, day: 01, hour: 0, minute: 0, event_type: EventType::BeginShift(10)},
		Event{year: 1518, month: 11, day: 01, hour: 0, minute: 5, event_type: EventType::FallAsleep},
		Event{year: 1518, month: 11, day: 01, hour: 0, minute: 25, event_type: EventType::WakeUp},
		Event{year: 1518, month: 11, day: 01, hour: 0, minute: 30, event_type: EventType::FallAsleep},
		Event{year: 1518, month: 11, day: 01, hour: 0, minute: 55, event_type: EventType::WakeUp},
		Event{year: 1518, month: 11, day: 01, hour: 23, minute: 58, event_type: EventType::BeginShift(99)},
		Event{year: 1518, month: 11, day: 02, hour: 0, minute: 40, event_type: EventType::FallAsleep},
		Event{year: 1518, month: 11, day: 02, hour: 0, minute: 50, event_type: EventType::WakeUp},
	    ],
	    parse_input(&input).unwrap()
	);
    }

    #[test]
    fn part1_test() {
	let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
	let events = parse_input(&input).unwrap();
	assert_eq!(part1(events), 240);
    }

    #[test]
    fn part2_test() {
	let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
	let events = parse_input(&input).unwrap();
	assert_eq!(part2(events), 4455);
    }
}
