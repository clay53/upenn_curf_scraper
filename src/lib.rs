#![feature(let_chains)]
use std::{collections::HashMap, path::Path};

use serde::{Serialize, Deserialize};

pub const CURF_ROOT: &'static str = "https://curf.upenn.edu";

pub mod update_auth;
pub mod scrape_links;
pub mod scrape_opportunities;
pub mod process_opportunities;
pub mod filter;

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub simple_saml_auth_token: String,
    pub simple_saml_session_id: String,
    pub sse_pair: (String, String),
}

pub async fn load_auth<P: AsRef<Path>>(path: P) -> tokio::io::Result<Auth> {
    Ok(ron::from_str(&tokio::fs::read_to_string(path.as_ref().join("auth.ron")).await?).unwrap())
}

pub async fn load_cookie_string<P: AsRef<Path>>(path: P) -> tokio::io::Result<String> {
    let auth = load_auth(path).await?;
    Ok(format!("SimpleSAMLAuthToken={}; SimpleSAMLSessionID={}; {}={}", auth.simple_saml_auth_token, auth.simple_saml_session_id, auth.sse_pair.0, auth.sse_pair.1))
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    pub links_map: HashMap<String, Link>,
}

pub async fn load_links<P: AsRef<Path>>(path: P) -> tokio::io::Result<Links> {
    Ok(ron::from_str(&tokio::fs::read_to_string(path.as_ref().join("links.ron")).await?).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub description: String,
    pub researcher: Option<String>,
    pub departments: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Opportunities {
    pub opportunity_map: HashMap<String, Opportunity>,
}

pub async fn load_opportunities<P: AsRef<Path>>(path: P) -> tokio::io::Result<Opportunities> {
    Ok(ron::from_str(&tokio::fs::read_to_string(path.as_ref().join("opportunities.ron")).await?).unwrap())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opportunity {
    pub mentor_areas: Option<String>,
    pub description: Option<String>,
    pub preferred_qualifications: Option<String>,
    pub preferred_student_years: Option<PreferredStudentYears>,
    pub compensation: Option<Compensation>,
    pub academic_years: Option<Vec<usize>>,
    // pub researcher_name: Option<String>,
    // pub researcher_email: Option<String>,
    // pub researcher_link: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedOpportunities {
    pub processed_opportunity_map: HashMap<String, ProcessedOpportunity>,
}

pub async fn load_processed_opportunities<P: AsRef<Path>>(path: P) -> tokio::io::Result<ProcessedOpportunities> {
    Ok(ron::from_str(&tokio::fs::read_to_string(path.as_ref().join("processed_opportunities.ron")).await?).unwrap())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedOpportunity {
    pub title: String,
    pub short_description: String,
    pub researcher: Option<String>,
    pub departments: Vec<String>,
    pub mentor_areas: Option<String>,
    pub description: Option<String>,
    pub preferred_qualifications: Option<String>,
    pub preferred_student_years: Option<PreferredStudentYears>,
    pub compensation: Option<Compensation>,
    pub academic_years: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreferredStudentYears {
    pub first_year: bool,
    pub second_year: bool,
    pub third_year: bool,
    pub fourth_year: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compensation {
    pub volunteer: bool,
    pub salary: bool,
    pub independent_study: bool,
    pub work_study: bool,
}