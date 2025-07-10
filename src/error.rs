// Copyright (c) 2025 Oleg Pavlenko

use std::error::Error;

pub type AnyError = Box<dyn Error + Send + Sync + 'static>;