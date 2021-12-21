use sitewriter::{ChangeFreq, UrlEntry, UrlEntryBuilder};
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

fn main() {
    let urls = vec![
        UrlEntryBuilder::default()
            .loc("https://edgarluque.com/projects".parse().unwrap())
            .build()
            .unwrap(),
        UrlEntry {
            loc: "https://edgarluque.com/".parse().unwrap(),
            changefreq: Some(ChangeFreq::Daily),
            priority: Some(1.0),
            lastmod: Some(OffsetDateTime::now_utc()),
        },
        UrlEntry {
            loc: "https://edgarluque.com/blog".parse().unwrap(),
            changefreq: Some(ChangeFreq::Weekly),
            priority: Some(0.8),
            lastmod: Some(OffsetDateTime::now_utc()),
        },
        UrlEntry {
            loc: "https://edgarluque.com/blog/sitewriter".parse().unwrap(),
            changefreq: Some(ChangeFreq::Never),
            priority: Some(0.5),
            lastmod: Some(
                PrimitiveDateTime::new(
                    Date::from_calendar_date(2020, time::Month::November, 22).unwrap(),
                    Time::from_hms(15, 10, 15).unwrap(),
                )
                .assume_utc(),
            ),
        },
        UrlEntry {
            loc: "https://edgarluque.com/blog/some-future-post"
                .parse()
                .unwrap(),
            changefreq: Some(ChangeFreq::Never),
            priority: Some(0.5),
            lastmod: Some(
                PrimitiveDateTime::new(
                    Date::from_calendar_date(2020, time::Month::December, 5).unwrap(),
                    Time::from_hms(12, 30, 0).unwrap(),
                )
                .assume_utc(),
            ),
        },
        // Entity escaping
        UrlEntry {
            loc: "https://edgarluque.com/blog/test&id='<test>'"
                .parse()
                .unwrap(),
            changefreq: Some(ChangeFreq::Never),
            priority: Some(0.5),
            lastmod: Some(
                PrimitiveDateTime::new(
                    Date::from_calendar_date(2020, time::Month::December, 5).unwrap(),
                    Time::from_hms(12, 30, 0).unwrap(),
                )
                .assume_utc(),
            ),
        },
    ];

    let result = sitewriter::generate_str(&urls);
    println!("{}", result);
}
