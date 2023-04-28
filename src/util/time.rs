/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2023 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::time::UNIX_EPOCH;

use chrono::{NaiveDateTime, Utc};
use darkfi_serial::{SerialDecodable, SerialEncodable};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Helper structure providing time related calculations.
#[derive(Clone)]
pub struct TimeKeeper {
    /// Genesis block creation timestamp
    pub genesis_ts: Timestamp,
    /// Currently configured epoch duration.
    pub epoch_length: u64,
    /// Currently configured slot duration.
    pub slot_time: u64,
}

impl TimeKeeper {
    pub fn new(genesis_ts: Timestamp, epoch_length: u64, slot_time: u64) -> Self {
        Self { genesis_ts, epoch_length, slot_time }
    }

    /// Calculates current epoch.
    pub fn current_epoch(&self) -> u64 {
        self.slot_epoch(self.current_slot())
    }

    /// Calculates the epoch of the provided slot.    
    pub fn slot_epoch(&self, slot: u64) -> u64 {
        slot / self.epoch_length
    }

    /// Calculates current slot, based on elapsed time from the genesis block.
    pub fn current_slot(&self) -> u64 {
        self.genesis_ts.elapsed() / self.slot_time
    }

    /// Calculates the relative number of the provided slot.
    pub fn relative_slot(&self, slot: u64) -> u64 {
        slot % self.epoch_length
    }

    /// Calculates seconds until next Nth slot starting time.
    pub fn next_n_slot_start(&self, n: u64) -> u64 {
        assert!(n > 0);
        let next_slot_start = self.genesis_ts.0 + (self.current_slot() + n) * self.slot_time;
        next_slot_start - Timestamp::current_time().0
    }

    /// Calculate slots until next Nth epoch.
    /// Epoch duration is configured using the EPOCH_LENGTH value.
    pub fn slots_to_next_n_epoch(&self, n: u64) -> u64 {
        assert!(n > 0);
        let slots_till_next_epoch = self.epoch_length - self.relative_slot(self.current_slot());
        ((n - 1) * self.epoch_length) + slots_till_next_epoch
    }

    /// Calculates seconds until next Nth epoch starting time.
    pub fn next_n_epoch_start(&self, n: u64) -> u64 {
        self.next_n_slot_start(self.slots_to_next_n_epoch(n))
    }

    /// Calculates current blockchain timestamp.
    /// Blockchain timestamp is the time elapsed since
    /// Genesis timestamp, based on slot time ticking,
    /// therefore representing the starting timestamp of
    /// current slot.
    pub fn blockchain_timestamp(&self) -> u64 {
        self.genesis_ts.0 + self.current_slot() * self.slot_time
    }

    /// Calculates current system timestamp.
    pub fn system_timestamp(&self) -> Result<u64> {
        Ok(UNIX_EPOCH.elapsed()?.as_secs())
    }
}

/// Wrapper struct to represent system timestamps.
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    SerialEncodable,
    SerialDecodable,
    PartialEq,
    PartialOrd,
    Eq,
)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Generate a `Timestamp` of the current time.
    pub fn current_time() -> Self {
        Self(UNIX_EPOCH.elapsed().unwrap().as_secs())
    }

    /// Calculates elapsed time of a `Timestamp`.
    pub fn elapsed(&self) -> u64 {
        UNIX_EPOCH.elapsed().unwrap().as_secs() - self.0
    }

    /// Increment a 'Timestamp'.
    pub fn add(&mut self, inc: u64) {
        self.0 += inc;
    }
}

// TODO: NanoTimestamp to not use chrono
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    SerialEncodable,
    SerialDecodable,
    PartialEq,
    PartialOrd,
    Eq,
)]
pub struct NanoTimestamp(pub i64);

impl NanoTimestamp {
    pub fn current_time() -> Self {
        Self(Utc::now().timestamp_nanos())
    }
}
impl std::fmt::Display for NanoTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = timestamp_to_date(self.0, DateFormat::Nanos);
        write!(f, "{}", date)
    }
}

pub enum DateFormat {
    Default,
    Date,
    DateTime,
    Nanos,
}

pub fn timestamp_to_date(timestamp: i64, format: DateFormat) -> String {
    if timestamp <= 0 {
        return "".to_string()
    }

    match format {
        DateFormat::Date => NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .unwrap()
            .date()
            .format("%-d %b")
            .to_string(),
        DateFormat::DateTime => NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .unwrap()
            .format("%H:%M:%S %A %-d %B")
            .to_string(),
        DateFormat::Nanos => {
            const A_BILLION: i64 = 1_000_000_000;
            NaiveDateTime::from_timestamp_opt(timestamp / A_BILLION, (timestamp % A_BILLION) as u32)
                .unwrap()
                .format("%H:%M:%S.%f")
                .to_string()
        }
        DateFormat::Default => "".to_string(),
    }
}
