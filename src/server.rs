use crate::{models, request};
use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use ics::ICalendar;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get_calendar_for_user(
    Path(username): Path<String>,
    Extension(accounts): Extension<Arc<RwLock<Vec<models::Account>>>>,
) -> impl IntoResponse {
    let accounts = accounts.read().await;

    let Some(user) = accounts.iter().find(|&a| a.username == username) else {
        println!(
            "Calendar generation failed, no account found for username: {}",
            username
        );

        return (StatusCode::NOT_FOUND, "Account not found").into_response();
    };

    let mut cal = ICalendar::new("2.0", "manus-scraper");

    let mut tasks = FuturesUnordered::new();

    let now = Utc::now();

    let current_week = now.iso_week().week();
    let current_year = now.iso_week().year();

    let week_start = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon).unwrap();

    for offset in -12..=4 {
        let shifted = week_start + Duration::weeks(offset);

        let year = shifted.iso_week().year() as u32;
        let week = shifted.iso_week().week();

        tasks.push(async move {
            request::get_manus_data(
                &user.me.node_id,
                &user.me.employee_id,
                &year,
                &week,
                &user.token,
            )
            .await
            .ok()
            .map(|data| data.parse_events(&user))
        });
    }

    while let Some(events_opt) = tasks.next().await {
        if let Some(events) = events_opt {
            for event in events {
                cal.add_event(event);
            }
        }
    }

    let ics_string = cal.to_string();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/calendar".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"calendar.ics\"".parse().unwrap(),
    );

    println!("Calendar generation success, username: {}", username);

    (headers, ics_string).into_response()
}
