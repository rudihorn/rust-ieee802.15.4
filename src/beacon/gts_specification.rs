#[doc = "Guarranteed timeslot specification field."]
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
#[doc = "Specifies if the coordinator is accepting guaranteed timeslot requests."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PermitA {
    #[doc = "The coordinator is not accepting GTS requests."]
    NotPermitted = 0,
    #[doc = "The coordinator is accepting GTS requests."]
    Permitted = 1,
}
impl From<PermitA> for bool {
    #[inline(always)]
    fn from(variant: PermitA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Permit` reader - Specifies if the coordinator is accepting guaranteed timeslot requests."]
pub struct PermitR(crate::FieldReader<bool, PermitA>);
impl PermitR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PermitR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> PermitA {
        match self.bits {
            false => PermitA::NotPermitted,
            true => PermitA::Permitted,
        }
    }
    #[doc = "Checks if the value of the `Permit` field is `NotPermitted`"]
    #[inline(always)]
    pub fn is_not_permitted(&self) -> bool {
        **self == PermitA::NotPermitted
    }
    #[doc = "Checks if the value of the `Permit` field is `Permitted`"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        **self == PermitA::Permitted
    }
}
impl core::ops::Deref for PermitR {
    type Target = crate::FieldReader<bool, PermitA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct PermitW<'a> {
    w: &'a mut W,
}
impl<'a> PermitW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: PermitA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `Permit` field to `NotPermitted`"]
    #[inline(always)]
    pub fn not_permitted(self) -> &'a mut W {
        self.variant(PermitA::NotPermitted)
    }
    #[doc = "Set the value of the `Permit` field to `Permitted`"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut W {
        self.variant(PermitA::Permitted)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `DescriptorCount` field."]
    #[inline(always)]
    pub fn descriptor_count(&self) -> u8 {
        ((self.bits >> 0) & 0x07) as u8
    }
    #[doc = "Read the `Permit` field."]
    #[inline(always)]
    pub fn permit(&self) -> PermitR {
        PermitR::new((self.bits & 0x80) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `DescriptorCount` field."]
    #[inline(always)]
    pub fn descriptor_count(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x07 << 0)) | ((value as u8 & 0x07) << 0);
        Self { bits, ..*self }
    }
    #[doc = "Set the `Permit` field."]
    #[inline(always)]
    pub fn permit(&mut self) -> PermitW {
        PermitW { w: self }
    }
}
