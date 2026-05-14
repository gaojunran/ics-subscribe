use icalendar::{Calendar, Component, Event, EventLike, Alarm, Trigger, UnvalidatedRRule};
use crate::config::{Config, EventConfig};
use chrono::{DateTime, Utc, Duration};

pub fn build_calendar(config: &Config) -> String {
    let mut calendar = Calendar::new();
    if let Some(name) = &config.calendar_name {
        calendar.name(name);
    }
    for event_config in &config.events {
        calendar.push(build_event(event_config));
    }
    calendar.done().to_string()
}

fn build_event(config: &EventConfig) -> Event {
    let start: DateTime<Utc> = config.start_time.parse()
        .expect("invalid start_time format, expected RFC 3339");

    let mut event = Event::new();
    event.summary(&config.title).starts(start);

    if let Some(end_str) = &config.end_time {
        let end: DateTime<Utc> = end_str.parse()
            .expect("invalid end_time format, expected RFC 3339");
        event.ends(end);
    }
    if let Some(loc) = &config.location {
        event.location(loc);
    }
    if let Some(desc) = &config.description {
        event.description(desc);
    }
    // Parse RRULE string if present
    if let Some(rrule_str) = &config.rrule {
        let rrule: UnvalidatedRRule = rrule_str
            .parse()
            .expect("invalid rrule format — expected RRULE format like FREQ=MONTHLY;COUNT=6");
        event
            .recurrence(rrule)
            .expect("failed to set recurrence rule");
    }

    // Parse reminder minutes if present
    if let Some(minutes) = &config.reminder {
        let duration = Duration::minutes(*minutes);
        let alarm = Alarm::display(
            &config.title,
            Trigger::before_start(duration),
        );
        event.alarm(alarm);
    }

    event.done()
}
