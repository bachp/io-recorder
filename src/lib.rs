/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

extern crate embedded_hal;

#[macro_use]
extern crate serde_derive;

extern crate csv;
extern crate serde;

extern crate chrono;

use chrono::prelude::*;

use std::io;

use std::fmt;

macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        // Rust does not come with a simple way of converting a
                        // number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}

enum_number!(HighLow {
    Low = 0,
    High = 1,
});

#[derive(Debug, Serialize)]
struct Sample<T> {
    timestamp: DateTime<Utc>,
    value: T,
}

impl<T> Sample<T> {
    fn new(value: T) -> Sample<T> {
        Sample {
            timestamp: Utc::now(),
            value,
        }
    }
}
/// Record Gpio Data
///
#[derive(Default, Debug)]
pub struct GpioRecorder {
    samples: Vec<Sample<HighLow>>,
}

impl GpioRecorder {
    fn add_sample(&mut self, value: HighLow) {
        let sample = Sample::new(value);
        self.samples.push(sample);
    }

    /// Create a new `GpioRecorder`
    pub fn new() -> GpioRecorder {
        GpioRecorder::default()
    }

    /// Save all the samples in the current GpioRecorder in CSV format
    /// 
    /// The csv contains two columns: `timestamp` and `value`.
    pub fn save_csv<W: io::Write>(self, sink: W) -> Result<(), Box<std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(sink);

        for s in self.samples {
            wtr.serialize(s)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

impl embedded_hal::digital::OutputPin for GpioRecorder {
    fn set_low(&mut self) {
        self.add_sample(HighLow::Low)
    }

    fn set_high(&mut self) {
        self.add_sample(HighLow::High)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use embedded_hal::digital::OutputPin;
        use std::fs::File;
        use std::thread;
        use std::time::Duration;
        use GpioRecorder;

        let mut gr = GpioRecorder::new();

        for i in 1..10 {
            gr.set_low();
            thread::sleep(Duration::from_millis(10));
            gr.set_high();
            thread::sleep(Duration::from_millis(2));
        }
        gr.save_csv(std::io::stdout()).unwrap();
    }
}
