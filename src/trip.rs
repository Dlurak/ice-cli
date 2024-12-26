use crate::consts::UNFETCHABLE_ERROR;
use ansi_term::{Colour, Style};
use chrono::{naive::NaiveDateTime, TimeZone};
use chrono_tz::Europe::Berlin;
use iceportal::{global_models::Track, ICEPortal};

fn fmt_time(time: Option<NaiveDateTime>, delay: Option<i32>) -> Option<String> {
    let time = time
        .and_then(|x| {
            let local = Berlin.from_local_datetime(&x);
            local.single()
        })
        .map(|x| x.format("%H:%M"));
    let delay = delay.and_then(|d| if d == 0 { None } else { Some(d) });

    match (time, delay) {
        (Some(time), None) => Some(format!("Ankunft {time}\n")),
        (Some(time), Some(delay)) => Some(format!("Ankunft {time} (+{delay})\n")),
        _ => None,
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

pub async fn handle_trip() {
    let response = ICEPortal::fetch_trip_info()
        .await
        .expect(UNFETCHABLE_ERROR)
        .trip;
    for stop in response.stops {
        let is_next_one = stop.station.eva_nr == response.stop_info.actual_next;

        let bold = Style::new().bold().underline();
        let name = if is_next_one {
            format!(
                "{} (Nächster Halt)",
                bold.fg(Colour::Red).paint(stop.station.name)
            )
        } else {
            bold.paint(stop.station.name).to_string()
        };
        let track = fmt_track(stop.track);
        let arrival = fmt_time(
            stop.timetable.actual_arrival_time,
            stop.timetable.arrival_delay,
        )
        .unwrap_or(String::new());
        let departure = fmt_time(
            stop.timetable.actual_departure_time,
            stop.timetable.departure_delay,
        )
        .unwrap_or(String::new());

        print!("{}\nGleis {}\n{}{}", name, track, arrival, departure);
    }
}
