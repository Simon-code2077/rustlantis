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
pub fn fn0(mut _1: bool,mut _2: u32,mut _3: usize,mut _4: i8,mut _5: u8,mut _6: i32,mut _7: u128) -> *mut bool {
mir! {
type RET = *mut bool;
let _8: isize;
let _9: *mut Adt18;
let _10: f64;
let _11: isize;
let _12: isize;
let _13: *const u32;
let _14: u16;
let _15: (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _16: isize;
let _17: [i32; 3];
let _18: &'static (Adt58,);
let _19: bool;
let _20: isize;
let _21: *mut *const [i64; 6];
let _22: bool;
let _23: char;
let _24: usize;
let _25: bool;
let _26: isize;
let _27: char;
let _28: u16;
let _29: bool;
let _30: *const [i64; 6];
let _31: &'static f64;
let _32: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _33: u128;
let _34: f32;
let _35: [i128; 5];
let _36: *mut *mut &'static Adt18;
let _37: *const f32;
let _38: (i16, (char, i32, f64, char), i32, i64);
let _39: [u64; 4];
let _40: isize;
let _41: isize;
let _42: (u16, *mut bool);
let _43: u16;
let _44: f64;
let _45: (u16, *mut bool);
let _46: i16;
let _47: char;
let _48: isize;
let _49: char;
let _50: isize;
let _51: Adt31;
let _52: [usize; 7];
let _53: i32;
let _54: *const &'static f64;
let _55: Adt41;
let _56: char;
let _57: ();
let _58: ();
{
_1 = (-7203381860499016845_i64) < 5990152263281096501_i64;
_7 = (-18490_i16) as u128;
RET = core::ptr::addr_of_mut!(_1);
_5 = 31_u8;
(*RET) = true;
_3 = '\u{dd03a}' as usize;
(*RET) = _7 != _7;
(*RET) = false;
_1 = true;
_6 = 1587240843_i32 << _3;
(*RET) = !true;
(*RET) = !true;
(*RET) = false;
(*RET) = true ^ true;
Goto(bb1)
}
bb1 = {
match _5 {
0 => bb2,
1 => bb3,
31 => bb5,
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
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb6 = {
(*RET) = false | true;
(*RET) = !true;
(*RET) = true & false;
match _5 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
31 => bb10,
_ => bb9
}
}
bb7 = {
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
match _5 {
0 => bb2,
1 => bb3,
31 => bb5,
_ => bb4
}
}
bb10 = {
_4 = 48_i8 + 76_i8;
(*RET) = false;
_8 = 9223372036854775807_isize << _3;
_2 = 30326_i16 as u32;
Call((*RET) = fn1(Move(RET), _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = 5850426423655396907_u64 as u8;
Goto(bb12)
}
bb12 = {
_1 = false;
RET = core::ptr::addr_of_mut!(_1);
(*RET) = _4 != _4;
(*RET) = _4 != _4;
(*RET) = _4 != _4;
_5 = _2 as u8;
_2 = !1328339911_u32;
(*RET) = !false;
(*RET) = false;
(*RET) = _8 >= _8;
(*RET) = true ^ false;
(*RET) = _7 <= _7;
(*RET) = !true;
(*RET) = _7 > _7;
(*RET) = true;
(*RET) = _6 < _6;
(*RET) = false;
_5 = 152849231324952540401159482240500949175_i128 as u8;
(*RET) = !false;
_7 = !251793795582405074028346960831825464780_u128;
(*RET) = _7 > _7;
_1 = _6 >= _6;
(*RET) = _4 >= _4;
_7 = 33341542387525684129199275347167869567_u128 >> _6;
(*RET) = true;
_11 = _8 * _8;
(*RET) = false;
_5 = 87_u8 - 200_u8;
Goto(bb13)
}
bb13 = {
(*RET) = !false;
(*RET) = !true;
RET = core::ptr::addr_of_mut!((*RET));
_12 = _11 - _11;
_4 = (-121_i8);
(*RET) = _6 <= _6;
_10 = _12 as f64;
(*RET) = true & false;
_12 = _11;
(*RET) = _3 >= _3;
(*RET) = _4 == _4;
(*RET) = _6 != _6;
(*RET) = _10 > _10;
_3 = 12928845822171596996_usize;
_1 = _11 > _12;
(*RET) = !false;
(*RET) = _8 < _8;
(*RET) = _10 <= _10;
match _4 {
0 => bb9,
1 => bb12,
2 => bb3,
340282366920938463463374607431768211335 => bb14,
_ => bb4
}
}
bb14 = {
_15.2 = [_11,_11,_8,_11,_12];
(*RET) = _7 <= _7;
_5 = _3 as u8;
_13 = core::ptr::addr_of!(_2);
(*RET) = true;
_13 = core::ptr::addr_of!(_2);
(*RET) = !false;
(*_13) = _10 as u32;
(*_13) = 509836161_u32 + 1771767718_u32;
(*RET) = (*_13) < (*_13);
(*_13) = 4203276715_u32;
(*RET) = false;
(*_13) = 1061975424_u32 + 3589357157_u32;
(*RET) = !false;
(*RET) = (*_13) < (*_13);
(*RET) = !false;
(*_13) = 3580933234_u32;
_6 = 3129023940396907823_i64 as i32;
_17 = [_6,_6,_6];
_1 = _5 > _5;
_3 = 4784254560967573180_usize;
(*RET) = true;
(*_13) = 997806223_u32 ^ 488499745_u32;
(*RET) = false ^ true;
(*RET) = !true;
(*_13) = 846879869_u32 * 2336844087_u32;
(*_13) = _3 as u32;
Goto(bb15)
}
bb15 = {
(*RET) = _2 < (*_13);
(*_13) = 1292813512_u32;
(*_13) = 914050179_u32 + 3900865432_u32;
(*_13) = 2821162300_u32 << _12;
Goto(bb16)
}
bb16 = {
_14 = 41157_u16 | 56581_u16;
_13 = core::ptr::addr_of!((*_13));
(*_13) = 2858452390_u32;
(*_13) = (*RET) as u32;
(*_13) = 3893405423_u32 + 3959127628_u32;
_16 = _12;
(*_13) = !862038173_u32;
_19 = (*_13) > (*_13);
(*RET) = !_19;
(*_13) = 596461385_u32 + 1691214935_u32;
(*_13) = !1701052958_u32;
(*_13) = 1030577393_u32;
(*RET) = (*_13) > (*_13);
(*RET) = _19;
_6 = -166321164_i32;
_11 = _16 >> (*_13);
_7 = 263239692233275948398618590702201455913_u128 * 81337909403118636604197580630241728895_u128;
RET = core::ptr::addr_of_mut!((*RET));
(*_13) = '\u{1fa35}' as u32;
_6 = !(-289047101_i32);
(*RET) = _19;
_20 = _16;
(*RET) = _12 > _16;
_20 = -_12;
(*_13) = _7 as u32;
_4 = 111_i8;
match _4 {
0 => bb17,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
111 => bb25,
_ => bb24
}
}
bb17 = {
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb18 = {
_15.2 = [_11,_11,_8,_11,_12];
(*RET) = _7 <= _7;
_5 = _3 as u8;
_13 = core::ptr::addr_of!(_2);
(*RET) = true;
_13 = core::ptr::addr_of!(_2);
(*RET) = !false;
(*_13) = _10 as u32;
(*_13) = 509836161_u32 + 1771767718_u32;
(*RET) = (*_13) < (*_13);
(*_13) = 4203276715_u32;
(*RET) = false;
(*_13) = 1061975424_u32 + 3589357157_u32;
(*RET) = !false;
(*RET) = (*_13) < (*_13);
(*RET) = !false;
(*_13) = 3580933234_u32;
_6 = 3129023940396907823_i64 as i32;
_17 = [_6,_6,_6];
_1 = _5 > _5;
_3 = 4784254560967573180_usize;
(*RET) = true;
(*_13) = 997806223_u32 ^ 488499745_u32;
(*RET) = false ^ true;
(*RET) = !true;
(*_13) = 846879869_u32 * 2336844087_u32;
(*_13) = _3 as u32;
Goto(bb15)
}
bb19 = {
Return()
}
bb20 = {
_1 = false;
RET = core::ptr::addr_of_mut!(_1);
(*RET) = _4 != _4;
(*RET) = _4 != _4;
(*RET) = _4 != _4;
_5 = _2 as u8;
_2 = !1328339911_u32;
(*RET) = !false;
(*RET) = false;
(*RET) = _8 >= _8;
(*RET) = true ^ false;
(*RET) = _7 <= _7;
(*RET) = !true;
(*RET) = _7 > _7;
(*RET) = true;
(*RET) = _6 < _6;
(*RET) = false;
_5 = 152849231324952540401159482240500949175_i128 as u8;
(*RET) = !false;
_7 = !251793795582405074028346960831825464780_u128;
(*RET) = _7 > _7;
_1 = _6 >= _6;
(*RET) = _4 >= _4;
_7 = 33341542387525684129199275347167869567_u128 >> _6;
(*RET) = true;
_11 = _8 * _8;
(*RET) = false;
_5 = 87_u8 - 200_u8;
Goto(bb13)
}
bb21 = {
_5 = 5850426423655396907_u64 as u8;
Goto(bb12)
}
bb22 = {
_4 = 48_i8 + 76_i8;
(*RET) = false;
_8 = 9223372036854775807_isize << _3;
_2 = 30326_i16 as u32;
Call((*RET) = fn1(Move(RET), _7), ReturnTo(bb11), UnwindUnreachable())
}
bb23 = {
Return()
}
bb24 = {
(*RET) = false | true;
(*RET) = !true;
(*RET) = true & false;
match _5 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
31 => bb10,
_ => bb9
}
}
bb25 = {
(*_13) = 2447823810_u32 | 3720250875_u32;
(*RET) = !_19;
(*RET) = _19;
_5 = !154_u8;
(*RET) = !_19;
_3 = _5 as usize;
_19 = !(*RET);
_19 = (*RET);
(*RET) = _19 | _19;
(*_13) = 779895788_u32 | 861606188_u32;
match _4 {
0 => bb26,
1 => bb27,
2 => bb28,
3 => bb29,
111 => bb31,
_ => bb30
}
}
bb26 = {
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb27 = {
(*RET) = _2 < (*_13);
(*_13) = 1292813512_u32;
(*_13) = 914050179_u32 + 3900865432_u32;
(*_13) = 2821162300_u32 << _12;
Goto(bb16)
}
bb28 = {
_4 = 48_i8 + 76_i8;
(*RET) = false;
_8 = 9223372036854775807_isize << _3;
_2 = 30326_i16 as u32;
Call((*RET) = fn1(Move(RET), _7), ReturnTo(bb11), UnwindUnreachable())
}
bb29 = {
Return()
}
bb30 = {
_15.2 = [_11,_11,_8,_11,_12];
(*RET) = _7 <= _7;
_5 = _3 as u8;
_13 = core::ptr::addr_of!(_2);
(*RET) = true;
_13 = core::ptr::addr_of!(_2);
(*RET) = !false;
(*_13) = _10 as u32;
(*_13) = 509836161_u32 + 1771767718_u32;
(*RET) = (*_13) < (*_13);
(*_13) = 4203276715_u32;
(*RET) = false;
(*_13) = 1061975424_u32 + 3589357157_u32;
(*RET) = !false;
(*RET) = (*_13) < (*_13);
(*RET) = !false;
(*_13) = 3580933234_u32;
_6 = 3129023940396907823_i64 as i32;
_17 = [_6,_6,_6];
_1 = _5 > _5;
_3 = 4784254560967573180_usize;
(*RET) = true;
(*_13) = 997806223_u32 ^ 488499745_u32;
(*RET) = false ^ true;
(*RET) = !true;
(*_13) = 846879869_u32 * 2336844087_u32;
(*_13) = _3 as u32;
Goto(bb15)
}
bb31 = {
_17 = [_6,_6,_6];
(*RET) = _19;
(*RET) = _10 > _10;
(*RET) = _7 != _7;
(*_13) = 1190674588_u32 | 3996643191_u32;
(*_13) = 149011748_u32 << _5;
(*_13) = 3343503104_u32 - 3184777675_u32;
Goto(bb32)
}
bb32 = {
(*_13) = !1516793052_u32;
_1 = _19;
_10 = _14 as f64;
(*_13) = !4289960820_u32;
_22 = !(*RET);
(*_13) = 3623395746_u32 >> _20;
(*RET) = _22;
(*RET) = (*_13) < (*_13);
(*_13) = 1323045133_u32;
_10 = 16615410734976171392_u64 as f64;
_23 = '\u{2ee7a}';
(*_13) = 2501000309_u32;
(*RET) = _22 | _19;
(*RET) = _12 <= _11;
_1 = _22 & _22;
(*_13) = _6 as u32;
(*RET) = _19 & _19;
_22 = !(*RET);
(*RET) = _22 & _19;
(*_13) = 3834042737_u32 >> _12;
(*RET) = !_22;
_13 = core::ptr::addr_of!((*_13));
(*_13) = !2938374825_u32;
(*_13) = 1992317783_u32 >> _11;
(*RET) = !_19;
(*_13) = 926907053_u32;
(*RET) = _22 & _19;
(*RET) = !_19;
match (*_13) {
0 => bb26,
1 => bb24,
2 => bb12,
3 => bb33,
4 => bb34,
5 => bb35,
926907053 => bb37,
_ => bb36
}
}
bb33 = {
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb34 = {
_14 = 41157_u16 | 56581_u16;
_13 = core::ptr::addr_of!((*_13));
(*_13) = 2858452390_u32;
(*_13) = (*RET) as u32;
(*_13) = 3893405423_u32 + 3959127628_u32;
_16 = _12;
(*_13) = !862038173_u32;
_19 = (*_13) > (*_13);
(*RET) = !_19;
(*_13) = 596461385_u32 + 1691214935_u32;
(*_13) = !1701052958_u32;
(*_13) = 1030577393_u32;
(*RET) = (*_13) > (*_13);
(*RET) = _19;
_6 = -166321164_i32;
_11 = _16 >> (*_13);
_7 = 263239692233275948398618590702201455913_u128 * 81337909403118636604197580630241728895_u128;
RET = core::ptr::addr_of_mut!((*RET));
(*_13) = '\u{1fa35}' as u32;
_6 = !(-289047101_i32);
(*RET) = _19;
_20 = _16;
(*RET) = _12 > _16;
_20 = -_12;
(*_13) = _7 as u32;
_4 = 111_i8;
match _4 {
0 => bb17,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
111 => bb25,
_ => bb24
}
}
bb35 = {
Return()
}
bb36 = {
_8 = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*RET) = !true;
(*RET) = _8 <= _8;
(*RET) = !false;
(*RET) = !true;
Goto(bb6)
}
bb37 = {
(*_13) = !1276399019_u32;
(*RET) = _22;
(*RET) = _20 != _8;
(*_13) = 3218395071_u32 ^ 1612479180_u32;
_4 = (-9_i8) ^ (-61_i8);
_6 = 1381878721_i32 >> (*_13);
_26 = _8;
(*_13) = _3 as u32;
_23 = '\u{d2b0a}';
_23 = '\u{bc165}';
_8 = !_12;
_3 = 5_usize >> _12;
(*RET) = _23 <= _23;
(*_13) = 2557630814_u32 * 2658486625_u32;
(*_13) = 1886156670_u32 ^ 144712277_u32;
_27 = _23;
_12 = _8 ^ _11;
(*RET) = _22;
_29 = !(*RET);
(*RET) = !_22;
_22 = (*RET);
_28 = (-4686677424526534691_i64) as u16;
Goto(bb38)
}
bb38 = {
(*RET) = (*_13) != (*_13);
_26 = _16 - _16;
_11 = !_20;
(*_13) = 2398168084_u32 * 3824983633_u32;
(*RET) = _26 >= _12;
_29 = (*RET) == (*RET);
(*RET) = _29 ^ _29;
_1 = !_29;
(*RET) = _29;
(*_13) = _20 as u32;
(*_13) = !3363822868_u32;
Goto(bb39)
}
bb39 = {
_3 = 2_usize >> _20;
(*_13) = 2177045413_u32 >> _26;
(*_13) = !225138046_u32;
_3 = 1_usize - 4_usize;
_5 = 118_u8 - 205_u8;
(*RET) = !_29;
(*_13) = !3989469137_u32;
_5 = 50_u8 << _7;
(*RET) = _29;
_2 = 1399250543671415309_u64 as u32;
_16 = _20 * _8;
_1 = !_29;
_22 = _1 == (*RET);
_10 = _5 as f64;
(*RET) = _22;
(*_13) = !3329760934_u32;
_17 = [_6,_6,_6];
_5 = 224_u8 ^ 239_u8;
_33 = _7 >> (*_13);
_16 = _20 >> _12;
(*RET) = _29;
_4 = 1_i8 * (-48_i8);
(*RET) = _20 > _12;
Goto(bb40)
}
bb40 = {
_21 = core::ptr::addr_of_mut!(_30);
_24 = !_3;
(*_13) = 2288011576_u32 + 2847320287_u32;
_28 = _14 | _14;
(*_13) = 597686344_u32;
(*_13) = (-8040760400535707110_i64) as u32;
_19 = (*RET) | (*RET);
Goto(bb41)
}
bb41 = {
(*_13) = !3598253611_u32;
(*_13) = 2910390592_u32 >> _33;
_27 = _23;
_16 = _12;
(*_13) = 873079415_u32;
(*RET) = _22 | _22;
_38.2 = 2724149613428761663_i64 as i32;
Goto(bb42)
}
bb42 = {
_38.1.2 = -_10;
(*RET) = !_22;
_23 = _27;
_39 = [14660175593856210880_u64,14721461262233940053_u64,14420765960862485240_u64,7292345971906608170_u64];
_38.1.0 = _27;
(*RET) = _22;
(*RET) = _29;
_29 = (*RET) | (*RET);
(*_13) = 1356581135_u32 | 1543186316_u32;
(*RET) = _19 >= _22;
(*RET) = !_22;
_38.1 = (_27, _6, _10, _27);
_38.1.3 = _27;
(*_13) = _23 as u32;
_33 = _7;
_22 = (*RET) == (*RET);
(*RET) = _16 <= _8;
_32 = core::ptr::addr_of_mut!(_15);
(*_32).2 = [_26,_12,_12,_8,_11];
(*_13) = 3847942192_u32;
(*RET) = _29 ^ _22;
Goto(bb43)
}
bb43 = {
_33 = _7 * _7;
_12 = _16 >> _33;
_38.3 = 3599379831314005509_i64;
_44 = _24 as f64;
_4 = _26 as i8;
_31 = &_10;
(*_32).2 = [_12,_16,_12,_26,_16];
_43 = _28 - _14;
_46 = (-24920_i16);
(*_32).2 = [_8,_12,_20,_26,_16];
_38.1.2 = (*_31) * (*_31);
match (*_13) {
0 => bb35,
1 => bb33,
2 => bb17,
3 => bb44,
3847942192 => bb46,
_ => bb45
}
}
bb44 = {
Return()
}
bb45 = {
_14 = 41157_u16 | 56581_u16;
_13 = core::ptr::addr_of!((*_13));
(*_13) = 2858452390_u32;
(*_13) = (*RET) as u32;
(*_13) = 3893405423_u32 + 3959127628_u32;
_16 = _12;
(*_13) = !862038173_u32;
_19 = (*_13) > (*_13);
(*RET) = !_19;
(*_13) = 596461385_u32 + 1691214935_u32;
(*_13) = !1701052958_u32;
(*_13) = 1030577393_u32;
(*RET) = (*_13) > (*_13);
(*RET) = _19;
_6 = -166321164_i32;
_11 = _16 >> (*_13);
_7 = 263239692233275948398618590702201455913_u128 * 81337909403118636604197580630241728895_u128;
RET = core::ptr::addr_of_mut!((*RET));
(*_13) = '\u{1fa35}' as u32;
_6 = !(-289047101_i32);
(*RET) = _19;
_20 = _16;
(*RET) = _12 > _16;
_20 = -_12;
(*_13) = _7 as u32;
_4 = 111_i8;
match _4 {
0 => bb17,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
111 => bb25,
_ => bb24
}
}
bb46 = {
(*RET) = _29;
(*_32).2 = [_12,_11,_12,_12,_16];
_33 = _7 >> _11;
(*RET) = _22 == _22;
_14 = _28;
(*_32).2 = [_12,_12,_11,_16,_12];
_15.2 = [_12,_16,_16,_12,_16];
_42 = (_43, Move(RET));
match (*_13) {
3847942192 => bb48,
_ => bb47
}
}
bb47 = {
match _5 {
0 => bb2,
1 => bb3,
31 => bb5,
_ => bb4
}
}
bb48 = {
_43 = _14;
_34 = _24 as f32;
_41 = _38.1.1 as isize;
(*_32).2 = [_26,_12,_11,_16,_8];
_2 = 896872516_u32;
match (*_13) {
0 => bb33,
1 => bb41,
2 => bb18,
3 => bb42,
4 => bb49,
896872516 => bb51,
_ => bb50
}
}
bb49 = {
_15.2 = [_11,_11,_8,_11,_12];
(*RET) = _7 <= _7;
_5 = _3 as u8;
_13 = core::ptr::addr_of!(_2);
(*RET) = true;
_13 = core::ptr::addr_of!(_2);
(*RET) = !false;
(*_13) = _10 as u32;
(*_13) = 509836161_u32 + 1771767718_u32;
(*RET) = (*_13) < (*_13);
(*_13) = 4203276715_u32;
(*RET) = false;
(*_13) = 1061975424_u32 + 3589357157_u32;
(*RET) = !false;
(*RET) = (*_13) < (*_13);
(*RET) = !false;
(*_13) = 3580933234_u32;
_6 = 3129023940396907823_i64 as i32;
_17 = [_6,_6,_6];
_1 = _5 > _5;
_3 = 4784254560967573180_usize;
(*RET) = true;
(*_13) = 997806223_u32 ^ 488499745_u32;
(*RET) = false ^ true;
(*RET) = !true;
(*_13) = 846879869_u32 * 2336844087_u32;
(*_13) = _3 as u32;
Goto(bb15)
}
bb50 = {
Return()
}
bb51 = {
(*_13) = !3462638838_u32;
_50 = -_26;
(*_13) = 1023376718_u32;
_14 = _42.0 ^ _28;
(*_32).2 = [_12,_26,_41,_26,_16];
_48 = _12;
RET = Move(_42.1);
_8 = -_12;
Goto(bb52)
}
bb52 = {
Call(_57 = dump_var(Move(_4), Move(_8), Move(_29), Move(_46)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_57 = dump_var(Move(_22), Move(_7), Move(_39), Move(_5)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_57 = dump_var(Move(_26), Move(_11), Move(_20), Move(_19)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_57 = dump_var(Move(_28), Move(_50), _58, _58), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: *mut bool,mut _2: u128) -> bool {
mir! {
type RET = bool;
let _3: ((*mut bool,), usize);
let _4: bool;
let _5: isize;
let _6: Adt18;
let _7: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _8: u64;
let _9: [u64; 3];
let _10: ((*mut bool,), usize);
let _11: [i32; 1];
let _12: i32;
let _13: ();
let _14: ();
{
_2 = 126545231435895598741562837432631637405_u128 - 205077221116923568740923637042399538149_u128;
_3.0.0 = Move(_1);
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = true & true;
(*_1) = _2 > _2;
_3.1 = !16150511323057667779_usize;
(*_1) = false;
_5 = (-42_isize) - (-9223372036854775808_isize);
_4 = (*_1) > (*_1);
(*_1) = _3.1 > _3.1;
(*_1) = _4 | _4;
_6 = Adt18::Variant1 { fld0: _2,fld1: _3.1,fld2: 38_u8,fld3: 98_i8 };
(*_1) = _4;
_3.1 = Field::<usize>(Variant(_6, 1), 1) | Field::<usize>(Variant(_6, 1), 1);
_2 = Field::<u128>(Variant(_6, 1), 0);
(*_1) = Field::<usize>(Variant(_6, 1), 1) >= _3.1;
(*_1) = !_4;
(*_1) = !_4;
(*_1) = !_4;
(*_1) = _2 >= _2;
Goto(bb1)
}
bb1 = {
place!(Field::<u8>(Variant(_6, 1), 2)) = 63_u8 + 191_u8;
place!(Field::<u8>(Variant(_6, 1), 2)) = 172_u8 >> _2;
(*_1) = _4;
_5 = 19_isize & 9223372036854775807_isize;
(*_1) = _4 <= _4;
_5 = (-9223372036854775808_isize) & 9223372036854775807_isize;
place!(Field::<i8>(Variant(_6, 1), 3)) = 50_i8 - 118_i8;
(*_1) = _4 ^ _4;
place!(Field::<u8>(Variant(_6, 1), 2)) = (*_1) as u8;
_2 = Field::<u128>(Variant(_6, 1), 0) + Field::<u128>(Variant(_6, 1), 0);
Goto(bb2)
}
bb2 = {
place!(Field::<usize>(Variant(_6, 1), 1)) = !_3.1;
(*_1) = !_4;
(*_1) = _4;
(*_1) = _2 >= _2;
(*_1) = _4 | _4;
(*_1) = Field::<usize>(Variant(_6, 1), 1) < _3.1;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _2 != Field::<u128>(Variant(_6, 1), 0);
_2 = !Field::<u128>(Variant(_6, 1), 0);
(*_1) = _3.1 < Field::<usize>(Variant(_6, 1), 1);
_10.1 = '\u{90617}' as usize;
Call((*_1) = fn2(Move(_3.0.0), Move(_1), _5, Field::<i8>(Variant(_6, 1), 3), Field::<u8>(Variant(_6, 1), 2), _6, _4, Field::<usize>(Variant(_6, 1), 1), _10.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = core::ptr::addr_of_mut!(_4);
(*_1) = RET <= RET;
(*_1) = !RET;
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(Move(_2), _14, _14, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: *mut bool,mut _2: *mut bool,mut _3: isize,mut _4: i8,mut _5: u8,mut _6: Adt18,mut _7: bool,mut _8: usize,mut _9: usize) -> bool {
mir! {
type RET = bool;
let _10: *mut &'static mut *const u32;
let _11: isize;
let _12: *mut bool;
let _13: &'static f64;
let _14: (Adt58, Adt20);
let _15: f32;
let _16: f64;
let _17: char;
let _18: ();
let _19: ();
{
place!(Field::<u8>(Variant(_6, 1), 2)) = _5 + _5;
_2 = core::ptr::addr_of_mut!(_7);
(*_2) = Field::<usize>(Variant(_6, 1), 1) >= _8;
place!(Field::<u128>(Variant(_6, 1), 0)) = 232024465999882423531654008418764841112_u128 * 301798300727615087970425823786115537457_u128;
(*_2) = Field::<u8>(Variant(_6, 1), 2) != Field::<u8>(Variant(_6, 1), 2);
_1 = core::ptr::addr_of_mut!((*_2));
Goto(bb1)
}
bb1 = {
place!(Field::<i8>(Variant(_6, 1), 3)) = _4 >> Field::<u128>(Variant(_6, 1), 0);
Goto(bb2)
}
bb2 = {
(*_1) = Field::<u8>(Variant(_6, 1), 2) <= Field::<u8>(Variant(_6, 1), 2);
(*_2) = false;
_1 = Move(_2);
place!(Field::<usize>(Variant(_6, 1), 1)) = !_9;
RET = _7 | _7;
_11 = _3 * _3;
place!(Field::<u8>(Variant(_6, 1), 2)) = _5;
_4 = Field::<i8>(Variant(_6, 1), 3) | Field::<i8>(Variant(_6, 1), 3);
place!(Field::<u8>(Variant(_6, 1), 2)) = _5 & _5;
_4 = Field::<i8>(Variant(_6, 1), 3) * Field::<i8>(Variant(_6, 1), 3);
place!(Field::<u128>(Variant(_6, 1), 0)) = 217873409914643194946892264300432394875_u128;
_4 = Field::<i8>(Variant(_6, 1), 3) & Field::<i8>(Variant(_6, 1), 3);
_11 = _3 ^ _3;
_1 = core::ptr::addr_of_mut!(RET);
place!(Field::<usize>(Variant(_6, 1), 1)) = _8;
(*_1) = _8 == _8;
(*_1) = !_7;
_4 = 6046469864732075295_u64 as i8;
match Field::<u128>(Variant(_6, 1), 0) {
0 => bb1,
1 => bb3,
217873409914643194946892264300432394875 => bb5,
_ => bb4
}
}
bb3 = {
place!(Field::<i8>(Variant(_6, 1), 3)) = _4 >> Field::<u128>(Variant(_6, 1), 0);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*_1) = !_7;
_8 = !Field::<usize>(Variant(_6, 1), 1);
_8 = Field::<usize>(Variant(_6, 1), 1) - _9;
(*_1) = _8 <= _8;
(*_1) = !_7;
(*_1) = !_7;
_6 = Adt18::Variant1 { fld0: 48569577928608269401239907495758482861_u128,fld1: _8,fld2: _5,fld3: _4 };
(*_1) = _5 >= Field::<u8>(Variant(_6, 1), 2);
(*_1) = _5 <= _5;
(*_1) = _7 ^ _7;
_7 = (*_1) ^ (*_1);
(*_1) = _7 & _7;
(*_1) = _11 > _11;
(*_1) = _11 < _11;
(*_1) = _7;
Call((*_1) = fn3(_7, Move(_1), Field::<i8>(Variant(_6, 1), 3), _4, _9, _9, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = RET >= RET;
_12 = core::ptr::addr_of_mut!(_7);
(*_12) = !RET;
_3 = _11 | _11;
(*_12) = Field::<i8>(Variant(_6, 1), 3) == Field::<i8>(Variant(_6, 1), 3);
_3 = _11 - _11;
_2 = core::ptr::addr_of_mut!((*_12));
(*_2) = !RET;
(*_12) = RET < RET;
_7 = _5 < _5;
(*_12) = !RET;
_8 = _9 ^ Field::<usize>(Variant(_6, 1), 1);
_9 = Field::<usize>(Variant(_6, 1), 1) * _8;
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = RET == RET;
(*_12) = Field::<u8>(Variant(_6, 1), 2) == _5;
(*_12) = RET;
Goto(bb7)
}
bb7 = {
Call(_18 = dump_var(Move(_11), Move(_4), Move(_3), _19), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: *mut bool,mut _3: i8,mut _4: i8,mut _5: usize,mut _6: usize,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: &'static u128;
let _9: *const i16;
let _10: f64;
let _11: f64;
let _12: (char, i32, f64, char);
let _13: isize;
let _14: *mut [i128; 5];
let _15: ();
let _16: ();
{
_2 = core::ptr::addr_of_mut!(_1);
Call((*_2) = fn4(Move(_2), _3, _4, _3, _7, _3, _7, _4, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _5 * _5;
_4 = _3;
_10 = 290622776175854593370454534896437831942_u128 as f64;
RET = _1;
_3 = !_4;
RET = _1 > _1;
_7 = _1 | _1;
_2 = core::ptr::addr_of_mut!(RET);
RET = _7 != _7;
(*_2) = !_1;
(*_2) = _7 < _1;
(*_2) = _1;
_12.1 = -660093499_i32;
(*_2) = _7 | _7;
_12.0 = '\u{73343}';
_5 = _6 + _6;
_13 = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_2) = _1 | _1;
(*_2) = _7;
(*_2) = _7 | _1;
(*_2) = _1 < _7;
_2 = core::ptr::addr_of_mut!((*_2));
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(Move(_4), Move(_3), Move(_1), _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: *mut bool,mut _2: i8,mut _3: i8,mut _4: i8,mut _5: bool,mut _6: i8,mut _7: bool,mut _8: i8,mut _9: i8,mut _10: i8) -> bool {
mir! {
type RET = bool;
let _11: f32;
let _12: &'static mut &'static u128;
let _13: bool;
let _14: *const [i64; 6];
let _15: f64;
let _16: ();
let _17: ();
{
_8 = _10 ^ _4;
_10 = (-116514689325093251128583263659379332373_i128) as i8;
_10 = _8 | _8;
_10 = (-750774249_i32) as i8;
_5 = _6 >= _9;
Call(_2 = fn5(Move(_1), _5, _6, _8, _10, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _2 < _2;
_9 = _4;
_6 = !_2;
RET = _7 <= _7;
_7 = !RET;
_9 = (-2724_i16) as i8;
_8 = 3057341158_u32 as i8;
_6 = -_2;
_11 = 1746698970_u32 as f32;
_13 = !_7;
_6 = _2;
_2 = _6;
_8 = _6;
_2 = 920197059843244896_u64 as i8;
_10 = 39518_u16 as i8;
_7 = RET;
_8 = 0_usize as i8;
_1 = core::ptr::addr_of_mut!(_7);
(*_1) = _13 | RET;
(*_1) = RET & RET;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(Move(_3), Move(_9), Move(_7), Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_16 = dump_var(Move(_4), _17, _17, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut bool,mut _2: bool,mut _3: i8,mut _4: i8,mut _5: i8,mut _6: i8) -> i8 {
mir! {
type RET = i8;
let _7: &'static mut &'static u128;
let _8: [i32; 1];
let _9: &'static f64;
let _10: *mut &'static Adt18;
let _11: u16;
let _12: &'static i16;
let _13: *mut *mut &'static Adt18;
let _14: f32;
let _15: (*mut bool,);
let _16: isize;
let _17: (*const &'static f64, char);
let _18: char;
let _19: isize;
let _20: *const u32;
let _21: u8;
let _22: f64;
let _23: isize;
let _24: isize;
let _25: bool;
let _26: isize;
let _27: Adt18;
let _28: isize;
let _29: [usize; 7];
let _30: ();
let _31: ();
{
_5 = 1460289118_i32 as i8;
_3 = _4 | _5;
_3 = !_5;
RET = _5 ^ _5;
_6 = 4104379716857206550_u64 as i8;
_6 = _4;
RET = 515011360_u32 as i8;
_6 = -RET;
_6 = _4 | _4;
RET = _6;
_5 = RET | _3;
_4 = !_6;
_5 = 14463525312335176169_u64 as i8;
_2 = false;
_8 = [(-1520203481_i32)];
_1 = core::ptr::addr_of_mut!(_2);
(*_1) = !false;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = true ^ true;
Call(_2 = fn6(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_1) = _6 > _5;
(*_1) = true;
(*_1) = _5 < RET;
(*_1) = false & true;
(*_1) = RET > _6;
_11 = 39048_u16;
_8 = [(-326065781_i32)];
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
39048 => bb6,
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
_11 = 68_u8 as u16;
(*_1) = false;
(*_1) = _4 != _6;
(*_1) = RET >= _4;
(*_1) = _11 > _11;
(*_1) = false | true;
(*_1) = _11 > _11;
(*_1) = true;
(*_1) = true;
_8 = [1691201850_i32];
(*_1) = false;
_16 = -(-1_isize);
_16 = (-14_isize) >> _6;
_3 = _4 >> _4;
(*_1) = !false;
(*_1) = true | false;
_15 = (Move(_1),);
_16 = '\u{b8391}' as isize;
Goto(bb7)
}
bb7 = {
_14 = 1405888158_u32 as f32;
_3 = _6 + _6;
_2 = !true;
Goto(bb8)
}
bb8 = {
_13 = core::ptr::addr_of_mut!(_10);
_6 = _3 + _4;
_15.0 = core::ptr::addr_of_mut!(_2);
_11 = 24427_u16 - 49371_u16;
_17.1 = '\u{5bd73}';
_4 = -_3;
_2 = _11 == _11;
_8 = [1277833012_i32];
_8 = [(-1309814893_i32)];
_13 = core::ptr::addr_of_mut!((*_13));
_5 = -_6;
Goto(bb9)
}
bb9 = {
_18 = _17.1;
RET = _6 - _6;
_19 = _16;
_5 = _6 << RET;
_19 = -_16;
_3 = -RET;
_22 = _19 as f64;
_5 = _3 + RET;
_3 = 1_usize as i8;
_1 = core::ptr::addr_of_mut!(_2);
_4 = 5822074951555995070_u64 as i8;
_24 = _19 >> _5;
(*_1) = !true;
(*_1) = true | true;
_11 = (-85536192733439208181164603428491788672_i128) as u16;
(*_1) = _5 < _5;
_19 = _24 - _24;
_15.0 = core::ptr::addr_of_mut!((*_1));
_19 = _24 << _24;
_23 = _19 * _19;
Goto(bb10)
}
bb10 = {
Call(_30 = dump_var(Move(_3), Move(_6), Move(_2), Move(_23)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_30 = dump_var(Move(_5), Move(_19), _31, _31), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6() -> bool {
mir! {
type RET = bool;
let _1: &'static mut &'static u128;
let _2: i64;
let _3: &'static i8;
let _4: *const i16;
let _5: i128;
let _6: f32;
let _7: *mut *const [i64; 6];
let _8: [i64; 6];
let _9: i64;
let _10: [i64; 6];
let _11: i32;
let _12: bool;
let _13: isize;
let _14: *mut &'static mut *const u32;
let _15: char;
let _16: bool;
let _17: isize;
let _18: *const [i64; 6];
let _19: [usize; 4];
let _20: &'static isize;
let _21: (i16, (char, i32, f64, char), i32, i64);
let _22: *mut *mut &'static Adt18;
let _23: f32;
let _24: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _25: &'static mut &'static u128;
let _26: char;
let _27: Adt41;
let _28: usize;
let _29: [i128; 5];
let _30: i128;
let _31: isize;
let _32: &'static mut usize;
let _33: Adt18;
let _34: i16;
let _35: *mut *const f32;
let _36: &'static Adt51;
let _37: char;
let _38: &'static i16;
let _39: *mut *const [i64; 6];
let _40: *mut [u32; 7];
let _41: *mut *const [i64; 6];
let _42: bool;
let _43: f32;
let _44: &'static mut u32;
let _45: &'static Adt51;
let _46: Adt31;
let _47: [u64; 3];
let _48: usize;
let _49: i16;
let _50: char;
let _51: &'static f64;
let _52: [char; 6];
let _53: &'static mut usize;
let _54: &'static mut *const u32;
let _55: bool;
let _56: *mut bool;
let _57: [u32; 7];
let _58: *const [i64; 6];
let _59: (u16, *mut bool);
let _60: f64;
let _61: *mut *mut &'static Adt18;
let _62: u64;
let _63: *mut Adt18;
let _64: &'static Adt51;
let _65: ();
let _66: ();
{
RET = false ^ false;
RET = false ^ true;
RET = !false;
RET = (-24189601606524901744986203387375157977_i128) != (-102226888484350346894926816996298274585_i128);
RET = 3180034268527706828_u64 != 17161159453730686404_u64;
RET = false | true;
RET = !false;
RET = !false;
RET = true;
RET = !true;
Goto(bb1)
}
bb1 = {
RET = true;
RET = false;
RET = !true;
_2 = 6584285563371434248_i64;
_2 = -(-8795764487066980347_i64);
_2 = 8132162272786267833_i64;
RET = _2 != _2;
_2 = -(-9005568026748573092_i64);
_2 = (-1636258870185655892_i64);
RET = !false;
RET = !true;
_2 = (-1964548125884282076_i64);
_2 = 5779626913233718414_i64 & (-442052909168490681_i64);
RET = _2 == _2;
RET = _2 == _2;
RET = !false;
Goto(bb2)
}
bb2 = {
RET = _2 > _2;
_2 = !8080934420064017429_i64;
_2 = (-6342791374067221810_i64) << 106_u8;
_2 = 5921482785399625412_i64 >> 126_i8;
_2 = 2872769338643053952_i64;
_2 = -(-963009019400336819_i64);
RET = !true;
_2 = 4605962337924327848_i64;
_2 = (-2163501687290304040_i64) | (-8141728023564454001_i64);
_5 = (-23621866157063986153552867885592997122_i128) * 25524550747354729068632700384993412723_i128;
_5 = (-106038018476895633184569307793634184158_i128);
_2 = (-848359239_i32) as i64;
RET = false | false;
_6 = 111_u8 as f32;
_6 = (-10268_i16) as f32;
RET = _2 == _2;
_6 = 52365_u16 as f32;
_2 = 3199702147326945868_i64 >> _5;
RET = true;
_5 = -88925851116567992453925520588910173905_i128;
_6 = _2 as f32;
_8 = [_2,_2,_2,_2,_2,_2];
_2 = (-7999422722689512600_i64) - (-9209373950841026367_i64);
RET = !true;
Goto(bb3)
}
bb3 = {
_6 = _5 as f32;
RET = _5 != _5;
_5 = (-99634386741805083145117017171504787435_i128) << _2;
RET = _2 < _2;
_10 = _8;
Call(_9 = core::intrinsics::bswap(_2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = 47914810844751185416511975942401839883_i128 ^ 62942917733260018042535253747120356982_i128;
_5 = (-145185141251450030931214957491499229570_i128) ^ (-20818822925270617064550423555479542467_i128);
_11 = (-1538579154_i32) & 243355013_i32;
RET = false;
_10 = [_2,_2,_2,_2,_2,_2];
_11 = 2136761183_i32;
_5 = !51938509517256328088103103852478320934_i128;
_9 = -_2;
_2 = _9;
_6 = 27814_u16 as f32;
_8 = _10;
_10 = _8;
_6 = (-9223372036854775808_isize) as f32;
_2 = _9 ^ _9;
_11 = (-9223372036854775808_isize) as i32;
_10 = _8;
_12 = !RET;
_10 = [_2,_2,_2,_2,_2,_9];
RET = _2 <= _2;
_15 = '\u{ba8ba}';
_13 = _9 as isize;
RET = _15 <= _15;
_16 = _12;
_10 = [_9,_2,_9,_9,_9,_9];
_11 = (-1598478241_i32) << _2;
Call(_9 = fn7(_6, _6, _10, _6, _16, _2, _8, _15, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = _2;
_8 = [_9,_2,_9,_2,_2,_2];
_12 = _2 >= _2;
_13 = 9223372036854775807_isize * 117_isize;
_15 = '\u{bfea8}';
_16 = !_12;
_17 = _13 | _13;
_19 = [6_usize,12029774816151108371_usize,9765592147860157287_usize,1_usize];
_8 = [_2,_2,_9,_2,_2,_2];
_5 = (-21606866757459727846928385018813287785_i128) & (-160918151573445564602503879085084962526_i128);
_2 = _9 << _9;
_13 = _17;
_15 = '\u{c54e4}';
_7 = core::ptr::addr_of_mut!(_18);
(*_7) = core::ptr::addr_of!(_8);
(*_18) = [_2,_2,_2,_2,_2,_9];
(*_18) = _10;
(*_18) = [_2,_2,_2,_2,_2,_2];
_13 = _17;
(*_7) = core::ptr::addr_of!((*_18));
(*_18) = [_2,_9,_2,_2,_2,_9];
(*_7) = core::ptr::addr_of!((*_18));
Goto(bb6)
}
bb6 = {
_8 = [_2,_2,_2,_2,_9,_2];
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
(*_18) = [_2,_2,_2,_9,_9,_2];
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
(*_18) = [_2,_9,_2,_9,_9,_2];
(*_18) = _10;
(*_7) = core::ptr::addr_of!((*_18));
_17 = _13 | _13;
(*_18) = _10;
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!(_8);
(*_18) = [_2,_2,_2,_9,_2,_2];
(*_7) = core::ptr::addr_of!((*_18));
_2 = _9;
_21.0 = 14424_i16 - 14487_i16;
(*_18) = [_9,_2,_9,_9,_9,_9];
(*_18) = _10;
_21.1.1 = _11;
_7 = core::ptr::addr_of_mut!((*_7));
_19 = [12342606372658339167_usize,1_usize,8407922732020285591_usize,9278552621196950851_usize];
(*_18) = _10;
_18 = core::ptr::addr_of!((*_18));
_20 = &_17;
Goto(bb7)
}
bb7 = {
(*_18) = [_9,_2,_2,_9,_9,_9];
(*_18) = [_2,_2,_2,_2,_9,_9];
(*_7) = core::ptr::addr_of!(_10);
_21.1.0 = _15;
(*_7) = core::ptr::addr_of!((*_18));
(*_18) = _8;
(*_18) = [_9,_2,_2,_2,_2,_2];
(*_18) = [_2,_2,_9,_9,_2,_9];
_20 = &_13;
_21.3 = -_9;
_9 = _6 as i64;
_26 = _15;
(*_7) = core::ptr::addr_of!((*_18));
_17 = (*_20) << _21.1.1;
(*_18) = _8;
(*_18) = _8;
_26 = _15;
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
_21.1.0 = _26;
(*_7) = core::ptr::addr_of!((*_18));
_21.1.0 = _15;
(*_7) = core::ptr::addr_of!((*_18));
Goto(bb8)
}
bb8 = {
_16 = _11 < _21.1.1;
_9 = _2 - _2;
_8 = [_2,_2,_2,_2,_9,_9];
(*_18) = [_9,_9,_9,_21.3,_2,_21.3];
(*_7) = core::ptr::addr_of!((*_18));
_21.1.0 = _15;
Goto(bb9)
}
bb9 = {
_21.1.3 = _15;
(*_7) = core::ptr::addr_of!((*_18));
_21.0 = (-23125_i16) & 28953_i16;
(*_18) = _8;
(*_18) = [_2,_2,_21.3,_2,_9,_21.3];
_16 = (*_20) != (*_20);
_4 = core::ptr::addr_of!(_21.0);
(*_7) = core::ptr::addr_of!((*_18));
_23 = 100_u8 as f32;
(*_7) = core::ptr::addr_of!((*_18));
_21.1.2 = _5 as f64;
(*_7) = core::ptr::addr_of!((*_18));
(*_18) = _8;
_29 = [_5,_5,_5,_5,_5];
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = 26102_i16;
Goto(bb10)
}
bb10 = {
(*_18) = [_2,_2,_9,_21.3,_21.3,_21.3];
(*_18) = _8;
_21.3 = 30773_u16 as i64;
_26 = _21.1.0;
(*_4) = _15 as i16;
_13 = _17 * _17;
_18 = core::ptr::addr_of!(_10);
(*_4) = _5 as i16;
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = (-245_i16) >> _9;
(*_4) = 27739_i16 * 2580_i16;
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = _23 as i16;
(*_18) = _8;
_31 = _13;
(*_7) = core::ptr::addr_of!(_8);
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = (-26862_i16) << _2;
(*_4) = (-3942_i16) ^ (-20060_i16);
Goto(bb11)
}
bb11 = {
(*_4) = 1612_i16;
(*_18) = [_2,_9,_9,_9,_2,_2];
(*_18) = _10;
_21.2 = -_11;
_21.0 = 19519_i16 & (-9882_i16);
_21.1.0 = _21.1.3;
(*_7) = core::ptr::addr_of!((*_18));
_20 = &_17;
(*_18) = [_2,_9,_9,_9,_2,_9];
(*_18) = [_9,_9,_9,_9,_9,_2];
(*_18) = [_9,_21.3,_9,_9,_9,_2];
_39 = core::ptr::addr_of_mut!((*_7));
(*_18) = [_9,_9,_9,_21.3,_9,_9];
_2 = _9 >> (*_20);
_2 = _9;
(*_39) = core::ptr::addr_of!((*_18));
(*_4) = (-619_i16);
(*_39) = core::ptr::addr_of!((*_18));
Goto(bb12)
}
bb12 = {
(*_7) = core::ptr::addr_of!((*_18));
Goto(bb13)
}
bb13 = {
(*_18) = _10;
(*_18) = _10;
(*_4) = !(-17892_i16);
(*_18) = _10;
_17 = _31 ^ _13;
(*_7) = core::ptr::addr_of!((*_18));
_28 = _11 as usize;
(*_4) = 11724_i16 << _17;
Goto(bb14)
}
bb14 = {
(*_18) = _10;
(*_18) = [_9,_2,_9,_2,_9,_2];
_11 = _21.2;
(*_7) = core::ptr::addr_of!((*_18));
Goto(bb15)
}
bb15 = {
(*_7) = core::ptr::addr_of!((*_18));
_19 = [_28,_28,_28,_28];
(*_4) = (-18585_i16);
(*_4) = (-28142_i16);
(*_18) = [_9,_9,_9,_2,_9,_2];
_5 = (-74237719661678695363271204505511135965_i128) << _17;
_34 = !(*_4);
_49 = -(*_4);
(*_4) = !_34;
_21.1.0 = _26;
(*_4) = -_34;
_10 = [_2,_9,_9,_2,_2,_2];
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = _34 >> _17;
(*_4) = _34 << _11;
(*_18) = [_2,_9,_2,_9,_9,_9];
_52 = [_21.1.3,_21.1.3,_21.1.0,_21.1.3,_21.1.3,_15];
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = _34 & _34;
(*_18) = [_9,_2,_2,_2,_9,_2];
(*_18) = [_9,_9,_9,_9,_9,_9];
(*_4) = !_49;
Goto(bb16)
}
bb16 = {
(*_4) = _34;
_21.3 = _9 & _2;
_42 = _16 & _16;
_30 = _2 as i128;
Goto(bb17)
}
bb17 = {
_29 = [_5,_5,_5,_5,_5];
(*_4) = _34 * _34;
_21.2 = -_21.1.1;
(*_4) = -_34;
(*_4) = _49;
(*_7) = core::ptr::addr_of!((*_18));
(*_4) = _34;
_32 = &mut _28;
(*_18) = [_21.3,_21.3,_21.3,_9,_21.3,_2];
(*_32) = 52888_u16 as usize;
(*_4) = _34 & _34;
Goto(bb18)
}
bb18 = {
(*_18) = [_21.3,_21.3,_21.3,_2,_9,_21.3];
(*_4) = _49 ^ _49;
(*_4) = 3615378330_u32 as i16;
_50 = _21.1.3;
_8 = _10;
_6 = -_23;
_53 = &mut (*_32);
(*_4) = _34 ^ _34;
(*_7) = core::ptr::addr_of!((*_18));
_21.0 = !_34;
_10 = (*_18);
_21.1.2 = _5 as f64;
_13 = _17 & _31;
_13 = _21.1.2 as isize;
Goto(bb19)
}
bb19 = {
(*_7) = core::ptr::addr_of!((*_18));
Call(_59.0 = core::intrinsics::bswap(47246_u16), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
(*_18) = [_21.3,_21.3,_21.3,_2,_9,_21.3];
RET = _42 == _16;
_33 = Adt18::Variant1 { fld0: 186523432991909271403014161828084146916_u128,fld1: (*_53),fld2: 120_u8,fld3: 39_i8 };
(*_7) = core::ptr::addr_of!((*_18));
(*_7) = core::ptr::addr_of!(_8);
(*_53) = !Field::<usize>(Variant(_33, 1), 1);
_55 = RET | _42;
_56 = core::ptr::addr_of_mut!(RET);
_51 = &_21.1.2;
_31 = _17;
_5 = 3366794455_u32 as i128;
_21.1.1 = _21.2 << (*_53);
(*_4) = -_49;
_12 = (*_56) ^ (*_56);
_26 = _21.1.0;
(*_53) = Field::<usize>(Variant(_33, 1), 1) & Field::<usize>(Variant(_33, 1), 1);
Goto(bb21)
}
bb21 = {
Call(_65 = dump_var(Move(_10), Move(_17), Move(_13), Move(_5)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_65 = dump_var(Move(_34), Move(_9), Move(_26), Move(_42)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_65 = dump_var(Move(_55), Move(_15), Move(_29), _66), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: f32,mut _2: f32,mut _3: [i64; 6],mut _4: f32,mut _5: bool,mut _6: i64,mut _7: [i64; 6],mut _8: char,mut _9: isize) -> i64 {
mir! {
type RET = i64;
let _10: (Adt58,);
let _11: [i32; 3];
let _12: i64;
let _13: Adt78;
let _14: &'static f64;
let _15: f32;
let _16: *const &'static f64;
let _17: f32;
let _18: *mut [usize; 4];
let _19: usize;
let _20: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _21: ();
let _22: ();
{
_8 = '\u{2c0d8}';
_7 = [_6,_6,_6,_6,_6,_6];
_5 = _6 < _6;
_9 = !2_isize;
RET = -_6;
_2 = _1;
RET = !_6;
Goto(bb1)
}
bb1 = {
_1 = -_4;
_8 = '\u{76a07}';
_11 = [(-490298579_i32),(-1915302498_i32),(-1909080963_i32)];
_11 = [(-1319895911_i32),723662021_i32,1353030755_i32];
Goto(bb2)
}
bb2 = {
_1 = -_2;
_7 = _3;
_5 = _6 != _6;
Call(_10 = fn8(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [3436791577_u32,2355798085_u32,3322500880_u32,648291906_u32,2401106208_u32,1434334254_u32];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = (-14450_i16) - 19208_i16;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = 13720_i16;
_4 = _1 + _1;
_8 = '\u{45873}';
_3 = [_6,_6,RET,_6,RET,RET];
_9 = !(-9223372036854775808_isize);
_11 = [(-1457551648_i32),1144327155_i32,969311134_i32];
_6 = RET ^ RET;
_12 = 17200232995603338804_usize as i64;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.0 = 13_u8;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.0 = 826646939536211479_u64 as u8;
_9 = 20_isize | (-9223372036854775808_isize);
_2 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).3.2 as f32;
_7 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).2;
_6 = -RET;
RET = !_6;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.1 = core::ptr::addr_of!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [2963989615_u32,840694866_u32,427562837_u32,2895834978_u32,736673827_u32,3927183902_u32];
Goto(bb4)
}
bb4 = {
_12 = _6 + _6;
_3 = [RET,_12,RET,_12,_12,RET];
_4 = _1 - _1;
_9 = 49_isize;
_11 = [(-660884569_i32),(-135778668_i32),216808186_i32];
_5 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).1;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = (-73_i16) - 12319_i16;
_5 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).1;
_7 = [RET,_12,_12,_12,_6,_6];
_7 = _3;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).1 = _5;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = 22906_i16;
_1 = _2 * _4;
_5 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).1 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).1;
_12 = !_6;
_4 = -_1;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = _9 as i16;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.0 = 3697180990_u32 as u8;
_7 = [_12,_6,_12,RET,_12,_6];
_7 = [_12,_12,RET,_12,RET,_6];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = _9 as i16;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [1740730775_u32,4276176321_u32,2103544337_u32,3951868940_u32,4001902051_u32,3326505475_u32];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.1 = core::ptr::addr_of!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [2437965543_u32,1283327070_u32,2608574587_u32,2222103925_u32,840324278_u32,2958989099_u32];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.1 = core::ptr::addr_of!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = !19597_i16;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).2 = [RET,_6,_6,_12,RET,_6];
_9 = 3322594083773974643_u64 as isize;
_7 = [RET,RET,RET,_6,_6,_6];
_2 = _4 + _4;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).1 = _5 & _5;
Goto(bb5)
}
bb5 = {
_12 = RET;
RET = _6 * _6;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.0 = 30982_u16 as u8;
_8 = '\u{841c6}';
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [1234117610_u32,3980055958_u32,4094815051_u32,3573006835_u32,1763567015_u32,1444462331_u32];
_7 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).2;
_3 = [RET,RET,RET,RET,RET,RET];
_15 = -_2;
_11 = [(-1001093062_i32),2042144267_i32,1588440902_i32];
_12 = RET & RET;
Call(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.1 = fn10(_2, _9, _9, Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).1, _11, _2, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).2 = [_12,RET,_12,_12,_12,_12];
_2 = _15 - _15;
_1 = _15 - _2;
_15 = -_1;
_12 = RET - RET;
_15 = -_1;
_15 = _1 + _2;
_6 = _12 ^ RET;
_7 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0).2;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).2 = [_6,_6,RET,_12,RET,_12];
_3 = [_6,_6,_6,RET,_12,_12];
_17 = _1 + _15;
_2 = 840773247_i32 as f32;
Goto(bb7)
}
bb7 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = 22178_i16;
_8 = '\u{47170}';
_1 = 12235_u16 as f32;
_11 = [(-1617644393_i32),140624694_i32,(-1299082609_i32)];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).1 = _5 ^ _5;
_9 = (-96_isize) & (-9223372036854775808_isize);
_7 = [_6,_6,_12,_6,RET,_6];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).2 = [RET,_6,_6,_12,_6,_6];
RET = _6 - _6;
_7 = [RET,RET,RET,_6,RET,_12];
_15 = _17;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.1 = core::ptr::addr_of!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2);
_12 = -RET;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).3.2 = 6102_i16 ^ (-7785_i16);
_8 = '\u{df8e2}';
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).2 = [RET,RET,_12,RET,_6,RET];
_4 = -_17;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_10.0, 1), 0)).0 = [1495366640_u32,2878910115_u32,408213248_u32,2494825506_u32,1556886877_u32,41465328_u32];
Goto(bb8)
}
bb8 = {
Call(_21 = dump_var(Move(_3), Move(_5), Move(_8), Move(_11)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize) -> (Adt58,) {
mir! {
type RET = (Adt58,);
let _2: u64;
let _3: *mut *const [i64; 6];
let _4: *mut [usize; 4];
let _5: f32;
let _6: bool;
let _7: u8;
let _8: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _9: usize;
let _10: &'static isize;
let _11: [u32; 7];
let _12: &'static i16;
let _13: isize;
let _14: char;
let _15: f64;
let _16: u128;
let _17: [u64; 4];
let _18: i128;
let _19: f64;
let _20: isize;
let _21: isize;
let _22: char;
let _23: f64;
let _24: (u8, *const i16, i16);
let _25: u16;
let _26: f64;
let _27: usize;
let _28: i64;
let _29: *const i16;
let _30: u32;
let _31: char;
let _32: *mut *mut &'static Adt18;
let _33: [i64; 6];
let _34: isize;
let _35: *mut [u32; 7];
let _36: isize;
let _37: &'static mut &'static u128;
let _38: f32;
let _39: (*const &'static f64, char);
let _40: [i128; 5];
let _41: ();
let _42: ();
{
_1 = (-9223372036854775808_isize) ^ 23_isize;
_1 = 9223372036854775807_isize * (-9223372036854775808_isize);
_1 = 9223372036854775807_isize;
_1 = 9223372036854775807_isize + (-78_isize);
_2 = 18101354967854599107_u64 << _1;
_2 = 3977387589850484580_u64;
_2 = 2212411629265883885_u64 ^ 2198177986663068921_u64;
_2 = 4300255032112597436_u64;
_1 = -(-111_isize);
_1 = !(-9_isize);
_1 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_1 = !(-71_isize);
_1 = 57_u8 as isize;
_2 = 15919044661897952463_u64 * 9633876396092797580_u64;
_2 = !13162784961970375306_u64;
_2 = (-75_i8) as u64;
_1 = 9223372036854775807_isize | (-9223372036854775808_isize);
_2 = 14151110510930366616_u64;
_2 = 9053634956367059332_u64;
_1 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_1 = (-69_isize) | (-9223372036854775808_isize);
_1 = 9223372036854775807_isize << _2;
_1 = (-1_isize) + (-9223372036854775808_isize);
_2 = 12189363280449449377_u64 >> _1;
Goto(bb1)
}
bb1 = {
_1 = (-84_isize);
_2 = 60035936861195188_u64 & 12159094852639082349_u64;
_1 = -(-128_isize);
_2 = 6457910811866055184_u64 >> _1;
_2 = 4494194705979993114_u64 | 1391781045045503225_u64;
_2 = 4692733826943304286_u64 ^ 10026127316465874332_u64;
_1 = (-99_isize);
_5 = 110446974560099386493333767087586945044_i128 as f32;
_2 = 98_u8 as u64;
_1 = 9223372036854775807_isize;
_5 = (-31138_i16) as f32;
_2 = 371319968497266700_u64;
_1 = (-9223372036854775808_isize);
_6 = false ^ false;
_7 = !219_u8;
_2 = !8664601956867444716_u64;
Goto(bb2)
}
bb2 = {
_2 = !350926018750660646_u64;
_5 = (-3322828253418579867_i64) as f32;
_5 = 5_usize as f32;
Goto(bb3)
}
bb3 = {
_2 = !5243612152167118474_u64;
_8.0 = [2509023886_u32,910751181_u32,2922829315_u32,1000396090_u32,1749487962_u32,1497598764_u32];
_8.1 = !_6;
_8.3.0 = _7 << _7;
_8.0 = [3488927693_u32,2934078202_u32,746893326_u32,3412817675_u32,1170882492_u32,128481017_u32];
_7 = _8.3.0;
_8.2 = [(-6094169962221256934_i64),2987756448610059159_i64,(-4567923157105135894_i64),9142295467754125785_i64,(-238448383715892815_i64),(-7536443057024930046_i64)];
_8.0 = [3690397355_u32,428134934_u32,332573951_u32,708862741_u32,157739839_u32,3707785022_u32];
_7 = (-8565_i16) as u8;
_8.3.2 = 20530_i16;
_7 = _8.3.0;
_1 = 9223372036854775807_isize & 118_isize;
_10 = &_1;
_1 = !(-9223372036854775808_isize);
_6 = _8.1;
_8.3.1 = core::ptr::addr_of!(_8.3.2);
_1 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_14 = '\u{d6c39}';
_6 = !_8.1;
_1 = 9223372036854775807_isize >> _8.3.2;
_9 = 55882_u16 as usize;
_11 = [2696362476_u32,3939260564_u32,1667857186_u32,2821501917_u32,1968735787_u32,794658260_u32,541094845_u32];
_6 = _1 > _1;
_15 = 3462280307_u32 as f64;
_13 = 3223314167_u32 as isize;
Goto(bb4)
}
bb4 = {
_8.0 = [4120912576_u32,2431343628_u32,3772557817_u32,915298270_u32,3230864461_u32,3235973897_u32];
_2 = 7394599470711816646_u64 & 225938578056239764_u64;
_5 = _13 as f32;
_8.2 = [(-9009741995457392965_i64),(-9175660760349064194_i64),(-7016859825946641675_i64),5086175953622946588_i64,(-173072984600686579_i64),(-9077480891556810868_i64)];
_8.3.0 = _8.3.2 as u8;
_2 = 10738219281132572876_u64 * 5069697708459775328_u64;
_11 = [435140038_u32,67205562_u32,967560770_u32,186354284_u32,4081604102_u32,4132298026_u32,975530334_u32];
_8.0 = [583648589_u32,1761377241_u32,452269744_u32,1918651654_u32,3378115444_u32,3754464064_u32];
_8.3.0 = _7;
_8.3.0 = _7 >> _2;
_5 = _9 as f32;
_6 = !_8.1;
_9 = 57201_u16 as usize;
_8.3.1 = core::ptr::addr_of!(_8.3.2);
Goto(bb5)
}
bb5 = {
_19 = _15 - _15;
_1 = _13 - _13;
RET.0 = Adt58::Variant1 { fld0: Move(_8) };
_23 = _19;
_24 = (_7, Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.1), Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.2);
_8.0 = [1346184976_u32,29663915_u32,1466377693_u32,2898909540_u32,1694288122_u32,4280536304_u32];
Goto(bb6)
}
bb6 = {
_17 = [_2,_2,_2,_2];
_20 = _13;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.0 = _14 as u8;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3 = (_24.0, Move(_24.1), _24.2);
_8 = Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0));
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).2 = _8.2;
_5 = 1826225767_i32 as f32;
_16 = 262634313026031033536154994840958015330_u128;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.1 = core::ptr::addr_of!(_8.3.2);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).1 = _23 <= _19;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.2 = _24.2;
_22 = _14;
_24.2 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.2 + Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.2;
_12 = &_24.2;
_21 = _13 - _20;
_23 = _19 + _19;
_24.1 = core::ptr::addr_of!((*_12));
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)) = (_8.0, _8.1, _8.2, Move(_8.3));
_20 = -_1;
_25 = 42337_u16 * 33763_u16;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.1 = Move(_24.1);
_23 = (*_12) as f64;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.1 = core::ptr::addr_of!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.2);
_8.3.2 = (*_12);
_24.0 = _7 | _7;
_26 = _2 as f64;
_19 = _20 as f64;
Goto(bb7)
}
bb7 = {
_16 = !276919573114639176319833095866434896071_u128;
_5 = _2 as f32;
_24.1 = Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.1);
_8.3.0 = _7 + _24.0;
_9 = 2_usize;
Call(_8.3.2 = core::intrinsics::transmute((*_12)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = _1 - _21;
_13 = _8.1 as isize;
_12 = &place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.2;
_12 = &_24.2;
_2 = !_17[_9];
_20 = _21;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).2 = [_8.2[_9],_8.2[_9],_8.2[_9],_8.2[_9],_8.2[_9],_8.2[_9]];
_27 = !_9;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.1 = core::ptr::addr_of!((*_12));
_8.3 = Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3);
_20 = -_21;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.2 = (*_12) * (*_12);
_8.2[_9] = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).2[_9];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.0 = !_24.0;
Goto(bb9)
}
bb9 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).0 = [_11[_9],_8.0[_9],_8.0[_9],_8.0[_9],_11[_9],_11[_9]];
_26 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.2 as f64;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).3.1 = core::ptr::addr_of!(_8.3.2);
_16 = 302215095899457884109329725392897991368_u128;
Goto(bb10)
}
bb10 = {
_1 = _21 - _21;
_28 = _8.2[_9] + Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).2[_9];
_18 = (-69215040941065980066772410610989243931_i128) << _27;
_8.3.0 = _7;
_27 = _2 as usize;
_11 = [Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],_8.0[_9],_8.0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9]];
_11 = [Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],_8.0[_9],_8.0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).0[_9],_8.0[_9],_8.0[_9]];
_7 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3.0 & _24.0;
_11[_9] = _8.0[_9] ^ _8.0[_9];
_17[_9] = _2 & _2;
Call(_8.3.0 = fn9(Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0).3), (*_12), Move(_24), Move(_10), _8.3.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = _22;
RET.0 = Adt58::Variant1 { fld0: Move(_8) };
_13 = _21;
_8.3.1 = core::ptr::addr_of!(_24.2);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).1 = _19 != _26;
_19 = _16 as f64;
_10 = &_20;
_24.0 = _7;
_17 = [_2,_2,_2,_2];
_24.0 = _14 as u8;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(RET.0, 1), 0)).1 = _6;
_26 = _23 * _23;
Goto(bb12)
}
bb12 = {
Call(_41 = dump_var(Move(_27), Move(_18), Move(_11), Move(_17)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_41 = dump_var(Move(_16), Move(_25), Move(_14), Move(_28)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (u8, *const i16, i16),mut _2: i16,mut _3: (u8, *const i16, i16),mut _4: &'static isize,mut _5: i16) -> u8 {
mir! {
type RET = u8;
let _6: ((*mut bool,), usize);
let _7: Adt58;
let _8: *const i16;
let _9: *mut *mut &'static Adt18;
let _10: (*const &'static f64, char);
let _11: ();
let _12: ();
{
_1.0 = _3.0 | _3.0;
_6.1 = 443705554659352254_usize;
_3 = (_1.0, Move(_1.1), _1.2);
RET = _3.0;
_3.2 = '\u{f6fdc}' as i16;
_3.2 = _1.2 >> _6.1;
_3.2 = 2749843353_u32 as i16;
_2 = _1.2 & _1.2;
_3.0 = _1.0 | RET;
_6.1 = 16316582619244978841_u64 as usize;
_3.2 = (-1663381198_i32) as i16;
_2 = _3.2 & _1.2;
_1 = (RET, Move(_3.1), _2);
_3.1 = core::ptr::addr_of!(_1.2);
RET = _3.0 | _3.0;
_3.0 = RET;
_1.1 = core::ptr::addr_of!(_3.2);
_1.2 = _2;
_6.1 = (-9223372036854775808_isize) as usize;
_1.0 = _3.0;
_3 = Move(_1);
_3.1 = core::ptr::addr_of!(_5);
_1.1 = core::ptr::addr_of!(_3.2);
_3.1 = core::ptr::addr_of!(_5);
_2 = !_3.2;
_1 = (_3.0, Move(_3.1), _2);
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(Move(_5), _12, _12, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f32,mut _2: isize,mut _3: isize,mut _4: bool,mut _5: [i32; 3],mut _6: f32,mut _7: i64) -> *const i16 {
mir! {
type RET = *const i16;
let _8: (u16, *mut bool);
let _9: isize;
let _10: u8;
let _11: isize;
let _12: *mut u32;
let _13: Adt20;
let _14: *const [i64; 6];
let _15: bool;
let _16: *mut *mut &'static Adt18;
let _17: *mut &'static mut *const u32;
let _18: [bool; 6];
let _19: (u16, *mut bool);
let _20: [u64; 3];
let _21: u128;
let _22: [bool; 6];
let _23: i16;
let _24: isize;
let _25: char;
let _26: isize;
let _27: bool;
let _28: u64;
let _29: *mut &'static mut *const u32;
let _30: i32;
let _31: bool;
let _32: [u64; 4];
let _33: isize;
let _34: *mut [usize; 4];
let _35: bool;
let _36: [usize; 7];
let _37: *mut Adt18;
let _38: *mut u32;
let _39: f32;
let _40: &'static f64;
let _41: *const [i64; 6];
let _42: [i32; 3];
let _43: i8;
let _44: Adt18;
let _45: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _46: [char; 6];
let _47: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _48: isize;
let _49: char;
let _50: isize;
let _51: [u32; 6];
let _52: f64;
let _53: [u64; 4];
let _54: *mut bool;
let _55: isize;
let _56: *mut *const f32;
let _57: &'static i16;
let _58: f32;
let _59: &'static isize;
let _60: Adt46;
let _61: (i16, (char, i32, f64, char), i32, i64);
let _62: f32;
let _63: isize;
let _64: bool;
let _65: isize;
let _66: Adt18;
let _67: i64;
let _68: i32;
let _69: &'static mut u32;
let _70: &'static mut &'static u128;
let _71: isize;
let _72: *const i16;
let _73: &'static mut u32;
let _74: &'static u16;
let _75: *mut [usize; 4];
let _76: u16;
let _77: f32;
let _78: &'static *mut &'static Adt18;
let _79: f32;
let _80: *const i16;
let _81: Adt31;
let _82: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _83: *const [u16; 1];
let _84: [u32; 6];
let _85: f32;
let _86: char;
let _87: (u8, *const i16, i16);
let _88: isize;
let _89: usize;
let _90: u8;
let _91: *mut &'static mut *const u32;
let _92: Adt20;
let _93: [i8; 7];
let _94: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _95: ();
let _96: ();
{
_5 = [387463102_i32,1343759595_i32,(-857718263_i32)];
_4 = _6 <= _6;
_5 = [1503453455_i32,1321895125_i32,1999551767_i32];
_3 = _2 | _2;
_7 = -(-1712308501045091237_i64);
_7 = (-7937522525118764834_i64);
_4 = !true;
_7 = 5886879860479596084_i64 ^ 3893049600631311764_i64;
_5 = [(-568048329_i32),(-999854293_i32),1492045339_i32];
_2 = _3;
_6 = _1;
_8.1 = core::ptr::addr_of_mut!(_4);
_7 = 8451197185251161618_i64 * 297700198434431303_i64;
Goto(bb1)
}
bb1 = {
_3 = _2 & _2;
_7 = 256935161782967773896518872839799772560_u128 as i64;
_1 = -_6;
_3 = _2 - _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_10 = 85_u8;
_3 = _2 | _2;
_6 = _1 + _1;
_4 = _3 > _3;
_9 = _3;
_8.0 = !23488_u16;
_2 = 120087338415350969225843513489157154130_u128 as isize;
_7 = _4 as i64;
_8.1 = core::ptr::addr_of_mut!(_4);
_6 = _1 * _1;
_5 = [(-742522629_i32),(-569577096_i32),(-2109049409_i32)];
_8.0 = 57183_u16 >> _10;
_8.0 = 44099_u16 * 44172_u16;
_9 = _3;
_5 = [(-848596348_i32),(-1978182738_i32),415813_i32];
Call(_8 = fn11(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = true;
_2 = -_3;
_11 = 4398611324265654527_u64 as isize;
_3 = -_9;
_11 = (-96323330_i32) as isize;
_9 = _2 + _3;
_9 = _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = _3 - _2;
_9 = _2;
_13 = Adt20::Variant1 { fld0: _4,fld1: _2 };
_10 = 176_u8;
_9 = _2 + _2;
_4 = Field::<bool>(Variant(_13, 1), 0);
_9 = Field::<isize>(Variant(_13, 1), 1) << _8.0;
_1 = -_6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _4;
_6 = 18112_i16 as f32;
_5 = [(-1585623295_i32),(-516701161_i32),638659103_i32];
_10 = 248_u8;
Goto(bb3)
}
bb3 = {
_1 = _6 - _6;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_15 = !_4;
_2 = -_9;
_15 = _8.0 == _8.0;
_3 = _9 << Field::<isize>(Variant(_13, 1), 1);
_7 = !(-3776746764166920927_i64);
_8.0 = 49427_u16;
_7 = 29804_i16 as i64;
_8.0 = !18706_u16;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_3 = _9;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = !_2;
_6 = _1;
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
248 => bb7,
_ => bb6
}
}
bb4 = {
_4 = true;
_2 = -_3;
_11 = 4398611324265654527_u64 as isize;
_3 = -_9;
_11 = (-96323330_i32) as isize;
_9 = _2 + _3;
_9 = _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = _3 - _2;
_9 = _2;
_13 = Adt20::Variant1 { fld0: _4,fld1: _2 };
_10 = 176_u8;
_9 = _2 + _2;
_4 = Field::<bool>(Variant(_13, 1), 0);
_9 = Field::<isize>(Variant(_13, 1), 1) << _8.0;
_1 = -_6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _4;
_6 = 18112_i16 as f32;
_5 = [(-1585623295_i32),(-516701161_i32),638659103_i32];
_10 = 248_u8;
Goto(bb3)
}
bb5 = {
_3 = _2 & _2;
_7 = 256935161782967773896518872839799772560_u128 as i64;
_1 = -_6;
_3 = _2 - _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_10 = 85_u8;
_3 = _2 | _2;
_6 = _1 + _1;
_4 = _3 > _3;
_9 = _3;
_8.0 = !23488_u16;
_2 = 120087338415350969225843513489157154130_u128 as isize;
_7 = _4 as i64;
_8.1 = core::ptr::addr_of_mut!(_4);
_6 = _1 * _1;
_5 = [(-742522629_i32),(-569577096_i32),(-2109049409_i32)];
_8.0 = 57183_u16 >> _10;
_8.0 = 44099_u16 * 44172_u16;
_9 = _3;
_5 = [(-848596348_i32),(-1978182738_i32),415813_i32];
Call(_8 = fn11(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_7 = (-1502125387987207407_i64);
_6 = -_1;
_8.0 = 30647_u16 - 40898_u16;
_6 = _1;
_15 = !Field::<bool>(Variant(_13, 1), 0);
place!(Field::<isize>(Variant(_13, 1), 1)) = _2 << _3;
_1 = _6 + _6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _3 > Field::<isize>(Variant(_13, 1), 1);
_19.0 = 11514_i16 as u16;
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0)];
_8.0 = !_19.0;
_19 = (_8.0, Move(_8.1));
_8.0 = _19.0;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_3 = Field::<isize>(Variant(_13, 1), 1) | _9;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_18 = [Field::<bool>(Variant(_13, 1), 0),_15,_15,Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15];
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15,_15];
_8.1 = Move(_19.1);
_21 = 220130483133349174066579597963261132920_u128;
_10 = 163_u8 | 118_u8;
_11 = _3 << Field::<isize>(Variant(_13, 1), 1);
_15 = !Field::<bool>(Variant(_13, 1), 0);
_15 = !Field::<bool>(Variant(_13, 1), 0);
Goto(bb8)
}
bb8 = {
_7 = (-8764879149497624630_i64) << Field::<isize>(Variant(_13, 1), 1);
_4 = Field::<isize>(Variant(_13, 1), 1) > _3;
_9 = _4 as isize;
match _21 {
0 => bb1,
220130483133349174066579597963261132920 => bb9,
_ => bb4
}
}
bb9 = {
_8.0 = !_19.0;
place!(Field::<isize>(Variant(_13, 1), 1)) = (-44546090342761050964695821738228939397_i128) as isize;
_19 = Move(_8);
_13 = Adt20::Variant0 { fld0: 7_usize,fld1: 20250_i16 };
_4 = _15 != _15;
_13 = Adt20::Variant0 { fld0: 3942644153955395530_usize,fld1: 8359_i16 };
_19.0 = 40576_u16 - 13812_u16;
_8 = (_19.0, Move(_19.1));
_19.0 = _8.0 & _8.0;
_5 = [(-348948977_i32),(-665064700_i32),(-579442380_i32)];
_22 = [_4,_4,_15,_15,_4,_4];
_15 = _4 ^ _4;
_10 = !207_u8;
_26 = _11 >> _11;
_19 = Move(_8);
_5 = [407603857_i32,1039554220_i32,(-880763005_i32)];
_8.1 = Move(_19.1);
_8.1 = core::ptr::addr_of_mut!(_15);
_11 = 5350228276222607705_u64 as isize;
_23 = !14104_i16;
_26 = _9 << _3;
_28 = (-112_i8) as u64;
Goto(bb10)
}
bb10 = {
_3 = -_9;
_19 = (57596_u16, Move(_8.1));
_2 = _3 >> _7;
_4 = _15 > _15;
_23 = -7651_i16;
_23 = (-4384_i16);
_15 = _4 ^ _4;
_1 = _6;
_27 = _15 ^ _4;
_3 = _2 ^ _9;
_4 = _15;
_25 = '\u{72406}';
_25 = '\u{9f0c5}';
_10 = 28_i8 as u8;
_9 = _26 * _3;
_7 = 7253804984668707930_i64 + 1251557374280839750_i64;
_3 = _2 | _26;
Goto(bb11)
}
bb11 = {
_19.1 = core::ptr::addr_of_mut!(_31);
_32 = [_28,_28,_28,_28];
_9 = _28 as isize;
_33 = _19.0 as isize;
_18 = [_27,_27,_27,_4,_15,_4];
_7 = 1335989549500173397_i64 >> _33;
_5 = [(-1262485329_i32),(-1225710294_i32),(-604947905_i32)];
_4 = !_27;
_2 = _3;
_24 = _33;
_31 = _19.0 > _19.0;
_31 = !_15;
_1 = _6;
_35 = !_4;
place!(Field::<i16>(Variant(_13, 0), 1)) = _23 ^ _23;
_21 = 308351396944849695681307689353619376769_u128 | 34581102862819718194056163996180881433_u128;
_8.1 = core::ptr::addr_of_mut!(_4);
_7 = -7877610092553662639_i64;
_5 = [(-1807815988_i32),(-336512690_i32),1750727262_i32];
_15 = _27 | _27;
_19.0 = Field::<i16>(Variant(_13, 0), 1) as u16;
_24 = _26 - _2;
place!(Field::<i16>(Variant(_13, 0), 1)) = !_23;
_24 = _26 | _3;
_20 = [_28,_28,_28];
Goto(bb12)
}
bb12 = {
_11 = !_3;
_3 = _26;
_33 = !_3;
_30 = (-685168099_i32);
_7 = (-2256374119627764304_i64) * (-2010471704215480845_i64);
_36 = [11487906031614114708_usize,17504814774488766298_usize,3_usize,7813070048376799372_usize,16739061193196739528_usize,5_usize,4631247504823799362_usize];
_35 = _3 != _26;
_32 = [_28,_28,_28,_28];
_33 = _24;
place!(Field::<usize>(Variant(_13, 0), 0)) = 10269725674701483025_usize + 14355920293955968986_usize;
place!(Field::<usize>(Variant(_13, 0), 0)) = _25 as usize;
_25 = '\u{550c3}';
_33 = _24 | _11;
_19.0 = 49361_u16 & 19861_u16;
_5 = [_30,_30,_30];
_4 = _24 < _11;
_30 = !779541212_i32;
match _23 {
340282366920938463463374607431768207072 => bb14,
_ => bb13
}
}
bb13 = {
_3 = -_9;
_19 = (57596_u16, Move(_8.1));
_2 = _3 >> _7;
_4 = _15 > _15;
_23 = -7651_i16;
_23 = (-4384_i16);
_15 = _4 ^ _4;
_1 = _6;
_27 = _15 ^ _4;
_3 = _2 ^ _9;
_4 = _15;
_25 = '\u{72406}';
_25 = '\u{9f0c5}';
_10 = 28_i8 as u8;
_9 = _26 * _3;
_7 = 7253804984668707930_i64 + 1251557374280839750_i64;
_3 = _2 | _26;
Goto(bb11)
}
bb14 = {
_9 = _3 & _2;
place!(Field::<usize>(Variant(_13, 0), 0)) = _7 as usize;
_32 = [_28,_28,_28,_28];
_7 = (-2750431183742424305_i64) ^ 2346532494135004400_i64;
_6 = -_1;
Goto(bb15)
}
bb15 = {
_13 = Adt20::Variant3 { fld0: _1,fld1: _10 };
_19.1 = core::ptr::addr_of_mut!(_31);
_21 = _23 as u128;
_8 = Move(_19);
_19 = Move(_8);
_8.0 = _4 as u16;
_8.1 = Move(_19.1);
_8.0 = _25 as u16;
_19 = (_8.0, Move(_8.1));
_35 = _15 == _27;
_8.1 = Move(_19.1);
_35 = _4 ^ _15;
_35 = _31 > _4;
_44 = Adt18::Variant1 { fld0: _21,fld1: 3950438736483434459_usize,fld2: _10,fld3: (-124_i8) };
_39 = Field::<f32>(Variant(_13, 3), 0) * Field::<f32>(Variant(_13, 3), 0);
_5 = [_30,_30,_30];
_19.1 = core::ptr::addr_of_mut!(_15);
_19.1 = core::ptr::addr_of_mut!(_15);
_43 = (-64_i8) + (-42_i8);
_21 = Field::<u128>(Variant(_44, 1), 0) << _11;
_15 = _35 & _4;
_24 = _3;
match _23 {
0 => bb2,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
340282366920938463463374607431768207072 => bb21,
_ => bb20
}
}
bb16 = {
_9 = _3 & _2;
place!(Field::<usize>(Variant(_13, 0), 0)) = _7 as usize;
_32 = [_28,_28,_28,_28];
_7 = (-2750431183742424305_i64) ^ 2346532494135004400_i64;
_6 = -_1;
Goto(bb15)
}
bb17 = {
_4 = true;
_2 = -_3;
_11 = 4398611324265654527_u64 as isize;
_3 = -_9;
_11 = (-96323330_i32) as isize;
_9 = _2 + _3;
_9 = _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = _3 - _2;
_9 = _2;
_13 = Adt20::Variant1 { fld0: _4,fld1: _2 };
_10 = 176_u8;
_9 = _2 + _2;
_4 = Field::<bool>(Variant(_13, 1), 0);
_9 = Field::<isize>(Variant(_13, 1), 1) << _8.0;
_1 = -_6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _4;
_6 = 18112_i16 as f32;
_5 = [(-1585623295_i32),(-516701161_i32),638659103_i32];
_10 = 248_u8;
Goto(bb3)
}
bb18 = {
_1 = _6 - _6;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_15 = !_4;
_2 = -_9;
_15 = _8.0 == _8.0;
_3 = _9 << Field::<isize>(Variant(_13, 1), 1);
_7 = !(-3776746764166920927_i64);
_8.0 = 49427_u16;
_7 = 29804_i16 as i64;
_8.0 = !18706_u16;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_3 = _9;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = !_2;
_6 = _1;
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
248 => bb7,
_ => bb6
}
}
bb19 = {
_3 = _2 & _2;
_7 = 256935161782967773896518872839799772560_u128 as i64;
_1 = -_6;
_3 = _2 - _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_10 = 85_u8;
_3 = _2 | _2;
_6 = _1 + _1;
_4 = _3 > _3;
_9 = _3;
_8.0 = !23488_u16;
_2 = 120087338415350969225843513489157154130_u128 as isize;
_7 = _4 as i64;
_8.1 = core::ptr::addr_of_mut!(_4);
_6 = _1 * _1;
_5 = [(-742522629_i32),(-569577096_i32),(-2109049409_i32)];
_8.0 = 57183_u16 >> _10;
_8.0 = 44099_u16 * 44172_u16;
_9 = _3;
_5 = [(-848596348_i32),(-1978182738_i32),415813_i32];
Call(_8 = fn11(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
Return()
}
bb21 = {
_27 = _3 == _26;
place!(Field::<i8>(Variant(_44, 1), 3)) = _43 ^ _43;
_42 = [_30,_30,_30];
_11 = _33 << _26;
_37 = core::ptr::addr_of_mut!(_44);
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 0_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
_32 = [_28,_28,_28,_28];
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 - _43;
_42 = [_30,_30,_30];
_19.1 = Move(_8.1);
place!(Field::<usize>(Variant((*_37), 1), 1)) = 1_usize;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21;
place!(Field::<u128>(Variant((*_37), 1), 0)) = !_21;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 & _43;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 >> Field::<u128>(Variant((*_37), 1), 0);
_8.0 = !_19.0;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 5_usize * 2_usize;
match _23 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb22,
4 => bb23,
340282366920938463463374607431768207072 => bb25,
_ => bb24
}
}
bb22 = {
_4 = true;
_2 = -_3;
_11 = 4398611324265654527_u64 as isize;
_3 = -_9;
_11 = (-96323330_i32) as isize;
_9 = _2 + _3;
_9 = _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = _3 - _2;
_9 = _2;
_13 = Adt20::Variant1 { fld0: _4,fld1: _2 };
_10 = 176_u8;
_9 = _2 + _2;
_4 = Field::<bool>(Variant(_13, 1), 0);
_9 = Field::<isize>(Variant(_13, 1), 1) << _8.0;
_1 = -_6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _4;
_6 = 18112_i16 as f32;
_5 = [(-1585623295_i32),(-516701161_i32),638659103_i32];
_10 = 248_u8;
Goto(bb3)
}
bb23 = {
_3 = _2 & _2;
_7 = 256935161782967773896518872839799772560_u128 as i64;
_1 = -_6;
_3 = _2 - _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_10 = 85_u8;
_3 = _2 | _2;
_6 = _1 + _1;
_4 = _3 > _3;
_9 = _3;
_8.0 = !23488_u16;
_2 = 120087338415350969225843513489157154130_u128 as isize;
_7 = _4 as i64;
_8.1 = core::ptr::addr_of_mut!(_4);
_6 = _1 * _1;
_5 = [(-742522629_i32),(-569577096_i32),(-2109049409_i32)];
_8.0 = 57183_u16 >> _10;
_8.0 = 44099_u16 * 44172_u16;
_9 = _3;
_5 = [(-848596348_i32),(-1978182738_i32),415813_i32];
Call(_8 = fn11(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb24 = {
_1 = _6 - _6;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_15 = !_4;
_2 = -_9;
_15 = _8.0 == _8.0;
_3 = _9 << Field::<isize>(Variant(_13, 1), 1);
_7 = !(-3776746764166920927_i64);
_8.0 = 49427_u16;
_7 = 29804_i16 as i64;
_8.0 = !18706_u16;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_3 = _9;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = !_2;
_6 = _1;
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
248 => bb7,
_ => bb6
}
}
bb25 = {
place!(Field::<usize>(Variant((*_37), 1), 1)) = !9157017331413373637_usize;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _28 as u8;
place!(Field::<u128>(Variant((*_37), 1), 0)) = !_21;
_26 = _28 as isize;
_46 = [_25,_25,_25,_25,_25,_25];
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<u8>(Variant(_13, 3), 1);
place!(Field::<i8>(Variant((*_37), 1), 3)) = -_43;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43;
_35 = Field::<u128>(Variant((*_37), 1), 0) > Field::<u128>(Variant((*_37), 1), 0);
place!(Field::<usize>(Variant(_44, 1), 1)) = 807359759325973901_usize >> Field::<u128>(Variant((*_37), 1), 0);
_50 = _24 ^ _33;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 | _43;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _19.0 as u8;
_19.0 = _8.0;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21;
place!(Field::<usize>(Variant((*_37), 1), 1)) = _7 as usize;
_43 = _1 as i8;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 4_usize * 13877649114314969820_usize;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 >> _33;
_48 = _33 + _33;
_49 = _25;
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<u128>(Variant((*_37), 1), 0) as u8;
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<u8>(Variant(_13, 3), 1) >> Field::<i8>(Variant((*_37), 1), 3);
_15 = _4 < _4;
Goto(bb26)
}
bb26 = {
_52 = _39 as f64;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 5471580384433715640_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
place!(Field::<usize>(Variant((*_37), 1), 1)) = _8.0 as usize;
_3 = _28 as isize;
place!(Field::<usize>(Variant(_44, 1), 1)) = 8189599950643162346_usize >> Field::<u128>(Variant((*_37), 1), 0);
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _25,fld2: _52,fld3: 26148949347078895305132942463395392389_i128,fld4: Field::<u8>(Variant(_13, 3), 1),fld5: 7847426216175289436_usize,fld6: _7 };
place!(Field::<i128>(Variant((*_37), 0), 3)) = 142656940945477572938629016568417219926_i128 | 114859148801897031713761088230034345309_i128;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _49,fld2: _52,fld3: 114243930610138365203065247527219832887_i128,fld4: Field::<u8>(Variant(_13, 3), 1),fld5: 831056618490402767_usize,fld6: _7 };
place!(Field::<f64>(Variant((*_37), 0), 2)) = _52;
place!(Field::<i128>(Variant((*_37), 0), 3)) = (-140114088509810124897576350911359524853_i128) | (-1791929407636588333164757071507004850_i128);
_8.0 = _35 as u16;
place!(Field::<i64>(Variant((*_37), 0), 6)) = _7;
_39 = -_1;
place!(Field::<usize>(Variant((*_37), 0), 5)) = !2_usize;
_51 = [16086790_u32,3115978247_u32,2813074885_u32,3851346378_u32,2330081415_u32,2365835292_u32];
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _25,fld2: _52,fld3: 38840090778091912346089232724809569912_i128,fld4: Field::<u8>(Variant(_13, 3), 1),fld5: 13485823641571442707_usize,fld6: _7 };
place!(Field::<char>(Variant((*_37), 0), 1)) = _49;
_10 = Field::<u8>(Variant((*_37), 0), 4);
place!(Field::<u8>(Variant((*_37), 0), 4)) = _9 as u8;
place!(Field::<u128>(Variant((*_37), 0), 0)) = _21 + _21;
place!(Field::<i128>(Variant((*_37), 0), 3)) = -75258113653678918834985465226526524302_i128;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 14897060297655602440_usize,fld2: _10,fld3: _43 };
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 >> _24;
place!(Field::<u128>(Variant((*_37), 1), 0)) = Field::<u8>(Variant((*_37), 1), 2) as u128;
place!(Field::<usize>(Variant((*_37), 1), 1)) = !0_usize;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 & _21;
match _23 {
0 => bb10,
1 => bb27,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
340282366920938463463374607431768207072 => bb33,
_ => bb32
}
}
bb27 = {
_1 = _6 - _6;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_15 = !_4;
_2 = -_9;
_15 = _8.0 == _8.0;
_3 = _9 << Field::<isize>(Variant(_13, 1), 1);
_7 = !(-3776746764166920927_i64);
_8.0 = 49427_u16;
_7 = 29804_i16 as i64;
_8.0 = !18706_u16;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_3 = _9;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = !_2;
_6 = _1;
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
248 => bb7,
_ => bb6
}
}
bb28 = {
_1 = _6 - _6;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_15 = !_4;
_2 = -_9;
_15 = _8.0 == _8.0;
_3 = _9 << Field::<isize>(Variant(_13, 1), 1);
_7 = !(-3776746764166920927_i64);
_8.0 = 49427_u16;
_7 = 29804_i16 as i64;
_8.0 = !18706_u16;
place!(Field::<isize>(Variant(_13, 1), 1)) = _9;
_3 = _9;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = !_2;
_6 = _1;
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
248 => bb7,
_ => bb6
}
}
bb29 = {
_7 = (-1502125387987207407_i64);
_6 = -_1;
_8.0 = 30647_u16 - 40898_u16;
_6 = _1;
_15 = !Field::<bool>(Variant(_13, 1), 0);
place!(Field::<isize>(Variant(_13, 1), 1)) = _2 << _3;
_1 = _6 + _6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _3 > Field::<isize>(Variant(_13, 1), 1);
_19.0 = 11514_i16 as u16;
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0)];
_8.0 = !_19.0;
_19 = (_8.0, Move(_8.1));
_8.0 = _19.0;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_3 = Field::<isize>(Variant(_13, 1), 1) | _9;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_18 = [Field::<bool>(Variant(_13, 1), 0),_15,_15,Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15];
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15,_15];
_8.1 = Move(_19.1);
_21 = 220130483133349174066579597963261132920_u128;
_10 = 163_u8 | 118_u8;
_11 = _3 << Field::<isize>(Variant(_13, 1), 1);
_15 = !Field::<bool>(Variant(_13, 1), 0);
_15 = !Field::<bool>(Variant(_13, 1), 0);
Goto(bb8)
}
bb30 = {
_4 = true;
_2 = -_3;
_11 = 4398611324265654527_u64 as isize;
_3 = -_9;
_11 = (-96323330_i32) as isize;
_9 = _2 + _3;
_9 = _2;
_8.1 = core::ptr::addr_of_mut!(_4);
_9 = _3 - _2;
_9 = _2;
_13 = Adt20::Variant1 { fld0: _4,fld1: _2 };
_10 = 176_u8;
_9 = _2 + _2;
_4 = Field::<bool>(Variant(_13, 1), 0);
_9 = Field::<isize>(Variant(_13, 1), 1) << _8.0;
_1 = -_6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _4;
_6 = 18112_i16 as f32;
_5 = [(-1585623295_i32),(-516701161_i32),638659103_i32];
_10 = 248_u8;
Goto(bb3)
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
_22 = [_15,_4,_35,_31,_31,_27];
place!(Field::<u8>(Variant((*_37), 1), 2)) = !_10;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 10595284650221914859_usize ^ 3_usize;
place!(Field::<u128>(Variant((*_37), 1), 0)) = !_21;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 5_usize;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 >> _2;
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<u8>(Variant(_13, 3), 1) & _10;
_53 = [_28,_28,_28,_28];
_11 = _48 - _48;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _10 | _10;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _30 as u128;
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<usize>(Variant((*_37), 1), 1) as u8;
_3 = Field::<u8>(Variant((*_37), 1), 2) as isize;
place!(Field::<i8>(Variant(_44, 1), 3)) = _30 as i8;
_57 = &_23;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 11294322958868324473_usize | 7_usize;
Goto(bb34)
}
bb34 = {
place!(Field::<usize>(Variant((*_37), 1), 1)) = 3_usize << _2;
_24 = _33;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _25 as u8;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _10 << _8.0;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 + _21;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 | _21;
place!(Field::<usize>(Variant((*_37), 1), 1)) = _30 as usize;
place!(Field::<usize>(Variant((*_37), 1), 1)) = _2 as usize;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 8597600871672414701_usize;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 | _43;
_37 = core::ptr::addr_of_mut!((*_37));
_61.1.1 = _30;
place!(Field::<u8>(Variant((*_37), 1), 2)) = Field::<f32>(Variant(_13, 3), 0) as u8;
_20 = [_28,_28,_28];
place!(Field::<i8>(Variant((*_37), 1), 3)) = _39 as i8;
_46 = [_49,_25,_25,_49,_25,_25];
_44 = Adt18::Variant0 { fld0: _21,fld1: _25,fld2: _52,fld3: (-6657425611651099674486775407758785985_i128),fld4: Field::<u8>(Variant(_13, 3), 1),fld5: 10557965524826972152_usize,fld6: _7 };
place!(Field::<char>(Variant((*_37), 0), 1)) = _49;
place!(Field::<usize>(Variant((*_37), 0), 5)) = 2_usize | 1995113298862575194_usize;
match (*_57) {
0 => bb33,
1 => bb14,
2 => bb16,
340282366920938463463374607431768207072 => bb35,
_ => bb6
}
}
bb35 = {
place!(Field::<i128>(Variant((*_37), 0), 3)) = (-134473579097649839596127758322395319746_i128) << _11;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 3_usize,fld2: _10,fld3: _43 };
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 11452234257448324207_usize,fld2: _10,fld3: _43 };
place!(Field::<usize>(Variant((*_37), 1), 1)) = _30 as usize;
place!(Field::<i8>(Variant((*_37), 1), 3)) = !_43;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 & _43;
_37 = core::ptr::addr_of_mut!((*_37));
RET = core::ptr::addr_of!((*_57));
Goto(bb36)
}
bb36 = {
place!(Field::<u128>(Variant((*_37), 1), 0)) = _7 as u128;
_61.3 = (*_57) as i64;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 & _21;
_8 = Move(_19);
place!(Field::<usize>(Variant((*_37), 1), 1)) = 3731451387311302214_usize * 13377386447295465622_usize;
_49 = _25;
place!(Field::<u8>(Variant(_44, 1), 2)) = _10;
_61.1.2 = _52;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 14061720854007495387_usize;
place!(Field::<usize>(Variant((*_37), 1), 1)) = _28 as usize;
_71 = _2;
Goto(bb37)
}
bb37 = {
place!(Field::<usize>(Variant((*_37), 1), 1)) = 2_usize;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 1991403861063764665_usize - 1039234661862422545_usize;
_19 = (_8.0, Move(_8.1));
place!(Field::<usize>(Variant((*_37), 1), 1)) = !6_usize;
place!(Field::<u128>(Variant((*_37), 1), 0)) = !_21;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 >> Field::<u128>(Variant((*_37), 1), 0);
_61.1.1 = _30;
_67 = -_61.3;
_21 = !Field::<u128>(Variant((*_37), 1), 0);
_61.1.3 = _25;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _25,fld2: _52,fld3: 101052195928480142125054764240107461302_i128,fld4: _10,fld5: 5_usize,fld6: _7 };
match (*RET) {
0 => bb36,
1 => bb30,
2 => bb31,
3 => bb33,
4 => bb10,
5 => bb26,
340282366920938463463374607431768207072 => bb38,
_ => bb17
}
}
bb38 = {
place!(Field::<u128>(Variant((*_37), 0), 0)) = _21;
place!(Field::<u128>(Variant((*_37), 0), 0)) = 58134712484262970220663956409981975205_i128 as u128;
_10 = Field::<u8>(Variant((*_37), 0), 4) * Field::<u8>(Variant((*_37), 0), 4);
_51 = [89452616_u32,3738156684_u32,3834112727_u32,3890393565_u32,4106091766_u32,894346705_u32];
_5 = [_30,_61.1.1,_30];
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _61.1.3,fld2: _61.1.2,fld3: 33999884332747529116349152086922559107_i128,fld4: Field::<u8>(Variant(_13, 3), 1),fld5: 3154134178577158562_usize,fld6: _7 };
place!(Field::<f64>(Variant((*_37), 0), 2)) = (*_57) as f64;
place!(Field::<u8>(Variant((*_37), 0), 4)) = Field::<u128>(Variant((*_37), 0), 0) as u8;
place!(Field::<usize>(Variant((*_37), 0), 5)) = 3_usize << Field::<u128>(Variant((*_37), 0), 0);
_8.0 = !_19.0;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _25,fld2: _61.1.2,fld3: 127848771378898494697458712570635787511_i128,fld4: _10,fld5: 967719747974215689_usize,fld6: _61.3 };
_63 = _71 ^ _2;
_28 = 9428440704573678203_u64;
_61.1 = (Field::<char>(Variant(_44, 0), 1), _30, Field::<f64>(Variant((*_37), 0), 2), Field::<char>(Variant((*_37), 0), 1));
_8.0 = _19.0 & _19.0;
_22 = [_31,_15,_4,_35,_15,_15];
_63 = Field::<char>(Variant((*_37), 0), 1) as isize;
Goto(bb39)
}
bb39 = {
place!(Field::<i64>(Variant((*_37), 0), 6)) = _7;
place!(Field::<i128>(Variant((*_37), 0), 3)) = !68035433323835216989114034958078254960_i128;
_30 = _61.1.1 - _61.1.1;
place!(Field::<usize>(Variant((*_37), 0), 5)) = 5_usize + 7_usize;
_65 = _11 | _48;
place!(Field::<u128>(Variant((*_37), 0), 0)) = Field::<char>(Variant((*_37), 0), 1) as u128;
place!(Field::<i64>(Variant((*_37), 0), 6)) = Field::<f64>(Variant((*_37), 0), 2) as i64;
place!(Field::<i64>(Variant((*_37), 0), 6)) = !_7;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 16802810684996226589_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
_63 = _11 + _9;
place!(Field::<usize>(Variant((*_37), 1), 1)) = _49 as usize;
_82.3.2 = (*_57) ^ (*_57);
_42 = _5;
_35 = _31 == _31;
_74 = &_8.0;
Call(place!(Field::<i8>(Variant((*_37), 1), 3)) = core::intrinsics::bswap(_43), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
place!(Field::<usize>(Variant((*_37), 1), 1)) = 5_usize << _50;
_4 = Field::<u128>(Variant((*_37), 1), 0) <= Field::<u128>(Variant((*_37), 1), 0);
match (*RET) {
0 => bb36,
1 => bb9,
2 => bb27,
3 => bb10,
4 => bb13,
340282366920938463463374607431768207072 => bb41,
_ => bb34
}
}
bb41 = {
place!(Field::<usize>(Variant((*_37), 1), 1)) = 8454392751083137854_usize;
_6 = Field::<f32>(Variant(_13, 3), 0) - Field::<f32>(Variant(_13, 3), 0);
_21 = Field::<u128>(Variant((*_37), 1), 0) - Field::<u128>(Variant((*_37), 1), 0);
Goto(bb42)
}
bb42 = {
_84 = [1678020256_u32,3010052454_u32,1007635172_u32,2378864770_u32,2812204504_u32,885133885_u32];
place!(Field::<usize>(Variant((*_37), 1), 1)) = !6_usize;
_59 = &_65;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _10 * Field::<u8>(Variant(_13, 3), 1);
place!(Field::<i8>(Variant((*_37), 1), 3)) = (*_74) as i8;
_42 = _5;
place!(Field::<i8>(Variant((*_37), 1), 3)) = _61.1.2 as i8;
_68 = _61.1.1 * _30;
match (*RET) {
0 => bb38,
340282366920938463463374607431768207072 => bb44,
_ => bb43
}
}
bb43 = {
Return()
}
bb44 = {
place!(Field::<u8>(Variant((*_37), 1), 2)) = !_10;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 2_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
_19.0 = _68 as u16;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _49,fld2: _61.1.2,fld3: 61904158595246246822882040667941971105_i128,fld4: _10,fld5: 7_usize,fld6: _61.3 };
place!(Field::<usize>(Variant((*_37), 0), 5)) = 12602238893206377574_usize & 886407101852688747_usize;
_48 = Field::<char>(Variant((*_37), 0), 1) as isize;
_82.3.1 = core::ptr::addr_of!((*RET));
_59 = &_11;
place!(Field::<char>(Variant((*_37), 0), 1)) = _61.1.3;
_58 = Field::<f32>(Variant(_13, 3), 0) + _1;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _61.1.0,fld2: _52,fld3: (-97291970209067956006054283545347867995_i128),fld4: _10,fld5: 3_usize,fld6: _61.3 };
place!(Field::<u8>(Variant((*_37), 0), 4)) = (*_74) as u8;
place!(Field::<i64>(Variant(_44, 0), 6)) = _7;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 17748778973872493651_usize,fld2: _10,fld3: _43 };
_61.2 = Field::<u128>(Variant(_44, 1), 0) as i32;
Call(place!(Field::<u8>(Variant((*_37), 1), 2)) = core::intrinsics::transmute(Field::<i8>(Variant((*_37), 1), 3)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
place!(Field::<u8>(Variant((*_37), 1), 2)) = _10;
_76 = (*_74) - (*_74);
place!(Field::<u128>(Variant((*_37), 1), 0)) = 3134833869_u32 as u128;
place!(Field::<u128>(Variant(_44, 1), 0)) = _21 | _21;
_30 = !_61.2;
(*_37) = Adt18::Variant0 { fld0: _21,fld1: _61.1.0,fld2: _61.1.2,fld3: (-13219786609015381082757993603237861743_i128),fld4: _10,fld5: 14516900510884819634_usize,fld6: _61.3 };
_82.3.2 = (*RET) ^ (*_57);
place!(Field::<i128>(Variant((*_37), 0), 3)) = 57669211359867982454462851646376704791_i128;
place!(Field::<u128>(Variant((*_37), 0), 0)) = _21 & _21;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 11059795294943745637_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
place!(Field::<usize>(Variant((*_37), 1), 1)) = _28 as usize;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 16430994073444664999_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 * _21;
place!(Field::<u8>(Variant((*_37), 1), 2)) = !Field::<u8>(Variant(_13, 3), 1);
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43;
place!(Field::<usize>(Variant((*_37), 1), 1)) = 7_usize;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _10 & Field::<u8>(Variant(_13, 3), 1);
_36 = [Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1),Field::<usize>(Variant((*_37), 1), 1)];
_82.3.2 = (*_57) + (*RET);
Call(place!(Field::<u8>(Variant((*_37), 1), 2)) = core::intrinsics::transmute(_43), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_25 = _61.1.0;
place!(Field::<u128>(Variant((*_37), 1), 0)) = !_21;
_42 = _5;
_64 = !_15;
_80 = core::ptr::addr_of!((*RET));
place!(Field::<u8>(Variant((*_37), 1), 2)) = !Field::<u8>(Variant(_13, 3), 1);
_18 = [_4,_27,_31,_64,_27,_27];
place!(Field::<usize>(Variant((*_37), 1), 1)) = !3406984872415363269_usize;
_87.0 = 748975470_u32 as u8;
Goto(bb47)
}
bb47 = {
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 4_usize,fld2: _10,fld3: _43 };
place!(Field::<usize>(Variant((*_37), 1), 1)) = 17927228023371237049_usize;
_48 = (*_59) & (*_59);
place!(Field::<i8>(Variant((*_37), 1), 3)) = _43 + _43;
place!(Field::<usize>(Variant((*_37), 1), 1)) = !6670392489314484407_usize;
place!(Field::<u8>(Variant((*_37), 1), 2)) = _87.0 >> (*_59);
_82.0 = _84;
match (*_80) {
0 => bb48,
340282366920938463463374607431768207072 => bb50,
_ => bb49
}
}
bb48 = {
_7 = (-1502125387987207407_i64);
_6 = -_1;
_8.0 = 30647_u16 - 40898_u16;
_6 = _1;
_15 = !Field::<bool>(Variant(_13, 1), 0);
place!(Field::<isize>(Variant(_13, 1), 1)) = _2 << _3;
_1 = _6 + _6;
place!(Field::<bool>(Variant(_13, 1), 0)) = _3 > Field::<isize>(Variant(_13, 1), 1);
_19.0 = 11514_i16 as u16;
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0)];
_8.0 = !_19.0;
_19 = (_8.0, Move(_8.1));
_8.0 = _19.0;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_3 = Field::<isize>(Variant(_13, 1), 1) | _9;
_15 = !Field::<bool>(Variant(_13, 1), 0);
_18 = [Field::<bool>(Variant(_13, 1), 0),_15,_15,Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15];
_18 = [Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),Field::<bool>(Variant(_13, 1), 0),_15,_15];
_8.1 = Move(_19.1);
_21 = 220130483133349174066579597963261132920_u128;
_10 = 163_u8 | 118_u8;
_11 = _3 << Field::<isize>(Variant(_13, 1), 1);
_15 = !Field::<bool>(Variant(_13, 1), 0);
_15 = !Field::<bool>(Variant(_13, 1), 0);
Goto(bb8)
}
bb49 = {
Return()
}
bb50 = {
_18 = [_31,_35,_31,_31,_35,_27];
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 7733319520623859632_usize,fld2: _10,fld3: _43 };
_4 = Field::<u128>(Variant((*_37), 1), 0) >= Field::<u128>(Variant((*_37), 1), 0);
_61.3 = _67;
_41 = core::ptr::addr_of!(_82.2);
_7 = -_67;
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 ^ _21;
_33 = _71;
_19.0 = (*_74) | (*_74);
_71 = (*_59) << Field::<u128>(Variant((*_37), 1), 0);
_61.1.1 = _30 << (*_59);
_85 = _1 - Field::<f32>(Variant(_13, 3), 0);
place!(Field::<u128>(Variant((*_37), 1), 0)) = _21 >> (*_59);
_88 = _50;
(*_37) = Adt18::Variant1 { fld0: _21,fld1: 8543930414970422170_usize,fld2: Field::<u8>(Variant(_13, 3), 1),fld3: _43 };
(*_41) = [_61.3,_7,_7,_61.3,_67,_7];
Goto(bb51)
}
bb51 = {
Call(_95 = dump_var(Move(_10), Move(_42), Move(_3), Move(_84)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_95 = dump_var(Move(_53), Move(_35), Move(_65), Move(_67)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_95 = dump_var(Move(_49), Move(_64), Move(_18), Move(_88)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_95 = dump_var(Move(_23), Move(_7), Move(_4), Move(_20)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_95 = dump_var(Move(_71), Move(_30), Move(_32), Move(_36)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_95 = dump_var(Move(_46), _96, _96, _96), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i32; 3]) -> (u16, *mut bool) {
mir! {
type RET = (u16, *mut bool);
let _2: [u16; 1];
let _3: isize;
let _4: &'static f64;
let _5: *mut Adt18;
let _6: *const u32;
let _7: char;
let _8: *const u32;
let _9: f64;
let _10: [u32; 7];
let _11: [u64; 4];
let _12: isize;
let _13: i128;
let _14: f32;
let _15: char;
let _16: &'static i16;
let _17: (i16, (char, i32, f64, char), i32, i64);
let _18: [i32; 1];
let _19: bool;
let _20: (u8, *const i16, i16);
let _21: *mut [u32; 7];
let _22: &'static mut usize;
let _23: isize;
let _24: *const i16;
let _25: isize;
let _26: *mut [usize; 4];
let _27: char;
let _28: *mut u32;
let _29: u128;
let _30: f64;
let _31: *mut *const [i64; 6];
let _32: isize;
let _33: *mut u16;
let _34: isize;
let _35: [i32; 3];
let _36: i16;
let _37: &'static *mut &'static Adt18;
let _38: i64;
let _39: bool;
let _40: (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _41: bool;
let _42: [i32; 1];
let _43: *mut Adt18;
let _44: *mut [usize; 4];
let _45: f32;
let _46: i64;
let _47: *mut *const [i64; 6];
let _48: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _49: &'static mut usize;
let _50: bool;
let _51: isize;
let _52: u32;
let _53: [bool; 6];
let _54: u128;
let _55: *mut *mut &'static Adt18;
let _56: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _57: (Adt58,);
let _58: &'static mut &'static u128;
let _59: Adt31;
let _60: f32;
let _61: (u8, *const i16, i16);
let _62: bool;
let _63: bool;
let _64: i8;
let _65: [usize; 7];
let _66: *mut [usize; 4];
let _67: ();
let _68: ();
{
_2 = [43387_u16];
RET.0 = !45514_u16;
_1 = [1528321045_i32,350416371_i32,(-1294798151_i32)];
_1 = [(-1413482951_i32),2092826127_i32,165061954_i32];
_3 = (-9223372036854775808_isize);
_3 = (-8_isize) * 9223372036854775807_isize;
RET.0 = 130708150595676915121455456900537324613_u128 as u16;
RET.0 = 23227_u16 >> _3;
_2 = [RET.0];
RET.0 = 63236_u16 * 25261_u16;
RET.0 = 47708_u16;
_1 = [1863412562_i32,(-937669753_i32),(-1720614716_i32)];
_1 = [2084746069_i32,(-1655177719_i32),(-682242573_i32)];
_3 = !(-24_isize);
_3 = 9223372036854775807_isize + 9223372036854775807_isize;
RET.0 = 39408_u16;
_7 = '\u{97f6f}';
_10 = [1244601246_u32,4122412808_u32,2297501516_u32,196302708_u32,1672007940_u32,31015818_u32,2652345301_u32];
Goto(bb1)
}
bb1 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
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
_3 = 26988_i16 as isize;
_12 = 1_usize as isize;
_3 = _12;
_10 = [3104064366_u32,4068068987_u32,3144956850_u32,1636720587_u32,1337174089_u32,2362935813_u32,382148402_u32];
RET.0 = !50956_u16;
_17.1 = (_7, (-1951855830_i32), (*_4), _7);
_10 = [2342852886_u32,834149817_u32,2603485730_u32,3085919102_u32,2919873130_u32,41943879_u32,1269960902_u32];
_9 = 5874348908986882338_i64 as f64;
_4 = &_17.1.2;
_7 = _17.1.3;
_4 = &_9;
_17.1 = (_7, (-717564634_i32), (*_4), _7);
_3 = _13 as isize;
_3 = !_12;
_3 = _12 + _12;
_15 = _17.1.0;
_17.1.1 = (-775417718_i32) << _3;
_3 = 314627132483845788436543434691127783220_u128 as isize;
_17.2 = 3047569538_u32 as i32;
_11 = [12299687784677852783_u64,14120049097081708606_u64,103578284251608268_u64,18388454931187337578_u64];
_12 = _3;
_4 = &_17.1.2;
_7 = _15;
_17.1.3 = _15;
_14 = _13 as f32;
_3 = _14 as isize;
_17.1.1 = !_17.2;
_17.1.2 = -_9;
Call(_18 = fn12(Move(_4), _11, _17.1.0, _15, RET.0, _12, RET.0, _9, _17.1.0, RET.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = 16136050380324147791_usize as f32;
_17.0 = (-15303_i16) ^ 28357_i16;
_9 = _17.1.2 - _17.1.2;
_17.1.2 = -_9;
_12 = _3;
_16 = &_17.0;
_15 = _17.1.0;
_17.0 = !29336_i16;
_1 = [_17.2,_17.1.1,_17.2];
_4 = &_17.1.2;
_12 = -_3;
_17.3 = -(-764906850319098874_i64);
_20.2 = -_17.0;
_17.2 = _17.1.1;
_7 = _15;
_1 = [_17.2,_17.2,_17.2];
_17.3 = !(-1286069210822267028_i64);
_21 = core::ptr::addr_of_mut!(_10);
(*_21) = [2386288721_u32,3604178576_u32,2084279723_u32,779985739_u32,4198024216_u32,2047991460_u32,3122228211_u32];
_9 = 291035129959298950813831125097570589748_u128 as f64;
(*_21) = [1995373807_u32,2539134714_u32,3758001435_u32,1862471162_u32,3171397822_u32,2519906776_u32,4220371206_u32];
_17.3 = (-668029227114083641_i64);
match _17.3 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
340282366920938463462706578204654127815 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_20.1 = core::ptr::addr_of!(_20.2);
(*_21) = [1636171081_u32,412541419_u32,502667374_u32,2085160570_u32,2463847188_u32,3649716331_u32,700925934_u32];
match _17.3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463462706578204654127815 => bb11,
_ => bb8
}
}
bb11 = {
_14 = _20.2 as f32;
_17.1.2 = _9 + _9;
(*_21) = [835449728_u32,672030988_u32,3790542250_u32,1970418384_u32,946300565_u32,2460828224_u32,2804317710_u32];
(*_21) = [2931468926_u32,422899098_u32,2607728018_u32,3678149441_u32,8498257_u32,425614607_u32,2775511673_u32];
(*_21) = [3984221987_u32,3149057629_u32,3300195332_u32,4195192084_u32,2356356213_u32,875840310_u32,1544738276_u32];
(*_21) = [3665928537_u32,1248668976_u32,3709751453_u32,4099439511_u32,1387170249_u32,1380643585_u32,1769130073_u32];
RET.1 = core::ptr::addr_of_mut!(_19);
_20.0 = 219_u8;
(*_21) = [441911448_u32,3242942644_u32,928767292_u32,3924722637_u32,4089309435_u32,3742235967_u32,3995052937_u32];
_17.1 = (_15, _17.2, _9, _15);
_4 = &_9;
_19 = _12 < _12;
(*_21) = [2625784249_u32,3279644829_u32,2278417758_u32,1223917580_u32,616755429_u32,1240043960_u32,2299495438_u32];
(*_21) = [1482996129_u32,145809129_u32,3544264466_u32,780309915_u32,3420285124_u32,3420524834_u32,4048705830_u32];
_17.2 = _17.1.1 >> _13;
_23 = 4240538595982862048_u64 as isize;
_15 = _7;
_13 = 49700870951710907241115645880971026073_i128 * (-2468154056648642718216130668064379562_i128);
_2 = [RET.0];
_24 = core::ptr::addr_of!(_20.2);
_20.1 = Move(_24);
Goto(bb12)
}
bb12 = {
(*_21) = [1501048390_u32,538713324_u32,1970118046_u32,3442310695_u32,3795796089_u32,1675456097_u32,2472139285_u32];
(*_21) = [3264616531_u32,62112618_u32,2657891590_u32,3042564577_u32,2118219614_u32,884254498_u32,283286112_u32];
_17.1.3 = _15;
(*_21) = [1161447288_u32,245561056_u32,1740177825_u32,975440686_u32,3131071525_u32,1837189_u32,332003086_u32];
(*_21) = [125093426_u32,1897995994_u32,3194319545_u32,3827920821_u32,1512971431_u32,3957039822_u32,1493563233_u32];
_3 = _12 * _12;
(*_21) = [3920076825_u32,3882227105_u32,3755866198_u32,1113439348_u32,3949496121_u32,4154775827_u32,503430680_u32];
(*_21) = [2590535223_u32,1997138462_u32,2767861778_u32,3154217340_u32,594111689_u32,3850786446_u32,3669320223_u32];
_15 = _7;
(*_21) = [540022049_u32,4104229814_u32,2938963689_u32,1036470086_u32,467535557_u32,89293487_u32,4038885917_u32];
_19 = !true;
match _17.3 {
0 => bb7,
1 => bb8,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
340282366920938463462706578204654127815 => bb18,
_ => bb17
}
}
bb13 = {
_14 = _20.2 as f32;
_17.1.2 = _9 + _9;
(*_21) = [835449728_u32,672030988_u32,3790542250_u32,1970418384_u32,946300565_u32,2460828224_u32,2804317710_u32];
(*_21) = [2931468926_u32,422899098_u32,2607728018_u32,3678149441_u32,8498257_u32,425614607_u32,2775511673_u32];
(*_21) = [3984221987_u32,3149057629_u32,3300195332_u32,4195192084_u32,2356356213_u32,875840310_u32,1544738276_u32];
(*_21) = [3665928537_u32,1248668976_u32,3709751453_u32,4099439511_u32,1387170249_u32,1380643585_u32,1769130073_u32];
RET.1 = core::ptr::addr_of_mut!(_19);
_20.0 = 219_u8;
(*_21) = [441911448_u32,3242942644_u32,928767292_u32,3924722637_u32,4089309435_u32,3742235967_u32,3995052937_u32];
_17.1 = (_15, _17.2, _9, _15);
_4 = &_9;
_19 = _12 < _12;
(*_21) = [2625784249_u32,3279644829_u32,2278417758_u32,1223917580_u32,616755429_u32,1240043960_u32,2299495438_u32];
(*_21) = [1482996129_u32,145809129_u32,3544264466_u32,780309915_u32,3420285124_u32,3420524834_u32,4048705830_u32];
_17.2 = _17.1.1 >> _13;
_23 = 4240538595982862048_u64 as isize;
_15 = _7;
_13 = 49700870951710907241115645880971026073_i128 * (-2468154056648642718216130668064379562_i128);
_2 = [RET.0];
_24 = core::ptr::addr_of!(_20.2);
_20.1 = Move(_24);
Goto(bb12)
}
bb14 = {
_20.1 = core::ptr::addr_of!(_20.2);
(*_21) = [1636171081_u32,412541419_u32,502667374_u32,2085160570_u32,2463847188_u32,3649716331_u32,700925934_u32];
match _17.3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463462706578204654127815 => bb11,
_ => bb8
}
}
bb15 = {
Return()
}
bb16 = {
_3 = 26988_i16 as isize;
_12 = 1_usize as isize;
_3 = _12;
_10 = [3104064366_u32,4068068987_u32,3144956850_u32,1636720587_u32,1337174089_u32,2362935813_u32,382148402_u32];
RET.0 = !50956_u16;
_17.1 = (_7, (-1951855830_i32), (*_4), _7);
_10 = [2342852886_u32,834149817_u32,2603485730_u32,3085919102_u32,2919873130_u32,41943879_u32,1269960902_u32];
_9 = 5874348908986882338_i64 as f64;
_4 = &_17.1.2;
_7 = _17.1.3;
_4 = &_9;
_17.1 = (_7, (-717564634_i32), (*_4), _7);
_3 = _13 as isize;
_3 = !_12;
_3 = _12 + _12;
_15 = _17.1.0;
_17.1.1 = (-775417718_i32) << _3;
_3 = 314627132483845788436543434691127783220_u128 as isize;
_17.2 = 3047569538_u32 as i32;
_11 = [12299687784677852783_u64,14120049097081708606_u64,103578284251608268_u64,18388454931187337578_u64];
_12 = _3;
_4 = &_17.1.2;
_7 = _15;
_17.1.3 = _15;
_14 = _13 as f32;
_3 = _14 as isize;
_17.1.1 = !_17.2;
_17.1.2 = -_9;
Call(_18 = fn12(Move(_4), _11, _17.1.0, _15, RET.0, _12, RET.0, _9, _17.1.0, RET.0), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
_ => bb5
}
}
bb18 = {
_11 = [11681559120003767455_u64,5257054913010345193_u64,890354767186518241_u64,326550178421054654_u64];
_20.2 = -_17.0;
_30 = _17.2 as f64;
(*_21) = [3490851119_u32,213545032_u32,2772883003_u32,3777072691_u32,1542766055_u32,3345364544_u32,1579463013_u32];
_27 = _7;
_29 = 77843951299658943734922773702178889551_u128 << _17.2;
(*_21) = [3767625872_u32,431341948_u32,4148310539_u32,2591000078_u32,3874221475_u32,581827621_u32,1037727623_u32];
_10 = [2098962067_u32,1728655696_u32,1594657647_u32,2534373603_u32,1904510484_u32,4104872412_u32,3846862980_u32];
(*_21) = [3224795714_u32,1592490721_u32,89313845_u32,257222710_u32,1163853411_u32,1138740165_u32,723056915_u32];
(*_21) = [1932881288_u32,1909819673_u32,1499522271_u32,339674689_u32,2096554148_u32,693268137_u32,2323188697_u32];
_17.1.3 = _7;
_25 = _3 + _3;
match _17.3 {
340282366920938463462706578204654127815 => bb20,
_ => bb19
}
}
bb19 = {
_3 = 26988_i16 as isize;
_12 = 1_usize as isize;
_3 = _12;
_10 = [3104064366_u32,4068068987_u32,3144956850_u32,1636720587_u32,1337174089_u32,2362935813_u32,382148402_u32];
RET.0 = !50956_u16;
_17.1 = (_7, (-1951855830_i32), (*_4), _7);
_10 = [2342852886_u32,834149817_u32,2603485730_u32,3085919102_u32,2919873130_u32,41943879_u32,1269960902_u32];
_9 = 5874348908986882338_i64 as f64;
_4 = &_17.1.2;
_7 = _17.1.3;
_4 = &_9;
_17.1 = (_7, (-717564634_i32), (*_4), _7);
_3 = _13 as isize;
_3 = !_12;
_3 = _12 + _12;
_15 = _17.1.0;
_17.1.1 = (-775417718_i32) << _3;
_3 = 314627132483845788436543434691127783220_u128 as isize;
_17.2 = 3047569538_u32 as i32;
_11 = [12299687784677852783_u64,14120049097081708606_u64,103578284251608268_u64,18388454931187337578_u64];
_12 = _3;
_4 = &_17.1.2;
_7 = _15;
_17.1.3 = _15;
_14 = _13 as f32;
_3 = _14 as isize;
_17.1.1 = !_17.2;
_17.1.2 = -_9;
Call(_18 = fn12(Move(_4), _11, _17.1.0, _15, RET.0, _12, RET.0, _9, _17.1.0, RET.0), ReturnTo(bb7), UnwindUnreachable())
}
bb20 = {
_30 = (*_4);
_20.1 = core::ptr::addr_of!(_17.0);
_11 = [14218698147997550372_u64,12984151675485136082_u64,16922959723190492842_u64,12015903262203335229_u64];
_17.1.0 = _27;
_17.1.1 = -_17.2;
(*_21) = [371514032_u32,1606877994_u32,392403202_u32,3884714928_u32,2808343594_u32,4113745689_u32,1765641855_u32];
Goto(bb21)
}
bb21 = {
_20.2 = _14 as i16;
_10 = [1594087300_u32,1203530075_u32,2433980323_u32,3008496651_u32,3811202401_u32,2266501988_u32,1114290347_u32];
_18 = [_17.2];
_24 = core::ptr::addr_of!(_20.2);
_20.2 = _17.0;
_17.1 = (_7, _17.2, (*_4), _27);
_17.1 = (_15, _17.2, (*_4), _7);
_30 = (*_4);
(*_24) = _17.0 >> _12;
_24 = core::ptr::addr_of!((*_24));
_21 = core::ptr::addr_of_mut!((*_21));
_17.2 = _17.1.1 * _17.1.1;
(*_24) = _17.0 - _17.0;
_12 = _25 | _25;
(*_21) = [1705021935_u32,3164491832_u32,3499523375_u32,3527655674_u32,3896712801_u32,1943788742_u32,2159017391_u32];
(*_21) = [2363926177_u32,2413490345_u32,3035258860_u32,2786816422_u32,4079364695_u32,3985588875_u32,904055893_u32];
(*_24) = -_17.0;
(*_24) = 607178833_u32 as i16;
RET.1 = core::ptr::addr_of_mut!(_19);
(*_21) = [2670471731_u32,100522965_u32,2885949607_u32,2392021256_u32,4144594936_u32,3982880686_u32,3152918416_u32];
(*_21) = [3742938507_u32,4127462713_u32,3329431517_u32,3720723608_u32,604939094_u32,2179559107_u32,1664349242_u32];
(*_21) = [2607533792_u32,2107616672_u32,3094198715_u32,2001129236_u32,4197596855_u32,163998392_u32,1719302878_u32];
(*_24) = _17.0 >> _12;
_3 = !_12;
(*_21) = [4281938077_u32,2723330113_u32,3946837345_u32,1051948677_u32,2688680298_u32,793277225_u32,2669697265_u32];
(*_24) = _17.0 ^ _17.0;
(*_24) = _17.0 * _17.0;
(*_21) = [211878066_u32,2473983697_u32,2900860731_u32,1453757002_u32,108806750_u32,1466288719_u32,2338008505_u32];
Goto(bb22)
}
bb22 = {
(*_24) = 10494614396197382281_usize as i16;
(*_21) = [1780831559_u32,2740566732_u32,1979650763_u32,1116364616_u32,2850354991_u32,1103777129_u32,47829252_u32];
_21 = core::ptr::addr_of_mut!((*_21));
(*_24) = _17.0;
match _20.0 {
0 => bb11,
1 => bb19,
2 => bb3,
219 => bb24,
_ => bb23
}
}
bb23 = {
_14 = _20.2 as f32;
_17.1.2 = _9 + _9;
(*_21) = [835449728_u32,672030988_u32,3790542250_u32,1970418384_u32,946300565_u32,2460828224_u32,2804317710_u32];
(*_21) = [2931468926_u32,422899098_u32,2607728018_u32,3678149441_u32,8498257_u32,425614607_u32,2775511673_u32];
(*_21) = [3984221987_u32,3149057629_u32,3300195332_u32,4195192084_u32,2356356213_u32,875840310_u32,1544738276_u32];
(*_21) = [3665928537_u32,1248668976_u32,3709751453_u32,4099439511_u32,1387170249_u32,1380643585_u32,1769130073_u32];
RET.1 = core::ptr::addr_of_mut!(_19);
_20.0 = 219_u8;
(*_21) = [441911448_u32,3242942644_u32,928767292_u32,3924722637_u32,4089309435_u32,3742235967_u32,3995052937_u32];
_17.1 = (_15, _17.2, _9, _15);
_4 = &_9;
_19 = _12 < _12;
(*_21) = [2625784249_u32,3279644829_u32,2278417758_u32,1223917580_u32,616755429_u32,1240043960_u32,2299495438_u32];
(*_21) = [1482996129_u32,145809129_u32,3544264466_u32,780309915_u32,3420285124_u32,3420524834_u32,4048705830_u32];
_17.2 = _17.1.1 >> _13;
_23 = 4240538595982862048_u64 as isize;
_15 = _7;
_13 = 49700870951710907241115645880971026073_i128 * (-2468154056648642718216130668064379562_i128);
_2 = [RET.0];
_24 = core::ptr::addr_of!(_20.2);
_20.1 = Move(_24);
Goto(bb12)
}
bb24 = {
_23 = _3;
_17.1.2 = (*_4);
_20.1 = core::ptr::addr_of!((*_24));
(*_21) = [768906346_u32,1504535050_u32,495655959_u32,2820775703_u32,1285442819_u32,2886335830_u32,2235145660_u32];
_32 = _17.1.0 as isize;
_35 = _1;
_32 = !_12;
(*_24) = _17.0 ^ _17.0;
_18 = [_17.1.1];
_39 = _19;
_33 = core::ptr::addr_of_mut!(RET.0);
_23 = _29 as isize;
_17.3 = (-2032296298817431209_i64);
(*_24) = 1205691543_u32 as i16;
(*_21) = [2508611870_u32,1173723228_u32,3616411892_u32,3069807668_u32,2285606262_u32,3956144969_u32,121880591_u32];
(*_33) = !1676_u16;
(*_33) = !56089_u16;
(*_21) = [1667463980_u32,1403540862_u32,739948966_u32,3719085827_u32,1841738409_u32,1299629375_u32,3408448409_u32];
_20 = (119_u8, Move(_24), _17.0);
_36 = _17.0 - _20.2;
_16 = &_20.2;
_30 = (*_4);
(*_21) = [23189051_u32,2407925491_u32,3635204182_u32,459721046_u32,2857418415_u32,4211233658_u32,2460045416_u32];
(*_33) = 13313_u16 + 46135_u16;
_17.1.2 = -(*_4);
(*_21) = [4286061518_u32,2157041150_u32,500082134_u32,1832229556_u32,3244658311_u32,1858386542_u32,2369668220_u32];
match _20.0 {
0 => bb14,
1 => bb11,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
119 => bb30,
_ => bb29
}
}
bb25 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
_ => bb5
}
}
bb26 = {
Return()
}
bb27 = {
_20.2 = _14 as i16;
_10 = [1594087300_u32,1203530075_u32,2433980323_u32,3008496651_u32,3811202401_u32,2266501988_u32,1114290347_u32];
_18 = [_17.2];
_24 = core::ptr::addr_of!(_20.2);
_20.2 = _17.0;
_17.1 = (_7, _17.2, (*_4), _27);
_17.1 = (_15, _17.2, (*_4), _7);
_30 = (*_4);
(*_24) = _17.0 >> _12;
_24 = core::ptr::addr_of!((*_24));
_21 = core::ptr::addr_of_mut!((*_21));
_17.2 = _17.1.1 * _17.1.1;
(*_24) = _17.0 - _17.0;
_12 = _25 | _25;
(*_21) = [1705021935_u32,3164491832_u32,3499523375_u32,3527655674_u32,3896712801_u32,1943788742_u32,2159017391_u32];
(*_21) = [2363926177_u32,2413490345_u32,3035258860_u32,2786816422_u32,4079364695_u32,3985588875_u32,904055893_u32];
(*_24) = -_17.0;
(*_24) = 607178833_u32 as i16;
RET.1 = core::ptr::addr_of_mut!(_19);
(*_21) = [2670471731_u32,100522965_u32,2885949607_u32,2392021256_u32,4144594936_u32,3982880686_u32,3152918416_u32];
(*_21) = [3742938507_u32,4127462713_u32,3329431517_u32,3720723608_u32,604939094_u32,2179559107_u32,1664349242_u32];
(*_21) = [2607533792_u32,2107616672_u32,3094198715_u32,2001129236_u32,4197596855_u32,163998392_u32,1719302878_u32];
(*_24) = _17.0 >> _12;
_3 = !_12;
(*_21) = [4281938077_u32,2723330113_u32,3946837345_u32,1051948677_u32,2688680298_u32,793277225_u32,2669697265_u32];
(*_24) = _17.0 ^ _17.0;
(*_24) = _17.0 * _17.0;
(*_21) = [211878066_u32,2473983697_u32,2900860731_u32,1453757002_u32,108806750_u32,1466288719_u32,2338008505_u32];
Goto(bb22)
}
bb28 = {
_30 = (*_4);
_20.1 = core::ptr::addr_of!(_17.0);
_11 = [14218698147997550372_u64,12984151675485136082_u64,16922959723190492842_u64,12015903262203335229_u64];
_17.1.0 = _27;
_17.1.1 = -_17.2;
(*_21) = [371514032_u32,1606877994_u32,392403202_u32,3884714928_u32,2808343594_u32,4113745689_u32,1765641855_u32];
Goto(bb21)
}
bb29 = {
Return()
}
bb30 = {
_20.0 = 4_usize as u8;
_24 = core::ptr::addr_of!(_17.0);
_13 = 125786400641165627473149377096281710618_i128 * (-140096803572189073696272766255743263499_i128);
_41 = !_19;
_30 = (*_4) - (*_4);
_20.0 = 225_u8 - 48_u8;
_21 = core::ptr::addr_of_mut!((*_21));
_12 = _3;
_17.0 = (*_16) - (*_16);
(*_33) = 46095_u16 & 3330_u16;
(*_21) = [2100622792_u32,3025546200_u32,1692444863_u32,3178921130_u32,1934486819_u32,2163935690_u32,2468325894_u32];
(*_24) = !(*_16);
(*_21) = [3968558138_u32,3081518458_u32,3003402893_u32,3317981827_u32,3521525869_u32,2502481781_u32,2490521976_u32];
(*_33) = 42688_u16;
(*_24) = (*_16);
(*_33) = 5125_u16;
_45 = _14;
match (*_33) {
0 => bb14,
1 => bb15,
2 => bb31,
3 => bb32,
5125 => bb34,
_ => bb33
}
}
bb31 = {
Return()
}
bb32 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
_ => bb5
}
}
bb33 = {
_11 = [11681559120003767455_u64,5257054913010345193_u64,890354767186518241_u64,326550178421054654_u64];
_20.2 = -_17.0;
_30 = _17.2 as f64;
(*_21) = [3490851119_u32,213545032_u32,2772883003_u32,3777072691_u32,1542766055_u32,3345364544_u32,1579463013_u32];
_27 = _7;
_29 = 77843951299658943734922773702178889551_u128 << _17.2;
(*_21) = [3767625872_u32,431341948_u32,4148310539_u32,2591000078_u32,3874221475_u32,581827621_u32,1037727623_u32];
_10 = [2098962067_u32,1728655696_u32,1594657647_u32,2534373603_u32,1904510484_u32,4104872412_u32,3846862980_u32];
(*_21) = [3224795714_u32,1592490721_u32,89313845_u32,257222710_u32,1163853411_u32,1138740165_u32,723056915_u32];
(*_21) = [1932881288_u32,1909819673_u32,1499522271_u32,339674689_u32,2096554148_u32,693268137_u32,2323188697_u32];
_17.1.3 = _7;
_25 = _3 + _3;
match _17.3 {
340282366920938463462706578204654127815 => bb20,
_ => bb19
}
}
bb34 = {
(*_33) = !3605_u16;
_2 = [(*_33)];
(*_24) = _29 as i16;
(*_33) = 11652_u16;
_38 = _17.3;
_30 = (*_33) as f64;
(*_24) = (*_16) + (*_16);
_40.2 = [_23,_32,_3,_12,_25];
(*_33) = _38 as u16;
_10 = [851285820_u32,3766616067_u32,2786280903_u32,825793139_u32,365368645_u32,918092227_u32,4163083192_u32];
_23 = _41 as isize;
(*_24) = _38 as i16;
(*_33) = (*_16) as u16;
_30 = (*_4);
(*_24) = (*_16) >> _32;
_50 = (*_24) < (*_24);
_17.3 = _38 ^ _38;
(*_24) = _17.3 as i16;
(*_21) = [2718279311_u32,3114625318_u32,3470558097_u32,3956108161_u32,1631771968_u32,3033273870_u32,3105621457_u32];
_33 = core::ptr::addr_of_mut!((*_33));
RET.0 = !33220_u16;
(*_24) = (*_16) + (*_16);
(*_33) = 60345_u16 * 29970_u16;
(*_24) = _36 & _36;
(*_21) = [3210200436_u32,1108277740_u32,3038002309_u32,180441879_u32,200578519_u32,4092156940_u32,4252534098_u32];
_52 = 1615733945_u32 >> _29;
_46 = _20.0 as i64;
Goto(bb35)
}
bb35 = {
(*_24) = !_36;
_20 = (111_u8, Move(_24), (*_24));
_13 = !(-85808054151642798244210801103478810547_i128);
_3 = !_32;
_34 = _32 * _12;
_53 = [_50,_50,_50,_19,_50,_50];
_2 = [(*_33)];
(*_21) = [_52,_52,_52,_52,_52,_52,_52];
_39 = !_50;
(*_21) = [_52,_52,_52,_52,_52,_52,_52];
(*_21) = [_52,_52,_52,_52,_52,_52,_52];
_25 = _27 as isize;
_51 = _20.0 as isize;
Goto(bb36)
}
bb36 = {
_17.2 = -_17.1.1;
(*_33) = 54024_u16 + 37705_u16;
RET.1 = core::ptr::addr_of_mut!(_19);
_21 = core::ptr::addr_of_mut!((*_21));
_3 = _34 + _12;
(*_33) = !21751_u16;
_46 = !_17.3;
_54 = (*_33) as u128;
_29 = _45 as u128;
(*_33) = 738_u16 + 16856_u16;
_10 = [_52,_52,_52,_52,_52,_52,_52];
_30 = (*_4);
match _20.0 {
0 => bb21,
1 => bb35,
2 => bb10,
3 => bb12,
4 => bb28,
5 => bb37,
111 => bb39,
_ => bb38
}
}
bb37 = {
_20.1 = core::ptr::addr_of!(_20.2);
(*_21) = [1636171081_u32,412541419_u32,502667374_u32,2085160570_u32,2463847188_u32,3649716331_u32,700925934_u32];
match _17.3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463462706578204654127815 => bb11,
_ => bb8
}
}
bb38 = {
Return()
}
bb39 = {
(*_21) = [_52,_52,_52,_52,_52,_52,_52];
_4 = &_30;
_10 = [_52,_52,_52,_52,_52,_52,_52];
(*_21) = [_52,_52,_52,_52,_52,_52,_52];
_8 = core::ptr::addr_of!(_52);
match _38 {
0 => bb37,
1 => bb21,
2 => bb16,
3 => bb20,
4 => bb15,
5 => bb40,
6 => bb41,
340282366920938463461342311132950780247 => bb43,
_ => bb42
}
}
bb40 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
_ => bb5
}
}
bb41 = {
_3 = 26988_i16 as isize;
_12 = 1_usize as isize;
_3 = _12;
_10 = [3104064366_u32,4068068987_u32,3144956850_u32,1636720587_u32,1337174089_u32,2362935813_u32,382148402_u32];
RET.0 = !50956_u16;
_17.1 = (_7, (-1951855830_i32), (*_4), _7);
_10 = [2342852886_u32,834149817_u32,2603485730_u32,3085919102_u32,2919873130_u32,41943879_u32,1269960902_u32];
_9 = 5874348908986882338_i64 as f64;
_4 = &_17.1.2;
_7 = _17.1.3;
_4 = &_9;
_17.1 = (_7, (-717564634_i32), (*_4), _7);
_3 = _13 as isize;
_3 = !_12;
_3 = _12 + _12;
_15 = _17.1.0;
_17.1.1 = (-775417718_i32) << _3;
_3 = 314627132483845788436543434691127783220_u128 as isize;
_17.2 = 3047569538_u32 as i32;
_11 = [12299687784677852783_u64,14120049097081708606_u64,103578284251608268_u64,18388454931187337578_u64];
_12 = _3;
_4 = &_17.1.2;
_7 = _15;
_17.1.3 = _15;
_14 = _13 as f32;
_3 = _14 as isize;
_17.1.1 = !_17.2;
_17.1.2 = -_9;
Call(_18 = fn12(Move(_4), _11, _17.1.0, _15, RET.0, _12, RET.0, _9, _17.1.0, RET.0), ReturnTo(bb7), UnwindUnreachable())
}
bb42 = {
Return()
}
bb43 = {
_61.0 = _20.0 ^ _20.0;
(*_33) = !35368_u16;
_13 = 85600821869656324021074818839748309268_i128;
_17.2 = -_17.1.1;
_9 = (*_4) * (*_4);
_61 = (_20.0, Move(_20.1), _20.2);
_29 = _54 - _54;
(*_8) = 4046058954_u32 + 834714841_u32;
(*_33) = !58452_u16;
(*_8) = _39 as u32;
_7 = _27;
_63 = !_50;
_38 = _17.3 ^ _17.3;
_21 = core::ptr::addr_of_mut!((*_21));
(*_33) = !61322_u16;
_19 = _39;
(*_33) = 15969_u16;
_3 = (*_33) as isize;
_45 = _14;
_17.1.0 = _27;
_17.1.0 = _27;
_40.2 = [_12,_34,_12,_34,_12];
_20 = (_61.0, Move(_61.1), _17.0);
match (*_33) {
0 => bb44,
15969 => bb46,
_ => bb45
}
}
bb44 = {
Return()
}
bb45 = {
_12 = 94_i8 as isize;
_1 = [(-2104938992_i32),1926046531_i32,648429833_i32];
_10 = [3802800239_u32,2945276176_u32,881844814_u32,2328157039_u32,2406714298_u32,4282482931_u32,1595687484_u32];
_2 = [RET.0];
_11 = [8305701138050851776_u64,8628274269802182025_u64,18122274879770625842_u64,10417495313086176996_u64];
_1 = [1849834518_i32,(-2016304707_i32),(-1463463889_i32)];
_10 = [1561777131_u32,2337278834_u32,1213769008_u32,2756641917_u32,4184339809_u32,282151988_u32,1670396214_u32];
_7 = '\u{fc59b}';
_10 = [3634766794_u32,2765162746_u32,577983277_u32,4051841219_u32,2276799511_u32,2907391585_u32,954824466_u32];
_13 = (-108542184858773941237675937427265022387_i128) ^ (-122838962321025235157816948445311323199_i128);
_12 = 618396382_u32 as isize;
_10 = [2703379304_u32,690027590_u32,827528153_u32,2543606749_u32,98304759_u32,1183151332_u32,2298431102_u32];
_9 = (-2105531591_i32) as f64;
_12 = _3;
_4 = &_9;
_3 = !_12;
_14 = _13 as f32;
_12 = _14 as isize;
_13 = (-92442811409799288572044210658828623047_i128) >> _3;
_10 = [3068079550_u32,441596695_u32,3011173257_u32,1504664049_u32,4257546530_u32,1234401245_u32,2238543840_u32];
_12 = _3 * _3;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
39408 => bb6,
_ => bb5
}
}
bb46 = {
(*_33) = 53956_u16 >> (*_8);
Goto(bb47)
}
bb47 = {
Call(_67 = dump_var(Move(_23), Move(_41), Move(_12), Move(_19)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_67 = dump_var(Move(_1), Move(_51), Move(_13), Move(_53)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_67 = dump_var(Move(_15), Move(_50), Move(_3), Move(_18)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_67 = dump_var(Move(_2), Move(_27), _68, _68), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: &'static f64,mut _2: [u64; 4],mut _3: char,mut _4: char,mut _5: u16,mut _6: isize,mut _7: u16,mut _8: f64,mut _9: char,mut _10: u16) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _11: *mut bool;
let _12: [i32; 3];
let _13: *mut bool;
let _14: *mut &'static mut *const u32;
let _15: *mut [u32; 7];
let _16: usize;
let _17: *const f32;
let _18: u32;
let _19: isize;
let _20: (*const &'static f64, char);
let _21: f32;
let _22: &'static f64;
let _23: u64;
let _24: i8;
let _25: [isize; 5];
let _26: &'static u16;
let _27: i16;
let _28: bool;
let _29: *const u32;
let _30: bool;
let _31: bool;
let _32: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _33: i64;
let _34: char;
let _35: u64;
let _36: f32;
let _37: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _38: &'static (Adt58,);
let _39: i8;
let _40: f64;
let _41: i128;
let _42: char;
let _43: &'static u128;
let _44: i8;
let _45: f64;
let _46: i128;
let _47: f32;
let _48: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _49: &'static isize;
let _50: [i128; 5];
let _51: u16;
let _52: *mut bool;
let _53: (*mut bool,);
let _54: *mut bool;
let _55: char;
let _56: *const i16;
let _57: u8;
let _58: &'static u128;
let _59: *mut u16;
let _60: *mut *mut &'static Adt18;
let _61: u8;
let _62: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _63: *const i16;
let _64: *mut *mut &'static Adt18;
let _65: f32;
let _66: isize;
let _67: &'static (Adt58,);
let _68: u8;
let _69: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _70: isize;
let _71: isize;
let _72: isize;
let _73: f64;
let _74: i8;
let _75: f32;
let _76: i128;
let _77: u16;
let _78: [u32; 7];
let _79: &'static u16;
let _80: bool;
let _81: &'static f64;
let _82: Adt18;
let _83: [u64; 3];
let _84: usize;
let _85: *const [u16; 1];
let _86: &'static mut usize;
let _87: char;
let _88: (i16, (char, i32, f64, char), i32, i64);
let _89: i8;
let _90: isize;
let _91: Adt31;
let _92: (u8, *const i16, i16);
let _93: [i32; 1];
let _94: bool;
let _95: *mut *const [i64; 6];
let _96: (i16, (char, i32, f64, char), i32, i64);
let _97: *const u32;
let _98: u32;
let _99: bool;
let _100: Adt78;
let _101: f64;
let _102: Adt51;
let _103: isize;
let _104: bool;
let _105: isize;
let _106: isize;
let _107: ();
let _108: ();
{
_1 = &_8;
_5 = (-762497013_i32) as u16;
_4 = _3;
_2 = [10166850799897489092_u64,14844848928910847015_u64,16280665816641883628_u64,6162068885249785410_u64];
_9 = _4;
RET = [1978171923_i32];
_4 = _9;
_4 = _9;
_12 = [(-1515739891_i32),(-1458091526_i32),(-1561222323_i32)];
_8 = _5 as f64;
_2 = [4040429313510196085_u64,5153719465006994403_u64,4674370679044427408_u64,4792476259240145731_u64];
_6 = (-9223372036854775808_isize);
_6 = (-9223372036854775808_isize);
_12 = [(-1121110445_i32),1414132008_i32,1655228018_i32];
_5 = 4032534836870648257_u64 as u16;
Goto(bb1)
}
bb1 = {
RET = [1600181121_i32];
_5 = !_7;
_6 = false as isize;
_12 = [(-1050637923_i32),(-383511295_i32),(-905020265_i32)];
_4 = _9;
_8 = 249_u8 as f64;
_8 = 68760513418352077169224477311906108313_u128 as f64;
_10 = 221323034832296743998109015516580312286_u128 as u16;
_10 = _5 + _7;
_2 = [15726189765404685651_u64,1445775280671992051_u64,2740360623416478989_u64,6272160739986087776_u64];
_8 = 22_u8 as f64;
Goto(bb2)
}
bb2 = {
_3 = _9;
_8 = _10 as f64;
_8 = 151292990609216964033580511246610229448_i128 as f64;
_12 = [(-2048789560_i32),1660858849_i32,251470225_i32];
_19 = -_6;
_6 = _19 * _19;
_5 = !_10;
_8 = (-76008244997265368306586430552413174019_i128) as f64;
_2 = [12178863009716464156_u64,9665896028779116990_u64,3649520259033345871_u64,1274917648740190614_u64];
_19 = _6 & _6;
_18 = 4008570247_u32 << _5;
_5 = (-3658906931531767257_i64) as u16;
Call(_2 = fn13(_9, _4, _8, Move(_1), _4, _9, _4, _19, _8, _5, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _4;
_3 = _9;
_18 = 367750929_u32 ^ 3048275516_u32;
_8 = 2018871080259778568_i64 as f64;
_8 = (-45_i8) as f64;
_20.1 = _9;
_12 = [1732836916_i32,(-683023380_i32),194625715_i32];
_2 = [4156167391438847358_u64,1968838220085998641_u64,10829927840579030258_u64,14010880953450127050_u64];
_7 = _10 + _10;
_2 = [6609784071738174648_u64,12033656752854870843_u64,595951574221863049_u64,11260016441390865296_u64];
_17 = core::ptr::addr_of!(_21);
_17 = core::ptr::addr_of!((*_17));
(*_17) = 2122435310_i32 as f32;
RET = [(-1550670066_i32)];
Goto(bb4)
}
bb4 = {
(*_17) = (-131703126754413099600551399705659247731_i128) as f32;
_20.1 = _9;
_21 = _6 as f32;
_16 = (*_17) as usize;
_6 = _19;
_12 = [(-1123745995_i32),(-820100304_i32),(-1856491803_i32)];
(*_17) = 338524297547463339466950894358338616979_u128 as f32;
(*_17) = 8964294852803792255490191623242418607_u128 as f32;
_19 = _6 << _6;
(*_17) = _19 as f32;
(*_17) = _16 as f32;
_24 = (-44_i8);
_18 = 2787295629_u32 ^ 2340617732_u32;
_19 = !_6;
_13 = core::ptr::addr_of_mut!(_28);
match _24 {
0 => bb5,
1 => bb6,
340282366920938463463374607431768211412 => bb8,
_ => bb7
}
}
bb5 = {
_9 = _4;
_3 = _9;
_18 = 367750929_u32 ^ 3048275516_u32;
_8 = 2018871080259778568_i64 as f64;
_8 = (-45_i8) as f64;
_20.1 = _9;
_12 = [1732836916_i32,(-683023380_i32),194625715_i32];
_2 = [4156167391438847358_u64,1968838220085998641_u64,10829927840579030258_u64,14010880953450127050_u64];
_7 = _10 + _10;
_2 = [6609784071738174648_u64,12033656752854870843_u64,595951574221863049_u64,11260016441390865296_u64];
_17 = core::ptr::addr_of!(_21);
_17 = core::ptr::addr_of!((*_17));
(*_17) = 2122435310_i32 as f32;
RET = [(-1550670066_i32)];
Goto(bb4)
}
bb6 = {
_3 = _9;
_8 = _10 as f64;
_8 = 151292990609216964033580511246610229448_i128 as f64;
_12 = [(-2048789560_i32),1660858849_i32,251470225_i32];
_19 = -_6;
_6 = _19 * _19;
_5 = !_10;
_8 = (-76008244997265368306586430552413174019_i128) as f64;
_2 = [12178863009716464156_u64,9665896028779116990_u64,3649520259033345871_u64,1274917648740190614_u64];
_19 = _6 & _6;
_18 = 4008570247_u32 << _5;
_5 = (-3658906931531767257_i64) as u16;
Call(_2 = fn13(_9, _4, _8, Move(_1), _4, _9, _4, _19, _8, _5, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = [1600181121_i32];
_5 = !_7;
_6 = false as isize;
_12 = [(-1050637923_i32),(-383511295_i32),(-905020265_i32)];
_4 = _9;
_8 = 249_u8 as f64;
_8 = 68760513418352077169224477311906108313_u128 as f64;
_10 = 221323034832296743998109015516580312286_u128 as u16;
_10 = _5 + _7;
_2 = [15726189765404685651_u64,1445775280671992051_u64,2740360623416478989_u64,6272160739986087776_u64];
_8 = 22_u8 as f64;
Goto(bb2)
}
bb8 = {
_5 = !_7;
Goto(bb9)
}
bb9 = {
_26 = &_7;
(*_13) = _24 <= _24;
_28 = false;
_23 = 6386205951188751739_u64 | 2317735820417868754_u64;
(*_17) = (-3480082_i32) as f32;
_6 = _19 << (*_26);
_21 = _8 as f32;
_24 = !(-108_i8);
(*_17) = _16 as f32;
(*_13) = _21 <= (*_17);
(*_17) = _16 as f32;
_3 = _4;
(*_13) = !false;
(*_17) = _8 as f32;
(*_13) = (*_26) == (*_26);
(*_13) = !false;
(*_13) = (*_26) >= _5;
_11 = core::ptr::addr_of_mut!((*_13));
_11 = core::ptr::addr_of_mut!((*_13));
(*_13) = true;
(*_17) = (-866250126_i32) as f32;
(*_17) = (-7756453413108396302_i64) as f32;
(*_13) = (*_17) < (*_17);
(*_13) = (*_26) == (*_26);
Goto(bb10)
}
bb10 = {
_6 = _19 | _19;
(*_13) = false & true;
(*_13) = true | false;
(*_17) = _23 as f32;
Goto(bb11)
}
bb11 = {
_7 = !_5;
(*_13) = (*_17) > (*_17);
(*_13) = false;
_17 = core::ptr::addr_of!((*_17));
(*_17) = _8 as f32;
(*_17) = _6 as f32;
(*_17) = _8 as f32;
(*_13) = false & true;
_30 = _9 > _9;
_19 = _5 as isize;
_29 = core::ptr::addr_of!(_18);
_2 = [_23,_23,_23,_23];
(*_29) = 2335339005_u32 - 2329743595_u32;
_25 = [_6,_6,_6,_6,_6];
(*_17) = 190_u8 as f32;
(*_17) = _8 as f32;
_19 = !_6;
(*_29) = 64219740_u32 - 219515733_u32;
(*_29) = 4098413819_u32 ^ 3821350214_u32;
Goto(bb12)
}
bb12 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb13 = {
_36 = (*_17);
_21 = _36 * _36;
_28 = _31 & _30;
_25 = [_19,_19,_6,_19,_19];
(*_17) = -_36;
(*_13) = _30;
(*_29) = 3496843506_u32;
(*_13) = !_30;
(*_17) = _23 as f32;
(*_13) = !_30;
_31 = (*_29) >= (*_29);
(*_13) = _30;
(*_13) = !_31;
(*_17) = _36 - _36;
Goto(bb14)
}
bb14 = {
_29 = core::ptr::addr_of!((*_29));
(*_17) = _36 - _36;
_23 = 18256966882041521203_u64 | 2278899978199393917_u64;
(*_29) = 553023774_u32 * 381649086_u32;
_33 = 6521791991527710629_i64;
RET = [(-2060362854_i32)];
(*_29) = 3685179212_u32;
Goto(bb15)
}
bb15 = {
(*_13) = _30 & _30;
(*_29) = 726142224_u32 * 3448156302_u32;
(*_17) = _36 * _36;
_22 = &(*_1);
_13 = core::ptr::addr_of_mut!((*_13));
_28 = _19 <= _6;
_35 = _33 as u64;
(*_17) = _36;
_46 = (-157228948125456134015588938816361101175_i128);
(*_13) = _30;
_6 = !_19;
(*_13) = _31;
(*_29) = 144076234_u32;
(*_29) = 2232174275_u32 & 1124233980_u32;
(*_13) = !_30;
(*_17) = _36 + _36;
_20.0 = core::ptr::addr_of!(_1);
(*_13) = !_31;
Goto(bb16)
}
bb16 = {
(*_13) = _30 ^ _30;
(*_13) = (*_29) > (*_29);
_41 = _46 & _46;
(*_29) = 4115580005_u32 | 254200143_u32;
(*_13) = _31;
Goto(bb17)
}
bb17 = {
_44 = _24 >> (*_29);
_51 = _7 << _6;
(*_29) = (*_17) as u32;
(*_17) = _23 as f32;
(*_13) = (*_29) <= (*_29);
(*_29) = 2241480880_u32 << _6;
_47 = _27 as f32;
_41 = _46;
_34 = _3;
(*_13) = _31;
(*_17) = -_47;
Goto(bb18)
}
bb18 = {
(*_17) = _47 - _47;
(*_13) = (*_1) >= (*_1);
(*_13) = !_31;
(*_29) = _20.1 as u32;
(*_29) = 2247569497_u32;
_23 = !_35;
(*_29) = 4240652195_u32;
(*_13) = _30;
(*_13) = _31 ^ _30;
_3 = _9;
(*_17) = _47 * _47;
(*_29) = 1419746467_u32 + 1674327581_u32;
_36 = 125_u8 as f32;
_31 = !(*_13);
RET = [63616497_i32];
(*_13) = _31;
(*_17) = (*_22) as f32;
_11 = core::ptr::addr_of_mut!(_30);
match _41 {
183053418795482329447785668615407110281 => bb19,
_ => bb3
}
}
bb19 = {
(*_29) = 3982183215_u32 * 2561808599_u32;
(*_29) = 2844087496_u32 & 2976206303_u32;
_42 = _20.1;
(*_11) = !(*_13);
(*_29) = !1291318096_u32;
_20.1 = _3;
_45 = -(*_1);
_50 = [_46,_41,_41,_41,_41];
(*_11) = (*_13);
(*_13) = !(*_11);
_9 = _4;
(*_29) = !1744682899_u32;
(*_13) = (*_11) < (*_11);
(*_11) = !(*_13);
(*_17) = _16 as f32;
_41 = _46;
(*_13) = (*_11) & (*_11);
_25 = [_6,_6,_19,_19,_6];
(*_13) = (*_11) ^ _30;
_6 = _19 ^ _19;
(*_29) = 630740296_u32;
(*_17) = _47 + _47;
(*_13) = (*_11);
_28 = (*_17) <= (*_17);
match (*_29) {
0 => bb18,
630740296 => bb20,
_ => bb12
}
}
bb20 = {
(*_13) = !(*_11);
_12 = [63043916_i32,(-1045086744_i32),(-1725210733_i32)];
_22 = &_45;
(*_29) = !2230683740_u32;
_31 = (*_13);
_40 = (*_1);
_21 = _47 - _36;
(*_11) = !(*_13);
(*_13) = (*_11);
(*_29) = !1134104348_u32;
(*_29) = 143_u8 as u32;
_55 = _34;
(*_11) = (*_13) ^ (*_13);
(*_29) = (*_22) as u32;
match _33 {
0 => bb6,
1 => bb13,
2 => bb21,
6521791991527710629 => bb23,
_ => bb22
}
}
bb21 = {
_5 = !_7;
Goto(bb9)
}
bb22 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb23 = {
(*_29) = 1204807775_u32 >> _10;
_1 = Move(_22);
_45 = _8;
(*_17) = _36 + _36;
(*_11) = (*_13) & (*_13);
_55 = _9;
(*_17) = _44 as f32;
(*_29) = 1569761727_u32 >> _44;
_6 = _19 << (*_29);
_53 = (Move(_11),);
_28 = _30;
(*_29) = 1997508427_u32;
_56 = core::ptr::addr_of!(_27);
(*_17) = _47;
(*_56) = !(-11294_i16);
_54 = core::ptr::addr_of_mut!((*_13));
(*_17) = -_36;
(*_13) = _30;
(*_17) = _36 * _36;
(*_56) = 2393_i16;
_26 = &_10;
(*_29) = 4173199113_u32;
match (*_56) {
0 => bb10,
1 => bb6,
2393 => bb24,
_ => bb14
}
}
bb24 = {
_23 = !_35;
_66 = _19 ^ _19;
_53 = (Move(_13),);
_16 = (-825018336_i32) as usize;
RET = [(-1552834630_i32)];
_23 = _35;
(*_17) = -_47;
(*_17) = _47;
_54 = core::ptr::addr_of_mut!(_31);
_1 = &_8;
(*_29) = _44 as u32;
(*_54) = !_30;
_20.1 = _4;
_59 = core::ptr::addr_of_mut!((*_26));
(*_29) = 4048113023_u32 + 993460405_u32;
(*_54) = (*_29) > (*_29);
_4 = _9;
_61 = 147_u8 >> _6;
(*_29) = 2950628783_u32 + 3556349495_u32;
(*_29) = _55 as u32;
(*_54) = _30;
_53.0 = Move(_54);
(*_17) = _36 * _36;
(*_29) = 2835106460_u32;
_27 = (-40_i16);
match (*_56) {
0 => bb7,
1 => bb16,
340282366920938463463374607431768211416 => bb26,
_ => bb25
}
}
bb25 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb26 = {
(*_29) = 1022286515_u32 << (*_56);
_20.0 = core::ptr::addr_of!(_1);
_1 = &_45;
(*_29) = 4105941676_u32 ^ 986104528_u32;
_5 = (*_59) + (*_26);
(*_17) = _16 as f32;
(*_56) = 24337_i16 | (-14648_i16);
_68 = _61 | _61;
(*_17) = -_36;
(*_17) = _68 as f32;
_50 = [_46,_41,_41,_46,_41];
_22 = &(*_1);
_63 = core::ptr::addr_of!((*_56));
(*_63) = (*_29) as i16;
(*_63) = -5723_i16;
(*_56) = (-7867_i16) & (-20794_i16);
_73 = (*_22) * (*_22);
_74 = _24;
match _33 {
0 => bb1,
1 => bb17,
2 => bb24,
3 => bb10,
4 => bb5,
5 => bb19,
6 => bb9,
6521791991527710629 => bb27,
_ => bb8
}
}
bb27 = {
_73 = (*_22) + (*_22);
_39 = _44 - _44;
_71 = -_19;
_51 = (*_26);
RET = [522765704_i32];
(*_17) = -_36;
_25 = [_66,_19,_66,_71,_19];
_36 = (*_17);
(*_17) = _36 * _47;
(*_17) = -_47;
_50 = [_41,_46,_41,_46,_41];
(*_56) = (*_29) as i16;
(*_56) = (-12436_i16) << _6;
_56 = core::ptr::addr_of!((*_56));
_21 = _47 * _47;
(*_29) = _23 as u32;
_21 = -_47;
_41 = _46;
(*_29) = 2509206882_u32;
_30 = !_31;
(*_56) = !(-8635_i16);
match (*_29) {
0 => bb21,
1 => bb17,
2 => bb11,
3 => bb9,
4 => bb28,
2509206882 => bb30,
_ => bb29
}
}
bb28 = {
_26 = &_7;
(*_13) = _24 <= _24;
_28 = false;
_23 = 6386205951188751739_u64 | 2317735820417868754_u64;
(*_17) = (-3480082_i32) as f32;
_6 = _19 << (*_26);
_21 = _8 as f32;
_24 = !(-108_i8);
(*_17) = _16 as f32;
(*_13) = _21 <= (*_17);
(*_17) = _16 as f32;
_3 = _4;
(*_13) = !false;
(*_17) = _8 as f32;
(*_13) = (*_26) == (*_26);
(*_13) = !false;
(*_13) = (*_26) >= _5;
_11 = core::ptr::addr_of_mut!((*_13));
_11 = core::ptr::addr_of_mut!((*_13));
(*_13) = true;
(*_17) = (-866250126_i32) as f32;
(*_17) = (-7756453413108396302_i64) as f32;
(*_13) = (*_17) < (*_17);
(*_13) = (*_26) == (*_26);
Goto(bb10)
}
bb29 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb30 = {
_8 = (*_1) + (*_1);
(*_29) = !3478430839_u32;
_20.1 = _4;
_1 = Move(_22);
_29 = core::ptr::addr_of!((*_29));
(*_56) = (-25507_i16) | (-18180_i16);
_18 = 168092558166287205176642623366126118974_u128 as u32;
(*_17) = -_47;
Call((*_17) = core::intrinsics::transmute((*_29)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_59 = core::ptr::addr_of_mut!((*_59));
_35 = _16 as u64;
(*_56) = _16 as i16;
(*_17) = _66 as f32;
_22 = &_45;
_21 = -_36;
(*_17) = _23 as f32;
_65 = (*_17) - (*_17);
_49 = &_19;
Goto(bb32)
}
bb32 = {
_21 = _47;
_12 = [1868969374_i32,(-321402311_i32),(-1731413430_i32)];
(*_56) = _33 as i16;
_78 = [(*_29),_18,(*_29),_18,(*_29),(*_29),(*_29)];
_1 = &(*_22);
(*_17) = _65 - _65;
_36 = -(*_17);
_41 = _46 >> (*_26);
match _33 {
0 => bb20,
1 => bb10,
2 => bb16,
3 => bb30,
4 => bb23,
5 => bb13,
6521791991527710629 => bb34,
_ => bb33
}
}
bb33 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb34 = {
_5 = (*_59) * (*_59);
_76 = (*_29) as i128;
(*_29) = !1031728450_u32;
(*_29) = 2134534185_u32 >> (*_49);
_49 = &_6;
(*_17) = -_47;
(*_17) = _16 as f32;
_66 = _71 * (*_49);
(*_17) = _36 + _65;
_66 = _42 as isize;
match _33 {
0 => bb15,
1 => bb14,
2 => bb35,
3 => bb36,
4 => bb37,
6521791991527710629 => bb39,
_ => bb38
}
}
bb35 = {
_31 = !(*_13);
_1 = &_8;
_27 = (-17367_i16) - 27747_i16;
Goto(bb13)
}
bb36 = {
_26 = &_7;
(*_13) = _24 <= _24;
_28 = false;
_23 = 6386205951188751739_u64 | 2317735820417868754_u64;
(*_17) = (-3480082_i32) as f32;
_6 = _19 << (*_26);
_21 = _8 as f32;
_24 = !(-108_i8);
(*_17) = _16 as f32;
(*_13) = _21 <= (*_17);
(*_17) = _16 as f32;
_3 = _4;
(*_13) = !false;
(*_17) = _8 as f32;
(*_13) = (*_26) == (*_26);
(*_13) = !false;
(*_13) = (*_26) >= _5;
_11 = core::ptr::addr_of_mut!((*_13));
_11 = core::ptr::addr_of_mut!((*_13));
(*_13) = true;
(*_17) = (-866250126_i32) as f32;
(*_17) = (-7756453413108396302_i64) as f32;
(*_13) = (*_17) < (*_17);
(*_13) = (*_26) == (*_26);
Goto(bb10)
}
bb37 = {
_5 = !_7;
Goto(bb9)
}
bb38 = {
_6 = _19 | _19;
(*_13) = false & true;
(*_13) = true | false;
(*_17) = _23 as f32;
Goto(bb11)
}
bb39 = {
_82 = Adt18::Variant1 { fld0: 134585587944303513153513795518037375404_u128,fld1: _16,fld2: _68,fld3: _39 };
_25 = [(*_49),_71,(*_49),(*_49),(*_49)];
_82 = Adt18::Variant0 { fld0: 288126814471638563375158185312542104690_u128,fld1: _20.1,fld2: (*_22),fld3: _41,fld4: _68,fld5: _16,fld6: _33 };
_70 = (*_49) ^ (*_49);
_31 = !_28;
_21 = _65;
(*_29) = 621095306_u32 | 1872383196_u32;
(*_17) = -_36;
(*_56) = (*_17) as i16;
_77 = (*_59);
_45 = -_8;
_54 = core::ptr::addr_of_mut!(_31);
(*_17) = _36 * _36;
_24 = !_39;
(*_29) = 1469869525_u32 & 1404570612_u32;
_88.3 = _33;
_52 = core::ptr::addr_of_mut!((*_54));
Goto(bb40)
}
bb40 = {
_18 = _23 as u32;
(*_56) = (-8660_i16) & 6827_i16;
_74 = _39 * _44;
_72 = (*_49) << _19;
(*_29) = _34 as u32;
(*_52) = !_28;
_41 = _46 ^ _46;
(*_54) = _28;
_75 = (*_56) as f32;
(*_56) = 14366_i16 | 1951_i16;
(*_56) = 2408_i16;
_9 = _34;
_79 = &(*_26);
_54 = core::ptr::addr_of_mut!(_30);
Call((*_29) = core::intrinsics::bswap(1474919398_u32), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
_83 = [_35,_23,_23];
(*_54) = _31;
_82 = Adt18::Variant1 { fld0: 11079342380778598762858230882281557516_u128,fld1: _16,fld2: _61,fld3: _39 };
_11 = core::ptr::addr_of_mut!((*_54));
(*_17) = _36 + _36;
(*_56) = 28526_i16 >> (*_49);
(*_17) = -_47;
(*_54) = _31;
_86 = &mut _16;
_27 = -(-10950_i16);
(*_54) = _31;
(*_86) = Field::<usize>(Variant(_82, 1), 1) + Field::<usize>(Variant(_82, 1), 1);
(*_29) = !2999724491_u32;
_53 = (Move(_52),);
(*_54) = _31;
_31 = (*_54) >= (*_54);
_79 = &_7;
_25 = [_19,(*_49),(*_49),(*_49),_6];
(*_86) = !Field::<usize>(Variant(_82, 1), 1);
(*_17) = (-1819087429_i32) as f32;
(*_29) = 1816447619_u32 + 1346625850_u32;
(*_29) = 3201246760_u32 - 1047605701_u32;
(*_56) = 2776_i16 ^ 17924_i16;
_94 = (*_54);
Goto(bb42)
}
bb42 = {
_92.0 = _68;
(*_17) = _75 - _65;
place!(Field::<u8>(Variant(_82, 1), 2)) = _92.0;
(*_54) = (*_49) >= (*_49);
(*_29) = !3558013521_u32;
_13 = core::ptr::addr_of_mut!(_94);
_73 = _40;
_88.2 = 1427144896_i32;
(*_29) = !2544448588_u32;
_68 = Field::<u8>(Variant(_82, 1), 2);
match _88.3 {
0 => bb37,
1 => bb14,
2 => bb20,
6521791991527710629 => bb43,
_ => bb15
}
}
bb43 = {
(*_13) = (*_54);
(*_56) = 21586_i16 >> (*_49);
_71 = -(*_49);
place!(Field::<u8>(Variant(_82, 1), 2)) = !_68;
(*_54) = !(*_13);
Goto(bb44)
}
bb44 = {
_54 = Move(_53.0);
_4 = _42;
_50 = [_46,_41,_41,_76,_41];
_15 = core::ptr::addr_of_mut!(_78);
(*_29) = _68 as u32;
_53 = (Move(_11),);
(*_13) = (*_29) <= (*_29);
_92 = (Field::<u8>(Variant(_82, 1), 2), Move(_56), (*_56));
(*_13) = _28;
_88.1.2 = _40;
(*_13) = (*_59) < (*_79);
(*_17) = _65;
(*_15) = [(*_29),_18,_18,(*_29),(*_29),_18,(*_29)];
place!(Field::<i8>(Variant(_82, 1), 3)) = _74 | _39;
_88.1.2 = _8 * _8;
match _88.2 {
0 => bb5,
1 => bb34,
2 => bb30,
3 => bb10,
4 => bb45,
1427144896 => bb47,
_ => bb46
}
}
bb45 = {
_29 = core::ptr::addr_of!((*_29));
(*_17) = _36 - _36;
_23 = 18256966882041521203_u64 | 2278899978199393917_u64;
(*_29) = 553023774_u32 * 381649086_u32;
_33 = 6521791991527710629_i64;
RET = [(-2060362854_i32)];
(*_29) = 3685179212_u32;
Goto(bb15)
}
bb46 = {
_26 = &_7;
(*_13) = _24 <= _24;
_28 = false;
_23 = 6386205951188751739_u64 | 2317735820417868754_u64;
(*_17) = (-3480082_i32) as f32;
_6 = _19 << (*_26);
_21 = _8 as f32;
_24 = !(-108_i8);
(*_17) = _16 as f32;
(*_13) = _21 <= (*_17);
(*_17) = _16 as f32;
_3 = _4;
(*_13) = !false;
(*_17) = _8 as f32;
(*_13) = (*_26) == (*_26);
(*_13) = !false;
(*_13) = (*_26) >= _5;
_11 = core::ptr::addr_of_mut!((*_13));
_11 = core::ptr::addr_of_mut!((*_13));
(*_13) = true;
(*_17) = (-866250126_i32) as f32;
(*_17) = (-7756453413108396302_i64) as f32;
(*_13) = (*_17) < (*_17);
(*_13) = (*_26) == (*_26);
Goto(bb10)
}
bb47 = {
_83 = [_35,_35,_23];
_78 = [(*_29),(*_29),(*_29),_18,(*_29),(*_29),(*_29)];
_89 = -_39;
Goto(bb48)
}
bb48 = {
(*_17) = _88.2 as f32;
(*_17) = _36;
(*_13) = !_31;
(*_13) = _28;
(*_17) = _65 - _36;
(*_86) = Field::<usize>(Variant(_82, 1), 1) ^ Field::<usize>(Variant(_82, 1), 1);
(*_29) = 1969615912_u32 >> _92.0;
(*_86) = _92.2 as usize;
_96.1.2 = _88.1.2 * _73;
match _88.2 {
0 => bb49,
1 => bb50,
1427144896 => bb52,
_ => bb51
}
}
bb49 = {
_73 = (*_22) + (*_22);
_39 = _44 - _44;
_71 = -_19;
_51 = (*_26);
RET = [522765704_i32];
(*_17) = -_36;
_25 = [_66,_19,_66,_71,_19];
_36 = (*_17);
(*_17) = _36 * _47;
(*_17) = -_47;
_50 = [_41,_46,_41,_46,_41];
(*_56) = (*_29) as i16;
(*_56) = (-12436_i16) << _6;
_56 = core::ptr::addr_of!((*_56));
_21 = _47 * _47;
(*_29) = _23 as u32;
_21 = -_47;
_41 = _46;
(*_29) = 2509206882_u32;
_30 = !_31;
(*_56) = !(-8635_i16);
match (*_29) {
0 => bb21,
1 => bb17,
2 => bb11,
3 => bb9,
4 => bb28,
2509206882 => bb30,
_ => bb29
}
}
bb50 = {
_5 = !_7;
Goto(bb9)
}
bb51 = {
_92.0 = _68;
(*_17) = _75 - _65;
place!(Field::<u8>(Variant(_82, 1), 2)) = _92.0;
(*_54) = (*_49) >= (*_49);
(*_29) = !3558013521_u32;
_13 = core::ptr::addr_of_mut!(_94);
_73 = _40;
_88.2 = 1427144896_i32;
(*_29) = !2544448588_u32;
_68 = Field::<u8>(Variant(_82, 1), 2);
match _88.3 {
0 => bb37,
1 => bb14,
2 => bb20,
6521791991527710629 => bb43,
_ => bb15
}
}
bb52 = {
_93 = [_88.2];
(*_15) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
Goto(bb53)
}
bb53 = {
Call(_107 = dump_var(Move(_61), Move(_89), Move(_77), Move(_34)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_107 = dump_var(Move(_4), Move(_7), Move(_19), Move(_74)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_107 = dump_var(Move(_12), Move(_35), Move(_2), Move(_78)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_107 = dump_var(Move(_9), Move(_3), Move(_51), Move(_55)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_107 = dump_var(Move(_18), Move(_76), Move(_94), Move(_42)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_107 = dump_var(Move(_44), Move(_41), _108, _108), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: char,mut _3: f64,mut _4: &'static f64,mut _5: char,mut _6: char,mut _7: char,mut _8: isize,mut _9: f64,mut _10: u16,mut _11: char) -> [u64; 4] {
mir! {
type RET = [u64; 4];
let _12: [u32; 6];
let _13: bool;
let _14: isize;
let _15: i16;
let _16: i64;
let _17: *const u32;
let _18: *mut [usize; 4];
let _19: *const u32;
let _20: f64;
let _21: [i32; 3];
let _22: char;
let _23: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _24: [u16; 1];
let _25: f64;
let _26: char;
let _27: (i16, (char, i32, f64, char), i32, i64);
let _28: isize;
let _29: ((*mut bool,), usize);
let _30: [u16; 1];
let _31: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _32: char;
let _33: u32;
let _34: i8;
let _35: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _36: [i64; 6];
let _37: u8;
let _38: i16;
let _39: Adt20;
let _40: *mut *mut &'static Adt18;
let _41: f32;
let _42: (i16, (char, i32, f64, char), i32, i64);
let _43: &'static i8;
let _44: [usize; 4];
let _45: i32;
let _46: &'static mut *const u32;
let _47: [usize; 7];
let _48: f64;
let _49: (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _50: f32;
let _51: isize;
let _52: isize;
let _53: &'static u16;
let _54: Adt20;
let _55: char;
let _56: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _57: u16;
let _58: u64;
let _59: Adt20;
let _60: bool;
let _61: *mut [i128; 5];
let _62: ((*mut bool,), usize);
let _63: isize;
let _64: *const f32;
let _65: &'static (Adt58,);
let _66: char;
let _67: i64;
let _68: isize;
let _69: [u64; 3];
let _70: i16;
let _71: i128;
let _72: (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _73: char;
let _74: u32;
let _75: bool;
let _76: *const [i64; 6];
let _77: [u16; 1];
let _78: Adt20;
let _79: isize;
let _80: f32;
let _81: f32;
let _82: [char; 6];
let _83: i32;
let _84: i64;
let _85: f64;
let _86: isize;
let _87: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _88: *mut bool;
let _89: [u32; 6];
let _90: bool;
let _91: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _92: char;
let _93: f64;
let _94: i8;
let _95: u128;
let _96: i16;
let _97: (u8, *const i16, i16);
let _98: bool;
let _99: isize;
let _100: *mut *const f32;
let _101: bool;
let _102: f32;
let _103: isize;
let _104: isize;
let _105: ();
let _106: ();
{
_1 = _11;
_1 = _6;
RET = [16040096159596169176_u64,1386619042829515440_u64,8497899070574082562_u64,8259337295662073827_u64];
_11 = _2;
_7 = _11;
_12 = [521608556_u32,746602626_u32,1566658854_u32,857636812_u32,3004495601_u32,712526229_u32];
_3 = -_9;
Goto(bb1)
}
bb1 = {
_4 = &_9;
RET = [7661217424720337004_u64,10310431476291622779_u64,1479585239200973043_u64,2334876788066097931_u64];
_6 = _1;
_13 = _1 != _6;
_7 = _5;
_9 = -_3;
_9 = _3 * _3;
_9 = _3 + _3;
_3 = _9 + _9;
Goto(bb2)
}
bb2 = {
_2 = _7;
_15 = _13 as i16;
_12 = [3279449674_u32,3762957252_u32,1337002263_u32,1504571231_u32,1194276218_u32,2885057712_u32];
_7 = _11;
Call(_8 = fn14(Move(_4), _15, _15, _5, RET, _15, _2, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _2;
_14 = _8 * _8;
_6 = _5;
_9 = _3 + _3;
_13 = false;
_16 = (-3308785802763384865_i64) >> _15;
_15 = (-1800_i16) - 11032_i16;
_2 = _1;
_16 = 1214810951_i32 as i64;
_4 = &_3;
_14 = _8;
_15 = 24874_i16 + 30828_i16;
RET = [12773703534077189377_u64,18276690950332334019_u64,10552220286266361491_u64,18749847592883265_u64];
Call(_14 = core::intrinsics::transmute(_16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = 195706361790167626130125044148496258266_u128 as isize;
Goto(bb5)
}
bb5 = {
_4 = &_9;
_9 = _8 as f64;
_10 = 10428_u16;
_8 = _16 as isize;
RET = [7774887467090715293_u64,3864468145973236994_u64,598090342397496743_u64,9364424437852946061_u64];
_9 = _3;
_15 = (-3084_i16) - 32035_i16;
match _10 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
10428 => bb8,
_ => bb7
}
}
bb6 = {
_2 = _7;
_15 = _13 as i16;
_12 = [3279449674_u32,3762957252_u32,1337002263_u32,1504571231_u32,1194276218_u32,2885057712_u32];
_7 = _11;
Call(_8 = fn14(Move(_4), _15, _15, _5, RET, _15, _2, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_11 = _2;
_14 = _8 * _8;
_6 = _5;
_9 = _3 + _3;
_13 = false;
_16 = (-3308785802763384865_i64) >> _15;
_15 = (-1800_i16) - 11032_i16;
_2 = _1;
_16 = 1214810951_i32 as i64;
_4 = &_3;
_14 = _8;
_15 = 24874_i16 + 30828_i16;
RET = [12773703534077189377_u64,18276690950332334019_u64,10552220286266361491_u64,18749847592883265_u64];
Call(_14 = core::intrinsics::transmute(_16), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_2 = _6;
_9 = _3 - _3;
_15 = 17445656666563071597_u64 as i16;
_16 = 4482777844736439286_i64 ^ (-8412803945228010160_i64);
_16 = 805997150260274343_i64 + 3471760761409782877_i64;
_16 = -(-7458457899659348874_i64);
_5 = _6;
_20 = -_9;
_16 = -(-8773896931117923361_i64);
_6 = _2;
_1 = _2;
_2 = _5;
_4 = &_3;
_16 = (-6207492676559565030_i64) * (-8678578928605815645_i64);
_14 = !_8;
_3 = _20 * _20;
_16 = (-3111596398677656451_i64) - (-6953134919732555244_i64);
_7 = _1;
_20 = _14 as f64;
_22 = _6;
_1 = _7;
_20 = _3 * _9;
_21 = [1372055506_i32,1000452818_i32,1273599181_i32];
match _10 {
0 => bb1,
1 => bb5,
10428 => bb10,
_ => bb9
}
}
bb9 = {
_4 = &_9;
_9 = _8 as f64;
_10 = 10428_u16;
_8 = _16 as isize;
RET = [7774887467090715293_u64,3864468145973236994_u64,598090342397496743_u64,9364424437852946061_u64];
_9 = _3;
_15 = (-3084_i16) - 32035_i16;
match _10 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
10428 => bb8,
_ => bb7
}
}
bb10 = {
_14 = 1626574018_i32 as isize;
_13 = true ^ false;
_9 = _20;
_13 = true ^ false;
_8 = _14 - _14;
_4 = &_20;
_9 = _15 as f64;
_12 = [1334073201_u32,141854207_u32,3834047836_u32,2256503047_u32,2681063991_u32,1858457352_u32];
_24 = [_10];
_15 = (-18392_i16) + (-9393_i16);
_14 = _8 - _8;
_8 = _14 ^ _14;
_15 = 27740_i16 ^ 31317_i16;
_9 = (*_4);
_1 = _7;
_27.3 = _16;
_10 = 12387_u16 & 1274_u16;
_25 = 616930863_u32 as f64;
_27.0 = _15 | _15;
_25 = 6743542701738711380_usize as f64;
_27.1.3 = _5;
_15 = _27.0;
_11 = _1;
_5 = _11;
_14 = _8 ^ _8;
Goto(bb11)
}
bb11 = {
_2 = _11;
_27.3 = !_16;
_28 = _14 - _8;
_14 = !_8;
_1 = _6;
_27.1.2 = (*_4) - (*_4);
_28 = 52744963277975183622428302677522790084_u128 as isize;
Goto(bb12)
}
bb12 = {
_29.0.0 = core::ptr::addr_of_mut!(_13);
RET = [8500262457818657611_u64,5454410606040663613_u64,13433602137976118469_u64,3189782540199087540_u64];
_27.1 = (_6, (-1935970204_i32), (*_4), _22);
_9 = -(*_4);
_1 = _22;
_24 = [_10];
_29.1 = !11241959475637398069_usize;
_29.1 = 1538093799403540795_usize << _27.1.1;
_27.3 = _16 >> _14;
_25 = _27.1.1 as f64;
_10 = 15365_u16;
_27.1.2 = (*_4) + (*_4);
_1 = _11;
_27.2 = _27.1.1 + _27.1.1;
_2 = _7;
_2 = _27.1.3;
_20 = _29.1 as f64;
_27.1.1 = _27.2 + _27.2;
_9 = _27.1.2 - _27.1.2;
_28 = _14 * _14;
_27.1.1 = _27.2 - _27.2;
_7 = _27.1.3;
_16 = _27.3 * _27.3;
_10 = 8618_u16 + 29605_u16;
_24 = [_10];
_5 = _2;
_6 = _27.1.0;
_30 = [_10];
_20 = _25 - _27.1.2;
Goto(bb13)
}
bb13 = {
_27.2 = 67_i8 as i32;
_28 = !_14;
_30 = [_10];
_14 = !_28;
_32 = _5;
_12 = [222941005_u32,2187567047_u32,732219004_u32,639846668_u32,2815449797_u32,4229362228_u32];
_27.1.3 = _2;
_10 = 36858_u16 | 55681_u16;
_20 = _9 + _27.1.2;
_27.1.1 = _27.2;
_10 = 53781_u16 << _28;
_26 = _27.1.3;
_30 = [_10];
_11 = _6;
_3 = -_20;
_34 = 119_i8 & (-22_i8);
_10 = 45592_u16 - 1263_u16;
_27.3 = _13 as i64;
_34 = (-28_i8) << _29.1;
_27.3 = 160299481581364349114846610155205637452_i128 as i64;
_15 = _27.0 >> _8;
_33 = !877407462_u32;
_29.1 = 2101296094123615511_usize & 4_usize;
Goto(bb14)
}
bb14 = {
_1 = _7;
Goto(bb15)
}
bb15 = {
_14 = _28;
_27.1.1 = _27.2 ^ _27.2;
_27.1.1 = -_27.2;
_1 = _6;
_24 = [_10];
_33 = 4075487713_u32;
_32 = _2;
_25 = _20;
_16 = _27.3;
_17 = core::ptr::addr_of!(_33);
(*_17) = !2463937123_u32;
_8 = !_14;
_38 = _15 - _15;
Goto(bb16)
}
bb16 = {
_42.1.0 = _27.1.3;
_42.1.3 = _2;
_2 = _1;
_41 = _15 as f32;
(*_17) = _9 as u32;
_8 = _28;
_3 = _38 as f64;
_36 = [_16,_16,_16,_27.3,_16,_27.3];
_33 = !3636022113_u32;
_29.1 = 6_usize;
_12 = [(*_17),(*_17),(*_17),(*_17),(*_17),(*_17)];
_36 = [_16,_27.3,_27.3,_27.3,_27.3,_16];
Goto(bb17)
}
bb17 = {
(*_17) = 3215372738_u32;
_27.3 = _16;
_18 = core::ptr::addr_of_mut!(_44);
_6 = _26;
(*_17) = !2174166631_u32;
Goto(bb18)
}
bb18 = {
_45 = _27.1.1 - _27.2;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_28 = !_14;
_4 = &_25;
(*_17) = 964340853_u32;
RET = [1611248741426147789_u64,10466413271392043378_u64,12553473062899668326_u64,6368434472287256262_u64];
(*_17) = 2301719804_u32;
_34 = 105_i8 | 21_i8;
_14 = _28 * _8;
(*_17) = 3738438026_u32 & 1141193854_u32;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_39 = Adt20::Variant3 { fld0: _41,fld1: 99_u8 };
_42.1.2 = (*_4);
_43 = &_34;
_34 = (-113_i8);
_36 = [_27.3,_27.3,_16,_27.3,_16,_27.3];
_9 = _16 as f64;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_32 = _26;
_33 = 15812674078953605582_u64 as u32;
(*_17) = _13 as u32;
_42.1.1 = _27.1.1 & _27.2;
Goto(bb19)
}
bb19 = {
_42.3 = _16 * _16;
_33 = _10 as u32;
_6 = _27.1.3;
_27.0 = _38 & _38;
_26 = _22;
match _34 {
0 => bb20,
1 => bb21,
2 => bb22,
340282366920938463463374607431768211343 => bb24,
_ => bb23
}
}
bb20 = {
_14 = 195706361790167626130125044148496258266_u128 as isize;
Goto(bb5)
}
bb21 = {
_2 = _6;
_9 = _3 - _3;
_15 = 17445656666563071597_u64 as i16;
_16 = 4482777844736439286_i64 ^ (-8412803945228010160_i64);
_16 = 805997150260274343_i64 + 3471760761409782877_i64;
_16 = -(-7458457899659348874_i64);
_5 = _6;
_20 = -_9;
_16 = -(-8773896931117923361_i64);
_6 = _2;
_1 = _2;
_2 = _5;
_4 = &_3;
_16 = (-6207492676559565030_i64) * (-8678578928605815645_i64);
_14 = !_8;
_3 = _20 * _20;
_16 = (-3111596398677656451_i64) - (-6953134919732555244_i64);
_7 = _1;
_20 = _14 as f64;
_22 = _6;
_1 = _7;
_20 = _3 * _9;
_21 = [1372055506_i32,1000452818_i32,1273599181_i32];
match _10 {
0 => bb1,
1 => bb5,
10428 => bb10,
_ => bb9
}
}
bb22 = {
_42.1.0 = _27.1.3;
_42.1.3 = _2;
_2 = _1;
_41 = _15 as f32;
(*_17) = _9 as u32;
_8 = _28;
_3 = _38 as f64;
_36 = [_16,_16,_16,_27.3,_16,_27.3];
_33 = !3636022113_u32;
_29.1 = 6_usize;
_12 = [(*_17),(*_17),(*_17),(*_17),(*_17),(*_17)];
_36 = [_16,_27.3,_27.3,_27.3,_27.3,_16];
Goto(bb17)
}
bb23 = {
_11 = _2;
_14 = _8 * _8;
_6 = _5;
_9 = _3 + _3;
_13 = false;
_16 = (-3308785802763384865_i64) >> _15;
_15 = (-1800_i16) - 11032_i16;
_2 = _1;
_16 = 1214810951_i32 as i64;
_4 = &_3;
_14 = _8;
_15 = 24874_i16 + 30828_i16;
RET = [12773703534077189377_u64,18276690950332334019_u64,10552220286266361491_u64,18749847592883265_u64];
Call(_14 = core::intrinsics::transmute(_16), ReturnTo(bb4), UnwindUnreachable())
}
bb24 = {
_42.0 = _38 & _27.0;
_29.1 = (*_17) as usize;
_29.0.0 = core::ptr::addr_of_mut!(_13);
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_34 = _42.0 as i8;
_15 = !_27.0;
_42.2 = _42.1.1;
(*_17) = (*_4) as u32;
(*_17) = !1438495717_u32;
_44 = [_29.1,_29.1,_29.1,_29.1];
place!(Field::<u8>(Variant(_39, 3), 1)) = 180_u8;
_45 = _27.1.1 & _42.1.1;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_27.1.3 = _22;
_16 = _27.0 as i64;
(*_17) = !3986969089_u32;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_24 = [_10];
_41 = Field::<f32>(Variant(_39, 3), 0) * Field::<f32>(Variant(_39, 3), 0);
_7 = _32;
(*_17) = (*_4) as u32;
place!(Field::<u8>(Variant(_39, 3), 1)) = 8_u8;
_42.1.0 = _6;
_13 = (*_4) != (*_4);
(*_17) = _29.1 as u32;
_10 = !60791_u16;
Goto(bb25)
}
bb25 = {
_29.1 = 15521770720800611234_u64 as usize;
(*_17) = 3201097627_u32 >> _38;
_35 = core::ptr::addr_of_mut!(_49);
_20 = _16 as f64;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
(*_35).2 = [_28,_14,_14,_8,_14];
_44 = [_29.1,_29.1,_29.1,_29.1];
_24 = _30;
_14 = _8;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
(*_35).0 = core::ptr::addr_of_mut!((*_18));
_16 = Field::<u8>(Variant(_39, 3), 1) as i64;
_46 = &mut _17;
_42.1.1 = _42.2 - _42.2;
_42.1.1 = _27.2;
(*_35).0 = core::ptr::addr_of_mut!(_44);
(*_35).0 = core::ptr::addr_of_mut!((*_18));
_37 = !Field::<u8>(Variant(_39, 3), 1);
place!(Field::<f32>(Variant(_39, 3), 0)) = -_41;
_27.1.0 = _26;
_50 = _38 as f32;
(*_35).2 = [_14,_8,_28,_28,_8];
_41 = _50;
Goto(bb26)
}
bb26 = {
_10 = !20112_u16;
Goto(bb27)
}
bb27 = {
_53 = &_10;
_27 = (_42.0, _42.1, _42.2, _42.3);
_42.2 = -_45;
Goto(bb28)
}
bb28 = {
_42.1 = (_5, _42.2, (*_4), _2);
_27 = _42;
_52 = _28 & _8;
(*_35).2 = [_14,_14,_8,_28,_28];
(*_35).1 = Adt46::Variant1 { fld0: (*_18),fld1: _12 };
(*_18) = [_29.1,_29.1,_29.1,_29.1];
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
(*_35).0 = Move(_18);
(*_46) = core::ptr::addr_of!(_33);
Goto(bb29)
}
bb29 = {
_27.1.2 = -(*_4);
_54 = Adt20::Variant1 { fld0: _13,fld1: _14 };
(*_35).2 = [_52,Field::<isize>(Variant(_54, 1), 1),_8,_52,_8];
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant(_49.1, 1), 0)));
_55 = _5;
_19 = core::ptr::addr_of!(_33);
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [(*_19),(*_19),(*_19),(*_19),(*_19),(*_19)];
(*_35).2 = [_52,Field::<isize>(Variant(_54, 1), 1),_28,_52,Field::<isize>(Variant(_54, 1), 1)];
RET = [12281628101803383069_u64,2973845006794360950_u64,4715302776418896942_u64,11310515719516990785_u64];
(*_35).2 = [Field::<isize>(Variant(_54, 1), 1),_52,_8,_52,_52];
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [(*_19),(*_19),(*_19),(*_19),(*_19),(*_19)];
place!(Field::<u8>(Variant(_39, 3), 1)) = !_37;
_5 = _6;
(*_35).2 = [_52,_14,_52,_52,Field::<isize>(Variant(_54, 1), 1)];
_12 = [(*_19),(*_19),(*_19),(*_19),_33,(*_19)];
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
_16 = _42.3 - _27.3;
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
_15 = _42.0 >> _42.0;
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = _12;
_41 = Field::<f32>(Variant(_39, 3), 0) * Field::<f32>(Variant(_39, 3), 0);
_33 = !2545282280_u32;
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
_16 = !_27.3;
Goto(bb30)
}
bb30 = {
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
Call(_29.1 = core::intrinsics::transmute(_14), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_14 = _28 & _52;
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = _44;
(*_35).2 = [Field::<isize>(Variant(_54, 1), 1),_14,_8,_14,_28];
Goto(bb32)
}
bb32 = {
(*_46) = Move(_19);
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
(*_35).2 = [_52,_52,_14,_14,_52];
(*_35).2 = [_8,_14,_14,_52,_28];
(*_35).2 = [_8,_52,_8,Field::<isize>(Variant(_54, 1), 1),_28];
Goto(bb33)
}
bb33 = {
_27.1.1 = _27.2 + _42.1.1;
_59 = Move(_39);
_49.0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
_15 = _27.0;
(*_46) = core::ptr::addr_of!(_33);
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
_39 = Adt20::Variant3 { fld0: Field::<f32>(Variant(_59, 3), 0),fld1: _37 };
(*_46) = core::ptr::addr_of!(_33);
_50 = Field::<f32>(Variant(_59, 3), 0);
(*_46) = core::ptr::addr_of!(_33);
_49.0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
_54 = Move(_39);
place!(Field::<[usize; 4]>(Variant(_49.1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
_48 = (*_4) * (*_4);
(*_35).2 = [_14,_14,_14,_28,_14];
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
_27.1.2 = (*_4);
_6 = _7;
_60 = (*_4) != (*_4);
Goto(bb34)
}
bb34 = {
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
(*_35).0 = core::ptr::addr_of_mut!(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)));
Call(place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = core::intrinsics::transmute(_44), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
_64 = core::ptr::addr_of!(_41);
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
(*_64) = _16 as f32;
Goto(bb36)
}
bb36 = {
_48 = _29.1 as f64;
(*_35).2 = [_14,_14,_14,_14,_28];
_68 = _52 << _42.0;
_27 = _42;
(*_35).2 = [_68,_14,_14,_68,_68];
Goto(bb37)
}
bb37 = {
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
(*_64) = Field::<f32>(Variant(_54, 3), 0) - Field::<f32>(Variant(_59, 3), 0);
_66 = _5;
_18 = core::ptr::addr_of_mut!(_44);
_64 = core::ptr::addr_of!((*_64));
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = _12;
_20 = (*_64) as f64;
_57 = (*_53) & (*_53);
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
_54 = Move(_59);
_3 = (*_4) - (*_4);
(*_64) = Field::<f32>(Variant(_54, 3), 0);
(*_35).2 = [_68,_14,_8,_68,_68];
(*_18) = [_29.1,_29.1,_29.1,_29.1];
Goto(bb38)
}
bb38 = {
(*_35).0 = core::ptr::addr_of_mut!((*_18));
place!(Field::<[usize; 4]>(Variant(_49.1, 1), 0)) = (*_18);
_33 = 926488303_u32 - 979525282_u32;
(*_35).0 = core::ptr::addr_of_mut!((*_18));
(*_64) = Field::<f32>(Variant(_54, 3), 0);
_32 = _42.1.3;
place!(Field::<[usize; 4]>(Variant(_49.1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
_70 = _27.0;
_37 = Field::<u8>(Variant(_54, 3), 1) * Field::<u8>(Variant(_54, 3), 1);
_12 = [_33,_33,_33,_33,_33,_33];
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = (*_18);
_63 = _52;
Call(_29.1 = core::intrinsics::transmute(_8), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_27.0 = _38;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
_70 = _15;
(*_46) = core::ptr::addr_of!(_74);
(*_35).2 = [_68,_14,_14,_68,_8];
_69 = [6356871625401833804_u64,7074292156036424855_u64,2977140734527851414_u64];
(*_64) = _38 as f32;
Goto(bb40)
}
bb40 = {
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_33,_33,_33,_33,_33,_33];
_58 = (*_53) as u64;
(*_35).0 = core::ptr::addr_of_mut!((*_18));
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = _44;
Goto(bb41)
}
bb41 = {
_27.0 = _70;
(*_35).2 = [_68,_63,_52,_68,_52];
_47 = [_29.1,_29.1,_29.1,_29.1,_29.1,_29.1,_29.1];
_12 = [_33,_33,_33,_33,_33,_33];
_26 = _2;
(*_64) = Field::<f32>(Variant(_54, 3), 0) + Field::<f32>(Variant(_54, 3), 0);
_68 = _14 << _27.0;
place!(Field::<[usize; 4]>(Variant(_49.1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
_42 = (_15, _27.1, _27.1.1, _16);
_73 = _42.1.0;
_27 = (_42.0, _42.1, _42.1.1, _42.3);
(*_35).2 = [_14,_14,_68,_28,_63];
_14 = -_63;
_42.2 = _27.2;
(*_18) = [_29.1,_29.1,_29.1,_29.1];
(*_35).0 = core::ptr::addr_of_mut!((*_18));
_42.1.2 = (*_4) * (*_4);
(*_35).1 = Adt46::Variant2 { fld0: Move(_64) };
_74 = _29.1 as u32;
(*_35).0 = core::ptr::addr_of_mut!((*_18));
Goto(bb42)
}
bb42 = {
(*_35).0 = Move(_18);
_59 = Move(_54);
_39 = Adt20::Variant1 { fld0: _60,fld1: _68 };
(*_35).2 = [_63,Field::<isize>(Variant(_39, 1), 1),_68,_68,Field::<isize>(Variant(_39, 1), 1)];
(*_35).0 = core::ptr::addr_of_mut!(_44);
(*_35).2 = [_14,_68,Field::<isize>(Variant(_39, 1), 1),Field::<isize>(Variant(_39, 1), 1),Field::<isize>(Variant(_39, 1), 1)];
_27.1 = _42.1;
(*_35).2 = [Field::<isize>(Variant(_39, 1), 1),_28,_68,Field::<isize>(Variant(_39, 1), 1),Field::<isize>(Variant(_39, 1), 1)];
_42.1.1 = _27.2;
_70 = _27.0;
_14 = _68 * Field::<isize>(Variant(_39, 1), 1);
Goto(bb43)
}
bb43 = {
(*_35).2 = [_68,_63,Field::<isize>(Variant(_39, 1), 1),_14,Field::<isize>(Variant(_39, 1), 1)];
(*_35).2 = [_68,Field::<isize>(Variant(_39, 1), 1),_14,_68,Field::<isize>(Variant(_39, 1), 1)];
_42.1.3 = _5;
_66 = _11;
_13 = _60;
_53 = &_57;
_58 = 12344415624552108580_u64 << _68;
_42.1.0 = _22;
(*_46) = core::ptr::addr_of!(_74);
_42 = (_15, _27.1, _27.2, _16);
(*_35).2 = [Field::<isize>(Variant(_39, 1), 1),Field::<isize>(Variant(_39, 1), 1),_68,_14,_14];
(*_35).0 = core::ptr::addr_of_mut!(_44);
(*_35).0 = core::ptr::addr_of_mut!(_44);
(*_35).0 = core::ptr::addr_of_mut!(_44);
(*_35).0 = core::ptr::addr_of_mut!(_44);
place!(Field::<*const f32>(Variant((*_35).1, 2), 0)) = core::ptr::addr_of!(_81);
_1 = _42.1.0;
_79 = _14;
(*_35).2 = [_14,_14,_68,_14,Field::<isize>(Variant(_39, 1), 1)];
_42.1.2 = (*_53) as f64;
place!(Field::<*const f32>(Variant((*_35).1, 2), 0)) = core::ptr::addr_of!(_80);
_79 = (*_53) as isize;
(*_35).2 = [_14,_68,Field::<isize>(Variant(_39, 1), 1),_68,Field::<isize>(Variant(_39, 1), 1)];
_84 = _74 as i64;
Goto(bb44)
}
bb44 = {
(*_46) = core::ptr::addr_of!(_33);
(*_46) = core::ptr::addr_of!(_74);
_27.1.3 = _22;
(*_35).0 = core::ptr::addr_of_mut!(_44);
_83 = _38 as i32;
_51 = Field::<isize>(Variant(_39, 1), 1) ^ _14;
_71 = -(-78471644145946876145980518941579603541_i128);
(*_46) = core::ptr::addr_of!(_74);
place!(Field::<*const f32>(Variant((*_35).1, 2), 0)) = core::ptr::addr_of!(_41);
_54 = Adt20::Variant3 { fld0: Field::<f32>(Variant(_59, 3), 0),fld1: Field::<u8>(Variant(_59, 3), 1) };
(*_35).2 = [_68,_68,_14,_68,_14];
_64 = core::ptr::addr_of!(_50);
Goto(bb45)
}
bb45 = {
_11 = _32;
_52 = _68 ^ _28;
(*_35).2 = [_68,_51,_14,_68,_51];
(*_46) = core::ptr::addr_of!(_33);
(*_64) = _83 as f32;
(*_64) = Field::<f32>(Variant(_59, 3), 0) * Field::<f32>(Variant(_59, 3), 0);
Goto(bb46)
}
bb46 = {
(*_64) = -_41;
_5 = _55;
(*_35).2 = [_14,_51,_52,_68,_52];
place!(Field::<isize>(Variant(_39, 1), 1)) = _28 >> _68;
_6 = _55;
_72.1 = Move((*_35).1);
(*_64) = _74 as f32;
_42.0 = -_38;
_12 = [_74,_33,_33,_74,_74,_74];
_15 = !_27.0;
_49.1 = Move(_72.1);
(*_35).0 = core::ptr::addr_of_mut!(_44);
_27.1.1 = !_83;
_62.0.0 = core::ptr::addr_of_mut!(_90);
Goto(bb47)
}
bb47 = {
_26 = _5;
_75 = !_13;
Goto(bb48)
}
bb48 = {
_36 = [_84,_16,_84,_42.3,_27.3,_27.3];
_42.2 = _71 as i32;
_85 = (*_4);
place!(Field::<*const f32>(Variant((*_35).1, 2), 0)) = core::ptr::addr_of!((*_64));
_48 = (*_4) * (*_4);
_88 = core::ptr::addr_of_mut!(_60);
_28 = Field::<isize>(Variant(_39, 1), 1) * _14;
(*_35).0 = core::ptr::addr_of_mut!(_44);
place!(Field::<*const f32>(Variant((*_35).1, 2), 0)) = core::ptr::addr_of!((*_64));
(*_35).1 = Adt46::Variant1 { fld0: _44,fld1: _12 };
(*_46) = core::ptr::addr_of!(_74);
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
_29.0 = (Move(_88),);
_12 = [_74,_74,_33,_74,_74,_74];
_73 = _42.1.0;
_55 = _27.1.0;
Goto(bb49)
}
bb49 = {
_19 = core::ptr::addr_of!(_74);
_16 = _84 | _84;
_46 = &mut _19;
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = _44;
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_29.1,_29.1,_29.1,_29.1];
_6 = _27.1.0;
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_74,_74,_74,_74,_33,_33];
_20 = _27.0 as f64;
_75 = _60 ^ _60;
_42.1.2 = _29.1 as f64;
_42.1.1 = _27.1.1 >> _63;
(*_35).2 = [_14,_28,_51,Field::<isize>(Variant(_39, 1), 1),_28];
_95 = 22317522301839634892112701516954388242_u128 * 254899913933708544801172583512744199823_u128;
(*_64) = _41 + _41;
_62.1 = _29.1 - _29.1;
place!(Field::<[u32; 6]>(Variant((*_35).1, 1), 1)) = [_74,_33,_74,_74,_74,_74];
(*_64) = _41 + Field::<f32>(Variant(_59, 3), 0);
_49.2 = [Field::<isize>(Variant(_39, 1), 1),Field::<isize>(Variant(_39, 1), 1),_14,_63,_28];
_12 = Field::<[u32; 6]>(Variant((*_35).1, 1), 1);
_96 = _58 as i16;
_27.1 = (_73, _83, (*_4), _55);
place!(Field::<[usize; 4]>(Variant((*_35).1, 1), 0)) = [_62.1,_29.1,_29.1,_29.1];
Goto(bb50)
}
bb50 = {
Call(_105 = dump_var(Move(_55), Move(_12), Move(_36), Move(_1)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_105 = dump_var(Move(_69), Move(_2), Move(_13), Move(_21)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_105 = dump_var(Move(_95), Move(_63), Move(_15), Move(_6)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_105 = dump_var(Move(_70), Move(_74), Move(_60), Move(_8)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_105 = dump_var(Move(_45), Move(_75), Move(_68), Move(_96)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_105 = dump_var(Move(_66), Move(_16), Move(_44), Move(_51)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static f64,mut _2: i16,mut _3: i16,mut _4: char,mut _5: [u64; 4],mut _6: i16,mut _7: char,mut _8: f64) -> isize {
mir! {
type RET = isize;
let _9: f64;
let _10: i8;
let _11: *const &'static f64;
let _12: &'static Adt51;
let _13: Adt78;
let _14: u32;
let _15: bool;
let _16: i64;
let _17: u8;
let _18: *mut Adt18;
let _19: isize;
let _20: f64;
let _21: Adt78;
let _22: *const [u16; 1];
let _23: [i128; 5];
let _24: [i32; 3];
let _25: usize;
let _26: i64;
let _27: *const u32;
let _28: &'static *mut &'static Adt18;
let _29: u8;
let _30: Adt51;
let _31: i128;
let _32: isize;
let _33: *mut [usize; 4];
let _34: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _35: u8;
let _36: &'static mut usize;
let _37: u16;
let _38: &'static i16;
let _39: isize;
let _40: [i8; 7];
let _41: Adt41;
let _42: f32;
let _43: char;
let _44: [u64; 3];
let _45: (Adt58, Adt20);
let _46: isize;
let _47: *mut bool;
let _48: [u64; 3];
let _49: (*const &'static f64, char);
let _50: *mut Adt18;
let _51: &'static u128;
let _52: &'static f64;
let _53: *const i16;
let _54: bool;
let _55: f32;
let _56: i128;
let _57: char;
let _58: *mut Adt18;
let _59: (i16, (char, i32, f64, char), i32, i64);
let _60: [usize; 4];
let _61: &'static i16;
let _62: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _63: i32;
let _64: i8;
let _65: *const [u16; 1];
let _66: &'static u16;
let _67: [char; 6];
let _68: usize;
let _69: char;
let _70: i64;
let _71: isize;
let _72: f32;
let _73: f32;
let _74: isize;
let _75: Adt51;
let _76: [i32; 1];
let _77: f32;
let _78: *const u32;
let _79: usize;
let _80: isize;
let _81: u128;
let _82: *mut *const [i64; 6];
let _83: *mut [i128; 5];
let _84: f64;
let _85: ();
let _86: ();
{
RET = (-9223372036854775808_isize);
_1 = &_8;
_5 = [8715847117013486137_u64,6613860987770370972_u64,7580580652264727281_u64,7696127145904808563_u64];
_6 = _2 + _3;
_2 = -_6;
RET = (-9223372036854775808_isize);
_6 = _2 << _3;
_9 = 19694_u16 as f64;
_6 = _2 << _2;
Goto(bb1)
}
bb1 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb2 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb3 = {
(*_11) = &_9;
match _10 {
0 => bb4,
1 => bb5,
84 => bb7,
_ => bb6
}
}
bb4 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb5 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_1 = &_8;
(*_11) = &_9;
_5 = [15930562113688514702_u64,3867079716983499020_u64,11979764125215062570_u64,924470916229218429_u64];
(*_11) = &_8;
RET = -(-9223372036854775808_isize);
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
_1 = &_8;
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
(*_11) = &_8;
_14 = 4041806810_u32 - 186492622_u32;
_3 = -_6;
(*_11) = &_9;
_2 = _14 as i16;
Goto(bb8)
}
bb8 = {
_14 = !2018906760_u32;
(*_11) = &_8;
_3 = 211682814083047345261360903658608335949_u128 as i16;
(*_11) = &_9;
(*_11) = &_8;
Goto(bb9)
}
bb9 = {
_7 = _4;
(*_11) = &_9;
(*_11) = &_8;
_15 = (*_1) > (*_1);
(*_11) = &_9;
(*_11) = &_8;
_4 = _7;
_17 = 211_u8;
_9 = -(*_1);
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
_20 = -(*_1);
(*_11) = &_8;
Call(_9 = core::intrinsics::fmaf64((*_1), (*_1), (*_1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = 4857323546867594729_i64 >> _6;
(*_11) = &_20;
(*_11) = &_8;
(*_11) = &_20;
(*_11) = &_9;
(*_11) = &_20;
_16 = (-5708814815160220145_i64);
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
_19 = RET;
(*_11) = &_8;
_11 = core::ptr::addr_of!((*_11));
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
(*_11) = &_20;
(*_11) = &_8;
_5 = [16199507370976656749_u64,6604535555630798805_u64,2465509279648146741_u64,17816688470014574090_u64];
_24 = [(-2102456283_i32),1793430607_i32,(-251338560_i32)];
(*_11) = &_9;
_1 = &_8;
(*_11) = &_9;
Call(_5 = fn15(Move((*_11)), Move(_11), (*_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25 = 4_usize;
_1 = &_9;
_23[_25] = (-152482182620679873021759066131922757394_i128) | 129820526301815696567562159297307737814_i128;
_20 = _14 as f64;
match _10 {
0 => bb3,
84 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_15 = (*_1) < (*_1);
_17 = !58_u8;
_3 = 5351_u16 as i16;
_23[_25] = _16 as i128;
_17 = !19_u8;
_16 = 845042302540410907_i64;
_19 = RET << _6;
_7 = _4;
_1 = &_8;
_26 = _16;
_20 = (*_1) + (*_1);
_5 = [724193427195117780_u64,17354192251960180888_u64,29678741511293363_u64,9003542257683706830_u64];
_5 = [11067360266493038801_u64,14514135773362150004_u64,6717653234679893770_u64,16051963192446695148_u64];
_16 = _26;
_8 = _20 * _20;
_11 = core::ptr::addr_of!(_1);
_17 = 81_u8 >> _19;
_19 = _16 as isize;
_5 = [9198466790326359878_u64,6078621062072551829_u64,14859755163217737389_u64,18258121913414865685_u64];
match _25 {
0 => bb14,
1 => bb15,
2 => bb16,
4 => bb18,
_ => bb17
}
}
bb14 = {
(*_11) = &_9;
match _10 {
0 => bb4,
1 => bb5,
84 => bb7,
_ => bb6
}
}
bb15 = {
_25 = 4_usize;
_1 = &_9;
_23[_25] = (-152482182620679873021759066131922757394_i128) | 129820526301815696567562159297307737814_i128;
_20 = _14 as f64;
match _10 {
0 => bb3,
84 => bb13,
_ => bb12
}
}
bb16 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb17 = {
_7 = _4;
(*_11) = &_9;
(*_11) = &_8;
_15 = (*_1) > (*_1);
(*_11) = &_9;
(*_11) = &_8;
_4 = _7;
_17 = 211_u8;
_9 = -(*_1);
(*_11) = &_9;
(*_11) = &_8;
(*_11) = &_9;
_20 = -(*_1);
(*_11) = &_8;
Call(_9 = core::intrinsics::fmaf64((*_1), (*_1), (*_1)), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
_19 = !RET;
_7 = _4;
(*_11) = &_20;
_8 = (*_1) * (*_1);
(*_11) = &_8;
_26 = !_16;
_23[_25] = 85104415216636529451863781019473888710_i128;
_27 = core::ptr::addr_of!(_14);
_23 = [(-53737930511652578374150171764428161047_i128),42798047127407912413360418056341021916_i128,95084326656474557832910887838461229387_i128,(-153123537650755089899104977775177123803_i128),(-61637769164744175699151735315450654470_i128)];
(*_11) = &_9;
(*_27) = _17 as u32;
(*_27) = 3579088978_u32 * 3976107646_u32;
_14 = 2064370160_u32;
(*_11) = &_8;
(*_11) = &_20;
(*_11) = &_9;
_27 = core::ptr::addr_of!((*_27));
(*_11) = &_8;
_27 = core::ptr::addr_of!((*_27));
(*_27) = 246648957_u32 << _6;
(*_11) = &_20;
_17 = 7847713239085555108_u64 as u8;
match _23[_25] {
0 => bb9,
1 => bb2,
278644597756194287764222872116317556986 => bb19,
_ => bb4
}
}
bb19 = {
(*_27) = 2314053837_u32;
(*_27) = 2104589968_u32 * 763514846_u32;
(*_27) = 4239842148_u32 << _19;
_3 = _6 - _2;
(*_11) = &_9;
(*_11) = &_8;
_25 = 7_usize;
(*_11) = &_9;
(*_27) = 2207932836_u32 & 758443510_u32;
(*_11) = &_8;
_5 = [17424514914686526851_u64,15009304809359886191_u64,14280868299777777656_u64,13788994777875682778_u64];
(*_27) = 3676957783_u32 - 130977923_u32;
_32 = _19 >> (*_27);
(*_11) = &_9;
_16 = _26;
(*_11) = &_8;
_34.0 = [(*_27),(*_27),(*_27),(*_27),(*_27),(*_27)];
_34.3.1 = core::ptr::addr_of!(_2);
Goto(bb20)
}
bb20 = {
(*_27) = !3949640469_u32;
_34.3.2 = _3 - _3;
_34.1 = (*_1) <= (*_1);
(*_27) = !1803013530_u32;
_23 = [146955263841423621762960094840314374384_i128,3444767922783991807702644067306296155_i128,163510794388510111488392724568069166788_i128,15217068993253598439795632993931928588_i128,(-32821208396098430576117800873985528950_i128)];
(*_11) = &_9;
_8 = (*_1) - (*_1);
(*_11) = &_20;
Goto(bb21)
}
bb21 = {
_15 = _34.1;
_38 = &_2;
_1 = &_9;
_25 = 18396061214530268196_usize - 12276628035052659646_usize;
_39 = _19 ^ _32;
(*_11) = &_8;
(*_27) = 815160596_u32 << _39;
(*_11) = &_9;
_17 = 76_u8;
(*_27) = 2640161951_u32;
(*_27) = 2928836309_u32 - 1193847376_u32;
(*_27) = 1419281398_u32;
_15 = _34.1;
(*_27) = 3603054154_u32 - 1044453262_u32;
Goto(bb22)
}
bb22 = {
_39 = 310605207_i32 as isize;
_4 = _7;
_29 = !_17;
(*_11) = &_20;
_8 = RET as f64;
_35 = _29 & _17;
_8 = (-87889321903073114385392312168736244767_i128) as f64;
_43 = _4;
_29 = _19 as u8;
_35 = _17;
_20 = _8 - _9;
(*_27) = 718936346_u32;
_31 = (-30918712867748811483005183206552536677_i128);
(*_11) = &_9;
(*_11) = &_20;
_36 = &mut _25;
(*_36) = 3705223310689565943_usize >> _39;
(*_11) = &_8;
_40 = [_10,_10,_10,_10,_10,_10,_10];
(*_11) = &_9;
(*_36) = 9824628535316269173_usize;
_20 = -(*_1);
(*_27) = (*_36) as u32;
Goto(bb23)
}
bb23 = {
(*_36) = _31 as usize;
_24 = [88499167_i32,1348590903_i32,2013531521_i32];
_44 = [18172700621428406244_u64,9415680425727207372_u64,13221561862562843846_u64];
_46 = _31 as isize;
(*_36) = 16863902446692779574_usize ^ 4_usize;
_37 = 47071_u16;
_34.3.2 = !(*_38);
(*_11) = &_20;
_34.0 = [(*_27),(*_27),(*_27),(*_27),(*_27),(*_27)];
(*_27) = 3206608874_u32 >> (*_36);
(*_27) = 1132883345_u32 >> _16;
_52 = &(*_1);
(*_36) = !0_usize;
(*_27) = 2403569411_u32;
_10 = 11_i8 ^ (-107_i8);
_48 = _44;
Call((*_27) = core::intrinsics::bswap(307897734_u32), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_36) = 2_usize & 4_usize;
_56 = _31 << (*_38);
_47 = core::ptr::addr_of_mut!(_34.1);
_5 = [9764212474761374690_u64,3238076828552421016_u64,16388356169379588196_u64,18240859662987672626_u64];
(*_11) = &_9;
_45.1 = Adt20::Variant1 { fld0: (*_47),fld1: _32 };
_30 = Adt51::Variant1 { fld0: Move(_45.1),fld1: (-140945734_i32) };
(*_27) = !3793046881_u32;
(*_27) = !3164619413_u32;
_9 = (*_52) - (*_52);
_23 = [_31,_31,_31,_56,_56];
(*_11) = &(*_52);
(*_27) = !2453877036_u32;
(*_27) = 704820516_u32;
(*_27) = _37 as u32;
_49 = (Move(_11), _43);
_34.3.0 = _35;
(*_27) = 134687135_u32;
_59.2 = (*_47) as i32;
_2 = _3 | _6;
_43 = _49.1;
match (*_27) {
0 => bb11,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
134687135 => bb32,
_ => bb31
}
}
bb25 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb26 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb27 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb28 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb29 = {
(*_11) = &_9;
match _10 {
0 => bb4,
1 => bb5,
84 => bb7,
_ => bb6
}
}
bb30 = {
_14 = !2018906760_u32;
(*_11) = &_8;
_3 = 211682814083047345261360903658608335949_u128 as i16;
(*_11) = &_9;
(*_11) = &_8;
Goto(bb9)
}
bb31 = {
Return()
}
bb32 = {
_39 = RET + Field::<isize>(Variant(Field::<Adt20>(Variant(_30, 1), 0), 1), 1);
_11 = core::ptr::addr_of!(_1);
(*_47) = Field::<bool>(Variant(Field::<Adt20>(Variant(_30, 1), 0), 1), 0);
match (*_27) {
134687135 => bb33,
_ => bb8
}
}
bb33 = {
(*_27) = 3538473636_u32 >> _56;
match _31 {
0 => bb14,
1 => bb17,
309363654053189651980369424225215674779 => bb35,
_ => bb34
}
}
bb34 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb35 = {
_43 = _7;
_59.1 = (_43, _59.2, (*_52), _43);
(*_27) = 2519847086_u32 << _59.2;
(*_47) = !Field::<bool>(Variant(Field::<Adt20>(Variant(_30, 1), 0), 1), 0);
_9 = (*_1);
Goto(bb36)
}
bb36 = {
_42 = _35 as f32;
_31 = _34.3.0 as i128;
_45.1 = Move(Field::<Adt20>(Variant(_30, 1), 0));
_53 = Move(_34.3.1);
(*_27) = Field::<isize>(Variant(_45.1, 1), 1) as u32;
(*_47) = _20 > (*_1);
(*_11) = &_8;
_39 = (*_27) as isize;
_17 = !_35;
(*_47) = Field::<bool>(Variant(_45.1, 1), 0) | _15;
_59.1.2 = (*_52) + (*_52);
(*_27) = 260897836_u32 << _59.1.1;
_26 = _7 as i64;
(*_47) = (*_27) <= (*_27);
_59.3 = _26;
match _37 {
0 => bb37,
1 => bb38,
2 => bb39,
3 => bb40,
47071 => bb42,
_ => bb41
}
}
bb37 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb38 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb39 = {
Return()
}
bb40 = {
_39 = RET + Field::<isize>(Variant(Field::<Adt20>(Variant(_30, 1), 0), 1), 1);
_11 = core::ptr::addr_of!(_1);
(*_47) = Field::<bool>(Variant(Field::<Adt20>(Variant(_30, 1), 0), 1), 0);
match (*_27) {
134687135 => bb33,
_ => bb8
}
}
bb41 = {
_9 = (*_1);
_10 = (-128_i8);
_2 = 998949056_u32 as i16;
_4 = _7;
_8 = _9 - _9;
_1 = &_9;
_7 = _4;
_10 = 84_i8;
_2 = -_6;
_7 = _4;
_6 = _3 | _2;
_1 = &_8;
_11 = core::ptr::addr_of!(_1);
_8 = _9;
Goto(bb3)
}
bb42 = {
(*_47) = (*_27) >= (*_27);
_32 = Field::<isize>(Variant(_45.1, 1), 1);
(*_47) = (*_27) <= (*_27);
_8 = -_20;
(*_11) = &(*_52);
_6 = _42 as i16;
(*_36) = !6_usize;
_69 = _7;
_63 = _59.1.1 - _59.2;
_27 = core::ptr::addr_of!((*_27));
(*_36) = 9068998387864999784_usize;
place!(Field::<i32>(Variant(_30, 1), 1)) = _59.2 * _63;
(*_27) = Field::<i32>(Variant(_30, 1), 1) as u32;
(*_47) = (*_27) != (*_27);
_59.1.0 = _49.1;
_57 = _59.1.3;
_52 = &_9;
_64 = _10;
_61 = Move(_38);
Goto(bb43)
}
bb43 = {
_59.1.0 = _4;
_66 = &_37;
(*_11) = &_8;
_34.2 = [_59.3,_26,_16,_26,_26,_26];
(*_11) = &(*_52);
(*_27) = !61242871_u32;
(*_27) = 4109557106_u32;
(*_27) = 3213325223_u32 - 1116766044_u32;
_70 = _26;
_34.1 = _15 | Field::<bool>(Variant(_45.1, 1), 0);
Goto(bb44)
}
bb44 = {
(*_36) = !6_usize;
_54 = (*_47);
RET = _46;
_59.1.1 = _42 as i32;
(*_27) = 2682042505_u32;
RET = _32 ^ _32;
(*_11) = &_59.1.2;
(*_36) = _49.1 as usize;
(*_11) = &(*_52);
_34.3.1 = core::ptr::addr_of!(_34.3.2);
(*_47) = _15 & _54;
_47 = core::ptr::addr_of_mut!((*_47));
_8 = -(*_52);
(*_36) = !16919962853664456539_usize;
(*_47) = Field::<bool>(Variant(_45.1, 1), 0) | _54;
(*_27) = !158814568_u32;
(*_47) = Field::<bool>(Variant(_45.1, 1), 0) ^ _54;
(*_36) = _70 as usize;
_53 = core::ptr::addr_of!(_34.3.2);
(*_11) = &_59.1.2;
(*_53) = !_2;
(*_11) = &(*_52);
(*_36) = _59.1.3 as usize;
Goto(bb45)
}
bb45 = {
_19 = -_39;
(*_11) = &_8;
_71 = _32 ^ RET;
(*_11) = &(*_52);
_26 = Field::<i32>(Variant(_30, 1), 1) as i64;
_69 = _59.1.3;
_33 = core::ptr::addr_of_mut!(_60);
(*_47) = !_15;
(*_11) = &_20;
(*_33) = [(*_36),(*_36),(*_36),(*_36)];
(*_53) = 5278335670377015376989182578443621165_u128 as i16;
(*_11) = Move(_52);
_72 = -_42;
_6 = _2;
RET = _56 as isize;
(*_11) = &_8;
_20 = (*_1) + (*_1);
_78 = core::ptr::addr_of!((*_27));
(*_33) = [(*_36),(*_36),(*_36),(*_36)];
(*_47) = (*_78) == (*_27);
_77 = _42 + _42;
_55 = _77 + _77;
_40 = [_64,_10,_10,_10,_64,_64,_10];
_83 = core::ptr::addr_of_mut!(_23);
(*_47) = !_54;
match (*_66) {
0 => bb32,
1 => bb25,
2 => bb46,
3 => bb47,
47071 => bb49,
_ => bb48
}
}
bb46 = {
RET = (-113821596046400928413942378133693436166_i128) as isize;
_7 = _4;
_1 = &_9;
_6 = _2 ^ _2;
_1 = &_8;
_7 = _4;
Goto(bb2)
}
bb47 = {
(*_27) = !3949640469_u32;
_34.3.2 = _3 - _3;
_34.1 = (*_1) <= (*_1);
(*_27) = !1803013530_u32;
_23 = [146955263841423621762960094840314374384_i128,3444767922783991807702644067306296155_i128,163510794388510111488392724568069166788_i128,15217068993253598439795632993931928588_i128,(-32821208396098430576117800873985528950_i128)];
(*_11) = &_9;
_8 = (*_1) - (*_1);
(*_11) = &_20;
Goto(bb21)
}
bb48 = {
(*_47) = (*_27) >= (*_27);
_32 = Field::<isize>(Variant(_45.1, 1), 1);
(*_47) = (*_27) <= (*_27);
_8 = -_20;
(*_11) = &(*_52);
_6 = _42 as i16;
(*_36) = !6_usize;
_69 = _7;
_63 = _59.1.1 - _59.2;
_27 = core::ptr::addr_of!((*_27));
(*_36) = 9068998387864999784_usize;
place!(Field::<i32>(Variant(_30, 1), 1)) = _59.2 * _63;
(*_27) = Field::<i32>(Variant(_30, 1), 1) as u32;
(*_47) = (*_27) != (*_27);
_59.1.0 = _49.1;
_57 = _59.1.3;
_52 = &_9;
_64 = _10;
_61 = Move(_38);
Goto(bb43)
}
bb49 = {
_80 = _39 ^ Field::<isize>(Variant(_45.1, 1), 1);
(*_53) = _63 as i16;
(*_11) = &_59.1.2;
(*_78) = 3096476040_u32;
_34.3.0 = _29 * _35;
Goto(bb50)
}
bb50 = {
Call(_85 = dump_var(Move(_17), Move(_80), Move(_57), Move(_44)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_85 = dump_var(Move(_40), Move(_6), Move(_39), Move(_4)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_85 = dump_var(Move(_29), Move(_15), Move(_19), Move(_7)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_85 = dump_var(Move(_43), Move(_71), Move(_37), Move(_32)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_85 = dump_var(Move(_23), Move(_3), _86, _86), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static f64,mut _2: *const &'static f64,mut _3: f64) -> [u64; 4] {
mir! {
type RET = [u64; 4];
let _4: char;
let _5: *mut u16;
let _6: isize;
let _7: isize;
let _8: *mut &'static mut *const u32;
let _9: (char, i32, f64, char);
let _10: [i32; 1];
let _11: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _12: [i128; 5];
let _13: char;
let _14: f32;
let _15: bool;
let _16: Adt20;
let _17: u16;
let _18: *mut [usize; 4];
let _19: &'static Adt18;
let _20: &'static (Adt58,);
let _21: f64;
let _22: i64;
let _23: *const [u16; 1];
let _24: i8;
let _25: [usize; 4];
let _26: u64;
let _27: *mut &'static mut *const u32;
let _28: i128;
let _29: [u16; 1];
let _30: isize;
let _31: i32;
let _32: bool;
let _33: f32;
let _34: *mut Adt18;
let _35: *mut &'static Adt18;
let _36: f32;
let _37: f32;
let _38: &'static mut u32;
let _39: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _40: &'static isize;
let _41: (i16, (char, i32, f64, char), i32, i64);
let _42: u64;
let _43: u64;
let _44: &'static (Adt58,);
let _45: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _46: *mut u32;
let _47: bool;
let _48: [u16; 1];
let _49: usize;
let _50: *mut [i128; 5];
let _51: bool;
let _52: f32;
let _53: [i32; 3];
let _54: u128;
let _55: bool;
let _56: u128;
let _57: f32;
let _58: *const f32;
let _59: &'static isize;
let _60: *mut (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _61: u64;
let _62: (u16, *mut bool);
let _63: [u64; 3];
let _64: i64;
let _65: *mut [i128; 5];
let _66: *mut u32;
let _67: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _68: *mut *mut &'static Adt18;
let _69: [u32; 7];
let _70: *mut u32;
let _71: isize;
let _72: f64;
let _73: [i128; 5];
let _74: bool;
let _75: u8;
let _76: isize;
let _77: i64;
let _78: f32;
let _79: char;
let _80: f32;
let _81: f64;
let _82: *const i16;
let _83: (u16, *mut bool);
let _84: [i32; 1];
let _85: char;
let _86: ((*mut bool,), usize);
let _87: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _88: (Adt58,);
let _89: i32;
let _90: i32;
let _91: *mut Adt18;
let _92: *const f32;
let _93: &'static (Adt58,);
let _94: isize;
let _95: f64;
let _96: u8;
let _97: ();
let _98: ();
{
RET = [6393127606300943868_u64,11535244218863753840_u64,18111764636965867141_u64,923736489503021482_u64];
RET = [5994698906385488807_u64,12478894483960235045_u64,8932670293243736940_u64,4674775268337944308_u64];
_4 = '\u{41936}';
_1 = &_3;
_4 = '\u{9c36e}';
_2 = core::ptr::addr_of!(_1);
_4 = '\u{3427d}';
RET = [15074945623531361246_u64,1856937985884767123_u64,13744446340180386879_u64,16918691559102391089_u64];
_4 = '\u{a7393}';
RET = [2099329815629613730_u64,7394915671936471688_u64,10277994639924397718_u64,15688642465388554272_u64];
RET = [1743056638575790778_u64,3716851633502360160_u64,7284617645851365887_u64,17949753793227187348_u64];
RET = [1810991193197316863_u64,12499650383876659281_u64,10251590921790690299_u64,8216399701074087122_u64];
RET = [9801221545118532835_u64,11976816126588514960_u64,16022377998449421467_u64,432205309297271783_u64];
_6 = (-107_isize) >> (-9223372036854775808_isize);
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
_3 = (-8568818524642525194837937165640967217_i128) as f64;
RET = [12522076119023382461_u64,15933286350412158795_u64,16224061619949477294_u64,8554235329116052909_u64];
_4 = '\u{d254c}';
RET = [3159718034725964296_u64,15635609220299581061_u64,11050028719293676011_u64,5214430872251202785_u64];
_4 = '\u{38b8c}';
_2 = core::ptr::addr_of!((*_2));
_4 = '\u{29d70}';
_2 = core::ptr::addr_of!((*_2));
_4 = '\u{62d05}';
_7 = true as isize;
Goto(bb1)
}
bb1 = {
RET = [7408227860721613600_u64,12665178726345716367_u64,4283110600656633668_u64,10969790734876084807_u64];
_4 = '\u{2e794}';
_9.1 = !(-398055220_i32);
_9.2 = _3 + _3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_9 = (_4, (-1510233528_i32), _3, _4);
(*_2) = &_3;
_1 = &_9.2;
Goto(bb2)
}
bb2 = {
_9.2 = _3 - _3;
_4 = _9.3;
(*_2) = &_3;
(*_2) = &_9.2;
Goto(bb3)
}
bb3 = {
_9.0 = _4;
Goto(bb4)
}
bb4 = {
(*_2) = &_3;
(*_2) = &_9.2;
(*_2) = &_3;
RET = [2933532987733118095_u64,1364205900871023286_u64,8942442252962285196_u64,8108983920674508260_u64];
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_10 = [_9.1];
(*_2) = &_3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_6 = _7 & _7;
_6 = _7;
(*_2) = &_3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_6 = _7 << _9.1;
match _9.1 {
0 => bb3,
1 => bb5,
340282366920938463463374607430257977928 => bb7,
_ => bb6
}
}
bb5 = {
_9.0 = _4;
Goto(bb4)
}
bb6 = {
RET = [7408227860721613600_u64,12665178726345716367_u64,4283110600656633668_u64,10969790734876084807_u64];
_4 = '\u{2e794}';
_9.1 = !(-398055220_i32);
_9.2 = _3 + _3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_9 = (_4, (-1510233528_i32), _3, _4);
(*_2) = &_3;
_1 = &_9.2;
Goto(bb2)
}
bb7 = {
(*_2) = &_3;
_11.3.0 = 2068258664_u32 as u8;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_14 = 4_usize as f32;
(*_2) = &_3;
_13 = _4;
_9.2 = (-148204743542568657250970443544732421226_i128) as f64;
_11.3.2 = (-13757_i16) | (-20133_i16);
_9.0 = _9.3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_12 = [164927075340966359638500519006806914108_i128,167183701487675484072316355901897373341_i128,1464657015335594504046762109819090618_i128,(-2343879166991725753751770269438334418_i128),(-30283954077513423444217261103159492049_i128)];
(*_2) = &_3;
(*_2) = &_9.2;
_11.0 = [4248483773_u32,269471058_u32,289966372_u32,3581296097_u32,717178064_u32,484697185_u32];
Goto(bb8)
}
bb8 = {
_11.3.0 = 64_u8 * 45_u8;
_16 = Adt20::Variant0 { fld0: 10131693898847042087_usize,fld1: _11.3.2 };
_9 = (_13, 172606100_i32, _3, _4);
_13 = _4;
(*_2) = &_3;
_9.3 = _9.0;
RET = [15681218984339990649_u64,4123880651301856611_u64,5182675979338735699_u64,7732867829059056216_u64];
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_11.1 = true;
_4 = _9.3;
_12 = [168358237166954722265576195329475924434_i128,(-83555328490487814642430689463841370336_i128),(-151938816390507393548793652708884168263_i128),(-27005368291154586694488008590811707957_i128),62812993357378410742108658233142658294_i128];
(*_2) = &_9.2;
_1 = &_3;
RET = [7141477277709454140_u64,14882526027220486717_u64,16773724479367383245_u64,15654876326806991635_u64];
_15 = !_11.1;
(*_2) = &_9.2;
_1 = &_3;
_11.1 = _15 | _15;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_6 = _7;
Goto(bb9)
}
bb9 = {
(*_2) = &_9.2;
_15 = !_11.1;
_11.3.2 = Field::<i16>(Variant(_16, 0), 1);
(*_2) = &_3;
_9.0 = _9.3;
_14 = (*_1) as f32;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!(_17);
(*_2) = &_3;
(*_5) = _14 as u16;
_12 = [22188068744734957684566726956526787536_i128,57640193147987889623929279086842422667_i128,55433091392340959116325671756841774098_i128,(-67308382785040138069036407043527325963_i128),144520950471885414265479229006700727481_i128];
(*_2) = &_9.2;
(*_5) = 280533560_u32 as u16;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
match _9.1 {
0 => bb4,
1 => bb10,
2 => bb11,
172606100 => bb13,
_ => bb12
}
}
bb10 = {
_11.3.0 = 64_u8 * 45_u8;
_16 = Adt20::Variant0 { fld0: 10131693898847042087_usize,fld1: _11.3.2 };
_9 = (_13, 172606100_i32, _3, _4);
_13 = _4;
(*_2) = &_3;
_9.3 = _9.0;
RET = [15681218984339990649_u64,4123880651301856611_u64,5182675979338735699_u64,7732867829059056216_u64];
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_11.1 = true;
_4 = _9.3;
_12 = [168358237166954722265576195329475924434_i128,(-83555328490487814642430689463841370336_i128),(-151938816390507393548793652708884168263_i128),(-27005368291154586694488008590811707957_i128),62812993357378410742108658233142658294_i128];
(*_2) = &_9.2;
_1 = &_3;
RET = [7141477277709454140_u64,14882526027220486717_u64,16773724479367383245_u64,15654876326806991635_u64];
_15 = !_11.1;
(*_2) = &_9.2;
_1 = &_3;
_11.1 = _15 | _15;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_6 = _7;
Goto(bb9)
}
bb11 = {
_9.0 = _4;
Goto(bb4)
}
bb12 = {
RET = [7408227860721613600_u64,12665178726345716367_u64,4283110600656633668_u64,10969790734876084807_u64];
_4 = '\u{2e794}';
_9.1 = !(-398055220_i32);
_9.2 = _3 + _3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_9 = (_4, (-1510233528_i32), _3, _4);
(*_2) = &_3;
_1 = &_9.2;
Goto(bb2)
}
bb13 = {
_17 = 41253_u16 - 48723_u16;
_11.2 = [5748957645056772971_i64,(-7885198838633186130_i64),7791255596074738274_i64,3165569237233656315_i64,8702528597559792998_i64,(-7126024370195186894_i64)];
_17 = !4192_u16;
_9.1 = 476609324_i32 - (-1372018908_i32);
(*_2) = &_3;
(*_2) = &_9.2;
_25 = [6_usize,5883363008681950855_usize,5_usize,9299957893454082923_usize];
(*_2) = &_3;
_1 = &_9.2;
_24 = 126_i8 | 9_i8;
_15 = !_11.1;
_15 = (*_5) > (*_5);
(*_5) = 39607_u16 >> _11.3.2;
_9.3 = _9.0;
Goto(bb14)
}
bb14 = {
(*_5) = 53837_u16 | 61117_u16;
_11.0 = [1928201607_u32,264745824_u32,3179874335_u32,3453851302_u32,2605341260_u32,1019269342_u32];
(*_5) = 59886_u16;
(*_5) = !22805_u16;
(*_5) = _6 as u16;
Goto(bb15)
}
bb15 = {
_25 = [1_usize,7629603022612998540_usize,2_usize,7_usize];
_12 = [(-20691906479972066193310375964083059284_i128),(-93537617706643352168892881037458321583_i128),(-62693282217410992441504640563564051798_i128),(-92700506243726889601437607029183621884_i128),(-759342641514142619978208955200847494_i128)];
(*_5) = 11951_u16 ^ 41827_u16;
(*_2) = &_3;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = [_9.1];
_16 = Adt20::Variant1 { fld0: _11.1,fld1: _6 };
(*_2) = &_3;
(*_2) = &_9.2;
_4 = _9.0;
_28 = 147018222503673313703323782352477797985_i128 * 127982424778060068933787248118522740407_i128;
(*_5) = 34609_u16 | 41981_u16;
(*_5) = 25217_u16 & 38594_u16;
(*_2) = &_3;
(*_2) = &_9.2;
_29 = [(*_5)];
_2 = core::ptr::addr_of!((*_2));
place!(Field::<isize>(Variant(_16, 1), 1)) = _7;
Call(RET = fn16(), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*_5) = _24 as u16;
(*_5) = 34453_u16 << _28;
RET = [4077059409498474720_u64,3964107156094347017_u64,4922413488155679107_u64,17420297393756573427_u64];
_30 = Field::<isize>(Variant(_16, 1), 1) ^ _7;
_11.0 = [372410485_u32,3878002961_u32,1149651883_u32,3539626435_u32,3894094187_u32,3084611173_u32];
(*_2) = &_3;
(*_5) = 56336_u16 - 45053_u16;
(*_5) = 64448_u16 >> _24;
_18 = core::ptr::addr_of_mut!(_25);
_10 = [_9.1];
(*_18) = [5708601593007939004_usize,7_usize,4_usize,4868991753820032094_usize];
(*_2) = &_9.2;
(*_5) = (*_1) as u16;
_26 = 1795376251107698830_u64 + 11895660137927706353_u64;
(*_18) = [17670170198735935047_usize,5_usize,4_usize,7122586115214195293_usize];
(*_18) = [6667967087958298056_usize,4_usize,5574678528245963253_usize,14164597097525520934_usize];
(*_5) = 32915_u16 >> _24;
place!(Field::<bool>(Variant(_16, 1), 0)) = _11.1;
_11.3.0 = 138_u8;
match _11.3.0 {
0 => bb10,
1 => bb2,
2 => bb14,
3 => bb17,
4 => bb18,
5 => bb19,
138 => bb21,
_ => bb20
}
}
bb17 = {
(*_2) = &_9.2;
_15 = !_11.1;
_11.3.2 = Field::<i16>(Variant(_16, 0), 1);
(*_2) = &_3;
_9.0 = _9.3;
_14 = (*_1) as f32;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!(_17);
(*_2) = &_3;
(*_5) = _14 as u16;
_12 = [22188068744734957684566726956526787536_i128,57640193147987889623929279086842422667_i128,55433091392340959116325671756841774098_i128,(-67308382785040138069036407043527325963_i128),144520950471885414265479229006700727481_i128];
(*_2) = &_9.2;
(*_5) = 280533560_u32 as u16;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
match _9.1 {
0 => bb4,
1 => bb10,
2 => bb11,
172606100 => bb13,
_ => bb12
}
}
bb18 = {
_9.0 = _4;
Goto(bb4)
}
bb19 = {
_9.2 = _3 - _3;
_4 = _9.3;
(*_2) = &_3;
(*_2) = &_9.2;
Goto(bb3)
}
bb20 = {
RET = [7408227860721613600_u64,12665178726345716367_u64,4283110600656633668_u64,10969790734876084807_u64];
_4 = '\u{2e794}';
_9.1 = !(-398055220_i32);
_9.2 = _3 + _3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_9 = (_4, (-1510233528_i32), _3, _4);
(*_2) = &_3;
_1 = &_9.2;
Goto(bb2)
}
bb21 = {
_23 = core::ptr::addr_of!(_29);
_32 = Field::<bool>(Variant(_16, 1), 0);
Goto(bb22)
}
bb22 = {
_11.3.0 = !225_u8;
_25 = [5_usize,16205731328417794336_usize,3_usize,17378826863501513743_usize];
_11.3.0 = 24_u8;
(*_18) = [2_usize,1_usize,3_usize,0_usize];
(*_18) = [2_usize,15873060395129786892_usize,16086857592060323762_usize,9994580346138931218_usize];
(*_5) = 14502_u16 - 6085_u16;
(*_5) = 3491_u16;
(*_5) = 33659_u16;
(*_2) = &_3;
(*_5) = 23410_u16 | 44964_u16;
(*_5) = _13 as u16;
_22 = _9.1 as i64;
_13 = _9.0;
(*_5) = _11.3.2 as u16;
Goto(bb23)
}
bb23 = {
(*_23) = [(*_5)];
(*_23) = [(*_5)];
(*_5) = !20813_u16;
(*_18) = [14681102548428694012_usize,6_usize,6_usize,909979998363990020_usize];
_26 = !12466928292554766449_u64;
(*_5) = !27311_u16;
(*_5) = 44256_u16 * 36941_u16;
_22 = (-6822663234954532491_i64) & (-6831079621395298655_i64);
_37 = _14;
Goto(bb24)
}
bb24 = {
_13 = _9.3;
_23 = core::ptr::addr_of!((*_23));
_9.1 = 1551584230_i32 * 208705251_i32;
Goto(bb25)
}
bb25 = {
(*_18) = [0_usize,7_usize,5_usize,16218339217255701863_usize];
(*_18) = [16836545792958590657_usize,2575531536358147193_usize,3_usize,0_usize];
_28 = _22 as i128;
Goto(bb26)
}
bb26 = {
(*_23) = [(*_5)];
(*_18) = [16802305948826984919_usize,3_usize,1_usize,10141324608049644150_usize];
(*_23) = [(*_5)];
_13 = _4;
_1 = &_9.2;
(*_5) = !7036_u16;
(*_23) = [(*_5)];
_41 = (_11.3.2, _9, _9.1, _22);
(*_5) = 60685_u16 << _24;
(*_2) = &_41.1.2;
(*_5) = _26 as u16;
(*_5) = 4012_u16 ^ 6925_u16;
(*_23) = [(*_5)];
_41.3 = _22;
_11.0 = [2529576131_u32,4090022630_u32,2441901791_u32,944934430_u32,1395481046_u32,1554696030_u32];
(*_2) = &_9.2;
(*_2) = &_3;
Call(_6 = core::intrinsics::bswap(Field::<isize>(Variant(_16, 1), 1)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_23) = [(*_5)];
(*_23) = [(*_5)];
_4 = _13;
(*_18) = [7461386162717152821_usize,16471427835682312414_usize,16205646605267019812_usize,565665598340116982_usize];
(*_2) = &_41.1.2;
(*_2) = &_9.2;
(*_23) = [(*_5)];
(*_5) = _11.3.2 as u16;
(*_5) = !50308_u16;
_43 = _26 << _9.1;
(*_2) = &_41.1.2;
(*_18) = [14420169608887436208_usize,906756318576264751_usize,5_usize,3948856591819124716_usize];
_45 = &mut _11;
(*_45).1 = Field::<bool>(Variant(_16, 1), 0) & Field::<bool>(Variant(_16, 1), 0);
(*_5) = !4037_u16;
(*_23) = [(*_5)];
(*_23) = [(*_5)];
(*_2) = &_9.2;
_50 = core::ptr::addr_of_mut!(_12);
(*_45).3.0 = !80_u8;
_41.1.2 = _26 as f64;
(*_45).1 = _15;
(*_45).2 = [_22,_22,_22,_41.3,_22,_22];
(*_45).2 = [_22,_41.3,_41.3,_41.3,_22,_41.3];
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
Goto(bb28)
}
bb28 = {
(*_5) = 40167_u16 ^ 38034_u16;
(*_50) = [_28,_28,_28,_28,_28];
(*_2) = &_3;
(*_45).2 = [_41.3,_22,_41.3,_22,_22,_22];
(*_45).2 = [_41.3,_41.3,_22,_41.3,_22,_22];
Goto(bb29)
}
bb29 = {
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_45).2 = [_41.3,_41.3,_41.3,_22,_22,_41.3];
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_5) = 13482_u16 | 971_u16;
(*_5) = 22381_u16;
(*_2) = &_41.1.2;
_29 = [(*_5)];
_12 = [_28,_28,_28,_28,_28];
_23 = core::ptr::addr_of!((*_23));
(*_45).3.2 = _41.0 ^ _41.0;
Goto(bb30)
}
bb30 = {
(*_2) = &_3;
(*_45).0 = [2036948450_u32,288891323_u32,3582198230_u32,495975151_u32,374793284_u32,901773402_u32];
(*_23) = [(*_5)];
_33 = -_14;
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
_57 = _33 + _33;
_14 = -_33;
(*_23) = [(*_5)];
(*_45).3.2 = _41.0 | _41.0;
(*_45).3.2 = _41.0 + _41.0;
Goto(bb31)
}
bb31 = {
_58 = core::ptr::addr_of!(_37);
(*_5) = (*_45).1 as u16;
(*_18) = [11240284645933579722_usize,11853808410382062984_usize,7_usize,6_usize];
(*_23) = [(*_5)];
(*_45).3.0 = 118_u8 ^ 8_u8;
(*_50) = [_28,_28,_28,_28,_28];
(*_23) = [(*_5)];
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
Goto(bb32)
}
bb32 = {
_10 = [_9.1];
(*_45).0 = [759499579_u32,1066781355_u32,3656000315_u32,633688352_u32,4093749711_u32,4009630088_u32];
(*_5) = 29586_u16 | 44935_u16;
(*_45).1 = Field::<bool>(Variant(_16, 1), 0);
_24 = (*_5) as i8;
(*_23) = [(*_5)];
_32 = (*_45).1;
(*_45).3.2 = -_41.0;
(*_23) = [(*_5)];
(*_45).2 = [_22,_22,_41.3,_22,_41.3,_22];
_40 = &place!(Field::<isize>(Variant(_16, 1), 1));
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
_49 = 1_usize << (*_45).3.0;
(*_2) = &_9.2;
(*_50) = [_28,_28,_28,_28,_28];
(*_5) = 307302024053455599677935242521145396321_u128 as u16;
_53 = [_9.1,_41.2,_41.1.1];
(*_50) = [_28,_28,_28,_28,_28];
(*_45).3.0 = 11_u8 - 105_u8;
(*_45).1 = Field::<bool>(Variant(_16, 1), 0);
(*_45).1 = Field::<bool>(Variant(_16, 1), 0) ^ _32;
_62.1 = core::ptr::addr_of_mut!((*_45).1);
_15 = (*_1) > (*_1);
(*_2) = &_3;
(*_45).0 = [638350856_u32,1147055580_u32,2210840934_u32,3460806330_u32,4012630615_u32,4007796411_u32];
Goto(bb33)
}
bb33 = {
(*_45).3.0 = !70_u8;
(*_45).3.0 = (*_45).1 as u8;
(*_50) = [_28,_28,_28,_28,_28];
(*_45).3.2 = _41.0;
(*_58) = (*_45).3.2 as f32;
(*_45).0 = [301483997_u32,2951727432_u32,2678672787_u32,1257464032_u32,2331491643_u32,386736307_u32];
_3 = _41.1.2 + _41.1.2;
_41.1.2 = _3;
place!(Field::<bool>(Variant(_16, 1), 0)) = (*_45).3.2 == (*_45).3.2;
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_5) = 50740_u16;
place!(Field::<isize>(Variant(_16, 1), 1)) = _30 * _7;
(*_45).2 = [_41.3,_22,_22,_22,_22,_22];
match (*_5) {
0 => bb19,
1 => bb14,
2 => bb34,
50740 => bb36,
_ => bb35
}
}
bb34 = {
_9.2 = _3 - _3;
_4 = _9.3;
(*_2) = &_3;
(*_2) = &_9.2;
Goto(bb3)
}
bb35 = {
(*_2) = &_9.2;
_15 = !_11.1;
_11.3.2 = Field::<i16>(Variant(_16, 0), 1);
(*_2) = &_3;
_9.0 = _9.3;
_14 = (*_1) as f32;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!(_17);
(*_2) = &_3;
(*_5) = _14 as u16;
_12 = [22188068744734957684566726956526787536_i128,57640193147987889623929279086842422667_i128,55433091392340959116325671756841774098_i128,(-67308382785040138069036407043527325963_i128),144520950471885414265479229006700727481_i128];
(*_2) = &_9.2;
(*_5) = 280533560_u32 as u16;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
match _9.1 {
0 => bb4,
1 => bb10,
2 => bb11,
172606100 => bb13,
_ => bb12
}
}
bb36 = {
_21 = _3 + _9.2;
_54 = !134231442272414288707392219795821718856_u128;
(*_23) = [(*_5)];
_39 = &(*_45);
(*_45).0 = [748871672_u32,831213927_u32,2341931974_u32,602739474_u32,2243772996_u32,1093833892_u32];
(*_50) = [_28,_28,_28,_28,_28];
_55 = !(*_45).1;
_5 = core::ptr::addr_of_mut!(_62.0);
(*_45).1 = (*_39).3.2 >= (*_39).3.2;
_12 = [_28,_28,_28,_28,_28];
_64 = -_22;
_48 = (*_23);
(*_5) = _17 >> (*_45).3.2;
(*_45).0 = [2448591661_u32,1224586023_u32,4175778972_u32,884304098_u32,2128041005_u32,1273845048_u32];
_55 = !(*_45).1;
_52 = -(*_58);
_4 = _13;
_68 = core::ptr::addr_of_mut!(_35);
(*_5) = _17 - _17;
_62.0 = _17 & _17;
_28 = 108450955231370839211162059540098271037_i128 - (-92731178550611634715901369772898292239_i128);
(*_58) = Field::<isize>(Variant(_16, 1), 1) as f32;
Goto(bb37)
}
bb37 = {
_18 = core::ptr::addr_of_mut!((*_18));
(*_45).2 = [_64,_64,_41.3,_64,_41.3,_41.3];
_67.2 = [_22,_64,_22,_22,_22,_41.3];
_28 = 124516005491273275688545495262440164461_i128 + 87508210435431579598124289571543518311_i128;
(*_50) = [_28,_28,_28,_28,_28];
_52 = 3172158274_u32 as f32;
(*_5) = !_17;
(*_2) = &_9.2;
(*_50) = [_28,_28,_28,_28,_28];
(*_23) = [(*_5)];
(*_18) = [_49,_49,_49,_49];
(*_18) = [_49,_49,_49,_49];
(*_18) = [_49,_49,_49,_49];
_67.3.2 = (*_39).3.2 & (*_45).3.2;
_50 = core::ptr::addr_of_mut!(_12);
(*_45).2 = [_22,_22,_41.3,_22,_41.3,_41.3];
_49 = 2_usize;
_26 = !RET[_49];
(*_45).1 = (*_39).3.0 <= (*_39).3.0;
(*_45).0[_49] = 4267984789_u32;
match (*_45).0[_49] {
0 => bb31,
1 => bb13,
2 => bb3,
3 => bb32,
4 => bb34,
4267984789 => bb39,
_ => bb38
}
}
bb38 = {
_11.3.0 = 64_u8 * 45_u8;
_16 = Adt20::Variant0 { fld0: 10131693898847042087_usize,fld1: _11.3.2 };
_9 = (_13, 172606100_i32, _3, _4);
_13 = _4;
(*_2) = &_3;
_9.3 = _9.0;
RET = [15681218984339990649_u64,4123880651301856611_u64,5182675979338735699_u64,7732867829059056216_u64];
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_11.1 = true;
_4 = _9.3;
_12 = [168358237166954722265576195329475924434_i128,(-83555328490487814642430689463841370336_i128),(-151938816390507393548793652708884168263_i128),(-27005368291154586694488008590811707957_i128),62812993357378410742108658233142658294_i128];
(*_2) = &_9.2;
_1 = &_3;
RET = [7141477277709454140_u64,14882526027220486717_u64,16773724479367383245_u64,15654876326806991635_u64];
_15 = !_11.1;
(*_2) = &_9.2;
_1 = &_3;
_11.1 = _15 | _15;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_6 = _7;
Goto(bb9)
}
bb39 = {
(*_45).1 = _32;
_23 = core::ptr::addr_of!(_48);
(*_50)[_49] = -_28;
(*_45).3.2 = _15 as i16;
(*_45).0[_49] = 2392817288_u32 * 3021403702_u32;
(*_45).0[_49] = !1062077589_u32;
(*_45).3.2 = _41.0 << (*_50)[_49];
(*_5) = !_17;
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_45).0[_49] = _6 as u32;
(*_50)[_49] = _28 << (*_39).3.0;
(*_23) = _29;
match RET[_49] {
0 => bb40,
1 => bb41,
4922413488155679107 => bb43,
_ => bb42
}
}
bb40 = {
(*_5) = 53837_u16 | 61117_u16;
_11.0 = [1928201607_u32,264745824_u32,3179874335_u32,3453851302_u32,2605341260_u32,1019269342_u32];
(*_5) = 59886_u16;
(*_5) = !22805_u16;
(*_5) = _6 as u16;
Goto(bb15)
}
bb41 = {
_23 = core::ptr::addr_of!(_29);
_32 = Field::<bool>(Variant(_16, 1), 0);
Goto(bb22)
}
bb42 = {
_21 = _3 + _9.2;
_54 = !134231442272414288707392219795821718856_u128;
(*_23) = [(*_5)];
_39 = &(*_45);
(*_45).0 = [748871672_u32,831213927_u32,2341931974_u32,602739474_u32,2243772996_u32,1093833892_u32];
(*_50) = [_28,_28,_28,_28,_28];
_55 = !(*_45).1;
_5 = core::ptr::addr_of_mut!(_62.0);
(*_45).1 = (*_39).3.2 >= (*_39).3.2;
_12 = [_28,_28,_28,_28,_28];
_64 = -_22;
_48 = (*_23);
(*_5) = _17 >> (*_45).3.2;
(*_45).0 = [2448591661_u32,1224586023_u32,4175778972_u32,884304098_u32,2128041005_u32,1273845048_u32];
_55 = !(*_45).1;
_52 = -(*_58);
_4 = _13;
_68 = core::ptr::addr_of_mut!(_35);
(*_5) = _17 - _17;
_62.0 = _17 & _17;
_28 = 108450955231370839211162059540098271037_i128 - (-92731178550611634715901369772898292239_i128);
(*_58) = Field::<isize>(Variant(_16, 1), 1) as f32;
Goto(bb37)
}
bb43 = {
(*_5) = !_17;
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
_31 = _41.2;
(*_18) = [_49,_49,_49,_49];
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_45).1 = _55 | _32;
_29 = [(*_5)];
(*_58) = (*_45).3.2 as f32;
_63[_49] = !_43;
(*_45).0[_49] = _24 as u32;
_9 = (_41.1.3, _53[_49], _41.1.2, _41.1.3);
(*_45).0 = [3931596357_u32,301292419_u32,472993852_u32,2016813824_u32,665048770_u32,3806645624_u32];
(*_18)[_49] = !_49;
(*_18)[_49] = (*_45).0[_49] as usize;
_69 = [(*_45).0[_49],(*_45).0[_49],(*_45).0[_49],(*_45).0[_49],(*_45).0[_49],(*_45).0[_49],(*_45).0[_49]];
(*_58) = _14;
match (*_45).0[_49] {
472993852 => bb44,
_ => bb30
}
}
bb44 = {
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_5) = _17;
(*_45).2 = [_67.2[_49],_22,_41.3,_22,_41.3,_41.3];
(*_45).3.0 = 137_u8 >> _24;
(*_45).1 = _15 & _15;
_83 = ((*_5), Move(_62.1));
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
Goto(bb45)
}
bb45 = {
(*_23) = _29;
_71 = Field::<isize>(Variant(_16, 1), 1);
(*_5) = _83.0;
(*_18)[_49] = _49 << (*_45).0[_49];
(*_45).0[_49] = _69[_49];
_67 = ((*_45).0, (*_45).1, (*_45).2, Move((*_45).3));
(*_18)[_49] = (*_50)[_49] as usize;
(*_18) = [_49,_49,_49,_49];
_3 = -_9.2;
(*_5) = _17 >> (*_50)[_49];
(*_58) = _14 * _57;
Goto(bb46)
}
bb46 = {
(*_45).2 = _67.2;
_41.2 = _9.1 >> (*_50)[_49];
(*_50)[_49] = !_28;
_4 = _9.3;
_61 = _43 + _43;
(*_45).0 = _67.0;
(*_18) = [_49,_49,_49,_49];
(*_18)[_49] = !_49;
_46 = core::ptr::addr_of_mut!((*_45).0[_49]);
(*_50) = [_28,_28,_28,_28,_28];
(*_18) = [_49,_49,_49,_49];
_40 = &_71;
_74 = _32;
_67.3.2 = _26 as i16;
(*_23) = [(*_5)];
_67.3.2 = _41.0;
(*_50)[_49] = _28 & _28;
(*_18)[_49] = _49 | _49;
_73 = [(*_50)[_49],(*_50)[_49],(*_50)[_49],(*_50)[_49],(*_50)[_49]];
(*_5) = _17 * _83.0;
(*_45).2[_49] = _24 as i64;
_36 = -_33;
_86.0 = (Move(_83.1),);
(*_50) = [_73[_49],_73[_49],_28,_28,_73[_49]];
_82 = core::ptr::addr_of!(_41.0);
Goto(bb47)
}
bb47 = {
(*_45).0[_49] = _67.0[_49];
(*_5) = _17 / _83.0;
_49 = (*_18)[_49] >> (*_45).0[_49];
_56 = _54 | _54;
(*_5) = _17;
(*_82) = _67.3.2 ^ _67.3.2;
(*_2) = &_41.1.2;
_41.1.3 = _41.1.0;
_47 = !(*_45).1;
_61 = !_26;
(*_58) = _52 * _52;
_54 = (*_5) as u128;
_62 = (_83.0, Move(_86.0.0));
_13 = _4;
_85 = _4;
(*_50) = [_28,_28,_28,_28,_28];
(*_2) = &_3;
_80 = (*_58);
_86.1 = !_49;
place!(Field::<bool>(Variant(_16, 1), 0)) = _6 <= (*_40);
(*_5) = !_83.0;
(*_58) = _14 * _80;
_9.1 = _41.2 - _41.2;
(*_2) = &_21;
_9.1 = -_41.2;
_41 = (_67.3.2, _9, _9.1, _22);
match _17 {
0 => bb48,
1 => bb49,
2 => bb50,
3 => bb51,
50740 => bb53,
_ => bb52
}
}
bb48 = {
(*_2) = &_9.2;
_15 = !_11.1;
_11.3.2 = Field::<i16>(Variant(_16, 0), 1);
(*_2) = &_3;
_9.0 = _9.3;
_14 = (*_1) as f32;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!(_17);
(*_2) = &_3;
(*_5) = _14 as u16;
_12 = [22188068744734957684566726956526787536_i128,57640193147987889623929279086842422667_i128,55433091392340959116325671756841774098_i128,(-67308382785040138069036407043527325963_i128),144520950471885414265479229006700727481_i128];
(*_2) = &_9.2;
(*_5) = 280533560_u32 as u16;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
match _9.1 {
0 => bb4,
1 => bb10,
2 => bb11,
172606100 => bb13,
_ => bb12
}
}
bb49 = {
(*_45).3.0 = !70_u8;
(*_45).3.0 = (*_45).1 as u8;
(*_50) = [_28,_28,_28,_28,_28];
(*_45).3.2 = _41.0;
(*_58) = (*_45).3.2 as f32;
(*_45).0 = [301483997_u32,2951727432_u32,2678672787_u32,1257464032_u32,2331491643_u32,386736307_u32];
_3 = _41.1.2 + _41.1.2;
_41.1.2 = _3;
place!(Field::<bool>(Variant(_16, 1), 0)) = (*_45).3.2 == (*_45).3.2;
(*_45).3.1 = core::ptr::addr_of!((*_45).3.2);
(*_5) = 50740_u16;
place!(Field::<isize>(Variant(_16, 1), 1)) = _30 * _7;
(*_45).2 = [_41.3,_22,_22,_22,_22,_22];
match (*_5) {
0 => bb19,
1 => bb14,
2 => bb34,
50740 => bb36,
_ => bb35
}
}
bb50 = {
(*_2) = &_9.2;
_15 = !_11.1;
_11.3.2 = Field::<i16>(Variant(_16, 0), 1);
(*_2) = &_3;
_9.0 = _9.3;
_14 = (*_1) as f32;
(*_2) = &_9.2;
_5 = core::ptr::addr_of_mut!(_17);
(*_2) = &_3;
(*_5) = _14 as u16;
_12 = [22188068744734957684566726956526787536_i128,57640193147987889623929279086842422667_i128,55433091392340959116325671756841774098_i128,(-67308382785040138069036407043527325963_i128),144520950471885414265479229006700727481_i128];
(*_2) = &_9.2;
(*_5) = 280533560_u32 as u16;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
match _9.1 {
0 => bb4,
1 => bb10,
2 => bb11,
172606100 => bb13,
_ => bb12
}
}
bb51 = {
_21 = _3 + _9.2;
_54 = !134231442272414288707392219795821718856_u128;
(*_23) = [(*_5)];
_39 = &(*_45);
(*_45).0 = [748871672_u32,831213927_u32,2341931974_u32,602739474_u32,2243772996_u32,1093833892_u32];
(*_50) = [_28,_28,_28,_28,_28];
_55 = !(*_45).1;
_5 = core::ptr::addr_of_mut!(_62.0);
(*_45).1 = (*_39).3.2 >= (*_39).3.2;
_12 = [_28,_28,_28,_28,_28];
_64 = -_22;
_48 = (*_23);
(*_5) = _17 >> (*_45).3.2;
(*_45).0 = [2448591661_u32,1224586023_u32,4175778972_u32,884304098_u32,2128041005_u32,1273845048_u32];
_55 = !(*_45).1;
_52 = -(*_58);
_4 = _13;
_68 = core::ptr::addr_of_mut!(_35);
(*_5) = _17 - _17;
_62.0 = _17 & _17;
_28 = 108450955231370839211162059540098271037_i128 - (-92731178550611634715901369772898292239_i128);
(*_58) = Field::<isize>(Variant(_16, 1), 1) as f32;
Goto(bb37)
}
bb52 = {
(*_2) = &_3;
_11.3.0 = 2068258664_u32 as u8;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_14 = 4_usize as f32;
(*_2) = &_3;
_13 = _4;
_9.2 = (-148204743542568657250970443544732421226_i128) as f64;
_11.3.2 = (-13757_i16) | (-20133_i16);
_9.0 = _9.3;
(*_2) = &_9.2;
(*_2) = &_3;
(*_2) = &_9.2;
_12 = [164927075340966359638500519006806914108_i128,167183701487675484072316355901897373341_i128,1464657015335594504046762109819090618_i128,(-2343879166991725753751770269438334418_i128),(-30283954077513423444217261103159492049_i128)];
(*_2) = &_3;
(*_2) = &_9.2;
_11.0 = [4248483773_u32,269471058_u32,289966372_u32,3581296097_u32,717178064_u32,484697185_u32];
Goto(bb8)
}
bb53 = {
_74 = (*_82) <= (*_82);
(*_5) = _83.0 | _83.0;
(*_18) = [_86.1,_86.1,_49,_86.1];
(*_45).1 = _67.1;
(*_23) = [(*_5)];
(*_50) = [_28,_28,_28,_28,_28];
_9.3 = _13;
(*_45).2 = [_64,_41.3,_22,_22,_41.3,_41.3];
_65 = core::ptr::addr_of_mut!((*_50));
(*_58) = -_52;
(*_5) = _17 / _83.0;
(*_2) = &_41.1.2;
_7 = (*_40);
_78 = _80 + (*_58);
_83.1 = core::ptr::addr_of_mut!((*_45).1);
Goto(bb54)
}
bb54 = {
Call(_97 = dump_var(Move(_56), Move(_17), Move(_4), Move(_74)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_97 = dump_var(Move(_47), Move(_25), Move(_13), Move(_12)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_97 = dump_var(Move(_69), Move(_64), Move(_71), Move(_24)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_97 = dump_var(Move(_73), Move(_61), Move(_28), Move(_53)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16() -> [u64; 4] {
mir! {
type RET = [u64; 4];
let _1: f32;
let _2: *const &'static f64;
let _3: bool;
let _4: [i8; 7];
let _5: *const f32;
let _6: *mut &'static Adt18;
let _7: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _8: *mut u16;
let _9: (Adt58,);
let _10: &'static u16;
let _11: char;
let _12: isize;
let _13: *mut *mut &'static Adt18;
let _14: *mut *const [i64; 6];
let _15: i32;
let _16: Adt41;
let _17: char;
let _18: *const i16;
let _19: char;
let _20: *mut bool;
let _21: [i32; 1];
let _22: isize;
let _23: (Adt58, Adt20);
let _24: u64;
let _25: *mut [u32; 7];
let _26: i64;
let _27: *mut [i128; 5];
let _28: Adt31;
let _29: *mut Adt18;
let _30: isize;
let _31: [i32; 1];
let _32: (u16, *mut bool);
let _33: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _34: f64;
let _35: *mut *const f32;
let _36: *const [i64; 6];
let _37: [u64; 3];
let _38: usize;
let _39: u128;
let _40: &'static mut u32;
let _41: &'static f64;
let _42: char;
let _43: char;
let _44: &'static (Adt58,);
let _45: *mut *const [i64; 6];
let _46: &'static f64;
let _47: (*mut [usize; 4], Adt46, [isize; 5], Adt41);
let _48: u32;
let _49: &'static i16;
let _50: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _51: [u32; 7];
let _52: [isize; 5];
let _53: bool;
let _54: &'static *mut &'static Adt18;
let _55: f64;
let _56: &'static mut &'static u128;
let _57: u8;
let _58: &'static i16;
let _59: *mut [usize; 4];
let _60: *mut bool;
let _61: u16;
let _62: usize;
let _63: u8;
let _64: &'static i8;
let _65: &'static mut usize;
let _66: ();
let _67: ();
{
RET = [8844271595285963227_u64,13304177521533907442_u64,1645889571420433956_u64,377544831764932926_u64];
RET = [8856139880374430755_u64,7595709255288832555_u64,15386007076230334390_u64,11327812805157793374_u64];
RET = [8743692362036531874_u64,5152138988302887898_u64,4207856233602056779_u64,15739002502739949365_u64];
RET = [9430438872209559768_u64,11103352812531915748_u64,5356536202443730087_u64,7477009989891608643_u64];
RET = [513977785734402151_u64,12798577843878485833_u64,16903326868417190991_u64,11616235635514301345_u64];
RET = [5203074131665750208_u64,17052033130724891144_u64,13720190023487241916_u64,2017885049910710777_u64];
_1 = 66411403_u32 as f32;
RET = [3279252771807188703_u64,552939997363754252_u64,1685488534919084681_u64,13302303481824482714_u64];
RET = [1814793437081526320_u64,14144964223770669961_u64,9079118312947088198_u64,5381366811802863002_u64];
_1 = 3358449302_u32 as f32;
_1 = 154486644190273328317990850747768021950_i128 as f32;
_4 = [(-1_i8),(-16_i8),123_i8,(-58_i8),9_i8,81_i8,49_i8];
_5 = core::ptr::addr_of!(_1);
_4 = [29_i8,57_i8,(-39_i8),(-79_i8),80_i8,(-28_i8),24_i8];
Call(_3 = fn17(Move(_5), (*_5), _1, (*_5), (*_5), _1, (*_5), (*_5), (*_5), (*_5), (*_5), (*_5)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb2 = {
(*_5) = 2066859711_u32 as f32;
_12 = 1538528462_u32 as isize;
_11 = '\u{105c2}';
(*_5) = _12 as f32;
_7.1 = _3;
_11 = '\u{74ec2}';
_7.2 = [8579760626358891893_i64,(-7163471359632771224_i64),729594779246027393_i64,(-397040332697876110_i64),942225495813202957_i64,646069908222605675_i64];
_3 = _7.1 <= _7.1;
_13 = core::ptr::addr_of_mut!(_6);
(*_5) = _12 as f32;
(*_5) = 64_i8 as f32;
_11 = '\u{ee0ae}';
(*_5) = (-772686239715935518_i64) as f32;
Goto(bb3)
}
bb3 = {
(*_5) = (-5152315031682754503_i64) as f32;
_17 = _11;
_17 = _11;
(*_5) = 17929079493009589993_usize as f32;
(*_5) = (-165734911245004537426734842348515696620_i128) as f32;
_18 = core::ptr::addr_of!(_7.3.2);
(*_5) = (-1972889206078361633_i64) as f32;
(*_5) = 68141320_u32 as f32;
Goto(bb4)
}
bb4 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb5 = {
_7.3.1 = core::ptr::addr_of!((*_18));
_23.0 = Adt58::Variant1 { fld0: Move(_7) };
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_21 = [_15];
(*_5) = (-63320277691663278224841910523789076281_i128) as f32;
(*_5) = 138003540224686702308007583732385381851_i128 as f32;
match Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
4692 => bb9,
_ => bb8
}
}
bb6 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb7 = {
(*_5) = (-5152315031682754503_i64) as f32;
_17 = _11;
_17 = _11;
(*_5) = 17929079493009589993_usize as f32;
(*_5) = (-165734911245004537426734842348515696620_i128) as f32;
_18 = core::ptr::addr_of!(_7.3.2);
(*_5) = (-1972889206078361633_i64) as f32;
(*_5) = 68141320_u32 as f32;
Goto(bb4)
}
bb8 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb9 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).3.0 = 59_u8 * 84_u8;
_15 = 665307062_i32;
_11 = _17;
_7.0 = [2585501492_u32,2898025162_u32,4036027535_u32,2621531198_u32,1600829170_u32,1482889451_u32];
_18 = core::ptr::addr_of!((*_18));
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!((*_18));
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).1 = _3;
_18 = core::ptr::addr_of!((*_18));
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 | Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).0 = _7.0;
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_5) = 58212_u16 as f32;
_7.2 = [4213886470369151455_i64,(-5681492021806470993_i64),3275500014910317938_i64,(-5009961109558559445_i64),(-1672942118721390322_i64),8616863141089931647_i64];
_23.1 = Adt20::Variant1 { fld0: _3,fld1: _22 };
_7.3.1 = core::ptr::addr_of!((*_18));
_9 = (Move(_23.0),);
(*_5) = 27650_u16 as f32;
(*_5) = 3297743184056871172_u64 as f32;
_13 = core::ptr::addr_of_mut!((*_13));
_7.3.2 = (-150881899136406339338782238614535166157_i128) as i16;
_7.0 = [1973861022_u32,1891704673_u32,1518038526_u32,3369249802_u32,1080086114_u32,2207153125_u32];
_4 = [97_i8,6_i8,114_i8,1_i8,4_i8,10_i8,(-121_i8)];
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2 ^ Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
Goto(bb10)
}
bb10 = {
_21 = [_15];
_23.0 = Move(_9.0);
_26 = (-3956939028057054182_i64) * 8591497981624291161_i64;
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_19 = _17;
_7.0 = [514108683_u32,315553382_u32,3556357416_u32,1308470194_u32,259354011_u32,291397178_u32];
_13 = core::ptr::addr_of_mut!((*_13));
(*_5) = 7246364279825367701_u64 as f32;
(*_5) = 7140_u16 as f32;
_21 = [_15];
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_5) = (*_18) as f32;
_7.1 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).1 >= Field::<bool>(Variant(_23.1, 1), 0);
match (*_18) {
4692 => bb12,
_ => bb11
}
}
bb11 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb12 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).3.2 = (*_18) >> (*_18);
(*_18) = _19 as i16;
match _15 {
0 => bb11,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
665307062 => bb18,
_ => bb17
}
}
bb13 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb14 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb15 = {
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).3.0 = 59_u8 * 84_u8;
_15 = 665307062_i32;
_11 = _17;
_7.0 = [2585501492_u32,2898025162_u32,4036027535_u32,2621531198_u32,1600829170_u32,1482889451_u32];
_18 = core::ptr::addr_of!((*_18));
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!((*_18));
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).1 = _3;
_18 = core::ptr::addr_of!((*_18));
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 | Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).0 = _7.0;
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_5) = 58212_u16 as f32;
_7.2 = [4213886470369151455_i64,(-5681492021806470993_i64),3275500014910317938_i64,(-5009961109558559445_i64),(-1672942118721390322_i64),8616863141089931647_i64];
_23.1 = Adt20::Variant1 { fld0: _3,fld1: _22 };
_7.3.1 = core::ptr::addr_of!((*_18));
_9 = (Move(_23.0),);
(*_5) = 27650_u16 as f32;
(*_5) = 3297743184056871172_u64 as f32;
_13 = core::ptr::addr_of_mut!((*_13));
_7.3.2 = (-150881899136406339338782238614535166157_i128) as i16;
_7.0 = [1973861022_u32,1891704673_u32,1518038526_u32,3369249802_u32,1080086114_u32,2207153125_u32];
_4 = [97_i8,6_i8,114_i8,1_i8,4_i8,10_i8,(-121_i8)];
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2 ^ Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
Goto(bb10)
}
bb16 = {
_7.3.1 = core::ptr::addr_of!((*_18));
_23.0 = Adt58::Variant1 { fld0: Move(_7) };
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_21 = [_15];
(*_5) = (-63320277691663278224841910523789076281_i128) as f32;
(*_5) = 138003540224686702308007583732385381851_i128 as f32;
match Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
4692 => bb9,
_ => bb8
}
}
bb17 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb18 = {
_21 = [_15];
(*_5) = 0_usize as f32;
_7.3 = (Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0, Move(_18), Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2);
_18 = core::ptr::addr_of!(_7.3.2);
_32.0 = 65188_u16 + 32812_u16;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_31 = _21;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 >> Field::<isize>(Variant(_23.1, 1), 1);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).3.1 = core::ptr::addr_of!((*_18));
_15 = (-1077178209_i32);
(*_5) = 2174737589_u32 as f32;
_7.0 = [2975024320_u32,3283946141_u32,4032525218_u32,634876303_u32,1332814611_u32,2132615180_u32];
(*_5) = 406677193_u32 as f32;
match _15 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
4 => bb11,
5 => bb6,
6 => bb13,
340282366920938463463374607430691033247 => bb19,
_ => bb15
}
}
bb19 = {
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_32.1 = core::ptr::addr_of_mut!(place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).1);
(*_5) = 2764859464_u32 as f32;
_4 = [14_i8,81_i8,26_i8,(-7_i8),69_i8,54_i8,35_i8];
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0)).3 = Move(_7.3);
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_36 = core::ptr::addr_of!(_7.2);
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_18) = -Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!((*_18));
(*_18) = _26 as i16;
match _15 {
0 => bb6,
340282366920938463463374607430691033247 => bb21,
_ => bb20
}
}
bb20 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb21 = {
_36 = core::ptr::addr_of!((*_36));
_9 = (Move(_23.0),);
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
(*_5) = 102_i8 as f32;
_3 = _7.1 >= Field::<bool>(Variant(_23.1, 1), 0);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0)).3.0 = !19_u8;
_14 = core::ptr::addr_of_mut!(_36);
_17 = _19;
(*_5) = _12 as f32;
(*_14) = core::ptr::addr_of!((*_36));
(*_14) = core::ptr::addr_of!((*_36));
(*_36) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).2;
match _15 {
0 => bb12,
1 => bb8,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
340282366920938463463374607430691033247 => bb27,
_ => bb26
}
}
bb22 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb23 = {
_21 = [_15];
_23.0 = Move(_9.0);
_26 = (-3956939028057054182_i64) * 8591497981624291161_i64;
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_19 = _17;
_7.0 = [514108683_u32,315553382_u32,3556357416_u32,1308470194_u32,259354011_u32,291397178_u32];
_13 = core::ptr::addr_of_mut!((*_13));
(*_5) = 7246364279825367701_u64 as f32;
(*_5) = 7140_u16 as f32;
_21 = [_15];
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_5) = (*_18) as f32;
_7.1 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).1 >= Field::<bool>(Variant(_23.1, 1), 0);
match (*_18) {
4692 => bb12,
_ => bb11
}
}
bb24 = {
(*_5) = 2066859711_u32 as f32;
_12 = 1538528462_u32 as isize;
_11 = '\u{105c2}';
(*_5) = _12 as f32;
_7.1 = _3;
_11 = '\u{74ec2}';
_7.2 = [8579760626358891893_i64,(-7163471359632771224_i64),729594779246027393_i64,(-397040332697876110_i64),942225495813202957_i64,646069908222605675_i64];
_3 = _7.1 <= _7.1;
_13 = core::ptr::addr_of_mut!(_6);
(*_5) = _12 as f32;
(*_5) = 64_i8 as f32;
_11 = '\u{ee0ae}';
(*_5) = (-772686239715935518_i64) as f32;
Goto(bb3)
}
bb25 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb26 = {
(*_5) = (-5152315031682754503_i64) as f32;
_17 = _11;
_17 = _11;
(*_5) = 17929079493009589993_usize as f32;
(*_5) = (-165734911245004537426734842348515696620_i128) as f32;
_18 = core::ptr::addr_of!(_7.3.2);
(*_5) = (-1972889206078361633_i64) as f32;
(*_5) = 68141320_u32 as f32;
Goto(bb4)
}
bb27 = {
_24 = 13279211829702647179_u64 - 18138165700667728005_u64;
_36 = core::ptr::addr_of!((*_36));
(*_36) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).2;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
_43 = _11;
(*_14) = core::ptr::addr_of!((*_36));
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_14) = core::ptr::addr_of!((*_36));
_26 = !(-3056612196874765973_i64);
(*_36) = [_26,_26,_26,_26,_26,_26];
_3 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).1;
match _15 {
0 => bb1,
1 => bb11,
2 => bb24,
340282366920938463463374607430691033247 => bb28,
_ => bb26
}
}
bb28 = {
(*_18) = 2941998811_u32 as i16;
(*_14) = core::ptr::addr_of!((*_36));
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_5) = 7_usize as f32;
(*_14) = core::ptr::addr_of!((*_36));
_45 = core::ptr::addr_of_mut!((*_14));
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_18) = 42_i8 as i16;
_7.3.0 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.0;
(*_36) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).2;
(*_5) = 179015237_u32 as f32;
(*_14) = core::ptr::addr_of!((*_36));
(*_5) = (*_18) as f32;
(*_18) = -Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
_7.3.0 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.0 | Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.0;
(*_14) = core::ptr::addr_of!((*_36));
_18 = Move(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.1);
_3 = _7.1;
_4 = [(-107_i8),(-44_i8),111_i8,76_i8,122_i8,(-93_i8),(-72_i8)];
_32.1 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_23.1, 1), 0)));
_9.0 = Adt58::Variant1 { fld0: Move(_7) };
(*_36) = [_26,_26,_26,_26,_26,_26];
_44 = &_9;
Goto(bb29)
}
bb29 = {
(*_5) = 94504508925439518282384231257221285432_i128 as f32;
match _15 {
0 => bb27,
1 => bb30,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
340282366920938463463374607430691033247 => bb36,
_ => bb35
}
}
bb30 = {
(*_5) = (-5152315031682754503_i64) as f32;
_17 = _11;
_17 = _11;
(*_5) = 17929079493009589993_usize as f32;
(*_5) = (-165734911245004537426734842348515696620_i128) as f32;
_18 = core::ptr::addr_of!(_7.3.2);
(*_5) = (-1972889206078361633_i64) as f32;
(*_5) = 68141320_u32 as f32;
Goto(bb4)
}
bb31 = {
_21 = [_15];
_23.0 = Move(_9.0);
_26 = (-3956939028057054182_i64) * 8591497981624291161_i64;
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2 & Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.2;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_19 = _17;
_7.0 = [514108683_u32,315553382_u32,3556357416_u32,1308470194_u32,259354011_u32,291397178_u32];
_13 = core::ptr::addr_of_mut!((*_13));
(*_5) = 7246364279825367701_u64 as f32;
(*_5) = 7140_u16 as f32;
_21 = [_15];
_7.3.0 = !Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).3.0;
(*_5) = (*_18) as f32;
_7.1 = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_23.0, 1), 0).1 >= Field::<bool>(Variant(_23.1, 1), 0);
match (*_18) {
4692 => bb12,
_ => bb11
}
}
bb32 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb33 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb34 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb35 = {
(*_5) = 2066859711_u32 as f32;
_12 = 1538528462_u32 as isize;
_11 = '\u{105c2}';
(*_5) = _12 as f32;
_7.1 = _3;
_11 = '\u{74ec2}';
_7.2 = [8579760626358891893_i64,(-7163471359632771224_i64),729594779246027393_i64,(-397040332697876110_i64),942225495813202957_i64,646069908222605675_i64];
_3 = _7.1 <= _7.1;
_13 = core::ptr::addr_of_mut!(_6);
(*_5) = _12 as f32;
(*_5) = 64_i8 as f32;
_11 = '\u{ee0ae}';
(*_5) = (-772686239715935518_i64) as f32;
Goto(bb3)
}
bb36 = {
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_14) = core::ptr::addr_of!((*_36));
_50 = &mut place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0));
_1 = (-132071433901976690794802953094709483481_i128) as f32;
(*_14) = core::ptr::addr_of!((*_50).2);
(*_5) = _24 as f32;
(*_14) = core::ptr::addr_of!((*_36));
_8 = core::ptr::addr_of_mut!(_32.0);
(*_5) = (-93_i8) as f32;
(*_5) = (*_50).3.0 as f32;
match _15 {
340282366920938463463374607430691033247 => bb37,
_ => bb15
}
}
bb37 = {
_38 = _15 as usize;
(*_8) = (*_50).3.0 as u16;
_55 = _26 as f64;
(*_14) = core::ptr::addr_of!((*_36));
_7.0 = [4060908587_u32,2973459399_u32,2949028075_u32,602853581_u32,3484306742_u32,120178747_u32];
_46 = &_55;
_20 = core::ptr::addr_of_mut!((*_50).1);
(*_14) = core::ptr::addr_of!((*_50).2);
(*_14) = core::ptr::addr_of!((*_36));
_8 = core::ptr::addr_of_mut!((*_8));
_18 = core::ptr::addr_of!(_7.3.2);
(*_5) = Field::<isize>(Variant(_23.1, 1), 1) as f32;
match _15 {
0 => bb12,
1 => bb24,
340282366920938463463374607430691033247 => bb38,
_ => bb34
}
}
bb38 = {
(*_5) = _15 as f32;
(*_5) = (*_50).3.2 as f32;
(*_5) = 80_i8 as f32;
_7.1 = (*_20);
_39 = 116538074971366275092918590978127400607_u128;
_31 = [_15];
match _15 {
0 => bb29,
1 => bb16,
2 => bb35,
3 => bb28,
4 => bb36,
5 => bb39,
6 => bb40,
340282366920938463463374607430691033247 => bb42,
_ => bb41
}
}
bb39 = {
_36 = core::ptr::addr_of!((*_36));
_9 = (Move(_23.0),);
(*_18) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).3.2;
(*_5) = 102_i8 as f32;
_3 = _7.1 >= Field::<bool>(Variant(_23.1, 1), 0);
place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0)).3.0 = !19_u8;
_14 = core::ptr::addr_of_mut!(_36);
_17 = _19;
(*_5) = _12 as f32;
(*_14) = core::ptr::addr_of!((*_36));
(*_14) = core::ptr::addr_of!((*_36));
(*_36) = Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0).2;
match _15 {
0 => bb12,
1 => bb8,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
340282366920938463463374607430691033247 => bb27,
_ => bb26
}
}
bb40 = {
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_14) = core::ptr::addr_of!((*_36));
_50 = &mut place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0));
_1 = (-132071433901976690794802953094709483481_i128) as f32;
(*_14) = core::ptr::addr_of!((*_50).2);
(*_5) = _24 as f32;
(*_14) = core::ptr::addr_of!((*_36));
_8 = core::ptr::addr_of_mut!(_32.0);
(*_5) = (-93_i8) as f32;
(*_5) = (*_50).3.0 as f32;
match _15 {
340282366920938463463374607430691033247 => bb37,
_ => bb15
}
}
bb41 = {
(*_5) = 2066859711_u32 as f32;
_12 = 1538528462_u32 as isize;
_11 = '\u{105c2}';
(*_5) = _12 as f32;
_7.1 = _3;
_11 = '\u{74ec2}';
_7.2 = [8579760626358891893_i64,(-7163471359632771224_i64),729594779246027393_i64,(-397040332697876110_i64),942225495813202957_i64,646069908222605675_i64];
_3 = _7.1 <= _7.1;
_13 = core::ptr::addr_of_mut!(_6);
(*_5) = _12 as f32;
(*_5) = 64_i8 as f32;
_11 = '\u{ee0ae}';
(*_5) = (-772686239715935518_i64) as f32;
Goto(bb3)
}
bb42 = {
_51 = [2126628572_u32,3426098703_u32,2446614138_u32,891104801_u32,827135805_u32,4210647265_u32,3922778624_u32];
_30 = !Field::<isize>(Variant(_23.1, 1), 1);
_10 = &(*_8);
(*_8) = 14686_u16;
(*_8) = _24 as u16;
(*_8) = 40376_u16 | 61742_u16;
_31 = [_15];
(*_18) = (*_50).3.2 + (*_50).3.2;
_7.1 = !(*_20);
_49 = &(*_50).3.2;
_2 = core::ptr::addr_of!(_46);
_8 = core::ptr::addr_of_mut!((*_8));
(*_14) = core::ptr::addr_of!((*_36));
(*_5) = _39 as f32;
(*_8) = 27839_u16;
_30 = Field::<isize>(Variant(_23.1, 1), 1) * _12;
_30 = _22;
_47.2 = [_12,_22,_12,Field::<isize>(Variant(_23.1, 1), 1),Field::<isize>(Variant(_23.1, 1), 1)];
(*_8) = 15145_u16;
_32.0 = 20255_u16;
_41 = Move((*_2));
(*_8) = 7660_u16 >> (*_49);
place!(Field::<bool>(Variant(_23.1, 1), 0)) = (*_50).3.2 == (*_50).3.2;
(*_14) = core::ptr::addr_of!((*_36));
(*_5) = (*_8) as f32;
_48 = 1533583994_u32 - 1912338445_u32;
(*_8) = (*_50).1 as u16;
match _15 {
0 => bb43,
1 => bb44,
340282366920938463463374607430691033247 => bb46,
_ => bb45
}
}
bb43 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb44 = {
_7.3.2 = (-6666_i16);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_7.3.2 = 5498796856110733002_i64 as i16;
_7.3.0 = 134_u8 - 80_u8;
_7.3.1 = core::ptr::addr_of!(_7.3.2);
_1 = (-43_isize) as f32;
_5 = core::ptr::addr_of!(_1);
(*_5) = 6950688160874735502_i64 as f32;
(*_5) = 1276885338_i32 as f32;
(*_5) = 48569_u16 as f32;
_7.2 = [(-5175142180260792241_i64),3626527318662536057_i64,8209192618672951558_i64,(-8481030551723160345_i64),(-591762702691936844_i64),(-4181034251558825383_i64)];
(*_5) = 84_isize as f32;
(*_5) = (-137193207596427859318014948571890394100_i128) as f32;
_7.0 = [690214784_u32,2730747857_u32,1689318137_u32,2766236886_u32,2649934110_u32,2915640671_u32];
(*_5) = 11739_u16 as f32;
(*_5) = 820065165561655846_i64 as f32;
(*_5) = 7666389946891926568_u64 as f32;
(*_5) = 9223372036854775807_isize as f32;
_7.3.0 = !5_u8;
_4 = [2_i8,(-63_i8),(-16_i8),69_i8,(-94_i8),84_i8,28_i8];
RET = [9442244780585207950_u64,8321274966454486762_u64,12606906561410318482_u64,17113433701943545578_u64];
(*_5) = 30293_u16 as f32;
_7.3.0 = 85410138187692335255504364571801028917_i128 as u8;
Goto(bb2)
}
bb45 = {
(*_18) = -(-29525_i16);
_7.3.1 = core::ptr::addr_of!((*_18));
_17 = _11;
(*_18) = 14939514391343842003_u64 as i16;
_3 = _7.1;
_18 = Move(_7.3.1);
_19 = _17;
_17 = _11;
_11 = _19;
_18 = core::ptr::addr_of!(_7.3.2);
_15 = (-617338919_i32) + 2116960583_i32;
(*_18) = !(-1897_i16);
(*_5) = 496277842_u32 as f32;
_15 = (-1357860555_i32) + 807607418_i32;
_17 = _11;
(*_5) = _12 as f32;
_4 = [(-39_i8),87_i8,(-56_i8),7_i8,(-61_i8),(-68_i8),(-78_i8)];
(*_18) = 11471_i16 + (-13796_i16);
(*_5) = (-9_i8) as f32;
(*_18) = 14027_i16 + (-15826_i16);
(*_18) = 14635_i16;
_1 = 409546195611977846_usize as f32;
_22 = _7.3.0 as isize;
(*_18) = 4692_i16;
_5 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb46 = {
(*_5) = _48 as f32;
_11 = _17;
_19 = _43;
(*_18) = (*_50).3.2 & (*_50).3.2;
_14 = core::ptr::addr_of_mut!(_36);
(*_2) = &_55;
_1 = 110479374807781580921543951972597603743_i128 as f32;
_41 = Move((*_2));
_4 = [24_i8,(-26_i8),(-13_i8),13_i8,(-121_i8),31_i8,(-96_i8)];
(*_8) = 11334_u16 << (*_18);
(*_14) = core::ptr::addr_of!((*_36));
_34 = (*_8) as f64;
_1 = _48 as f32;
(*_2) = &_34;
match _39 {
116538074971366275092918590978127400607 => bb47,
_ => bb19
}
}
bb47 = {
(*_18) = (*_50).3.0 as i16;
_37 = [_24,_24,_24];
(*_8) = !5744_u16;
_25 = core::ptr::addr_of_mut!(_51);
match _39 {
116538074971366275092918590978127400607 => bb49,
_ => bb48
}
}
bb48 = {
(*_36) = [_26,_26,_26,_26,_26,_26];
(*_14) = core::ptr::addr_of!((*_36));
_50 = &mut place!(Field::<([u32; 6], bool, [i64; 6], (u8, *const i16, i16))>(Variant(_9.0, 1), 0));
_1 = (-132071433901976690794802953094709483481_i128) as f32;
(*_14) = core::ptr::addr_of!((*_50).2);
(*_5) = _24 as f32;
(*_14) = core::ptr::addr_of!((*_36));
_8 = core::ptr::addr_of_mut!(_32.0);
(*_5) = (-93_i8) as f32;
(*_5) = (*_50).3.0 as f32;
match _15 {
340282366920938463463374607430691033247 => bb37,
_ => bb15
}
}
bb49 = {
(*_25) = [_48,_48,_48,_48,_48,_48,_48];
_7.3.0 = _38 as u8;
RET = [_24,_24,_24,_24];
_33 = &(*_50);
_4 = [108_i8,(-116_i8),(-54_i8),103_i8,16_i8,72_i8,100_i8];
(*_5) = _15 as f32;
(*_2) = &_55;
(*_8) = 12644_u16 + 850_u16;
_53 = (*_33).1 > (*_50).1;
_40 = &mut _48;
_38 = (*_46) as usize;
(*_18) = _39 as i16;
(*_2) = &_34;
_37 = [_24,_24,_24];
_55 = _24 as f64;
(*_40) = 3114080150_u32 - 4039706722_u32;
(*_2) = &_55;
(*_8) = !9473_u16;
_5 = core::ptr::addr_of!((*_5));
(*_14) = core::ptr::addr_of!(_7.2);
_32.0 = !30740_u16;
_7.3 = ((*_33).3.0, Move(_18), (*_33).3.2);
Goto(bb50)
}
bb50 = {
Call(_66 = dump_var(Move(_22), Move(_11), Move(_17), Move(_4)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_66 = dump_var(Move(_3), Move(_19), Move(_38), Move(_15)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_66 = dump_var(Move(_37), Move(_31), _67, _67), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *const f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: f32,mut _12: f32) -> bool {
mir! {
type RET = bool;
let _13: f64;
let _14: *mut [i128; 5];
let _15: isize;
let _16: &'static mut *const u32;
let _17: Adt51;
let _18: char;
let _19: u16;
let _20: i16;
let _21: (u16, *mut bool);
let _22: u16;
let _23: Adt20;
let _24: f64;
let _25: &'static mut &'static u128;
let _26: isize;
let _27: isize;
let _28: (*mut bool,);
let _29: isize;
let _30: u8;
let _31: Adt31;
let _32: char;
let _33: *mut u16;
let _34: ();
let _35: ();
{
RET = true ^ false;
_8 = 16705241712472237056_u64 as f32;
_10 = 7033436260064204222_u64 as f32;
_3 = 293018759154584778255620369331050294219_u128 as f32;
_8 = _12 - _9;
_4 = _6 * _6;
RET = _4 > _6;
_11 = -_9;
_10 = _6 - _3;
_13 = 474766222_u32 as f64;
_12 = -_4;
_12 = _4;
_12 = (-16016_i16) as f32;
_1 = core::ptr::addr_of!(_2);
(*_1) = _4 + _4;
_11 = (*_1) * (*_1);
(*_1) = _10 - _12;
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of!((*_1));
_5 = -(*_1);
RET = !true;
_4 = -(*_1);
(*_1) = _3 - _11;
(*_1) = _11;
_10 = (*_1) * (*_1);
(*_1) = _6 - _10;
Goto(bb2)
}
bb2 = {
(*_1) = (-993467342_i32) as f32;
(*_1) = -_10;
(*_1) = -_6;
_6 = _10 - (*_1);
(*_1) = _11 - _10;
_3 = (*_1);
(*_1) = _6 + _3;
(*_1) = _6 * _6;
_5 = 5_isize as f32;
_9 = (*_1) + _3;
(*_1) = _9;
(*_1) = _9 + _9;
_7 = (*_1);
(*_1) = _9 - _10;
(*_1) = -_10;
(*_1) = -_10;
(*_1) = _7;
_15 = 9223372036854775807_isize >> 5934636053549997783_u64;
(*_1) = _11 + _10;
(*_1) = _10 * _10;
Goto(bb3)
}
bb3 = {
(*_1) = _9;
(*_1) = _8 * _9;
_21.1 = core::ptr::addr_of_mut!(RET);
_4 = (-113_i8) as f32;
_18 = '\u{fc76d}';
(*_1) = (-115912417770170105604042688751931048650_i128) as f32;
_6 = _7 + (*_1);
_11 = _9 - _9;
_20 = (-31654_i16) >> _15;
_21.0 = 30788_u16;
(*_1) = _15 as f32;
_5 = _10 - _3;
_9 = _15 as f32;
(*_1) = 3_usize as f32;
(*_1) = _7 * _6;
_23 = Adt20::Variant3 { fld0: (*_1),fld1: 226_u8 };
(*_1) = 57599811782515240042992211613605168085_u128 as f32;
_13 = 23_i8 as f64;
Call((*_1) = fn18(_20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_19 = _21.0;
(*_1) = -Field::<f32>(Variant(_23, 3), 0);
_26 = _15 << _15;
(*_1) = 5_usize as f32;
_7 = _11;
_24 = _13;
(*_1) = -_3;
(*_1) = _7;
(*_1) = (-1804512736825796812_i64) as f32;
_24 = _20 as f64;
_28.0 = Move(_21.1);
_11 = _10;
_28.0 = core::ptr::addr_of_mut!(RET);
Goto(bb5)
}
bb5 = {
_2 = _3 + _6;
_24 = _13;
_2 = 65876752179032261478520150618126011744_i128 as f32;
place!(Field::<u8>(Variant(_23, 3), 1)) = (-1126178962_i32) as u8;
_4 = _7 * _7;
_22 = _19;
(*_1) = _26 as f32;
_21 = (_22, Move(_28.0));
(*_1) = Field::<f32>(Variant(_23, 3), 0) + _10;
_12 = (*_1) - (*_1);
_30 = Field::<u8>(Variant(_23, 3), 1) >> _26;
_12 = -_4;
(*_1) = _6 * _3;
(*_1) = -_6;
(*_1) = (-7497878599068153425_i64) as f32;
_13 = _24;
_12 = -_6;
_1 = core::ptr::addr_of!((*_1));
_6 = _4;
_17 = Adt51::Variant1 { fld0: Move(_23),fld1: (-1679398691_i32) };
_7 = _26 as f32;
_3 = _12;
_13 = -_24;
(*_1) = _13 as f32;
place!(Field::<i32>(Variant(_17, 1), 1)) = 1047275913_i32 - (-434596525_i32);
_23 = Move(Field::<Adt20>(Variant(_17, 1), 0));
_27 = !_26;
match _21.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
30788 => bb12,
_ => bb11
}
}
bb6 = {
_19 = _21.0;
(*_1) = -Field::<f32>(Variant(_23, 3), 0);
_26 = _15 << _15;
(*_1) = 5_usize as f32;
_7 = _11;
_24 = _13;
(*_1) = -_3;
(*_1) = _7;
(*_1) = (-1804512736825796812_i64) as f32;
_24 = _20 as f64;
_28.0 = Move(_21.1);
_11 = _10;
_28.0 = core::ptr::addr_of_mut!(RET);
Goto(bb5)
}
bb7 = {
(*_1) = _9;
(*_1) = _8 * _9;
_21.1 = core::ptr::addr_of_mut!(RET);
_4 = (-113_i8) as f32;
_18 = '\u{fc76d}';
(*_1) = (-115912417770170105604042688751931048650_i128) as f32;
_6 = _7 + (*_1);
_11 = _9 - _9;
_20 = (-31654_i16) >> _15;
_21.0 = 30788_u16;
(*_1) = _15 as f32;
_5 = _10 - _3;
_9 = _15 as f32;
(*_1) = 3_usize as f32;
(*_1) = _7 * _6;
_23 = Adt20::Variant3 { fld0: (*_1),fld1: 226_u8 };
(*_1) = 57599811782515240042992211613605168085_u128 as f32;
_13 = 23_i8 as f64;
Call((*_1) = fn18(_20), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
(*_1) = (-993467342_i32) as f32;
(*_1) = -_10;
(*_1) = -_6;
_6 = _10 - (*_1);
(*_1) = _11 - _10;
_3 = (*_1);
(*_1) = _6 + _3;
(*_1) = _6 * _6;
_5 = 5_isize as f32;
_9 = (*_1) + _3;
(*_1) = _9;
(*_1) = _9 + _9;
_7 = (*_1);
(*_1) = _9 - _10;
(*_1) = -_10;
(*_1) = -_10;
(*_1) = _7;
_15 = 9223372036854775807_isize >> 5934636053549997783_u64;
(*_1) = _11 + _10;
(*_1) = _10 * _10;
Goto(bb3)
}
bb9 = {
_1 = core::ptr::addr_of!((*_1));
_5 = -(*_1);
RET = !true;
_4 = -(*_1);
(*_1) = _3 - _11;
(*_1) = _11;
_10 = (*_1) * (*_1);
(*_1) = _6 - _10;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<Adt20>(Variant(_17, 1), 0)) = Move(_23);
_5 = 3204216453972816040_u64 as f32;
place!(Field::<Adt20>(Variant(_17, 1), 0)) = Adt20::Variant0 { fld0: 0_usize,fld1: _20 };
place!(Field::<usize>(Variant(place!(Field::<Adt20>(Variant(_17, 1), 0)), 0), 0)) = 7_usize >> _26;
(*_1) = _4 - _3;
place!(Field::<usize>(Variant(place!(Field::<Adt20>(Variant(_17, 1), 0)), 0), 0)) = !6_usize;
match _21.0 {
0 => bb4,
1 => bb13,
30788 => bb15,
_ => bb14
}
}
bb13 = {
_19 = _21.0;
(*_1) = -Field::<f32>(Variant(_23, 3), 0);
_26 = _15 << _15;
(*_1) = 5_usize as f32;
_7 = _11;
_24 = _13;
(*_1) = -_3;
(*_1) = _7;
(*_1) = (-1804512736825796812_i64) as f32;
_24 = _20 as f64;
_28.0 = Move(_21.1);
_11 = _10;
_28.0 = core::ptr::addr_of_mut!(RET);
Goto(bb5)
}
bb14 = {
_19 = _21.0;
(*_1) = -Field::<f32>(Variant(_23, 3), 0);
_26 = _15 << _15;
(*_1) = 5_usize as f32;
_7 = _11;
_24 = _13;
(*_1) = -_3;
(*_1) = _7;
(*_1) = (-1804512736825796812_i64) as f32;
_24 = _20 as f64;
_28.0 = Move(_21.1);
_11 = _10;
_28.0 = core::ptr::addr_of_mut!(RET);
Goto(bb5)
}
bb15 = {
_28 = (Move(_21.1),);
(*_1) = _6 + _3;
(*_1) = _3;
_12 = _21.0 as f32;
_1 = core::ptr::addr_of!(_3);
RET = (*_1) <= (*_1);
_21.1 = core::ptr::addr_of_mut!(RET);
_23 = Move(Field::<Adt20>(Variant(_17, 1), 0));
(*_1) = _13 as f32;
_5 = Field::<i16>(Variant(_23, 0), 1) as f32;
_29 = _26 & _27;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(Move(_19), Move(_22), Move(_26), Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: i16) -> f32 {
mir! {
type RET = f32;
let _2: *const [u16; 1];
let _3: u32;
let _4: [u64; 3];
let _5: bool;
let _6: [i128; 5];
let _7: *const [u16; 1];
let _8: char;
let _9: &'static isize;
let _10: Adt41;
let _11: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _12: [i32; 3];
let _13: f64;
let _14: [bool; 6];
let _15: [i64; 6];
let _16: &'static mut usize;
let _17: isize;
let _18: &'static Adt18;
let _19: isize;
let _20: i32;
let _21: *mut [i128; 5];
let _22: i16;
let _23: char;
let _24: [bool; 6];
let _25: u8;
let _26: isize;
let _27: u8;
let _28: (*const &'static f64, char);
let _29: f32;
let _30: u128;
let _31: usize;
let _32: *mut *mut &'static Adt18;
let _33: Adt78;
let _34: i16;
let _35: isize;
let _36: isize;
let _37: bool;
let _38: bool;
let _39: f32;
let _40: u8;
let _41: i128;
let _42: isize;
let _43: u32;
let _44: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _45: f32;
let _46: u8;
let _47: *const f32;
let _48: *const f32;
let _49: ((*mut bool,), usize);
let _50: ((*mut bool,), usize);
let _51: (*mut bool,);
let _52: isize;
let _53: [i32; 3];
let _54: usize;
let _55: Adt20;
let _56: &'static mut usize;
let _57: i128;
let _58: &'static (Adt58,);
let _59: &'static mut *const u32;
let _60: u16;
let _61: *mut *const f32;
let _62: *mut *mut &'static Adt18;
let _63: *mut bool;
let _64: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _65: *const [u16; 1];
let _66: [isize; 5];
let _67: ();
let _68: ();
{
RET = 8311480656222240157563001204747649354_i128 as f32;
RET = 97071776472696254129948122599025246934_u128 as f32;
_1 = (-13229_i16);
RET = 2984914875836384912_u64 as f32;
RET = 2005005307_i32 as f32;
_1 = 1414_i16 >> 1105394750_i32;
Goto(bb1)
}
bb1 = {
RET = 44521757304731106989091432895640620066_u128 as f32;
_3 = 4184754136_u32;
_1 = 20674_i16 * (-22426_i16);
_1 = 3326412219660020750_u64 as i16;
_3 = 4271854317_u32 * 1153118180_u32;
_1 = 9146859946180496141_u64 as i16;
_1 = (-22121_i16) - (-8276_i16);
_4 = [6107615256064928642_u64,16464806623583814583_u64,17304994560289787397_u64];
_5 = false & false;
_4 = [8444134949441971024_u64,12526719258111147364_u64,16653316868486847508_u64];
RET = 15890720972142846851_usize as f32;
_8 = '\u{d578d}';
_4 = [2141157745117238962_u64,14304519638319300298_u64,3676151583137922533_u64];
_3 = 223875501_u32 ^ 542226449_u32;
_1 = 15402_i16 | 10970_i16;
Goto(bb2)
}
bb2 = {
_1 = !(-29745_i16);
_4 = [12657512304811101954_u64,12390914940438305006_u64,6039752619626889675_u64];
_1 = -(-9964_i16);
_5 = _3 >= _3;
_6 = [30119266875426280000174109733659612559_i128,(-107827329662342029349004948104268292222_i128),(-38385071613911895243686157139747955006_i128),64090182990726796380418493998313693483_i128,126446049660816718227538086475029112951_i128];
RET = 29609_u16 as f32;
RET = _1 as f32;
_11.1 = _5 & _5;
_1 = -21250_i16;
_4 = [9824973643718610899_u64,7192175852943072083_u64,5524738045942732532_u64];
_11.2 = [7406951099069815087_i64,7896485564890356712_i64,(-8561687100553298414_i64),5514201939842338278_i64,800033489974712146_i64,(-4454612060337635355_i64)];
_11.3.2 = _8 as i16;
_12 = [(-1825380283_i32),1369358508_i32,(-1663497281_i32)];
_11.0 = [_3,_3,_3,_3,_3,_3];
_3 = !1621125438_u32;
_3 = 227_u8 as u32;
_11.3.0 = 202_u8;
_5 = !_11.1;
Goto(bb3)
}
bb3 = {
RET = 316662412045652863845507651195845249550_u128 as f32;
_8 = '\u{ef8ef}';
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_14 = [_5,_11.1,_11.1,_11.1,_11.1,_11.1];
_12 = [(-39724211_i32),1068128733_i32,(-1537937562_i32)];
_6 = [(-63175715070761692929871270286806274257_i128),(-66754192044525849352376670520294250488_i128),(-104458394897690368050471867006912323229_i128),60498275687082684902265522216321386442_i128,147635816022705794112721394984159581945_i128];
_4 = [18009318166543921096_u64,6145241164720534742_u64,10306533242065938610_u64];
_14 = [_5,_11.1,_5,_5,_5,_11.1];
_4 = [17525438450294123055_u64,18444560672721614280_u64,12193983937464010947_u64];
_15 = [(-2250132818171338881_i64),2784164811519368944_i64,(-5606848405208848205_i64),456321762634459376_i64,7850441712562259419_i64,(-7989337537931818764_i64)];
_13 = _11.3.0 as f64;
_8 = '\u{9544e}';
_15 = _11.2;
_11.3.2 = _1 >> _11.3.0;
_4 = [13641621826399030651_u64,15776508648411326450_u64,11269875727715640903_u64];
RET = 136377224240810558295062092820810411157_u128 as f32;
_8 = '\u{9e64b}';
_14 = [_5,_11.1,_11.1,_5,_11.1,_11.1];
_13 = _11.3.0 as f64;
_1 = !_11.3.2;
match _11.3.0 {
0 => bb4,
202 => bb6,
_ => bb5
}
}
bb4 = {
_1 = !(-29745_i16);
_4 = [12657512304811101954_u64,12390914940438305006_u64,6039752619626889675_u64];
_1 = -(-9964_i16);
_5 = _3 >= _3;
_6 = [30119266875426280000174109733659612559_i128,(-107827329662342029349004948104268292222_i128),(-38385071613911895243686157139747955006_i128),64090182990726796380418493998313693483_i128,126446049660816718227538086475029112951_i128];
RET = 29609_u16 as f32;
RET = _1 as f32;
_11.1 = _5 & _5;
_1 = -21250_i16;
_4 = [9824973643718610899_u64,7192175852943072083_u64,5524738045942732532_u64];
_11.2 = [7406951099069815087_i64,7896485564890356712_i64,(-8561687100553298414_i64),5514201939842338278_i64,800033489974712146_i64,(-4454612060337635355_i64)];
_11.3.2 = _8 as i16;
_12 = [(-1825380283_i32),1369358508_i32,(-1663497281_i32)];
_11.0 = [_3,_3,_3,_3,_3,_3];
_3 = !1621125438_u32;
_3 = 227_u8 as u32;
_11.3.0 = 202_u8;
_5 = !_11.1;
Goto(bb3)
}
bb5 = {
RET = 44521757304731106989091432895640620066_u128 as f32;
_3 = 4184754136_u32;
_1 = 20674_i16 * (-22426_i16);
_1 = 3326412219660020750_u64 as i16;
_3 = 4271854317_u32 * 1153118180_u32;
_1 = 9146859946180496141_u64 as i16;
_1 = (-22121_i16) - (-8276_i16);
_4 = [6107615256064928642_u64,16464806623583814583_u64,17304994560289787397_u64];
_5 = false & false;
_4 = [8444134949441971024_u64,12526719258111147364_u64,16653316868486847508_u64];
RET = 15890720972142846851_usize as f32;
_8 = '\u{d578d}';
_4 = [2141157745117238962_u64,14304519638319300298_u64,3676151583137922533_u64];
_3 = 223875501_u32 ^ 542226449_u32;
_1 = 15402_i16 | 10970_i16;
Goto(bb2)
}
bb6 = {
_19 = (-9223372036854775808_isize);
_1 = _19 as i16;
_4 = [2825256847508055575_u64,2325914006939548405_u64,9238043765855073364_u64];
_19 = !(-9223372036854775808_isize);
_6 = [(-51015861408200685708705392866679194103_i128),50831280768760105080449576977722720050_i128,(-154441902678992902257806396139249332043_i128),(-129121018706594248056479830466594632702_i128),114750047343306573095588755635227989850_i128];
_20 = (-213115674_i32);
_11.1 = _5 ^ _5;
_11.1 = RET >= RET;
_14 = [_5,_5,_5,_5,_11.1,_5];
_20 = 742349510_i32;
_3 = 559099709_u32 - 1617663338_u32;
Goto(bb7)
}
bb7 = {
_11.3.0 = _8 as u8;
match _20 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
742349510 => bb11,
_ => bb10
}
}
bb8 = {
_19 = (-9223372036854775808_isize);
_1 = _19 as i16;
_4 = [2825256847508055575_u64,2325914006939548405_u64,9238043765855073364_u64];
_19 = !(-9223372036854775808_isize);
_6 = [(-51015861408200685708705392866679194103_i128),50831280768760105080449576977722720050_i128,(-154441902678992902257806396139249332043_i128),(-129121018706594248056479830466594632702_i128),114750047343306573095588755635227989850_i128];
_20 = (-213115674_i32);
_11.1 = _5 ^ _5;
_11.1 = RET >= RET;
_14 = [_5,_5,_5,_5,_11.1,_5];
_20 = 742349510_i32;
_3 = 559099709_u32 - 1617663338_u32;
Goto(bb7)
}
bb9 = {
RET = 44521757304731106989091432895640620066_u128 as f32;
_3 = 4184754136_u32;
_1 = 20674_i16 * (-22426_i16);
_1 = 3326412219660020750_u64 as i16;
_3 = 4271854317_u32 * 1153118180_u32;
_1 = 9146859946180496141_u64 as i16;
_1 = (-22121_i16) - (-8276_i16);
_4 = [6107615256064928642_u64,16464806623583814583_u64,17304994560289787397_u64];
_5 = false & false;
_4 = [8444134949441971024_u64,12526719258111147364_u64,16653316868486847508_u64];
RET = 15890720972142846851_usize as f32;
_8 = '\u{d578d}';
_4 = [2141157745117238962_u64,14304519638319300298_u64,3676151583137922533_u64];
_3 = 223875501_u32 ^ 542226449_u32;
_1 = 15402_i16 | 10970_i16;
Goto(bb2)
}
bb10 = {
_1 = !(-29745_i16);
_4 = [12657512304811101954_u64,12390914940438305006_u64,6039752619626889675_u64];
_1 = -(-9964_i16);
_5 = _3 >= _3;
_6 = [30119266875426280000174109733659612559_i128,(-107827329662342029349004948104268292222_i128),(-38385071613911895243686157139747955006_i128),64090182990726796380418493998313693483_i128,126446049660816718227538086475029112951_i128];
RET = 29609_u16 as f32;
RET = _1 as f32;
_11.1 = _5 & _5;
_1 = -21250_i16;
_4 = [9824973643718610899_u64,7192175852943072083_u64,5524738045942732532_u64];
_11.2 = [7406951099069815087_i64,7896485564890356712_i64,(-8561687100553298414_i64),5514201939842338278_i64,800033489974712146_i64,(-4454612060337635355_i64)];
_11.3.2 = _8 as i16;
_12 = [(-1825380283_i32),1369358508_i32,(-1663497281_i32)];
_11.0 = [_3,_3,_3,_3,_3,_3];
_3 = !1621125438_u32;
_3 = 227_u8 as u32;
_11.3.0 = 202_u8;
_5 = !_11.1;
Goto(bb3)
}
bb11 = {
_17 = _19;
_8 = '\u{746d}';
_1 = _11.3.2;
_6 = [23752809816790067280805600696579026303_i128,160159094215438250282478736563290030468_i128,(-111066778185992659425480883058048693551_i128),(-75856410375593878091809072992638830137_i128),(-137298149485052375211636158052245845110_i128)];
_11.3.2 = _1 * _1;
_12 = [_20,_20,_20];
_9 = &_17;
_11.3.0 = !247_u8;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_19 = (*_9);
_12 = [_20,_20,_20];
Call(_17 = core::intrinsics::bswap(_19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_23 = _8;
match _20 {
0 => bb10,
1 => bb2,
2 => bb13,
3 => bb14,
742349510 => bb16,
_ => bb15
}
}
bb13 = {
_17 = _19;
_8 = '\u{746d}';
_1 = _11.3.2;
_6 = [23752809816790067280805600696579026303_i128,160159094215438250282478736563290030468_i128,(-111066778185992659425480883058048693551_i128),(-75856410375593878091809072992638830137_i128),(-137298149485052375211636158052245845110_i128)];
_11.3.2 = _1 * _1;
_12 = [_20,_20,_20];
_9 = &_17;
_11.3.0 = !247_u8;
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_19 = (*_9);
_12 = [_20,_20,_20];
Call(_17 = core::intrinsics::bswap(_19), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_19 = (-9223372036854775808_isize);
_1 = _19 as i16;
_4 = [2825256847508055575_u64,2325914006939548405_u64,9238043765855073364_u64];
_19 = !(-9223372036854775808_isize);
_6 = [(-51015861408200685708705392866679194103_i128),50831280768760105080449576977722720050_i128,(-154441902678992902257806396139249332043_i128),(-129121018706594248056479830466594632702_i128),114750047343306573095588755635227989850_i128];
_20 = (-213115674_i32);
_11.1 = _5 ^ _5;
_11.1 = RET >= RET;
_14 = [_5,_5,_5,_5,_11.1,_5];
_20 = 742349510_i32;
_3 = 559099709_u32 - 1617663338_u32;
Goto(bb7)
}
bb15 = {
RET = 44521757304731106989091432895640620066_u128 as f32;
_3 = 4184754136_u32;
_1 = 20674_i16 * (-22426_i16);
_1 = 3326412219660020750_u64 as i16;
_3 = 4271854317_u32 * 1153118180_u32;
_1 = 9146859946180496141_u64 as i16;
_1 = (-22121_i16) - (-8276_i16);
_4 = [6107615256064928642_u64,16464806623583814583_u64,17304994560289787397_u64];
_5 = false & false;
_4 = [8444134949441971024_u64,12526719258111147364_u64,16653316868486847508_u64];
RET = 15890720972142846851_usize as f32;
_8 = '\u{d578d}';
_4 = [2141157745117238962_u64,14304519638319300298_u64,3676151583137922533_u64];
_3 = 223875501_u32 ^ 542226449_u32;
_1 = 15402_i16 | 10970_i16;
Goto(bb2)
}
bb16 = {
_14 = [_5,_5,_5,_5,_5,_5];
_11.0 = [_3,_3,_3,_3,_3,_3];
_11.1 = !_5;
_24 = _14;
_14 = [_11.1,_5,_5,_5,_5,_5];
_12 = [_20,_20,_20];
_22 = 90456715502440686084138696285323461895_u128 as i16;
_3 = _22 as u32;
_20 = (-1182771702_i32);
_5 = _3 <= _3;
RET = _3 as f32;
_24 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_25 = _20 as u8;
_11.0 = [_3,_3,_3,_3,_3,_3];
_11.3.1 = core::ptr::addr_of!(_1);
_11.0 = [_3,_3,_3,_3,_3,_3];
Goto(bb17)
}
bb17 = {
_15 = [(-2207812052031550368_i64),(-5249416709454657371_i64),1636884149113064686_i64,(-665573466669404179_i64),(-6703780730219117421_i64),1871609928274686659_i64];
_11.0 = [_3,_3,_3,_3,_3,_3];
RET = 1708_u16 as f32;
_14 = [_11.1,_11.1,_11.1,_5,_11.1,_5];
_11.3.0 = _22 as u8;
_19 = _17 ^ _17;
_25 = !_11.3.0;
_19 = -_17;
_3 = 54084_u16 as u32;
_20 = !1296435086_i32;
_11.3.0 = _25 | _25;
_23 = _8;
_4 = [17477898513568552328_u64,3364127396215421978_u64,9728838948665894220_u64];
_5 = _11.1;
_6 = [(-168077565222663137064555623880077339175_i128),26485175259015660886152855102522024341_i128,(-29049435860705849990180556375145618016_i128),(-114315191074629987808973152487810048382_i128),(-19229676993294900593008554887769551407_i128)];
_25 = _11.3.0 + _11.3.0;
_30 = _20 as u128;
_29 = RET + RET;
_19 = _17 | _17;
_5 = _11.1;
_11.0 = [_3,_3,_3,_3,_3,_3];
Goto(bb18)
}
bb18 = {
_11.3.1 = core::ptr::addr_of!(_11.3.2);
_27 = _25 & _25;
_14 = [_5,_5,_11.1,_11.1,_5,_5];
_28.1 = _23;
_1 = _11.3.2 + _11.3.2;
_34 = _19 as i16;
_4 = [14927280729674381101_u64,3600664607461683675_u64,17366097061999545096_u64];
Call(_26 = core::intrinsics::bswap(_17), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_26 = _19 ^ _19;
_21 = core::ptr::addr_of_mut!(_6);
_11.3.0 = _30 as u8;
_13 = 96_i8 as f64;
_14 = [_11.1,_5,_11.1,_5,_11.1,_11.1];
_26 = (-226656739850462224_i64) as isize;
_14 = [_11.1,_5,_11.1,_5,_11.1,_11.1];
(*_21) = [(-81592860727567739237509310032046029292_i128),11880544333162837829028440055464351585_i128,133585826143387227336865612235941243372_i128,103839541479034263984179935664541871648_i128,(-35124206976429041684203453865272392908_i128)];
_35 = _27 as isize;
_11.3.0 = _27;
_35 = _19 << _11.3.0;
(*_21) = [119392070088107328691271276195934069336_i128,(-14775836951691353481770785629758205523_i128),(-158779391934560604122442573072247416537_i128),(-148997286135721801374377109722112235352_i128),(-45629345484700558975194356230727158436_i128)];
(*_21) = [(-27110400197403360266512426960143720869_i128),(-111245548046472480339586609160122470013_i128),79655493348048581331027185281000722352_i128,74773208563886088943049282043293326514_i128,169581861281298239379601366695525726761_i128];
_11.0 = [_3,_3,_3,_3,_3,_3];
_28.1 = _8;
_11.3.0 = _25 ^ _27;
_35 = _19 | _19;
(*_21) = [4071158175322206036655787447390072590_i128,109859797185937205798112227813843251172_i128,(-77414774727420575587127154634507088906_i128),117045139542475220935387210860976600997_i128,60070022668585317384949122569886568798_i128];
Goto(bb20)
}
bb20 = {
_26 = _35 & _35;
(*_21) = [62263929877034041118416579988852742521_i128,(-88648947787690895590156137249770454059_i128),(-112838574061748042011663079905461641929_i128),(-127228727787935292141536363873451860093_i128),50258986252939687639507462326051473345_i128];
(*_21) = [85291209668460890774183901178817061289_i128,53455610898240353677796761800978974248_i128,166968739016971626662455389277656740463_i128,58946447331410442222672942924822168775_i128,(-119509355284479121682545270630035756091_i128)];
_21 = core::ptr::addr_of_mut!((*_21));
(*_21) = [(-62740522366212122808271812368011844445_i128),118371252729937407746569793049602216696_i128,(-122846862137837070622280923690551216498_i128),162508857124407499868334748811540980457_i128,90247160812012536513617774903502881769_i128];
_8 = _23;
_28.1 = _23;
_34 = _11.3.2;
(*_21) = [72194813918929624235608865442164226576_i128,(-83837206074171250501608563750920775458_i128),78905531286552730274550588692608301117_i128,61519302373483401906745127268320542073_i128,(-126422224092681860288451094279901538376_i128)];
_6 = [(-166468057781133495656788043565550445265_i128),78092812414950423782315981189224451730_i128,(-3106282234774320275730906941049206149_i128),43606548558384239613760363992654236473_i128,(-128505379223606412156306430650609561158_i128)];
_27 = _28.1 as u8;
_27 = _11.3.0 * _25;
(*_21) = [48355518543035597064059842093933331875_i128,(-108725980744895126729166466717975026052_i128),(-135826049987444652987512345984286510219_i128),119284014385815975886402034918770251324_i128,39557428464111093259715457102209971957_i128];
(*_21) = [101236690293829911606097456908507390355_i128,(-119699511285128874103376664992624924940_i128),41356808099791528994667666999980272456_i128,39070576426568148137056597002695276203_i128,(-51251124025888179080041798230355608130_i128)];
Goto(bb21)
}
bb21 = {
_11.0 = [_3,_3,_3,_3,_3,_3];
_37 = _11.1 | _11.1;
_11.3.1 = core::ptr::addr_of!(_22);
_28.1 = _8;
_4 = [253237836673778607_u64,15563158038282193844_u64,3227019735347622943_u64];
_19 = _17 + _26;
Goto(bb22)
}
bb22 = {
_30 = 3181315574192266769_u64 as u128;
(*_21) = [(-116772743894249215446745147089834112503_i128),(-165265394309904896581327793721749700506_i128),(-141112721830257030793135008014447726951_i128),(-50154355083428868177672039425065664421_i128),129703904580431093300602269939034728551_i128];
_31 = !2_usize;
_9 = &_19;
_8 = _23;
_38 = _37 | _37;
_20 = 1451495335_i32;
(*_21) = [(-65546444735261232551107937814333610632_i128),151302101744346012434766480718047592056_i128,(-11339680387452436706553835135330255419_i128),53729771139088419083779417306285679890_i128,(-21427780828213408902685263180965830356_i128)];
_4 = [18197776969592215482_u64,9759014013858846016_u64,3039190238254394988_u64];
_11.3.1 = core::ptr::addr_of!(_1);
(*_21) = [10741359591961558257440512815546186347_i128,122570325162965166000372773529090348416_i128,52893778751335042970653019457685632735_i128,159097069331480941676495937803283396981_i128,(-30610108680997725437352061336784048199_i128)];
(*_21) = [102567385184645858131884904220991371505_i128,15297485451126441975467311826918979201_i128,8252144940712898361281604519852816719_i128,(-134086207577083721098564263192879799346_i128),58303152298787073477752138981926706188_i128];
(*_21) = [161168617994496147848024917635185454360_i128,98087206750838084757992703723119336973_i128,51941643446575368303205450723086981589_i128,50226350816138276285029216665734530622_i128,12458790467410302471588106080921051924_i128];
_44.3.0 = _11.3.0 ^ _27;
_38 = _11.1;
_42 = (*_9) << _27;
_39 = _29 + _29;
_40 = _30 as u8;
match _20 {
0 => bb4,
1451495335 => bb23,
_ => bb2
}
}
bb23 = {
_34 = _1;
(*_21) = [19651173390774317986364170428321042610_i128,20377148825340791643631425966513078092_i128,(-41777817775039069876807415471192799463_i128),161887424585453035304491094603951157364_i128,(-116782282844647916107837342961495482388_i128)];
_15 = [(-3513324080911037242_i64),(-8747489383512289872_i64),(-4149305228143167900_i64),(-4505460872202293747_i64),2803935753527516332_i64,394257542093036697_i64];
(*_21) = [(-116809557626864741295832661845035754532_i128),(-75564187716754973406556590862076401781_i128),(-134681748530437347319494139269685753396_i128),(-24911822434088834695183039263379598727_i128),1496233277542354327483033918561304095_i128];
_44.0 = [_3,_3,_3,_3,_3,_3];
_5 = _37;
(*_21) = [92082267756987192537934612820357799576_i128,5482002772489378419450585214624549599_i128,(-106060130555206960610138459830001081177_i128),(-63598029779679383693285064011483163919_i128),(-118580509242224310442929774427355165394_i128)];
_5 = _37;
_9 = &_42;
match _20 {
0 => bb10,
1 => bb20,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb15,
1451495335 => bb24,
_ => bb16
}
}
bb24 = {
_16 = &mut _31;
_11.2 = _15;
_13 = 7188320214362375497_i64 as f64;
_47 = core::ptr::addr_of!(_45);
(*_47) = -_29;
_27 = _44.3.0 << (*_9);
_13 = (-40769472464752315985855540292429292232_i128) as f64;
_44 = Move(_11);
_44.3.0 = !_27;
_19 = (*_9);
(*_16) = 1_usize + 8993902936931979255_usize;
_11.3.2 = !_44.3.2;
(*_16) = 4_usize + 6_usize;
(*_47) = _39 + RET;
(*_21) = [(-103798613934619120795267093455655821600_i128),111516764406707775863999431796119470427_i128,(-30874238793301688973159453523640168691_i128),8043368840372022766695077511733105858_i128,(-108443182771624602274837699581529433498_i128)];
(*_21) = [89756684733579382426557425161146545542_i128,(-74736727760385666039647747104158696891_i128),119258722694936780058744223510947168125_i128,122441716656581846307056731809014751538_i128,(-39737454885704104622844722218761330724_i128)];
(*_21) = [53193593980281512989637026985083056808_i128,118354812779940874069404073961182672454_i128,142435624884789585830230246152444209091_i128,(-64895722346762571518462241715560416002_i128),(-116521567934930084398811986280297554575_i128)];
_11.3.1 = Move(_44.3.1);
_43 = _3;
_51.0 = core::ptr::addr_of_mut!(_44.1);
_22 = _34 * _1;
_20 = (-1216406121_i32) << (*_9);
(*_16) = 8987443912356933530_u64 as usize;
_50.0.0 = Move(_51.0);
Call(_44.3.0 = fn19(), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_17 = 4_i8 as isize;
(*_47) = _29;
_49.0 = Move(_50.0);
_11.3.2 = 6824_u16 as i16;
_52 = !(*_9);
_6 = [82533136698988026429188192922003475637_i128,19062289506295177329644292046863783966_i128,(-154661980395073768331139734158533704746_i128),(-78194341645478934759307764660945218815_i128),14510506617488172287797692515502117556_i128];
_8 = _28.1;
Call(_50.1 = core::intrinsics::transmute((*_16)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_11.3.0 = _27 | _27;
_9 = &_52;
_34 = -_1;
_21 = core::ptr::addr_of_mut!((*_21));
_54 = (*_16) << _52;
_16 = &mut _54;
(*_47) = -_39;
_23 = _8;
_44.1 = (*_16) >= (*_16);
_25 = _11.3.0 - _11.3.0;
(*_21) = [158131389382054984504650900753460278368_i128,8102536427592065671684152524660960915_i128,(-18943323582536538699258738254027477005_i128),158965992080145164757197661097840329064_i128,79784904021916534863450127364462894501_i128];
_17 = (*_16) as isize;
_34 = _13 as i16;
_11.3.0 = _27 << (*_16);
(*_16) = _50.1;
_44.3.0 = _11.3.0 - _25;
Call(_49.1 = core::intrinsics::bswap((*_16)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_51.0 = core::ptr::addr_of_mut!(_5);
_29 = (*_47) + (*_47);
(*_16) = (*_47) as usize;
(*_16) = _50.1 | _50.1;
(*_21) = [94810673096393791706173518856831584781_i128,74680743143981310133150098591655157650_i128,(-166473496397514110359030953095441260620_i128),47668544290937001938084373390837772348_i128,146678311168145411242724025907271915216_i128];
(*_47) = _43 as f32;
_44.1 = _5;
(*_47) = _39 * RET;
_45 = _39 - _39;
(*_16) = (*_47) as usize;
(*_21) = [9810086293840521816482413792294774181_i128,(-138432299682425156502949587976532327581_i128),(-16137843064249284898258104393537192762_i128),(-163869127372777849885773730825745411353_i128),(-39838955559910801746475311435500333371_i128)];
(*_47) = -_29;
_36 = _19 ^ (*_9);
(*_16) = !_50.1;
Goto(bb28)
}
bb28 = {
(*_16) = 30007345113850427518232590816087010878_i128 as usize;
(*_16) = !_50.1;
_25 = _27 * _11.3.0;
_41 = (-16918037929651653869735767610692155317_i128) * 96099078488912797073716490208127927075_i128;
(*_21) = [_41,_41,_41,_41,_41];
_1 = !_22;
(*_16) = _50.1 + _50.1;
_22 = _1 + _44.3.2;
RET = (*_9) as f32;
_13 = _30 as f64;
(*_47) = RET - RET;
_11.0 = [_3,_43,_43,_43,_43,_3];
_56 = &mut (*_16);
(*_21) = [_41,_41,_41,_41,_41];
_13 = 14898031017283165427_u64 as f64;
(*_47) = RET * RET;
_11.0 = [_3,_43,_3,_3,_43,_43];
_9 = &_26;
(*_21) = [_41,_41,_41,_41,_41];
_63 = core::ptr::addr_of_mut!(_38);
(*_56) = _3 as usize;
_46 = _25;
_52 = _42 + _17;
Goto(bb29)
}
bb29 = {
Call(_67 = dump_var(Move(_25), Move(_37), Move(_24), Move(_36)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_67 = dump_var(Move(_27), Move(_19), Move(_54), Move(_38)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_67 = dump_var(Move(_22), Move(_23), Move(_26), Move(_12)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_67 = dump_var(Move(_15), Move(_6), Move(_1), Move(_31)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19() -> u8 {
mir! {
type RET = u8;
let _1: *mut u16;
let _2: (Adt58, Adt20);
let _3: *mut *mut &'static Adt18;
let _4: *mut [i128; 5];
let _5: isize;
let _6: char;
let _7: &'static mut *const u32;
let _8: *const [i64; 6];
let _9: u64;
let _10: [usize; 7];
let _11: bool;
let _12: [isize; 5];
let _13: *mut [i128; 5];
let _14: f32;
let _15: [bool; 6];
let _16: (u8, *const i16, i16);
let _17: isize;
let _18: f64;
let _19: &'static *mut &'static Adt18;
let _20: *mut *const [i64; 6];
let _21: [bool; 6];
let _22: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _23: i16;
let _24: bool;
let _25: bool;
let _26: char;
let _27: u8;
let _28: [u64; 4];
let _29: &'static ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _30: char;
let _31: *const f32;
let _32: f32;
let _33: bool;
let _34: &'static mut u32;
let _35: [u16; 1];
let _36: &'static isize;
let _37: [i128; 5];
let _38: ((*mut bool,), usize);
let _39: &'static i16;
let _40: (*mut bool,);
let _41: isize;
let _42: i64;
let _43: (u8, *const i16, i16);
let _44: isize;
let _45: i8;
let _46: &'static mut ([u32; 6], bool, [i64; 6], (u8, *const i16, i16));
let _47: f32;
let _48: [i8; 7];
let _49: ();
let _50: ();
{
RET = !246_u8;
RET = !118_u8;
RET = !59_u8;
RET = !147_u8;
RET = 108_u8 & 131_u8;
RET = !183_u8;
RET = false as u8;
RET = 68_u8 >> (-10138_i16);
RET = '\u{842f9}' as u8;
RET = !184_u8;
Goto(bb1)
}
bb1 = {
RET = 209_u8;
_6 = '\u{e435c}';
_5 = 9223372036854775807_isize ^ (-84_isize);
_5 = 277930378610843743710208277717426551592_u128 as isize;
RET = 157_u8;
_6 = '\u{d9ed5}';
_6 = '\u{1290f}';
_5 = 1812088012_i32 as isize;
RET = 167439109758305380687725264193882319165_u128 as u8;
_5 = 2286005891594437569_i64 as isize;
_6 = '\u{af11d}';
_5 = (-9223372036854775808_isize) << RET;
_2.1 = Adt20::Variant0 { fld0: 11465736715993562083_usize,fld1: (-4140_i16) };
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 16669_i16;
_5 = 9223372036854775807_isize * 9223372036854775807_isize;
match Field::<i16>(Variant(_2.1, 0), 1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
16669 => bb7,
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
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 18855_i16 << RET;
_6 = '\u{106806}';
_9 = !6921014361654113071_u64;
_6 = '\u{6eb04}';
RET = 7_u8 - 86_u8;
_6 = '\u{780af}';
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 23825_i16;
RET = !96_u8;
place!(Field::<i16>(Variant(_2.1, 0), 1)) = !(-4339_i16);
RET = 206_u8 >> Field::<i16>(Variant(_2.1, 0), 1);
_5 = (-51_isize) * 9223372036854775807_isize;
RET = 191_u8 + 124_u8;
_11 = RET <= RET;
_11 = true;
_5 = 3512480085_u32 as isize;
place!(Field::<usize>(Variant(_2.1, 0), 0)) = 2303762681640242462_usize;
_5 = 9223372036854775807_isize & (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 25392_i16 & (-6142_i16);
place!(Field::<i16>(Variant(_2.1, 0), 1)) = (-8804911898252012385_i64) as i16;
_2.1 = Adt20::Variant0 { fld0: 0_usize,fld1: 3964_i16 };
_11 = !true;
place!(Field::<i16>(Variant(_2.1, 0), 1)) = !2264_i16;
Goto(bb8)
}
bb8 = {
_10 = [4_usize,3_usize,7_usize,14475526902267318117_usize,641388091169723593_usize,1_usize,0_usize];
_2.1 = Adt20::Variant1 { fld0: _11,fld1: _5 };
_10 = [0_usize,2_usize,2_usize,5_usize,3052068343649658931_usize,12371489262703082176_usize,7945480177800799727_usize];
_9 = 17445367159916667002_u64;
place!(Field::<bool>(Variant(_2.1, 1), 0)) = _11;
place!(Field::<isize>(Variant(_2.1, 1), 1)) = _5;
place!(Field::<bool>(Variant(_2.1, 1), 0)) = _11;
_12 = [_5,Field::<isize>(Variant(_2.1, 1), 1),Field::<isize>(Variant(_2.1, 1), 1),Field::<isize>(Variant(_2.1, 1), 1),Field::<isize>(Variant(_2.1, 1), 1)];
_6 = '\u{a3512}';
_5 = Field::<isize>(Variant(_2.1, 1), 1) & Field::<isize>(Variant(_2.1, 1), 1);
_6 = '\u{1a71c}';
_9 = 15875224219457639542_u64;
_5 = Field::<isize>(Variant(_2.1, 1), 1) >> Field::<isize>(Variant(_2.1, 1), 1);
RET = 59_u8 ^ 250_u8;
_6 = '\u{e4633}';
_5 = 185019520998730961085356652758963506598_u128 as isize;
place!(Field::<isize>(Variant(_2.1, 1), 1)) = _5 ^ _5;
_2.1 = Adt20::Variant0 { fld0: 7_usize,fld1: 21155_i16 };
RET = _5 as u8;
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 25358_i16 << _5;
_14 = 136096485_u32 as f32;
_11 = true ^ false;
_5 = (-9223372036854775808_isize);
place!(Field::<i16>(Variant(_2.1, 0), 1)) = 26342_i16 ^ 15417_i16;
_10 = [117199489575435266_usize,1_usize,4537723210315139130_usize,782479134941983149_usize,13820638462625000896_usize,16480745320953875617_usize,17716905824284375646_usize];
RET = !5_u8;
_14 = _9 as f32;
_10 = [7844251007773612088_usize,8923213866666967941_usize,837756988749771326_usize,6864294473127209728_usize,13120456171502035352_usize,13118082037094063872_usize,7_usize];
_6 = '\u{1d297}';
Goto(bb9)
}
bb9 = {
_6 = '\u{bef12}';
_14 = RET as f32;
_9 = !1989836638174560360_u64;
RET = !202_u8;
_5 = -(-9223372036854775808_isize);
_2.1 = Adt20::Variant0 { fld0: 9842497135408250258_usize,fld1: 2625_i16 };
_14 = (-14782_i16) as f32;
_10 = [8790062623702287747_usize,0_usize,11106776272732173015_usize,4_usize,5967876133246952397_usize,5421593165331521860_usize,5_usize];
_14 = 3783192959457271915_usize as f32;
RET = 140_u8 - 185_u8;
_15 = [_11,_11,_11,_11,_11,_11];
_6 = '\u{92e1}';
place!(Field::<i16>(Variant(_2.1, 0), 1)) = !11383_i16;
_16.0 = !RET;
_16.2 = (-41229149643468374104557578389323788683_i128) as i16;
_16.2 = Field::<i16>(Variant(_2.1, 0), 1) & Field::<i16>(Variant(_2.1, 0), 1);
place!(Field::<usize>(Variant(_2.1, 0), 0)) = !7_usize;
_11 = !false;
_10 = [Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0),Field::<usize>(Variant(_2.1, 0), 0)];
Goto(bb10)
}
bb10 = {
_15 = [_11,_11,_11,_11,_11,_11];
_16.2 = Field::<i16>(Variant(_2.1, 0), 1);
place!(Field::<i16>(Variant(_2.1, 0), 1)) = _16.2;
_11 = _6 == _6;
_16.0 = !RET;
place!(Field::<usize>(Variant(_2.1, 0), 0)) = (-1598006229430457365_i64) as usize;
_15 = [_11,_11,_11,_11,_11,_11];
_15 = [_11,_11,_11,_11,_11,_11];
_5 = 43013189013358871775864823031570835634_i128 as isize;
_2.1 = Adt20::Variant0 { fld0: 6_usize,fld1: _16.2 };
_16.1 = core::ptr::addr_of!(_16.2);
_16.1 = core::ptr::addr_of!(_16.2);
place!(Field::<i16>(Variant(_2.1, 0), 1)) = _16.2;
_5 = -9223372036854775807_isize;
_9 = 13824774596905417517_u64 << _16.2;
_16.2 = 1163768485888526243439526785121946746_i128 as i16;
_11 = RET == _16.0;
_16.2 = Field::<i16>(Variant(_2.1, 0), 1) >> RET;
_16.2 = !Field::<i16>(Variant(_2.1, 0), 1);
_15 = [_11,_11,_11,_11,_11,_11];
_16.0 = 16_i8 as u8;
_14 = 3_usize as f32;
place!(Field::<usize>(Variant(_2.1, 0), 0)) = 1_usize;
Goto(bb11)
}
bb11 = {
_2.1 = Adt20::Variant1 { fld0: _11,fld1: _5 };
_21 = [Field::<bool>(Variant(_2.1, 1), 0),_11,_11,_11,Field::<bool>(Variant(_2.1, 1), 0),_11];
_16.0 = RET | RET;
_9 = !7001002447208889525_u64;
_14 = _16.2 as f32;
_18 = 50_i8 as f64;
_21 = [Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0),_11,Field::<bool>(Variant(_2.1, 1), 0),_11,Field::<bool>(Variant(_2.1, 1), 0)];
_12 = [_5,_5,Field::<isize>(Variant(_2.1, 1), 1),_5,_5];
_15 = [_11,_11,Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0)];
RET = 4495591110707126145_usize as u8;
place!(Field::<bool>(Variant(_2.1, 1), 0)) = !_11;
_9 = 211994665535350934523421281059693998543_u128 as u64;
_20 = core::ptr::addr_of_mut!(_8);
_20 = core::ptr::addr_of_mut!((*_20));
_14 = 6757117195826532529_usize as f32;
_17 = Field::<isize>(Variant(_2.1, 1), 1);
_21 = _15;
place!(Field::<isize>(Variant(_2.1, 1), 1)) = _17 | _17;
_20 = core::ptr::addr_of_mut!((*_20));
_20 = core::ptr::addr_of_mut!((*_20));
_16.0 = _11 as u8;
_20 = core::ptr::addr_of_mut!((*_20));
_16.0 = RET ^ RET;
_15 = [Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0),_11,_11,Field::<bool>(Variant(_2.1, 1), 0),Field::<bool>(Variant(_2.1, 1), 0)];
_16.1 = core::ptr::addr_of!(_16.2);
_17 = Field::<isize>(Variant(_2.1, 1), 1) >> _16.0;
_14 = (-157567599042786448815252943970056398887_i128) as f32;
Goto(bb12)
}
bb12 = {
_6 = '\u{e1dcd}';
_14 = RET as f32;
_16.2 = -3717_i16;
_14 = _18 as f32;
_20 = core::ptr::addr_of_mut!((*_20));
_10 = [2779968255159034131_usize,2086053818583448342_usize,0_usize,6_usize,13271020076030156376_usize,5_usize,12784617413857222934_usize];
_14 = 4767_u16 as f32;
_9 = 9037969447152056192_u64 + 16212774473117708125_u64;
_25 = _17 < Field::<isize>(Variant(_2.1, 1), 1);
_9 = RET as u64;
_15 = _21;
_15 = _21;
_14 = (-285179041429385818_i64) as f32;
_21 = [_25,Field::<bool>(Variant(_2.1, 1), 0),_11,_25,_25,_25];
Goto(bb13)
}
bb13 = {
_27 = RET;
_18 = (-20281765490402166611593892510833298465_i128) as f64;
_9 = !16277448132370779660_u64;
_17 = _5 & Field::<isize>(Variant(_2.1, 1), 1);
_9 = 15375726897109076052_u64 + 890325274008202342_u64;
_2.1 = Adt20::Variant3 { fld0: _14,fld1: _27 };
_18 = (-1622100291_i32) as f64;
_28 = [_9,_9,_9,_9];
_5 = 1207028403_u32 as isize;
_23 = _25 as i16;
_5 = _17;
_18 = Field::<f32>(Variant(_2.1, 3), 0) as f64;
_21 = [_25,_25,_11,_25,_25,_25];
_18 = _23 as f64;
_27 = Field::<f32>(Variant(_2.1, 3), 0) as u8;
_16.0 = _27 + RET;
_16.1 = core::ptr::addr_of!(_16.2);
_20 = core::ptr::addr_of_mut!((*_20));
_16.0 = !_27;
_14 = Field::<f32>(Variant(_2.1, 3), 0);
_12 = [_17,_5,_5,_17,_5];
_30 = _6;
Goto(bb14)
}
bb14 = {
_14 = 3_usize as f32;
_24 = _25;
_9 = 16336483029666214520_u64;
_32 = -_14;
_14 = _32 - _32;
_14 = 3733791468_u32 as f32;
_26 = _6;
_24 = _11 | _11;
_31 = core::ptr::addr_of!(place!(Field::<f32>(Variant(_2.1, 3), 0)));
_32 = (*_31) - (*_31);
(*_31) = _32;
(*_31) = _32 + _32;
(*_31) = _32;
(*_31) = -_32;
_31 = core::ptr::addr_of!((*_31));
(*_31) = -_14;
Goto(bb15)
}
bb15 = {
_10 = [7790133074136272552_usize,2_usize,9474126400957786961_usize,14249307547275009573_usize,7_usize,12015657488320191355_usize,5_usize];
_36 = &_17;
(*_31) = _14 * _32;
(*_31) = (*_36) as f32;
_38.1 = 2_usize;
_15 = [_25,_25,_24,_11,_24,_24];
_32 = (*_31) + (*_31);
_35 = [36297_u16];
_28 = [_9,_9,_9,_9];
RET = !Field::<u8>(Variant(_2.1, 3), 1);
_37 = [144784098222838811466570939723438817365_i128,139815072452643482249593507572892515248_i128,(-80268408445297355086054094618278266766_i128),(-81995425937680534890592509589147927882_i128),(-22928931680027365042599468017318019075_i128)];
_2.1 = Adt20::Variant3 { fld0: _32,fld1: RET };
_6 = _26;
_18 = (*_36) as f64;
Call(_9 = core::intrinsics::bswap(401560466619322775_u64), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_39 = &_16.2;
_26 = _30;
_10 = [_38.1,_38.1,_38.1,_38.1,_38.1,_38.1,_38.1];
_10 = [_38.1,_38.1,_38.1,_38.1,_38.1,_38.1,_38.1];
_33 = _11;
_40.0 = core::ptr::addr_of_mut!(_25);
_26 = _6;
RET = _27 * _27;
_38 = (Move(_40), 7_usize);
_41 = (*_36) ^ (*_36);
_38.1 = 16776553753278417507_usize & 1985743984295481880_usize;
_38.0.0 = core::ptr::addr_of_mut!(_33);
_37 = [167657658474285434358572310057026232891_i128,(-77176723134207519097401051400009670464_i128),108912732665303676187927076773605243219_i128,74364707211257505517248100598438715726_i128,31076478853027871760404961377106904058_i128];
_40 = Move(_38.0);
_16.1 = core::ptr::addr_of!((*_39));
_13 = core::ptr::addr_of_mut!(_37);
_14 = _18 as f32;
(*_13) = [(-37963573314931522224892627030327860835_i128),165934501987401082020686946015322335549_i128,(-60817887792006784314969244348430840178_i128),3737212374440777779000547803803716073_i128,144852336007764388070607446833866677738_i128];
_26 = _6;
_10 = [_38.1,_38.1,_38.1,_38.1,_38.1,_38.1,_38.1];
Call(_5 = core::intrinsics::transmute(_41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
(*_13) = [114098024928834700406688764418262329238_i128,148298325254879316629697074868979920084_i128,95068957843934866406907869344088218624_i128,136792753003470839406693261050840095415_i128,134876551493678000161576673251505520388_i128];
_4 = core::ptr::addr_of_mut!((*_13));
(*_4) = [(-21938606350058299911835360682825812906_i128),142021109730071460396151582609469206060_i128,69181449596876489783190197161560588548_i128,(-160632234891670372326872275600814420130_i128),64203593294487172796637510624869351617_i128];
(*_4) = [(-16032519903782289641812914438403158888_i128),(-78619487185102979315573424187334794847_i128),(-52664910158131435825405623285895522301_i128),(-160679091610860793201337804877260700766_i128),(-4149345907357463050788972609631620422_i128)];
_38.0 = Move(_40);
_43 = (_16.0, Move(_16.1), _23);
(*_13) = [(-69909604926361556086293470963103375255_i128),112607387362903332977393534057854705756_i128,25776935630333329811528979946913427784_i128,(-80362387154791694008651214714727109775_i128),(-100167463234235305544411910119765967765_i128)];
(*_13) = [146875254696800538465578073235178142346_i128,164873572917586885496418024301350824225_i128,(-133745516197768175964920050709731729981_i128),(-54863719077109849152687284187334685870_i128),111219908052172873097628435125723030726_i128];
(*_13) = [162450145296707701678471973153382908340_i128,144642050635361126554698850494168492214_i128,16157464956842190993026542325417426513_i128,(-99743130635004581670352271043262804082_i128),(-57728630629754438582479557090297091593_i128)];
_9 = 17594863376603195269_u64 >> (*_36);
(*_13) = [(-132001168924888113014847277763360205308_i128),93157731116764353941782560126721159878_i128,49951975778680181678309287426012329006_i128,(-31485704375303194865620292112892644585_i128),127302351384600235593957819982090785377_i128];
(*_13) = [70370844351171710850297407556714299640_i128,(-150290832298637962986982754662092259446_i128),118830302465676908778691460203495031027_i128,(-18111593121127288045469237822272713159_i128),138830004310168221389601459165061957461_i128];
_15 = [_24,_11,_24,_33,_33,_33];
_40 = (Move(_38.0.0),);
(*_13) = [(-3823242414324177199881233117012517467_i128),(-531895572100233997692112266005926798_i128),(-7316239615284660859728980310990510740_i128),98022729088241929418462139455192726972_i128,6384519615045840801972292637841056897_i128];
(*_13) = [(-133814905928997979600037001356773484215_i128),128180173397494975738618237687336498573_i128,(-34934349353966770325343448185043285282_i128),115691413280421951400479340858496647079_i128,136152952498921261291201474527380158393_i128];
_30 = _26;
_16.1 = Move(_43.1);
Goto(bb18)
}
bb18 = {
_44 = -_41;
_38 = (Move(_40), 4_usize);
_23 = (*_39);
(*_13) = [(-163285438833952606275323091091236970102_i128),169251919939266931568380755531482327530_i128,(-111105476000112815184460691215844184026_i128),(-90931378584943375172099375580381050594_i128),(-9161796149365283517166859262564550717_i128)];
_24 = !_11;
_26 = _6;
_27 = _43.2 as u8;
_42 = (-1737681238796053566_i64) ^ (-7167045120862998442_i64);
(*_13) = [66921284648623944664788252749217481614_i128,(-137690752784168649454718323842487596182_i128),(-40809474781319551175379799206765136702_i128),6793963946597780302694641775794834751_i128,(-133945262018745838328667783105801610910_i128)];
_40.0 = core::ptr::addr_of_mut!(_25);
(*_13) = [(-128361172468230869336114090781521925631_i128),(-50817761039216798865118476157155875269_i128),77098527647384999263936506707423546802_i128,(-77242817941841492379809437653620867818_i128),142365518049867628567687378172917166560_i128];
_15 = _21;
_16.2 = !_23;
_38 = (Move(_40), 14461131296374429273_usize);
_42 = 7404277788382390489_i64 >> (*_36);
_25 = (*_36) < (*_36);
_37 = [151736729996021930542988903969455753051_i128,(-80089935817364636959963795193521349487_i128),(-65682983116645411797965937659867401890_i128),148837540857929664940546048583559432813_i128,141153420310497877387177654921933525616_i128];
_43.1 = Move(_16.1);
_40 = (Move(_38.0.0),);
_17 = 225571005606229117265218956656862583954_u128 as isize;
(*_13) = [(-40163796436190669932462125003980419753_i128),115118362305042558614250696107807832702_i128,(-79489563233698229743710026982034222670_i128),22195287052237086800263600053240592243_i128,(-75336337765916656738431182824830554413_i128)];
RET = _27 & _27;
_25 = !_24;
(*_13) = [(-115058992488295260122658841904754355389_i128),83758927832591705769744248090088294971_i128,12152598629211185762889075566012501369_i128,(-108002147182292933388477069833177034404_i128),(-10604654642295401885889970394836663991_i128)];
_45 = !101_i8;
_37 = [58848495429446189341573923909630797856_i128,(-44448322991125282487039896676139123729_i128),129905266435682817349870189440292841310_i128,(-51831299995704718175771563881127794119_i128),(-128935879933824975551825851326046581603_i128)];
Goto(bb19)
}
bb19 = {
Call(_49 = dump_var(Move(_44), Move(_11), Move(_21), Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(Move(_42), Move(_24), Move(_26), Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_49 = dump_var(Move(_25), Move(_33), Move(_45), _50), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(1590030166_u32), std::hint::black_box(13596305929509233030_usize), std::hint::black_box(27_i8), std::hint::black_box(2_u8), std::hint::black_box(321881177_i32), std::hint::black_box(202822717397763990414582880333218821903_u128));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt18 {
Variant0{
fld0: u128,
fld1: char,
fld2: f64,
fld3: i128,
fld4: u8,
fld5: usize,
fld6: i64,

},
Variant1{
fld0: u128,
fld1: usize,
fld2: u8,
fld3: i8,

}}
#[derive(Debug)]
pub enum Adt20 {
Variant0{
fld0: usize,
fld1: i16,

},
Variant1{
fld0: bool,
fld1: isize,

},
Variant2{
fld0: u128,
fld1: f32,
fld2: isize,
fld3: i8,
fld4: i16,
fld5: f64,
fld6: *const i16,
fld7: i128,

},
Variant3{
fld0: f32,
fld1: u8,

}}
#[derive(Debug)]
pub enum Adt31 {
Variant0{
fld0: i64,

},
Variant1{
fld0: bool,
fld1: (*mut bool,),

},
Variant2{
fld0: u16,
fld1: (*mut bool,),
fld2: i128,
fld3: f32,
fld4: (char, i32, f64, char),

},
Variant3{
fld0: (*mut bool,),
fld1: i8,

}}
#[derive(Debug)]
pub enum Adt41 {
Variant0{
fld0: *const [i64; 6],
fld1: [u32; 6],
fld2: isize,
fld3: i8,

},
Variant1{
fld0: [usize; 4],
fld1: char,
fld2: *mut Adt18,
fld3: i8,
fld4: u128,
fld5: i32,
fld6: *const f32,

}}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: i32,
fld1: Adt41,
fld2: *mut Adt18,

},
Variant1{
fld0: [usize; 4],
fld1: [u32; 6],

},
Variant2{
fld0: *const f32,

},
Variant3{
fld0: bool,
fld1: [i64; 6],
fld2: Adt41,
fld3: usize,
fld4: Adt18,
fld5: u32,

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: (u8, *const i16, i16),
fld1: *mut [usize; 4],
fld2: ((*mut bool,), usize),
fld3: i8,
fld4: *const i16,
fld5: [u64; 3],
fld6: *const f32,
fld7: *const u32,

},
Variant1{
fld0: Adt20,
fld1: i32,

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: i64,
fld1: Adt20,
fld2: Adt18,
fld3: (*mut bool,),
fld4: [u32; 7],

},
Variant1{
fld0: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16)),

},
Variant2{
fld0: Adt41,
fld1: ([u32; 6], bool, [i64; 6], (u8, *const i16, i16)),
fld2: i32,
fld3: Adt20,

},
Variant3{
fld0: [i64; 6],
fld1: char,
fld2: [usize; 4],
fld3: *const i16,
fld4: u8,

}}
#[derive(Debug)]
pub enum Adt78 {
Variant0{
fld0: [u64; 4],
fld1: (u8, *const i16, i16),
fld2: *mut bool,
fld3: *mut [usize; 4],
fld4: [i32; 3],
fld5: (*mut [usize; 4], Adt46, [isize; 5], Adt41),
fld6: (*mut bool,),
fld7: Adt58,

},
Variant1{
fld0: *mut u16,
fld1: char,
fld2: i16,

},
Variant2{
fld0: *mut Adt18,
fld1: Adt51,
fld2: [i8; 7],
fld3: i16,

}}

