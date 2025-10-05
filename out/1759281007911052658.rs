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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> i16 {
mir! {
type RET = i16;
let _13: char;
let _14: [char; 1];
let _15: &'static i32;
let _16: ([u16; 4], f32);
let _17: Adt62;
let _18: &'static [u16; 4];
let _19: i16;
let _20: char;
let _21: *const Adt27;
let _22: (&'static mut bool, (Adt19,), *mut u128, &'static [i8; 3]);
let _23: &'static [char; 1];
let _24: f64;
let _25: u8;
let _26: &'static mut bool;
let _27: Adt27;
let _28: &'static Adt27;
let _29: u8;
let _30: char;
let _31: u32;
let _32: &'static mut bool;
let _33: u16;
let _34: u16;
let _35: [u16; 4];
let _36: char;
let _37: &'static mut (u64, [i128; 2]);
let _38: &'static mut u8;
let _39: [i8; 8];
let _40: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _41: &'static mut bool;
let _42: i128;
let _43: isize;
let _44: &'static u32;
let _45: &'static i32;
let _46: isize;
let _47: usize;
let _48: i128;
let _49: i128;
let _50: *mut isize;
let _51: &'static [i8; 3];
let _52: &'static (Adt19,);
let _53: f32;
let _54: bool;
let _55: f64;
let _56: f32;
let _57: isize;
let _58: [isize; 1];
let _59: *const Adt38;
let _60: (Adt19,);
let _61: &'static i64;
let _62: *const [bool; 7];
let _63: u32;
let _64: u32;
let _65: &'static i32;
let _66: i128;
let _67: isize;
let _68: *mut [u128; 6];
let _69: bool;
let _70: *const Adt20;
let _71: isize;
let _72: usize;
let _73: *const Adt20;
let _74: isize;
let _75: isize;
let _76: i32;
let _77: i128;
let _78: f64;
let _79: i16;
let _80: *mut u128;
let _81: isize;
let _82: f64;
let _83: Adt76;
let _84: f32;
let _85: *mut f32;
let _86: i128;
let _87: *const &'static [i8; 3];
let _88: [u128; 6];
let _89: Adt38;
let _90: i32;
let _91: &'static (Adt19,);
let _92: isize;
let _93: f32;
let _94: &'static mut &'static mut u8;
let _95: &'static &'static i64;
let _96: usize;
let _97: Adt62;
let _98: u32;
let _99: f64;
let _100: f32;
let _101: f64;
let _102: &'static mut &'static mut u8;
let _103: &'static &'static Adt27;
let _104: *mut Adt38;
let _105: bool;
let _106: u64;
let _107: [i32; 3];
let _108: f64;
let _109: &'static i64;
let _110: (*const isize,);
let _111: *const &'static [i8; 3];
let _112: *const [bool; 7];
let _113: i16;
let _114: bool;
let _115: char;
let _116: isize;
let _117: [u8; 7];
let _118: ([u16; 4], f32);
let _119: &'static [i8; 3];
let _120: ();
let _121: ();
{
_6 = (-1076273863_i32);
_3 = 213016558806510771062210853470176450830_u128 - 108988842383996422579415010106375563816_u128;
_5 = (-4835_i16) | 24294_i16;
_11 = 53692_u16 + 49266_u16;
RET = false as i16;
_13 = '\u{14eba}';
_10 = !28_u8;
_12 = 73285522_u32;
_1 = true | true;
_2 = _13;
_7 = !15044784456230869200_u64;
_2 = _13;
_6 = 932975033_i32 >> _7;
_2 = _13;
_9 = 0_usize;
_14[_9] = _2;
_14 = [_13];
_10 = 37_u8 >> _11;
_13 = _2;
RET = _5 >> _6;
_14[_9] = _13;
_4 = 63_i8 ^ (-34_i8);
_10 = 90_u8;
_6 = 1510157848_i32 << RET;
_8 = (-140200822563556383559080573692305113593_i128);
_3 = _11 as u128;
_12 = !4117096763_u32;
_16.0[_9] = _11 ^ _11;
Call(_14 = fn1(_8, _2, _12, _7, _12, RET, _13, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _13;
_16.0 = [_11,_11,_11,_11];
_4 = 56_i8 * (-50_i8);
_15 = &_6;
_16.0 = [_11,_11,_11,_11];
_9 = !4_usize;
_3 = 112999765583324069080529990255412187422_u128 & 289094199052503208882427765885163674060_u128;
_13 = _2;
_5 = (*_15) as i16;
_4 = (-77_i8) << (*_15);
_8 = (-149577695034732906529288648868955003519_i128) | (-114927400429533593723180159279766667340_i128);
_20 = _13;
_16.1 = (*_15) as f32;
_16.1 = RET as f32;
_9 = _12 as usize;
_10 = 44_u8;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
Goto(bb2)
}
bb2 = {
_22.1.0.fld5 = _8;
_3 = 227337937602883239993398939222339787818_u128 & 223946138030930942918725259728389756166_u128;
_11 = 60749_u16 * 61153_u16;
_19 = RET;
_6 = (-436992209_i32) | 697310765_i32;
_3 = 200104507778654341022299624209003709203_u128 + 180289380221643531442040397387169481742_u128;
_9 = 6_usize ^ 7875900891906518840_usize;
_22.1.0.fld5 = _8;
_22.0 = &mut _1;
_24 = 56_isize as f64;
_22.1.0.fld2 = _10;
_16.1 = _11 as f32;
_14 = [_20];
_14 = [_2];
_16.0 = [_11,_11,_11,_11];
_22.1.0.fld0 = [_22.1.0.fld5,_8];
_16.0 = [_11,_11,_11,_11];
match _22.1.0.fld2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
44 => bb7,
_ => bb6
}
}
bb3 = {
_2 = _13;
_16.0 = [_11,_11,_11,_11];
_4 = 56_i8 * (-50_i8);
_15 = &_6;
_16.0 = [_11,_11,_11,_11];
_9 = !4_usize;
_3 = 112999765583324069080529990255412187422_u128 & 289094199052503208882427765885163674060_u128;
_13 = _2;
_5 = (*_15) as i16;
_4 = (-77_i8) << (*_15);
_8 = (-149577695034732906529288648868955003519_i128) | (-114927400429533593723180159279766667340_i128);
_20 = _13;
_16.1 = (*_15) as f32;
_16.1 = RET as f32;
_9 = _12 as usize;
_10 = 44_u8;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
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
_22.1.0.fld3 = _6 as u16;
_22.1.0.fld3 = _11;
RET = !_5;
_20 = _13;
_22.1.0.fld1 = !_3;
_22.1.0.fld4 = _22.1.0.fld2 as i16;
_22.1.0.fld1 = _3 + _3;
_16.1 = _7 as f32;
_2 = _20;
_13 = _2;
_24 = _4 as f64;
_4 = !(-66_i8);
_10 = _22.1.0.fld2 + _22.1.0.fld2;
_23 = &_14;
_13 = _2;
_3 = !_22.1.0.fld1;
_25 = _10;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
_22.1.0.fld4 = !_19;
_9 = 2215865130181291702_usize;
_27.fld1 = Adt20::Variant2 { fld0: _6 };
place!(Field::<i32>(Variant(_27.fld1, 2), 0)) = _6 & _6;
_27.fld1 = Adt20::Variant2 { fld0: _6 };
_27.fld3 = _7 + _7;
_22.1.0.fld2 = _25;
_18 = &_16.0;
RET = _19 - _22.1.0.fld4;
_27.fld2 = !_22.1.0.fld2;
match _9 {
0 => bb1,
1 => bb6,
2 => bb8,
3 => bb9,
2215865130181291702 => bb11,
_ => bb10
}
}
bb8 = {
_2 = _13;
_16.0 = [_11,_11,_11,_11];
_4 = 56_i8 * (-50_i8);
_15 = &_6;
_16.0 = [_11,_11,_11,_11];
_9 = !4_usize;
_3 = 112999765583324069080529990255412187422_u128 & 289094199052503208882427765885163674060_u128;
_13 = _2;
_5 = (*_15) as i16;
_4 = (-77_i8) << (*_15);
_8 = (-149577695034732906529288648868955003519_i128) | (-114927400429533593723180159279766667340_i128);
_20 = _13;
_16.1 = (*_15) as f32;
_16.1 = RET as f32;
_9 = _12 as usize;
_10 = 44_u8;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
Goto(bb2)
}
bb9 = {
_22.1.0.fld5 = _8;
_3 = 227337937602883239993398939222339787818_u128 & 223946138030930942918725259728389756166_u128;
_11 = 60749_u16 * 61153_u16;
_19 = RET;
_6 = (-436992209_i32) | 697310765_i32;
_3 = 200104507778654341022299624209003709203_u128 + 180289380221643531442040397387169481742_u128;
_9 = 6_usize ^ 7875900891906518840_usize;
_22.1.0.fld5 = _8;
_22.0 = &mut _1;
_24 = 56_isize as f64;
_22.1.0.fld2 = _10;
_16.1 = _11 as f32;
_14 = [_20];
_14 = [_2];
_16.0 = [_11,_11,_11,_11];
_22.1.0.fld0 = [_22.1.0.fld5,_8];
_16.0 = [_11,_11,_11,_11];
match _22.1.0.fld2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
44 => bb7,
_ => bb6
}
}
bb10 = {
Return()
}
bb11 = {
_20 = _13;
Goto(bb12)
}
bb12 = {
_4 = -(-64_i8);
_25 = _10;
_27.fld3 = !_7;
_22.1.0.fld3 = _11 >> _5;
_22.1.0.fld2 = !_10;
_27.fld0 = _24;
_7 = _27.fld3;
_20 = _2;
_13 = _20;
_19 = _22.1.0.fld5 as i16;
_26 = Move(_22.0);
_9 = 8223746925474658261_usize;
_27.fld4 = RET | _22.1.0.fld4;
_10 = _27.fld2 & _22.1.0.fld2;
_22.1.0.fld3 = !_11;
_3 = _22.1.0.fld1 | _22.1.0.fld1;
_15 = &place!(Field::<i32>(Variant(_27.fld1, 2), 0));
Goto(bb13)
}
bb13 = {
_7 = !_27.fld3;
_24 = (*_15) as f64;
_10 = _27.fld2 & _25;
_3 = !_22.1.0.fld1;
place!(Field::<i32>(Variant(_27.fld1, 2), 0)) = _6 * _6;
_30 = _13;
_7 = _16.1 as u64;
_15 = &_6;
_20 = _2;
match _9 {
0 => bb9,
1 => bb12,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb6,
6 => bb14,
8223746925474658261 => bb16,
_ => bb15
}
}
bb14 = {
_2 = _13;
_16.0 = [_11,_11,_11,_11];
_4 = 56_i8 * (-50_i8);
_15 = &_6;
_16.0 = [_11,_11,_11,_11];
_9 = !4_usize;
_3 = 112999765583324069080529990255412187422_u128 & 289094199052503208882427765885163674060_u128;
_13 = _2;
_5 = (*_15) as i16;
_4 = (-77_i8) << (*_15);
_8 = (-149577695034732906529288648868955003519_i128) | (-114927400429533593723180159279766667340_i128);
_20 = _13;
_16.1 = (*_15) as f32;
_16.1 = RET as f32;
_9 = _12 as usize;
_10 = 44_u8;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
Goto(bb2)
}
bb15 = {
_20 = _13;
Goto(bb12)
}
bb16 = {
_27.fld3 = (*_15) as u64;
_9 = 57_isize as usize;
_10 = _22.1.0.fld2;
_33 = !_11;
_22.1.0.fld1 = !_3;
_22.1.0.fld3 = _33;
_13 = _2;
RET = -_5;
_27.fld4 = true as i16;
_11 = _22.1.0.fld3 - _33;
RET = _22.1.0.fld4;
_27.fld5 = _16.1 * _16.1;
_15 = &place!(Field::<i32>(Variant(_27.fld1, 2), 0));
_27.fld4 = -_19;
_27.fld2 = !_25;
Goto(bb17)
}
bb17 = {
_12 = _22.1.0.fld1 as u32;
_25 = _22.1.0.fld2;
_21 = core::ptr::addr_of!(_27);
(*_21).fld2 = _9 as u8;
(*_21).fld5 = _16.1 - _16.1;
_36 = _30;
(*_21).fld0 = _24 - _24;
(*_21).fld2 = !_22.1.0.fld2;
Goto(bb18)
}
bb18 = {
_27.fld5 = _22.1.0.fld3 as f32;
_27.fld1 = Adt20::Variant0 { fld0: _8,fld1: _30,fld2: (*_21).fld2,fld3: (*_21).fld3,fld4: _6 };
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (-2334216234054330223_i64) as u64;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2 + (*_21).fld2;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _13;
(*_21).fld4 = !RET;
_24 = (*_21).fld4 as f64;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2 | (*_21).fld2;
Call(_10 = core::intrinsics::bswap((*_21).fld2), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = Field::<i128>(Variant((*_21).fld1, 0), 0) as i32;
(*_21).fld4 = _22.1.0.fld4 * RET;
_28 = &_27;
_30 = Field::<char>(Variant((*_21).fld1, 0), 1);
(*_21).fld0 = Field::<i32>(Variant((*_21).fld1, 0), 4) as f64;
_31 = (*_21).fld0 as u32;
_15 = &place!(Field::<i32>(Variant((*_21).fld1, 0), 4));
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld3 as u8;
(*_21).fld5 = _16.1 + _16.1;
(*_21).fld0 = _24 - _24;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_28).fld2 >> Field::<i128>(Variant((*_28).fld1, 0), 0);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = !_6;
_16.0 = [_11,_11,_11,_11];
place!(Field::<char>(Variant(_27.fld1, 0), 1)) = _30;
(*_21).fld2 = !_10;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _30;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
Goto(bb20)
}
bb20 = {
(*_21).fld5 = -_16.1;
(*_21).fld4 = _22.1.0.fld4;
(*_21).fld0 = _24 - _24;
(*_21).fld4 = _5 << (*_28).fld3;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _22.1.0.fld5 * _8;
_4 = 7_i8 * (-110_i8);
(*_21).fld0 = _24 * _24;
_13 = Field::<char>(Variant((*_21).fld1, 0), 1);
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld2 = !Field::<u8>(Variant(_27.fld1, 0), 2);
_42 = Field::<i128>(Variant((*_21).fld1, 0), 0) << (*_21).fld2;
Goto(bb21)
}
bb21 = {
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 - (*_21).fld3;
Goto(bb22)
}
bb22 = {
_6 = !Field::<i32>(Variant((*_21).fld1, 0), 4);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
_9 = !7_usize;
_40.3 = core::ptr::addr_of!(_22.1.0);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
_46 = 9223372036854775807_isize | 9223372036854775807_isize;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6 | _6;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) - Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld5 = (*_21).fld0 as f32;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 | (*_21).fld3;
(*_21).fld4 = _22.1.0.fld4 ^ RET;
place!(Field::<i32>(Variant(_27.fld1, 0), 4)) = !_6;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2 - (*_21).fld2;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6 * _6;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2);
_44 = &_31;
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2) * Field::<u8>(Variant((*_21).fld1, 0), 2);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _30;
Call((*_21).fld5 = core::intrinsics::transmute(Field::<char>(Variant((*_21).fld1, 0), 1)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
place!(Field::<u64>(Variant(_27.fld1, 0), 3)) = (*_21).fld3 | (*_21).fld3;
(*_21).fld0 = _24 * _24;
_44 = &_12;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2;
(*_21).fld2 = !Field::<u8>(Variant((*_21).fld1, 0), 2);
_16.1 = (*_21).fld5 * (*_21).fld5;
(*_21).fld3 = _7;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = !_6;
(*_21).fld5 = _16.1 + _16.1;
_16.0 = [_33,_22.1.0.fld3,_11,_33];
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_21).fld5 as i128;
(*_21).fld5 = Field::<u8>(Variant((*_21).fld1, 0), 2) as f32;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = Field::<char>(Variant((*_21).fld1, 0), 1) as u8;
Goto(bb24)
}
bb24 = {
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2;
(*_21).fld5 = -_16.1;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = (*_21).fld2 | (*_21).fld2;
place!(Field::<u8>(Variant((*_21).fld1, 0), 2)) = _27.fld2 - (*_21).fld2;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 << _27.fld4;
(*_21).fld4 = _5 << (*_44);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _9 as i128;
(*_21).fld0 = _24;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) + Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2) >> (*_21).fld3;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _13;
place!(Field::<i128>(Variant(_27.fld1, 0), 0)) = _42;
(*_21).fld4 = RET >> (*_44);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
(*_21).fld0 = _9 as f64;
(*_21).fld3 = _46 as u64;
_45 = &place!(Field::<i32>(Variant((*_21).fld1, 0), 4));
(*_21).fld5 = _16.1 + _16.1;
Goto(bb25)
}
bb25 = {
(*_21).fld4 = (*_21).fld0 as i16;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6;
_22.1.0.fld2 = Field::<i128>(Variant((*_21).fld1, 0), 0) as u8;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = -_6;
Goto(bb26)
}
bb26 = {
_43 = _46 + _46;
place!(Field::<i32>(Variant(_27.fld1, 0), 4)) = !_6;
(*_21).fld4 = _5 >> Field::<u8>(Variant((*_21).fld1, 0), 2);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6 << Field::<u8>(Variant((*_21).fld1, 0), 2);
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2) + Field::<u8>(Variant((*_21).fld1, 0), 2);
(*_21).fld2 = !Field::<u8>(Variant((*_21).fld1, 0), 2);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _30;
_27.fld5 = _16.1;
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2) & Field::<u8>(Variant((*_21).fld1, 0), 2);
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2) | Field::<u8>(Variant((*_21).fld1, 0), 2);
(*_21).fld2 = !Field::<u8>(Variant((*_21).fld1, 0), 2);
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) | Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld4 = _5;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
Call((*_21).fld2 = core::intrinsics::transmute(Field::<u8>(Variant((*_21).fld1, 0), 2)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_35 = [_11,_11,_11,_22.1.0.fld3];
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 << Field::<u8>(Variant(_27.fld1, 0), 2);
(*_21).fld4 = Field::<i128>(Variant(_27.fld1, 0), 0) as i16;
_55 = _9 as f64;
_22.1.0.fld0 = [Field::<i128>(Variant((*_21).fld1, 0), 0),Field::<i128>(Variant((*_21).fld1, 0), 0)];
place!(Field::<u8>(Variant(_27.fld1, 0), 2)) = (*_21).fld2 | (*_21).fld2;
(*_21).fld2 = Field::<u8>(Variant((*_21).fld1, 0), 2);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
_38 = &mut place!(Field::<u8>(Variant((*_21).fld1, 0), 2));
(*_38) = !(*_21).fld2;
(*_21).fld2 = _4 as u8;
_39 = [_4,_4,_4,_4,_4,_4,_4,_4];
(*_21).fld4 = RET ^ _22.1.0.fld4;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _42 << (*_38);
(*_21).fld0 = _24 * _55;
_36 = Field::<char>(Variant((*_21).fld1, 0), 1);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = !(*_21).fld3;
(*_21).fld4 = _9 as i16;
(*_21).fld3 = (*_21).fld5 as u64;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) - _7;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) | Field::<u64>(Variant((*_21).fld1, 0), 3);
_16 = (_35, (*_21).fld5);
_60.0 = Adt19 { fld0: _22.1.0.fld0,fld1: _3,fld2: (*_38),fld3: _11,fld4: RET,fld5: Field::<i128>(Variant((*_21).fld1, 0), 0) };
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _60.0.fld5 >> (*_38);
_57 = _46;
_6 = Field::<i32>(Variant((*_21).fld1, 0), 4) + Field::<i32>(Variant((*_21).fld1, 0), 4);
_58 = [_43];
Goto(bb28)
}
bb28 = {
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
(*_38) = _4 as u8;
_4 = (-119_i8);
(*_21).fld2 = _60.0.fld2 & _22.1.0.fld2;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6 * _6;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_21).fld0 as i128;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = true as u64;
(*_21).fld0 = _55 - _24;
Goto(bb29)
}
bb29 = {
(*_21).fld2 = _10 | (*_38);
(*_21).fld0 = _60.0.fld1 as f64;
_53 = (*_21).fld0 as f32;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3;
(*_38) = !_60.0.fld2;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 - (*_21).fld3;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = _43 as u64;
_39 = [_4,_4,_4,_4,_4,_4,_4,_4];
_40.2 = -(*_21).fld0;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
Call(place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = core::intrinsics::bswap(_60.0.fld5), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = !(*_21).fld3;
_11 = !_22.1.0.fld3;
_48 = Field::<i128>(Variant((*_21).fld1, 0), 0);
_16.0 = [_60.0.fld3,_60.0.fld3,_22.1.0.fld3,_33];
_56 = (*_21).fld5 + (*_21).fld5;
(*_38) = !(*_21).fld2;
_14 = [Field::<char>(Variant((*_21).fld1, 0), 1)];
(*_21).fld5 = (-4567033332593362647_i64) as f32;
(*_21).fld4 = _22.1.0.fld4 ^ _22.1.0.fld4;
_3 = _60.0.fld1;
(*_21).fld0 = _40.2 * _24;
(*_21).fld0 = (*_44) as f64;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = _6 - _6;
_22.1.0.fld3 = _11 >> _42;
_39 = [_4,_4,_4,_4,_4,_4,_4,_4];
_60.0.fld5 = Field::<i128>(Variant((*_21).fld1, 0), 0) | Field::<i128>(Variant((*_21).fld1, 0), 0);
match _4 {
0 => bb16,
340282366920938463463374607431768211337 => bb32,
_ => bb31
}
}
bb31 = {
_2 = _13;
_16.0 = [_11,_11,_11,_11];
_4 = 56_i8 * (-50_i8);
_15 = &_6;
_16.0 = [_11,_11,_11,_11];
_9 = !4_usize;
_3 = 112999765583324069080529990255412187422_u128 & 289094199052503208882427765885163674060_u128;
_13 = _2;
_5 = (*_15) as i16;
_4 = (-77_i8) << (*_15);
_8 = (-149577695034732906529288648868955003519_i128) | (-114927400429533593723180159279766667340_i128);
_20 = _13;
_16.1 = (*_15) as f32;
_16.1 = RET as f32;
_9 = _12 as usize;
_10 = 44_u8;
_22.2 = core::ptr::addr_of_mut!(_22.1.0.fld1);
Goto(bb2)
}
bb32 = {
(*_21).fld5 = _53 + _53;
(*_38) = !_60.0.fld2;
_4 = (-33_i8) * 115_i8;
_9 = 4_usize;
(*_21).fld4 = (*_44) as i16;
(*_38) = !_60.0.fld2;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = !_42;
(*_21).fld5 = -_53;
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _42 | _60.0.fld5;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld5 as u64;
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) + Field::<u64>(Variant((*_21).fld1, 0), 3);
_65 = &place!(Field::<i32>(Variant((*_21).fld1, 0), 4));
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
_40.3 = core::ptr::addr_of!(_60.0);
_50 = core::ptr::addr_of_mut!(_57);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = -_42;
_30 = _20;
match _39[_9] {
0 => bb18,
1 => bb31,
2 => bb24,
3 => bb10,
4 => bb5,
5 => bb14,
340282366920938463463374607431768211337 => bb33,
_ => bb9
}
}
bb33 = {
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 + (*_21).fld3;
(*_21).fld2 = 9072103839908842770_i64 as u8;
_48 = Field::<i128>(Variant((*_21).fld1, 0), 0) + Field::<i128>(Variant((*_21).fld1, 0), 0);
_3 = (-6933287158113194894_i64) as u128;
(*_21).fld4 = _5;
_39[_9] = -_4;
(*_21).fld4 = _5;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = !_60.0.fld5;
(*_21).fld2 = (*_38) * (*_38);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _2;
_22.2 = core::ptr::addr_of_mut!(_60.0.fld1);
_29 = (*_21).fld2;
(*_21).fld2 = (*_38);
(*_21).fld4 = !_5;
(*_38) = (*_21).fld2 + (*_21).fld2;
_65 = &_6;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) + Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_38) = (*_21).fld0 as u8;
(*_50) = _46 + _43;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 << (*_21).fld2;
(*_21).fld3 = Field::<char>(Variant((*_21).fld1, 0), 1) as u64;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = !(*_21).fld3;
_34 = _60.0.fld3 + _22.1.0.fld3;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_21).fld5 as i128;
(*_38) = (*_21).fld2 | (*_21).fld2;
Goto(bb34)
}
bb34 = {
(*_50) = _43 * _43;
_57 = Field::<i32>(Variant((*_21).fld1, 0), 4) as isize;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 + (*_21).fld3;
match _9 {
4 => bb35,
_ => bb28
}
}
bb35 = {
(*_38) = (*_21).fld2 >> (*_50);
(*_21).fld3 = (*_21).fld4 as u64;
_66 = Field::<i128>(Variant((*_21).fld1, 0), 0) & Field::<i128>(Variant((*_21).fld1, 0), 0);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = (*_65) ^ (*_65);
_40.2 = -(*_21).fld0;
(*_21).fld4 = _5 ^ _22.1.0.fld4;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) ^ Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = _9 as u64;
_74 = (*_50);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = _9 as u64;
(*_21).fld5 = _16.1 + _53;
(*_21).fld5 = -_53;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _48 + _60.0.fld5;
_33 = _60.0.fld3 | _22.1.0.fld3;
Call((*_21).fld0 = core::intrinsics::fmaf64(_40.2, _24, _24), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
(*_21).fld4 = _22.1.0.fld4;
(*_21).fld5 = _53 + _56;
(*_21).fld4 = _60.0.fld4;
(*_50) = _74 + _74;
(*_50) = !_74;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
_22.1.0.fld4 = (*_21).fld4;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 - (*_21).fld3;
(*_38) = !(*_21).fld2;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 + (*_21).fld3;
_29 = (*_38);
(*_21).fld4 = RET;
(*_21).fld0 = -_24;
(*_50) = (*_21).fld5 as isize;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_44) as i128;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 - (*_21).fld3;
_5 = (*_21).fld2 as i16;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) * Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = (*_65);
(*_21).fld2 = !(*_38);
Goto(bb37)
}
bb37 = {
(*_38) = !(*_21).fld2;
(*_21).fld0 = Field::<i32>(Variant((*_21).fld1, 0), 4) as f64;
_24 = Field::<i32>(Variant((*_21).fld1, 0), 4) as f64;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _42 & _48;
(*_21).fld2 = _60.0.fld2 >> Field::<i32>(Variant((*_21).fld1, 0), 4);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3;
_10 = Field::<i128>(Variant((*_21).fld1, 0), 0) as u8;
(*_21).fld5 = _53 * _53;
(*_21).fld4 = _5;
(*_50) = !_74;
_20 = Field::<char>(Variant((*_21).fld1, 0), 1);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (-8475864377301628069_i64) as i128;
(*_50) = !_74;
_9 = !2_usize;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = (*_65);
(*_21).fld5 = (*_65) as f32;
(*_50) = (*_21).fld4 as isize;
(*_21).fld5 = -_16.1;
(*_21).fld2 = _5 as u8;
(*_21).fld0 = _24 + _24;
(*_21).fld2 = !(*_38);
(*_21).fld2 = (*_38);
_46 = (*_50) - (*_50);
_79 = (*_21).fld4 * (*_21).fld4;
Call((*_21).fld0 = core::intrinsics::fmaf64(_24, _24, _24), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
_54 = !true;
(*_50) = _43;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _30;
(*_38) = (*_21).fld2 & (*_21).fld2;
Goto(bb39)
}
bb39 = {
_43 = _22.1.0.fld1 as isize;
(*_50) = _74 - _74;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (-8168897861211500441_i64) as i128;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 | (*_21).fld3;
_22.2 = core::ptr::addr_of_mut!(_60.0.fld1);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = Field::<u64>(Variant((*_21).fld1, 0), 3) as i32;
(*_21).fld2 = (*_38) | (*_38);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _48 >> (*_38);
_16.0 = _35;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = (*_21).fld2 as i32;
_79 = (*_21).fld4 << (*_38);
_20 = Field::<char>(Variant((*_21).fld1, 0), 1);
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
_60.0 = Adt19 { fld0: _22.1.0.fld0,fld1: _22.1.0.fld1,fld2: (*_38),fld3: _33,fld4: (*_21).fld4,fld5: Field::<i128>(Variant((*_21).fld1, 0), 0) };
_22.1.0.fld2 = (*_21).fld2 >> (*_38);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3 << Field::<i128>(Variant((*_21).fld1, 0), 0);
_22.1.0.fld5 = _4 as i128;
(*_38) = (*_21).fld2 * _22.1.0.fld2;
(*_50) = _74 ^ _46;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) >> (*_21).fld4;
_60.0.fld1 = !_3;
_39 = [_4,_4,_4,_4,_4,_4,_4,_4];
(*_21).fld5 = _53;
_75 = (*_50);
_88 = [_3,_3,_3,_22.1.0.fld1,_60.0.fld1,_22.1.0.fld1];
Goto(bb40)
}
bb40 = {
_22.1.0.fld4 = !(*_21).fld4;
_78 = (*_21).fld0 * (*_21).fld0;
_74 = (*_50) ^ (*_50);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = !(*_21).fld3;
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
_60.0.fld0 = [_60.0.fld5,Field::<i128>(Variant((*_21).fld1, 0), 0)];
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_21).fld5 as i128;
_67 = (*_50) & (*_50);
_22.1.0.fld4 = (*_21).fld4;
_90 = -(*_65);
_96 = !_9;
(*_50) = -_46;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _36;
_80 = Move(_22.2);
Goto(bb41)
}
bb41 = {
(*_21).fld3 = _34 as u64;
(*_21).fld0 = _24;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _60.0.fld5 & _60.0.fld5;
Goto(bb42)
}
bb42 = {
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = !(*_65);
(*_21).fld2 = (*_38) ^ (*_38);
(*_21).fld0 = _3 as f64;
_63 = _22.1.0.fld1 as u32;
_76 = (*_65);
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = !_48;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _13;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _60.0.fld5;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = _4 as u64;
(*_21).fld2 = (*_38) + (*_38);
(*_21).fld0 = _22.1.0.fld1 as f64;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
_22.1.0.fld4 = _79;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = !(*_21).fld3;
_67 = (*_21).fld3 as isize;
_78 = _24;
_64 = (*_44);
_59 = core::ptr::addr_of!(_89);
_94 = &mut _38;
(*_50) = _74 >> Field::<i128>(Variant((*_21).fld1, 0), 0);
_77 = !Field::<i128>(Variant((*_21).fld1, 0), 0);
Goto(bb43)
}
bb43 = {
_22.1.0.fld2 = !(*_21).fld2;
_68 = core::ptr::addr_of_mut!(_88);
(*_94) = &mut (*_21).fld2;
_49 = Field::<i128>(Variant((*_21).fld1, 0), 0);
_22.0 = &mut _54;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
(*_21).fld4 = _79 ^ _79;
_88 = [_22.1.0.fld1,_3,_22.1.0.fld1,_60.0.fld1,_22.1.0.fld1,_3];
(*_68) = [_60.0.fld1,_3,_22.1.0.fld1,_3,_22.1.0.fld1,_60.0.fld1];
Goto(bb44)
}
bb44 = {
(*_50) = _75 ^ _75;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) - Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_21).fld0 = _78;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = (*_65);
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = true as u64;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3) | Field::<u64>(Variant((*_21).fld1, 0), 3);
(*_50) = _74;
(*_94) = &mut _25;
(*_21).fld4 = _60.0.fld4;
_10 = (*_44) as u8;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _30;
place!(Field::<u64>(Variant((*_21).fld1, 0), 3)) = (*_21).fld3;
_60.0.fld3 = _22.1.0.fld3;
(*_68) = [_22.1.0.fld1,_22.1.0.fld1,_22.1.0.fld1,_60.0.fld1,_3,_60.0.fld1];
_63 = (*_21).fld4 as u32;
_99 = (*_21).fld0 - (*_21).fld0;
_44 = &_64;
_71 = (*_50);
_82 = (*_21).fld0 + (*_21).fld0;
_65 = &place!(Field::<i32>(Variant((*_21).fld1, 0), 4));
(*_50) = _74;
(*_21).fld0 = _24 - _24;
_32 = Move(_22.0);
_46 = (*_50) * (*_50);
_30 = _2;
_19 = !(*_21).fld4;
Goto(bb45)
}
bb45 = {
_79 = Field::<u64>(Variant((*_21).fld1, 0), 3) as i16;
(*_21).fld3 = !Field::<u64>(Variant((*_21).fld1, 0), 3);
_7 = !(*_21).fld3;
(*_94) = &mut _60.0.fld2;
(*_21).fld4 = _19 * _5;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = !_48;
_16.1 = -(*_21).fld5;
_104 = core::ptr::addr_of_mut!(_89);
_35 = _16.0;
(*_21).fld0 = (*_50) as f64;
(*_50) = !_74;
Goto(bb46)
}
bb46 = {
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = _77 >> (*_21).fld4;
(*_21).fld4 = _22.1.0.fld2 as i16;
_22.1.0.fld3 = !_33;
(*_68) = [_3,_22.1.0.fld1,_22.1.0.fld1,_22.1.0.fld1,_22.1.0.fld1,_22.1.0.fld1];
_93 = Field::<i32>(Variant((*_21).fld1, 0), 4) as f32;
Goto(bb47)
}
bb47 = {
_58 = [(*_50)];
_47 = _96 - _9;
place!(Field::<i128>(Variant((*_21).fld1, 0), 0)) = (*_50) as i128;
(*_94) = &mut _29;
(*_21).fld0 = _4 as f64;
(*_50) = _47 as isize;
(*_21).fld0 = _55 - _82;
(*_21).fld0 = -_24;
(*_21).fld0 = _24 - _40.2;
(*_21).fld5 = -_93;
(*_21).fld3 = Field::<u64>(Variant((*_21).fld1, 0), 3);
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = !_6;
place!(Field::<char>(Variant((*_21).fld1, 0), 1)) = _20;
Call(place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = core::intrinsics::bswap(_90), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
_115 = _20;
(*_21).fld5 = -_16.1;
_85 = core::ptr::addr_of_mut!((*_21).fld5);
(*_21).fld5 = (-5925965864908281073_i64) as f32;
(*_50) = !_75;
place!(Field::<i32>(Variant((*_21).fld1, 0), 4)) = !_76;
_45 = &_90;
(*_21).fld0 = _99 + _82;
_118.1 = _93;
Goto(bb49)
}
bb49 = {
_101 = (-6611632607272937389_i64) as f64;
_86 = (*_21).fld5 as i128;
Goto(bb50)
}
bb50 = {
Call(_120 = dump_var(Move(_74), Move(_75), Move(_79), Move(_35)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_120 = dump_var(Move(_63), Move(_66), Move(_11), Move(_96)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_120 = dump_var(Move(_115), Move(_57), Move(_4), Move(_42)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_120 = dump_var(Move(_20), Move(_12), Move(_71), Move(_77)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_120 = dump_var(Move(_49), Move(_9), Move(_76), Move(_54)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_120 = dump_var(Move(_86), Move(_39), Move(_10), Move(_29)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i128,mut _2: char,mut _3: u32,mut _4: u64,mut _5: u32,mut _6: i16,mut _7: char,mut _8: char) -> [char; 1] {
mir! {
type RET = [char; 1];
let _9: isize;
let _10: [usize; 6];
let _11: f32;
let _12: *const Adt27;
let _13: [usize; 6];
let _14: *mut isize;
let _15: u64;
let _16: isize;
let _17: [u32; 8];
let _18: [i8; 4];
let _19: u16;
let _20: &'static mut u8;
let _21: bool;
let _22: *const Adt27;
let _23: &'static mut &'static mut u8;
let _24: &'static mut bool;
let _25: &'static i32;
let _26: *mut Adt38;
let _27: [i8; 3];
let _28: char;
let _29: char;
let _30: i128;
let _31: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _32: i16;
let _33: f32;
let _34: &'static mut Adt20;
let _35: f32;
let _36: isize;
let _37: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _38: Adt20;
let _39: f32;
let _40: Adt27;
let _41: isize;
let _42: [u8; 7];
let _43: *const isize;
let _44: usize;
let _45: i16;
let _46: u8;
let _47: bool;
let _48: *const Adt38;
let _49: u32;
let _50: u128;
let _51: *mut Adt38;
let _52: isize;
let _53: char;
let _54: u64;
let _55: [i8; 4];
let _56: &'static u32;
let _57: [usize; 6];
let _58: &'static mut [i128; 2];
let _59: &'static mut Adt20;
let _60: [i8; 3];
let _61: [usize; 4];
let _62: char;
let _63: usize;
let _64: f32;
let _65: isize;
let _66: &'static [i8; 3];
let _67: *mut u128;
let _68: isize;
let _69: &'static &'static mut &'static u32;
let _70: &'static [u16; 4];
let _71: &'static mut (&'static i32, *mut u128, i8, Adt20);
let _72: isize;
let _73: isize;
let _74: ([u16; 4], f32);
let _75: [bool; 7];
let _76: *mut isize;
let _77: char;
let _78: *mut Adt38;
let _79: &'static i64;
let _80: [i32; 3];
let _81: i64;
let _82: i128;
let _83: f32;
let _84: isize;
let _85: &'static mut [i128; 2];
let _86: *const u32;
let _87: Adt31;
let _88: *mut u128;
let _89: i128;
let _90: ();
let _91: ();
{
RET = [_7];
_7 = _8;
_9 = _5 as isize;
_4 = 9829852758525347843_u64 >> _6;
_4 = 3085046748078970501_u64 * 5585423533774645269_u64;
_6 = (-9017_i16) & 9960_i16;
_8 = _2;
_1 = (-4983209983256542106810710554943924395_i128) ^ 14869943438204401200622924921308874631_i128;
_6 = _3 as i16;
_4 = 10770137919070535151_u64 << _1;
_8 = _2;
Goto(bb1)
}
bb1 = {
_11 = _4 as f32;
_3 = _5 ^ _5;
_5 = _3;
_10 = [13378267753434425372_usize,2_usize,6569209418661572405_usize,7_usize,3723250223892773070_usize,4_usize];
_4 = 6993832938444239758_u64;
_6 = 2360_i16 + (-830_i16);
_6 = !23261_i16;
_8 = _7;
_2 = _7;
_7 = _8;
_2 = _7;
_11 = 46017_u16 as f32;
_3 = !_5;
_1 = -(-14956167896067825598334963332765073124_i128);
_11 = 224_u8 as f32;
_8 = _2;
_10 = [14572818896478392122_usize,9697813282063629580_usize,4_usize,4_usize,10118419479538180808_usize,11274127403141525392_usize];
_1 = (-39276798103313727610094407538968982757_i128);
RET = [_2];
_7 = _8;
_10 = [7_usize,6828317630341108134_usize,7913289401590057249_usize,6_usize,1_usize,0_usize];
_4 = 3873552190146728772_u64 + 8451243369556598954_u64;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_14) = _7 as isize;
(*_14) = 9223372036854775807_isize ^ (-65_isize);
Goto(bb2)
}
bb2 = {
(*_14) = 9223372036854775807_isize;
Call((*_14) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_14) = 127_i8 as isize;
_13 = [5_usize,0_usize,3904927811172845639_usize,1829838232838299317_usize,4_usize,4_usize];
(*_14) = _2 as isize;
(*_14) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_11 = (*_14) as f32;
_17 = [_3,_5,_5,_5,_3,_3,_3,_5];
_19 = !5283_u16;
_11 = (-811329553_i32) as f32;
(*_14) = !123_isize;
(*_14) = _6 as isize;
_8 = _7;
(*_14) = !9223372036854775807_isize;
(*_14) = _1 as isize;
_16 = (*_14) | (*_14);
_21 = true ^ false;
(*_14) = _16 ^ _16;
(*_14) = _16 | _16;
(*_14) = _11 as isize;
(*_14) = _16 ^ _16;
Call(_4 = fn2((*_14), _2, Move(_14), _7, (*_14)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _3;
_11 = 303191232656065125043509099627579297484_u128 as f32;
_13 = [17101046174725184833_usize,0_usize,2273338055795160968_usize,15536614177590484673_usize,6970877005017746566_usize,8842035561725506280_usize];
RET = [_7];
_8 = _2;
RET = [_2];
_16 = _9 * _9;
_28 = _2;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = _16 & _16;
_6 = (-24748_i16) & (-11589_i16);
_3 = 1_usize as u32;
_19 = !30597_u16;
_18 = [56_i8,23_i8,43_i8,(-63_i8)];
(*_14) = _16;
(*_14) = _5 as isize;
_13 = _10;
_5 = _3 & _3;
Goto(bb5)
}
bb5 = {
(*_14) = -_16;
(*_14) = -_16;
_9 = _16 ^ _16;
(*_14) = _16;
_24 = &mut _21;
_14 = core::ptr::addr_of_mut!((*_14));
_32 = _6 - _6;
_29 = _7;
(*_14) = !_16;
_8 = _2;
_1 = 44534170646434227121393927517369567659_i128 + (-136238874622172260424771203862069900215_i128);
(*_24) = !true;
_15 = _4 ^ _4;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
5477765985521074611 => bb11,
_ => bb10
}
}
bb6 = {
_5 = _3;
_11 = 303191232656065125043509099627579297484_u128 as f32;
_13 = [17101046174725184833_usize,0_usize,2273338055795160968_usize,15536614177590484673_usize,6970877005017746566_usize,8842035561725506280_usize];
RET = [_7];
_8 = _2;
RET = [_2];
_16 = _9 * _9;
_28 = _2;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = _16 & _16;
_6 = (-24748_i16) & (-11589_i16);
_3 = 1_usize as u32;
_19 = !30597_u16;
_18 = [56_i8,23_i8,43_i8,(-63_i8)];
(*_14) = _16;
(*_14) = _5 as isize;
_13 = _10;
_5 = _3 & _3;
Goto(bb5)
}
bb7 = {
(*_14) = 127_i8 as isize;
_13 = [5_usize,0_usize,3904927811172845639_usize,1829838232838299317_usize,4_usize,4_usize];
(*_14) = _2 as isize;
(*_14) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_11 = (*_14) as f32;
_17 = [_3,_5,_5,_5,_3,_3,_3,_5];
_19 = !5283_u16;
_11 = (-811329553_i32) as f32;
(*_14) = !123_isize;
(*_14) = _6 as isize;
_8 = _7;
(*_14) = !9223372036854775807_isize;
(*_14) = _1 as isize;
_16 = (*_14) | (*_14);
_21 = true ^ false;
(*_14) = _16 ^ _16;
(*_14) = _16 | _16;
(*_14) = _11 as isize;
(*_14) = _16 ^ _16;
Call(_4 = fn2((*_14), _2, Move(_14), _7, (*_14)), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
(*_14) = 9223372036854775807_isize;
Call((*_14) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_11 = _4 as f32;
_3 = _5 ^ _5;
_5 = _3;
_10 = [13378267753434425372_usize,2_usize,6569209418661572405_usize,7_usize,3723250223892773070_usize,4_usize];
_4 = 6993832938444239758_u64;
_6 = 2360_i16 + (-830_i16);
_6 = !23261_i16;
_8 = _7;
_2 = _7;
_7 = _8;
_2 = _7;
_11 = 46017_u16 as f32;
_3 = !_5;
_1 = -(-14956167896067825598334963332765073124_i128);
_11 = 224_u8 as f32;
_8 = _2;
_10 = [14572818896478392122_usize,9697813282063629580_usize,4_usize,4_usize,10118419479538180808_usize,11274127403141525392_usize];
_1 = (-39276798103313727610094407538968982757_i128);
RET = [_2];
_7 = _8;
_10 = [7_usize,6828317630341108134_usize,7913289401590057249_usize,6_usize,1_usize,0_usize];
_4 = 3873552190146728772_u64 + 8451243369556598954_u64;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_14) = _7 as isize;
(*_14) = 9223372036854775807_isize ^ (-65_isize);
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
(*_14) = _16 ^ _16;
_19 = 31002_u16 ^ 47399_u16;
_19 = 60890_u16 + 42232_u16;
(*_24) = false | true;
(*_24) = (*_14) >= (*_14);
_3 = !_5;
(*_14) = 12991216499431938638_usize as isize;
(*_14) = _15 as isize;
_29 = _7;
(*_24) = !true;
(*_14) = _16 & _16;
(*_24) = false;
(*_24) = false | true;
_14 = core::ptr::addr_of_mut!((*_14));
(*_24) = true;
_13 = [6_usize,15252476677969026338_usize,5_usize,9095939657881628223_usize,5_usize,7_usize];
_13 = _10;
_30 = _1;
match _4 {
0 => bb1,
1 => bb10,
2 => bb9,
3 => bb4,
4 => bb8,
5 => bb6,
5477765985521074611 => bb12,
_ => bb7
}
}
bb12 = {
_37.2 = _1 as f64;
_38 = Adt20::Variant0 { fld0: _1,fld1: _2,fld2: 245_u8,fld3: _4,fld4: 926163145_i32 };
_35 = _11 - _11;
_3 = _5;
_27 = [(-33_i8),36_i8,37_i8];
_40.fld0 = _15 as f64;
_1 = 0_usize as i128;
(*_24) = (*_14) > (*_14);
(*_14) = _16 - _16;
_8 = _2;
_39 = -_35;
(*_14) = _16;
(*_24) = !true;
place!(Field::<char>(Variant(_38, 0), 1)) = _2;
(*_14) = _16;
_13 = [1_usize,5_usize,14008483042205153401_usize,5_usize,4597345792579480926_usize,15680763498969856239_usize];
(*_24) = !false;
(*_24) = !false;
(*_24) = true & true;
_35 = -_39;
(*_24) = false;
(*_24) = !true;
_2 = _28;
place!(Field::<u64>(Variant(_38, 0), 3)) = _4 / _4;
Goto(bb13)
}
bb13 = {
_40.fld5 = -_39;
_31.2 = _40.fld0;
_10 = [6690716346714480316_usize,3_usize,0_usize,1121994195108718032_usize,2_usize,0_usize];
_4 = Field::<u64>(Variant(_38, 0), 3) << (*_14);
_36 = _40.fld0 as isize;
_40.fld3 = _15;
(*_14) = _16;
(*_24) = true;
_40.fld2 = 38_u8 | 44_u8;
(*_24) = _7 < _7;
(*_14) = _16 - _16;
(*_14) = _16 * _16;
(*_14) = Field::<i128>(Variant(_38, 0), 0) as isize;
(*_24) = !true;
_10 = [15404726045316486535_usize,14773768256397167939_usize,6_usize,4_usize,17127124207883535811_usize,18246620790765150101_usize];
Goto(bb14)
}
bb14 = {
place!(Field::<u64>(Variant(_38, 0), 3)) = !_15;
_40.fld2 = 118_u8 & 169_u8;
(*_24) = false;
place!(Field::<char>(Variant(_38, 0), 1)) = _28;
_16 = (*_14) ^ (*_14);
RET = [_2];
_8 = _28;
(*_24) = false;
Call(_5 = core::intrinsics::transmute(_2), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_36 = (*_14);
place!(Field::<char>(Variant(_38, 0), 1)) = _28;
(*_24) = true & false;
_11 = -_40.fld5;
_40.fld3 = _4;
_13 = [3_usize,15553320000582565241_usize,6_usize,2_usize,4_usize,4_usize];
(*_24) = true;
_1 = Field::<i128>(Variant(_38, 0), 0);
_20 = &mut _40.fld2;
_43 = core::ptr::addr_of!((*_14));
place!(Field::<i32>(Variant(_38, 0), 4)) = _39 as i32;
_28 = _29;
(*_43) = _11 as isize;
_8 = _29;
_11 = _35 + _35;
_42 = [(*_20),(*_20),(*_20),(*_20),(*_20),(*_20),(*_20)];
_23 = &mut _20;
(*_43) = _36 * _16;
_10 = [4_usize,0_usize,6_usize,15917082307918542950_usize,2_usize,5_usize];
(*_43) = _16;
_5 = _3;
_42 = [111_u8,43_u8,141_u8,223_u8,155_u8,249_u8,172_u8];
(*_24) = _1 != Field::<i128>(Variant(_38, 0), 0);
place!(Field::<u8>(Variant(_38, 0), 2)) = (*_43) as u8;
Goto(bb16)
}
bb16 = {
(*_24) = (*_43) >= (*_14);
_10 = [2_usize,4_usize,5_usize,7_usize,15506665412529079092_usize,18038996071168010244_usize];
_39 = _4 as f32;
_49 = !_3;
_42 = [Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2),Field::<u8>(Variant(_38, 0), 2)];
(*_43) = _16 & _16;
(*_23) = &mut place!(Field::<u8>(Variant(_38, 0), 2));
(*_43) = _16;
(*_24) = (*_43) != (*_43);
Goto(bb17)
}
bb17 = {
_50 = 222140424653020979882263922735729408476_u128;
(*_24) = false & false;
match _50 {
0 => bb11,
1 => bb2,
2 => bb5,
3 => bb18,
4 => bb19,
222140424653020979882263922735729408476 => bb21,
_ => bb20
}
}
bb18 = {
(*_14) = 9223372036854775807_isize;
Call((*_14) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_11 = _4 as f32;
_3 = _5 ^ _5;
_5 = _3;
_10 = [13378267753434425372_usize,2_usize,6569209418661572405_usize,7_usize,3723250223892773070_usize,4_usize];
_4 = 6993832938444239758_u64;
_6 = 2360_i16 + (-830_i16);
_6 = !23261_i16;
_8 = _7;
_2 = _7;
_7 = _8;
_2 = _7;
_11 = 46017_u16 as f32;
_3 = !_5;
_1 = -(-14956167896067825598334963332765073124_i128);
_11 = 224_u8 as f32;
_8 = _2;
_10 = [14572818896478392122_usize,9697813282063629580_usize,4_usize,4_usize,10118419479538180808_usize,11274127403141525392_usize];
_1 = (-39276798103313727610094407538968982757_i128);
RET = [_2];
_7 = _8;
_10 = [7_usize,6828317630341108134_usize,7913289401590057249_usize,6_usize,1_usize,0_usize];
_4 = 3873552190146728772_u64 + 8451243369556598954_u64;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_14) = _7 as isize;
(*_14) = 9223372036854775807_isize ^ (-65_isize);
Goto(bb2)
}
bb20 = {
(*_14) = -_16;
(*_14) = -_16;
_9 = _16 ^ _16;
(*_14) = _16;
_24 = &mut _21;
_14 = core::ptr::addr_of_mut!((*_14));
_32 = _6 - _6;
_29 = _7;
(*_14) = !_16;
_8 = _2;
_1 = 44534170646434227121393927517369567659_i128 + (-136238874622172260424771203862069900215_i128);
(*_24) = !true;
_15 = _4 ^ _4;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
5477765985521074611 => bb11,
_ => bb10
}
}
bb21 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb22 = {
(*_14) = _36 - _16;
_6 = !_32;
_17 = [_49,_49,_3,_5,_5,_3,_5,_3];
_35 = _31.2 as f32;
_33 = _3 as f32;
_32 = _6;
(*_24) = !_47;
_45 = !_6;
(*_24) = _47;
(*_14) = _36;
_49 = _39 as u32;
_60 = _27;
_17 = [_49,_5,_49,_49,_5,_49,_49,_49];
_2 = _8;
_37.2 = -_31.2;
(*_14) = _36;
(*_14) = _36 >> _36;
(*_24) = _47;
match _50 {
0 => bb19,
1 => bb9,
2 => bb5,
3 => bb13,
4 => bb23,
222140424653020979882263922735729408476 => bb25,
_ => bb24
}
}
bb23 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb24 = {
Return()
}
bb25 = {
(*_24) = _47 & _47;
Goto(bb26)
}
bb26 = {
_19 = 47896_u16;
_57 = [1_usize,7_usize,17716750927887786660_usize,17573656564475644109_usize,7195034393986415598_usize,1_usize];
_19 = !15092_u16;
(*_14) = _50 as isize;
(*_14) = !_16;
_52 = (*_14) << (*_14);
(*_24) = !_47;
_43 = core::ptr::addr_of!(_16);
_56 = &_3;
_33 = -_39;
_35 = _39 * _33;
_46 = 220_u8;
(*_43) = _1 as isize;
_10 = [6_usize,14210037876209500652_usize,9663014750082390330_usize,6_usize,7_usize,5_usize];
_11 = _33;
_61 = [3586565716790126752_usize,192868599852446307_usize,5483325745991865070_usize,15597877257610569068_usize];
match _50 {
0 => bb23,
1 => bb2,
2 => bb17,
3 => bb22,
222140424653020979882263922735729408476 => bb28,
_ => bb27
}
}
bb27 = {
_11 = _4 as f32;
_3 = _5 ^ _5;
_5 = _3;
_10 = [13378267753434425372_usize,2_usize,6569209418661572405_usize,7_usize,3723250223892773070_usize,4_usize];
_4 = 6993832938444239758_u64;
_6 = 2360_i16 + (-830_i16);
_6 = !23261_i16;
_8 = _7;
_2 = _7;
_7 = _8;
_2 = _7;
_11 = 46017_u16 as f32;
_3 = !_5;
_1 = -(-14956167896067825598334963332765073124_i128);
_11 = 224_u8 as f32;
_8 = _2;
_10 = [14572818896478392122_usize,9697813282063629580_usize,4_usize,4_usize,10118419479538180808_usize,11274127403141525392_usize];
_1 = (-39276798103313727610094407538968982757_i128);
RET = [_2];
_7 = _8;
_10 = [7_usize,6828317630341108134_usize,7913289401590057249_usize,6_usize,1_usize,0_usize];
_4 = 3873552190146728772_u64 + 8451243369556598954_u64;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_14) = _7 as isize;
(*_14) = 9223372036854775807_isize ^ (-65_isize);
Goto(bb2)
}
bb28 = {
_52 = (*_14) * (*_14);
match _46 {
0 => bb18,
1 => bb7,
2 => bb3,
220 => bb30,
_ => bb29
}
}
bb29 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb30 = {
_61 = [6176280687097800587_usize,520951444609005396_usize,6_usize,2059986924620117365_usize];
(*_24) = !_47;
_39 = _33 * _35;
_63 = 2396883855038675637_usize ^ 2_usize;
(*_24) = _39 > _35;
(*_24) = _39 >= _11;
(*_43) = _39 as isize;
_31.2 = -_37.2;
_9 = (*_43);
_54 = !_4;
(*_24) = _47 & _47;
(*_43) = (*_14);
_14 = core::ptr::addr_of_mut!(_41);
(*_14) = !(*_43);
_19 = 27196_u16 >> (*_14);
(*_23) = &mut _46;
_41 = _52 - (*_43);
(*_14) = (*_43) << _52;
(*_14) = (*_43);
(*_24) = _47;
Goto(bb31)
}
bb31 = {
_55 = [97_i8,79_i8,(-37_i8),(-68_i8)];
(*_24) = _47 | _47;
(*_14) = -(*_43);
_13 = [_63,_63,_63,_63,_63,_63];
RET = [_8];
(*_14) = _9;
_43 = core::ptr::addr_of!((*_43));
(*_14) = (*_43);
_39 = -_33;
(*_24) = !_47;
(*_43) = _41;
_28 = _2;
_65 = (*_14) & (*_43);
_53 = _28;
_44 = (-1484737107_i32) as usize;
_60 = [(-110_i8),(-39_i8),(-67_i8)];
_2 = _28;
(*_14) = 15_u8 as isize;
Call((*_14) = core::intrinsics::transmute((*_43)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_63 = !_44;
(*_43) = 120_i8 as isize;
_64 = _35 - _11;
(*_14) = _63 as isize;
_29 = _2;
_53 = _8;
(*_24) = _16 < _65;
(*_43) = (*_14);
_62 = _7;
Goto(bb33)
}
bb33 = {
(*_43) = _65;
_43 = core::ptr::addr_of!((*_43));
_19 = 19818_u16 * 34354_u16;
(*_24) = _47;
_39 = _44 as f32;
(*_43) = -_65;
(*_43) = _36 + _65;
_44 = _63 + _63;
match _50 {
0 => bb34,
1 => bb35,
2 => bb36,
3 => bb37,
4 => bb38,
5 => bb39,
6 => bb40,
222140424653020979882263922735729408476 => bb42,
_ => bb41
}
}
bb34 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb35 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb36 = {
(*_14) = _16 ^ _16;
_19 = 31002_u16 ^ 47399_u16;
_19 = 60890_u16 + 42232_u16;
(*_24) = false | true;
(*_24) = (*_14) >= (*_14);
_3 = !_5;
(*_14) = 12991216499431938638_usize as isize;
(*_14) = _15 as isize;
_29 = _7;
(*_24) = !true;
(*_14) = _16 & _16;
(*_24) = false;
(*_24) = false | true;
_14 = core::ptr::addr_of_mut!((*_14));
(*_24) = true;
_13 = [6_usize,15252476677969026338_usize,5_usize,9095939657881628223_usize,5_usize,7_usize];
_13 = _10;
_30 = _1;
match _4 {
0 => bb1,
1 => bb10,
2 => bb9,
3 => bb4,
4 => bb8,
5 => bb6,
5477765985521074611 => bb12,
_ => bb7
}
}
bb37 = {
(*_43) = _16;
(*_24) = !false;
(*_24) = !true;
(*_24) = true & false;
(*_24) = true;
(*_24) = !false;
_3 = _5;
_53 = _28;
(*_14) = !_36;
_47 = !(*_24);
_4 = !_15;
_27 = [(-114_i8),112_i8,8_i8];
(*_14) = 764049178_i32 as isize;
(*_24) = !_47;
(*_14) = _16 >> _49;
_37.2 = _31.2 + _31.2;
_36 = (*_14) + (*_14);
_19 = 42381_u16 ^ 48884_u16;
_54 = _1 as u64;
_5 = _49;
(*_24) = _47;
_4 = _54;
_4 = !_54;
Goto(bb22)
}
bb38 = {
_52 = (*_14) * (*_14);
match _46 {
0 => bb18,
1 => bb7,
2 => bb3,
220 => bb30,
_ => bb29
}
}
bb39 = {
_11 = _4 as f32;
_3 = _5 ^ _5;
_5 = _3;
_10 = [13378267753434425372_usize,2_usize,6569209418661572405_usize,7_usize,3723250223892773070_usize,4_usize];
_4 = 6993832938444239758_u64;
_6 = 2360_i16 + (-830_i16);
_6 = !23261_i16;
_8 = _7;
_2 = _7;
_7 = _8;
_2 = _7;
_11 = 46017_u16 as f32;
_3 = !_5;
_1 = -(-14956167896067825598334963332765073124_i128);
_11 = 224_u8 as f32;
_8 = _2;
_10 = [14572818896478392122_usize,9697813282063629580_usize,4_usize,4_usize,10118419479538180808_usize,11274127403141525392_usize];
_1 = (-39276798103313727610094407538968982757_i128);
RET = [_2];
_7 = _8;
_10 = [7_usize,6828317630341108134_usize,7913289401590057249_usize,6_usize,1_usize,0_usize];
_4 = 3873552190146728772_u64 + 8451243369556598954_u64;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_14) = _7 as isize;
(*_14) = 9223372036854775807_isize ^ (-65_isize);
Goto(bb2)
}
bb40 = {
(*_14) = 9223372036854775807_isize;
Call((*_14) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb3), UnwindUnreachable())
}
bb41 = {
place!(Field::<u64>(Variant(_38, 0), 3)) = !_15;
_40.fld2 = 118_u8 & 169_u8;
(*_24) = false;
place!(Field::<char>(Variant(_38, 0), 1)) = _28;
_16 = (*_14) ^ (*_14);
RET = [_2];
_8 = _28;
(*_24) = false;
Call(_5 = core::intrinsics::transmute(_2), ReturnTo(bb15), UnwindUnreachable())
}
bb42 = {
_27 = [49_i8,8_i8,77_i8];
_43 = core::ptr::addr_of!((*_14));
_66 = &_27;
_61 = [_63,_44,_44,_44];
(*_43) = _65;
(*_14) = -_9;
(*_14) = _31.2 as isize;
(*_24) = _16 >= (*_14);
(*_14) = !_16;
(*_24) = (*_14) >= (*_14);
_61 = [_44,_63,_44,_44];
(*_24) = _47;
(*_14) = -_16;
Goto(bb43)
}
bb43 = {
_56 = &_5;
(*_24) = !_47;
_8 = _2;
_8 = _53;
(*_24) = _47 | _47;
_32 = _6 << (*_14);
_47 = (*_14) > (*_14);
(*_24) = (*_14) > (*_14);
_56 = &_49;
_18 = _55;
_37.2 = -_31.2;
_41 = !_9;
_75 = [(*_24),(*_24),(*_24),(*_24),(*_24),(*_24),(*_24)];
_6 = _32 ^ _32;
_67 = core::ptr::addr_of_mut!(_50);
(*_67) = !304414924045230214021355221468945720542_u128;
_11 = _35 * _35;
Goto(bb44)
}
bb44 = {
(*_14) = _9 - _36;
_39 = _35 - _11;
(*_67) = 151305455592422571928284496759385880361_u128;
_43 = core::ptr::addr_of!((*_14));
_60 = [113_i8,37_i8,39_i8];
(*_14) = _16 + _9;
(*_24) = _47 ^ _47;
_74.0 = [_19,_19,_19,_19];
_31.2 = -_37.2;
_8 = _29;
(*_67) = _30 as u128;
(*_67) = 590216123516868349171507948987078303_u128 & 159501656190850746449339050859712653845_u128;
(*_67) = _1 as u128;
_44 = _63 + _63;
(*_24) = (*_14) < (*_14);
(*_14) = !_9;
_28 = _8;
(*_67) = !133419611158272165086563309617215842384_u128;
_77 = _7;
(*_14) = -_65;
(*_14) = !_16;
_37.2 = _31.2;
(*_24) = _47 & _47;
(*_14) = _9 ^ _16;
_70 = &_74.0;
Goto(bb45)
}
bb45 = {
_17 = [(*_56),(*_56),(*_56),(*_56),(*_56),(*_56),_5,(*_56)];
(*_67) = _4 as u128;
_76 = core::ptr::addr_of_mut!(_72);
(*_67) = !55358841020360614166047491260874548060_u128;
_31.2 = -_37.2;
_74.1 = _63 as f32;
_73 = _41;
(*_14) = _65 & _65;
(*_76) = (*_14);
_24 = &mut _47;
_60 = (*_66);
_31.2 = _1 as f64;
_56 = &_5;
(*_76) = _30 as isize;
_9 = (*_14) - (*_14);
(*_67) = _39 as u128;
(*_67) = 79058100646367540707086558111644454709_u128 + 297153637884736591026306659249951943395_u128;
_37.2 = _44 as f64;
(*_76) = (*_14) * (*_14);
_55 = _18;
(*_76) = _9 ^ (*_14);
(*_76) = _73 >> (*_14);
_63 = _44 & _44;
(*_24) = !true;
_44 = !_63;
(*_24) = true | false;
(*_24) = false;
_77 = _7;
(*_14) = -(*_76);
(*_14) = -(*_76);
_31.2 = _37.2 + _37.2;
Goto(bb46)
}
bb46 = {
_13 = [_44,_63,_44,_63,_63,_44];
(*_67) = 205343605057679479933463611924535844239_u128 << (*_76);
(*_14) = -(*_76);
(*_24) = (*_14) > _65;
(*_76) = 91_u8 as isize;
_31.2 = -_37.2;
(*_14) = _65 | _52;
_11 = _39 * _35;
(*_24) = false | false;
(*_24) = _73 > (*_14);
(*_14) = _63 as isize;
(*_14) = _73;
_84 = (*_14) + _16;
(*_76) = _73 << (*_67);
_15 = _4 & _54;
Goto(bb47)
}
bb47 = {
(*_76) = (*_14) & (*_14);
_67 = core::ptr::addr_of_mut!((*_67));
(*_24) = (*_67) == (*_67);
_82 = _30;
(*_67) = 76444181011083436692593564932748919485_u128;
_62 = _53;
Goto(bb48)
}
bb48 = {
_83 = _35 + _11;
(*_14) = (*_76);
(*_14) = (*_76);
_7 = _28;
(*_76) = _41 * (*_14);
match (*_67) {
0 => bb24,
1 => bb11,
76444181011083436692593564932748919485 => bb49,
_ => bb34
}
}
bb49 = {
_50 = _37.2 as u128;
_6 = _32 + _32;
_44 = _63;
(*_24) = false & true;
_5 = _3;
(*_67) = !280424607389454658635792236661517370675_u128;
(*_76) = (*_14) + (*_14);
_72 = -(*_14);
_54 = _6 as u64;
(*_76) = _50 as isize;
(*_14) = _54 as isize;
(*_76) = -(*_14);
(*_14) = (*_24) as isize;
(*_67) = !62384825458464272185761713653887337956_u128;
(*_24) = !false;
Goto(bb50)
}
bb50 = {
Call(_90 = dump_var(Move(_1), Move(_47), Move(_75), Move(_50)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_90 = dump_var(Move(_3), Move(_62), Move(_54), Move(_29)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_90 = dump_var(Move(_52), Move(_28), Move(_46), Move(_10)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_90 = dump_var(Move(_42), Move(_72), Move(_30), Move(_21)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_90 = dump_var(Move(_2), Move(_17), Move(_8), Move(_9)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_90 = dump_var(Move(_5), Move(_16), Move(_61), _91), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: char,mut _3: *mut isize,mut _4: char,mut _5: isize) -> u64 {
mir! {
type RET = u64;
let _6: &'static mut [i128; 2];
let _7: *const isize;
let _8: (u64, [i128; 2]);
let _9: *const *mut [u128; 6];
let _10: char;
let _11: u16;
let _12: *const Adt27;
let _13: isize;
let _14: [i8; 8];
let _15: isize;
let _16: &'static &'static i64;
let _17: (u64, [i128; 2]);
let _18: &'static [i8; 3];
let _19: f64;
let _20: ([u16; 4], f32);
let _21: isize;
let _22: f32;
let _23: isize;
let _24: &'static mut [i128; 2];
let _25: *mut isize;
let _26: &'static i64;
let _27: (u64, [i128; 2]);
let _28: *const Adt19;
let _29: i32;
let _30: Adt27;
let _31: u64;
let _32: isize;
let _33: i16;
let _34: ([u16; 4], f32);
let _35: &'static mut [u128; 6];
let _36: f64;
let _37: i64;
let _38: &'static [i8; 3];
let _39: &'static [u16; 4];
let _40: bool;
let _41: usize;
let _42: *const u32;
let _43: isize;
let _44: i8;
let _45: isize;
let _46: u64;
let _47: f64;
let _48: ([char; 1], Adt20);
let _49: isize;
let _50: &'static [i8; 3];
let _51: &'static mut bool;
let _52: *const Adt38;
let _53: [u8; 7];
let _54: i8;
let _55: ();
let _56: ();
{
_3 = core::ptr::addr_of_mut!(_5);
Goto(bb1)
}
bb1 = {
(*_3) = _1 * _1;
RET = 5477765985521074611_u64;
_5 = 159129235232296921325399278470828445273_i128 as isize;
(*_3) = (-9_i8) as isize;
(*_3) = -_1;
_4 = _2;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5477765985521074611 => bb8,
_ => bb7
}
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
(*_3) = _1 & _1;
Goto(bb9)
}
bb9 = {
(*_3) = _1;
(*_3) = _1 ^ _1;
(*_3) = _1 ^ _1;
(*_3) = _1 >> RET;
(*_3) = _1 ^ _1;
_4 = _2;
(*_3) = _1;
(*_3) = _1 << _1;
_8.0 = RET % RET;
_3 = core::ptr::addr_of_mut!((*_3));
_5 = !_1;
(*_3) = _1 << _1;
_7 = core::ptr::addr_of!((*_3));
(*_7) = _1 & _1;
(*_7) = _1 ^ _1;
_11 = 27493_u16 | 44492_u16;
(*_3) = -_1;
(*_3) = _1 << _1;
(*_3) = 1391484721_i32 as isize;
_10 = _4;
_1 = (*_3) << (*_3);
Goto(bb10)
}
bb10 = {
_8.1 = [(-130884193252230114971788174325987358704_i128),(-78719236747753117759006036981834371544_i128)];
(*_3) = _1;
(*_3) = _1;
(*_3) = _1 * _1;
(*_3) = _1 << _1;
(*_3) = _1 ^ _1;
(*_3) = _1;
(*_3) = _10 as isize;
(*_3) = _1 + _1;
_8.0 = !RET;
(*_3) = _1 * _1;
_6 = &mut _8.1;
(*_3) = _1 + _1;
(*_3) = !_1;
(*_3) = _1 - _1;
(*_3) = _1;
(*_6) = [(-88440413294325964377633489752811639099_i128),848033087411865731777544101230232273_i128];
_7 = core::ptr::addr_of!((*_3));
(*_6) = [(-41633040022822158699186838220894954997_i128),(-124478356969427886148759168298505888043_i128)];
(*_6) = [37230300089744763441440286412407884728_i128,(-141033795004832055967523635574196696134_i128)];
(*_7) = 11562393261818045557_usize as isize;
(*_6) = [(-168013972614800506469580205810244226665_i128),(-95357314125733387614726674729862781515_i128)];
(*_6) = [(-2528803910879121246332154835961004094_i128),108303863172065525228740848880978406649_i128];
(*_7) = 4487979026057784209_i64 as isize;
(*_6) = [70674754656940638855501629121874807998_i128,(-160728940933827163100226504075035762804_i128)];
(*_6) = [71652793922483614273822127492519234638_i128,1577853303135377350946211165486202337_i128];
Goto(bb11)
}
bb11 = {
(*_6) = [(-72902043834178541491635244614640744574_i128),(-114775843654994860197486187008430953653_i128)];
(*_6) = [67437061287460346496084686685755742072_i128,(-65917696659811517361509065012505436839_i128)];
(*_6) = [61665716865190276082336406218303646617_i128,(-133802251759133432483657531156454652126_i128)];
(*_6) = [100856275769702757642713449931600079430_i128,(-162581428885500952871857843001136160650_i128)];
(*_3) = _1;
(*_6) = [(-122704213380128003425567182152082622061_i128),(-134092327707859119861239986290261570754_i128)];
_19 = (-36239647899128818846037062712779719701_i128) as f64;
(*_6) = [(-84725194532561329190336437286658843886_i128),(-43930764362903760975381002390842517690_i128)];
_15 = _10 as isize;
(*_6) = [139065995331284910201610330915971527479_i128,(-60016383072707101655618373112104481713_i128)];
match RET {
0 => bb7,
5477765985521074611 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
(*_6) = [(-75551494569279985543669472890701831730_i128),(-122915816320771610957477138035523603677_i128)];
(*_3) = 68_u8 as isize;
(*_3) = _1 << _11;
_15 = (*_3) << (*_3);
Goto(bb14)
}
bb14 = {
(*_6) = [68757973415409585709788565935335046299_i128,95908015234724829574483888271723776838_i128];
(*_6) = [(-69870140889657702179308647881007281081_i128),(-146516668242875889986858913603867756417_i128)];
(*_6) = [166452933301173689942623843870207495563_i128,(-89587158737993665682206058569326224064_i128)];
(*_6) = [(-46066942716546785541172331815173245709_i128),(-135174845606028172119408371746948945157_i128)];
(*_6) = [(-82469717769529507110116800531062710516_i128),(-86919692146740178259252287919997475266_i128)];
_5 = _15 | _1;
(*_6) = [87829932532821954699140435625189216982_i128,(-107072428935929993021994162649488561625_i128)];
(*_6) = [17779627461234554878019541280514116998_i128,(-74660162140660301117595154020137002761_i128)];
(*_3) = 2732909031_u32 as isize;
(*_3) = _10 as isize;
_17 = (RET, (*_6));
_17 = (RET, (*_6));
(*_3) = (-1718502132_i32) as isize;
(*_3) = _15 & _15;
_14 = [(-91_i8),(-124_i8),50_i8,34_i8,(-41_i8),61_i8,(-69_i8),(-7_i8)];
_23 = 9687879227466556669_usize as isize;
Call(_13 = core::intrinsics::bswap((*_3)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_6) = [83922120070163343032757960269737076087_i128,83049187798928251373424758232926992830_i128];
_7 = core::ptr::addr_of!(_13);
(*_7) = (*_3) * (*_3);
(*_3) = _13 + (*_7);
(*_6) = [91044132996015699255194992532091633310_i128,7115063070595718391208157721574903417_i128];
(*_7) = (*_3) | (*_3);
(*_7) = (*_3) * (*_3);
(*_7) = (*_3);
(*_7) = (*_3);
(*_6) = [76884152062882583518229452829869984784_i128,(-26528389338033508800469031860927710974_i128)];
Call((*_3) = core::intrinsics::transmute((*_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*_6) = _17.1;
(*_7) = _11 as isize;
(*_3) = (*_7) | (*_7);
(*_6) = _17.1;
(*_6) = [(-37906961794276899611151508648327236660_i128),(-41351166615276000959469743091137028776_i128)];
_23 = (*_7) - (*_3);
(*_7) = -(*_3);
_22 = 230_u8 as f32;
_19 = 231141705450010137686247222627622529307_u128 as f64;
_5 = (*_7) << (*_7);
_20.0 = [_11,_11,_11,_11];
(*_6) = [64327788608465149170350042857170084047_i128,92014916527263193909207158264346478894_i128];
(*_7) = _23 << (*_3);
Goto(bb17)
}
bb17 = {
_11 = 47053_u16 & 35572_u16;
_17.0 = RET % RET;
(*_3) = (*_7) >> (*_7);
Call(_17.0 = fn3(_17.1, Move(_7), (*_6), _5, Move(_6), (*_7), (*_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_3) = _13 * _23;
(*_3) = 187_u8 as isize;
_4 = _10;
_23 = !_15;
match RET {
0 => bb12,
1 => bb2,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb16,
6 => bb15,
5477765985521074611 => bb19,
_ => bb17
}
}
bb19 = {
(*_3) = _13 - _13;
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb20)
}
bb20 = {
_27.1 = [(-26337807669262527383094688887713261257_i128),(-7565257535185544090044108234374891345_i128)];
_14 = [93_i8,80_i8,(-56_i8),97_i8,10_i8,55_i8,(-3_i8),56_i8];
_27.0 = _17.0;
_27.1 = [106094531749715886086817801614534129799_i128,(-144599364742454992036665794013957441938_i128)];
_20.1 = (*_3) as f32;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = (-1566510484_i32) as isize;
(*_3) = _15 >> _17.0;
_20.1 = 99172970763628680056572748765763924290_i128 as f32;
(*_3) = _1 * _15;
_25 = core::ptr::addr_of_mut!((*_3));
Goto(bb21)
}
bb21 = {
(*_25) = _23 * _23;
_5 = -_15;
_30.fld3 = _17.0 * _17.0;
_25 = core::ptr::addr_of_mut!((*_25));
_30.fld0 = _19;
_12 = core::ptr::addr_of!(_30);
(*_12).fld2 = !158_u8;
(*_12).fld5 = _22 + _22;
_10 = _4;
(*_12).fld1 = Adt20::Variant2 { fld0: 1275109065_i32 };
(*_12).fld2 = 220_u8 << (*_12).fld3;
(*_12).fld3 = (-112934339651267018008075775836700102563_i128) as u64;
(*_12).fld0 = _19 + _19;
_20.1 = _27.0 as f32;
Goto(bb22)
}
bb22 = {
(*_12).fld2 = 12_u8;
(*_12).fld0 = -_19;
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = (-1030188455_i32) & 220880010_i32;
_3 = core::ptr::addr_of_mut!((*_25));
_30.fld1 = Adt20::Variant0 { fld0: (-47374194267769677942980437455102425850_i128),fld1: _10,fld2: (*_12).fld2,fld3: _17.0,fld4: 102813641_i32 };
Goto(bb23)
}
bb23 = {
(*_12).fld1 = Adt20::Variant0 { fld0: (-49370464799253483171399163375921343158_i128),fld1: _2,fld2: (*_12).fld2,fld3: _27.0,fld4: (-2034842578_i32) };
match Field::<u8>(Variant(_30.fld1, 0), 2) {
0 => bb24,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
12 => bb32,
_ => bb31
}
}
bb24 = {
(*_12).fld2 = 12_u8;
(*_12).fld0 = -_19;
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = (-1030188455_i32) & 220880010_i32;
_3 = core::ptr::addr_of_mut!((*_25));
_30.fld1 = Adt20::Variant0 { fld0: (-47374194267769677942980437455102425850_i128),fld1: _10,fld2: (*_12).fld2,fld3: _17.0,fld4: 102813641_i32 };
Goto(bb23)
}
bb25 = {
(*_25) = _23 * _23;
_5 = -_15;
_30.fld3 = _17.0 * _17.0;
_25 = core::ptr::addr_of_mut!((*_25));
_30.fld0 = _19;
_12 = core::ptr::addr_of!(_30);
(*_12).fld2 = !158_u8;
(*_12).fld5 = _22 + _22;
_10 = _4;
(*_12).fld1 = Adt20::Variant2 { fld0: 1275109065_i32 };
(*_12).fld2 = 220_u8 << (*_12).fld3;
(*_12).fld3 = (-112934339651267018008075775836700102563_i128) as u64;
(*_12).fld0 = _19 + _19;
_20.1 = _27.0 as f32;
Goto(bb22)
}
bb26 = {
(*_3) = _1 * _1;
RET = 5477765985521074611_u64;
_5 = 159129235232296921325399278470828445273_i128 as isize;
(*_3) = (-9_i8) as isize;
(*_3) = -_1;
_4 = _2;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5477765985521074611 => bb8,
_ => bb7
}
}
bb27 = {
(*_3) = _13 - _13;
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb20)
}
bb28 = {
Return()
}
bb29 = {
_8.1 = [(-130884193252230114971788174325987358704_i128),(-78719236747753117759006036981834371544_i128)];
(*_3) = _1;
(*_3) = _1;
(*_3) = _1 * _1;
(*_3) = _1 << _1;
(*_3) = _1 ^ _1;
(*_3) = _1;
(*_3) = _10 as isize;
(*_3) = _1 + _1;
_8.0 = !RET;
(*_3) = _1 * _1;
_6 = &mut _8.1;
(*_3) = _1 + _1;
(*_3) = !_1;
(*_3) = _1 - _1;
(*_3) = _1;
(*_6) = [(-88440413294325964377633489752811639099_i128),848033087411865731777544101230232273_i128];
_7 = core::ptr::addr_of!((*_3));
(*_6) = [(-41633040022822158699186838220894954997_i128),(-124478356969427886148759168298505888043_i128)];
(*_6) = [37230300089744763441440286412407884728_i128,(-141033795004832055967523635574196696134_i128)];
(*_7) = 11562393261818045557_usize as isize;
(*_6) = [(-168013972614800506469580205810244226665_i128),(-95357314125733387614726674729862781515_i128)];
(*_6) = [(-2528803910879121246332154835961004094_i128),108303863172065525228740848880978406649_i128];
(*_7) = 4487979026057784209_i64 as isize;
(*_6) = [70674754656940638855501629121874807998_i128,(-160728940933827163100226504075035762804_i128)];
(*_6) = [71652793922483614273822127492519234638_i128,1577853303135377350946211165486202337_i128];
Goto(bb11)
}
bb30 = {
(*_6) = _17.1;
(*_7) = _11 as isize;
(*_3) = (*_7) | (*_7);
(*_6) = _17.1;
(*_6) = [(-37906961794276899611151508648327236660_i128),(-41351166615276000959469743091137028776_i128)];
_23 = (*_7) - (*_3);
(*_7) = -(*_3);
_22 = 230_u8 as f32;
_19 = 231141705450010137686247222627622529307_u128 as f64;
_5 = (*_7) << (*_7);
_20.0 = [_11,_11,_11,_11];
(*_6) = [64327788608465149170350042857170084047_i128,92014916527263193909207158264346478894_i128];
(*_7) = _23 << (*_3);
Goto(bb17)
}
bb31 = {
Return()
}
bb32 = {
(*_25) = -_13;
place!(Field::<u64>(Variant((*_12).fld1, 0), 3)) = !_27.0;
(*_12).fld1 = Adt20::Variant2 { fld0: (-64742514_i32) };
(*_12).fld5 = 5768484388845910877394627184136078706_i128 as f32;
(*_12).fld4 = (-2415_i16) & 11310_i16;
Goto(bb33)
}
bb33 = {
(*_12).fld1 = Adt20::Variant2 { fld0: 482337620_i32 };
(*_12).fld2 = 7_u8 << _15;
_24 = &mut _17.1;
(*_12).fld4 = 19163_i16;
(*_12).fld2 = 83_u8;
(*_12).fld3 = _27.0 / RET;
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = -(-950256150_i32);
match (*_12).fld2 {
0 => bb4,
1 => bb34,
2 => bb35,
3 => bb36,
83 => bb38,
_ => bb37
}
}
bb34 = {
(*_12).fld2 = 12_u8;
(*_12).fld0 = -_19;
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = (-1030188455_i32) & 220880010_i32;
_3 = core::ptr::addr_of_mut!((*_25));
_30.fld1 = Adt20::Variant0 { fld0: (-47374194267769677942980437455102425850_i128),fld1: _10,fld2: (*_12).fld2,fld3: _17.0,fld4: 102813641_i32 };
Goto(bb23)
}
bb35 = {
Return()
}
bb36 = {
(*_6) = _17.1;
(*_7) = _11 as isize;
(*_3) = (*_7) | (*_7);
(*_6) = _17.1;
(*_6) = [(-37906961794276899611151508648327236660_i128),(-41351166615276000959469743091137028776_i128)];
_23 = (*_7) - (*_3);
(*_7) = -(*_3);
_22 = 230_u8 as f32;
_19 = 231141705450010137686247222627622529307_u128 as f64;
_5 = (*_7) << (*_7);
_20.0 = [_11,_11,_11,_11];
(*_6) = [64327788608465149170350042857170084047_i128,92014916527263193909207158264346478894_i128];
(*_7) = _23 << (*_3);
Goto(bb17)
}
bb37 = {
(*_3) = _13 - _13;
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb20)
}
bb38 = {
(*_12).fld4 = 25_i8 as i16;
_21 = (*_25) & (*_25);
(*_12).fld4 = 13656_i16 - 10477_i16;
(*_25) = _13 >> (*_12).fld3;
_30.fld4 = 8890_i16 << _1;
_6 = &mut (*_24);
(*_12).fld4 = 20016_i16 + 18347_i16;
(*_12).fld0 = (*_12).fld5 as f64;
(*_12).fld5 = _22 + _20.1;
(*_12).fld1 = Adt20::Variant2 { fld0: 173521810_i32 };
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = 85441719_i32 & 58992290_i32;
(*_6) = [132243757323326528767513831859224244412_i128,(-68171954669568230041611476073594846700_i128)];
(*_12).fld1 = Adt20::Variant0 { fld0: (-86533034878497470962963585750118239634_i128),fld1: _2,fld2: (*_12).fld2,fld3: (*_12).fld3,fld4: 163321804_i32 };
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = 210125219_i32;
(*_12).fld2 = !Field::<u8>(Variant((*_12).fld1, 0), 2);
place!(Field::<i128>(Variant((*_12).fld1, 0), 0)) = -68021059612213256506079166074653772789_i128;
(*_12).fld3 = (-4486572272133055164_i64) as u64;
(*_12).fld3 = Field::<u64>(Variant((*_12).fld1, 0), 3);
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = (*_12).fld0 as i32;
(*_12).fld3 = !Field::<u64>(Variant((*_12).fld1, 0), 3);
(*_12).fld2 = Field::<i128>(Variant((*_12).fld1, 0), 0) as u8;
match Field::<u8>(Variant((*_12).fld1, 0), 2) {
0 => bb11,
1 => bb10,
2 => bb36,
3 => bb22,
4 => bb23,
5 => bb20,
6 => bb39,
83 => bb41,
_ => bb40
}
}
bb39 = {
(*_25) = _23 * _23;
_5 = -_15;
_30.fld3 = _17.0 * _17.0;
_25 = core::ptr::addr_of_mut!((*_25));
_30.fld0 = _19;
_12 = core::ptr::addr_of!(_30);
(*_12).fld2 = !158_u8;
(*_12).fld5 = _22 + _22;
_10 = _4;
(*_12).fld1 = Adt20::Variant2 { fld0: 1275109065_i32 };
(*_12).fld2 = 220_u8 << (*_12).fld3;
(*_12).fld3 = (-112934339651267018008075775836700102563_i128) as u64;
(*_12).fld0 = _19 + _19;
_20.1 = _27.0 as f32;
Goto(bb22)
}
bb40 = {
Return()
}
bb41 = {
_15 = 6_usize as isize;
place!(Field::<u64>(Variant(_30.fld1, 0), 3)) = (*_12).fld3;
(*_6) = [Field::<i128>(Variant((*_12).fld1, 0), 0),Field::<i128>(Variant((*_12).fld1, 0), 0)];
place!(Field::<i128>(Variant((*_12).fld1, 0), 0)) = 103007798293069661859638287405375552911_i128 << Field::<u64>(Variant((*_12).fld1, 0), 3);
_27 = ((*_12).fld3, (*_6));
place!(Field::<i32>(Variant(_30.fld1, 0), 4)) = (-1736358203_i32) * 1894942595_i32;
(*_12).fld3 = Field::<u64>(Variant((*_12).fld1, 0), 3) / RET;
(*_12).fld1 = Adt20::Variant2 { fld0: 1642177475_i32 };
_36 = -(*_12).fld0;
(*_12).fld1 = Adt20::Variant1 { fld0: (*_12).fld4 };
(*_12).fld1 = Adt20::Variant2 { fld0: (-1321865948_i32) };
(*_6) = [(-17061736056553590259638328704153521219_i128),62183128976613463896573994027055644653_i128];
_37 = 8422999244077822482_i64 - 1422832106046259599_i64;
_27 = ((*_12).fld3, (*_6));
place!(Field::<i32>(Variant((*_12).fld1, 2), 0)) = !1486941207_i32;
(*_12).fld2 = 231_u8;
(*_25) = (*_12).fld3 as isize;
(*_25) = _13;
(*_12).fld5 = -_20.1;
(*_12).fld1 = Adt20::Variant0 { fld0: 6965558200188776418426862035394777563_i128,fld1: _4,fld2: (*_12).fld2,fld3: (*_12).fld3,fld4: 1632364830_i32 };
(*_12).fld4 = 5704_i16 * (-6118_i16);
_25 = core::ptr::addr_of_mut!((*_25));
Goto(bb42)
}
bb42 = {
_33 = (*_25) as i16;
_34 = (_20.0, (*_12).fld5);
(*_12).fld2 = 6830162706758482835_usize as u8;
place!(Field::<char>(Variant((*_12).fld1, 0), 1)) = _4;
(*_12).fld4 = (*_12).fld5 as i16;
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = 1414772312_i32 << (*_12).fld4;
place!(Field::<i128>(Variant((*_12).fld1, 0), 0)) = -151073512938849670312774411524870511906_i128;
place!(Field::<u8>(Variant((*_12).fld1, 0), 2)) = (*_12).fld2;
(*_12).fld4 = _33 >> _5;
_46 = Field::<u64>(Variant((*_12).fld1, 0), 3) | (*_12).fld3;
(*_12).fld1 = Adt20::Variant0 { fld0: 86825744585594184934161030367508687571_i128,fld1: _4,fld2: (*_12).fld2,fld3: _46,fld4: 1303979366_i32 };
(*_6) = [63090809558953709233520977974130489486_i128,124629472903577143410191507986805735065_i128];
place!(Field::<char>(Variant((*_12).fld1, 0), 1)) = _2;
(*_12).fld5 = _20.1 - _34.1;
(*_12).fld4 = _33 * _33;
_43 = !(*_25);
(*_6) = [155103374275915941789068372714282709557_i128,164179757341160732442401311796384014116_i128];
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = 1568801031_i32;
match Field::<i32>(Variant((*_12).fld1, 0), 4) {
0 => bb18,
1568801031 => bb44,
_ => bb43
}
}
bb43 = {
Return()
}
bb44 = {
(*_12).fld1 = Adt20::Variant0 { fld0: (-53238851946148936612704209305303428516_i128),fld1: _2,fld2: (*_12).fld2,fld3: (*_12).fld3,fld4: (-1077761277_i32) };
(*_12).fld3 = (*_12).fld5 as u64;
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = (-1699377329_i32) * 927119347_i32;
(*_25) = _43;
(*_6) = _27.1;
place!(Field::<i32>(Variant((*_12).fld1, 0), 4)) = -1270908807_i32;
place!(Field::<u64>(Variant((*_12).fld1, 0), 3)) = (*_12).fld3 & (*_12).fld3;
(*_12).fld3 = !Field::<u64>(Variant((*_12).fld1, 0), 3);
(*_12).fld1 = Adt20::Variant0 { fld0: 167742540904550580624281432107793878278_i128,fld1: _10,fld2: (*_12).fld2,fld3: (*_12).fld3,fld4: 1047877305_i32 };
place!(Field::<i128>(Variant((*_12).fld1, 0), 0)) = (-153181377593782085313002715550228226085_i128) & (-81330303944259972122523355350936038453_i128);
(*_12).fld1 = Adt20::Variant1 { fld0: (*_12).fld4 };
_34.0 = [_11,_11,_11,_11];
_36 = (*_25) as f64;
(*_12).fld4 = Field::<i16>(Variant((*_12).fld1, 1), 0) >> (*_12).fld3;
_48.0 = [_10];
_4 = _2;
(*_6) = [(-110600571012821107902769607458738469476_i128),92025537958431451651881715674181727266_i128];
(*_25) = (-221908371_i32) as isize;
(*_12).fld0 = _36 + _36;
_29 = (-797673240_i32);
_41 = 0_usize & 0_usize;
_32 = (*_12).fld3 as isize;
(*_12).fld0 = -_36;
(*_12).fld3 = !_46;
(*_12).fld1 = Adt20::Variant1 { fld0: (*_12).fld4 };
Goto(bb45)
}
bb45 = {
(*_25) = _32 * _32;
place!(Field::<i16>(Variant((*_12).fld1, 1), 0)) = -(*_12).fld4;
(*_12).fld3 = !_46;
(*_12).fld5 = _20.1 - _20.1;
(*_12).fld0 = _36 - _36;
_10 = _4;
match _29 {
0 => bb28,
1 => bb15,
2 => bb35,
3 => bb4,
4 => bb46,
5 => bb47,
340282366920938463463374607430970538216 => bb49,
_ => bb48
}
}
bb46 = {
(*_6) = _17.1;
(*_7) = _11 as isize;
(*_3) = (*_7) | (*_7);
(*_6) = _17.1;
(*_6) = [(-37906961794276899611151508648327236660_i128),(-41351166615276000959469743091137028776_i128)];
_23 = (*_7) - (*_3);
(*_7) = -(*_3);
_22 = 230_u8 as f32;
_19 = 231141705450010137686247222627622529307_u128 as f64;
_5 = (*_7) << (*_7);
_20.0 = [_11,_11,_11,_11];
(*_6) = [64327788608465149170350042857170084047_i128,92014916527263193909207158264346478894_i128];
(*_7) = _23 << (*_3);
Goto(bb17)
}
bb47 = {
(*_3) = _1 * _1;
RET = 5477765985521074611_u64;
_5 = 159129235232296921325399278470828445273_i128 as isize;
(*_3) = (-9_i8) as isize;
(*_3) = -_1;
_4 = _2;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5477765985521074611 => bb8,
_ => bb7
}
}
bb48 = {
(*_25) = _23 * _23;
_5 = -_15;
_30.fld3 = _17.0 * _17.0;
_25 = core::ptr::addr_of_mut!((*_25));
_30.fld0 = _19;
_12 = core::ptr::addr_of!(_30);
(*_12).fld2 = !158_u8;
(*_12).fld5 = _22 + _22;
_10 = _4;
(*_12).fld1 = Adt20::Variant2 { fld0: 1275109065_i32 };
(*_12).fld2 = 220_u8 << (*_12).fld3;
(*_12).fld3 = (-112934339651267018008075775836700102563_i128) as u64;
(*_12).fld0 = _19 + _19;
_20.1 = _27.0 as f32;
Goto(bb22)
}
bb49 = {
(*_12).fld2 = 157_u8 ^ 210_u8;
(*_25) = (*_12).fld0 as isize;
(*_12).fld5 = _11 as f32;
(*_12).fld1 = Adt20::Variant0 { fld0: 94901477996686430171967667793730716138_i128,fld1: _10,fld2: (*_12).fld2,fld3: (*_12).fld3,fld4: _29 };
place!(Field::<u8>(Variant((*_12).fld1, 0), 2)) = (-119_i8) as u8;
Goto(bb50)
}
bb50 = {
Call(_55 = dump_var(Move(_14), Move(_1), Move(_4), Move(_17)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_55 = dump_var(Move(_32), Move(_29), Move(_37), Move(_41)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_55 = dump_var(Move(_23), Move(_2), _56, _56), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i128; 2],mut _2: *const isize,mut _3: [i128; 2],mut _4: isize,mut _5: &'static mut [i128; 2],mut _6: isize,mut _7: isize) -> u64 {
mir! {
type RET = u64;
let _8: u64;
let _9: Adt31;
let _10: &'static Adt27;
let _11: &'static mut &'static mut u8;
let _12: f64;
let _13: [usize; 4];
let _14: *mut isize;
let _15: *mut Adt38;
let _16: char;
let _17: u16;
let _18: *const Adt20;
let _19: *mut u128;
let _20: u32;
let _21: &'static mut (u64, [i128; 2]);
let _22: *const Adt19;
let _23: isize;
let _24: (&'static i32, *mut u128, i8, Adt20);
let _25: ();
let _26: ();
{
_4 = _7 - _7;
RET = 4811520112571895686_u64 * 5662870853558065438_u64;
_1 = _3;
_1 = _3;
Goto(bb1)
}
bb1 = {
_4 = _6;
_2 = core::ptr::addr_of!(_4);
(*_2) = !_7;
_5 = &mut _3;
(*_5) = [(-84697693297384224907612401450868752337_i128),(-80973151046317673421770658766145054098_i128)];
(*_5) = _1;
_1 = [(-97941775861349201593175415723122917950_i128),90329754242585412185301664303318574422_i128];
(*_2) = _7 & _7;
(*_2) = _6 | _7;
(*_5) = _1;
(*_2) = _7;
(*_5) = [138620007879969771455218819718215218798_i128,(-144490561835880211613145440765731629192_i128)];
(*_5) = [(-160831468734650467015365399001417450821_i128),(-63529683908884722634197503756393936815_i128)];
(*_2) = _7;
Call((*_2) = fn4((*_5), Move(_5), (*_5), (*_5), (*_5), (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_2) = (-18974_i16) as isize;
RET = !18069491285583241391_u64;
(*_2) = !_6;
(*_2) = _6 * _6;
(*_2) = _7 << _7;
(*_2) = !_7;
(*_2) = _7;
_8 = RET * RET;
(*_2) = _8 as isize;
_4 = (-476982356_i32) as isize;
(*_2) = _6 - _7;
_1 = [98704244838501318915756870409339713111_i128,(-7674198975488608926889470479046240628_i128)];
_8 = RET - RET;
_7 = (*_2);
(*_2) = 48474_u16 as isize;
_4 = 113_i8 as isize;
(*_2) = _6 + _7;
_13 = [9396119581908312371_usize,3246890155698141779_usize,4_usize,1438978531288690091_usize];
(*_2) = _6;
(*_2) = _7;
(*_2) = _7;
Goto(bb3)
}
bb3 = {
_1 = [(-27658877139405848104674030277336703697_i128),(-49629819183798587913527937255455687790_i128)];
_13 = [16030990234196871165_usize,7_usize,1539029556927189112_usize,4_usize];
RET = 3_usize as u64;
(*_2) = _6;
_12 = 25366_u16 as f64;
_4 = _7;
(*_2) = 168339963893372838422247915876991354157_u128 as isize;
(*_2) = -_6;
(*_2) = _6 + _7;
_4 = _7;
(*_2) = _7;
_2 = core::ptr::addr_of!((*_2));
_13 = [7634694197025387387_usize,6548907647928842570_usize,4016993028107374210_usize,1_usize];
(*_2) = _8 as isize;
Goto(bb4)
}
bb4 = {
_5 = &mut _1;
(*_5) = [(-43649954316210855475890597299752015100_i128),128703990904184990038379921689168980058_i128];
RET = _8 * _8;
Goto(bb5)
}
bb5 = {
(*_5) = [33213119083571729313082585312277043911_i128,121363454721459643013224051091675927008_i128];
_13 = [2_usize,8095120481791983558_usize,9564328076515852298_usize,4_usize];
(*_5) = [(-162593211234236086007193913863517823233_i128),(-14108803238465476768899297788527467650_i128)];
(*_2) = _6 >> RET;
(*_2) = _6 ^ _7;
(*_5) = [27239175035870102554322315014564735697_i128,144959073145358381311259298455549149881_i128];
(*_2) = 95_i8 as isize;
(*_5) = [72071964112064169753396844774209465186_i128,(-14231684079235675019217712597635828311_i128)];
(*_5) = [(-60889705674573971858787184124356394564_i128),150004253791509166526626036790250720596_i128];
(*_2) = _7;
(*_2) = -_6;
_23 = (*_2) - _6;
(*_2) = _7 * _7;
(*_2) = !_7;
(*_5) = [(-48560459499191910333133217734583883824_i128),(-71252214519335310466983450287708006226_i128)];
(*_5) = [147980756036737401060372086397797322263_i128,(-81458692275872211074103826218128393794_i128)];
_7 = (*_2) | (*_2);
RET = _8 << (*_2);
_20 = 3676687911_u32 ^ 3906960304_u32;
_18 = core::ptr::addr_of!(_24.3);
Goto(bb6)
}
bb6 = {
Call(_25 = dump_var(Move(_3), Move(_1), Move(_13), Move(_20)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [i128; 2],mut _2: &'static mut [i128; 2],mut _3: [i128; 2],mut _4: [i128; 2],mut _5: [i128; 2],mut _6: [i128; 2]) -> isize {
mir! {
type RET = isize;
let _7: *mut f32;
let _8: *mut [u128; 6];
let _9: &'static &'static i64;
let _10: *const Adt38;
let _11: [i8; 3];
let _12: isize;
let _13: *mut f32;
let _14: isize;
let _15: &'static [char; 1];
let _16: isize;
let _17: bool;
let _18: &'static mut Adt20;
let _19: &'static [i8; 3];
let _20: &'static mut Adt20;
let _21: *mut isize;
let _22: Adt76;
let _23: isize;
let _24: (*const isize,);
let _25: f32;
let _26: *mut Adt38;
let _27: i8;
let _28: char;
let _29: u64;
let _30: isize;
let _31: u32;
let _32: isize;
let _33: i16;
let _34: isize;
let _35: [isize; 1];
let _36: u32;
let _37: &'static mut (u64, [i128; 2]);
let _38: isize;
let _39: f32;
let _40: (&'static i32, *mut u128, i8, Adt20);
let _41: f64;
let _42: i128;
let _43: Adt19;
let _44: *const [i8; 3];
let _45: f64;
let _46: isize;
let _47: f32;
let _48: &'static mut (u64, [i128; 2]);
let _49: &'static (Adt19,);
let _50: [bool; 7];
let _51: [i8; 4];
let _52: isize;
let _53: char;
let _54: &'static [i8; 3];
let _55: char;
let _56: *mut [u128; 6];
let _57: char;
let _58: *mut [u128; 6];
let _59: *const Adt27;
let _60: *const *mut [u128; 6];
let _61: char;
let _62: [bool; 7];
let _63: &'static [char; 1];
let _64: *const Adt38;
let _65: &'static i32;
let _66: *const Adt19;
let _67: u64;
let _68: isize;
let _69: Adt20;
let _70: i32;
let _71: isize;
let _72: f32;
let _73: (Adt19,);
let _74: &'static mut bool;
let _75: Adt62;
let _76: f32;
let _77: isize;
let _78: f32;
let _79: (*const isize,);
let _80: &'static [char; 1];
let _81: ([u16; 4], f32);
let _82: &'static [i8; 3];
let _83: isize;
let _84: *mut f32;
let _85: ();
let _86: ();
{
_4 = [1226050909910992696533800844827721743_i128,(-34543239452048975133090622858819874831_i128)];
_5 = [69229708753030581448621629945875078274_i128,61369524772230010198697000417472634623_i128];
_4 = [115656072014446110344982903333315301829_i128,67783027871745504224280425504327125326_i128];
RET = (-95_isize) & 9223372036854775807_isize;
_3 = _6;
_3 = [(-10557473830299637970546644682984108324_i128),4872847898716532709887640047570132952_i128];
_1 = [52025858310982213162941943284752880193_i128,(-90935393470236274623206693001845845967_i128)];
_3 = [(-55509157271291262682434708375756032376_i128),101010431959121873608785327142213632631_i128];
_6 = _1;
_1 = _6;
_5 = _6;
RET = 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [62914853026882891429118029291446534007_i128,54421208516680461363935343050140028024_i128];
_5 = [(-12681413915545976550524476103734415863_i128),155066174397371602027831528368539148205_i128];
_5 = _4;
_11 = [(-35_i8),86_i8,(-17_i8)];
_1 = [(-159841353305301264804213563494606941293_i128),35389207435872955280342641644550760775_i128];
_2 = &mut _5;
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [157870328626323183600307790345497307298_i128,(-69040092062156484425152563502086130999_i128)];
_1 = [(-116132374621909431546700984211906897373_i128),108680854346069525504815486979487768161_i128];
(*_2) = _6;
(*_2) = [133961601417495471041319417208371770_i128,111378030655284127486556235383072967331_i128];
(*_2) = [(-157411651331416348805064027569206895404_i128),25486491775347124477847178626140733422_i128];
(*_2) = [(-115447452785642147781913008919863331893_i128),79061090159246745325092155016746449565_i128];
RET = 9223372036854775807_isize;
(*_2) = [106752007708591069347438841453586352769_i128,(-114544389098700606162566938076022777856_i128)];
(*_2) = [(-135425853931998733435405010688069452873_i128),79675638169115993286119476097841584128_i128];
(*_2) = [(-136846637187870706099713781843349310547_i128),(-71121879068682862554526230438450180957_i128)];
_12 = RET;
_14 = RET & RET;
(*_2) = _4;
Goto(bb3)
}
bb3 = {
(*_2) = [72624598990617977850781525728031177377_i128,124613466491793809261867436841418798544_i128];
(*_2) = [(-149299464159757732542689305647060019224_i128),(-146828529607084970091032726527754250730_i128)];
(*_2) = _6;
(*_2) = _4;
match _12 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb4 = {
_3 = [62914853026882891429118029291446534007_i128,54421208516680461363935343050140028024_i128];
_5 = [(-12681413915545976550524476103734415863_i128),155066174397371602027831528368539148205_i128];
_5 = _4;
_11 = [(-35_i8),86_i8,(-17_i8)];
_1 = [(-159841353305301264804213563494606941293_i128),35389207435872955280342641644550760775_i128];
_2 = &mut _5;
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [157870328626323183600307790345497307298_i128,(-69040092062156484425152563502086130999_i128)];
_1 = [(-116132374621909431546700984211906897373_i128),108680854346069525504815486979487768161_i128];
(*_2) = _6;
(*_2) = [133961601417495471041319417208371770_i128,111378030655284127486556235383072967331_i128];
(*_2) = [(-157411651331416348805064027569206895404_i128),25486491775347124477847178626140733422_i128];
(*_2) = [(-115447452785642147781913008919863331893_i128),79061090159246745325092155016746449565_i128];
RET = 9223372036854775807_isize;
(*_2) = [106752007708591069347438841453586352769_i128,(-114544389098700606162566938076022777856_i128)];
(*_2) = [(-135425853931998733435405010688069452873_i128),79675638169115993286119476097841584128_i128];
(*_2) = [(-136846637187870706099713781843349310547_i128),(-71121879068682862554526230438450180957_i128)];
_12 = RET;
_14 = RET & RET;
(*_2) = _4;
Goto(bb3)
}
bb5 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_6 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_2) = [22728489457630719433184746541542286051_i128,14112062877658005087780211069075838096_i128];
(*_2) = [(-21067652547752059431240144694053352146_i128),69119901137374950042332930196087881840_i128];
_3 = [(-17596668911223728716775341496867540616_i128),(-119874719340047678134220424091752866211_i128)];
_1 = (*_2);
(*_2) = [(-125171229313808345475779124561752557402_i128),88519928512023399013988510885427775753_i128];
(*_2) = [(-69798018985872996923766921540586815787_i128),(-21834403610317898616624541473873080626_i128)];
_14 = _12 & _12;
Goto(bb9)
}
bb9 = {
_17 = _14 >= RET;
_16 = RET;
(*_2) = [(-79270741840972222881963318245779302857_i128),158830164702195191317556528695900263914_i128];
(*_2) = [59790786735653919702018714789945514375_i128,125326976655038501845689211928579388888_i128];
(*_2) = [(-79754385722363799051024003532837767630_i128),64346852105886486011083151438890498006_i128];
(*_2) = _3;
(*_2) = _6;
(*_2) = [23789664694500709400929559635733628124_i128,81668624441289722587141669231339652826_i128];
(*_2) = [120071527841731872361198584736272597110_i128,(-79924583965981106370766466121096987833_i128)];
Goto(bb10)
}
bb10 = {
(*_2) = [(-96542056494238996074906028784353928808_i128),(-142186563916120448435535509104484477317_i128)];
_3 = [(-98142268109421097244028556484464617157_i128),(-20770978415795464114165495898581462701_i128)];
_3 = [92091747961857388788039685207126143117_i128,146887429180185213933599563960332967746_i128];
(*_2) = _1;
_6 = [(-166147118725964327398328137797190682752_i128),(-58234935957077670138607053868703211455_i128)];
(*_2) = [(-137653803704044165879408086437453959777_i128),11844393994694857809091992120050544372_i128];
(*_2) = [36327532834446608292364120898755144953_i128,(-116529899929226670527211279611165249581_i128)];
_4 = [69438035541437330546866901468947419962_i128,(-79820172000047887064135974283698426281_i128)];
(*_2) = [163469736476442877946986956044440044555_i128,59299773683916698570241337805336045387_i128];
_11 = [(-16_i8),(-38_i8),95_i8];
_17 = true ^ true;
_17 = _14 > _14;
_23 = !RET;
(*_2) = [(-126814341958689795338627058799937494903_i128),143811250065415349878192406508297937993_i128];
_12 = _14;
(*_2) = _1;
_1 = [31365852308680324735301513127055829516_i128,(-60343251697232768844838407916423609729_i128)];
_12 = _14 ^ _14;
_12 = _23;
match _16 {
0 => bb2,
1 => bb11,
9223372036854775807 => bb13,
_ => bb12
}
}
bb11 = {
(*_2) = [72624598990617977850781525728031177377_i128,124613466491793809261867436841418798544_i128];
(*_2) = [(-149299464159757732542689305647060019224_i128),(-146828529607084970091032726527754250730_i128)];
(*_2) = _6;
(*_2) = _4;
match _12 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb12 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
(*_2) = _6;
_4 = [(-134140034723913376717672459230979676544_i128),40687541503751053388349526103579170019_i128];
(*_2) = [(-158711666956166755267205937713134739129_i128),88154883115296609387693494994703589183_i128];
_21 = core::ptr::addr_of_mut!(_23);
_2 = &mut _6;
(*_21) = _16 * _14;
(*_2) = _1;
(*_2) = _3;
(*_2) = [84973048462751346082692479348279081448_i128,(-54037273507029820353003275951521502855_i128)];
(*_21) = !_16;
(*_21) = _16 | _14;
(*_2) = _4;
_23 = _16 | _16;
(*_21) = _12 << _14;
_7 = core::ptr::addr_of_mut!(_25);
_19 = &_11;
(*_7) = 2260093224_u32 as f32;
(*_7) = 35076_u16 as f32;
Call(_3 = core::intrinsics::transmute((*_2)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_28 = '\u{fa7d3}';
_13 = core::ptr::addr_of_mut!((*_7));
_23 = RET;
_11 = [(-19_i8),(-35_i8),(-107_i8)];
_2 = &mut _4;
(*_2) = [(-50569722623458041366565388988879844447_i128),(-13613754364536445958238292020058378351_i128)];
_30 = 5_i8 as isize;
_33 = !(-16899_i16);
Goto(bb15)
}
bb15 = {
(*_7) = 328719854923513892800998964860759494928_u128 as f32;
(*_2) = _3;
(*_21) = RET << _30;
_29 = 514859464621522971_u64 >> (*_21);
_34 = !_14;
(*_7) = 2539602224795909847_usize as f32;
_24.0 = core::ptr::addr_of!((*_21));
(*_7) = (-60334114028249162886474512039943365382_i128) as f32;
(*_21) = _12 * _30;
(*_21) = !RET;
_21 = core::ptr::addr_of_mut!((*_21));
(*_21) = _34 >> _14;
(*_2) = [(-74710589097650180968826619992704387741_i128),(-150981006451022935807023798814866900647_i128)];
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_21) = _12 ^ _34;
(*_2) = [(-55329840680909298355172284914401189678_i128),(-6370098222051776927488407268447841053_i128)];
(*_7) = (-77_i8) as f32;
(*_7) = (-996413930_i32) as f32;
_1 = (*_2);
(*_2) = [(-111193967825859971522592613369427356427_i128),(-2808816754131224233469133051104155242_i128)];
_34 = (*_21) - (*_21);
_38 = RET & (*_21);
Call((*_21) = core::intrinsics::bswap(_34), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*_7) = 35846_u16 as f32;
_11 = [(-102_i8),(-96_i8),12_i8];
(*_7) = (-103_i8) as f32;
(*_2) = _3;
(*_2) = [24005642044749032312857167751725736353_i128,39345525712348394126903139809276104377_i128];
_7 = core::ptr::addr_of_mut!((*_7));
(*_2) = [(-123815185130470367033271528118416892606_i128),71454913028297026856972437632569237211_i128];
(*_21) = -_34;
_33 = _29 as i16;
_28 = '\u{c6345}';
_12 = (*_21) ^ _34;
(*_21) = -_12;
(*_21) = RET;
(*_21) = (*_7) as isize;
(*_2) = [58847581235256152389329899499438404483_i128,80020980343925949683162358574547879551_i128];
_29 = 14870451629070001604_u64 & 10412768271326118553_u64;
_14 = 16387_u16 as isize;
_30 = _28 as isize;
(*_21) = _17 as isize;
_30 = (*_21);
(*_2) = _1;
_17 = true | true;
(*_7) = 99_u8 as f32;
(*_21) = _12;
match _16 {
0 => bb11,
1 => bb17,
2 => bb18,
9223372036854775807 => bb20,
_ => bb19
}
}
bb17 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_6 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_2) = [22728489457630719433184746541542286051_i128,14112062877658005087780211069075838096_i128];
(*_2) = [(-21067652547752059431240144694053352146_i128),69119901137374950042332930196087881840_i128];
_3 = [(-17596668911223728716775341496867540616_i128),(-119874719340047678134220424091752866211_i128)];
_1 = (*_2);
(*_2) = [(-125171229313808345475779124561752557402_i128),88519928512023399013988510885427775753_i128];
(*_2) = [(-69798018985872996923766921540586815787_i128),(-21834403610317898616624541473873080626_i128)];
_14 = _12 & _12;
Goto(bb9)
}
bb19 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
(*_21) = _38;
_43.fld2 = 184_u8 | 224_u8;
(*_21) = 6_usize as isize;
_27 = !85_i8;
(*_7) = 4_usize as f32;
_43.fld4 = _33;
(*_2) = _3;
(*_2) = [9064665479678194727277425819565926637_i128,87260520372008658767156072226689669669_i128];
(*_2) = _3;
(*_21) = _38 ^ _34;
(*_21) = _12 ^ _38;
match _16 {
0 => bb17,
1 => bb19,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb21,
6 => bb22,
9223372036854775807 => bb24,
_ => bb23
}
}
bb21 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb22 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb23 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb24 = {
(*_2) = _1;
_40.2 = 44478172984546242080160015348655323999_i128 as i8;
(*_2) = _1;
_43 = Adt19 { fld0: (*_2),fld1: 256615564939763192869136142135057489691_u128,fld2: 7_u8,fld3: 47488_u16,fld4: _33,fld5: (-57253595715029977409339208563932345565_i128) };
_31 = 165049284_u32 ^ 179398458_u32;
_35 = [(*_21)];
_21 = core::ptr::addr_of_mut!((*_21));
_33 = !_43.fld4;
_43 = Adt19 { fld0: (*_2),fld1: 15227528637921385847525527353879808815_u128,fld2: 164_u8,fld3: 26785_u16,fld4: _33,fld5: (-28722580621832438175090707140707396977_i128) };
(*_7) = 7326698402382197860_usize as f32;
(*_2) = [_43.fld5,_43.fld5];
(*_21) = RET + _12;
(*_21) = _34 + _34;
_40.1 = core::ptr::addr_of_mut!(_43.fld1);
(*_21) = _12 << _12;
(*_7) = _43.fld2 as f32;
(*_7) = _40.2 as f32;
match _43.fld5 {
0 => bb14,
1 => bb2,
2 => bb11,
3 => bb25,
4 => bb26,
5 => bb27,
311559786299106025288283900291060814479 => bb29,
_ => bb28
}
}
bb25 = {
Return()
}
bb26 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb27 = {
(*_7) = 328719854923513892800998964860759494928_u128 as f32;
(*_2) = _3;
(*_21) = RET << _30;
_29 = 514859464621522971_u64 >> (*_21);
_34 = !_14;
(*_7) = 2539602224795909847_usize as f32;
_24.0 = core::ptr::addr_of!((*_21));
(*_7) = (-60334114028249162886474512039943365382_i128) as f32;
(*_21) = _12 * _30;
(*_21) = !RET;
_21 = core::ptr::addr_of_mut!((*_21));
(*_21) = _34 >> _14;
(*_2) = [(-74710589097650180968826619992704387741_i128),(-150981006451022935807023798814866900647_i128)];
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_21) = _12 ^ _34;
(*_2) = [(-55329840680909298355172284914401189678_i128),(-6370098222051776927488407268447841053_i128)];
(*_7) = (-77_i8) as f32;
(*_7) = (-996413930_i32) as f32;
_1 = (*_2);
(*_2) = [(-111193967825859971522592613369427356427_i128),(-2808816754131224233469133051104155242_i128)];
_34 = (*_21) - (*_21);
_38 = RET & (*_21);
Call((*_21) = core::intrinsics::bswap(_34), ReturnTo(bb16), UnwindUnreachable())
}
bb28 = {
_3 = [62914853026882891429118029291446534007_i128,54421208516680461363935343050140028024_i128];
_5 = [(-12681413915545976550524476103734415863_i128),155066174397371602027831528368539148205_i128];
_5 = _4;
_11 = [(-35_i8),86_i8,(-17_i8)];
_1 = [(-159841353305301264804213563494606941293_i128),35389207435872955280342641644550760775_i128];
_2 = &mut _5;
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [157870328626323183600307790345497307298_i128,(-69040092062156484425152563502086130999_i128)];
_1 = [(-116132374621909431546700984211906897373_i128),108680854346069525504815486979487768161_i128];
(*_2) = _6;
(*_2) = [133961601417495471041319417208371770_i128,111378030655284127486556235383072967331_i128];
(*_2) = [(-157411651331416348805064027569206895404_i128),25486491775347124477847178626140733422_i128];
(*_2) = [(-115447452785642147781913008919863331893_i128),79061090159246745325092155016746449565_i128];
RET = 9223372036854775807_isize;
(*_2) = [106752007708591069347438841453586352769_i128,(-114544389098700606162566938076022777856_i128)];
(*_2) = [(-135425853931998733435405010688069452873_i128),79675638169115993286119476097841584128_i128];
(*_2) = [(-136846637187870706099713781843349310547_i128),(-71121879068682862554526230438450180957_i128)];
_12 = RET;
_14 = RET & RET;
(*_2) = _4;
Goto(bb3)
}
bb29 = {
(*_2) = [_43.fld5,_43.fld5];
(*_21) = _34 >> _43.fld5;
(*_21) = _12 - _12;
(*_7) = (-5848334254607709230_i64) as f32;
_25 = _43.fld2 as f32;
Goto(bb30)
}
bb30 = {
(*_21) = !_34;
(*_21) = _12;
(*_21) = (-2084007289_i32) as isize;
_32 = _12 >> _34;
(*_21) = _38;
_27 = -_40.2;
_33 = _43.fld4;
(*_2) = [_43.fld5,_43.fld5];
(*_21) = _14;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = _43.fld3 as f32;
(*_7) = 5714855451594396488_usize as f32;
_35 = [_30];
(*_21) = RET;
(*_7) = _31 as f32;
(*_2) = _1;
Call(_43.fld3 = core::intrinsics::transmute(_43.fld4), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_40.3 = Adt20::Variant2 { fld0: (-1551308316_i32) };
_12 = (*_21) | (*_21);
_32 = _43.fld1 as isize;
(*_7) = _43.fld3 as f32;
_40.3 = Adt20::Variant1 { fld0: _43.fld4 };
_42 = _43.fld5;
(*_21) = _32;
(*_2) = _3;
(*_21) = _12 << _12;
_14 = (*_21) * (*_21);
(*_7) = (-8491287807446452777_i64) as f32;
(*_21) = _38;
(*_7) = _43.fld4 as f32;
_40.2 = _27;
_35 = [(*_21)];
(*_7) = (-24058797_i32) as f32;
_23 = 2477141938806302185_i64 as isize;
(*_7) = _42 as f32;
_45 = _27 as f64;
_43.fld0 = [_42,_42];
(*_7) = 5824186384847466683_i64 as f32;
Call((*_2) = core::intrinsics::transmute(_3), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
(*_7) = 6_usize as f32;
_3 = [_43.fld5,_43.fld5];
_46 = -_14;
(*_2) = [_43.fld5,_42];
_46 = (*_21) * (*_21);
(*_21) = _14 * _12;
(*_2) = [_42,_42];
place!(Field::<i16>(Variant(_40.3, 1), 0)) = _43.fld4;
_25 = 6_usize as f32;
_25 = _43.fld3 as f32;
(*_2) = [_43.fld5,_43.fld5];
(*_2) = [_43.fld5,_43.fld5];
(*_21) = _14 << _43.fld1;
(*_7) = _45 as f32;
(*_21) = _12 & _32;
_45 = _42 as f64;
_20 = &mut _40.3;
(*_21) = _14 ^ _32;
(*_2) = [_42,_42];
(*_21) = _34;
_2 = &mut _1;
Goto(bb33)
}
bb33 = {
_27 = (-126_i8);
_44 = core::ptr::addr_of!(_11);
(*_44) = [_27,_27,_27];
(*_2) = [_42,_43.fld5];
(*_44) = [_27,_27,_27];
place!(Field::<i16>(Variant((*_20), 1), 0)) = _43.fld4 + _43.fld4;
(*_2) = [_43.fld5,_43.fld5];
_36 = _31 | _31;
place!(Field::<i16>(Variant((*_20), 1), 0)) = _33;
(*_21) = _34 << _33;
(*_21) = _34 ^ _38;
place!(Field::<i16>(Variant((*_20), 1), 0)) = !_43.fld4;
(*_20) = Adt20::Variant1 { fld0: _33 };
(*_44) = [_27,_27,_27];
_57 = _28;
(*_21) = _34 << Field::<i16>(Variant((*_20), 1), 0);
(*_44) = [_27,_27,_27];
place!(Field::<i16>(Variant((*_20), 1), 0)) = (-107561647690996131_i64) as i16;
(*_2) = [_42,_42];
(*_2) = _3;
(*_2) = [_42,_42];
(*_7) = _27 as f32;
(*_44) = [_27,_27,_27];
Goto(bb34)
}
bb34 = {
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_21) = _14 | _34;
_24.0 = core::ptr::addr_of!(_12);
_23 = 6_usize as isize;
(*_44) = [_27,_27,_27];
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
_24.0 = core::ptr::addr_of!((*_21));
(*_7) = _31 as f32;
(*_7) = _45 as f32;
(*_7) = _43.fld1 as f32;
(*_7) = _42 as f32;
(*_7) = _43.fld2 as f32;
_62 = [_17,_17,_17,_17,_17,_17,_17];
(*_20) = Adt20::Variant0 { fld0: _42,fld1: _28,fld2: _43.fld2,fld3: _29,fld4: (-2012104224_i32) };
(*_44) = [_27,_27,_27];
place!(Field::<u8>(Variant((*_20), 0), 2)) = _43.fld2;
place!(Field::<i32>(Variant((*_20), 0), 4)) = -(-1496951062_i32);
_27 = (-15_i8) ^ 100_i8;
(*_21) = _30 << Field::<i128>(Variant((*_20), 0), 0);
(*_44) = [_27,_27,_27];
match Field::<u8>(Variant((*_20), 0), 2) {
0 => bb35,
164 => bb37,
_ => bb36
}
}
bb35 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb36 = {
_6 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_2) = [22728489457630719433184746541542286051_i128,14112062877658005087780211069075838096_i128];
(*_2) = [(-21067652547752059431240144694053352146_i128),69119901137374950042332930196087881840_i128];
_3 = [(-17596668911223728716775341496867540616_i128),(-119874719340047678134220424091752866211_i128)];
_1 = (*_2);
(*_2) = [(-125171229313808345475779124561752557402_i128),88519928512023399013988510885427775753_i128];
(*_2) = [(-69798018985872996923766921540586815787_i128),(-21834403610317898616624541473873080626_i128)];
_14 = _12 & _12;
Goto(bb9)
}
bb37 = {
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_2) = _43.fld0;
_44 = core::ptr::addr_of!((*_44));
(*_2) = [Field::<i128>(Variant((*_20), 0), 0),Field::<i128>(Variant((*_20), 0), 0)];
match Field::<u8>(Variant((*_20), 0), 2) {
0 => bb16,
1 => bb8,
2 => bb20,
3 => bb38,
164 => bb40,
_ => bb39
}
}
bb38 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb39 = {
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_21) = _14 | _34;
_24.0 = core::ptr::addr_of!(_12);
_23 = 6_usize as isize;
(*_44) = [_27,_27,_27];
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
_24.0 = core::ptr::addr_of!((*_21));
(*_7) = _31 as f32;
(*_7) = _45 as f32;
(*_7) = _43.fld1 as f32;
(*_7) = _42 as f32;
(*_7) = _43.fld2 as f32;
_62 = [_17,_17,_17,_17,_17,_17,_17];
(*_20) = Adt20::Variant0 { fld0: _42,fld1: _28,fld2: _43.fld2,fld3: _29,fld4: (-2012104224_i32) };
(*_44) = [_27,_27,_27];
place!(Field::<u8>(Variant((*_20), 0), 2)) = _43.fld2;
place!(Field::<i32>(Variant((*_20), 0), 4)) = -(-1496951062_i32);
_27 = (-15_i8) ^ 100_i8;
(*_21) = _30 << Field::<i128>(Variant((*_20), 0), 0);
(*_44) = [_27,_27,_27];
match Field::<u8>(Variant((*_20), 0), 2) {
0 => bb35,
164 => bb37,
_ => bb36
}
}
bb40 = {
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_7) = 7_usize as f32;
_60 = core::ptr::addr_of!(_8);
place!(Field::<i128>(Variant((*_20), 0), 0)) = Field::<u8>(Variant((*_20), 0), 2) as i128;
(*_20) = Adt20::Variant2 { fld0: (-1521170725_i32) };
_61 = _57;
(*_44) = [_27,_27,_27];
match _43.fld1 {
0 => bb33,
1 => bb28,
15227528637921385847525527353879808815 => bb41,
_ => bb10
}
}
bb41 = {
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
_25 = 4263973105232613587_i64 as f32;
(*_20) = Adt20::Variant1 { fld0: _33 };
match _42 {
0 => bb3,
1 => bb42,
2 => bb43,
311559786299106025288283900291060814479 => bb45,
_ => bb44
}
}
bb42 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb43 = {
(*_2) = [_43.fld5,_43.fld5];
(*_21) = _34 >> _43.fld5;
(*_21) = _12 - _12;
(*_7) = (-5848334254607709230_i64) as f32;
_25 = _43.fld2 as f32;
Goto(bb30)
}
bb44 = {
(*_2) = _6;
_4 = [(-134140034723913376717672459230979676544_i128),40687541503751053388349526103579170019_i128];
(*_2) = [(-158711666956166755267205937713134739129_i128),88154883115296609387693494994703589183_i128];
_21 = core::ptr::addr_of_mut!(_23);
_2 = &mut _6;
(*_21) = _16 * _14;
(*_2) = _1;
(*_2) = _3;
(*_2) = [84973048462751346082692479348279081448_i128,(-54037273507029820353003275951521502855_i128)];
(*_21) = !_16;
(*_21) = _16 | _14;
(*_2) = _4;
_23 = _16 | _16;
(*_21) = _12 << _14;
_7 = core::ptr::addr_of_mut!(_25);
_19 = &_11;
(*_7) = 2260093224_u32 as f32;
(*_7) = 35076_u16 as f32;
Call(_3 = core::intrinsics::transmute((*_2)), ReturnTo(bb14), UnwindUnreachable())
}
bb45 = {
(*_20) = Adt20::Variant0 { fld0: _43.fld5,fld1: _61,fld2: _43.fld2,fld3: _29,fld4: 706829277_i32 };
(*_21) = _14;
place!(Field::<i128>(Variant((*_20), 0), 0)) = (-5644322451841924124_i64) as i128;
(*_7) = 0_usize as f32;
_46 = -(*_21);
match Field::<u8>(Variant((*_20), 0), 2) {
0 => bb37,
164 => bb46,
_ => bb25
}
}
bb46 = {
place!(Field::<i32>(Variant((*_20), 0), 4)) = 1043007335_i32 >> Field::<i128>(Variant((*_20), 0), 0);
place!(Field::<i32>(Variant((*_20), 0), 4)) = (-863882254_i32) & (-802200375_i32);
(*_20) = Adt20::Variant1 { fld0: _33 };
(*_21) = -_38;
_55 = _61;
match _43.fld1 {
0 => bb44,
1 => bb17,
2 => bb47,
3 => bb48,
4 => bb49,
5 => bb50,
6 => bb51,
15227528637921385847525527353879808815 => bb53,
_ => bb52
}
}
bb47 = {
_28 = '\u{fa7d3}';
_13 = core::ptr::addr_of_mut!((*_7));
_23 = RET;
_11 = [(-19_i8),(-35_i8),(-107_i8)];
_2 = &mut _4;
(*_2) = [(-50569722623458041366565388988879844447_i128),(-13613754364536445958238292020058378351_i128)];
_30 = 5_i8 as isize;
_33 = !(-16899_i16);
Goto(bb15)
}
bb48 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb49 = {
_6 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_2) = [22728489457630719433184746541542286051_i128,14112062877658005087780211069075838096_i128];
(*_2) = [(-21067652547752059431240144694053352146_i128),69119901137374950042332930196087881840_i128];
_3 = [(-17596668911223728716775341496867540616_i128),(-119874719340047678134220424091752866211_i128)];
_1 = (*_2);
(*_2) = [(-125171229313808345475779124561752557402_i128),88519928512023399013988510885427775753_i128];
(*_2) = [(-69798018985872996923766921540586815787_i128),(-21834403610317898616624541473873080626_i128)];
_14 = _12 & _12;
Goto(bb9)
}
bb50 = {
_3 = [(-105660863322085541884487180637141232910_i128),58798980437488200688658257087768871250_i128];
Call(_5 = fn5(_3, RET, RET, _3, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb51 = {
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_44) = [_27,_27,_27];
(*_21) = _14 | _34;
_24.0 = core::ptr::addr_of!(_12);
_23 = 6_usize as isize;
(*_44) = [_27,_27,_27];
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
_24.0 = core::ptr::addr_of!((*_21));
(*_7) = _31 as f32;
(*_7) = _45 as f32;
(*_7) = _43.fld1 as f32;
(*_7) = _42 as f32;
(*_7) = _43.fld2 as f32;
_62 = [_17,_17,_17,_17,_17,_17,_17];
(*_20) = Adt20::Variant0 { fld0: _42,fld1: _28,fld2: _43.fld2,fld3: _29,fld4: (-2012104224_i32) };
(*_44) = [_27,_27,_27];
place!(Field::<u8>(Variant((*_20), 0), 2)) = _43.fld2;
place!(Field::<i32>(Variant((*_20), 0), 4)) = -(-1496951062_i32);
_27 = (-15_i8) ^ 100_i8;
(*_21) = _30 << Field::<i128>(Variant((*_20), 0), 0);
(*_44) = [_27,_27,_27];
match Field::<u8>(Variant((*_20), 0), 2) {
0 => bb35,
164 => bb37,
_ => bb36
}
}
bb52 = {
_6 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
(*_2) = [22728489457630719433184746541542286051_i128,14112062877658005087780211069075838096_i128];
(*_2) = [(-21067652547752059431240144694053352146_i128),69119901137374950042332930196087881840_i128];
_3 = [(-17596668911223728716775341496867540616_i128),(-119874719340047678134220424091752866211_i128)];
_1 = (*_2);
(*_2) = [(-125171229313808345475779124561752557402_i128),88519928512023399013988510885427775753_i128];
(*_2) = [(-69798018985872996923766921540586815787_i128),(-21834403610317898616624541473873080626_i128)];
_14 = _12 & _12;
Goto(bb9)
}
bb53 = {
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
(*_20) = Adt20::Variant0 { fld0: _42,fld1: _55,fld2: _43.fld2,fld3: _29,fld4: 1342252796_i32 };
_61 = Field::<char>(Variant((*_20), 0), 1);
(*_2) = [Field::<i128>(Variant((*_20), 0), 0),Field::<i128>(Variant((*_20), 0), 0)];
(*_21) = -_32;
_43 = Adt19 { fld0: (*_2),fld1: 183143883205396500131852990061289263825_u128,fld2: Field::<u8>(Variant((*_20), 0), 2),fld3: 16532_u16,fld4: _33,fld5: Field::<i128>(Variant((*_20), 0), 0) };
place!(Field::<i128>(Variant((*_20), 0), 0)) = _42 >> (*_21);
place!(Field::<i128>(Variant((*_20), 0), 0)) = -_42;
place!(Field::<i32>(Variant((*_20), 0), 4)) = !1211388417_i32;
_73.0.fld0 = (*_2);
(*_20) = Adt20::Variant0 { fld0: _43.fld5,fld1: _28,fld2: _43.fld2,fld3: _29,fld4: 172612559_i32 };
_73.0.fld3 = !_43.fld3;
(*_20) = Adt20::Variant1 { fld0: _43.fld4 };
(*_44) = [_27,_27,_27];
(*_21) = -_46;
(*_7) = Field::<i16>(Variant((*_20), 1), 0) as f32;
_78 = (*_7) + (*_7);
(*_44) = [_27,_27,_27];
_32 = _27 as isize;
_53 = _61;
(*_2) = _3;
_17 = (*_21) >= (*_21);
Goto(bb54)
}
bb54 = {
Call(_85 = dump_var(Move(_12), Move(_5), Move(_3), Move(_16)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_85 = dump_var(Move(_62), Move(_6), Move(_61), Move(_17)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_85 = dump_var(Move(_11), Move(_42), Move(_34), Move(_55)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_85 = dump_var(Move(_28), Move(_31), _86, _86), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i128; 2],mut _2: isize,mut _3: isize,mut _4: [i128; 2],mut _5: [i128; 2],mut _6: [i128; 2]) -> [i128; 2] {
mir! {
type RET = [i128; 2];
let _7: Adt38;
let _8: *mut u128;
let _9: (u64, [i128; 2]);
let _10: [bool; 7];
let _11: isize;
let _12: &'static [u16; 4];
let _13: i8;
let _14: &'static [i8; 3];
let _15: f64;
let _16: &'static mut &'static u32;
let _17: (*const isize,);
let _18: [usize; 6];
let _19: i32;
let _20: *mut Adt38;
let _21: char;
let _22: isize;
let _23: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _24: &'static mut bool;
let _25: &'static mut &'static mut u8;
let _26: [isize; 1];
let _27: u16;
let _28: Adt76;
let _29: &'static mut &'static u32;
let _30: (&'static [i8; 3],);
let _31: char;
let _32: u16;
let _33: usize;
let _34: (u64, [i128; 2]);
let _35: &'static mut [i128; 2];
let _36: isize;
let _37: bool;
let _38: *const Adt20;
let _39: &'static &'static mut &'static u32;
let _40: &'static mut u8;
let _41: u128;
let _42: u128;
let _43: *const Adt19;
let _44: i128;
let _45: f32;
let _46: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _47: char;
let _48: [u128; 6];
let _49: usize;
let _50: &'static [char; 1];
let _51: u16;
let _52: [i32; 3];
let _53: &'static mut [u128; 6];
let _54: f32;
let _55: &'static bool;
let _56: usize;
let _57: &'static &'static i64;
let _58: &'static mut &'static u32;
let _59: [usize; 4];
let _60: f64;
let _61: &'static mut [u128; 6];
let _62: [i8; 3];
let _63: &'static &'static Adt27;
let _64: &'static (Adt19,);
let _65: usize;
let _66: [i8; 8];
let _67: isize;
let _68: [u8; 7];
let _69: bool;
let _70: f32;
let _71: usize;
let _72: &'static &'static Adt27;
let _73: [u8; 7];
let _74: &'static mut (u64, [i128; 2]);
let _75: u64;
let _76: (*const isize,);
let _77: (u64, [i128; 2]);
let _78: i64;
let _79: &'static mut [u128; 6];
let _80: isize;
let _81: &'static [u16; 4];
let _82: &'static mut bool;
let _83: [i8; 3];
let _84: [usize; 6];
let _85: &'static bool;
let _86: u128;
let _87: &'static &'static Adt27;
let _88: &'static &'static i64;
let _89: i64;
let _90: char;
let _91: ();
let _92: ();
{
_4 = [102244275919211634885482626490722823179_i128,52154345173166397269639534150715062513_i128];
_6 = _1;
_1 = [139003955015481668936839615435095697414_i128,40348846806326069313614036235048545094_i128];
_6 = _5;
_6 = [162292288860670600603093369434178262439_i128,(-112854575065361743971531277925370933019_i128)];
Goto(bb1)
}
bb1 = {
RET = [(-89749296031460096818839955377267979292_i128),(-110762525592286670999991255583943778248_i128)];
_4 = [133922258124468765373163068454263145053_i128,162711844677506369997904583637102222952_i128];
_1 = _6;
_9 = (18289721086953900278_u64, _6);
_3 = _2 ^ _2;
_2 = 201783499686822489469426922908145941596_u128 as isize;
_9.0 = 7427582791430215642_u64;
_5 = [(-121372160937871814375354457392936271516_i128),(-84990754767240255501398521695525548007_i128)];
_3 = _2 >> _9.0;
_9 = (2901003408484776068_u64, _6);
_6 = [2544697446778011693574597300033645486_i128,152184472602298486485195552114656240084_i128];
_9.0 = !16093843330608790565_u64;
_6 = _9.1;
_11 = _3 * _3;
_10 = [true,true,true,false,true,false,true];
_9.0 = true as u64;
_13 = 46_i8;
Call(_3 = fn6(_9.0, _13, _2, _9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.0 = !5643455496504559506_u64;
match _13 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
46 => bb9,
_ => bb8
}
}
bb3 = {
RET = [(-89749296031460096818839955377267979292_i128),(-110762525592286670999991255583943778248_i128)];
_4 = [133922258124468765373163068454263145053_i128,162711844677506369997904583637102222952_i128];
_1 = _6;
_9 = (18289721086953900278_u64, _6);
_3 = _2 ^ _2;
_2 = 201783499686822489469426922908145941596_u128 as isize;
_9.0 = 7427582791430215642_u64;
_5 = [(-121372160937871814375354457392936271516_i128),(-84990754767240255501398521695525548007_i128)];
_3 = _2 >> _9.0;
_9 = (2901003408484776068_u64, _6);
_6 = [2544697446778011693574597300033645486_i128,152184472602298486485195552114656240084_i128];
_9.0 = !16093843330608790565_u64;
_6 = _9.1;
_11 = _3 * _3;
_10 = [true,true,true,false,true,false,true];
_9.0 = true as u64;
_13 = 46_i8;
Call(_3 = fn6(_9.0, _13, _2, _9.0), ReturnTo(bb2), UnwindUnreachable())
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
RET = [(-150919980193866988939963912335923966461_i128),(-127464699785241855001815237257933424944_i128)];
_6 = [(-81449980900861364870554184269808686801_i128),131794019508923105894637651320367635582_i128];
_9 = (17692488768204440147_u64, _4);
_5 = RET;
_15 = 215678882848919936066256804621353710148_u128 as f64;
_13 = 271134777573214373945052203079024426658_u128 as i8;
_17.0 = core::ptr::addr_of!(_11);
_4 = [(-31191176358302744560553362951641040218_i128),25322123121263115745554819793856141791_i128];
_13 = 53672_u16 as i8;
_17.0 = core::ptr::addr_of!(_3);
_10 = [false,false,true,true,true,true,false];
_2 = (-17243525229526795420109118855412530080_i128) as isize;
_13 = -(-4_i8);
_18 = [1590847972787634403_usize,3_usize,14069718407945856281_usize,16420258910297007139_usize,18415264641323769724_usize,2_usize];
_18 = [6_usize,5752273257009781123_usize,3_usize,1_usize,14655034246067103359_usize,16865850160589216076_usize];
_1 = [19665593689736538143738570120907514435_i128,(-157866298815450038265027630783671680092_i128)];
_6 = [(-16589330082453300665414861244783893589_i128),(-94197114695706170684628097503428320057_i128)];
_6 = _9.1;
match _9.0 {
0 => bb7,
1 => bb5,
17692488768204440147 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
RET = _6;
_11 = _13 as isize;
_10 = [true,false,false,false,false,false,true];
_5 = _1;
_18 = [3_usize,7_usize,13226307814637282256_usize,2_usize,3_usize,2_usize];
_20 = core::ptr::addr_of_mut!(_7);
_13 = (-102_i8) >> _2;
_23.2 = -_15;
_3 = 136_u8 as isize;
_22 = 216918696164384125384730834392729627772_u128 as isize;
_19 = 7135716584961253227235616730698166267_u128 as i32;
_18 = [7_usize,6_usize,16132162040753918901_usize,4_usize,735286130205085887_usize,7_usize];
_19 = 1647707341_i32;
_9.1 = [(-139245422068829331473321963383093754631_i128),145704185312634177225722754517790114476_i128];
_15 = _23.2 + _23.2;
_11 = !_3;
_2 = _22 >> _22;
_9.0 = 88937092648520213118296350282917511342_i128 as u64;
_2 = _3;
Goto(bb12)
}
bb12 = {
_5 = [(-95156063829849014244741600560727343615_i128),78043356377270938796092577449093864657_i128];
RET = _9.1;
_17.0 = core::ptr::addr_of!(_2);
_21 = '\u{65202}';
_3 = _11 - _11;
_3 = _2 | _22;
_10 = [false,false,false,false,true,true,false];
_2 = _23.2 as isize;
RET = [(-134093772423014550981828493527940822743_i128),(-20983298506863449035600003474799926732_i128)];
_22 = _11;
_23.2 = _15 + _15;
_26 = [_3];
_15 = _23.2 - _23.2;
_1 = [(-22864518391513686429734013996491837777_i128),29223392563399289569745926053298159953_i128];
_17.0 = core::ptr::addr_of!(_11);
_20 = core::ptr::addr_of_mut!((*_20));
_9.1 = _5;
_27 = false as u16;
_15 = _23.2;
match _19 {
0 => bb1,
1 => bb13,
2 => bb14,
1647707341 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_13 = _27 as i8;
_26 = [_3];
_19 = 1399579662_i32;
_9.0 = 17372135689012434277_u64 & 13518405536863784391_u64;
_11 = _22 ^ _3;
_3 = false as isize;
_26 = [_2];
_23.2 = _15 * _15;
_23.2 = _15 * _15;
_5 = _9.1;
_34.1 = _9.1;
_17.0 = core::ptr::addr_of!(_2);
_32 = _27 + _27;
_9.1 = [(-160592405792197323895375573991054665342_i128),105617893519123763727598562030880993654_i128];
_17.0 = core::ptr::addr_of!(_22);
_34.0 = 105792790574955022106187996367443256780_u128 as u64;
_36 = !_2;
_26 = [_2];
_18 = [6_usize,2532602161833581846_usize,0_usize,1881599678794845839_usize,4591860706673931735_usize,0_usize];
_13 = (-12_i8) ^ (-44_i8);
_9.1 = [41626789990153858356010132904645693631_i128,128302880576469893546107483193169189130_i128];
match _19 {
0 => bb15,
1 => bb10,
2 => bb17,
1399579662 => bb19,
_ => bb18
}
}
bb17 = {
_9.0 = !5643455496504559506_u64;
match _13 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
46 => bb9,
_ => bb8
}
}
bb18 = {
Return()
}
bb19 = {
_20 = core::ptr::addr_of_mut!((*_20));
_10 = [false,false,true,false,false,false,false];
_18 = [16652584932589305036_usize,4_usize,16347087344578910792_usize,1958725382512208843_usize,16378161681816380175_usize,11487395066994786360_usize];
_27 = !_32;
_35 = &mut _5;
_32 = _27 >> _3;
_37 = true | true;
_26 = [_11];
_24 = &mut _37;
(*_35) = [103068284647019521235369406820631667088_i128,(-18853326847418855536691540559959154932_i128)];
match _19 {
0 => bb20,
1399579662 => bb22,
_ => bb21
}
}
bb20 = {
RET = [(-89749296031460096818839955377267979292_i128),(-110762525592286670999991255583943778248_i128)];
_4 = [133922258124468765373163068454263145053_i128,162711844677506369997904583637102222952_i128];
_1 = _6;
_9 = (18289721086953900278_u64, _6);
_3 = _2 ^ _2;
_2 = 201783499686822489469426922908145941596_u128 as isize;
_9.0 = 7427582791430215642_u64;
_5 = [(-121372160937871814375354457392936271516_i128),(-84990754767240255501398521695525548007_i128)];
_3 = _2 >> _9.0;
_9 = (2901003408484776068_u64, _6);
_6 = [2544697446778011693574597300033645486_i128,152184472602298486485195552114656240084_i128];
_9.0 = !16093843330608790565_u64;
_6 = _9.1;
_11 = _3 * _3;
_10 = [true,true,true,false,true,false,true];
_9.0 = true as u64;
_13 = 46_i8;
Call(_3 = fn6(_9.0, _13, _2, _9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb21 = {
RET = [(-150919980193866988939963912335923966461_i128),(-127464699785241855001815237257933424944_i128)];
_6 = [(-81449980900861364870554184269808686801_i128),131794019508923105894637651320367635582_i128];
_9 = (17692488768204440147_u64, _4);
_5 = RET;
_15 = 215678882848919936066256804621353710148_u128 as f64;
_13 = 271134777573214373945052203079024426658_u128 as i8;
_17.0 = core::ptr::addr_of!(_11);
_4 = [(-31191176358302744560553362951641040218_i128),25322123121263115745554819793856141791_i128];
_13 = 53672_u16 as i8;
_17.0 = core::ptr::addr_of!(_3);
_10 = [false,false,true,true,true,true,false];
_2 = (-17243525229526795420109118855412530080_i128) as isize;
_13 = -(-4_i8);
_18 = [1590847972787634403_usize,3_usize,14069718407945856281_usize,16420258910297007139_usize,18415264641323769724_usize,2_usize];
_18 = [6_usize,5752273257009781123_usize,3_usize,1_usize,14655034246067103359_usize,16865850160589216076_usize];
_1 = [19665593689736538143738570120907514435_i128,(-157866298815450038265027630783671680092_i128)];
_6 = [(-16589330082453300665414861244783893589_i128),(-94197114695706170684628097503428320057_i128)];
_6 = _9.1;
match _9.0 {
0 => bb7,
1 => bb5,
17692488768204440147 => bb11,
_ => bb10
}
}
bb22 = {
_17.0 = core::ptr::addr_of!(_22);
_23.2 = _15 * _15;
_9.0 = !_34.0;
_35 = &mut _6;
RET = (*_35);
_34.0 = _9.0 - _9.0;
_34.1 = [102147650272572839958172061898889327962_i128,(-149304088132105204617272019838205596519_i128)];
_11 = _22 + _36;
_31 = _21;
(*_35) = [(-3681908983890903892870585177113865177_i128),13447634892631952254849234937595602931_i128];
_3 = _11 << _32;
_36 = _3 | _22;
(*_35) = [(-163535540795813250258137241496578992957_i128),132291323109778250666420380017268146026_i128];
(*_35) = [(-84295569751881464118736434509712082993_i128),81707423632115677416852739469464104392_i128];
_35 = &mut _9.1;
(*_35) = [(-111533052532196865022347462608484303717_i128),128159527193117613252718588662275791848_i128];
(*_24) = false;
(*_35) = _4;
(*_24) = true;
(*_35) = _4;
match _19 {
0 => bb6,
1 => bb16,
2 => bb9,
1399579662 => bb23,
_ => bb19
}
}
bb23 = {
_32 = _27 << _36;
_23.2 = -_15;
(*_24) = false ^ true;
_17.0 = core::ptr::addr_of!(_22);
(*_35) = [(-115135325075267264533808801511213618667_i128),110869424968160136629392824956892297581_i128];
_44 = 140850375390084804289002726405474725039_i128 ^ 116813378065261473588025079193769121271_i128;
_41 = _36 as u128;
_27 = _32;
_18 = [797934461619581763_usize,14845061376707894249_usize,7_usize,12943127710088593247_usize,3914851410654840734_usize,0_usize];
(*_24) = _36 <= _3;
(*_24) = _32 == _32;
(*_35) = [_44,_44];
(*_35) = _1;
_36 = _3 >> _27;
(*_35) = [_44,_44];
(*_24) = !true;
(*_24) = _36 == _36;
match _19 {
0 => bb14,
1399579662 => bb24,
_ => bb13
}
}
bb24 = {
_1 = [_44,_44];
(*_24) = _27 != _27;
_18 = [5_usize,14319782642213180800_usize,14996751016156429055_usize,11819269653193491758_usize,18071137576385247877_usize,3_usize];
_34 = (6827517471023354160_u64, (*_35));
_32 = _3 as u16;
_3 = _36 - _36;
(*_35) = [_44,_44];
(*_35) = [_44,_44];
_26 = [_3];
_51 = _27;
(*_35) = [_44,_44];
(*_35) = [_44,_44];
_34.1 = [_44,_44];
(*_35) = [_44,_44];
(*_24) = _51 != _27;
(*_35) = _1;
_48 = [_41,_41,_41,_41,_41,_41];
_19 = (*_24) as i32;
_1 = [_44,_44];
(*_35) = [_44,_44];
_4 = (*_35);
(*_35) = [_44,_44];
_46.1 = &mut _48;
Goto(bb25)
}
bb25 = {
(*_35) = _34.1;
_33 = 2_usize;
_47 = _31;
_19 = 1410977118_i32 & 1876688603_i32;
_18 = [_33,_33,_33,_33,_33,_33];
(*_24) = !_10[_33];
(*_35) = [_44,_44];
_52[_33] = _33 as i32;
_35 = &mut RET;
_21 = _31;
(*_24) = _10[_33];
Call((*_20) = fn19(Move(_17), (*_24), (*_35), Move(_24), Move(_46.1), (*_24), (*_35)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 | _19;
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
_8 = Move(Field::<*mut u128>(Variant((*_20), 1), 2));
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = Move(_8);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant(_7, 1), 1), 0), 0)];
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 - _19;
_38 = core::ptr::addr_of!(place!(Field::<Adt20>(Variant((*_20), 1), 1)));
place!(Field::<i128>(Variant((*_38), 0), 0)) = _44;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant(_7, 1), 1)), 0), 4)) = -_19;
place!(Field::<char>(Variant((*_38), 0), 1)) = _31;
_41 = 324001642748878915641664547209486407709_u128 + 5205399302536314885916262960649814034_u128;
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = -_44;
place!(Field::<char>(Variant((*_38), 0), 1)) = _47;
place!(Field::<Adt20>(Variant((*_20), 1), 1)) = Adt20::Variant1 { fld0: (-2565_i16) };
place!(Field::<u64>(Variant((*_20), 1), 0)) = _34.0;
place!(Field::<i16>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 1), 0)) = 11685_i16 | 3156_i16;
place!(Field::<u64>(Variant((*_20), 1), 0)) = _21 as u64;
(*_35) = _34.1;
place!(Field::<Adt20>(Variant((*_20), 1), 1)) = Adt20::Variant0 { fld0: _44,fld1: _21,fld2: 114_u8,fld3: Field::<u64>(Variant((*_20), 1), 0),fld4: _19 };
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = !_19;
_11 = _41 as isize;
Goto(bb27)
}
bb27 = {
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_42);
_22 = 102_u8 as isize;
place!(Field::<u64>(Variant(_7, 1), 0)) = _27 as u64;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _13 as i32;
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _47;
place!(Field::<u8>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 2)) = !202_u8;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
_40 = &mut place!(Field::<u8>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 2));
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
(*_40) = (-388_i16) as u8;
place!(Field::<u64>(Variant((*_20), 1), 0)) = _27 as u64;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = !Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _47;
match _34.0 {
0 => bb7,
6827517471023354160 => bb29,
_ => bb28
}
}
bb28 = {
Return()
}
bb29 = {
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
_2 = -_3;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = 5004352763372766196_i64 as i32;
(*_40) = Field::<u64>(Variant((*_20), 1), 0) as u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 - _19;
_60 = -_23.2;
_11 = _2 - _2;
place!(Field::<u64>(Variant((*_20), 1), 0)) = (*_40) as u64;
_27 = _51 | _51;
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _21;
_13 = 16_i8 - 22_i8;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0) - Field::<u64>(Variant((*_20), 1), 0);
(*_40) = !53_u8;
Goto(bb30)
}
bb30 = {
(*_40) = 221_u8 & 203_u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 ^ _19;
_27 = Field::<char>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 1) as u16;
_34.1 = (*_35);
(*_40) = _41 as u8;
(*_35) = _1;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = !Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3) >> Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 & _44;
_62 = [_13,_13,_13];
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
_30.0 = &_62;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 ^ _19;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_42);
place!(Field::<u64>(Variant((*_20), 1), 0)) = _23.2 as u64;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = Field::<char>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 1) as i32;
_59 = [_33,_33,_33,_33];
place!(Field::<u64>(Variant((*_20), 1), 0)) = !Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
(*_40) = 7_u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
(*_40) = 225_u8;
Goto(bb31)
}
bb31 = {
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 + _44;
_66 = [_13,_13,_13,_13,_13,_13,_13,_13];
(*_40) = _3 as u8;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0) + Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44;
_4 = (*_35);
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0) - Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 ^ _19;
(*_40) = Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0) as u8;
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
(*_40) = 229_u8 >> Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _47;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 >> Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0) | Field::<u64>(Variant((*_20), 1), 0);
(*_40) = 6_u8 * 105_u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = !_19;
Goto(bb32)
}
bb32 = {
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 * _44;
_41 = 190562689631618730654215311630723303766_u128;
_69 = true ^ true;
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 | _44;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44;
_49 = _33;
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 | _44;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
(*_35) = _34.1;
match _33 {
0 => bb9,
1 => bb33,
3 => bb35,
4 => bb36,
5 => bb37,
6 => bb38,
2 => bb40,
_ => bb39
}
}
bb33 = {
Return()
}
bb34 = {
(*_40) = 221_u8 & 203_u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 ^ _19;
_27 = Field::<char>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 1) as u16;
_34.1 = (*_35);
(*_40) = _41 as u8;
(*_35) = _1;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = !Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3) >> Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 & _44;
_62 = [_13,_13,_13];
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
_30.0 = &_62;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 ^ _19;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_42);
place!(Field::<u64>(Variant((*_20), 1), 0)) = _23.2 as u64;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = Field::<char>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 1) as i32;
_59 = [_33,_33,_33,_33];
place!(Field::<u64>(Variant((*_20), 1), 0)) = !Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
(*_40) = 7_u8;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
(*_40) = 225_u8;
Goto(bb31)
}
bb35 = {
RET = [(-89749296031460096818839955377267979292_i128),(-110762525592286670999991255583943778248_i128)];
_4 = [133922258124468765373163068454263145053_i128,162711844677506369997904583637102222952_i128];
_1 = _6;
_9 = (18289721086953900278_u64, _6);
_3 = _2 ^ _2;
_2 = 201783499686822489469426922908145941596_u128 as isize;
_9.0 = 7427582791430215642_u64;
_5 = [(-121372160937871814375354457392936271516_i128),(-84990754767240255501398521695525548007_i128)];
_3 = _2 >> _9.0;
_9 = (2901003408484776068_u64, _6);
_6 = [2544697446778011693574597300033645486_i128,152184472602298486485195552114656240084_i128];
_9.0 = !16093843330608790565_u64;
_6 = _9.1;
_11 = _3 * _3;
_10 = [true,true,true,false,true,false,true];
_9.0 = true as u64;
_13 = 46_i8;
Call(_3 = fn6(_9.0, _13, _2, _9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb36 = {
_17.0 = core::ptr::addr_of!(_22);
_23.2 = _15 * _15;
_9.0 = !_34.0;
_35 = &mut _6;
RET = (*_35);
_34.0 = _9.0 - _9.0;
_34.1 = [102147650272572839958172061898889327962_i128,(-149304088132105204617272019838205596519_i128)];
_11 = _22 + _36;
_31 = _21;
(*_35) = [(-3681908983890903892870585177113865177_i128),13447634892631952254849234937595602931_i128];
_3 = _11 << _32;
_36 = _3 | _22;
(*_35) = [(-163535540795813250258137241496578992957_i128),132291323109778250666420380017268146026_i128];
(*_35) = [(-84295569751881464118736434509712082993_i128),81707423632115677416852739469464104392_i128];
_35 = &mut _9.1;
(*_35) = [(-111533052532196865022347462608484303717_i128),128159527193117613252718588662275791848_i128];
(*_24) = false;
(*_35) = _4;
(*_24) = true;
(*_35) = _4;
match _19 {
0 => bb6,
1 => bb16,
2 => bb9,
1399579662 => bb23,
_ => bb19
}
}
bb37 = {
Return()
}
bb38 = {
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 | _19;
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
_8 = Move(Field::<*mut u128>(Variant((*_20), 1), 2));
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = Move(_8);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant(_7, 1), 1), 0), 0)];
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19 - _19;
_38 = core::ptr::addr_of!(place!(Field::<Adt20>(Variant((*_20), 1), 1)));
place!(Field::<i128>(Variant((*_38), 0), 0)) = _44;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant(_7, 1), 1)), 0), 4)) = -_19;
place!(Field::<char>(Variant((*_38), 0), 1)) = _31;
_41 = 324001642748878915641664547209486407709_u128 + 5205399302536314885916262960649814034_u128;
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = -_44;
place!(Field::<char>(Variant((*_38), 0), 1)) = _47;
place!(Field::<Adt20>(Variant((*_20), 1), 1)) = Adt20::Variant1 { fld0: (-2565_i16) };
place!(Field::<u64>(Variant((*_20), 1), 0)) = _34.0;
place!(Field::<i16>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 1), 0)) = 11685_i16 | 3156_i16;
place!(Field::<u64>(Variant((*_20), 1), 0)) = _21 as u64;
(*_35) = _34.1;
place!(Field::<Adt20>(Variant((*_20), 1), 1)) = Adt20::Variant0 { fld0: _44,fld1: _21,fld2: 114_u8,fld3: Field::<u64>(Variant((*_20), 1), 0),fld4: _19 };
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = !_19;
_11 = _41 as isize;
Goto(bb27)
}
bb39 = {
(*_35) = _34.1;
_33 = 2_usize;
_47 = _31;
_19 = 1410977118_i32 & 1876688603_i32;
_18 = [_33,_33,_33,_33,_33,_33];
(*_24) = !_10[_33];
(*_35) = [_44,_44];
_52[_33] = _33 as i32;
_35 = &mut RET;
_21 = _31;
(*_24) = _10[_33];
Call((*_20) = fn19(Move(_17), (*_24), (*_35), Move(_24), Move(_46.1), (*_24), (*_35)), ReturnTo(bb26), UnwindUnreachable())
}
bb40 = {
_25 = &mut _40;
_78 = 3369277508772612289_i64;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _52[_49];
_10 = [_69,_69,_69,_69,_69,_69,_69];
_67 = _2;
_24 = &mut _69;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _52[_49];
place!(Field::<u64>(Variant((*_20), 1), 0)) = _47 as u64;
(*_24) = _10[_49];
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0);
_36 = _2 << _19;
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3) | Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _47;
_26 = [_3];
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44;
_59 = [_33,_18[_49],_49,_49];
_66 = [_62[_49],_62[_49],_62[_49],_13,_62[_49],_62[_49],_13,_62[_49]];
_65 = !_59[_49];
_70 = _11 as f32;
_77 = (_34.0, (*_35));
_66[_49] = _62[_49] | _13;
_73 = [34_u8,224_u8,155_u8,114_u8,145_u8,244_u8,224_u8];
_77.0 = Field::<u64>(Variant((*_20), 1), 0) >> Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0);
_59[_49] = _65;
Call(_26 = core::intrinsics::transmute(Field::<u64>(Variant((*_20), 1), 0)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
(*_24) = _10[_49] | _10[_49];
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _19;
_55 = &(*_24);
(*_35) = _1;
_83 = _62;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0);
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 0)) = _44 ^ _44;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
(*_25) = &mut _73[_49];
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_42);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _52[_49] * _19;
(*_24) = _10[_49] ^ _10[_49];
(*_24) = _36 >= _67;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = !_77.0;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = _70 as u64;
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = !_19;
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _31;
(*_24) = !_10[_49];
(*_24) = _67 <= _11;
match _33 {
0 => bb28,
1 => bb42,
3 => bb44,
4 => bb45,
2 => bb47,
_ => bb46
}
}
bb42 = {
_17.0 = core::ptr::addr_of!(_22);
_23.2 = _15 * _15;
_9.0 = !_34.0;
_35 = &mut _6;
RET = (*_35);
_34.0 = _9.0 - _9.0;
_34.1 = [102147650272572839958172061898889327962_i128,(-149304088132105204617272019838205596519_i128)];
_11 = _22 + _36;
_31 = _21;
(*_35) = [(-3681908983890903892870585177113865177_i128),13447634892631952254849234937595602931_i128];
_3 = _11 << _32;
_36 = _3 | _22;
(*_35) = [(-163535540795813250258137241496578992957_i128),132291323109778250666420380017268146026_i128];
(*_35) = [(-84295569751881464118736434509712082993_i128),81707423632115677416852739469464104392_i128];
_35 = &mut _9.1;
(*_35) = [(-111533052532196865022347462608484303717_i128),128159527193117613252718588662275791848_i128];
(*_24) = false;
(*_35) = _4;
(*_24) = true;
(*_35) = _4;
match _19 {
0 => bb6,
1 => bb16,
2 => bb9,
1399579662 => bb23,
_ => bb19
}
}
bb43 = {
_1 = [_44,_44];
(*_24) = _27 != _27;
_18 = [5_usize,14319782642213180800_usize,14996751016156429055_usize,11819269653193491758_usize,18071137576385247877_usize,3_usize];
_34 = (6827517471023354160_u64, (*_35));
_32 = _3 as u16;
_3 = _36 - _36;
(*_35) = [_44,_44];
(*_35) = [_44,_44];
_26 = [_3];
_51 = _27;
(*_35) = [_44,_44];
(*_35) = [_44,_44];
_34.1 = [_44,_44];
(*_35) = [_44,_44];
(*_24) = _51 != _27;
(*_35) = _1;
_48 = [_41,_41,_41,_41,_41,_41];
_19 = (*_24) as i32;
_1 = [_44,_44];
(*_35) = [_44,_44];
_4 = (*_35);
(*_35) = [_44,_44];
_46.1 = &mut _48;
Goto(bb25)
}
bb44 = {
RET = _6;
_11 = _13 as isize;
_10 = [true,false,false,false,false,false,true];
_5 = _1;
_18 = [3_usize,7_usize,13226307814637282256_usize,2_usize,3_usize,2_usize];
_20 = core::ptr::addr_of_mut!(_7);
_13 = (-102_i8) >> _2;
_23.2 = -_15;
_3 = 136_u8 as isize;
_22 = 216918696164384125384730834392729627772_u128 as isize;
_19 = 7135716584961253227235616730698166267_u128 as i32;
_18 = [7_usize,6_usize,16132162040753918901_usize,4_usize,735286130205085887_usize,7_usize];
_19 = 1647707341_i32;
_9.1 = [(-139245422068829331473321963383093754631_i128),145704185312634177225722754517790114476_i128];
_15 = _23.2 + _23.2;
_11 = !_3;
_2 = _22 >> _22;
_9.0 = 88937092648520213118296350282917511342_i128 as u64;
_2 = _3;
Goto(bb12)
}
bb45 = {
Return()
}
bb46 = {
RET = [(-89749296031460096818839955377267979292_i128),(-110762525592286670999991255583943778248_i128)];
_4 = [133922258124468765373163068454263145053_i128,162711844677506369997904583637102222952_i128];
_1 = _6;
_9 = (18289721086953900278_u64, _6);
_3 = _2 ^ _2;
_2 = 201783499686822489469426922908145941596_u128 as isize;
_9.0 = 7427582791430215642_u64;
_5 = [(-121372160937871814375354457392936271516_i128),(-84990754767240255501398521695525548007_i128)];
_3 = _2 >> _9.0;
_9 = (2901003408484776068_u64, _6);
_6 = [2544697446778011693574597300033645486_i128,152184472602298486485195552114656240084_i128];
_9.0 = !16093843330608790565_u64;
_6 = _9.1;
_11 = _3 * _3;
_10 = [true,true,true,false,true,false,true];
_9.0 = true as u64;
_13 = 46_i8;
Call(_3 = fn6(_9.0, _13, _2, _9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb47 = {
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _2 as i32;
(*_35) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0)];
_62[_49] = _83[_49] >> _2;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_20), 1), 0) - Field::<u64>(Variant((*_20), 1), 0);
_75 = Field::<char>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 1) as u64;
_18 = [_65,_65,_59[_49],_65,_49,_49];
_33 = _49 ^ _49;
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3) << Field::<i32>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 4);
_82 = &mut (*_24);
_75 = Field::<u64>(Variant((*_20), 1), 0) % _34.0;
_59[_49] = (-28462_i16) as usize;
_68 = [59_u8,124_u8,216_u8,199_u8,50_u8,255_u8,36_u8];
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 1)) = _47;
_52[_49] = Field::<i32>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 4) + Field::<i32>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 4);
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
(*_82) = Field::<i32>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 4) < Field::<i32>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 4);
_44 = Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0) << _3;
place!(Field::<u64>(Variant((*_20), 1), 0)) = !Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3);
place!(Field::<u64>(Variant((*_20), 1), 0)) = Field::<u64>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 3) * _75;
Goto(bb48)
}
bb48 = {
_71 = _33 ^ _33;
_83 = [_62[_49],_66[_49],_66[_49]];
_77.0 = _49 as u64;
place!(Field::<*mut u128>(Variant((*_20), 1), 2)) = core::ptr::addr_of_mut!(_41);
_44 = -Field::<i128>(Variant(Field::<Adt20>(Variant((*_20), 1), 1), 0), 0);
_89 = _78;
Goto(bb49)
}
bb49 = {
(*_25) = &mut _68[_49];
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_20), 1), 1)), 0), 4)) = _52[_49];
Goto(bb50)
}
bb50 = {
Call(_91 = dump_var(Move(_1), Move(_4), Move(_9), Move(_49)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_91 = dump_var(Move(_34), Move(_44), Move(_11), Move(_3)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_91 = dump_var(Move(_21), Move(_67), Move(_48), Move(_37)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_91 = dump_var(Move(_71), Move(_78), Move(_65), Move(_36)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_91 = dump_var(Move(_31), Move(_69), Move(_89), Move(_51)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_91 = dump_var(Move(_27), _92, _92, _92), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u64,mut _2: i8,mut _3: isize,mut _4: u64) -> isize {
mir! {
type RET = isize;
let _5: *const [bool; 7];
let _6: (&'static i32, *mut u128, i8, Adt20);
let _7: &'static mut [u128; 6];
let _8: usize;
let _9: bool;
let _10: [i8; 3];
let _11: &'static (Adt19,);
let _12: *const Adt20;
let _13: (u64, [i128; 2]);
let _14: *const *mut [u128; 6];
let _15: *const Adt19;
let _16: &'static mut [u128; 6];
let _17: *const Adt27;
let _18: isize;
let _19: [usize; 4];
let _20: [u8; 7];
let _21: (u64, [i128; 2]);
let _22: f32;
let _23: u64;
let _24: u32;
let _25: [bool; 7];
let _26: &'static i32;
let _27: i16;
let _28: [char; 1];
let _29: &'static mut bool;
let _30: &'static u32;
let _31: *const u32;
let _32: u64;
let _33: Adt76;
let _34: f32;
let _35: bool;
let _36: &'static mut Adt20;
let _37: char;
let _38: bool;
let _39: [usize; 4];
let _40: f32;
let _41: *const [bool; 7];
let _42: &'static &'static i64;
let _43: [usize; 4];
let _44: *const isize;
let _45: isize;
let _46: char;
let _47: &'static Adt27;
let _48: char;
let _49: f64;
let _50: Adt19;
let _51: i16;
let _52: i64;
let _53: isize;
let _54: char;
let _55: [u128; 6];
let _56: bool;
let _57: &'static mut u8;
let _58: u128;
let _59: isize;
let _60: *mut isize;
let _61: &'static bool;
let _62: usize;
let _63: *mut u128;
let _64: Adt38;
let _65: Adt76;
let _66: isize;
let _67: Adt19;
let _68: &'static mut [i128; 2];
let _69: *const Adt38;
let _70: &'static mut &'static mut u8;
let _71: &'static mut [u128; 6];
let _72: f64;
let _73: [char; 1];
let _74: f64;
let _75: bool;
let _76: &'static i32;
let _77: ([char; 1], Adt20);
let _78: [u32; 8];
let _79: bool;
let _80: bool;
let _81: &'static u32;
let _82: u128;
let _83: *mut [u128; 6];
let _84: isize;
let _85: *mut isize;
let _86: *const isize;
let _87: char;
let _88: [u32; 8];
let _89: *const Adt19;
let _90: u32;
let _91: *mut isize;
let _92: &'static u32;
let _93: u128;
let _94: &'static u32;
let _95: Adt27;
let _96: f64;
let _97: (u64, [i128; 2]);
let _98: char;
let _99: u32;
let _100: &'static mut (&'static i32, *mut u128, i8, Adt20);
let _101: i128;
let _102: &'static [u16; 4];
let _103: [i8; 8];
let _104: *const [bool; 7];
let _105: *const [bool; 7];
let _106: &'static &'static Adt27;
let _107: *mut f32;
let _108: u128;
let _109: bool;
let _110: *const Adt20;
let _111: char;
let _112: f32;
let _113: isize;
let _114: i64;
let _115: *const u32;
let _116: *const isize;
let _117: *mut isize;
let _118: &'static [u16; 4];
let _119: &'static mut [u128; 6];
let _120: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _121: *mut isize;
let _122: ();
let _123: ();
{
_3 = (-23_isize);
_2 = (-59_i8) | (-66_i8);
_4 = _1 << _1;
_4 = !_1;
_6.3 = Adt20::Variant2 { fld0: (-509298706_i32) };
RET = _3 | _3;
_6.2 = _2 - _2;
_3 = RET & RET;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = (-1154369188_i32);
_4 = !_1;
_2 = _6.2;
_4 = _1 + _1;
RET = _3;
_2 = _6.2 + _6.2;
_6.2 = Field::<i32>(Variant(_6.3, 2), 0) as i8;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = (-761646747_i32) - 682753773_i32;
_4 = _1 >> _2;
_6.0 = &place!(Field::<i32>(Variant(_6.3, 2), 0));
_8 = !1_usize;
_2 = RET as i8;
_3 = RET | RET;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = (-1415905172_i32);
Goto(bb1)
}
bb1 = {
_2 = _6.2 * _6.2;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1844272698_i32 >> RET;
_1 = Field::<i32>(Variant(_6.3, 2), 0) as u64;
_9 = true;
_10 = [_2,_2,_2];
_6.2 = _2;
Goto(bb2)
}
bb2 = {
_6.2 = 27738_i16 as i8;
_2 = !_6.2;
_12 = core::ptr::addr_of!(_6.3);
Goto(bb3)
}
bb3 = {
place!(Field::<i32>(Variant((*_12), 2), 0)) = 81726249147676564170768108526429460238_i128 as i32;
place!(Field::<i32>(Variant((*_12), 2), 0)) = 2129007516_i32 + 1391150319_i32;
(*_12) = Adt20::Variant0 { fld0: (-124162263630598505469142114116213616489_i128),fld1: '\u{ec58a}',fld2: 79_u8,fld3: _4,fld4: 1116117888_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = -2087421412_i32;
(*_12) = Adt20::Variant1 { fld0: (-31306_i16) };
place!(Field::<i16>(Variant(_6.3, 1), 0)) = !3128_i16;
place!(Field::<i16>(Variant((*_12), 1), 0)) = 285_i16 ^ (-14200_i16);
(*_12) = Adt20::Variant0 { fld0: 70183301431485464916153625003438269479_i128,fld1: '\u{f1b4}',fld2: 63_u8,fld3: _4,fld4: 900138664_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-1448206978_i32) | 1744252373_i32;
place!(Field::<i32>(Variant((*_12), 0), 4)) = 1649347498_i32 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{96f08}';
(*_12) = Adt20::Variant2 { fld0: 947694757_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 259440887_i32;
_13.1 = [104936425986764342171329294523137017098_i128,(-112123470732516126602600738652581764813_i128)];
(*_12) = Adt20::Variant0 { fld0: (-146890344886304287503445419809511659091_i128),fld1: '\u{ef412}',fld2: 30_u8,fld3: _4,fld4: 1573893390_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 478545352_i32 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<u8>(Variant((*_12), 0), 2)) = 138_u8;
(*_12) = Adt20::Variant0 { fld0: 92574131153095145713471808674701166878_i128,fld1: '\u{81700}',fld2: 207_u8,fld3: _4,fld4: 1011108783_i32 };
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{50c2b}';
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-158777072076536010421727061713186599611_i128) as i32;
place!(Field::<i128>(Variant((*_12), 0), 0)) = !66325320572981648056421905340447083368_i128;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4 * _1;
(*_12) = Adt20::Variant1 { fld0: 8720_i16 };
(*_12) = Adt20::Variant0 { fld0: (-107223752675454863562894116399195176803_i128),fld1: '\u{dbe9b}',fld2: 146_u8,fld3: _4,fld4: (-551157593_i32) };
(*_12) = Adt20::Variant0 { fld0: 54589463496451985302242692874377229020_i128,fld1: '\u{10ec03}',fld2: 9_u8,fld3: _1,fld4: (-1075721524_i32) };
(*_12) = Adt20::Variant0 { fld0: (-130343386632826272004565358922637682646_i128),fld1: '\u{aeb40}',fld2: 81_u8,fld3: _4,fld4: 23014087_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 25800_u16 as i32;
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-97115285069782218710739086573395163965_i128) << Field::<u64>(Variant((*_12), 0), 3);
Call((*_12) = fn7(Move(_12), _8, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _1 << Field::<i32>(Variant(_6.3, 0), 4);
_12 = core::ptr::addr_of!(_6.3);
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-137803016815654163740610010757836020884_i128) | 39619516352087172927591497610564231579_i128;
(*_12) = Adt20::Variant0 { fld0: 47354703894769808663887416251743990379_i128,fld1: '\u{db1a3}',fld2: 141_u8,fld3: _4,fld4: (-295405030_i32) };
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-1792751517_i32) & 391086142_i32;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4 ^ _4;
place!(Field::<u8>(Variant((*_12), 0), 2)) = (-3608755535662740693_i64) as u8;
place!(Field::<i128>(Variant((*_12), 0), 0)) = 158088642274986834734929335999177443836_i128 & 15285631494785596801497743288081890010_i128;
(*_12) = Adt20::Variant2 { fld0: (-903021403_i32) };
place!(Field::<i32>(Variant((*_12), 2), 0)) = -700943160_i32;
_8 = 5_usize >> _1;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = '\u{1ca70}' as i32;
place!(Field::<i32>(Variant((*_12), 2), 0)) = -1948148970_i32;
(*_12) = Adt20::Variant0 { fld0: (-16710412342470526075197898224039910150_i128),fld1: '\u{8e2e}',fld2: 92_u8,fld3: _1,fld4: 1988689078_i32 };
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{3ac94}';
place!(Field::<u8>(Variant((*_12), 0), 2)) = 73_u8 | 244_u8;
place!(Field::<i32>(Variant((*_12), 0), 4)) = 1874818640_i32;
place!(Field::<i128>(Variant(_6.3, 0), 0)) = 2572930915_u32 as i128;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4 - _4;
(*_12) = Adt20::Variant2 { fld0: 766078684_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-558537554_i32);
(*_12) = Adt20::Variant1 { fld0: 30_i16 };
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-17451_i16);
match Field::<i16>(Variant((*_12), 1), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
340282366920938463463374607431768194005 => bb8,
_ => bb7
}
}
bb5 = {
place!(Field::<i32>(Variant((*_12), 2), 0)) = 81726249147676564170768108526429460238_i128 as i32;
place!(Field::<i32>(Variant((*_12), 2), 0)) = 2129007516_i32 + 1391150319_i32;
(*_12) = Adt20::Variant0 { fld0: (-124162263630598505469142114116213616489_i128),fld1: '\u{ec58a}',fld2: 79_u8,fld3: _4,fld4: 1116117888_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = -2087421412_i32;
(*_12) = Adt20::Variant1 { fld0: (-31306_i16) };
place!(Field::<i16>(Variant(_6.3, 1), 0)) = !3128_i16;
place!(Field::<i16>(Variant((*_12), 1), 0)) = 285_i16 ^ (-14200_i16);
(*_12) = Adt20::Variant0 { fld0: 70183301431485464916153625003438269479_i128,fld1: '\u{f1b4}',fld2: 63_u8,fld3: _4,fld4: 900138664_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-1448206978_i32) | 1744252373_i32;
place!(Field::<i32>(Variant((*_12), 0), 4)) = 1649347498_i32 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{96f08}';
(*_12) = Adt20::Variant2 { fld0: 947694757_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 259440887_i32;
_13.1 = [104936425986764342171329294523137017098_i128,(-112123470732516126602600738652581764813_i128)];
(*_12) = Adt20::Variant0 { fld0: (-146890344886304287503445419809511659091_i128),fld1: '\u{ef412}',fld2: 30_u8,fld3: _4,fld4: 1573893390_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 478545352_i32 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<u8>(Variant((*_12), 0), 2)) = 138_u8;
(*_12) = Adt20::Variant0 { fld0: 92574131153095145713471808674701166878_i128,fld1: '\u{81700}',fld2: 207_u8,fld3: _4,fld4: 1011108783_i32 };
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{50c2b}';
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-158777072076536010421727061713186599611_i128) as i32;
place!(Field::<i128>(Variant((*_12), 0), 0)) = !66325320572981648056421905340447083368_i128;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4 * _1;
(*_12) = Adt20::Variant1 { fld0: 8720_i16 };
(*_12) = Adt20::Variant0 { fld0: (-107223752675454863562894116399195176803_i128),fld1: '\u{dbe9b}',fld2: 146_u8,fld3: _4,fld4: (-551157593_i32) };
(*_12) = Adt20::Variant0 { fld0: 54589463496451985302242692874377229020_i128,fld1: '\u{10ec03}',fld2: 9_u8,fld3: _1,fld4: (-1075721524_i32) };
(*_12) = Adt20::Variant0 { fld0: (-130343386632826272004565358922637682646_i128),fld1: '\u{aeb40}',fld2: 81_u8,fld3: _4,fld4: 23014087_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 25800_u16 as i32;
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-97115285069782218710739086573395163965_i128) << Field::<u64>(Variant((*_12), 0), 3);
Call((*_12) = fn7(Move(_12), _8, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_6.2 = 27738_i16 as i8;
_2 = !_6.2;
_12 = core::ptr::addr_of!(_6.3);
Goto(bb3)
}
bb7 = {
_2 = _6.2 * _6.2;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1844272698_i32 >> RET;
_1 = Field::<i32>(Variant(_6.3, 2), 0) as u64;
_9 = true;
_10 = [_2,_2,_2];
_6.2 = _2;
Goto(bb2)
}
bb8 = {
(*_12) = Adt20::Variant0 { fld0: 36678714917071768566898685149761954833_i128,fld1: '\u{d9d44}',fld2: 145_u8,fld3: _4,fld4: 364578093_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = -(-225594637_i32);
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-63169192663984423268496462855778727981_i128) * (-123971365905385377840328618739064936357_i128);
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{6e08a}';
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-98695208909239532041372550220952826061_i128) - (-30381507111443791073854137568834610660_i128);
(*_12) = Adt20::Variant2 { fld0: 1158381875_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 963407579_i32 + (-291675805_i32);
(*_12) = Adt20::Variant0 { fld0: 61727658514429235342792662495345566797_i128,fld1: '\u{c3b3f}',fld2: 183_u8,fld3: _1,fld4: (-1624984404_i32) };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 882844700_i32;
_1 = !Field::<u64>(Variant((*_12), 0), 3);
(*_12) = Adt20::Variant1 { fld0: (-21564_i16) };
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-5175_i16);
RET = _9 as isize;
_8 = 74219756479420089_usize | 2680368508466849577_usize;
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-29058_i16) * 13003_i16;
_6.3 = Adt20::Variant2 { fld0: 205728674_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-880819509_i32);
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1069792866_i32;
(*_12) = Adt20::Variant0 { fld0: (-96491365523238521951913456271339468054_i128),fld1: '\u{b4a2f}',fld2: 126_u8,fld3: _1,fld4: (-1783712851_i32) };
place!(Field::<i128>(Variant((*_12), 0), 0)) = _6.2 as i128;
place!(Field::<u64>(Variant((*_12), 0), 3)) = !_4;
place!(Field::<u8>(Variant((*_12), 0), 2)) = !17_u8;
Goto(bb9)
}
bb9 = {
place!(Field::<i32>(Variant((*_12), 0), 4)) = 1709686165_i32;
(*_12) = Adt20::Variant1 { fld0: (-8283_i16) };
(*_12) = Adt20::Variant0 { fld0: 69191478159770785572820726727342800974_i128,fld1: '\u{d1383}',fld2: 162_u8,fld3: _4,fld4: (-1312903894_i32) };
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{f8761}';
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-115485839_i32) >> Field::<u64>(Variant((*_12), 0), 3);
(*_12) = Adt20::Variant2 { fld0: 523022575_i32 };
(*_12) = Adt20::Variant0 { fld0: 116292296412080168212793189705067175415_i128,fld1: '\u{8e646}',fld2: 15_u8,fld3: _4,fld4: 1847392793_i32 };
place!(Field::<i128>(Variant((*_12), 0), 0)) = _8 as i128;
(*_12) = Adt20::Variant0 { fld0: 114266994817425276047211791578733218931_i128,fld1: '\u{66dde}',fld2: 95_u8,fld3: _4,fld4: (-1124290418_i32) };
place!(Field::<u8>(Variant((*_12), 0), 2)) = 191_u8 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<i128>(Variant((*_12), 0), 0)) = _9 as i128;
(*_12) = Adt20::Variant1 { fld0: 3393_i16 };
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-28492_i16);
(*_12) = Adt20::Variant0 { fld0: 39054190576035727050204792561792626070_i128,fld1: '\u{d40e3}',fld2: 81_u8,fld3: _1,fld4: 210349433_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = _3 as i32;
_21 = (Field::<u64>(Variant((*_12), 0), 3), _13.1);
(*_12) = Adt20::Variant2 { fld0: (-708893949_i32) };
(*_12) = Adt20::Variant0 { fld0: 36999647274634969659464345347782436044_i128,fld1: '\u{69af9}',fld2: 60_u8,fld3: _1,fld4: (-1076538058_i32) };
(*_12) = Adt20::Variant1 { fld0: 3851_i16 };
(*_12) = Adt20::Variant2 { fld0: (-1520631495_i32) };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-792814229_i32) * (-577972119_i32);
_21 = (_4, _13.1);
Goto(bb10)
}
bb10 = {
(*_12) = Adt20::Variant0 { fld0: 158975391155317954428596037942234091260_i128,fld1: '\u{39fbe}',fld2: 16_u8,fld3: _4,fld4: (-1325441078_i32) };
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4;
(*_12) = Adt20::Variant2 { fld0: 271810643_i32 };
_13.0 = _4 * _4;
(*_12) = Adt20::Variant0 { fld0: 2788826919162843622117509995493788541_i128,fld1: '\u{b1a52}',fld2: 82_u8,fld3: _21.0,fld4: 77825043_i32 };
place!(Field::<u8>(Variant((*_12), 0), 2)) = !10_u8;
place!(Field::<u8>(Variant((*_12), 0), 2)) = 236850806487967980570641277372246685080_u128 as u8;
(*_12) = Adt20::Variant0 { fld0: 1052484492638383976347427109546075431_i128,fld1: '\u{22df8}',fld2: 16_u8,fld3: _21.0,fld4: 672430621_i32 };
_19 = [_8,_8,_8,_8];
_24 = !383898268_u32;
(*_12) = Adt20::Variant2 { fld0: (-119081388_i32) };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 245_u8 as i32;
_23 = _21.0 << _13.0;
Goto(bb11)
}
bb11 = {
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-1690296674_i32) ^ 2135889668_i32;
_5 = core::ptr::addr_of!(_25);
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_27 = !(-5261_i16);
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-1674925284_i32);
_22 = 114_u16 as f32;
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_1 = _23 | _23;
_21.1 = [(-75795224786753685306793571227107800030_i128),71812734741743678608987033666141543597_i128];
match Field::<i32>(Variant((*_12), 2), 0) {
0 => bb7,
1 => bb9,
2 => bb3,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463463374607430093286172 => bb17,
_ => bb16
}
}
bb12 = {
(*_12) = Adt20::Variant0 { fld0: 158975391155317954428596037942234091260_i128,fld1: '\u{39fbe}',fld2: 16_u8,fld3: _4,fld4: (-1325441078_i32) };
place!(Field::<u64>(Variant((*_12), 0), 3)) = _4;
(*_12) = Adt20::Variant2 { fld0: 271810643_i32 };
_13.0 = _4 * _4;
(*_12) = Adt20::Variant0 { fld0: 2788826919162843622117509995493788541_i128,fld1: '\u{b1a52}',fld2: 82_u8,fld3: _21.0,fld4: 77825043_i32 };
place!(Field::<u8>(Variant((*_12), 0), 2)) = !10_u8;
place!(Field::<u8>(Variant((*_12), 0), 2)) = 236850806487967980570641277372246685080_u128 as u8;
(*_12) = Adt20::Variant0 { fld0: 1052484492638383976347427109546075431_i128,fld1: '\u{22df8}',fld2: 16_u8,fld3: _21.0,fld4: 672430621_i32 };
_19 = [_8,_8,_8,_8];
_24 = !383898268_u32;
(*_12) = Adt20::Variant2 { fld0: (-119081388_i32) };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 245_u8 as i32;
_23 = _21.0 << _13.0;
Goto(bb11)
}
bb13 = {
place!(Field::<i32>(Variant((*_12), 0), 4)) = 1709686165_i32;
(*_12) = Adt20::Variant1 { fld0: (-8283_i16) };
(*_12) = Adt20::Variant0 { fld0: 69191478159770785572820726727342800974_i128,fld1: '\u{d1383}',fld2: 162_u8,fld3: _4,fld4: (-1312903894_i32) };
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{f8761}';
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-115485839_i32) >> Field::<u64>(Variant((*_12), 0), 3);
(*_12) = Adt20::Variant2 { fld0: 523022575_i32 };
(*_12) = Adt20::Variant0 { fld0: 116292296412080168212793189705067175415_i128,fld1: '\u{8e646}',fld2: 15_u8,fld3: _4,fld4: 1847392793_i32 };
place!(Field::<i128>(Variant((*_12), 0), 0)) = _8 as i128;
(*_12) = Adt20::Variant0 { fld0: 114266994817425276047211791578733218931_i128,fld1: '\u{66dde}',fld2: 95_u8,fld3: _4,fld4: (-1124290418_i32) };
place!(Field::<u8>(Variant((*_12), 0), 2)) = 191_u8 << Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<i128>(Variant((*_12), 0), 0)) = _9 as i128;
(*_12) = Adt20::Variant1 { fld0: 3393_i16 };
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-28492_i16);
(*_12) = Adt20::Variant0 { fld0: 39054190576035727050204792561792626070_i128,fld1: '\u{d40e3}',fld2: 81_u8,fld3: _1,fld4: 210349433_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = _3 as i32;
_21 = (Field::<u64>(Variant((*_12), 0), 3), _13.1);
(*_12) = Adt20::Variant2 { fld0: (-708893949_i32) };
(*_12) = Adt20::Variant0 { fld0: 36999647274634969659464345347782436044_i128,fld1: '\u{69af9}',fld2: 60_u8,fld3: _1,fld4: (-1076538058_i32) };
(*_12) = Adt20::Variant1 { fld0: 3851_i16 };
(*_12) = Adt20::Variant2 { fld0: (-1520631495_i32) };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-792814229_i32) * (-577972119_i32);
_21 = (_4, _13.1);
Goto(bb10)
}
bb14 = {
_2 = _6.2 * _6.2;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1844272698_i32 >> RET;
_1 = Field::<i32>(Variant(_6.3, 2), 0) as u64;
_9 = true;
_10 = [_2,_2,_2];
_6.2 = _2;
Goto(bb2)
}
bb15 = {
_6.2 = 27738_i16 as i8;
_2 = !_6.2;
_12 = core::ptr::addr_of!(_6.3);
Goto(bb3)
}
bb16 = {
_6.2 = 27738_i16 as i8;
_2 = !_6.2;
_12 = core::ptr::addr_of!(_6.3);
Goto(bb3)
}
bb17 = {
_5 = core::ptr::addr_of!((*_5));
(*_12) = Adt20::Variant2 { fld0: (-222623361_i32) };
_19 = [_8,_8,_8,_8];
Goto(bb18)
}
bb18 = {
_31 = core::ptr::addr_of!(_24);
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-542674936_i32) & (-636515330_i32);
_6.3 = Adt20::Variant2 { fld0: 408558000_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 32099794178714999799688293886225290783_u128 as i32;
(*_12) = Adt20::Variant1 { fld0: _27 };
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
(*_12) = Adt20::Variant2 { fld0: 1666095225_i32 };
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_25 = [_9,_9,_9,_9,_9,_9,_9];
(*_12) = Adt20::Variant1 { fld0: _27 };
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_9 = !true;
Goto(bb19)
}
bb19 = {
_23 = _13.0 - _4;
(*_31) = !2427762483_u32;
place!(Field::<i16>(Variant((*_12), 1), 0)) = _27;
(*_31) = _22 as u32;
_6.3 = Adt20::Variant2 { fld0: 862580425_i32 };
_13.1 = _21.1;
Goto(bb20)
}
bb20 = {
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_34 = 253_u8 as f32;
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-256431313_i32) >> _4;
(*_12) = Adt20::Variant0 { fld0: 119939657062797578227758928865001937432_i128,fld1: '\u{2555}',fld2: 157_u8,fld3: _13.0,fld4: (-1360595863_i32) };
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-1641489784_i32);
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{48b9f}';
place!(Field::<i128>(Variant((*_12), 0), 0)) = 198_u8 as i128;
place!(Field::<i32>(Variant((*_12), 0), 4)) = -2063680993_i32;
place!(Field::<u64>(Variant((*_12), 0), 3)) = !_23;
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
place!(Field::<i32>(Variant((*_12), 0), 4)) = 458069794_i32;
place!(Field::<i32>(Variant((*_12), 0), 4)) = -(-184147715_i32);
place!(Field::<u8>(Variant((*_12), 0), 2)) = 14_u8 >> Field::<u64>(Variant((*_12), 0), 3);
Goto(bb21)
}
bb21 = {
place!(Field::<u8>(Variant((*_12), 0), 2)) = _8 as u8;
place!(Field::<i32>(Variant((*_12), 0), 4)) = -576787431_i32;
place!(Field::<u8>(Variant((*_12), 0), 2)) = 183_u8 - 162_u8;
place!(Field::<i32>(Variant((*_12), 0), 4)) = (-1786086131_i32) >> Field::<u64>(Variant((*_12), 0), 3);
place!(Field::<i32>(Variant((*_12), 0), 4)) = -(-852221617_i32);
_5 = core::ptr::addr_of!((*_5));
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
(*_5) = [_9,_9,_9,_9,_9,_9,_9];
_3 = RET & RET;
Goto(bb22)
}
bb22 = {
_29 = &mut _9;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _27 as u64;
place!(Field::<u64>(Variant((*_12), 0), 3)) = _1 | _1;
place!(Field::<u8>(Variant((*_12), 0), 2)) = !188_u8;
_5 = core::ptr::addr_of!((*_5));
_26 = &place!(Field::<i32>(Variant((*_12), 0), 4));
place!(Field::<i32>(Variant((*_12), 0), 4)) = 2131821676_i32 - (-1486581320_i32);
_36 = &mut (*_12);
Goto(bb23)
}
bb23 = {
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{ad788}';
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{ce263}';
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{6cea6}';
place!(Field::<i128>(Variant((*_36), 0), 0)) = 4007006053895132867835168247724124618_i128 - (-131326753436011189443647765110782767271_i128);
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_31) = _3 as u32;
place!(Field::<i128>(Variant((*_36), 0), 0)) = (-105851332607620032606322060687969554323_i128) + 87397215453262311306450530005215254547_i128;
_19 = [_8,_8,_8,_8];
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{24c3f}';
(*_36) = Adt20::Variant1 { fld0: _27 };
_1 = _23 | _23;
_23 = _21.0 ^ _1;
(*_29) = true ^ false;
_27 = -Field::<i16>(Variant((*_36), 1), 0);
_30 = &(*_31);
Goto(bb24)
}
bb24 = {
_20 = [253_u8,166_u8,155_u8,121_u8,168_u8,174_u8,255_u8];
(*_29) = !true;
_4 = !_1;
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_36) = Adt20::Variant0 { fld0: 36341865991578757289916439449891144655_i128,fld1: '\u{dea7e}',fld2: 107_u8,fld3: _23,fld4: (-1860677568_i32) };
place!(Field::<i32>(Variant((*_36), 0), 4)) = !899778502_i32;
(*_36) = Adt20::Variant2 { fld0: 1290828868_i32 };
(*_29) = !true;
_39 = [_8,_8,_8,_8];
_3 = RET & RET;
(*_36) = Adt20::Variant0 { fld0: 43418438505644947681118517577493375869_i128,fld1: '\u{b283}',fld2: 184_u8,fld3: _13.0,fld4: (-54132251_i32) };
_38 = Field::<u64>(Variant((*_36), 0), 3) >= _4;
place!(Field::<u64>(Variant((*_36), 0), 3)) = '\u{1fca}' as u64;
(*_36) = Adt20::Variant1 { fld0: _27 };
(*_5) = [_38,(*_29),_38,_38,_38,_38,(*_29)];
(*_5) = [_38,_38,_38,_38,_38,_38,_38];
_27 = !Field::<i16>(Variant((*_36), 1), 0);
place!(Field::<i16>(Variant((*_36), 1), 0)) = _22 as i16;
(*_5) = [(*_29),_38,_38,(*_29),_38,_38,_38];
(*_36) = Adt20::Variant2 { fld0: (-1314943363_i32) };
_51 = (*_30) as i16;
Call(place!(Field::<i32>(Variant((*_36), 2), 0)) = core::intrinsics::transmute((*_31)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_12 = core::ptr::addr_of!((*_36));
(*_29) = !_38;
_10 = [_2,_2,_2];
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_12) = Adt20::Variant2 { fld0: 1781792179_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-1292136137_i32) ^ (-1382106989_i32);
Call((*_31) = core::intrinsics::transmute(Field::<i32>(Variant((*_36), 2), 0)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_36) = Adt20::Variant0 { fld0: (-90767265703796257837976275759571955018_i128),fld1: '\u{5638d}',fld2: 210_u8,fld3: _23,fld4: 1799780989_i32 };
place!(Field::<u8>(Variant((*_36), 0), 2)) = !107_u8;
place!(Field::<i128>(Variant((*_36), 0), 0)) = (-5869593402730763062533426894206690864_i128) * (-117408861798034940795566674196362901276_i128);
place!(Field::<i128>(Variant((*_36), 0), 0)) = (-142748057302567403649145970160311158669_i128) - (-160845671989203154660052812404136962767_i128);
place!(Field::<i32>(Variant((*_36), 0), 4)) = 1327317637_i32;
(*_31) = 368963787_u32;
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{5572e}';
_52 = 4281383825276382003_i64 - 4800163738337203941_i64;
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{3fb32}';
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{d5031}';
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{b580c}';
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),_38,(*_29)];
_35 = _13.0 >= _1;
place!(Field::<u64>(Variant((*_36), 0), 3)) = _1;
(*_36) = Adt20::Variant1 { fld0: _27 };
_51 = Field::<i16>(Variant((*_36), 1), 0) << _23;
(*_29) = !_38;
(*_31) = _52 as u32;
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),_38];
_21 = _13;
(*_5) = [_38,_35,(*_29),(*_29),(*_29),(*_29),(*_29)];
place!(Field::<i16>(Variant((*_36), 1), 0)) = (*_31) as i16;
Call(_50.fld2 = core::intrinsics::transmute(_35), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_21 = _13;
_50.fld5 = (-9227115823093898610272447481705712256_i128) | (-14424284299392161972373606584120385706_i128);
_43 = [_8,_8,_8,_8];
(*_36) = Adt20::Variant0 { fld0: _50.fld5,fld1: '\u{785a8}',fld2: _50.fld2,fld3: _13.0,fld4: 354736966_i32 };
(*_31) = 2573285338_u32 >> Field::<u8>(Variant((*_36), 0), 2);
(*_36) = Adt20::Variant2 { fld0: 1931686848_i32 };
place!(Field::<i32>(Variant((*_36), 2), 0)) = -(-858237679_i32);
(*_5) = [(*_29),_38,_35,(*_29),(*_29),(*_29),(*_29)];
(*_36) = Adt20::Variant1 { fld0: _51 };
_13 = (_21.0, _21.1);
_28 = ['\u{939f5}'];
_32 = _1 & _4;
(*_29) = Field::<i16>(Variant((*_36), 1), 0) >= _51;
(*_36) = Adt20::Variant1 { fld0: _51 };
place!(Field::<i16>(Variant((*_36), 1), 0)) = -_51;
place!(Field::<i16>(Variant((*_36), 1), 0)) = _51;
Goto(bb28)
}
bb28 = {
(*_31) = _50.fld2 as u32;
(*_36) = Adt20::Variant1 { fld0: _51 };
_32 = _4 << (*_31);
(*_29) = _24 > (*_31);
_13.1 = _21.1;
(*_31) = 2760566436_u32;
place!(Field::<i16>(Variant((*_36), 1), 0)) = _51 ^ _51;
(*_36) = Adt20::Variant2 { fld0: 753116852_i32 };
(*_29) = _4 != _32;
place!(Field::<i32>(Variant((*_36), 2), 0)) = 1606213879_i32 >> _23;
(*_31) = 1799024823_u32;
(*_31) = 1085726319_u32 >> Field::<i32>(Variant((*_36), 2), 0);
(*_5) = [(*_29),_35,(*_29),(*_29),(*_29),(*_29),_35];
(*_29) = !_35;
_18 = _3;
place!(Field::<i32>(Variant((*_36), 2), 0)) = _22 as i32;
(*_29) = _35;
(*_36) = Adt20::Variant0 { fld0: _50.fld5,fld1: '\u{166c3}',fld2: _50.fld2,fld3: _32,fld4: 684736598_i32 };
_67.fld2 = Field::<u8>(Variant((*_36), 0), 2);
Goto(bb29)
}
bb29 = {
place!(Field::<u8>(Variant((*_36), 0), 2)) = _50.fld2;
_40 = _2 as f32;
(*_5) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
_24 = 2586928399_u32 & 3491102925_u32;
place!(Field::<u64>(Variant((*_36), 0), 3)) = !_4;
_31 = core::ptr::addr_of!((*_31));
place!(Field::<u64>(Variant((*_36), 0), 3)) = _32;
(*_36) = Adt20::Variant2 { fld0: 1880549399_i32 };
(*_36) = Adt20::Variant2 { fld0: (-1512541377_i32) };
place!(Field::<i32>(Variant((*_36), 2), 0)) = (-1082645227_i32) & (-1972941754_i32);
(*_36) = Adt20::Variant0 { fld0: _50.fld5,fld1: '\u{13ac}',fld2: _67.fld2,fld3: _32,fld4: 475663040_i32 };
place!(Field::<i32>(Variant((*_36), 0), 4)) = 460013425_i32 ^ 1241553224_i32;
place!(Field::<char>(Variant((*_36), 0), 1)) = '\u{24ef1}';
_46 = Field::<char>(Variant((*_36), 0), 1);
place!(Field::<u8>(Variant((*_36), 0), 2)) = !_50.fld2;
place!(Field::<i128>(Variant((*_36), 0), 0)) = !_50.fld5;
place!(Field::<i128>(Variant((*_36), 0), 0)) = _50.fld5 & _50.fld5;
(*_31) = !4126583009_u32;
(*_29) = !_35;
(*_31) = 929306214_u32 + 2185426821_u32;
(*_36) = Adt20::Variant0 { fld0: _50.fld5,fld1: _46,fld2: _50.fld2,fld3: _1,fld4: 1171994483_i32 };
place!(Field::<u64>(Variant((*_36), 0), 3)) = _23 & _1;
(*_31) = 1269512373_u32 + 3914126479_u32;
_44 = core::ptr::addr_of!(_3);
(*_31) = 1347596433_u32 & 913159736_u32;
_73 = [Field::<char>(Variant((*_36), 0), 1)];
Goto(bb30)
}
bb30 = {
_24 = 1599846551_u32;
_41 = core::ptr::addr_of!((*_5));
place!(Field::<i32>(Variant((*_36), 0), 4)) = (*_44) as i32;
_69 = core::ptr::addr_of!(_64);
_67.fld5 = Field::<i128>(Variant((*_36), 0), 0) * Field::<i128>(Variant((*_36), 0), 0);
_37 = Field::<char>(Variant((*_36), 0), 1);
place!(Field::<i32>(Variant((*_36), 0), 4)) = (-1282594193_i32) | (-160336786_i32);
_54 = Field::<char>(Variant((*_36), 0), 1);
place!(Field::<u8>(Variant((*_36), 0), 2)) = Field::<i128>(Variant((*_36), 0), 0) as u8;
place!(Field::<u64>(Variant((*_36), 0), 3)) = _67.fld2 as u64;
Goto(bb31)
}
bb31 = {
_2 = -(-91_i8);
_50.fld4 = _51 * _51;
match (*_31) {
0 => bb30,
1 => bb18,
2 => bb13,
3 => bb29,
4 => bb7,
5 => bb32,
1599846551 => bb34,
_ => bb33
}
}
bb32 = {
(*_12) = Adt20::Variant0 { fld0: 36678714917071768566898685149761954833_i128,fld1: '\u{d9d44}',fld2: 145_u8,fld3: _4,fld4: 364578093_i32 };
place!(Field::<i32>(Variant((*_12), 0), 4)) = -(-225594637_i32);
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-63169192663984423268496462855778727981_i128) * (-123971365905385377840328618739064936357_i128);
place!(Field::<char>(Variant((*_12), 0), 1)) = '\u{6e08a}';
place!(Field::<i128>(Variant((*_12), 0), 0)) = (-98695208909239532041372550220952826061_i128) - (-30381507111443791073854137568834610660_i128);
(*_12) = Adt20::Variant2 { fld0: 1158381875_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = 963407579_i32 + (-291675805_i32);
(*_12) = Adt20::Variant0 { fld0: 61727658514429235342792662495345566797_i128,fld1: '\u{c3b3f}',fld2: 183_u8,fld3: _1,fld4: (-1624984404_i32) };
place!(Field::<i32>(Variant((*_12), 0), 4)) = 882844700_i32;
_1 = !Field::<u64>(Variant((*_12), 0), 3);
(*_12) = Adt20::Variant1 { fld0: (-21564_i16) };
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-5175_i16);
RET = _9 as isize;
_8 = 74219756479420089_usize | 2680368508466849577_usize;
place!(Field::<i16>(Variant((*_12), 1), 0)) = (-29058_i16) * 13003_i16;
_6.3 = Adt20::Variant2 { fld0: 205728674_i32 };
place!(Field::<i32>(Variant((*_12), 2), 0)) = (-880819509_i32);
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1069792866_i32;
(*_12) = Adt20::Variant0 { fld0: (-96491365523238521951913456271339468054_i128),fld1: '\u{b4a2f}',fld2: 126_u8,fld3: _1,fld4: (-1783712851_i32) };
place!(Field::<i128>(Variant((*_12), 0), 0)) = _6.2 as i128;
place!(Field::<u64>(Variant((*_12), 0), 3)) = !_4;
place!(Field::<u8>(Variant((*_12), 0), 2)) = !17_u8;
Goto(bb9)
}
bb33 = {
_2 = _6.2 * _6.2;
place!(Field::<i32>(Variant(_6.3, 2), 0)) = 1844272698_i32 >> RET;
_1 = Field::<i32>(Variant(_6.3, 2), 0) as u64;
_9 = true;
_10 = [_2,_2,_2];
_6.2 = _2;
Goto(bb2)
}
bb34 = {
_32 = _1 >> _51;
place!(Field::<i128>(Variant((*_36), 0), 0)) = -_67.fld5;
_60 = core::ptr::addr_of_mut!((*_44));
(*_29) = _35;
_26 = &place!(Field::<i32>(Variant((*_36), 0), 4));
place!(Field::<i32>(Variant((*_36), 0), 4)) = (-688927760_i32) >> _23;
_56 = Field::<i32>(Variant((*_36), 0), 4) != Field::<i32>(Variant((*_36), 0), 4);
place!(Field::<i128>(Variant((*_36), 0), 0)) = _67.fld5 - _67.fld5;
(*_44) = Field::<i128>(Variant((*_36), 0), 0) as isize;
(*_31) = 278291832_u32 + 638499447_u32;
_43 = _39;
place!(Field::<u8>(Variant((*_36), 0), 2)) = _67.fld2 | _50.fld2;
_43 = _39;
place!(Field::<i32>(Variant((*_36), 0), 4)) = 1875764587_i32 ^ (-1216357145_i32);
(*_36) = Adt20::Variant2 { fld0: (-424028252_i32) };
_67.fld1 = 283282551785774250646957195267617360504_u128 & 327876045222860332584266104386099634744_u128;
(*_29) = _56 >= _35;
_44 = core::ptr::addr_of!(_53);
Goto(bb35)
}
bb35 = {
_50.fld1 = _67.fld1;
(*_44) = _34 as isize;
(*_31) = !765405792_u32;
(*_29) = (*_31) >= (*_31);
_77.1 = Adt20::Variant1 { fld0: _51 };
(*_29) = !_56;
(*_29) = !_56;
_50.fld5 = _67.fld5 * _67.fld5;
(*_44) = _18 + _3;
(*_29) = _56;
_54 = _46;
Goto(bb36)
}
bb36 = {
_55 = [_50.fld1,_50.fld1,_50.fld1,_50.fld1,_50.fld1,_67.fld1];
_79 = _38;
Goto(bb37)
}
bb37 = {
(*_41) = [(*_29),(*_29),_35,(*_29),(*_29),(*_29),(*_29)];
(*_5) = [_79,_35,(*_29),(*_29),_56,_79,(*_29)];
_59 = _53 >> _50.fld2;
place!(Field::<i32>(Variant((*_36), 2), 0)) = 1040063218_i32;
(*_36) = Adt20::Variant2 { fld0: (-2093179780_i32) };
_45 = (*_44) - _59;
(*_36) = Adt20::Variant2 { fld0: (-1418788340_i32) };
(*_5) = [(*_29),(*_29),_38,(*_29),(*_29),(*_29),(*_29)];
_36 = &mut _77.1;
_84 = !_45;
place!(Field::<i16>(Variant((*_36), 1), 0)) = !_51;
(*_29) = !_79;
(*_31) = 3949692657_u32 | 3340517061_u32;
place!(Field::<i16>(Variant((*_36), 1), 0)) = _50.fld4 & _51;
_66 = _84 | (*_44);
_80 = Field::<i16>(Variant((*_36), 1), 0) >= Field::<i16>(Variant((*_36), 1), 0);
_78 = [(*_31),(*_31),(*_31),(*_31),(*_31),_24,(*_31),(*_31)];
(*_44) = -_3;
_43 = _19;
_13.0 = _4 | _32;
(*_36) = Adt20::Variant1 { fld0: _51 };
_31 = core::ptr::addr_of!((*_31));
Goto(bb38)
}
bb38 = {
(*_44) = _84;
_85 = core::ptr::addr_of_mut!(_59);
(*_29) = _79;
Goto(bb39)
}
bb39 = {
(*_5) = [(*_29),(*_29),_79,(*_29),_79,_56,(*_29)];
_90 = (*_31) << Field::<i16>(Variant((*_36), 1), 0);
_16 = &mut _55;
(*_16) = [_67.fld1,_50.fld1,_50.fld1,_67.fld1,_67.fld1,_50.fld1];
(*_16) = [_50.fld1,_50.fld1,_67.fld1,_50.fld1,_67.fld1,_50.fld1];
_88 = [_90,_90,_90,_90,_90,_90,_24,_90];
_20 = [_67.fld2,_67.fld2,_50.fld2,_67.fld2,_67.fld2,_50.fld2,_67.fld2];
_46 = _54;
_73 = _28;
Goto(bb40)
}
bb40 = {
(*_36) = Adt20::Variant0 { fld0: _67.fld5,fld1: _37,fld2: _67.fld2,fld3: _1,fld4: 1324769997_i32 };
(*_31) = _90;
place!(Field::<u8>(Variant((*_36), 0), 2)) = _22 as u8;
(*_36) = Adt20::Variant2 { fld0: (-1833661064_i32) };
_14 = core::ptr::addr_of!(_83);
place!(Field::<i32>(Variant((*_36), 2), 0)) = (-555849715_i32) >> (*_85);
Goto(bb41)
}
bb41 = {
(*_31) = !_90;
(*_16) = [_50.fld1,_67.fld1,_50.fld1,_50.fld1,_67.fld1,_67.fld1];
(*_29) = Field::<i32>(Variant((*_36), 2), 0) != Field::<i32>(Variant((*_36), 2), 0);
(*_16) = [_67.fld1,_67.fld1,_50.fld1,_67.fld1,_50.fld1,_50.fld1];
_93 = _50.fld1;
_84 = (*_85) * (*_44);
Goto(bb42)
}
bb42 = {
(*_36) = Adt20::Variant1 { fld0: _50.fld4 };
(*_16) = [_93,_93,_50.fld1,_93,_67.fld1,_50.fld1];
_50.fld5 = _67.fld5;
(*_85) = (*_44) * _45;
_48 = _46;
_44 = core::ptr::addr_of!((*_44));
(*_31) = (*_85) as u32;
(*_36) = Adt20::Variant1 { fld0: _50.fld4 };
(*_14) = core::ptr::addr_of_mut!((*_16));
(*_83) = [_93,_67.fld1,_50.fld1,_93,_93,_93];
_59 = 388850910_i32 as isize;
(*_83) = [_93,_93,_67.fld1,_50.fld1,_93,_50.fld1];
(*_16) = [_67.fld1,_93,_93,_50.fld1,_93,_93];
(*_85) = -(*_44);
Call(place!(Field::<i16>(Variant((*_36), 1), 0)) = core::intrinsics::bswap(_50.fld4), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
(*_14) = core::ptr::addr_of_mut!((*_16));
(*_14) = core::ptr::addr_of_mut!((*_16));
(*_29) = (*_44) <= _53;
_38 = (*_29) | (*_29);
Goto(bb44)
}
bb44 = {
_92 = &(*_31);
(*_83) = [_67.fld1,_67.fld1,_67.fld1,_67.fld1,_93,_93];
_82 = _2 as u128;
_74 = (*_31) as f64;
_97.1 = [_67.fld5,_50.fld5];
(*_29) = _56;
Goto(bb45)
}
bb45 = {
(*_16) = [_50.fld1,_93,_93,_93,_50.fld1,_67.fld1];
(*_29) = _79 < _35;
(*_31) = _90 ^ _90;
(*_36) = Adt20::Variant2 { fld0: (-366167317_i32) };
_50.fld4 = _51 * _51;
(*_16) = [_93,_93,_93,_67.fld1,_67.fld1,_93];
_19 = [_8,_8,_8,_8];
(*_44) = (*_85) << (*_31);
(*_16) = [_50.fld1,_50.fld1,_67.fld1,_67.fld1,_50.fld1,_82];
_13 = (_23, _97.1);
_12 = core::ptr::addr_of!((*_36));
(*_31) = _90 | _90;
_57 = &mut _50.fld2;
(*_5) = [(*_29),_38,(*_29),(*_29),(*_29),(*_29),(*_29)];
_94 = &(*_31);
(*_31) = _90 | _90;
_91 = core::ptr::addr_of_mut!((*_44));
Goto(bb46)
}
bb46 = {
(*_12) = Adt20::Variant2 { fld0: 865451275_i32 };
(*_29) = (*_44) >= (*_85);
(*_36) = Adt20::Variant1 { fld0: _51 };
_29 = &mut _80;
(*_14) = core::ptr::addr_of_mut!((*_16));
_67 = Adt19 { fld0: _97.1,fld1: _93,fld2: (*_57),fld3: 1834_u16,fld4: Field::<i16>(Variant((*_36), 1), 0),fld5: 107515166300699623691666876575532960808_i128 };
_99 = Field::<i16>(Variant((*_36), 1), 0) as u32;
_12 = core::ptr::addr_of!(_95.fld1);
(*_14) = core::ptr::addr_of_mut!((*_16));
_54 = _37;
(*_91) = (*_85) + (*_85);
(*_12) = (*_36);
(*_91) = _46 as isize;
(*_83) = [_93,_93,_67.fld1,_93,_82,_93];
_99 = !_90;
_108 = _67.fld1 ^ _82;
(*_14) = core::ptr::addr_of_mut!((*_83));
(*_44) = (*_85);
(*_16) = [_93,_67.fld1,_108,_108,_67.fld1,_93];
_21.0 = _32 & _32;
place!(Field::<i16>(Variant((*_36), 1), 0)) = Field::<i16>(Variant((*_12), 1), 0) ^ Field::<i16>(Variant((*_12), 1), 0);
(*_5) = [(*_29),(*_29),(*_29),_38,(*_29),(*_29),_38];
(*_29) = _79;
_32 = _1 | _1;
_7 = &mut (*_16);
(*_36) = (*_12);
Goto(bb47)
}
bb47 = {
(*_7) = [_67.fld1,_108,_108,_108,_67.fld1,_93];
(*_36) = (*_12);
(*_85) = !(*_44);
_97.0 = _32 - _21.0;
(*_44) = !(*_85);
(*_31) = _90;
Call((*_85) = core::intrinsics::bswap(_66), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
(*_85) = (*_44) & (*_44);
_103 = [_2,_2,_2,_2,_2,_2,_2,_2];
(*_7) = [_93,_93,_82,_108,_108,_67.fld1];
_48 = _54;
(*_44) = _67.fld5 as isize;
place!(Field::<i16>(Variant((*_36), 1), 0)) = Field::<i16>(Variant((*_12), 1), 0) | Field::<i16>(Variant((*_12), 1), 0);
(*_29) = _38;
(*_31) = !_90;
_75 = _56;
_88 = [(*_31),(*_31),(*_31),(*_31),(*_31),_90,(*_31),(*_31)];
(*_14) = core::ptr::addr_of_mut!((*_7));
_105 = core::ptr::addr_of!((*_5));
(*_36) = (*_12);
(*_57) = _67.fld2 - _67.fld2;
(*_36) = Adt20::Variant1 { fld0: Field::<i16>(Variant((*_12), 1), 0) };
(*_44) = -(*_85);
(*_14) = core::ptr::addr_of_mut!((*_7));
(*_29) = _79;
Goto(bb49)
}
bb49 = {
(*_12) = Adt20::Variant0 { fld0: _67.fld5,fld1: _48,fld2: (*_57),fld3: _21.0,fld4: (-993088957_i32) };
_95.fld2 = !_67.fld2;
(*_14) = core::ptr::addr_of_mut!((*_7));
place!(Field::<i16>(Variant((*_36), 1), 0)) = _67.fld4 | _51;
_13 = (_32, _21.1);
(*_85) = (*_44);
place!(Field::<u64>(Variant((*_12), 0), 3)) = !_97.0;
_74 = _52 as f64;
(*_36) = Adt20::Variant0 { fld0: _67.fld5,fld1: Field::<char>(Variant((*_12), 0), 1),fld2: Field::<u8>(Variant((*_12), 0), 2),fld3: _23,fld4: (-1901060661_i32) };
place!(Field::<i128>(Variant((*_12), 0), 0)) = Field::<i128>(Variant((*_36), 0), 0) & Field::<i128>(Variant((*_36), 0), 0);
(*_29) = !_79;
Goto(bb50)
}
bb50 = {
Call(_122 = dump_var(Move(_19), Move(_59), Move(_38), Move(_28)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_122 = dump_var(Move(_8), Move(_43), Move(_45), Move(_73)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_122 = dump_var(Move(_79), Move(_46), Move(_24), Move(_99)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_122 = dump_var(Move(_55), Move(_37), Move(_23), Move(_27)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_122 = dump_var(Move(_84), Move(_3), Move(_103), Move(_35)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_122 = dump_var(Move(_108), Move(_10), Move(_88), Move(_39)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const Adt20,mut _2: usize,mut _3: isize) -> Adt20 {
mir! {
type RET = Adt20;
let _4: [usize; 6];
let _5: isize;
let _6: f32;
let _7: f64;
let _8: isize;
let _9: isize;
let _10: isize;
let _11: &'static mut &'static u32;
let _12: (&'static mut bool, (Adt19,), *mut u128, &'static [i8; 3]);
let _13: &'static [u16; 4];
let _14: &'static &'static i64;
let _15: char;
let _16: &'static Adt27;
let _17: &'static mut (u64, [i128; 2]);
let _18: [usize; 4];
let _19: f32;
let _20: isize;
let _21: [usize; 6];
let _22: u8;
let _23: Adt19;
let _24: f64;
let _25: [i32; 3];
let _26: &'static (Adt19,);
let _27: i16;
let _28: u64;
let _29: f64;
let _30: usize;
let _31: (&'static i32, *mut u128, i8, Adt20);
let _32: u128;
let _33: bool;
let _34: *const Adt19;
let _35: *mut u128;
let _36: [char; 1];
let _37: isize;
let _38: [usize; 4];
let _39: [u128; 6];
let _40: Adt19;
let _41: ();
let _42: ();
{
_3 = -(-9223372036854775808_isize);
RET = Adt20::Variant0 { fld0: 99289907099421219627995663702506383888_i128,fld1: '\u{dd63c}',fld2: 126_u8,fld3: 3792106101099743803_u64,fld4: 218596800_i32 };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{c93dc}';
place!(Field::<i128>(Variant(RET, 0), 0)) = -(-52897488482844503622148501315522910699_i128);
RET = Adt20::Variant2 { fld0: 392182443_i32 };
_3 = 50_isize;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-6178117037229165451_i64) as i32;
_1 = core::ptr::addr_of!(RET);
(*_1) = Adt20::Variant2 { fld0: (-891989790_i32) };
(*_1) = Adt20::Variant1 { fld0: (-30881_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = (-15398_i16) >> _3;
place!(Field::<i16>(Variant((*_1), 1), 0)) = 14537_i16 - 3479_i16;
_1 = core::ptr::addr_of!((*_1));
(*_1) = Adt20::Variant2 { fld0: 40740134_i32 };
(*_1) = Adt20::Variant1 { fld0: 1759_i16 };
place!(Field::<i16>(Variant((*_1), 1), 0)) = -(-4435_i16);
(*_1) = Adt20::Variant1 { fld0: (-30067_i16) };
_4 = [_2,_2,_2,_2,_2,_2];
place!(Field::<i16>(Variant((*_1), 1), 0)) = (-7049_i16) | 10864_i16;
(*_1) = Adt20::Variant1 { fld0: (-8164_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = 3758_i16;
(*_1) = Adt20::Variant1 { fld0: (-17903_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = -20615_i16;
(*_1) = Adt20::Variant1 { fld0: 19413_i16 };
place!(Field::<i16>(Variant((*_1), 1), 0)) = 13211_i16 & 32481_i16;
place!(Field::<i16>(Variant((*_1), 1), 0)) = 307364077_i32 as i16;
Goto(bb1)
}
bb1 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
Goto(bb2)
}
bb2 = {
place!(Field::<i128>(Variant((*_1), 0), 0)) = (-63154567969212623522240879452213693631_i128);
place!(Field::<u8>(Variant((*_1), 0), 2)) = !220_u8;
(*_1) = Adt20::Variant1 { fld0: (-701_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = 3184607253_u32 as i16;
place!(Field::<i16>(Variant((*_1), 1), 0)) = 235841390806582270883142801395095723858_u128 as i16;
(*_1) = Adt20::Variant2 { fld0: 1380090817_i32 };
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
50 => bb8,
_ => bb7
}
}
bb3 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
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
(*_1) = Adt20::Variant0 { fld0: (-122054771426344388222616570935027631122_i128),fld1: '\u{fdaae}',fld2: 129_u8,fld3: 4681258667257009961_u64,fld4: (-1936677372_i32) };
place!(Field::<u8>(Variant((*_1), 0), 2)) = 98_u8 & 72_u8;
_3 = 50525_u16 as isize;
place!(Field::<char>(Variant((*_1), 0), 1)) = '\u{4c8c7}';
place!(Field::<u8>(Variant((*_1), 0), 2)) = 1478414080_u32 as u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 161_u8 >> _9;
(*_1) = Adt20::Variant1 { fld0: (-26134_i16) };
(*_1) = Adt20::Variant1 { fld0: (-20015_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = (-71_i8) as i16;
_12.1.0.fld2 = '\u{6d5ba}' as u8;
_10 = -_3;
(*_1) = Adt20::Variant2 { fld0: (-1625944528_i32) };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 1736028371_i32 - 203858106_i32;
(*_1) = Adt20::Variant0 { fld0: 17277653861732153877177210977986614833_i128,fld1: '\u{1b0d}',fld2: _12.1.0.fld2,fld3: 9465805005988356539_u64,fld4: (-1144882583_i32) };
place!(Field::<char>(Variant((*_1), 0), 1)) = '\u{f916c}';
place!(Field::<u64>(Variant((*_1), 0), 3)) = !11894947171934011835_u64;
(*_1) = Adt20::Variant2 { fld0: 909028974_i32 };
_5 = _9 * _9;
_7 = 29_i8 as f64;
(*_1) = Adt20::Variant0 { fld0: (-41135571364639440477569525533009768233_i128),fld1: '\u{1740e}',fld2: _12.1.0.fld2,fld3: 11721922098713378108_u64,fld4: 321421162_i32 };
(*_1) = Adt20::Variant0 { fld0: 153406658380468025875389574444609602057_i128,fld1: '\u{36aaf}',fld2: _12.1.0.fld2,fld3: 18086912475818540037_u64,fld4: (-217102690_i32) };
place!(Field::<i128>(Variant((*_1), 0), 0)) = !(-58678414880235937774374608613357547344_i128);
place!(Field::<i32>(Variant((*_1), 0), 4)) = Field::<i128>(Variant((*_1), 0), 0) as i32;
Goto(bb9)
}
bb9 = {
place!(Field::<u8>(Variant((*_1), 0), 2)) = _12.1.0.fld2;
place!(Field::<char>(Variant((*_1), 0), 1)) = '\u{7810f}';
place!(Field::<i32>(Variant((*_1), 0), 4)) = !(-2105244538_i32);
_10 = !_9;
place!(Field::<u8>(Variant((*_1), 0), 2)) = _12.1.0.fld2 * _12.1.0.fld2;
place!(Field::<u64>(Variant((*_1), 0), 3)) = !8648377563622306618_u64;
_12.1.0.fld3 = !7368_u16;
place!(Field::<u8>(Variant((*_1), 0), 2)) = !_12.1.0.fld2;
place!(Field::<i32>(Variant((*_1), 0), 4)) = 715447984_i32 << Field::<u8>(Variant((*_1), 0), 2);
place!(Field::<u64>(Variant(RET, 0), 3)) = 2463687558335796525_u64 << Field::<u8>(Variant((*_1), 0), 2);
place!(Field::<i128>(Variant((*_1), 0), 0)) = _7 as i128;
_10 = Field::<char>(Variant((*_1), 0), 1) as isize;
place!(Field::<u8>(Variant((*_1), 0), 2)) = _12.1.0.fld2 << Field::<i32>(Variant((*_1), 0), 4);
_8 = _5 >> Field::<u8>(Variant((*_1), 0), 2);
place!(Field::<char>(Variant((*_1), 0), 1)) = '\u{9d004}';
place!(Field::<i32>(Variant((*_1), 0), 4)) = 885347127_i32;
_1 = core::ptr::addr_of!((*_1));
place!(Field::<u64>(Variant((*_1), 0), 3)) = 11280206532477790065_u64 * 8348896108769346881_u64;
(*_1) = Adt20::Variant0 { fld0: (-120323880358556684762421187424586564780_i128),fld1: '\u{967c}',fld2: _12.1.0.fld2,fld3: 7918660820035397513_u64,fld4: 608973114_i32 };
_12.1.0.fld4 = (-21136_i16) >> _5;
(*_1) = Adt20::Variant0 { fld0: 8803114842577154687122107647918757622_i128,fld1: '\u{f3aba}',fld2: _12.1.0.fld2,fld3: 8239921692551569375_u64,fld4: (-1025616619_i32) };
place!(Field::<i32>(Variant((*_1), 0), 4)) = !324375044_i32;
place!(Field::<char>(Variant((*_1), 0), 1)) = '\u{7e3ea}';
Call(place!(Field::<u8>(Variant((*_1), 0), 2)) = fn8(Field::<char>(Variant((*_1), 0), 1), _2, _8, Move(_12.2), Move(_1), Field::<char>(Variant((*_1), 0), 1), Field::<i32>(Variant((*_1), 0), 4)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = !_8;
Goto(bb11)
}
bb11 = {
_12.1.0.fld1 = Field::<char>(Variant(RET, 0), 1) as u128;
_7 = _2 as f64;
_15 = Field::<char>(Variant(RET, 0), 1);
_12.1.0.fld0 = [43663119179705418360224846922165551098_i128,(-126395418468627519661512434879899852243_i128)];
_18 = [_2,_2,_2,_2];
_5 = _9 << _8;
Goto(bb12)
}
bb12 = {
place!(Field::<u64>(Variant(RET, 0), 3)) = (-10_i8) as u64;
_8 = _3 << _3;
_12.1.0.fld0 = [83694995389146261343526220644929539023_i128,50347250704377647081110318052641201117_i128];
Goto(bb13)
}
bb13 = {
_12.1.0.fld2 = Field::<u8>(Variant(RET, 0), 2) - Field::<u8>(Variant(RET, 0), 2);
_12.1.0.fld5 = !105440640021062526142456225139928600805_i128;
place!(Field::<i32>(Variant(RET, 0), 4)) = (-7833513562826207250_i64) as i32;
_12.1.0.fld3 = !22924_u16;
Goto(bb14)
}
bb14 = {
_5 = _3 * _3;
_21 = [_2,_2,_2,_2,_2,_2];
_12.1.0.fld1 = 312685966257023147942581634620372185156_u128 & 241183091915856912813085776478292810480_u128;
Goto(bb15)
}
bb15 = {
_12.1.0.fld3 = 25101_u16;
_23.fld3 = !_12.1.0.fld3;
_23.fld4 = _7 as i16;
_23 = Adt19 { fld0: _12.1.0.fld0,fld1: _12.1.0.fld1,fld2: _12.1.0.fld2,fld3: _12.1.0.fld3,fld4: _12.1.0.fld4,fld5: _12.1.0.fld5 };
_20 = _10 | _5;
match _23.fld3 {
0 => bb5,
1 => bb16,
2 => bb17,
25101 => bb19,
_ => bb18
}
}
bb16 = {
_3 = !_8;
Goto(bb11)
}
bb17 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
Goto(bb2)
}
bb18 = {
Return()
}
bb19 = {
_12.1.0.fld3 = _23.fld3 * _23.fld3;
_22 = Field::<u8>(Variant(RET, 0), 2) | Field::<u8>(Variant(RET, 0), 2);
place!(Field::<i128>(Variant(RET, 0), 0)) = _12.1.0.fld5;
_23.fld2 = _12.1.0.fld2 >> Field::<u8>(Variant(RET, 0), 2);
_21 = [_2,_2,_2,_2,_2,_2];
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
_23.fld2 = Field::<u64>(Variant(RET, 0), 3) as u8;
match _23.fld3 {
25101 => bb20,
_ => bb18
}
}
bb20 = {
_23.fld0 = [_23.fld5,Field::<i128>(Variant(RET, 0), 0)];
_4 = _21;
_10 = _8 & _5;
Call(_6 = core::intrinsics::transmute(_15), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_19 = _6 + _6;
place!(Field::<i128>(Variant(RET, 0), 0)) = _23.fld5 + _23.fld5;
_12.1.0.fld4 = Field::<i32>(Variant(RET, 0), 4) as i16;
_12.1.0.fld5 = -Field::<i128>(Variant(RET, 0), 0);
_3 = _10;
_7 = 4122899815_u32 as f64;
match _23.fld3 {
0 => bb16,
1 => bb2,
2 => bb3,
3 => bb20,
4 => bb17,
5 => bb12,
6 => bb18,
25101 => bb22,
_ => bb8
}
}
bb22 = {
_5 = _10 * _10;
_5 = 4563073789502909650_i64 as isize;
_20 = _7 as isize;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
place!(Field::<i32>(Variant(RET, 0), 4)) = Field::<u64>(Variant(RET, 0), 3) as i32;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
_23.fld4 = !_12.1.0.fld4;
_28 = Field::<i32>(Variant(RET, 0), 4) as u64;
_2 = 1_usize ^ 14550191461403243311_usize;
_29 = _7 + _7;
_27 = _12.1.0.fld4 ^ _12.1.0.fld4;
_32 = !_23.fld1;
_12.1.0.fld5 = _23.fld5 >> _3;
_12.2 = core::ptr::addr_of_mut!(_23.fld1);
Goto(bb23)
}
bb23 = {
_1 = core::ptr::addr_of!(_31.3);
_12.1.0.fld4 = _23.fld4 - _27;
(*_1) = Adt20::Variant1 { fld0: _27 };
place!(Field::<i16>(Variant((*_1), 1), 0)) = _23.fld4 ^ _12.1.0.fld4;
place!(Field::<i16>(Variant((*_1), 1), 0)) = _12.1.0.fld4;
_22 = _23.fld2;
_25 = [Field::<i32>(Variant(RET, 0), 4),Field::<i32>(Variant(RET, 0), 4),Field::<i32>(Variant(RET, 0), 4)];
_24 = -_29;
place!(Field::<i16>(Variant((*_1), 1), 0)) = Field::<i32>(Variant(RET, 0), 4) as i16;
(*_1) = Adt20::Variant0 { fld0: _12.1.0.fld5,fld1: _15,fld2: _12.1.0.fld2,fld3: Field::<u64>(Variant(RET, 0), 3),fld4: Field::<i32>(Variant(RET, 0), 4) };
_23.fld4 = _12.1.0.fld4 + _12.1.0.fld4;
place!(Field::<u8>(Variant((*_1), 0), 2)) = _23.fld2 + _12.1.0.fld2;
place!(Field::<u8>(Variant((*_1), 0), 2)) = _12.1.0.fld5 as u8;
place!(Field::<char>(Variant((*_1), 0), 1)) = Field::<char>(Variant(RET, 0), 1);
(*_1) = RET;
place!(Field::<u8>(Variant((*_1), 0), 2)) = _23.fld2;
Goto(bb24)
}
bb24 = {
place!(Field::<i128>(Variant((*_1), 0), 0)) = _12.1.0.fld4 as i128;
place!(Field::<char>(Variant((*_1), 0), 1)) = _15;
(*_1) = Adt20::Variant1 { fld0: _23.fld4 };
_2 = 2_usize << _12.1.0.fld5;
_26 = &_12.1;
place!(Field::<i16>(Variant((*_1), 1), 0)) = (*_26).0.fld5 as i16;
_12.1.0.fld0 = _23.fld0;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
place!(Field::<i16>(Variant((*_1), 1), 0)) = _10 as i16;
match _23.fld3 {
0 => bb13,
1 => bb8,
2 => bb22,
3 => bb7,
25101 => bb26,
_ => bb25
}
}
bb25 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
Goto(bb2)
}
bb26 = {
_30 = Field::<i32>(Variant(RET, 0), 4) as usize;
Goto(bb27)
}
bb27 = {
_20 = _10 - _5;
place!(Field::<i16>(Variant((*_1), 1), 0)) = (*_26).0.fld4 << (*_26).0.fld5;
(*_1) = Adt20::Variant1 { fld0: (*_26).0.fld4 };
_31.2 = 36_i8;
_12.1.0.fld0 = _23.fld0;
(*_1) = Adt20::Variant2 { fld0: Field::<i32>(Variant(RET, 0), 4) };
_12.1.0.fld1 = _23.fld1;
_23.fld5 = Field::<i32>(Variant((*_1), 2), 0) as i128;
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = Field::<i32>(Variant(RET, 0), 4);
_23.fld3 = _12.1.0.fld3 * (*_26).0.fld3;
(*_1) = Adt20::Variant0 { fld0: (*_26).0.fld5,fld1: Field::<char>(Variant(RET, 0), 1),fld2: (*_26).0.fld2,fld3: _28,fld4: Field::<i32>(Variant(RET, 0), 4) };
place!(Field::<i128>(Variant((*_1), 0), 0)) = _12.1.0.fld5 - (*_26).0.fld5;
place!(Field::<u8>(Variant((*_1), 0), 2)) = (*_26).0.fld2 << Field::<i128>(Variant((*_1), 0), 0);
place!(Field::<i32>(Variant((*_1), 0), 4)) = 1353025966_u32 as i32;
place!(Field::<i128>(Variant((*_1), 0), 0)) = (*_26).0.fld5;
_27 = (*_26).0.fld4;
(*_1) = Adt20::Variant2 { fld0: Field::<i32>(Variant(RET, 0), 4) };
place!(Field::<i32>(Variant((*_1), 2), 0)) = Field::<i32>(Variant(RET, 0), 4);
_12.1.0.fld2 = _30 as u8;
_23.fld1 = _23.fld2 as u128;
_10 = -_3;
(*_1) = Adt20::Variant2 { fld0: Field::<i32>(Variant(RET, 0), 4) };
match _31.2 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
36 => bb35,
_ => bb34
}
}
bb28 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
Goto(bb2)
}
bb29 = {
(*_1) = Adt20::Variant2 { fld0: 157310927_i32 };
_9 = _2 as isize;
(*_1) = Adt20::Variant2 { fld0: 400580020_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = 7464444598816867906_i64 as i32;
_9 = !_3;
(*_1) = Adt20::Variant2 { fld0: 89423755_i32 };
place!(Field::<i32>(Variant((*_1), 2), 0)) = (-2090033399_i32);
_12.2 = core::ptr::addr_of_mut!(_12.1.0.fld1);
place!(Field::<i32>(Variant((*_1), 2), 0)) = false as i32;
RET = Adt20::Variant1 { fld0: (-31654_i16) };
(*_1) = Adt20::Variant2 { fld0: (-56900381_i32) };
(*_1) = Adt20::Variant0 { fld0: 52192796783489340455233904219088830565_i128,fld1: '\u{28cf7}',fld2: 146_u8,fld3: 14594535055072042119_u64,fld4: 28142981_i32 };
place!(Field::<u8>(Variant((*_1), 0), 2)) = !178_u8;
place!(Field::<u8>(Variant((*_1), 0), 2)) = 28_u8 << _2;
Goto(bb2)
}
bb30 = {
place!(Field::<i128>(Variant((*_1), 0), 0)) = _12.1.0.fld4 as i128;
place!(Field::<char>(Variant((*_1), 0), 1)) = _15;
(*_1) = Adt20::Variant1 { fld0: _23.fld4 };
_2 = 2_usize << _12.1.0.fld5;
_26 = &_12.1;
place!(Field::<i16>(Variant((*_1), 1), 0)) = (*_26).0.fld5 as i16;
_12.1.0.fld0 = _23.fld0;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
place!(Field::<i16>(Variant((*_1), 1), 0)) = _10 as i16;
match _23.fld3 {
0 => bb13,
1 => bb8,
2 => bb22,
3 => bb7,
25101 => bb26,
_ => bb25
}
}
bb31 = {
place!(Field::<i128>(Variant((*_1), 0), 0)) = (-63154567969212623522240879452213693631_i128);
place!(Field::<u8>(Variant((*_1), 0), 2)) = !220_u8;
(*_1) = Adt20::Variant1 { fld0: (-701_i16) };
place!(Field::<i16>(Variant((*_1), 1), 0)) = 3184607253_u32 as i16;
place!(Field::<i16>(Variant((*_1), 1), 0)) = 235841390806582270883142801395095723858_u128 as i16;
(*_1) = Adt20::Variant2 { fld0: 1380090817_i32 };
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
50 => bb8,
_ => bb7
}
}
bb32 = {
_5 = _10 * _10;
_5 = 4563073789502909650_i64 as isize;
_20 = _7 as isize;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
place!(Field::<i32>(Variant(RET, 0), 4)) = Field::<u64>(Variant(RET, 0), 3) as i32;
place!(Field::<char>(Variant(RET, 0), 1)) = _15;
_23.fld4 = !_12.1.0.fld4;
_28 = Field::<i32>(Variant(RET, 0), 4) as u64;
_2 = 1_usize ^ 14550191461403243311_usize;
_29 = _7 + _7;
_27 = _12.1.0.fld4 ^ _12.1.0.fld4;
_32 = !_23.fld1;
_12.1.0.fld5 = _23.fld5 >> _3;
_12.2 = core::ptr::addr_of_mut!(_23.fld1);
Goto(bb23)
}
bb33 = {
Return()
}
bb34 = {
_23.fld0 = [_23.fld5,Field::<i128>(Variant(RET, 0), 0)];
_4 = _21;
_10 = _8 & _5;
Call(_6 = core::intrinsics::transmute(_15), ReturnTo(bb21), UnwindUnreachable())
}
bb35 = {
_23.fld5 = Field::<i32>(Variant((*_1), 2), 0) as i128;
place!(Field::<i32>(Variant(RET, 0), 4)) = _2 as i32;
_21 = _4;
(*_1) = Adt20::Variant1 { fld0: (*_26).0.fld4 };
_33 = true;
_32 = !_23.fld1;
place!(Field::<i16>(Variant((*_1), 1), 0)) = (*_26).0.fld4 & (*_26).0.fld4;
_23.fld4 = (*_26).0.fld4 & Field::<i16>(Variant((*_1), 1), 0);
place!(Field::<i16>(Variant((*_1), 1), 0)) = (*_26).0.fld4 - (*_26).0.fld4;
_10 = _7 as isize;
(*_1) = Adt20::Variant2 { fld0: Field::<i32>(Variant(RET, 0), 4) };
_31.0 = &place!(Field::<i32>(Variant((*_1), 2), 0));
place!(Field::<i32>(Variant((*_1), 2), 0)) = Field::<i32>(Variant(RET, 0), 4);
_1 = core::ptr::addr_of!((*_1));
_24 = Field::<u8>(Variant(RET, 0), 2) as f64;
_12.1.0 = Adt19 { fld0: _23.fld0,fld1: _32,fld2: Field::<u8>(Variant(RET, 0), 2),fld3: _23.fld3,fld4: _23.fld4,fld5: _23.fld5 };
_40.fld5 = Field::<i128>(Variant(RET, 0), 0);
_24 = _29 + _29;
_4 = [_2,_2,_2,_30,_2,_2];
(*_1) = RET;
place!(Field::<i32>(Variant((*_1), 0), 4)) = Field::<i32>(Variant(RET, 0), 4) | Field::<i32>(Variant(RET, 0), 4);
Goto(bb36)
}
bb36 = {
Call(_41 = dump_var(Move(_15), Move(_21), Move(_4), Move(_25)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_41 = dump_var(Move(_28), Move(_9), Move(_10), Move(_5)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_41 = dump_var(Move(_20), _42, _42, _42), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: char,mut _2: usize,mut _3: isize,mut _4: *mut u128,mut _5: *const Adt20,mut _6: char,mut _7: i32) -> u8 {
mir! {
type RET = u8;
let _8: f32;
let _9: i16;
let _10: &'static mut &'static u32;
let _11: (u64, [i128; 2]);
let _12: char;
let _13: f32;
let _14: [i8; 8];
let _15: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _16: *const u32;
let _17: (*const isize,);
let _18: &'static [u16; 4];
let _19: char;
let _20: *const [i8; 3];
let _21: char;
let _22: i32;
let _23: i32;
let _24: f32;
let _25: *const u32;
let _26: [u8; 7];
let _27: *const Adt27;
let _28: Adt62;
let _29: u64;
let _30: char;
let _31: isize;
let _32: u8;
let _33: Adt31;
let _34: isize;
let _35: Adt76;
let _36: bool;
let _37: *const Adt38;
let _38: *const &'static [i8; 3];
let _39: ([u16; 4], f32);
let _40: Adt76;
let _41: i32;
let _42: Adt31;
let _43: *const isize;
let _44: f32;
let _45: f32;
let _46: u128;
let _47: *mut u128;
let _48: [char; 1];
let _49: (&'static i32, *mut u128, i8, Adt20);
let _50: *const isize;
let _51: *mut f32;
let _52: &'static mut u8;
let _53: &'static [u16; 4];
let _54: f64;
let _55: &'static i32;
let _56: i8;
let _57: Adt20;
let _58: u64;
let _59: u16;
let _60: u8;
let _61: ([char; 1], Adt20);
let _62: (&'static mut bool, (Adt19,), *mut u128, &'static [i8; 3]);
let _63: *const &'static [i8; 3];
let _64: [i32; 3];
let _65: u128;
let _66: i64;
let _67: *const Adt19;
let _68: bool;
let _69: &'static bool;
let _70: &'static [i8; 3];
let _71: Adt62;
let _72: (*const isize,);
let _73: u64;
let _74: Adt20;
let _75: isize;
let _76: u16;
let _77: *mut Adt38;
let _78: &'static mut &'static u32;
let _79: isize;
let _80: Adt20;
let _81: f32;
let _82: isize;
let _83: &'static i64;
let _84: Adt23;
let _85: *mut u128;
let _86: *mut f32;
let _87: isize;
let _88: ();
let _89: ();
{
RET = 178_u8;
_7 = 1947103816_u32 as i32;
RET = 232_u8;
_3 = 9223372036854775807_isize ^ (-1_isize);
Goto(bb1)
}
bb1 = {
_1 = _6;
_7 = (-902497162_i32) + 1154022356_i32;
_8 = 37148_u16 as f32;
_6 = _1;
_3 = 43_isize - 9223372036854775807_isize;
_7 = (-2126608785_i32);
_1 = _6;
_7 = _8 as i32;
_6 = _1;
_8 = _2 as f32;
match RET {
0 => bb2,
1 => bb3,
232 => bb5,
_ => bb4
}
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
_2 = 0_usize << _3;
_3 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_12 = _6;
_11.0 = !5924633145004860842_u64;
_9 = !(-7300_i16);
_11.1 = [(-11258200454970015526161194255445689279_i128),52863100749048736175895551000726870729_i128];
_11.0 = 4314321423750385154_u64 & 3587558798082253240_u64;
match RET {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
232 => bb8,
_ => bb7
}
}
bb6 = {
Return()
}
bb7 = {
_1 = _6;
_7 = (-902497162_i32) + 1154022356_i32;
_8 = 37148_u16 as f32;
_6 = _1;
_3 = 43_isize - 9223372036854775807_isize;
_7 = (-2126608785_i32);
_1 = _6;
_7 = _8 as i32;
_6 = _1;
_8 = _2 as f32;
match RET {
0 => bb2,
1 => bb3,
232 => bb5,
_ => bb4
}
}
bb8 = {
_2 = 8407370163250677906_usize & 7_usize;
_9 = 1869676110_u32 as i16;
_1 = _6;
_9 = 25163_i16 - 23109_i16;
_12 = _1;
_9 = (-30288_i16);
_11.1 = [(-36659454951356511683117008899329153529_i128),(-83788804716741455273299776468542921452_i128)];
_8 = 314678073431949840405732525657437589292_u128 as f32;
_1 = _12;
_7 = _3 as i32;
_2 = 0_usize;
_11.1 = [129523117599685242319632245130735815995_i128,117873388888340084279957817220810300913_i128];
RET = 100_u8 & 68_u8;
_6 = _1;
_14[_2] = 90_i8 + 9_i8;
_12 = _6;
Goto(bb9)
}
bb9 = {
_2 = !11387393420639631877_usize;
_15.2 = _7 as f64;
_17.0 = core::ptr::addr_of!(_3);
_13 = _7 as f32;
_13 = _8 * _8;
_9 = _15.2 as i16;
_14 = [(-7_i8),(-72_i8),(-22_i8),90_i8,(-113_i8),(-19_i8),(-116_i8),(-123_i8)];
_17.0 = core::ptr::addr_of!(_3);
_2 = 42387_u16 as usize;
_8 = _13 * _13;
_11.1 = [158180162476973189357725958354656361946_i128,(-22966904557570213434190584451401133920_i128)];
_11.0 = 2146459845237093556_u64;
_11.0 = 17521434185385142775_u64;
match _11.0 {
0 => bb10,
17521434185385142775 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_1 = _6;
_7 = (-902497162_i32) + 1154022356_i32;
_8 = 37148_u16 as f32;
_6 = _1;
_3 = 43_isize - 9223372036854775807_isize;
_7 = (-2126608785_i32);
_1 = _6;
_7 = _8 as i32;
_6 = _1;
_8 = _2 as f32;
match RET {
0 => bb2,
1 => bb3,
232 => bb5,
_ => bb4
}
}
bb12 = {
_3 = -9223372036854775807_isize;
_6 = _1;
_11.0 = 3677214034596218096_u64 - 10504310499727176150_u64;
_1 = _6;
_7 = (-8375257512609800733_i64) as i32;
_13 = _8 + _8;
_11.0 = 4335146393861453965_u64 << RET;
_8 = 38448_u16 as f32;
_11.1 = [71868391966777745103820566157267298779_i128,138407356545891901285351730096232911034_i128];
RET = 184_u8 + 7_u8;
_8 = _13 * _13;
_9 = (-29347_i16) & 11174_i16;
RET = 27_u8 - 182_u8;
_2 = 5736852703896983929_usize + 611857215017411948_usize;
_11.1 = [(-145221504681516088236966013455492484393_i128),(-133686590320146950461175685004740723556_i128)];
_9 = 13275_i16 - (-12168_i16);
_11.1 = [119685710282151586574863658242548502405_i128,(-153252166514077849286872875631028235662_i128)];
_15.2 = _8 as f64;
_15.2 = _7 as f64;
_1 = _12;
_3 = (-9223372036854775808_isize);
_14 = [42_i8,(-42_i8),(-75_i8),38_i8,(-91_i8),(-52_i8),79_i8,63_i8];
RET = 150_u8 + 160_u8;
Goto(bb13)
}
bb13 = {
_2 = 7_usize >> _7;
_12 = _1;
_23 = 2221370891_u32 as i32;
_15.2 = _13 as f64;
_19 = _12;
_14 = [(-8_i8),(-68_i8),(-3_i8),99_i8,(-103_i8),(-99_i8),(-11_i8),(-26_i8)];
_3 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_6 = _19;
_21 = _1;
Goto(bb14)
}
bb14 = {
_22 = _7 ^ _23;
_6 = _1;
_17.0 = core::ptr::addr_of!(_3);
_11.0 = !18415515500204588473_u64;
_1 = _12;
_14 = [(-4_i8),99_i8,(-79_i8),92_i8,108_i8,(-82_i8),(-121_i8),25_i8];
_6 = _12;
_1 = _12;
_24 = -_8;
_1 = _21;
_14 = [122_i8,(-77_i8),(-93_i8),(-3_i8),(-93_i8),125_i8,(-76_i8),(-95_i8)];
_23 = !_22;
RET = 213_u8;
_6 = _1;
_21 = _12;
_26 = [RET,RET,RET,RET,RET,RET,RET];
RET = _2 as u8;
_14 = [(-86_i8),(-19_i8),(-123_i8),(-63_i8),27_i8,(-34_i8),(-70_i8),(-124_i8)];
_15.2 = _2 as f64;
_23 = 20220_u16 as i32;
_23 = _11.0 as i32;
_2 = 5_usize;
_26 = [RET,RET,RET,RET,RET,RET,RET];
_6 = _12;
_19 = _6;
_8 = _13 * _24;
Goto(bb15)
}
bb15 = {
_26 = [RET,RET,RET,RET,RET,RET,RET];
_11.1 = [42018554025960737355959165262252635002_i128,45948799760442023341750593936236451136_i128];
_8 = -_13;
Call(_14[_2] = core::intrinsics::bswap(87_i8), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_1 = _6;
_6 = _21;
_31 = _3;
_17.0 = core::ptr::addr_of!(_3);
_19 = _6;
RET = _26[_2] + _26[_2];
_14 = [(-33_i8),(-61_i8),(-113_i8),(-110_i8),(-28_i8),(-119_i8),46_i8,119_i8];
_23 = _7 & _22;
_11.1 = [(-93698436414174397266980431863232904139_i128),125115855150486575435072872093896641245_i128];
_29 = _11.0 * _11.0;
_21 = _12;
_30 = _6;
_31 = _3 * _3;
_6 = _21;
_9 = (-26292_i16);
_31 = _3 << _26[_2];
Call(_32 = fn9(_31, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_2 = !3_usize;
_12 = _21;
_15.2 = 99_i8 as f64;
_29 = _11.0 * _11.0;
_21 = _12;
_8 = -_13;
_23 = 50910308690462218993692141609196131435_i128 as i32;
_24 = -_13;
_11.1 = [(-90416991888436602136285396362909255585_i128),150969533132779785687046816190575192207_i128];
_30 = _19;
_31 = _3 - _3;
_8 = _24 + _24;
_9 = 20854_i16;
_29 = _11.0 * _11.0;
_12 = _21;
_13 = -_8;
_7 = _23;
_6 = _30;
Call(_8 = fn17(Move(_5), _31, Move(_4), _24, _11, _15.2, _9, _12, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_8 = _24 - _13;
_21 = _6;
_9 = 2179_i16;
_2 = !3811569949713479742_usize;
_15.2 = _23 as f64;
_14 = [42_i8,(-32_i8),(-23_i8),71_i8,120_i8,34_i8,(-52_i8),103_i8];
_23 = _7 * _22;
_22 = 8190_u16 as i32;
_30 = _12;
_6 = _12;
_22 = _23 << _32;
_2 = 185638090393821434_usize + 2_usize;
_36 = _8 != _8;
Goto(bb19)
}
bb19 = {
_23 = _2 as i32;
_23 = _22;
_11.1 = [155006236209903579020093852492210506452_i128,84899117799821922886694980803831281760_i128];
_30 = _19;
_6 = _30;
_32 = !RET;
_41 = _23 * _23;
_12 = _6;
_17.0 = core::ptr::addr_of!(_34);
_29 = _11.0 | _11.0;
_36 = !false;
_3 = _31 << RET;
_17.0 = core::ptr::addr_of!(_31);
_14 = [119_i8,(-16_i8),(-34_i8),108_i8,(-30_i8),(-12_i8),(-73_i8),(-119_i8)];
_11.0 = _29 * _29;
_43 = core::ptr::addr_of!(_34);
_31 = -_3;
(*_43) = _31 & _3;
_21 = _6;
(*_43) = _31 - _31;
(*_43) = _31;
(*_43) = _3 + _3;
(*_43) = _31 | _31;
Call((*_43) = core::intrinsics::transmute(_3), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
(*_43) = -_31;
(*_43) = _3 * _31;
_6 = _12;
(*_43) = _31 & _31;
_11.0 = _9 as u64;
(*_43) = _3;
_36 = (*_43) > (*_43);
_31 = -_34;
(*_43) = _3;
(*_43) = _3;
_45 = _8;
_15.2 = 28238_u16 as f64;
(*_43) = _3 << _23;
Goto(bb21)
}
bb21 = {
_39.0 = [50010_u16,57158_u16,9319_u16,43321_u16];
_8 = _45 - _45;
_48 = [_21];
_4 = core::ptr::addr_of_mut!(_46);
(*_43) = _15.2 as isize;
_6 = _30;
(*_43) = _31 >> _32;
(*_43) = _3 | _3;
(*_4) = !207996264130787732700403659773201457464_u128;
_29 = _9 as u64;
(*_4) = 45741744430805312389761213534095214633_u128 >> (*_43);
(*_43) = !_3;
_6 = _12;
(*_43) = _3 << (*_4);
match _9 {
0 => bb13,
1 => bb20,
2 => bb6,
3 => bb8,
4 => bb10,
2179 => bb23,
_ => bb22
}
}
bb22 = {
_2 = !11387393420639631877_usize;
_15.2 = _7 as f64;
_17.0 = core::ptr::addr_of!(_3);
_13 = _7 as f32;
_13 = _8 * _8;
_9 = _15.2 as i16;
_14 = [(-7_i8),(-72_i8),(-22_i8),90_i8,(-113_i8),(-19_i8),(-116_i8),(-123_i8)];
_17.0 = core::ptr::addr_of!(_3);
_2 = 42387_u16 as usize;
_8 = _13 * _13;
_11.1 = [158180162476973189357725958354656361946_i128,(-22966904557570213434190584451401133920_i128)];
_11.0 = 2146459845237093556_u64;
_11.0 = 17521434185385142775_u64;
match _11.0 {
0 => bb10,
17521434185385142775 => bb12,
_ => bb11
}
}
bb23 = {
_15.2 = _29 as f64;
(*_4) = _32 as u128;
_50 = core::ptr::addr_of!((*_43));
_49.2 = 67_i8 >> _34;
_8 = _45 + _24;
(*_4) = !118653283947670276510027497569282555019_u128;
_2 = 3_usize - 808036617489464153_usize;
_48 = [_21];
(*_4) = 153035220435207430704301040174178097036_u128 | 252298168357433863065999792691969966244_u128;
(*_43) = _3;
_49.0 = &_7;
Goto(bb24)
}
bb24 = {
_14 = [_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2];
_17.0 = core::ptr::addr_of!((*_43));
(*_43) = _31;
(*_43) = _31 << (*_4);
_47 = core::ptr::addr_of_mut!((*_4));
(*_43) = _31 & _3;
_18 = &_39.0;
(*_43) = 1351402137_u32 as isize;
(*_4) = 103465771734310277572030411244124341698_u128;
_52 = &mut _32;
_49.1 = core::ptr::addr_of_mut!((*_4));
Call((*_43) = core::intrinsics::bswap(_3), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_19 = _1;
_51 = core::ptr::addr_of_mut!(_13);
(*_51) = _8 * _8;
(*_51) = 55864_u16 as f32;
(*_52) = !RET;
_50 = core::ptr::addr_of!((*_43));
(*_52) = RET + RET;
_12 = _30;
(*_4) = _29 as u128;
_51 = core::ptr::addr_of_mut!((*_51));
_29 = _11.0 - _11.0;
(*_43) = _31 | _31;
(*_43) = -_3;
(*_43) = _3 >> _41;
(*_4) = (*_52) as u128;
(*_4) = 8631206667856381158145935072566152929_u128 - 333587287321263084816982778714801025794_u128;
_2 = 3_usize;
_39.1 = _13 * _8;
(*_51) = _8 - _39.1;
_49.3 = Adt20::Variant0 { fld0: 150736453197943975006550570304988772946_i128,fld1: _19,fld2: (*_52),fld3: _29,fld4: _23 };
(*_43) = _31 >> (*_52);
(*_52) = _26[_2] - RET;
(*_43) = _31 ^ _31;
Goto(bb26)
}
bb26 = {
_17 = (Move(_50),);
_12 = Field::<char>(Variant(_49.3, 0), 1);
place!(Field::<u8>(Variant(_49.3, 0), 2)) = !(*_52);
_24 = (*_51) * (*_51);
_58 = 2736956801_u32 as u64;
(*_43) = -_31;
(*_43) = _3;
_36 = (*_51) <= _24;
(*_4) = !294356355872680028291594730415429449276_u128;
(*_52) = 6375849525020769240_i64 as u8;
(*_4) = 46901263266162532086450807270836254728_u128 * 198629476806448396946970240502099239373_u128;
_53 = &(*_18);
_11.1 = [(-138318242368070599045130604685438117937_i128),(-54477805640005445146328019789658930358_i128)];
_14[_2] = 3000445817_u32 as i8;
(*_51) = -_39.1;
(*_43) = _31 + _31;
_61.1 = Adt20::Variant1 { fld0: _9 };
Goto(bb27)
}
bb27 = {
_62.0 = &mut _36;
_55 = &_23;
(*_43) = -_3;
(*_4) = 55777428659003379494711865694160444137_u128 >> (*_43);
_62.1.0.fld2 = !(*_52);
_60 = (*_52) >> (*_4);
_62.1.0.fld4 = _9;
_44 = (*_51);
_5 = core::ptr::addr_of!(_49.3);
(*_51) = -_44;
_62.1.0.fld0 = _11.1;
_6 = _30;
match (*_18)[_2] {
0 => bb22,
1 => bb28,
2 => bb29,
3 => bb30,
4 => bb31,
5 => bb32,
43321 => bb34,
_ => bb33
}
}
bb28 = {
_23 = _2 as i32;
_23 = _22;
_11.1 = [155006236209903579020093852492210506452_i128,84899117799821922886694980803831281760_i128];
_30 = _19;
_6 = _30;
_32 = !RET;
_41 = _23 * _23;
_12 = _6;
_17.0 = core::ptr::addr_of!(_34);
_29 = _11.0 | _11.0;
_36 = !false;
_3 = _31 << RET;
_17.0 = core::ptr::addr_of!(_31);
_14 = [119_i8,(-16_i8),(-34_i8),108_i8,(-30_i8),(-12_i8),(-73_i8),(-119_i8)];
_11.0 = _29 * _29;
_43 = core::ptr::addr_of!(_34);
_31 = -_3;
(*_43) = _31 & _3;
_21 = _6;
(*_43) = _31 - _31;
(*_43) = _31;
(*_43) = _3 + _3;
(*_43) = _31 | _31;
Call((*_43) = core::intrinsics::transmute(_3), ReturnTo(bb20), UnwindUnreachable())
}
bb29 = {
Return()
}
bb30 = {
_1 = _6;
_7 = (-902497162_i32) + 1154022356_i32;
_8 = 37148_u16 as f32;
_6 = _1;
_3 = 43_isize - 9223372036854775807_isize;
_7 = (-2126608785_i32);
_1 = _6;
_7 = _8 as i32;
_6 = _1;
_8 = _2 as f32;
match RET {
0 => bb2,
1 => bb3,
232 => bb5,
_ => bb4
}
}
bb31 = {
_2 = 8407370163250677906_usize & 7_usize;
_9 = 1869676110_u32 as i16;
_1 = _6;
_9 = 25163_i16 - 23109_i16;
_12 = _1;
_9 = (-30288_i16);
_11.1 = [(-36659454951356511683117008899329153529_i128),(-83788804716741455273299776468542921452_i128)];
_8 = 314678073431949840405732525657437589292_u128 as f32;
_1 = _12;
_7 = _3 as i32;
_2 = 0_usize;
_11.1 = [129523117599685242319632245130735815995_i128,117873388888340084279957817220810300913_i128];
RET = 100_u8 & 68_u8;
_6 = _1;
_14[_2] = 90_i8 + 9_i8;
_12 = _6;
Goto(bb9)
}
bb32 = {
Return()
}
bb33 = {
_1 = _6;
_7 = (-902497162_i32) + 1154022356_i32;
_8 = 37148_u16 as f32;
_6 = _1;
_3 = 43_isize - 9223372036854775807_isize;
_7 = (-2126608785_i32);
_1 = _6;
_7 = _8 as i32;
_6 = _1;
_8 = _2 as f32;
match RET {
0 => bb2,
1 => bb3,
232 => bb5,
_ => bb4
}
}
bb34 = {
_5 = core::ptr::addr_of!((*_5));
_46 = 30207316889637231473331914954570822643_u128;
place!(Field::<i32>(Variant((*_5), 0), 4)) = 82974113799323105247899100493349609516_i128 as i32;
place!(Field::<i128>(Variant((*_5), 0), 0)) = (-161564905860837936412356060785933773670_i128);
(*_43) = _31 & _31;
(*_4) = 51477094324184162314450706965612255261_u128 & 302781968687104994237116655516574273638_u128;
_11 = (_58, _62.1.0.fld0);
(*_4) = 42657619442493881374503416907540601580_u128;
Call((*_5) = fn18(), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
_14 = [_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2];
(*_43) = _31 * _3;
(*_4) = !331476455539215369493123497119401668176_u128;
_56 = _49.2;
(*_43) = !_31;
_26 = [(*_52),(*_52),(*_52),(*_52),RET,RET,_60];
_46 = 312514105405949488870579956025751573927_u128 << (*_55);
_62.1.0.fld4 = -_9;
(*_52) = _56 as u8;
(*_52) = !_60;
(*_5) = Adt20::Variant0 { fld0: (-11730342018794731731248833993409409912_i128),fld1: _6,fld2: (*_52),fld3: _29,fld4: (*_55) };
_17 = (Move(_43),);
place!(Field::<u8>(Variant((*_5), 0), 2)) = !(*_52);
(*_5) = _61.1;
_64 = [(*_55),(*_55),(*_55)];
(*_52) = _60;
(*_5) = Adt20::Variant1 { fld0: Field::<i16>(Variant(_61.1, 1), 0) };
(*_5) = _61.1;
(*_51) = _8;
(*_51) = _8 - _8;
match _9 {
0 => bb17,
1 => bb8,
2 => bb20,
3 => bb6,
4 => bb27,
5 => bb36,
2179 => bb38,
_ => bb37
}
}
bb36 = {
_2 = 7_usize >> _7;
_12 = _1;
_23 = 2221370891_u32 as i32;
_15.2 = _13 as f64;
_19 = _12;
_14 = [(-8_i8),(-68_i8),(-3_i8),99_i8,(-103_i8),(-99_i8),(-11_i8),(-26_i8)];
_3 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_6 = _19;
_21 = _1;
Goto(bb14)
}
bb37 = {
_2 = !11387393420639631877_usize;
_15.2 = _7 as f64;
_17.0 = core::ptr::addr_of!(_3);
_13 = _7 as f32;
_13 = _8 * _8;
_9 = _15.2 as i16;
_14 = [(-7_i8),(-72_i8),(-22_i8),90_i8,(-113_i8),(-19_i8),(-116_i8),(-123_i8)];
_17.0 = core::ptr::addr_of!(_3);
_2 = 42387_u16 as usize;
_8 = _13 * _13;
_11.1 = [158180162476973189357725958354656361946_i128,(-22966904557570213434190584451401133920_i128)];
_11.0 = 2146459845237093556_u64;
_11.0 = 17521434185385142775_u64;
match _11.0 {
0 => bb10,
17521434185385142775 => bb12,
_ => bb11
}
}
bb38 = {
(*_5) = Adt20::Variant1 { fld0: _62.1.0.fld4 };
_11.1 = _62.1.0.fld0;
_49 = (Move(_55), Move(_4), _56, _61.1);
(*_52) = false as u8;
(*_5) = Adt20::Variant0 { fld0: (-140136405241000444072804717250098993156_i128),fld1: _1,fld2: (*_52),fld3: _29,fld4: _23 };
place!(Field::<char>(Variant(_49.3, 0), 1)) = _19;
place!(Field::<i128>(Variant((*_5), 0), 0)) = 119249427975648704068342891250731087747_i128 + (-167666491852040230165869413925643926392_i128);
place!(Field::<u8>(Variant((*_5), 0), 2)) = (*_51) as u8;
(*_5) = Adt20::Variant2 { fld0: _41 };
(*_5) = Adt20::Variant1 { fld0: Field::<i16>(Variant(_61.1, 1), 0) };
(*_52) = _23 as u8;
place!(Field::<i16>(Variant((*_5), 1), 0)) = -_9;
_62.1.0.fld3 = !17888_u16;
place!(Field::<i16>(Variant((*_5), 1), 0)) = _9 + Field::<i16>(Variant(_61.1, 1), 0);
(*_52) = _62.1.0.fld2;
_15.3 = core::ptr::addr_of!(_62.1.0);
(*_52) = !_60;
_66 = 1245096943166625697_i64 >> (*_52);
_62.1.0 = Adt19 { fld0: _11.1,fld1: _46,fld2: (*_52),fld3: 33727_u16,fld4: Field::<i16>(Variant((*_5), 1), 0),fld5: (-117907136306641859014067722287241788340_i128) };
place!(Field::<i16>(Variant((*_5), 1), 0)) = _9 ^ _62.1.0.fld4;
_67 = core::ptr::addr_of!(_62.1.0);
(*_67) = Adt19 { fld0: _11.1,fld1: _46,fld2: (*_52),fld3: 36688_u16,fld4: Field::<i16>(Variant((*_5), 1), 0),fld5: 28185803543557903078799134646563289678_i128 };
_4 = core::ptr::addr_of_mut!((*_67).fld1);
match (*_67).fld5 {
0 => bb39,
28185803543557903078799134646563289678 => bb41,
_ => bb40
}
}
bb39 = {
Return()
}
bb40 = {
_14 = [_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2,_49.2];
(*_43) = _31 * _3;
(*_4) = !331476455539215369493123497119401668176_u128;
_56 = _49.2;
(*_43) = !_31;
_26 = [(*_52),(*_52),(*_52),(*_52),RET,RET,_60];
_46 = 312514105405949488870579956025751573927_u128 << (*_55);
_62.1.0.fld4 = -_9;
(*_52) = _56 as u8;
(*_52) = !_60;
(*_5) = Adt20::Variant0 { fld0: (-11730342018794731731248833993409409912_i128),fld1: _6,fld2: (*_52),fld3: _29,fld4: (*_55) };
_17 = (Move(_43),);
place!(Field::<u8>(Variant((*_5), 0), 2)) = !(*_52);
(*_5) = _61.1;
_64 = [(*_55),(*_55),(*_55)];
(*_52) = _60;
(*_5) = Adt20::Variant1 { fld0: Field::<i16>(Variant(_61.1, 1), 0) };
(*_5) = _61.1;
(*_51) = _8;
(*_51) = _8 - _8;
match _9 {
0 => bb17,
1 => bb8,
2 => bb20,
3 => bb6,
4 => bb27,
5 => bb36,
2179 => bb38,
_ => bb37
}
}
bb41 = {
(*_67).fld1 = _46 & _46;
_49.0 = &_23;
(*_52) = (*_67).fld2 >> (*_67).fld1;
_50 = core::ptr::addr_of!(_31);
_46 = (*_67).fld4 as u128;
_5 = core::ptr::addr_of!((*_5));
(*_67).fld5 = _24 as i128;
(*_67).fld2 = (*_52);
(*_52) = (*_67).fld2;
(*_67).fld1 = !_46;
(*_67).fld1 = 57019767_u32 as u128;
Goto(bb42)
}
bb42 = {
_65 = _46 + (*_67).fld1;
(*_67).fld0 = _11.1;
(*_67).fld5 = -(-20887743645014336911062331226169770048_i128);
(*_67).fld0 = [(*_67).fld5,(*_67).fld5];
(*_67) = Adt19 { fld0: _11.1,fld1: _46,fld2: (*_52),fld3: 18817_u16,fld4: Field::<i16>(Variant((*_5), 1), 0),fld5: 46347028489368792659574189943713276857_i128 };
(*_67).fld5 = _8 as i128;
(*_67).fld4 = !Field::<i16>(Variant((*_5), 1), 0);
(*_67).fld2 = (*_52);
place!(Field::<i16>(Variant((*_5), 1), 0)) = (*_67).fld4;
(*_67).fld3 = !14579_u16;
(*_52) = !(*_67).fld2;
(*_67).fld5 = (-9893032062074436982538211829203054722_i128) | (-4798761709615778543914955920672403263_i128);
_33 = Adt31::Variant3 { fld0: 3721908278_u32,fld1: Move(_49.1) };
(*_67).fld4 = Field::<i16>(Variant((*_5), 1), 0);
_57 = Adt20::Variant1 { fld0: Field::<i16>(Variant((*_5), 1), 0) };
(*_67).fld4 = Field::<i16>(Variant((*_5), 1), 0);
_16 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_33, 3), 0)));
_9 = 359927904_u32 as i16;
_17 = (Move(_50),);
_48 = [_19];
(*_16) = 2194154405_u32 | 461019812_u32;
(*_67) = Adt19 { fld0: _11.1,fld1: _46,fld2: (*_52),fld3: 16799_u16,fld4: Field::<i16>(Variant((*_5), 1), 0),fld5: (-120621730998553458028938939608001791822_i128) };
(*_67).fld3 = 2890_u16;
Call((*_67).fld5 = core::intrinsics::transmute(_65), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_62.2 = core::ptr::addr_of_mut!((*_67).fld1);
(*_51) = _2 as f32;
(*_67).fld1 = _15.2 as u128;
_42 = Move(_33);
(*_67) = Adt19 { fld0: _11.1,fld1: _65,fld2: (*_52),fld3: 2736_u16,fld4: Field::<i16>(Variant((*_5), 1), 0),fld5: 127407662389059269562750736789902354499_i128 };
(*_67).fld5 = _56 as i128;
(*_5) = _57;
(*_67).fld1 = (*_52) as u128;
place!(Field::<i16>(Variant(_49.3, 1), 0)) = (*_67).fld2 as i16;
(*_16) = Field::<u32>(Variant(_42, 3), 0) >> (*_67).fld3;
(*_5) = Adt20::Variant0 { fld0: (*_67).fld5,fld1: _1,fld2: (*_52),fld3: _29,fld4: _41 };
(*_67).fld1 = _65 | _46;
place!(Field::<i128>(Variant((*_5), 0), 0)) = (*_67).fld4 as i128;
(*_67).fld2 = Field::<char>(Variant((*_5), 0), 1) as u8;
place!(Field::<u64>(Variant((*_5), 0), 3)) = _11.0 - _29;
_11 = (Field::<u64>(Variant((*_5), 0), 3), (*_67).fld0);
place!(Field::<i32>(Variant((*_5), 0), 4)) = _22 * _23;
place!(Field::<i128>(Variant((*_5), 0), 0)) = (*_67).fld5 & (*_67).fld5;
Goto(bb44)
}
bb44 = {
place!(Field::<u8>(Variant((*_5), 0), 2)) = !(*_52);
_4 = core::ptr::addr_of_mut!(_65);
place!(Field::<u8>(Variant((*_5), 0), 2)) = !(*_52);
_43 = Move(_17.0);
(*_67).fld5 = !Field::<i128>(Variant((*_5), 0), 0);
(*_67) = Adt19 { fld0: _11.1,fld1: (*_4),fld2: (*_52),fld3: 13535_u16,fld4: Field::<i16>(Variant(_57, 1), 0),fld5: Field::<i128>(Variant((*_5), 0), 0) };
place!(Field::<i128>(Variant((*_5), 0), 0)) = (*_67).fld5 >> Field::<i32>(Variant(_49.3, 0), 4);
(*_67).fld2 = !Field::<u8>(Variant((*_5), 0), 2);
(*_51) = _8 + _24;
(*_67).fld4 = Field::<i16>(Variant(_57, 1), 0) ^ _9;
(*_67).fld5 = Field::<i128>(Variant((*_5), 0), 0);
_60 = (*_67).fld2;
(*_67).fld5 = Field::<i128>(Variant((*_5), 0), 0) << (*_4);
(*_67) = Adt19 { fld0: _11.1,fld1: (*_4),fld2: Field::<u8>(Variant((*_5), 0), 2),fld3: 25427_u16,fld4: Field::<i16>(Variant(_57, 1), 0),fld5: Field::<i128>(Variant((*_5), 0), 0) };
RET = _15.2 as u8;
(*_67).fld1 = (*_4) & (*_4);
(*_67).fld0 = [(*_67).fld5,Field::<i128>(Variant((*_5), 0), 0)];
_75 = _66 as isize;
place!(Field::<char>(Variant((*_5), 0), 1)) = _6;
_41 = -Field::<i32>(Variant((*_5), 0), 4);
_55 = Move(_49.0);
(*_67).fld5 = Field::<i128>(Variant((*_5), 0), 0) + Field::<i128>(Variant(_49.3, 0), 0);
(*_67).fld4 = Field::<i16>(Variant(_57, 1), 0) - Field::<i16>(Variant(_57, 1), 0);
Goto(bb45)
}
bb45 = {
_62.1.0.fld4 = Field::<i16>(Variant(_57, 1), 0) ^ Field::<i16>(Variant(_57, 1), 0);
_11.1 = [_62.1.0.fld5,(*_67).fld5];
_73 = Field::<u64>(Variant((*_5), 0), 3) ^ Field::<u64>(Variant((*_5), 0), 3);
_52 = &mut (*_67).fld2;
(*_67).fld0 = [(*_67).fld5,Field::<i128>(Variant((*_5), 0), 0)];
(*_52) = _56 as u8;
(*_4) = (*_67).fld1 >> Field::<i128>(Variant((*_5), 0), 0);
(*_4) = true as u128;
_50 = core::ptr::addr_of!(_31);
_33 = Move(_42);
(*_5) = Adt20::Variant1 { fld0: (*_67).fld4 };
(*_52) = _60;
_17 = (Move(_50),);
Goto(bb46)
}
bb46 = {
_18 = Move(_53);
(*_5) = _61.1;
(*_67).fld3 = 4699_u16;
(*_67).fld5 = (*_52) as i128;
(*_52) = !_60;
(*_67).fld4 = _15.2 as i16;
place!(Field::<i16>(Variant(_49.3, 1), 0)) = (*_67).fld4;
Goto(bb47)
}
bb47 = {
_80 = Adt20::Variant2 { fld0: _23 };
(*_4) = (*_67).fld1 ^ (*_67).fld1;
place!(Field::<i16>(Variant((*_5), 1), 0)) = (*_67).fld4 ^ (*_67).fld4;
place!(Field::<i16>(Variant((*_5), 1), 0)) = (*_67).fld4 * (*_67).fld4;
(*_51) = _66 as f32;
_56 = _49.2;
_76 = (*_67).fld3;
(*_5) = _80;
_41 = Field::<i32>(Variant((*_5), 2), 0) - Field::<i32>(Variant((*_5), 2), 0);
place!(Field::<i32>(Variant((*_5), 2), 0)) = _23;
_84 = Adt23::Variant0 { fld0: (*_4),fld1: (*_67).fld5 };
_29 = _73 * _73;
(*_67).fld5 = Field::<i128>(Variant(_84, 0), 1);
(*_4) = _2 as u128;
(*_67).fld0 = [(*_67).fld5,(*_67).fld5];
(*_67).fld3 = _76 & _76;
(*_67).fld4 = Field::<i16>(Variant(_57, 1), 0);
_83 = &_66;
(*_67).fld1 = (*_4);
_17 = (Move(_43),);
(*_67).fld0 = [(*_67).fld5,(*_67).fld5];
place!(Field::<i32>(Variant((*_5), 2), 0)) = _41;
Goto(bb48)
}
bb48 = {
(*_67).fld3 = _76 << Field::<i32>(Variant((*_5), 2), 0);
(*_4) = _2 as u128;
_75 = _34;
Goto(bb49)
}
bb49 = {
(*_51) = (*_83) as f32;
_43 = core::ptr::addr_of!(_34);
(*_67).fld0 = _11.1;
(*_67).fld1 = Field::<u128>(Variant(_84, 0), 0) | (*_4);
_8 = (*_51);
_75 = (*_43) & (*_43);
(*_43) = _49.2 as isize;
_15.0 = &_83;
(*_5) = Adt20::Variant0 { fld0: (*_67).fld5,fld1: _1,fld2: (*_52),fld3: _29,fld4: _23 };
place!(Field::<u8>(Variant((*_5), 0), 2)) = (*_52);
place!(Field::<u8>(Variant(_49.3, 0), 2)) = (*_52) + (*_52);
(*_43) = _75 & _75;
_60 = (*_52);
(*_67).fld5 = Field::<i128>(Variant((*_5), 0), 0);
(*_67).fld1 = _15.2 as u128;
place!(Field::<u64>(Variant((*_5), 0), 3)) = _29 & _73;
(*_5) = Adt20::Variant0 { fld0: (*_67).fld5,fld1: _21,fld2: (*_52),fld3: _29,fld4: _41 };
(*_5) = _80;
_25 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_33, 3), 0)));
place!(Field::<i32>(Variant((*_5), 2), 0)) = (*_67).fld3 as i32;
(*_67).fld3 = _76 >> (*_43);
_42 = Move(_33);
place!(Field::<i32>(Variant((*_5), 2), 0)) = Field::<i32>(Variant(_80, 2), 0);
Goto(bb50)
}
bb50 = {
Call(_88 = dump_var(Move(_46), Move(_29), Move(_22), Move(_1)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_88 = dump_var(Move(_7), Move(_3), Move(_2), Move(_41)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_88 = dump_var(Move(_9), Move(_26), Move(_21), Move(_48)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_88 = dump_var(Move(_31), Move(_14), Move(_56), Move(_11)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: (*const isize,)) -> u8 {
mir! {
type RET = u8;
let _3: i32;
let _4: &'static i64;
let _5: isize;
let _6: isize;
let _7: *const [i8; 3];
let _8: u8;
let _9: &'static &'static mut &'static u32;
let _10: u128;
let _11: *const &'static [i8; 3];
let _12: i64;
let _13: &'static mut [i128; 2];
let _14: ();
let _15: ();
{
RET = 219_u8;
_1 = 9223372036854775807_isize;
RET = 246_u8 * 173_u8;
_2.0 = core::ptr::addr_of!(_1);
_1 = 2_i8 as isize;
_1 = -1_isize;
_2.0 = core::ptr::addr_of!(_1);
RET = 176_u8 - 206_u8;
Call(RET = fn10(Move(_2), _1, _1, _1, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 3762926268_u32 as i32;
_2.0 = core::ptr::addr_of!(_1);
_5 = 2073_i16 as isize;
_3 = -(-302256695_i32);
_2.0 = core::ptr::addr_of!(_5);
RET = !247_u8;
_2.0 = core::ptr::addr_of!(_1);
RET = !194_u8;
_1 = _5;
RET = 60_u8 >> _3;
_3 = 2_usize as i32;
_3 = (-13662_i16) as i32;
_5 = 3143221253_u32 as isize;
_5 = _1 << _1;
_5 = -_1;
Goto(bb2)
}
bb2 = {
_1 = 3_usize as isize;
_1 = _5;
_3 = 1189049806_i32 >> RET;
_5 = _1 - _1;
_2.0 = core::ptr::addr_of!(_6);
_5 = _1 - _1;
_2.0 = core::ptr::addr_of!(_1);
_6 = _5 & _5;
Goto(bb3)
}
bb3 = {
_2.0 = core::ptr::addr_of!(_1);
RET = !182_u8;
_5 = !_6;
_6 = _5 << _1;
RET = !51_u8;
RET = _3 as u8;
_1 = _5;
_3 = -1506570543_i32;
_5 = _1;
_1 = -_5;
Goto(bb4)
}
bb4 = {
_8 = RET >> _6;
_1 = 186322536109412347339018102530411001203_u128 as isize;
_5 = _6 ^ _6;
_5 = _1 - _6;
_3 = 1032542171_i32;
_8 = RET - RET;
_8 = RET ^ RET;
RET = _8 << _5;
_2.0 = core::ptr::addr_of!(_1);
_10 = _5 as u128;
_5 = 4137801005_u32 as isize;
_1 = 5056486563196931773_usize as isize;
_2.0 = core::ptr::addr_of!(_1);
_3 = 4103_i16 as i32;
_10 = 86460676578068655462221828681906366936_u128 << RET;
RET = _8 & _8;
_10 = 338132867048679650410816865077210689056_u128 & 150958038837898752695662055472996675210_u128;
_1 = _5;
Goto(bb5)
}
bb5 = {
Call(_14 = dump_var(Move(_8), Move(_5), Move(_1), _15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (*const isize,),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize) -> u8 {
mir! {
type RET = u8;
let _8: *const &'static [i8; 3];
let _9: isize;
let _10: f32;
let _11: ([char; 1], Adt20);
let _12: *const [bool; 7];
let _13: *const isize;
let _14: Adt38;
let _15: f32;
let _16: Adt62;
let _17: u16;
let _18: &'static [i8; 3];
let _19: *const Adt19;
let _20: i16;
let _21: Adt20;
let _22: f32;
let _23: i16;
let _24: f32;
let _25: isize;
let _26: f32;
let _27: ();
let _28: ();
{
_6 = _5 << _7;
RET = 217_u8 ^ 201_u8;
Goto(bb1)
}
bb1 = {
RET = 137_u8;
_6 = _3;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
137 => bb7,
_ => bb6
}
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
_3 = _7;
_3 = !_6;
_2 = _7 | _3;
_9 = _5 & _4;
_5 = !_2;
_6 = _7 >> _4;
_10 = 13905_i16 as f32;
_9 = 7494703140371623542_i64 as isize;
_1.0 = core::ptr::addr_of!(_5);
_11.0 = ['\u{2bb4e}'];
_11.1 = Adt20::Variant2 { fld0: 1074830798_i32 };
_2 = !_3;
Goto(bb8)
}
bb8 = {
place!(Field::<i32>(Variant(_11.1, 2), 0)) = 103_i8 as i32;
RET = 23_u8;
_6 = 17496_i16 as isize;
_11.1 = Adt20::Variant1 { fld0: (-28567_i16) };
_11.1 = Adt20::Variant1 { fld0: 11531_i16 };
Call(place!(Field::<i16>(Variant(_11.1, 1), 0)) = fn11(Move(_1.0), _2, _4, _5, _4, _5, RET, _3, _6, _5, _5, _11.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = _9;
_3 = _7 + _5;
_1.0 = core::ptr::addr_of!(_5);
RET = 145_u8;
_10 = Field::<i16>(Variant(_11.1, 1), 0) as f32;
_9 = !_6;
_4 = _9 * _3;
_5 = (-112_i8) as isize;
place!(Field::<i16>(Variant(_11.1, 1), 0)) = (-4838_i16) >> _2;
_7 = _4 << _3;
_5 = _7;
RET = 14859560768602246932_u64 as u8;
_11.0 = ['\u{acaf2}'];
_3 = 1206638065_i32 as isize;
_7 = _4;
_1.0 = core::ptr::addr_of!(_4);
Goto(bb10)
}
bb10 = {
_13 = core::ptr::addr_of!(_5);
_2 = -_5;
_11.0 = ['\u{10e5a4}'];
(*_13) = -_7;
_1 = (Move(_13),);
_13 = core::ptr::addr_of!(_9);
place!(Field::<i16>(Variant(_11.1, 1), 0)) = 10740741178396991174_u64 as i16;
Goto(bb11)
}
bb11 = {
_11.1 = Adt20::Variant1 { fld0: (-13583_i16) };
(*_13) = !_2;
_15 = _10 * _10;
(*_13) = _5;
_5 = (*_13);
(*_13) = _5;
_7 = (-23030_i16) as isize;
(*_13) = '\u{36cfc}' as isize;
(*_13) = _2 | _4;
place!(Field::<i16>(Variant(_11.1, 1), 0)) = RET as i16;
_17 = 31990_u16 >> (*_13);
_3 = (*_13) | (*_13);
(*_13) = _17 as isize;
(*_13) = _3 & _2;
_4 = (*_13) << (*_13);
_11.0 = ['\u{10d704}'];
place!(Field::<i16>(Variant(_11.1, 1), 0)) = (-12067_i16) | (-204_i16);
(*_13) = _5 << _3;
Goto(bb12)
}
bb12 = {
_4 = -(*_13);
_1 = (Move(_13),);
_5 = _4;
_15 = _10 + _10;
_4 = -_2;
_17 = !2545_u16;
_20 = !Field::<i16>(Variant(_11.1, 1), 0);
_6 = -_9;
_3 = _9 ^ _6;
_21 = Adt20::Variant2 { fld0: (-1958473827_i32) };
_21 = Adt20::Variant1 { fld0: _20 };
_7 = _9;
_5 = !_6;
_1.0 = core::ptr::addr_of!(_2);
Goto(bb13)
}
bb13 = {
RET = 171_u8 >> _3;
_22 = _10;
_23 = _20 + Field::<i16>(Variant(_21, 1), 0);
_10 = _15 * _15;
_23 = !Field::<i16>(Variant(_11.1, 1), 0);
_25 = -_3;
_13 = Move(_1.0);
_9 = _3 + _3;
_1.0 = core::ptr::addr_of!(_25);
_1 = (Move(_13),);
_21 = Adt20::Variant2 { fld0: (-300182369_i32) };
_13 = core::ptr::addr_of!(_4);
(*_13) = _10 as isize;
_21 = Adt20::Variant0 { fld0: 151833188388301602936477626267410776982_i128,fld1: '\u{d1183}',fld2: RET,fld3: 13642787275771565062_u64,fld4: (-951258784_i32) };
(*_13) = _9;
(*_13) = 94_i8 as isize;
(*_13) = _5 ^ _7;
Goto(bb14)
}
bb14 = {
Call(_27 = dump_var(Move(_7), Move(_20), Move(_3), Move(_17)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_27 = dump_var(Move(_25), _28, _28, _28), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: *const isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: u8,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: [char; 1]) -> i16 {
mir! {
type RET = i16;
let _13: &'static [i8; 3];
let _14: *const &'static [i8; 3];
let _15: u128;
let _16: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _17: i8;
let _18: *const &'static [i8; 3];
let _19: usize;
let _20: [i8; 4];
let _21: &'static mut bool;
let _22: i32;
let _23: *mut [u128; 6];
let _24: [u16; 4];
let _25: *mut f32;
let _26: isize;
let _27: [char; 1];
let _28: *mut Adt38;
let _29: *const [i8; 3];
let _30: f64;
let _31: isize;
let _32: bool;
let _33: u128;
let _34: isize;
let _35: i64;
let _36: [i8; 4];
let _37: isize;
let _38: i8;
let _39: i16;
let _40: isize;
let _41: Adt23;
let _42: *const u32;
let _43: &'static i64;
let _44: [char; 1];
let _45: *const u32;
let _46: &'static mut Adt20;
let _47: bool;
let _48: char;
let _49: &'static mut &'static mut u8;
let _50: (&'static [i8; 3],);
let _51: u32;
let _52: isize;
let _53: &'static mut (u64, [i128; 2]);
let _54: u64;
let _55: f64;
let _56: &'static i64;
let _57: f64;
let _58: usize;
let _59: &'static Adt27;
let _60: f32;
let _61: &'static mut [u128; 6];
let _62: &'static mut bool;
let _63: *const Adt38;
let _64: char;
let _65: ([char; 1], Adt20);
let _66: *const Adt19;
let _67: *const Adt20;
let _68: &'static u32;
let _69: char;
let _70: f64;
let _71: char;
let _72: i8;
let _73: &'static mut bool;
let _74: ();
let _75: ();
{
_8 = '\u{dc67a}' as isize;
_9 = -_11;
_2 = _9 * _4;
_8 = _3 << _2;
_5 = _9;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
23 => bb9,
_ => bb8
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
Return()
}
bb9 = {
_5 = !_8;
_1 = core::ptr::addr_of!(_9);
(*_1) = _5;
(*_1) = 7_usize as isize;
(*_1) = _5 - _8;
(*_1) = 7693371592738236929524342589974890661_u128 as isize;
Goto(bb10)
}
bb10 = {
(*_1) = !_5;
(*_1) = _8 & _10;
(*_1) = _5;
(*_1) = !_8;
(*_1) = _5;
_9 = 12198_u16 as isize;
RET = !2193_i16;
(*_1) = _3;
(*_1) = -_8;
_10 = _4 | (*_1);
_16.2 = (-84_i8) as f64;
_8 = (*_1) * (*_1);
_15 = 17645691284652665115_u64 as u128;
(*_1) = !_8;
_17 = (-55_i8) - 114_i8;
Goto(bb11)
}
bb11 = {
_1 = core::ptr::addr_of!((*_1));
RET = -8947_i16;
_1 = core::ptr::addr_of!((*_1));
(*_1) = _10;
_12 = ['\u{94e0a}'];
_11 = _8 * (*_1);
(*_1) = -_11;
(*_1) = _8 ^ _6;
_8 = (*_1);
(*_1) = _11 << _10;
(*_1) = _11 + _5;
(*_1) = _8;
(*_1) = _8;
_15 = !124830363195254869928181087773287690403_u128;
(*_1) = _6 << _11;
(*_1) = _5;
RET = !1594_i16;
_5 = (*_1);
_17 = !(-121_i8);
Goto(bb12)
}
bb12 = {
(*_1) = 1170071942_u32 as isize;
_10 = !_8;
_15 = 239922567248058704914681479323890311166_u128 + 208895586098948936841029791941884268293_u128;
(*_1) = -_10;
_16.2 = 6_usize as f64;
(*_1) = -_2;
_12 = ['\u{3ffac}'];
(*_1) = _10 + _10;
(*_1) = _2 << _8;
_1 = core::ptr::addr_of!((*_1));
_16.2 = _15 as f64;
(*_1) = _11;
(*_1) = _8 >> _8;
_1 = core::ptr::addr_of!((*_1));
_12 = ['\u{68c65}'];
(*_1) = _10 << _11;
(*_1) = -_5;
(*_1) = _11 | _5;
(*_1) = -_11;
(*_1) = _8 << _11;
(*_1) = _8 >> _6;
_10 = (*_1) & (*_1);
(*_1) = _8 * _10;
Call(_20 = core::intrinsics::transmute(_12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_1) = _10 & _11;
_4 = 13861814074798013233_usize as isize;
(*_1) = !_11;
_19 = !12321428469911851984_usize;
(*_1) = _5 << _3;
(*_1) = _8;
(*_1) = _10 << _11;
(*_1) = _11 ^ _8;
(*_1) = _11 & _10;
_7 = 136_u8 & 246_u8;
_15 = 279516768375059225691302339616510329147_u128 >> (*_1);
(*_1) = _10 & _10;
_7 = !74_u8;
_17 = 74_i8;
_12 = ['\u{6a154}'];
_19 = RET as usize;
(*_1) = _8 << _10;
_19 = 1_usize | 3248807487675716187_usize;
match _17 {
0 => bb12,
1 => bb7,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
74 => bb15,
_ => bb14
}
}
bb14 = {
(*_1) = !_5;
(*_1) = _8 & _10;
(*_1) = _5;
(*_1) = !_8;
(*_1) = _5;
_9 = 12198_u16 as isize;
RET = !2193_i16;
(*_1) = _3;
(*_1) = -_8;
_10 = _4 | (*_1);
_16.2 = (-84_i8) as f64;
_8 = (*_1) * (*_1);
_15 = 17645691284652665115_u64 as u128;
(*_1) = !_8;
_17 = (-55_i8) - 114_i8;
Goto(bb11)
}
bb15 = {
(*_1) = (-3269135039498606092_i64) as isize;
(*_1) = !_11;
(*_1) = _10 & _3;
_17 = 23_i8 - (-84_i8);
(*_1) = -_10;
_26 = _15 as isize;
(*_1) = _26 - _26;
_12 = ['\u{bdc1b}'];
(*_1) = _26 | _26;
(*_1) = _17 as isize;
_2 = !_26;
(*_1) = -_10;
(*_1) = -_11;
(*_1) = _26;
RET = _7 as i16;
(*_1) = !_26;
(*_1) = _10 * _2;
(*_1) = _8;
_16.2 = 3871137747631339725_i64 as f64;
(*_1) = false as isize;
_27 = ['\u{78722}'];
_27 = ['\u{8602a}'];
(*_1) = _16.2 as isize;
Goto(bb16)
}
bb16 = {
(*_1) = _10 - _11;
_30 = _16.2 * _16.2;
_11 = (*_1) >> (*_1);
(*_1) = -_26;
(*_1) = -_10;
_27 = ['\u{7528c}'];
(*_1) = !_8;
Call((*_1) = fn12(Move(_1), _3, _3, _15, _16.2, _11, _3, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_30 = _16.2 - _16.2;
_24 = [21327_u16,53526_u16,19353_u16,37889_u16];
_9 = _11 & _2;
_26 = -_10;
_27 = ['\u{a6758}'];
_16.2 = _30 + _30;
_7 = 3820676600_u32 as u8;
_1 = core::ptr::addr_of!(_9);
_16.2 = _30 * _30;
_31 = (*_1) ^ _9;
(*_1) = _31 - _31;
_8 = _7 as isize;
(*_1) = _2 | _2;
_27 = _12;
_24 = [4020_u16,41020_u16,62991_u16,14134_u16];
_22 = (-467144545_i32) | 1758078745_i32;
_12 = ['\u{ed988}'];
_30 = _16.2 + _16.2;
Goto(bb18)
}
bb18 = {
_9 = !_31;
(*_1) = _11 >> _11;
(*_1) = _11 >> _10;
(*_1) = _22 as isize;
Goto(bb19)
}
bb19 = {
_32 = _11 != _26;
_5 = _26;
_10 = RET as isize;
(*_1) = _11;
_33 = !_15;
_26 = (*_1) | (*_1);
_21 = &mut _32;
_19 = 10965942234946147796_usize;
_34 = -(*_1);
(*_1) = _11;
(*_21) = true | true;
(*_21) = true ^ false;
(*_1) = _26;
(*_21) = (*_1) > (*_1);
(*_1) = _11 | _26;
_8 = (*_1) << (*_1);
(*_1) = -_31;
Goto(bb20)
}
bb20 = {
(*_1) = _26 >> _26;
(*_1) = _26;
(*_1) = 28754_u16 as isize;
(*_1) = _5;
(*_1) = -_11;
(*_21) = !false;
_8 = (*_1) ^ (*_1);
_6 = (*_1) & (*_1);
_30 = -_16.2;
(*_1) = _26 | _26;
(*_21) = true;
(*_21) = (*_1) < (*_1);
match _19 {
10965942234946147796 => bb21,
_ => bb15
}
}
bb21 = {
_7 = !183_u8;
_6 = 17152184797283182600_u64 as isize;
(*_21) = !true;
(*_21) = (*_1) != (*_1);
(*_1) = _8 >> _26;
(*_21) = false & true;
(*_21) = (*_1) >= (*_1);
_11 = (*_1);
_6 = 1327316037153232798_i64 as isize;
_2 = (*_1);
_37 = (*_1);
_15 = _33 >> _26;
Goto(bb22)
}
bb22 = {
(*_21) = !false;
(*_21) = !true;
_11 = _31;
_16.2 = -_30;
_27 = ['\u{1196d}'];
(*_21) = !true;
_38 = _17 * _17;
_35 = (-663747372729290818_i64) - 6798614076409743873_i64;
(*_21) = !true;
(*_1) = _11 << _34;
(*_21) = (*_1) <= _31;
_43 = &_35;
(*_21) = (*_1) < (*_1);
_44 = ['\u{14423}'];
_3 = 3322340377212038563_u64 as isize;
_11 = _22 as isize;
_41 = Adt23::Variant1 { fld0: (*_43) };
Call((*_21) = fn13((*_1), _35, _33, (*_1), Move(_1), (*_43), _6, (*_1), (*_43), _7), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_10 = _9;
Goto(bb24)
}
bb24 = {
_44 = ['\u{5fb84}'];
(*_21) = _2 <= _2;
_36 = [_38,_17,_38,_38];
(*_21) = !true;
_26 = _19 as isize;
Goto(bb25)
}
bb25 = {
(*_21) = true;
_17 = (*_21) as i8;
_40 = _2 - _10;
_20 = [_38,_17,_38,_38];
(*_21) = _10 < _31;
(*_21) = !true;
(*_21) = true;
_19 = (*_43) as usize;
_17 = _38 - _38;
_16.2 = _30 - _30;
(*_21) = _2 > _8;
_8 = _31;
_16.0 = &_43;
_47 = (*_21);
_38 = _17 >> _37;
Call(_30 = core::intrinsics::transmute(_34), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_21) = _47;
_1 = core::ptr::addr_of!(_9);
_43 = &place!(Field::<i64>(Variant(_41, 1), 0));
_51 = 2649604180_u32 - 3721932264_u32;
(*_1) = _8 ^ _2;
_17 = _38 ^ _38;
(*_1) = _51 as isize;
Goto(bb27)
}
bb27 = {
(*_1) = _31 * _8;
(*_21) = (*_1) == (*_1);
_6 = (*_1);
_9 = -_8;
_52 = _9 | _31;
(*_21) = !_47;
(*_1) = _17 as isize;
_1 = core::ptr::addr_of!(_34);
(*_1) = _52 | _8;
_54 = (-156862676984906220128507080953634785763_i128) as u64;
_26 = (*_1);
(*_1) = _37 | _5;
(*_21) = (*_1) >= (*_1);
(*_21) = _47 ^ _47;
_42 = core::ptr::addr_of!(_51);
(*_42) = _19 as u32;
(*_42) = _19 as u32;
_30 = _16.2;
(*_42) = 528114016_u32 * 1775068301_u32;
(*_42) = 3449255035_u32;
_37 = !(*_1);
_56 = &(*_43);
_39 = !RET;
(*_21) = !_47;
(*_42) = 4037876050_u32 | 1100801824_u32;
_16.2 = _15 as f64;
Call((*_42) = fn14((*_1), Move(_1), (*_43), Move(_43), Move(_42), (*_56), (*_1), (*_1), _3, _6, _5, Move(_56)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_8 = _26;
(*_21) = !_47;
_43 = &_35;
(*_21) = _9 == _40;
(*_21) = !_47;
Goto(bb29)
}
bb29 = {
_54 = 11725916989422752490_u64;
_40 = _31;
(*_21) = _10 != _8;
(*_21) = _37 != _40;
_34 = -_8;
_57 = _16.2 * _16.2;
_35 = -Field::<i64>(Variant(_41, 1), 0);
_45 = core::ptr::addr_of!(_51);
match _54 {
0 => bb9,
1 => bb5,
2 => bb13,
3 => bb30,
4 => bb31,
11725916989422752490 => bb33,
_ => bb32
}
}
bb30 = {
Return()
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
(*_45) = _22 as u32;
_51 = (*_21) as u32;
_51 = 2841783268_u32 ^ 2290996689_u32;
(*_45) = !3827573106_u32;
_38 = 53626_u16 as i8;
_48 = '\u{9eb79}';
_31 = _37 + _40;
(*_21) = _47 ^ _47;
(*_45) = 654437564_u32 + 2338407290_u32;
_21 = &mut _47;
match _54 {
0 => bb8,
1 => bb5,
2 => bb4,
3 => bb34,
4 => bb35,
5 => bb36,
6 => bb37,
11725916989422752490 => bb39,
_ => bb38
}
}
bb34 = {
_32 = _11 != _26;
_5 = _26;
_10 = RET as isize;
(*_1) = _11;
_33 = !_15;
_26 = (*_1) | (*_1);
_21 = &mut _32;
_19 = 10965942234946147796_usize;
_34 = -(*_1);
(*_1) = _11;
(*_21) = true | true;
(*_21) = true ^ false;
(*_1) = _26;
(*_21) = (*_1) > (*_1);
(*_1) = _11 | _26;
_8 = (*_1) << (*_1);
(*_1) = -_31;
Goto(bb20)
}
bb35 = {
(*_1) = _10 & _11;
_4 = 13861814074798013233_usize as isize;
(*_1) = !_11;
_19 = !12321428469911851984_usize;
(*_1) = _5 << _3;
(*_1) = _8;
(*_1) = _10 << _11;
(*_1) = _11 ^ _8;
(*_1) = _11 & _10;
_7 = 136_u8 & 246_u8;
_15 = 279516768375059225691302339616510329147_u128 >> (*_1);
(*_1) = _10 & _10;
_7 = !74_u8;
_17 = 74_i8;
_12 = ['\u{6a154}'];
_19 = RET as usize;
(*_1) = _8 << _10;
_19 = 1_usize | 3248807487675716187_usize;
match _17 {
0 => bb12,
1 => bb7,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
74 => bb15,
_ => bb14
}
}
bb36 = {
Return()
}
bb37 = {
_54 = 11725916989422752490_u64;
_40 = _31;
(*_21) = _10 != _8;
(*_21) = _37 != _40;
_34 = -_8;
_57 = _16.2 * _16.2;
_35 = -Field::<i64>(Variant(_41, 1), 0);
_45 = core::ptr::addr_of!(_51);
match _54 {
0 => bb9,
1 => bb5,
2 => bb13,
3 => bb30,
4 => bb31,
11725916989422752490 => bb33,
_ => bb32
}
}
bb38 = {
Return()
}
bb39 = {
_52 = _8 << _9;
(*_21) = _52 == _40;
(*_21) = true | true;
_36 = [_17,_17,_17,_17];
(*_45) = 1291527625_u32 + 845543910_u32;
(*_21) = !false;
(*_45) = !2139952517_u32;
_5 = _52 << _37;
(*_45) = !2032249492_u32;
_27 = [_48];
(*_45) = !891214346_u32;
Goto(bb40)
}
bb40 = {
_22 = -1706526825_i32;
(*_21) = _40 < _10;
_1 = core::ptr::addr_of!(_3);
_55 = _57 - _16.2;
(*_21) = _55 >= _57;
(*_1) = -_31;
Call(_35 = core::intrinsics::transmute(_19), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
_40 = !(*_1);
(*_45) = !1311987405_u32;
(*_1) = _2 - _34;
(*_1) = _31;
_27 = [_48];
(*_45) = 3441233216_u32 >> (*_1);
_48 = '\u{ae54f}';
_12 = [_48];
(*_1) = _40;
_33 = _57 as u128;
(*_45) = !2043764275_u32;
(*_21) = (*_1) != _40;
(*_45) = 2447546175_u32 - 1261325434_u32;
(*_21) = !false;
_58 = !_19;
(*_21) = (*_1) <= (*_1);
(*_45) = !704579118_u32;
(*_45) = 302197417_u32;
(*_1) = -_6;
(*_1) = _37 * _5;
(*_21) = !false;
_52 = (*_21) as isize;
_11 = _8 | (*_1);
_7 = 54_u8 & 60_u8;
match (*_45) {
0 => bb8,
302197417 => bb43,
_ => bb42
}
}
bb42 = {
Return()
}
bb43 = {
_56 = &place!(Field::<i64>(Variant(_41, 1), 0));
_16.2 = _55 + _55;
_42 = core::ptr::addr_of!((*_45));
_27 = _44;
(*_1) = _9;
(*_1) = !_37;
(*_1) = _7 as isize;
(*_45) = 598931317_u32 & 713063272_u32;
(*_21) = false;
(*_45) = 3961427678_u32 >> _6;
(*_1) = _26;
Goto(bb44)
}
bb44 = {
_24 = [62466_u16,33546_u16,40830_u16,8054_u16];
_22 = (-827859312_i32) - 1646521963_i32;
(*_21) = true ^ false;
(*_45) = !2886864687_u32;
(*_1) = _9 | _31;
_3 = _10;
(*_21) = _37 < (*_1);
_64 = _48;
_30 = _55;
_44 = [_48];
_15 = _33 >> (*_1);
RET = !_39;
_11 = !(*_1);
(*_45) = _64 as u32;
_60 = _17 as f32;
(*_1) = _8 << _17;
(*_21) = !false;
_33 = _15 << _2;
(*_45) = 953316759_u32;
_58 = _19;
_65.1 = Adt20::Variant0 { fld0: (-58263033430530783041475605976219842949_i128),fld1: _48,fld2: _7,fld3: _54,fld4: _22 };
_19 = !_58;
_17 = _7 as i8;
_34 = (*_1) - _40;
(*_21) = (*_56) < (*_56);
(*_1) = _9 & _2;
Goto(bb45)
}
bb45 = {
_3 = (*_21) as isize;
place!(Field::<i32>(Variant(_65.1, 0), 4)) = _22 ^ _22;
_37 = _48 as isize;
(*_21) = false;
_33 = _15 << (*_1);
_48 = Field::<char>(Variant(_65.1, 0), 1);
_60 = _17 as f32;
_6 = _2 * _34;
(*_45) = 1922327092_u32 ^ 2814394751_u32;
_27 = [Field::<char>(Variant(_65.1, 0), 1)];
_20 = [_17,_17,_38,_38];
(*_21) = false;
(*_1) = !_40;
_62 = Move(_21);
_30 = _55;
match Field::<u64>(Variant(_65.1, 0), 3) {
0 => bb9,
1 => bb27,
2 => bb22,
3 => bb36,
4 => bb31,
11725916989422752490 => bb46,
_ => bb6
}
}
bb46 = {
(*_45) = 437824974_u32;
place!(Field::<char>(Variant(_65.1, 0), 1)) = _64;
(*_1) = _34;
_41 = Adt23::Variant0 { fld0: _33,fld1: (-88802836988246859219163447523201341867_i128) };
_56 = &_35;
_9 = (*_1);
_51 = _54 as u32;
(*_45) = !600015543_u32;
_15 = _33 - _33;
_65.0 = [Field::<char>(Variant(_65.1, 0), 1)];
_25 = core::ptr::addr_of_mut!(_60);
_39 = RET ^ RET;
(*_1) = _5 * _11;
(*_25) = Field::<u8>(Variant(_65.1, 0), 2) as f32;
(*_25) = 32307410437026548307035022887498292923_i128 as f32;
(*_45) = 3759963312_u32;
_31 = (*_1);
(*_45) = RET as u32;
_56 = Move(_43);
(*_45) = 553067465_u32 - 2758495869_u32;
(*_45) = !3323018874_u32;
(*_45) = 1401189113_u32 >> _3;
_52 = (*_1) * _6;
_39 = -RET;
Goto(bb47)
}
bb47 = {
_38 = _17 << (*_45);
_6 = (*_1) * (*_1);
(*_1) = _8 << _2;
_54 = Field::<u64>(Variant(_65.1, 0), 3) | Field::<u64>(Variant(_65.1, 0), 3);
(*_25) = _38 as f32;
_60 = 153987390589048974585601359006480630371_i128 as f32;
(*_45) = 4013955482_u32 ^ 1582904762_u32;
place!(Field::<u64>(Variant(_65.1, 0), 3)) = _54 & _54;
_5 = (*_1) + _40;
(*_45) = 504837658_u32 ^ 1087371055_u32;
(*_25) = Field::<u128>(Variant(_41, 0), 0) as f32;
place!(Field::<char>(Variant(_65.1, 0), 1)) = _48;
(*_1) = _40 * _34;
(*_25) = Field::<u8>(Variant(_65.1, 0), 2) as f32;
_1 = core::ptr::addr_of!((*_1));
_42 = core::ptr::addr_of!((*_45));
(*_45) = 1296591081_u32 ^ 2997177098_u32;
(*_25) = _7 as f32;
(*_45) = !2362944301_u32;
_65.1 = Adt20::Variant0 { fld0: 103110689695997677193784842875247300027_i128,fld1: _64,fld2: _7,fld3: _54,fld4: _22 };
_38 = _17;
(*_25) = (-76106147892607952902688134899544669681_i128) as f32;
_15 = Field::<u128>(Variant(_41, 0), 0);
Goto(bb48)
}
bb48 = {
_31 = (*_1) * (*_1);
(*_25) = Field::<u8>(Variant(_65.1, 0), 2) as f32;
(*_25) = _10 as f32;
(*_45) = 1108348221_u32 * 3898720962_u32;
_69 = _48;
(*_1) = !_10;
Goto(bb49)
}
bb49 = {
place!(Field::<char>(Variant(_65.1, 0), 1)) = _48;
(*_1) = 141620301930908903403454215813018254688_i128 as isize;
(*_45) = !3227722007_u32;
(*_1) = _40;
(*_25) = _58 as f32;
(*_25) = 37813_u16 as f32;
(*_1) = Field::<char>(Variant(_65.1, 0), 1) as isize;
_36 = [_38,_17,_38,_38];
(*_45) = 61042_u16 as u32;
_27 = [Field::<char>(Variant(_65.1, 0), 1)];
_37 = -_9;
(*_1) = !_11;
_55 = _16.2;
Goto(bb50)
}
bb50 = {
Call(_74 = dump_var(Move(_9), Move(_47), Move(_34), Move(_2)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_74 = dump_var(Move(_48), Move(_4), Move(_39), Move(_38)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_74 = dump_var(Move(_44), Move(_10), Move(_20), Move(_64)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_74 = dump_var(Move(_27), Move(_26), Move(_19), Move(_35)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_74 = dump_var(Move(_15), Move(_31), Move(_40), _75), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *const isize,mut _2: isize,mut _3: isize,mut _4: u128,mut _5: f64,mut _6: isize,mut _7: isize,mut _8: isize) -> isize {
mir! {
type RET = isize;
let _9: u64;
let _10: char;
let _11: [i128; 2];
let _12: *const [bool; 7];
let _13: i32;
let _14: isize;
let _15: ();
let _16: ();
{
RET = _8;
_2 = RET << _6;
_8 = _2;
_8 = RET << _6;
_4 = (-972503455_i32) as u128;
_2 = -_6;
_10 = '\u{941ae}';
_4 = 319580209407337390083467128299542567107_u128 * 245901805613691320482465360408548933484_u128;
_7 = _6 + _2;
_4 = 9154434385082487317_i64 as u128;
_3 = _2 + _7;
_5 = 30_u8 as f64;
_7 = _6;
_2 = !_8;
_2 = _6 + _8;
_8 = _7;
RET = _2;
_6 = RET | _8;
_9 = false as u64;
_1 = core::ptr::addr_of!(_2);
_5 = 5_usize as f64;
_1 = core::ptr::addr_of!((*_1));
(*_1) = _8 * RET;
(*_1) = _6 & _8;
_6 = !_2;
(*_1) = _3 ^ _8;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(Move(_6), Move(_8), Move(_9), Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: i64,mut _3: u128,mut _4: isize,mut _5: *const isize,mut _6: i64,mut _7: isize,mut _8: isize,mut _9: i64,mut _10: u8) -> bool {
mir! {
type RET = bool;
let _11: bool;
let _12: bool;
let _13: u32;
let _14: i128;
let _15: bool;
let _16: [usize; 6];
let _17: ();
let _18: ();
{
RET = true;
_4 = _1 | _1;
_10 = 108_u8 | 30_u8;
_2 = _6 | _9;
_3 = 95145076522741966950968272129845016135_u128 & 42230576065431194338397021037071950591_u128;
_1 = _4 - _4;
_10 = 226_u8 >> _1;
_5 = core::ptr::addr_of!(_1);
(*_5) = -_4;
Call((*_5) = core::intrinsics::transmute(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _4 >> _8;
(*_5) = !_8;
(*_5) = '\u{27c1e}' as isize;
(*_5) = _8;
_6 = _2;
_12 = RET ^ RET;
(*_5) = -_4;
_6 = _9;
_14 = _9 as i128;
(*_5) = _8;
Goto(bb2)
}
bb2 = {
_5 = core::ptr::addr_of!((*_5));
Goto(bb3)
}
bb3 = {
(*_5) = -_8;
_10 = !204_u8;
_12 = (*_5) == (*_5);
_8 = (*_5) * (*_5);
Goto(bb4)
}
bb4 = {
(*_5) = _12 as isize;
_7 = _8 * (*_5);
(*_5) = !_4;
_2 = _6;
_15 = _12;
(*_5) = 17001931414143224702_u64 as isize;
RET = !_12;
RET = !_12;
Goto(bb5)
}
bb5 = {
Call(_17 = dump_var(Move(_7), Move(_2), Move(_10), Move(_12)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_17 = dump_var(Move(_3), Move(_4), _18, _18), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: *const isize,mut _3: i64,mut _4: &'static i64,mut _5: *const u32,mut _6: i64,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: &'static i64) -> u32 {
mir! {
type RET = u32;
let _13: Adt76;
let _14: char;
let _15: f32;
let _16: [char; 1];
let _17: Adt31;
let _18: *mut isize;
let _19: isize;
let _20: isize;
let _21: [i8; 8];
let _22: (Adt19,);
let _23: *const [i8; 3];
let _24: [usize; 4];
let _25: u128;
let _26: bool;
let _27: (u64, [i128; 2]);
let _28: i64;
let _29: *mut Adt38;
let _30: [u32; 8];
let _31: [u8; 7];
let _32: ([u16; 4], f32);
let _33: i64;
let _34: *const Adt19;
let _35: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _36: char;
let _37: &'static i64;
let _38: usize;
let _39: bool;
let _40: &'static u32;
let _41: bool;
let _42: *const [bool; 7];
let _43: &'static mut &'static u32;
let _44: &'static bool;
let _45: *const u32;
let _46: f32;
let _47: char;
let _48: &'static mut &'static u32;
let _49: char;
let _50: u128;
let _51: *mut f32;
let _52: &'static i32;
let _53: i128;
let _54: f32;
let _55: ([u16; 4], f32);
let _56: *mut isize;
let _57: ();
let _58: ();
{
_8 = (-19013_i16) as isize;
RET = 63024_u16 as u32;
_4 = &_3;
_6 = (*_4) - _3;
_9 = 2295902440077048394_usize as isize;
_4 = &_6;
_9 = _7;
_8 = -_7;
_3 = (*_4) - (*_4);
_5 = core::ptr::addr_of!(RET);
_7 = 152151041351149020357454344627161348700_u128 as isize;
(*_5) = 16554415020298158471_usize as u32;
_15 = (*_4) as f32;
(*_5) = 2225552066_u32 << _10;
_2 = core::ptr::addr_of!(_8);
_11 = !(*_2);
(*_5) = 2027310501_u32 + 923024402_u32;
_2 = core::ptr::addr_of!(_11);
_7 = _11 | (*_2);
(*_2) = _10 ^ _9;
Goto(bb1)
}
bb1 = {
_14 = '\u{27ef7}';
(*_2) = 10304322412804521665621609475172815192_i128 as isize;
_2 = core::ptr::addr_of!((*_2));
(*_2) = _10 ^ _7;
(*_2) = _8 >> (*_4);
(*_2) = 3_usize as isize;
_21 = [(-113_i8),(-46_i8),76_i8,8_i8,97_i8,50_i8,12_i8,(-35_i8)];
_8 = _9 >> _9;
(*_2) = -_10;
_7 = (*_2) | (*_2);
(*_5) = !4092587161_u32;
_2 = core::ptr::addr_of!(_7);
(*_5) = 2467685521_u32 ^ 2815796040_u32;
(*_5) = true as u32;
(*_5) = 933053765_u32 - 1400411301_u32;
_22.0.fld5 = !78877907393793746250158046945809317324_i128;
(*_2) = _11 ^ _9;
_20 = _22.0.fld5 as isize;
_22.0.fld2 = 84_u8 + 58_u8;
_22.0.fld2 = true as u8;
_11 = (*_2);
(*_2) = _11;
Goto(bb2)
}
bb2 = {
_9 = (*_2) >> (*_2);
_21 = [93_i8,(-12_i8),(-88_i8),75_i8,(-44_i8),(-105_i8),67_i8,60_i8];
(*_2) = -_11;
_20 = (*_2);
(*_5) = !1496259015_u32;
(*_2) = _20;
(*_2) = _9 + _10;
_14 = '\u{502be}';
_19 = _15 as isize;
(*_5) = 1199349200_u32 >> (*_2);
(*_2) = 9848168263336344538_u64 as isize;
(*_5) = 843114871_u32;
_4 = &_3;
(*_2) = _8 << _20;
(*_5) = 4124199199_u32 ^ 372783644_u32;
(*_2) = _1;
_22.0.fld2 = 209_u8 & 179_u8;
_2 = core::ptr::addr_of!(_10);
Goto(bb3)
}
bb3 = {
(*_5) = _22.0.fld2 as u32;
_27.1 = [_22.0.fld5,_22.0.fld5];
(*_5) = 369087110_u32 * 1045778653_u32;
_16 = [_14];
_22.0.fld3 = !2820_u16;
_3 = _6;
_24 = [4_usize,7_usize,4_usize,0_usize];
_31 = [_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2];
_22.0.fld1 = !272312631315465198589976550560240235321_u128;
(*_2) = (-108_i8) as isize;
_25 = _22.0.fld1;
(*_2) = -_20;
_12 = &_3;
(*_2) = _22.0.fld3 as isize;
_31 = [_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2];
(*_5) = _14 as u32;
_19 = (*_5) as isize;
_18 = core::ptr::addr_of_mut!(_11);
(*_2) = !(*_18);
Call((*_18) = fn15(Move(_12), (*_2), Move(_2), Move(_18), _24, (*_12), (*_2), Move(_4), (*_12), (*_2), (*_2), _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_36 = _14;
_32.0 = [_22.0.fld3,_22.0.fld3,_22.0.fld3,_22.0.fld3];
RET = 2129739707_u32 * 2527374956_u32;
(*_5) = _3 as u32;
_22.0.fld3 = 59879_u16;
match _22.0.fld3 {
0 => bb1,
1 => bb3,
59879 => bb6,
_ => bb5
}
}
bb5 = {
(*_5) = _22.0.fld2 as u32;
_27.1 = [_22.0.fld5,_22.0.fld5];
(*_5) = 369087110_u32 * 1045778653_u32;
_16 = [_14];
_22.0.fld3 = !2820_u16;
_3 = _6;
_24 = [4_usize,7_usize,4_usize,0_usize];
_31 = [_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2];
_22.0.fld1 = !272312631315465198589976550560240235321_u128;
(*_2) = (-108_i8) as isize;
_25 = _22.0.fld1;
(*_2) = -_20;
_12 = &_3;
(*_2) = _22.0.fld3 as isize;
_31 = [_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2];
(*_5) = _14 as u32;
_19 = (*_5) as isize;
_18 = core::ptr::addr_of_mut!(_11);
(*_2) = !(*_18);
Call((*_18) = fn15(Move(_12), (*_2), Move(_2), Move(_18), _24, (*_12), (*_2), Move(_4), (*_12), (*_2), (*_2), _7), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
(*_5) = _20 as u32;
_33 = _6 * _6;
(*_5) = 1958821948_u32 * 2173819010_u32;
_34 = core::ptr::addr_of!(_22.0);
(*_34).fld3 = 32454_u16 << _9;
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 210_u8,fld3: 24130_u16,fld4: 18935_i16,fld5: 83063334019893473422964225002573754144_i128 };
_35.2 = (*_34).fld2 as f64;
_30 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5)];
(*_34).fld3 = !50730_u16;
(*_34).fld4 = (-31715_i16);
(*_34).fld2 = !239_u8;
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 246_u8,fld3: 33482_u16,fld4: (-27189_i16),fld5: (-15096052385465000871291344816012992276_i128) };
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 56_u8,fld3: 42178_u16,fld4: 32236_i16,fld5: 101341831745741753313258971753550553374_i128 };
(*_34).fld3 = 9905_u16;
(*_34).fld5 = 5_usize as i128;
(*_5) = !3573866568_u32;
_10 = !_8;
(*_34).fld4 = (-2918_i16) | 29208_i16;
(*_34).fld5 = 54_i8 as i128;
(*_34).fld5 = 1675733470_i32 as i128;
(*_34).fld2 = 133_u8 ^ 76_u8;
(*_34).fld2 = 247_u8 << _10;
Goto(bb7)
}
bb7 = {
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 253_u8,fld3: 24287_u16,fld4: (-22280_i16),fld5: 60447285828092361059650887173036812683_i128 };
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 15_u8,fld3: 3313_u16,fld4: (-944_i16),fld5: (-90018949522955388222593742624974189398_i128) };
(*_34).fld2 = 169_u8;
(*_34).fld5 = _35.2 as i128;
(*_34).fld4 = 7171_i16 * 26086_i16;
_19 = _8 ^ _11;
(*_34).fld1 = _25 & _25;
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 149_u8,fld3: 64210_u16,fld4: 15417_i16,fld5: 92227242967737640261086095665538825291_i128 };
_39 = (*_34).fld4 < (*_34).fld4;
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 69_u8,fld3: 11315_u16,fld4: (-13985_i16),fld5: (-79114450111560545798673157668327658616_i128) };
(*_34).fld3 = !18570_u16;
(*_5) = !3316225938_u32;
_27.0 = 12147688944321671766_u64;
_22.0.fld3 = !33536_u16;
_22.0.fld2 = _27.0 as u8;
(*_34).fld4 = !25487_i16;
(*_34).fld2 = !46_u8;
(*_34) = Adt19 { fld0: _27.1,fld1: _25,fld2: 75_u8,fld3: 53932_u16,fld4: 24984_i16,fld5: (-66291672919089682912460117167346437264_i128) };
(*_34).fld1 = _25 & _25;
Goto(bb8)
}
bb8 = {
match _22.0.fld4 {
0 => bb7,
1 => bb5,
2 => bb3,
24984 => bb9,
_ => bb6
}
}
bb9 = {
_32.0 = [(*_34).fld3,(*_34).fld3,(*_34).fld3,(*_34).fld3];
_22.0.fld1 = !_25;
(*_34).fld2 = 206_u8 << (*_34).fld3;
(*_34).fld5 = (-115787046119270181678232327787044200586_i128);
(*_34).fld1 = _35.2 as u128;
_25 = (*_34).fld1;
_22.0.fld2 = 0_u8 - 252_u8;
(*_5) = 2546124054_u32 & 2191777156_u32;
(*_34).fld4 = !(-4348_i16);
(*_34).fld1 = _25 * _25;
(*_5) = !2324830976_u32;
_28 = _6 | _33;
(*_34).fld4 = (*_5) as i16;
(*_5) = 993495443_u32;
(*_34).fld5 = _39 as i128;
(*_34).fld2 = 56_u8 ^ 75_u8;
Goto(bb10)
}
bb10 = {
(*_34).fld3 = 31679_u16 + 37593_u16;
(*_5) = 606661248_u32 - 2416071341_u32;
_37 = &_6;
_41 = _39 & _39;
_44 = &_41;
_22.0.fld0 = [(*_34).fld5,(*_34).fld5];
(*_34).fld1 = _25 | _25;
match _27.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb7,
12147688944321671766 => bb11,
_ => bb8
}
}
bb11 = {
(*_34).fld1 = _25 >> _11;
_36 = _14;
(*_34).fld0 = [(*_34).fld5,(*_34).fld5];
(*_5) = !3189914194_u32;
Call((*_34) = fn16((*_5), (*_44), Move(_37), _27.0, _19, (*_37), (*_37)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_12 = &_3;
(*_34).fld0 = _27.1;
(*_34).fld2 = 32_u8 << (*_34).fld1;
_37 = &_6;
(*_34).fld0 = _27.1;
_35.0 = &_12;
_47 = _14;
(*_34).fld4 = (-1128_i16) * (-22216_i16);
(*_5) = 2861584532_u32 * 2372686920_u32;
(*_34).fld3 = 7_usize as u16;
(*_34).fld2 = (*_34).fld1 as u8;
(*_34).fld3 = _9 as u16;
(*_5) = !2149174875_u32;
(*_5) = 2537263750_u32 | 3828080398_u32;
RET = 3550461708_u32;
(*_34).fld3 = 63610_u16 - 22234_u16;
Goto(bb13)
}
bb13 = {
_22.0.fld3 = 14855_u16 << (*_34).fld2;
_7 = _11;
(*_34).fld0 = [(*_34).fld5,(*_34).fld5];
_26 = !(*_44);
_22.0.fld5 = (-107723579976781990723902606743161042438_i128) | (-76473605689890909364489174320267616692_i128);
(*_34).fld3 = 51489_u16 | 21863_u16;
(*_34).fld2 = (*_34).fld3 as u8;
_49 = _47;
(*_5) = (*_34).fld1 as u32;
_38 = 2446989424810088064_usize | 14550139157620590789_usize;
(*_34).fld3 = (*_44) as u16;
(*_34).fld4 = 17009_i16 >> (*_34).fld3;
Goto(bb14)
}
bb14 = {
Call(_57 = dump_var(Move(_38), Move(_30), Move(_28), Move(_25)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_57 = dump_var(Move(_39), Move(_6), Move(_7), Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(Move(_14), Move(_26), Move(_1), Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(Move(_20), _58, _58, _58), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static i64,mut _2: isize,mut _3: *const isize,mut _4: *mut isize,mut _5: [usize; 4],mut _6: i64,mut _7: isize,mut _8: &'static i64,mut _9: i64,mut _10: isize,mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: u8;
let _14: *const isize;
let _15: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _16: [usize; 6];
let _17: ();
let _18: ();
{
_1 = &_6;
_12 = _10 * _2;
_5 = [8381101533295761507_usize,13342741558260548614_usize,14569700859424836912_usize,9600230709959422943_usize];
_9 = (*_1) << _12;
_11 = _12 - _7;
_8 = &(*_1);
_5 = [16437722500909720471_usize,0_usize,6517251615830883427_usize,4654562187514496824_usize];
_7 = _11;
_2 = _7 << _11;
_6 = !_9;
RET = _7;
_14 = Move(_3);
_15.0 = &_1;
_3 = Move(_14);
RET = _10 * _11;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(Move(_12), Move(_9), Move(_7), Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u32,mut _2: bool,mut _3: &'static i64,mut _4: u64,mut _5: isize,mut _6: i64,mut _7: i64) -> Adt19 {
mir! {
type RET = Adt19;
let _8: &'static mut &'static mut u8;
let _9: f32;
let _10: ();
let _11: ();
{
RET.fld0 = [8813942446330664398572579984527333211_i128,64357609989907770982138809175376463673_i128];
RET.fld1 = 70205819509325536737508121179179154999_u128 << _5;
RET.fld4 = (-3679_i16) ^ (-9074_i16);
_3 = &_7;
_7 = _6 << RET.fld1;
RET.fld5 = 7018448862665462428241042795335533627_i128 | 40746687532447867352047408311013588435_i128;
RET.fld2 = _1 as u8;
_7 = _6;
_7 = _6;
RET.fld4 = 18264_i16;
RET.fld4 = (-18038_i16);
RET.fld3 = 57204_u16;
_3 = &_6;
_3 = &_7;
RET.fld2 = 146_u8 & 53_u8;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(Move(_1), Move(_5), Move(_6), _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *const Adt20,mut _2: isize,mut _3: *mut u128,mut _4: f32,mut _5: (u64, [i128; 2]),mut _6: f64,mut _7: i16,mut _8: char,mut _9: char) -> f32 {
mir! {
type RET = f32;
let _10: Adt31;
let _11: f64;
let _12: bool;
let _13: isize;
let _14: u128;
let _15: &'static u32;
let _16: isize;
let _17: *const [bool; 7];
let _18: f32;
let _19: [bool; 7];
let _20: &'static mut (&'static i32, *mut u128, i8, Adt20);
let _21: f64;
let _22: (Adt19,);
let _23: char;
let _24: (&'static i32, *mut u128, i8, Adt20);
let _25: usize;
let _26: &'static mut bool;
let _27: &'static mut (u64, [i128; 2]);
let _28: i32;
let _29: &'static u32;
let _30: [i8; 4];
let _31: &'static &'static mut &'static u32;
let _32: &'static [char; 1];
let _33: [bool; 7];
let _34: *const [i8; 3];
let _35: [u128; 6];
let _36: u8;
let _37: &'static [u16; 4];
let _38: [u8; 7];
let _39: i8;
let _40: bool;
let _41: u8;
let _42: &'static Adt27;
let _43: *const isize;
let _44: u8;
let _45: *const Adt20;
let _46: *mut u128;
let _47: [i8; 8];
let _48: Adt27;
let _49: *const Adt20;
let _50: isize;
let _51: f64;
let _52: char;
let _53: isize;
let _54: *const u32;
let _55: bool;
let _56: u16;
let _57: isize;
let _58: i128;
let _59: ();
let _60: ();
{
_8 = _9;
_2 = (-88_isize);
_5.1 = [(-165503392319901686195282581946067268653_i128),8806745910585754402661446604913576437_i128];
_5.1 = [38608227019687550475755270408307742466_i128,156312265025069899258570148243553889834_i128];
_4 = (-53_i8) as f32;
_5.1 = [7120153249536093499625272074344039768_i128,(-106730462376469775331258715156120525240_i128)];
RET = _4 * _4;
RET = _4 * _4;
_2 = 9223372036854775807_isize >> _5.0;
_2 = 5_usize as isize;
RET = _4 + _4;
_6 = 383384084_i32 as f64;
_2 = -(-18_isize);
_4 = RET;
_4 = _6 as f32;
match _7 {
0 => bb1,
1 => bb2,
20854 => bb4,
_ => bb3
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
_6 = (-1659326494_i32) as f64;
_2 = 98_isize >> _7;
_13 = -_2;
_5.1 = [(-43690952077740999021337535622832689458_i128),(-96136922603074464489850185331828473158_i128)];
_5.1 = [66070738052466123825383945542754616955_i128,(-8984518873853698457744918361419788201_i128)];
_11 = _6;
_14 = 231019724982064996377892079258287463769_u128 ^ 81476937279264989896567304727273884457_u128;
_6 = RET as f64;
_4 = (-1449157676_i32) as f32;
_6 = (-49_i8) as f64;
_5.0 = !16972440487535992281_u64;
_16 = _13 * _13;
_2 = _16;
_18 = _4;
_12 = _6 >= _6;
_7 = 107_u8 as i16;
_3 = core::ptr::addr_of_mut!(_14);
_10 = Adt31::Variant3 { fld0: 2708661650_u32,fld1: Move(_3) };
_17 = core::ptr::addr_of!(_19);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_14);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_9 = _8;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_5.0 = !14756060586616334121_u64;
place!(Field::<u32>(Variant(_10, 3), 0)) = 1145371796_u32 | 3225535408_u32;
Goto(bb5)
}
bb5 = {
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_14);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_12 = _18 >= RET;
Goto(bb6)
}
bb6 = {
_16 = 6280_u16 as isize;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_22.0.fld1);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_9 = _8;
_22.0.fld3 = 12265_u16;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_4 = _18;
_22.0.fld0 = _5.1;
_9 = _8;
_22.0.fld5 = _2 as i128;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_3 = core::ptr::addr_of_mut!(_14);
(*_3) = !74000934106666917999561275540013604808_u128;
_21 = _11;
(*_3) = 178782579737433527889717020396787998068_u128 | 216625709162051112917258403025355383441_u128;
_19 = [_12,_12,_12,_12,_12,_12,_12];
_5 = (14818502910737268063_u64, _22.0.fld0);
_24.1 = Move(_3);
place!(Field::<u32>(Variant(_10, 3), 0)) = !1197469943_u32;
_4 = RET * RET;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
Goto(bb7)
}
bb7 = {
_3 = core::ptr::addr_of_mut!(_22.0.fld1);
(*_3) = _14 - _14;
(*_3) = _14 >> _13;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!((*_3));
_22.0.fld2 = 18_u8;
_14 = !(*_3);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_3) = _14;
(*_3) = 456008153_i32 as u128;
(*_3) = _14 & _14;
_22.0.fld1 = _14 << _5.0;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = Move(_3);
_15 = &place!(Field::<u32>(Variant(_10, 3), 0));
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_26 = &mut _12;
(*_26) = true;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_14);
_27 = &mut _5;
(*_26) = _14 >= _14;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
place!(Field::<u32>(Variant(_10, 3), 0)) = 454185517_u32 + 1840179377_u32;
_22.0.fld4 = _7 << (*_27).0;
Call((*_27).1 = core::intrinsics::transmute(_22.0.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_26) = true;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_23 = _8;
Goto(bb9)
}
bb9 = {
(*_27) = (4236731679316060171_u64, _22.0.fld0);
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_29 = &place!(Field::<u32>(Variant(_10, 3), 0));
(*_27).1 = [_22.0.fld5,_22.0.fld5];
match (*_27).0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4236731679316060171 => bb15,
_ => bb14
}
}
bb10 = {
(*_26) = true;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_23 = _8;
Goto(bb9)
}
bb11 = {
_3 = core::ptr::addr_of_mut!(_22.0.fld1);
(*_3) = _14 - _14;
(*_3) = _14 >> _13;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!((*_3));
_22.0.fld2 = 18_u8;
_14 = !(*_3);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_3) = _14;
(*_3) = 456008153_i32 as u128;
(*_3) = _14 & _14;
_22.0.fld1 = _14 << _5.0;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = Move(_3);
_15 = &place!(Field::<u32>(Variant(_10, 3), 0));
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_26 = &mut _12;
(*_26) = true;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_14);
_27 = &mut _5;
(*_26) = _14 >= _14;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
place!(Field::<u32>(Variant(_10, 3), 0)) = 454185517_u32 + 1840179377_u32;
_22.0.fld4 = _7 << (*_27).0;
Call((*_27).1 = core::intrinsics::transmute(_22.0.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_16 = 6280_u16 as isize;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_22.0.fld1);
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_9 = _8;
_22.0.fld3 = 12265_u16;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_4 = _18;
_22.0.fld0 = _5.1;
_9 = _8;
_22.0.fld5 = _2 as i128;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
_3 = core::ptr::addr_of_mut!(_14);
(*_3) = !74000934106666917999561275540013604808_u128;
_21 = _11;
(*_3) = 178782579737433527889717020396787998068_u128 | 216625709162051112917258403025355383441_u128;
_19 = [_12,_12,_12,_12,_12,_12,_12];
_5 = (14818502910737268063_u64, _22.0.fld0);
_24.1 = Move(_3);
place!(Field::<u32>(Variant(_10, 3), 0)) = !1197469943_u32;
_4 = RET * RET;
(*_17) = [_12,_12,_12,_12,_12,_12,_12];
Goto(bb7)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_27) = (3533646108014480345_u64, _22.0.fld0);
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_39 = 69_i8 ^ (-91_i8);
(*_27).1 = _22.0.fld0;
place!(Field::<*mut u128>(Variant(_10, 3), 1)) = core::ptr::addr_of_mut!(_22.0.fld1);
(*_27).1 = [_22.0.fld5,_22.0.fld5];
(*_27) = (1140364477220402307_u64, _22.0.fld0);
(*_27).1 = _22.0.fld0;
(*_27).1 = _22.0.fld0;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_7 = _22.0.fld4;
(*_27).1 = _22.0.fld0;
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_13 = _2;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_24.3 = Adt20::Variant2 { fld0: 940473471_i32 };
_30 = [_39,_39,_39,_39];
(*_27).1 = [_22.0.fld5,_22.0.fld5];
Goto(bb16)
}
bb16 = {
_24.3 = Adt20::Variant1 { fld0: _22.0.fld4 };
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_28 = 2132221727_i32 | 305981323_i32;
(*_27) = (15162173003404560588_u64, _22.0.fld0);
_6 = _21;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_26) = _7 != Field::<i16>(Variant(_24.3, 1), 0);
_35 = [_14,_22.0.fld1,_14,_22.0.fld1,_22.0.fld1,_14];
_22.0 = Adt19 { fld0: (*_27).1,fld1: _14,fld2: 25_u8,fld3: 30814_u16,fld4: _7,fld5: 66331391874797585700809415582792639694_i128 };
(*_26) = _14 == _14;
(*_27) = (9695607851592267756_u64, _22.0.fld0);
_14 = !_22.0.fld1;
_22.0.fld1 = !_14;
(*_27).0 = !9744718634806492965_u64;
(*_27).1 = [_22.0.fld5,_22.0.fld5];
(*_27).1 = [_22.0.fld5,_22.0.fld5];
(*_26) = _14 <= _22.0.fld1;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_22.0.fld3 = 52792_u16;
(*_27).0 = 8908144362164908867_u64 * 5409831300950923197_u64;
_48.fld0 = _6 * _6;
(*_27).1 = _22.0.fld0;
_35 = [_14,_14,_22.0.fld1,_22.0.fld1,_14,_14];
(*_27).1 = [_22.0.fld5,_22.0.fld5];
(*_27).1 = _22.0.fld0;
Goto(bb17)
}
bb17 = {
(*_26) = !true;
Goto(bb18)
}
bb18 = {
_48.fld3 = (*_27).0 * (*_27).0;
_18 = _4 - _4;
(*_27) = (_48.fld3, _22.0.fld0);
(*_27) = (_48.fld3, _22.0.fld0);
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_26) = true;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
(*_27) = (_48.fld3, _22.0.fld0);
(*_26) = true;
(*_27).1 = _22.0.fld0;
(*_27) = (_48.fld3, _22.0.fld0);
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_38 = [_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2,_22.0.fld2];
_52 = _8;
_3 = core::ptr::addr_of_mut!(_14);
(*_27) = (_48.fld3, _22.0.fld0);
(*_27) = (_48.fld3, _22.0.fld0);
_48.fld2 = _22.0.fld2;
place!(Field::<i16>(Variant(_24.3, 1), 0)) = _7;
RET = -_18;
_24.1 = core::ptr::addr_of_mut!((*_3));
_13 = !_2;
match _22.0.fld5 {
66331391874797585700809415582792639694 => bb20,
_ => bb19
}
}
bb19 = {
(*_26) = true;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_23 = _8;
Goto(bb9)
}
bb20 = {
(*_27) = (_48.fld3, _22.0.fld0);
(*_26) = true;
(*_27).0 = _48.fld3 * _48.fld3;
(*_27).1 = [_22.0.fld5,_22.0.fld5];
(*_26) = false;
(*_17) = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_25 = 0_usize + 7132864112357677024_usize;
_36 = _18 as u8;
_7 = _52 as i16;
_56 = _22.0.fld3;
(*_27).1 = [_22.0.fld5,_22.0.fld5];
_41 = _22.0.fld2 ^ _22.0.fld2;
(*_27) = (_48.fld3, _22.0.fld0);
(*_26) = true & false;
RET = _4 - _18;
_48.fld3 = (*_27).0 | (*_27).0;
Goto(bb21)
}
bb21 = {
Call(_59 = dump_var(Move(_13), Move(_2), Move(_14), Move(_5)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_59 = dump_var(Move(_12), Move(_25), Move(_38), Move(_23)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_59 = dump_var(Move(_9), Move(_8), _60, _60), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> Adt20 {
mir! {
type RET = Adt20;
let _1: &'static Adt27;
let _2: *const [i8; 3];
let _3: char;
let _4: *mut [u128; 6];
let _5: u32;
let _6: char;
let _7: &'static [u16; 4];
let _8: Adt19;
let _9: isize;
let _10: *mut isize;
let _11: [i8; 3];
let _12: [i128; 2];
let _13: u128;
let _14: bool;
let _15: &'static mut (&'static i32, *mut u128, i8, Adt20);
let _16: *const u32;
let _17: i16;
let _18: i128;
let _19: isize;
let _20: isize;
let _21: *const Adt19;
let _22: [i8; 3];
let _23: isize;
let _24: *mut f32;
let _25: f64;
let _26: char;
let _27: isize;
let _28: (&'static i32, *mut u128, i8, Adt20);
let _29: f64;
let _30: isize;
let _31: &'static mut bool;
let _32: Adt20;
let _33: bool;
let _34: *const [bool; 7];
let _35: u8;
let _36: (&'static [i8; 3],);
let _37: &'static u32;
let _38: f64;
let _39: isize;
let _40: [usize; 4];
let _41: &'static &'static mut &'static u32;
let _42: [i8; 3];
let _43: bool;
let _44: &'static mut [i128; 2];
let _45: [i32; 3];
let _46: isize;
let _47: f32;
let _48: f64;
let _49: i128;
let _50: Adt38;
let _51: &'static mut [u128; 6];
let _52: ([u16; 4], f32);
let _53: isize;
let _54: &'static &'static Adt27;
let _55: *const [i8; 3];
let _56: &'static i64;
let _57: Adt62;
let _58: f64;
let _59: char;
let _60: u8;
let _61: i128;
let _62: i16;
let _63: u64;
let _64: &'static &'static mut &'static u32;
let _65: f64;
let _66: u16;
let _67: &'static mut u8;
let _68: isize;
let _69: f64;
let _70: i64;
let _71: char;
let _72: ();
let _73: ();
{
RET = Adt20::Variant1 { fld0: (-28216_i16) };
RET = Adt20::Variant2 { fld0: (-429807200_i32) };
RET = Adt20::Variant1 { fld0: 6610_i16 };
RET = Adt20::Variant2 { fld0: (-973112459_i32) };
Call(place!(Field::<i32>(Variant(RET, 2), 0)) = core::intrinsics::bswap(1824462426_i32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb2 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb3 = {
_5 = 3761469346_u32 * 298799211_u32;
_6 = _3;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-2041_i16);
_6 = _3;
_3 = _6;
place!(Field::<i16>(Variant(RET, 1), 0)) = 25980_i16 << _5;
_8.fld5 = _3 as i128;
place!(Field::<i16>(Variant(RET, 1), 0)) = -6637_i16;
Goto(bb4)
}
bb4 = {
_8.fld3 = 2913_u16;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld4 = Field::<i16>(Variant(RET, 1), 0) * Field::<i16>(Variant(RET, 1), 0);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: 248_u8,fld3: 16032960608073371866_u64,fld4: 391572483_i32 };
place!(Field::<i32>(Variant(RET, 0), 4)) = !(-1956247955_i32);
place!(Field::<i32>(Variant(RET, 0), 4)) = _8.fld3 as i32;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
_8.fld2 = !131_u8;
place!(Field::<i32>(Variant(RET, 0), 4)) = 1831272129_i32;
_8.fld2 = (-4991707669208280838_i64) as u8;
RET = Adt20::Variant1 { fld0: _8.fld4 };
place!(Field::<i16>(Variant(RET, 1), 0)) = _8.fld4 + _8.fld4;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld5 = (-123585201503074304577664571243543462811_i128);
_10 = core::ptr::addr_of_mut!(_9);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: _8.fld2,fld3: 1188457126822929711_u64,fld4: (-1289510151_i32) };
(*_10) = 9223372036854775807_isize & 9223372036854775807_isize;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
(*_10) = 9223372036854775807_isize;
_3 = _6;
_8.fld2 = false as u8;
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
(*_10) = 47_isize * (-93_isize);
match _8.fld3 {
0 => bb1,
1 => bb2,
2913 => bb5,
_ => bb3
}
}
bb5 = {
_13 = 231530495931530239483201142084438472276_u128 | 231798650891430137625402659936236985323_u128;
_12 = [_8.fld5,_8.fld5];
(*_10) = (-100_isize);
_2 = core::ptr::addr_of!(_11);
_14 = false;
(*_2) = [(-62_i8),113_i8,65_i8];
place!(Field::<u64>(Variant(RET, 0), 3)) = 8063928772419072164_u64;
(*_10) = 34_isize;
(*_10) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_2) = [(-110_i8),19_i8,85_i8];
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [(-99_i8),(-88_i8),(-114_i8)];
match Field::<i128>(Variant(RET, 0), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
216697165417864158885710036188224748645 => bb8,
_ => bb7
}
}
bb6 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb7 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb8 = {
_16 = core::ptr::addr_of!(_5);
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
match _8.fld5 {
0 => bb4,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
216697165417864158885710036188224748645 => bb16,
_ => bb15
}
}
bb9 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb10 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb11 = {
_13 = 231530495931530239483201142084438472276_u128 | 231798650891430137625402659936236985323_u128;
_12 = [_8.fld5,_8.fld5];
(*_10) = (-100_isize);
_2 = core::ptr::addr_of!(_11);
_14 = false;
(*_2) = [(-62_i8),113_i8,65_i8];
place!(Field::<u64>(Variant(RET, 0), 3)) = 8063928772419072164_u64;
(*_10) = 34_isize;
(*_10) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_2) = [(-110_i8),19_i8,85_i8];
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [(-99_i8),(-88_i8),(-114_i8)];
match Field::<i128>(Variant(RET, 0), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
216697165417864158885710036188224748645 => bb8,
_ => bb7
}
}
bb12 = {
_8.fld3 = 2913_u16;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld4 = Field::<i16>(Variant(RET, 1), 0) * Field::<i16>(Variant(RET, 1), 0);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: 248_u8,fld3: 16032960608073371866_u64,fld4: 391572483_i32 };
place!(Field::<i32>(Variant(RET, 0), 4)) = !(-1956247955_i32);
place!(Field::<i32>(Variant(RET, 0), 4)) = _8.fld3 as i32;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
_8.fld2 = !131_u8;
place!(Field::<i32>(Variant(RET, 0), 4)) = 1831272129_i32;
_8.fld2 = (-4991707669208280838_i64) as u8;
RET = Adt20::Variant1 { fld0: _8.fld4 };
place!(Field::<i16>(Variant(RET, 1), 0)) = _8.fld4 + _8.fld4;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld5 = (-123585201503074304577664571243543462811_i128);
_10 = core::ptr::addr_of_mut!(_9);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: _8.fld2,fld3: 1188457126822929711_u64,fld4: (-1289510151_i32) };
(*_10) = 9223372036854775807_isize & 9223372036854775807_isize;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
(*_10) = 9223372036854775807_isize;
_3 = _6;
_8.fld2 = false as u8;
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
(*_10) = 47_isize * (-93_isize);
match _8.fld3 {
0 => bb1,
1 => bb2,
2913 => bb5,
_ => bb3
}
}
bb13 = {
_5 = 3761469346_u32 * 298799211_u32;
_6 = _3;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-2041_i16);
_6 = _3;
_3 = _6;
place!(Field::<i16>(Variant(RET, 1), 0)) = 25980_i16 << _5;
_8.fld5 = _3 as i128;
place!(Field::<i16>(Variant(RET, 1), 0)) = -6637_i16;
Goto(bb4)
}
bb14 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb15 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb16 = {
(*_2) = [50_i8,50_i8,(-80_i8)];
(*_16) = 1249607155_u32 << (*_10);
(*_16) = !3977142807_u32;
_3 = _6;
(*_2) = [(-71_i8),(-88_i8),(-45_i8)];
(*_16) = 1327403839_u32;
(*_2) = [(-117_i8),(-62_i8),(-128_i8)];
(*_2) = [119_i8,52_i8,70_i8];
_8.fld1 = _13 + _13;
(*_16) = 1522488450_u32;
(*_10) = !77_isize;
(*_2) = [(-53_i8),(-85_i8),35_i8];
(*_2) = [92_i8,15_i8,45_i8];
place!(Field::<i128>(Variant(RET, 0), 0)) = !_8.fld5;
(*_2) = [91_i8,(-69_i8),(-83_i8)];
(*_16) = !897480955_u32;
(*_16) = 41352135_u32;
(*_16) = !3529835227_u32;
(*_16) = 436662145_u32;
(*_10) = (-9223372036854775808_isize) + 76_isize;
(*_16) = (-28_i8) as u32;
(*_10) = !(-9223372036854775808_isize);
place!(Field::<i128>(Variant(RET, 0), 0)) = 736749096_i32 as i128;
_9 = 9223372036854775807_isize * (-120_isize);
_8.fld0 = _12;
(*_16) = !1731014408_u32;
(*_2) = [(-127_i8),98_i8,117_i8];
place!(Field::<i32>(Variant(RET, 0), 4)) = 1824531356_i32 | 459411478_i32;
(*_10) = (-109_isize) ^ (-35_isize);
Goto(bb17)
}
bb17 = {
(*_10) = 88_isize;
_20 = !(*_10);
_8.fld0 = _12;
(*_10) = _20 | _20;
(*_10) = _20;
_12 = _8.fld0;
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
(*_2) = [59_i8,(-20_i8),(-114_i8)];
place!(Field::<i128>(Variant(RET, 0), 0)) = !_8.fld5;
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
(*_10) = _20;
(*_2) = [111_i8,59_i8,79_i8];
(*_2) = [(-52_i8),(-42_i8),54_i8];
(*_10) = _20 ^ _20;
(*_10) = _14 as isize;
(*_16) = 998950148_u32;
match (*_16) {
0 => bb8,
1 => bb4,
2 => bb18,
3 => bb19,
998950148 => bb21,
_ => bb20
}
}
bb18 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb19 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb20 = {
_16 = core::ptr::addr_of!(_5);
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
match _8.fld5 {
0 => bb4,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
216697165417864158885710036188224748645 => bb16,
_ => bb15
}
}
bb21 = {
RET = Adt20::Variant1 { fld0: _8.fld4 };
_17 = _8.fld4;
(*_10) = _20 ^ _20;
(*_2) = [2_i8,(-119_i8),(-47_i8)];
RET = Adt20::Variant2 { fld0: 1253064298_i32 };
(*_10) = _20;
_8.fld0 = [_8.fld5,_8.fld5];
Goto(bb22)
}
bb22 = {
(*_2) = [41_i8,(-127_i8),11_i8];
Goto(bb23)
}
bb23 = {
_20 = -(*_10);
_8.fld1 = _13 + _13;
(*_10) = _20;
(*_16) = 3985656237_u32 << _8.fld2;
match _8.fld5 {
0 => bb7,
1 => bb4,
2 => bb5,
3 => bb24,
4 => bb25,
5 => bb26,
216697165417864158885710036188224748645 => bb28,
_ => bb27
}
}
bb24 = {
_8.fld3 = 2913_u16;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld4 = Field::<i16>(Variant(RET, 1), 0) * Field::<i16>(Variant(RET, 1), 0);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: 248_u8,fld3: 16032960608073371866_u64,fld4: 391572483_i32 };
place!(Field::<i32>(Variant(RET, 0), 4)) = !(-1956247955_i32);
place!(Field::<i32>(Variant(RET, 0), 4)) = _8.fld3 as i32;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
_8.fld2 = !131_u8;
place!(Field::<i32>(Variant(RET, 0), 4)) = 1831272129_i32;
_8.fld2 = (-4991707669208280838_i64) as u8;
RET = Adt20::Variant1 { fld0: _8.fld4 };
place!(Field::<i16>(Variant(RET, 1), 0)) = _8.fld4 + _8.fld4;
_8.fld0 = [_8.fld5,_8.fld5];
_8.fld5 = (-123585201503074304577664571243543462811_i128);
_10 = core::ptr::addr_of_mut!(_9);
RET = Adt20::Variant0 { fld0: _8.fld5,fld1: _6,fld2: _8.fld2,fld3: 1188457126822929711_u64,fld4: (-1289510151_i32) };
(*_10) = 9223372036854775807_isize & 9223372036854775807_isize;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
_8.fld0 = [Field::<i128>(Variant(RET, 0), 0),_8.fld5];
(*_10) = 9223372036854775807_isize;
_3 = _6;
_8.fld2 = false as u8;
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
(*_10) = 47_isize * (-93_isize);
match _8.fld3 {
0 => bb1,
1 => bb2,
2913 => bb5,
_ => bb3
}
}
bb25 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb26 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb27 = {
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1212897214_i32) << 1396175964_u32;
RET = Adt20::Variant1 { fld0: 12186_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = 19892_i16 + (-20113_i16);
RET = Adt20::Variant1 { fld0: 28370_i16 };
place!(Field::<i16>(Variant(RET, 1), 0)) = (-11608_i16) + (-20860_i16);
place!(Field::<i16>(Variant(RET, 1), 0)) = -7899_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = !26266_i16;
RET = Adt20::Variant2 { fld0: (-1627089808_i32) };
place!(Field::<i32>(Variant(RET, 2), 0)) = 306245332_i32;
RET = Adt20::Variant2 { fld0: 215524598_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = (-286042673_i32) | (-145461692_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1799933316_i32) ^ (-935211866_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = (-1156043451_i32);
place!(Field::<i32>(Variant(RET, 2), 0)) = 323321640_i32 ^ 697042455_i32;
RET = Adt20::Variant2 { fld0: 840659060_i32 };
place!(Field::<i32>(Variant(RET, 2), 0)) = true as i32;
place!(Field::<i32>(Variant(RET, 2), 0)) = (-701152815_i32);
RET = Adt20::Variant1 { fld0: 24588_i16 };
RET = Adt20::Variant1 { fld0: (-22049_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -(-9072_i16);
Goto(bb2)
}
bb28 = {
_23 = (*_16) as isize;
(*_16) = _8.fld5 as u32;
(*_16) = 3214982479_u32;
_10 = core::ptr::addr_of_mut!((*_10));
_8.fld5 = (-135055331939253796687366116775772205631_i128) * 169772197641112986939789668714635926474_i128;
(*_10) = !_23;
(*_10) = _23;
(*_10) = -_23;
(*_2) = [(-31_i8),74_i8,(-61_i8)];
(*_16) = 4285202922_u32 | 3424977572_u32;
(*_2) = [(-90_i8),124_i8,(-102_i8)];
_21 = core::ptr::addr_of!(_8);
(*_21).fld3 = 55064_u16 >> (*_21).fld1;
(*_21).fld0 = _12;
_19 = (*_10) - (*_10);
(*_10) = _19 & _19;
(*_10) = _19 << (*_16);
(*_21).fld2 = !197_u8;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 231_u8,fld3: 28496_u16,fld4: _17,fld5: 142340313450150343840682873615528523219_i128 };
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 104_u8,fld3: 44733_u16,fld4: _17,fld5: (-98103642568956865546655482344992276550_i128) };
(*_2) = [(-114_i8),(-31_i8),119_i8];
(*_21).fld3 = 46583_u16 + 33839_u16;
Goto(bb29)
}
bb29 = {
_28.2 = (*_21).fld5 as i8;
_20 = (*_10) ^ (*_10);
(*_21).fld5 = 149079343114009589841880165684780431559_i128 + (-151601393380100326185914476032383441828_i128);
(*_16) = 16210904176491208833_u64 as u32;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 21_u8,fld3: 6573_u16,fld4: _17,fld5: (-35981274501699798122225696029014814232_i128) };
(*_21).fld4 = (*_21).fld1 as i16;
(*_21).fld4 = !_17;
(*_21).fld1 = !_13;
(*_21).fld2 = 94_u8 ^ 182_u8;
(*_21).fld2 = 210_u8 - 122_u8;
(*_21).fld4 = _17 * _17;
(*_21).fld2 = 1908750590_i32 as u8;
_5 = !902648355_u32;
_8.fld2 = !25_u8;
(*_21).fld3 = 59968_u16 & 24528_u16;
(*_2) = [_28.2,_28.2,_28.2];
_25 = 8817008209656049153_i64 as f64;
(*_2) = [_28.2,_28.2,_28.2];
(*_10) = _20;
(*_21).fld5 = (*_21).fld4 as i128;
Call((*_21).fld3 = core::intrinsics::bswap(62000_u16), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_19 = (*_10) << (*_21).fld4;
(*_21).fld4 = _17 << (*_10);
(*_21).fld5 = (-148841546224793977167127119185259712876_i128) ^ 95588676640611307728010614954590138663_i128;
(*_2) = [_28.2,_28.2,_28.2];
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
_32 = Adt20::Variant2 { fld0: 228713233_i32 };
_26 = _6;
(*_21).fld4 = _17 * _17;
(*_21).fld5 = -124615443965025434902864078206224262796_i128;
Goto(bb31)
}
bb31 = {
Goto(bb32)
}
bb32 = {
(*_21).fld5 = !147644928766747025893857979287745570527_i128;
(*_16) = 3894713921_u32;
(*_16) = (*_21).fld5 as u32;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 131_u8,fld3: 37569_u16,fld4: _17,fld5: (-73416285990681615941072678356619031400_i128) };
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 102_u8,fld3: 26183_u16,fld4: _17,fld5: (-59092053007301248760534164250213078682_i128) };
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
(*_21).fld4 = _17 << _19;
_38 = _25 + _25;
(*_21).fld1 = _13 + _13;
(*_10) = _14 as isize;
(*_21).fld3 = 767035236591120141_usize as u16;
_8.fld2 = (*_21).fld1 as u8;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 203_u8,fld3: 63811_u16,fld4: _17,fld5: (-168445110834842186424133358041724074136_i128) };
(*_2) = [_28.2,_28.2,_28.2];
(*_21).fld2 = 142_u8 & 191_u8;
(*_21).fld2 = 17_u8;
(*_21).fld5 = !(-85304012093146519334055253782967730593_i128);
(*_21).fld0 = [_8.fld5,(*_21).fld5];
(*_10) = _23 - _20;
(*_21).fld1 = !_13;
(*_21).fld3 = 13791_u16;
place!(Field::<i32>(Variant(_32, 2), 0)) = 11390914954791310838_usize as i32;
_6 = _3;
(*_21).fld1 = _13 * _13;
(*_21).fld2 = 25_u8 & 185_u8;
Goto(bb33)
}
bb33 = {
_37 = &(*_16);
(*_21).fld5 = 101229780588048699184198966787713289059_i128;
match (*_21).fld5 {
0 => bb23,
1 => bb16,
2 => bb22,
3 => bb30,
4 => bb34,
5 => bb35,
6 => bb36,
101229780588048699184198966787713289059 => bb38,
_ => bb37
}
}
bb34 = {
(*_10) = 88_isize;
_20 = !(*_10);
_8.fld0 = _12;
(*_10) = _20 | _20;
(*_10) = _20;
_12 = _8.fld0;
place!(Field::<char>(Variant(RET, 0), 1)) = _6;
(*_2) = [59_i8,(-20_i8),(-114_i8)];
place!(Field::<i128>(Variant(RET, 0), 0)) = !_8.fld5;
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
(*_10) = _20;
(*_2) = [111_i8,59_i8,79_i8];
(*_2) = [(-52_i8),(-42_i8),54_i8];
(*_10) = _20 ^ _20;
(*_10) = _14 as isize;
(*_16) = 998950148_u32;
match (*_16) {
0 => bb8,
1 => bb4,
2 => bb18,
3 => bb19,
998950148 => bb21,
_ => bb20
}
}
bb35 = {
RET = Adt20::Variant1 { fld0: (-5697_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 4610_i16;
RET = Adt20::Variant0 { fld0: (-153261025485873051903732354415593388842_i128),fld1: '\u{d1992}',fld2: 148_u8,fld3: 3690609270278159121_u64,fld4: (-625847117_i32) };
place!(Field::<char>(Variant(RET, 0), 1)) = '\u{61d97}';
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<i32>(Variant(RET, 0), 4)) = 1153387824_i32 + 1514316837_i32;
_3 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<u8>(Variant(RET, 0), 2)) = Field::<i32>(Variant(RET, 0), 4) as u8;
place!(Field::<u64>(Variant(RET, 0), 3)) = 14342085420480421436_u64 << Field::<i32>(Variant(RET, 0), 4);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
RET = Adt20::Variant1 { fld0: (-10286_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = -20799_i16;
_3 = '\u{16f52}';
_3 = '\u{97fa3}';
RET = Adt20::Variant1 { fld0: (-7483_i16) };
place!(Field::<i16>(Variant(RET, 1), 0)) = 9965_i16 + 22934_i16;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-9147_i16) | (-16776_i16);
RET = Adt20::Variant2 { fld0: (-537581354_i32) };
RET = Adt20::Variant1 { fld0: 21842_i16 };
_3 = '\u{4a281}';
RET = Adt20::Variant1 { fld0: 22121_i16 };
_5 = !2664626784_u32;
place!(Field::<i16>(Variant(RET, 1), 0)) = -15307_i16;
_3 = '\u{20aa6}';
Goto(bb3)
}
bb36 = {
_5 = 3761469346_u32 * 298799211_u32;
_6 = _3;
place!(Field::<i16>(Variant(RET, 1), 0)) = (-2041_i16);
_6 = _3;
_3 = _6;
place!(Field::<i16>(Variant(RET, 1), 0)) = 25980_i16 << _5;
_8.fld5 = _3 as i128;
place!(Field::<i16>(Variant(RET, 1), 0)) = -6637_i16;
Goto(bb4)
}
bb37 = {
_20 = -(*_10);
_8.fld1 = _13 + _13;
(*_10) = _20;
(*_16) = 3985656237_u32 << _8.fld2;
match _8.fld5 {
0 => bb7,
1 => bb4,
2 => bb5,
3 => bb24,
4 => bb25,
5 => bb26,
216697165417864158885710036188224748645 => bb28,
_ => bb27
}
}
bb38 = {
(*_16) = (*_21).fld5 as u32;
match (*_21).fld5 {
0 => bb24,
101229780588048699184198966787713289059 => bb40,
_ => bb39
}
}
bb39 = {
_13 = 231530495931530239483201142084438472276_u128 | 231798650891430137625402659936236985323_u128;
_12 = [_8.fld5,_8.fld5];
(*_10) = (-100_isize);
_2 = core::ptr::addr_of!(_11);
_14 = false;
(*_2) = [(-62_i8),113_i8,65_i8];
place!(Field::<u64>(Variant(RET, 0), 3)) = 8063928772419072164_u64;
(*_10) = 34_isize;
(*_10) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_2) = [(-110_i8),19_i8,85_i8];
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_2) = [(-99_i8),(-88_i8),(-114_i8)];
match Field::<i128>(Variant(RET, 0), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
216697165417864158885710036188224748645 => bb8,
_ => bb7
}
}
bb40 = {
(*_21).fld1 = _13 - _13;
(*_16) = !290827702_u32;
(*_21).fld5 = (-1880473701746615898096332491016587891_i128) + 12076139282391555204951873426843571693_i128;
(*_16) = !764759015_u32;
(*_21).fld2 = 122_u8;
(*_21).fld5 = !(-104735187209348397680107936556446738996_i128);
(*_21).fld3 = 28746_u16;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 220_u8,fld3: 46103_u16,fld4: _17,fld5: (-157455051368529894347416692343496825804_i128) };
(*_10) = _19 ^ _19;
_28.0 = &place!(Field::<i32>(Variant(_32, 2), 0));
(*_21).fld4 = _17 << (*_21).fld5;
(*_21).fld2 = (*_16) as u8;
_16 = core::ptr::addr_of!((*_16));
(*_21).fld1 = _13;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 66_u8,fld3: 24846_u16,fld4: _17,fld5: 63748510538375154825289387586249151358_i128 };
_42 = [_28.2,_28.2,_28.2];
(*_2) = [_28.2,_28.2,_28.2];
(*_21).fld3 = 36717_u16;
_16 = core::ptr::addr_of!((*_16));
(*_21).fld2 = !91_u8;
Goto(bb41)
}
bb41 = {
(*_10) = _19 - _19;
(*_10) = _19;
(*_21).fld3 = _20 as u16;
(*_16) = _8.fld5 as u32;
_38 = _25 + _25;
_20 = (*_10) ^ (*_10);
Call((*_2) = core::intrinsics::transmute(_42), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
(*_21).fld5 = !38002337505329085389918722631595046834_i128;
(*_10) = _20 + _20;
_8 = Adt19 { fld0: _12,fld1: _13,fld2: 249_u8,fld3: 5972_u16,fld4: _17,fld5: 157435957724199841678484799423864050542_i128 };
(*_16) = 77970213_u32 >> (*_10);
(*_21).fld2 = 83_u8 * 26_u8;
(*_16) = (*_21).fld5 as u32;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 104_u8,fld3: 14616_u16,fld4: _17,fld5: 108906984695673357867239121266564356902_i128 };
_8.fld1 = _13 | _13;
(*_16) = 3143600532_u32;
(*_21).fld5 = (-74110563461502241989352826107672975198_i128) >> (*_10);
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 219_u8,fld3: 58951_u16,fld4: _17,fld5: 24213001987737797246478235022910110019_i128 };
(*_21).fld2 = Field::<i32>(Variant(_32, 2), 0) as u8;
(*_21).fld5 = (*_21).fld1 as i128;
(*_21).fld3 = (-4127778875998007725_i64) as u16;
(*_10) = Field::<i32>(Variant(_32, 2), 0) as isize;
_26 = _3;
(*_21).fld5 = (*_21).fld4 as i128;
(*_21).fld2 = _13 as u8;
_2 = core::ptr::addr_of!(_11);
_46 = _14 as isize;
(*_21).fld4 = _17;
_38 = (-2250038333793645091_i64) as f64;
(*_2) = _42;
(*_21).fld5 = (-66750395337233383240529781711405574277_i128) >> (*_21).fld2;
(*_10) = -_19;
(*_21).fld1 = !_13;
(*_16) = _38 as u32;
(*_21).fld4 = -_17;
Call((*_21).fld4 = core::intrinsics::transmute((*_21).fld3), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_27 = (*_10) - (*_10);
(*_2) = [_28.2,_28.2,_28.2];
(*_21).fld2 = 52_u8 ^ 168_u8;
_46 = _19 ^ (*_10);
(*_21).fld1 = !_13;
_48 = (-3190296950114563201_i64) as f64;
(*_21).fld2 = 194_u8;
Goto(bb44)
}
bb44 = {
(*_16) = 150859263_u32 >> _46;
(*_21).fld3 = !21030_u16;
(*_16) = 3351661847_u32;
(*_21).fld1 = _13 ^ _13;
_29 = 1589937193524994991_i64 as f64;
(*_10) = _46;
(*_21).fld2 = 81_u8 << (*_21).fld1;
(*_21).fld1 = _28.2 as u128;
(*_21).fld1 = _13 - _13;
(*_2) = [_28.2,_28.2,_28.2];
_31 = &mut _14;
(*_21).fld3 = 2330_u16 * 137_u16;
(*_21).fld2 = 0_u8 * 94_u8;
_28.1 = core::ptr::addr_of_mut!((*_21).fld1);
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 219_u8,fld3: 57928_u16,fld4: _17,fld5: 143756653360621079728023288706938573239_i128 };
(*_21).fld4 = -_17;
_52.1 = (*_21).fld3 as f32;
(*_21).fld2 = 40_u8 - 170_u8;
(*_31) = !false;
(*_31) = !true;
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
_28.3 = _32;
(*_21).fld5 = 121714799125985344645477338990383546196_i128;
(*_21).fld2 = 13_u8 >> _46;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 104_u8,fld3: 31172_u16,fld4: _17,fld5: 1446836523387669631914553736847005834_i128 };
Goto(bb45)
}
bb45 = {
(*_2) = _42;
(*_10) = _27;
(*_16) = (*_21).fld4 as u32;
(*_2) = [_28.2,_28.2,_28.2];
_35 = (*_21).fld2 - (*_21).fld2;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: _35,fld3: 7632_u16,fld4: _17,fld5: (-152458623003292657564553413198053999650_i128) };
(*_21).fld1 = _13;
_28.3 = Adt20::Variant1 { fld0: (*_21).fld4 };
_5 = 3258765091_u32;
_8.fld5 = (-164523304039988560617789671978433536117_i128);
(*_10) = _46 * _46;
(*_10) = _27 >> (*_21).fld4;
(*_21).fld2 = _35 >> _20;
(*_2) = _42;
(*_21).fld1 = _13 ^ _13;
(*_21).fld5 = -150455047199655736742480570317567542541_i128;
(*_21).fld2 = _28.2 as u8;
(*_21).fld1 = _13;
(*_21).fld4 = Field::<i16>(Variant(_28.3, 1), 0) - Field::<i16>(Variant(_28.3, 1), 0);
_20 = (*_10) & (*_10);
Goto(bb46)
}
bb46 = {
(*_2) = _42;
(*_21).fld0 = _12;
(*_21).fld2 = _35 | _35;
(*_21).fld0 = [(*_21).fld5,_8.fld5];
(*_2) = [_28.2,_28.2,_28.2];
RET = Adt20::Variant2 { fld0: Field::<i32>(Variant(_32, 2), 0) };
(*_31) = true;
_12 = (*_21).fld0;
(*_21).fld2 = _35 + _35;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: _35,fld3: 57347_u16,fld4: Field::<i16>(Variant(_28.3, 1), 0),fld5: 43072655527098105639590894603274916669_i128 };
_29 = _25 + _48;
match (*_21).fld5 {
43072655527098105639590894603274916669 => bb48,
_ => bb47
}
}
bb47 = {
(*_21).fld5 = !147644928766747025893857979287745570527_i128;
(*_16) = 3894713921_u32;
(*_16) = (*_21).fld5 as u32;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 131_u8,fld3: 37569_u16,fld4: _17,fld5: (-73416285990681615941072678356619031400_i128) };
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 102_u8,fld3: 26183_u16,fld4: _17,fld5: (-59092053007301248760534164250213078682_i128) };
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
(*_21).fld4 = _17 << _19;
_38 = _25 + _25;
(*_21).fld1 = _13 + _13;
(*_10) = _14 as isize;
(*_21).fld3 = 767035236591120141_usize as u16;
_8.fld2 = (*_21).fld1 as u8;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: 203_u8,fld3: 63811_u16,fld4: _17,fld5: (-168445110834842186424133358041724074136_i128) };
(*_2) = [_28.2,_28.2,_28.2];
(*_21).fld2 = 142_u8 & 191_u8;
(*_21).fld2 = 17_u8;
(*_21).fld5 = !(-85304012093146519334055253782967730593_i128);
(*_21).fld0 = [_8.fld5,(*_21).fld5];
(*_10) = _23 - _20;
(*_21).fld1 = !_13;
(*_21).fld3 = 13791_u16;
place!(Field::<i32>(Variant(_32, 2), 0)) = 11390914954791310838_usize as i32;
_6 = _3;
(*_21).fld1 = _13 * _13;
(*_21).fld2 = 25_u8 & 185_u8;
Goto(bb33)
}
bb48 = {
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
(*_21).fld3 = !13217_u16;
(*_16) = 2705813123_u32 * 1586163221_u32;
(*_16) = 447648573_u32 - 3743001845_u32;
(*_21).fld4 = Field::<i16>(Variant(_28.3, 1), 0) << (*_21).fld5;
(*_10) = _46 - _46;
(*_10) = (*_16) as isize;
(*_21) = Adt19 { fld0: _12,fld1: _13,fld2: _35,fld3: 14890_u16,fld4: Field::<i16>(Variant(_28.3, 1), 0),fld5: 62120798747079242308153379321897337487_i128 };
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
(*_31) = !true;
(*_2) = [_28.2,_28.2,_28.2];
_47 = Field::<i32>(Variant(RET, 2), 0) as f32;
(*_21).fld0 = [(*_21).fld5,(*_21).fld5];
_6 = _26;
_27 = (*_10);
_44 = &mut (*_21).fld0;
(*_21).fld2 = _29 as u8;
(*_31) = true | true;
(*_21).fld2 = _35 + _35;
(*_21).fld4 = _17 << _28.2;
(*_21).fld5 = !(-157589327780681491337269724780768485455_i128);
(*_21).fld5 = 50191372608655731596309161729163940551_i128 & 94835293704233653072887718039217094862_i128;
(*_44) = [(*_21).fld5,(*_21).fld5];
(*_21).fld2 = !_35;
(*_10) = Field::<i32>(Variant(_32, 2), 0) as isize;
(*_21).fld2 = _52.1 as u8;
Goto(bb49)
}
bb49 = {
(*_44) = [(*_21).fld5,(*_21).fld5];
_32 = Adt20::Variant2 { fld0: Field::<i32>(Variant(RET, 2), 0) };
(*_16) = !1602835723_u32;
(*_10) = -_46;
_45 = [Field::<i32>(Variant(RET, 2), 0),Field::<i32>(Variant(RET, 2), 0),Field::<i32>(Variant(RET, 2), 0)];
(*_2) = _42;
(*_10) = _19;
(*_16) = 2075546697_u32 & 1421135853_u32;
(*_21).fld5 = 151246538997772751556178406187924649029_i128 << (*_21).fld1;
(*_21).fld2 = !_35;
(*_21).fld1 = !_13;
(*_21).fld4 = -Field::<i16>(Variant(_28.3, 1), 0);
(*_21).fld5 = -(-30049911547392768637643517592183844994_i128);
_18 = (*_21).fld1 as i128;
(*_31) = false;
(*_21).fld4 = Field::<i32>(Variant(RET, 2), 0) as i16;
(*_44) = _12;
(*_21).fld2 = _35 >> _20;
(*_2) = [_28.2,_28.2,_28.2];
(*_16) = _26 as u32;
(*_10) = _20 << _19;
(*_21).fld3 = 11624_u16;
(*_21).fld4 = !_17;
Goto(bb50)
}
bb50 = {
Call(_72 = dump_var(Move(_19), Move(_14), Move(_42), Move(_17)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_72 = dump_var(Move(_9), Move(_27), Move(_45), Move(_35)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_72 = dump_var(Move(_11), _73, _73, _73), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: (*const isize,),mut _2: bool,mut _3: [i128; 2],mut _4: &'static mut bool,mut _5: &'static mut [u128; 6],mut _6: bool,mut _7: [i128; 2]) -> Adt38 {
mir! {
type RET = Adt38;
let _8: (&'static [i8; 3],);
let _9: &'static mut &'static mut u8;
let _10: bool;
let _11: i128;
let _12: &'static Adt27;
let _13: [u32; 8];
let _14: (&'static &'static i64, &'static mut [u128; 6], f64, *const Adt19);
let _15: i128;
let _16: isize;
let _17: u8;
let _18: &'static mut [i128; 2];
let _19: f32;
let _20: u128;
let _21: char;
let _22: bool;
let _23: &'static &'static mut &'static u32;
let _24: bool;
let _25: char;
let _26: *const Adt19;
let _27: isize;
let _28: Adt31;
let _29: f32;
let _30: [u32; 8];
let _31: &'static mut Adt20;
let _32: *const Adt27;
let _33: &'static [char; 1];
let _34: isize;
let _35: (*const isize,);
let _36: [i128; 2];
let _37: f32;
let _38: bool;
let _39: i128;
let _40: u16;
let _41: *const &'static [i8; 3];
let _42: u64;
let _43: isize;
let _44: i8;
let _45: i128;
let _46: u8;
let _47: (&'static [i8; 3],);
let _48: (&'static i32, *mut u128, i8, Adt20);
let _49: Adt76;
let _50: char;
let _51: char;
let _52: bool;
let _53: &'static mut Adt20;
let _54: isize;
let _55: i128;
let _56: &'static &'static mut &'static u32;
let _57: f32;
let _58: i64;
let _59: f32;
let _60: Adt19;
let _61: bool;
let _62: isize;
let _63: f32;
let _64: [u32; 8];
let _65: *mut Adt38;
let _66: Adt20;
let _67: &'static u32;
let _68: i64;
let _69: i64;
let _70: [bool; 7];
let _71: u128;
let _72: u32;
let _73: &'static [i8; 3];
let _74: *const [i8; 3];
let _75: isize;
let _76: [i8; 4];
let _77: &'static mut &'static mut u8;
let _78: bool;
let _79: ([char; 1], Adt20);
let _80: &'static mut u8;
let _81: bool;
let _82: [u128; 6];
let _83: bool;
let _84: u8;
let _85: &'static mut Adt20;
let _86: *const Adt19;
let _87: i8;
let _88: *const u32;
let _89: &'static &'static i64;
let _90: *const &'static [i8; 3];
let _91: char;
let _92: f64;
let _93: *mut isize;
let _94: isize;
let _95: &'static Adt27;
let _96: &'static i32;
let _97: ();
let _98: ();
{
_6 = !_2;
_6 = _2 & _2;
_7 = [(-26924372717305651497168151517411746551_i128),(-11461729779430646784931854774904863411_i128)];
_2 = _6;
_6 = _2 != _2;
_6 = !_2;
_3 = [111963285746901913257496915381220644962_i128,(-144689158968700835359758835412579747901_i128)];
_3 = [4161535188327320607207900279057906374_i128,(-107825511862661849396960967413908380593_i128)];
_2 = _6;
_4 = &mut _2;
(*_4) = _6 <= _6;
(*_4) = _6 == _6;
(*_4) = _6 < _6;
_6 = (*_4);
(*_4) = _6 <= _6;
(*_4) = _6 >= _6;
_6 = (*_4) > (*_4);
(*_4) = _6;
(*_4) = _6 ^ _6;
(*_4) = _6 == _6;
(*_4) = _6;
(*_4) = _6 & _6;
(*_4) = _6;
(*_4) = _6 < _6;
Goto(bb1)
}
bb1 = {
(*_4) = !_6;
(*_4) = _6 <= _6;
_11 = 887246948911333381483544845206295887_i128;
(*_4) = _6 | _6;
(*_4) = !_6;
_4 = &mut _6;
(*_4) = _11 < _11;
(*_4) = true;
(*_4) = _11 < _11;
(*_4) = _11 == _11;
(*_4) = !true;
(*_4) = _11 != _11;
match _11 {
887246948911333381483544845206295887 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
(*_4) = !false;
(*_4) = _11 > _11;
match _11 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
887246948911333381483544845206295887 => bb10,
_ => bb9
}
}
bb4 = {
Return()
}
bb5 = {
(*_4) = !_6;
(*_4) = _6 <= _6;
_11 = 887246948911333381483544845206295887_i128;
(*_4) = _6 | _6;
(*_4) = !_6;
_4 = &mut _6;
(*_4) = _11 < _11;
(*_4) = true;
(*_4) = _11 < _11;
(*_4) = _11 == _11;
(*_4) = !true;
(*_4) = _11 != _11;
match _11 {
887246948911333381483544845206295887 => bb3,
_ => bb2
}
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
(*_4) = !true;
_18 = &mut _3;
(*_4) = _11 > _11;
_18 = &mut _7;
(*_18) = [_11,_11];
_14.2 = 3471447527272572991_u64 as f64;
(*_4) = true & false;
_20 = 38_i8 as u128;
_21 = '\u{44e88}';
_14.2 = 2955005116_u32 as f64;
(*_4) = false | false;
(*_18) = [_11,_11];
(*_4) = false;
(*_4) = !true;
match _11 {
0 => bb8,
1 => bb11,
887246948911333381483544845206295887 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
(*_4) = !true;
_15 = _11;
(*_4) = false;
match _11 {
0 => bb7,
1 => bb2,
2 => bb9,
887246948911333381483544845206295887 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
(*_4) = true;
(*_4) = _15 >= _15;
_14.2 = 246_u8 as f64;
_25 = _21;
(*_18) = [_11,_11];
_10 = (*_4) == (*_4);
(*_18) = [_15,_11];
(*_4) = !_10;
(*_4) = _20 < _20;
(*_18) = [_15,_11];
(*_18) = [_11,_11];
(*_18) = [_15,_11];
(*_18) = [_11,_11];
(*_18) = [_11,_15];
_20 = 55815166375208039641187571252019625185_u128 - 273133865007279302061644950394861412500_u128;
(*_18) = [_11,_15];
_22 = (*_4) == (*_4);
_17 = 199_u8 - 173_u8;
_22 = !(*_4);
(*_18) = [_15,_15];
_30 = [1565242809_u32,4133432938_u32,1900352576_u32,265053808_u32,896759710_u32,2132638250_u32,3089515688_u32,4142142624_u32];
(*_4) = _10;
(*_18) = [_11,_11];
(*_4) = _10;
_11 = _15;
(*_18) = [_11,_11];
_13 = [125114900_u32,2191442770_u32,1576200458_u32,1064550471_u32,3369136771_u32,2800135342_u32,3488963309_u32,1574804632_u32];
Goto(bb16)
}
bb16 = {
_13 = _30;
(*_18) = [_11,_11];
(*_4) = _10 < _10;
(*_18) = [_15,_15];
_30 = [3141484997_u32,3181773967_u32,1285237869_u32,3674700445_u32,3791580190_u32,638659842_u32,2957559767_u32,2887106939_u32];
(*_18) = [_15,_15];
(*_18) = [_15,_11];
_24 = (*_4) | (*_4);
_17 = !69_u8;
_30 = _13;
_20 = 126839053123904802999824109131800532715_u128 << _11;
(*_4) = _22;
(*_4) = _24 == _24;
(*_4) = _24;
(*_18) = [_15,_11];
_20 = 228241730364003581500346539680951839751_u128 | 329825207633747047140642319945299660084_u128;
_10 = _20 != _20;
_29 = (-45_i8) as f32;
(*_4) = !_24;
(*_18) = [_11,_15];
(*_4) = !_24;
(*_18) = [_15,_15];
_25 = _21;
(*_4) = _10;
_16 = (-417655846_i32) as isize;
(*_4) = _24;
(*_4) = _29 <= _29;
match _15 {
0 => bb8,
1 => bb2,
2 => bb13,
3 => bb4,
4 => bb15,
5 => bb6,
887246948911333381483544845206295887 => bb18,
_ => bb17
}
}
bb17 = {
Return()
}
bb18 = {
(*_18) = [_15,_11];
_1.0 = core::ptr::addr_of!(_27);
_11 = -_15;
(*_18) = [_11,_15];
_34 = _16 * _16;
(*_4) = _24 <= _10;
_14.2 = 55645_u16 as f64;
(*_4) = !_10;
_13 = [2179924226_u32,3227623983_u32,1355917999_u32,410719176_u32,2394597460_u32,4080610402_u32,1145501180_u32,3139175735_u32];
(*_18) = [_15,_11];
_35.0 = core::ptr::addr_of!(_16);
Goto(bb19)
}
bb19 = {
(*_4) = _24 != _24;
(*_18) = [_11,_15];
_16 = _34;
(*_4) = _34 != _16;
(*_18) = [_11,_15];
_22 = !(*_4);
_15 = !_11;
_38 = (*_4);
(*_4) = _22 ^ _24;
(*_4) = _24 != _22;
(*_18) = [_15,_15];
(*_4) = !_24;
(*_18) = [_15,_15];
(*_4) = !_24;
(*_4) = _24 < _24;
_11 = !_15;
(*_4) = _21 <= _21;
(*_18) = [_15,_11];
(*_4) = _24 ^ _24;
(*_4) = _38;
(*_18) = [_15,_11];
(*_4) = _38 < _24;
_21 = _25;
(*_18) = [_15,_11];
_1 = Move(_35);
(*_18) = [_11,_11];
_17 = _25 as u8;
_27 = _16;
_24 = (*_4);
Goto(bb20)
}
bb20 = {
_38 = (*_4);
(*_4) = _24 == _24;
(*_4) = !_24;
_35.0 = core::ptr::addr_of!(_34);
(*_4) = _34 >= _16;
_11 = _15 - _15;
_40 = !43076_u16;
_19 = -_29;
_37 = -_29;
Goto(bb21)
}
bb21 = {
_1.0 = core::ptr::addr_of!(_34);
Goto(bb22)
}
bb22 = {
_1.0 = core::ptr::addr_of!(_34);
(*_18) = [_15,_11];
_27 = _34 * _16;
(*_4) = _15 >= _15;
_39 = _15;
(*_18) = [_11,_11];
_16 = _27 & _34;
_29 = _19;
_10 = !_22;
_45 = _11;
(*_4) = _38 == _38;
_4 = &mut _22;
_40 = 41368_u16 | 55908_u16;
_16 = _17 as isize;
_36 = (*_18);
(*_18) = _36;
_35 = Move(_1);
(*_4) = _24 <= _38;
(*_18) = _36;
(*_18) = [_15,_11];
_34 = _27 * _27;
_15 = _11;
Goto(bb23)
}
bb23 = {
_18 = &mut _36;
(*_18) = [_45,_11];
(*_18) = [_15,_45];
_1 = (Move(_35.0),);
_25 = _21;
Goto(bb24)
}
bb24 = {
(*_18) = [_15,_11];
(*_4) = _34 <= _27;
_43 = _34 - _27;
_20 = 1202322975_i32 as u128;
Goto(bb25)
}
bb25 = {
(*_4) = _24 | _38;
(*_18) = [_39,_15];
(*_4) = !_24;
(*_4) = !_24;
_48.2 = (-37_i8);
_15 = _17 as i128;
_16 = _43 ^ _34;
_45 = -_11;
(*_4) = _38 | _38;
(*_18) = [_15,_11];
_17 = _20 as u8;
_40 = _16 as u16;
(*_4) = _24 ^ _24;
(*_4) = _27 != _27;
(*_4) = _16 > _16;
match _48.2 {
0 => bb18,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
340282366920938463463374607431768211419 => bb26,
_ => bb13
}
}
bb26 = {
(*_4) = _24 != _24;
(*_18) = [_39,_11];
(*_18) = [_15,_15];
_44 = 5317359704109051253_i64 as i8;
(*_4) = _24;
(*_18) = [_45,_11];
(*_4) = _24 & _24;
(*_4) = !_24;
Goto(bb27)
}
bb27 = {
_46 = !_17;
(*_4) = _24 | _24;
_42 = _40 as u64;
_35.0 = core::ptr::addr_of!(_16);
_48.1 = core::ptr::addr_of_mut!(_20);
_20 = 11577322483474337545867140125138222862_u128;
(*_4) = _38 & _24;
match _20 {
0 => bb6,
1 => bb28,
11577322483474337545867140125138222862 => bb30,
_ => bb29
}
}
bb28 = {
_1.0 = core::ptr::addr_of!(_34);
Goto(bb22)
}
bb29 = {
Return()
}
bb30 = {
_28 = Adt31::Variant3 { fld0: 1900848303_u32,fld1: Move(_48.1) };
_48.2 = _44 | _44;
_51 = _25;
(*_4) = _16 <= _34;
_35 = (Move(_1.0),);
(*_18) = [_45,_45];
_57 = -_19;
_44 = _21 as i8;
(*_4) = _24;
_48.1 = core::ptr::addr_of_mut!(_20);
(*_4) = _24 ^ _38;
_48.1 = core::ptr::addr_of_mut!(_20);
_58 = -3146964816318047368_i64;
_57 = _19 - _29;
(*_18) = [_15,_15];
(*_4) = _24;
_4 = &mut _38;
_16 = _34;
_27 = _43 * _34;
(*_4) = _24;
(*_4) = !_24;
_60.fld2 = !_46;
_52 = !(*_4);
(*_4) = !_24;
_26 = core::ptr::addr_of!(_60);
(*_26).fld3 = _40 * _40;
match _20 {
0 => bb31,
1 => bb32,
2 => bb33,
3 => bb34,
4 => bb35,
5 => bb36,
11577322483474337545867140125138222862 => bb38,
_ => bb37
}
}
bb31 = {
Return()
}
bb32 = {
_1.0 = core::ptr::addr_of!(_34);
Goto(bb22)
}
bb33 = {
Return()
}
bb34 = {
(*_4) = _24 != _24;
(*_18) = [_39,_11];
(*_18) = [_15,_15];
_44 = 5317359704109051253_i64 as i8;
(*_4) = _24;
(*_18) = [_45,_11];
(*_4) = _24 & _24;
(*_4) = !_24;
Goto(bb27)
}
bb35 = {
(*_4) = _24 | _38;
(*_18) = [_39,_15];
(*_4) = !_24;
(*_4) = !_24;
_48.2 = (-37_i8);
_15 = _17 as i128;
_16 = _43 ^ _34;
_45 = -_11;
(*_4) = _38 | _38;
(*_18) = [_15,_11];
_17 = _20 as u8;
_40 = _16 as u16;
(*_4) = _24 ^ _24;
(*_4) = _27 != _27;
(*_4) = _16 > _16;
match _48.2 {
0 => bb18,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
340282366920938463463374607431768211419 => bb26,
_ => bb13
}
}
bb36 = {
_1.0 = core::ptr::addr_of!(_34);
(*_18) = [_15,_11];
_27 = _34 * _16;
(*_4) = _15 >= _15;
_39 = _15;
(*_18) = [_11,_11];
_16 = _27 & _34;
_29 = _19;
_10 = !_22;
_45 = _11;
(*_4) = _38 == _38;
_4 = &mut _22;
_40 = 41368_u16 | 55908_u16;
_16 = _17 as isize;
_36 = (*_18);
(*_18) = _36;
_35 = Move(_1);
(*_4) = _24 <= _38;
(*_18) = _36;
(*_18) = [_15,_11];
_34 = _27 * _27;
_15 = _11;
Goto(bb23)
}
bb37 = {
_13 = _30;
(*_18) = [_11,_11];
(*_4) = _10 < _10;
(*_18) = [_15,_15];
_30 = [3141484997_u32,3181773967_u32,1285237869_u32,3674700445_u32,3791580190_u32,638659842_u32,2957559767_u32,2887106939_u32];
(*_18) = [_15,_15];
(*_18) = [_15,_11];
_24 = (*_4) | (*_4);
_17 = !69_u8;
_30 = _13;
_20 = 126839053123904802999824109131800532715_u128 << _11;
(*_4) = _22;
(*_4) = _24 == _24;
(*_4) = _24;
(*_18) = [_15,_11];
_20 = 228241730364003581500346539680951839751_u128 | 329825207633747047140642319945299660084_u128;
_10 = _20 != _20;
_29 = (-45_i8) as f32;
(*_4) = !_24;
(*_18) = [_11,_15];
(*_4) = !_24;
(*_18) = [_15,_15];
_25 = _21;
(*_4) = _10;
_16 = (-417655846_i32) as isize;
(*_4) = _24;
(*_4) = _29 <= _29;
match _15 {
0 => bb8,
1 => bb2,
2 => bb13,
3 => bb4,
4 => bb15,
5 => bb6,
887246948911333381483544845206295887 => bb18,
_ => bb17
}
}
bb38 = {
_34 = -_43;
(*_26).fld5 = _16 as i128;
(*_26) = Adt19 { fld0: (*_18),fld1: _20,fld2: _46,fld3: _40,fld4: (-20489_i16),fld5: _45 };
_44 = _48.2;
(*_26).fld0 = (*_18);
(*_26).fld4 = 15953_i16;
(*_26).fld5 = (*_26).fld3 as i128;
(*_18) = [(*_26).fld5,(*_26).fld5];
_55 = _21 as i128;
(*_26).fld3 = _44 as u16;
_50 = _21;
(*_26).fld3 = _40 << _34;
(*_26) = Adt19 { fld0: (*_18),fld1: _20,fld2: _17,fld3: _40,fld4: 7987_i16,fld5: _45 };
match (*_26).fld4 {
0 => bb5,
7987 => bb40,
_ => bb39
}
}
bb39 = {
_1.0 = core::ptr::addr_of!(_34);
(*_18) = [_15,_11];
_27 = _34 * _16;
(*_4) = _15 >= _15;
_39 = _15;
(*_18) = [_11,_11];
_16 = _27 & _34;
_29 = _19;
_10 = !_22;
_45 = _11;
(*_4) = _38 == _38;
_4 = &mut _22;
_40 = 41368_u16 | 55908_u16;
_16 = _17 as isize;
_36 = (*_18);
(*_18) = _36;
_35 = Move(_1);
(*_4) = _24 <= _38;
(*_18) = _36;
(*_18) = [_15,_11];
_34 = _27 * _27;
_15 = _11;
Goto(bb23)
}
bb40 = {
(*_26).fld5 = _45 ^ _45;
_64 = _30;
(*_26).fld2 = (*_26).fld4 as u8;
(*_26) = Adt19 { fld0: (*_18),fld1: _20,fld2: _46,fld3: _40,fld4: 2007_i16,fld5: _45 };
_43 = _27 & _27;
_54 = -_34;
_42 = !9463382047516571331_u64;
(*_26).fld0 = [(*_26).fld5,(*_26).fld5];
_37 = _57;
(*_26).fld5 = _39 * _55;
_50 = _25;
_11 = (*_26).fld5;
_24 = !(*_4);
(*_26).fld4 = -17217_i16;
(*_26).fld5 = _45 >> _43;
(*_26).fld4 = (*_26).fld3 as i16;
(*_26) = Adt19 { fld0: (*_18),fld1: _20,fld2: _17,fld3: _40,fld4: (-28616_i16),fld5: _11 };
(*_18) = (*_26).fld0;
(*_26).fld1 = _20;
_60.fld3 = _40 >> _43;
(*_26).fld1 = _20 >> (*_26).fld3;
(*_26) = Adt19 { fld0: (*_18),fld1: _20,fld2: _17,fld3: _40,fld4: 14230_i16,fld5: _45 };
(*_26).fld4 = 30483_i16 + 15010_i16;
_46 = (*_26).fld2;
_14.3 = Move(_26);
_48.2 = _44;
_4 = &mut _24;
_45 = _15 ^ _39;
Goto(bb41)
}
bb41 = {
_60.fld3 = _40 & _40;
(*_18) = [_45,_55];
_46 = 14417922545425646585_usize as u8;
_1 = (Move(_35.0),);
_68 = (*_4) as i64;
_65 = core::ptr::addr_of_mut!(RET);
(*_4) = !_52;
_59 = _37 * _19;
(*_18) = _60.fld0;
(*_18) = _60.fld0;
(*_18) = [_60.fld5,_60.fld5];
_60.fld4 = 17955_i16 ^ (-18746_i16);
_43 = _60.fld4 as isize;
_21 = _51;
_45 = _39 - _39;
(*_18) = _60.fld0;
(*_4) = _27 > _16;
_62 = _60.fld5 as isize;
_62 = _27 | _27;
place!(Field::<u32>(Variant(_28, 3), 0)) = 8241611500978975970_usize as u32;
place!(Field::<u32>(Variant(_28, 3), 0)) = 700423964_u32 << _27;
_27 = _54 - _62;
(*_4) = _52;
_57 = -_37;
_55 = _11 + _45;
Goto(bb42)
}
bb42 = {
_60.fld1 = _20 % _20;
(*_18) = [_55,_55];
(*_4) = _52;
_71 = _60.fld1;
_72 = _57 as u32;
(*_4) = _52 | _52;
_18 = &mut _60.fld0;
_21 = _51;
_42 = !15040832965326889842_u64;
_75 = _34 << _44;
(*_18) = [_15,_55];
(*_18) = [_55,_45];
_66 = Adt20::Variant2 { fld0: (-1767658400_i32) };
_70 = [(*_4),(*_4),(*_4),(*_4),(*_4),(*_4),(*_4)];
(*_4) = !_52;
(*_4) = _52 & _52;
(*_4) = _52;
_63 = -_19;
Goto(bb43)
}
bb43 = {
(*_18) = [_55,_55];
_40 = 11746_u16 - 38871_u16;
_55 = _15 ^ _11;
_16 = _27 | _27;
_69 = (*_4) as i64;
_26 = Move(_14.3);
_35.0 = core::ptr::addr_of!(_62);
_67 = &_72;
(*_4) = _52;
_79.1 = Adt20::Variant0 { fld0: _55,fld1: _50,fld2: _17,fld3: _42,fld4: (-1282782925_i32) };
_37 = _59;
(*_18) = [_45,Field::<i128>(Variant(_79.1, 0), 0)];
(*_18) = [_45,_45];
(*_4) = _52;
_67 = &place!(Field::<u32>(Variant(_28, 3), 0));
_1 = (Move(_35.0),);
place!(Field::<i32>(Variant(_79.1, 0), 4)) = 12615318767519608390_usize as i32;
(*_65) = Adt38::Variant1 { fld0: Field::<u64>(Variant(_79.1, 0), 3),fld1: _79.1,fld2: Move(_48.1) };
Goto(bb44)
}
bb44 = {
_48.3 = Field::<Adt20>(Variant((*_65), 1), 1);
_79.0 = [Field::<char>(Variant(Field::<Adt20>(Variant(RET, 1), 1), 0), 1)];
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 0)) = _55 | Field::<i128>(Variant(_79.1, 0), 0);
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 4)) = Field::<i32>(Variant(_48.3, 0), 4) + Field::<i32>(Variant(_79.1, 0), 4);
place!(Field::<Adt20>(Variant((*_65), 1), 1)) = Adt20::Variant0 { fld0: _55,fld1: Field::<char>(Variant(_48.3, 0), 1),fld2: Field::<u8>(Variant(_79.1, 0), 2),fld3: Field::<u64>(Variant((*_65), 1), 0),fld4: Field::<i32>(Variant(_48.3, 0), 4) };
(*_18) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0)];
place!(Field::<*mut u128>(Variant((*_65), 1), 2)) = core::ptr::addr_of_mut!(_71);
place!(Field::<i32>(Variant(_66, 2), 0)) = !Field::<i32>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 4);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 0)) = -Field::<i128>(Variant(_48.3, 0), 0);
place!(Field::<u8>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 2)) = Field::<u8>(Variant(_79.1, 0), 2);
place!(Field::<Adt20>(Variant((*_65), 1), 1)) = Adt20::Variant0 { fld0: Field::<i128>(Variant(_48.3, 0), 0),fld1: Field::<char>(Variant(_48.3, 0), 1),fld2: Field::<u8>(Variant(_48.3, 0), 2),fld3: Field::<u64>(Variant(RET, 1), 0),fld4: Field::<i32>(Variant(_79.1, 0), 4) };
_67 = &_72;
_28 = Adt31::Variant3 { fld0: (*_67),fld1: Move(Field::<*mut u128>(Variant((*_65), 1), 2)) };
Goto(bb45)
}
bb45 = {
_76 = [_48.2,_48.2,_44,_48.2];
place!(Field::<Adt20>(Variant((*_65), 1), 1)) = Adt20::Variant1 { fld0: 23457_i16 };
place!(Field::<i16>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 1), 0)) = (-25225_i16);
_10 = (*_4) | (*_4);
(*_18) = [Field::<i128>(Variant(_79.1, 0), 0),Field::<i128>(Variant(_79.1, 0), 0)];
_54 = _16;
place!(Field::<Adt20>(Variant((*_65), 1), 1)) = Adt20::Variant0 { fld0: _39,fld1: _25,fld2: Field::<u8>(Variant(_79.1, 0), 2),fld3: Field::<u64>(Variant(_48.3, 0), 3),fld4: Field::<i32>(Variant(_66, 2), 0) };
(*_4) = _54 == _27;
_88 = core::ptr::addr_of!((*_67));
(*_4) = !_10;
place!(Field::<*mut u128>(Variant((*_65), 1), 2)) = Move(Field::<*mut u128>(Variant(_28, 3), 1));
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 0)) = Field::<i128>(Variant(_48.3, 0), 0) >> (*_67);
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 3)) = _42;
place!(Field::<u64>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 3)) = Field::<u64>(Variant((*_65), 1), 0);
(*_18) = [Field::<i128>(Variant(_48.3, 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0)];
_86 = Move(_26);
place!(Field::<char>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 1)) = _51;
_11 = -Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0);
place!(Field::<u8>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 2)) = !Field::<u8>(Variant(_48.3, 0), 2);
_35.0 = core::ptr::addr_of!(_54);
_53 = &mut _48.3;
place!(Field::<u64>(Variant((*_65), 1), 0)) = !Field::<u64>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 3);
place!(Field::<i32>(Variant((*_53), 0), 4)) = Field::<i32>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 4) + Field::<i32>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 4);
(*_18) = [Field::<i128>(Variant((*_53), 0), 0),Field::<i128>(Variant((*_53), 0), 0)];
match _20 {
0 => bb15,
1 => bb10,
11577322483474337545867140125138222862 => bb46,
_ => bb19
}
}
bb46 = {
_31 = &mut (*_53);
(*_31) = Adt20::Variant1 { fld0: (-23735_i16) };
(*_18) = [Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0),Field::<i128>(Variant(Field::<Adt20>(Variant((*_65), 1), 1), 0), 0)];
place!(Field::<i32>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 4)) = -Field::<i32>(Variant(_66, 2), 0);
place!(Field::<i128>(Variant(place!(Field::<Adt20>(Variant((*_65), 1), 1)), 0), 0)) = (*_4) as i128;
_87 = !_44;
Goto(bb47)
}
bb47 = {
Call(_97 = dump_var(Move(_51), Move(_70), Move(_71), Move(_52)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_97 = dump_var(Move(_20), Move(_55), Move(_2), Move(_17)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_97 = dump_var(Move(_62), Move(_58), Move(_24), Move(_10)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_97 = dump_var(Move(_13), Move(_64), Move(_69), Move(_45)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_97 = dump_var(Move(_42), Move(_15), Move(_25), Move(_6)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_97 = dump_var(Move(_36), _98, _98, _98), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{57800}'), std::hint::black_box(11301049451302817183449518571618048589_u128), std::hint::black_box(5_i8), std::hint::black_box(22825_i16), std::hint::black_box(1264455708_i32), std::hint::black_box(12696722524495988550_u64), std::hint::black_box((-159470928134148252600943643156378644605_i128)), std::hint::black_box(2_usize), std::hint::black_box(220_u8), std::hint::black_box(25988_u16), std::hint::black_box(4165563999_u32));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub struct Adt19 {
fld0: [i128; 2],
fld1: u128,
fld2: u8,
fld3: u16,
fld4: i16,
fld5: i128,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt20 {
Variant0{
fld0: i128,
fld1: char,
fld2: u8,
fld3: u64,
fld4: i32,

},
Variant1{
fld0: i16,

},
Variant2{
fld0: i32,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt23 {
Variant0{
fld0: u128,
fld1: i128,

},
Variant1{
fld0: i64,

},
Variant2{
fld0: bool,
fld1: u16,
fld2: i64,
fld3: usize,
fld4: i16,
fld5: [i8; 3],

}}
#[derive(Debug)]
pub struct Adt27 {
fld0: f64,
fld1: Adt20,
fld2: u8,
fld3: u64,
fld4: i16,
fld5: f32,
}
#[derive(Debug)]
pub enum Adt31 {
Variant0{
fld0: i64,
fld1: u16,
fld2: [u128; 6],
fld3: f32,

},
Variant1{
fld0: [u128; 6],
fld1: Adt19,

},
Variant2{
fld0: *mut u128,
fld1: (u64, [i128; 2]),
fld2: Adt27,
fld3: i128,
fld4: u8,
fld5: i32,
fld6: Adt23,

},
Variant3{
fld0: u32,
fld1: *mut u128,

}}
#[derive(Debug)]
pub enum Adt38 {
Variant0{
fld0: bool,
fld1: char,
fld2: *const Adt20,
fld3: i8,
fld4: i16,
fld5: u8,
fld6: Adt19,
fld7: u16,

},
Variant1{
fld0: u64,
fld1: Adt20,
fld2: *mut u128,

}}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: *const Adt20,
fld1: [u128; 6],
fld2: Adt20,

},
Variant1{
fld0: *const Adt19,
fld1: [u32; 8],

},
Variant2{
fld0: *mut f32,
fld1: [i32; 3],
fld2: [i8; 4],

}}
#[derive(Debug)]
pub enum Adt76 {
Variant0{
fld0: f32,
fld1: Adt31,
fld2: [i128; 2],
fld3: i32,

},
Variant1{
fld0: Adt27,
fld1: [usize; 4],
fld2: [u32; 8],
fld3: *const [i8; 3],
fld4: *mut f32,
fld5: *const isize,
fld6: [i8; 4],
fld7: Adt62,

},
Variant2{
fld0: [i8; 4],
fld1: char,
fld2: i32,
fld3: ([char; 1], Adt20),

},
Variant3{
fld0: *const [i8; 3],
fld1: char,
fld2: Adt38,
fld3: [char; 1],
fld4: Adt19,
fld5: *const Adt20,
fld6: i64,

}}

