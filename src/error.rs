// Copyright (c) 2025 Oleg Pavlenko

///
/// Error Handling
/// 
#[derive(Debug)]
pub enum YookassaError {
    // Unwrap or Error from reqwest
    Reqwest(reqwest::Error),
    // Any code except 200
    Code(reqwest::StatusCode),
}