//! sh1106 SPI interface

use hal::{self, digital::OutputPin};

use super::DisplayInterface;
use crate::Error;

/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC, CommE, PinE> SpiInterface<SPI, DC>
where
    // we shouldn't need the whole bus but we need to flush before setting dc
    SPI: hal::spi::SpiDevice<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    /// Create new SPI interface for communciation with sh1106
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC, CommE, PinE> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::spi::SpiDevice<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn init(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.dc.set_low().map_err(Error::Pin)?;
        self.spi.write(&cmds).map_err(Error::Comm)?;
        self.dc.set_high().map_err(Error::Pin)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // 1 = data, 0 = command
        self.dc.set_high().map_err(Error::Pin)?;
        self.spi.write(&buf).map_err(Error::Comm)
    }
}
