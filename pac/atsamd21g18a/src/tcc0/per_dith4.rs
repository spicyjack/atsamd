#[doc = "Reader of register PER_DITH4"]
pub type R = crate::R<u32, super::PER_DITH4>;
#[doc = "Writer for register PER_DITH4"]
pub type W = crate::W<u32, super::PER_DITH4>;
#[doc = "Register PER_DITH4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PER_DITH4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DITHERCY`"]
pub type DITHERCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DITHERCY`"]
pub struct DITHERCY_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DITHERCY_R {
        DITHERCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&mut self) -> DITHERCY_W {
        DITHERCY_W { w: self }
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
