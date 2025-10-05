#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, lazy_get)]
    #![allow(unused_parens, unused_assignments, overflowing_literals)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::sync::LazyLock;

    static mut H: LazyLock<DefaultHasher> = LazyLock::new(|| DefaultHasher::new());

    #[inline(never)]
    fn dump_var(
        val0: impl Hash,
        val1: impl Hash,
        val2: impl Hash,
        val3: impl Hash,
    ) {
        unsafe {
            val0.hash(LazyLock::force_mut(&mut H));
            val1.hash(LazyLock::force_mut(&mut H));
            val2.hash(LazyLock::force_mut(&mut H));
            val3.hash(LazyLock::force_mut(&mut H));
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: u8) -> char {
mir! {
type RET = char;
let _2: f32;
let _3: char;
let _4: u8;
let _5: bool;
let _6: &'static &'static mut *const u64;
let _7: &'static u32;
let _8: u8;
let _9: *mut *const &'static mut f32;
let _10: f64;
let _11: u64;
let _12: i8;
let _13: [char; 5];
let _14: char;
let _15: (Adt26, Adt41);
let _16: *const u64;
let _17: f32;
let _18: u16;
let _19: (f64, f64, u128, i16);
let _20: char;
let _21: *const Adt41;
let _22: u32;
let _23: i128;
let _24: *const Adt22;
let _25: &'static f32;
let _26: &'static isize;
let _27: *mut &'static mut [char; 6];
let _28: *const Adt22;
let _29: (*mut [u16; 5], [u16; 5], u32, [u16; 5]);
let _30: &'static mut (u64,);
let _31: u8;
let _32: &'static u32;
let _33: f32;
let _34: u8;
let _35: char;
let _36: char;
let _37: f32;
let _38: (f64, f64, u128, i16);
let _39: i32;
let _40: Adt41;
let _41: [i16; 7];
let _42: i64;
let _43: i32;
let _44: *const &'static mut f32;
let _45: *mut i16;
let _46: char;
let _47: Adt26;
let _48: isize;
let _49: u64;
let _50: u64;
let _51: f32;
let _52: bool;
let _53: (i8, u64, u16);
let _54: &'static &'static mut *const u64;
let _55: &'static &'static mut &'static mut f32;
let _56: i16;
let _57: u64;
let _58: isize;
let _59: char;
let _60: u128;
let _61: f32;
let _62: (*const u64, (i8, u64, u16), Adt22, isize);
let _63: isize;
let _64: &'static mut [char; 6];
let _65: isize;
let _66: isize;
let _67: *const (u64,);
let _68: [u32; 7];
let _69: &'static mut *mut i128;
let _70: *const Adt41;
let _71: Adt78;
let _72: [char; 5];
let _73: isize;
let _74: (*mut [u16; 5], [u16; 5], u32, [u16; 5]);
let _75: Adt49;
let _76: bool;
let _77: *mut u128;
let _78: i128;
let _79: *mut [u16; 5];
let _80: isize;
let _81: [u8; 4];
let _82: u64;
let _83: char;
let _84: f64;
let _85: u64;
let _86: *mut [u128; 7];
let _87: char;
let _88: *mut [u128; 7];
let _89: [char; 6];
let _90: isize;
let _91: [i16; 7];
let _92: f32;
let _93: ();
let _94: ();
{
RET = '\u{131f8}';
RET = '\u{cdaf0}';
_1 = 189_u8;
_2 = 6_usize as f32;
_2 = 84_i8 as f32;
_2 = (-130517172556542154433081859849969021504_i128) as f32;
_1 = !53_u8;
RET = '\u{f0f4f}';
RET = '\u{f91c7}';
_2 = 3924030822997691277_u64 as f32;
_2 = (-1548430778_i32) as f32;
_3 = RET;
_3 = RET;
_3 = RET;
RET = _3;
_1 = 1_u8 * 235_u8;
_2 = (-3108791875868824108_i64) as f32;
_3 = RET;
RET = _3;
_2 = 1112344712_i32 as f32;
_2 = 1802320452_u32 as f32;
RET = _3;
Call(_3 = fn1(RET, _1, RET, _1, RET, _2, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 9373_i16 as f32;
_5 = !false;
_2 = _1 as f32;
_4 = _1 * _1;
_1 = _4 << _4;
_5 = true;
_5 = _1 >= _1;
RET = _3;
_3 = RET;
_4 = !_1;
_1 = 728689971_i32 as u8;
RET = _3;
_5 = _4 != _4;
_4 = _1 + _1;
RET = _3;
_2 = 54027005728178793595851150403143967887_u128 as f32;
RET = _3;
_1 = 33772232133063800865750589457378122789_u128 as u8;
RET = _3;
_5 = true;
_5 = _4 >= _4;
_1 = _4;
_2 = 4298889839590723734_usize as f32;
_3 = RET;
_2 = (-10792_i16) as f32;
RET = _3;
_5 = !true;
Goto(bb2)
}
bb2 = {
_3 = RET;
_3 = RET;
_1 = _4;
_1 = _4 >> _4;
_5 = _4 < _1;
RET = _3;
_1 = _4 << _4;
_2 = 7542377717365535626_u64 as f32;
_2 = 41499_u16 as f32;
_8 = _1 + _4;
Goto(bb3)
}
bb3 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb4 = {
_3 = RET;
_8 = _1;
_11 = 12653473696017041380_u64 | 15367281469381108351_u64;
RET = _3;
_10 = 660617473260587142048970026642946173_i128 as f64;
_1 = (-17747_i16) as u8;
_5 = !true;
_13 = [RET,_3,RET,_3,_3];
match _12 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211381 => bb8,
_ => bb7
}
}
bb5 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb6 = {
_3 = RET;
_3 = RET;
_1 = _4;
_1 = _4 >> _4;
_5 = _4 < _1;
RET = _3;
_1 = _4 << _4;
_2 = 7542377717365535626_u64 as f32;
_2 = 41499_u16 as f32;
_8 = _1 + _4;
Goto(bb3)
}
bb7 = {
_2 = 9373_i16 as f32;
_5 = !false;
_2 = _1 as f32;
_4 = _1 * _1;
_1 = _4 << _4;
_5 = true;
_5 = _1 >= _1;
RET = _3;
_3 = RET;
_4 = !_1;
_1 = 728689971_i32 as u8;
RET = _3;
_5 = _4 != _4;
_4 = _1 + _1;
RET = _3;
_2 = 54027005728178793595851150403143967887_u128 as f32;
RET = _3;
_1 = 33772232133063800865750589457378122789_u128 as u8;
RET = _3;
_5 = true;
_5 = _4 >= _4;
_1 = _4;
_2 = 4298889839590723734_usize as f32;
_3 = RET;
_2 = (-10792_i16) as f32;
RET = _3;
_5 = !true;
Goto(bb2)
}
bb8 = {
_12 = (-47_i8) + (-62_i8);
_11 = 12749605101708780770_u64 * 13181562172513075282_u64;
_13 = [_3,_3,RET,_3,_3];
_15.0.fld3 = _10 * _10;
_15.0.fld4 = _11;
_15.0.fld1.0 = (-416592450_i32) as u64;
_3 = RET;
_14 = _3;
_3 = _14;
_12 = !(-17_i8);
_15.0.fld1 = (_15.0.fld4,);
_15.0.fld0 = _2;
_15.0.fld5 = 7_usize as u16;
_10 = _12 as f64;
_13 = [_14,_3,RET,RET,RET];
_10 = _15.0.fld3 - _15.0.fld3;
_10 = _15.0.fld3;
_15.0.fld0 = _2;
Goto(bb9)
}
bb9 = {
_15.0.fld3 = _10 * _10;
_15.0.fld1 = (_11,);
_11 = _15.0.fld1.0 & _15.0.fld4;
_8 = _1 & _1;
RET = _3;
_2 = _15.0.fld0 * _15.0.fld0;
_19.3 = (-24794_i16);
_19.2 = 284283684858710891659903431475079422759_u128 & 288454052772123525312499924713092198456_u128;
_17 = _2 - _2;
_15.0.fld3 = -_10;
_1 = !_8;
_19.3 = -(-13436_i16);
_15.0.fld6 = core::ptr::addr_of!(_1);
_20 = _14;
_18 = !_15.0.fld5;
_15.0.fld1 = (_15.0.fld4,);
_15.0.fld1.0 = _15.0.fld4 + _11;
Call(_15.0.fld3 = core::intrinsics::transmute(_15.0.fld1.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = core::ptr::addr_of!(_15.0.fld1.0);
_19.1 = _15.0.fld3 + _15.0.fld3;
_12 = (-87_i8) << (*_16);
_15.0.fld5 = _18;
Goto(bb11)
}
bb11 = {
(*_16) = !_11;
_21 = core::ptr::addr_of!(_15.1);
(*_16) = 16542795083157909663_usize as u64;
_19.0 = -_19.1;
_22 = 1947486470_u32;
_15.0.fld2 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
(*_16) = !_11;
_23 = (-14790408222930620721091142695749425978_i128);
_23 = (-167617648186405998062446544981485809294_i128) | (-33943863955018741521810732922710010984_i128);
_15.0.fld2 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
(*_21) = Adt41::Variant2 { fld0: Move(_16),fld1: _22,fld2: _2,fld3: _12,fld4: (-5708401897631373888_i64) };
place!(Field::<f32>(Variant((*_21), 2), 2)) = _19.0 as f32;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (-1482473931392948781_i64);
_19.1 = _19.0 + _19.0;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (-4036378926168164914_i64) & (-2275785666918159718_i64);
place!(Field::<i64>(Variant((*_21), 2), 4)) = 6612690040406764997_i64;
_7 = &place!(Field::<u32>(Variant((*_21), 2), 1));
Goto(bb12)
}
bb12 = {
_29.2 = _23 as u32;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (*_7) as i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 4705831936782248566_i64 ^ 8163708743587683210_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 8167864631328996001_i64 & 5577726668841165514_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _17 as i64;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 - _17;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 >> _12;
_19.0 = 9223372036854775807_isize as f64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = !(-9214834222469628886_i64);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 + _12;
_19 = (_15.0.fld3, _15.0.fld3, 64916826490002032122748762771291743660_u128, (-5117_i16));
_15.0.fld1.0 = _15.0.fld4;
Goto(bb13)
}
bb13 = {
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
RET = _20;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 | _12;
_25 = &place!(Field::<f32>(Variant((*_21), 2), 2));
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 + _17;
place!(Field::<u32>(Variant(_15.1, 2), 1)) = !_22;
place!(Field::<f32>(Variant((*_21), 2), 2)) = -_15.0.fld0;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2;
_15.0.fld3 = -_19.1;
_19.1 = _15.0.fld3;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _2;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
_10 = _19.0 + _19.1;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 ^ _29.2;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _4 as u32;
_29.2 = Field::<u32>(Variant((*_21), 2), 1) ^ Field::<u32>(Variant((*_21), 2), 1);
_15.0.fld5 = (-1853292797_i32) as u16;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 * _29.2;
_35 = _3;
Goto(bb14)
}
bb14 = {
place!(Field::<f32>(Variant((*_21), 2), 2)) = -_17;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (-8477217975333870995_i64) ^ (-275427506452191538_i64);
_38 = (_19.0, _15.0.fld3, _19.2, _19.3);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
_38.0 = _19.1;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (-3215382797270516318_i64) << Field::<u32>(Variant((*_21), 2), 1);
_29.0 = core::ptr::addr_of_mut!(_29.3);
_19.0 = _38.0 + _10;
Goto(bb15)
}
bb15 = {
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_11);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_11);
_15.0.fld4 = _11 * _11;
place!(Field::<i64>(Variant((*_21), 2), 4)) = !7890935031470417242_i64;
_36 = _14;
Goto(bb16)
}
bb16 = {
_37 = Field::<f32>(Variant((*_21), 2), 2) + Field::<f32>(Variant((*_21), 2), 2);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _22;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _2 * _37;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _15.0.fld5 as i64;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_11);
_15.0.fld3 = _38.3 as f64;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2;
place!(Field::<i8>(Variant((*_21), 2), 3)) = Field::<f32>(Variant((*_21), 2), 2) as i8;
place!(Field::<u32>(Variant((*_21), 2), 1)) = Field::<i8>(Variant((*_21), 2), 3) as u32;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 / _22;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
_12 = RET as i8;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 - _12;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 5982251778589913459_i64 | 7214503777603374345_i64;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _38.1 as u32;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 >> Field::<i64>(Variant((*_21), 2), 4);
_33 = -Field::<f32>(Variant((*_21), 2), 2);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _22 + _29.2;
_21 = core::ptr::addr_of!((*_21));
Goto(bb17)
}
bb17 = {
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
_19.2 = _38.2;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
_47.fld1 = (_15.0.fld4,);
_9 = core::ptr::addr_of_mut!(_44);
_38.0 = _15.0.fld3 * _10;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld0;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12;
_47 = Adt26 { fld0: _33,fld1: _15.0.fld1,fld2: _15.0.fld2,fld3: _15.0.fld3,fld4: _15.0.fld4,fld5: _15.0.fld5,fld6: Move(_15.0.fld6) };
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 % _22;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
place!(Field::<f32>(Variant(_15.1, 2), 2)) = _17 - _17;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _23 as f32;
_15.0.fld5 = _23 as u16;
_23 = (-10450488004304516442518353961829561262_i128) ^ 103708427241749818986581371761595397505_i128;
_47.fld3 = _19.0;
_15.0.fld0 = Field::<u32>(Variant((*_21), 2), 1) as f32;
_42 = -Field::<i64>(Variant((*_21), 2), 4);
_11 = _47.fld4;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
_17 = _15.0.fld0;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_11);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _38.3 as u32;
match _38.3 {
0 => bb3,
1 => bb18,
2 => bb19,
3 => bb20,
340282366920938463463374607431768206339 => bb22,
_ => bb21
}
}
bb18 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb19 = {
_29.2 = _23 as u32;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (*_7) as i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 4705831936782248566_i64 ^ 8163708743587683210_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 8167864631328996001_i64 & 5577726668841165514_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _17 as i64;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 - _17;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 >> _12;
_19.0 = 9223372036854775807_isize as f64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = !(-9214834222469628886_i64);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 + _12;
_19 = (_15.0.fld3, _15.0.fld3, 64916826490002032122748762771291743660_u128, (-5117_i16));
_15.0.fld1.0 = _15.0.fld4;
Goto(bb13)
}
bb20 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb21 = {
_2 = 9373_i16 as f32;
_5 = !false;
_2 = _1 as f32;
_4 = _1 * _1;
_1 = _4 << _4;
_5 = true;
_5 = _1 >= _1;
RET = _3;
_3 = RET;
_4 = !_1;
_1 = 728689971_i32 as u8;
RET = _3;
_5 = _4 != _4;
_4 = _1 + _1;
RET = _3;
_2 = 54027005728178793595851150403143967887_u128 as f32;
RET = _3;
_1 = 33772232133063800865750589457378122789_u128 as u8;
RET = _3;
_5 = true;
_5 = _4 >= _4;
_1 = _4;
_2 = 4298889839590723734_usize as f32;
_3 = RET;
_2 = (-10792_i16) as f32;
RET = _3;
_5 = !true;
Goto(bb2)
}
bb22 = {
place!(Field::<i64>(Variant((*_21), 2), 4)) = _42;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 + _29.2;
_29.3 = [_47.fld5,_18,_18,_15.0.fld5,_18];
_29.2 = Field::<u32>(Variant((*_21), 2), 1) | Field::<u32>(Variant((*_21), 2), 1);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 ^ _12;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _42;
_38.2 = !_19.2;
_48 = 95_isize;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld4 as f32;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 - _29.2;
place!(Field::<f32>(Variant((*_21), 2), 2)) = -_17;
match _38.3 {
0 => bb23,
340282366920938463463374607431768206339 => bb25,
_ => bb24
}
}
bb23 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb24 = {
_3 = RET;
_3 = RET;
_1 = _4;
_1 = _4 >> _4;
_5 = _4 < _1;
RET = _3;
_1 = _4 << _4;
_2 = 7542377717365535626_u64 as f32;
_2 = 41499_u16 as f32;
_8 = _1 + _4;
Goto(bb3)
}
bb25 = {
place!(Field::<i8>(Variant(_15.1, 2), 3)) = Field::<u32>(Variant(_15.1, 2), 1) as i8;
_4 = _8 & _1;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld0;
_26 = &_48;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12;
_29.3 = [_18,_15.0.fld5,_47.fld5,_47.fld5,_15.0.fld5];
_34 = _4;
_29.0 = core::ptr::addr_of_mut!(_29.3);
_15.0.fld1.0 = _19.2 as u64;
_15.0 = Adt26 { fld0: Field::<f32>(Variant((*_21), 2), 2),fld1: _47.fld1,fld2: _47.fld2,fld3: _38.0,fld4: _47.fld4,fld5: _18,fld6: Move(_47.fld6) };
_19.3 = _38.3 ^ _38.3;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_50);
Goto(bb26)
}
bb26 = {
place!(Field::<i64>(Variant((*_21), 2), 4)) = !_42;
_17 = Field::<f32>(Variant((*_21), 2), 2);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _48 as i8;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _15.0.fld5 as i64;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _34 as f32;
_41 = [_38.3,_38.3,_38.3,_19.3,_19.3,_19.3,_19.3];
place!(Field::<u32>(Variant((*_21), 2), 1)) = _4 as u32;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2;
_38 = (_47.fld3, _15.0.fld3, _19.2, _19.3);
_13 = [_3,_36,_35,RET,_3];
_46 = _35;
_29.1 = [_47.fld5,_18,_15.0.fld5,_47.fld5,_18];
_47.fld4 = _11;
_53.0 = _38.0 as i8;
place!(Field::<i8>(Variant((*_21), 2), 3)) = Field::<i64>(Variant((*_21), 2), 4) as i8;
_38.3 = _19.3;
_1 = !_4;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_49);
match _22 {
0 => bb5,
1 => bb8,
2 => bb20,
3 => bb15,
4 => bb27,
5 => bb28,
1947486470 => bb30,
_ => bb29
}
}
bb27 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb28 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb29 = {
_3 = RET;
_3 = RET;
_1 = _4;
_1 = _4 >> _4;
_5 = _4 < _1;
RET = _3;
_1 = _4 << _4;
_2 = 7542377717365535626_u64 as f32;
_2 = 41499_u16 as f32;
_8 = _1 + _4;
Goto(bb3)
}
bb30 = {
_47.fld5 = _15.0.fld5;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _23 as i64;
Goto(bb31)
}
bb31 = {
_29.3 = [_18,_15.0.fld5,_15.0.fld5,_15.0.fld5,_47.fld5];
_19 = (_38.1, _15.0.fld3, _38.2, _38.3);
place!(Field::<i8>(Variant(_15.1, 2), 3)) = _53.0;
_15.0.fld5 = !_47.fld5;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_62.1.1);
_47.fld1.0 = _15.0.fld4 * _47.fld4;
_47.fld6 = core::ptr::addr_of!(_31);
_37 = Field::<f32>(Variant((*_21), 2), 2);
match _19.2 {
0 => bb24,
1 => bb29,
2 => bb32,
3 => bb33,
4 => bb34,
5 => bb35,
64916826490002032122748762771291743660 => bb37,
_ => bb36
}
}
bb32 = {
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld1.0);
_19.2 = _38.2;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
_47.fld1 = (_15.0.fld4,);
_9 = core::ptr::addr_of_mut!(_44);
_38.0 = _15.0.fld3 * _10;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld0;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12;
_47 = Adt26 { fld0: _33,fld1: _15.0.fld1,fld2: _15.0.fld2,fld3: _15.0.fld3,fld4: _15.0.fld4,fld5: _15.0.fld5,fld6: Move(_15.0.fld6) };
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 % _22;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
place!(Field::<f32>(Variant(_15.1, 2), 2)) = _17 - _17;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _23 as f32;
_15.0.fld5 = _23 as u16;
_23 = (-10450488004304516442518353961829561262_i128) ^ 103708427241749818986581371761595397505_i128;
_47.fld3 = _19.0;
_15.0.fld0 = Field::<u32>(Variant((*_21), 2), 1) as f32;
_42 = -Field::<i64>(Variant((*_21), 2), 4);
_11 = _47.fld4;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
_17 = _15.0.fld0;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_11);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _38.3 as u32;
match _38.3 {
0 => bb3,
1 => bb18,
2 => bb19,
3 => bb20,
340282366920938463463374607431768206339 => bb22,
_ => bb21
}
}
bb33 = {
_16 = core::ptr::addr_of!(_15.0.fld1.0);
_19.1 = _15.0.fld3 + _15.0.fld3;
_12 = (-87_i8) << (*_16);
_15.0.fld5 = _18;
Goto(bb11)
}
bb34 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb35 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb36 = {
RET = _3;
_5 = false;
_8 = _1 - _4;
RET = _3;
_4 = _8;
_5 = _1 < _1;
_4 = _8 & _8;
_3 = RET;
RET = _3;
_2 = 64104_u16 as f32;
_4 = _1 << _1;
_10 = (-9223372036854775808_isize) as f64;
_3 = RET;
_4 = _8 * _1;
_2 = 57745_u16 as f32;
_12 = 48_i8 >> _4;
_12 = (-75_i8);
_11 = 1544109003205103010_u64;
_4 = !_1;
_5 = _8 >= _8;
Goto(bb4)
}
bb37 = {
place!(Field::<i64>(Variant((*_21), 2), 4)) = _47.fld5 as i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _42;
_11 = !_47.fld1.0;
_15.0.fld4 = _15.0.fld1.0;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _47.fld5 as i64;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 * _17;
_51 = -Field::<f32>(Variant((*_21), 2), 2);
place!(Field::<i64>(Variant((*_21), 2), 4)) = !_42;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _53.0 << Field::<u32>(Variant((*_21), 2), 1);
_14 = _3;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_49);
_58 = _36 as isize;
_16 = core::ptr::addr_of!(_53.1);
(*_16) = _47.fld4;
place!(Field::<i8>(Variant((*_21), 2), 3)) = -_53.0;
_42 = Field::<i64>(Variant((*_21), 2), 4) & Field::<i64>(Variant((*_21), 2), 4);
place!(Field::<i64>(Variant((*_21), 2), 4)) = _42 * _42;
(*_16) = _11 >> Field::<i8>(Variant((*_21), 2), 3);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _53.0 | _53.0;
(*_16) = _11 * _47.fld1.0;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld0;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _5 as u32;
_60 = _29.2 as u128;
(*_16) = !_11;
_21 = core::ptr::addr_of!((*_21));
_43 = (-1427692735_i32);
_45 = core::ptr::addr_of_mut!(_38.3);
_21 = core::ptr::addr_of!((*_21));
match (*_26) {
0 => bb1,
1 => bb31,
2 => bb7,
3 => bb9,
95 => bb38,
_ => bb24
}
}
bb38 = {
_62.1.0 = _15.0.fld5 as i8;
Goto(bb39)
}
bb39 = {
place!(Field::<i8>(Variant((*_21), 2), 3)) = Field::<i64>(Variant((*_21), 2), 4) as i8;
_40 = Move((*_21));
(*_45) = _19.3;
(*_21) = Move(_40);
_32 = &place!(Field::<u32>(Variant((*_21), 2), 1));
place!(Field::<f32>(Variant((*_21), 2), 2)) = -_51;
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_15.0.fld4);
_68 = [(*_32),_29.2,_29.2,_29.2,(*_32),_29.2,_29.2];
RET = _46;
_15.0.fld6 = core::ptr::addr_of!(_31);
_24 = core::ptr::addr_of!(_62.2);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!(_50);
_39 = !_43;
place!(Field::<f32>(Variant((*_21), 2), 2)) = (*_45) as f32;
_52 = _5 | _5;
match _22 {
0 => bb11,
1 => bb2,
2 => bb40,
1947486470 => bb42,
_ => bb41
}
}
bb40 = {
_3 = RET;
_8 = _1;
_11 = 12653473696017041380_u64 | 15367281469381108351_u64;
RET = _3;
_10 = 660617473260587142048970026642946173_i128 as f64;
_1 = (-17747_i16) as u8;
_5 = !true;
_13 = [RET,_3,RET,_3,_3];
match _12 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211381 => bb8,
_ => bb7
}
}
bb41 = {
_2 = 9373_i16 as f32;
_5 = !false;
_2 = _1 as f32;
_4 = _1 * _1;
_1 = _4 << _4;
_5 = true;
_5 = _1 >= _1;
RET = _3;
_3 = RET;
_4 = !_1;
_1 = 728689971_i32 as u8;
RET = _3;
_5 = _4 != _4;
_4 = _1 + _1;
RET = _3;
_2 = 54027005728178793595851150403143967887_u128 as f32;
RET = _3;
_1 = 33772232133063800865750589457378122789_u128 as u8;
RET = _3;
_5 = true;
_5 = _4 >= _4;
_1 = _4;
_2 = 4298889839590723734_usize as f32;
_3 = RET;
_2 = (-10792_i16) as f32;
RET = _3;
_5 = !true;
Goto(bb2)
}
bb42 = {
(*_45) = _19.3 ^ _19.3;
_41 = [(*_45),(*_45),_38.3,(*_45),(*_45),(*_45),(*_45)];
_53.0 = !_12;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _15.0.fld0;
_12 = Field::<i8>(Variant((*_21), 2), 3) + Field::<i8>(Variant((*_21), 2), 3);
place!(Field::<f32>(Variant((*_21), 2), 2)) = -_2;
(*_24) = Adt22::Variant3 { fld0: _52,fld1: _36,fld2: Field::<i64>(Variant((*_21), 2), 4),fld3: (*_16),fld4: _29.1,fld5: Field::<f32>(Variant((*_21), 2), 2) };
place!(Field::<char>(Variant((*_24), 3), 1)) = _36;
match _38.2 {
64916826490002032122748762771291743660 => bb43,
_ => bb41
}
}
bb43 = {
(*_24) = Adt22::Variant0 { fld0: _52,fld1: _15.0.fld3,fld2: (*_26),fld3: _33,fld4: 0_usize,fld5: _47.fld5,fld6: _47.fld1,fld7: _60 };
_53 = (Field::<i8>(Variant((*_21), 2), 3), _11, Field::<u16>(Variant((*_24), 0), 5));
_17 = Field::<f32>(Variant((*_24), 0), 3) + Field::<f32>(Variant((*_24), 0), 3);
(*_24) = Adt22::Variant2 { fld0: _53 };
_62.3 = (*_26) & (*_26);
(*_24) = Adt22::Variant3 { fld0: _52,fld1: RET,fld2: Field::<i64>(Variant((*_21), 2), 4),fld3: (*_16),fld4: _29.3,fld5: Field::<f32>(Variant((*_21), 2), 2) };
place!(Field::<i64>(Variant((*_24), 3), 2)) = Field::<u32>(Variant((*_21), 2), 1) as i64;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _18 as i8;
(*_16) = Field::<u64>(Variant((*_24), 3), 3) >> (*_45);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = core::ptr::addr_of!((*_16));
(*_45) = _19.3 | _19.3;
place!(Field::<*const u64>(Variant(_15.1, 2), 0)) = core::ptr::addr_of!((*_16));
place!(Field::<i64>(Variant(_62.2, 3), 2)) = (*_45) as i64;
place!(Field::<u32>(Variant((*_21), 2), 1)) = Field::<i64>(Variant((*_24), 3), 2) as u32;
place!(Field::<f32>(Variant((*_21), 2), 2)) = Field::<f32>(Variant((*_24), 3), 5) + _51;
_21 = core::ptr::addr_of!((*_21));
place!(Field::<u64>(Variant((*_24), 3), 3)) = !(*_16);
_15.0.fld4 = !(*_16);
match _38.2 {
0 => bb19,
1 => bb6,
64916826490002032122748762771291743660 => bb44,
_ => bb28
}
}
bb44 = {
_60 = !_19.2;
place!(Field::<*const u64>(Variant(_15.1, 2), 0)) = core::ptr::addr_of!(_50);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 << (*_16);
_11 = (*_45) as u64;
place!(Field::<i64>(Variant(_15.1, 2), 4)) = !Field::<i64>(Variant((*_24), 3), 2);
place!(Field::<*const u64>(Variant((*_21), 2), 0)) = Move(_16);
_59 = Field::<char>(Variant((*_24), 3), 1);
place!(Field::<bool>(Variant((*_24), 3), 0)) = Field::<i64>(Variant((*_21), 2), 4) > Field::<i64>(Variant((*_24), 3), 2);
place!(Field::<u64>(Variant((*_24), 3), 3)) = _15.0.fld4 & _11;
place!(Field::<u32>(Variant((*_21), 2), 1)) = !_29.2;
_63 = Field::<u32>(Variant((*_21), 2), 1) as isize;
place!(Field::<[u16; 5]>(Variant(_62.2, 3), 4)) = [_53.2,_53.2,_53.2,_47.fld5,_18];
_78 = _23 | _23;
place!(Field::<char>(Variant((*_24), 3), 1)) = _46;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _37 * _47.fld0;
place!(Field::<char>(Variant((*_24), 3), 1)) = RET;
_15.0 = Move(_47);
place!(Field::<f32>(Variant((*_21), 2), 2)) = _33;
_29.0 = core::ptr::addr_of_mut!(place!(Field::<[u16; 5]>(Variant((*_24), 3), 4)));
(*_24) = Adt22::Variant2 { fld0: _53 };
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _53.2 + _53.2;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 * _51;
match _19.2 {
0 => bb41,
64916826490002032122748762771291743660 => bb45,
_ => bb20
}
}
bb45 = {
place!(Field::<(i8, u64, u16)>(Variant(_62.2, 2), 0)).2 = _15.0.fld5 >> Field::<i8>(Variant((*_21), 2), 3);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = Field::<i8>(Variant((*_21), 2), 3) >> Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).2;
(*_45) = _19.3;
_38 = (_19.0, _19.0, _60, _19.3);
_15.0.fld0 = Field::<f32>(Variant((*_21), 2), 2);
Goto(bb46)
}
bb46 = {
place!(Field::<*const u64>(Variant(_15.1, 2), 0)) = core::ptr::addr_of!(place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _53;
_17 = Field::<f32>(Variant((*_21), 2), 2) - Field::<f32>(Variant((*_21), 2), 2);
_17 = _19.2 as f32;
_48 = !_63;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _42 | _42;
(*_24) = Adt22::Variant0 { fld0: _52,fld1: _38.1,fld2: _48,fld3: Field::<f32>(Variant(_15.1, 2), 2),fld4: 4129616134134041739_usize,fld5: _18,fld6: _15.0.fld1,fld7: _60 };
place!(Field::<(u64,)>(Variant((*_24), 0), 6)) = (_53.1,);
place!(Field::<bool>(Variant((*_24), 0), 0)) = !_52;
place!(Field::<bool>(Variant((*_24), 0), 0)) = !_5;
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 * _29.2;
place!(Field::<u32>(Variant((*_21), 2), 1)) = (*_45) as u32;
place!(Field::<isize>(Variant((*_24), 0), 2)) = _48;
(*_24) = Adt22::Variant0 { fld0: _52,fld1: _38.0,fld2: _63,fld3: Field::<f32>(Variant((*_21), 2), 2),fld4: 16883318742703907389_usize,fld5: _15.0.fld5,fld6: _15.0.fld1,fld7: _19.2 };
_62.1.2 = Field::<u16>(Variant(_62.2, 0), 5) | _18;
place!(Field::<bool>(Variant((*_24), 0), 0)) = _5;
place!(Field::<f32>(Variant((*_21), 2), 2)) = Field::<f32>(Variant((*_24), 0), 3);
place!(Field::<u32>(Variant((*_21), 2), 1)) = _29.2 % _22;
(*_24) = Adt22::Variant0 { fld0: _52,fld1: _19.1,fld2: _63,fld3: _17,fld4: 4_usize,fld5: _15.0.fld5,fld6: _15.0.fld1,fld7: _38.2 };
_47.fld1 = (Field::<(u64,)>(Variant((*_24), 0), 6).0,);
match _19.2 {
0 => bb32,
1 => bb22,
2 => bb19,
3 => bb44,
64916826490002032122748762771291743660 => bb48,
_ => bb47
}
}
bb47 = {
_29.2 = _23 as u32;
place!(Field::<i64>(Variant((*_21), 2), 4)) = (*_7) as i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 4705831936782248566_i64 ^ 8163708743587683210_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = 8167864631328996001_i64 & 5577726668841165514_i64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = _17 as i64;
place!(Field::<f32>(Variant((*_21), 2), 2)) = _17 - _17;
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 >> _12;
_19.0 = 9223372036854775807_isize as f64;
place!(Field::<i64>(Variant((*_21), 2), 4)) = !(-9214834222469628886_i64);
place!(Field::<i8>(Variant((*_21), 2), 3)) = _12 + _12;
_19 = (_15.0.fld3, _15.0.fld3, 64916826490002032122748762771291743660_u128, (-5117_i16));
_15.0.fld1.0 = _15.0.fld4;
Goto(bb13)
}
bb48 = {
place!(Field::<(u64,)>(Variant((*_24), 0), 6)).0 = Field::<bool>(Variant((*_24), 0), 0) as u64;
_57 = Field::<(u64,)>(Variant((*_24), 0), 6).0 ^ _53.1;
_40 = Move((*_21));
place!(Field::<f32>(Variant((*_24), 0), 3)) = _15.0.fld0 * _15.0.fld0;
_79 = core::ptr::addr_of_mut!(_29.1);
match _19.2 {
0 => bb44,
64916826490002032122748762771291743660 => bb50,
_ => bb49
}
}
bb49 = {
_3 = RET;
_8 = _1;
_11 = 12653473696017041380_u64 | 15367281469381108351_u64;
RET = _3;
_10 = 660617473260587142048970026642946173_i128 as f64;
_1 = (-17747_i16) as u8;
_5 = !true;
_13 = [RET,_3,RET,_3,_3];
match _12 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211381 => bb8,
_ => bb7
}
}
bb50 = {
(*_21) = Move(_40);
_53.2 = _62.1.2;
(*_79) = [Field::<u16>(Variant((*_24), 0), 5),Field::<u16>(Variant((*_24), 0), 5),Field::<u16>(Variant((*_24), 0), 5),Field::<u16>(Variant(_62.2, 0), 5),Field::<u16>(Variant((*_24), 0), 5)];
place!(Field::<(u64,)>(Variant((*_24), 0), 6)).0 = _53.1 - _57;
_51 = Field::<f32>(Variant((*_24), 0), 3) + Field::<f32>(Variant((*_21), 2), 2);
_38.2 = Field::<u128>(Variant(_62.2, 0), 7);
_36 = _59;
place!(Field::<(u64,)>(Variant((*_24), 0), 6)).0 = _15.0.fld4 & _57;
(*_45) = -_19.3;
(*_24) = Adt22::Variant3 { fld0: _5,fld1: _14,fld2: Field::<i64>(Variant((*_21), 2), 4),fld3: _11,fld4: _29.1,fld5: Field::<f32>(Variant((*_21), 2), 2) };
Goto(bb51)
}
bb51 = {
Call(_93 = dump_var(Move(_1), Move(_59), Move(_41), Move(_52)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_93 = dump_var(Move(_14), Move(_11), Move(_48), Move(_39)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_93 = dump_var(Move(_43), Move(_12), Move(_23), Move(_53)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_93 = dump_var(Move(_46), Move(_60), Move(_57), _94), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: u8,mut _3: char,mut _4: u8,mut _5: char,mut _6: f32,mut _7: u8,mut _8: f32) -> char {
mir! {
type RET = char;
let _9: *const *const (u64,);
let _10: u8;
let _11: *mut i128;
let _12: &'static mut &'static mut f32;
let _13: [i64; 3];
let _14: &'static isize;
let _15: &'static mut f32;
let _16: usize;
let _17: (f64, f64, u128, i16);
let _18: &'static mut [char; 6];
let _19: Adt75;
let _20: [i64; 6];
let _21: i32;
let _22: &'static mut f64;
let _23: char;
let _24: (isize, u8, Adt49);
let _25: isize;
let _26: *const u8;
let _27: &'static mut *const u64;
let _28: &'static f32;
let _29: *const *const (u64,);
let _30: [u128; 7];
let _31: f64;
let _32: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _33: isize;
let _34: [u32; 1];
let _35: bool;
let _36: (&'static isize, *const (u64,), i32);
let _37: f32;
let _38: i32;
let _39: char;
let _40: (f64, f64, u128, i16);
let _41: isize;
let _42: bool;
let _43: usize;
let _44: Adt26;
let _45: (*const u64, (i8, u64, u16), Adt22, isize);
let _46: &'static &'static mut &'static mut f32;
let _47: *const Adt41;
let _48: &'static mut *mut i128;
let _49: f32;
let _50: isize;
let _51: &'static mut f32;
let _52: *mut i128;
let _53: (&'static mut u32,);
let _54: bool;
let _55: usize;
let _56: f32;
let _57: &'static mut f32;
let _58: [i64; 3];
let _59: [i64; 3];
let _60: *const (u64,);
let _61: i32;
let _62: &'static mut &'static mut f32;
let _63: i32;
let _64: [i16; 7];
let _65: [isize; 6];
let _66: i8;
let _67: u128;
let _68: bool;
let _69: &'static mut *mut i128;
let _70: &'static mut &'static mut f32;
let _71: usize;
let _72: i64;
let _73: u8;
let _74: *const *const (u64,);
let _75: Adt49;
let _76: f32;
let _77: bool;
let _78: u64;
let _79: [i8; 2];
let _80: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _81: u128;
let _82: &'static mut u32;
let _83: f32;
let _84: *mut i128;
let _85: *const &'static mut f32;
let _86: f32;
let _87: *const (u64,);
let _88: &'static mut f64;
let _89: f64;
let _90: isize;
let _91: *const Adt41;
let _92: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _93: [u8; 4];
let _94: usize;
let _95: usize;
let _96: u32;
let _97: (&'static mut u32,);
let _98: u64;
let _99: [u32; 7];
let _100: (Adt26, Adt41);
let _101: bool;
let _102: char;
let _103: &'static isize;
let _104: &'static u32;
let _105: &'static f32;
let _106: i32;
let _107: isize;
let _108: *mut &'static mut [char; 6];
let _109: isize;
let _110: [u64; 2];
let _111: (i8, u64, u16);
let _112: Adt80;
let _113: u128;
let _114: char;
let _115: *const (u64,);
let _116: &'static isize;
let _117: [i64; 6];
let _118: *mut i128;
let _119: u8;
let _120: (i64, u64, &'static mut *const u64);
let _121: (i64, u64, &'static mut *const u64);
let _122: Adt75;
let _123: isize;
let _124: isize;
let _125: *const *const (u64,);
let _126: *const Adt41;
let _127: &'static &'static mut &'static mut f32;
let _128: bool;
let _129: *const (u64,);
let _130: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _131: u64;
let _132: (*const u64, (i8, u64, u16), Adt22, isize);
let _133: f32;
let _134: *const (u64,);
let _135: ();
let _136: ();
{
_5 = _3;
RET = _3;
_4 = !_7;
_4 = _7 * _7;
_5 = _1;
_10 = _7 + _7;
_1 = _5;
_6 = _8 * _8;
_5 = _1;
_4 = _7 - _10;
_7 = _10 | _4;
_13 = [(-5998493647604229080_i64),8867113661845403303_i64,(-5348958164144939511_i64)];
_8 = _6;
_7 = _4 >> _4;
_2 = _7 & _4;
_2 = _4 * _7;
_1 = _5;
_5 = _3;
Goto(bb1)
}
bb1 = {
_5 = _3;
Goto(bb2)
}
bb2 = {
_1 = _5;
_8 = -_6;
_6 = 9289119480671461121_u64 as f32;
_8 = _6 - _6;
_17.0 = (-75_i8) as f64;
_5 = _3;
_6 = _8;
_17.2 = 137578162786092821184886544331205429538_u128 ^ 92144283371467027792896830884769452870_u128;
_17.0 = 5624176141186673724_u64 as f64;
_17.3 = 16367_i16 << _2;
RET = _3;
_7 = _2;
Goto(bb3)
}
bb3 = {
_3 = _5;
_17.2 = !127681219109159905176786690291714675550_u128;
_16 = !11331466898324895249_usize;
_20 = [(-7587962766561221222_i64),(-6114623048044691578_i64),(-331324684292729453_i64),4697897144527134627_i64,6457874762601632536_i64,(-1912530512217347897_i64)];
_7 = _10 ^ _2;
_7 = _17.2 as u8;
_15 = &mut _8;
(*_15) = _6;
(*_15) = _6 * _6;
_12 = &mut _15;
_17.0 = _17.2 as f64;
(*_12) = &mut _6;
_10 = _2 + _4;
_7 = !_10;
_17.1 = (-110_i8) as f64;
Call(_13 = fn2(Move((*_12)), RET, RET, _17.1, _17.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17.0 = _17.1;
_20 = [(-668931423659365054_i64),(-1587643612638056116_i64),(-5513884718349301538_i64),3489372900203665695_i64,4636109813993766956_i64,(-552918515614536910_i64)];
_22 = &mut _17.1;
(*_22) = 14697679365468473520784186430633063597_u128 as f64;
_21 = 301858222_u32 as i32;
_23 = RET;
(*_22) = _21 as f64;
(*_22) = 907603289_u32 as f64;
(*_22) = (-26517_i16) as f64;
_21 = _16 as i32;
(*_22) = (-70389084953033019147861958451207308808_i128) as f64;
_24.0 = 76_isize << _2;
(*_22) = (-32363_i16) as f64;
(*_22) = 34599_u16 as f64;
_24.1 = 67600683567461558803426579264530377778_u128 as u8;
_26 = core::ptr::addr_of!(_24.1);
(*_26) = !_2;
(*_26) = _7 * _2;
(*_26) = _2 - _10;
(*_22) = 57846_u16 as f64;
(*_22) = 95476651938253740629480783297777048083_u128 as f64;
(*_26) = _2;
_14 = &_24.0;
_24.2.fld1 = [(*_14),(*_14),_24.0,(*_14),_24.0,(*_14)];
(*_26) = (-3116651967525550111_i64) as u8;
_3 = RET;
Goto(bb5)
}
bb5 = {
(*_22) = 16316_i16 as f64;
_5 = _23;
(*_26) = _3 as u8;
_23 = _1;
_21 = -499524584_i32;
(*_26) = _7;
_2 = (*_26) ^ (*_26);
(*_22) = 44614634551485107682938471019981354859_i128 as f64;
_25 = -(*_14);
(*_22) = _16 as f64;
_16 = 5_usize;
(*_26) = !_2;
(*_22) = _20[_16] as f64;
(*_22) = 53514_u16 as f64;
_9 = core::ptr::addr_of!(_32.0.1);
_29 = core::ptr::addr_of!((*_9));
(*_26) = _7 & _10;
_25 = !_24.0;
(*_26) = _10 << _4;
_31 = (*_22);
match _16 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
6 => bb9,
5 => bb11,
_ => bb10
}
}
bb6 = {
_17.0 = _17.1;
_20 = [(-668931423659365054_i64),(-1587643612638056116_i64),(-5513884718349301538_i64),3489372900203665695_i64,4636109813993766956_i64,(-552918515614536910_i64)];
_22 = &mut _17.1;
(*_22) = 14697679365468473520784186430633063597_u128 as f64;
_21 = 301858222_u32 as i32;
_23 = RET;
(*_22) = _21 as f64;
(*_22) = 907603289_u32 as f64;
(*_22) = (-26517_i16) as f64;
_21 = _16 as i32;
(*_22) = (-70389084953033019147861958451207308808_i128) as f64;
_24.0 = 76_isize << _2;
(*_22) = (-32363_i16) as f64;
(*_22) = 34599_u16 as f64;
_24.1 = 67600683567461558803426579264530377778_u128 as u8;
_26 = core::ptr::addr_of!(_24.1);
(*_26) = !_2;
(*_26) = _7 * _2;
(*_26) = _2 - _10;
(*_22) = 57846_u16 as f64;
(*_22) = 95476651938253740629480783297777048083_u128 as f64;
(*_26) = _2;
_14 = &_24.0;
_24.2.fld1 = [(*_14),(*_14),_24.0,(*_14),_24.0,(*_14)];
(*_26) = (-3116651967525550111_i64) as u8;
_3 = RET;
Goto(bb5)
}
bb7 = {
_3 = _5;
_17.2 = !127681219109159905176786690291714675550_u128;
_16 = !11331466898324895249_usize;
_20 = [(-7587962766561221222_i64),(-6114623048044691578_i64),(-331324684292729453_i64),4697897144527134627_i64,6457874762601632536_i64,(-1912530512217347897_i64)];
_7 = _10 ^ _2;
_7 = _17.2 as u8;
_15 = &mut _8;
(*_15) = _6;
(*_15) = _6 * _6;
_12 = &mut _15;
_17.0 = _17.2 as f64;
(*_12) = &mut _6;
_10 = _2 + _4;
_7 = !_10;
_17.1 = (-110_i8) as f64;
Call(_13 = fn2(Move((*_12)), RET, RET, _17.1, _17.1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1 = _5;
_8 = -_6;
_6 = 9289119480671461121_u64 as f32;
_8 = _6 - _6;
_17.0 = (-75_i8) as f64;
_5 = _3;
_6 = _8;
_17.2 = 137578162786092821184886544331205429538_u128 ^ 92144283371467027792896830884769452870_u128;
_17.0 = 5624176141186673724_u64 as f64;
_17.3 = 16367_i16 << _2;
RET = _3;
_7 = _2;
Goto(bb3)
}
bb9 = {
_5 = _3;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
(*_26) = _7 << (*_14);
_26 = core::ptr::addr_of!((*_26));
RET = _1;
(*_26) = !_2;
_22 = &mut _31;
(*_26) = _2 ^ _2;
_24.2.fld1 = [(*_14),(*_14),(*_14),(*_14),(*_14),(*_14)];
(*_26) = !_2;
_24.2.fld1 = [(*_14),(*_14),(*_14),(*_14),(*_14),(*_14)];
_7 = 2640374344388601609_u64 as u8;
_34 = [987469747_u32];
match _20[_16] {
340282366920938463462821688916153674546 => bb13,
_ => bb12
}
}
bb12 = {
_17.0 = _17.1;
_20 = [(-668931423659365054_i64),(-1587643612638056116_i64),(-5513884718349301538_i64),3489372900203665695_i64,4636109813993766956_i64,(-552918515614536910_i64)];
_22 = &mut _17.1;
(*_22) = 14697679365468473520784186430633063597_u128 as f64;
_21 = 301858222_u32 as i32;
_23 = RET;
(*_22) = _21 as f64;
(*_22) = 907603289_u32 as f64;
(*_22) = (-26517_i16) as f64;
_21 = _16 as i32;
(*_22) = (-70389084953033019147861958451207308808_i128) as f64;
_24.0 = 76_isize << _2;
(*_22) = (-32363_i16) as f64;
(*_22) = 34599_u16 as f64;
_24.1 = 67600683567461558803426579264530377778_u128 as u8;
_26 = core::ptr::addr_of!(_24.1);
(*_26) = !_2;
(*_26) = _7 * _2;
(*_26) = _2 - _10;
(*_22) = 57846_u16 as f64;
(*_22) = 95476651938253740629480783297777048083_u128 as f64;
(*_26) = _2;
_14 = &_24.0;
_24.2.fld1 = [(*_14),(*_14),_24.0,(*_14),_24.0,(*_14)];
(*_26) = (-3116651967525550111_i64) as u8;
_3 = RET;
Goto(bb5)
}
bb13 = {
(*_22) = _21 as f64;
(*_26) = _21 as u8;
(*_26) = _2 << _24.2.fld1[_16];
(*_22) = 12518759485781705312_u64 as f64;
(*_26) = _2;
_38 = _21;
(*_26) = _38 as u8;
_30 = [107698389516590640051344981369807037488_u128,170931495715731613454742314168717680515_u128,320058546012485524661101731279936294968_u128,112693338137468425742403628934074082764_u128,166356303492996213035459050316208565932_u128,59780456650511605078650699979872278126_u128,316037012653163699949748042141657895418_u128];
(*_26) = !_4;
_36.0 = Move(_14);
(*_22) = 19682_u16 as f64;
(*_26) = _2;
(*_22) = (-72460557346346904733182099412283093457_i128) as f64;
_3 = _5;
(*_22) = _30[_16] as f64;
(*_26) = 83_i8 as u8;
_26 = core::ptr::addr_of!(_4);
(*_22) = 3726387242158782948_u64 as f64;
_20[_16] = 7695857078686153134_i64 << (*_26);
(*_26) = _2 - _10;
(*_22) = _30[_16] as f64;
match _30[_16] {
59780456650511605078650699979872278126 => bb15,
_ => bb14
}
}
bb14 = {
_5 = _3;
Goto(bb2)
}
bb15 = {
(*_22) = _30[_16] as f64;
(*_26) = _16 as u8;
_2 = (*_26) << (*_26);
(*_22) = _21 as f64;
_24.0 = _25;
_36.2 = _38 * _21;
(*_22) = 722706237_u32 as f64;
_40.2 = _30[_16] << _30[_16];
_9 = core::ptr::addr_of!((*_29));
(*_22) = 71929118_u32 as f64;
_36.0 = &_24.0;
_37 = 32_i8 as f32;
_39 = _5;
(*_22) = 27445_u16 as f64;
_41 = _25;
_40.1 = (*_22) + (*_22);
(*_22) = _40.1 + _40.1;
(*_26) = _10;
_42 = (*_26) == (*_26);
(*_22) = _40.1 * _40.1;
_35 = (*_26) != _24.1;
(*_26) = _2 - _10;
_35 = _42;
match _30[_16] {
0 => bb9,
1 => bb6,
2 => bb3,
59780456650511605078650699979872278126 => bb16,
_ => bb5
}
}
bb16 = {
(*_22) = _40.1 + _40.1;
(*_26) = !_10;
(*_9) = core::ptr::addr_of!(_44.fld1);
(*_22) = _40.1 - _40.1;
(*_29) = core::ptr::addr_of!(_44.fld1);
(*_26) = _10;
(*_26) = _10 >> _10;
(*_22) = _40.1;
(*_22) = _40.1 + _40.1;
_44.fld3 = (*_22) - (*_22);
_24.0 = _25;
_44.fld1.0 = 16063020444051218189_u64;
(*_22) = -_40.1;
Goto(bb17)
}
bb17 = {
_44.fld1 = (9792082729780951561_u64,);
(*_22) = 917030512_u32 as f64;
_44.fld5 = !1654_u16;
(*_29) = core::ptr::addr_of!(_44.fld1);
_37 = (-70_i8) as f32;
_16 = 12042238685000840355_usize;
_40.3 = 32_i8 as i16;
_25 = _24.0;
(*_26) = _2;
_44.fld0 = -_37;
_40.1 = (*_22) * (*_22);
match _16 {
0 => bb15,
1 => bb2,
2 => bb3,
12042238685000840355 => bb19,
_ => bb18
}
}
bb18 = {
_5 = _3;
Goto(bb2)
}
bb19 = {
(*_29) = core::ptr::addr_of!(_44.fld1);
(*_26) = _24.1;
(*_22) = _44.fld3 - _44.fld3;
Goto(bb20)
}
bb20 = {
(*_22) = -_44.fld3;
_36.1 = core::ptr::addr_of!(_44.fld1);
_45.1 = (22_i8, _44.fld1.0, _44.fld5);
_36.0 = &_25;
_23 = _1;
_15 = &mut _44.fld0;
_25 = _24.0 << _41;
_45.1.2 = (*_22) as u16;
(*_29) = Move(_36.1);
_50 = _41 + _24.0;
_40.3 = 5564_i16 << (*_26);
_43 = _16;
(*_15) = -_37;
(*_15) = _37 + _37;
_38 = -_36.2;
(*_22) = -_40.1;
(*_22) = (-582729975204963195_i64) as f64;
match _45.1.0 {
0 => bb7,
1 => bb21,
2 => bb22,
22 => bb24,
_ => bb23
}
}
bb21 = {
_17.0 = _17.1;
_20 = [(-668931423659365054_i64),(-1587643612638056116_i64),(-5513884718349301538_i64),3489372900203665695_i64,4636109813993766956_i64,(-552918515614536910_i64)];
_22 = &mut _17.1;
(*_22) = 14697679365468473520784186430633063597_u128 as f64;
_21 = 301858222_u32 as i32;
_23 = RET;
(*_22) = _21 as f64;
(*_22) = 907603289_u32 as f64;
(*_22) = (-26517_i16) as f64;
_21 = _16 as i32;
(*_22) = (-70389084953033019147861958451207308808_i128) as f64;
_24.0 = 76_isize << _2;
(*_22) = (-32363_i16) as f64;
(*_22) = 34599_u16 as f64;
_24.1 = 67600683567461558803426579264530377778_u128 as u8;
_26 = core::ptr::addr_of!(_24.1);
(*_26) = !_2;
(*_26) = _7 * _2;
(*_26) = _2 - _10;
(*_22) = 57846_u16 as f64;
(*_22) = 95476651938253740629480783297777048083_u128 as f64;
(*_26) = _2;
_14 = &_24.0;
_24.2.fld1 = [(*_14),(*_14),_24.0,(*_14),_24.0,(*_14)];
(*_26) = (-3116651967525550111_i64) as u8;
_3 = RET;
Goto(bb5)
}
bb22 = {
(*_22) = _30[_16] as f64;
(*_26) = _16 as u8;
_2 = (*_26) << (*_26);
(*_22) = _21 as f64;
_24.0 = _25;
_36.2 = _38 * _21;
(*_22) = 722706237_u32 as f64;
_40.2 = _30[_16] << _30[_16];
_9 = core::ptr::addr_of!((*_29));
(*_22) = 71929118_u32 as f64;
_36.0 = &_24.0;
_37 = 32_i8 as f32;
_39 = _5;
(*_22) = 27445_u16 as f64;
_41 = _25;
_40.1 = (*_22) + (*_22);
(*_22) = _40.1 + _40.1;
(*_26) = _10;
_42 = (*_26) == (*_26);
(*_22) = _40.1 * _40.1;
_35 = (*_26) != _24.1;
(*_26) = _2 - _10;
_35 = _42;
match _30[_16] {
0 => bb9,
1 => bb6,
2 => bb3,
59780456650511605078650699979872278126 => bb16,
_ => bb5
}
}
bb23 = {
_1 = _5;
_8 = -_6;
_6 = 9289119480671461121_u64 as f32;
_8 = _6 - _6;
_17.0 = (-75_i8) as f64;
_5 = _3;
_6 = _8;
_17.2 = 137578162786092821184886544331205429538_u128 ^ 92144283371467027792896830884769452870_u128;
_17.0 = 5624176141186673724_u64 as f64;
_17.3 = 16367_i16 << _2;
RET = _3;
_7 = _2;
Goto(bb3)
}
bb24 = {
(*_26) = 26127369399998586128001703081457021229_i128 as u8;
(*_26) = _10;
_56 = 4177882828_u32 as f32;
_45.3 = -_24.0;
(*_26) = _10 >> _45.3;
_14 = &_45.3;
_22 = &mut _40.1;
(*_22) = _38 as f64;
(*_15) = -_37;
(*_26) = _45.1.0 as u8;
_57 = &mut _56;
_28 = &(*_57);
_45.1.1 = !11022726020077669140_u64;
_35 = (*_14) >= (*_14);
Goto(bb25)
}
bb25 = {
(*_15) = (*_28) - _37;
(*_57) = -_37;
_36.2 = _3 as i32;
(*_22) = _45.1.2 as f64;
_46 = &_12;
(*_26) = _10;
_42 = _35;
(*_57) = (*_15);
_16 = !_43;
(*_22) = (*_26) as f64;
_54 = _42;
(*_22) = 293438116057583662132927042194294751910_u128 as f64;
_36 = (Move(_14), Move((*_29)), _21);
(*_15) = (*_22) as f32;
(*_29) = Move(_36.1);
_7 = (*_26) + (*_26);
_9 = core::ptr::addr_of!((*_29));
match _45.1.0 {
22 => bb27,
_ => bb26
}
}
bb26 = {
(*_26) = 26127369399998586128001703081457021229_i128 as u8;
(*_26) = _10;
_56 = 4177882828_u32 as f32;
_45.3 = -_24.0;
(*_26) = _10 >> _45.3;
_14 = &_45.3;
_22 = &mut _40.1;
(*_22) = _38 as f64;
(*_15) = -_37;
(*_26) = _45.1.0 as u8;
_57 = &mut _56;
_28 = &(*_57);
_45.1.1 = !11022726020077669140_u64;
_35 = (*_14) >= (*_14);
Goto(bb25)
}
bb27 = {
(*_26) = !_10;
(*_15) = (*_57);
_26 = core::ptr::addr_of!((*_26));
_51 = &mut (*_57);
(*_51) = (*_26) as f32;
(*_15) = -(*_51);
_24.2.fld0 = _45.1.1;
(*_22) = (-25166_i16) as f64;
(*_26) = _7 & _7;
(*_22) = _45.1.0 as f64;
_45.0 = core::ptr::addr_of!(_24.2.fld0);
_63 = _21 + _21;
(*_22) = (*_15) as f64;
(*_26) = _10;
(*_22) = (*_15) as f64;
(*_26) = _7 ^ _7;
(*_51) = (*_15);
(*_22) = (-12772_i16) as f64;
_20 = [3027229649465334736_i64,(-7420891790476686580_i64),25146685037089698_i64,(-6358484406454784290_i64),(-7148404094887572543_i64),2666028861341555327_i64];
_41 = _45.3;
(*_15) = 27917169316559413259728114764477547674_i128 as f32;
(*_26) = _39 as u8;
_64 = [18103_i16,(-21238_i16),(-1773_i16),(-29788_i16),(-4857_i16),(-32687_i16),(-24907_i16)];
_37 = (*_51) + (*_51);
_28 = &(*_15);
_58 = [(-3104238432495599165_i64),(-3597420398077585420_i64),(-8787369991624464278_i64)];
(*_51) = (*_15) + _37;
match _45.1.0 {
22 => bb29,
_ => bb28
}
}
bb28 = {
(*_29) = core::ptr::addr_of!(_44.fld1);
(*_26) = _24.1;
(*_22) = _44.fld3 - _44.fld3;
Goto(bb20)
}
bb29 = {
(*_22) = _38 as f64;
_14 = &_45.3;
_21 = _38;
_27 = &mut _45.0;
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_43 = _16;
(*_15) = (*_51);
_36.0 = &_25;
(*_15) = -(*_51);
(*_15) = _37 * (*_51);
(*_51) = 77_i8 as f32;
(*_51) = (*_15);
_36 = (Move(_14), Move((*_29)), _21);
(*_29) = Move(_36.1);
(*_51) = _24.2.fld0 as f32;
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_51) = (*_15) + (*_15);
_20 = [7839827620936289945_i64,4817403003632921681_i64,419662206452410602_i64,(-3414255723879479818_i64),4244058172890865403_i64,(-2848226616802413697_i64)];
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_65 = [_50,_41,_25,_25,_41,_50];
(*_26) = !_10;
RET = _39;
_13 = _58;
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_36.0 = &_24.0;
Call((*_51) = core::intrinsics::transmute(_3), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_23 = _3;
_55 = 61383096498462438107928178416378791864_i128 as usize;
(*_15) = -(*_51);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_10 = !(*_26);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
Goto(bb31)
}
bb31 = {
_36.1 = Move((*_29));
_21 = _36.2 | _38;
(*_29) = Move(_36.1);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_26) = !_7;
(*_15) = -_37;
(*_22) = _24.2.fld0 as f64;
_33 = (*_22) as isize;
(*_22) = _50 as f64;
(*_51) = -(*_15);
_51 = &mut (*_15);
(*_26) = !_7;
(*_26) = _10 & _10;
_51 = &mut _37;
Goto(bb32)
}
bb32 = {
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_66 = -17_i8;
(*_26) = _7 & _10;
_38 = _36.2 ^ _21;
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_22) = 233637970066710301090447681072783448431_u128 as f64;
_72 = !4788347588710824306_i64;
_30 = [90845024585041985006932060289449290732_u128,253353663955053385655096502422879927500_u128,236109183107787520634326735088861960479_u128,316625851917668160184949496425950774163_u128,17733649178437908329857949579210770447_u128,294216272330409619034249924932310754723_u128,105677797964944232129095871137619722272_u128];
(*_27) = core::ptr::addr_of!(_75.fld0);
_76 = -(*_51);
_64 = [(-4723_i16),(-871_i16),10479_i16,(-25686_i16),(-14239_i16),3233_i16,22949_i16];
(*_27) = core::ptr::addr_of!(_75.fld0);
_75.fld1 = [_24.0,_25,_50,_33,_50,_25];
_67 = !30225551438873571531133585805291323554_u128;
_49 = (*_51) + (*_51);
(*_26) = _10 >> _10;
_73 = (*_26);
(*_27) = core::ptr::addr_of!(_24.2.fld0);
(*_22) = _36.2 as f64;
_42 = _35;
_24.1 = _50 as u8;
_36.1 = Move((*_29));
Goto(bb33)
}
bb33 = {
_39 = _5;
(*_51) = _76 + _49;
(*_29) = Move(_36.1);
_70 = Move(_12);
(*_51) = _76 * _49;
(*_26) = _7 << _7;
(*_51) = _49;
_79 = [_66,_66];
(*_22) = _72 as f64;
Goto(bb34)
}
bb34 = {
(*_26) = _7;
Goto(bb35)
}
bb35 = {
(*_26) = !_73;
(*_27) = core::ptr::addr_of!(_78);
(*_27) = core::ptr::addr_of!(_78);
(*_26) = _67 as u8;
(*_22) = _72 as f64;
_68 = !_54;
(*_27) = core::ptr::addr_of!(_75.fld0);
(*_51) = _16 as f32;
_28 = &(*_51);
_65 = [_24.0,_50,_25,_50,_50,_25];
_63 = (-83311874045030153220111146328925762471_i128) as i32;
_12 = &mut _57;
_1 = _3;
(*_51) = _49 - _49;
(*_27) = core::ptr::addr_of!(_75.fld0);
_30 = [_67,_67,_67,_67,_67,_67,_67];
(*_27) = core::ptr::addr_of!(_24.2.fld0);
_80.1 = 45330726883230333377783737415323359401_i128 as u32;
Goto(bb36)
}
bb36 = {
_80.0 = Move(_27);
_75.fld0 = _24.2.fld0 ^ _24.2.fld0;
(*_12) = &mut (*_51);
_14 = &_33;
(*_22) = _24.0 as f64;
(*_26) = _73 ^ _7;
(*_26) = _24.1 - _7;
_78 = (-12988_i16) as u64;
_16 = !_43;
(*_26) = _7 | _7;
_65 = _75.fld1;
_89 = _49 as f64;
_83 = _67 as f32;
(*_22) = _49 as f64;
(*_22) = _89;
(*_22) = _89 + _89;
Goto(bb37)
}
bb37 = {
_67 = 32094527329169640755683131002334247078_u128 ^ 285547557901234347661334472307968046928_u128;
_66 = _36.2 as i8;
(*_26) = _7 * _7;
_77 = !_68;
_36.1 = Move((*_29));
Goto(bb38)
}
bb38 = {
(*_29) = Move(_36.1);
_58 = [_72,_72,_72];
(*_12) = &mut _76;
_36 = (Move(_14), Move((*_29)), _21);
_74 = core::ptr::addr_of!((*_29));
(*_12) = &mut _49;
(*_26) = _7 >> _50;
(*_22) = _89;
(*_26) = !_24.1;
Goto(bb39)
}
bb39 = {
_83 = _72 as f32;
(*_74) = Move(_36.1);
_50 = 37250_u16 as isize;
_92.0.1 = Move((*_74));
_35 = _42 == _42;
(*_74) = Move(_92.0.1);
(*_12) = &mut _83;
(*_12) = Move(_51);
_43 = !_16;
_78 = _24.2.fld0;
(*_12) = Move(_15);
(*_29) = core::ptr::addr_of!(_100.0.fld1);
_26 = core::ptr::addr_of!((*_26));
_66 = (-19547250618995853881078831381613664168_i128) as i8;
_25 = _23 as isize;
_100.0.fld0 = _78 as f32;
_77 = _68;
(*_22) = _89 + _89;
(*_29) = core::ptr::addr_of!(_100.0.fld1);
(*_22) = _89 + _89;
(*_22) = _80.1 as f64;
(*_29) = core::ptr::addr_of!(_100.0.fld1);
_62 = Move(_12);
(*_26) = _73;
_100.0.fld1 = (_78,);
Goto(bb40)
}
bb40 = {
(*_29) = core::ptr::addr_of!(_100.0.fld1);
_64 = [(-1354_i16),32491_i16,24084_i16,13737_i16,(-4066_i16),6936_i16,29217_i16];
_80.3.1 = 29637_u16;
(*_22) = -_89;
(*_29) = core::ptr::addr_of!(_100.0.fld1);
(*_22) = _100.0.fld0 as f64;
(*_29) = core::ptr::addr_of!(_100.0.fld1);
_100.0.fld2 = [_67,_67,_67,_67,_67,_67,_67];
(*_29) = core::ptr::addr_of!(_100.0.fld1);
_7 = _10 ^ (*_26);
_94 = _16 & _16;
Goto(bb41)
}
bb41 = {
_51 = &mut _100.0.fld0;
_90 = _41 << (*_26);
_59 = [_72,_72,_72];
_24 = (_90, _4, Move(_75));
_99 = [_80.1,_80.1,_80.1,_80.1,_80.1,_80.1,_80.1];
_80.3.3 = _5;
_35 = !_54;
(*_51) = _24.2.fld0 as f32;
Goto(bb42)
}
bb42 = {
_59 = _13;
_92.0.1 = Move((*_29));
_61 = _21 | _21;
(*_29) = Move(_92.0.1);
(*_51) = _80.3.1 as f32;
(*_22) = _89 - _89;
_75.fld0 = _24.2.fld0 << _90;
RET = _80.3.3;
_97.0 = &mut _80.1;
Goto(bb43)
}
bb43 = {
_75.fld0 = _78;
(*_26) = !_73;
(*_26) = !_7;
(*_51) = _67 as f32;
_86 = (*_51) + (*_51);
_58 = _13;
(*_26) = _7 * _73;
_96 = 1552536452_u32 - 1396934138_u32;
(*_26) = !_7;
(*_26) = _7;
(*_22) = _89;
_20 = [_72,_72,_72,_72,_72,_72];
_36.2 = _38;
(*_51) = -_86;
_81 = !_67;
_4 = !_24.1;
_14 = &_25;
(*_51) = _86;
(*_51) = -_86;
_97.0 = &mut _96;
_55 = _94;
(*_26) = _24.1;
_72 = _43 as i64;
Goto(bb44)
}
bb44 = {
_24.2.fld1 = [_90,(*_14),_24.0,_41,_41,_41];
(*_22) = _24.2.fld0 as f64;
(*_26) = _73 - _7;
_50 = _90 >> (*_26);
_98 = _24.2.fld0;
_82 = Move(_97.0);
_75 = Adt49 { fld0: _24.2.fld0,fld1: _24.2.fld1 };
(*_51) = _86;
_13 = _59;
_58 = [_72,_72,_72];
_36.2 = !_61;
_41 = _90;
_3 = _23;
(*_22) = _89 - _89;
_62 = &mut _51;
_75 = Adt49 { fld0: _98,fld1: _65 };
_111.1 = _75.fld0 + _75.fld0;
_88 = &mut _89;
Goto(bb45)
}
bb45 = {
_93 = [(*_26),(*_26),(*_26),(*_26)];
_109 = _41;
(*_88) = -(*_22);
(*_26) = _10;
(*_62) = &mut _86;
_63 = _38;
_38 = _63 + _21;
_106 = -_36.2;
(*_88) = (*_22);
_113 = 3701522933_u32 as u128;
(*_22) = -(*_88);
(*_22) = -(*_88);
_85 = core::ptr::addr_of!((*_62));
_111 = (_66, _75.fld0, 61638_u16);
(*_26) = !_73;
_38 = _106;
_92.0.1 = Move((*_29));
(*_26) = _24.1 << _24.1;
_109 = -_41;
_74 = core::ptr::addr_of!((*_29));
(*_74) = Move(_92.0.1);
(*_88) = (*_22);
(*_22) = _109 as f64;
_116 = &_33;
Goto(bb46)
}
bb46 = {
_92.0.1 = Move((*_29));
_87 = Move(_92.0.1);
_66 = _111.0;
_43 = _94 + _55;
_79 = [_111.0,_66];
_38 = _63 >> (*_26);
_90 = _50 ^ (*_14);
(*_88) = (*_22);
(*_29) = Move(_87);
_29 = core::ptr::addr_of!((*_29));
_72 = _98 as i64;
_24.0 = _109;
_14 = &(*_116);
(*_26) = !_73;
(*_26) = _7 & _7;
Goto(bb47)
}
bb47 = {
_24.2.fld0 = !_111.1;
_36 = (Move(_116), Move((*_29)), _38);
_116 = &(*_14);
_77 = _54 & _42;
Goto(bb48)
}
bb48 = {
(*_29) = Move(_36.1);
(*_88) = (*_22) - (*_22);
(*_22) = (*_26) as f64;
(*_22) = (*_88) * (*_88);
_24.2.fld1 = [_24.0,_50,_90,_41,_109,_109];
_24.0 = _109 ^ _50;
(*_88) = -(*_22);
_72 = (-3929234473542178579_i64);
_9 = core::ptr::addr_of!((*_29));
_64 = [3057_i16,19890_i16,(-23752_i16),(-23188_i16),19067_i16,20657_i16,10280_i16];
(*_26) = _24.1 + _7;
(*_26) = 645544840_u32 as u8;
_22 = &mut (*_88);
_21 = -_36.2;
_102 = _3;
(*_26) = !_24.1;
_95 = 15926_i16 as usize;
_4 = _73 | _10;
_9 = core::ptr::addr_of!((*_9));
_92.0.1 = Move((*_29));
(*_29) = Move(_92.0.1);
(*_22) = _111.2 as f64;
Goto(bb49)
}
bb49 = {
_38 = _21 ^ _21;
_90 = _67 as isize;
_36.0 = &(*_116);
_4 = _24.1 ^ _24.1;
(*_26) = !_7;
_111.1 = _55 as u64;
_130.0.1 = Move((*_29));
_70 = &mut (*_62);
_23 = RET;
_87 = Move(_130.0.1);
_103 = &(*_14);
_106 = _36.2;
_12 = &mut (*_70);
_120.0 = !_72;
_107 = -_24.0;
(*_29) = Move(_87);
_41 = !_50;
_36.0 = &_50;
_132.3 = _107;
_93 = [(*_26),(*_26),(*_26),(*_26)];
_95 = !_55;
(*_22) = _109 as f64;
_133 = (-78051822143252235516697182773499306385_i128) as f32;
(*_12) = &mut _133;
_36.2 = -_21;
_88 = Move(_22);
_71 = _16 & _94;
(*_26) = _7;
Goto(bb50)
}
bb50 = {
Call(_135 = dump_var(Move(_93), Move(_61), Move(_25), Move(_95)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_135 = dump_var(Move(_106), Move(_72), Move(_54), Move(_78)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_135 = dump_var(Move(_20), Move(_4), Move(_111), Move(_21)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_135 = dump_var(Move(_96), Move(_94), Move(_99), Move(_23)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_135 = dump_var(Move(_107), Move(_67), Move(_59), Move(_109)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_135 = dump_var(Move(_50), Move(_71), Move(_42), Move(_102)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_135 = dump_var(Move(_55), Move(_81), Move(_33), _136), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: &'static mut f32,mut _2: char,mut _3: char,mut _4: f64,mut _5: f64) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _6: &'static mut (u64,);
let _7: *const u64;
let _8: f64;
let _9: *const *const (u64,);
let _10: &'static mut f64;
let _11: &'static f32;
let _12: (&'static isize, *const (u64,), i32);
let _13: f32;
let _14: f32;
let _15: u32;
let _16: f64;
let _17: f64;
let _18: f64;
let _19: &'static mut &'static mut f32;
let _20: [i64; 6];
let _21: isize;
let _22: i16;
let _23: isize;
let _24: &'static mut *mut i128;
let _25: [i8; 2];
let _26: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _27: *const (u64,);
let _28: &'static mut *const u64;
let _29: bool;
let _30: (u16, u16, &'static f32, char);
let _31: (isize, u8, Adt49);
let _32: *const *const (u64,);
let _33: &'static mut f32;
let _34: i128;
let _35: (&'static f32, *mut i128);
let _36: f64;
let _37: [char; 5];
let _38: char;
let _39: [i16; 8];
let _40: i8;
let _41: *const u8;
let _42: *const u32;
let _43: &'static mut *mut i128;
let _44: [i16; 8];
let _45: i128;
let _46: i8;
let _47: char;
let _48: [u128; 7];
let _49: &'static u32;
let _50: Adt78;
let _51: Adt49;
let _52: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _53: usize;
let _54: *mut [u128; 7];
let _55: i32;
let _56: f32;
let _57: (Adt26, Adt41);
let _58: i64;
let _59: char;
let _60: *mut i128;
let _61: &'static isize;
let _62: [u8; 4];
let _63: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _64: &'static f32;
let _65: (*const u64, (i8, u64, u16), Adt22, isize);
let _66: isize;
let _67: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _68: isize;
let _69: &'static f32;
let _70: f32;
let _71: char;
let _72: &'static mut [char; 6];
let _73: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _74: bool;
let _75: bool;
let _76: *mut u128;
let _77: &'static mut [char; 6];
let _78: isize;
let _79: &'static mut *mut i128;
let _80: &'static isize;
let _81: *const u8;
let _82: *const (u64,);
let _83: f64;
let _84: (*mut &'static mut [char; 6],);
let _85: [u8; 4];
let _86: u16;
let _87: isize;
let _88: [char; 6];
let _89: &'static isize;
let _90: (i64, u64, &'static mut *const u64);
let _91: *mut [u16; 5];
let _92: u8;
let _93: [u16; 5];
let _94: Adt41;
let _95: ();
let _96: ();
{
_3 = _2;
RET = [7181823254947658607_i64,(-2679442424800314772_i64),(-3024947746117344012_i64)];
_3 = _2;
RET = [2034817456189457038_i64,(-7719450411219862200_i64),6273556687077720281_i64];
Goto(bb1)
}
bb1 = {
_4 = 195_u8 as f64;
_2 = _3;
_3 = _2;
_3 = _2;
_2 = _3;
RET = [2050560949539197818_i64,8466804370721045880_i64,2731461789589970955_i64];
Call(_3 = fn3(_4, _4, _5, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [(-1945343413310084901_i64),5114575903385016719_i64,547336960907776992_i64];
_4 = -_5;
_4 = _5 + _5;
_4 = _5;
RET = [4381320460611416770_i64,9088382081473659945_i64,6809983393366137631_i64];
_5 = _4 + _4;
_3 = _2;
_4 = _5 + _5;
_4 = 3381124789_u32 as f64;
_2 = _3;
_2 = _3;
_5 = _4 * _4;
_5 = -_4;
_2 = _3;
_8 = -_4;
_4 = _5 * _5;
_4 = _8 + _5;
RET = [(-586610554803563446_i64),(-3813823055323283613_i64),9154274383270734402_i64];
RET = [(-7966131706367936931_i64),(-5886946657828540377_i64),(-6398941727764623962_i64)];
_2 = _3;
_10 = &mut _5;
(*_10) = _8 - _8;
(*_10) = _8 - _4;
(*_10) = _8 + _8;
(*_10) = _4;
(*_10) = _4 - _8;
(*_10) = _4 * _8;
Goto(bb3)
}
bb3 = {
(*_10) = _4 * _4;
(*_10) = _8 + _8;
_8 = (*_10);
(*_10) = 17_u8 as f64;
(*_10) = _8 * _4;
(*_10) = _4;
(*_10) = _4 * _8;
_14 = (*_10) as f32;
_9 = core::ptr::addr_of!(_12.1);
(*_10) = _8 - _4;
Goto(bb4)
}
bb4 = {
(*_10) = -_8;
_16 = _4 + (*_10);
_15 = 2005250466_u32 ^ 285353831_u32;
_12.2 = -429514284_i32;
_2 = _3;
(*_10) = _16 - _4;
(*_10) = _16 * _16;
_11 = &_14;
(*_10) = _15 as f64;
(*_10) = _8 - _16;
(*_10) = _16 + _8;
(*_10) = _16;
(*_10) = 181_u8 as f64;
(*_10) = -_16;
_17 = (*_10) - (*_10);
_1 = &mut (*_11);
(*_10) = _16 - _17;
_13 = (*_1) * (*_1);
_17 = _4 + (*_10);
_23 = (-9223372036854775808_isize) << _15;
_19 = &mut _1;
_4 = (*_10) + (*_10);
_3 = _2;
_12.0 = &_23;
Goto(bb5)
}
bb5 = {
(*_10) = _4 + _17;
_20 = [(-5008263303934102208_i64),(-6977668491577599445_i64),(-2688203226251731563_i64),(-2318993406368664431_i64),(-807245089453595290_i64),(-4253860678977294051_i64)];
(*_19) = &mut _13;
_22 = (-10978_i16) | (-26684_i16);
_21 = _23;
_15 = 1800386542_u32 * 1653231574_u32;
(*_10) = _16;
(*_10) = _16 - _4;
(*_10) = _8 * _4;
_21 = -_23;
(*_10) = _4 + _17;
(*_10) = 35903_u16 as f64;
(*_10) = (-17709650326550314586158428049558264975_i128) as f64;
_3 = _2;
_18 = _23 as f64;
(*_10) = _22 as f64;
_21 = !_23;
_26.3.1 = !40455_u16;
_26.3.1 = !18821_u16;
_4 = (*_10) + (*_10);
(*_10) = _17 * _4;
_26.3.2 = Move(_11);
(*_10) = _16 * _17;
(*_10) = -_4;
_12.2 = 1476823731_i32 ^ 655819748_i32;
_26.1 = _15 - _15;
(*_10) = _17 + _17;
(*_10) = -_17;
Goto(bb6)
}
bb6 = {
_12.2 = (-297588123_i32) ^ 1213946669_i32;
_2 = _3;
(*_10) = _18;
_26.2 = 115_i8 - (-90_i8);
(*_10) = _4 * _4;
Goto(bb7)
}
bb7 = {
_3 = _2;
_23 = _21 << _22;
(*_10) = _17 - _18;
_18 = (*_10);
_26.1 = _15;
_29 = !false;
_20 = [8922832648747546834_i64,(-6535147556823381736_i64),(-9060043407599134805_i64),(-7989843621702132658_i64),5073528026979422528_i64,7114925766108334538_i64];
_31.2.fld0 = 18281001506320144326_u64 + 7376288614283058880_u64;
_31.1 = _22 as u8;
_31.1 = 128_u8;
(*_10) = -_8;
_33 = Move((*_19));
_26.1 = _15 ^ _15;
(*_10) = _17 + _17;
Call((*_10) = core::intrinsics::fmaf64(_18, _17, _17), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = [(-7610347732743647683_i64),(-1385121869087580879_i64),(-7867602714600921899_i64),(-7293460418528419733_i64),5876023550471686583_i64,(-2933902089448663219_i64)];
_21 = !_23;
(*_10) = -_16;
_21 = _23 << _22;
_16 = (*_10) * (*_10);
_26.3.3 = _3;
_22 = _2 as i16;
_23 = _21;
_31.0 = -_21;
_34 = _31.2.fld0 as i128;
_17 = _15 as f64;
(*_10) = _18 * _18;
_37 = [_26.3.3,_2,_2,_3,_26.3.3];
(*_10) = -_18;
(*_10) = _26.3.1 as f64;
(*_10) = _18 - _16;
(*_10) = _16;
_30.0 = !_26.3.1;
_17 = (*_10);
(*_10) = _17 * _4;
(*_10) = -_16;
_3 = _26.3.3;
_38 = _3;
_35.1 = core::ptr::addr_of_mut!(_34);
_21 = _31.1 as isize;
_4 = _17 - _18;
Call(_26.3.1 = core::intrinsics::bswap(_30.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_10) = -_4;
_18 = _34 as f64;
_4 = -(*_10);
_39 = [_22,_22,_22,_22,_22,_22,_22,_22];
(*_10) = -_18;
Goto(bb10)
}
bb10 = {
(*_10) = _34 as f64;
_20 = [(-983812306359920471_i64),9188249473262209692_i64,718592881808513794_i64,1845399479349592814_i64,(-3813753981454080048_i64),4244322101343419470_i64];
(*_10) = _4;
_17 = (*_10);
_32 = core::ptr::addr_of!((*_9));
_31.2.fld1 = [_31.0,_23,_23,_31.0,_31.0,_31.0];
_12.0 = &_21;
(*_10) = -_4;
_32 = core::ptr::addr_of!((*_9));
(*_10) = _26.1 as f64;
_31.0 = _23;
_24 = &mut _35.1;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = 147994528206312815972569190017603709914_u128 as f64;
(*_10) = _16;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = _4 - _16;
(*_24) = core::ptr::addr_of_mut!(_34);
_21 = _23;
(*_10) = -_16;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
_26.1 = _15;
(*_10) = _17;
match _31.1 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
128 => bb19,
_ => bb18
}
}
bb11 = {
_4 = 195_u8 as f64;
_2 = _3;
_3 = _2;
_3 = _2;
_2 = _3;
RET = [2050560949539197818_i64,8466804370721045880_i64,2731461789589970955_i64];
Call(_3 = fn3(_4, _4, _5, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_20 = [(-7610347732743647683_i64),(-1385121869087580879_i64),(-7867602714600921899_i64),(-7293460418528419733_i64),5876023550471686583_i64,(-2933902089448663219_i64)];
_21 = !_23;
(*_10) = -_16;
_21 = _23 << _22;
_16 = (*_10) * (*_10);
_26.3.3 = _3;
_22 = _2 as i16;
_23 = _21;
_31.0 = -_21;
_34 = _31.2.fld0 as i128;
_17 = _15 as f64;
(*_10) = _18 * _18;
_37 = [_26.3.3,_2,_2,_3,_26.3.3];
(*_10) = -_18;
(*_10) = _26.3.1 as f64;
(*_10) = _18 - _16;
(*_10) = _16;
_30.0 = !_26.3.1;
_17 = (*_10);
(*_10) = _17 * _4;
(*_10) = -_16;
_3 = _26.3.3;
_38 = _3;
_35.1 = core::ptr::addr_of_mut!(_34);
_21 = _31.1 as isize;
_4 = _17 - _18;
Call(_26.3.1 = core::intrinsics::bswap(_30.0), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_3 = _2;
_23 = _21 << _22;
(*_10) = _17 - _18;
_18 = (*_10);
_26.1 = _15;
_29 = !false;
_20 = [8922832648747546834_i64,(-6535147556823381736_i64),(-9060043407599134805_i64),(-7989843621702132658_i64),5073528026979422528_i64,7114925766108334538_i64];
_31.2.fld0 = 18281001506320144326_u64 + 7376288614283058880_u64;
_31.1 = _22 as u8;
_31.1 = 128_u8;
(*_10) = -_8;
_33 = Move((*_19));
_26.1 = _15 ^ _15;
(*_10) = _17 + _17;
Call((*_10) = core::intrinsics::fmaf64(_18, _17, _17), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_12.2 = (-297588123_i32) ^ 1213946669_i32;
_2 = _3;
(*_10) = _18;
_26.2 = 115_i8 - (-90_i8);
(*_10) = _4 * _4;
Goto(bb7)
}
bb15 = {
(*_10) = _4 + _17;
_20 = [(-5008263303934102208_i64),(-6977668491577599445_i64),(-2688203226251731563_i64),(-2318993406368664431_i64),(-807245089453595290_i64),(-4253860678977294051_i64)];
(*_19) = &mut _13;
_22 = (-10978_i16) | (-26684_i16);
_21 = _23;
_15 = 1800386542_u32 * 1653231574_u32;
(*_10) = _16;
(*_10) = _16 - _4;
(*_10) = _8 * _4;
_21 = -_23;
(*_10) = _4 + _17;
(*_10) = 35903_u16 as f64;
(*_10) = (-17709650326550314586158428049558264975_i128) as f64;
_3 = _2;
_18 = _23 as f64;
(*_10) = _22 as f64;
_21 = !_23;
_26.3.1 = !40455_u16;
_26.3.1 = !18821_u16;
_4 = (*_10) + (*_10);
(*_10) = _17 * _4;
_26.3.2 = Move(_11);
(*_10) = _16 * _17;
(*_10) = -_4;
_12.2 = 1476823731_i32 ^ 655819748_i32;
_26.1 = _15 - _15;
(*_10) = _17 + _17;
(*_10) = -_17;
Goto(bb6)
}
bb16 = {
(*_10) = -_8;
_16 = _4 + (*_10);
_15 = 2005250466_u32 ^ 285353831_u32;
_12.2 = -429514284_i32;
_2 = _3;
(*_10) = _16 - _4;
(*_10) = _16 * _16;
_11 = &_14;
(*_10) = _15 as f64;
(*_10) = _8 - _16;
(*_10) = _16 + _8;
(*_10) = _16;
(*_10) = 181_u8 as f64;
(*_10) = -_16;
_17 = (*_10) - (*_10);
_1 = &mut (*_11);
(*_10) = _16 - _17;
_13 = (*_1) * (*_1);
_17 = _4 + (*_10);
_23 = (-9223372036854775808_isize) << _15;
_19 = &mut _1;
_4 = (*_10) + (*_10);
_3 = _2;
_12.0 = &_23;
Goto(bb5)
}
bb17 = {
(*_10) = _4 * _4;
(*_10) = _8 + _8;
_8 = (*_10);
(*_10) = 17_u8 as f64;
(*_10) = _8 * _4;
(*_10) = _4;
(*_10) = _4 * _8;
_14 = (*_10) as f32;
_9 = core::ptr::addr_of!(_12.1);
(*_10) = _8 - _4;
Goto(bb4)
}
bb18 = {
RET = [(-1945343413310084901_i64),5114575903385016719_i64,547336960907776992_i64];
_4 = -_5;
_4 = _5 + _5;
_4 = _5;
RET = [4381320460611416770_i64,9088382081473659945_i64,6809983393366137631_i64];
_5 = _4 + _4;
_3 = _2;
_4 = _5 + _5;
_4 = 3381124789_u32 as f64;
_2 = _3;
_2 = _3;
_5 = _4 * _4;
_5 = -_4;
_2 = _3;
_8 = -_4;
_4 = _5 * _5;
_4 = _8 + _5;
RET = [(-586610554803563446_i64),(-3813823055323283613_i64),9154274383270734402_i64];
RET = [(-7966131706367936931_i64),(-5886946657828540377_i64),(-6398941727764623962_i64)];
_2 = _3;
_10 = &mut _5;
(*_10) = _8 - _8;
(*_10) = _8 - _4;
(*_10) = _8 + _8;
(*_10) = _4;
(*_10) = _4 - _8;
(*_10) = _4 * _8;
Goto(bb3)
}
bb19 = {
(*_24) = core::ptr::addr_of_mut!(_34);
_26.3.1 = !_30.0;
_40 = _26.2;
match _31.1 {
0 => bb17,
1 => bb16,
2 => bb8,
3 => bb10,
4 => bb9,
5 => bb6,
128 => bb20,
_ => bb7
}
}
bb20 = {
_30.3 = _38;
(*_10) = _17 + _16;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = _17 - _17;
(*_10) = -_4;
_7 = core::ptr::addr_of!(_31.2.fld0);
(*_10) = _4 + _16;
_41 = core::ptr::addr_of!(_31.1);
_36 = -(*_10);
(*_24) = core::ptr::addr_of_mut!(_45);
_4 = _36;
(*_24) = core::ptr::addr_of_mut!(_34);
_10 = &mut _17;
(*_10) = (-1023717090133267702_i64) as f64;
(*_7) = 18143878012310123734_u64;
(*_24) = core::ptr::addr_of_mut!(_45);
(*_24) = core::ptr::addr_of_mut!(_34);
_31.0 = _23 ^ _23;
(*_24) = core::ptr::addr_of_mut!(_34);
Goto(bb21)
}
bb21 = {
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
_52.1.3 = -_22;
(*_41) = 1_u8;
_25 = [_26.2,_40];
match (*_7) {
0 => bb5,
18143878012310123734 => bb22,
_ => bb13
}
}
bb22 = {
_2 = _3;
(*_24) = core::ptr::addr_of_mut!(_45);
_30.1 = _26.3.1 << _31.0;
_48 = [301058409598548380369003678729332803070_u128,117721565139976729117485302380399977084_u128,276459291567628159995077007248484517320_u128,193526444368818106952966183657017775669_u128,85986385212616917988384044448033883411_u128,52807815819410101737973987285514176215_u128,199853786231443929697552790833703056675_u128];
_25 = [_26.2,_40];
_10 = &mut _4;
_44 = _39;
(*_10) = _36 - _36;
_36 = (*_10) + (*_10);
_26.0 = &mut _7;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
match _31.1 {
0 => bb19,
2 => bb7,
3 => bb21,
1 => bb23,
_ => bb5
}
}
bb23 = {
(*_41) = _36 as u8;
_52.0.2 = _30.1;
Goto(bb24)
}
bb24 = {
_52.0.2 = _30.1 + _30.1;
_57.0.fld1.0 = _31.2.fld0 * _31.2.fld0;
(*_24) = core::ptr::addr_of_mut!(_45);
_52.0.0 = _34 as i8;
_22 = _52.1.3 ^ _52.1.3;
_52.0.0 = _12.2 as i8;
match _31.2.fld0 {
0 => bb15,
1 => bb14,
2 => bb20,
3 => bb4,
18143878012310123734 => bb25,
_ => bb16
}
}
bb25 = {
_31.2.fld1 = [_31.0,_31.0,_31.0,_31.0,_23,_31.0];
_51.fld0 = _34 as u64;
_52.1.0 = (-2178583344550712598_i64) as f64;
(*_24) = core::ptr::addr_of_mut!(_34);
_26.3.0 = _52.0.2 * _52.0.2;
_12.0 = &_23;
(*_41) = _12.2 as u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = 134_u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = !195_u8;
_57.0.fld5 = _26.3.0 << _26.3.0;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = -_36;
_59 = _30.3;
_56 = _34 as f32;
_63.0.2 = core::ptr::addr_of!(_57.1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_21 = _23 >> _26.3.0;
Goto(bb26)
}
bb26 = {
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_63.0.1 = core::ptr::addr_of!(_57.0.fld1);
_30.1 = _52.0.2 - _52.0.2;
match _31.2.fld0 {
0 => bb8,
1 => bb22,
18143878012310123734 => bb28,
_ => bb27
}
}
bb27 = {
(*_10) = _4 * _4;
(*_10) = _8 + _8;
_8 = (*_10);
(*_10) = 17_u8 as f64;
(*_10) = _8 * _4;
(*_10) = _4;
(*_10) = _4 * _8;
_14 = (*_10) as f32;
_9 = core::ptr::addr_of!(_12.1);
(*_10) = _8 - _4;
Goto(bb4)
}
bb28 = {
_40 = _52.0.0 - _26.2;
(*_41) = 251_u8;
(*_41) = 2_u8 >> _52.0.2;
_64 = &_56;
_52.1 = ((*_10), (*_10), 49741028249246688918541385651690005355_u128, _22);
_42 = core::ptr::addr_of!(_26.1);
_65.1.0 = _57.0.fld5 as i8;
_33 = &mut (*_64);
(*_42) = !_15;
(*_10) = _36 - _52.1.1;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_42) = _15 >> _31.1;
match _52.1.2 {
0 => bb1,
1 => bb16,
2 => bb29,
3 => bb30,
49741028249246688918541385651690005355 => bb32,
_ => bb31
}
}
bb29 = {
(*_10) = -_8;
_16 = _4 + (*_10);
_15 = 2005250466_u32 ^ 285353831_u32;
_12.2 = -429514284_i32;
_2 = _3;
(*_10) = _16 - _4;
(*_10) = _16 * _16;
_11 = &_14;
(*_10) = _15 as f64;
(*_10) = _8 - _16;
(*_10) = _16 + _8;
(*_10) = _16;
(*_10) = 181_u8 as f64;
(*_10) = -_16;
_17 = (*_10) - (*_10);
_1 = &mut (*_11);
(*_10) = _16 - _17;
_13 = (*_1) * (*_1);
_17 = _4 + (*_10);
_23 = (-9223372036854775808_isize) << _15;
_19 = &mut _1;
_4 = (*_10) + (*_10);
_3 = _2;
_12.0 = &_23;
Goto(bb5)
}
bb30 = {
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_63.0.1 = core::ptr::addr_of!(_57.0.fld1);
_30.1 = _52.0.2 - _52.0.2;
match _31.2.fld0 {
0 => bb8,
1 => bb22,
18143878012310123734 => bb28,
_ => bb27
}
}
bb31 = {
_31.2.fld1 = [_31.0,_31.0,_31.0,_31.0,_23,_31.0];
_51.fld0 = _34 as u64;
_52.1.0 = (-2178583344550712598_i64) as f64;
(*_24) = core::ptr::addr_of_mut!(_34);
_26.3.0 = _52.0.2 * _52.0.2;
_12.0 = &_23;
(*_41) = _12.2 as u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = 134_u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = !195_u8;
_57.0.fld5 = _26.3.0 << _26.3.0;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = -_36;
_59 = _30.3;
_56 = _34 as f32;
_63.0.2 = core::ptr::addr_of!(_57.1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_21 = _23 >> _26.3.0;
Goto(bb26)
}
bb32 = {
(*_42) = _15 ^ _15;
(*_24) = core::ptr::addr_of_mut!(_45);
_52.1 = ((*_10), (*_10), 250861932051308683554209012964004773883_u128, _22);
_43 = &mut (*_24);
_52.1.3 = !_22;
(*_42) = _15 * _15;
_27 = core::ptr::addr_of!(_57.0.fld1);
_22 = (*_27).0 as i16;
_30.0 = !_30.1;
_57.0.fld1.0 = _51.fld0;
(*_43) = core::ptr::addr_of_mut!(_34);
(*_9) = core::ptr::addr_of!((*_27));
_12.1 = core::ptr::addr_of!((*_27));
(*_10) = _36 * _52.1.0;
(*_42) = _52.1.2 as u32;
(*_9) = core::ptr::addr_of!((*_27));
_23 = (*_10) as isize;
(*_27) = (_51.fld0,);
_30.2 = &(*_33);
_57.0.fld0 = (*_33) - (*_33);
_31.2.fld1 = [_21,_21,_23,_21,_21,_23];
(*_42) = _15;
_71 = _2;
_62 = [(*_41),(*_41),(*_41),(*_41)];
_30.0 = _57.0.fld5;
Goto(bb33)
}
bb33 = {
_73.1.3 = _34 as i16;
(*_42) = !_15;
_51 = Adt49 { fld0: (*_27).0,fld1: _31.2.fld1 };
(*_42) = _15 * _15;
_65.3 = -_21;
_45 = _34 | _34;
(*_9) = core::ptr::addr_of!((*_27));
(*_27) = (_31.2.fld0,);
(*_27) = (_31.2.fld0,);
(*_27) = (_51.fld0,);
(*_9) = core::ptr::addr_of!((*_27));
_70 = -(*_33);
match _52.1.2 {
0 => bb19,
250861932051308683554209012964004773883 => bb34,
_ => bb17
}
}
bb34 = {
_76 = core::ptr::addr_of_mut!(_52.1.2);
(*_27) = (_51.fld0,);
_65.1 = (_40, (*_27).0, _26.3.0);
(*_41) = 8463947444757422413_i64 as u8;
_25 = [_52.0.0,_40];
_57.0.fld4 = (*_27).0 * (*_27).0;
(*_41) = 223_u8;
(*_43) = core::ptr::addr_of_mut!(_34);
_65.1.1 = (*_27).0;
_57.0.fld2 = [(*_76),(*_76),(*_76),(*_76),_52.1.2,(*_76),(*_76)];
_65.1.0 = _52.0.0;
match (*_76) {
0 => bb26,
1 => bb14,
2 => bb16,
3 => bb35,
4 => bb36,
250861932051308683554209012964004773883 => bb38,
_ => bb37
}
}
bb35 = {
_73.1.3 = _34 as i16;
(*_42) = !_15;
_51 = Adt49 { fld0: (*_27).0,fld1: _31.2.fld1 };
(*_42) = _15 * _15;
_65.3 = -_21;
_45 = _34 | _34;
(*_9) = core::ptr::addr_of!((*_27));
(*_27) = (_31.2.fld0,);
(*_27) = (_31.2.fld0,);
(*_27) = (_51.fld0,);
(*_9) = core::ptr::addr_of!((*_27));
_70 = -(*_33);
match _52.1.2 {
0 => bb19,
250861932051308683554209012964004773883 => bb34,
_ => bb17
}
}
bb36 = {
(*_24) = core::ptr::addr_of_mut!(_34);
(*_24) = core::ptr::addr_of_mut!(_34);
_52.1.3 = -_22;
(*_41) = 1_u8;
_25 = [_26.2,_40];
match (*_7) {
0 => bb5,
18143878012310123734 => bb22,
_ => bb13
}
}
bb37 = {
(*_10) = _4 * _4;
(*_10) = _8 + _8;
_8 = (*_10);
(*_10) = 17_u8 as f64;
(*_10) = _8 * _4;
(*_10) = _4;
(*_10) = _4 * _8;
_14 = (*_10) as f32;
_9 = core::ptr::addr_of!(_12.1);
(*_10) = _8 - _4;
Goto(bb4)
}
bb38 = {
_75 = _29 & _29;
match (*_76) {
0 => bb18,
1 => bb21,
250861932051308683554209012964004773883 => bb40,
_ => bb39
}
}
bb39 = {
_4 = 195_u8 as f64;
_2 = _3;
_3 = _2;
_3 = _2;
_2 = _3;
RET = [2050560949539197818_i64,8466804370721045880_i64,2731461789589970955_i64];
Call(_3 = fn3(_4, _4, _5, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb40 = {
_54 = core::ptr::addr_of_mut!(_48);
(*_27).0 = (*_42) as u64;
_57.0.fld0 = (*_76) as f32;
(*_41) = 189_u8 - 98_u8;
(*_10) = -_52.1.0;
(*_54) = _57.0.fld2;
_57.0.fld1.0 = _57.0.fld4 + _31.2.fld0;
(*_41) = _23 as u8;
(*_27).0 = _57.0.fld4 ^ _57.0.fld4;
_15 = !(*_42);
(*_43) = core::ptr::addr_of_mut!(_34);
_52.0.1 = _12.2 as u64;
_3 = _38;
_19 = &mut _33;
_69 = &_70;
_41 = core::ptr::addr_of!((*_41));
(*_76) = _22 as u128;
_32 = core::ptr::addr_of!((*_9));
_67.0.1 = core::ptr::addr_of!((*_27));
_34 = (*_69) as i128;
(*_27).0 = (-8293519499980235372_i64) as u64;
(*_43) = core::ptr::addr_of_mut!(_34);
(*_42) = _15 + _15;
(*_19) = &mut (*_69);
_67.0.1 = Move((*_9));
(*_41) = 189_u8 << (*_42);
(*_54) = _57.0.fld2;
(*_9) = core::ptr::addr_of!((*_27));
_61 = &_21;
match _31.2.fld0 {
0 => bb14,
1 => bb41,
2 => bb42,
18143878012310123734 => bb44,
_ => bb43
}
}
bb41 = {
_4 = 195_u8 as f64;
_2 = _3;
_3 = _2;
_3 = _2;
_2 = _3;
RET = [2050560949539197818_i64,8466804370721045880_i64,2731461789589970955_i64];
Call(_3 = fn3(_4, _4, _5, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb42 = {
(*_10) = _4 * _4;
(*_10) = _8 + _8;
_8 = (*_10);
(*_10) = 17_u8 as f64;
(*_10) = _8 * _4;
(*_10) = _4;
(*_10) = _4 * _8;
_14 = (*_10) as f32;
_9 = core::ptr::addr_of!(_12.1);
(*_10) = _8 - _4;
Goto(bb4)
}
bb43 = {
_31.2.fld1 = [_31.0,_31.0,_31.0,_31.0,_23,_31.0];
_51.fld0 = _34 as u64;
_52.1.0 = (-2178583344550712598_i64) as f64;
(*_24) = core::ptr::addr_of_mut!(_34);
_26.3.0 = _52.0.2 * _52.0.2;
_12.0 = &_23;
(*_41) = _12.2 as u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = 134_u8;
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_41) = !195_u8;
_57.0.fld5 = _26.3.0 << _26.3.0;
(*_24) = core::ptr::addr_of_mut!(_34);
(*_10) = -_36;
_59 = _30.3;
_56 = _34 as f32;
_63.0.2 = core::ptr::addr_of!(_57.1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_21 = _23 >> _26.3.0;
Goto(bb26)
}
bb44 = {
_51.fld1 = _31.2.fld1;
(*_54) = [(*_76),(*_76),(*_76),(*_76),(*_76),(*_76),(*_76)];
(*_9) = core::ptr::addr_of!((*_27));
(*_27) = (_57.0.fld4,);
(*_19) = &mut _57.0.fld0;
_18 = (*_10);
(*_10) = _36 - _52.1.0;
(*_43) = core::ptr::addr_of_mut!(_45);
_2 = _59;
(*_41) = 27_u8 - 63_u8;
_47 = _2;
_81 = core::ptr::addr_of!((*_41));
_3 = _26.3.3;
_65.1.1 = 2650932331310353700_i64 as u64;
(*_42) = !_15;
_9 = core::ptr::addr_of!((*_9));
match _31.2.fld0 {
0 => bb38,
18143878012310123734 => bb45,
_ => bb5
}
}
bb45 = {
(*_9) = Move(_63.0.1);
_11 = Move(_30.2);
(*_81) = 97_u8 & 237_u8;
(*_43) = core::ptr::addr_of_mut!(_45);
(*_27).0 = _30.3 as u64;
(*_42) = _15 + _15;
_39 = _44;
(*_41) = 20_u8 - 69_u8;
(*_54) = [(*_76),(*_76),(*_76),(*_76),(*_76),(*_76),(*_76)];
Goto(bb46)
}
bb46 = {
_12.0 = Move(_61);
_52.0.0 = (*_41) as i8;
(*_54) = [(*_76),(*_76),(*_76),(*_76),(*_76),(*_76),(*_76)];
(*_76) = 128889158838687269791995621447671723564_u128 | 193043875142155360515718040749821551595_u128;
(*_41) = _23 as u8;
(*_10) = -_52.1.1;
_73.0.1 = (*_27).0 & (*_27).0;
(*_27).0 = !_51.fld0;
(*_43) = core::ptr::addr_of_mut!(_34);
_26.3.0 = _29 as u16;
(*_41) = (-6232803299604031972_i64) as u8;
_76 = core::ptr::addr_of_mut!((*_76));
_1 = Move((*_19));
_47 = _38;
match _31.2.fld0 {
0 => bb9,
1 => bb29,
2 => bb3,
3 => bb47,
18143878012310123734 => bb49,
_ => bb48
}
}
bb47 = {
_73.1.3 = _34 as i16;
(*_42) = !_15;
_51 = Adt49 { fld0: (*_27).0,fld1: _31.2.fld1 };
(*_42) = _15 * _15;
_65.3 = -_21;
_45 = _34 | _34;
(*_9) = core::ptr::addr_of!((*_27));
(*_27) = (_31.2.fld0,);
(*_27) = (_31.2.fld0,);
(*_27) = (_51.fld0,);
(*_9) = core::ptr::addr_of!((*_27));
_70 = -(*_33);
match _52.1.2 {
0 => bb19,
250861932051308683554209012964004773883 => bb34,
_ => bb17
}
}
bb48 = {
(*_9) = core::ptr::addr_of!(_57.0.fld1);
(*_9) = core::ptr::addr_of!(_57.0.fld1);
_63.0.1 = core::ptr::addr_of!(_57.0.fld1);
_30.1 = _52.0.2 - _52.0.2;
match _31.2.fld0 {
0 => bb8,
1 => bb22,
18143878012310123734 => bb28,
_ => bb27
}
}
bb49 = {
(*_43) = core::ptr::addr_of_mut!(_34);
(*_27) = (_52.0.1,);
_49 = &(*_42);
_78 = (*_41) as isize;
(*_42) = _12.2 as u32;
_73.1.1 = (*_10) - (*_10);
_73.1.0 = (*_10) + (*_10);
_42 = core::ptr::addr_of!((*_42));
(*_42) = !_15;
(*_27) = (_31.2.fld0,);
(*_42) = (*_76) as u32;
_12.1 = core::ptr::addr_of!((*_27));
(*_27).0 = _52.0.1 & _65.1.1;
Goto(bb50)
}
bb50 = {
Call(_95 = dump_var(Move(_15), Move(_22), Move(_23), Move(_2)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_95 = dump_var(Move(_25), Move(_20), Move(_21), Move(_44)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_95 = dump_var(Move(_75), Move(_45), Move(_37), _96), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64) -> char {
mir! {
type RET = char;
let _5: bool;
let _6: char;
let _7: Adt22;
let _8: *mut &'static mut [char; 6];
let _9: f64;
let _10: i8;
let _11: i32;
let _12: Adt26;
let _13: isize;
let _14: *const &'static mut f32;
let _15: f32;
let _16: i64;
let _17: *mut *const &'static mut f32;
let _18: bool;
let _19: i32;
let _20: f32;
let _21: &'static mut f64;
let _22: u128;
let _23: isize;
let _24: i128;
let _25: u8;
let _26: *const &'static mut f32;
let _27: *mut [u16; 5];
let _28: *mut i16;
let _29: (Adt26, Adt41);
let _30: Adt22;
let _31: i16;
let _32: u16;
let _33: (i64, u64, &'static mut *const u64);
let _34: &'static &'static mut &'static mut f32;
let _35: &'static f32;
let _36: *const (u64,);
let _37: *const u8;
let _38: &'static mut *mut i128;
let _39: *mut &'static mut [char; 6];
let _40: isize;
let _41: f32;
let _42: i128;
let _43: &'static mut (u64,);
let _44: isize;
let _45: (*mut &'static mut [char; 6],);
let _46: char;
let _47: f32;
let _48: [u8; 4];
let _49: f32;
let _50: [i64; 3];
let _51: &'static isize;
let _52: Adt75;
let _53: i64;
let _54: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _55: usize;
let _56: &'static mut *mut i128;
let _57: (f64, f64, u128, i16);
let _58: bool;
let _59: &'static &'static mut &'static mut f32;
let _60: Adt49;
let _61: *mut i128;
let _62: Adt80;
let _63: *mut i16;
let _64: char;
let _65: Adt53;
let _66: isize;
let _67: &'static u32;
let _68: *mut u128;
let _69: (*mut &'static mut [char; 6],);
let _70: f32;
let _71: usize;
let _72: isize;
let _73: bool;
let _74: isize;
let _75: Adt22;
let _76: u8;
let _77: &'static mut (u64,);
let _78: &'static &'static mut &'static mut f32;
let _79: [char; 6];
let _80: isize;
let _81: f32;
let _82: u8;
let _83: i128;
let _84: i32;
let _85: &'static u32;
let _86: (*mut &'static mut [char; 6],);
let _87: (u16, u16, &'static f32, char);
let _88: f64;
let _89: isize;
let _90: u64;
let _91: *const u64;
let _92: *const u32;
let _93: isize;
let _94: u128;
let _95: isize;
let _96: (Adt26, Adt41);
let _97: u128;
let _98: *const &'static mut f32;
let _99: *mut *const &'static mut f32;
let _100: u64;
let _101: ();
let _102: ();
{
_3 = -_2;
RET = '\u{32c09}';
_1 = _2 - _2;
_4 = 4012_u16 as f64;
_3 = _1 - _4;
Goto(bb1)
}
bb1 = {
_3 = _2 * _1;
_5 = _3 > _1;
_1 = _3 * _3;
_5 = _1 < _3;
_2 = _3 + _1;
RET = '\u{8e2e5}';
_2 = _1;
_1 = _3 + _3;
_4 = -_2;
RET = '\u{f91df}';
_1 = _2 + _4;
_1 = _4 - _4;
_1 = _4;
_6 = RET;
_3 = -_2;
_6 = RET;
RET = _6;
_3 = 2866329642050573786054534559237735223_i128 as f64;
_3 = -_1;
RET = _6;
RET = _6;
_9 = _1;
_5 = _2 < _2;
Goto(bb2)
}
bb2 = {
_3 = _2 + _1;
RET = _6;
_9 = _3;
_6 = RET;
_11 = 50291949445632859499181396037369520099_i128 as i32;
_6 = RET;
_10 = (-27_i8) ^ 126_i8;
Goto(bb3)
}
bb3 = {
_6 = RET;
_12.fld4 = 18238071681755477265_u64;
_10 = (-127_i8);
_12.fld0 = _10 as f32;
_10 = (-125_i8);
_1 = _9;
_6 = RET;
_12.fld5 = 10313_u16 + 17425_u16;
_9 = _1 - _1;
_1 = -_3;
_6 = RET;
_1 = -_9;
_12.fld3 = _3 + _1;
_12.fld2 = [242531303485513844275627323977385609140_u128,5674026579105628365314256314850555440_u128,134023103066403312132316077940743537183_u128,49196800216135834024111758659931296551_u128,106185567804729595640711664777047188008_u128,1329480148701767072961769583960468212_u128,274143612159304970498504111656329953967_u128];
_1 = _9 * _12.fld3;
_9 = _2 - _1;
_12.fld5 = !288_u16;
_18 = !_5;
_17 = core::ptr::addr_of_mut!(_14);
_12.fld1 = (_12.fld4,);
_6 = RET;
_19 = -_11;
_22 = 82016494886002683185116364875920823909_u128 << _12.fld1.0;
match _12.fld1.0 {
0 => bb1,
18238071681755477265 => bb5,
_ => bb4
}
}
bb4 = {
_3 = _2 * _1;
_5 = _3 > _1;
_1 = _3 * _3;
_5 = _1 < _3;
_2 = _3 + _1;
RET = '\u{8e2e5}';
_2 = _1;
_1 = _3 + _3;
_4 = -_2;
RET = '\u{f91df}';
_1 = _2 + _4;
_1 = _4 - _4;
_1 = _4;
_6 = RET;
_3 = -_2;
_6 = RET;
RET = _6;
_3 = 2866329642050573786054534559237735223_i128 as f64;
_3 = -_1;
RET = _6;
RET = _6;
_9 = _1;
_5 = _2 < _2;
Goto(bb2)
}
bb5 = {
_17 = core::ptr::addr_of_mut!((*_17));
_9 = _12.fld3 + _4;
_19 = _12.fld1.0 as i32;
_3 = _12.fld3;
_3 = -_1;
_15 = _12.fld0;
_19 = _11 << _12.fld1.0;
_20 = _12.fld0;
_4 = _1;
RET = _6;
_1 = _4 - _3;
_13 = 9223372036854775807_isize & 63_isize;
_12.fld2 = [_22,_22,_22,_22,_22,_22,_22];
_13 = (-30_isize) | 9223372036854775807_isize;
_23 = !_13;
_12.fld4 = _20 as u64;
_21 = &mut _9;
_7 = Adt22::Variant0 { fld0: _18,fld1: (*_21),fld2: _23,fld3: _20,fld4: 6_usize,fld5: _12.fld5,fld6: _12.fld1,fld7: _22 };
place!(Field::<f64>(Variant(_7, 0), 1)) = 123_u8 as f64;
_1 = (*_21) + (*_21);
_21 = &mut _12.fld3;
place!(Field::<(u64,)>(Variant(_7, 0), 6)) = (11551581222515242639_u64,);
place!(Field::<isize>(Variant(_7, 0), 2)) = _23 + _13;
_16 = 507010454204674600_i64 + 7306713009680382245_i64;
_20 = _15 + _15;
Goto(bb6)
}
bb6 = {
_16 = -(-123116169513104834_i64);
(*_21) = 1402940242_u32 as f64;
_29.0.fld5 = Field::<u16>(Variant(_7, 0), 5) + Field::<u16>(Variant(_7, 0), 5);
RET = _6;
(*_21) = 2885458137_u32 as f64;
_22 = Field::<u128>(Variant(_7, 0), 7) + Field::<u128>(Variant(_7, 0), 7);
_1 = -_4;
_10 = _11 as i8;
place!(Field::<(u64,)>(Variant(_7, 0), 6)) = (8846578728097700378_u64,);
(*_21) = _4 * _4;
_21 = &mut place!(Field::<f64>(Variant(_7, 0), 1));
_25 = !208_u8;
_29.0.fld0 = _16 as f32;
(*_21) = -_4;
(*_21) = _29.0.fld5 as f64;
(*_21) = _3;
(*_21) = _3;
(*_21) = _4 + _1;
_19 = _5 as i32;
_15 = 7570827840361642105_u64 as f32;
_29.0.fld4 = 14865692846191880835_u64 + 8103759665565870922_u64;
_18 = !_5;
_29.0.fld1 = (_29.0.fld4,);
_31 = _23 as i16;
Goto(bb7)
}
bb7 = {
(*_21) = _1 + _4;
(*_21) = _1;
_23 = _13 - _13;
_2 = _29.0.fld5 as f64;
_6 = RET;
_29.0.fld6 = core::ptr::addr_of!(_25);
(*_21) = _1 * _3;
_22 = !219665060667117407587146422531141664038_u128;
_5 = !_18;
_33.0 = _16 & _16;
_3 = (*_21) + (*_21);
(*_21) = _4 - _3;
(*_21) = _4 * _4;
_33.1 = !_29.0.fld4;
_18 = _5 & _5;
_29.0.fld5 = 45722_u16;
(*_21) = _4 * _3;
_2 = (*_21);
(*_21) = _3 + _4;
_11 = _19 | _19;
_29.0.fld2 = [_22,_22,_22,_22,_22,_22,_22];
(*_21) = _2 - _1;
Call((*_21) = fn4(_13, Move(_17)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _18 ^ _18;
_31 = !(-22885_i16);
_10 = (-76_i8);
_29.0.fld5 = 16861_u16;
(*_21) = _4 * _3;
_29.0.fld4 = !_29.0.fld1.0;
_16 = _33.0 * _33.0;
_29.0.fld1 = (_29.0.fld4,);
_29.0.fld4 = _29.0.fld1.0;
_2 = (*_21) * (*_21);
(*_21) = _3;
_35 = &_20;
_29.0.fld3 = (*_21);
_10 = (-81_i8);
_29.0.fld5 = 9523_u16 << _23;
(*_21) = -_4;
_21 = &mut _29.0.fld3;
_25 = 133_u8;
_35 = &_15;
(*_21) = _1;
match _10 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211375 => bb9,
_ => bb3
}
}
bb9 = {
_10 = 9_i8 >> _19;
Goto(bb10)
}
bb10 = {
_20 = -(*_35);
(*_21) = -_3;
_24 = (-123558175940185274190393435771864508057_i128) << _33.0;
(*_21) = -_3;
(*_21) = _3 * _3;
_40 = !_13;
_1 = (*_21);
_25 = !150_u8;
(*_21) = _1;
_5 = (*_21) > _3;
_19 = _22 as i32;
_37 = core::ptr::addr_of!(_25);
_17 = core::ptr::addr_of_mut!(_14);
(*_37) = 127_u8 + 194_u8;
_42 = 25239_u16 as i128;
_2 = -(*_21);
_3 = (*_21) + (*_21);
(*_21) = _4;
RET = _6;
(*_21) = _3 - _2;
_32 = !24352_u16;
(*_37) = 9_u8 << _11;
Goto(bb11)
}
bb11 = {
(*_37) = 67_u8 + 155_u8;
(*_37) = 76_u8 + 110_u8;
_17 = core::ptr::addr_of_mut!((*_17));
(*_21) = _1;
Goto(bb12)
}
bb12 = {
(*_37) = _32 as u8;
(*_37) = 198_u8 - 221_u8;
(*_21) = -_2;
_44 = _13;
(*_21) = -_1;
(*_37) = 118_u8 | 11_u8;
(*_37) = 17_u8 + 28_u8;
(*_37) = 164_u8 ^ 137_u8;
(*_37) = _10 as u8;
(*_37) = 62_u8 ^ 166_u8;
_1 = (*_21) + (*_21);
_28 = core::ptr::addr_of_mut!(_31);
_11 = _19 ^ _19;
_1 = _40 as f64;
(*_21) = _2 * _2;
(*_37) = 219_u8 << _10;
(*_37) = 178_u8 + 132_u8;
_31 = (-24722_i16);
match (*_28) {
0 => bb1,
1 => bb8,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607431768186734 => bb14,
_ => bb13
}
}
bb13 = {
(*_37) = 67_u8 + 155_u8;
(*_37) = 76_u8 + 110_u8;
_17 = core::ptr::addr_of_mut!((*_17));
(*_21) = _1;
Goto(bb12)
}
bb14 = {
_2 = (*_21) * (*_21);
_21 = &mut _1;
_33.1 = (*_35) as u64;
(*_37) = 129_u8 - 185_u8;
(*_28) = !30135_i16;
_48 = [(*_37),(*_37),(*_37),(*_37)];
(*_37) = 239_u8 << _11;
_47 = (*_35);
_46 = RET;
Call((*_37) = core::intrinsics::transmute(_18), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_21) = _4 + _4;
_21 = &mut _2;
(*_21) = _3 * _3;
_48 = [(*_37),(*_37),(*_37),(*_37)];
(*_28) = (-19162_i16) | (-17311_i16);
(*_37) = 129_u8 >> (*_28);
(*_37) = !11_u8;
_16 = -_33.0;
(*_37) = 2_u8 << (*_28);
_20 = (*_35);
_47 = (*_28) as f32;
(*_37) = !219_u8;
(*_28) = (-7584_i16) & 24271_i16;
(*_28) = _11 as i16;
_16 = _33.0 * _33.0;
(*_37) = !193_u8;
_50 = [_16,_16,_33.0];
(*_37) = 14_u8 + 60_u8;
(*_28) = (-7732_i16);
(*_21) = _3;
match (*_28) {
340282366920938463463374607431768203724 => bb17,
_ => bb16
}
}
bb16 = {
_5 = _18 ^ _18;
_31 = !(-22885_i16);
_10 = (-76_i8);
_29.0.fld5 = 16861_u16;
(*_21) = _4 * _3;
_29.0.fld4 = !_29.0.fld1.0;
_16 = _33.0 * _33.0;
_29.0.fld1 = (_29.0.fld4,);
_29.0.fld4 = _29.0.fld1.0;
_2 = (*_21) * (*_21);
(*_21) = _3;
_35 = &_20;
_29.0.fld3 = (*_21);
_10 = (-81_i8);
_29.0.fld5 = 9523_u16 << _23;
(*_21) = -_4;
_21 = &mut _29.0.fld3;
_25 = 133_u8;
_35 = &_15;
(*_21) = _1;
match _10 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211375 => bb9,
_ => bb3
}
}
bb17 = {
_20 = _15 * (*_35);
(*_21) = _3;
(*_37) = 99_u8 * 67_u8;
(*_21) = _4 - _3;
_21 = &mut _4;
(*_21) = -_3;
RET = _46;
(*_37) = 172_u8 | 65_u8;
_47 = -(*_35);
(*_21) = _3;
_57.0 = -(*_21);
(*_28) = 15909_i16;
(*_21) = -_3;
(*_21) = _3 + _3;
_42 = -_24;
_3 = (*_21) + (*_21);
_46 = _6;
(*_28) = _10 as i16;
Call((*_37) = core::intrinsics::transmute(_5), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_54.3.0 = !_32;
(*_37) = 201_u8 * 244_u8;
Goto(bb19)
}
bb19 = {
(*_37) = 80_u8;
Goto(bb20)
}
bb20 = {
(*_21) = _3;
(*_28) = !21969_i16;
_54.1 = 3121131412_u32;
_13 = _44 >> (*_28);
(*_28) = (-4068_i16);
_22 = 201011075132178355769552366275483513237_u128 & 9014769792595613893678629820859947237_u128;
_31 = (-17470_i16) - 15166_i16;
match (*_37) {
0 => bb18,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
80 => bb27,
_ => bb26
}
}
bb21 = {
_3 = _2 * _1;
_5 = _3 > _1;
_1 = _3 * _3;
_5 = _1 < _3;
_2 = _3 + _1;
RET = '\u{8e2e5}';
_2 = _1;
_1 = _3 + _3;
_4 = -_2;
RET = '\u{f91df}';
_1 = _2 + _4;
_1 = _4 - _4;
_1 = _4;
_6 = RET;
_3 = -_2;
_6 = RET;
RET = _6;
_3 = 2866329642050573786054534559237735223_i128 as f64;
_3 = -_1;
RET = _6;
RET = _6;
_9 = _1;
_5 = _2 < _2;
Goto(bb2)
}
bb22 = {
_54.3.0 = !_32;
(*_37) = 201_u8 * 244_u8;
Goto(bb19)
}
bb23 = {
(*_37) = _32 as u8;
(*_37) = 198_u8 - 221_u8;
(*_21) = -_2;
_44 = _13;
(*_21) = -_1;
(*_37) = 118_u8 | 11_u8;
(*_37) = 17_u8 + 28_u8;
(*_37) = 164_u8 ^ 137_u8;
(*_37) = _10 as u8;
(*_37) = 62_u8 ^ 166_u8;
_1 = (*_21) + (*_21);
_28 = core::ptr::addr_of_mut!(_31);
_11 = _19 ^ _19;
_1 = _40 as f64;
(*_21) = _2 * _2;
(*_37) = 219_u8 << _10;
(*_37) = 178_u8 + 132_u8;
_31 = (-24722_i16);
match (*_28) {
0 => bb1,
1 => bb8,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607431768186734 => bb14,
_ => bb13
}
}
bb24 = {
_5 = _18 ^ _18;
_31 = !(-22885_i16);
_10 = (-76_i8);
_29.0.fld5 = 16861_u16;
(*_21) = _4 * _3;
_29.0.fld4 = !_29.0.fld1.0;
_16 = _33.0 * _33.0;
_29.0.fld1 = (_29.0.fld4,);
_29.0.fld4 = _29.0.fld1.0;
_2 = (*_21) * (*_21);
(*_21) = _3;
_35 = &_20;
_29.0.fld3 = (*_21);
_10 = (-81_i8);
_29.0.fld5 = 9523_u16 << _23;
(*_21) = -_4;
_21 = &mut _29.0.fld3;
_25 = 133_u8;
_35 = &_15;
(*_21) = _1;
match _10 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211375 => bb9,
_ => bb3
}
}
bb25 = {
_16 = -(-123116169513104834_i64);
(*_21) = 1402940242_u32 as f64;
_29.0.fld5 = Field::<u16>(Variant(_7, 0), 5) + Field::<u16>(Variant(_7, 0), 5);
RET = _6;
(*_21) = 2885458137_u32 as f64;
_22 = Field::<u128>(Variant(_7, 0), 7) + Field::<u128>(Variant(_7, 0), 7);
_1 = -_4;
_10 = _11 as i8;
place!(Field::<(u64,)>(Variant(_7, 0), 6)) = (8846578728097700378_u64,);
(*_21) = _4 * _4;
_21 = &mut place!(Field::<f64>(Variant(_7, 0), 1));
_25 = !208_u8;
_29.0.fld0 = _16 as f32;
(*_21) = -_4;
(*_21) = _29.0.fld5 as f64;
(*_21) = _3;
(*_21) = _3;
(*_21) = _4 + _1;
_19 = _5 as i32;
_15 = 7570827840361642105_u64 as f32;
_29.0.fld4 = 14865692846191880835_u64 + 8103759665565870922_u64;
_18 = !_5;
_29.0.fld1 = (_29.0.fld4,);
_31 = _23 as i16;
Goto(bb7)
}
bb26 = {
_20 = -(*_35);
(*_21) = -_3;
_24 = (-123558175940185274190393435771864508057_i128) << _33.0;
(*_21) = -_3;
(*_21) = _3 * _3;
_40 = !_13;
_1 = (*_21);
_25 = !150_u8;
(*_21) = _1;
_5 = (*_21) > _3;
_19 = _22 as i32;
_37 = core::ptr::addr_of!(_25);
_17 = core::ptr::addr_of_mut!(_14);
(*_37) = 127_u8 + 194_u8;
_42 = 25239_u16 as i128;
_2 = -(*_21);
_3 = (*_21) + (*_21);
(*_21) = _4;
RET = _6;
(*_21) = _3 - _2;
_32 = !24352_u16;
(*_37) = 9_u8 << _11;
Goto(bb11)
}
bb27 = {
_54.3.0 = _32 << (*_28);
_17 = core::ptr::addr_of_mut!((*_17));
_53 = -_33.0;
_54.3.2 = Move(_35);
(*_28) = _54.3.0 as i16;
_25 = 58_u8 + 34_u8;
(*_37) = _42 as u8;
_19 = _11;
_15 = -_20;
(*_21) = _54.1 as f64;
(*_28) = (-20757_i16) | 6141_i16;
(*_37) = 169_u8 << (*_28);
(*_21) = -_57.0;
(*_21) = _57.0 * _3;
(*_21) = (*_37) as f64;
_57.1 = _54.3.0 as f64;
(*_28) = !27103_i16;
(*_21) = -_3;
_58 = _5;
_57 = ((*_21), (*_21), _22, (*_28));
(*_28) = _57.3 * _57.3;
match _54.1 {
0 => bb21,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb26,
5 => bb14,
6 => bb17,
3121131412 => bb29,
_ => bb28
}
}
bb28 = {
_20 = _15 * (*_35);
(*_21) = _3;
(*_37) = 99_u8 * 67_u8;
(*_21) = _4 - _3;
_21 = &mut _4;
(*_21) = -_3;
RET = _46;
(*_37) = 172_u8 | 65_u8;
_47 = -(*_35);
(*_21) = _3;
_57.0 = -(*_21);
(*_28) = 15909_i16;
(*_21) = -_3;
(*_21) = _3 + _3;
_42 = -_24;
_3 = (*_21) + (*_21);
_46 = _6;
(*_28) = _10 as i16;
Call((*_37) = core::intrinsics::transmute(_5), ReturnTo(bb18), UnwindUnreachable())
}
bb29 = {
_54.2 = _54.1 as i8;
(*_21) = -_57.1;
_16 = _53;
_65.fld1 = (*_21);
_64 = RET;
(*_21) = _3 - _65.fld1;
_57.2 = _22;
Goto(bb30)
}
bb30 = {
(*_21) = _57.1 - _57.0;
_66 = _40 & _23;
match _54.1 {
0 => bb6,
1 => bb14,
2 => bb27,
3 => bb31,
4 => bb32,
3121131412 => bb34,
_ => bb33
}
}
bb31 = {
_3 = _2 + _1;
RET = _6;
_9 = _3;
_6 = RET;
_11 = 50291949445632859499181396037369520099_i128 as i32;
_6 = RET;
_10 = (-27_i8) ^ 126_i8;
Goto(bb3)
}
bb32 = {
_5 = _18 ^ _18;
_31 = !(-22885_i16);
_10 = (-76_i8);
_29.0.fld5 = 16861_u16;
(*_21) = _4 * _3;
_29.0.fld4 = !_29.0.fld1.0;
_16 = _33.0 * _33.0;
_29.0.fld1 = (_29.0.fld4,);
_29.0.fld4 = _29.0.fld1.0;
_2 = (*_21) * (*_21);
(*_21) = _3;
_35 = &_20;
_29.0.fld3 = (*_21);
_10 = (-81_i8);
_29.0.fld5 = 9523_u16 << _23;
(*_21) = -_4;
_21 = &mut _29.0.fld3;
_25 = 133_u8;
_35 = &_15;
(*_21) = _1;
match _10 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211375 => bb9,
_ => bb3
}
}
bb33 = {
_10 = 9_i8 >> _19;
Goto(bb10)
}
bb34 = {
(*_21) = _57.0;
(*_28) = _57.3 ^ _57.3;
(*_28) = _57.3;
(*_28) = 10645946461978859117_usize as i16;
_54.3.2 = &_20;
_48 = [(*_37),(*_37),(*_37),(*_37)];
_54.3.2 = &_47;
(*_37) = !150_u8;
_54.3.2 = &_15;
(*_28) = !_57.3;
_63 = core::ptr::addr_of_mut!((*_28));
(*_28) = _57.3 + _57.3;
_60.fld1 = [_66,_66,_23,_13,_40,_40];
(*_28) = (*_37) as i16;
(*_21) = _3;
_35 = &_47;
RET = _46;
(*_28) = _33.0 as i16;
_51 = &_66;
_54.3.2 = &(*_35);
_5 = _58;
(*_37) = 223_u8;
_61 = core::ptr::addr_of_mut!(_24);
(*_21) = (*_35) as f64;
_68 = core::ptr::addr_of_mut!(_22);
(*_21) = _57.1;
_54.3.3 = RET;
(*_61) = (*_21) as i128;
match (*_37) {
0 => bb1,
1 => bb24,
2 => bb22,
3 => bb27,
223 => bb35,
_ => bb33
}
}
bb35 = {
_70 = (*_35) + (*_35);
(*_61) = _42;
_13 = -(*_51);
(*_61) = _42 ^ _42;
(*_21) = -_65.fld1;
_6 = RET;
(*_68) = _57.2 - _57.2;
(*_21) = -_57.0;
_49 = 15378793525631632888_usize as f32;
(*_61) = _19 as i128;
_71 = 2_usize << (*_51);
(*_37) = !142_u8;
match _54.1 {
0 => bb15,
1 => bb28,
2 => bb11,
3121131412 => bb37,
_ => bb36
}
}
bb36 = {
_20 = _15 * (*_35);
(*_21) = _3;
(*_37) = 99_u8 * 67_u8;
(*_21) = _4 - _3;
_21 = &mut _4;
(*_21) = -_3;
RET = _46;
(*_37) = 172_u8 | 65_u8;
_47 = -(*_35);
(*_21) = _3;
_57.0 = -(*_21);
(*_28) = 15909_i16;
(*_21) = -_3;
(*_21) = _3 + _3;
_42 = -_24;
_3 = (*_21) + (*_21);
_46 = _6;
(*_28) = _10 as i16;
Call((*_37) = core::intrinsics::transmute(_5), ReturnTo(bb18), UnwindUnreachable())
}
bb37 = {
_57 = ((*_21), (*_21), (*_68), (*_28));
(*_68) = _57.2 ^ _57.2;
_73 = !_5;
(*_61) = -_42;
(*_21) = _24 as f64;
_55 = !_71;
(*_37) = !0_u8;
(*_61) = _42 | _42;
(*_68) = !_57.2;
_57.1 = _3;
(*_21) = _3 + _57.0;
(*_21) = _10 as f64;
(*_28) = _57.3 + _57.3;
(*_21) = _57.1;
_67 = &_54.1;
(*_61) = _42;
(*_28) = _57.3 - _57.3;
_37 = core::ptr::addr_of!((*_37));
_33.1 = 12568508742349239596_u64 | 6586478102964958762_u64;
_38 = &mut _61;
Goto(bb38)
}
bb38 = {
(*_37) = (*_28) as u8;
Goto(bb39)
}
bb39 = {
_54.3.0 = _32;
_41 = _49 + (*_35);
(*_68) = !_57.2;
(*_38) = core::ptr::addr_of_mut!(_42);
(*_68) = _57.2 | _57.2;
(*_38) = core::ptr::addr_of_mut!(_24);
(*_28) = -_57.3;
_16 = _53 >> (*_68);
(*_37) = 184_u8 | 11_u8;
_54.3.3 = RET;
(*_21) = _57.0 * _57.0;
(*_68) = _57.2;
(*_38) = core::ptr::addr_of_mut!(_42);
(*_28) = _57.3 - _57.3;
_64 = _6;
(*_37) = 6_u8 >> (*_51);
Call((*_21) = core::intrinsics::transmute((*_51)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
(*_38) = core::ptr::addr_of_mut!(_42);
(*_28) = _64 as i16;
(*_68) = _57.2 | _57.2;
(*_68) = _57.1 as u128;
_68 = core::ptr::addr_of_mut!((*_68));
_22 = _57.2 << (*_67);
_19 = _33.1 as i32;
(*_28) = _57.3;
_33.0 = _53 | _16;
_33.0 = _73 as i64;
RET = _46;
(*_37) = 71_u8;
(*_28) = _57.3 & _57.3;
(*_28) = _33.0 as i16;
_74 = _23 + (*_51);
_47 = _55 as f32;
(*_68) = _58 as u128;
match (*_67) {
0 => bb5,
1 => bb15,
2 => bb14,
3 => bb19,
4 => bb41,
5 => bb42,
6 => bb43,
3121131412 => bb45,
_ => bb44
}
}
bb41 = {
(*_21) = _3;
(*_28) = !21969_i16;
_54.1 = 3121131412_u32;
_13 = _44 >> (*_28);
(*_28) = (-4068_i16);
_22 = 201011075132178355769552366275483513237_u128 & 9014769792595613893678629820859947237_u128;
_31 = (-17470_i16) - 15166_i16;
match (*_37) {
0 => bb18,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
80 => bb27,
_ => bb26
}
}
bb42 = {
(*_21) = _4 + _4;
_21 = &mut _2;
(*_21) = _3 * _3;
_48 = [(*_37),(*_37),(*_37),(*_37)];
(*_28) = (-19162_i16) | (-17311_i16);
(*_37) = 129_u8 >> (*_28);
(*_37) = !11_u8;
_16 = -_33.0;
(*_37) = 2_u8 << (*_28);
_20 = (*_35);
_47 = (*_28) as f32;
(*_37) = !219_u8;
(*_28) = (-7584_i16) & 24271_i16;
(*_28) = _11 as i16;
_16 = _33.0 * _33.0;
(*_37) = !193_u8;
_50 = [_16,_16,_33.0];
(*_37) = 14_u8 + 60_u8;
(*_28) = (-7732_i16);
(*_21) = _3;
match (*_28) {
340282366920938463463374607431768203724 => bb17,
_ => bb16
}
}
bb43 = {
_20 = _15 * (*_35);
(*_21) = _3;
(*_37) = 99_u8 * 67_u8;
(*_21) = _4 - _3;
_21 = &mut _4;
(*_21) = -_3;
RET = _46;
(*_37) = 172_u8 | 65_u8;
_47 = -(*_35);
(*_21) = _3;
_57.0 = -(*_21);
(*_28) = 15909_i16;
(*_21) = -_3;
(*_21) = _3 + _3;
_42 = -_24;
_3 = (*_21) + (*_21);
_46 = _6;
(*_28) = _10 as i16;
Call((*_37) = core::intrinsics::transmute(_5), ReturnTo(bb18), UnwindUnreachable())
}
bb44 = {
_5 = _18 ^ _18;
_31 = !(-22885_i16);
_10 = (-76_i8);
_29.0.fld5 = 16861_u16;
(*_21) = _4 * _3;
_29.0.fld4 = !_29.0.fld1.0;
_16 = _33.0 * _33.0;
_29.0.fld1 = (_29.0.fld4,);
_29.0.fld4 = _29.0.fld1.0;
_2 = (*_21) * (*_21);
(*_21) = _3;
_35 = &_20;
_29.0.fld3 = (*_21);
_10 = (-81_i8);
_29.0.fld5 = 9523_u16 << _23;
(*_21) = -_4;
_21 = &mut _29.0.fld3;
_25 = 133_u8;
_35 = &_15;
(*_21) = _1;
match _10 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211375 => bb9,
_ => bb3
}
}
bb45 = {
(*_21) = (*_28) as f64;
_25 = 180_u8;
(*_37) = _46 as u8;
(*_37) = 176_u8;
_74 = _23;
(*_21) = -_57.1;
_54.1 = 1491529516_u32 & 2562072204_u32;
_54.3.1 = _32;
(*_21) = _3 - _57.1;
(*_21) = _57.0 + _57.1;
(*_37) = !5_u8;
_22 = _57.2;
(*_38) = core::ptr::addr_of_mut!(_83);
(*_21) = _65.fld1 + _65.fld1;
(*_68) = _57.2 << (*_28);
(*_28) = !_57.3;
Goto(bb46)
}
bb46 = {
(*_38) = core::ptr::addr_of_mut!(_42);
_82 = _25 ^ (*_37);
_31 = _57.3 - _57.3;
(*_38) = core::ptr::addr_of_mut!(_83);
_54.3.1 = !_54.3.0;
_63 = core::ptr::addr_of_mut!((*_28));
(*_38) = core::ptr::addr_of_mut!(_42);
_15 = (*_21) as f32;
_91 = core::ptr::addr_of!(_90);
(*_91) = _33.1 >> (*_68);
(*_21) = _24 as f64;
_70 = _15 - _15;
(*_68) = _57.2;
Goto(bb47)
}
bb47 = {
(*_91) = _33.1 & _33.1;
_42 = _24;
_64 = RET;
_23 = (*_51) - (*_51);
(*_37) = _82;
_57.2 = _5 as u128;
(*_21) = _65.fld1 + _3;
(*_68) = _57.2 + _57.2;
(*_63) = _57.3;
(*_38) = core::ptr::addr_of_mut!(_83);
(*_63) = -_57.3;
Goto(bb48)
}
bb48 = {
(*_91) = _33.1;
_82 = !(*_37);
_87.2 = &_47;
_46 = _6;
_96.0.fld5 = (*_21) as u16;
_87.0 = _96.0.fld5 * _96.0.fld5;
_80 = !(*_51);
(*_38) = core::ptr::addr_of_mut!(_24);
(*_91) = !_33.1;
Goto(bb49)
}
bb49 = {
(*_63) = !_57.3;
(*_63) = -_57.3;
_87.1 = (*_37) as u16;
(*_21) = _57.0 - _57.0;
_49 = _70 * _70;
_23 = (*_51) * (*_51);
(*_68) = _19 as u128;
(*_38) = core::ptr::addr_of_mut!(_83);
_54.3.1 = (*_21) as u16;
(*_63) = _57.3;
(*_38) = core::ptr::addr_of_mut!(_24);
(*_21) = _57.1;
(*_28) = _57.3 & _57.3;
_66 = _13;
_54.1 = 768627333_u32 >> _57.2;
Goto(bb50)
}
bb50 = {
Call(_101 = dump_var(Move(_19), Move(_24), Move(_31), Move(_71)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_101 = dump_var(Move(_50), Move(_13), Move(_48), Move(_6)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_101 = dump_var(Move(_58), Move(_73), Move(_46), Move(_53)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_101 = dump_var(Move(_55), Move(_40), Move(_10), _102), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: *mut *const &'static mut f32) -> f64 {
mir! {
type RET = f64;
let _3: usize;
let _4: char;
let _5: u8;
let _6: i32;
let _7: f64;
let _8: [u128; 7];
let _9: [u128; 7];
let _10: isize;
let _11: *mut i128;
let _12: *mut u128;
let _13: *const u64;
let _14: f32;
let _15: isize;
let _16: *mut &'static mut [char; 6];
let _17: &'static &'static mut *const u64;
let _18: *const Adt22;
let _19: *const Adt41;
let _20: bool;
let _21: Adt78;
let _22: [u16; 5];
let _23: &'static mut &'static mut f32;
let _24: bool;
let _25: &'static mut *const u64;
let _26: f64;
let _27: isize;
let _28: isize;
let _29: i16;
let _30: *const Adt41;
let _31: *const *const (u64,);
let _32: *mut i128;
let _33: isize;
let _34: isize;
let _35: char;
let _36: *const Adt41;
let _37: isize;
let _38: char;
let _39: *const u64;
let _40: i64;
let _41: &'static mut &'static mut f32;
let _42: (&'static mut u32,);
let _43: Adt53;
let _44: *mut [u16; 5];
let _45: *mut i128;
let _46: *const Adt41;
let _47: u16;
let _48: bool;
let _49: ();
let _50: ();
{
RET = (-22556_i16) as f64;
_1 = 9223372036854775807_isize + 9223372036854775807_isize;
_1 = 31972_i16 as isize;
RET = _1 as f64;
RET = 42846_u16 as f64;
_1 = !86_isize;
RET = 2806666977_u32 as f64;
_1 = 37_isize;
_1 = !(-9223372036854775808_isize);
_1 = 40486_u16 as isize;
Call(RET = fn5(Move(_2), _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 2550196079314739997_usize;
_5 = RET as u8;
_3 = 1063367908_u32 as usize;
_5 = 42_u8;
RET = _5 as f64;
Goto(bb2)
}
bb2 = {
_4 = '\u{ff737}';
_4 = '\u{d01d3}';
_5 = 203_u8;
_3 = 0_usize;
_3 = 4_usize * 4115548727402675456_usize;
_4 = '\u{2e538}';
_3 = (-31_i8) as usize;
_6 = (-1630649458_i32) >> _3;
RET = (-8961628591625005237_i64) as f64;
_6 = -(-100981076_i32);
RET = _1 as f64;
_5 = 237_u8 << _6;
_3 = _4 as usize;
_5 = 65_u8 | 149_u8;
_3 = 1_usize;
_6 = 1661404974_i32 & (-732299195_i32);
_1 = true as isize;
_3 = (-308813954727647529_i64) as usize;
_3 = 6123952223953733051_usize + 2_usize;
_1 = 103714532059056786717620478574109382494_i128 as isize;
Goto(bb3)
}
bb3 = {
_7 = RET;
_7 = RET - RET;
_7 = RET - RET;
_3 = !2_usize;
RET = _7 + _7;
_3 = 13185715357093573969_usize - 5_usize;
_6 = 20340822_i32;
_5 = 92_u8 & 73_u8;
_7 = RET + RET;
_3 = 6_usize - 4186932242256361564_usize;
_4 = '\u{be6cb}';
_5 = 173379509091846044506139018027115685404_u128 as u8;
match _6 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
20340822 => bb9,
_ => bb8
}
}
bb4 = {
_4 = '\u{ff737}';
_4 = '\u{d01d3}';
_5 = 203_u8;
_3 = 0_usize;
_3 = 4_usize * 4115548727402675456_usize;
_4 = '\u{2e538}';
_3 = (-31_i8) as usize;
_6 = (-1630649458_i32) >> _3;
RET = (-8961628591625005237_i64) as f64;
_6 = -(-100981076_i32);
RET = _1 as f64;
_5 = 237_u8 << _6;
_3 = _4 as usize;
_5 = 65_u8 | 149_u8;
_3 = 1_usize;
_6 = 1661404974_i32 & (-732299195_i32);
_1 = true as isize;
_3 = (-308813954727647529_i64) as usize;
_3 = 6123952223953733051_usize + 2_usize;
_1 = 103714532059056786717620478574109382494_i128 as isize;
Goto(bb3)
}
bb5 = {
_3 = 2550196079314739997_usize;
_5 = RET as u8;
_3 = 1063367908_u32 as usize;
_5 = 42_u8;
RET = _5 as f64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
RET = _1 as f64;
_3 = 6_usize - 7_usize;
_4 = '\u{902be}';
Goto(bb10)
}
bb10 = {
_5 = 235_u8;
_7 = RET + RET;
_1 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_8 = [14106924196303435657936184005240081297_u128,336213520462399562624749659688394928371_u128,246613578898507507581355609477650048765_u128,42921814070111210250913810574371363023_u128,33471482164574084799612259150975945225_u128,15904019183416143975415351990936471976_u128,325283024101416202626374438399273690589_u128];
_8 = [37918384277430100953851550706768098959_u128,95447956691022097405407545227158058402_u128,233225496298566830054999979622410599175_u128,91799914144200599437902673721604510786_u128,285266617765994466419660395067330915026_u128,223179168155206724327452376493841969708_u128,199851493566610511768254452148945466177_u128];
_6 = 1222014025_i32 & 1296511256_i32;
_4 = '\u{b97a0}';
_8 = [252690845396255180123596450380500957239_u128,288218558412820560358123921217682816849_u128,121049541916148994888875880025970918294_u128,80532189877963789134083628384792160570_u128,35250534131112514067048191478539407036_u128,162815391032113112667607410633150999991_u128,230038963817436434590444562855888857886_u128];
_5 = 80_u8 & 76_u8;
_7 = 1605377033783297298_i64 as f64;
_10 = RET as isize;
_8 = [45123403270939078162711883949961010828_u128,227375778595246484544476314991275323119_u128,219873970543757919256275385448959112395_u128,312926643637562980395599164654367579591_u128,137913740284234903026764394519716686063_u128,171434818503645907908032447343729393170_u128,188674167673238453566094047889109236071_u128];
_1 = _10;
_1 = -_10;
_5 = 45_u8 | 214_u8;
_3 = (-8720784426152616478_i64) as usize;
_7 = -RET;
_1 = 16121373141051838136_u64 as isize;
_10 = _1;
_10 = _1 * _1;
_3 = !4863743478902573533_usize;
Goto(bb11)
}
bb11 = {
RET = _7 + _7;
_9 = [134288114506563709350079239114478066762_u128,332552142558232311052527228146552729849_u128,69992495446432597845395334237071822272_u128,28875786573707847388562301322482786059_u128,11602437917082634810019443241490064784_u128,66564974986298427661867416445245339260_u128,320284875888867907957546533861771799413_u128];
_6 = 198462231_i32 & 169824333_i32;
_7 = -RET;
_7 = RET;
RET = _7;
_10 = _1 >> _3;
_3 = 3_usize & 11799441846386938818_usize;
_9 = [17299488894714673930390392884033408729_u128,81988265734528844273119314930769051709_u128,251548725719291562713916321765801789076_u128,134271835437277513398969098695100416214_u128,263401144925427432816894016408159430060_u128,153635235779699958624968685919532903201_u128,55199664547709295419312882625623235410_u128];
Goto(bb12)
}
bb12 = {
_9 = [57807861984877550912039444218075763751_u128,135844945918705752725754099607915644312_u128,246662417324041289822408282703094951823_u128,152454702953260796650836207204614676052_u128,59521787518896535944458833355358504055_u128,14101179633928080497602944234234853841_u128,245214426621404192903848621336122376148_u128];
_9 = [284881553137732430486523520645003731506_u128,311628997907377765512990774486824456003_u128,111131644244346806412826164203344054041_u128,145933035309164951781864474904062460263_u128,127640324113076214503934149115153982360_u128,184136572109726541024465546553778114454_u128,305046626818569580078148382394873449894_u128];
_7 = RET;
_14 = 29411_u16 as f32;
_15 = _10;
_4 = '\u{3a59c}';
RET = -_7;
_3 = !2743320826511817427_usize;
_6 = 1246658903_i32 | (-1667835502_i32);
_5 = 99_u8;
_14 = 11064355934215616397_u64 as f32;
_9 = [236125454301700920760697680436580660092_u128,12481388808260436978793297793900310586_u128,200707758280379127550569191891507893272_u128,254313187150843178468278067591542070937_u128,233253558830303338182301489304725671884_u128,176571905363291984899016488199135499517_u128,101315688490558201192026017870351478760_u128];
_8 = _9;
_9 = [201221339290554086995719446539473537933_u128,236629278510147782604324715441584960791_u128,34079496690114203076108892832784997314_u128,99032183384785304614563586807736715933_u128,119673854959982013347740097332488394808_u128,10903972111281527403766349081205530780_u128,307736043051938468446550394870571771435_u128];
_1 = _15 >> _10;
_5 = 84_u8 >> _15;
_10 = !_1;
_9 = _8;
_14 = (-124_i8) as f32;
_8 = [294194183040000008570455009254689564826_u128,25540643592474390847580187311938282038_u128,259470239043685613725711525445372032907_u128,231275421497214851346064052358728161291_u128,194825562892268273667887878510239291944_u128,28870084200755594247919804718890957805_u128,103301104926447299539496258224474188962_u128];
_15 = _3 as isize;
_8 = [91225663567730925515783309144194032223_u128,302738004034680912431511225327492518776_u128,212515283107552217629121277773291766724_u128,138042932859504116414816731740369570070_u128,113552948531266035441846301823003424470_u128,198108013596959999481800219971131438852_u128,54757190093328686603473949431116682598_u128];
_5 = 84_u8 | 179_u8;
_3 = 2_usize * 15104196799775104011_usize;
_9 = [163752542223438159636631475752818827033_u128,36164967614869909416595825694744823159_u128,169589753884677820961871959854758110823_u128,162935922026112835580985727352656771979_u128,75359837468830175385260503271790965285_u128,50440427801505899633435917185527308878_u128,63814526995190880213052635743705745175_u128];
RET = _7;
_3 = 6_usize & 1_usize;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_8 = _9;
_8 = [38487690153549656549212365402287746392_u128,119526517740406496044584137774540427055_u128,147254648096295532092230128662540228814_u128,178859778752014240363265834971492256535_u128,237295036247242201434191002045053334455_u128,92382868925745929476099301782403982235_u128,311999283766885636706022693872065912971_u128];
_10 = -_15;
_1 = _10;
_10 = _1 * _15;
Goto(bb14)
}
bb14 = {
_15 = _10 >> _3;
_10 = 47907453522523789695562339337217296765_i128 as isize;
_14 = 4217870334048535724_i64 as f32;
_5 = 158_u8 & 176_u8;
_10 = 2720093269708530200_i64 as isize;
_5 = _3 as u8;
_14 = (-101_i8) as f32;
_6 = (-1578096028_i32) ^ 428073757_i32;
_1 = _15 >> _15;
Goto(bb15)
}
bb15 = {
_14 = 2363640791_u32 as f32;
_1 = _15 ^ _15;
_7 = RET;
_14 = _6 as f32;
_6 = -(-146422079_i32);
_6 = (-1069236068_i32) ^ (-1368443013_i32);
_9 = [116294937992487134583262050528602881280_u128,325551837848283818593893183885221803350_u128,100464782114235341724482726956938275922_u128,309905269595282233980251900086483922526_u128,95175847922295760025447277894893015057_u128,311732769455744633775008229049739640985_u128,318236877832339491232469444489520470841_u128];
_14 = _7 as f32;
_15 = _4 as isize;
Goto(bb16)
}
bb16 = {
_10 = !_1;
_3 = 5_usize << _10;
_4 = '\u{d2d05}';
_14 = 28529280866001874884072891102941422704_u128 as f32;
_3 = _5 as usize;
_15 = _10 + _1;
_9 = [151638261763154634276898593138383719951_u128,209985202442306278575685167046739820013_u128,17685262004890253804858047164966877588_u128,305144768786028349679261826777041529577_u128,105595496295338273664468629572373885898_u128,219807680636757764707688850816325716939_u128,136533865398000373485242756159151949372_u128];
RET = -_7;
_6 = 365056354_i32 + 1065303388_i32;
_3 = 4_usize & 6136706476746196325_usize;
_6 = _10 as i32;
_10 = _15 & _15;
_20 = true;
_8 = [82737194747562918367641816589039950427_u128,222819642332264346859120063144725425263_u128,70426543592607879276989471763474590944_u128,230546761794623799801845124054029327430_u128,263982675698842258560154508192448990131_u128,249973608485956143775990709156973114906_u128,211148126839112888802560940310451185236_u128];
RET = _7;
_15 = _10;
_14 = _6 as f32;
_14 = (-41628302689312068289693744326923995931_i128) as f32;
_14 = RET as f32;
_15 = !_1;
_5 = _6 as u8;
_9 = _8;
_1 = _10 + _10;
_8 = [222660158020787808342965215303529239018_u128,175932256382874148278070985125852220883_u128,246886758790624264218442297768977619476_u128,21902336297376655239304743691558705230_u128,264709379943955484758743867387344313176_u128,337835402273640071409631371564197311057_u128,175802469586275756494752668407577931924_u128];
Goto(bb17)
}
bb17 = {
_14 = 17440376072820900427_u64 as f32;
_5 = 116_u8;
_1 = _10;
_15 = 8640_i16 as isize;
_3 = 1_usize * 17478324929547461236_usize;
_7 = RET * RET;
_15 = _1 | _1;
_6 = 473198842_i32;
_20 = true;
_7 = -RET;
_3 = 1354194878981245733_usize;
_1 = _10;
_27 = _1;
_1 = _15;
_4 = '\u{810e1}';
_3 = 4_usize * 8380790181049003479_usize;
_3 = 12544511642225782241_usize >> _10;
_5 = 163_u8 - 94_u8;
_3 = 17661079786553566858_usize | 5_usize;
_7 = RET + RET;
Goto(bb18)
}
bb18 = {
_9 = [179009763244721239355627684763284394889_u128,38201619449811794191815104300160955766_u128,300685690413748514542901740087291382361_u128,193932790376363767098671384236126453792_u128,229027790088060583153646360324826737743_u128,28234822782012943519047515594668249592_u128,107184365962464284449157088382309136682_u128];
_8 = [99701124417680401915885086854467166632_u128,204080569503107366075049401267071548824_u128,146989253266589799502561119192155710496_u128,322047706225634160143254164528189385853_u128,152744149719740717606900380981136749531_u128,190519782618587485891127879550971673659_u128,144889354944305283797444827488429309457_u128];
_24 = _10 <= _15;
_22 = [7698_u16,22234_u16,52030_u16,31959_u16,61977_u16];
_6 = -(-1490245235_i32);
_29 = 25614_i16;
RET = _7 - _7;
_4 = '\u{d2cd6}';
_22 = [64171_u16,22987_u16,21555_u16,21755_u16,54969_u16];
_10 = _1 << _15;
_8 = [25355603027640444098385405698822995478_u128,243778076819131123852060339995168602205_u128,54789207805451712548597028209835625251_u128,20672480808639438547803432401243649494_u128,161387150593806782418500850394178927218_u128,336366662451794967072250960521408503563_u128,34931105246296377207485279386152930431_u128];
_28 = _29 as isize;
_27 = _1 | _15;
_6 = 1274111661_i32;
_20 = _24;
_28 = 10831302430523975850_u64 as isize;
_29 = 27661_i16 + (-16049_i16);
_33 = _1 | _10;
_35 = _4;
_5 = 149_u8;
_34 = _10 ^ _27;
_5 = !185_u8;
_27 = 3694986038866769583_u64 as isize;
Goto(bb19)
}
bb19 = {
RET = _7 - _7;
_28 = _1 << _10;
_27 = !_33;
_7 = RET;
_9 = _8;
_22 = [7081_u16,6391_u16,4215_u16,49730_u16,3204_u16];
_26 = _7 * RET;
_14 = _6 as f32;
Goto(bb20)
}
bb20 = {
_3 = _20 as usize;
_5 = 16_u8;
_29 = (-10418_i16) & (-19361_i16);
_28 = 87_i8 as isize;
_8 = _9;
_29 = (-19272_i16) | (-26654_i16);
_4 = _35;
_40 = 1918276632_u32 as i64;
Goto(bb21)
}
bb21 = {
_43.fld1 = -RET;
_44 = core::ptr::addr_of_mut!(_22);
_38 = _4;
(*_44) = [11278_u16,11503_u16,63532_u16,44625_u16,42226_u16];
_1 = _29 as isize;
_22 = [29049_u16,26309_u16,38137_u16,42856_u16,3662_u16];
(*_44) = [45551_u16,64642_u16,36890_u16,4035_u16,55295_u16];
_24 = !_20;
(*_44) = [59316_u16,53866_u16,42577_u16,56196_u16,45007_u16];
_34 = _33;
(*_44) = [31051_u16,48185_u16,25229_u16,13417_u16,45572_u16];
_6 = (-1_i8) as i32;
(*_44) = [56893_u16,64673_u16,3227_u16,51796_u16,10689_u16];
_22 = [40850_u16,30384_u16,4540_u16,60137_u16,37019_u16];
(*_44) = [58437_u16,42637_u16,12871_u16,39412_u16,28893_u16];
(*_44) = [49193_u16,60317_u16,23427_u16,58586_u16,27322_u16];
RET = _26 * _26;
(*_44) = [28378_u16,62460_u16,2618_u16,32301_u16,2170_u16];
_40 = 3533873189387287127_i64 - (-3359361520491366109_i64);
_37 = _33 >> _27;
(*_44) = [33772_u16,61909_u16,26609_u16,48272_u16,15297_u16];
(*_44) = [17218_u16,52844_u16,309_u16,634_u16,40187_u16];
_29 = 542911673_u32 as i16;
(*_44) = [41473_u16,8411_u16,58718_u16,52012_u16,52162_u16];
_33 = _37 >> _27;
_8 = [231329492515227364390026818475119026276_u128,177699938600907659137876079712031221324_u128,204602925116908694677521421888443004575_u128,195455056738986287542779003740879515714_u128,41501528479387514762650310963436111377_u128,10895850096614749262289500555254840424_u128,218190345333018139535458787659516736899_u128];
Goto(bb22)
}
bb22 = {
Call(_49 = dump_var(Move(_6), Move(_35), Move(_15), Move(_22)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_49 = dump_var(Move(_24), Move(_5), Move(_33), Move(_4)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_49 = dump_var(Move(_9), Move(_28), _50, _50), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *mut *const &'static mut f32,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> f64 {
mir! {
type RET = f64;
let _13: Adt22;
let _14: Adt75;
let _15: *const Adt41;
let _16: &'static &'static mut &'static mut f32;
let _17: &'static mut &'static mut f32;
let _18: u16;
let _19: (f64, f64, u128, i16);
let _20: isize;
let _21: usize;
let _22: *mut &'static mut [char; 6];
let _23: i64;
let _24: *const u32;
let _25: Adt75;
let _26: *const Adt41;
let _27: i16;
let _28: (u64,);
let _29: [u128; 7];
let _30: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _31: *const u8;
let _32: isize;
let _33: f64;
let _34: [isize; 6];
let _35: char;
let _36: i32;
let _37: char;
let _38: isize;
let _39: &'static u32;
let _40: isize;
let _41: u32;
let _42: (&'static mut u32,);
let _43: char;
let _44: char;
let _45: f64;
let _46: isize;
let _47: [u64; 2];
let _48: u16;
let _49: u16;
let _50: isize;
let _51: usize;
let _52: *mut [u128; 7];
let _53: *mut u128;
let _54: &'static mut *mut i128;
let _55: *mut [u16; 5];
let _56: [char; 5];
let _57: f32;
let _58: (i64, u64, &'static mut *const u64);
let _59: f32;
let _60: u32;
let _61: &'static mut &'static mut f32;
let _62: *const *const (u64,);
let _63: isize;
let _64: ();
let _65: ();
{
_6 = _8 | _5;
_10 = _12 << _6;
_11 = -_9;
_9 = _8 - _4;
_10 = !_6;
_8 = !_6;
_11 = _3 - _9;
_7 = _11;
RET = 4932591897846751880_u64 as f64;
_11 = _9 ^ _12;
_5 = _9;
_9 = _11;
_3 = 263924587451010475146292075734665795688_u128 as isize;
_2 = 65502_u16 as isize;
_9 = !_11;
_8 = _5 & _5;
_4 = (-1535643017_i32) as isize;
_2 = _8 ^ _7;
_18 = 61567_u16;
_9 = _5 + _5;
_18 = !53393_u16;
_19.3 = 17347389978267553352_u64 as i16;
RET = _3 as f64;
Call(_4 = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !_8;
_10 = -_2;
_6 = _5;
_7 = _10 + _5;
Goto(bb2)
}
bb2 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _19.0;
_19.2 = 83766609327637150961076744856094922382_u128 | 70024725602017830695190176216508820339_u128;
_3 = _12 * _2;
_18 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2 & Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_4 = _12;
_10 = !_2;
_11 = _5 | _9;
_3 = -_4;
_4 = 1639837155_i32 as isize;
_4 = !_5;
_19 = (RET, RET, 339748633416932516009857667443411394024_u128, (-29085_i16));
_19 = (RET, RET, 3949650001471542983215188755461818201_u128, (-17331_i16));
_5 = _11;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_11 = !_10;
_10 = _6 ^ _9;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = '\u{105490}' as u64;
_27 = '\u{47117}' as i16;
_11 = _4 << Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_3 = _10;
_19.3 = _27;
_21 = 4597959615335791620_usize >> _8;
_10 = _12;
_23 = -(-5668634848944443774_i64);
_19.1 = RET * RET;
Goto(bb4)
}
bb4 = {
_18 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
Goto(bb5)
}
bb5 = {
_9 = _11;
_7 = true as isize;
_19.3 = _27 ^ _27;
_23 = -2700356298481967864_i64;
_28 = (Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1,);
_23 = (-914817064508342447_i64);
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 as u16;
_10 = _5;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = (123_i8, _28.0, _18);
_19.3 = -_27;
_19.1 = _19.0 * _19.0;
_4 = _5 & _2;
match Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 {
123 => bb6,
_ => bb4
}
}
bb6 = {
_21 = 3_usize << _12;
_21 = 5_usize ^ 4_usize;
_5 = _12 | _4;
_8 = 1847673909_i32 as isize;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = _28.0 & _28.0;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = ((-116_i8), _28.0, _18);
_7 = _11 * _4;
_23 = 4133449787630686097_i64;
_28.0 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1 ^ Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_7 = _11 - _6;
Goto(bb7)
}
bb7 = {
_23 = '\u{e2ffe}' as i64;
_2 = _8 * _9;
_20 = 221_u8 as isize;
_3 = !_12;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = !_18;
_7 = _5 * _9;
_27 = _19.3 - _19.3;
match _19.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3949650001471542983215188755461818201 => bb12,
_ => bb11
}
}
bb8 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_2 = !_8;
_10 = -_2;
_6 = _5;
_7 = _10 + _5;
Goto(bb2)
}
bb10 = {
_18 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
Goto(bb5)
}
bb11 = {
RET = _19.0;
_19.2 = 83766609327637150961076744856094922382_u128 | 70024725602017830695190176216508820339_u128;
_3 = _12 * _2;
_18 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2 & Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_4 = _12;
_10 = !_2;
_11 = _5 | _9;
_3 = -_4;
_4 = 1639837155_i32 as isize;
_4 = !_5;
_19 = (RET, RET, 339748633416932516009857667443411394024_u128, (-29085_i16));
_19 = (RET, RET, 3949650001471542983215188755461818201_u128, (-17331_i16));
_5 = _11;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_11 = !_10;
_10 = _6 ^ _9;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = '\u{105490}' as u64;
_27 = '\u{47117}' as i16;
_11 = _4 << Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_3 = _10;
_19.3 = _27;
_21 = 4597959615335791620_usize >> _8;
_10 = _12;
_23 = -(-5668634848944443774_i64);
_19.1 = RET * RET;
Goto(bb4)
}
bb12 = {
_20 = _2;
_20 = _10;
_10 = _5 + _7;
_30.0.0 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0;
_30.0.1 = _19.0 as u64;
_30.1 = _19;
_30.0.1 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_19.3 = _30.1.2 as i16;
_11 = !_10;
_9 = -_11;
_30.1.1 = _30.1.0 + _19.0;
_30.0 = (Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0, _28.0, _18);
_2 = _10 - _10;
_6 = _30.0.2 as isize;
_3 = !_7;
_19.2 = 140722205545573696719051471213218944074_i128 as u128;
RET = -_19.1;
match _30.0.0 {
0 => bb13,
1 => bb14,
340282366920938463463374607431768211340 => bb16,
_ => bb15
}
}
bb13 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_23 = '\u{e2ffe}' as i64;
_2 = _8 * _9;
_20 = 221_u8 as isize;
_3 = !_12;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = !_18;
_7 = _5 * _9;
_27 = _19.3 - _19.3;
match _19.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3949650001471542983215188755461818201 => bb12,
_ => bb11
}
}
bb15 = {
_2 = !_8;
_10 = -_2;
_6 = _5;
_7 = _10 + _5;
Goto(bb2)
}
bb16 = {
_2 = _23 as isize;
_19 = (_30.1.1, _30.1.0, _30.1.2, _27);
_30.0.1 = 115_u8 as u64;
_28.0 = _20 as u64;
_28.0 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_30.1.1 = (-155182658893568961645884349671632418267_i128) as f64;
_30.0.0 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 << _7;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_5 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1 as isize;
_8 = 1086148757_u32 as isize;
match Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 {
340282366920938463463374607431768211340 => bb18,
_ => bb17
}
}
bb17 = {
_23 = '\u{e2ffe}' as i64;
_2 = _8 * _9;
_20 = 221_u8 as isize;
_3 = !_12;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = !_18;
_7 = _5 * _9;
_27 = _19.3 - _19.3;
match _19.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3949650001471542983215188755461818201 => bb12,
_ => bb11
}
}
bb18 = {
_2 = _30.1.2 as isize;
_30.1 = (RET, _19.0, _19.2, _27);
_29 = [_19.2,_30.1.2,_19.2,_30.1.2,_19.2,_19.2,_30.1.2];
_4 = _10 | _7;
_19.3 = _27 | _30.1.3;
_3 = _9 * _9;
_34 = [_4,_4,_11,_12,_3,_4];
_11 = _20 | _2;
_12 = !_10;
_30.1.2 = _19.2;
_21 = 7_usize | 4_usize;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = (_30.0.0, _28.0, _18);
_33 = _19.0 - _30.1.0;
_2 = (-24355746395970727931071182155622243435_i128) as isize;
_30.1.0 = _30.1.1 + _19.1;
_33 = 2683303186_u32 as f64;
Goto(bb19)
}
bb19 = {
_30.1.3 = -_27;
_28.0 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_19 = _30.1;
_38 = _30.1.2 as isize;
_13 = Adt22::Variant2 { fld0: _30.0 };
_10 = _3 << Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0;
_30.1.1 = _19.1;
_30.0.2 = _18 >> _10;
_3 = !_12;
_3 = !_10;
_30.1 = (RET, _19.1, _19.2, _19.3);
_19.1 = RET * _30.1.0;
_20 = _4 & _7;
_12 = _20 - _11;
_29 = [_19.2,_19.2,_30.1.2,_19.2,_19.2,_30.1.2,_19.2];
_5 = _10;
_8 = _10;
_6 = !_5;
match _30.1.2 {
0 => bb17,
1 => bb20,
2 => bb21,
3 => bb22,
4 => bb23,
5 => bb24,
6 => bb25,
3949650001471542983215188755461818201 => bb27,
_ => bb26
}
}
bb20 = {
_2 = _30.1.2 as isize;
_30.1 = (RET, _19.0, _19.2, _27);
_29 = [_19.2,_30.1.2,_19.2,_30.1.2,_19.2,_19.2,_30.1.2];
_4 = _10 | _7;
_19.3 = _27 | _30.1.3;
_3 = _9 * _9;
_34 = [_4,_4,_11,_12,_3,_4];
_11 = _20 | _2;
_12 = !_10;
_30.1.2 = _19.2;
_21 = 7_usize | 4_usize;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = (_30.0.0, _28.0, _18);
_33 = _19.0 - _30.1.0;
_2 = (-24355746395970727931071182155622243435_i128) as isize;
_30.1.0 = _30.1.1 + _19.1;
_33 = 2683303186_u32 as f64;
Goto(bb19)
}
bb21 = {
RET = _19.0;
_19.2 = 83766609327637150961076744856094922382_u128 | 70024725602017830695190176216508820339_u128;
_3 = _12 * _2;
_18 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2 & Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_4 = _12;
_10 = !_2;
_11 = _5 | _9;
_3 = -_4;
_4 = 1639837155_i32 as isize;
_4 = !_5;
_19 = (RET, RET, 339748633416932516009857667443411394024_u128, (-29085_i16));
_19 = (RET, RET, 3949650001471542983215188755461818201_u128, (-17331_i16));
_5 = _11;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_11 = !_10;
_10 = _6 ^ _9;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = '\u{105490}' as u64;
_27 = '\u{47117}' as i16;
_11 = _4 << Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_3 = _10;
_19.3 = _27;
_21 = 4597959615335791620_usize >> _8;
_10 = _12;
_23 = -(-5668634848944443774_i64);
_19.1 = RET * RET;
Goto(bb4)
}
bb22 = {
_2 = _23 as isize;
_19 = (_30.1.1, _30.1.0, _30.1.2, _27);
_30.0.1 = 115_u8 as u64;
_28.0 = _20 as u64;
_28.0 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_30.1.1 = (-155182658893568961645884349671632418267_i128) as f64;
_30.0.0 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 << _7;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_5 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1 as isize;
_8 = 1086148757_u32 as isize;
match Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0 {
340282366920938463463374607431768211340 => bb18,
_ => bb17
}
}
bb23 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb24 = {
_21 = 3_usize << _12;
_21 = 5_usize ^ 4_usize;
_5 = _12 | _4;
_8 = 1847673909_i32 as isize;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = _28.0 & _28.0;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = ((-116_i8), _28.0, _18);
_7 = _11 * _4;
_23 = 4133449787630686097_i64;
_28.0 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1 ^ Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1;
_7 = _11 - _6;
Goto(bb7)
}
bb25 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb26 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb27 = {
_38 = _6;
RET = _30.1.1 * _30.1.0;
_19 = (_30.1.1, _30.1.0, _30.1.2, _30.1.3);
match _19.2 {
0 => bb20,
1 => bb28,
3949650001471542983215188755461818201 => bb30,
_ => bb29
}
}
bb28 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb29 = {
_18 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
Goto(bb5)
}
bb30 = {
_30.0 = (Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0, Field::<(i8, u64, u16)>(Variant(_13, 2), 0).1, _18);
_44 = '\u{9934d}';
_21 = 6_usize & 9834234673532828823_usize;
_10 = _3 & _9;
_41 = _19.2 as u32;
_43 = _44;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)) = _30.0;
_41 = 192448544_u32 + 69115163_u32;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _30.0.0 as u16;
_33 = _30.1.0 + _30.1.1;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _30.0.2 * _18;
match _30.1.2 {
0 => bb2,
1 => bb31,
2 => bb32,
3949650001471542983215188755461818201 => bb34,
_ => bb33
}
}
bb31 = {
_19 = (RET, RET, 317688791756666497912092251682351638521_u128, 22437_i16);
_19.2 = !71745072649093930101162920084164754838_u128;
RET = _19.0;
_10 = !_7;
_19.2 = 285187908049381356115795468764807921871_u128;
_20 = '\u{60334}' as isize;
_5 = !_2;
_19.3 = 21090_i16 >> _11;
_19.3 = _7 as i16;
_12 = -_2;
_21 = !4_usize;
_19.2 = _18 as u128;
RET = _19.1 + _19.1;
_10 = -_7;
_18 = 40678_u16 - 45700_u16;
_3 = 54_u8 as isize;
_19.3 = (-18317_i16) | (-15347_i16);
_19.0 = _19.3 as f64;
RET = 2380513321_u32 as f64;
_10 = !_7;
_9 = _5 - _6;
_11 = RET as isize;
_7 = -_12;
_2 = !_10;
_12 = !_9;
_21 = 16312603184178892199_usize << _5;
Call(_13 = fn6(Move(_1), _18, _12, _19.0), ReturnTo(bb3), UnwindUnreachable())
}
bb32 = {
_23 = '\u{e2ffe}' as i64;
_2 = _8 * _9;
_20 = 221_u8 as isize;
_3 = !_12;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = !_18;
_7 = _5 * _9;
_27 = _19.3 - _19.3;
match _19.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3949650001471542983215188755461818201 => bb12,
_ => bb11
}
}
bb33 = {
_38 = _6;
RET = _30.1.1 * _30.1.0;
_19 = (_30.1.1, _30.1.0, _30.1.2, _30.1.3);
match _19.2 {
0 => bb20,
1 => bb28,
3949650001471542983215188755461818201 => bb30,
_ => bb29
}
}
bb34 = {
_19.2 = _30.1.2;
_13 = Adt22::Variant2 { fld0: _30.0 };
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = _28.0;
_19 = (RET, _33, _30.1.2, _30.1.3);
_30.1.0 = _33;
_30.1.3 = _27 | _19.3;
_29 = [_30.1.2,_19.2,_30.1.2,_30.1.2,_19.2,_30.1.2,_30.1.2];
_27 = _30.1.3 << _4;
_30.1 = _19;
_30.1.3 = _27 - _27;
_30.1.0 = RET;
_30.1.3 = _30.0.2 as i16;
_40 = _19.2 as isize;
_38 = (-696092351_i32) as isize;
_41 = 1622326271_u32 - 3312778333_u32;
_43 = _44;
_6 = _20 << _20;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).0 = -_30.0.0;
_39 = &_41;
_30.1.3 = !_27;
_3 = _9;
Goto(bb35)
}
bb35 = {
_30.0 = (Field::<(i8, u64, u16)>(Variant(_13, 2), 0).0, _28.0, _18);
_35 = _44;
_24 = core::ptr::addr_of!(_41);
_42.0 = &mut (*_24);
_30.1 = (_19.0, RET, _19.2, _27);
_30.1.3 = _27 | _27;
_49 = !Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_36 = (-1390847739_i32);
_51 = false as usize;
_30.1.0 = _33;
_7 = !_12;
_50 = _9;
_5 = (-12426209098265524806517825458571107343_i128) as isize;
_53 = core::ptr::addr_of_mut!(_30.1.2);
(*_53) = _19.2;
_20 = _4;
match (*_53) {
0 => bb27,
1 => bb2,
2 => bb21,
3 => bb36,
3949650001471542983215188755461818201 => bb38,
_ => bb37
}
}
bb36 = {
RET = _19.0;
_19.2 = 83766609327637150961076744856094922382_u128 | 70024725602017830695190176216508820339_u128;
_3 = _12 * _2;
_18 = Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2 & Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_4 = _12;
_10 = !_2;
_11 = _5 | _9;
_3 = -_4;
_4 = 1639837155_i32 as isize;
_4 = !_5;
_19 = (RET, RET, 339748633416932516009857667443411394024_u128, (-29085_i16));
_19 = (RET, RET, 3949650001471542983215188755461818201_u128, (-17331_i16));
_5 = _11;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).2 = _19.2 as u16;
_11 = !_10;
_10 = _6 ^ _9;
place!(Field::<(i8, u64, u16)>(Variant(_13, 2), 0)).1 = '\u{105490}' as u64;
_27 = '\u{47117}' as i16;
_11 = _4 << Field::<(i8, u64, u16)>(Variant(_13, 2), 0).2;
_3 = _10;
_19.3 = _27;
_21 = 4597959615335791620_usize >> _8;
_10 = _12;
_23 = -(-5668634848944443774_i64);
_19.1 = RET * RET;
Goto(bb4)
}
bb37 = {
_2 = !_8;
_10 = -_2;
_6 = _5;
_7 = _10 + _5;
Goto(bb2)
}
bb38 = {
_52 = core::ptr::addr_of_mut!(_29);
(*_53) = _30.0.2 as u128;
_21 = _51;
_5 = !_4;
_30.0.2 = _49 * _18;
RET = _33;
_30.1.2 = _33 as u128;
match _19.2 {
0 => bb36,
1 => bb17,
2 => bb27,
3 => bb7,
3949650001471542983215188755461818201 => bb40,
_ => bb39
}
}
bb39 = {
_2 = !_8;
_10 = -_2;
_6 = _5;
_7 = _10 + _5;
Goto(bb2)
}
bb40 = {
_48 = _30.0.2 * _30.0.2;
_30.0.1 = true as u64;
(*_53) = _30.1.3 as u128;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
Call((*_53) = core::intrinsics::transmute(_19.2), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
(*_52) = [(*_53),(*_53),(*_53),(*_53),_19.2,(*_53),(*_53)];
_30.1.0 = 51470311522900025494074512217145211711_i128 as f64;
(*_52) = [_30.1.2,(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_11 = _7 ^ _4;
(*_53) = _19.2 >> _9;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_50 = _8 << (*_53);
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),_30.1.2,(*_53)];
_46 = _6;
Call(_10 = core::intrinsics::bswap(_4), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
_7 = _20 - _6;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_6 = _10 - _50;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_7 = _6 - _6;
_37 = _43;
_40 = _50 | _50;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_58.1 = 126_u8 as u64;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_38 = _20;
_40 = _37 as isize;
_11 = _5 ^ _6;
(*_53) = _19.2 ^ _19.2;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
_32 = _5 + _10;
(*_52) = [(*_53),(*_53),(*_53),(*_53),(*_53),(*_53),(*_53)];
(*_52) = [(*_53),(*_53),(*_53),(*_53),_30.1.2,(*_53),(*_53)];
_57 = (-19212017904212655958773729790273716149_i128) as f32;
RET = _30.1.1 - _19.0;
_60 = 3229407388_u32;
Goto(bb43)
}
bb43 = {
Call(_64 = dump_var(Move(_34), Move(_60), Move(_48), Move(_29)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_64 = dump_var(Move(_43), Move(_11), Move(_44), Move(_3)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_64 = dump_var(Move(_6), Move(_9), Move(_40), Move(_41)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_64 = dump_var(Move(_27), Move(_50), Move(_20), Move(_35)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_64 = dump_var(Move(_5), _65, _65, _65), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut *const &'static mut f32,mut _2: u16,mut _3: isize,mut _4: f64) -> Adt22 {
mir! {
type RET = Adt22;
let _5: &'static mut u32;
let _6: f64;
let _7: *const *const (u64,);
let _8: f64;
let _9: char;
let _10: f64;
let _11: &'static mut f32;
let _12: isize;
let _13: f64;
let _14: Adt41;
let _15: f64;
let _16: &'static &'static mut *const u64;
let _17: (*mut &'static mut [char; 6],);
let _18: u8;
let _19: [i16; 7];
let _20: *const Adt22;
let _21: *const Adt41;
let _22: *const Adt22;
let _23: f64;
let _24: f32;
let _25: f64;
let _26: [u32; 1];
let _27: Adt75;
let _28: &'static mut f32;
let _29: &'static mut &'static mut f32;
let _30: &'static mut &'static mut f32;
let _31: isize;
let _32: i16;
let _33: i64;
let _34: f32;
let _35: f64;
let _36: f64;
let _37: (&'static mut u32,);
let _38: &'static u32;
let _39: [char; 6];
let _40: char;
let _41: bool;
let _42: ();
let _43: ();
{
_2 = 55279_u16;
_3 = 9223372036854775807_isize * 25_isize;
_4 = 50_u8 as f64;
_2 = 7760179451181631526_usize as u16;
_2 = 19172_u16 << _3;
Goto(bb1)
}
bb1 = {
_2 = 49930_u16 + 9719_u16;
_4 = 13340532773058068466_u64 as f64;
_2 = 12820_u16 << _3;
_3 = 94_u8 as isize;
_3 = -(-9223372036854775808_isize);
_4 = 9754727374978891675_u64 as f64;
_6 = _4 + _4;
_4 = _6;
_6 = _4 - _4;
_4 = _6 - _6;
_4 = _6;
_2 = 41480_u16;
_6 = _4 + _4;
_2 = 33664_u16;
_6 = -_4;
_3 = (-9223372036854775808_isize);
_2 = !42680_u16;
_6 = _4 * _4;
_3 = 10_isize >> _2;
_2 = !64893_u16;
_3 = 9223372036854775807_isize;
_6 = -_4;
_6 = _4 - _4;
_8 = _6 - _4;
Call(_2 = fn7(Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _6 * _8;
_4 = _8 - _8;
_2 = 56016_u16 & 930_u16;
_3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_3 = -59_isize;
_8 = _4;
_8 = _4 + _6;
_2 = 41294_u16 >> _3;
_4 = _6 * _6;
_4 = _8;
_9 = '\u{76156}';
_6 = -_4;
_6 = 8751_i16 as f64;
_9 = '\u{c5f95}';
_4 = _8 - _8;
_9 = '\u{9e620}';
_3 = -69_isize;
_8 = _4 - _4;
_4 = _8 + _8;
_9 = '\u{d6e13}';
_8 = _4 * _4;
Goto(bb3)
}
bb3 = {
_8 = -_4;
_4 = _8;
_2 = 24067_u16 - 6668_u16;
_3 = (-19833948386749939988673996084034125155_i128) as isize;
_3 = 38_isize | 13_isize;
_10 = -_4;
_8 = _4 * _4;
_8 = -_4;
_3 = _9 as isize;
_2 = 18366_u16 & 18447_u16;
_6 = -_10;
_4 = -_10;
_2 = 10236_u16;
_12 = -_3;
_9 = '\u{7f2be}';
_2 = 117_u8 as u16;
_6 = _4 * _4;
_2 = 31428_u16 - 57119_u16;
_6 = 50704802260411337050069277005145347060_u128 as f64;
Goto(bb4)
}
bb4 = {
_9 = '\u{f89a1}';
_10 = _8 - _8;
_6 = -_4;
_9 = '\u{5839a}';
Call(_6 = fn17(_9, _10, _10, _2, _2, _3, _12, _10, _8, _2, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = _10;
_6 = _13;
_15 = _10;
_10 = _15 * _4;
_4 = _6 * _6;
_8 = _6 - _13;
_10 = (-95_i8) as f64;
_10 = 6699600125168792349_i64 as f64;
_13 = -_6;
_9 = '\u{5401a}';
_13 = _6;
_18 = _9 as u8;
_18 = _12 as u8;
_10 = -_8;
_12 = _3 ^ _3;
_18 = 114_u8;
_4 = _8 * _13;
Goto(bb6)
}
bb6 = {
_18 = _3 as u8;
_9 = '\u{f295d}';
_2 = !62308_u16;
_10 = -_4;
_15 = _6;
_9 = '\u{f8d08}';
Call(RET = fn18(_4, _6, _3, _3, _13, _13, _9, _18, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = _12 as f64;
_12 = -_3;
_4 = -_15;
_10 = -_6;
_15 = _6 * _6;
_4 = _13 - _15;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)) = (28_i8, 479322299635950251_u64, _2);
_18 = 178_u8 >> Field::<(i8, u64, u16)>(Variant(RET, 2), 0).0;
_3 = _12 * _12;
_9 = '\u{2f837}';
_10 = _8;
_18 = 134_u8 | 54_u8;
_20 = core::ptr::addr_of!(RET);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 15050557543193428794_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = (18_i8, 4067352663757015575_u64, _2);
_3 = !_12;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = (2_i8, 4564514216409063980_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = 45_i8;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 5691731831178124680_u64 >> Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).0;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).0 as u16;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = !16305764648187254762_u64;
_15 = _10 * _4;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = 110_i8 | 116_i8;
_23 = 268567865647016796771715884699204530078_u128 as f64;
_15 = _10 - _10;
Goto(bb8)
}
bb8 = {
_24 = 8000004552579390437_usize as f32;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = (36_i8, 16268368107573014828_u64, _2);
_22 = core::ptr::addr_of!((*_20));
_19 = [21464_i16,(-5903_i16),26960_i16,(-21711_i16),(-1818_i16),3158_i16,17484_i16];
place!(Field::<(i8, u64, u16)>(Variant((*_22), 2), 0)).0 = -(-127_i8);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 14390827405466133090_u64 & 2852440847564515134_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = !6311031836530703201_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = (-36_i8);
_12 = _3 & _3;
Call(_18 = core::intrinsics::transmute(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = _18 as i8;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-22_i8), 14487162115964038869_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = !12_i8;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 7556298692224709279_u64 + 11334834454672254966_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = false as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-88_i8), 12655455910534039617_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 15194978154839357493_u64 & 18129003416679474403_u64;
_10 = -_8;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _18 as u16;
Goto(bb10)
}
bb10 = {
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = (45_i8, 1577055493004397366_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 6923185278930003162_u64 ^ 12332643414960408585_u64;
_11 = &mut _24;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = _12 as i8;
_26 = [3082125067_u32];
_31 = _12 + _3;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 23022988569943066_u64 | 9505299120232571528_u64;
_34 = (*_11) + (*_11);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = _9 as u64;
(*_11) = _34 + _34;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2 - _2;
_30 = &mut _11;
_28 = &mut _34;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2 - _2;
Goto(bb11)
}
bb11 = {
(*_28) = _31 as f32;
(*_28) = 5579382896185543360_i64 as f32;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 1770835057225048108_u64 | 869325006108880702_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2;
_31 = _3;
(*_28) = 368402389_i32 as f32;
(*_30) = &mut (*_28);
_32 = (-7802_i16) ^ 25300_i16;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 12475062967435222622_u64 << Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = (112_i8, 17487156153678695275_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = 1666518149_i32 as u16;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = !18080047362058623952_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 8558146538426866724_u64 * 17040492415658970800_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).1 as i8;
_20 = core::ptr::addr_of!((*_20));
Call(place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = core::intrinsics::transmute(_3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_30) = Move(_28);
_29 = Move(_30);
_23 = (-1209383648_i32) as f64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 2667236921450218483_u64 ^ 2892226313765775257_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-93_i8), 12402577482526258784_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 12294164452087548276_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-127_i8), 2300786285591733663_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = 23_i8 * 9_i8;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).0 = (-38_i8);
_10 = -_13;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).1 = 17859206816489969447_u64 | 16518590707991457756_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-107_i8), 11612674750616019669_u64, _2);
_33 = -(-3910411801090515753_i64);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-52_i8), 6642544512812027077_u64, _2);
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 5695318694877271662_u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2 ^ _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)) = ((-73_i8), 4212077210914616110_u64, _2);
Goto(bb13)
}
bb13 = {
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 153089393931483732075475858754768734303_u128 as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _2 + _2;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).1 = 9945029299435237978_u64 << Field::<(i8, u64, u16)>(Variant((*_20), 2), 0).0;
place!(Field::<(i8, u64, u16)>(Variant((*_20), 2), 0)).2 = _6 as u16;
_32 = 19105_i16 | (-23945_i16);
Goto(bb14)
}
bb14 = {
Call(_42 = dump_var(Move(_9), Move(_2), Move(_19), Move(_12)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_42 = dump_var(Move(_31), _43, _43, _43), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *mut *const &'static mut f32) -> u16 {
mir! {
type RET = u16;
let _2: bool;
let _3: isize;
let _4: isize;
let _5: [u8; 4];
let _6: char;
let _7: Adt75;
let _8: &'static &'static mut &'static mut f32;
let _9: &'static mut (u64,);
let _10: f64;
let _11: Adt22;
let _12: *const &'static mut f32;
let _13: isize;
let _14: [i8; 2];
let _15: *mut [u16; 5];
let _16: (bool, [u16; 5]);
let _17: *const Adt41;
let _18: [i64; 3];
let _19: u128;
let _20: f32;
let _21: u64;
let _22: &'static f32;
let _23: u128;
let _24: char;
let _25: *const u8;
let _26: f32;
let _27: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _28: f32;
let _29: (Adt26, Adt41);
let _30: [u128; 7];
let _31: f32;
let _32: &'static f32;
let _33: f32;
let _34: f32;
let _35: i128;
let _36: &'static f32;
let _37: (u16, u16, &'static f32, char);
let _38: f64;
let _39: *const (u64,);
let _40: f64;
let _41: &'static mut *const u64;
let _42: i128;
let _43: isize;
let _44: bool;
let _45: i8;
let _46: f64;
let _47: f64;
let _48: isize;
let _49: (&'static isize, *const (u64,), i32);
let _50: (*mut [u16; 5], [u16; 5], u32, [u16; 5]);
let _51: u8;
let _52: f32;
let _53: isize;
let _54: u32;
let _55: u32;
let _56: *mut *const &'static mut f32;
let _57: &'static &'static mut &'static mut f32;
let _58: *mut [u16; 5];
let _59: u128;
let _60: (&'static isize, *const (u64,), i32);
let _61: ();
let _62: ();
{
RET = 60490_u16 << (-9223372036854775808_isize);
RET = 21996_u16 ^ 49761_u16;
RET = 12931_u16 * 35845_u16;
RET = 54339_u16;
RET = 65509_u16 - 36224_u16;
RET = 9223372036854775807_isize as u16;
RET = 2835_u16 - 61302_u16;
RET = 17182_u16;
RET = 23050_u16 - 24979_u16;
RET = 27184_u16;
_2 = true;
RET = 32201_u16;
_2 = false;
_5 = [95_u8,116_u8,120_u8,120_u8];
_4 = 9223372036854775807_isize ^ 34_isize;
_3 = _4 >> RET;
_2 = _4 < _4;
_6 = '\u{44377}';
_3 = _4 - _4;
_4 = _3 - _3;
_3 = _4;
_3 = _4;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
32201 => bb8,
_ => bb7
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_4 = _3;
_6 = '\u{887d2}';
_4 = _3 - _3;
_10 = 23143034376508525300810118655056393222_i128 as f64;
RET = 33702_u16 >> _4;
_4 = 79067499602674116983157181913762964182_u128 as isize;
_6 = '\u{7ed1f}';
_5 = [253_u8,250_u8,56_u8,61_u8];
_10 = 123_i8 as f64;
_5 = [228_u8,173_u8,243_u8,60_u8];
_6 = '\u{9165c}';
_5 = [178_u8,118_u8,159_u8,180_u8];
_6 = '\u{392c}';
_10 = 187_u8 as f64;
_5 = [91_u8,32_u8,95_u8,150_u8];
_4 = -_3;
_1 = core::ptr::addr_of_mut!(_12);
_5 = [89_u8,65_u8,252_u8,115_u8];
_4 = _3 << _3;
RET = 29868_u16 ^ 21637_u16;
_4 = -_3;
_3 = -_4;
_2 = _4 == _3;
_2 = _4 == _4;
Goto(bb9)
}
bb9 = {
_13 = _4 * _4;
_14 = [(-19_i8),27_i8];
_2 = !true;
_13 = _3;
_6 = '\u{109508}';
_16.1 = [RET,RET,RET,RET,RET];
_10 = 1821528860_u32 as f64;
Call(_2 = fn8(Move(_1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6 = '\u{1c273}';
_1 = core::ptr::addr_of_mut!(_12);
_13 = _3 << _4;
_18 = [5151950544112951995_i64,5712492713889813300_i64,(-2301444205007113189_i64)];
_15 = core::ptr::addr_of_mut!(_16.1);
(*_15) = [RET,RET,RET,RET,RET];
_3 = -_13;
(*_15) = [RET,RET,RET,RET,RET];
_16.0 = !_2;
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
_16.0 = _2 ^ _2;
(*_15) = [RET,RET,RET,RET,RET];
Call(_13 = fn15(Move(_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16.0 = _2 ^ _2;
(*_15) = [RET,RET,RET,RET,RET];
_13 = -_3;
_19 = 6_i8 as u128;
_1 = core::ptr::addr_of_mut!(_12);
(*_15) = [RET,RET,RET,RET,RET];
_1 = core::ptr::addr_of_mut!((*_1));
_21 = 18349354732197213605_u64 - 17452124502085954337_u64;
_2 = _16.0;
_16.1 = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
Goto(bb12)
}
bb12 = {
_24 = _6;
Goto(bb13)
}
bb13 = {
(*_15) = [RET,RET,RET,RET,RET];
_21 = 3013794164370042156_u64;
(*_15) = [RET,RET,RET,RET,RET];
_26 = RET as f32;
_26 = (-68099234994237296046589389741156884678_i128) as f32;
(*_15) = [RET,RET,RET,RET,RET];
_23 = _19;
_10 = _4 as f64;
_5 = [56_u8,254_u8,168_u8,1_u8];
_15 = core::ptr::addr_of_mut!((*_15));
(*_15) = [RET,RET,RET,RET,RET];
_15 = core::ptr::addr_of_mut!((*_15));
_18 = [(-317347634898752353_i64),5550638999156392187_i64,(-7923923989086999418_i64)];
(*_15) = [RET,RET,RET,RET,RET];
_29.0.fld1 = (_21,);
_29.0.fld1 = (_21,);
_9 = &mut _29.0.fld1;
_22 = &_26;
Goto(bb14)
}
bb14 = {
_14 = [115_i8,121_i8];
(*_9).0 = _21 | _21;
_32 = &(*_22);
(*_9).0 = _21 - _21;
_23 = !_19;
(*_9) = (_21,);
(*_15) = [RET,RET,RET,RET,RET];
(*_9).0 = _21 << _13;
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
_15 = core::ptr::addr_of_mut!((*_15));
(*_15) = [RET,RET,RET,RET,RET];
(*_9).0 = _21 + _21;
(*_15) = [RET,RET,RET,RET,RET];
Goto(bb15)
}
bb15 = {
(*_9).0 = 2850002791_u32 as u64;
(*_9) = (_21,);
_3 = 245_u8 as isize;
Goto(bb16)
}
bb16 = {
(*_15) = [RET,RET,RET,RET,RET];
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb17)
}
bb17 = {
(*_9) = (_21,);
(*_15) = [RET,RET,RET,RET,RET];
_5 = [182_u8,132_u8,77_u8,151_u8];
(*_9) = (_21,);
_11 = Adt22::Variant0 { fld0: _16.0,fld1: _10,fld2: _13,fld3: (*_32),fld4: 5_usize,fld5: RET,fld6: (*_9),fld7: _19 };
(*_9) = (_21,);
_5 = [63_u8,8_u8,74_u8,73_u8];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_15) = [RET,RET,Field::<u16>(Variant(_11, 0), 5),RET,RET];
(*_15) = [Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),RET];
_19 = 5_usize as u128;
(*_9) = Field::<(u64,)>(Variant(_11, 0), 6);
(*_9).0 = !Field::<(u64,)>(Variant(_11, 0), 6).0;
(*_15) = [RET,Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),RET,Field::<u16>(Variant(_11, 0), 5)];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_15) = [RET,RET,RET,RET,Field::<u16>(Variant(_11, 0), 5)];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_9).0 = (*_22) as u64;
_14 = [88_i8,63_i8];
_34 = -(*_32);
_10 = Field::<f64>(Variant(_11, 0), 1) - Field::<f64>(Variant(_11, 0), 1);
(*_9).0 = _21 - Field::<(u64,)>(Variant(_11, 0), 6).0;
_35 = (-115125419891599982525034874802236131484_i128) | 125714154404642560664723423739288610409_i128;
place!(Field::<bool>(Variant(_11, 0), 0)) = _10 > _10;
(*_9) = (_21,);
_37.0 = Field::<u16>(Variant(_11, 0), 5) * Field::<u16>(Variant(_11, 0), 5);
(*_15) = [Field::<u16>(Variant(_11, 0), 5),_37.0,_37.0,_37.0,Field::<u16>(Variant(_11, 0), 5)];
match (*_9).0 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
3013794164370042156 => bb25,
_ => bb24
}
}
bb18 = {
(*_15) = [RET,RET,RET,RET,RET];
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb17)
}
bb19 = {
(*_9).0 = 2850002791_u32 as u64;
(*_9) = (_21,);
_3 = 245_u8 as isize;
Goto(bb16)
}
bb20 = {
Return()
}
bb21 = {
(*_15) = [RET,RET,RET,RET,RET];
_21 = 3013794164370042156_u64;
(*_15) = [RET,RET,RET,RET,RET];
_26 = RET as f32;
_26 = (-68099234994237296046589389741156884678_i128) as f32;
(*_15) = [RET,RET,RET,RET,RET];
_23 = _19;
_10 = _4 as f64;
_5 = [56_u8,254_u8,168_u8,1_u8];
_15 = core::ptr::addr_of_mut!((*_15));
(*_15) = [RET,RET,RET,RET,RET];
_15 = core::ptr::addr_of_mut!((*_15));
_18 = [(-317347634898752353_i64),5550638999156392187_i64,(-7923923989086999418_i64)];
(*_15) = [RET,RET,RET,RET,RET];
_29.0.fld1 = (_21,);
_29.0.fld1 = (_21,);
_9 = &mut _29.0.fld1;
_22 = &_26;
Goto(bb14)
}
bb22 = {
_4 = _3;
_6 = '\u{887d2}';
_4 = _3 - _3;
_10 = 23143034376508525300810118655056393222_i128 as f64;
RET = 33702_u16 >> _4;
_4 = 79067499602674116983157181913762964182_u128 as isize;
_6 = '\u{7ed1f}';
_5 = [253_u8,250_u8,56_u8,61_u8];
_10 = 123_i8 as f64;
_5 = [228_u8,173_u8,243_u8,60_u8];
_6 = '\u{9165c}';
_5 = [178_u8,118_u8,159_u8,180_u8];
_6 = '\u{392c}';
_10 = 187_u8 as f64;
_5 = [91_u8,32_u8,95_u8,150_u8];
_4 = -_3;
_1 = core::ptr::addr_of_mut!(_12);
_5 = [89_u8,65_u8,252_u8,115_u8];
_4 = _3 << _3;
RET = 29868_u16 ^ 21637_u16;
_4 = -_3;
_3 = -_4;
_2 = _4 == _3;
_2 = _4 == _4;
Goto(bb9)
}
bb23 = {
_16.0 = _2 ^ _2;
(*_15) = [RET,RET,RET,RET,RET];
_13 = -_3;
_19 = 6_i8 as u128;
_1 = core::ptr::addr_of_mut!(_12);
(*_15) = [RET,RET,RET,RET,RET];
_1 = core::ptr::addr_of_mut!((*_1));
_21 = 18349354732197213605_u64 - 17452124502085954337_u64;
_2 = _16.0;
_16.1 = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
Goto(bb12)
}
bb24 = {
_6 = '\u{1c273}';
_1 = core::ptr::addr_of_mut!(_12);
_13 = _3 << _4;
_18 = [5151950544112951995_i64,5712492713889813300_i64,(-2301444205007113189_i64)];
_15 = core::ptr::addr_of_mut!(_16.1);
(*_15) = [RET,RET,RET,RET,RET];
_3 = -_13;
(*_15) = [RET,RET,RET,RET,RET];
_16.0 = !_2;
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
(*_15) = [RET,RET,RET,RET,RET];
_16.0 = _2 ^ _2;
(*_15) = [RET,RET,RET,RET,RET];
Call(_13 = fn15(Move(_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb25 = {
_20 = (*_9).0 as f32;
_30 = [_19,_23,_19,Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),_19,_19];
Goto(bb26)
}
bb26 = {
(*_15) = [_37.0,Field::<u16>(Variant(_11, 0), 5),RET,RET,Field::<u16>(Variant(_11, 0), 5)];
_14 = [(-113_i8),(-56_i8)];
place!(Field::<(u64,)>(Variant(_11, 0), 6)).0 = (*_9).0;
_31 = Field::<f32>(Variant(_11, 0), 3);
(*_9).0 = 3818877239108521797_usize as u64;
(*_9) = Field::<(u64,)>(Variant(_11, 0), 6);
_39 = core::ptr::addr_of!((*_9));
(*_9).0 = Field::<(u64,)>(Variant(_11, 0), 6).0 * _21;
_4 = !Field::<isize>(Variant(_11, 0), 2);
(*_9).0 = Field::<(u64,)>(Variant(_11, 0), 6).0;
place!(Field::<u128>(Variant(_11, 0), 7)) = _2 as u128;
(*_15) = [_37.0,Field::<u16>(Variant(_11, 0), 5),RET,RET,_37.0];
_37.3 = _24;
(*_15) = [_37.0,_37.0,Field::<u16>(Variant(_11, 0), 5),RET,RET];
match _21 {
0 => bb27,
1 => bb28,
3013794164370042156 => bb30,
_ => bb29
}
}
bb27 = {
Return()
}
bb28 = {
Return()
}
bb29 = {
(*_9) = (_21,);
(*_15) = [RET,RET,RET,RET,RET];
_5 = [182_u8,132_u8,77_u8,151_u8];
(*_9) = (_21,);
_11 = Adt22::Variant0 { fld0: _16.0,fld1: _10,fld2: _13,fld3: (*_32),fld4: 5_usize,fld5: RET,fld6: (*_9),fld7: _19 };
(*_9) = (_21,);
_5 = [63_u8,8_u8,74_u8,73_u8];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_15) = [RET,RET,Field::<u16>(Variant(_11, 0), 5),RET,RET];
(*_15) = [Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),RET];
_19 = 5_usize as u128;
(*_9) = Field::<(u64,)>(Variant(_11, 0), 6);
(*_9).0 = !Field::<(u64,)>(Variant(_11, 0), 6).0;
(*_15) = [RET,Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5),RET,Field::<u16>(Variant(_11, 0), 5)];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_15) = [RET,RET,RET,RET,Field::<u16>(Variant(_11, 0), 5)];
(*_9) = (Field::<(u64,)>(Variant(_11, 0), 6).0,);
(*_9).0 = (*_22) as u64;
_14 = [88_i8,63_i8];
_34 = -(*_32);
_10 = Field::<f64>(Variant(_11, 0), 1) - Field::<f64>(Variant(_11, 0), 1);
(*_9).0 = _21 - Field::<(u64,)>(Variant(_11, 0), 6).0;
_35 = (-115125419891599982525034874802236131484_i128) | 125714154404642560664723423739288610409_i128;
place!(Field::<bool>(Variant(_11, 0), 0)) = _10 > _10;
(*_9) = (_21,);
_37.0 = Field::<u16>(Variant(_11, 0), 5) * Field::<u16>(Variant(_11, 0), 5);
(*_15) = [Field::<u16>(Variant(_11, 0), 5),_37.0,_37.0,_37.0,Field::<u16>(Variant(_11, 0), 5)];
match (*_9).0 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
3013794164370042156 => bb25,
_ => bb24
}
}
bb30 = {
place!(Field::<bool>(Variant(_11, 0), 0)) = !_2;
(*_9).0 = !_21;
_5 = [121_u8,188_u8,58_u8,250_u8];
(*_15) = [RET,RET,RET,Field::<u16>(Variant(_11, 0), 5),Field::<u16>(Variant(_11, 0), 5)];
(*_9).0 = Field::<(u64,)>(Variant(_11, 0), 6).0 - Field::<(u64,)>(Variant(_11, 0), 6).0;
_37 = (Field::<u16>(Variant(_11, 0), 5), Field::<u16>(Variant(_11, 0), 5), Move(_22), _6);
(*_15) = [_37.1,RET,_37.1,_37.1,RET];
_38 = _10 - _10;
_5 = [244_u8,145_u8,115_u8,23_u8];
(*_9) = Field::<(u64,)>(Variant(_11, 0), 6);
(*_9) = Field::<(u64,)>(Variant(_11, 0), 6);
_39 = core::ptr::addr_of!((*_9));
_34 = (*_32) * (*_32);
(*_39).0 = Field::<(u64,)>(Variant(_11, 0), 6).0;
_36 = &(*_32);
_35 = !(-107041135846347773655074788485283970917_i128);
(*_39).0 = Field::<(u64,)>(Variant(_11, 0), 6).0 | Field::<(u64,)>(Variant(_11, 0), 6).0;
place!(Field::<(u64,)>(Variant(_11, 0), 6)).0 = !(*_39).0;
(*_39).0 = (*_32) as u64;
_21 = !(*_39).0;
place!(Field::<(u64,)>(Variant(_11, 0), 6)) = ((*_9).0,);
(*_15) = [RET,_37.1,_37.1,_37.0,RET];
Goto(bb31)
}
bb31 = {
(*_39).0 = _21 >> Field::<u128>(Variant(_11, 0), 7);
_39 = core::ptr::addr_of!((*_39));
(*_15) = [RET,Field::<u16>(Variant(_11, 0), 5),_37.1,_37.1,_37.1];
_30 = [Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7),Field::<u128>(Variant(_11, 0), 7)];
_37.0 = _13 as u16;
_43 = 7_usize as isize;
_26 = _31;
_37.2 = &_20;
_11 = Adt22::Variant3 { fld0: _2,fld1: _24,fld2: 97733486873674422_i64,fld3: (*_39).0,fld4: (*_15),fld5: _26 };
_50.2 = 3604513163_u32 >> (*_9).0;
(*_15) = Field::<[u16; 5]>(Variant(_11, 3), 4);
_5 = [91_u8,27_u8,87_u8,30_u8];
_33 = 7625510334007374518_i64 as f32;
(*_9).0 = Field::<u64>(Variant(_11, 3), 3) ^ Field::<u64>(Variant(_11, 3), 3);
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
Goto(bb32)
}
bb32 = {
_5 = [85_u8,173_u8,82_u8,79_u8];
(*_9).0 = Field::<u64>(Variant(_11, 3), 3) - Field::<u64>(Variant(_11, 3), 3);
(*_9) = (_21,);
(*_15) = [_37.0,_37.0,RET,_37.0,_37.0];
(*_15) = [_37.0,_37.0,_37.0,_37.0,_37.0];
(*_15) = [_37.0,_37.0,RET,_37.0,_37.0];
(*_9).0 = _35 as u64;
(*_9).0 = !Field::<u64>(Variant(_11, 3), 3);
_22 = &_33;
_47 = _38;
(*_15) = [_37.0,_37.0,_37.0,_37.0,_37.0];
_53 = _13 << (*_9).0;
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
_25 = core::ptr::addr_of!(_51);
_27.0.1 = Move(_39);
(*_25) = _19 as u8;
(*_9).0 = !Field::<u64>(Variant(_11, 3), 3);
(*_25) = (*_9).0 as u8;
Goto(bb33)
}
bb33 = {
(*_9).0 = Field::<u64>(Variant(_11, 3), 3) | Field::<u64>(Variant(_11, 3), 3);
_6 = _24;
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
_35 = (-69914248372120683760230105032944313422_i128) + (-137360705343800345456035983979194900935_i128);
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
(*_9).0 = !Field::<u64>(Variant(_11, 3), 3);
_28 = -(*_22);
(*_25) = Field::<bool>(Variant(_11, 3), 0) as u8;
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
place!(Field::<u64>(Variant(_11, 3), 3)) = (*_9).0 | (*_9).0;
_48 = 3_usize as isize;
_39 = core::ptr::addr_of!((*_9));
(*_15) = Field::<[u16; 5]>(Variant(_11, 3), 4);
RET = _37.0 - _37.1;
(*_39) = (Field::<u64>(Variant(_11, 3), 3),);
_23 = _19 << _53;
_26 = -_33;
(*_9) = (Field::<u64>(Variant(_11, 3), 3),);
(*_25) = !172_u8;
(*_9).0 = Field::<u64>(Variant(_11, 3), 3) >> _50.2;
(*_15) = [_37.0,_37.0,_37.0,RET,RET];
_60.0 = &_3;
(*_9).0 = 1_usize as u64;
_37.2 = Move(_22);
_31 = _47 as f32;
Goto(bb34)
}
bb34 = {
Call(_61 = dump_var(Move(_2), Move(_18), Move(_53), Move(_35)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_61 = dump_var(Move(_14), Move(_4), Move(_19), Move(_3)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_61 = dump_var(Move(_5), _62, _62, _62), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *mut *const &'static mut f32) -> bool {
mir! {
type RET = bool;
let _2: f64;
let _3: (*mut &'static mut [char; 6],);
let _4: u32;
let _5: usize;
let _6: u16;
let _7: f32;
let _8: Adt75;
let _9: f32;
let _10: [isize; 6];
let _11: *const u64;
let _12: *const Adt41;
let _13: isize;
let _14: *const *const (u64,);
let _15: ();
let _16: ();
{
RET = !false;
RET = true;
RET = true;
RET = 718210648181003855_i64 < 5991095016500936332_i64;
RET = false;
RET = !true;
_2 = 267774461595641384568947547029086357738_u128 as f64;
RET = false;
RET = !false;
RET = true;
RET = true;
_2 = 121204096699484351955070965077815472531_i128 as f64;
RET = _2 <= _2;
_2 = 2143318333_u32 as f64;
RET = !true;
RET = _2 == _2;
_4 = 4071283151_u32;
_4 = 15305037463779657950_u64 as u32;
_5 = 1259514044902286145_usize & 17180734993695336762_usize;
RET = true;
RET = !true;
_6 = 3520_u16 << _4;
_6 = 6030_u16 << _4;
_2 = 26497_i16 as f64;
_4 = (-113325410975128619156497396610487647668_i128) as u32;
_4 = 146014730051743793577615778254696810314_i128 as u32;
Goto(bb1)
}
bb1 = {
Call(RET = fn9(Move(_1), _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 5_usize + 6_usize;
_5 = 3_usize;
_2 = 9223372036854775807_isize as f64;
_6 = 11489_u16 << _4;
_7 = 9223372036854775807_isize as f32;
_7 = (-105206841451364849000772322878737017321_i128) as f32;
_7 = _6 as f32;
_2 = 156785757108544517106233506013173443803_u128 as f64;
_7 = 410336085_i32 as f32;
RET = !false;
RET = false;
_9 = _7;
_10 = [(-116_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-97_isize),9223372036854775807_isize];
RET = !false;
_10 = [9223372036854775807_isize,(-40_isize),(-18_isize),9223372036854775807_isize,9223372036854775807_isize,(-98_isize)];
_9 = _7 * _7;
_2 = _6 as f64;
_10[_5] = _4 as isize;
_10[_5] = 9223372036854775807_isize * 9223372036854775807_isize;
_10[_5] = 9223372036854775807_isize;
_6 = 38152_u16 - 167_u16;
_10 = [(-1_isize),(-9223372036854775808_isize),12_isize,19_isize,69_isize,115_isize];
_10[_5] = (-9223372036854775808_isize) & 9223372036854775807_isize;
_7 = _9 + _9;
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
4 => bb7,
5 => bb8,
3 => bb10,
_ => bb9
}
}
bb3 = {
Call(RET = fn9(Move(_1), _6), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_6 = (-52879043343459728389028379556851452027_i128) as u16;
_4 = 3931849978_u32 - 3878546729_u32;
_10 = [96_isize,(-90_isize),(-104_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [33_isize,9223372036854775807_isize,(-9223372036854775808_isize),27_isize,9223372036854775807_isize,9223372036854775807_isize];
_6 = _2 as u16;
RET = _5 < _5;
_6 = 45091_u16;
_9 = _7 * _7;
_5 = _7 as usize;
_2 = (-30_isize) as f64;
_10 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),12_isize,(-9223372036854775808_isize),(-9_isize)];
_10 = [111_isize,5_isize,(-9223372036854775808_isize),(-81_isize),(-81_isize),(-9223372036854775808_isize)];
_7 = _9 + _9;
_9 = _7 + _7;
_4 = 3703082939_u32;
_2 = _4 as f64;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-128_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),88_isize];
_4 = !494105399_u32;
_13 = 9223372036854775807_isize + (-9223372036854775808_isize);
RET = true & true;
RET = _9 == _9;
_10 = [_13,_13,_13,_13,_13,_13];
_13 = (-9223372036854775808_isize);
_9 = _7 + _7;
RET = _9 != _7;
_6 = _13 as u16;
Goto(bb11)
}
bb11 = {
Call(_15 = dump_var(Move(_6), Move(_13), _16, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *mut *const &'static mut f32,mut _2: u16) -> bool {
mir! {
type RET = bool;
let _3: *const u32;
let _4: char;
let _5: f64;
let _6: *const (u64,);
let _7: *mut i16;
let _8: isize;
let _9: &'static mut (u64,);
let _10: u16;
let _11: *mut u128;
let _12: f64;
let _13: i32;
let _14: isize;
let _15: f64;
let _16: f32;
let _17: isize;
let _18: f32;
let _19: &'static mut (u64,);
let _20: isize;
let _21: ();
let _22: ();
{
RET = !true;
RET = false;
RET = !false;
Call(RET = fn10(Move(_1), _2, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 6967_u16 - 61505_u16;
_2 = 8_u16;
RET = false & true;
_2 = 23276_u16;
_2 = !42131_u16;
RET = _2 >= _2;
RET = false | true;
_2 = 18716_u16 & 54844_u16;
RET = false;
_2 = 15402_u16;
RET = true;
RET = !true;
RET = !false;
RET = true & true;
RET = _2 != _2;
_2 = 286656286829819055571591841056674743178_u128 as u16;
RET = _2 >= _2;
RET = true | true;
RET = _2 == _2;
RET = false;
_2 = 36125_u16 - 62597_u16;
_2 = (-4860040463771920591_i64) as u16;
RET = !false;
_2 = 33607_u16 >> 1415143665728397882303937557885792616_i128;
_2 = 59376_u16 << (-50_i8);
_5 = 6_usize as f64;
_4 = '\u{a1352}';
RET = true;
Goto(bb2)
}
bb2 = {
_5 = (-4536409048375025814_i64) as f64;
_5 = 26_i8 as f64;
RET = true;
_2 = 54297_u16;
_4 = '\u{c7afd}';
_8 = -91_isize;
_8 = 9223372036854775807_isize;
_5 = (-87764202340059183929844628565197350166_i128) as f64;
_4 = '\u{67e16}';
_5 = (-135625870511627284893803205387648719309_i128) as f64;
_5 = 1163846444_i32 as f64;
_4 = '\u{26f65}';
RET = !false;
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
54297 => bb7,
_ => bb6
}
}
bb3 = {
_2 = 6967_u16 - 61505_u16;
_2 = 8_u16;
RET = false & true;
_2 = 23276_u16;
_2 = !42131_u16;
RET = _2 >= _2;
RET = false | true;
_2 = 18716_u16 & 54844_u16;
RET = false;
_2 = 15402_u16;
RET = true;
RET = !true;
RET = !false;
RET = true & true;
RET = _2 != _2;
_2 = 286656286829819055571591841056674743178_u128 as u16;
RET = _2 >= _2;
RET = true | true;
RET = _2 == _2;
RET = false;
_2 = 36125_u16 - 62597_u16;
_2 = (-4860040463771920591_i64) as u16;
RET = !false;
_2 = 33607_u16 >> 1415143665728397882303937557885792616_i128;
_2 = 59376_u16 << (-50_i8);
_5 = 6_usize as f64;
_4 = '\u{a1352}';
RET = true;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_8 = -(-9223372036854775808_isize);
_10 = _2 * _2;
_10 = !_2;
_10 = _2;
RET = false ^ true;
_10 = _2 ^ _2;
_10 = _2;
_12 = _5;
RET = false;
_5 = _12 * _12;
_5 = 37714233_i32 as f64;
_2 = _10 + _10;
_5 = _12 * _12;
_12 = _5 + _5;
_5 = -_12;
_8 = (-62_isize);
RET = true;
RET = true | true;
_5 = -_12;
_13 = -(-1736967928_i32);
_13 = 99_u8 as i32;
_15 = 207561105080150313684860804228314130246_u128 as f64;
_2 = _10;
_15 = _12 - _5;
Call(_6 = fn14(RET, _2, _4, RET, _8, _4, _2, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _15 + _15;
_16 = (-102931361539130642118596211533246898418_i128) as f32;
_13 = 2110161486_i32;
_13 = !(-1276887499_i32);
_10 = _2;
_17 = 23425_i16 as isize;
_15 = _5;
RET = true ^ false;
_2 = _10 * _10;
_12 = _5;
_2 = !_10;
Goto(bb9)
}
bb9 = {
_15 = _5 * _12;
_4 = '\u{b8449}';
_18 = -_16;
_18 = _16 - _16;
_17 = -_8;
_2 = _10 | _10;
RET = _15 >= _15;
_10 = _2;
_15 = _12 - _5;
_17 = _8;
_10 = !_2;
_17 = -_8;
_15 = _5 + _5;
_8 = _17;
_14 = !_8;
Goto(bb10)
}
bb10 = {
Call(_21 = dump_var(Move(_14), Move(_13), Move(_17), _22), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *mut *const &'static mut f32,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16) -> bool {
mir! {
type RET = bool;
let _6: i32;
let _7: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _8: (Adt26, Adt41);
let _9: (*mut [u16; 5], *const (u64,), *const Adt41);
let _10: i8;
let _11: *const Adt41;
let _12: (*mut &'static mut [char; 6],);
let _13: *const Adt22;
let _14: ();
let _15: ();
{
_3 = !_4;
RET = _3 >= _2;
_3 = 113008179947156534580885395664839380610_i128 as u16;
_6 = 334845931_i32;
_7.2 = -(-116_i8);
_3 = _5 ^ _5;
_7.3.1 = _4 | _3;
_7.3.3 = '\u{f8ecf}';
_3 = !_2;
_3 = _5 | _7.3.1;
_7.3.3 = '\u{b99a9}';
_3 = _7.2 as u16;
RET = !true;
_7.3.0 = _5 + _5;
_2 = !_5;
_8.0.fld3 = 16483_i16 as f64;
_8.0.fld0 = 3411822373_u32 as f32;
_7.2 = (-24_i8);
_3 = (-2797730873467615532_i64) as u16;
_2 = _7.3.0;
_8.0.fld0 = 129452373771089443483040895538397298095_u128 as f32;
_7.3.2 = &_8.0.fld0;
Goto(bb1)
}
bb1 = {
_8.0.fld4 = 13286115730257704169_u64 + 14860039262661678359_u64;
_8.0.fld0 = _7.2 as f32;
_8.0.fld5 = !_3;
Call(_7.1 = fn11(Move(_1), _3, _7.2, Move(_7.3.2), _4, _8.0.fld3, _8.0.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.0.fld4 = 5350372191710618805_u64 * 7864829020582475717_u64;
_7.3.0 = _7.3.1 * _7.3.1;
match _7.2 {
340282366920938463463374607431768211432 => bb3,
_ => bb1
}
}
bb3 = {
_8.0.fld3 = _6 as f64;
_9.2 = core::ptr::addr_of!(_8.1);
RET = !false;
_7.3.0 = !_2;
_4 = _7.3.1 * _5;
match _6 {
0 => bb1,
1 => bb2,
334845931 => bb5,
_ => bb4
}
}
bb4 = {
_8.0.fld4 = 5350372191710618805_u64 * 7864829020582475717_u64;
_7.3.0 = _7.3.1 * _7.3.1;
match _7.2 {
340282366920938463463374607431768211432 => bb3,
_ => bb1
}
}
bb5 = {
_8.0.fld5 = _7.3.3 as u16;
_7.3.2 = &_8.0.fld0;
_8.0.fld2 = [245121111248912074702865834081180182715_u128,111021877913977439270386059189475846605_u128,181033197128966229975668215831422509242_u128,221084962530477193533829345025723085523_u128,234860466038059700732831666749765442407_u128,182880362016363844968401890347432435561_u128,105146743083311912560458526907883154826_u128];
_7.2 = 34_i8 + 70_i8;
_11 = core::ptr::addr_of!(_8.1);
_10 = _7.2;
_9.2 = core::ptr::addr_of!((*_11));
RET = _7.1 != _7.1;
Goto(bb6)
}
bb6 = {
Call(_14 = dump_var(Move(_10), Move(_2), Move(_3), _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: *mut *const &'static mut f32,mut _2: u16,mut _3: i8,mut _4: &'static f32,mut _5: u16,mut _6: f64,mut _7: f32) -> u32 {
mir! {
type RET = u32;
let _8: f32;
let _9: *const u8;
let _10: f32;
let _11: [isize; 6];
let _12: (&'static isize, *const (u64,), i32);
let _13: u32;
let _14: isize;
let _15: u8;
let _16: isize;
let _17: *mut u128;
let _18: isize;
let _19: &'static isize;
let _20: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _21: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _22: u64;
let _23: f32;
let _24: i8;
let _25: i32;
let _26: u16;
let _27: *const u32;
let _28: &'static mut *const u64;
let _29: u128;
let _30: u32;
let _31: bool;
let _32: ();
let _33: ();
{
_4 = &_7;
_6 = _5 as f64;
_7 = 191_u8 as f32;
RET = !1667461593_u32;
_5 = _6 as u16;
_5 = !_2;
RET = _7 as u32;
_7 = 96_isize as f32;
_7 = _6 as f32;
_8 = (-4352942112459759883_i64) as f32;
_5 = _2 << _2;
RET = 83309109_u32 << _5;
RET = !2146267913_u32;
_8 = _7 + _7;
Call(_8 = fn12(Move(_1), Move(_4), _5, _2, _5, _7, _2, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _8 + _8;
_6 = 13355674552972651184_u64 as f64;
_7 = _8;
_6 = 1394877156_i32 as f64;
_5 = _2 >> RET;
_5 = _2 ^ _2;
_6 = 5412939099883756648_usize as f64;
RET = 1173020685_u32;
_2 = 6_usize as u16;
_2 = _5 >> _3;
RET = 2148035490_u32 + 213249517_u32;
_11 = [(-9223372036854775808_isize),69_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_2 = !_5;
_10 = _8 - _8;
_2 = (-3346045529650860427_i64) as u16;
_7 = _8 - _10;
_10 = 248_u8 as f32;
_4 = &_7;
_2 = _5 << RET;
_12.2 = 2144267301_i32 - 1614414754_i32;
RET = 1863652711_u32 * 3332723594_u32;
_13 = 457725760550484157_u64 as u32;
_14 = 57_isize | (-88_isize);
_9 = core::ptr::addr_of!(_15);
(*_9) = 1861255851742887333_i64 as u8;
_3 = 99_i8 >> (*_9);
Goto(bb2)
}
bb2 = {
(*_9) = !240_u8;
_9 = core::ptr::addr_of!((*_9));
_2 = _5;
_14 = -(-9223372036854775808_isize);
(*_9) = 12_u8 & 126_u8;
_11 = [_14,_14,_14,_14,_14,_14];
(*_9) = !103_u8;
_7 = -_8;
_14 = 8_isize >> _13;
_5 = _2 & _2;
_15 = _6 as u8;
(*_9) = 19678_i16 as u8;
_4 = &_10;
_7 = _8;
(*_9) = false as u8;
_15 = 207_u8 + 248_u8;
_18 = !_14;
_9 = core::ptr::addr_of!((*_9));
(*_9) = 175_u8;
(*_9) = 696_i16 as u8;
(*_9) = 84_u8 & 246_u8;
(*_9) = 88_u8 << _14;
Goto(bb3)
}
bb3 = {
(*_9) = 146_u8 ^ 201_u8;
(*_9) = 69_u8;
_12.0 = &_18;
(*_9) = 187_u8 ^ 230_u8;
_21.0.2 = _5;
(*_9) = 184_u8 * 162_u8;
_11 = [_18,_14,_14,_14,_14,_18];
_20.3.0 = (-132405865830363281083822479022531731974_i128) as u16;
_15 = 226_u8 - 141_u8;
_15 = !72_u8;
_21.0.2 = '\u{fd909}' as u16;
_21.1.3 = 24977_i16;
_18 = 1484206551413887469_i64 as isize;
_21.1.0 = _6 + _6;
_21.1.0 = _13 as f64;
_20.3.1 = 174624186717196294175010094432037687621_u128 as u16;
_23 = _8;
_20.3 = (_5, _2, Move(_4), '\u{a46ab}');
_21.0 = (_3, 13802442184419167108_u64, _20.3.0);
(*_9) = 74_u8 - 21_u8;
(*_9) = 56_u8 * 200_u8;
_13 = _8 as u32;
_26 = _14 as u16;
Goto(bb4)
}
bb4 = {
_20.2 = _5 as i8;
(*_9) = 232_u8;
_14 = -_18;
_12.0 = &_14;
(*_9) = _21.0.1 as u8;
_18 = _14 - _14;
(*_9) = _20.3.3 as u8;
RET = _13 << (*_9);
(*_9) = !198_u8;
_20.2 = _3 >> _13;
_3 = _20.2 | _20.2;
(*_9) = !22_u8;
_29 = 248990172174123966256830578429689752275_u128;
Goto(bb5)
}
bb5 = {
Call(_32 = dump_var(Move(_5), Move(_18), Move(_11), Move(_15)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_32 = dump_var(Move(_14), _33, _33, _33), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *mut *const &'static mut f32,mut _2: &'static f32,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: f32,mut _7: u16,mut _8: u32,mut _9: u16) -> f32 {
mir! {
type RET = f32;
let _10: (i8, u64, u16);
let _11: i64;
let _12: &'static mut (u64,);
let _13: isize;
let _14: *const Adt22;
let _15: *const Adt41;
let _16: char;
let _17: *mut [u128; 7];
let _18: &'static isize;
let _19: i32;
let _20: &'static mut u32;
let _21: *const u64;
let _22: *const (u64,);
let _23: ();
let _24: ();
{
_5 = _4 ^ _3;
RET = -_6;
_5 = _3 ^ _3;
_4 = 101_u8 as u16;
_8 = 1048540048709160410_i64 as u32;
_4 = !_5;
_8 = 3270492982_u32 ^ 4081156869_u32;
_10.0 = 67_i8 - (-101_i8);
_2 = &_6;
_7 = 75_u8 as u16;
_5 = true as u16;
_3 = _4 * _5;
_8 = !2764603569_u32;
_2 = &RET;
_4 = _8 as u16;
_11 = (-6936167964484726181_i64) + (-5749401251744473258_i64);
_10.1 = _10.0 as u64;
_10.2 = _3 * _5;
_10.1 = 11586609499961894319_u64 << _3;
_10.2 = !_7;
_10 = (22_i8, 1936195627296999595_u64, _4);
_2 = &_6;
_4 = _10.0 as u16;
Call(_9 = fn13(Move(_1), _10.2, Move(_2)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = (-4250756483475095483_i64) - 4831420145730259888_i64;
_7 = _9;
_3 = _9 << _7;
_13 = (-9223372036854775808_isize) << _7;
_13 = (-60_isize) ^ (-9223372036854775808_isize);
_7 = _3;
_10.1 = 15717039967071766583_u64 ^ 13751755025437397202_u64;
_3 = _9 | _7;
_13 = 9223372036854775807_isize;
RET = (-1803_i16) as f32;
match _10.0 {
0 => bb2,
22 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_5 = (-1995229498_i32) as u16;
_2 = &_6;
_10 = ((-94_i8), 9924784856326026476_u64, _3);
_6 = -RET;
_5 = _3 & _10.2;
_9 = _5 | _7;
_10.2 = !_7;
_10.0 = 87_i8;
_11 = (-280114784488636496_i64);
_13 = '\u{65ba7}' as isize;
_5 = _3;
RET = _6;
_13 = (-9223372036854775808_isize);
_6 = 24908979952722622271513418163866632007_i128 as f32;
Call(_10.2 = core::intrinsics::transmute(_9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = 9223372036854775807_isize * (-9223372036854775808_isize);
_6 = _10.1 as f32;
_10.1 = 17020819340501926434_u64;
_16 = '\u{40598}';
_16 = '\u{c036b}';
_5 = _9 + _3;
RET = _6 - _6;
_3 = _9 & _9;
_7 = _3 * _5;
RET = _6;
_5 = (-106340706097249329116000483871323564759_i128) as u16;
_10 = ((-80_i8), 4588570790270081500_u64, _7);
_16 = '\u{5f804}';
_3 = !_10.2;
_5 = _13 as u16;
_8 = _11 as u32;
_13 = 9223372036854775807_isize;
_3 = !_9;
_6 = _10.0 as f32;
_21 = core::ptr::addr_of!(_10.1);
Goto(bb6)
}
bb6 = {
Call(_23 = dump_var(Move(_3), Move(_9), Move(_13), Move(_4)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_23 = dump_var(Move(_16), _24, _24, _24), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *mut *const &'static mut f32,mut _2: u16,mut _3: &'static f32) -> u16 {
mir! {
type RET = u16;
let _4: &'static mut [char; 6];
let _5: &'static f32;
let _6: *mut [u16; 5];
let _7: (isize, u8, Adt49);
let _8: ();
let _9: ();
{
RET = _2;
_2 = RET + RET;
_2 = RET;
_2 = RET >> RET;
RET = _2 & _2;
RET = _2 << _2;
RET = !_2;
_2 = RET << RET;
RET = _2 << _2;
RET = _2 << _2;
RET = 59_isize as u16;
RET = _2;
RET = _2 << _2;
RET = _2 << _2;
RET = _2 + _2;
_2 = 105617554395221590607363515622700245977_u128 as u16;
RET = _2 + _2;
_2 = !RET;
_2 = RET;
_2 = !RET;
RET = 168_u8 as u16;
_2 = RET;
RET = !_2;
_2 = !RET;
RET = _2 - _2;
RET = _2;
RET = 9676489270336934585_usize as u16;
_2 = RET ^ RET;
RET = _2 - _2;
Call(RET = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !_2;
_2 = RET + RET;
RET = _2;
_2 = !RET;
_2 = RET ^ RET;
_2 = RET;
_2 = RET - RET;
RET = !_2;
_2 = RET * RET;
_2 = RET << RET;
_2 = RET;
_2 = RET;
RET = _2 >> _2;
_7.2.fld0 = 2393034472999625940_u64 - 5692498011555387067_u64;
_2 = RET;
_2 = RET | RET;
RET = _2;
_7.0 = -(-9223372036854775808_isize);
_7.2.fld1 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
Goto(bb2)
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: u16,mut _3: char,mut _4: bool,mut _5: isize,mut _6: char,mut _7: u16,mut _8: char) -> *const (u64,) {
mir! {
type RET = *const (u64,);
let _9: Adt26;
let _10: bool;
let _11: u32;
let _12: *const u64;
let _13: *mut *const &'static mut f32;
let _14: &'static mut *const u64;
let _15: [u32; 1];
let _16: u128;
let _17: isize;
let _18: &'static mut f32;
let _19: &'static mut &'static mut f32;
let _20: f32;
let _21: [u128; 7];
let _22: &'static mut [char; 6];
let _23: *const *const (u64,);
let _24: i8;
let _25: &'static mut f32;
let _26: f64;
let _27: Adt49;
let _28: *const Adt41;
let _29: &'static mut *mut i128;
let _30: bool;
let _31: *const *const (u64,);
let _32: &'static mut &'static mut f32;
let _33: &'static &'static mut *const u64;
let _34: f64;
let _35: char;
let _36: u128;
let _37: i64;
let _38: &'static mut f32;
let _39: &'static mut *const u64;
let _40: *mut &'static mut [char; 6];
let _41: (isize, u8, Adt49);
let _42: [u16; 5];
let _43: &'static mut f64;
let _44: *const *const (u64,);
let _45: f64;
let _46: bool;
let _47: &'static isize;
let _48: isize;
let _49: &'static mut [char; 6];
let _50: u64;
let _51: char;
let _52: u16;
let _53: *mut *const &'static mut f32;
let _54: i8;
let _55: u64;
let _56: isize;
let _57: f64;
let _58: bool;
let _59: *mut [u128; 7];
let _60: &'static mut [char; 6];
let _61: isize;
let _62: isize;
let _63: [u128; 7];
let _64: Adt78;
let _65: f32;
let _66: isize;
let _67: char;
let _68: usize;
let _69: *const &'static mut f32;
let _70: ();
let _71: ();
{
_5 = -6_isize;
_3 = _8;
_5 = -9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb2 = {
_9.fld0 = _2 as f32;
(*RET) = (14535503994638729837_u64,);
(*RET) = (6523360201099244738_u64,);
_5 = (-23_isize) & (-9223372036854775808_isize);
(*RET).0 = 11499854283726628082_u64 & 10003489300332763594_u64;
(*RET) = (3482249185071058011_u64,);
(*RET) = (18188309345226604164_u64,);
match (*RET).0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
18188309345226604164 => bb11,
_ => bb10
}
}
bb3 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_12 = core::ptr::addr_of!((*RET).0);
(*_12) = 125104007051990630444410373549143577652_u128 as u64;
_3 = _8;
(*RET) = (11904413518022033928_u64,);
_9.fld4 = (*RET).0;
_9.fld1.0 = !_9.fld4;
(*RET).0 = _9.fld4 - _9.fld4;
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4;
(*RET).0 = _9.fld4;
_15 = [2633081672_u32];
(*RET).0 = !_9.fld4;
(*RET).0 = !_9.fld4;
Call(_17 = core::intrinsics::transmute((*RET).0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*RET) = (_9.fld4,);
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4 + _9.fld4;
_16 = 278619266316922653444743255029762809942_u128 | 94160854039734464133980901688656122376_u128;
(*RET).0 = !_9.fld4;
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4;
(*RET) = (_9.fld4,);
_10 = _1 | _1;
(*RET).0 = _9.fld4 ^ _9.fld4;
_9.fld3 = (-4828467076342084794_i64) as f64;
(*RET).0 = !_9.fld4;
_9.fld5 = 29_i8 as u16;
_17 = _5 & _5;
_11 = 1193261306_u32 << _5;
(*RET) = (_9.fld4,);
_2 = _9.fld5 * _7;
_18 = &mut _9.fld0;
_4 = !_1;
_7 = _2;
match (*RET).0 {
0 => bb6,
11904413518022033928 => bb14,
_ => bb13
}
}
bb13 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb14 = {
(*RET).0 = 8823900142483638197_u64;
(*RET).0 = 146388624929116931820246808499771399571_i128 as u64;
(*_18) = (-6542960979436747551_i64) as f32;
(*RET).0 = !14939768380091057584_u64;
_21 = [_16,_16,_16,_16,_16,_16,_16];
_20 = 67_i8 as f32;
(*_18) = -_20;
(*RET) = (3940865918497275039_u64,);
(*RET).0 = 6797605898050745969_u64 + 1745875966625512534_u64;
(*RET) = (4587515403769815771_u64,);
_11 = _8 as u32;
(*RET) = (7194455444329633506_u64,);
_5 = _17;
Goto(bb15)
}
bb15 = {
_24 = (-114_i8) - 25_i8;
(*RET) = (3121138795184110181_u64,);
(*RET).0 = 3678989019071692706_u64 | 5624167482402296898_u64;
_12 = core::ptr::addr_of!((*RET).0);
(*RET) = (14881386600335846550_u64,);
(*RET) = (8087294624692404070_u64,);
_27.fld0 = (*RET).0;
_15 = [_11];
(*_18) = _20;
(*RET).0 = _27.fld0;
Goto(bb16)
}
bb16 = {
(*RET) = (_27.fld0,);
_2 = _24 as u16;
_3 = _6;
(*RET).0 = _27.fld0 & _27.fld0;
_23 = core::ptr::addr_of!(RET);
_2 = _7 << (*RET).0;
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*RET).0 = !_27.fld0;
(*_18) = 5372972152250487687_i64 as f32;
(*_18) = -_20;
(*_18) = _20 + _20;
(*RET).0 = _27.fld0 % _27.fld0;
_31 = core::ptr::addr_of!((*_23));
(*RET).0 = !_27.fld0;
_1 = (*RET).0 > (*RET).0;
(*_31) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*_18) = _16 as f32;
(*_23) = core::ptr::addr_of!((*RET));
_17 = _5;
(*RET).0 = _27.fld0;
(*_18) = 6369_i16 as f32;
match (*RET).0 {
8087294624692404070 => bb18,
_ => bb17
}
}
bb17 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb18 = {
(*_18) = -_20;
(*RET) = (_27.fld0,);
_15 = [_11];
(*RET) = (_27.fld0,);
_24 = !(-85_i8);
(*_18) = -_20;
(*_23) = core::ptr::addr_of!((*RET));
_37 = 3945676991056523406_i64 & 4760506443579084920_i64;
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = 13162440541049281421_usize as u64;
(*RET) = (_27.fld0,);
_20 = -(*_18);
(*_23) = core::ptr::addr_of!((*RET));
_1 = (*RET).0 >= (*RET).0;
(*RET).0 = _27.fld0 ^ _27.fld0;
match _27.fld0 {
0 => bb7,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
8087294624692404070 => bb26,
_ => bb25
}
}
bb19 = {
Return()
}
bb20 = {
(*RET) = (_27.fld0,);
_2 = _24 as u16;
_3 = _6;
(*RET).0 = _27.fld0 & _27.fld0;
_23 = core::ptr::addr_of!(RET);
_2 = _7 << (*RET).0;
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*RET).0 = !_27.fld0;
(*_18) = 5372972152250487687_i64 as f32;
(*_18) = -_20;
(*_18) = _20 + _20;
(*RET).0 = _27.fld0 % _27.fld0;
_31 = core::ptr::addr_of!((*_23));
(*RET).0 = !_27.fld0;
_1 = (*RET).0 > (*RET).0;
(*_31) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*_18) = _16 as f32;
(*_23) = core::ptr::addr_of!((*RET));
_17 = _5;
(*RET).0 = _27.fld0;
(*_18) = 6369_i16 as f32;
match (*RET).0 {
8087294624692404070 => bb18,
_ => bb17
}
}
bb21 = {
_24 = (-114_i8) - 25_i8;
(*RET) = (3121138795184110181_u64,);
(*RET).0 = 3678989019071692706_u64 | 5624167482402296898_u64;
_12 = core::ptr::addr_of!((*RET).0);
(*RET) = (14881386600335846550_u64,);
(*RET) = (8087294624692404070_u64,);
_27.fld0 = (*RET).0;
_15 = [_11];
(*_18) = _20;
(*RET).0 = _27.fld0;
Goto(bb16)
}
bb22 = {
(*RET).0 = 8823900142483638197_u64;
(*RET).0 = 146388624929116931820246808499771399571_i128 as u64;
(*_18) = (-6542960979436747551_i64) as f32;
(*RET).0 = !14939768380091057584_u64;
_21 = [_16,_16,_16,_16,_16,_16,_16];
_20 = 67_i8 as f32;
(*_18) = -_20;
(*RET) = (3940865918497275039_u64,);
(*RET).0 = 6797605898050745969_u64 + 1745875966625512534_u64;
(*RET) = (4587515403769815771_u64,);
_11 = _8 as u32;
(*RET) = (7194455444329633506_u64,);
_5 = _17;
Goto(bb15)
}
bb23 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb24 = {
Return()
}
bb25 = {
_9.fld0 = _2 as f32;
(*RET) = (14535503994638729837_u64,);
(*RET) = (6523360201099244738_u64,);
_5 = (-23_isize) & (-9223372036854775808_isize);
(*RET).0 = 11499854283726628082_u64 & 10003489300332763594_u64;
(*RET) = (3482249185071058011_u64,);
(*RET) = (18188309345226604164_u64,);
match (*RET).0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
18188309345226604164 => bb11,
_ => bb10
}
}
bb26 = {
(*RET) = (_27.fld0,);
(*RET) = (_27.fld0,);
_1 = !_10;
_21 = [_16,_16,_16,_16,_16,_16,_16];
(*RET) = (_27.fld0,);
(*RET).0 = _1 as u64;
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = _27.fld0 | _27.fld0;
(*_18) = -_20;
_14 = &mut _12;
(*RET).0 = !_27.fld0;
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = _3 as u64;
(*_23) = core::ptr::addr_of!((*RET));
_10 = _4;
(*RET) = (_27.fld0,);
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*RET).0 = _27.fld0;
(*RET) = (_27.fld0,);
(*RET).0 = _27.fld0 & _27.fld0;
(*_18) = _2 as f32;
(*_14) = core::ptr::addr_of!((*RET).0);
(*RET).0 = _27.fld0 / _27.fld0;
match _27.fld0 {
0 => bb23,
1 => bb2,
2 => bb18,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb12,
8087294624692404070 => bb28,
_ => bb27
}
}
bb27 = {
_24 = (-114_i8) - 25_i8;
(*RET) = (3121138795184110181_u64,);
(*RET).0 = 3678989019071692706_u64 | 5624167482402296898_u64;
_12 = core::ptr::addr_of!((*RET).0);
(*RET) = (14881386600335846550_u64,);
(*RET) = (8087294624692404070_u64,);
_27.fld0 = (*RET).0;
_15 = [_11];
(*_18) = _20;
(*RET).0 = _27.fld0;
Goto(bb16)
}
bb28 = {
_47 = &_5;
(*_14) = core::ptr::addr_of!((*RET).0);
_33 = &_14;
(*_14) = core::ptr::addr_of!((*RET).0);
(*RET).0 = !_27.fld0;
(*RET).0 = _27.fld0 >> _2;
_41.0 = !(*_47);
(*_18) = -_20;
(*RET).0 = _4 as u64;
_32 = &mut _18;
RET = core::ptr::addr_of!((*RET));
(*_32) = &mut _20;
_37 = !(-1984458231183837058_i64);
Goto(bb29)
}
bb29 = {
(*RET).0 = _27.fld0 << _16;
(*_14) = core::ptr::addr_of!((*RET).0);
(*_23) = core::ptr::addr_of!((*RET));
_4 = !_10;
(*RET) = (_27.fld0,);
_2 = !_7;
_34 = (-22437_i16) as f64;
_42 = [_7,_7,_7,_7,_7];
_44 = core::ptr::addr_of!((*_23));
(*_23) = core::ptr::addr_of!((*RET));
_41.1 = 195_u8 | 38_u8;
_8 = _3;
(*RET).0 = _27.fld0;
(*_23) = core::ptr::addr_of!((*RET));
(*_14) = core::ptr::addr_of!((*RET).0);
_39 = Move(_14);
(*RET) = (_27.fld0,);
(*RET).0 = !_27.fld0;
_30 = _41.0 != (*_47);
(*RET).0 = _8 as u64;
_4 = _10 | _1;
_27.fld0 = (-7938_i16) as u64;
(*RET) = (_27.fld0,);
(*RET) = (_27.fld0,);
_35 = _6;
(*RET) = (_27.fld0,);
Goto(bb30)
}
bb30 = {
(*RET).0 = _27.fld0;
Goto(bb31)
}
bb31 = {
_48 = (*_47);
_47 = &_41.0;
(*_23) = core::ptr::addr_of!((*RET));
_24 = -102_i8;
(*_23) = core::ptr::addr_of!((*RET));
_45 = _24 as f64;
(*RET) = (_27.fld0,);
_25 = Move((*_32));
_15 = [_11];
_42 = [_7,_7,_2,_2,_2];
_5 = -(*_47);
_41.2.fld0 = !(*RET).0;
_52 = _7 & _7;
_36 = _16;
_8 = _35;
(*RET).0 = !_27.fld0;
_3 = _8;
_36 = _16;
_5 = (*_47);
(*RET) = (_27.fld0,);
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = !_41.2.fld0;
Goto(bb32)
}
bb32 = {
_41.0 = _48 - _48;
_30 = !_1;
Goto(bb33)
}
bb33 = {
_35 = _3;
_43 = &mut _34;
_46 = _4 != _30;
_42 = [_52,_52,_52,_2,_2];
(*RET) = (_41.2.fld0,);
_2 = _52 | _52;
_27.fld1 = [_41.0,_17,_5,_41.0,_41.0,_48];
_6 = _8;
(*RET).0 = _27.fld0 - _27.fld0;
(*RET) = (_41.2.fld0,);
_23 = core::ptr::addr_of!((*_23));
_6 = _8;
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = _41.2.fld0;
Goto(bb34)
}
bb34 = {
(*RET) = (_41.2.fld0,);
_59 = core::ptr::addr_of_mut!(_21);
_17 = !_48;
(*_43) = 8639588901133042507_usize as f64;
_30 = !_46;
_16 = !_36;
_41 = (_5, 139_u8, Move(_27));
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = _41.2.fld0;
_52 = _30 as u16;
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
match _41.1 {
0 => bb33,
1 => bb8,
2 => bb35,
3 => bb36,
4 => bb37,
139 => bb39,
_ => bb38
}
}
bb35 = {
Return()
}
bb36 = {
(*RET) = (_27.fld0,);
_2 = _24 as u16;
_3 = _6;
(*RET).0 = _27.fld0 & _27.fld0;
_23 = core::ptr::addr_of!(RET);
_2 = _7 << (*RET).0;
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*RET).0 = !_27.fld0;
(*_18) = 5372972152250487687_i64 as f32;
(*_18) = -_20;
(*_18) = _20 + _20;
(*RET).0 = _27.fld0 % _27.fld0;
_31 = core::ptr::addr_of!((*_23));
(*RET).0 = !_27.fld0;
_1 = (*RET).0 > (*RET).0;
(*_31) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*_18) = _16 as f32;
(*_23) = core::ptr::addr_of!((*RET));
_17 = _5;
(*RET).0 = _27.fld0;
(*_18) = 6369_i16 as f32;
match (*RET).0 {
8087294624692404070 => bb18,
_ => bb17
}
}
bb37 = {
_9.fld0 = _2 as f32;
(*RET) = (14535503994638729837_u64,);
(*RET) = (6523360201099244738_u64,);
_5 = (-23_isize) & (-9223372036854775808_isize);
(*RET).0 = 11499854283726628082_u64 & 10003489300332763594_u64;
(*RET) = (3482249185071058011_u64,);
(*RET) = (18188309345226604164_u64,);
match (*RET).0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
18188309345226604164 => bb11,
_ => bb10
}
}
bb38 = {
_12 = core::ptr::addr_of!((*RET).0);
(*_12) = 125104007051990630444410373549143577652_u128 as u64;
_3 = _8;
(*RET) = (11904413518022033928_u64,);
_9.fld4 = (*RET).0;
_9.fld1.0 = !_9.fld4;
(*RET).0 = _9.fld4 - _9.fld4;
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4;
(*RET).0 = _9.fld4;
_15 = [2633081672_u32];
(*RET).0 = !_9.fld4;
(*RET).0 = !_9.fld4;
Call(_17 = core::intrinsics::transmute((*RET).0), ReturnTo(bb12), UnwindUnreachable())
}
bb39 = {
(*RET) = (_41.2.fld0,);
(*_43) = _45 + _45;
match _41.1 {
0 => bb5,
1 => bb15,
2 => bb40,
3 => bb41,
4 => bb42,
139 => bb44,
_ => bb43
}
}
bb40 = {
_12 = core::ptr::addr_of!((*RET).0);
(*_12) = 125104007051990630444410373549143577652_u128 as u64;
_3 = _8;
(*RET) = (11904413518022033928_u64,);
_9.fld4 = (*RET).0;
_9.fld1.0 = !_9.fld4;
(*RET).0 = _9.fld4 - _9.fld4;
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4;
(*RET).0 = _9.fld4;
_15 = [2633081672_u32];
(*RET).0 = !_9.fld4;
(*RET).0 = !_9.fld4;
Call(_17 = core::intrinsics::transmute((*RET).0), ReturnTo(bb12), UnwindUnreachable())
}
bb41 = {
Return()
}
bb42 = {
(*RET) = (_27.fld0,);
_2 = _24 as u16;
_3 = _6;
(*RET).0 = _27.fld0 & _27.fld0;
_23 = core::ptr::addr_of!(RET);
_2 = _7 << (*RET).0;
(*_23) = core::ptr::addr_of!((*RET));
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*RET).0 = !_27.fld0;
(*_18) = 5372972152250487687_i64 as f32;
(*_18) = -_20;
(*_18) = _20 + _20;
(*RET).0 = _27.fld0 % _27.fld0;
_31 = core::ptr::addr_of!((*_23));
(*RET).0 = !_27.fld0;
_1 = (*RET).0 > (*RET).0;
(*_31) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
(*_18) = _16 as f32;
(*_23) = core::ptr::addr_of!((*RET));
_17 = _5;
(*RET).0 = _27.fld0;
(*_18) = 6369_i16 as f32;
match (*RET).0 {
8087294624692404070 => bb18,
_ => bb17
}
}
bb43 = {
_48 = (*_47);
_47 = &_41.0;
(*_23) = core::ptr::addr_of!((*RET));
_24 = -102_i8;
(*_23) = core::ptr::addr_of!((*RET));
_45 = _24 as f64;
(*RET) = (_27.fld0,);
_25 = Move((*_32));
_15 = [_11];
_42 = [_7,_7,_2,_2,_2];
_5 = -(*_47);
_41.2.fld0 = !(*RET).0;
_52 = _7 & _7;
_36 = _16;
_8 = _35;
(*RET).0 = !_27.fld0;
_3 = _8;
_36 = _16;
_5 = (*_47);
(*RET) = (_27.fld0,);
(*_23) = core::ptr::addr_of!((*RET));
(*RET).0 = !_41.2.fld0;
Goto(bb32)
}
bb44 = {
(*_23) = core::ptr::addr_of!((*RET));
_59 = core::ptr::addr_of_mut!(_21);
(*_59) = [_36,_36,_36,_36,_36,_36,_36];
_7 = !_2;
_7 = _52 + _2;
(*RET).0 = _24 as u64;
match _41.1 {
0 => bb45,
1 => bb46,
2 => bb47,
3 => bb48,
4 => bb49,
139 => bb51,
_ => bb50
}
}
bb45 = {
Return()
}
bb46 = {
_4 = !_1;
_9.fld1.0 = 3645732961842706584_u64 | 15734727864831856742_u64;
RET = core::ptr::addr_of!(_9.fld1);
_3 = _6;
(*RET) = (4431649077563224288_u64,);
(*RET) = (15518100613130635098_u64,);
(*RET).0 = 13063739534803240462_u64;
Goto(bb2)
}
bb47 = {
_24 = (-114_i8) - 25_i8;
(*RET) = (3121138795184110181_u64,);
(*RET).0 = 3678989019071692706_u64 | 5624167482402296898_u64;
_12 = core::ptr::addr_of!((*RET).0);
(*RET) = (14881386600335846550_u64,);
(*RET) = (8087294624692404070_u64,);
_27.fld0 = (*RET).0;
_15 = [_11];
(*_18) = _20;
(*RET).0 = _27.fld0;
Goto(bb16)
}
bb48 = {
_12 = core::ptr::addr_of!((*RET).0);
(*_12) = 125104007051990630444410373549143577652_u128 as u64;
_3 = _8;
(*RET) = (11904413518022033928_u64,);
_9.fld4 = (*RET).0;
_9.fld1.0 = !_9.fld4;
(*RET).0 = _9.fld4 - _9.fld4;
(*RET) = (_9.fld4,);
(*RET).0 = _9.fld4;
(*RET).0 = _9.fld4;
_15 = [2633081672_u32];
(*RET).0 = !_9.fld4;
(*RET).0 = !_9.fld4;
Call(_17 = core::intrinsics::transmute((*RET).0), ReturnTo(bb12), UnwindUnreachable())
}
bb49 = {
Return()
}
bb50 = {
Return()
}
bb51 = {
_56 = _48;
(*_43) = _45 + _45;
(*RET) = (_41.2.fld0,);
(*_23) = core::ptr::addr_of!((*RET));
_57 = (*_43);
_27.fld0 = (*RET).0;
(*RET).0 = _41.2.fld0;
_66 = _17 | _41.0;
(*_23) = core::ptr::addr_of!((*RET));
(*RET) = (_27.fld0,);
_68 = 4_usize;
_8 = _6;
(*RET) = (_41.2.fld0,);
(*_59) = [_16,_36,_36,_36,_16,_16,_16];
_41.2.fld1 = [_56,_56,_41.0,_5,_17,_41.0];
Goto(bb52)
}
bb52 = {
Call(_70 = dump_var(Move(_35), Move(_6), Move(_42), Move(_7)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_70 = dump_var(Move(_56), Move(_3), Move(_24), Move(_16)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_70 = dump_var(Move(_17), Move(_52), Move(_10), Move(_21)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_70 = dump_var(Move(_37), _71, _71, _71), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *mut *const &'static mut f32) -> isize {
mir! {
type RET = isize;
let _2: Adt53;
let _3: &'static mut f32;
let _4: &'static mut [char; 6];
let _5: *mut &'static mut [char; 6];
let _6: char;
let _7: [u16; 5];
let _8: *const Adt41;
let _9: &'static mut &'static mut f32;
let _10: [isize; 6];
let _11: i16;
let _12: [char; 5];
let _13: isize;
let _14: &'static mut f64;
let _15: i16;
let _16: i32;
let _17: u8;
let _18: Adt80;
let _19: *mut i128;
let _20: [u32; 1];
let _21: char;
let _22: f64;
let _23: &'static mut f32;
let _24: Adt78;
let _25: u128;
let _26: (*mut &'static mut [char; 6],);
let _27: u128;
let _28: *const &'static mut f32;
let _29: (&'static mut *const u64, u32, i8, (u16, u16, &'static f32, char));
let _30: u32;
let _31: *mut u128;
let _32: Adt78;
let _33: (&'static f32, *mut i128);
let _34: char;
let _35: *mut [u128; 7];
let _36: char;
let _37: &'static &'static mut &'static mut f32;
let _38: [u128; 7];
let _39: isize;
let _40: Adt41;
let _41: *mut i16;
let _42: isize;
let _43: [u32; 1];
let _44: u32;
let _45: Adt49;
let _46: &'static mut *mut i128;
let _47: f32;
let _48: *mut *const &'static mut f32;
let _49: isize;
let _50: *mut *const &'static mut f32;
let _51: (*mut &'static mut [char; 6],);
let _52: bool;
let _53: f64;
let _54: usize;
let _55: f64;
let _56: usize;
let _57: Adt22;
let _58: *mut i128;
let _59: u128;
let _60: &'static mut [char; 6];
let _61: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _62: *mut i128;
let _63: *const u64;
let _64: [i64; 6];
let _65: (Adt26, Adt41);
let _66: &'static mut &'static mut f32;
let _67: char;
let _68: Adt41;
let _69: *const &'static mut f32;
let _70: char;
let _71: *const u8;
let _72: u8;
let _73: i128;
let _74: isize;
let _75: isize;
let _76: [u16; 5];
let _77: *const u64;
let _78: Adt80;
let _79: f32;
let _80: isize;
let _81: [char; 5];
let _82: (u16, u16, &'static f32, char);
let _83: &'static mut (u64,);
let _84: *const Adt22;
let _85: f32;
let _86: ();
let _87: ();
{
_2.fld1 = 1474119928_u32 as f64;
RET = (-13_isize) + 47_isize;
_2.fld1 = 8721887914688781835_usize as f64;
RET = (-119_isize) << 123566757195916988470620370082347082050_i128;
_2.fld1 = 80566010829557364320170855975987631089_i128 as f64;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
RET = !9223372036854775807_isize;
RET = 9223372036854775807_isize & (-67_isize);
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_2.fld1 = 1855237237749333259_i64 as f64;
_2.fld1 = 97_i8 as f64;
RET = -(-9223372036854775808_isize);
_2.fld1 = 2758330822_u32 as f64;
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_2.fld1 = 13839_u16 as f64;
RET = -9223372036854775807_isize;
_2.fld1 = 7512556359543090374_u64 as f64;
_7 = [1903_u16,12783_u16,45879_u16,38619_u16,39185_u16];
_7 = [64079_u16,12790_u16,9596_u16,18352_u16,33607_u16];
Goto(bb1)
}
bb1 = {
_6 = '\u{2192b}';
_7 = [6909_u16,50016_u16,64926_u16,24667_u16,18123_u16];
_7 = [3025_u16,54737_u16,21260_u16,13733_u16,44806_u16];
RET = (-22_isize);
_6 = '\u{3e8ab}';
RET = (-9223372036854775808_isize) << (-15174_i16);
_7 = [6413_u16,31563_u16,26018_u16,3410_u16,22569_u16];
_6 = '\u{ee1a4}';
_6 = '\u{5dbeb}';
_7 = [28717_u16,19482_u16,23876_u16,12673_u16,32102_u16];
RET = (-126_isize);
_2.fld1 = 34586272850893137037267424881081082761_u128 as f64;
_2.fld1 = RET as f64;
_7 = [31133_u16,10302_u16,53138_u16,49021_u16,60547_u16];
_6 = '\u{e132a}';
RET = -(-64_isize);
_7 = [10890_u16,52692_u16,7740_u16,33335_u16,14546_u16];
Goto(bb2)
}
bb2 = {
_2.fld1 = RET as f64;
RET = -(-9223372036854775808_isize);
RET = -9223372036854775807_isize;
_6 = '\u{1032f2}';
_7 = [18535_u16,44106_u16,4954_u16,58927_u16,27168_u16];
_7 = [26891_u16,1069_u16,471_u16,5941_u16,59968_u16];
_6 = '\u{d4301}';
_7 = [36468_u16,17107_u16,32455_u16,57140_u16,44442_u16];
_10 = [RET,RET,RET,RET,RET,RET];
_6 = '\u{20ab0}';
_7 = [48709_u16,61454_u16,5555_u16,50052_u16,25941_u16];
_7 = [62430_u16,61130_u16,1852_u16,54507_u16,62135_u16];
_6 = '\u{fc58}';
_7 = [6238_u16,54038_u16,58673_u16,39097_u16,41182_u16];
_7 = [41931_u16,42536_u16,7060_u16,64377_u16,30469_u16];
_6 = '\u{dd34c}';
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_2.fld1 = 1859600846_i32 as f64;
_7 = [44668_u16,45779_u16,30577_u16,58122_u16,11157_u16];
_11 = (-6883_i16) << RET;
_2.fld1 = 1923817442901819269_i64 as f64;
_11 = -(-31155_i16);
_11 = -(-31796_i16);
_6 = '\u{dd9e6}';
_2.fld1 = (-2092286170_i32) as f64;
_10 = [RET,RET,RET,RET,RET,RET];
RET = 9223372036854775807_isize;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
_6 = '\u{2192b}';
_7 = [6909_u16,50016_u16,64926_u16,24667_u16,18123_u16];
_7 = [3025_u16,54737_u16,21260_u16,13733_u16,44806_u16];
RET = (-22_isize);
_6 = '\u{3e8ab}';
RET = (-9223372036854775808_isize) << (-15174_i16);
_7 = [6413_u16,31563_u16,26018_u16,3410_u16,22569_u16];
_6 = '\u{ee1a4}';
_6 = '\u{5dbeb}';
_7 = [28717_u16,19482_u16,23876_u16,12673_u16,32102_u16];
RET = (-126_isize);
_2.fld1 = 34586272850893137037267424881081082761_u128 as f64;
_2.fld1 = RET as f64;
_7 = [31133_u16,10302_u16,53138_u16,49021_u16,60547_u16];
_6 = '\u{e132a}';
RET = -(-64_isize);
_7 = [10890_u16,52692_u16,7740_u16,33335_u16,14546_u16];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_12 = [_6,_6,_6,_6,_6];
_13 = RET - RET;
_12 = [_6,_6,_6,_6,_6];
_7 = [51487_u16,47826_u16,32049_u16,40209_u16,30527_u16];
_2.fld1 = _13 as f64;
RET = _13 * _13;
_13 = !RET;
_2.fld1 = 0_i8 as f64;
_10 = [_13,_13,RET,RET,RET,_13];
_10 = [_13,RET,RET,RET,_13,_13];
_12 = [_6,_6,_6,_6,_6];
_2.fld1 = RET as f64;
_13 = false as isize;
_13 = _11 as isize;
_14 = &mut _2.fld1;
(*_14) = 55_i8 as f64;
_11 = 24673_i16 + 29688_i16;
Goto(bb7)
}
bb7 = {
(*_14) = 0_usize as f64;
_16 = (-1264277733_i32) & (-1546452089_i32);
_16 = 794569776_i32 + 54744724_i32;
(*_14) = 75_i8 as f64;
RET = _13 * _13;
(*_14) = 394483794125372850385652374764238500_i128 as f64;
_16 = (-313232231_i32) & (-1610735946_i32);
(*_14) = 1405486434539072969_i64 as f64;
_17 = 244_u8 << _11;
_13 = !RET;
_11 = (-8572_i16);
(*_14) = (-48_i8) as f64;
(*_14) = _17 as f64;
(*_14) = _17 as f64;
_7 = [26042_u16,12208_u16,11033_u16,54333_u16,18949_u16];
_15 = _11 - _11;
_7 = [38194_u16,36531_u16,26615_u16,2134_u16,33791_u16];
(*_14) = 0_usize as f64;
(*_14) = 7158700746250787441_u64 as f64;
_11 = _15;
_15 = 7188317054757652613_i64 as i16;
Goto(bb8)
}
bb8 = {
(*_14) = 7_usize as f64;
_6 = '\u{1b824}';
(*_14) = _13 as f64;
(*_14) = 67480210425371876121412565431832272625_i128 as f64;
(*_14) = _17 as f64;
(*_14) = 23333_u16 as f64;
(*_14) = _11 as f64;
(*_14) = 3_usize as f64;
(*_14) = 12811719948253042908_u64 as f64;
Goto(bb9)
}
bb9 = {
(*_14) = (-111_i8) as f64;
_21 = _6;
(*_14) = _16 as f64;
(*_14) = _17 as f64;
(*_14) = 509388813058656369_i64 as f64;
(*_14) = _16 as f64;
_17 = 32_u8;
(*_14) = 187194762050466182953913505265011281841_u128 as f64;
(*_14) = (-6159620663145065238_i64) as f64;
_15 = _11 ^ _11;
(*_14) = 8593869503484613414377104187155439174_u128 as f64;
_20 = [2514233324_u32];
(*_14) = _17 as f64;
_22 = _15 as f64;
(*_14) = -_22;
Goto(bb10)
}
bb10 = {
(*_14) = -_22;
(*_14) = -_22;
_16 = (-574616859_i32) | 587179564_i32;
_16 = false as i32;
(*_14) = -_22;
(*_14) = _22 - _22;
(*_14) = _22;
(*_14) = _22;
_15 = _11 * _11;
(*_14) = _22 * _22;
(*_14) = -_22;
(*_14) = _22;
(*_14) = -_22;
Goto(bb11)
}
bb11 = {
(*_14) = _22;
_21 = _6;
_6 = _21;
(*_14) = _22;
(*_14) = _22 * _22;
(*_14) = _22 - _22;
_17 = !213_u8;
_25 = !336152456769583345793048374781323608914_u128;
_12 = [_21,_21,_6,_21,_6];
(*_14) = -_22;
(*_14) = _22 - _22;
_21 = _6;
_25 = 294115918905758423565702047793719848904_u128 - 93346564971624152869042834608778654897_u128;
_13 = _21 as isize;
(*_14) = _22 * _22;
_6 = _21;
_13 = RET;
_22 = -(*_14);
_1 = core::ptr::addr_of_mut!(_28);
Goto(bb12)
}
bb12 = {
_17 = 44_u8;
_31 = core::ptr::addr_of_mut!(_27);
_15 = _11 * _11;
_20 = [4039564848_u32];
(*_31) = _6 as u128;
(*_31) = (-20806264899584389949828246407424166812_i128) as u128;
_29.3.3 = _6;
_17 = 158_u8;
(*_14) = _22;
_27 = _25 | _25;
(*_14) = _22 * _22;
(*_14) = _22 - _22;
_29.2 = 22_i8 * (-109_i8);
_27 = _25 ^ _25;
_29.1 = 3981475970_u32;
_13 = RET;
match _17 {
158 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
(*_31) = !_25;
_25 = (*_31) << (*_31);
_6 = _29.3.3;
(*_31) = _25 ^ _25;
(*_14) = _22 - _22;
(*_14) = _22 - _22;
_29.3.1 = !42658_u16;
(*_14) = _22 - _22;
Goto(bb15)
}
bb15 = {
(*_31) = _29.1 as u128;
(*_31) = _25 << _25;
_15 = _11;
_36 = _21;
(*_31) = _25 >> _29.2;
_29.3.0 = _29.3.1 + _29.3.1;
(*_14) = -_22;
_10 = [_13,RET,RET,_13,RET,_13];
(*_14) = _22;
_21 = _36;
(*_14) = 9280816154668904232_u64 as f64;
(*_31) = _25 | _25;
(*_31) = _25 * _25;
_27 = !_25;
Call((*_14) = fn16(Move(_1), Move(_31), (*_31), _29.3.0, _29.3.3), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_30 = _16 as u32;
_22 = (*_14) - (*_14);
_34 = _36;
_27 = _25 << _29.3.0;
_29.2 = (-79_i8) ^ 118_i8;
(*_14) = _22 * _22;
_10 = [_13,_13,RET,_13,RET,RET];
(*_14) = _22 * _22;
_31 = core::ptr::addr_of_mut!(_27);
(*_31) = _25 ^ _25;
_30 = !_29.1;
(*_14) = _22 - _22;
_15 = _11 << (*_31);
Goto(bb17)
}
bb17 = {
_35 = core::ptr::addr_of_mut!(_38);
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_31) = _25;
(*_14) = _22 * _22;
(*_31) = _25;
_20 = [_30];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),_25];
(*_35) = [(*_31),(*_31),(*_31),(*_31),_27,(*_31),(*_31)];
_1 = core::ptr::addr_of_mut!(_28);
(*_14) = _22;
_12 = [_36,_34,_34,_21,_6];
(*_35) = [(*_31),(*_31),(*_31),_27,(*_31),(*_31),(*_31)];
(*_35) = [_25,(*_31),(*_31),_25,(*_31),(*_31),(*_31)];
(*_14) = _22 - _22;
_36 = _34;
_38 = [(*_31),(*_31),_25,(*_31),(*_31),(*_31),(*_31)];
_31 = core::ptr::addr_of_mut!((*_31));
_22 = -(*_14);
(*_31) = !_25;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_31) = 35450679662502063746437426340451198558_i128 as u128;
_43 = [_30];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),_25];
_8 = core::ptr::addr_of!(_40);
_39 = _13 << _15;
_21 = _34;
Goto(bb18)
}
bb18 = {
(*_31) = _25 | _25;
(*_35) = [(*_31),_25,(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_14) = -_22;
(*_35) = [(*_31),(*_31),(*_31),_27,(*_31),(*_31),(*_31)];
(*_14) = -_22;
_45.fld1 = _10;
(*_31) = _25 + _25;
_45 = Adt49 { fld0: 10849488004210442991_u64,fld1: _10 };
(*_31) = !_25;
(*_31) = !_25;
_41 = core::ptr::addr_of_mut!(_15);
(*_31) = 2_usize as u128;
(*_31) = _25 * _25;
(*_41) = _11 ^ _11;
_16 = 1938536156_i32 - 300018634_i32;
_25 = (*_31);
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_47 = (*_31) as f32;
_45.fld0 = 4688628002940270063_i64 as u64;
(*_41) = _11 | _11;
(*_41) = 3212339791465790050_i64 as i16;
_48 = core::ptr::addr_of_mut!((*_1));
(*_31) = !_25;
(*_31) = _25 >> (*_41);
(*_14) = -_22;
(*_31) = _25;
Goto(bb19)
}
bb19 = {
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_41 = core::ptr::addr_of_mut!((*_41));
(*_14) = _22;
(*_14) = _22;
_6 = _21;
(*_41) = _11 * _11;
match _29.1 {
0 => bb20,
3981475970 => bb22,
_ => bb21
}
}
bb20 = {
_17 = 44_u8;
_31 = core::ptr::addr_of_mut!(_27);
_15 = _11 * _11;
_20 = [4039564848_u32];
(*_31) = _6 as u128;
(*_31) = (-20806264899584389949828246407424166812_i128) as u128;
_29.3.3 = _6;
_17 = 158_u8;
(*_14) = _22;
_27 = _25 | _25;
(*_14) = _22 * _22;
(*_14) = _22 - _22;
_29.2 = 22_i8 * (-109_i8);
_27 = _25 ^ _25;
_29.1 = 3981475970_u32;
_13 = RET;
match _17 {
158 => bb14,
_ => bb13
}
}
bb21 = {
_2.fld1 = RET as f64;
RET = -(-9223372036854775808_isize);
RET = -9223372036854775807_isize;
_6 = '\u{1032f2}';
_7 = [18535_u16,44106_u16,4954_u16,58927_u16,27168_u16];
_7 = [26891_u16,1069_u16,471_u16,5941_u16,59968_u16];
_6 = '\u{d4301}';
_7 = [36468_u16,17107_u16,32455_u16,57140_u16,44442_u16];
_10 = [RET,RET,RET,RET,RET,RET];
_6 = '\u{20ab0}';
_7 = [48709_u16,61454_u16,5555_u16,50052_u16,25941_u16];
_7 = [62430_u16,61130_u16,1852_u16,54507_u16,62135_u16];
_6 = '\u{fc58}';
_7 = [6238_u16,54038_u16,58673_u16,39097_u16,41182_u16];
_7 = [41931_u16,42536_u16,7060_u16,64377_u16,30469_u16];
_6 = '\u{dd34c}';
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_2.fld1 = 1859600846_i32 as f64;
_7 = [44668_u16,45779_u16,30577_u16,58122_u16,11157_u16];
_11 = (-6883_i16) << RET;
_2.fld1 = 1923817442901819269_i64 as f64;
_11 = -(-31155_i16);
_11 = -(-31796_i16);
_6 = '\u{dd9e6}';
_2.fld1 = (-2092286170_i32) as f64;
_10 = [RET,RET,RET,RET,RET,RET];
RET = 9223372036854775807_isize;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb22 = {
_34 = _36;
(*_41) = !_11;
_17 = 99_u8 * 99_u8;
(*_14) = _39 as f64;
_43 = [_29.1];
(*_14) = _22;
_50 = core::ptr::addr_of_mut!((*_1));
Goto(bb23)
}
bb23 = {
_53 = _16 as f64;
(*_31) = _25;
(*_14) = -_22;
_29.3.2 = &_47;
(*_14) = _22 - _22;
(*_31) = !_25;
Call((*_14) = core::intrinsics::transmute(_13), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_29.3.3 = _21;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_14) = _22;
(*_14) = _22;
(*_41) = _11 - _11;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_29.3.1 = _29.3.0 >> _13;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [_27,_25,(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
Goto(bb25)
}
bb25 = {
(*_35) = [(*_31),(*_31),(*_31),(*_31),_27,(*_31),(*_31)];
(*_31) = _25;
(*_41) = _11 & _11;
_49 = (-1076456541242597413_i64) as isize;
(*_14) = -_22;
_15 = _11 * _11;
(*_31) = _25 + _25;
(*_41) = _11 ^ _11;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
match _29.1 {
0 => bb9,
1 => bb13,
2 => bb3,
3981475970 => bb26,
_ => bb11
}
}
bb26 = {
_22 = _29.1 as f64;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),_25,(*_31)];
(*_41) = _11 >> (*_31);
_53 = -(*_14);
_12 = [_34,_21,_29.3.3,_34,_21];
(*_41) = _11;
Goto(bb27)
}
bb27 = {
(*_31) = _25 >> _13;
(*_14) = _53;
_30 = _29.1 + _29.1;
_27 = _25 - _25;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_31) = _47 as u128;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),_25];
Goto(bb28)
}
bb28 = {
(*_31) = !_25;
_53 = -(*_14);
(*_41) = _11 - _11;
_30 = (*_14) as u32;
(*_31) = _25 ^ _25;
(*_14) = -_53;
(*_41) = _11;
(*_14) = 35695147612955060791344871583751984655_i128 as f64;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_11 = (*_41);
_33.0 = &_47;
_42 = _39 + _39;
_61.0.0 = core::ptr::addr_of_mut!(_7);
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_63 = core::ptr::addr_of!(_45.fld0);
(*_31) = (-2661075579730681953_i64) as u128;
_43 = _20;
_44 = _30;
_65.0.fld5 = _29.2 as u16;
_45.fld0 = 959006934243056628_u64 | 2471197766030781198_u64;
_35 = core::ptr::addr_of_mut!((*_35));
(*_63) = 401789333410456558_u64 << _29.3.1;
match _29.1 {
0 => bb12,
1 => bb29,
3981475970 => bb31,
_ => bb30
}
}
bb29 = {
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_41 = core::ptr::addr_of_mut!((*_41));
(*_14) = _22;
(*_14) = _22;
_6 = _21;
(*_41) = _11 * _11;
match _29.1 {
0 => bb20,
3981475970 => bb22,
_ => bb21
}
}
bb30 = {
_53 = _16 as f64;
(*_31) = _25;
(*_14) = -_22;
_29.3.2 = &_47;
(*_14) = _22 - _22;
(*_31) = !_25;
Call((*_14) = core::intrinsics::transmute(_13), ReturnTo(bb24), UnwindUnreachable())
}
bb31 = {
(*_63) = 8281819592930804978_u64 >> (*_31);
(*_14) = _53 * _22;
_1 = core::ptr::addr_of_mut!((*_50));
(*_14) = _53 - _53;
_22 = (*_14) - (*_14);
_1 = core::ptr::addr_of_mut!((*_1));
(*_41) = !_11;
_44 = _29.1;
(*_35) = [(*_31),(*_31),(*_31),_25,(*_31),_27,_25];
(*_14) = _25 as f64;
(*_63) = 9513625967777204603_u64;
_30 = 18176451091471845217_usize as u32;
(*_35) = [(*_31),(*_31),_25,_25,_27,_25,(*_31)];
_59 = _25 - (*_31);
(*_35) = [(*_31),(*_31),(*_31),_25,_59,_59,(*_31)];
(*_31) = _59 | _25;
(*_8) = Adt41::Variant2 { fld0: Move(_63),fld1: _44,fld2: _47,fld3: _29.2,fld4: 6945102425466409223_i64 };
(*_31) = _59 >> _39;
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2 & _29.2;
match Field::<u32>(Variant((*_8), 2), 1) {
0 => bb8,
1 => bb4,
2 => bb32,
3981475970 => bb34,
_ => bb33
}
}
bb32 = {
_6 = '\u{2192b}';
_7 = [6909_u16,50016_u16,64926_u16,24667_u16,18123_u16];
_7 = [3025_u16,54737_u16,21260_u16,13733_u16,44806_u16];
RET = (-22_isize);
_6 = '\u{3e8ab}';
RET = (-9223372036854775808_isize) << (-15174_i16);
_7 = [6413_u16,31563_u16,26018_u16,3410_u16,22569_u16];
_6 = '\u{ee1a4}';
_6 = '\u{5dbeb}';
_7 = [28717_u16,19482_u16,23876_u16,12673_u16,32102_u16];
RET = (-126_isize);
_2.fld1 = 34586272850893137037267424881081082761_u128 as f64;
_2.fld1 = RET as f64;
_7 = [31133_u16,10302_u16,53138_u16,49021_u16,60547_u16];
_6 = '\u{e132a}';
RET = -(-64_isize);
_7 = [10890_u16,52692_u16,7740_u16,33335_u16,14546_u16];
Goto(bb2)
}
bb33 = {
_34 = _36;
(*_41) = !_11;
_17 = 99_u8 * 99_u8;
(*_14) = _39 as f64;
_43 = [_29.1];
(*_14) = _22;
_50 = core::ptr::addr_of_mut!((*_1));
Goto(bb23)
}
bb34 = {
_27 = _59 | _25;
place!(Field::<i64>(Variant((*_8), 2), 4)) = 2641829149842754482_i64 & (-4317177520590888872_i64);
_29.3.1 = _29.3.0 & _29.3.0;
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2 - _29.2;
(*_35) = [_59,(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
place!(Field::<u32>(Variant(_40, 2), 1)) = !_44;
place!(Field::<i64>(Variant((*_8), 2), 4)) = (-5314590995062309939_i64);
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2 >> (*_41);
(*_14) = _16 as f64;
_45 = Adt49 { fld0: 2060917464332634131_u64,fld1: _10 };
(*_41) = _11 + _11;
place!(Field::<*const u64>(Variant((*_8), 2), 0)) = core::ptr::addr_of!(_65.0.fld1.0);
place!(Field::<*const u64>(Variant((*_8), 2), 0)) = core::ptr::addr_of!(_65.0.fld1.0);
(*_41) = !_11;
_29.1 = !Field::<u32>(Variant((*_8), 2), 1);
place!(Field::<i64>(Variant((*_8), 2), 4)) = (-2939492457507019956_i64) + (-1363106273023501323_i64);
_65.0.fld6 = core::ptr::addr_of!(_17);
_23 = &mut place!(Field::<f32>(Variant((*_8), 2), 2));
place!(Field::<u32>(Variant((*_8), 2), 1)) = _29.1 | _30;
_35 = core::ptr::addr_of_mut!((*_35));
_65.0.fld4 = _45.fld0;
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2;
(*_41) = -_11;
Goto(bb35)
}
bb35 = {
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2 ^ _29.2;
(*_1) = core::ptr::addr_of!(_23);
place!(Field::<i64>(Variant((*_8), 2), 4)) = Field::<u32>(Variant((*_8), 2), 1) as i64;
(*_50) = core::ptr::addr_of!((*_28));
(*_14) = _53;
(*_28) = &mut _47;
(*_50) = core::ptr::addr_of!(_23);
place!(Field::<u32>(Variant((*_8), 2), 1)) = _44 ^ _30;
_45.fld1 = [_42,_42,_42,_42,_42,_42];
_7 = [_29.3.1,_29.3.1,_29.3.1,_29.3.0,_29.3.1];
(*_35) = [(*_31),(*_31),_27,(*_31),(*_31),_27,(*_31)];
(*_48) = core::ptr::addr_of!((*_28));
place!(Field::<i64>(Variant((*_8), 2), 4)) = (*_23) as i64;
place!(Field::<i64>(Variant((*_8), 2), 4)) = (-2931157695569194620_i64);
_39 = (*_23) as isize;
match Field::<i64>(Variant((*_8), 2), 4) {
0 => bb36,
1 => bb37,
2 => bb38,
3 => bb39,
4 => bb40,
340282366920938463460443449736199016836 => bb42,
_ => bb41
}
}
bb36 = {
Return()
}
bb37 = {
Return()
}
bb38 = {
(*_31) = _25 | _25;
(*_35) = [(*_31),_25,(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_14) = -_22;
(*_35) = [(*_31),(*_31),(*_31),_27,(*_31),(*_31),(*_31)];
(*_14) = -_22;
_45.fld1 = _10;
(*_31) = _25 + _25;
_45 = Adt49 { fld0: 10849488004210442991_u64,fld1: _10 };
(*_31) = !_25;
(*_31) = !_25;
_41 = core::ptr::addr_of_mut!(_15);
(*_31) = 2_usize as u128;
(*_31) = _25 * _25;
(*_41) = _11 ^ _11;
_16 = 1938536156_i32 - 300018634_i32;
_25 = (*_31);
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_47 = (*_31) as f32;
_45.fld0 = 4688628002940270063_i64 as u64;
(*_41) = _11 | _11;
(*_41) = 3212339791465790050_i64 as i16;
_48 = core::ptr::addr_of_mut!((*_1));
(*_31) = !_25;
(*_31) = _25 >> (*_41);
(*_14) = -_22;
(*_31) = _25;
Goto(bb19)
}
bb39 = {
_53 = _16 as f64;
(*_31) = _25;
(*_14) = -_22;
_29.3.2 = &_47;
(*_14) = _22 - _22;
(*_31) = !_25;
Call((*_14) = core::intrinsics::transmute(_13), ReturnTo(bb24), UnwindUnreachable())
}
bb40 = {
(*_14) = (-111_i8) as f64;
_21 = _6;
(*_14) = _16 as f64;
(*_14) = _17 as f64;
(*_14) = 509388813058656369_i64 as f64;
(*_14) = _16 as f64;
_17 = 32_u8;
(*_14) = 187194762050466182953913505265011281841_u128 as f64;
(*_14) = (-6159620663145065238_i64) as f64;
_15 = _11 ^ _11;
(*_14) = 8593869503484613414377104187155439174_u128 as f64;
_20 = [2514233324_u32];
(*_14) = _17 as f64;
_22 = _15 as f64;
(*_14) = -_22;
Goto(bb10)
}
bb41 = {
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_41 = core::ptr::addr_of_mut!((*_41));
(*_14) = _22;
(*_14) = _22;
_6 = _21;
(*_41) = _11 * _11;
match _29.1 {
0 => bb20,
3981475970 => bb22,
_ => bb21
}
}
bb42 = {
(*_41) = _11 | _11;
_65.0.fld0 = -(*_23);
place!(Field::<i8>(Variant((*_8), 2), 3)) = _29.2 + _29.2;
(*_14) = _22 * _22;
_3 = &mut (*_23);
_67 = _36;
place!(Field::<u32>(Variant((*_8), 2), 1)) = _30 ^ _29.1;
place!(Field::<i8>(Variant((*_8), 2), 3)) = -_29.2;
place!(Field::<*const u64>(Variant((*_8), 2), 0)) = core::ptr::addr_of!(_45.fld0);
(*_48) = core::ptr::addr_of!((*_28));
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_63 = core::ptr::addr_of!(_65.0.fld4);
_15 = _11 >> (*_63);
place!(Field::<*const u64>(Variant((*_8), 2), 0)) = core::ptr::addr_of!((*_63));
(*_41) = (*_31) as i16;
_73 = (-2241653080153341227351288924310110395_i128) + (-94268703273805721194381042276163536656_i128);
_9 = &mut _23;
_34 = _29.3.3;
_65.0.fld1 = ((*_63),);
_65.0.fld3 = Field::<i64>(Variant((*_8), 2), 4) as f64;
match (*_63) {
0 => bb30,
1 => bb33,
2 => bb34,
3 => bb40,
4 => bb23,
5 => bb38,
2060917464332634131 => bb44,
_ => bb43
}
}
bb43 = {
(*_31) = _25 | _25;
(*_35) = [(*_31),_25,(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_14) = -_22;
(*_35) = [(*_31),(*_31),(*_31),_27,(*_31),(*_31),(*_31)];
(*_14) = -_22;
_45.fld1 = _10;
(*_31) = _25 + _25;
_45 = Adt49 { fld0: 10849488004210442991_u64,fld1: _10 };
(*_31) = !_25;
(*_31) = !_25;
_41 = core::ptr::addr_of_mut!(_15);
(*_31) = 2_usize as u128;
(*_31) = _25 * _25;
(*_41) = _11 ^ _11;
_16 = 1938536156_i32 - 300018634_i32;
_25 = (*_31);
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
_47 = (*_31) as f32;
_45.fld0 = 4688628002940270063_i64 as u64;
(*_41) = _11 | _11;
(*_41) = 3212339791465790050_i64 as i16;
_48 = core::ptr::addr_of_mut!((*_1));
(*_31) = !_25;
(*_31) = _25 >> (*_41);
(*_14) = -_22;
(*_31) = _25;
Goto(bb19)
}
bb44 = {
(*_31) = _59 >> (*_41);
_14 = &mut _22;
_7 = [_29.3.1,_29.3.1,_29.3.0,_29.3.1,_65.0.fld5];
Goto(bb45)
}
bb45 = {
(*_41) = _11;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
place!(Field::<u32>(Variant((*_8), 2), 1)) = _30;
_65.0.fld4 = !_65.0.fld1.0;
_41 = core::ptr::addr_of_mut!((*_41));
(*_35) = [(*_31),(*_31),_27,(*_31),(*_31),(*_31),(*_31)];
(*_41) = _11 & _11;
place!(Field::<u32>(Variant((*_8), 2), 1)) = !_29.1;
_61.0.2 = Move(_8);
(*_9) = &mut _65.0.fld0;
_33.1 = core::ptr::addr_of_mut!(_73);
(*_63) = _17 as u64;
_75 = _42 & _39;
(*_48) = core::ptr::addr_of!((*_9));
(*_31) = _73 as u128;
(*_48) = core::ptr::addr_of!((*_9));
_67 = _6;
(*_14) = _53 * _53;
(*_48) = core::ptr::addr_of!((*_28));
Goto(bb46)
}
bb46 = {
_59 = !_25;
_69 = core::ptr::addr_of!((*_28));
(*_48) = core::ptr::addr_of!((*_69));
_82 = (_29.3.1, _29.3.0, Move(_33.0), _67);
(*_35) = [_25,(*_31),_25,(*_31),(*_31),(*_31),(*_31)];
_73 = _17 as i128;
(*_9) = &mut (*_3);
_43 = _20;
_81 = _12;
match _45.fld0 {
0 => bb47,
1 => bb48,
2 => bb49,
3 => bb50,
4 => bb51,
5 => bb52,
6 => bb53,
2060917464332634131 => bb55,
_ => bb54
}
}
bb47 = {
(*_31) = _25 >> _13;
(*_14) = _53;
_30 = _29.1 + _29.1;
_27 = _25 - _25;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),(*_31)];
(*_31) = _47 as u128;
(*_35) = [(*_31),(*_31),(*_31),(*_31),(*_31),(*_31),_25];
Goto(bb28)
}
bb48 = {
(*_31) = _59 >> (*_41);
_14 = &mut _22;
_7 = [_29.3.1,_29.3.1,_29.3.0,_29.3.1,_65.0.fld5];
Goto(bb45)
}
bb49 = {
_34 = _36;
(*_41) = !_11;
_17 = 99_u8 * 99_u8;
(*_14) = _39 as f64;
_43 = [_29.1];
(*_14) = _22;
_50 = core::ptr::addr_of_mut!((*_1));
Goto(bb23)
}
bb50 = {
_2.fld1 = RET as f64;
RET = -(-9223372036854775808_isize);
RET = -9223372036854775807_isize;
_6 = '\u{1032f2}';
_7 = [18535_u16,44106_u16,4954_u16,58927_u16,27168_u16];
_7 = [26891_u16,1069_u16,471_u16,5941_u16,59968_u16];
_6 = '\u{d4301}';
_7 = [36468_u16,17107_u16,32455_u16,57140_u16,44442_u16];
_10 = [RET,RET,RET,RET,RET,RET];
_6 = '\u{20ab0}';
_7 = [48709_u16,61454_u16,5555_u16,50052_u16,25941_u16];
_7 = [62430_u16,61130_u16,1852_u16,54507_u16,62135_u16];
_6 = '\u{fc58}';
_7 = [6238_u16,54038_u16,58673_u16,39097_u16,41182_u16];
_7 = [41931_u16,42536_u16,7060_u16,64377_u16,30469_u16];
_6 = '\u{dd34c}';
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_2.fld1 = 1859600846_i32 as f64;
_7 = [44668_u16,45779_u16,30577_u16,58122_u16,11157_u16];
_11 = (-6883_i16) << RET;
_2.fld1 = 1923817442901819269_i64 as f64;
_11 = -(-31155_i16);
_11 = -(-31796_i16);
_6 = '\u{dd9e6}';
_2.fld1 = (-2092286170_i32) as f64;
_10 = [RET,RET,RET,RET,RET,RET];
RET = 9223372036854775807_isize;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb51 = {
Return()
}
bb52 = {
(*_14) = (-111_i8) as f64;
_21 = _6;
(*_14) = _16 as f64;
(*_14) = _17 as f64;
(*_14) = 509388813058656369_i64 as f64;
(*_14) = _16 as f64;
_17 = 32_u8;
(*_14) = 187194762050466182953913505265011281841_u128 as f64;
(*_14) = (-6159620663145065238_i64) as f64;
_15 = _11 ^ _11;
(*_14) = 8593869503484613414377104187155439174_u128 as f64;
_20 = [2514233324_u32];
(*_14) = _17 as f64;
_22 = _15 as f64;
(*_14) = -_22;
Goto(bb10)
}
bb53 = {
_6 = '\u{2192b}';
_7 = [6909_u16,50016_u16,64926_u16,24667_u16,18123_u16];
_7 = [3025_u16,54737_u16,21260_u16,13733_u16,44806_u16];
RET = (-22_isize);
_6 = '\u{3e8ab}';
RET = (-9223372036854775808_isize) << (-15174_i16);
_7 = [6413_u16,31563_u16,26018_u16,3410_u16,22569_u16];
_6 = '\u{ee1a4}';
_6 = '\u{5dbeb}';
_7 = [28717_u16,19482_u16,23876_u16,12673_u16,32102_u16];
RET = (-126_isize);
_2.fld1 = 34586272850893137037267424881081082761_u128 as f64;
_2.fld1 = RET as f64;
_7 = [31133_u16,10302_u16,53138_u16,49021_u16,60547_u16];
_6 = '\u{e132a}';
RET = -(-64_isize);
_7 = [10890_u16,52692_u16,7740_u16,33335_u16,14546_u16];
Goto(bb2)
}
bb54 = {
(*_14) = _22;
_21 = _6;
_6 = _21;
(*_14) = _22;
(*_14) = _22 * _22;
(*_14) = _22 - _22;
_17 = !213_u8;
_25 = !336152456769583345793048374781323608914_u128;
_12 = [_21,_21,_6,_21,_6];
(*_14) = -_22;
(*_14) = _22 - _22;
_21 = _6;
_25 = 294115918905758423565702047793719848904_u128 - 93346564971624152869042834608778654897_u128;
_13 = _21 as isize;
(*_14) = _22 * _22;
_6 = _21;
_13 = RET;
_22 = -(*_14);
_1 = core::ptr::addr_of_mut!(_28);
Goto(bb12)
}
bb55 = {
(*_41) = (*_14) as i16;
Goto(bb56)
}
bb56 = {
Call(_86 = dump_var(Move(_34), Move(_16), Move(_17), Move(_6)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_86 = dump_var(Move(_27), Move(_49), Move(_42), Move(_81)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_86 = dump_var(Move(_20), Move(_10), Move(_25), Move(_67)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_86 = dump_var(Move(_59), _87, _87, _87), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *mut *const &'static mut f32,mut _2: *mut u128,mut _3: u128,mut _4: u16,mut _5: char) -> f64 {
mir! {
type RET = f64;
let _6: &'static &'static mut &'static mut f32;
let _7: (*mut &'static mut [char; 6],);
let _8: f64;
let _9: f32;
let _10: &'static mut *const u64;
let _11: usize;
let _12: &'static f32;
let _13: char;
let _14: *mut *const &'static mut f32;
let _15: bool;
let _16: (&'static isize, *const (u64,), i32);
let _17: u64;
let _18: &'static mut u32;
let _19: f32;
let _20: (bool, [u16; 5]);
let _21: isize;
let _22: *const Adt22;
let _23: *mut i128;
let _24: char;
let _25: f32;
let _26: &'static mut [char; 6];
let _27: *mut i16;
let _28: f64;
let _29: isize;
let _30: (*mut [u16; 5], *const (u64,), *const Adt41);
let _31: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _32: usize;
let _33: (i8, u64, u16);
let _34: &'static &'static mut *const u64;
let _35: (f64, f64, u128, i16);
let _36: isize;
let _37: i8;
let _38: *mut u128;
let _39: *mut [u16; 5];
let _40: char;
let _41: [u64; 2];
let _42: f32;
let _43: f64;
let _44: bool;
let _45: f32;
let _46: *mut u128;
let _47: *const u32;
let _48: *mut &'static mut [char; 6];
let _49: &'static u32;
let _50: *const u32;
let _51: u32;
let _52: i16;
let _53: [u128; 7];
let _54: Adt49;
let _55: f64;
let _56: Adt26;
let _57: f32;
let _58: (Adt26, Adt41);
let _59: (f64, f64, u128, i16);
let _60: char;
let _61: char;
let _62: isize;
let _63: char;
let _64: Adt75;
let _65: bool;
let _66: f64;
let _67: &'static mut &'static mut f32;
let _68: f32;
let _69: &'static u32;
let _70: (u16, u16, &'static f32, char);
let _71: i64;
let _72: &'static &'static mut *const u64;
let _73: f32;
let _74: i32;
let _75: isize;
let _76: *const Adt41;
let _77: char;
let _78: [u16; 5];
let _79: (&'static f32, *mut i128);
let _80: ((*mut [u16; 5], *const (u64,), *const Adt41), &'static mut [char; 6]);
let _81: &'static mut *mut i128;
let _82: isize;
let _83: *mut &'static mut [char; 6];
let _84: (isize, u8, Adt49);
let _85: *const &'static mut f32;
let _86: isize;
let _87: char;
let _88: (bool, [u16; 5]);
let _89: f64;
let _90: ();
let _91: ();
{
RET = 2923918138498436235_i64 as f64;
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = 3451404633_u32 as u128;
(*_2) = 59929203762173185392594882632637522693_u128 | 237524449283007428474078376348498647398_u128;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = 106544600232079329144419509225071107670_u128;
(*_2) = 57_isize as u128;
(*_2) = 516749192_i32 as u128;
(*_2) = 256587705241397376387743688012924723617_u128;
(*_2) = 15365313324548643472_usize as u128;
(*_2) = 1767797686818203785_i64 as u128;
(*_2) = 109089934103003076640099033388686567903_u128;
_5 = '\u{bedd5}';
Goto(bb1)
}
bb1 = {
(*_2) = 50872314568087055845285946141557741225_i128 as u128;
(*_2) = 181633941004415883636773058159965394247_u128 - 22254483570838178782905937337798020123_u128;
(*_2) = 4193758187_u32 as u128;
(*_2) = (-5187661176644804969_i64) as u128;
_5 = '\u{344a6}';
(*_2) = 112259883449225830349900379893996010998_u128;
(*_2) = 223102601357065667340818054069159971484_u128 >> _4;
(*_2) = !70164505198208633299336983750149804674_u128;
(*_2) = (-111_i8) as u128;
RET = (-63_i8) as f64;
(*_2) = 45559457030410187717708842107448661871_u128 | 63333064782845732505940725030031836093_u128;
_5 = '\u{62d35}';
(*_2) = (-7681798112746501194_i64) as u128;
_5 = '\u{e9161}';
(*_2) = 92_u8 as u128;
(*_2) = 38081850885102611301560620492207430156_u128;
(*_2) = 326011518619573011942938557051271060058_u128;
(*_2) = 232471401145338849942911517736470516809_u128 - 261734584488926155006314775430868334909_u128;
Goto(bb2)
}
bb2 = {
(*_2) = !60703892825000297398293708585599090976_u128;
(*_2) = 53222403051380274599796181895687258050_u128 - 192405936233096565622057095127026231772_u128;
(*_2) = 194053295506542315233901245947709428133_u128;
(*_2) = 172325708266120093133574515900111021389_u128 + 236365218527037596250442959091735990363_u128;
_13 = _5;
(*_2) = !55457461121432140809633531581504990479_u128;
(*_2) = !267109584217948294995299945089449859927_u128;
_5 = _13;
_5 = _13;
_11 = 18297294695920398765_usize;
RET = 203148127_i32 as f64;
_8 = -RET;
_9 = (-29309_i16) as f32;
(*_2) = 36477181511543069743560490633223925704_u128;
(*_2) = !123758718525990319432979020434721464040_u128;
(*_2) = 279737432615881634092838109354309339895_u128;
Goto(bb3)
}
bb3 = {
_5 = _13;
Goto(bb4)
}
bb4 = {
RET = _8 - _8;
(*_2) = 198928128280361054231964336167598143025_u128;
(*_2) = _4 as u128;
(*_2) = 2590999725302682869_i64 as u128;
(*_2) = _11 as u128;
(*_2) = 323939122181872528585257433588268794348_u128 & 268234403736229052658577651831947048273_u128;
_3 = 14205667730748794081131622676237573651_u128;
(*_2) = 172058637759125465010786813099429310248_u128;
(*_2) = 12475611183353484672_u64 as u128;
(*_2) = true as u128;
_3 = 162183827827600809137268073703198072123_u128 ^ 308036345787773246510425769338767281892_u128;
_9 = RET as f32;
_9 = 1789561984184166358_u64 as f32;
_12 = &_9;
_19 = (*_12) * (*_12);
(*_2) = 53689482320936297342379362439000724717_u128 & 67445201907656243369557451976887416730_u128;
_20.1 = [_4,_4,_4,_4,_4];
(*_2) = 15847337927113023854_u64 as u128;
_20.1 = [_4,_4,_4,_4,_4];
_9 = -_19;
RET = _11 as f64;
(*_2) = !77599390784585530765015893525840319140_u128;
Goto(bb5)
}
bb5 = {
(*_2) = !259737205088292515383114857840660816949_u128;
(*_2) = 162862173872999536329342147823235993494_u128 + 36319258342701890964232119595961630306_u128;
(*_2) = 228298184367856340477378269947269855817_u128;
(*_2) = 205635998712660386278046285162628796649_u128;
_20.0 = (*_2) != (*_2);
(*_2) = 150186343640681541138522520032281183914_u128 - 231737708968630727161129155602805615429_u128;
(*_2) = !58075471261582707221907769438220364041_u128;
_9 = 175383661919064963_i64 as f32;
_15 = _20.0;
_17 = _19 as u64;
_8 = (-83_isize) as f64;
_21 = 9223372036854775807_isize;
_16.2 = _9 as i32;
_16.2 = _4 as i32;
_12 = &_19;
(*_2) = 90178430969477030002638834614577693188_u128;
(*_2) = 191310347442784767612060788112646752608_u128 * 197401237398038075799496804240661830938_u128;
_16.0 = &_21;
(*_2) = 77_i8 as u128;
(*_2) = 166341768343420536985647426501725630189_u128;
(*_2) = 333001792793308282207002633541544736010_u128;
_20.0 = _15 ^ _15;
(*_2) = _21 as u128;
(*_2) = !41589449074559192171554091482392365294_u128;
match _21 {
0 => bb3,
9223372036854775807 => bb6,
_ => bb2
}
}
bb6 = {
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = (-17865_i16) as u128;
(*_2) = 73786476240129924225675345928398284017_u128 >> _17;
_24 = _13;
RET = (-84_i8) as f64;
(*_2) = _11 as u128;
RET = _8 + _8;
(*_2) = 3790834368_u32 as u128;
(*_2) = !248045169434288361483206359247132712261_u128;
(*_2) = _16.2 as u128;
(*_2) = 226777797352635126574722561541211187616_u128;
_13 = _5;
(*_2) = _20.0 as u128;
(*_2) = !91974362893129213402815971448217039843_u128;
_24 = _13;
_25 = (*_12) * (*_12);
_3 = 221_u8 as u128;
_24 = _13;
(*_2) = (-64_i8) as u128;
(*_2) = !153715101373822378439916207868769054982_u128;
(*_2) = !75655576955301556169569708691416442317_u128;
(*_2) = 75739450691916715186410529560177492617_u128;
(*_2) = _16.2 as u128;
(*_2) = !316237047062899219308251168742910152525_u128;
(*_2) = 136421932043846886066536719617830131247_u128 - 105080039081416307451332004729120101938_u128;
Goto(bb7)
}
bb7 = {
_4 = !6488_u16;
_20.1 = [_4,_4,_4,_4,_4];
_20.0 = !_15;
(*_2) = 147768708094224353775256420848909219410_u128 ^ 298191499449081694167924846417150293371_u128;
(*_2) = 46_i8 as u128;
_17 = 12480519114082067004_u64 + 3594785990907152225_u64;
(*_2) = 264430445531000972658061903077541355246_u128 >> _4;
_19 = _21 as f32;
(*_2) = 223470808305977637747491661008926003509_u128 | 311583042927625305163578926727855560668_u128;
_28 = (*_2) as f64;
(*_2) = 156498689950569296696280326538274173959_u128 | 126656324037732287114790041121330817210_u128;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
_30.0 = Move(_31.0.0);
_17 = 15157646235932609025_u64 >> (*_2);
_29 = _5 as isize;
(*_2) = _4 as u128;
_33 = (70_i8, _17, _4);
(*_2) = 332039014295693919566587421183438818910_u128 << _4;
_27 = core::ptr::addr_of_mut!(_35.3);
(*_27) = -(-27952_i16);
_21 = _29;
Goto(bb8)
}
bb8 = {
(*_2) = 278725431585167937425622630241262738678_u128;
(*_27) = _11 as i16;
(*_27) = (-3214262484185550161_i64) as i16;
(*_2) = 223177699725062238145761752054923976189_u128;
(*_27) = 31670_i16 + (-15213_i16);
(*_2) = 92669841545525378836957926064059699612_u128 >> _16.2;
_35.0 = _28;
_16.2 = (-917373395_i32) & (-1504623622_i32);
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
_35 = (_28, _28, (*_2), (-22563_i16));
(*_2) = _35.2;
(*_27) = (-31055_i16) << _21;
(*_2) = _4 as u128;
_37 = _11 as i8;
RET = -_35.1;
(*_2) = _35.2 - _35.2;
_3 = _35.2 * _35.2;
(*_27) = 6447_i16;
(*_27) = 25634_i16 - (-23386_i16);
_30.0 = core::ptr::addr_of_mut!(_20.1);
(*_27) = (-16772_i16);
_20.1 = [_4,_4,_4,_33.2,_4];
_36 = _29 ^ _21;
(*_27) = -20909_i16;
Goto(bb9)
}
bb9 = {
_15 = (*_2) <= (*_2);
_20.0 = _15;
(*_2) = _35.2 ^ _35.2;
(*_27) = 17834_i16 * 11901_i16;
(*_27) = (-16740_i16) - 24105_i16;
_3 = !_35.2;
(*_2) = _11 as u128;
_35 = (_28, _28, (*_2), (-27251_i16));
_35.2 = (*_2);
_36 = _21;
(*_27) = -14854_i16;
_38 = core::ptr::addr_of_mut!((*_2));
(*_27) = (-7276_i16) - (-11243_i16);
(*_2) = _35.2 << _17;
_35.1 = _8 + _35.0;
_4 = !_33.2;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
(*_2) = _35.2;
_42 = _9 - _25;
_41 = [_17,_33.1];
(*_27) = (-30730_i16) << (*_2);
_32 = !_11;
RET = _35.1;
_39 = core::ptr::addr_of_mut!(_20.1);
(*_27) = (-5822_i16);
(*_27) = 17493_i16 * 3375_i16;
(*_2) = _35.2;
Goto(bb10)
}
bb10 = {
(*_27) = (-18126_i16) & 22649_i16;
_28 = -RET;
_41 = [_33.1,_17];
_2 = core::ptr::addr_of_mut!((*_2));
_13 = _5;
(*_27) = -20112_i16;
(*_2) = _35.2;
(*_27) = 29143_i16 >> (*_2);
(*_27) = 128_u8 as i16;
match _33.0 {
0 => bb9,
1 => bb8,
70 => bb11,
_ => bb6
}
}
bb11 = {
(*_39) = [_33.2,_4,_33.2,_4,_33.2];
(*_27) = (-2017_i16) ^ 20025_i16;
(*_27) = 22456_i16;
(*_2) = _35.2 << _33.1;
_38 = core::ptr::addr_of_mut!((*_2));
_4 = _16.2 as u16;
(*_39) = [_33.2,_4,_4,_4,_33.2];
(*_2) = _35.2 ^ _35.2;
(*_39) = [_4,_33.2,_4,_4,_4];
(*_27) = _28 as i16;
(*_39) = [_4,_33.2,_4,_4,_4];
_5 = _24;
(*_27) = (-7883_i16);
_32 = _11 + _11;
(*_27) = _19 as i16;
_42 = -_25;
_5 = _13;
(*_39) = [_33.2,_4,_4,_33.2,_33.2];
(*_27) = (-24169_i16);
(*_2) = !_35.2;
_8 = _33.0 as f64;
(*_27) = !(-641_i16);
_19 = -_42;
match _33.0 {
0 => bb7,
1 => bb2,
2 => bb12,
3 => bb13,
70 => bb15,
_ => bb14
}
}
bb12 = {
RET = _8 - _8;
(*_2) = 198928128280361054231964336167598143025_u128;
(*_2) = _4 as u128;
(*_2) = 2590999725302682869_i64 as u128;
(*_2) = _11 as u128;
(*_2) = 323939122181872528585257433588268794348_u128 & 268234403736229052658577651831947048273_u128;
_3 = 14205667730748794081131622676237573651_u128;
(*_2) = 172058637759125465010786813099429310248_u128;
(*_2) = 12475611183353484672_u64 as u128;
(*_2) = true as u128;
_3 = 162183827827600809137268073703198072123_u128 ^ 308036345787773246510425769338767281892_u128;
_9 = RET as f32;
_9 = 1789561984184166358_u64 as f32;
_12 = &_9;
_19 = (*_12) * (*_12);
(*_2) = 53689482320936297342379362439000724717_u128 & 67445201907656243369557451976887416730_u128;
_20.1 = [_4,_4,_4,_4,_4];
(*_2) = 15847337927113023854_u64 as u128;
_20.1 = [_4,_4,_4,_4,_4];
_9 = -_19;
RET = _11 as f64;
(*_2) = !77599390784585530765015893525840319140_u128;
Goto(bb5)
}
bb13 = {
(*_2) = 50872314568087055845285946141557741225_i128 as u128;
(*_2) = 181633941004415883636773058159965394247_u128 - 22254483570838178782905937337798020123_u128;
(*_2) = 4193758187_u32 as u128;
(*_2) = (-5187661176644804969_i64) as u128;
_5 = '\u{344a6}';
(*_2) = 112259883449225830349900379893996010998_u128;
(*_2) = 223102601357065667340818054069159971484_u128 >> _4;
(*_2) = !70164505198208633299336983750149804674_u128;
(*_2) = (-111_i8) as u128;
RET = (-63_i8) as f64;
(*_2) = 45559457030410187717708842107448661871_u128 | 63333064782845732505940725030031836093_u128;
_5 = '\u{62d35}';
(*_2) = (-7681798112746501194_i64) as u128;
_5 = '\u{e9161}';
(*_2) = 92_u8 as u128;
(*_2) = 38081850885102611301560620492207430156_u128;
(*_2) = 326011518619573011942938557051271060058_u128;
(*_2) = 232471401145338849942911517736470516809_u128 - 261734584488926155006314775430868334909_u128;
Goto(bb2)
}
bb14 = {
_5 = _13;
Goto(bb4)
}
bb15 = {
_3 = _35.2 & _35.2;
_39 = Move(_31.0.0);
_47 = core::ptr::addr_of!(_51);
(*_47) = 3107460112_u32 >> (*_2);
(*_27) = (-30051_i16) - (-15789_i16);
(*_2) = _35.2;
(*_27) = _11 as i16;
(*_27) = (-14406_i16);
(*_27) = 77_u8 as i16;
(*_2) = !_35.2;
_29 = _33.2 as isize;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
_35.3 = !22617_i16;
_52 = (*_27) * (*_27);
_5 = _13;
(*_47) = 139115790_u32 << (*_27);
_29 = _21 << (*_47);
(*_27) = _52 & _52;
Goto(bb16)
}
bb16 = {
(*_2) = _35.2;
_58.0.fld2 = [(*_2),_35.2,(*_2),_3,(*_2),(*_2),(*_2)];
_55 = _8 * _35.0;
_35.0 = _55 + _55;
_35.2 = !(*_2);
(*_2) = _35.2 - _35.2;
_57 = -_42;
_52 = (*_27) | (*_27);
_9 = _25 - _57;
(*_27) = _4 as i16;
_45 = _9;
_58.0.fld0 = _42;
_56.fld3 = _35.0;
(*_27) = -_52;
(*_47) = 75954802_u32 ^ 3204712790_u32;
_21 = _29 * _36;
_30.2 = core::ptr::addr_of!(_58.1);
(*_2) = _35.2 ^ _35.2;
Call((*_2) = core::intrinsics::bswap(_35.2), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_50 = core::ptr::addr_of!((*_47));
(*_47) = !1144310667_u32;
_52 = !(*_27);
_55 = _56.fld3 + _56.fld3;
(*_2) = !_35.2;
_8 = (*_47) as f64;
_56.fld0 = -_9;
_30.2 = core::ptr::addr_of!(_58.1);
(*_2) = _35.2;
_60 = _5;
match _11 {
0 => bb16,
1 => bb2,
2 => bb10,
3 => bb6,
4 => bb18,
5 => bb19,
18297294695920398765 => bb21,
_ => bb20
}
}
bb18 = {
_4 = !6488_u16;
_20.1 = [_4,_4,_4,_4,_4];
_20.0 = !_15;
(*_2) = 147768708094224353775256420848909219410_u128 ^ 298191499449081694167924846417150293371_u128;
(*_2) = 46_i8 as u128;
_17 = 12480519114082067004_u64 + 3594785990907152225_u64;
(*_2) = 264430445531000972658061903077541355246_u128 >> _4;
_19 = _21 as f32;
(*_2) = 223470808305977637747491661008926003509_u128 | 311583042927625305163578926727855560668_u128;
_28 = (*_2) as f64;
(*_2) = 156498689950569296696280326538274173959_u128 | 126656324037732287114790041121330817210_u128;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
_30.0 = Move(_31.0.0);
_17 = 15157646235932609025_u64 >> (*_2);
_29 = _5 as isize;
(*_2) = _4 as u128;
_33 = (70_i8, _17, _4);
(*_2) = 332039014295693919566587421183438818910_u128 << _4;
_27 = core::ptr::addr_of_mut!(_35.3);
(*_27) = -(-27952_i16);
_21 = _29;
Goto(bb8)
}
bb19 = {
_15 = (*_2) <= (*_2);
_20.0 = _15;
(*_2) = _35.2 ^ _35.2;
(*_27) = 17834_i16 * 11901_i16;
(*_27) = (-16740_i16) - 24105_i16;
_3 = !_35.2;
(*_2) = _11 as u128;
_35 = (_28, _28, (*_2), (-27251_i16));
_35.2 = (*_2);
_36 = _21;
(*_27) = -14854_i16;
_38 = core::ptr::addr_of_mut!((*_2));
(*_27) = (-7276_i16) - (-11243_i16);
(*_2) = _35.2 << _17;
_35.1 = _8 + _35.0;
_4 = !_33.2;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
(*_2) = _35.2;
_42 = _9 - _25;
_41 = [_17,_33.1];
(*_27) = (-30730_i16) << (*_2);
_32 = !_11;
RET = _35.1;
_39 = core::ptr::addr_of_mut!(_20.1);
(*_27) = (-5822_i16);
(*_27) = 17493_i16 * 3375_i16;
(*_2) = _35.2;
Goto(bb10)
}
bb20 = {
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = (-17865_i16) as u128;
(*_2) = 73786476240129924225675345928398284017_u128 >> _17;
_24 = _13;
RET = (-84_i8) as f64;
(*_2) = _11 as u128;
RET = _8 + _8;
(*_2) = 3790834368_u32 as u128;
(*_2) = !248045169434288361483206359247132712261_u128;
(*_2) = _16.2 as u128;
(*_2) = 226777797352635126574722561541211187616_u128;
_13 = _5;
(*_2) = _20.0 as u128;
(*_2) = !91974362893129213402815971448217039843_u128;
_24 = _13;
_25 = (*_12) * (*_12);
_3 = 221_u8 as u128;
_24 = _13;
(*_2) = (-64_i8) as u128;
(*_2) = !153715101373822378439916207868769054982_u128;
(*_2) = !75655576955301556169569708691416442317_u128;
(*_2) = 75739450691916715186410529560177492617_u128;
(*_2) = _16.2 as u128;
(*_2) = !316237047062899219308251168742910152525_u128;
(*_2) = 136421932043846886066536719617830131247_u128 - 105080039081416307451332004729120101938_u128;
Goto(bb7)
}
bb21 = {
_31.0.2 = core::ptr::addr_of!(_58.1);
(*_2) = !_35.2;
(*_27) = _52 - _52;
_59.3 = (*_27);
(*_47) = !3091429876_u32;
_45 = _25 - _56.fld0;
(*_27) = -_59.3;
_13 = _24;
(*_2) = _35.2;
_59 = _35;
_17 = !_33.1;
(*_27) = 34_u8 as i16;
_54.fld1 = [_21,_21,_36,_29,_21,_21];
_56.fld2 = _58.0.fld2;
_56.fld4 = _33.1 << _36;
Goto(bb22)
}
bb22 = {
(*_47) = !3670993850_u32;
_16.0 = &_29;
_47 = core::ptr::addr_of!((*_47));
_29 = _36;
_70.3 = _24;
_58.0.fld1.0 = _56.fld4 | _56.fld4;
(*_47) = !159223800_u32;
match _33.0 {
0 => bb18,
1 => bb7,
2 => bb17,
3 => bb20,
70 => bb24,
_ => bb23
}
}
bb23 = {
_15 = (*_2) <= (*_2);
_20.0 = _15;
(*_2) = _35.2 ^ _35.2;
(*_27) = 17834_i16 * 11901_i16;
(*_27) = (-16740_i16) - 24105_i16;
_3 = !_35.2;
(*_2) = _11 as u128;
_35 = (_28, _28, (*_2), (-27251_i16));
_35.2 = (*_2);
_36 = _21;
(*_27) = -14854_i16;
_38 = core::ptr::addr_of_mut!((*_2));
(*_27) = (-7276_i16) - (-11243_i16);
(*_2) = _35.2 << _17;
_35.1 = _8 + _35.0;
_4 = !_33.2;
_31.0.0 = core::ptr::addr_of_mut!(_20.1);
(*_2) = _35.2;
_42 = _9 - _25;
_41 = [_17,_33.1];
(*_27) = (-30730_i16) << (*_2);
_32 = !_11;
RET = _35.1;
_39 = core::ptr::addr_of_mut!(_20.1);
(*_27) = (-5822_i16);
(*_27) = 17493_i16 * 3375_i16;
(*_2) = _35.2;
Goto(bb10)
}
bb24 = {
_27 = core::ptr::addr_of_mut!((*_27));
_46 = core::ptr::addr_of_mut!((*_2));
_5 = _70.3;
_44 = _45 < _45;
_58.0.fld3 = _57 as f64;
(*_2) = !_59.2;
_36 = -_21;
_53 = [(*_2),(*_2),(*_2),_3,(*_2),(*_2),(*_2)];
(*_47) = 3808003457_u32 * 2237499223_u32;
_53 = [(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2)];
_59.2 = !_35.2;
_40 = _5;
(*_27) = !_52;
_73 = _57;
(*_27) = _59.3 & _52;
_43 = _59.0 - _59.0;
_57 = _45 * _25;
(*_2) = _56.fld3 as u128;
_39 = core::ptr::addr_of_mut!(_78);
(*_39) = [_33.2,_4,_4,_4,_33.2];
Goto(bb25)
}
bb25 = {
(*_27) = !_59.3;
_58.0.fld1.0 = _33.1;
_20.0 = _44;
_46 = Move(_38);
_75 = _21 - _29;
(*_27) = _59.3 | _59.3;
(*_47) = !1131055101_u32;
_2 = core::ptr::addr_of_mut!((*_2));
(*_27) = _52 ^ _52;
_69 = &(*_47);
_30.1 = core::ptr::addr_of!(_58.0.fld1);
_15 = !_20.0;
(*_39) = _20.1;
Goto(bb26)
}
bb26 = {
(*_39) = _20.1;
_59.2 = !(*_2);
_80.0.1 = core::ptr::addr_of!(_56.fld1);
(*_39) = [_4,_4,_33.2,_33.2,_4];
_19 = _9 - _57;
(*_2) = _59.2;
_46 = Move(_2);
(*_39) = _20.1;
_58.0.fld4 = (*_69) as u64;
_33 = (_37, _58.0.fld4, _4);
(*_39) = [_4,_33.2,_4,_33.2,_33.2];
_70.0 = _33.2;
_88.0 = (*_69) <= (*_47);
_76 = core::ptr::addr_of!(_58.1);
_13 = _24;
_58.0.fld6 = core::ptr::addr_of!(_84.1);
_88.1 = [_33.2,_70.0,_70.0,_70.0,_70.0];
Goto(bb27)
}
bb27 = {
_70.2 = &_19;
RET = _59.0 + _43;
_16.1 = core::ptr::addr_of!(_56.fld1);
(*_39) = [_4,_4,_4,_70.0,_70.0];
Goto(bb28)
}
bb28 = {
Call(_90 = dump_var(Move(_75), Move(_13), Move(_78), Move(_17)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_90 = dump_var(Move(_20), Move(_32), Move(_5), Move(_15)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_90 = dump_var(Move(_24), Move(_41), Move(_11), Move(_37)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: char,mut _2: f64,mut _3: f64,mut _4: u16,mut _5: u16,mut _6: isize,mut _7: isize,mut _8: f64,mut _9: f64,mut _10: u16,mut _11: char) -> f64 {
mir! {
type RET = f64;
let _12: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _13: ();
let _14: ();
{
RET = _2 - _3;
_11 = _1;
_12.0.0 = (-121_i8) * 100_i8;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(Move(_4), Move(_6), Move(_5), _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: f64,mut _2: f64,mut _3: isize,mut _4: isize,mut _5: f64,mut _6: f64,mut _7: char,mut _8: u8,mut _9: f64) -> Adt22 {
mir! {
type RET = Adt22;
let _10: i128;
let _11: (&'static isize, *const (u64,), i32);
let _12: (bool, [u16; 5]);
let _13: &'static &'static mut &'static mut f32;
let _14: *mut i16;
let _15: *mut i16;
let _16: *mut &'static mut [char; 6];
let _17: &'static mut u32;
let _18: [char; 5];
let _19: u8;
let _20: &'static &'static mut &'static mut f32;
let _21: *mut &'static mut [char; 6];
let _22: bool;
let _23: isize;
let _24: isize;
let _25: &'static mut f64;
let _26: bool;
let _27: Adt80;
let _28: &'static mut *const u64;
let _29: *const Adt22;
let _30: *const u32;
let _31: &'static mut [char; 6];
let _32: f32;
let _33: Adt41;
let _34: f64;
let _35: f32;
let _36: &'static isize;
let _37: isize;
let _38: i128;
let _39: f64;
let _40: Adt80;
let _41: f32;
let _42: (&'static isize, *const (u64,), i32);
let _43: *const (u64,);
let _44: char;
let _45: [i64; 6];
let _46: isize;
let _47: &'static mut (u64,);
let _48: *mut *const &'static mut f32;
let _49: isize;
let _50: i128;
let _51: &'static f32;
let _52: &'static mut (u64,);
let _53: isize;
let _54: ();
let _55: ();
{
_4 = !_3;
_1 = _2 * _9;
_2 = _6;
_7 = '\u{b5aa6}';
Goto(bb1)
}
bb1 = {
_9 = -_1;
_5 = -_1;
_6 = 7_usize as f64;
_1 = _5;
_4 = _3;
Goto(bb2)
}
bb2 = {
_6 = _1 + _2;
_4 = _3 - _3;
_5 = -_6;
_2 = _9 - _9;
_10 = 16786_u16 as i128;
_11.2 = 1149979926_i32 + 530817717_i32;
_8 = 160_u8;
_6 = _2 * _2;
_11.0 = &_3;
_5 = _2 * _2;
_5 = _9 - _9;
_6 = _5 + _2;
_4 = _3 - _3;
_10 = !(-30294530142470213630152628284248680894_i128);
_12.1 = [19942_u16,65096_u16,22729_u16,46774_u16,50385_u16];
_12.0 = _2 == _5;
_9 = -_2;
_8 = 146_u8;
_4 = _3 - _3;
_11.0 = &_4;
_8 = 218_u8;
_11.0 = &_3;
_8 = (-55_i8) as u8;
_10 = (-7466705973371298532763957500120473205_i128);
Goto(bb3)
}
bb3 = {
_12.0 = !false;
_8 = (-79_i8) as u8;
_6 = _5 + _9;
_12.1 = [46705_u16,11893_u16,52747_u16,37882_u16,24612_u16];
_12.0 = _6 > _1;
_1 = _6 * _9;
_12.0 = !false;
_3 = !_4;
_6 = _2 + _9;
_6 = _2;
_8 = 185_u8;
Goto(bb4)
}
bb4 = {
_12.0 = false;
_8 = 51_u8 | 53_u8;
_6 = _2;
_2 = _6 + _6;
_9 = _2 + _6;
_10 = !(-39985742050586213112288971933308275144_i128);
_5 = _1;
_3 = _12.0 as isize;
_11.0 = &_4;
_11.0 = &_3;
_8 = (-112_i8) as u8;
_12.0 = _6 == _6;
_19 = !_8;
_4 = _3;
_2 = -_6;
_4 = _10 as isize;
_19 = !_8;
_11.0 = &_4;
_5 = -_9;
_19 = _8 << _11.2;
_2 = -_5;
_7 = '\u{5aa7c}';
_6 = 61610_u16 as f64;
_10 = 3231748477282885262_i64 as i128;
_2 = _1;
_6 = _11.2 as f64;
Call(_19 = core::intrinsics::bswap(_8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = _3;
_4 = !_3;
_7 = '\u{d081a}';
_19 = _8;
_12.0 = !true;
_22 = _2 > _2;
_12.1 = [49703_u16,52064_u16,34573_u16,20432_u16,50476_u16];
_24 = _7 as isize;
_2 = -_5;
_9 = _1 - _2;
_11.2 = (-998317945_i32) ^ 1200685398_i32;
_12.0 = _22 & _22;
_23 = _9 as isize;
_22 = _12.0 & _12.0;
_19 = _8 - _8;
_6 = 47810_u16 as f64;
_7 = '\u{da227}';
_24 = _23 - _23;
_7 = '\u{62d0b}';
_11.0 = &_3;
_2 = _5 + _9;
_8 = !_19;
_9 = _2;
_9 = _23 as f64;
_7 = '\u{ea898}';
_2 = 3273373485635003722_usize as f64;
_23 = -_24;
Goto(bb6)
}
bb6 = {
_19 = _8 - _8;
_18 = [_7,_7,_7,_7,_7];
_26 = _9 < _5;
_25 = &mut _2;
_4 = -_24;
_6 = -_1;
_24 = 52_i8 as isize;
_11.2 = 1082111222_i32 - (-1701995124_i32);
(*_25) = 3814_i16 as f64;
(*_25) = -_1;
(*_25) = 37427334279707289942338840333634886239_u128 as f64;
(*_25) = 3031569463444805745_u64 as f64;
_22 = _12.0;
_19 = _7 as u8;
(*_25) = _1 * _5;
(*_25) = _6 - _1;
_11.0 = &_4;
(*_25) = _1;
(*_25) = _11.2 as f64;
_24 = _4 >> _23;
_11.2 = (-2034109992_i32);
_1 = _9 + _9;
_29 = core::ptr::addr_of!(RET);
_23 = _4 & _4;
Goto(bb7)
}
bb7 = {
(*_25) = -_5;
_12.0 = _26;
(*_25) = _11.2 as f64;
(*_25) = _6 - _9;
(*_25) = _1 - _1;
_22 = _12.0;
_1 = (*_25) - _5;
(*_25) = -_6;
_11.2 = _7 as i32;
_10 = _7 as i128;
(*_25) = -_1;
_6 = (*_25) * (*_25);
_12.0 = _26;
_6 = (*_25);
(*_25) = _6 * _6;
(*_25) = -_5;
_3 = _23;
(*_25) = _9;
_11.2 = !(-362211909_i32);
_6 = _3 as f64;
(*_25) = _9 - _1;
(*_25) = _1 - _1;
(*_25) = _1 + _6;
_9 = _23 as f64;
_19 = _8 | _8;
_11.2 = (-1111581500_i32) ^ (-2004957022_i32);
(*_25) = -_6;
_22 = (*_25) == (*_25);
_11.2 = 1060215754_i32 & 1191521949_i32;
Goto(bb8)
}
bb8 = {
(*_25) = _6 - _5;
(*_25) = 1683410190_u32 as f64;
_19 = _8 >> _23;
(*_25) = _5 - _1;
(*_25) = _6 + _1;
(*_25) = _5;
(*_25) = _9;
_12.0 = (*_25) <= _6;
(*_25) = -_1;
_9 = _5 - _6;
_24 = _23 ^ _4;
_11.0 = &_23;
_4 = -_23;
(*_25) = _1 * _5;
(*_25) = _9;
_9 = (*_25);
_9 = (*_25) - (*_25);
_3 = _23 >> _23;
_11.2 = (-2079484810_i32);
(*_25) = _5 * _5;
_18 = [_7,_7,_7,_7,_7];
_7 = '\u{ddc76}';
_4 = _10 as isize;
_12.0 = !_26;
(*_25) = _6;
(*_25) = _10 as f64;
_23 = _24 + _3;
(*_25) = _5 + _9;
(*_25) = 23215_i16 as f64;
_4 = _23;
match _11.2 {
0 => bb9,
340282366920938463463374607429688726646 => bb11,
_ => bb10
}
}
bb9 = {
_12.0 = !false;
_8 = (-79_i8) as u8;
_6 = _5 + _9;
_12.1 = [46705_u16,11893_u16,52747_u16,37882_u16,24612_u16];
_12.0 = _6 > _1;
_1 = _6 * _9;
_12.0 = !false;
_3 = !_4;
_6 = _2 + _9;
_6 = _2;
_8 = 185_u8;
Goto(bb4)
}
bb10 = {
_9 = -_1;
_5 = -_1;
_6 = 7_usize as f64;
_1 = _5;
_4 = _3;
Goto(bb2)
}
bb11 = {
_25 = &mut _1;
_35 = (-30_i8) as f32;
(*_25) = _6 * _5;
_18 = [_7,_7,_7,_7,_7];
_34 = _9;
_9 = (*_25) + (*_25);
(*_25) = _5 - _34;
Goto(bb12)
}
bb12 = {
(*_25) = _34 - _9;
(*_25) = -_5;
(*_29) = Adt22::Variant3 { fld0: _26,fld1: _7,fld2: (-689600124308524265_i64),fld3: 4721645199440651419_u64,fld4: _12.1,fld5: _35 };
_34 = _11.2 as f64;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [49631_u16,5891_u16,37006_u16,32717_u16,6464_u16];
Goto(bb13)
}
bb13 = {
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [1060_u16,33285_u16,48003_u16,28512_u16,5429_u16];
_10 = !142028577259692930115862710794000038087_i128;
place!(Field::<i64>(Variant((*_29), 3), 2)) = (-7695297641259433391_i64) + 7570378713633479201_i64;
_6 = (*_25);
Goto(bb14)
}
bb14 = {
_32 = (*_25) as f32;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [50667_u16,28226_u16,18503_u16,18596_u16,56926_u16];
(*_25) = -_6;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = _12.1;
(*_29) = Adt22::Variant3 { fld0: _22,fld1: _7,fld2: 7587816334031553913_i64,fld3: 2097315149987899707_u64,fld4: _12.1,fld5: _32 };
place!(Field::<i64>(Variant((*_29), 3), 2)) = 6000974698172859080_i64;
place!(Field::<i64>(Variant((*_29), 3), 2)) = (-15395594633170189_i64) << _19;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [56961_u16,34367_u16,41631_u16,496_u16,40006_u16];
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [2611_u16,27782_u16,4543_u16,8075_u16,32379_u16];
place!(Field::<u64>(Variant((*_29), 3), 3)) = 1887664724314588619_u64;
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [6616_u16,44961_u16,50267_u16,7243_u16,34391_u16];
place!(Field::<u64>(Variant((*_29), 3), 3)) = 569387109467618526_u64 - 846747255590463885_u64;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [24353_u16,5332_u16,45797_u16,9766_u16,4744_u16];
(*_25) = -_5;
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
_25 = &mut _5;
(*_29) = Adt22::Variant3 { fld0: _26,fld1: _7,fld2: 315311026516547057_i64,fld3: 8091041548488263404_u64,fld4: _12.1,fld5: _32 };
_23 = _24;
place!(Field::<u64>(Variant((*_29), 3), 3)) = 836886890017270588_u64 * 11633181000701178081_u64;
_19 = _8 + _8;
place!(Field::<bool>(Variant((*_29), 3), 0)) = _22;
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
Goto(bb15)
}
bb15 = {
place!(Field::<u64>(Variant(RET, 3), 3)) = !12478168136743361304_u64;
place!(Field::<u64>(Variant((*_29), 3), 3)) = 25577_i16 as u64;
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
place!(Field::<bool>(Variant((*_29), 3), 0)) = _26;
_12.0 = Field::<bool>(Variant((*_29), 3), 0);
(*_25) = _6 - _9;
_39 = _6 - _6;
(*_25) = -_6;
place!(Field::<i64>(Variant((*_29), 3), 2)) = Field::<char>(Variant((*_29), 3), 1) as i64;
(*_25) = _9 - _39;
place!(Field::<f32>(Variant((*_29), 3), 5)) = _32;
(*_25) = _39;
(*_25) = _39;
place!(Field::<i64>(Variant((*_29), 3), 2)) = _10 as i64;
place!(Field::<u64>(Variant((*_29), 3), 3)) = 130837550504756272_u64 ^ 5748743868234585587_u64;
(*_29) = Adt22::Variant3 { fld0: _12.0,fld1: _7,fld2: (-4536018469825781155_i64),fld3: 7693246182438494115_u64,fld4: _12.1,fld5: _32 };
place!(Field::<bool>(Variant((*_29), 3), 0)) = !_26;
Call((*_29) = fn19(Move(_11.0), Move(_29), Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)) = (82_i8, 17869880729315575555_u64, 26695_u16);
_39 = -_6;
_42.2 = _11.2 & _11.2;
_37 = _7 as isize;
_26 = _12.0;
_44 = _7;
_42.0 = &_24;
_11.0 = &_24;
_19 = !_8;
_34 = _6 - _6;
_18 = [_7,_44,_44,_7,_7];
_44 = _7;
_8 = 3277933299_u32 as u8;
_26 = _12.0;
_37 = _3;
match Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1 {
0 => bb1,
1 => bb8,
2 => bb15,
3 => bb11,
4 => bb5,
5 => bb17,
6 => bb18,
17869880729315575555 => bb20,
_ => bb19
}
}
bb17 = {
_12.0 = !false;
_8 = (-79_i8) as u8;
_6 = _5 + _9;
_12.1 = [46705_u16,11893_u16,52747_u16,37882_u16,24612_u16];
_12.0 = _6 > _1;
_1 = _6 * _9;
_12.0 = !false;
_3 = !_4;
_6 = _2 + _9;
_6 = _2;
_8 = 185_u8;
Goto(bb4)
}
bb18 = {
_19 = _8 - _8;
_18 = [_7,_7,_7,_7,_7];
_26 = _9 < _5;
_25 = &mut _2;
_4 = -_24;
_6 = -_1;
_24 = 52_i8 as isize;
_11.2 = 1082111222_i32 - (-1701995124_i32);
(*_25) = 3814_i16 as f64;
(*_25) = -_1;
(*_25) = 37427334279707289942338840333634886239_u128 as f64;
(*_25) = 3031569463444805745_u64 as f64;
_22 = _12.0;
_19 = _7 as u8;
(*_25) = _1 * _5;
(*_25) = _6 - _1;
_11.0 = &_4;
(*_25) = _1;
(*_25) = _11.2 as f64;
_24 = _4 >> _23;
_11.2 = (-2034109992_i32);
_1 = _9 + _9;
_29 = core::ptr::addr_of!(RET);
_23 = _4 & _4;
Goto(bb7)
}
bb19 = {
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [1060_u16,33285_u16,48003_u16,28512_u16,5429_u16];
_10 = !142028577259692930115862710794000038087_i128;
place!(Field::<i64>(Variant((*_29), 3), 2)) = (-7695297641259433391_i64) + 7570378713633479201_i64;
_6 = (*_25);
Goto(bb14)
}
bb20 = {
_10 = 39389224055212599290767096298559503120_i128 | 6330697472236111306958110867189151127_i128;
_42.2 = _11.2 * _11.2;
_19 = _8;
_8 = _11.2 as u8;
_23 = _4 - _24;
_24 = _32 as isize;
_4 = _3;
match Field::<(i8, u64, u16)>(Variant(RET, 2), 0).2 {
0 => bb21,
1 => bb22,
2 => bb23,
3 => bb24,
4 => bb25,
26695 => bb27,
_ => bb26
}
}
bb21 = {
place!(Field::<char>(Variant((*_29), 3), 1)) = _7;
place!(Field::<[u16; 5]>(Variant((*_29), 3), 4)) = [1060_u16,33285_u16,48003_u16,28512_u16,5429_u16];
_10 = !142028577259692930115862710794000038087_i128;
place!(Field::<i64>(Variant((*_29), 3), 2)) = (-7695297641259433391_i64) + 7570378713633479201_i64;
_6 = (*_25);
Goto(bb14)
}
bb22 = {
_19 = _8 - _8;
_18 = [_7,_7,_7,_7,_7];
_26 = _9 < _5;
_25 = &mut _2;
_4 = -_24;
_6 = -_1;
_24 = 52_i8 as isize;
_11.2 = 1082111222_i32 - (-1701995124_i32);
(*_25) = 3814_i16 as f64;
(*_25) = -_1;
(*_25) = 37427334279707289942338840333634886239_u128 as f64;
(*_25) = 3031569463444805745_u64 as f64;
_22 = _12.0;
_19 = _7 as u8;
(*_25) = _1 * _5;
(*_25) = _6 - _1;
_11.0 = &_4;
(*_25) = _1;
(*_25) = _11.2 as f64;
_24 = _4 >> _23;
_11.2 = (-2034109992_i32);
_1 = _9 + _9;
_29 = core::ptr::addr_of!(RET);
_23 = _4 & _4;
Goto(bb7)
}
bb23 = {
_12.0 = !false;
_8 = (-79_i8) as u8;
_6 = _5 + _9;
_12.1 = [46705_u16,11893_u16,52747_u16,37882_u16,24612_u16];
_12.0 = _6 > _1;
_1 = _6 * _9;
_12.0 = !false;
_3 = !_4;
_6 = _2 + _9;
_6 = _2;
_8 = 185_u8;
Goto(bb4)
}
bb24 = {
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)) = (82_i8, 17869880729315575555_u64, 26695_u16);
_39 = -_6;
_42.2 = _11.2 & _11.2;
_37 = _7 as isize;
_26 = _12.0;
_44 = _7;
_42.0 = &_24;
_11.0 = &_24;
_19 = !_8;
_34 = _6 - _6;
_18 = [_7,_44,_44,_7,_7];
_44 = _7;
_8 = 3277933299_u32 as u8;
_26 = _12.0;
_37 = _3;
match Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1 {
0 => bb1,
1 => bb8,
2 => bb15,
3 => bb11,
4 => bb5,
5 => bb17,
6 => bb18,
17869880729315575555 => bb20,
_ => bb19
}
}
bb25 = {
_4 = _3;
_4 = !_3;
_7 = '\u{d081a}';
_19 = _8;
_12.0 = !true;
_22 = _2 > _2;
_12.1 = [49703_u16,52064_u16,34573_u16,20432_u16,50476_u16];
_24 = _7 as isize;
_2 = -_5;
_9 = _1 - _2;
_11.2 = (-998317945_i32) ^ 1200685398_i32;
_12.0 = _22 & _22;
_23 = _9 as isize;
_22 = _12.0 & _12.0;
_19 = _8 - _8;
_6 = 47810_u16 as f64;
_7 = '\u{da227}';
_24 = _23 - _23;
_7 = '\u{62d0b}';
_11.0 = &_3;
_2 = _5 + _9;
_8 = !_19;
_9 = _2;
_9 = _23 as f64;
_7 = '\u{ea898}';
_2 = 3273373485635003722_usize as f64;
_23 = -_24;
Goto(bb6)
}
bb26 = {
(*_25) = _6 - _5;
(*_25) = 1683410190_u32 as f64;
_19 = _8 >> _23;
(*_25) = _5 - _1;
(*_25) = _6 + _1;
(*_25) = _5;
(*_25) = _9;
_12.0 = (*_25) <= _6;
(*_25) = -_1;
_9 = _5 - _6;
_24 = _23 ^ _4;
_11.0 = &_23;
_4 = -_23;
(*_25) = _1 * _5;
(*_25) = _9;
_9 = (*_25);
_9 = (*_25) - (*_25);
_3 = _23 >> _23;
_11.2 = (-2079484810_i32);
(*_25) = _5 * _5;
_18 = [_7,_7,_7,_7,_7];
_7 = '\u{ddc76}';
_4 = _10 as isize;
_12.0 = !_26;
(*_25) = _6;
(*_25) = _10 as f64;
_23 = _24 + _3;
(*_25) = _5 + _9;
(*_25) = 23215_i16 as f64;
_4 = _23;
match _11.2 {
0 => bb9,
340282366920938463463374607429688726646 => bb11,
_ => bb10
}
}
bb27 = {
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).0 = !35_i8;
_38 = 3600125887_u32 as i128;
_45 = [2072165542787863154_i64,1235181075249033569_i64,5681924667644243560_i64,4546477776347486020_i64,2810007942018247953_i64,6451996096452841788_i64];
_38 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1 as i128;
_18 = [_7,_44,_44,_44,_7];
_22 = _26 | _12.0;
_38 = _10 >> _24;
_42.0 = &_3;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).1 = 16201177701586964542_u64 - 6770297913607419770_u64;
_18 = [_44,_7,_7,_7,_44];
_25 = &mut _6;
_35 = _32 + _32;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).2 = _38 as u16;
(*_25) = _9 + _39;
(*_25) = -_9;
_12.0 = (*_25) <= (*_25);
Goto(bb28)
}
bb28 = {
Call(_54 = dump_var(Move(_8), Move(_38), Move(_26), Move(_45)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_54 = dump_var(Move(_22), Move(_18), Move(_37), Move(_7)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: &'static isize,mut _2: *const Adt22,mut _3: &'static mut f64) -> Adt22 {
mir! {
type RET = Adt22;
let _4: bool;
let _5: *const (u64,);
let _6: &'static mut (u64,);
let _7: *const *const (u64,);
let _8: *mut [u128; 7];
let _9: [i16; 7];
let _10: i128;
let _11: &'static mut *const u64;
let _12: u128;
let _13: bool;
let _14: (&'static isize, *const (u64,), i32);
let _15: *const u8;
let _16: u128;
let _17: char;
let _18: f32;
let _19: f64;
let _20: &'static isize;
let _21: i128;
let _22: char;
let _23: &'static mut *const u64;
let _24: *const Adt22;
let _25: *mut *const &'static mut f32;
let _26: [u32; 7];
let _27: &'static mut f32;
let _28: i32;
let _29: bool;
let _30: *mut u128;
let _31: i128;
let _32: *const u32;
let _33: u64;
let _34: f64;
let _35: *mut *const &'static mut f32;
let _36: f64;
let _37: &'static &'static mut *const u64;
let _38: (f64, f64, u128, i16);
let _39: *const u32;
let _40: [i64; 6];
let _41: isize;
let _42: *const (u64,);
let _43: *mut u128;
let _44: f32;
let _45: [u32; 7];
let _46: usize;
let _47: *const u64;
let _48: [isize; 6];
let _49: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _50: u64;
let _51: &'static mut u32;
let _52: isize;
let _53: *mut [u128; 7];
let _54: char;
let _55: i16;
let _56: u128;
let _57: &'static mut f64;
let _58: &'static &'static mut *const u64;
let _59: *mut [u16; 5];
let _60: u64;
let _61: u8;
let _62: bool;
let _63: f32;
let _64: u8;
let _65: *mut [u128; 7];
let _66: u128;
let _67: &'static f32;
let _68: [char; 6];
let _69: *const Adt22;
let _70: u64;
let _71: ((i8, u64, u16), (f64, f64, u128, i16), *mut *const &'static mut f32);
let _72: f32;
let _73: *mut &'static mut [char; 6];
let _74: i16;
let _75: Adt80;
let _76: bool;
let _77: (&'static isize, *const (u64,), i32);
let _78: f64;
let _79: &'static isize;
let _80: [u8; 4];
let _81: *mut u128;
let _82: &'static mut u32;
let _83: ();
let _84: ();
{
_2 = core::ptr::addr_of!(RET);
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
Goto(bb1)
}
bb1 = {
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
Goto(bb2)
}
bb2 = {
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!(RET);
_4 = 7487293010678668936_usize > 13584801359956047404_usize;
_4 = !false;
_2 = core::ptr::addr_of!((*_2));
_4 = 17727_u16 < 12331_u16;
Goto(bb3)
}
bb3 = {
_2 = core::ptr::addr_of!((*_2));
_4 = false;
_4 = false | false;
_2 = core::ptr::addr_of!((*_2));
_4 = !false;
_4 = true;
_2 = core::ptr::addr_of!((*_2));
_4 = false ^ false;
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_7 = core::ptr::addr_of!(_5);
_9 = [21302_i16,13425_i16,16001_i16,(-25482_i16),(-4135_i16),16721_i16,1499_i16];
_4 = !true;
Goto(bb4)
}
bb4 = {
_2 = core::ptr::addr_of!((*_2));
_9 = [18491_i16,(-8876_i16),10837_i16,(-3396_i16),(-7508_i16),(-11751_i16),(-13068_i16)];
_9 = [(-32580_i16),(-19385_i16),(-7728_i16),17413_i16,19449_i16,(-6104_i16),26299_i16];
_2 = core::ptr::addr_of!(RET);
_7 = core::ptr::addr_of!((*_7));
_7 = core::ptr::addr_of!((*_7));
_9 = [11158_i16,31977_i16,6850_i16,13202_i16,(-22745_i16),3607_i16,(-8598_i16)];
_12 = !310798599390643605718534240151622807049_u128;
_13 = _4;
_4 = _13;
_14.2 = !(-791382963_i32);
_14.2 = -(-629287541_i32);
_10 = !40007168329471724922247989190326771975_i128;
_13 = _4;
_16 = _12;
_4 = _13 ^ _13;
Goto(bb5)
}
bb5 = {
_13 = !_4;
_2 = core::ptr::addr_of!((*_2));
_7 = core::ptr::addr_of!((*_7));
_10 = -38141131487570401113585091590172271136_i128;
_12 = _16 >> _14.2;
_16 = 31753_i16 as u128;
Goto(bb6)
}
bb6 = {
_10 = -122302378104620871074757874859352450245_i128;
_7 = core::ptr::addr_of!(_14.1);
_14.2 = 1755142544_i32 ^ 1537970620_i32;
_4 = _14.2 == _14.2;
_10 = '\u{835e7}' as i128;
_7 = core::ptr::addr_of!((*_7));
_2 = core::ptr::addr_of!((*_2));
Goto(bb7)
}
bb7 = {
_17 = '\u{f0bba}';
_13 = _4;
_4 = _17 == _17;
_4 = !_13;
_12 = _16;
_9 = [(-29599_i16),14354_i16,23938_i16,23559_i16,(-6557_i16),(-12598_i16),(-24143_i16)];
_2 = core::ptr::addr_of!((*_2));
_7 = core::ptr::addr_of!((*_7));
_14.2 = 974139451_i32 & (-744118529_i32);
_18 = (-9223372036854775808_isize) as f32;
_9 = [30827_i16,(-11818_i16),24008_i16,20788_i16,(-27958_i16),14810_i16,(-16068_i16)];
_9 = [24988_i16,(-1410_i16),(-852_i16),24194_i16,5610_i16,31400_i16,2452_i16];
_16 = _12;
_16 = 18_i8 as u128;
_2 = core::ptr::addr_of!((*_2));
_16 = _12 ^ _12;
_14.2 = (-1311826449_i32);
_21 = _16 as i128;
_13 = !_4;
_4 = !_13;
_16 = !_12;
_21 = _10 >> _10;
match _14.2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607430456385007 => bb8,
_ => bb6
}
}
bb8 = {
_18 = (-54_i8) as f32;
_4 = _21 != _21;
Goto(bb9)
}
bb9 = {
_19 = 26591_i16 as f64;
_4 = _13 & _13;
_22 = _17;
_4 = !_13;
_10 = _21 * _21;
_4 = _18 != _18;
_16 = _12 & _12;
_7 = core::ptr::addr_of!((*_7));
_17 = _22;
_21 = _10 ^ _10;
_9 = [(-28956_i16),(-18379_i16),(-9359_i16),27720_i16,5718_i16,(-3336_i16),28277_i16];
_16 = _12;
_14.2 = 1952785221_i32 & (-721683559_i32);
_16 = _12 - _12;
_14.2 = (-741148347_i32) + (-1338935701_i32);
_4 = _13 & _13;
_18 = 8451308063755061378_i64 as f32;
_19 = 17607627233049415631_u64 as f64;
_2 = core::ptr::addr_of!((*_2));
_10 = _21;
_14.2 = !1074436814_i32;
_22 = _17;
_21 = !_10;
_22 = _17;
_19 = _16 as f64;
Goto(bb10)
}
bb10 = {
_13 = _10 != _10;
_18 = (-59_i8) as f32;
_7 = core::ptr::addr_of!((*_7));
_19 = 2808250964_u32 as f64;
_14.2 = -(-1801545526_i32);
_24 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_14.2 = 16539377518288964381_u64 as i32;
_17 = _22;
_22 = _17;
_21 = _10;
_13 = _4 & _4;
_9 = [3482_i16,(-21659_i16),(-12359_i16),16502_i16,(-23952_i16),24683_i16,(-21016_i16)];
_22 = _17;
_7 = core::ptr::addr_of!((*_7));
_17 = _22;
_7 = core::ptr::addr_of!((*_7));
_22 = _17;
_9 = [(-6827_i16),5393_i16,28547_i16,14948_i16,(-6597_i16),(-270_i16),6766_i16];
_12 = _16;
_22 = _17;
_13 = !_4;
_12 = !_16;
_12 = _16 & _16;
_22 = _17;
_13 = _4;
_24 = core::ptr::addr_of!((*_2));
_21 = _10 >> _10;
_19 = (-9223372036854775808_isize) as f64;
_7 = core::ptr::addr_of!((*_7));
Goto(bb11)
}
bb11 = {
_12 = _18 as u128;
_24 = core::ptr::addr_of!((*_24));
Goto(bb12)
}
bb12 = {
_21 = _10 & _10;
_26 = [3738378533_u32,4068999758_u32,2216827940_u32,2669804865_u32,728411546_u32,571604717_u32,243290869_u32];
_22 = _17;
_3 = &mut _19;
(*_3) = 3913818765085856117_u64 as f64;
(*_3) = 20084_u16 as f64;
(*_3) = 126_u8 as f64;
(*_3) = 13232_u16 as f64;
_21 = _10;
_28 = _14.2 + _14.2;
_7 = core::ptr::addr_of!((*_7));
(*_3) = 11_isize as f64;
_29 = !_13;
(*_3) = 56452_u16 as f64;
_9 = [31344_i16,25749_i16,15481_i16,(-15829_i16),(-28433_i16),(-30525_i16),4933_i16];
_10 = 11_i8 as i128;
(*_3) = 6_u8 as f64;
(*_3) = 4_usize as f64;
(*_3) = (-26561_i16) as f64;
(*_3) = 0_usize as f64;
Goto(bb13)
}
bb13 = {
(*_3) = _16 as f64;
(*_3) = 122_i8 as f64;
_26 = [3536772798_u32,898454932_u32,2421775654_u32,1830109653_u32,476196385_u32,626684115_u32,2759904622_u32];
_27 = &mut _18;
(*_3) = _21 as f64;
(*_27) = 198_u8 as f32;
(*_27) = 26_isize as f32;
Goto(bb14)
}
bb14 = {
(*_3) = 9223372036854775807_isize as f64;
(*_3) = _21 as f64;
_26 = [3274587085_u32,2006900186_u32,363942557_u32,3271710891_u32,249100299_u32,2440697816_u32,2260226389_u32];
(*_27) = 8286_u16 as f32;
(*_3) = _28 as f64;
Goto(bb15)
}
bb15 = {
(*_27) = 1153131365684983294_i64 as f32;
(*_3) = 20754_i16 as f64;
(*_3) = 56089_u16 as f64;
(*_27) = _16 as f32;
_10 = _21 - _21;
(*_3) = _12 as f64;
(*_3) = 64_i8 as f64;
(*_3) = 32_isize as f64;
Goto(bb16)
}
bb16 = {
(*_27) = 13605270728661815214_u64 as f32;
_28 = _14.2 >> _10;
Goto(bb17)
}
bb17 = {
(*_3) = (-51_i8) as f64;
(*_27) = 7_usize as f32;
(*_27) = 8197870920688797077_usize as f32;
_28 = _14.2 << _21;
(*_27) = _16 as f32;
_17 = _22;
(*_3) = 101_u8 as f64;
(*_3) = (-3340911801308049525_i64) as f64;
_21 = _10;
_13 = !_29;
_31 = 59968_u16 as i128;
Goto(bb18)
}
bb18 = {
(*_3) = 3821448484_u32 as f64;
(*_3) = _28 as f64;
_34 = (*_3) - (*_3);
(*_3) = _34 + _34;
(*_27) = 19797_i16 as f32;
(*_27) = (-4892437506661598407_i64) as f32;
(*_27) = 547563345734875818_u64 as f32;
(*_27) = (-6235323781876547121_i64) as f32;
_10 = _21 >> _28;
_30 = core::ptr::addr_of_mut!(_12);
(*_27) = (-5421803049660628960_i64) as f32;
(*_27) = (*_30) as f32;
(*_27) = 15442298285745607613_u64 as f32;
Goto(bb19)
}
bb19 = {
_36 = (*_3);
(*_3) = _36 - _36;
(*_3) = _34;
(*_3) = -_34;
(*_30) = !_16;
(*_30) = _16;
(*_27) = _28 as f32;
(*_30) = _16;
(*_3) = _34;
(*_27) = 2026324068_u32 as f32;
_29 = (*_3) >= (*_3);
(*_30) = !_16;
(*_27) = _10 as f32;
Goto(bb20)
}
bb20 = {
_30 = core::ptr::addr_of_mut!((*_30));
(*_27) = 1_usize as f32;
(*_30) = _16 >> _28;
_2 = core::ptr::addr_of!((*_24));
(*_3) = _36;
(*_3) = -_36;
(*_3) = -_36;
(*_27) = _21 as f32;
_40 = [(-4340293384005190798_i64),(-4145455908319053902_i64),8323505073939976898_i64,(-4735199879212199259_i64),(-3489820535407802040_i64),(-7241582387001544164_i64)];
_36 = (*_3);
Goto(bb21)
}
bb21 = {
(*_27) = 103_isize as f32;
_30 = core::ptr::addr_of_mut!((*_30));
_38.1 = -(*_3);
_14.2 = 13587_i16 as i32;
_40 = [(-4302626715167180699_i64),(-2530399740787942592_i64),4153191594525998487_i64,(-1487119577748951157_i64),2307980028583141761_i64,5037915238131246439_i64];
(*_27) = 13276_u16 as f32;
(*_30) = _16 - _16;
(*_27) = 238_u8 as f32;
_26 = [2844553105_u32,3073042482_u32,2576091738_u32,3448211915_u32,724045452_u32,2227914718_u32,169054848_u32];
(*_3) = _38.1 * _36;
(*_3) = 216_u8 as f64;
(*_30) = !_16;
_38.0 = _38.1 + _34;
(*_27) = _28 as f32;
(*_30) = _16 * _16;
(*_3) = _38.0;
(*_30) = (-27184_i16) as u128;
_38 = ((*_3), (*_3), (*_30), (-13345_i16));
_38.2 = !(*_30);
(*_30) = _38.2 >> _38.3;
_44 = (*_27);
_33 = 6823718929955488771_u64 | 9872922989867419793_u64;
_10 = _21 + _21;
_43 = core::ptr::addr_of_mut!((*_30));
_41 = 9223372036854775807_isize + (-9223372036854775808_isize);
_34 = -(*_3);
(*_30) = _38.2 * _16;
(*_27) = 63417_u16 as f32;
Goto(bb22)
}
bb22 = {
(*_27) = -_44;
_46 = 1_usize * 4_usize;
(*_30) = _38.3 as u128;
(*_27) = -_44;
(*_30) = _16 >> _10;
(*_3) = -_38.1;
_49.1.2 = (*_27) as u128;
_45 = [3555464216_u32,3425755206_u32,3274775045_u32,1452621384_u32,2769264819_u32,1292751796_u32,1966241252_u32];
(*_30) = _16;
(*_27) = _44;
(*_27) = _41 as f32;
_49.0 = ((-117_i8), _33, 3890_u16);
(*_27) = _44;
Goto(bb23)
}
bb23 = {
(*_3) = -_36;
(*_3) = _38.1 + _34;
_2 = core::ptr::addr_of!((*_2));
_49.0.1 = _33;
_38.1 = (*_3) * (*_3);
(*_24) = Adt22::Variant2 { fld0: _49.0 };
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _17 as u64;
(*_3) = _36 - _34;
_49.0.0 = _22 as i8;
_38.2 = (*_30);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 / _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 + _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 % _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 - _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _33 >> (*_30);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = !_49.0.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = !_33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
(*_30) = _49.1.2;
_20 = &_41;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = !_49.0.2;
Goto(bb24)
}
bb24 = {
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _49.0.1 | _33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 ^ _49.0.0;
_44 = (*_30) as f32;
_54 = _22;
_22 = _54;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
(*_27) = _44 - _44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = (*_3) as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
_50 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1;
_21 = 2215915763_u32 as i128;
(*_27) = (*_20) as f32;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
_49.0.1 = Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 - _49.0.0;
_49.1.3 = _38.3 - _38.3;
(*_24) = Adt22::Variant2 { fld0: _49.0 };
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _49.0.1, _49.0.2);
_21 = 126_u8 as i128;
_41 = _38.3 as isize;
(*_30) = _49.1.3 as u128;
_49.1 = ((*_3), (*_3), (*_30), _38.3);
(*_30) = !_49.1.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = -_49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _49.0.1 ^ _50;
Goto(bb25)
}
bb25 = {
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = (-8846944773736784699_i64) as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 * _49.0.0;
_41 = 9223372036854775807_isize;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _33 - _49.0.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _22 as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 >> (*_30);
(*_3) = _34 * _38.1;
(*_27) = _44 * _44;
_48 = [_41,_41,_41,_41,_41,_41];
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _50 - _50;
(*_3) = _49.1.0 * _49.1.0;
(*_30) = _49.1.2 >> Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).2;
_38.1 = (*_3) * (*_3);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
_33 = Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).1 ^ Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).1;
(*_27) = _44 * _44;
(*_27) = -_44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).2 = _49.0.2;
(*_27) = _44 - _44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
(*_30) = _10 as u128;
(*_30) = _49.1.2;
_62 = (*_27) != (*_27);
match _49.1.3 {
0 => bb23,
1 => bb7,
2 => bb14,
3 => bb11,
4 => bb26,
340282366920938463463374607431768198111 => bb28,
_ => bb27
}
}
bb26 = {
_18 = (-54_i8) as f32;
_4 = _21 != _21;
Goto(bb9)
}
bb27 = {
(*_3) = -_36;
(*_3) = _38.1 + _34;
_2 = core::ptr::addr_of!((*_2));
_49.0.1 = _33;
_38.1 = (*_3) * (*_3);
(*_24) = Adt22::Variant2 { fld0: _49.0 };
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _17 as u64;
(*_3) = _36 - _34;
_49.0.0 = _22 as i8;
_38.2 = (*_30);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 / _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 + _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 % _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 - _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _33 >> (*_30);
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = !_49.0.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = !_33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
(*_30) = _49.1.2;
_20 = &_41;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = !_49.0.2;
Goto(bb24)
}
bb28 = {
_55 = _49.1.3 ^ _49.1.3;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = !_49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = 104_u8 as u64;
_47 = core::ptr::addr_of!(place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).1);
(*_27) = _21 as f32;
(*_47) = _49.0.1 | _50;
(*_27) = _44 * _44;
(*_27) = _44 - _44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
(*_3) = _49.1.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _49.0.1, _49.0.2);
_14.0 = &_41;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _49.0.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = 146_u8 as i8;
(*_3) = _38.1 + _49.1.1;
(*_30) = !_49.1.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2;
_3 = &mut _34;
_13 = (*_27) != (*_27);
match _49.1.3 {
340282366920938463463374607431768198111 => bb30,
_ => bb29
}
}
bb29 = {
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _49.0.1 | _33;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 ^ _49.0.0;
_44 = (*_30) as f32;
_54 = _22;
_22 = _54;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
(*_27) = _44 - _44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = (*_3) as u64;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
_50 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1;
_21 = 2215915763_u32 as i128;
(*_27) = (*_20) as f32;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
_49.0.1 = Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.0.0 - _49.0.0;
_49.1.3 = _38.3 - _38.3;
(*_24) = Adt22::Variant2 { fld0: _49.0 };
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _49.0.1, _49.0.2);
_21 = 126_u8 as i128;
_41 = _38.3 as isize;
(*_30) = _49.1.3 as u128;
_49.1 = ((*_3), (*_3), (*_30), _38.3);
(*_30) = !_49.1.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = -_49.0.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = _49.0.1 ^ _50;
Goto(bb25)
}
bb30 = {
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 * _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = (_49.0.0, _33, _49.0.2);
_28 = _14.2;
(*_30) = _49.1.2 | _49.1.2;
(*_27) = -_44;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _49.1.3 as i8;
_49.1.3 = -_38.3;
_40 = [(-6381388547784968664_i64),2332572731005098249_i64,4072604228584882441_i64,3091103966172021737_i64,(-8556806809652955857_i64),6206431513260570692_i64];
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)) = _49.0;
_54 = _22;
(*_27) = _44;
_48 = [_41,_41,_41,_41,_41,_41];
(*_30) = _49.1.2 ^ _49.1.2;
(*_30) = 5620474864661585942_i64 as u128;
_66 = _49.1.2 + _49.1.2;
(*_30) = _33 as u128;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).2 = _49.0.2 + _49.0.2;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = _41 as i8;
Goto(bb31)
}
bb31 = {
(*_24) = Adt22::Variant2 { fld0: _49.0 };
_16 = _49.1.2;
(*_3) = _49.1.1 + _38.1;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).0 = -_49.0.0;
(*_30) = !_66;
place!(Field::<(i8, u64, u16)>(Variant((*_24), 2), 0)).1 = Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).2 as u64;
(*_30) = _38.2 * _16;
(*_27) = _44 - _44;
_14.2 = _28;
(*_30) = _66 - _49.1.2;
(*_3) = _49.1.1;
(*_30) = !_49.1.2;
(*_3) = _38.1 + _49.1.0;
_60 = _33 * Field::<(i8, u64, u16)>(Variant((*_24), 2), 0).1;
_15 = core::ptr::addr_of!(_64);
_28 = _14.2 | _14.2;
(*_24) = Adt22::Variant2 { fld0: _49.0 };
(*_15) = (*_27) as u8;
_24 = Move(_2);
(*_3) = _49.1.1 - _38.1;
(*_27) = _44;
_49.0.0 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).0 | Field::<(i8, u64, u16)>(Variant(RET, 2), 0).0;
(*_15) = 237_u8;
_56 = (*_30);
_55 = -_49.1.3;
(*_3) = _49.1.0 + _38.1;
_61 = (*_15);
_23 = &mut _47;
match _38.3 {
0 => bb8,
340282366920938463463374607431768198111 => bb32,
_ => bb21
}
}
bb32 = {
(*_15) = _28 as u8;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)) = _49.0;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)) = (_49.0.0, _33, _49.0.2);
_71.1.2 = (*_30) + (*_30);
(*_3) = _49.1.1 * _49.1.1;
(*_3) = _38.1 + _38.1;
(*_23) = core::ptr::addr_of!(_70);
(*_15) = _61;
_49.0.0 = !Field::<(i8, u64, u16)>(Variant(RET, 2), 0).0;
(*_30) = _56;
(*_23) = core::ptr::addr_of!(_71.0.1);
(*_23) = core::ptr::addr_of!(_71.0.1);
(*_3) = _49.1.0;
(*_27) = _44;
_72 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1 as f32;
_71.0.2 = _17 as u16;
_43 = Move(_30);
Goto(bb33)
}
bb33 = {
(*_27) = _72 - _44;
(*_27) = _72 - _72;
(*_3) = _38.1 - _49.1.0;
(*_3) = -_38.1;
Goto(bb34)
}
bb34 = {
_14.0 = Move(_20);
_22 = _54;
_71.0.1 = Field::<(i8, u64, u16)>(Variant(RET, 2), 0).1;
(*_27) = _44;
_63 = (*_27);
_58 = &_23;
(*_27) = _72 - _63;
(*_27) = -_63;
_44 = -(*_27);
_71.1.3 = -_49.1.3;
(*_15) = _61;
_77.0 = &_41;
_68 = [_54,_17,_22,_17,_17,_22];
(*_23) = core::ptr::addr_of!(_50);
_71.1 = ((*_3), (*_3), _66, _49.1.3);
(*_27) = _63 * _63;
Goto(bb35)
}
bb35 = {
_50 = _13 as u64;
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).0 = _46 as i8;
(*_23) = core::ptr::addr_of!(_33);
_21 = _22 as i128;
(*_3) = _71.1.0 - _71.1.0;
_57 = Move(_3);
_20 = Move(_77.0);
(*_27) = _49.1.0 as f32;
(*_23) = core::ptr::addr_of!(_60);
(*_23) = core::ptr::addr_of!(_70);
_3 = &mut _71.1.0;
(*_27) = _72 + _44;
_49.1.2 = _12 << _56;
(*_23) = core::ptr::addr_of!(_33);
_77.0 = &_41;
_31 = _10;
_33 = _50;
_49.0.0 = (*_3) as i8;
(*_3) = _49.1.1 + _38.0;
_49.0.2 = !Field::<(i8, u64, u16)>(Variant(RET, 2), 0).2;
(*_27) = _63 + _63;
(*_3) = -_38.1;
Call(place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).0 = core::intrinsics::transmute(_29), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
_52 = _41;
_70 = _33;
_11 = &mut (*_23);
(*_27) = _63 + _44;
_49.1.1 = (*_3);
place!(Field::<(i8, u64, u16)>(Variant(RET, 2), 0)).1 = _70 & _50;
_7 = core::ptr::addr_of!((*_7));
(*_11) = core::ptr::addr_of!(_33);
(*_3) = _49.1.0 - _49.1.1;
(*_27) = _16 as f32;
_80 = [(*_15),(*_15),(*_15),(*_15)];
_69 = core::ptr::addr_of!(RET);
_49.1.1 = -(*_3);
(*_15) = _61 << Field::<(i8, u64, u16)>(Variant((*_69), 2), 0).0;
_38.3 = -_55;
(*_27) = _44 + _72;
_7 = core::ptr::addr_of!((*_7));
(*_27) = _44 - _63;
Goto(bb37)
}
bb37 = {
Call(_83 = dump_var(Move(_21), Move(_9), Move(_61), Move(_29)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_83 = dump_var(Move(_64), Move(_48), Move(_68), Move(_52)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_83 = dump_var(Move(_17), Move(_56), Move(_80), Move(_41)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_83 = dump_var(Move(_40), Move(_54), Move(_66), Move(_55)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(123_u8));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt22 {
Variant0{
fld0: bool,
fld1: f64,
fld2: isize,
fld3: f32,
fld4: usize,
fld5: u16,
fld6: (u64,),
fld7: u128,

},
Variant1{
fld0: (i8, u64, u16),
fld1: char,
fld2: u16,
fld3: (u64,),
fld4: i16,
fld5: i32,

},
Variant2{
fld0: (i8, u64, u16),

},
Variant3{
fld0: bool,
fld1: char,
fld2: i64,
fld3: u64,
fld4: [u16; 5],
fld5: f32,

}}
#[derive(Debug)]
pub struct Adt26 {
fld0: f32,
fld1: (u64,),
fld2: [u128; 7],
fld3: f64,
fld4: u64,
fld5: u16,
fld6: *const u8,
}
#[derive(Debug)]
pub enum Adt41 {
Variant0{
fld0: *const u8,
fld1: [u16; 5],
fld2: *const u32,
fld3: i8,
fld4: *mut i128,
fld5: *const Adt22,

},
Variant1{
fld0: [u128; 7],
fld1: f64,
fld2: (*const u64, (i8, u64, u16), Adt22, isize),
fld3: [u16; 5],
fld4: i16,

},
Variant2{
fld0: *const u64,
fld1: u32,
fld2: f32,
fld3: i8,
fld4: i64,

},
Variant3{
fld0: (bool, [u16; 5]),
fld1: *const u8,
fld2: u16,
fld3: *const Adt22,

}}
#[derive(Debug)]
pub struct Adt49 {
fld0: u64,
fld1: [isize; 6],
}
#[derive(Debug)]
pub struct Adt53 {
fld0: *const Adt22,
fld1: f64,
}
#[derive(Debug)]
pub enum Adt75 {
Variant0{
fld0: [char; 5],
fld1: [i16; 8],
fld2: [i16; 7],
fld3: (i8, u64, u16),

},
Variant1{
fld0: bool,
fld1: *const Adt22,
fld2: *mut i128,
fld3: [isize; 6],
fld4: (Adt26, Adt41),
fld5: (*mut [u16; 5], *const (u64,), *const Adt41),
fld6: (i8, u64, u16),

},
Variant2{
fld0: [u128; 7],
fld1: (f64, f64, u128, i16),
fld2: (i8, u64, u16),
fld3: *const *const (u64,),
fld4: (*mut [u16; 5], *const (u64,), *const Adt41),
fld5: *mut i16,
fld6: Adt41,
fld7: *const Adt41,

}}
#[derive(Debug)]
pub enum Adt78 {
Variant0{
fld0: *const u64,
fld1: *const u8,
fld2: (*mut [u16; 5], [u16; 5], u32, [u16; 5]),
fld3: (i8, u64, u16),
fld4: (Adt26, Adt41),
fld5: Adt41,

},
Variant1{
fld0: *const u8,
fld1: *const u32,
fld2: u32,
fld3: Adt53,
fld4: f64,
fld5: *mut [u128; 7],
fld6: [isize; 6],

}}
#[derive(Debug)]
pub enum Adt80 {
Variant0{
fld0: Adt53,
fld1: Adt75,
fld2: *const u32,
fld3: u32,
fld4: [u16; 5],
fld5: (*const u64, (i8, u64, u16), Adt22, isize),

},
Variant1{
fld0: (bool, [u16; 5]),
fld1: [i16; 7],
fld2: Adt26,
fld3: u16,
fld4: *const (u64,),
fld5: i128,

}}

