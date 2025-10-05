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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u32) -> isize {
mir! {
type RET = isize;
let _8: isize;
let _9: u16;
let _10: *const i128;
let _11: i8;
let _12: &'static ((*const &'static mut [i128; 2], Adt47), [i16; 5], &'static mut *const isize, Adt33);
let _13: u128;
let _14: [i64; 5];
let _15: isize;
let _16: isize;
let _17: ([i128; 2], bool, &'static mut u32);
let _18: ();
let _19: ();
{
_4 = (-73290732517866802890648463732401314388_i128) as i8;
_2 = '\u{10cdb0}';
_1 = _2 < _2;
RET = 46512_u16 as isize;
_3 = 125_u8 as isize;
_7 = 3293947930_u32;
_3 = !RET;
_1 = false;
_8 = -RET;
_7 = 2447688415_u32 & 614131043_u32;
_5 = (-22387_i16) >> _7;
_5 = (-27904_i16);
_11 = _4 << RET;
_4 = _11 | _11;
_3 = 19961_u16 as isize;
_9 = !23043_u16;
_7 = _2 as u32;
_8 = _3 & RET;
_5 = _9 as i16;
_3 = _8 ^ _8;
_6 = !733200173_i32;
_2 = '\u{d2998}';
_11 = _4 - _4;
_5 = 14598_i16 + (-1572_i16);
Goto(bb1)
}
bb1 = {
_6 = (-1426861782_i32) - 1339777011_i32;
_2 = '\u{8eb84}';
_8 = !RET;
_14 = [(-7042130152096581121_i64),4900357470192253313_i64,(-5802911294067535017_i64),3178300161792576654_i64,(-8005253249426544227_i64)];
_9 = !65104_u16;
_13 = 273030331213070126544369178903849448593_u128;
_8 = -_3;
_8 = !_3;
RET = _3;
_5 = _1 as i16;
RET = _3 & _3;
_14 = [7800030990171717397_i64,5993095810466925218_i64,(-609803393003240476_i64),(-7102122654043397347_i64),(-2898544102077998254_i64)];
_14 = [(-968882208988299128_i64),(-5782928523265715375_i64),448950921341131525_i64,211761925952916113_i64,(-3383589568309010564_i64)];
_9 = 31_u8 as u16;
_2 = '\u{104b25}';
_3 = RET << _8;
RET = !_3;
_4 = _9 as i8;
_6 = !(-243633809_i32);
_1 = true;
_15 = RET & _3;
_13 = 222787375071686684269073467883931259227_u128 & 98871585086176788059640375254350147389_u128;
_15 = !RET;
_4 = _1 as i8;
_7 = 2796463073_u32;
_15 = 28744668232984459770517683646698689860_i128 as isize;
_11 = _6 as i8;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(Move(_8), Move(_5), Move(_11), Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(Move(_6), Move(_4), _19, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{7cb4d}'), std::hint::black_box(125_isize), std::hint::black_box((-115_i8)), std::hint::black_box((-20493_i16)), std::hint::black_box((-751602523_i32)), std::hint::black_box(460001696_u32));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub struct Adt33 {
fld0: u8,
fld1: i8,
}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: usize,
fld1: char,
fld2: isize,
fld3: *const [i16; 5],
fld4: [i32; 5],
fld5: [u8; 2],
fld6: i64,

},
Variant1{
fld0: *mut usize,
fld1: [i16; 5],
fld2: isize,
fld3: i8,
fld4: *const *const [i16; 5],
fld5: u64,
fld6: *const i64,

},
Variant2{
fld0: usize,
fld1: [i128; 2],
fld2: [usize; 6],
fld3: [i16; 5],
fld4: *const isize,
fld5: ([u8; 2], i16, i64, bool),
fld6: i64,

},
Variant3{
fld0: bool,
fld1: [u64; 5],
fld2: [usize; 6],
fld3: f64,
fld4: Adt33,
fld5: i32,
fld6: *const i64,
fld7: i128,

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: *const [i16; 5],
fld1: (f64, i16, [i32; 5], u16),
fld2: f32,
fld3: [i32; 5],
fld4: u8,
fld5: (u64, i8),

},
Variant1{
fld0: bool,
fld1: *const isize,
fld2: [usize; 6],
fld3: ((f64, i16, [i32; 5], u16),),
fld4: i16,
fld5: [i128; 2],
fld6: [u64; 5],

},
Variant2{
fld0: i128,
fld1: [i32; 5],
fld2: isize,
fld3: *const i64,
fld4: u8,
fld5: i32,
fld6: [u64; 5],

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: (f64, i16, [i32; 5], u16),
fld1: char,
fld2: u128,
fld3: u32,
fld4: [u128; 3],
fld5: f64,

},
Variant1{
fld0: [usize; 6],
fld1: *const isize,
fld2: [u8; 2],
fld3: Adt33,

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: *mut *const [i16; 5],
fld1: u64,
fld2: *mut usize,
fld3: [i16; 5],
fld4: i16,
fld5: f32,
fld6: *mut u64,
fld7: u32,

},
Variant1{
fld0: [i16; 5],
fld1: [u128; 3],
fld2: ((f64, i16, [i32; 5], u16),),
fld3: f32,
fld4: i16,
fld5: i32,
fld6: u64,

},
Variant2{
fld0: isize,
fld1: u128,

}}
#[derive(Debug)]
pub struct Adt68 {
fld0: bool,
fld1: [i16; 5],
fld2: *const i8,
fld3: [u128; 3],
fld4: *const isize,
fld5: ((f64, i16, [i32; 5], u16),),
fld6: u8,
fld7: i128,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt70 {
fld0: [i128; 2],
fld1: isize,
}
#[derive(Debug)]
pub enum Adt74 {
Variant0{
fld0: [usize; 6],

},
Variant1{
fld0: *mut Adt51,
fld1: (*const *const [i16; 5],),
fld2: [isize; 2],
fld3: *mut usize,

},
Variant2{
fld0: bool,
fld1: *const u8,
fld2: *const i8,

}}

