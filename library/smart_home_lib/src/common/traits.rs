pub trait Described {
    fn description(&mut self) -> String {
        "none".to_string()
    }
}

pub mod device {
    use std::fmt::{Debug, Display, Formatter};

    pub trait SmartDevice: super::Described {}

    pub trait Switchable {
        fn turn_on(&mut self) -> Replay<bool>;
        fn turn_off(&mut self) -> Replay<bool>;
        fn current_state(&mut self) -> Replay<bool>;
    }

    pub trait PowerConsumptionMeter {
        fn power_consumption_wt(&mut self) -> OptReplay<f32>;
    }

    pub trait Thermometer {
        fn temperature_deg_celsius(&self) -> OptReplay<f32>;
    }

    pub type Replay<T> = Result<T, ErrorSm>;
    pub type OptReplay<T> = Result<Option<T>, ErrorSm>;

    #[derive(Debug)]
    pub struct ErrorSm {
        pub msg: String,
    }

    impl Display for ErrorSm {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Operation not performed because : {}",
                self.msg
            )
        }
    }

    impl std::error::Error for ErrorSm {}
    impl From<std::io::Error> for ErrorSm {
        fn from(value: std::io::Error) -> Self {
            ErrorSm { msg: value.to_string() }
        }
    }
}