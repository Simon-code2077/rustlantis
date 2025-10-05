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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: isize,mut _4: u128,mut _5: i16,mut _6: u64,mut _7: i64,mut _8: u16,mut _9: usize) -> f64 {
mir! {
type RET = f64;
let _10: i64;
let _11: [i16; 2];
let _12: char;
let _13: u128;
let _14: isize;
let _15: f32;
let _16: bool;
let _17: Adt45;
let _18: Adt47;
let _19: u32;
let _20: [i16; 2];
let _21: [usize; 3];
let _22: &'static mut i16;
let _23: *const char;
let _24: u64;
let _25: &'static mut [i128; 6];
let _26: *mut u32;
let _27: (u8, usize, i128, i128);
let _28: *mut u32;
let _29: bool;
let _30: i16;
let _31: *const [u8; 7];
let _32: [i64; 4];
let _33: &'static (u32, f32, bool);
let _34: usize;
let _35: &'static mut *mut [u64; 5];
let _36: (&'static mut i16, *mut [u64; 5]);
let _37: u64;
let _38: i64;
let _39: *mut u32;
let _40: [i32; 8];
let _41: u128;
let _42: &'static mut *mut [u64; 5];
let _43: [u16; 3];
let _44: bool;
let _45: i64;
let _46: Adt25;
let _47: &'static mut &'static mut bool;
let _48: f64;
let _49: isize;
let _50: &'static mut *mut [u64; 5];
let _51: *const char;
let _52: *mut i32;
let _53: isize;
let _54: char;
let _55: *mut u32;
let _56: [u16; 3];
let _57: &'static Adt19;
let _58: char;
let _59: bool;
let _60: *mut i128;
let _61: usize;
let _62: *const &'static mut i16;
let _63: f64;
let _64: isize;
let _65: (u8, usize, i128, i128);
let _66: bool;
let _67: bool;
let _68: u128;
let _69: ();
let _70: ();
{
_6 = 18433125084064169154_u64;
_1 = false;
_9 = 4_usize & 11644052576953308410_usize;
_8 = 31147_u16;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
18433125084064169154 => bb6,
_ => bb5
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
_7 = !1681823609853586879_i64;
_3 = !(-9223372036854775808_isize);
_8 = 1732785701_i32 as u16;
_5 = (-30286_i16) ^ (-16587_i16);
_4 = '\u{f665}' as u128;
RET = _4 as f64;
_1 = _7 <= _7;
_12 = '\u{955f7}';
RET = _4 as f64;
_6 = 3397153249988381028_u64 & 9478567957172497787_u64;
_10 = _7 ^ _7;
_2 = 51_u8 >> _4;
_1 = _3 >= _3;
_6 = 4611268302970989995_u64;
RET = (-744483429_i32) as f64;
_10 = -_7;
_3 = !26_isize;
_5 = 22510_i16;
_14 = _3 ^ _3;
_6 = 9825643186524657896_u64;
_13 = _14 as u128;
_8 = !20452_u16;
_7 = _10 + _10;
_7 = 24121537_i32 as i64;
Call(_4 = fn1(_8, _8, _7, RET, _14, _3, _14, _8, _13, _13, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = 9268243939272325165_u64 ^ 8186245013232994985_u64;
_3 = !_14;
_6 = 9582776533216563389_u64 - 11955736840215125324_u64;
_11 = [_5,_5];
_6 = 5465974866180934857_u64 | 1137869931450770785_u64;
_15 = _8 as f32;
_4 = _13 | _13;
Goto(bb8)
}
bb8 = {
_7 = (-111_i8) as i64;
_4 = _13 & _13;
Goto(bb9)
}
bb9 = {
_1 = false | true;
_2 = !70_u8;
match _5 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb7,
4 => bb5,
22510 => bb10,
_ => bb6
}
}
bb10 = {
_12 = '\u{fa5ef}';
_8 = 22997_u16 | 1123_u16;
_16 = _10 == _10;
_13 = _4;
_1 = _2 != _2;
_12 = '\u{9809c}';
match _5 {
0 => bb8,
22510 => bb11,
_ => bb3
}
}
bb11 = {
_9 = 17669732717633744212_usize;
_18.fld5.fld5 = 560807287_u32;
_18.fld1 = _12;
_18.fld5.fld2 = !_9;
_3 = !_14;
_4 = _13;
_9 = _3 as usize;
_18.fld6 = Adt19::Variant1 { fld0: 78_i8,fld1: 127909324728381331278464997049188524213_i128,fld2: (-1225289316_i32) };
_20 = _11;
_18.fld5.fld4 = (-61_i8) as u16;
_18.fld0.0 = _18.fld1;
_18.fld2 = _18.fld5.fld5 as f32;
place!(Field::<i8>(Variant(_18.fld6, 1), 0)) = (-107_i8) + 63_i8;
_18.fld5.fld0 = _16;
_3 = _14 | _14;
_21 = [_9,_9,_18.fld5.fld2];
Call(_18.fld5.fld1 = fn2(_18.fld5.fld5, _18.fld5.fld5, _2, _14, _18.fld5.fld0, _5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_18.fld5 = Adt25 { fld0: _1,fld1: _18.fld2,fld2: _9,fld3: Field::<i8>(Variant(_18.fld6, 1), 0),fld4: _8,fld5: 2285373212_u32 };
_4 = _13 << _8;
_15 = _18.fld2 * _18.fld2;
_18.fld5.fld4 = !_8;
_24 = _6;
match _18.fld5.fld5 {
0 => bb4,
2285373212 => bb14,
_ => bb13
}
}
bb13 = {
_12 = '\u{fa5ef}';
_8 = 22997_u16 | 1123_u16;
_16 = _10 == _10;
_13 = _4;
_1 = _2 != _2;
_12 = '\u{9809c}';
match _5 {
0 => bb8,
22510 => bb11,
_ => bb3
}
}
bb14 = {
_18.fld0.0 = _18.fld1;
_18.fld0.0 = _18.fld1;
_14 = _3 << _13;
_3 = _14 * _14;
_18.fld4 = -_5;
Goto(bb15)
}
bb15 = {
_18.fld3 = 86056255743380513201815560701949098380_i128 as u64;
_18.fld4 = _5;
_27.3 = -(-162388920088193599074206872704912816077_i128);
_23 = core::ptr::addr_of!(_12);
_2 = !221_u8;
place!(Field::<i128>(Variant(_18.fld6, 1), 1)) = !_27.3;
place!(Field::<i8>(Variant(_18.fld6, 1), 0)) = !_18.fld5.fld3;
_10 = _7 + _7;
_26 = core::ptr::addr_of_mut!(_18.fld5.fld5);
(*_23) = _18.fld0.0;
_27.2 = -Field::<i128>(Variant(_18.fld6, 1), 1);
_13 = _4 << Field::<i8>(Variant(_18.fld6, 1), 0);
(*_23) = _18.fld0.0;
_18.fld0.0 = (*_23);
(*_26) = 1325078971_u32;
_18.fld6 = Adt19::Variant1 { fld0: _18.fld5.fld3,fld1: _27.2,fld2: (-975601628_i32) };
(*_23) = _18.fld0.0;
(*_23) = _18.fld1;
_27 = (_2, _18.fld5.fld2, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
_2 = (*_23) as u8;
_27 = (_2, _18.fld5.fld2, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
(*_23) = _18.fld0.0;
_9 = !_18.fld5.fld2;
_5 = _18.fld4 + _18.fld4;
match (*_26) {
0 => bb8,
1 => bb13,
2 => bb10,
1325078971 => bb17,
_ => bb16
}
}
bb16 = {
Return()
}
bb17 = {
(*_23) = _18.fld0.0;
_22 = &mut _5;
(*_26) = RET as u32;
place!(Field::<i128>(Variant(_18.fld6, 1), 1)) = _27.2;
(*_22) = _18.fld4 | _18.fld4;
(*_23) = _18.fld0.0;
_28 = core::ptr::addr_of_mut!((*_26));
(*_28) = 325986856_u32 ^ 1553555571_u32;
Goto(bb18)
}
bb18 = {
(*_28) = 488577470_u32;
RET = _24 as f64;
(*_23) = _18.fld0.0;
(*_26) = !231061580_u32;
_9 = !_18.fld5.fld2;
(*_23) = _18.fld0.0;
(*_23) = _18.fld0.0;
_13 = !_4;
_26 = core::ptr::addr_of_mut!((*_26));
(*_26) = 1513781059_u32 * 3490924918_u32;
_34 = _9 << _18.fld5.fld5;
(*_22) = _18.fld4 ^ _18.fld4;
(*_26) = 922269075_u32 >> (*_22);
_19 = (*_26) | (*_26);
_19 = (*_26) | (*_26);
(*_22) = _18.fld4;
(*_26) = _19 & _19;
(*_23) = _18.fld1;
_30 = _18.fld5.fld3 as i16;
(*_23) = _18.fld1;
_18.fld7 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_18.fld6, 1), 2)));
(*_26) = _19;
_19 = 1192948382_i32 as u32;
Call(_19 = core::intrinsics::bswap((*_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
(*_26) = Field::<i8>(Variant(_18.fld6, 1), 0) as u32;
Goto(bb20)
}
bb20 = {
(*_23) = _18.fld1;
(*_26) = _10 as u32;
(*_26) = _19;
(*_22) = _10 as i16;
_29 = !_18.fld5.fld0;
_24 = !_18.fld3;
(*_26) = _19 ^ _19;
(*_26) = _19 * _19;
match _18.fld4 {
0 => bb7,
1 => bb10,
2 => bb21,
3 => bb22,
22510 => bb24,
_ => bb23
}
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
_18.fld3 = 86056255743380513201815560701949098380_i128 as u64;
_18.fld4 = _5;
_27.3 = -(-162388920088193599074206872704912816077_i128);
_23 = core::ptr::addr_of!(_12);
_2 = !221_u8;
place!(Field::<i128>(Variant(_18.fld6, 1), 1)) = !_27.3;
place!(Field::<i8>(Variant(_18.fld6, 1), 0)) = !_18.fld5.fld3;
_10 = _7 + _7;
_26 = core::ptr::addr_of_mut!(_18.fld5.fld5);
(*_23) = _18.fld0.0;
_27.2 = -Field::<i128>(Variant(_18.fld6, 1), 1);
_13 = _4 << Field::<i8>(Variant(_18.fld6, 1), 0);
(*_23) = _18.fld0.0;
_18.fld0.0 = (*_23);
(*_26) = 1325078971_u32;
_18.fld6 = Adt19::Variant1 { fld0: _18.fld5.fld3,fld1: _27.2,fld2: (-975601628_i32) };
(*_23) = _18.fld0.0;
(*_23) = _18.fld1;
_27 = (_2, _18.fld5.fld2, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
_2 = (*_23) as u8;
_27 = (_2, _18.fld5.fld2, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
(*_23) = _18.fld0.0;
_9 = !_18.fld5.fld2;
_5 = _18.fld4 + _18.fld4;
match (*_26) {
0 => bb8,
1 => bb13,
2 => bb10,
1325078971 => bb17,
_ => bb16
}
}
bb24 = {
_18.fld5 = Adt25 { fld0: _29,fld1: _15,fld2: _9,fld3: Field::<i8>(Variant(_18.fld6, 1), 0),fld4: _8,fld5: _19 };
_6 = !_24;
_29 = (*_23) <= (*_23);
_27 = (_2, _34, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
(*_23) = _18.fld0.0;
_27.1 = _18.fld5.fld2 * _34;
_18.fld2 = _15 - _15;
Goto(bb25)
}
bb25 = {
_34 = !_27.1;
(*_26) = _19 & _19;
(*_23) = _18.fld0.0;
(*_22) = _30;
place!(Field::<i128>(Variant(_18.fld6, 1), 1)) = _27.3;
(*_22) = -_18.fld4;
(*_22) = !_30;
_18.fld5.fld4 = (*_26) as u16;
(*_22) = -_18.fld4;
(*_22) = -_30;
_27 = (_2, _34, Field::<i128>(Variant(_18.fld6, 1), 1), Field::<i128>(Variant(_18.fld6, 1), 1));
_38 = _10;
_36.0 = &mut (*_22);
_15 = _18.fld2 + _18.fld2;
(*_23) = _18.fld0.0;
(*_26) = _19;
_4 = _13 ^ _13;
_30 = _9 as i16;
(*_26) = _19 & _19;
(*_26) = _19 | _19;
_27.2 = !_27.3;
_18.fld5.fld0 = (*_26) == (*_26);
(*_23) = _18.fld1;
_18.fld2 = _15 - _15;
_18.fld0.0 = (*_23);
match _18.fld4 {
0 => bb26,
22510 => bb28,
_ => bb27
}
}
bb26 = {
Return()
}
bb27 = {
_7 = !1681823609853586879_i64;
_3 = !(-9223372036854775808_isize);
_8 = 1732785701_i32 as u16;
_5 = (-30286_i16) ^ (-16587_i16);
_4 = '\u{f665}' as u128;
RET = _4 as f64;
_1 = _7 <= _7;
_12 = '\u{955f7}';
RET = _4 as f64;
_6 = 3397153249988381028_u64 & 9478567957172497787_u64;
_10 = _7 ^ _7;
_2 = 51_u8 >> _4;
_1 = _3 >= _3;
_6 = 4611268302970989995_u64;
RET = (-744483429_i32) as f64;
_10 = -_7;
_3 = !26_isize;
_5 = 22510_i16;
_14 = _3 ^ _3;
_6 = 9825643186524657896_u64;
_13 = _14 as u128;
_8 = !20452_u16;
_7 = _10 + _10;
_7 = 24121537_i32 as i64;
Call(_4 = fn1(_8, _8, _7, RET, _14, _3, _14, _8, _13, _13, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb28 = {
_27.2 = _27.3 & _27.3;
_18.fld1 = (*_23);
_38 = (*_23) as i64;
_41 = _13 << _3;
_14 = _3 ^ _3;
(*_23) = _18.fld1;
_41 = !_4;
_24 = _18.fld3 * _6;
_6 = !_18.fld3;
(*_23) = _18.fld1;
(*_26) = _8 as u32;
_38 = RET as i64;
_4 = _41;
_10 = _7;
_15 = _18.fld2;
(*_26) = _19;
_37 = _24 - _18.fld3;
_18.fld5.fld1 = _18.fld2;
_37 = _30 as u64;
(*_26) = _19;
_18.fld5.fld5 = _19;
(*_23) = _18.fld1;
_24 = !_18.fld3;
(*_23) = _18.fld0.0;
match _18.fld4 {
0 => bb19,
1 => bb20,
2 => bb13,
3 => bb4,
22510 => bb30,
_ => bb29
}
}
bb29 = {
(*_28) = 488577470_u32;
RET = _24 as f64;
(*_23) = _18.fld0.0;
(*_26) = !231061580_u32;
_9 = !_18.fld5.fld2;
(*_23) = _18.fld0.0;
(*_23) = _18.fld0.0;
_13 = !_4;
_26 = core::ptr::addr_of_mut!((*_26));
(*_26) = 1513781059_u32 * 3490924918_u32;
_34 = _9 << _18.fld5.fld5;
(*_22) = _18.fld4 ^ _18.fld4;
(*_26) = 922269075_u32 >> (*_22);
_19 = (*_26) | (*_26);
_19 = (*_26) | (*_26);
(*_22) = _18.fld4;
(*_26) = _19 & _19;
(*_23) = _18.fld1;
_30 = _18.fld5.fld3 as i16;
(*_23) = _18.fld1;
_18.fld7 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_18.fld6, 1), 2)));
(*_26) = _19;
_19 = 1192948382_i32 as u32;
Call(_19 = core::intrinsics::bswap((*_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb30 = {
(*_26) = !_19;
(*_26) = _19 << _14;
(*_26) = _19;
_18.fld0.0 = (*_23);
(*_23) = _18.fld0.0;
_37 = _6 | _18.fld3;
_18.fld5.fld2 = _34 + _27.1;
(*_23) = _18.fld0.0;
(*_23) = _18.fld1;
_44 = !_29;
(*_23) = _18.fld0.0;
place!(Field::<i8>(Variant(_18.fld6, 1), 0)) = _18.fld5.fld3;
(*_23) = _18.fld0.0;
_18.fld4 = _30;
_18.fld0.1 = Adt19::Variant1 { fld0: _18.fld5.fld3,fld1: _27.2,fld2: 2010898767_i32 };
(*_26) = _19 >> _34;
_46.fld1 = -_15;
(*_26) = _19 - _19;
Goto(bb31)
}
bb31 = {
(*_26) = !_19;
(*_26) = _19 + _19;
_26 = core::ptr::addr_of_mut!((*_26));
_27.0 = !_2;
_1 = !_18.fld5.fld0;
(*_23) = _18.fld0.0;
_46.fld1 = _18.fld2;
_18.fld5.fld1 = _10 as f32;
(*_23) = _18.fld0.0;
_1 = _16;
_20 = _11;
_3 = RET as isize;
_18.fld0.1 = Adt19::Variant1 { fld0: _18.fld5.fld3,fld1: Field::<i128>(Variant(_18.fld6, 1), 1),fld2: (-101427602_i32) };
Goto(bb32)
}
bb32 = {
_40 = [174861655_i32,(-606408803_i32),1636276051_i32,1624600901_i32,362472228_i32,1583398441_i32,1941474078_i32,362637274_i32];
Goto(bb33)
}
bb33 = {
_46.fld5 = !(*_26);
_30 = -_18.fld4;
_18.fld2 = -_46.fld1;
_41 = _14 as u128;
_37 = _24 << _41;
(*_26) = _41 as u32;
_18.fld5.fld3 = !Field::<i8>(Variant(_18.fld0.1, 1), 0);
_6 = _18.fld3;
_18.fld2 = _46.fld1 * _46.fld1;
_18.fld7 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_18.fld0.1, 1), 2)));
_27.0 = !_2;
_41 = _4;
_19 = !(*_26);
_27.3 = _46.fld1 as i128;
(*_26) = _27.3 as u32;
(*_26) = _46.fld5 - _19;
_18.fld5.fld4 = _27.3 as u16;
_8 = _14 as u16;
_48 = _46.fld1 as f64;
_10 = _38 << (*_26);
(*_23) = _18.fld1;
(*_23) = _18.fld1;
_23 = core::ptr::addr_of!((*_23));
_9 = _14 as usize;
_15 = _18.fld2 + _46.fld1;
_45 = (*_26) as i64;
Goto(bb34)
}
bb34 = {
(*_23) = _18.fld0.0;
_39 = core::ptr::addr_of_mut!((*_26));
(*_23) = _18.fld0.0;
_3 = _14;
(*_26) = _19 - _19;
_3 = _14;
_34 = _9;
_40 = [(-823168913_i32),640834262_i32,208728653_i32,(-1325378435_i32),222423515_i32,(-1839217658_i32),1913841960_i32,1548897562_i32];
(*_23) = _18.fld1;
_18.fld5.fld5 = _19 << _9;
_23 = core::ptr::addr_of!(_54);
_16 = (*_26) == _18.fld5.fld5;
_27.2 = _27.3 | Field::<i128>(Variant(_18.fld0.1, 1), 1);
_46.fld3 = Field::<i8>(Variant(_18.fld0.1, 1), 0) << (*_26);
_32 = [_45,_45,_10,_45];
(*_23) = _12;
_26 = core::ptr::addr_of_mut!((*_26));
_27.1 = _34;
_13 = _4;
_46.fld0 = (*_26) < (*_26);
_26 = core::ptr::addr_of_mut!((*_26));
(*_23) = _18.fld0.0;
Call(_45 = core::intrinsics::bswap(_10), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
(*_23) = _18.fld0.0;
Goto(bb36)
}
bb36 = {
_18.fld5.fld2 = _15 as usize;
_36.0 = &mut _18.fld4;
_1 = (*_26) >= (*_26);
(*_26) = _19 & _46.fld5;
_46.fld0 = _16;
RET = _48 - _48;
_23 = core::ptr::addr_of!((*_23));
(*_23) = _12;
(*_23) = _12;
_8 = 35859_u16;
_46.fld5 = (*_26) + (*_26);
_19 = (*_26) ^ (*_26);
_6 = (-1258367182_i32) as u64;
_55 = Move(_26);
_26 = core::ptr::addr_of_mut!(_19);
_14 = _3;
_46.fld4 = _8 >> _19;
_26 = Move(_39);
_64 = _14 | _3;
_40 = [(-1038261066_i32),(-1671129471_i32),(-1234351132_i32),(-925584290_i32),2057373105_i32,914429432_i32,2025974859_i32,(-900830409_i32)];
_26 = core::ptr::addr_of_mut!(_46.fld5);
_51 = core::ptr::addr_of!((*_23));
_46.fld2 = _9 * _9;
(*_26) = _19 ^ _19;
_60 = core::ptr::addr_of_mut!(_65.2);
_23 = core::ptr::addr_of!((*_51));
(*_60) = !_27.3;
Goto(bb37)
}
bb37 = {
Call(_69 = dump_var(Move(_2), Move(_38), Move(_6), Move(_40)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_69 = dump_var(Move(_1), Move(_30), Move(_12), Move(_9)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_69 = dump_var(Move(_16), Move(_64), Move(_27), Move(_20)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_69 = dump_var(Move(_37), Move(_5), Move(_34), Move(_7)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u16,mut _2: u16,mut _3: i64,mut _4: f64,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: u16,mut _9: u128,mut _10: u128,mut _11: u64) -> u128 {
mir! {
type RET = u128;
let _12: *const (u8, usize, i128, i128);
let _13: *const &'static &'static mut bool;
let _14: *const (u8, usize, i128, i128);
let _15: i16;
let _16: *const (u8, usize, i128, i128);
let _17: *const (u8, usize, i128, i128);
let _18: &'static mut &'static mut bool;
let _19: *const u128;
let _20: &'static Adt19;
let _21: bool;
let _22: ();
let _23: ();
{
RET = _10;
_3 = 1880154598_u32 as i64;
_11 = !3105566909396818767_u64;
_10 = RET - _9;
Call(_1 = core::intrinsics::bswap(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 10299820400876028184_u64;
RET = _10 >> _11;
_11 = 54_u8 as u64;
_11 = 7128154497677302650_u64 * 17280371158247019387_u64;
_6 = _5 >> RET;
_2 = !_1;
_11 = 5680321735241770615_u64 ^ 8519103576996133921_u64;
_11 = 739528369829106658_u64;
_11 = !1327069310636875409_u64;
_10 = _9 << _6;
RET = !_10;
_3 = 1198780825338640521_i64 ^ 6142673796805083792_i64;
_6 = !_5;
_4 = (-71084737037896442410622643609490346027_i128) as f64;
Goto(bb2)
}
bb2 = {
_9 = RET;
_6 = (-1668802441_i32) as isize;
_5 = _7;
_3 = (-8959913702590497774_i64) * 1507302179354540432_i64;
_7 = _9 as isize;
_10 = _3 as u128;
_11 = 18395832730214694086_u64;
_6 = _7 ^ _7;
_6 = _5 >> _3;
_3 = 5941924320281453894_i64 | 2746328459548920233_i64;
_1 = _4 as u16;
_8 = _2 + _2;
_5 = _6;
_2 = 1316789415_u32 as u16;
_1 = _8 * _8;
_10 = _9 >> _5;
_15 = !(-10478_i16);
_15 = !(-10930_i16);
_2 = 27_i8 as u16;
_1 = _2 ^ _8;
_8 = !_1;
_3 = _11 as i64;
Call(_1 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _9 - RET;
Goto(bb4)
}
bb4 = {
Goto(bb5)
}
bb5 = {
_19 = core::ptr::addr_of!(_10);
RET = (*_19) - (*_19);
_10 = _9;
_2 = !_1;
(*_19) = RET | RET;
_7 = _6 & _5;
(*_19) = RET;
Goto(bb6)
}
bb6 = {
Call(_22 = dump_var(Move(_8), Move(_6), Move(_10), Move(_9)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_22 = dump_var(Move(_15), _23, _23, _23), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u32,mut _2: u32,mut _3: u8,mut _4: isize,mut _5: bool,mut _6: i16) -> f32 {
mir! {
type RET = f32;
let _7: f32;
let _8: char;
let _9: *const &'static mut i16;
let _10: *mut *mut [u64; 5];
let _11: &'static mut [i128; 6];
let _12: isize;
let _13: i32;
let _14: Adt46;
let _15: isize;
let _16: &'static (char,);
let _17: [u16; 3];
let _18: f64;
let _19: u16;
let _20: i8;
let _21: i64;
let _22: &'static &'static mut *mut i32;
let _23: *const *mut i32;
let _24: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _25: f32;
let _26: f32;
let _27: u64;
let _28: *const [i16; 2];
let _29: isize;
let _30: char;
let _31: char;
let _32: i16;
let _33: [u64; 4];
let _34: *mut [u64; 5];
let _35: *const u16;
let _36: f64;
let _37: (f64,);
let _38: Adt47;
let _39: isize;
let _40: isize;
let _41: *mut [u64; 5];
let _42: [u8; 7];
let _43: i64;
let _44: isize;
let _45: char;
let _46: isize;
let _47: f32;
let _48: u128;
let _49: &'static Adt19;
let _50: f32;
let _51: char;
let _52: &'static Adt19;
let _53: &'static mut [u8; 6];
let _54: isize;
let _55: u32;
let _56: char;
let _57: &'static (char,);
let _58: *const (u8, usize, i128, i128);
let _59: [usize; 3];
let _60: ((char, Adt19),);
let _61: isize;
let _62: u16;
let _63: ();
let _64: ();
{
RET = 71_i8 as f32;
_7 = -RET;
_3 = 4_usize as u8;
_8 = '\u{d29f0}';
_5 = false;
_5 = RET < RET;
_2 = _1;
_5 = !false;
_5 = !false;
_1 = 22537_u16 as u32;
_3 = 183_u8 * 210_u8;
RET = (-144377679763482340232002566695783658118_i128) as f32;
_1 = _2 - _2;
_5 = true & true;
_5 = true;
_7 = -RET;
_8 = '\u{abc92}';
_3 = 148_u8 ^ 175_u8;
_4 = 9223372036854775807_isize | (-9223372036854775808_isize);
_6 = _3 as i16;
_6 = 9969_i16 + 6515_i16;
_8 = '\u{e0fd5}';
_5 = false;
Goto(bb1)
}
bb1 = {
RET = _7 - _7;
_5 = true;
_12 = _4;
_2 = _4 as u32;
_3 = 182_u8;
_4 = _12 | _12;
_3 = 20_u8 | 36_u8;
_8 = '\u{61dff}';
_14.fld3.1 = (-1832918450_i32) as f32;
_14.fld3.2 = !_5;
_14.fld0 = [228489327715615885_u64,10584732060252432433_u64,8472394461965653307_u64,8802510244165892079_u64,8514120017572864128_u64];
_14.fld7 = 38671510457074503850156872062178714772_i128 - (-21775373498938922675010364476936035111_i128);
_13 = (-2003039473_i32) >> _3;
_14.fld6 = (-5686467597809037705_i64) & 2376020946068203748_i64;
_14.fld4.fld5 = 16459083356456961521_usize as u32;
_12 = _4 ^ _4;
Goto(bb2)
}
bb2 = {
_14.fld2 = 41207_u16 as usize;
_14.fld4.fld4 = 35358_u16;
_4 = _12 & _12;
_12 = _4;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_14.fld4.fld0 = !_14.fld3.2;
_1 = _14.fld4.fld5;
_14.fld4.fld3 = _6 as i8;
_15 = 17945225330204618140_u64 as isize;
_14.fld3.1 = RET + RET;
Goto(bb3)
}
bb3 = {
_14.fld3 = (_14.fld4.fld5, RET, _5);
_3 = !164_u8;
_14.fld4 = Adt25 { fld0: _5,fld1: RET,fld2: _14.fld2,fld3: (-28_i8),fld4: 40709_u16,fld5: _2 };
_14.fld6 = 1469549600015267981_i64 & (-4168693041796091392_i64);
Goto(bb4)
}
bb4 = {
_15 = _12;
_7 = _14.fld4.fld4 as f32;
_14.fld4.fld0 = _14.fld3.2 & _14.fld3.2;
_14.fld7 = 11299153637978868212141496741957372239_i128 * (-110172340185287298934606561159303680003_i128);
_14.fld5 = Adt19::Variant1 { fld0: _14.fld4.fld3,fld1: _14.fld7,fld2: _13 };
_7 = _14.fld3.1;
_14.fld4.fld3 = _14.fld6 as i8;
_7 = RET;
_15 = _12 << _14.fld4.fld4;
_14.fld3.2 = !_14.fld4.fld0;
_14.fld3 = (_14.fld4.fld5, RET, _14.fld4.fld0);
_14.fld7 = Field::<i128>(Variant(_14.fld5, 1), 1) & Field::<i128>(Variant(_14.fld5, 1), 1);
_13 = Field::<i32>(Variant(_14.fld5, 1), 2);
_14.fld4 = Adt25 { fld0: _14.fld3.2,fld1: _14.fld3.1,fld2: _14.fld2,fld3: Field::<i8>(Variant(_14.fld5, 1), 0),fld4: 59334_u16,fld5: _2 };
_4 = _12 >> _13;
_7 = RET - RET;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_6 = (-1061_i16) & (-16611_i16);
_12 = _15 | _4;
RET = _14.fld4.fld1 + _14.fld3.1;
_18 = _6 as f64;
_14.fld4.fld0 = !_5;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_19 = !_14.fld4.fld4;
Call(_17 = fn3(_15, _5, _14.fld4.fld4, _4, RET, _5, _14.fld4.fld3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_24.1 = [_3,_3,_3,_3,_3,_3,_3];
_14.fld4.fld5 = _18 as u32;
_12 = _14.fld3.0 as isize;
_24.3 = (_8,);
_6 = 9590_i16;
_14.fld4.fld3 = Field::<i8>(Variant(_14.fld5, 1), 0) >> _15;
_8 = _24.3.0;
_3 = 165_u8 >> _19;
_14.fld6 = -(-5204392396777111893_i64);
_14.fld4.fld3 = _6 as i8;
_1 = _14.fld6 as u32;
match _14.fld4.fld4 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
59334 => bb13,
_ => bb12
}
}
bb6 = {
_15 = _12;
_7 = _14.fld4.fld4 as f32;
_14.fld4.fld0 = _14.fld3.2 & _14.fld3.2;
_14.fld7 = 11299153637978868212141496741957372239_i128 * (-110172340185287298934606561159303680003_i128);
_14.fld5 = Adt19::Variant1 { fld0: _14.fld4.fld3,fld1: _14.fld7,fld2: _13 };
_7 = _14.fld3.1;
_14.fld4.fld3 = _14.fld6 as i8;
_7 = RET;
_15 = _12 << _14.fld4.fld4;
_14.fld3.2 = !_14.fld4.fld0;
_14.fld3 = (_14.fld4.fld5, RET, _14.fld4.fld0);
_14.fld7 = Field::<i128>(Variant(_14.fld5, 1), 1) & Field::<i128>(Variant(_14.fld5, 1), 1);
_13 = Field::<i32>(Variant(_14.fld5, 1), 2);
_14.fld4 = Adt25 { fld0: _14.fld3.2,fld1: _14.fld3.1,fld2: _14.fld2,fld3: Field::<i8>(Variant(_14.fld5, 1), 0),fld4: 59334_u16,fld5: _2 };
_4 = _12 >> _13;
_7 = RET - RET;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_6 = (-1061_i16) & (-16611_i16);
_12 = _15 | _4;
RET = _14.fld4.fld1 + _14.fld3.1;
_18 = _6 as f64;
_14.fld4.fld0 = !_5;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_19 = !_14.fld4.fld4;
Call(_17 = fn3(_15, _5, _14.fld4.fld4, _4, RET, _5, _14.fld4.fld3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_14.fld3 = (_14.fld4.fld5, RET, _5);
_3 = !164_u8;
_14.fld4 = Adt25 { fld0: _5,fld1: RET,fld2: _14.fld2,fld3: (-28_i8),fld4: 40709_u16,fld5: _2 };
_14.fld6 = 1469549600015267981_i64 & (-4168693041796091392_i64);
Goto(bb4)
}
bb8 = {
_14.fld2 = 41207_u16 as usize;
_14.fld4.fld4 = 35358_u16;
_4 = _12 & _12;
_12 = _4;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_14.fld4.fld0 = !_14.fld3.2;
_1 = _14.fld4.fld5;
_14.fld4.fld3 = _6 as i8;
_15 = 17945225330204618140_u64 as isize;
_14.fld3.1 = RET + RET;
Goto(bb3)
}
bb9 = {
RET = _7 - _7;
_5 = true;
_12 = _4;
_2 = _4 as u32;
_3 = 182_u8;
_4 = _12 | _12;
_3 = 20_u8 | 36_u8;
_8 = '\u{61dff}';
_14.fld3.1 = (-1832918450_i32) as f32;
_14.fld3.2 = !_5;
_14.fld0 = [228489327715615885_u64,10584732060252432433_u64,8472394461965653307_u64,8802510244165892079_u64,8514120017572864128_u64];
_14.fld7 = 38671510457074503850156872062178714772_i128 - (-21775373498938922675010364476936035111_i128);
_13 = (-2003039473_i32) >> _3;
_14.fld6 = (-5686467597809037705_i64) & 2376020946068203748_i64;
_14.fld4.fld5 = 16459083356456961521_usize as u32;
_12 = _4 ^ _4;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_14.fld4.fld5 = _8 as u32;
_12 = _4;
_24.2 = (_14.fld4.fld5, RET, _14.fld3.2);
_14.fld4.fld0 = _24.2.2 ^ _24.2.2;
_24.3 = (_8,);
_14.fld0 = [3229590803533547411_u64,4999004948851138307_u64,505352400382006146_u64,6093567442148849912_u64,6657190155456666948_u64];
_20 = Field::<i8>(Variant(_14.fld5, 1), 0) + Field::<i8>(Variant(_14.fld5, 1), 0);
_14.fld3 = _24.2;
_14.fld6 = 53840232348229079_i64 | (-8794226671460805393_i64);
_14.fld3 = (_14.fld4.fld5, _7, _14.fld4.fld0);
_14.fld4 = Adt25 { fld0: _14.fld3.2,fld1: _14.fld3.1,fld2: _14.fld2,fld3: Field::<i8>(Variant(_14.fld5, 1), 0),fld4: _19,fld5: _24.2.0 };
_14.fld4.fld2 = _14.fld2 << _19;
_12 = -_15;
Goto(bb14)
}
bb14 = {
RET = _14.fld4.fld1 * _14.fld3.1;
_14.fld3.2 = _14.fld4.fld0 | _24.2.2;
_14.fld1 = Adt27::Variant1 { fld0: Field::<i128>(Variant(_14.fld5, 1), 1),fld1: _8,fld2: RET,fld3: _20,fld4: _14.fld3,fld5: _14.fld0 };
_14.fld0 = Field::<[u64; 5]>(Variant(_14.fld1, 1), 5);
_24.2.2 = !_5;
match _14.fld4.fld3 {
0 => bb8,
1 => bb11,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
340282366920938463463374607431768211428 => bb21,
_ => bb20
}
}
bb15 = {
_14.fld3 = (_14.fld4.fld5, RET, _5);
_3 = !164_u8;
_14.fld4 = Adt25 { fld0: _5,fld1: RET,fld2: _14.fld2,fld3: (-28_i8),fld4: 40709_u16,fld5: _2 };
_14.fld6 = 1469549600015267981_i64 & (-4168693041796091392_i64);
Goto(bb4)
}
bb16 = {
Return()
}
bb17 = {
_14.fld2 = 41207_u16 as usize;
_14.fld4.fld4 = 35358_u16;
_4 = _12 & _12;
_12 = _4;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_14.fld4.fld0 = !_14.fld3.2;
_1 = _14.fld4.fld5;
_14.fld4.fld3 = _6 as i8;
_15 = 17945225330204618140_u64 as isize;
_14.fld3.1 = RET + RET;
Goto(bb3)
}
bb18 = {
_15 = _12;
_7 = _14.fld4.fld4 as f32;
_14.fld4.fld0 = _14.fld3.2 & _14.fld3.2;
_14.fld7 = 11299153637978868212141496741957372239_i128 * (-110172340185287298934606561159303680003_i128);
_14.fld5 = Adt19::Variant1 { fld0: _14.fld4.fld3,fld1: _14.fld7,fld2: _13 };
_7 = _14.fld3.1;
_14.fld4.fld3 = _14.fld6 as i8;
_7 = RET;
_15 = _12 << _14.fld4.fld4;
_14.fld3.2 = !_14.fld4.fld0;
_14.fld3 = (_14.fld4.fld5, RET, _14.fld4.fld0);
_14.fld7 = Field::<i128>(Variant(_14.fld5, 1), 1) & Field::<i128>(Variant(_14.fld5, 1), 1);
_13 = Field::<i32>(Variant(_14.fld5, 1), 2);
_14.fld4 = Adt25 { fld0: _14.fld3.2,fld1: _14.fld3.1,fld2: _14.fld2,fld3: Field::<i8>(Variant(_14.fld5, 1), 0),fld4: 59334_u16,fld5: _2 };
_4 = _12 >> _13;
_7 = RET - RET;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_6 = (-1061_i16) & (-16611_i16);
_12 = _15 | _4;
RET = _14.fld4.fld1 + _14.fld3.1;
_18 = _6 as f64;
_14.fld4.fld0 = !_5;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_19 = !_14.fld4.fld4;
Call(_17 = fn3(_15, _5, _14.fld4.fld4, _4, RET, _5, _14.fld4.fld3), ReturnTo(bb5), UnwindUnreachable())
}
bb19 = {
RET = _7 - _7;
_5 = true;
_12 = _4;
_2 = _4 as u32;
_3 = 182_u8;
_4 = _12 | _12;
_3 = 20_u8 | 36_u8;
_8 = '\u{61dff}';
_14.fld3.1 = (-1832918450_i32) as f32;
_14.fld3.2 = !_5;
_14.fld0 = [228489327715615885_u64,10584732060252432433_u64,8472394461965653307_u64,8802510244165892079_u64,8514120017572864128_u64];
_14.fld7 = 38671510457074503850156872062178714772_i128 - (-21775373498938922675010364476936035111_i128);
_13 = (-2003039473_i32) >> _3;
_14.fld6 = (-5686467597809037705_i64) & 2376020946068203748_i64;
_14.fld4.fld5 = 16459083356456961521_usize as u32;
_12 = _4 ^ _4;
Goto(bb2)
}
bb20 = {
_14.fld2 = 41207_u16 as usize;
_14.fld4.fld4 = 35358_u16;
_4 = _12 & _12;
_12 = _4;
_17 = [_14.fld4.fld4,_14.fld4.fld4,_14.fld4.fld4];
_14.fld4.fld0 = !_14.fld3.2;
_1 = _14.fld4.fld5;
_14.fld4.fld3 = _6 as i8;
_15 = 17945225330204618140_u64 as isize;
_14.fld3.1 = RET + RET;
Goto(bb3)
}
bb21 = {
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).1 = -RET;
_27 = _14.fld7 as u64;
place!(Field::<f32>(Variant(_14.fld1, 1), 2)) = _7 * Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).1;
Goto(bb22)
}
bb22 = {
_14.fld4.fld3 = _20;
place!(Field::<i128>(Variant(_14.fld1, 1), 0)) = -_14.fld7;
_26 = _13 as f32;
_14.fld2 = _14.fld4.fld2;
place!(Field::<f32>(Variant(_14.fld1, 1), 2)) = _14.fld3.1;
place!(Field::<i8>(Variant(_14.fld5, 1), 0)) = _14.fld4.fld3;
_18 = _14.fld4.fld4 as f64;
_14.fld4.fld0 = !Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).2;
_14.fld4.fld0 = !_24.2.2;
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).2 = _12 != _4;
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).0 = !_14.fld4.fld5;
_6 = Field::<i32>(Variant(_14.fld5, 1), 2) as i16;
_30 = _24.3.0;
RET = Field::<i32>(Variant(_14.fld5, 1), 2) as f32;
_24.2.0 = _2;
_4 = _15;
Call(_24 = fn4(Field::<i8>(Variant(_14.fld5, 1), 0)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
place!(Field::<char>(Variant(_14.fld1, 1), 1)) = _30;
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).0 = _2 << _14.fld2;
_14.fld4.fld3 = _3 as i8;
_14.fld3.1 = Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).1;
_24.1 = [_3,_3,_3,_3,_3,_3,_3];
_16 = &_24.3;
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).0 = _14.fld6 as u32;
_3 = 98_u8 - 21_u8;
_1 = _24.2.0 * Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).0;
place!(Field::<[u64; 5]>(Variant(_14.fld1, 1), 5)) = _14.fld0;
place!(Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4)).0 = _14.fld4.fld5 - _14.fld3.0;
place!(Field::<i128>(Variant(_14.fld5, 1), 1)) = _14.fld2 as i128;
_24.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_14.fld1, 1), 0)));
_14.fld4.fld2 = !_14.fld2;
_32 = Field::<i128>(Variant(_14.fld5, 1), 1) as i16;
_13 = Field::<i32>(Variant(_14.fld5, 1), 2) ^ Field::<i32>(Variant(_14.fld5, 1), 2);
_14.fld4.fld5 = _15 as u32;
_30 = (*_16).0;
_14.fld1 = Adt27::Variant1 { fld0: Field::<i128>(Variant(_14.fld5, 1), 1),fld1: (*_16).0,fld2: _14.fld3.1,fld3: _20,fld4: _24.2,fld5: _14.fld0 };
_29 = _15 ^ _4;
_14.fld4.fld2 = _14.fld2 * _14.fld2;
_14.fld4.fld0 = _20 == _14.fld4.fld3;
_21 = _14.fld6;
Goto(bb24)
}
bb24 = {
_17 = [_19,_19,_19];
_24.3 = (_30,);
place!(Field::<i8>(Variant(_14.fld1, 1), 3)) = _3 as i8;
_24.2.1 = Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).1;
_14.fld4.fld4 = _19 & _19;
_14.fld4.fld1 = _24.2.1 + Field::<f32>(Variant(_14.fld1, 1), 2);
place!(Field::<i128>(Variant(_14.fld5, 1), 1)) = -Field::<i128>(Variant(_14.fld1, 1), 0);
_18 = _13 as f64;
_24.2.0 = _14.fld4.fld5;
_14.fld4.fld3 = Field::<i8>(Variant(_14.fld5, 1), 0) - Field::<i8>(Variant(_14.fld5, 1), 0);
_33 = [_27,_27,_27,_27];
place!(Field::<i32>(Variant(_14.fld5, 1), 2)) = _13 + _13;
Goto(bb25)
}
bb25 = {
_38.fld0.0 = _24.3.0;
_23 = core::ptr::addr_of!(_38.fld7);
_26 = -_24.2.1;
(*_23) = core::ptr::addr_of_mut!(_13);
_38.fld5.fld4 = _19 * _14.fld4.fld4;
_38.fld2 = -_7;
Goto(bb26)
}
bb26 = {
_13 = -Field::<i32>(Variant(_14.fld5, 1), 2);
(*_23) = core::ptr::addr_of_mut!(_13);
_14.fld4.fld0 = !Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).2;
(*_23) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_14.fld5, 1), 2)));
_14.fld4.fld3 = Field::<i8>(Variant(_14.fld5, 1), 0);
_36 = _18 * _18;
_13 = Field::<i32>(Variant(_14.fld5, 1), 2) * Field::<i32>(Variant(_14.fld5, 1), 2);
(*_23) = core::ptr::addr_of_mut!(_13);
_14.fld3.1 = -Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).1;
(*_23) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_14.fld5, 1), 2)));
_42 = [_3,_3,_3,_3,_3,_3,_3];
_14.fld3.0 = _14.fld2 as u32;
(*_23) = core::ptr::addr_of_mut!(_13);
place!(Field::<i32>(Variant(_14.fld5, 1), 2)) = _13 - _13;
_33 = [_27,_27,_27,_27];
_38.fld5 = _14.fld4;
_14.fld4.fld0 = _24.2.2 | Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).2;
_14.fld4.fld4 = _38.fld5.fld4 + _38.fld5.fld4;
Goto(bb27)
}
bb27 = {
_17 = [_14.fld4.fld4,_38.fld5.fld4,_14.fld4.fld4];
_3 = 247_u8 & 97_u8;
_47 = 262959678719849598815554666044762053520_u128 as f32;
_6 = _32 - _32;
_38.fld5.fld4 = 294662657345142194176837044138747342605_u128 as u16;
_7 = _14.fld4.fld1;
_24.1 = [_3,_3,_3,_3,_3,_3,_3];
_19 = _14.fld4.fld4 & _14.fld4.fld4;
_5 = _24.2.2 & _24.2.2;
Goto(bb28)
}
bb28 = {
_38.fld5.fld1 = _26 - _14.fld4.fld1;
_38.fld3 = !_27;
_38.fld5.fld5 = _14.fld3.0;
_14.fld4.fld0 = !_38.fld5.fld0;
_38.fld5.fld5 = _24.2.0 & _24.2.0;
_17 = [_19,_19,_19];
_43 = _21 - _21;
_38.fld3 = _27 * _27;
_14.fld2 = _38.fld5.fld2 * _38.fld5.fld2;
_17 = [_14.fld4.fld4,_19,_14.fld4.fld4];
_38.fld0.0 = Field::<char>(Variant(_14.fld1, 1), 1);
_30 = _8;
place!(Field::<i8>(Variant(_14.fld1, 1), 3)) = _24.2.0 as i8;
_12 = _26 as isize;
_31 = _8;
Goto(bb29)
}
bb29 = {
_34 = core::ptr::addr_of_mut!(_14.fld0);
_25 = Field::<(u32, f32, bool)>(Variant(_14.fld1, 1), 4).1 + _14.fld3.1;
_36 = _18;
_38.fld3 = _27 << Field::<i32>(Variant(_14.fld5, 1), 2);
_35 = core::ptr::addr_of!(_14.fld4.fld4);
(*_34) = [_38.fld3,_38.fld3,_38.fld3,_38.fld3,_38.fld3];
_38.fld5.fld1 = _7 - _7;
(*_23) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_14.fld5, 1), 2)));
(*_23) = core::ptr::addr_of_mut!(_13);
Goto(bb30)
}
bb30 = {
(*_34) = [_38.fld3,_38.fld3,_38.fld3,_38.fld3,_38.fld3];
_38.fld0.0 = _30;
_14.fld3.0 = _14.fld4.fld5;
Goto(bb31)
}
bb31 = {
(*_35) = _19 & _19;
_54 = _38.fld0.0 as isize;
Goto(bb32)
}
bb32 = {
_38.fld0.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_14.fld1, 1), 3),fld1: Field::<i128>(Variant(_14.fld5, 1), 1),fld2: Field::<i32>(Variant(_14.fld5, 1), 2) };
(*_35) = !_19;
(*_34) = Field::<[u64; 5]>(Variant(_14.fld1, 1), 5);
(*_23) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_38.fld0.1, 1), 2)));
(*_34) = Field::<[u64; 5]>(Variant(_14.fld1, 1), 5);
_52 = &_38.fld0.1;
_24.1 = _42;
_14.fld5 = Adt19::Variant1 { fld0: Field::<i8>(Variant((*_52), 1), 0),fld1: Field::<i128>(Variant(_14.fld1, 1), 0),fld2: Field::<i32>(Variant((*_52), 1), 2) };
_41 = core::ptr::addr_of_mut!((*_34));
_38.fld6 = Move(_14.fld5);
(*_41) = Field::<[u64; 5]>(Variant(_14.fld1, 1), 5);
_3 = _15 as u8;
_38.fld4 = _18 as i16;
_52 = &_38.fld6;
_41 = core::ptr::addr_of_mut!((*_34));
_32 = _6 - _38.fld4;
_30 = _8;
_21 = _43 & _43;
(*_41) = [_38.fld3,_38.fld3,_38.fld3,_38.fld3,_38.fld3];
(*_41) = Field::<[u64; 5]>(Variant(_14.fld1, 1), 5);
_38.fld1 = _31;
_38.fld4 = _3 as i16;
_60.0.0 = _38.fld0.0;
Goto(bb33)
}
bb33 = {
_60.0 = Move(_38.fld0);
_31 = Field::<char>(Variant(_14.fld1, 1), 1);
_14.fld4.fld3 = -Field::<i8>(Variant((*_52), 1), 0);
_46 = !_29;
_14.fld4.fld2 = _29 as usize;
(*_23) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant((*_52), 1), 2)));
_50 = Field::<i32>(Variant((*_52), 1), 2) as f32;
RET = -_38.fld5.fld1;
Goto(bb34)
}
bb34 = {
Call(_63 = dump_var(Move(_33), Move(_19), Move(_15), Move(_42)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_63 = dump_var(Move(_54), Move(_5), Move(_31), Move(_46)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_63 = dump_var(Move(_27), Move(_8), Move(_6), Move(_13)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: bool,mut _3: u16,mut _4: isize,mut _5: f32,mut _6: bool,mut _7: i8) -> [u16; 3] {
mir! {
type RET = [u16; 3];
let _8: isize;
let _9: ();
let _10: ();
{
RET = [_3,_3,_3];
_6 = _1 < _1;
_4 = -_1;
_5 = _3 as f32;
_8 = _4;
_3 = 28676_u16 << _7;
_8 = _4 ^ _1;
_8 = _1;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(Move(_6), Move(_7), Move(_8), _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8) -> (*mut i128, [u8; 7], (u32, f32, bool), (char,)) {
mir! {
type RET = (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _2: *const char;
let _3: i32;
let _4: f32;
let _5: *const (u8, usize, i128, i128);
let _6: *mut i128;
let _7: bool;
let _8: *mut i128;
let _9: &'static (u32, f32, bool);
let _10: f32;
let _11: f64;
let _12: char;
let _13: u64;
let _14: &'static mut &'static (char,);
let _15: bool;
let _16: isize;
let _17: u8;
let _18: &'static (char,);
let _19: [bool; 7];
let _20: char;
let _21: bool;
let _22: f32;
let _23: *const char;
let _24: u8;
let _25: bool;
let _26: u16;
let _27: f32;
let _28: &'static (char,);
let _29: *mut i32;
let _30: u128;
let _31: &'static &'static mut bool;
let _32: &'static mut &'static mut bool;
let _33: i128;
let _34: &'static (u32, f32, bool);
let _35: i8;
let _36: usize;
let _37: &'static mut &'static (char,);
let _38: i64;
let _39: [isize; 5];
let _40: Adt47;
let _41: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _42: isize;
let _43: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _44: ();
let _45: ();
{
RET.2.0 = 1942752514_u32 ^ 3402922798_u32;
RET.3 = ('\u{998bf}',);
RET.2.2 = false & false;
RET.3.0 = '\u{7561a}';
RET.1 = [10_u8,152_u8,199_u8,76_u8,200_u8,170_u8,252_u8];
RET.3 = ('\u{718bb}',);
RET.2.1 = 2768927592637254828_i64 as f32;
_1 = 24_u8 as i8;
_1 = (-59_i8);
RET.3.0 = '\u{442f6}';
RET.2.0 = 3487292967_u32 + 1962810591_u32;
_1 = 1434_u16 as i8;
RET.1 = [197_u8,176_u8,243_u8,3_u8,148_u8,83_u8,99_u8];
_2 = core::ptr::addr_of!(RET.3.0);
_3 = 750580840_i32 >> RET.2.0;
(*_2) = '\u{8b7eb}';
(*_2) = '\u{7658a}';
(*_2) = '\u{a16f1}';
(*_2) = '\u{6da46}';
_3 = (-413756656_i32) >> _1;
_4 = -RET.2.1;
(*_2) = '\u{7bd0a}';
RET.2.0 = 2251107259_u32;
match RET.2.0 {
2251107259 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
(*_2) = '\u{872cd}';
RET.2.1 = -_4;
RET.1 = [155_u8,155_u8,145_u8,145_u8,152_u8,11_u8,116_u8];
(*_2) = '\u{b4797}';
(*_2) = '\u{19023}';
(*_2) = '\u{30aff}';
RET.3 = ('\u{2afc0}',);
(*_2) = '\u{76e3}';
(*_2) = '\u{e6062}';
(*_2) = '\u{695aa}';
RET.1 = [103_u8,76_u8,163_u8,61_u8,198_u8,37_u8,206_u8];
(*_2) = '\u{b9773}';
(*_2) = '\u{32721}';
Goto(bb3)
}
bb3 = {
RET.2.0 = !3223689631_u32;
(*_2) = '\u{2f312}';
(*_2) = '\u{84546}';
_3 = 17264739946261980117_u64 as i32;
(*_2) = '\u{1ee84}';
_4 = -RET.2.1;
(*_2) = '\u{2915b}';
(*_2) = '\u{ca6f1}';
RET.1 = [75_u8,212_u8,157_u8,123_u8,175_u8,192_u8,205_u8];
(*_2) = '\u{f8f2f}';
_1 = (-3066772131095969859_i64) as i8;
(*_2) = '\u{69e38}';
(*_2) = '\u{5e93b}';
(*_2) = '\u{5fa9f}';
(*_2) = '\u{79bc4}';
(*_2) = '\u{52b81}';
(*_2) = '\u{81e48}';
(*_2) = '\u{4b180}';
(*_2) = '\u{a60f5}';
_2 = core::ptr::addr_of!((*_2));
Goto(bb4)
}
bb4 = {
_10 = _4 * _4;
RET.2 = (2188846935_u32, _10, true);
(*_2) = '\u{acff}';
_2 = core::ptr::addr_of!((*_2));
RET.2.0 = 4094757834_u32;
(*_2) = '\u{31195}';
RET.2.0 = 3885662466_u32 * 1327775817_u32;
(*_2) = '\u{b6164}';
(*_2) = '\u{1dd51}';
RET.3 = ('\u{bff36}',);
RET.2.0 = 2809599488_u32;
_12 = RET.3.0;
_4 = _10 * _10;
Call(_12 = fn5(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = 196328009874467490984763083937400394457_u128 as i32;
_3 = 968624315_i32 ^ 1600260572_i32;
RET.2 = (590312629_u32, _4, false);
(*_2) = _12;
(*_2) = _12;
(*_2) = _12;
_10 = _4;
(*_2) = _12;
(*_2) = _12;
(*_2) = _12;
(*_2) = _12;
_3 = 2124236748_i32 | 1543445601_i32;
RET.2 = (1259178907_u32, _4, true);
(*_2) = _12;
_11 = 66326421983814671113157453737808229611_u128 as f64;
_11 = (-1999_i16) as f64;
_11 = 126_u8 as f64;
match RET.2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
1259178907 => bb8,
_ => bb7
}
}
bb6 = {
Return()
}
bb7 = {
(*_2) = '\u{872cd}';
RET.2.1 = -_4;
RET.1 = [155_u8,155_u8,145_u8,145_u8,152_u8,11_u8,116_u8];
(*_2) = '\u{b4797}';
(*_2) = '\u{19023}';
(*_2) = '\u{30aff}';
RET.3 = ('\u{2afc0}',);
(*_2) = '\u{76e3}';
(*_2) = '\u{e6062}';
(*_2) = '\u{695aa}';
RET.1 = [103_u8,76_u8,163_u8,61_u8,198_u8,37_u8,206_u8];
(*_2) = '\u{b9773}';
(*_2) = '\u{32721}';
Goto(bb3)
}
bb8 = {
(*_2) = _12;
RET.2 = (2885263600_u32, _4, true);
RET.2 = (903813206_u32, _10, false);
_10 = RET.2.1 + RET.2.1;
Goto(bb9)
}
bb9 = {
RET.2.1 = 15045889585702627083_u64 as f32;
_7 = _10 < _10;
_9 = &RET.2;
_4 = 118_isize as f32;
_2 = core::ptr::addr_of!((*_2));
(*_2) = _12;
_15 = (*_9).2;
_7 = (*_9).0 == (*_9).0;
_17 = _3 as u8;
(*_2) = _12;
RET.2.2 = (*_9).0 != (*_9).0;
_13 = 6514425471473132806_u64 ^ 10817328946224276101_u64;
RET.2.0 = 30693_u16 as u32;
(*_2) = _12;
(*_2) = _12;
Goto(bb10)
}
bb10 = {
_15 = !_7;
_20 = (*_2);
(*_2) = _20;
(*_2) = _12;
_16 = (-108_isize) | (-61_isize);
_13 = 889822541847259415_u64 | 3064872485988754474_u64;
_16 = 9223372036854775807_isize;
RET.2.2 = !_7;
_10 = _17 as f32;
RET.2.2 = _7 <= _15;
(*_2) = _20;
(*_2) = _20;
_22 = RET.2.0 as f32;
RET.3 = (_12,);
(*_2) = _20;
_12 = (*_2);
RET.2.1 = 12326_i16 as f32;
(*_2) = _20;
(*_2) = _12;
_23 = core::ptr::addr_of!((*_2));
(*_2) = _12;
(*_2) = _20;
(*_2) = _20;
_16 = 18_isize;
Goto(bb11)
}
bb11 = {
(*_2) = _12;
(*_2) = _12;
_3 = -(-1919920144_i32);
_3 = 741069066_i32;
(*_2) = _20;
_16 = 9223372036854775807_isize ^ (-27_isize);
Call(_16 = core::intrinsics::transmute(_13), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15 = _7 > RET.2.2;
_21 = RET.2.2 ^ _15;
(*_2) = _20;
(*_2) = _20;
_19 = [RET.2.2,_21,_15,RET.2.2,RET.2.2,_21,_21];
(*_2) = _20;
(*_2) = _20;
_15 = !RET.2.2;
_4 = _10 + RET.2.1;
(*_2) = _12;
(*_2) = _12;
_12 = (*_2);
_21 = _15;
_23 = Move(_2);
_24 = _17 ^ _17;
_1 = 23_i8;
_2 = core::ptr::addr_of!(_12);
(*_2) = _20;
(*_2) = _20;
_18 = &RET.3;
(*_2) = (*_18).0;
match _1 {
0 => bb13,
1 => bb14,
2 => bb15,
23 => bb17,
_ => bb16
}
}
bb13 = {
(*_2) = _12;
(*_2) = _12;
_3 = -(-1919920144_i32);
_3 = 741069066_i32;
(*_2) = _20;
_16 = 9223372036854775807_isize ^ (-27_isize);
Call(_16 = core::intrinsics::transmute(_13), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
(*_2) = _12;
RET.2 = (2885263600_u32, _4, true);
RET.2 = (903813206_u32, _10, false);
_10 = RET.2.1 + RET.2.1;
Goto(bb9)
}
bb17 = {
(*_2) = (*_18).0;
(*_2) = _20;
(*_2) = _20;
Goto(bb18)
}
bb18 = {
_19 = [RET.2.2,_7,RET.2.2,_15,_15,_15,_15];
_29 = core::ptr::addr_of_mut!(_3);
(*_29) = 1468944232_i32 * 1278175446_i32;
(*_29) = (-165485288_i32) * 1087343849_i32;
RET.3 = ((*_2),);
(*_29) = !(-2064205135_i32);
_11 = _24 as f64;
_3 = 438889976_i32 >> _24;
(*_29) = !(-1928001485_i32);
RET.1 = [_24,_24,_17,_24,_24,_24,_17];
(*_2) = _20;
match _1 {
0 => bb8,
23 => bb20,
_ => bb19
}
}
bb19 = {
(*_2) = '\u{872cd}';
RET.2.1 = -_4;
RET.1 = [155_u8,155_u8,145_u8,145_u8,152_u8,11_u8,116_u8];
(*_2) = '\u{b4797}';
(*_2) = '\u{19023}';
(*_2) = '\u{30aff}';
RET.3 = ('\u{2afc0}',);
(*_2) = '\u{76e3}';
(*_2) = '\u{e6062}';
(*_2) = '\u{695aa}';
RET.1 = [103_u8,76_u8,163_u8,61_u8,198_u8,37_u8,206_u8];
(*_2) = '\u{b9773}';
(*_2) = '\u{32721}';
Goto(bb3)
}
bb20 = {
_4 = _10 * RET.2.1;
_27 = _10 - _22;
(*_29) = (-1244587637_i32);
(*_2) = _20;
(*_2) = _20;
(*_29) = !(-569219416_i32);
_24 = !_17;
(*_2) = _20;
_30 = (*_29) as u128;
Goto(bb21)
}
bb21 = {
(*_2) = _20;
_11 = _30 as f64;
(*_29) = 69870740862944985255789267215249988256_i128 as i32;
(*_29) = (-1399586381_i32) >> _24;
(*_29) = !(-511368019_i32);
_15 = !RET.2.2;
(*_29) = _4 as i32;
_26 = 26351_u16;
(*_29) = (-2087393015_i32) | 1632960773_i32;
_12 = RET.3.0;
_15 = (*_29) > (*_29);
_24 = !_17;
(*_2) = RET.3.0;
_24 = _17 + _17;
RET.1 = [_17,_24,_17,_24,_24,_17,_24];
_3 = (-1822270771_i32);
(*_29) = (-1324302173_i32);
_17 = _24;
_7 = !_21;
(*_2) = _20;
Call((*_29) = core::intrinsics::bswap(483377825_i32), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
(*_2) = _20;
(*_2) = RET.3.0;
(*_2) = RET.3.0;
(*_29) = (-657345548_i32);
(*_29) = !1168959162_i32;
RET.2.1 = _27 + _10;
_16 = 9223372036854775807_isize;
_7 = _21 | RET.2.2;
_8 = core::ptr::addr_of_mut!(_33);
_28 = &RET.3;
(*_8) = 48337440208658038182704584137250192125_i128;
(*_2) = (*_28).0;
_6 = Move(_8);
_3 = _7 as i32;
(*_2) = (*_28).0;
_1 = RET.2.1 as i8;
(*_2) = (*_28).0;
Goto(bb23)
}
bb23 = {
(*_29) = !402649375_i32;
_22 = -_4;
RET.3 = ((*_2),);
_26 = 4688_u16 | 37166_u16;
_40.fld4 = (-22442_i16);
(*_2) = _20;
(*_29) = 1066708573_i32 & (-1039800359_i32);
_25 = !_7;
(*_29) = _27 as i32;
(*_2) = RET.3.0;
_40.fld5 = Adt25 { fld0: _25,fld1: _27,fld2: 8847940401706349307_usize,fld3: _1,fld4: _26,fld5: RET.2.0 };
RET.2.0 = !_40.fld5.fld5;
_40.fld5.fld2 = 6_usize;
(*_29) = 1401215099_i32 + (-2016951761_i32);
_18 = Move(_28);
_40.fld5.fld2 = _4 as usize;
(*_2) = _20;
(*_2) = RET.3.0;
RET.0 = core::ptr::addr_of_mut!(_33);
(*_2) = RET.3.0;
_35 = _1 & _1;
_17 = _24;
_11 = _24 as f64;
Goto(bb24)
}
bb24 = {
Call(_44 = dump_var(Move(_13), Move(_7), Move(_30), Move(_35)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_44 = dump_var(Move(_26), Move(_3), Move(_25), Move(_20)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5() -> char {
mir! {
type RET = char;
let _1: bool;
let _2: isize;
let _3: bool;
let _4: [u128; 6];
let _5: &'static u16;
let _6: &'static u16;
let _7: u128;
let _8: *mut *mut [u64; 5];
let _9: *mut i128;
let _10: *const &'static &'static mut bool;
let _11: &'static mut &'static mut bool;
let _12: [i64; 4];
let _13: *const [i16; 2];
let _14: f64;
let _15: usize;
let _16: usize;
let _17: [u32; 2];
let _18: (&'static mut i16, *mut [u64; 5]);
let _19: *const [u8; 7];
let _20: f64;
let _21: char;
let _22: char;
let _23: isize;
let _24: usize;
let _25: i128;
let _26: &'static u16;
let _27: bool;
let _28: &'static mut [u8; 6];
let _29: i8;
let _30: *const [u8; 7];
let _31: ((char, Adt19),);
let _32: &'static mut u16;
let _33: isize;
let _34: *const *mut i32;
let _35: *const isize;
let _36: i16;
let _37: (char,);
let _38: *mut *mut [u64; 5];
let _39: *const [i16; 2];
let _40: &'static mut &'static mut bool;
let _41: f32;
let _42: &'static mut [bool; 7];
let _43: *const &'static &'static mut bool;
let _44: f32;
let _45: f64;
let _46: [isize; 6];
let _47: *mut [u64; 5];
let _48: Adt25;
let _49: f32;
let _50: char;
let _51: &'static mut i16;
let _52: &'static mut i16;
let _53: &'static &'static mut *mut i32;
let _54: i64;
let _55: [u64; 4];
let _56: ((char, Adt19),);
let _57: f32;
let _58: isize;
let _59: &'static &'static mut *mut i32;
let _60: char;
let _61: [u64; 4];
let _62: usize;
let _63: [char; 2];
let _64: (&'static mut i16, *mut [u64; 5]);
let _65: isize;
let _66: (u8, u8);
let _67: *const char;
let _68: [i128; 6];
let _69: *const isize;
let _70: isize;
let _71: f32;
let _72: bool;
let _73: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _74: (char, *const u128, (char, Adt19), *mut i32);
let _75: u32;
let _76: u128;
let _77: Adt25;
let _78: ();
let _79: ();
{
RET = '\u{d2f8e}';
RET = '\u{449de}';
RET = '\u{fc040}';
RET = '\u{145e0}';
RET = '\u{ca012}';
RET = '\u{565b}';
RET = '\u{5a7db}';
RET = '\u{e7c77}';
Goto(bb1)
}
bb1 = {
RET = '\u{b99b7}';
RET = '\u{e8aa1}';
RET = '\u{4d1aa}';
RET = '\u{4db4c}';
RET = '\u{66b45}';
Goto(bb2)
}
bb2 = {
RET = '\u{4a32b}';
Call(_1 = fn6(RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = !true;
_1 = true;
_1 = !false;
RET = '\u{c63ef}';
_1 = false & true;
RET = '\u{33e0b}';
RET = '\u{71f8d}';
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = !_1;
_3 = _1 == _1;
_3 = !_1;
_3 = !_1;
_3 = _1;
_4 = [88527177725152584702444187432585276628_u128,291271873107620463465544913787915165901_u128,307502546046487705244197423797186370214_u128,119206013539530678207743579683705734109_u128,79542517118125160551427610965299252275_u128,188512572650188408782850375384340401807_u128];
_7 = 148778095022499963106678600075090560488_u128 + 76122375549378057529722996201518740777_u128;
RET = '\u{b4254}';
_3 = _1;
_7 = _1 as u128;
RET = '\u{b3a8c}';
_1 = !_3;
_1 = !_3;
_3 = _1 | _1;
_1 = _2 == _2;
_7 = 279606776570934205477945705201052270396_u128 & 303820743820011367495613567011666191543_u128;
_2 = (-9223372036854775808_isize);
match _2 {
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb4 = {
RET = '\u{b99b7}';
RET = '\u{e8aa1}';
RET = '\u{4d1aa}';
RET = '\u{4db4c}';
RET = '\u{66b45}';
Goto(bb2)
}
bb5 = {
_4 = [_7,_7,_7,_7,_7,_7];
match _2 {
0 => bb1,
1 => bb4,
2 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb6 = {
RET = '\u{b99b7}';
RET = '\u{e8aa1}';
RET = '\u{4d1aa}';
RET = '\u{4db4c}';
RET = '\u{66b45}';
Goto(bb2)
}
bb7 = {
_1 = !true;
_1 = true;
_1 = !false;
RET = '\u{c63ef}';
_1 = false & true;
RET = '\u{33e0b}';
RET = '\u{71f8d}';
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = !_1;
_3 = _1 == _1;
_3 = !_1;
_3 = !_1;
_3 = _1;
_4 = [88527177725152584702444187432585276628_u128,291271873107620463465544913787915165901_u128,307502546046487705244197423797186370214_u128,119206013539530678207743579683705734109_u128,79542517118125160551427610965299252275_u128,188512572650188408782850375384340401807_u128];
_7 = 148778095022499963106678600075090560488_u128 + 76122375549378057529722996201518740777_u128;
RET = '\u{b4254}';
_3 = _1;
_7 = _1 as u128;
RET = '\u{b3a8c}';
_1 = !_3;
_1 = !_3;
_3 = _1 | _1;
_1 = _2 == _2;
_7 = 279606776570934205477945705201052270396_u128 & 303820743820011367495613567011666191543_u128;
_2 = (-9223372036854775808_isize);
match _2 {
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb8 = {
_1 = _7 != _7;
_7 = _2 as u128;
_7 = 12731418262556431963508332310386909982_u128 & 281352002615787105279799494046632653104_u128;
RET = '\u{32425}';
RET = '\u{38f4d}';
_4 = [_7,_7,_7,_7,_7,_7];
_1 = _3 <= _3;
Goto(bb9)
}
bb9 = {
RET = '\u{313fb}';
match _2 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb5,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb10 = {
_1 = !true;
_1 = true;
_1 = !false;
RET = '\u{c63ef}';
_1 = false & true;
RET = '\u{33e0b}';
RET = '\u{71f8d}';
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = !_1;
_3 = _1 == _1;
_3 = !_1;
_3 = !_1;
_3 = _1;
_4 = [88527177725152584702444187432585276628_u128,291271873107620463465544913787915165901_u128,307502546046487705244197423797186370214_u128,119206013539530678207743579683705734109_u128,79542517118125160551427610965299252275_u128,188512572650188408782850375384340401807_u128];
_7 = 148778095022499963106678600075090560488_u128 + 76122375549378057529722996201518740777_u128;
RET = '\u{b4254}';
_3 = _1;
_7 = _1 as u128;
RET = '\u{b3a8c}';
_1 = !_3;
_1 = !_3;
_3 = _1 | _1;
_1 = _2 == _2;
_7 = 279606776570934205477945705201052270396_u128 & 303820743820011367495613567011666191543_u128;
_2 = (-9223372036854775808_isize);
match _2 {
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb11 = {
RET = '\u{b99b7}';
RET = '\u{e8aa1}';
RET = '\u{4d1aa}';
RET = '\u{4db4c}';
RET = '\u{66b45}';
Goto(bb2)
}
bb12 = {
RET = '\u{b99b7}';
RET = '\u{e8aa1}';
RET = '\u{4d1aa}';
RET = '\u{4db4c}';
RET = '\u{66b45}';
Goto(bb2)
}
bb13 = {
_1 = _3;
_1 = _7 == _7;
_7 = 190233492307082658980956920217311506887_u128 | 159245074898605179267185850881605288745_u128;
RET = '\u{5c8db}';
_7 = !187923729228759596336115954278435961918_u128;
RET = '\u{10ce55}';
_1 = _3 > _3;
_4 = [_7,_7,_7,_7,_7,_7];
_1 = !_3;
_7 = 280780548121693286190705014818248750876_u128;
_1 = _3;
_3 = RET > RET;
_12 = [(-3862246428249132689_i64),(-2594543489971451069_i64),(-1238053017923894022_i64),2662484436877636598_i64];
_12 = [(-9083166824021557269_i64),4015422671189955839_i64,4883610683647907255_i64,(-3388989061000245348_i64)];
_3 = _1 ^ _1;
RET = '\u{27854}';
_1 = _3 | _3;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{737a0}';
_14 = 3759935147_u32 as f64;
_2 = -(-9223372036854775808_isize);
_14 = (-104_i8) as f64;
Goto(bb14)
}
bb14 = {
_7 = 15339363798673402955992712058622789893_u128;
_4 = [_7,_7,_7,_7,_7,_7];
_2 = _3 as isize;
_3 = !_1;
_7 = _3 as u128;
_2 = (-9223372036854775808_isize) & 9223372036854775807_isize;
RET = '\u{27fb2}';
Goto(bb15)
}
bb15 = {
_4 = [_7,_7,_7,_7,_7,_7];
_12 = [(-2244433305890720165_i64),4989087407492228391_i64,(-1895959535539644995_i64),(-1922455912619846738_i64)];
_7 = 142247648172670580080965695838991517470_u128 * 331257333884119147988862400255074375859_u128;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{80ad7}';
Goto(bb16)
}
bb16 = {
RET = '\u{4550}';
_15 = 5603233592290196726_i64 as usize;
_2 = 22_isize * 45_isize;
_16 = !_15;
_15 = _3 as usize;
_16 = _15 + _15;
_15 = _7 as usize;
_15 = _16;
_1 = _3 == _3;
_7 = _3 as u128;
_15 = _16 * _16;
_14 = 125_u8 as f64;
_7 = 169720104258499787732760506338572135088_u128 - 305820586901475651133668724052654634821_u128;
_7 = !72025326262768016326539236962386322356_u128;
_1 = _15 > _15;
_16 = _15 & _15;
_16 = _15 << _15;
_3 = !_1;
_15 = _14 as usize;
_12 = [(-8910334950137922598_i64),3112599287021860239_i64,(-9017090870604989164_i64),247687495110593691_i64];
_4 = [_7,_7,_7,_7,_7,_7];
_12 = [6960838772347644252_i64,(-7540775458254883364_i64),(-3076927375762826454_i64),1975978109737486360_i64];
_3 = _1 == _1;
Goto(bb17)
}
bb17 = {
_12 = [(-66763878309073806_i64),(-7532866891788428404_i64),7816218929746741198_i64,(-7554646278928968594_i64)];
_16 = _15;
_8 = core::ptr::addr_of_mut!(_18.1);
_7 = !244804299716341845403184997454322857175_u128;
_17 = [1630704604_u32,1341765818_u32];
_3 = _1 < _1;
_1 = !_3;
_1 = _3 < _3;
_14 = 160_u8 as f64;
Goto(bb18)
}
bb18 = {
_20 = _14 - _14;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{e2f2e}';
_8 = core::ptr::addr_of_mut!((*_8));
_12 = [5813666711100999277_i64,3783442226618774488_i64,(-5661348700874023674_i64),6174963025674777150_i64];
_1 = _3 & _3;
_7 = 106477010830364721476188478456287662403_u128;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = 3563595132817718948_u64 as isize;
_1 = _3 == _3;
_22 = RET;
match _7 {
0 => bb7,
1 => bb4,
106477010830364721476188478456287662403 => bb19,
_ => bb11
}
}
bb19 = {
_7 = _15 as u128;
Goto(bb20)
}
bb20 = {
_3 = _1 & _1;
_22 = RET;
_25 = (-116920434298717672033872467356380250925_i128) * 85279567315196948079702828636713574636_i128;
_14 = _20 + _20;
_9 = core::ptr::addr_of_mut!(_25);
_24 = _16 + _16;
_17 = [1529981839_u32,2707281652_u32];
(*_9) = !(-43475597081697286405077077645180393663_i128);
(*_9) = _14 as i128;
_3 = _1 ^ _1;
_24 = _15 & _15;
_25 = (-142396486186244735889892524542118565203_i128) & (-104848687669065296144848448735221921832_i128);
_12 = [62083201539658351_i64,5645961696823967172_i64,(-1407661539610759692_i64),1036410733858571626_i64];
(*_9) = (-83596281028521558351062841025809833180_i128);
(*_9) = 149294520412283774298937993258943008241_i128 ^ (-119603549377385930542705942217689387752_i128);
_29 = -(-101_i8);
(*_9) = -109566009971830593860248331540628897337_i128;
(*_9) = -(-151137683447531403104667750674644196114_i128);
_4 = [_7,_7,_7,_7,_7,_7];
(*_9) = 133012368870191549811405312337055564059_i128;
_31.0.1 = Adt19::Variant1 { fld0: _29,fld1: (*_9),fld2: 1184847016_i32 };
Goto(bb21)
}
bb21 = {
_25 = 178_u8 as i128;
(*_9) = (-1936_i16) as i128;
(*_9) = !Field::<i128>(Variant(_31.0.1, 1), 1);
Goto(bb22)
}
bb22 = {
_23 = (*_9) as isize;
match Field::<i128>(Variant(_31.0.1, 1), 1) {
0 => bb1,
1 => bb6,
2 => bb14,
3 => bb12,
4 => bb23,
5 => bb24,
133012368870191549811405312337055564059 => bb26,
_ => bb25
}
}
bb23 = {
_20 = _14 - _14;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{e2f2e}';
_8 = core::ptr::addr_of_mut!((*_8));
_12 = [5813666711100999277_i64,3783442226618774488_i64,(-5661348700874023674_i64),6174963025674777150_i64];
_1 = _3 & _3;
_7 = 106477010830364721476188478456287662403_u128;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = 3563595132817718948_u64 as isize;
_1 = _3 == _3;
_22 = RET;
match _7 {
0 => bb7,
1 => bb4,
106477010830364721476188478456287662403 => bb19,
_ => bb11
}
}
bb24 = {
_12 = [(-66763878309073806_i64),(-7532866891788428404_i64),7816218929746741198_i64,(-7554646278928968594_i64)];
_16 = _15;
_8 = core::ptr::addr_of_mut!(_18.1);
_7 = !244804299716341845403184997454322857175_u128;
_17 = [1630704604_u32,1341765818_u32];
_3 = _1 < _1;
_1 = !_3;
_1 = _3 < _3;
_14 = 160_u8 as f64;
Goto(bb18)
}
bb25 = {
_1 = _3;
_1 = _7 == _7;
_7 = 190233492307082658980956920217311506887_u128 | 159245074898605179267185850881605288745_u128;
RET = '\u{5c8db}';
_7 = !187923729228759596336115954278435961918_u128;
RET = '\u{10ce55}';
_1 = _3 > _3;
_4 = [_7,_7,_7,_7,_7,_7];
_1 = !_3;
_7 = 280780548121693286190705014818248750876_u128;
_1 = _3;
_3 = RET > RET;
_12 = [(-3862246428249132689_i64),(-2594543489971451069_i64),(-1238053017923894022_i64),2662484436877636598_i64];
_12 = [(-9083166824021557269_i64),4015422671189955839_i64,4883610683647907255_i64,(-3388989061000245348_i64)];
_3 = _1 ^ _1;
RET = '\u{27854}';
_1 = _3 | _3;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{737a0}';
_14 = 3759935147_u32 as f64;
_2 = -(-9223372036854775808_isize);
_14 = (-104_i8) as f64;
Goto(bb14)
}
bb26 = {
(*_9) = Field::<i128>(Variant(_31.0.1, 1), 1) << _16;
(*_9) = Field::<i128>(Variant(_31.0.1, 1), 1) >> _2;
match Field::<i128>(Variant(_31.0.1, 1), 1) {
0 => bb9,
133012368870191549811405312337055564059 => bb27,
_ => bb14
}
}
bb27 = {
(*_9) = _14 as i128;
_8 = core::ptr::addr_of_mut!((*_8));
_15 = _24 >> _25;
(*_9) = 1508911751573582482_i64 as i128;
_15 = 688774708_i32 as usize;
_8 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i8>(Variant(_31.0.1, 1), 0)) = _29 & _29;
match Field::<i128>(Variant(_31.0.1, 1), 1) {
0 => bb5,
133012368870191549811405312337055564059 => bb29,
_ => bb28
}
}
bb28 = {
_7 = 15339363798673402955992712058622789893_u128;
_4 = [_7,_7,_7,_7,_7,_7];
_2 = _3 as isize;
_3 = !_1;
_7 = _3 as u128;
_2 = (-9223372036854775808_isize) & 9223372036854775807_isize;
RET = '\u{27fb2}';
Goto(bb15)
}
bb29 = {
_38 = core::ptr::addr_of_mut!((*_8));
_31.0.0 = _22;
_36 = (-21022_i16);
_24 = _29 as usize;
(*_9) = Field::<i128>(Variant(_31.0.1, 1), 1) << _2;
_16 = 1043974144527005086_i64 as usize;
(*_9) = -Field::<i128>(Variant(_31.0.1, 1), 1);
Goto(bb30)
}
bb30 = {
_9 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_31.0.1, 1), 1)));
(*_9) = _25 << _29;
_14 = _20 * _20;
_1 = _3;
place!(Field::<i8>(Variant(_31.0.1, 1), 0)) = _29 | _29;
_33 = !_2;
_35 = core::ptr::addr_of!(_2);
(*_35) = _33 | _23;
(*_9) = _25 & _25;
(*_9) = _25;
Goto(bb31)
}
bb31 = {
_27 = _3 & _1;
(*_9) = _25;
_2 = _33 * _33;
(*_9) = _7 as i128;
(*_35) = -_33;
(*_9) = _25 - _25;
(*_35) = _33 | _33;
(*_35) = _23;
_38 = core::ptr::addr_of_mut!((*_38));
(*_35) = 17281_u16 as isize;
_14 = _20;
Goto(bb32)
}
bb32 = {
(*_35) = _36 as isize;
_7 = 20551816862443988019376531848573841867_u128;
_2 = !_23;
_2 = _23 ^ _33;
(*_9) = -_25;
_17 = [1390129163_u32,3302717065_u32];
_37.0 = _22;
_14 = (*_35) as f64;
_37 = (RET,);
_21 = _37.0;
place!(Field::<i8>(Variant(_31.0.1, 1), 0)) = _29 + _29;
place!(Field::<i128>(Variant(_31.0.1, 1), 1)) = _25 & _25;
_38 = core::ptr::addr_of_mut!((*_38));
_25 = (-1266762548_i32) as i128;
RET = _22;
(*_35) = !_33;
(*_35) = _27 as isize;
_46 = [_2,(*_35),(*_35),(*_35),(*_35),(*_35)];
_18.0 = &mut _36;
match _7 {
0 => bb27,
1 => bb33,
20551816862443988019376531848573841867 => bb35,
_ => bb34
}
}
bb33 = {
_4 = [_7,_7,_7,_7,_7,_7];
match _2 {
0 => bb1,
1 => bb4,
2 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb34 = {
_23 = (*_9) as isize;
match Field::<i128>(Variant(_31.0.1, 1), 1) {
0 => bb1,
1 => bb6,
2 => bb14,
3 => bb12,
4 => bb23,
5 => bb24,
133012368870191549811405312337055564059 => bb26,
_ => bb25
}
}
bb35 = {
_46 = [(*_35),(*_35),(*_35),(*_35),(*_35),(*_35)];
_41 = 1714155105_i32 as f32;
_41 = (-1802893286_i32) as f32;
(*_35) = _23 << Field::<i8>(Variant(_31.0.1, 1), 0);
(*_9) = !_25;
_45 = _14 * _20;
Goto(bb36)
}
bb36 = {
_16 = _15 - _15;
_25 = (*_9) << _2;
(*_9) = _45 as i128;
(*_9) = _25 * _25;
_33 = (*_35);
(*_35) = _7 as isize;
(*_35) = 6538849393063794114_i64 as isize;
_12 = [(-832147377726332436_i64),(-889173973371596304_i64),2566698960255365325_i64,7136576496089967999_i64];
_44 = _41;
(*_9) = _25;
(*_9) = _25;
(*_9) = _7 as i128;
_48.fld4 = 21871_u16 * 8726_u16;
(*_9) = 3690183380_u32 as i128;
_35 = core::ptr::addr_of!((*_35));
_9 = core::ptr::addr_of_mut!((*_9));
_31.0.0 = _37.0;
_5 = &_48.fld4;
Goto(bb37)
}
bb37 = {
place!(Field::<i128>(Variant(_31.0.1, 1), 1)) = _25 ^ _25;
(*_9) = _25 + _25;
_23 = (*_35);
_14 = _45;
(*_35) = _33 << _24;
_48.fld0 = _1 ^ _27;
(*_9) = 192_u8 as i128;
(*_35) = _33 | _23;
(*_35) = _23 - _23;
match _7 {
0 => bb29,
1 => bb22,
2 => bb9,
3 => bb16,
4 => bb5,
5 => bb17,
6 => bb26,
20551816862443988019376531848573841867 => bb39,
_ => bb38
}
}
bb38 = {
RET = '\u{4a32b}';
Call(_1 = fn6(RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb39 = {
_26 = &(*_5);
_51 = Move(_18.0);
(*_9) = _25 | _25;
(*_9) = _25;
_14 = _20;
_31.0.0 = _37.0;
_37.0 = _31.0.0;
_20 = -_45;
(*_9) = !_25;
_8 = core::ptr::addr_of_mut!((*_8));
_50 = _31.0.0;
(*_35) = !_33;
_48.fld2 = _16;
_48.fld5 = (*_5) as u32;
(*_35) = _33;
_7 = 303917760599229948815547146410009939430_u128 >> (*_35);
_48.fld3 = _29 ^ Field::<i8>(Variant(_31.0.1, 1), 0);
_2 = _33;
(*_35) = _33;
(*_9) = _25 & _25;
_54 = (-1480316780550606632_i64);
_38 = core::ptr::addr_of_mut!((*_8));
_44 = _29 as f32;
Goto(bb40)
}
bb40 = {
(*_35) = _33 | _33;
RET = _37.0;
(*_35) = _33 >> _54;
_50 = RET;
_33 = (*_9) as isize;
(*_9) = -_25;
_58 = (*_35);
_1 = !_48.fld0;
_48.fld1 = -_41;
Goto(bb41)
}
bb41 = {
(*_35) = _33;
_38 = core::ptr::addr_of_mut!((*_38));
_57 = _48.fld3 as f32;
(*_35) = _33;
_37 = (_22,);
_7 = !118016551807445179502439615380319441762_u128;
_7 = 247120430002785020647912713269140643417_u128 * 169698157569303665354356558928759121053_u128;
(*_9) = _25 ^ _25;
place!(Field::<i128>(Variant(_31.0.1, 1), 1)) = !_25;
_41 = _44 + _48.fld1;
_46 = [(*_35),(*_35),(*_35),(*_35),(*_35),(*_35)];
_60 = _31.0.0;
(*_9) = !_25;
(*_35) = _58 - _58;
_41 = _57 + _44;
(*_9) = _48.fld5 as i128;
_14 = _20 + _45;
Goto(bb42)
}
bb42 = {
_48.fld2 = _24 ^ _16;
_61 = [16337314317597025561_u64,15865318201114902078_u64,1170228378164134547_u64,9711990329377761419_u64];
_24 = _48.fld2 + _15;
_21 = _31.0.0;
(*_35) = _33 << _48.fld3;
_65 = (*_35) - _2;
_22 = _50;
_48.fld4 = 50743_u16;
(*_35) = _65 & _65;
_46 = [(*_35),_33,(*_35),(*_35),(*_35),(*_35)];
(*_9) = -_25;
(*_9) = _25 | _25;
_68 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_65 = (*_9) as isize;
match _48.fld4 {
0 => bb5,
1 => bb20,
2 => bb24,
50743 => bb43,
_ => bb32
}
}
bb43 = {
(*_35) = _65;
(*_9) = _25 >> _33;
_49 = (*_9) as f32;
_44 = _57;
_48 = Adt25 { fld0: _27,fld1: _49,fld2: _15,fld3: Field::<i8>(Variant(_31.0.1, 1), 0),fld4: 3061_u16,fld5: 2660110217_u32 };
(*_9) = -_25;
_20 = -_14;
_57 = -_44;
(*_9) = !_25;
_29 = -Field::<i8>(Variant(_31.0.1, 1), 0);
(*_9) = _25;
_66.0 = 5152712172100032110_u64 as u8;
_48.fld2 = !_16;
_37.0 = RET;
_66 = (82_u8, 84_u8);
_6 = &_48.fld4;
place!(Field::<i8>(Variant(_31.0.1, 1), 0)) = _29;
(*_9) = _25 << (*_6);
_48.fld5 = 2249888571_u32 << (*_9);
_31.0.0 = _21;
_71 = 3754_i16 as f32;
_8 = core::ptr::addr_of_mut!((*_8));
(*_9) = -_25;
_16 = _20 as usize;
_31.0.1 = Adt19::Variant1 { fld0: _48.fld3,fld1: _25,fld2: (-815228989_i32) };
Goto(bb44)
}
bb44 = {
_27 = (*_6) > (*_6);
(*_35) = _65 ^ _65;
_38 = core::ptr::addr_of_mut!((*_38));
_57 = _66.1 as f32;
_73.2 = (_48.fld5, _48.fld1, _3);
_66 = (186_u8, 182_u8);
_37.0 = _21;
_32 = &mut (*_6);
_56.0.1 = Adt19::Variant1 { fld0: _29,fld1: Field::<i128>(Variant(_31.0.1, 1), 1),fld2: 927389865_i32 };
_60 = _50;
_73.0 = Move(_9);
_31.0.0 = _22;
_66 = (136_u8, 200_u8);
(*_35) = (*_32) as isize;
_20 = _14;
_37.0 = _31.0.0;
(*_35) = _33 + _33;
_73.2.1 = 6318843628277331850_u64 as f32;
_1 = !_3;
_57 = _44;
_27 = _3;
match (*_32) {
0 => bb12,
1 => bb30,
2 => bb45,
3 => bb46,
3061 => bb48,
_ => bb47
}
}
bb45 = {
RET = '\u{313fb}';
match _2 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb5,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb46 = {
_1 = _3;
_1 = _7 == _7;
_7 = 190233492307082658980956920217311506887_u128 | 159245074898605179267185850881605288745_u128;
RET = '\u{5c8db}';
_7 = !187923729228759596336115954278435961918_u128;
RET = '\u{10ce55}';
_1 = _3 > _3;
_4 = [_7,_7,_7,_7,_7,_7];
_1 = !_3;
_7 = 280780548121693286190705014818248750876_u128;
_1 = _3;
_3 = RET > RET;
_12 = [(-3862246428249132689_i64),(-2594543489971451069_i64),(-1238053017923894022_i64),2662484436877636598_i64];
_12 = [(-9083166824021557269_i64),4015422671189955839_i64,4883610683647907255_i64,(-3388989061000245348_i64)];
_3 = _1 ^ _1;
RET = '\u{27854}';
_1 = _3 | _3;
_4 = [_7,_7,_7,_7,_7,_7];
RET = '\u{737a0}';
_14 = 3759935147_u32 as f64;
_2 = -(-9223372036854775808_isize);
_14 = (-104_i8) as f64;
Goto(bb14)
}
bb47 = {
_1 = !true;
_1 = true;
_1 = !false;
RET = '\u{c63ef}';
_1 = false & true;
RET = '\u{33e0b}';
RET = '\u{71f8d}';
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = !_1;
_3 = _1 == _1;
_3 = !_1;
_3 = !_1;
_3 = _1;
_4 = [88527177725152584702444187432585276628_u128,291271873107620463465544913787915165901_u128,307502546046487705244197423797186370214_u128,119206013539530678207743579683705734109_u128,79542517118125160551427610965299252275_u128,188512572650188408782850375384340401807_u128];
_7 = 148778095022499963106678600075090560488_u128 + 76122375549378057529722996201518740777_u128;
RET = '\u{b4254}';
_3 = _1;
_7 = _1 as u128;
RET = '\u{b3a8c}';
_1 = !_3;
_1 = !_3;
_3 = _1 | _1;
_1 = _2 == _2;
_7 = 279606776570934205477945705201052270396_u128 & 303820743820011367495613567011666191543_u128;
_2 = (-9223372036854775808_isize);
match _2 {
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb48 = {
_72 = (*_35) != (*_35);
_67 = core::ptr::addr_of!(_31.0.0);
_58 = _1 as isize;
_22 = (*_67);
_55 = [15441941298502785292_u64,4590517741859498289_u64,14118567216347387763_u64,13041883574559346799_u64];
Goto(bb49)
}
bb49 = {
(*_35) = _1 as isize;
_65 = (*_35) << (*_32);
_7 = 80464226831405402495181878642915717419_u128 - 41250194556851006559827935726112627462_u128;
(*_35) = -_58;
(*_35) = _65;
_57 = _41 * _41;
(*_35) = (*_67) as isize;
_34 = core::ptr::addr_of!(_74.3);
_3 = !_73.2.2;
Goto(bb50)
}
bb50 = {
Call(_78 = dump_var(Move(_60), Move(_25), Move(_37), Move(_58)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_78 = dump_var(Move(_61), Move(_16), Move(_17), Move(_33)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_78 = dump_var(Move(_7), Move(_29), Move(_22), Move(_72)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_78 = dump_var(Move(_1), Move(_50), Move(_68), _79), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: char,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char) -> bool {
mir! {
type RET = bool;
let _10: f64;
let _11: [u16; 3];
let _12: (f64,);
let _13: f32;
let _14: u128;
let _15: isize;
let _16: &'static mut [i128; 6];
let _17: char;
let _18: bool;
let _19: &'static (u32, f32, bool);
let _20: ((char, Adt19),);
let _21: &'static Adt19;
let _22: f32;
let _23: bool;
let _24: (&'static mut i16, *mut [u64; 5]);
let _25: char;
let _26: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _27: i64;
let _28: *const (u8, usize, i128, i128);
let _29: Adt46;
let _30: &'static u16;
let _31: f32;
let _32: &'static Adt19;
let _33: Adt45;
let _34: [i64; 4];
let _35: isize;
let _36: &'static mut u16;
let _37: bool;
let _38: *mut *mut [u64; 5];
let _39: f32;
let _40: i128;
let _41: u32;
let _42: f32;
let _43: u16;
let _44: &'static u128;
let _45: *const u128;
let _46: *const *mut i32;
let _47: f64;
let _48: *const (u8, usize, i128, i128);
let _49: u16;
let _50: u32;
let _51: *const (u8, usize, i128, i128);
let _52: bool;
let _53: f32;
let _54: *mut *mut [u64; 5];
let _55: u16;
let _56: &'static u128;
let _57: *mut [u64; 5];
let _58: i8;
let _59: i64;
let _60: &'static Adt19;
let _61: Adt47;
let _62: &'static mut [i128; 6];
let _63: isize;
let _64: u64;
let _65: char;
let _66: f32;
let _67: [isize; 5];
let _68: *const u128;
let _69: ();
let _70: ();
{
_7 = _8;
_4 = _3;
RET = true;
_7 = _3;
_7 = _2;
_8 = _6;
_1 = _6;
_6 = _1;
_1 = _8;
RET = false;
_2 = _9;
_7 = _1;
_1 = _9;
_7 = _9;
_4 = _2;
_4 = _9;
RET = !true;
_10 = 8119214406063892181_u64 as f64;
_5 = _1;
_8 = _6;
Goto(bb1)
}
bb1 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _1;
_12 = (_10,);
_5 = _3;
_12 = (_10,);
_9 = _4;
_5 = _7;
_14 = (-33643931303288171987504494688091294537_i128) as u128;
_15 = 43_isize - (-9223372036854775808_isize);
_10 = -_12.0;
_5 = _7;
_4 = _8;
_17 = _7;
_14 = (-36347652295210616424095003280849368130_i128) as u128;
_14 = 297567738997484882195112011836831623243_u128;
_7 = _17;
_12 = (_10,);
_8 = _3;
_6 = _7;
Goto(bb3)
}
bb3 = {
_12 = (_10,);
_9 = _8;
_14 = 18396265524762350056_usize as u128;
_7 = _4;
_14 = 140728573375295219683803934128664933615_u128 - 270660140248108033738958704749650639082_u128;
_15 = 9223372036854775807_isize & (-76_isize);
_20.0.0 = _8;
_8 = _3;
_23 = RET;
_10 = _12.0;
_7 = _6;
_9 = _17;
_15 = (-53_isize);
_12 = (_10,);
_20.0.1 = Adt19::Variant1 { fld0: 54_i8,fld1: (-164739263372447333395599851826191318521_i128),fld2: (-1705123763_i32) };
_18 = _23;
place!(Field::<i8>(Variant(_20.0.1, 1), 0)) = 7101696574732010597_usize as i8;
RET = !_23;
_17 = _1;
_14 = 164358676942928924725226077811360886190_u128 - 312059998037182395552802214100498254715_u128;
_20.0.0 = _4;
_22 = _14 as f32;
_6 = _3;
Goto(bb4)
}
bb4 = {
_5 = _3;
_20.0.0 = _2;
_26.3.0 = _9;
_25 = _3;
_25 = _9;
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = (-64271119021675291799357369031155362521_i128) ^ 70533409756295363225031509267139479175_i128;
_7 = _26.3.0;
_10 = _12.0;
_10 = _15 as f64;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = -(-1645314416_i32);
_21 = &_20.0.1;
_26.3.0 = _7;
_26.3.0 = _7;
_15 = 38256_u16 as isize;
_13 = 36_u8 as f32;
_26.2.1 = _22;
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = -14139629723706751452446966631303295897_i128;
_26.2 = (1352548940_u32, _22, _23);
_26.3 = (_1,);
_13 = Field::<i8>(Variant((*_21), 1), 0) as f32;
_27 = 1925714148060657875_i64 | 1985284464593882700_i64;
_27 = !(-4590531614846546797_i64);
_17 = _7;
_8 = _7;
_26.2 = (3442800328_u32, _13, _23);
_29.fld3 = _26.2;
match _26.2.0 {
0 => bb3,
1 => bb2,
3442800328 => bb6,
_ => bb5
}
}
bb5 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
RET = _29.fld3.2;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_26.3.0 = _8;
_27 = 654943192664360925_i64;
_29.fld4.fld3 = !Field::<i8>(Variant((*_21), 1), 0);
_29.fld3 = (_26.2.0, _22, _26.2.2);
_3 = _5;
_12.0 = -_10;
_15 = (-30_isize);
_13 = -_29.fld3.1;
_29.fld4.fld4 = 33090_u16 * 41501_u16;
_29.fld4.fld1 = _14 as f32;
_29.fld3.0 = 11501156207027043981_u64 as u32;
_19 = &_26.2;
_31 = _22 - _13;
_29.fld6 = !_27;
_26.1 = [227_u8,49_u8,131_u8,11_u8,129_u8,33_u8,200_u8];
_29.fld4.fld5 = (*_19).0 + (*_19).0;
_9 = _7;
_27 = _29.fld6 + _29.fld6;
_29.fld6 = (*_19).1 as i64;
_29.fld4.fld2 = 6_usize;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1919878803_i32 & (-974297787_i32);
_17 = _4;
_34 = [_27,_27,_27,_27];
_17 = _25;
_29.fld4.fld3 = -Field::<i8>(Variant((*_21), 1), 0);
Goto(bb7)
}
bb7 = {
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_29.fld5 = Adt19::Variant1 { fld0: Field::<i8>(Variant((*_21), 1), 0),fld1: Field::<i128>(Variant(_20.0.1, 1), 1),fld2: Field::<i32>(Variant(_20.0.1, 1), 2) };
_26.2.0 = _29.fld4.fld5;
_29.fld7 = !Field::<i128>(Variant(_20.0.1, 1), 1);
_29.fld4.fld0 = Field::<i8>(Variant((*_21), 1), 0) == Field::<i8>(Variant((*_21), 1), 0);
_29.fld4.fld3 = Field::<i8>(Variant((*_21), 1), 0) ^ Field::<i8>(Variant((*_21), 1), 0);
place!(Field::<i32>(Variant(_29.fld5, 1), 2)) = Field::<i128>(Variant(_20.0.1, 1), 1) as i32;
_20.0.1 = Move(_29.fld5);
_3 = _1;
_29.fld4.fld2 = 9020524197087284339_usize;
_27 = !_29.fld6;
_29.fld7 = _29.fld6 as i128;
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _29.fld7 + _29.fld7;
_20.0.0 = _4;
_30 = &_29.fld4.fld4;
_40 = _29.fld7 & Field::<i128>(Variant(_20.0.1, 1), 1);
_29.fld3 = (_29.fld4.fld5, (*_19).1, _18);
match _29.fld4.fld2 {
0 => bb6,
1 => bb8,
2 => bb9,
3 => bb10,
9020524197087284339 => bb12,
_ => bb11
}
}
bb8 = {
RET = _29.fld3.2;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_26.3.0 = _8;
_27 = 654943192664360925_i64;
_29.fld4.fld3 = !Field::<i8>(Variant((*_21), 1), 0);
_29.fld3 = (_26.2.0, _22, _26.2.2);
_3 = _5;
_12.0 = -_10;
_15 = (-30_isize);
_13 = -_29.fld3.1;
_29.fld4.fld4 = 33090_u16 * 41501_u16;
_29.fld4.fld1 = _14 as f32;
_29.fld3.0 = 11501156207027043981_u64 as u32;
_19 = &_26.2;
_31 = _22 - _13;
_29.fld6 = !_27;
_26.1 = [227_u8,49_u8,131_u8,11_u8,129_u8,33_u8,200_u8];
_29.fld4.fld5 = (*_19).0 + (*_19).0;
_9 = _7;
_27 = _29.fld6 + _29.fld6;
_29.fld6 = (*_19).1 as i64;
_29.fld4.fld2 = 6_usize;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1919878803_i32 & (-974297787_i32);
_17 = _4;
_34 = [_27,_27,_27,_27];
_17 = _25;
_29.fld4.fld3 = -Field::<i8>(Variant((*_21), 1), 0);
Goto(bb7)
}
bb9 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_8 = _1;
_12 = (_10,);
_5 = _3;
_12 = (_10,);
_9 = _4;
_5 = _7;
_14 = (-33643931303288171987504494688091294537_i128) as u128;
_15 = 43_isize - (-9223372036854775808_isize);
_10 = -_12.0;
_5 = _7;
_4 = _8;
_17 = _7;
_14 = (-36347652295210616424095003280849368130_i128) as u128;
_14 = 297567738997484882195112011836831623243_u128;
_7 = _17;
_12 = (_10,);
_8 = _3;
_6 = _7;
Goto(bb3)
}
bb11 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_29.fld0 = [9315396867836329680_u64,12268429753324109156_u64,4312034197770117730_u64,16253741659685790467_u64,10879469664377708865_u64];
_3 = _5;
_17 = _20.0.0;
_29.fld3.0 = _26.2.0 << _29.fld4.fld5;
_26.2.1 = _40 as f32;
_29.fld3 = (_29.fld4.fld5, _29.fld4.fld1, (*_19).2);
_29.fld4.fld1 = _15 as f32;
place!(Field::<i8>(Variant(_20.0.1, 1), 0)) = _29.fld4.fld3 | _29.fld4.fld3;
_43 = !(*_30);
_29.fld4.fld4 = !_43;
_5 = _6;
_29.fld4.fld2 = 5_usize >> _29.fld7;
_12 = (_10,);
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _40 << Field::<i32>(Variant(_20.0.1, 1), 2);
_29.fld5 = Adt19::Variant0 { fld0: (*_19).2,fld1: 46_u8,fld2: _15,fld3: _12,fld4: _29.fld4.fld2,fld5: _43 };
_9 = _26.3.0;
_38 = core::ptr::addr_of_mut!(_24.1);
place!(Field::<usize>(Variant(_29.fld5, 0), 4)) = !_29.fld4.fld2;
_29.fld2 = 3729_i16 as usize;
match _15 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607431768211426 => bb18,
_ => bb17
}
}
bb13 = {
_12 = (_10,);
_9 = _8;
_14 = 18396265524762350056_usize as u128;
_7 = _4;
_14 = 140728573375295219683803934128664933615_u128 - 270660140248108033738958704749650639082_u128;
_15 = 9223372036854775807_isize & (-76_isize);
_20.0.0 = _8;
_8 = _3;
_23 = RET;
_10 = _12.0;
_7 = _6;
_9 = _17;
_15 = (-53_isize);
_12 = (_10,);
_20.0.1 = Adt19::Variant1 { fld0: 54_i8,fld1: (-164739263372447333395599851826191318521_i128),fld2: (-1705123763_i32) };
_18 = _23;
place!(Field::<i8>(Variant(_20.0.1, 1), 0)) = 7101696574732010597_usize as i8;
RET = !_23;
_17 = _1;
_14 = 164358676942928924725226077811360886190_u128 - 312059998037182395552802214100498254715_u128;
_20.0.0 = _4;
_22 = _14 as f32;
_6 = _3;
Goto(bb4)
}
bb14 = {
RET = _29.fld3.2;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_26.3.0 = _8;
_27 = 654943192664360925_i64;
_29.fld4.fld3 = !Field::<i8>(Variant((*_21), 1), 0);
_29.fld3 = (_26.2.0, _22, _26.2.2);
_3 = _5;
_12.0 = -_10;
_15 = (-30_isize);
_13 = -_29.fld3.1;
_29.fld4.fld4 = 33090_u16 * 41501_u16;
_29.fld4.fld1 = _14 as f32;
_29.fld3.0 = 11501156207027043981_u64 as u32;
_19 = &_26.2;
_31 = _22 - _13;
_29.fld6 = !_27;
_26.1 = [227_u8,49_u8,131_u8,11_u8,129_u8,33_u8,200_u8];
_29.fld4.fld5 = (*_19).0 + (*_19).0;
_9 = _7;
_27 = _29.fld6 + _29.fld6;
_29.fld6 = (*_19).1 as i64;
_29.fld4.fld2 = 6_usize;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1919878803_i32 & (-974297787_i32);
_17 = _4;
_34 = [_27,_27,_27,_27];
_17 = _25;
_29.fld4.fld3 = -Field::<i8>(Variant((*_21), 1), 0);
Goto(bb7)
}
bb15 = {
_8 = _1;
_12 = (_10,);
_5 = _3;
_12 = (_10,);
_9 = _4;
_5 = _7;
_14 = (-33643931303288171987504494688091294537_i128) as u128;
_15 = 43_isize - (-9223372036854775808_isize);
_10 = -_12.0;
_5 = _7;
_4 = _8;
_17 = _7;
_14 = (-36347652295210616424095003280849368130_i128) as u128;
_14 = 297567738997484882195112011836831623243_u128;
_7 = _17;
_12 = (_10,);
_8 = _3;
_6 = _7;
Goto(bb3)
}
bb16 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_29.fld5 = Adt19::Variant1 { fld0: Field::<i8>(Variant((*_21), 1), 0),fld1: Field::<i128>(Variant(_20.0.1, 1), 1),fld2: Field::<i32>(Variant(_20.0.1, 1), 2) };
_26.2.0 = _29.fld4.fld5;
_29.fld7 = !Field::<i128>(Variant(_20.0.1, 1), 1);
_29.fld4.fld0 = Field::<i8>(Variant((*_21), 1), 0) == Field::<i8>(Variant((*_21), 1), 0);
_29.fld4.fld3 = Field::<i8>(Variant((*_21), 1), 0) ^ Field::<i8>(Variant((*_21), 1), 0);
place!(Field::<i32>(Variant(_29.fld5, 1), 2)) = Field::<i128>(Variant(_20.0.1, 1), 1) as i32;
_20.0.1 = Move(_29.fld5);
_3 = _1;
_29.fld4.fld2 = 9020524197087284339_usize;
_27 = !_29.fld6;
_29.fld7 = _29.fld6 as i128;
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _29.fld7 + _29.fld7;
_20.0.0 = _4;
_30 = &_29.fld4.fld4;
_40 = _29.fld7 & Field::<i128>(Variant(_20.0.1, 1), 1);
_29.fld3 = (_29.fld4.fld5, (*_19).1, _18);
match _29.fld4.fld2 {
0 => bb6,
1 => bb8,
2 => bb9,
3 => bb10,
9020524197087284339 => bb12,
_ => bb11
}
}
bb18 = {
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = -_40;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_29.fld3.1 = _22 - _13;
_44 = &_14;
_42 = (*_44) as f32;
_26.2.1 = _31 - _31;
place!(Field::<bool>(Variant(_29.fld5, 0), 0)) = !_29.fld4.fld0;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_6 = _17;
_32 = &_20.0.1;
_29.fld2 = _29.fld4.fld2 & _29.fld4.fld2;
_29.fld4.fld0 = !Field::<bool>(Variant(_29.fld5, 0), 0);
_34 = [_27,_27,_27,_29.fld6];
_9 = _25;
_26.2 = _29.fld3;
_39 = _29.fld4.fld1 * _29.fld3.1;
_3 = _9;
_20.0.0 = _4;
_35 = _15;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1614572385_i32 | 567643733_i32;
_42 = _31 + _13;
_26.3 = (_17,);
place!(Field::<u8>(Variant(_29.fld5, 0), 1)) = 251_u8 << (*_44);
_41 = _29.fld3.0 >> Field::<i8>(Variant((*_32), 1), 0);
_29.fld6 = !_27;
_47 = -_12.0;
match Field::<isize>(Variant(_29.fld5, 0), 2) {
0 => bb10,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
340282366920938463463374607431768211426 => bb26,
_ => bb25
}
}
bb19 = {
_12 = (_10,);
_9 = _8;
_14 = 18396265524762350056_usize as u128;
_7 = _4;
_14 = 140728573375295219683803934128664933615_u128 - 270660140248108033738958704749650639082_u128;
_15 = 9223372036854775807_isize & (-76_isize);
_20.0.0 = _8;
_8 = _3;
_23 = RET;
_10 = _12.0;
_7 = _6;
_9 = _17;
_15 = (-53_isize);
_12 = (_10,);
_20.0.1 = Adt19::Variant1 { fld0: 54_i8,fld1: (-164739263372447333395599851826191318521_i128),fld2: (-1705123763_i32) };
_18 = _23;
place!(Field::<i8>(Variant(_20.0.1, 1), 0)) = 7101696574732010597_usize as i8;
RET = !_23;
_17 = _1;
_14 = 164358676942928924725226077811360886190_u128 - 312059998037182395552802214100498254715_u128;
_20.0.0 = _4;
_22 = _14 as f32;
_6 = _3;
Goto(bb4)
}
bb20 = {
_8 = _1;
_12 = (_10,);
_5 = _3;
_12 = (_10,);
_9 = _4;
_5 = _7;
_14 = (-33643931303288171987504494688091294537_i128) as u128;
_15 = 43_isize - (-9223372036854775808_isize);
_10 = -_12.0;
_5 = _7;
_4 = _8;
_17 = _7;
_14 = (-36347652295210616424095003280849368130_i128) as u128;
_14 = 297567738997484882195112011836831623243_u128;
_7 = _17;
_12 = (_10,);
_8 = _3;
_6 = _7;
Goto(bb3)
}
bb21 = {
_8 = _1;
_12 = (_10,);
_5 = _3;
_12 = (_10,);
_9 = _4;
_5 = _7;
_14 = (-33643931303288171987504494688091294537_i128) as u128;
_15 = 43_isize - (-9223372036854775808_isize);
_10 = -_12.0;
_5 = _7;
_4 = _8;
_17 = _7;
_14 = (-36347652295210616424095003280849368130_i128) as u128;
_14 = 297567738997484882195112011836831623243_u128;
_7 = _17;
_12 = (_10,);
_8 = _3;
_6 = _7;
Goto(bb3)
}
bb22 = {
RET = _29.fld3.2;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_26.3.0 = _8;
_27 = 654943192664360925_i64;
_29.fld4.fld3 = !Field::<i8>(Variant((*_21), 1), 0);
_29.fld3 = (_26.2.0, _22, _26.2.2);
_3 = _5;
_12.0 = -_10;
_15 = (-30_isize);
_13 = -_29.fld3.1;
_29.fld4.fld4 = 33090_u16 * 41501_u16;
_29.fld4.fld1 = _14 as f32;
_29.fld3.0 = 11501156207027043981_u64 as u32;
_19 = &_26.2;
_31 = _22 - _13;
_29.fld6 = !_27;
_26.1 = [227_u8,49_u8,131_u8,11_u8,129_u8,33_u8,200_u8];
_29.fld4.fld5 = (*_19).0 + (*_19).0;
_9 = _7;
_27 = _29.fld6 + _29.fld6;
_29.fld6 = (*_19).1 as i64;
_29.fld4.fld2 = 6_usize;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1919878803_i32 & (-974297787_i32);
_17 = _4;
_34 = [_27,_27,_27,_27];
_17 = _25;
_29.fld4.fld3 = -Field::<i8>(Variant((*_21), 1), 0);
Goto(bb7)
}
bb23 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb24 = {
_29.fld0 = [9315396867836329680_u64,12268429753324109156_u64,4312034197770117730_u64,16253741659685790467_u64,10879469664377708865_u64];
_3 = _5;
_17 = _20.0.0;
_29.fld3.0 = _26.2.0 << _29.fld4.fld5;
_26.2.1 = _40 as f32;
_29.fld3 = (_29.fld4.fld5, _29.fld4.fld1, (*_19).2);
_29.fld4.fld1 = _15 as f32;
place!(Field::<i8>(Variant(_20.0.1, 1), 0)) = _29.fld4.fld3 | _29.fld4.fld3;
_43 = !(*_30);
_29.fld4.fld4 = !_43;
_5 = _6;
_29.fld4.fld2 = 5_usize >> _29.fld7;
_12 = (_10,);
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _40 << Field::<i32>(Variant(_20.0.1, 1), 2);
_29.fld5 = Adt19::Variant0 { fld0: (*_19).2,fld1: 46_u8,fld2: _15,fld3: _12,fld4: _29.fld4.fld2,fld5: _43 };
_9 = _26.3.0;
_38 = core::ptr::addr_of_mut!(_24.1);
place!(Field::<usize>(Variant(_29.fld5, 0), 4)) = !_29.fld4.fld2;
_29.fld2 = 3729_i16 as usize;
match _15 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607431768211426 => bb18,
_ => bb17
}
}
bb25 = {
_6 = _8;
_10 = 153839197819745821088538809519058946114_i128 as f64;
_7 = _1;
_10 = 316564824884384038384450338055563988628_u128 as f64;
_6 = _8;
_7 = _2;
_10 = 307810751033716366757915113285117440834_u128 as f64;
_11 = [39235_u16,46714_u16,10871_u16];
_5 = _8;
_11 = [39965_u16,50919_u16,26327_u16];
_11 = [1625_u16,45829_u16,45329_u16];
_8 = _6;
_12.0 = _10 + _10;
_5 = _8;
_10 = -_12.0;
Call(_13 = fn7(_11, _2, _12, _1, _6, _4, _5, _6, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb26 = {
place!(Field::<usize>(Variant(_29.fld5, 0), 4)) = !_29.fld2;
place!(Field::<(f64,)>(Variant(_29.fld5, 0), 3)) = (_12.0,);
_22 = _42 * _13;
_26.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant((*_32), 1), 1)));
_18 = _29.fld4.fld0 ^ _29.fld4.fld0;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_18 = _29.fld4.fld0 | _26.2.2;
_26.2.1 = Field::<isize>(Variant(_29.fld5, 0), 2) as f32;
_5 = _6;
_20.0.0 = _8;
_19 = &_29.fld3;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_9 = _6;
_29.fld3.0 = _41 * _26.2.0;
_26.1 = [Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1)];
_29.fld4.fld1 = -(*_19).1;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
Goto(bb27)
}
bb27 = {
_47 = -_10;
_35 = _15 + Field::<isize>(Variant(_29.fld5, 0), 2);
_5 = _1;
_26.3 = (_2,);
_26.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant((*_32), 1), 1)));
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _40 | _29.fld7;
_29.fld0 = [17581823236511944265_u64,14950156849443596483_u64,5917776297700579777_u64,8672173671376346582_u64,13190460184575884422_u64];
_35 = _15 + Field::<isize>(Variant(_29.fld5, 0), 2);
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_13 = -_42;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_38 = core::ptr::addr_of_mut!((*_38));
_45 = core::ptr::addr_of!((*_44));
_11 = [_29.fld4.fld4,Field::<u16>(Variant(_29.fld5, 0), 5),_29.fld4.fld4];
_30 = &_43;
_50 = _29.fld3.0 & _29.fld4.fld5;
_41 = !_50;
_30 = &_29.fld4.fld4;
_35 = Field::<isize>(Variant(_29.fld5, 0), 2);
_26.0 = core::ptr::addr_of_mut!(_40);
_26.1 = [Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1),Field::<u8>(Variant(_29.fld5, 0), 1)];
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
match _15 {
0 => bb26,
1 => bb25,
340282366920938463463374607431768211426 => bb28,
_ => bb9
}
}
bb28 = {
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = _50 as i32;
(*_38) = core::ptr::addr_of_mut!(_29.fld0);
_26.2.0 = _41 & _50;
_52 = !_29.fld4.fld0;
place!(Field::<(f64,)>(Variant(_29.fld5, 0), 3)) = (_12.0,);
_29.fld3.1 = _22 + _22;
_21 = &_29.fld5;
_36 = &mut place!(Field::<u16>(Variant((*_21), 0), 5));
_23 = _26.2.0 <= _26.2.0;
_5 = _25;
_56 = &(*_44);
_45 = core::ptr::addr_of!((*_56));
_20.0.1 = Adt19::Variant1 { fld0: (-12_i8),fld1: _40,fld2: 922488985_i32 };
_26.3.0 = _4;
_8 = _3;
_43 = !(*_30);
Goto(bb29)
}
bb29 = {
_43 = (*_36) * (*_36);
_53 = _39 + _31;
_6 = _1;
_44 = Move(_56);
_37 = _23;
_54 = core::ptr::addr_of_mut!((*_38));
_56 = &(*_45);
_26.1 = [Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1)];
_2 = _8;
Goto(bb30)
}
bb30 = {
_55 = (*_36) - (*_36);
_11 = [(*_36),(*_36),(*_36)];
_12.0 = -Field::<(f64,)>(Variant((*_21), 0), 3).0;
_61.fld5.fld2 = Field::<usize>(Variant((*_21), 0), 4) ^ Field::<usize>(Variant((*_21), 0), 4);
_20.0.1 = Adt19::Variant1 { fld0: 70_i8,fld1: _40,fld2: (-1947704787_i32) };
_39 = _26.2.1;
_40 = Field::<i128>(Variant(_20.0.1, 1), 1) ^ Field::<i128>(Variant(_20.0.1, 1), 1);
_59 = _27 + _27;
_45 = core::ptr::addr_of!((*_45));
_26.1 = [Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1),Field::<u8>(Variant((*_21), 0), 1)];
match Field::<isize>(Variant((*_21), 0), 2) {
0 => bb19,
1 => bb2,
2 => bb16,
3 => bb7,
340282366920938463463374607431768211426 => bb32,
_ => bb31
}
}
bb31 = {
RET = _29.fld3.2;
_24.1 = core::ptr::addr_of_mut!(_29.fld0);
_26.3.0 = _8;
_27 = 654943192664360925_i64;
_29.fld4.fld3 = !Field::<i8>(Variant((*_21), 1), 0);
_29.fld3 = (_26.2.0, _22, _26.2.2);
_3 = _5;
_12.0 = -_10;
_15 = (-30_isize);
_13 = -_29.fld3.1;
_29.fld4.fld4 = 33090_u16 * 41501_u16;
_29.fld4.fld1 = _14 as f32;
_29.fld3.0 = 11501156207027043981_u64 as u32;
_19 = &_26.2;
_31 = _22 - _13;
_29.fld6 = !_27;
_26.1 = [227_u8,49_u8,131_u8,11_u8,129_u8,33_u8,200_u8];
_29.fld4.fld5 = (*_19).0 + (*_19).0;
_9 = _7;
_27 = _29.fld6 + _29.fld6;
_29.fld6 = (*_19).1 as i64;
_29.fld4.fld2 = 6_usize;
place!(Field::<i32>(Variant(_20.0.1, 1), 2)) = 1919878803_i32 & (-974297787_i32);
_17 = _4;
_34 = [_27,_27,_27,_27];
_17 = _25;
_29.fld4.fld3 = -Field::<i8>(Variant((*_21), 1), 0);
Goto(bb7)
}
bb32 = {
place!(Field::<i128>(Variant(_20.0.1, 1), 1)) = _40 * _40;
_63 = Field::<isize>(Variant((*_21), 0), 2);
_42 = -_26.2.1;
_3 = _1;
_68 = core::ptr::addr_of!(_14);
_39 = _13;
RET = _23;
_61.fld7 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_20.0.1, 1), 2)));
_27 = _59;
Goto(bb33)
}
bb33 = {
Call(_69 = dump_var(Move(_4), Move(_8), Move(_55), Move(_11)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_69 = dump_var(Move(_5), Move(_6), Move(_35), Move(_15)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_69 = dump_var(Move(_18), Move(_52), Move(_14), Move(_9)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_69 = dump_var(Move(_1), Move(_25), _70, _70), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [u16; 3],mut _2: char,mut _3: (f64,),mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char) -> f32 {
mir! {
type RET = f32;
let _11: Adt78;
let _12: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _13: u64;
let _14: i128;
let _15: [isize; 6];
let _16: &'static &'static mut *mut i32;
let _17: i8;
let _18: u8;
let _19: char;
let _20: [u32; 2];
let _21: (u32, f32, bool);
let _22: &'static &'static mut bool;
let _23: (f64,);
let _24: ((char, Adt19),);
let _25: u16;
let _26: i8;
let _27: isize;
let _28: [u32; 2];
let _29: usize;
let _30: ();
let _31: ();
{
_5 = _8;
_6 = _4;
_3.0 = (-6546182795539889119594282591429504374_i128) as f64;
_9 = _10;
_3.0 = (-100144271731073402786421890762952605397_i128) as f64;
_3.0 = 10773_i16 as f64;
_2 = _6;
_2 = _6;
_5 = _6;
_8 = _7;
RET = 16100844475690515730_u64 as f32;
_3.0 = 12208624154123877647_usize as f64;
_5 = _8;
RET = 87_i8 as f32;
_3.0 = 860145089_i32 as f64;
_14 = -17484840420151004419493419825570544902_i128;
_7 = _10;
_9 = _8;
RET = (-9223372036854775808_isize) as f32;
_8 = _7;
_7 = _6;
Goto(bb1)
}
bb1 = {
_13 = 1183634681351327888_u64 & 7456817918517505941_u64;
_4 = _10;
_18 = 232_u8;
_19 = _8;
_19 = _10;
_10 = _6;
_10 = _19;
_20 = [3297230083_u32,30394121_u32];
_14 = !(-1543592305361109116350315496997530325_i128);
_20 = [1933790801_u32,558668755_u32];
_15 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),64_isize];
_1 = [58760_u16,56317_u16,27875_u16];
_14 = 5_usize as i128;
_10 = _4;
_19 = _8;
_6 = _10;
_4 = _8;
_17 = 35_i8 * 94_i8;
_21.0 = RET as u32;
_21.0 = _14 as u32;
_6 = _4;
_9 = _10;
_8 = _19;
_15 = [84_isize,81_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-86_isize)];
_21.2 = false;
_1 = [11575_u16,45938_u16,13232_u16];
Call(_13 = core::intrinsics::transmute(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.1 = &mut _21.2;
_5 = _19;
_12.3 = &mut _12.1;
_2 = _6;
Call(RET = fn8(_15, _14, _1, _8, _5, _3.0, _19, _17, _18, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = _9;
_10 = _4;
_23.0 = -_3.0;
_4 = _8;
_19 = _6;
_19 = _7;
_10 = _2;
_10 = _8;
_23.0 = _3.0 + _3.0;
_23.0 = _3.0 * _3.0;
_14 = (-90001181232046668636563504408866988037_i128) - (-34463543532410572569951717789471508003_i128);
_4 = _6;
_9 = _4;
_9 = _4;
_23 = (_3.0,);
_1 = [17476_u16,26400_u16,38084_u16];
_23 = (_3.0,);
_24.0.0 = _6;
_6 = _9;
_18 = 210_u8 * 252_u8;
_24.0.1 = Adt19::Variant0 { fld0: true,fld1: _18,fld2: 9223372036854775807_isize,fld3: _3,fld4: 11538646273074413623_usize,fld5: 59942_u16 };
place!(Field::<isize>(Variant(_24.0.1, 0), 2)) = 20_isize;
place!(Field::<usize>(Variant(_24.0.1, 0), 4)) = !7_usize;
_27 = Field::<isize>(Variant(_24.0.1, 0), 2);
_28 = [51203711_u32,2682982196_u32];
Goto(bb4)
}
bb4 = {
Call(_30 = dump_var(Move(_7), Move(_4), Move(_1), Move(_9)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_30 = dump_var(Move(_5), Move(_14), Move(_27), Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_30 = dump_var(Move(_17), _31, _31, _31), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [isize; 6],mut _2: i128,mut _3: [u16; 3],mut _4: char,mut _5: char,mut _6: f64,mut _7: char,mut _8: i8,mut _9: u8,mut _10: i8) -> f32 {
mir! {
type RET = f32;
let _11: u8;
let _12: &'static mut [i128; 6];
let _13: isize;
let _14: (u32, f32, bool);
let _15: [i64; 4];
let _16: isize;
let _17: u16;
let _18: f32;
let _19: isize;
let _20: isize;
let _21: char;
let _22: f64;
let _23: f32;
let _24: f64;
let _25: *const u128;
let _26: f32;
let _27: f32;
let _28: (f64,);
let _29: usize;
let _30: &'static Adt19;
let _31: f64;
let _32: bool;
let _33: ();
let _34: ();
{
RET = 80171748758028312787414187587579291403_u128 as f32;
_1 = [(-81_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_11 = !_9;
_1 = [118_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-122_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
Goto(bb1)
}
bb1 = {
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-77_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_8 = -_10;
_6 = (-1430200172185074758_i64) as f64;
_9 = _2 as u8;
_14 = (538185386_u32, RET, true);
match _14.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
538185386 => bb10,
_ => bb9
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
Return()
}
bb9 = {
Return()
}
bb10 = {
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),127_isize,9223372036854775807_isize];
_4 = _5;
_15 = [1029897746641599450_i64,(-262569622111976068_i64),5151378905728183000_i64,7208579331418923512_i64];
_11 = _9 << _10;
match _14.0 {
0 => bb6,
1 => bb11,
538185386 => bb13,
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
RET = -_14.1;
_7 = _4;
_6 = 238357185998499950409658360050110440438_u128 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = (-9223372036854775808_isize);
_14.1 = -RET;
_14.1 = 22536_i16 as f32;
Goto(bb14)
}
bb14 = {
Goto(bb15)
}
bb15 = {
_1 = [_13,_13,_13,_13,_13,_13];
_5 = _4;
_8 = _10 << _14.0;
_17 = 11290_u16;
_19 = !_13;
_14.2 = true;
_15 = [(-668474040725171412_i64),4905897290191682162_i64,(-3459763909604422788_i64),2630286660313090321_i64];
_18 = RET + RET;
_20 = _13 | _13;
_10 = _8 | _8;
_16 = -_20;
_9 = _11;
_13 = RET as isize;
_13 = 90579348837758138194541686331028917425_u128 as isize;
Call(_7 = fn9(_8, _4, _19, _20, _13, _9, _20, RET, RET), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_19 = _20 << _10;
_22 = -_6;
_8 = -_10;
_4 = _7;
_23 = _18 - _14.1;
_19 = _14.2 as isize;
_14 = (2968411056_u32, _23, true);
_15 = [(-4463401502093876775_i64),5524057153170255318_i64,2692941362747051660_i64,(-8563245832889498947_i64)];
_11 = !_9;
_1 = [_16,_16,_20,_20,_13,_20];
_7 = _4;
_13 = _16 >> _10;
_6 = _22 + _22;
RET = _14.1;
_4 = _5;
_21 = _7;
_24 = -_22;
_2 = (-50815877991217637374247133144487815009_i128);
_2 = 8143918858601801021804400668975776252_i128;
_13 = !_16;
_14.2 = !true;
_9 = _11 * _11;
Call(_24 = core::intrinsics::fmaf64(_6, _6, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_14.2 = true | true;
_14.1 = -_23;
_7 = _5;
_24 = _9 as f64;
_14.0 = 3559681916_u32;
_9 = !_11;
_8 = RET as i8;
Goto(bb18)
}
bb18 = {
_13 = _17 as isize;
_8 = (-1535929484_i32) as i8;
_9 = _11;
RET = _23 - _23;
Goto(bb19)
}
bb19 = {
_14 = (498258348_u32, _18, false);
_18 = RET;
_14.0 = 3378709351_u32;
_28.0 = -_22;
_6 = _10 as f64;
_3 = [_17,_17,_17];
_6 = 20451_i16 as f64;
_28.0 = -_24;
_28 = (_24,);
_14 = (3197585464_u32, RET, true);
_4 = _21;
_2 = (-31796276291345633191820303195620612284_i128) >> _14.0;
_1 = [_20,_13,_20,_13,_19,_19];
_5 = _7;
_27 = _18 + RET;
_21 = _5;
_13 = -_16;
_5 = _7;
_29 = 1_usize * 2_usize;
_17 = _29 as u16;
_8 = 415338996284724620_u64 as i8;
_5 = _4;
_31 = _24 - _28.0;
_23 = _17 as f32;
_23 = _27 + RET;
_26 = _23 + RET;
match _14.0 {
0 => bb1,
1 => bb3,
3197585464 => bb21,
_ => bb20
}
}
bb20 = {
RET = -_14.1;
_7 = _4;
_6 = 238357185998499950409658360050110440438_u128 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_13 = (-9223372036854775808_isize);
_14.1 = -RET;
_14.1 = 22536_i16 as f32;
Goto(bb14)
}
bb21 = {
_14 = (1618200569_u32, _26, false);
RET = _26;
_13 = _16 * _16;
_14 = (1133741815_u32, _27, false);
Goto(bb22)
}
bb22 = {
Call(_33 = dump_var(Move(_17), Move(_1), Move(_9), Move(_11)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_33 = dump_var(Move(_2), Move(_7), Move(_29), Move(_13)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_33 = dump_var(Move(_10), _34, _34, _34), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i8,mut _2: char,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u8,mut _7: isize,mut _8: f32,mut _9: f32) -> char {
mir! {
type RET = char;
let _10: [u8; 6];
let _11: i32;
let _12: isize;
let _13: char;
let _14: &'static mut bool;
let _15: u64;
let _16: [u8; 6];
let _17: bool;
let _18: i8;
let _19: *mut *mut [u64; 5];
let _20: char;
let _21: [char; 2];
let _22: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _23: char;
let _24: f64;
let _25: u8;
let _26: u128;
let _27: Adt19;
let _28: [u8; 7];
let _29: i8;
let _30: i32;
let _31: u128;
let _32: u8;
let _33: [u64; 6];
let _34: i64;
let _35: u64;
let _36: &'static mut [i128; 6];
let _37: &'static mut &'static (char,);
let _38: &'static mut *mut [u64; 5];
let _39: u128;
let _40: u128;
let _41: *const [u8; 7];
let _42: f32;
let _43: i128;
let _44: [u64; 6];
let _45: (u32, f32, bool);
let _46: isize;
let _47: (&'static mut i16, *mut [u64; 5]);
let _48: f32;
let _49: [isize; 6];
let _50: char;
let _51: [bool; 7];
let _52: bool;
let _53: &'static &'static mut *mut i32;
let _54: i64;
let _55: &'static mut [i128; 6];
let _56: i64;
let _57: &'static Adt19;
let _58: *const [i16; 2];
let _59: (u32, f32, bool);
let _60: *const isize;
let _61: isize;
let _62: i16;
let _63: *mut i128;
let _64: *const &'static mut i16;
let _65: i8;
let _66: &'static mut [u8; 6];
let _67: char;
let _68: [char; 2];
let _69: [u64; 6];
let _70: [isize; 6];
let _71: i32;
let _72: i64;
let _73: Adt47;
let _74: ();
let _75: ();
{
RET = _2;
_6 = 223_u8 & 108_u8;
RET = _2;
RET = _2;
_1 = -(-9_i8);
_3 = _5 & _5;
_3 = _4 << _7;
_4 = _3 | _3;
RET = _2;
_2 = RET;
RET = _2;
_3 = _4;
_8 = -_9;
_4 = 2312470738634735129_u64 as isize;
_1 = !121_i8;
_10 = [_6,_6,_6,_6,_6,_6];
_9 = _8 + _8;
_5 = _3 | _3;
_4 = !_5;
_2 = RET;
_2 = RET;
_1 = _9 as i8;
_9 = -_8;
_2 = RET;
_11 = 1925206040_i32 | (-911431596_i32);
_2 = RET;
_1 = 8169282677289663866_u64 as i8;
Goto(bb1)
}
bb1 = {
_2 = RET;
_6 = 61_u8;
_11 = 901179414_i32;
_1 = 104_i8 >> _7;
RET = _2;
_3 = 3589_u16 as isize;
_12 = _7 + _5;
_12 = _4 ^ _5;
_3 = 104473924_u32 as isize;
_5 = -_12;
_9 = _8;
_5 = !_12;
_4 = _5;
_13 = RET;
_12 = _5;
_2 = _13;
Goto(bb2)
}
bb2 = {
_2 = RET;
_3 = _4;
_3 = _12 | _5;
_13 = RET;
_2 = RET;
_16 = [_6,_6,_6,_6,_6,_6];
_13 = RET;
_5 = _3;
_2 = _13;
_6 = 81_u8;
_8 = 6135001366975005958_usize as f32;
Goto(bb3)
}
bb3 = {
_3 = -_12;
_11 = 1930447928_i32 + 1676381720_i32;
_16 = [_6,_6,_6,_6,_6,_6];
RET = _2;
_16 = [_6,_6,_6,_6,_6,_6];
_1 = (-111123753529106112178252978340877175133_i128) as i8;
_7 = _2 as isize;
_7 = _12;
_18 = _12 as i8;
_13 = RET;
_11 = 419969012_i32 | 967828965_i32;
_8 = _9 - _9;
_5 = _4 >> _12;
_20 = _13;
match _6 {
81 => bb4,
_ => bb2
}
}
bb4 = {
_21 = [RET,_13];
_9 = _8 + _8;
match _6 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
81 => bb12,
_ => bb11
}
}
bb5 = {
_3 = -_12;
_11 = 1930447928_i32 + 1676381720_i32;
_16 = [_6,_6,_6,_6,_6,_6];
RET = _2;
_16 = [_6,_6,_6,_6,_6,_6];
_1 = (-111123753529106112178252978340877175133_i128) as i8;
_7 = _2 as isize;
_7 = _12;
_18 = _12 as i8;
_13 = RET;
_11 = 419969012_i32 | 967828965_i32;
_8 = _9 - _9;
_5 = _4 >> _12;
_20 = _13;
match _6 {
81 => bb4,
_ => bb2
}
}
bb6 = {
_2 = RET;
_3 = _4;
_3 = _12 | _5;
_13 = RET;
_2 = RET;
_16 = [_6,_6,_6,_6,_6,_6];
_13 = RET;
_5 = _3;
_2 = _13;
_6 = 81_u8;
_8 = 6135001366975005958_usize as f32;
Goto(bb3)
}
bb7 = {
_2 = RET;
_6 = 61_u8;
_11 = 901179414_i32;
_1 = 104_i8 >> _7;
RET = _2;
_3 = 3589_u16 as isize;
_12 = _7 + _5;
_12 = _4 ^ _5;
_3 = 104473924_u32 as isize;
_5 = -_12;
_9 = _8;
_5 = !_12;
_4 = _5;
_13 = RET;
_12 = _5;
_2 = _13;
Goto(bb2)
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
Return()
}
bb12 = {
_7 = 49995_u16 as isize;
_20 = RET;
_5 = -_3;
_22.2.0 = 3893618505_u32;
_3 = _12 << _12;
_13 = RET;
Goto(bb13)
}
bb13 = {
_17 = true;
_9 = (-11262_i16) as f32;
RET = _2;
_22.3 = (_2,);
match _22.2.0 {
3893618505 => bb14,
_ => bb7
}
}
bb14 = {
_9 = -_8;
_7 = (-5699932371318326893_i64) as isize;
_24 = (-23433_i16) as f64;
_25 = _6 / _6;
_11 = (-2037845348_i32);
_16 = [_25,_6,_25,_25,_6,_6];
_15 = !12531230964099561930_u64;
_1 = _18 >> _3;
_17 = false;
_14 = &mut _17;
(*_14) = _12 <= _5;
_23 = _22.3.0;
_22.2.1 = -_8;
_22.2 = (3471226283_u32, _8, (*_14));
(*_14) = _22.2.2;
_22.1 = [_6,_6,_25,_25,_25,_6,_25];
_23 = RET;
_22.3.0 = _20;
_21 = [_23,_20];
_26 = 224908065638555197137826511451873638170_u128 & 240753214878927548904463004606309940406_u128;
_22.2.0 = 2725935478_u32;
_1 = _18 | _18;
_8 = _22.2.1;
(*_14) = _22.2.2;
match _6 {
81 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
(*_14) = !_22.2.2;
_4 = 46014_u16 as isize;
_18 = _22.2.0 as i8;
(*_14) = _1 <= _1;
(*_14) = !_22.2.2;
_30 = -_11;
_28 = [_6,_6,_6,_25,_25,_25,_25];
(*_14) = _22.2.2 & _22.2.2;
_30 = _25 as i32;
_16 = [_25,_6,_25,_6,_25,_25];
(*_14) = !_22.2.2;
match _6 {
0 => bb13,
1 => bb9,
2 => bb17,
3 => bb18,
81 => bb20,
_ => bb19
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_29 = _1;
_3 = _5 ^ _12;
_31 = _26 | _26;
(*_14) = _5 == _12;
(*_14) = _5 == _12;
_9 = _1 as f32;
Call((*_14) = fn10(_7, _25, _20, _11, _20, _23, _22.1, RET, _6), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
(*_14) = _22.2.2;
(*_14) = _22.2.2;
(*_14) = _22.2.2 & _22.2.2;
(*_14) = _22.2.2;
_25 = _6 % _6;
_18 = _1 + _29;
Goto(bb22)
}
bb22 = {
_29 = !_1;
_1 = _29 << _29;
_32 = _25 - _25;
_20 = RET;
_26 = _31 & _31;
_22.2.1 = _9;
_20 = _23;
_22.1 = [_25,_32,_32,_6,_32,_32,_25];
RET = _2;
_2 = _13;
(*_14) = _5 != _3;
_1 = _22.2.0 as i8;
(*_14) = !_22.2.2;
(*_14) = _18 >= _29;
match _11 {
0 => bb4,
1 => bb15,
2 => bb23,
3 => bb24,
4 => bb25,
340282366920938463463374607429730366108 => bb27,
_ => bb26
}
}
bb23 = {
(*_14) = _22.2.2;
(*_14) = _22.2.2;
(*_14) = _22.2.2 & _22.2.2;
(*_14) = _22.2.2;
_25 = _6 % _6;
_18 = _1 + _29;
Goto(bb22)
}
bb24 = {
_3 = -_12;
_11 = 1930447928_i32 + 1676381720_i32;
_16 = [_6,_6,_6,_6,_6,_6];
RET = _2;
_16 = [_6,_6,_6,_6,_6,_6];
_1 = (-111123753529106112178252978340877175133_i128) as i8;
_7 = _2 as isize;
_7 = _12;
_18 = _12 as i8;
_13 = RET;
_11 = 419969012_i32 | 967828965_i32;
_8 = _9 - _9;
_5 = _4 >> _12;
_20 = _13;
match _6 {
81 => bb4,
_ => bb2
}
}
bb25 = {
Return()
}
bb26 = {
_7 = 49995_u16 as isize;
_20 = RET;
_5 = -_3;
_22.2.0 = 3893618505_u32;
_3 = _12 << _12;
_13 = RET;
Goto(bb13)
}
bb27 = {
_8 = 9115013781296512090_i64 as f32;
_18 = _6 as i8;
(*_14) = !_22.2.2;
(*_14) = _22.2.2 | _22.2.2;
_21 = [RET,_13];
(*_14) = _22.2.2;
_15 = 6331010142909227904_u64;
_22.2 = (1375834365_u32, _9, (*_14));
_7 = -_3;
_25 = _9 as u8;
_25 = _32 & _32;
(*_14) = _5 >= _7;
_25 = _32;
_35 = !_15;
_20 = _2;
(*_14) = _22.2.2 | _22.2.2;
(*_14) = _22.2.2 & _22.2.2;
(*_14) = _22.2.1 > _22.2.1;
_12 = _5 & _5;
(*_14) = _22.2.2;
_10 = [_25,_25,_32,_6,_32,_32];
(*_14) = !_22.2.2;
(*_14) = _22.2.0 <= _22.2.0;
_31 = _26 - _26;
(*_14) = _22.2.2;
Goto(bb28)
}
bb28 = {
(*_14) = _22.2.2 ^ _22.2.2;
_22.2.2 = (*_14) != (*_14);
(*_14) = !_22.2.2;
(*_14) = _22.2.0 <= _22.2.0;
(*_14) = _22.2.2;
(*_14) = _22.2.2 | _22.2.2;
_20 = RET;
(*_14) = _22.2.0 >= _22.2.0;
(*_14) = _12 == _3;
_30 = _11 << _29;
_20 = _2;
_10 = [_25,_32,_25,_32,_32,_25];
_4 = _7 ^ _12;
_23 = _2;
(*_14) = !_22.2.2;
_22.3 = (_20,);
_27 = Adt19::Variant1 { fld0: _29,fld1: (-13487592764581664634496882983519038903_i128),fld2: _30 };
(*_14) = _29 == _29;
_22.0 = core::ptr::addr_of_mut!(_43);
(*_14) = _22.2.2;
(*_14) = !_22.2.2;
_13 = _20;
_12 = _4 << Field::<i32>(Variant(_27, 1), 2);
_44 = [_35,_35,_15,_35,_15,_15];
_15 = !_35;
Goto(bb29)
}
bb29 = {
(*_14) = _22.2.1 >= _22.2.1;
_40 = _26;
_2 = _13;
_30 = !Field::<i32>(Variant(_27, 1), 2);
_31 = 51973_u16 as u128;
_45.0 = Field::<i32>(Variant(_27, 1), 2) as u32;
_22.2.1 = _9 - _9;
_4 = _3;
_39 = _31;
_41 = core::ptr::addr_of!(_22.1);
(*_41) = _28;
(*_14) = _22.2.2;
Goto(bb30)
}
bb30 = {
_18 = _29 * Field::<i8>(Variant(_27, 1), 0);
_40 = _26 - _39;
(*_41) = [_6,_25,_32,_32,_32,_32,_25];
_33 = [_35,_35,_35,_35,_15,_15];
_1 = _29;
(*_14) = _3 <= _3;
_42 = _22.2.1;
(*_14) = !_22.2.2;
_5 = _12 & _3;
(*_41) = [_6,_32,_25,_25,_25,_32,_32];
_12 = _5;
_19 = core::ptr::addr_of_mut!(_47.1);
Goto(bb31)
}
bb31 = {
_49 = [_4,_3,_12,_5,_12,_12];
(*_41) = _28;
_29 = _18 & _18;
_18 = _22.2.0 as i8;
(*_41) = _28;
place!(Field::<i8>(Variant(_27, 1), 0)) = _29;
_22.0 = core::ptr::addr_of_mut!(_43);
_39 = _40;
place!(Field::<i8>(Variant(_27, 1), 0)) = _1;
_11 = Field::<i32>(Variant(_27, 1), 2) & Field::<i32>(Variant(_27, 1), 2);
place!(Field::<i32>(Variant(_27, 1), 2)) = 6_usize as i32;
match _22.2.0 {
0 => bb32,
1375834365 => bb34,
_ => bb33
}
}
bb32 = {
Return()
}
bb33 = {
_29 = !_1;
_1 = _29 << _29;
_32 = _25 - _25;
_20 = RET;
_26 = _31 & _31;
_22.2.1 = _9;
_20 = _23;
_22.1 = [_25,_32,_32,_6,_32,_32,_25];
RET = _2;
_2 = _13;
(*_14) = _5 != _3;
_1 = _22.2.0 as i8;
(*_14) = !_22.2.2;
(*_14) = _18 >= _29;
match _11 {
0 => bb4,
1 => bb15,
2 => bb23,
3 => bb24,
4 => bb25,
340282366920938463463374607429730366108 => bb27,
_ => bb26
}
}
bb34 = {
_7 = _5 << _1;
(*_14) = _22.2.2;
(*_41) = _28;
(*_14) = _42 != _22.2.1;
Goto(bb35)
}
bb35 = {
(*_41) = [_32,_32,_25,_32,_6,_25,_25];
_24 = _25 as f64;
_44 = _33;
(*_14) = _22.2.2;
(*_41) = [_32,_25,_25,_32,_25,_6,_32];
_13 = _20;
_35 = _15 & _15;
(*_41) = [_25,_25,_32,_25,_25,_32,_32];
_22.2.0 = _45.0 * _45.0;
_28 = [_25,_25,_32,_25,_25,_32,_25];
(*_14) = _22.2.2;
_32 = _25 + _25;
_51 = [(*_14),(*_14),(*_14),(*_14),(*_14),(*_14),(*_14)];
_44 = [_35,_35,_35,_35,_35,_35];
_26 = !_39;
_45.0 = _22.2.0;
_28 = [_32,_25,_32,_32,_25,_32,_32];
_45.2 = (*_14) ^ (*_14);
place!(Field::<i8>(Variant(_27, 1), 0)) = _18;
_35 = _15;
RET = _2;
match _6 {
0 => bb36,
1 => bb37,
2 => bb38,
3 => bb39,
81 => bb41,
_ => bb40
}
}
bb36 = {
(*_14) = _22.2.1 >= _22.2.1;
_40 = _26;
_2 = _13;
_30 = !Field::<i32>(Variant(_27, 1), 2);
_31 = 51973_u16 as u128;
_45.0 = Field::<i32>(Variant(_27, 1), 2) as u32;
_22.2.1 = _9 - _9;
_4 = _3;
_39 = _31;
_41 = core::ptr::addr_of!(_22.1);
(*_41) = _28;
(*_14) = _22.2.2;
Goto(bb30)
}
bb37 = {
_29 = !_1;
_1 = _29 << _29;
_32 = _25 - _25;
_20 = RET;
_26 = _31 & _31;
_22.2.1 = _9;
_20 = _23;
_22.1 = [_25,_32,_32,_6,_32,_32,_25];
RET = _2;
_2 = _13;
(*_14) = _5 != _3;
_1 = _22.2.0 as i8;
(*_14) = !_22.2.2;
(*_14) = _18 >= _29;
match _11 {
0 => bb4,
1 => bb15,
2 => bb23,
3 => bb24,
4 => bb25,
340282366920938463463374607429730366108 => bb27,
_ => bb26
}
}
bb38 = {
_7 = 49995_u16 as isize;
_20 = RET;
_5 = -_3;
_22.2.0 = 3893618505_u32;
_3 = _12 << _12;
_13 = RET;
Goto(bb13)
}
bb39 = {
(*_14) = _22.2.2 ^ _22.2.2;
_22.2.2 = (*_14) != (*_14);
(*_14) = !_22.2.2;
(*_14) = _22.2.0 <= _22.2.0;
(*_14) = _22.2.2;
(*_14) = _22.2.2 | _22.2.2;
_20 = RET;
(*_14) = _22.2.0 >= _22.2.0;
(*_14) = _12 == _3;
_30 = _11 << _29;
_20 = _2;
_10 = [_25,_32,_25,_32,_32,_25];
_4 = _7 ^ _12;
_23 = _2;
(*_14) = !_22.2.2;
_22.3 = (_20,);
_27 = Adt19::Variant1 { fld0: _29,fld1: (-13487592764581664634496882983519038903_i128),fld2: _30 };
(*_14) = _29 == _29;
_22.0 = core::ptr::addr_of_mut!(_43);
(*_14) = _22.2.2;
(*_14) = !_22.2.2;
_13 = _20;
_12 = _4 << Field::<i32>(Variant(_27, 1), 2);
_44 = [_35,_35,_15,_35,_15,_15];
_15 = !_35;
Goto(bb29)
}
bb40 = {
_2 = RET;
_3 = _4;
_3 = _12 | _5;
_13 = RET;
_2 = RET;
_16 = [_6,_6,_6,_6,_6,_6];
_13 = RET;
_5 = _3;
_2 = _13;
_6 = 81_u8;
_8 = 6135001366975005958_usize as f32;
Goto(bb3)
}
bb41 = {
(*_14) = _22.2.2 & _45.2;
(*_14) = _45.2 ^ _22.2.2;
place!(Field::<i128>(Variant(_27, 1), 1)) = -151607737806262850101939090913859465901_i128;
_48 = -_42;
_34 = Field::<i128>(Variant(_27, 1), 1) as i64;
_20 = _13;
_19 = core::ptr::addr_of_mut!((*_19));
_56 = _24 as i64;
(*_14) = _11 > _11;
(*_41) = _28;
_54 = _15 as i64;
(*_41) = _28;
_22.2 = (_45.0, _42, (*_14));
_41 = core::ptr::addr_of!((*_41));
_4 = _12 + _7;
_11 = _39 as i32;
(*_14) = _12 > _7;
(*_14) = !_22.2.2;
_26 = _31 >> _3;
_59.1 = _42;
_24 = _45.0 as f64;
(*_14) = _22.2.2;
_57 = &_27;
Goto(bb42)
}
bb42 = {
_59.0 = (*_14) as u32;
place!(Field::<i32>(Variant(_27, 1), 2)) = _30 & _30;
_1 = Field::<i8>(Variant((*_57), 1), 0) >> _29;
(*_14) = _22.2.2;
(*_41) = [_25,_32,_32,_25,_25,_32,_6];
_61 = _12;
(*_41) = [_25,_32,_6,_25,_32,_32,_32];
(*_41) = [_25,_6,_32,_6,_32,_32,_25];
(*_14) = _45.2 ^ _45.2;
_48 = _22.2.1;
(*_41) = [_25,_6,_25,_25,_32,_25,_32];
(*_14) = !_22.2.2;
_45.1 = _59.1 * _48;
Goto(bb43)
}
bb43 = {
_56 = !_34;
(*_41) = [_32,_25,_25,_32,_32,_32,_6];
(*_41) = _28;
_59 = (_22.2.0, _22.2.1, (*_14));
_34 = _54;
_45.0 = _59.0 * _22.2.0;
_63 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant((*_57), 1), 1)));
(*_14) = !_22.2.2;
_56 = _54 << Field::<i8>(Variant((*_57), 1), 0);
_43 = Field::<i128>(Variant((*_57), 1), 1) & (*_63);
_39 = _26 & _26;
(*_41) = [_25,_25,_25,_32,_25,_25,_6];
(*_14) = _59.2;
_9 = -_48;
_56 = _54 * _54;
_59.1 = _9 - _9;
_63 = core::ptr::addr_of_mut!((*_63));
_10 = [_32,_32,_25,_6,_32,_32];
(*_41) = [_32,_32,_32,_25,_25,_6,_32];
(*_14) = Field::<i8>(Variant((*_57), 1), 0) > Field::<i8>(Variant((*_57), 1), 0);
_22.2.1 = -_48;
place!(Field::<i128>(Variant(_27, 1), 1)) = _43;
Goto(bb44)
}
bb44 = {
_49 = [_5,_61,_3,_12,_7,_3];
_66 = &mut _16;
_22.0 = core::ptr::addr_of_mut!(_43);
_54 = _56;
(*_14) = _59.2;
_45.1 = -_42;
place!(Field::<i128>(Variant(_27, 1), 1)) = _43 | _43;
(*_14) = !_45.2;
(*_41) = [_32,_25,_25,_25,_32,_25,_32];
match _6 {
0 => bb26,
1 => bb30,
2 => bb21,
3 => bb32,
4 => bb43,
5 => bb23,
81 => bb45,
_ => bb39
}
}
bb45 = {
_34 = _32 as i64;
_10 = (*_66);
(*_66) = [_25,_25,_32,_25,_25,_25];
_68 = _21;
place!(Field::<i8>(Variant(_27, 1), 0)) = _18 & _18;
_12 = -_5;
_44 = [_35,_35,_15,_35,_15,_15];
place!(Field::<i8>(Variant(_27, 1), 0)) = !_18;
(*_66) = [_32,_32,_6,_25,_25,_32];
_34 = -_56;
place!(Field::<i32>(Variant(_27, 1), 2)) = -_30;
(*_66) = [_25,_32,_32,_6,_6,_6];
_34 = _29 as i64;
_50 = _13;
_44 = _33;
match _6 {
0 => bb19,
1 => bb14,
2 => bb46,
3 => bb47,
4 => bb48,
81 => bb50,
_ => bb49
}
}
bb46 = {
(*_14) = _22.2.2 ^ _22.2.2;
_22.2.2 = (*_14) != (*_14);
(*_14) = !_22.2.2;
(*_14) = _22.2.0 <= _22.2.0;
(*_14) = _22.2.2;
(*_14) = _22.2.2 | _22.2.2;
_20 = RET;
(*_14) = _22.2.0 >= _22.2.0;
(*_14) = _12 == _3;
_30 = _11 << _29;
_20 = _2;
_10 = [_25,_32,_25,_32,_32,_25];
_4 = _7 ^ _12;
_23 = _2;
(*_14) = !_22.2.2;
_22.3 = (_20,);
_27 = Adt19::Variant1 { fld0: _29,fld1: (-13487592764581664634496882983519038903_i128),fld2: _30 };
(*_14) = _29 == _29;
_22.0 = core::ptr::addr_of_mut!(_43);
(*_14) = _22.2.2;
(*_14) = !_22.2.2;
_13 = _20;
_12 = _4 << Field::<i32>(Variant(_27, 1), 2);
_44 = [_35,_35,_15,_35,_15,_15];
_15 = !_35;
Goto(bb29)
}
bb47 = {
Return()
}
bb48 = {
_29 = _1;
_3 = _5 ^ _12;
_31 = _26 | _26;
(*_14) = _5 == _12;
(*_14) = _5 == _12;
_9 = _1 as f32;
Call((*_14) = fn10(_7, _25, _20, _11, _20, _23, _22.1, RET, _6), ReturnTo(bb21), UnwindUnreachable())
}
bb49 = {
_2 = RET;
_6 = 61_u8;
_11 = 901179414_i32;
_1 = 104_i8 >> _7;
RET = _2;
_3 = 3589_u16 as isize;
_12 = _7 + _5;
_12 = _4 ^ _5;
_3 = 104473924_u32 as isize;
_5 = -_12;
_9 = _8;
_5 = !_12;
_4 = _5;
_13 = RET;
_12 = _5;
_2 = _13;
Goto(bb2)
}
bb50 = {
_54 = _34;
(*_66) = [_25,_32,_32,_25,_32,_32];
_18 = 16511_i16 as i8;
(*_14) = _22.2.2 ^ _59.2;
(*_14) = !_22.2.2;
_1 = -_29;
(*_66) = [_32,_25,_32,_32,_32,_32];
_19 = core::ptr::addr_of_mut!((*_19));
_65 = _29 << _59.0;
(*_66) = [_32,_32,_6,_32,_32,_32];
_66 = &mut _10;
_62 = (-20176_i16) | 25834_i16;
_19 = core::ptr::addr_of_mut!((*_19));
_73.fld0.0 = _20;
(*_14) = _59.2 == _22.2.2;
_49 = [_5,_61,_5,_7,_4,_7];
(*_14) = !_45.2;
_22.2.1 = _42 - _42;
Goto(bb51)
}
bb51 = {
Call(_74 = dump_var(Move(_28), Move(_54), Move(_40), Move(_62)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_74 = dump_var(Move(_16), Move(_50), Move(_2), Move(_3)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_74 = dump_var(Move(_43), Move(_29), Move(_10), Move(_26)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_74 = dump_var(Move(_20), Move(_21), Move(_30), Move(_23)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_74 = dump_var(Move(_44), Move(_31), Move(_34), Move(_25)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: u8,mut _3: char,mut _4: i32,mut _5: char,mut _6: char,mut _7: [u8; 7],mut _8: char,mut _9: u8) -> bool {
mir! {
type RET = bool;
let _10: bool;
let _11: f64;
let _12: [u64; 4];
let _13: u64;
let _14: char;
let _15: isize;
let _16: isize;
let _17: [bool; 7];
let _18: bool;
let _19: char;
let _20: i8;
let _21: char;
let _22: (&'static mut i16, *mut [u64; 5]);
let _23: char;
let _24: *const *mut i32;
let _25: Adt47;
let _26: bool;
let _27: isize;
let _28: f64;
let _29: *mut i32;
let _30: isize;
let _31: Adt47;
let _32: char;
let _33: f64;
let _34: isize;
let _35: char;
let _36: Adt45;
let _37: u32;
let _38: bool;
let _39: Adt45;
let _40: *const char;
let _41: char;
let _42: [u16; 3];
let _43: isize;
let _44: bool;
let _45: Adt53;
let _46: i64;
let _47: u32;
let _48: bool;
let _49: isize;
let _50: &'static mut &'static mut bool;
let _51: *const &'static &'static mut bool;
let _52: i64;
let _53: Adt78;
let _54: u16;
let _55: u8;
let _56: f64;
let _57: [u64; 6];
let _58: bool;
let _59: *const u128;
let _60: (u8, u8);
let _61: bool;
let _62: char;
let _63: isize;
let _64: &'static &'static mut bool;
let _65: *const &'static &'static mut bool;
let _66: f32;
let _67: f32;
let _68: &'static mut (f64,);
let _69: i64;
let _70: &'static u128;
let _71: i16;
let _72: (u8, usize, i128, i128);
let _73: *const (u8, usize, i128, i128);
let _74: [u32; 2];
let _75: i8;
let _76: f64;
let _77: *const &'static mut i16;
let _78: i16;
let _79: ();
let _80: ();
{
_6 = _8;
RET = false;
_6 = _5;
_1 = 0_usize as isize;
_4 = (-1643336902_i32);
_10 = _2 != _2;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
81 => bb9,
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
RET = _10 | _10;
_2 = _9 & _9;
_3 = _8;
_7 = [_2,_2,_2,_2,_2,_2,_2];
_12 = [3705052211165824452_u64,6995577897749062757_u64,1418038115150742526_u64,6597518539902060041_u64];
_12 = [4678071437142878918_u64,8068025965951463647_u64,9397160812789062441_u64,285133234744134765_u64];
_3 = _5;
_9 = _2 + _2;
_12 = [7370893343444340783_u64,17858521196146361207_u64,7647048091039873420_u64,1530382139621788086_u64];
_11 = 254210245839143342194563410275400157775_u128 as f64;
_6 = _3;
_1 = 3906548123858604749_usize as isize;
_13 = (-3_i8) as u64;
_12 = [_13,_13,_13,_13];
_14 = _8;
_8 = _6;
_12 = [_13,_13,_13,_13];
Goto(bb10)
}
bb10 = {
RET = _10 | _10;
match _4 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb11,
340282366920938463463374607430124874554 => bb13,
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
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb14 = {
_4 = (-1576520836_i32);
RET = !_10;
_6 = _21;
_20 = 1619154881_u32 as i8;
_7 = [_2,_2,_2,_2,_9,_9,_9];
_13 = 8225472334869725944_u64 - 7801009475213556023_u64;
_8 = _5;
_17 = [RET,_10,_18,_10,RET,_18,_10];
_3 = _8;
_19 = _3;
_17 = [_10,RET,_18,_18,RET,_10,RET];
_9 = _2;
match _4 {
0 => bb8,
1 => bb5,
2 => bb12,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463463374607430191690620 => bb19,
_ => bb18
}
}
bb15 = {
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET = _10 | _10;
match _4 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb11,
340282366920938463463374607430124874554 => bb13,
_ => bb12
}
}
bb19 = {
_2 = _20 as u8;
_25.fld0.1 = Adt19::Variant1 { fld0: _20,fld1: 25202128475659999818171107348765965114_i128,fld2: _4 };
_10 = _18;
_8 = _6;
place!(Field::<i128>(Variant(_25.fld0.1, 1), 1)) = _13 as i128;
_26 = _18 == RET;
_25.fld4 = !(-29572_i16);
place!(Field::<i8>(Variant(_25.fld0.1, 1), 0)) = !_20;
place!(Field::<i8>(Variant(_25.fld0.1, 1), 0)) = _20;
_25.fld6 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_25.fld0.1, 1), 0),fld1: Field::<i128>(Variant(_25.fld0.1, 1), 1),fld2: _4 };
_11 = _13 as f64;
_8 = _14;
_25.fld0.1 = Move(_25.fld6);
_10 = RET;
_4 = Field::<i32>(Variant(_25.fld0.1, 1), 2) ^ Field::<i32>(Variant(_25.fld0.1, 1), 2);
_1 = _15 - _16;
_25.fld5.fld5 = _3 as u32;
_6 = _3;
_25.fld7 = core::ptr::addr_of_mut!(_4);
_23 = _19;
_21 = _3;
_1 = _15;
_27 = _16;
_25.fld5.fld1 = _11 as f32;
_25.fld3 = _13 | _13;
Call(_1 = fn11(_13, RET, _14, _25.fld5.fld1, _12, _20, Move(_25.fld7), _21, _8, _14, _8), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_4 = Field::<i32>(Variant(_25.fld0.1, 1), 2);
_25.fld7 = core::ptr::addr_of_mut!(_4);
_30 = _1 >> _25.fld3;
_6 = _3;
_19 = _6;
_29 = core::ptr::addr_of_mut!(_4);
_4 = Field::<i32>(Variant(_25.fld0.1, 1), 2) << _30;
_31.fld5.fld5 = _25.fld5.fld5 * _25.fld5.fld5;
_31.fld0.0 = _5;
(*_29) = !Field::<i32>(Variant(_25.fld0.1, 1), 2);
_25.fld2 = Field::<i128>(Variant(_25.fld0.1, 1), 1) as f32;
(*_29) = Field::<i32>(Variant(_25.fld0.1, 1), 2) ^ Field::<i32>(Variant(_25.fld0.1, 1), 2);
(*_29) = !Field::<i32>(Variant(_25.fld0.1, 1), 2);
_31.fld2 = _11 as f32;
(*_29) = Field::<i32>(Variant(_25.fld0.1, 1), 2) + Field::<i32>(Variant(_25.fld0.1, 1), 2);
_6 = _21;
_25.fld5.fld0 = _26;
_13 = !_25.fld3;
(*_29) = Field::<i32>(Variant(_25.fld0.1, 1), 2) - Field::<i32>(Variant(_25.fld0.1, 1), 2);
Goto(bb21)
}
bb21 = {
(*_29) = 72300931235612978054339341801999745177_u128 as i32;
_31.fld6 = Move(_25.fld0.1);
(*_29) = _25.fld2 as i32;
_6 = _19;
_22.0 = &mut _25.fld4;
(*_29) = 102759206862544635481840747073059590631_u128 as i32;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2) | Field::<i32>(Variant(_31.fld6, 1), 2);
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2) - Field::<i32>(Variant(_31.fld6, 1), 2);
_31.fld7 = core::ptr::addr_of_mut!((*_29));
_14 = _21;
(*_29) = _31.fld5.fld5 as i32;
_18 = _26 | RET;
_14 = _21;
_18 = !_26;
match Field::<i32>(Variant(_31.fld6, 1), 2) {
340282366920938463463374607430191690620 => bb22,
_ => bb11
}
}
bb22 = {
_29 = core::ptr::addr_of_mut!((*_29));
place!(Field::<i8>(Variant(_31.fld6, 1), 0)) = _20 & _20;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2);
(*_29) = _19 as i32;
_16 = _11 as isize;
_30 = _27 | _15;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2);
_21 = _23;
_26 = _13 <= _13;
_32 = _3;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2) & Field::<i32>(Variant(_31.fld6, 1), 2);
_31.fld4 = 5_usize as i16;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2);
_31.fld1 = _19;
Goto(bb23)
}
bb23 = {
_13 = 6253671737106359941_u64;
place!(Field::<i8>(Variant(_31.fld6, 1), 0)) = _20 | _20;
_31.fld5 = Adt25 { fld0: _10,fld1: _31.fld2,fld2: 0_usize,fld3: Field::<i8>(Variant(_31.fld6, 1), 0),fld4: 56350_u16,fld5: 3512888997_u32 };
_22.0 = &mut _31.fld4;
Goto(bb24)
}
bb24 = {
_38 = (*_29) == (*_29);
(*_29) = (-1802316514_i32);
_20 = _1 as i8;
_38 = _26 & _18;
(*_29) = 788752276_i32;
_34 = _1 >> (*_29);
_29 = core::ptr::addr_of_mut!((*_29));
(*_29) = 978532596_i32 & (-1358106957_i32);
_16 = -_15;
(*_29) = -401741277_i32;
_37 = 516860391_u32;
_24 = core::ptr::addr_of!(_29);
_40 = core::ptr::addr_of!(_8);
_33 = _11;
_37 = 1734361616_u32 * 528024097_u32;
_9 = _2;
(*_40) = _6;
_9 = !_2;
(*_40) = _23;
_17 = [_10,_26,_10,_18,_38,_10,_26];
(*_29) = 149789018_i32 * 1649845047_i32;
(*_40) = _19;
(*_29) = 1728472953_i32 | 1157305823_i32;
Goto(bb25)
}
bb25 = {
_2 = _9 & _9;
_14 = (*_40);
(*_24) = core::ptr::addr_of_mut!((*_29));
_28 = _33;
_42 = [38106_u16,63047_u16,33016_u16];
(*_40) = _5;
_43 = _34 >> _2;
(*_29) = (-158399831_i32) * (-107254565_i32);
_27 = -_15;
match _13 {
0 => bb20,
1 => bb17,
2 => bb26,
6253671737106359941 => bb28,
_ => bb27
}
}
bb26 = {
RET = _10 | _10;
match _4 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb11,
340282366920938463463374607430124874554 => bb13,
_ => bb12
}
}
bb27 = {
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb28 = {
_16 = _34;
_28 = -_33;
_16 = _43 + _15;
_35 = _21;
Goto(bb29)
}
bb29 = {
_18 = !_38;
(*_24) = core::ptr::addr_of_mut!((*_29));
(*_24) = core::ptr::addr_of_mut!((*_29));
_43 = !_1;
(*_24) = core::ptr::addr_of_mut!((*_29));
_14 = (*_40);
match _13 {
0 => bb26,
6253671737106359941 => bb30,
_ => bb13
}
}
bb30 = {
(*_40) = _3;
_36 = Adt45::Variant2 { fld0: Move((*_24)) };
(*_24) = core::ptr::addr_of_mut!(_4);
(*_24) = core::ptr::addr_of_mut!((*_29));
_47 = !_37;
(*_24) = core::ptr::addr_of_mut!((*_29));
_52 = (-10586_i16) as i64;
_7 = [_9,_2,_2,_2,_2,_2,_9];
(*_29) = (-142980694_i32) - 673801747_i32;
(*_24) = core::ptr::addr_of_mut!((*_29));
_37 = _47 - _47;
(*_24) = core::ptr::addr_of_mut!((*_29));
_39 = Move(_36);
(*_29) = (-1039019847_i32) + (-652131925_i32);
_10 = (*_29) <= (*_29);
_19 = _23;
_33 = _13 as f64;
(*_40) = _23;
_41 = (*_40);
match _13 {
0 => bb9,
6253671737106359941 => bb31,
_ => bb8
}
}
bb31 = {
_23 = (*_40);
(*_40) = _32;
_1 = _43 ^ _30;
_13 = 16642734149382002291_u64;
(*_24) = core::ptr::addr_of_mut!((*_29));
_19 = (*_40);
_23 = (*_40);
_11 = _33 * _33;
_21 = _41;
(*_29) = (-777903690_i32) << _20;
(*_24) = core::ptr::addr_of_mut!(_4);
_56 = _33;
Call(_2 = core::intrinsics::transmute(_38), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_17 = [_10,_18,_18,_18,_18,_18,_38];
(*_29) = 73709482_i32 | 518215724_i32;
Goto(bb33)
}
bb33 = {
(*_24) = core::ptr::addr_of_mut!((*_29));
_3 = _14;
(*_40) = _6;
(*_40) = _41;
(*_24) = core::ptr::addr_of_mut!((*_29));
_11 = -_56;
_4 = !844920063_i32;
(*_40) = _3;
(*_40) = _23;
(*_40) = _3;
(*_24) = core::ptr::addr_of_mut!((*_29));
_45 = Adt53::Variant1 { fld0: _38 };
_43 = !_1;
(*_24) = core::ptr::addr_of_mut!((*_29));
(*_29) = 1197005365_i32;
(*_29) = _27 as i32;
_21 = _8;
_20 = 95_i8 & 95_i8;
(*_24) = Move(Field::<*mut i32>(Variant(_39, 2), 0));
Goto(bb34)
}
bb34 = {
_2 = !_9;
_60.1 = 4_usize as u8;
(*_24) = core::ptr::addr_of_mut!(_4);
(*_24) = core::ptr::addr_of_mut!((*_29));
(*_29) = _52 as i32;
(*_29) = (-269002002_i32) & 1175044957_i32;
_17 = [_38,Field::<bool>(Variant(_45, 1), 0),_38,Field::<bool>(Variant(_45, 1), 0),_18,_38,_38];
_38 = Field::<bool>(Variant(_45, 1), 0) | _18;
_6 = _5;
(*_40) = _19;
_34 = _16 << _15;
(*_40) = _14;
_16 = _30 + _34;
_5 = _35;
(*_40) = _14;
_26 = !Field::<bool>(Variant(_45, 1), 0);
_57 = [_13,_13,_13,_13,_13,_13];
(*_40) = _6;
_62 = (*_40);
(*_29) = !(-1794009381_i32);
(*_29) = 44881982_i32;
_15 = -_16;
(*_29) = 8920822504781646384_usize as i32;
(*_40) = _6;
(*_24) = core::ptr::addr_of_mut!((*_29));
_63 = _1;
(*_40) = _23;
match _13 {
0 => bb22,
1 => bb26,
2 => bb15,
16642734149382002291 => bb36,
_ => bb35
}
}
bb35 = {
Return()
}
bb36 = {
(*_40) = _21;
(*_29) = -140295571_i32;
_60.1 = 157327275145024325230800742861971158786_u128 as u8;
(*_24) = core::ptr::addr_of_mut!((*_29));
_66 = _52 as f32;
_15 = _16;
_41 = (*_40);
_20 = 8_i8 << _63;
_20 = 84_i8;
_60 = (_2, _9);
(*_29) = !(-146385874_i32);
_54 = 3797_u16 ^ 1909_u16;
(*_29) = 1190960113_i32;
_24 = core::ptr::addr_of!((*_24));
_61 = _34 >= _15;
Call((*_29) = core::intrinsics::bswap((-1308972248_i32)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
_46 = _66 as i64;
(*_24) = core::ptr::addr_of_mut!((*_29));
_71 = (-30816_i16) | (-32631_i16);
(*_24) = core::ptr::addr_of_mut!((*_29));
_26 = RET ^ _38;
(*_40) = _32;
_6 = _19;
_23 = (*_40);
(*_24) = core::ptr::addr_of_mut!((*_29));
_32 = (*_40);
(*_29) = (-832500637_i32) + 1029136539_i32;
match _13 {
0 => bb35,
1 => bb38,
2 => bb39,
3 => bb40,
4 => bb41,
5 => bb42,
6 => bb43,
16642734149382002291 => bb45,
_ => bb44
}
}
bb38 = {
Return()
}
bb39 = {
Return()
}
bb40 = {
Return()
}
bb41 = {
(*_24) = core::ptr::addr_of_mut!((*_29));
_3 = _14;
(*_40) = _6;
(*_40) = _41;
(*_24) = core::ptr::addr_of_mut!((*_29));
_11 = -_56;
_4 = !844920063_i32;
(*_40) = _3;
(*_40) = _23;
(*_40) = _3;
(*_24) = core::ptr::addr_of_mut!((*_29));
_45 = Adt53::Variant1 { fld0: _38 };
_43 = !_1;
(*_24) = core::ptr::addr_of_mut!((*_29));
(*_29) = 1197005365_i32;
(*_29) = _27 as i32;
_21 = _8;
_20 = 95_i8 & 95_i8;
(*_24) = Move(Field::<*mut i32>(Variant(_39, 2), 0));
Goto(bb34)
}
bb42 = {
_17 = [_10,_18,_18,_18,_18,_18,_38];
(*_29) = 73709482_i32 | 518215724_i32;
Goto(bb33)
}
bb43 = {
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb44 = {
_2 = _9 & _9;
_14 = (*_40);
(*_24) = core::ptr::addr_of_mut!((*_29));
_28 = _33;
_42 = [38106_u16,63047_u16,33016_u16];
(*_40) = _5;
_43 = _34 >> _2;
(*_29) = (-158399831_i32) * (-107254565_i32);
_27 = -_15;
match _13 {
0 => bb20,
1 => bb17,
2 => bb26,
6253671737106359941 => bb28,
_ => bb27
}
}
bb45 = {
(*_29) = (*_40) as i32;
_48 = !Field::<bool>(Variant(_45, 1), 0);
(*_24) = core::ptr::addr_of_mut!((*_29));
_30 = _18 as isize;
(*_40) = _23;
_28 = _11;
place!(Field::<*mut i32>(Variant(_39, 2), 0)) = core::ptr::addr_of_mut!((*_29));
(*_24) = core::ptr::addr_of_mut!((*_29));
(*_24) = core::ptr::addr_of_mut!((*_29));
_44 = _38;
_46 = !_52;
_36 = Move(_39);
(*_40) = _6;
_72 = (_9, 5_usize, (-135021703417424575270395993438959618045_i128), (-104969866501380091988300091335308921641_i128));
_72.2 = _72.3 - _72.3;
match _20 {
84 => bb47,
_ => bb46
}
}
bb46 = {
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb47 = {
(*_29) = (-20504427_i32) >> _34;
(*_24) = Move(Field::<*mut i32>(Variant(_36, 2), 0));
(*_40) = _62;
_37 = !_47;
place!(Field::<*mut i32>(Variant(_36, 2), 0)) = Move((*_24));
_10 = Field::<bool>(Variant(_45, 1), 0);
_48 = _15 >= _1;
place!(Field::<bool>(Variant(_45, 1), 0)) = _10;
(*_40) = _3;
_54 = 27202_u16;
match _72.1 {
0 => bb31,
1 => bb40,
2 => bb48,
3 => bb49,
4 => bb50,
5 => bb52,
_ => bb51
}
}
bb48 = {
Return()
}
bb49 = {
(*_29) = 72300931235612978054339341801999745177_u128 as i32;
_31.fld6 = Move(_25.fld0.1);
(*_29) = _25.fld2 as i32;
_6 = _19;
_22.0 = &mut _25.fld4;
(*_29) = 102759206862544635481840747073059590631_u128 as i32;
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2) | Field::<i32>(Variant(_31.fld6, 1), 2);
(*_29) = Field::<i32>(Variant(_31.fld6, 1), 2) - Field::<i32>(Variant(_31.fld6, 1), 2);
_31.fld7 = core::ptr::addr_of_mut!((*_29));
_14 = _21;
(*_29) = _31.fld5.fld5 as i32;
_18 = _26 | RET;
_14 = _21;
_18 = !_26;
match Field::<i32>(Variant(_31.fld6, 1), 2) {
340282366920938463463374607430191690620 => bb22,
_ => bb11
}
}
bb50 = {
Return()
}
bb51 = {
_16 = -_1;
_7 = [_2,_2,_9,_9,_2,_9,_9];
_4 = 553489892_u32 as i32;
_1 = _16 & _16;
_11 = _13 as f64;
_17 = [RET,RET,RET,_10,_10,RET,RET];
_15 = _1 & _16;
_19 = _5;
_11 = 3318103266_u32 as f64;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
_13 = 7394006594457125379_u64 | 6947307517228238973_u64;
_3 = _8;
_5 = _14;
_18 = _9 == _2;
_10 = !_18;
_21 = _14;
_8 = _21;
_10 = RET;
_8 = _3;
_20 = -(-2_i8);
_16 = !_1;
_8 = _19;
_19 = _8;
Goto(bb14)
}
bb52 = {
_71 = 12733_i16;
_17 = [_38,_61,_26,_61,_10,_48,_10];
_67 = _66;
_11 = 166934937136888768306347732389687055662_u128 as f64;
(*_40) = _41;
_76 = -_56;
_75 = _20 + _20;
(*_40) = _41;
_72.2 = _52 as i128;
(*_40) = _41;
Goto(bb53)
}
bb53 = {
Call(_79 = dump_var(Move(_35), Move(_38), Move(_46), Move(_52)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_79 = dump_var(Move(_44), Move(_23), Move(_63), Move(_9)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_79 = dump_var(Move(_5), Move(_41), Move(_21), Move(_1)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_79 = dump_var(Move(_57), Move(_72), Move(_4), Move(_43)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_79 = dump_var(Move(_30), Move(_42), Move(_14), Move(_34)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_79 = dump_var(Move(_7), Move(_12), Move(_17), _80), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: u64,mut _2: bool,mut _3: char,mut _4: f32,mut _5: [u64; 4],mut _6: i8,mut _7: *mut i32,mut _8: char,mut _9: char,mut _10: char,mut _11: char) -> isize {
mir! {
type RET = isize;
let _12: char;
let _13: isize;
let _14: &'static (char,);
let _15: u8;
let _16: *mut i32;
let _17: f64;
let _18: bool;
let _19: bool;
let _20: f64;
let _21: &'static mut (f64,);
let _22: (u8, u8);
let _23: char;
let _24: char;
let _25: [i64; 4];
let _26: (char, Adt19);
let _27: &'static mut *mut [u64; 5];
let _28: i16;
let _29: [i16; 2];
let _30: bool;
let _31: &'static mut u16;
let _32: isize;
let _33: &'static mut *mut [u64; 5];
let _34: bool;
let _35: [isize; 5];
let _36: [isize; 6];
let _37: [char; 2];
let _38: &'static u128;
let _39: *const *mut i32;
let _40: f64;
let _41: isize;
let _42: char;
let _43: &'static mut *mut [u64; 5];
let _44: [isize; 5];
let _45: isize;
let _46: [i16; 2];
let _47: *const char;
let _48: &'static mut (f64,);
let _49: &'static u16;
let _50: bool;
let _51: &'static (char,);
let _52: *mut i128;
let _53: i32;
let _54: isize;
let _55: &'static mut bool;
let _56: *mut *mut [u64; 5];
let _57: isize;
let _58: &'static mut *mut i32;
let _59: i128;
let _60: &'static mut bool;
let _61: &'static mut &'static mut bool;
let _62: isize;
let _63: &'static u128;
let _64: i128;
let _65: (f64,);
let _66: u128;
let _67: *const &'static mut i16;
let _68: bool;
let _69: i32;
let _70: &'static mut u16;
let _71: isize;
let _72: *const [i16; 2];
let _73: &'static mut [i128; 6];
let _74: &'static &'static mut *mut i32;
let _75: char;
let _76: ();
let _77: ();
{
_1 = 13784325766992400486_u64;
_9 = _10;
_4 = 8461618461684777146_i64 as f32;
RET = (-96_isize) * (-9223372036854775808_isize);
_11 = _3;
_9 = _3;
_4 = (-32246_i16) as f32;
_2 = true & true;
_10 = _8;
_13 = -RET;
_3 = _10;
_13 = RET;
_5 = [_1,_1,_1,_1];
_10 = _3;
_5 = [_1,_1,_1,_1];
match _1 {
0 => bb1,
1 => bb2,
13784325766992400486 => bb4,
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
_1 = !9553040457570760746_u64;
_12 = _8;
_2 = !false;
_17 = 3702543042_u32 as f64;
_9 = _12;
_18 = _2 & _2;
_3 = _12;
_1 = !14128186585214022255_u64;
_4 = _1 as f32;
_15 = RET as u8;
_8 = _10;
_10 = _8;
Call(_20 = fn12(_3, Move(_7), _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17 = 6_usize as f64;
Goto(bb6)
}
bb6 = {
_8 = _10;
_6 = !(-56_i8);
_10 = _12;
_8 = _3;
RET = _13 & _13;
_1 = 11774804667636329903_u64 >> RET;
_9 = _12;
_17 = 5819080398948519994_usize as f64;
_8 = _3;
_15 = 1_u8 - 131_u8;
_5 = [_1,_1,_1,_1];
Goto(bb7)
}
bb7 = {
_1 = 15981740101450447193385909395615860154_u128 as u64;
_11 = _9;
RET = _13 - _13;
_3 = _9;
_17 = 32115_i16 as f64;
_19 = !_18;
_18 = !_19;
_2 = _18 & _19;
_17 = _20 - _20;
_1 = 9143273754642929080_u64;
_1 = 1358822742282764613_u64 << _6;
_11 = _12;
_9 = _12;
_5 = [_1,_1,_1,_1];
RET = !_13;
_19 = _2 ^ _2;
_9 = _11;
Goto(bb8)
}
bb8 = {
_2 = _19;
Goto(bb9)
}
bb9 = {
_12 = _11;
_3 = _8;
_15 = !249_u8;
_19 = _17 == _17;
_2 = !_19;
_10 = _9;
_23 = _11;
_3 = _12;
_15 = 163_u8;
_9 = _3;
_24 = _3;
_20 = _17 - _17;
_22.0 = _15 - _15;
_5 = [_1,_1,_1,_1];
_1 = 3526066685431637631_u64 | 5640991152198202503_u64;
_8 = _11;
_6 = 281277094109266613_usize as i8;
_23 = _24;
_22 = (_15, _15);
_8 = _24;
_3 = _12;
Goto(bb10)
}
bb10 = {
_18 = _19 | _2;
Goto(bb11)
}
bb11 = {
_8 = _3;
_8 = _24;
_24 = _11;
_9 = _8;
Call(_15 = core::intrinsics::transmute(_22.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22.1 = _22.0 ^ _22.0;
_20 = _17 - _17;
_11 = _10;
_23 = _11;
_13 = RET & RET;
_25 = [(-2353306376223801278_i64),1858548718960651824_i64,(-6765093445474950652_i64),5002797202178862650_i64];
_12 = _23;
_17 = _20;
_11 = _3;
_1 = 14374261522461065857_u64 * 7686104348534352896_u64;
_22.0 = _22.1 >> _13;
_8 = _24;
_19 = !_18;
_15 = _22.0 & _22.0;
_26.0 = _9;
_24 = _12;
_4 = 121309923_i32 as f32;
_15 = _22.1 | _22.0;
_8 = _24;
_25 = [(-2253760024548289730_i64),(-3686088028099469367_i64),5382092781745028252_i64,5573164087778028948_i64];
_11 = _3;
Goto(bb13)
}
bb13 = {
_20 = _17 - _17;
_15 = _22.0;
_9 = _23;
_4 = (-27594950400184227225777665721609000118_i128) as f32;
_2 = _19 > _18;
Call(_5 = core::intrinsics::transmute(_25), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = (_15, _15);
_12 = _11;
_11 = _24;
_20 = -_17;
_20 = _17 - _17;
_19 = _18 <= _2;
_4 = _15 as f32;
_29 = [(-20537_i16),(-21131_i16)];
_32 = _13 ^ _13;
Goto(bb15)
}
bb15 = {
_20 = _17 - _17;
_22.0 = _4 as u8;
_18 = _19 ^ _19;
_24 = _9;
_23 = _3;
_6 = 31_i8;
_18 = !_19;
_2 = !_18;
_26.1 = Adt19::Variant1 { fld0: _6,fld1: 123122331162926896909078636904442846860_i128,fld2: (-1910813175_i32) };
_20 = _17 - _17;
_29 = [7341_i16,(-32284_i16)];
_23 = _11;
_3 = _24;
_29 = [24867_i16,(-14802_i16)];
_30 = !_18;
_18 = _30;
place!(Field::<i128>(Variant(_26.1, 1), 1)) = (-277668862696292239742120391517688756_i128) >> _22.0;
place!(Field::<i128>(Variant(_26.1, 1), 1)) = 349430949_u32 as i128;
_22 = (_15, _15);
_1 = 2095930840508684090_u64 + 6794392772087292212_u64;
match _6 {
0 => bb16,
1 => bb17,
2 => bb18,
3 => bb19,
31 => bb21,
_ => bb20
}
}
bb16 = {
Return()
}
bb17 = {
_20 = _17 - _17;
_15 = _22.0;
_9 = _23;
_4 = (-27594950400184227225777665721609000118_i128) as f32;
_2 = _19 > _18;
Call(_5 = core::intrinsics::transmute(_25), ReturnTo(bb14), UnwindUnreachable())
}
bb18 = {
_1 = !9553040457570760746_u64;
_12 = _8;
_2 = !false;
_17 = 3702543042_u32 as f64;
_9 = _12;
_18 = _2 & _2;
_3 = _12;
_1 = !14128186585214022255_u64;
_4 = _1 as f32;
_15 = RET as u8;
_8 = _10;
_10 = _8;
Call(_20 = fn12(_3, Move(_7), _11), ReturnTo(bb5), UnwindUnreachable())
}
bb19 = {
_8 = _3;
_8 = _24;
_24 = _11;
_9 = _8;
Call(_15 = core::intrinsics::transmute(_22.0), ReturnTo(bb12), UnwindUnreachable())
}
bb20 = {
_12 = _11;
_3 = _8;
_15 = !249_u8;
_19 = _17 == _17;
_2 = !_19;
_10 = _9;
_23 = _11;
_3 = _12;
_15 = 163_u8;
_9 = _3;
_24 = _3;
_20 = _17 - _17;
_22.0 = _15 - _15;
_5 = [_1,_1,_1,_1];
_1 = 3526066685431637631_u64 | 5640991152198202503_u64;
_8 = _11;
_6 = 281277094109266613_usize as i8;
_23 = _24;
_22 = (_15, _15);
_8 = _24;
_3 = _12;
Goto(bb10)
}
bb21 = {
_15 = !_22.0;
_34 = !_30;
_28 = (-29365_i16) | 12882_i16;
_3 = _8;
_6 = Field::<i8>(Variant(_26.1, 1), 0) & Field::<i8>(Variant(_26.1, 1), 0);
_15 = _22.1 | _22.1;
_16 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_26.1, 1), 2)));
(*_16) = 775857625_i32;
_12 = _23;
_8 = _12;
(*_16) = 1272876798_i32 - (-1119030724_i32);
_19 = _30;
match Field::<i8>(Variant(_26.1, 1), 0) {
0 => bb22,
31 => bb24,
_ => bb23
}
}
bb22 = {
_18 = _19 | _2;
Goto(bb11)
}
bb23 = {
_17 = 6_usize as f64;
Goto(bb6)
}
bb24 = {
_10 = _11;
(*_16) = 980655156_i32;
_7 = Move(_16);
_7 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_26.1, 1), 2)));
_26.0 = _24;
(*_7) = !(-1110589645_i32);
_35 = [RET,_32,_32,_13,_32];
_3 = _11;
_26.0 = _3;
place!(Field::<i8>(Variant(_26.1, 1), 0)) = _6;
_39 = core::ptr::addr_of!(_16);
(*_39) = core::ptr::addr_of_mut!((*_7));
(*_7) = !(-2107218661_i32);
_36 = [_32,_13,_32,_13,_32,_13];
_10 = _8;
(*_7) = 34147_u16 as i32;
(*_7) = !1732603060_i32;
Goto(bb25)
}
bb25 = {
place!(Field::<i128>(Variant(_26.1, 1), 1)) = (-92620811211143110595755668912691903039_i128);
_1 = 4172582137776796394_u64 & 15870405028726774962_u64;
(*_39) = Move(_7);
(*_39) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_26.1, 1), 2)));
_22.0 = _15 | _15;
_9 = _23;
(*_16) = _28 as i32;
_5 = [_1,_1,_1,_1];
_22.1 = _24 as u8;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = _17 as i32;
_34 = _30 ^ _19;
_28 = -12488_i16;
_18 = (*_16) >= (*_16);
(*_39) = core::ptr::addr_of_mut!((*_16));
_28 = _32 as i16;
_34 = _17 != _17;
(*_16) = 1012800987_i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
RET = _32;
(*_39) = core::ptr::addr_of_mut!((*_16));
_5 = [_1,_1,_1,_1];
(*_16) = -(-1487819700_i32);
_2 = _18;
_42 = _11;
(*_16) = (-1004368329_i32) | (-737456156_i32);
Goto(bb26)
}
bb26 = {
_4 = _22.0 as f32;
_44 = _35;
_1 = 7003920276129234851_u64 | 9165787967305137889_u64;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
match Field::<i128>(Variant(_26.1, 1), 1) {
0 => bb24,
1 => bb27,
247661555709795352867618938519076308417 => bb29,
_ => bb28
}
}
bb27 = {
_18 = _19 | _2;
Goto(bb11)
}
bb28 = {
_20 = _17 - _17;
_22.0 = _4 as u8;
_18 = _19 ^ _19;
_24 = _9;
_23 = _3;
_6 = 31_i8;
_18 = !_19;
_2 = !_18;
_26.1 = Adt19::Variant1 { fld0: _6,fld1: 123122331162926896909078636904442846860_i128,fld2: (-1910813175_i32) };
_20 = _17 - _17;
_29 = [7341_i16,(-32284_i16)];
_23 = _11;
_3 = _24;
_29 = [24867_i16,(-14802_i16)];
_30 = !_18;
_18 = _30;
place!(Field::<i128>(Variant(_26.1, 1), 1)) = (-277668862696292239742120391517688756_i128) >> _22.0;
place!(Field::<i128>(Variant(_26.1, 1), 1)) = 349430949_u32 as i128;
_22 = (_15, _15);
_1 = 2095930840508684090_u64 + 6794392772087292212_u64;
match _6 {
0 => bb16,
1 => bb17,
2 => bb18,
3 => bb19,
31 => bb21,
_ => bb20
}
}
bb29 = {
_47 = core::ptr::addr_of!(_23);
_37 = [(*_47),(*_47)];
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _11;
Goto(bb30)
}
bb30 = {
(*_47) = _24;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _26.0;
_5 = [_1,_1,_1,_1];
_36 = [_32,RET,RET,RET,_32,_32];
(*_39) = core::ptr::addr_of_mut!((*_16));
_45 = RET * _13;
(*_47) = _9;
(*_47) = _42;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = 185198909_i32 + (-1854896359_i32);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
_16 = core::ptr::addr_of_mut!((*_16));
_22.0 = _15 + _22.1;
(*_16) = -255113975_i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
_22.1 = !_15;
(*_16) = 790211801211956459_i64 as i32;
_10 = _23;
_28 = (-22898_i16);
_34 = _15 == _22.1;
Goto(bb31)
}
bb31 = {
(*_47) = _10;
_20 = _17 * _17;
_6 = Field::<i8>(Variant(_26.1, 1), 0);
_40 = _20 + _17;
_35 = [RET,_45,_13,RET,_32];
_13 = !RET;
(*_47) = _3;
_20 = _40 - _40;
(*_47) = _11;
(*_39) = core::ptr::addr_of_mut!((*_16));
_5 = [_1,_1,_1,_1];
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = 1_usize as i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
_22.0 = _22.1 - _22.1;
(*_47) = _26.0;
(*_47) = _10;
Goto(bb32)
}
bb32 = {
_45 = !_13;
(*_47) = _11;
(*_47) = _9;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
_30 = _34 <= _18;
(*_39) = core::ptr::addr_of_mut!((*_16));
_37 = [(*_47),_12];
_13 = _32 | _32;
_2 = _20 == _20;
(*_16) = 1009174046_i32 | (-2086604317_i32);
(*_16) = (-2127387884_i32) & 1163582820_i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
place!(Field::<i8>(Variant(_26.1, 1), 0)) = 3160258118840333516_i64 as i8;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!(_53);
(*_47) = _9;
_29 = [_28,_28];
_50 = !_19;
_8 = (*_47);
(*_16) = Field::<i32>(Variant(_26.1, 1), 2);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = Field::<i32>(Variant(_26.1, 1), 2);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _3;
match _28 {
340282366920938463463374607431768188558 => bb34,
_ => bb33
}
}
bb33 = {
Return()
}
bb34 = {
(*_16) = Field::<i32>(Variant(_26.1, 1), 2) - Field::<i32>(Variant(_26.1, 1), 2);
(*_39) = core::ptr::addr_of_mut!((*_16));
_42 = (*_47);
(*_47) = _3;
_15 = 284379703651713420501843924085688785639_u128 as u8;
(*_16) = _4 as i32;
(*_47) = _26.0;
_55 = &mut _19;
(*_39) = core::ptr::addr_of_mut!((*_16));
_28 = (-30316_i16) + 23941_i16;
_25 = [1670174612800780431_i64,8946108431731675161_i64,(-345453314969290613_i64),949784547791372263_i64];
(*_16) = 1657252890_u32 as i32;
(*_47) = _12;
(*_47) = _3;
(*_16) = Field::<i32>(Variant(_26.1, 1), 2) * Field::<i32>(Variant(_26.1, 1), 2);
_1 = 14101139215085254408_u64 | 15281640289244225399_u64;
_28 = 6728_i16 | 12776_i16;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_55) = _22.0 == _22.0;
(*_55) = _50 & _18;
Goto(bb35)
}
bb35 = {
(*_47) = _26.0;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = !Field::<i32>(Variant(_26.1, 1), 2);
place!(Field::<i8>(Variant(_26.1, 1), 0)) = _13 as i8;
(*_16) = Field::<i32>(Variant(_26.1, 1), 2) * Field::<i32>(Variant(_26.1, 1), 2);
_18 = !(*_55);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_55) = _18 & _34;
_44 = _35;
_28 = 27435_i16 - 9747_i16;
(*_16) = Field::<i32>(Variant(_26.1, 1), 2);
_3 = _26.0;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _24;
(*_16) = _13 as i32;
(*_55) = _2 | _30;
(*_47) = _42;
(*_47) = _42;
match Field::<i128>(Variant(_26.1, 1), 1) {
0 => bb6,
247661555709795352867618938519076308417 => bb36,
_ => bb19
}
}
bb36 = {
(*_55) = (*_16) == (*_16);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _42;
(*_47) = _9;
_54 = _13 | _45;
(*_47) = _12;
_24 = (*_47);
(*_39) = core::ptr::addr_of_mut!(_53);
_65.0 = -_20;
(*_55) = _50 >= _50;
_7 = core::ptr::addr_of_mut!((*_16));
Goto(bb37)
}
bb37 = {
_26.1 = Adt19::Variant1 { fld0: _6,fld1: 24337487259472013000846829746026620495_i128,fld2: (*_7) };
place!(Field::<i128>(Variant(_26.1, 1), 1)) = (-103168419273258328074413882188888570068_i128);
(*_47) = _26.0;
(*_7) = (-1310013412171478175_i64) as i32;
_62 = -_45;
(*_47) = _8;
_52 = core::ptr::addr_of_mut!(_64);
_20 = -_17;
(*_55) = !_2;
(*_52) = _1 as i128;
(*_52) = -Field::<i128>(Variant(_26.1, 1), 1);
(*_47) = _42;
(*_16) = Field::<i32>(Variant(_26.1, 1), 2) | Field::<i32>(Variant(_26.1, 1), 2);
_28 = (-19267_i16) | 20393_i16;
_60 = &mut (*_55);
_47 = core::ptr::addr_of!((*_47));
_26.1 = Adt19::Variant0 { fld0: _50,fld1: _22.1,fld2: _13,fld3: _65,fld4: 7_usize,fld5: 37167_u16 };
_3 = (*_47);
_50 = (*_60) == Field::<bool>(Variant(_26.1, 0), 0);
_2 = (*_16) <= (*_16);
(*_47) = _12;
place!(Field::<bool>(Variant(_26.1, 0), 0)) = _6 > _6;
(*_16) = 1499502293_i32;
(*_52) = (-136850431407752178082722028310487269808_i128) * 9191053120764516303088650017634815419_i128;
(*_39) = core::ptr::addr_of_mut!((*_16));
_47 = core::ptr::addr_of!((*_47));
match (*_16) {
0 => bb27,
1 => bb38,
2 => bb39,
3 => bb40,
4 => bb41,
5 => bb42,
1499502293 => bb44,
_ => bb43
}
}
bb38 = {
Return()
}
bb39 = {
(*_47) = _24;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_47) = _26.0;
_5 = [_1,_1,_1,_1];
_36 = [_32,RET,RET,RET,_32,_32];
(*_39) = core::ptr::addr_of_mut!((*_16));
_45 = RET * _13;
(*_47) = _9;
(*_47) = _42;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = 185198909_i32 + (-1854896359_i32);
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_39) = core::ptr::addr_of_mut!((*_16));
_16 = core::ptr::addr_of_mut!((*_16));
_22.0 = _15 + _22.1;
(*_16) = -255113975_i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
_22.1 = !_15;
(*_16) = 790211801211956459_i64 as i32;
_10 = _23;
_28 = (-22898_i16);
_34 = _15 == _22.1;
Goto(bb31)
}
bb40 = {
place!(Field::<i128>(Variant(_26.1, 1), 1)) = (-92620811211143110595755668912691903039_i128);
_1 = 4172582137776796394_u64 & 15870405028726774962_u64;
(*_39) = Move(_7);
(*_39) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_26.1, 1), 2)));
_22.0 = _15 | _15;
_9 = _23;
(*_16) = _28 as i32;
_5 = [_1,_1,_1,_1];
_22.1 = _24 as u8;
(*_39) = core::ptr::addr_of_mut!((*_16));
(*_16) = _17 as i32;
_34 = _30 ^ _19;
_28 = -12488_i16;
_18 = (*_16) >= (*_16);
(*_39) = core::ptr::addr_of_mut!((*_16));
_28 = _32 as i16;
_34 = _17 != _17;
(*_16) = 1012800987_i32;
(*_39) = core::ptr::addr_of_mut!((*_16));
RET = _32;
(*_39) = core::ptr::addr_of_mut!((*_16));
_5 = [_1,_1,_1,_1];
(*_16) = -(-1487819700_i32);
_2 = _18;
_42 = _11;
(*_16) = (-1004368329_i32) | (-737456156_i32);
Goto(bb26)
}
bb41 = {
_8 = _3;
_8 = _24;
_24 = _11;
_9 = _8;
Call(_15 = core::intrinsics::transmute(_22.0), ReturnTo(bb12), UnwindUnreachable())
}
bb42 = {
_22 = (_15, _15);
_12 = _11;
_11 = _24;
_20 = -_17;
_20 = _17 - _17;
_19 = _18 <= _2;
_4 = _15 as f32;
_29 = [(-20537_i16),(-21131_i16)];
_32 = _13 ^ _13;
Goto(bb15)
}
bb43 = {
_22.1 = _22.0 ^ _22.0;
_20 = _17 - _17;
_11 = _10;
_23 = _11;
_13 = RET & RET;
_25 = [(-2353306376223801278_i64),1858548718960651824_i64,(-6765093445474950652_i64),5002797202178862650_i64];
_12 = _23;
_17 = _20;
_11 = _3;
_1 = 14374261522461065857_u64 * 7686104348534352896_u64;
_22.0 = _22.1 >> _13;
_8 = _24;
_19 = !_18;
_15 = _22.0 & _22.0;
_26.0 = _9;
_24 = _12;
_4 = 121309923_i32 as f32;
_15 = _22.1 | _22.0;
_8 = _24;
_25 = [(-2253760024548289730_i64),(-3686088028099469367_i64),5382092781745028252_i64,5573164087778028948_i64];
_11 = _3;
Goto(bb13)
}
bb44 = {
(*_16) = 413831371_i32 << _62;
(*_16) = (-91837974_i32) * (-923815532_i32);
place!(Field::<(f64,)>(Variant(_26.1, 0), 3)).0 = _40;
Goto(bb45)
}
bb45 = {
_28 = !(-5531_i16);
(*_47) = _11;
_34 = (*_60) != (*_60);
_57 = RET;
_69 = (*_16) ^ (*_16);
_50 = (*_60) ^ (*_60);
(*_16) = _69 + _69;
(*_39) = core::ptr::addr_of_mut!((*_16));
_30 = _2 == (*_60);
(*_47) = _12;
(*_47) = _8;
(*_16) = _69;
_23 = _24;
(*_60) = _2 > _34;
Goto(bb46)
}
bb46 = {
(*_16) = _69 >> _22.0;
_20 = _40 * Field::<(f64,)>(Variant(_26.1, 0), 3).0;
(*_47) = _3;
_18 = (*_60) | (*_60);
(*_16) = _69;
_72 = core::ptr::addr_of!(_46);
(*_47) = _10;
(*_47) = _10;
_45 = _13 & _32;
(*_72) = [_28,_28];
_20 = 65326_u16 as f64;
Goto(bb47)
}
bb47 = {
(*_47) = _24;
_61 = &mut _60;
_8 = (*_47);
_10 = (*_47);
(*_16) = _69 << RET;
(*_61) = &mut _34;
(*_72) = [_28,_28];
(*_72) = [_28,_28];
_75 = (*_47);
Goto(bb48)
}
bb48 = {
(*_47) = _42;
_50 = _30;
Goto(bb49)
}
bb49 = {
_6 = 61_i8;
_71 = Field::<u8>(Variant(_26.1, 0), 1) as isize;
(*_39) = Move(_7);
(*_47) = _42;
_59 = _1 as i128;
_8 = (*_47);
(*_47) = _8;
(*_39) = core::ptr::addr_of_mut!(_69);
_39 = core::ptr::addr_of!((*_39));
(*_72) = [_28,_28];
(*_72) = _29;
Goto(bb50)
}
bb50 = {
Call(_76 = dump_var(Move(_18), Move(_3), Move(_23), Move(_22)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_76 = dump_var(Move(_64), Move(_35), Move(_2), Move(_45)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_76 = dump_var(Move(_30), Move(_13), Move(_12), Move(_37)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_76 = dump_var(Move(_11), Move(_59), Move(_6), Move(_46)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_76 = dump_var(Move(_75), Move(_28), Move(_15), Move(_42)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: char,mut _2: *mut i32,mut _3: char) -> f64 {
mir! {
type RET = f64;
let _4: f64;
let _5: f32;
let _6: &'static mut *mut i32;
let _7: Adt46;
let _8: &'static mut &'static (char,);
let _9: bool;
let _10: i128;
let _11: &'static (char,);
let _12: i32;
let _13: i32;
let _14: isize;
let _15: usize;
let _16: f64;
let _17: [u64; 4];
let _18: *const [u8; 7];
let _19: &'static &'static mut bool;
let _20: bool;
let _21: i128;
let _22: bool;
let _23: u16;
let _24: u128;
let _25: &'static mut [u8; 6];
let _26: [isize; 6];
let _27: u32;
let _28: ();
let _29: ();
{
_1 = _3;
_4 = (-9223372036854775808_isize) as f64;
RET = _4 * _4;
_3 = _1;
_1 = _3;
RET = _4 + _4;
_5 = 1106812574323092843_u64 as f32;
RET = _4 - _4;
_3 = _1;
_3 = _1;
_5 = 285589847236221703572943808675909398949_u128 as f32;
_4 = 29729_u16 as f64;
_1 = _3;
_5 = 9811789790701495461_u64 as f32;
Call(_3 = fn13(Move(_2), RET, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _1;
RET = -_4;
_4 = 2850567159_u32 as f64;
_1 = _3;
_4 = RET - RET;
_3 = _1;
_4 = RET;
_7.fld4.fld1 = _5 + _5;
_5 = _7.fld4.fld1 + _7.fld4.fld1;
_7.fld3 = (2049557324_u32, _7.fld4.fld1, true);
match _7.fld3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
2049557324 => bb7,
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
_7.fld2 = !3_usize;
_7.fld3.0 = 1809711355_u32 & 508488771_u32;
_7.fld6 = 8747516513038983740_i64 - (-5721905093685189311_i64);
_7.fld4 = Adt25 { fld0: _7.fld3.2,fld1: _5,fld2: _7.fld2,fld3: (-77_i8),fld4: 49675_u16,fld5: _7.fld3.0 };
_7.fld5 = Adt19::Variant1 { fld0: _7.fld4.fld3,fld1: (-24372577862817962900690951061226208611_i128),fld2: (-356453052_i32) };
_7.fld3.2 = _7.fld4.fld0;
_7.fld4.fld4 = 27715_u16 ^ 36705_u16;
_7.fld3 = (_7.fld4.fld5, _7.fld4.fld1, _7.fld4.fld0);
_7.fld4.fld3 = -Field::<i8>(Variant(_7.fld5, 1), 0);
Call(_7.fld6 = core::intrinsics::bswap((-4754122164602743135_i64)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7.fld4.fld2 = !_7.fld2;
_7.fld4.fld3 = Field::<i8>(Variant(_7.fld5, 1), 0) & Field::<i8>(Variant(_7.fld5, 1), 0);
Goto(bb9)
}
bb9 = {
_2 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
(*_2) = 69154795_i32 | (-1673777706_i32);
RET = -_4;
_7.fld4 = Adt25 { fld0: _7.fld3.2,fld1: _7.fld3.1,fld2: _7.fld2,fld3: Field::<i8>(Variant(_7.fld5, 1), 0),fld4: 16633_u16,fld5: _7.fld3.0 };
_7.fld4.fld2 = _7.fld2 - _7.fld2;
_6 = &mut _2;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_9 = !_7.fld4.fld0;
(*_6) = core::ptr::addr_of_mut!(_12);
_3 = _1;
(*_6) = core::ptr::addr_of_mut!(_13);
_9 = !_7.fld3.2;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
(*_6) = core::ptr::addr_of_mut!(_12);
_7.fld5 = Adt19::Variant1 { fld0: _7.fld4.fld3,fld1: 91608587060987994836698053338792478991_i128,fld2: 1870384936_i32 };
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(_12);
_7.fld3.2 = !_7.fld4.fld0;
_7.fld4 = Adt25 { fld0: _7.fld3.2,fld1: _5,fld2: _7.fld2,fld3: Field::<i8>(Variant(_7.fld5, 1), 0),fld4: 45311_u16,fld5: _7.fld3.0 };
_15 = !_7.fld2;
_5 = _7.fld4.fld4 as f32;
_14 = -(-9223372036854775808_isize);
(*_6) = core::ptr::addr_of_mut!(_12);
match _7.fld4.fld3 {
0 => bb5,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463463374607431768211379 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_3 = _1;
RET = -_4;
_4 = 2850567159_u32 as f64;
_1 = _3;
_4 = RET - RET;
_3 = _1;
_4 = RET;
_7.fld4.fld1 = _5 + _5;
_5 = _7.fld4.fld1 + _7.fld4.fld1;
_7.fld3 = (2049557324_u32, _7.fld4.fld1, true);
match _7.fld3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
2049557324 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_13 = (-1802062575_i32) | 184264572_i32;
_10 = 91091469413636712233981941902992813066_i128 * 15917518803790210661462132811372881667_i128;
place!(Field::<i32>(Variant(_7.fld5, 1), 2)) = _13 + _13;
_7.fld4.fld4 = 31476_u16 + 46908_u16;
_7.fld0 = [12951046337941693192_u64,14527585805378944603_u64,6988275924201358678_u64,12872198935956950376_u64,11450202062033336684_u64];
(*_6) = core::ptr::addr_of_mut!(_13);
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(_13);
(*_6) = core::ptr::addr_of_mut!(_13);
_7.fld3.0 = _7.fld6 as u32;
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld2 = _7.fld6 as usize;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld3.2 = _9 != _9;
(*_6) = core::ptr::addr_of_mut!(_13);
_16 = _7.fld4.fld4 as f64;
_9 = _7.fld3.2 & _7.fld3.2;
_4 = -_16;
(*_6) = core::ptr::addr_of_mut!(_12);
_12 = Field::<i32>(Variant(_7.fld5, 1), 2) >> _10;
place!(Field::<i128>(Variant(_7.fld5, 1), 1)) = -_10;
_7.fld2 = _15 >> _7.fld4.fld3;
_7.fld3.2 = _5 > _5;
place!(Field::<i128>(Variant(_7.fld5, 1), 1)) = _10 * _10;
_7.fld3.1 = _5 + _5;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld0 = [1519463382025946088_u64,2769245329679921067_u64,12381962297395197844_u64,3010338069707417935_u64,2779648888125189724_u64];
match _7.fld4.fld3 {
0 => bb11,
1 => bb3,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
340282366920938463463374607431768211379 => bb21,
_ => bb20
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_3 = _1;
RET = -_4;
_4 = 2850567159_u32 as f64;
_1 = _3;
_4 = RET - RET;
_3 = _1;
_4 = RET;
_7.fld4.fld1 = _5 + _5;
_5 = _7.fld4.fld1 + _7.fld4.fld1;
_7.fld3 = (2049557324_u32, _7.fld4.fld1, true);
match _7.fld3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
2049557324 => bb7,
_ => bb6
}
}
bb18 = {
Return()
}
bb19 = {
_2 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
(*_2) = 69154795_i32 | (-1673777706_i32);
RET = -_4;
_7.fld4 = Adt25 { fld0: _7.fld3.2,fld1: _7.fld3.1,fld2: _7.fld2,fld3: Field::<i8>(Variant(_7.fld5, 1), 0),fld4: 16633_u16,fld5: _7.fld3.0 };
_7.fld4.fld2 = _7.fld2 - _7.fld2;
_6 = &mut _2;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_9 = !_7.fld4.fld0;
(*_6) = core::ptr::addr_of_mut!(_12);
_3 = _1;
(*_6) = core::ptr::addr_of_mut!(_13);
_9 = !_7.fld3.2;
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
(*_6) = core::ptr::addr_of_mut!(_12);
_7.fld5 = Adt19::Variant1 { fld0: _7.fld4.fld3,fld1: 91608587060987994836698053338792478991_i128,fld2: 1870384936_i32 };
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(_12);
_7.fld3.2 = !_7.fld4.fld0;
_7.fld4 = Adt25 { fld0: _7.fld3.2,fld1: _5,fld2: _7.fld2,fld3: Field::<i8>(Variant(_7.fld5, 1), 0),fld4: 45311_u16,fld5: _7.fld3.0 };
_15 = !_7.fld2;
_5 = _7.fld4.fld4 as f32;
_14 = -(-9223372036854775808_isize);
(*_6) = core::ptr::addr_of_mut!(_12);
match _7.fld4.fld3 {
0 => bb5,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463463374607431768211379 => bb14,
_ => bb13
}
}
bb20 = {
Return()
}
bb21 = {
_7.fld3 = (_7.fld4.fld5, _5, _9);
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld4.fld0 = Field::<i128>(Variant(_7.fld5, 1), 1) != Field::<i128>(Variant(_7.fld5, 1), 1);
_7.fld3 = (_7.fld4.fld5, _5, _7.fld4.fld0);
Goto(bb22)
}
bb22 = {
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld0 = [230027765531254855_u64,15853399407615539435_u64,18354059809181724882_u64,7238952327221321853_u64,1437145786916352596_u64];
_20 = _9 != _7.fld3.2;
_5 = -_7.fld3.1;
_7.fld7 = Field::<i128>(Variant(_7.fld5, 1), 1) + Field::<i128>(Variant(_7.fld5, 1), 1);
(*_6) = core::ptr::addr_of_mut!(_13);
(*_6) = core::ptr::addr_of_mut!(_12);
_7.fld1 = Adt27::Variant1 { fld0: _7.fld7,fld1: _1,fld2: _7.fld3.1,fld3: Field::<i8>(Variant(_7.fld5, 1), 0),fld4: _7.fld3,fld5: _7.fld0 };
_7.fld6 = 1744063547204054433_i64;
place!(Field::<[u64; 5]>(Variant(_7.fld1, 1), 5)) = [14501503290926667206_u64,17499824384505128391_u64,2942380884913331263_u64,15579991028262613032_u64,10241859199041062200_u64];
_17 = [8098375621219636982_u64,8422593820164781919_u64,2433763716511265183_u64,5432810645755303334_u64];
Goto(bb23)
}
bb23 = {
(*_6) = core::ptr::addr_of_mut!(_12);
_5 = Field::<f32>(Variant(_7.fld1, 1), 2) * _7.fld4.fld1;
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld4.fld3 = -Field::<i8>(Variant(_7.fld1, 1), 3);
RET = _4 * _16;
place!(Field::<f32>(Variant(_7.fld1, 1), 2)) = Field::<(u32, f32, bool)>(Variant(_7.fld1, 1), 4).1 + _5;
_7.fld4.fld5 = _10 as u32;
_5 = -_7.fld3.1;
(*_6) = core::ptr::addr_of_mut!(_13);
_14 = (-9223372036854775808_isize);
Goto(bb24)
}
bb24 = {
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
(*_6) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_7.fld5, 1), 2)));
_7.fld7 = _14 as i128;
_20 = !_7.fld3.2;
(*_6) = core::ptr::addr_of_mut!(_12);
(*_6) = core::ptr::addr_of_mut!(_13);
_7.fld6 = (-8585586726596288627_i64) | (-5746508952238963886_i64);
_27 = _7.fld4.fld5;
place!(Field::<i8>(Variant(_7.fld1, 1), 3)) = 244_u8 as i8;
place!(Field::<i128>(Variant(_7.fld5, 1), 1)) = 10846684201462118457_u64 as i128;
_14 = Field::<f32>(Variant(_7.fld1, 1), 2) as isize;
_16 = Field::<f32>(Variant(_7.fld1, 1), 2) as f64;
(*_6) = core::ptr::addr_of_mut!(_13);
Goto(bb25)
}
bb25 = {
_7.fld2 = _15 & _7.fld4.fld2;
_7.fld3.0 = Field::<(u32, f32, bool)>(Variant(_7.fld1, 1), 4).0 ^ _7.fld4.fld5;
RET = -_16;
_7.fld3 = (_7.fld4.fld5, Field::<(u32, f32, bool)>(Variant(_7.fld1, 1), 4).1, _9);
_26 = [_14,_14,_14,_14,_14,_14];
_13 = Field::<i32>(Variant(_7.fld5, 1), 2) + _12;
_13 = !_12;
_20 = _9;
_22 = _9 | _7.fld3.2;
_17 = [5218519426996619439_u64,10870572116302553748_u64,2527550878656312918_u64,17402077821302484915_u64];
_10 = Field::<i128>(Variant(_7.fld1, 1), 0) - Field::<i128>(Variant(_7.fld1, 1), 0);
_21 = _10 + Field::<i128>(Variant(_7.fld1, 1), 0);
Goto(bb26)
}
bb26 = {
Call(_28 = dump_var(Move(_26), Move(_15), Move(_27), Move(_21)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_28 = dump_var(Move(_9), Move(_20), Move(_13), _29), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut i32,mut _2: f64,mut _3: char,mut _4: char,mut _5: char) -> char {
mir! {
type RET = char;
let _6: *mut [u64; 5];
let _7: isize;
let _8: &'static &'static mut bool;
let _9: i16;
let _10: *const &'static &'static mut bool;
let _11: *mut *mut [u64; 5];
let _12: *mut [u64; 5];
let _13: u64;
let _14: u8;
let _15: isize;
let _16: bool;
let _17: [u32; 2];
let _18: [u8; 6];
let _19: i128;
let _20: [bool; 7];
let _21: u32;
let _22: Adt27;
let _23: isize;
let _24: &'static mut *mut [u64; 5];
let _25: isize;
let _26: f32;
let _27: &'static u128;
let _28: &'static u16;
let _29: f64;
let _30: u8;
let _31: isize;
let _32: &'static mut [u8; 6];
let _33: &'static mut i16;
let _34: isize;
let _35: [bool; 7];
let _36: (char,);
let _37: u16;
let _38: &'static mut i16;
let _39: f64;
let _40: i128;
let _41: Adt53;
let _42: char;
let _43: f32;
let _44: char;
let _45: [u32; 2];
let _46: isize;
let _47: u8;
let _48: [u32; 2];
let _49: f32;
let _50: *const (u8, usize, i128, i128);
let _51: isize;
let _52: *mut u32;
let _53: (char, Adt19);
let _54: u64;
let _55: isize;
let _56: &'static u128;
let _57: u32;
let _58: [u16; 3];
let _59: *mut i32;
let _60: Adt19;
let _61: &'static &'static mut bool;
let _62: [u64; 5];
let _63: f32;
let _64: [u16; 3];
let _65: i8;
let _66: i64;
let _67: bool;
let _68: f32;
let _69: f32;
let _70: u64;
let _71: *mut [u64; 5];
let _72: *const u128;
let _73: ();
let _74: ();
{
RET = _4;
_2 = (-48067210417552768097917891743373151424_i128) as f64;
RET = _3;
RET = _4;
_3 = _5;
RET = _3;
_3 = RET;
_5 = _3;
_7 = 12015565020639245679_u64 as isize;
_2 = (-975182671_i32) as f64;
_2 = 1444743239_u32 as f64;
RET = _3;
_2 = 16293_i16 as f64;
_3 = _4;
_3 = _5;
RET = _5;
_5 = RET;
_5 = _3;
RET = _4;
Goto(bb1)
}
bb1 = {
_4 = _3;
_5 = RET;
_4 = RET;
_2 = 2_usize as f64;
_5 = RET;
_9 = (-27491_i16) << _7;
_5 = RET;
_9 = 48006_u16 as i16;
_9 = 138_i16 ^ 29739_i16;
_7 = (-9223372036854775808_isize) >> _9;
_2 = 165049539195381673582426191144712255611_u128 as f64;
_5 = RET;
_5 = _3;
_3 = _4;
_5 = _3;
_2 = 1_usize as f64;
_9 = 10493_i16;
_5 = _3;
_7 = -(-9223372036854775808_isize);
_9 = 6923_u16 as i16;
_11 = core::ptr::addr_of_mut!(_6);
_9 = (-112_i16);
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211344 => bb10,
_ => bb9
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
Return()
}
bb9 = {
Return()
}
bb10 = {
_3 = _4;
_5 = _3;
_2 = 2499087007_u32 as f64;
Goto(bb11)
}
bb11 = {
_4 = _3;
_9 = (-51_i16) * (-23973_i16);
_4 = _3;
_11 = core::ptr::addr_of_mut!((*_11));
_14 = 6_u8;
match _14 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_3 = _4;
_11 = core::ptr::addr_of_mut!((*_11));
_15 = _7 >> _7;
_11 = core::ptr::addr_of_mut!((*_11));
_4 = _5;
_13 = 17244795692340534301_u64 & 8724797211594303458_u64;
_3 = _4;
_13 = 8275536966453283818_u64 ^ 2874984573217066525_u64;
_4 = _3;
RET = _5;
_5 = _3;
_11 = core::ptr::addr_of_mut!((*_11));
_15 = _7;
_7 = !_15;
_16 = false;
_3 = _5;
_7 = _15 ^ _15;
_16 = true ^ false;
_2 = 940049523_u32 as f64;
_3 = _4;
_13 = 12835829669372764586_u64 | 6728903608353098574_u64;
_7 = _2 as isize;
_4 = _3;
RET = _4;
_13 = 64035198999023681214047362690642337098_i128 as u64;
_11 = core::ptr::addr_of_mut!((*_11));
_14 = 187_u8 << _13;
Goto(bb14)
}
bb14 = {
_5 = _3;
_17 = [1282938407_u32,1576964507_u32];
_17 = [27085345_u32,2979814099_u32];
_7 = _15;
_17 = [2952939697_u32,761718806_u32];
_7 = 4_usize as isize;
_2 = 7404040093101025156_i64 as f64;
Goto(bb15)
}
bb15 = {
_11 = core::ptr::addr_of_mut!((*_11));
_3 = RET;
_18 = [_14,_14,_14,_14,_14,_14];
_7 = _15 >> _9;
Goto(bb16)
}
bb16 = {
_3 = RET;
_9 = -(-9753_i16);
_9 = -(-26902_i16);
_4 = _3;
_4 = _5;
_16 = false;
_4 = _5;
_17 = [1816834531_u32,3017117617_u32];
_2 = 2731115849_u32 as f64;
_4 = RET;
_2 = (-1576898798_i32) as f64;
_19 = (-62039099226970507211230433967784340645_i128) + 27394111643774807023866364429311357607_i128;
_5 = _4;
_3 = RET;
_7 = _15 - _15;
_2 = _13 as f64;
_18 = [_14,_14,_14,_14,_14,_14];
_20 = [_16,_16,_16,_16,_16,_16,_16];
_4 = RET;
_18 = [_14,_14,_14,_14,_14,_14];
_7 = -_15;
_15 = _13 as isize;
_16 = _14 < _14;
_13 = 14556762088567380795_u64 * 10041922475377391684_u64;
_2 = 6_usize as f64;
_21 = !1976578315_u32;
Goto(bb17)
}
bb17 = {
_15 = -_7;
Goto(bb18)
}
bb18 = {
_5 = _4;
_11 = core::ptr::addr_of_mut!((*_11));
_23 = _15 * _7;
Goto(bb19)
}
bb19 = {
_2 = 14690_u16 as f64;
_21 = (-5143942056654334577_i64) as u32;
_7 = _23 | _23;
_9 = (-10387_i16) >> _21;
_7 = -_23;
_15 = _7 - _7;
RET = _4;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_23 = 120_i8 as isize;
_11 = core::ptr::addr_of_mut!((*_11));
_15 = -_23;
_3 = RET;
_15 = _23 ^ _7;
_5 = _3;
_2 = _9 as f64;
_17 = [_21,_21];
Goto(bb20)
}
bb20 = {
RET = _5;
_14 = 198_u8 << _21;
_20 = [_16,_16,_16,_16,_16,_16,_16];
_17 = [_21,_21];
_19 = (-17966368987264368950515012725816871695_i128);
_11 = core::ptr::addr_of_mut!((*_11));
_13 = 16658431190814849014_u64 - 3455181326830810128_u64;
RET = _5;
_7 = _23;
_5 = _4;
_5 = RET;
_18 = [_14,_14,_14,_14,_14,_14];
_11 = core::ptr::addr_of_mut!((*_11));
_25 = 103_i8 as isize;
_21 = 1485268559_u32;
_18 = [_14,_14,_14,_14,_14,_14];
_20 = [_16,_16,_16,_16,_16,_16,_16];
_11 = core::ptr::addr_of_mut!((*_11));
_26 = 44824_u16 as f32;
_25 = _15 >> _15;
_18 = [_14,_14,_14,_14,_14,_14];
_20 = [_16,_16,_16,_16,_16,_16,_16];
_25 = -_23;
_4 = _5;
_13 = (-1100430476_i32) as u64;
_3 = _5;
Goto(bb21)
}
bb21 = {
_23 = _15 << _25;
_25 = !_23;
RET = _5;
RET = _4;
_11 = core::ptr::addr_of_mut!((*_11));
_14 = _2 as u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
RET = _3;
_15 = !_25;
_2 = _21 as f64;
_23 = _25;
_29 = _2;
_14 = 130_u8 & 9_u8;
RET = _4;
_4 = _3;
_30 = _14;
_13 = 3083384211107263713_u64 ^ 9174077504406550550_u64;
_30 = !_14;
_17 = [_21,_21];
_9 = 2636_i16;
_5 = _4;
_25 = -_23;
_3 = _5;
_11 = core::ptr::addr_of_mut!((*_11));
_3 = RET;
_17 = [_21,_21];
_4 = _5;
match _19 {
0 => bb6,
1 => bb2,
322315997933674094512859594705951339761 => bb22,
_ => bb9
}
}
bb22 = {
_19 = (-134260734989956506291754983413772344348_i128) * 83403216780695274823039590553248878624_i128;
_2 = _29 - _29;
_19 = (-62408155199023418432259173280151576105_i128) | 29628063055950932410309718993740069287_i128;
_4 = _3;
_29 = -_2;
_33 = &mut _9;
(*_33) = !(-6249_i16);
_30 = (-114_i8) as u8;
Goto(bb23)
}
bb23 = {
(*_33) = -(-10674_i16);
(*_33) = (-18879_i16) << _15;
(*_33) = 17346_i16 >> _23;
_7 = _23;
(*_33) = _26 as i16;
(*_33) = (-7234_i16);
(*_33) = (-7987_i16) + (-14601_i16);
_11 = core::ptr::addr_of_mut!((*_11));
(*_33) = (-18542_i16) | 19794_i16;
_19 = (-117056906889526709259335603164077412703_i128);
RET = _4;
_18 = [_30,_14,_14,_14,_14,_14];
Goto(bb24)
}
bb24 = {
_26 = _19 as f32;
_29 = -_2;
(*_33) = !(-11912_i16);
(*_33) = 31481_i16 * (-27819_i16);
(*_33) = _21 as i16;
_34 = _23;
_3 = RET;
(*_33) = (-18378_i16) ^ (-8925_i16);
_15 = 6324953043463737041_usize as isize;
_39 = _29 - _2;
Goto(bb25)
}
bb25 = {
_29 = _39;
(*_33) = 26000_u16 as i16;
_36.0 = _3;
_2 = _39 - _39;
_32 = &mut _18;
(*_33) = (-19711_i16) | (-14339_i16);
_39 = -_2;
_2 = -_29;
_11 = core::ptr::addr_of_mut!((*_11));
(*_33) = 31987_i16;
(*_32) = [_14,_14,_14,_14,_14,_30];
(*_32) = [_30,_30,_30,_14,_14,_30];
_17 = [_21,_21];
_31 = _34 & _34;
(*_32) = [_14,_14,_30,_30,_14,_14];
(*_32) = [_30,_14,_14,_14,_14,_30];
_2 = -_39;
(*_33) = 25376_i16;
(*_33) = 6_usize as i16;
RET = _3;
match _21 {
0 => bb22,
1 => bb17,
1485268559 => bb27,
_ => bb26
}
}
bb26 = {
_15 = -_7;
Goto(bb18)
}
bb27 = {
(*_33) = 32407_i16 - (-22943_i16);
(*_33) = -(-32033_i16);
(*_33) = (-19594_i16);
_11 = core::ptr::addr_of_mut!(_6);
_3 = _36.0;
(*_33) = _21 as i16;
_39 = _2 - _29;
_37 = 49297_u16 << _34;
(*_32) = [_14,_14,_14,_14,_14,_14];
(*_33) = (-12841_i16);
_38 = &mut (*_33);
_43 = _26;
(*_32) = [_14,_30,_30,_30,_30,_14];
_36 = (_4,);
(*_38) = 17572_i16 | (-2801_i16);
(*_38) = 23267_i16 * 2263_i16;
_7 = _25 << _31;
_42 = _5;
(*_38) = _39 as i16;
(*_32) = [_14,_30,_14,_30,_14,_30];
_11 = core::ptr::addr_of_mut!((*_11));
_37 = 55964_u16;
_40 = _19 - _19;
Goto(bb28)
}
bb28 = {
_33 = Move(_38);
_16 = true ^ true;
(*_32) = [_30,_30,_14,_14,_30,_14];
_46 = !_7;
_44 = _3;
(*_32) = [_14,_30,_14,_14,_30,_30];
_25 = _31;
_46 = _23 + _25;
match _19 {
0 => bb1,
1 => bb15,
2 => bb8,
3 => bb24,
4 => bb11,
5 => bb27,
6 => bb7,
223225460031411754204039004267690798753 => bb30,
_ => bb29
}
}
bb29 = {
_4 = _3;
_9 = (-51_i16) * (-23973_i16);
_4 = _3;
_11 = core::ptr::addr_of_mut!((*_11));
_14 = 6_u8;
match _14 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb13,
_ => bb12
}
}
bb30 = {
_21 = 39875522_u32;
_36.0 = _42;
_45 = [_21,_21];
_47 = _37 as u8;
_44 = RET;
(*_32) = [_30,_47,_47,_30,_47,_47];
(*_32) = [_14,_14,_14,_14,_14,_47];
_20 = [_16,_16,_16,_16,_16,_16,_16];
(*_32) = [_30,_47,_14,_47,_30,_14];
(*_32) = [_14,_47,_14,_14,_14,_47];
match _21 {
0 => bb5,
1 => bb25,
2 => bb8,
3 => bb31,
4 => bb32,
5 => bb33,
6 => bb34,
39875522 => bb36,
_ => bb35
}
}
bb31 = {
_23 = _15 << _25;
_25 = !_23;
RET = _5;
RET = _4;
_11 = core::ptr::addr_of_mut!((*_11));
_14 = _2 as u8;
_20 = [_16,_16,_16,_16,_16,_16,_16];
RET = _3;
_15 = !_25;
_2 = _21 as f64;
_23 = _25;
_29 = _2;
_14 = 130_u8 & 9_u8;
RET = _4;
_4 = _3;
_30 = _14;
_13 = 3083384211107263713_u64 ^ 9174077504406550550_u64;
_30 = !_14;
_17 = [_21,_21];
_9 = 2636_i16;
_5 = _4;
_25 = -_23;
_3 = _5;
_11 = core::ptr::addr_of_mut!((*_11));
_3 = RET;
_17 = [_21,_21];
_4 = _5;
match _19 {
0 => bb6,
1 => bb2,
322315997933674094512859594705951339761 => bb22,
_ => bb9
}
}
bb32 = {
_3 = RET;
_9 = -(-9753_i16);
_9 = -(-26902_i16);
_4 = _3;
_4 = _5;
_16 = false;
_4 = _5;
_17 = [1816834531_u32,3017117617_u32];
_2 = 2731115849_u32 as f64;
_4 = RET;
_2 = (-1576898798_i32) as f64;
_19 = (-62039099226970507211230433967784340645_i128) + 27394111643774807023866364429311357607_i128;
_5 = _4;
_3 = RET;
_7 = _15 - _15;
_2 = _13 as f64;
_18 = [_14,_14,_14,_14,_14,_14];
_20 = [_16,_16,_16,_16,_16,_16,_16];
_4 = RET;
_18 = [_14,_14,_14,_14,_14,_14];
_7 = -_15;
_15 = _13 as isize;
_16 = _14 < _14;
_13 = 14556762088567380795_u64 * 10041922475377391684_u64;
_2 = 6_usize as f64;
_21 = !1976578315_u32;
Goto(bb17)
}
bb33 = {
Return()
}
bb34 = {
_15 = -_7;
Goto(bb18)
}
bb35 = {
Return()
}
bb36 = {
_40 = _19 & _19;
_31 = _46;
_48 = _17;
match _19 {
0 => bb9,
1 => bb20,
2 => bb3,
3 => bb14,
4 => bb30,
5 => bb37,
6 => bb38,
223225460031411754204039004267690798753 => bb40,
_ => bb39
}
}
bb37 = {
Return()
}
bb38 = {
_29 = _39;
(*_33) = 26000_u16 as i16;
_36.0 = _3;
_2 = _39 - _39;
_32 = &mut _18;
(*_33) = (-19711_i16) | (-14339_i16);
_39 = -_2;
_2 = -_29;
_11 = core::ptr::addr_of_mut!((*_11));
(*_33) = 31987_i16;
(*_32) = [_14,_14,_14,_14,_14,_30];
(*_32) = [_30,_30,_30,_14,_14,_30];
_17 = [_21,_21];
_31 = _34 & _34;
(*_32) = [_14,_14,_30,_30,_14,_14];
(*_32) = [_30,_14,_14,_14,_14,_30];
_2 = -_39;
(*_33) = 25376_i16;
(*_33) = 6_usize as i16;
RET = _3;
match _21 {
0 => bb22,
1 => bb17,
1485268559 => bb27,
_ => bb26
}
}
bb39 = {
Return()
}
bb40 = {
(*_32) = [_30,_47,_47,_30,_14,_14];
_2 = -_39;
Goto(bb41)
}
bb41 = {
_17 = _45;
(*_32) = [_14,_14,_14,_30,_14,_14];
_15 = !_25;
_26 = -_43;
_36.0 = RET;
RET = _44;
(*_32) = [_14,_14,_30,_30,_30,_14];
_21 = 1693277584_u32;
(*_32) = [_47,_14,_14,_30,_30,_30];
(*_32) = [_14,_47,_47,_30,_14,_14];
_26 = -_43;
(*_32) = [_14,_14,_14,_30,_47,_14];
_2 = -_39;
(*_32) = [_47,_14,_14,_47,_30,_30];
(*_32) = [_14,_30,_47,_30,_30,_47];
_30 = _47 >> _21;
_51 = _7 << _25;
(*_32) = [_47,_47,_47,_14,_14,_30];
Goto(bb42)
}
bb42 = {
(*_32) = [_30,_14,_30,_30,_30,_14];
_49 = _43 + _26;
_42 = _4;
_16 = true;
(*_32) = [_14,_14,_47,_47,_30,_14];
_36 = (_3,);
_51 = _15;
_11 = core::ptr::addr_of_mut!((*_11));
_15 = 290972239_i32 as isize;
_42 = _36.0;
_13 = 1625509025_i32 as u64;
_53.0 = _44;
_45 = [_21,_21];
_47 = _14;
_36.0 = _42;
_52 = core::ptr::addr_of_mut!(_57);
(*_32) = [_47,_14,_30,_14,_14,_30];
_15 = _7 ^ _7;
_53.0 = _3;
Call(_53.0 = fn14(Move(_11), Move(_1), Move(_32)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_5 = _42;
_16 = !true;
_23 = -_31;
(*_52) = !_21;
_13 = (-30766_i16) as u64;
(*_52) = _21 >> _23;
(*_52) = _21 - _21;
(*_52) = _21 * _21;
_5 = _36.0;
_35 = _20;
_14 = _30 & _30;
_34 = _31 | _51;
_39 = _2;
(*_52) = _40 as u32;
_6 = core::ptr::addr_of_mut!(_62);
_58 = [_37,_37,_37];
(*_6) = [_13,_13,_13,_13,_13];
_7 = -_15;
(*_52) = _16 as u32;
(*_52) = _21 % _21;
(*_52) = _21 * _21;
(*_52) = _16 as u32;
(*_52) = _16 as u32;
_24 = &mut _6;
(*_24) = core::ptr::addr_of_mut!(_62);
Goto(bb44)
}
bb44 = {
(*_24) = core::ptr::addr_of_mut!(_62);
_4 = _3;
_15 = _7 - _25;
(*_24) = core::ptr::addr_of_mut!(_62);
_58 = [_37,_37,_37];
(*_24) = core::ptr::addr_of_mut!(_62);
(*_52) = (-17567_i16) as u32;
_28 = &_37;
(*_52) = _21 >> _25;
(*_52) = !_21;
_55 = _15;
_51 = _55 - _23;
(*_52) = _21 >> _34;
_7 = _55 * _23;
_63 = -_43;
_15 = _34 >> (*_52);
_36.0 = RET;
_41 = Adt53::Variant1 { fld0: _16 };
_4 = _53.0;
_2 = _39 * _39;
_47 = !_14;
_25 = !_7;
Goto(bb45)
}
bb45 = {
_53.1 = Adt19::Variant1 { fld0: (-42_i8),fld1: _19,fld2: (-1622583686_i32) };
_11 = core::ptr::addr_of_mut!((*_24));
_42 = _53.0;
_59 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_53.1, 1), 2)));
(*_59) = (-1532029389_i32) * (-1106040469_i32);
(*_59) = -1343041030_i32;
(*_59) = (-358956690_i32) >> (*_52);
_49 = _63;
_35 = _20;
(*_52) = _21 + _21;
(*_24) = core::ptr::addr_of_mut!(_62);
(*_24) = core::ptr::addr_of_mut!(_62);
(*_52) = !_21;
(*_52) = _21;
_16 = (*_59) > (*_59);
(*_52) = _21;
_49 = _63 + _26;
(*_59) = (-163812576_i32) * 1626265332_i32;
_11 = core::ptr::addr_of_mut!((*_24));
_55 = _34;
(*_11) = core::ptr::addr_of_mut!(_62);
(*_24) = core::ptr::addr_of_mut!(_62);
_11 = core::ptr::addr_of_mut!((*_24));
(*_59) = _13 as i32;
(*_59) = (-643096489_i32) * (-63072524_i32);
_34 = -_55;
_60 = Adt19::Variant1 { fld0: (-32_i8),fld1: _19,fld2: (*_59) };
(*_52) = _21 - _21;
(*_11) = core::ptr::addr_of_mut!(_62);
(*_11) = core::ptr::addr_of_mut!(_62);
match (*_28) {
0 => bb46,
55964 => bb48,
_ => bb47
}
}
bb46 = {
_11 = core::ptr::addr_of_mut!((*_11));
_3 = RET;
_18 = [_14,_14,_14,_14,_14,_14];
_7 = _15 >> _9;
Goto(bb16)
}
bb47 = {
Return()
}
bb48 = {
(*_24) = core::ptr::addr_of_mut!(_62);
_21 = !(*_52);
_34 = !_51;
(*_24) = core::ptr::addr_of_mut!(_62);
_25 = _23 | _34;
place!(Field::<i8>(Variant(_60, 1), 0)) = _16 as i8;
match (*_28) {
55964 => bb50,
_ => bb49
}
}
bb49 = {
(*_32) = [_30,_47,_47,_30,_14,_14];
_2 = -_39;
Goto(bb41)
}
bb50 = {
place!(Field::<bool>(Variant(_41, 1), 0)) = _16 | _16;
_20 = [Field::<bool>(Variant(_41, 1), 0),Field::<bool>(Variant(_41, 1), 0),_16,Field::<bool>(Variant(_41, 1), 0),_16,Field::<bool>(Variant(_41, 1), 0),Field::<bool>(Variant(_41, 1), 0)];
(*_52) = _21;
_37 = 3592_u16 >> _55;
_26 = _49 - _63;
(*_24) = core::ptr::addr_of_mut!(_62);
_70 = !_13;
_4 = _36.0;
(*_52) = _21;
(*_52) = !_21;
_19 = _40 & _40;
_66 = 1882663088522429542_i64;
_25 = (*_52) as isize;
_53.1 = Move(_60);
_39 = _13 as f64;
(*_52) = _21;
(*_24) = core::ptr::addr_of_mut!(_62);
_14 = Field::<bool>(Variant(_41, 1), 0) as u8;
_12 = core::ptr::addr_of_mut!(_62);
(*_12) = [_70,_70,_70,_13,_13];
Goto(bb51)
}
bb51 = {
Call(_73 = dump_var(Move(_3), Move(_58), Move(_44), Move(_47)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_73 = dump_var(Move(_35), Move(_5), Move(_36), Move(_31)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_73 = dump_var(Move(_57), Move(_13), Move(_20), Move(_45)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_73 = dump_var(Move(_62), Move(_37), Move(_4), Move(_70)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_73 = dump_var(Move(_55), Move(_15), _74, _74), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *mut *mut [u64; 5],mut _2: *mut i32,mut _3: &'static mut [u8; 6]) -> char {
mir! {
type RET = char;
let _4: [isize; 5];
let _5: [u8; 7];
let _6: f64;
let _7: Adt45;
let _8: f32;
let _9: (u8, usize, i128, i128);
let _10: &'static u16;
let _11: &'static mut u16;
let _12: *mut u32;
let _13: (f64,);
let _14: char;
let _15: *const [i16; 2];
let _16: [u64; 5];
let _17: &'static mut bool;
let _18: i64;
let _19: i128;
let _20: bool;
let _21: &'static mut &'static (char,);
let _22: &'static &'static mut *mut i32;
let _23: i128;
let _24: u64;
let _25: Adt78;
let _26: [isize; 5];
let _27: *const char;
let _28: &'static u128;
let _29: u64;
let _30: i8;
let _31: [u128; 6];
let _32: &'static mut [u8; 6];
let _33: *mut [u64; 5];
let _34: (char, *const u128, (char, Adt19), *mut i32);
let _35: i32;
let _36: (&'static mut i16, *mut [u64; 5]);
let _37: i64;
let _38: &'static mut i16;
let _39: isize;
let _40: (char, *const u128, (char, Adt19), *mut i32);
let _41: (char,);
let _42: usize;
let _43: &'static mut i16;
let _44: usize;
let _45: f32;
let _46: [u8; 7];
let _47: i32;
let _48: *const char;
let _49: *mut i32;
let _50: i16;
let _51: f32;
let _52: char;
let _53: ();
let _54: ();
{
RET = '\u{441d5}';
RET = '\u{bb9ae}';
RET = '\u{6d48e}';
RET = '\u{40e44}';
RET = '\u{24427}';
RET = '\u{53117}';
RET = '\u{ae54c}';
Goto(bb1)
}
bb1 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb2 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb3 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb4 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb5 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_13.0 = _9.1 as f64;
_9 = (5_u8, 2187159840021405312_usize, (-29745813153974901195976161192992118073_i128), 60976539350942234769981485463769775802_i128);
_6 = _13.0 - _13.0;
_6 = -_13.0;
_9.0 = !77_u8;
_14 = RET;
_9.0 = 246_u8 << _9.2;
_14 = RET;
_9.2 = _9.3 << _9.3;
_9 = (209_u8, 1727656792486702589_usize, (-110774587813530345724565126990168588189_i128), 119418597944198306854281302812452673124_i128);
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
_6 = 13746_i16 as f64;
_8 = 454430713_u32 as f32;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_14 = RET;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-72_isize)];
_4 = [9223372036854775807_isize,(-89_isize),(-20_isize),86_isize,(-9223372036854775808_isize)];
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
match _9.0 {
0 => bb6,
1 => bb8,
2 => bb9,
209 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb10 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb11 = {
_13 = (_6,);
match _9.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
1727656792486702589 => bb18,
_ => bb17
}
}
bb12 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb13 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_13.0 = _9.1 as f64;
_9 = (5_u8, 2187159840021405312_usize, (-29745813153974901195976161192992118073_i128), 60976539350942234769981485463769775802_i128);
_6 = _13.0 - _13.0;
_6 = -_13.0;
_9.0 = !77_u8;
_14 = RET;
_9.0 = 246_u8 << _9.2;
_14 = RET;
_9.2 = _9.3 << _9.3;
_9 = (209_u8, 1727656792486702589_usize, (-110774587813530345724565126990168588189_i128), 119418597944198306854281302812452673124_i128);
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
_6 = 13746_i16 as f64;
_8 = 454430713_u32 as f32;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_14 = RET;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-72_isize)];
_4 = [9223372036854775807_isize,(-89_isize),(-20_isize),86_isize,(-9223372036854775808_isize)];
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
match _9.0 {
0 => bb6,
1 => bb8,
2 => bb9,
209 => bb11,
_ => bb10
}
}
bb16 = {
Return()
}
bb17 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb18 = {
_8 = _6 as f32;
_18 = 39043_u16 as i64;
_9.0 = !147_u8;
_7 = Adt45::Variant2 { fld0: Move(_2) };
_9 = (154_u8, 7_usize, (-101051011184877166652596622426161855301_i128), (-36964845456006583449338136694219145812_i128));
Goto(bb19)
}
bb19 = {
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
_9.0 = 107_u8 & 213_u8;
_9.0 = 9223372036854775807_isize as u8;
Call(_4 = fn15(Move(_2), Move(_1), _8, _9), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_13 = (_6,);
RET = _14;
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-95_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_14 = RET;
_19 = _9.2 ^ _9.2;
_9.2 = _19 + _9.3;
_20 = _9.1 > _9.1;
_9.0 = 92_u8;
_9.3 = _9.1 as i128;
_9.1 = 12909139983490065912_usize | 2_usize;
_9.2 = (-21320_i16) as i128;
_5 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_9.3 = _19;
_17 = &mut _20;
_5 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
(*_17) = true | true;
match _9.0 {
0 => bb21,
1 => bb22,
2 => bb23,
3 => bb24,
4 => bb25,
5 => bb26,
6 => bb27,
92 => bb29,
_ => bb28
}
}
bb21 = {
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
_9.0 = 107_u8 & 213_u8;
_9.0 = 9223372036854775807_isize as u8;
Call(_4 = fn15(Move(_2), Move(_1), _8, _9), ReturnTo(bb20), UnwindUnreachable())
}
bb22 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb26 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb27 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb28 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb29 = {
RET = _14;
(*_17) = false | true;
(*_17) = _9.1 == _9.1;
(*_17) = false;
_5 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_14 = RET;
_6 = 54383_u16 as f64;
_13 = (_6,);
(*_17) = !true;
_9.0 = 215_u8 ^ 184_u8;
_24 = !1610623850122951098_u64;
(*_17) = !false;
_6 = _13.0 * _13.0;
(*_17) = true;
_13 = (_6,);
(*_17) = _9.3 >= _9.2;
_26 = [(-9223372036854775808_isize),36_isize,9223372036854775807_isize,60_isize,9223372036854775807_isize];
Goto(bb30)
}
bb30 = {
_26 = [60_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _14;
(*_17) = _9.0 == _9.0;
_23 = 49362_u16 as i128;
_31 = [218527510784676959494813057930830804274_u128,246223252593900919580145444061958585413_u128,138017933288057453840729359635151912122_u128,127968606971080619708438704562324631046_u128,89622746233675323333522390550217571722_u128,303221199798220449952175473436193490026_u128];
(*_17) = _19 < _19;
(*_17) = _19 > _19;
_9.1 = 0_usize - 3_usize;
_24 = 5616447758659087638_u64;
_9.3 = _19 - _19;
(*_17) = _9.3 <= _9.3;
_26 = [9223372036854775807_isize,91_isize,(-9223372036854775808_isize),39_isize,(-117_isize)];
(*_17) = false;
(*_17) = !false;
_34.0 = RET;
_29 = !_24;
RET = _34.0;
(*_17) = true;
_29 = _24;
match _24 {
0 => bb10,
1 => bb7,
2 => bb17,
5616447758659087638 => bb32,
_ => bb31
}
}
bb31 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb32 = {
(*_17) = true;
_29 = _24;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_35);
_34.2.1 = Adt19::Variant1 { fld0: (-107_i8),fld1: _19,fld2: 1962485582_i32 };
_34.3 = core::ptr::addr_of_mut!(_35);
_9.3 = -Field::<i128>(Variant(_34.2.1, 1), 1);
RET = _14;
(*_17) = Field::<i128>(Variant(_34.2.1, 1), 1) == _9.2;
_34.2.0 = _34.0;
_9.0 = _9.1 as u8;
(*_17) = _9.3 > _9.2;
_33 = core::ptr::addr_of_mut!(_16);
(*_33) = [_24,_29,_29,_29,_24];
place!(Field::<i8>(Variant(_34.2.1, 1), 0)) = -123_i8;
(*_17) = !false;
_34.2.1 = Adt19::Variant0 { fld0: (*_17),fld1: _9.0,fld2: (-5_isize),fld3: _13,fld4: _9.1,fld5: 899_u16 };
match _24 {
0 => bb17,
5616447758659087638 => bb33,
_ => bb28
}
}
bb33 = {
(*_33) = [_24,_24,_24,_29,_24];
_13.0 = _6 * Field::<(f64,)>(Variant(_34.2.1, 0), 3).0;
_23 = _9.3 - _19;
_27 = core::ptr::addr_of!(_14);
(*_27) = _34.2.0;
(*_27) = RET;
_19 = _9.3 * _9.3;
place!(Field::<u16>(Variant(_34.2.1, 0), 5)) = 2847311380_u32 as u16;
(*_33) = [_29,_29,_29,_24,_24];
(*_27) = _34.2.0;
_9 = (Field::<u8>(Variant(_34.2.1, 0), 1), Field::<usize>(Variant(_34.2.1, 0), 4), _23, _19);
Goto(bb34)
}
bb34 = {
(*_17) = RET > (*_27);
(*_33) = [_24,_24,_24,_29,_29];
_13.0 = _6;
(*_27) = _34.2.0;
(*_33) = [_24,_29,_24,_24,_29];
_5 = [Field::<u8>(Variant(_34.2.1, 0), 1),Field::<u8>(Variant(_34.2.1, 0), 1),_9.0,_9.0,Field::<u8>(Variant(_34.2.1, 0), 1),_9.0,Field::<u8>(Variant(_34.2.1, 0), 1)];
_34.3 = core::ptr::addr_of_mut!(_35);
(*_33) = [_29,_29,_24,_29,_29];
(*_33) = [_29,_24,_24,_24,_29];
_34.2.1 = Adt19::Variant0 { fld0: (*_17),fld1: _9.0,fld2: 9223372036854775807_isize,fld3: _13,fld4: _9.1,fld5: 52847_u16 };
(*_27) = _34.2.0;
_16 = [_24,_24,_29,_29,_24];
_9.0 = Field::<u8>(Variant(_34.2.1, 0), 1) * Field::<u8>(Variant(_34.2.1, 0), 1);
(*_17) = Field::<bool>(Variant(_34.2.1, 0), 0);
_33 = core::ptr::addr_of_mut!((*_33));
(*_27) = RET;
(*_33) = [_24,_24,_29,_29,_24];
match _29 {
0 => bb26,
1 => bb16,
2 => bb15,
3 => bb22,
4 => bb11,
5 => bb10,
5616447758659087638 => bb35,
_ => bb23
}
}
bb35 = {
(*_27) = RET;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_35);
_36.1 = core::ptr::addr_of_mut!((*_33));
_8 = 331112662591763757092134027322188438630_u128 as f32;
(*_27) = _34.2.0;
_40.3 = core::ptr::addr_of_mut!(_35);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0;
_9.3 = Field::<usize>(Variant(_34.2.1, 0), 4) as i128;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_35);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0;
(*_17) = Field::<bool>(Variant(_34.2.1, 0), 0) & Field::<bool>(Variant(_34.2.1, 0), 0);
(*_27) = _34.0;
(*_17) = Field::<(f64,)>(Variant(_34.2.1, 0), 3).0 >= _13.0;
(*_33) = [_29,_24,_24,_24,_29];
_1 = core::ptr::addr_of_mut!(_33);
_9.0 = !Field::<u8>(Variant(_34.2.1, 0), 1);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0 << _9.2;
_35 = !(-1259608149_i32);
(*_33) = [_29,_29,_24,_24,_29];
_41.0 = (*_27);
_27 = core::ptr::addr_of!((*_27));
(*_33) = [_29,_29,_24,_29,_24];
(*_1) = core::ptr::addr_of_mut!((*_33));
(*_33) = [_24,_24,_24,_29,_29];
_41 = ((*_27),);
Goto(bb36)
}
bb36 = {
(*_17) = _19 >= _9.2;
(*_1) = Move(_36.1);
(*_17) = !Field::<bool>(Variant(_34.2.1, 0), 0);
_39 = (-48_isize) >> _29;
(*_17) = _9.2 == _23;
(*_1) = core::ptr::addr_of_mut!(_16);
(*_17) = (*_27) >= (*_27);
(*_17) = _23 > _9.2;
_34.2.1 = Adt19::Variant0 { fld0: (*_17),fld1: _9.0,fld2: _39,fld3: _13,fld4: _9.1,fld5: 36721_u16 };
(*_1) = core::ptr::addr_of_mut!((*_33));
(*_27) = RET;
_19 = -_9.2;
(*_27) = _34.0;
_1 = core::ptr::addr_of_mut!((*_1));
(*_17) = !Field::<bool>(Variant(_34.2.1, 0), 0);
_40.2.1 = Adt19::Variant1 { fld0: (-66_i8),fld1: _19,fld2: _35 };
_31 = [230057802975636167082364210974508469390_u128,304559953823032101231245678519492872224_u128,165038044939857006791690435255427027404_u128,266245485402639637185386754063518590661_u128,301024450623260987349750408796242724118_u128,247055558370118897521723596683729029247_u128];
(*_33) = [_29,_29,_29,_29,_29];
(*_17) = (*_27) >= (*_27);
(*_27) = RET;
_31 = [103332825511091823984503472635437026356_u128,328582406833774971970234516331434226886_u128,177705144075567401825040308228033250200_u128,245048976897081654352022490263524457431_u128,202428496519605979147063944474327172323_u128,246316057026838874306693269032564219614_u128];
Goto(bb37)
}
bb37 = {
_42 = !_9.1;
(*_33) = [_29,_24,_24,_29,_29];
_34.2.1 = Adt19::Variant1 { fld0: (-98_i8),fld1: _23,fld2: _35 };
Goto(bb38)
}
bb38 = {
(*_27) = _34.2.0;
(*_1) = core::ptr::addr_of_mut!(_16);
_40.2.1 = Adt19::Variant0 { fld0: (*_17),fld1: _9.0,fld2: _39,fld3: _13,fld4: _42,fld5: 61195_u16 };
(*_27) = _34.2.0;
_45 = _8 - _8;
_36.1 = core::ptr::addr_of_mut!((*_33));
(*_17) = !Field::<bool>(Variant(_40.2.1, 0), 0);
_9.0 = Field::<u8>(Variant(_40.2.1, 0), 1) << _23;
Goto(bb39)
}
bb39 = {
(*_33) = [_29,_29,_24,_29,_24];
_34.2.0 = (*_27);
(*_17) = (*_27) < (*_27);
_41 = ((*_27),);
place!(Field::<bool>(Variant(_40.2.1, 0), 0)) = (*_17) & (*_17);
match _29 {
0 => bb1,
1 => bb35,
2 => bb40,
3 => bb41,
5616447758659087638 => bb43,
_ => bb42
}
}
bb40 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb41 = {
_26 = [60_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _14;
(*_17) = _9.0 == _9.0;
_23 = 49362_u16 as i128;
_31 = [218527510784676959494813057930830804274_u128,246223252593900919580145444061958585413_u128,138017933288057453840729359635151912122_u128,127968606971080619708438704562324631046_u128,89622746233675323333522390550217571722_u128,303221199798220449952175473436193490026_u128];
(*_17) = _19 < _19;
(*_17) = _19 > _19;
_9.1 = 0_usize - 3_usize;
_24 = 5616447758659087638_u64;
_9.3 = _19 - _19;
(*_17) = _9.3 <= _9.3;
_26 = [9223372036854775807_isize,91_isize,(-9223372036854775808_isize),39_isize,(-117_isize)];
(*_17) = false;
(*_17) = !false;
_34.0 = RET;
_29 = !_24;
RET = _34.0;
(*_17) = true;
_29 = _24;
match _24 {
0 => bb10,
1 => bb7,
2 => bb17,
5616447758659087638 => bb32,
_ => bb31
}
}
bb42 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb43 = {
(*_33) = [_24,_24,_24,_29,_29];
_16 = [_24,_24,_24,_24,_24];
_35 = -Field::<i32>(Variant(_34.2.1, 1), 2);
(*_17) = Field::<bool>(Variant(_40.2.1, 0), 0);
_49 = core::ptr::addr_of_mut!(_47);
_13.0 = _45 as f64;
_2 = core::ptr::addr_of_mut!((*_49));
(*_17) = _19 == _23;
(*_33) = [_29,_24,_29,_24,_24];
place!(Field::<i8>(Variant(_34.2.1, 1), 0)) = (-6_i8) >> _23;
_40.2.0 = (*_27);
(*_2) = _18 as i32;
Goto(bb44)
}
bb44 = {
(*_2) = _35;
_48 = core::ptr::addr_of!(_41.0);
(*_49) = 15969_i16 as i32;
(*_49) = Field::<i32>(Variant(_34.2.1, 1), 2) - Field::<i32>(Variant(_34.2.1, 1), 2);
_37 = _18;
_18 = _37 | _37;
(*_48) = (*_27);
(*_48) = (*_27);
(*_1) = Move(_36.1);
(*_27) = (*_48);
(*_1) = core::ptr::addr_of_mut!(_16);
(*_33) = [_29,_29,_29,_29,_24];
_34.2.1 = Adt19::Variant0 { fld0: (*_17),fld1: _9.0,fld2: _39,fld3: Field::<(f64,)>(Variant(_40.2.1, 0), 3),fld4: _42,fld5: 11784_u16 };
_45 = _8;
(*_1) = core::ptr::addr_of_mut!((*_33));
_41 = (RET,);
(*_17) = (*_48) <= (*_27);
_48 = core::ptr::addr_of!((*_27));
(*_33) = [_24,_24,_29,_24,_29];
Call((*_49) = core::intrinsics::transmute((*_48)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
(*_1) = core::ptr::addr_of_mut!((*_33));
_34.0 = (*_27);
match _29 {
0 => bb46,
1 => bb47,
2 => bb48,
3 => bb49,
4 => bb50,
5 => bb51,
5616447758659087638 => bb53,
_ => bb52
}
}
bb46 = {
_9.1 = 3_usize + 17412363180259197723_usize;
_9.2 = 169_u8 as i128;
RET = '\u{ddc28}';
RET = '\u{aa65d}';
_5 = [149_u8,98_u8,213_u8,29_u8,197_u8,45_u8,230_u8];
_4 = [(-9223372036854775808_isize),(-45_isize),9223372036854775807_isize,(-91_isize),9223372036854775807_isize];
_9.2 = 90869071152508288703447126710699833971_i128;
_9.3 = -_9.2;
_9.1 = 8217392693314930211_u64 as usize;
_9 = (181_u8, 13259308704818075969_usize, 21631070190187663295199282016538586743_i128, (-25230387160450081679436751859382528440_i128));
match _9.1 {
0 => bb1,
1 => bb4,
2 => bb5,
13259308704818075969 => bb7,
_ => bb6
}
}
bb47 = {
(*_27) = RET;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_35);
_36.1 = core::ptr::addr_of_mut!((*_33));
_8 = 331112662591763757092134027322188438630_u128 as f32;
(*_27) = _34.2.0;
_40.3 = core::ptr::addr_of_mut!(_35);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0;
_9.3 = Field::<usize>(Variant(_34.2.1, 0), 4) as i128;
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_35);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0;
(*_17) = Field::<bool>(Variant(_34.2.1, 0), 0) & Field::<bool>(Variant(_34.2.1, 0), 0);
(*_27) = _34.0;
(*_17) = Field::<(f64,)>(Variant(_34.2.1, 0), 3).0 >= _13.0;
(*_33) = [_29,_24,_24,_24,_29];
_1 = core::ptr::addr_of_mut!(_33);
_9.0 = !Field::<u8>(Variant(_34.2.1, 0), 1);
place!(Field::<u8>(Variant(_34.2.1, 0), 1)) = _9.0 << _9.2;
_35 = !(-1259608149_i32);
(*_33) = [_29,_29,_24,_24,_29];
_41.0 = (*_27);
_27 = core::ptr::addr_of!((*_27));
(*_33) = [_29,_29,_24,_29,_24];
(*_1) = core::ptr::addr_of_mut!((*_33));
(*_33) = [_24,_24,_24,_29,_29];
_41 = ((*_27),);
Goto(bb36)
}
bb48 = {
RET = '\u{74bbf}';
RET = '\u{7a636}';
RET = '\u{b6222}';
RET = '\u{a1912}';
RET = '\u{1067c7}';
RET = '\u{947a8}';
RET = '\u{9af5}';
RET = '\u{40f52}';
RET = '\u{7d424}';
RET = '\u{723b6}';
RET = '\u{8c43d}';
RET = '\u{e9c8}';
RET = '\u{8f075}';
_4 = [3_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-6_isize)];
RET = '\u{5fbf2}';
RET = '\u{73684}';
_5 = [211_u8,230_u8,140_u8,54_u8,191_u8,243_u8,62_u8];
Goto(bb2)
}
bb49 = {
_26 = [60_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _14;
(*_17) = _9.0 == _9.0;
_23 = 49362_u16 as i128;
_31 = [218527510784676959494813057930830804274_u128,246223252593900919580145444061958585413_u128,138017933288057453840729359635151912122_u128,127968606971080619708438704562324631046_u128,89622746233675323333522390550217571722_u128,303221199798220449952175473436193490026_u128];
(*_17) = _19 < _19;
(*_17) = _19 > _19;
_9.1 = 0_usize - 3_usize;
_24 = 5616447758659087638_u64;
_9.3 = _19 - _19;
(*_17) = _9.3 <= _9.3;
_26 = [9223372036854775807_isize,91_isize,(-9223372036854775808_isize),39_isize,(-117_isize)];
(*_17) = false;
(*_17) = !false;
_34.0 = RET;
_29 = !_24;
RET = _34.0;
(*_17) = true;
_29 = _24;
match _24 {
0 => bb10,
1 => bb7,
2 => bb17,
5616447758659087638 => bb32,
_ => bb31
}
}
bb50 = {
Return()
}
bb51 = {
(*_33) = [_29,_29,_24,_29,_24];
_34.2.0 = (*_27);
(*_17) = (*_27) < (*_27);
_41 = ((*_27),);
place!(Field::<bool>(Variant(_40.2.1, 0), 0)) = (*_17) & (*_17);
match _29 {
0 => bb1,
1 => bb35,
2 => bb40,
3 => bb41,
5616447758659087638 => bb43,
_ => bb42
}
}
bb52 = {
RET = '\u{86f7}';
_5 = [82_u8,47_u8,51_u8,101_u8,10_u8,99_u8,1_u8];
_5 = [77_u8,2_u8,244_u8,113_u8,40_u8,8_u8,55_u8];
_7 = Adt45::Variant2 { fld0: Move(_2) };
_5 = [96_u8,112_u8,165_u8,185_u8,212_u8,47_u8,111_u8];
RET = '\u{34698}';
_2 = Move(Field::<*mut i32>(Variant(_7, 2), 0));
place!(Field::<*mut i32>(Variant(_7, 2), 0)) = Move(_2);
_8 = (-84_isize) as f32;
_9.2 = -76492747400033879737644262796156675785_i128;
Goto(bb3)
}
bb53 = {
(*_27) = _34.0;
_9.0 = (*_49) as u8;
Goto(bb54)
}
bb54 = {
Call(_53 = dump_var(Move(_29), Move(_31), Move(_5), Move(_14)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_53 = dump_var(Move(_26), Move(_24), Move(_35), Move(_16)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_53 = dump_var(Move(_9), _54, _54, _54), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *mut i32,mut _2: *mut *mut [u64; 5],mut _3: f32,mut _4: (u8, usize, i128, i128)) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _5: [u32; 2];
let _6: &'static mut bool;
let _7: [u8; 7];
let _8: *mut i128;
let _9: i8;
let _10: &'static &'static mut *mut i32;
let _11: (u8, usize, i128, i128);
let _12: i8;
let _13: *mut i32;
let _14: Adt25;
let _15: *mut u32;
let _16: i8;
let _17: &'static &'static mut *mut i32;
let _18: i16;
let _19: (&'static mut i16, *mut [u64; 5]);
let _20: f32;
let _21: *const (u8, usize, i128, i128);
let _22: u32;
let _23: i128;
let _24: Adt19;
let _25: &'static mut *mut [u64; 5];
let _26: i8;
let _27: isize;
let _28: *const &'static &'static mut bool;
let _29: [u8; 6];
let _30: &'static Adt19;
let _31: (char,);
let _32: &'static mut u16;
let _33: isize;
let _34: u64;
let _35: isize;
let _36: Adt19;
let _37: i128;
let _38: &'static mut u16;
let _39: [u16; 3];
let _40: bool;
let _41: isize;
let _42: f64;
let _43: i128;
let _44: *mut u32;
let _45: &'static &'static mut bool;
let _46: *const &'static &'static mut bool;
let _47: i16;
let _48: u8;
let _49: *const char;
let _50: u16;
let _51: [usize; 3];
let _52: f32;
let _53: bool;
let _54: usize;
let _55: &'static mut &'static (char,);
let _56: f64;
let _57: [u32; 2];
let _58: &'static mut [u8; 6];
let _59: char;
let _60: ((char, Adt19),);
let _61: char;
let _62: *const *mut i32;
let _63: &'static mut (f64,);
let _64: &'static mut [i128; 6];
let _65: i32;
let _66: u128;
let _67: Adt46;
let _68: &'static u128;
let _69: i8;
let _70: *const &'static &'static mut bool;
let _71: i128;
let _72: i64;
let _73: [i16; 2];
let _74: *const u16;
let _75: Adt27;
let _76: &'static mut i16;
let _77: &'static mut *mut [u64; 5];
let _78: u16;
let _79: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _80: f64;
let _81: isize;
let _82: ((char, Adt19),);
let _83: Adt45;
let _84: [usize; 3];
let _85: ();
let _86: ();
{
RET = [28_isize,(-9223372036854775808_isize),110_isize,127_isize,(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,111_isize,(-108_isize),(-96_isize),(-9223372036854775808_isize)];
_4.3 = _4.2;
match _4.2 {
0 => bb1,
239231355736061296810777985005606356155 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
_5 = [1580653283_u32,3096269795_u32];
_5 = [2392954963_u32,4242370936_u32];
_3 = 28160_u16 as f32;
_4.2 = _4.3;
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),70_isize,9223372036854775807_isize];
_8 = core::ptr::addr_of_mut!(_11.3);
(*_8) = -_4.2;
_11 = _4;
_12 = !1_i8;
_5 = [1809572049_u32,19386262_u32];
(*_8) = !_4.3;
(*_8) = _3 as i128;
(*_8) = _4.3 * _11.2;
(*_8) = !_11.2;
_3 = _12 as f32;
Goto(bb4)
}
bb4 = {
_4.1 = _11.1 - _11.1;
(*_8) = _4.2 + _11.2;
(*_8) = _11.2 - _4.3;
(*_8) = _4.2;
(*_8) = _4.3 * _11.2;
_11.0 = _4.0 - _4.0;
(*_8) = 55067_u16 as i128;
(*_8) = _11.2 | _11.2;
(*_8) = _4.2 & _4.2;
RET = [9223372036854775807_isize,9223372036854775807_isize,(-96_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
(*_8) = !_4.3;
(*_8) = _11.2 >> _4.2;
(*_8) = 224127093812538681926682533632644101936_u128 as i128;
_12 = -2_i8;
_9 = _12 << (*_8);
(*_8) = _4.2 << _4.1;
Call((*_8) = fn16(RET), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_8) = !_11.2;
(*_8) = _11.2;
(*_8) = _4.3 >> _11.2;
_7 = [_4.0,_11.0,_4.0,_4.0,_11.0,_4.0,_4.0];
(*_8) = 6719446371745397950_i64 as i128;
_4 = _11;
(*_8) = -_4.2;
_11.2 = (-31402_i16) as i128;
(*_8) = _4.2 & _4.3;
(*_8) = 153533209737535826297460067702467436477_u128 as i128;
(*_8) = _4.3 + _11.2;
_8 = core::ptr::addr_of_mut!((*_8));
_4.3 = -(*_8);
(*_8) = _4.3 << _11.0;
_4 = _11;
(*_8) = _4.3 >> _4.3;
match _11.1 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
7 => bb10,
_ => bb9
}
}
bb6 = {
_4.1 = _11.1 - _11.1;
(*_8) = _4.2 + _11.2;
(*_8) = _11.2 - _4.3;
(*_8) = _4.2;
(*_8) = _4.3 * _11.2;
_11.0 = _4.0 - _4.0;
(*_8) = 55067_u16 as i128;
(*_8) = _11.2 | _11.2;
(*_8) = _4.2 & _4.2;
RET = [9223372036854775807_isize,9223372036854775807_isize,(-96_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
(*_8) = !_4.3;
(*_8) = _11.2 >> _4.2;
(*_8) = 224127093812538681926682533632644101936_u128 as i128;
_12 = -2_i8;
_9 = _12 << (*_8);
(*_8) = _4.2 << _4.1;
Call((*_8) = fn16(RET), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
_5 = [1580653283_u32,3096269795_u32];
_5 = [2392954963_u32,4242370936_u32];
_3 = 28160_u16 as f32;
_4.2 = _4.3;
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),70_isize,9223372036854775807_isize];
_8 = core::ptr::addr_of_mut!(_11.3);
(*_8) = -_4.2;
_11 = _4;
_12 = !1_i8;
_5 = [1809572049_u32,19386262_u32];
(*_8) = !_4.3;
(*_8) = _3 as i128;
(*_8) = _4.3 * _11.2;
(*_8) = !_11.2;
_3 = _12 as f32;
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
(*_8) = _3 as i128;
_4.0 = !_11.0;
_14.fld4 = 9223372036854775807_isize as u16;
(*_8) = _4.3 + _4.3;
(*_8) = !_4.3;
_16 = _9;
_11 = (_4.0, _4.1, _4.3, _4.2);
_4.0 = _11.0 - _11.0;
(*_8) = _4.3 - _4.3;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-83_isize)];
_11.3 = _3 as i128;
(*_8) = _11.2 + _11.2;
(*_8) = _11.2;
_7 = [_4.0,_11.0,_4.0,_4.0,_11.0,_4.0,_4.0];
_8 = core::ptr::addr_of_mut!((*_8));
_18 = 13355_i16 - 9203_i16;
(*_8) = _3 as i128;
_11 = _4;
_14.fld2 = !_4.1;
(*_8) = -_11.2;
(*_8) = !_4.3;
(*_8) = _4.3 << _4.1;
(*_8) = !_4.3;
(*_8) = _4.3;
(*_8) = _4.2;
_2 = core::ptr::addr_of_mut!(_19.1);
Goto(bb11)
}
bb11 = {
(*_8) = _4.3 << _18;
(*_8) = _16 as i128;
_16 = _12 ^ _9;
(*_8) = _3 as i128;
_11.1 = _14.fld2;
_14.fld5 = !627122425_u32;
_16 = _9;
(*_8) = -_4.3;
_11.3 = _4.2;
_14.fld5 = !1856064448_u32;
_14.fld3 = _16 - _12;
(*_8) = _4.3 >> _12;
(*_8) = !_4.3;
_13 = Move(_1);
_4 = (_11.0, _11.1, (*_8), (*_8));
(*_8) = -_4.3;
_15 = core::ptr::addr_of_mut!(_14.fld5);
(*_15) = 4214939163_u32 >> (*_8);
(*_8) = _11.2;
_16 = _14.fld3 << (*_15);
_14.fld1 = _3 + _3;
_11.2 = !_11.3;
_1 = Move(_13);
_14.fld3 = _16 | _16;
(*_8) = _14.fld4 as i128;
_14.fld0 = false | true;
(*_8) = !_4.3;
(*_8) = _4.2;
Goto(bb12)
}
bb12 = {
(*_8) = _14.fld0 as i128;
Goto(bb13)
}
bb13 = {
(*_8) = !_4.3;
(*_8) = _14.fld0 as i128;
_18 = (*_15) as i16;
_18 = !(-19740_i16);
(*_15) = 3389136334_u32 ^ 4114799372_u32;
_20 = _14.fld1 - _14.fld1;
(*_15) = (-715471911_i32) as u32;
(*_15) = !4110104514_u32;
_19.0 = &mut _18;
_14.fld0 = true ^ false;
_21 = core::ptr::addr_of!(_11);
(*_21).0 = _4.0 - _4.0;
(*_21) = (_4.0, _14.fld2, _4.2, _4.2);
(*_15) = !609136237_u32;
(*_21).0 = (-15733_i16) as u8;
(*_21).2 = (*_8) << (*_21).3;
_6 = &mut _14.fld0;
(*_21).0 = _4.0 * _4.0;
(*_21) = _4;
(*_21).2 = (*_21).3 - (*_8);
Goto(bb14)
}
bb14 = {
(*_21).0 = (*_15) as u8;
(*_21).3 = (*_21).2 & (*_21).2;
(*_6) = !false;
(*_21).0 = !_4.0;
(*_21).1 = _4.1 - _4.1;
(*_21).1 = 15021954553640923521_u64 as usize;
(*_21) = (_4.0, _4.1, _4.2, _4.2);
(*_8) = 12384_i16 as i128;
(*_21).0 = (*_21).1 as u8;
(*_21).1 = _4.1 | _4.1;
(*_8) = (*_21).2;
Goto(bb15)
}
bb15 = {
(*_15) = 1048172809_u32 | 3953171145_u32;
_4.2 = _4.3 ^ (*_21).2;
(*_21).1 = _4.1 - _4.1;
(*_8) = _20 as i128;
(*_21).2 = -(*_8);
(*_21).2 = (*_8) ^ _11.3;
(*_8) = (*_21).2 + (*_21).2;
(*_6) = true;
(*_15) = 1796947700_u32;
_22 = !(*_15);
(*_21).1 = !_4.1;
_4 = _11;
_4.3 = _16 as i128;
(*_21).1 = !_4.1;
(*_21).2 = -(*_8);
_11.0 = _4.0 * _4.0;
(*_6) = (*_21).0 != (*_21).0;
(*_21).1 = _16 as usize;
(*_21).2 = _4.3 | (*_8);
match (*_15) {
0 => bb13,
1796947700 => bb17,
_ => bb16
}
}
bb16 = {
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
_5 = [1580653283_u32,3096269795_u32];
_5 = [2392954963_u32,4242370936_u32];
_3 = 28160_u16 as f32;
_4.2 = _4.3;
_7 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),70_isize,9223372036854775807_isize];
_8 = core::ptr::addr_of_mut!(_11.3);
(*_8) = -_4.2;
_11 = _4;
_12 = !1_i8;
_5 = [1809572049_u32,19386262_u32];
(*_8) = !_4.3;
(*_8) = _3 as i128;
(*_8) = _4.3 * _11.2;
(*_8) = !_11.2;
_3 = _12 as f32;
Goto(bb4)
}
bb17 = {
(*_15) = _22 | _22;
(*_21).2 = !(*_8);
(*_15) = !_22;
(*_15) = !_22;
(*_21).1 = _4.1 + _4.1;
_11 = _4;
(*_21).0 = !_4.0;
(*_15) = _22 ^ _22;
(*_8) = (*_6) as i128;
(*_15) = _22 - _22;
(*_21).2 = -_11.3;
_21 = core::ptr::addr_of!(_4);
(*_21).0 = !_11.0;
(*_21).3 = -(*_8);
(*_21).2 = (*_21).3;
(*_21).2 = (*_8) >> (*_8);
(*_8) = (*_21).3 << (*_21).3;
Goto(bb18)
}
bb18 = {
(*_21) = _11;
(*_8) = (*_6) as i128;
(*_21).3 = (*_8);
_4 = (_11.0, _11.1, (*_8), (*_8));
(*_21) = (_11.0, _11.1, (*_8), (*_8));
(*_15) = !_22;
Call(_2 = fn17(), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
(*_15) = _22;
_33 = (-102_isize) + (-9223372036854775808_isize);
(*_21).2 = -(*_8);
(*_8) = (*_21).2;
(*_8) = (*_21).2;
(*_6) = !false;
(*_21).0 = 7231769571465458522_u64 as u8;
_27 = _33 | _33;
(*_6) = !true;
(*_21).0 = _11.0 ^ _11.0;
Call((*_21).0 = core::intrinsics::transmute((*_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_4.1 = !_11.1;
(*_8) = (*_21).3 + (*_21).2;
(*_8) = (*_21).3 | (*_21).2;
_34 = 17999165776941921955_u64 | 11883034571818334032_u64;
(*_8) = (*_21).3 + (*_21).2;
(*_6) = true;
Goto(bb21)
}
bb21 = {
(*_21).0 = _11.0;
(*_21).2 = (*_8);
_4 = (_11.0, _11.1, (*_8), (*_8));
(*_8) = (*_21).2 & (*_21).3;
(*_21).3 = '\u{27568}' as i128;
(*_21).3 = (*_8) | (*_8);
(*_21) = (_11.0, _11.1, (*_8), (*_8));
_31 = ('\u{a719e}',);
Goto(bb22)
}
bb22 = {
(*_21) = _11;
(*_8) = (*_21).2 >> (*_21).2;
(*_21).0 = _11.0 >> (*_8);
(*_8) = 52832_u16 as i128;
(*_21).1 = !_11.1;
_20 = _3;
_27 = (*_15) as isize;
_11.0 = !(*_21).0;
(*_15) = _34 as u32;
(*_8) = -(*_21).3;
(*_8) = -(*_21).3;
(*_8) = _34 as i128;
(*_21).3 = (*_21).2 ^ _4.2;
_26 = _16 << _4.0;
(*_21).0 = _11.0;
(*_21) = _11;
(*_21).3 = (*_21).2;
Goto(bb23)
}
bb23 = {
(*_15) = _22 & _22;
(*_15) = 27998_i16 as u32;
(*_15) = _22 << (*_21).0;
Goto(bb24)
}
bb24 = {
_4.1 = !_11.1;
_5 = [(*_15),(*_15)];
(*_21) = _11;
(*_8) = !(*_21).2;
_35 = _33;
(*_21) = (_11.0, _11.1, (*_8), (*_8));
(*_8) = _26 as i128;
(*_8) = (*_21).3;
(*_21).3 = (*_21).2 >> _26;
(*_21).3 = !(*_21).2;
(*_21).2 = (*_21).1 as i128;
(*_6) = (*_15) != (*_15);
(*_6) = _26 == _26;
_42 = _34 as f64;
(*_21) = (_11.0, _11.1, (*_8), (*_8));
_37 = (*_8) << (*_8);
_9 = (*_21).1 as i8;
(*_6) = true;
_4.1 = _11.1;
(*_21).3 = 8285_u16 as i128;
(*_21).0 = !_11.0;
(*_21).2 = _11.2 ^ (*_8);
(*_21).0 = _11.0 << _37;
(*_6) = !true;
_37 = !(*_21).2;
(*_21).1 = _11.1 & _11.1;
Goto(bb25)
}
bb25 = {
(*_15) = _22 + _22;
(*_21).2 = _20 as i128;
_5 = [(*_15),(*_15)];
(*_6) = _11.0 > (*_21).0;
(*_15) = _22 << (*_8);
(*_21).2 = (*_6) as i128;
Call(_35 = core::intrinsics::bswap(_33), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_43 = (*_21).2 << (*_21).2;
(*_15) = !_22;
_40 = _11.0 < (*_21).0;
(*_21).1 = !_11.1;
(*_6) = (*_8) < (*_21).2;
_49 = core::ptr::addr_of!(_31.0);
(*_21).0 = _11.0 >> (*_21).2;
_31.0 = '\u{78623}';
_29 = [(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0];
_21 = core::ptr::addr_of!(_11);
Goto(bb27)
}
bb27 = {
_15 = core::ptr::addr_of_mut!((*_15));
(*_8) = -_43;
(*_21).2 = (*_8) * (*_8);
_8 = core::ptr::addr_of_mut!((*_8));
(*_6) = (*_21).2 < (*_8);
(*_21).1 = _4.1;
_44 = core::ptr::addr_of_mut!((*_15));
(*_21).1 = !_4.1;
_45 = &_6;
(*_15) = _22;
(*_21).2 = (*_8) + (*_8);
(*_49) = '\u{e99c9}';
_49 = core::ptr::addr_of!((*_49));
_53 = (*_6);
_31.0 = '\u{6668e}';
(*_49) = '\u{8eb02}';
(*_15) = _22 | _22;
_47 = 16154_i16 ^ 11790_i16;
_4.0 = (*_21).0;
(*_21).0 = _3 as u8;
Goto(bb28)
}
bb28 = {
(*_49) = '\u{34a89}';
_39 = [62121_u16,54424_u16,47843_u16];
(*_6) = !_53;
(*_6) = !_40;
(*_21).1 = _4.1 >> (*_8);
(*_21).1 = _4.1 * _4.1;
(*_49) = '\u{43aa3}';
_41 = !_27;
_52 = _3 + _3;
(*_6) = _53;
(*_21).2 = (*_8);
(*_6) = _53;
(*_15) = _22 + _22;
_53 = (*_6) ^ (*_6);
Call((*_8) = core::intrinsics::bswap((*_21).2), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
(*_6) = (*_21).2 <= (*_21).2;
(*_21).2 = _11.1 as i128;
(*_21).0 = 9077603417476002667_i64 as u8;
_4.2 = (*_8) << (*_8);
Goto(bb30)
}
bb30 = {
(*_21).0 = (-4978265153281266588_i64) as u8;
_11.0 = 53156_u16 as u8;
(*_8) = _43 - _4.2;
(*_6) = _40;
_31.0 = '\u{caa0e}';
(*_15) = _22 + _22;
_23 = (*_8) >> (*_21).0;
(*_49) = '\u{885a7}';
_44 = core::ptr::addr_of_mut!((*_15));
_59 = (*_49);
(*_21).0 = _4.0 * _4.0;
(*_8) = _4.2;
(*_21).0 = (*_6) as u8;
_39 = [32486_u16,52349_u16,38366_u16];
(*_21).2 = (*_8) << (*_15);
(*_44) = _22 << _23;
(*_21).0 = (-2843958598510501719_i64) as u8;
Goto(bb31)
}
bb31 = {
(*_6) = (*_44) <= (*_15);
(*_44) = _22;
_66 = 116686458026709956041302424468280574759_u128 & 11578774252495372275600969782153212969_u128;
Goto(bb32)
}
bb32 = {
Goto(bb33)
}
bb33 = {
(*_49) = _59;
(*_44) = _22;
Goto(bb34)
}
bb34 = {
(*_21).2 = (*_8) * (*_8);
_29 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
(*_21).0 = _4.0;
_33 = (*_8) as isize;
_67.fld7 = (*_44) as i128;
_67.fld4.fld1 = (*_8) as f32;
_1 = core::ptr::addr_of_mut!(_65);
(*_8) = !(*_21).2;
_11.1 = _4.1 ^ _4.1;
Goto(bb35)
}
bb35 = {
_40 = (*_21).2 != (*_21).2;
_52 = (*_21).0 as f32;
(*_49) = _59;
(*_1) = -194798769_i32;
(*_15) = _22;
(*_15) = _67.fld4.fld1 as u32;
_73 = [_47,_47];
_70 = core::ptr::addr_of!(_45);
_58 = &mut _29;
(*_1) = (-371024344_i32) & (-1577223026_i32);
(*_1) = 241299041_i32 << (*_21).2;
Goto(bb36)
}
bb36 = {
_67.fld4.fld0 = (*_1) == (*_1);
(*_21).1 = _4.1 & _4.1;
(*_21).1 = _4.1 - _4.1;
(*_21).0 = !_4.0;
_57 = [(*_15),(*_15)];
(*_21).1 = _4.1 >> (*_1);
(*_49) = _59;
(*_49) = _59;
(*_1) = 478022149_i32 << (*_21).0;
_60.0.0 = (*_49);
_65 = (-952999815_i32) & (-105169975_i32);
Call(_67.fld4.fld4 = core::intrinsics::bswap(51304_u16), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
_5 = _57;
Goto(bb38)
}
bb38 = {
_52 = (*_21).1 as f32;
_7 = [(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0];
(*_21).2 = _66 as i128;
(*_21).0 = _4.0 << (*_8);
Goto(bb39)
}
bb39 = {
(*_49) = _60.0.0;
(*_15) = !_22;
(*_8) = (*_6) as i128;
_56 = _42;
_67.fld3.1 = (*_15) as f32;
_47 = 15442_i16 | 12639_i16;
Call(_15 = fn18(Move(_70), Move((*_70))), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
(*_49) = _60.0.0;
(*_21).2 = (*_8) * (*_8);
Goto(bb41)
}
bb41 = {
_13 = core::ptr::addr_of_mut!((*_1));
(*_21).2 = (*_8) - _43;
_4.2 = -(*_21).2;
(*_6) = !_53;
_61 = (*_49);
_67.fld4.fld3 = _26 ^ _16;
(*_21).2 = (*_8);
_45 = &_6;
_78 = 60706_u16;
_19.1 = core::ptr::addr_of_mut!(_67.fld0);
(*_21).0 = _4.0 | _4.0;
_72 = !(-6046114384076167174_i64);
(*_6) = (*_21).1 >= (*_21).1;
(*_21).2 = -(*_8);
(*_1) = (-782434829_i32) ^ (-449038501_i32);
(*_58) = [(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0];
_43 = (*_21).2 * (*_8);
(*_1) = -(-869338284_i32);
(*_6) = !_53;
match _78 {
0 => bb42,
60706 => bb44,
_ => bb43
}
}
bb42 = {
(*_49) = '\u{34a89}';
_39 = [62121_u16,54424_u16,47843_u16];
(*_6) = !_53;
(*_6) = !_40;
(*_21).1 = _4.1 >> (*_8);
(*_21).1 = _4.1 * _4.1;
(*_49) = '\u{43aa3}';
_41 = !_27;
_52 = _3 + _3;
(*_6) = _53;
(*_21).2 = (*_8);
(*_6) = _53;
(*_15) = _22 + _22;
_53 = (*_6) ^ (*_6);
Call((*_8) = core::intrinsics::bswap((*_21).2), ReturnTo(bb29), UnwindUnreachable())
}
bb43 = {
_5 = _57;
Goto(bb38)
}
bb44 = {
_71 = (*_21).2 * (*_8);
_61 = (*_49);
(*_49) = _60.0.0;
_4.2 = (*_8) ^ (*_21).2;
_13 = core::ptr::addr_of_mut!((*_1));
(*_6) = !_67.fld4.fld0;
_28 = core::ptr::addr_of!(_45);
(*_8) = (*_21).2;
(*_13) = (-647549931_i32) ^ 865384323_i32;
match _78 {
0 => bb45,
1 => bb46,
2 => bb47,
3 => bb48,
60706 => bb50,
_ => bb49
}
}
bb45 = {
(*_8) = _14.fld0 as i128;
Goto(bb13)
}
bb46 = {
(*_15) = _22 & _22;
(*_15) = 27998_i16 as u32;
(*_15) = _22 << (*_21).0;
Goto(bb24)
}
bb47 = {
(*_21).2 = (*_8) * (*_8);
_29 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
(*_21).0 = _4.0;
_33 = (*_8) as isize;
_67.fld7 = (*_44) as i128;
_67.fld4.fld1 = (*_8) as f32;
_1 = core::ptr::addr_of_mut!(_65);
(*_8) = !(*_21).2;
_11.1 = _4.1 ^ _4.1;
Goto(bb35)
}
bb48 = {
Return()
}
bb49 = {
(*_49) = '\u{34a89}';
_39 = [62121_u16,54424_u16,47843_u16];
(*_6) = !_53;
(*_6) = !_40;
(*_21).1 = _4.1 >> (*_8);
(*_21).1 = _4.1 * _4.1;
(*_49) = '\u{43aa3}';
_41 = !_27;
_52 = _3 + _3;
(*_6) = _53;
(*_21).2 = (*_8);
(*_6) = _53;
(*_15) = _22 + _22;
_53 = (*_6) ^ (*_6);
Call((*_8) = core::intrinsics::bswap((*_21).2), ReturnTo(bb29), UnwindUnreachable())
}
bb50 = {
(*_58) = [(*_21).0,_11.0,(*_21).0,(*_21).0,(*_21).0,(*_21).0];
_72 = !(-1006424432606223260_i64);
(*_58) = [(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0];
(*_8) = _72 as i128;
(*_8) = !(*_21).2;
_31.0 = _61;
(*_21).2 = -(*_8);
(*_49) = _60.0.0;
(*_49) = _60.0.0;
(*_58) = [(*_21).0,(*_21).0,(*_21).0,(*_21).0,(*_21).0,_4.0];
Goto(bb51)
}
bb51 = {
Call(_85 = dump_var(Move(_11), Move(_12), Move(_29), Move(_34)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_85 = dump_var(Move(_57), Move(_43), Move(_59), Move(_9)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_85 = dump_var(Move(_5), Move(_23), Move(_47), Move(_35)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_85 = dump_var(Move(_78), Move(_31), Move(_22), Move(_65)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [isize; 5]) -> i128 {
mir! {
type RET = i128;
let _2: bool;
let _3: Adt27;
let _4: u16;
let _5: *mut [u64; 5];
let _6: &'static &'static mut *mut i32;
let _7: [usize; 3];
let _8: i128;
let _9: u8;
let _10: isize;
let _11: [isize; 6];
let _12: (char, Adt19);
let _13: u8;
let _14: *const char;
let _15: (u8, u8);
let _16: ();
let _17: ();
{
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = !9138401331124329316875901433908038608_i128;
RET = 46908592067067130395232874909198669909_i128 & (-90875839753159187179738307173970087749_i128);
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = '\u{9fb03}' as u16;
RET = !66322433087946588932859824030307261440_i128;
_2 = _4 != _4;
_2 = true ^ true;
_2 = true;
_2 = !true;
RET = 37493799305127917595159125094239593677_i128 & (-99540093771112256295423763077700865429_i128);
_1 = [9223372036854775807_isize,89_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_7 = [0_usize,5453125710273481391_usize,6966886713662916428_usize];
_9 = 2210032989_u32 as u8;
_7 = [1_usize,0_usize,2_usize];
_4 = !2432_u16;
_4 = 8519_u16 - 62074_u16;
_2 = true;
Goto(bb1)
}
bb1 = {
_7 = [3_usize,0_usize,13630074796111174986_usize];
_8 = RET ^ RET;
_10 = 9223372036854775807_isize;
RET = _8 & _8;
RET = -_8;
_9 = 223_u8;
RET = _8 - _8;
_8 = 4249162091_u32 as i128;
_2 = RET <= RET;
_8 = RET * RET;
_1 = [_10,_10,_10,_10,_10];
_7 = [0_usize,11409567174190563751_usize,0_usize];
_8 = -RET;
_1 = [_10,_10,_10,_10,_10];
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
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
_11 = [_10,_10,_10,_10,_10,_10];
_10 = (-24913956_i32) as isize;
RET = _8;
_7 = [3648588011388596950_usize,1_usize,2_usize];
_10 = 8101_i16 as isize;
_7 = [3_usize,1_usize,5_usize];
_10 = 182930372816378947725931383880953067739_u128 as isize;
_10 = (-88_isize) * 116_isize;
_11 = [_10,_10,_10,_10,_10,_10];
_7 = [7_usize,6_usize,15584183448998463263_usize];
Goto(bb8)
}
bb8 = {
_1 = [_10,_10,_10,_10,_10];
_7 = [14830857609727269901_usize,1342378989047413554_usize,0_usize];
RET = !_8;
_12.1 = Adt19::Variant1 { fld0: 37_i8,fld1: _8,fld2: (-932124981_i32) };
place!(Field::<i8>(Variant(_12.1, 1), 0)) = 112_i8;
place!(Field::<i8>(Variant(_12.1, 1), 0)) = '\u{10cefa}' as i8;
RET = Field::<i128>(Variant(_12.1, 1), 1) * _8;
_1 = [_10,_10,_10,_10,_10];
place!(Field::<i128>(Variant(_12.1, 1), 1)) = _8;
_1 = [_10,_10,_10,_10,_10];
Goto(bb9)
}
bb9 = {
Call(_16 = dump_var(Move(_11), Move(_4), Move(_9), Move(_7)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17() -> *mut *mut [u64; 5] {
mir! {
type RET = *mut *mut [u64; 5];
let _1: u64;
let _2: *const [i16; 2];
let _3: f32;
let _4: f32;
let _5: [u64; 4];
let _6: i32;
let _7: u128;
let _8: &'static mut [u8; 6];
let _9: isize;
let _10: &'static mut [u8; 6];
let _11: &'static mut (f64,);
let _12: bool;
let _13: [u16; 3];
let _14: *const &'static &'static mut bool;
let _15: f32;
let _16: usize;
let _17: i32;
let _18: &'static mut [i128; 6];
let _19: isize;
let _20: char;
let _21: u32;
let _22: isize;
let _23: isize;
let _24: bool;
let _25: char;
let _26: *const char;
let _27: f32;
let _28: char;
let _29: [usize; 3];
let _30: &'static mut &'static (char,);
let _31: u32;
let _32: Adt45;
let _33: *const [u8; 7];
let _34: *mut [u64; 5];
let _35: i16;
let _36: f32;
let _37: *const [u8; 7];
let _38: f64;
let _39: isize;
let _40: u128;
let _41: i16;
let _42: &'static mut &'static (char,);
let _43: &'static mut &'static mut bool;
let _44: isize;
let _45: &'static u16;
let _46: f32;
let _47: *mut u32;
let _48: &'static &'static mut *mut i32;
let _49: &'static (u32, f32, bool);
let _50: isize;
let _51: f64;
let _52: [bool; 7];
let _53: bool;
let _54: (&'static mut i16, *mut [u64; 5]);
let _55: f64;
let _56: *mut [u64; 5];
let _57: &'static &'static mut bool;
let _58: f64;
let _59: u32;
let _60: bool;
let _61: &'static u128;
let _62: [u32; 2];
let _63: *const char;
let _64: *const [u8; 7];
let _65: (u32, f32, bool);
let _66: *const (u8, usize, i128, i128);
let _67: f64;
let _68: &'static (char,);
let _69: u64;
let _70: (char,);
let _71: f32;
let _72: &'static mut [bool; 7];
let _73: *mut *mut [u64; 5];
let _74: isize;
let _75: i64;
let _76: (char, Adt19);
let _77: f64;
let _78: &'static u128;
let _79: *mut i128;
let _80: &'static mut [i128; 6];
let _81: [u8; 7];
let _82: [usize; 3];
let _83: bool;
let _84: ();
let _85: ();
{
_1 = !2083664553818080135_u64;
_1 = 6224245663708851718_u64 << (-471373274480327902_i64);
_1 = !12490528251203679901_u64;
_1 = 16086878224962626570_u64 ^ 6355502797895151640_u64;
_1 = 3571043180856762914_u64 >> (-70339149628601023602384456518321321618_i128);
_1 = 999268340205933716_u64 - 7555157851545724352_u64;
_1 = 960830842938007178_u64;
_1 = 16813641610973837113_u64;
Goto(bb1)
}
bb1 = {
_3 = (-193607503_i32) as f32;
_3 = (-6927214523957263164_i64) as f32;
_3 = 223_u8 as f32;
_4 = -_3;
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_1 = 7495960536126002728_u64;
_5 = [_1,_1,_1,_1];
_1 = 1681979838234961202_u64;
_1 = '\u{811bb}' as u64;
_5 = [_1,_1,_1,_1];
_6 = (-122357061_i32) | 2069189707_i32;
_4 = _3 + _3;
_5 = [_1,_1,_1,_1];
Goto(bb2)
}
bb2 = {
_6 = (-1819328203_i32) << _1;
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_3 = _4 + _4;
_7 = 180840959643826050885566199479084495013_u128 | 283027413030671528483838348835045807110_u128;
_7 = false as u128;
_9 = 9223372036854775807_isize + 9223372036854775807_isize;
_6 = 44_i8 as i32;
_4 = _1 as f32;
Goto(bb3)
}
bb3 = {
_9 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_9 = 9223372036854775807_isize - (-9223372036854775808_isize);
_7 = 326826675756994275445043312463881496889_u128;
_5 = [_1,_1,_1,_1];
_1 = 14815747202271490239_u64;
_1 = !10967443289419482566_u64;
_3 = -_4;
_4 = _6 as f32;
_7 = 55398789076765101321787563943467056209_u128;
_13 = [41999_u16,60938_u16,55972_u16];
_9 = (-9223372036854775808_isize);
_1 = 9552773243903878926_u64;
_13 = [21866_u16,45241_u16,63661_u16];
_13 = [6525_u16,44928_u16,16660_u16];
_13 = [41177_u16,60547_u16,29182_u16];
_12 = false ^ false;
_13 = [62290_u16,60409_u16,36573_u16];
_1 = 7114757294389946728_u64 >> _9;
_12 = !false;
_7 = !237426603249735977580295923615982347724_u128;
_5 = [_1,_1,_1,_1];
_7 = 214857902458772051934398387541146595454_u128 - 307386318885936443619003028309094568555_u128;
_15 = 1607400218_u32 as f32;
_20 = '\u{a7655}';
Goto(bb4)
}
bb4 = {
_17 = (-3528_i16) as i32;
_5 = [_1,_1,_1,_1];
_9 = !(-73_isize);
_17 = _6 << _6;
_21 = 2003954495_u32 & 4059919724_u32;
_15 = _3;
_4 = _15 * _15;
_15 = -_4;
Goto(bb5)
}
bb5 = {
_21 = 81781644_u32 | 2210803351_u32;
_16 = 2_usize >> _17;
_15 = _17 as f32;
_21 = 2966427255_u32 + 2451501121_u32;
_7 = 136134303999119304606407577210925888918_i128 as u128;
_20 = '\u{d0c7}';
_4 = _3 - _15;
_7 = !49961011799984209843515271887328712162_u128;
_9 = 9223372036854775807_isize;
_13 = [47230_u16,52204_u16,42243_u16];
_17 = _6 & _6;
_22 = _9 & _9;
_12 = false;
_20 = '\u{78892}';
_15 = _3 - _4;
_3 = _4 * _15;
_21 = 3962944497_u32 >> _17;
_9 = _22 + _22;
_20 = '\u{71a08}';
_24 = _12;
_17 = _6;
_23 = 87_u8 as isize;
_3 = -_4;
Goto(bb6)
}
bb6 = {
_5 = [_1,_1,_1,_1];
Goto(bb7)
}
bb7 = {
_5 = [_1,_1,_1,_1];
_6 = _17;
_15 = -_4;
_5 = [_1,_1,_1,_1];
_6 = _17 * _17;
_15 = _3 * _3;
Goto(bb8)
}
bb8 = {
_6 = -_17;
_16 = 6497482687711094991_usize + 6548594662810908549_usize;
_16 = 6_usize;
_25 = _20;
_15 = -_3;
_26 = core::ptr::addr_of!(_25);
(*_26) = _20;
(*_26) = _20;
_22 = 4709_i16 as isize;
_20 = (*_26);
(*_26) = _20;
_17 = 54287_u16 as i32;
Goto(bb9)
}
bb9 = {
(*_26) = _20;
_12 = (*_26) < (*_26);
_13 = [40889_u16,44046_u16,10072_u16];
(*_26) = _20;
_26 = core::ptr::addr_of!(_28);
(*_26) = _20;
_20 = (*_26);
(*_26) = _25;
_13 = [42247_u16,21042_u16,25729_u16];
(*_26) = _25;
(*_26) = _25;
_13 = [29419_u16,21755_u16,25160_u16];
_5 = [_1,_1,_1,_1];
(*_26) = _20;
(*_26) = _25;
(*_26) = _25;
(*_26) = _25;
(*_26) = _20;
_21 = !2712290447_u32;
_21 = 3531856339_u32 + 1784479115_u32;
(*_26) = _25;
_19 = _23 >> _21;
_21 = 55706_u16 as u32;
(*_26) = _25;
Goto(bb10)
}
bb10 = {
_26 = core::ptr::addr_of!((*_26));
(*_26) = _25;
(*_26) = _20;
_27 = _15;
_7 = _17 as u128;
(*_26) = _20;
(*_26) = _25;
_13 = [24881_u16,48362_u16,2883_u16];
_12 = _24;
(*_26) = _25;
_19 = !_22;
_4 = _3;
_1 = _16 as u64;
_31 = _21 + _21;
RET = core::ptr::addr_of_mut!(_34);
(*_26) = _20;
(*_26) = _25;
_15 = -_27;
(*_26) = _25;
(*_26) = _25;
(*_26) = _25;
_29 = [_16,_16,_16];
Goto(bb11)
}
bb11 = {
(*_26) = _25;
(*_26) = _25;
_19 = _9 + _22;
_15 = 1834638466634753543_i64 as f32;
(*_26) = _25;
RET = core::ptr::addr_of_mut!((*RET));
(*_26) = _20;
_22 = !_9;
(*_26) = _20;
_29 = [_16,_16,_16];
_15 = _27 - _3;
(*_26) = _25;
_5 = [_1,_1,_1,_1];
_7 = _16 as u128;
(*_26) = _20;
_29 = [_16,_16,_16];
match _16 {
0 => bb12,
6 => bb14,
_ => bb13
}
}
bb12 = {
_3 = (-193607503_i32) as f32;
_3 = (-6927214523957263164_i64) as f32;
_3 = 223_u8 as f32;
_4 = -_3;
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_1 = 7495960536126002728_u64;
_5 = [_1,_1,_1,_1];
_1 = 1681979838234961202_u64;
_1 = '\u{811bb}' as u64;
_5 = [_1,_1,_1,_1];
_6 = (-122357061_i32) | 2069189707_i32;
_4 = _3 + _3;
_5 = [_1,_1,_1,_1];
Goto(bb2)
}
bb13 = {
_17 = (-3528_i16) as i32;
_5 = [_1,_1,_1,_1];
_9 = !(-73_isize);
_17 = _6 << _6;
_21 = 2003954495_u32 & 4059919724_u32;
_15 = _3;
_4 = _15 * _15;
_15 = -_4;
Goto(bb5)
}
bb14 = {
_16 = 11307250049833865493_usize * 7_usize;
_9 = _16 as isize;
(*_26) = _25;
_7 = !287570035859577674638146055331095135487_u128;
_22 = !_9;
_16 = 4_usize * 5169481066407249286_usize;
(*_26) = _20;
(*_26) = _25;
(*_26) = _20;
(*_26) = _20;
(*_26) = _25;
_16 = 17277028605272073060_usize - 12461243844755465658_usize;
_36 = -_15;
(*_26) = _20;
(*_26) = _25;
_7 = _36 as u128;
_12 = _24;
_27 = _4 * _36;
(*_26) = _20;
(*_26) = _25;
Goto(bb15)
}
bb15 = {
_3 = _4 - _36;
_29 = [_16,_16,_16];
(*_26) = _25;
(*_26) = _20;
_31 = _21 >> _7;
(*_26) = _25;
_24 = _12;
(*_26) = _20;
(*_26) = _20;
(*_26) = _25;
_15 = _36 * _4;
_40 = (-31_i8) as u128;
_35 = (-5523_i16);
RET = core::ptr::addr_of_mut!((*RET));
_19 = !_9;
(*_26) = _20;
(*_26) = _25;
_15 = -_3;
Goto(bb16)
}
bb16 = {
_24 = _12 ^ _12;
_46 = _15 + _4;
_35 = 150231207487543861275964309593381371107_i128 as i16;
_39 = !_19;
_38 = (-22503088504666613982910958828085751313_i128) as f64;
_38 = _31 as f64;
_38 = _21 as f64;
_47 = core::ptr::addr_of_mut!(_31);
_41 = _35 | _35;
_15 = _3 - _27;
Goto(bb17)
}
bb17 = {
(*_47) = _21 >> _17;
_3 = _38 as f32;
(*_26) = _25;
_24 = _12;
_27 = -_4;
_3 = -_4;
(*_26) = _20;
_22 = -_19;
(*_47) = _21 & _21;
_38 = _1 as f64;
(*_47) = !_21;
(*_26) = _25;
_22 = _39;
Goto(bb18)
}
bb18 = {
(*_47) = _12 as u32;
(*_26) = _25;
_31 = _21;
_46 = _7 as f32;
(*_47) = _21 - _21;
(*_47) = _21 >> _17;
Goto(bb19)
}
bb19 = {
(*_47) = _21;
_47 = core::ptr::addr_of_mut!((*_47));
(*_47) = _21 << _19;
(*_47) = _21 + _21;
_9 = _22;
(*_26) = _20;
Goto(bb20)
}
bb20 = {
_52 = [_24,_12,_24,_12,_12,_12,_12];
(*_47) = _15 as u32;
_53 = _31 > (*_47);
_22 = _9 & _19;
_15 = _36;
_35 = _41;
(*_26) = _25;
(*_26) = _25;
(*_26) = _20;
Goto(bb21)
}
bb21 = {
_3 = _15;
_50 = _19;
_41 = !_35;
_31 = _17 as u32;
_13 = [38656_u16,21769_u16,20898_u16];
(*_47) = _21;
(*_26) = _20;
_55 = _16 as f64;
(*_47) = _21;
_24 = _53 & _53;
_5 = [_1,_1,_1,_1];
_39 = _7 as isize;
Goto(bb22)
}
bb22 = {
_38 = _55 + _55;
_38 = _6 as f64;
(*_47) = _21 * _21;
(*_47) = _21 & _21;
_31 = _21 * _21;
(*_26) = _25;
(*_26) = _25;
(*_47) = _21 | _21;
(*_47) = _21 | _21;
(*_47) = (-124724708086518585294387516181658399055_i128) as u32;
(*_26) = _25;
_39 = _9 + _22;
_41 = _35 ^ _35;
(*_26) = _25;
_31 = !_21;
_20 = (*_26);
(*_47) = _21;
_59 = _21 >> _19;
_53 = !_24;
_7 = (-4304866274234654473_i64) as u128;
Goto(bb23)
}
bb23 = {
(*_47) = _21 & _59;
_39 = _19;
(*_26) = _20;
_15 = _46;
_52 = [_24,_53,_24,_24,_24,_24,_24];
_12 = _53 < _24;
_41 = _35 >> (*_47);
(*_26) = _20;
_61 = &_40;
_58 = _55 + _55;
_25 = (*_26);
(*_26) = _20;
_54.0 = &mut _35;
_17 = _6 << (*_47);
(*_47) = _59 - _59;
_51 = _38 - _38;
_3 = _15 + _15;
_15 = -_3;
_9 = !_39;
_60 = _24 >= _53;
(*_47) = _21;
_20 = (*_26);
_58 = _51 - _51;
Goto(bb24)
}
bb24 = {
_60 = _12 >= _53;
_6 = _53 as i32;
_25 = (*_26);
(*_26) = _25;
_28 = _25;
Goto(bb25)
}
bb25 = {
_44 = 3_i8 as isize;
(*_26) = _20;
(*_47) = _59 + _59;
_65.0 = (*_47) - (*_47);
_26 = core::ptr::addr_of!((*_26));
_5 = [_1,_1,_1,_1];
(*_26) = _20;
_65 = ((*_47), _46, _12);
_26 = core::ptr::addr_of!((*_26));
(*_47) = _59 + _59;
(*_26) = _20;
_20 = (*_26);
_13 = [26081_u16,21412_u16,60446_u16];
(*_47) = _65.0 - _21;
_65.0 = (*_47) & (*_47);
_27 = -_15;
(*_26) = _25;
_62 = [(*_47),(*_47)];
(*_26) = _20;
(*_26) = _25;
_61 = &_7;
Call(_40 = core::intrinsics::transmute((*_61)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_47) = _59 + _65.0;
(*_47) = _65.0;
_49 = &_65;
_65 = ((*_47), _27, _53);
_20 = (*_26);
_41 = 14770_i16;
(*_47) = 6_i8 as u32;
_4 = -_27;
_63 = core::ptr::addr_of!((*_26));
_21 = (-103_i8) as u32;
_22 = _50;
(*_63) = _20;
_4 = _3 - _36;
(*_63) = _20;
(*_47) = !_59;
_50 = _44 * _19;
(*_47) = _58 as u32;
(*_63) = _25;
match _41 {
0 => bb27,
1 => bb28,
2 => bb29,
3 => bb30,
4 => bb31,
5 => bb32,
6 => bb33,
14770 => bb35,
_ => bb34
}
}
bb27 = {
_3 = _4 - _36;
_29 = [_16,_16,_16];
(*_26) = _25;
(*_26) = _20;
_31 = _21 >> _7;
(*_26) = _25;
_24 = _12;
(*_26) = _20;
(*_26) = _20;
(*_26) = _25;
_15 = _36 * _4;
_40 = (-31_i8) as u128;
_35 = (-5523_i16);
RET = core::ptr::addr_of_mut!((*RET));
_19 = !_9;
(*_26) = _20;
(*_26) = _25;
_15 = -_3;
Goto(bb16)
}
bb28 = {
_60 = _12 >= _53;
_6 = _53 as i32;
_25 = (*_26);
(*_26) = _25;
_28 = _25;
Goto(bb25)
}
bb29 = {
_6 = -_17;
_16 = 6497482687711094991_usize + 6548594662810908549_usize;
_16 = 6_usize;
_25 = _20;
_15 = -_3;
_26 = core::ptr::addr_of!(_25);
(*_26) = _20;
(*_26) = _20;
_22 = 4709_i16 as isize;
_20 = (*_26);
(*_26) = _20;
_17 = 54287_u16 as i32;
Goto(bb9)
}
bb30 = {
_26 = core::ptr::addr_of!((*_26));
(*_26) = _25;
(*_26) = _20;
_27 = _15;
_7 = _17 as u128;
(*_26) = _20;
(*_26) = _25;
_13 = [24881_u16,48362_u16,2883_u16];
_12 = _24;
(*_26) = _25;
_19 = !_22;
_4 = _3;
_1 = _16 as u64;
_31 = _21 + _21;
RET = core::ptr::addr_of_mut!(_34);
(*_26) = _20;
(*_26) = _25;
_15 = -_27;
(*_26) = _25;
(*_26) = _25;
(*_26) = _25;
_29 = [_16,_16,_16];
Goto(bb11)
}
bb31 = {
_5 = [_1,_1,_1,_1];
_6 = _17;
_15 = -_4;
_5 = [_1,_1,_1,_1];
_6 = _17 * _17;
_15 = _3 * _3;
Goto(bb8)
}
bb32 = {
_9 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_9 = 9223372036854775807_isize - (-9223372036854775808_isize);
_7 = 326826675756994275445043312463881496889_u128;
_5 = [_1,_1,_1,_1];
_1 = 14815747202271490239_u64;
_1 = !10967443289419482566_u64;
_3 = -_4;
_4 = _6 as f32;
_7 = 55398789076765101321787563943467056209_u128;
_13 = [41999_u16,60938_u16,55972_u16];
_9 = (-9223372036854775808_isize);
_1 = 9552773243903878926_u64;
_13 = [21866_u16,45241_u16,63661_u16];
_13 = [6525_u16,44928_u16,16660_u16];
_13 = [41177_u16,60547_u16,29182_u16];
_12 = false ^ false;
_13 = [62290_u16,60409_u16,36573_u16];
_1 = 7114757294389946728_u64 >> _9;
_12 = !false;
_7 = !237426603249735977580295923615982347724_u128;
_5 = [_1,_1,_1,_1];
_7 = 214857902458772051934398387541146595454_u128 - 307386318885936443619003028309094568555_u128;
_15 = 1607400218_u32 as f32;
_20 = '\u{a7655}';
Goto(bb4)
}
bb33 = {
(*_47) = _21;
_47 = core::ptr::addr_of_mut!((*_47));
(*_47) = _21 << _19;
(*_47) = _21 + _21;
_9 = _22;
(*_26) = _20;
Goto(bb20)
}
bb34 = {
_24 = _12 ^ _12;
_46 = _15 + _4;
_35 = 150231207487543861275964309593381371107_i128 as i16;
_39 = !_19;
_38 = (-22503088504666613982910958828085751313_i128) as f64;
_38 = _31 as f64;
_38 = _21 as f64;
_47 = core::ptr::addr_of_mut!(_31);
_41 = _35 | _35;
_15 = _3 - _27;
Goto(bb17)
}
bb35 = {
(*_63) = _20;
_71 = _46 * _3;
(*_26) = _25;
_69 = _1;
match _41 {
0 => bb12,
1 => bb9,
2 => bb26,
3 => bb27,
4 => bb36,
5 => bb37,
14770 => bb39,
_ => bb38
}
}
bb36 = {
_26 = core::ptr::addr_of!((*_26));
(*_26) = _25;
(*_26) = _20;
_27 = _15;
_7 = _17 as u128;
(*_26) = _20;
(*_26) = _25;
_13 = [24881_u16,48362_u16,2883_u16];
_12 = _24;
(*_26) = _25;
_19 = !_22;
_4 = _3;
_1 = _16 as u64;
_31 = _21 + _21;
RET = core::ptr::addr_of_mut!(_34);
(*_26) = _20;
(*_26) = _25;
_15 = -_27;
(*_26) = _25;
(*_26) = _25;
(*_26) = _25;
_29 = [_16,_16,_16];
Goto(bb11)
}
bb37 = {
_3 = _15;
_50 = _19;
_41 = !_35;
_31 = _17 as u32;
_13 = [38656_u16,21769_u16,20898_u16];
(*_47) = _21;
(*_26) = _20;
_55 = _16 as f64;
(*_47) = _21;
_24 = _53 & _53;
_5 = [_1,_1,_1,_1];
_39 = _7 as isize;
Goto(bb22)
}
bb38 = {
_3 = (-193607503_i32) as f32;
_3 = (-6927214523957263164_i64) as f32;
_3 = 223_u8 as f32;
_4 = -_3;
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_1 = 7495960536126002728_u64;
_5 = [_1,_1,_1,_1];
_1 = 1681979838234961202_u64;
_1 = '\u{811bb}' as u64;
_5 = [_1,_1,_1,_1];
_6 = (-122357061_i32) | 2069189707_i32;
_4 = _3 + _3;
_5 = [_1,_1,_1,_1];
Goto(bb2)
}
bb39 = {
_65.1 = -_36;
(*_26) = _20;
_70 = ((*_26),);
_15 = _71 + _3;
(*_26) = _70.0;
_23 = _41 as isize;
match _41 {
0 => bb36,
1 => bb40,
2 => bb41,
3 => bb42,
4 => bb43,
5 => bb44,
6 => bb45,
14770 => bb47,
_ => bb46
}
}
bb40 = {
_9 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_9 = 9223372036854775807_isize - (-9223372036854775808_isize);
_7 = 326826675756994275445043312463881496889_u128;
_5 = [_1,_1,_1,_1];
_1 = 14815747202271490239_u64;
_1 = !10967443289419482566_u64;
_3 = -_4;
_4 = _6 as f32;
_7 = 55398789076765101321787563943467056209_u128;
_13 = [41999_u16,60938_u16,55972_u16];
_9 = (-9223372036854775808_isize);
_1 = 9552773243903878926_u64;
_13 = [21866_u16,45241_u16,63661_u16];
_13 = [6525_u16,44928_u16,16660_u16];
_13 = [41177_u16,60547_u16,29182_u16];
_12 = false ^ false;
_13 = [62290_u16,60409_u16,36573_u16];
_1 = 7114757294389946728_u64 >> _9;
_12 = !false;
_7 = !237426603249735977580295923615982347724_u128;
_5 = [_1,_1,_1,_1];
_7 = 214857902458772051934398387541146595454_u128 - 307386318885936443619003028309094568555_u128;
_15 = 1607400218_u32 as f32;
_20 = '\u{a7655}';
Goto(bb4)
}
bb41 = {
_5 = [_1,_1,_1,_1];
Goto(bb7)
}
bb42 = {
_26 = core::ptr::addr_of!((*_26));
(*_26) = _25;
(*_26) = _20;
_27 = _15;
_7 = _17 as u128;
(*_26) = _20;
(*_26) = _25;
_13 = [24881_u16,48362_u16,2883_u16];
_12 = _24;
(*_26) = _25;
_19 = !_22;
_4 = _3;
_1 = _16 as u64;
_31 = _21 + _21;
RET = core::ptr::addr_of_mut!(_34);
(*_26) = _20;
(*_26) = _25;
_15 = -_27;
(*_26) = _25;
(*_26) = _25;
(*_26) = _25;
_29 = [_16,_16,_16];
Goto(bb11)
}
bb43 = {
(*_26) = _25;
(*_26) = _25;
_19 = _9 + _22;
_15 = 1834638466634753543_i64 as f32;
(*_26) = _25;
RET = core::ptr::addr_of_mut!((*RET));
(*_26) = _20;
_22 = !_9;
(*_26) = _20;
_29 = [_16,_16,_16];
_15 = _27 - _3;
(*_26) = _25;
_5 = [_1,_1,_1,_1];
_7 = _16 as u128;
(*_26) = _20;
_29 = [_16,_16,_16];
match _16 {
0 => bb12,
6 => bb14,
_ => bb13
}
}
bb44 = {
_44 = 3_i8 as isize;
(*_26) = _20;
(*_47) = _59 + _59;
_65.0 = (*_47) - (*_47);
_26 = core::ptr::addr_of!((*_26));
_5 = [_1,_1,_1,_1];
(*_26) = _20;
_65 = ((*_47), _46, _12);
_26 = core::ptr::addr_of!((*_26));
(*_47) = _59 + _59;
(*_26) = _20;
_20 = (*_26);
_13 = [26081_u16,21412_u16,60446_u16];
(*_47) = _65.0 - _21;
_65.0 = (*_47) & (*_47);
_27 = -_15;
(*_26) = _25;
_62 = [(*_47),(*_47)];
(*_26) = _20;
(*_26) = _25;
_61 = &_7;
Call(_40 = core::intrinsics::transmute((*_61)), ReturnTo(bb26), UnwindUnreachable())
}
bb45 = {
_5 = [_1,_1,_1,_1];
_6 = _17;
_15 = -_4;
_5 = [_1,_1,_1,_1];
_6 = _17 * _17;
_15 = _3 * _3;
Goto(bb8)
}
bb46 = {
_3 = (-193607503_i32) as f32;
_3 = (-6927214523957263164_i64) as f32;
_3 = 223_u8 as f32;
_4 = -_3;
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_4 = _3 * _3;
_5 = [_1,_1,_1,_1];
_1 = 7495960536126002728_u64;
_5 = [_1,_1,_1,_1];
_1 = 1681979838234961202_u64;
_1 = '\u{811bb}' as u64;
_5 = [_1,_1,_1,_1];
_6 = (-122357061_i32) | 2069189707_i32;
_4 = _3 + _3;
_5 = [_1,_1,_1,_1];
Goto(bb2)
}
bb47 = {
_4 = (-10_i8) as f32;
(*_47) = _59 << _6;
(*_26) = _25;
_63 = core::ptr::addr_of!((*_26));
(*_63) = _20;
(*_26) = _25;
_7 = _40 + _40;
_36 = _71;
(*_26) = _70.0;
_68 = &_70;
_23 = _22;
_27 = _22 as f32;
_16 = 1826003202548011814_usize - 14245469054898480779_usize;
_13 = [28462_u16,2980_u16,58851_u16];
(*_47) = !_59;
_78 = &_40;
_69 = _1 - _1;
_28 = (*_68).0;
_3 = -_36;
_72 = &mut _52;
_76.0 = (*_26);
_58 = 5712825717280209008_i64 as f64;
match _41 {
0 => bb13,
14770 => bb49,
_ => bb48
}
}
bb48 = {
(*_47) = _21;
_47 = core::ptr::addr_of_mut!((*_47));
(*_47) = _21 << _19;
(*_47) = _21 + _21;
_9 = _22;
(*_26) = _20;
Goto(bb20)
}
bb49 = {
_76.1 = Adt19::Variant1 { fld0: 41_i8,fld1: 49163137727178110563670079992538666856_i128,fld2: _6 };
(*_26) = (*_68).0;
(*_26) = (*_68).0;
(*_47) = _55 as u32;
_19 = -_39;
(*_26) = _20;
(*_72) = [_60,_12,_12,_53,_60,_12,_24];
(*_47) = (-1906635119274974298_i64) as u32;
_71 = _3 + _36;
_13 = [14117_u16,25838_u16,46028_u16];
_73 = core::ptr::addr_of_mut!((*RET));
(*_26) = _76.0;
_6 = Field::<i32>(Variant(_76.1, 1), 2);
_73 = core::ptr::addr_of_mut!((*_73));
(*_72) = [_65.2,_12,_12,_12,_60,_60,_12];
(*_72) = [_53,_60,_12,_12,_60,_60,_24];
(*_47) = _6 as u32;
_20 = (*_26);
_39 = _23 * _50;
(*_72) = [_12,_53,_12,_53,_53,_60,_12];
_38 = _36 as f64;
Goto(bb50)
}
bb50 = {
Call(_84 = dump_var(Move(_70), Move(_40), Move(_35), Move(_50)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_84 = dump_var(Move(_31), Move(_22), Move(_41), Move(_9)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_84 = dump_var(Move(_6), Move(_29), Move(_39), Move(_21)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_84 = dump_var(Move(_16), Move(_62), Move(_20), Move(_52)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *const &'static &'static mut bool,mut _2: &'static &'static mut bool) -> *mut u32 {
mir! {
type RET = *mut u32;
let _3: Adt25;
let _4: [i16; 2];
let _5: [u64; 6];
let _6: (char,);
let _7: bool;
let _8: *const char;
let _9: [u64; 4];
let _10: Adt27;
let _11: (char, *const u128, (char, Adt19), *mut i32);
let _12: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _13: [u8; 6];
let _14: u128;
let _15: *mut i128;
let _16: *const [u8; 7];
let _17: char;
let _18: u128;
let _19: f32;
let _20: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _21: f64;
let _22: f32;
let _23: (f64,);
let _24: Adt25;
let _25: &'static mut i16;
let _26: i16;
let _27: Adt78;
let _28: *const &'static mut i16;
let _29: [u128; 6];
let _30: isize;
let _31: (char, Adt19);
let _32: i16;
let _33: u32;
let _34: *const [i16; 2];
let _35: isize;
let _36: bool;
let _37: char;
let _38: isize;
let _39: u16;
let _40: isize;
let _41: f64;
let _42: isize;
let _43: bool;
let _44: f64;
let _45: [i64; 4];
let _46: isize;
let _47: bool;
let _48: isize;
let _49: u128;
let _50: char;
let _51: &'static mut bool;
let _52: char;
let _53: &'static Adt19;
let _54: isize;
let _55: &'static mut i16;
let _56: isize;
let _57: &'static &'static mut *mut i32;
let _58: (*mut [u64; 5], &'static mut bool, &'static mut bool, &'static mut &'static mut bool);
let _59: f32;
let _60: *const u128;
let _61: Adt45;
let _62: &'static Adt19;
let _63: &'static mut [i128; 6];
let _64: *const &'static &'static mut bool;
let _65: &'static (u32, f32, bool);
let _66: *const &'static mut i16;
let _67: isize;
let _68: i32;
let _69: &'static mut [i128; 6];
let _70: &'static u128;
let _71: char;
let _72: &'static mut *mut i32;
let _73: f32;
let _74: f32;
let _75: [isize; 6];
let _76: char;
let _77: (u8, usize, i128, i128);
let _78: *mut i32;
let _79: [char; 2];
let _80: u32;
let _81: i128;
let _82: &'static mut *mut i32;
let _83: u128;
let _84: isize;
let _85: isize;
let _86: Adt46;
let _87: *const u16;
let _88: &'static mut *mut [u64; 5];
let _89: ((char, Adt19),);
let _90: bool;
let _91: u64;
let _92: *mut [u64; 5];
let _93: f32;
let _94: isize;
let _95: f64;
let _96: isize;
let _97: *const u128;
let _98: Adt45;
let _99: f32;
let _100: *const [i16; 2];
let _101: isize;
let _102: i16;
let _103: *const *mut i32;
let _104: (&'static mut i16, *mut [u64; 5]);
let _105: ();
let _106: ();
{
_3.fld3 = (-98_i8) & (-71_i8);
_3.fld1 = (-7953280396725538596_i64) as f32;
_3.fld5 = !640623442_u32;
_3.fld2 = !1_usize;
_3.fld2 = 3_usize;
match _3.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
_ => bb5
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
_3.fld0 = true;
_3.fld3 = 38_i8 << _3.fld5;
_3.fld2 = 7_usize ^ 0_usize;
_3.fld3 = 28_i8;
RET = core::ptr::addr_of_mut!(_3.fld5);
_4 = [(-7395_i16),10291_i16];
_4 = [31516_i16,19790_i16];
(*RET) = !1877361674_u32;
(*RET) = _3.fld1 as u32;
(*RET) = 450895774_u32 * 1008967881_u32;
_5 = [6942298977549351130_u64,11433222070571038407_u64,13547893507993714943_u64,16120218271060572350_u64,15416625803392786845_u64,457335424964545565_u64];
(*RET) = 2015455440_u32 | 993135941_u32;
(*RET) = !1649518340_u32;
(*RET) = 2540835542_u32 ^ 4083698555_u32;
_6 = ('\u{2de76}',);
(*RET) = 1817692841_u32 - 3163945610_u32;
_3.fld5 = 12207396262127327519_u64 as u32;
Goto(bb7)
}
bb7 = {
_3.fld2 = 1144970427884525945_usize;
RET = core::ptr::addr_of_mut!((*RET));
_6 = ('\u{66709}',);
match _3.fld2 {
0 => bb8,
1 => bb9,
1144970427884525945 => bb11,
_ => bb10
}
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
(*RET) = 1757113669_u32;
(*RET) = 1073728211_u32;
_3.fld3 = !16_i8;
(*RET) = 3653037345_u32 * 510312244_u32;
(*RET) = 4269058133_u32;
(*RET) = 1465549555_u32 >> _3.fld2;
(*RET) = (-865540097_i32) as u32;
(*RET) = 9653884898127522055_u64 as u32;
_4 = [(-25813_i16),(-30454_i16)];
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 4154648421_u32 & 3551016345_u32;
(*RET) = 6390190288413853029_i64 as u32;
_7 = _3.fld0 < _3.fld0;
(*RET) = 1848928872_u32 | 613871506_u32;
_7 = !_3.fld0;
_3.fld0 = (*RET) == (*RET);
Goto(bb12)
}
bb12 = {
_3.fld5 = 4128593907_u32;
_3.fld2 = _3.fld3 as usize;
_3.fld4 = !13220_u16;
(*RET) = 3131309327_u32;
_8 = core::ptr::addr_of!(_6.0);
_3.fld0 = _7 | _7;
_3.fld5 = 577788470_u32 & 1764436844_u32;
_5 = [275029002491233380_u64,11484949182781231512_u64,9027439454718466887_u64,8164481969983830294_u64,17731572691263052094_u64,3075237836356290465_u64];
(*RET) = 738177482_u32 & 8733997_u32;
(*_8) = '\u{d246e}';
(*RET) = 3162793916_u32 | 1266287815_u32;
(*_8) = '\u{49caa}';
_3.fld4 = 37827_u16 - 16832_u16;
(*RET) = 2899370092_u32 ^ 3806073777_u32;
(*_8) = '\u{7f80a}';
(*_8) = '\u{ae7e3}';
(*_8) = '\u{e5a8a}';
(*RET) = 2923732666_u32 | 397550689_u32;
_7 = !_3.fld0;
_3.fld1 = _3.fld2 as f32;
(*RET) = 2651827515_u32 | 4234192209_u32;
(*RET) = 3862460516_u32 * 4079227932_u32;
(*RET) = !4189576829_u32;
(*RET) = 1206475531_u32 * 2050335637_u32;
_3.fld0 = (*_8) != (*_8);
(*RET) = 3329723429_u32 >> _3.fld3;
(*RET) = 69647844_u32 - 1181901570_u32;
(*_8) = '\u{428f4}';
Goto(bb13)
}
bb13 = {
(*_8) = '\u{e4f2d}';
(*_8) = '\u{6a47b}';
(*RET) = !1370547655_u32;
_3.fld0 = !_7;
(*RET) = 3405317606_u32 << _3.fld2;
(*RET) = 2141909462_u32 ^ 3573119656_u32;
(*RET) = 3245299998_u32;
(*_8) = '\u{10f7f9}';
_3.fld1 = 57760327768093526997234813488755689097_u128 as f32;
(*_8) = '\u{f8be3}';
(*RET) = (-58204100385653350627264803379272309471_i128) as u32;
_8 = core::ptr::addr_of!((*_8));
Goto(bb14)
}
bb14 = {
_3.fld5 = 516093392_i32 as u32;
_12.1 = &mut _3.fld0;
(*_8) = '\u{3773f}';
_12.2 = &mut _7;
_13 = [10_u8,201_u8,239_u8,62_u8,175_u8,236_u8];
(*RET) = !4232430105_u32;
(*RET) = 3575117827_u32 & 1701880278_u32;
_8 = core::ptr::addr_of!((*_8));
(*RET) = 1677803483611638610_u64 as u32;
(*RET) = 2739529154_u32;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 4201374664_u32;
_12.2 = Move(_12.1);
Goto(bb15)
}
bb15 = {
_11.0 = (*_8);
match (*RET) {
0 => bb16,
4201374664 => bb18,
_ => bb17
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_13 = [206_u8,45_u8,187_u8,113_u8,219_u8,232_u8];
_4 = [(-16714_i16),30298_i16];
(*_8) = _11.0;
(*RET) = 1779734659_u32 << (-6396307681623366037_i64);
_6.0 = _11.0;
(*_8) = _11.0;
Goto(bb19)
}
bb19 = {
_18 = 333649409067903526834872045972240891957_u128 << (*RET);
(*_8) = _11.0;
(*RET) = !1602709477_u32;
(*RET) = 1763889198_u32;
_19 = (-571598518_i32) as f32;
(*_8) = _11.0;
(*_8) = _11.0;
(*RET) = 103_i8 as u32;
(*RET) = !165876115_u32;
_24.fld1 = _18 as f32;
Goto(bb20)
}
bb20 = {
(*RET) = !892891085_u32;
(*RET) = 588351439_u32 * 1997275375_u32;
_24.fld0 = (*RET) == (*RET);
RET = core::ptr::addr_of_mut!((*RET));
_24.fld5 = !(*RET);
_23.0 = 35785_u16 as f64;
_20.2 = &mut _24.fld0;
(*_8) = _11.0;
Goto(bb21)
}
bb21 = {
(*_8) = _11.0;
_20.3 = &mut _20.2;
(*_8) = _11.0;
(*RET) = 2588085401_u32 << _18;
(*RET) = 3318510768_u32 & 3036303153_u32;
_11.2.1 = Adt19::Variant1 { fld0: 1_i8,fld1: (-89222960078027389758410468372067392043_i128),fld2: (-1153980772_i32) };
(*_8) = _11.0;
(*RET) = 1263905291_u32 << _18;
_14 = false as u128;
(*_8) = _11.0;
Call((*RET) = fn19(Move(RET), Move(_1), (*_8), (*_8), (*_8), Move(_8), (*_8), (*_8), (*_8), _5, (*_8), (*_8)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
place!(Field::<i32>(Variant(_11.2.1, 1), 2)) = (-185424439_i32);
place!(Field::<i8>(Variant(_11.2.1, 1), 0)) = -(-51_i8);
_11.1 = core::ptr::addr_of!(_18);
_8 = core::ptr::addr_of!(_6.0);
_22 = -_19;
place!(Field::<i8>(Variant(_11.2.1, 1), 0)) = 23_i8;
_17 = (*_8);
place!(Field::<i8>(Variant(_11.2.1, 1), 0)) = (-33_i8) & 28_i8;
_5 = [3557252833735490081_u64,3573729402346080104_u64,441159671095896758_u64,5168293664013708829_u64,13793237730740303225_u64,2884871421221767485_u64];
_26 = (-18528_i16);
_8 = core::ptr::addr_of!((*_8));
(*_8) = _17;
place!(Field::<i128>(Variant(_11.2.1, 1), 1)) = !(-161853111566084993713590568522253233170_i128);
_11.1 = core::ptr::addr_of!(_18);
_21 = _23.0 + _23.0;
match _26 {
0 => bb11,
1 => bb16,
2 => bb3,
3 => bb10,
4 => bb8,
5 => bb6,
6 => bb18,
340282366920938463463374607431768192928 => bb24,
_ => bb23
}
}
bb23 = {
(*RET) = 1757113669_u32;
(*RET) = 1073728211_u32;
_3.fld3 = !16_i8;
(*RET) = 3653037345_u32 * 510312244_u32;
(*RET) = 4269058133_u32;
(*RET) = 1465549555_u32 >> _3.fld2;
(*RET) = (-865540097_i32) as u32;
(*RET) = 9653884898127522055_u64 as u32;
_4 = [(-25813_i16),(-30454_i16)];
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 4154648421_u32 & 3551016345_u32;
(*RET) = 6390190288413853029_i64 as u32;
_7 = _3.fld0 < _3.fld0;
(*RET) = 1848928872_u32 | 613871506_u32;
_7 = !_3.fld0;
_3.fld0 = (*RET) == (*RET);
Goto(bb12)
}
bb24 = {
_31 = ((*_8), Move(_11.2.1));
_11.2.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_31.1, 1), 0),fld1: Field::<i128>(Variant(_31.1, 1), 1),fld2: Field::<i32>(Variant(_31.1, 1), 2) };
_31 = ((*_8), Move(_11.2.1));
place!(Field::<i8>(Variant(_31.1, 1), 0)) = !69_i8;
_8 = core::ptr::addr_of!((*_8));
(*_8) = _17;
(*_8) = _17;
(*_8) = _17;
RET = core::ptr::addr_of_mut!(_33);
_32 = _26 | _26;
_15 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_31.1, 1), 1)));
(*_8) = _17;
(*RET) = 695975111_u32 - 2202304325_u32;
(*_15) = 26833281041133488863667023215567949890_i128 & 44644528416785827554470012663125983508_i128;
(*_8) = _17;
_11.2.1 = Move(_31.1);
_11.2.0 = (*_8);
_29 = [_18,_18,_14,_14,_18,_18];
(*RET) = 2565933649_u32 >> Field::<i32>(Variant(_11.2.1, 1), 2);
_22 = 42106_u16 as f32;
_31.0 = (*_8);
(*RET) = 3397639059_u32 | 3619158623_u32;
(*_15) = Field::<i32>(Variant(_11.2.1, 1), 2) as i128;
(*_15) = Field::<i128>(Variant(_11.2.1, 1), 1) << (*RET);
(*_8) = _17;
match _26 {
340282366920938463463374607431768192928 => bb25,
_ => bb5
}
}
bb25 = {
_36 = false | true;
(*_15) = Field::<i128>(Variant(_11.2.1, 1), 1);
(*_15) = 9223372036854775807_isize as i128;
_31 = ((*_8), Move(_11.2.1));
Goto(bb26)
}
bb26 = {
_38 = (*RET) as isize;
_11.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_31.1, 1), 2)));
_4 = [_32,_32];
(*_8) = _11.0;
Goto(bb27)
}
bb27 = {
_39 = !38037_u16;
_11.2.1 = Adt19::Variant0 { fld0: _36,fld1: 53_u8,fld2: _38,fld3: _23,fld4: 5_usize,fld5: _39 };
(*_8) = _11.2.0;
_38 = !Field::<isize>(Variant(_11.2.1, 0), 2);
(*RET) = !2141144142_u32;
_26 = _32 >> (*RET);
(*RET) = 1857954795_u32 + 2783965954_u32;
(*RET) = 3889523912_u32 + 2288453814_u32;
(*_8) = _31.0;
(*RET) = 3161823837_u32 + 3407262294_u32;
_10 = Adt27::Variant0 { fld0: Move(_11.1),fld1: Move(_11.3),fld2: _38,fld3: Field::<i8>(Variant(_31.1, 1), 0),fld4: 206_u8,fld5: Field::<i32>(Variant(_31.1, 1), 2),fld6: (*RET) };
(*RET) = !Field::<u32>(Variant(_10, 0), 6);
_31.0 = (*_8);
_9 = [9531533705661631261_u64,4847310523818806352_u64,2135162295887570959_u64,2866670232323273127_u64];
_11.0 = (*_8);
(*RET) = Field::<u32>(Variant(_10, 0), 6) << _18;
(*RET) = _32 as u32;
place!(Field::<u8>(Variant(_10, 0), 4)) = 69_u8 & 142_u8;
(*_8) = _11.0;
place!(Field::<i32>(Variant(_31.1, 1), 2)) = -Field::<i32>(Variant(_10, 0), 5);
place!(Field::<isize>(Variant(_11.2.1, 0), 2)) = (*_8) as isize;
_14 = _18 << (*RET);
Goto(bb28)
}
bb28 = {
place!(Field::<isize>(Variant(_11.2.1, 0), 2)) = _38;
_11 = ((*_8), Move(Field::<*const u128>(Variant(_10, 0), 0)), Move(_31), Move(Field::<*mut i32>(Variant(_10, 0), 1)));
_9 = [7760693931232315606_u64,11862567106556927746_u64,16713944148624419107_u64,1206399585868037122_u64];
_31.1 = Adt19::Variant0 { fld0: _36,fld1: Field::<u8>(Variant(_10, 0), 4),fld2: Field::<isize>(Variant(_10, 0), 2),fld3: _23,fld4: 1_usize,fld5: _39 };
_5 = [10253615446521988656_u64,3679619290533340986_u64,11826110001474241774_u64,10722002647631076517_u64,10249686684428218386_u64,5130470822409569574_u64];
_8 = core::ptr::addr_of!((*_8));
_41 = _23.0 * _21;
_29 = [_14,_18,_14,_18,_14,_14];
_18 = Field::<u8>(Variant(_10, 0), 4) as u128;
place!(Field::<u32>(Variant(_10, 0), 6)) = (*RET) | (*RET);
match Field::<i32>(Variant(_10, 0), 5) {
0 => bb8,
1 => bb20,
2 => bb6,
340282366920938463463374607431582787017 => bb29,
_ => bb23
}
}
bb29 = {
(*_8) = _11.2.0;
place!(Field::<i128>(Variant(_11.2.1, 1), 1)) = !(-111721069141325967234554918522031768156_i128);
_31.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_11.2.1, 1), 0),fld1: Field::<i128>(Variant(_11.2.1, 1), 1),fld2: Field::<i32>(Variant(_11.2.1, 1), 2) };
place!(Field::<*mut i32>(Variant(_10, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11.2.1, 1), 2)));
(*_8) = _17;
_8 = core::ptr::addr_of!((*_8));
_5 = [9892798661119294971_u64,2716604932297544966_u64,12960879795231437622_u64,2586857892709906251_u64,16638149723278513042_u64,10044655316703090755_u64];
_12.1 = &mut _36;
_31.0 = (*_8);
_31.1 = Move(_11.2.1);
_9 = [14386235341510458263_u64,17776640112963301778_u64,3544810580292444368_u64,16770435500122249135_u64];
_6 = (_17,);
_46 = _38;
place!(Field::<i8>(Variant(_10, 0), 3)) = Field::<i8>(Variant(_31.1, 1), 0);
place!(Field::<u8>(Variant(_10, 0), 4)) = 10927788511105547067_u64 as u8;
place!(Field::<i32>(Variant(_10, 0), 5)) = Field::<i32>(Variant(_31.1, 1), 2) | Field::<i32>(Variant(_31.1, 1), 2);
Goto(bb30)
}
bb30 = {
_11.2.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_31.1, 1), 0),fld1: Field::<i128>(Variant(_31.1, 1), 1),fld2: Field::<i32>(Variant(_10, 0), 5) };
_12.2 = Move(_12.1);
_49 = true as u128;
(*_8) = _11.2.0;
_48 = 1984701677652590362_u64 as isize;
_34 = core::ptr::addr_of!(_4);
place!(Field::<i128>(Variant(_31.1, 1), 1)) = Field::<i128>(Variant(_11.2.1, 1), 1);
(*_8) = _11.0;
_47 = (*_8) >= (*_8);
_17 = (*_8);
_11.2 = ((*_8), Move(_31.1));
_34 = core::ptr::addr_of!(_4);
(*_8) = _11.0;
_52 = _11.2.0;
_31.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_10, 0), 3),fld1: Field::<i128>(Variant(_11.2.1, 1), 1),fld2: Field::<i32>(Variant(_10, 0), 5) };
(*_34) = [_26,_26];
(*_34) = [_32,_26];
_34 = core::ptr::addr_of!((*_34));
_23 = (_41,);
(*_34) = [_32,_26];
_10 = Adt27::Variant3 { fld0: Move(_8),fld1: (-2244617243166166923_i64) };
(*RET) = _47 as u32;
_40 = _48 | _38;
(*RET) = 1827327783_u32;
match (*RET) {
0 => bb15,
1 => bb14,
2 => bb27,
3 => bb31,
1827327783 => bb33,
_ => bb32
}
}
bb31 = {
(*_8) = _11.2.0;
place!(Field::<i128>(Variant(_11.2.1, 1), 1)) = !(-111721069141325967234554918522031768156_i128);
_31.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_11.2.1, 1), 0),fld1: Field::<i128>(Variant(_11.2.1, 1), 1),fld2: Field::<i32>(Variant(_11.2.1, 1), 2) };
place!(Field::<*mut i32>(Variant(_10, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11.2.1, 1), 2)));
(*_8) = _17;
_8 = core::ptr::addr_of!((*_8));
_5 = [9892798661119294971_u64,2716604932297544966_u64,12960879795231437622_u64,2586857892709906251_u64,16638149723278513042_u64,10044655316703090755_u64];
_12.1 = &mut _36;
_31.0 = (*_8);
_31.1 = Move(_11.2.1);
_9 = [14386235341510458263_u64,17776640112963301778_u64,3544810580292444368_u64,16770435500122249135_u64];
_6 = (_17,);
_46 = _38;
place!(Field::<i8>(Variant(_10, 0), 3)) = Field::<i8>(Variant(_31.1, 1), 0);
place!(Field::<u8>(Variant(_10, 0), 4)) = 10927788511105547067_u64 as u8;
place!(Field::<i32>(Variant(_10, 0), 5)) = Field::<i32>(Variant(_31.1, 1), 2) | Field::<i32>(Variant(_31.1, 1), 2);
Goto(bb30)
}
bb32 = {
place!(Field::<isize>(Variant(_11.2.1, 0), 2)) = _38;
_11 = ((*_8), Move(Field::<*const u128>(Variant(_10, 0), 0)), Move(_31), Move(Field::<*mut i32>(Variant(_10, 0), 1)));
_9 = [7760693931232315606_u64,11862567106556927746_u64,16713944148624419107_u64,1206399585868037122_u64];
_31.1 = Adt19::Variant0 { fld0: _36,fld1: Field::<u8>(Variant(_10, 0), 4),fld2: Field::<isize>(Variant(_10, 0), 2),fld3: _23,fld4: 1_usize,fld5: _39 };
_5 = [10253615446521988656_u64,3679619290533340986_u64,11826110001474241774_u64,10722002647631076517_u64,10249686684428218386_u64,5130470822409569574_u64];
_8 = core::ptr::addr_of!((*_8));
_41 = _23.0 * _21;
_29 = [_14,_18,_14,_18,_14,_14];
_18 = Field::<u8>(Variant(_10, 0), 4) as u128;
place!(Field::<u32>(Variant(_10, 0), 6)) = (*RET) | (*RET);
match Field::<i32>(Variant(_10, 0), 5) {
0 => bb8,
1 => bb20,
2 => bb6,
340282366920938463463374607431582787017 => bb29,
_ => bb23
}
}
bb33 = {
_52 = _31.0;
(*RET) = _47 as u32;
_50 = _31.0;
(*RET) = 7569527_u32;
_9 = [14124822469727174686_u64,4504038699267921130_u64,7852139157701752101_u64,12989296139717974945_u64];
_37 = _50;
_11.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11.2.1, 1), 2)));
Goto(bb34)
}
bb34 = {
_30 = _46 * _40;
(*_34) = [_32,_32];
(*RET) = Field::<i8>(Variant(_11.2.1, 1), 0) as u32;
_56 = _30 ^ _40;
_45 = [8808021758330676349_i64,5704813680166637260_i64,2856396350522749721_i64,190443680059510337_i64];
_56 = _40 + _40;
_8 = Move(Field::<*const char>(Variant(_10, 3), 0));
_11.1 = core::ptr::addr_of!(_18);
_42 = _30;
_38 = -_42;
place!(Field::<i64>(Variant(_10, 3), 1)) = !(-5846624042549727731_i64);
place!(Field::<*const char>(Variant(_10, 3), 0)) = core::ptr::addr_of!(_11.0);
_60 = Move(_11.1);
Goto(bb35)
}
bb35 = {
_38 = -_56;
_48 = _42 << _42;
_47 = _22 >= _22;
(*_34) = [_26,_26];
(*_34) = [_32,_32];
(*_34) = [_26,_26];
(*_34) = [_26,_32];
_12.2 = &mut _47;
_22 = -_19;
place!(Field::<i8>(Variant(_11.2.1, 1), 0)) = false as i8;
_58.1 = Move(_12.2);
(*_34) = [_32,_32];
(*RET) = !3324881717_u32;
_55 = &mut _32;
(*RET) = 395576235_u32 >> (*_55);
_22 = _19 - _19;
(*_55) = _26 * _26;
_54 = !_56;
_67 = !_30;
_44 = _23.0 * _41;
(*_55) = _11.2.0 as i16;
RET = core::ptr::addr_of_mut!((*RET));
_15 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_11.2.1, 1), 1)));
Call((*_34) = core::intrinsics::transmute((*RET)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
_35 = -_48;
_6 = (_37,);
_46 = _56 - _42;
(*_34) = [(*_55),(*_55)];
_56 = _22 as isize;
_23 = (_41,);
(*RET) = _56 as u32;
Goto(bb37)
}
bb37 = {
Goto(bb38)
}
bb38 = {
place!(Field::<i32>(Variant(_11.2.1, 1), 2)) = Field::<i32>(Variant(_31.1, 1), 2) >> _42;
_50 = _52;
(*RET) = 1_usize as u32;
(*_55) = _26 - _26;
_68 = Field::<i32>(Variant(_11.2.1, 1), 2) & Field::<i32>(Variant(_11.2.1, 1), 2);
_23 = (_44,);
_17 = _37;
(*RET) = 1152759496_u32;
_11.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11.2.1, 1), 2)));
_45 = [Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1)];
_38 = !_56;
(*RET) = !639929051_u32;
_67 = _38 | _48;
(*_55) = _26;
_74 = _19 - _19;
(*_34) = [_26,(*_55)];
(*_34) = [(*_55),(*_55)];
(*_55) = _26 + _26;
(*RET) = 1953387307_u32 << (*_55);
(*RET) = 3871681233_u32 | 1905106272_u32;
(*_34) = [(*_55),(*_55)];
Goto(bb39)
}
bb39 = {
(*_15) = Field::<i128>(Variant(_31.1, 1), 1);
_5 = [8384559503129777605_u64,16505002492642112462_u64,11682670396134614749_u64,13988257033942985649_u64,2288083747784903764_u64,6100329667543998445_u64];
(*_15) = _22 as i128;
_75 = [_48,_38,_46,_54,_48,_48];
(*RET) = 2648263760_u32;
(*_34) = [(*_55),(*_55)];
_37 = _17;
place!(Field::<i128>(Variant(_31.1, 1), 1)) = (*_15);
(*_15) = Field::<i128>(Variant(_31.1, 1), 1) >> _30;
(*RET) = 2532336355_u32 + 39082746_u32;
_22 = _74 - _74;
(*_15) = Field::<i8>(Variant(_11.2.1, 1), 0) as i128;
(*_15) = !Field::<i128>(Variant(_31.1, 1), 1);
(*_15) = -Field::<i128>(Variant(_31.1, 1), 1);
_78 = Move(_11.3);
_59 = -_19;
(*_55) = _26 >> _54;
_29 = [_14,_14,_14,_18,_14,_14];
_72 = &mut _78;
(*_72) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_11.2.1, 1), 2)));
_31.1 = Adt19::Variant0 { fld0: false,fld1: 180_u8,fld2: _38,fld3: _23,fld4: 10353767417810647724_usize,fld5: _39 };
_53 = &_11.2.1;
Goto(bb40)
}
bb40 = {
_11.2.0 = _6.0;
place!(Field::<usize>(Variant(_31.1, 0), 4)) = 3547154607210954394_usize << _68;
(*_34) = [(*_55),(*_55)];
(*_72) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant((*_53), 1), 2)));
_79 = [_11.2.0,_11.2.0];
_33 = !2761606952_u32;
place!(Field::<i8>(Variant(_11.2.1, 1), 0)) = 123_i8 | (-7_i8);
(*_72) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant((*_53), 1), 2)));
_40 = _48 | _38;
_59 = _14 as f32;
_23.0 = -Field::<(f64,)>(Variant(_31.1, 0), 3).0;
_31.0 = _11.2.0;
_57 = &_72;
(*_15) = _59 as i128;
_27 = Adt78::Variant2 { fld0: Move(_11.2),fld1: _13 };
(*_72) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)).1, 1), 2)));
(*_55) = _26 - _26;
_80 = _18 as u32;
_46 = _54 - _48;
(*_15) = Field::<i128>(Variant(Field::<(char, Adt19)>(Variant(_27, 2), 0).1, 1), 1) - Field::<i128>(Variant(Field::<(char, Adt19)>(Variant(_27, 2), 0).1, 1), 1);
(*RET) = _80 >> (*_15);
_56 = _48;
_56 = _40 >> (*RET);
_71 = _37;
_62 = &place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)).1;
_41 = Field::<i64>(Variant(_10, 3), 1) as f64;
_35 = _44 as isize;
Goto(bb41)
}
bb41 = {
_31.1 = Adt19::Variant0 { fld0: false,fld1: 118_u8,fld2: _40,fld3: _23,fld4: 3_usize,fld5: _39 };
_70 = &_49;
(*_72) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)).1, 1), 2)));
(*RET) = Field::<i64>(Variant(_10, 3), 1) as u32;
_54 = (*_70) as isize;
_11.2.0 = _50;
_31 = Move(Field::<(char, Adt19)>(Variant(_27, 2), 0));
(*_72) = core::ptr::addr_of_mut!(_68);
_66 = core::ptr::addr_of!(_55);
place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)).0 = _17;
_86.fld0 = [16508649919987126884_u64,17811881211592010069_u64,6241353223605611255_u64,8789892829847712947_u64,13716494672578046258_u64];
(*_15) = Field::<i128>(Variant(_31.1, 1), 1) - Field::<i128>(Variant(_31.1, 1), 1);
_11 = (_50, Move(_60), Move(_31), Move((*_72)));
place!(Field::<[u8; 6]>(Variant(_27, 2), 1)) = [70_u8,66_u8,89_u8,48_u8,162_u8,83_u8];
_15 = core::ptr::addr_of_mut!(_81);
(*_34) = [(*_55),(*_55)];
(*_34) = [(*_55),(*_55)];
(*RET) = _80 << (*_55);
(*_34) = [(*_55),(*_55)];
place!(Field::<i32>(Variant(_11.2.1, 1), 2)) = _23.0 as i32;
(*_34) = [(*_55),(*_55)];
_50 = _11.2.0;
_43 = !false;
(*RET) = !_80;
Call((*_15) = core::intrinsics::transmute(_49), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
_58.1 = &mut _43;
_31.0 = _50;
_4 = [(*_55),(*_55)];
_86.fld3.0 = !(*RET);
_2 = &_58.1;
(*RET) = _86.fld3.0;
(*RET) = _86.fld3.0;
_42 = _46;
RET = core::ptr::addr_of_mut!(_33);
(*_34) = [(*_55),(*_55)];
(*_55) = _26;
_45 = [Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1),Field::<i64>(Variant(_10, 3), 1)];
(*RET) = _86.fld3.0 | _86.fld3.0;
(*RET) = _86.fld3.0;
_58.3 = &mut (*_2);
(*_15) = 151_u8 as i128;
_89.0.1 = Move(_11.2.1);
(*_34) = [(*_55),(*_55)];
_11.2.1 = Adt19::Variant1 { fld0: Field::<i8>(Variant(_89.0.1, 1), 0),fld1: (*_15),fld2: Field::<i32>(Variant(_89.0.1, 1), 2) };
(*_66) = &mut _26;
_15 = core::ptr::addr_of_mut!((*_15));
_86.fld0 = [17318840979871072325_u64,12022278572368952983_u64,11264225333974570721_u64,12158982238893751531_u64,6212332330951775759_u64];
(*_55) = (-11822_i16) + (-28167_i16);
_86.fld4.fld3 = Field::<i8>(Variant(_11.2.1, 1), 0) | Field::<i8>(Variant(_11.2.1, 1), 0);
(*_15) = Field::<i128>(Variant(_89.0.1, 1), 1) - Field::<i128>(Variant(_89.0.1, 1), 1);
_81 = !Field::<i128>(Variant(_89.0.1, 1), 1);
_13 = [146_u8,14_u8,38_u8,28_u8,167_u8,47_u8];
Goto(bb43)
}
bb43 = {
(*_34) = [(*_55),(*_55)];
_86.fld4.fld3 = Field::<i8>(Variant(_89.0.1, 1), 0) * Field::<i8>(Variant(_11.2.1, 1), 0);
(*_55) = (-16829_i16) & 17284_i16;
_10 = Adt27::Variant3 { fld0: Move(_8),fld1: 838970946827875055_i64 };
(*_15) = !Field::<i128>(Variant(_11.2.1, 1), 1);
_14 = (*_70);
_78 = core::ptr::addr_of_mut!(_68);
_86.fld4.fld2 = !7_usize;
_23.0 = _44 * _21;
(*_15) = -Field::<i128>(Variant(_89.0.1, 1), 1);
(*RET) = _86.fld3.0;
Goto(bb44)
}
bb44 = {
_86.fld6 = 2840971126786549894_i64;
(*RET) = _80 ^ _80;
_94 = !_42;
_83 = (*_70) << (*_78);
_92 = core::ptr::addr_of_mut!(_86.fld0);
(*_92) = [14690709579069995990_u64,8137508100696018386_u64,7931000858596919499_u64,15154501539796891047_u64,14064815209358783603_u64];
_77.0 = !47_u8;
_94 = _42 * _42;
(*_92) = [3212767042834561341_u64,7447136322482706666_u64,9847670779120591203_u64,4207308458294441855_u64,9453434289355293641_u64];
_31 = Move(_11.2);
_86.fld3.1 = _86.fld6 as f32;
_22 = Field::<i8>(Variant(_89.0.1, 1), 0) as f32;
(*_15) = _86.fld4.fld3 as i128;
_86.fld4.fld1 = -_74;
_85 = -_56;
(*_55) = -3768_i16;
match _86.fld6 {
0 => bb17,
1 => bb16,
2 => bb3,
3 => bb4,
4 => bb5,
2840971126786549894 => bb45,
_ => bb28
}
}
bb45 = {
(*_34) = [(*_55),(*_55)];
(*_55) = 22772_i16;
_86.fld4.fld0 = true;
_87 = core::ptr::addr_of!(_39);
(*_34) = [(*_55),(*_55)];
_89.0.0 = _71;
(*_55) = 30871_i16 ^ 13304_i16;
match _86.fld6 {
0 => bb37,
1 => bb11,
2 => bb10,
3 => bb31,
4 => bb15,
5 => bb33,
2840971126786549894 => bb46,
_ => bb40
}
}
bb46 = {
_67 = _85 * _85;
_1 = core::ptr::addr_of!(_2);
(*_55) = (-24870_i16) << (*_78);
(*RET) = _86.fld3.0 ^ _86.fld3.0;
(*_92) = [7996343035248159321_u64,12859755373747390141_u64,7656353011601905777_u64,1150903486240625452_u64,741031403906892013_u64];
_77.1 = _86.fld4.fld2 ^ _86.fld4.fld2;
(*_92) = [7270248659976740247_u64,5786316534016075695_u64,13349911037600437507_u64,9820329449675662136_u64,2350332628668731684_u64];
place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)) = (_6.0, Move(_89.0.1));
place!(Field::<i128>(Variant(place!(Field::<(char, Adt19)>(Variant(_27, 2), 0)).1, 1), 1)) = (*_15) - _81;
_23.0 = _86.fld4.fld3 as f64;
match _86.fld6 {
0 => bb11,
1 => bb44,
2 => bb9,
3 => bb32,
4 => bb38,
5 => bb47,
2840971126786549894 => bb49,
_ => bb48
}
}
bb47 = {
Return()
}
bb48 = {
_18 = 333649409067903526834872045972240891957_u128 << (*RET);
(*_8) = _11.0;
(*RET) = !1602709477_u32;
(*RET) = 1763889198_u32;
_19 = (-571598518_i32) as f32;
(*_8) = _11.0;
(*_8) = _11.0;
(*RET) = 103_i8 as u32;
(*RET) = !165876115_u32;
_24.fld1 = _18 as f32;
Goto(bb20)
}
bb49 = {
(*_34) = [(*_55),(*_55)];
(*_34) = [(*_55),(*_55)];
(*RET) = !_80;
_60 = core::ptr::addr_of!(_18);
_11 = (_37, Move(_60), Move(_31), Move(_78));
_5 = [3299278453740995081_u64,9584396831778786330_u64,6469769261067010696_u64,2951819547327377642_u64,1709519898706675132_u64,1262310232288605542_u64];
_70 = &_18;
(*_34) = [(*_55),(*_55)];
(*_55) = 19330_i16 + (-22313_i16);
(*_92) = [13903226453157572573_u64,9326009053031886187_u64,11923531328151198854_u64,14081900951860821708_u64,4138581992546338857_u64];
_14 = _77.0 as u128;
_35 = -_40;
place!(Field::<i128>(Variant(_11.2.1, 1), 1)) = (*_15) - (*_15);
_89.0 = (_11.0, Move(Field::<(char, Adt19)>(Variant(_27, 2), 0).1));
(*_87) = 4591_u16 * 47968_u16;
(*_34) = [(*_55),(*_55)];
_87 = core::ptr::addr_of!((*_87));
(*_92) = [15929087645055639912_u64,18005231467457909305_u64,2990510919383759032_u64,14997534039869928954_u64,752446244628469952_u64];
(*_34) = [(*_55),(*_55)];
(*_15) = _77.1 as i128;
Goto(bb50)
}
bb50 = {
Call(_105 = dump_var(Move(_49), Move(_29), Move(_32), Move(_46)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_105 = dump_var(Move(_6), Move(_7), Move(_79), Move(_18)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_105 = dump_var(Move(_33), Move(_13), Move(_81), Move(_4)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_105 = dump_var(Move(_85), Move(_71), Move(_39), Move(_37)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_105 = dump_var(Move(_54), Move(_94), Move(_38), Move(_83)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: *mut u32,mut _2: *const &'static &'static mut bool,mut _3: char,mut _4: char,mut _5: char,mut _6: *const char,mut _7: char,mut _8: char,mut _9: char,mut _10: [u64; 6],mut _11: char,mut _12: char) -> u32 {
mir! {
type RET = u32;
let _13: (&'static mut i16, *mut [u64; 5]);
let _14: [isize; 6];
let _15: u128;
let _16: f64;
let _17: (char, *const u128, (char, Adt19), *mut i32);
let _18: u16;
let _19: *const char;
let _20: char;
let _21: u8;
let _22: *const char;
let _23: *const u16;
let _24: *const (u8, usize, i128, i128);
let _25: i8;
let _26: u16;
let _27: *const [u8; 7];
let _28: u64;
let _29: &'static mut *mut [u64; 5];
let _30: isize;
let _31: usize;
let _32: *const u128;
let _33: u64;
let _34: f64;
let _35: *const isize;
let _36: isize;
let _37: &'static u16;
let _38: i128;
let _39: [i16; 2];
let _40: (*mut i128, [u8; 7], (u32, f32, bool), (char,));
let _41: *const (u8, usize, i128, i128);
let _42: isize;
let _43: *mut i128;
let _44: [u64; 5];
let _45: *const isize;
let _46: &'static &'static mut *mut i32;
let _47: *mut u32;
let _48: &'static mut [bool; 7];
let _49: Adt45;
let _50: [u128; 6];
let _51: i32;
let _52: f64;
let _53: f32;
let _54: [isize; 6];
let _55: i16;
let _56: f32;
let _57: f32;
let _58: [u128; 6];
let _59: &'static mut [i128; 6];
let _60: &'static &'static mut *mut i32;
let _61: isize;
let _62: f64;
let _63: i16;
let _64: isize;
let _65: [u64; 5];
let _66: (char,);
let _67: i8;
let _68: usize;
let _69: (char, Adt19);
let _70: [u8; 7];
let _71: [usize; 3];
let _72: f64;
let _73: (char, Adt19);
let _74: f64;
let _75: *const (u8, usize, i128, i128);
let _76: &'static mut bool;
let _77: u128;
let _78: bool;
let _79: &'static mut &'static (char,);
let _80: ();
let _81: ();
{
RET = 4_usize as u32;
_3 = _7;
_8 = _12;
_14 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-17_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-123_isize)];
_3 = _12;
_8 = _3;
_14 = [(-9223372036854775808_isize),(-112_isize),9223372036854775807_isize,95_isize,47_isize,9223372036854775807_isize];
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!(RET);
_15 = !38778451689032480625362268010092785258_u128;
(*_1) = (-1425492856_i32) as u32;
(*_1) = 4076678741_u32 | 1177279858_u32;
_15 = 47879947803274881251109959353655999586_u128;
(*_1) = 337611_u32;
_9 = _5;
_7 = _4;
(*_1) = 3403233043_u32 | 1385784931_u32;
(*_1) = 5_usize as u32;
(*_1) = 955772128_u32 >> _15;
(*_1) = !2203130129_u32;
(*_1) = 4091720754_u32 * 1193464840_u32;
_16 = 2092_u16 as f64;
_16 = 0_usize as f64;
_17.1 = core::ptr::addr_of!(_15);
(*_1) = 4027579306_u32 * 3067597232_u32;
(*_1) = 1108416188_u32 >> _15;
_17.0 = _11;
_17.0 = _3;
(*_1) = !10717738_u32;
(*_1) = (-62_i8) as u32;
(*_1) = 2437560151_u32 * 1577108802_u32;
(*_1) = !413446138_u32;
(*_1) = 1862437816_u32 >> _15;
(*_1) = 2936963541_u32;
_18 = !32658_u16;
match (*_1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
2936963541 => bb7,
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
(*_1) = 3688371385_u32 * 912475700_u32;
_17.2.0 = _7;
_11 = _17.0;
(*_1) = !1736782118_u32;
_9 = _12;
_17.1 = core::ptr::addr_of!(_15);
_20 = _4;
(*_1) = 524234079_u32 >> _15;
(*_1) = 303693399_u32;
_17.1 = core::ptr::addr_of!(_15);
_3 = _17.0;
(*_1) = !4157706759_u32;
(*_1) = 9223372036854775807_isize as u32;
(*_1) = 1621977347_u32 * 3032616348_u32;
Goto(bb8)
}
bb8 = {
_17.2.1 = Adt19::Variant1 { fld0: 9_i8,fld1: (-139064428313868704279594173268188802815_i128),fld2: (-393477932_i32) };
(*_1) = !2703562244_u32;
_17.0 = _5;
_20 = _17.2.0;
_20 = _9;
_5 = _4;
_21 = 239_u8 | 20_u8;
Goto(bb9)
}
bb9 = {
place!(Field::<i32>(Variant(_17.2.1, 1), 2)) = !(-1213061850_i32);
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 886601042_u32 << _21;
Goto(bb10)
}
bb10 = {
_12 = _20;
(*_1) = 467469222_u32 + 475592418_u32;
_21 = 94_u8 - 235_u8;
_6 = core::ptr::addr_of!(_12);
_17.1 = core::ptr::addr_of!(_15);
_11 = _4;
(*_6) = _17.0;
_7 = _17.0;
_10 = [9931554336397365092_u64,14951243029174011829_u64,3853263571702597323_u64,8284376162453646208_u64,16939627780745154391_u64,800731886388410777_u64];
(*_1) = _16 as u32;
_17.2.1 = Adt19::Variant1 { fld0: (-10_i8),fld1: 19901598234920704926539973955940960493_i128,fld2: 1653096737_i32 };
(*_6) = _9;
(*_6) = _7;
_4 = (*_6);
_21 = 184_u8 >> (*_1);
_3 = _17.2.0;
(*_6) = _3;
_21 = _18 as u8;
(*_1) = 3019115442_u32;
_5 = _20;
Goto(bb11)
}
bb11 = {
(*_6) = _9;
place!(Field::<i32>(Variant(_17.2.1, 1), 2)) = _16 as i32;
(*_1) = 1004330329_u32;
_14 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-122_isize)];
place!(Field::<i8>(Variant(_17.2.1, 1), 0)) = (-15_i8);
(*_1) = !1434474689_u32;
_15 = 166009346639695323643926682182311085659_u128 >> (*_1);
Goto(bb12)
}
bb12 = {
(*_1) = 2493071973_u32 + 662768474_u32;
(*_1) = _16 as u32;
(*_1) = 948373892_u32 & 3976320553_u32;
_14 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),25_isize,9223372036854775807_isize,(-63_isize)];
_26 = _18 << (*_1);
(*_1) = _16 as u32;
(*_1) = !3832897505_u32;
_6 = core::ptr::addr_of!((*_6));
_5 = (*_6);
(*_1) = 2979658630_u32 * 399895350_u32;
(*_1) = !3354772560_u32;
(*_6) = _8;
_4 = _5;
(*_6) = _17.2.0;
(*_1) = 9223372036854775807_isize as u32;
(*_6) = _5;
(*_1) = !3086038616_u32;
(*_1) = !4016856874_u32;
match Field::<i8>(Variant(_17.2.1, 1), 0) {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb6,
340282366920938463463374607431768211441 => bb13,
_ => bb8
}
}
bb13 = {
(*_1) = !2447100140_u32;
_4 = _17.2.0;
(*_6) = _3;
(*_6) = _11;
(*_6) = _17.0;
(*_6) = _8;
_28 = _15 as u64;
Goto(bb14)
}
bb14 = {
(*_6) = _4;
(*_1) = 716208440_u32 << _26;
(*_1) = _16 as u32;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _17.2.0;
_19 = core::ptr::addr_of!((*_6));
(*_19) = _7;
_12 = _9;
(*_6) = _4;
RET = 3485000183_u32 * 3272838492_u32;
(*_6) = _4;
_17.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_17.2.1, 1), 2)));
RET = !1127814435_u32;
_30 = 60_isize + 9223372036854775807_isize;
_17.2.0 = _12;
_4 = (*_6);
_36 = _30 - _30;
(*_6) = _11;
(*_6) = _7;
_17.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_17.2.1, 1), 2)));
_23 = core::ptr::addr_of!(_18);
(*_23) = _26;
_25 = Field::<i8>(Variant(_17.2.1, 1), 0) & Field::<i8>(Variant(_17.2.1, 1), 0);
_8 = (*_6);
(*_6) = _4;
(*_1) = 3093773633_u32 & 2551899766_u32;
Goto(bb15)
}
bb15 = {
place!(Field::<i8>(Variant(_17.2.1, 1), 0)) = !_25;
(*_1) = 569372655_u32 * 528739837_u32;
Goto(bb16)
}
bb16 = {
(*_23) = !_26;
(*_23) = 65120015042440407016067785429488875278_i128 as u16;
(*_23) = _26;
(*_6) = _7;
(*_1) = (-4089798143208967710_i64) as u32;
(*_23) = _26;
_26 = (*_23);
(*_1) = !835649439_u32;
(*_1) = 564954132_u32;
(*_1) = 2272750786_u32 * 3128326008_u32;
(*_1) = 2543171221_u32 + 2380826788_u32;
_40.2.0 = (*_1) >> _18;
_40.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_17.2.1, 1), 1)));
(*_6) = _8;
_14 = [_30,_36,_36,_36,_36,_36];
(*_1) = _40.2.0;
(*_6) = _17.0;
(*_23) = _26 | _26;
(*_6) = _8;
Goto(bb17)
}
bb17 = {
_11 = (*_6);
(*_1) = _40.2.0;
_30 = _36 ^ _36;
_40.2.2 = !true;
_18 = !_26;
(*_23) = !_26;
_7 = _9;
Goto(bb18)
}
bb18 = {
_32 = core::ptr::addr_of!(_15);
(*_32) = !235147200679691982383621875784174952919_u128;
(*_32) = 13427145582278129407573316112797441276_u128 | 10448730164539369929669321397216262580_u128;
(*_23) = _26 & _26;
(*_1) = Field::<i32>(Variant(_17.2.1, 1), 2) as u32;
_22 = core::ptr::addr_of!((*_6));
_12 = _17.2.0;
_40.1 = [_21,_21,_21,_21,_21,_21,_21];
_39 = [(-2099_i16),(-7760_i16)];
(*_22) = _8;
(*_6) = _5;
(*_23) = 2814315907329065980_usize as u16;
(*_6) = _9;
(*_1) = _40.2.0 << (*_23);
(*_23) = _26;
(*_6) = _4;
(*_23) = _30 as u16;
place!(Field::<i32>(Variant(_17.2.1, 1), 2)) = 1037639370_i32;
(*_32) = 69862787452878076502661740207988678389_u128;
(*_6) = _7;
_9 = (*_6);
(*_32) = 95652037380675668875471299343179181303_u128 + 318808195504068773936124343282543861333_u128;
_33 = (*_32) as u64;
(*_6) = _7;
(*_32) = 160117160309572025731915381136806342690_u128 + 72771173530143175713581798214884780484_u128;
match Field::<i32>(Variant(_17.2.1, 1), 2) {
0 => bb19,
1037639370 => bb21,
_ => bb20
}
}
bb19 = {
_12 = _20;
(*_1) = 467469222_u32 + 475592418_u32;
_21 = 94_u8 - 235_u8;
_6 = core::ptr::addr_of!(_12);
_17.1 = core::ptr::addr_of!(_15);
_11 = _4;
(*_6) = _17.0;
_7 = _17.0;
_10 = [9931554336397365092_u64,14951243029174011829_u64,3853263571702597323_u64,8284376162453646208_u64,16939627780745154391_u64,800731886388410777_u64];
(*_1) = _16 as u32;
_17.2.1 = Adt19::Variant1 { fld0: (-10_i8),fld1: 19901598234920704926539973955940960493_i128,fld2: 1653096737_i32 };
(*_6) = _9;
(*_6) = _7;
_4 = (*_6);
_21 = 184_u8 >> (*_1);
_3 = _17.2.0;
(*_6) = _3;
_21 = _18 as u8;
(*_1) = 3019115442_u32;
_5 = _20;
Goto(bb11)
}
bb20 = {
(*_23) = !_26;
(*_23) = 65120015042440407016067785429488875278_i128 as u16;
(*_23) = _26;
(*_6) = _7;
(*_1) = (-4089798143208967710_i64) as u32;
(*_23) = _26;
_26 = (*_23);
(*_1) = !835649439_u32;
(*_1) = 564954132_u32;
(*_1) = 2272750786_u32 * 3128326008_u32;
(*_1) = 2543171221_u32 + 2380826788_u32;
_40.2.0 = (*_1) >> _18;
_40.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_17.2.1, 1), 1)));
(*_6) = _8;
_14 = [_30,_36,_36,_36,_36,_36];
(*_1) = _40.2.0;
(*_6) = _17.0;
(*_23) = _26 | _26;
(*_6) = _8;
Goto(bb17)
}
bb21 = {
_40.3 = ((*_6),);
(*_6) = _11;
_11 = (*_6);
(*_6) = _9;
_38 = 53464856707854534620299248938357082674_i128 + 96731895088350944534425933756780881904_i128;
_9 = (*_6);
(*_23) = _16 as u16;
(*_23) = _26 | _26;
(*_1) = _40.2.0;
(*_23) = Field::<i32>(Variant(_17.2.1, 1), 2) as u16;
(*_6) = _4;
_30 = _36 | _36;
_18 = !_26;
_42 = _36 - _36;
Goto(bb22)
}
bb22 = {
_44 = [_28,_28,_28,_33,_33];
_47 = core::ptr::addr_of_mut!((*_1));
(*_23) = _26 + _26;
_8 = _17.0;
(*_47) = 8254143489067133825_i64 as u32;
(*_47) = Field::<i32>(Variant(_17.2.1, 1), 2) as u32;
(*_6) = _11;
(*_32) = 76859234289692050560663030659839164761_u128 >> Field::<i8>(Variant(_17.2.1, 1), 0);
_10 = [_33,_28,_33,_28,_28,_28];
(*_32) = 63825246325763533118936947420710961222_u128;
(*_1) = _40.2.0;
(*_23) = _40.2.2 as u16;
_3 = (*_6);
_17.2.1 = Adt19::Variant1 { fld0: _25,fld1: _38,fld2: 1571591971_i32 };
(*_32) = 308760238696757546664801305595047294292_u128;
(*_1) = _21 as u32;
(*_1) = (-22692_i16) as u32;
(*_23) = _26;
(*_23) = 10312257966823908814_usize as u16;
(*_1) = _40.2.0 - _40.2.0;
(*_32) = 265863392851673564178184598574342861816_u128 * 98870104234785937331988104294479926830_u128;
_45 = core::ptr::addr_of!(_30);
(*_1) = _40.2.0 + _40.2.0;
_44 = [_28,_33,_33,_33,_28];
(*_45) = _42;
_9 = (*_6);
(*_32) = 303498711159033999553417288322493050267_u128;
Goto(bb23)
}
bb23 = {
(*_23) = _26;
_40.3 = ((*_6),);
_23 = core::ptr::addr_of!((*_23));
_27 = core::ptr::addr_of!(_40.1);
(*_45) = _42 | _42;
_54 = [(*_45),(*_45),(*_45),(*_45),(*_45),(*_45)];
(*_23) = !_26;
Call((*_23) = core::intrinsics::transmute(_26), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
(*_32) = 31266520643706308618983151690587092503_u128 | 125652027997406551749784051325338234187_u128;
(*_1) = _40.2.0 ^ _40.2.0;
(*_32) = 53215483445030324996224774332428817282_u128;
(*_1) = !_40.2.0;
(*_1) = (-1893169774_i32) as u32;
Goto(bb25)
}
bb25 = {
_40.2.1 = Field::<i8>(Variant(_17.2.1, 1), 0) as f32;
_17.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_17.2.1, 1), 2)));
(*_32) = 119307515983794400192384556204305332480_u128;
(*_6) = _9;
Goto(bb26)
}
bb26 = {
(*_23) = _26 - _26;
(*_1) = _40.2.0;
_1 = Move(_47);
_7 = (*_6);
_52 = _28 as f64;
(*_32) = _40.2.0 as u128;
(*_23) = _26 * _26;
_9 = (*_6);
_56 = -_40.2.1;
_58 = [(*_32),(*_32),(*_32),(*_32),(*_32),(*_32)];
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_40.1 = [_21,_21,_21,_21,_21,_21,_21];
(*_45) = _42;
_25 = -Field::<i8>(Variant(_17.2.1, 1), 0);
(*_45) = (*_23) as isize;
(*_23) = _26 & _26;
_35 = core::ptr::addr_of!((*_45));
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
(*_32) = 335872233832830634587728627951254426615_u128 ^ 77821919076280839678211648891401253203_u128;
(*_23) = !_26;
(*_35) = _36 & _36;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_34 = _52;
Goto(bb27)
}
bb27 = {
(*_23) = !_26;
(*_35) = _36 & _36;
(*_23) = _40.2.0 as u16;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
(*_35) = _42 * _36;
_17.1 = core::ptr::addr_of!((*_32));
_31 = 5_usize << (*_35);
(*_45) = _42 + _36;
(*_32) = _40.2.1 as u128;
place!(Field::<i32>(Variant(_17.2.1, 1), 2)) = (*_23) as i32;
(*_45) = _36 - _42;
(*_45) = -_42;
(*_32) = 111494004337509972396346151569347591430_u128 & 54360896564697936466444357162194571095_u128;
(*_6) = _5;
_13.1 = core::ptr::addr_of_mut!(_44);
_17.1 = core::ptr::addr_of!((*_32));
Goto(bb28)
}
bb28 = {
(*_23) = _26;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_27 = core::ptr::addr_of!((*_27));
(*_6) = _40.3.0;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_52 = _34 - _34;
_35 = core::ptr::addr_of!((*_45));
place!(Field::<i32>(Variant(_17.2.1, 1), 2)) = -689698068_i32;
_14 = [(*_45),(*_45),(*_45),(*_35),(*_35),(*_35)];
(*_23) = !_26;
_23 = core::ptr::addr_of!((*_23));
(*_32) = (*_45) as u128;
(*_45) = _40.2.0 as isize;
_34 = _52 + _52;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_38 = Field::<i128>(Variant(_17.2.1, 1), 1) >> (*_32);
(*_6) = _17.0;
(*_45) = _36 | _36;
(*_23) = _26;
_58 = [(*_32),(*_32),(*_32),(*_32),(*_32),(*_32)];
Goto(bb29)
}
bb29 = {
_20 = (*_6);
_50 = _58;
_47 = core::ptr::addr_of_mut!(RET);
_40.2.1 = _56 + _56;
(*_47) = _40.2.0 >> (*_23);
_22 = core::ptr::addr_of!(_66.0);
_17.2.0 = _9;
(*_22) = (*_6);
_14 = [(*_45),(*_45),(*_45),(*_45),(*_45),(*_45)];
(*_32) = 220479466091301710017030071512377883777_u128 >> (*_45);
(*_6) = (*_22);
(*_32) = (*_47) as u128;
Goto(bb30)
}
bb30 = {
_69 = Move(_17.2);
_17.1 = core::ptr::addr_of!((*_32));
_42 = (*_45) ^ (*_45);
(*_22) = (*_6);
_52 = _34 + _16;
_58 = [(*_32),(*_32),(*_32),(*_32),(*_32),(*_32)];
_40.3 = ((*_22),);
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_73.0 = (*_22);
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
(*_23) = _26;
Goto(bb31)
}
bb31 = {
_12 = (*_22);
Goto(bb32)
}
bb32 = {
(*_22) = _4;
Goto(bb33)
}
bb33 = {
(*_6) = (*_22);
(*_45) = 1711_i16 as isize;
(*_45) = _31 as isize;
_40.2 = ((*_47), _56, false);
_13.1 = core::ptr::addr_of_mut!(_65);
(*_22) = _3;
(*_23) = !_26;
Call(_30 = core::intrinsics::transmute(_42), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_62 = -_52;
_57 = _56 - _56;
(*_47) = _40.3.0 as u32;
Goto(bb35)
}
bb35 = {
(*_6) = _40.3.0;
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
_65 = _44;
_40.1 = [_21,_21,_21,_21,_21,_21,_21];
(*_45) = _36 + _42;
_57 = _56 * _56;
(*_32) = 332440925318474596528268001271777476246_u128;
(*_23) = _26;
_73.1 = Move(_69.1);
(*_22) = (*_6);
_66.0 = _40.3.0;
(*_23) = _26 & _26;
_52 = -_62;
(*_47) = _40.2.0 | _40.2.0;
(*_45) = Field::<i32>(Variant(_73.1, 1), 2) as isize;
(*_22) = (*_6);
(*_27) = [_21,_21,_21,_21,_21,_21,_21];
Goto(bb36)
}
bb36 = {
Call(_80 = dump_var(Move(_20), Move(_7), Move(_18), Move(_10)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_80 = dump_var(Move(_50), Move(_15), Move(_12), Move(_58)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_80 = dump_var(Move(_26), Move(_30), Move(_39), Move(_21)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_80 = dump_var(Move(_65), Move(_14), Move(_4), _81), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(16_u8), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(173986977919419959739247226464074616855_u128), std::hint::black_box(8199_i16), std::hint::black_box(2131297927656935976_u64), std::hint::black_box((-5030581000216205747_i64)), std::hint::black_box(8635_u16), std::hint::black_box(5_usize));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt19 {
Variant0{
fld0: bool,
fld1: u8,
fld2: isize,
fld3: (f64,),
fld4: usize,
fld5: u16,

},
Variant1{
fld0: i8,
fld1: i128,
fld2: i32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt25 {
fld0: bool,
fld1: f32,
fld2: usize,
fld3: i8,
fld4: u16,
fld5: u32,
}
#[derive(Debug)]
pub enum Adt27 {
Variant0{
fld0: *const u128,
fld1: *mut i32,
fld2: isize,
fld3: i8,
fld4: u8,
fld5: i32,
fld6: u32,

},
Variant1{
fld0: i128,
fld1: char,
fld2: f32,
fld3: i8,
fld4: (u32, f32, bool),
fld5: [u64; 5],

},
Variant2{
fld0: (u32, f32, bool),
fld1: Adt19,
fld2: *const u128,
fld3: (char,),
fld4: i128,
fld5: u64,
fld6: u8,

},
Variant3{
fld0: *const char,
fld1: i64,

}}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: Adt27,
fld1: u16,
fld2: [u64; 5],
fld3: *const char,
fld4: usize,
fld5: f64,
fld6: i64,

},
Variant1{
fld0: *mut [u64; 5],
fld1: Adt19,
fld2: u32,
fld3: *const isize,
fld4: (char, Adt19),
fld5: u128,
fld6: i64,
fld7: i128,

},
Variant2{
fld0: *mut i32,

}}
#[derive(Debug)]
pub struct Adt46 {
fld0: [u64; 5],
fld1: Adt27,
fld2: usize,
fld3: (u32, f32, bool),
fld4: Adt25,
fld5: Adt19,
fld6: i64,
fld7: i128,
}
#[derive(Debug)]
pub struct Adt47 {
fld0: (char, Adt19),
fld1: char,
fld2: f32,
fld3: u64,
fld4: i16,
fld5: Adt25,
fld6: Adt19,
fld7: *mut i32,
}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: [usize; 3],
fld1: char,
fld2: Adt46,
fld3: i32,
fld4: u16,

},
Variant1{
fld0: bool,

},
Variant2{
fld0: (u8, usize, i128, i128),
fld1: Adt27,
fld2: [u128; 6],
fld3: *const isize,
fld4: i16,
fld5: [usize; 3],
fld6: *mut i128,

}}
#[derive(Debug)]
pub enum Adt78 {
Variant0{
fld0: (u8, usize, i128, i128),
fld1: *const [u8; 7],
fld2: *const u128,
fld3: (u32, f32, bool),
fld4: [i64; 4],
fld5: (f64,),

},
Variant1{
fld0: [usize; 3],
fld1: (char, *const u128, (char, Adt19), *mut i32),

},
Variant2{
fld0: (char, Adt19),
fld1: [u8; 6],

}}

