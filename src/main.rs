//! Script to automate login and Adobe reservation actions on the KMUTNB software portal.
//!
//! # Purpose
//! - Log in to https://software.kmutnb.ac.th using credentials provided via environment
//!   variables.
//! - Submit a request to the portal's Adobe reservation endpoint to extend/grant access
//!   (the script computes a target `date_expire` value automatically).

use chrono::{Datelike, Local};
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use std::env;
use std::time::Duration;

// LOGIN/ENDPOINT URL CONSTANTS
const LOGIN_URL: &str = "https://software.kmutnb.ac.th/login/";
const ADOBE_PROCESS_URL: &str = "https://software.kmutnb.ac.th/adobe-reserve/processa.php";
const ADOBE_URL: &str = "https://software.kmutnb.ac.th:443/adobe-reserve/add2.php";

const USER_AGENT_VALUE: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:146.0) Gecko/20100101 Firefox/146.0";
const ORIGIN_VALUE: &str = "https://software.kmutnb.ac.th";

/// Compute the first day of the next month using the original script's logic.
///
/// The original one-liner used a special case where if month == 1, the year was
/// decremented by 1 and the month set to 12; this function preserves that behavior.
/// Returns a string formatted as 'YYYY-MM-01'.
fn make_date_expire(year: i32, month: u32) -> String {
    let (new_year, new_month) = if month == 1 {
        (year - 1, 12)
    } else {
        (year, month + 1)
    };
    format!("{:04}-{:02}-01", new_year, new_month)
}

/// Build headers for the login POST request.
fn build_login_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert(ORIGIN, HeaderValue::from_static(ORIGIN_VALUE));
    headers.insert(REFERER, HeaderValue::from_static(LOGIN_URL));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers
}

/// Build headers for the Adobe reservation POST request.
fn build_adobe_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert(ORIGIN, HeaderValue::from_static(ORIGIN_VALUE));
    headers.insert(REFERER, HeaderValue::from_static(ADOBE_PROCESS_URL));
    headers
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from a .env file (if present)
    dotenv().ok();

    // Credentials from environment
    let username = env::var("KMUTNB_USERNAME").expect("KMUTNB_USERNAME must be set");
    let password = env::var("KMUTNB_PASSWORD").expect("KMUTNB_PASSWORD must be set");

    // Get current date for date_expire calculation
    let today = Local::now().date_naive();
    let date_expire = make_date_expire(today.year(), today.month());

    // Build the HTTP client with cookie store and disabled SSL verification
    // WARNING: Disabling SSL verification is insecure; only use for testing or
    // when you understand the risks.
    let client = Client::builder()
        .cookie_store(true)
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    // Login form data
    let login_data = [
        ("myusername", username.as_str()),
        ("mypassword", password.as_str()),
        ("Submit", ""),
    ];

    // Perform login
    let login_resp = client
        .post(LOGIN_URL)
        .headers(build_login_headers())
        .form(&login_data)
        .send()?;

    // Check if login succeeded
    if !login_resp.status().is_success() {
        eprintln!("Login request failed with status: {}", login_resp.status());
        return Err(format!("Login failed with status: {}", login_resp.status()).into());
    }

    // Adobe reservation form data
    let adobe_data = [
        ("userId", ""),
        ("date_expire", date_expire.as_str()),
        ("status_number", "0"),
        ("Submit_get", ""),
    ];

    // Submit Adobe reservation/add request
    let adobe_resp = client
        .post(ADOBE_URL)
        .headers(build_adobe_headers())
        .form(&adobe_data)
        .send()?;

    // Print the response body for visibility (original behavior)
    println!("{}", adobe_resp.text()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_date_expire_january() {
        // If month is January, should return December of previous year
        assert_eq!(make_date_expire(2024, 1), "2023-12-01");
    }

    #[test]
    fn test_make_date_expire_other_months() {
        // For other months, should return next month same year
        assert_eq!(make_date_expire(2024, 6), "2024-07-01");
        assert_eq!(make_date_expire(2024, 11), "2024-12-01");
        assert_eq!(make_date_expire(2024, 2), "2024-03-01");
    }
}
