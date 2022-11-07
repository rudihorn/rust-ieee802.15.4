#[doc = "The starting slot and length of a guaranteed time slot. Note that this does not include the device short address."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct R {
    bits: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct W {
    bits: u8,
}
impl core::ops::Deref for W {
    type Target = u8;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}
#[doc = "The starting slot of the guaranteed time slot."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StartingSlotA {}
impl From<StartingSlotA> for u8 {
    #[inline(always)]
    fn from(variant: StartingSlotA) -> Self {
        variant as _
    }
}
#[doc = "Field `StartingSlot` reader - The starting slot of the guaranteed time slot."]
pub struct StartingSlotR(crate::FieldReader<u8, StartingSlotA>);
impl StartingSlotR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        StartingSlotR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> StartingSlotA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for StartingSlotR {
    type Target = crate::FieldReader<u8, StartingSlotA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct StartingSlotW<'a> {
    w: &'a mut W,
}
impl<'a> StartingSlotW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: StartingSlotA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 0)) | ((value as u8 & 0x0f) << 0);
        self.w
    }
}
#[doc = "The number of contiguous superframe slots over which this guaranteed time slot is active."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthA {}
impl From<LengthA> for u8 {
    #[inline(always)]
    fn from(variant: LengthA) -> Self {
        variant as _
    }
}
#[doc = "Field `Length` reader - The number of contiguous superframe slots over which this guaranteed time slot is active."]
pub struct LengthR(crate::FieldReader<u8, LengthA>);
impl LengthR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LengthR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> LengthA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for LengthR {
    type Target = crate::FieldReader<u8, LengthA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct LengthW<'a> {
    w: &'a mut W,
}
impl<'a> LengthW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: LengthA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `StartingSlot` field."]
    #[inline(always)]
    pub fn starting_slot(&self) -> StartingSlotR {
        StartingSlotR::new(((self.bits >> 0) & 0x0f) as u8)
    }
    #[doc = "Read the `Length` field."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `StartingSlot` field."]
    #[inline(always)]
    pub fn starting_slot(&mut self) -> StartingSlotW {
        StartingSlotW { w: self }
    }
    #[doc = "Set the `Length` field."]
    #[inline(always)]
    pub fn length(&mut self) -> LengthW {
        LengthW { w: self }
    }
}
