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
pub fn fn0(mut _1: bool,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u32) -> i128 {
mir! {
type RET = i128;
let _11: f32;
let _12: i16;
let _13: usize;
let _14: f64;
let _15: f64;
let _16: Adt29;
let _17: [i128; 1];
let _18: ((bool, (i8, u16, Adt29), i128, u64),);
let _19: (*mut i128,);
let _20: bool;
let _21: u64;
let _22: &'static *mut i128;
let _23: Adt18;
let _24: char;
let _25: *mut usize;
let _26: u64;
let _27: &'static i128;
let _28: u16;
let _29: *mut *const usize;
let _30: i16;
let _31: &'static mut &'static mut &'static &'static Adt18;
let _32: i8;
let _33: isize;
let _34: f32;
let _35: &'static &'static Adt18;
let _36: &'static i128;
let _37: f64;
let _38: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _39: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _40: usize;
let _41: *mut usize;
let _42: f32;
let _43: *const i8;
let _44: *const (u32,);
let _45: char;
let _46: char;
let _47: *mut usize;
let _48: ((bool, (i8, u16, Adt29), i128, u64),);
let _49: &'static mut i8;
let _50: *const i8;
let _51: isize;
let _52: *const *const &'static mut u128;
let _53: &'static u128;
let _54: Adt18;
let _55: &'static &'static Adt18;
let _56: &'static mut &'static &'static Adt18;
let _57: Adt63;
let _58: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _59: char;
let _60: Adt63;
let _61: *const [bool; 1];
let _62: *mut *const usize;
let _63: *mut f64;
let _64: (bool, (i8, u16, Adt29), i128, u64);
let _65: char;
let _66: u128;
let _67: isize;
let _68: &'static mut i8;
let _69: Adt25;
let _70: ();
let _71: ();
{
_9 = 0_usize - 5_usize;
RET = 36497349226383442197435516365849944365_i128 ^ (-75823658984929996783272831468265347296_i128);
_1 = !true;
_5 = 22187_i16 & (-20574_i16);
_5 = (-11655_i16) ^ (-11910_i16);
Call(_2 = fn1(_5, _9, RET, RET, _9, _5, _1, RET, RET, _9, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-2010755988211303306_i64) >> _5;
_6 = 1772927164_i32 >> _7;
_6 = !(-966975388_i32);
_5 = (-3279_i16);
RET = !169530333290565591166945214227001852784_i128;
_8 = !RET;
_10 = 1546172177_u32 >> RET;
_11 = _9 as f32;
_2 = !6332925815505656270_u64;
match _5 {
0 => bb2,
340282366920938463463374607431768208177 => bb4,
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
_4 = 24_i8 * 52_i8;
_6 = -(-1006264338_i32);
_2 = 2908768962432868487_u64 & 5803611618106058736_u64;
_7 = !(-8541487020226679426_i64);
_10 = 426709307_u32;
Goto(bb5)
}
bb5 = {
_10 = 1809677817_u32 | 4141984366_u32;
_10 = !2089622191_u32;
_12 = _5 ^ _5;
_1 = !false;
_12 = _5 | _5;
_12 = !_5;
_6 = -49673918_i32;
_2 = 8979963310006090576_u64;
_3 = 9223372036854775807_isize >> _9;
_6 = 151011142101425190202462624697148222407_u128 as i32;
_4 = 258709411849641477388205782994783877068_u128 as i8;
_13 = 274679785480931686513356672167826799220_u128 as usize;
_7 = (-1994745089166720908_i64) << _9;
_15 = _7 as f64;
_13 = _9;
Call(_4 = core::intrinsics::bswap(127_i8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15 = _10 as f64;
_12 = _7 as i16;
_10 = 673968424_u32 & 1793494801_u32;
_7 = _11 as i64;
_2 = 11053161193491778320_u64;
_11 = _7 as f32;
_9 = RET as usize;
_18.0.3 = _2 ^ _2;
_18.0.0 = _1 & _1;
_18.0.1.0 = _18.0.3 as i8;
_15 = 154_u16 as f64;
_4 = _18.0.1.0 ^ _18.0.1.0;
_18.0.2 = _8;
_12 = _5 << _18.0.1.0;
_18.0.1.1 = 35006_u16;
_10 = 527700466_u32 * 1364176837_u32;
_1 = _18.0.0 ^ _18.0.0;
match _18.0.1.1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
35006 => bb12,
_ => bb11
}
}
bb7 = {
_10 = 1809677817_u32 | 4141984366_u32;
_10 = !2089622191_u32;
_12 = _5 ^ _5;
_1 = !false;
_12 = _5 | _5;
_12 = !_5;
_6 = -49673918_i32;
_2 = 8979963310006090576_u64;
_3 = 9223372036854775807_isize >> _9;
_6 = 151011142101425190202462624697148222407_u128 as i32;
_4 = 258709411849641477388205782994783877068_u128 as i8;
_13 = 274679785480931686513356672167826799220_u128 as usize;
_7 = (-1994745089166720908_i64) << _9;
_15 = _7 as f64;
_13 = _9;
Call(_4 = core::intrinsics::bswap(127_i8), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_4 = 24_i8 * 52_i8;
_6 = -(-1006264338_i32);
_2 = 2908768962432868487_u64 & 5803611618106058736_u64;
_7 = !(-8541487020226679426_i64);
_10 = 426709307_u32;
Goto(bb5)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_7 = (-2010755988211303306_i64) >> _5;
_6 = 1772927164_i32 >> _7;
_6 = !(-966975388_i32);
_5 = (-3279_i16);
RET = !169530333290565591166945214227001852784_i128;
_8 = !RET;
_10 = 1546172177_u32 >> RET;
_11 = _9 as f32;
_2 = !6332925815505656270_u64;
match _5 {
0 => bb2,
340282366920938463463374607431768208177 => bb4,
_ => bb3
}
}
bb12 = {
_20 = _1 > _1;
_23.fld4 = _12 >> _18.0.1.0;
_23.fld0 = !_18.0.0;
_17 = [_18.0.2];
_13 = _9 - _9;
Goto(bb13)
}
bb13 = {
match _5 {
0 => bb12,
1 => bb14,
2 => bb15,
340282366920938463463374607431768208177 => bb17,
_ => bb16
}
}
bb14 = {
_20 = _1 > _1;
_23.fld4 = _12 >> _18.0.1.0;
_23.fld0 = !_18.0.0;
_17 = [_18.0.2];
_13 = _9 - _9;
Goto(bb13)
}
bb15 = {
_7 = (-2010755988211303306_i64) >> _5;
_6 = 1772927164_i32 >> _7;
_6 = !(-966975388_i32);
_5 = (-3279_i16);
RET = !169530333290565591166945214227001852784_i128;
_8 = !RET;
_10 = 1546172177_u32 >> RET;
_11 = _9 as f32;
_2 = !6332925815505656270_u64;
match _5 {
0 => bb2,
340282366920938463463374607431768208177 => bb4,
_ => bb3
}
}
bb16 = {
_4 = 24_i8 * 52_i8;
_6 = -(-1006264338_i32);
_2 = 2908768962432868487_u64 & 5803611618106058736_u64;
_7 = !(-8541487020226679426_i64);
_10 = 426709307_u32;
Goto(bb5)
}
bb17 = {
_20 = !_1;
_18.0.2 = !RET;
_23.fld2 = _3;
_13 = _9;
_9 = _13 << _13;
_21 = 33_u8 as u64;
_20 = _11 < _11;
_18.0.2 = RET | _8;
_23.fld0 = _20 >= _18.0.0;
_14 = _15 + _15;
_18.0.3 = _21;
_4 = 849151507881715793177732033880119736_u128 as i8;
_17 = [_18.0.2];
_18.0.0 = !_1;
_21 = _18.0.3 * _2;
_18.0.1.0 = _6 as i8;
RET = 22238776234495427330002769078832581042_u128 as i128;
_21 = _18.0.3;
_23.fld2 = _3;
match _5 {
0 => bb15,
1 => bb2,
2 => bb3,
3 => bb16,
4 => bb14,
5 => bb9,
6 => bb18,
340282366920938463463374607431768208177 => bb20,
_ => bb19
}
}
bb18 = {
_7 = (-2010755988211303306_i64) >> _5;
_6 = 1772927164_i32 >> _7;
_6 = !(-966975388_i32);
_5 = (-3279_i16);
RET = !169530333290565591166945214227001852784_i128;
_8 = !RET;
_10 = 1546172177_u32 >> RET;
_11 = _9 as f32;
_2 = !6332925815505656270_u64;
match _5 {
0 => bb2,
340282366920938463463374607431768208177 => bb4,
_ => bb3
}
}
bb19 = {
_20 = _1 > _1;
_23.fld4 = _12 >> _18.0.1.0;
_23.fld0 = !_18.0.0;
_17 = [_18.0.2];
_13 = _9 - _9;
Goto(bb13)
}
bb20 = {
_19.0 = core::ptr::addr_of_mut!(RET);
_18.0.2 = _8 - RET;
_6 = 1074705687_i32 >> _18.0.2;
_23.fld5 = _9;
_18.0.1.1 = !20465_u16;
_23.fld1 = _23.fld5 as u8;
_6 = _23.fld1 as i32;
_26 = _21;
_18.0.0 = _23.fld0;
Goto(bb21)
}
bb21 = {
_14 = -_15;
_8 = RET;
_28 = _11 as u16;
_23.fld5 = _13 | _13;
_4 = _18.0.1.0 & _18.0.1.0;
_13 = _9;
_23.fld1 = 7_u8;
_7 = 513694500053114241_i64 + 5243558390172949284_i64;
_4 = _18.0.1.0 * _18.0.1.0;
_23.fld3 = core::ptr::addr_of!(_13);
RET = _18.0.2 & _8;
_18.0.1.1 = !_28;
_21 = _26;
_23.fld1 = !241_u8;
_26 = !_18.0.3;
_18.0.2 = _4 as i128;
_18.0.0 = !_23.fld0;
_18.0.1.0 = _4 | _4;
_18.0.2 = _8 + RET;
_23.fld3 = core::ptr::addr_of!(_13);
_21 = _9 as u64;
_24 = '\u{e3a6a}';
_10 = !1573676475_u32;
_12 = -_23.fld4;
_25 = core::ptr::addr_of_mut!(_13);
match _2 {
0 => bb11,
1 => bb18,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb26,
11053161193491778320 => bb28,
_ => bb27
}
}
bb22 = {
_4 = 24_i8 * 52_i8;
_6 = -(-1006264338_i32);
_2 = 2908768962432868487_u64 & 5803611618106058736_u64;
_7 = !(-8541487020226679426_i64);
_10 = 426709307_u32;
Goto(bb5)
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_20 = !_1;
_18.0.2 = !RET;
_23.fld2 = _3;
_13 = _9;
_9 = _13 << _13;
_21 = 33_u8 as u64;
_20 = _11 < _11;
_18.0.2 = RET | _8;
_23.fld0 = _20 >= _18.0.0;
_14 = _15 + _15;
_18.0.3 = _21;
_4 = 849151507881715793177732033880119736_u128 as i8;
_17 = [_18.0.2];
_18.0.0 = !_1;
_21 = _18.0.3 * _2;
_18.0.1.0 = _6 as i8;
RET = 22238776234495427330002769078832581042_u128 as i128;
_21 = _18.0.3;
_23.fld2 = _3;
match _5 {
0 => bb15,
1 => bb2,
2 => bb3,
3 => bb16,
4 => bb14,
5 => bb9,
6 => bb18,
340282366920938463463374607431768208177 => bb20,
_ => bb19
}
}
bb26 = {
_7 = (-2010755988211303306_i64) >> _5;
_6 = 1772927164_i32 >> _7;
_6 = !(-966975388_i32);
_5 = (-3279_i16);
RET = !169530333290565591166945214227001852784_i128;
_8 = !RET;
_10 = 1546172177_u32 >> RET;
_11 = _9 as f32;
_2 = !6332925815505656270_u64;
match _5 {
0 => bb2,
340282366920938463463374607431768208177 => bb4,
_ => bb3
}
}
bb27 = {
_10 = 1809677817_u32 | 4141984366_u32;
_10 = !2089622191_u32;
_12 = _5 ^ _5;
_1 = !false;
_12 = _5 | _5;
_12 = !_5;
_6 = -49673918_i32;
_2 = 8979963310006090576_u64;
_3 = 9223372036854775807_isize >> _9;
_6 = 151011142101425190202462624697148222407_u128 as i32;
_4 = 258709411849641477388205782994783877068_u128 as i8;
_13 = 274679785480931686513356672167826799220_u128 as usize;
_7 = (-1994745089166720908_i64) << _9;
_15 = _7 as f64;
_13 = _9;
Call(_4 = core::intrinsics::bswap(127_i8), ReturnTo(bb6), UnwindUnreachable())
}
bb28 = {
_23.fld2 = _3 * _3;
_18.0.1.2 = Adt29::Variant0 { fld0: _8,fld1: Move(_23.fld3),fld2: _14,fld3: _6 };
_2 = _18.0.3 - _21;
place!(Field::<i128>(Variant(_18.0.1.2, 0), 0)) = _18.0.2 - _18.0.2;
_22 = &_19.0;
_23.fld3 = core::ptr::addr_of!((*_25));
(*_25) = _23.fld5 ^ _23.fld5;
(*_25) = _9 << _12;
_19.0 = core::ptr::addr_of_mut!(_18.0.2);
Goto(bb29)
}
bb29 = {
_23.fld1 = (*_25) as u8;
Goto(bb30)
}
bb30 = {
_19.0 = core::ptr::addr_of_mut!(_8);
(*_25) = _9;
_23.fld4 = _12;
(*_25) = _23.fld5;
_9 = (*_25) & (*_25);
_18.0.1.1 = _11 as u16;
_18.0.1.2 = Adt29::Variant0 { fld0: RET,fld1: Move(_23.fld3),fld2: _14,fld3: _6 };
_23.fld3 = core::ptr::addr_of!((*_25));
_29 = core::ptr::addr_of_mut!(place!(Field::<*const usize>(Variant(_18.0.1.2, 0), 1)));
(*_29) = Move(_23.fld3);
(*_25) = _23.fld5 | _9;
_27 = &_18.0.2;
Goto(bb31)
}
bb31 = {
(*_25) = _9 & _9;
_18.0.2 = RET | Field::<i128>(Variant(_18.0.1.2, 0), 0);
_23.fld2 = !_3;
_3 = _23.fld2;
_1 = _5 < _12;
_30 = _11 as i16;
_12 = _18.0.1.0 as i16;
_5 = _23.fld4 + _30;
_33 = _3 | _23.fld2;
Goto(bb32)
}
bb32 = {
(*_29) = core::ptr::addr_of!((*_25));
(*_29) = core::ptr::addr_of!((*_25));
_5 = 207905847512717054178776189088330379442_u128 as i16;
(*_25) = _9 - _23.fld5;
_7 = !(-3362491959808838942_i64);
_7 = 7832671740111897029_i64 + 6611938312196931921_i64;
(*_29) = core::ptr::addr_of!((*_25));
(*_29) = core::ptr::addr_of!((*_25));
_14 = (*_25) as f64;
(*_29) = core::ptr::addr_of!(_13);
(*_25) = _23.fld5 * _9;
_23 = Adt18 { fld0: _1,fld1: 20_u8,fld2: _33,fld3: Move((*_29)),fld4: _12,fld5: (*_25) };
_37 = Field::<f64>(Variant(_18.0.1.2, 0), 2) + Field::<f64>(Variant(_18.0.1.2, 0), 2);
(*_25) = !_23.fld5;
match _23.fld1 {
20 => bb33,
_ => bb9
}
}
bb33 = {
(*_25) = !_23.fld5;
(*_29) = core::ptr::addr_of!((*_25));
place!(Field::<*const usize>(Variant(_18.0.1.2, 0), 1)) = core::ptr::addr_of!((*_25));
_21 = !_2;
(*_25) = _2 as usize;
_28 = (*_25) as u16;
_38.1 = _28 << _18.0.2;
_38.1 = _18.0.1.1;
_7 = _33 as i64;
(*_25) = _24 as usize;
(*_29) = Move(_23.fld3);
_38.3 = [_8];
(*_25) = _9 & _23.fld5;
_39.2 = [_1];
_42 = _11 + _11;
(*_25) = _23.fld5 & _9;
_16 = Adt29::Variant0 { fld0: _18.0.2,fld1: Move((*_29)),fld2: _14,fld3: Field::<i32>(Variant(_18.0.1.2, 0), 3) };
_18.0.1 = (_4, _38.1, Move(_16));
_43 = core::ptr::addr_of!(_18.0.1.0);
_21 = _11 as u64;
(*_43) = -_4;
_25 = core::ptr::addr_of_mut!((*_25));
match _23.fld1 {
0 => bb5,
1 => bb6,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
20 => bb39,
_ => bb38
}
}
bb34 = {
_19.0 = core::ptr::addr_of_mut!(RET);
_18.0.2 = _8 - RET;
_6 = 1074705687_i32 >> _18.0.2;
_23.fld5 = _9;
_18.0.1.1 = !20465_u16;
_23.fld1 = _23.fld5 as u8;
_6 = _23.fld1 as i32;
_26 = _21;
_18.0.0 = _23.fld0;
Goto(bb21)
}
bb35 = {
Return()
}
bb36 = {
Return()
}
bb37 = {
_20 = _1 > _1;
_23.fld4 = _12 >> _18.0.1.0;
_23.fld0 = !_18.0.0;
_17 = [_18.0.2];
_13 = _9 - _9;
Goto(bb13)
}
bb38 = {
_10 = 1809677817_u32 | 4141984366_u32;
_10 = !2089622191_u32;
_12 = _5 ^ _5;
_1 = !false;
_12 = _5 | _5;
_12 = !_5;
_6 = -49673918_i32;
_2 = 8979963310006090576_u64;
_3 = 9223372036854775807_isize >> _9;
_6 = 151011142101425190202462624697148222407_u128 as i32;
_4 = 258709411849641477388205782994783877068_u128 as i8;
_13 = 274679785480931686513356672167826799220_u128 as usize;
_7 = (-1994745089166720908_i64) << _9;
_15 = _7 as f64;
_13 = _9;
Call(_4 = core::intrinsics::bswap(127_i8), ReturnTo(bb6), UnwindUnreachable())
}
bb39 = {
_4 = 86278104952794224553024382060028602673_u128 as i8;
_38.1 = !_18.0.1.1;
_36 = &_18.0.2;
(*_43) = -_4;
(*_25) = _23.fld1 as usize;
(*_25) = _23.fld5 & _23.fld5;
_28 = _18.0.1.1 << _21;
_39.3.0 = Move(_18.0);
(*_43) = _4 & _39.3.0.1.0;
_38.2.1 = Move(_39.3.0.1.2);
_34 = 145044464079495011969533618893071880601_u128 as f32;
_43 = core::ptr::addr_of!((*_43));
_34 = _42 + _42;
_48.0.1.2 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_38.2.1, 0), 0),fld1: Move(Field::<*const usize>(Variant(_38.2.1, 0), 1)),fld2: Field::<f64>(Variant(_38.2.1, 0), 2),fld3: _6 };
_17 = [Field::<i128>(Variant(_38.2.1, 0), 0)];
(*_25) = _34 as usize;
Goto(bb40)
}
bb40 = {
_1 = !_23.fld0;
(*_25) = _23.fld5;
(*_25) = _23.fld5;
_16 = Adt29::Variant0 { fld0: _39.3.0.2,fld1: Move(Field::<*const usize>(Variant(_48.0.1.2, 0), 1)),fld2: _14,fld3: _6 };
_39.3.0.1.2 = Move(_16);
(*_25) = !_23.fld5;
Goto(bb41)
}
bb41 = {
_39.3.0.1.0 = -(*_43);
(*_43) = -_39.3.0.1.0;
_46 = _24;
Goto(bb42)
}
bb42 = {
_12 = -_30;
_33 = _23.fld2;
_29 = core::ptr::addr_of_mut!(place!(Field::<*const usize>(Variant(_48.0.1.2, 0), 1)));
_51 = _23.fld2 * _23.fld2;
_18.0 = Move(_39.3.0);
(*_25) = _9 * _23.fld5;
Goto(bb43)
}
bb43 = {
_4 = (*_43);
_54.fld1 = _23.fld1 << (*_25);
(*_43) = _4 ^ _4;
(*_25) = _23.fld5 >> (*_43);
_18.0.3 = _23.fld1 as u64;
_21 = _7 as u64;
_38.2.1 = Adt29::Variant0 { fld0: _18.0.2,fld1: Move(Field::<*const usize>(Variant(_18.0.1.2, 0), 1)),fld2: Field::<f64>(Variant(_48.0.1.2, 0), 2),fld3: Field::<i32>(Variant(_48.0.1.2, 0), 3) };
(*_43) = _4 | _4;
_58.1.2 = Field::<i128>(Variant(_48.0.1.2, 0), 0) & Field::<i128>(Variant(_48.0.1.2, 0), 0);
(*_29) = core::ptr::addr_of!((*_25));
_54 = Adt18 { fld0: _18.0.0,fld1: _23.fld1,fld2: _51,fld3: Move((*_29)),fld4: _12,fld5: _23.fld5 };
place!(Field::<f64>(Variant(_48.0.1.2, 0), 2)) = _14 + Field::<f64>(Variant(_38.2.1, 0), 2);
_38.3 = [Field::<i128>(Variant(_18.0.1.2, 0), 0)];
(*_29) = core::ptr::addr_of!((*_25));
place!(Field::<i128>(Variant(_18.0.1.2, 0), 0)) = Field::<i128>(Variant(_38.2.1, 0), 0) + _8;
_48.0.1.1 = _18.0.1.1;
_39.3.0.1 = ((*_43), _48.0.1.1, Move(_38.2.1));
_40 = (*_25);
(*_25) = _54.fld5 | _40;
place!(Field::<*const usize>(Variant(_18.0.1.2, 0), 1)) = core::ptr::addr_of!((*_25));
Goto(bb44)
}
bb44 = {
(*_29) = core::ptr::addr_of!(_54.fld5);
place!(Field::<*const usize>(Variant(_39.3.0.1.2, 0), 1)) = core::ptr::addr_of!((*_25));
_26 = _18.0.1.1 as u64;
(*_25) = 308022555721212236968806349739996796910_u128 as usize;
_16 = Adt29::Variant0 { fld0: _58.1.2,fld1: Move((*_29)),fld2: Field::<f64>(Variant(_48.0.1.2, 0), 2),fld3: Field::<i32>(Variant(_48.0.1.2, 0), 3) };
(*_29) = Move(_54.fld3);
_32 = (*_43);
(*_29) = Move(Field::<*const usize>(Variant(_16, 0), 1));
_3 = _51 - _33;
_28 = _39.3.0.1.1 - _48.0.1.1;
_38.2.2 = (*_25) * (*_25);
_49 = &mut (*_43);
(*_25) = _40;
_23.fld4 = _5 | _30;
(*_29) = core::ptr::addr_of!(_23.fld5);
_39.3.0.1.2 = Move(_48.0.1.2);
_19.0 = core::ptr::addr_of_mut!(_58.1.2);
_41 = core::ptr::addr_of_mut!((*_25));
(*_25) = _1 as usize;
match _23.fld1 {
0 => bb40,
20 => bb46,
_ => bb45
}
}
bb45 = {
_15 = _10 as f64;
_12 = _7 as i16;
_10 = 673968424_u32 & 1793494801_u32;
_7 = _11 as i64;
_2 = 11053161193491778320_u64;
_11 = _7 as f32;
_9 = RET as usize;
_18.0.3 = _2 ^ _2;
_18.0.0 = _1 & _1;
_18.0.1.0 = _18.0.3 as i8;
_15 = 154_u16 as f64;
_4 = _18.0.1.0 ^ _18.0.1.0;
_18.0.2 = _8;
_12 = _5 << _18.0.1.0;
_18.0.1.1 = 35006_u16;
_10 = 527700466_u32 * 1364176837_u32;
_1 = _18.0.0 ^ _18.0.0;
match _18.0.1.1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
35006 => bb12,
_ => bb11
}
}
bb46 = {
(*_25) = _23.fld5 & _54.fld5;
_17 = _38.3;
_64.0 = !_1;
place!(Field::<i128>(Variant(_39.3.0.1.2, 0), 0)) = Field::<i128>(Variant(_16, 0), 0) - Field::<i128>(Variant(_16, 0), 0);
(*_25) = _9 & _40;
_48.0.1.1 = !_39.3.0.1.1;
Goto(bb47)
}
bb47 = {
_38.1 = _48.0.1.1 ^ _28;
_58.3 = &_19.0;
place!(Field::<i32>(Variant(_39.3.0.1.2, 0), 3)) = _6 << (*_25);
(*_29) = core::ptr::addr_of!((*_25));
_38.0 = core::ptr::addr_of!((*_25));
_38.2.1 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_39.3.0.1.2, 0), 0),fld1: Move((*_29)),fld2: _14,fld3: Field::<i32>(Variant(_39.3.0.1.2, 0), 3) };
match _23.fld1 {
0 => bb48,
1 => bb49,
20 => bb51,
_ => bb50
}
}
bb48 = {
(*_25) = _23.fld5 & _54.fld5;
_17 = _38.3;
_64.0 = !_1;
place!(Field::<i128>(Variant(_39.3.0.1.2, 0), 0)) = Field::<i128>(Variant(_16, 0), 0) - Field::<i128>(Variant(_16, 0), 0);
(*_25) = _9 & _40;
_48.0.1.1 = !_39.3.0.1.1;
Goto(bb47)
}
bb49 = {
_20 = !_1;
_18.0.2 = !RET;
_23.fld2 = _3;
_13 = _9;
_9 = _13 << _13;
_21 = 33_u8 as u64;
_20 = _11 < _11;
_18.0.2 = RET | _8;
_23.fld0 = _20 >= _18.0.0;
_14 = _15 + _15;
_18.0.3 = _21;
_4 = 849151507881715793177732033880119736_u128 as i8;
_17 = [_18.0.2];
_18.0.0 = !_1;
_21 = _18.0.3 * _2;
_18.0.1.0 = _6 as i8;
RET = 22238776234495427330002769078832581042_u128 as i128;
_21 = _18.0.3;
_23.fld2 = _3;
match _5 {
0 => bb15,
1 => bb2,
2 => bb3,
3 => bb16,
4 => bb14,
5 => bb9,
6 => bb18,
340282366920938463463374607431768208177 => bb20,
_ => bb19
}
}
bb50 = {
_4 = 86278104952794224553024382060028602673_u128 as i8;
_38.1 = !_18.0.1.1;
_36 = &_18.0.2;
(*_43) = -_4;
(*_25) = _23.fld1 as usize;
(*_25) = _23.fld5 & _23.fld5;
_28 = _18.0.1.1 << _21;
_39.3.0 = Move(_18.0);
(*_43) = _4 & _39.3.0.1.0;
_38.2.1 = Move(_39.3.0.1.2);
_34 = 145044464079495011969533618893071880601_u128 as f32;
_43 = core::ptr::addr_of!((*_43));
_34 = _42 + _42;
_48.0.1.2 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_38.2.1, 0), 0),fld1: Move(Field::<*const usize>(Variant(_38.2.1, 0), 1)),fld2: Field::<f64>(Variant(_38.2.1, 0), 2),fld3: _6 };
_17 = [Field::<i128>(Variant(_38.2.1, 0), 0)];
(*_25) = _34 as usize;
Goto(bb40)
}
bb51 = {
_48.0.2 = Field::<i128>(Variant(_38.2.1, 0), 0);
_63 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_16, 0), 2)));
(*_63) = Field::<f64>(Variant(_39.3.0.1.2, 0), 2) - Field::<f64>(Variant(_39.3.0.1.2, 0), 2);
(*_29) = core::ptr::addr_of!(_40);
(*_29) = core::ptr::addr_of!((*_25));
_23 = Adt18 { fld0: _1,fld1: _54.fld1,fld2: _3,fld3: Move((*_29)),fld4: _5,fld5: (*_25) };
_64.3 = _26;
_23.fld2 = _54.fld2;
(*_29) = core::ptr::addr_of!(_9);
_48.0.3 = _21 >> Field::<i32>(Variant(_39.3.0.1.2, 0), 3);
(*_49) = _32 << Field::<i32>(Variant(_38.2.1, 0), 3);
Goto(bb52)
}
bb52 = {
Call(_70 = dump_var(Move(_21), Move(_20), Move(_1), Move(_7)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_70 = dump_var(Move(_6), Move(_8), Move(_9), Move(_5)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_70 = dump_var(Move(_4), Move(_24), Move(_10), Move(_3)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i16,mut _2: usize,mut _3: i128,mut _4: i128,mut _5: usize,mut _6: i16,mut _7: bool,mut _8: i128,mut _9: i128,mut _10: usize,mut _11: i128) -> u64 {
mir! {
type RET = u64;
let _12: u64;
let _13: [bool; 2];
let _14: &'static &'static Adt18;
let _15: *const usize;
let _16: Adt48;
let _17: *const *const &'static mut u128;
let _18: &'static mut u32;
let _19: i16;
let _20: &'static mut i8;
let _21: isize;
let _22: i32;
let _23: &'static mut i8;
let _24: *const [bool; 1];
let _25: [i128; 2];
let _26: char;
let _27: &'static Adt48;
let _28: [u64; 2];
let _29: &'static i128;
let _30: &'static mut i16;
let _31: f32;
let _32: (f32, i128);
let _33: *mut usize;
let _34: ();
let _35: ();
{
_3 = !_4;
_4 = _8;
_2 = _5;
_5 = _10 + _10;
_1 = -_6;
_2 = _5;
RET = 3563779500454784560_u64 & 13545548017369594235_u64;
_8 = _11 + _9;
_8 = _4 | _11;
_6 = _1 | _1;
_5 = !_2;
_1 = _6 >> _9;
RET = (-800525353899774395_i64) as u64;
_12 = _1 as u64;
_9 = _11 >> _11;
_2 = _5;
_1 = _6;
_13 = [_7,_7];
_3 = _9;
_6 = _1 ^ _1;
_3 = !_11;
_4 = -_8;
_15 = core::ptr::addr_of!(_10);
_7 = false & true;
_15 = core::ptr::addr_of!((*_15));
(*_15) = !_2;
Call((*_15) = fn2(Move(_15), RET, _5, _5, _6, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _4 >> _11;
_11 = _3 & _9;
_15 = core::ptr::addr_of!(_10);
(*_15) = 6508899260769296894_i64 as usize;
(*_15) = _2 >> _12;
(*_15) = _5 << _11;
_8 = !_11;
_5 = (*_15);
(*_15) = _5 & _5;
_21 = (-61_isize);
_8 = _11 + _3;
_10 = _5 ^ _5;
(*_15) = !_5;
_5 = (*_15);
_22 = (-845984071_i32) << (*_15);
(*_15) = _2 >> _8;
_22 = 1970117093_i32 ^ (-836316689_i32);
_6 = _8 as i16;
match _21 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211395 => bb6,
_ => bb5
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
(*_15) = _5 - _5;
(*_15) = _5;
_11 = _3 >> (*_15);
(*_15) = _5 << _2;
Goto(bb7)
}
bb7 = {
(*_15) = _5;
(*_15) = !_2;
(*_15) = _5 | _5;
(*_15) = !_5;
_28 = [_12,RET];
(*_15) = _5;
(*_15) = _2;
_26 = '\u{23a92}';
_25 = [_11,_11];
match _21 {
0 => bb1,
1 => bb6,
340282366920938463463374607431768211395 => bb9,
_ => bb8
}
}
bb8 = {
(*_15) = _5 - _5;
(*_15) = _5;
_11 = _3 >> (*_15);
(*_15) = _5 << _2;
Goto(bb7)
}
bb9 = {
_19 = !_6;
_28 = [_12,_12];
_26 = '\u{55cab}';
(*_15) = _5;
_10 = _5 * _2;
(*_15) = _5;
RET = _12 - _12;
_22 = (-1627761164_i32) - (-1697637234_i32);
_8 = _11 ^ _4;
(*_15) = !_5;
_7 = (*_15) < (*_15);
_7 = !true;
_6 = _19 * _19;
_26 = '\u{6dade}';
_9 = _3 ^ _11;
_5 = (*_15) | (*_15);
Goto(bb10)
}
bb10 = {
Call(_34 = dump_var(Move(_10), Move(_9), Move(_3), Move(_21)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_34 = dump_var(Move(_2), Move(_25), Move(_7), Move(_12)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_34 = dump_var(Move(_19), _35, _35, _35), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: *const usize,mut _2: u64,mut _3: usize,mut _4: usize,mut _5: i16,mut _6: i128) -> usize {
mir! {
type RET = usize;
let _7: bool;
let _8: &'static mut u32;
let _9: f32;
let _10: u64;
let _11: *const usize;
let _12: &'static &'static Adt18;
let _13: *const i8;
let _14: Adt26;
let _15: i128;
let _16: Adt75;
let _17: ();
let _18: ();
{
_1 = core::ptr::addr_of!(RET);
(*_1) = _4 - _4;
(*_1) = _3 >> _4;
(*_1) = _3 >> _2;
(*_1) = _4 | _4;
(*_1) = _5 as usize;
_10 = _2;
(*_1) = _3 & _4;
Goto(bb1)
}
bb1 = {
(*_1) = _3 * _4;
(*_1) = _6 as usize;
RET = _4;
(*_1) = 9223372036854775807_isize as usize;
(*_1) = _4;
_4 = !(*_1);
(*_1) = 22845254778034711917852419079824317887_u128 as usize;
(*_1) = !_3;
(*_1) = _6 as usize;
Call(_1 = fn3((*_1), (*_1), _3, (*_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1), _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = -(-67423354966033297710167217623448060153_i128);
_9 = _6 as f32;
_11 = core::ptr::addr_of!(RET);
(*_11) = _3 * _3;
(*_11) = _2 as usize;
_7 = (*_11) < (*_11);
(*_11) = _4;
_1 = core::ptr::addr_of!((*_11));
(*_11) = _4 - _4;
(*_11) = _4 ^ _3;
(*_11) = _4;
_6 = (-59192198968636413514902554765573798396_i128);
(*_11) = _3 | _3;
_3 = (*_11);
(*_11) = _3;
_6 = (-67002258416996050909499961777396476982_i128);
(*_11) = (-87_isize) as usize;
(*_11) = _3 | _4;
(*_11) = _3;
_6 = (-169848356832948088548032630155684128020_i128) & (-46953523288622496226880703026885181951_i128);
_6 = (-130682337560483416680585334867616561493_i128) + (-94577071316283521661553651447187080073_i128);
(*_11) = _3 << _4;
_6 = 51631540654894362636957221097616964184_i128 << (*_11);
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(Move(_5), Move(_6), Move(_3), _18), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize) -> *const usize {
mir! {
type RET = *const usize;
let _12: &'static mut u32;
let _13: f64;
let _14: i8;
let _15: &'static mut f32;
let _16: Adt26;
let _17: i128;
let _18: &'static mut f32;
let _19: &'static mut i8;
let _20: u128;
let _21: Adt63;
let _22: Adt29;
let _23: *mut usize;
let _24: f32;
let _25: &'static i128;
let _26: bool;
let _27: &'static *mut i128;
let _28: *const *const &'static mut u128;
let _29: *const i8;
let _30: bool;
let _31: Adt51;
let _32: char;
let _33: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _34: bool;
let _35: &'static Adt18;
let _36: [i128; 1];
let _37: &'static Adt48;
let _38: *mut i128;
let _39: u8;
let _40: i64;
let _41: &'static Adt18;
let _42: isize;
let _43: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _44: bool;
let _45: f32;
let _46: char;
let _47: *const (u32,);
let _48: *const *const &'static mut u128;
let _49: (&'static u128, Adt29, usize);
let _50: &'static mut u128;
let _51: ();
let _52: ();
{
_1 = _4 & _11;
RET = core::ptr::addr_of!(_1);
_8 = (-107_i8) as usize;
(*RET) = _11;
(*RET) = _9;
_11 = !(*RET);
Call((*RET) = fn4(_4, _2, Move(RET), _7, _7, _2, _3, _2, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_1;
_2 = _1;
_4 = '\u{35dc7}' as usize;
_11 = _2;
_5 = _2 * _6;
_1 = 67519699791294640541502897082515446654_u128 as usize;
_5 = _2;
_11 = _10 | _4;
_8 = _3 | _3;
Goto(bb2)
}
bb2 = {
_1 = 8756378581174154015_u64 as usize;
Goto(bb3)
}
bb3 = {
_4 = _7 + _5;
_11 = !_4;
_1 = 975599723_u32 as usize;
_5 = _9 | _8;
_8 = _11 + _5;
_6 = _8 | _8;
_4 = (-1469326408325296263_i64) as usize;
_2 = _6 | _8;
_13 = 4121921442719820976_u64 as f64;
_5 = '\u{7f36a}' as usize;
_1 = !_2;
_11 = _2 << _1;
_8 = (-34_i8) as usize;
_6 = _1 ^ _1;
_8 = _6 >> _2;
_2 = _8 & _7;
Goto(bb4)
}
bb4 = {
_7 = _1;
_13 = 613959182_i32 as f64;
RET = core::ptr::addr_of!(_10);
(*RET) = (-38_i8) as usize;
(*RET) = '\u{53b7b}' as usize;
_10 = _11 >> _6;
(*RET) = _6;
(*RET) = _7 - _11;
(*RET) = _8 ^ _11;
_16 = Adt26::Variant2 { fld0: (-1199166812_i32),fld1: 59368_u16 };
(*RET) = !_3;
Call(_4 = core::intrinsics::bswap(_6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = _2;
(*RET) = _8 >> _11;
_4 = !_11;
_14 = (-49_i8) >> _6;
(*RET) = !_11;
_8 = (*RET);
Goto(bb6)
}
bb6 = {
(*RET) = _4 | _6;
_1 = (*RET) >> (*RET);
_2 = !(*RET);
_9 = (*RET);
_17 = (-81451921002395570464967342311331269255_i128);
(*RET) = 406259694_u32 as usize;
place!(Field::<u16>(Variant(_16, 2), 1)) = _17 as u16;
RET = core::ptr::addr_of!((*RET));
Call(_20 = core::intrinsics::transmute(_17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*RET) = !_4;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
258830445918542892998407265120436942201 => bb10,
_ => bb9
}
}
bb8 = {
(*RET) = _4 | _6;
_1 = (*RET) >> (*RET);
_2 = !(*RET);
_9 = (*RET);
_17 = (-81451921002395570464967342311331269255_i128);
(*RET) = 406259694_u32 as usize;
place!(Field::<u16>(Variant(_16, 2), 1)) = _17 as u16;
RET = core::ptr::addr_of!((*RET));
Call(_20 = core::intrinsics::transmute(_17), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_4 = _7 + _5;
_11 = !_4;
_1 = 975599723_u32 as usize;
_5 = _9 | _8;
_8 = _11 + _5;
_6 = _8 | _8;
_4 = (-1469326408325296263_i64) as usize;
_2 = _6 | _8;
_13 = 4121921442719820976_u64 as f64;
_5 = '\u{7f36a}' as usize;
_1 = !_2;
_11 = _2 << _1;
_8 = (-34_i8) as usize;
_6 = _1 ^ _1;
_8 = _6 >> _2;
_2 = _8 & _7;
Goto(bb4)
}
bb10 = {
(*RET) = !_4;
_19 = &mut _14;
(*_19) = (-51_i8) & (-126_i8);
(*RET) = _2 | _4;
(*_19) = 11_i8;
(*RET) = _9 ^ _7;
(*_19) = 66_i8 >> _11;
(*RET) = _11 >> _7;
_2 = (*RET) >> (*RET);
_8 = _6 >> (*RET);
_6 = (*RET) >> (*_19);
(*RET) = !_2;
(*_19) = 100_i8;
(*RET) = _11 - _4;
(*_19) = 35_i8 + (-40_i8);
(*_19) = 62_i8 + (-21_i8);
(*_19) = -(-85_i8);
_20 = 334621451028281943513858795913013920352_u128 >> (*RET);
(*RET) = _8 | _4;
_1 = (*RET) * (*RET);
(*RET) = _4 << _20;
(*RET) = _17 as usize;
(*_19) = 103_i8 * 98_i8;
place!(Field::<i32>(Variant(_16, 2), 0)) = -590555959_i32;
(*_19) = 97_i8;
(*_19) = 97_i8;
(*_19) = 38_i8 ^ 7_i8;
place!(Field::<u16>(Variant(_16, 2), 1)) = 30648_u16;
(*_19) = !41_i8;
(*RET) = (*_19) as usize;
Goto(bb11)
}
bb11 = {
_24 = 5971_i16 as f32;
(*_19) = (-112_i8);
_11 = _17 as usize;
(*_19) = (-76_i8);
(*_19) = 167_u8 as i8;
place!(Field::<i32>(Variant(_16, 2), 0)) = 1521176491_i32 << _8;
(*_19) = '\u{d4777}' as i8;
(*RET) = _4 & _9;
_18 = &mut _24;
(*_19) = -(-68_i8);
_4 = (*RET) & (*RET);
_26 = !false;
(*_18) = 251_u8 as f32;
Goto(bb12)
}
bb12 = {
(*_18) = _20 as f32;
(*_19) = 26_u8 as i8;
(*_18) = 26_u8 as f32;
(*_19) = (-128_i8) + (-66_i8);
(*_18) = (-9223372036854775808_isize) as f32;
Goto(bb13)
}
bb13 = {
(*_18) = 13432_i16 as f32;
(*_19) = 20_i8 * (-38_i8);
_23 = core::ptr::addr_of_mut!((*RET));
_25 = &_17;
_9 = (-20500_i16) as usize;
(*_23) = !_1;
(*_19) = 76_i8 << (*_23);
_23 = core::ptr::addr_of_mut!((*RET));
Goto(bb14)
}
bb14 = {
_30 = (*_19) < (*_19);
(*RET) = _6 | _1;
(*_19) = (-28_i8) >> _1;
(*RET) = !_1;
(*_19) = !(-51_i8);
(*_18) = _6 as f32;
(*_19) = 16_i8;
(*_18) = (*_19) as f32;
(*RET) = _4;
(*RET) = _2 | _6;
(*_18) = _13 as f32;
(*_18) = 6010868947165069375_i64 as f32;
(*_19) = !68_i8;
(*_18) = 1365470682_u32 as f32;
(*_18) = 28_isize as f32;
(*_19) = (-32_i8) - (-121_i8);
_33.2 = [_30];
(*_19) = (*_18) as i8;
(*_19) = 101_i8;
_33.3.0.2 = !(*_25);
(*_18) = 860104132928690229_u64 as f32;
(*_19) = 75_i8 >> _6;
_33.3.0.1.1 = Field::<u16>(Variant(_16, 2), 1);
_17 = _33.3.0.2;
_33.3.0.3 = 3772_i16 as u64;
_33.1 = -Field::<i32>(Variant(_16, 2), 0);
match Field::<u16>(Variant(_16, 2), 1) {
30648 => bb15,
_ => bb11
}
}
bb15 = {
_30 = !_26;
(*RET) = _1 - _4;
(*RET) = (*_18) as usize;
(*RET) = !_2;
_33.3.0.2 = _20 as i128;
_33.1 = -Field::<i32>(Variant(_16, 2), 0);
_22 = Adt29::Variant0 { fld0: _33.3.0.2,fld1: Move(RET),fld2: _13,fld3: Field::<i32>(Variant(_16, 2), 0) };
match Field::<u16>(Variant(_16, 2), 1) {
0 => bb1,
1 => bb7,
2 => bb11,
3 => bb4,
4 => bb13,
5 => bb16,
30648 => bb18,
_ => bb17
}
}
bb16 = {
(*_18) = _20 as f32;
(*_19) = 26_u8 as i8;
(*_18) = 26_u8 as f32;
(*_19) = (-128_i8) + (-66_i8);
(*_18) = (-9223372036854775808_isize) as f32;
Goto(bb13)
}
bb17 = {
_4 = _7 + _5;
_11 = !_4;
_1 = 975599723_u32 as usize;
_5 = _9 | _8;
_8 = _11 + _5;
_6 = _8 | _8;
_4 = (-1469326408325296263_i64) as usize;
_2 = _6 | _8;
_13 = 4121921442719820976_u64 as f64;
_5 = '\u{7f36a}' as usize;
_1 = !_2;
_11 = _2 << _1;
_8 = (-34_i8) as usize;
_6 = _1 ^ _1;
_8 = _6 >> _2;
_2 = _8 & _7;
Goto(bb4)
}
bb18 = {
place!(Field::<*const usize>(Variant(_22, 0), 1)) = core::ptr::addr_of!(_3);
(*_18) = 29000_i16 as f32;
_29 = core::ptr::addr_of!((*_19));
_3 = _2;
(*_29) = !71_i8;
_15 = &mut (*_18);
match _33.3.0.1.1 {
0 => bb13,
1 => bb5,
2 => bb14,
3 => bb12,
30648 => bb20,
_ => bb19
}
}
bb19 = {
_30 = !_26;
(*RET) = _1 - _4;
(*RET) = (*_18) as usize;
(*RET) = !_2;
_33.3.0.2 = _20 as i128;
_33.1 = -Field::<i32>(Variant(_16, 2), 0);
_22 = Adt29::Variant0 { fld0: _33.3.0.2,fld1: Move(RET),fld2: _13,fld3: Field::<i32>(Variant(_16, 2), 0) };
match Field::<u16>(Variant(_16, 2), 1) {
0 => bb1,
1 => bb7,
2 => bb11,
3 => bb4,
4 => bb13,
5 => bb16,
30648 => bb18,
_ => bb17
}
}
bb20 = {
_38 = core::ptr::addr_of_mut!(_33.3.0.2);
_36 = [(*_38)];
_30 = !_26;
_33.3.0.1.2 = Move(_22);
(*_19) = (*_15) as i8;
(*_19) = !50_i8;
place!(Field::<f64>(Variant(_33.3.0.1.2, 0), 2)) = -_13;
_18 = Move(_15);
_27 = &_38;
(*_38) = Field::<i128>(Variant(_33.3.0.1.2, 0), 0) - Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
(*_38) = Field::<i128>(Variant(_33.3.0.1.2, 0), 0) * Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
_36 = [(*_38)];
_29 = core::ptr::addr_of!((*_19));
(*_38) = -Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
(*_19) = !19_i8;
(*_19) = _1 as i8;
_4 = _6 << (*_38);
Goto(bb21)
}
bb21 = {
(*_19) = -54_i8;
(*_19) = (-41_i8) - 33_i8;
(*_38) = Field::<i128>(Variant(_33.3.0.1.2, 0), 0) | Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
_33.3.0.0 = _30;
_43.2.2 = _10 * _7;
_22 = Adt29::Variant0 { fld0: (*_38),fld1: Move(Field::<*const usize>(Variant(_33.3.0.1.2, 0), 1)),fld2: _13,fld3: Field::<i32>(Variant(_16, 2), 0) };
_43.0 = core::ptr::addr_of!(_1);
_33.3.0.1 = ((*_19), Field::<u16>(Variant(_16, 2), 1), Move(_22));
place!(Field::<*const usize>(Variant(_33.3.0.1.2, 0), 1)) = core::ptr::addr_of!(_11);
_6 = _2;
(*_38) = Field::<i128>(Variant(_33.3.0.1.2, 0), 0) >> _8;
_45 = Field::<u16>(Variant(_16, 2), 1) as f32;
_30 = !_33.3.0.0;
_33.2 = [_33.3.0.0];
_44 = (*_38) > (*_38);
_18 = &mut _45;
_32 = '\u{82a2b}';
_15 = &mut (*_18);
match Field::<u16>(Variant(_16, 2), 1) {
0 => bb16,
1 => bb17,
2 => bb9,
3 => bb14,
4 => bb22,
30648 => bb24,
_ => bb23
}
}
bb22 = {
_8 = _2;
(*RET) = _8 >> _11;
_4 = !_11;
_14 = (-49_i8) >> _6;
(*RET) = !_11;
_8 = (*RET);
Goto(bb6)
}
bb23 = {
_24 = 5971_i16 as f32;
(*_19) = (-112_i8);
_11 = _17 as usize;
(*_19) = (-76_i8);
(*_19) = 167_u8 as i8;
place!(Field::<i32>(Variant(_16, 2), 0)) = 1521176491_i32 << _8;
(*_19) = '\u{d4777}' as i8;
(*RET) = _4 & _9;
_18 = &mut _24;
(*_19) = -(-68_i8);
_4 = (*RET) & (*RET);
_26 = !false;
(*_18) = 251_u8 as f32;
Goto(bb12)
}
bb24 = {
_26 = (*_38) == (*_38);
_43.2.0 = &_20;
_26 = _44;
(*_19) = _33.3.0.1.0 ^ _33.3.0.1.0;
_6 = 366036491_u32 as usize;
_39 = !130_u8;
RET = core::ptr::addr_of!(_10);
_40 = 2135193419446578907_i64 << Field::<i32>(Variant(_33.3.0.1.2, 0), 3);
(*RET) = _8 - _8;
(*_38) = !Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
(*_38) = Field::<i128>(Variant(_33.3.0.1.2, 0), 0) * Field::<i128>(Variant(_33.3.0.1.2, 0), 0);
(*_15) = _33.3.0.3 as f32;
_43.3 = [(*_38)];
(*_19) = _33.3.0.1.0 & _33.3.0.1.0;
(*_15) = _40 as f32;
Goto(bb25)
}
bb25 = {
Call(_51 = dump_var(Move(_26), Move(_40), Move(_20), Move(_14)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_51 = dump_var(Move(_4), Move(_30), Move(_1), Move(_7)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_51 = dump_var(Move(_10), Move(_8), _52, _52), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: usize,mut _2: usize,mut _3: *const usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize) -> usize {
mir! {
type RET = usize;
let _10: isize;
let _11: *mut i128;
let _12: *mut u64;
let _13: bool;
let _14: *const usize;
let _15: f64;
let _16: f64;
let _17: ();
let _18: ();
{
_3 = core::ptr::addr_of!(_9);
(*_3) = _1 | _6;
_7 = (-9223372036854775808_isize) as usize;
_6 = _1 ^ _8;
(*_3) = _4 + _6;
(*_3) = !_1;
(*_3) = _8 ^ _4;
(*_3) = !_1;
Goto(bb1)
}
bb1 = {
_9 = _5 - _1;
_9 = _6 ^ _5;
(*_3) = !_8;
(*_3) = !_2;
Call(_11 = fn5(_1, Move(_3), (*_3), (*_3), (*_3), (*_3), _7, _7, (*_3), (*_3), (*_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 18209_i16 as usize;
_4 = 50_i8 as usize;
_9 = _5 >> _6;
RET = _6 << _9;
_13 = true;
_2 = _6 + _5;
_5 = _8;
_3 = core::ptr::addr_of!(_1);
(*_3) = RET ^ RET;
(*_3) = !_6;
_7 = (*_3);
_14 = core::ptr::addr_of!(_9);
_10 = -9223372036854775807_isize;
_9 = _6 & (*_3);
(*_14) = !_8;
_14 = core::ptr::addr_of!((*_3));
(*_3) = _8;
(*_3) = RET;
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(Move(_7), Move(_8), Move(_4), Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_17 = dump_var(Move(_9), _18, _18, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: *const usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize) -> *mut i128 {
mir! {
type RET = *mut i128;
let _12: *mut u32;
let _13: usize;
let _14: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _15: char;
let _16: u32;
let _17: char;
let _18: bool;
let _19: *const &'static mut u128;
let _20: ();
let _21: ();
{
_3 = _1 << _6;
_4 = !_9;
_3 = _5 >> _1;
_4 = 85_u8 as usize;
_11 = _3 * _4;
_4 = _11;
_10 = false as usize;
_4 = _9;
_8 = _3;
_11 = _6 & _3;
_13 = _11 + _5;
Call(_9 = fn6(_5, Move(_2), _8, _1, _10, _10, _10, _3, _5, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = false as usize;
Call(RET = fn7(_7, _1, _8, _3, _4, _10, _3, _6, _5, _9, _5, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _6 | _1;
_2 = core::ptr::addr_of!(_8);
(*_2) = (-42787189437060939726689436800712770820_i128) as usize;
(*_2) = _4 & _3;
(*_2) = _9;
_17 = '\u{17fbc}';
(*_2) = _7 + _5;
(*_2) = (-4225504220424409663_i64) as usize;
(*_2) = _7;
_3 = _13;
(*_2) = !_11;
(*_2) = _11 ^ _3;
(*_2) = !_5;
_13 = (*_2) << (*_2);
_14.3 = [(-94930429124135509462327379899563198379_i128)];
(*_2) = !_3;
_8 = _11 - _13;
_14.2.2 = (*_2);
(*_2) = _5 << _14.2.2;
_7 = _11;
(*_2) = _3 << _7;
_16 = 3562496721_u32 | 1995634471_u32;
(*_2) = 287337424_i32 as usize;
_14.2.2 = 199_u8 as usize;
_7 = !(*_2);
_13 = _3;
(*_2) = _4;
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(Move(_9), Move(_11), Move(_16), Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_20 = dump_var(Move(_1), Move(_3), _21, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: usize,mut _2: *const usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize) -> usize {
mir! {
type RET = usize;
let _12: f32;
let _13: isize;
let _14: (Adt26, (i8, u16, Adt29), (u64, char, u8, i16), (u32,));
let _15: &'static i128;
let _16: u64;
let _17: ();
let _18: ();
{
_2 = core::ptr::addr_of!(_3);
_2 = core::ptr::addr_of!((*_2));
RET = (*_2);
_12 = 9223372036854775807_isize as f32;
_14.3.0 = 2061994482_u32;
_4 = (-3520961058344717864_i64) as usize;
RET = _10 & (*_2);
_1 = (*_2) + _8;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(Move(_11), Move(_3), Move(_1), Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(Move(_5), _18, _18, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize,mut _12: usize) -> *mut i128 {
mir! {
type RET = *mut i128;
let _13: i64;
let _14: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _15: *const *const &'static mut u128;
let _16: (bool, [i8; 7]);
let _17: f64;
let _18: f32;
let _19: &'static mut [bool; 2];
let _20: *const &'static mut u128;
let _21: i128;
let _22: ();
let _23: ();
{
_9 = 29351160535123059242490391944088041629_u128 as usize;
_11 = '\u{42861}' as usize;
_2 = _3 * _9;
_3 = _7 >> _4;
_9 = 20032_u16 as usize;
_7 = _10 >> _3;
_13 = (-3066084050808867792_i64) - 9099385297517848835_i64;
_7 = _4 >> _8;
_16.0 = _7 >= _3;
_16.1 = [(-97_i8),30_i8,104_i8,(-100_i8),(-47_i8),(-67_i8),29_i8];
_3 = !_10;
_16.1 = [42_i8,(-9_i8),(-117_i8),24_i8,16_i8,(-87_i8),64_i8];
_11 = _5;
_10 = !_2;
_16.0 = false;
_6 = !_2;
_4 = !_11;
_16.1 = [64_i8,(-93_i8),103_i8,(-30_i8),86_i8,33_i8,(-48_i8)];
_11 = _7 * _9;
_9 = _5 * _10;
_8 = 361617682_i32 as usize;
_7 = _12;
_9 = _6 << _3;
Call(RET = fn8(_10, _7, _13, _2, _16, _9, _16.0, _11, _5, _7, _16.0, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _10;
_17 = _11 as f64;
_6 = _3 << _5;
_4 = 116411846197535365691292420127815010710_i128 as usize;
_4 = _9 ^ _3;
_18 = (-942693813_i32) as f32;
_2 = _9 ^ _4;
_5 = _11 - _4;
_2 = _11 - _5;
Goto(bb2)
}
bb2 = {
Call(_22 = dump_var(Move(_1), Move(_16), Move(_2), Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_22 = dump_var(Move(_9), Move(_10), Move(_11), _23), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: usize,mut _2: usize,mut _3: i64,mut _4: usize,mut _5: (bool, [i8; 7]),mut _6: usize,mut _7: bool,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: bool,mut _12: usize) -> *mut i128 {
mir! {
type RET = *mut i128;
let _13: i32;
let _14: ([bool; 1],);
let _15: char;
let _16: ((*mut i128,),);
let _17: [i32; 7];
let _18: *mut u64;
let _19: [u128; 7];
let _20: (u64, char, u8, i16);
let _21: f64;
let _22: u128;
let _23: char;
let _24: Adt51;
let _25: &'static Adt18;
let _26: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _27: *mut u64;
let _28: &'static mut &'static mut &'static &'static Adt18;
let _29: *mut &'static i128;
let _30: isize;
let _31: u8;
let _32: Adt25;
let _33: *mut *const usize;
let _34: isize;
let _35: *const [bool; 1];
let _36: f32;
let _37: usize;
let _38: [u64; 1];
let _39: i64;
let _40: (bool, [i8; 7]);
let _41: *mut &'static i128;
let _42: &'static mut u128;
let _43: &'static mut i8;
let _44: &'static mut i8;
let _45: f32;
let _46: &'static mut u128;
let _47: &'static mut u128;
let _48: Adt29;
let _49: u128;
let _50: usize;
let _51: f32;
let _52: f32;
let _53: Adt18;
let _54: i128;
let _55: f64;
let _56: *mut u64;
let _57: isize;
let _58: [i16; 2];
let _59: &'static i128;
let _60: f32;
let _61: Adt63;
let _62: char;
let _63: i32;
let _64: &'static mut i16;
let _65: Adt48;
let _66: char;
let _67: &'static i128;
let _68: Adt18;
let _69: f32;
let _70: *const &'static mut u128;
let _71: &'static mut f32;
let _72: ();
let _73: ();
{
_7 = _6 > _12;
_6 = _8 | _1;
_5.0 = _6 != _6;
_11 = !_7;
_8 = _12 << _6;
_14.0 = [_11];
_8 = !_6;
_2 = !_6;
_2 = _8 - _9;
_4 = 39431_u16 as usize;
_2 = _6 >> _12;
_6 = _11 as usize;
_5.1 = [55_i8,31_i8,(-6_i8),(-34_i8),(-67_i8),92_i8,30_i8];
_3 = (-6608595876546158078_i64);
_3 = 211_u8 as i64;
_5.1 = [113_i8,(-109_i8),(-45_i8),(-61_i8),76_i8,(-3_i8),35_i8];
_4 = _6 >> _8;
_15 = '\u{dfc2f}';
_1 = _4;
_2 = _6 * _4;
_3 = (-5565139360641112180_i64);
_12 = _15 as usize;
_14.0 = [_5.0];
_3 = !(-4726376157105296225_i64);
_3 = 3191211249209850222_i64 - (-8424413896785796134_i64);
_5.1 = [127_i8,(-102_i8),63_i8,17_i8,126_i8,92_i8,103_i8];
Call(_15 = fn9(_14.0, _7, _2, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = _5.0 & _5.0;
_15 = '\u{155ef}';
_8 = !_6;
_5.1 = [27_i8,(-13_i8),12_i8,112_i8,(-63_i8),(-64_i8),0_i8];
_5.1 = [(-10_i8),(-119_i8),(-107_i8),(-64_i8),(-49_i8),(-85_i8),(-109_i8)];
_2 = _1 & _1;
_19 = [294670442978308788197858293373636447680_u128,271444416854365938563989637155709861789_u128,291690899833143921564304442171395041540_u128,285122096823576315361757009704419607339_u128,261149901663204517374890271391222210219_u128,56025063190157374101918571005641354295_u128,90444954224372703781221037580883206906_u128];
_20.3 = (-4729_i16);
_14.0 = [_11];
_20 = (17394086790974935342_u64, _15, 79_u8, 4177_i16);
_13 = _20.0 as i32;
_1 = _8 >> _2;
_3 = 9223372036854775807_isize as i64;
_7 = !_5.0;
_21 = _20.0 as f64;
_12 = !_8;
_20.1 = _15;
_22 = _1 as u128;
_20.0 = _21 as u64;
match _20.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
79 => bb8,
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
_20.2 = 74_u8 - 127_u8;
_18 = core::ptr::addr_of_mut!(_20.0);
(*_18) = !2796442059753507118_u64;
_4 = _2 & _1;
_22 = !279042382199747086197032452354902329583_u128;
(*_18) = 15103020475468255188_u64 + 15243043094383666414_u64;
_20.2 = !58_u8;
(*_18) = 18392467069710133291_u64;
_11 = !_7;
(*_18) = 17019237000440050477_u64 & 897868041212770322_u64;
(*_18) = 24_i8 as u64;
match _20.3 {
0 => bb6,
1 => bb2,
2 => bb7,
4177 => bb9,
_ => bb4
}
}
bb9 = {
_15 = _20.1;
match _20.3 {
0 => bb3,
4177 => bb10,
_ => bb6
}
}
bb10 = {
(*_18) = 1605549928974772003_u64 << _2;
(*_18) = _20.2 as u64;
(*_18) = 37_isize as u64;
_18 = core::ptr::addr_of_mut!((*_18));
_20.3 = -18386_i16;
(*_18) = 16159847234133256547_u64;
_21 = 44141_u16 as f64;
_20.0 = 9_i8 as u64;
_22 = _21 as u128;
_23 = _20.1;
(*_18) = !17653948127073650521_u64;
(*_18) = 602056092814708423_u64 - 18113630247774677981_u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_9 = _6 ^ _1;
_19 = [_22,_22,_22,_22,_22,_22,_22];
Goto(bb11)
}
bb11 = {
(*_18) = 18324042865056496024_u64 | 6460031161951068940_u64;
(*_18) = _20.3 as u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_3 = (-3690318297002336942_i64) | 5744410512878669548_i64;
(*_18) = !4759103030741729766_u64;
_15 = _20.1;
(*_18) = 192235734841379597_u64;
(*_18) = !15246928159189540184_u64;
(*_18) = 12605033509124518118_u64;
_21 = (-141464648546102212317969447286249937966_i128) as f64;
_21 = _22 as f64;
_8 = _4;
(*_18) = !6538906444241749952_u64;
Goto(bb12)
}
bb12 = {
_5.0 = _7;
(*_18) = 13721050114021726987_u64;
_14.0 = [_5.0];
_27 = core::ptr::addr_of_mut!((*_18));
(*_27) = 7213168326154399853_u64;
(*_27) = !918325690751082863_u64;
_21 = _22 as f64;
(*_27) = 9582934275336730305_u64 | 16871631363498111875_u64;
(*_18) = 101_isize as u64;
_11 = _7;
_30 = _13 as isize;
_9 = _21 as usize;
(*_18) = 11111815847974123246_u64 >> _4;
_15 = _23;
_5.1 = [(-52_i8),(-83_i8),56_i8,(-68_i8),10_i8,(-1_i8),(-57_i8)];
(*_18) = 11738958218258425840_u64 - 6895958106734837988_u64;
(*_18) = 18300670670425509392_u64;
_5.0 = _7;
_1 = _6 ^ _4;
_5.1 = [(-99_i8),32_i8,(-40_i8),(-34_i8),(-68_i8),(-104_i8),114_i8];
match (*_18) {
0 => bb13,
1 => bb14,
18300670670425509392 => bb16,
_ => bb15
}
}
bb13 = {
(*_18) = 18324042865056496024_u64 | 6460031161951068940_u64;
(*_18) = _20.3 as u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_3 = (-3690318297002336942_i64) | 5744410512878669548_i64;
(*_18) = !4759103030741729766_u64;
_15 = _20.1;
(*_18) = 192235734841379597_u64;
(*_18) = !15246928159189540184_u64;
(*_18) = 12605033509124518118_u64;
_21 = (-141464648546102212317969447286249937966_i128) as f64;
_21 = _22 as f64;
_8 = _4;
(*_18) = !6538906444241749952_u64;
Goto(bb12)
}
bb14 = {
(*_18) = 1605549928974772003_u64 << _2;
(*_18) = _20.2 as u64;
(*_18) = 37_isize as u64;
_18 = core::ptr::addr_of_mut!((*_18));
_20.3 = -18386_i16;
(*_18) = 16159847234133256547_u64;
_21 = 44141_u16 as f64;
_20.0 = 9_i8 as u64;
_22 = _21 as u128;
_23 = _20.1;
(*_18) = !17653948127073650521_u64;
(*_18) = 602056092814708423_u64 - 18113630247774677981_u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_9 = _6 ^ _1;
_19 = [_22,_22,_22,_22,_22,_22,_22];
Goto(bb11)
}
bb15 = {
_11 = _5.0 & _5.0;
_15 = '\u{155ef}';
_8 = !_6;
_5.1 = [27_i8,(-13_i8),12_i8,112_i8,(-63_i8),(-64_i8),0_i8];
_5.1 = [(-10_i8),(-119_i8),(-107_i8),(-64_i8),(-49_i8),(-85_i8),(-109_i8)];
_2 = _1 & _1;
_19 = [294670442978308788197858293373636447680_u128,271444416854365938563989637155709861789_u128,291690899833143921564304442171395041540_u128,285122096823576315361757009704419607339_u128,261149901663204517374890271391222210219_u128,56025063190157374101918571005641354295_u128,90444954224372703781221037580883206906_u128];
_20.3 = (-4729_i16);
_14.0 = [_11];
_20 = (17394086790974935342_u64, _15, 79_u8, 4177_i16);
_13 = _20.0 as i32;
_1 = _8 >> _2;
_3 = 9223372036854775807_isize as i64;
_7 = !_5.0;
_21 = _20.0 as f64;
_12 = !_8;
_20.1 = _15;
_22 = _1 as u128;
_20.0 = _21 as u64;
match _20.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
79 => bb8,
_ => bb7
}
}
bb16 = {
_20.1 = _23;
(*_18) = 10651738338520557663_u64;
_31 = !_20.2;
(*_18) = _3 as u64;
_20.2 = _31 - _31;
_19 = [_22,_22,_22,_22,_22,_22,_22];
(*_18) = !6921216009058285572_u64;
(*_18) = 674790662820667233_u64 + 5179784541493620210_u64;
(*_18) = 3364848713556812439_u64 & 12793558736019515044_u64;
_22 = _4 as u128;
_15 = _23;
_32.fld0 = ((*_18), _23, _20.2, _20.3);
_32.fld0.3 = _20.3 >> _22;
_32.fld1 = 4391_u16 >> _12;
_5.1 = [(-17_i8),(-57_i8),(-77_i8),(-112_i8),(-16_i8),26_i8,32_i8];
_14.0 = [_7];
_9 = _15 as usize;
Goto(bb17)
}
bb17 = {
_32.fld0.2 = _20.2 | _20.2;
(*_18) = _32.fld0.0 * _32.fld0.0;
(*_18) = _32.fld0.0 ^ _32.fld0.0;
_20.3 = _32.fld0.3 - _32.fld0.3;
_32.fld0.1 = _20.1;
_2 = _4 | _12;
(*_18) = !_32.fld0.0;
_32.fld1 = 57617_u16 * 37783_u16;
_12 = !_4;
_3 = 5734285214490292830_i64;
(*_18) = _32.fld0.0 >> _1;
(*_18) = _32.fld0.0 * _32.fld0.0;
_37 = _12 << _2;
_36 = (*_18) as f32;
_34 = _22 as isize;
_6 = _37 * _4;
_3 = 8893298775159704748_i64;
_22 = 21684697439975576355469899052382932935_u128 << _32.fld0.3;
_20 = (_32.fld0.0, _23, _32.fld0.2, _32.fld0.3);
(*_18) = !_32.fld0.0;
(*_18) = _32.fld0.0;
_39 = _15 as i64;
match _3 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
8893298775159704748 => bb26,
_ => bb25
}
}
bb18 = {
_20.1 = _23;
(*_18) = 10651738338520557663_u64;
_31 = !_20.2;
(*_18) = _3 as u64;
_20.2 = _31 - _31;
_19 = [_22,_22,_22,_22,_22,_22,_22];
(*_18) = !6921216009058285572_u64;
(*_18) = 674790662820667233_u64 + 5179784541493620210_u64;
(*_18) = 3364848713556812439_u64 & 12793558736019515044_u64;
_22 = _4 as u128;
_15 = _23;
_32.fld0 = ((*_18), _23, _20.2, _20.3);
_32.fld0.3 = _20.3 >> _22;
_32.fld1 = 4391_u16 >> _12;
_5.1 = [(-17_i8),(-57_i8),(-77_i8),(-112_i8),(-16_i8),26_i8,32_i8];
_14.0 = [_7];
_9 = _15 as usize;
Goto(bb17)
}
bb19 = {
Return()
}
bb20 = {
(*_18) = 1605549928974772003_u64 << _2;
(*_18) = _20.2 as u64;
(*_18) = 37_isize as u64;
_18 = core::ptr::addr_of_mut!((*_18));
_20.3 = -18386_i16;
(*_18) = 16159847234133256547_u64;
_21 = 44141_u16 as f64;
_20.0 = 9_i8 as u64;
_22 = _21 as u128;
_23 = _20.1;
(*_18) = !17653948127073650521_u64;
(*_18) = 602056092814708423_u64 - 18113630247774677981_u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_9 = _6 ^ _1;
_19 = [_22,_22,_22,_22,_22,_22,_22];
Goto(bb11)
}
bb21 = {
(*_18) = 18324042865056496024_u64 | 6460031161951068940_u64;
(*_18) = _20.3 as u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_3 = (-3690318297002336942_i64) | 5744410512878669548_i64;
(*_18) = !4759103030741729766_u64;
_15 = _20.1;
(*_18) = 192235734841379597_u64;
(*_18) = !15246928159189540184_u64;
(*_18) = 12605033509124518118_u64;
_21 = (-141464648546102212317969447286249937966_i128) as f64;
_21 = _22 as f64;
_8 = _4;
(*_18) = !6538906444241749952_u64;
Goto(bb12)
}
bb22 = {
_20.2 = 74_u8 - 127_u8;
_18 = core::ptr::addr_of_mut!(_20.0);
(*_18) = !2796442059753507118_u64;
_4 = _2 & _1;
_22 = !279042382199747086197032452354902329583_u128;
(*_18) = 15103020475468255188_u64 + 15243043094383666414_u64;
_20.2 = !58_u8;
(*_18) = 18392467069710133291_u64;
_11 = !_7;
(*_18) = 17019237000440050477_u64 & 897868041212770322_u64;
(*_18) = 24_i8 as u64;
match _20.3 {
0 => bb6,
1 => bb2,
2 => bb7,
4177 => bb9,
_ => bb4
}
}
bb23 = {
(*_18) = 18324042865056496024_u64 | 6460031161951068940_u64;
(*_18) = _20.3 as u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_3 = (-3690318297002336942_i64) | 5744410512878669548_i64;
(*_18) = !4759103030741729766_u64;
_15 = _20.1;
(*_18) = 192235734841379597_u64;
(*_18) = !15246928159189540184_u64;
(*_18) = 12605033509124518118_u64;
_21 = (-141464648546102212317969447286249937966_i128) as f64;
_21 = _22 as f64;
_8 = _4;
(*_18) = !6538906444241749952_u64;
Goto(bb12)
}
bb24 = {
Return()
}
bb25 = {
_15 = _20.1;
match _20.3 {
0 => bb3,
4177 => bb10,
_ => bb6
}
}
bb26 = {
_32.fld0.1 = _23;
_12 = _8 << _2;
_20.3 = -_32.fld0.3;
_19 = [_22,_22,_22,_22,_22,_22,_22];
_23 = _15;
_20 = _32.fld0;
_6 = _37;
_22 = 86927229705329177313391171871788123523_u128;
(*_18) = !_32.fld0.0;
_21 = _22 as f64;
_24 = Adt51::Variant1 { fld0: (-48349146820400275539635295944666548640_i128),fld1: _39,fld2: _17,fld3: 111_i8 };
_32.fld0.1 = _20.1;
(*_18) = _22 as u64;
_34 = -_30;
_32.fld0.0 = (*_18) & (*_18);
_32.fld0.0 = (*_18) - (*_18);
(*_18) = _32.fld0.0;
_3 = _23 as i64;
_23 = _15;
_20 = _32.fld0;
(*_18) = _32.fld0.0;
_5.1 = [(-111_i8),(-45_i8),117_i8,2_i8,21_i8,(-128_i8),(-79_i8)];
_34 = _30 >> _32.fld0.3;
_32.fld0 = ((*_18), _20.1, _31, _20.3);
match _22 {
0 => bb8,
1 => bb14,
2 => bb25,
3 => bb17,
4 => bb5,
5 => bb12,
6 => bb27,
86927229705329177313391171871788123523 => bb29,
_ => bb28
}
}
bb27 = {
_11 = _5.0 & _5.0;
_15 = '\u{155ef}';
_8 = !_6;
_5.1 = [27_i8,(-13_i8),12_i8,112_i8,(-63_i8),(-64_i8),0_i8];
_5.1 = [(-10_i8),(-119_i8),(-107_i8),(-64_i8),(-49_i8),(-85_i8),(-109_i8)];
_2 = _1 & _1;
_19 = [294670442978308788197858293373636447680_u128,271444416854365938563989637155709861789_u128,291690899833143921564304442171395041540_u128,285122096823576315361757009704419607339_u128,261149901663204517374890271391222210219_u128,56025063190157374101918571005641354295_u128,90444954224372703781221037580883206906_u128];
_20.3 = (-4729_i16);
_14.0 = [_11];
_20 = (17394086790974935342_u64, _15, 79_u8, 4177_i16);
_13 = _20.0 as i32;
_1 = _8 >> _2;
_3 = 9223372036854775807_isize as i64;
_7 = !_5.0;
_21 = _20.0 as f64;
_12 = !_8;
_20.1 = _15;
_22 = _1 as u128;
_20.0 = _21 as u64;
match _20.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
79 => bb8,
_ => bb7
}
}
bb28 = {
(*_18) = 18324042865056496024_u64 | 6460031161951068940_u64;
(*_18) = _20.3 as u64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_3 = (-3690318297002336942_i64) | 5744410512878669548_i64;
(*_18) = !4759103030741729766_u64;
_15 = _20.1;
(*_18) = 192235734841379597_u64;
(*_18) = !15246928159189540184_u64;
(*_18) = 12605033509124518118_u64;
_21 = (-141464648546102212317969447286249937966_i128) as f64;
_21 = _22 as f64;
_8 = _4;
(*_18) = !6538906444241749952_u64;
Goto(bb12)
}
bb29 = {
_36 = (*_18) as f32;
_21 = _2 as f64;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*_18) = _22 as u64;
_32.fld0.2 = !_31;
_14.0 = [_11];
_32.fld0.3 = _20.3;
(*_18) = !_32.fld0.0;
_7 = _11 | _11;
(*_18) = !_32.fld0.0;
_36 = 6011884466719473018889897468007427536_i128 as f32;
_16.0.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_24, 1), 0)));
(*_18) = _32.fld0.0 ^ _32.fld0.0;
(*_18) = _13 as u64;
_20 = _32.fld0;
_23 = _15;
_20.3 = _32.fld0.3;
place!(Field::<i8>(Variant(_24, 1), 3)) = 58_i8 & 88_i8;
_38 = [(*_18)];
_20.2 = _32.fld0.1 as u8;
_3 = _39;
_32.fld0.1 = _23;
place!(Field::<i8>(Variant(_24, 1), 3)) = (-105_i8);
(*_18) = !_32.fld0.0;
Call((*_18) = core::intrinsics::transmute(_8), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
(*_18) = !_32.fld0.0;
_9 = !_37;
match Field::<i8>(Variant(_24, 1), 3) {
0 => bb1,
1 => bb2,
2 => bb23,
3 => bb4,
4 => bb19,
5 => bb10,
6 => bb28,
340282366920938463463374607431768211351 => bb32,
_ => bb29
}
}
bb32 = {
_20 = (_32.fld0.0, _32.fld0.1, _32.fld0.2, _32.fld0.3);
Call(_3 = core::intrinsics::bswap(Field::<i64>(Variant(_24, 1), 1)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_38 = [(*_18)];
_32.fld0.3 = _20.3;
_3 = _23 as i64;
_2 = _6 & _37;
_20.0 = _32.fld0.0 + _32.fld0.0;
_40.1 = _5.1;
(*_18) = _32.fld0.0 & _32.fld0.0;
_5.0 = _7;
_1 = !_37;
_47 = &mut _22;
(*_47) = 27641229412244912115860456620673446943_u128 ^ 331814714911639285847676128032168365219_u128;
(*_18) = _32.fld0.0 << _4;
(*_18) = !_32.fld0.0;
_17 = Field::<[i32; 7]>(Variant(_24, 1), 2);
(*_47) = 106114100463115618686136175704814161156_u128 - 161685804713393266363659169928854645573_u128;
Goto(bb34)
}
bb34 = {
(*_18) = !_32.fld0.0;
RET = Move(_16.0.0);
_9 = !_8;
(*_47) = 120168388508046769846909465738470777229_u128 << _9;
_17 = [_13,_13,_13,_13,_13,_13,_13];
(*_18) = _32.fld0.0 * _32.fld0.0;
_7 = _5.0 & _5.0;
(*_47) = 270976249129503238252549100689580082133_u128;
_31 = !_20.2;
match (*_47) {
0 => bb15,
1 => bb2,
2 => bb31,
3 => bb28,
4 => bb7,
270976249129503238252549100689580082133 => bb35,
_ => bb23
}
}
bb35 = {
(*_47) = 217864815597617930231439867235047682289_u128 - 138501790642814303609899965744728284329_u128;
_19 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
_35 = core::ptr::addr_of!(_14.0);
_32.fld0.3 = _20.3;
Call((*_47) = core::intrinsics::bswap(184868575451186554712108316780740775033_u128), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
(*_47) = 239818439275768039283030033391515010460_u128;
Call(_53.fld2 = core::intrinsics::bswap(_34), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
(*_35) = [_11];
(*_35) = [_11];
(*_18) = _32.fld0.0 + _32.fld0.0;
_53.fld2 = _34;
(*_47) = _21 as u128;
(*_47) = !229699783089111601357129610428340171425_u128;
(*_18) = _32.fld0.0 + _32.fld0.0;
_34 = _53.fld2 & _53.fld2;
(*_35) = [_5.0];
_53.fld0 = _7;
place!(Field::<i8>(Variant(_24, 1), 3)) = (-25_i8) & (-100_i8);
_32.fld0 = ((*_18), _15, _20.2, _20.3);
_17 = Field::<[i32; 7]>(Variant(_24, 1), 2);
(*_35) = [_5.0];
(*_18) = _32.fld0.0 - _32.fld0.0;
_3 = _39;
Goto(bb38)
}
bb38 = {
_20.1 = _32.fld0.1;
_52 = _36;
(*_18) = _3 as u64;
(*_18) = _32.fld0.0;
(*_35) = [_11];
(*_35) = [_5.0];
_4 = _12 - _9;
_20.3 = _32.fld0.3;
Goto(bb39)
}
bb39 = {
_10 = _32.fld1 as usize;
(*_18) = _32.fld0.0 & _32.fld0.0;
_53.fld1 = (*_47) as u8;
(*_47) = 34110883211229714403786068846042964184_u128;
_5.1 = [Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3),Field::<i8>(Variant(_24, 1), 3)];
(*_18) = !_32.fld0.0;
_2 = _1;
(*_47) = !78209663970774399706539912604668485687_u128;
_20.2 = _53.fld1;
_4 = _1 << _9;
Goto(bb40)
}
bb40 = {
_32.fld0.0 = (*_18) ^ (*_18);
_10 = _9 | _12;
_51 = _52 + _52;
RET = core::ptr::addr_of_mut!(_54);
(*RET) = (-99918950228277032149395958291400763592_i128) + 5481577098617740316605902245798714690_i128;
_10 = Field::<i64>(Variant(_24, 1), 1) as usize;
_17 = Field::<[i32; 7]>(Variant(_24, 1), 2);
(*_18) = _5.0 as u64;
_10 = _4 * _12;
(*_47) = 309549400138170103752768577741714209207_u128;
_21 = _13 as f64;
_23 = _32.fld0.1;
_50 = !_2;
Goto(bb41)
}
bb41 = {
(*_18) = _32.fld0.0;
(*_35) = [_5.0];
_49 = !(*_47);
(*_18) = !_32.fld0.0;
place!(Field::<i64>(Variant(_24, 1), 1)) = _3 - _3;
(*RET) = (*_47) as i128;
_20.1 = _32.fld0.1;
_20.1 = _15;
_57 = _30 & _53.fld2;
_59 = &(*RET);
(*RET) = (-61646292384207238895116094046008506792_i128) & (-8815407579788510477872554646526594249_i128);
_16.0.0 = core::ptr::addr_of_mut!((*RET));
(*_35) = [_7];
(*_35) = [_53.fld0];
(*RET) = (-48370178603309001470877141958351474193_i128) & 31833156071367023450397652315266765251_i128;
_35 = core::ptr::addr_of!(_14.0);
match (*_47) {
0 => bb1,
1 => bb27,
2 => bb3,
3 => bb42,
4 => bb43,
5 => bb44,
6 => bb45,
309549400138170103752768577741714209207 => bb47,
_ => bb46
}
}
bb42 = {
_11 = _5.0 & _5.0;
_15 = '\u{155ef}';
_8 = !_6;
_5.1 = [27_i8,(-13_i8),12_i8,112_i8,(-63_i8),(-64_i8),0_i8];
_5.1 = [(-10_i8),(-119_i8),(-107_i8),(-64_i8),(-49_i8),(-85_i8),(-109_i8)];
_2 = _1 & _1;
_19 = [294670442978308788197858293373636447680_u128,271444416854365938563989637155709861789_u128,291690899833143921564304442171395041540_u128,285122096823576315361757009704419607339_u128,261149901663204517374890271391222210219_u128,56025063190157374101918571005641354295_u128,90444954224372703781221037580883206906_u128];
_20.3 = (-4729_i16);
_14.0 = [_11];
_20 = (17394086790974935342_u64, _15, 79_u8, 4177_i16);
_13 = _20.0 as i32;
_1 = _8 >> _2;
_3 = 9223372036854775807_isize as i64;
_7 = !_5.0;
_21 = _20.0 as f64;
_12 = !_8;
_20.1 = _15;
_22 = _1 as u128;
_20.0 = _21 as u64;
match _20.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
79 => bb8,
_ => bb7
}
}
bb43 = {
Return()
}
bb44 = {
Return()
}
bb45 = {
(*_18) = !_32.fld0.0;
RET = Move(_16.0.0);
_9 = !_8;
(*_47) = 120168388508046769846909465738470777229_u128 << _9;
_17 = [_13,_13,_13,_13,_13,_13,_13];
(*_18) = _32.fld0.0 * _32.fld0.0;
_7 = _5.0 & _5.0;
(*_47) = 270976249129503238252549100689580082133_u128;
_31 = !_20.2;
match (*_47) {
0 => bb15,
1 => bb2,
2 => bb31,
3 => bb28,
4 => bb7,
270976249129503238252549100689580082133 => bb35,
_ => bb23
}
}
bb46 = {
(*_47) = 217864815597617930231439867235047682289_u128 - 138501790642814303609899965744728284329_u128;
_19 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
_35 = core::ptr::addr_of!(_14.0);
_32.fld0.3 = _20.3;
Call((*_47) = core::intrinsics::bswap(184868575451186554712108316780740775033_u128), ReturnTo(bb36), UnwindUnreachable())
}
bb47 = {
(*_47) = _49;
_54 = 160511083874385538234360357244034060626_i128;
_6 = _8 - _1;
_20.2 = _53.fld1 * _32.fld0.2;
_44 = &mut place!(Field::<i8>(Variant(_24, 1), 3));
_62 = _23;
_32 = Adt25 { fld0: _20,fld1: 64394_u16 };
_57 = _53.fld2 & _53.fld2;
_56 = core::ptr::addr_of_mut!((*_18));
(*_35) = [_53.fld0];
(*_56) = !_32.fld0.0;
Goto(bb48)
}
bb48 = {
_45 = _51 * _51;
_32 = Adt25 { fld0: _20,fld1: 51362_u16 };
(*RET) = _57 as i128;
(*_18) = _32.fld0.0;
(*_35) = [_53.fld0];
(*RET) = 128212240536867950313176529919602396646_i128 + 140191783260867434148351956013374611023_i128;
(*RET) = 136476103114102688842840085656349343716_i128;
(*_18) = _32.fld0.0 << _10;
_39 = (*RET) as i64;
_30 = _34;
(*RET) = 18918012507067129388531624248008893327_i128 << (*_18);
(*_35) = [_5.0];
(*_35) = [_11];
_64 = &mut _32.fld0.3;
(*_44) = _53.fld2 as i8;
Goto(bb49)
}
bb49 = {
(*_35) = [_53.fld0];
_13 = -1122221028_i32;
(*_47) = _53.fld0 as u128;
_40 = _5;
(*_18) = 4206214415548477067_u64 * 11252662799188666374_u64;
_63 = (*_64) as i32;
_8 = _1 - _6;
(*_47) = _39 as u128;
(*_47) = _49 & _49;
(*_35) = [_5.0];
(*_44) = 39_i8 + 19_i8;
_3 = _36 as i64;
_67 = Move(_59);
(*_64) = _20.3 | _20.3;
(*_44) = -13_i8;
(*_47) = !_49;
(*_47) = 24450_u16 as u128;
_40 = (_5.0, _5.1);
_9 = _5.0 as usize;
_43 = &mut (*_44);
(*_64) = -_20.3;
(*_47) = _49 & _49;
(*_64) = -_20.3;
(*_43) = (-85_i8);
(*_43) = _2 as i8;
Goto(bb50)
}
bb50 = {
Call(_72 = dump_var(Move(_13), Move(_19), Move(_14), Move(_23)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_72 = dump_var(Move(_50), Move(_17), Move(_30), Move(_40)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_72 = dump_var(Move(_1), Move(_37), Move(_8), Move(_49)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_72 = dump_var(Move(_57), Move(_7), Move(_31), Move(_62)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [bool; 1],mut _2: bool,mut _3: usize,mut _4: usize) -> char {
mir! {
type RET = char;
let _5: (u32,);
let _6: (bool, (i8, u16, Adt29), i128, u64);
let _7: Adt29;
let _8: i8;
let _9: i32;
let _10: *mut u32;
let _11: isize;
let _12: i32;
let _13: u64;
let _14: [i32; 7];
let _15: f32;
let _16: isize;
let _17: isize;
let _18: *const (u32,);
let _19: u32;
let _20: &'static mut [bool; 2];
let _21: u128;
let _22: &'static mut &'static &'static Adt18;
let _23: isize;
let _24: bool;
let _25: i64;
let _26: &'static mut u128;
let _27: &'static Adt48;
let _28: u16;
let _29: bool;
let _30: isize;
let _31: *mut f64;
let _32: *mut f64;
let _33: u32;
let _34: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _35: f64;
let _36: *mut i128;
let _37: *const *const &'static mut u128;
let _38: &'static Adt18;
let _39: f64;
let _40: &'static *mut i128;
let _41: &'static u128;
let _42: *mut f64;
let _43: bool;
let _44: &'static mut &'static mut &'static &'static Adt18;
let _45: *mut f64;
let _46: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _47: i32;
let _48: isize;
let _49: *mut u64;
let _50: char;
let _51: [i8; 1];
let _52: isize;
let _53: (*mut i128, isize);
let _54: [char; 2];
let _55: (u64, char, u8, i16);
let _56: u32;
let _57: [isize; 3];
let _58: &'static mut f32;
let _59: *const [bool; 1];
let _60: [i16; 2];
let _61: &'static *mut i128;
let _62: *mut usize;
let _63: *mut u8;
let _64: (Adt26, (i8, u16, Adt29), (u64, char, u8, i16), (u32,));
let _65: char;
let _66: &'static mut i8;
let _67: &'static mut [bool; 2];
let _68: bool;
let _69: isize;
let _70: *mut f64;
let _71: &'static mut f32;
let _72: *mut *const usize;
let _73: u8;
let _74: (i8, u16, Adt29);
let _75: &'static mut &'static &'static Adt18;
let _76: &'static mut i16;
let _77: &'static mut i16;
let _78: (Adt26, (i8, u16, Adt29), (u64, char, u8, i16), (u32,));
let _79: u64;
let _80: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _81: &'static u128;
let _82: bool;
let _83: Adt63;
let _84: Adt18;
let _85: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _86: i8;
let _87: f64;
let _88: u16;
let _89: isize;
let _90: *mut u8;
let _91: isize;
let _92: *const [bool; 1];
let _93: isize;
let _94: bool;
let _95: &'static mut f32;
let _96: *mut i128;
let _97: (u32,);
let _98: *mut &'static i128;
let _99: *const i8;
let _100: &'static &'static Adt18;
let _101: f64;
let _102: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _103: ();
let _104: ();
{
RET = '\u{f9e12}';
_1 = [_2];
Call(_3 = fn10(_4, RET, RET, _2, _2, _2, RET, RET, RET, RET, RET, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = (608691346_u32,);
_5 = (1081295953_u32,);
_1 = [_2];
_5.0 = 3644682212_u32 * 3390069023_u32;
_2 = _3 == _3;
RET = '\u{1e39f}';
_6.1.0 = 30_i8 - 67_i8;
_6.1.1 = 13565_u16;
_6.3 = !2210508928549601336_u64;
_3 = 8387602331016318404_i64 as usize;
RET = '\u{a5e9c}';
_4 = _3 - _3;
_8 = _6.1.0;
_10 = core::ptr::addr_of_mut!(_5.0);
_6.2 = 154022701199087486982099269948730506033_i128 << (*_10);
(*_10) = (-9223372036854775808_isize) as u32;
_6.2 = !(-60746617398694748358041145939131995312_i128);
(*_10) = !4047907637_u32;
_6.1.0 = (-2123717008787531624_i64) as i8;
_6.3 = 4461749964009506708_u64 + 3137425631876063417_u64;
(*_10) = 4189710043_u32;
(*_10) = _6.1.1 as u32;
Goto(bb2)
}
bb2 = {
_4 = _3;
_1 = [_2];
_5.0 = 3968168185_u32 & 2513600765_u32;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = 2839381822_u32 & 3339028722_u32;
(*_10) = _6.3 as u32;
_11 = (-9223372036854775808_isize) + (-40_isize);
_5.0 = 1726211279_u32 << _6.1.1;
(*_10) = 1292920810_u32;
(*_10) = 158187954_u32;
_5.0 = _6.2 as u32;
_4 = !_3;
_13 = !_6.3;
_1 = [_2];
_3 = _4 >> (*_10);
(*_10) = 1271209239_u32 | 2692932098_u32;
_12 = (-1842435295_i32) & 1245408900_i32;
RET = '\u{e02a0}';
_2 = true & true;
(*_10) = 1860136459_u32 | 495031466_u32;
_6.3 = _13 | _13;
(*_10) = !2863386565_u32;
Call((*_10) = core::intrinsics::bswap(3660915276_u32), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.1.0 = _8;
(*_10) = 900604957_u32;
_1 = [_2];
_2 = !false;
_6.0 = _3 == _3;
_13 = _6.3 * _6.3;
(*_10) = _13 as u32;
_14 = [_12,_12,_12,_12,_12,_12,_12];
(*_10) = 203_u8 as u32;
(*_10) = 4137695098_u32;
RET = '\u{e1d44}';
(*_10) = _11 as u32;
(*_10) = !1442809430_u32;
_6.3 = _13 | _13;
(*_10) = 269332527246469814_i64 as u32;
_6.1.0 = _8;
_15 = _12 as f32;
(*_10) = 2935550893_u32;
_9 = _12 & _12;
_14 = [_9,_9,_9,_12,_12,_12,_9];
Goto(bb4)
}
bb4 = {
(*_10) = _11 as u32;
(*_10) = 395206870_u32 | 3823267250_u32;
_5 = (4068039639_u32,);
_6.0 = _2;
(*_10) = 1080786886_u32;
(*_10) = RET as u32;
_6.3 = _13;
_6.0 = _13 >= _13;
_11 = -9223372036854775807_isize;
_10 = core::ptr::addr_of_mut!((*_10));
_12 = _9;
_11 = !9223372036854775807_isize;
(*_10) = 237630463_u32;
_2 = _6.0;
_6.1.1 = 40266_u16 * 12349_u16;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = _3 as u32;
_14 = [_12,_9,_12,_12,_9,_9,_12];
(*_10) = 3415724049_u32 + 1512984606_u32;
_16 = _11 >> (*_10);
(*_10) = 4120330698_u32 + 1881581343_u32;
Goto(bb5)
}
bb5 = {
_14 = [_12,_9,_9,_9,_12,_9,_9];
Goto(bb6)
}
bb6 = {
_17 = _16 ^ _16;
_6.1.0 = _8;
_8 = _6.1.0 >> (*_10);
_4 = _15 as usize;
_2 = _6.0 ^ _6.0;
(*_10) = 3777186380_u32 >> _12;
_2 = _6.0;
(*_10) = 2329161818_u32 ^ 2926095389_u32;
_18 = core::ptr::addr_of!(_5);
_6.1.0 = -_8;
_1 = [_6.0];
(*_18).0 = 3463268158_u32 * 844383445_u32;
(*_18) = (2321344965_u32,);
(*_18).0 = 2317163622_u32 >> _6.2;
(*_18) = (2317093525_u32,);
(*_10) = 3698655162_u32 ^ 2766818792_u32;
(*_10) = !3904106637_u32;
Goto(bb7)
}
bb7 = {
_11 = _8 as isize;
_6.1.1 = 48331_u16;
_1 = [_2];
_6.3 = !_13;
Goto(bb8)
}
bb8 = {
(*_10) = _8 as u32;
_6.0 = _2 ^ _2;
match _6.1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
48331 => bb9,
_ => bb6
}
}
bb9 = {
_23 = -_16;
(*_10) = 65_u8 as u32;
_6.3 = (*_10) as u64;
_6.3 = RET as u64;
(*_10) = 194321801_u32;
_13 = _6.3 * _6.3;
_6.1.0 = -_8;
_21 = 134335938085400541737543806736554738414_u128;
_24 = _2;
_25 = (-1136867722663723248_i64) | (-1244813717893550033_i64);
_6.3 = _13;
RET = '\u{7ae8b}';
(*_10) = 4017314534_u32;
(*_10) = _2 as u32;
_16 = _23;
(*_10) = 3826437111_u32;
_13 = _15 as u64;
_17 = _21 as isize;
(*_10) = 2523891797_u32 >> _12;
_19 = (*_10);
_17 = _16 >> (*_10);
Goto(bb10)
}
bb10 = {
_5.0 = RET as u32;
(*_10) = _19 >> _13;
_6.1.0 = _8 << _11;
(*_10) = !_19;
_6.0 = _24;
(*_10) = _6.1.1 as u32;
(*_10) = _19 * _19;
_18 = core::ptr::addr_of!(_5);
Call((*_18).0 = core::intrinsics::transmute(_12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = _17 + _17;
_18 = core::ptr::addr_of!((*_18));
(*_10) = _6.1.0 as u32;
(*_10) = _19 * _19;
_18 = core::ptr::addr_of!(_5);
(*_18).0 = _6.0 as u32;
_24 = !_2;
(*_10) = !_19;
_14 = [_12,_9,_12,_12,_12,_9,_12];
(*_10) = _19 | _19;
(*_10) = _13 as u32;
(*_10) = !_19;
_24 = _6.0 | _2;
(*_10) = _19 ^ _19;
Call(_15 = core::intrinsics::transmute((*_10)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_10) = _19;
match _6.1.1 {
0 => bb4,
1 => bb13,
48331 => bb15,
_ => bb14
}
}
bb13 = {
_4 = _3;
_1 = [_2];
_5.0 = 3968168185_u32 & 2513600765_u32;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = 2839381822_u32 & 3339028722_u32;
(*_10) = _6.3 as u32;
_11 = (-9223372036854775808_isize) + (-40_isize);
_5.0 = 1726211279_u32 << _6.1.1;
(*_10) = 1292920810_u32;
(*_10) = 158187954_u32;
_5.0 = _6.2 as u32;
_4 = !_3;
_13 = !_6.3;
_1 = [_2];
_3 = _4 >> (*_10);
(*_10) = 1271209239_u32 | 2692932098_u32;
_12 = (-1842435295_i32) & 1245408900_i32;
RET = '\u{e02a0}';
_2 = true & true;
(*_10) = 1860136459_u32 | 495031466_u32;
_6.3 = _13 | _13;
(*_10) = !2863386565_u32;
Call((*_10) = core::intrinsics::bswap(3660915276_u32), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_11 = _8 as isize;
_6.1.1 = 48331_u16;
_1 = [_2];
_6.3 = !_13;
Goto(bb8)
}
bb15 = {
_28 = _6.1.1 - _6.1.1;
(*_10) = _19;
(*_10) = !_19;
(*_10) = _19 >> _17;
_2 = (*_10) < (*_10);
_1 = [_6.0];
_28 = (-32515_i16) as u16;
_6.0 = (*_10) >= (*_10);
_23 = _16;
(*_10) = _19;
match _21 {
0 => bb12,
1 => bb8,
2 => bb4,
3 => bb16,
134335938085400541737543806736554738414 => bb18,
_ => bb17
}
}
bb16 = {
_11 = _8 as isize;
_6.1.1 = 48331_u16;
_1 = [_2];
_6.3 = !_13;
Goto(bb8)
}
bb17 = {
_4 = _3;
_1 = [_2];
_5.0 = 3968168185_u32 & 2513600765_u32;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = 2839381822_u32 & 3339028722_u32;
(*_10) = _6.3 as u32;
_11 = (-9223372036854775808_isize) + (-40_isize);
_5.0 = 1726211279_u32 << _6.1.1;
(*_10) = 1292920810_u32;
(*_10) = 158187954_u32;
_5.0 = _6.2 as u32;
_4 = !_3;
_13 = !_6.3;
_1 = [_2];
_3 = _4 >> (*_10);
(*_10) = 1271209239_u32 | 2692932098_u32;
_12 = (-1842435295_i32) & 1245408900_i32;
RET = '\u{e02a0}';
_2 = true & true;
(*_10) = 1860136459_u32 | 495031466_u32;
_6.3 = _13 | _13;
(*_10) = !2863386565_u32;
Call((*_10) = core::intrinsics::bswap(3660915276_u32), ReturnTo(bb3), UnwindUnreachable())
}
bb18 = {
(*_10) = !_19;
(*_10) = _19 - _19;
_5 = (_19,);
_23 = _25 as isize;
_6.2 = !101396660437095127037376052957401522679_i128;
_25 = _17 as i64;
_25 = (-3871671184735422956_i64) & 7502245418807706072_i64;
_28 = _6.1.1 | _6.1.1;
_15 = _6.1.1 as f32;
_2 = _6.0;
(*_10) = !_19;
(*_10) = _19 * _19;
(*_10) = !_19;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = _13 as u32;
_6.1.1 = _15 as u16;
(*_10) = _19 ^ _19;
_3 = _6.3 as usize;
_34.3.0.1.0 = _8;
Goto(bb19)
}
bb19 = {
_18 = core::ptr::addr_of!(_5);
_6.0 = (*_18).0 != (*_10);
(*_18).0 = _19 | _19;
_35 = _15 as f64;
_30 = !_17;
(*_10) = !_19;
(*_10) = 29745_i16 as u32;
_34.3.0.0 = _6.0 == _24;
_28 = (*_10) as u16;
_8 = !_6.1.0;
_12 = _9;
_17 = _34.3.0.1.0 as isize;
(*_10) = _19 - _19;
_29 = _24 != _24;
_12 = -_9;
Goto(bb20)
}
bb20 = {
_6.2 = _24 as i128;
_32 = core::ptr::addr_of_mut!(_35);
_33 = !(*_10);
_26 = &mut _21;
_34.3.0.2 = 5120_i16 as i128;
(*_32) = _6.2 as f64;
(*_10) = !_19;
(*_26) = !220064148240559159608025291185662484777_u128;
(*_10) = (-10126_i16) as u32;
(*_10) = _19 * _19;
(*_26) = 234673688340696434580688372504547868540_u128;
_34.3.0.3 = _13 ^ _6.3;
_1 = [_34.3.0.0];
(*_10) = _8 as u32;
_10 = core::ptr::addr_of_mut!((*_10));
Goto(bb21)
}
bb21 = {
(*_26) = !262537789546812902560655247707693146601_u128;
(*_10) = 194_u8 as u32;
(*_10) = _33 >> _8;
Goto(bb22)
}
bb22 = {
_43 = (*_10) == (*_10);
(*_26) = 214954651025996427438616620946207239908_u128;
(*_32) = _15 as f64;
(*_32) = _6.2 as f64;
(*_32) = (*_26) as f64;
(*_26) = 37111482319135649152298915292978536494_u128 << (*_10);
(*_32) = _6.1.1 as f64;
_36 = core::ptr::addr_of_mut!(_34.3.0.2);
(*_36) = _6.1.0 as i128;
(*_26) = 88758385244079687324309211549556598986_u128 + 338082886237547487822686542875636083865_u128;
_34.1 = _9;
_35 = (*_36) as f64;
(*_26) = 241096844696510408875794300221666915061_u128 ^ 128414371597762794525540201319505094393_u128;
(*_36) = _6.2 ^ _6.2;
_36 = core::ptr::addr_of_mut!(_6.2);
(*_10) = !_33;
(*_10) = !_33;
(*_10) = _33;
Goto(bb23)
}
bb23 = {
(*_10) = _33 & _33;
(*_26) = !165768516189969606002715806279939737568_u128;
(*_26) = 196503237081077268562569284876013686283_u128 | 304148179854552700375445105130758804467_u128;
(*_10) = _19 + _33;
(*_10) = _33;
_10 = core::ptr::addr_of_mut!((*_10));
(*_32) = _12 as f64;
_14 = [_12,_9,_12,_12,_9,_12,_9];
_29 = (*_36) <= (*_36);
(*_36) = _34.3.0.2 + _34.3.0.2;
(*_26) = !339682647676771858496656610658880069447_u128;
_12 = -_34.1;
_43 = !_29;
_45 = Move(_32);
_25 = (-357314156848404313_i64) * (-8453207133311227795_i64);
_5.0 = (*_36) as u32;
(*_36) = _34.3.0.2 & _34.3.0.2;
(*_26) = _3 as u128;
(*_36) = !_34.3.0.2;
(*_36) = _34.3.0.2 + _34.3.0.2;
(*_10) = _19;
_34.3.0.1.1 = _28;
(*_10) = _33 << (*_36);
_4 = _34.1 as usize;
Goto(bb24)
}
bb24 = {
_19 = (*_10) >> (*_10);
_24 = (*_10) >= (*_10);
(*_26) = 275195671984600466532871677576282120016_u128 * 46003826894531537340724391634131646102_u128;
(*_36) = _34.3.0.2;
Goto(bb25)
}
bb25 = {
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = !_19;
_11 = _16 << (*_10);
_52 = _11 * _11;
(*_26) = 243936977902808270640881049688899633319_u128;
_48 = _52 * _11;
_1 = [_24];
_16 = _52 * _11;
(*_26) = 65664172332059676318891192951857632695_u128 << (*_36);
(*_10) = !_19;
(*_10) = (*_26) as u32;
(*_26) = 222829207818836223587816536084930400606_u128 << (*_10);
_2 = (*_10) != (*_10);
_53.1 = _30 >> (*_36);
_3 = _4;
_47 = _9 << _34.3.0.1.1;
(*_10) = _4 as u32;
_28 = !_34.3.0.1.1;
_41 = &(*_26);
_34.3.0.0 = _2 <= _2;
_55.3 = (-16099_i16) & (-14581_i16);
(*_10) = _19 * _19;
(*_36) = _34.3.0.2 + _34.3.0.2;
_40 = &_36;
_15 = _25 as f32;
_53 = (Move(_36), _52);
_52 = !_16;
Goto(bb26)
}
bb26 = {
(*_26) = _25 as u128;
_50 = RET;
_12 = !_34.1;
(*_10) = _19 | _19;
_14 = [_34.1,_12,_47,_9,_34.1,_47,_47];
(*_10) = _19 * _19;
(*_26) = !188804184117070393547260126058919476503_u128;
(*_10) = !_19;
_28 = _6.1.1;
_42 = core::ptr::addr_of_mut!(_35);
_11 = !_48;
_39 = (*_42) * (*_42);
_34.2 = [_34.3.0.0];
_10 = core::ptr::addr_of_mut!((*_10));
_34.3.0.1.1 = _6.1.1 - _28;
_34.3.0.1.0 = (*_10) as i8;
_16 = -_53.1;
_31 = core::ptr::addr_of_mut!((*_42));
_30 = _16 + _53.1;
Goto(bb27)
}
bb27 = {
_49 = core::ptr::addr_of_mut!(_13);
Call(_64.1.0 = core::intrinsics::bswap(_34.3.0.1.0), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_57 = [_16,_16,_52];
(*_42) = _39;
(*_10) = _19 * _19;
_1 = _34.2;
_14 = [_12,_9,_34.1,_47,_34.1,_9,_12];
(*_26) = 223672825189744158790233058514415627191_u128;
(*_26) = 166523441192744201614722345901356390713_u128 - 115900541125099850763139415911686386165_u128;
_1 = [_34.3.0.0];
_12 = -_47;
(*_49) = _4 as u64;
(*_26) = 107641127789475474867222342360269538908_u128;
(*_10) = _19 << _19;
_12 = _9 >> _19;
Goto(bb29)
}
bb29 = {
_70 = core::ptr::addr_of_mut!((*_42));
(*_49) = !_6.3;
Goto(bb30)
}
bb30 = {
_36 = core::ptr::addr_of_mut!(_6.2);
(*_49) = _6.3 | _34.3.0.3;
(*_36) = _34.3.0.2 - _34.3.0.2;
_6.1.0 = _34.3.0.1.0;
_24 = _43 ^ _29;
_47 = _12 ^ _12;
_70 = core::ptr::addr_of_mut!((*_70));
_64.2.3 = _6.1.1 as i16;
(*_26) = !184764464700577954067208613704187419892_u128;
(*_49) = _34.3.0.3;
(*_49) = !_34.3.0.3;
(*_36) = _43 as i128;
(*_36) = _34.3.0.2;
_71 = &mut _15;
(*_49) = (*_10) as u64;
_48 = -_16;
Goto(bb31)
}
bb31 = {
(*_49) = _43 as u64;
_34.0 = core::ptr::addr_of_mut!((*_49));
_64.3.0 = (*_10) + (*_10);
(*_36) = _34.3.0.2 | _34.3.0.2;
(*_26) = 96343967037570837450607101102412277820_u128 >> (*_36);
_64.2.2 = !129_u8;
(*_70) = _39 + _39;
(*_70) = -_39;
_57 = [_52,_16,_23];
_64.2 = ((*_49), RET, 140_u8, _55.3);
(*_71) = _34.3.0.1.1 as f32;
_56 = (*_10) & (*_10);
(*_42) = _39;
(*_10) = !_19;
_54 = [_64.2.1,_64.2.1];
(*_36) = _34.3.0.2;
(*_71) = _64.2.3 as f32;
Goto(bb32)
}
bb32 = {
_34.3.0.2 = (*_36);
_58 = Move(_71);
(*_26) = 62052895235022663949295808804763253111_u128 ^ 251778663599405184647096316638406640359_u128;
_68 = !_2;
_53.1 = _48 | _48;
(*_49) = _64.2.0 + _64.2.0;
(*_10) = _64.3.0 & _64.3.0;
_29 = _43;
(*_36) = _34.3.0.2;
(*_36) = _34.3.0.2 | _34.3.0.2;
_65 = RET;
_42 = core::ptr::addr_of_mut!(_39);
_29 = (*_42) != _35;
match _64.2.2 {
0 => bb18,
1 => bb29,
2 => bb31,
140 => bb33,
_ => bb26
}
}
bb33 = {
(*_10) = _19 * _64.3.0;
_29 = !_34.3.0.0;
(*_36) = _34.3.0.2 >> (*_10);
Goto(bb34)
}
bb34 = {
_73 = !_64.2.2;
_5.0 = !_64.3.0;
(*_10) = _19;
_17 = _52;
_6.2 = _34.3.0.2 | _34.3.0.2;
_77 = &mut _55.3;
_6.1.1 = _28;
(*_26) = 310054312871219642982902611820851345794_u128 * 62968326484800538829428232168085323948_u128;
_6.1.1 = _28 - _34.3.0.1.1;
_64.1.0 = _8;
(*_77) = _64.2.3 ^ _64.2.3;
_80.2 = _73;
(*_26) = !80811427729200843721929413004214002002_u128;
(*_49) = !_64.2.0;
_1 = _34.2;
_34.3.0.1.0 = _6.1.0 + _6.1.0;
_78.2.0 = (*_49);
Goto(bb35)
}
bb35 = {
(*_77) = _64.2.3;
_12 = _47 >> (*_36);
_74.1 = _28 - _6.1.1;
(*_77) = _64.2.3 >> _6.1.0;
_70 = Move(_42);
(*_36) = _34.3.0.2;
_36 = core::ptr::addr_of_mut!((*_36));
_48 = _52;
(*_36) = -_34.3.0.2;
_78.2.3 = (*_77) >> (*_49);
_6.1.0 = _39 as i8;
(*_36) = _6.1.0 as i128;
_32 = core::ptr::addr_of_mut!(_39);
_48 = (*_77) as isize;
(*_32) = -_35;
_64.3.0 = _47 as u32;
(*_26) = 331858251815549056674592983602023955587_u128;
match _64.2.2 {
0 => bb1,
1 => bb19,
2 => bb17,
3 => bb36,
140 => bb38,
_ => bb37
}
}
bb36 = {
_17 = _16 ^ _16;
_6.1.0 = _8;
_8 = _6.1.0 >> (*_10);
_4 = _15 as usize;
_2 = _6.0 ^ _6.0;
(*_10) = 3777186380_u32 >> _12;
_2 = _6.0;
(*_10) = 2329161818_u32 ^ 2926095389_u32;
_18 = core::ptr::addr_of!(_5);
_6.1.0 = -_8;
_1 = [_6.0];
(*_18).0 = 3463268158_u32 * 844383445_u32;
(*_18) = (2321344965_u32,);
(*_18).0 = 2317163622_u32 >> _6.2;
(*_18) = (2317093525_u32,);
(*_10) = 3698655162_u32 ^ 2766818792_u32;
(*_10) = !3904106637_u32;
Goto(bb7)
}
bb37 = {
_4 = _3;
_1 = [_2];
_5.0 = 3968168185_u32 & 2513600765_u32;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = 2839381822_u32 & 3339028722_u32;
(*_10) = _6.3 as u32;
_11 = (-9223372036854775808_isize) + (-40_isize);
_5.0 = 1726211279_u32 << _6.1.1;
(*_10) = 1292920810_u32;
(*_10) = 158187954_u32;
_5.0 = _6.2 as u32;
_4 = !_3;
_13 = !_6.3;
_1 = [_2];
_3 = _4 >> (*_10);
(*_10) = 1271209239_u32 | 2692932098_u32;
_12 = (-1842435295_i32) & 1245408900_i32;
RET = '\u{e02a0}';
_2 = true & true;
(*_10) = 1860136459_u32 | 495031466_u32;
_6.3 = _13 | _13;
(*_10) = !2863386565_u32;
Call((*_10) = core::intrinsics::bswap(3660915276_u32), ReturnTo(bb3), UnwindUnreachable())
}
bb38 = {
(*_26) = 214786185667910396231640000711998104308_u128 << (*_49);
(*_36) = _34.3.0.2 | _34.3.0.2;
(*_26) = 75298719447835750271782688304040265779_u128 + 211052592286089776302300037432780573365_u128;
_34.3.0.3 = (*_49) ^ (*_49);
(*_36) = _34.3.0.2 & _34.3.0.2;
_84.fld0 = _43 | _68;
(*_32) = -_35;
Goto(bb39)
}
bb39 = {
(*_26) = 206331569836076432967145144751120077460_u128 ^ 275263646216376910113887063054319410040_u128;
_11 = _17 >> _5.0;
Goto(bb40)
}
bb40 = {
_84.fld4 = (*_77) | (*_77);
_40 = &_53.0;
_59 = core::ptr::addr_of!(_34.2);
_79 = (*_77) as u64;
(*_59) = _1;
(*_32) = _35 * _35;
_32 = core::ptr::addr_of_mut!((*_32));
(*_10) = _73 as u32;
_61 = &(*_40);
(*_59) = [_43];
(*_59) = [_24];
_6.0 = _29 & _2;
(*_59) = [_24];
(*_77) = _84.fld4 + _84.fld4;
(*_59) = [_43];
(*_49) = !_78.2.0;
(*_32) = _84.fld4 as f64;
match _64.2.2 {
0 => bb41,
140 => bb43,
_ => bb42
}
}
bb41 = {
(*_77) = _64.2.3;
_12 = _47 >> (*_36);
_74.1 = _28 - _6.1.1;
(*_77) = _64.2.3 >> _6.1.0;
_70 = Move(_42);
(*_36) = _34.3.0.2;
_36 = core::ptr::addr_of_mut!((*_36));
_48 = _52;
(*_36) = -_34.3.0.2;
_78.2.3 = (*_77) >> (*_49);
_6.1.0 = _39 as i8;
(*_36) = _6.1.0 as i128;
_32 = core::ptr::addr_of_mut!(_39);
_48 = (*_77) as isize;
(*_32) = -_35;
_64.3.0 = _47 as u32;
(*_26) = 331858251815549056674592983602023955587_u128;
match _64.2.2 {
0 => bb1,
1 => bb19,
2 => bb17,
3 => bb36,
140 => bb38,
_ => bb37
}
}
bb42 = {
(*_26) = _25 as u128;
_50 = RET;
_12 = !_34.1;
(*_10) = _19 | _19;
_14 = [_34.1,_12,_47,_9,_34.1,_47,_47];
(*_10) = _19 * _19;
(*_26) = !188804184117070393547260126058919476503_u128;
(*_10) = !_19;
_28 = _6.1.1;
_42 = core::ptr::addr_of_mut!(_35);
_11 = !_48;
_39 = (*_42) * (*_42);
_34.2 = [_34.3.0.0];
_10 = core::ptr::addr_of_mut!((*_10));
_34.3.0.1.1 = _6.1.1 - _28;
_34.3.0.1.0 = (*_10) as i8;
_16 = -_53.1;
_31 = core::ptr::addr_of_mut!((*_42));
_30 = _16 + _53.1;
Goto(bb27)
}
bb43 = {
_84.fld1 = !_73;
_42 = Move(_32);
_80.3 = &(*_61);
(*_77) = _84.fld4 * _84.fld4;
_64.1.1 = _74.1;
_78.2.3 = (*_77);
(*_26) = (*_49) as u128;
_84.fld5 = !_4;
Goto(bb44)
}
bb44 = {
_85.2.2 = !_3;
(*_49) = _25 as u64;
Goto(bb45)
}
bb45 = {
_64.2.1 = _65;
_8 = -_34.3.0.1.0;
_5 = (_19,);
(*_26) = !170141809464849442597246813247235005891_u128;
(*_77) = _25 as i16;
_94 = _6.0 & _2;
_78.3.0 = (*_36) as u32;
_34.3.0.0 = (*_36) < _6.2;
Call(_84.fld4 = core::intrinsics::transmute((*_77)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_53 = (Move(_36), _52);
_78.1.0 = _6.2 as i8;
_33 = _78.3.0 + (*_10);
_90 = core::ptr::addr_of_mut!(_84.fld1);
_34.3.0.1.1 = _64.1.1;
_93 = -_11;
(*_26) = 289410350785977895162815696931293960229_u128 - 262084416173306860253908482747757754490_u128;
_10 = core::ptr::addr_of_mut!((*_10));
_51 = [_34.3.0.1.0];
(*_59) = _1;
_34.3.0.0 = _94;
_85.0 = core::ptr::addr_of!(_85.2.2);
_80.1.1.0 = _64.1.0 ^ _78.1.0;
_56 = _64.3.0;
_87 = _25 as f64;
_80.1.2 = _34.3.0.2 + _34.3.0.2;
_60 = [_78.2.3,_78.2.3];
(*_10) = _56 * _56;
(*_77) = !_78.2.3;
(*_90) = _25 as u8;
_84.fld0 = _6.2 == _80.1.2;
_36 = Move(_53.0);
(*_77) = _78.2.3 | _78.2.3;
(*_90) = _80.2 | _80.2;
_84.fld1 = (*_77) as u8;
(*_10) = _79 as u32;
match _64.2.2 {
0 => bb47,
1 => bb48,
2 => bb49,
3 => bb50,
140 => bb52,
_ => bb51
}
}
bb47 = {
_64.2.1 = _65;
_8 = -_34.3.0.1.0;
_5 = (_19,);
(*_26) = !170141809464849442597246813247235005891_u128;
(*_77) = _25 as i16;
_94 = _6.0 & _2;
_78.3.0 = (*_36) as u32;
_34.3.0.0 = (*_36) < _6.2;
Call(_84.fld4 = core::intrinsics::transmute((*_77)), ReturnTo(bb46), UnwindUnreachable())
}
bb48 = {
(*_77) = _64.2.3;
_12 = _47 >> (*_36);
_74.1 = _28 - _6.1.1;
(*_77) = _64.2.3 >> _6.1.0;
_70 = Move(_42);
(*_36) = _34.3.0.2;
_36 = core::ptr::addr_of_mut!((*_36));
_48 = _52;
(*_36) = -_34.3.0.2;
_78.2.3 = (*_77) >> (*_49);
_6.1.0 = _39 as i8;
(*_36) = _6.1.0 as i128;
_32 = core::ptr::addr_of_mut!(_39);
_48 = (*_77) as isize;
(*_32) = -_35;
_64.3.0 = _47 as u32;
(*_26) = 331858251815549056674592983602023955587_u128;
match _64.2.2 {
0 => bb1,
1 => bb19,
2 => bb17,
3 => bb36,
140 => bb38,
_ => bb37
}
}
bb49 = {
(*_10) = _11 as u32;
(*_10) = 395206870_u32 | 3823267250_u32;
_5 = (4068039639_u32,);
_6.0 = _2;
(*_10) = 1080786886_u32;
(*_10) = RET as u32;
_6.3 = _13;
_6.0 = _13 >= _13;
_11 = -9223372036854775807_isize;
_10 = core::ptr::addr_of_mut!((*_10));
_12 = _9;
_11 = !9223372036854775807_isize;
(*_10) = 237630463_u32;
_2 = _6.0;
_6.1.1 = 40266_u16 * 12349_u16;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = _3 as u32;
_14 = [_12,_9,_12,_12,_9,_9,_12];
(*_10) = 3415724049_u32 + 1512984606_u32;
_16 = _11 >> (*_10);
(*_10) = 4120330698_u32 + 1881581343_u32;
Goto(bb5)
}
bb50 = {
(*_49) = _43 as u64;
_34.0 = core::ptr::addr_of_mut!((*_49));
_64.3.0 = (*_10) + (*_10);
(*_36) = _34.3.0.2 | _34.3.0.2;
(*_26) = 96343967037570837450607101102412277820_u128 >> (*_36);
_64.2.2 = !129_u8;
(*_70) = _39 + _39;
(*_70) = -_39;
_57 = [_52,_16,_23];
_64.2 = ((*_49), RET, 140_u8, _55.3);
(*_71) = _34.3.0.1.1 as f32;
_56 = (*_10) & (*_10);
(*_42) = _39;
(*_10) = !_19;
_54 = [_64.2.1,_64.2.1];
(*_36) = _34.3.0.2;
(*_71) = _64.2.3 as f32;
Goto(bb32)
}
bb51 = {
(*_26) = !262537789546812902560655247707693146601_u128;
(*_10) = 194_u8 as u32;
(*_10) = _33 >> _8;
Goto(bb22)
}
bb52 = {
(*_59) = [_6.0];
(*_90) = _73 % _64.2.2;
_88 = _6.1.1 * _64.1.1;
_89 = _16 << _78.2.0;
_6.1.1 = !_64.1.1;
_34.2 = _1;
(*_10) = !_19;
_6.2 = !_34.3.0.2;
_65 = _64.2.1;
(*_77) = _6.2 as i16;
_64.2.0 = _78.2.0 + _79;
Goto(bb53)
}
bb53 = {
Call(_103 = dump_var(Move(_48), Move(_13), Move(_93), Move(_94)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_103 = dump_var(Move(_11), Move(_14), Move(_1), Move(_50)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_103 = dump_var(Move(_25), Move(_47), Move(_30), Move(_24)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_103 = dump_var(Move(_89), Move(_4), Move(_43), Move(_52)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_103 = dump_var(Move(_51), Move(_68), Move(_79), Move(_9)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: usize,mut _2: char,mut _3: char,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: usize) -> usize {
mir! {
type RET = usize;
let _13: i128;
let _14: f32;
let _15: *const [bool; 1];
let _16: isize;
let _17: &'static i128;
let _18: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _19: &'static &'static Adt18;
let _20: isize;
let _21: u64;
let _22: bool;
let _23: *const [bool; 1];
let _24: *const &'static mut u128;
let _25: f32;
let _26: [i128; 1];
let _27: &'static Adt18;
let _28: &'static i128;
let _29: Adt48;
let _30: &'static i128;
let _31: i128;
let _32: u32;
let _33: isize;
let _34: [bool; 1];
let _35: bool;
let _36: char;
let _37: i128;
let _38: f64;
let _39: i16;
let _40: f32;
let _41: f64;
let _42: f32;
let _43: Adt25;
let _44: (bool, [i8; 7]);
let _45: *const usize;
let _46: bool;
let _47: (*mut i128, isize);
let _48: i64;
let _49: &'static mut f32;
let _50: f64;
let _51: bool;
let _52: *const *const &'static mut u128;
let _53: &'static mut u32;
let _54: *const (u32,);
let _55: &'static u128;
let _56: Adt18;
let _57: [i16; 2];
let _58: f32;
let _59: Adt51;
let _60: &'static mut [bool; 2];
let _61: [i8; 8];
let _62: ();
let _63: ();
{
_1 = _12 & _12;
_5 = _6;
_10 = _7;
_5 = _4 < _4;
_13 = (-14404323129150604349885299970393677417_i128) << _1;
_1 = _12 >> _13;
_9 = _2;
RET = _12 - _1;
_14 = (-778609213323425686_i64) as f32;
_12 = !_1;
_13 = -(-135539877959593985578092705498239948009_i128);
RET = !_12;
_2 = _9;
_4 = !_6;
_10 = _7;
RET = _13 as usize;
_11 = _9;
_11 = _2;
_13 = -(-117434396282142345240634752033628248883_i128);
Goto(bb1)
}
bb1 = {
_12 = _1 - RET;
_10 = _8;
_1 = _12 + RET;
_7 = _8;
_14 = (-104_isize) as f32;
_2 = _3;
_8 = _7;
_2 = _7;
_6 = _5;
_5 = _4;
_16 = -53_isize;
_14 = (-693684899_i32) as f32;
_10 = _8;
_8 = _10;
Goto(bb2)
}
bb2 = {
_1 = RET;
_14 = 25498_u16 as f32;
_6 = _4 == _4;
_10 = _9;
_17 = &_13;
Goto(bb3)
}
bb3 = {
_4 = _5 | _5;
_3 = _10;
_2 = _3;
_5 = _6 & _4;
_16 = 9223372036854775807_isize;
_4 = (*_17) == (*_17);
_8 = _10;
_8 = _3;
_8 = _11;
_18.1.1.1 = 32988_u16 | 18505_u16;
_10 = _9;
_7 = _2;
_13 = 71916207899191389362037037388860536223_i128 ^ 129121812324077673768939079763713410598_i128;
_18.1.2 = _13 << _13;
_16 = (-38_isize) ^ (-9223372036854775808_isize);
_18.2 = 234_u8 - 80_u8;
_3 = _8;
_8 = _3;
_10 = _9;
Call(_13 = fn11(_16, _14, _1, _11, _2, _3, _2, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18.1.3 = 7259147747790993910_u64;
_9 = _7;
_20 = !_16;
_4 = _6;
_1 = _12 * _12;
_18.1.0 = !_5;
_17 = &_18.1.2;
_18.2 = 0_u8 | 240_u8;
_1 = !RET;
_18.1.1.0 = -55_i8;
_8 = _3;
_18.0 = _18.1.3 + _18.1.3;
match _18.1.3 {
0 => bb3,
1 => bb2,
7259147747790993910 => bb6,
_ => bb5
}
}
bb5 = {
_1 = RET;
_14 = 25498_u16 as f32;
_6 = _4 == _4;
_10 = _9;
_17 = &_13;
Goto(bb3)
}
bb6 = {
_22 = _4;
_10 = _2;
_22 = !_18.1.0;
_18.0 = _18.1.3;
_18.0 = _18.1.3;
_13 = (*_17);
_10 = _2;
_26 = [(*_17)];
_18.1.1.1 = 43285_u16;
match _18.1.3 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
7259147747790993910 => bb10,
_ => bb9
}
}
bb7 = {
_1 = RET;
_14 = 25498_u16 as f32;
_6 = _4 == _4;
_10 = _9;
_17 = &_13;
Goto(bb3)
}
bb8 = {
_1 = RET;
_14 = 25498_u16 as f32;
_6 = _4 == _4;
_10 = _9;
_17 = &_13;
Goto(bb3)
}
bb9 = {
_12 = _1 - RET;
_10 = _8;
_1 = _12 + RET;
_7 = _8;
_14 = (-104_isize) as f32;
_2 = _3;
_8 = _7;
_2 = _7;
_6 = _5;
_5 = _4;
_16 = -53_isize;
_14 = (-693684899_i32) as f32;
_10 = _8;
_8 = _10;
Goto(bb2)
}
bb10 = {
_7 = _3;
RET = _12;
_22 = !_18.1.0;
_8 = _2;
_21 = _18.0 | _18.0;
_21 = _3 as u64;
_20 = _16 >> (*_17);
_18.1.3 = _18.0 / _18.0;
_10 = _11;
_4 = (*_17) >= (*_17);
_5 = !_18.1.0;
_18.1.1.1 = 46439_u16 << (*_17);
_21 = !_18.1.3;
_7 = _11;
_17 = &_13;
_8 = _11;
_26 = [(*_17)];
_18.1.1.0 = 40_i8;
_18.1.2 = _12 as i128;
_18.1.3 = !_18.0;
_10 = _8;
_5 = (*_17) != (*_17);
_1 = !RET;
match _18.0 {
0 => bb11,
7259147747790993910 => bb13,
_ => bb12
}
}
bb11 = {
_18.1.3 = 7259147747790993910_u64;
_9 = _7;
_20 = !_16;
_4 = _6;
_1 = _12 * _12;
_18.1.0 = !_5;
_17 = &_18.1.2;
_18.2 = 0_u8 | 240_u8;
_1 = !RET;
_18.1.1.0 = -55_i8;
_8 = _3;
_18.0 = _18.1.3 + _18.1.3;
match _18.1.3 {
0 => bb3,
1 => bb2,
7259147747790993910 => bb6,
_ => bb5
}
}
bb12 = {
_12 = _1 - RET;
_10 = _8;
_1 = _12 + RET;
_7 = _8;
_14 = (-104_isize) as f32;
_2 = _3;
_8 = _7;
_2 = _7;
_6 = _5;
_5 = _4;
_16 = -53_isize;
_14 = (-693684899_i32) as f32;
_10 = _8;
_8 = _10;
Goto(bb2)
}
bb13 = {
_10 = _3;
_17 = &_18.1.2;
_9 = _2;
_8 = _2;
_31 = (*_17) | _18.1.2;
_25 = -_14;
_14 = _25 * _25;
_30 = Move(_17);
_3 = _9;
RET = _12;
_9 = _7;
_16 = -_20;
_31 = !_13;
_6 = _22 | _18.1.0;
_2 = _7;
match _18.1.1.0 {
0 => bb5,
1 => bb3,
40 => bb15,
_ => bb14
}
}
bb14 = {
_4 = _5 | _5;
_3 = _10;
_2 = _3;
_5 = _6 & _4;
_16 = 9223372036854775807_isize;
_4 = (*_17) == (*_17);
_8 = _10;
_8 = _3;
_8 = _11;
_18.1.1.1 = 32988_u16 | 18505_u16;
_10 = _9;
_7 = _2;
_13 = 71916207899191389362037037388860536223_i128 ^ 129121812324077673768939079763713410598_i128;
_18.1.2 = _13 << _13;
_16 = (-38_isize) ^ (-9223372036854775808_isize);
_18.2 = 234_u8 - 80_u8;
_3 = _8;
_8 = _3;
_10 = _9;
Call(_13 = fn11(_16, _14, _1, _11, _2, _3, _2, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
_31 = _18.2 as i128;
_18.1.0 = !_22;
_1 = _12;
_18.1.1.0 = -121_i8;
_34 = [_22];
_13 = (-18453_i16) as i128;
RET = _12 * _12;
Goto(bb16)
}
bb16 = {
_20 = _16 | _16;
_28 = &_18.1.2;
_25 = _21 as f32;
_8 = _10;
_33 = RET as isize;
_20 = _16 ^ _16;
_34 = [_18.1.0];
_3 = _2;
_23 = core::ptr::addr_of!(_34);
_26 = [(*_28)];
(*_23) = [_5];
(*_23) = [_6];
_18.1.3 = _18.0 + _18.0;
_12 = RET - RET;
_20 = _33 - _33;
_4 = !_18.1.0;
_16 = _33 - _33;
_18.1.0 = _18.1.2 < _31;
_37 = (*_28) ^ (*_28);
_17 = &(*_28);
(*_23) = [_4];
_11 = _8;
(*_23) = [_4];
_16 = _33 >> _18.2;
match _18.0 {
0 => bb7,
1 => bb2,
2 => bb15,
3 => bb9,
4 => bb14,
7259147747790993910 => bb18,
_ => bb17
}
}
bb17 = {
_12 = _1 - RET;
_10 = _8;
_1 = _12 + RET;
_7 = _8;
_14 = (-104_isize) as f32;
_2 = _3;
_8 = _7;
_2 = _7;
_6 = _5;
_5 = _4;
_16 = -53_isize;
_14 = (-693684899_i32) as f32;
_10 = _8;
_8 = _10;
Goto(bb2)
}
bb18 = {
_28 = &_37;
_31 = (*_28);
_40 = _14 + _14;
Goto(bb19)
}
bb19 = {
(*_23) = [_4];
_44.1 = [_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0];
_43.fld0.2 = _18.1.1.1 as u8;
_32 = 3567436815_u32 ^ 1554812694_u32;
_13 = (*_17) + (*_28);
(*_23) = [_4];
_13 = (*_17) + (*_28);
_38 = _21 as f64;
(*_23) = [_4];
_18.1.1.0 = _40 as i8;
_43.fld1 = _18.1.1.1 ^ _18.1.1.1;
_25 = -_14;
(*_23) = [_22];
_30 = &(*_28);
_33 = _16;
_18.0 = _18.1.3 * _18.1.3;
_38 = _16 as f64;
_43.fld0.1 = _3;
_31 = _38 as i128;
_16 = _40 as isize;
_15 = core::ptr::addr_of!((*_23));
_18.1.1.1 = _4 as u16;
Goto(bb20)
}
bb20 = {
_18.0 = !_21;
_47.1 = _33 * _33;
_35 = _22;
_43.fld0.0 = _18.0 >> _18.1.1.1;
(*_23) = [_4];
_22 = _6 & _6;
_9 = _7;
_23 = core::ptr::addr_of!((*_23));
_14 = (-1602217559_i32) as f32;
_47.0 = core::ptr::addr_of_mut!((*_17));
_5 = !_4;
_39 = !20595_i16;
_18.2 = _43.fld0.2;
Goto(bb21)
}
bb21 = {
_48 = _18.1.1.1 as i64;
_17 = &(*_28);
_8 = _3;
_43.fld0.2 = !_18.2;
_47.1 = _33 * _20;
Goto(bb22)
}
bb22 = {
_25 = 79246504787778388847828465537422632115_u128 as f32;
(*_23) = [_5];
(*_23) = [_4];
_11 = _8;
Goto(bb23)
}
bb23 = {
(*_23) = [_5];
_44.0 = !_4;
_52 = core::ptr::addr_of!(_24);
_5 = _6 ^ _35;
_56.fld2 = !_20;
_10 = _9;
_43.fld0 = (_18.1.3, _2, _18.2, _39);
_18.1.0 = !_6;
Goto(bb24)
}
bb24 = {
_31 = -_37;
_18.1.3 = _18.0;
Goto(bb25)
}
bb25 = {
RET = _12 + _12;
_52 = core::ptr::addr_of!((*_52));
(*_23) = [_6];
(*_23) = [_22];
_44.1 = [_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0,_18.1.1.0];
(*_23) = [_6];
_18.1.2 = (*_28);
_41 = _38 - _38;
_38 = _41 + _41;
_40 = _14 + _25;
_7 = _2;
Goto(bb26)
}
bb26 = {
Call(_62 = dump_var(Move(_16), Move(_12), Move(_3), Move(_48)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_62 = dump_var(Move(_7), Move(_13), Move(_32), Move(_21)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_62 = dump_var(Move(_8), Move(_5), Move(_1), Move(_31)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_62 = dump_var(Move(_11), _63, _63, _63), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: f32,mut _3: usize,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char) -> i128 {
mir! {
type RET = i128;
let _9: f64;
let _10: bool;
let _11: bool;
let _12: &'static mut i8;
let _13: f32;
let _14: isize;
let _15: i8;
let _16: isize;
let _17: (u64, char, u8, i16);
let _18: (f32, i128);
let _19: u64;
let _20: bool;
let _21: *const &'static mut u128;
let _22: &'static mut i16;
let _23: u128;
let _24: bool;
let _25: &'static Adt18;
let _26: *const usize;
let _27: &'static mut i16;
let _28: isize;
let _29: *mut u8;
let _30: i8;
let _31: char;
let _32: f32;
let _33: (bool, (i8, u16, Adt29), i128, u64);
let _34: f64;
let _35: *mut &'static i128;
let _36: isize;
let _37: isize;
let _38: u16;
let _39: isize;
let _40: (&'static u128, Adt29, usize);
let _41: ((*mut i128,),);
let _42: bool;
let _43: isize;
let _44: Adt63;
let _45: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _46: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _47: *mut u64;
let _48: i8;
let _49: Adt25;
let _50: isize;
let _51: i8;
let _52: f32;
let _53: u64;
let _54: ();
let _55: ();
{
RET = -(-111981986784908513684828096403810363868_i128);
_2 = 23920_u16 as f32;
RET = 94932452360447059568048478796998198567_i128;
_1 = 9223372036854775807_isize << _3;
RET = (-131901873264295775297453693246196164579_i128) & (-111328757355428065787505422135163503793_i128);
_2 = (-5009_i16) as f32;
_2 = 8379_i16 as f32;
_1 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_4 = _8;
_4 = _7;
_7 = _8;
_4 = _6;
_7 = _5;
_9 = _2 as f64;
_6 = _8;
_5 = _7;
_11 = !true;
Call(_3 = core::intrinsics::bswap(3268785827418071502_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 9223372036854775807_isize;
_1 = _2 as isize;
_6 = _4;
Goto(bb2)
}
bb2 = {
_10 = _11 ^ _11;
_4 = _7;
_1 = (-100_isize) & (-127_isize);
_5 = _7;
_5 = _8;
_10 = !_11;
_3 = !7701696617156466298_usize;
_2 = 64861_u16 as f32;
_11 = _6 >= _6;
_5 = _7;
_9 = 148_u8 as f64;
Call(_13 = fn12(_1, _5, RET, _7, _6, _10, _8, _6, _11, _2, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb4 = {
_18.0 = 37_i8 as f32;
_17.2 = _5 as u8;
_18.1 = RET & RET;
_6 = _7;
_17 = (530651678564576488_u64, _4, 119_u8, (-20646_i16));
_13 = _18.0;
_11 = _10;
_2 = -_13;
Call(_14 = core::intrinsics::bswap(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = (_2, RET);
_18 = (_13, RET);
_15 = !79_i8;
_17.1 = _4;
_15 = (-35_i8) ^ (-51_i8);
Goto(bb6)
}
bb6 = {
_17 = (10145486102111300230_u64, _4, 213_u8, (-28470_i16));
_20 = !_11;
_12 = &mut _15;
(*_12) = (-114_i8) | 100_i8;
_7 = _4;
(*_12) = 43_i8 + (-91_i8);
(*_12) = 92_i8 >> _17.0;
_22 = &mut _17.3;
(*_12) = !75_i8;
(*_12) = 5918268642991115029_i64 as i8;
_2 = _18.0 * _18.0;
(*_22) = (-1849_i16);
_18.0 = _2 - _2;
(*_12) = _9 as i8;
(*_22) = 30209_i16;
(*_22) = 21986_i16 + (-19118_i16);
_7 = _5;
_19 = 12042051681373053410_u64 << (*_12);
_19 = 335923200583885781244749789487271856179_u128 as u64;
_8 = _7;
(*_22) = 113_u8 as i16;
(*_22) = _19 as i16;
(*_12) = (-37_i8) - 2_i8;
Goto(bb7)
}
bb7 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb8 = {
(*_12) = 96_i8 | (-79_i8);
_4 = _8;
_24 = (*_12) < (*_12);
(*_22) = (-8943_i16) | (-21538_i16);
(*_12) = _1 as i8;
_5 = _6;
(*_12) = -118_i8;
(*_12) = 1729042838_u32 as i8;
(*_22) = 7274915743153759536_i64 as i16;
(*_22) = 26253_i16 - (-29181_i16);
(*_22) = -(-30367_i16);
(*_22) = _16 as i16;
(*_22) = !10041_i16;
(*_22) = (*_12) as i16;
_6 = _4;
Goto(bb9)
}
bb9 = {
(*_22) = 642109558_u32 as i16;
(*_22) = 9063_i16;
_19 = !11253987012568691999_u64;
(*_22) = 4610_i16;
(*_12) = -(-30_i8);
(*_12) = !64_i8;
(*_22) = (-2627_i16);
_4 = _6;
(*_22) = 4247_i16 | 18769_i16;
_34 = _9 * _9;
_26 = core::ptr::addr_of!(_3);
(*_12) = 115_i8 | (-121_i8);
(*_12) = _23 as i8;
_20 = !_10;
(*_22) = (*_26) as i16;
(*_12) = _1 as i8;
(*_22) = (-10265_i16) >> (*_12);
(*_12) = (-45_i8);
(*_26) = !5_usize;
_28 = !_16;
_6 = _5;
_33.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _34,fld3: (-517057647_i32) };
(*_22) = -(-26355_i16);
match (*_12) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211411 => bb11,
_ => bb10
}
}
bb10 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb11 = {
(*_22) = (-6764_i16);
_32 = _18.0 - _18.0;
_31 = _7;
(*_12) = 76_i8 + 80_i8;
(*_12) = -(-90_i8);
(*_12) = 20_i8 ^ (-29_i8);
_33.1.0 = (*_12) << (*_12);
_28 = _16;
_28 = _1 & _1;
_33.3 = _28 as u64;
_33.2 = _18.1;
_33.1.1 = 16693_u16;
_11 = _33.3 != _33.3;
_9 = -Field::<f64>(Variant(_33.1.2, 0), 2);
(*_22) = 6533_i16 & 26290_i16;
place!(Field::<i128>(Variant(_33.1.2, 0), 0)) = _33.2 >> (*_22);
(*_22) = !27595_i16;
(*_12) = -_33.1.0;
_27 = &mut (*_22);
(*_27) = (-468794481_i32) as i16;
Goto(bb12)
}
bb12 = {
(*_12) = _33.1.0;
_31 = _8;
(*_27) = 12932_i16;
_28 = _16 - _16;
_32 = _13 - _2;
(*_12) = _32 as i8;
_31 = _8;
_14 = _1 - _28;
_3 = 3_usize;
_1 = !_28;
_36 = _1 + _1;
_14 = _1 ^ _1;
_6 = _8;
_20 = !_10;
(*_27) = (-22296_i16) << _33.1.0;
(*_12) = _33.1.0 | _33.1.0;
(*_27) = (-1106_i16) + 9412_i16;
(*_12) = _33.1.0 ^ _33.1.0;
Goto(bb13)
}
bb13 = {
(*_27) = _33.3 as i16;
place!(Field::<*const usize>(Variant(_33.1.2, 0), 1)) = core::ptr::addr_of!(_3);
_19 = 4918166608695409629_i64 as u64;
(*_27) = 31983_i16;
_33.2 = Field::<i128>(Variant(_33.1.2, 0), 0) * RET;
(*_27) = (-32647_i16);
_10 = (*_12) != (*_12);
(*_12) = _33.1.0 >> Field::<i128>(Variant(_33.1.2, 0), 0);
_33.1.1 = 41045_u16 ^ 47850_u16;
(*_12) = _33.1.0;
(*_12) = !_33.1.0;
_31 = _7;
(*_27) = 22412_i16 + 17311_i16;
(*_27) = _33.3 as i16;
(*_12) = _33.1.0 | _33.1.0;
(*_12) = _33.1.0 - _33.1.0;
(*_27) = (-4413_i16) - 30311_i16;
place!(Field::<i32>(Variant(_33.1.2, 0), 3)) = -(-190269101_i32);
(*_12) = _33.1.0 ^ _33.1.0;
_1 = (*_27) as isize;
match _3 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb19,
_ => bb18
}
}
bb14 = {
_10 = _11 ^ _11;
_4 = _7;
_1 = (-100_isize) & (-127_isize);
_5 = _7;
_5 = _8;
_10 = !_11;
_3 = !7701696617156466298_usize;
_2 = 64861_u16 as f32;
_11 = _6 >= _6;
_5 = _7;
_9 = 148_u8 as f64;
Call(_13 = fn12(_1, _5, RET, _7, _6, _10, _8, _6, _11, _2, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_18.0 = 37_i8 as f32;
_17.2 = _5 as u8;
_18.1 = RET & RET;
_6 = _7;
_17 = (530651678564576488_u64, _4, 119_u8, (-20646_i16));
_13 = _18.0;
_11 = _10;
_2 = -_13;
Call(_14 = core::intrinsics::bswap(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb17 = {
(*_22) = 642109558_u32 as i16;
(*_22) = 9063_i16;
_19 = !11253987012568691999_u64;
(*_22) = 4610_i16;
(*_12) = -(-30_i8);
(*_12) = !64_i8;
(*_22) = (-2627_i16);
_4 = _6;
(*_22) = 4247_i16 | 18769_i16;
_34 = _9 * _9;
_26 = core::ptr::addr_of!(_3);
(*_12) = 115_i8 | (-121_i8);
(*_12) = _23 as i8;
_20 = !_10;
(*_22) = (*_26) as i16;
(*_12) = _1 as i8;
(*_22) = (-10265_i16) >> (*_12);
(*_12) = (-45_i8);
(*_26) = !5_usize;
_28 = !_16;
_6 = _5;
_33.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _34,fld3: (-517057647_i32) };
(*_22) = -(-26355_i16);
match (*_12) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211411 => bb11,
_ => bb10
}
}
bb18 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb19 = {
(*_27) = -(-17637_i16);
(*_12) = -_33.1.0;
(*_12) = Field::<f64>(Variant(_33.1.2, 0), 2) as i8;
(*_27) = 8351_i16;
_12 = &mut _33.1.0;
(*_12) = _9 as i8;
match (*_27) {
0 => bb9,
1 => bb18,
2 => bb3,
3 => bb20,
4 => bb21,
5 => bb22,
8351 => bb24,
_ => bb23
}
}
bb20 = {
(*_22) = 642109558_u32 as i16;
(*_22) = 9063_i16;
_19 = !11253987012568691999_u64;
(*_22) = 4610_i16;
(*_12) = -(-30_i8);
(*_12) = !64_i8;
(*_22) = (-2627_i16);
_4 = _6;
(*_22) = 4247_i16 | 18769_i16;
_34 = _9 * _9;
_26 = core::ptr::addr_of!(_3);
(*_12) = 115_i8 | (-121_i8);
(*_12) = _23 as i8;
_20 = !_10;
(*_22) = (*_26) as i16;
(*_12) = _1 as i8;
(*_22) = (-10265_i16) >> (*_12);
(*_12) = (-45_i8);
(*_26) = !5_usize;
_28 = !_16;
_6 = _5;
_33.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _34,fld3: (-517057647_i32) };
(*_22) = -(-26355_i16);
match (*_12) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211411 => bb11,
_ => bb10
}
}
bb21 = {
(*_22) = 642109558_u32 as i16;
(*_22) = 9063_i16;
_19 = !11253987012568691999_u64;
(*_22) = 4610_i16;
(*_12) = -(-30_i8);
(*_12) = !64_i8;
(*_22) = (-2627_i16);
_4 = _6;
(*_22) = 4247_i16 | 18769_i16;
_34 = _9 * _9;
_26 = core::ptr::addr_of!(_3);
(*_12) = 115_i8 | (-121_i8);
(*_12) = _23 as i8;
_20 = !_10;
(*_22) = (*_26) as i16;
(*_12) = _1 as i8;
(*_22) = (-10265_i16) >> (*_12);
(*_12) = (-45_i8);
(*_26) = !5_usize;
_28 = !_16;
_6 = _5;
_33.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _34,fld3: (-517057647_i32) };
(*_22) = -(-26355_i16);
match (*_12) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211411 => bb11,
_ => bb10
}
}
bb22 = {
_1 = 9223372036854775807_isize;
_1 = _2 as isize;
_6 = _4;
Goto(bb2)
}
bb23 = {
_18.0 = 37_i8 as f32;
_17.2 = _5 as u8;
_18.1 = RET & RET;
_6 = _7;
_17 = (530651678564576488_u64, _4, 119_u8, (-20646_i16));
_13 = _18.0;
_11 = _10;
_2 = -_13;
Call(_14 = core::intrinsics::bswap(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb24 = {
(*_12) = (-32_i8) & (-122_i8);
(*_27) = 19897_i16;
(*_12) = !(-15_i8);
_18.1 = -RET;
_41.0.0 = core::ptr::addr_of_mut!(RET);
(*_12) = 5_i8 | (-124_i8);
(*_27) = 21859_i16;
_34 = -_9;
(*_12) = (-28_i8);
(*_12) = !47_i8;
(*_12) = 90_i8;
_9 = _34 - _34;
_1 = _28;
_9 = _34 * _34;
(*_27) = (-24703_i16) - (-9107_i16);
_10 = (*_12) <= (*_12);
(*_12) = 99_i8 << (*_27);
(*_27) = 4888_i16 * (-18883_i16);
_41.0.0 = core::ptr::addr_of_mut!(_18.1);
(*_27) = (-14093_i16) & (-28908_i16);
_37 = _28 * _36;
match _3 {
0 => bb23,
1 => bb6,
2 => bb17,
4 => bb13,
5 => bb25,
3 => bb27,
_ => bb26
}
}
bb25 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb26 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb27 = {
_40.0 = &_23;
_28 = _37 | _14;
_40.2 = RET as usize;
(*_12) = 1033716290_i32 as i8;
(*_27) = -(-7092_i16);
_36 = _23 as isize;
(*_12) = _9 as i8;
(*_27) = (-29866_i16) | (-11298_i16);
_24 = !_20;
_38 = !24964_u16;
_31 = _6;
_6 = _7;
(*_12) = 77_i8 * (-58_i8);
(*_12) = 120_i8;
(*_27) = (-15898_i16) - (-16951_i16);
(*_27) = 1737_i16;
_7 = _31;
(*_27) = (-24187_i16) & (-30719_i16);
_16 = _28 & _28;
match (*_12) {
0 => bb18,
1 => bb4,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
120 => bb34,
_ => bb33
}
}
bb28 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb29 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb30 = {
(*_22) = (-6764_i16);
_32 = _18.0 - _18.0;
_31 = _7;
(*_12) = 76_i8 + 80_i8;
(*_12) = -(-90_i8);
(*_12) = 20_i8 ^ (-29_i8);
_33.1.0 = (*_12) << (*_12);
_28 = _16;
_28 = _1 & _1;
_33.3 = _28 as u64;
_33.2 = _18.1;
_33.1.1 = 16693_u16;
_11 = _33.3 != _33.3;
_9 = -Field::<f64>(Variant(_33.1.2, 0), 2);
(*_22) = 6533_i16 & 26290_i16;
place!(Field::<i128>(Variant(_33.1.2, 0), 0)) = _33.2 >> (*_22);
(*_22) = !27595_i16;
(*_12) = -_33.1.0;
_27 = &mut (*_22);
(*_27) = (-468794481_i32) as i16;
Goto(bb12)
}
bb31 = {
_18.0 = 37_i8 as f32;
_17.2 = _5 as u8;
_18.1 = RET & RET;
_6 = _7;
_17 = (530651678564576488_u64, _4, 119_u8, (-20646_i16));
_13 = _18.0;
_11 = _10;
_2 = -_13;
Call(_14 = core::intrinsics::bswap(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb32 = {
(*_12) = _33.1.0;
_31 = _8;
(*_27) = 12932_i16;
_28 = _16 - _16;
_32 = _13 - _2;
(*_12) = _32 as i8;
_31 = _8;
_14 = _1 - _28;
_3 = 3_usize;
_1 = !_28;
_36 = _1 + _1;
_14 = _1 ^ _1;
_6 = _8;
_20 = !_10;
(*_27) = (-22296_i16) << _33.1.0;
(*_12) = _33.1.0 | _33.1.0;
(*_27) = (-1106_i16) + 9412_i16;
(*_12) = _33.1.0 ^ _33.1.0;
Goto(bb13)
}
bb33 = {
(*_22) = 642109558_u32 as i16;
(*_22) = 9063_i16;
_19 = !11253987012568691999_u64;
(*_22) = 4610_i16;
(*_12) = -(-30_i8);
(*_12) = !64_i8;
(*_22) = (-2627_i16);
_4 = _6;
(*_22) = 4247_i16 | 18769_i16;
_34 = _9 * _9;
_26 = core::ptr::addr_of!(_3);
(*_12) = 115_i8 | (-121_i8);
(*_12) = _23 as i8;
_20 = !_10;
(*_22) = (*_26) as i16;
(*_12) = _1 as i8;
(*_22) = (-10265_i16) >> (*_12);
(*_12) = (-45_i8);
(*_26) = !5_usize;
_28 = !_16;
_6 = _5;
_33.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _34,fld3: (-517057647_i32) };
(*_22) = -(-26355_i16);
match (*_12) {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211411 => bb11,
_ => bb10
}
}
bb34 = {
(*_12) = -46_i8;
(*_12) = 68_i8;
_31 = _4;
(*_27) = (-14849_i16);
_41.0.0 = core::ptr::addr_of_mut!(RET);
_16 = _3 as isize;
(*_12) = (-32_i8) + 39_i8;
_1 = _14 & _14;
_45.3.0.1.0 = (*_12) << (*_27);
_45.1 = 353410116_i32 * (-795441180_i32);
(*_12) = _45.3.0.1.0 & _45.3.0.1.0;
_4 = _6;
_45.3.0.1.0 = !(*_12);
_16 = _1 | _37;
match _3 {
0 => bb2,
1 => bb35,
3 => bb37,
_ => bb36
}
}
bb35 = {
(*_27) = _33.3 as i16;
place!(Field::<*const usize>(Variant(_33.1.2, 0), 1)) = core::ptr::addr_of!(_3);
_19 = 4918166608695409629_i64 as u64;
(*_27) = 31983_i16;
_33.2 = Field::<i128>(Variant(_33.1.2, 0), 0) * RET;
(*_27) = (-32647_i16);
_10 = (*_12) != (*_12);
(*_12) = _33.1.0 >> Field::<i128>(Variant(_33.1.2, 0), 0);
_33.1.1 = 41045_u16 ^ 47850_u16;
(*_12) = _33.1.0;
(*_12) = !_33.1.0;
_31 = _7;
(*_27) = 22412_i16 + 17311_i16;
(*_27) = _33.3 as i16;
(*_12) = _33.1.0 | _33.1.0;
(*_12) = _33.1.0 - _33.1.0;
(*_27) = (-4413_i16) - 30311_i16;
place!(Field::<i32>(Variant(_33.1.2, 0), 3)) = -(-190269101_i32);
(*_12) = _33.1.0 ^ _33.1.0;
_1 = (*_27) as isize;
match _3 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb19,
_ => bb18
}
}
bb36 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb37 = {
(*_12) = _45.3.0.1.0 - _45.3.0.1.0;
(*_12) = (*_27) as i8;
(*_12) = _45.3.0.1.0 << _45.3.0.1.0;
_18 = (_2, RET);
(*_12) = _45.3.0.1.0;
_26 = core::ptr::addr_of!(_3);
(*_26) = _40.2;
(*_12) = _45.3.0.1.0 ^ _45.3.0.1.0;
_45.3.0.2 = -_18.1;
_16 = _28 ^ _37;
(*_27) = -(-23049_i16);
_49.fld0 = (_19, _4, 238_u8, (*_27));
(*_26) = _19 as usize;
(*_27) = _49.fld0.3 * _49.fld0.3;
match _49.fld0.2 {
0 => bb38,
1 => bb39,
238 => bb41,
_ => bb40
}
}
bb38 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb39 = {
(*_12) = 96_i8 | (-79_i8);
_4 = _8;
_24 = (*_12) < (*_12);
(*_22) = (-8943_i16) | (-21538_i16);
(*_12) = _1 as i8;
_5 = _6;
(*_12) = -118_i8;
(*_12) = 1729042838_u32 as i8;
(*_22) = 7274915743153759536_i64 as i16;
(*_22) = 26253_i16 - (-29181_i16);
(*_22) = -(-30367_i16);
(*_22) = _16 as i16;
(*_22) = !10041_i16;
(*_22) = (*_12) as i16;
_6 = _4;
Goto(bb9)
}
bb40 = {
(*_12) = (-32_i8) & (-122_i8);
(*_27) = 19897_i16;
(*_12) = !(-15_i8);
_18.1 = -RET;
_41.0.0 = core::ptr::addr_of_mut!(RET);
(*_12) = 5_i8 | (-124_i8);
(*_27) = 21859_i16;
_34 = -_9;
(*_12) = (-28_i8);
(*_12) = !47_i8;
(*_12) = 90_i8;
_9 = _34 - _34;
_1 = _28;
_9 = _34 * _34;
(*_27) = (-24703_i16) - (-9107_i16);
_10 = (*_12) <= (*_12);
(*_12) = 99_i8 << (*_27);
(*_27) = 4888_i16 * (-18883_i16);
_41.0.0 = core::ptr::addr_of_mut!(_18.1);
(*_27) = (-14093_i16) & (-28908_i16);
_37 = _28 * _36;
match _3 {
0 => bb23,
1 => bb6,
2 => bb17,
4 => bb13,
5 => bb25,
3 => bb27,
_ => bb26
}
}
bb41 = {
(*_26) = _40.2 | _40.2;
_45.2 = [_24];
_49.fld1 = _38 ^ _38;
(*_12) = _45.3.0.1.0 & _45.3.0.1.0;
(*_27) = _49.fld0.3 - _49.fld0.3;
Goto(bb42)
}
bb42 = {
(*_26) = _49.fld0.1 as usize;
(*_26) = _40.2 << (*_12);
_52 = -_18.0;
match _49.fld0.2 {
0 => bb15,
1 => bb24,
2 => bb43,
3 => bb44,
4 => bb45,
5 => bb46,
6 => bb47,
238 => bb49,
_ => bb48
}
}
bb43 = {
_6 = _4;
(*_12) = -33_i8;
_5 = _8;
(*_12) = -37_i8;
(*_12) = -91_i8;
_4 = _8;
(*_22) = (-17715_i16) + 11184_i16;
(*_22) = (-23993_i16) >> _19;
(*_12) = (-3324979986715230115_i64) as i8;
(*_12) = !(-14_i8);
(*_22) = (*_12) as i16;
_23 = _6 as u128;
(*_12) = -(-8_i8);
(*_22) = 23334_i16;
_18 = (_2, RET);
(*_12) = (-47_i8) & 111_i8;
_16 = -_1;
(*_22) = 14494_i16;
(*_22) = (-24933_i16);
(*_12) = !(-115_i8);
(*_12) = -(-60_i8);
_9 = _1 as f64;
_16 = _1;
(*_12) = (-101_i8) ^ (-105_i8);
(*_22) = _23 as i16;
_9 = _3 as f64;
(*_22) = _18.1 as i16;
_5 = _4;
Goto(bb8)
}
bb44 = {
(*_12) = (-32_i8) & (-122_i8);
(*_27) = 19897_i16;
(*_12) = !(-15_i8);
_18.1 = -RET;
_41.0.0 = core::ptr::addr_of_mut!(RET);
(*_12) = 5_i8 | (-124_i8);
(*_27) = 21859_i16;
_34 = -_9;
(*_12) = (-28_i8);
(*_12) = !47_i8;
(*_12) = 90_i8;
_9 = _34 - _34;
_1 = _28;
_9 = _34 * _34;
(*_27) = (-24703_i16) - (-9107_i16);
_10 = (*_12) <= (*_12);
(*_12) = 99_i8 << (*_27);
(*_27) = 4888_i16 * (-18883_i16);
_41.0.0 = core::ptr::addr_of_mut!(_18.1);
(*_27) = (-14093_i16) & (-28908_i16);
_37 = _28 * _36;
match _3 {
0 => bb23,
1 => bb6,
2 => bb17,
4 => bb13,
5 => bb25,
3 => bb27,
_ => bb26
}
}
bb45 = {
_2 = _13 * _13;
RET = _3 as i128;
_9 = 6305_u16 as f64;
_3 = !7074894801111122363_usize;
_7 = _5;
_10 = _3 >= _3;
_13 = RET as f32;
_7 = _4;
_11 = _10;
_10 = _11;
_4 = _7;
_4 = _5;
_10 = _11 | _11;
_1 = -(-82_isize);
_2 = -_13;
_16 = _1 * _1;
RET = 157635083952495076199098149437884443481_i128 ^ (-99848121294121966784989339271147369756_i128);
_11 = !_10;
_17 = (15839982975674183416_u64, _7, 232_u8, (-25320_i16));
_17.0 = 9413084826546569196_u64 | 5869511614055671295_u64;
_2 = _13;
_2 = (-1351654794_i32) as f32;
Goto(bb4)
}
bb46 = {
_18.0 = 37_i8 as f32;
_17.2 = _5 as u8;
_18.1 = RET & RET;
_6 = _7;
_17 = (530651678564576488_u64, _4, 119_u8, (-20646_i16));
_13 = _18.0;
_11 = _10;
_2 = -_13;
Call(_14 = core::intrinsics::bswap(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb47 = {
(*_12) = (-32_i8) & (-122_i8);
(*_27) = 19897_i16;
(*_12) = !(-15_i8);
_18.1 = -RET;
_41.0.0 = core::ptr::addr_of_mut!(RET);
(*_12) = 5_i8 | (-124_i8);
(*_27) = 21859_i16;
_34 = -_9;
(*_12) = (-28_i8);
(*_12) = !47_i8;
(*_12) = 90_i8;
_9 = _34 - _34;
_1 = _28;
_9 = _34 * _34;
(*_27) = (-24703_i16) - (-9107_i16);
_10 = (*_12) <= (*_12);
(*_12) = 99_i8 << (*_27);
(*_27) = 4888_i16 * (-18883_i16);
_41.0.0 = core::ptr::addr_of_mut!(_18.1);
(*_27) = (-14093_i16) & (-28908_i16);
_37 = _28 * _36;
match _3 {
0 => bb23,
1 => bb6,
2 => bb17,
4 => bb13,
5 => bb25,
3 => bb27,
_ => bb26
}
}
bb48 = {
_1 = 9223372036854775807_isize;
_1 = _2 as isize;
_6 = _4;
Goto(bb2)
}
bb49 = {
_50 = _37 << (*_26);
_16 = !_37;
(*_27) = _49.fld0.3 ^ _49.fld0.3;
_23 = 120454253559972082820355063702524364709_u128;
_34 = -_9;
_48 = _23 as i8;
_45.3.0.1.2 = Adt29::Variant0 { fld0: _18.1,fld1: Move(_26),fld2: _9,fld3: _45.1 };
_49.fld0.2 = _13 as u8;
(*_12) = _45.3.0.1.0 << RET;
_37 = _1 >> (*_12);
(*_12) = _45.1 as i8;
_10 = !_11;
(*_12) = _45.3.0.1.0;
_45.1 = Field::<i32>(Variant(_45.3.0.1.2, 0), 3) & Field::<i32>(Variant(_45.3.0.1.2, 0), 3);
_47 = core::ptr::addr_of_mut!(_49.fld0.0);
(*_12) = _48 << (*_27);
Goto(bb50)
}
bb50 = {
Call(_54 = dump_var(Move(_37), Move(_24), Move(_20), Move(_15)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_54 = dump_var(Move(_4), Move(_36), Move(_10), Move(_31)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_54 = dump_var(Move(_5), Move(_3), Move(_17), Move(_48)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: char,mut _3: i128,mut _4: char,mut _5: char,mut _6: bool,mut _7: char,mut _8: char,mut _9: bool,mut _10: f32,mut _11: isize) -> f32 {
mir! {
type RET = f32;
let _12: char;
let _13: Adt26;
let _14: isize;
let _15: isize;
let _16: *mut usize;
let _17: [i16; 2];
let _18: char;
let _19: bool;
let _20: *mut u8;
let _21: Adt48;
let _22: i128;
let _23: isize;
let _24: bool;
let _25: char;
let _26: isize;
let _27: isize;
let _28: isize;
let _29: f64;
let _30: f64;
let _31: bool;
let _32: char;
let _33: (u64, char, u8, i16);
let _34: *mut u32;
let _35: f64;
let _36: u128;
let _37: (*mut i128, isize);
let _38: &'static u128;
let _39: [u128; 7];
let _40: char;
let _41: char;
let _42: f32;
let _43: &'static mut i8;
let _44: ((bool, (i8, u16, Adt29), i128, u64),);
let _45: i16;
let _46: &'static mut f32;
let _47: [i128; 1];
let _48: Adt25;
let _49: [i8; 8];
let _50: f32;
let _51: bool;
let _52: u128;
let _53: isize;
let _54: f64;
let _55: f64;
let _56: f32;
let _57: &'static u128;
let _58: &'static mut i8;
let _59: ((bool, (i8, u16, Adt29), i128, u64),);
let _60: i8;
let _61: i16;
let _62: f32;
let _63: (Adt26, (i8, u16, Adt29), (u64, char, u8, i16), (u32,));
let _64: i32;
let _65: &'static u128;
let _66: *const *const &'static mut u128;
let _67: [u64; 3];
let _68: u8;
let _69: &'static mut i8;
let _70: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _71: i16;
let _72: &'static u128;
let _73: *const [bool; 1];
let _74: i32;
let _75: *const (u32,);
let _76: i8;
let _77: i8;
let _78: char;
let _79: usize;
let _80: *mut i128;
let _81: i16;
let _82: bool;
let _83: i64;
let _84: ();
let _85: ();
{
_6 = _9;
_12 = _4;
_7 = _5;
_4 = _12;
_5 = _2;
_5 = _4;
RET = _3 as f32;
Goto(bb1)
}
bb1 = {
_11 = _1;
_13 = Adt26::Variant2 { fld0: (-706393350_i32),fld1: 31093_u16 };
_11 = _1 - _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = 1294_u16 ^ 22280_u16;
RET = _10;
RET = _3 as f32;
_3 = Field::<u16>(Variant(_13, 2), 1) as i128;
_15 = _11 ^ _11;
_13 = Adt26::Variant1 { fld0: 62736_u16 };
RET = -_10;
_13 = Adt26::Variant1 { fld0: 63968_u16 };
_11 = !_15;
_14 = -_11;
Goto(bb2)
}
bb2 = {
_7 = _12;
_13 = Adt26::Variant2 { fld0: (-1622130750_i32),fld1: 53160_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-2040654502_i32) ^ 1591315410_i32;
_14 = _11;
_6 = _4 < _12;
_18 = _4;
_9 = _6 | _6;
place!(Field::<i32>(Variant(_13, 2), 0)) = 416997307_i32;
_14 = _15 * _11;
_1 = _14 << _15;
_12 = _7;
_4 = _18;
place!(Field::<i32>(Variant(_13, 2), 0)) = !1290830803_i32;
_1 = _11 | _14;
_8 = _2;
place!(Field::<u16>(Variant(_13, 2), 1)) = 3859_u16 & 20012_u16;
_7 = _4;
_11 = Field::<u16>(Variant(_13, 2), 1) as isize;
_17 = [(-22627_i16),27943_i16];
_1 = _15 >> _14;
_13 = Adt26::Variant1 { fld0: 41247_u16 };
Goto(bb3)
}
bb3 = {
_9 = _6 <= _6;
_15 = _1 & _1;
_5 = _2;
RET = -_10;
_4 = _2;
place!(Field::<u16>(Variant(_13, 1), 0)) = 47609_u16 * 20346_u16;
_5 = _2;
_6 = _9;
_7 = _8;
_13 = Adt26::Variant2 { fld0: 1471213956_i32,fld1: 18357_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-1434931483_i32) >> _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = !42326_u16;
_8 = _5;
_17 = [(-21696_i16),(-17087_i16)];
_4 = _7;
_4 = _18;
_17 = [15266_i16,(-13924_i16)];
RET = _1 as f32;
place!(Field::<u16>(Variant(_13, 2), 1)) = 44988_u16 * 60321_u16;
_17 = [(-21427_i16),5801_i16];
_11 = _1;
_1 = !_15;
place!(Field::<u16>(Variant(_13, 2), 1)) = 43460_u16 - 49401_u16;
RET = _10;
_19 = Field::<i32>(Variant(_13, 2), 0) == Field::<i32>(Variant(_13, 2), 0);
_1 = _11;
_11 = _15;
Goto(bb4)
}
bb4 = {
_3 = (-125165898102373079162278541302314982849_i128) | (-109793664217430576625542900271680307296_i128);
_7 = _2;
_18 = _5;
_1 = _11 << Field::<i32>(Variant(_13, 2), 0);
_19 = _9;
_8 = _7;
_1 = -_15;
RET = _10 * _10;
_5 = _2;
_1 = _15 ^ _11;
_5 = _7;
Call(_25 = fn13(Move(_13), _7, _18, _5, _7, _15, _6, _15, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12 = _8;
_24 = !_19;
_10 = 198_u8 as f32;
_26 = _1 | _15;
_22 = !_3;
_22 = _3;
_4 = _7;
_6 = _26 < _1;
_4 = _25;
_7 = _4;
RET = (-46_i8) as f32;
_2 = _5;
_2 = _8;
_25 = _18;
_4 = _2;
_2 = _7;
RET = -_10;
_13 = Adt26::Variant1 { fld0: 10155_u16 };
_27 = (-10664_i16) as isize;
_27 = !_11;
_19 = _6 & _6;
Goto(bb6)
}
bb6 = {
_8 = _7;
_17 = [(-31654_i16),10144_i16];
_19 = _6 < _6;
Goto(bb7)
}
bb7 = {
_29 = _3 as f64;
_5 = _25;
_5 = _12;
RET = -_10;
_6 = _26 <= _1;
_15 = _26 * _26;
_26 = _15 * _1;
_27 = _26;
_23 = _11 * _1;
place!(Field::<u16>(Variant(_13, 1), 0)) = 54758_u16;
_25 = _5;
_27 = 5_usize as isize;
_1 = _23 + _15;
_12 = _8;
Goto(bb8)
}
bb8 = {
_15 = _23 | _23;
place!(Field::<u16>(Variant(_13, 1), 0)) = !28246_u16;
_22 = 334144132496994758236170519767213630416_u128 as i128;
_18 = _8;
_29 = (-20055_i16) as f64;
_5 = _25;
_4 = _7;
_24 = _1 != _1;
_4 = _2;
RET = _10 + _10;
_31 = _24;
_4 = _7;
_19 = _31;
_4 = _8;
_19 = _7 == _18;
_13 = Adt26::Variant2 { fld0: 734384731_i32,fld1: 28062_u16 };
RET = _10 + _10;
_32 = _12;
_12 = _32;
Goto(bb9)
}
bb9 = {
_31 = _6 | _24;
_27 = (-7383107995608547932_i64) as isize;
_32 = _7;
_28 = _23;
_1 = _23;
_37.1 = _26 + _28;
_25 = _2;
_37.1 = !_26;
_13 = Adt26::Variant2 { fld0: 1507175870_i32,fld1: 52070_u16 };
_31 = _24;
_36 = 63119000960803199805342663336249443978_u128;
_20 = core::ptr::addr_of_mut!(_33.2);
_30 = _29 * _29;
(*_20) = !255_u8;
_6 = _24;
(*_20) = !223_u8;
_2 = _8;
_3 = !_22;
Goto(bb10)
}
bb10 = {
match _36 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
63119000960803199805342663336249443978 => bb16,
_ => bb15
}
}
bb11 = {
_9 = _6 <= _6;
_15 = _1 & _1;
_5 = _2;
RET = -_10;
_4 = _2;
place!(Field::<u16>(Variant(_13, 1), 0)) = 47609_u16 * 20346_u16;
_5 = _2;
_6 = _9;
_7 = _8;
_13 = Adt26::Variant2 { fld0: 1471213956_i32,fld1: 18357_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-1434931483_i32) >> _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = !42326_u16;
_8 = _5;
_17 = [(-21696_i16),(-17087_i16)];
_4 = _7;
_4 = _18;
_17 = [15266_i16,(-13924_i16)];
RET = _1 as f32;
place!(Field::<u16>(Variant(_13, 2), 1)) = 44988_u16 * 60321_u16;
_17 = [(-21427_i16),5801_i16];
_11 = _1;
_1 = !_15;
place!(Field::<u16>(Variant(_13, 2), 1)) = 43460_u16 - 49401_u16;
RET = _10;
_19 = Field::<i32>(Variant(_13, 2), 0) == Field::<i32>(Variant(_13, 2), 0);
_1 = _11;
_11 = _15;
Goto(bb4)
}
bb12 = {
_15 = _23 | _23;
place!(Field::<u16>(Variant(_13, 1), 0)) = !28246_u16;
_22 = 334144132496994758236170519767213630416_u128 as i128;
_18 = _8;
_29 = (-20055_i16) as f64;
_5 = _25;
_4 = _7;
_24 = _1 != _1;
_4 = _2;
RET = _10 + _10;
_31 = _24;
_4 = _7;
_19 = _31;
_4 = _8;
_19 = _7 == _18;
_13 = Adt26::Variant2 { fld0: 734384731_i32,fld1: 28062_u16 };
RET = _10 + _10;
_32 = _12;
_12 = _32;
Goto(bb9)
}
bb13 = {
_3 = (-125165898102373079162278541302314982849_i128) | (-109793664217430576625542900271680307296_i128);
_7 = _2;
_18 = _5;
_1 = _11 << Field::<i32>(Variant(_13, 2), 0);
_19 = _9;
_8 = _7;
_1 = -_15;
RET = _10 * _10;
_5 = _2;
_1 = _15 ^ _11;
_5 = _7;
Call(_25 = fn13(Move(_13), _7, _18, _5, _7, _15, _6, _15, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb14 = {
_8 = _7;
_17 = [(-31654_i16),10144_i16];
_19 = _6 < _6;
Goto(bb7)
}
bb15 = {
_12 = _8;
_24 = !_19;
_10 = 198_u8 as f32;
_26 = _1 | _15;
_22 = !_3;
_22 = _3;
_4 = _7;
_6 = _26 < _1;
_4 = _25;
_7 = _4;
RET = (-46_i8) as f32;
_2 = _5;
_2 = _8;
_25 = _18;
_4 = _2;
_2 = _7;
RET = -_10;
_13 = Adt26::Variant1 { fld0: 10155_u16 };
_27 = (-10664_i16) as isize;
_27 = !_11;
_19 = _6 & _6;
Goto(bb6)
}
bb16 = {
place!(Field::<u16>(Variant(_13, 2), 1)) = 2109571950163745039_u64 as u16;
(*_20) = 205_u8 << _1;
match _36 {
0 => bb3,
1 => bb4,
2 => bb17,
3 => bb18,
4 => bb19,
5 => bb20,
63119000960803199805342663336249443978 => bb22,
_ => bb21
}
}
bb17 = {
_31 = _6 | _24;
_27 = (-7383107995608547932_i64) as isize;
_32 = _7;
_28 = _23;
_1 = _23;
_37.1 = _26 + _28;
_25 = _2;
_37.1 = !_26;
_13 = Adt26::Variant2 { fld0: 1507175870_i32,fld1: 52070_u16 };
_31 = _24;
_36 = 63119000960803199805342663336249443978_u128;
_20 = core::ptr::addr_of_mut!(_33.2);
_30 = _29 * _29;
(*_20) = !255_u8;
_6 = _24;
(*_20) = !223_u8;
_2 = _8;
_3 = !_22;
Goto(bb10)
}
bb18 = {
_11 = _1;
_13 = Adt26::Variant2 { fld0: (-706393350_i32),fld1: 31093_u16 };
_11 = _1 - _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = 1294_u16 ^ 22280_u16;
RET = _10;
RET = _3 as f32;
_3 = Field::<u16>(Variant(_13, 2), 1) as i128;
_15 = _11 ^ _11;
_13 = Adt26::Variant1 { fld0: 62736_u16 };
RET = -_10;
_13 = Adt26::Variant1 { fld0: 63968_u16 };
_11 = !_15;
_14 = -_11;
Goto(bb2)
}
bb19 = {
_12 = _8;
_24 = !_19;
_10 = 198_u8 as f32;
_26 = _1 | _15;
_22 = !_3;
_22 = _3;
_4 = _7;
_6 = _26 < _1;
_4 = _25;
_7 = _4;
RET = (-46_i8) as f32;
_2 = _5;
_2 = _8;
_25 = _18;
_4 = _2;
_2 = _7;
RET = -_10;
_13 = Adt26::Variant1 { fld0: 10155_u16 };
_27 = (-10664_i16) as isize;
_27 = !_11;
_19 = _6 & _6;
Goto(bb6)
}
bb20 = {
_8 = _7;
_17 = [(-31654_i16),10144_i16];
_19 = _6 < _6;
Goto(bb7)
}
bb21 = {
_9 = _6 <= _6;
_15 = _1 & _1;
_5 = _2;
RET = -_10;
_4 = _2;
place!(Field::<u16>(Variant(_13, 1), 0)) = 47609_u16 * 20346_u16;
_5 = _2;
_6 = _9;
_7 = _8;
_13 = Adt26::Variant2 { fld0: 1471213956_i32,fld1: 18357_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-1434931483_i32) >> _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = !42326_u16;
_8 = _5;
_17 = [(-21696_i16),(-17087_i16)];
_4 = _7;
_4 = _18;
_17 = [15266_i16,(-13924_i16)];
RET = _1 as f32;
place!(Field::<u16>(Variant(_13, 2), 1)) = 44988_u16 * 60321_u16;
_17 = [(-21427_i16),5801_i16];
_11 = _1;
_1 = !_15;
place!(Field::<u16>(Variant(_13, 2), 1)) = 43460_u16 - 49401_u16;
RET = _10;
_19 = Field::<i32>(Variant(_13, 2), 0) == Field::<i32>(Variant(_13, 2), 0);
_1 = _11;
_11 = _15;
Goto(bb4)
}
bb22 = {
_5 = _12;
_24 = _31 ^ _31;
(*_20) = 17575888061987254898_usize as u8;
_26 = !_15;
_18 = _2;
_5 = _7;
(*_20) = 249_u8 - 146_u8;
_37.0 = core::ptr::addr_of_mut!(_3);
_26 = _1 - _15;
place!(Field::<i32>(Variant(_13, 2), 0)) = (-758027195_i32);
(*_20) = 238_u8;
_29 = _30 + _30;
_27 = _23;
_33.2 = 103_u8;
_39 = [_36,_36,_36,_36,_36,_36,_36];
(*_20) = 157_u8 | 225_u8;
_33 = (3359946112526847517_u64, _2, 118_u8, (-6411_i16));
_31 = _24;
_14 = !_26;
_41 = _33.1;
_7 = _4;
_39 = [_36,_36,_36,_36,_36,_36,_36];
_44.0.1.1 = Field::<u16>(Variant(_13, 2), 1) & Field::<u16>(Variant(_13, 2), 1);
_42 = _10 + RET;
(*_20) = 17995119908582334226_usize as u8;
match _33.0 {
3359946112526847517 => bb23,
_ => bb11
}
}
bb23 = {
_35 = _29;
_36 = 125732944069729372896710642595842766737_u128 * 77349961586789829871863856092700911844_u128;
(*_20) = 125_u8 ^ 73_u8;
(*_20) = !75_u8;
_30 = _1 as f64;
match _33.0 {
0 => bb19,
1 => bb13,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
3359946112526847517 => bb24,
_ => bb20
}
}
bb24 = {
_48.fld0 = (_33.0, _33.1, (*_20), _33.3);
_48.fld0.1 = _7;
RET = _10 * _42;
(*_20) = _48.fld0.2 - _48.fld0.2;
_48.fld0.1 = _12;
_45 = -_33.3;
_44.0.1.2 = Adt29::Variant1 { fld0: Field::<u16>(Variant(_13, 2), 1),fld1: _33,fld2: _28,fld3: Move(_37.0),fld4: RET,fld5: _36 };
_37.1 = 6095466673721771632_i64 as isize;
_44.0.1.1 = Field::<u16>(Variant(_13, 2), 1) ^ Field::<u16>(Variant(_13, 2), 1);
_48.fld1 = Field::<u16>(Variant(_13, 2), 1) >> _15;
(*_20) = !Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
_6 = _31;
_7 = _48.fld0.1;
(*_20) = Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2 & Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
_26 = !_28;
match _33.0 {
3359946112526847517 => bb26,
_ => bb25
}
}
bb25 = {
_12 = _8;
_24 = !_19;
_10 = 198_u8 as f32;
_26 = _1 | _15;
_22 = !_3;
_22 = _3;
_4 = _7;
_6 = _26 < _1;
_4 = _25;
_7 = _4;
RET = (-46_i8) as f32;
_2 = _5;
_2 = _8;
_25 = _18;
_4 = _2;
_2 = _7;
RET = -_10;
_13 = Adt26::Variant1 { fld0: 10155_u16 };
_27 = (-10664_i16) as isize;
_27 = !_11;
_19 = _6 & _6;
Goto(bb6)
}
bb26 = {
_33.0 = _48.fld0.0 * Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).0;
_48.fld1 = Field::<u16>(Variant(_13, 2), 1) << _28;
_52 = _36;
(*_20) = _32 as u8;
_38 = &_52;
_48.fld1 = _44.0.1.1;
_48.fld0 = Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1);
_24 = _6 | _31;
_50 = Field::<i32>(Variant(_13, 2), 0) as f32;
_44.0.1.0 = _30 as i8;
_14 = -_1;
_6 = _24 | _24;
Goto(bb27)
}
bb27 = {
_26 = _27 ^ _28;
Goto(bb28)
}
bb28 = {
_37.0 = core::ptr::addr_of_mut!(_22);
_37.1 = -_1;
_37.1 = _28;
_47 = [_22];
_30 = -_29;
_51 = _24;
(*_20) = Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
(*_20) = _48.fld0.2 ^ Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
_47 = [_22];
place!(Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1)) = (_48.fld0.0, _5, (*_20), _45);
_48.fld0.3 = -_33.3;
_48.fld1 = Field::<u16>(Variant(_44.0.1.2, 1), 0) * Field::<u16>(Variant(_13, 2), 1);
_46 = &mut _10;
_48 = Adt25 { fld0: _33,fld1: _44.0.1.1 };
_54 = _29 * _30;
_55 = _35 + _54;
match Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).0 {
0 => bb16,
1 => bb15,
2 => bb13,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
3359946112526847517 => bb34,
_ => bb33
}
}
bb29 = {
_8 = _7;
_17 = [(-31654_i16),10144_i16];
_19 = _6 < _6;
Goto(bb7)
}
bb30 = {
_33.0 = _48.fld0.0 * Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).0;
_48.fld1 = Field::<u16>(Variant(_13, 2), 1) << _28;
_52 = _36;
(*_20) = _32 as u8;
_38 = &_52;
_48.fld1 = _44.0.1.1;
_48.fld0 = Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1);
_24 = _6 | _31;
_50 = Field::<i32>(Variant(_13, 2), 0) as f32;
_44.0.1.0 = _30 as i8;
_14 = -_1;
_6 = _24 | _24;
Goto(bb27)
}
bb31 = {
_7 = _12;
_13 = Adt26::Variant2 { fld0: (-1622130750_i32),fld1: 53160_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-2040654502_i32) ^ 1591315410_i32;
_14 = _11;
_6 = _4 < _12;
_18 = _4;
_9 = _6 | _6;
place!(Field::<i32>(Variant(_13, 2), 0)) = 416997307_i32;
_14 = _15 * _11;
_1 = _14 << _15;
_12 = _7;
_4 = _18;
place!(Field::<i32>(Variant(_13, 2), 0)) = !1290830803_i32;
_1 = _11 | _14;
_8 = _2;
place!(Field::<u16>(Variant(_13, 2), 1)) = 3859_u16 & 20012_u16;
_7 = _4;
_11 = Field::<u16>(Variant(_13, 2), 1) as isize;
_17 = [(-22627_i16),27943_i16];
_1 = _15 >> _14;
_13 = Adt26::Variant1 { fld0: 41247_u16 };
Goto(bb3)
}
bb32 = {
_3 = (-125165898102373079162278541302314982849_i128) | (-109793664217430576625542900271680307296_i128);
_7 = _2;
_18 = _5;
_1 = _11 << Field::<i32>(Variant(_13, 2), 0);
_19 = _9;
_8 = _7;
_1 = -_15;
RET = _10 * _10;
_5 = _2;
_1 = _15 ^ _11;
_5 = _7;
Call(_25 = fn13(Move(_13), _7, _18, _5, _7, _15, _6, _15, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb33 = {
_31 = _6 | _24;
_27 = (-7383107995608547932_i64) as isize;
_32 = _7;
_28 = _23;
_1 = _23;
_37.1 = _26 + _28;
_25 = _2;
_37.1 = !_26;
_13 = Adt26::Variant2 { fld0: 1507175870_i32,fld1: 52070_u16 };
_31 = _24;
_36 = 63119000960803199805342663336249443978_u128;
_20 = core::ptr::addr_of_mut!(_33.2);
_30 = _29 * _29;
(*_20) = !255_u8;
_6 = _24;
(*_20) = !223_u8;
_2 = _8;
_3 = !_22;
Goto(bb10)
}
bb34 = {
_47 = [_22];
(*_20) = _48.fld0.2 >> _14;
_33.2 = _48.fld0.2;
_43 = &mut _44.0.1.0;
_13 = Adt26::Variant1 { fld0: _48.fld1 };
(*_43) = (-69_i8) * (-40_i8);
(*_46) = _42;
(*_20) = _48.fld1 as u8;
(*_43) = !(-99_i8);
_7 = _5;
(*_20) = _48.fld0.2;
_40 = _25;
(*_46) = RET + RET;
match _48.fld0.3 {
0 => bb1,
1 => bb18,
2 => bb25,
340282366920938463463374607431768205045 => bb36,
_ => bb35
}
}
bb35 = {
_9 = _6 <= _6;
_15 = _1 & _1;
_5 = _2;
RET = -_10;
_4 = _2;
place!(Field::<u16>(Variant(_13, 1), 0)) = 47609_u16 * 20346_u16;
_5 = _2;
_6 = _9;
_7 = _8;
_13 = Adt26::Variant2 { fld0: 1471213956_i32,fld1: 18357_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-1434931483_i32) >> _1;
place!(Field::<u16>(Variant(_13, 2), 1)) = !42326_u16;
_8 = _5;
_17 = [(-21696_i16),(-17087_i16)];
_4 = _7;
_4 = _18;
_17 = [15266_i16,(-13924_i16)];
RET = _1 as f32;
place!(Field::<u16>(Variant(_13, 2), 1)) = 44988_u16 * 60321_u16;
_17 = [(-21427_i16),5801_i16];
_11 = _1;
_1 = !_15;
place!(Field::<u16>(Variant(_13, 2), 1)) = 43460_u16 - 49401_u16;
RET = _10;
_19 = Field::<i32>(Variant(_13, 2), 0) == Field::<i32>(Variant(_13, 2), 0);
_1 = _11;
_11 = _15;
Goto(bb4)
}
bb36 = {
_41 = _40;
(*_43) = (-120_i8) | (-109_i8);
_7 = _4;
_20 = core::ptr::addr_of_mut!((*_20));
_19 = (*_43) == (*_43);
(*_20) = _48.fld0.2;
_11 = _7 as isize;
_59.0.3 = _48.fld0.0 ^ _33.0;
_52 = _36;
_53 = _37.1;
(*_43) = 1039021403_i32 as i8;
_59.0.3 = _33.0;
(*_46) = _42 + RET;
_48.fld1 = _3 as u16;
_3 = _22;
match _33.3 {
0 => bb31,
1 => bb6,
2 => bb37,
3 => bb38,
340282366920938463463374607431768205045 => bb40,
_ => bb39
}
}
bb37 = {
_7 = _12;
_13 = Adt26::Variant2 { fld0: (-1622130750_i32),fld1: 53160_u16 };
place!(Field::<i32>(Variant(_13, 2), 0)) = (-2040654502_i32) ^ 1591315410_i32;
_14 = _11;
_6 = _4 < _12;
_18 = _4;
_9 = _6 | _6;
place!(Field::<i32>(Variant(_13, 2), 0)) = 416997307_i32;
_14 = _15 * _11;
_1 = _14 << _15;
_12 = _7;
_4 = _18;
place!(Field::<i32>(Variant(_13, 2), 0)) = !1290830803_i32;
_1 = _11 | _14;
_8 = _2;
place!(Field::<u16>(Variant(_13, 2), 1)) = 3859_u16 & 20012_u16;
_7 = _4;
_11 = Field::<u16>(Variant(_13, 2), 1) as isize;
_17 = [(-22627_i16),27943_i16];
_1 = _15 >> _14;
_13 = Adt26::Variant1 { fld0: 41247_u16 };
Goto(bb3)
}
bb38 = {
_37.0 = core::ptr::addr_of_mut!(_22);
_37.1 = -_1;
_37.1 = _28;
_47 = [_22];
_30 = -_29;
_51 = _24;
(*_20) = Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
(*_20) = _48.fld0.2 ^ Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).2;
_47 = [_22];
place!(Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1)) = (_48.fld0.0, _5, (*_20), _45);
_48.fld0.3 = -_33.3;
_48.fld1 = Field::<u16>(Variant(_44.0.1.2, 1), 0) * Field::<u16>(Variant(_13, 2), 1);
_46 = &mut _10;
_48 = Adt25 { fld0: _33,fld1: _44.0.1.1 };
_54 = _29 * _30;
_55 = _35 + _54;
match Field::<(u64, char, u8, i16)>(Variant(_44.0.1.2, 1), 1).0 {
0 => bb16,
1 => bb15,
2 => bb13,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
3359946112526847517 => bb34,
_ => bb33
}
}
bb39 = {
_8 = _7;
_17 = [(-31654_i16),10144_i16];
_19 = _6 < _6;
Goto(bb7)
}
bb40 = {
(*_20) = _48.fld0.2 - _48.fld0.2;
_54 = (*_20) as f64;
_59.0.2 = -_3;
_59.0.1.1 = Field::<u16>(Variant(_13, 1), 0) - Field::<u16>(Variant(_13, 1), 0);
_59.0.3 = _48.fld0.0;
(*_20) = 10560944944738142658_usize as u8;
(*_43) = (-11_i8) - 33_i8;
_52 = _36 + _36;
_38 = &_36;
(*_46) = _50;
(*_46) = _42 - _50;
_61 = -_48.fld0.3;
_1 = _15;
_47 = [_3];
(*_20) = _48.fld0.2 ^ _48.fld0.2;
_60 = _55 as i8;
_3 = _22;
(*_20) = _48.fld0.2 | _48.fld0.2;
_15 = 1752616141_u32 as isize;
_37.0 = core::ptr::addr_of_mut!(_3);
(*_46) = RET;
Goto(bb41)
}
bb41 = {
_33.0 = _59.0.3;
_4 = _8;
_59.0.1.2 = Adt29::Variant1 { fld0: _59.0.1.1,fld1: _48.fld0,fld2: _14,fld3: Move(_37.0),fld4: (*_46),fld5: (*_38) };
(*_46) = _42 - RET;
_51 = _26 > _27;
_63.1.1 = !_59.0.1.1;
_59.0.3 = _48.fld0.0 >> _60;
_59.0.2 = !_3;
_68 = (*_20) | (*_20);
(*_46) = _63.1.1 as f32;
_17 = [_45,Field::<(u64, char, u8, i16)>(Variant(_59.0.1.2, 1), 1).3];
_24 = _51;
_63.1.0 = (*_43);
(*_43) = _60;
_48.fld0 = Field::<(u64, char, u8, i16)>(Variant(_59.0.1.2, 1), 1);
_26 = Field::<isize>(Variant(_59.0.1.2, 1), 2);
_58 = &mut (*_43);
(*_20) = _14 as u8;
_48.fld0.2 = !(*_20);
(*_58) = !_63.1.0;
(*_46) = _53 as f32;
Goto(bb42)
}
bb42 = {
match _48.fld0.3 {
0 => bb4,
340282366920938463463374607431768205045 => bb43,
_ => bb20
}
}
bb43 = {
_43 = &mut _60;
_59.0.0 = _31;
(*_58) = (*_43) - (*_43);
Goto(bb44)
}
bb44 = {
_8 = _32;
_37 = (Move(Field::<*mut i128>(Variant(_59.0.1.2, 1), 3)), _27);
_63.1.0 = (*_58) >> _27;
place!(Field::<isize>(Variant(_59.0.1.2, 1), 2)) = _14 - _23;
_2 = _40;
_68 = !(*_20);
(*_43) = 9380526107369745977_usize as i8;
(*_20) = _48.fld0.2;
_47 = [_3];
_63.2.3 = (*_58) as i16;
_37.0 = core::ptr::addr_of_mut!(_59.0.2);
_69 = Move(_58);
_59.0.3 = _48.fld0.0 ^ _33.0;
match Field::<(u64, char, u8, i16)>(Variant(_59.0.1.2, 1), 1).3 {
0 => bb28,
1 => bb2,
2 => bb35,
340282366920938463463374607431768205045 => bb45,
_ => bb26
}
}
bb45 = {
_30 = _55 + _55;
(*_46) = 8792525517923793074_i64 as f32;
match _33.3 {
0 => bb46,
340282366920938463463374607431768205045 => bb48,
_ => bb47
}
}
bb46 = {
match _36 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
63119000960803199805342663336249443978 => bb16,
_ => bb15
}
}
bb47 = {
_12 = _8;
_24 = !_19;
_10 = 198_u8 as f32;
_26 = _1 | _15;
_22 = !_3;
_22 = _3;
_4 = _7;
_6 = _26 < _1;
_4 = _25;
_7 = _4;
RET = (-46_i8) as f32;
_2 = _5;
_2 = _8;
_25 = _18;
_4 = _2;
_2 = _7;
RET = -_10;
_13 = Adt26::Variant1 { fld0: 10155_u16 };
_27 = (-10664_i16) as isize;
_27 = !_11;
_19 = _6 & _6;
Goto(bb6)
}
bb48 = {
_2 = _18;
(*_20) = _48.fld0.2 - _68;
_1 = _26 | Field::<isize>(Variant(_59.0.1.2, 1), 2);
place!(Field::<f32>(Variant(_59.0.1.2, 1), 4)) = (*_46) * _42;
_26 = !Field::<isize>(Variant(_59.0.1.2, 1), 2);
(*_43) = _63.1.0 & _63.1.0;
_33.1 = _18;
_65 = Move(_38);
(*_20) = _48.fld0.2;
_63.2.3 = (*_20) as i16;
_75 = core::ptr::addr_of!(_63.3);
_30 = _63.2.3 as f64;
(*_75) = (3246750764_u32,);
_49 = [(*_43),(*_43),(*_43),(*_43),(*_43),(*_43),(*_43),(*_43)];
place!(Field::<f32>(Variant(_59.0.1.2, 1), 4)) = RET + (*_46);
(*_75) = (442767239_u32,);
(*_43) = -_63.1.0;
(*_75).0 = 3205706623_u32 ^ 2276900631_u32;
(*_75) = (2425266382_u32,);
_13 = Adt26::Variant2 { fld0: 2117363629_i32,fld1: _63.1.1 };
_74 = (-1222857756_i32) & (-1895061350_i32);
(*_46) = _42 - Field::<f32>(Variant(_59.0.1.2, 1), 4);
(*_46) = _42 - Field::<f32>(Variant(_59.0.1.2, 1), 4);
Goto(bb49)
}
bb49 = {
_59.0.2 = -_22;
(*_75).0 = 573166931_u32 << (*_43);
(*_46) = _36 as f32;
(*_75) = (809229925_u32,);
(*_75).0 = 2967640115_u32 - 1562186314_u32;
_63.3.0 = 1702104612_u32;
_2 = Field::<(u64, char, u8, i16)>(Variant(_59.0.1.2, 1), 1).1;
(*_20) = _68 - _48.fld0.2;
place!(Field::<u16>(Variant(_59.0.1.2, 1), 0)) = _63.1.1 | _63.1.1;
place!(Field::<f32>(Variant(_59.0.1.2, 1), 4)) = (*_46) * (*_46);
(*_43) = _63.2.3 as i8;
(*_20) = _59.0.3 as u8;
_34 = core::ptr::addr_of_mut!((*_75).0);
(*_46) = -_42;
(*_75) = (1147450582_u32,);
_63.2.0 = !_59.0.3;
(*_20) = 6234891173071688545_i64 as u8;
(*_75).0 = !2592527926_u32;
_43 = &mut _63.1.0;
(*_20) = _48.fld0.2 - _68;
(*_46) = Field::<f32>(Variant(_59.0.1.2, 1), 4);
(*_46) = -Field::<f32>(Variant(_59.0.1.2, 1), 4);
Goto(bb50)
}
bb50 = {
Call(_84 = dump_var(Move(_45), Move(_17), Move(_7), Move(_33)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_84 = dump_var(Move(_74), Move(_60), Move(_26), Move(_3)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_84 = dump_var(Move(_5), Move(_27), Move(_22), Move(_49)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_84 = dump_var(Move(_25), Move(_52), Move(_41), Move(_11)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_84 = dump_var(Move(_18), Move(_39), Move(_4), Move(_68)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: Adt26,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: isize,mut _7: bool,mut _8: isize,mut _9: char) -> char {
mir! {
type RET = char;
let _10: f64;
let _11: *mut f64;
let _12: (i8, u16, Adt29);
let _13: i32;
let _14: u64;
let _15: &'static *mut i128;
let _16: &'static mut i16;
let _17: f32;
let _18: i32;
let _19: (bool, (i8, u16, Adt29), i128, u64);
let _20: &'static mut u32;
let _21: f32;
let _22: u16;
let _23: u8;
let _24: bool;
let _25: [char; 2];
let _26: &'static mut i8;
let _27: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _28: f32;
let _29: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _30: [u64; 1];
let _31: usize;
let _32: i64;
let _33: *const i8;
let _34: i32;
let _35: isize;
let _36: [i8; 1];
let _37: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _38: ([bool; 1],);
let _39: i16;
let _40: &'static Adt18;
let _41: bool;
let _42: f32;
let _43: (char, Adt18, *mut u64);
let _44: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _45: bool;
let _46: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _47: (u32,);
let _48: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _49: usize;
let _50: isize;
let _51: (u64, char, u8, i16);
let _52: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _53: u16;
let _54: &'static mut i16;
let _55: i128;
let _56: [i16; 2];
let _57: Adt18;
let _58: &'static mut u32;
let _59: char;
let _60: isize;
let _61: i16;
let _62: &'static i128;
let _63: &'static mut i16;
let _64: *mut u8;
let _65: *mut u32;
let _66: (f32, i128);
let _67: bool;
let _68: u128;
let _69: *const &'static mut u128;
let _70: &'static mut u128;
let _71: f64;
let _72: i16;
let _73: i16;
let _74: &'static mut &'static &'static Adt18;
let _75: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _76: *const *const &'static mut u128;
let _77: char;
let _78: char;
let _79: isize;
let _80: &'static mut [bool; 2];
let _81: Adt26;
let _82: char;
let _83: [u128; 7];
let _84: [i128; 1];
let _85: [i8; 7];
let _86: *mut usize;
let _87: bool;
let _88: &'static Adt18;
let _89: &'static Adt18;
let _90: u128;
let _91: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _92: [bool; 2];
let _93: isize;
let _94: f32;
let _95: char;
let _96: u8;
let _97: char;
let _98: usize;
let _99: bool;
let _100: u64;
let _101: u32;
let _102: u8;
let _103: u16;
let _104: ();
let _105: ();
{
_8 = _6 + _6;
_2 = _4;
place!(Field::<i32>(Variant(_1, 2), 0)) = _8 as i32;
_10 = _8 as f64;
Goto(bb1)
}
bb1 = {
RET = _2;
_9 = _3;
_8 = _6;
_11 = core::ptr::addr_of_mut!(_10);
(*_11) = (-65586944922409600480861468137689021561_i128) as f64;
_5 = _2;
(*_11) = 5610337865206814415_i64 as f64;
RET = _3;
_5 = RET;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-1951637512_i32) * (-931085737_i32);
(*_11) = (-7605971931701405798_i64) as f64;
(*_11) = 133961775956611631448191628398770802078_i128 as f64;
(*_11) = 163_u8 as f64;
(*_11) = _6 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = 3031098852151244943_u64 as u16;
_9 = RET;
place!(Field::<i32>(Variant(_1, 2), 0)) = 943486917_i32;
(*_11) = (-87245683993016916433614367589056522906_i128) as f64;
(*_11) = (-7737877252995080452_i64) as f64;
(*_11) = 8865692521799526267_i64 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = 17138336099242389835_usize as u16;
_8 = _6 - _6;
(*_11) = 229_u8 as f64;
(*_11) = _8 as f64;
_5 = _4;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb2,
1 => bb3,
2 => bb4,
943486917 => bb6,
_ => bb5
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
_12.0 = 115_i8 & (-77_i8);
(*_11) = 783893147_u32 as f64;
_3 = _2;
_8 = (*_11) as isize;
(*_11) = 1_usize as f64;
(*_11) = 79449490505387877110217928305898018586_i128 as f64;
(*_11) = (-119462164669677050906018098783280298259_i128) as f64;
_3 = RET;
_11 = core::ptr::addr_of_mut!((*_11));
_19.1.1 = 2159391074_u32 as u16;
_12.0 = -55_i8;
_17 = (-151540275542651322290929809861269415970_i128) as f32;
(*_11) = _6 as f64;
Goto(bb7)
}
bb7 = {
RET = _2;
(*_11) = _12.0 as f64;
_14 = 5527659647503213500_u64;
_19.1.0 = !_12.0;
(*_11) = 840005648_u32 as f64;
_4 = _3;
(*_11) = _19.1.1 as f64;
_19.1.0 = _12.0 | _12.0;
(*_11) = 110296237454331305931557785165078874295_u128 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = !_19.1.1;
_9 = _4;
(*_11) = Field::<i32>(Variant(_1, 2), 0) as f64;
_4 = _2;
_19.0 = _7;
_11 = core::ptr::addr_of_mut!((*_11));
_13 = Field::<i32>(Variant(_1, 2), 0) & Field::<i32>(Variant(_1, 2), 0);
_19.0 = _7;
(*_11) = _17 as f64;
(*_11) = (-7935870065712118315_i64) as f64;
_12.0 = _19.1.0;
_8 = _6 * _6;
_5 = _9;
_11 = core::ptr::addr_of_mut!((*_11));
(*_11) = 132789871630773530924807054727170727788_i128 as f64;
_22 = !_19.1.1;
_12.1 = Field::<u16>(Variant(_1, 2), 1);
_19.0 = _7 & _7;
(*_11) = _13 as f64;
Goto(bb8)
}
bb8 = {
_23 = 2941736365_u32 as u8;
(*_11) = 3939773556_u32 as f64;
_19.3 = _14 + _14;
(*_11) = 3_usize as f64;
(*_11) = _13 as f64;
_2 = _3;
(*_11) = 38000904930063395821720764568550293395_i128 as f64;
(*_11) = 235269790782365734832595427174573367551_u128 as f64;
_2 = _3;
(*_11) = (-3096163598787440104_i64) as f64;
Call(_19.2 = fn14(Move(_1), _19.3, (*_11), _12.1, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = !_8;
(*_11) = _19.3 as f64;
(*_11) = 4708479288436687623_i64 as f64;
_21 = _19.3 as f32;
_21 = _8 as f32;
_7 = _19.0;
_22 = 252597502707979715002806616146494171300_u128 as u16;
(*_11) = _19.1.0 as f64;
(*_11) = 254153269_u32 as f64;
_18 = _13 & _13;
(*_11) = 1634625175_u32 as f64;
_12.0 = _19.1.0 & _19.1.0;
(*_11) = (-18192_i16) as f64;
_3 = RET;
_12.0 = _19.1.0 << _6;
_7 = _12.0 > _12.0;
_3 = _2;
_6 = _8 * _8;
(*_11) = _12.0 as f64;
_14 = !_19.3;
_6 = !_8;
_12.1 = !_19.1.1;
_7 = !_19.0;
_17 = _21 * _21;
_19.1.0 = _12.0 << _6;
(*_11) = 4_usize as f64;
_4 = _9;
_2 = _9;
_25 = [_5,RET];
Goto(bb10)
}
bb10 = {
_18 = _13 * _13;
(*_11) = 7_usize as f64;
(*_11) = _22 as f64;
_3 = RET;
(*_11) = 901057392937576412_i64 as f64;
_19.3 = !_14;
_12.1 = !_22;
_1 = Adt26::Variant2 { fld0: _13,fld1: _22 };
_12.1 = Field::<u16>(Variant(_1, 2), 1) << _19.1.0;
_18 = !_13;
Goto(bb11)
}
bb11 = {
_22 = _12.1 << _19.1.0;
(*_11) = _8 as f64;
(*_11) = 177915868613188530481361400278486543189_u128 as f64;
Goto(bb12)
}
bb12 = {
_13 = 16038_i16 as i32;
_12.1 = _22 * _19.1.1;
_3 = _2;
Goto(bb13)
}
bb13 = {
_19.1.1 = !_12.1;
_7 = _19.0;
(*_11) = 5250536256377552855_usize as f64;
_21 = _17 - _17;
_22 = _19.1.1 & _19.1.1;
_29.3.0.2 = 288977619081951986169668804835414330895_u128 as i128;
_10 = _29.3.0.2 as f64;
(*_11) = _6 as f64;
_1 = Adt26::Variant2 { fld0: _18,fld1: _22 };
_18 = _19.0 as i32;
_13 = -_18;
Call(place!(Field::<u16>(Variant(_1, 2), 1)) = core::intrinsics::bswap(_22), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_8 = _6 | _6;
(*_11) = _13 as f64;
_30 = [_14];
_29.3.0.0 = Field::<u16>(Variant(_1, 2), 1) == Field::<u16>(Variant(_1, 2), 1);
(*_11) = Field::<i32>(Variant(_1, 2), 0) as f64;
_9 = _5;
_31 = 12255940703573970639_usize >> _12.0;
_29.3.0.3 = _19.3 - _14;
_32 = 8949781994876975038_i64 + (-7431998830917037570_i64);
_9 = RET;
_29.3.0.2 = _19.2 & _19.2;
_29.3.0.3 = _14 - _19.3;
_25 = [_9,_9];
_22 = _12.1 + _12.1;
_9 = _4;
(*_11) = _19.1.0 as f64;
(*_11) = _21 as f64;
_19.3 = _29.3.0.3 & _29.3.0.3;
_28 = _6 as f32;
_24 = !_7;
_19.2 = _29.3.0.2 ^ _29.3.0.2;
(*_11) = _23 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = !_19.1.1;
_29.3.0.0 = _24;
_26 = &mut _19.1.0;
(*_26) = _12.0 + _12.0;
(*_11) = _29.3.0.2 as f64;
Goto(bb15)
}
bb15 = {
_33 = core::ptr::addr_of!((*_26));
(*_33) = _29.3.0.2 as i8;
_29.3.0.1.1 = Field::<u16>(Variant(_1, 2), 1);
(*_33) = _12.0 * _12.0;
(*_11) = _29.3.0.2 as f64;
(*_26) = _12.0 >> _12.0;
_8 = _6;
_31 = !14506734202078533296_usize;
_13 = _18;
(*_26) = _12.0 << Field::<u16>(Variant(_1, 2), 1);
_25 = [_3,_3];
(*_26) = _12.0 - _12.0;
_4 = _3;
(*_11) = _17 as f64;
_30 = [_29.3.0.3];
(*_11) = _29.3.0.3 as f64;
_22 = _29.3.0.1.1 - _29.3.0.1.1;
(*_26) = _12.0 * _12.0;
Goto(bb16)
}
bb16 = {
(*_26) = _32 as i8;
_29.0 = core::ptr::addr_of_mut!(_29.3.0.3);
_38.0 = [_29.3.0.0];
_14 = _29.3.0.3 << _12.0;
_13 = !Field::<i32>(Variant(_1, 2), 0);
(*_11) = _29.3.0.2 as f64;
(*_11) = _12.0 as f64;
_33 = core::ptr::addr_of!((*_26));
(*_11) = _21 as f64;
_10 = _32 as f64;
_7 = _24;
(*_11) = _32 as f64;
(*_26) = 28759_i16 as i8;
_36 = [_12.0];
(*_11) = 24781_i16 as f64;
_21 = _17;
_5 = _3;
_37.1.1.1 = !_29.3.0.1.1;
Goto(bb17)
}
bb17 = {
(*_11) = 17266_i16 as f64;
_38.0 = [_7];
place!(Field::<u16>(Variant(_1, 2), 1)) = !_12.1;
_37.1.0 = _14 < _14;
(*_11) = _8 as f64;
_11 = core::ptr::addr_of_mut!((*_11));
(*_11) = _23 as f64;
_44.0 = core::ptr::addr_of_mut!(_37.1.3);
_39 = 31541_i16 * 8208_i16;
_43.2 = core::ptr::addr_of_mut!(_44.3.0.3);
(*_11) = _32 as f64;
_37.0 = !_14;
(*_11) = _23 as f64;
Goto(bb18)
}
bb18 = {
(*_26) = _8 as i8;
_44.1 = _29.3.0.2 as i32;
(*_11) = _44.1 as f64;
(*_11) = _37.0 as f64;
(*_11) = 13072338496315225915019270724560197221_u128 as f64;
_29.3.0.0 = _22 != _12.1;
_23 = 229_u8 - 74_u8;
_43.1.fld1 = !_23;
_34 = -_44.1;
_29.3.0.2 = 22499787444526391081618489679946366197_i128 >> (*_26);
RET = _9;
(*_26) = _12.0;
(*_11) = _43.1.fld1 as f64;
_28 = _21 + _17;
_45 = _29.3.0.0;
(*_11) = _39 as f64;
_43.1.fld0 = !_45;
_35 = _8;
_7 = _37.1.0;
_7 = (*_26) != (*_26);
_33 = core::ptr::addr_of!((*_26));
_37.1.1.0 = (*_33);
(*_33) = _37.1.1.0;
Goto(bb19)
}
bb19 = {
_29.3.0.3 = _37.0 & _14;
place!(Field::<i32>(Variant(_1, 2), 0)) = _18 | _18;
_37.1.0 = _45;
(*_11) = _37.1.1.1 as f64;
(*_11) = _21 as f64;
_3 = _4;
_29.0 = core::ptr::addr_of_mut!(_37.1.3);
(*_26) = _29.3.0.2 as i8;
_29.0 = core::ptr::addr_of_mut!(_37.0);
_29.3.0.3 = _37.0;
(*_26) = _37.1.1.0 + _12.0;
(*_26) = _37.1.1.0 * _37.1.1.0;
(*_11) = _21 as f64;
_12.1 = _29.3.0.1.1 & _29.3.0.1.1;
place!(Field::<u16>(Variant(_1, 2), 1)) = !_22;
_16 = &mut _39;
_44.3.0.3 = _14;
_43.0 = _3;
_29.1 = _18 ^ _18;
_37.1.3 = _29.3.0.3 * _29.3.0.3;
_45 = (*_26) == (*_26);
(*_16) = Field::<i32>(Variant(_1, 2), 0) as i16;
(*_11) = _29.3.0.3 as f64;
_29.3.0.1.0 = (*_26) * (*_26);
_44.3.0.0 = _29.3.0.0;
_44.3.0.1.0 = -(*_26);
_2 = _4;
Call((*_26) = core::intrinsics::transmute(_44.3.0.1.0), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_3 = _43.0;
_8 = !_35;
(*_16) = (-8859_i16);
_43.2 = core::ptr::addr_of_mut!(_37.0);
(*_16) = 20079_i16 - 25672_i16;
(*_11) = _31 as f64;
(*_26) = _29.3.0.1.0 >> (*_16);
_29.3.0.1.0 = (*_26);
(*_11) = _23 as f64;
(*_11) = _14 as f64;
_43.0 = _4;
_47 = (2987131962_u32,);
(*_11) = 314654941049989794709771790698698591305_u128 as f64;
_44.3.0.3 = _37.1.3;
_35 = _6;
(*_16) = 1077_i16 + 6698_i16;
_8 = _35 + _35;
_44.3.0.1.1 = _47.0 as u16;
_49 = (*_11) as usize;
(*_16) = Field::<i32>(Variant(_1, 2), 0) as i16;
(*_16) = _12.1 as i16;
(*_11) = _28 as f64;
_43.1.fld5 = (*_26) as usize;
_17 = _28 + _21;
_44.1 = _23 as i32;
_51 = (_37.0, _2, _23, (*_16));
Goto(bb21)
}
bb21 = {
_37.0 = _37.1.3;
(*_11) = (*_16) as f64;
_49 = _43.1.fld5 >> (*_16);
_37.1.3 = _29.3.0.3;
(*_11) = 116338553189265924048203574617681063700_u128 as f64;
_42 = _17;
_32 = (-7100046345923602537_i64) << _37.1.3;
(*_26) = -_37.1.1.0;
_22 = !_29.3.0.1.1;
_44.3.0.2 = _29.3.0.2 & _29.3.0.2;
(*_26) = _37.1.1.0 >> _37.1.3;
_37.2 = _23 + _43.1.fld1;
_51.3 = _43.1.fld5 as i16;
_17 = -_28;
_2 = _9;
_54 = Move(_16);
_50 = _6 | _35;
_44.2 = _38.0;
_42 = -_21;
_44.1 = _43.1.fld5 as i32;
(*_26) = _44.3.0.1.0;
(*_11) = _37.1.1.1 as f64;
_26 = &mut _29.3.0.1.0;
(*_26) = !_37.1.1.0;
Goto(bb22)
}
bb22 = {
_55 = _44.3.0.2 & _44.3.0.2;
_37.0 = !_51.0;
RET = _4;
_44.1 = _18;
_37.1.0 = (*_26) >= (*_26);
_43.1.fld4 = _51.3;
_22 = !Field::<u16>(Variant(_1, 2), 1);
_44.1 = Field::<i32>(Variant(_1, 2), 0);
(*_26) = _44.3.0.1.0;
(*_26) = _37.1.1.0 + _37.1.1.0;
_57.fld5 = !_43.1.fld5;
_44.3.0.2 = _47.0 as i128;
(*_26) = _44.3.0.1.0;
_43.1.fld2 = _21 as isize;
_16 = &mut _43.1.fld4;
_37.1.3 = _37.0 ^ _44.3.0.3;
(*_26) = _37.1.1.0 & _44.3.0.1.0;
_38.0 = [_37.1.0];
(*_16) = _51.3;
_16 = &mut _51.3;
(*_11) = Field::<i32>(Variant(_1, 2), 0) as f64;
(*_26) = _8 as i8;
_2 = _3;
match _47.0 {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
2987131962 => bb30,
_ => bb29
}
}
bb23 = {
_37.0 = _37.1.3;
(*_11) = (*_16) as f64;
_49 = _43.1.fld5 >> (*_16);
_37.1.3 = _29.3.0.3;
(*_11) = 116338553189265924048203574617681063700_u128 as f64;
_42 = _17;
_32 = (-7100046345923602537_i64) << _37.1.3;
(*_26) = -_37.1.1.0;
_22 = !_29.3.0.1.1;
_44.3.0.2 = _29.3.0.2 & _29.3.0.2;
(*_26) = _37.1.1.0 >> _37.1.3;
_37.2 = _23 + _43.1.fld1;
_51.3 = _43.1.fld5 as i16;
_17 = -_28;
_2 = _9;
_54 = Move(_16);
_50 = _6 | _35;
_44.2 = _38.0;
_42 = -_21;
_44.1 = _43.1.fld5 as i32;
(*_26) = _44.3.0.1.0;
(*_11) = _37.1.1.1 as f64;
_26 = &mut _29.3.0.1.0;
(*_26) = !_37.1.1.0;
Goto(bb22)
}
bb24 = {
_19.1.1 = !_12.1;
_7 = _19.0;
(*_11) = 5250536256377552855_usize as f64;
_21 = _17 - _17;
_22 = _19.1.1 & _19.1.1;
_29.3.0.2 = 288977619081951986169668804835414330895_u128 as i128;
_10 = _29.3.0.2 as f64;
(*_11) = _6 as f64;
_1 = Adt26::Variant2 { fld0: _18,fld1: _22 };
_18 = _19.0 as i32;
_13 = -_18;
Call(place!(Field::<u16>(Variant(_1, 2), 1)) = core::intrinsics::bswap(_22), ReturnTo(bb14), UnwindUnreachable())
}
bb25 = {
_22 = _12.1 << _19.1.0;
(*_11) = _8 as f64;
(*_11) = 177915868613188530481361400278486543189_u128 as f64;
Goto(bb12)
}
bb26 = {
_13 = 16038_i16 as i32;
_12.1 = _22 * _19.1.1;
_3 = _2;
Goto(bb13)
}
bb27 = {
(*_11) = 17266_i16 as f64;
_38.0 = [_7];
place!(Field::<u16>(Variant(_1, 2), 1)) = !_12.1;
_37.1.0 = _14 < _14;
(*_11) = _8 as f64;
_11 = core::ptr::addr_of_mut!((*_11));
(*_11) = _23 as f64;
_44.0 = core::ptr::addr_of_mut!(_37.1.3);
_39 = 31541_i16 * 8208_i16;
_43.2 = core::ptr::addr_of_mut!(_44.3.0.3);
(*_11) = _32 as f64;
_37.0 = !_14;
(*_11) = _23 as f64;
Goto(bb18)
}
bb28 = {
_23 = 2941736365_u32 as u8;
(*_11) = 3939773556_u32 as f64;
_19.3 = _14 + _14;
(*_11) = 3_usize as f64;
(*_11) = _13 as f64;
_2 = _3;
(*_11) = 38000904930063395821720764568550293395_i128 as f64;
(*_11) = 235269790782365734832595427174573367551_u128 as f64;
_2 = _3;
(*_11) = (-3096163598787440104_i64) as f64;
Call(_19.2 = fn14(Move(_1), _19.3, (*_11), _12.1, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb29 = {
_6 = !_8;
(*_11) = _19.3 as f64;
(*_11) = 4708479288436687623_i64 as f64;
_21 = _19.3 as f32;
_21 = _8 as f32;
_7 = _19.0;
_22 = 252597502707979715002806616146494171300_u128 as u16;
(*_11) = _19.1.0 as f64;
(*_11) = 254153269_u32 as f64;
_18 = _13 & _13;
(*_11) = 1634625175_u32 as f64;
_12.0 = _19.1.0 & _19.1.0;
(*_11) = (-18192_i16) as f64;
_3 = RET;
_12.0 = _19.1.0 << _6;
_7 = _12.0 > _12.0;
_3 = _2;
_6 = _8 * _8;
(*_11) = _12.0 as f64;
_14 = !_19.3;
_6 = !_8;
_12.1 = !_19.1.1;
_7 = !_19.0;
_17 = _21 * _21;
_19.1.0 = _12.0 << _6;
(*_11) = 4_usize as f64;
_4 = _9;
_2 = _9;
_25 = [_5,RET];
Goto(bb10)
}
bb30 = {
_44.0 = core::ptr::addr_of_mut!(_37.0);
(*_26) = -_12.0;
(*_16) = 28357_i16 - (-20122_i16);
(*_26) = -_37.1.1.0;
_63 = &mut (*_16);
_49 = !_57.fld5;
(*_11) = _42 as f64;
(*_11) = _49 as f64;
(*_63) = !(-21514_i16);
_9 = RET;
(*_11) = 339707755505734071641261219262308684755_u128 as f64;
_37.1.1.1 = _22;
_57.fld2 = _6 << (*_26);
_55 = _44.3.0.2 << (*_26);
_54 = &mut (*_63);
(*_26) = -_37.1.1.0;
_57.fld0 = (*_26) != (*_26);
_38.0 = _44.2;
place!(Field::<u16>(Variant(_1, 2), 1)) = !_37.1.1.1;
_53 = Field::<u16>(Variant(_1, 2), 1) * _37.1.1.1;
_47.0 = 452187279_u32 & 1558811494_u32;
(*_54) = !(-5853_i16);
_57.fld2 = _6 | _8;
_5 = _2;
(*_11) = (*_26) as f64;
(*_54) = 11630_i16;
match (*_54) {
0 => bb31,
1 => bb32,
2 => bb33,
11630 => bb35,
_ => bb34
}
}
bb31 = {
_55 = _44.3.0.2 & _44.3.0.2;
_37.0 = !_51.0;
RET = _4;
_44.1 = _18;
_37.1.0 = (*_26) >= (*_26);
_43.1.fld4 = _51.3;
_22 = !Field::<u16>(Variant(_1, 2), 1);
_44.1 = Field::<i32>(Variant(_1, 2), 0);
(*_26) = _44.3.0.1.0;
(*_26) = _37.1.1.0 + _37.1.1.0;
_57.fld5 = !_43.1.fld5;
_44.3.0.2 = _47.0 as i128;
(*_26) = _44.3.0.1.0;
_43.1.fld2 = _21 as isize;
_16 = &mut _43.1.fld4;
_37.1.3 = _37.0 ^ _44.3.0.3;
(*_26) = _37.1.1.0 & _44.3.0.1.0;
_38.0 = [_37.1.0];
(*_16) = _51.3;
_16 = &mut _51.3;
(*_11) = Field::<i32>(Variant(_1, 2), 0) as f64;
(*_26) = _8 as i8;
_2 = _3;
match _47.0 {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
2987131962 => bb30,
_ => bb29
}
}
bb32 = {
RET = _2;
_9 = _3;
_8 = _6;
_11 = core::ptr::addr_of_mut!(_10);
(*_11) = (-65586944922409600480861468137689021561_i128) as f64;
_5 = _2;
(*_11) = 5610337865206814415_i64 as f64;
RET = _3;
_5 = RET;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-1951637512_i32) * (-931085737_i32);
(*_11) = (-7605971931701405798_i64) as f64;
(*_11) = 133961775956611631448191628398770802078_i128 as f64;
(*_11) = 163_u8 as f64;
(*_11) = _6 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = 3031098852151244943_u64 as u16;
_9 = RET;
place!(Field::<i32>(Variant(_1, 2), 0)) = 943486917_i32;
(*_11) = (-87245683993016916433614367589056522906_i128) as f64;
(*_11) = (-7737877252995080452_i64) as f64;
(*_11) = 8865692521799526267_i64 as f64;
place!(Field::<u16>(Variant(_1, 2), 1)) = 17138336099242389835_usize as u16;
_8 = _6 - _6;
(*_11) = 229_u8 as f64;
(*_11) = _8 as f64;
_5 = _4;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb2,
1 => bb3,
2 => bb4,
943486917 => bb6,
_ => bb5
}
}
bb33 = {
_12.0 = 115_i8 & (-77_i8);
(*_11) = 783893147_u32 as f64;
_3 = _2;
_8 = (*_11) as isize;
(*_11) = 1_usize as f64;
(*_11) = 79449490505387877110217928305898018586_i128 as f64;
(*_11) = (-119462164669677050906018098783280298259_i128) as f64;
_3 = RET;
_11 = core::ptr::addr_of_mut!((*_11));
_19.1.1 = 2159391074_u32 as u16;
_12.0 = -55_i8;
_17 = (-151540275542651322290929809861269415970_i128) as f32;
(*_11) = _6 as f64;
Goto(bb7)
}
bb34 = {
_37.0 = _37.1.3;
(*_11) = (*_16) as f64;
_49 = _43.1.fld5 >> (*_16);
_37.1.3 = _29.3.0.3;
(*_11) = 116338553189265924048203574617681063700_u128 as f64;
_42 = _17;
_32 = (-7100046345923602537_i64) << _37.1.3;
(*_26) = -_37.1.1.0;
_22 = !_29.3.0.1.1;
_44.3.0.2 = _29.3.0.2 & _29.3.0.2;
(*_26) = _37.1.1.0 >> _37.1.3;
_37.2 = _23 + _43.1.fld1;
_51.3 = _43.1.fld5 as i16;
_17 = -_28;
_2 = _9;
_54 = Move(_16);
_50 = _6 | _35;
_44.2 = _38.0;
_42 = -_21;
_44.1 = _43.1.fld5 as i32;
(*_26) = _44.3.0.1.0;
(*_11) = _37.1.1.1 as f64;
_26 = &mut _29.3.0.1.0;
(*_26) = !_37.1.1.0;
Goto(bb22)
}
bb35 = {
(*_11) = _44.3.0.3 as f64;
(*_26) = !_37.1.1.0;
(*_54) = _32 as i16;
(*_11) = (*_26) as f64;
_4 = _3;
(*_26) = _44.3.0.1.0;
_57.fld1 = _37.2 ^ _37.2;
_57.fld0 = _37.1.0;
_57.fld2 = _45 as isize;
_24 = _12.0 < (*_26);
(*_26) = _37.1.1.0 - _44.3.0.1.0;
_37.0 = !_44.3.0.3;
_44.3.0.2 = !_55;
_44.0 = core::ptr::addr_of_mut!(_37.0);
_66.0 = _42 + _21;
_66.1 = _55 + _44.3.0.2;
(*_26) = !_44.3.0.1.0;
(*_26) = !_37.1.1.0;
_8 = _35 ^ _57.fld2;
(*_54) = 6875_i16 - (-5671_i16);
_57.fld3 = core::ptr::addr_of!(_49);
(*_54) = 24347_i16;
place!(Field::<u16>(Variant(_1, 2), 1)) = _37.1.1.1;
_12.1 = (*_26) as u16;
match (*_54) {
24347 => bb36,
_ => bb5
}
}
bb36 = {
_37.1.0 = _45 | _57.fld0;
place!(Field::<i32>(Variant(_1, 2), 0)) = (*_26) as i32;
_37.1.1.0 = _32 as i8;
RET = _9;
_50 = -_6;
_34 = -Field::<i32>(Variant(_1, 2), 0);
_65 = core::ptr::addr_of_mut!(_47.0);
_37.1.0 = Field::<i32>(Variant(_1, 2), 0) > _34;
(*_65) = 1882226031_u32 * 2355186684_u32;
_66.1 = !_55;
_1 = Adt26::Variant1 { fld0: _12.1 };
_9 = _2;
(*_65) = !2995637362_u32;
(*_65) = !2054236774_u32;
_13 = _34 >> Field::<u16>(Variant(_1, 1), 0);
_31 = !_57.fld5;
(*_65) = _37.2 as u32;
_66 = (_21, _44.3.0.2);
(*_11) = _28 as f64;
_34 = _13 | _18;
(*_65) = !1532934159_u32;
(*_11) = _12.1 as f64;
_12.1 = !_53;
_25 = [_2,_3];
Goto(bb37)
}
bb37 = {
_61 = (*_54) >> (*_26);
_57.fld4 = _61;
(*_65) = !176566565_u32;
(*_11) = Field::<u16>(Variant(_1, 1), 0) as f64;
(*_65) = 2191063128_u32;
_37.1.1.1 = _22;
_77 = _3;
_57.fld5 = _31 | _31;
_79 = _6 | _57.fld2;
_9 = _77;
_12.1 = !_37.1.1.1;
_59 = _77;
_68 = !201543574170479942381624577511884426932_u128;
_71 = (*_11);
_1 = Adt26::Variant1 { fld0: _37.1.1.1 };
_70 = &mut _68;
_13 = -_34;
_71 = (*_11);
_25 = [_4,_59];
(*_26) = -_44.3.0.1.0;
_44.3.0.1.2 = Adt29::Variant0 { fld0: _55,fld1: Move(_57.fld3),fld2: _10,fld3: _34 };
Goto(bb38)
}
bb38 = {
_37.1.1.2 = Adt29::Variant0 { fld0: _55,fld1: Move(Field::<*const usize>(Variant(_44.3.0.1.2, 0), 1)),fld2: (*_11),fld3: _34 };
(*_11) = -Field::<f64>(Variant(_44.3.0.1.2, 0), 2);
_32 = (-7221467855622387190_i64);
_54 = &mut _57.fld4;
_69 = core::ptr::addr_of!(_70);
(*_65) = 652945375_u32;
(*_65) = 3883551604_u32 & 1186007210_u32;
_54 = &mut _61;
(*_65) = 2818675985_u32 & 3658346018_u32;
_3 = _9;
_20 = &mut (*_65);
(*_11) = -Field::<f64>(Variant(_37.1.1.2, 0), 2);
_78 = _77;
_33 = core::ptr::addr_of!((*_26));
_35 = (*_26) as isize;
_84 = [_44.3.0.2];
(*_20) = _50 as u32;
(*_20) = !2197566064_u32;
(*_33) = _24 as i8;
(*_54) = 6483_i16;
(*_33) = _37.1.1.0 >> _79;
(*_33) = !_37.1.1.0;
place!(Field::<i32>(Variant(_44.3.0.1.2, 0), 3)) = _55 as i32;
Goto(bb39)
}
bb39 = {
place!(Field::<i32>(Variant(_37.1.1.2, 0), 3)) = !_34;
(*_26) = !_44.3.0.1.0;
_28 = _66.0 - _21;
Goto(bb40)
}
bb40 = {
_12.1 = Field::<u16>(Variant(_1, 1), 0) - Field::<u16>(Variant(_1, 1), 0);
(*_26) = _12.0 ^ _12.0;
place!(Field::<*const usize>(Variant(_37.1.1.2, 0), 1)) = core::ptr::addr_of!(_49);
_53 = _37.1.1.1 - _37.1.1.1;
Goto(bb41)
}
bb41 = {
(*_70) = 241180332645258390444680004974245544595_u128;
_65 = core::ptr::addr_of_mut!((*_20));
_7 = !_44.3.0.0;
(*_11) = Field::<f64>(Variant(_44.3.0.1.2, 0), 2) - Field::<f64>(Variant(_44.3.0.1.2, 0), 2);
(*_54) = !20872_i16;
_44.3.0.1.2 = Move(_37.1.1.2);
(*_26) = _12.0 | _44.3.0.1.0;
_64 = core::ptr::addr_of_mut!(_23);
_44.3.0.0 = _14 != _37.1.3;
(*_65) = 3986757587_u32 >> (*_26);
_65 = core::ptr::addr_of_mut!((*_20));
_76 = core::ptr::addr_of!(_69);
_21 = -_66.0;
(*_64) = _37.2 + _37.2;
_14 = _44.3.0.3 - _37.1.3;
(*_54) = (-22880_i16);
(*_65) = 861559339_u32 - 71683931_u32;
(*_54) = 31348_i16 * (-17396_i16);
(*_76) = core::ptr::addr_of!((*_69));
(*_11) = Field::<f64>(Variant(_44.3.0.1.2, 0), 2);
(*_26) = _44.3.0.1.0 * _44.3.0.1.0;
_56 = [(*_54),(*_54)];
_60 = (*_65) as isize;
(*_11) = _71 - Field::<f64>(Variant(_44.3.0.1.2, 0), 2);
Goto(bb42)
}
bb42 = {
(*_70) = 122941296073978553680048533662197619435_u128;
(*_11) = -Field::<f64>(Variant(_44.3.0.1.2, 0), 2);
(*_64) = !_37.2;
(*_26) = (*_70) as i8;
_37.1.1.2 = Move(_44.3.0.1.2);
(*_26) = _44.3.0.1.0;
(*_64) = _37.2 | _37.2;
(*_76) = core::ptr::addr_of!((*_69));
_50 = _49 as isize;
(*_70) = 282554072822905288758090749645959354441_u128 - 168494183095740054389587664942901267975_u128;
_59 = _3;
match _32 {
340282366920938463456153139576145824266 => bb44,
_ => bb43
}
}
bb43 = {
(*_26) = _8 as i8;
_44.1 = _29.3.0.2 as i32;
(*_11) = _44.1 as f64;
(*_11) = _37.0 as f64;
(*_11) = 13072338496315225915019270724560197221_u128 as f64;
_29.3.0.0 = _22 != _12.1;
_23 = 229_u8 - 74_u8;
_43.1.fld1 = !_23;
_34 = -_44.1;
_29.3.0.2 = 22499787444526391081618489679946366197_i128 >> (*_26);
RET = _9;
(*_26) = _12.0;
(*_11) = _43.1.fld1 as f64;
_28 = _21 + _17;
_45 = _29.3.0.0;
(*_11) = _39 as f64;
_43.1.fld0 = !_45;
_35 = _8;
_7 = _37.1.0;
_7 = (*_26) != (*_26);
_33 = core::ptr::addr_of!((*_26));
_37.1.1.0 = (*_33);
(*_33) = _37.1.1.0;
Goto(bb19)
}
bb44 = {
_41 = _45;
_63 = Move(_54);
_44.3.0.1 = Move(_37.1.1);
Goto(bb45)
}
bb45 = {
_37.1.2 = _66.1;
_50 = _35;
_12 = ((*_26), _44.3.0.1.1, Move(_44.3.0.1.2));
_95 = _77;
(*_26) = _44.3.0.1.0 ^ _12.0;
_54 = Move(_16);
(*_70) = _17 as u128;
(*_65) = 1173608980_u32;
_44.3.0 = (_7, Move(_12), _55, _37.1.3);
_67 = (*_70) > (*_70);
(*_11) = _66.1 as f64;
_66.1 = _37.1.2 * Field::<i128>(Variant(_44.3.0.1.2, 0), 0);
_81 = Adt26::Variant1 { fld0: _44.3.0.1.1 };
_64 = core::ptr::addr_of_mut!((*_64));
(*_65) = !795854139_u32;
(*_64) = !_37.2;
_42 = -_28;
_37.1.0 = _45 ^ _24;
(*_65) = !275264061_u32;
(*_26) = (*_70) as i8;
(*_70) = _37.1.2 as u128;
(*_26) = _44.3.0.1.0;
_79 = -_8;
_81 = Move(_1);
_56 = [(-4892_i16),30233_i16];
_44.3.0.1.1 = _53;
_94 = _28 * _21;
_44.3.0.1.0 = (*_64) as i8;
(*_76) = core::ptr::addr_of!((*_69));
match _32 {
0 => bb40,
1 => bb28,
2 => bb46,
3 => bb47,
4 => bb48,
340282366920938463456153139576145824266 => bb50,
_ => bb49
}
}
bb46 = {
_37.0 = _37.1.3;
(*_11) = (*_16) as f64;
_49 = _43.1.fld5 >> (*_16);
_37.1.3 = _29.3.0.3;
(*_11) = 116338553189265924048203574617681063700_u128 as f64;
_42 = _17;
_32 = (-7100046345923602537_i64) << _37.1.3;
(*_26) = -_37.1.1.0;
_22 = !_29.3.0.1.1;
_44.3.0.2 = _29.3.0.2 & _29.3.0.2;
(*_26) = _37.1.1.0 >> _37.1.3;
_37.2 = _23 + _43.1.fld1;
_51.3 = _43.1.fld5 as i16;
_17 = -_28;
_2 = _9;
_54 = Move(_16);
_50 = _6 | _35;
_44.2 = _38.0;
_42 = -_21;
_44.1 = _43.1.fld5 as i32;
(*_26) = _44.3.0.1.0;
(*_11) = _37.1.1.1 as f64;
_26 = &mut _29.3.0.1.0;
(*_26) = !_37.1.1.0;
Goto(bb22)
}
bb47 = {
_22 = _12.1 << _19.1.0;
(*_11) = _8 as f64;
(*_11) = 177915868613188530481361400278486543189_u128 as f64;
Goto(bb12)
}
bb48 = {
Return()
}
bb49 = {
_55 = _44.3.0.2 & _44.3.0.2;
_37.0 = !_51.0;
RET = _4;
_44.1 = _18;
_37.1.0 = (*_26) >= (*_26);
_43.1.fld4 = _51.3;
_22 = !Field::<u16>(Variant(_1, 2), 1);
_44.1 = Field::<i32>(Variant(_1, 2), 0);
(*_26) = _44.3.0.1.0;
(*_26) = _37.1.1.0 + _37.1.1.0;
_57.fld5 = !_43.1.fld5;
_44.3.0.2 = _47.0 as i128;
(*_26) = _44.3.0.1.0;
_43.1.fld2 = _21 as isize;
_16 = &mut _43.1.fld4;
_37.1.3 = _37.0 ^ _44.3.0.3;
(*_26) = _37.1.1.0 & _44.3.0.1.0;
_38.0 = [_37.1.0];
(*_16) = _51.3;
_16 = &mut _51.3;
(*_11) = Field::<i32>(Variant(_1, 2), 0) as f64;
(*_26) = _8 as i8;
_2 = _3;
match _47.0 {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
2987131962 => bb30,
_ => bb29
}
}
bb50 = {
(*_70) = _95 as u128;
(*_64) = _37.2;
_37.1.1.2 = Move(_44.3.0.1.2);
(*_76) = core::ptr::addr_of!((*_69));
_8 = !_6;
(*_64) = !_37.2;
_44.3.0.0 = !_41;
(*_64) = _37.2 | _37.2;
(*_64) = (*_70) as u8;
(*_76) = core::ptr::addr_of!((*_69));
_25 = [_3,_4];
_22 = _44.3.0.1.1 ^ _53;
_3 = _9;
(*_76) = core::ptr::addr_of!((*_69));
_37.1.1.1 = _44.3.0.1.1;
_86 = core::ptr::addr_of_mut!(_49);
(*_20) = _79 as u32;
(*_86) = _31;
(*_76) = core::ptr::addr_of!((*_69));
_13 = _18 + Field::<i32>(Variant(_37.1.1.2, 0), 3);
_32 = 7158839973393739926_i64 * 2435187691187873868_i64;
_78 = _4;
Goto(bb51)
}
bb51 = {
Call(_104 = dump_var(Move(_79), Move(_4), Move(_39), Move(_25)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_104 = dump_var(Move(_35), Move(_23), Move(_45), Move(_49)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_104 = dump_var(Move(_56), Move(_84), Move(_77), Move(_7)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_104 = dump_var(Move(_14), Move(_6), Move(_22), Move(_36)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_104 = dump_var(Move(_68), Move(_55), Move(_59), Move(_2)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_104 = dump_var(Move(_61), _105, _105, _105), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: Adt26,mut _2: u64,mut _3: f64,mut _4: u16,mut _5: char) -> i128 {
mir! {
type RET = i128;
let _6: *mut f64;
let _7: [i8; 1];
let _8: (u32,);
let _9: isize;
let _10: u128;
let _11: &'static mut [bool; 2];
let _12: (bool, (i8, u16, Adt29), i128, u64);
let _13: &'static mut u32;
let _14: *mut &'static i128;
let _15: *const i8;
let _16: &'static u128;
let _17: bool;
let _18: &'static Adt18;
let _19: ([bool; 1],);
let _20: isize;
let _21: [i8; 8];
let _22: bool;
let _23: isize;
let _24: *const usize;
let _25: (&'static u128, Adt29, usize);
let _26: u64;
let _27: bool;
let _28: isize;
let _29: usize;
let _30: [i8; 8];
let _31: (Adt26, (i8, u16, Adt29), (u64, char, u8, i16), (u32,));
let _32: &'static mut &'static &'static Adt18;
let _33: [u64; 2];
let _34: u16;
let _35: f32;
let _36: &'static mut i8;
let _37: &'static i128;
let _38: char;
let _39: [i8; 7];
let _40: Adt51;
let _41: i32;
let _42: *mut u32;
let _43: *const usize;
let _44: &'static i128;
let _45: f32;
let _46: f32;
let _47: f64;
let _48: &'static mut f32;
let _49: u16;
let _50: bool;
let _51: [i8; 7];
let _52: &'static u128;
let _53: ((*mut i128,),);
let _54: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _55: i8;
let _56: *mut f64;
let _57: *const (u32,);
let _58: ([bool; 1],);
let _59: u8;
let _60: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _61: f32;
let _62: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _63: bool;
let _64: (*mut i128,);
let _65: Adt18;
let _66: char;
let _67: (f32, i128);
let _68: f32;
let _69: *mut usize;
let _70: char;
let _71: &'static &'static Adt18;
let _72: &'static mut u32;
let _73: i32;
let _74: (bool, [i8; 7]);
let _75: f32;
let _76: u8;
let _77: u128;
let _78: i32;
let _79: [u64; 2];
let _80: &'static mut &'static &'static Adt18;
let _81: &'static mut &'static &'static Adt18;
let _82: &'static i128;
let _83: *const *const &'static mut u128;
let _84: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _85: &'static mut &'static &'static Adt18;
let _86: i128;
let _87: [i128; 2];
let _88: u16;
let _89: char;
let _90: i32;
let _91: [char; 2];
let _92: u16;
let _93: i128;
let _94: *mut &'static i128;
let _95: isize;
let _96: f64;
let _97: ();
let _98: ();
{
RET = _3 as i128;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb1,
1 => bb2,
943486917 => bb4,
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
_5 = '\u{4bdf1}';
_2 = 11568156578763906493_u64;
_4 = Field::<u16>(Variant(_1, 2), 1);
_3 = 3191_i16 as f64;
_2 = 243_u8 as u64;
_5 = '\u{f3943}';
place!(Field::<u16>(Variant(_1, 2), 1)) = _4;
RET = true as i128;
_3 = 2534468623_u32 as f64;
_6 = core::ptr::addr_of_mut!(_3);
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb1,
1 => bb5,
943486917 => bb7,
_ => bb6
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
place!(Field::<i32>(Variant(_1, 2), 0)) = 1385220318_i32 * (-528749603_i32);
(*_6) = _2 as f64;
(*_6) = 314747478_u32 as f64;
_5 = '\u{46b14}';
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_8 = (978498389_u32,);
(*_6) = (-4100_i16) as f64;
(*_6) = 4_usize as f64;
_7 = [36_i8];
Goto(bb8)
}
bb8 = {
_1 = Adt26::Variant2 { fld0: (-1378206097_i32),fld1: _4 };
(*_6) = 245_u8 as f64;
_5 = '\u{d31c5}';
_4 = Field::<u16>(Variant(_1, 2), 1) << Field::<u16>(Variant(_1, 2), 1);
(*_6) = 189496265742838361512022170074730023737_u128 as f64;
(*_6) = 237_u8 as f64;
_1 = Adt26::Variant2 { fld0: 1607432232_i32,fld1: _4 };
_10 = (*_6) as u128;
(*_6) = (-113_i8) as f64;
(*_6) = RET as f64;
(*_6) = 0_usize as f64;
(*_6) = _8.0 as f64;
_3 = 6_usize as f64;
(*_6) = _10 as f64;
(*_6) = 32198_i16 as f64;
(*_6) = RET as f64;
(*_6) = (-914747443994292975_i64) as f64;
_9 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
(*_6) = Field::<u16>(Variant(_1, 2), 1) as f64;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-1509181461_i32);
_8 = (3423475372_u32,);
match _8.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3423475372 => bb9,
_ => bb4
}
}
bb9 = {
(*_6) = (-36_i8) as f64;
(*_6) = 31_i8 as f64;
_8 = (3360215128_u32,);
(*_6) = 120_i8 as f64;
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_9 = (*_6) as isize;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb6,
1 => bb2,
340282366920938463463374607430259029995 => bb10,
_ => bb8
}
}
bb10 = {
(*_6) = _2 as f64;
_9 = !9223372036854775807_isize;
match _8.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb6,
3360215128 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_13 = &mut _8.0;
_9 = (-9223372036854775808_isize) >> (*_13);
(*_13) = 1247457801_u32 + 1319665199_u32;
_12.0 = false;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-1962201719_i32) >> (*_13);
_2 = !878964298298528743_u64;
_1 = Adt26::Variant2 { fld0: 778490598_i32,fld1: _4 };
(*_13) = !255566622_u32;
_12.1.0 = (-73_i8) & (-10_i8);
(*_13) = 3393261214_u32;
(*_6) = 264218408_i32 as f64;
(*_13) = Field::<u16>(Variant(_1, 2), 1) as u32;
(*_6) = 399570614_i32 as f64;
(*_13) = 2569809427_u32 & 1613159976_u32;
(*_6) = _10 as f64;
_12.3 = (*_6) as u64;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-2031173635_i32) - (-441845517_i32);
_6 = core::ptr::addr_of_mut!((*_6));
_10 = _5 as u128;
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_12.1.1 = _4;
(*_13) = !4169030064_u32;
(*_6) = _12.1.0 as f64;
(*_6) = _10 as f64;
_9 = 29_isize & (-9223372036854775808_isize);
_5 = '\u{9acc}';
Goto(bb13)
}
bb13 = {
(*_6) = _12.1.0 as f64;
_12.0 = !true;
_19.0 = [_12.0];
(*_13) = _4 as u32;
(*_6) = 40_u8 as f64;
(*_6) = 6_usize as f64;
(*_13) = 1695223222_u32 * 1842625190_u32;
_12.0 = true;
(*_13) = !2595728798_u32;
(*_13) = 2287461781_u32 - 1385089435_u32;
(*_13) = 1833995682_u32 - 4007378426_u32;
_16 = &_10;
(*_13) = 738818253_u32;
(*_6) = (*_13) as f64;
(*_6) = 209_u8 as f64;
_7 = [_12.1.0];
(*_13) = 2786873775_u32 - 2022862390_u32;
RET = 36138036500953139188465167830377476576_i128 * (-83872093752844058552201963243570243677_i128);
Call((*_13) = fn15(Move(_16), (*_6), Move(_1), (*_6), (*_6), Move(_6), _19, _12.1.0, _5, _19, (*_16), (*_16)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = !_12.0;
_20 = _9;
_16 = &_10;
(*_13) = !2847527791_u32;
_12.0 = _12.1.0 == _12.1.0;
(*_13) = !5878157_u32;
(*_13) = 510211414_u32 & 3313910292_u32;
_4 = _12.1.1;
(*_13) = 3799767265_u32 & 1159704059_u32;
_10 = 24859849400477343254217565757648245235_u128 | 213430664417260824821150513634267979813_u128;
(*_13) = !2852995155_u32;
_19.0 = [_22];
_12.0 = _12.1.0 > _12.1.0;
(*_13) = 292892443_u32 >> _2;
(*_13) = !335474560_u32;
_12.2 = RET * RET;
_15 = core::ptr::addr_of!(_12.1.0);
_7 = [(*_15)];
_12.1.1 = _4;
(*_13) = 3954303888_u32 << (*_15);
(*_13) = 390366534_u32 - 3334343316_u32;
(*_15) = (-75_i8);
Goto(bb15)
}
bb15 = {
_22 = RET <= _12.2;
_2 = _12.3;
_27 = !_12.0;
(*_13) = 2756775728_u32;
(*_15) = (-128_i8);
(*_13) = 943655044_u32 * 1289079924_u32;
(*_13) = 3942517369_u32 ^ 733624588_u32;
_22 = !_27;
_10 = 64371191338620117412176085327032408681_u128;
(*_13) = 2247887872_u32;
_25.0 = &_10;
_24 = core::ptr::addr_of!(_25.2);
(*_24) = 42_u8 as usize;
_10 = _12.1.1 as u128;
(*_13) = !4044789260_u32;
(*_24) = (*_15) as usize;
_17 = _27 & _22;
(*_13) = !106521544_u32;
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb13,
4 => bb16,
340282366920938463463374607431768211328 => bb18,
_ => bb17
}
}
bb16 = {
Return()
}
bb17 = {
(*_6) = _12.1.0 as f64;
_12.0 = !true;
_19.0 = [_12.0];
(*_13) = _4 as u32;
(*_6) = 40_u8 as f64;
(*_6) = 6_usize as f64;
(*_13) = 1695223222_u32 * 1842625190_u32;
_12.0 = true;
(*_13) = !2595728798_u32;
(*_13) = 2287461781_u32 - 1385089435_u32;
(*_13) = 1833995682_u32 - 4007378426_u32;
_16 = &_10;
(*_13) = 738818253_u32;
(*_6) = (*_13) as f64;
(*_6) = 209_u8 as f64;
_7 = [_12.1.0];
(*_13) = 2786873775_u32 - 2022862390_u32;
RET = 36138036500953139188465167830377476576_i128 * (-83872093752844058552201963243570243677_i128);
Call((*_13) = fn15(Move(_16), (*_6), Move(_1), (*_6), (*_6), Move(_6), _19, _12.1.0, _5, _19, (*_16), (*_16)), ReturnTo(bb14), UnwindUnreachable())
}
bb18 = {
(*_24) = _12.1.1 as usize;
(*_24) = 6_usize & 10093557432845355616_usize;
_31.2.1 = _5;
(*_13) = 4202572281_u32 >> (*_24);
_16 = Move(_25.0);
(*_24) = 5160666726304506870_usize - 252303199864714712_usize;
_31.2.0 = _12.3;
_31.1.2 = Adt29::Variant0 { fld0: RET,fld1: Move(_24),fld2: _3,fld3: (-1346798480_i32) };
_24 = core::ptr::addr_of!(_25.2);
_31.2.2 = !119_u8;
(*_13) = 3437616438_u32 - 4144446921_u32;
(*_24) = (-621378683_i32) as usize;
(*_13) = 1039875446_u32 + 3544596350_u32;
(*_15) = _4 as i8;
_25.1 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_31.1.2, 0), 0),fld1: Move(Field::<*const usize>(Variant(_31.1.2, 0), 1)),fld2: _3,fld3: 762527488_i32 };
_31.1.0 = !(*_15);
_12.3 = _31.2.0 & _31.2.0;
_38 = _5;
_31.2.3 = (-11796_i16) << (*_24);
(*_13) = 2069359443_u32 >> (*_15);
(*_15) = !_31.1.0;
Goto(bb19)
}
bb19 = {
_9 = (*_15) as isize;
(*_15) = _31.1.0 >> _12.2;
Goto(bb20)
}
bb20 = {
_34 = !_12.1.1;
(*_15) = _31.1.0 * _31.1.0;
_28 = !_9;
(*_24) = 9177680218961173572_usize << (*_15);
_29 = (*_24);
(*_13) = 3166186526_u32;
_27 = _17 & _22;
_12.1.1 = _4 >> _20;
(*_15) = _31.1.0 - _31.1.0;
(*_15) = _31.1.0 - _31.1.0;
(*_13) = !46977701_u32;
_31.1.1 = !_12.1.1;
(*_24) = _29 - _29;
_12.0 = !_17;
(*_13) = _12.3 as u32;
(*_24) = _29;
(*_13) = _31.1.1 as u32;
_41 = (-862115519_i32);
_21 = [(*_15),(*_15),(*_15),(*_15),(*_15),(*_15),(*_15),(*_15)];
_12.1.0 = !_31.1.0;
_7 = [(*_15)];
(*_13) = 3910232484_u32;
place!(Field::<f64>(Variant(_25.1, 0), 2)) = 3028185530648554837_i64 as f64;
Goto(bb21)
}
bb21 = {
(*_15) = _31.1.0 << (*_24);
_39 = [(*_15),_31.1.0,(*_15),(*_15),(*_15),(*_15),(*_15)];
(*_13) = 4077663015_u32;
_43 = core::ptr::addr_of!((*_24));
(*_43) = !_29;
_12.1.2 = Adt29::Variant0 { fld0: _12.2,fld1: Move(_24),fld2: _3,fld3: _41 };
_12.1.1 = _31.1.1 - _31.1.1;
_23 = _28 ^ _9;
_12.0 = (*_15) >= (*_15);
_17 = !_27;
(*_43) = _29 >> (*_15);
_23 = _9 >> (*_43);
place!(Field::<f64>(Variant(_12.1.2, 0), 2)) = Field::<f64>(Variant(_25.1, 0), 2);
_17 = !_27;
(*_13) = 2054532768_u32 * 3718581420_u32;
_24 = core::ptr::addr_of!(_25.2);
(*_24) = _29;
_3 = Field::<f64>(Variant(_25.1, 0), 2);
_42 = core::ptr::addr_of_mut!((*_13));
_43 = Move(Field::<*const usize>(Variant(_25.1, 0), 1));
(*_42) = _12.1.1 as u32;
Goto(bb22)
}
bb22 = {
(*_15) = _31.1.0 - _31.1.0;
(*_13) = 1481518633_u32 + 1853048195_u32;
_28 = _23 - _23;
(*_24) = _29;
_33 = [_12.3,_2];
_36 = &mut _31.1.0;
(*_36) = (-6304493136036583749_i64) as i8;
_36 = &mut (*_15);
(*_24) = _29 | _29;
(*_36) = _10 as i8;
_28 = -_23;
_45 = _2 as f32;
match _41 {
0 => bb1,
1 => bb2,
2 => bb23,
3 => bb24,
4 => bb25,
5 => bb26,
340282366920938463463374607430906095937 => bb28,
_ => bb27
}
}
bb23 = {
_22 = !_12.0;
_20 = _9;
_16 = &_10;
(*_13) = !2847527791_u32;
_12.0 = _12.1.0 == _12.1.0;
(*_13) = !5878157_u32;
(*_13) = 510211414_u32 & 3313910292_u32;
_4 = _12.1.1;
(*_13) = 3799767265_u32 & 1159704059_u32;
_10 = 24859849400477343254217565757648245235_u128 | 213430664417260824821150513634267979813_u128;
(*_13) = !2852995155_u32;
_19.0 = [_22];
_12.0 = _12.1.0 > _12.1.0;
(*_13) = 292892443_u32 >> _2;
(*_13) = !335474560_u32;
_12.2 = RET * RET;
_15 = core::ptr::addr_of!(_12.1.0);
_7 = [(*_15)];
_12.1.1 = _4;
(*_13) = 3954303888_u32 << (*_15);
(*_13) = 390366534_u32 - 3334343316_u32;
(*_15) = (-75_i8);
Goto(bb15)
}
bb24 = {
(*_6) = _12.1.0 as f64;
_12.0 = !true;
_19.0 = [_12.0];
(*_13) = _4 as u32;
(*_6) = 40_u8 as f64;
(*_6) = 6_usize as f64;
(*_13) = 1695223222_u32 * 1842625190_u32;
_12.0 = true;
(*_13) = !2595728798_u32;
(*_13) = 2287461781_u32 - 1385089435_u32;
(*_13) = 1833995682_u32 - 4007378426_u32;
_16 = &_10;
(*_13) = 738818253_u32;
(*_6) = (*_13) as f64;
(*_6) = 209_u8 as f64;
_7 = [_12.1.0];
(*_13) = 2786873775_u32 - 2022862390_u32;
RET = 36138036500953139188465167830377476576_i128 * (-83872093752844058552201963243570243677_i128);
Call((*_13) = fn15(Move(_16), (*_6), Move(_1), (*_6), (*_6), Move(_6), _19, _12.1.0, _5, _19, (*_16), (*_16)), ReturnTo(bb14), UnwindUnreachable())
}
bb25 = {
(*_6) = (-36_i8) as f64;
(*_6) = 31_i8 as f64;
_8 = (3360215128_u32,);
(*_6) = 120_i8 as f64;
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_9 = (*_6) as isize;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb6,
1 => bb2,
340282366920938463463374607430259029995 => bb10,
_ => bb8
}
}
bb26 = {
_13 = &mut _8.0;
_9 = (-9223372036854775808_isize) >> (*_13);
(*_13) = 1247457801_u32 + 1319665199_u32;
_12.0 = false;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-1962201719_i32) >> (*_13);
_2 = !878964298298528743_u64;
_1 = Adt26::Variant2 { fld0: 778490598_i32,fld1: _4 };
(*_13) = !255566622_u32;
_12.1.0 = (-73_i8) & (-10_i8);
(*_13) = 3393261214_u32;
(*_6) = 264218408_i32 as f64;
(*_13) = Field::<u16>(Variant(_1, 2), 1) as u32;
(*_6) = 399570614_i32 as f64;
(*_13) = 2569809427_u32 & 1613159976_u32;
(*_6) = _10 as f64;
_12.3 = (*_6) as u64;
place!(Field::<i32>(Variant(_1, 2), 0)) = (-2031173635_i32) - (-441845517_i32);
_6 = core::ptr::addr_of_mut!((*_6));
_10 = _5 as u128;
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_12.1.1 = _4;
(*_13) = !4169030064_u32;
(*_6) = _12.1.0 as f64;
(*_6) = _10 as f64;
_9 = 29_isize & (-9223372036854775808_isize);
_5 = '\u{9acc}';
Goto(bb13)
}
bb27 = {
(*_6) = _12.1.0 as f64;
_12.0 = !true;
_19.0 = [_12.0];
(*_13) = _4 as u32;
(*_6) = 40_u8 as f64;
(*_6) = 6_usize as f64;
(*_13) = 1695223222_u32 * 1842625190_u32;
_12.0 = true;
(*_13) = !2595728798_u32;
(*_13) = 2287461781_u32 - 1385089435_u32;
(*_13) = 1833995682_u32 - 4007378426_u32;
_16 = &_10;
(*_13) = 738818253_u32;
(*_6) = (*_13) as f64;
(*_6) = 209_u8 as f64;
_7 = [_12.1.0];
(*_13) = 2786873775_u32 - 2022862390_u32;
RET = 36138036500953139188465167830377476576_i128 * (-83872093752844058552201963243570243677_i128);
Call((*_13) = fn15(Move(_16), (*_6), Move(_1), (*_6), (*_6), Move(_6), _19, _12.1.0, _5, _19, (*_16), (*_16)), ReturnTo(bb14), UnwindUnreachable())
}
bb28 = {
_25.1 = Adt29::Variant0 { fld0: RET,fld1: Move(_24),fld2: _3,fld3: _41 };
_49 = _10 as u16;
_48 = &mut _45;
(*_36) = (-34_i8);
_25.1 = Adt29::Variant0 { fld0: RET,fld1: Move(_43),fld2: _3,fld3: _41 };
(*_36) = !(-114_i8);
(*_36) = (*_48) as i8;
(*_13) = _23 as u32;
_46 = _10 as f32;
_48 = &mut _46;
_17 = !_27;
_25.2 = _29;
(*_48) = _28 as f32;
(*_48) = Field::<i32>(Variant(_25.1, 0), 3) as f32;
(*_13) = 4073420588_u32;
_54.2.2 = _25.2 << (*_36);
(*_36) = (-22_i8) + (-111_i8);
_51 = _39;
_54.1 = _49 & _4;
(*_36) = (-118_i8);
(*_13) = 8926239082483280627_i64 as u32;
match (*_36) {
0 => bb14,
1 => bb24,
2 => bb26,
3 => bb29,
4 => bb30,
340282366920938463463374607431768211338 => bb32,
_ => bb31
}
}
bb29 = {
Return()
}
bb30 = {
Return()
}
bb31 = {
(*_6) = (-36_i8) as f64;
(*_6) = 31_i8 as f64;
_8 = (3360215128_u32,);
(*_6) = 120_i8 as f64;
(*_6) = Field::<i32>(Variant(_1, 2), 0) as f64;
_9 = (*_6) as isize;
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb6,
1 => bb2,
340282366920938463463374607430259029995 => bb10,
_ => bb8
}
}
bb32 = {
_39 = _51;
(*_13) = 9492130_u32 >> _28;
_35 = -(*_48);
Goto(bb33)
}
bb33 = {
(*_36) = (-125_i8) << (*_13);
(*_48) = _35 - _35;
_54.2.0 = &_10;
(*_36) = 77_i8 + (-24_i8);
_52 = &_10;
place!(Field::<*const usize>(Variant(_25.1, 0), 1)) = core::ptr::addr_of!(_25.2);
_47 = _3;
(*_36) = (*_48) as i8;
_25.0 = &(*_52);
_5 = _38;
_51 = [(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36)];
_48 = &mut _35;
(*_13) = !1232525474_u32;
(*_48) = _54.2.2 as f32;
(*_36) = 105_i8;
(*_13) = Field::<i32>(Variant(_25.1, 0), 3) as u32;
(*_36) = 5_i8 * (-77_i8);
(*_48) = _23 as f32;
place!(Field::<f64>(Variant(_25.1, 0), 2)) = _47;
_54.2.0 = Move(_25.0);
_9 = _28 | _23;
match _41 {
0 => bb5,
340282366920938463463374607430906095937 => bb34,
_ => bb19
}
}
bb34 = {
_53.0.0 = core::ptr::addr_of_mut!(RET);
_55 = (*_36);
_6 = core::ptr::addr_of_mut!(_3);
_5 = _38;
(*_36) = _55 - _55;
_30 = _21;
(*_36) = _55 * _55;
(*_48) = _29 as f32;
(*_6) = _47 + Field::<f64>(Variant(_25.1, 0), 2);
_58.0 = [_27];
(*_13) = _25.2 as u32;
Goto(bb35)
}
bb35 = {
(*_36) = _55 << _9;
(*_13) = 3277772926_u32 | 1599067305_u32;
(*_36) = _27 as i8;
(*_6) = _47 * Field::<f64>(Variant(_25.1, 0), 2);
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
(*_6) = -Field::<f64>(Variant(_25.1, 0), 2);
_54.0 = Move(Field::<*const usize>(Variant(_25.1, 0), 1));
place!(Field::<i128>(Variant(_25.1, 0), 0)) = RET << (*_13);
_7 = [(*_36)];
_50 = !_17;
_62.1.1.0 = (*_36) ^ (*_36);
(*_6) = -Field::<f64>(Variant(_25.1, 0), 2);
(*_36) = _55 - _62.1.1.0;
_64 = (Move(_53.0.0),);
_62.1.1.1 = _5 as u16;
_60.3 = [Field::<i128>(Variant(_25.1, 0), 0)];
_60.2.1 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_25.1, 0), 0),fld1: Move(_54.0),fld2: _3,fld3: _41 };
_42 = core::ptr::addr_of_mut!((*_13));
(*_42) = !2889966470_u32;
(*_48) = (*_42) as f32;
_65.fld3 = core::ptr::addr_of!(_54.2.2);
Goto(bb36)
}
bb36 = {
(*_6) = Field::<f64>(Variant(_25.1, 0), 2) + _47;
(*_48) = 11_u8 as f32;
_62.1.1.1 = _54.1 + _34;
(*_48) = Field::<f64>(Variant(_60.2.1, 0), 2) as f32;
_62.0 = _2 >> (*_36);
_62.1.1 = ((*_36), _34, Move(_60.2.1));
_62.1.3 = _54.1 as u64;
(*_36) = _62.1.1.0;
(*_6) = Field::<f64>(Variant(_62.1.1.2, 0), 2);
(*_42) = (*_48) as u32;
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
_56 = core::ptr::addr_of_mut!((*_6));
(*_48) = 134_u8 as f32;
(*_13) = (-4506186523782351562_i64) as u32;
(*_48) = (*_13) as f32;
_59 = 50_u8 + 44_u8;
(*_36) = _62.1.1.0 | _62.1.1.0;
(*_13) = Field::<i128>(Variant(_25.1, 0), 0) as u32;
(*_48) = _62.0 as f32;
_67.0 = (*_48) * (*_48);
match Field::<i32>(Variant(_62.1.1.2, 0), 3) {
0 => bb6,
1 => bb20,
2 => bb15,
3 => bb37,
4 => bb38,
5 => bb39,
340282366920938463463374607430906095937 => bb41,
_ => bb40
}
}
bb37 = {
(*_36) = _55 << _9;
(*_13) = 3277772926_u32 | 1599067305_u32;
(*_36) = _27 as i8;
(*_6) = _47 * Field::<f64>(Variant(_25.1, 0), 2);
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
(*_6) = -Field::<f64>(Variant(_25.1, 0), 2);
_54.0 = Move(Field::<*const usize>(Variant(_25.1, 0), 1));
place!(Field::<i128>(Variant(_25.1, 0), 0)) = RET << (*_13);
_7 = [(*_36)];
_50 = !_17;
_62.1.1.0 = (*_36) ^ (*_36);
(*_6) = -Field::<f64>(Variant(_25.1, 0), 2);
(*_36) = _55 - _62.1.1.0;
_64 = (Move(_53.0.0),);
_62.1.1.1 = _5 as u16;
_60.3 = [Field::<i128>(Variant(_25.1, 0), 0)];
_60.2.1 = Adt29::Variant0 { fld0: Field::<i128>(Variant(_25.1, 0), 0),fld1: Move(_54.0),fld2: _3,fld3: _41 };
_42 = core::ptr::addr_of_mut!((*_13));
(*_42) = !2889966470_u32;
(*_48) = (*_42) as f32;
_65.fld3 = core::ptr::addr_of!(_54.2.2);
Goto(bb36)
}
bb38 = {
(*_15) = _31.1.0 << (*_24);
_39 = [(*_15),_31.1.0,(*_15),(*_15),(*_15),(*_15),(*_15)];
(*_13) = 4077663015_u32;
_43 = core::ptr::addr_of!((*_24));
(*_43) = !_29;
_12.1.2 = Adt29::Variant0 { fld0: _12.2,fld1: Move(_24),fld2: _3,fld3: _41 };
_12.1.1 = _31.1.1 - _31.1.1;
_23 = _28 ^ _9;
_12.0 = (*_15) >= (*_15);
_17 = !_27;
(*_43) = _29 >> (*_15);
_23 = _9 >> (*_43);
place!(Field::<f64>(Variant(_12.1.2, 0), 2)) = Field::<f64>(Variant(_25.1, 0), 2);
_17 = !_27;
(*_13) = 2054532768_u32 * 3718581420_u32;
_24 = core::ptr::addr_of!(_25.2);
(*_24) = _29;
_3 = Field::<f64>(Variant(_25.1, 0), 2);
_42 = core::ptr::addr_of_mut!((*_13));
_43 = Move(Field::<*const usize>(Variant(_25.1, 0), 1));
(*_42) = _12.1.1 as u32;
Goto(bb22)
}
bb39 = {
_5 = '\u{4bdf1}';
_2 = 11568156578763906493_u64;
_4 = Field::<u16>(Variant(_1, 2), 1);
_3 = 3191_i16 as f64;
_2 = 243_u8 as u64;
_5 = '\u{f3943}';
place!(Field::<u16>(Variant(_1, 2), 1)) = _4;
RET = true as i128;
_3 = 2534468623_u32 as f64;
_6 = core::ptr::addr_of_mut!(_3);
match Field::<i32>(Variant(_1, 2), 0) {
0 => bb1,
1 => bb5,
943486917 => bb7,
_ => bb6
}
}
bb40 = {
Return()
}
bb41 = {
(*_6) = -Field::<f64>(Variant(_62.1.1.2, 0), 2);
(*_36) = !_62.1.1.0;
_29 = !_25.2;
_27 = (*_36) < (*_36);
place!(Field::<f64>(Variant(_62.1.1.2, 0), 2)) = -(*_6);
_54.0 = core::ptr::addr_of!(_60.2.2);
_60.2.1 = Move(_62.1.1.2);
_33 = [_62.0,_62.0];
_53 = (Move(_64),);
_43 = Move(Field::<*const usize>(Variant(_60.2.1, 0), 1));
(*_13) = 4213050353_u32 * 1595617788_u32;
_58 = (_19.0,);
_69 = core::ptr::addr_of_mut!(_54.2.2);
_72 = &mut (*_13);
place!(Field::<i128>(Variant(_60.2.1, 0), 0)) = -Field::<i128>(Variant(_25.1, 0), 0);
_62.2 = _59 << _62.0;
_54.0 = core::ptr::addr_of!((*_69));
_49 = _62.1.1.1 & _62.1.1.1;
_61 = (*_48);
(*_72) = 1393784322_u32 + 2874473737_u32;
Call((*_72) = core::intrinsics::transmute(_38), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
place!(Field::<i32>(Variant(_25.1, 0), 3)) = _41 ^ _41;
(*_6) = Field::<f64>(Variant(_60.2.1, 0), 2);
place!(Field::<*const usize>(Variant(_60.2.1, 0), 1)) = core::ptr::addr_of!(_54.2.2);
_24 = core::ptr::addr_of!((*_69));
_25.1 = Move(_60.2.1);
(*_36) = (*_52) as i8;
_67.0 = (*_48) + (*_48);
_3 = _47;
_65.fld0 = _27 & _50;
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
(*_24) = _29 + _29;
(*_72) = (*_6) as u32;
_48 = &mut _61;
_3 = _47;
(*_69) = !_25.2;
(*_6) = Field::<f64>(Variant(_25.1, 0), 2) + Field::<f64>(Variant(_25.1, 0), 2);
_74.1 = _51;
(*_36) = _62.1.1.0 + _62.1.1.0;
(*_72) = !725821150_u32;
(*_72) = 3141331261_u32 << (*_36);
(*_48) = _67.0;
_65.fld2 = _9;
(*_36) = -_62.1.1.0;
_48 = &mut _67.0;
(*_6) = Field::<f64>(Variant(_25.1, 0), 2) + _47;
Goto(bb43)
}
bb43 = {
_60.2.0 = &(*_52);
_39 = [(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36)];
(*_6) = -Field::<f64>(Variant(_25.1, 0), 2);
_13 = Move(_72);
_47 = (*_6) * (*_6);
(*_6) = Field::<f64>(Variant(_25.1, 0), 2) + _47;
_36 = &mut _55;
(*_48) = (-7184154776808732727_i64) as f32;
_58.0 = [_27];
(*_36) = _62.1.1.0 & _62.1.1.0;
(*_48) = (-28005_i16) as f32;
_63 = !_27;
(*_36) = _62.1.1.0;
match Field::<i32>(Variant(_25.1, 0), 3) {
0 => bb44,
340282366920938463463374607430906095937 => bb46,
_ => bb45
}
}
bb44 = {
(*_6) = Field::<f64>(Variant(_25.1, 0), 2) + _47;
(*_48) = 11_u8 as f32;
_62.1.1.1 = _54.1 + _34;
(*_48) = Field::<f64>(Variant(_60.2.1, 0), 2) as f32;
_62.0 = _2 >> (*_36);
_62.1.1 = ((*_36), _34, Move(_60.2.1));
_62.1.3 = _54.1 as u64;
(*_36) = _62.1.1.0;
(*_6) = Field::<f64>(Variant(_62.1.1.2, 0), 2);
(*_42) = (*_48) as u32;
(*_6) = Field::<f64>(Variant(_25.1, 0), 2);
_56 = core::ptr::addr_of_mut!((*_6));
(*_48) = 134_u8 as f32;
(*_13) = (-4506186523782351562_i64) as u32;
(*_48) = (*_13) as f32;
_59 = 50_u8 + 44_u8;
(*_36) = _62.1.1.0 | _62.1.1.0;
(*_13) = Field::<i128>(Variant(_25.1, 0), 0) as u32;
(*_48) = _62.0 as f32;
_67.0 = (*_48) * (*_48);
match Field::<i32>(Variant(_62.1.1.2, 0), 3) {
0 => bb6,
1 => bb20,
2 => bb15,
3 => bb37,
4 => bb38,
5 => bb39,
340282366920938463463374607430906095937 => bb41,
_ => bb40
}
}
bb45 = {
_34 = !_12.1.1;
(*_15) = _31.1.0 * _31.1.0;
_28 = !_9;
(*_24) = 9177680218961173572_usize << (*_15);
_29 = (*_24);
(*_13) = 3166186526_u32;
_27 = _17 & _22;
_12.1.1 = _4 >> _20;
(*_15) = _31.1.0 - _31.1.0;
(*_15) = _31.1.0 - _31.1.0;
(*_13) = !46977701_u32;
_31.1.1 = !_12.1.1;
(*_24) = _29 - _29;
_12.0 = !_17;
(*_13) = _12.3 as u32;
(*_24) = _29;
(*_13) = _31.1.1 as u32;
_41 = (-862115519_i32);
_21 = [(*_15),(*_15),(*_15),(*_15),(*_15),(*_15),(*_15),(*_15)];
_12.1.0 = !_31.1.0;
_7 = [(*_15)];
(*_13) = 3910232484_u32;
place!(Field::<f64>(Variant(_25.1, 0), 2)) = 3028185530648554837_i64 as f64;
Goto(bb21)
}
bb46 = {
_60.2.0 = Move(_52);
_25.2 = (*_69) & (*_69);
(*_36) = _62.1.1.0 & _62.1.1.0;
(*_48) = _41 as f32;
(*_36) = _62.1.1.0;
_79 = [_62.0,_62.0];
_25.0 = &_10;
_44 = &place!(Field::<i128>(Variant(_25.1, 0), 0));
(*_69) = (-3776_i16) as usize;
(*_36) = _62.1.1.0 + _62.1.1.0;
_26 = 3091421911_u32 as u64;
Goto(bb47)
}
bb47 = {
_22 = (*_36) > (*_36);
Goto(bb48)
}
bb48 = {
_38 = _5;
_87 = [(*_44),(*_44)];
(*_6) = _65.fld2 as f64;
_65.fld2 = _28 << (*_36);
(*_48) = _62.2 as f32;
_60.2.1 = Move(_25.1);
_58.0 = _19.0;
_65.fld1 = 3411772406_u32 as u8;
(*_6) = 3172077813_u32 as f64;
(*_48) = _10 as f32;
_62.0 = _62.1.3;
_4 = 1975098751_u32 as u16;
_78 = _41 << (*_36);
(*_48) = (-5168457463894286799_i64) as f32;
(*_36) = _62.1.1.0;
_68 = (*_48);
(*_36) = _62.1.1.0 & _62.1.1.0;
_60.3 = [RET];
_89 = _38;
_51 = _39;
_36 = &mut _62.1.1.0;
match _41 {
340282366920938463463374607430906095937 => bb49,
_ => bb19
}
}
bb49 = {
_73 = _78 + _78;
_77 = _10 + _10;
_49 = !_34;
_23 = _65.fld2;
(*_48) = _68 - _68;
_60.3 = [Field::<i128>(Variant(_60.2.1, 0), 0)];
_60.2.2 = (*_69) & (*_69);
_54.2.2 = _29 | _25.2;
(*_6) = Field::<f64>(Variant(_60.2.1, 0), 2) + _47;
_54.2.2 = _25.2 ^ _25.2;
Goto(bb50)
}
bb50 = {
Call(_97 = dump_var(Move(_73), Move(_39), Move(_17), Move(_23)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_97 = dump_var(Move(_41), Move(_28), Move(_20), Move(_26)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_97 = dump_var(Move(_51), Move(_7), Move(_55), Move(_49)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_97 = dump_var(Move(_77), Move(_19), Move(_59), Move(_22)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_97 = dump_var(Move(_10), Move(_5), _98, _98), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static u128,mut _2: f64,mut _3: Adt26,mut _4: f64,mut _5: f64,mut _6: *mut f64,mut _7: ([bool; 1],),mut _8: i8,mut _9: char,mut _10: ([bool; 1],),mut _11: u128,mut _12: u128) -> u32 {
mir! {
type RET = u32;
let _13: u8;
let _14: f64;
let _15: &'static mut u32;
let _16: &'static mut f32;
let _17: f64;
let _18: [u64; 3];
let _19: usize;
let _20: [i8; 7];
let _21: &'static mut i8;
let _22: Adt75;
let _23: f64;
let _24: &'static mut u128;
let _25: *mut f64;
let _26: i8;
let _27: *const [bool; 1];
let _28: &'static &'static Adt18;
let _29: isize;
let _30: &'static i128;
let _31: f64;
let _32: [i8; 7];
let _33: bool;
let _34: &'static mut i16;
let _35: &'static Adt18;
let _36: f64;
let _37: f32;
let _38: [isize; 3];
let _39: (char, Adt18, *mut u64);
let _40: i16;
let _41: &'static i128;
let _42: Adt26;
let _43: isize;
let _44: *mut usize;
let _45: u16;
let _46: (&'static u128, Adt29, usize);
let _47: isize;
let _48: isize;
let _49: i8;
let _50: ([bool; 1],);
let _51: &'static Adt48;
let _52: &'static Adt48;
let _53: f32;
let _54: Adt48;
let _55: *const [bool; 1];
let _56: *mut *const usize;
let _57: u32;
let _58: &'static mut u128;
let _59: *mut u8;
let _60: u16;
let _61: &'static Adt48;
let _62: ([bool; 1],);
let _63: bool;
let _64: f32;
let _65: i128;
let _66: u64;
let _67: isize;
let _68: *const usize;
let _69: *mut u32;
let _70: &'static i128;
let _71: isize;
let _72: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _73: bool;
let _74: ((bool, (i8, u16, Adt29), i128, u64),);
let _75: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _76: char;
let _77: isize;
let _78: f64;
let _79: [u64; 2];
let _80: bool;
let _81: *mut *const usize;
let _82: Adt29;
let _83: isize;
let _84: u64;
let _85: &'static mut &'static mut &'static &'static Adt18;
let _86: isize;
let _87: u8;
let _88: i32;
let _89: i16;
let _90: f64;
let _91: isize;
let _92: &'static &'static Adt18;
let _93: *mut u8;
let _94: &'static &'static Adt18;
let _95: i8;
let _96: char;
let _97: *const i8;
let _98: i32;
let _99: i64;
let _100: Adt48;
let _101: u32;
let _102: ();
let _103: ();
{
_11 = _12 >> Field::<u16>(Variant(_3, 2), 1);
_13 = 85_u8 ^ 167_u8;
_7.0 = [false];
_1 = &_11;
_10 = _7;
place!(Field::<u16>(Variant(_3, 2), 1)) = 56397_u16;
_7.0 = [false];
_5 = _2 * _2;
_10.0 = [true];
place!(Field::<i32>(Variant(_3, 2), 0)) = (-616131506_i32) << (*_1);
_7 = _10;
RET = 2966476473_u32;
_7 = (_10.0,);
_5 = _2 * _4;
_8 = -(-16_i8);
place!(Field::<u16>(Variant(_3, 2), 1)) = 55528_u16 + 5388_u16;
_5 = -_2;
_11 = (-83522117698519885071964149241616597790_i128) as u128;
_7 = _10;
_15 = &mut RET;
(*_15) = 919240761_u32 * 720580072_u32;
(*_15) = 2301325624_u32 >> _13;
_7.0 = [false];
_5 = _4 - _4;
(*_15) = 1879030117_u32 << Field::<i32>(Variant(_3, 2), 0);
_7 = (_10.0,);
Goto(bb1)
}
bb1 = {
(*_15) = !100613415_u32;
(*_15) = 1843582195_u32;
(*_15) = 2468460020_u32 * 55540265_u32;
_2 = -_4;
(*_15) = 1874901819_u32 * 4283715256_u32;
_8 = !(-59_i8);
(*_15) = 2479420247_u32 + 3697352715_u32;
_8 = 5_i8 | 100_i8;
_3 = Adt26::Variant2 { fld0: 1698122595_i32,fld1: 59338_u16 };
_14 = _2 * _4;
_12 = _11;
_8 = 63_i8 - (-73_i8);
_1 = &_12;
_11 = 1_usize as u128;
(*_15) = 5048256294689219376_i64 as u32;
place!(Field::<i32>(Variant(_3, 2), 0)) = -681747084_i32;
(*_15) = 4092179917_u32 | 546594155_u32;
(*_15) = 141141831_u32 * 3197444369_u32;
(*_15) = 2360729168_u32;
_10 = _7;
_13 = _9 as u8;
_3 = Adt26::Variant1 { fld0: 16543_u16 };
(*_15) = (-75441180_i32) as u32;
(*_15) = 2887100828_u32 + 578817496_u32;
(*_15) = _8 as u32;
Call((*_15) = fn16(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = 13183738767685040552_u64 as f64;
(*_15) = 3291040543_u32 | 1652307428_u32;
(*_15) = !919128051_u32;
_11 = (*_1);
_6 = core::ptr::addr_of_mut!(_4);
_2 = -(*_6);
_10.0 = _7.0;
(*_6) = -_5;
_7 = _10;
(*_6) = _5;
(*_15) = 857077418_u32 | 3964223297_u32;
_24 = &mut (*_1);
(*_6) = -_14;
(*_6) = -_5;
(*_15) = 1923040422_u32 ^ 2501285227_u32;
_2 = (*_6) - _4;
_14 = (*_6) - (*_6);
(*_6) = _14 * _2;
_20 = [_8,_8,_8,_8,_8,_8,_8];
Goto(bb3)
}
bb3 = {
_19 = 17228899545572553017_usize * 3_usize;
(*_15) = 383304176_u32 >> (*_24);
_26 = _8 | _8;
(*_15) = 3415981162_u32;
_13 = 30_u8 >> (*_24);
(*_15) = !4120178499_u32;
(*_15) = 1825548164_u32;
(*_6) = 54610_u16 as f64;
_27 = core::ptr::addr_of!(_10.0);
(*_6) = _14;
(*_27) = [false];
(*_27) = [true];
(*_15) = !3109021927_u32;
(*_15) = 264383061_u32 | 2971707077_u32;
(*_27) = [false];
_19 = 5_usize ^ 3_usize;
(*_15) = 4141105555_u32 ^ 1986901165_u32;
_29 = 78_isize - (-9223372036854775808_isize);
(*_27) = [false];
Goto(bb4)
}
bb4 = {
(*_6) = _14 + _2;
_27 = core::ptr::addr_of!((*_27));
(*_15) = 3831176859280451410_i64 as u32;
_7.0 = (*_27);
(*_6) = (-39964853820129401578019650205319783103_i128) as f64;
(*_15) = 1352389191_u32 - 401776544_u32;
_10.0 = [false];
_13 = true as u8;
(*_27) = [false];
(*_27) = _7.0;
(*_27) = [false];
(*_27) = _7.0;
(*_27) = [false];
(*_27) = [true];
(*_15) = 4188666328_u32 | 1954997180_u32;
(*_15) = 2708812629_u32 - 2352289272_u32;
(*_27) = _7.0;
(*_15) = (-8080509518343507714_i64) as u32;
_14 = -(*_6);
_18 = [3774375129458804087_u64,825299339945161300_u64,11135934919232599463_u64];
(*_15) = 807358928_u32 | 1407746901_u32;
(*_6) = (-121571196748451469108258414076714607766_i128) as f64;
(*_15) = 3499606891_u32 - 1490030540_u32;
(*_6) = -_5;
(*_6) = _5 - _5;
(*_6) = -_5;
_33 = true;
Goto(bb5)
}
bb5 = {
(*_15) = 644431348_u32;
_11 = (*_24) - (*_24);
_23 = -_5;
_31 = -_2;
_11 = 17259906771560130298_u64 as u128;
(*_15) = _33 as u32;
(*_27) = [_33];
(*_15) = 3494354862_u32;
_17 = -_2;
(*_6) = _5 - _14;
_26 = !_8;
_5 = -(*_6);
Goto(bb6)
}
bb6 = {
_32 = [_26,_26,_8,_26,_8,_26,_26];
_21 = &mut _26;
_25 = Move(_6);
(*_21) = 8910028304475134049_i64 as i8;
(*_21) = _8;
(*_15) = 881458907_u32 ^ 2649223498_u32;
(*_21) = _29 as i8;
(*_21) = _8 >> (*_15);
_33 = false & false;
(*_21) = _8 + _8;
(*_15) = 1679183980_u32 >> (*_21);
_18 = [9572824146762064589_u64,5509205805945482827_u64,2453831950630163951_u64];
_1 = &_11;
(*_21) = -_8;
_37 = _19 as f32;
_29 = 9223372036854775807_isize | 9223372036854775807_isize;
_39.1.fld3 = core::ptr::addr_of!(_39.1.fld5);
Call(_5 = core::intrinsics::fmaf64(_14, _17, _17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_27) = [_33];
(*_15) = !2828380390_u32;
_39.1.fld0 = _33 ^ _33;
_39.1.fld1 = !_13;
(*_21) = -_8;
_25 = core::ptr::addr_of_mut!(_17);
_40 = !9218_i16;
(*_21) = _9 as i8;
(*_25) = _4 + _4;
(*_15) = 2106981501_u32;
_39.1.fld1 = _13 >> _13;
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb3,
2106981501 => bb9,
_ => bb8
}
}
bb8 = {
_4 = 13183738767685040552_u64 as f64;
(*_15) = 3291040543_u32 | 1652307428_u32;
(*_15) = !919128051_u32;
_11 = (*_1);
_6 = core::ptr::addr_of_mut!(_4);
_2 = -(*_6);
_10.0 = _7.0;
(*_6) = -_5;
_7 = _10;
(*_6) = _5;
(*_15) = 857077418_u32 | 3964223297_u32;
_24 = &mut (*_1);
(*_6) = -_14;
(*_6) = -_5;
(*_15) = 1923040422_u32 ^ 2501285227_u32;
_2 = (*_6) - _4;
_14 = (*_6) - (*_6);
(*_6) = _14 * _2;
_20 = [_8,_8,_8,_8,_8,_8,_8];
Goto(bb3)
}
bb9 = {
(*_21) = _8 ^ _8;
(*_21) = !_8;
(*_25) = -_5;
_39.1.fld4 = _40;
_34 = &mut _39.1.fld4;
(*_15) = !1947065106_u32;
(*_27) = [_33];
(*_15) = 3873525056_u32 >> (*_24);
(*_27) = [_33];
_18 = [12619308004342470682_u64,8594912291440310105_u64,540986350849904013_u64];
(*_25) = _13 as f64;
(*_27) = [_33];
(*_27) = _7.0;
(*_21) = _8 << (*_1);
(*_34) = _40;
(*_27) = [_33];
(*_15) = !370294925_u32;
(*_27) = [_33];
_38 = [_29,_29,_29];
(*_34) = _40;
_24 = &mut (*_1);
_46.0 = &(*_24);
_34 = &mut _40;
Goto(bb10)
}
bb10 = {
(*_27) = [_33];
(*_15) = (*_21) as u32;
(*_15) = (*_24) as u32;
(*_15) = 384119853_u32 >> (*_21);
(*_21) = _29 as i8;
_14 = (*_25) - _31;
(*_27) = [_33];
_19 = 0_usize & 14377008314538528016_usize;
(*_15) = !22444590_u32;
_46.0 = Move(_1);
(*_21) = _8;
(*_15) = 2253866711_u32;
(*_15) = 1901486176_u32 + 2947107971_u32;
_36 = _14;
_45 = !51873_u16;
(*_27) = [_33];
(*_25) = _14 * _36;
Call(place!(Field::<u16>(Variant(_3, 1), 0)) = core::intrinsics::bswap(_45), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_34) = -31444_i16;
(*_25) = _14;
(*_25) = _4 + _31;
_8 = 415362271_i32 as i8;
_48 = _29;
(*_21) = _8 << (*_34);
_32 = [(*_21),(*_21),(*_21),(*_21),(*_21),(*_21),(*_21)];
(*_25) = _36;
(*_21) = _8;
(*_27) = _7.0;
(*_25) = _4;
(*_21) = _8;
(*_34) = (-1405_i16) | 18148_i16;
Goto(bb12)
}
bb12 = {
_8 = (*_21);
(*_27) = [_33];
(*_15) = 4142383958_u32 & 4088581300_u32;
(*_27) = _7.0;
_1 = &(*_24);
(*_25) = _2 * _14;
(*_34) = -(-5570_i16);
(*_34) = (-23723_i16);
_14 = (*_25) * (*_25);
_19 = 2_usize ^ 14433321627619849709_usize;
(*_27) = [_33];
(*_27) = [_33];
_37 = _29 as f32;
(*_34) = -(-29830_i16);
(*_21) = 162623707151386983443680334203666884119_i128 as i8;
(*_27) = _7.0;
(*_34) = 5565_i16 * (-53_i16);
(*_15) = !1580663644_u32;
_42 = Adt26::Variant2 { fld0: 178479074_i32,fld1: _45 };
Goto(bb13)
}
bb13 = {
_7 = _10;
(*_15) = 4395187426079627436_u64 as u32;
_23 = -_14;
(*_21) = _8 | _8;
_47 = -_48;
(*_15) = 3271608827_u32 * 1427191485_u32;
(*_27) = _7.0;
(*_27) = [_33];
(*_15) = !856015385_u32;
Goto(bb14)
}
bb14 = {
place!(Field::<u16>(Variant(_42, 2), 1)) = !_45;
(*_21) = _8 + _8;
(*_25) = 1592144674_i32 as f64;
(*_25) = -_23;
(*_27) = _7.0;
(*_27) = _7.0;
(*_34) = 26573_i16 - (-26796_i16);
_8 = _37 as i8;
(*_15) = !1687512174_u32;
_6 = core::ptr::addr_of_mut!(_23);
(*_21) = !_8;
(*_6) = -_14;
(*_34) = (-25508_i16) * (-17311_i16);
(*_21) = _8 & _8;
(*_34) = 1073094440187102529_i64 as i16;
(*_25) = (*_6) - (*_6);
(*_21) = _8 & _8;
(*_21) = _8 ^ _8;
_37 = 5858186077590745817_u64 as f32;
(*_21) = _8 & _8;
Call(_18 = core::intrinsics::transmute(_38), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_18 = [14536542366715928166_u64,4138105743569392333_u64,16394609080500735992_u64];
_59 = core::ptr::addr_of_mut!(_13);
(*_59) = 124_u8;
(*_21) = _8;
(*_6) = (*_25) * (*_25);
(*_27) = [_33];
(*_34) = 16825_i16 + (-16298_i16);
place!(Field::<u16>(Variant(_3, 1), 0)) = Field::<u16>(Variant(_42, 2), 1) - _45;
_62 = ((*_27),);
(*_21) = _8;
(*_15) = 2305952753_u32 ^ 2982689315_u32;
Call(_45 = core::intrinsics::transmute(Field::<u16>(Variant(_3, 1), 0)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_49 = _48 as i8;
_55 = core::ptr::addr_of!((*_27));
(*_15) = 1801772539_u32;
(*_15) = _37 as u32;
(*_15) = 1796768317_u32 << (*_24);
(*_59) = 96_u8;
_66 = 8832491682554024073_u64;
(*_27) = [_33];
(*_27) = [_33];
Goto(bb17)
}
bb17 = {
place!(Field::<i32>(Variant(_42, 2), 0)) = 8200659773522834877_i64 as i32;
(*_21) = _8;
_38 = [_29,_48,_29];
(*_6) = (*_25) + (*_25);
_50.0 = [_33];
(*_6) = (*_25) * (*_25);
_3 = Move(_42);
(*_27) = [_33];
_47 = _29 & _29;
_58 = &mut (*_24);
(*_59) = 159_u8 | 0_u8;
_74.0.0 = _23 <= (*_6);
(*_15) = 3522526470_u32 & 2635741245_u32;
(*_25) = (*_6) + (*_6);
(*_6) = -(*_25);
(*_15) = !3932575038_u32;
(*_27) = [_74.0.0];
Goto(bb18)
}
bb18 = {
_45 = Field::<u16>(Variant(_3, 2), 1) - Field::<u16>(Variant(_3, 2), 1);
_74.0.1.0 = (*_21);
(*_25) = _19 as f64;
_14 = -(*_6);
(*_6) = _19 as f64;
(*_25) = _14 + _36;
_75.1.3 = !_66;
_66 = _75.1.3 * _75.1.3;
_18 = [_66,_75.1.3,_66];
(*_6) = -(*_25);
Goto(bb19)
}
bb19 = {
(*_21) = !_49;
(*_27) = [_74.0.0];
(*_27) = [_74.0.0];
(*_25) = -(*_6);
(*_6) = _14 + _14;
_80 = !_74.0.0;
_58 = Move(_24);
(*_6) = (*_25);
(*_27) = [_74.0.0];
(*_25) = -_14;
(*_21) = _74.0.1.0 ^ _8;
(*_6) = -(*_25);
_75.2 = !(*_59);
(*_27) = [_74.0.0];
(*_15) = 2171794550_u32 & 2222342123_u32;
(*_15) = 1304592024_u32;
(*_15) = 2373216406_u32 * 1686136665_u32;
(*_27) = [_74.0.0];
_74.0.1.1 = _45 + Field::<u16>(Variant(_3, 2), 1);
Goto(bb20)
}
bb20 = {
_29 = _47 - _48;
_4 = -(*_25);
(*_59) = _75.2;
_75.1.1.1 = _74.0.1.1 << (*_34);
_43 = !_29;
(*_6) = _14 + _17;
_7 = ((*_27),);
(*_59) = _75.2 << (*_34);
(*_59) = _66 as u8;
_62 = ((*_27),);
_56 = core::ptr::addr_of_mut!(_68);
_57 = (*_15) + (*_15);
(*_21) = (*_34) as i8;
_67 = _43;
(*_34) = (-24559_i16) & 30723_i16;
(*_27) = [_74.0.0];
_3 = Adt26::Variant2 { fld0: (-1090518447_i32),fld1: _45 };
(*_25) = (*_6) - (*_6);
(*_21) = _49 + _74.0.1.0;
(*_25) = (*_6) * (*_6);
(*_25) = _48 as f64;
_74.0.3 = _66;
(*_27) = [_80];
_2 = (*_6) + (*_6);
(*_15) = _57;
_74.0.2 = (-42375560081101617893920952320395089690_i128) * (-150784469464527260888325840330353549025_i128);
(*_56) = core::ptr::addr_of!(_46.2);
(*_27) = [_80];
Goto(bb21)
}
bb21 = {
(*_56) = core::ptr::addr_of!((*_68));
_88 = (-1244217446_i32) | 1511396390_i32;
(*_25) = -(*_6);
(*_34) = 30241_i16 + 3876_i16;
_77 = _43 | _47;
_60 = _74.0.1.1 * _45;
Goto(bb22)
}
bb22 = {
(*_59) = !_75.2;
(*_68) = _19 * _19;
(*_15) = 215567152643264721429083732619332567132_u128 as u32;
(*_25) = -_14;
(*_59) = _75.2 * _75.2;
(*_25) = -(*_6);
(*_27) = _62.0;
(*_25) = (*_6) * (*_6);
(*_59) = _75.2;
(*_68) = _19 >> _77;
(*_27) = [_80];
Goto(bb23)
}
bb23 = {
_75.1.1.0 = (*_21) * (*_21);
_41 = &_74.0.2;
_31 = (*_25);
(*_34) = 15746_i16;
(*_15) = _57;
_90 = -(*_6);
_75.0 = _75.1.3 >> (*_68);
_67 = _77;
Call(_20 = core::intrinsics::transmute(_32), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_34) = 21220_i16;
place!(Field::<i32>(Variant(_3, 2), 0)) = (*_59) as i32;
(*_56) = core::ptr::addr_of!((*_68));
(*_59) = (*_41) as u8;
_7 = (_10.0,);
(*_68) = (-2285487856261029624_i64) as usize;
_53 = _37 * _37;
_46.1 = Adt29::Variant0 { fld0: (*_41),fld1: Move((*_56)),fld2: (*_25),fld3: _88 };
_75.1.2 = 276325812366775253951423791333071151630_u128 as i128;
(*_21) = (*_34) as i8;
_43 = _29;
(*_59) = Field::<i128>(Variant(_46.1, 0), 0) as u8;
Goto(bb25)
}
bb25 = {
(*_59) = !_75.2;
(*_56) = core::ptr::addr_of!(_46.2);
(*_27) = [_74.0.0];
_75.2 = !(*_59);
(*_15) = _88 as u32;
(*_15) = !_57;
(*_59) = _75.2;
_75.1.1 = (_74.0.1.0, _74.0.1.1, Move(_46.1));
_76 = _9;
(*_15) = !_57;
(*_56) = core::ptr::addr_of!((*_68));
place!(Field::<i128>(Variant(_75.1.1.2, 0), 0)) = (*_41) << Field::<i32>(Variant(_75.1.1.2, 0), 3);
_96 = _76;
(*_34) = (-21455_i16) - 2833_i16;
(*_15) = !_57;
(*_15) = !_57;
(*_56) = Move(Field::<*const usize>(Variant(_75.1.1.2, 0), 1));
(*_25) = (*_6) + (*_6);
(*_6) = (*_15) as f64;
_27 = core::ptr::addr_of!((*_27));
(*_59) = _75.2 ^ _75.2;
_74.0.1.2 = Adt29::Variant0 { fld0: (*_41),fld1: Move((*_56)),fld2: (*_25),fld3: Field::<i32>(Variant(_3, 2), 0) };
(*_15) = !_57;
_68 = core::ptr::addr_of!(_46.2);
Goto(bb26)
}
bb26 = {
_84 = !_75.0;
_29 = _77;
(*_68) = !_19;
_63 = !_80;
_13 = _75.2 - _75.2;
_65 = !(*_41);
_46 = (Move(_1), Move(_74.0.1.2), _19);
_27 = Move(_55);
(*_56) = core::ptr::addr_of!((*_68));
(*_6) = (*_25) - (*_25);
_10 = (_62.0,);
(*_56) = core::ptr::addr_of!((*_68));
(*_21) = _75.1.1.0;
(*_15) = _74.0.0 as u32;
Goto(bb27)
}
bb27 = {
Call(_102 = dump_var(Move(_66), Move(_38), Move(_63), Move(_49)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_102 = dump_var(Move(_9), Move(_20), Move(_45), Move(_8)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_102 = dump_var(Move(_84), Move(_43), Move(_33), Move(_18)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_102 = dump_var(Move(_7), Move(_50), Move(_62), Move(_26)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_102 = dump_var(Move(_13), _103, _103, _103), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16() -> u32 {
mir! {
type RET = u32;
let _1: *mut u32;
let _2: bool;
let _3: f32;
let _4: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _5: &'static mut u128;
let _6: isize;
let _7: u128;
let _8: f64;
let _9: [u64; 1];
let _10: bool;
let _11: isize;
let _12: f64;
let _13: [i128; 2];
let _14: [i8; 8];
let _15: char;
let _16: u16;
let _17: u128;
let _18: &'static mut i8;
let _19: f32;
let _20: f64;
let _21: u128;
let _22: char;
let _23: &'static u128;
let _24: i16;
let _25: i128;
let _26: ((bool, (i8, u16, Adt29), i128, u64),);
let _27: *mut u64;
let _28: Adt63;
let _29: &'static Adt18;
let _30: u8;
let _31: isize;
let _32: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _33: Adt18;
let _34: bool;
let _35: f32;
let _36: char;
let _37: *mut u8;
let _38: [i32; 7];
let _39: u64;
let _40: &'static mut i16;
let _41: *mut *const usize;
let _42: *mut f64;
let _43: isize;
let _44: i8;
let _45: [i8; 7];
let _46: f64;
let _47: bool;
let _48: &'static mut &'static &'static Adt18;
let _49: u128;
let _50: *mut u64;
let _51: (*mut i128,);
let _52: u128;
let _53: Adt51;
let _54: &'static mut &'static mut &'static &'static Adt18;
let _55: *const i8;
let _56: isize;
let _57: f32;
let _58: [i128; 2];
let _59: f32;
let _60: i8;
let _61: *const &'static mut u128;
let _62: isize;
let _63: *mut &'static i128;
let _64: [i16; 2];
let _65: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _66: ();
let _67: ();
{
RET = !1976830858_u32;
RET = '\u{e8f05}' as u32;
RET = (-8670044987846721274_i64) as u32;
RET = 3656521165_u32;
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 1977944364_u32 >> 27_i8;
(*_1) = !2526805058_u32;
(*_1) = !2413380105_u32;
_2 = RET < (*_1);
_2 = (*_1) != (*_1);
(*_1) = 4195833442_u32 << 6557005959921180843_i64;
(*_1) = 112_isize as u32;
(*_1) = (-24301_i16) as u32;
(*_1) = !3292403039_u32;
_2 = !true;
_4.1.0 = _2 ^ _2;
_3 = (*_1) as f32;
(*_1) = 30455_i16 as u32;
_4.2 = _3 as u8;
_4.1.2 = (-57381513362735173747024953410389550004_i128) << (*_1);
(*_1) = 288842315_u32 * 54699103_u32;
(*_1) = 219239304_u32;
_4.0 = 1784466273672556945_u64;
(*_1) = 3_usize as u32;
(*_1) = 3704998720_u32 | 3566406434_u32;
_3 = 7909309527035805816_i64 as f32;
match _4.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
1784466273672556945 => bb6,
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
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = !3678994179_u32;
(*_1) = _4.2 as u32;
Goto(bb7)
}
bb7 = {
(*_1) = 2744486412_u32;
_4.1.3 = _4.1.0 as u64;
(*_1) = 3559393493_u32 << _4.0;
_4.0 = _4.1.3 | _4.1.3;
(*_1) = !4174734085_u32;
_3 = _4.1.2 as f32;
(*_1) = 266734509_u32 + 2462940749_u32;
(*_1) = 3057246086_u32 & 1089514996_u32;
_3 = 29876_u16 as f32;
RET = _4.1.3 as u32;
(*_1) = 415893203_u32;
_6 = 9223372036854775807_isize - 9_isize;
_2 = _4.1.0 <= _4.1.0;
(*_1) = '\u{983f6}' as u32;
_2 = !_4.1.0;
(*_1) = !3419095423_u32;
(*_1) = 2220039366_u32;
match (*_1) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
2220039366 => bb15,
_ => bb14
}
}
bb8 = {
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = !3678994179_u32;
(*_1) = _4.2 as u32;
Goto(bb7)
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
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
(*_1) = !1679039341_u32;
(*_1) = _6 as u32;
_4.2 = 79_u8;
(*_1) = 37366406_u32 | 2825546279_u32;
_4.1.1.0 = !(-119_i8);
(*_1) = 3595703238_u32;
(*_1) = 3702254157_u32 - 701791799_u32;
_4.1.0 = !_2;
_4.1.1.1 = !30826_u16;
(*_1) = 2406471273_u32 & 480855482_u32;
(*_1) = 1984729813_u32 ^ 1475475358_u32;
_4.2 = 1442021912_i32 as u8;
(*_1) = 2568203246_u32;
_4.1.3 = _4.0 | _4.0;
(*_1) = 172792018_u32 + 2864835577_u32;
(*_1) = 3615051974_u32 >> _4.1.1.0;
(*_1) = 3123962132_u32 - 3819693356_u32;
(*_1) = 4147174225_u32 + 797692462_u32;
(*_1) = !1910759794_u32;
_6 = (-47_isize) | 58_isize;
(*_1) = _2 as u32;
(*_1) = _4.1.1.1 as u32;
_4.1.0 = _4.1.3 == _4.1.3;
(*_1) = 1794727370_u32 >> _4.1.3;
Goto(bb16)
}
bb16 = {
_6 = (-9223372036854775808_isize);
_4.1.0 = _2;
(*_1) = 2571283661_u32;
RET = !2741178166_u32;
(*_1) = !757259816_u32;
_4.1.0 = !_2;
RET = _4.1.2 as u32;
_10 = _4.1.0 ^ _2;
(*_1) = 1201805088_u32 & 424837490_u32;
_7 = 287166014087528794237679195650362010280_u128;
(*_1) = 33318079_u32 + 3187017770_u32;
_4.1.3 = !_4.0;
_6 = _10 as isize;
_3 = _4.1.1.0 as f32;
(*_1) = _4.0 as u32;
_2 = (*_1) > (*_1);
_9 = [_4.0];
_1 = core::ptr::addr_of_mut!((*_1));
_5 = &mut _7;
(*_1) = !4067588278_u32;
(*_1) = _6 as u32;
(*_5) = _6 as u128;
(*_1) = !3528237149_u32;
_3 = (*_1) as f32;
_11 = _6 & _6;
Goto(bb17)
}
bb17 = {
(*_5) = _11 as u128;
(*_1) = 0_usize as u32;
_9 = [_4.1.3];
(*_5) = _4.2 as u128;
(*_5) = 43208431879439732426765483950257045154_u128 - 1117913808004583950791032291742526536_u128;
(*_1) = 1089278690_u32;
(*_1) = 3319802013_u32;
(*_5) = 196341405067466588468365878452856709748_u128 + 196280284346448933957744333980077192272_u128;
(*_5) = 111794205266550237846255094921667338834_u128 << _6;
_13 = [_4.1.2,_4.1.2];
(*_1) = _6 as u32;
_14 = [_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0];
_4.1.2 = 33372570635025875668299083636732747756_i128 * 9497246838868336406974111772319015646_i128;
(*_1) = 618078460_u32 * 2102628337_u32;
(*_1) = _4.2 as u32;
_10 = (*_5) >= (*_5);
(*_5) = 207025139293892971406947447166622807073_u128 * 281047427285560340241846690634755437177_u128;
_16 = _4.1.1.1 + _4.1.1.1;
RET = 1489803448_u32 & 867230776_u32;
(*_5) = 69125736249268095008090323507850157027_u128 ^ 255421649654762931356186608218966863017_u128;
_15 = '\u{6983e}';
_10 = _3 > _3;
(*_5) = 192222549442607420439848859911294205979_u128 | 153643582061430975236933453235790265616_u128;
(*_5) = 221625132163142348833301899303387971127_u128 * 146649331302621529220105842401302464126_u128;
Goto(bb18)
}
bb18 = {
(*_1) = 3527889881_u32;
_8 = (*_1) as f64;
_17 = 1676928026_i32 as u128;
_4.1.1.0 = -51_i8;
(*_1) = 2779240661_u32;
(*_1) = !3071500615_u32;
(*_5) = _17;
_15 = '\u{9bd4}';
_19 = _3;
(*_1) = 792122001_u32 & 1191340782_u32;
(*_1) = 613658374_u32;
Goto(bb19)
}
bb19 = {
_12 = _8 - _8;
_17 = (*_5) >> _6;
_9 = [_4.0];
(*_5) = _11 as u128;
(*_1) = 3018994028_u32 << (*_5);
_14 = [_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0];
_3 = _19;
(*_5) = _17 + _17;
_11 = 23739_i16 as isize;
(*_1) = 3277555907_u32;
(*_1) = !3073703942_u32;
(*_1) = 4245136756_u32 | 3882389210_u32;
(*_1) = 1088315817_u32 ^ 1409519135_u32;
(*_1) = 2666723612_u32 >> (*_5);
_8 = _12;
(*_1) = !2463126326_u32;
_8 = _12 * _12;
Call(_8 = fn17(Move(_1), (*_1), _6, (*_5), (*_5), (*_5), (*_5), _15, (*_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_14 = [_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0];
_1 = core::ptr::addr_of_mut!(RET);
_4.0 = _4.1.3;
_14 = [_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0,_4.1.1.0];
_1 = core::ptr::addr_of_mut!((*_1));
(*_5) = _17;
_5 = &mut _17;
(*_1) = 2505416813_u32 >> (*_5);
_2 = !_10;
_15 = '\u{10e75d}';
_4.2 = !69_u8;
_2 = (*_5) != (*_5);
_4.1.2 = (-90972171596549441037937863076069908999_i128);
(*_1) = 91044584_u32;
(*_1) = 1889279797_u32;
_16 = !_4.1.1.1;
(*_5) = 173466613950907338300992356487396375898_u128;
_18 = &mut _4.1.1.0;
_19 = _3 + _3;
match (*_1) {
0 => bb9,
1889279797 => bb22,
_ => bb21
}
}
bb21 = {
Return()
}
bb22 = {
(*_18) = -56_i8;
Goto(bb23)
}
bb23 = {
(*_18) = 30_i8 * 14_i8;
_13 = [(-119218627966274196044824358856812179818_i128),13486297104381707679396538459494235421_i128];
(*_1) = !880478125_u32;
(*_5) = 173383449750202896731100535556805287383_u128;
(*_1) = !2670668098_u32;
(*_5) = 204833412542398177570644512431603846905_u128 - 212910702866154110429253307077163464232_u128;
(*_18) = (-95_i8);
_25 = (-50766918342903836855244759146567241135_i128) ^ (-82997512504291335247555847431957042925_i128);
(*_5) = !234058574869137410477401179679762409131_u128;
_26.0.1.0 = !(*_18);
_22 = _15;
_24 = 17636_i16 << (*_18);
(*_5) = !237071157508004236482644165968253757903_u128;
_26.0.2 = _25 >> (*_18);
_10 = !_2;
_21 = (*_5) << (*_18);
_23 = &(*_5);
(*_5) = _21;
(*_5) = _21 >> (*_1);
_2 = _10;
(*_18) = _26.0.1.0 ^ _26.0.1.0;
_26.0.0 = _2;
_26.0.1.0 = 8105199568019411998_usize as i8;
Call(_8 = fn18(_25, Move(_5), Move(_1), Move(_18), _26.0.2), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_9 = [13227529049860121658_u64];
_5 = &mut _21;
_18 = &mut _26.0.1.0;
_2 = (*_18) != (*_18);
Goto(bb25)
}
bb25 = {
(*_18) = 1_i8;
_15 = _22;
(*_18) = -18_i8;
(*_18) = 46_i8;
(*_18) = -(-89_i8);
(*_5) = _25 as u128;
_16 = 52265_u16 + 58042_u16;
_30 = 97_u8;
(*_5) = !142557547071546491576746146067236367020_u128;
(*_5) = 18828734308342577185471092039854803667_u128;
(*_5) = _25 as u128;
match _30 {
97 => bb26,
_ => bb6
}
}
bb26 = {
(*_5) = 339455672428408618120240355388742135439_u128;
_30 = 5_u8 >> (*_5);
_13 = [_25,_25];
(*_18) = 30_i8 - (-59_i8);
(*_18) = (-45_i8) + (-55_i8);
(*_5) = (-2553250525376944850_i64) as u128;
(*_5) = 198847491144979547463588958821810174548_u128 | 79877822972987193379717365891713099486_u128;
_9 = [1864995740775161882_u64];
(*_18) = (-32_i8) >> _6;
(*_5) = 240175272562920409967999694871480704779_u128 ^ 190868809487847856023925559672928808890_u128;
Goto(bb27)
}
bb27 = {
(*_5) = 104766948074900280621552350262794119691_u128;
(*_5) = 92252833873933412902697709405596634139_u128 << (*_18);
_10 = !_2;
_9 = [5393272586609168633_u64];
(*_5) = !190541142877080479381747925306520344944_u128;
_12 = _8 + _8;
_33.fld0 = _2;
(*_18) = !(-42_i8);
_31 = _6 * _11;
_1 = core::ptr::addr_of_mut!(RET);
_20 = _12;
Goto(bb28)
}
bb28 = {
(*_5) = 145042910564951136374996590537295709282_u128 & 60164282262288292309285370166836660626_u128;
(*_18) = 55_i8 ^ (-3_i8);
Goto(bb29)
}
bb29 = {
(*_18) = (-111_i8) * (-41_i8);
(*_1) = _33.fld0 as u32;
(*_5) = !136347127696715865657911315468866339804_u128;
_14 = [(*_18),(*_18),(*_18),(*_18),(*_18),(*_18),(*_18),(*_18)];
(*_1) = 4093774427_u32;
_34 = _10 & _2;
(*_18) = (-89_i8) << (*_5);
Call((*_1) = core::intrinsics::transmute(_15), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_36 = _15;
_16 = !8202_u16;
_3 = _19 - _19;
_10 = (*_18) >= (*_18);
(*_18) = (-69_i8) & (-21_i8);
_39 = 6540998109652366228_u64 ^ 15823679211688903407_u64;
(*_5) = 137777775851080019838715438931923705289_u128 - 111052450937181139705126876116633717304_u128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = !1083281422_u32;
(*_5) = 275148321626340041001950232337269228297_u128;
_33.fld1 = _6 as u8;
(*_1) = _10 as u32;
(*_5) = 30668969089452214838558872142317161739_u128 ^ 191865398917688481230736908719089123110_u128;
(*_1) = _3 as u32;
(*_1) = !3059844780_u32;
Call((*_5) = core::intrinsics::bswap(198746792239378530237845320688288859277_u128), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_8 = -_20;
_3 = (-8214047283158563431_i64) as f32;
_3 = _19 + _19;
(*_1) = 3363447054_u32;
_13 = [_25,_25];
(*_18) = !(-116_i8);
_27 = core::ptr::addr_of_mut!(_39);
(*_5) = 213238052137794527576766843011078313179_u128;
(*_27) = _24 as u64;
(*_1) = !4123953983_u32;
(*_18) = 99_i8 << (*_27);
_37 = core::ptr::addr_of_mut!(_33.fld1);
(*_27) = 692860359219322802_u64;
(*_37) = _6 as u8;
(*_18) = 51_i8;
(*_37) = _30 | _30;
(*_37) = _30;
_33.fld4 = _24 & _24;
(*_1) = _8 as u32;
(*_1) = 3563769637_u32 ^ 1355341210_u32;
Goto(bb32)
}
bb32 = {
(*_5) = 292101771232383960816673675533988996879_u128;
(*_1) = 2816745560_u32;
(*_18) = (-99_i8);
(*_1) = 3863958066_u32;
_33.fld2 = _6 * _11;
_38 = [1334484255_i32,1313334292_i32,1376757022_i32,482969131_i32,(-714344485_i32),136604242_i32,667813224_i32];
(*_18) = (-16_i8) ^ (-94_i8);
match (*_27) {
0 => bb27,
1 => bb19,
2 => bb4,
692860359219322802 => bb34,
_ => bb33
}
}
bb33 = {
Return()
}
bb34 = {
(*_27) = _3 as u64;
(*_1) = 91404795_u32 ^ 1799596898_u32;
(*_37) = _30 & _30;
(*_1) = 4053851225_u32 | 1318807035_u32;
_49 = (*_5) + (*_5);
(*_37) = _30 - _30;
(*_27) = _3 as u64;
(*_5) = _49 & _49;
_52 = _24 as u128;
(*_18) = 37_i8 + 37_i8;
_43 = _19 as isize;
(*_27) = 10172675147799340849_u64 - 13949269379046865408_u64;
_20 = (*_1) as f64;
_13 = [_25,_25];
_33.fld3 = core::ptr::addr_of!(_33.fld5);
(*_27) = 10776842647886773934_u64;
(*_27) = 2596600246530335470_u64 - 18178665280851045978_u64;
(*_37) = _30;
_2 = !_34;
(*_27) = _34 as u64;
_35 = _3 * _3;
(*_5) = _49;
(*_27) = _16 as u64;
_36 = _15;
(*_27) = 2242529570106925774_u64 - 7951321897762373861_u64;
Goto(bb35)
}
bb35 = {
(*_37) = _30 + _30;
(*_1) = !856790823_u32;
(*_1) = !1503350954_u32;
(*_5) = !_49;
_33.fld3 = core::ptr::addr_of!(_33.fld5);
(*_37) = _30;
(*_37) = _30 << (*_5);
(*_1) = 4292402863_u32 ^ 2398896118_u32;
(*_27) = 3325613438956818793_u64 >> (*_37);
_57 = -_35;
_44 = (*_18) + (*_18);
(*_5) = !_49;
(*_27) = !14074194500497623644_u64;
_6 = _33.fld2;
Goto(bb36)
}
bb36 = {
(*_1) = 5782793913860843839_i64 as u32;
(*_18) = _16 as i8;
(*_5) = !_49;
(*_1) = !818439785_u32;
(*_1) = 932827308_i32 as u32;
(*_18) = 17645374132978072777_usize as i8;
(*_27) = !12992510530542311982_u64;
Goto(bb37)
}
bb37 = {
_56 = !_6;
(*_18) = _33.fld4 as i8;
(*_27) = _36 as u64;
(*_37) = _30 | _30;
_36 = _22;
(*_37) = _30 + _30;
(*_37) = _30 >> (*_18);
_45 = [(*_18),(*_18),(*_18),(*_18),_44,(*_18),(*_18)];
_64 = [_33.fld4,_33.fld4];
(*_18) = _44 + _44;
(*_5) = _52 & _52;
_53 = Adt51::Variant1 { fld0: _25,fld1: (-402066399478940699_i64),fld2: _38,fld3: (*_18) };
_27 = core::ptr::addr_of_mut!((*_27));
place!(Field::<i64>(Variant(_53, 1), 1)) = 2398902509414051093_i64 - 1575426889374367078_i64;
RET = _12 as u32;
(*_5) = !_52;
_59 = -_35;
Goto(bb38)
}
bb38 = {
Call(_66 = dump_var(Move(_13), Move(_14), Move(_38), Move(_16)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_66 = dump_var(Move(_49), Move(_44), Move(_45), Move(_22)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_66 = dump_var(Move(_15), Move(_7), Move(_24), Move(_64)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_66 = dump_var(Move(_31), Move(_25), _67, _67), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut u32,mut _2: u32,mut _3: isize,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: char,mut _9: u32) -> f64 {
mir! {
type RET = f64;
let _10: f32;
let _11: *mut u32;
let _12: u8;
let _13: isize;
let _14: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _15: ();
let _16: ();
{
RET = _6 as f64;
_6 = (-2492608361978909531_i64) as u128;
_6 = _7 * _5;
RET = 4557194782136572149_usize as f64;
Goto(bb1)
}
bb1 = {
_7 = _6;
_7 = 49150_u16 as u128;
_1 = core::ptr::addr_of_mut!(_2);
(*_1) = _9 * _9;
_9 = (*_1) >> _4;
Call(_4 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_1) = !_9;
_2 = _9;
RET = _6 as f64;
_4 = true as u128;
(*_1) = _3 as u32;
_8 = '\u{e79d9}';
(*_1) = _9;
_11 = core::ptr::addr_of_mut!((*_1));
(*_11) = _9 | _9;
(*_1) = _9 - _9;
(*_1) = _9 & _9;
(*_1) = !_9;
_10 = 4_usize as f32;
_2 = _9 | _9;
(*_1) = (-11671_i16) as u32;
_12 = 209_u8;
_11 = core::ptr::addr_of_mut!((*_1));
_4 = _6;
_9 = !(*_11);
(*_1) = _9;
_8 = '\u{17ea5}';
Goto(bb3)
}
bb3 = {
Call(_15 = dump_var(Move(_6), Move(_4), Move(_7), Move(_8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i128,mut _2: &'static mut u128,mut _3: *mut u32,mut _4: &'static mut i8,mut _5: i128) -> f64 {
mir! {
type RET = f64;
let _6: &'static mut i16;
let _7: &'static Adt48;
let _8: isize;
let _9: u32;
let _10: i64;
let _11: i16;
let _12: &'static mut i16;
let _13: bool;
let _14: &'static mut u128;
let _15: isize;
let _16: ([bool; 1],);
let _17: u128;
let _18: Adt25;
let _19: i128;
let _20: Adt25;
let _21: isize;
let _22: isize;
let _23: &'static mut &'static &'static Adt18;
let _24: (*mut i128,);
let _25: char;
let _26: f64;
let _27: f32;
let _28: f64;
let _29: i32;
let _30: i64;
let _31: isize;
let _32: isize;
let _33: *mut usize;
let _34: u16;
let _35: Adt48;
let _36: *mut usize;
let _37: isize;
let _38: *const i8;
let _39: f64;
let _40: &'static &'static Adt18;
let _41: Adt75;
let _42: (*mut i128,);
let _43: &'static i128;
let _44: char;
let _45: bool;
let _46: Adt63;
let _47: &'static Adt48;
let _48: &'static mut &'static &'static Adt18;
let _49: (*mut u64, i32, [bool; 1], ((bool, (i8, u16, Adt29), i128, u64),));
let _50: f64;
let _51: *mut f64;
let _52: isize;
let _53: &'static Adt18;
let _54: Adt25;
let _55: char;
let _56: u64;
let _57: u64;
let _58: *mut *const usize;
let _59: ();
let _60: ();
{
_5 = !_1;
_5 = _1 - _1;
_1 = _5;
Goto(bb1)
}
bb1 = {
RET = _5 as f64;
_1 = _5 ^ _5;
_1 = 3556321253_u32 as i128;
RET = _5 as f64;
_1 = _5 >> _5;
_1 = _5 ^ _5;
_5 = !_1;
_1 = 2728865687348865425_usize as i128;
_1 = 12719446922601708418_u64 as i128;
_5 = -_1;
_1 = _5 << _5;
_5 = _1;
_8 = 84_isize - (-9223372036854775808_isize);
RET = _8 as f64;
_1 = _5;
Goto(bb2)
}
bb2 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb3 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 3657208423_u32 - 3272508411_u32;
_15 = _8 ^ _8;
_16.0 = [true];
_8 = -_15;
_16.0 = [true];
(*_3) = !2989763442_u32;
_16.0 = [false];
_16.0 = [true];
_13 = true ^ false;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 2673451723_u32 >> _15;
(*_3) = !2486471996_u32;
_18.fld0 = (12210392977459191166_u64, '\u{c1335}', 102_u8, (-6384_i16));
(*_3) = 1482078314_u32 >> _18.fld0.0;
_11 = _18.fld0.3 + _18.fld0.3;
(*_3) = 3375998835_u32 - 488934926_u32;
_20.fld0 = _18.fld0;
(*_3) = _8 as u32;
(*_3) = 2784977540_u32;
_18.fld1 = 63889_u16 - 917_u16;
_18.fld0.3 = -_20.fld0.3;
match _20.fld0.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768205072 => bb8,
_ => bb7
}
}
bb4 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb5 = {
RET = _5 as f64;
_1 = _5 ^ _5;
_1 = 3556321253_u32 as i128;
RET = _5 as f64;
_1 = _5 >> _5;
_1 = _5 ^ _5;
_5 = !_1;
_1 = 2728865687348865425_usize as i128;
_1 = 12719446922601708418_u64 as i128;
_5 = -_1;
_1 = _5 << _5;
_5 = _1;
_8 = 84_isize - (-9223372036854775808_isize);
RET = _8 as f64;
_1 = _5;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_8 = _15;
(*_3) = _18.fld0.2 as u32;
_17 = !188051991989012121010978459267364868306_u128;
_5 = _1;
(*_3) = !3312175010_u32;
(*_3) = 2585150460_u32 | 4086222819_u32;
_18.fld0.1 = _20.fld0.1;
(*_3) = 3652757568_u32;
_17 = RET as u128;
_16.0 = [_13];
_16.0 = [_13];
_10 = -4289931621425436062_i64;
(*_3) = 3846404673_u32;
_18.fld1 = 71_i8 as u16;
_14 = &mut _17;
(*_14) = 110976233583449184988755422988446369200_u128 ^ 252155378707798864282128442216547946292_u128;
(*_3) = 1537545034_u32 ^ 1004100210_u32;
_11 = !_18.fld0.3;
(*_14) = _18.fld0.3 as u128;
(*_3) = 406333214_u32 << (*_14);
Goto(bb9)
}
bb9 = {
_1 = 522236744_i32 as i128;
(*_14) = _10 as u128;
_20.fld0 = _18.fld0;
_25 = _18.fld0.1;
_10 = !(-9190220472627099310_i64);
_25 = _20.fld0.1;
_28 = RET + RET;
(*_14) = 188607135085279979198388385969270003975_u128 >> (*_3);
(*_14) = 107909987757150838806700331492016495166_u128;
(*_14) = 157591141338230041036179988279107822066_u128;
(*_3) = 4022474931_u32;
_18.fld0 = (_20.fld0.0, _20.fld0.1, _20.fld0.2, _11);
match (*_3) {
0 => bb6,
1 => bb2,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
4022474931 => bb16,
_ => bb15
}
}
bb10 = {
_8 = _15;
(*_3) = _18.fld0.2 as u32;
_17 = !188051991989012121010978459267364868306_u128;
_5 = _1;
(*_3) = !3312175010_u32;
(*_3) = 2585150460_u32 | 4086222819_u32;
_18.fld0.1 = _20.fld0.1;
(*_3) = 3652757568_u32;
_17 = RET as u128;
_16.0 = [_13];
_16.0 = [_13];
_10 = -4289931621425436062_i64;
(*_3) = 3846404673_u32;
_18.fld1 = 71_i8 as u16;
_14 = &mut _17;
(*_14) = 110976233583449184988755422988446369200_u128 ^ 252155378707798864282128442216547946292_u128;
(*_3) = 1537545034_u32 ^ 1004100210_u32;
_11 = !_18.fld0.3;
(*_14) = _18.fld0.3 as u128;
(*_3) = 406333214_u32 << (*_14);
Goto(bb9)
}
bb11 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb12 = {
Return()
}
bb13 = {
RET = _5 as f64;
_1 = _5 ^ _5;
_1 = 3556321253_u32 as i128;
RET = _5 as f64;
_1 = _5 >> _5;
_1 = _5 ^ _5;
_5 = !_1;
_1 = 2728865687348865425_usize as i128;
_1 = 12719446922601708418_u64 as i128;
_5 = -_1;
_1 = _5 << _5;
_5 = _1;
_8 = 84_isize - (-9223372036854775808_isize);
RET = _8 as f64;
_1 = _5;
Goto(bb2)
}
bb14 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb15 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 3657208423_u32 - 3272508411_u32;
_15 = _8 ^ _8;
_16.0 = [true];
_8 = -_15;
_16.0 = [true];
(*_3) = !2989763442_u32;
_16.0 = [false];
_16.0 = [true];
_13 = true ^ false;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 2673451723_u32 >> _15;
(*_3) = !2486471996_u32;
_18.fld0 = (12210392977459191166_u64, '\u{c1335}', 102_u8, (-6384_i16));
(*_3) = 1482078314_u32 >> _18.fld0.0;
_11 = _18.fld0.3 + _18.fld0.3;
(*_3) = 3375998835_u32 - 488934926_u32;
_20.fld0 = _18.fld0;
(*_3) = _8 as u32;
(*_3) = 2784977540_u32;
_18.fld1 = 63889_u16 - 917_u16;
_18.fld0.3 = -_20.fld0.3;
match _20.fld0.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768205072 => bb8,
_ => bb7
}
}
bb16 = {
(*_3) = 169827442_u32 + 2511553019_u32;
_16.0 = [_13];
_20 = Adt25 { fld0: _18.fld0,fld1: _18.fld1 };
(*_14) = 226960103282479922680374969059074576677_u128 * 295674718206036160111761688578787158048_u128;
(*_14) = _1 as u128;
_21 = _11 as isize;
_20.fld0 = (_18.fld0.0, _25, _18.fld0.2, _11);
(*_14) = 288827830703184334112483778464357225435_u128 * 337582351111063592292769520249272511271_u128;
match _18.fld0.0 {
0 => bb2,
12210392977459191166 => bb18,
_ => bb17
}
}
bb17 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb18 = {
_19 = -_1;
match _18.fld0.2 {
0 => bb12,
1 => bb19,
2 => bb20,
3 => bb21,
102 => bb23,
_ => bb22
}
}
bb19 = {
_8 = _15;
(*_3) = _18.fld0.2 as u32;
_17 = !188051991989012121010978459267364868306_u128;
_5 = _1;
(*_3) = !3312175010_u32;
(*_3) = 2585150460_u32 | 4086222819_u32;
_18.fld0.1 = _20.fld0.1;
(*_3) = 3652757568_u32;
_17 = RET as u128;
_16.0 = [_13];
_16.0 = [_13];
_10 = -4289931621425436062_i64;
(*_3) = 3846404673_u32;
_18.fld1 = 71_i8 as u16;
_14 = &mut _17;
(*_14) = 110976233583449184988755422988446369200_u128 ^ 252155378707798864282128442216547946292_u128;
(*_3) = 1537545034_u32 ^ 1004100210_u32;
_11 = !_18.fld0.3;
(*_14) = _18.fld0.3 as u128;
(*_3) = 406333214_u32 << (*_14);
Goto(bb9)
}
bb20 = {
Return()
}
bb21 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 3657208423_u32 - 3272508411_u32;
_15 = _8 ^ _8;
_16.0 = [true];
_8 = -_15;
_16.0 = [true];
(*_3) = !2989763442_u32;
_16.0 = [false];
_16.0 = [true];
_13 = true ^ false;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 2673451723_u32 >> _15;
(*_3) = !2486471996_u32;
_18.fld0 = (12210392977459191166_u64, '\u{c1335}', 102_u8, (-6384_i16));
(*_3) = 1482078314_u32 >> _18.fld0.0;
_11 = _18.fld0.3 + _18.fld0.3;
(*_3) = 3375998835_u32 - 488934926_u32;
_20.fld0 = _18.fld0;
(*_3) = _8 as u32;
(*_3) = 2784977540_u32;
_18.fld1 = 63889_u16 - 917_u16;
_18.fld0.3 = -_20.fld0.3;
match _20.fld0.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768205072 => bb8,
_ => bb7
}
}
bb22 = {
RET = _5 as f64;
_1 = _5 ^ _5;
_1 = 3556321253_u32 as i128;
RET = _5 as f64;
_1 = _5 >> _5;
_1 = _5 ^ _5;
_5 = !_1;
_1 = 2728865687348865425_usize as i128;
_1 = 12719446922601708418_u64 as i128;
_5 = -_1;
_1 = _5 << _5;
_5 = _1;
_8 = 84_isize - (-9223372036854775808_isize);
RET = _8 as f64;
_1 = _5;
Goto(bb2)
}
bb23 = {
_27 = _18.fld0.2 as f32;
_19 = _11 as i128;
_24.0 = core::ptr::addr_of_mut!(_1);
(*_14) = 73806437532166608119132211598532615922_u128 & 52601590049420863551156881648814705047_u128;
(*_3) = _18.fld1 as u32;
(*_3) = !3918863322_u32;
(*_14) = (-1230973715_i32) as u128;
(*_14) = 209433593071003342025530527221595613464_u128 | 316736702740713324569218586836389829420_u128;
_19 = _1 * _5;
(*_3) = 819169246_u32 >> _20.fld0.0;
(*_14) = 159172960399124609070172011748600626399_u128 >> _8;
_30 = _18.fld0.0 as i64;
_12 = &mut _20.fld0.3;
_8 = _21 ^ _21;
_19 = _1 << (*_3);
_29 = (-1091694448_i32);
(*_3) = 2885414694_u32;
(*_14) = _5 as u128;
Goto(bb24)
}
bb24 = {
_18.fld0.3 = !(*_12);
_30 = _10;
(*_3) = 3747689800_u32 - 1202077079_u32;
(*_14) = 262312051899191215943081669621664964639_u128;
_31 = _8;
Goto(bb25)
}
bb25 = {
_13 = _31 == _15;
(*_12) = _11;
(*_12) = _18.fld0.3 << _8;
(*_14) = 337294544916345277184029376722520327848_u128 << (*_12);
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 4179067789_u32;
_18.fld0.0 = !11285951993384786176_u64;
_16.0 = [_13];
Call((*_12) = core::intrinsics::transmute(_18.fld0.3), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_12) = _18.fld0.3 >> (*_14);
(*_12) = _30 as i16;
(*_12) = _18.fld0.3;
Call((*_12) = fn19(Move(_14), Move(_24), (*_3), (*_3), (*_3), (*_3), Move(_3)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_12) = _18.fld0.0 as i16;
(*_12) = !_18.fld0.3;
_42.0 = core::ptr::addr_of_mut!(_5);
(*_12) = 59_i8 as i16;
_39 = -RET;
Goto(bb28)
}
bb28 = {
_15 = _18.fld0.1 as isize;
_18.fld0 = (6555126639369290728_u64, _25, 104_u8, (*_12));
_8 = _13 as isize;
(*_12) = -_11;
_10 = !_30;
_22 = _21;
_31 = _29 as isize;
(*_12) = _25 as i16;
_18.fld0 = (10998699976103966930_u64, _25, 213_u8, (*_12));
_18.fld0.2 = 141_u8 & 240_u8;
_24.0 = core::ptr::addr_of_mut!(_5);
_25 = _18.fld0.1;
Goto(bb29)
}
bb29 = {
_18.fld0.2 = 177_u8;
(*_12) = -_18.fld0.3;
_26 = _28 - RET;
_31 = -_15;
(*_12) = _11 * _11;
_10 = RET as i64;
_5 = !_19;
_13 = _18.fld0.3 != (*_12);
(*_12) = _27 as i16;
_37 = _22;
_11 = -(*_12);
Goto(bb30)
}
bb30 = {
_6 = &mut (*_12);
_26 = _28;
_22 = _37 * _8;
Goto(bb31)
}
bb31 = {
_27 = _18.fld0.0 as f32;
_49.3.0.3 = _18.fld0.0 & _18.fld0.0;
(*_6) = _18.fld0.1 as i16;
_44 = _18.fld0.1;
_29 = (-708838100_i32) + 433486057_i32;
_19 = _13 as i128;
(*_6) = _18.fld0.3;
_49.3.0.1.1 = (-88_i8) as u16;
(*_6) = 34_i8 as i16;
_39 = _26 - _28;
_52 = !_37;
_49.0 = core::ptr::addr_of_mut!(_49.3.0.3);
_49.3.0.0 = _13 ^ _13;
(*_6) = _11 ^ _11;
_31 = -_8;
_1 = -_19;
match _18.fld0.0 {
0 => bb32,
10998699976103966930 => bb34,
_ => bb33
}
}
bb32 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb33 = {
_1 = (-407137180_i32) as i128;
RET = 2831319365_u32 as f64;
_9 = !2971036290_u32;
RET = 24011_i16 as f64;
_3 = core::ptr::addr_of_mut!(_9);
(*_3) = 2053099341_u32 - 3614689692_u32;
_10 = -7982672813693870365_i64;
(*_3) = !1050443994_u32;
(*_3) = !1290659384_u32;
(*_3) = 3755037427_u32 + 953187231_u32;
(*_3) = 3879805320_u32;
(*_3) = 3733115549_u32 ^ 3410312234_u32;
(*_3) = !366367057_u32;
(*_3) = 2196094327_u32 << _5;
(*_3) = 1869866099_u32 << _8;
_10 = 6395702780902815447_i64 ^ 597113089228792840_i64;
RET = _10 as f64;
(*_3) = !2371613190_u32;
_9 = 210984911_u32 >> _5;
_8 = (-9223372036854775808_isize);
(*_3) = !1008876464_u32;
(*_3) = 1161818679_u32;
(*_3) = 606755670_u32;
RET = (-52_i8) as f64;
Goto(bb3)
}
bb34 = {
_18.fld0 = (_49.3.0.3, _44, 237_u8, (*_6));
_50 = -_39;
_18.fld0.1 = _25;
_37 = 3_usize as isize;
_22 = _8 ^ _15;
(*_6) = _49.3.0.0 as i16;
_49.3.0.1.0 = _13 as i8;
(*_6) = !_18.fld0.3;
_38 = core::ptr::addr_of!(_49.3.0.1.0);
Goto(bb35)
}
bb35 = {
(*_6) = _18.fld0.3 >> (*_38);
(*_6) = _10 as i16;
(*_38) = 5_i8 & (-81_i8);
(*_38) = -(-13_i8);
_49.3.0.1.1 = _18.fld1;
Goto(bb36)
}
bb36 = {
(*_38) = (-7_i8) << _1;
(*_6) = _18.fld0.3 + _18.fld0.3;
_44 = _18.fld0.1;
_13 = (*_6) <= (*_6);
_29 = 1841540481_i32 - 418779148_i32;
_16.0 = [_13];
_9 = 173994045_u32 - 12740073_u32;
_28 = _26 * _50;
_25 = _44;
_49.2 = [_13];
(*_38) = 61_i8;
_44 = _18.fld0.1;
match _18.fld0.2 {
0 => bb20,
1 => bb18,
2 => bb35,
3 => bb15,
4 => bb12,
5 => bb26,
6 => bb17,
237 => bb38,
_ => bb37
}
}
bb37 = {
_18.fld0 = (_49.3.0.3, _44, 237_u8, (*_6));
_50 = -_39;
_18.fld0.1 = _25;
_37 = 3_usize as isize;
_22 = _8 ^ _15;
(*_6) = _49.3.0.0 as i16;
_49.3.0.1.0 = _13 as i8;
(*_6) = !_18.fld0.3;
_38 = core::ptr::addr_of!(_49.3.0.1.0);
Goto(bb35)
}
bb38 = {
_25 = _44;
(*_6) = _18.fld0.3 - _18.fld0.3;
_32 = !_15;
(*_6) = (*_38) as i16;
_45 = !_13;
(*_38) = _1 as i8;
_42 = Move(_24);
_43 = &_1;
(*_6) = _18.fld0.3 + _11;
(*_38) = -83_i8;
_44 = _18.fld0.1;
(*_6) = _18.fld0.3 << (*_43);
_51 = core::ptr::addr_of_mut!(_39);
(*_38) = 4_i8 - 71_i8;
_49.1 = _29 - _29;
(*_38) = (-60_i8);
_25 = _18.fld0.1;
Goto(bb39)
}
bb39 = {
(*_51) = -_28;
_50 = (*_51);
(*_6) = _11 ^ _11;
_51 = core::ptr::addr_of_mut!((*_51));
match _18.fld0.2 {
0 => bb28,
1 => bb29,
2 => bb40,
3 => bb41,
237 => bb43,
_ => bb42
}
}
bb40 = {
_8 = _15;
(*_3) = _18.fld0.2 as u32;
_17 = !188051991989012121010978459267364868306_u128;
_5 = _1;
(*_3) = !3312175010_u32;
(*_3) = 2585150460_u32 | 4086222819_u32;
_18.fld0.1 = _20.fld0.1;
(*_3) = 3652757568_u32;
_17 = RET as u128;
_16.0 = [_13];
_16.0 = [_13];
_10 = -4289931621425436062_i64;
(*_3) = 3846404673_u32;
_18.fld1 = 71_i8 as u16;
_14 = &mut _17;
(*_14) = 110976233583449184988755422988446369200_u128 ^ 252155378707798864282128442216547946292_u128;
(*_3) = 1537545034_u32 ^ 1004100210_u32;
_11 = !_18.fld0.3;
(*_14) = _18.fld0.3 as u128;
(*_3) = 406333214_u32 << (*_14);
Goto(bb9)
}
bb41 = {
_15 = _18.fld0.1 as isize;
_18.fld0 = (6555126639369290728_u64, _25, 104_u8, (*_12));
_8 = _13 as isize;
(*_12) = -_11;
_10 = !_30;
_22 = _21;
_31 = _29 as isize;
(*_12) = _25 as i16;
_18.fld0 = (10998699976103966930_u64, _25, 213_u8, (*_12));
_18.fld0.2 = 141_u8 & 240_u8;
_24.0 = core::ptr::addr_of_mut!(_5);
_25 = _18.fld0.1;
Goto(bb29)
}
bb42 = {
(*_12) = _18.fld0.0 as i16;
(*_12) = !_18.fld0.3;
_42.0 = core::ptr::addr_of_mut!(_5);
(*_12) = 59_i8 as i16;
_39 = -RET;
Goto(bb28)
}
bb43 = {
(*_51) = _28 * _28;
(*_38) = 111_i8;
_3 = core::ptr::addr_of_mut!(_9);
(*_6) = _18.fld0.3 << (*_3);
(*_51) = -RET;
(*_51) = -_28;
(*_38) = _10 as i8;
_6 = &mut _18.fld0.3;
_3 = core::ptr::addr_of_mut!((*_3));
(*_38) = (*_6) as i8;
_39 = -_28;
_15 = 99_u8 as isize;
RET = (*_51) * (*_51);
(*_51) = _28 - RET;
_54.fld0 = (_49.3.0.3, _44, 103_u8, (*_6));
_38 = core::ptr::addr_of!((*_38));
(*_3) = 3094129496_u32 ^ 1125759688_u32;
_52 = _37 >> _54.fld0.3;
_11 = (*_3) as i16;
(*_3) = 380629645_u32 << (*_6);
(*_3) = _54.fld0.3 as u32;
Goto(bb44)
}
bb44 = {
Call(_59 = dump_var(Move(_16), Move(_32), Move(_1), Move(_19)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_59 = dump_var(Move(_31), Move(_37), Move(_21), Move(_5)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_59 = dump_var(Move(_44), Move(_11), Move(_22), _60), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static mut u128,mut _2: (*mut i128,),mut _3: u32,mut _4: u32,mut _5: u32,mut _6: u32,mut _7: *mut u32) -> i16 {
mir! {
type RET = i16;
let _8: *mut i128;
let _9: (u64, (bool, (i8, u16, Adt29), i128, u64), u8, &'static *mut i128);
let _10: u16;
let _11: u64;
let _12: &'static mut i8;
let _13: [bool; 1];
let _14: isize;
let _15: char;
let _16: *mut u8;
let _17: char;
let _18: u32;
let _19: isize;
let _20: (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _21: *mut u32;
let _22: &'static mut &'static &'static Adt18;
let _23: Adt26;
let _24: i16;
let _25: isize;
let _26: [i8; 8];
let _27: (bool, (i8, u16, Adt29), i128, u64);
let _28: Adt48;
let _29: u64;
let _30: usize;
let _31: isize;
let _32: char;
let _33: *mut usize;
let _34: isize;
let _35: bool;
let _36: *const (*const usize, u16, (&'static u128, Adt29, usize), [i128; 1]);
let _37: Adt29;
let _38: usize;
let _39: i64;
let _40: i64;
let _41: isize;
let _42: &'static mut i16;
let _43: &'static i128;
let _44: &'static u128;
let _45: f64;
let _46: f32;
let _47: *const *const &'static mut u128;
let _48: i64;
let _49: u128;
let _50: Adt63;
let _51: *const i8;
let _52: f64;
let _53: isize;
let _54: f64;
let _55: u128;
let _56: u64;
let _57: ();
let _58: ();
{
_8 = Move(_2.0);
_3 = _5;
_3 = (-22_i8) as u32;
_2 = (Move(_8),);
RET = (-86_i8) as i16;
_7 = core::ptr::addr_of_mut!(_5);
(*_7) = _4 - _3;
_8 = Move(_2.0);
_6 = RET as u32;
(*_7) = _4;
_6 = (*_7) >> (*_7);
(*_7) = 6048793417404977348_i64 as u32;
(*_7) = _6 >> _6;
Goto(bb1)
}
bb1 = {
RET = !(-24515_i16);
(*_7) = _3;
(*_7) = (-2042555156_i32) as u32;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4179067789 => bb7,
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
RET = 20284_i16 << _6;
_3 = 224_u8 as u32;
(*_7) = 47_isize as u32;
(*_7) = _4 & _4;
(*_7) = _4 - _4;
_9.0 = 9512638226018625184_u64;
(*_7) = !_6;
Call((*_7) = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9.1.2 = (-121415476481457695855727632073486393435_i128) << _5;
_9.1.1.1 = (-9223372036854775808_isize) as u16;
(*_7) = _3 << _6;
_2 = (Move(_8),);
(*_7) = 6_usize as u32;
_9.1.1.1 = 31265_u16 | 8276_u16;
_6 = !(*_7);
_9.1.0 = !false;
_9.2 = 28_u8;
_9.2 = 8320858607019574320_i64 as u8;
_9.1.1.0 = -36_i8;
_5 = _4 - _3;
_5 = !_4;
_9.1.3 = _9.0 << (*_7);
(*_7) = _3 % _4;
_11 = _9.1.1.0 as u64;
_9.1.1.1 = 27965_u16 >> (*_7);
_4 = (*_7) << (*_7);
Goto(bb9)
}
bb9 = {
(*_7) = _4;
(*_7) = _6 << _9.1.1.1;
(*_7) = _3;
_9.2 = 165_u8 | 248_u8;
(*_7) = _4 ^ _4;
_9.1.2 = (-44670317818859495219809716650287852057_i128) >> (*_7);
(*_7) = _4;
_9.1.1.1 = 61870_u16 >> (*_7);
(*_7) = _4 ^ _4;
_12 = &mut _9.1.1.0;
(*_7) = !_3;
(*_12) = (-77_i8) * (-6_i8);
(*_7) = _4 >> (*_12);
Goto(bb10)
}
bb10 = {
(*_12) = (-69_i8) << (*_7);
(*_12) = 106_i8 - 101_i8;
_14 = -(-120_isize);
(*_12) = (-9_i8);
(*_12) = 59_i8;
(*_12) = 21_i8 & (-89_i8);
(*_7) = _4;
_23 = Adt26::Variant2 { fld0: (-189970218_i32),fld1: 25218_u16 };
(*_12) = !(-74_i8);
_20.0 = core::ptr::addr_of!(_20.2.2);
(*_12) = 58064379315814992211589660626901529514_i128 as i8;
Goto(bb11)
}
bb11 = {
(*_12) = (-86_i8) + 59_i8;
(*_12) = (-72_i8);
place!(Field::<i32>(Variant(_23, 2), 0)) = (-1339591951_i32) ^ 1455948732_i32;
(*_12) = 82_i8 >> (*_7);
(*_12) = 10087_u16 as i8;
_10 = 979_u16 ^ 63305_u16;
(*_12) = 74_i8 >> (*_7);
(*_7) = _6 ^ _4;
(*_7) = !_4;
_27.2 = Field::<i32>(Variant(_23, 2), 0) as i128;
_26 = [(*_12),(*_12),(*_12),(*_12),(*_12),(*_12),(*_12),(*_12)];
_21 = core::ptr::addr_of_mut!((*_7));
Call(_27.1.1 = core::intrinsics::transmute(_10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_7) = !_3;
_27.0 = RET != RET;
_20.2.2 = _14 as usize;
(*_7) = _6 << (*_12);
(*_7) = _6;
_5 = !_6;
(*_7) = _27.2 as u32;
_27.1.1 = !_10;
_3 = (*_7) << _5;
_31 = _14;
_20.1 = _20.2.2 as u16;
(*_12) = 109_i8 - 68_i8;
_27.3 = !_11;
(*_12) = 23_i8;
_7 = core::ptr::addr_of_mut!((*_7));
RET = (-29460_i16);
(*_7) = _3 | _3;
_4 = (*_7) ^ (*_7);
(*_12) = -126_i8;
_13 = [_27.0];
(*_12) = _11 as i8;
_27.0 = !false;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb13,
5 => bb14,
340282366920938463463374607431768181996 => bb16,
_ => bb15
}
}
bb13 = {
RET = 20284_i16 << _6;
_3 = 224_u8 as u32;
(*_7) = 47_isize as u32;
(*_7) = _4 & _4;
(*_7) = _4 - _4;
_9.0 = 9512638226018625184_u64;
(*_7) = !_6;
Call((*_7) = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
RET = !(-24515_i16);
(*_7) = _3;
(*_7) = (-2042555156_i32) as u32;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4179067789 => bb7,
_ => bb6
}
}
bb15 = {
Return()
}
bb16 = {
(*_7) = _4 << _4;
_24 = RET * RET;
(*_7) = _3 + _4;
_2.0 = core::ptr::addr_of_mut!(_27.2);
_13 = [_27.0];
RET = _27.1.1 as i16;
(*_12) = (-10_i8);
(*_12) = (-16_i8) & 121_i8;
_25 = _14 | _14;
(*_12) = (-116_i8) | (-4_i8);
(*_7) = _4;
_8 = Move(_2.0);
_17 = '\u{37401}';
_3 = !(*_7);
_30 = _20.2.2 + _20.2.2;
_29 = Field::<i32>(Variant(_23, 2), 0) as u64;
(*_12) = 82_u8 as i8;
_23 = Adt26::Variant2 { fld0: 368503434_i32,fld1: _10 };
RET = _24 + _24;
_14 = _25;
_6 = !(*_7);
_34 = (-1767198812_i32) as isize;
(*_12) = (-48_i8) & 63_i8;
(*_7) = !_4;
Goto(bb17)
}
bb17 = {
_7 = core::ptr::addr_of_mut!((*_7));
place!(Field::<i32>(Variant(_23, 2), 0)) = (-1010307576_i32);
_18 = _29 as u32;
Goto(bb18)
}
bb18 = {
_35 = !_27.0;
(*_7) = _4 * _4;
_27.1.1 = _20.1 | Field::<u16>(Variant(_23, 2), 1);
(*_7) = !_3;
_10 = _27.1.1 | _27.1.1;
(*_12) = !78_i8;
(*_7) = _3 - _6;
_27.2 = (-19457565444344687349406999888489703400_i128);
(*_7) = 69_u8 as u32;
_8 = core::ptr::addr_of_mut!(_27.2);
(*_12) = (-6_i8);
_27.3 = _29 & _29;
Goto(bb19)
}
bb19 = {
_5 = _4;
(*_12) = (-92_i8) & 40_i8;
_39 = 5858457853849302710_i64 + (-4973130036650648733_i64);
_40 = _39 - _39;
_27.1.0 = (*_12) >> (*_7);
(*_7) = _27.1.1 as u32;
(*_7) = _3;
_26 = [(*_12),(*_12),(*_12),_27.1.0,(*_12),(*_12),(*_12),(*_12)];
(*_8) = (-61029645526735333729136906252961495520_i128) - (-28516596987258525761056230753701348216_i128);
_31 = -_25;
_5 = !_6;
_2 = (Move(_8),);
_8 = core::ptr::addr_of_mut!(_27.2);
(*_8) = _27.0 as i128;
_32 = _17;
_26 = [(*_12),(*_12),(*_12),(*_12),(*_12),(*_12),(*_12),(*_12)];
(*_8) = !(-150426795042924702079935852260081784338_i128);
(*_12) = _27.1.0 + _27.1.0;
_29 = _27.3;
_24 = RET & RET;
(*_8) = Field::<i32>(Variant(_23, 2), 0) as i128;
(*_12) = (*_8) as i8;
(*_7) = _14 as u32;
(*_7) = !_3;
(*_7) = _32 as u32;
Goto(bb20)
}
bb20 = {
_35 = !_27.0;
_35 = _27.0 | _27.0;
_27.1.0 = 88_u8 as i8;
(*_8) = -167054860480840492028762461306250595953_i128;
_46 = RET as f32;
(*_8) = 23133221371501039153645129394227703363_i128;
match (*_8) {
23133221371501039153645129394227703363 => bb22,
_ => bb21
}
}
bb21 = {
RET = 20284_i16 << _6;
_3 = 224_u8 as u32;
(*_7) = 47_isize as u32;
(*_7) = _4 & _4;
(*_7) = _4 - _4;
_9.0 = 9512638226018625184_u64;
(*_7) = !_6;
Call((*_7) = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb22 = {
_41 = _34;
_21 = core::ptr::addr_of_mut!((*_7));
_33 = core::ptr::addr_of_mut!(_30);
_8 = Move(_2.0);
_7 = Move(_21);
_15 = _17;
_20.3 = [_27.2];
_42 = &mut RET;
(*_12) = _27.1.0 * _27.1.0;
_31 = _25 | _14;
Goto(bb23)
}
bb23 = {
_32 = _17;
_41 = _25 & _34;
(*_42) = _46 as i16;
_11 = !_27.3;
(*_42) = Field::<i32>(Variant(_23, 2), 0) as i16;
_18 = !_6;
(*_33) = _20.2.2 | _20.2.2;
(*_33) = _20.2.2;
(*_42) = _24 ^ _24;
_54 = _27.2 as f64;
_20.3 = [_27.2];
(*_12) = _27.1.0 & _27.1.0;
(*_12) = _27.1.0;
_49 = 264653008064666779716143657184350449074_u128 << (*_12);
(*_33) = _20.2.2 >> _6;
_40 = _29 as i64;
_53 = _34;
(*_33) = !_20.2.2;
(*_42) = _18 as i16;
(*_42) = _24;
(*_33) = !_20.2.2;
(*_12) = _15 as i8;
(*_42) = 60_u8 as i16;
(*_33) = _20.2.2;
(*_12) = -_27.1.0;
(*_42) = _24;
(*_33) = _10 as usize;
match _27.2 {
0 => bb19,
1 => bb7,
2 => bb24,
23133221371501039153645129394227703363 => bb26,
_ => bb25
}
}
bb24 = {
_41 = _34;
_21 = core::ptr::addr_of_mut!((*_7));
_33 = core::ptr::addr_of_mut!(_30);
_8 = Move(_2.0);
_7 = Move(_21);
_15 = _17;
_20.3 = [_27.2];
_42 = &mut RET;
(*_12) = _27.1.0 * _27.1.0;
_31 = _25 | _14;
Goto(bb23)
}
bb25 = {
RET = 20284_i16 << _6;
_3 = 224_u8 as u32;
(*_7) = 47_isize as u32;
(*_7) = _4 & _4;
(*_7) = _4 - _4;
_9.0 = 9512638226018625184_u64;
(*_7) = !_6;
Call((*_7) = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb26 = {
(*_33) = _20.2.2;
_20.1 = Field::<u16>(Variant(_23, 2), 1);
_5 = _18 * _18;
_18 = !_4;
(*_12) = _27.1.0 ^ _27.1.0;
_29 = !_11;
(*_12) = _27.1.0 >> _5;
_2.0 = core::ptr::addr_of_mut!(_27.2);
match _27.2 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb23,
23133221371501039153645129394227703363 => bb28,
_ => bb27
}
}
bb27 = {
Return()
}
bb28 = {
_41 = !_31;
_39 = Field::<i32>(Variant(_23, 2), 0) as i64;
(*_42) = -_24;
_21 = core::ptr::addr_of_mut!(_4);
_52 = _54;
_44 = &_49;
(*_42) = _24 * _24;
(*_33) = !_20.2.2;
_20.0 = core::ptr::addr_of!((*_33));
Goto(bb29)
}
bb29 = {
Call(_57 = dump_var(Move(_26), Move(_10), Move(_24), Move(_31)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_57 = dump_var(Move(_30), Move(_15), Move(_4), Move(_13)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_57 = dump_var(Move(_25), Move(_40), Move(_39), Move(_29)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(14654653687691710574_u64), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(45_i8), std::hint::black_box((-13205_i16)), std::hint::black_box((-238895690_i32)), std::hint::black_box((-3895387719226967216_i64)), std::hint::black_box(96136944687659868354659770695292927868_i128), std::hint::black_box(9973311151826785324_usize), std::hint::black_box(530898032_u32));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub struct Adt18 {
fld0: bool,
fld1: u8,
fld2: isize,
fld3: *const usize,
fld4: i16,
fld5: usize,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt25 {
fld0: (u64, char, u8, i16),
fld1: u16,
}
#[derive(Debug)]
pub enum Adt26 {
Variant0{
fld0: u8,
fld1: Adt18,
fld2: isize,
fld3: u64,
fld4: (u64, char, u8, i16),
fld5: u32,
fld6: f64,

},
Variant1{
fld0: u16,

},
Variant2{
fld0: i32,
fld1: u16,

}}
#[derive(Debug)]
pub enum Adt29 {
Variant0{
fld0: i128,
fld1: *const usize,
fld2: f64,
fld3: i32,

},
Variant1{
fld0: u16,
fld1: (u64, char, u8, i16),
fld2: isize,
fld3: *mut i128,
fld4: f32,
fld5: u128,

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: [i32; 7],

},
Variant1{
fld0: (bool, [i8; 7]),
fld1: *mut u8,

},
Variant2{
fld0: u32,
fld1: Adt26,
fld2: (char, Adt18, *mut u64),

},
Variant3{
fld0: (u32,),
fld1: (i8, u16, Adt29),

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: i64,
fld1: (f32, i128),
fld2: *mut *const usize,
fld3: *const i8,
fld4: Adt48,

},
Variant1{
fld0: i128,
fld1: i64,
fld2: [i32; 7],
fld3: i8,

},
Variant2{
fld0: *mut usize,
fld1: (u32,),
fld2: Adt18,
fld3: i8,
fld4: (*mut i128,),

}}
#[derive(Debug)]
pub enum Adt63 {
Variant0{
fld0: [i8; 7],

},
Variant1{
fld0: [i128; 1],
fld1: (u64, char, u8, i16),
fld2: *mut *const usize,
fld3: i8,
fld4: *mut usize,
fld5: Adt51,
fld6: [u128; 7],
fld7: [u64; 2],

}}
#[derive(Debug)]
pub enum Adt75 {
Variant0{
fld0: u8,
fld1: *mut u64,
fld2: [u64; 3],
fld3: Adt18,
fld4: [u64; 1],
fld5: (u64, char, u8, i16),
fld6: *mut i128,

},
Variant1{
fld0: f32,
fld1: *const usize,
fld2: (*mut i128,),
fld3: [i128; 1],
fld4: (u32,),

}}

