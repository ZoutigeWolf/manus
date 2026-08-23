use crate::date;
use chrono::Utc;
use ics::Event;
use ics::properties::{DtEnd, DtStart, Location, Summary};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub token: Token,
    pub me: Me,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    #[serde(rename = "access_token")]
    pub access_token: String,

    #[serde(rename = "expires_in")]
    pub expires_in: u32,

    #[serde(rename = "token_type")]
    pub token_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Me {
    #[serde(rename = "id")]
    pub employee_id: String,

    #[serde(rename = "userName")]
    pub username: String,

    #[serde(rename = "fullname")]
    pub full_name: String,

    #[serde(rename = "nodeId")]
    pub node_id: String,

    #[serde(rename = "nodeCode")]
    pub node_code: String,

    #[serde(rename = "nodeName")]
    pub node_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HourCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "fullName")]
    pub full_name: String,

    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Department {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "isActive")]
    pub active: bool,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "nodeId")]
    pub node_id: String,

    #[serde(rename = "departmentId")]
    pub department_id: u32,

    #[serde(rename = "hourCodeId")]
    pub hour_code_id: u32,

    #[serde(rename = "startTime")]
    pub start_time: u32,

    #[serde(rename = "endTime")]
    pub end_time: u32,

    #[serde(rename = "totalTime")]
    pub total_time: f32,

    #[serde(rename = "notes")]
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vacation {
    #[serde(rename = "startTime")]
    pub start_time: u32,

    #[serde(rename = "endTime")]
    pub end_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "date")]
    pub date: u32,

    #[serde(rename = "entries")]
    pub entries: Vec<Entry>,

    #[serde(rename = "vacation")]
    pub vacation: Vec<Vacation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weekday {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "name")]
    pub name: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ManusData {
    #[serde(rename = "departments")]
    pub departments: HashMap<u32, Department>,

    #[serde(rename = "nodes")]
    pub nodes: HashMap<String, Node>,

    #[serde(rename = "hourCodes")]
    pub hour_codes: HashMap<u32, HourCode>,

    #[serde(rename = "schedule")]
    pub schedule: Vec<Schedule>,

    #[serde(rename = "weekdays")]
    pub days: Vec<Weekday>,
}

impl ManusData {
    pub fn parse_events(&self, account: &Account) -> Vec<Event<'static>> {
        let mut events = Vec::new();

        for schedule in self
            .schedule
            .iter()
            .filter(|&s| !s.entries.is_empty() && s.vacation.is_empty())
        {
            for entry in schedule.entries.iter() {
                let start = date::parse_datetime(schedule.date, entry.start_time);
                let end = date::parse_datetime(schedule.date, entry.end_time);

                let mut event = Event::new(entry.id.to_string(), date::to_string(Utc::now()));
                event.push(DtStart::new(date::to_string(start)));
                event.push(DtEnd::new(date::to_string(end)));

                let department = self.departments.get(&entry.department_id).unwrap();

                event.push(Summary::new(department.name.clone()));

                event.push(Location::new(format!(
                    "{} - {}",
                    self.nodes.get(&entry.node_id.clone()).unwrap().code.clone(),
                    self.nodes.get(&entry.node_id.clone()).unwrap().name.clone(),
                )));

                events.push(event);
            }
        }

        events
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Employee {
    #[serde(rename = "key")]
    pub id: String,

    #[serde(rename = "text")]
    pub name: String,

    #[serde(rename = "employeeTypeAbbr")]
    pub employee_type: String,

    #[serde(rename = "functionCode")]
    pub function_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DayScheduleData {
    #[serde(rename = "departments")]
    pub departments: HashMap<u32, Department>,

    #[serde(rename = "hourCodes")]
    pub hour_codes: HashMap<u32, HourCode>,

    #[serde(rename = "schedule")]
    pub schedule: HashMap<String, Schedule>,

    #[serde(rename = "employees")]
    pub employees: HashMap<String, Employee>,

    #[serde(rename = "nodes")]
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DayScheduleResponseEntry {
    #[serde(rename = "employee")]
    pub employee: Employee,

    #[serde(rename = "node")]
    pub node: Node,

    #[serde(rename = "department")]
    pub department: Department,

    #[serde(rename = "hour_code")]
    pub hour_code: HourCode,

    #[serde(rename = "start_time")]
    pub start_time: u32,

    #[serde(rename = "end_time")]
    pub end_time: u32,

    #[serde(rename = "total_time")]
    pub total_time: f32,

    #[serde(rename = "notes")]
    pub notes: String,
}