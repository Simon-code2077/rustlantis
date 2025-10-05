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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: u64,mut _5: i16,mut _6: i32,mut _7: u128,mut _8: i128,mut _9: u16) -> f32 {
mir! {
type RET = f32;
let _10: f32;
let _11: *mut char;
let _12: isize;
let _13: Adt70;
let _14: f64;
let _15: i128;
let _16: (*mut *mut u16, Adt50, Adt50, (char, i128, Adt27, f32));
let _17: &'static mut u8;
let _18: (&'static usize, *const i32, i16, &'static isize);
let _19: &'static bool;
let _20: (*const i32, &'static (i128, *mut (i128, (i128,)), [i128; 5], u64), Adt43);
let _21: (char, i128, Adt27, f32);
let _22: (*mut *mut u16, Adt50, Adt50, (char, i128, Adt27, f32));
let _23: isize;
let _24: f64;
let _25: isize;
let _26: *mut u16;
let _27: Adt50;
let _28: ();
let _29: ();
{
_2 = '\u{59a3f}';
_7 = 318507204019137075878392993687817199960_u128 >> (-95_i8);
_7 = 228787883278881357655793755050376816_u128 | 324044328062505967000057568113716595704_u128;
_6 = (-2000879020_i32) << _7;
RET = 11283751646411050179_u64 as f32;
_4 = !2073683433693179028_u64;
_8 = (-28944654272848342538710487899328526957_i128) >> _6;
_5 = !3829_i16;
_4 = 3354698212376857366_u64;
_12 = 9223372036854775807_isize;
_9 = 46764_u16;
_5 = _7 as i16;
_2 = '\u{4b46f}';
_9 = _8 as u16;
_2 = '\u{37ccd}';
_9 = 35898_u16 * 2089_u16;
_15 = _8 - _8;
_2 = '\u{1ad3d}';
Goto(bb1)
}
bb1 = {
_11 = core::ptr::addr_of_mut!(_16.3.0);
_9 = 18642_u16 + 3585_u16;
_5 = -4673_i16;
(*_11) = _2;
_1 = true;
(*_11) = _2;
_16.3.1 = RET as i128;
_19 = &_1;
_18.3 = &_12;
_16.3.1 = RET as i128;
_18.2 = _5 ^ _5;
_9 = 30439_u16 ^ 30537_u16;
(*_11) = _2;
_6 = 1631756453_i32 - (-1242187009_i32);
(*_11) = _2;
_16.3.3 = RET - RET;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9223372036854775807 => bb9,
_ => bb8
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
(*_11) = _2;
_9 = 20074_u16 - 39937_u16;
(*_11) = _2;
(*_11) = _2;
(*_11) = _2;
(*_11) = _2;
_10 = _16.3.3 - _16.3.3;
(*_11) = _2;
Goto(bb10)
}
bb10 = {
(*_11) = _2;
_16.3.3 = _9 as f32;
(*_11) = _2;
_18.2 = _5 * _5;
_16.3.0 = _2;
_21.0 = (*_11);
_22.3.3 = _16.3.3;
(*_11) = _2;
_16.3.3 = _22.3.3 * RET;
_7 = 270606344441259713725457028391717811749_u128;
(*_11) = _2;
_2 = (*_11);
_3 = !_12;
_3 = !_12;
(*_11) = _2;
_4 = 0_usize as u64;
_8 = _15;
(*_11) = _2;
match _12 {
0 => bb11,
1 => bb12,
2 => bb13,
9223372036854775807 => bb15,
_ => bb14
}
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
_11 = core::ptr::addr_of_mut!(_16.3.0);
_9 = 18642_u16 + 3585_u16;
_5 = -4673_i16;
(*_11) = _2;
_1 = true;
(*_11) = _2;
_16.3.1 = RET as i128;
_19 = &_1;
_18.3 = &_12;
_16.3.1 = RET as i128;
_18.2 = _5 ^ _5;
_9 = 30439_u16 ^ 30537_u16;
(*_11) = _2;
_6 = 1631756453_i32 - (-1242187009_i32);
(*_11) = _2;
_16.3.3 = RET - RET;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb15 = {
_21.1 = _15 ^ _8;
(*_11) = _2;
match _12 {
0 => bb3,
1 => bb11,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
9223372036854775807 => bb21,
_ => bb20
}
}
bb16 = {
Return()
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
(*_11) = _2;
_16.3.3 = _9 as f32;
(*_11) = _2;
_18.2 = _5 * _5;
_16.3.0 = _2;
_21.0 = (*_11);
_22.3.3 = _16.3.3;
(*_11) = _2;
_16.3.3 = _22.3.3 * RET;
_7 = 270606344441259713725457028391717811749_u128;
(*_11) = _2;
_2 = (*_11);
_3 = !_12;
_3 = !_12;
(*_11) = _2;
_4 = 0_usize as u64;
_8 = _15;
(*_11) = _2;
match _12 {
0 => bb11,
1 => bb12,
2 => bb13,
9223372036854775807 => bb15,
_ => bb14
}
}
bb21 = {
(*_11) = _21.0;
_1 = !false;
(*_11) = _2;
_21.1 = _8 - _15;
_12 = _3;
_21.1 = _15 << _15;
(*_11) = _21.0;
(*_11) = _2;
_21.3 = _10 + _16.3.3;
_4 = 16008351721656171746_u64;
(*_11) = _21.0;
_20.0 = core::ptr::addr_of!(_6);
RET = _21.3 - _21.3;
_16.3.1 = _10 as i128;
(*_11) = _21.0;
_18.1 = Move(_20.0);
_12 = _3 + _3;
Goto(bb22)
}
bb22 = {
Call(_28 = dump_var(Move(_9), Move(_5), Move(_8), Move(_4)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_28 = dump_var(Move(_1), _29, _29, _29), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{9c30c}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(16590976658359436110_u64), std::hint::black_box(30659_i16), std::hint::black_box(161385270_i32), std::hint::black_box(167597514634780908130649882210097028263_u128), std::hint::black_box(1775712273915831078534912992864823796_i128), std::hint::black_box(27851_u16));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt27 {
Variant0{
fld0: bool,
fld1: char,
fld2: [usize; 6],
fld3: (i128,),
fld4: i128,

},
Variant1{
fld0: *mut f64,
fld1: f64,
fld2: usize,
fld3: u32,
fld4: i16,
fld5: i32,

},
Variant2{
fld0: (i128,),
fld1: u64,
fld2: isize,
fld3: i8,
fld4: [usize; 6],
fld5: u16,
fld6: i64,

}}
#[derive(Debug)]
pub struct Adt40 {
fld0: *mut u8,
fld1: u8,
fld2: [i128; 5],
fld3: f32,
fld4: [i16; 4],
fld5: *mut u16,
fld6: i64,
fld7: i128,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt43 {
Variant0{
fld0: (i128, (i128,)),
fld1: u128,
fld2: u8,
fld3: i8,
fld4: f64,

},
Variant1{
fld0: bool,
fld1: u128,
fld2: (i128,),
fld3: [char; 5],

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: [usize; 3],
fld1: u64,

},
Variant1{
fld0: f32,
fld1: u32,
fld2: Adt40,
fld3: i8,
fld4: *mut char,
fld5: i32,
fld6: i64,
fld7: *mut i64,

}}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: bool,
fld1: i64,
fld2: [i128; 5],
fld3: f64,
fld4: u8,

},
Variant1{
fld0: (u64, (i128,), u128, [usize; 6]),
fld1: *mut u16,
fld2: Adt43,
fld3: *mut u8,

},
Variant2{
fld0: bool,
fld1: *mut (i128, (i128,)),
fld2: *mut u8,
fld3: u128,
fld4: u8,
fld5: i32,
fld6: i64,
fld7: f64,

},
Variant3{
fld0: i8,

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: [usize; 4],
fld1: *const i32,
fld2: isize,
fld3: u16,

},
Variant1{
fld0: (i128,),
fld1: u8,
fld2: i128,
fld3: *const u128,

},
Variant2{
fld0: Adt50,
fld1: *mut (i128, (i128,)),
fld2: f64,
fld3: [char; 5],
fld4: u32,
fld5: Adt27,
fld6: (*mut *mut u16, Adt50, Adt50, (char, i128, Adt27, f32)),

},
Variant3{
fld0: bool,
fld1: [usize; 4],
fld2: *const u128,
fld3: *mut u8,
fld4: Adt27,
fld5: *mut char,

}}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: *const u128,
fld1: *const i32,
fld2: isize,
fld3: usize,
fld4: *mut *mut u16,
fld5: *mut (i128, (i128,)),

},
Variant1{
fld0: (i128, *mut (i128, (i128,)), [i128; 5], u64),
fld1: u8,

}}
#[derive(Debug)]
pub enum Adt70 {
Variant0{
fld0: (i128, (i128,)),
fld1: i64,
fld2: [usize; 4],

},
Variant1{
fld0: u16,
fld1: Adt56,
fld2: Adt50,
fld3: u8,
fld4: [u8; 7],

},
Variant2{
fld0: i128,
fld1: [usize; 3],
fld2: isize,

}}

