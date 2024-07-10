use async_trait::async_trait;

#[async_trait]
pub trait Described {
    async fn description(&mut self) -> String {
        "none".to_string()
    }
}

pub mod device {
    use std::fmt::{Display, Formatter};

    use super::*;

    pub type Replay<T> = Result<T, Err>;
    pub type OptReplay<T> = Result<Option<T>, Err>;

    #[derive(Debug)]
    pub struct Err {
        pub msg: String,
    }

    impl Display for Err {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Operation not performed because : {}",
                self.msg
            )
        }
    }

    impl std::error::Error for Err {}

    #[async_trait]
    pub trait PowerConsumptionMeter {
        async fn power_consumption_wt(&mut self) -> OptReplay<f32>;
    }

    #[async_trait]
    pub trait Thermometer {
        async fn temperature_deg_celsius(&self) -> crate::common::traits::device::OptReplay<f32>;
    }

    #[async_trait]
    pub trait Switchable {
        async fn turn_on(&mut self) -> Replay<bool>;
        async fn turn_off(&mut self) -> Replay<bool>;
        async fn current_state(&mut self) -> Replay<bool>;
    }
}