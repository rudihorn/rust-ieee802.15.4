use core2::io::{Error, Read, Write};
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct GtsDescriptor {
    short_address: u16,
    config: u8,
}
pub struct ShortAddress<'a> {
    data: &'a mut GtsDescriptor,
}
impl<'a> ShortAddress<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut GtsDescriptor) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u16 {
        self.data.short_address
    }
    #[inline(always)]
    pub fn set(&'a mut self, v: u16) -> &'a mut GtsDescriptor {
        self.data.short_address = v;
        self.data
    }
}
pub struct Config<'a> {
    data: &'a mut GtsDescriptor,
}
impl<'a> Config<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut GtsDescriptor) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> super::gts_descriptor_config::R {
        super::gts_descriptor_config::R::new(self.data.config)
    }
    #[inline(always)]
    pub fn modify<F>(&'a mut self, f: F) -> &'a mut GtsDescriptor
    where
        for<'w> F: FnOnce(
            &'w mut super::gts_descriptor_config::W,
        ) -> &'w mut super::gts_descriptor_config::W,
    {
        let bits = self.data.config;
        self.data.config = **f(&mut super::gts_descriptor_config::W::new(bits));
        self.data
    }
}
impl GtsDescriptor {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            short_address: 0,
            config: 0,
        }
    }
    pub fn short_address(&mut self) -> ShortAddress {
        ShortAddress::new(self)
    }
    pub fn config(&mut self) -> Config {
        Config::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.short_address.to_be_bytes())?;
        out.write(&self.config.to_be_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;
        let short_address = u16::from_be_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let config = u8::from_be_bytes(buffer);
        Ok(Self {
            short_address,
            config,
        })
    }
}
