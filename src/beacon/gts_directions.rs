#[doc = "Guarranteed timeslot directions field."]
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
#[doc = "Mask identifying the directions of the GTSs in the superframe."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectionsMaskA {}
impl From<DirectionsMaskA> for u8 {
    #[inline(always)]
    fn from(variant: DirectionsMaskA) -> Self {
        variant as _
    }
}
#[doc = "Field `DirectionsMask` reader - Mask identifying the directions of the GTSs in the superframe."]
pub struct DirectionsMaskR(crate::FieldReader<u8, DirectionsMaskA>);
impl DirectionsMaskR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DirectionsMaskR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> DirectionsMaskA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for DirectionsMaskR {
    type Target = crate::FieldReader<u8, DirectionsMaskA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct DirectionsMaskW<'a> {
    w: &'a mut W,
}
impl<'a> DirectionsMaskW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: DirectionsMaskA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 0)) | ((value as u8 & 0x7f) << 0);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `DirectionsMask` field."]
    #[inline(always)]
    pub fn directions_mask(&self) -> DirectionsMaskR {
        DirectionsMaskR::new(((self.bits >> 0) & 0x7f) as u8)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `DirectionsMask` field."]
    #[inline(always)]
    pub fn directions_mask(&mut self) -> DirectionsMaskW {
        DirectionsMaskW { w: self }
    }
}
