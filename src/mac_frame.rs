#[repr(packed)]
#[derive(Clone, Copy)]
pub struct AddrNone {}
impl AddrNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AddrShort {
    address: u16,
}
impl AddrShort {
    pub fn new() -> Self {
        Self { address: 0 }
    }
    pub fn get(&self) -> u16 {
        self.address
    }
    pub fn set(&mut self, v: u16) -> &mut Self {
        self.address = v;
        self
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AddrExtended {
    address: u64,
}
impl AddrExtended {
    pub fn new() -> Self {
        Self { address: 0 }
    }
    pub fn get(&self) -> u64 {
        self.address
    }
    pub fn set(&mut self, v: u64) -> &mut Self {
        self.address = v;
        self
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct PanNone {}
impl PanNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanShort {
    pan: u16,
}
impl PanShort {
    pub fn new() -> Self {
        Self { pan: 0 }
    }
    pub fn get(&self) -> u16 {
        self.pan
    }
    pub fn set(&mut self, v: u16) -> &mut Self {
        self.pan = v;
        self
    }
}
pub trait Panid: Copy {
    fn default() -> Self;
}
pub enum PanidA {
    PanNone(PanNone),
    PanShort(PanShort),
}
pub trait Address: Copy {
    fn default() -> Self;
}
pub enum AddressA {
    AddrNone(AddrNone),
    AddrShort(AddrShort),
    AddrExtended(AddrExtended),
}
impl Panid for PanNone {
    fn default() -> Self {
        Self::new()
    }
}
impl Panid for PanShort {
    fn default() -> Self {
        Self::new()
    }
}
impl Address for AddrNone {
    fn default() -> Self {
        Self::new()
    }
}
impl Address for AddrShort {
    fn default() -> Self {
        Self::new()
    }
}
impl Address for AddrExtended {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    frame_control: u16,
    sequence_number: u8,
    dest_pan: DestPanT,
    dest_address: DestAddressT,
    source_pan: SourcePanT,
    source_address: SourceAddressT,
}
pub struct FrameControl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    FrameControl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> crate::frame_control::R {
        crate::frame_control::R::new(self.data.frame_control)
    }
    #[inline(always)]
    pub fn modify<F>(
        &'a mut self,
        f: F,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        for<'w> F: FnOnce(&'w mut crate::frame_control::W) -> &'w mut crate::frame_control::W,
    {
        let bits = self.data.frame_control;
        self.data.frame_control = **f(&mut crate::frame_control::W::new(bits));
        self.data
    }
}
pub struct SequenceNumber<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    SequenceNumber<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.data.sequence_number
    }
    #[inline(always)]
    pub fn set(
        &'a mut self,
        v: u8,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        self.data.sequence_number = v;
        self.data
    }
}
pub struct DestPan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    DestPan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> DestPanT {
        self.data.dest_pan
    }
    #[inline(always)]
    pub fn modify<F>(
        &'a mut self,
        f: F,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        for<'w> F: FnOnce(&'w mut DestPanT) -> &'w mut DestPanT,
    {
        let mut cp = self.data.dest_pan;
        self.data.dest_pan = *f(&mut cp);
        self.data
    }
}
pub struct DestAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    DestAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> DestAddressT {
        self.data.dest_address
    }
    #[inline(always)]
    pub fn modify<F>(
        &'a mut self,
        f: F,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        for<'w> F: FnOnce(&'w mut DestAddressT) -> &'w mut DestAddressT,
    {
        let mut cp = self.data.dest_address;
        self.data.dest_address = *f(&mut cp);
        self.data
    }
}
pub struct SourcePan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    SourcePan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> SourcePanT {
        self.data.source_pan
    }
    #[inline(always)]
    pub fn modify<F>(
        &'a mut self,
        f: F,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        for<'w> F: FnOnce(&'w mut SourcePanT) -> &'w mut SourcePanT,
    {
        let mut cp = self.data.source_pan;
        self.data.source_pan = *f(&mut cp);
        self.data
    }
}
pub struct SourceAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
}
impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    SourceAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub(crate) fn new(
        data: &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    ) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> SourceAddressT {
        self.data.source_address
    }
    #[inline(always)]
    pub fn modify<F>(
        &'a mut self,
        f: F,
    ) -> &'a mut Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        for<'w> F: FnOnce(&'w mut SourceAddressT) -> &'w mut SourceAddressT,
    {
        let mut cp = self.data.source_address;
        self.data.source_address = *f(&mut cp);
        self.data
    }
}
impl<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            frame_control: 0,
            sequence_number: 0,
            dest_pan: DestPanT::default(),
            dest_address: DestAddressT::default(),
            source_pan: SourcePanT::default(),
            source_address: SourceAddressT::default(),
        }
    }
    pub fn frame_control(
        &mut self,
    ) -> FrameControl<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        FrameControl::new(self)
    }
    pub fn sequence_number(
        &mut self,
    ) -> SequenceNumber<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        SequenceNumber::new(self)
    }
    pub fn dest_pan(&mut self) -> DestPan<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        DestPan::new(self)
    }
    pub fn dest_address(
        &mut self,
    ) -> DestAddress<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        DestAddress::new(self)
    }
    pub fn source_pan(&mut self) -> SourcePan<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        SourcePan::new(self)
    }
    pub fn source_address(
        &mut self,
    ) -> SourceAddress<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        SourceAddress::new(self)
    }
}
pub type MhrDefault = Mhr<PanNone, AddrNone, PanNone, AddrNone>;
