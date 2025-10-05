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
pub fn fn0(mut _1: u64,mut _2: i64) -> Adt18 {
mir! {
type RET = Adt18;
let _3: bool;
let _4: *mut isize;
let _5: f32;
let _6: u16;
let _7: isize;
let _8: char;
let _9: i8;
let _10: i128;
let _11: &'static *mut i8;
let _12: &'static mut ([u16; 8], f64, *mut isize);
let _13: &'static mut Adt29;
let _14: ();
let _15: ();
{
_2 = 1506350900958029685_i64;
_1 = 17665212525238547671_u64;
_1 = 9855661149850747041_u64 - 1353349237672718480_u64;
RET = Adt18::Variant1 { fld0: false,fld1: 295015734536097496497923515781269595452_u128,fld2: (-9223372036854775808_isize),fld3: (-110_i8),fld4: _1,fld5: 79_u8,fld6: _2,fld7: 14099_u16 };
place!(Field::<i8>(Variant(RET, 1), 3)) = 74_i8 - (-43_i8);
place!(Field::<i64>(Variant(RET, 1), 6)) = _2;
place!(Field::<u16>(Variant(RET, 1), 7)) = 50850_u16;
place!(Field::<u8>(Variant(RET, 1), 5)) = (-100760186117217420405151459311855731067_i128) as u8;
_3 = Field::<u64>(Variant(RET, 1), 4) > _1;
place!(Field::<isize>(Variant(RET, 1), 2)) = !(-9223372036854775808_isize);
RET = Adt18::Variant1 { fld0: _3,fld1: 123010635549917467246731739143385442509_u128,fld2: (-9223372036854775808_isize),fld3: 31_i8,fld4: _1,fld5: 163_u8,fld6: _2,fld7: 27182_u16 };
Call(place!(Field::<u64>(Variant(RET, 1), 4)) = core::intrinsics::transmute(Field::<i64>(Variant(RET, 1), 6)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<u128>(Variant(RET, 1), 1)) = 8790975498256285425889746762774978036_u128 ^ 120135849178582113357571913906449495087_u128;
_1 = Field::<u64>(Variant(RET, 1), 4) | Field::<u64>(Variant(RET, 1), 4);
_4 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
place!(Field::<bool>(Variant(RET, 1), 0)) = _3;
(*_4) = (-22_isize) * (-9223372036854775808_isize);
(*_4) = 2_isize * (-127_isize);
(*_4) = 9223372036854775807_isize << _2;
(*_4) = 89_isize | 38_isize;
match Field::<i64>(Variant(RET, 1), 6) {
0 => bb2,
1 => bb3,
1506350900958029685 => bb5,
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
place!(Field::<u16>(Variant(RET, 1), 7)) = !25354_u16;
_6 = !Field::<u16>(Variant(RET, 1), 7);
(*_4) = -(-15_isize);
(*_4) = 9223372036854775807_isize * 9223372036854775807_isize;
(*_4) = (-9223372036854775808_isize);
(*_4) = 9223372036854775807_isize - (-95_isize);
(*_4) = -9223372036854775807_isize;
(*_4) = !(-9223372036854775808_isize);
(*_4) = -(-9223372036854775808_isize);
(*_4) = 114_isize + 9223372036854775807_isize;
(*_4) = (-3965_i16) as isize;
_7 = (*_4) >> (*_4);
(*_4) = !_7;
(*_4) = _3 as isize;
_1 = Field::<u64>(Variant(RET, 1), 4);
place!(Field::<i8>(Variant(RET, 1), 3)) = (-22_i8) | 63_i8;
place!(Field::<bool>(Variant(RET, 1), 0)) = _3 | _3;
place!(Field::<u64>(Variant(RET, 1), 4)) = !_1;
Call(_4 = fn1((*_4), (*_4), _2, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<u8>(Variant(RET, 1), 5)) = !195_u8;
place!(Field::<u128>(Variant(RET, 1), 1)) = 124199317256497300383121595333701770214_u128;
_6 = 17512064233504758975_usize as u16;
place!(Field::<i64>(Variant(RET, 1), 6)) = (-2107234765_i32) as i64;
_9 = _3 as i8;
Goto(bb7)
}
bb7 = {
Call(_14 = dump_var(Move(_1), Move(_3), Move(_2), _15), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: isize,mut _3: i64,mut _4: u64) -> *mut isize {
mir! {
type RET = *mut isize;
let _5: bool;
let _6: bool;
let _7: [i32; 3];
let _8: f64;
let _9: u32;
let _10: isize;
let _11: &'static mut Adt23;
let _12: (&'static f32,);
let _13: bool;
let _14: Adt17;
let _15: &'static mut Adt23;
let _16: i16;
let _17: &'static mut *mut (i32, f64);
let _18: *const u8;
let _19: *mut isize;
let _20: u8;
let _21: ();
let _22: ();
{
_3 = -(-6242836882433229351_i64);
RET = core::ptr::addr_of_mut!(_2);
(*RET) = _1 * _1;
_4 = 1557834517_i32 as u64;
(*RET) = !_1;
(*RET) = 2_usize as isize;
_1 = 37379721657947369531808357242410802439_i128 as isize;
(*RET) = !_1;
(*RET) = _1;
(*RET) = -_1;
_1 = 1985134503_u32 as isize;
(*RET) = _1;
(*RET) = _1 * _1;
_2 = _1 ^ _1;
_5 = false;
(*RET) = _1 - _1;
(*RET) = _1 << _1;
(*RET) = -_1;
(*RET) = !_1;
(*RET) = !_1;
(*RET) = 229761016058208351560161163157312569518_u128 as isize;
(*RET) = _1;
(*RET) = _1;
Call((*RET) = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*RET) = _1 | _1;
(*RET) = _1 >> _1;
(*RET) = _1 >> _4;
(*RET) = _1 << _4;
(*RET) = _1 + _1;
_8 = (-30427_i16) as f64;
_5 = (*RET) >= (*RET);
(*RET) = _1 - _1;
_9 = _5 as u32;
(*RET) = _1;
(*RET) = -_1;
Goto(bb2)
}
bb2 = {
(*RET) = _1 + _1;
(*RET) = !_1;
(*RET) = _1 >> _9;
(*RET) = -_1;
(*RET) = !_1;
(*RET) = _1 ^ _1;
(*RET) = _1 - _1;
_1 = (*RET);
_6 = _5 & _5;
(*RET) = _1 - _1;
(*RET) = '\u{108301}' as isize;
(*RET) = _1 >> _4;
(*RET) = _1 * _1;
_8 = (*RET) as f64;
(*RET) = _1 >> _1;
(*RET) = 0_usize as isize;
(*RET) = !_1;
_13 = _5;
_6 = _13 ^ _5;
(*RET) = _1 + _1;
_3 = (-2892145614130212261_i64);
(*RET) = _1 - _1;
(*RET) = 1989822525472334491125600775271850690_i128 as isize;
(*RET) = (-117139620571264344303540181601358517423_i128) as isize;
_4 = 11305005169960966786_u64 + 16130865069747025500_u64;
(*RET) = 226981001170820630982801343143294765758_u128 as isize;
(*RET) = _8 as isize;
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463460482461817637999195 => bb8,
_ => bb7
}
}
bb3 = {
(*RET) = _1 | _1;
(*RET) = _1 >> _1;
(*RET) = _1 >> _4;
(*RET) = _1 << _4;
(*RET) = _1 + _1;
_8 = (-30427_i16) as f64;
_5 = (*RET) >= (*RET);
(*RET) = _1 - _1;
_9 = _5 as u32;
(*RET) = _1;
(*RET) = -_1;
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
(*RET) = _3 as isize;
(*RET) = _1;
_8 = (-13975_i16) as f64;
(*RET) = 24260_i16 as isize;
(*RET) = _1 << _3;
(*RET) = _6 as isize;
(*RET) = (-589998858_i32) as isize;
(*RET) = _1 | _1;
(*RET) = _1 * _1;
(*RET) = _1;
(*RET) = -_1;
(*RET) = _1 * _1;
_16 = (-15855_i16) * 30150_i16;
(*RET) = _1 << _4;
_16 = 12796_i16 & 22780_i16;
_7 = [1742842434_i32,(-1332787513_i32),1979552864_i32];
(*RET) = _1;
(*RET) = _1 >> _1;
_2 = 1372748888_i32 as isize;
(*RET) = _13 as isize;
(*RET) = _1 ^ _1;
_5 = _6 ^ _13;
Goto(bb9)
}
bb9 = {
_10 = (*RET) ^ _2;
_5 = !_13;
(*RET) = _10 * _10;
_9 = 3884352842_u32 + 246176831_u32;
_4 = 5700145016429145163_u64 ^ 2482439256797667605_u64;
_14 = Adt17::Variant0 { fld0: 149605108996832628655437311384816767164_u128,fld1: 55_u8 };
(*RET) = _1 + _10;
_19 = core::ptr::addr_of_mut!(_1);
(*RET) = _10 + (*_19);
(*_19) = (*RET);
(*RET) = (*_19);
(*RET) = !(*_19);
(*RET) = (*_19) ^ (*_19);
_14 = Adt17::Variant1 { fld0: _8,fld1: 1_usize,fld2: (*RET),fld3: 98_i8,fld4: _16,fld5: 42_u8,fld6: _4 };
(*_19) = (*RET) & (*RET);
(*RET) = (*_19);
RET = core::ptr::addr_of_mut!((*RET));
Goto(bb10)
}
bb10 = {
Call(_21 = dump_var(Move(_6), Move(_2), Move(_13), Move(_3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_21 = dump_var(Move(_4), _22, _22, _22), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(15205771837399620951_u64), std::hint::black_box(7553699978678057309_i64));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt17 {
Variant0{
fld0: u128,
fld1: u8,

},
Variant1{
fld0: f64,
fld1: usize,
fld2: isize,
fld3: i8,
fld4: i16,
fld5: u8,
fld6: u64,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt18 {
Variant0{
fld0: u64,
fld1: f64,
fld2: u32,
fld3: i8,
fld4: i16,
fld5: i32,

},
Variant1{
fld0: bool,
fld1: u128,
fld2: isize,
fld3: i8,
fld4: u64,
fld5: u8,
fld6: i64,
fld7: u16,

},
Variant2{
fld0: f32,
fld1: i128,

},
Variant3{
fld0: u32,
fld1: i16,
fld2: u64,

}}
#[derive(Debug)]
pub enum Adt23 {
Variant0{
fld0: Adt17,
fld1: u128,
fld2: isize,
fld3: (i32, f64),
fld4: i16,
fld5: u16,

},
Variant1{
fld0: f64,

},
Variant2{
fld0: bool,
fld1: Adt18,
fld2: isize,
fld3: f32,
fld4: u128,

}}
#[derive(Debug)]
pub struct Adt28 {
fld0: u128,
fld1: f32,
fld2: *mut i8,
}
#[derive(Debug)]
pub enum Adt29 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt28,
fld3: [u16; 8],
fld4: f32,

},
Variant1{
fld0: bool,
fld1: f32,
fld2: usize,
fld3: Adt28,
fld4: Adt23,
fld5: i32,
fld6: [char; 2],

},
Variant2{
fld0: bool,
fld1: Adt23,

},
Variant3{
fld0: f32,
fld1: char,
fld2: isize,
fld3: (i32, f64),
fld4: i16,
fld5: Adt23,
fld6: i64,
fld7: Adt18,

}}
#[derive(Debug)]
pub struct Adt32 {
fld0: u8,
fld1: u16,
fld2: [u16; 8],
fld3: i8,
fld4: f64,
fld5: (i32, f64),
}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: *mut [i8; 1],
fld1: u16,
fld2: isize,
fld3: u8,
fld4: i16,
fld5: Adt29,
fld6: *const *mut (i32, f64),
fld7: i128,

},
Variant1{
fld0: [char; 2],
fld1: [i8; 1],

}}
#[derive(Debug)]
pub enum Adt70 {
Variant0{
fld0: ([u16; 8], f64, *mut isize),
fld1: (Adt23,),
fld2: u32,
fld3: i8,
fld4: *mut *mut (i32, f64),
fld5: *mut [i8; 1],
fld6: *const ((i32, f64), [u16; 8], [i8; 5]),

},
Variant1{
fld0: *mut [i8; 1],
fld1: *mut i64,
fld2: (i32, f64),
fld3: u16,
fld4: *mut isize,
fld5: *mut (Adt23,),

},
Variant2{
fld0: usize,
fld1: f32,
fld2: u64,
fld3: i32,

}}

