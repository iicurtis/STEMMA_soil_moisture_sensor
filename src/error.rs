use thiserror::Error;
use defmt::Format;

#[derive(Error, Format, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SoilMoistureSensorError {
    #[error("Write Read I2C Error")]
    WriteReadI2CError,
    #[error("Write I2C Error")]
    WriteI2CError,
    #[error("Read I2C Error")]
    ReadI2CError,
}
