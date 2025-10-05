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
pub fn fn0(mut _1: i32,mut _2: char,mut _3: u8) -> bool {
mir! {
type RET = bool;
let _4: *mut (i128, &'static mut u128);
let _5: isize;
let _6: *mut *const usize;
let _7: *const [u64; 4];
let _8: f64;
let _9: [isize; 5];
let _10: i64;
let _11: &'static mut i32;
let _12: ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22);
let _13: char;
let _14: &'static u32;
let _15: *mut Adt48;
let _16: &'static mut i32;
let _17: ((i128, &'static mut u128),);
let _18: &'static mut (i32, *const usize);
let _19: i128;
let _20: u128;
let _21: f32;
let _22: *const *const [u64; 4];
let _23: *const &'static mut (i32, *const usize);
let _24: *const *const [u64; 4];
let _25: u64;
let _26: u64;
let _27: (i128, &'static mut u128);
let _28: bool;
let _29: u16;
let _30: f32;
let _31: i32;
let _32: isize;
let _33: u8;
let _34: *const *const [u64; 4];
let _35: &'static u32;
let _36: [u32; 2];
let _37: isize;
let _38: f32;
let _39: *const &'static mut (i128, &'static mut u128);
let _40: (((u8, usize, isize), &'static mut u128), u8);
let _41: isize;
let _42: u128;
let _43: bool;
let _44: *const u8;
let _45: &'static mut i32;
let _46: u128;
let _47: *const (i32, *const usize);
let _48: Adt22;
let _49: *const u8;
let _50: bool;
let _51: (((u8, usize, isize), &'static mut u128), u8);
let _52: *const f64;
let _53: &'static &'static Adt25;
let _54: &'static &'static u32;
let _55: i32;
let _56: &'static &'static u32;
let _57: u16;
let _58: u32;
let _59: i128;
let _60: *mut (i128, &'static mut u128);
let _61: (i128, &'static mut u128);
let _62: Adt22;
let _63: &'static mut &'static mut (i128, &'static mut u128);
let _64: i64;
let _65: &'static mut &'static mut (i128, &'static mut u128);
let _66: isize;
let _67: ();
let _68: ();
{
_1 = (-1772693241_i32) & (-1233486383_i32);
RET = false & false;
_3 = 44_u8;
_2 = '\u{5e659}';
RET = true | true;
_1 = 549993016_i32 | 409505785_i32;
_2 = '\u{14e4}';
_2 = '\u{9e1cf}';
_2 = '\u{73769}';
_3 = 103_isize as u8;
_1 = !1242049048_i32;
_3 = !187_u8;
_3 = 161_u8 + 202_u8;
_3 = 16928743303917694185_u64 as u8;
_2 = '\u{d2cd4}';
_3 = !151_u8;
_2 = '\u{361f9}';
RET = true;
_2 = '\u{c758e}';
_5 = 6_isize;
_2 = '\u{45268}';
Goto(bb1)
}
bb1 = {
_5 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_5 = (-60_isize);
_3 = 12_u8 << _1;
_1 = (-1497811135_i32);
_3 = 69_u8 - 141_u8;
_1 = (-1431663158_i32) + 959012849_i32;
RET = !true;
_5 = 9223372036854775807_isize & 9223372036854775807_isize;
_5 = _3 as isize;
_3 = !243_u8;
_2 = '\u{d42f5}';
_5 = _1 as isize;
_1 = _3 as i32;
_2 = '\u{d54a4}';
_2 = '\u{ba970}';
_3 = 175_u8 | 132_u8;
_3 = 59_u8 >> _5;
_1 = -(-1176298825_i32);
_2 = '\u{22f62}';
_2 = '\u{6d95}';
RET = true;
RET = !true;
RET = true & true;
_5 = -9223372036854775807_isize;
_8 = 10284_u16 as f64;
_2 = '\u{cdc38}';
_5 = -(-9223372036854775808_isize);
_2 = '\u{6dd1b}';
Call(_1 = core::intrinsics::bswap(1167841760_i32), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 9223372036854775807_isize;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb3)
}
bb3 = {
_12.1.2 = &mut _1;
_9 = [_5,_5,_5,_5,_5];
_13 = _2;
_11 = Move(_12.1.2);
_10 = _8 as i64;
_6 = core::ptr::addr_of_mut!((*_6));
_3 = 13_u8 - 19_u8;
_12.0 = [_13,_2,_2,_13,_2,_2,_2];
_12.0 = [_2,_2,_13,_13,_13,_2,_13];
_3 = _5 as u8;
_10 = 16244465114253307548_usize as i64;
RET = true ^ true;
_2 = _13;
_12.2 = [(-41_i8),(-85_i8),70_i8,(-61_i8),(-9_i8),60_i8,(-125_i8)];
_13 = _2;
_13 = _2;
RET = false & false;
_9 = [_5,_5,_5,_5,_5];
_3 = 99_u8 * 121_u8;
_6 = core::ptr::addr_of_mut!((*_6));
_8 = (-10422167_i32) as f64;
RET = _2 < _13;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _13 };
RET = !false;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_3 = 6_u8 << _10;
_12.0 = [_2,_13,_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_13,Field::<char>(Variant(_12.3, 0), 1)];
_13 = Field::<char>(Variant(_12.3, 0), 1);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5];
Goto(bb4)
}
bb4 = {
_8 = 1439854152_i32 as f64;
place!(Field::<char>(Variant(_12.3, 0), 1)) = _13;
_9 = [_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5,_5];
_5 = RET as isize;
_12.0 = [Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_2,_13,Field::<char>(Variant(_12.3, 0), 1)];
_10 = -(-4968872394689632360_i64);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0)];
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0),_5];
_13 = Field::<char>(Variant(_12.3, 0), 1);
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5;
Goto(bb5)
}
bb5 = {
_17.0.0 = (-106196032200901727055115976750637792694_i128) | (-15116947839390943836228096740922939887_i128);
Call((*_6) = fn1(Move(_6), Move(_12.3), _12.0, _5, _3, _5, _13, _5, _12.2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.3 = Adt22::Variant1 { fld0: Move(_12.1.1),fld1: 5043774320887746974_usize };
place!(Field::<usize>(Variant(_12.3, 1), 1)) = 4_usize * 6_usize;
_19 = _17.0.0 * _17.0.0;
_9 = [_5,_5,_5,_5,_5];
Goto(bb7)
}
bb7 = {
_8 = 623_u16 as f64;
_10 = 3442808955338523683_i64;
_12.1.1 = Move(Field::<*const usize>(Variant(_12.3, 1), 0));
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_19 = _17.0.0 << _3;
_19 = _17.0.0;
_3 = !66_u8;
match _10 {
0 => bb5,
1 => bb8,
2 => bb9,
3 => bb10,
3442808955338523683 => bb12,
_ => bb11
}
}
bb8 = {
_12.3 = Adt22::Variant1 { fld0: Move(_12.1.1),fld1: 5043774320887746974_usize };
place!(Field::<usize>(Variant(_12.3, 1), 1)) = 4_usize * 6_usize;
_19 = _17.0.0 * _17.0.0;
_9 = [_5,_5,_5,_5,_5];
Goto(bb7)
}
bb9 = {
_17.0.0 = (-106196032200901727055115976750637792694_i128) | (-15116947839390943836228096740922939887_i128);
Call((*_6) = fn1(Move(_6), Move(_12.3), _12.0, _5, _3, _5, _13, _5, _12.2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_5 = 9223372036854775807_isize;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb3)
}
bb11 = {
_12.1.2 = &mut _1;
_9 = [_5,_5,_5,_5,_5];
_13 = _2;
_11 = Move(_12.1.2);
_10 = _8 as i64;
_6 = core::ptr::addr_of_mut!((*_6));
_3 = 13_u8 - 19_u8;
_12.0 = [_13,_2,_2,_13,_2,_2,_2];
_12.0 = [_2,_2,_13,_13,_13,_2,_13];
_3 = _5 as u8;
_10 = 16244465114253307548_usize as i64;
RET = true ^ true;
_2 = _13;
_12.2 = [(-41_i8),(-85_i8),70_i8,(-61_i8),(-9_i8),60_i8,(-125_i8)];
_13 = _2;
_13 = _2;
RET = false & false;
_9 = [_5,_5,_5,_5,_5];
_3 = 99_u8 * 121_u8;
_6 = core::ptr::addr_of_mut!((*_6));
_8 = (-10422167_i32) as f64;
RET = _2 < _13;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _13 };
RET = !false;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_3 = 6_u8 << _10;
_12.0 = [_2,_13,_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_13,Field::<char>(Variant(_12.3, 0), 1)];
_13 = Field::<char>(Variant(_12.3, 0), 1);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5];
Goto(bb4)
}
bb12 = {
_2 = _13;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_13 = Field::<char>(Variant(_12.3, 0), 1);
_17.0.0 = _19 << _10;
_10 = 5365293784664031882_i64;
_24 = core::ptr::addr_of!(_7);
_26 = !15440430626704260429_u64;
_13 = _2;
RET = true;
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5 >> _17.0.0;
_6 = core::ptr::addr_of_mut!((*_6));
_2 = Field::<char>(Variant(_12.3, 0), 1);
_20 = 220877271967733165243381368327821615794_u128 * 319249032534925825955471403779391814356_u128;
_19 = _17.0.0 + _17.0.0;
_24 = core::ptr::addr_of!((*_24));
_22 = core::ptr::addr_of!((*_24));
_25 = _26 & _26;
_20 = 205274934115261537572671956663826495104_u128 << _19;
_22 = core::ptr::addr_of!((*_22));
_27.1 = &mut _20;
_28 = !RET;
match _10 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
5365293784664031882 => bb21,
_ => bb20
}
}
bb13 = {
_12.1.2 = &mut _1;
_9 = [_5,_5,_5,_5,_5];
_13 = _2;
_11 = Move(_12.1.2);
_10 = _8 as i64;
_6 = core::ptr::addr_of_mut!((*_6));
_3 = 13_u8 - 19_u8;
_12.0 = [_13,_2,_2,_13,_2,_2,_2];
_12.0 = [_2,_2,_13,_13,_13,_2,_13];
_3 = _5 as u8;
_10 = 16244465114253307548_usize as i64;
RET = true ^ true;
_2 = _13;
_12.2 = [(-41_i8),(-85_i8),70_i8,(-61_i8),(-9_i8),60_i8,(-125_i8)];
_13 = _2;
_13 = _2;
RET = false & false;
_9 = [_5,_5,_5,_5,_5];
_3 = 99_u8 * 121_u8;
_6 = core::ptr::addr_of_mut!((*_6));
_8 = (-10422167_i32) as f64;
RET = _2 < _13;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _13 };
RET = !false;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_3 = 6_u8 << _10;
_12.0 = [_2,_13,_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_13,Field::<char>(Variant(_12.3, 0), 1)];
_13 = Field::<char>(Variant(_12.3, 0), 1);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5];
Goto(bb4)
}
bb14 = {
_5 = 9223372036854775807_isize;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb3)
}
bb15 = {
_17.0.0 = (-106196032200901727055115976750637792694_i128) | (-15116947839390943836228096740922939887_i128);
Call((*_6) = fn1(Move(_6), Move(_12.3), _12.0, _5, _3, _5, _13, _5, _12.2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_5 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_5 = (-60_isize);
_3 = 12_u8 << _1;
_1 = (-1497811135_i32);
_3 = 69_u8 - 141_u8;
_1 = (-1431663158_i32) + 959012849_i32;
RET = !true;
_5 = 9223372036854775807_isize & 9223372036854775807_isize;
_5 = _3 as isize;
_3 = !243_u8;
_2 = '\u{d42f5}';
_5 = _1 as isize;
_1 = _3 as i32;
_2 = '\u{d54a4}';
_2 = '\u{ba970}';
_3 = 175_u8 | 132_u8;
_3 = 59_u8 >> _5;
_1 = -(-1176298825_i32);
_2 = '\u{22f62}';
_2 = '\u{6d95}';
RET = true;
RET = !true;
RET = true & true;
_5 = -9223372036854775807_isize;
_8 = 10284_u16 as f64;
_2 = '\u{cdc38}';
_5 = -(-9223372036854775808_isize);
_2 = '\u{6dd1b}';
Call(_1 = core::intrinsics::bswap(1167841760_i32), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_8 = 623_u16 as f64;
_10 = 3442808955338523683_i64;
_12.1.1 = Move(Field::<*const usize>(Variant(_12.3, 1), 0));
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_19 = _17.0.0 << _3;
_19 = _17.0.0;
_3 = !66_u8;
match _10 {
0 => bb5,
1 => bb8,
2 => bb9,
3 => bb10,
3442808955338523683 => bb12,
_ => bb11
}
}
bb18 = {
_5 = 9223372036854775807_isize;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb3)
}
bb19 = {
_17.0.0 = (-106196032200901727055115976750637792694_i128) | (-15116947839390943836228096740922939887_i128);
Call((*_6) = fn1(Move(_6), Move(_12.3), _12.0, _5, _3, _5, _13, _5, _12.2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb20 = {
_8 = 1439854152_i32 as f64;
place!(Field::<char>(Variant(_12.3, 0), 1)) = _13;
_9 = [_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5,_5];
_5 = RET as isize;
_12.0 = [Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_2,_13,Field::<char>(Variant(_12.3, 0), 1)];
_10 = -(-4968872394689632360_i64);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0)];
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0),_5];
_13 = Field::<char>(Variant(_12.3, 0), 1);
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5;
Goto(bb5)
}
bb21 = {
_24 = core::ptr::addr_of!((*_22));
RET = Field::<isize>(Variant(_12.3, 0), 0) < Field::<isize>(Variant(_12.3, 0), 0);
_6 = core::ptr::addr_of_mut!((*_6));
_21 = _5 as f32;
_4 = core::ptr::addr_of_mut!(_27);
(*_4).0 = !_17.0.0;
_21 = (-636_i16) as f32;
Goto(bb22)
}
bb22 = {
_24 = core::ptr::addr_of!((*_22));
(*_4).0 = Field::<isize>(Variant(_12.3, 0), 0) as i128;
_28 = _3 <= _3;
_17.0 = Move((*_4));
match _10 {
0 => bb12,
1 => bb7,
2 => bb6,
3 => bb20,
4 => bb21,
5 => bb23,
6 => bb24,
5365293784664031882 => bb26,
_ => bb25
}
}
bb23 = {
_5 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_5 = (-60_isize);
_3 = 12_u8 << _1;
_1 = (-1497811135_i32);
_3 = 69_u8 - 141_u8;
_1 = (-1431663158_i32) + 959012849_i32;
RET = !true;
_5 = 9223372036854775807_isize & 9223372036854775807_isize;
_5 = _3 as isize;
_3 = !243_u8;
_2 = '\u{d42f5}';
_5 = _1 as isize;
_1 = _3 as i32;
_2 = '\u{d54a4}';
_2 = '\u{ba970}';
_3 = 175_u8 | 132_u8;
_3 = 59_u8 >> _5;
_1 = -(-1176298825_i32);
_2 = '\u{22f62}';
_2 = '\u{6d95}';
RET = true;
RET = !true;
RET = true & true;
_5 = -9223372036854775807_isize;
_8 = 10284_u16 as f64;
_2 = '\u{cdc38}';
_5 = -(-9223372036854775808_isize);
_2 = '\u{6dd1b}';
Call(_1 = core::intrinsics::bswap(1167841760_i32), ReturnTo(bb2), UnwindUnreachable())
}
bb24 = {
_12.3 = Adt22::Variant1 { fld0: Move(_12.1.1),fld1: 5043774320887746974_usize };
place!(Field::<usize>(Variant(_12.3, 1), 1)) = 4_usize * 6_usize;
_19 = _17.0.0 * _17.0.0;
_9 = [_5,_5,_5,_5,_5];
Goto(bb7)
}
bb25 = {
_12.1.2 = &mut _1;
_9 = [_5,_5,_5,_5,_5];
_13 = _2;
_11 = Move(_12.1.2);
_10 = _8 as i64;
_6 = core::ptr::addr_of_mut!((*_6));
_3 = 13_u8 - 19_u8;
_12.0 = [_13,_2,_2,_13,_2,_2,_2];
_12.0 = [_2,_2,_13,_13,_13,_2,_13];
_3 = _5 as u8;
_10 = 16244465114253307548_usize as i64;
RET = true ^ true;
_2 = _13;
_12.2 = [(-41_i8),(-85_i8),70_i8,(-61_i8),(-9_i8),60_i8,(-125_i8)];
_13 = _2;
_13 = _2;
RET = false & false;
_9 = [_5,_5,_5,_5,_5];
_3 = 99_u8 * 121_u8;
_6 = core::ptr::addr_of_mut!((*_6));
_8 = (-10422167_i32) as f64;
RET = _2 < _13;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _13 };
RET = !false;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_3 = 6_u8 << _10;
_12.0 = [_2,_13,_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_13,Field::<char>(Variant(_12.3, 0), 1)];
_13 = Field::<char>(Variant(_12.3, 0), 1);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5];
Goto(bb4)
}
bb26 = {
_2 = Field::<char>(Variant(_12.3, 0), 1);
_30 = _21;
(*_4).0 = _17.0.0;
(*_4).0 = _19;
(*_4).0 = _19 - _19;
match _10 {
0 => bb6,
5365293784664031882 => bb27,
_ => bb17
}
}
bb27 = {
_28 = (*_4).0 > (*_4).0;
Goto(bb28)
}
bb28 = {
(*_4).0 = 587280159_i32 as i128;
(*_4).0 = 3719097562_u32 as i128;
(*_4).0 = _17.0.0 & _17.0.0;
(*_4).0 = _19 >> _17.0.0;
(*_4).0 = _17.0.0;
_21 = (*_4).0 as f32;
_32 = Field::<isize>(Variant(_12.3, 0), 0);
(*_4).0 = _19 >> _32;
_31 = !(-1813987215_i32);
_16 = &mut _31;
_8 = 2341_u16 as f64;
_32 = Field::<isize>(Variant(_12.3, 0), 0) & Field::<isize>(Variant(_12.3, 0), 0);
(*_4).0 = (-3_i8) as i128;
(*_4).0 = _17.0.0 - _19;
_34 = core::ptr::addr_of!((*_22));
(*_16) = 506277280_i32;
(*_16) = -(-1969276192_i32);
(*_4).0 = _19;
(*_4).0 = -_19;
(*_16) = -(-2007843775_i32);
Goto(bb29)
}
bb29 = {
(*_4).0 = _17.0.0;
_25 = !_26;
_28 = (*_16) >= (*_16);
_12.3 = Adt22::Variant0 { fld0: _32,fld1: _13 };
_11 = &mut (*_16);
_37 = _32;
place!(Field::<char>(Variant(_12.3, 0), 1)) = _2;
_3 = !138_u8;
_30 = _21 * _21;
match _10 {
0 => bb16,
1 => bb20,
2 => bb3,
5365293784664031882 => bb31,
_ => bb30
}
}
bb30 = {
_8 = 1439854152_i32 as f64;
place!(Field::<char>(Variant(_12.3, 0), 1)) = _13;
_9 = [_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5,_5];
_5 = RET as isize;
_12.0 = [Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_2,_13,Field::<char>(Variant(_12.3, 0), 1)];
_10 = -(-4968872394689632360_i64);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0)];
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0),_5];
_13 = Field::<char>(Variant(_12.3, 0), 1);
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5;
Goto(bb5)
}
bb31 = {
place!(Field::<char>(Variant(_12.3, 0), 1)) = _2;
place!(Field::<isize>(Variant(_12.3, 0), 0)) = -_32;
(*_4).0 = _17.0.0 + _17.0.0;
(*_11) = 1514992533_i32 << (*_4).0;
(*_11) = Field::<isize>(Variant(_12.3, 0), 0) as i32;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_40.1 = (-40_i8) as u8;
(*_11) = _10 as i32;
(*_11) = !1939334087_i32;
(*_11) = !(-559123755_i32);
_40.0.0.2 = _10 as isize;
(*_4).0 = _17.0.0 - _17.0.0;
_4 = core::ptr::addr_of_mut!((*_4));
(*_11) = !1784344479_i32;
(*_4).0 = _17.0.0 & _19;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_42 = 76659478793874642125165913668827937750_u128 << (*_4).0;
(*_4).1 = &mut _42;
_27.0 = _19;
_30 = _21 * _21;
_12.1.2 = &mut (*_11);
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_45 = Move(_12.1.2);
Goto(bb32)
}
bb32 = {
_12.1.2 = Move(_16);
_30 = _37 as f32;
_32 = RET as isize;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_12.0 = [_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_2,Field::<char>(Variant(_12.3, 0), 1),_2,_2];
_40.0.0.1 = !12762879862770417766_usize;
_40.0.0.1 = _10 as usize;
_40.0.1 = Move((*_4).1);
_6 = core::ptr::addr_of_mut!(_12.1.1);
_19 = -(*_4).0;
_46 = !119815263028832284803318938083027983665_u128;
(*_4).0 = _19;
match _10 {
0 => bb1,
1 => bb26,
2 => bb21,
3 => bb33,
4 => bb34,
5 => bb35,
5365293784664031882 => bb37,
_ => bb36
}
}
bb33 = {
_12.3 = Adt22::Variant1 { fld0: Move(_12.1.1),fld1: 5043774320887746974_usize };
place!(Field::<usize>(Variant(_12.3, 1), 1)) = 4_usize * 6_usize;
_19 = _17.0.0 * _17.0.0;
_9 = [_5,_5,_5,_5,_5];
Goto(bb7)
}
bb34 = {
_17.0.0 = (-106196032200901727055115976750637792694_i128) | (-15116947839390943836228096740922939887_i128);
Call((*_6) = fn1(Move(_6), Move(_12.3), _12.0, _5, _3, _5, _13, _5, _12.2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb35 = {
_5 = 9223372036854775807_isize;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb3)
}
bb36 = {
_2 = _13;
_6 = core::ptr::addr_of_mut!(_12.1.1);
_13 = Field::<char>(Variant(_12.3, 0), 1);
_17.0.0 = _19 << _10;
_10 = 5365293784664031882_i64;
_24 = core::ptr::addr_of!(_7);
_26 = !15440430626704260429_u64;
_13 = _2;
RET = true;
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5 >> _17.0.0;
_6 = core::ptr::addr_of_mut!((*_6));
_2 = Field::<char>(Variant(_12.3, 0), 1);
_20 = 220877271967733165243381368327821615794_u128 * 319249032534925825955471403779391814356_u128;
_19 = _17.0.0 + _17.0.0;
_24 = core::ptr::addr_of!((*_24));
_22 = core::ptr::addr_of!((*_24));
_25 = _26 & _26;
_20 = 205274934115261537572671956663826495104_u128 << _19;
_22 = core::ptr::addr_of!((*_22));
_27.1 = &mut _20;
_28 = !RET;
match _10 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
5365293784664031882 => bb21,
_ => bb20
}
}
bb37 = {
(*_4).1 = &mut _46;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_16 = Move(_11);
_48 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_12.3, 0), 0),fld1: Field::<char>(Variant(_12.3, 0), 1) };
_40.0.0.0 = !_3;
_38 = _30 * _30;
_43 = _28;
_33 = 1776198117_i32 as u8;
RET = !_43;
_30 = -_38;
Goto(bb38)
}
bb38 = {
_29 = 28100_u16 & 28278_u16;
(*_4).0 = _19 | _19;
_3 = (-136095915_i32) as u8;
(*_4).0 = _33 as i128;
(*_4).0 = _17.0.0 ^ _19;
_12.3 = Adt22::Variant1 { fld0: Move((*_6)),fld1: _40.0.0.1 };
(*_6) = core::ptr::addr_of!(_40.0.0.1);
(*_4).0 = _17.0.0 | _19;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
(*_4).0 = _19;
(*_4).0 = _19 >> Field::<isize>(Variant(_48, 0), 0);
_12.1.1 = core::ptr::addr_of!(_40.0.0.1);
_32 = Field::<isize>(Variant(_48, 0), 0) & Field::<isize>(Variant(_48, 0), 0);
_2 = _13;
match _10 {
0 => bb33,
5365293784664031882 => bb40,
_ => bb39
}
}
bb39 = {
_8 = 1439854152_i32 as f64;
place!(Field::<char>(Variant(_12.3, 0), 1)) = _13;
_9 = [_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5,_5];
_5 = RET as isize;
_12.0 = [Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_2,_13,Field::<char>(Variant(_12.3, 0), 1)];
_10 = -(-4968872394689632360_i64);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0)];
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),Field::<isize>(Variant(_12.3, 0), 0),_5];
_13 = Field::<char>(Variant(_12.3, 0), 1);
place!(Field::<isize>(Variant(_12.3, 0), 0)) = _5;
Goto(bb5)
}
bb40 = {
_30 = 2435140871_u32 as f32;
(*_4).0 = _19;
(*_4).0 = _13 as i128;
place!(Field::<isize>(Variant(_48, 0), 0)) = _32 * _32;
(*_4).0 = _17.0.0 << _17.0.0;
(*_4).0 = _19 >> _19;
(*_4).0 = _19 | _19;
_10 = (-2675491448402473632_i64);
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
_6 = core::ptr::addr_of_mut!((*_6));
_50 = !_28;
(*_4).0 = _19 - _19;
_22 = core::ptr::addr_of!((*_24));
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_40.0.0.1 = !Field::<usize>(Variant(_12.3, 1), 1);
_17 = (Move((*_4)),);
(*_6) = core::ptr::addr_of!(_40.0.0.1);
place!(Field::<usize>(Variant(_12.3, 1), 1)) = _40.0.0.1 + _40.0.0.1;
match _10 {
0 => bb32,
1 => bb30,
2 => bb24,
3 => bb41,
4 => bb42,
340282366920938463460699115983365737824 => bb44,
_ => bb43
}
}
bb41 = {
_12.3 = Adt22::Variant1 { fld0: Move(_12.1.1),fld1: 5043774320887746974_usize };
place!(Field::<usize>(Variant(_12.3, 1), 1)) = 4_usize * 6_usize;
_19 = _17.0.0 * _17.0.0;
_9 = [_5,_5,_5,_5,_5];
Goto(bb7)
}
bb42 = {
_12.1.2 = &mut _1;
_9 = [_5,_5,_5,_5,_5];
_13 = _2;
_11 = Move(_12.1.2);
_10 = _8 as i64;
_6 = core::ptr::addr_of_mut!((*_6));
_3 = 13_u8 - 19_u8;
_12.0 = [_13,_2,_2,_13,_2,_2,_2];
_12.0 = [_2,_2,_13,_13,_13,_2,_13];
_3 = _5 as u8;
_10 = 16244465114253307548_usize as i64;
RET = true ^ true;
_2 = _13;
_12.2 = [(-41_i8),(-85_i8),70_i8,(-61_i8),(-9_i8),60_i8,(-125_i8)];
_13 = _2;
_13 = _2;
RET = false & false;
_9 = [_5,_5,_5,_5,_5];
_3 = 99_u8 * 121_u8;
_6 = core::ptr::addr_of_mut!((*_6));
_8 = (-10422167_i32) as f64;
RET = _2 < _13;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _13 };
RET = !false;
_12.3 = Adt22::Variant0 { fld0: _5,fld1: _2 };
_3 = 6_u8 << _10;
_12.0 = [_2,_13,_2,Field::<char>(Variant(_12.3, 0), 1),Field::<char>(Variant(_12.3, 0), 1),_13,Field::<char>(Variant(_12.3, 0), 1)];
_13 = Field::<char>(Variant(_12.3, 0), 1);
_9 = [Field::<isize>(Variant(_12.3, 0), 0),_5,Field::<isize>(Variant(_12.3, 0), 0),_5,_5];
Goto(bb4)
}
bb43 = {
_5 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_5 = (-60_isize);
_3 = 12_u8 << _1;
_1 = (-1497811135_i32);
_3 = 69_u8 - 141_u8;
_1 = (-1431663158_i32) + 959012849_i32;
RET = !true;
_5 = 9223372036854775807_isize & 9223372036854775807_isize;
_5 = _3 as isize;
_3 = !243_u8;
_2 = '\u{d42f5}';
_5 = _1 as isize;
_1 = _3 as i32;
_2 = '\u{d54a4}';
_2 = '\u{ba970}';
_3 = 175_u8 | 132_u8;
_3 = 59_u8 >> _5;
_1 = -(-1176298825_i32);
_2 = '\u{22f62}';
_2 = '\u{6d95}';
RET = true;
RET = !true;
RET = true & true;
_5 = -9223372036854775807_isize;
_8 = 10284_u16 as f64;
_2 = '\u{cdc38}';
_5 = -(-9223372036854775808_isize);
_2 = '\u{6dd1b}';
Call(_1 = core::intrinsics::bswap(1167841760_i32), ReturnTo(bb2), UnwindUnreachable())
}
bb44 = {
_44 = core::ptr::addr_of!(_3);
(*_44) = _40.0.0.0;
(*_6) = Move(Field::<*const usize>(Variant(_12.3, 1), 0));
_10 = (-8947202908573204720_i64);
_27.0 = _17.0.0 - _19;
(*_44) = _43 as u8;
(*_44) = _33;
_10 = _38 as i64;
(*_4).0 = _17.0.0 | _19;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_33 = _40.1;
_40.0.0.1 = Field::<usize>(Variant(_12.3, 1), 1) >> (*_4).0;
(*_4).0 = !_19;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
(*_4).0 = _19;
Goto(bb45)
}
bb45 = {
(*_44) = _40.1 ^ _33;
_30 = _21 - _38;
(*_44) = !_40.0.0.0;
(*_44) = _8 as u8;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
_41 = _32;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_44 = core::ptr::addr_of!((*_44));
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
Goto(bb46)
}
bb46 = {
(*_6) = core::ptr::addr_of!(_40.0.0.1);
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
(*_44) = _40.1 | _33;
_28 = _43 | RET;
_27.0 = _19 << (*_44);
(*_4).0 = _19 + _19;
_29 = 46311_u16 | 46711_u16;
(*_44) = _40.0.0.0 - _33;
(*_44) = _33 + _40.0.0.0;
_17.0.0 = _30 as i128;
_40.0.0.1 = Field::<usize>(Variant(_12.3, 1), 1) ^ Field::<usize>(Variant(_12.3, 1), 1);
place!(Field::<char>(Variant(_48, 0), 1)) = _13;
(*_4).0 = -_17.0.0;
(*_44) = _38 as u8;
(*_6) = core::ptr::addr_of!(_51.0.0.1);
(*_4).0 = -_17.0.0;
_24 = core::ptr::addr_of!((*_22));
_51.1 = Field::<isize>(Variant(_48, 0), 0) as u8;
_36 = [2977063483_u32,692703052_u32];
_25 = _26 & _26;
Goto(bb47)
}
bb47 = {
_2 = _13;
(*_44) = 91_i8 as u8;
_51.0.0.1 = !Field::<usize>(Variant(_12.3, 1), 1);
(*_44) = _51.1 ^ _51.1;
_51.1 = (*_44) >> (*_44);
_29 = 61996_u16 ^ 63856_u16;
RET = !_28;
(*_44) = !_40.1;
(*_6) = core::ptr::addr_of!(_51.0.0.1);
_12.2 = [127_i8,(-12_i8),(-33_i8),36_i8,88_i8,(-9_i8),(-33_i8)];
(*_6) = core::ptr::addr_of!(_51.0.0.1);
(*_44) = _51.1 * _51.1;
(*_6) = core::ptr::addr_of!(_51.0.0.1);
_33 = (*_44) - (*_44);
(*_4).0 = -_17.0.0;
_4 = core::ptr::addr_of_mut!(_27);
Goto(bb48)
}
bb48 = {
_44 = core::ptr::addr_of!((*_44));
(*_44) = _51.1;
(*_6) = core::ptr::addr_of!(_40.0.0.1);
_30 = _38 + _38;
_29 = !7504_u16;
_51.0.0.2 = Field::<isize>(Variant(_48, 0), 0);
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
_58 = 3336659500_u32;
_12.0 = [_2,_2,_13,_13,_13,_13,_2];
_55 = (-228658402_i32) * (-777810509_i32);
(*_4).0 = -_17.0.0;
_30 = -_38;
place!(Field::<char>(Variant(_48, 0), 1)) = _2;
_61.0 = -(*_4).0;
(*_6) = core::ptr::addr_of!(_51.0.0.1);
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12.3, 1), 1)));
_22 = core::ptr::addr_of!((*_34));
(*_6) = core::ptr::addr_of!(_51.0.0.1);
(*_44) = !_51.1;
Goto(bb49)
}
bb49 = {
_35 = &_58;
_44 = core::ptr::addr_of!(_33);
_58 = 880005103_u32 | 3430218335_u32;
(*_44) = !_3;
_52 = core::ptr::addr_of!(_8);
_59 = (*_4).0 + (*_4).0;
_13 = Field::<char>(Variant(_48, 0), 1);
place!(Field::<char>(Variant(_48, 0), 1)) = _2;
_51.1 = !(*_44);
(*_6) = core::ptr::addr_of!(_40.0.0.1);
(*_4).0 = _59 + _61.0;
(*_6) = core::ptr::addr_of!(_51.0.0.1);
_41 = _37;
(*_4).0 = _17.0.0;
_66 = -Field::<isize>(Variant(_48, 0), 0);
Goto(bb50)
}
bb50 = {
Call(_67 = dump_var(Move(_59), Move(_42), Move(_50), Move(_1)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_67 = dump_var(Move(_25), Move(_2), Move(_29), Move(_33)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_67 = dump_var(Move(_19), Move(_9), Move(_28), Move(_26)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_67 = dump_var(Move(_58), _68, _68, _68), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: *mut *const usize,mut _2: Adt22,mut _3: [char; 7],mut _4: isize,mut _5: u8,mut _6: isize,mut _7: char,mut _8: isize,mut _9: [i8; 7],mut _10: isize) -> *const usize {
mir! {
type RET = *const usize;
let _11: *const [u64; 4];
let _12: &'static mut &'static mut (i128, &'static mut u128);
let _13: i64;
let _14: &'static mut i32;
let _15: [i32; 7];
let _16: &'static mut &'static mut (i128, &'static mut u128);
let _17: usize;
let _18: i16;
let _19: u16;
let _20: &'static &'static Adt25;
let _21: isize;
let _22: f64;
let _23: *mut [i64; 2];
let _24: f32;
let _25: f32;
let _26: *const *const [u64; 4];
let _27: &'static mut u128;
let _28: &'static u32;
let _29: ();
let _30: ();
{
_6 = _4;
_1 = core::ptr::addr_of_mut!(RET);
_3 = [Field::<char>(Variant(_2, 0), 1),Field::<char>(Variant(_2, 0), 1),Field::<char>(Variant(_2, 0), 1),_7,_7,Field::<char>(Variant(_2, 0), 1),Field::<char>(Variant(_2, 0), 1)];
place!(Field::<isize>(Variant(_2, 0), 0)) = _6 >> _4;
_1 = core::ptr::addr_of_mut!((*_1));
_9 = [0_i8,(-17_i8),79_i8,(-94_i8),(-122_i8),(-42_i8),(-43_i8)];
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
_7 = Field::<char>(Variant(_2, 0), 1);
_4 = Field::<isize>(Variant(_2, 0), 0);
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
_7 = Field::<char>(Variant(_2, 0), 1);
_3 = [Field::<char>(Variant(_2, 0), 1),_7,Field::<char>(Variant(_2, 0), 1),_7,Field::<char>(Variant(_2, 0), 1),_7,Field::<char>(Variant(_2, 0), 1)];
_10 = -_4;
Goto(bb1)
}
bb1 = {
_8 = _6 ^ _10;
_5 = 129_u8 | 191_u8;
_6 = _4;
_4 = _6 * _8;
_4 = _6 + _8;
_9 = [102_i8,(-4_i8),(-35_i8),18_i8,68_i8,79_i8,57_i8];
_2 = Adt22::Variant0 { fld0: _10,fld1: _7 };
_10 = _4;
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
_10 = -_8;
_1 = core::ptr::addr_of_mut!((*_1));
_7 = Field::<char>(Variant(_2, 0), 1);
_4 = _6;
_5 = false as u8;
_13 = !(-733596399311336201_i64);
_8 = _10;
_15 = [(-1350440601_i32),1774041613_i32,718187920_i32,1416407164_i32,430275234_i32,573965210_i32,(-621676332_i32)];
_5 = 92_u8 & 145_u8;
place!(Field::<isize>(Variant(_2, 0), 0)) = _8 | _8;
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
Goto(bb2)
}
bb2 = {
_4 = (-12965_i16) as isize;
_3 = [_7,Field::<char>(Variant(_2, 0), 1),_7,_7,Field::<char>(Variant(_2, 0), 1),_7,Field::<char>(Variant(_2, 0), 1)];
_4 = Field::<isize>(Variant(_2, 0), 0) & Field::<isize>(Variant(_2, 0), 0);
Call(RET = fn2(_9, Move(_2), Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = '\u{2549f}';
_15 = [(-1605404237_i32),(-1283813956_i32),1390538721_i32,(-1219760633_i32),(-1091057966_i32),109602400_i32,498985272_i32];
_15 = [(-296495454_i32),(-1846099225_i32),1213363663_i32,1010537216_i32,891249128_i32,879126330_i32,(-1917505655_i32)];
_9 = [(-9_i8),80_i8,110_i8,36_i8,123_i8,(-29_i8),(-81_i8)];
_1 = core::ptr::addr_of_mut!(RET);
_2 = Adt22::Variant0 { fld0: _6,fld1: _7 };
(*_1) = core::ptr::addr_of!(_17);
_3 = [Field::<char>(Variant(_2, 0), 1),_7,_7,_7,Field::<char>(Variant(_2, 0), 1),_7,_7];
(*RET) = !9968372205391112484_usize;
(*RET) = 7_usize ^ 17979796968936704448_usize;
(*_1) = core::ptr::addr_of!((*RET));
_17 = 0_usize;
(*_1) = core::ptr::addr_of!((*RET));
Goto(bb4)
}
bb4 = {
(*RET) = 2_usize << _5;
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
Goto(bb5)
}
bb5 = {
(*RET) = 0_usize ^ 0_usize;
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
_13 = -(-4941775911172177257_i64);
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = 3_usize & 898405857307356812_usize;
(*RET) = 3_usize ^ 0_usize;
(*RET) = 5_usize - 2_usize;
(*RET) = 5972340416875889922_usize;
_17 = 540550251_i32 as usize;
(*RET) = 5318757129624773044_usize - 6_usize;
(*RET) = !2927351355595633877_usize;
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = 5_usize * 7701189274675921464_usize;
(*_1) = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_17);
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = !1_usize;
Goto(bb6)
}
bb6 = {
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = 919095706952595902_usize;
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
_13 = 2939228779914088621_i64 * (-3380459037455118590_i64);
_9 = [(-44_i8),119_i8,13_i8,(-101_i8),(-100_i8),60_i8,(-62_i8)];
(*_1) = core::ptr::addr_of!((*RET));
match (*RET) {
0 => bb3,
1 => bb5,
2 => bb7,
3 => bb8,
4 => bb9,
919095706952595902 => bb11,
_ => bb10
}
}
bb7 = {
_8 = _6 ^ _10;
_5 = 129_u8 | 191_u8;
_6 = _4;
_4 = _6 * _8;
_4 = _6 + _8;
_9 = [102_i8,(-4_i8),(-35_i8),18_i8,68_i8,79_i8,57_i8];
_2 = Adt22::Variant0 { fld0: _10,fld1: _7 };
_10 = _4;
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
_10 = -_8;
_1 = core::ptr::addr_of_mut!((*_1));
_7 = Field::<char>(Variant(_2, 0), 1);
_4 = _6;
_5 = false as u8;
_13 = !(-733596399311336201_i64);
_8 = _10;
_15 = [(-1350440601_i32),1774041613_i32,718187920_i32,1416407164_i32,430275234_i32,573965210_i32,(-621676332_i32)];
_5 = 92_u8 & 145_u8;
place!(Field::<isize>(Variant(_2, 0), 0)) = _8 | _8;
place!(Field::<char>(Variant(_2, 0), 1)) = _7;
Goto(bb2)
}
bb8 = {
(*RET) = 2_usize << _5;
(*_1) = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!((*RET));
Goto(bb5)
}
bb9 = {
_7 = '\u{2549f}';
_15 = [(-1605404237_i32),(-1283813956_i32),1390538721_i32,(-1219760633_i32),(-1091057966_i32),109602400_i32,498985272_i32];
_15 = [(-296495454_i32),(-1846099225_i32),1213363663_i32,1010537216_i32,891249128_i32,879126330_i32,(-1917505655_i32)];
_9 = [(-9_i8),80_i8,110_i8,36_i8,123_i8,(-29_i8),(-81_i8)];
_1 = core::ptr::addr_of_mut!(RET);
_2 = Adt22::Variant0 { fld0: _6,fld1: _7 };
(*_1) = core::ptr::addr_of!(_17);
_3 = [Field::<char>(Variant(_2, 0), 1),_7,_7,_7,Field::<char>(Variant(_2, 0), 1),_7,_7];
(*RET) = !9968372205391112484_usize;
(*RET) = 7_usize ^ 17979796968936704448_usize;
(*_1) = core::ptr::addr_of!((*RET));
_17 = 0_usize;
(*_1) = core::ptr::addr_of!((*RET));
Goto(bb4)
}
bb10 = {
_4 = (-12965_i16) as isize;
_3 = [_7,Field::<char>(Variant(_2, 0), 1),_7,_7,Field::<char>(Variant(_2, 0), 1),_7,Field::<char>(Variant(_2, 0), 1)];
_4 = Field::<isize>(Variant(_2, 0), 0) & Field::<isize>(Variant(_2, 0), 0);
Call(RET = fn2(_9, Move(_2), Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
(*RET) = 7_usize << _4;
(*_1) = core::ptr::addr_of!((*RET));
_5 = 65_u8 >> (*RET);
_22 = (*RET) as f64;
_24 = (-21_i8) as f32;
_21 = _4 & Field::<isize>(Variant(_2, 0), 0);
_22 = 261372160_u32 as f64;
Goto(bb12)
}
bb12 = {
Call(_29 = dump_var(Move(_6), Move(_21), Move(_4), Move(_3)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_29 = dump_var(Move(_8), Move(_17), _30, _30), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [i8; 7],mut _2: Adt22,mut _3: *mut *const usize) -> *const usize {
mir! {
type RET = *const usize;
let _4: *const f64;
let _5: isize;
let _6: u32;
let _7: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _8: f32;
let _9: f64;
let _10: *mut (i128, &'static mut u128);
let _11: bool;
let _12: &'static mut i32;
let _13: &'static mut i32;
let _14: ([i64; 2], [u128; 5], &'static mut u128);
let _15: bool;
let _16: f64;
let _17: char;
let _18: *mut [i64; 2];
let _19: &'static Adt25;
let _20: Adt38;
let _21: &'static mut i32;
let _22: i64;
let _23: i64;
let _24: i128;
let _25: [isize; 7];
let _26: f32;
let _27: *const *const [u64; 4];
let _28: [i32; 2];
let _29: isize;
let _30: [u128; 6];
let _31: *mut Adt48;
let _32: isize;
let _33: i8;
let _34: f32;
let _35: *mut [i64; 2];
let _36: &'static Adt25;
let _37: u64;
let _38: usize;
let _39: f32;
let _40: i64;
let _41: *const usize;
let _42: (&'static &'static u32, [u128; 6], char);
let _43: [char; 8];
let _44: isize;
let _45: &'static &'static u32;
let _46: f32;
let _47: [u32; 2];
let _48: i8;
let _49: f32;
let _50: &'static mut (i128, &'static mut u128);
let _51: i32;
let _52: ((i128, &'static mut u128),);
let _53: isize;
let _54: u64;
let _55: Adt73;
let _56: isize;
let _57: u128;
let _58: &'static mut (i32, *const usize);
let _59: isize;
let _60: u16;
let _61: f64;
let _62: usize;
let _63: *const &'static mut (i128, &'static mut u128);
let _64: &'static mut &'static mut (i128, &'static mut u128);
let _65: &'static mut i32;
let _66: isize;
let _67: ();
let _68: ();
{
_1 = [(-86_i8),(-91_i8),71_i8,97_i8,(-121_i8),(-94_i8),(-9_i8)];
_3 = core::ptr::addr_of_mut!(RET);
place!(Field::<isize>(Variant(_2, 0), 0)) = (-9223372036854775808_isize) * 113_isize;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = [(-9_i8),9_i8,105_i8,4_i8,31_i8,88_i8,103_i8];
Goto(bb1)
}
bb1 = {
place!(Field::<char>(Variant(_2, 0), 1)) = '\u{357a1}';
place!(Field::<char>(Variant(_2, 0), 1)) = '\u{f9a8d}';
_3 = core::ptr::addr_of_mut!((*_3));
place!(Field::<isize>(Variant(_2, 0), 0)) = (-9223372036854775808_isize);
_5 = Field::<isize>(Variant(_2, 0), 0) + Field::<isize>(Variant(_2, 0), 0);
place!(Field::<char>(Variant(_2, 0), 1)) = '\u{bb403}';
_2 = Adt22::Variant0 { fld0: _5,fld1: '\u{7d2d8}' };
_5 = !Field::<isize>(Variant(_2, 0), 0);
_5 = -Field::<isize>(Variant(_2, 0), 0);
place!(Field::<char>(Variant(_2, 0), 1)) = '\u{d5919}';
_8 = 195_u8 as f32;
_7.1 = Field::<char>(Variant(_2, 0), 1);
place!(Field::<isize>(Variant(_2, 0), 0)) = _5 | _5;
_7.2 = !(-29817_i16);
Call((*_3) = fn3(Move(_2), Move(_3), _7.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = core::ptr::addr_of_mut!(RET);
_2 = Adt22::Variant0 { fld0: _5,fld1: _7.1 };
_7.2 = !770_i16;
_5 = Field::<isize>(Variant(_2, 0), 0);
place!(Field::<char>(Variant(_2, 0), 1)) = _7.1;
_7.2 = 7855_i16 + 2659_i16;
_7.1 = Field::<char>(Variant(_2, 0), 1);
_5 = 15495212188321827729_u64 as isize;
_8 = 4153249229_u32 as f32;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 15346740216678127413_usize };
_7.1 = '\u{ae770}';
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = !4069617852813097398_usize;
(*RET) = 4_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 5725731884996183523_usize;
(*RET) = (-75_i8) as usize;
(*RET) = !11844546944524624302_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_6 = 9050299325970230886_u64 as u32;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 3986007982190988368651760659151154501_u128 as usize;
_4 = core::ptr::addr_of!(_9);
(*_4) = 14073417993821424751_u64 as f64;
Call((*RET) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_9 = 13992852134308336326_u64 as f64;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 7899121447099896165_usize & 14541414676073709482_usize;
(*RET) = 7929986877315095878_usize;
(*RET) = !6_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_11 = true;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = _5 as f32;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_4) = 5_u8 as f64;
(*RET) = 7902684732526400913_usize << _6;
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Goto(bb4)
}
bb4 = {
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 13784451795644834750_usize;
_7.1 = '\u{3e763}';
_6 = 177_u8 as u32;
(*_4) = _8 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 57078_u16 as f64;
(*RET) = !1699250695581916208_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = !1_usize;
_1 = [80_i8,94_i8,(-107_i8),(-28_i8),124_i8,8_i8,125_i8];
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 149_u8 as f64;
(*RET) = _11 as usize;
_17 = _7.1;
(*RET) = 6_usize * 6_usize;
(*RET) = 4626040888299866634_usize >> _7.2;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 165185308414627707603064705498487597960_i128 as f64;
_14.0 = [7905584152773944739_i64,1552234828642392793_i64];
Goto(bb5)
}
bb5 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb6 = {
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _5 as usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_24 = (-85518781735048623556943036658987368728_i128);
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 0_usize };
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
match _24 {
254763585185889839906431570772780842728 => bb8,
_ => bb7
}
}
bb7 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb8 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb9 = {
_14.1 = [239314058136682829058618153901243015011_u128,289311638826614431101199910288557460563_u128,99229067879290877682794088951773781136_u128,197952172790162409347790455960451473041_u128,186501940519248061059826754361695227186_u128];
(*RET) = 13242077828781098131_usize;
(*RET) = 4_usize << _6;
(*_4) = -_16;
(*RET) = 5_usize;
_15 = (*_4) != (*_4);
(*_18) = [718920382768500318_i64,7556351282731927408_i64];
(*_4) = -_16;
_9 = -_16;
(*_4) = _16 - _16;
(*_18) = [5209694812337029311_i64,(-8428909366433842712_i64)];
(*RET) = !18318325918657224612_usize;
(*_4) = _16;
(*RET) = 3_usize;
_26 = _8;
(*_4) = _16 + _16;
_8 = _26;
(*_3) = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_7.2 = 11443_i16;
(*_18) = [(-290657760362600975_i64),4818107213583910136_i64];
Goto(bb10)
}
bb10 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16 - _16;
_7.1 = _17;
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_18) = [(-7153013453803509182_i64),8868349503432131141_i64];
match (*RET) {
0 => bb8,
1 => bb2,
2 => bb7,
4 => bb12,
3 => bb14,
_ => bb13
}
}
bb11 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_9 = 13992852134308336326_u64 as f64;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 7899121447099896165_usize & 14541414676073709482_usize;
(*RET) = 7929986877315095878_usize;
(*RET) = !6_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_11 = true;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = _5 as f32;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_4) = 5_u8 as f64;
(*RET) = 7902684732526400913_usize << _6;
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Goto(bb4)
}
bb12 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb13 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb14 = {
(*_18) = [4289082507770162061_i64,(-1183504578506993144_i64)];
(*_4) = _5 as f64;
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = _16 - _16;
place!(Field::<usize>(Variant(_2, 1), 1)) = 23558020092869729516613727834772295079_u128 as usize;
_1 = [74_i8,37_i8,(-75_i8),(-73_i8),53_i8,113_i8,66_i8];
(*RET) = 5_usize + 6_usize;
(*_4) = _16 * _16;
(*_4) = (-61_i8) as f64;
(*RET) = 10775180495414966827_usize >> _7.2;
(*RET) = !2223682988010111493_usize;
(*RET) = 0_usize;
(*_4) = -_16;
(*RET) = 4_usize;
(*_4) = -_16;
(*_4) = _16 * _16;
(*RET) = 6_usize;
(*_4) = _16;
(*_4) = _16 * _16;
(*_4) = -_16;
_7.1 = _17;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_16;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_18) = [3739376633070700344_i64,2706753521791367251_i64];
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = -_16;
Goto(bb15)
}
bb15 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16 - _16;
_33 = (-3894705087791618736_i64) as i8;
(*RET) = 16415417054187111062_usize + 1_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16;
(*RET) = 3_usize | 2_usize;
(*_18) = [(-3298559107091536368_i64),2818205780506181156_i64];
(*_4) = _16 - _16;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16;
RET = core::ptr::addr_of!((*RET));
(*RET) = 6_usize;
(*_18) = [(-6617452493696170633_i64),6361332992090828009_i64];
(*RET) = 17672434362323984206_usize * 11737374329184078978_usize;
_26 = -_8;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb16)
}
bb16 = {
(*_18) = [(-1531759842864910026_i64),(-1088022569427804454_i64)];
(*_3) = core::ptr::addr_of!((*RET));
(*_18) = [2099566491080510962_i64,7031824450450010234_i64];
(*_3) = core::ptr::addr_of!((*RET));
_37 = !9133912606952027485_u64;
(*_3) = core::ptr::addr_of!(_38);
_5 = 122_isize >> _37;
(*RET) = Field::<usize>(Variant(_2, 1), 1) & Field::<usize>(Variant(_2, 1), 1);
(*_18) = [1421323758255396311_i64,(-8601385567997044122_i64)];
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
_30 = [91770079223736312425566092810996038424_u128,301121251498098905266152616319286209498_u128,301701619862770761418500565987465099165_u128,210327128307186579097692734973361028063_u128,153833566421168125267809761550827600441_u128,284169667671632830114587087545681630587_u128];
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
(*_4) = _6 as f64;
(*_4) = _16 - _16;
_35 = core::ptr::addr_of_mut!((*_18));
(*_4) = _5 as f64;
match _7.2 {
0 => bb11,
1 => bb6,
2 => bb8,
3 => bb13,
4 => bb17,
5 => bb18,
11443 => bb20,
_ => bb19
}
}
bb17 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb18 = {
(*_18) = [4289082507770162061_i64,(-1183504578506993144_i64)];
(*_4) = _5 as f64;
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = _16 - _16;
place!(Field::<usize>(Variant(_2, 1), 1)) = 23558020092869729516613727834772295079_u128 as usize;
_1 = [74_i8,37_i8,(-75_i8),(-73_i8),53_i8,113_i8,66_i8];
(*RET) = 5_usize + 6_usize;
(*_4) = _16 * _16;
(*_4) = (-61_i8) as f64;
(*RET) = 10775180495414966827_usize >> _7.2;
(*RET) = !2223682988010111493_usize;
(*RET) = 0_usize;
(*_4) = -_16;
(*RET) = 4_usize;
(*_4) = -_16;
(*_4) = _16 * _16;
(*RET) = 6_usize;
(*_4) = _16;
(*_4) = _16 * _16;
(*_4) = -_16;
_7.1 = _17;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_16;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_18) = [3739376633070700344_i64,2706753521791367251_i64];
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = -_16;
Goto(bb15)
}
bb19 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb20 = {
(*_18) = [(-7694901404219952187_i64),1297998594407023530_i64];
(*_4) = 928427704_i32 as f64;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
(*RET) = Field::<usize>(Variant(_2, 1), 1) & Field::<usize>(Variant(_2, 1), 1);
(*_4) = _16 * _16;
(*_4) = -_16;
(*RET) = Field::<usize>(Variant(_2, 1), 1) | Field::<usize>(Variant(_2, 1), 1);
(*_18) = [8898294256205568588_i64,(-1369679175549971301_i64)];
(*_4) = -_16;
_24 = (-135552975922953633445503035408715819852_i128) & 74746828846973376006209127926454390847_i128;
(*_4) = -_16;
(*_18) = [1474242179697440535_i64,(-846828315351297229_i64)];
_11 = !_15;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_49 = -_26;
(*RET) = Field::<usize>(Variant(_2, 1), 1) >> _5;
(*_18) = [(-6754024256999642270_i64),4649696996118852348_i64];
match _7.2 {
0 => bb14,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb26,
11443 => bb28,
_ => bb27
}
}
bb21 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb22 = {
_3 = core::ptr::addr_of_mut!(RET);
_2 = Adt22::Variant0 { fld0: _5,fld1: _7.1 };
_7.2 = !770_i16;
_5 = Field::<isize>(Variant(_2, 0), 0);
place!(Field::<char>(Variant(_2, 0), 1)) = _7.1;
_7.2 = 7855_i16 + 2659_i16;
_7.1 = Field::<char>(Variant(_2, 0), 1);
_5 = 15495212188321827729_u64 as isize;
_8 = 4153249229_u32 as f32;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 15346740216678127413_usize };
_7.1 = '\u{ae770}';
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = !4069617852813097398_usize;
(*RET) = 4_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 5725731884996183523_usize;
(*RET) = (-75_i8) as usize;
(*RET) = !11844546944524624302_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_6 = 9050299325970230886_u64 as u32;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 3986007982190988368651760659151154501_u128 as usize;
_4 = core::ptr::addr_of!(_9);
(*_4) = 14073417993821424751_u64 as f64;
Call((*RET) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb23 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb24 = {
(*_18) = [(-1531759842864910026_i64),(-1088022569427804454_i64)];
(*_3) = core::ptr::addr_of!((*RET));
(*_18) = [2099566491080510962_i64,7031824450450010234_i64];
(*_3) = core::ptr::addr_of!((*RET));
_37 = !9133912606952027485_u64;
(*_3) = core::ptr::addr_of!(_38);
_5 = 122_isize >> _37;
(*RET) = Field::<usize>(Variant(_2, 1), 1) & Field::<usize>(Variant(_2, 1), 1);
(*_18) = [1421323758255396311_i64,(-8601385567997044122_i64)];
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
_30 = [91770079223736312425566092810996038424_u128,301121251498098905266152616319286209498_u128,301701619862770761418500565987465099165_u128,210327128307186579097692734973361028063_u128,153833566421168125267809761550827600441_u128,284169667671632830114587087545681630587_u128];
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
(*_4) = _6 as f64;
(*_4) = _16 - _16;
_35 = core::ptr::addr_of_mut!((*_18));
(*_4) = _5 as f64;
match _7.2 {
0 => bb11,
1 => bb6,
2 => bb8,
3 => bb13,
4 => bb17,
5 => bb18,
11443 => bb20,
_ => bb19
}
}
bb25 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16 - _16;
_33 = (-3894705087791618736_i64) as i8;
(*RET) = 16415417054187111062_usize + 1_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16;
(*RET) = 3_usize | 2_usize;
(*_18) = [(-3298559107091536368_i64),2818205780506181156_i64];
(*_4) = _16 - _16;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _16;
RET = core::ptr::addr_of!((*RET));
(*RET) = 6_usize;
(*_18) = [(-6617452493696170633_i64),6361332992090828009_i64];
(*RET) = 17672434362323984206_usize * 11737374329184078978_usize;
_26 = -_8;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb16)
}
bb26 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_9 = 13992852134308336326_u64 as f64;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 7899121447099896165_usize & 14541414676073709482_usize;
(*RET) = 7929986877315095878_usize;
(*RET) = !6_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_11 = true;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = _5 as f32;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_4) = 5_u8 as f64;
(*RET) = 7902684732526400913_usize << _6;
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Goto(bb4)
}
bb27 = {
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _5 as usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_24 = (-85518781735048623556943036658987368728_i128);
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 0_usize };
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
match _24 {
254763585185889839906431570772780842728 => bb8,
_ => bb7
}
}
bb28 = {
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
_18 = Move(_35);
(*RET) = Field::<usize>(Variant(_2, 1), 1) ^ Field::<usize>(Variant(_2, 1), 1);
_14.1 = [9405498038158203932500269247631150050_u128,224199959659172077123460416175874047140_u128,120653462681885651645060385008356437444_u128,290579292835659690521676296826750306834_u128,31797805574973786496516653087815273804_u128];
_39 = -_8;
_47 = [_6,_6];
(*_3) = core::ptr::addr_of!((*RET));
_33 = 69_i8;
_30 = [319373550924762078480660190945057701129_u128,220198904487499962666600470544796807389_u128,321815912595412725751943634427749991678_u128,254950200043935698672761852795595120436_u128,98448717205934308569686927952553798283_u128,72647146665349260948854247087853292266_u128];
_41 = core::ptr::addr_of!((*RET));
(*RET) = Field::<usize>(Variant(_2, 1), 1) - Field::<usize>(Variant(_2, 1), 1);
(*_4) = _16 * _16;
(*RET) = _37 as usize;
_42.1 = [28468576342898070379899129123338698712_u128,265485728123108076196877649891786893350_u128,214575596893629440157593309132198653513_u128,223018126896965448937886179073189645236_u128,41628198348099107333348155024652083453_u128,258464270683983894331507445417573922766_u128];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_3) = core::ptr::addr_of!(_38);
(*_4) = -_16;
(*_4) = _16 - _16;
(*_3) = Move(_41);
(*_4) = _16;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_14.1 = [2867022689969317110987689820627716033_u128,66185085388784690864410823795587708863_u128,61895540052036797412185260863588283059_u128,58250898634397785216159922987776883856_u128,112668054695680721431519148839937924540_u128];
match _33 {
0 => bb12,
1 => bb4,
2 => bb29,
3 => bb30,
69 => bb32,
_ => bb31
}
}
bb29 = {
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _5 as usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_24 = (-85518781735048623556943036658987368728_i128);
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 0_usize };
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
match _24 {
254763585185889839906431570772780842728 => bb8,
_ => bb7
}
}
bb30 = {
(*_18) = [(-7694901404219952187_i64),1297998594407023530_i64];
(*_4) = 928427704_i32 as f64;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
(*RET) = Field::<usize>(Variant(_2, 1), 1) & Field::<usize>(Variant(_2, 1), 1);
(*_4) = _16 * _16;
(*_4) = -_16;
(*RET) = Field::<usize>(Variant(_2, 1), 1) | Field::<usize>(Variant(_2, 1), 1);
(*_18) = [8898294256205568588_i64,(-1369679175549971301_i64)];
(*_4) = -_16;
_24 = (-135552975922953633445503035408715819852_i128) & 74746828846973376006209127926454390847_i128;
(*_4) = -_16;
(*_18) = [1474242179697440535_i64,(-846828315351297229_i64)];
_11 = !_15;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_49 = -_26;
(*RET) = Field::<usize>(Variant(_2, 1), 1) >> _5;
(*_18) = [(-6754024256999642270_i64),4649696996118852348_i64];
match _7.2 {
0 => bb14,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb26,
11443 => bb28,
_ => bb27
}
}
bb31 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb32 = {
_41 = core::ptr::addr_of!((*RET));
_32 = _5 | _5;
(*_41) = _38 ^ _38;
(*_41) = (*_4) as usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_42.1 = _30;
_6 = !1247626603_u32;
_5 = _32;
_39 = _49 + _49;
(*_41) = !_38;
(*_4) = _16 + _16;
(*_3) = core::ptr::addr_of!((*_41));
(*_41) = !_38;
_41 = core::ptr::addr_of!((*_41));
(*_3) = core::ptr::addr_of!((*_41));
(*_4) = _16;
(*RET) = _38 * _38;
(*_4) = -_16;
(*_4) = _16 + _16;
(*_4) = _32 as f64;
match _33 {
0 => bb15,
1 => bb29,
2 => bb25,
3 => bb33,
69 => bb35,
_ => bb34
}
}
bb33 = {
_14.1 = [239314058136682829058618153901243015011_u128,289311638826614431101199910288557460563_u128,99229067879290877682794088951773781136_u128,197952172790162409347790455960451473041_u128,186501940519248061059826754361695227186_u128];
(*RET) = 13242077828781098131_usize;
(*RET) = 4_usize << _6;
(*_4) = -_16;
(*RET) = 5_usize;
_15 = (*_4) != (*_4);
(*_18) = [718920382768500318_i64,7556351282731927408_i64];
(*_4) = -_16;
_9 = -_16;
(*_4) = _16 - _16;
(*_18) = [5209694812337029311_i64,(-8428909366433842712_i64)];
(*RET) = !18318325918657224612_usize;
(*_4) = _16;
(*RET) = 3_usize;
_26 = _8;
(*_4) = _16 + _16;
_8 = _26;
(*_3) = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_7.2 = 11443_i16;
(*_18) = [(-290657760362600975_i64),4818107213583910136_i64];
Goto(bb10)
}
bb34 = {
(*_18) = [(-7694901404219952187_i64),1297998594407023530_i64];
(*_4) = 928427704_i32 as f64;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
(*RET) = Field::<usize>(Variant(_2, 1), 1) & Field::<usize>(Variant(_2, 1), 1);
(*_4) = _16 * _16;
(*_4) = -_16;
(*RET) = Field::<usize>(Variant(_2, 1), 1) | Field::<usize>(Variant(_2, 1), 1);
(*_18) = [8898294256205568588_i64,(-1369679175549971301_i64)];
(*_4) = -_16;
_24 = (-135552975922953633445503035408715819852_i128) & 74746828846973376006209127926454390847_i128;
(*_4) = -_16;
(*_18) = [1474242179697440535_i64,(-846828315351297229_i64)];
_11 = !_15;
(*RET) = !Field::<usize>(Variant(_2, 1), 1);
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_49 = -_26;
(*RET) = Field::<usize>(Variant(_2, 1), 1) >> _5;
(*_18) = [(-6754024256999642270_i64),4649696996118852348_i64];
match _7.2 {
0 => bb14,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb26,
11443 => bb28,
_ => bb27
}
}
bb35 = {
(*RET) = _15 as usize;
(*_41) = _17 as usize;
(*_41) = _24 as usize;
(*_41) = _38 << _5;
(*_3) = core::ptr::addr_of!((*_41));
_30 = [158602366464799972588082319709330097820_u128,166527680970963154875055166331012520234_u128,45893894373600197273878010117443169954_u128,110036232356819816205421365415608535356_u128,93092115801688384631791891914319792341_u128,24467593655404080590214861895165687096_u128];
_5 = _32 << (*RET);
(*_41) = 33962_u16 as usize;
place!(Field::<usize>(Variant(_2, 1), 1)) = _38;
_51 = 101_u8 as i32;
_55.fld5 = _51 - _51;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_21 = &mut _51;
_14.1 = [102227798877445667373434785873256204878_u128,123824507109587711398745900539248460861_u128,122147325340119540242870610404951034649_u128,29940083505996029539921579266571382762_u128,717584078881325568954728948844745766_u128];
(*RET) = _38 - _38;
(*_3) = core::ptr::addr_of!((*_41));
_55.fld4 = _8 * _39;
_3 = core::ptr::addr_of_mut!(place!(Field::<*const usize>(Variant(_2, 1), 0)));
_1 = [_33,_33,_33,_33,_33,_33,_33];
(*_21) = 189_u8 as i32;
_31 = core::ptr::addr_of_mut!(_55.fld0);
_2 = Adt22::Variant0 { fld0: _5,fld1: _17 };
_33 = 16_i8 | (-38_i8);
_7.0 = &mut (*_21);
_57 = 28685718287018179890716007721049330677_u128;
_54 = Field::<char>(Variant(_2, 0), 1) as u64;
Goto(bb36)
}
bb36 = {
_7.1 = Field::<char>(Variant(_2, 0), 1);
_40 = (-5681473136132046163_i64);
(*_4) = _16 + _16;
_14.2 = &mut _57;
place!(Field::<char>(Variant(_2, 0), 1)) = _7.1;
(*_31) = Adt48::Variant3 { fld0: _24,fld1: Move(_3),fld2: 6080_u16 };
place!(Field::<u16>(Variant((*_31), 3), 2)) = _38 as u16;
match _7.2 {
0 => bb26,
1 => bb35,
2 => bb37,
3 => bb38,
4 => bb39,
5 => bb40,
6 => bb41,
11443 => bb43,
_ => bb42
}
}
bb37 = {
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 13784451795644834750_usize;
_7.1 = '\u{3e763}';
_6 = 177_u8 as u32;
(*_4) = _8 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 57078_u16 as f64;
(*RET) = !1699250695581916208_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = !1_usize;
_1 = [80_i8,94_i8,(-107_i8),(-28_i8),124_i8,8_i8,125_i8];
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 149_u8 as f64;
(*RET) = _11 as usize;
_17 = _7.1;
(*RET) = 6_usize * 6_usize;
(*RET) = 4626040888299866634_usize >> _7.2;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 165185308414627707603064705498487597960_i128 as f64;
_14.0 = [7905584152773944739_i64,1552234828642392793_i64];
Goto(bb5)
}
bb38 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb39 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_9 = 13992852134308336326_u64 as f64;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 7899121447099896165_usize & 14541414676073709482_usize;
(*RET) = 7929986877315095878_usize;
(*RET) = !6_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_11 = true;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = _5 as f32;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_4) = 5_u8 as f64;
(*RET) = 7902684732526400913_usize << _6;
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Goto(bb4)
}
bb40 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb41 = {
(*_18) = [4289082507770162061_i64,(-1183504578506993144_i64)];
(*_4) = _5 as f64;
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = _16 - _16;
place!(Field::<usize>(Variant(_2, 1), 1)) = 23558020092869729516613727834772295079_u128 as usize;
_1 = [74_i8,37_i8,(-75_i8),(-73_i8),53_i8,113_i8,66_i8];
(*RET) = 5_usize + 6_usize;
(*_4) = _16 * _16;
(*_4) = (-61_i8) as f64;
(*RET) = 10775180495414966827_usize >> _7.2;
(*RET) = !2223682988010111493_usize;
(*RET) = 0_usize;
(*_4) = -_16;
(*RET) = 4_usize;
(*_4) = -_16;
(*_4) = _16 * _16;
(*RET) = 6_usize;
(*_4) = _16;
(*_4) = _16 * _16;
(*_4) = -_16;
_7.1 = _17;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_16;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_18) = [3739376633070700344_i64,2706753521791367251_i64];
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = -_16;
Goto(bb15)
}
bb42 = {
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _5 as usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_24 = (-85518781735048623556943036658987368728_i128);
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 0_usize };
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
match _24 {
254763585185889839906431570772780842728 => bb8,
_ => bb7
}
}
bb43 = {
(*_4) = _16 - _16;
_42.2 = _17;
_25 = [_32,_32,_5,Field::<isize>(Variant(_2, 0), 0),_5,_5,_5];
match _7.2 {
0 => bb5,
1 => bb44,
2 => bb45,
3 => bb46,
4 => bb47,
5 => bb48,
6 => bb49,
11443 => bb51,
_ => bb50
}
}
bb44 = {
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _5 as usize;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_24 = (-85518781735048623556943036658987368728_i128);
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 0_usize };
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
match _24 {
254763585185889839906431570772780842728 => bb8,
_ => bb7
}
}
bb45 = {
_16 = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = 5228458573976492455_i64 as f64;
_11 = (*_4) < (*_4);
_5 = !9223372036854775807_isize;
_8 = _7.2 as f32;
_7.1 = _17;
(*RET) = !12912089188654647159_usize;
(*RET) = !6923389026260143121_usize;
RET = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
Goto(bb6)
}
bb46 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb47 = {
_14.1 = [239314058136682829058618153901243015011_u128,289311638826614431101199910288557460563_u128,99229067879290877682794088951773781136_u128,197952172790162409347790455960451473041_u128,186501940519248061059826754361695227186_u128];
(*RET) = 13242077828781098131_usize;
(*RET) = 4_usize << _6;
(*_4) = -_16;
(*RET) = 5_usize;
_15 = (*_4) != (*_4);
(*_18) = [718920382768500318_i64,7556351282731927408_i64];
(*_4) = -_16;
_9 = -_16;
(*_4) = _16 - _16;
(*_18) = [5209694812337029311_i64,(-8428909366433842712_i64)];
(*RET) = !18318325918657224612_usize;
(*_4) = _16;
(*RET) = 3_usize;
_26 = _8;
(*_4) = _16 + _16;
_8 = _26;
(*_3) = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_7.2 = 11443_i16;
(*_18) = [(-290657760362600975_i64),4818107213583910136_i64];
Goto(bb10)
}
bb48 = {
_18 = core::ptr::addr_of_mut!(_14.0);
_15 = _11;
_6 = _7.2 as u32;
(*_18) = [4095711069535553170_i64,(-5112594251563332001_i64)];
_11 = _15;
(*_18) = [(-8640504939896989745_i64),(-4347076551418624133_i64)];
(*_18) = [(-9203824863977020102_i64),2606994901228273821_i64];
_16 = -_9;
_5 = !9223372036854775807_isize;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = _17 as usize;
_15 = !_11;
(*RET) = 0_usize & 4_usize;
(*_18) = [5968769040722834547_i64,(-787870128598203566_i64)];
(*RET) = 2393906703850611942_usize << _7.2;
_2 = Adt22::Variant1 { fld0: Move((*_3)),fld1: 6_usize };
_9 = _16 * _16;
(*_18) = [(-4017728228241370495_i64),1350602227154885479_i64];
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*RET) = 481879652158064332_usize >> _7.2;
_18 = core::ptr::addr_of_mut!((*_18));
(*_4) = -_16;
(*_18) = [(-555121744400751065_i64),3081973290570641252_i64];
(*RET) = !4235110578179002035_usize;
match _24 {
254763585185889839906431570772780842728 => bb9,
_ => bb7
}
}
bb49 = {
(*_18) = [4289082507770162061_i64,(-1183504578506993144_i64)];
(*_4) = _5 as f64;
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = _16 - _16;
place!(Field::<usize>(Variant(_2, 1), 1)) = 23558020092869729516613727834772295079_u128 as usize;
_1 = [74_i8,37_i8,(-75_i8),(-73_i8),53_i8,113_i8,66_i8];
(*RET) = 5_usize + 6_usize;
(*_4) = _16 * _16;
(*_4) = (-61_i8) as f64;
(*RET) = 10775180495414966827_usize >> _7.2;
(*RET) = !2223682988010111493_usize;
(*RET) = 0_usize;
(*_4) = -_16;
(*RET) = 4_usize;
(*_4) = -_16;
(*_4) = _16 * _16;
(*RET) = 6_usize;
(*_4) = _16;
(*_4) = _16 * _16;
(*_4) = -_16;
_7.1 = _17;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_25 = [_5,_5,_5,_5,_5,_5,_5];
(*_4) = -_16;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_18) = [3739376633070700344_i64,2706753521791367251_i64];
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = -_16;
Goto(bb15)
}
bb50 = {
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!((*RET));
_9 = 13992852134308336326_u64 as f64;
(*RET) = !2_usize;
(*_3) = core::ptr::addr_of!((*RET));
(*RET) = 7899121447099896165_usize & 14541414676073709482_usize;
(*RET) = 7929986877315095878_usize;
(*RET) = !6_usize;
(*_3) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_11 = true;
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = _5 as f32;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!((*RET));
(*_4) = _5 as f64;
(*_4) = 5_u8 as f64;
(*RET) = 7902684732526400913_usize << _6;
(*_4) = _5 as f64;
(*_3) = core::ptr::addr_of!((*RET));
(*_3) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Goto(bb4)
}
bb51 = {
_30 = _42.1;
_46 = _39 - _39;
_24 = Field::<i128>(Variant((*_31), 3), 0) * Field::<i128>(Variant((*_31), 3), 0);
_13 = &mut _55.fld5;
(*_13) = !(-1657219973_i32);
place!(Field::<*mut *const usize>(Variant((*_31), 3), 1)) = core::ptr::addr_of_mut!(_41);
place!(Field::<*mut *const usize>(Variant((*_31), 3), 1)) = core::ptr::addr_of_mut!(_41);
place!(Field::<isize>(Variant(_2, 0), 0)) = _5;
place!(Field::<i128>(Variant((*_31), 3), 0)) = -_24;
place!(Field::<*mut *const usize>(Variant((*_31), 3), 1)) = core::ptr::addr_of_mut!(RET);
(*_31) = Adt48::Variant0 { fld0: _24,fld1: _7.1,fld2: _54,fld3: _33 };
place!(Field::<i128>(Variant((*_31), 0), 0)) = !_24;
_7.0 = &mut (*_13);
_54 = Field::<u64>(Variant((*_31), 0), 2) + Field::<u64>(Variant((*_31), 0), 2);
place!(Field::<i128>(Variant((*_31), 0), 0)) = -_24;
_49 = _46;
_25 = [_32,_5,_32,_5,_32,_5,Field::<isize>(Variant(_2, 0), 0)];
_32 = Field::<isize>(Variant(_2, 0), 0) ^ _5;
place!(Field::<i128>(Variant((*_31), 0), 0)) = _24 >> Field::<u64>(Variant((*_31), 0), 2);
Goto(bb52)
}
bb52 = {
Call(_67 = dump_var(Move(_17), Move(_1), Move(_57), Move(_6)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_67 = dump_var(Move(_37), Move(_33), Move(_54), Move(_47)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_67 = dump_var(Move(_15), _68, _68, _68), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: Adt22,mut _2: *mut *const usize,mut _3: char) -> *const usize {
mir! {
type RET = *const usize;
let _4: (Adt22, *mut (i128, &'static mut u128));
let _5: bool;
let _6: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _7: i8;
let _8: u128;
let _9: &'static mut (i128, &'static mut u128);
let _10: *const [u64; 4];
let _11: &'static mut (i128, &'static mut u128);
let _12: isize;
let _13: *const f64;
let _14: f64;
let _15: *const &'static mut (i128, &'static mut u128);
let _16: char;
let _17: isize;
let _18: &'static &'static u32;
let _19: i64;
let _20: (i128, &'static mut u128);
let _21: i64;
let _22: *mut *const usize;
let _23: *const f64;
let _24: f32;
let _25: isize;
let _26: isize;
let _27: *mut (i128, &'static mut u128);
let _28: [isize; 5];
let _29: *const &'static mut (i32, *const usize);
let _30: [u8; 4];
let _31: i8;
let _32: u8;
let _33: f64;
let _34: (i32, *const usize);
let _35: &'static &'static Adt25;
let _36: *const &'static mut (i32, *const usize);
let _37: i64;
let _38: bool;
let _39: f32;
let _40: f64;
let _41: f32;
let _42: Adt48;
let _43: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _44: *const &'static mut (i128, &'static mut u128);
let _45: f64;
let _46: *const &'static mut (i128, &'static mut u128);
let _47: ();
let _48: ();
{
_3 = Field::<char>(Variant(_1, 0), 1);
Call(place!(Field::<isize>(Variant(_1, 0), 0)) = fn4(Move(_2), Field::<char>(Variant(_1, 0), 1), Field::<char>(Variant(_1, 0), 1), _3, _3, _3, _3, _3, Field::<char>(Variant(_1, 0), 1), _3, Field::<char>(Variant(_1, 0), 1), _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_3 = Field::<char>(Variant(_1, 0), 1);
_4.0 = Move(_1);
_3 = Field::<char>(Variant(_4.0, 0), 1);
_1 = Move(_4.0);
_2 = core::ptr::addr_of_mut!(RET);
_2 = core::ptr::addr_of_mut!((*_2));
_4.0 = Move(_1);
_6.1 = _3;
_6.2 = (-756_i16);
_1 = Move(_4.0);
_4.0 = Move(_1);
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
_6.1 = _3;
place!(Field::<isize>(Variant(_4.0, 0), 0)) = (-9223372036854775808_isize) >> _6.2;
_1 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_4.0, 0), 0),fld1: _6.1 };
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
match _6.2 {
0 => bb2,
1 => bb3,
340282366920938463463374607431768210700 => bb5,
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
_5 = Field::<char>(Variant(_4.0, 0), 1) != _6.1;
place!(Field::<char>(Variant(_4.0, 0), 1)) = _3;
_5 = Field::<char>(Variant(_1, 0), 1) > Field::<char>(Variant(_4.0, 0), 1);
_7 = (-38_i8) - (-116_i8);
place!(Field::<char>(Variant(_4.0, 0), 1)) = _3;
place!(Field::<isize>(Variant(_1, 0), 0)) = -Field::<isize>(Variant(_4.0, 0), 0);
_4.0 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_1, 0), 0),fld1: _6.1 };
_4.0 = Move(_1);
_7 = _6.2 as i8;
place!(Field::<char>(Variant(_4.0, 0), 1)) = _6.1;
_2 = core::ptr::addr_of_mut!((*_2));
_1 = Move(_4.0);
_8 = _5 as u128;
_4.0 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_1, 0), 0),fld1: _6.1 };
_2 = core::ptr::addr_of_mut!((*_2));
place!(Field::<isize>(Variant(_1, 0), 0)) = Field::<isize>(Variant(_4.0, 0), 0) + Field::<isize>(Variant(_4.0, 0), 0);
_3 = Field::<char>(Variant(_4.0, 0), 1);
_6.2 = _5 as i16;
Goto(bb6)
}
bb6 = {
_3 = Field::<char>(Variant(_1, 0), 1);
_6.1 = _3;
_6.1 = Field::<char>(Variant(_1, 0), 1);
place!(Field::<char>(Variant(_1, 0), 1)) = _6.1;
_6.2 = (-27273_i16) - 23359_i16;
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_7 = -(-46_i8);
place!(Field::<isize>(Variant(_1, 0), 0)) = Field::<isize>(Variant(_4.0, 0), 0) & Field::<isize>(Variant(_4.0, 0), 0);
_5 = Field::<isize>(Variant(_1, 0), 0) < Field::<isize>(Variant(_4.0, 0), 0);
_6.1 = _3;
place!(Field::<isize>(Variant(_1, 0), 0)) = Field::<isize>(Variant(_4.0, 0), 0);
Goto(bb7)
}
bb7 = {
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_1 = Move(_4.0);
_2 = core::ptr::addr_of_mut!((*_2));
_6.1 = _3;
place!(Field::<char>(Variant(_1, 0), 1)) = _6.1;
_4.0 = Move(_1);
_1 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_4.0, 0), 0),fld1: _6.1 };
_1 = Move(_4.0);
_6.1 = Field::<char>(Variant(_1, 0), 1);
_1 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _3 };
_8 = 90350938462333907508524156926353808464_u128 + 195598602503619405653719514258925848603_u128;
_5 = false;
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_5 = _6.2 == _6.2;
_6.1 = _3;
_1 = Adt22::Variant0 { fld0: 56_isize,fld1: _6.1 };
_2 = core::ptr::addr_of_mut!((*_2));
_1 = Adt22::Variant0 { fld0: (-40_isize),fld1: _3 };
_8 = 110_isize as u128;
_4.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _6.1 };
place!(Field::<isize>(Variant(_4.0, 0), 0)) = 9223372036854775807_isize;
place!(Field::<isize>(Variant(_1, 0), 0)) = 13141936313634015096_u64 as isize;
_12 = Field::<isize>(Variant(_4.0, 0), 0);
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
place!(Field::<char>(Variant(_4.0, 0), 1)) = _3;
_4.0 = Move(_1);
_12 = 2870437156349911108_u64 as isize;
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
place!(Field::<isize>(Variant(_4.0, 0), 0)) = _12 * _12;
_8 = 39338157024309561523249627080613731802_u128 >> _6.2;
_12 = Field::<isize>(Variant(_4.0, 0), 0);
_13 = core::ptr::addr_of!(_14);
(*_13) = _12 as f64;
(*_13) = _6.2 as f64;
(*_13) = _8 as f64;
(*_13) = 1053085417_u32 as f64;
(*_13) = _8 as f64;
(*_13) = 312320106_i32 as f64;
(*_13) = 12_u8 as f64;
(*_13) = 3650759575_u32 as f64;
(*_13) = _7 as f64;
(*_13) = _8 as f64;
_14 = 5103210179769529290_u64 as f64;
_1 = Move(_4.0);
(*_13) = 8450842321384764365_usize as f64;
_6.1 = Field::<char>(Variant(_1, 0), 1);
(*_13) = Field::<isize>(Variant(_1, 0), 0) as f64;
_4.0 = Move(_1);
_14 = _8 as f64;
_14 = 13147583674484043307_usize as f64;
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
place!(Field::<isize>(Variant(_4.0, 0), 0)) = (-474361644_i32) as isize;
Goto(bb10)
}
bb10 = {
(*_13) = 5760174370838435870_i64 as f64;
(*_13) = 3499390034_u32 as f64;
(*_13) = 3919166741_u32 as f64;
(*_13) = 740201281_i32 as f64;
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
(*_13) = 6073992542008122171_u64 as f64;
(*_13) = _7 as f64;
(*_13) = _7 as f64;
_21 = (-2202164308823884024_i64) + (-3765111028158275350_i64);
(*_13) = Field::<isize>(Variant(_4.0, 0), 0) as f64;
place!(Field::<isize>(Variant(_4.0, 0), 0)) = _12 >> _8;
_17 = Field::<isize>(Variant(_4.0, 0), 0);
_1 = Adt22::Variant0 { fld0: _17,fld1: _3 };
(*_13) = _21 as f64;
_16 = _3;
_20.0 = 38558004873876945841399821595429819736_i128;
_22 = core::ptr::addr_of_mut!((*_2));
(*_13) = 18345977488994999263_usize as f64;
_17 = _12;
(*_13) = 184_u8 as f64;
_1 = Move(_4.0);
_3 = Field::<char>(Variant(_1, 0), 1);
match _20.0 {
0 => bb11,
1 => bb12,
38558004873876945841399821595429819736 => bb14,
_ => bb13
}
}
bb11 = {
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
place!(Field::<isize>(Variant(_4.0, 0), 0)) = _12 * _12;
_8 = 39338157024309561523249627080613731802_u128 >> _6.2;
_12 = Field::<isize>(Variant(_4.0, 0), 0);
_13 = core::ptr::addr_of!(_14);
(*_13) = _12 as f64;
(*_13) = _6.2 as f64;
(*_13) = _8 as f64;
(*_13) = 1053085417_u32 as f64;
(*_13) = _8 as f64;
(*_13) = 312320106_i32 as f64;
(*_13) = 12_u8 as f64;
(*_13) = 3650759575_u32 as f64;
(*_13) = _7 as f64;
(*_13) = _8 as f64;
_14 = 5103210179769529290_u64 as f64;
_1 = Move(_4.0);
(*_13) = 8450842321384764365_usize as f64;
_6.1 = Field::<char>(Variant(_1, 0), 1);
(*_13) = Field::<isize>(Variant(_1, 0), 0) as f64;
_4.0 = Move(_1);
_14 = _8 as f64;
_14 = 13147583674484043307_usize as f64;
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
place!(Field::<isize>(Variant(_4.0, 0), 0)) = (-474361644_i32) as isize;
Goto(bb10)
}
bb12 = {
Goto(bb9)
}
bb13 = {
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_3 = Field::<char>(Variant(_1, 0), 1);
_4.0 = Move(_1);
_3 = Field::<char>(Variant(_4.0, 0), 1);
_1 = Move(_4.0);
_2 = core::ptr::addr_of_mut!(RET);
_2 = core::ptr::addr_of_mut!((*_2));
_4.0 = Move(_1);
_6.1 = _3;
_6.2 = (-756_i16);
_1 = Move(_4.0);
_4.0 = Move(_1);
_6.1 = Field::<char>(Variant(_4.0, 0), 1);
_6.1 = _3;
place!(Field::<isize>(Variant(_4.0, 0), 0)) = (-9223372036854775808_isize) >> _6.2;
_1 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_4.0, 0), 0),fld1: _6.1 };
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
match _6.2 {
0 => bb2,
1 => bb3,
340282366920938463463374607431768210700 => bb5,
_ => bb4
}
}
bb14 = {
_5 = !true;
(*_13) = 1323860170_i32 as f64;
(*_13) = 971766004_i32 as f64;
(*_13) = _21 as f64;
_8 = 54072535981771452354929125306913298907_u128 << Field::<isize>(Variant(_1, 0), 0);
_7 = (-63_i8) << Field::<isize>(Variant(_1, 0), 0);
(*_13) = _8 as f64;
_2 = core::ptr::addr_of_mut!((*_2));
_23 = core::ptr::addr_of!((*_13));
(*_23) = _7 as f64;
_2 = core::ptr::addr_of_mut!((*_22));
place!(Field::<isize>(Variant(_1, 0), 0)) = _17;
(*_23) = Field::<isize>(Variant(_1, 0), 0) as f64;
_2 = core::ptr::addr_of_mut!((*_2));
(*_13) = _20.0 as f64;
_4.0 = Adt22::Variant0 { fld0: _17,fld1: _6.1 };
(*_13) = 4678_u16 as f64;
_7 = 15602185366429508010_u64 as i8;
_12 = !Field::<isize>(Variant(_4.0, 0), 0);
_23 = core::ptr::addr_of!((*_13));
(*_23) = _8 as f64;
_4.0 = Move(_1);
(*_23) = 37319_u16 as f64;
_3 = _6.1;
Goto(bb15)
}
bb15 = {
_14 = _21 as f64;
(*_13) = 3449612523_u32 as f64;
_8 = _20.0 as u128;
_8 = 213943072822515397193312942450299706395_u128;
_20.0 = 115145295391980003028647427383852808632_i128 >> _6.2;
(*_13) = Field::<isize>(Variant(_4.0, 0), 0) as f64;
_20.0 = (-68794265971447764270365353194684950584_i128) + (-56043174853029977386661266336151815051_i128);
(*_13) = 3362709881132964739_u64 as f64;
(*_13) = 7713777332094117832_usize as f64;
(*_13) = 6675799518848549983_u64 as f64;
_17 = Field::<isize>(Variant(_4.0, 0), 0) >> _12;
_19 = -_21;
_14 = 680449563_i32 as f64;
(*_13) = 163_u8 as f64;
Goto(bb16)
}
bb16 = {
_17 = _5 as isize;
_12 = 3313990102_u32 as isize;
(*_13) = _7 as f64;
(*_13) = 121_u8 as f64;
_1 = Move(_4.0);
_26 = _12 * Field::<isize>(Variant(_1, 0), 0);
_25 = _26 | _12;
(*_13) = 644928950_i32 as f64;
(*_13) = 617160825_u32 as f64;
_13 = Move(_23);
_1 = Adt22::Variant0 { fld0: _26,fld1: _16 };
_24 = 12_u8 as f32;
_2 = core::ptr::addr_of_mut!((*_22));
_7 = (-66_i8) & (-31_i8);
_4.0 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_1, 0), 0),fld1: Field::<char>(Variant(_1, 0), 1) };
_7 = 99_i8 - 118_i8;
_2 = core::ptr::addr_of_mut!((*_2));
_30 = [213_u8,100_u8,225_u8,239_u8];
_20.1 = &mut _8;
_5 = true & true;
Goto(bb17)
}
bb17 = {
place!(Field::<isize>(Variant(_1, 0), 0)) = _25 >> _12;
_23 = core::ptr::addr_of!(_14);
_7 = !28_i8;
(*_23) = 5061624408877315108_u64 as f64;
_9 = &mut _20;
(*_23) = 3889082064_u32 as f64;
_31 = _6.2 as i8;
_6.2 = -16450_i16;
(*_23) = 176841907926022718901824655934344760944_u128 as f64;
(*_9).0 = (-103826461281574482357885033538742078573_i128) >> Field::<isize>(Variant(_1, 0), 0);
(*_9).0 = 126007677816671502921424098759524664262_i128 * 119878139398963147097642180499205016654_i128;
(*_23) = _24 as f64;
(*_23) = _21 as f64;
(*_9).0 = 56233035134691368750325395169077447898_i128 - (-160513480105178245775819223539844504836_i128);
_15 = core::ptr::addr_of!(_9);
place!(Field::<char>(Variant(_4.0, 0), 1)) = _6.1;
Goto(bb18)
}
bb18 = {
(*_9).0 = 96493013977913253308243585563517680375_i128;
_11 = &mut (*_9);
(*_11).0 = 85375965207687170182879291547970684186_i128 >> _26;
_15 = core::ptr::addr_of!((*_15));
_1 = Move(_4.0);
_32 = _24 as u8;
(*_11).0 = 64840657005064584395720019587964629198_i128 & (-149291554799135006678336963471339065372_i128);
(*_11).0 = 145754268215407839898662806405507255698_i128 - 10066527129903321563640848992469148471_i128;
_23 = Move(_13);
(*_11).0 = 99591521072868314137782332381578577693_i128;
(*_11).0 = (-44513155566833381755601129711382061731_i128);
_33 = _14;
(*_15) = Move(_11);
_7 = _5 as i8;
_14 = _33 - _33;
_26 = !_25;
_4.0 = Adt22::Variant0 { fld0: _12,fld1: _6.1 };
_24 = _14 as f32;
_16 = Field::<char>(Variant(_1, 0), 1);
_7 = _31 * _31;
_14 = 3398500104_u32 as f64;
_37 = _21;
place!(Field::<isize>(Variant(_1, 0), 0)) = _6.2 as isize;
_25 = _24 as isize;
place!(Field::<isize>(Variant(_1, 0), 0)) = 60158_u16 as isize;
Goto(bb19)
}
bb19 = {
_4.0 = Adt22::Variant0 { fld0: _26,fld1: Field::<char>(Variant(_1, 0), 1) };
_34.0 = _7 as i32;
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_14 = -_33;
_32 = 209_u8 ^ 218_u8;
_19 = _21 ^ _37;
_16 = Field::<char>(Variant(_1, 0), 1);
_38 = _5;
place!(Field::<isize>(Variant(_1, 0), 0)) = _26 + _17;
_6.0 = &mut _34.0;
_4.0 = Adt22::Variant0 { fld0: _25,fld1: Field::<char>(Variant(_1, 0), 1) };
_21 = !_37;
_28 = [Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_4.0, 0), 0),Field::<isize>(Variant(_1, 0), 0),_17,Field::<isize>(Variant(_1, 0), 0)];
_21 = _37 * _37;
place!(Field::<isize>(Variant(_1, 0), 0)) = Field::<isize>(Variant(_4.0, 0), 0);
place!(Field::<char>(Variant(_1, 0), 1)) = _6.1;
_31 = _7 + _7;
place!(Field::<isize>(Variant(_4.0, 0), 0)) = _21 as isize;
_7 = _31 - _31;
_21 = 5_usize as i64;
_23 = core::ptr::addr_of!(_40);
(*_23) = _33 - _14;
_5 = !_38;
_13 = Move(_23);
Goto(bb20)
}
bb20 = {
_33 = -_14;
_31 = !_7;
_23 = core::ptr::addr_of!(_40);
(*_23) = 1746523729_u32 as f64;
_23 = core::ptr::addr_of!((*_23));
place!(Field::<char>(Variant(_4.0, 0), 1)) = Field::<char>(Variant(_1, 0), 1);
_26 = Field::<isize>(Variant(_1, 0), 0);
(*_23) = _14 - _14;
_2 = core::ptr::addr_of_mut!((*_2));
_7 = !_31;
_21 = Field::<isize>(Variant(_4.0, 0), 0) as i64;
(*_23) = _33 + _14;
_13 = core::ptr::addr_of!((*_23));
(*_23) = -_33;
_38 = _5 ^ _5;
(*_23) = _33 + _14;
(*_23) = 293602854884146729328033548446620622862_u128 as f64;
place!(Field::<isize>(Variant(_4.0, 0), 0)) = !_26;
_33 = -(*_23);
_42 = Adt48::Variant2 { fld0: _28,fld1: _6.2,fld2: (-16781380811010918057895686207719039900_i128),fld3: 14605301330646541724_usize };
(*_2) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_42, 2), 3)));
place!(Field::<usize>(Variant(_42, 2), 3)) = 4_usize;
(*_22) = core::ptr::addr_of!((*RET));
_24 = _32 as f32;
Goto(bb21)
}
bb21 = {
(*RET) = 6_usize * 3_usize;
Goto(bb22)
}
bb22 = {
(*_23) = _14 + _33;
Goto(bb23)
}
bb23 = {
(*_23) = _14 - _14;
_28 = [_25,Field::<isize>(Variant(_4.0, 0), 0),Field::<isize>(Variant(_4.0, 0), 0),_25,Field::<isize>(Variant(_1, 0), 0)];
_3 = Field::<char>(Variant(_4.0, 0), 1);
_43.1.1.1 = core::ptr::addr_of!((*RET));
_43.1.0 = [_6.1,Field::<char>(Variant(_1, 0), 1),Field::<char>(Variant(_1, 0), 1),Field::<char>(Variant(_1, 0), 1),_3,Field::<char>(Variant(_4.0, 0), 1),_16];
(*RET) = !0_usize;
_44 = Move(_15);
place!(Field::<isize>(Variant(_4.0, 0), 0)) = _31 as isize;
(*RET) = 6_usize >> _31;
(*_22) = core::ptr::addr_of!((*RET));
(*_23) = -_14;
Goto(bb24)
}
bb24 = {
Call(_47 = dump_var(Move(_26), Move(_25), Move(_32), Move(_31)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_47 = dump_var(Move(_30), Move(_19), Move(_5), Move(_12)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: *mut *const usize,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: char) -> isize {
mir! {
type RET = isize;
let _13: [u16; 4];
let _14: char;
let _15: i16;
let _16: f32;
let _17: &'static Adt25;
let _18: &'static mut (i128, &'static mut u128);
let _19: f64;
let _20: isize;
let _21: [u32; 5];
let _22: (i128, &'static mut u128);
let _23: &'static Adt25;
let _24: f64;
let _25: *mut [i64; 2];
let _26: f32;
let _27: f64;
let _28: *const &'static mut (i128, &'static mut u128);
let _29: *mut [i64; 2];
let _30: (u16, i32, char, u128);
let _31: isize;
let _32: &'static mut (i128, &'static mut u128);
let _33: u32;
let _34: f64;
let _35: isize;
let _36: &'static &'static u32;
let _37: bool;
let _38: f32;
let _39: i64;
let _40: char;
let _41: isize;
let _42: &'static u32;
let _43: isize;
let _44: f32;
let _45: *const u8;
let _46: i32;
let _47: isize;
let _48: *const u8;
let _49: *mut Adt48;
let _50: f32;
let _51: isize;
let _52: &'static mut (i128, &'static mut u128);
let _53: char;
let _54: *const f64;
let _55: i8;
let _56: char;
let _57: f64;
let _58: i16;
let _59: usize;
let _60: char;
let _61: [isize; 3];
let _62: [isize; 5];
let _63: u32;
let _64: *const [u64; 4];
let _65: bool;
let _66: &'static &'static Adt25;
let _67: bool;
let _68: *mut [i64; 2];
let _69: i64;
let _70: &'static mut (i128, &'static mut u128);
let _71: &'static Adt25;
let _72: i16;
let _73: *const (i32, *const usize);
let _74: Adt25;
let _75: *mut (i128, &'static mut u128);
let _76: &'static mut i32;
let _77: [u8; 4];
let _78: bool;
let _79: char;
let _80: [u128; 5];
let _81: isize;
let _82: u128;
let _83: f64;
let _84: u16;
let _85: *mut Adt48;
let _86: isize;
let _87: i32;
let _88: [char; 7];
let _89: i128;
let _90: &'static u32;
let _91: f64;
let _92: *const f64;
let _93: isize;
let _94: u16;
let _95: *const u8;
let _96: f64;
let _97: ();
let _98: ();
{
_10 = _12;
_8 = _12;
_2 = _9;
_5 = _2;
_13 = [20465_u16,207_u16,24384_u16,65473_u16];
_13 = [15902_u16,40764_u16,38102_u16,60149_u16];
_7 = _10;
_13 = [30516_u16,24894_u16,61886_u16,5689_u16];
RET = (-9223372036854775808_isize) * (-5_isize);
_3 = _11;
_2 = _6;
_12 = _7;
_4 = _12;
_6 = _3;
_2 = _7;
_4 = _6;
_14 = _12;
_14 = _7;
_11 = _3;
_12 = _2;
_6 = _8;
_7 = _3;
_7 = _6;
Call(_14 = fn5(Move(_1), _7, _4, _3, _12, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _14;
Goto(bb2)
}
bb2 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb3 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb4 = {
_4 = _8;
_7 = _9;
_21 = [1037529787_u32,2178892489_u32,2781909047_u32,194751873_u32,310593874_u32];
_2 = _5;
_22.0 = (-166826779550477446985492715072470383330_i128);
_19 = 245_u8 as f64;
_15 = (-20330_i16);
_13 = [29031_u16,29511_u16,64197_u16,841_u16];
_3 = _2;
_24 = -_19;
_21 = [2017102198_u32,3668466344_u32,3181571683_u32,1907525725_u32,2841281086_u32];
match _15 {
0 => bb5,
1 => bb6,
340282366920938463463374607431768191126 => bb8,
_ => bb7
}
}
bb5 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb6 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb7 = {
_9 = _14;
Goto(bb2)
}
bb8 = {
_4 = _3;
_19 = _24 * _24;
_24 = _15 as f64;
RET = 76_isize;
_16 = 317574751231608779917989961294710781144_u128 as f32;
_10 = _7;
match _15 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607431768191126 => bb14,
_ => bb13
}
}
bb9 = {
_9 = _14;
Goto(bb2)
}
bb10 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb11 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb12 = {
_4 = _8;
_7 = _9;
_21 = [1037529787_u32,2178892489_u32,2781909047_u32,194751873_u32,310593874_u32];
_2 = _5;
_22.0 = (-166826779550477446985492715072470383330_i128);
_19 = 245_u8 as f64;
_15 = (-20330_i16);
_13 = [29031_u16,29511_u16,64197_u16,841_u16];
_3 = _2;
_24 = -_19;
_21 = [2017102198_u32,3668466344_u32,3181571683_u32,1907525725_u32,2841281086_u32];
match _15 {
0 => bb5,
1 => bb6,
340282366920938463463374607431768191126 => bb8,
_ => bb7
}
}
bb13 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb14 = {
_22.0 = 35155935104924443279199980167789231599_i128;
_24 = 2_usize as f64;
_13 = [23090_u16,50042_u16,111_u16,64211_u16];
_22.0 = -138367646349481509740751584907711840723_i128;
_24 = _16 as f64;
_11 = _10;
_13 = [20530_u16,15206_u16,49588_u16,48510_u16];
_11 = _5;
_26 = _16;
_13 = [20517_u16,63014_u16,41478_u16,52932_u16];
_20 = !RET;
_8 = _5;
_4 = _3;
_21 = [1402475255_u32,231017275_u32,2232766890_u32,1213180531_u32,2185956976_u32];
_13 = [5923_u16,24120_u16,3068_u16,18418_u16];
_13 = [33018_u16,8779_u16,40269_u16,30215_u16];
_4 = _8;
match _15 {
0 => bb13,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb15,
340282366920938463463374607431768191126 => bb17,
_ => bb16
}
}
bb15 = {
_9 = _14;
Goto(bb2)
}
bb16 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb17 = {
_22.0 = (-5933932148479241461267009056722203081_i128) - 21690002572286619109504491544690334361_i128;
_3 = _14;
_4 = _14;
_15 = 25623_i16;
_15 = (-7101973862529000252_i64) as i16;
_31 = RET;
_21 = [959498348_u32,2576995657_u32,2610470538_u32,1697653327_u32,3509407662_u32];
_30.1 = _19 as i32;
_30.3 = 25353695652365475627457697643097250611_u128 * 224965100368015178086597480711049603797_u128;
_14 = _9;
_30.1 = 990447382_i32;
_27 = -_24;
match _31 {
0 => bb1,
1 => bb5,
2 => bb3,
76 => bb18,
_ => bb16
}
}
bb18 = {
_19 = _27 * _27;
_30 = (28309_u16, 916622722_i32, _14, 79606259205250182622789607073899103391_u128);
_26 = 1508269472_u32 as f32;
_21 = [3304545861_u32,1970650789_u32,2454024931_u32,1731798593_u32,4125898973_u32];
_7 = _5;
_31 = _20;
_9 = _12;
_30.0 = 2247208255595878177_u64 as u16;
_15 = 3_usize as i16;
_33 = _15 as u32;
_26 = _16 + _16;
_6 = _8;
_33 = 9110116_u32 ^ 2882602670_u32;
_19 = _24 + _24;
_16 = _26;
_11 = _9;
_13 = [_30.0,_30.0,_30.0,_30.0];
_6 = _7;
_5 = _7;
_26 = _16 + _16;
_10 = _6;
_21 = [_33,_33,_33,_33,_33];
match _30.3 {
0 => bb10,
79606259205250182622789607073899103391 => bb19,
_ => bb13
}
}
bb19 = {
_30.3 = !222885242098467769534734062769377198149_u128;
_16 = _26 - _26;
_15 = _20 as i16;
_37 = _16 <= _16;
_20 = RET * _31;
_5 = _7;
_37 = false;
_7 = _5;
_30.0 = _2 as u16;
_38 = _26 * _16;
_35 = _31;
_39 = 79_u8 as i64;
_4 = _14;
_43 = _20 + RET;
_8 = _5;
_33 = 2148671606_u32;
_10 = _4;
_31 = _20 >> _43;
match _30.1 {
0 => bb10,
1 => bb11,
2 => bb20,
3 => bb21,
916622722 => bb23,
_ => bb22
}
}
bb20 = {
_4 = _3;
_19 = _24 * _24;
_24 = _15 as f64;
RET = 76_isize;
_16 = 317574751231608779917989961294710781144_u128 as f32;
_10 = _7;
match _15 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607431768191126 => bb14,
_ => bb13
}
}
bb21 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb22 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb23 = {
_35 = _26 as isize;
_27 = _16 as f64;
_7 = _12;
_3 = _10;
_13 = [_30.0,_30.0,_30.0,_30.0];
_14 = _5;
_9 = _3;
_11 = _7;
_44 = -_26;
_19 = 158_u8 as f64;
_34 = _27;
_46 = _30.1 * _30.1;
_38 = -_44;
_13 = [_30.0,_30.0,_30.0,_30.0];
_14 = _11;
_30.1 = _46 | _46;
_41 = !_35;
_15 = (-21217_i16) | (-1620_i16);
_22.1 = &mut _30.3;
_31 = _43 + _35;
_47 = _31 ^ _35;
_20 = 7_i8 as isize;
Goto(bb24)
}
bb24 = {
_42 = &_33;
_8 = _10;
_7 = _12;
RET = _35;
_15 = (-19758_i16) - (-10709_i16);
Goto(bb25)
}
bb25 = {
_33 = 2685248838_u32 & 2563848904_u32;
_12 = _14;
_24 = _27;
_26 = _16;
_43 = _31;
_15 = _5 as i16;
_47 = !_41;
_8 = _4;
_26 = _16;
_40 = _5;
_14 = _2;
_18 = &mut _22;
(*_18).0 = 63_u8 as i128;
(*_18).0 = 118188173153184083090829088909805232433_i128;
(*_18).0 = (-105640501862606438107006914299951658291_i128);
_9 = _7;
_36 = &_42;
(*_18).0 = (-16756834126089735608040307980455257269_i128) & 128193066122930749866349107840713504529_i128;
_24 = _34 + _27;
_38 = _26 - _26;
(*_18).0 = (-9308072609691826839678862276769864074_i128) + 110963378692203868944046557796083744671_i128;
_10 = _11;
_28 = core::ptr::addr_of!(_18);
_5 = _40;
Goto(bb26)
}
bb26 = {
_26 = -_38;
Goto(bb27)
}
bb27 = {
_3 = _6;
(*_18).0 = _39 as i128;
_53 = _10;
(*_18).0 = -31083332606982770863644355400746198323_i128;
_35 = _31 - _31;
_8 = _2;
_54 = core::ptr::addr_of!(_27);
_52 = &mut (*_18);
_41 = _47 * _31;
_37 = (*_54) >= (*_54);
(*_28) = Move(_52);
(*_54) = _33 as f64;
_37 = (*_54) >= (*_54);
_8 = _14;
(*_54) = _24 + _19;
_34 = _27 + (*_54);
Goto(bb28)
}
bb28 = {
_56 = _4;
_34 = (*_54) - (*_54);
_3 = _9;
_51 = _35 * _43;
(*_54) = _33 as f64;
(*_54) = _34 + _34;
(*_54) = _34;
_47 = _35 - _51;
_19 = (*_54) - (*_54);
_44 = _26;
Goto(bb29)
}
bb29 = {
_14 = _56;
(*_54) = 17_i8 as f64;
(*_54) = _19;
_39 = 2270694170401776975_i64;
_8 = _12;
(*_54) = _19 - _19;
_19 = 8941929649493846515_u64 as f64;
_50 = _38 + _38;
_37 = (*_54) <= _27;
(*_54) = _24 - _34;
_26 = -_50;
_2 = _9;
_8 = _40;
(*_54) = _24;
(*_54) = _24 - _24;
_2 = _11;
(*_54) = _34;
(*_54) = _34;
_19 = (*_54) - (*_54);
_47 = -_41;
_15 = -(-16066_i16);
_41 = _35;
Goto(bb30)
}
bb30 = {
_53 = _5;
_54 = core::ptr::addr_of!((*_54));
(*_54) = -_19;
_16 = _50 - _26;
_59 = 12400634121886326876_usize;
(*_54) = _34;
_59 = 3277578551759939585_usize & 18343901254969716618_usize;
_7 = _10;
(*_54) = -_19;
_59 = 4_usize - 6_usize;
_56 = _7;
_57 = 7912704293276486562_u64 as f64;
Goto(bb31)
}
bb31 = {
_34 = (*_54);
_38 = -_44;
_3 = _53;
_50 = _16;
_62 = [_35,_35,_35,_43,_41];
_60 = _10;
_53 = _10;
(*_54) = _24;
_63 = _33 << _51;
_8 = _6;
(*_54) = -_34;
_19 = (*_54) - (*_54);
_61 = [_51,_35,RET];
_10 = _4;
Goto(bb32)
}
bb32 = {
_13 = [27344_u16,695_u16,34132_u16,60026_u16];
_11 = _10;
(*_54) = _34 - _34;
(*_54) = _19;
_37 = !true;
_69 = _39 | _39;
_6 = _11;
_33 = _63 * _63;
(*_54) = _16 as f64;
_51 = _41 | _35;
_58 = _15;
_60 = _3;
_50 = _16 - _16;
_69 = _39 * _39;
(*_54) = _24;
_16 = _50;
_65 = _37 | _37;
(*_54) = _34;
_6 = _5;
_20 = -_41;
match _39 {
2270694170401776975 => bb34,
_ => bb33
}
}
bb33 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb34 = {
(*_54) = -_24;
_31 = -_51;
_65 = _35 >= _31;
_72 = _15 << _35;
_27 = _19;
(*_54) = -_34;
_44 = _50 * _50;
_42 = &_63;
_44 = _33 as f32;
_63 = !_33;
_69 = _39;
_77 = [197_u8,222_u8,102_u8,151_u8];
(*_54) = _34 * _19;
(*_54) = _19 - _19;
(*_54) = _34 + _34;
_44 = 65_i8 as f32;
_16 = _38 - _50;
_12 = _10;
_67 = _72 < _72;
_10 = _8;
_20 = _31 * _41;
_62 = [_51,_20,_35,_20,_41];
_4 = _60;
match _69 {
0 => bb18,
1 => bb35,
2 => bb36,
3 => bb37,
4 => bb38,
5 => bb39,
6 => bb40,
2270694170401776975 => bb42,
_ => bb41
}
}
bb35 = {
_16 = 1744863831_u32 as f32;
RET = (-47_isize) << _15;
_5 = _9;
_8 = _14;
_7 = _2;
_16 = 58228_u16 as f32;
_8 = _14;
_4 = _8;
_8 = _12;
Goto(bb4)
}
bb36 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb37 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb38 = {
_26 = -_38;
Goto(bb27)
}
bb39 = {
_14 = _8;
_4 = _2;
_5 = _11;
_8 = _7;
_15 = 5231138200774007753_i64 as i16;
_2 = _12;
_2 = _11;
_2 = _4;
Goto(bb3)
}
bb40 = {
_4 = _8;
_7 = _9;
_21 = [1037529787_u32,2178892489_u32,2781909047_u32,194751873_u32,310593874_u32];
_2 = _5;
_22.0 = (-166826779550477446985492715072470383330_i128);
_19 = 245_u8 as f64;
_15 = (-20330_i16);
_13 = [29031_u16,29511_u16,64197_u16,841_u16];
_3 = _2;
_24 = -_19;
_21 = [2017102198_u32,3668466344_u32,3181571683_u32,1907525725_u32,2841281086_u32];
match _15 {
0 => bb5,
1 => bb6,
340282366920938463463374607431768191126 => bb8,
_ => bb7
}
}
bb41 = {
_9 = _14;
Goto(bb2)
}
bb42 = {
_63 = _33 - _33;
(*_54) = _19;
match _69 {
0 => bb29,
1 => bb23,
2 => bb9,
3 => bb11,
4 => bb5,
5 => bb26,
2270694170401776975 => bb43,
_ => bb28
}
}
bb43 = {
_81 = _41 | _20;
_55 = _69 as i8;
_2 = _40;
_16 = _50 - _50;
_55 = (-1_i8) << _51;
_9 = _40;
_44 = _50;
_83 = -_19;
_3 = _60;
_63 = 14799749922159492368_u64 as u32;
(*_54) = -_34;
_44 = _16 + _50;
_8 = _12;
_47 = _35;
Goto(bb44)
}
bb44 = {
_26 = -_16;
_83 = (*_54) - (*_54);
_72 = _58;
(*_54) = -_34;
_7 = _9;
(*_54) = _34 * _24;
_62 = [_20,RET,_51,_51,_20];
_78 = _16 <= _16;
(*_54) = _83 + _83;
(*_54) = _19 - _19;
(*_54) = -_34;
_46 = _78 as i32;
_6 = _53;
_54 = core::ptr::addr_of!((*_54));
_37 = _78;
_8 = _14;
(*_54) = 86_u8 as f64;
Call(_50 = core::intrinsics::transmute(_12), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
(*_54) = _19 - _24;
_2 = _4;
_34 = (*_54);
_14 = _5;
_56 = _4;
_86 = _81;
Goto(bb46)
}
bb46 = {
_72 = _15 | _58;
(*_54) = -_34;
Goto(bb47)
}
bb47 = {
_69 = !_39;
_88 = [_56,_4,_6,_56,_12,_9,_4];
_76 = &mut _46;
_81 = _16 as isize;
_2 = _9;
_42 = &_33;
(*_76) = (-1625400868_i32) & 1594090933_i32;
(*_76) = -685719654_i32;
_47 = _20 >> (*_42);
_79 = _7;
_16 = _59 as f32;
(*_54) = (*_76) as f64;
_2 = _6;
_20 = _31 << (*_42);
_89 = (-152981645917415947851788338416614618504_i128);
_14 = _56;
_65 = !_78;
_43 = -_81;
_72 = _58 << _81;
_82 = 81777779193064722045538213350964254271_u128 + 186181193748743608702288256803225162530_u128;
(*_54) = _44 as f64;
(*_54) = _55 as f64;
_12 = _53;
_2 = _60;
_21 = [(*_42),(*_42),(*_42),(*_42),(*_42)];
(*_54) = _34;
(*_54) = -_19;
_90 = Move(_42);
_26 = _44;
_81 = _43 & _43;
Goto(bb48)
}
bb48 = {
_84 = 48207_u16 | 16976_u16;
_27 = _19;
(*_76) = 934855410_i32;
(*_54) = _34;
_47 = _81 + _86;
_15 = _72 | _72;
_43 = _69 as isize;
_42 = &_33;
Goto(bb49)
}
bb49 = {
(*_76) = (-637919467_i32) + 262246806_i32;
(*_54) = _19 + _34;
_69 = _39;
_9 = _8;
(*_76) = 1358602850_i32 | (-2138809199_i32);
_59 = 0_usize;
(*_76) = 1478999431_i32 - (-1259763287_i32);
(*_76) = 382542826_i32;
(*_54) = _19 * _34;
_37 = !_78;
_13 = [_84,_84,_84,_84];
_54 = core::ptr::addr_of!(_91);
_15 = _77[_59] as i16;
_5 = _60;
_90 = &(*_42);
_77 = [242_u8,141_u8,72_u8,81_u8];
(*_76) = _59 as i32;
_8 = _60;
_93 = _62[_59] + _47;
(*_54) = -_19;
_7 = _88[_59];
Goto(bb50)
}
bb50 = {
Call(_97 = dump_var(Move(_14), Move(_88), Move(_6), Move(_59)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_97 = dump_var(Move(_31), Move(_41), Move(_81), Move(_55)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_97 = dump_var(Move(_82), Move(_63), Move(_3), Move(_4)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_97 = dump_var(Move(_84), Move(_40), Move(_20), Move(_7)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_97 = dump_var(Move(_69), Move(_30), Move(_33), Move(_47)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_97 = dump_var(Move(_12), Move(_46), Move(_79), Move(_61)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_97 = dump_var(Move(_43), _98, _98, _98), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *mut *const usize,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: char) -> char {
mir! {
type RET = char;
let _7: bool;
let _8: u64;
let _9: ((i128, &'static mut u128),);
let _10: (u16, i32, char, u128);
let _11: f32;
let _12: *mut *const usize;
let _13: u128;
let _14: [i16; 4];
let _15: f64;
let _16: Adt48;
let _17: i64;
let _18: [u128; 4];
let _19: *mut Adt48;
let _20: *mut Adt48;
let _21: f32;
let _22: isize;
let _23: *const *const [u64; 4];
let _24: *const [u64; 4];
let _25: i8;
let _26: [u64; 4];
let _27: *const f64;
let _28: i16;
let _29: u8;
let _30: i32;
let _31: *const [u64; 4];
let _32: isize;
let _33: *mut Adt48;
let _34: *mut (i128, &'static mut u128);
let _35: isize;
let _36: char;
let _37: *const *const [u64; 4];
let _38: (*mut [i64; 2],);
let _39: *const u8;
let _40: u8;
let _41: [i128; 4];
let _42: [u128; 5];
let _43: i16;
let _44: &'static u32;
let _45: &'static &'static u32;
let _46: f32;
let _47: f32;
let _48: char;
let _49: (i16, Adt59, &'static u32);
let _50: char;
let _51: i64;
let _52: u32;
let _53: *mut (&'static &'static u32, [u128; 6], char);
let _54: (i128, &'static mut u128);
let _55: char;
let _56: isize;
let _57: &'static mut u128;
let _58: [i32; 2];
let _59: &'static mut &'static mut (i128, &'static mut u128);
let _60: isize;
let _61: f64;
let _62: isize;
let _63: *mut *const usize;
let _64: [u8; 4];
let _65: [i8; 7];
let _66: i128;
let _67: Adt22;
let _68: [i16; 4];
let _69: &'static mut (i128, &'static mut u128);
let _70: isize;
let _71: *const u8;
let _72: *const [u64; 4];
let _73: (Adt38, *const usize, &'static mut i32);
let _74: *mut Adt48;
let _75: [i64; 2];
let _76: u16;
let _77: f64;
let _78: bool;
let _79: [u64; 4];
let _80: *mut (i128, &'static mut u128);
let _81: &'static Adt25;
let _82: f32;
let _83: (i16, Adt59, &'static u32);
let _84: i32;
let _85: ();
let _86: ();
{
_5 = _6;
_6 = _2;
_6 = _5;
_7 = _4 != _6;
_4 = _2;
_3 = _2;
_7 = false | true;
_6 = _2;
_3 = _2;
_8 = !4352830365533225045_u64;
_6 = _2;
_8 = 1919695159618899763_u64 << 8720113217826361205_i64;
_9.0.0 = 163399650977738736025906504699735417393_i128;
_6 = _4;
_2 = _3;
_7 = _9.0.0 > _9.0.0;
_10.2 = _3;
_6 = _2;
_10.2 = _6;
_10.1 = -(-221753419_i32);
_10.1 = 1998071473_i32 * 896400954_i32;
_11 = 51_u8 as f32;
_7 = true;
match _9.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
163399650977738736025906504699735417393 => bb6,
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
_2 = _5;
RET = _10.2;
_11 = _8 as f32;
_14 = [10539_i16,(-23063_i16),(-2093_i16),8612_i16];
_4 = _5;
_10.0 = 27460_u16;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
27460 => bb13,
_ => bb12
}
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
Return()
}
bb12 = {
Return()
}
bb13 = {
_10.3 = !121479254628528160622583650090225650229_u128;
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-7_i8) };
_10.1 = 9223372036854775807_isize as i32;
_8 = Field::<u64>(Variant(_16, 0), 2) * Field::<u64>(Variant(_16, 0), 2);
place!(Field::<i8>(Variant(_16, 0), 3)) = 102_i8 * (-99_i8);
_15 = Field::<i128>(Variant(_16, 0), 0) as f64;
place!(Field::<u64>(Variant(_16, 0), 2)) = _7 as u64;
_18 = [_10.3,_10.3,_10.3,_10.3];
_15 = 74369110_u32 as f64;
_10.3 = 78731800005111046354217370529551905611_u128 ^ 25446031779562098639688268305745710173_u128;
RET = Field::<char>(Variant(_16, 0), 1);
_10.1 = 1972947348_i32 * 1638009466_i32;
_17 = 5183235153482359937_i64;
_10.1 = 391515497_i32 >> Field::<i128>(Variant(_16, 0), 0);
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-107_i8) };
_21 = _11 - _11;
Goto(bb14)
}
bb14 = {
_9.0.0 = Field::<i128>(Variant(_16, 0), 0);
_3 = _10.2;
RET = _4;
_10.0 = 3891_u16;
_13 = _10.3 << Field::<u64>(Variant(_16, 0), 2);
_13 = _10.3 ^ _10.3;
place!(Field::<i8>(Variant(_16, 0), 3)) = 55_i8;
_20 = core::ptr::addr_of_mut!(_16);
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 >> Field::<u64>(Variant((*_20), 0), 2);
place!(Field::<i128>(Variant(_16, 0), 0)) = _9.0.0;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 - _8;
(*_20) = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-109_i8) };
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 & _8;
_13 = _10.3 ^ _10.3;
place!(Field::<i8>(Variant((*_20), 0), 3)) = (-44_i8) - 9_i8;
place!(Field::<i8>(Variant((*_20), 0), 3)) = _17 as i8;
_21 = _11 + _11;
_10 = (56434_u16, 759087200_i32, _3, _13);
place!(Field::<char>(Variant((*_20), 0), 1)) = RET;
_21 = 9223372036854775807_isize as f32;
_22 = _4 as isize;
place!(Field::<char>(Variant((*_20), 0), 1)) = RET;
_28 = 26424_i16 & 20239_i16;
Goto(bb15)
}
bb15 = {
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 >> Field::<u64>(Variant((*_20), 0), 2);
place!(Field::<u64>(Variant(_16, 0), 2)) = _10.1 as u64;
_6 = Field::<char>(Variant((*_20), 0), 1);
place!(Field::<char>(Variant((*_20), 0), 1)) = _2;
_22 = !85_isize;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 - _8;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0;
place!(Field::<i128>(Variant((*_20), 0), 0)) = -_9.0.0;
match _10.1 {
0 => bb16,
1 => bb17,
2 => bb18,
3 => bb19,
4 => bb20,
5 => bb21,
759087200 => bb23,
_ => bb22
}
}
bb16 = {
_9.0.0 = Field::<i128>(Variant(_16, 0), 0);
_3 = _10.2;
RET = _4;
_10.0 = 3891_u16;
_13 = _10.3 << Field::<u64>(Variant(_16, 0), 2);
_13 = _10.3 ^ _10.3;
place!(Field::<i8>(Variant(_16, 0), 3)) = 55_i8;
_20 = core::ptr::addr_of_mut!(_16);
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 >> Field::<u64>(Variant((*_20), 0), 2);
place!(Field::<i128>(Variant(_16, 0), 0)) = _9.0.0;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 - _8;
(*_20) = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-109_i8) };
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 & _8;
_13 = _10.3 ^ _10.3;
place!(Field::<i8>(Variant((*_20), 0), 3)) = (-44_i8) - 9_i8;
place!(Field::<i8>(Variant((*_20), 0), 3)) = _17 as i8;
_21 = _11 + _11;
_10 = (56434_u16, 759087200_i32, _3, _13);
place!(Field::<char>(Variant((*_20), 0), 1)) = RET;
_21 = 9223372036854775807_isize as f32;
_22 = _4 as isize;
place!(Field::<char>(Variant((*_20), 0), 1)) = RET;
_28 = 26424_i16 & 20239_i16;
Goto(bb15)
}
bb17 = {
_10.3 = !121479254628528160622583650090225650229_u128;
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-7_i8) };
_10.1 = 9223372036854775807_isize as i32;
_8 = Field::<u64>(Variant(_16, 0), 2) * Field::<u64>(Variant(_16, 0), 2);
place!(Field::<i8>(Variant(_16, 0), 3)) = 102_i8 * (-99_i8);
_15 = Field::<i128>(Variant(_16, 0), 0) as f64;
place!(Field::<u64>(Variant(_16, 0), 2)) = _7 as u64;
_18 = [_10.3,_10.3,_10.3,_10.3];
_15 = 74369110_u32 as f64;
_10.3 = 78731800005111046354217370529551905611_u128 ^ 25446031779562098639688268305745710173_u128;
RET = Field::<char>(Variant(_16, 0), 1);
_10.1 = 1972947348_i32 * 1638009466_i32;
_17 = 5183235153482359937_i64;
_10.1 = 391515497_i32 >> Field::<i128>(Variant(_16, 0), 0);
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-107_i8) };
_21 = _11 - _11;
Goto(bb14)
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
_2 = _5;
RET = _10.2;
_11 = _8 as f32;
_14 = [10539_i16,(-23063_i16),(-2093_i16),8612_i16];
_4 = _5;
_10.0 = 27460_u16;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
27460 => bb13,
_ => bb12
}
}
bb23 = {
place!(Field::<char>(Variant((*_20), 0), 1)) = _6;
place!(Field::<i128>(Variant((*_20), 0), 0)) = !_9.0.0;
_19 = core::ptr::addr_of_mut!((*_20));
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
_26 = [Field::<u64>(Variant(_16, 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_19), 0), 2),Field::<u64>(Variant((*_19), 0), 2)];
place!(Field::<u64>(Variant((*_19), 0), 2)) = !_8;
place!(Field::<char>(Variant((*_20), 0), 1)) = _3;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 - _9.0.0;
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _2,fld2: _8,fld3: 123_i8 };
place!(Field::<u64>(Variant((*_20), 0), 2)) = !_8;
place!(Field::<i8>(Variant((*_20), 0), 3)) = !22_i8;
_26 = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2)];
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8;
_29 = 157_u8 >> Field::<u64>(Variant((*_20), 0), 2);
place!(Field::<char>(Variant(_16, 0), 1)) = _6;
place!(Field::<char>(Variant(_16, 0), 1)) = _3;
_9.0.1 = &mut _13;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 - _9.0.0;
place!(Field::<i8>(Variant((*_20), 0), 3)) = _10.3 as i8;
place!(Field::<i128>(Variant((*_20), 0), 0)) = -_9.0.0;
place!(Field::<i8>(Variant((*_20), 0), 3)) = 85_i8 - (-54_i8);
place!(Field::<i8>(Variant((*_20), 0), 3)) = 81_i8;
_32 = -_22;
place!(Field::<i8>(Variant((*_20), 0), 3)) = 54_i8 * (-12_i8);
place!(Field::<char>(Variant((*_20), 0), 1)) = RET;
_10.1 = 1988567832_i32 - 6076174_i32;
place!(Field::<i8>(Variant((*_20), 0), 3)) = -92_i8;
match _17 {
5183235153482359937 => bb25,
_ => bb24
}
}
bb24 = {
_10.3 = !121479254628528160622583650090225650229_u128;
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-7_i8) };
_10.1 = 9223372036854775807_isize as i32;
_8 = Field::<u64>(Variant(_16, 0), 2) * Field::<u64>(Variant(_16, 0), 2);
place!(Field::<i8>(Variant(_16, 0), 3)) = 102_i8 * (-99_i8);
_15 = Field::<i128>(Variant(_16, 0), 0) as f64;
place!(Field::<u64>(Variant(_16, 0), 2)) = _7 as u64;
_18 = [_10.3,_10.3,_10.3,_10.3];
_15 = 74369110_u32 as f64;
_10.3 = 78731800005111046354217370529551905611_u128 ^ 25446031779562098639688268305745710173_u128;
RET = Field::<char>(Variant(_16, 0), 1);
_10.1 = 1972947348_i32 * 1638009466_i32;
_17 = 5183235153482359937_i64;
_10.1 = 391515497_i32 >> Field::<i128>(Variant(_16, 0), 0);
_16 = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: (-107_i8) };
_21 = _11 - _11;
Goto(bb14)
}
bb25 = {
place!(Field::<i8>(Variant((*_20), 0), 3)) = (-115_i8) << Field::<u64>(Variant((*_20), 0), 2);
place!(Field::<char>(Variant((*_20), 0), 1)) = _2;
place!(Field::<i8>(Variant((*_20), 0), 3)) = (-30_i8);
place!(Field::<u64>(Variant((*_20), 0), 2)) = !_8;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 << Field::<u64>(Variant((*_20), 0), 2);
_22 = -_32;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 ^ _9.0.0;
Goto(bb26)
}
bb26 = {
place!(Field::<char>(Variant((*_20), 0), 1)) = _2;
place!(Field::<char>(Variant((*_20), 0), 1)) = _10.2;
place!(Field::<char>(Variant((*_20), 0), 1)) = _3;
_25 = Field::<i8>(Variant((*_20), 0), 3);
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
place!(Field::<char>(Variant((*_20), 0), 1)) = _3;
_5 = Field::<char>(Variant((*_20), 0), 1);
place!(Field::<i8>(Variant((*_20), 0), 3)) = _25 | _25;
_33 = core::ptr::addr_of_mut!((*_20));
place!(Field::<char>(Variant(_16, 0), 1)) = _2;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _10.1 as u64;
place!(Field::<char>(Variant((*_33), 0), 1)) = _2;
_24 = core::ptr::addr_of!(_26);
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2)];
place!(Field::<i8>(Variant((*_20), 0), 3)) = _25;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 << _10.0;
Goto(bb27)
}
bb27 = {
place!(Field::<i8>(Variant((*_20), 0), 3)) = -_25;
_22 = _32 | _32;
_14 = [_28,_28,_28,_28];
place!(Field::<i8>(Variant((*_20), 0), 3)) = _25 ^ _25;
place!(Field::<char>(Variant((*_33), 0), 1)) = _4;
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2)];
_14 = [_28,_28,_28,_28];
place!(Field::<i8>(Variant((*_20), 0), 3)) = _25;
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
Goto(bb28)
}
bb28 = {
_35 = !_22;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _3 as u64;
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),_8,Field::<u64>(Variant((*_20), 0), 2),_8];
_10.2 = _2;
_4 = Field::<char>(Variant((*_20), 0), 1);
place!(Field::<i8>(Variant((*_20), 0), 3)) = -_25;
place!(Field::<char>(Variant(_16, 0), 1)) = RET;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 ^ _9.0.0;
match _10.0 {
0 => bb18,
1 => bb2,
2 => bb8,
3 => bb7,
4 => bb15,
56434 => bb29,
_ => bb6
}
}
bb29 = {
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),_8];
place!(Field::<i128>(Variant((*_20), 0), 0)) = _11 as i128;
place!(Field::<char>(Variant((*_20), 0), 1)) = _10.2;
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2)];
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 * _8;
_30 = Field::<i8>(Variant((*_20), 0), 3) as i32;
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
_23 = core::ptr::addr_of!(_31);
place!(Field::<u64>(Variant((*_20), 0), 2)) = _17 as u64;
place!(Field::<i128>(Variant((*_20), 0), 0)) = _9.0.0 << Field::<i8>(Variant((*_20), 0), 3);
place!(Field::<char>(Variant((*_20), 0), 1)) = _4;
place!(Field::<i8>(Variant((*_20), 0), 3)) = _25 ^ _25;
(*_20) = Adt48::Variant0 { fld0: _9.0.0,fld1: _5,fld2: _8,fld3: _25 };
Goto(bb30)
}
bb30 = {
(*_24) = [Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2),Field::<u64>(Variant((*_20), 0), 2)];
Call((*_23) = fn6(Move(_24), Move((*_20)), Move(_9), _2, Move(_1), Move(_33), Move(_23), Move(_19), Move(_20), (*_24), (*_24), RET), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_14 = [_28,_28,_28,_28];
_5 = _4;
_27 = core::ptr::addr_of!(_15);
(*_27) = 129387604066680589336524152119183801200_i128 as f64;
_33 = core::ptr::addr_of_mut!(_16);
(*_27) = _10.0 as f64;
_36 = _4;
_23 = core::ptr::addr_of!(_31);
_15 = _8 as f64;
_2 = _36;
(*_23) = core::ptr::addr_of!(_26);
(*_23) = core::ptr::addr_of!((*_31));
(*_27) = _25 as f64;
_18 = [_10.3,_10.3,_10.3,_10.3];
(*_23) = core::ptr::addr_of!((*_31));
_35 = !_32;
(*_27) = _10.0 as f64;
_28 = (-29388_i16) & (-32653_i16);
_39 = core::ptr::addr_of!(_29);
(*_23) = core::ptr::addr_of!((*_31));
(*_27) = 3284842463_u32 as f64;
_17 = (*_39) as i64;
(*_39) = 159_u8;
(*_23) = core::ptr::addr_of!((*_31));
(*_23) = core::ptr::addr_of!((*_31));
_19 = core::ptr::addr_of_mut!((*_33));
Call(_10.0 = core::intrinsics::transmute(_28), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_10.0 = !9974_u16;
Goto(bb33)
}
bb33 = {
RET = _2;
_17 = _28 as i64;
_20 = core::ptr::addr_of_mut!((*_33));
(*_20) = Adt48::Variant0 { fld0: (-116412015112256103358453680822156232328_i128),fld1: _5,fld2: _8,fld3: _25 };
Goto(bb34)
}
bb34 = {
(*_39) = 234_u8 | 50_u8;
place!(Field::<u64>(Variant((*_20), 0), 2)) = _8 << (*_39);
place!(Field::<u64>(Variant((*_19), 0), 2)) = _8 >> Field::<i8>(Variant((*_20), 0), 3);
_9.0.0 = (-116292259495420098508721007419079654171_i128) - 136607945393774949915987417230200762052_i128;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25;
place!(Field::<char>(Variant((*_20), 0), 1)) = _10.2;
place!(Field::<i8>(Variant((*_33), 0), 3)) = !_25;
place!(Field::<i128>(Variant((*_19), 0), 0)) = _9.0.0 + _9.0.0;
(*_33) = Adt48::Variant0 { fld0: _9.0.0,fld1: _3,fld2: _8,fld3: _25 };
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8;
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),_8];
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25;
place!(Field::<char>(Variant((*_33), 0), 1)) = _5;
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 & _25;
(*_27) = _11 as f64;
_37 = Move(_23);
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 + _8;
Goto(bb35)
}
bb35 = {
place!(Field::<char>(Variant((*_33), 0), 1)) = _4;
place!(Field::<char>(Variant((*_33), 0), 1)) = _6;
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0 - _9.0.0;
_36 = Field::<char>(Variant(_16, 0), 1);
(*_39) = 150_u8;
(*_31) = [Field::<u64>(Variant(_16, 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
_8 = (*_39) as u64;
(*_27) = _10.3 as f64;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 * _8;
place!(Field::<char>(Variant((*_33), 0), 1)) = _4;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 + _8;
_15 = _17 as f64;
match (*_39) {
0 => bb21,
1 => bb17,
2 => bb33,
3 => bb9,
4 => bb36,
150 => bb38,
_ => bb37
}
}
bb36 = {
Return()
}
bb37 = {
Return()
}
bb38 = {
(*_33) = Adt48::Variant0 { fld0: _9.0.0,fld1: RET,fld2: _8,fld3: _25 };
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
(*_27) = _17 as f64;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _7 as u64;
(*_33) = Adt48::Variant0 { fld0: _9.0.0,fld1: _36,fld2: _8,fld3: _25 };
_6 = _5;
place!(Field::<char>(Variant((*_33), 0), 1)) = _36;
place!(Field::<i8>(Variant((*_33), 0), 3)) = Field::<u64>(Variant((*_33), 0), 2) as i8;
(*_39) = 63_u8 - 42_u8;
(*_27) = _10.1 as f64;
place!(Field::<i8>(Variant((*_33), 0), 3)) = -_25;
_10.3 = 280237336493823976321806140973990938571_u128 >> Field::<i128>(Variant((*_33), 0), 0);
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0 ^ _9.0.0;
_31 = core::ptr::addr_of!((*_31));
_11 = _21 * _21;
place!(Field::<i8>(Variant((*_33), 0), 3)) = -_25;
_23 = Move(_37);
_35 = _22;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 * _8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _11 as u64;
match _25 {
0 => bb30,
1 => bb25,
2 => bb9,
340282366920938463463374607431768211426 => bb39,
_ => bb27
}
}
bb39 = {
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 | _8;
_49.1.fld3.0 = _30;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 ^ _25;
(*_27) = _10.3 as f64;
_1 = core::ptr::addr_of_mut!(_49.1.fld3.1);
place!(Field::<char>(Variant((*_33), 0), 1)) = _2;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 | _8;
place!(Field::<i8>(Variant((*_33), 0), 3)) = !_25;
place!(Field::<char>(Variant((*_33), 0), 1)) = _5;
(*_39) = 4114245455794483418_usize as u8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _10.3 as u64;
(*_39) = 234_u8;
match (*_39) {
234 => bb41,
_ => bb40
}
}
bb40 = {
Return()
}
bb41 = {
(*_27) = 13957050759476678118_usize as f64;
_8 = Field::<u64>(Variant((*_33), 0), 2);
_10.1 = _10.3 as i32;
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0 >> _28;
(*_33) = Adt48::Variant0 { fld0: _9.0.0,fld1: RET,fld2: _8,fld3: _25 };
place!(Field::<char>(Variant((*_33), 0), 1)) = RET;
place!(Field::<char>(Variant((*_33), 0), 1)) = RET;
(*_39) = _11 as u8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8;
place!(Field::<char>(Variant((*_33), 0), 1)) = _3;
_55 = Field::<char>(Variant((*_33), 0), 1);
_12 = core::ptr::addr_of_mut!((*_1));
(*_27) = 3681908443_u32 as f64;
(*_33) = Adt48::Variant0 { fld0: _9.0.0,fld1: RET,fld2: _8,fld3: _25 };
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0 + _9.0.0;
_21 = -_11;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 * _25;
(*_27) = Field::<i8>(Variant((*_33), 0), 3) as f64;
(*_27) = _10.3 as f64;
(*_39) = !245_u8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 + _8;
match _25 {
340282366920938463463374607431768211426 => bb43,
_ => bb42
}
}
bb42 = {
_2 = _5;
RET = _10.2;
_11 = _8 as f32;
_14 = [10539_i16,(-23063_i16),(-2093_i16),8612_i16];
_4 = _5;
_10.0 = 27460_u16;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
27460 => bb13,
_ => bb12
}
}
bb43 = {
place!(Field::<u64>(Variant((*_33), 0), 2)) = _7 as u64;
place!(Field::<i128>(Variant((*_33), 0), 0)) = !_9.0.0;
(*_27) = _17 as f64;
place!(Field::<i8>(Variant((*_33), 0), 3)) = -_25;
(*_39) = _10.3 as u8;
_43 = -_28;
place!(Field::<char>(Variant((*_33), 0), 1)) = _4;
(*_31) = [_8,Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant(_16, 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0 - _9.0.0;
_41 = [Field::<i128>(Variant((*_33), 0), 0),Field::<i128>(Variant((*_33), 0), 0),Field::<i128>(Variant((*_33), 0), 0),Field::<i128>(Variant((*_33), 0), 0)];
place!(Field::<i8>(Variant((*_33), 0), 3)) = !_25;
_49.1.fld1.2 = RET;
_58 = [_30,_10.1];
_8 = Field::<u64>(Variant((*_33), 0), 2) ^ Field::<u64>(Variant((*_33), 0), 2);
(*_27) = _10.3 as f64;
(*_39) = 204_u8 ^ 86_u8;
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),_8,_8,Field::<u64>(Variant((*_33), 0), 2)];
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 >> Field::<i128>(Variant((*_33), 0), 0);
_7 = false;
_51 = -_17;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 * _25;
Goto(bb44)
}
bb44 = {
(*_39) = 33_u8 + 118_u8;
_48 = Field::<char>(Variant((*_33), 0), 1);
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 | _8;
_64 = [(*_39),(*_39),(*_39),(*_39)];
place!(Field::<char>(Variant((*_33), 0), 1)) = _49.1.fld1.2;
_32 = _35 * _22;
(*_31) = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
place!(Field::<i128>(Variant((*_33), 0), 0)) = _9.0.0;
_49.1.fld1.3 = 1131986952_u32 as u128;
_15 = 9610997518858045590_usize as f64;
_54.0 = _9.0.0 - Field::<i128>(Variant((*_33), 0), 0);
_54.0 = !Field::<i128>(Variant((*_33), 0), 0);
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 | _8;
place!(Field::<i8>(Variant((*_33), 0), 3)) = !_25;
_10.3 = _49.1.fld1.3 << Field::<i128>(Variant((*_33), 0), 0);
_57 = &mut _10.3;
_62 = _32;
_49.1.fld4 = !8657_u16;
match _25 {
0 => bb13,
1 => bb43,
2 => bb5,
3 => bb4,
4 => bb45,
340282366920938463463374607431768211426 => bb47,
_ => bb46
}
}
bb45 = {
Return()
}
bb46 = {
_10.0 = !9974_u16;
Goto(bb33)
}
bb47 = {
_26 = [Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2),Field::<u64>(Variant((*_33), 0), 2)];
_31 = core::ptr::addr_of!((*_31));
_21 = _11;
_73.2 = &mut _30;
place!(Field::<i128>(Variant((*_33), 0), 0)) = !_54.0;
place!(Field::<i128>(Variant((*_33), 0), 0)) = -_9.0.0;
place!(Field::<char>(Variant((*_33), 0), 1)) = _6;
_60 = _32 | _32;
_56 = -_22;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 << (*_57);
(*_27) = _62 as f64;
_42 = [(*_57),(*_57),(*_57),(*_57),(*_57)];
(*_57) = _49.1.fld1.3 ^ _49.1.fld1.3;
_71 = Move(_39);
(*_27) = _49.1.fld3.0 as f64;
_72 = core::ptr::addr_of!((*_31));
_49.1.fld2 = [Field::<char>(Variant((*_33), 0), 1),_4,Field::<char>(Variant((*_33), 0), 1),Field::<char>(Variant((*_33), 0), 1),Field::<char>(Variant((*_33), 0), 1),Field::<char>(Variant((*_33), 0), 1),Field::<char>(Variant((*_33), 0), 1)];
_4 = RET;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _49.1.fld3.0 as i8;
place!(Field::<u64>(Variant((*_33), 0), 2)) = _8 ^ _8;
place!(Field::<i8>(Variant((*_33), 0), 3)) = _25 << Field::<u64>(Variant((*_33), 0), 2);
_64 = [_29,_29,_29,_29];
_54 = (Field::<i128>(Variant((*_33), 0), 0), Move(_57));
match _25 {
340282366920938463463374607431768211426 => bb48,
_ => bb45
}
}
bb48 = {
_23 = core::ptr::addr_of!(_24);
Call(place!(Field::<i128>(Variant((*_33), 0), 0)) = core::intrinsics::transmute(_54.0), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_55 = Field::<char>(Variant((*_33), 0), 1);
(*_27) = _35 as f64;
_23 = core::ptr::addr_of!((*_23));
(*_27) = _49.1.fld4 as f64;
place!(Field::<i8>(Variant((*_33), 0), 3)) = !_25;
_38.0 = core::ptr::addr_of_mut!(_75);
_61 = (*_27) + (*_27);
_49.1.fld6 = _42;
_83.0 = _28 - _28;
(*_23) = core::ptr::addr_of!(_79);
_49.1.fld1.1 = _49.1.fld3.0;
_28 = _43 * _83.0;
_72 = core::ptr::addr_of!((*_31));
Goto(bb50)
}
bb50 = {
Call(_85 = dump_var(Move(_4), Move(_55), Move(_48), Move(_30)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_85 = dump_var(Move(_41), Move(_64), Move(_35), Move(_5)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_85 = dump_var(Move(_36), Move(_26), Move(_8), Move(_32)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_85 = dump_var(Move(_17), Move(_29), Move(_42), Move(_10)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *const [u64; 4],mut _2: Adt48,mut _3: ((i128, &'static mut u128),),mut _4: char,mut _5: *mut *const usize,mut _6: *mut Adt48,mut _7: *const *const [u64; 4],mut _8: *mut Adt48,mut _9: *mut Adt48,mut _10: [u64; 4],mut _11: [u64; 4],mut _12: char) -> *const [u64; 4] {
mir! {
type RET = *const [u64; 4];
let _13: i16;
let _14: *const [u64; 4];
let _15: &'static &'static Adt25;
let _16: &'static &'static Adt25;
let _17: *mut (&'static &'static u32, [u128; 6], char);
let _18: [i128; 4];
let _19: u8;
let _20: ((i128, &'static mut u128),);
let _21: [i128; 4];
let _22: u32;
let _23: i64;
let _24: f32;
let _25: isize;
let _26: ((i128, &'static mut u128),);
let _27: char;
let _28: [char; 8];
let _29: &'static mut i32;
let _30: i128;
let _31: *mut [i64; 2];
let _32: u64;
let _33: ((i128, &'static mut u128),);
let _34: f64;
let _35: i16;
let _36: i8;
let _37: char;
let _38: isize;
let _39: [isize; 7];
let _40: [i64; 2];
let _41: isize;
let _42: [i32; 2];
let _43: ([i64; 2], [u128; 5], &'static mut u128);
let _44: &'static Adt25;
let _45: *const *const [u64; 4];
let _46: [char; 8];
let _47: &'static &'static Adt25;
let _48: f32;
let _49: u16;
let _50: *mut (i128, &'static mut u128);
let _51: bool;
let _52: [i64; 2];
let _53: *const (i32, *const usize);
let _54: Adt73;
let _55: char;
let _56: *const *const [u64; 4];
let _57: char;
let _58: ((i128, &'static mut u128),);
let _59: [u128; 6];
let _60: *mut (i128, &'static mut u128);
let _61: [u128; 4];
let _62: &'static mut (i128, &'static mut u128);
let _63: (i16, Adt59, &'static u32);
let _64: *mut Adt48;
let _65: ((i128, &'static mut u128),);
let _66: (i32, *const usize);
let _67: bool;
let _68: u8;
let _69: bool;
let _70: f64;
let _71: [u128; 6];
let _72: *const [u64; 4];
let _73: [isize; 3];
let _74: [u128; 4];
let _75: *const u8;
let _76: &'static &'static Adt25;
let _77: isize;
let _78: (Adt22, *mut (i128, &'static mut u128));
let _79: isize;
let _80: (i16, Adt59, &'static u32);
let _81: f32;
let _82: bool;
let _83: i8;
let _84: char;
let _85: &'static mut &'static mut (i128, &'static mut u128);
let _86: *const f64;
let _87: i16;
let _88: u32;
let _89: f32;
let _90: u32;
let _91: ((u8, usize, isize), &'static mut u128);
let _92: [i128; 4];
let _93: (i32, *const usize);
let _94: isize;
let _95: isize;
let _96: f64;
let _97: Adt28;
let _98: u16;
let _99: (((u8, usize, isize), &'static mut u128), u8);
let _100: char;
let _101: *mut (i128, &'static mut u128);
let _102: bool;
let _103: ();
let _104: ();
{
RET = core::ptr::addr_of!(_11);
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_9 = core::ptr::addr_of_mut!(_2);
place!(Field::<char>(Variant(_2, 0), 1)) = _4;
place!(Field::<i8>(Variant(_2, 0), 3)) = !32_i8;
place!(Field::<i128>(Variant((*_9), 0), 0)) = Field::<char>(Variant((*_9), 0), 1) as i128;
(*RET) = _10;
place!(Field::<char>(Variant((*_9), 0), 1)) = _4;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0 & _3.0.0;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0 | _3.0.0;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2)];
place!(Field::<i8>(Variant((*_9), 0), 3)) = 77_i8 | 30_i8;
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-34_i8) | 5_i8;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<i128>(Variant((*_9), 0), 0)) = !_3.0.0;
place!(Field::<char>(Variant((*_9), 0), 1)) = _4;
match _3.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
163399650977738736025906504699735417393 => bb9,
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
_2 = Adt48::Variant0 { fld0: _3.0.0,fld1: _12,fld2: 9557852582848188102_u64,fld3: (-45_i8) };
(*RET) = [9685222160245655113_u64,772100498458635567_u64,7987459458070934221_u64,3148265757322854566_u64];
(*RET) = [8350370592881319539_u64,4508081087516366530_u64,16420789939863265071_u64,18381304321003616735_u64];
place!(Field::<u64>(Variant((*_9), 0), 2)) = 7937333646677527327_u64;
place!(Field::<u64>(Variant((*_9), 0), 2)) = 397674230073547605_u64;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0 ^ _3.0.0;
(*_9) = Adt48::Variant0 { fld0: _3.0.0,fld1: _4,fld2: 10357029504252182283_u64,fld3: 32_i8 };
_8 = Move(_9);
(*RET) = [3066706511007396558_u64,8898584344352449321_u64,213156922405960459_u64,16294099726487813033_u64];
(*RET) = [10992073283539627443_u64,3816059899660390255_u64,3736832181331773304_u64,16048101007448395763_u64];
place!(Field::<i8>(Variant(_2, 0), 3)) = (-111_i8) - (-18_i8);
_14 = core::ptr::addr_of!((*RET));
(*RET) = [13693303588612842191_u64,6455676629122728648_u64,5877543686923781425_u64,16673640227089502733_u64];
place!(Field::<u64>(Variant(_2, 0), 2)) = 2322026270800955565_u64;
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_9 = core::ptr::addr_of_mut!(_2);
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-112_i8);
_9 = core::ptr::addr_of_mut!((*_9));
(*RET) = _10;
place!(Field::<u64>(Variant((*_9), 0), 2)) = 915212366_u32 as u64;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0 * _3.0.0;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0 * _3.0.0;
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2)];
place!(Field::<i128>(Variant((*_9), 0), 0)) = 181282842469960458083589606484722472397_u128 as i128;
(*_9) = Adt48::Variant0 { fld0: _3.0.0,fld1: _12,fld2: 2916680102990420661_u64,fld3: 62_i8 };
place!(Field::<u64>(Variant((*_9), 0), 2)) = 18_isize as u64;
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-69_i8);
Goto(bb10)
}
bb10 = {
place!(Field::<u64>(Variant((*_9), 0), 2)) = 5694780856573471866_u64 & 4942932495910881737_u64;
_12 = Field::<char>(Variant((*_9), 0), 1);
place!(Field::<i8>(Variant((*_9), 0), 3)) = -66_i8;
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2)];
place!(Field::<i8>(Variant((*_9), 0), 3)) = 108_i8 >> Field::<u64>(Variant((*_9), 0), 2);
Call(place!(Field::<i8>(Variant((*_9), 0), 3)) = fn7(Move(RET), Move(_9), Move(_6)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3.0.0 = Field::<i128>(Variant(_2, 0), 0) * Field::<i128>(Variant(_2, 0), 0);
RET = core::ptr::addr_of!(_10);
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_14 = core::ptr::addr_of!((*RET));
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_8 = core::ptr::addr_of_mut!(_2);
place!(Field::<i128>(Variant(_2, 0), 0)) = 17585319920640874606_usize as i128;
(*RET) = _11;
place!(Field::<i8>(Variant((*_8), 0), 3)) = !(-75_i8);
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i8>(Variant((*_9), 0), 3)) = Field::<char>(Variant((*_8), 0), 1) as i8;
place!(Field::<i128>(Variant((*_9), 0), 0)) = Field::<u64>(Variant((*_9), 0), 2) as i128;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0;
place!(Field::<u64>(Variant((*_9), 0), 2)) = 3622371636574376000_u64 - 15372657881443442129_u64;
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_9), 0), 2)];
(*RET) = _11;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_8), 0), 3)) = !43_i8;
_6 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0;
place!(Field::<char>(Variant((*_6), 0), 1)) = _4;
Goto(bb12)
}
bb12 = {
place!(Field::<u64>(Variant((*_9), 0), 2)) = 4076796829869406959_u64 * 14641724512871849247_u64;
_18 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_9), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_9), 0), 0)];
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
_18 = [Field::<i128>(Variant((*_9), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<i128>(Variant((*_9), 0), 0)) = !_3.0.0;
place!(Field::<i8>(Variant((*_6), 0), 3)) = (-49_i8);
place!(Field::<i128>(Variant((*_8), 0), 0)) = !_3.0.0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 3506929825862076284_u64;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_6), 0), 3)) = false as i8;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = 80_i8 | (-46_i8);
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_3.0.0;
match Field::<u64>(Variant((*_8), 0), 2) {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb13,
3506929825862076284 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-123_i8) & 124_i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-78_i8) | 57_i8;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -51_i8;
_24 = Field::<i128>(Variant((*_8), 0), 0) as f32;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 32_i8;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-66_i8) >> Field::<i128>(Variant((*_8), 0), 0);
(*RET) = _11;
_19 = 140_u8 + 33_u8;
place!(Field::<i128>(Variant((*_8), 0), 0)) = Field::<char>(Variant((*_8), 0), 1) as i128;
_21 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_8 = core::ptr::addr_of_mut!((*_8));
_22 = !3683433967_u32;
_30 = !Field::<i128>(Variant((*_8), 0), 0);
_18 = [_30,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-36_i8) & (-74_i8);
Call(place!(Field::<i8>(Variant((*_8), 0), 3)) = core::intrinsics::bswap((-23_i8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<u64>(Variant((*_8), 0), 2)) = _24 as u64;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _30 & _3.0.0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 6779501935231177118_u64 >> Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i128>(Variant(_2, 0), 0)) = !_3.0.0;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 4769975872938822315_u64;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
_25 = -(-2_isize);
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_33.0.0 = Field::<i128>(Variant((*_8), 0), 0) ^ Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-37_i8);
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = !(-6_i8);
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-2_i8) >> _19;
_26.0.0 = 286921705874937175008532869070536503549_u128 as i128;
place!(Field::<i128>(Variant(_2, 0), 0)) = _33.0.0 + _30;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
Goto(bb17)
}
bb17 = {
place!(Field::<u64>(Variant((*_8), 0), 2)) = 6930811353539273807_u64 << Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_33.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -(-30_i8);
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
_11 = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_18 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-108_i8) | (-38_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !8722278137249967229_u64;
_13 = (-3258_i16) << Field::<u64>(Variant((*_8), 0), 2);
_20.0.0 = Field::<i128>(Variant((*_8), 0), 0) + Field::<i128>(Variant((*_8), 0), 0);
Goto(bb18)
}
bb18 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-5_i8) + (-4_i8);
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i8>(Variant((*_8), 0), 3)) = 67_i8 << Field::<u64>(Variant((*_8), 0), 2);
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_38 = _25 * _25;
place!(Field::<char>(Variant(_2, 0), 1)) = _12;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _20.0.0 * _20.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-17_i8) ^ (-90_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = 3143423808089901176_u64;
(*_8) = Adt48::Variant0 { fld0: _20.0.0,fld1: _4,fld2: 17723456195675155972_u64,fld3: 63_i8 };
place!(Field::<u64>(Variant((*_8), 0), 2)) = 15009398962412549006_u64 | 9099529361440387695_u64;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
Goto(bb19)
}
bb19 = {
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
Goto(bb20)
}
bb20 = {
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<u64>(Variant(_2, 0), 2)) = _38 as u64;
place!(Field::<i128>(Variant(_2, 0), 0)) = _20.0.0 & _20.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 9195813758792672905_u64;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_10 = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 >> _25;
place!(Field::<u64>(Variant((*_8), 0), 2)) = Field::<char>(Variant((*_8), 0), 1) as u64;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -21_i8;
(*RET) = _11;
(*RET) = _11;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
Goto(bb21)
}
bb21 = {
_11 = (*RET);
(*_8) = Adt48::Variant0 { fld0: _20.0.0,fld1: _4,fld2: 15093598472632470289_u64,fld3: (-56_i8) };
(*RET) = [15161575559460681737_u64,3504380099229199286_u64,9727564936074945803_u64,8997238797036797409_u64];
(*_8) = Adt48::Variant0 { fld0: _3.0.0,fld1: _4,fld2: 316408972218319338_u64,fld3: 68_i8 };
_39 = [_25,_38,_38,_38,_25,_38,_25];
place!(Field::<i128>(Variant((*_8), 0), 0)) = 31435_u16 as i128;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _3.0.0 >> _3.0.0;
place!(Field::<i8>(Variant(_2, 0), 3)) = 86_i8 * (-20_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !1122388842439176141_u64;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-23_i8);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _3.0.0 ^ _33.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
(*_8) = Adt48::Variant0 { fld0: _20.0.0,fld1: _4,fld2: 2315144243642885184_u64,fld3: (-6_i8) };
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 - _20.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-33_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !10267685143396388682_u64;
_13 = (-22255_i16) * 25306_i16;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 - _33.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_3.0.0;
Goto(bb22)
}
bb22 = {
(*RET) = _11;
place!(Field::<char>(Variant(_2, 0), 1)) = _4;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 9064319146022149346_u64 & 11014228340604373835_u64;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_43.0 = [2590589204510312992_i64,(-28657102958442739_i64)];
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_33.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !10361061417097145436_u64;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 8047943419679360416_u64;
_38 = _25 ^ _25;
_21 = [Field::<i128>(Variant((*_8), 0), 0),_33.0.0,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_21 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
(*_8) = Adt48::Variant0 { fld0: _20.0.0,fld1: _12,fld2: 5638489774190809833_u64,fld3: 11_i8 };
(*RET) = [10094866117047509405_u64,9552223987589791037_u64,1990308428779296872_u64,15790914445517715356_u64];
_19 = 33_u8 & 155_u8;
place!(Field::<u64>(Variant((*_8), 0), 2)) = (-98_i8) as u64;
_41 = _38 >> Field::<u64>(Variant((*_8), 0), 2);
(*RET) = _11;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-19_i8) << _19;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
_35 = _13 & _13;
_7 = core::ptr::addr_of!(_14);
_38 = -_41;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -0_i8;
Goto(bb23)
}
bb23 = {
_26.0.0 = Field::<i128>(Variant((*_8), 0), 0) - Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i8>(Variant((*_8), 0), 3)) = 10691603430475912151_usize as i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-35_i8) * (-116_i8);
_4 = Field::<char>(Variant((*_8), 0), 1);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _3.0.0 | _20.0.0;
(*_7) = Move(RET);
place!(Field::<i128>(Variant(_2, 0), 0)) = !_26.0.0;
_40 = [(-8548279926301578462_i64),(-4724361563760286090_i64)];
place!(Field::<u64>(Variant((*_8), 0), 2)) = !5291659306063265941_u64;
_42 = [(-2132931493_i32),(-39470216_i32)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0 * _26.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-6_i8) << Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !15852927311263215866_u64;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0 << Field::<i8>(Variant((*_8), 0), 3);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _35 as i128;
_35 = _13;
Goto(bb24)
}
bb24 = {
_14 = Move(_1);
place!(Field::<i8>(Variant(_2, 0), 3)) = _41 as i8;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0 * _33.0.0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !8767167837888506335_u64;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-125_i8) * (-115_i8);
_22 = 2351500993_u32 + 3968345152_u32;
(*_7) = core::ptr::addr_of!(_10);
place!(Field::<u64>(Variant((*_8), 0), 2)) = _24 as u64;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 11343974581049235426_u64 - 17032274480478587288_u64;
place!(Field::<char>(Variant(_2, 0), 1)) = _12;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 68_i8 - (-7_i8);
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_33.0.0;
_26.0.0 = Field::<i128>(Variant((*_8), 0), 0);
_11 = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_23 = (-3154033715203661770_i64) << Field::<i8>(Variant((*_8), 0), 3);
Goto(bb25)
}
bb25 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = 121_i8 * (-7_i8);
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*_7) = core::ptr::addr_of!(_11);
_42 = [(-1097493707_i32),1461741910_i32];
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_31 = core::ptr::addr_of_mut!(_40);
(*_31) = [_23,_23];
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
Goto(bb26)
}
bb26 = {
place!(Field::<i128>(Variant((*_8), 0), 0)) = Field::<i8>(Variant((*_8), 0), 3) as i128;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
(*_31) = [_23,_23];
(*_8) = Adt48::Variant0 { fld0: _26.0.0,fld1: _12,fld2: 4781606911437642645_u64,fld3: (-47_i8) };
_45 = Move(_7);
_1 = core::ptr::addr_of!((*_14));
place!(Field::<u64>(Variant((*_8), 0), 2)) = _19 as u64;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !11056758086400668305_u64;
RET = Move(_14);
place!(Field::<u64>(Variant((*_8), 0), 2)) = 3400511406186371184_u64 >> Field::<i128>(Variant((*_8), 0), 0);
_6 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i128>(Variant(_2, 0), 0)) = _20.0.0 | _20.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-77_i8) >> Field::<u64>(Variant((*_8), 0), 2);
place!(Field::<u64>(Variant((*_8), 0), 2)) = 13891014707026541723_u64;
(*_1) = _10;
_22 = 1459842493_u32 >> Field::<i128>(Variant((*_6), 0), 0);
(*_31) = _43.0;
(*_31) = [_23,_23];
_7 = core::ptr::addr_of!(RET);
_21 = _18;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
_20.0.0 = Field::<i128>(Variant((*_6), 0), 0) | Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i128>(Variant((*_8), 0), 0)) = 1631089697_i32 as i128;
(*_31) = [_23,_23];
match Field::<u64>(Variant(_2, 0), 2) {
0 => bb16,
1 => bb2,
2 => bb3,
3 => bb25,
4 => bb10,
5 => bb14,
6 => bb21,
13891014707026541723 => bb28,
_ => bb27
}
}
bb27 = {
Return()
}
bb28 = {
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 - _20.0.0;
(*_7) = core::ptr::addr_of!((*_1));
_48 = (-1628632503_i32) as f32;
(*_31) = _43.0;
Goto(bb29)
}
bb29 = {
(*_1) = _10;
place!(Field::<i8>(Variant(_2, 0), 3)) = (-117_i8) + (-89_i8);
place!(Field::<i8>(Variant((*_8), 0), 3)) = !115_i8;
(*_7) = Move(_1);
_14 = core::ptr::addr_of!(_10);
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_4 = Field::<char>(Variant((*_8), 0), 1);
place!(Field::<u64>(Variant((*_8), 0), 2)) = 9976897964414712505_u64 >> Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i128>(Variant((*_8), 0), 0)) = !_20.0.0;
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_20.0.0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 14569512765760184514_u64 - 9055436459025734074_u64;
_13 = _35;
(*_31) = _43.0;
(*_31) = _43.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _23 as i128;
Call((*_31) = core::intrinsics::transmute(Field::<i128>(Variant((*_8), 0), 0)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = 17_i8 >> Field::<u64>(Variant((*_8), 0), 2);
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-82_i8) & 21_i8;
(*_31) = [_23,_23];
_7 = core::ptr::addr_of!((*_7));
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_32 = Field::<u64>(Variant((*_8), 0), 2);
_4 = Field::<char>(Variant((*_8), 0), 1);
(*_14) = _11;
_33.0.0 = Field::<char>(Variant((*_8), 0), 1) as i128;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 >> Field::<i8>(Variant((*_8), 0), 3);
_52 = [_23,_23];
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-117_i8) ^ 8_i8;
_35 = 1_usize as i16;
Goto(bb31)
}
bb31 = {
_2 = Adt48::Variant0 { fld0: _20.0.0,fld1: _12,fld2: _32,fld3: 95_i8 };
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 | _20.0.0;
_33.0.0 = Field::<i128>(Variant((*_8), 0), 0) << Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<u64>(Variant((*_8), 0), 2)) = _22 as u64;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-90_i8) ^ 58_i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -(-81_i8);
(*_7) = Move(_14);
(*_7) = core::ptr::addr_of!(_10);
(*_31) = _52;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 - _32;
place!(Field::<i8>(Variant((*_8), 0), 3)) = !98_i8;
(*_7) = core::ptr::addr_of!((*RET));
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_33.0.0;
RET = core::ptr::addr_of!((*RET));
place!(Field::<i8>(Variant((*_8), 0), 3)) = 85_i8 >> Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i8>(Variant((*_8), 0), 3)) = _19 as i8;
(*_31) = _43.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 1571152644_i32 as i8;
(*_7) = core::ptr::addr_of!((*RET));
(*_31) = _43.0;
_18 = [_33.0.0,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = true as i128;
_52 = [_23,_23];
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _33.0.0 ^ _33.0.0;
(*_7) = core::ptr::addr_of!((*RET));
Goto(bb32)
}
bb32 = {
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 ^ _33.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 << _33.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 50_i8 | (-107_i8);
(*_31) = [_23,_23];
_23 = 6438539071825472003_i64 - 7050052315712293860_i64;
(*RET) = [_32,Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32;
Goto(bb33)
}
bb33 = {
(*_7) = core::ptr::addr_of!((*RET));
(*_31) = [_23,_23];
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 - _32;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-75_i8) >> Field::<i128>(Variant((*_8), 0), 0);
_54.fld2 = [(-1979118691_i32),1725162177_i32];
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _23 as i128;
(*_8) = Adt48::Variant0 { fld0: _33.0.0,fld1: _4,fld2: _32,fld3: (-73_i8) };
place!(Field::<i8>(Variant((*_8), 0), 3)) = -(-52_i8);
_57 = Field::<char>(Variant((*_8), 0), 1);
_58.0.0 = Field::<i128>(Variant((*_8), 0), 0);
(*_31) = _52;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _58.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _57;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),_32,Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = !_33.0.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -(-32_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 | _32;
Goto(bb34)
}
bb34 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = 13_i8 + (-31_i8);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _20.0.0 * _33.0.0;
(*_7) = core::ptr::addr_of!((*RET));
(*_31) = _52;
_39 = [_25,_38,_38,_41,_38,_38,_25];
_54.fld5 = 1430131865_i32 * 1162831658_i32;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 110_i8 - 39_i8;
_43.1 = [214992095187628515906051088202976843242_u128,325144495889309073140933489943187438724_u128,101976524793803448168593488763809673512_u128,53054494109480592445872716814209472905_u128,308001267931470507070281743720122368056_u128];
_59 = [113145602866315944748417496689937567198_u128,276450036544408906520503349999026253421_u128,148940098192567089734375098279939432543_u128,121378037195067776001455495737869615274_u128,54222333232759207805140318525260225431_u128,11377625164327401656288342254700925810_u128];
_63.2 = &_22;
_54.fld0 = Move((*_8));
_41 = _25 >> _58.0.0;
(*_8) = Adt48::Variant0 { fld0: _58.0.0,fld1: Field::<char>(Variant(_54.fld0, 0), 1),fld2: Field::<u64>(Variant(_54.fld0, 0), 2),fld3: Field::<i8>(Variant(_54.fld0, 0), 3) };
_43.0 = [_23,_23];
_10 = _11;
_7 = core::ptr::addr_of!((*_7));
_8 = core::ptr::addr_of_mut!(_2);
_61 = [138792646393919562310202191576342197012_u128,269706359864178495444325684348940392015_u128,124687374354187373922207330455015825253_u128,294680353087760047244614106995019758506_u128];
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant(_54.fld0, 0), 2)];
_56 = core::ptr::addr_of!((*_7));
(*_31) = [_23,_23];
(*_56) = core::ptr::addr_of!((*RET));
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_63.1.fld1.1 = Field::<char>(Variant((*_8), 0), 1) as i32;
_51 = false;
_43.1 = [29760838210293412764810084429245993464_u128,161633566556848399158817744020570213936_u128,61024926196547946682968124145632126826_u128,71653719478038929024030056429476371699_u128,336017338080942573095361438517297593607_u128];
place!(Field::<i8>(Variant((*_8), 0), 3)) = _41 as i8;
Goto(bb35)
}
bb35 = {
(*_8) = Move(_54.fld0);
_63.1.fld1 = (2224_u16, _54.fld5, Field::<char>(Variant((*_8), 0), 1), 109965319447662195244485441403391368297_u128);
(*_7) = core::ptr::addr_of!((*RET));
_57 = Field::<char>(Variant((*_8), 0), 1);
place!(Field::<i8>(Variant((*_8), 0), 3)) = 4_i8 + (-86_i8);
_34 = _63.1.fld1.1 as f64;
(*_31) = _43.0;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 33_i8 - 85_i8;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 + _32;
place!(Field::<char>(Variant((*_8), 0), 1)) = _63.1.fld1.2;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -18_i8;
_38 = _41 | _41;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_61 = [_63.1.fld1.3,_63.1.fld1.3,_63.1.fld1.3,_63.1.fld1.3];
_5 = core::ptr::addr_of_mut!(_63.1.fld3.1);
_70 = _34 - _34;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
Goto(bb36)
}
bb36 = {
(*_7) = core::ptr::addr_of!((*RET));
_61 = [_63.1.fld1.3,_63.1.fld1.3,_63.1.fld1.3,_63.1.fld1.3];
place!(Field::<u64>(Variant((*_8), 0), 2)) = _63.1.fld1.0 as u64;
place!(Field::<char>(Variant((*_8), 0), 1)) = _57;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 23_i8 * (-109_i8);
_41 = Field::<i8>(Variant((*_8), 0), 3) as isize;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _63.1.fld1.0 as i128;
_74 = _61;
place!(Field::<i8>(Variant((*_8), 0), 3)) = Field::<char>(Variant((*_8), 0), 1) as i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -81_i8;
_45 = core::ptr::addr_of!((*_7));
(*_7) = core::ptr::addr_of!((*RET));
_8 = core::ptr::addr_of_mut!((*_8));
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = 102_i8 - (-56_i8);
place!(Field::<i8>(Variant((*_8), 0), 3)) = 25_i8 | (-105_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !_32;
_65.0.0 = !_58.0.0;
_23 = Field::<i8>(Variant((*_8), 0), 3) as i64;
match _63.1.fld1.0 {
0 => bb37,
1 => bb38,
2 => bb39,
3 => bb40,
4 => bb41,
2224 => bb43,
_ => bb42
}
}
bb37 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = 121_i8 * (-7_i8);
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*_7) = core::ptr::addr_of!(_11);
_42 = [(-1097493707_i32),1461741910_i32];
(*_14) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_31 = core::ptr::addr_of_mut!(_40);
(*_31) = [_23,_23];
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
Goto(bb26)
}
bb38 = {
_26.0.0 = Field::<i128>(Variant((*_8), 0), 0) - Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<i8>(Variant((*_8), 0), 3)) = 10691603430475912151_usize as i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-35_i8) * (-116_i8);
_4 = Field::<char>(Variant((*_8), 0), 1);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _3.0.0 | _20.0.0;
(*_7) = Move(RET);
place!(Field::<i128>(Variant(_2, 0), 0)) = !_26.0.0;
_40 = [(-8548279926301578462_i64),(-4724361563760286090_i64)];
place!(Field::<u64>(Variant((*_8), 0), 2)) = !5291659306063265941_u64;
_42 = [(-2132931493_i32),(-39470216_i32)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0 * _26.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-6_i8) << Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<u64>(Variant((*_8), 0), 2)) = !15852927311263215866_u64;
place!(Field::<i128>(Variant((*_8), 0), 0)) = _26.0.0 << Field::<i8>(Variant((*_8), 0), 3);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _35 as i128;
_35 = _13;
Goto(bb24)
}
bb39 = {
_3.0.0 = Field::<i128>(Variant(_2, 0), 0) * Field::<i128>(Variant(_2, 0), 0);
RET = core::ptr::addr_of!(_10);
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_14 = core::ptr::addr_of!((*RET));
(*RET) = [Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2)];
_8 = core::ptr::addr_of_mut!(_2);
place!(Field::<i128>(Variant(_2, 0), 0)) = 17585319920640874606_usize as i128;
(*RET) = _11;
place!(Field::<i8>(Variant((*_8), 0), 3)) = !(-75_i8);
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i8>(Variant((*_9), 0), 3)) = Field::<char>(Variant((*_8), 0), 1) as i8;
place!(Field::<i128>(Variant((*_9), 0), 0)) = Field::<u64>(Variant((*_9), 0), 2) as i128;
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0;
place!(Field::<u64>(Variant((*_9), 0), 2)) = 3622371636574376000_u64 - 15372657881443442129_u64;
(*RET) = [Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_9), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_9), 0), 2)];
(*RET) = _11;
place!(Field::<char>(Variant((*_9), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_8), 0), 3)) = !43_i8;
_6 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i128>(Variant((*_9), 0), 0)) = _3.0.0;
place!(Field::<char>(Variant((*_6), 0), 1)) = _4;
Goto(bb12)
}
bb40 = {
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-123_i8) & 124_i8;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-78_i8) | 57_i8;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -51_i8;
_24 = Field::<i128>(Variant((*_8), 0), 0) as f32;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
place!(Field::<i8>(Variant((*_8), 0), 3)) = 32_i8;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-66_i8) >> Field::<i128>(Variant((*_8), 0), 0);
(*RET) = _11;
_19 = 140_u8 + 33_u8;
place!(Field::<i128>(Variant((*_8), 0), 0)) = Field::<char>(Variant((*_8), 0), 1) as i128;
_21 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_8 = core::ptr::addr_of_mut!((*_8));
_22 = !3683433967_u32;
_30 = !Field::<i128>(Variant((*_8), 0), 0);
_18 = [_30,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_9 = core::ptr::addr_of_mut!((*_8));
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i8>(Variant((*_9), 0), 3)) = (-36_i8) & (-74_i8);
Call(place!(Field::<i8>(Variant((*_8), 0), 3)) = core::intrinsics::bswap((-23_i8)), ReturnTo(bb16), UnwindUnreachable())
}
bb41 = {
(*RET) = _11;
place!(Field::<char>(Variant(_2, 0), 1)) = _4;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 9064319146022149346_u64 & 11014228340604373835_u64;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_43.0 = [2590589204510312992_i64,(-28657102958442739_i64)];
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<i128>(Variant((*_8), 0), 0)) = -_33.0.0;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<u64>(Variant((*_8), 0), 2)) = !10361061417097145436_u64;
place!(Field::<u64>(Variant((*_8), 0), 2)) = 8047943419679360416_u64;
_38 = _25 ^ _25;
_21 = [Field::<i128>(Variant((*_8), 0), 0),_33.0.0,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
_21 = [Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
(*_8) = Adt48::Variant0 { fld0: _20.0.0,fld1: _12,fld2: 5638489774190809833_u64,fld3: 11_i8 };
(*RET) = [10094866117047509405_u64,9552223987589791037_u64,1990308428779296872_u64,15790914445517715356_u64];
_19 = 33_u8 & 155_u8;
place!(Field::<u64>(Variant((*_8), 0), 2)) = (-98_i8) as u64;
_41 = _38 >> Field::<u64>(Variant((*_8), 0), 2);
(*RET) = _11;
place!(Field::<i8>(Variant((*_8), 0), 3)) = (-19_i8) << _19;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
_35 = _13 & _13;
_7 = core::ptr::addr_of!(_14);
_38 = -_41;
place!(Field::<i8>(Variant((*_8), 0), 3)) = -0_i8;
Goto(bb23)
}
bb42 = {
Return()
}
bb43 = {
(*_7) = core::ptr::addr_of!((*RET));
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_54.fld4 = -_24;
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 | _32;
place!(Field::<char>(Variant((*_8), 0), 1)) = _57;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32;
_3.0.1 = &mut _63.1.fld1.3;
_80.1.fld3.0 = _54.fld5 << _65.0.0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = Field::<char>(Variant((*_8), 0), 1) as u64;
_77 = _38;
_80.1.fld4 = 24844_u16 >> _77;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
(*_8) = Adt48::Variant0 { fld0: _58.0.0,fld1: _4,fld2: _32,fld3: (-74_i8) };
Goto(bb44)
}
bb44 = {
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_58.0.0 = Field::<i128>(Variant((*_8), 0), 0);
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32;
_60 = core::ptr::addr_of_mut!(_3.0);
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
place!(Field::<i8>(Variant((*_8), 0), 3)) = !(-113_i8);
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 << Field::<i8>(Variant((*_8), 0), 3);
_80.1.fld0 = Move(_31);
place!(Field::<i128>(Variant((*_8), 0), 0)) = (*_60).0 >> _38;
_58.0 = Move((*_60));
(*_7) = core::ptr::addr_of!((*RET));
(*_7) = core::ptr::addr_of!((*RET));
_68 = _19 >> Field::<i128>(Variant((*_8), 0), 0);
_64 = Move(_9);
place!(Field::<i128>(Variant((*_8), 0), 0)) = _22 as i128;
(*_60).0 = _77 as i128;
(*_7) = core::ptr::addr_of!((*RET));
_13 = _51 as i16;
_83 = _22 as i8;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant(_2, 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_66.0 = _80.1.fld3.0 >> (*_60).0;
place!(Field::<u64>(Variant((*_8), 0), 2)) = _32 | _32;
_25 = _66.0 as isize;
(*RET) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
Goto(bb45)
}
bb45 = {
(*_8) = Adt48::Variant0 { fld0: _65.0.0,fld1: _57,fld2: _32,fld3: _83 };
_18 = [Field::<i128>(Variant((*_8), 0), 0),(*_60).0,(*_60).0,Field::<i128>(Variant((*_8), 0), 0)];
place!(Field::<i128>(Variant((*_8), 0), 0)) = -(*_60).0;
_39 = [_38,_77,_77,_25,_77,_77,_38];
place!(Field::<i128>(Variant((*_8), 0), 0)) = (*_60).0 + (*_60).0;
_80.1.fld3.0 = -_66.0;
(*_5) = core::ptr::addr_of!(_91.0.1);
_80.0 = _35 | _13;
_81 = -_54.fld4;
(*_7) = core::ptr::addr_of!((*RET));
place!(Field::<char>(Variant((*_8), 0), 1)) = _12;
_18 = [(*_60).0,(*_60).0,Field::<i128>(Variant((*_8), 0), 0),Field::<i128>(Variant((*_8), 0), 0)];
Call((*_60).0 = core::intrinsics::transmute(Field::<i128>(Variant((*_8), 0), 0)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
(*_5) = core::ptr::addr_of!(_91.0.1);
place!(Field::<u64>(Variant(_2, 0), 2)) = _22 as u64;
_19 = _68;
_1 = core::ptr::addr_of!((*RET));
_80.2 = &_22;
_54.fld0 = Move((*_8));
(*_5) = core::ptr::addr_of!(_91.0.1);
(*_1) = _11;
_78.1 = core::ptr::addr_of_mut!((*_60));
(*_8) = Move(_54.fld0);
Goto(bb47)
}
bb47 = {
_93.1 = core::ptr::addr_of!(_91.0.1);
_79 = Field::<i8>(Variant((*_8), 0), 3) as isize;
_69 = _51;
_73 = [_38,_77,_77];
(*_1) = [Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2),Field::<u64>(Variant((*_8), 0), 2)];
_14 = core::ptr::addr_of!((*_1));
place!(Field::<i8>(Variant((*_8), 0), 3)) = Field::<u64>(Variant((*_8), 0), 2) as i8;
place!(Field::<u64>(Variant(_2, 0), 2)) = !_32;
(*_5) = core::ptr::addr_of!(_99.0.0.1);
_49 = !_80.1.fld4;
place!(Field::<char>(Variant((*_8), 0), 1)) = _4;
(*_60).0 = Field::<i128>(Variant((*_8), 0), 0) >> Field::<i128>(Variant((*_8), 0), 0);
(*_7) = core::ptr::addr_of!((*_1));
_53 = core::ptr::addr_of!(_93);
(*_7) = core::ptr::addr_of!(_10);
(*_8) = Adt48::Variant0 { fld0: _33.0.0,fld1: _4,fld2: _32,fld3: _83 };
(*_53).1 = core::ptr::addr_of!(_99.0.0.1);
_80.1.fld1 = (_49, _66.0, Field::<char>(Variant((*_8), 0), 1), 88484530949263262926968934063409265321_u128);
(*_14) = _11;
_48 = _54.fld4 * _54.fld4;
_19 = Field::<i128>(Variant((*_8), 0), 0) as u8;
_75 = core::ptr::addr_of!(_68);
(*_75) = 5_usize as u8;
(*_60).1 = &mut _80.1.fld1.3;
Goto(bb48)
}
bb48 = {
Call(_103 = dump_var(Move(_35), Move(_18), Move(_49), Move(_21)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_103 = dump_var(Move(_30), Move(_12), Move(_41), Move(_83)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_103 = dump_var(Move(_10), Move(_11), Move(_23), Move(_51)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_103 = dump_var(Move(_25), Move(_59), Move(_77), Move(_22)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *const [u64; 4],mut _2: *mut Adt48,mut _3: *mut Adt48) -> i8 {
mir! {
type RET = i8;
let _4: *mut (i128, &'static mut u128);
let _5: &'static mut &'static mut (i128, &'static mut u128);
let _6: *const f64;
let _7: i32;
let _8: f32;
let _9: Adt28;
let _10: i128;
let _11: &'static Adt25;
let _12: *mut (i128, &'static mut u128);
let _13: [isize; 3];
let _14: char;
let _15: &'static mut (i128, &'static mut u128);
let _16: &'static mut &'static mut (i128, &'static mut u128);
let _17: (Adt22, *mut (i128, &'static mut u128));
let _18: *const &'static mut (i32, *const usize);
let _19: u8;
let _20: i64;
let _21: [i32; 2];
let _22: f64;
let _23: char;
let _24: u64;
let _25: Adt28;
let _26: [i32; 2];
let _27: [isize; 7];
let _28: bool;
let _29: bool;
let _30: &'static mut &'static mut (i128, &'static mut u128);
let _31: bool;
let _32: *mut (&'static &'static u32, [u128; 6], char);
let _33: *mut [i64; 2];
let _34: Adt38;
let _35: *mut [i64; 2];
let _36: bool;
let _37: [u32; 2];
let _38: i128;
let _39: *mut (&'static &'static u32, [u128; 6], char);
let _40: f32;
let _41: isize;
let _42: *mut (&'static &'static u32, [u128; 6], char);
let _43: &'static mut (i32, *const usize);
let _44: i32;
let _45: [char; 8];
let _46: i16;
let _47: usize;
let _48: &'static mut (i32, *const usize);
let _49: f32;
let _50: *const [u64; 4];
let _51: [isize; 3];
let _52: f32;
let _53: &'static u32;
let _54: &'static mut &'static mut (i128, &'static mut u128);
let _55: [u128; 4];
let _56: ();
let _57: ();
{
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = 101_i8 & 124_i8;
RET = 74_i8 << (-69_i8);
RET = !96_i8;
_3 = Move(_2);
_2 = Move(_3);
RET = 897472217_i32 as i8;
Goto(bb1)
}
bb1 = {
RET = !(-12_i8);
_3 = Move(_2);
_2 = Move(_3);
RET = (-118_i8) >> (-337958849_i32);
_3 = Move(_2);
RET = (-2_i8);
RET = !(-102_i8);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = (-105_i8) | 63_i8;
RET = (-90_i8) - (-127_i8);
_3 = Move(_2);
Goto(bb2)
}
bb2 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb3 = {
RET = (-70_i8) * 24_i8;
RET = (-36_i8) | 16_i8;
RET = (-86_i8) ^ (-67_i8);
_10 = (-137085961315796797039872782834512124868_i128);
match _10 {
0 => bb1,
1 => bb4,
203196405605141666423501824597256086588 => bb6,
_ => bb5
}
}
bb4 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb5 = {
RET = !(-12_i8);
_3 = Move(_2);
_2 = Move(_3);
RET = (-118_i8) >> (-337958849_i32);
_3 = Move(_2);
RET = (-2_i8);
RET = !(-102_i8);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = (-105_i8) | 63_i8;
RET = (-90_i8) - (-127_i8);
_3 = Move(_2);
Goto(bb2)
}
bb6 = {
_3 = Move(_2);
RET = 117_i8;
_8 = 6_usize as f32;
_2 = Move(_3);
_8 = 401276490334669053_usize as f32;
match _10 {
0 => bb1,
1 => bb2,
2 => bb5,
203196405605141666423501824597256086588 => bb7,
_ => bb4
}
}
bb7 = {
_3 = Move(_2);
RET = _8 as i8;
_8 = 221157954088692653997548224326470237725_u128 as f32;
_2 = Move(_3);
_10 = 146058791552639627984259758785232425802_i128 >> _7;
_8 = 145_u8 as f32;
_10 = (-39558352281062205930239273559066905507_i128) << _7;
_8 = 12266229807795335839_usize as f32;
RET = (-79_i8) << _10;
RET = 11371_i16 as i8;
_13 = [9223372036854775807_isize,(-96_isize),(-9223372036854775808_isize)];
_3 = Move(_2);
_7 = 726224085_i32;
_14 = '\u{41074}';
_2 = Move(_3);
_17.0 = Adt22::Variant0 { fld0: 32_isize,fld1: _14 };
Goto(bb8)
}
bb8 = {
place!(Field::<isize>(Variant(_17.0, 0), 0)) = 9223372036854775807_isize >> RET;
_3 = Move(_2);
_17.0 = Adt22::Variant0 { fld0: 76_isize,fld1: _14 };
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = Field::<char>(Variant(_17.0, 0), 1) as isize;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = 6692827080208532721_u64 as isize;
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
_13 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
Goto(bb9)
}
bb9 = {
_10 = 55261_u16 as i128;
_17.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _14 };
_8 = (-9223372036854775808_isize) as f32;
_17.0 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _14 };
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-5_isize) * (-9223372036854775808_isize);
_2 = Move(_3);
_8 = _7 as f32;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-38_isize) & 119_isize;
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
726224085 => bb15,
_ => bb14
}
}
bb10 = {
RET = (-70_i8) * 24_i8;
RET = (-36_i8) | 16_i8;
RET = (-86_i8) ^ (-67_i8);
_10 = (-137085961315796797039872782834512124868_i128);
match _10 {
0 => bb1,
1 => bb4,
203196405605141666423501824597256086588 => bb6,
_ => bb5
}
}
bb11 = {
RET = !(-12_i8);
_3 = Move(_2);
_2 = Move(_3);
RET = (-118_i8) >> (-337958849_i32);
_3 = Move(_2);
RET = (-2_i8);
RET = !(-102_i8);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = (-105_i8) | 63_i8;
RET = (-90_i8) - (-127_i8);
_3 = Move(_2);
Goto(bb2)
}
bb12 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb13 = {
RET = !(-12_i8);
_3 = Move(_2);
_2 = Move(_3);
RET = (-118_i8) >> (-337958849_i32);
_3 = Move(_2);
RET = (-2_i8);
RET = !(-102_i8);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = (-105_i8) | 63_i8;
RET = (-90_i8) - (-127_i8);
_3 = Move(_2);
Goto(bb2)
}
bb14 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb15 = {
_10 = _8 as i128;
_10 = 60434_u16 as i128;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = 9223372036854775807_isize;
_3 = Move(_2);
_10 = !(-100364114832697373952878980932349800465_i128);
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
RET = (-86_i8) >> _7;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-9223372036854775808_isize) & 9223372036854775807_isize;
RET = 59_i8 + (-42_i8);
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
_19 = 86_u8;
_8 = 1230328142769938574_i64 as f32;
_21 = [_7,_7];
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
_6 = core::ptr::addr_of!(_22);
(*_6) = (-13714_i16) as f64;
_10 = RET as i128;
(*_6) = 656164517_u32 as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_2 = Move(_3);
(*_6) = RET as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_23 = _14;
(*_6) = 2567650071_u32 as f64;
(*_6) = 16082_u16 as f64;
(*_6) = _7 as f64;
(*_6) = 35858_u16 as f64;
Goto(bb16)
}
bb16 = {
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
(*_6) = 5920350426669353397_usize as f64;
_3 = Move(_2);
_20 = (-1914769187004853261_i64) * 4345127842682280676_i64;
(*_6) = 171612598470402270261110601843599961093_u128 as f64;
(*_6) = 3478213100_u32 as f64;
_2 = Move(_3);
(*_6) = _10 as f64;
(*_6) = _10 as f64;
_20 = Field::<isize>(Variant(_17.0, 0), 0) as i64;
_14 = Field::<char>(Variant(_17.0, 0), 1);
_7 = 1901858869_i32 - (-464505244_i32);
(*_6) = _8 as f64;
match _19 {
0 => bb7,
86 => bb17,
_ => bb6
}
}
bb17 = {
_20 = 3613598325844415318_i64 & (-626361265250022786_i64);
(*_6) = _20 as f64;
_22 = _19 as f64;
(*_6) = RET as f64;
_10 = (-17721746780998655668124329155728980925_i128) - (-134144005435923153123760339491734378077_i128);
_20 = _8 as i64;
(*_6) = 19878_i16 as f64;
(*_6) = 12793650104125993802_u64 as f64;
_3 = Move(_2);
_2 = Move(_3);
_24 = 17849106510297119492_u64 ^ 1181790260431754163_u64;
_26 = [_7,_7];
(*_6) = _24 as f64;
(*_6) = 3839785053962811599_usize as f64;
_23 = _14;
(*_6) = _19 as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 as f64;
(*_6) = _10 as f64;
(*_6) = _20 as f64;
(*_6) = 225189842196510223325626285720936175788_u128 as f64;
_13 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
_27 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
_24 = 3891897724269894621_u64;
(*_6) = 22259_i16 as f64;
_22 = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_19 = 216_u8 | 160_u8;
_14 = Field::<char>(Variant(_17.0, 0), 1);
match _24 {
0 => bb14,
1 => bb12,
2 => bb3,
3 => bb4,
4 => bb15,
5 => bb6,
3891897724269894621 => bb19,
_ => bb18
}
}
bb18 = {
_10 = 55261_u16 as i128;
_17.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _14 };
_8 = (-9223372036854775808_isize) as f32;
_17.0 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _14 };
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-5_isize) * (-9223372036854775808_isize);
_2 = Move(_3);
_8 = _7 as f32;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-38_isize) & 119_isize;
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
726224085 => bb15,
_ => bb14
}
}
bb19 = {
(*_6) = 0_usize as f64;
_10 = 149890636070410476467232718243285088703_i128;
_24 = 11830082298962858546_u64 >> _19;
(*_6) = _7 as f64;
(*_6) = 2236683222_u32 as f64;
(*_6) = 21708_u16 as f64;
match _10 {
0 => bb1,
1 => bb6,
2 => bb15,
3 => bb13,
149890636070410476467232718243285088703 => bb20,
_ => bb18
}
}
bb20 = {
_27 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
(*_6) = 11877_i16 as f64;
(*_6) = 3305124940_u32 as f64;
RET = (-17_i8) >> _7;
(*_6) = _8 as f64;
_27 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
Goto(bb21)
}
bb21 = {
_24 = 16127433421831551148_u64;
_20 = _24 as i64;
RET = 69_i8;
(*_6) = _7 as f64;
(*_6) = 22328_u16 as f64;
(*_6) = RET as f64;
(*_6) = _8 as f64;
(*_6) = _19 as f64;
_22 = _24 as f64;
(*_6) = 64243_u16 as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = 168324380882310424744642970810852274423_u128 as f64;
_6 = core::ptr::addr_of!((*_6));
_19 = 209_u8 - 233_u8;
Call(_13 = fn8(Move(_17.0), Move(_1), _23, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
(*_6) = 40766_u16 as f64;
(*_6) = _24 as f64;
(*_6) = 64240929855854879556654794316104408540_u128 as f64;
(*_6) = _7 as f64;
Goto(bb23)
}
bb23 = {
(*_6) = _7 as f64;
(*_6) = _24 as f64;
_20 = 3060951357748484137_i64;
RET = -123_i8;
(*_6) = _20 as f64;
_10 = _24 as i128;
(*_6) = _20 as f64;
Goto(bb24)
}
bb24 = {
(*_6) = _24 as f64;
(*_6) = 7_usize as f64;
(*_6) = _20 as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = 3370841377_u32 as f64;
_31 = !true;
(*_6) = 1929649704_u32 as f64;
match _20 {
0 => bb25,
1 => bb26,
2 => bb27,
3 => bb28,
4 => bb29,
5 => bb30,
3060951357748484137 => bb32,
_ => bb31
}
}
bb25 = {
_3 = Move(_2);
RET = 117_i8;
_8 = 6_usize as f32;
_2 = Move(_3);
_8 = 401276490334669053_usize as f32;
match _10 {
0 => bb1,
1 => bb2,
2 => bb5,
203196405605141666423501824597256086588 => bb7,
_ => bb4
}
}
bb26 = {
(*_6) = 40766_u16 as f64;
(*_6) = _24 as f64;
(*_6) = 64240929855854879556654794316104408540_u128 as f64;
(*_6) = _7 as f64;
Goto(bb23)
}
bb27 = {
_24 = 16127433421831551148_u64;
_20 = _24 as i64;
RET = 69_i8;
(*_6) = _7 as f64;
(*_6) = 22328_u16 as f64;
(*_6) = RET as f64;
(*_6) = _8 as f64;
(*_6) = _19 as f64;
_22 = _24 as f64;
(*_6) = 64243_u16 as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = 168324380882310424744642970810852274423_u128 as f64;
_6 = core::ptr::addr_of!((*_6));
_19 = 209_u8 - 233_u8;
Call(_13 = fn8(Move(_17.0), Move(_1), _23, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb28 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb29 = {
_10 = 55261_u16 as i128;
_17.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _14 };
_8 = (-9223372036854775808_isize) as f32;
_17.0 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _14 };
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-5_isize) * (-9223372036854775808_isize);
_2 = Move(_3);
_8 = _7 as f32;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-38_isize) & 119_isize;
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
726224085 => bb15,
_ => bb14
}
}
bb30 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb31 = {
RET = !(-12_i8);
_3 = Move(_2);
_2 = Move(_3);
RET = (-118_i8) >> (-337958849_i32);
_3 = Move(_2);
RET = (-2_i8);
RET = !(-102_i8);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
RET = (-105_i8) | 63_i8;
RET = (-90_i8) - (-127_i8);
_3 = Move(_2);
Goto(bb2)
}
bb32 = {
(*_6) = _7 as f64;
(*_6) = 20040_u16 as f64;
(*_6) = 244372287483328280103174051472821771376_u128 as f64;
(*_6) = RET as f64;
_14 = _23;
(*_6) = 8069695683448015258_usize as f64;
_10 = (-20464_i16) as i128;
_19 = 38_u8;
_7 = (-1842969555_i32) >> _20;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 as f64;
_21 = _26;
(*_6) = 15716094810939181527_usize as f64;
_21 = _26;
_26 = [_7,_7];
_28 = _31 ^ _31;
_10 = (-29625392489596213890949260056085802113_i128);
(*_6) = _8 as f64;
(*_6) = _19 as f64;
_27 = [9223372036854775807_isize,74_isize,(-69_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,77_isize];
(*_6) = 518358620_u32 as f64;
(*_6) = _8 as f64;
(*_6) = 1278476279_u32 as f64;
Goto(bb33)
}
bb33 = {
(*_6) = _24 as f64;
Goto(bb34)
}
bb34 = {
_26 = [_7,_7];
_36 = _31;
(*_6) = 52733_u16 as f64;
(*_6) = RET as f64;
_37 = [887923131_u32,1642281434_u32];
(*_6) = 3_usize as f64;
(*_6) = 4184896150_u32 as f64;
(*_6) = _10 as f64;
(*_6) = 1566_i16 as f64;
(*_6) = _19 as f64;
(*_6) = 0_usize as f64;
_8 = 17497298555194797489_usize as f32;
_17.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _14 };
_14 = _23;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = -(-9223372036854775808_isize);
(*_6) = 21893_u16 as f64;
(*_6) = _8 as f64;
(*_6) = _8 as f64;
match _24 {
0 => bb13,
1 => bb20,
2 => bb7,
16127433421831551148 => bb35,
_ => bb10
}
}
bb35 = {
_20 = 127041476739418116491124916478546783718_u128 as i64;
place!(Field::<char>(Variant(_17.0, 0), 1)) = _23;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = !(-124_isize);
_37 = [2518769060_u32,3239478854_u32];
_29 = _31;
match _10 {
0 => bb29,
1 => bb36,
2 => bb37,
310656974431342249572425347375682409343 => bb39,
_ => bb38
}
}
bb36 = {
place!(Field::<isize>(Variant(_17.0, 0), 0)) = 9223372036854775807_isize >> RET;
_3 = Move(_2);
_17.0 = Adt22::Variant0 { fld0: 76_isize,fld1: _14 };
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = Field::<char>(Variant(_17.0, 0), 1) as isize;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = 6692827080208532721_u64 as isize;
place!(Field::<char>(Variant(_17.0, 0), 1)) = _14;
_13 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0)];
Goto(bb9)
}
bb37 = {
_3 = Move(_2);
RET = 117_i8;
_8 = 6_usize as f32;
_2 = Move(_3);
_8 = 401276490334669053_usize as f32;
match _10 {
0 => bb1,
1 => bb2,
2 => bb5,
203196405605141666423501824597256086588 => bb7,
_ => bb4
}
}
bb38 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb39 = {
(*_6) = _10 as f64;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = !(-40_isize);
_20 = (-7826344941752780170_i64);
_10 = (-48676429274496291622065888300591274179_i128) ^ 139074172521055770623130202444644624747_i128;
_40 = _8 - _8;
match _20 {
0 => bb1,
1 => bb40,
2 => bb41,
340282366920938463455548262490015431286 => bb43,
_ => bb42
}
}
bb40 = {
_24 = 16127433421831551148_u64;
_20 = _24 as i64;
RET = 69_i8;
(*_6) = _7 as f64;
(*_6) = 22328_u16 as f64;
(*_6) = RET as f64;
(*_6) = _8 as f64;
(*_6) = _19 as f64;
_22 = _24 as f64;
(*_6) = 64243_u16 as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = 168324380882310424744642970810852274423_u128 as f64;
_6 = core::ptr::addr_of!((*_6));
_19 = 209_u8 - 233_u8;
Call(_13 = fn8(Move(_17.0), Move(_1), _23, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb41 = {
RET = (-70_i8) * 24_i8;
RET = (-36_i8) | 16_i8;
RET = (-86_i8) ^ (-67_i8);
_10 = (-137085961315796797039872782834512124868_i128);
match _10 {
0 => bb1,
1 => bb4,
203196405605141666423501824597256086588 => bb6,
_ => bb5
}
}
bb42 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb43 = {
_41 = Field::<isize>(Variant(_17.0, 0), 0);
_21 = _26;
_41 = Field::<isize>(Variant(_17.0, 0), 0) * Field::<isize>(Variant(_17.0, 0), 0);
_40 = _8 * _8;
(*_6) = RET as f64;
_36 = _28 & _29;
(*_6) = _7 as f64;
_7 = 1090441943_i32 & (-544783939_i32);
_38 = _10;
_13 = [_41,_41,_41];
(*_6) = _41 as f64;
(*_6) = _40 as f64;
_19 = 116_u8;
(*_6) = 17614976070689829178_usize as f64;
_13 = [Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),_41];
_38 = _10;
_44 = _19 as i32;
_10 = _38;
_44 = _7 << RET;
_45 = [_23,_14,_14,Field::<char>(Variant(_17.0, 0), 1),Field::<char>(Variant(_17.0, 0), 1),Field::<char>(Variant(_17.0, 0), 1),Field::<char>(Variant(_17.0, 0), 1),_14];
_8 = _40 * _40;
match _20 {
0 => bb24,
1 => bb2,
2 => bb42,
3 => bb19,
4 => bb39,
340282366920938463455548262490015431286 => bb44,
_ => bb6
}
}
bb44 = {
_37 = [1749060667_u32,3192189831_u32];
(*_6) = _19 as f64;
(*_6) = _44 as f64;
_47 = 0_usize >> Field::<isize>(Variant(_17.0, 0), 0);
(*_6) = _24 as f64;
_38 = _44 as i128;
(*_6) = 1507568371_u32 as f64;
(*_6) = _8 as f64;
_10 = _38;
_49 = -_40;
_31 = !_28;
_44 = _7 >> _41;
_13 = [_41,Field::<isize>(Variant(_17.0, 0), 0),_41];
(*_6) = 37028_u16 as f64;
(*_6) = 1070_u16 as f64;
_37 = [1551840750_u32,1025405447_u32];
(*_6) = _7 as f64;
_37 = [3205618977_u32,2039277282_u32];
match _19 {
0 => bb45,
116 => bb47,
_ => bb46
}
}
bb45 = {
_2 = Move(_3);
_3 = Move(_2);
RET = 126_i8 << 2888199002_u32;
_2 = Move(_3);
_3 = Move(_2);
_2 = Move(_3);
_7 = 538536240_i32 << RET;
RET = 12562713889195717796_u64 as i8;
_8 = RET as f32;
_3 = Move(_2);
_7 = 2098346912_i32;
_2 = Move(_3);
_8 = (-27374_i16) as f32;
Goto(bb3)
}
bb46 = {
(*_6) = 40766_u16 as f64;
(*_6) = _24 as f64;
(*_6) = 64240929855854879556654794316104408540_u128 as f64;
(*_6) = _7 as f64;
Goto(bb23)
}
bb47 = {
place!(Field::<isize>(Variant(_17.0, 0), 0)) = _41 * _41;
(*_6) = _44 as f64;
_41 = -Field::<isize>(Variant(_17.0, 0), 0);
(*_6) = 51509_u16 as f64;
_52 = _8;
(*_6) = _47 as f64;
match _24 {
0 => bb21,
1 => bb48,
2 => bb49,
16127433421831551148 => bb51,
_ => bb50
}
}
bb48 = {
_24 = 16127433421831551148_u64;
_20 = _24 as i64;
RET = 69_i8;
(*_6) = _7 as f64;
(*_6) = 22328_u16 as f64;
(*_6) = RET as f64;
(*_6) = _8 as f64;
(*_6) = _19 as f64;
_22 = _24 as f64;
(*_6) = 64243_u16 as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_6 = core::ptr::addr_of!((*_6));
(*_6) = 168324380882310424744642970810852274423_u128 as f64;
_6 = core::ptr::addr_of!((*_6));
_19 = 209_u8 - 233_u8;
Call(_13 = fn8(Move(_17.0), Move(_1), _23, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb49 = {
(*_6) = _24 as f64;
Goto(bb34)
}
bb50 = {
_10 = 55261_u16 as i128;
_17.0 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _14 };
_8 = (-9223372036854775808_isize) as f32;
_17.0 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _14 };
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-5_isize) * (-9223372036854775808_isize);
_2 = Move(_3);
_8 = _7 as f32;
place!(Field::<isize>(Variant(_17.0, 0), 0)) = (-38_isize) & 119_isize;
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
726224085 => bb15,
_ => bb14
}
}
bb51 = {
_47 = 2_usize;
_27 = [_13[_47],_41,_41,Field::<isize>(Variant(_17.0, 0), 0),Field::<isize>(Variant(_17.0, 0), 0),_41,_13[_47]];
(*_6) = 1895105630_u32 as f64;
_46 = !(-2825_i16);
(*_6) = RET as f64;
(*_6) = Field::<isize>(Variant(_17.0, 0), 0) as f64;
_44 = _7;
_38 = _10 + _10;
(*_6) = 4082401573_u32 as f64;
_21 = _26;
_55 = [163728007293932604691042929619951328542_u128,58686974208935792481431598137931423557_u128,306902258934488916989748125028376414420_u128,140945301276152837182301902343515028997_u128];
_55 = [35113779876943122484275821038413849024_u128,128339598219571849124510954411110030933_u128,59938045686988653504142445361161507845_u128,204466045583544349795450919424470831905_u128];
Goto(bb52)
}
bb52 = {
Call(_56 = dump_var(Move(_10), Move(_29), Move(_26), Move(_14)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_56 = dump_var(Move(_44), Move(_31), Move(_55), Move(_41)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_56 = dump_var(Move(_37), Move(_24), Move(_36), _57), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: Adt22,mut _2: *const [u64; 4],mut _3: char,mut _4: *mut Adt48) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _5: &'static mut (i32, *const usize);
let _6: *mut *const usize;
let _7: bool;
let _8: isize;
let _9: isize;
let _10: (Adt38, *const usize, &'static mut i32);
let _11: (i32, *const usize);
let _12: u32;
let _13: *mut *const usize;
let _14: f64;
let _15: *const u8;
let _16: Adt38;
let _17: [u128; 5];
let _18: Adt59;
let _19: *const &'static mut (i32, *const usize);
let _20: [char; 7];
let _21: Adt73;
let _22: &'static mut &'static mut (i128, &'static mut u128);
let _23: ();
let _24: ();
{
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize);
RET = [Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0)];
RET = [Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0)];
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize) + 9223372036854775807_isize;
_3 = Field::<char>(Variant(_1, 0), 1);
_1 = Adt22::Variant0 { fld0: (-125_isize),fld1: _3 };
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_3 = Field::<char>(Variant(_1, 0), 1);
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize) << 1744242941_u32;
RET = [Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0)];
_7 = Field::<isize>(Variant(_1, 0), 0) <= Field::<isize>(Variant(_1, 0), 0);
_1 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _3 };
_1 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _3 };
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_12 = 9223372036854775807_isize as u32;
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize) + 9223372036854775807_isize;
_11.0 = 386951928_i32 + 180587718_i32;
_10.2 = &mut _11.0;
Call(_3 = fn9(Move(_4), Move(_10.2), Move(_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _3 };
_8 = (-108_isize) >> _12;
_6 = core::ptr::addr_of_mut!(_10.1);
_12 = 4141890536_u32 | 1913104335_u32;
_6 = core::ptr::addr_of_mut!((*_6));
RET = [_8,_8,_8];
place!(Field::<isize>(Variant(_1, 0), 0)) = _8 + _8;
_7 = !false;
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
place!(Field::<isize>(Variant(_1, 0), 0)) = _8 + _8;
_14 = _12 as f64;
Goto(bb2)
}
bb2 = {
_9 = _14 as isize;
place!(Field::<isize>(Variant(_1, 0), 0)) = _9 * _9;
_1 = Adt22::Variant0 { fld0: _8,fld1: _3 };
place!(Field::<isize>(Variant(_1, 0), 0)) = _12 as isize;
_14 = _12 as f64;
_12 = 3021675678_u32 << _8;
place!(Field::<isize>(Variant(_1, 0), 0)) = _9;
Goto(bb3)
}
bb3 = {
_17 = [33162583357712989745061938824013393135_u128,230350041913440715871886147177093707361_u128,292676981576151813561813156237706135811_u128,8193047929740850361580559370460148219_u128,17935608947796827683289070933965958199_u128];
_17 = [17275495319505336609935003942273658188_u128,181831439156414020742966537835441877961_u128,158101051592568599230856144620879507247_u128,284182759907289216044767306148086906929_u128,316899807109413195204950571531388874744_u128];
RET = [_8,_8,_8];
_8 = Field::<isize>(Variant(_1, 0), 0) * Field::<isize>(Variant(_1, 0), 0);
_12 = 1735522728_u32;
_9 = Field::<isize>(Variant(_1, 0), 0) - _8;
_13 = core::ptr::addr_of_mut!((*_6));
_17 = [128207190541727633270990326222599351490_u128,230117421684497706899197818616120666443_u128,48623268471338838783704049672742498700_u128,265719099044024797241845993114899336355_u128,244780614433366101068683658453642723705_u128];
_13 = core::ptr::addr_of_mut!((*_13));
_14 = (-1128928472_i32) as f64;
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
place!(Field::<isize>(Variant(_1, 0), 0)) = _8 << _9;
_18.fld5 = [11697_u16,55388_u16,1173_u16,15562_u16];
_9 = _8 * Field::<isize>(Variant(_1, 0), 0);
_8 = Field::<isize>(Variant(_1, 0), 0) * Field::<isize>(Variant(_1, 0), 0);
_9 = Field::<isize>(Variant(_1, 0), 0) ^ _8;
Goto(bb4)
}
bb4 = {
place!(Field::<char>(Variant(_1, 0), 1)) = _3;
_18.fld1.0 = !48342_u16;
_18.fld2 = [_3,_3,_3,Field::<char>(Variant(_1, 0), 1),Field::<char>(Variant(_1, 0), 1),Field::<char>(Variant(_1, 0), 1),_3];
_18.fld1 = (44827_u16, 921792094_i32, Field::<char>(Variant(_1, 0), 1), 30621311571498912595648889254082574973_u128);
_18.fld5 = [_18.fld1.0,_18.fld1.0,_18.fld1.0,_18.fld1.0];
_18.fld1.1 = (-1535950345_i32) | (-257967347_i32);
_21.fld2 = [_18.fld1.1,_18.fld1.1];
_18.fld1.2 = Field::<char>(Variant(_1, 0), 1);
_21.fld4 = _12 as f32;
_8 = _9 | _9;
_21.fld5 = (-102135204094930438486456542137394590479_i128) as i32;
RET = [_8,_8,_8];
_12 = 2_usize as u32;
_12 = _8 as u32;
Goto(bb5)
}
bb5 = {
Call(_23 = dump_var(Move(_17), Move(_12), Move(_8), _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *mut Adt48,mut _2: &'static mut i32,mut _3: Adt22) -> char {
mir! {
type RET = char;
let _4: [u64; 4];
let _5: *const [u64; 4];
let _6: i64;
let _7: [u32; 3];
let _8: ([i64; 2], [u128; 5], &'static mut u128);
let _9: (Adt22, *mut (i128, &'static mut u128));
let _10: isize;
let _11: u64;
let _12: i16;
let _13: *const [u64; 4];
let _14: isize;
let _15: Adt28;
let _16: [isize; 3];
let _17: *mut (i128, &'static mut u128);
let _18: &'static mut (i128, &'static mut u128);
let _19: &'static u32;
let _20: *const (i32, *const usize);
let _21: f32;
let _22: *mut *const usize;
let _23: [i16; 4];
let _24: isize;
let _25: Adt28;
let _26: (Adt22, *mut (i128, &'static mut u128));
let _27: *const &'static mut (i128, &'static mut u128);
let _28: *const (i32, *const usize);
let _29: char;
let _30: f32;
let _31: isize;
let _32: f64;
let _33: (*mut [i64; 2],);
let _34: (u8, usize, isize);
let _35: i64;
let _36: &'static Adt25;
let _37: f64;
let _38: bool;
let _39: (Adt22, *mut (i128, &'static mut u128));
let _40: bool;
let _41: ((u8, usize, isize), &'static mut u128);
let _42: bool;
let _43: *const &'static mut (i32, *const usize);
let _44: usize;
let _45: *const f64;
let _46: f32;
let _47: [char; 8];
let _48: &'static Adt25;
let _49: u8;
let _50: u8;
let _51: *const &'static mut (i32, *const usize);
let _52: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _53: *const f64;
let _54: i128;
let _55: f32;
let _56: isize;
let _57: bool;
let _58: bool;
let _59: i8;
let _60: (u16, i32, char, u128);
let _61: u128;
let _62: ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22);
let _63: u64;
let _64: f32;
let _65: i128;
let _66: u64;
let _67: i16;
let _68: u8;
let _69: char;
let _70: i64;
let _71: &'static Adt25;
let _72: f64;
let _73: Adt59;
let _74: (u8, usize, isize);
let _75: *mut (i128, &'static mut u128);
let _76: *const &'static mut (i128, &'static mut u128);
let _77: (u8, usize, isize);
let _78: *const (i32, *const usize);
let _79: *const f64;
let _80: &'static mut &'static mut (i128, &'static mut u128);
let _81: &'static mut i32;
let _82: (&'static &'static u32, [u128; 6], char);
let _83: *mut Adt48;
let _84: i64;
let _85: *const f64;
let _86: [i32; 7];
let _87: *const &'static mut (i32, *const usize);
let _88: isize;
let _89: bool;
let _90: u16;
let _91: [u32; 5];
let _92: *mut *const usize;
let _93: i16;
let _94: *const &'static mut (i128, &'static mut u128);
let _95: [u128; 5];
let _96: isize;
let _97: &'static mut (i32, *const usize);
let _98: (i128, &'static mut u128);
let _99: char;
let _100: *mut Adt48;
let _101: (i128, &'static mut u128);
let _102: u32;
let _103: ();
let _104: ();
{
place!(Field::<isize>(Variant(_3, 0), 0)) = 1653_u16 as isize;
RET = Field::<char>(Variant(_3, 0), 1);
RET = Field::<char>(Variant(_3, 0), 1);
_4 = [15562278941678637543_u64,4182519112990981969_u64,4745256949765525837_u64,16636875405211263560_u64];
_3 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: RET };
RET = Field::<char>(Variant(_3, 0), 1);
RET = Field::<char>(Variant(_3, 0), 1);
_5 = core::ptr::addr_of!(_4);
(*_5) = [9383266584092135075_u64,14010873820267707615_u64,13420063843758352454_u64,5705833296606441721_u64];
_4 = [7389739628033734731_u64,17610176933937314788_u64,12165033395112078127_u64,8098942792542128426_u64];
place!(Field::<char>(Variant(_3, 0), 1)) = RET;
(*_5) = [11285783079086127754_u64,1455739278954119282_u64,13801251159327254512_u64,10866811520838996888_u64];
RET = Field::<char>(Variant(_3, 0), 1);
(*_5) = [14065068387174828674_u64,1616994730030245408_u64,15218805550137936239_u64,471764533798582585_u64];
Goto(bb1)
}
bb1 = {
place!(Field::<isize>(Variant(_3, 0), 0)) = (-9223372036854775808_isize) | 1_isize;
RET = Field::<char>(Variant(_3, 0), 1);
_6 = !(-4969311718748261514_i64);
(*_5) = [6761206713499528910_u64,8994125579409337581_u64,5466287195532026281_u64,5366223188498255060_u64];
(*_5) = [6803796079893227912_u64,4315172910076547691_u64,2803049607749492280_u64,14255220086189564145_u64];
Goto(bb2)
}
bb2 = {
(*_5) = [12008443955455598880_u64,8789532611837107557_u64,1152881810889116865_u64,1049716854082490552_u64];
(*_5) = [32269756893767867_u64,16020733137606888743_u64,5702208838699627238_u64,2009677410991533089_u64];
(*_5) = [12318008446688663321_u64,5378551456908508471_u64,11834980979613534723_u64,5079345496633685262_u64];
RET = Field::<char>(Variant(_3, 0), 1);
_8.1 = [335632827537029984335752110785265914440_u128,85513195890883882685385465584133971563_u128,279434355494118111711514697164133944693_u128,121285988042619980695448330194534390512_u128,66473260600109165272762280012516275385_u128];
(*_5) = [14772496595986643039_u64,15005712987399823294_u64,2709056403748004981_u64,13426016612237821294_u64];
(*_5) = [12789717916031149173_u64,14509956375932591711_u64,17221362963611644250_u64,2210505961810049085_u64];
(*_5) = [7574741257319471310_u64,8920838534454419396_u64,5842590183563638452_u64,2552784153305255603_u64];
(*_5) = [12629792638772490596_u64,6700448816303655287_u64,8066226870338880720_u64,1952862896686142753_u64];
(*_5) = [14673598390000919206_u64,9637066296747602368_u64,12992315202458293131_u64,8456214508492892324_u64];
_8.0 = [_6,_6];
_7 = [2479544425_u32,488933065_u32,3219634966_u32];
place!(Field::<isize>(Variant(_3, 0), 0)) = (-9223372036854775808_isize) >> _6;
(*_5) = [15313352700059922838_u64,11489380982447427059_u64,12197988697464457610_u64,7712998360782777415_u64];
(*_5) = [15910398293366011429_u64,9240804075942547502_u64,7093977630268773940_u64,9291267675558485373_u64];
_12 = -3427_i16;
_9.0 = Move(_3);
_11 = !9567137888520308189_u64;
_5 = core::ptr::addr_of!((*_5));
Call((*_5) = fn10(Move(_9.0), Move(_1), Move(_5), _6, _11, _8.0, _11, RET, RET, _8.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = [2112968081_u32,4119840173_u32,2767092937_u32];
_10 = (-169557121002910857273167299432207942848_i128) as isize;
_10 = (-124_isize) & 9223372036854775807_isize;
Goto(bb4)
}
bb4 = {
_7 = [2368535466_u32,673793284_u32,454674578_u32];
Goto(bb5)
}
bb5 = {
_16 = [_10,_10,_10];
_7 = [2877590377_u32,39465325_u32,2271018126_u32];
_6 = -2723946766954972221_i64;
_5 = core::ptr::addr_of!(_4);
_13 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
Goto(bb6)
}
bb6 = {
_5 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
_12 = 24268_i16 ^ (-28504_i16);
_8.0 = [_6,_6];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
RET = '\u{246a5}';
(*_5) = [_11,_11,_11,_11];
_13 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
_12 = (-23752_i16) * 15101_i16;
(*_5) = [_11,_11,_11,_11];
_12 = -(-25681_i16);
(*_5) = [_11,_11,_11,_11];
_14 = !_10;
_14 = 1488615111752347502_usize as isize;
RET = '\u{50031}';
(*_5) = [_11,_11,_11,_11];
_12 = 13142_i16 + (-4387_i16);
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
_16 = [_10,_10,_14];
(*_5) = [_11,_11,_11,_11];
Goto(bb7)
}
bb7 = {
RET = '\u{ca360}';
_6 = 387368459308447153_i64 >> _12;
_21 = _10 as f32;
Goto(bb8)
}
bb8 = {
_5 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
_13 = Move(_5);
_16 = [_10,_10,_14];
_23 = [_12,_12,_12,_12];
_21 = _6 as f32;
_12 = !19099_i16;
_7 = [1311168548_u32,2720220902_u32,3159740274_u32];
RET = '\u{ca956}';
_8.1 = [261925203377349395950523988729718217299_u128,181246417156393294217203280655766479602_u128,73558090663827861427286014859319097592_u128,189387230839263672227905003649534069664_u128,159557373116810428530051589691208582773_u128];
_5 = core::ptr::addr_of!(_4);
(*_5) = [_11,_11,_11,_11];
_11 = (-1796831932_i32) as u64;
(*_5) = [_11,_11,_11,_11];
_24 = _14 & _14;
_6 = (-8280641218467857253_i64);
RET = '\u{d5038}';
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
_24 = _10 | _14;
_26.0 = Adt22::Variant0 { fld0: _24,fld1: RET };
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
match _6 {
340282366920938463455093966213300354203 => bb10,
_ => bb9
}
}
bb9 = {
RET = '\u{ca360}';
_6 = 387368459308447153_i64 >> _12;
_21 = _10 as f32;
Goto(bb8)
}
bb10 = {
place!(Field::<char>(Variant(_26.0, 0), 1)) = RET;
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
_9.0 = Move(_26.0);
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
place!(Field::<char>(Variant(_9.0, 0), 1)) = RET;
_5 = Move(_13);
_3 = Move(_9.0);
RET = Field::<char>(Variant(_3, 0), 1);
_10 = _24 | _24;
_9.0 = Adt22::Variant0 { fld0: _24,fld1: RET };
_24 = Field::<isize>(Variant(_9.0, 0), 0);
_8.0 = [_6,_6];
_29 = Field::<char>(Variant(_9.0, 0), 1);
_24 = _10 >> _10;
place!(Field::<char>(Variant(_9.0, 0), 1)) = Field::<char>(Variant(_3, 0), 1);
_7 = [4278438411_u32,1935617645_u32,1509708370_u32];
_26.0 = Move(_9.0);
_21 = 156039029960176907163765237638952524609_u128 as f32;
_8.0 = [_6,_6];
_16 = [Field::<isize>(Variant(_26.0, 0), 0),_14,_10];
_13 = core::ptr::addr_of!(_4);
_8.1 = [244612184162908277492289042139369288828_u128,183289058969657806094772656136902020062_u128,36519986841162490987939033846018015113_u128,182574539152108910209142634340617981363_u128,66527426721229367942581800030449198018_u128];
(*_13) = [_11,_11,_11,_11];
Goto(bb11)
}
bb11 = {
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
place!(Field::<isize>(Variant(_26.0, 0), 0)) = true as isize;
(*_13) = [_11,_11,_11,_11];
_14 = _24 + _24;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_14 = 19802_u16 as isize;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_23 = [_12,_12,_12,_12];
_6 = 2206275309355193687_i64;
_8.0 = [_6,_6];
_6 = (-1251155220079852355_i64) - 4864728585525404069_i64;
_21 = 198_u8 as f32;
_21 = 159558014612612471325687582502753020351_i128 as f32;
(*_13) = [_11,_11,_11,_11];
place!(Field::<isize>(Variant(_3, 0), 0)) = Field::<isize>(Variant(_26.0, 0), 0) + _24;
(*_13) = [_11,_11,_11,_11];
_14 = _21 as isize;
(*_13) = [_11,_11,_11,_11];
_8.1 = [226435020243341751188083837447225376443_u128,221839512586216475826891594790644629407_u128,185154707949198258499916511094876620245_u128,94763051207011019447831679952579309905_u128,147154327437125785939517432089419330515_u128];
(*_13) = [_11,_11,_11,_11];
_10 = _24;
(*_13) = [_11,_11,_11,_11];
_9.0 = Move(_26.0);
(*_13) = [_11,_11,_11,_11];
Goto(bb12)
}
bb12 = {
place!(Field::<isize>(Variant(_3, 0), 0)) = _24 * _24;
_34 = (251_u8, 7_usize, _10);
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_31 = _10 & _24;
(*_13) = [_11,_11,_11,_11];
Goto(bb13)
}
bb13 = {
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_9.0 = Adt22::Variant0 { fld0: _24,fld1: Field::<char>(Variant(_3, 0), 1) };
(*_13) = [_11,_11,_11,_11];
place!(Field::<char>(Variant(_9.0, 0), 1)) = Field::<char>(Variant(_3, 0), 1);
_30 = -_21;
_6 = _34.1 as i64;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_37 = 2888_u16 as f64;
_24 = -_10;
RET = _29;
RET = Field::<char>(Variant(_9.0, 0), 1);
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_21 = _30 * _30;
_32 = 204501319069486672535778590853013599633_u128 as f64;
_24 = !Field::<isize>(Variant(_3, 0), 0);
_24 = !Field::<isize>(Variant(_9.0, 0), 0);
_21 = _30 + _30;
place!(Field::<isize>(Variant(_9.0, 0), 0)) = Field::<isize>(Variant(_3, 0), 0) & _10;
_23 = [_12,_12,_12,_12];
Goto(bb14)
}
bb14 = {
place!(Field::<char>(Variant(_3, 0), 1)) = _29;
(*_13) = [_11,_11,_11,_11];
_32 = _37 * _37;
(*_13) = [_11,_11,_11,_11];
_12 = -(-26326_i16);
_41.0.0 = _34.0;
_4 = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_9.0 = Adt22::Variant0 { fld0: _34.2,fld1: Field::<char>(Variant(_3, 0), 1) };
(*_13) = [_11,_11,_11,_11];
_34.2 = _34.1 as isize;
_34.1 = _24 as usize;
_33.0 = core::ptr::addr_of_mut!(_8.0);
_34.2 = _31;
(*_13) = [_11,_11,_11,_11];
_31 = (-155924259513517502271760220484696419638_i128) as isize;
_41.0.0 = _34.0 / _34.0;
place!(Field::<char>(Variant(_3, 0), 1)) = _29;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
place!(Field::<isize>(Variant(_3, 0), 0)) = _34.2 | Field::<isize>(Variant(_9.0, 0), 0);
_41.0.0 = !_34.0;
_41.0.2 = _12 as isize;
_26.0 = Move(_3);
_29 = RET;
_39.0 = Adt22::Variant0 { fld0: _24,fld1: RET };
_26.0 = Move(_39.0);
Goto(bb15)
}
bb15 = {
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_4 = [_11,_11,_11,_11];
_41.0.1 = !_34.1;
_12 = 11284_i16;
_3 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_9.0, 0), 0),fld1: _29 };
_44 = 833080150_i32 as usize;
_11 = 4898140336286827146_u64;
_46 = _21 - _21;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_13 = core::ptr::addr_of!((*_13));
(*_13) = [_11,_11,_11,_11];
_8.0 = [_6,_6];
(*_13) = [_11,_11,_11,_11];
Goto(bb16)
}
bb16 = {
_45 = core::ptr::addr_of!(_32);
(*_13) = [_11,_11,_11,_11];
Goto(bb17)
}
bb17 = {
_40 = _46 != _46;
_34 = (_41.0.0, _41.0.1, Field::<isize>(Variant(_9.0, 0), 0));
_34.1 = 11_i8 as usize;
place!(Field::<isize>(Variant(_3, 0), 0)) = _10 << _10;
(*_13) = [_11,_11,_11,_11];
_41.0 = (_34.0, _34.1, Field::<isize>(Variant(_26.0, 0), 0));
(*_13) = [_11,_11,_11,_11];
_47 = [RET,_29,Field::<char>(Variant(_3, 0), 1),Field::<char>(Variant(_3, 0), 1),RET,Field::<char>(Variant(_9.0, 0), 1),Field::<char>(Variant(_9.0, 0), 1),_29];
_32 = _37 - _37;
(*_45) = _37 - _37;
_39.0 = Move(_3);
_3 = Move(_39.0);
(*_13) = [_11,_11,_11,_11];
Goto(bb18)
}
bb18 = {
_41.0.2 = !_10;
_44 = !_41.0.1;
(*_13) = [_11,_11,_11,_11];
_47 = [Field::<char>(Variant(_9.0, 0), 1),Field::<char>(Variant(_9.0, 0), 1),Field::<char>(Variant(_26.0, 0), 1),Field::<char>(Variant(_3, 0), 1),Field::<char>(Variant(_26.0, 0), 1),RET,Field::<char>(Variant(_26.0, 0), 1),Field::<char>(Variant(_26.0, 0), 1)];
(*_45) = -_37;
_34.2 = Field::<isize>(Variant(_26.0, 0), 0) + Field::<isize>(Variant(_26.0, 0), 0);
(*_45) = _37 * _37;
_44 = _34.1 ^ _41.0.1;
_40 = false ^ false;
_35 = _6 + _6;
(*_13) = [_11,_11,_11,_11];
(*_45) = -_37;
(*_13) = [_11,_11,_11,_11];
(*_45) = -_37;
_49 = _41.0.0;
Goto(bb19)
}
bb19 = {
(*_45) = _37 * _37;
(*_13) = [_11,_11,_11,_11];
place!(Field::<char>(Variant(_3, 0), 1)) = Field::<char>(Variant(_9.0, 0), 1);
(*_45) = 101395679650379899293762902932829227272_i128 as f64;
_52.1 = _29;
(*_13) = [_11,_11,_11,_11];
_45 = core::ptr::addr_of!((*_45));
_37 = (*_45);
_12 = !14311_i16;
match _11 {
0 => bb15,
1 => bb2,
2 => bb6,
3 => bb10,
4898140336286827146 => bb20,
_ => bb5
}
}
bb20 = {
_34.1 = _44 + _44;
(*_45) = _37 + _37;
place!(Field::<isize>(Variant(_3, 0), 0)) = _34.2 ^ _41.0.2;
(*_13) = [_11,_11,_11,_11];
_57 = Field::<isize>(Variant(_3, 0), 0) >= _34.2;
_8.0 = [_35,_6];
(*_45) = _37;
(*_13) = [_11,_11,_11,_11];
_52.1 = Field::<char>(Variant(_9.0, 0), 1);
match _11 {
0 => bb8,
4898140336286827146 => bb22,
_ => bb21
}
}
bb21 = {
_45 = core::ptr::addr_of!(_32);
(*_13) = [_11,_11,_11,_11];
Goto(bb17)
}
bb22 = {
_30 = _46 + _46;
(*_13) = [_11,_11,_11,_11];
_55 = _46;
_45 = core::ptr::addr_of!((*_45));
_11 = 3397825319875378397_u64;
_60.2 = Field::<char>(Variant(_26.0, 0), 1);
_60.2 = Field::<char>(Variant(_3, 0), 1);
_52.2 = _12 * _12;
(*_45) = -_37;
_14 = _10 - _10;
_58 = _57 & _57;
_50 = _41.0.0;
_60.2 = Field::<char>(Variant(_3, 0), 1);
_34.1 = !_44;
(*_13) = [_11,_11,_11,_11];
_5 = core::ptr::addr_of!((*_13));
_46 = _55;
place!(Field::<isize>(Variant(_3, 0), 0)) = _34.1 as isize;
place!(Field::<char>(Variant(_26.0, 0), 1)) = _29;
(*_45) = 10454_u16 as f64;
_35 = _6 << _24;
(*_13) = [_11,_11,_11,_11];
_53 = Move(_45);
place!(Field::<isize>(Variant(_26.0, 0), 0)) = _34.2 ^ _41.0.2;
_31 = _41.0.2 >> _41.0.0;
_58 = _24 < Field::<isize>(Variant(_26.0, 0), 0);
_40 = _57 & _57;
_60.0 = 68_i8 as u16;
match _11 {
0 => bb16,
1 => bb17,
2 => bb23,
3397825319875378397 => bb25,
_ => bb24
}
}
bb23 = {
_5 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
_12 = 24268_i16 ^ (-28504_i16);
_8.0 = [_6,_6];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
RET = '\u{246a5}';
(*_5) = [_11,_11,_11,_11];
_13 = core::ptr::addr_of!((*_5));
(*_5) = [_11,_11,_11,_11];
_12 = (-23752_i16) * 15101_i16;
(*_5) = [_11,_11,_11,_11];
_12 = -(-25681_i16);
(*_5) = [_11,_11,_11,_11];
_14 = !_10;
_14 = 1488615111752347502_usize as isize;
RET = '\u{50031}';
(*_5) = [_11,_11,_11,_11];
_12 = 13142_i16 + (-4387_i16);
(*_5) = [_11,_11,_11,_11];
(*_5) = [_11,_11,_11,_11];
_16 = [_10,_10,_14];
(*_5) = [_11,_11,_11,_11];
Goto(bb7)
}
bb24 = {
place!(Field::<isize>(Variant(_3, 0), 0)) = _24 * _24;
_34 = (251_u8, 7_usize, _10);
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_31 = _10 & _24;
(*_13) = [_11,_11,_11,_11];
Goto(bb13)
}
bb25 = {
(*_13) = [_11,_11,_11,_11];
_24 = 72187556231815811938683843694575410298_u128 as isize;
_8.0 = [_35,_35];
_38 = _34.0 == _41.0.0;
_46 = -_55;
match _11 {
0 => bb10,
1 => bb20,
2 => bb3,
3 => bb8,
4 => bb22,
5 => bb12,
6 => bb26,
3397825319875378397 => bb28,
_ => bb27
}
}
bb26 = {
_34.1 = _44 + _44;
(*_45) = _37 + _37;
place!(Field::<isize>(Variant(_3, 0), 0)) = _34.2 ^ _41.0.2;
(*_13) = [_11,_11,_11,_11];
_57 = Field::<isize>(Variant(_3, 0), 0) >= _34.2;
_8.0 = [_35,_6];
(*_45) = _37;
(*_13) = [_11,_11,_11,_11];
_52.1 = Field::<char>(Variant(_9.0, 0), 1);
match _11 {
0 => bb8,
4898140336286827146 => bb22,
_ => bb21
}
}
bb27 = {
place!(Field::<char>(Variant(_3, 0), 1)) = _29;
(*_13) = [_11,_11,_11,_11];
_32 = _37 * _37;
(*_13) = [_11,_11,_11,_11];
_12 = -(-26326_i16);
_41.0.0 = _34.0;
_4 = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_9.0 = Adt22::Variant0 { fld0: _34.2,fld1: Field::<char>(Variant(_3, 0), 1) };
(*_13) = [_11,_11,_11,_11];
_34.2 = _34.1 as isize;
_34.1 = _24 as usize;
_33.0 = core::ptr::addr_of_mut!(_8.0);
_34.2 = _31;
(*_13) = [_11,_11,_11,_11];
_31 = (-155924259513517502271760220484696419638_i128) as isize;
_41.0.0 = _34.0 / _34.0;
place!(Field::<char>(Variant(_3, 0), 1)) = _29;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
place!(Field::<isize>(Variant(_3, 0), 0)) = _34.2 | Field::<isize>(Variant(_9.0, 0), 0);
_41.0.0 = !_34.0;
_41.0.2 = _12 as isize;
_26.0 = Move(_3);
_29 = RET;
_39.0 = Adt22::Variant0 { fld0: _24,fld1: RET };
_26.0 = Move(_39.0);
Goto(bb15)
}
bb28 = {
_61 = 231364418099806618097095702911067957411_u128 << _34.2;
(*_13) = [_11,_11,_11,_11];
_56 = _60.2 as isize;
_62.3 = Move(_3);
_60 = (32801_u16, 14746172_i32, RET, _61);
_14 = !_31;
_3 = Move(_26.0);
_41.1 = &mut _60.3;
_53 = core::ptr::addr_of!(_37);
_41.0.2 = _34.1 as isize;
(*_53) = _32;
RET = Field::<char>(Variant(_62.3, 0), 1);
(*_53) = -_32;
_13 = core::ptr::addr_of!((*_13));
_42 = _58 ^ _57;
Goto(bb29)
}
bb29 = {
(*_53) = _32;
_41.1 = &mut _61;
(*_13) = [_11,_11,_11,_11];
_62.1.1 = core::ptr::addr_of!(_41.0.1);
(*_53) = _32 + _32;
Goto(bb30)
}
bb30 = {
(*_53) = _32;
match _11 {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb13,
4 => bb11,
5 => bb24,
6 => bb31,
3397825319875378397 => bb33,
_ => bb32
}
}
bb31 = {
_7 = [2112968081_u32,4119840173_u32,2767092937_u32];
_10 = (-169557121002910857273167299432207942848_i128) as isize;
_10 = (-124_isize) & 9223372036854775807_isize;
Goto(bb4)
}
bb32 = {
_45 = core::ptr::addr_of!(_32);
(*_13) = [_11,_11,_11,_11];
Goto(bb17)
}
bb33 = {
_62.1.1 = core::ptr::addr_of!(_44);
_62.1.1 = core::ptr::addr_of!(_44);
_35 = _6 ^ _6;
_62.2 = [126_i8,114_i8,96_i8,(-37_i8),(-23_i8),(-26_i8),97_i8];
(*_53) = _32;
place!(Field::<isize>(Variant(_9.0, 0), 0)) = _10 + _14;
(*_53) = _32 * _32;
_39.0 = Move(_62.3);
match _11 {
0 => bb32,
1 => bb24,
2 => bb9,
3397825319875378397 => bb34,
_ => bb27
}
}
bb34 = {
_4 = [_11,_11,_11,_11];
_63 = _11 & _11;
_41.0 = _34;
_6 = 210294760416899879577810844467295267003_u128 as i64;
(*_13) = [_63,_11,_63,_63];
_26.0 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_3, 0), 0),fld1: _52.1 };
(*_53) = _32;
_11 = _63 ^ _63;
_55 = -_30;
RET = _52.1;
Goto(bb35)
}
bb35 = {
_62.0 = [Field::<char>(Variant(_26.0, 0), 1),Field::<char>(Variant(_39.0, 0), 1),Field::<char>(Variant(_9.0, 0), 1),Field::<char>(Variant(_39.0, 0), 1),Field::<char>(Variant(_9.0, 0), 1),RET,Field::<char>(Variant(_39.0, 0), 1)];
_55 = _21;
Goto(bb36)
}
bb36 = {
_35 = _6 >> _41.0.0;
(*_13) = [_11,_11,_11,_63];
_52.2 = _12 + _12;
place!(Field::<isize>(Variant(_39.0, 0), 0)) = 598582568_u32 as isize;
_35 = _21 as i64;
_31 = Field::<isize>(Variant(_26.0, 0), 0) >> _34.2;
place!(Field::<char>(Variant(_26.0, 0), 1)) = _29;
_22 = core::ptr::addr_of_mut!(_62.1.1);
(*_13) = [_11,_63,_11,_11];
_34 = (_50, _41.0.1, Field::<isize>(Variant(_9.0, 0), 0));
_47 = [Field::<char>(Variant(_26.0, 0), 1),Field::<char>(Variant(_26.0, 0), 1),Field::<char>(Variant(_3, 0), 1),RET,Field::<char>(Variant(_39.0, 0), 1),Field::<char>(Variant(_3, 0), 1),Field::<char>(Variant(_9.0, 0), 1),Field::<char>(Variant(_39.0, 0), 1)];
(*_22) = core::ptr::addr_of!(_34.1);
(*_13) = [_63,_11,_11,_11];
_33.0 = core::ptr::addr_of_mut!(_8.0);
(*_22) = core::ptr::addr_of!(_41.0.1);
(*_22) = core::ptr::addr_of!(_41.0.1);
(*_22) = core::ptr::addr_of!(_34.1);
(*_53) = _32 + _32;
(*_13) = [_11,_11,_11,_11];
(*_22) = core::ptr::addr_of!(_44);
_49 = _46 as u8;
_67 = _52.2 & _52.2;
place!(Field::<isize>(Variant(_3, 0), 0)) = -_41.0.2;
(*_22) = core::ptr::addr_of!(_44);
Goto(bb37)
}
bb37 = {
_73.fld3.1 = core::ptr::addr_of!(_34.1);
_70 = _35 & _35;
(*_22) = core::ptr::addr_of!(_74.1);
(*_22) = core::ptr::addr_of!(_34.1);
(*_53) = -_32;
_52.1 = Field::<char>(Variant(_3, 0), 1);
_28 = core::ptr::addr_of!(_73.fld3);
(*_28).0 = -(-636561264_i32);
(*_13) = [_63,_11,_63,_11];
(*_28).0 = 551664692_i32 ^ 437505447_i32;
_74 = (_41.0.0, _41.0.1, _31);
(*_28).0 = (-1190626216_i32);
(*_28).0 = Field::<char>(Variant(_39.0, 0), 1) as i32;
Goto(bb38)
}
bb38 = {
_73.fld1 = (59985_u16, (*_28).0, Field::<char>(Variant(_26.0, 0), 1), 198951197007949349254193495078068297447_u128);
RET = Field::<char>(Variant(_3, 0), 1);
(*_28) = (_73.fld1.1, Move((*_22)));
(*_28).1 = core::ptr::addr_of!(_34.1);
(*_28).1 = core::ptr::addr_of!(_44);
(*_22) = core::ptr::addr_of!(_74.1);
_45 = core::ptr::addr_of!((*_53));
(*_45) = (-5_i8) as f64;
match _73.fld1.0 {
0 => bb5,
1 => bb2,
2 => bb21,
59985 => bb39,
_ => bb32
}
}
bb39 = {
(*_45) = _32 + _32;
_73.fld4 = _73.fld1.0 - _73.fld1.0;
_42 = _40 & _38;
_73.fld0 = core::ptr::addr_of_mut!(_8.0);
_34 = (_74.0, _44, Field::<isize>(Variant(_9.0, 0), 0));
(*_13) = [_11,_11,_63,_11];
(*_28).0 = _73.fld1.1;
_73.fld0 = core::ptr::addr_of_mut!(_8.0);
(*_28).0 = -_73.fld1.1;
(*_28).1 = core::ptr::addr_of!(_44);
(*_28).0 = _73.fld1.1 + _73.fld1.1;
(*_22) = Move((*_28).1);
(*_13) = [_11,_11,_63,_11];
match _73.fld1.3 {
198951197007949349254193495078068297447 => bb40,
_ => bb15
}
}
bb40 = {
(*_28).0 = _73.fld1.1;
_2 = &mut (*_28).0;
(*_2) = -(-1315149349_i32);
(*_45) = _32 + _32;
(*_28).1 = core::ptr::addr_of!(_74.1);
_7 = [4090905189_u32,1645144222_u32,864205013_u32];
Call(_6 = core::intrinsics::bswap(_70), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
_30 = -_46;
_42 = _40;
(*_2) = (-1626785600_i32) | 1067114685_i32;
(*_13) = [_11,_63,_11,_11];
(*_45) = _63 as f64;
_82.2 = _29;
(*_45) = _32;
_56 = 13215_u16 as isize;
_6 = _70 - _35;
_82.2 = Field::<char>(Variant(_39.0, 0), 1);
_23 = [_67,_67,_52.2,_67];
_52.2 = (*_2) as i16;
_62.1.2 = &mut (*_2);
_52.2 = -_12;
(*_22) = core::ptr::addr_of!(_44);
_44 = _41.0.1;
Goto(bb42)
}
bb42 = {
(*_13) = [_63,_11,_63,_11];
_4 = [_11,_63,_11,_11];
_24 = -_31;
_67 = (*_45) as i16;
_52.1 = Field::<char>(Variant(_39.0, 0), 1);
(*_22) = core::ptr::addr_of!(_34.1);
(*_28).1 = core::ptr::addr_of!(_77.1);
(*_53) = _12 as f64;
_50 = _41.0.0 | _41.0.0;
_52.0 = Move(_62.1.2);
_41.0.1 = _34.1 + _34.1;
_58 = _21 == _30;
(*_22) = core::ptr::addr_of!(_74.1);
(*_22) = core::ptr::addr_of!(_34.1);
_82.2 = Field::<char>(Variant(_3, 0), 1);
(*_28).1 = core::ptr::addr_of!(_34.1);
(*_13) = [_11,_63,_11,_11];
_63 = (*_53) as u64;
(*_28).1 = core::ptr::addr_of!(_44);
_85 = core::ptr::addr_of!((*_53));
_63 = !_11;
_84 = _67 as i64;
Goto(bb43)
}
bb43 = {
(*_85) = _32;
(*_85) = _32 + _32;
_57 = _42 <= _40;
_62.3 = Adt22::Variant1 { fld0: Move((*_28).1),fld1: _44 };
_8.1 = [142867548861476291439231956087652354390_u128,9702784486326241132246759457859448771_u128,52739810748301965983453442804090210842_u128,64362451954610416100087900114874065131_u128,206620190392360088361502907635032171473_u128];
(*_28).1 = core::ptr::addr_of!(_34.1);
_52.1 = Field::<char>(Variant(_39.0, 0), 1);
(*_85) = _32;
(*_22) = core::ptr::addr_of!(_74.1);
_47 = [Field::<char>(Variant(_9.0, 0), 1),_82.2,Field::<char>(Variant(_26.0, 0), 1),_29,_82.2,Field::<char>(Variant(_9.0, 0), 1),_52.1,_52.1];
_62.3 = Adt22::Variant1 { fld0: Move(_62.1.1),fld1: _74.1 };
_9.0 = Adt22::Variant1 { fld0: Move(Field::<*const usize>(Variant(_62.3, 1), 0)),fld1: Field::<usize>(Variant(_62.3, 1), 1) };
(*_22) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_62.3, 1), 1)));
(*_28).1 = core::ptr::addr_of!(_74.1);
_70 = _35 & _6;
_50 = _70 as u8;
(*_85) = _30 as f64;
_34.2 = _41.0.2 >> Field::<isize>(Variant(_3, 0), 0);
(*_85) = _32 + _32;
_81 = Move(_2);
(*_28).1 = core::ptr::addr_of!(_41.0.1);
_62.3 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_26.0, 0), 0),fld1: Field::<char>(Variant(_39.0, 0), 1) };
place!(Field::<isize>(Variant(_3, 0), 0)) = _74.2 - _74.2;
(*_28).1 = core::ptr::addr_of!(_77.1);
_5 = core::ptr::addr_of!((*_13));
(*_85) = -_32;
Goto(bb44)
}
bb44 = {
_63 = _52.2 as u64;
place!(Field::<char>(Variant(_3, 0), 1)) = _82.2;
(*_53) = -_32;
_86 = [222068110_i32,55986485_i32,612205259_i32,1302666025_i32,(-1552534192_i32),1640332249_i32,(-1835725316_i32)];
_39.0 = Move(_3);
(*_22) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_9.0, 1), 1)));
(*_13) = [_11,_11,_63,_11];
_14 = _74.2 & Field::<isize>(Variant(_62.3, 0), 0);
_4 = [_11,_11,_11,_63];
_56 = _31 - Field::<isize>(Variant(_39.0, 0), 0);
(*_53) = _32;
RET = _29;
_78 = Move(_28);
_41.0 = (_34.0, Field::<usize>(Variant(_9.0, 1), 1), Field::<isize>(Variant(_62.3, 0), 0));
_77.0 = (-40_i8) as u8;
place!(Field::<isize>(Variant(_26.0, 0), 0)) = Field::<isize>(Variant(_39.0, 0), 0);
(*_13) = [_11,_11,_11,_11];
Goto(bb45)
}
bb45 = {
(*_13) = [_63,_63,_11,_11];
(*_53) = _32;
_77.1 = !_44;
(*_22) = Move(Field::<*const usize>(Variant(_9.0, 1), 0));
_52.1 = Field::<char>(Variant(_26.0, 0), 1);
_82.1 = [272695169835202484920598482408505294674_u128,150556752224604755732857627033258052556_u128,317936505413328309711311709462069724589_u128,89355153869712773402301242383707301369_u128,97417956280921899249255841797447888324_u128,221337783120435499357606822196186988973_u128];
_35 = _70;
(*_22) = core::ptr::addr_of!(_41.0.1);
(*_22) = core::ptr::addr_of!(_41.0.1);
place!(Field::<isize>(Variant(_26.0, 0), 0)) = 58732_u16 as isize;
_57 = _74.2 == _74.2;
(*_53) = _32;
_65 = 163190404901530413111491255145570281321_i128;
_8.0 = [_70,_35];
_5 = core::ptr::addr_of!((*_13));
_72 = _37 - (*_53);
_69 = Field::<char>(Variant(_62.3, 0), 1);
(*_5) = [_11,_63,_11,_11];
_64 = _21 * _30;
(*_13) = [_11,_11,_11,_11];
(*_53) = _72 + _32;
(*_53) = _32 + _32;
(*_22) = core::ptr::addr_of!(_74.1);
(*_22) = core::ptr::addr_of!(_77.1);
_21 = _35 as f32;
_63 = !_11;
_49 = !_41.0.0;
match _65 {
0 => bb1,
1 => bb21,
2 => bb30,
3 => bb19,
4 => bb35,
163190404901530413111491255145570281321 => bb46,
_ => bb44
}
}
bb46 = {
_7 = [471431545_u32,1521409632_u32,2921220204_u32];
_68 = (-102_i8) as u8;
(*_22) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_9.0, 1), 1)));
_77.2 = Field::<isize>(Variant(_62.3, 0), 0) - _34.2;
_77.0 = _74.0;
_57 = !_42;
_88 = -Field::<isize>(Variant(_62.3, 0), 0);
(*_53) = _32 + _72;
(*_22) = core::ptr::addr_of!(_34.1);
(*_22) = core::ptr::addr_of!(_41.0.1);
(*_13) = [_11,_63,_63,_11];
_53 = core::ptr::addr_of!((*_53));
_4 = [_63,_63,_63,_63];
(*_22) = core::ptr::addr_of!(_74.1);
place!(Field::<char>(Variant(_39.0, 0), 1)) = Field::<char>(Variant(_26.0, 0), 1);
_54 = RET as i128;
(*_53) = _72 * _72;
_84 = _54 as i64;
place!(Field::<char>(Variant(_26.0, 0), 1)) = RET;
match _65 {
0 => bb20,
1 => bb4,
2 => bb25,
3 => bb47,
163190404901530413111491255145570281321 => bb49,
_ => bb48
}
}
bb47 = {
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_9.0 = Adt22::Variant0 { fld0: _24,fld1: Field::<char>(Variant(_3, 0), 1) };
(*_13) = [_11,_11,_11,_11];
place!(Field::<char>(Variant(_9.0, 0), 1)) = Field::<char>(Variant(_3, 0), 1);
_30 = -_21;
_6 = _34.1 as i64;
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_37 = 2888_u16 as f64;
_24 = -_10;
RET = _29;
RET = Field::<char>(Variant(_9.0, 0), 1);
(*_13) = [_11,_11,_11,_11];
(*_13) = [_11,_11,_11,_11];
_21 = _30 * _30;
_32 = 204501319069486672535778590853013599633_u128 as f64;
_24 = !Field::<isize>(Variant(_3, 0), 0);
_24 = !Field::<isize>(Variant(_9.0, 0), 0);
_21 = _30 + _30;
place!(Field::<isize>(Variant(_9.0, 0), 0)) = Field::<isize>(Variant(_3, 0), 0) & _10;
_23 = [_12,_12,_12,_12];
Goto(bb14)
}
bb48 = {
_7 = [2112968081_u32,4119840173_u32,2767092937_u32];
_10 = (-169557121002910857273167299432207942848_i128) as isize;
_10 = (-124_isize) & 9223372036854775807_isize;
Goto(bb4)
}
bb49 = {
(*_22) = core::ptr::addr_of!(_77.1);
_35 = _6 | _70;
(*_22) = core::ptr::addr_of!(_77.1);
(*_53) = _41.0.0 as f64;
_74.0 = _34.0;
_77.2 = _54 as isize;
(*_22) = core::ptr::addr_of!(_34.1);
place!(Field::<char>(Variant(_26.0, 0), 1)) = _52.1;
_64 = _46 + _55;
_96 = _46 as isize;
_69 = Field::<char>(Variant(_26.0, 0), 1);
(*_13) = [_63,_63,_63,_11];
place!(Field::<isize>(Variant(_62.3, 0), 0)) = _24;
_63 = !_11;
Goto(bb50)
}
bb50 = {
Call(_103 = dump_var(Move(_70), Move(_88), Move(_57), Move(_42)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_103 = dump_var(Move(_23), Move(_69), Move(_63), Move(_7)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_103 = dump_var(Move(_31), Move(_34), Move(_50), Move(_67)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_103 = dump_var(Move(_77), Move(_11), Move(_16), Move(_58)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_103 = dump_var(Move(_38), Move(_6), Move(_60), _104), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: Adt22,mut _2: *mut Adt48,mut _3: *const [u64; 4],mut _4: i64,mut _5: u64,mut _6: [i64; 2],mut _7: u64,mut _8: char,mut _9: char,mut _10: [u128; 5]) -> [u64; 4] {
mir! {
type RET = [u64; 4];
let _11: i32;
let _12: [u128; 5];
let _13: *mut Adt48;
let _14: u8;
let _15: u8;
let _16: i128;
let _17: ([i64; 2], [u128; 5], &'static mut u128);
let _18: &'static mut u128;
let _19: f64;
let _20: f64;
let _21: *const (i32, *const usize);
let _22: *mut (i128, &'static mut u128);
let _23: bool;
let _24: [i128; 4];
let _25: u64;
let _26: i64;
let _27: f64;
let _28: i16;
let _29: (i32, *const usize);
let _30: &'static &'static u32;
let _31: Adt73;
let _32: &'static u32;
let _33: usize;
let _34: &'static mut &'static mut (i128, &'static mut u128);
let _35: *const usize;
let _36: bool;
let _37: &'static u32;
let _38: f64;
let _39: i32;
let _40: f64;
let _41: [i32; 7];
let _42: char;
let _43: isize;
let _44: f64;
let _45: [u32; 5];
let _46: bool;
let _47: &'static mut (i32, *const usize);
let _48: bool;
let _49: *mut *const usize;
let _50: f32;
let _51: &'static mut (i128, &'static mut u128);
let _52: Adt25;
let _53: [i128; 4];
let _54: u32;
let _55: *mut (i128, &'static mut u128);
let _56: i8;
let _57: usize;
let _58: [isize; 7];
let _59: *const (i32, *const usize);
let _60: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _61: isize;
let _62: i64;
let _63: f32;
let _64: *mut (&'static &'static u32, [u128; 6], char);
let _65: [u128; 6];
let _66: [i64; 2];
let _67: *const *const [u64; 4];
let _68: *const *const [u64; 4];
let _69: isize;
let _70: i32;
let _71: *mut *const usize;
let _72: ([i64; 2], [u128; 5], &'static mut u128);
let _73: isize;
let _74: isize;
let _75: *const usize;
let _76: i32;
let _77: f32;
let _78: *const &'static mut (i32, *const usize);
let _79: isize;
let _80: u64;
let _81: f64;
let _82: &'static u32;
let _83: *const f64;
let _84: i32;
let _85: u64;
let _86: ((i128, &'static mut u128),);
let _87: usize;
let _88: isize;
let _89: u8;
let _90: ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22);
let _91: &'static Adt25;
let _92: [u32; 2];
let _93: u8;
let _94: &'static u32;
let _95: char;
let _96: [u8; 4];
let _97: (i32, *const usize);
let _98: isize;
let _99: isize;
let _100: &'static &'static Adt25;
let _101: u16;
let _102: (i128, &'static mut u128);
let _103: (&'static &'static u32, [u128; 6], char);
let _104: (i32, *const usize);
let _105: ();
let _106: ();
{
_7 = _5 | _5;
_1 = Adt22::Variant0 { fld0: (-34_isize),fld1: _8 };
_11 = -2005469991_i32;
_5 = _7 - _7;
_3 = core::ptr::addr_of!(RET);
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_7,_5];
_9 = Field::<char>(Variant(_1, 0), 1);
(*_3) = [_5,_7,_5,_5];
(*_3) = [_5,_7,_5,_7];
(*_3) = [_5,_7,_5,_7];
(*_3) = [_5,_7,_7,_5];
_3 = core::ptr::addr_of!(RET);
(*_3) = [_5,_7,_5,_5];
(*_3) = [_5,_5,_7,_5];
(*_3) = [_7,_5,_7,_7];
_5 = _7;
(*_3) = [_5,_7,_5,_5];
_12 = [285174155341448247028534185346184569709_u128,39321777516631842765906055026936979799_u128,329659930999084833280251382084457116530_u128,193969062154718834470002293305344428203_u128,3000420394706300412977847355352057100_u128];
_6 = [_4,_4];
(*_3) = [_5,_5,_7,_7];
_13 = Move(_2);
(*_3) = [_5,_5,_7,_5];
Goto(bb1)
}
bb1 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb2 = {
_4 = 8152581085717902712_i64;
_14 = 123_u8 << _4;
_4 = 713260543515271398_i64 * 6686606539044773005_i64;
_2 = Move(_13);
place!(Field::<char>(Variant(_1, 0), 1)) = _9;
(*_3) = [_5,_7,_5,_5];
(*_3) = [_7,_5,_7,_5];
(*_3) = [_5,_5,_7,_7];
(*_3) = [_5,_5,_5,_5];
(*_3) = [_7,_7,_5,_5];
(*_3) = [_7,_7,_5,_7];
(*_3) = [_7,_7,_7,_7];
_16 = 45334312276773940925686388280971167747_u128 as i128;
(*_3) = [_5,_5,_7,_5];
(*_3) = [_7,_7,_5,_5];
_16 = (-12435391793569478278584993820244770578_i128);
_5 = !_7;
(*_3) = [_5,_7,_7,_5];
(*_3) = [_5,_5,_7,_7];
_17.1 = _10;
(*_3) = [_7,_7,_5,_7];
(*_3) = [_7,_5,_7,_7];
(*_3) = [_7,_7,_7,_7];
_20 = _14 as f64;
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
327846975127368985184789613611523440878 => bb7,
_ => bb6
}
}
bb3 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
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
_12 = _17.1;
(*_3) = [_5,_5,_7,_5];
(*_3) = [_7,_7,_7,_5];
(*_3) = [_7,_7,_5,_5];
_20 = 194529440388761509408800572186976548057_u128 as f64;
_13 = Move(_2);
_14 = !191_u8;
_12 = [277365627854236529919462344100448223572_u128,56882640205428648753355726850808543064_u128,195766550911367449634894432603431402557_u128,204367754633949256512217702706434064214_u128,156392622959202785841266724227103461033_u128];
RET = [_5,_7,_7,_7];
_2 = Move(_13);
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_7,_5,_7];
_6 = [_4,_4];
_17.0 = _6;
(*_3) = [_7,_5,_5,_5];
_11 = (-15064_i16) as i32;
_23 = true;
(*_3) = [_7,_5,_5,_7];
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize);
_16 = -133223142185498877494418281093862646535_i128;
(*_3) = [_7,_7,_7,_5];
(*_3) = [_7,_7,_5,_5];
(*_3) = [_7,_5,_5,_5];
(*_3) = [_5,_7,_7,_7];
_19 = _20 - _20;
match Field::<isize>(Variant(_1, 0), 0) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb11 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb12 = {
_4 = 8152581085717902712_i64;
_14 = 123_u8 << _4;
_4 = 713260543515271398_i64 * 6686606539044773005_i64;
_2 = Move(_13);
place!(Field::<char>(Variant(_1, 0), 1)) = _9;
(*_3) = [_5,_7,_5,_5];
(*_3) = [_7,_5,_7,_5];
(*_3) = [_5,_5,_7,_7];
(*_3) = [_5,_5,_5,_5];
(*_3) = [_7,_7,_5,_5];
(*_3) = [_7,_7,_5,_7];
(*_3) = [_7,_7,_7,_7];
_16 = 45334312276773940925686388280971167747_u128 as i128;
(*_3) = [_5,_5,_7,_5];
(*_3) = [_7,_7,_5,_5];
_16 = (-12435391793569478278584993820244770578_i128);
_5 = !_7;
(*_3) = [_5,_7,_7,_5];
(*_3) = [_5,_5,_7,_7];
_17.1 = _10;
(*_3) = [_7,_7,_5,_7];
(*_3) = [_7,_5,_7,_7];
(*_3) = [_7,_7,_7,_7];
_20 = _14 as f64;
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
327846975127368985184789613611523440878 => bb7,
_ => bb6
}
}
bb13 = {
_12 = [225091342036045694155370472966664971876_u128,29995202705558487876776344039762295821_u128,132874607375365085549772757120496256675_u128,216502649404318673526128210438604835130_u128,334546060844477822242377023330656705737_u128];
_12 = [327330931873255785784141975542216888687_u128,80310344041020586055502091239132178288_u128,260859684851436814003812782170052908046_u128,82938798557602860517904427573852252864_u128,13448933924384861304270214077855274166_u128];
(*_3) = [_7,_5,_5,_5];
_17.0 = [_4,_4];
_3 = core::ptr::addr_of!((*_3));
_25 = _7 - _7;
_19 = _20 + _20;
_10 = [169347312890729600807224129831666164080_u128,79881440754482367927822578698821641476_u128,197986665142605350128292146596013693128_u128,132813417728230872560727557744140277149_u128,133078767890847017540953213557853114333_u128];
_6 = _17.0;
(*_3) = [_25,_25,_5,_7];
(*_3) = [_25,_5,_5,_25];
(*_3) = [_5,_25,_25,_7];
_25 = _7 - _5;
_19 = _20;
_15 = _4 as u8;
match Field::<isize>(Variant(_1, 0), 0) {
0 => bb14,
1 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb14 = {
_12 = _17.1;
(*_3) = [_5,_5,_7,_5];
(*_3) = [_7,_7,_7,_5];
(*_3) = [_7,_7,_5,_5];
_20 = 194529440388761509408800572186976548057_u128 as f64;
_13 = Move(_2);
_14 = !191_u8;
_12 = [277365627854236529919462344100448223572_u128,56882640205428648753355726850808543064_u128,195766550911367449634894432603431402557_u128,204367754633949256512217702706434064214_u128,156392622959202785841266724227103461033_u128];
RET = [_5,_7,_7,_7];
_2 = Move(_13);
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_7,_5,_7];
_6 = [_4,_4];
_17.0 = _6;
(*_3) = [_7,_5,_5,_5];
_11 = (-15064_i16) as i32;
_23 = true;
(*_3) = [_7,_5,_5,_7];
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize);
_16 = -133223142185498877494418281093862646535_i128;
(*_3) = [_7,_7,_7,_5];
(*_3) = [_7,_7,_5,_5];
(*_3) = [_7,_5,_5,_5];
(*_3) = [_5,_7,_7,_7];
_19 = _20 - _20;
match Field::<isize>(Variant(_1, 0), 0) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_14 = _15 * _15;
_13 = Move(_2);
_8 = _9;
_3 = core::ptr::addr_of!((*_3));
_6 = [_4,_4];
(*_3) = [_7,_5,_5,_7];
_26 = _4 | _4;
_8 = _9;
(*_3) = [_5,_25,_25,_5];
(*_3) = [_5,_7,_25,_5];
_4 = _26 & _26;
_3 = core::ptr::addr_of!((*_3));
_9 = Field::<char>(Variant(_1, 0), 1);
_9 = Field::<char>(Variant(_1, 0), 1);
_17.1 = [241359494685815653161197030667310182623_u128,316905624634706936196355580385874592996_u128,266989046624413578400663254721270827167_u128,580679669798677186650846526982423601_u128,157040503902958545927290130853838602657_u128];
_27 = _19;
_15 = _14;
(*_3) = [_25,_25,_25,_25];
(*_3) = [_25,_25,_7,_25];
(*_3) = [_25,_5,_7,_7];
_9 = Field::<char>(Variant(_1, 0), 1);
(*_3) = [_25,_25,_25,_5];
_3 = core::ptr::addr_of!(RET);
_24 = [_16,_16,_16,_16];
_11 = (-241282231_i32);
_31.fld2 = [_11,_11];
Call(_1 = fn11(_8), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_3) = [_7,_7,_25,_25];
_25 = !_5;
_24 = [_16,_16,_16,_16];
RET = [_7,_5,_25,_25];
_13 = core::ptr::addr_of_mut!(_31.fld0);
_31.fld0 = Adt48::Variant0 { fld0: _16,fld1: Field::<char>(Variant(_1, 0), 1),fld2: _5,fld3: 1_i8 };
_31.fld4 = _11 as f32;
match _11 {
0 => bb19,
340282366920938463463374607431526929225 => bb21,
_ => bb20
}
}
bb19 = {
_14 = _15 * _15;
_13 = Move(_2);
_8 = _9;
_3 = core::ptr::addr_of!((*_3));
_6 = [_4,_4];
(*_3) = [_7,_5,_5,_7];
_26 = _4 | _4;
_8 = _9;
(*_3) = [_5,_25,_25,_5];
(*_3) = [_5,_7,_25,_5];
_4 = _26 & _26;
_3 = core::ptr::addr_of!((*_3));
_9 = Field::<char>(Variant(_1, 0), 1);
_9 = Field::<char>(Variant(_1, 0), 1);
_17.1 = [241359494685815653161197030667310182623_u128,316905624634706936196355580385874592996_u128,266989046624413578400663254721270827167_u128,580679669798677186650846526982423601_u128,157040503902958545927290130853838602657_u128];
_27 = _19;
_15 = _14;
(*_3) = [_25,_25,_25,_25];
(*_3) = [_25,_25,_7,_25];
(*_3) = [_25,_5,_7,_7];
_9 = Field::<char>(Variant(_1, 0), 1);
(*_3) = [_25,_25,_25,_5];
_3 = core::ptr::addr_of!(RET);
_24 = [_16,_16,_16,_16];
_11 = (-241282231_i32);
_31.fld2 = [_11,_11];
Call(_1 = fn11(_8), ReturnTo(bb18), UnwindUnreachable())
}
bb20 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb21 = {
place!(Field::<i8>(Variant(_31.fld0, 0), 3)) = 89_i8;
place!(Field::<i128>(Variant((*_13), 0), 0)) = _16 + _16;
_31.fld3 = [Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1)];
place!(Field::<i8>(Variant((*_13), 0), 3)) = !116_i8;
place!(Field::<u64>(Variant((*_13), 0), 2)) = _7 - _25;
_31.fld4 = _11 as f32;
place!(Field::<i128>(Variant((*_13), 0), 0)) = _16;
(*_3) = [Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),_25];
place!(Field::<char>(Variant((*_13), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
place!(Field::<i128>(Variant((*_13), 0), 0)) = -_16;
place!(Field::<i128>(Variant((*_13), 0), 0)) = _16 - _16;
place!(Field::<char>(Variant(_1, 0), 1)) = Field::<char>(Variant((*_13), 0), 1);
place!(Field::<char>(Variant((*_13), 0), 1)) = _9;
_31.fld3 = [Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant((*_13), 0), 1),Field::<char>(Variant(_31.fld0, 0), 1),Field::<char>(Variant((*_13), 0), 1)];
(*_3) = [Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2)];
place!(Field::<isize>(Variant(_1, 0), 0)) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_13) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _7,fld3: (-65_i8) };
place!(Field::<char>(Variant((*_13), 0), 1)) = _8;
place!(Field::<i8>(Variant((*_13), 0), 3)) = _4 as i8;
(*_13) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _25,fld3: (-36_i8) };
place!(Field::<i8>(Variant((*_13), 0), 3)) = 55_i8 ^ 20_i8;
_2 = core::ptr::addr_of_mut!((*_13));
_6 = [_4,_4];
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
Goto(bb22)
}
bb22 = {
place!(Field::<u64>(Variant((*_2), 0), 2)) = _7 & _5;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
place!(Field::<i128>(Variant((*_13), 0), 0)) = _16 - _16;
place!(Field::<i128>(Variant((*_13), 0), 0)) = !_16;
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
place!(Field::<i8>(Variant((*_13), 0), 3)) = -43_i8;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2)];
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2)];
_13 = core::ptr::addr_of_mut!((*_13));
_15 = _14;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
_33 = !7_usize;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2)];
place!(Field::<i8>(Variant((*_13), 0), 3)) = (-92_i8) * 126_i8;
_29.0 = _11 * _11;
place!(Field::<i128>(Variant((*_13), 0), 0)) = _29.0 as i128;
_31.fld2 = [_29.0,_29.0];
place!(Field::<u64>(Variant((*_13), 0), 2)) = !_25;
Goto(bb23)
}
bb23 = {
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
place!(Field::<isize>(Variant(_1, 0), 0)) = -(-51_isize);
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),_5,Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_5;
_31.fld3 = [Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1)];
place!(Field::<u64>(Variant((*_2), 0), 2)) = _7;
_15 = _31.fld4 as u8;
_10 = [133309554844829861893494844023468025884_u128,19070844670116335330607888917034815145_u128,179937346064161551237483033009218381217_u128,155674049343143424170502443643411882754_u128,96278342313421178941119876440629853177_u128];
_2 = core::ptr::addr_of_mut!(_31.fld0);
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_7;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _7;
place!(Field::<i8>(Variant((*_2), 0), 3)) = Field::<isize>(Variant(_1, 0), 0) as i8;
_31.fld3 = [Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1)];
_39 = -_29.0;
place!(Field::<i128>(Variant((*_2), 0), 0)) = -_16;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _26 as i128;
place!(Field::<u64>(Variant((*_2), 0), 2)) = 226396915_u32 as u64;
Call(place!(Field::<i8>(Variant((*_13), 0), 3)) = core::intrinsics::transmute(_15), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
place!(Field::<u64>(Variant((*_2), 0), 2)) = Field::<char>(Variant((*_2), 0), 1) as u64;
_43 = Field::<isize>(Variant(_1, 0), 0);
_42 = Field::<char>(Variant((*_2), 0), 1);
_46 = !_23;
_25 = Field::<u64>(Variant((*_2), 0), 2) << Field::<i128>(Variant((*_2), 0), 0);
_40 = _27;
_2 = core::ptr::addr_of_mut!(_31.fld0);
_50 = _43 as f32;
_13 = core::ptr::addr_of_mut!((*_2));
place!(Field::<u64>(Variant((*_13), 0), 2)) = _7 >> Field::<i128>(Variant((*_2), 0), 0);
(*_3) = [Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_13), 0), 2),Field::<u64>(Variant((*_13), 0), 2)];
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25 >> Field::<i8>(Variant((*_13), 0), 3);
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
_36 = Field::<i128>(Variant((*_13), 0), 0) >= Field::<i128>(Variant((*_2), 0), 0);
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
place!(Field::<i8>(Variant((*_13), 0), 3)) = (-44_i8) >> Field::<i128>(Variant((*_2), 0), 0);
place!(Field::<i128>(Variant((*_2), 0), 0)) = 261085055801825199337377241625906925641_u128 as i128;
place!(Field::<i8>(Variant((*_2), 0), 3)) = -(-4_i8);
_53 = _24;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 + _16;
match _11 {
0 => bb16,
1 => bb25,
2 => bb26,
3 => bb27,
340282366920938463463374607431526929225 => bb29,
_ => bb28
}
}
bb25 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb26 = {
_4 = (-2683688182666532191_i64) | (-7593273565520764305_i64);
_4 = (-6370899678654024781_i64) << _5;
(*_3) = [_5,_5,_5,_7];
_9 = _8;
(*_3) = [_7,_5,_5,_7];
_3 = core::ptr::addr_of!((*_3));
(*_3) = [_5,_5,_5,_7];
(*_3) = [_5,_5,_5,_7];
Goto(bb2)
}
bb27 = {
Return()
}
bb28 = {
Return()
}
bb29 = {
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant(_31.fld0, 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
_38 = _33 as f64;
_42 = Field::<char>(Variant((*_2), 0), 1);
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25;
place!(Field::<i128>(Variant((*_2), 0), 0)) = !_16;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_5;
place!(Field::<i8>(Variant((*_2), 0), 3)) = (-83_i8) << _4;
_41 = [_11,_29.0,_29.0,_39,_39,_29.0,_39];
_31.fld4 = _50 - _50;
_17.0 = [_4,_4];
_35 = core::ptr::addr_of!(_33);
_50 = 157233168350728573665048627622838356491_u128 as f32;
_57 = !(*_35);
_16 = Field::<i128>(Variant((*_2), 0), 0);
place!(Field::<char>(Variant((*_2), 0), 1)) = _42;
(*_35) = !_57;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_7;
_31.fld0 = Adt48::Variant0 { fld0: _16,fld1: Field::<char>(Variant(_1, 0), 1),fld2: _7,fld3: (-2_i8) };
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_7;
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 & _16;
_42 = Field::<char>(Variant((*_2), 0), 1);
_31.fld5 = _29.0 + _39;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_5;
Call(_42 = fn12(Move(_13), Move(_1), _24, Move(_2), (*_3), Move(_35), _29.0, Move(_3), Field::<char>(Variant(_31.fld0, 0), 1), Field::<char>(Variant((*_2), 0), 1), Field::<char>(Variant((*_2), 0), 1)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_5 = 58746_u16 as u64;
_60.1.2 = [(-14_i8),31_i8,12_i8,20_i8,(-61_i8),(-43_i8),86_i8];
_57 = !_33;
_2 = core::ptr::addr_of_mut!(_31.fld0);
place!(Field::<i128>(Variant(_31.fld0, 0), 0)) = _16;
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _42,fld2: _25,fld3: (-51_i8) };
place!(Field::<i8>(Variant((*_2), 0), 3)) = 61_i8 >> Field::<u64>(Variant((*_2), 0), 2);
_44 = _27 - _19;
_31.fld5 = _29.0;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25;
place!(Field::<i8>(Variant((*_2), 0), 3)) = (-55_i8) | (-4_i8);
_36 = _46 ^ _46;
place!(Field::<i8>(Variant((*_2), 0), 3)) = !(-126_i8);
place!(Field::<i8>(Variant((*_2), 0), 3)) = !52_i8;
Call(place!(Field::<i128>(Variant((*_2), 0), 0)) = core::intrinsics::transmute(_16), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_57 = _33 * _33;
place!(Field::<i8>(Variant((*_2), 0), 3)) = !56_i8;
_60.1.3 = Adt22::Variant0 { fld0: _43,fld1: _9 };
place!(Field::<i128>(Variant((*_2), 0), 0)) = _31.fld4 as i128;
_3 = core::ptr::addr_of!(RET);
_6 = [_26,_4];
_13 = core::ptr::addr_of_mut!((*_2));
place!(Field::<i8>(Variant((*_2), 0), 3)) = 46_i8 | (-10_i8);
place!(Field::<i128>(Variant((*_13), 0), 0)) = 55625416651437200522932603546011518961_u128 as i128;
place!(Field::<char>(Variant((*_13), 0), 1)) = _42;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _7 - _25;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 ^ _16;
RET = [Field::<u64>(Variant((*_2), 0), 2),_25,Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
_27 = _38;
place!(Field::<i128>(Variant(_31.fld0, 0), 0)) = _16 & _16;
_60.1.1.2 = &mut _31.fld5;
_28 = -7895_i16;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _7 ^ _7;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
place!(Field::<char>(Variant((*_13), 0), 1)) = _9;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_25;
_50 = Field::<isize>(Variant(_60.1.3, 0), 0) as f32;
Goto(bb32)
}
bb32 = {
_59 = core::ptr::addr_of!(_29);
match _11 {
0 => bb24,
1 => bb33,
340282366920938463463374607431526929225 => bb35,
_ => bb34
}
}
bb33 = {
Return()
}
bb34 = {
Return()
}
bb35 = {
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_25;
place!(Field::<char>(Variant((*_13), 0), 1)) = _8;
(*_59).1 = core::ptr::addr_of!(_33);
_60.1.2 = [Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3),Field::<i8>(Variant((*_2), 0), 3)];
Goto(bb36)
}
bb36 = {
_49 = core::ptr::addr_of_mut!((*_59).1);
_65 = [182584711605362747649469848397560115265_u128,228741265211010206325998923804312930572_u128,255272842455552380120851304542327747854_u128,90808767245300707116781092764883281354_u128,299646940210669882869956465255428158963_u128,106009806777647384974260225642075789848_u128];
_26 = !_4;
_65 = [63016569339794300509190249146817034654_u128,132141385204352274837080763586163719359_u128,235307329896084906811431850528993807804_u128,3998867714638151178041744107181015684_u128,99229443594406516032759529911971269552_u128,122700004534540128260852447516396760186_u128];
_58 = [Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0),_43,_43,Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0)];
place!(Field::<i8>(Variant((*_2), 0), 3)) = (-50_i8) ^ (-65_i8);
_11 = !(*_59).0;
(*_49) = core::ptr::addr_of!(_33);
_60.1.3 = Adt22::Variant0 { fld0: _43,fld1: Field::<char>(Variant((*_13), 0), 1) };
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 | _16;
_39 = !(*_59).0;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),_25];
(*_2) = Adt48::Variant0 { fld0: _16,fld1: Field::<char>(Variant(_60.1.3, 0), 1),fld2: _7,fld3: (-27_i8) };
Goto(bb37)
}
bb37 = {
_25 = !Field::<u64>(Variant((*_2), 0), 2);
place!(Field::<u64>(Variant((*_2), 0), 2)) = Field::<i128>(Variant((*_2), 0), 0) as u64;
_38 = _40 - _44;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
(*_49) = core::ptr::addr_of!(_33);
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_60.1.3, 0), 1);
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_60.1.3, 0), 1);
_56 = 105_i8 >> (*_59).0;
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _25,fld3: _56 };
_54 = 27737_u16 as u32;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),_25,_25,Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56;
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _42,fld2: _5,fld3: _56 };
(*_59).0 = !_39;
(*_49) = core::ptr::addr_of!(_57);
_1 = Adt22::Variant1 { fld0: Move((*_59).1),fld1: _57 };
_63 = _50 * _50;
Goto(bb38)
}
bb38 = {
(*_59).0 = _46 as i32;
_17.1 = [76238130510643183051044740161336509716_u128,251824262614366523289539639872470075168_u128,324750523171724198202436741379705858671_u128,113310309916527940562623852596772302082_u128,300585664669342366429014420250463762745_u128];
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 - _16;
_60.1.0 = [Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant((*_2), 0), 1),Field::<char>(Variant(_60.1.3, 0), 1),_8,_42];
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _25,fld3: _56 };
(*_49) = core::ptr::addr_of!(_33);
_40 = Field::<u64>(Variant((*_2), 0), 2) as f64;
(*_59) = (_39, Move(Field::<*const usize>(Variant(_1, 1), 0)));
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25 * _7;
(*_59).1 = core::ptr::addr_of!(_33);
_39 = !(*_59).0;
_17.0 = _6;
_42 = Field::<char>(Variant((*_2), 0), 1);
_49 = core::ptr::addr_of_mut!((*_59).1);
place!(Field::<char>(Variant((*_2), 0), 1)) = _42;
_12 = _17.1;
(*_59).1 = core::ptr::addr_of!(_33);
place!(Field::<i8>(Variant((*_2), 0), 3)) = -_56;
_8 = Field::<char>(Variant((*_2), 0), 1);
place!(Field::<i8>(Variant((*_2), 0), 3)) = _54 as i8;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 >> (*_59).0;
_66 = [_4,_26];
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_60.1.3, 0), 1);
place!(Field::<char>(Variant((*_2), 0), 1)) = _42;
(*_59).0 = -_39;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
_58 = [Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0),_43,Field::<isize>(Variant(_60.1.3, 0), 0),Field::<isize>(Variant(_60.1.3, 0), 0),_43];
Goto(bb39)
}
bb39 = {
(*_2) = Adt48::Variant0 { fld0: _16,fld1: Field::<char>(Variant(_60.1.3, 0), 1),fld2: _5,fld3: _56 };
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
(*_59).1 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1, 1), 1)));
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 * _56;
_70 = (*_59).0 - (*_59).0;
_20 = -_38;
place!(Field::<i8>(Variant((*_2), 0), 3)) = !_56;
_49 = core::ptr::addr_of_mut!((*_59).1);
_69 = Field::<isize>(Variant(_60.1.3, 0), 0) ^ Field::<isize>(Variant(_60.1.3, 0), 0);
_32 = &_54;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
_16 = Field::<i128>(Variant((*_2), 0), 0);
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_60.1.3, 0), 1);
_1 = Adt22::Variant0 { fld0: _69,fld1: _8 };
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 ^ _16;
place!(Field::<u64>(Variant((*_2), 0), 2)) = Field::<char>(Variant((*_2), 0), 1) as u64;
_58 = [Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),Field::<isize>(Variant(_1, 0), 0),_43,_69,_69];
_62 = _26 - _26;
_67 = core::ptr::addr_of!(_3);
place!(Field::<char>(Variant((*_2), 0), 1)) = _42;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 ^ _56;
(*_67) = core::ptr::addr_of!(RET);
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),_7,Field::<u64>(Variant((*_2), 0), 2)];
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _42,fld2: _7,fld3: _56 };
place!(Field::<char>(Variant((*_2), 0), 1)) = _42;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _40 as i128;
_47 = &mut (*_59);
_1 = Adt22::Variant0 { fld0: _69,fld1: Field::<char>(Variant((*_2), 0), 1) };
Goto(bb40)
}
bb40 = {
_63 = _50 - _50;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 << (*_47).0;
_72.0 = [_62,_62];
_27 = _40;
_6 = [_62,_4];
(*_47).0 = _70 * _70;
(*_47).1 = core::ptr::addr_of!(_57);
_46 = !_23;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _28 as u64;
_5 = (*_32) as u64;
_75 = core::ptr::addr_of!(_57);
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 << (*_75);
(*_47) = (_70, Move(_75));
(*_47).1 = core::ptr::addr_of!(_57);
_75 = core::ptr::addr_of!(_57);
(*_67) = core::ptr::addr_of!((*_3));
(*_47).0 = _70 - _39;
(*_47) = (_39, Move(_75));
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 - _56;
_62 = _26 | _26;
_37 = Move(_32);
(*_47).0 = !_70;
(*_47).1 = core::ptr::addr_of!(_57);
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
place!(Field::<i8>(Variant((*_2), 0), 3)) = 181288473918512985745738950390771076461_u128 as i8;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _26 as i8;
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25 - _5;
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16 << (*_47).0;
(*_47).1 = core::ptr::addr_of!(_57);
(*_47).1 = core::ptr::addr_of!(_33);
Call(_74 = core::intrinsics::transmute(_7), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
(*_47).1 = core::ptr::addr_of!(_57);
place!(Field::<i8>(Variant((*_2), 0), 3)) = !_56;
_60.1.3 = Adt22::Variant1 { fld0: Move((*_47).1),fld1: _57 };
(*_59).1 = core::ptr::addr_of!(_57);
(*_59).1 = core::ptr::addr_of!(_57);
(*_2) = Adt48::Variant3 { fld0: _16,fld1: Move(_49),fld2: 50237_u16 };
_37 = &_54;
_60.0 = !_25;
Goto(bb42)
}
bb42 = {
(*_3) = [_7,_25,_25,_25];
(*_3) = [_25,_7,_60.0,_5];
place!(Field::<u16>(Variant((*_2), 3), 2)) = 5531_u16;
(*_47).0 = _39 >> Field::<i128>(Variant((*_2), 3), 0);
_9 = _42;
place!(Field::<*mut *const usize>(Variant((*_2), 3), 1)) = core::ptr::addr_of_mut!(_60.1.1.1);
place!(Field::<u16>(Variant((*_2), 3), 2)) = 5569_u16 + 54218_u16;
Goto(bb43)
}
bb43 = {
place!(Field::<u16>(Variant((*_2), 3), 2)) = _63 as u16;
place!(Field::<u16>(Variant((*_2), 3), 2)) = 51677_u16 - 15218_u16;
place!(Field::<u16>(Variant((*_2), 3), 2)) = 9180_u16 - 14104_u16;
(*_3) = [_5,_7,_25,_60.0];
_39 = -(*_47).0;
Goto(bb44)
}
bb44 = {
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<i128>(Variant((*_2), 3), 0)) = _16 ^ _16;
place!(Field::<*mut *const usize>(Variant((*_2), 3), 1)) = core::ptr::addr_of_mut!((*_59).1);
_3 = core::ptr::addr_of!((*_3));
place!(Field::<u16>(Variant((*_2), 3), 2)) = !30099_u16;
place!(Field::<i128>(Variant((*_2), 3), 0)) = _16 & _16;
(*_3) = [_25,_60.0,_60.0,_7];
(*_3) = [_7,_25,_7,_7];
_56 = _42 as i8;
_20 = _27 + _40;
_60.1.3 = Adt22::Variant1 { fld0: Move((*_59).1),fld1: _33 };
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _7,fld3: _56 };
(*_59).1 = core::ptr::addr_of!(_57);
Call(place!(Field::<i8>(Variant((*_2), 0), 3)) = core::intrinsics::transmute(_56), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
_67 = core::ptr::addr_of!((*_67));
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 << Field::<isize>(Variant(_1, 0), 0);
place!(Field::<u64>(Variant((*_2), 0), 2)) = _60.0 ^ _7;
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
_26 = !_62;
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
place!(Field::<i128>(Variant((*_2), 0), 0)) = _16;
_17.0 = _6;
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<usize>(Variant(_60.1.3, 1), 1)) = !_57;
_30 = &_37;
(*_59).1 = core::ptr::addr_of!(_33);
_25 = Field::<u64>(Variant((*_2), 0), 2);
_39 = (*_47).0;
_9 = Field::<char>(Variant((*_2), 0), 1);
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 << (*_47).0;
_60.1.3 = Adt22::Variant0 { fld0: _69,fld1: Field::<char>(Variant((*_2), 0), 1) };
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
_14 = _15;
(*_59).1 = core::ptr::addr_of!(_57);
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_60.0;
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
place!(Field::<u64>(Variant((*_2), 0), 2)) = _25 << _70;
place!(Field::<i128>(Variant((*_2), 0), 0)) = -_16;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 - _56;
Goto(bb46)
}
bb46 = {
_24 = _53;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 + _56;
Goto(bb47)
}
bb47 = {
(*_59).1 = core::ptr::addr_of!(_33);
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_60.1.3, 0), 1);
(*_67) = core::ptr::addr_of!((*_3));
_52 = Adt25::Variant0 { fld0: _28,fld1: Move((*_59).1) };
(*_59).1 = Move(Field::<*const usize>(Variant(_52, 0), 1));
_65 = [265699119199093812365332177661783018967_u128,116155320399265126832118564941666823131_u128,125305268927279954998142464664208449827_u128,157376557031665265657455497709113689825_u128,92730588846408908357518273038140575020_u128,52647924723817378619723171758040366364_u128];
place!(Field::<char>(Variant((*_2), 0), 1)) = Field::<char>(Variant(_1, 0), 1);
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_25;
place!(Field::<i8>(Variant((*_2), 0), 3)) = _56 ^ _56;
_93 = _15;
_6 = _17.0;
Goto(bb48)
}
bb48 = {
_40 = 21552_u16 as f64;
_60.1.1.2 = &mut (*_47).0;
place!(Field::<isize>(Variant(_60.1.3, 0), 0)) = _43 + Field::<isize>(Variant(_1, 0), 0);
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
_60.1.3 = Adt22::Variant0 { fld0: _69,fld1: Field::<char>(Variant((*_2), 0), 1) };
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
_90.3 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_60.1.3, 0), 0),fld1: Field::<char>(Variant((*_2), 0), 1) };
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
_32 = &(*_37);
place!(Field::<char>(Variant((*_2), 0), 1)) = _8;
_25 = Field::<i128>(Variant((*_2), 0), 0) as u64;
place!(Field::<u64>(Variant((*_2), 0), 2)) = !_7;
(*_67) = core::ptr::addr_of!((*_3));
place!(Field::<char>(Variant((*_2), 0), 1)) = _9;
place!(Field::<i8>(Variant((*_2), 0), 3)) = !_56;
_30 = &_32;
(*_2) = Adt48::Variant0 { fld0: _16,fld1: _9,fld2: _7,fld3: _56 };
Goto(bb49)
}
bb49 = {
_28 = Field::<i16>(Variant(_52, 0), 0) >> (*_32);
(*_3) = [Field::<u64>(Variant((*_2), 0), 2),_60.0,Field::<u64>(Variant((*_2), 0), 2),Field::<u64>(Variant((*_2), 0), 2)];
_12 = _17.1;
(*_59).1 = core::ptr::addr_of!(_87);
_39 = _70;
_20 = _27 - _27;
_8 = Field::<char>(Variant(_1, 0), 1);
(*_67) = core::ptr::addr_of!((*_3));
Goto(bb50)
}
bb50 = {
Call(_105 = dump_var(Move(_6), Move(_25), Move(_16), Move(_69)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_105 = dump_var(Move(_62), Move(_65), Move(_57), Move(_58)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_105 = dump_var(Move(_74), Move(_11), Move(_10), Move(_28)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_105 = dump_var(Move(_93), Move(_46), Move(_4), Move(_26)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_105 = dump_var(Move(_8), Move(_5), _106, _106), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: char) -> Adt22 {
mir! {
type RET = Adt22;
let _2: *const *const [u64; 4];
let _3: char;
let _4: isize;
let _5: *mut (&'static &'static u32, [u128; 6], char);
let _6: &'static mut u128;
let _7: [i128; 4];
let _8: &'static &'static Adt25;
let _9: i128;
let _10: f32;
let _11: *const &'static mut (i128, &'static mut u128);
let _12: *const *const [u64; 4];
let _13: Adt28;
let _14: isize;
let _15: i64;
let _16: *mut (i128, &'static mut u128);
let _17: ();
let _18: ();
{
RET = Adt22::Variant0 { fld0: 31_isize,fld1: _1 };
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<isize>(Variant(RET, 0), 0)) = -(-9223372036854775808_isize);
_1 = Field::<char>(Variant(RET, 0), 1);
_1 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<isize>(Variant(RET, 0), 0)) = !61_isize;
place!(Field::<isize>(Variant(RET, 0), 0)) = (-9223372036854775808_isize);
_1 = Field::<char>(Variant(RET, 0), 1);
Goto(bb1)
}
bb1 = {
_1 = Field::<char>(Variant(RET, 0), 1);
RET = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _1 };
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
_1 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<isize>(Variant(RET, 0), 0)) = (-8615_i16) as isize;
_1 = Field::<char>(Variant(RET, 0), 1);
_1 = Field::<char>(Variant(RET, 0), 1);
_1 = Field::<char>(Variant(RET, 0), 1);
_1 = Field::<char>(Variant(RET, 0), 1);
Goto(bb2)
}
bb2 = {
_1 = Field::<char>(Variant(RET, 0), 1);
RET = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: _1 };
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<isize>(Variant(RET, 0), 0)) = (-78_isize) & 11_isize;
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
_3 = _1;
_1 = _3;
place!(Field::<isize>(Variant(RET, 0), 0)) = 61_i8 as isize;
place!(Field::<isize>(Variant(RET, 0), 0)) = !9223372036854775807_isize;
place!(Field::<isize>(Variant(RET, 0), 0)) = (-86_isize);
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
RET = Adt22::Variant0 { fld0: 114_isize,fld1: _3 };
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
place!(Field::<char>(Variant(RET, 0), 1)) = _1;
RET = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _3 };
place!(Field::<isize>(Variant(RET, 0), 0)) = !9223372036854775807_isize;
_3 = Field::<char>(Variant(RET, 0), 1);
_3 = Field::<char>(Variant(RET, 0), 1);
_3 = _1;
Goto(bb3)
}
bb3 = {
_3 = _1;
_1 = Field::<char>(Variant(RET, 0), 1);
_1 = Field::<char>(Variant(RET, 0), 1);
_4 = -Field::<isize>(Variant(RET, 0), 0);
place!(Field::<isize>(Variant(RET, 0), 0)) = _4;
_3 = _1;
place!(Field::<isize>(Variant(RET, 0), 0)) = _4;
_3 = Field::<char>(Variant(RET, 0), 1);
_1 = _3;
RET = Adt22::Variant0 { fld0: _4,fld1: _3 };
_1 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
place!(Field::<isize>(Variant(RET, 0), 0)) = !_4;
_3 = Field::<char>(Variant(RET, 0), 1);
_3 = Field::<char>(Variant(RET, 0), 1);
_3 = Field::<char>(Variant(RET, 0), 1);
_3 = _1;
_3 = _1;
Goto(bb4)
}
bb4 = {
place!(Field::<isize>(Variant(RET, 0), 0)) = _4 ^ _4;
_4 = Field::<isize>(Variant(RET, 0), 0);
_9 = (-27971_i16) as i128;
_10 = 202_u8 as f32;
_10 = 2_usize as f32;
_10 = 7446324420819950233_i64 as f32;
_7 = [_9,_9,_9,_9];
place!(Field::<isize>(Variant(RET, 0), 0)) = 73028797_u32 as isize;
_1 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<isize>(Variant(RET, 0), 0)) = !_4;
_7 = [_9,_9,_9,_9];
_4 = !Field::<isize>(Variant(RET, 0), 0);
_3 = _1;
_14 = _4 & Field::<isize>(Variant(RET, 0), 0);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
_3 = Field::<char>(Variant(RET, 0), 1);
_7 = [_9,_9,_9,_9];
_14 = Field::<isize>(Variant(RET, 0), 0) | Field::<isize>(Variant(RET, 0), 0);
RET = Adt22::Variant0 { fld0: _14,fld1: _3 };
place!(Field::<isize>(Variant(RET, 0), 0)) = _4 << _14;
_9 = 137162905394716822292158066861411267488_i128;
place!(Field::<isize>(Variant(RET, 0), 0)) = _14 >> _4;
_7 = [_9,_9,_9,_9];
_4 = _14;
Goto(bb5)
}
bb5 = {
_1 = _3;
_14 = Field::<isize>(Variant(RET, 0), 0) & Field::<isize>(Variant(RET, 0), 0);
place!(Field::<isize>(Variant(RET, 0), 0)) = _14;
place!(Field::<isize>(Variant(RET, 0), 0)) = _14 * _14;
_1 = _3;
_10 = 2885864526_u32 as f32;
_15 = (-2080242608279876456_i64);
Goto(bb6)
}
bb6 = {
Call(_17 = dump_var(Move(_3), Move(_15), Move(_9), _18), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: *mut Adt48,mut _2: Adt22,mut _3: [i128; 4],mut _4: *mut Adt48,mut _5: [u64; 4],mut _6: *const usize,mut _7: i32,mut _8: *const [u64; 4],mut _9: char,mut _10: char,mut _11: char) -> char {
mir! {
type RET = char;
let _12: char;
let _13: [u16; 4];
let _14: *mut (i128, &'static mut u128);
let _15: i8;
let _16: *mut *const usize;
let _17: isize;
let _18: isize;
let _19: *mut (&'static &'static u32, [u128; 6], char);
let _20: *const &'static mut (i128, &'static mut u128);
let _21: *mut (i128, &'static mut u128);
let _22: f32;
let _23: Adt22;
let _24: u8;
let _25: u64;
let _26: f64;
let _27: ([i64; 2], [u128; 5], &'static mut u128);
let _28: (*mut [i64; 2],);
let _29: ([i64; 2], [u128; 5], &'static mut u128);
let _30: &'static &'static u32;
let _31: *mut [i64; 2];
let _32: u64;
let _33: i8;
let _34: [i128; 4];
let _35: bool;
let _36: (&'static &'static u32, [u128; 6], char);
let _37: *const [u64; 4];
let _38: Adt28;
let _39: [isize; 5];
let _40: *mut (&'static &'static u32, [u128; 6], char);
let _41: bool;
let _42: f32;
let _43: &'static mut &'static mut (i128, &'static mut u128);
let _44: *const &'static mut (i128, &'static mut u128);
let _45: *const (i32, *const usize);
let _46: &'static mut (i32, *const usize);
let _47: [i32; 7];
let _48: &'static mut (i32, *const usize);
let _49: u128;
let _50: f64;
let _51: *mut *const usize;
let _52: isize;
let _53: char;
let _54: *mut [i64; 2];
let _55: (*mut [i64; 2],);
let _56: *const [u64; 4];
let _57: i32;
let _58: *const usize;
let _59: i8;
let _60: usize;
let _61: f32;
let _62: isize;
let _63: isize;
let _64: *mut *const usize;
let _65: *const (i32, *const usize);
let _66: isize;
let _67: &'static mut u128;
let _68: i16;
let _69: u16;
let _70: isize;
let _71: f64;
let _72: isize;
let _73: isize;
let _74: u32;
let _75: bool;
let _76: i128;
let _77: i16;
let _78: *const &'static mut (i128, &'static mut u128);
let _79: *const (i32, *const usize);
let _80: (u8, usize, isize);
let _81: i8;
let _82: [i16; 4];
let _83: ();
let _84: ();
{
RET = _9;
place!(Field::<char>(Variant(_2, 0), 1)) = _11;
place!(Field::<char>(Variant(_2, 0), 1)) = RET;
_5 = [689085761795624423_u64,15059574249489899049_u64,9217318043379764599_u64,15789419848511190113_u64];
_3 = [138542199098333660690568291529851551963_i128,95917158683958214924620650786091565042_i128,(-43992573509170409373692741192349444171_i128),(-113284696455162317497695836176603581593_i128)];
_12 = Field::<char>(Variant(_2, 0), 1);
_2 = Adt22::Variant0 { fld0: (-9223372036854775808_isize),fld1: RET };
_2 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: RET };
_2 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _10 };
_3 = [168379375106897225476834007594484459210_i128,(-36798690934481423753189253574391800811_i128),81490248500019087915652293606981240195_i128,127774588469101292203806468083781278109_i128];
_7 = -450998980_i32;
_5 = [7124326297551336146_u64,3890869059906384151_u64,8279753303619967687_u64,11276266722407885951_u64];
_13 = [60401_u16,56315_u16,21096_u16,24202_u16];
_9 = _11;
_5 = [8475640074610901151_u64,362760785296712306_u64,17722305464973381701_u64,11078671319794609574_u64];
Call(_6 = fn13(Move(_8), Move(_1), _13, _10, Move(_4), _12, _3, _5, _9, RET, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = 10954869201470590750_u64 as i8;
_3 = [142474130701399269797586116793753098035_i128,89065130442120953169087175528262312448_i128,(-82384967940469597936420415805792724588_i128),110348353039594349915698613979715472142_i128];
_10 = Field::<char>(Variant(_2, 0), 1);
Goto(bb2)
}
bb2 = {
_2 = Adt22::Variant0 { fld0: 9223372036854775807_isize,fld1: _9 };
_15 = -(-78_i8);
_3 = [46304070794362668545229663699548667470_i128,(-62517631804438888489292340795494431305_i128),(-110904932960386653948070940296419334293_i128),96983921038860560687434706935820422294_i128];
_3 = [52442812567691908893764843794563732095_i128,(-132094757879786007281829351529478839566_i128),(-116294662393857116962440830337107256207_i128),148303126248754927814047794537244081561_i128];
RET = _12;
RET = _9;
_3 = [89943679681643906869878195086286433012_i128,(-9913719446091783846912911226942606678_i128),(-124184102035785699657678874464868346459_i128),166984701499988835961959806659324241816_i128];
_10 = _9;
_8 = core::ptr::addr_of!(_5);
(*_8) = [11482798966862981578_u64,1794628716780784630_u64,8097207417255707145_u64,10878159292428457622_u64];
(*_8) = [14060031834935584075_u64,14273688983579342745_u64,7527217366875787891_u64,3968269350088941751_u64];
(*_8) = [16337636615611813617_u64,13482199138236404890_u64,4157653127101171258_u64,2556381732930387251_u64];
(*_8) = [13452736309057375566_u64,8276264662405158311_u64,16368318627216693293_u64,16681670487573548524_u64];
(*_8) = [303232102222945135_u64,7427944784409621705_u64,17761452170428590500_u64,16479923025020425776_u64];
(*_8) = [15669051950100040377_u64,12813222571155919831_u64,3109620310368173273_u64,5550563322323442711_u64];
(*_8) = [17350899549322708689_u64,4748556503664758365_u64,2686377281352724914_u64,11805908131955164355_u64];
_16 = core::ptr::addr_of_mut!(_6);
_10 = Field::<char>(Variant(_2, 0), 1);
(*_8) = [8158182283487265707_u64,17555016246945215512_u64,6710942498684910001_u64,10900224113355466226_u64];
(*_8) = [9403213479904128329_u64,16560966405380161847_u64,16707737786641815271_u64,4246906659530782075_u64];
Goto(bb3)
}
bb3 = {
(*_8) = [8260411302449044753_u64,16397996729600455319_u64,7090123969720708857_u64,10459111062380255798_u64];
Goto(bb4)
}
bb4 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb5 = {
(*_8) = [16019980935924784893_u64,6185828536329261166_u64,15010786339091579785_u64,2793881637615745103_u64];
(*_8) = [5633034868172798539_u64,5961378673961375925_u64,1713698759352004668_u64,8992245304720486598_u64];
_10 = Field::<char>(Variant(_2, 0), 1);
_3 = [(-82809489764229480833188965653667548253_i128),131337061933768677770558614628756292063_i128,114154787760495010083191983454210348922_i128,108070848135423461286205750792296190138_i128];
_16 = core::ptr::addr_of_mut!((*_16));
_16 = core::ptr::addr_of_mut!((*_16));
(*_8) = [10409728006806300303_u64,14764334429452661936_u64,3959227079729576144_u64,3030826258379634601_u64];
place!(Field::<char>(Variant(_2, 0), 1)) = _9;
(*_8) = [7015736465846920685_u64,7917989615038160420_u64,3525695268470747857_u64,8169044809523866260_u64];
_11 = _12;
(*_8) = [12342970281129924060_u64,15430790921040870563_u64,13363679698793282034_u64,13263141524527916726_u64];
_8 = core::ptr::addr_of!((*_8));
(*_8) = [11032805131952999416_u64,73846090717718250_u64,12923294909231889966_u64,3589074683625547986_u64];
_13 = [7164_u16,18322_u16,32032_u16,26978_u16];
(*_8) = [2602707416824242185_u64,6755556943563103468_u64,1616274543565664289_u64,4653994241849065474_u64];
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9223372036854775807 => bb7,
_ => bb6
}
}
bb6 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb7 = {
(*_8) = [7067916893708747360_u64,7080528655899946637_u64,1456636470060453889_u64,13922892651551791189_u64];
(*_8) = [12519881779995972856_u64,11884947757526284619_u64,12838392647100186935_u64,7794170424844263899_u64];
(*_8) = [4555042794423113584_u64,10451568293031599931_u64,2573768363776753814_u64,14004443377639946354_u64];
_15 = 121_i8 + 3_i8;
(*_8) = [10914517443363647162_u64,11590815299085361568_u64,7538219505927160267_u64,14504238963587801402_u64];
(*_8) = [10704896336515397558_u64,11239062265883865753_u64,5527195893847963063_u64,9212752856394221930_u64];
Goto(bb8)
}
bb8 = {
(*_8) = [12266771239292892573_u64,9912560477408032804_u64,17142397470382164621_u64,2417621551136654620_u64];
_3 = [(-113902219171842972086448702320525666523_i128),59082288090594069888953207142916194861_i128,(-27692546159624217403187662835840138185_i128),61305026094668867650342467983144567292_i128];
(*_8) = [6410167567815780232_u64,14038407708300752109_u64,8128034793073014736_u64,840320896073664686_u64];
(*_8) = [2835657370426786419_u64,4793443775860371598_u64,973641759390391238_u64,3066002372841574675_u64];
Goto(bb9)
}
bb9 = {
_24 = 76_u8 - 39_u8;
(*_8) = [7653741817995168664_u64,283426482563904720_u64,11572184653004552071_u64,2823445767163111947_u64];
_11 = _10;
RET = _10;
_10 = _9;
(*_8) = [6448418395058957100_u64,17650290424681921148_u64,2976518544238129087_u64,4311815020371435034_u64];
(*_8) = [2013397397434063658_u64,16313427745432600092_u64,11917044631720727773_u64,10663717248899360716_u64];
_3 = [(-61063672940962124995094535045796663212_i128),(-20988342003345797887347876235598023617_i128),91454151675357695071953413251385624058_i128,(-117236205913582829747372737691739034598_i128)];
_18 = Field::<isize>(Variant(_2, 0), 0) | Field::<isize>(Variant(_2, 0), 0);
match Field::<isize>(Variant(_2, 0), 0) {
9223372036854775807 => bb10,
_ => bb5
}
}
bb10 = {
_3 = [(-45089803448208514759408715216638427916_i128),135865250728113002819207518445218904399_i128,9526620427057688117670069241343741031_i128,86470369192014663973023386468725572462_i128];
_23 = Adt22::Variant0 { fld0: _18,fld1: _12 };
(*_8) = [16884110653605987357_u64,6316607217103277723_u64,5310050245871774144_u64,494479706672493242_u64];
_12 = _9;
_9 = _12;
(*_8) = [10777063397408193210_u64,9539868289352490598_u64,16887852381063024652_u64,3527096255594554944_u64];
_2 = Move(_23);
_5 = [14326806385186391837_u64,8399181515086624598_u64,18269488683748060219_u64,742638649500437869_u64];
(*_8) = [13237630164780650360_u64,1208254615922836912_u64,11343633984515951843_u64,11147376759533600300_u64];
_23 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_2, 0), 0),fld1: RET };
place!(Field::<isize>(Variant(_23, 0), 0)) = (-50402030301732970094301193384368719330_i128) as isize;
_2 = Move(_23);
(*_8) = [7541244274959382477_u64,7746470095009334928_u64,6800036547132589218_u64,176446214268114058_u64];
(*_8) = [13304704319845089506_u64,9156396471466404490_u64,13547991943689352453_u64,12028296551854917337_u64];
_23 = Move(_2);
(*_8) = [15350441759262959901_u64,5716108718902541735_u64,12890746736979021594_u64,2010539893773016783_u64];
_17 = (-6285050391975280762_i64) as isize;
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _18;
_3 = [91850511802604293747887219499757897359_i128,(-147950140623973400788574737083901271494_i128),70094645315480515584216792373822938350_i128,103519321289653757336142933108818610113_i128];
_26 = (-17853893244982903683473587389519303894_i128) as f64;
place!(Field::<isize>(Variant(_23, 0), 0)) = -_17;
Goto(bb11)
}
bb11 = {
_23 = Adt22::Variant1 { fld0: Move((*_16)),fld1: 5_usize };
(*_16) = Move(Field::<*const usize>(Variant(_23, 1), 0));
_17 = !_18;
_15 = 112_i8 & 106_i8;
(*_8) = [5041018057371938605_u64,12575552089418989466_u64,13879175092820139519_u64,10036460547023980653_u64];
_29.1 = [329409542750601273215932540773320488221_u128,198574237657373981205141172907524332102_u128,116563334802848574994550675526101534598_u128,101245688070299733614902331742849620633_u128,147560951471046094286523518553871059314_u128];
(*_8) = [7668057519579894627_u64,17488970158141060080_u64,16674976150392453564_u64,13885309205516933962_u64];
_25 = 275319791150732186310749535279314091111_u128 as u64;
_29.1 = [172016578717996170067499301320452521930_u128,193715335544789743942516806131062791409_u128,47422014693447581344929299563802568872_u128,269838855217576569294480013973918475246_u128,336934586536156349841691721307166466071_u128];
(*_8) = [_25,_25,_25,_25];
_3 = [(-120298588466914553165306750199144038546_i128),(-56255507557536282521493265440623472285_i128),111361530728801086310584302972567699620_i128,15480364095217488307199659533929758563_i128];
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_23, 1), 1)));
(*_6) = 220788955996595018894730440076128175395_u128 as usize;
_10 = RET;
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_9 = _12;
_15 = !(-64_i8);
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
Goto(bb12)
}
bb12 = {
(*_6) = false as usize;
_2 = Adt22::Variant0 { fld0: _18,fld1: _11 };
(*_16) = core::ptr::addr_of!((*_6));
_25 = !8004954031278202552_u64;
_13 = [58679_u16,35905_u16,44451_u16,61954_u16];
_16 = core::ptr::addr_of_mut!((*_16));
_28.0 = core::ptr::addr_of_mut!(_29.0);
(*_8) = [_25,_25,_25,_25];
_18 = _15 as isize;
place!(Field::<*const usize>(Variant(_23, 1), 0)) = core::ptr::addr_of!((*_6));
_27.0 = [(-6110358686671712528_i64),8928112844255932933_i64];
_29.1 = [323493490880921838466815366436202864138_u128,16816165847254527314432575173074794401_u128,301213939946364218598266381565039844300_u128,145126222886527584083086361296483921710_u128,336179402299525392531842667870599900038_u128];
(*_6) = _25 as usize;
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!((*_6));
_25 = 3657997485171929279_u64;
(*_16) = core::ptr::addr_of!((*_6));
_16 = core::ptr::addr_of_mut!((*_16));
match _25 {
0 => bb9,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
3657997485171929279 => bb18,
_ => bb17
}
}
bb13 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb14 = {
(*_8) = [16019980935924784893_u64,6185828536329261166_u64,15010786339091579785_u64,2793881637615745103_u64];
(*_8) = [5633034868172798539_u64,5961378673961375925_u64,1713698759352004668_u64,8992245304720486598_u64];
_10 = Field::<char>(Variant(_2, 0), 1);
_3 = [(-82809489764229480833188965653667548253_i128),131337061933768677770558614628756292063_i128,114154787760495010083191983454210348922_i128,108070848135423461286205750792296190138_i128];
_16 = core::ptr::addr_of_mut!((*_16));
_16 = core::ptr::addr_of_mut!((*_16));
(*_8) = [10409728006806300303_u64,14764334429452661936_u64,3959227079729576144_u64,3030826258379634601_u64];
place!(Field::<char>(Variant(_2, 0), 1)) = _9;
(*_8) = [7015736465846920685_u64,7917989615038160420_u64,3525695268470747857_u64,8169044809523866260_u64];
_11 = _12;
(*_8) = [12342970281129924060_u64,15430790921040870563_u64,13363679698793282034_u64,13263141524527916726_u64];
_8 = core::ptr::addr_of!((*_8));
(*_8) = [11032805131952999416_u64,73846090717718250_u64,12923294909231889966_u64,3589074683625547986_u64];
_13 = [7164_u16,18322_u16,32032_u16,26978_u16];
(*_8) = [2602707416824242185_u64,6755556943563103468_u64,1616274543565664289_u64,4653994241849065474_u64];
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9223372036854775807 => bb7,
_ => bb6
}
}
bb15 = {
_15 = 10954869201470590750_u64 as i8;
_3 = [142474130701399269797586116793753098035_i128,89065130442120953169087175528262312448_i128,(-82384967940469597936420415805792724588_i128),110348353039594349915698613979715472142_i128];
_10 = Field::<char>(Variant(_2, 0), 1);
Goto(bb2)
}
bb16 = {
(*_8) = [12266771239292892573_u64,9912560477408032804_u64,17142397470382164621_u64,2417621551136654620_u64];
_3 = [(-113902219171842972086448702320525666523_i128),59082288090594069888953207142916194861_i128,(-27692546159624217403187662835840138185_i128),61305026094668867650342467983144567292_i128];
(*_8) = [6410167567815780232_u64,14038407708300752109_u64,8128034793073014736_u64,840320896073664686_u64];
(*_8) = [2835657370426786419_u64,4793443775860371598_u64,973641759390391238_u64,3066002372841574675_u64];
Goto(bb9)
}
bb17 = {
(*_8) = [7067916893708747360_u64,7080528655899946637_u64,1456636470060453889_u64,13922892651551791189_u64];
(*_8) = [12519881779995972856_u64,11884947757526284619_u64,12838392647100186935_u64,7794170424844263899_u64];
(*_8) = [4555042794423113584_u64,10451568293031599931_u64,2573768363776753814_u64,14004443377639946354_u64];
_15 = 121_i8 + 3_i8;
(*_8) = [10914517443363647162_u64,11590815299085361568_u64,7538219505927160267_u64,14504238963587801402_u64];
(*_8) = [10704896336515397558_u64,11239062265883865753_u64,5527195893847963063_u64,9212752856394221930_u64];
Goto(bb8)
}
bb18 = {
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
(*_6) = 5_usize ^ 6_usize;
(*_16) = Move(Field::<*const usize>(Variant(_23, 1), 0));
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_23, 1), 1)));
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!((*_6));
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!((*_6));
_35 = false;
(*_8) = [_25,_25,_25,_25];
_3 = [70713109346123562470954962087151184957_i128,461143283959419565496428604060080506_i128,(-162297233476401062561179324635302208896_i128),131699412734535151734591521907939991841_i128];
(*_16) = core::ptr::addr_of!((*_6));
place!(Field::<usize>(Variant(_23, 1), 1)) = _26 as usize;
(*_16) = core::ptr::addr_of!((*_6));
(*_6) = 11571331150260860724_usize * 5457267898152181484_usize;
(*_16) = core::ptr::addr_of!((*_6));
match _25 {
0 => bb8,
1 => bb2,
2 => bb14,
3 => bb19,
4 => bb20,
3657997485171929279 => bb22,
_ => bb21
}
}
bb19 = {
(*_8) = [7067916893708747360_u64,7080528655899946637_u64,1456636470060453889_u64,13922892651551791189_u64];
(*_8) = [12519881779995972856_u64,11884947757526284619_u64,12838392647100186935_u64,7794170424844263899_u64];
(*_8) = [4555042794423113584_u64,10451568293031599931_u64,2573768363776753814_u64,14004443377639946354_u64];
_15 = 121_i8 + 3_i8;
(*_8) = [10914517443363647162_u64,11590815299085361568_u64,7538219505927160267_u64,14504238963587801402_u64];
(*_8) = [10704896336515397558_u64,11239062265883865753_u64,5527195893847963063_u64,9212752856394221930_u64];
Goto(bb8)
}
bb20 = {
_24 = 76_u8 - 39_u8;
(*_8) = [7653741817995168664_u64,283426482563904720_u64,11572184653004552071_u64,2823445767163111947_u64];
_11 = _10;
RET = _10;
_10 = _9;
(*_8) = [6448418395058957100_u64,17650290424681921148_u64,2976518544238129087_u64,4311815020371435034_u64];
(*_8) = [2013397397434063658_u64,16313427745432600092_u64,11917044631720727773_u64,10663717248899360716_u64];
_3 = [(-61063672940962124995094535045796663212_i128),(-20988342003345797887347876235598023617_i128),91454151675357695071953413251385624058_i128,(-117236205913582829747372737691739034598_i128)];
_18 = Field::<isize>(Variant(_2, 0), 0) | Field::<isize>(Variant(_2, 0), 0);
match Field::<isize>(Variant(_2, 0), 0) {
9223372036854775807 => bb10,
_ => bb5
}
}
bb21 = {
_23 = Adt22::Variant1 { fld0: Move((*_16)),fld1: 5_usize };
(*_16) = Move(Field::<*const usize>(Variant(_23, 1), 0));
_17 = !_18;
_15 = 112_i8 & 106_i8;
(*_8) = [5041018057371938605_u64,12575552089418989466_u64,13879175092820139519_u64,10036460547023980653_u64];
_29.1 = [329409542750601273215932540773320488221_u128,198574237657373981205141172907524332102_u128,116563334802848574994550675526101534598_u128,101245688070299733614902331742849620633_u128,147560951471046094286523518553871059314_u128];
(*_8) = [7668057519579894627_u64,17488970158141060080_u64,16674976150392453564_u64,13885309205516933962_u64];
_25 = 275319791150732186310749535279314091111_u128 as u64;
_29.1 = [172016578717996170067499301320452521930_u128,193715335544789743942516806131062791409_u128,47422014693447581344929299563802568872_u128,269838855217576569294480013973918475246_u128,336934586536156349841691721307166466071_u128];
(*_8) = [_25,_25,_25,_25];
_3 = [(-120298588466914553165306750199144038546_i128),(-56255507557536282521493265440623472285_i128),111361530728801086310584302972567699620_i128,15480364095217488307199659533929758563_i128];
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_23, 1), 1)));
(*_6) = 220788955996595018894730440076128175395_u128 as usize;
_10 = RET;
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_9 = _12;
_15 = !(-64_i8);
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
Goto(bb12)
}
bb22 = {
(*_6) = !10872087339772614427_usize;
_36.2 = _10;
(*_16) = core::ptr::addr_of!((*_6));
(*_6) = Field::<char>(Variant(_2, 0), 1) as usize;
_16 = core::ptr::addr_of_mut!((*_16));
_10 = _36.2;
place!(Field::<char>(Variant(_2, 0), 1)) = _9;
_27.1 = [293385979484523301984822897283854596981_u128,237524027150641588800180997486134059819_u128,92458950607218186281557697473323759925_u128,41921185695705880980041827764113381588_u128,56249144683153790636466592179937268473_u128];
(*_16) = core::ptr::addr_of!((*_6));
Goto(bb23)
}
bb23 = {
_29.1 = [177265947209899162952966341740737438166_u128,260600713552076636676501390176319362746_u128,51393729394608053392312321531769351526_u128,153564970308819941533540925764690168203_u128,26728937409405028547315145487660672376_u128];
(*_6) = _24 as usize;
(*_16) = core::ptr::addr_of!((*_6));
_24 = !55_u8;
(*_8) = [_25,_25,_25,_25];
Goto(bb24)
}
bb24 = {
place!(Field::<*const usize>(Variant(_23, 1), 0)) = core::ptr::addr_of!((*_6));
_23 = Move(_2);
_22 = 8684875432027089393_usize as f32;
_29.0 = [2339833116335852469_i64,(-8860900673350452269_i64)];
(*_8) = [_25,_25,_25,_25];
_25 = _26 as u64;
(*_8) = [_25,_25,_25,_25];
_9 = _11;
_13 = [30808_u16,5465_u16,31141_u16,49555_u16];
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_37 = core::ptr::addr_of!((*_8));
(*_8) = [_25,_25,_25,_25];
_41 = _35 & _35;
(*_8) = [_25,_25,_25,_25];
_36.1 = [2228078015036501720407826463506361267_u128,268574670572820509607208208739898236024_u128,332741638294965708116840225383008662025_u128,9665254005167396214576191294911786087_u128,205669651063559673197166480518957744663_u128,182408739201035093320437891124455196829_u128];
_41 = _11 != _9;
Goto(bb25)
}
bb25 = {
_10 = _11;
_39 = [Field::<isize>(Variant(_23, 0), 0),_17,_17,_18,_17];
(*_8) = [_25,_25,_25,_25];
_8 = core::ptr::addr_of!((*_8));
_24 = !158_u8;
Goto(bb26)
}
bb26 = {
_18 = !Field::<isize>(Variant(_23, 0), 0);
(*_8) = [_25,_25,_25,_25];
_27.1 = [68371707244326519458952457979097324734_u128,153022997665047036757048715316167242920_u128,258186953063681193362226368106517476403_u128,110141534528304340332998620783993487382_u128,151231522377433768538207004872635202994_u128];
_25 = 15889698451650417945_u64 - 8921188450940382469_u64;
_49 = 99301304537301126045247300805983894258_u128 + 168980426897531290797212430885252625241_u128;
_34 = _3;
_34 = [(-74454538685193147630113207340785573407_i128),59317651845180194704029662014163449953_i128,47439409067245841986173610331747543159_i128,(-38438486408510030467182831726103492782_i128)];
_27.1 = _29.1;
(*_8) = [_25,_25,_25,_25];
_15 = 13_i8;
_26 = _24 as f64;
(*_8) = [_25,_25,_25,_25];
_29.2 = &mut _49;
_51 = core::ptr::addr_of_mut!((*_16));
_17 = Field::<isize>(Variant(_23, 0), 0) ^ Field::<isize>(Variant(_23, 0), 0);
Goto(bb27)
}
bb27 = {
_28.0 = core::ptr::addr_of_mut!(_29.0);
_27 = Move(_29);
_51 = core::ptr::addr_of_mut!((*_51));
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_41 = _18 == Field::<isize>(Variant(_23, 0), 0);
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_12 = RET;
(*_8) = [_25,_25,_25,_25];
_9 = _11;
place!(Field::<isize>(Variant(_23, 0), 0)) = -_17;
_31 = core::ptr::addr_of_mut!(_27.0);
_3 = [76309834130276766198705850331970750122_i128,113263238146398400649018443680989567512_i128,149286881395962004004008997645126570226_i128,(-108454430506432655374022636057217051578_i128)];
(*_31) = [383436156487222091_i64,6033539856502233988_i64];
(*_8) = [_25,_25,_25,_25];
match _15 {
0 => bb16,
1 => bb2,
2 => bb8,
3 => bb11,
4 => bb26,
5 => bb24,
13 => bb29,
_ => bb28
}
}
bb28 = {
_3 = [(-45089803448208514759408715216638427916_i128),135865250728113002819207518445218904399_i128,9526620427057688117670069241343741031_i128,86470369192014663973023386468725572462_i128];
_23 = Adt22::Variant0 { fld0: _18,fld1: _12 };
(*_8) = [16884110653605987357_u64,6316607217103277723_u64,5310050245871774144_u64,494479706672493242_u64];
_12 = _9;
_9 = _12;
(*_8) = [10777063397408193210_u64,9539868289352490598_u64,16887852381063024652_u64,3527096255594554944_u64];
_2 = Move(_23);
_5 = [14326806385186391837_u64,8399181515086624598_u64,18269488683748060219_u64,742638649500437869_u64];
(*_8) = [13237630164780650360_u64,1208254615922836912_u64,11343633984515951843_u64,11147376759533600300_u64];
_23 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_2, 0), 0),fld1: RET };
place!(Field::<isize>(Variant(_23, 0), 0)) = (-50402030301732970094301193384368719330_i128) as isize;
_2 = Move(_23);
(*_8) = [7541244274959382477_u64,7746470095009334928_u64,6800036547132589218_u64,176446214268114058_u64];
(*_8) = [13304704319845089506_u64,9156396471466404490_u64,13547991943689352453_u64,12028296551854917337_u64];
_23 = Move(_2);
(*_8) = [15350441759262959901_u64,5716108718902541735_u64,12890746736979021594_u64,2010539893773016783_u64];
_17 = (-6285050391975280762_i64) as isize;
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _18;
_3 = [91850511802604293747887219499757897359_i128,(-147950140623973400788574737083901271494_i128),70094645315480515584216792373822938350_i128,103519321289653757336142933108818610113_i128];
_26 = (-17853893244982903683473587389519303894_i128) as f64;
place!(Field::<isize>(Variant(_23, 0), 0)) = -_17;
Goto(bb11)
}
bb29 = {
_47 = [_7,_7,_7,_7,_7,_7,_7];
_29.0 = [(-3510237958465535615_i64),(-1192907426148523240_i64)];
_55.0 = core::ptr::addr_of_mut!((*_31));
_41 = _9 > Field::<char>(Variant(_23, 0), 1);
(*_8) = [_25,_25,_25,_25];
_23 = Adt22::Variant0 { fld0: _18,fld1: _36.2 };
_35 = _17 <= Field::<isize>(Variant(_23, 0), 0);
_33 = _22 as i8;
(*_31) = [(-4578998810913810956_i64),4172764794822321615_i64];
_41 = _35;
_56 = core::ptr::addr_of!((*_8));
_42 = 4_usize as f32;
(*_16) = core::ptr::addr_of!(_60);
(*_31) = _29.0;
_51 = core::ptr::addr_of_mut!((*_16));
(*_51) = core::ptr::addr_of!((*_6));
_36.1 = [177916649939848366639323500646983569765_u128,204094358201026696811519998541754539553_u128,206135076957728270854436277155637215520_u128,140600952908048436488829684089917944872_u128,281233519491287118111851788609022824992_u128,1730443281143081611273637855048540014_u128];
(*_56) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
(*_16) = core::ptr::addr_of!((*_6));
(*_6) = 7_usize - 2_usize;
_28.0 = core::ptr::addr_of_mut!((*_31));
match _15 {
0 => bb17,
1 => bb26,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
6 => bb34,
13 => bb36,
_ => bb35
}
}
bb30 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb31 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb32 = {
_18 = !Field::<isize>(Variant(_23, 0), 0);
(*_8) = [_25,_25,_25,_25];
_27.1 = [68371707244326519458952457979097324734_u128,153022997665047036757048715316167242920_u128,258186953063681193362226368106517476403_u128,110141534528304340332998620783993487382_u128,151231522377433768538207004872635202994_u128];
_25 = 15889698451650417945_u64 - 8921188450940382469_u64;
_49 = 99301304537301126045247300805983894258_u128 + 168980426897531290797212430885252625241_u128;
_34 = _3;
_34 = [(-74454538685193147630113207340785573407_i128),59317651845180194704029662014163449953_i128,47439409067245841986173610331747543159_i128,(-38438486408510030467182831726103492782_i128)];
_27.1 = _29.1;
(*_8) = [_25,_25,_25,_25];
_15 = 13_i8;
_26 = _24 as f64;
(*_8) = [_25,_25,_25,_25];
_29.2 = &mut _49;
_51 = core::ptr::addr_of_mut!((*_16));
_17 = Field::<isize>(Variant(_23, 0), 0) ^ Field::<isize>(Variant(_23, 0), 0);
Goto(bb27)
}
bb33 = {
_3 = [(-45089803448208514759408715216638427916_i128),135865250728113002819207518445218904399_i128,9526620427057688117670069241343741031_i128,86470369192014663973023386468725572462_i128];
_23 = Adt22::Variant0 { fld0: _18,fld1: _12 };
(*_8) = [16884110653605987357_u64,6316607217103277723_u64,5310050245871774144_u64,494479706672493242_u64];
_12 = _9;
_9 = _12;
(*_8) = [10777063397408193210_u64,9539868289352490598_u64,16887852381063024652_u64,3527096255594554944_u64];
_2 = Move(_23);
_5 = [14326806385186391837_u64,8399181515086624598_u64,18269488683748060219_u64,742638649500437869_u64];
(*_8) = [13237630164780650360_u64,1208254615922836912_u64,11343633984515951843_u64,11147376759533600300_u64];
_23 = Adt22::Variant0 { fld0: Field::<isize>(Variant(_2, 0), 0),fld1: RET };
place!(Field::<isize>(Variant(_23, 0), 0)) = (-50402030301732970094301193384368719330_i128) as isize;
_2 = Move(_23);
(*_8) = [7541244274959382477_u64,7746470095009334928_u64,6800036547132589218_u64,176446214268114058_u64];
(*_8) = [13304704319845089506_u64,9156396471466404490_u64,13547991943689352453_u64,12028296551854917337_u64];
_23 = Move(_2);
(*_8) = [15350441759262959901_u64,5716108718902541735_u64,12890746736979021594_u64,2010539893773016783_u64];
_17 = (-6285050391975280762_i64) as isize;
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _18;
_3 = [91850511802604293747887219499757897359_i128,(-147950140623973400788574737083901271494_i128),70094645315480515584216792373822938350_i128,103519321289653757336142933108818610113_i128];
_26 = (-17853893244982903683473587389519303894_i128) as f64;
place!(Field::<isize>(Variant(_23, 0), 0)) = -_17;
Goto(bb11)
}
bb34 = {
(*_8) = [7067916893708747360_u64,7080528655899946637_u64,1456636470060453889_u64,13922892651551791189_u64];
(*_8) = [12519881779995972856_u64,11884947757526284619_u64,12838392647100186935_u64,7794170424844263899_u64];
(*_8) = [4555042794423113584_u64,10451568293031599931_u64,2573768363776753814_u64,14004443377639946354_u64];
_15 = 121_i8 + 3_i8;
(*_8) = [10914517443363647162_u64,11590815299085361568_u64,7538219505927160267_u64,14504238963587801402_u64];
(*_8) = [10704896336515397558_u64,11239062265883865753_u64,5527195893847963063_u64,9212752856394221930_u64];
Goto(bb8)
}
bb35 = {
_29.1 = [177265947209899162952966341740737438166_u128,260600713552076636676501390176319362746_u128,51393729394608053392312321531769351526_u128,153564970308819941533540925764690168203_u128,26728937409405028547315145487660672376_u128];
(*_6) = _24 as usize;
(*_16) = core::ptr::addr_of!((*_6));
_24 = !55_u8;
(*_8) = [_25,_25,_25,_25];
Goto(bb24)
}
bb36 = {
_57 = _7;
(*_6) = 14075115410194005300_usize >> _17;
_2 = Adt22::Variant1 { fld0: Move((*_16)),fld1: (*_6) };
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _17;
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
_26 = 66993973819577560333937663522771479438_u128 as f64;
_18 = _17;
(*_6) = (-5790920007269072715_i64) as usize;
(*_31) = [5681652129214300627_i64,(-4398905727102824793_i64)];
Goto(bb37)
}
bb37 = {
_6 = core::ptr::addr_of!((*_6));
(*_31) = [2244744552039222319_i64,6800147713327391128_i64];
_39 = [_18,Field::<isize>(Variant(_23, 0), 0),_17,_17,Field::<isize>(Variant(_23, 0), 0)];
_8 = core::ptr::addr_of!((*_8));
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!(_60);
_63 = Field::<isize>(Variant(_23, 0), 0) >> _15;
(*_16) = core::ptr::addr_of!((*_6));
(*_16) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_57 = 228064561826853285092817974583175896166_u128 as i32;
_22 = -_42;
_8 = core::ptr::addr_of!((*_8));
_27.1 = [14750795869015129034024326855045540499_u128,43309260838891542003286203136895798443_u128,246341715740768877406886361982381459248_u128,314860919946306826130306435977885924630_u128,271221069671108509534957520306713209810_u128];
(*_31) = [6556473008717080580_i64,1855515726014763094_i64];
_32 = !_25;
(*_16) = core::ptr::addr_of!(_60);
(*_16) = core::ptr::addr_of!((*_6));
_50 = _26;
(*_8) = [_32,_25,_25,_25];
(*_6) = Field::<usize>(Variant(_2, 1), 1) ^ Field::<usize>(Variant(_2, 1), 1);
(*_16) = core::ptr::addr_of!((*_6));
(*_6) = Field::<usize>(Variant(_2, 1), 1) << Field::<usize>(Variant(_2, 1), 1);
(*_16) = core::ptr::addr_of!((*_6));
_52 = _7 as isize;
match _15 {
0 => bb20,
1 => bb11,
13 => bb38,
_ => bb34
}
}
bb38 = {
(*_6) = !Field::<usize>(Variant(_2, 1), 1);
_32 = _25;
_69 = 2382725052_u32 as u16;
(*_8) = [_25,_32,_25,_25];
(*_31) = _29.0;
_23 = Adt22::Variant0 { fld0: _52,fld1: _12 };
(*_6) = Field::<usize>(Variant(_2, 1), 1) >> _17;
(*_31) = [(-1684727324443593627_i64),6654282965356573005_i64];
Call((*_6) = core::intrinsics::bswap(Field::<usize>(Variant(_2, 1), 1)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
(*_6) = !Field::<usize>(Variant(_2, 1), 1);
_3 = [(-77587838617567007061754304017808262354_i128),24463716869136679324347617684631077010_i128,81600037262808360540234490147499506956_i128,77106572431405538719386218316176750759_i128];
_2 = Move(_23);
Call(_57 = core::intrinsics::bswap(_7), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_64 = core::ptr::addr_of_mut!((*_16));
(*_64) = core::ptr::addr_of!((*_6));
_62 = _18 << _69;
(*_6) = _24 as usize;
_29.1 = [319973608632431290508403511507528003839_u128,100317449296018437293881565895152292260_u128,275229902141439796154028569824535882092_u128,68201972146460536641324539533594067495_u128,74824239486187729444404008865261915930_u128];
(*_8) = [_25,_32,_32,_32];
(*_64) = core::ptr::addr_of!((*_6));
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_32];
(*_6) = 5_usize;
(*_8) = [_32,_32,_32,_32];
(*_31) = [8272904291394601078_i64,(-9166236897377009567_i64)];
(*_6) = 4223087680977998465_usize - 5563502974019243682_usize;
match _15 {
0 => bb9,
1 => bb33,
2 => bb8,
3 => bb4,
13 => bb42,
_ => bb41
}
}
bb41 = {
RET = _9;
(*_8) = [7537877536698963398_u64,7306591249710344478_u64,13864349403563170612_u64,951073436197006485_u64];
(*_8) = [17799892455866772926_u64,6578065339776806299_u64,1816057491780167177_u64,1769766273417417269_u64];
(*_8) = [14983664766079123375_u64,14455293996776005440_u64,3202454397546224719_u64,13163726464496862282_u64];
place!(Field::<isize>(Variant(_2, 0), 0)) = 9223372036854775807_isize;
(*_8) = [5064672899417034638_u64,15365179199428261119_u64,17990545984092838649_u64,5126644211936345269_u64];
(*_8) = [4247999595301351505_u64,8250420236054323237_u64,11479984333382045668_u64,10340118207220759081_u64];
(*_8) = [15993802719316468147_u64,1523763562736126182_u64,637559695765587726_u64,6949588302872110000_u64];
_11 = _10;
(*_8) = [13742257646707683243_u64,4263579507408977199_u64,4330089460332062037_u64,16396235142284001068_u64];
(*_8) = [12877416161800270596_u64,7158720465148465598_u64,7660807645895123351_u64,16326497600169145673_u64];
(*_8) = [6066262393250568463_u64,907898407738995967_u64,16045991810773607845_u64,14092665428301067231_u64];
Goto(bb5)
}
bb42 = {
(*_8) = [_25,_32,_32,_25];
_74 = 4185677789_u32 * 2573540301_u32;
_13 = [_69,_69,_69,_69];
_32 = _24 as u64;
(*_8) = [_32,_25,_25,_32];
match _15 {
0 => bb43,
13 => bb45,
_ => bb44
}
}
bb43 = {
_57 = _7;
(*_6) = 14075115410194005300_usize >> _17;
_2 = Adt22::Variant1 { fld0: Move((*_16)),fld1: (*_6) };
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _17;
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
_26 = 66993973819577560333937663522771479438_u128 as f64;
_18 = _17;
(*_6) = (-5790920007269072715_i64) as usize;
(*_31) = [5681652129214300627_i64,(-4398905727102824793_i64)];
Goto(bb37)
}
bb44 = {
_6 = core::ptr::addr_of!((*_6));
(*_31) = [2244744552039222319_i64,6800147713327391128_i64];
_39 = [_18,Field::<isize>(Variant(_23, 0), 0),_17,_17,Field::<isize>(Variant(_23, 0), 0)];
_8 = core::ptr::addr_of!((*_8));
place!(Field::<*const usize>(Variant(_2, 1), 0)) = core::ptr::addr_of!(_60);
_63 = Field::<isize>(Variant(_23, 0), 0) >> _15;
(*_16) = core::ptr::addr_of!((*_6));
(*_16) = Move(Field::<*const usize>(Variant(_2, 1), 0));
_57 = 228064561826853285092817974583175896166_u128 as i32;
_22 = -_42;
_8 = core::ptr::addr_of!((*_8));
_27.1 = [14750795869015129034024326855045540499_u128,43309260838891542003286203136895798443_u128,246341715740768877406886361982381459248_u128,314860919946306826130306435977885924630_u128,271221069671108509534957520306713209810_u128];
(*_31) = [6556473008717080580_i64,1855515726014763094_i64];
_32 = !_25;
(*_16) = core::ptr::addr_of!(_60);
(*_16) = core::ptr::addr_of!((*_6));
_50 = _26;
(*_8) = [_32,_25,_25,_25];
(*_6) = Field::<usize>(Variant(_2, 1), 1) ^ Field::<usize>(Variant(_2, 1), 1);
(*_16) = core::ptr::addr_of!((*_6));
(*_6) = Field::<usize>(Variant(_2, 1), 1) << Field::<usize>(Variant(_2, 1), 1);
(*_16) = core::ptr::addr_of!((*_6));
_52 = _7 as isize;
match _15 {
0 => bb20,
1 => bb11,
13 => bb38,
_ => bb34
}
}
bb45 = {
(*_8) = [_25,_25,_25,_32];
_72 = _17 | _52;
_22 = _42;
(*_6) = 7_usize + 3_usize;
_58 = core::ptr::addr_of!((*_6));
_56 = core::ptr::addr_of!((*_8));
(*_8) = [_32,_32,_25,_25];
_25 = _32 - _32;
(*_16) = core::ptr::addr_of!((*_6));
(*_31) = _29.0;
(*_6) = 253025546622235259862793688027964555979_u128 as usize;
_37 = core::ptr::addr_of!((*_8));
_68 = (-22264_i16) - 23881_i16;
(*_31) = [(-2844017520252153869_i64),3512768533969226385_i64];
(*_31) = _29.0;
(*_37) = [_25,_25,_25,_25];
(*_6) = 3_usize << _72;
_23 = Adt22::Variant0 { fld0: _62,fld1: _12 };
_71 = _24 as f64;
_34 = [(-109930645403042449558807765401546156917_i128),(-39676429311751422604111730221055202734_i128),58785785567977199267529871265085609782_i128,69386865077023967601671735560923073423_i128];
match _15 {
0 => bb13,
1 => bb4,
2 => bb34,
3 => bb46,
4 => bb47,
13 => bb49,
_ => bb48
}
}
bb46 = {
(*_8) = [7067916893708747360_u64,7080528655899946637_u64,1456636470060453889_u64,13922892651551791189_u64];
(*_8) = [12519881779995972856_u64,11884947757526284619_u64,12838392647100186935_u64,7794170424844263899_u64];
(*_8) = [4555042794423113584_u64,10451568293031599931_u64,2573768363776753814_u64,14004443377639946354_u64];
_15 = 121_i8 + 3_i8;
(*_8) = [10914517443363647162_u64,11590815299085361568_u64,7538219505927160267_u64,14504238963587801402_u64];
(*_8) = [10704896336515397558_u64,11239062265883865753_u64,5527195893847963063_u64,9212752856394221930_u64];
Goto(bb8)
}
bb47 = {
_57 = _7;
(*_6) = 14075115410194005300_usize >> _17;
_2 = Adt22::Variant1 { fld0: Move((*_16)),fld1: (*_6) };
(*_16) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
place!(Field::<isize>(Variant(_23, 0), 0)) = _18 ^ _17;
(*_16) = core::ptr::addr_of!((*_6));
(*_8) = [_25,_25,_25,_25];
_26 = 66993973819577560333937663522771479438_u128 as f64;
_18 = _17;
(*_6) = (-5790920007269072715_i64) as usize;
(*_31) = [5681652129214300627_i64,(-4398905727102824793_i64)];
Goto(bb37)
}
bb48 = {
_28.0 = core::ptr::addr_of_mut!(_29.0);
_27 = Move(_29);
_51 = core::ptr::addr_of_mut!((*_51));
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_41 = _18 == Field::<isize>(Variant(_23, 0), 0);
(*_8) = [_25,_25,_25,_25];
(*_8) = [_25,_25,_25,_25];
_12 = RET;
(*_8) = [_25,_25,_25,_25];
_9 = _11;
place!(Field::<isize>(Variant(_23, 0), 0)) = -_17;
_31 = core::ptr::addr_of_mut!(_27.0);
_3 = [76309834130276766198705850331970750122_i128,113263238146398400649018443680989567512_i128,149286881395962004004008997645126570226_i128,(-108454430506432655374022636057217051578_i128)];
(*_31) = [383436156487222091_i64,6033539856502233988_i64];
(*_8) = [_25,_25,_25,_25];
match _15 {
0 => bb16,
1 => bb2,
2 => bb8,
3 => bb11,
4 => bb26,
5 => bb24,
13 => bb29,
_ => bb28
}
}
bb49 = {
_39 = [_63,_17,Field::<isize>(Variant(_23, 0), 0),Field::<isize>(Variant(_23, 0), 0),_72];
(*_37) = [_32,_32,_32,_25];
(*_37) = [_25,_25,_25,_25];
_73 = _62 >> (*_58);
(*_6) = 3_usize | 7_usize;
(*_31) = [3692538945551578168_i64,7834952080452610421_i64];
(*_8) = [_25,_32,_32,_25];
_27.0 = _29.0;
_75 = (*_6) > (*_58);
(*_6) = 98702414458581445685203261317065275031_u128 as usize;
(*_58) = 3_usize;
(*_58) = _68 as usize;
(*_8) = [_25,_25,_25,_25];
_63 = !_17;
(*_16) = core::ptr::addr_of!(_60);
(*_6) = 6_usize * 5341589336853224705_usize;
_56 = core::ptr::addr_of!((*_8));
(*_8) = [_25,_32,_25,_25];
_29.0 = (*_31);
(*_16) = Move(_58);
_33 = _15;
_27.0 = _29.0;
Goto(bb50)
}
bb50 = {
Call(_83 = dump_var(Move(_32), Move(_49), Move(_34), Move(_72)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_83 = dump_var(Move(_25), Move(_11), Move(_39), Move(_73)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_83 = dump_var(Move(_18), Move(_75), Move(_47), Move(_60)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(Move(_13), Move(_57), Move(_68), Move(_62)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const [u64; 4],mut _2: *mut Adt48,mut _3: [u16; 4],mut _4: char,mut _5: *mut Adt48,mut _6: char,mut _7: [i128; 4],mut _8: [u64; 4],mut _9: char,mut _10: char,mut _11: [i128; 4]) -> *const usize {
mir! {
type RET = *const usize;
let _12: &'static mut (i32, *const usize);
let _13: i64;
let _14: *const [u64; 4];
let _15: *const u8;
let _16: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _17: *mut [i64; 2];
let _18: i64;
let _19: usize;
let _20: isize;
let _21: f32;
let _22: i32;
let _23: &'static mut (i128, &'static mut u128);
let _24: isize;
let _25: *mut [i64; 2];
let _26: char;
let _27: f32;
let _28: *const *const [u64; 4];
let _29: &'static u32;
let _30: [i64; 2];
let _31: *const &'static mut (i128, &'static mut u128);
let _32: [isize; 3];
let _33: *const &'static mut (i32, *const usize);
let _34: isize;
let _35: *const *const [u64; 4];
let _36: &'static mut (i32, *const usize);
let _37: f32;
let _38: u8;
let _39: i32;
let _40: *mut *const usize;
let _41: f32;
let _42: Adt22;
let _43: *mut (i128, &'static mut u128);
let _44: f64;
let _45: u32;
let _46: u128;
let _47: char;
let _48: [i8; 7];
let _49: *mut (&'static &'static u32, [u128; 6], char);
let _50: *const &'static mut (i32, *const usize);
let _51: *const *const [u64; 4];
let _52: usize;
let _53: [u8; 4];
let _54: u128;
let _55: *mut (i128, &'static mut u128);
let _56: [i16; 4];
let _57: isize;
let _58: *mut Adt48;
let _59: &'static &'static u32;
let _60: bool;
let _61: [u16; 4];
let _62: (u8, usize, isize);
let _63: bool;
let _64: f64;
let _65: &'static u32;
let _66: f64;
let _67: u16;
let _68: u64;
let _69: *const usize;
let _70: *const (i32, *const usize);
let _71: u8;
let _72: *const f64;
let _73: &'static mut &'static mut (i128, &'static mut u128);
let _74: u32;
let _75: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _76: isize;
let _77: bool;
let _78: bool;
let _79: ((u8, usize, isize), &'static mut u128);
let _80: &'static &'static Adt25;
let _81: f32;
let _82: isize;
let _83: u8;
let _84: u16;
let _85: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _86: isize;
let _87: *mut (&'static &'static u32, [u128; 6], char);
let _88: *const &'static mut (i128, &'static mut u128);
let _89: (i128, &'static mut u128);
let _90: f64;
let _91: *const &'static mut (i32, *const usize);
let _92: ();
let _93: ();
{
_7 = [(-150718468446375589756688994464548838144_i128),(-24692221152866283639306049125584716916_i128),(-83421493089673883043012292373232533184_i128),(-68988447656678315543192271140361530244_i128)];
_4 = _9;
_10 = _9;
_6 = _4;
_5 = Move(_2);
_7 = [57376001260568539948782496562554351774_i128,153014919138476485067093987274705711353_i128,(-77357994113480969990871841705398030054_i128),8089772692530127837365183622414273589_i128];
_11 = [58379274090951324736952105961467360655_i128,82543037699807717024995158571284189249_i128,(-248294831675790276652566538177147900_i128),(-150032948986785264250486437745782832341_i128)];
_19 = !7_usize;
RET = core::ptr::addr_of!(_19);
_11 = [72550933246729991245066367452964646360_i128,148603299738141243550073349891321558088_i128,(-42923556607099325909928371440946721513_i128),(-29675456447857231716925044567970363928_i128)];
(*RET) = !14562862969313146731_usize;
_6 = _9;
Goto(bb1)
}
bb1 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb2 = {
_16.1.1.1 = core::ptr::addr_of!((*RET));
_3 = [46899_u16,5831_u16,27179_u16,60699_u16];
_14 = Move(_1);
(*RET) = _26 as usize;
_2 = Move(_5);
_16.1.2 = [33_i8,64_i8,(-47_i8),113_i8,82_i8,116_i8,(-76_i8)];
match _13 {
0 => bb3,
9001071506815032666 => bb5,
_ => bb4
}
}
bb3 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*RET) = 17255269926314953815_usize & 3_usize;
_1 = core::ptr::addr_of!(_8);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_21 = (-108586245_i32) as f32;
_20 = -(-57_isize);
_25 = core::ptr::addr_of_mut!(_30);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
match _13 {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
9001071506815032666 => bb9,
_ => bb8
}
}
bb6 = {
Return()
}
bb7 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb8 = {
_16.1.1.1 = core::ptr::addr_of!((*RET));
_3 = [46899_u16,5831_u16,27179_u16,60699_u16];
_14 = Move(_1);
(*RET) = _26 as usize;
_2 = Move(_5);
_16.1.2 = [33_i8,64_i8,(-47_i8),113_i8,82_i8,116_i8,(-76_i8)];
match _13 {
0 => bb3,
9001071506815032666 => bb5,
_ => bb4
}
}
bb9 = {
_22 = -1783970610_i32;
_16.1.0 = [_4,_4,_9,_26,_10,_10,_26];
_17 = core::ptr::addr_of_mut!((*_25));
(*RET) = 218_u8 as usize;
_1 = Move(_14);
_22 = 609675376_i32;
(*_25) = [_13,_13];
(*RET) = 12258021794111351008_usize * 7902588249478044655_usize;
_4 = _9;
(*RET) = (-32456_i16) as usize;
(*_25) = [_13,_13];
(*_25) = [_13,_13];
Goto(bb10)
}
bb10 = {
(*_25) = [_13,_13];
_35 = core::ptr::addr_of!(_1);
Call(_27 = core::intrinsics::transmute(_10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24 = _20;
(*RET) = 5_usize * 6863676090365139362_usize;
(*_35) = core::ptr::addr_of!(_8);
(*RET) = 452305513_u32 as usize;
(*_25) = [_13,_13];
(*RET) = 2_usize << _24;
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
_16.1.3 = Adt22::Variant1 { fld0: Move(RET),fld1: (*RET) };
(*_25) = [_13,_13];
_11 = [(-142527656103534951497133123916105535222_i128),131849217404034861725040089184113048741_i128,31256292779899659508881885428826178101_i128,(-3873878991487648461021000175426625744_i128)];
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_35) = core::ptr::addr_of!((*_1));
_16.1.2 = [(-108_i8),30_i8,(-42_i8),122_i8,(-31_i8),25_i8,44_i8];
_16.1.1.2 = &mut _22;
_24 = _20 | _20;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_13,_13];
_5 = Move(_2);
_3 = [3322_u16,42644_u16,53378_u16,33080_u16];
(*_25) = [_13,_13];
(*_25) = [_13,_13];
Goto(bb12)
}
bb12 = {
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
(*_35) = core::ptr::addr_of!((*_1));
_16.1.0 = [_10,_26,_6,_6,_10,_4,_9];
(*_25) = [_13,_13];
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = _19 * _19;
(*_35) = core::ptr::addr_of!((*_1));
match _13 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb4,
4 => bb7,
9001071506815032666 => bb13,
_ => bb6
}
}
bb13 = {
_25 = core::ptr::addr_of_mut!((*_25));
_10 = _6;
_13 = 6877837621614870147_i64;
(*_25) = [_13,_13];
Goto(bb14)
}
bb14 = {
_37 = _20 as f32;
_3 = [56264_u16,23179_u16,10228_u16,62759_u16];
(*_25) = [_13,_13];
(*_25) = [_13,_13];
_21 = _27 + _37;
_28 = core::ptr::addr_of!((*_35));
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
_11 = [(-140643413287165652751600132012896252273_i128),95193565499689055593930768662892478766_i128,92218879477447277132480705773290448121_i128,124183870762587445590152391881898741726_i128];
_3 = [42357_u16,22174_u16,63639_u16,10508_u16];
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_35) = core::ptr::addr_of!((*_1));
(*_35) = core::ptr::addr_of!((*_1));
(*_25) = [_13,_13];
RET = Move(Field::<*const usize>(Variant(_16.1.3, 1), 0));
_27 = _21;
_17 = core::ptr::addr_of_mut!((*_25));
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_13,_13];
_34 = _20 + _24;
_28 = core::ptr::addr_of!((*_35));
match _13 {
0 => bb10,
1 => bb12,
2 => bb3,
3 => bb4,
4 => bb5,
6877837621614870147 => bb16,
_ => bb15
}
}
bb15 = {
_25 = core::ptr::addr_of_mut!((*_25));
_10 = _6;
_13 = 6877837621614870147_i64;
(*_25) = [_13,_13];
Goto(bb14)
}
bb16 = {
_35 = Move(_28);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_13,_13];
_28 = core::ptr::addr_of!(_1);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_35 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_1));
(*_25) = [_13,_13];
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_16.1.3, 1), 1)));
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 + _19;
_39 = 122591957_i32 - 1844390688_i32;
(*RET) = !_19;
_20 = _24 - _34;
(*_25) = [_13,_13];
(*RET) = _19;
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 | _19;
(*RET) = _19 * _19;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_37 = -_21;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
Goto(bb17)
}
bb17 = {
(*_25) = [_13,_13];
_38 = 62_u8 | 180_u8;
(*RET) = _19 * _19;
_38 = 215_u8;
(*_25) = [_13,_13];
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_19 = !(*RET);
(*RET) = _19;
(*RET) = _20 as usize;
(*RET) = _19;
(*_25) = [_13,_13];
Call((*RET) = core::intrinsics::bswap(_19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_41 = -_21;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_28) = core::ptr::addr_of!((*_1));
_39 = !1871273724_i32;
RET = core::ptr::addr_of!((*RET));
_18 = _13 + _13;
(*_25) = [_13,_13];
(*_28) = core::ptr::addr_of!((*_1));
_35 = Move(_28);
_17 = Move(_25);
_27 = _41 - _21;
_40 = core::ptr::addr_of_mut!(RET);
(*_40) = core::ptr::addr_of!((*RET));
(*RET) = _19 + _19;
_46 = 27449579576796357670880556811899204355_u128 | 56064169287345624185359474020228734471_u128;
_42 = Adt22::Variant1 { fld0: Move((*_40)),fld1: (*RET) };
(*_40) = core::ptr::addr_of!(_19);
_14 = core::ptr::addr_of!((*_1));
_28 = core::ptr::addr_of!(_14);
_28 = core::ptr::addr_of!((*_28));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_40) = core::ptr::addr_of!((*RET));
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*RET) = Field::<usize>(Variant(_16.1.3, 1), 1);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*RET) = Field::<usize>(Variant(_16.1.3, 1), 1);
place!(Field::<*const usize>(Variant(_16.1.3, 1), 0)) = Move((*_40));
Goto(bb19)
}
bb19 = {
(*_28) = Move(_1);
(*_40) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_16.1.3, 1), 1)));
(*_28) = core::ptr::addr_of!(_8);
_39 = !(-1950324049_i32);
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*RET) = !Field::<usize>(Variant(_42, 1), 1);
_35 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_14));
_4 = _9;
RET = core::ptr::addr_of!((*RET));
match _13 {
6877837621614870147 => bb21,
_ => bb20
}
}
bb20 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb21 = {
_2 = Move(_5);
_25 = core::ptr::addr_of_mut!(_30);
(*_28) = core::ptr::addr_of!((*_14));
(*RET) = Field::<usize>(Variant(_42, 1), 1) | Field::<usize>(Variant(_42, 1), 1);
_5 = Move(_2);
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_18,_13];
_20 = _34;
(*RET) = !Field::<usize>(Variant(_42, 1), 1);
(*RET) = Field::<usize>(Variant(_42, 1), 1) + Field::<usize>(Variant(_42, 1), 1);
(*_40) = Move(Field::<*const usize>(Variant(_42, 1), 0));
_40 = core::ptr::addr_of_mut!((*_40));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
place!(Field::<*const usize>(Variant(_42, 1), 0)) = core::ptr::addr_of!(_19);
_37 = _41 * _41;
_30 = [_18,_18];
_52 = _19 ^ Field::<usize>(Variant(_16.1.3, 1), 1);
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_27 = _37 + _41;
(*_40) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_42, 1), 1)));
_54 = 51333649719716945545938029744020594066_i128 as u128;
(*_25) = [_18,_13];
(*_25) = [_18,_13];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*RET) = !_52;
_51 = core::ptr::addr_of!((*_28));
Goto(bb22)
}
bb22 = {
(*_40) = core::ptr::addr_of!((*RET));
_16.1.3 = Adt22::Variant1 { fld0: Move((*_40)),fld1: (*RET) };
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = !_19;
(*_40) = Move(Field::<*const usize>(Variant(_16.1.3, 1), 0));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_18,_18];
(*_51) = core::ptr::addr_of!((*_14));
_45 = 3913836441_u32 * 2375835878_u32;
_39 = 1670892947_i32 & 1326686352_i32;
_48 = [(-47_i8),0_i8,119_i8,(-38_i8),(-85_i8),35_i8,43_i8];
(*_40) = Move(_16.1.1.1);
(*_25) = [_18,_13];
(*_25) = [_18,_13];
match _38 {
0 => bb23,
1 => bb24,
215 => bb26,
_ => bb25
}
}
bb23 = {
Return()
}
bb24 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb25 = {
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
(*_35) = core::ptr::addr_of!((*_1));
_16.1.0 = [_10,_26,_6,_6,_10,_4,_9];
(*_25) = [_13,_13];
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = _19 * _19;
(*_35) = core::ptr::addr_of!((*_1));
match _13 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb4,
4 => bb7,
9001071506815032666 => bb13,
_ => bb6
}
}
bb26 = {
place!(Field::<*const usize>(Variant(_42, 1), 0)) = Move(RET);
(*_40) = core::ptr::addr_of!(_52);
(*_51) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_40) = core::ptr::addr_of!((*RET));
(*RET) = !Field::<usize>(Variant(_16.1.3, 1), 1);
(*RET) = Field::<usize>(Variant(_16.1.3, 1), 1) & Field::<usize>(Variant(_16.1.3, 1), 1);
(*_28) = core::ptr::addr_of!(_8);
(*RET) = _16.0 as usize;
_2 = Move(_5);
(*_40) = core::ptr::addr_of!((*RET));
(*_28) = core::ptr::addr_of!((*_14));
(*_28) = core::ptr::addr_of!((*_14));
(*_25) = [_18,_13];
_58 = Move(_2);
Call(_26 = fn14(Move(_35), Move(_42), Move((*_40)), Move((*_28)), _6, Move(_58), _6, (*_14), Move(_17), Move(_40), _11), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_28) = core::ptr::addr_of!(_8);
(*_25) = [_18,_18];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_4 = _9;
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_17 = core::ptr::addr_of_mut!((*_25));
(*_17) = [_13,_18];
_42 = Adt22::Variant0 { fld0: _24,fld1: _10 };
_38 = _45 as u8;
_8 = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_1 = Move((*_28));
(*_17) = [_13,_13];
_10 = Field::<char>(Variant(_42, 0), 1);
_9 = _6;
_62.0 = _38 + _38;
Call((*_25) = fn15(Move(_17), _24), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
(*_28) = core::ptr::addr_of!(_8);
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_56 = [(-26673_i16),14147_i16,16126_i16,18982_i16];
(*_28) = core::ptr::addr_of!((*_14));
_48 = _16.1.2;
_56 = [32553_i16,23068_i16,32408_i16,(-23372_i16)];
(*_25) = [_13,_13];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_44 = _45 as f64;
_26 = _9;
_16.1.3 = Move(_42);
_9 = _4;
Goto(bb29)
}
bb29 = {
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_62 = (_38, _19, _24);
_61 = [55662_u16,5847_u16,50611_u16,38205_u16];
_35 = core::ptr::addr_of!((*_28));
_53 = [_38,_38,_62.0,_38];
_40 = core::ptr::addr_of_mut!(_16.1.1.1);
(*_35) = core::ptr::addr_of!((*_14));
(*_25) = [_18,_18];
(*_25) = [_13,_13];
_30 = [_18,_18];
_62.2 = -_34;
(*_35) = core::ptr::addr_of!(_8);
(*_28) = core::ptr::addr_of!((*_14));
_13 = _18;
(*_40) = core::ptr::addr_of!(_52);
(*_40) = core::ptr::addr_of!(_62.1);
_11 = [40401109710404670441778710344935462635_i128,145963755588195219852816647330088547246_i128,1256383349874399365771091917863607628_i128,75076807439774695615154697321310901600_i128];
(*_25) = [_18,_18];
Goto(bb30)
}
bb30 = {
(*_25) = [_18,_13];
(*_25) = [_18,_18];
_37 = _21 + _41;
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_57 = _34 + _20;
(*_40) = core::ptr::addr_of!(_62.1);
_63 = !false;
_20 = _57;
_66 = _44 * _44;
_62 = (_38, _19, _34);
_16.1.1.1 = core::ptr::addr_of!(_62.1);
_71 = _34 as u8;
_62.0 = _71;
_47 = _4;
_25 = core::ptr::addr_of_mut!((*_25));
(*_28) = Move(_1);
(*_40) = core::ptr::addr_of!(_62.1);
(*_40) = core::ptr::addr_of!(_19);
(*_28) = core::ptr::addr_of!(_8);
(*_25) = [_13,_18];
RET = Move((*_40));
(*_25) = [_18,_13];
(*_28) = core::ptr::addr_of!((*_14));
_17 = core::ptr::addr_of_mut!((*_25));
(*_40) = core::ptr::addr_of!(_19);
_11 = _7;
_16.1.2 = [(-63_i8),26_i8,65_i8,107_i8,(-93_i8),(-100_i8),52_i8];
Goto(bb31)
}
bb31 = {
_32 = [_24,_20,_24];
_29 = &_45;
(*_17) = [_18,_18];
(*_17) = [_18,_13];
(*_17) = [_18,_18];
_68 = _16.0 << _39;
_64 = _66;
(*_25) = [_18,_13];
Goto(bb32)
}
bb32 = {
_72 = core::ptr::addr_of!(_64);
place!(Field::<isize>(Variant(_16.1.3, 0), 0)) = _62.2 ^ _24;
(*_28) = core::ptr::addr_of!((*_14));
(*_28) = core::ptr::addr_of!((*_14));
(*_28) = core::ptr::addr_of!((*_14));
_1 = Move((*_28));
_62 = (_71, _19, _57);
_75.1 = Field::<char>(Variant(_16.1.3, 0), 1);
(*_28) = core::ptr::addr_of!(_8);
Goto(bb33)
}
bb33 = {
(*_40) = core::ptr::addr_of!(_52);
(*_14) = [_68,_68,_68,_68];
_45 = 1056828526_u32;
_59 = &_29;
(*_14) = [_16.0,_68,_16.0,_68];
_10 = Field::<char>(Variant(_16.1.3, 0), 1);
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_68,_16.0,_16.0,_16.0];
_78 = _63;
(*_72) = _41 as f64;
(*_14) = [_68,_68,_16.0,_68];
(*_72) = _44 + _66;
(*_14) = [_16.0,_68,_68,_68];
(*_40) = core::ptr::addr_of!(_79.0.1);
_24 = _34 << _71;
_10 = Field::<char>(Variant(_16.1.3, 0), 1);
_16.1.3 = Adt22::Variant1 { fld0: Move(RET),fld1: _62.1 };
(*_72) = _66 - _44;
(*_40) = core::ptr::addr_of!(_19);
(*_28) = core::ptr::addr_of!((*_14));
(*_40) = core::ptr::addr_of!(_52);
_16.1.1.1 = Move(Field::<*const usize>(Variant(_16.1.3, 1), 0));
(*_25) = [_18,_13];
match _45 {
0 => bb28,
1 => bb21,
1056828526 => bb34,
_ => bb12
}
}
bb34 = {
(*_14) = [_16.0,_68,_68,_68];
(*_25) = [_18,_13];
(*_28) = core::ptr::addr_of!((*_14));
(*_28) = core::ptr::addr_of!((*_14));
_53 = [_62.0,_62.0,_38,_62.0];
(*_28) = Move(_1);
RET = core::ptr::addr_of!(_52);
(*_40) = core::ptr::addr_of!((*RET));
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = (-29728_i16) as usize;
(*_25) = [_13,_13];
(*_25) = [_18,_13];
(*_72) = _66 * _44;
(*_72) = _66 - _44;
_79.0.0 = _71;
(*_28) = core::ptr::addr_of!(_8);
(*_25) = [_18,_13];
(*_40) = core::ptr::addr_of!((*RET));
_81 = _37 * _27;
_67 = (*_72) as u16;
(*_25) = [_18,_13];
_40 = core::ptr::addr_of_mut!((*_40));
(*_25) = [_18,_13];
(*_40) = core::ptr::addr_of!((*RET));
(*_28) = core::ptr::addr_of!((*_14));
match _45 {
0 => bb35,
1056828526 => bb37,
_ => bb36
}
}
bb35 = {
(*_25) = [_13,_13];
_35 = core::ptr::addr_of!(_1);
Call(_27 = core::intrinsics::transmute(_10), ReturnTo(bb11), UnwindUnreachable())
}
bb36 = {
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_62 = (_38, _19, _24);
_61 = [55662_u16,5847_u16,50611_u16,38205_u16];
_35 = core::ptr::addr_of!((*_28));
_53 = [_38,_38,_62.0,_38];
_40 = core::ptr::addr_of_mut!(_16.1.1.1);
(*_35) = core::ptr::addr_of!((*_14));
(*_25) = [_18,_18];
(*_25) = [_13,_13];
_30 = [_18,_18];
_62.2 = -_34;
(*_35) = core::ptr::addr_of!(_8);
(*_28) = core::ptr::addr_of!((*_14));
_13 = _18;
(*_40) = core::ptr::addr_of!(_52);
(*_40) = core::ptr::addr_of!(_62.1);
_11 = [40401109710404670441778710344935462635_i128,145963755588195219852816647330088547246_i128,1256383349874399365771091917863607628_i128,75076807439774695615154697321310901600_i128];
(*_25) = [_18,_18];
Goto(bb30)
}
bb37 = {
(*RET) = _62.1 & _62.1;
_42 = Adt22::Variant0 { fld0: _57,fld1: _4 };
_7 = [134428655259432491381231060497024899477_i128,(-154865701176549795758741055593580657110_i128),(-54950480605924890634898593488485466661_i128),(-5183752082247292291563124037609917592_i128)];
_18 = _13 >> (*RET);
(*RET) = _62.1 >> _71;
(*_25) = [_18,_18];
_1 = core::ptr::addr_of!((*_14));
_34 = _62.2;
_3 = [_67,_67,_67,_67];
(*_25) = [_18,_18];
(*_40) = core::ptr::addr_of!((*RET));
_64 = -_44;
(*RET) = _19 & _62.1;
(*_40) = core::ptr::addr_of!((*RET));
match _45 {
0 => bb19,
1 => bb24,
2 => bb25,
3 => bb29,
4 => bb8,
5 => bb18,
6 => bb38,
1056828526 => bb40,
_ => bb39
}
}
bb38 = {
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
(*_35) = core::ptr::addr_of!((*_1));
_16.1.0 = [_10,_26,_6,_6,_10,_4,_9];
(*_25) = [_13,_13];
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = _19 * _19;
(*_35) = core::ptr::addr_of!((*_1));
match _13 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb4,
4 => bb7,
9001071506815032666 => bb13,
_ => bb6
}
}
bb39 = {
_9 = _4;
_16.0 = 2762320052_u32 as u64;
_3 = [24107_u16,44319_u16,29459_u16,50331_u16];
(*RET) = !404230717580738341_usize;
_3 = [44360_u16,28292_u16,16202_u16,2905_u16];
(*RET) = (-111_i8) as usize;
(*RET) = !6_usize;
(*RET) = !8833920778307801287_usize;
_11 = _7;
(*RET) = 1_usize;
_9 = _10;
_7 = [_11[_19],_11[_19],_11[_19],_11[_19]];
(*RET) = !7791026297074798553_usize;
_13 = 9001071506815032666_i64;
(*RET) = _16.0 as usize;
(*RET) = !12858721961888908876_usize;
(*RET) = 4264633355686148547_usize << _13;
_26 = _4;
(*RET) = 6_usize;
_10 = _26;
(*RET) = _10 as usize;
Goto(bb2)
}
bb40 = {
(*_25) = [_18,_18];
(*_1) = [_16.0,_16.0,_16.0,_68];
(*_14) = [_68,_68,_16.0,_68];
(*_72) = -_66;
(*_72) = _66 + _66;
(*_40) = core::ptr::addr_of!((*RET));
_74 = !_45;
_79.0.2 = -_24;
(*_40) = core::ptr::addr_of!(_19);
(*_40) = core::ptr::addr_of!((*RET));
(*RET) = _19 | _19;
match _45 {
0 => bb15,
1 => bb41,
2 => bb42,
3 => bb43,
4 => bb44,
5 => bb45,
1056828526 => bb47,
_ => bb46
}
}
bb41 = {
_35 = Move(_28);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_13,_13];
_28 = core::ptr::addr_of!(_1);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_35 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_1));
(*_25) = [_13,_13];
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_16.1.3, 1), 1)));
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 + _19;
_39 = 122591957_i32 - 1844390688_i32;
(*RET) = !_19;
_20 = _24 - _34;
(*_25) = [_13,_13];
(*RET) = _19;
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 | _19;
(*RET) = _19 * _19;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_37 = -_21;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
Goto(bb17)
}
bb42 = {
(*_40) = core::ptr::addr_of!(_52);
(*_14) = [_68,_68,_68,_68];
_45 = 1056828526_u32;
_59 = &_29;
(*_14) = [_16.0,_68,_16.0,_68];
_10 = Field::<char>(Variant(_16.1.3, 0), 1);
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_68,_16.0,_16.0,_16.0];
_78 = _63;
(*_72) = _41 as f64;
(*_14) = [_68,_68,_16.0,_68];
(*_72) = _44 + _66;
(*_14) = [_16.0,_68,_68,_68];
(*_40) = core::ptr::addr_of!(_79.0.1);
_24 = _34 << _71;
_10 = Field::<char>(Variant(_16.1.3, 0), 1);
_16.1.3 = Adt22::Variant1 { fld0: Move(RET),fld1: _62.1 };
(*_72) = _66 - _44;
(*_40) = core::ptr::addr_of!(_19);
(*_28) = core::ptr::addr_of!((*_14));
(*_40) = core::ptr::addr_of!(_52);
_16.1.1.1 = Move(Field::<*const usize>(Variant(_16.1.3, 1), 0));
(*_25) = [_18,_13];
match _45 {
0 => bb28,
1 => bb21,
1056828526 => bb34,
_ => bb12
}
}
bb43 = {
(*_25) = [_13,_13];
(*_35) = core::ptr::addr_of!((*_1));
(*_35) = core::ptr::addr_of!((*_1));
_16.1.0 = [_10,_26,_6,_6,_10,_4,_9];
(*_25) = [_13,_13];
place!(Field::<usize>(Variant(_16.1.3, 1), 1)) = _19 * _19;
(*_35) = core::ptr::addr_of!((*_1));
match _13 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb4,
4 => bb7,
9001071506815032666 => bb13,
_ => bb6
}
}
bb44 = {
_32 = [_24,_20,_24];
_29 = &_45;
(*_17) = [_18,_18];
(*_17) = [_18,_13];
(*_17) = [_18,_18];
_68 = _16.0 << _39;
_64 = _66;
(*_25) = [_18,_13];
Goto(bb32)
}
bb45 = {
_22 = -1783970610_i32;
_16.1.0 = [_4,_4,_9,_26,_10,_10,_26];
_17 = core::ptr::addr_of_mut!((*_25));
(*RET) = 218_u8 as usize;
_1 = Move(_14);
_22 = 609675376_i32;
(*_25) = [_13,_13];
(*RET) = 12258021794111351008_usize * 7902588249478044655_usize;
_4 = _9;
(*RET) = (-32456_i16) as usize;
(*_25) = [_13,_13];
(*_25) = [_13,_13];
Goto(bb10)
}
bb46 = {
(*_25) = [_18,_13];
(*_25) = [_18,_18];
_37 = _21 + _41;
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*_14) = [_16.0,_16.0,_16.0,_16.0];
_57 = _34 + _20;
(*_40) = core::ptr::addr_of!(_62.1);
_63 = !false;
_20 = _57;
_66 = _44 * _44;
_62 = (_38, _19, _34);
_16.1.1.1 = core::ptr::addr_of!(_62.1);
_71 = _34 as u8;
_62.0 = _71;
_47 = _4;
_25 = core::ptr::addr_of_mut!((*_25));
(*_28) = Move(_1);
(*_40) = core::ptr::addr_of!(_62.1);
(*_40) = core::ptr::addr_of!(_19);
(*_28) = core::ptr::addr_of!(_8);
(*_25) = [_13,_18];
RET = Move((*_40));
(*_25) = [_18,_13];
(*_28) = core::ptr::addr_of!((*_14));
_17 = core::ptr::addr_of_mut!((*_25));
(*_40) = core::ptr::addr_of!(_19);
_11 = _7;
_16.1.2 = [(-63_i8),26_i8,65_i8,107_i8,(-93_i8),(-100_i8),52_i8];
Goto(bb31)
}
bb47 = {
_81 = -_27;
_76 = _62.2;
(*RET) = _62.1 ^ _19;
_11 = [(-1634471899693657009997503608466107905_i128),92501823811311088907225404390686656422_i128,23642913912078594764647230744593970537_i128,68175397324013822604850250056094118475_i128];
(*_72) = _66 * _66;
_81 = _37;
_35 = Move(_51);
(*_40) = core::ptr::addr_of!((*RET));
_69 = core::ptr::addr_of!((*RET));
_37 = -_81;
_79.0.2 = (-18469_i16) as isize;
(*_72) = _66 + _66;
(*_28) = core::ptr::addr_of!(_8);
_42 = Adt22::Variant1 { fld0: Move(_16.1.1.1),fld1: (*RET) };
(*_72) = _66 + _66;
_69 = core::ptr::addr_of!(_52);
(*_72) = _66;
_16.1.1.1 = core::ptr::addr_of!((*RET));
_79.0.2 = !_20;
match _45 {
0 => bb20,
1 => bb48,
1056828526 => bb50,
_ => bb49
}
}
bb48 = {
_35 = Move(_28);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
(*_25) = [_13,_13];
_28 = core::ptr::addr_of!(_1);
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_35 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_1));
(*_25) = [_13,_13];
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_16.1.3, 1), 1)));
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 + _19;
_39 = 122591957_i32 - 1844390688_i32;
(*RET) = !_19;
_20 = _24 - _34;
(*_25) = [_13,_13];
(*RET) = _19;
(*_28) = core::ptr::addr_of!((*_1));
(*RET) = _19 | _19;
(*RET) = _19 * _19;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
_37 = -_21;
(*_1) = [_16.0,_16.0,_16.0,_16.0];
Goto(bb17)
}
bb49 = {
(*_28) = Move(_1);
(*_40) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_16.1.3, 1), 1)));
(*_28) = core::ptr::addr_of!(_8);
_39 = !(-1950324049_i32);
(*_28) = core::ptr::addr_of!((*_14));
(*_14) = [_16.0,_16.0,_16.0,_16.0];
(*RET) = !Field::<usize>(Variant(_42, 1), 1);
_35 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_14));
_4 = _9;
RET = core::ptr::addr_of!((*RET));
match _13 {
6877837621614870147 => bb21,
_ => bb20
}
}
bb50 = {
_85.1.2 = _48;
(*RET) = _19 - _62.1;
(*_28) = core::ptr::addr_of!(_8);
_24 = _57;
(*_72) = -_44;
(*_25) = [_18,_13];
_14 = core::ptr::addr_of!(_8);
Goto(bb51)
}
bb51 = {
Call(_92 = dump_var(Move(_63), Move(_26), Move(_7), Move(_8)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_92 = dump_var(Move(_62), Move(_24), Move(_57), Move(_4)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_92 = dump_var(Move(_68), Move(_38), Move(_76), Move(_52)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_92 = dump_var(Move(_71), Move(_74), Move(_19), Move(_18)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_92 = dump_var(Move(_53), Move(_3), Move(_20), _93), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const *const [u64; 4],mut _2: Adt22,mut _3: *const usize,mut _4: *const [u64; 4],mut _5: char,mut _6: *mut Adt48,mut _7: char,mut _8: [u64; 4],mut _9: *mut [i64; 2],mut _10: *mut *const usize,mut _11: [i128; 4]) -> char {
mir! {
type RET = char;
let _12: [i32; 2];
let _13: *const [u64; 4];
let _14: u16;
let _15: *mut Adt48;
let _16: u128;
let _17: bool;
let _18: i128;
let _19: usize;
let _20: Adt48;
let _21: i32;
let _22: isize;
let _23: &'static mut &'static mut (i128, &'static mut u128);
let _24: [i32; 7];
let _25: f64;
let _26: bool;
let _27: ((u8, usize, isize), &'static mut u128);
let _28: u32;
let _29: u32;
let _30: (i32, *const usize);
let _31: f64;
let _32: char;
let _33: bool;
let _34: *const *const [u64; 4];
let _35: [u128; 6];
let _36: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _37: Adt54;
let _38: u8;
let _39: Adt73;
let _40: &'static u32;
let _41: isize;
let _42: Adt59;
let _43: &'static mut i32;
let _44: f32;
let _45: &'static mut (i32, *const usize);
let _46: i64;
let _47: char;
let _48: i8;
let _49: char;
let _50: u128;
let _51: f64;
let _52: f64;
let _53: isize;
let _54: (*mut [i64; 2],);
let _55: isize;
let _56: &'static &'static Adt25;
let _57: (Adt38, *const usize, &'static mut i32);
let _58: bool;
let _59: [u32; 5];
let _60: [u128; 5];
let _61: i8;
let _62: *mut [i64; 2];
let _63: isize;
let _64: *const usize;
let _65: &'static mut u128;
let _66: &'static mut &'static mut (i128, &'static mut u128);
let _67: [i16; 3];
let _68: bool;
let _69: &'static mut i32;
let _70: bool;
let _71: *mut [i64; 2];
let _72: (*mut [i64; 2],);
let _73: (u16, i32, char, u128);
let _74: f64;
let _75: *mut (&'static &'static u32, [u128; 6], char);
let _76: *mut [i64; 2];
let _77: u128;
let _78: (i16, Adt59, &'static u32);
let _79: (&'static mut i32, char, i16, &'static mut (i128, &'static mut u128));
let _80: i128;
let _81: ();
let _82: ();
{
_4 = core::ptr::addr_of!(_8);
RET = _7;
_7 = RET;
(*_4) = [15556566990764255271_u64,7962867909566985816_u64,1379684831411503533_u64,5487839544169407440_u64];
_11 = [(-28040485625726116304232091466337745649_i128),(-84762325860729909229164771252671241351_i128),(-114164710609614988396639936058786069257_i128),(-125240472328120762396070061205390357257_i128)];
_12 = [(-91830741_i32),(-746895385_i32)];
_5 = RET;
_10 = core::ptr::addr_of_mut!(place!(Field::<*const usize>(Variant(_2, 1), 0)));
(*_4) = [16354140917587636805_u64,5827087507193827240_u64,4845216006069689809_u64,103507202340188468_u64];
_7 = _5;
(*_10) = Move(_3);
(*_4) = [17812970585932677394_u64,16504367855005815843_u64,12615947452637597611_u64,493095014739994923_u64];
(*_4) = [10868739751436900729_u64,10487460950621923459_u64,18043218282807355191_u64,14544098490922361185_u64];
_13 = core::ptr::addr_of!((*_4));
(*_4) = [7369589207592978781_u64,17499081888733806280_u64,6071186536314041354_u64,17843875454043604904_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_3 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13091091609621200203_u64,14517271062168127537_u64,10527446802973399355_u64,8965452569454185786_u64];
_3 = core::ptr::addr_of!((*_3));
(*_3) = 13819225814488230491_usize;
(*_10) = core::ptr::addr_of!((*_3));
_1 = core::ptr::addr_of!(_4);
(*_3) = 4_usize;
match (*_3) {
0 => bb1,
4 => bb3,
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
(*_3) = 4_usize << 2597_i16;
(*_1) = core::ptr::addr_of!((*_4));
(*_3) = !16495206437260871962_usize;
(*_10) = Move(_3);
_8 = [1406498141316138686_u64,16889953593208289558_u64,17168318156648143886_u64,11828163089596339750_u64];
(*_4) = [9086837493717199555_u64,15383556051157883257_u64,1824726175159133252_u64,4108291474106787352_u64];
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!(_8);
(*_4) = [10087958362258036158_u64,10893814415527436163_u64,16981347075774345965_u64,8184620563873376943_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = !1_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [15670668508591792494_u64,9199471968222790956_u64,12293889171996378903_u64,16610246190375120198_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_4 = core::ptr::addr_of!((*_4));
(*_4) = [7147933663074561612_u64,5627035394907802102_u64,6192481649244668608_u64,15502265422394592459_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 5_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13285361988881728290_u64,17037965399115745888_u64,11140957102683107004_u64,17479575701729252030_u64];
_14 = 61940_u16 & 52217_u16;
(*_4) = [1681843217876975123_u64,11918395435000920343_u64,15032990644563221834_u64,8097949832771688222_u64];
(*_1) = core::ptr::addr_of!((*_4));
Goto(bb4)
}
bb4 = {
RET = _5;
(*_4) = [7597935993821664063_u64,10185735226819064477_u64,13999753804027896872_u64,45230449466673600_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_12 = [1606250972_i32,(-1251821505_i32)];
(*_1) = core::ptr::addr_of!((*_4));
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
match Field::<usize>(Variant(_2, 1), 1) {
0 => bb2,
1 => bb5,
2 => bb6,
5 => bb8,
_ => bb7
}
}
bb5 = {
(*_3) = 4_usize << 2597_i16;
(*_1) = core::ptr::addr_of!((*_4));
(*_3) = !16495206437260871962_usize;
(*_10) = Move(_3);
_8 = [1406498141316138686_u64,16889953593208289558_u64,17168318156648143886_u64,11828163089596339750_u64];
(*_4) = [9086837493717199555_u64,15383556051157883257_u64,1824726175159133252_u64,4108291474106787352_u64];
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!(_8);
(*_4) = [10087958362258036158_u64,10893814415527436163_u64,16981347075774345965_u64,8184620563873376943_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = !1_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [15670668508591792494_u64,9199471968222790956_u64,12293889171996378903_u64,16610246190375120198_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_4 = core::ptr::addr_of!((*_4));
(*_4) = [7147933663074561612_u64,5627035394907802102_u64,6192481649244668608_u64,15502265422394592459_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 5_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13285361988881728290_u64,17037965399115745888_u64,11140957102683107004_u64,17479575701729252030_u64];
_14 = 61940_u16 & 52217_u16;
(*_4) = [1681843217876975123_u64,11918395435000920343_u64,15032990644563221834_u64,8097949832771688222_u64];
(*_1) = core::ptr::addr_of!((*_4));
Goto(bb4)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [8063800256003257563_u64,7117554059453148170_u64,3546910869638124029_u64,6359851916799702551_u64];
(*_1) = core::ptr::addr_of!((*_4));
_15 = Move(_6);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [12678486184494513602_u64,6761222946786018730_u64,198401022428350692_u64,15616475862028268632_u64];
(*_4) = [8460052606983795056_u64,11039401217635879969_u64,568672930921967106_u64,10746965034463131332_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!((*_4));
place!(Field::<usize>(Variant(_2, 1), 1)) = !4_usize;
Goto(bb9)
}
bb9 = {
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
Call(_13 = core::intrinsics::arith_offset((*_1), (-9223372036854775808_isize)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_1) = core::ptr::addr_of!((*_4));
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_6 = core::ptr::addr_of_mut!(_20);
_11 = [83097064101253039635641168021133925743_i128,(-116599625994178178796210044937876447274_i128),(-10983479461404225096354810081204933966_i128),41444483444002053798338199800080316700_i128];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_2 = Adt22::Variant0 { fld0: (-43_isize),fld1: RET };
Call(_22 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<isize>(Variant(_2, 0), 0)) = (-4_isize);
_17 = false;
(*_6) = Adt48::Variant0 { fld0: (-39015161681013028157092917838185771957_i128),fld1: Field::<char>(Variant(_2, 0), 1),fld2: 6259774304212440679_u64,fld3: 42_i8 };
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<u64>(Variant((*_6), 0), 2)) = !16029252203193961703_u64;
(*_6) = Adt48::Variant0 { fld0: 30603631659273019867801583590151368831_i128,fld1: _7,fld2: 14903760535504080955_u64,fld3: (-119_i8) };
_1 = core::ptr::addr_of!((*_1));
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb9,
340282366920938463463374607431768211452 => bb12,
_ => bb6
}
}
bb12 = {
place!(Field::<char>(Variant(_2, 0), 1)) = Field::<char>(Variant((*_6), 0), 1);
place!(Field::<u64>(Variant((*_6), 0), 2)) = !12549407397199962967_u64;
RET = Field::<char>(Variant((*_6), 0), 1);
place!(Field::<u64>(Variant((*_6), 0), 2)) = 1238810112703460959_u64 ^ 11613031754253220296_u64;
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_6), 0), 0)) = (-62240551672062039464681787093188406433_i128) ^ 103820310707502234271890973284497806340_i128;
_18 = Field::<i128>(Variant((*_6), 0), 0) - Field::<i128>(Variant(_20, 0), 0);
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18 + _18;
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<u64>(Variant((*_6), 0), 2)) = 6108016012297523931_u64 | 703810286302900445_u64;
place!(Field::<char>(Variant((*_6), 0), 1)) = _5;
place!(Field::<i128>(Variant((*_6), 0), 0)) = !_18;
(*_6) = Adt48::Variant3 { fld0: _18,fld1: Move(_10),fld2: _14 };
place!(Field::<i128>(Variant((*_6), 3), 0)) = 791101107_u32 as i128;
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18 >> Field::<u16>(Variant((*_6), 3), 2);
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb5,
1 => bb13,
340282366920938463463374607431768211452 => bb15,
_ => bb14
}
}
bb13 = {
place!(Field::<isize>(Variant(_2, 0), 0)) = (-4_isize);
_17 = false;
(*_6) = Adt48::Variant0 { fld0: (-39015161681013028157092917838185771957_i128),fld1: Field::<char>(Variant(_2, 0), 1),fld2: 6259774304212440679_u64,fld3: 42_i8 };
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<u64>(Variant((*_6), 0), 2)) = !16029252203193961703_u64;
(*_6) = Adt48::Variant0 { fld0: 30603631659273019867801583590151368831_i128,fld1: _7,fld2: 14903760535504080955_u64,fld3: (-119_i8) };
_1 = core::ptr::addr_of!((*_1));
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb9,
340282366920938463463374607431768211452 => bb12,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18 | _18;
place!(Field::<i128>(Variant((*_6), 3), 0)) = -_18;
place!(Field::<u16>(Variant((*_6), 3), 2)) = _14 << Field::<i128>(Variant((*_6), 3), 0);
place!(Field::<isize>(Variant(_2, 0), 0)) = _5 as isize;
place!(Field::<u16>(Variant((*_6), 3), 2)) = (-93_i8) as u16;
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant((*_6), 3), 0)) = !_18;
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18;
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18 * _18;
_22 = Field::<isize>(Variant(_2, 0), 0) >> Field::<i128>(Variant((*_6), 3), 0);
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
_18 = Field::<u16>(Variant((*_6), 3), 2) as i128;
place!(Field::<u16>(Variant((*_6), 3), 2)) = !_14;
_19 = 7_usize ^ 4143510104998316501_usize;
(*_6) = Adt48::Variant0 { fld0: _18,fld1: Field::<char>(Variant(_2, 0), 1),fld2: 16855472601829034981_u64,fld3: 79_i8 };
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<char>(Variant((*_6), 0), 1)) = _7;
place!(Field::<i8>(Variant(_20, 0), 3)) = !(-97_i8);
Goto(bb16)
}
bb16 = {
place!(Field::<u64>(Variant((*_6), 0), 2)) = 3357972968_u32 as u64;
place!(Field::<i128>(Variant((*_6), 0), 0)) = !_18;
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18;
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18 * _18;
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<i8>(Variant((*_6), 0), 3)) = !(-116_i8);
_11 = [Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),_18];
place!(Field::<u64>(Variant((*_6), 0), 2)) = 377059573296944695_u64 << Field::<i128>(Variant((*_6), 0), 0);
place!(Field::<char>(Variant((*_6), 0), 1)) = _7;
_6 = core::ptr::addr_of_mut!((*_6));
_15 = core::ptr::addr_of_mut!(_20);
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 ^ _18;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18;
_7 = Field::<char>(Variant((*_6), 0), 1);
_19 = 0_usize;
place!(Field::<i8>(Variant((*_6), 0), 3)) = 56_i8 >> Field::<u64>(Variant((*_6), 0), 2);
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18 << Field::<i8>(Variant((*_6), 0), 3);
place!(Field::<i8>(Variant((*_6), 0), 3)) = 22_i8 - 54_i8;
place!(Field::<i8>(Variant((*_6), 0), 3)) = _17 as i8;
place!(Field::<u64>(Variant((*_6), 0), 2)) = _8[_19];
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<i8>(Variant((*_6), 0), 3)) = 276529188953184714511013116370414801639_u128 as i8;
place!(Field::<u64>(Variant((*_6), 0), 2)) = !_8[_19];
_11 = [Field::<i128>(Variant(_20, 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0)];
RET = Field::<char>(Variant((*_6), 0), 1);
match _8[_19] {
0 => bb8,
1 => bb9,
2 => bb3,
3 => bb6,
8460052606983795056 => bb18,
_ => bb17
}
}
bb17 = {
Return()
}
bb18 = {
_12[_19] = (-125936608_i32) * 1970690386_i32;
_6 = Move(_15);
_19 = 15900508486795759954_usize + 13475266754618309642_usize;
_1 = core::ptr::addr_of!((*_1));
_19 = 5_usize;
_7 = Field::<char>(Variant(_2, 0), 1);
RET = _7;
_24[_19] = _7 as i32;
_30.0 = _24[_19] - _24[_19];
_32 = RET;
_18 = Field::<i128>(Variant(_20, 0), 0);
_28 = 2047495102_u32 ^ 2145639205_u32;
_10 = core::ptr::addr_of_mut!(_3);
(*_10) = core::ptr::addr_of!(_19);
(*_10) = core::ptr::addr_of!((*_3));
_16 = (*_3) as u128;
_15 = core::ptr::addr_of_mut!(_20);
place!(Field::<u64>(Variant((*_15), 0), 2)) = 12075920036772224029_u64 ^ 9637041307711128166_u64;
place!(Field::<char>(Variant(_2, 0), 1)) = Field::<char>(Variant((*_15), 0), 1);
_26 = _17;
(*_3) = _14 as usize;
(*_3) = !10955601992191569120_usize;
_27.0.2 = !_22;
(*_3) = !5324906660524115730_usize;
(*_10) = core::ptr::addr_of!((*_3));
Goto(bb19)
}
bb19 = {
(*_10) = core::ptr::addr_of!((*_3));
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<char>(Variant((*_15), 0), 1) as u64;
(*_10) = core::ptr::addr_of!((*_3));
_27.1 = &mut _16;
_34 = Move(_1);
_3 = core::ptr::addr_of!((*_3));
place!(Field::<char>(Variant((*_15), 0), 1)) = _7;
_35 = [52693830540713449979434015126602038316_u128,16568122018849959879017994141970017393_u128,114025261159433712544124745592391225144_u128,127832573791671930213521922681133000757_u128,24885641406732454396752781587901953554_u128,104568725706244187744952728296235248531_u128];
Goto(bb20)
}
bb20 = {
_21 = _30.0;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _22 as i128;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 & _18;
_2 = Adt22::Variant1 { fld0: Move((*_10)),fld1: _19 };
place!(Field::<i8>(Variant((*_15), 0), 3)) = 39_i8 + (-20_i8);
_39.fld5 = Field::<usize>(Variant(_2, 1), 1) as i32;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 4396305410651925982_u64 + 17532718596946286131_u64;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 5341876855220701595_u64 >> Field::<i128>(Variant((*_15), 0), 0);
(*_10) = Move(Field::<*const usize>(Variant(_2, 1), 0));
place!(Field::<u64>(Variant((*_15), 0), 2)) = 12215834998434186126_u64;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 17751074340868304190_u64 - 6400473818032577348_u64;
_27.0.1 = !_19;
place!(Field::<i128>(Variant((*_15), 0), 0)) = 7795025015600738882_i64 as i128;
place!(Field::<i8>(Variant((*_15), 0), 3)) = (-14_i8);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = [Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2)];
_42.fld1.3 = 136522716072020043080611853056038964714_u128 + 74333377983380506197641794564691660788_u128;
_6 = core::ptr::addr_of_mut!((*_15));
(*_15) = Adt48::Variant3 { fld0: _18,fld1: Move(_10),fld2: _14 };
_39.fld4 = (-21192_i16) as f32;
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 >> Field::<i128>(Variant(_20, 3), 0);
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 * _14;
Goto(bb21)
}
bb21 = {
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_42.fld3.1);
_15 = core::ptr::addr_of_mut!((*_15));
_40 = &_28;
_42.fld3.1 = Move(_3);
_10 = core::ptr::addr_of_mut!(_30.1);
Goto(bb22)
}
bb22 = {
_3 = core::ptr::addr_of!(_27.0.1);
_19 = _30.0 as usize;
_27.0 = (19_u8, Field::<usize>(Variant(_2, 1), 1), _22);
(*_10) = core::ptr::addr_of!(_27.0.1);
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = Move((*_10));
_24 = [_30.0,_21,_39.fld5,_30.0,_39.fld5,_30.0,_21];
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 - _14;
_15 = core::ptr::addr_of_mut!((*_15));
_36.1 = _5;
(*_10) = core::ptr::addr_of!((*_3));
_19 = (*_3) ^ (*_3);
(*_3) = Field::<i128>(Variant((*_15), 3), 0) as usize;
(*_15) = Adt48::Variant3 { fld0: _18,fld1: Move(_10),fld2: _14 };
(*_3) = _21 as usize;
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_30.1);
Goto(bb23)
}
bb23 = {
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 * _14;
_38 = _27.0.0 & _27.0.0;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 13806672804632180465_u64,fld3: 75_i8 };
_28 = !3698490686_u32;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _7,fld2: 650872691167685205_u64,fld3: (-25_i8) };
_6 = core::ptr::addr_of_mut!((*_15));
place!(Field::<i8>(Variant((*_6), 0), 3)) = (-24_i8) << Field::<i128>(Variant((*_6), 0), 0);
_11 = [Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_6), 0), 0)];
place!(Field::<i128>(Variant((*_6), 0), 0)) = _14 as i128;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 >> _18;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 10906375368826329245_u64 & 4265956464430050042_u64;
_27.0.1 = !_19;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 & _18;
_36.2 = _42.fld1.3 as i16;
_27.0.2 = _22 | _22;
_36.1 = Field::<char>(Variant((*_15), 0), 1);
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
_19 = !(*_3);
(*_3) = _19 & _19;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _42.fld1.3 as i128;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = Move(_3);
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 7242747485717625023_u64,fld3: (-29_i8) };
_42.fld1.0 = !_14;
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
Goto(bb24)
}
bb24 = {
RET = Field::<char>(Variant((*_15), 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = 17307947469927176480_u64;
place!(Field::<i8>(Variant((*_15), 0), 3)) = (-54_i8) << Field::<i128>(Variant((*_15), 0), 0);
place!(Field::<i8>(Variant((*_15), 0), 3)) = (-61_i8) + 20_i8;
place!(Field::<char>(Variant((*_15), 0), 1)) = _7;
place!(Field::<i128>(Variant(_20, 0), 0)) = _18 << _27.0.2;
place!(Field::<i8>(Variant((*_15), 0), 3)) = -(-27_i8);
place!(Field::<char>(Variant((*_15), 0), 1)) = _32;
_37 = Adt54::Variant0 { fld0: _11,fld1: Field::<u64>(Variant((*_15), 0), 2),fld2: _21 };
_6 = core::ptr::addr_of_mut!((*_15));
place!(Field::<u64>(Variant((*_6), 0), 2)) = Field::<u64>(Variant(_37, 0), 1);
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) & Field::<u64>(Variant(_37, 0), 1);
_30.0 = _39.fld5;
_3 = core::ptr::addr_of!(_19);
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18;
place!(Field::<u64>(Variant((*_15), 0), 2)) = (*_3) as u64;
_39.fld0 = Adt48::Variant0 { fld0: Field::<i128>(Variant((*_15), 0), 0),fld1: Field::<char>(Variant((*_6), 0), 1),fld2: Field::<u64>(Variant((*_15), 0), 2),fld3: Field::<i8>(Variant((*_15), 0), 3) };
match _27.0.0 {
0 => bb21,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
19 => bb30,
_ => bb29
}
}
bb25 = {
_12[_19] = (-125936608_i32) * 1970690386_i32;
_6 = Move(_15);
_19 = 15900508486795759954_usize + 13475266754618309642_usize;
_1 = core::ptr::addr_of!((*_1));
_19 = 5_usize;
_7 = Field::<char>(Variant(_2, 0), 1);
RET = _7;
_24[_19] = _7 as i32;
_30.0 = _24[_19] - _24[_19];
_32 = RET;
_18 = Field::<i128>(Variant(_20, 0), 0);
_28 = 2047495102_u32 ^ 2145639205_u32;
_10 = core::ptr::addr_of_mut!(_3);
(*_10) = core::ptr::addr_of!(_19);
(*_10) = core::ptr::addr_of!((*_3));
_16 = (*_3) as u128;
_15 = core::ptr::addr_of_mut!(_20);
place!(Field::<u64>(Variant((*_15), 0), 2)) = 12075920036772224029_u64 ^ 9637041307711128166_u64;
place!(Field::<char>(Variant(_2, 0), 1)) = Field::<char>(Variant((*_15), 0), 1);
_26 = _17;
(*_3) = _14 as usize;
(*_3) = !10955601992191569120_usize;
_27.0.2 = !_22;
(*_3) = !5324906660524115730_usize;
(*_10) = core::ptr::addr_of!((*_3));
Goto(bb19)
}
bb26 = {
place!(Field::<isize>(Variant(_2, 0), 0)) = (-4_isize);
_17 = false;
(*_6) = Adt48::Variant0 { fld0: (-39015161681013028157092917838185771957_i128),fld1: Field::<char>(Variant(_2, 0), 1),fld2: 6259774304212440679_u64,fld3: 42_i8 };
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<u64>(Variant((*_6), 0), 2)) = !16029252203193961703_u64;
(*_6) = Adt48::Variant0 { fld0: 30603631659273019867801583590151368831_i128,fld1: _7,fld2: 14903760535504080955_u64,fld3: (-119_i8) };
_1 = core::ptr::addr_of!((*_1));
match Field::<isize>(Variant(_2, 0), 0) {
0 => bb9,
340282366920938463463374607431768211452 => bb12,
_ => bb6
}
}
bb27 = {
Return()
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
place!(Field::<u64>(Variant((*_15), 0), 2)) = _27.0.2 as u64;
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_39.fld0, 0), 1);
place!(Field::<char>(Variant(_20, 0), 1)) = _5;
_27.0.2 = !_22;
_10 = core::ptr::addr_of_mut!(place!(Field::<*const usize>(Variant(_2, 1), 0)));
_42.fld2 = [Field::<char>(Variant((*_6), 0), 1),Field::<char>(Variant((*_15), 0), 1),_36.1,Field::<char>(Variant((*_15), 0), 1),Field::<char>(Variant((*_15), 0), 1),Field::<char>(Variant((*_15), 0), 1),Field::<char>(Variant((*_6), 0), 1)];
_54.0 = Move(_9);
(*_10) = core::ptr::addr_of!((*_3));
place!(Field::<i128>(Variant((*_15), 0), 0)) = Field::<i128>(Variant(_39.fld0, 0), 0);
place!(Field::<i8>(Variant((*_15), 0), 3)) = Field::<i8>(Variant(_39.fld0, 0), 3);
_32 = Field::<char>(Variant((*_6), 0), 1);
(*_10) = Move(_3);
match _27.0.0 {
0 => bb27,
1 => bb19,
2 => bb21,
3 => bb4,
4 => bb16,
5 => bb17,
6 => bb31,
19 => bb33,
_ => bb32
}
}
bb31 = {
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18 | _18;
place!(Field::<i128>(Variant((*_6), 3), 0)) = -_18;
place!(Field::<u16>(Variant((*_6), 3), 2)) = _14 << Field::<i128>(Variant((*_6), 3), 0);
place!(Field::<isize>(Variant(_2, 0), 0)) = _5 as isize;
place!(Field::<u16>(Variant((*_6), 3), 2)) = (-93_i8) as u16;
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant((*_6), 3), 0)) = !_18;
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18;
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
place!(Field::<i128>(Variant((*_6), 3), 0)) = _18 * _18;
_22 = Field::<isize>(Variant(_2, 0), 0) >> Field::<i128>(Variant((*_6), 3), 0);
place!(Field::<*mut *const usize>(Variant((*_6), 3), 1)) = core::ptr::addr_of_mut!(_3);
_18 = Field::<u16>(Variant((*_6), 3), 2) as i128;
place!(Field::<u16>(Variant((*_6), 3), 2)) = !_14;
_19 = 7_usize ^ 4143510104998316501_usize;
(*_6) = Adt48::Variant0 { fld0: _18,fld1: Field::<char>(Variant(_2, 0), 1),fld2: 16855472601829034981_u64,fld3: 79_i8 };
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<char>(Variant((*_6), 0), 1)) = _7;
place!(Field::<i8>(Variant(_20, 0), 3)) = !(-97_i8);
Goto(bb16)
}
bb32 = {
(*_3) = 4_usize << 2597_i16;
(*_1) = core::ptr::addr_of!((*_4));
(*_3) = !16495206437260871962_usize;
(*_10) = Move(_3);
_8 = [1406498141316138686_u64,16889953593208289558_u64,17168318156648143886_u64,11828163089596339750_u64];
(*_4) = [9086837493717199555_u64,15383556051157883257_u64,1824726175159133252_u64,4108291474106787352_u64];
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!(_8);
(*_4) = [10087958362258036158_u64,10893814415527436163_u64,16981347075774345965_u64,8184620563873376943_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = !1_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [15670668508591792494_u64,9199471968222790956_u64,12293889171996378903_u64,16610246190375120198_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_4 = core::ptr::addr_of!((*_4));
(*_4) = [7147933663074561612_u64,5627035394907802102_u64,6192481649244668608_u64,15502265422394592459_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 5_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13285361988881728290_u64,17037965399115745888_u64,11140957102683107004_u64,17479575701729252030_u64];
_14 = 61940_u16 & 52217_u16;
(*_4) = [1681843217876975123_u64,11918395435000920343_u64,15032990644563221834_u64,8097949832771688222_u64];
(*_1) = core::ptr::addr_of!((*_4));
Goto(bb4)
}
bb33 = {
_42.fld3 = (_21, Move((*_10)));
_1 = core::ptr::addr_of!(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(_19);
place!(Field::<i128>(Variant((*_15), 0), 0)) = !Field::<i128>(Variant(_39.fld0, 0), 0);
(*_15) = Move(_39.fld0);
_42.fld6 = [_42.fld1.3,_42.fld1.3,_42.fld1.3,_42.fld1.3,_42.fld1.3];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = Move(_42.fld3.1);
place!(Field::<[i128; 4]>(Variant(_37, 0), 0)) = _11;
_46 = !8106499811276865584_i64;
_27.0.0 = _38 >> Field::<u64>(Variant((*_15), 0), 2);
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
place!(Field::<char>(Variant((*_15), 0), 1)) = _5;
(*_10) = core::ptr::addr_of!(_19);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) - Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) | Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) >> _30.0;
_55 = !_27.0.2;
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
match Field::<u64>(Variant(_37, 0), 1) {
0 => bb30,
17307947469927176480 => bb34,
_ => bb20
}
}
bb34 = {
place!(Field::<char>(Variant((*_15), 0), 1)) = _7;
(*_10) = core::ptr::addr_of!(_27.0.1);
_27.1 = &mut _42.fld1.3;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
_29 = !_28;
match Field::<u64>(Variant(_37, 0), 1) {
0 => bb23,
1 => bb25,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
17307947469927176480 => bb40,
_ => bb39
}
}
bb35 = {
(*_3) = 4_usize << 2597_i16;
(*_1) = core::ptr::addr_of!((*_4));
(*_3) = !16495206437260871962_usize;
(*_10) = Move(_3);
_8 = [1406498141316138686_u64,16889953593208289558_u64,17168318156648143886_u64,11828163089596339750_u64];
(*_4) = [9086837493717199555_u64,15383556051157883257_u64,1824726175159133252_u64,4108291474106787352_u64];
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!(_8);
(*_4) = [10087958362258036158_u64,10893814415527436163_u64,16981347075774345965_u64,8184620563873376943_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = !1_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [15670668508591792494_u64,9199471968222790956_u64,12293889171996378903_u64,16610246190375120198_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_4 = core::ptr::addr_of!((*_4));
(*_4) = [7147933663074561612_u64,5627035394907802102_u64,6192481649244668608_u64,15502265422394592459_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 5_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13285361988881728290_u64,17037965399115745888_u64,11140957102683107004_u64,17479575701729252030_u64];
_14 = 61940_u16 & 52217_u16;
(*_4) = [1681843217876975123_u64,11918395435000920343_u64,15032990644563221834_u64,8097949832771688222_u64];
(*_1) = core::ptr::addr_of!((*_4));
Goto(bb4)
}
bb36 = {
(*_3) = 4_usize << 2597_i16;
(*_1) = core::ptr::addr_of!((*_4));
(*_3) = !16495206437260871962_usize;
(*_10) = Move(_3);
_8 = [1406498141316138686_u64,16889953593208289558_u64,17168318156648143886_u64,11828163089596339750_u64];
(*_4) = [9086837493717199555_u64,15383556051157883257_u64,1824726175159133252_u64,4108291474106787352_u64];
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_1) = core::ptr::addr_of!(_8);
(*_4) = [10087958362258036158_u64,10893814415527436163_u64,16981347075774345965_u64,8184620563873376943_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = !1_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [15670668508591792494_u64,9199471968222790956_u64,12293889171996378903_u64,16610246190375120198_u64];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_4 = core::ptr::addr_of!((*_4));
(*_4) = [7147933663074561612_u64,5627035394907802102_u64,6192481649244668608_u64,15502265422394592459_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 5_usize;
(*_1) = core::ptr::addr_of!((*_4));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_4) = [13285361988881728290_u64,17037965399115745888_u64,11140957102683107004_u64,17479575701729252030_u64];
_14 = 61940_u16 & 52217_u16;
(*_4) = [1681843217876975123_u64,11918395435000920343_u64,15032990644563221834_u64,8097949832771688222_u64];
(*_1) = core::ptr::addr_of!((*_4));
Goto(bb4)
}
bb37 = {
_21 = _30.0;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _22 as i128;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 & _18;
_2 = Adt22::Variant1 { fld0: Move((*_10)),fld1: _19 };
place!(Field::<i8>(Variant((*_15), 0), 3)) = 39_i8 + (-20_i8);
_39.fld5 = Field::<usize>(Variant(_2, 1), 1) as i32;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 4396305410651925982_u64 + 17532718596946286131_u64;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 5341876855220701595_u64 >> Field::<i128>(Variant((*_15), 0), 0);
(*_10) = Move(Field::<*const usize>(Variant(_2, 1), 0));
place!(Field::<u64>(Variant((*_15), 0), 2)) = 12215834998434186126_u64;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 17751074340868304190_u64 - 6400473818032577348_u64;
_27.0.1 = !_19;
place!(Field::<i128>(Variant((*_15), 0), 0)) = 7795025015600738882_i64 as i128;
place!(Field::<i8>(Variant((*_15), 0), 3)) = (-14_i8);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_8 = [Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2),Field::<u64>(Variant((*_15), 0), 2)];
_42.fld1.3 = 136522716072020043080611853056038964714_u128 + 74333377983380506197641794564691660788_u128;
_6 = core::ptr::addr_of_mut!((*_15));
(*_15) = Adt48::Variant3 { fld0: _18,fld1: Move(_10),fld2: _14 };
_39.fld4 = (-21192_i16) as f32;
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 >> Field::<i128>(Variant(_20, 3), 0);
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 * _14;
Goto(bb21)
}
bb38 = {
Return()
}
bb39 = {
Return()
}
bb40 = {
_48 = Field::<i8>(Variant((*_15), 0), 3) << Field::<u64>(Variant((*_15), 0), 2);
_28 = _29;
match Field::<u64>(Variant(_37, 0), 1) {
17307947469927176480 => bb41,
_ => bb16
}
}
bb41 = {
_63 = _22 | _27.0.2;
place!(Field::<char>(Variant((*_15), 0), 1)) = _5;
_39.fld2 = [_39.fld5,Field::<i32>(Variant(_37, 0), 2)];
place!(Field::<i8>(Variant((*_15), 0), 3)) = Field::<u64>(Variant((*_15), 0), 2) as i8;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 + _18;
_53 = -_63;
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
place!(Field::<i128>(Variant((*_15), 0), 0)) = Field::<char>(Variant(_20, 0), 1) as i128;
place!(Field::<i8>(Variant((*_15), 0), 3)) = -_48;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _14 as i128;
Goto(bb42)
}
bb42 = {
place!(Field::<u64>(Variant((*_15), 0), 2)) = !Field::<u64>(Variant(_37, 0), 1);
place!(Field::<i128>(Variant((*_15), 0), 0)) = -_18;
place!(Field::<char>(Variant((*_15), 0), 1)) = _32;
_57.2 = &mut _39.fld5;
_7 = _5;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _7,fld2: Field::<u64>(Variant(_37, 0), 1),fld3: _48 };
place!(Field::<[i128; 4]>(Variant(_37, 0), 0)) = [Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0)];
place!(Field::<char>(Variant((*_15), 0), 1)) = _7;
_41 = _53;
place!(Field::<i8>(Variant((*_15), 0), 3)) = _48 ^ _48;
_70 = _26 | _17;
_57.1 = core::ptr::addr_of!(_27.0.1);
place!(Field::<char>(Variant((*_15), 0), 1)) = _36.1;
_32 = Field::<char>(Variant((*_15), 0), 1);
match Field::<u64>(Variant((*_15), 0), 2) {
0 => bb43,
1 => bb44,
2 => bb45,
17307947469927176480 => bb47,
_ => bb46
}
}
bb43 = {
(*_1) = core::ptr::addr_of!((*_4));
(*_1) = Move(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_6 = core::ptr::addr_of_mut!(_20);
_11 = [83097064101253039635641168021133925743_i128,(-116599625994178178796210044937876447274_i128),(-10983479461404225096354810081204933966_i128),41444483444002053798338199800080316700_i128];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
_2 = Adt22::Variant0 { fld0: (-43_isize),fld1: RET };
Call(_22 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb11), UnwindUnreachable())
}
bb44 = {
_48 = Field::<i8>(Variant((*_15), 0), 3) << Field::<u64>(Variant((*_15), 0), 2);
_28 = _29;
match Field::<u64>(Variant(_37, 0), 1) {
17307947469927176480 => bb41,
_ => bb16
}
}
bb45 = {
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 * _14;
_38 = _27.0.0 & _27.0.0;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 13806672804632180465_u64,fld3: 75_i8 };
_28 = !3698490686_u32;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _7,fld2: 650872691167685205_u64,fld3: (-25_i8) };
_6 = core::ptr::addr_of_mut!((*_15));
place!(Field::<i8>(Variant((*_6), 0), 3)) = (-24_i8) << Field::<i128>(Variant((*_6), 0), 0);
_11 = [Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_6), 0), 0)];
place!(Field::<i128>(Variant((*_6), 0), 0)) = _14 as i128;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 >> _18;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 10906375368826329245_u64 & 4265956464430050042_u64;
_27.0.1 = !_19;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 & _18;
_36.2 = _42.fld1.3 as i16;
_27.0.2 = _22 | _22;
_36.1 = Field::<char>(Variant((*_15), 0), 1);
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
_19 = !(*_3);
(*_3) = _19 & _19;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _42.fld1.3 as i128;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = Move(_3);
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 7242747485717625023_u64,fld3: (-29_i8) };
_42.fld1.0 = !_14;
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
Goto(bb24)
}
bb46 = {
_42.fld3 = (_21, Move((*_10)));
_1 = core::ptr::addr_of!(_13);
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = core::ptr::addr_of!(_19);
place!(Field::<i128>(Variant((*_15), 0), 0)) = !Field::<i128>(Variant(_39.fld0, 0), 0);
(*_15) = Move(_39.fld0);
_42.fld6 = [_42.fld1.3,_42.fld1.3,_42.fld1.3,_42.fld1.3,_42.fld1.3];
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
(*_10) = Move(_42.fld3.1);
place!(Field::<[i128; 4]>(Variant(_37, 0), 0)) = _11;
_46 = !8106499811276865584_i64;
_27.0.0 = _38 >> Field::<u64>(Variant((*_15), 0), 2);
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
place!(Field::<char>(Variant((*_15), 0), 1)) = _5;
(*_10) = core::ptr::addr_of!(_19);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) - Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) | Field::<u64>(Variant(_37, 0), 1);
place!(Field::<u64>(Variant((*_15), 0), 2)) = Field::<u64>(Variant(_37, 0), 1) >> _30.0;
_55 = !_27.0.2;
(*_10) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_2, 1), 1)));
match Field::<u64>(Variant(_37, 0), 1) {
0 => bb30,
17307947469927176480 => bb34,
_ => bb20
}
}
bb47 = {
_63 = Field::<char>(Variant((*_15), 0), 1) as isize;
_74 = _19 as f64;
_67 = [_36.2,_36.2,_36.2];
(*_10) = core::ptr::addr_of!(_27.0.1);
_73.0 = _14 ^ _14;
_68 = _70;
(*_15) = Adt48::Variant3 { fld0: _18,fld1: Move(_10),fld2: _73.0 };
_25 = 169076185751071686314957569528123191677_u128 as f64;
_58 = !_70;
_30.1 = core::ptr::addr_of!(_19);
_51 = _74 - _74;
match Field::<u64>(Variant(_37, 0), 1) {
0 => bb38,
1 => bb45,
2 => bb13,
3 => bb48,
4 => bb49,
17307947469927176480 => bb51,
_ => bb50
}
}
bb48 = {
Return()
}
bb49 = {
place!(Field::<u64>(Variant((*_6), 0), 2)) = 3357972968_u32 as u64;
place!(Field::<i128>(Variant((*_6), 0), 0)) = !_18;
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18;
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18 * _18;
place!(Field::<char>(Variant((*_6), 0), 1)) = Field::<char>(Variant(_2, 0), 1);
place!(Field::<i8>(Variant((*_6), 0), 3)) = !(-116_i8);
_11 = [Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),_18];
place!(Field::<u64>(Variant((*_6), 0), 2)) = 377059573296944695_u64 << Field::<i128>(Variant((*_6), 0), 0);
place!(Field::<char>(Variant((*_6), 0), 1)) = _7;
_6 = core::ptr::addr_of_mut!((*_6));
_15 = core::ptr::addr_of_mut!(_20);
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 ^ _18;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18;
_7 = Field::<char>(Variant((*_6), 0), 1);
_19 = 0_usize;
place!(Field::<i8>(Variant((*_6), 0), 3)) = 56_i8 >> Field::<u64>(Variant((*_6), 0), 2);
place!(Field::<i128>(Variant((*_6), 0), 0)) = _18 << Field::<i8>(Variant((*_6), 0), 3);
place!(Field::<i8>(Variant((*_6), 0), 3)) = 22_i8 - 54_i8;
place!(Field::<i8>(Variant((*_6), 0), 3)) = _17 as i8;
place!(Field::<u64>(Variant((*_6), 0), 2)) = _8[_19];
place!(Field::<char>(Variant((*_6), 0), 1)) = RET;
place!(Field::<i8>(Variant((*_6), 0), 3)) = 276529188953184714511013116370414801639_u128 as i8;
place!(Field::<u64>(Variant((*_6), 0), 2)) = !_8[_19];
_11 = [Field::<i128>(Variant(_20, 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_6), 0), 0)];
RET = Field::<char>(Variant((*_6), 0), 1);
match _8[_19] {
0 => bb8,
1 => bb9,
2 => bb3,
3 => bb6,
8460052606983795056 => bb18,
_ => bb17
}
}
bb50 = {
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 * _14;
_38 = _27.0.0 & _27.0.0;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 13806672804632180465_u64,fld3: 75_i8 };
_28 = !3698490686_u32;
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _7,fld2: 650872691167685205_u64,fld3: (-25_i8) };
_6 = core::ptr::addr_of_mut!((*_15));
place!(Field::<i8>(Variant((*_6), 0), 3)) = (-24_i8) << Field::<i128>(Variant((*_6), 0), 0);
_11 = [Field::<i128>(Variant((*_6), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_15), 0), 0),Field::<i128>(Variant((*_6), 0), 0)];
place!(Field::<i128>(Variant((*_6), 0), 0)) = _14 as i128;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 >> _18;
place!(Field::<u64>(Variant((*_15), 0), 2)) = 10906375368826329245_u64 & 4265956464430050042_u64;
_27.0.1 = !_19;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _18 & _18;
_36.2 = _42.fld1.3 as i16;
_27.0.2 = _22 | _22;
_36.1 = Field::<char>(Variant((*_15), 0), 1);
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
_19 = !(*_3);
(*_3) = _19 & _19;
place!(Field::<char>(Variant((*_15), 0), 1)) = RET;
place!(Field::<i128>(Variant((*_15), 0), 0)) = _42.fld1.3 as i128;
place!(Field::<*const usize>(Variant(_2, 1), 0)) = Move(_3);
(*_15) = Adt48::Variant0 { fld0: _18,fld1: _32,fld2: 7242747485717625023_u64,fld3: (-29_i8) };
_42.fld1.0 = !_14;
place!(Field::<i128>(Variant((*_15), 0), 0)) = !_18;
Goto(bb24)
}
bb51 = {
_36.0 = &mut _21;
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_30.1);
place!(Field::<*mut *const usize>(Variant((*_15), 3), 1)) = core::ptr::addr_of_mut!(_64);
_73.1 = _68 as i32;
place!(Field::<i128>(Variant((*_15), 3), 0)) = _18 + _18;
_78.1.fld3.1 = Move(Field::<*const usize>(Variant(_2, 1), 0));
_78.1.fld5 = [Field::<u16>(Variant((*_15), 3), 2),Field::<u16>(Variant((*_15), 3), 2),Field::<u16>(Variant((*_15), 3), 2),Field::<u16>(Variant((*_15), 3), 2)];
place!(Field::<i128>(Variant((*_15), 3), 0)) = _18;
place!(Field::<i128>(Variant((*_15), 3), 0)) = _18;
_78.1.fld0 = Move(_54.0);
place!(Field::<u16>(Variant((*_15), 3), 2)) = _14 - _14;
Goto(bb52)
}
bb52 = {
Call(_81 = dump_var(Move(_21), Move(_18), Move(_28), Move(_70)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_81 = dump_var(Move(_41), Move(_53), Move(_14), Move(_58)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_81 = dump_var(Move(_46), Move(_38), Move(_55), Move(_29)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_81 = dump_var(Move(_16), Move(_48), _82, _82), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *mut [i64; 2],mut _2: isize) -> [i64; 2] {
mir! {
type RET = [i64; 2];
let _3: &'static &'static u32;
let _4: isize;
let _5: usize;
let _6: f32;
let _7: Adt28;
let _8: isize;
let _9: char;
let _10: *const &'static mut (i32, *const usize);
let _11: [i32; 7];
let _12: u8;
let _13: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _14: *mut *const usize;
let _15: u8;
let _16: [u32; 5];
let _17: *mut *const usize;
let _18: [u8; 4];
let _19: *const [u64; 4];
let _20: u64;
let _21: *mut [i64; 2];
let _22: f64;
let _23: u32;
let _24: &'static &'static Adt25;
let _25: &'static u32;
let _26: i8;
let _27: i128;
let _28: &'static u32;
let _29: char;
let _30: Adt54;
let _31: isize;
let _32: (i16, Adt59, &'static u32);
let _33: *const *const [u64; 4];
let _34: f64;
let _35: isize;
let _36: [u64; 4];
let _37: i64;
let _38: i128;
let _39: i16;
let _40: f64;
let _41: *const usize;
let _42: i128;
let _43: [u16; 4];
let _44: f32;
let _45: [isize; 7];
let _46: f32;
let _47: isize;
let _48: *const usize;
let _49: bool;
let _50: i8;
let _51: [isize; 3];
let _52: f32;
let _53: [u8; 4];
let _54: isize;
let _55: isize;
let _56: u64;
let _57: &'static mut u128;
let _58: ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22);
let _59: &'static Adt25;
let _60: f32;
let _61: f64;
let _62: char;
let _63: &'static mut (i128, &'static mut u128);
let _64: [i32; 2];
let _65: isize;
let _66: Adt48;
let _67: *mut (&'static &'static u32, [u128; 6], char);
let _68: i64;
let _69: u32;
let _70: u32;
let _71: *mut (&'static &'static u32, [u128; 6], char);
let _72: bool;
let _73: usize;
let _74: isize;
let _75: f32;
let _76: &'static mut i32;
let _77: *mut (&'static &'static u32, [u128; 6], char);
let _78: *mut *const usize;
let _79: i8;
let _80: bool;
let _81: bool;
let _82: [u32; 2];
let _83: f32;
let _84: ();
let _85: ();
{
RET = [(-969857340149122682_i64),(-2642002788934892769_i64)];
RET = [(-258395727780743221_i64),413151401793837971_i64];
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = [(-4488075604287565_i64),8490918422759301762_i64];
(*_1) = [(-8033043989979747755_i64),(-1123378122803608873_i64)];
_4 = _2 | _2;
(*_1) = [4867104687641065667_i64,(-1619970939523300234_i64)];
(*_1) = [(-3358408407575727834_i64),4750494493815723033_i64];
(*_1) = [(-3115131494054449142_i64),2619310552771315255_i64];
(*_1) = [(-5547457658871650445_i64),8831932687884836602_i64];
(*_1) = [(-65718533608921172_i64),1512513770605324246_i64];
(*_1) = [2888794330814221680_i64,(-7076356883813963307_i64)];
(*_1) = [9008059989092632424_i64,705826816259572469_i64];
(*_1) = [874255649831671730_i64,1840090621184970412_i64];
(*_1) = [7004227910654861692_i64,1563449467514851517_i64];
(*_1) = [3215528746446360029_i64,(-6370862156672243771_i64)];
Goto(bb1)
}
bb1 = {
_5 = 3070537110516408837_usize | 3887514277553566159_usize;
_6 = 5_i8 as f32;
(*_1) = [(-5378138718588547102_i64),7683791136795761140_i64];
(*_1) = [(-575662725983858853_i64),(-7444371716716387241_i64)];
(*_1) = [8951661526106810177_i64,806289365870514425_i64];
Goto(bb2)
}
bb2 = {
(*_1) = [(-32743693554491587_i64),(-5901565382875634355_i64)];
_6 = 13665741914873838735_u64 as f32;
(*_1) = [8684014240282797732_i64,(-6705939933954163980_i64)];
(*_1) = [3551927481438495981_i64,(-5895522874744544084_i64)];
Goto(bb3)
}
bb3 = {
(*_1) = [(-2683439211767134081_i64),(-2192997552370574695_i64)];
(*_1) = [7774572220823287171_i64,1741298902607285341_i64];
_9 = '\u{5844f}';
_8 = !_4;
_8 = _4 * _4;
(*_1) = [(-3518614116207325703_i64),(-8827661238283311478_i64)];
(*_1) = [6130012806118282569_i64,(-7571983837763521254_i64)];
(*_1) = [(-4260899360718852229_i64),5866225251552130157_i64];
_11 = [(-467785660_i32),159235587_i32,(-1320443942_i32),1291336400_i32,(-566970873_i32),823070393_i32,2126126741_i32];
(*_1) = [(-8593362149872578223_i64),976599827170654378_i64];
(*_1) = [(-6739327558557426986_i64),6861556967125274018_i64];
(*_1) = [5609902797966093131_i64,8621931051475787481_i64];
(*_1) = [1639747908947169468_i64,5830910921769703125_i64];
(*_1) = [(-1988670649312085491_i64),2051275031109189507_i64];
(*_1) = [8048385755533792067_i64,3725276168571985650_i64];
_11 = [(-208964769_i32),1422048409_i32,1024757018_i32,(-769826127_i32),(-720249302_i32),15099818_i32,713719838_i32];
(*_1) = [(-8597112897508220554_i64),2619021298424085760_i64];
(*_1) = [1096589524561273723_i64,(-94325234305634631_i64)];
(*_1) = [(-6750061677317042055_i64),(-3222181236924864901_i64)];
_11 = [(-1510756535_i32),802818672_i32,44062418_i32,(-324812088_i32),1520908364_i32,475269266_i32,876539845_i32];
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = [(-7210740079216872266_i64),4043729713563101015_i64];
_6 = 2443402456_u32 as f32;
(*_1) = [2868439907547660531_i64,2117811616004462970_i64];
_11 = [(-124048017_i32),(-1654974674_i32),(-470024004_i32),(-1760439969_i32),(-981457961_i32),(-2111919046_i32),(-8352066_i32)];
(*_1) = [(-1290869014450233154_i64),(-3058730054866983689_i64)];
(*_1) = [(-5571823322935795353_i64),8549486316883890580_i64];
_6 = (-2828403595892553451_i64) as f32;
(*_1) = [3522796183720068350_i64,(-1352998989240137235_i64)];
Goto(bb4)
}
bb4 = {
(*_1) = [2629081544194584015_i64,(-5047307215083583793_i64)];
(*_1) = [1197028401421845731_i64,(-4910604878550116932_i64)];
_12 = !63_u8;
(*_1) = [3684881459784713320_i64,4779177201284979433_i64];
_13.0 = 2663146404433244413_u64 & 7651700212655384743_u64;
_13.1.1.1 = core::ptr::addr_of!(_5);
Goto(bb5)
}
bb5 = {
(*_1) = [2208283408421051447_i64,(-1344037967390040205_i64)];
(*_1) = [(-4704341464015426658_i64),5625261746261471528_i64];
_14 = core::ptr::addr_of_mut!(_13.1.1.1);
(*_1) = [7090825551742935535_i64,(-5286232988340379577_i64)];
(*_14) = core::ptr::addr_of!(_5);
_12 = 1_u8 >> _4;
_13.1.2 = [51_i8,31_i8,(-18_i8),51_i8,109_i8,(-96_i8),43_i8];
(*_1) = [(-1273448996499862128_i64),(-3300243518389067025_i64)];
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [(-1438527789202084113_i64),(-5621859685323431438_i64)];
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [(-8509524393248333430_i64),7433824085683574618_i64];
(*_1) = [(-5112691837504595227_i64),(-7213103897020718384_i64)];
_11 = [685407109_i32,(-87496806_i32),1174645232_i32,1501506632_i32,1231180823_i32,(-1474735968_i32),85889588_i32];
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [(-5199792218717618331_i64),5632304909651591091_i64];
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
_13.1.1.1 = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [1198098627965112943_i64,(-8687774380106945510_i64)];
_16 = [510173117_u32,4265173196_u32,869253974_u32,1347996791_u32,1377196958_u32];
Goto(bb6)
}
bb6 = {
(*_14) = core::ptr::addr_of!(_5);
_13.1.0 = [_9,_9,_9,_9,_9,_9,_9];
_13.0 = !14416069916889190454_u64;
(*_1) = [5405523354198647003_i64,2122606350404803031_i64];
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [56822306204749850_i64,(-2091709136829467499_i64)];
_18 = [_12,_12,_12,_12];
_13.1.3 = Adt22::Variant0 { fld0: _8,fld1: _9 };
(*_1) = [(-8946516821636756558_i64),8780084261447826237_i64];
(*_1) = [8366324584769096024_i64,6283909748654096929_i64];
Goto(bb7)
}
bb7 = {
_13.1.2 = [(-47_i8),61_i8,42_i8,44_i8,127_i8,(-112_i8),(-76_i8)];
_13.1.1.1 = core::ptr::addr_of!(_5);
_17 = Move(_14);
_13.0 = 12242948204688497463_u64 | 14311686004458640802_u64;
_14 = core::ptr::addr_of_mut!(_13.1.1.1);
_8 = Field::<isize>(Variant(_13.1.3, 0), 0) * _4;
(*_14) = core::ptr::addr_of!(_5);
_6 = _12 as f32;
(*_1) = [(-5594869206611836420_i64),6631281577119837213_i64];
(*_14) = core::ptr::addr_of!(_5);
_13.1.0 = [Field::<char>(Variant(_13.1.3, 0), 1),Field::<char>(Variant(_13.1.3, 0), 1),Field::<char>(Variant(_13.1.3, 0), 1),_9,_9,Field::<char>(Variant(_13.1.3, 0), 1),_9];
(*_1) = [3839280715220358098_i64,(-1291903786410907843_i64)];
(*_14) = core::ptr::addr_of!(_5);
Goto(bb8)
}
bb8 = {
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [3302828633590461197_i64,633302556486948163_i64];
_12 = !116_u8;
(*_1) = [2469276067222166702_i64,1619125525584576954_i64];
RET = [(-6230822897534976120_i64),(-5910480176198083619_i64)];
Goto(bb9)
}
bb9 = {
_20 = !_13.0;
_22 = _5 as f64;
Goto(bb10)
}
bb10 = {
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
_20 = _22 as u64;
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [(-8182505388031843268_i64),4466492561056030752_i64];
_9 = Field::<char>(Variant(_13.1.3, 0), 1);
_11 = [1786039060_i32,(-355500179_i32),(-1807788364_i32),612822409_i32,1787748473_i32,1437985055_i32,836426271_i32];
_18 = [_12,_12,_12,_12];
_11 = [1349500682_i32,(-1730977641_i32),(-587043196_i32),445043101_i32,22695093_i32,1025996824_i32,(-696222135_i32)];
_21 = Move(_1);
(*_14) = core::ptr::addr_of!(_5);
RET = [(-9108711659302189020_i64),4256070301811950663_i64];
_20 = _13.0;
_13.0 = !_20;
_2 = _6 as isize;
(*_14) = core::ptr::addr_of!(_5);
_20 = _13.0 - _13.0;
(*_14) = core::ptr::addr_of!(_5);
_13.0 = _20 * _20;
_5 = 18399638144401870518_usize >> Field::<isize>(Variant(_13.1.3, 0), 0);
(*_14) = core::ptr::addr_of!(_5);
_2 = Field::<isize>(Variant(_13.1.3, 0), 0) - _8;
_6 = _5 as f32;
_13.0 = _12 as u64;
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
(*_14) = core::ptr::addr_of!(_5);
Goto(bb11)
}
bb11 = {
(*_14) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of_mut!(RET);
RET = [2170111610406035125_i64,6120459352667205396_i64];
(*_1) = [7760001309547193570_i64,(-1031500225580241971_i64)];
_27 = 38142745301183655299578782247626061411_i128 << _8;
_9 = Field::<char>(Variant(_13.1.3, 0), 1);
Call(_32.1.fld2 = fn16(Move((*_14)), Move(_21), (*_1), Move(_13.1.3), (*_1), Move(_14), _22, Move(_1), (*_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_32.1.fld3.0 = 306751539_i32;
RET = [309373862712396091_i64,(-6420346397370416985_i64)];
_15 = _6 as u8;
_32.1.fld1.0 = !554_u16;
Goto(bb13)
}
bb13 = {
_21 = core::ptr::addr_of_mut!(RET);
_8 = -_2;
_13.1.2 = [(-30_i8),(-4_i8),60_i8,(-92_i8),(-122_i8),16_i8,5_i8];
_32.1.fld1.3 = 159460369118005514262275035336951530856_u128 - 308675763737863493122388956558931631446_u128;
_26 = 14_i8 << _2;
(*_21) = [4994048877464628156_i64,(-1819222056146927109_i64)];
_32.1.fld4 = _32.1.fld1.0;
_13.1.0 = _32.1.fld2;
_1 = core::ptr::addr_of_mut!((*_21));
_32.1.fld1.0 = _32.1.fld4;
_32.1.fld1 = (_32.1.fld4, _32.1.fld3.0, _9, 191505856764011611257756164541961254591_u128);
_32.0 = !(-5762_i16);
(*_21) = [(-7393846211811808602_i64),525678701756430867_i64];
_2 = _4;
_32.1.fld1.3 = !93627125758988194632280875259307416689_u128;
_32.1.fld3.0 = _32.1.fld1.1 & _32.1.fld1.1;
_31 = _27 as isize;
(*_21) = [(-7196126786768361632_i64),8284171995256990465_i64];
_32.1.fld4 = _13.0 as u16;
_4 = _22 as isize;
_29 = _9;
(*_21) = [(-6888901844458671108_i64),6523748402180173842_i64];
_34 = _32.1.fld3.0 as f64;
_22 = _34 - _34;
_6 = (-861900542693634329_i64) as f32;
_33 = core::ptr::addr_of!(_19);
_32.1.fld3.0 = _32.1.fld1.1 | _32.1.fld1.1;
match _32.1.fld1.1 {
0 => bb8,
1 => bb4,
2 => bb10,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
306751539 => bb19,
_ => bb18
}
}
bb14 = {
_32.1.fld3.0 = 306751539_i32;
RET = [309373862712396091_i64,(-6420346397370416985_i64)];
_15 = _6 as u8;
_32.1.fld1.0 = !554_u16;
Goto(bb13)
}
bb15 = {
(*_14) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of_mut!(RET);
RET = [2170111610406035125_i64,6120459352667205396_i64];
(*_1) = [7760001309547193570_i64,(-1031500225580241971_i64)];
_27 = 38142745301183655299578782247626061411_i128 << _8;
_9 = Field::<char>(Variant(_13.1.3, 0), 1);
Call(_32.1.fld2 = fn16(Move((*_14)), Move(_21), (*_1), Move(_13.1.3), (*_1), Move(_14), _22, Move(_1), (*_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_13.1.2 = [(-47_i8),61_i8,42_i8,44_i8,127_i8,(-112_i8),(-76_i8)];
_13.1.1.1 = core::ptr::addr_of!(_5);
_17 = Move(_14);
_13.0 = 12242948204688497463_u64 | 14311686004458640802_u64;
_14 = core::ptr::addr_of_mut!(_13.1.1.1);
_8 = Field::<isize>(Variant(_13.1.3, 0), 0) * _4;
(*_14) = core::ptr::addr_of!(_5);
_6 = _12 as f32;
(*_1) = [(-5594869206611836420_i64),6631281577119837213_i64];
(*_14) = core::ptr::addr_of!(_5);
_13.1.0 = [Field::<char>(Variant(_13.1.3, 0), 1),Field::<char>(Variant(_13.1.3, 0), 1),Field::<char>(Variant(_13.1.3, 0), 1),_9,_9,Field::<char>(Variant(_13.1.3, 0), 1),_9];
(*_1) = [3839280715220358098_i64,(-1291903786410907843_i64)];
(*_14) = core::ptr::addr_of!(_5);
Goto(bb8)
}
bb17 = {
_5 = 3070537110516408837_usize | 3887514277553566159_usize;
_6 = 5_i8 as f32;
(*_1) = [(-5378138718588547102_i64),7683791136795761140_i64];
(*_1) = [(-575662725983858853_i64),(-7444371716716387241_i64)];
(*_1) = [8951661526106810177_i64,806289365870514425_i64];
Goto(bb2)
}
bb18 = {
(*_14) = core::ptr::addr_of!(_5);
(*_1) = [3302828633590461197_i64,633302556486948163_i64];
_12 = !116_u8;
(*_1) = [2469276067222166702_i64,1619125525584576954_i64];
RET = [(-6230822897534976120_i64),(-5910480176198083619_i64)];
Goto(bb9)
}
bb19 = {
_32.1.fld3.1 = core::ptr::addr_of!(_5);
(*_21) = [465389161492700875_i64,(-3780836652137325743_i64)];
(*_21) = [(-4179718047373082943_i64),(-5319712248365797795_i64)];
(*_21) = [(-6134362985995443094_i64),(-450524849481860527_i64)];
_23 = !350629106_u32;
(*_21) = [(-6922919638834010612_i64),(-3342334221295962753_i64)];
(*_21) = [(-2248358773599687984_i64),(-5885657286857775140_i64)];
(*_33) = core::ptr::addr_of!(_36);
_37 = 1738883149320219804_i64 << _15;
RET = [_37,_37];
(*_33) = core::ptr::addr_of!((*_19));
_32.1.fld6 = [_32.1.fld1.3,_32.1.fld1.3,_32.1.fld1.3,_32.1.fld1.3,_32.1.fld1.3];
(*_19) = [_20,_13.0,_20,_20];
(*_19) = [_20,_13.0,_20,_13.0];
_32.1.fld3.0 = _32.1.fld1.1;
_17 = core::ptr::addr_of_mut!(_32.1.fld3.1);
(*_21) = [_37,_37];
(*_33) = core::ptr::addr_of!((*_19));
(*_21) = [_37,_37];
Goto(bb20)
}
bb20 = {
(*_19) = [_20,_20,_20,_20];
_40 = _34;
_32.1.fld1.2 = _29;
(*_19) = [_13.0,_20,_13.0,_20];
(*_19) = [_20,_20,_20,_20];
(*_21) = [_37,_37];
_8 = _31 - _31;
(*_33) = core::ptr::addr_of!((*_19));
(*_19) = [_13.0,_20,_20,_20];
(*_17) = core::ptr::addr_of!(_5);
_28 = &_23;
(*_19) = [_13.0,_20,_20,_20];
(*_21) = [_37,_37];
_5 = _23 as usize;
(*_19) = [_13.0,_20,_13.0,_20];
_32.1.fld3.1 = core::ptr::addr_of!(_5);
(*_21) = [_37,_37];
(*_21) = [_37,_37];
_13.1.2 = [_26,_26,_26,_26,_26,_26,_26];
(*_19) = [_13.0,_20,_20,_20];
_14 = Move(_17);
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb21)
}
bb21 = {
(*_33) = core::ptr::addr_of!((*_19));
(*_33) = core::ptr::addr_of!((*_19));
_41 = core::ptr::addr_of!(_5);
_45 = [_8,_31,_8,_8,_31,_31,_31];
Goto(bb22)
}
bb22 = {
_46 = _6;
_15 = _12;
_13.0 = _20 * _20;
(*_21) = [_37,_37];
_13.1.0 = _32.1.fld2;
_13.1.1.2 = &mut _32.1.fld1.1;
_49 = !false;
Goto(bb23)
}
bb23 = {
_34 = _40 - _40;
(*_33) = core::ptr::addr_of!((*_19));
_22 = _40;
_42 = !_27;
_6 = _46 - _46;
(*_33) = core::ptr::addr_of!((*_19));
_42 = _27 ^ _27;
_17 = Move(_14);
(*_33) = core::ptr::addr_of!((*_19));
(*_41) = 6033308296650137938_usize >> _8;
(*_41) = 129559532015258532740061768879423063417_u128 as usize;
(*_19) = [_13.0,_13.0,_13.0,_13.0];
Goto(bb24)
}
bb24 = {
_13.1.1.1 = core::ptr::addr_of!((*_41));
(*_33) = core::ptr::addr_of!((*_19));
(*_19) = [_20,_13.0,_20,_20];
_51 = [_8,_2,_31];
(*_21) = [_37,_37];
Goto(bb25)
}
bb25 = {
_12 = _15 << _13.0;
(*_21) = [_37,_37];
_13.1.3 = Adt22::Variant1 { fld0: Move(_41),fld1: (*_41) };
_38 = _42 * _42;
(*_21) = [_37,_37];
(*_21) = [_37,_37];
(*_33) = core::ptr::addr_of!((*_19));
(*_21) = [_37,_37];
place!(Field::<*const usize>(Variant(_13.1.3, 1), 0)) = Move(_13.1.1.1);
_21 = Move(_1);
_27 = !_38;
_15 = _22 as u8;
_13.0 = _20 << _27;
_40 = _34 * _34;
_29 = _9;
_41 = Move(Field::<*const usize>(Variant(_13.1.3, 1), 0));
_13.1.1.1 = core::ptr::addr_of!(_5);
_13.1.3 = Adt22::Variant1 { fld0: Move(_13.1.1.1),fld1: _5 };
_54 = _2 * _2;
_13.1.0 = [_29,_29,_9,_9,_29,_29,_29];
_55 = _8 >> _5;
_13.1.1.1 = core::ptr::addr_of!(_5);
(*_19) = [_13.0,_13.0,_13.0,_20];
_2 = _54 - _55;
Goto(bb26)
}
bb26 = {
_44 = _6;
_5 = Field::<usize>(Variant(_13.1.3, 1), 1) & Field::<usize>(Variant(_13.1.3, 1), 1);
_39 = (-11806_i16);
(*_19) = [_13.0,_20,_13.0,_13.0];
_15 = _12 | _12;
_38 = _39 as i128;
_29 = _9;
_58.0 = _13.1.0;
(*_33) = core::ptr::addr_of!((*_19));
(*_19) = [_13.0,_13.0,_13.0,_13.0];
_58.0 = [_9,_9,_9,_9,_29,_9,_29];
Goto(bb27)
}
bb27 = {
(*_19) = [_13.0,_13.0,_13.0,_13.0];
_2 = _8 | _31;
_2 = -_55;
_31 = _2 - _2;
(*_33) = core::ptr::addr_of!((*_19));
_13.1.2 = [_26,_26,_26,_26,_26,_26,_26];
_47 = _54 << _55;
place!(Field::<usize>(Variant(_13.1.3, 1), 1)) = _5 & _5;
(*_19) = [_13.0,_13.0,_13.0,_13.0];
_3 = &_28;
(*_19) = [_13.0,_20,_13.0,_20];
_22 = _40 + _40;
_58.0 = _13.1.0;
_34 = 54017_u16 as f64;
_56 = _8 as u64;
Goto(bb28)
}
bb28 = {
(*_19) = [_13.0,_13.0,_56,_13.0];
_51 = [_31,_8,_31];
(*_19) = [_13.0,_56,_56,_13.0];
_13.1.0 = _58.0;
(*_33) = core::ptr::addr_of!((*_19));
_13.1.0 = [_29,_29,_9,_9,_9,_9,_9];
_52 = -_6;
_61 = _40 * _22;
_1 = Move(_21);
_43 = [4464_u16,62435_u16,10253_u16,36303_u16];
(*_33) = core::ptr::addr_of!((*_19));
(*_19) = [_56,_13.0,_13.0,_13.0];
(*_33) = core::ptr::addr_of!((*_19));
_19 = core::ptr::addr_of!((*_19));
_40 = -_22;
(*_33) = core::ptr::addr_of!((*_19));
_62 = _29;
(*_19) = [_13.0,_56,_20,_56];
Goto(bb29)
}
bb29 = {
_18 = [_15,_15,_12,_15];
_13.0 = _56;
(*_33) = core::ptr::addr_of!((*_19));
_27 = !_42;
(*_33) = core::ptr::addr_of!((*_19));
_58.3 = Adt22::Variant1 { fld0: Move(Field::<*const usize>(Variant(_13.1.3, 1), 0)),fld1: _5 };
(*_33) = core::ptr::addr_of!((*_19));
_1 = core::ptr::addr_of_mut!(RET);
_29 = _9;
match _39 {
0 => bb8,
340282366920938463463374607431768199650 => bb30,
_ => bb17
}
}
bb30 = {
_18 = [_15,_15,_15,_12];
(*_33) = core::ptr::addr_of!((*_19));
(*_33) = core::ptr::addr_of!((*_19));
_51 = [_47,_31,_54];
_37 = 5089505376217183346_i64 & (-2977919084948156800_i64);
(*_19) = [_56,_13.0,_13.0,_13.0];
_12 = _15;
_11 = [(-1262529484_i32),1637860243_i32,(-201602545_i32),1488616748_i32,(-1275214950_i32),684776426_i32,1344423213_i32];
(*_33) = core::ptr::addr_of!((*_19));
_33 = core::ptr::addr_of!((*_33));
_38 = _40 as i128;
(*_33) = core::ptr::addr_of!((*_19));
(*_19) = [_56,_13.0,_56,_13.0];
_5 = !Field::<usize>(Variant(_58.3, 1), 1);
Goto(bb31)
}
bb31 = {
(*_33) = core::ptr::addr_of!((*_19));
_58.2 = [_26,_26,_26,_26,_26,_26,_26];
_13.1.0 = [_62,_9,_62,_9,_29,_29,_9];
_22 = _61 - _40;
(*_19) = [_13.0,_13.0,_56,_13.0];
(*_33) = core::ptr::addr_of!((*_19));
Goto(bb32)
}
bb32 = {
_29 = _62;
_56 = !_13.0;
(*_33) = core::ptr::addr_of!((*_19));
_65 = (-328859049_i32) as isize;
_22 = _61 + _40;
_52 = _44 + _6;
place!(Field::<usize>(Variant(_58.3, 1), 1)) = _5;
_37 = 3634408802493286213_i64 << (*_28);
(*_19) = [_13.0,_56,_13.0,_56];
_13.1.3 = Adt22::Variant1 { fld0: Move(_13.1.1.1),fld1: Field::<usize>(Variant(_58.3, 1), 1) };
(*_33) = core::ptr::addr_of!((*_19));
_69 = _55 as u32;
_39 = 5450_i16;
(*_33) = core::ptr::addr_of!((*_19));
_33 = core::ptr::addr_of!((*_33));
_64 = [(-669868510_i32),729935859_i32];
_58.1.2 = Move(_13.1.1.2);
_33 = core::ptr::addr_of!((*_33));
Goto(bb33)
}
bb33 = {
_4 = !_2;
_33 = core::ptr::addr_of!((*_33));
(*_33) = core::ptr::addr_of!((*_19));
_41 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_58.3, 1), 1)));
_39 = 61681231610036182958189237044423692697_u128 as i16;
(*_33) = core::ptr::addr_of!((*_19));
(*_41) = Field::<usize>(Variant(_13.1.3, 1), 1) | _5;
_70 = _26 as u32;
Call((*_1) = core::intrinsics::transmute(_38), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_29 = _62;
(*_1) = [_37,_37];
_37 = (-2713610338869176238_i64) >> _2;
_2 = _42 as isize;
(*_41) = _5;
(*_1) = [_37,_37];
(*_1) = [_37,_37];
(*_19) = [_13.0,_13.0,_56,_13.0];
(*_19) = [_20,_13.0,_13.0,_13.0];
(*_41) = _9 as usize;
(*_19) = [_56,_20,_13.0,_56];
_80 = _13.0 < _13.0;
(*_33) = core::ptr::addr_of!((*_19));
_81 = _80;
_34 = _40 + _22;
_6 = -_52;
(*_41) = !_5;
_19 = core::ptr::addr_of!((*_19));
(*_19) = [_13.0,_13.0,_20,_56];
(*_19) = [_56,_13.0,_56,_13.0];
(*_33) = core::ptr::addr_of!((*_19));
_75 = 28355291825004098449981254209855692125_u128 as f32;
Goto(bb35)
}
bb35 = {
Call(_84 = dump_var(Move(_36), Move(_42), Move(_4), Move(_56)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_84 = dump_var(Move(_80), Move(_37), Move(_64), Move(_43)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_84 = dump_var(Move(_9), Move(_65), Move(_29), Move(_8)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_84 = dump_var(Move(_62), Move(_31), Move(_51), Move(_54)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_84 = dump_var(Move(_55), Move(_38), _85, _85), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const usize,mut _2: *mut [i64; 2],mut _3: [i64; 2],mut _4: Adt22,mut _5: [i64; 2],mut _6: *mut *const usize,mut _7: f64,mut _8: *mut [i64; 2],mut _9: [i64; 2]) -> [char; 7] {
mir! {
type RET = [char; 7];
let _10: [i8; 7];
let _11: u8;
let _12: f32;
let _13: i64;
let _14: f32;
let _15: f32;
let _16: u8;
let _17: i32;
let _18: f32;
let _19: isize;
let _20: *const f64;
let _21: *const (i32, *const usize);
let _22: [isize; 5];
let _23: char;
let _24: u64;
let _25: *mut Adt48;
let _26: [u64; 4];
let _27: isize;
let _28: isize;
let _29: i32;
let _30: i32;
let _31: isize;
let _32: [isize; 7];
let _33: *const [u64; 4];
let _34: f32;
let _35: f64;
let _36: u16;
let _37: isize;
let _38: [i8; 7];
let _39: i32;
let _40: u128;
let _41: &'static mut u128;
let _42: &'static &'static u32;
let _43: *const *const [u64; 4];
let _44: *const (i32, *const usize);
let _45: u64;
let _46: &'static Adt25;
let _47: *mut *const usize;
let _48: &'static u32;
let _49: *const &'static mut (i32, *const usize);
let _50: bool;
let _51: *mut (i128, &'static mut u128);
let _52: i32;
let _53: *mut *const usize;
let _54: *mut *const usize;
let _55: bool;
let _56: *const *const [u64; 4];
let _57: *mut (i128, &'static mut u128);
let _58: char;
let _59: *const usize;
let _60: *const &'static mut (i128, &'static mut u128);
let _61: &'static mut &'static mut (i128, &'static mut u128);
let _62: isize;
let _63: *mut [i64; 2];
let _64: &'static mut (i128, &'static mut u128);
let _65: i32;
let _66: *const *const [u64; 4];
let _67: bool;
let _68: &'static mut (i128, &'static mut u128);
let _69: char;
let _70: &'static mut (i32, *const usize);
let _71: char;
let _72: &'static mut i32;
let _73: *const usize;
let _74: i16;
let _75: bool;
let _76: u64;
let _77: ([i64; 2], [u128; 5], &'static mut u128);
let _78: &'static u32;
let _79: *const &'static mut (i128, &'static mut u128);
let _80: f32;
let _81: *const u8;
let _82: *const f64;
let _83: f32;
let _84: char;
let _85: (u8, usize, isize);
let _86: f64;
let _87: [isize; 5];
let _88: Adt48;
let _89: bool;
let _90: [isize; 5];
let _91: &'static Adt25;
let _92: char;
let _93: &'static mut &'static mut (i128, &'static mut u128);
let _94: i8;
let _95: ();
let _96: ();
{
_2 = core::ptr::addr_of_mut!(_9);
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{24d1b}';
(*_2) = [(-7119161911957839676_i64),5726383244092469243_i64];
(*_2) = [6983090067502956233_i64,(-5252552599051209352_i64)];
_7 = 153_u8 as f64;
(*_2) = [(-9194831725658729701_i64),1351377905369541222_i64];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [3000681815455735903_i64,2218025299170383958_i64];
_6 = core::ptr::addr_of_mut!(_1);
RET = [Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1)];
(*_2) = _3;
(*_2) = [3195744774514582397_i64,2835096721439241861_i64];
place!(Field::<isize>(Variant(_4, 0), 0)) = 9223372036854775807_isize << 40_u8;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{84599}';
(*_2) = _3;
_8 = Move(_2);
_7 = 782099336218336069_i64 as f64;
_2 = core::ptr::addr_of_mut!(_9);
_11 = 19_u8 - 198_u8;
Goto(bb1)
}
bb1 = {
(*_2) = [(-1153990258596951215_i64),1866465061394663383_i64];
_4 = Adt22::Variant0 { fld0: (-40_isize),fld1: '\u{d6070}' };
_12 = 5_usize as f32;
Goto(bb2)
}
bb2 = {
_5 = [7138731866141487837_i64,8492913491718002125_i64];
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = [8200476109087570047_i64,4284863361196384322_i64];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{bf737}';
(*_2) = _5;
_10 = [60_i8,50_i8,51_i8,(-11_i8),35_i8,(-28_i8),(-113_i8)];
(*_2) = _5;
_5 = [(-3384890550859648944_i64),(-5416193739146512500_i64)];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{38c70}';
(*_2) = _5;
(*_2) = _3;
(*_2) = [(-2211447276039678829_i64),1292431043112812808_i64];
_13 = (-9189968066149708722168362453401091444_i128) as i64;
(*_2) = [_13,_13];
_4 = Adt22::Variant1 { fld0: Move((*_6)),fld1: 17890368280127413047_usize };
_7 = (-77592234565992414686661379477397402800_i128) as f64;
(*_2) = [_13,_13];
place!(Field::<usize>(Variant(_4, 1), 1)) = 2_usize;
(*_2) = [_13,_13];
_9 = _3;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 1), 1)));
(*_1) = 1_usize - 7208217351960610096_usize;
(*_1) = 4_usize << _11;
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
Call((*_2) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_2) = [_13,_13];
RET = ['\u{29cff}','\u{7a85}','\u{93d8f}','\u{a094d}','\u{39211}','\u{2e00d}','\u{a054b}'];
_17 = (-1431596389_i32) << (*_1);
(*_1) = 12255757496542680618_usize << _17;
_11 = 137_u8;
(*_2) = [_13,_13];
_13 = 2499142286961145577_u64 as i64;
RET = ['\u{be59a}','\u{98608}','\u{1101b}','\u{17502}','\u{827af}','\u{a37ac}','\u{ddaaf}'];
(*_1) = 14942388718056006945_u64 as usize;
(*_2) = [_13,_13];
(*_6) = core::ptr::addr_of!((*_1));
(*_2) = [_13,_13];
(*_2) = _5;
(*_1) = (-9223372036854775808_isize) as usize;
_8 = core::ptr::addr_of_mut!((*_2));
_10 = [(-8_i8),(-37_i8),98_i8,(-12_i8),(-73_i8),(-47_i8),57_i8];
RET = ['\u{ad22b}','\u{5f265}','\u{2c73b}','\u{1b10d}','\u{7c7cf}','\u{bc021}','\u{2b566}'];
(*_2) = _3;
_18 = _12 * _12;
(*_1) = 14182801247438828267_usize | 17992579781570336339_usize;
(*_6) = core::ptr::addr_of!((*_1));
Goto(bb4)
}
bb4 = {
_23 = '\u{736aa}';
_19 = (-2_isize);
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
_8 = core::ptr::addr_of_mut!((*_2));
_4 = Adt22::Variant0 { fld0: _19,fld1: _23 };
(*_8) = [_13,_13];
Goto(bb5)
}
bb5 = {
(*_8) = [_13,_13];
(*_2) = _3;
(*_2) = _5;
(*_2) = [_13,_13];
(*_2) = [_13,_13];
(*_2) = [_13,_13];
_16 = _11;
_9 = _3;
(*_2) = _5;
_8 = core::ptr::addr_of_mut!((*_2));
_15 = _11 as f32;
place!(Field::<isize>(Variant(_4, 0), 0)) = _19;
_2 = Move(_8);
_12 = _18 * _15;
_11 = _16 + _16;
Goto(bb6)
}
bb6 = {
_8 = core::ptr::addr_of_mut!(_9);
_22 = [Field::<isize>(Variant(_4, 0), 0),Field::<isize>(Variant(_4, 0), 0),Field::<isize>(Variant(_4, 0), 0),Field::<isize>(Variant(_4, 0), 0),_19];
_14 = 33_i8 as f32;
_20 = core::ptr::addr_of!(_7);
_2 = core::ptr::addr_of_mut!((*_8));
(*_20) = 37001_u16 as f64;
(*_8) = [_13,_13];
(*_20) = 7249792273566576318_u64 as f64;
_13 = -4298261863854607993_i64;
(*_8) = [_13,_13];
_30 = true as i32;
(*_20) = 17077933012250735392_usize as f64;
(*_20) = 15590347256443282577_u64 as f64;
(*_8) = [_13,_13];
_17 = !_30;
(*_8) = [_13,_13];
_29 = _17 - _30;
(*_20) = (-106_i8) as f64;
(*_8) = _5;
_28 = 10732_i16 as isize;
RET = [_23,_23,_23,Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),_23,Field::<char>(Variant(_4, 0), 1)];
_30 = -_29;
Goto(bb7)
}
bb7 = {
_15 = 842596135_u32 as f32;
(*_20) = 76536340336794707449300859254424323469_i128 as f64;
_10 = [44_i8,(-22_i8),53_i8,54_i8,1_i8,(-89_i8),(-11_i8)];
_3 = (*_8);
Goto(bb8)
}
bb8 = {
(*_8) = _3;
(*_8) = [_13,_13];
_29 = (-54_i8) as i32;
(*_8) = _3;
(*_8) = [_13,_13];
_27 = 51930_u16 as isize;
(*_20) = 4324676497685925390_u64 as f64;
(*_20) = 2645830332_u32 as f64;
_3 = [_13,_13];
(*_20) = 2387259616_u32 as f64;
_9 = [_13,_13];
(*_20) = 162146335875381632006766975977095899766_u128 as f64;
Goto(bb9)
}
bb9 = {
(*_20) = 256612515372728984321706462512962788755_u128 as f64;
_31 = Field::<isize>(Variant(_4, 0), 0) & _27;
(*_8) = [_13,_13];
(*_20) = _19 as f64;
_30 = _17 - _29;
_33 = core::ptr::addr_of!(_26);
(*_33) = [3212472065839955292_u64,17039691816670317032_u64,7287505667453089858_u64,7708434861315966847_u64];
(*_20) = _30 as f64;
(*_33) = [12097619767398747005_u64,11017607453780620268_u64,2749905715199137073_u64,8049413009812123177_u64];
(*_33) = [8541829390974329828_u64,15856745529352009128_u64,14762535658442620063_u64,3156829043961016164_u64];
(*_20) = 102_i8 as f64;
(*_20) = _13 as f64;
_17 = !_30;
_35 = (*_20) + (*_20);
(*_20) = -_35;
(*_20) = -_35;
_17 = _30 + _29;
(*_33) = [15463937489578151580_u64,17740650795772578778_u64,3035283679562639522_u64,8939119888867352460_u64];
(*_33) = [4004503666325366702_u64,6500354245244629137_u64,4863706363140191147_u64,1792207620605929626_u64];
(*_33) = [11695511503228823543_u64,15066582832596852140_u64,10332590388224380379_u64,12291872558828046881_u64];
Goto(bb10)
}
bb10 = {
(*_8) = _3;
_9 = [_13,_13];
(*_33) = [8803995785163072412_u64,10588613240863906293_u64,15881777764495484865_u64,1526412433272096072_u64];
_28 = _19 & Field::<isize>(Variant(_4, 0), 0);
(*_20) = -_35;
(*_20) = _35 * _35;
_37 = !_28;
(*_33) = [1795706505348742082_u64,18173085486297918013_u64,10625854358728278234_u64,8025183861958469236_u64];
_8 = core::ptr::addr_of_mut!((*_8));
(*_33) = [9641084326939576850_u64,5369784435798027372_u64,14166700676708054102_u64,17222839682616869592_u64];
_19 = _37;
(*_33) = [11132868684682191044_u64,14938667651192743374_u64,12003557997154788564_u64,12091027382629093242_u64];
(*_8) = _5;
(*_8) = [_13,_13];
(*_20) = _35 + _35;
_14 = 35_i8 as f32;
(*_33) = [14040296552609069411_u64,11188138791108137490_u64,5892827065052532699_u64,15484320266460223699_u64];
(*_20) = _35 + _35;
_7 = _16 as f64;
(*_8) = _3;
_7 = _35 - _35;
_23 = Field::<char>(Variant(_4, 0), 1);
_34 = (*_20) as f32;
(*_20) = _35;
match _16 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb7,
137 => bb11,
_ => bb6
}
}
bb11 = {
(*_20) = _35;
_40 = !89092564442775082845482504404799594141_u128;
_16 = _11 >> Field::<isize>(Variant(_4, 0), 0);
(*_33) = [13322717959965782178_u64,12369615381173901799_u64,2619581271722034173_u64,9782842609023491939_u64];
_27 = -_28;
(*_20) = _35 + _35;
(*_20) = -_35;
(*_8) = _5;
(*_33) = [13529508579090698636_u64,10529890051174512086_u64,736037540829854495_u64,10096162647343957568_u64];
(*_20) = _35 - _35;
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
(*_33) = [999155666558370258_u64,10403137715630918246_u64,2448296148896033817_u64,17391235699707954644_u64];
(*_33) = [46890045048773967_u64,8864041400298804731_u64,17409410069884289_u64,12681434463123615049_u64];
_20 = core::ptr::addr_of!((*_20));
(*_20) = -_35;
_32 = [_28,_28,_28,_19,_19,Field::<isize>(Variant(_4, 0), 0),_31];
_2 = core::ptr::addr_of_mut!((*_8));
(*_8) = [_13,_13];
(*_33) = [15696636520848409901_u64,11898099858112413789_u64,6395901165230102995_u64,9770166910498768107_u64];
(*_33) = [12746628457555305783_u64,13464092363152304358_u64,15209124432262486799_u64,4638806257402883855_u64];
_41 = &mut _40;
(*_20) = _35 + _35;
(*_20) = 900001141_u32 as f64;
_10 = [(-111_i8),(-9_i8),19_i8,17_i8,84_i8,(-77_i8),(-125_i8)];
(*_8) = [_13,_13];
(*_8) = _3;
(*_20) = _35 - _35;
match Field::<isize>(Variant(_4, 0), 0) {
0 => bb10,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768211454 => bb17,
_ => bb16
}
}
bb12 = {
_5 = [7138731866141487837_i64,8492913491718002125_i64];
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = [8200476109087570047_i64,4284863361196384322_i64];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{bf737}';
(*_2) = _5;
_10 = [60_i8,50_i8,51_i8,(-11_i8),35_i8,(-28_i8),(-113_i8)];
(*_2) = _5;
_5 = [(-3384890550859648944_i64),(-5416193739146512500_i64)];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{38c70}';
(*_2) = _5;
(*_2) = _3;
(*_2) = [(-2211447276039678829_i64),1292431043112812808_i64];
_13 = (-9189968066149708722168362453401091444_i128) as i64;
(*_2) = [_13,_13];
_4 = Adt22::Variant1 { fld0: Move((*_6)),fld1: 17890368280127413047_usize };
_7 = (-77592234565992414686661379477397402800_i128) as f64;
(*_2) = [_13,_13];
place!(Field::<usize>(Variant(_4, 1), 1)) = 2_usize;
(*_2) = [_13,_13];
_9 = _3;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 1), 1)));
(*_1) = 1_usize - 7208217351960610096_usize;
(*_1) = 4_usize << _11;
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
Call((*_2) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
(*_2) = [_13,_13];
RET = ['\u{29cff}','\u{7a85}','\u{93d8f}','\u{a094d}','\u{39211}','\u{2e00d}','\u{a054b}'];
_17 = (-1431596389_i32) << (*_1);
(*_1) = 12255757496542680618_usize << _17;
_11 = 137_u8;
(*_2) = [_13,_13];
_13 = 2499142286961145577_u64 as i64;
RET = ['\u{be59a}','\u{98608}','\u{1101b}','\u{17502}','\u{827af}','\u{a37ac}','\u{ddaaf}'];
(*_1) = 14942388718056006945_u64 as usize;
(*_2) = [_13,_13];
(*_6) = core::ptr::addr_of!((*_1));
(*_2) = [_13,_13];
(*_2) = _5;
(*_1) = (-9223372036854775808_isize) as usize;
_8 = core::ptr::addr_of_mut!((*_2));
_10 = [(-8_i8),(-37_i8),98_i8,(-12_i8),(-73_i8),(-47_i8),57_i8];
RET = ['\u{ad22b}','\u{5f265}','\u{2c73b}','\u{1b10d}','\u{7c7cf}','\u{bc021}','\u{2b566}'];
(*_2) = _3;
_18 = _12 * _12;
(*_1) = 14182801247438828267_usize | 17992579781570336339_usize;
(*_6) = core::ptr::addr_of!((*_1));
Goto(bb4)
}
bb14 = {
(*_8) = _3;
(*_8) = [_13,_13];
_29 = (-54_i8) as i32;
(*_8) = _3;
(*_8) = [_13,_13];
_27 = 51930_u16 as isize;
(*_20) = 4324676497685925390_u64 as f64;
(*_20) = 2645830332_u32 as f64;
_3 = [_13,_13];
(*_20) = 2387259616_u32 as f64;
_9 = [_13,_13];
(*_20) = 162146335875381632006766975977095899766_u128 as f64;
Goto(bb9)
}
bb15 = {
_15 = 842596135_u32 as f32;
(*_20) = 76536340336794707449300859254424323469_i128 as f64;
_10 = [44_i8,(-22_i8),53_i8,54_i8,1_i8,(-89_i8),(-11_i8)];
_3 = (*_8);
Goto(bb8)
}
bb16 = {
_23 = '\u{736aa}';
_19 = (-2_isize);
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
_8 = core::ptr::addr_of_mut!((*_2));
_4 = Adt22::Variant0 { fld0: _19,fld1: _23 };
(*_8) = [_13,_13];
Goto(bb5)
}
bb17 = {
_7 = _35 * _35;
(*_41) = 35922562346699164040013266193078788311_u128 - 36317168575160522955267580557217331049_u128;
_29 = !_17;
_24 = 1604789064248360412_u64;
_26 = [_24,_24,_24,_24];
Goto(bb18)
}
bb18 = {
(*_8) = [_13,_13];
(*_8) = [_13,_13];
(*_33) = [_24,_24,_24,_24];
(*_20) = _35 - _35;
_13 = 2358205434029279120_i64 - 1704075393498566721_i64;
(*_8) = _3;
(*_8) = [_13,_13];
(*_20) = _35 + _35;
(*_33) = [_24,_24,_24,_24];
(*_8) = _5;
_26 = [_24,_24,_24,_24];
(*_20) = _35 * _35;
_35 = -(*_20);
_30 = _17;
_4 = Adt22::Variant0 { fld0: _28,fld1: _23 };
_23 = Field::<char>(Variant(_4, 0), 1);
(*_41) = 184999596057037927069216860410909896880_u128 - 339422729356575568049378978002163848413_u128;
(*_8) = [_13,_13];
(*_20) = _27 as f64;
(*_20) = _35 - _35;
(*_33) = [_24,_24,_24,_24];
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb19)
}
bb19 = {
_24 = !14700470226597442845_u64;
_33 = core::ptr::addr_of!((*_33));
_39 = _29 + _30;
(*_41) = 168836442877213759168298265673391872979_u128;
(*_33) = [_24,_24,_24,_24];
(*_41) = 57464077831873531209299207060830290908_u128 ^ 72089859021337299652583010598068282861_u128;
(*_41) = 106987822093246903959025014999352546552_u128 ^ 324272923138568367037662478931842390466_u128;
Goto(bb20)
}
bb20 = {
(*_41) = 185131135051727376169085351005305964057_u128;
(*_20) = _35;
(*_33) = [_24,_24,_24,_24];
_22 = [Field::<isize>(Variant(_4, 0), 0),_28,_19,_28,_31];
(*_20) = _35;
_12 = _24 as f32;
(*_33) = [_24,_24,_24,_24];
_43 = core::ptr::addr_of!(_33);
_43 = core::ptr::addr_of!((*_43));
(*_8) = [_13,_13];
_36 = (*_41) as u16;
(*_8) = _3;
(*_20) = _35;
_18 = _34 + _34;
(*_33) = [_24,_24,_24,_24];
(*_8) = [_13,_13];
_7 = _35;
(*_43) = core::ptr::addr_of!((*_33));
_11 = _16 << _39;
_22 = [_19,_28,_31,_19,_27];
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
match (*_41) {
0 => bb17,
1 => bb21,
185131135051727376169085351005305964057 => bb23,
_ => bb22
}
}
bb21 = {
_5 = [7138731866141487837_i64,8492913491718002125_i64];
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = [8200476109087570047_i64,4284863361196384322_i64];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{bf737}';
(*_2) = _5;
_10 = [60_i8,50_i8,51_i8,(-11_i8),35_i8,(-28_i8),(-113_i8)];
(*_2) = _5;
_5 = [(-3384890550859648944_i64),(-5416193739146512500_i64)];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{38c70}';
(*_2) = _5;
(*_2) = _3;
(*_2) = [(-2211447276039678829_i64),1292431043112812808_i64];
_13 = (-9189968066149708722168362453401091444_i128) as i64;
(*_2) = [_13,_13];
_4 = Adt22::Variant1 { fld0: Move((*_6)),fld1: 17890368280127413047_usize };
_7 = (-77592234565992414686661379477397402800_i128) as f64;
(*_2) = [_13,_13];
place!(Field::<usize>(Variant(_4, 1), 1)) = 2_usize;
(*_2) = [_13,_13];
_9 = _3;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 1), 1)));
(*_1) = 1_usize - 7208217351960610096_usize;
(*_1) = 4_usize << _11;
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
Call((*_2) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb22 = {
(*_8) = [_13,_13];
(*_8) = [_13,_13];
(*_33) = [_24,_24,_24,_24];
(*_20) = _35 - _35;
_13 = 2358205434029279120_i64 - 1704075393498566721_i64;
(*_8) = _3;
(*_8) = [_13,_13];
(*_20) = _35 + _35;
(*_33) = [_24,_24,_24,_24];
(*_8) = _5;
_26 = [_24,_24,_24,_24];
(*_20) = _35 * _35;
_35 = -(*_20);
_30 = _17;
_4 = Adt22::Variant0 { fld0: _28,fld1: _23 };
_23 = Field::<char>(Variant(_4, 0), 1);
(*_41) = 184999596057037927069216860410909896880_u128 - 339422729356575568049378978002163848413_u128;
(*_8) = [_13,_13];
(*_20) = _27 as f64;
(*_20) = _35 - _35;
(*_33) = [_24,_24,_24,_24];
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb19)
}
bb23 = {
(*_43) = core::ptr::addr_of!((*_33));
_38 = [(-84_i8),79_i8,125_i8,(-18_i8),(-96_i8),119_i8,60_i8];
(*_8) = _5;
(*_43) = core::ptr::addr_of!((*_33));
_27 = 20927_i16 as isize;
(*_33) = [_24,_24,_24,_24];
(*_8) = [_13,_13];
(*_8) = [_13,_13];
(*_41) = !142991607500338776590557803254133838091_u128;
_50 = _11 != _11;
_47 = core::ptr::addr_of_mut!((*_6));
(*_8) = _3;
_8 = Move(_2);
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
(*_41) = 69995057821304281687304067066119745379_u128 | 217465467455008384760616343979464315564_u128;
_29 = _17 - _39;
(*_41) = 8891733311064153560302436275957767542_i128 as u128;
(*_41) = 29789625129504131836131815980400131132_u128;
(*_33) = [_24,_24,_24,_24];
_13 = (-961779800092878466_i64) - (-2264239803063237766_i64);
Call((*_20) = core::intrinsics::transmute(Field::<isize>(Variant(_4, 0), 0)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_17 = _30 << (*_41);
(*_41) = 226855269995945904917007820507532491371_u128;
(*_41) = 109863857465854775885521718789289071198_u128 & 277923250281448944646772044807304135670_u128;
_8 = core::ptr::addr_of_mut!(_5);
(*_20) = _13 as f64;
(*_41) = _50 as u128;
(*_8) = [_13,_13];
_52 = _29;
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
_55 = _50 | _50;
_24 = !7468047804982929644_u64;
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
(*_41) = _13 as u128;
(*_41) = _24 as u128;
(*_41) = 302411181135896539055550082847142462421_u128;
Goto(bb25)
}
bb25 = {
_45 = !_24;
(*_41) = !76492395889125080733761932114301185841_u128;
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
_2 = Move(_8);
(*_20) = _35;
_30 = -_29;
_27 = _30 as isize;
(*_41) = !336323831120520602582107498322220057751_u128;
(*_33) = [_24,_45,_24,_45];
(*_33) = [_24,_24,_24,_24];
(*_41) = Field::<char>(Variant(_4, 0), 1) as u128;
RET = [_23,_23,Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1)];
_32 = [_28,_37,_27,_27,_27,_27,_27];
(*_20) = (*_41) as f64;
_30 = _52 | _17;
Goto(bb26)
}
bb26 = {
(*_20) = _35 * _35;
(*_33) = [_45,_24,_45,_45];
(*_41) = 164633065960356534007446944525482812623_u128 << _29;
(*_33) = [_24,_45,_45,_24];
(*_33) = [_45,_45,_24,_24];
_19 = !_27;
(*_43) = core::ptr::addr_of!((*_33));
_15 = _34 + _18;
(*_20) = _35;
(*_20) = _35 * _35;
(*_41) = 115647707869557827537065475303819801678_u128 | 101975214727829544151181504504318514095_u128;
(*_20) = _35;
(*_20) = _35 + _35;
_38 = [98_i8,82_i8,119_i8,1_i8,37_i8,29_i8,(-113_i8)];
_50 = (*_20) >= (*_20);
Goto(bb27)
}
bb27 = {
_5 = [_13,_13];
(*_33) = [_45,_45,_45,_45];
(*_33) = [_24,_24,_45,_24];
(*_41) = 49100761803709594280720217353284081929_u128 << _52;
(*_43) = core::ptr::addr_of!((*_33));
(*_41) = 172725216984976248642937338119258867808_u128 >> _45;
(*_33) = [_24,_45,_45,_45];
_23 = Field::<char>(Variant(_4, 0), 1);
(*_20) = _45 as f64;
(*_33) = [_24,_45,_45,_45];
(*_20) = _35 * _35;
Call(_66 = fn17(Move((*_43)), Move((*_6)), Move(_41), Move(_20), _23, _34, (*_41), Move(_43), Move(_6), Move(_2)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_7 = -_35;
_8 = core::ptr::addr_of_mut!(_5);
(*_8) = _3;
_43 = core::ptr::addr_of!(_33);
_33 = core::ptr::addr_of!(_26);
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
(*_33) = [_24,_45,_24,_45];
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
(*_8) = [_13,_13];
_45 = _15 as u64;
place!(Field::<isize>(Variant(_4, 0), 0)) = _28 << _52;
_37 = 3536549884_u32 as isize;
(*_43) = core::ptr::addr_of!((*_33));
_3 = (*_8);
(*_8) = [_13,_13];
_38 = [92_i8,(-58_i8),15_i8,(-116_i8),66_i8,62_i8,122_i8];
(*_8) = _3;
_19 = Field::<isize>(Variant(_4, 0), 0);
_14 = _15;
(*_33) = [_45,_45,_45,_45];
_37 = _18 as isize;
Goto(bb29)
}
bb29 = {
(*_8) = [_13,_13];
_23 = Field::<char>(Variant(_4, 0), 1);
_20 = core::ptr::addr_of!(_7);
(*_20) = _35;
_28 = !_27;
_67 = _50 | _55;
_22 = [Field::<isize>(Variant(_4, 0), 0),_19,Field::<isize>(Variant(_4, 0), 0),Field::<isize>(Variant(_4, 0), 0),_37];
_6 = Move(_47);
(*_20) = _35;
(*_20) = _35 * _35;
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
_58 = _23;
(*_33) = [_45,_24,_45,_45];
(*_33) = [_45,_45,_45,_45];
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
(*_20) = -_35;
(*_33) = [_45,_45,_45,_24];
_30 = _29 >> Field::<isize>(Variant(_4, 0), 0);
_26 = [_45,_45,_45,_45];
_35 = -(*_20);
(*_20) = _35 - _35;
_53 = core::ptr::addr_of_mut!(_1);
_69 = _58;
_37 = _31;
_53 = core::ptr::addr_of_mut!((*_53));
Goto(bb30)
}
bb30 = {
_17 = _29 & _29;
(*_33) = [_45,_45,_45,_45];
_62 = Field::<isize>(Variant(_4, 0), 0) - _19;
_6 = core::ptr::addr_of_mut!((*_53));
_63 = Move(_8);
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
_69 = _58;
(*_20) = _35;
_30 = _52;
(*_20) = -_35;
(*_33) = [_45,_45,_45,_45];
(*_43) = core::ptr::addr_of!((*_33));
(*_20) = _11 as f64;
_35 = (*_20);
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
Goto(bb31)
}
bb31 = {
_8 = core::ptr::addr_of_mut!(_5);
_16 = _11 * _11;
(*_43) = core::ptr::addr_of!((*_33));
(*_20) = _35;
_50 = _67;
(*_43) = core::ptr::addr_of!((*_33));
_29 = _52 * _17;
(*_20) = _35 + _35;
_67 = _55 & _50;
(*_43) = core::ptr::addr_of!((*_33));
_38 = _10;
(*_8) = [_13,_13];
_55 = _67 <= _67;
(*_20) = _35;
(*_8) = [_13,_13];
(*_33) = [_45,_45,_45,_45];
_45 = !_24;
_27 = _11 as isize;
_50 = (*_20) >= (*_20);
Call(_11 = fn19(Move(_20), Move(_53), Move(_4), Move(_6), (*_33), Move((*_43)), (*_20), (*_8), _28, (*_8)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_47 = core::ptr::addr_of_mut!(_1);
(*_43) = core::ptr::addr_of!(_26);
_27 = _62;
_6 = Move(_47);
(*_43) = core::ptr::addr_of!((*_33));
Goto(bb33)
}
bb33 = {
(*_8) = [_13,_13];
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
(*_33) = [_45,_45,_45,_24];
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
_10 = [30_i8,(-85_i8),(-57_i8),(-99_i8),(-108_i8),(-3_i8),85_i8];
_36 = 34302_u16;
_50 = !_67;
(*_8) = _3;
_8 = core::ptr::addr_of_mut!((*_8));
_31 = _27 >> _11;
_53 = core::ptr::addr_of_mut!(_1);
_23 = _69;
_11 = !_16;
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
_76 = _45;
Goto(bb34)
}
bb34 = {
(*_33) = [_76,_45,_76,_24];
(*_33) = [_24,_24,_76,_24];
_77.1 = [329011150720258416409619069745253623393_u128,336681022961040963232614277960786938314_u128,147368528485352007044851587542629246214_u128,148834065035206251610708588408363324726_u128,188414472690758301635241087097203578916_u128];
(*_33) = [_76,_24,_45,_76];
(*_33) = [_76,_24,_45,_24];
_15 = -_18;
_66 = Move(_43);
_31 = _19 >> _29;
_20 = core::ptr::addr_of!(_35);
(*_8) = [_13,_13];
_20 = core::ptr::addr_of!((*_20));
(*_20) = _7;
_81 = core::ptr::addr_of!(_16);
(*_33) = [_24,_24,_45,_24];
(*_33) = [_24,_24,_76,_45];
_5 = _9;
(*_81) = _11;
match _36 {
0 => bb6,
1 => bb5,
2 => bb31,
3 => bb35,
4 => bb36,
34302 => bb38,
_ => bb37
}
}
bb35 = {
_5 = [7138731866141487837_i64,8492913491718002125_i64];
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = [8200476109087570047_i64,4284863361196384322_i64];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{bf737}';
(*_2) = _5;
_10 = [60_i8,50_i8,51_i8,(-11_i8),35_i8,(-28_i8),(-113_i8)];
(*_2) = _5;
_5 = [(-3384890550859648944_i64),(-5416193739146512500_i64)];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{38c70}';
(*_2) = _5;
(*_2) = _3;
(*_2) = [(-2211447276039678829_i64),1292431043112812808_i64];
_13 = (-9189968066149708722168362453401091444_i128) as i64;
(*_2) = [_13,_13];
_4 = Adt22::Variant1 { fld0: Move((*_6)),fld1: 17890368280127413047_usize };
_7 = (-77592234565992414686661379477397402800_i128) as f64;
(*_2) = [_13,_13];
place!(Field::<usize>(Variant(_4, 1), 1)) = 2_usize;
(*_2) = [_13,_13];
_9 = _3;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 1), 1)));
(*_1) = 1_usize - 7208217351960610096_usize;
(*_1) = 4_usize << _11;
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
Call((*_2) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb36 = {
_15 = 842596135_u32 as f32;
(*_20) = 76536340336794707449300859254424323469_i128 as f64;
_10 = [44_i8,(-22_i8),53_i8,54_i8,1_i8,(-89_i8),(-11_i8)];
_3 = (*_8);
Goto(bb8)
}
bb37 = {
(*_8) = _3;
(*_8) = [_13,_13];
_29 = (-54_i8) as i32;
(*_8) = _3;
(*_8) = [_13,_13];
_27 = 51930_u16 as isize;
(*_20) = 4324676497685925390_u64 as f64;
(*_20) = 2645830332_u32 as f64;
_3 = [_13,_13];
(*_20) = 2387259616_u32 as f64;
_9 = [_13,_13];
(*_20) = 162146335875381632006766975977095899766_u128 as f64;
Goto(bb9)
}
bb38 = {
(*_33) = [_76,_45,_76,_45];
(*_20) = _7;
_38 = [(-71_i8),46_i8,(-20_i8),(-84_i8),(-79_i8),67_i8,(-25_i8)];
(*_81) = _11 ^ _11;
_66 = core::ptr::addr_of!(_33);
_14 = _18 + _15;
(*_81) = _50 as u8;
_39 = !_29;
_55 = !_67;
_9 = [_13,_13];
_2 = core::ptr::addr_of_mut!(_3);
(*_81) = _11 - _11;
(*_2) = (*_8);
_77.0 = [_13,_13];
match _36 {
0 => bb39,
1 => bb40,
2 => bb41,
3 => bb42,
4 => bb43,
5 => bb44,
6 => bb45,
34302 => bb47,
_ => bb46
}
}
bb39 = {
_7 = -_35;
_8 = core::ptr::addr_of_mut!(_5);
(*_8) = _3;
_43 = core::ptr::addr_of!(_33);
_33 = core::ptr::addr_of!(_26);
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
(*_33) = [_24,_45,_24,_45];
(*_43) = core::ptr::addr_of!((*_33));
(*_8) = [_13,_13];
(*_8) = [_13,_13];
_45 = _15 as u64;
place!(Field::<isize>(Variant(_4, 0), 0)) = _28 << _52;
_37 = 3536549884_u32 as isize;
(*_43) = core::ptr::addr_of!((*_33));
_3 = (*_8);
(*_8) = [_13,_13];
_38 = [92_i8,(-58_i8),15_i8,(-116_i8),66_i8,62_i8,122_i8];
(*_8) = _3;
_19 = Field::<isize>(Variant(_4, 0), 0);
_14 = _15;
(*_33) = [_45,_45,_45,_45];
_37 = _18 as isize;
Goto(bb29)
}
bb40 = {
_23 = '\u{736aa}';
_19 = (-2_isize);
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
_8 = core::ptr::addr_of_mut!((*_2));
_4 = Adt22::Variant0 { fld0: _19,fld1: _23 };
(*_8) = [_13,_13];
Goto(bb5)
}
bb41 = {
_15 = 842596135_u32 as f32;
(*_20) = 76536340336794707449300859254424323469_i128 as f64;
_10 = [44_i8,(-22_i8),53_i8,54_i8,1_i8,(-89_i8),(-11_i8)];
_3 = (*_8);
Goto(bb8)
}
bb42 = {
_5 = [7138731866141487837_i64,8492913491718002125_i64];
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = [8200476109087570047_i64,4284863361196384322_i64];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{bf737}';
(*_2) = _5;
_10 = [60_i8,50_i8,51_i8,(-11_i8),35_i8,(-28_i8),(-113_i8)];
(*_2) = _5;
_5 = [(-3384890550859648944_i64),(-5416193739146512500_i64)];
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{38c70}';
(*_2) = _5;
(*_2) = _3;
(*_2) = [(-2211447276039678829_i64),1292431043112812808_i64];
_13 = (-9189968066149708722168362453401091444_i128) as i64;
(*_2) = [_13,_13];
_4 = Adt22::Variant1 { fld0: Move((*_6)),fld1: 17890368280127413047_usize };
_7 = (-77592234565992414686661379477397402800_i128) as f64;
(*_2) = [_13,_13];
place!(Field::<usize>(Variant(_4, 1), 1)) = 2_usize;
(*_2) = [_13,_13];
_9 = _3;
(*_6) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 1), 1)));
(*_1) = 1_usize - 7208217351960610096_usize;
(*_1) = 4_usize << _11;
(*_6) = core::ptr::addr_of!((*_1));
(*_6) = core::ptr::addr_of!((*_1));
Call((*_2) = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb43 = {
(*_8) = [_13,_13];
(*_2) = _3;
(*_2) = _5;
(*_2) = [_13,_13];
(*_2) = [_13,_13];
(*_2) = [_13,_13];
_16 = _11;
_9 = _3;
(*_2) = _5;
_8 = core::ptr::addr_of_mut!((*_2));
_15 = _11 as f32;
place!(Field::<isize>(Variant(_4, 0), 0)) = _19;
_2 = Move(_8);
_12 = _18 * _15;
_11 = _16 + _16;
Goto(bb6)
}
bb44 = {
(*_20) = _35;
_40 = !89092564442775082845482504404799594141_u128;
_16 = _11 >> Field::<isize>(Variant(_4, 0), 0);
(*_33) = [13322717959965782178_u64,12369615381173901799_u64,2619581271722034173_u64,9782842609023491939_u64];
_27 = -_28;
(*_20) = _35 + _35;
(*_20) = -_35;
(*_8) = _5;
(*_33) = [13529508579090698636_u64,10529890051174512086_u64,736037540829854495_u64,10096162647343957568_u64];
(*_20) = _35 - _35;
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
(*_33) = [999155666558370258_u64,10403137715630918246_u64,2448296148896033817_u64,17391235699707954644_u64];
(*_33) = [46890045048773967_u64,8864041400298804731_u64,17409410069884289_u64,12681434463123615049_u64];
_20 = core::ptr::addr_of!((*_20));
(*_20) = -_35;
_32 = [_28,_28,_28,_19,_19,Field::<isize>(Variant(_4, 0), 0),_31];
_2 = core::ptr::addr_of_mut!((*_8));
(*_8) = [_13,_13];
(*_33) = [15696636520848409901_u64,11898099858112413789_u64,6395901165230102995_u64,9770166910498768107_u64];
(*_33) = [12746628457555305783_u64,13464092363152304358_u64,15209124432262486799_u64,4638806257402883855_u64];
_41 = &mut _40;
(*_20) = _35 + _35;
(*_20) = 900001141_u32 as f64;
_10 = [(-111_i8),(-9_i8),19_i8,17_i8,84_i8,(-77_i8),(-125_i8)];
(*_8) = [_13,_13];
(*_8) = _3;
(*_20) = _35 - _35;
match Field::<isize>(Variant(_4, 0), 0) {
0 => bb10,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768211454 => bb17,
_ => bb16
}
}
bb45 = {
_45 = !_24;
(*_41) = !76492395889125080733761932114301185841_u128;
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
_2 = Move(_8);
(*_20) = _35;
_30 = -_29;
_27 = _30 as isize;
(*_41) = !336323831120520602582107498322220057751_u128;
(*_33) = [_24,_45,_24,_45];
(*_33) = [_24,_24,_24,_24];
(*_41) = Field::<char>(Variant(_4, 0), 1) as u128;
RET = [_23,_23,Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1),Field::<char>(Variant(_4, 0), 1)];
_32 = [_28,_37,_27,_27,_27,_27,_27];
(*_20) = (*_41) as f64;
_30 = _52 | _17;
Goto(bb26)
}
bb46 = {
_17 = _29 & _29;
(*_33) = [_45,_45,_45,_45];
_62 = Field::<isize>(Variant(_4, 0), 0) - _19;
_6 = core::ptr::addr_of_mut!((*_53));
_63 = Move(_8);
place!(Field::<char>(Variant(_4, 0), 1)) = _23;
_69 = _58;
(*_20) = _35;
_30 = _52;
(*_20) = -_35;
(*_33) = [_45,_45,_45,_45];
(*_43) = core::ptr::addr_of!((*_33));
(*_20) = _11 as f64;
_35 = (*_20);
(*_43) = core::ptr::addr_of!((*_33));
(*_43) = core::ptr::addr_of!((*_33));
Goto(bb31)
}
bb47 = {
_30 = _39 - _17;
(*_20) = 47_i8 as f64;
(*_8) = (*_2);
_80 = _14;
_54 = core::ptr::addr_of_mut!((*_53));
(*_53) = core::ptr::addr_of!(_85.1);
(*_8) = [_13,_13];
(*_53) = core::ptr::addr_of!((*_1));
(*_8) = [_13,_13];
_81 = core::ptr::addr_of!((*_81));
(*_2) = [_13,_13];
_35 = _7;
Goto(bb48)
}
bb48 = {
(*_33) = [_45,_76,_24,_45];
(*_53) = core::ptr::addr_of!((*_1));
(*_33) = [_76,_24,_24,_45];
_4 = Adt22::Variant0 { fld0: _28,fld1: _58 };
(*_2) = [_13,_13];
(*_53) = core::ptr::addr_of!((*_1));
(*_2) = [_13,_13];
(*_33) = [_76,_45,_76,_45];
_6 = core::ptr::addr_of_mut!((*_53));
_82 = core::ptr::addr_of!((*_20));
(*_1) = (*_20) as usize;
(*_1) = !2762682002641222936_usize;
_58 = _23;
(*_81) = _11;
(*_81) = !_11;
(*_81) = _7 as u8;
(*_1) = 6404411232879064068_usize >> _31;
(*_53) = core::ptr::addr_of!((*_1));
_94 = 105_i8 & (-107_i8);
_69 = _23;
match _36 {
0 => bb42,
1 => bb2,
2 => bb45,
3 => bb4,
4 => bb17,
5 => bb14,
6 => bb20,
34302 => bb49,
_ => bb22
}
}
bb49 = {
(*_53) = core::ptr::addr_of!((*_1));
(*_2) = [_13,_13];
(*_2) = (*_8);
(*_81) = _11;
(*_81) = _24 as u8;
_3 = (*_8);
_83 = _15;
_75 = !_50;
(*_66) = core::ptr::addr_of!((*_33));
_47 = core::ptr::addr_of_mut!((*_53));
Goto(bb50)
}
bb50 = {
Call(_95 = dump_var(Move(_23), Move(_39), Move(_29), Move(_55)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_95 = dump_var(Move(_22), Move(_52), Move(_36), Move(_37)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_95 = dump_var(Move(_9), Move(_24), Move(_27), Move(_28)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_95 = dump_var(Move(_38), Move(_32), Move(_16), Move(_3)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_95 = dump_var(Move(_69), _96, _96, _96), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *const [u64; 4],mut _2: *const usize,mut _3: &'static mut u128,mut _4: *const f64,mut _5: char,mut _6: f32,mut _7: u128,mut _8: *const *const [u64; 4],mut _9: *mut *const usize,mut _10: *mut [i64; 2]) -> *const *const [u64; 4] {
mir! {
type RET = *const *const [u64; 4];
let _11: &'static u32;
let _12: [u64; 4];
let _13: isize;
let _14: [u128; 5];
let _15: (u16, i32, char, u128);
let _16: i128;
let _17: Adt28;
let _18: (((u8, usize, isize), &'static mut u128), u8);
let _19: f32;
let _20: [u128; 5];
let _21: bool;
let _22: u128;
let _23: [i8; 7];
let _24: bool;
let _25: Adt59;
let _26: f32;
let _27: *mut [i64; 2];
let _28: bool;
let _29: &'static Adt25;
let _30: f32;
let _31: i16;
let _32: f64;
let _33: i32;
let _34: &'static mut &'static mut (i128, &'static mut u128);
let _35: u32;
let _36: Adt28;
let _37: u64;
let _38: isize;
let _39: Adt54;
let _40: bool;
let _41: &'static mut u128;
let _42: [u16; 4];
let _43: [u128; 5];
let _44: *mut [i64; 2];
let _45: &'static mut &'static mut (i128, &'static mut u128);
let _46: isize;
let _47: u8;
let _48: &'static mut u128;
let _49: f64;
let _50: *mut *const usize;
let _51: [i32; 7];
let _52: *mut [i64; 2];
let _53: bool;
let _54: char;
let _55: &'static &'static Adt25;
let _56: bool;
let _57: f32;
let _58: f32;
let _59: f32;
let _60: u32;
let _61: i64;
let _62: char;
let _63: *const (i32, *const usize);
let _64: &'static &'static Adt25;
let _65: &'static u32;
let _66: isize;
let _67: (i128, &'static mut u128);
let _68: [i128; 4];
let _69: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _70: isize;
let _71: [u8; 4];
let _72: i32;
let _73: *mut *const usize;
let _74: u8;
let _75: Adt54;
let _76: [u32; 3];
let _77: bool;
let _78: isize;
let _79: &'static &'static u32;
let _80: u8;
let _81: Adt48;
let _82: u8;
let _83: [u128; 6];
let _84: isize;
let _85: isize;
let _86: &'static u32;
let _87: *const u8;
let _88: &'static &'static u32;
let _89: f64;
let _90: i8;
let _91: (i16, Adt59, &'static u32);
let _92: isize;
let _93: &'static mut &'static mut (i128, &'static mut u128);
let _94: u128;
let _95: *mut (&'static &'static u32, [u128; 6], char);
let _96: u32;
let _97: *const u8;
let _98: u16;
let _99: [u128; 6];
let _100: bool;
let _101: *const *const [u64; 4];
let _102: bool;
let _103: &'static mut &'static mut (i128, &'static mut u128);
let _104: isize;
let _105: isize;
let _106: u128;
let _107: u8;
let _108: &'static mut &'static mut (i128, &'static mut u128);
let _109: char;
let _110: bool;
let _111: Adt22;
let _112: *const &'static mut (i128, &'static mut u128);
let _113: *const u8;
let _114: u16;
let _115: Adt73;
let _116: &'static u32;
let _117: (u64, ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22), *mut (&'static &'static u32, [u128; 6], char));
let _118: *const (i32, *const usize);
let _119: f64;
let _120: i128;
let _121: &'static mut (i32, *const usize);
let _122: isize;
let _123: char;
let _124: &'static &'static u32;
let _125: [i16; 3];
let _126: isize;
let _127: &'static &'static Adt25;
let _128: usize;
let _129: u8;
let _130: [u128; 4];
let _131: i16;
let _132: &'static u32;
let _133: (u16, i32, char, u128);
let _134: *const &'static mut (i32, *const usize);
let _135: isize;
let _136: [u32; 5];
let _137: [i16; 4];
let _138: u128;
let _139: *const u8;
let _140: isize;
let _141: &'static Adt25;
let _142: isize;
let _143: u128;
let _144: isize;
let _145: bool;
let _146: [u32; 5];
let _147: f32;
let _148: isize;
let _149: &'static mut (i32, *const usize);
let _150: &'static mut i32;
let _151: bool;
let _152: isize;
let _153: [u64; 4];
let _154: char;
let _155: i8;
let _156: char;
let _157: i32;
let _158: f64;
let _159: f32;
let _160: &'static Adt25;
let _161: char;
let _162: i64;
let _163: f32;
let _164: *mut (&'static &'static u32, [u128; 6], char);
let _165: usize;
let _166: f64;
let _167: i128;
let _168: u128;
let _169: Adt38;
let _170: isize;
let _171: ();
let _172: ();
{
_9 = core::ptr::addr_of_mut!(_2);
_6 = 4131823412_u32 as f32;
_9 = core::ptr::addr_of_mut!(_2);
RET = core::ptr::addr_of!(_1);
_6 = 9223372036854775807_isize as f32;
_5 = '\u{680f8}';
_8 = Move(RET);
_7 = !322275516232850624869288213839748362749_u128;
_6 = (-45319945058017752219453800714673362024_i128) as f32;
_8 = core::ptr::addr_of!(_1);
_8 = core::ptr::addr_of!((*_8));
RET = core::ptr::addr_of!((*_8));
_6 = 224_u8 as f32;
RET = core::ptr::addr_of!((*_8));
(*_8) = core::ptr::addr_of!(_12);
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [11756394199293469322_u64,14880832746721168624_u64,2211039602579423890_u64,17719352957810226463_u64];
(*_1) = [1491208635093593114_u64,10699785178454138296_u64,16687877687685728653_u64,9090491711331858114_u64];
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
_14 = [_7,_7,_7,_7,_7];
_8 = core::ptr::addr_of!((*_8));
Goto(bb1)
}
bb1 = {
_8 = Move(RET);
_16 = 131668027761065884349655077423270633981_i128 >> _7;
(*_1) = [3234780354176104662_u64,5623540069258618539_u64,18363498713656820985_u64,5914102441011516738_u64];
_15.3 = _7 - _7;
(*_1) = [5102672399693413385_u64,11364386532850170189_u64,11008862092304939231_u64,16501124464905789426_u64];
_15.0 = !23894_u16;
(*_1) = [3442775437923707038_u64,9716944839008866680_u64,3374198821967240459_u64,9029584958961166207_u64];
_8 = core::ptr::addr_of!(_1);
_3 = &mut _7;
(*_3) = _15.3 >> _15.3;
(*_9) = core::ptr::addr_of!(_18.0.0.1);
Goto(bb2)
}
bb2 = {
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0 = (97_u8, 6868310142457563255_usize, 15_isize);
_3 = &mut _15.3;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [2987588233136186588_u64,7433649481946269513_u64,15340672620100756891_u64,2293528190624031202_u64];
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_19 = _6;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 7_usize;
Call((*_2) = core::intrinsics::bswap(15366107367143874554_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_2) = 2572905439_u32 as usize;
_25.fld3 = ((-1815949975_i32), Move((*_9)));
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_20 = [(*_3),(*_3),(*_3),(*_3),(*_3)];
Goto(bb4)
}
bb4 = {
(*_8) = core::ptr::addr_of!((*_1));
(*_3) = 119133234043300768411651887082362038908_u128;
(*_1) = [17781012850012831973_u64,14233400261718115437_u64,2897451465850298612_u64,17078108585383989827_u64];
(*_2) = !4153349304394369777_usize;
(*_9) = core::ptr::addr_of!((*_2));
_14 = [(*_3),(*_3),(*_3),(*_3),(*_3)];
_28 = (*_3) != (*_3);
(*_9) = core::ptr::addr_of!((*_2));
(*_3) = 175460940930723203673339951318285914245_u128 & 205574112600911637962752765541767348319_u128;
(*_1) = [1018370962618435342_u64,14317413095387908422_u64,6470050384068057180_u64,500856332508954404_u64];
Call(_18.1 = fn18(Move((*_9)), Move((*_8)), Move(_25.fld3), (*_1), (*_3), Move(_10), (*_1), Move(_3), Move(_8), Move(_4), Move(_9)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = _18.0.0.2 << _18.1;
_25.fld4 = 54313_u16 - 18450_u16;
_22 = !67930094717071754583911438247385535121_u128;
_14 = [_22,_22,_22,_22,_22];
_37 = !17310858414401347479_u64;
_25.fld1.3 = _22 * _22;
_25.fld1.2 = _5;
_2 = core::ptr::addr_of!(_18.0.0.1);
_35 = !3642493095_u32;
_25.fld1.1 = (-1206827933_i32) >> _18.1;
(*_2) = _25.fld1.3 as usize;
(*_2) = _25.fld1.3 as usize;
_18.0.1 = &mut _25.fld1.3;
_38 = _13 - _18.0.0.2;
_31 = _28 as i16;
_18.0.0 = (_18.1, 6087046900312169704_usize, _38);
_11 = &_35;
(*_2) = (-570027312_i32) as usize;
_2 = core::ptr::addr_of!((*_2));
_8 = core::ptr::addr_of!(_1);
_2 = core::ptr::addr_of!((*_2));
_31 = 88_i8 as i16;
Goto(bb6)
}
bb6 = {
_14 = [_22,_22,_22,_22,_22];
_9 = core::ptr::addr_of_mut!(_2);
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0.2 = _16 as isize;
(*_8) = core::ptr::addr_of!(_12);
_21 = _28;
_21 = (*_11) > (*_11);
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 1482078831_i32 as usize;
(*_2) = !3_usize;
(*_1) = [_37,_37,_37,_37];
_13 = _38 >> (*_11);
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_28 = _21;
Goto(bb7)
}
bb7 = {
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = !3286440496194051196_usize;
_6 = (*_11) as f32;
_18.0.0.0 = _18.1;
_20 = _14;
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb8)
}
bb8 = {
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
(*_1) = [_37,_37,_37,_37];
_18.0.0.1 = 1_usize * 12011461152280640345_usize;
(*_8) = core::ptr::addr_of!((*_1));
_47 = _18.0.0.0;
_32 = (-69_i8) as f64;
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
_38 = _13;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb9)
}
bb9 = {
(*_1) = [_37,_37,_37,_37];
(*_9) = core::ptr::addr_of!(_18.0.0.1);
(*_2) = 3_usize >> _47;
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_30 = (-1823211688_i32) as f32;
Goto(bb10)
}
bb10 = {
(*_2) = !5648056736658945604_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb11)
}
bb11 = {
_37 = 17186063856101712664_u64 - 12677473097411224412_u64;
_46 = _38;
(*_2) = (-1494421606_i32) as usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = !2077431363555088139_usize;
(*_1) = [_37,_37,_37,_37];
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = (-91_i8) as usize;
_47 = !_18.0.0.0;
(*_1) = [_37,_37,_37,_37];
(*_2) = 17597011747850788268_usize + 8349987729497840900_usize;
Goto(bb12)
}
bb12 = {
_53 = _21;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = !4292790559968350797_usize;
_42 = [52973_u16,60533_u16,55877_u16,46989_u16];
(*_2) = 2_usize >> _13;
_24 = !_28;
(*_9) = core::ptr::addr_of!((*_2));
(*_1) = [_37,_37,_37,_37];
(*_2) = _19 as usize;
RET = core::ptr::addr_of!((*_8));
(*RET) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb13)
}
bb13 = {
(*_8) = core::ptr::addr_of!((*_1));
_26 = -_19;
_18.0.0.1 = 11771070558233059645_usize;
(*_2) = !3_usize;
(*_1) = [_37,_37,_37,_37];
_23 = [12_i8,(-80_i8),78_i8,(-63_i8),(-67_i8),42_i8,0_i8];
_32 = _16 as f64;
(*_1) = [_37,_37,_37,_37];
_46 = _13;
_4 = core::ptr::addr_of!(_49);
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
_14 = [_22,_22,_22,_22,_22];
(*_1) = [_37,_37,_37,_37];
_35 = 7222994_u32 * 2694849223_u32;
(*_1) = [_37,_37,_37,_37];
Goto(bb14)
}
bb14 = {
(*_4) = _31 as f64;
_13 = -_46;
(*_4) = _22 as f64;
_54 = _5;
_20 = _14;
_43 = [_22,_22,_22,_22,_22];
Goto(bb15)
}
bb15 = {
(*_9) = core::ptr::addr_of!((*_2));
_54 = _5;
(*_2) = 8856145510149964686_usize >> _18.1;
_18.0.0.2 = _13 >> (*_2);
(*_2) = 5_usize & 2659170027772848947_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_4) = -_32;
(*_2) = 4992213655681271844_usize;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
_58 = _30;
(*_1) = [_37,_37,_37,_37];
(*_2) = !1_usize;
_1 = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 5_usize;
Call((*_2) = core::intrinsics::transmute(_46), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_68 = [_16,_16,_16,_16];
_56 = _24 ^ _28;
(*_8) = core::ptr::addr_of!((*_1));
_59 = _37 as f32;
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
Goto(bb17)
}
bb17 = {
_69.1.1.1 = Move((*_9));
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_60 = _35 & _35;
_66 = _46 & _13;
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_4) = _32;
_18.0.0 = (_47, 5_usize, _66);
_51 = [(-739724726_i32),(-148232454_i32),521717763_i32,849924487_i32,1476459367_i32,(-247607807_i32),1667933011_i32];
(*_2) = 6_usize >> _46;
(*_1) = [_37,_37,_37,_37];
(*_4) = _32 * _32;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 6_usize ^ 3_usize;
_74 = !_18.1;
(*_2) = 6_usize & 12619876330843191012_usize;
_73 = core::ptr::addr_of_mut!((*_9));
(*_8) = core::ptr::addr_of!((*_1));
_69.1.2 = [111_i8,(-78_i8),(-61_i8),(-47_i8),(-120_i8),59_i8,26_i8];
(*_73) = core::ptr::addr_of!((*_2));
Goto(bb18)
}
bb18 = {
_23 = [95_i8,91_i8,113_i8,41_i8,(-14_i8),(-127_i8),90_i8];
_30 = _19 + _19;
_74 = _22 as u8;
(*_2) = 10692827212796756441_usize;
_1 = core::ptr::addr_of!((*_1));
_70 = _37 as isize;
_11 = &_60;
(*_4) = _32 + _32;
(*_4) = -_32;
(*_1) = [_37,_37,_37,_37];
(*_4) = (*_2) as f64;
(*_4) = _32;
(*_9) = core::ptr::addr_of!((*_2));
_18.0.1 = &mut _22;
(*_4) = _31 as f64;
_56 = !_28;
_75 = Adt54::Variant0 { fld0: _68,fld1: _37,fld2: 293013388_i32 };
_77 = !_56;
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb19)
}
bb19 = {
_67.1 = Move(_18.0.1);
match (*_2) {
0 => bb9,
1 => bb16,
2 => bb20,
3 => bb21,
4 => bb22,
10692827212796756441 => bb24,
_ => bb23
}
}
bb20 = {
_23 = [95_i8,91_i8,113_i8,41_i8,(-14_i8),(-127_i8),90_i8];
_30 = _19 + _19;
_74 = _22 as u8;
(*_2) = 10692827212796756441_usize;
_1 = core::ptr::addr_of!((*_1));
_70 = _37 as isize;
_11 = &_60;
(*_4) = _32 + _32;
(*_4) = -_32;
(*_1) = [_37,_37,_37,_37];
(*_4) = (*_2) as f64;
(*_4) = _32;
(*_9) = core::ptr::addr_of!((*_2));
_18.0.1 = &mut _22;
(*_4) = _31 as f64;
_56 = !_28;
_75 = Adt54::Variant0 { fld0: _68,fld1: _37,fld2: 293013388_i32 };
_77 = !_56;
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb19)
}
bb21 = {
_69.1.1.1 = Move((*_9));
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_60 = _35 & _35;
_66 = _46 & _13;
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_4) = _32;
_18.0.0 = (_47, 5_usize, _66);
_51 = [(-739724726_i32),(-148232454_i32),521717763_i32,849924487_i32,1476459367_i32,(-247607807_i32),1667933011_i32];
(*_2) = 6_usize >> _46;
(*_1) = [_37,_37,_37,_37];
(*_4) = _32 * _32;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 6_usize ^ 3_usize;
_74 = !_18.1;
(*_2) = 6_usize & 12619876330843191012_usize;
_73 = core::ptr::addr_of_mut!((*_9));
(*_8) = core::ptr::addr_of!((*_1));
_69.1.2 = [111_i8,(-78_i8),(-61_i8),(-47_i8),(-120_i8),59_i8,26_i8];
(*_73) = core::ptr::addr_of!((*_2));
Goto(bb18)
}
bb22 = {
(*_2) = !5648056736658945604_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb11)
}
bb23 = {
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0 = (97_u8, 6868310142457563255_usize, 15_isize);
_3 = &mut _15.3;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [2987588233136186588_u64,7433649481946269513_u64,15340672620100756891_u64,2293528190624031202_u64];
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_19 = _6;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 7_usize;
Call((*_2) = core::intrinsics::bswap(15366107367143874554_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb24 = {
_62 = _5;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),_37];
(*_2) = 9292741780168803548_usize >> _38;
Goto(bb25)
}
bb25 = {
_69.1.2 = [49_i8,(-27_i8),80_i8,(-91_i8),74_i8,(-23_i8),(-19_i8)];
_18.1 = _47 + _18.0.0.0;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_78 = _13 + _38;
_54 = _5;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
place!(Field::<i32>(Variant(_75, 0), 2)) = -1152183671_i32;
_8 = Move(RET);
_72 = Field::<i32>(Variant(_75, 0), 2) & Field::<i32>(Variant(_75, 0), 2);
_65 = Move(_11);
(*_9) = core::ptr::addr_of!((*_2));
_28 = _56;
(*_1) = [Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),_37];
(*_2) = 3_usize;
(*_9) = core::ptr::addr_of!(_18.0.0.1);
match (*_2) {
3 => bb27,
_ => bb26
}
}
bb26 = {
(*_9) = core::ptr::addr_of!((*_2));
_54 = _5;
(*_2) = 8856145510149964686_usize >> _18.1;
_18.0.0.2 = _13 >> (*_2);
(*_2) = 5_usize & 2659170027772848947_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_4) = -_32;
(*_2) = 4992213655681271844_usize;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
_58 = _30;
(*_1) = [_37,_37,_37,_37];
(*_2) = !1_usize;
_1 = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 5_usize;
Call((*_2) = core::intrinsics::transmute(_46), ReturnTo(bb16), UnwindUnreachable())
}
bb27 = {
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = 6_usize;
_61 = !(-6154519000583895302_i64);
(*_2) = 7_usize - 13755639484738868346_usize;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_4) = _31 as f64;
(*_1) = [Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_19 = _30 + _59;
_80 = _47;
_4 = core::ptr::addr_of!((*_4));
Call(_33 = core::intrinsics::transmute(_60), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_23 = _69.1.2;
_1 = core::ptr::addr_of!((*_1));
_47 = _18.1;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
Goto(bb29)
}
bb29 = {
(*_1) = [_37,_37,_37,_37];
_67.0 = _16;
_6 = _59;
_59 = _61 as f32;
(*_9) = core::ptr::addr_of!((*_2));
_11 = &_60;
(*_4) = _32 * _32;
_40 = _28;
_9 = core::ptr::addr_of_mut!((*_9));
(*_2) = 0_usize;
_6 = _19;
(*_2) = !12563575633195237568_usize;
_62 = _54;
_66 = _38 | _78;
_28 = _78 > _18.0.0.2;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_2) = 5_usize - 7706615384262409171_usize;
(*_4) = _32 + _32;
_50 = core::ptr::addr_of_mut!((*_9));
(*_4) = -_32;
place!(Field::<i32>(Variant(_75, 0), 2)) = _33 & _33;
_83 = [267674562847476955549753872407863431145_u128,2379348603649307551865531756219779707_u128,184218914033163877995736990210551924419_u128,88313463333061237855524658813526666382_u128,28901571650490468360256310780951908286_u128,309549240907632722523524251564470151147_u128];
_43 = [202438408875998602328832416670367470587_u128,314052944973778762724752394998360679002_u128,22516703808786855170885031179485566347_u128,155017111170389520386204130155333116273_u128,259390521135580803154039333677246763172_u128];
(*_2) = !3_usize;
Goto(bb30)
}
bb30 = {
_59 = _31 as f32;
(*_50) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb31)
}
bb31 = {
(*_2) = 5495525255572908170_usize & 3_usize;
_4 = core::ptr::addr_of!((*_4));
(*_2) = !11875522457210214146_usize;
_16 = _67.0;
(*_4) = _32 - _32;
_69.1.1.2 = &mut _33;
Goto(bb32)
}
bb32 = {
_76 = [(*_11),(*_11),(*_11)];
_69.1.3 = Adt22::Variant1 { fld0: Move(_69.1.1.1),fld1: (*_2) };
_9 = core::ptr::addr_of_mut!((*_9));
_4 = core::ptr::addr_of!(_89);
_80 = !_47;
_65 = &(*_11);
(*_9) = core::ptr::addr_of!((*_2));
_91.1.fld4 = 13182_u16 - 20419_u16;
_63 = core::ptr::addr_of!(_91.1.fld3);
(*_2) = Field::<usize>(Variant(_69.1.3, 1), 1);
(*_63) = (_72, Move(Field::<*const usize>(Variant(_69.1.3, 1), 0)));
(*_63).1 = core::ptr::addr_of!((*_2));
_70 = _18.0.0.2;
(*_4) = _49 + _49;
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) & _72;
_91.1.fld6 = _43;
_91.1.fld6 = _43;
_91.1.fld5 = [_91.1.fld4,_91.1.fld4,_91.1.fld4,_91.1.fld4];
_60 = _35 << (*_63).0;
_90 = 12_i8;
_80 = !_47;
_59 = _58 - _30;
(*_2) = !Field::<usize>(Variant(_69.1.3, 1), 1);
(*_9) = Move((*_63).1);
_69.1.0 = [_54,_62,_5,_5,_62,_5,_62];
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) * _72;
(*_63) = (_72, Move((*_9)));
_73 = core::ptr::addr_of_mut!((*_63).1);
match _90 {
0 => bb16,
1 => bb20,
2 => bb10,
12 => bb33,
_ => bb4
}
}
bb33 = {
(*_63).0 = 47896714913127495360462521299828749007_u128 as i32;
_60 = _35 << _78;
_68 = [_67.0,_67.0,_16,_16];
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_9) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_69.1.3, 1), 1)));
_91.1.fld1 = (_91.1.fld4, (*_63).0, _5, 318032508709213973593432838957477110684_u128);
_43 = [_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3];
_66 = (*_2) as isize;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_91.1.fld1.2 = _62;
(*_2) = _18.0.0.1 * _18.0.0.1;
(*_73) = core::ptr::addr_of!((*_2));
_36 = Adt28::Variant1 { fld0: _60,fld1: _5,fld2: _91.1.fld1 };
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) + Field::<i32>(Variant(_75, 0), 2);
(*_73) = core::ptr::addr_of!((*_2));
_37 = Field::<u64>(Variant(_75, 0), 1) | Field::<u64>(Variant(_75, 0), 1);
_69.1.3 = Adt22::Variant0 { fld0: _78,fld1: _5 };
(*_73) = Move((*_9));
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_84 = _18.0.0.2 ^ _18.0.0.2;
_76 = [Field::<u32>(Variant(_36, 1), 0),Field::<u32>(Variant(_36, 1), 0),_35];
_20 = [Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,_91.1.fld1.3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3];
_59 = _58;
_69.1.1.0 = Adt38::Variant2 { fld0: _83,fld1: (*_4),fld2: _91.1.fld1.0,fld3: Move((*_63).1),fld4: _60 };
(*_63).1 = Move((*_9));
_69.1.2 = _23;
(*_4) = _90 as f64;
Call(_72 = core::intrinsics::bswap((*_63).0), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_16 = _91.1.fld4 as i128;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_82 = Field::<u32>(Variant(_36, 1), 0) as u8;
(*_63) = (_72, Move(Field::<*const usize>(Variant(_69.1.1.0, 2), 3)));
_50 = core::ptr::addr_of_mut!(_69.1.1.1);
place!(Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2)) = _91.1.fld1;
(*_4) = _49 * Field::<f64>(Variant(_69.1.1.0, 2), 1);
match Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3 {
0 => bb5,
1 => bb23,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
6 => bb39,
318032508709213973593432838957477110684 => bb41,
_ => bb40
}
}
bb35 = {
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0 = (97_u8, 6868310142457563255_usize, 15_isize);
_3 = &mut _15.3;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [2987588233136186588_u64,7433649481946269513_u64,15340672620100756891_u64,2293528190624031202_u64];
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_19 = _6;
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 7_usize;
Call((*_2) = core::intrinsics::bswap(15366107367143874554_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb36 = {
(*_2) = !5648056736658945604_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb11)
}
bb37 = {
(*_2) = 5495525255572908170_usize & 3_usize;
_4 = core::ptr::addr_of!((*_4));
(*_2) = !11875522457210214146_usize;
_16 = _67.0;
(*_4) = _32 - _32;
_69.1.1.2 = &mut _33;
Goto(bb32)
}
bb38 = {
_14 = [_22,_22,_22,_22,_22];
_9 = core::ptr::addr_of_mut!(_2);
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0.2 = _16 as isize;
(*_8) = core::ptr::addr_of!(_12);
_21 = _28;
_21 = (*_11) > (*_11);
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 1482078831_i32 as usize;
(*_2) = !3_usize;
(*_1) = [_37,_37,_37,_37];
_13 = _38 >> (*_11);
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_28 = _21;
Goto(bb7)
}
bb39 = {
(*_1) = [_37,_37,_37,_37];
_67.0 = _16;
_6 = _59;
_59 = _61 as f32;
(*_9) = core::ptr::addr_of!((*_2));
_11 = &_60;
(*_4) = _32 * _32;
_40 = _28;
_9 = core::ptr::addr_of_mut!((*_9));
(*_2) = 0_usize;
_6 = _19;
(*_2) = !12563575633195237568_usize;
_62 = _54;
_66 = _38 | _78;
_28 = _78 > _18.0.0.2;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_2) = 5_usize - 7706615384262409171_usize;
(*_4) = _32 + _32;
_50 = core::ptr::addr_of_mut!((*_9));
(*_4) = -_32;
place!(Field::<i32>(Variant(_75, 0), 2)) = _33 & _33;
_83 = [267674562847476955549753872407863431145_u128,2379348603649307551865531756219779707_u128,184218914033163877995736990210551924419_u128,88313463333061237855524658813526666382_u128,28901571650490468360256310780951908286_u128,309549240907632722523524251564470151147_u128];
_43 = [202438408875998602328832416670367470587_u128,314052944973778762724752394998360679002_u128,22516703808786855170885031179485566347_u128,155017111170389520386204130155333116273_u128,259390521135580803154039333677246763172_u128];
(*_2) = !3_usize;
Goto(bb30)
}
bb40 = {
(*_1) = [_37,_37,_37,_37];
(*_9) = core::ptr::addr_of!(_18.0.0.1);
(*_2) = 3_usize >> _47;
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_30 = (-1823211688_i32) as f32;
Goto(bb10)
}
bb41 = {
(*_50) = Move((*_63).1);
(*_63) = (Field::<i32>(Variant(_75, 0), 2), Move((*_50)));
_91.0 = _31 | _31;
(*_63).1 = core::ptr::addr_of!(_18.0.0.1);
match _91.1.fld1.3 {
0 => bb42,
318032508709213973593432838957477110684 => bb44,
_ => bb43
}
}
bb42 = {
_14 = [_22,_22,_22,_22,_22];
_9 = core::ptr::addr_of_mut!(_2);
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = core::ptr::addr_of!((*_2));
_18.0.0.2 = _16 as isize;
(*_8) = core::ptr::addr_of!(_12);
_21 = _28;
_21 = (*_11) > (*_11);
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 1482078831_i32 as usize;
(*_2) = !3_usize;
(*_1) = [_37,_37,_37,_37];
_13 = _38 >> (*_11);
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_28 = _21;
Goto(bb7)
}
bb43 = {
(*_1) = [_37,_37,_37,_37];
_67.0 = _16;
_6 = _59;
_59 = _61 as f32;
(*_9) = core::ptr::addr_of!((*_2));
_11 = &_60;
(*_4) = _32 * _32;
_40 = _28;
_9 = core::ptr::addr_of_mut!((*_9));
(*_2) = 0_usize;
_6 = _19;
(*_2) = !12563575633195237568_usize;
_62 = _54;
_66 = _38 | _78;
_28 = _78 > _18.0.0.2;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_2) = 5_usize - 7706615384262409171_usize;
(*_4) = _32 + _32;
_50 = core::ptr::addr_of_mut!((*_9));
(*_4) = -_32;
place!(Field::<i32>(Variant(_75, 0), 2)) = _33 & _33;
_83 = [267674562847476955549753872407863431145_u128,2379348603649307551865531756219779707_u128,184218914033163877995736990210551924419_u128,88313463333061237855524658813526666382_u128,28901571650490468360256310780951908286_u128,309549240907632722523524251564470151147_u128];
_43 = [202438408875998602328832416670367470587_u128,314052944973778762724752394998360679002_u128,22516703808786855170885031179485566347_u128,155017111170389520386204130155333116273_u128,259390521135580803154039333677246763172_u128];
(*_2) = !3_usize;
Goto(bb30)
}
bb44 = {
_59 = _19;
_80 = _47;
(*_9) = core::ptr::addr_of!(_18.0.0.1);
(*_50) = core::ptr::addr_of!((*_2));
place!(Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2)).3 = _91.1.fld1.3 | _91.1.fld1.3;
_69.0 = Field::<u64>(Variant(_75, 0), 1);
(*_63).1 = Move((*_9));
_41 = &mut place!(Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2)).3;
(*_9) = Move((*_50));
_51 = [(*_63).0,(*_63).0,(*_63).0,_91.1.fld3.0,(*_63).0,(*_63).0,(*_63).0];
(*_1) = [Field::<u64>(Variant(_75, 0), 1),_69.0,_69.0,Field::<u64>(Variant(_75, 0), 1)];
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_8 = core::ptr::addr_of!(_1);
_73 = core::ptr::addr_of_mut!((*_50));
(*_73) = core::ptr::addr_of!(_18.0.0.1);
_31 = _91.0 | _91.0;
match _91.1.fld1.3 {
0 => bb38,
1 => bb37,
2 => bb11,
3 => bb41,
4 => bb45,
5 => bb46,
6 => bb47,
318032508709213973593432838957477110684 => bb49,
_ => bb48
}
}
bb45 = {
(*_9) = core::ptr::addr_of!((*_2));
_54 = _5;
(*_2) = 8856145510149964686_usize >> _18.1;
_18.0.0.2 = _13 >> (*_2);
(*_2) = 5_usize & 2659170027772848947_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_4) = -_32;
(*_2) = 4992213655681271844_usize;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
_58 = _30;
(*_1) = [_37,_37,_37,_37];
(*_2) = !1_usize;
_1 = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 5_usize;
Call((*_2) = core::intrinsics::transmute(_46), ReturnTo(bb16), UnwindUnreachable())
}
bb46 = {
(*_1) = [_37,_37,_37,_37];
(*_9) = core::ptr::addr_of!(_18.0.0.1);
(*_2) = 3_usize >> _47;
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
_30 = (-1823211688_i32) as f32;
Goto(bb10)
}
bb47 = {
(*_9) = core::ptr::addr_of!((*_2));
_54 = _5;
(*_2) = 8856145510149964686_usize >> _18.1;
_18.0.0.2 = _13 >> (*_2);
(*_2) = 5_usize & 2659170027772848947_usize;
(*_8) = core::ptr::addr_of!((*_1));
(*_4) = -_32;
(*_2) = 4992213655681271844_usize;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
_58 = _30;
(*_1) = [_37,_37,_37,_37];
(*_2) = !1_usize;
_1 = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = 5_usize;
Call((*_2) = core::intrinsics::transmute(_46), ReturnTo(bb16), UnwindUnreachable())
}
bb48 = {
_8 = Move(RET);
_16 = 131668027761065884349655077423270633981_i128 >> _7;
(*_1) = [3234780354176104662_u64,5623540069258618539_u64,18363498713656820985_u64,5914102441011516738_u64];
_15.3 = _7 - _7;
(*_1) = [5102672399693413385_u64,11364386532850170189_u64,11008862092304939231_u64,16501124464905789426_u64];
_15.0 = !23894_u16;
(*_1) = [3442775437923707038_u64,9716944839008866680_u64,3374198821967240459_u64,9029584958961166207_u64];
_8 = core::ptr::addr_of!(_1);
_3 = &mut _7;
(*_3) = _15.3 >> _15.3;
(*_9) = core::ptr::addr_of!(_18.0.0.1);
Goto(bb2)
}
bb49 = {
(*_1) = [_37,_69.0,Field::<u64>(Variant(_75, 0), 1),_37];
_90 = 83_i8 | (-50_i8);
_104 = _84;
_18.1 = _82 + _18.0.0.0;
(*_50) = Move((*_9));
_102 = _104 >= _78;
(*_9) = core::ptr::addr_of!(_18.0.0.1);
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) | Field::<i32>(Variant(_75, 0), 2);
(*_2) = 10993299323557106368_usize | 2_usize;
(*_4) = _49 * Field::<f64>(Variant(_69.1.1.0, 2), 1);
(*_9) = core::ptr::addr_of!((*_2));
(*_63).1 = core::ptr::addr_of!((*_2));
_26 = _30 + _6;
_3 = &mut (*_41);
_24 = _28 | _102;
(*_63).1 = core::ptr::addr_of!((*_2));
Call(_91.1.fld1.0 = core::intrinsics::transmute(_91.1.fld4), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_50) = core::ptr::addr_of!((*_2));
_69.0 = _37 << _47;
_8 = core::ptr::addr_of!((*_8));
(*_3) = _91.1.fld1.3 << _18.1;
_51 = [(*_63).0,(*_63).0,(*_63).0,(*_63).0,(*_63).0,(*_63).0,(*_63).0];
(*_63).0 = !Field::<i32>(Variant(_75, 0), 2);
(*_63).0 = -_91.1.fld1.1;
(*_3) = !_91.1.fld1.3;
(*_9) = core::ptr::addr_of!((*_2));
(*_1) = [_69.0,_37,_69.0,_69.0];
place!(Field::<f64>(Variant(_69.1.1.0, 2), 1)) = -(*_4);
(*_3) = !_91.1.fld1.3;
_60 = !Field::<u32>(Variant(_69.1.1.0, 2), 4);
_40 = _13 >= Field::<isize>(Variant(_69.1.3, 0), 0);
(*_2) = 10637804935265328510_usize;
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) | Field::<i32>(Variant(_75, 0), 2);
(*_1) = [_69.0,_69.0,_69.0,_69.0];
(*_2) = (*_63).0 as usize;
(*_50) = core::ptr::addr_of!((*_2));
(*_2) = 6_usize & 9997907355971035888_usize;
_49 = (*_4) - (*_4);
_111 = Adt22::Variant1 { fld0: Move((*_63).1),fld1: (*_2) };
(*_63).0 = (*_2) as i32;
(*_50) = core::ptr::addr_of!(_18.0.0.1);
Goto(bb51)
}
bb51 = {
_91.1.fld5 = [_91.1.fld1.0,_91.1.fld1.0,_91.1.fld4,Field::<u16>(Variant(_69.1.1.0, 2), 2)];
(*_4) = _30 as f64;
(*_1) = [_69.0,_69.0,_69.0,_69.0];
_67.0 = _16;
(*_4) = _49 + _49;
_71 = [_18.1,_47,_47,_82];
(*_4) = Field::<f64>(Variant(_69.1.1.0, 2), 1);
_77 = _28;
_88 = &_11;
(*_63).0 = -_72;
place!(Field::<*const usize>(Variant(_69.1.1.0, 2), 3)) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
(*_63).0 = _72 - Field::<i32>(Variant(_75, 0), 2);
_73 = core::ptr::addr_of_mut!((*_63).1);
match _91.1.fld1.3 {
0 => bb37,
1 => bb41,
318032508709213973593432838957477110684 => bb52,
_ => bb27
}
}
bb52 = {
_21 = _40;
_18.0.0 = (_80, Field::<usize>(Variant(_111, 1), 1), _38);
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_69.0,_69.0,_69.0,_37];
(*_73) = Move((*_50));
_85 = _38 & _70;
(*_3) = !_91.1.fld1.3;
match _91.1.fld1.3 {
0 => bb25,
1 => bb17,
2 => bb53,
318032508709213973593432838957477110684 => bb55,
_ => bb54
}
}
bb53 = {
(*_2) = 5495525255572908170_usize & 3_usize;
_4 = core::ptr::addr_of!((*_4));
(*_2) = !11875522457210214146_usize;
_16 = _67.0;
(*_4) = _32 - _32;
_69.1.1.2 = &mut _33;
Goto(bb32)
}
bb54 = {
_16 = _91.1.fld4 as i128;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_82 = Field::<u32>(Variant(_36, 1), 0) as u8;
(*_63) = (_72, Move(Field::<*const usize>(Variant(_69.1.1.0, 2), 3)));
_50 = core::ptr::addr_of_mut!(_69.1.1.1);
place!(Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2)) = _91.1.fld1;
(*_4) = _49 * Field::<f64>(Variant(_69.1.1.0, 2), 1);
match Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3 {
0 => bb5,
1 => bb23,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
6 => bb39,
318032508709213973593432838957477110684 => bb41,
_ => bb40
}
}
bb55 = {
place!(Field::<[u128; 6]>(Variant(_69.1.1.0, 2), 0)) = _83;
_110 = !_77;
(*_4) = -Field::<f64>(Variant(_69.1.1.0, 2), 1);
_92 = -_104;
(*_50) = core::ptr::addr_of!((*_2));
_58 = _59;
_86 = &_60;
(*_50) = core::ptr::addr_of!((*_2));
(*_73) = Move((*_9));
(*_63).0 = Field::<i32>(Variant(_75, 0), 2);
_79 = &(*_88);
(*_63) = (Field::<i32>(Variant(_75, 0), 2), Move((*_50)));
Goto(bb56)
}
bb56 = {
(*_8) = core::ptr::addr_of!(_12);
_90 = 43_i8 ^ 11_i8;
_115.fld5 = (*_63).0;
place!(Field::<*const usize>(Variant(_111, 1), 0)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
(*_50) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
_98 = Field::<u16>(Variant(_69.1.1.0, 2), 2);
match _91.1.fld1.3 {
0 => bb32,
1 => bb51,
318032508709213973593432838957477110684 => bb58,
_ => bb57
}
}
bb57 = {
_13 = _18.0.0.2 << _18.1;
_25.fld4 = 54313_u16 - 18450_u16;
_22 = !67930094717071754583911438247385535121_u128;
_14 = [_22,_22,_22,_22,_22];
_37 = !17310858414401347479_u64;
_25.fld1.3 = _22 * _22;
_25.fld1.2 = _5;
_2 = core::ptr::addr_of!(_18.0.0.1);
_35 = !3642493095_u32;
_25.fld1.1 = (-1206827933_i32) >> _18.1;
(*_2) = _25.fld1.3 as usize;
(*_2) = _25.fld1.3 as usize;
_18.0.1 = &mut _25.fld1.3;
_38 = _13 - _18.0.0.2;
_31 = _28 as i16;
_18.0.0 = (_18.1, 6087046900312169704_usize, _38);
_11 = &_35;
(*_2) = (-570027312_i32) as usize;
_2 = core::ptr::addr_of!((*_2));
_8 = core::ptr::addr_of!(_1);
_2 = core::ptr::addr_of!((*_2));
_31 = 88_i8 as i16;
Goto(bb6)
}
bb58 = {
place!(Field::<char>(Variant(_69.1.3, 0), 1)) = _5;
(*_63) = (_91.1.fld1.1, Move((*_50)));
(*_9) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
(*_8) = core::ptr::addr_of!((*_1));
(*_63) = (Field::<i32>(Variant(_75, 0), 2), Move((*_9)));
(*_63).0 = _115.fld5;
_97 = core::ptr::addr_of!(_107);
_19 = _58;
(*_50) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
match _91.1.fld1.3 {
0 => bb59,
318032508709213973593432838957477110684 => bb61,
_ => bb60
}
}
bb59 = {
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = 6_usize;
_61 = !(-6154519000583895302_i64);
(*_2) = 7_usize - 13755639484738868346_usize;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_4) = _31 as f64;
(*_1) = [Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_19 = _30 + _59;
_80 = _47;
_4 = core::ptr::addr_of!((*_4));
Call(_33 = core::intrinsics::transmute(_60), ReturnTo(bb28), UnwindUnreachable())
}
bb60 = {
_62 = _5;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),_37];
(*_2) = 9292741780168803548_usize >> _38;
Goto(bb25)
}
bb61 = {
(*_4) = -Field::<f64>(Variant(_69.1.1.0, 2), 1);
(*_3) = _91.1.fld1.3 >> (*_86);
_23 = _69.1.2;
(*_8) = core::ptr::addr_of!((*_1));
place!(Field::<*const usize>(Variant(_69.1.1.0, 2), 3)) = Move((*_63).1);
_53 = _77;
(*_63).1 = core::ptr::addr_of!(_18.0.0.1);
_87 = core::ptr::addr_of!((*_97));
(*_1) = [_69.0,_69.0,_69.0,_69.0];
_117.1.1.2 = &mut (*_63).0;
_96 = (*_86) & Field::<u32>(Variant(_69.1.1.0, 2), 4);
_117.1.2 = _23;
(*_9) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
_69.0 = _37 >> (*_3);
(*_97) = _18.0.0.0;
(*_1) = [_69.0,_69.0,_69.0,_69.0];
(*_50) = core::ptr::addr_of!((*_2));
_18.1 = (*_97) | _47;
_115.fld4 = _58 + _58;
_117.1.1.0 = Move(_69.1.1.0);
Goto(bb62)
}
bb62 = {
_39 = Move(_75);
_3 = Move(_41);
_97 = Move(_87);
place!(Field::<[u128; 6]>(Variant(_117.1.1.0, 2), 0)) = [127163617234886738261817571223673721689_u128,49164118049955230539571856756862386956_u128,293024905522932183940238711938087989009_u128,313042343101932766841287847476079747096_u128,269008163084110182043101238171739290597_u128,221590135201760604424852929291015251413_u128];
_122 = !_70;
(*_2) = !_18.0.0.1;
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb63)
}
bb63 = {
_133 = (_98, _115.fld5, _5, 268436121544669300796747207133555590460_u128);
(*_50) = core::ptr::addr_of!((*_2));
_130 = [_133.3,_133.3,_133.3,_133.3];
_90 = 29_i8 | (-3_i8);
(*_9) = core::ptr::addr_of!((*_2));
_130 = [_133.3,_133.3,_133.3,_133.3];
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb64)
}
bb64 = {
(*_2) = _69.0 as usize;
match _133.3 {
0 => bb45,
1 => bb29,
268436121544669300796747207133555590460 => bb66,
_ => bb65
}
}
bb65 = {
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = 6_usize;
_61 = !(-6154519000583895302_i64);
(*_2) = 7_usize - 13755639484738868346_usize;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_4) = _32;
(*_4) = _31 as f64;
(*_1) = [Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_19 = _30 + _59;
_80 = _47;
_4 = core::ptr::addr_of!((*_4));
Call(_33 = core::intrinsics::transmute(_60), ReturnTo(bb28), UnwindUnreachable())
}
bb66 = {
place!(Field::<u64>(Variant(_39, 0), 1)) = _69.0 * _69.0;
_115.fld1 = Adt28::Variant1 { fld0: _96,fld1: _54,fld2: _133 };
_76 = [(*_86),(*_86),(*_86)];
_81 = Adt48::Variant0 { fld0: _16,fld1: _5,fld2: _69.0,fld3: _90 };
_123 = Field::<char>(Variant(_69.1.3, 0), 1);
_105 = -_84;
(*_2) = !_18.0.0.1;
(*_63).1 = core::ptr::addr_of!((*_2));
_35 = !(*_86);
_99 = _83;
_18.1 = _18.0.0.0 * _82;
_98 = _133.0 ^ Field::<u16>(Variant(_117.1.1.0, 2), 2);
_119 = Field::<u32>(Variant(_115.fld1, 1), 0) as f64;
match Field::<(u16, i32, char, u128)>(Variant(_115.fld1, 1), 2).3 {
0 => bb27,
1 => bb7,
268436121544669300796747207133555590460 => bb68,
_ => bb67
}
}
bb67 = {
_62 = _5;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),_37];
(*_2) = 9292741780168803548_usize >> _38;
Goto(bb25)
}
bb68 = {
_70 = _122 | _84;
match _133.3 {
0 => bb8,
1 => bb69,
2 => bb70,
3 => bb71,
4 => bb72,
268436121544669300796747207133555590460 => bb74,
_ => bb73
}
}
bb69 = {
_62 = _5;
(*_1) = [_37,_37,Field::<u64>(Variant(_75, 0), 1),_37];
(*_2) = 9292741780168803548_usize >> _38;
Goto(bb25)
}
bb70 = {
(*_8) = core::ptr::addr_of!((*_1));
(*_2) = !3286440496194051196_usize;
_6 = (*_11) as f32;
_18.0.0.0 = _18.1;
_20 = _14;
(*_8) = core::ptr::addr_of!((*_1));
Goto(bb8)
}
bb71 = {
_23 = _69.1.2;
_1 = core::ptr::addr_of!((*_1));
_47 = _18.1;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
Goto(bb29)
}
bb72 = {
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_37,_37,_37,_37];
(*_1) = [_37,_37,_37,_37];
_18.0.0.1 = 1_usize * 12011461152280640345_usize;
(*_8) = core::ptr::addr_of!((*_1));
_47 = _18.0.0.0;
_32 = (-69_i8) as f64;
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
_38 = _13;
(*_1) = [_37,_37,_37,_37];
(*_8) = core::ptr::addr_of!((*_1));
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb9)
}
bb73 = {
place!(Field::<char>(Variant(_69.1.3, 0), 1)) = _5;
(*_63) = (_91.1.fld1.1, Move((*_50)));
(*_9) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
(*_8) = core::ptr::addr_of!((*_1));
(*_63) = (Field::<i32>(Variant(_75, 0), 2), Move((*_9)));
(*_63).0 = _115.fld5;
_97 = core::ptr::addr_of!(_107);
_19 = _58;
(*_50) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_111, 1), 1)));
match _91.1.fld1.3 {
0 => bb59,
318032508709213973593432838957477110684 => bb61,
_ => bb60
}
}
bb74 = {
_94 = _133.3 % _133.3;
(*_2) = _18.0.0.1 - _18.0.0.1;
_71 = [_18.1,_80,_18.1,_47];
_11 = &_35;
_9 = Move(_50);
_101 = core::ptr::addr_of!((*_8));
_41 = &mut _133.3;
(*_2) = _18.0.0.1;
(*_1) = [_69.0,Field::<u64>(Variant(_39, 0), 1),_69.0,Field::<u64>(Variant(_39, 0), 1)];
(*_8) = core::ptr::addr_of!((*_1));
(*_8) = core::ptr::addr_of!((*_1));
_102 = _53;
_68 = Field::<[i128; 4]>(Variant(_39, 0), 0);
_142 = _85;
(*_2) = _18.0.0.1 | _18.0.0.1;
_13 = _105;
_79 = &_65;
_24 = _77;
_69.1.1 = (Move(_117.1.1.0), Move((*_63).1), Move(_117.1.1.2));
_21 = _53 | _53;
place!(Field::<char>(Variant(_69.1.3, 0), 1)) = Field::<char>(Variant(_115.fld1, 1), 1);
(*_1) = [Field::<u64>(Variant(_39, 0), 1),_37,Field::<u64>(Variant(_81, 0), 2),Field::<u64>(Variant(_39, 0), 1)];
Goto(bb75)
}
bb75 = {
_117.1.1.1 = core::ptr::addr_of!((*_2));
(*_63).1 = core::ptr::addr_of!((*_2));
_58 = -_59;
place!(Field::<i8>(Variant(_81, 0), 3)) = _90 * _90;
_105 = _84;
(*_8) = core::ptr::addr_of!((*_1));
_115.fld4 = _26 + _26;
(*_2) = !_18.0.0.1;
_147 = -_19;
_142 = (*_2) as isize;
_137 = [_31,_31,_31,_31];
_41 = &mut place!(Field::<(u16, i32, char, u128)>(Variant(_115.fld1, 1), 2)).3;
_38 = _105 >> Field::<u32>(Variant(_69.1.1.0, 2), 4);
_21 = _28 ^ _28;
(*_4) = _119 * _119;
match (*_41) {
0 => bb25,
1 => bb37,
2 => bb69,
3 => bb40,
4 => bb14,
5 => bb41,
6 => bb76,
268436121544669300796747207133555590460 => bb78,
_ => bb77
}
}
bb76 = {
(*_63).0 = 47896714913127495360462521299828749007_u128 as i32;
_60 = _35 << _78;
_68 = [_67.0,_67.0,_16,_16];
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),_37,_37];
(*_9) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_69.1.3, 1), 1)));
_91.1.fld1 = (_91.1.fld4, (*_63).0, _5, 318032508709213973593432838957477110684_u128);
_43 = [_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3,_91.1.fld1.3];
_66 = (*_2) as isize;
(*_1) = [_37,Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1),Field::<u64>(Variant(_75, 0), 1)];
_91.1.fld1.2 = _62;
(*_2) = _18.0.0.1 * _18.0.0.1;
(*_73) = core::ptr::addr_of!((*_2));
_36 = Adt28::Variant1 { fld0: _60,fld1: _5,fld2: _91.1.fld1 };
(*_63).0 = Field::<i32>(Variant(_75, 0), 2) + Field::<i32>(Variant(_75, 0), 2);
(*_73) = core::ptr::addr_of!((*_2));
_37 = Field::<u64>(Variant(_75, 0), 1) | Field::<u64>(Variant(_75, 0), 1);
_69.1.3 = Adt22::Variant0 { fld0: _78,fld1: _5 };
(*_73) = Move((*_9));
(*_9) = core::ptr::addr_of!(_18.0.0.1);
_84 = _18.0.0.2 ^ _18.0.0.2;
_76 = [Field::<u32>(Variant(_36, 1), 0),Field::<u32>(Variant(_36, 1), 0),_35];
_20 = [Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,_91.1.fld1.3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3,Field::<(u16, i32, char, u128)>(Variant(_36, 1), 2).3];
_59 = _58;
_69.1.1.0 = Adt38::Variant2 { fld0: _83,fld1: (*_4),fld2: _91.1.fld1.0,fld3: Move((*_63).1),fld4: _60 };
(*_63).1 = Move((*_9));
_69.1.2 = _23;
(*_4) = _90 as f64;
Call(_72 = core::intrinsics::bswap((*_63).0), ReturnTo(bb34), UnwindUnreachable())
}
bb77 = {
_21 = _40;
_18.0.0 = (_80, Field::<usize>(Variant(_111, 1), 1), _38);
(*_8) = core::ptr::addr_of!((*_1));
(*_1) = [_69.0,_69.0,_69.0,_37];
(*_73) = Move((*_50));
_85 = _38 & _70;
(*_3) = !_91.1.fld1.3;
match _91.1.fld1.3 {
0 => bb25,
1 => bb17,
2 => bb53,
318032508709213973593432838957477110684 => bb55,
_ => bb54
}
}
bb78 = {
(*_2) = _18.0.0.1;
_23 = [Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),_90,Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3)];
(*_8) = core::ptr::addr_of!((*_1));
_72 = Field::<i32>(Variant(_39, 0), 2) >> _104;
place!(Field::<u64>(Variant(_81, 0), 2)) = Field::<u64>(Variant(_39, 0), 1);
(*_4) = _119 - _119;
(*_8) = core::ptr::addr_of!((*_1));
match (*_41) {
268436121544669300796747207133555590460 => bb79,
_ => bb40
}
}
bb79 = {
_50 = core::ptr::addr_of_mut!((*_63).1);
(*_50) = core::ptr::addr_of!((*_2));
(*_8) = core::ptr::addr_of!((*_1));
_85 = _84;
(*_63).1 = core::ptr::addr_of!((*_2));
(*_4) = _119 - _119;
_129 = _107;
_73 = Move(_9);
(*_41) = _94 * _94;
_71 = [_47,_18.1,_47,_18.1];
(*_2) = !_18.0.0.1;
(*_41) = _54 as u128;
_4 = core::ptr::addr_of!((*_4));
place!(Field::<[u128; 6]>(Variant(_69.1.1.0, 2), 0)) = _83;
_21 = _77 | _77;
_138 = (*_41);
Goto(bb80)
}
bb80 = {
_151 = _77;
(*_41) = _78 as u128;
_13 = -_18.0.0.2;
(*_8) = core::ptr::addr_of!((*_1));
_125 = [_31,_31,_31];
(*_41) = _138;
_114 = Field::<char>(Variant(_81, 0), 1) as u16;
(*_41) = _94;
_92 = Field::<i8>(Variant(_81, 0), 3) as isize;
_53 = (*_11) > (*_11);
(*_2) = _18.0.0.1 * _18.0.0.1;
_69.1.2 = [_90,_90,Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),_90,Field::<i8>(Variant(_81, 0), 3),_90];
place!(Field::<*const usize>(Variant(_69.1.1.0, 2), 3)) = Move((*_63).1);
_111 = Adt22::Variant1 { fld0: Move(_2),fld1: _18.0.0.1 };
Goto(bb81)
}
bb81 = {
_69.0 = !Field::<u64>(Variant(_81, 0), 2);
(*_1) = [_69.0,Field::<u64>(Variant(_81, 0), 2),_69.0,_37];
_65 = Move(_86);
_135 = _105;
_86 = Move(_11);
_66 = _46;
_144 = _85 - _46;
_116 = &_96;
(*_63).1 = core::ptr::addr_of!(_128);
(*_8) = core::ptr::addr_of!((*_1));
_8 = Move(_101);
_106 = (*_41) ^ (*_41);
(*_4) = -_119;
_49 = -(*_4);
(*_4) = _49 + _49;
_128 = Field::<usize>(Variant(_111, 1), 1) | Field::<usize>(Variant(_111, 1), 1);
_12 = [Field::<u64>(Variant(_39, 0), 1),Field::<u64>(Variant(_81, 0), 2),Field::<u64>(Variant(_39, 0), 1),Field::<u64>(Variant(_81, 0), 2)];
place!(Field::<usize>(Variant(_111, 1), 1)) = _128;
_126 = _104 * _135;
Goto(bb82)
}
bb82 = {
_49 = _98 as f64;
_94 = (*_41);
(*_1) = [Field::<u64>(Variant(_39, 0), 1),Field::<u64>(Variant(_81, 0), 2),_69.0,Field::<u64>(Variant(_81, 0), 2)];
(*_41) = !_94;
_72 = !Field::<i32>(Variant(_39, 0), 2);
_23 = [Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),Field::<i8>(Variant(_81, 0), 3),_90,Field::<i8>(Variant(_81, 0), 3)];
_143 = (*_4) as u128;
place!(Field::<u64>(Variant(_39, 0), 1)) = !_69.0;
_2 = core::ptr::addr_of!(_128);
_144 = _61 as isize;
_18.0.0.1 = (*_2) + (*_2);
place!(Field::<i128>(Variant(_81, 0), 0)) = _67.0 - _67.0;
_159 = _59 * _30;
_8 = core::ptr::addr_of!(_1);
_13 = (*_4) as isize;
(*_41) = _106;
_60 = !(*_116);
_28 = _40;
_154 = _62;
(*_4) = _49 - _119;
_163 = -_159;
place!(Field::<isize>(Variant(_69.1.3, 0), 0)) = _135 ^ _84;
Goto(bb83)
}
bb83 = {
_101 = core::ptr::addr_of!((*_8));
place!(Field::<char>(Variant(_69.1.3, 0), 1)) = _54;
(*_41) = _94 | _106;
_117.0 = !Field::<u64>(Variant(_39, 0), 1);
(*_101) = core::ptr::addr_of!((*_1));
(*_1) = [Field::<u64>(Variant(_81, 0), 2),Field::<u64>(Variant(_39, 0), 1),Field::<u64>(Variant(_81, 0), 2),Field::<u64>(Variant(_81, 0), 2)];
(*_63).1 = Move(Field::<*const usize>(Variant(_69.1.1.0, 2), 3));
_40 = _77;
(*_101) = core::ptr::addr_of!((*_1));
(*_2) = _18.0.0.1 | Field::<usize>(Variant(_111, 1), 1);
_118 = Move(_63);
_88 = &_116;
(*_4) = _119;
_74 = !_129;
(*_8) = core::ptr::addr_of!((*_1));
RET = core::ptr::addr_of!((*_8));
(*_1) = [Field::<u64>(Variant(_81, 0), 2),_69.0,Field::<u64>(Variant(_81, 0), 2),Field::<u64>(Variant(_39, 0), 1)];
place!(Field::<char>(Variant(_81, 0), 1)) = Field::<char>(Variant(_69.1.3, 0), 1);
_161 = _123;
(*_1) = [_69.0,_117.0,_69.0,_117.0];
_117.0 = _69.0 >> _126;
place!(Field::<u64>(Variant(_81, 0), 2)) = _67.0 as u64;
_66 = _128 as isize;
(*_8) = core::ptr::addr_of!((*_1));
_104 = _84;
(*_41) = !_94;
Goto(bb84)
}
bb84 = {
Call(_171 = dump_var(Move(_47), Move(_28), Move(_142), Move(_13)), ReturnTo(bb85), UnwindUnreachable())
}
bb85 = {
Call(_171 = dump_var(Move(_151), Move(_107), Move(_23), Move(_85)), ReturnTo(bb86), UnwindUnreachable())
}
bb86 = {
Call(_171 = dump_var(Move(_161), Move(_144), Move(_154), Move(_37)), ReturnTo(bb87), UnwindUnreachable())
}
bb87 = {
Call(_171 = dump_var(Move(_16), Move(_71), Move(_133), Move(_42)), ReturnTo(bb88), UnwindUnreachable())
}
bb88 = {
Call(_171 = dump_var(Move(_84), Move(_5), Move(_102), Move(_38)), ReturnTo(bb89), UnwindUnreachable())
}
bb89 = {
Call(_171 = dump_var(Move(_129), Move(_35), Move(_135), Move(_51)), ReturnTo(bb90), UnwindUnreachable())
}
bb90 = {
Call(_171 = dump_var(Move(_24), Move(_105), Move(_61), Move(_33)), ReturnTo(bb91), UnwindUnreachable())
}
bb91 = {
Call(_171 = dump_var(Move(_122), Move(_83), Move(_60), Move(_77)), ReturnTo(bb92), UnwindUnreachable())
}
bb92 = {
Call(_171 = dump_var(Move(_76), Move(_54), Move(_62), Move(_92)), ReturnTo(bb93), UnwindUnreachable())
}
bb93 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *const usize,mut _2: *const [u64; 4],mut _3: (i32, *const usize),mut _4: [u64; 4],mut _5: u128,mut _6: *mut [i64; 2],mut _7: [u64; 4],mut _8: &'static mut u128,mut _9: *const *const [u64; 4],mut _10: *const f64,mut _11: *mut *const usize) -> u8 {
mir! {
type RET = u8;
let _12: ((u8, usize, isize), &'static mut u128);
let _13: [isize; 3];
let _14: &'static mut u128;
let _15: &'static Adt25;
let _16: f64;
let _17: isize;
let _18: [u32; 5];
let _19: isize;
let _20: [i128; 4];
let _21: *mut Adt48;
let _22: *const u8;
let _23: f64;
let _24: char;
let _25: [u32; 5];
let _26: isize;
let _27: *const &'static mut (i32, *const usize);
let _28: [isize; 3];
let _29: &'static Adt25;
let _30: char;
let _31: isize;
let _32: isize;
let _33: char;
let _34: &'static &'static u32;
let _35: isize;
let _36: f64;
let _37: isize;
let _38: *const &'static mut (i128, &'static mut u128);
let _39: *mut (&'static &'static u32, [u128; 6], char);
let _40: f32;
let _41: f32;
let _42: &'static mut i32;
let _43: i32;
let _44: ();
let _45: ();
{
_1 = Move(_3.1);
RET = 47_u8;
_9 = core::ptr::addr_of!(_2);
(*_9) = core::ptr::addr_of!(_4);
_9 = core::ptr::addr_of!((*_9));
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = _7;
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [899528011073560011_u64,659892215078455485_u64,3864349284304797672_u64,7212603612531072936_u64];
_16 = 16_i8 as f64;
(*_2) = [780930939021108974_u64,12742173323225005531_u64,4401817574562776627_u64,18107789070936372064_u64];
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb1)
}
bb1 = {
_12.1 = &mut _5;
(*_9) = core::ptr::addr_of!(_4);
(*_9) = core::ptr::addr_of!((*_2));
_14 = Move(_12.1);
_12.0.1 = 13330494754005553912_usize + 2842887533677100104_usize;
(*_9) = core::ptr::addr_of!((*_2));
_13 = [12_isize,(-126_isize),(-119_isize)];
_12.0 = (RET, 5_usize, 79_isize);
(*_2) = [5489598055253792850_u64,10787819666620455485_u64,16371905607667214332_u64,13632394890646275575_u64];
_17 = (-600234965114635417_i64) as isize;
_18 = [2664401336_u32,3474565908_u32,3030241996_u32,2154153706_u32,2679049103_u32];
_12.0.2 = _17 | _17;
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [471325266687560268_u64,13111914474321274482_u64,6623786720261075357_u64,10600039858385788435_u64];
_3.1 = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_12.0.2 = _17;
(*_9) = core::ptr::addr_of!((*_2));
_3 = ((-1512891978_i32), Move(_1));
Call((*_2) = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_2) = [1488662418737605562_u64,4478227065496960685_u64,3257503783494794645_u64,5344368177781133651_u64];
(*_9) = core::ptr::addr_of!((*_2));
_1 = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!((*_2));
(*_1) = !4_usize;
_7 = [3280383652909726257_u64,15874243823030729256_u64,244226330728072248_u64,1624556639225069434_u64];
(*_1) = _3.0 as usize;
(*_1) = 1_usize;
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [11209627710552782879_u64,12777643752818765380_u64,16486647728346153693_u64,16836962366243023486_u64];
(*_1) = 1330884009339602107_usize;
_3.1 = Move(_1);
match _12.0.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
47 => bb10,
_ => bb9
}
}
bb3 = {
_12.1 = &mut _5;
(*_9) = core::ptr::addr_of!(_4);
(*_9) = core::ptr::addr_of!((*_2));
_14 = Move(_12.1);
_12.0.1 = 13330494754005553912_usize + 2842887533677100104_usize;
(*_9) = core::ptr::addr_of!((*_2));
_13 = [12_isize,(-126_isize),(-119_isize)];
_12.0 = (RET, 5_usize, 79_isize);
(*_2) = [5489598055253792850_u64,10787819666620455485_u64,16371905607667214332_u64,13632394890646275575_u64];
_17 = (-600234965114635417_i64) as isize;
_18 = [2664401336_u32,3474565908_u32,3030241996_u32,2154153706_u32,2679049103_u32];
_12.0.2 = _17 | _17;
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [471325266687560268_u64,13111914474321274482_u64,6623786720261075357_u64,10600039858385788435_u64];
_3.1 = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_12.0.2 = _17;
(*_9) = core::ptr::addr_of!((*_2));
_3 = ((-1512891978_i32), Move(_1));
Call((*_2) = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
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
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [18335517427071269721_u64,5380663669908231838_u64,12369694329361986667_u64,7421395898921655448_u64];
(*_2) = _7;
(*_2) = _7;
(*_2) = [6456285232456354973_u64,1638144058771501565_u64,17997361718521509773_u64,7668040738524827505_u64];
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb11)
}
bb11 = {
(*_9) = core::ptr::addr_of!((*_2));
match _3.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
340282366920938463463374607430255319478 => bb12,
_ => bb5
}
}
bb12 = {
(*_2) = [4917338011801975618_u64,1705667749663756310_u64,12661476810936928908_u64,14084952853070444253_u64];
(*_2) = [12139133214457412052_u64,8039680646919715773_u64,9939877209124200914_u64,16143544745285259427_u64];
(*_9) = core::ptr::addr_of!((*_2));
_10 = core::ptr::addr_of!(_16);
(*_2) = [15871991970176987721_u64,114761839345131622_u64,17833397801684468553_u64,10659821415782010038_u64];
(*_9) = core::ptr::addr_of!((*_2));
_12.0.2 = _17;
(*_10) = 120717244409686545379804368497570244328_u128 as f64;
(*_10) = 4337614321841458151_u64 as f64;
Goto(bb13)
}
bb13 = {
(*_10) = 3817526488_u32 as f64;
(*_10) = 43_i8 as f64;
(*_10) = 5174_i16 as f64;
_10 = core::ptr::addr_of!((*_10));
(*_10) = _3.0 as f64;
_2 = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_3.1 = core::ptr::addr_of!(_12.0.1);
(*_10) = _3.0 as f64;
(*_10) = 164270350696645267793388309681706949924_u128 as f64;
(*_2) = _7;
_12.0.1 = _3.0 as usize;
_4 = [10497538638432515838_u64,10723805669797945525_u64,8678341487091490786_u64,12287166255322898490_u64];
_18 = [2660259678_u32,1854508014_u32,3181613858_u32,3361850828_u32,1540977668_u32];
(*_2) = [13109807537687816508_u64,6254195504687929488_u64,7572177496191735519_u64,18113554546578676838_u64];
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_10) = _12.0.2 as f64;
(*_10) = 34920067378803905402536805321258457006_i128 as f64;
(*_2) = _7;
(*_2) = [4483000516393174876_u64,2689534079396736088_u64,13661473172137969202_u64,3011300523381466385_u64];
_16 = 12745467152144601581_u64 as f64;
_25 = [190921419_u32,3045231703_u32,524489813_u32,3769916389_u32,430677884_u32];
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_10) = 28981_i16 as f64;
(*_10) = (-7212624013391209092_i64) as f64;
match _3.0 {
340282366920938463463374607430255319478 => bb15,
_ => bb14
}
}
bb14 = {
(*_9) = core::ptr::addr_of!((*_2));
match _3.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
340282366920938463463374607430255319478 => bb12,
_ => bb5
}
}
bb15 = {
_11 = core::ptr::addr_of_mut!(_3.1);
(*_10) = 288904982154400919551932918647039511347_u128 as f64;
(*_2) = _7;
(*_2) = [11627936680536261342_u64,10739268365788893639_u64,11265587281890271823_u64,2999385097637260665_u64];
(*_2) = [1508682023331681839_u64,325901663065898605_u64,11685405078431624404_u64,8397613118315993387_u64];
_1 = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!((*_2));
_22 = core::ptr::addr_of!(RET);
_20 = [(-59705864306788317886512067071468792648_i128),(-64012440674780707961123526586055897727_i128),(-129792444562905636111384141984858204259_i128),55423567713393836993402708398076263877_i128];
(*_22) = _12.0.0 >> (*_1);
(*_22) = 39865136145802264797208946550129769725_i128 as u8;
(*_2) = [10616133814774389508_u64,11231497091363690283_u64,1725600691452042488_u64,8054221831426350774_u64];
_2 = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_11) = core::ptr::addr_of!((*_1));
Goto(bb16)
}
bb16 = {
(*_11) = Move(_1);
(*_10) = (-156029994148727904724569502954163826370_i128) as f64;
_9 = core::ptr::addr_of!((*_9));
(*_11) = core::ptr::addr_of!(_12.0.1);
_16 = 3503384720_u32 as f64;
(*_2) = [12017148546958765985_u64,10426718261595331658_u64,587486712705375070_u64,7255848426832577446_u64];
_13 = [_12.0.2,_17,_12.0.2];
(*_9) = core::ptr::addr_of!((*_2));
(*_11) = core::ptr::addr_of!(_12.0.1);
(*_11) = core::ptr::addr_of!(_12.0.1);
(*_10) = 58316_u16 as f64;
(*_2) = _7;
_17 = _12.0.2 & _12.0.2;
(*_2) = [14484842458966892621_u64,2750724040103164101_u64,9464806995993489905_u64,15189721014228254551_u64];
match _3.0 {
0 => bb17,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
340282366920938463463374607430255319478 => bb23,
_ => bb22
}
}
bb17 = {
_11 = core::ptr::addr_of_mut!(_3.1);
(*_10) = 288904982154400919551932918647039511347_u128 as f64;
(*_2) = _7;
(*_2) = [11627936680536261342_u64,10739268365788893639_u64,11265587281890271823_u64,2999385097637260665_u64];
(*_2) = [1508682023331681839_u64,325901663065898605_u64,11685405078431624404_u64,8397613118315993387_u64];
_1 = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!((*_2));
_22 = core::ptr::addr_of!(RET);
_20 = [(-59705864306788317886512067071468792648_i128),(-64012440674780707961123526586055897727_i128),(-129792444562905636111384141984858204259_i128),55423567713393836993402708398076263877_i128];
(*_22) = _12.0.0 >> (*_1);
(*_22) = 39865136145802264797208946550129769725_i128 as u8;
(*_2) = [10616133814774389508_u64,11231497091363690283_u64,1725600691452042488_u64,8054221831426350774_u64];
_2 = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_11) = core::ptr::addr_of!((*_1));
Goto(bb16)
}
bb18 = {
Return()
}
bb19 = {
(*_10) = 3817526488_u32 as f64;
(*_10) = 43_i8 as f64;
(*_10) = 5174_i16 as f64;
_10 = core::ptr::addr_of!((*_10));
(*_10) = _3.0 as f64;
_2 = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
_3.1 = core::ptr::addr_of!(_12.0.1);
(*_10) = _3.0 as f64;
(*_10) = 164270350696645267793388309681706949924_u128 as f64;
(*_2) = _7;
_12.0.1 = _3.0 as usize;
_4 = [10497538638432515838_u64,10723805669797945525_u64,8678341487091490786_u64,12287166255322898490_u64];
_18 = [2660259678_u32,1854508014_u32,3181613858_u32,3361850828_u32,1540977668_u32];
(*_2) = [13109807537687816508_u64,6254195504687929488_u64,7572177496191735519_u64,18113554546578676838_u64];
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
(*_10) = _12.0.2 as f64;
(*_10) = 34920067378803905402536805321258457006_i128 as f64;
(*_2) = _7;
(*_2) = [4483000516393174876_u64,2689534079396736088_u64,13661473172137969202_u64,3011300523381466385_u64];
_16 = 12745467152144601581_u64 as f64;
_25 = [190921419_u32,3045231703_u32,524489813_u32,3769916389_u32,430677884_u32];
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_10) = 28981_i16 as f64;
(*_10) = (-7212624013391209092_i64) as f64;
match _3.0 {
340282366920938463463374607430255319478 => bb15,
_ => bb14
}
}
bb20 = {
Return()
}
bb21 = {
(*_9) = core::ptr::addr_of!((*_2));
match _3.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
340282366920938463463374607430255319478 => bb12,
_ => bb5
}
}
bb22 = {
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_9) = core::ptr::addr_of!((*_2));
(*_2) = [18335517427071269721_u64,5380663669908231838_u64,12369694329361986667_u64,7421395898921655448_u64];
(*_2) = _7;
(*_2) = _7;
(*_2) = [6456285232456354973_u64,1638144058771501565_u64,17997361718521509773_u64,7668040738524827505_u64];
(*_2) = _7;
(*_9) = core::ptr::addr_of!((*_2));
Goto(bb11)
}
bb23 = {
(*_11) = core::ptr::addr_of!(_12.0.1);
Goto(bb24)
}
bb24 = {
_1 = core::ptr::addr_of!(_12.0.1);
(*_1) = (-28169414318915041803028132431785785764_i128) as usize;
(*_9) = core::ptr::addr_of!((*_2));
_19 = _17 << (*_1);
(*_1) = !9460809400045376551_usize;
(*_22) = _12.0.0 << _17;
(*_1) = (*_22) as usize;
(*_10) = 12090_u16 as f64;
_1 = core::ptr::addr_of!((*_1));
_12.0.1 = 403021506241665138_usize;
Goto(bb25)
}
bb25 = {
_17 = _19 >> (*_22);
(*_9) = core::ptr::addr_of!((*_2));
_10 = core::ptr::addr_of!((*_10));
(*_11) = core::ptr::addr_of!((*_1));
(*_11) = core::ptr::addr_of!((*_1));
(*_11) = core::ptr::addr_of!((*_1));
_20 = [58382157963046949391769912034289278521_i128,4420695228699312811013924976160676829_i128,(-163164839367646633045320523600483179855_i128),26015873041083158674791901108438505399_i128];
(*_1) = 6_usize;
_3.0 = (*_22) as i32;
(*_10) = (-4764142556832402268_i64) as f64;
(*_1) = 0_usize;
(*_9) = core::ptr::addr_of!(_7);
_4 = (*_2);
(*_10) = 58637_u16 as f64;
(*_2) = _4;
_19 = _17 & _17;
(*_22) = 4265635990_u32 as u8;
(*_11) = core::ptr::addr_of!((*_1));
match (*_1) {
0 => bb27,
_ => bb26
}
}
bb26 = {
Return()
}
bb27 = {
(*_1) = 1_usize + 2_usize;
(*_10) = 148146635626109426572019992105462435922_u128 as f64;
(*_9) = core::ptr::addr_of!((*_2));
(*_11) = Move(_1);
_17 = _19 + _19;
_12.0.0 = (*_22);
_33 = '\u{2d7cb}';
(*_22) = 265531930874607717958262387245605614190_u128 as u8;
(*_9) = core::ptr::addr_of!((*_2));
_3.0 = 1068849798_i32 | (-1259536535_i32);
_9 = core::ptr::addr_of!((*_9));
_32 = -_17;
(*_22) = _12.0.0 | _12.0.0;
_2 = core::ptr::addr_of!((*_2));
_12.0.1 = (-120_i8) as usize;
(*_11) = core::ptr::addr_of!(_12.0.1);
(*_11) = core::ptr::addr_of!(_12.0.1);
(*_9) = core::ptr::addr_of!(_4);
(*_22) = _12.0.0;
(*_2) = [4011728608558076189_u64,6216429257741854206_u64,14397666284996519089_u64,9287520611778641936_u64];
(*_2) = [6980757966174840478_u64,17205932549998318383_u64,5710941133945454502_u64,3895420244589932400_u64];
RET = (*_10) as u8;
_19 = _32;
(*_22) = _19 as u8;
_23 = (*_10) - (*_10);
_30 = _33;
(*_9) = core::ptr::addr_of!((*_2));
_3.0 = 1474120569_i32;
Goto(bb28)
}
bb28 = {
Call(_44 = dump_var(Move(_19), Move(_13), Move(_20), Move(_4)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_44 = dump_var(Move(_33), Move(_18), _45, _45), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: *const f64,mut _2: *mut *const usize,mut _3: Adt22,mut _4: *mut *const usize,mut _5: [u64; 4],mut _6: *const [u64; 4],mut _7: f64,mut _8: [i64; 2],mut _9: isize,mut _10: [i64; 2]) -> u8 {
mir! {
type RET = u8;
let _11: *mut [i64; 2];
let _12: u8;
let _13: isize;
let _14: *mut (&'static &'static u32, [u128; 6], char);
let _15: [i8; 7];
let _16: f32;
let _17: i16;
let _18: [i16; 3];
let _19: *const u8;
let _20: f32;
let _21: *mut (&'static &'static u32, [u128; 6], char);
let _22: (u16, i32, char, u128);
let _23: char;
let _24: &'static mut (i32, *const usize);
let _25: &'static mut i32;
let _26: bool;
let _27: bool;
let _28: *const *const [u64; 4];
let _29: bool;
let _30: *const (i32, *const usize);
let _31: ((i128, &'static mut u128),);
let _32: bool;
let _33: i64;
let _34: ((i128, &'static mut u128),);
let _35: u8;
let _36: &'static mut (i32, *const usize);
let _37: isize;
let _38: *const u8;
let _39: bool;
let _40: i128;
let _41: *const u8;
let _42: u64;
let _43: bool;
let _44: u16;
let _45: i32;
let _46: f64;
let _47: i8;
let _48: *const (i32, *const usize);
let _49: u8;
let _50: *const f64;
let _51: Adt48;
let _52: (&'static &'static u32, [u128; 6], char);
let _53: bool;
let _54: f64;
let _55: f32;
let _56: isize;
let _57: i128;
let _58: f64;
let _59: isize;
let _60: &'static u32;
let _61: ([char; 7], (Adt38, *const usize, &'static mut i32), [i8; 7], Adt22);
let _62: u128;
let _63: &'static &'static Adt25;
let _64: *const (i32, *const usize);
let _65: f64;
let _66: &'static mut &'static mut (i128, &'static mut u128);
let _67: ((i128, &'static mut u128),);
let _68: u16;
let _69: i8;
let _70: f64;
let _71: Adt28;
let _72: char;
let _73: f32;
let _74: u32;
let _75: i8;
let _76: [u128; 6];
let _77: f64;
let _78: i128;
let _79: u32;
let _80: *const u8;
let _81: &'static &'static u32;
let _82: i32;
let _83: isize;
let _84: f32;
let _85: [u128; 6];
let _86: bool;
let _87: isize;
let _88: *const &'static mut (i32, *const usize);
let _89: *const [u64; 4];
let _90: ();
let _91: ();
{
place!(Field::<char>(Variant(_3, 0), 1)) = '\u{ff45a}';
_2 = Move(_4);
_1 = core::ptr::addr_of!(_7);
(*_1) = (-74624011854402863830545333386366841234_i128) as f64;
RET = 221_u8 * 29_u8;
place!(Field::<char>(Variant(_3, 0), 1)) = '\u{482f2}';
(*_1) = 3202559581_u32 as f64;
_9 = Field::<isize>(Variant(_3, 0), 0) * Field::<isize>(Variant(_3, 0), 0);
_1 = core::ptr::addr_of!((*_1));
(*_1) = 139543505050565594257016748101894873492_i128 as f64;
_1 = core::ptr::addr_of!((*_1));
(*_1) = (-1185784234_i32) as f64;
RET = 253_u8 | 76_u8;
_4 = Move(_2);
(*_1) = 208422439313546063629287242438797516736_u128 as f64;
_5 = [7073716991302568408_u64,17818274673500207289_u64,2956532916256063553_u64,15189744375622040226_u64];
_6 = core::ptr::addr_of!(_5);
(*_6) = [10373579074878975015_u64,12635978457942528870_u64,3241830709838516066_u64,2228555131902382428_u64];
Goto(bb1)
}
bb1 = {
_8 = [(-531718874676918789_i64),4732653136467825131_i64];
place!(Field::<isize>(Variant(_3, 0), 0)) = _9;
(*_1) = 9636_i16 as f64;
_11 = core::ptr::addr_of_mut!(_10);
(*_6) = [10501077270769427739_u64,12557212381582516097_u64,5528502764854467678_u64,14296412883675281549_u64];
(*_1) = 281825005333820343224781362218269352816_u128 as f64;
(*_11) = [6809300340780483496_i64,(-7351877720800921350_i64)];
(*_11) = [(-5801207820937238409_i64),8116916313635669816_i64];
Goto(bb2)
}
bb2 = {
(*_6) = [4787445135363271282_u64,10834233147834389692_u64,17493195554723079934_u64,11271168765776084490_u64];
(*_6) = [15587090419884950705_u64,8828016661204999769_u64,8144797337898631388_u64,17291112677192596914_u64];
(*_11) = _8;
(*_6) = [9678370856019724632_u64,15730531966849183105_u64,10457549229437381732_u64,8467676084270625897_u64];
(*_1) = RET as f64;
_10 = [204370052291745032_i64,(-6122157306466526616_i64)];
(*_1) = RET as f64;
_3 = Adt22::Variant0 { fld0: _9,fld1: '\u{ec948}' };
(*_11) = _8;
(*_6) = [9520596766999788155_u64,9657898521775622337_u64,6839994473478582051_u64,3548890818464586727_u64];
(*_11) = [(-5084770713071420671_i64),(-6805320907289061451_i64)];
(*_11) = _8;
(*_6) = [14109351896995068224_u64,6757242264507705315_u64,9517754395474057920_u64,16231621224597419836_u64];
(*_6) = [17970272846943667446_u64,9737548197769637623_u64,11527230247641310241_u64,1997290232802040745_u64];
_13 = _9 - _9;
(*_6) = [14347637820860476846_u64,4224263388560133388_u64,10985890323891968617_u64,550545800389390714_u64];
(*_1) = 70_i8 as f64;
_11 = core::ptr::addr_of_mut!(_8);
(*_1) = 2146_u16 as f64;
(*_11) = _10;
_2 = Move(_4);
_16 = (*_1) as f32;
(*_6) = [12008471934986592423_u64,10488896325943631689_u64,17574132046642553360_u64,13852919282685034243_u64];
Goto(bb3)
}
bb3 = {
(*_1) = 10249671073255208389_u64 as f64;
_11 = core::ptr::addr_of_mut!((*_11));
(*_1) = RET as f64;
(*_6) = [704092815369424969_u64,10162375373355065098_u64,16726117543002059942_u64,10738510765361269044_u64];
(*_6) = [13913684664410625341_u64,2736601744548861775_u64,4433841197700986675_u64,3507988873468251448_u64];
_16 = (-37_i8) as f32;
(*_6) = [9913976445714227991_u64,8934431156160666172_u64,11308149610888501090_u64,4211719356828234779_u64];
(*_11) = [(-1268076618339990525_i64),1549993354192274264_i64];
_18 = [(-25161_i16),24249_i16,(-7146_i16)];
_6 = core::ptr::addr_of!(_5);
(*_1) = 13343539474115977452_u64 as f64;
(*_1) = 2081757039_i32 as f64;
_4 = Move(_2);
(*_1) = 12785725457980501129_u64 as f64;
(*_1) = 7042633694422893790_u64 as f64;
(*_1) = (-2038348544_i32) as f64;
(*_6) = [17564443444689997006_u64,3750692340134683640_u64,5368388817598780393_u64,12151137853442493914_u64];
(*_11) = [(-4810656671064630975_i64),(-1572649332317125012_i64)];
_7 = 13942533645160786680_u64 as f64;
(*_1) = 17058432630493988568_u64 as f64;
_19 = core::ptr::addr_of!(_12);
_3 = Adt22::Variant0 { fld0: _9,fld1: '\u{a9a03}' };
(*_6) = [6448407006266862695_u64,17875624172047033668_u64,17817622504105467873_u64,8201501620385996897_u64];
(*_6) = [15798780254133519304_u64,15476832700070066028_u64,8567564056343520566_u64,12968506364199418607_u64];
(*_6) = [13222389988204623537_u64,1107141856452304137_u64,15136564996833626926_u64,13462711521159280044_u64];
Goto(bb4)
}
bb4 = {
(*_6) = [8946182417499468178_u64,4474015404692408842_u64,2829831546275172100_u64,1471388937840836545_u64];
_22 = (40685_u16, 270986425_i32, '\u{ba5f3}', 259873143038898694136910332624736715237_u128);
(*_6) = [12659257689388056453_u64,16723614439234099060_u64,16421226291239617052_u64,15243095650481944542_u64];
(*_19) = RET >> _9;
(*_6) = [6305800160635826869_u64,13920556203003254937_u64,4688647496947675192_u64,2084989226398962122_u64];
_15 = [(-1_i8),75_i8,68_i8,(-109_i8),66_i8,(-82_i8),(-40_i8)];
(*_1) = 5941260881731652981_i64 as f64;
(*_6) = [11984433511751697384_u64,5122961445079933654_u64,11549890918098887487_u64,2508892804742605863_u64];
(*_11) = [7923310717753093816_i64,(-1805181262483338050_i64)];
(*_11) = _10;
place!(Field::<isize>(Variant(_3, 0), 0)) = _9 >> (*_19);
(*_6) = [5529011316531424483_u64,8328816101709630585_u64,5186073279868784903_u64,17433000880864438693_u64];
(*_11) = _10;
(*_11) = [1012923606136040728_i64,2829198221663364120_i64];
(*_11) = _10;
(*_1) = _16 as f64;
_6 = core::ptr::addr_of!((*_6));
_6 = core::ptr::addr_of!((*_6));
(*_11) = [866316689007957400_i64,(-7938074628081310901_i64)];
Goto(bb5)
}
bb5 = {
(*_6) = [9601206736000536881_u64,15419888286969892054_u64,17636645961900233952_u64,11128246407640058547_u64];
(*_11) = [(-8099156079307511326_i64),7325840365806188319_i64];
(*_11) = _10;
(*_1) = (-36_i8) as f64;
(*_11) = [(-378658177325429656_i64),867707200795930539_i64];
(*_11) = _10;
Goto(bb6)
}
bb6 = {
_20 = _16 - _16;
(*_6) = [11244667242832275653_u64,10677989737361031728_u64,2123880651084120785_u64,11717840945098317172_u64];
(*_19) = !RET;
_16 = _20;
(*_19) = RET ^ RET;
(*_19) = RET;
(*_19) = 15573860045411108239_u64 as u8;
(*_6) = [14675859733730561124_u64,4883453119792261478_u64,12391449543310906558_u64,14537937502143853292_u64];
(*_19) = RET >> _13;
(*_11) = _10;
(*_11) = _10;
Goto(bb7)
}
bb7 = {
(*_1) = _22.0 as f64;
(*_1) = _22.1 as f64;
(*_6) = [13972618621204992363_u64,10262158138476455874_u64,12134495512660913198_u64,1995026687939953390_u64];
_20 = _16 * _16;
(*_19) = !RET;
place!(Field::<char>(Variant(_3, 0), 1)) = _22.2;
(*_19) = RET;
_13 = !Field::<isize>(Variant(_3, 0), 0);
(*_1) = (-4193684393471862717_i64) as f64;
place!(Field::<isize>(Variant(_3, 0), 0)) = -_13;
_10 = [7479040181079515713_i64,5054986213370301800_i64];
_16 = _20 - _20;
(*_1) = _16 as f64;
_10 = [1671612421062726522_i64,2583756239922698887_i64];
(*_11) = _10;
_22.2 = Field::<char>(Variant(_3, 0), 1);
Goto(bb8)
}
bb8 = {
(*_19) = !RET;
(*_11) = _10;
_29 = !false;
_23 = Field::<char>(Variant(_3, 0), 1);
_2 = Move(_4);
_17 = (-12016_i16) + (-23977_i16);
(*_6) = [15619768842415688146_u64,3065375817702984515_u64,15472467421149711506_u64,6259351917662067844_u64];
(*_1) = 191565532130923465_usize as f64;
_25 = &mut _22.1;
(*_1) = _20 as f64;
(*_11) = _10;
place!(Field::<isize>(Variant(_3, 0), 0)) = _13 ^ _13;
(*_11) = _10;
(*_19) = RET | RET;
(*_25) = 1175947164_i32;
_29 = !false;
_23 = Field::<char>(Variant(_3, 0), 1);
(*_19) = !RET;
(*_11) = [(-6526095315038839199_i64),(-3086294362882506169_i64)];
(*_19) = RET | RET;
(*_11) = _10;
(*_1) = Field::<isize>(Variant(_3, 0), 0) as f64;
(*_25) = 1162505060_i32;
match (*_25) {
0 => bb5,
1 => bb4,
2 => bb9,
1162505060 => bb11,
_ => bb10
}
}
bb9 = {
(*_1) = _22.0 as f64;
(*_1) = _22.1 as f64;
(*_6) = [13972618621204992363_u64,10262158138476455874_u64,12134495512660913198_u64,1995026687939953390_u64];
_20 = _16 * _16;
(*_19) = !RET;
place!(Field::<char>(Variant(_3, 0), 1)) = _22.2;
(*_19) = RET;
_13 = !Field::<isize>(Variant(_3, 0), 0);
(*_1) = (-4193684393471862717_i64) as f64;
place!(Field::<isize>(Variant(_3, 0), 0)) = -_13;
_10 = [7479040181079515713_i64,5054986213370301800_i64];
_16 = _20 - _20;
(*_1) = _16 as f64;
_10 = [1671612421062726522_i64,2583756239922698887_i64];
(*_11) = _10;
_22.2 = Field::<char>(Variant(_3, 0), 1);
Goto(bb8)
}
bb10 = {
(*_6) = [4787445135363271282_u64,10834233147834389692_u64,17493195554723079934_u64,11271168765776084490_u64];
(*_6) = [15587090419884950705_u64,8828016661204999769_u64,8144797337898631388_u64,17291112677192596914_u64];
(*_11) = _8;
(*_6) = [9678370856019724632_u64,15730531966849183105_u64,10457549229437381732_u64,8467676084270625897_u64];
(*_1) = RET as f64;
_10 = [204370052291745032_i64,(-6122157306466526616_i64)];
(*_1) = RET as f64;
_3 = Adt22::Variant0 { fld0: _9,fld1: '\u{ec948}' };
(*_11) = _8;
(*_6) = [9520596766999788155_u64,9657898521775622337_u64,6839994473478582051_u64,3548890818464586727_u64];
(*_11) = [(-5084770713071420671_i64),(-6805320907289061451_i64)];
(*_11) = _8;
(*_6) = [14109351896995068224_u64,6757242264507705315_u64,9517754395474057920_u64,16231621224597419836_u64];
(*_6) = [17970272846943667446_u64,9737548197769637623_u64,11527230247641310241_u64,1997290232802040745_u64];
_13 = _9 - _9;
(*_6) = [14347637820860476846_u64,4224263388560133388_u64,10985890323891968617_u64,550545800389390714_u64];
(*_1) = 70_i8 as f64;
_11 = core::ptr::addr_of_mut!(_8);
(*_1) = 2146_u16 as f64;
(*_11) = _10;
_2 = Move(_4);
_16 = (*_1) as f32;
(*_6) = [12008471934986592423_u64,10488896325943631689_u64,17574132046642553360_u64,13852919282685034243_u64];
Goto(bb3)
}
bb11 = {
(*_25) = (-2129014750_i32) * (-612862480_i32);
Goto(bb12)
}
bb12 = {
_10 = [(-8047825507160588411_i64),7090327450203215095_i64];
(*_19) = RET - RET;
(*_19) = RET ^ RET;
(*_11) = _10;
(*_1) = 571910649_u32 as f64;
(*_1) = (*_19) as f64;
(*_11) = [(-4788427094874620291_i64),9079392276171138171_i64];
(*_1) = 4431631697052258000_u64 as f64;
(*_25) = 1716071254_i32;
_26 = !_29;
_13 = Field::<isize>(Variant(_3, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = [17749371762666740107_u64,13592049448986689216_u64,11027676143795238081_u64,17744784317061117330_u64];
(*_11) = [7688308575169711903_i64,(-3403787310970181673_i64)];
(*_25) = (-135682072369202004856763403063224555637_i128) as i32;
(*_19) = !RET;
(*_6) = [8782916526784665212_u64,17853485064502006065_u64,11843935161036951728_u64,10648271082651152803_u64];
_32 = _29 ^ _29;
(*_19) = RET << _9;
(*_1) = _16 as f64;
Goto(bb13)
}
bb13 = {
(*_25) = _16 as i32;
(*_6) = [8427309875573277252_u64,5347062798865183620_u64,5276930180337547438_u64,11457502982276269230_u64];
(*_25) = (-496174220_i32) + 351262758_i32;
_27 = !_32;
(*_6) = [17651400571542258089_u64,12565907943105712941_u64,8767614671065807231_u64,2847244641148921152_u64];
_10 = (*_11);
(*_1) = 2197285208_u32 as f64;
(*_19) = RET >> _13;
(*_11) = [(-7512713691243821651_i64),1074771782711715452_i64];
(*_25) = 1208951221_i32 & (-1413179932_i32);
_35 = !(*_19);
(*_19) = _35 * _35;
(*_1) = (-764820765818948112_i64) as f64;
(*_19) = _35;
_11 = core::ptr::addr_of_mut!((*_11));
(*_6) = [9081893457803251045_u64,8668079331724745300_u64,827213202162079474_u64,16666979529895207872_u64];
Goto(bb14)
}
bb14 = {
(*_6) = [11379348284490477669_u64,6086654681896154497_u64,7516221492718450246_u64,4038523503118438963_u64];
(*_6) = [5917603467251576895_u64,13468202196281886065_u64,7718299419933982991_u64,2170698804234548402_u64];
(*_19) = !_35;
_4 = Move(_2);
(*_1) = 102599784153461463954721661559210575834_u128 as f64;
(*_25) = _17 as i32;
(*_25) = 2132440654_i32;
(*_25) = (-974031896_i32);
match (*_25) {
0 => bb10,
1 => bb7,
2 => bb9,
340282366920938463463374607430794179560 => bb15,
_ => bb6
}
}
bb15 = {
(*_6) = [3170182336493838526_u64,8061190279735148204_u64,10982242688115512573_u64,17585515122941004159_u64];
(*_19) = 96296282901192498186095010549346291076_i128 as u8;
(*_1) = (*_25) as f64;
(*_19) = _35;
_12 = _35;
(*_11) = _10;
(*_6) = [9691776977277704455_u64,6965269854086103369_u64,11626958530591016352_u64,6784389696681243340_u64];
_2 = Move(_4);
(*_25) = _23 as i32;
_38 = core::ptr::addr_of!((*_19));
(*_1) = (-54_i8) as f64;
(*_19) = _20 as u8;
(*_19) = _35;
_16 = _20 + _20;
_31.0.0 = 133201984925018801038539948559821834731_i128 << (*_19);
(*_11) = [6151565973194459437_i64,7354576877206438389_i64];
(*_6) = [11187477214565057908_u64,15928649647507928142_u64,18023325592834813535_u64,10625753483597979263_u64];
(*_19) = 30062_u16 as u8;
(*_25) = (-123771555_i32) & 689879039_i32;
_38 = core::ptr::addr_of!((*_19));
Goto(bb16)
}
bb16 = {
(*_6) = [5295658618944503827_u64,11392894462626117727_u64,4733745057732880179_u64,9109944856945604700_u64];
(*_6) = [2090014353455446461_u64,1817812643532732665_u64,3123521315286408333_u64,10380719714809356098_u64];
(*_38) = !_35;
(*_11) = [(-4735747871141153233_i64),(-5156890086207360383_i64)];
(*_38) = _35;
(*_25) = (-807155954_i32) & (-1766473427_i32);
(*_19) = _35 & _35;
(*_6) = [9399296900964429957_u64,11381003086292903236_u64,2982134747987682174_u64,15900365799239122359_u64];
(*_11) = [(-5504787870262325443_i64),7345311619876406486_i64];
_31.0.0 = 96436381921230724674474184827249516930_i128 + 55800663489379798551182443926702060412_i128;
Goto(bb17)
}
bb17 = {
(*_25) = !(-1120212715_i32);
(*_1) = (-41_i8) as f64;
(*_6) = [3326864902624244435_u64,12453006712134197287_u64,6070993752439206153_u64,16876213087315800918_u64];
(*_1) = 10744542829174354045_u64 as f64;
_39 = !_29;
_31.0.0 = 101400843695033282378335967211701717795_i128 >> (*_19);
(*_19) = _35 << _9;
Goto(bb18)
}
bb18 = {
(*_1) = _17 as f64;
_26 = !_39;
_41 = core::ptr::addr_of!((*_19));
(*_25) = (-1809937458_i32) | (-1092447110_i32);
(*_6) = [18122974291838992530_u64,15805224384703583722_u64,17174622902949354880_u64,697067327711912836_u64];
_40 = -_31.0.0;
(*_1) = _31.0.0 as f64;
(*_1) = _16 as f64;
_23 = Field::<char>(Variant(_3, 0), 1);
_10 = (*_11);
_11 = core::ptr::addr_of_mut!((*_11));
(*_6) = [9248638417647760869_u64,7012452889825269112_u64,8990646699630344761_u64,5487726868515769799_u64];
(*_11) = [113474811282712096_i64,(-2613360478252419891_i64)];
_19 = Move(_38);
(*_11) = [7506421390058832008_i64,(-6168692624147209828_i64)];
(*_41) = _35;
_1 = core::ptr::addr_of!((*_1));
(*_41) = _35 * _35;
_10 = [(-1720484780932290778_i64),5727017785317894986_i64];
(*_41) = _35 - _35;
(*_41) = _35 - _35;
_39 = (*_41) != (*_41);
_34.0.0 = _40 & _40;
(*_11) = [7580613054363117592_i64,6745496953876012187_i64];
(*_6) = [4672471501967585520_u64,16190382834174662429_u64,16282134294374960026_u64,324353216523614339_u64];
(*_6) = [8070260878060482057_u64,11096868835347633503_u64,3904503884457797707_u64,1783138622307350788_u64];
_46 = (*_1);
(*_1) = _34.0.0 as f64;
_42 = 3583879276098462512_i64 as u64;
(*_6) = [_42,_42,_42,_42];
Goto(bb19)
}
bb19 = {
(*_25) = (-863678347_i32) << _40;
_31.0.0 = 53274_u16 as i128;
(*_6) = [_42,_42,_42,_42];
_3 = Adt22::Variant0 { fld0: _9,fld1: _23 };
(*_1) = -_46;
Goto(bb20)
}
bb20 = {
_10 = (*_11);
(*_25) = (-3751132_i32) >> (*_41);
_20 = _16 * _16;
(*_25) = (-1312260504_i32) << _13;
_45 = (*_25) >> (*_41);
(*_1) = _46 * _46;
(*_41) = Field::<char>(Variant(_3, 0), 1) as u8;
(*_6) = [_42,_42,_42,_42];
_15 = [(-12_i8),(-89_i8),81_i8,(-108_i8),112_i8,50_i8,(-63_i8)];
_32 = _39;
Goto(bb21)
}
bb21 = {
_43 = _39 ^ _39;
_9 = _40 as isize;
(*_41) = _35 >> (*_25);
(*_25) = -_45;
(*_6) = [_42,_42,_42,_42];
(*_1) = _46 - _46;
_7 = -_46;
(*_1) = _46 - _46;
(*_6) = [_42,_42,_42,_42];
(*_25) = !_45;
_28 = core::ptr::addr_of!(_6);
(*_11) = [2201333354582299237_i64,7151132419197005881_i64];
_44 = !19666_u16;
(*_28) = core::ptr::addr_of!((*_6));
(*_6) = [_42,_42,_42,_42];
Goto(bb22)
}
bb22 = {
_41 = core::ptr::addr_of!((*_41));
(*_28) = core::ptr::addr_of!((*_6));
_49 = 2_usize as u8;
(*_11) = [(-2759823005742428626_i64),(-8464379036642991015_i64)];
_47 = 250903167658563928785909903034183534268_u128 as i8;
(*_28) = core::ptr::addr_of!((*_6));
_38 = core::ptr::addr_of!((*_41));
_50 = Move(_1);
(*_28) = core::ptr::addr_of!((*_6));
Goto(bb23)
}
bb23 = {
(*_38) = _35;
_26 = _32;
Goto(bb24)
}
bb24 = {
(*_6) = [_42,_42,_42,_42];
_20 = _16 + _16;
(*_11) = _10;
_52.1 = [7798764130328218650997209251143337966_u128,195966375663096760199067006653844558435_u128,59293429689913543153566227093605613725_u128,198168334601664180950759300893783493645_u128,92074160455923078555616406694829526653_u128,281672491549750204893676136867958005670_u128];
(*_25) = _45 ^ _45;
(*_11) = [(-8484972396889074025_i64),446448410963489789_i64];
(*_6) = [_42,_42,_42,_42];
(*_41) = _35 >> (*_25);
Goto(bb25)
}
bb25 = {
(*_28) = core::ptr::addr_of!((*_6));
(*_41) = (*_25) as u8;
_4 = Move(_2);
_27 = _32;
(*_6) = [_42,_42,_42,_42];
_26 = (*_41) < (*_41);
_32 = !_39;
_27 = _39 & _39;
_37 = _13 >> _35;
(*_41) = _35 + _35;
Goto(bb26)
}
bb26 = {
_8 = [228566242533614010_i64,207898945961111807_i64];
_28 = core::ptr::addr_of!((*_28));
(*_25) = _45;
(*_11) = [8941656517002818760_i64,8022909182807365526_i64];
_37 = _13;
(*_11) = [(-5394622293902678339_i64),7245627592052472604_i64];
_3 = Adt22::Variant0 { fld0: _9,fld1: _23 };
(*_6) = [_42,_42,_42,_42];
(*_28) = core::ptr::addr_of!((*_6));
_15 = [_47,_47,_47,_47,_47,_47,_47];
(*_41) = _35;
_56 = _9 | _9;
(*_11) = _10;
_58 = _7 - _7;
(*_6) = [_42,_42,_42,_42];
(*_28) = core::ptr::addr_of!((*_6));
(*_6) = [_42,_42,_42,_42];
(*_41) = 18292057668895041207_usize as u8;
_13 = Field::<isize>(Variant(_3, 0), 0) * _9;
(*_41) = _45 as u8;
Goto(bb27)
}
bb27 = {
(*_25) = 3687981551_u32 as i32;
_38 = core::ptr::addr_of!((*_41));
_57 = _40;
(*_11) = [(-6629179618835355651_i64),4871398412615600853_i64];
_56 = _37 * Field::<isize>(Variant(_3, 0), 0);
(*_41) = _17 as u8;
_5 = [_42,_42,_42,_42];
(*_6) = [_42,_42,_42,_42];
_56 = 1098077321057825026_i64 as isize;
_53 = !_26;
(*_41) = _35 + RET;
(*_11) = _10;
(*_25) = _45;
(*_41) = _35;
(*_25) = _45 ^ _45;
(*_25) = _45;
(*_41) = _57 as u8;
_54 = -_7;
_61.0 = [_23,Field::<char>(Variant(_3, 0), 1),Field::<char>(Variant(_3, 0), 1),Field::<char>(Variant(_3, 0), 1),_23,_23,Field::<char>(Variant(_3, 0), 1)];
Call((*_41) = core::intrinsics::transmute(_29), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_32 = !_26;
(*_6) = [_42,_42,_42,_42];
(*_28) = core::ptr::addr_of!((*_6));
Goto(bb29)
}
bb29 = {
(*_28) = core::ptr::addr_of!((*_6));
(*_28) = core::ptr::addr_of!((*_6));
(*_6) = [_42,_42,_42,_42];
(*_28) = core::ptr::addr_of!((*_6));
(*_28) = core::ptr::addr_of!((*_6));
_51 = Adt48::Variant3 { fld0: _40,fld1: Move(_4),fld2: _44 };
_29 = Field::<isize>(Variant(_3, 0), 0) > _9;
_40 = _34.0.0 ^ _34.0.0;
_12 = _35 << (*_25);
(*_6) = [_42,_42,_42,_42];
(*_25) = (-1956951684065897462_i64) as i32;
(*_25) = -_45;
Goto(bb30)
}
bb30 = {
_47 = (-93_i8) ^ 112_i8;
(*_6) = [_42,_42,_42,_42];
(*_25) = _34.0.0 as i32;
(*_6) = [_42,_42,_42,_42];
(*_11) = _10;
_52.1 = [302571865565352767490456753222462835251_u128,81482288165017085969340820135272752570_u128,317417979992965192284850168090186048045_u128,159225604442559471344046077816351881894_u128,215176926663573766909391427477572345365_u128,229478748481258732455166472810055313610_u128];
(*_11) = [(-7540870409424106977_i64),(-5117606639780890753_i64)];
place!(Field::<char>(Variant(_3, 0), 1)) = _23;
_62 = 329831054194421733783539716770626220675_u128 ^ 37000241031997544176030662528785375424_u128;
_59 = 3842129942_u32 as isize;
Goto(bb31)
}
bb31 = {
_61.3 = Adt22::Variant0 { fld0: _37,fld1: _23 };
(*_6) = [_42,_42,_42,_42];
(*_41) = RET;
(*_25) = _45;
(*_6) = [_42,_42,_42,_42];
(*_41) = _35 << (*_25);
(*_41) = !_35;
(*_11) = [(-1139032739295996977_i64),7500164790545719649_i64];
(*_28) = core::ptr::addr_of!((*_6));
(*_6) = [_42,_42,_42,_42];
_18 = [_17,_17,_17];
_52.2 = _23;
Goto(bb32)
}
bb32 = {
(*_28) = core::ptr::addr_of!((*_6));
_44 = !Field::<u16>(Variant(_51, 3), 2);
(*_6) = [_42,_42,_42,_42];
(*_6) = [_42,_42,_42,_42];
(*_41) = _17 as u8;
place!(Field::<u16>(Variant(_51, 3), 2)) = _44 - _44;
_65 = _7;
(*_28) = core::ptr::addr_of!((*_6));
(*_11) = _10;
_57 = Field::<i128>(Variant(_51, 3), 0) + _40;
Goto(bb33)
}
bb33 = {
_4 = Move(Field::<*mut *const usize>(Variant(_51, 3), 1));
_62 = !240299526474879349500328411103466947803_u128;
(*_28) = core::ptr::addr_of!((*_6));
_31.0.1 = &mut _62;
(*_11) = _10;
_26 = _29 ^ _53;
(*_25) = _26 as i32;
(*_41) = _35 >> (*_25);
(*_25) = !_45;
_49 = _34.0.0 as u8;
_2 = Move(_4);
_55 = _20 + _16;
_37 = _9 ^ _13;
place!(Field::<u16>(Variant(_51, 3), 2)) = _44 >> (*_25);
(*_25) = _45;
_28 = core::ptr::addr_of!((*_28));
_7 = _54;
_1 = core::ptr::addr_of!(_58);
Goto(bb34)
}
bb34 = {
(*_11) = _10;
Goto(bb35)
}
bb35 = {
(*_6) = [_42,_42,_42,_42];
(*_11) = _10;
(*_1) = _54 * _65;
(*_6) = [_42,_42,_42,_42];
(*_11) = _10;
_52.2 = _23;
_33 = 2397219907151882585_i64;
_7 = -(*_1);
(*_28) = core::ptr::addr_of!((*_6));
_50 = core::ptr::addr_of!((*_1));
(*_50) = -_46;
(*_41) = (*_1) as u8;
(*_41) = !_35;
_34.0 = (_57, Move(_31.0.1));
place!(Field::<*mut *const usize>(Variant(_51, 3), 1)) = core::ptr::addr_of_mut!(_61.1.1);
_58 = _54 + _7;
place!(Field::<u16>(Variant(_51, 3), 2)) = _44 + _44;
(*_50) = 240990118568694520695643034902733959761_u128 as f64;
_40 = (*_41) as i128;
Goto(bb36)
}
bb36 = {
(*_6) = [_42,_42,_42,_42];
_31.0.0 = Field::<i128>(Variant(_51, 3), 0);
(*_41) = _35 - _49;
_76 = [80994066442740840827173386442812438216_u128,17928363354721278935716579630592830152_u128,337263064479057386412568530321159281596_u128,118347179627321987861837318281842138990_u128,52454886429446778508816961311562964590_u128,117817363321981393026219155252126936907_u128];
(*_11) = [_33,_33];
(*_41) = Field::<u16>(Variant(_51, 3), 2) as u8;
(*_50) = _54;
(*_11) = _10;
(*_1) = _65 + _46;
(*_28) = core::ptr::addr_of!((*_6));
_11 = core::ptr::addr_of_mut!((*_11));
(*_1) = _54;
(*_1) = -_7;
(*_1) = _54;
_43 = (*_25) >= (*_25);
(*_41) = (*_1) as u8;
_70 = (*_1) + (*_1);
_76 = [32716437163286748469265109602302593836_u128,24439241115454797652701252076341752763_u128,137907829791076469862366312256964202378_u128,156524417906409589181876603799357475449_u128,273223599243200967701370454981404603773_u128,14697851767729661415755452053039344224_u128];
_37 = _9;
Call(_69 = core::intrinsics::transmute((*_41)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
(*_1) = _65 * _54;
(*_6) = [_42,_42,_42,_42];
(*_11) = [_33,_33];
_79 = _33 as u32;
(*_6) = [_42,_42,_42,_42];
(*_41) = _33 as u8;
_61.2 = [_69,_69,_69,_69,_69,_47,_69];
_79 = 1856448745_u32;
(*_6) = [_42,_42,_42,_42];
_28 = core::ptr::addr_of!((*_28));
(*_1) = _49 as f64;
match _33 {
0 => bb23,
1 => bb32,
2 => bb6,
3 => bb5,
4 => bb38,
5 => bb39,
2397219907151882585 => bb41,
_ => bb40
}
}
bb38 = {
_20 = _16 - _16;
(*_6) = [11244667242832275653_u64,10677989737361031728_u64,2123880651084120785_u64,11717840945098317172_u64];
(*_19) = !RET;
_16 = _20;
(*_19) = RET ^ RET;
(*_19) = RET;
(*_19) = 15573860045411108239_u64 as u8;
(*_6) = [14675859733730561124_u64,4883453119792261478_u64,12391449543310906558_u64,14537937502143853292_u64];
(*_19) = RET >> _13;
(*_11) = _10;
(*_11) = _10;
Goto(bb7)
}
bb39 = {
(*_6) = [5295658618944503827_u64,11392894462626117727_u64,4733745057732880179_u64,9109944856945604700_u64];
(*_6) = [2090014353455446461_u64,1817812643532732665_u64,3123521315286408333_u64,10380719714809356098_u64];
(*_38) = !_35;
(*_11) = [(-4735747871141153233_i64),(-5156890086207360383_i64)];
(*_38) = _35;
(*_25) = (-807155954_i32) & (-1766473427_i32);
(*_19) = _35 & _35;
(*_6) = [9399296900964429957_u64,11381003086292903236_u64,2982134747987682174_u64,15900365799239122359_u64];
(*_11) = [(-5504787870262325443_i64),7345311619876406486_i64];
_31.0.0 = 96436381921230724674474184827249516930_i128 + 55800663489379798551182443926702060412_i128;
Goto(bb17)
}
bb40 = {
_10 = (*_11);
(*_25) = (-3751132_i32) >> (*_41);
_20 = _16 * _16;
(*_25) = (-1312260504_i32) << _13;
_45 = (*_25) >> (*_41);
(*_1) = _46 * _46;
(*_41) = Field::<char>(Variant(_3, 0), 1) as u8;
(*_6) = [_42,_42,_42,_42];
_15 = [(-12_i8),(-89_i8),81_i8,(-108_i8),112_i8,50_i8,(-63_i8)];
_32 = _39;
Goto(bb21)
}
bb41 = {
(*_41) = !_35;
(*_11) = [_33,_33];
_45 = -(*_25);
_27 = (*_41) == (*_41);
_60 = &_79;
_72 = Field::<char>(Variant(_61.3, 0), 1);
_59 = _72 as isize;
_82 = Field::<char>(Variant(_61.3, 0), 1) as i32;
(*_25) = _45 & _45;
_51 = Adt48::Variant3 { fld0: _31.0.0,fld1: Move(_2),fld2: _44 };
(*_28) = core::ptr::addr_of!((*_6));
(*_28) = core::ptr::addr_of!((*_6));
_9 = Field::<isize>(Variant(_61.3, 0), 0);
_71 = Adt28::Variant0 { fld0: (*_1),fld1: Field::<char>(Variant(_3, 0), 1),fld2: Move(_61.3),fld3: _69,fld4: _34.0.0,fld5: _20,fld6: _18 };
(*_28) = core::ptr::addr_of!((*_6));
(*_11) = _10;
(*_41) = _49 - _35;
(*_6) = [_42,_42,_42,_42];
_38 = Move(_41);
(*_28) = core::ptr::addr_of!((*_6));
_68 = _44 | _44;
(*_11) = _10;
_85 = [227856655223376520841227516922342755720_u128,229576591778021379074093611034510036829_u128,302175069515596798319192809770843594526_u128,237370207075072475889825470039282322109_u128,133065015100162921900402078153992666610_u128,303622463429873559595851081991092055294_u128];
(*_1) = Field::<f64>(Variant(_71, 0), 0) - _7;
match (*_60) {
0 => bb31,
1 => bb16,
2 => bb37,
3 => bb33,
4 => bb10,
1856448745 => bb42,
_ => bb17
}
}
bb42 = {
(*_11) = [_33,_33];
RET = _35 ^ _12;
_16 = _55 * _20;
(*_28) = core::ptr::addr_of!((*_6));
_80 = core::ptr::addr_of!(_49);
(*_28) = core::ptr::addr_of!((*_6));
(*_11) = [_33,_33];
_49 = RET;
_4 = core::ptr::addr_of_mut!(_61.1.1);
place!(Field::<*mut *const usize>(Variant(_51, 3), 1)) = core::ptr::addr_of_mut!((*_4));
_67.0.0 = Field::<i128>(Variant(_51, 3), 0) >> (*_25);
_75 = Field::<i8>(Variant(_71, 0), 3);
(*_28) = core::ptr::addr_of!((*_6));
(*_28) = core::ptr::addr_of!((*_6));
_29 = !_53;
(*_1) = 42462211415755392184096855512336260123_u128 as f64;
_73 = _16 * _16;
_26 = (*_25) >= (*_25);
_6 = core::ptr::addr_of!((*_6));
(*_11) = [_33,_33];
(*_11) = _10;
Goto(bb43)
}
bb43 = {
Call(_90 = dump_var(Move(_27), Move(_49), Move(_72), Move(_47)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_90 = dump_var(Move(_85), Move(_53), Move(_18), Move(_33)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_90 = dump_var(Move(_37), Move(_35), Move(_82), Move(_22)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_90 = dump_var(Move(_79), Move(_12), Move(_10), Move(_5)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_90 = dump_var(Move(_69), Move(_45), Move(_29), _91), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(590053552_i32), std::hint::black_box('\u{9a0ae}'), std::hint::black_box(12_u8));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt22 {
Variant0{
fld0: isize,
fld1: char,

},
Variant1{
fld0: *const usize,
fld1: usize,

}}
#[derive(Debug)]
pub enum Adt25 {
Variant0{
fld0: i16,
fld1: *const usize,

},
Variant1{
fld0: [i64; 2],
fld1: char,
fld2: [i16; 3],
fld3: Adt22,
fld4: u16,
fld5: u8,
fld6: usize,
fld7: f64,

},
Variant2{
fld0: u8,

}}
#[derive(Debug)]
pub enum Adt28 {
Variant0{
fld0: f64,
fld1: char,
fld2: Adt22,
fld3: i8,
fld4: i128,
fld5: f32,
fld6: [i16; 3],

},
Variant1{
fld0: u32,
fld1: char,
fld2: (u16, i32, char, u128),

}}
#[derive(Debug)]
pub enum Adt38 {
Variant0{
fld0: bool,
fld1: [u16; 4],
fld2: u64,
fld3: *mut [i64; 2],
fld4: [u128; 6],

},
Variant1{
fld0: u8,
fld1: char,
fld2: [u128; 6],
fld3: Adt22,
fld4: u64,
fld5: i128,

},
Variant2{
fld0: [u128; 6],
fld1: f64,
fld2: u16,
fld3: *const usize,
fld4: u32,

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: i128,
fld1: char,
fld2: u64,
fld3: i8,

},
Variant1{
fld0: (u8, usize, isize),
fld1: [u128; 6],
fld2: Adt25,
fld3: i8,
fld4: Adt22,
fld5: u128,

},
Variant2{
fld0: [isize; 5],
fld1: i16,
fld2: i128,
fld3: usize,

},
Variant3{
fld0: i128,
fld1: *mut *const usize,
fld2: u16,

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: [i128; 4],
fld1: u64,
fld2: i32,

},
Variant1{
fld0: [i16; 3],

}}
#[derive(Debug)]
pub struct Adt59 {
fld0: *mut [i64; 2],
fld1: (u16, i32, char, u128),
fld2: [char; 7],
fld3: (i32, *const usize),
fld4: u16,
fld5: [u16; 4],
fld6: [u128; 5],
}
#[derive(Debug)]
pub struct Adt73 {
fld0: Adt48,
fld1: Adt28,
fld2: [i32; 2],
fld3: [char; 7],
fld4: f32,
fld5: i32,
}

