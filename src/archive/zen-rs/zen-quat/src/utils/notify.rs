use cached::proc_macro::cached;
use notify_rust::Notification;
use time::{format_description, Duration, OffsetDateTime};
use tws_rs::contracts::Contract;
use zen_core::objects::chan::Symbol;
use zen_core::objects::trade::{Event, Factor, Signal};

pub struct Notify {}

impl Notify {
    pub fn notify_signal(symbol: &Symbol, dt: OffsetDateTime, signal: Signal) {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
        notify(
            format!("{} - {}", symbol, signal.key()),
            Some(dt.format(&format).unwrap()),
            signal.value(),
            dt,
            true,
        );
    }

    pub fn notify_event(contract: &Contract, dt: OffsetDateTime, event: &Event, factor: &Factor) {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
        notify(
            format!("{} - {}", contract.symbol, event.name.clone()),
            Some(dt.format(&format).unwrap()),
            factor
                .signals_all
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
                .join("\n"),
            dt,
            false,
        );
    }
}

#[cached(size = 1000)]
fn notify(
    title: String,
    subtitle: Option<String>,
    body: String,
    dt: OffsetDateTime,
    realtime: bool,
) {
    if !realtime || dt > OffsetDateTime::now_utc() - Duration::hours(2) {
        Notification::new()
            .summary(title.as_str())
            .subtitle(subtitle.unwrap_or("".to_string()).as_str())
            .body(body.as_str())
            .sound_name("Submarine")
            .show()
            .unwrap();
    }
}
