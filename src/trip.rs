use iceportal::{ICEPortal, global_models::Track};
use ansi_term::Style;
use crate::consts::UNFETCHABLE_ERROR;
use chrono::{
    TimeZone,
    naive::NaiveDateTime
};
use chrono_tz::Europe::Berlin;

fn fmt_time(time: Option<NaiveDateTime>, delay: Option<i32>) -> Option<String> {
    let time = time
        .map(|x| {
            let local = Berlin.from_local_datetime(&x);
            local.single().expect("Invalid date")
        })
        .map(|x| x.format("%H:%M"));
    let delay = delay.and_then(|d| 
        if d == 0 {
            None
        } else {
            Some(d)
        }
    );

    match (time, delay) {
        (Some(time), None) => Some(format!("Ankunft {time}\n")),
        (Some(time), Some(delay)) => Some(format!("Ankunft {time} (+{delay})\n")),
        _ => None
    }
}

fn fmt_track(track: Track) -> String {
    if track.actual == track.scheduled {
        track.scheduled
    } else {
        format!(
            "{} {}",
            Style::new().strikethrough().paint(track.scheduled),
            track.actual,
        )
    }
}

pub async fn handle_trip()  {
    let response = ICEPortal::fetch_trip_info().await.expect(UNFETCHABLE_ERROR).trip;
    for stop in response.stops {
        let name = Style::new().bold().underline().paint(stop.station.name);
        let track = fmt_track(stop.track);
        let arrival = fmt_time(stop.timetable.actual_arrival_time, stop.timetable.arrival_delay).unwrap_or(String::new());
        let departure = fmt_time(stop.timetable.actual_departure_time, stop.timetable.departure_delay).unwrap_or(String::new());

        print!(
            "{}\nGleis {}\n{}{}",
            name,
            track,
            arrival,
            departure
        );
    }
}
