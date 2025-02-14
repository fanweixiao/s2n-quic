// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::frame::ack_elicitation::{AckElicitable, AckElicitation};
use core::ops::AddAssign;

pub mod constraint;
pub mod mode;

pub use constraint::Constraint;
pub use mode::Mode;

#[derive(Clone, Copy, Debug, Default)]
pub struct Outcome {
    pub ack_elicitation: AckElicitation,
    pub is_congestion_controlled: bool,
    pub bytes_sent: usize,
    pub bytes_progressed: usize,
}

impl AckElicitable for Outcome {
    fn ack_elicitation(&self) -> AckElicitation {
        self.ack_elicitation
    }
}

impl AddAssign for Outcome {
    fn add_assign(&mut self, rhs: Self) {
        self.ack_elicitation |= rhs.ack_elicitation;
        self.is_congestion_controlled |= rhs.is_congestion_controlled;
        self.bytes_sent += rhs.bytes_sent;
        self.bytes_progressed += rhs.bytes_progressed;
    }
}
