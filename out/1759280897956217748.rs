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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u32,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u64,mut _11: u16) -> isize {
mir! {
type RET = isize;
let _12: isize;
let _13: i16;
let _14: [i8; 6];
let _15: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _16: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _17: Adt19;
let _18: u32;
let _19: [u16; 1];
let _20: Adt52;
let _21: [char; 1];
let _22: [u64; 6];
let _23: f32;
let _24: f32;
let _25: isize;
let _26: i32;
let _27: isize;
let _28: Adt50;
let _29: isize;
let _30: Adt60;
let _31: *const f64;
let _32: char;
let _33: bool;
let _34: (*mut f64, &'static char);
let _35: (&'static char,);
let _36: [u64; 6];
let _37: bool;
let _38: Adt52;
let _39: f32;
let _40: f32;
let _41: (&'static char,);
let _42: &'static mut &'static mut usize;
let _43: &'static usize;
let _44: i32;
let _45: (f64, &'static (u32, u32, Adt17, f32));
let _46: &'static *mut bool;
let _47: *const &'static Adt19;
let _48: bool;
let _49: [u64; 6];
let _50: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _51: bool;
let _52: isize;
let _53: &'static mut *const i64;
let _54: &'static mut &'static *mut bool;
let _55: isize;
let _56: f64;
let _57: isize;
let _58: bool;
let _59: *mut (f64, &'static (u32, u32, Adt17, f32));
let _60: *const i64;
let _61: &'static &'static mut [u64; 6];
let _62: u8;
let _63: *const i64;
let _64: char;
let _65: u16;
let _66: f32;
let _67: isize;
let _68: bool;
let _69: *const (&'static char,);
let _70: u32;
let _71: i16;
let _72: char;
let _73: f64;
let _74: bool;
let _75: *const Adt50;
let _76: &'static (u32, u32, Adt17, f32);
let _77: [char; 1];
let _78: (Adt50,);
let _79: ();
let _80: ();
{
_5 = (-25832_i16) + (-13257_i16);
_11 = 59887_u16 << _5;
_8 = (-165360191939015048828112919985464589185_i128);
_11 = 17612_u16 << _8;
match _8 {
174922174981923414635261687446303622271 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_7 = (-4892462602729710824_i64) - (-6141032782114874668_i64);
_5 = (-22067_i16);
_13 = _5;
_3 = !3394925807_u32;
RET = 9223372036854775807_isize << _5;
_6 = !(-1002942757_i32);
Call(_13 = fn1(_8, _5, _6, RET, _3, _7, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb4 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb6 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb8 = {
_21 = [_20.fld1];
place!(Field::<bool>(Variant(_20.fld4.3, 0), 0)) = !_1;
place!(Field::<u64>(Variant(_20.fld4.3, 0), 2)) = _1 as u64;
_20.fld6.3 = (_18, _20.fld2.0, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: Field::<u64>(Variant(_20.fld4.3, 0), 2),fld2: Field::<f32>(Variant(_20.fld6.3.2, 1), 2) };
match _16.3.fld0 {
0 => bb5,
1 => bb7,
2 => bb4,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
338776639157727411227590800225980498799 => bb14,
_ => bb13
}
}
bb9 = {
Return()
}
bb10 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb12 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb14 = {
_20.fld2 = (_20.fld6.3.0, _20.fld6.3.0, _20.fld6.3.2, _20.fld6.3.3);
_7 = 6496837687199256212_i64 >> Field::<i128>(Variant(_20.fld2.2, 1), 0);
_20.fld3 = core::ptr::addr_of!(_7);
_20.fld4.2 = [_11];
_23 = -Field::<f32>(Variant(_20.fld2.2, 1), 2);
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -_20.fld2.3;
_20.fld6.2 = _6;
_28.fld0 = !_16.3.fld0;
_7 = 2561012623794209857_i64 | (-7611530199532835440_i64);
place!(Field::<u64>(Variant(_20.fld6.3.2, 1), 1)) = Field::<u64>(Variant(_20.fld4.3, 0), 2);
_11 = _9 as u16;
_20.fld7 = _16.3.fld0 as f64;
_26 = _6 + _6;
_29 = RET & _12;
_20.fld4.0 = core::ptr::addr_of_mut!(_11);
_14 = [_4,_4,_4,_4,_4,_4];
_29 = _12 | _12;
_20.fld2.3 = Field::<f32>(Variant(_20.fld2.2, 1), 2);
_30 = Adt60::Variant2 { fld0: Field::<bool>(Variant(_20.fld4.3, 0), 0),fld1: Field::<u64>(Variant(_20.fld4.3, 0), 2) };
place!(Field::<u64>(Variant(_20.fld4.3, 0), 2)) = _4 as u64;
_29 = _20.fld4.1 >> _5;
place!(Field::<u64>(Variant(_30, 2), 1)) = Field::<u64>(Variant(_20.fld6.3.2, 1), 1) & Field::<u64>(Variant(_20.fld6.3.2, 1), 1);
Goto(bb15)
}
bb15 = {
_11 = 58590_u16;
_34.1 = &_20.fld6.0;
_16.3 = Adt50 { fld0: _28.fld0 };
_12 = !_29;
place!(Field::<bool>(Variant(_30, 2), 0)) = Field::<bool>(Variant(_20.fld4.3, 0), 0) < Field::<bool>(Variant(_20.fld4.3, 0), 0);
place!(Field::<u64>(Variant(_20.fld2.2, 1), 1)) = Field::<u64>(Variant(_30, 2), 1) << _13;
_16.3.fld0 = _28.fld0 << _20.fld2.1;
_14 = [_4,_4,_4,_4,_4,_4];
_26 = !_20.fld6.2;
match _11 {
0 => bb1,
58590 => bb16,
_ => bb5
}
}
bb16 = {
_19 = [_11];
place!(Field::<bool>(Variant(_20.fld4.3, 0), 0)) = _16.3.fld0 == _16.3.fld0;
_26 = _20.fld6.2 ^ _20.fld6.2;
_20.fld4.0 = core::ptr::addr_of_mut!(_11);
_20.fld6.3 = (_20.fld2.0, _18, _20.fld2.2, _20.fld2.3);
_28 = Adt50 { fld0: _16.3.fld0 };
_20.fld4.1 = _20.fld6.3.0 as isize;
_23 = -_20.fld6.3.3;
_16.2 = core::ptr::addr_of!(_12);
_16.3 = Adt50 { fld0: _28.fld0 };
_13 = !_5;
_20.fld6.2 = Field::<bool>(Variant(_30, 2), 0) as i32;
_16.3.fld0 = _9 as u128;
_16.2 = core::ptr::addr_of!(_27);
_16.1 = [Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1)];
_32 = _2;
_20.fld1 = _32;
_35.0 = &_20.fld1;
_20.fld2.1 = _20.fld6.3.1;
place!(Field::<i128>(Variant(_20.fld6.3.2, 1), 0)) = _9 as i128;
_26 = _20.fld6.2;
_20.fld6 = (_32, Move(_20.fld4.3), _26, _20.fld2);
match _11 {
0 => bb6,
1 => bb9,
2 => bb3,
58590 => bb18,
_ => bb17
}
}
bb17 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb18 = {
_38.fld6.2 = _20.fld6.2;
_38.fld0 = Field::<bool>(Variant(_20.fld6.1, 0), 0) | Field::<bool>(Variant(_20.fld6.1, 0), 0);
_22 = [Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),_10,Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.1, 0), 2),Field::<u64>(Variant(_30, 2), 1)];
_2 = _32;
_15 = &_20.fld6;
_1 = _38.fld0 ^ Field::<bool>(Variant((*_15).1, 0), 0);
_20.fld2.2 = Adt17::Variant0 { fld0: (*_15).3.1,fld1: (*_15).0,fld2: _12,fld3: _9,fld4: _5,fld5: 59_u8,fld6: _28.fld0 };
_27 = _20.fld4.1 ^ _20.fld4.1;
_20.fld2.3 = (*_15).3.3;
_25 = _12 & _27;
_20.fld4.2 = [_11];
place!(Field::<usize>(Variant(_20.fld2.2, 0), 3)) = Field::<i128>(Variant((*_15).3.2, 1), 0) as usize;
_25 = _27 * _20.fld4.1;
place!(Field::<f32>(Variant(_20.fld6.3.2, 1), 2)) = -_20.fld2.3;
_38.fld6.3 = _20.fld6.3;
_34.0 = core::ptr::addr_of_mut!(_38.fld7);
_16.0 = core::ptr::addr_of!(_35);
_19 = _20.fld4.2;
match _11 {
0 => bb15,
1 => bb2,
2 => bb19,
3 => bb20,
4 => bb21,
58590 => bb23,
_ => bb22
}
}
bb19 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb20 = {
_19 = [_11];
place!(Field::<bool>(Variant(_20.fld4.3, 0), 0)) = _16.3.fld0 == _16.3.fld0;
_26 = _20.fld6.2 ^ _20.fld6.2;
_20.fld4.0 = core::ptr::addr_of_mut!(_11);
_20.fld6.3 = (_20.fld2.0, _18, _20.fld2.2, _20.fld2.3);
_28 = Adt50 { fld0: _16.3.fld0 };
_20.fld4.1 = _20.fld6.3.0 as isize;
_23 = -_20.fld6.3.3;
_16.2 = core::ptr::addr_of!(_12);
_16.3 = Adt50 { fld0: _28.fld0 };
_13 = !_5;
_20.fld6.2 = Field::<bool>(Variant(_30, 2), 0) as i32;
_16.3.fld0 = _9 as u128;
_16.2 = core::ptr::addr_of!(_27);
_16.1 = [Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1)];
_32 = _2;
_20.fld1 = _32;
_35.0 = &_20.fld1;
_20.fld2.1 = _20.fld6.3.1;
place!(Field::<i128>(Variant(_20.fld6.3.2, 1), 0)) = _9 as i128;
_26 = _20.fld6.2;
_20.fld6 = (_32, Move(_20.fld4.3), _26, _20.fld2);
match _11 {
0 => bb6,
1 => bb9,
2 => bb3,
58590 => bb18,
_ => bb17
}
}
bb21 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb22 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb23 = {
_9 = Field::<usize>(Variant(_20.fld2.2, 0), 3);
_37 = Field::<bool>(Variant((*_15).1, 0), 0) | Field::<bool>(Variant((*_15).1, 0), 0);
_20.fld4.3 = Adt19::Variant3 { fld0: Field::<bool>(Variant((*_15).1, 0), 0),fld1: _9,fld2: (*_15).3.0,fld3: _4,fld4: Move(_20.fld3),fld5: (*_15).2,fld6: _7,fld7: _11 };
_38.fld1 = (*_15).0;
_20.fld2.0 = _3;
place!(Field::<*const i64>(Variant(_20.fld4.3, 3), 4)) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_20.fld4.3, 3), 6)));
Goto(bb24)
}
bb24 = {
place!(Field::<i8>(Variant(_20.fld4.3, 3), 3)) = Field::<bool>(Variant((*_15).1, 0), 0) as i8;
_20.fld6.1 = Adt19::Variant1 { fld0: Move(Field::<*const i64>(Variant(_20.fld4.3, 3), 4)) };
_9 = _27 as usize;
place!(Field::<isize>(Variant(_20.fld2.2, 0), 2)) = _27;
_38.fld2.0 = (*_15).3.1 - (*_15).3.1;
match Field::<u16>(Variant(_20.fld4.3, 3), 7) {
58590 => bb25,
_ => bb11
}
}
bb25 = {
_20.fld2.2 = _20.fld6.3.2;
place!(Field::<bool>(Variant(_20.fld4.3, 3), 0)) = (*_15).2 <= (*_15).2;
_16.3.fld0 = _28.fld0 | _28.fld0;
_34.0 = core::ptr::addr_of_mut!(_38.fld7);
place!(Field::<i128>(Variant(_20.fld6.3.2, 1), 0)) = Field::<i128>(Variant(_38.fld6.3.2, 1), 0);
_38.fld6.3.3 = (*_15).3.3;
Goto(bb26)
}
bb26 = {
_38.fld2.3 = -(*_15).3.3;
_17 = Move(_20.fld6.1);
_44 = (*_15).2 + (*_15).2;
_38.fld7 = _20.fld7;
_40 = _38.fld6.3.3;
_41 = (Move(_35.0),);
_38.fld6.3 = ((*_15).3.1, (*_15).3.1, _20.fld6.3.2, (*_15).3.3);
_38.fld6 = ((*_15).0, Move(_17), _26, _20.fld2);
_20.fld6 = (_32, Move(_38.fld6.1), Field::<i32>(Variant(_20.fld4.3, 3), 5), _20.fld2);
_35.0 = Move(_34.1);
_17 = Move(_20.fld6.1);
match Field::<u16>(Variant(_20.fld4.3, 3), 7) {
58590 => bb27,
_ => bb15
}
}
bb27 = {
_2 = _38.fld1;
_3 = !_20.fld6.3.1;
_38.fld2 = (_3, _20.fld2.0, _38.fld6.3.2, _20.fld6.3.3);
_38.fld7 = _20.fld7 + _20.fld7;
_38.fld6.2 = _26;
place!(Field::<i128>(Variant(_20.fld2.2, 1), 0)) = _38.fld6.3.0 as i128;
_31 = core::ptr::addr_of!(_45.0);
_34.1 = &_20.fld1;
place!(Field::<u64>(Variant(_30, 2), 1)) = Field::<u64>(Variant(_20.fld6.3.2, 1), 1) - Field::<u64>(Variant(_38.fld2.2, 1), 1);
(*_31) = _20.fld7 - _38.fld7;
_20.fld0 = Field::<bool>(Variant(_20.fld4.3, 3), 0);
_38.fld6.1 = Adt19::Variant3 { fld0: Field::<bool>(Variant(_20.fld4.3, 3), 0),fld1: _9,fld2: _38.fld2.0,fld3: Field::<i8>(Variant(_20.fld4.3, 3), 3),fld4: Move(Field::<*const i64>(Variant(_17, 1), 0)),fld5: Field::<i32>(Variant(_20.fld4.3, 3), 5),fld6: _7,fld7: _11 };
_38.fld3 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_38.fld6.1, 3), 6)));
Call(_20.fld6.3.1 = core::intrinsics::bswap(_38.fld6.3.0), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_20.fld4.1 = _27;
_20.fld6.1 = Move(_38.fld6.1);
place!(Field::<f32>(Variant(_20.fld6.3.2, 1), 2)) = Field::<f32>(Variant(_20.fld2.2, 1), 2) * _38.fld2.3;
_38.fld4.3 = Move(_20.fld6.1);
_38.fld2.2 = Adt17::Variant0 { fld0: _3,fld1: _38.fld1,fld2: _20.fld4.1,fld3: _9,fld4: _5,fld5: 243_u8,fld6: _28.fld0 };
match _11 {
0 => bb15,
1 => bb4,
58590 => bb30,
_ => bb29
}
}
bb29 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb30 = {
_20.fld6.1 = Adt19::Variant2 { fld0: _44,fld1: _9,fld2: 153_u8,fld3: Field::<i8>(Variant(_38.fld4.3, 3), 3),fld4: Field::<i128>(Variant(_38.fld6.3.2, 1), 0) };
_45.0 = _20.fld7;
_20.fld3 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_20.fld4.3, 3), 6)));
_17 = Adt19::Variant3 { fld0: _20.fld0,fld1: Field::<usize>(Variant(_38.fld2.2, 0), 3),fld2: _3,fld3: Field::<i8>(Variant(_38.fld4.3, 3), 3),fld4: Move(_20.fld3),fld5: Field::<i32>(Variant(_20.fld6.1, 2), 0),fld6: _7,fld7: _11 };
_38.fld4.1 = _29;
_20.fld6.3.3 = _5 as f32;
_28 = Adt50 { fld0: _16.3.fld0 };
(*_31) = _13 as f64;
_39 = _20.fld6.3.3 - _20.fld6.3.3;
Goto(bb31)
}
bb31 = {
_45.1 = &_20.fld6.3;
_34.0 = core::ptr::addr_of_mut!((*_31));
_45.1 = &_20.fld2;
_20.fld6.0 = _2;
match Field::<u16>(Variant(_38.fld4.3, 3), 7) {
0 => bb18,
1 => bb10,
2 => bb27,
3 => bb4,
4 => bb12,
5 => bb24,
6 => bb7,
58590 => bb32,
_ => bb22
}
}
bb32 = {
_53 = &mut place!(Field::<*const i64>(Variant(_38.fld4.3, 3), 4));
_16.3.fld0 = !_28.fld0;
_55 = -_27;
_43 = &place!(Field::<usize>(Variant(_20.fld6.1, 2), 1));
(*_31) = _20.fld7;
(*_53) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_20.fld4.3, 3), 6)));
place!(Field::<usize>(Variant(_17, 3), 1)) = (*_43);
(*_53) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_17, 3), 6)));
_2 = _20.fld1;
_20.fld2.3 = _23 + _39;
_43 = &place!(Field::<usize>(Variant(_17, 3), 1));
_20.fld6.1 = Adt19::Variant1 { fld0: Move(Field::<*const i64>(Variant(_17, 3), 4)) };
_20.fld2.3 = _55 as f32;
Goto(bb33)
}
bb33 = {
_20.fld6.3.3 = Field::<f32>(Variant(_20.fld6.3.2, 1), 2) * Field::<f32>(Variant(_20.fld2.2, 1), 2);
Goto(bb34)
}
bb34 = {
place!(Field::<*const i64>(Variant(_20.fld6.1, 1), 0)) = core::ptr::addr_of!(_7);
place!(Field::<i64>(Variant(_20.fld4.3, 3), 6)) = _7 + _7;
_3 = Field::<i64>(Variant(_20.fld4.3, 3), 6) as u32;
_37 = Field::<bool>(Variant(_20.fld4.3, 3), 0) >= _1;
_14 = [Field::<i8>(Variant(_20.fld4.3, 3), 3),Field::<i8>(Variant(_20.fld4.3, 3), 3),Field::<i8>(Variant(_20.fld4.3, 3), 3),Field::<i8>(Variant(_20.fld4.3, 3), 3),Field::<i8>(Variant(_17, 3), 3),Field::<i8>(Variant(_20.fld4.3, 3), 3)];
place!(Field::<u32>(Variant(_17, 3), 2)) = !_3;
Goto(bb35)
}
bb35 = {
_20.fld0 = !_37;
(*_31) = Field::<f32>(Variant(_20.fld2.2, 1), 2) as f64;
(*_31) = -_20.fld7;
_65 = Field::<u16>(Variant(_17, 3), 7) * Field::<u16>(Variant(_20.fld4.3, 3), 7);
(*_53) = core::ptr::addr_of!(_7);
(*_31) = _20.fld7;
_33 = _20.fld0;
_60 = core::ptr::addr_of!(_7);
_20.fld7 = -(*_31);
place!(Field::<u32>(Variant(_17, 3), 2)) = _20.fld6.0 as u32;
(*_53) = core::ptr::addr_of!((*_60));
place!(Field::<u64>(Variant(_30, 2), 1)) = _27 as u64;
place!(Field::<i64>(Variant(_20.fld4.3, 3), 6)) = (*_60) - (*_60);
_32 = _20.fld1;
_20.fld2.1 = _18;
match Field::<u16>(Variant(_17, 3), 7) {
58590 => bb37,
_ => bb36
}
}
bb36 = {
_20.fld4.1 = _27;
_20.fld6.1 = Move(_38.fld6.1);
place!(Field::<f32>(Variant(_20.fld6.3.2, 1), 2)) = Field::<f32>(Variant(_20.fld2.2, 1), 2) * _38.fld2.3;
_38.fld4.3 = Move(_20.fld6.1);
_38.fld2.2 = Adt17::Variant0 { fld0: _3,fld1: _38.fld1,fld2: _20.fld4.1,fld3: _9,fld4: _5,fld5: 243_u8,fld6: _28.fld0 };
match _11 {
0 => bb15,
1 => bb4,
58590 => bb30,
_ => bb29
}
}
bb37 = {
_36 = [Field::<u64>(Variant(_20.fld6.3.2, 1), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1)];
(*_53) = core::ptr::addr_of!((*_60));
Call(place!(Field::<u32>(Variant(_20.fld4.3, 3), 2)) = core::intrinsics::bswap(_20.fld2.1), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
(*_53) = core::ptr::addr_of!((*_60));
place!(Field::<u64>(Variant(_30, 2), 1)) = !Field::<u64>(Variant(_20.fld2.2, 1), 1);
(*_60) = Field::<i64>(Variant(_20.fld4.3, 3), 6);
(*_60) = Field::<i64>(Variant(_20.fld4.3, 3), 6) >> (*_43);
(*_31) = 136_u8 as f64;
match Field::<u16>(Variant(_20.fld4.3, 3), 7) {
0 => bb39,
1 => bb40,
2 => bb41,
3 => bb42,
4 => bb43,
5 => bb44,
58590 => bb46,
_ => bb45
}
}
bb39 = {
Return()
}
bb40 = {
_19 = [_11];
place!(Field::<bool>(Variant(_20.fld4.3, 0), 0)) = _16.3.fld0 == _16.3.fld0;
_26 = _20.fld6.2 ^ _20.fld6.2;
_20.fld4.0 = core::ptr::addr_of_mut!(_11);
_20.fld6.3 = (_20.fld2.0, _18, _20.fld2.2, _20.fld2.3);
_28 = Adt50 { fld0: _16.3.fld0 };
_20.fld4.1 = _20.fld6.3.0 as isize;
_23 = -_20.fld6.3.3;
_16.2 = core::ptr::addr_of!(_12);
_16.3 = Adt50 { fld0: _28.fld0 };
_13 = !_5;
_20.fld6.2 = Field::<bool>(Variant(_30, 2), 0) as i32;
_16.3.fld0 = _9 as u128;
_16.2 = core::ptr::addr_of!(_27);
_16.1 = [Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld6.3.2, 1), 1)];
_32 = _2;
_20.fld1 = _32;
_35.0 = &_20.fld1;
_20.fld2.1 = _20.fld6.3.1;
place!(Field::<i128>(Variant(_20.fld6.3.2, 1), 0)) = _9 as i128;
_26 = _20.fld6.2;
_20.fld6 = (_32, Move(_20.fld4.3), _26, _20.fld2);
match _11 {
0 => bb6,
1 => bb9,
2 => bb3,
58590 => bb18,
_ => bb17
}
}
bb41 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb42 = {
_20.fld2.2 = _20.fld6.3.2;
place!(Field::<bool>(Variant(_20.fld4.3, 3), 0)) = (*_15).2 <= (*_15).2;
_16.3.fld0 = _28.fld0 | _28.fld0;
_34.0 = core::ptr::addr_of_mut!(_38.fld7);
place!(Field::<i128>(Variant(_20.fld6.3.2, 1), 0)) = Field::<i128>(Variant(_38.fld6.3.2, 1), 0);
_38.fld6.3.3 = (*_15).3.3;
Goto(bb26)
}
bb43 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb44 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb45 = {
_13 = _5 + _5;
_13 = 8429949733102598726_u64 as i16;
_9 = !12321153716890169780_usize;
_5 = _13 - _13;
_10 = 18244247757290362052_u64;
_7 = 9194166927104429966_i64 + (-8516872594644970956_i64);
_16.3 = Adt50 { fld0: 11363643293604669564431781596303585006_u128 };
_10 = RET as u64;
_16.1 = [_10,_10,_10,_10,_10,_10];
_16.3 = Adt50 { fld0: 116588873646017713698951136844290301870_u128 };
_12 = RET << _5;
_16.3.fld0 = 91935241034879804581301971198486106648_u128 ^ 301345771099559592215927584033511108583_u128;
_16.2 = core::ptr::addr_of!(_12);
_7 = _3 as i64;
_13 = _5 >> _8;
_16.2 = core::ptr::addr_of!(RET);
_16.2 = core::ptr::addr_of!(_12);
_4 = !25_i8;
_13 = -_5;
_12 = RET >> _3;
_8 = 128091644492158981843855689502405894762_i128 | 252068505668264742823172662084504900_i128;
_7 = (-2993372504695730643_i64) + (-3593520111626535394_i64);
_3 = 3530471644_u32 >> _12;
_8 = (-105696472105273103747266969033079498800_i128);
_4 = true as i8;
_14 = [_4,_4,_4,_4,_4,_4];
_9 = 3579328655719789137_usize & 4580824712246310102_usize;
_13 = _5;
Goto(bb4)
}
bb46 = {
_11 = Field::<u16>(Variant(_20.fld4.3, 3), 7) >> (*_43);
_1 = !Field::<bool>(Variant(_30, 2), 0);
_34.0 = core::ptr::addr_of_mut!(_45.0);
_45.1 = &_20.fld6.3;
place!(Field::<bool>(Variant(_30, 2), 0)) = !_20.fld0;
_16.1 = [Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_30, 2), 1),Field::<u64>(Variant(_20.fld2.2, 1), 1)];
(*_60) = -Field::<i64>(Variant(_17, 3), 6);
(*_60) = Field::<i64>(Variant(_20.fld4.3, 3), 6) << _28.fld0;
_24 = _40 * _20.fld2.3;
_37 = !Field::<bool>(Variant(_20.fld4.3, 3), 0);
_51 = !_33;
(*_31) = _20.fld7 - _20.fld7;
(*_31) = _20.fld7;
_62 = !208_u8;
(*_53) = core::ptr::addr_of!((*_60));
(*_53) = core::ptr::addr_of!((*_60));
_44 = Field::<i32>(Variant(_17, 3), 5);
_56 = -(*_31);
_20.fld2.2 = Adt17::Variant2 { fld0: (*_31),fld1: _11,fld2: _62 };
(*_60) = -Field::<i64>(Variant(_17, 3), 6);
_20.fld6.3.0 = _20.fld2.1 << (*_43);
_66 = _24 * _20.fld2.3;
_68 = _37 == _33;
(*_53) = core::ptr::addr_of!((*_60));
_25 = (*_31) as isize;
Call((*_60) = core::intrinsics::bswap(Field::<i64>(Variant(_20.fld4.3, 3), 6)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
_9 = !(*_43);
_20.fld6.2 = (*_31) as i32;
_48 = !_33;
(*_31) = _16.3.fld0 as f64;
_20.fld4.0 = core::ptr::addr_of_mut!(_11);
(*_53) = Move(_60);
_28 = Move(_16.3);
_16.3.fld0 = _20.fld6.0 as u128;
match Field::<u16>(Variant(_20.fld4.3, 3), 7) {
0 => bb33,
58590 => bb48,
_ => bb8
}
}
bb48 = {
_69 = Move(_16.0);
_20.fld6.3.3 = _66 * _24;
_59 = core::ptr::addr_of_mut!(_45);
_62 = Field::<u8>(Variant(_20.fld2.2, 2), 2) + Field::<u8>(Variant(_20.fld2.2, 2), 2);
place!(Field::<u16>(Variant(_20.fld4.3, 3), 7)) = Field::<u16>(Variant(_20.fld2.2, 2), 1);
_44 = -Field::<i32>(Variant(_17, 3), 5);
_20.fld2.0 = _20.fld6.3.0;
(*_31) = -_56;
place!(Field::<i32>(Variant(_20.fld4.3, 3), 5)) = _44;
(*_31) = _27 as f64;
place!(Field::<usize>(Variant(_17, 3), 1)) = _9 * _9;
(*_53) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_17, 3), 6)));
_40 = _5 as f32;
match Field::<u16>(Variant(_17, 3), 7) {
0 => bb19,
1 => bb43,
2 => bb47,
3 => bb15,
4 => bb49,
58590 => bb51,
_ => bb50
}
}
bb49 = {
_20.fld4.3 = Move(_20.fld6.1);
_20.fld6.3.1 = _3 & _18;
_20.fld6.3.3 = _12 as f32;
_20.fld6.3.0 = Field::<u16>(Variant(_20.fld6.3.2, 2), 1) as u32;
_20.fld1 = _2;
place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)) = !_11;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 76_u8 };
_20.fld4.2 = [_11];
_1 = _18 <= _20.fld6.3.1;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld6.3.2, 2), 1)));
RET = _5 as isize;
_20.fld6.2 = !_6;
_21 = [_20.fld1];
_20.fld2.0 = _18;
_16.3 = Adt50 { fld0: 20008239440095355776012082632576700884_u128 };
_16.1 = [Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2),_10,_10,Field::<u64>(Variant(_20.fld4.3, 0), 2),Field::<u64>(Variant(_20.fld4.3, 0), 2)];
_16.2 = core::ptr::addr_of!(_12);
_20.fld6.0 = _20.fld1;
_20.fld4.1 = !_12;
_20.fld2.2 = Adt17::Variant1 { fld0: _8,fld1: _10,fld2: _20.fld6.3.3 };
_20.fld6.3.2 = _20.fld2.2;
place!(Field::<i128>(Variant(_20.fld4.3, 0), 1)) = _8 * _8;
_16.3 = Adt50 { fld0: 338776639157727411227590800225980498799_u128 };
_19 = _20.fld4.2;
_12 = RET | _20.fld4.1;
_20.fld6.3 = (_3, _18, _20.fld2.2, Field::<f32>(Variant(_20.fld2.2, 1), 2));
place!(Field::<f32>(Variant(_20.fld2.2, 1), 2)) = -Field::<f32>(Variant(_20.fld6.3.2, 1), 2);
match _16.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
338776639157727411227590800225980498799 => bb8,
_ => bb7
}
}
bb50 = {
_16.3 = Adt50 { fld0: 55822685090834583826515903450575256303_u128 };
_1 = false & true;
_1 = _4 != _4;
_17 = Adt19::Variant0 { fld0: _1,fld1: _8,fld2: _10 };
_4 = 115_i8 & 21_i8;
_2 = '\u{3df48}';
_16.2 = core::ptr::addr_of!(RET);
_8 = -Field::<i128>(Variant(_17, 0), 1);
place!(Field::<u64>(Variant(_17, 0), 2)) = !_10;
place!(Field::<i128>(Variant(_17, 0), 1)) = _8 & _8;
_9 = _3 as usize;
_20.fld2.0 = _3;
_20.fld2.1 = _3 * _20.fld2.0;
_18 = _3 - _3;
RET = Field::<bool>(Variant(_17, 0), 0) as isize;
_20.fld6.1 = Move(_17);
_20.fld7 = _12 as f64;
_20.fld6.3.2 = Adt17::Variant2 { fld0: _20.fld7,fld1: _11,fld2: 172_u8 };
RET = _7 as isize;
Call(_12 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb51 = {
(*_59).1 = &_20.fld2;
_20.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_20.fld2.2, 2), 1)));
(*_31) = -_20.fld7;
_74 = _51;
(*_59).1 = &_20.fld6.3;
_67 = _29 | _20.fld4.1;
place!(Field::<f32>(Variant(_20.fld6.3.2, 1), 2)) = _24;
_35 = (Move(_34.1),);
(*_59).1 = &_20.fld2;
(*_59).1 = &_20.fld6.3;
_70 = _20.fld6.3.0 - _3;
(*_59).1 = &_20.fld2;
_50 = Move(_15);
RET = _67;
_4 = Field::<i8>(Variant(_20.fld4.3, 3), 3) & Field::<i8>(Variant(_20.fld4.3, 3), 3);
Goto(bb52)
}
bb52 = {
Call(_79 = dump_var(Move(_5), Move(_74), Move(_7), Move(_27)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_79 = dump_var(Move(_67), Move(_36), Move(_1), Move(_62)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_79 = dump_var(Move(_11), Move(_10), Move(_19), Move(_55)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_79 = dump_var(Move(_68), Move(_22), Move(_9), Move(_2)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_79 = dump_var(Move(_65), Move(_29), _80, _80), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: i16,mut _3: i32,mut _4: isize,mut _5: u32,mut _6: i64,mut _7: i32) -> i16 {
mir! {
type RET = i16;
let _8: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _9: i64;
let _10: Adt19;
let _11: f64;
let _12: char;
let _13: &'static (u32, u32, Adt17, f32);
let _14: u8;
let _15: &'static i32;
let _16: *mut f64;
let _17: f64;
let _18: u8;
let _19: char;
let _20: i32;
let _21: i64;
let _22: &'static i32;
let _23: isize;
let _24: bool;
let _25: char;
let _26: char;
let _27: Adt52;
let _28: f32;
let _29: Adt34;
let _30: char;
let _31: &'static mut usize;
let _32: i16;
let _33: bool;
let _34: f64;
let _35: i16;
let _36: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _37: u128;
let _38: u8;
let _39: Adt85;
let _40: bool;
let _41: (*mut f64, &'static char);
let _42: isize;
let _43: (f64, &'static (u32, u32, Adt17, f32));
let _44: i32;
let _45: (&'static char,);
let _46: i16;
let _47: (&'static char,);
let _48: bool;
let _49: *const *mut u16;
let _50: &'static &'static mut [u64; 6];
let _51: *mut *mut *const bool;
let _52: *mut *mut *const bool;
let _53: (&'static char,);
let _54: f64;
let _55: i128;
let _56: f64;
let _57: isize;
let _58: isize;
let _59: f32;
let _60: u8;
let _61: &'static mut *const i64;
let _62: ();
let _63: ();
{
RET = _2;
_6 = (-496811089937930439_i64);
_6 = (-5739758013615562986_i64) + 2879429041284954470_i64;
match _2 {
340282366920938463463374607431768189389 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2 = RET << _4;
_5 = 1865440090_u32 >> _2;
_6 = _4 as i64;
_7 = _2 as i32;
_7 = _3 << _2;
match RET {
0 => bb3,
1 => bb4,
340282366920938463463374607431768189389 => bb6,
_ => bb5
}
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
_4 = (-115_isize) + (-68_isize);
_4 = (-9223372036854775808_isize) >> _7;
_8.3 = [16045049186278175204_u64,15270758332993339970_u64,6453263274119183217_u64,7580373064158296231_u64,10978259717575720610_u64,4658122899839525879_u64];
_1 = !105208974905696272905599432792174651165_i128;
_8.3 = [17810533014130589186_u64,4328086664605131602_u64,4381138585342604617_u64,1384605455006909982_u64,611198594999129396_u64,2799850455578500625_u64];
_4 = 87_isize >> _1;
Goto(bb7)
}
bb7 = {
_8.3 = [6004546325408917688_u64,11207200862230477118_u64,12041748931043171254_u64,2699214063562872461_u64,10223476890544995833_u64,15437173537146982922_u64];
_4 = 73_isize;
_3 = _7 << _2;
_6 = -(-7508547020841942511_i64);
_1 = (-22946962327852121498967716583058658138_i128);
_1 = 17351297741336932705859052369333310518_i128 << _5;
_9 = _6 & _6;
_8.3 = [1376546058213842716_u64,8079321490819898198_u64,5737559442411314541_u64,11402497966324517006_u64,3072785874764114045_u64,6502270973415890796_u64];
RET = _2;
_12 = '\u{bd919}';
_8.1.1 = Adt17::Variant0 { fld0: _5,fld1: _12,fld2: _4,fld3: 4_usize,fld4: _2,fld5: 207_u8,fld6: 125619653580672468095157962006560726810_u128 };
_10 = Adt19::Variant0 { fld0: false,fld1: _1,fld2: 16150961209650386394_u64 };
_8.2.1 = &place!(Field::<char>(Variant(_8.1.1, 0), 1));
_11 = 1318_u16 as f64;
_7 = 16063769450958173324_usize as i32;
_6 = -_9;
place!(Field::<i16>(Variant(_8.1.1, 0), 4)) = RET << _3;
match _4 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
73 => bb13,
_ => bb12
}
}
bb8 = {
_4 = (-115_isize) + (-68_isize);
_4 = (-9223372036854775808_isize) >> _7;
_8.3 = [16045049186278175204_u64,15270758332993339970_u64,6453263274119183217_u64,7580373064158296231_u64,10978259717575720610_u64,4658122899839525879_u64];
_1 = !105208974905696272905599432792174651165_i128;
_8.3 = [17810533014130589186_u64,4328086664605131602_u64,4381138585342604617_u64,1384605455006909982_u64,611198594999129396_u64,2799850455578500625_u64];
_4 = 87_isize >> _1;
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
_2 = RET << _4;
_5 = 1865440090_u32 >> _2;
_6 = _4 as i64;
_7 = _2 as i32;
_7 = _3 << _2;
match RET {
0 => bb3,
1 => bb4,
340282366920938463463374607431768189389 => bb6,
_ => bb5
}
}
bb13 = {
_3 = _7 >> Field::<i128>(Variant(_10, 0), 1);
place!(Field::<usize>(Variant(_8.1.1, 0), 3)) = !1763184190605337372_usize;
place!(Field::<u128>(Variant(_8.1.1, 0), 6)) = 284475278111619896038692187079870873063_u128;
place!(Field::<u8>(Variant(_8.1.1, 0), 5)) = 24_u8 & 83_u8;
place!(Field::<isize>(Variant(_8.1.1, 0), 2)) = _4 - _4;
_11 = Field::<u32>(Variant(_8.1.1, 0), 0) as f64;
_8.2.1 = &_12;
_1 = Field::<i128>(Variant(_10, 0), 1) + Field::<i128>(Variant(_10, 0), 1);
_8.2.0 = core::ptr::addr_of_mut!(_11);
place!(Field::<char>(Variant(_8.1.1, 0), 1)) = _12;
place!(Field::<isize>(Variant(_8.1.1, 0), 2)) = _4;
_2 = -Field::<i16>(Variant(_8.1.1, 0), 4);
_14 = Field::<u8>(Variant(_8.1.1, 0), 5);
place!(Field::<u128>(Variant(_8.1.1, 0), 6)) = 185793298733399336198062425109894053577_u128;
_15 = &_3;
_8.0 = &mut place!(Field::<usize>(Variant(_8.1.1, 0), 3));
_15 = &_7;
_6 = _9 + _9;
place!(Field::<u64>(Variant(_10, 0), 2)) = _4 as u64;
RET = _2;
_6 = _9;
_15 = &_3;
_3 = _7;
Call(_16 = fn2(_3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_12 = '\u{e06ab}';
RET = _5 as i16;
_14 = 97_u8 * 75_u8;
_5 = 2924949374_u32 & 1125484632_u32;
place!(Field::<u64>(Variant(_10, 0), 2)) = 18383517731590920173_u64 ^ 17403206613425595106_u64;
_16 = core::ptr::addr_of_mut!(_17);
(*_16) = _2 as f64;
(*_16) = _11 + _11;
_5 = 54027739367097542940915655045553829300_u128 as u32;
(*_16) = _11;
_17 = _11;
(*_16) = _11;
_15 = &_7;
(*_16) = -_11;
_15 = &_3;
match _4 {
0 => bb9,
1 => bb2,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb15,
6 => bb16,
73 => bb18,
_ => bb17
}
}
bb15 = {
_3 = _7 >> Field::<i128>(Variant(_10, 0), 1);
place!(Field::<usize>(Variant(_8.1.1, 0), 3)) = !1763184190605337372_usize;
place!(Field::<u128>(Variant(_8.1.1, 0), 6)) = 284475278111619896038692187079870873063_u128;
place!(Field::<u8>(Variant(_8.1.1, 0), 5)) = 24_u8 & 83_u8;
place!(Field::<isize>(Variant(_8.1.1, 0), 2)) = _4 - _4;
_11 = Field::<u32>(Variant(_8.1.1, 0), 0) as f64;
_8.2.1 = &_12;
_1 = Field::<i128>(Variant(_10, 0), 1) + Field::<i128>(Variant(_10, 0), 1);
_8.2.0 = core::ptr::addr_of_mut!(_11);
place!(Field::<char>(Variant(_8.1.1, 0), 1)) = _12;
place!(Field::<isize>(Variant(_8.1.1, 0), 2)) = _4;
_2 = -Field::<i16>(Variant(_8.1.1, 0), 4);
_14 = Field::<u8>(Variant(_8.1.1, 0), 5);
place!(Field::<u128>(Variant(_8.1.1, 0), 6)) = 185793298733399336198062425109894053577_u128;
_15 = &_3;
_8.0 = &mut place!(Field::<usize>(Variant(_8.1.1, 0), 3));
_15 = &_7;
_6 = _9 + _9;
place!(Field::<u64>(Variant(_10, 0), 2)) = _4 as u64;
RET = _2;
_6 = _9;
_15 = &_3;
_3 = _7;
Call(_16 = fn2(_3), ReturnTo(bb14), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
(*_16) = _11 - _11;
(*_16) = _11;
_11 = _5 as f64;
_17 = _11 * _11;
(*_16) = _11;
(*_16) = _11 - _11;
RET = 7_usize as i16;
_9 = _2 as i64;
(*_16) = _11 * _11;
place!(Field::<bool>(Variant(_10, 0), 0)) = false | true;
(*_16) = Field::<u64>(Variant(_10, 0), 2) as f64;
_22 = &(*_15);
(*_16) = _11 * _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = _1 | _1;
_16 = core::ptr::addr_of_mut!(_17);
(*_16) = _11 + _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = (*_22) as i128;
_19 = _12;
match _4 {
0 => bb10,
1 => bb2,
2 => bb15,
73 => bb19,
_ => bb17
}
}
bb19 = {
_23 = _4 * _4;
_7 = (*_22);
(*_16) = _23 as f64;
_15 = &_7;
(*_16) = _11 + _11;
_7 = _3;
_14 = !73_u8;
_9 = _6 | _6;
(*_16) = _14 as f64;
Goto(bb20)
}
bb20 = {
_25 = _19;
(*_16) = _11 - _11;
_20 = _5 as i32;
(*_16) = 35_i8 as f64;
(*_16) = _11 - _11;
(*_16) = -_11;
(*_16) = _11 - _11;
(*_16) = _11 * _11;
(*_16) = _11 - _11;
_26 = _19;
_27.fld6.1 = Move(_10);
(*_16) = _11 - _11;
_27.fld6.3.1 = _5;
Goto(bb21)
}
bb21 = {
_21 = _6 + _9;
place!(Field::<i128>(Variant(_27.fld6.1, 0), 1)) = -_1;
_27.fld4.3 = Move(_27.fld6.1);
_27.fld4.2 = [54270_u16];
_7 = (*_22) << Field::<i128>(Variant(_27.fld4.3, 0), 1);
(*_16) = _11;
place!(Field::<bool>(Variant(_27.fld4.3, 0), 0)) = false | true;
_27.fld6.2 = !_20;
(*_16) = _6 as f64;
(*_16) = _11;
_27.fld0 = !Field::<bool>(Variant(_27.fld4.3, 0), 0);
_27.fld6.1 = Adt19::Variant2 { fld0: _7,fld1: 7098511140274012999_usize,fld2: _14,fld3: (-121_i8),fld4: _1 };
_27.fld2.1 = _4 as u32;
_28 = _27.fld6.3.1 as f32;
(*_16) = RET as f64;
_27.fld2.3 = (*_16) as f32;
_27.fld2.2 = Adt17::Variant0 { fld0: _27.fld2.1,fld1: _25,fld2: _4,fld3: 3_usize,fld4: _2,fld5: Field::<u8>(Variant(_27.fld6.1, 2), 2),fld6: 143506710178900930047024463104691377214_u128 };
(*_16) = -_11;
_18 = _14 + Field::<u8>(Variant(_27.fld6.1, 2), 2);
place!(Field::<usize>(Variant(_27.fld2.2, 0), 3)) = Field::<u8>(Variant(_27.fld6.1, 2), 2) as usize;
(*_16) = (*_22) as f64;
_27.fld2.1 = !_27.fld6.3.1;
(*_16) = _11 + _11;
(*_16) = _11 * _11;
place!(Field::<i16>(Variant(_27.fld2.2, 0), 4)) = 80_i8 as i16;
_29 = Adt34 { fld0: _12 };
_27.fld6.1 = Adt19::Variant2 { fld0: (*_22),fld1: Field::<usize>(Variant(_27.fld2.2, 0), 3),fld2: _18,fld3: (-4_i8),fld4: Field::<i128>(Variant(_27.fld4.3, 0), 1) };
_22 = &place!(Field::<i32>(Variant(_27.fld6.1, 2), 0));
match _4 {
0 => bb1,
1 => bb6,
2 => bb14,
3 => bb16,
4 => bb5,
5 => bb22,
73 => bb24,
_ => bb23
}
}
bb22 = {
(*_16) = _11 - _11;
(*_16) = _11;
_11 = _5 as f64;
_17 = _11 * _11;
(*_16) = _11;
(*_16) = _11 - _11;
RET = 7_usize as i16;
_9 = _2 as i64;
(*_16) = _11 * _11;
place!(Field::<bool>(Variant(_10, 0), 0)) = false | true;
(*_16) = Field::<u64>(Variant(_10, 0), 2) as f64;
_22 = &(*_15);
(*_16) = _11 * _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = _1 | _1;
_16 = core::ptr::addr_of_mut!(_17);
(*_16) = _11 + _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = (*_22) as i128;
_19 = _12;
match _4 {
0 => bb10,
1 => bb2,
2 => bb15,
73 => bb19,
_ => bb17
}
}
bb23 = {
_23 = _4 * _4;
_7 = (*_22);
(*_16) = _23 as f64;
_15 = &_7;
(*_16) = _11 + _11;
_7 = _3;
_14 = !73_u8;
_9 = _6 | _6;
(*_16) = _14 as f64;
Goto(bb20)
}
bb24 = {
_18 = Field::<u8>(Variant(_27.fld6.1, 2), 2) * Field::<u8>(Variant(_27.fld6.1, 2), 2);
_7 = !(*_22);
_27.fld4.2 = [14242_u16];
_27.fld4.2 = [54320_u16];
_15 = Move(_22);
(*_16) = _11;
place!(Field::<usize>(Variant(_27.fld6.1, 2), 1)) = Field::<usize>(Variant(_27.fld2.2, 0), 3);
_27.fld4.3 = Adt19::Variant2 { fld0: _7,fld1: Field::<usize>(Variant(_27.fld6.1, 2), 1),fld2: _18,fld3: (-9_i8),fld4: _1 };
_27.fld6.3.0 = Field::<u32>(Variant(_27.fld2.2, 0), 0) | _5;
_10 = Adt19::Variant0 { fld0: _27.fld0,fld1: Field::<i128>(Variant(_27.fld6.1, 2), 4),fld2: 15884089069665755293_u64 };
place!(Field::<usize>(Variant(_27.fld2.2, 0), 3)) = (*_16) as usize;
_9 = _21 >> _2;
_27.fld1 = _29.fld0;
_18 = 18_i8 as u8;
place!(Field::<i32>(Variant(_27.fld4.3, 2), 0)) = _20;
(*_16) = -_11;
Goto(bb25)
}
bb25 = {
_29 = Adt34 { fld0: _26 };
_2 = Field::<i16>(Variant(_27.fld2.2, 0), 4) * Field::<i16>(Variant(_27.fld2.2, 0), 4);
_27.fld6.3.0 = _5 ^ _27.fld6.3.1;
_28 = _27.fld2.3;
_6 = 9859455215608399152_u64 as i64;
_27.fld4.2 = [16198_u16];
_35 = RET << Field::<i128>(Variant(_10, 0), 1);
match _4 {
0 => bb19,
1 => bb23,
2 => bb26,
3 => bb27,
73 => bb29,
_ => bb28
}
}
bb26 = {
_23 = _4 * _4;
_7 = (*_22);
(*_16) = _23 as f64;
_15 = &_7;
(*_16) = _11 + _11;
_7 = _3;
_14 = !73_u8;
_9 = _6 | _6;
(*_16) = _14 as f64;
Goto(bb20)
}
bb27 = {
_21 = _6 + _9;
place!(Field::<i128>(Variant(_27.fld6.1, 0), 1)) = -_1;
_27.fld4.3 = Move(_27.fld6.1);
_27.fld4.2 = [54270_u16];
_7 = (*_22) << Field::<i128>(Variant(_27.fld4.3, 0), 1);
(*_16) = _11;
place!(Field::<bool>(Variant(_27.fld4.3, 0), 0)) = false | true;
_27.fld6.2 = !_20;
(*_16) = _6 as f64;
(*_16) = _11;
_27.fld0 = !Field::<bool>(Variant(_27.fld4.3, 0), 0);
_27.fld6.1 = Adt19::Variant2 { fld0: _7,fld1: 7098511140274012999_usize,fld2: _14,fld3: (-121_i8),fld4: _1 };
_27.fld2.1 = _4 as u32;
_28 = _27.fld6.3.1 as f32;
(*_16) = RET as f64;
_27.fld2.3 = (*_16) as f32;
_27.fld2.2 = Adt17::Variant0 { fld0: _27.fld2.1,fld1: _25,fld2: _4,fld3: 3_usize,fld4: _2,fld5: Field::<u8>(Variant(_27.fld6.1, 2), 2),fld6: 143506710178900930047024463104691377214_u128 };
(*_16) = -_11;
_18 = _14 + Field::<u8>(Variant(_27.fld6.1, 2), 2);
place!(Field::<usize>(Variant(_27.fld2.2, 0), 3)) = Field::<u8>(Variant(_27.fld6.1, 2), 2) as usize;
(*_16) = (*_22) as f64;
_27.fld2.1 = !_27.fld6.3.1;
(*_16) = _11 + _11;
(*_16) = _11 * _11;
place!(Field::<i16>(Variant(_27.fld2.2, 0), 4)) = 80_i8 as i16;
_29 = Adt34 { fld0: _12 };
_27.fld6.1 = Adt19::Variant2 { fld0: (*_22),fld1: Field::<usize>(Variant(_27.fld2.2, 0), 3),fld2: _18,fld3: (-4_i8),fld4: Field::<i128>(Variant(_27.fld4.3, 0), 1) };
_22 = &place!(Field::<i32>(Variant(_27.fld6.1, 2), 0));
match _4 {
0 => bb1,
1 => bb6,
2 => bb14,
3 => bb16,
4 => bb5,
5 => bb22,
73 => bb24,
_ => bb23
}
}
bb28 = {
(*_16) = _11 - _11;
(*_16) = _11;
_11 = _5 as f64;
_17 = _11 * _11;
(*_16) = _11;
(*_16) = _11 - _11;
RET = 7_usize as i16;
_9 = _2 as i64;
(*_16) = _11 * _11;
place!(Field::<bool>(Variant(_10, 0), 0)) = false | true;
(*_16) = Field::<u64>(Variant(_10, 0), 2) as f64;
_22 = &(*_15);
(*_16) = _11 * _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = _1 | _1;
_16 = core::ptr::addr_of_mut!(_17);
(*_16) = _11 + _11;
place!(Field::<i128>(Variant(_10, 0), 1)) = (*_22) as i128;
_19 = _12;
match _4 {
0 => bb10,
1 => bb2,
2 => bb15,
73 => bb19,
_ => bb17
}
}
bb29 = {
_27.fld6.1 = Adt19::Variant2 { fld0: _20,fld1: Field::<usize>(Variant(_27.fld4.3, 2), 1),fld2: _18,fld3: 120_i8,fld4: Field::<i128>(Variant(_27.fld4.3, 2), 4) };
_27.fld6.3.3 = _27.fld2.3;
match Field::<isize>(Variant(_27.fld2.2, 0), 2) {
73 => bb31,
_ => bb30
}
}
bb30 = {
_23 = _4 * _4;
_7 = (*_22);
(*_16) = _23 as f64;
_15 = &_7;
(*_16) = _11 + _11;
_7 = _3;
_14 = !73_u8;
_9 = _6 | _6;
(*_16) = _14 as f64;
Goto(bb20)
}
bb31 = {
(*_16) = _28 as f64;
_4 = Field::<isize>(Variant(_27.fld2.2, 0), 2) & _23;
_21 = _9;
_27.fld4.1 = Field::<isize>(Variant(_27.fld2.2, 0), 2);
place!(Field::<isize>(Variant(_27.fld2.2, 0), 2)) = _4 | _23;
_2 = RET;
_25 = Field::<char>(Variant(_27.fld2.2, 0), 1);
Call(place!(Field::<u8>(Variant(_27.fld2.2, 0), 5)) = core::intrinsics::bswap(Field::<u8>(Variant(_27.fld6.1, 2), 2)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_23 = Field::<isize>(Variant(_27.fld2.2, 0), 2) | Field::<isize>(Variant(_27.fld2.2, 0), 2);
_27.fld6.2 = Field::<i32>(Variant(_27.fld6.1, 2), 0) ^ _3;
place!(Field::<u8>(Variant(_27.fld6.1, 2), 2)) = Field::<u8>(Variant(_27.fld4.3, 2), 2) | _14;
_22 = &place!(Field::<i32>(Variant(_27.fld4.3, 2), 0));
_27.fld2.2 = Adt17::Variant2 { fld0: (*_16),fld1: 12795_u16,fld2: Field::<u8>(Variant(_27.fld6.1, 2), 2) };
_2 = _35 & _35;
_34 = (*_16) - (*_16);
_5 = 38_i8 as u32;
_27.fld3 = core::ptr::addr_of!(_9);
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = Field::<i128>(Variant(_10, 0), 1) & Field::<i128>(Variant(_10, 0), 1);
match _27.fld4.1 {
0 => bb15,
1 => bb12,
2 => bb19,
3 => bb4,
4 => bb33,
73 => bb35,
_ => bb34
}
}
bb33 = {
_2 = RET << _4;
_5 = 1865440090_u32 >> _2;
_6 = _4 as i64;
_7 = _2 as i32;
_7 = _3 << _2;
match RET {
0 => bb3,
1 => bb4,
340282366920938463463374607431768189389 => bb6,
_ => bb5
}
}
bb34 = {
Return()
}
bb35 = {
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = _2 as i128;
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = _1 >> _1;
_37 = !293560646365197235411319235358173103172_u128;
_26 = _29.fld0;
_12 = _19;
_17 = _34;
place!(Field::<i32>(Variant(_27.fld6.1, 2), 0)) = _9 as i32;
_27.fld2.0 = !_27.fld6.3.1;
(*_16) = -_11;
_27.fld4.3 = Adt19::Variant3 { fld0: Field::<bool>(Variant(_10, 0), 0),fld1: Field::<usize>(Variant(_27.fld6.1, 2), 1),fld2: _27.fld2.0,fld3: 104_i8,fld4: Move(_27.fld3),fld5: Field::<i32>(Variant(_27.fld6.1, 2), 0),fld6: _9,fld7: 40658_u16 };
place!(Field::<f64>(Variant(_27.fld2.2, 2), 0)) = (*_16) * _34;
(*_16) = -Field::<f64>(Variant(_27.fld2.2, 2), 0);
_4 = 33204_u16 as isize;
_41.0 = core::ptr::addr_of_mut!((*_16));
_40 = Field::<bool>(Variant(_27.fld4.3, 3), 0);
_24 = _40;
(*_16) = _34 * _34;
_33 = !_24;
(*_16) = -_34;
_22 = &_20;
Goto(bb36)
}
bb36 = {
_27.fld2.0 = !_27.fld6.3.1;
_18 = Field::<u8>(Variant(_27.fld2.2, 2), 2);
place!(Field::<u16>(Variant(_27.fld2.2, 2), 1)) = 59165_u16 + 31854_u16;
_27.fld4.3 = Adt19::Variant2 { fld0: (*_22),fld1: Field::<usize>(Variant(_27.fld6.1, 2), 1),fld2: Field::<u8>(Variant(_27.fld2.2, 2), 2),fld3: 73_i8,fld4: Field::<i128>(Variant(_27.fld6.1, 2), 4) };
_30 = _29.fld0;
_27.fld6.3.3 = _28 * _28;
_33 = !_27.fld0;
_27.fld6.3 = (_27.fld2.0, _5, _27.fld2.2, _28);
_27.fld4.3 = Adt19::Variant2 { fld0: (*_22),fld1: Field::<usize>(Variant(_27.fld6.1, 2), 1),fld2: Field::<u8>(Variant(_27.fld6.3.2, 2), 2),fld3: (-49_i8),fld4: _1 };
_27.fld0 = Field::<i128>(Variant(_27.fld4.3, 2), 4) != Field::<i128>(Variant(_27.fld6.1, 2), 4);
(*_16) = _20 as f64;
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = -Field::<i128>(Variant(_27.fld4.3, 2), 4);
(*_16) = Field::<usize>(Variant(_27.fld4.3, 2), 1) as f64;
_17 = Field::<f64>(Variant(_27.fld2.2, 2), 0) - Field::<f64>(Variant(_27.fld6.3.2, 2), 0);
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = -Field::<i128>(Variant(_10, 0), 1);
place!(Field::<i32>(Variant(_27.fld4.3, 2), 0)) = (*_22);
_44 = (*_22);
_18 = Field::<u8>(Variant(_27.fld6.1, 2), 2) ^ Field::<u8>(Variant(_27.fld6.1, 2), 2);
_43.0 = -(*_16);
place!(Field::<u8>(Variant(_27.fld6.1, 2), 2)) = _18 << _44;
place!(Field::<i8>(Variant(_27.fld4.3, 2), 3)) = (-78_i8) & (-28_i8);
(*_16) = (*_22) as f64;
_27.fld6.3.3 = _28 + _28;
Goto(bb37)
}
bb37 = {
_27.fld7 = (*_16) * (*_16);
_27.fld4.1 = _23;
_17 = -Field::<f64>(Variant(_27.fld6.3.2, 2), 0);
_35 = _2 + _2;
_30 = _27.fld1;
place!(Field::<u8>(Variant(_27.fld2.2, 2), 2)) = _37 as u8;
place!(Field::<i32>(Variant(_27.fld4.3, 2), 0)) = _9 as i32;
_30 = _26;
place!(Field::<u64>(Variant(_10, 0), 2)) = 15116368243027774893_u64 + 16990374683589504053_u64;
_27.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_27.fld6.3.2, 2), 1)));
_35 = _2 - _2;
place!(Field::<u8>(Variant(_27.fld6.3.2, 2), 2)) = Field::<u8>(Variant(_27.fld4.3, 2), 2) ^ Field::<u8>(Variant(_27.fld6.1, 2), 2);
_21 = _6 | _9;
_27.fld7 = (*_22) as f64;
place!(Field::<bool>(Variant(_10, 0), 0)) = (*_22) != Field::<i32>(Variant(_27.fld6.1, 2), 0);
place!(Field::<i128>(Variant(_27.fld6.1, 2), 4)) = -Field::<i128>(Variant(_27.fld4.3, 2), 4);
_25 = _30;
_16 = Move(_41.0);
_20 = _27.fld6.2 + Field::<i32>(Variant(_27.fld4.3, 2), 0);
_27.fld6.0 = _29.fld0;
_41.0 = Move(_16);
_27.fld6.0 = _12;
_27.fld4.3 = Move(_10);
Goto(bb38)
}
bb38 = {
_14 = Field::<u8>(Variant(_27.fld6.3.2, 2), 2);
Goto(bb39)
}
bb39 = {
place!(Field::<i128>(Variant(_27.fld4.3, 0), 1)) = _1 >> Field::<i32>(Variant(_27.fld6.1, 2), 0);
_10 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_27.fld4.3, 0), 0),fld1: Field::<i128>(Variant(_27.fld4.3, 0), 1),fld2: Field::<u64>(Variant(_27.fld4.3, 0), 2) };
_27.fld2.0 = _27.fld6.3.3 as u32;
_26 = _29.fld0;
_27.fld6.3.2 = Adt17::Variant2 { fld0: _34,fld1: Field::<u16>(Variant(_27.fld2.2, 2), 1),fld2: _18 };
Goto(bb40)
}
bb40 = {
_41.1 = &_19;
_23 = _37 as isize;
_27.fld2.1 = _27.fld6.3.0 & _27.fld2.0;
Goto(bb41)
}
bb41 = {
_27.fld6.0 = _29.fld0;
_6 = -_9;
_19 = _30;
_18 = !_14;
_53.0 = &_25;
_27.fld6.1 = Move(_27.fld4.3);
_2 = _35;
_29 = Adt34 { fld0: _25 };
_49 = core::ptr::addr_of!(_27.fld4.0);
_55 = Field::<u64>(Variant(_10, 0), 2) as i128;
_55 = Field::<i128>(Variant(_27.fld6.1, 0), 1);
_48 = _27.fld0 & _33;
_36.0 = (Move(_41.1),);
_27.fld4.3 = Move(_27.fld6.1);
_43.1 = &_27.fld6.3;
_27.fld0 = Field::<bool>(Variant(_10, 0), 0) > Field::<bool>(Variant(_27.fld4.3, 0), 0);
_27.fld4.3 = Adt19::Variant0 { fld0: _27.fld0,fld1: Field::<i128>(Variant(_10, 0), 1),fld2: Field::<u64>(Variant(_10, 0), 2) };
_55 = Field::<i128>(Variant(_27.fld4.3, 0), 1);
_45.0 = &_26;
_45 = Move(_53);
RET = -_2;
_56 = _35 as f64;
_54 = _37 as f64;
(*_49) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_27.fld6.3.2, 2), 1)));
Goto(bb42)
}
bb42 = {
Call(_62 = dump_var(Move(_14), Move(_44), Move(_18), Move(_26)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_62 = dump_var(Move(_30), Move(_4), Move(_55), Move(_2)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_62 = dump_var(Move(_21), Move(_7), Move(_20), Move(_33)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_62 = dump_var(Move(_9), _63, _63, _63), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i32) -> *mut f64 {
mir! {
type RET = *mut f64;
let _2: &'static &'static mut [u64; 6];
let _3: char;
let _4: [char; 1];
let _5: u128;
let _6: u64;
let _7: i128;
let _8: *const isize;
let _9: Adt34;
let _10: bool;
let _11: Adt34;
let _12: isize;
let _13: *const &'static Adt19;
let _14: u16;
let _15: &'static mut u128;
let _16: *const f64;
let _17: bool;
let _18: Adt85;
let _19: isize;
let _20: isize;
let _21: *const &'static Adt19;
let _22: Adt85;
let _23: &'static mut char;
let _24: &'static mut char;
let _25: isize;
let _26: u8;
let _27: f32;
let _28: &'static mut usize;
let _29: Adt34;
let _30: u32;
let _31: f32;
let _32: *const &'static Adt19;
let _33: *const Adt17;
let _34: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _35: i32;
let _36: [char; 6];
let _37: isize;
let _38: isize;
let _39: isize;
let _40: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _41: *const (&'static char,);
let _42: bool;
let _43: u16;
let _44: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _45: u8;
let _46: *mut u32;
let _47: *mut u16;
let _48: isize;
let _49: (usize, bool);
let _50: f32;
let _51: Adt85;
let _52: *mut (f64, &'static (u32, u32, Adt17, f32));
let _53: char;
let _54: f32;
let _55: u32;
let _56: *const Adt17;
let _57: [char; 1];
let _58: isize;
let _59: isize;
let _60: (f64, &'static (u32, u32, Adt17, f32));
let _61: *const Adt19;
let _62: bool;
let _63: *mut bool;
let _64: *const Adt60;
let _65: &'static *mut bool;
let _66: isize;
let _67: bool;
let _68: u8;
let _69: &'static mut *const i64;
let _70: &'static f32;
let _71: Adt19;
let _72: i64;
let _73: bool;
let _74: *mut u16;
let _75: ();
let _76: ();
{
_1 = -723912034_i32;
_1 = 54161_u16 as i32;
_1 = (-500935326_i32) - 528215965_i32;
_1 = (-1330147296_i32) - (-1282993930_i32);
_1 = (-1747498258_i32);
_3 = '\u{4693f}';
_1 = 24_isize as i32;
_1 = (-2137866001_i32) >> (-3349508574442522303_i64);
_3 = '\u{24080}';
_3 = '\u{10211f}';
_1 = 1845199022_i32 ^ (-1167264695_i32);
_1 = 2029969278_i32 - 131642387_i32;
_1 = 1515106148_i32;
_3 = '\u{d52c}';
_3 = '\u{6c116}';
_1 = -(-589878436_i32);
_4 = [_3];
_6 = 2391524590273919561_u64;
_5 = 85810678157865780102751474302696843799_u128 >> _6;
_5 = 271779439897014769760203533856283328797_u128;
_3 = '\u{9de9a}';
_5 = false as u128;
_5 = true as u128;
Goto(bb1)
}
bb1 = {
_4 = [_3];
_5 = !18622954926703348439192994590845008511_u128;
_1 = -1778003014_i32;
_5 = 176783256573066854932495068599303535049_u128 << _6;
_3 = '\u{8721c}';
Goto(bb2)
}
bb2 = {
_5 = !338264002071390451791086459227975464773_u128;
_1 = (-448881020_i32) ^ (-586310970_i32);
_3 = '\u{55e3}';
Call(_7 = fn3(_3, _1, _4, _4, _4, _4, _1, _4, _6, _3, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb4 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb5 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb6 = {
_5 = !338264002071390451791086459227975464773_u128;
_1 = (-448881020_i32) ^ (-586310970_i32);
_3 = '\u{55e3}';
Call(_7 = fn3(_3, _1, _4, _4, _4, _4, _1, _4, _6, _3, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = [_3];
_5 = !18622954926703348439192994590845008511_u128;
_1 = -1778003014_i32;
_5 = 176783256573066854932495068599303535049_u128 << _6;
_3 = '\u{8721c}';
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
(*_8) = (-9223372036854775808_isize) - 11_isize;
(*_8) = (-102_isize) - 77_isize;
_1 = 117709381_i32;
(*_8) = (-2515343341748420206_i64) as isize;
(*_8) = 9223372036854775807_isize;
(*_8) = !48_isize;
(*_8) = (-9223372036854775808_isize) >> _5;
_8 = core::ptr::addr_of!((*_8));
(*_8) = (-9223372036854775808_isize);
(*_8) = (-9223372036854775808_isize) >> _1;
_11 = Adt34 { fld0: _9.fld0 };
match _1 {
117709381 => bb10,
_ => bb6
}
}
bb10 = {
_7 = 21553031598715273191345282081552916603_i128;
match _14 {
0 => bb5,
1 => bb11,
41867 => bb13,
_ => bb12
}
}
bb11 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb12 = {
_4 = [_3];
_5 = !18622954926703348439192994590845008511_u128;
_1 = -1778003014_i32;
_5 = 176783256573066854932495068599303535049_u128 << _6;
_3 = '\u{8721c}';
Goto(bb2)
}
bb13 = {
(*_8) = (-9223372036854775808_isize) - 25_isize;
(*_8) = _3 as isize;
(*_8) = 77_isize - (-9223372036854775808_isize);
(*_8) = (-108_isize) + 77_isize;
(*_8) = (-9223372036854775808_isize);
(*_8) = (-46_isize) << _5;
_17 = (*_8) <= (*_8);
_20 = (*_8);
(*_8) = _20;
_5 = 24494_i16 as u128;
_10 = _17;
(*_8) = _20 << _20;
_3 = _9.fld0;
(*_8) = !_20;
(*_8) = _20 - _20;
_20 = 4022369044915260884_i64 as isize;
(*_8) = _20;
(*_8) = _20;
(*_8) = _20 | _20;
match _6 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
16448090900554620339 => bb19,
_ => bb18
}
}
bb14 = {
_4 = [_3];
_5 = !18622954926703348439192994590845008511_u128;
_1 = -1778003014_i32;
_5 = 176783256573066854932495068599303535049_u128 << _6;
_3 = '\u{8721c}';
Goto(bb2)
}
bb15 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb16 = {
_7 = 21553031598715273191345282081552916603_i128;
match _14 {
0 => bb5,
1 => bb11,
41867 => bb13,
_ => bb12
}
}
bb17 = {
_5 = !338264002071390451791086459227975464773_u128;
_1 = (-448881020_i32) ^ (-586310970_i32);
_3 = '\u{55e3}';
Call(_7 = fn3(_3, _1, _4, _4, _4, _4, _1, _4, _6, _3, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb18 = {
_5 = !338264002071390451791086459227975464773_u128;
_1 = (-448881020_i32) ^ (-586310970_i32);
_3 = '\u{55e3}';
Call(_7 = fn3(_3, _1, _4, _4, _4, _4, _1, _4, _6, _3, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_11 = Adt34 { fld0: _3 };
(*_8) = 4_usize as isize;
(*_8) = -_20;
_11.fld0 = _3;
_24 = &mut _3;
Goto(bb20)
}
bb20 = {
_24 = &mut _11.fld0;
_23 = &mut (*_24);
(*_23) = _9.fld0;
(*_23) = _9.fld0;
Goto(bb21)
}
bb21 = {
_1 = 2006310548_i32 & 1600055426_i32;
(*_23) = _9.fld0;
(*_8) = _20;
(*_8) = _20 | _20;
(*_23) = _9.fld0;
(*_8) = _14 as isize;
(*_23) = _9.fld0;
(*_23) = _9.fld0;
(*_8) = _20;
_29.fld0 = (*_23);
(*_8) = 5105727537908702744_i64 as isize;
_15 = &mut _5;
_9 = Adt34 { fld0: _29.fld0 };
(*_8) = (*_23) as isize;
(*_8) = _20 - _20;
(*_23) = _29.fld0;
(*_23) = _29.fld0;
(*_15) = 91749126869536208313079810752384077364_u128 * 274936977995335280079672870510951064272_u128;
(*_23) = _9.fld0;
_30 = !2291701417_u32;
(*_8) = _20 & _20;
_29.fld0 = (*_23);
match _7 {
0 => bb13,
1 => bb14,
2 => bb22,
3 => bb23,
21553031598715273191345282081552916603 => bb25,
_ => bb24
}
}
bb22 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb23 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb24 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb25 = {
(*_23) = _29.fld0;
(*_23) = _29.fld0;
(*_8) = 222_u8 as isize;
(*_23) = _29.fld0;
_10 = _17;
(*_23) = _29.fld0;
(*_15) = _6 as u128;
(*_8) = _20;
_25 = _6 as isize;
(*_23) = _9.fld0;
_27 = (*_15) as f32;
(*_15) = 275393379165519137720283802439776514302_u128 << (*_8);
(*_15) = 15795133531625442861201907621185143611_u128 & 55620555347998653424217249829716851153_u128;
(*_15) = !114767652955856229911847748910778065752_u128;
_12 = !_20;
(*_15) = 221044556760041030828752675762373596822_u128;
(*_15) = 44546864613692323683256988259896436116_u128 - 92300898674580114122465532942506615302_u128;
(*_8) = 43_u8 as isize;
(*_15) = 150675969962646881416545161937655226293_u128 * 50818341770826258787412608134848994576_u128;
(*_23) = _9.fld0;
(*_15) = 68278192150440494744464283185866783838_u128;
(*_15) = 77839351974199061680767754640810597037_u128 * 75951261354492341654458914870814110911_u128;
(*_15) = 237119214036969497006115094888436335787_u128 << (*_8);
(*_15) = 154686626911492461070037886952126449733_u128;
(*_23) = _29.fld0;
(*_23) = _9.fld0;
(*_15) = (*_23) as u128;
match _7 {
0 => bb16,
1 => bb2,
2 => bb22,
3 => bb17,
21553031598715273191345282081552916603 => bb27,
_ => bb26
}
}
bb26 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb27 = {
_24 = &mut _29.fld0;
(*_23) = (*_24);
(*_15) = 50765231837432559037981265133489890391_u128 | 258743653729613932214431311097152263904_u128;
(*_24) = (*_23);
(*_24) = (*_23);
(*_15) = 211618087613285853551317274657111363471_u128 >> (*_8);
Goto(bb28)
}
bb28 = {
_25 = -(*_8);
(*_8) = _20 ^ _20;
(*_15) = 3878127140962851893312391603183173917_u128 >> _30;
match _6 {
16448090900554620339 => bb30,
_ => bb29
}
}
bb29 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb30 = {
(*_23) = (*_24);
(*_23) = (*_24);
(*_8) = _25;
_34.3 = Adt50 { fld0: (*_15) };
(*_15) = _34.3.fld0 & _34.3.fld0;
(*_8) = _25 * _20;
(*_8) = _14 as isize;
match _14 {
0 => bb7,
1 => bb16,
2 => bb29,
3 => bb6,
41867 => bb31,
_ => bb20
}
}
bb31 = {
(*_24) = (*_23);
(*_23) = (*_24);
(*_23) = (*_24);
(*_8) = (*_24) as isize;
_17 = _10;
(*_15) = _34.3.fld0 >> (*_8);
(*_15) = _34.3.fld0;
(*_8) = !_20;
_9.fld0 = (*_24);
(*_8) = _20;
(*_8) = _20 | _20;
_34.1 = [_6,_6,_6,_6,_6,_6];
(*_24) = (*_23);
(*_15) = _34.3.fld0 + _34.3.fld0;
(*_15) = _34.3.fld0;
_8 = core::ptr::addr_of!(_37);
(*_8) = _25;
Call((*_15) = core::intrinsics::bswap(_34.3.fld0), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
(*_15) = _34.3.fld0;
_39 = (*_8);
(*_15) = _34.3.fld0;
_19 = -_39;
(*_23) = (*_24);
(*_24) = (*_23);
(*_23) = (*_24);
(*_24) = (*_23);
(*_23) = (*_24);
(*_8) = _19;
_12 = _20 * (*_8);
match _6 {
0 => bb21,
16448090900554620339 => bb33,
_ => bb16
}
}
bb33 = {
(*_23) = (*_24);
(*_15) = _34.3.fld0 & _34.3.fld0;
_26 = 12_u8 ^ 73_u8;
(*_24) = (*_23);
(*_24) = (*_23);
_14 = 48519_u16;
(*_15) = _34.3.fld0;
(*_24) = (*_23);
(*_8) = -_39;
(*_8) = _12 | _39;
(*_8) = _39 + _25;
(*_15) = 10420_i16 as u128;
(*_8) = _12 * _20;
_24 = &mut (*_23);
_38 = _19 << _1;
_31 = _27 * _27;
_15 = &mut _34.3.fld0;
_9 = Adt34 { fld0: (*_24) };
(*_15) = 93_i8 as u128;
_43 = _7 as u16;
(*_24) = _9.fld0;
Goto(bb34)
}
bb34 = {
(*_24) = _9.fld0;
_35 = -_1;
match _6 {
0 => bb14,
1 => bb7,
2 => bb3,
3 => bb15,
4 => bb35,
5 => bb36,
16448090900554620339 => bb38,
_ => bb37
}
}
bb35 = {
(*_23) = _29.fld0;
(*_23) = _29.fld0;
(*_8) = 222_u8 as isize;
(*_23) = _29.fld0;
_10 = _17;
(*_23) = _29.fld0;
(*_15) = _6 as u128;
(*_8) = _20;
_25 = _6 as isize;
(*_23) = _9.fld0;
_27 = (*_15) as f32;
(*_15) = 275393379165519137720283802439776514302_u128 << (*_8);
(*_15) = 15795133531625442861201907621185143611_u128 & 55620555347998653424217249829716851153_u128;
(*_15) = !114767652955856229911847748910778065752_u128;
_12 = !_20;
(*_15) = 221044556760041030828752675762373596822_u128;
(*_15) = 44546864613692323683256988259896436116_u128 - 92300898674580114122465532942506615302_u128;
(*_8) = 43_u8 as isize;
(*_15) = 150675969962646881416545161937655226293_u128 * 50818341770826258787412608134848994576_u128;
(*_23) = _9.fld0;
(*_15) = 68278192150440494744464283185866783838_u128;
(*_15) = 77839351974199061680767754640810597037_u128 * 75951261354492341654458914870814110911_u128;
(*_15) = 237119214036969497006115094888436335787_u128 << (*_8);
(*_15) = 154686626911492461070037886952126449733_u128;
(*_23) = _29.fld0;
(*_23) = _9.fld0;
(*_15) = (*_23) as u128;
match _7 {
0 => bb16,
1 => bb2,
2 => bb22,
3 => bb17,
21553031598715273191345282081552916603 => bb27,
_ => bb26
}
}
bb36 = {
_24 = &mut _11.fld0;
_23 = &mut (*_24);
(*_23) = _9.fld0;
(*_23) = _9.fld0;
Goto(bb21)
}
bb37 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb38 = {
_44.0 = (*_24);
_47 = core::ptr::addr_of_mut!(_14);
_44.3.0 = 3396795456112909287_i64 as u32;
_33 = core::ptr::addr_of!(_44.3.2);
(*_47) = !_43;
_44.3.1 = _44.3.0 ^ _44.3.0;
(*_33) = Adt17::Variant0 { fld0: _44.3.1,fld1: (*_24),fld2: (*_8),fld3: 17924967485941496221_usize,fld4: 29574_i16,fld5: _26,fld6: (*_15) };
(*_8) = !Field::<isize>(Variant((*_33), 0), 2);
_38 = Field::<isize>(Variant((*_33), 0), 2) << (*_15);
place!(Field::<i16>(Variant((*_33), 0), 4)) = (-3052_i16);
match Field::<i16>(Variant((*_33), 0), 4) {
0 => bb11,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb36,
340282366920938463463374607431768208404 => bb40,
_ => bb39
}
}
bb39 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb40 = {
(*_33) = Adt17::Variant0 { fld0: _44.3.1,fld1: (*_24),fld2: (*_8),fld3: 2848180332303378910_usize,fld4: (-11827_i16),fld5: _26,fld6: (*_15) };
place!(Field::<usize>(Variant((*_33), 0), 3)) = 0_usize << (*_15);
place!(Field::<usize>(Variant((*_33), 0), 3)) = 894517821673130919_usize;
place!(Field::<u128>(Variant((*_33), 0), 6)) = !(*_15);
(*_24) = Field::<char>(Variant((*_33), 0), 1);
(*_33) = Adt17::Variant0 { fld0: _30,fld1: _9.fld0,fld2: (*_8),fld3: 14002209822620518901_usize,fld4: 19454_i16,fld5: _26,fld6: (*_15) };
(*_8) = Field::<isize>(Variant((*_33), 0), 2) + Field::<isize>(Variant(_44.3.2, 0), 2);
place!(Field::<char>(Variant((*_33), 0), 1)) = (*_24);
(*_15) = Field::<u128>(Variant((*_33), 0), 6);
(*_24) = Field::<char>(Variant((*_33), 0), 1);
place!(Field::<usize>(Variant((*_33), 0), 3)) = 7_usize & 0_usize;
place!(Field::<i16>(Variant((*_33), 0), 4)) = (-16453_i16) & (-9412_i16);
(*_47) = !_43;
place!(Field::<u8>(Variant((*_33), 0), 5)) = (*_24) as u8;
_9 = Adt34 { fld0: (*_24) };
_8 = core::ptr::addr_of!((*_8));
(*_15) = Field::<u128>(Variant((*_33), 0), 6) >> Field::<isize>(Variant((*_33), 0), 2);
place!(Field::<u128>(Variant((*_33), 0), 6)) = !(*_15);
place!(Field::<u32>(Variant((*_33), 0), 0)) = Field::<u8>(Variant((*_33), 0), 5) as u32;
_46 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant((*_33), 0), 0)));
place!(Field::<isize>(Variant((*_33), 0), 2)) = (*_8);
(*_46) = 5356094874968371637_i64 as u32;
(*_47) = _43 >> Field::<u128>(Variant((*_33), 0), 6);
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15);
(*_47) = !_43;
place!(Field::<usize>(Variant((*_33), 0), 3)) = 2_usize + 1324456933735158368_usize;
(*_24) = Field::<char>(Variant((*_33), 0), 1);
place!(Field::<u32>(Variant((*_33), 0), 0)) = _44.3.0 * _44.3.1;
place!(Field::<u128>(Variant((*_33), 0), 6)) = !(*_15);
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15) ^ (*_15);
match _6 {
0 => bb36,
1 => bb2,
2 => bb26,
3 => bb14,
4 => bb41,
16448090900554620339 => bb43,
_ => bb42
}
}
bb41 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb42 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb43 = {
(*_33) = Adt17::Variant0 { fld0: _44.3.0,fld1: (*_24),fld2: (*_8),fld3: 4_usize,fld4: (-24171_i16),fld5: _26,fld6: (*_15) };
place!(Field::<usize>(Variant((*_33), 0), 3)) = 7351393643613783314_usize + 4973423223361924607_usize;
(*_33) = Adt17::Variant0 { fld0: _30,fld1: (*_24),fld2: (*_8),fld3: 3615597607267387350_usize,fld4: (-20525_i16),fld5: _26,fld6: (*_15) };
(*_47) = _43;
(*_33) = Adt17::Variant1 { fld0: _7,fld1: _6,fld2: _31 };
_44.2 = _1 * _35;
_43 = (*_47);
(*_47) = _43;
(*_8) = !_39;
_1 = 7108716370626116518_i64 as i32;
place!(Field::<i128>(Variant((*_33), 1), 0)) = _7 >> (*_15);
_49.1 = _10;
place!(Field::<i128>(Variant((*_33), 1), 0)) = -_7;
place!(Field::<f32>(Variant((*_33), 1), 2)) = _26 as f32;
_7 = Field::<i128>(Variant(_44.3.2, 1), 0) * Field::<i128>(Variant(_44.3.2, 1), 0);
(*_24) = _9.fld0;
(*_47) = 5531421273574525044_i64 as u16;
Goto(bb44)
}
bb44 = {
_49 = Checked(6055409329797370807_usize * 2070364620064431661_usize);
place!(Field::<i128>(Variant((*_33), 1), 0)) = !_7;
(*_47) = !_43;
place!(Field::<f32>(Variant((*_33), 1), 2)) = _30 as f32;
place!(Field::<i128>(Variant((*_33), 1), 0)) = Field::<f32>(Variant((*_33), 1), 2) as i128;
place!(Field::<i128>(Variant((*_33), 1), 0)) = _7 & _7;
_28 = &mut _49.0;
_24 = &mut _44.0;
(*_8) = _38 * _12;
match Field::<u64>(Variant((*_33), 1), 1) {
0 => bb45,
1 => bb46,
16448090900554620339 => bb48,
_ => bb47
}
}
bb45 = {
_14 = 41867_u16;
_6 = 16448090900554620339_u64;
_5 = 12701681709675505895507305258504577845_u128;
_7 = 22018125850410271667146630755052192383_i128 << (*_8);
(*_8) = 9223372036854775807_isize - (-9223372036854775808_isize);
_9 = Move(_11);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
16448090900554620339 => bb9,
_ => bb8
}
}
bb46 = {
_5 = !284465127885838619452120485856247269104_u128;
_1 = (-1432414502_i32) ^ (-1581214629_i32);
_5 = 263365620352362983128461810810162380221_u128 & 56752112894265712625866320313866165138_u128;
_3 = '\u{7b16f}';
_6 = 8099476314800770816_u64 >> _7;
_4 = [_3];
_9.fld0 = _3;
_4 = [_3];
_3 = _9.fld0;
_4 = [_3];
_9 = Adt34 { fld0: _3 };
_5 = !254233031761923901659533573016171200442_u128;
_4 = [_9.fld0];
_3 = _9.fld0;
_6 = 3374788529408083932_u64;
_10 = false;
_11 = Adt34 { fld0: _9.fld0 };
_4 = [_9.fld0];
_6 = 1713101048583183296_u64 * 11914567617414131020_u64;
_6 = 15681669734821790376_u64 ^ 6526384058692585139_u64;
_8 = core::ptr::addr_of!(_12);
(*_8) = (-48_isize);
_9.fld0 = _3;
Goto(bb4)
}
bb47 = {
_5 = !338264002071390451791086459227975464773_u128;
_1 = (-448881020_i32) ^ (-586310970_i32);
_3 = '\u{55e3}';
Call(_7 = fn3(_3, _1, _4, _4, _4, _4, _1, _4, _6, _3, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb48 = {
_25 = (*_8);
place!(Field::<u64>(Variant((*_33), 1), 1)) = _6 / _6;
_19 = (*_8) + (*_8);
place!(Field::<f32>(Variant((*_33), 1), 2)) = -_31;
(*_47) = !_43;
(*_15) = 17889_i16 as u128;
(*_33) = Adt17::Variant0 { fld0: _30,fld1: (*_24),fld2: (*_8),fld3: (*_28),fld4: 6613_i16,fld5: _26,fld6: (*_15) };
_20 = (*_8) | Field::<isize>(Variant((*_33), 0), 2);
(*_28) = Field::<usize>(Variant((*_33), 0), 3) << Field::<isize>(Variant((*_33), 0), 2);
_54 = -_27;
_23 = &mut place!(Field::<char>(Variant((*_33), 0), 1));
place!(Field::<isize>(Variant((*_33), 0), 2)) = -(*_8);
_42 = _10 | _10;
Goto(bb49)
}
bb49 = {
place!(Field::<i16>(Variant((*_33), 0), 4)) = -4221_i16;
(*_24) = (*_23);
_58 = Field::<u8>(Variant((*_33), 0), 5) as isize;
(*_47) = !_43;
place!(Field::<isize>(Variant((*_33), 0), 2)) = Field::<i16>(Variant((*_33), 0), 4) as isize;
_60.0 = Field::<i16>(Variant((*_33), 0), 4) as f64;
(*_28) = Field::<usize>(Variant((*_33), 0), 3);
(*_28) = Field::<usize>(Variant((*_33), 0), 3);
place!(Field::<u128>(Variant((*_33), 0), 6)) = _17 as u128;
(*_24) = (*_23);
place!(Field::<isize>(Variant((*_33), 0), 2)) = (*_8) ^ (*_8);
(*_47) = _43;
_58 = 4166977830691058876_i64 as isize;
(*_47) = _43 >> (*_15);
match _6 {
0 => bb10,
1 => bb35,
16448090900554620339 => bb50,
_ => bb29
}
}
bb50 = {
place!(Field::<i16>(Variant((*_33), 0), 4)) = _60.0 as i16;
place!(Field::<u8>(Variant((*_33), 0), 5)) = _26 - _26;
(*_47) = !_43;
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) ^ (*_28);
(*_8) = _19 << Field::<isize>(Variant((*_33), 0), 2);
(*_15) = Field::<u128>(Variant((*_33), 0), 6) >> (*_8);
(*_28) = Field::<usize>(Variant((*_33), 0), 3) - Field::<usize>(Variant((*_33), 0), 3);
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) - (*_28);
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) ^ (*_28);
(*_47) = !_43;
(*_23) = (*_24);
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15) ^ (*_15);
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) - (*_28);
place!(Field::<u32>(Variant((*_33), 0), 0)) = _30 & _30;
(*_47) = !_43;
(*_8) = Field::<isize>(Variant((*_33), 0), 2) | _19;
(*_47) = _43 >> (*_15);
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15) >> (*_15);
place!(Field::<u32>(Variant((*_33), 0), 0)) = !_30;
match _6 {
0 => bb15,
16448090900554620339 => bb51,
_ => bb31
}
}
bb51 = {
_58 = Field::<isize>(Variant((*_33), 0), 2);
(*_23) = (*_24);
(*_23) = (*_24);
place!(Field::<usize>(Variant((*_33), 0), 3)) = 73_i8 as usize;
(*_15) = Field::<u128>(Variant((*_33), 0), 6);
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15);
place!(Field::<i16>(Variant((*_33), 0), 4)) = (-2222_i16);
_53 = (*_23);
_36 = [(*_23),(*_23),(*_23),(*_23),(*_24),(*_23)];
_60.0 = (-5146933514711770375_i64) as f64;
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) & (*_28);
place!(Field::<usize>(Variant((*_33), 0), 3)) = (*_28) << _30;
_36 = [(*_23),(*_23),(*_24),(*_23),(*_24),(*_23)];
_19 = Field::<u32>(Variant((*_33), 0), 0) as isize;
match Field::<i16>(Variant((*_33), 0), 4) {
0 => bb49,
340282366920938463463374607431768209234 => bb52,
_ => bb22
}
}
bb52 = {
_66 = -Field::<isize>(Variant((*_33), 0), 2);
(*_23) = (*_24);
place!(Field::<u32>(Variant((*_33), 0), 0)) = _60.0 as u32;
_24 = Move(_23);
(*_8) = Field::<isize>(Variant((*_33), 0), 2) ^ Field::<isize>(Variant((*_33), 0), 2);
place!(Field::<u32>(Variant((*_33), 0), 0)) = _30;
place!(Field::<u32>(Variant((*_33), 0), 0)) = !_30;
RET = core::ptr::addr_of_mut!(_60.0);
(*_15) = Field::<u128>(Variant((*_33), 0), 6) - Field::<u128>(Variant((*_33), 0), 6);
(*RET) = _35 as f64;
place!(Field::<u128>(Variant((*_33), 0), 6)) = (*_15) * (*_15);
(*_8) = Field::<u128>(Variant((*_33), 0), 6) as isize;
(*_47) = _43 | _43;
place!(Field::<u128>(Variant((*_33), 0), 6)) = !(*_15);
place!(Field::<isize>(Variant((*_33), 0), 2)) = -(*_8);
(*_15) = _9.fld0 as u128;
(*RET) = Field::<isize>(Variant((*_33), 0), 2) as f64;
_9 = Adt34 { fld0: _53 };
_12 = (*_8) * (*_8);
place!(Field::<i16>(Variant((*_33), 0), 4)) = 8482_i16;
Goto(bb53)
}
bb53 = {
Call(_75 = dump_var(Move(_26), Move(_36), Move(_14), Move(_35)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_75 = dump_var(Move(_17), Move(_19), Move(_58), Move(_5)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_75 = dump_var(Move(_20), Move(_42), Move(_30), Move(_39)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_75 = dump_var(Move(_38), _76, _76, _76), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: i32,mut _3: [char; 1],mut _4: [char; 1],mut _5: [char; 1],mut _6: [char; 1],mut _7: i32,mut _8: [char; 1],mut _9: u64,mut _10: char,mut _11: char,mut _12: [char; 1]) -> i128 {
mir! {
type RET = i128;
let _13: u64;
let _14: *mut [char; 6];
let _15: isize;
let _16: (*mut f64, &'static char);
let _17: Adt40;
let _18: &'static (f64, &'static (u32, u32, Adt17, f32));
let _19: [u16; 1];
let _20: char;
let _21: char;
let _22: &'static *const *mut u16;
let _23: *mut bool;
let _24: f64;
let _25: u16;
let _26: char;
let _27: bool;
let _28: u16;
let _29: *const bool;
let _30: i8;
let _31: isize;
let _32: *const &'static Adt19;
let _33: i64;
let _34: f32;
let _35: char;
let _36: &'static Adt19;
let _37: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _38: (*mut u16, Adt17);
let _39: Adt17;
let _40: i16;
let _41: *const isize;
let _42: *mut u16;
let _43: &'static &'static mut u128;
let _44: &'static mut char;
let _45: &'static mut &'static mut usize;
let _46: *const i64;
let _47: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _48: *mut *const bool;
let _49: usize;
let _50: &'static f32;
let _51: bool;
let _52: isize;
let _53: &'static *const *mut u16;
let _54: i32;
let _55: Adt17;
let _56: [char; 1];
let _57: isize;
let _58: f32;
let _59: &'static usize;
let _60: *const Adt19;
let _61: bool;
let _62: Adt60;
let _63: u64;
let _64: *mut u16;
let _65: bool;
let _66: &'static mut usize;
let _67: (&'static mut char, f32, Adt40, usize);
let _68: *const *mut u16;
let _69: *const Adt60;
let _70: f32;
let _71: *const i64;
let _72: Adt17;
let _73: ();
let _74: ();
{
_3 = _12;
_2 = (-9223372036854775808_isize) as i32;
Call(_12 = fn4(_6, _11, _7, _4, _9, _10, _6, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = _9 - _9;
_11 = _1;
_12 = [_10];
_3 = _12;
_10 = _11;
_10 = _11;
_3 = _6;
_13 = _9 ^ _9;
_6 = _5;
_2 = 1278078519_u32 as i32;
_4 = [_11];
_7 = _2;
_8 = _5;
_6 = [_1];
_7 = _2;
_3 = [_11];
RET = _10 as i128;
_13 = _9 ^ _9;
_2 = _7 + _7;
_9 = _13 << _13;
_2 = _7 + _7;
_12 = _6;
_11 = _1;
_11 = _1;
_3 = [_1];
_2 = -_7;
_15 = 9223372036854775807_isize;
_5 = [_10];
_5 = _12;
Goto(bb2)
}
bb2 = {
_6 = _4;
_7 = _2 + _2;
_8 = _6;
_2 = _7 - _7;
_16.1 = &_1;
_16.1 = &_11;
_7 = _2 << RET;
_7 = _2 * _2;
_11 = _1;
_16.1 = &_1;
_9 = _13 & _13;
_15 = -9223372036854775807_isize;
_11 = _10;
_15 = (-9223372036854775808_isize);
RET = -(-43510769066990309199578786630508645458_i128);
_16.1 = &_10;
_3 = _4;
_4 = _3;
_17.fld0 = 1378907926_u32 as isize;
Goto(bb3)
}
bb3 = {
_15 = 3693521635_u32 as isize;
_15 = -_17.fld0;
RET = -28598952003776695033729951032140733572_i128;
_7 = -_2;
_19 = [1284_u16];
Goto(bb4)
}
bb4 = {
_12 = [_10];
_13 = _9 & _9;
_1 = _11;
_15 = -_17.fld0;
_10 = _11;
_2 = _7 ^ _7;
_6 = _12;
_1 = _10;
_20 = _10;
_11 = _20;
_13 = _9 & _9;
_21 = _11;
_1 = _10;
_17.fld0 = -_15;
_5 = [_1];
_4 = [_21];
_20 = _21;
_2 = _7;
RET = (-69106614649298137649550624415833131818_i128) | (-60105172882094210977618017668122107735_i128);
_16.1 = &_1;
_3 = _5;
_15 = _17.fld0;
Goto(bb5)
}
bb5 = {
_7 = !_2;
_7 = !_2;
_17.fld1 = core::ptr::addr_of_mut!(_24);
Goto(bb6)
}
bb6 = {
_15 = _7 as isize;
_24 = _15 as f64;
_13 = _9 - _9;
_2 = _7 - _7;
_15 = _17.fld0;
_21 = _11;
_25 = 23761_u16;
_9 = true as u64;
_24 = 45_i8 as f64;
_19 = [_25];
match _25 {
0 => bb7,
23761 => bb9,
_ => bb8
}
}
bb7 = {
_7 = !_2;
_7 = !_2;
_17.fld1 = core::ptr::addr_of_mut!(_24);
Goto(bb6)
}
bb8 = {
_12 = [_10];
_13 = _9 & _9;
_1 = _11;
_15 = -_17.fld0;
_10 = _11;
_2 = _7 ^ _7;
_6 = _12;
_1 = _10;
_20 = _10;
_11 = _20;
_13 = _9 & _9;
_21 = _11;
_1 = _10;
_17.fld0 = -_15;
_5 = [_1];
_4 = [_21];
_20 = _21;
_2 = _7;
RET = (-69106614649298137649550624415833131818_i128) | (-60105172882094210977618017668122107735_i128);
_16.1 = &_1;
_3 = _5;
_15 = _17.fld0;
Goto(bb5)
}
bb9 = {
_8 = [_20];
_16.0 = core::ptr::addr_of_mut!(_24);
_27 = false ^ false;
_11 = _20;
_12 = [_21];
_3 = [_21];
_23 = core::ptr::addr_of_mut!(_27);
_28 = _25 - _25;
_29 = core::ptr::addr_of!((*_23));
_20 = _11;
(*_29) = RET <= RET;
(*_29) = false;
match _25 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
23761 => bb10,
_ => bb8
}
}
bb10 = {
(*_23) = true;
(*_23) = !false;
_12 = [_21];
_6 = _5;
_26 = _10;
_16.1 = &_20;
(*_23) = _13 != _13;
_25 = !_28;
_17 = Adt40 { fld0: _15,fld1: Move(_16.0) };
_25 = _28;
_15 = _17.fld0 << _9;
(*_23) = !false;
(*_23) = true;
_20 = _10;
_21 = _26;
(*_23) = _13 > _13;
_23 = core::ptr::addr_of_mut!((*_23));
_5 = [_11];
(*_23) = !true;
_28 = !_25;
(*_23) = _20 > _1;
_1 = _10;
(*_23) = !false;
_16.0 = core::ptr::addr_of_mut!(_24);
(*_23) = false | true;
_17 = Adt40 { fld0: _15,fld1: Move(_16.0) };
_30 = 331279080186482357370479130784269796033_u128 as i8;
_29 = core::ptr::addr_of!((*_23));
Goto(bb11)
}
bb11 = {
RET = (-137416666132536595482447868585372409807_i128) << _2;
Goto(bb12)
}
bb12 = {
_5 = [_20];
(*_29) = _26 != _11;
_26 = _21;
(*_23) = !true;
_25 = _28 & _28;
(*_23) = true;
(*_23) = false;
(*_23) = _7 < _2;
_11 = _1;
_16.0 = core::ptr::addr_of_mut!(_24);
(*_23) = RET > RET;
_30 = 13253929046919833598_usize as i8;
(*_23) = false;
_13 = _2 as u64;
_28 = !_25;
Goto(bb13)
}
bb13 = {
_20 = _10;
(*_23) = false & true;
_19 = [_28];
_7 = _13 as i32;
(*_23) = false;
(*_23) = RET > RET;
_5 = [_21];
(*_23) = true;
(*_23) = false;
(*_23) = !false;
Goto(bb14)
}
bb14 = {
_24 = _13 as f64;
(*_23) = false & false;
_34 = _24 as f32;
_33 = (*_23) as i64;
_39 = Adt17::Variant1 { fld0: RET,fld1: _13,fld2: _34 };
_13 = Field::<u64>(Variant(_39, 1), 1) * Field::<u64>(Variant(_39, 1), 1);
(*_23) = false & false;
(*_23) = !false;
_39 = Adt17::Variant2 { fld0: _24,fld1: _28,fld2: 209_u8 };
(*_23) = false;
_38.0 = core::ptr::addr_of_mut!(_25);
(*_23) = false | true;
_25 = Field::<u16>(Variant(_39, 2), 1);
_21 = _26;
_17.fld0 = _15 >> _13;
(*_23) = _13 < _13;
(*_23) = true;
(*_23) = false ^ true;
place!(Field::<u8>(Variant(_39, 2), 2)) = 16_u8 + 24_u8;
(*_23) = false;
_9 = _13 ^ _13;
Goto(bb15)
}
bb15 = {
_17.fld0 = _7 as isize;
_39 = Adt17::Variant1 { fld0: RET,fld1: _9,fld2: _34 };
_38.1 = Adt17::Variant1 { fld0: RET,fld1: _9,fld2: _34 };
(*_23) = !false;
_1 = _10;
_29 = core::ptr::addr_of!(_27);
Goto(bb16)
}
bb16 = {
(*_23) = true;
_28 = _25;
(*_23) = true;
_16.1 = &_11;
_17.fld0 = Field::<f32>(Variant(_39, 1), 2) as isize;
(*_23) = !false;
_25 = !_28;
_47.0.0 = &_21;
_42 = core::ptr::addr_of_mut!(_25);
_28 = (*_42) + (*_42);
_34 = -Field::<f32>(Variant(_38.1, 1), 2);
(*_42) = _28;
(*_42) = !_28;
Goto(bb17)
}
bb17 = {
_1 = _21;
(*_42) = !_28;
_42 = core::ptr::addr_of_mut!((*_42));
(*_42) = _28 - _28;
(*_23) = !false;
(*_42) = _28 - _28;
_16.1 = Move(_47.0.0);
(*_23) = !true;
_34 = -Field::<f32>(Variant(_39, 1), 2);
_49 = !3_usize;
_31 = _17.fld0;
_49 = !5_usize;
_35 = _26;
(*_23) = true & true;
_34 = -Field::<f32>(Variant(_39, 1), 2);
_17 = Adt40 { fld0: _15,fld1: Move(_16.0) };
_21 = _10;
_12 = [_21];
_30 = 253020499812664059794536920317951353347_u128 as i8;
_51 = _27 | (*_23);
_7 = _49 as i32;
(*_23) = _51;
Goto(bb18)
}
bb18 = {
_16.0 = core::ptr::addr_of_mut!(_24);
(*_42) = _28;
_9 = !Field::<u64>(Variant(_38.1, 1), 1);
_19 = [(*_42)];
_24 = 102477655696660756276465860441773225899_u128 as f64;
_25 = _28 * _28;
_41 = core::ptr::addr_of!(_17.fld0);
_38 = (Move(_42), _39);
(*_41) = -_31;
RET = _33 as i128;
_46 = core::ptr::addr_of!(_33);
_26 = _21;
(*_23) = !_51;
_16.1 = &_21;
_46 = core::ptr::addr_of!((*_46));
Goto(bb19)
}
bb19 = {
_48 = core::ptr::addr_of_mut!(_29);
_28 = _9 as u16;
place!(Field::<i128>(Variant(_39, 1), 0)) = Field::<i128>(Variant(_38.1, 1), 0) & RET;
_7 = _2 | _2;
_5 = _12;
(*_23) = _51 ^ _51;
_33 = !(-4425571558911733276_i64);
(*_41) = -_31;
(*_48) = core::ptr::addr_of!((*_23));
(*_46) = (-4389332512618059983_i64);
(*_23) = _51;
match (*_46) {
0 => bb6,
1 => bb10,
2 => bb17,
340282366920938463458985274919150151473 => bb21,
_ => bb20
}
}
bb20 = {
_6 = _4;
_7 = _2 + _2;
_8 = _6;
_2 = _7 - _7;
_16.1 = &_1;
_16.1 = &_11;
_7 = _2 << RET;
_7 = _2 * _2;
_11 = _1;
_16.1 = &_1;
_9 = _13 & _13;
_15 = -9223372036854775807_isize;
_11 = _10;
_15 = (-9223372036854775808_isize);
RET = -(-43510769066990309199578786630508645458_i128);
_16.1 = &_10;
_3 = _4;
_4 = _3;
_17.fld0 = 1378907926_u32 as isize;
Goto(bb3)
}
bb21 = {
(*_23) = _51;
_1 = _21;
(*_41) = _31 | _31;
_1 = _11;
_3 = [_11];
(*_41) = _31 & _31;
(*_41) = _31 ^ _31;
_49 = 4958440476947152813_usize ^ 7_usize;
(*_48) = core::ptr::addr_of!(_51);
(*_23) = (*_41) < (*_41);
_50 = &place!(Field::<f32>(Variant(_38.1, 1), 2));
(*_46) = (-2830393481382248623_i64);
(*_46) = (-6684186877916075657_i64);
(*_46) = _21 as i64;
_35 = _20;
(*_23) = (*_29) ^ (*_29);
_59 = &_49;
Goto(bb22)
}
bb22 = {
_40 = (-709_i16) ^ (-22182_i16);
_48 = core::ptr::addr_of_mut!((*_48));
(*_46) = 7472931706102851139_i64 | 4750430452502912490_i64;
(*_23) = (*_29) ^ (*_29);
_44 = &mut _1;
Goto(bb23)
}
bb23 = {
(*_29) = (*_23);
_47.0 = (Move(_16.1),);
Goto(bb24)
}
bb24 = {
_41 = core::ptr::addr_of!((*_41));
(*_29) = !(*_23);
_30 = 82_i8;
(*_23) = !(*_29);
(*_44) = _20;
(*_23) = (*_29) == (*_29);
(*_44) = _10;
_17.fld1 = core::ptr::addr_of_mut!(_24);
_34 = -(*_50);
_39 = _38.1;
_17.fld0 = (*_59) as isize;
match _30 {
0 => bb10,
1 => bb13,
2 => bb25,
3 => bb26,
4 => bb27,
82 => bb29,
_ => bb28
}
}
bb25 = {
(*_29) = (*_23);
_47.0 = (Move(_16.1),);
Goto(bb24)
}
bb26 = {
_15 = 3693521635_u32 as isize;
_15 = -_17.fld0;
RET = -28598952003776695033729951032140733572_i128;
_7 = -_2;
_19 = [1284_u16];
Goto(bb4)
}
bb27 = {
_12 = [_10];
_13 = _9 & _9;
_1 = _11;
_15 = -_17.fld0;
_10 = _11;
_2 = _7 ^ _7;
_6 = _12;
_1 = _10;
_20 = _10;
_11 = _20;
_13 = _9 & _9;
_21 = _11;
_1 = _10;
_17.fld0 = -_15;
_5 = [_1];
_4 = [_21];
_20 = _21;
_2 = _7;
RET = (-69106614649298137649550624415833131818_i128) | (-60105172882094210977618017668122107735_i128);
_16.1 = &_1;
_3 = _5;
_15 = _17.fld0;
Goto(bb5)
}
bb28 = {
RET = (-137416666132536595482447868585372409807_i128) << _2;
Goto(bb12)
}
bb29 = {
(*_46) = !6529148834110874312_i64;
(*_44) = _11;
(*_46) = -879030229815273780_i64;
(*_23) = Field::<i128>(Variant(_39, 1), 0) <= Field::<i128>(Variant(_39, 1), 0);
_55 = Adt17::Variant0 { fld0: 1909265783_u32,fld1: (*_44),fld2: (*_41),fld3: (*_59),fld4: _40,fld5: 34_u8,fld6: 64811535652680592789662079057038402423_u128 };
(*_48) = core::ptr::addr_of!((*_29));
_49 = Field::<usize>(Variant(_55, 0), 3) << _28;
(*_46) = (*_29) as i64;
_63 = !_9;
(*_46) = 6686455167576811116_i64 * 626164677610071060_i64;
(*_23) = (*_29) ^ (*_29);
(*_23) = (*_41) == (*_41);
(*_46) = (*_50) as i64;
match _30 {
82 => bb30,
_ => bb10
}
}
bb30 = {
(*_41) = Field::<isize>(Variant(_55, 0), 2) - _31;
_38.1 = Adt17::Variant2 { fld0: _24,fld1: _28,fld2: 55_u8 };
(*_23) = (*_29) ^ (*_29);
match _30 {
0 => bb26,
1 => bb2,
2 => bb12,
3 => bb4,
4 => bb28,
5 => bb23,
6 => bb24,
82 => bb31,
_ => bb10
}
}
bb31 = {
_67.2 = Move(_17);
(*_48) = core::ptr::addr_of!((*_23));
(*_44) = _21;
(*_48) = core::ptr::addr_of!((*_23));
(*_46) = -(-43686580058734025_i64);
_11 = (*_44);
_52 = !_15;
(*_41) = _67.2.fld0 & _67.2.fld0;
place!(Field::<u8>(Variant(_55, 0), 5)) = !252_u8;
(*_46) = (-8121253945965753227_i64) >> (*_41);
(*_48) = core::ptr::addr_of!((*_23));
place!(Field::<char>(Variant(_55, 0), 1)) = (*_44);
(*_23) = _51 & _51;
_21 = _20;
(*_41) = !_52;
_17.fld1 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_38.1, 2), 0)));
RET = Field::<i128>(Variant(_39, 1), 0) ^ Field::<i128>(Variant(_39, 1), 0);
(*_44) = Field::<char>(Variant(_55, 0), 1);
_8 = [(*_44)];
_65 = (*_46) >= (*_46);
_70 = _34 + Field::<f32>(Variant(_39, 1), 2);
_7 = _9 as i32;
_8 = [Field::<char>(Variant(_55, 0), 1)];
_61 = (*_46) > (*_46);
_42 = Move(_38.0);
Goto(bb32)
}
bb32 = {
Call(_73 = dump_var(Move(_11), Move(_30), Move(_52), Move(_49)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_73 = dump_var(Move(_20), Move(_6), Move(_7), Move(_28)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_73 = dump_var(Move(_40), Move(_61), Move(_4), Move(_31)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_73 = dump_var(Move(_19), Move(_3), Move(_65), Move(_35)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [char; 1],mut _2: char,mut _3: i32,mut _4: [char; 1],mut _5: u64,mut _6: char,mut _7: [char; 1],mut _8: char,mut _9: [char; 1]) -> [char; 1] {
mir! {
type RET = [char; 1];
let _10: bool;
let _11: *const *mut u16;
let _12: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _13: &'static f32;
let _14: *const Adt19;
let _15: *const f64;
let _16: i32;
let _17: &'static char;
let _18: u128;
let _19: *const (&'static char,);
let _20: &'static *const isize;
let _21: f32;
let _22: u64;
let _23: f64;
let _24: isize;
let _25: u128;
let _26: [char; 1];
let _27: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _28: *const isize;
let _29: [i8; 6];
let _30: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _31: *const Adt50;
let _32: isize;
let _33: i16;
let _34: isize;
let _35: Adt40;
let _36: *mut *const bool;
let _37: i128;
let _38: i64;
let _39: *mut [char; 6];
let _40: &'static mut u128;
let _41: i8;
let _42: *const Adt17;
let _43: isize;
let _44: isize;
let _45: isize;
let _46: Adt50;
let _47: [char; 6];
let _48: isize;
let _49: *const Adt19;
let _50: isize;
let _51: isize;
let _52: &'static &'static mut u128;
let _53: bool;
let _54: *mut u32;
let _55: u16;
let _56: f64;
let _57: f64;
let _58: &'static usize;
let _59: Adt19;
let _60: *mut u16;
let _61: isize;
let _62: isize;
let _63: &'static mut char;
let _64: isize;
let _65: (&'static mut char, f32, Adt40, usize);
let _66: f32;
let _67: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _68: isize;
let _69: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _70: i16;
let _71: i32;
let _72: isize;
let _73: bool;
let _74: &'static mut [u64; 6];
let _75: [u64; 6];
let _76: (f64, &'static (u32, u32, Adt17, f32));
let _77: &'static usize;
let _78: isize;
let _79: usize;
let _80: (f64, &'static (u32, u32, Adt17, f32));
let _81: &'static &'static mut [u64; 6];
let _82: *mut [char; 6];
let _83: *const i64;
let _84: &'static mut usize;
let _85: &'static char;
let _86: i16;
let _87: ();
let _88: ();
{
_8 = _2;
_1 = [_8];
Goto(bb1)
}
bb1 = {
_4 = _7;
_9 = [_2];
_4 = _9;
RET = [_8];
_10 = true & false;
_2 = _6;
_2 = _8;
_2 = _8;
RET = _4;
_12.3 = Adt50 { fld0: 119954998944918679582920275750984989418_u128 };
_6 = _2;
_12.3.fld0 = 198754146547935697362957620848745164544_u128;
RET = [_6];
_12.1 = [_5,_5,_5,_5,_5,_5];
_2 = _8;
_9 = [_6];
_7 = _4;
_6 = _8;
_7 = _1;
_10 = _5 < _5;
_5 = 8875571133633563166_u64 & 15854411475375699749_u64;
_7 = [_2];
_8 = _2;
_6 = _8;
_6 = _2;
_5 = 3540279695_u32 as u64;
Goto(bb2)
}
bb2 = {
_12.1 = [_5,_5,_5,_5,_5,_5];
_16 = -_3;
_9 = [_6];
_17 = &_6;
_3 = _16 ^ _16;
_1 = _7;
_6 = _8;
_16 = _3 + _3;
_12.3.fld0 = 274057411989758199374107167053752239706_u128;
_2 = _6;
match _12.3.fld0 {
0 => bb3,
1 => bb4,
274057411989758199374107167053752239706 => bb6,
_ => bb5
}
}
bb3 = {
_4 = _7;
_9 = [_2];
_4 = _9;
RET = [_8];
_10 = true & false;
_2 = _6;
_2 = _8;
_2 = _8;
RET = _4;
_12.3 = Adt50 { fld0: 119954998944918679582920275750984989418_u128 };
_6 = _2;
_12.3.fld0 = 198754146547935697362957620848745164544_u128;
RET = [_6];
_12.1 = [_5,_5,_5,_5,_5,_5];
_2 = _8;
_9 = [_6];
_7 = _4;
_6 = _8;
_7 = _1;
_10 = _5 < _5;
_5 = 8875571133633563166_u64 & 15854411475375699749_u64;
_7 = [_2];
_8 = _2;
_6 = _8;
_6 = _2;
_5 = 3540279695_u32 as u64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_10 = false;
_4 = [_2];
_9 = [_2];
_2 = _6;
_7 = [_2];
_9 = [_6];
_18 = _12.3.fld0 + _12.3.fld0;
_9 = [_8];
_18 = _12.3.fld0;
_4 = [_2];
_9 = [_2];
_10 = _16 < _16;
_5 = 9641277899729225645_u64 + 196840856364979165_u64;
_17 = &_8;
RET = [(*_17)];
_1 = [(*_17)];
_2 = (*_17);
_12.3 = Adt50 { fld0: _18 };
_3 = _16 & _16;
_2 = (*_17);
_17 = &_6;
_2 = (*_17);
_2 = (*_17);
_2 = (*_17);
Call(_2 = fn5((*_17)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _16;
_12.1 = [_5,_5,_5,_5,_5,_5];
_12.1 = [_5,_5,_5,_5,_5,_5];
_24 = _10 as isize;
_8 = (*_17);
_12.2 = core::ptr::addr_of!(_24);
_23 = 1591395210_u32 as f64;
_25 = _12.3.fld0 + _18;
_12.2 = core::ptr::addr_of!(_24);
_5 = !16500139712255393781_u64;
_12.3.fld0 = _25 ^ _25;
_22 = !_5;
_1 = _9;
_18 = _12.3.fld0;
_17 = &_8;
_5 = _22;
_15 = core::ptr::addr_of!(_23);
_25 = _18 | _12.3.fld0;
(*_15) = 3_usize as f64;
_16 = -_3;
Goto(bb8)
}
bb8 = {
_5 = !_22;
_8 = _6;
(*_15) = _25 as f64;
_5 = _22 ^ _22;
Call(_5 = core::intrinsics::bswap(_22), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_30.3.2 = Adt17::Variant0 { fld0: 89489635_u32,fld1: _2,fld2: _24,fld3: 0_usize,fld4: (-14141_i16),fld5: 136_u8,fld6: _25 };
_12.3 = Adt50 { fld0: _18 };
place!(Field::<usize>(Variant(_30.3.2, 0), 3)) = 6617766229953219762378445743681136323_i128 as usize;
(*_15) = (-44_i8) as f64;
_17 = &_2;
_16 = _3 >> Field::<usize>(Variant(_30.3.2, 0), 3);
_6 = (*_17);
_12.3 = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_29 = [(-63_i8),(-63_i8),104_i8,90_i8,(-108_i8),85_i8];
(*_15) = (-19_i8) as f64;
_6 = (*_17);
_1 = [(*_17)];
_9 = _1;
_11 = core::ptr::addr_of!(_27.1.0);
_30.3.3 = 108450568615189895988502597453225751692_i128 as f32;
_25 = _12.3.fld0 << _5;
_29 = [(-94_i8),91_i8,38_i8,55_i8,(-99_i8),86_i8];
_31 = core::ptr::addr_of!(_12.3);
RET = [(*_17)];
(*_15) = Field::<usize>(Variant(_30.3.2, 0), 3) as f64;
(*_31) = Adt50 { fld0: _25 };
_8 = _2;
_21 = -_30.3.3;
_22 = _5 ^ _5;
_26 = [_2];
Goto(bb10)
}
bb10 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb11 = {
_27.3 = [_22,_22,_22,_5,_22,_22];
_29 = [(-48_i8),(-80_i8),69_i8,7_i8,61_i8,(-69_i8)];
Goto(bb12)
}
bb12 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
(*_15) = _21 as f64;
(*_31).fld0 = Field::<u128>(Variant(_30.3.2, 0), 6) | _25;
_27.0 = &mut place!(Field::<usize>(Variant(_30.3.2, 0), 3));
(*_31).fld0 = _25 ^ _25;
_3 = !_16;
_29 = [(-116_i8),77_i8,50_i8,70_i8,(-49_i8),(-111_i8)];
_24 = (-9223372036854775808_isize);
(*_31) = Adt50 { fld0: _25 };
(*_31).fld0 = _18 - _25;
(*_31).fld0 = !_25;
_26 = [(*_17)];
(*_15) = (-4471117964055770401_i64) as f64;
(*_31).fld0 = (*_17) as u128;
_32 = _5 as isize;
(*_31).fld0 = _18 + _18;
_21 = (-9010582659557470859_i64) as f32;
_29 = [37_i8,49_i8,98_i8,(-78_i8),(-81_i8),92_i8];
_27.2.0 = core::ptr::addr_of_mut!((*_15));
_9 = _1;
_33 = (-24262_i16) | 30_i16;
_35.fld0 = !_32;
Goto(bb13)
}
bb13 = {
(*_15) = 23929_u16 as f64;
_4 = _7;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_33 = (-20261_i16);
(*_15) = _16 as f64;
_25 = (*_31).fld0 + (*_31).fld0;
(*_15) = (-95955537368751560264649535904766277973_i128) as f64;
(*_31).fld0 = _25 * _25;
_4 = [_6];
(*_31).fld0 = _18;
(*_31) = Adt50 { fld0: _25 };
_7 = [(*_17)];
(*_15) = _22 as f64;
_22 = (*_17) as u64;
_4 = [_8];
(*_15) = (-46_i8) as f64;
Goto(bb14)
}
bb14 = {
_27.2 = (Move(_35.fld1), Move(_17));
_12.3 = Adt50 { fld0: _25 };
_15 = core::ptr::addr_of!((*_15));
_34 = _35.fld0 << _12.3.fld0;
_8 = _2;
(*_31).fld0 = _5 as u128;
(*_31) = Adt50 { fld0: _25 };
_1 = [_2];
_43 = !_34;
(*_15) = _34 as f64;
_22 = !_5;
(*_15) = 2_usize as f64;
_22 = _5 + _5;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_12.3 = Adt50 { fld0: _25 };
_44 = _34;
Goto(bb15)
}
bb15 = {
_37 = !(-4857603534155266326584817724105800011_i128);
(*_31).fld0 = 120_u8 as u128;
(*_31).fld0 = _25 & _25;
_35.fld0 = _43;
(*_31) = Adt50 { fld0: _18 };
(*_31) = Adt50 { fld0: _25 };
(*_31).fld0 = _25 | _18;
_44 = _34;
_27.2.0 = Move(_35.fld1);
(*_15) = 21_u8 as f64;
_21 = 429415516_u32 as f32;
(*_15) = (-1351016541688487933_i64) as f64;
_17 = &_2;
_7 = _9;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_45 = !_24;
_28 = core::ptr::addr_of!(_35.fld0);
(*_31) = Adt50 { fld0: _25 };
(*_28) = _24 | _43;
(*_31).fld0 = !_25;
(*_28) = (-85_i8) as isize;
(*_31).fld0 = _25 * _25;
_26 = _1;
(*_28) = _37 as isize;
(*_31).fld0 = _33 as u128;
_16 = _3 & _3;
(*_31) = Adt50 { fld0: _25 };
(*_31) = Adt50 { fld0: _25 };
match _24 {
0 => bb6,
1 => bb7,
2 => bb3,
3 => bb12,
4 => bb16,
340282366920938463454151235394913435648 => bb18,
_ => bb17
}
}
bb16 = {
_30.3.2 = Adt17::Variant0 { fld0: 89489635_u32,fld1: _2,fld2: _24,fld3: 0_usize,fld4: (-14141_i16),fld5: 136_u8,fld6: _25 };
_12.3 = Adt50 { fld0: _18 };
place!(Field::<usize>(Variant(_30.3.2, 0), 3)) = 6617766229953219762378445743681136323_i128 as usize;
(*_15) = (-44_i8) as f64;
_17 = &_2;
_16 = _3 >> Field::<usize>(Variant(_30.3.2, 0), 3);
_6 = (*_17);
_12.3 = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_29 = [(-63_i8),(-63_i8),104_i8,90_i8,(-108_i8),85_i8];
(*_15) = (-19_i8) as f64;
_6 = (*_17);
_1 = [(*_17)];
_9 = _1;
_11 = core::ptr::addr_of!(_27.1.0);
_30.3.3 = 108450568615189895988502597453225751692_i128 as f32;
_25 = _12.3.fld0 << _5;
_29 = [(-94_i8),91_i8,38_i8,55_i8,(-99_i8),86_i8];
_31 = core::ptr::addr_of!(_12.3);
RET = [(*_17)];
(*_15) = Field::<usize>(Variant(_30.3.2, 0), 3) as f64;
(*_31) = Adt50 { fld0: _25 };
_8 = _2;
_21 = -_30.3.3;
_22 = _5 ^ _5;
_26 = [_2];
Goto(bb10)
}
bb17 = {
(*_15) = 23929_u16 as f64;
_4 = _7;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_33 = (-20261_i16);
(*_15) = _16 as f64;
_25 = (*_31).fld0 + (*_31).fld0;
(*_15) = (-95955537368751560264649535904766277973_i128) as f64;
(*_31).fld0 = _25 * _25;
_4 = [_6];
(*_31).fld0 = _18;
(*_31) = Adt50 { fld0: _25 };
_7 = [(*_17)];
(*_15) = _22 as f64;
_22 = (*_17) as u64;
_4 = [_8];
(*_15) = (-46_i8) as f64;
Goto(bb14)
}
bb18 = {
(*_28) = _33 as isize;
(*_28) = 2_usize as isize;
(*_15) = 179_u8 as f64;
_35 = Adt40 { fld0: _34,fld1: Move(_27.2.0) };
(*_31).fld0 = !_25;
_27.1.1 = Adt17::Variant0 { fld0: 1776646806_u32,fld1: _6,fld2: (*_28),fld3: 1086298961541181184_usize,fld4: _33,fld5: 155_u8,fld6: (*_31).fld0 };
_43 = !(*_28);
_46.fld0 = _12.3.fld0;
match Field::<i16>(Variant(_27.1.1, 0), 4) {
340282366920938463463374607431768191195 => bb20,
_ => bb19
}
}
bb19 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb20 = {
(*_31) = Adt50 { fld0: _46.fld0 };
Goto(bb21)
}
bb21 = {
(*_15) = _21 as f64;
(*_28) = _34;
_38 = _33 as i64;
_35.fld0 = -_44;
(*_31) = Adt50 { fld0: _25 };
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_27.1.1, 0), 6) };
_40 = &mut (*_31).fld0;
_9 = RET;
_27.2.1 = &(*_17);
_43 = _33 as isize;
(*_15) = (*_40) as f64;
_27.2 = (Move(_35.fld1), Move(_17));
(*_15) = (*_28) as f64;
(*_40) = Field::<u128>(Variant(_27.1.1, 0), 6) + _25;
(*_28) = _34 ^ _34;
(*_28) = _44 + Field::<isize>(Variant(_27.1.1, 0), 2);
(*_40) = _25 * _46.fld0;
(*_40) = _46.fld0 + _25;
_35.fld0 = _34 | _44;
(*_15) = _38 as f64;
Goto(bb22)
}
bb22 = {
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_5 = 6_usize as u64;
match Field::<i16>(Variant(_27.1.1, 0), 4) {
0 => bb1,
1 => bb13,
2 => bb23,
3 => bb24,
4 => bb25,
340282366920938463463374607431768191195 => bb27,
_ => bb26
}
}
bb23 = {
Return()
}
bb24 = {
(*_15) = 23929_u16 as f64;
_4 = _7;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_33 = (-20261_i16);
(*_15) = _16 as f64;
_25 = (*_31).fld0 + (*_31).fld0;
(*_15) = (-95955537368751560264649535904766277973_i128) as f64;
(*_31).fld0 = _25 * _25;
_4 = [_6];
(*_31).fld0 = _18;
(*_31) = Adt50 { fld0: _25 };
_7 = [(*_17)];
(*_15) = _22 as f64;
_22 = (*_17) as u64;
_4 = [_8];
(*_15) = (-46_i8) as f64;
Goto(bb14)
}
bb25 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb26 = {
_27.2 = (Move(_35.fld1), Move(_17));
_12.3 = Adt50 { fld0: _25 };
_15 = core::ptr::addr_of!((*_15));
_34 = _35.fld0 << _12.3.fld0;
_8 = _2;
(*_31).fld0 = _5 as u128;
(*_31) = Adt50 { fld0: _25 };
_1 = [_2];
_43 = !_34;
(*_15) = _34 as f64;
_22 = !_5;
(*_15) = 2_usize as f64;
_22 = _5 + _5;
_35.fld1 = core::ptr::addr_of_mut!((*_15));
_12.3 = Adt50 { fld0: _25 };
_44 = _34;
Goto(bb15)
}
bb27 = {
(*_28) = -Field::<isize>(Variant(_27.1.1, 0), 2);
(*_28) = _44;
(*_15) = _37 as f64;
_26 = [_8];
_5 = !_22;
_27.1.1 = Adt17::Variant1 { fld0: _37,fld1: _22,fld2: _21 };
_33 = 26448_u16 as i16;
_27.3 = [Field::<u64>(Variant(_27.1.1, 1), 1),_22,_22,_22,_22,_5];
(*_28) = _44 | _34;
(*_15) = _38 as f64;
place!(Field::<u64>(Variant(_27.1.1, 1), 1)) = _22 & _5;
place!(Field::<i128>(Variant(_27.1.1, 1), 0)) = _37 + _37;
_51 = !(*_28);
(*_40) = !_25;
RET = _7;
(*_15) = 7101259868167650497_usize as f64;
(*_40) = !_25;
_48 = (*_28) | (*_28);
(*_28) = _48 & _34;
(*_15) = _33 as f64;
(*_40) = _21 as u128;
(*_15) = 600_u16 as f64;
_38 = 5205607601967287576_i64 - 8226233770659950094_i64;
_38 = 741705694050630422_i64 & (-1042131365056359079_i64);
(*_40) = !_46.fld0;
_25 = !(*_40);
_27.3 = [Field::<u64>(Variant(_27.1.1, 1), 1),_22,Field::<u64>(Variant(_27.1.1, 1), 1),_5,_22,Field::<u64>(Variant(_27.1.1, 1), 1)];
(*_40) = _18 | _25;
Goto(bb28)
}
bb28 = {
(*_15) = 4101367846_u32 as f64;
(*_15) = _37 as f64;
place!(Field::<f32>(Variant(_27.1.1, 1), 2)) = -_21;
place!(Field::<u64>(Variant(_27.1.1, 1), 1)) = _5;
_20 = &_28;
(*_11) = core::ptr::addr_of_mut!(_55);
(*_40) = _25 ^ _18;
Goto(bb29)
}
bb29 = {
(*_11) = core::ptr::addr_of_mut!(_55);
(*_15) = _33 as f64;
(*_28) = _48 - _48;
(*_40) = _25 << (*_28);
_26 = _9;
_17 = &_2;
_28 = core::ptr::addr_of!((*_28));
_41 = 106_i8;
(*_11) = core::ptr::addr_of_mut!(_55);
_53 = _10;
(*_11) = core::ptr::addr_of_mut!(_55);
(*_28) = _48 | _48;
Call((*_28) = core::intrinsics::bswap(_48), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_55 = !62572_u16;
(*_15) = _16 as f64;
_42 = core::ptr::addr_of!(_27.1.1);
(*_40) = 2_usize as u128;
(*_40) = _25 | _46.fld0;
(*_40) = _25 - _25;
(*_40) = (*_17) as u128;
(*_11) = core::ptr::addr_of_mut!(_55);
match _24 {
0 => bb1,
1 => bb14,
2 => bb6,
340282366920938463454151235394913435648 => bb32,
_ => bb31
}
}
bb31 = {
_5 = !_22;
_8 = _6;
(*_15) = _25 as f64;
_5 = _22 ^ _22;
Call(_5 = core::intrinsics::bswap(_22), ReturnTo(bb9), UnwindUnreachable())
}
bb32 = {
_31 = core::ptr::addr_of!(_46);
_27.3 = [Field::<u64>(Variant((*_42), 1), 1),Field::<u64>(Variant((*_42), 1), 1),Field::<u64>(Variant((*_42), 1), 1),Field::<u64>(Variant((*_42), 1), 1),Field::<u64>(Variant((*_42), 1), 1),Field::<u64>(Variant((*_42), 1), 1)];
(*_15) = _33 as f64;
place!(Field::<u64>(Variant((*_42), 1), 1)) = _5 * _22;
(*_28) = (*_17) as isize;
(*_42) = Adt17::Variant0 { fld0: 4240206568_u32,fld1: (*_17),fld2: _34,fld3: 7_usize,fld4: _33,fld5: 34_u8,fld6: (*_31).fld0 };
place!(Field::<u8>(Variant((*_42), 0), 5)) = 23_u8;
_16 = _3 + _3;
place!(Field::<char>(Variant((*_42), 0), 1)) = (*_17);
place!(Field::<i16>(Variant(_27.1.1, 0), 4)) = !_33;
(*_31) = Adt50 { fld0: Field::<u128>(Variant((*_42), 0), 6) };
place!(Field::<isize>(Variant((*_42), 0), 2)) = (*_28);
(*_15) = Field::<u8>(Variant((*_42), 0), 5) as f64;
place!(Field::<u32>(Variant((*_42), 0), 0)) = 3090456475_u32 - 2955996903_u32;
(*_31).fld0 = _55 as u128;
(*_11) = core::ptr::addr_of_mut!(_55);
place!(Field::<usize>(Variant((*_42), 0), 3)) = 16050586816509733729_usize * 2071048880983443411_usize;
place!(Field::<char>(Variant((*_42), 0), 1)) = (*_17);
(*_11) = core::ptr::addr_of_mut!(_55);
place!(Field::<char>(Variant((*_42), 0), 1)) = _6;
place!(Field::<isize>(Variant((*_42), 0), 2)) = _21 as isize;
(*_42) = Adt17::Variant1 { fld0: _37,fld1: _5,fld2: _21 };
match _24 {
0 => bb6,
340282366920938463454151235394913435648 => bb33,
_ => bb18
}
}
bb33 = {
(*_31) = Adt50 { fld0: _25 };
(*_11) = core::ptr::addr_of_mut!(_55);
(*_40) = !(*_31).fld0;
_65.0 = &mut (*_17);
(*_31) = Adt50 { fld0: (*_40) };
_69.1 = Adt17::Variant0 { fld0: 2227105526_u32,fld1: _8,fld2: _34,fld3: 1652240092839798086_usize,fld4: _33,fld5: 252_u8,fld6: (*_40) };
place!(Field::<u64>(Variant((*_42), 1), 1)) = _41 as u64;
_70 = Field::<u64>(Variant((*_42), 1), 1) as i16;
(*_11) = core::ptr::addr_of_mut!(_55);
_8 = Field::<char>(Variant(_69.1, 0), 1);
place!(Field::<u8>(Variant(_69.1, 0), 5)) = 112_u8 * 58_u8;
_28 = core::ptr::addr_of!(_35.fld0);
_32 = _48 & _48;
_49 = core::ptr::addr_of!(_59);
place!(Field::<u64>(Variant((*_42), 1), 1)) = !_5;
place!(Field::<u64>(Variant((*_42), 1), 1)) = _22;
_50 = !_48;
(*_31).fld0 = Field::<i128>(Variant((*_42), 1), 0) as u128;
place!(Field::<u64>(Variant((*_42), 1), 1)) = Field::<char>(Variant(_69.1, 0), 1) as u64;
(*_31) = Adt50 { fld0: (*_40) };
(*_31) = Adt50 { fld0: (*_40) };
(*_49) = Adt19::Variant2 { fld0: _16,fld1: 3088727291543263766_usize,fld2: Field::<u8>(Variant(_69.1, 0), 5),fld3: _41,fld4: Field::<i128>(Variant((*_42), 1), 0) };
(*_28) = Field::<i8>(Variant((*_49), 2), 3) as isize;
match Field::<i8>(Variant((*_49), 2), 3) {
0 => bb34,
1 => bb35,
2 => bb36,
3 => bb37,
4 => bb38,
5 => bb39,
106 => bb41,
_ => bb40
}
}
bb34 = {
Return()
}
bb35 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb36 = {
_55 = !62572_u16;
(*_15) = _16 as f64;
_42 = core::ptr::addr_of!(_27.1.1);
(*_40) = 2_usize as u128;
(*_40) = _25 | _46.fld0;
(*_40) = _25 - _25;
(*_40) = (*_17) as u128;
(*_11) = core::ptr::addr_of_mut!(_55);
match _24 {
0 => bb1,
1 => bb14,
2 => bb6,
340282366920938463454151235394913435648 => bb32,
_ => bb31
}
}
bb37 = {
Return()
}
bb38 = {
(*_31) = Adt50 { fld0: _46.fld0 };
Goto(bb21)
}
bb39 = {
_27.3 = [_22,_22,_22,_5,_22,_22];
_29 = [(-48_i8),(-80_i8),69_i8,7_i8,61_i8,(-69_i8)];
Goto(bb12)
}
bb40 = {
_30.3.2 = Adt17::Variant0 { fld0: 89489635_u32,fld1: _2,fld2: _24,fld3: 0_usize,fld4: (-14141_i16),fld5: 136_u8,fld6: _25 };
_12.3 = Adt50 { fld0: _18 };
place!(Field::<usize>(Variant(_30.3.2, 0), 3)) = 6617766229953219762378445743681136323_i128 as usize;
(*_15) = (-44_i8) as f64;
_17 = &_2;
_16 = _3 >> Field::<usize>(Variant(_30.3.2, 0), 3);
_6 = (*_17);
_12.3 = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_29 = [(-63_i8),(-63_i8),104_i8,90_i8,(-108_i8),85_i8];
(*_15) = (-19_i8) as f64;
_6 = (*_17);
_1 = [(*_17)];
_9 = _1;
_11 = core::ptr::addr_of!(_27.1.0);
_30.3.3 = 108450568615189895988502597453225751692_i128 as f32;
_25 = _12.3.fld0 << _5;
_29 = [(-94_i8),91_i8,38_i8,55_i8,(-99_i8),86_i8];
_31 = core::ptr::addr_of!(_12.3);
RET = [(*_17)];
(*_15) = Field::<usize>(Variant(_30.3.2, 0), 3) as f64;
(*_31) = Adt50 { fld0: _25 };
_8 = _2;
_21 = -_30.3.3;
_22 = _5 ^ _5;
_26 = [_2];
Goto(bb10)
}
bb41 = {
_33 = Field::<i16>(Variant(_69.1, 0), 4);
place!(Field::<i32>(Variant((*_49), 2), 0)) = !_3;
match Field::<i8>(Variant((*_49), 2), 3) {
0 => bb15,
1 => bb42,
2 => bb43,
3 => bb44,
106 => bb46,
_ => bb45
}
}
bb42 = {
(*_15) = 4101367846_u32 as f64;
(*_15) = _37 as f64;
place!(Field::<f32>(Variant(_27.1.1, 1), 2)) = -_21;
place!(Field::<u64>(Variant(_27.1.1, 1), 1)) = _5;
_20 = &_28;
(*_11) = core::ptr::addr_of_mut!(_55);
(*_40) = _25 ^ _18;
Goto(bb29)
}
bb43 = {
(*_15) = _21 as f64;
(*_28) = _34;
_38 = _33 as i64;
_35.fld0 = -_44;
(*_31) = Adt50 { fld0: _25 };
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_27.1.1, 0), 6) };
_40 = &mut (*_31).fld0;
_9 = RET;
_27.2.1 = &(*_17);
_43 = _33 as isize;
(*_15) = (*_40) as f64;
_27.2 = (Move(_35.fld1), Move(_17));
(*_15) = (*_28) as f64;
(*_40) = Field::<u128>(Variant(_27.1.1, 0), 6) + _25;
(*_28) = _34 ^ _34;
(*_28) = _44 + Field::<isize>(Variant(_27.1.1, 0), 2);
(*_40) = _25 * _46.fld0;
(*_40) = _46.fld0 + _25;
_35.fld0 = _34 | _44;
(*_15) = _38 as f64;
Goto(bb22)
}
bb44 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb45 = {
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_30.3.2, 0), 6) };
_12.3 = Adt50 { fld0: _25 };
(*_31).fld0 = (-84_i8) as u128;
_27.2.1 = &(*_17);
_11 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_30.3.2, 0), 2)) = _24 | _24;
_20 = &_12.2;
Goto(bb11)
}
bb46 = {
(*_28) = -_51;
_28 = core::ptr::addr_of!(_51);
(*_11) = core::ptr::addr_of_mut!(_55);
place!(Field::<i8>(Variant((*_49), 2), 3)) = _41;
(*_40) = _8 as u128;
_48 = _50 | (*_28);
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_69.1, 0), 6) };
place!(Field::<i32>(Variant((*_49), 2), 0)) = _16 + _16;
_3 = Field::<i32>(Variant((*_49), 2), 0) << (*_31).fld0;
place!(Field::<i128>(Variant((*_42), 1), 0)) = Field::<i128>(Variant((*_49), 2), 4) * Field::<i128>(Variant((*_49), 2), 4);
_24 = _50 >> Field::<i32>(Variant((*_49), 2), 0);
place!(Field::<usize>(Variant(_69.1, 0), 3)) = 13799737196660980539_usize + 5_usize;
place!(Field::<u8>(Variant((*_49), 2), 2)) = !Field::<u8>(Variant(_69.1, 0), 5);
_69.0.0 = Move(_17);
place!(Field::<u64>(Variant((*_42), 1), 1)) = _5;
place!(Field::<i128>(Variant((*_49), 2), 4)) = Field::<i128>(Variant((*_42), 1), 0) ^ Field::<i128>(Variant((*_42), 1), 0);
(*_31) = Adt50 { fld0: _25 };
place!(Field::<i128>(Variant((*_49), 2), 4)) = Field::<i128>(Variant((*_42), 1), 0);
_39 = core::ptr::addr_of_mut!(_47);
place!(Field::<u8>(Variant((*_49), 2), 2)) = !Field::<u8>(Variant(_69.1, 0), 5);
(*_49) = Adt19::Variant0 { fld0: _53,fld1: Field::<i128>(Variant((*_42), 1), 0),fld2: Field::<u64>(Variant((*_42), 1), 1) };
(*_42) = Adt17::Variant0 { fld0: 1382998766_u32,fld1: _8,fld2: _32,fld3: Field::<usize>(Variant(_69.1, 0), 3),fld4: Field::<i16>(Variant(_69.1, 0), 4),fld5: Field::<u8>(Variant(_69.1, 0), 5),fld6: (*_31).fld0 };
place!(Field::<char>(Variant((*_42), 0), 1)) = _6;
place!(Field::<i16>(Variant((*_42), 0), 4)) = Field::<i16>(Variant(_69.1, 0), 4) ^ _33;
place!(Field::<i128>(Variant(_59, 0), 1)) = _37;
_10 = Field::<bool>(Variant((*_49), 0), 0);
match _41 {
106 => bb48,
_ => bb47
}
}
bb47 = {
(*_15) = _21 as f64;
(*_28) = _34;
_38 = _33 as i64;
_35.fld0 = -_44;
(*_31) = Adt50 { fld0: _25 };
(*_31) = Adt50 { fld0: Field::<u128>(Variant(_27.1.1, 0), 6) };
_40 = &mut (*_31).fld0;
_9 = RET;
_27.2.1 = &(*_17);
_43 = _33 as isize;
(*_15) = (*_40) as f64;
_27.2 = (Move(_35.fld1), Move(_17));
(*_15) = (*_28) as f64;
(*_40) = Field::<u128>(Variant(_27.1.1, 0), 6) + _25;
(*_28) = _34 ^ _34;
(*_28) = _44 + Field::<isize>(Variant(_27.1.1, 0), 2);
(*_40) = _25 * _46.fld0;
(*_40) = _46.fld0 + _25;
_35.fld0 = _34 | _44;
(*_15) = _38 as f64;
Goto(bb22)
}
bb48 = {
(*_39) = [Field::<char>(Variant((*_42), 0), 1),Field::<char>(Variant((*_42), 0), 1),Field::<char>(Variant((*_42), 0), 1),Field::<char>(Variant((*_42), 0), 1),Field::<char>(Variant((*_42), 0), 1),Field::<char>(Variant((*_42), 0), 1)];
(*_40) = (*_31).fld0;
place!(Field::<u128>(Variant((*_42), 0), 6)) = !(*_40);
place!(Field::<usize>(Variant((*_42), 0), 3)) = Field::<usize>(Variant(_69.1, 0), 3) & Field::<usize>(Variant(_69.1, 0), 3);
place!(Field::<i128>(Variant((*_49), 0), 1)) = _37;
place!(Field::<u64>(Variant((*_49), 0), 2)) = _22 & _22;
_11 = core::ptr::addr_of!((*_11));
place!(Field::<u64>(Variant((*_49), 0), 2)) = _5 ^ _5;
place!(Field::<u64>(Variant((*_49), 0), 2)) = _5;
(*_42) = Adt17::Variant2 { fld0: (*_15),fld1: _55,fld2: Field::<u8>(Variant(_69.1, 0), 5) };
place!(Field::<u16>(Variant((*_42), 2), 1)) = !_55;
_73 = Field::<bool>(Variant((*_49), 0), 0) & Field::<bool>(Variant((*_49), 0), 0);
(*_40) = (*_31).fld0;
place!(Field::<u8>(Variant((*_42), 2), 2)) = Field::<u8>(Variant(_69.1, 0), 5);
_58 = &place!(Field::<usize>(Variant(_69.1, 0), 3));
_69.1 = Adt17::Variant2 { fld0: (*_15),fld1: _55,fld2: Field::<u8>(Variant((*_42), 2), 2) };
(*_11) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant((*_42), 2), 1)));
(*_15) = Field::<i128>(Variant((*_49), 0), 1) as f64;
_65.2 = Adt40 { fld0: _51,fld1: Move(_27.2.0) };
(*_15) = Field::<f64>(Variant((*_42), 2), 0);
Goto(bb49)
}
bb49 = {
place!(Field::<f64>(Variant(_27.1.1, 2), 0)) = (*_15) - (*_15);
_69.0.0 = &_6;
_38 = !(-7207742032171958144_i64);
place!(Field::<f64>(Variant((*_42), 2), 0)) = Field::<f64>(Variant(_69.1, 2), 0) + (*_15);
_27.2 = (Move(_35.fld1), Move(_69.0.0));
_80.0 = _23 * Field::<f64>(Variant((*_42), 2), 0);
place!(Field::<i128>(Variant((*_49), 0), 1)) = _21 as i128;
_71 = _3 & _16;
(*_42) = _69.1;
_26 = [_8];
place!(Field::<u8>(Variant((*_42), 2), 2)) = !Field::<u8>(Variant(_69.1, 2), 2);
(*_40) = (*_31).fld0 - (*_31).fld0;
(*_40) = _46.fld0 << Field::<u64>(Variant((*_49), 0), 2);
_65.3 = _10 as usize;
(*_31).fld0 = (*_40) + (*_40);
_15 = core::ptr::addr_of!(place!(Field::<f64>(Variant((*_42), 2), 0)));
_27.0 = &mut _65.3;
_26 = [_8];
_69.2.fld0 = _6;
_50 = Field::<u8>(Variant((*_42), 2), 2) as isize;
_72 = -_32;
place!(Field::<u64>(Variant((*_49), 0), 2)) = _22 & _22;
(*_28) = !_34;
_76.0 = Field::<f64>(Variant((*_42), 2), 0);
Goto(bb50)
}
bb50 = {
Call(_87 = dump_var(Move(_18), Move(_26), Move(_9), Move(_16)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_87 = dump_var(Move(_24), Move(_70), Move(_25), Move(_7)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_87 = dump_var(Move(_4), Move(_51), Move(_37), Move(_43)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_87 = dump_var(Move(_44), Move(_53), Move(_1), Move(_5)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_87 = dump_var(Move(_47), Move(_50), _88, _88), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: char) -> char {
mir! {
type RET = char;
let _2: *mut *const bool;
let _3: Adt19;
let _4: i32;
let _5: &'static Adt19;
let _6: i8;
let _7: *const isize;
let _8: (Adt50,);
let _9: (Adt50,);
let _10: Adt85;
let _11: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _12: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _13: i16;
let _14: &'static *const isize;
let _15: bool;
let _16: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _17: &'static mut char;
let _18: &'static Adt19;
let _19: f32;
let _20: &'static mut [u64; 6];
let _21: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _22: isize;
let _23: *const f64;
let _24: char;
let _25: &'static *const isize;
let _26: [u16; 1];
let _27: usize;
let _28: (*mut u16, Adt17);
let _29: isize;
let _30: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _31: isize;
let _32: f32;
let _33: *const f64;
let _34: i128;
let _35: char;
let _36: &'static &'static mut [u64; 6];
let _37: i64;
let _38: usize;
let _39: *mut *const bool;
let _40: f64;
let _41: i8;
let _42: (*mut f64, &'static char);
let _43: i64;
let _44: f32;
let _45: f32;
let _46: bool;
let _47: f64;
let _48: bool;
let _49: isize;
let _50: [char; 7];
let _51: usize;
let _52: [char; 7];
let _53: isize;
let _54: [i8; 6];
let _55: &'static (u32, u32, Adt17, f32);
let _56: *mut *mut f64;
let _57: ();
let _58: ();
{
RET = _1;
_1 = RET;
RET = _1;
_1 = RET;
RET = _1;
_1 = RET;
_1 = RET;
_1 = RET;
RET = _1;
_1 = RET;
RET = _1;
RET = _1;
RET = _1;
RET = _1;
Goto(bb1)
}
bb1 = {
RET = _1;
_1 = RET;
_3 = Adt19::Variant2 { fld0: 1379495547_i32,fld1: 0_usize,fld2: 165_u8,fld3: (-97_i8),fld4: (-88148007606830711140540424000315667636_i128) };
place!(Field::<usize>(Variant(_3, 2), 1)) = 5_usize | 14283562979267674029_usize;
place!(Field::<i128>(Variant(_3, 2), 4)) = (-98849197913561413706591282400047408309_i128) >> Field::<usize>(Variant(_3, 2), 1);
_3 = Adt19::Variant2 { fld0: 2114329344_i32,fld1: 4_usize,fld2: 208_u8,fld3: (-11_i8),fld4: (-164311954500768475021094199637186826718_i128) };
_1 = RET;
place!(Field::<i128>(Variant(_3, 2), 4)) = 56877_u16 as i128;
place!(Field::<usize>(Variant(_3, 2), 1)) = 7216_i16 as usize;
_3 = Adt19::Variant0 { fld0: false,fld1: 68342858647638845288039057292847799565_i128,fld2: 16900267164053525226_u64 };
_1 = RET;
_4 = 1482395681_i32 << 9129823788777240702486483837818628172_i128;
_3 = Adt19::Variant2 { fld0: _4,fld1: 2_usize,fld2: 174_u8,fld3: 92_i8,fld4: 36319391001599312183978420982351286383_i128 };
_3 = Adt19::Variant0 { fld0: false,fld1: 145137566176686201526460417618956879424_i128,fld2: 17751391792337963385_u64 };
place!(Field::<bool>(Variant(_3, 0), 0)) = !true;
_4 = 94208425526341676509063785586727616997_i128 as i32;
_4 = 624479746_i32 * (-1620760352_i32);
place!(Field::<u64>(Variant(_3, 0), 2)) = !3053817525076252542_u64;
Goto(bb2)
}
bb2 = {
RET = _1;
_3 = Adt19::Variant0 { fld0: true,fld1: 88100105865226147557638984499129421426_i128,fld2: 18116329335444976812_u64 };
_1 = RET;
_1 = RET;
_6 = -97_i8;
RET = _1;
_6 = (-121_i8) + 16_i8;
_4 = 127049026_i32 ^ 1729764593_i32;
_1 = RET;
place!(Field::<i128>(Variant(_3, 0), 1)) = 112329821772061012938086585410046993254_i128 << _4;
place!(Field::<u64>(Variant(_3, 0), 2)) = 13284466488368831445_u64 * 14480582781744095255_u64;
place!(Field::<bool>(Variant(_3, 0), 0)) = Field::<u64>(Variant(_3, 0), 2) == Field::<u64>(Variant(_3, 0), 2);
place!(Field::<bool>(Variant(_3, 0), 0)) = !true;
_4 = 746330323_i32 ^ (-2028305871_i32);
place!(Field::<i128>(Variant(_3, 0), 1)) = Field::<bool>(Variant(_3, 0), 0) as i128;
_5 = &_3;
_8.0 = Adt50 { fld0: 157297585309475046407115457645050937614_u128 };
match _8.0.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
157297585309475046407115457645050937614 => bb9,
_ => bb8
}
}
bb3 = {
RET = _1;
_1 = RET;
_3 = Adt19::Variant2 { fld0: 1379495547_i32,fld1: 0_usize,fld2: 165_u8,fld3: (-97_i8),fld4: (-88148007606830711140540424000315667636_i128) };
place!(Field::<usize>(Variant(_3, 2), 1)) = 5_usize | 14283562979267674029_usize;
place!(Field::<i128>(Variant(_3, 2), 4)) = (-98849197913561413706591282400047408309_i128) >> Field::<usize>(Variant(_3, 2), 1);
_3 = Adt19::Variant2 { fld0: 2114329344_i32,fld1: 4_usize,fld2: 208_u8,fld3: (-11_i8),fld4: (-164311954500768475021094199637186826718_i128) };
_1 = RET;
place!(Field::<i128>(Variant(_3, 2), 4)) = 56877_u16 as i128;
place!(Field::<usize>(Variant(_3, 2), 1)) = 7216_i16 as usize;
_3 = Adt19::Variant0 { fld0: false,fld1: 68342858647638845288039057292847799565_i128,fld2: 16900267164053525226_u64 };
_1 = RET;
_4 = 1482395681_i32 << 9129823788777240702486483837818628172_i128;
_3 = Adt19::Variant2 { fld0: _4,fld1: 2_usize,fld2: 174_u8,fld3: 92_i8,fld4: 36319391001599312183978420982351286383_i128 };
_3 = Adt19::Variant0 { fld0: false,fld1: 145137566176686201526460417618956879424_i128,fld2: 17751391792337963385_u64 };
place!(Field::<bool>(Variant(_3, 0), 0)) = !true;
_4 = 94208425526341676509063785586727616997_i128 as i32;
_4 = 624479746_i32 * (-1620760352_i32);
place!(Field::<u64>(Variant(_3, 0), 2)) = !3053817525076252542_u64;
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
_6 = !26_i8;
_4 = (-1977208523_i32);
_6 = _8.0.fld0 as i8;
_4 = (-917722456_i32);
_1 = RET;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-155388707460859618820126471210140707219_i128) - (-107757502008485065991428165376559946323_i128);
_8.0 = Adt50 { fld0: 262644473571107444148430529037996287291_u128 };
RET = _1;
_1 = RET;
_8.0.fld0 = 547039474_u32 as u128;
_9.0 = Move(_8.0);
_8.0.fld0 = _9.0.fld0;
_9 = (Move(_8.0),);
_9.0.fld0 = !75531403001743826383562529803129082284_u128;
_8 = (Move(_9.0),);
place!(Field::<bool>(Variant(_3, 0), 0)) = false ^ true;
_9.0 = Adt50 { fld0: _8.0.fld0 };
place!(Field::<u64>(Variant(_3, 0), 2)) = !612792326982716199_u64;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-166848123227820650993176368348251613329_i128) & 87609242963281824578817854705294276864_i128;
Call(_7 = fn6(Move(_5), Move(_8.0), _6, _1, Field::<i128>(Variant(_3, 0), 1), _1, RET, RET, _6, _4, _9.0.fld0, Field::<i128>(Variant(_3, 0), 1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9.0.fld0 = _6 as u128;
_9.0.fld0 = 71972012219398496891279643062736201879_u128 - 185207671389660961578236477429743385926_u128;
_8 = (Move(_9.0),);
_9.0.fld0 = _8.0.fld0;
_8.0 = Move(_9.0);
_4 = 32580_i16 as i32;
RET = _1;
_12.3.fld0 = !_8.0.fld0;
_13 = (-15083_i16) >> Field::<i128>(Variant(_3, 0), 1);
_14 = &_7;
_8 = (Move(_12.3),);
Goto(bb11)
}
bb11 = {
_13 = _1 as i16;
place!(Field::<bool>(Variant(_3, 0), 0)) = true & true;
_9.0 = Adt50 { fld0: _8.0.fld0 };
_12.3 = Move(_9.0);
_8 = (Move(_12.3),);
_6 = 69_u8 as i8;
_8.0.fld0 = 330489870346532019384832476170802622521_u128 - 150655958231625653057661742763783785149_u128;
_12.3.fld0 = _8.0.fld0 << Field::<u64>(Variant(_3, 0), 2);
_12.3.fld0 = _8.0.fld0 >> Field::<i128>(Variant(_3, 0), 1);
RET = _1;
_9 = (Move(_12.3),);
_12.2 = Move(_7);
_14 = &_12.2;
_3 = Adt19::Variant2 { fld0: _4,fld1: 4_usize,fld2: 9_u8,fld3: _6,fld4: 17290474754661342176501328903121141292_i128 };
_3 = Adt19::Variant0 { fld0: false,fld1: (-92046198348881471040941736325369944111_i128),fld2: 13442060084740063002_u64 };
_1 = RET;
_6 = 88_i8;
Goto(bb12)
}
bb12 = {
_12.1 = [11175672089579437695_u64,9633959153620731031_u64,7879700463196085334_u64,3019571979543582957_u64,15679772624361011751_u64,11097110697004078439_u64];
_1 = RET;
RET = _1;
_13 = (-30570_i16);
_4 = -151800431_i32;
_16.3.3 = 0_usize as f32;
place!(Field::<i128>(Variant(_3, 0), 1)) = -110874107459835463026357974848149147893_i128;
_9.0.fld0 = !_8.0.fld0;
_15 = true;
_8 = Move(_9);
_1 = RET;
_9.0.fld0 = !_8.0.fld0;
match _6 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb13,
88 => bb15,
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
_12.3 = Adt50 { fld0: _9.0.fld0 };
_16.2 = 25897_u16 as i32;
_6 = 35_i8 | (-20_i8);
_8.0.fld0 = _12.3.fld0;
_16.3.1 = 4231729764_u32 << _12.3.fld0;
RET = _1;
_16.3.2 = Adt17::Variant0 { fld0: _16.3.1,fld1: RET,fld2: 9223372036854775807_isize,fld3: 0_usize,fld4: _13,fld5: 126_u8,fld6: _8.0.fld0 };
place!(Field::<u32>(Variant(_16.3.2, 0), 0)) = _6 as u32;
_6 = !66_i8;
_12.2 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_16.3.2, 0), 2)));
_16.3.0 = 9223372036854775807_isize as u32;
_1 = Field::<char>(Variant(_16.3.2, 0), 1);
place!(Field::<bool>(Variant(_3, 0), 0)) = _15;
_17 = &mut RET;
_16.0 = (*_17);
_16.3.1 = Field::<u32>(Variant(_16.3.2, 0), 0) + Field::<u32>(Variant(_16.3.2, 0), 0);
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _16.0;
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _1;
_9.0 = Adt50 { fld0: _8.0.fld0 };
_16.1 = Adt19::Variant2 { fld0: _4,fld1: 1_usize,fld2: 205_u8,fld3: _6,fld4: Field::<i128>(Variant(_3, 0), 1) };
_1 = (*_17);
place!(Field::<u32>(Variant(_16.3.2, 0), 0)) = _16.3.0 >> _4;
_12.2 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_16.3.2, 0), 2)));
match Field::<i16>(Variant(_16.3.2, 0), 4) {
340282366920938463463374607431768180886 => bb16,
_ => bb4
}
}
bb16 = {
_16.1 = Adt19::Variant0 { fld0: _15,fld1: Field::<i128>(Variant(_3, 0), 1),fld2: 8158640891720098179_u64 };
_13 = Field::<i16>(Variant(_16.3.2, 0), 4) | Field::<i16>(Variant(_16.3.2, 0), 4);
_16.3.0 = Field::<u32>(Variant(_16.3.2, 0), 0) + _16.3.1;
place!(Field::<char>(Variant(_16.3.2, 0), 1)) = (*_17);
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _1;
Goto(bb17)
}
bb17 = {
place!(Field::<u64>(Variant(_3, 0), 2)) = 3489640212161513331_u64 | 12491560818411745742_u64;
place!(Field::<usize>(Variant(_16.3.2, 0), 3)) = 3_usize | 6_usize;
_17 = &mut _16.0;
(*_17) = _1;
place!(Field::<bool>(Variant(_3, 0), 0)) = (*_17) != (*_17);
place!(Field::<bool>(Variant(_3, 0), 0)) = _13 < _13;
_18 = &_3;
_4 = 1656181923_i32;
(*_17) = _1;
(*_17) = _1;
(*_17) = _1;
_4 = (-1384970003_i32);
_6 = !98_i8;
_20 = &mut _12.1;
(*_17) = _1;
(*_20) = [Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2)];
_5 = &(*_18);
(*_17) = _1;
(*_17) = _1;
_21.1 = [Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_18), 0), 2)];
(*_20) = [Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2)];
(*_20) = [Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_5), 0), 2)];
(*_17) = _1;
_17 = &mut _1;
Goto(bb18)
}
bb18 = {
_22 = -(-9223372036854775808_isize);
(*_17) = '\u{b9ddd}';
place!(Field::<u64>(Variant(_3, 0), 2)) = (*_17) as u64;
(*_17) = '\u{d8ebc}';
_21.3 = Adt50 { fld0: _9.0.fld0 };
(*_20) = _21.1;
(*_17) = '\u{991ec}';
(*_17) = '\u{bbb2a}';
_19 = _4 as f32;
_22 = _13 as isize;
(*_17) = '\u{47a1e}';
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_24 = (*_17);
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_20) = _21.1;
(*_17) = _24;
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_13 = (-29482_i16) | 9134_i16;
Goto(bb19)
}
bb19 = {
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_21.2 = core::ptr::addr_of!(_22);
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = _21.1;
_9.0.fld0 = 28948_u16 as u128;
_27 = 23226_u16 as usize;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
place!(Field::<i128>(Variant(_3, 0), 1)) = _27 as i128;
(*_20) = _21.1;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_9 = (Move(_8.0),);
_8.0.fld0 = !_21.3.fld0;
_14 = &_21.2;
_15 = !Field::<bool>(Variant((*_18), 0), 0);
_9.0 = Adt50 { fld0: _8.0.fld0 };
(*_17) = _24;
(*_17) = _24;
(*_17) = _24;
_25 = &_21.2;
Goto(bb20)
}
bb20 = {
_19 = Field::<u64>(Variant(_3, 0), 2) as f32;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = _21.1;
_24 = (*_17);
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_32 = -_19;
_5 = Move(_18);
(*_17) = _24;
(*_20) = _21.1;
_29 = _22 ^ _22;
place!(Field::<u64>(Variant(_3, 0), 2)) = !7299504231752842334_u64;
_17 = &mut _24;
(*_17) = '\u{34238}';
(*_20) = _21.1;
_8 = (Move(_21.3),);
(*_20) = _21.1;
_29 = _15 as isize;
_21.3.fld0 = (*_17) as u128;
_8.0 = Move(_9.0);
match _4 {
0 => bb15,
1 => bb13,
2 => bb12,
3 => bb11,
4 => bb14,
5 => bb10,
340282366920938463463374607430383241453 => bb21,
_ => bb7
}
}
bb21 = {
_8.0.fld0 = _21.3.fld0;
(*_20) = _21.1;
(*_17) = '\u{dc4ab}';
_31 = -_29;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_21.3 = Adt50 { fld0: _8.0.fld0 };
(*_20) = _21.1;
_30.3 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = '\u{10f11f}';
match _4 {
0 => bb14,
1 => bb22,
2 => bb23,
3 => bb24,
4 => bb25,
5 => bb26,
6 => bb27,
340282366920938463463374607430383241453 => bb29,
_ => bb28
}
}
bb22 = {
_19 = Field::<u64>(Variant(_3, 0), 2) as f32;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = _21.1;
_24 = (*_17);
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_32 = -_19;
_5 = Move(_18);
(*_17) = _24;
(*_20) = _21.1;
_29 = _22 ^ _22;
place!(Field::<u64>(Variant(_3, 0), 2)) = !7299504231752842334_u64;
_17 = &mut _24;
(*_17) = '\u{34238}';
(*_20) = _21.1;
_8 = (Move(_21.3),);
(*_20) = _21.1;
_29 = _15 as isize;
_21.3.fld0 = (*_17) as u128;
_8.0 = Move(_9.0);
match _4 {
0 => bb15,
1 => bb13,
2 => bb12,
3 => bb11,
4 => bb14,
5 => bb10,
340282366920938463463374607430383241453 => bb21,
_ => bb7
}
}
bb23 = {
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_21.2 = core::ptr::addr_of!(_22);
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = _21.1;
_9.0.fld0 = 28948_u16 as u128;
_27 = 23226_u16 as usize;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
place!(Field::<i128>(Variant(_3, 0), 1)) = _27 as i128;
(*_20) = _21.1;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_9 = (Move(_8.0),);
_8.0.fld0 = !_21.3.fld0;
_14 = &_21.2;
_15 = !Field::<bool>(Variant((*_18), 0), 0);
_9.0 = Adt50 { fld0: _8.0.fld0 };
(*_17) = _24;
(*_17) = _24;
(*_17) = _24;
_25 = &_21.2;
Goto(bb20)
}
bb24 = {
_22 = -(-9223372036854775808_isize);
(*_17) = '\u{b9ddd}';
place!(Field::<u64>(Variant(_3, 0), 2)) = (*_17) as u64;
(*_17) = '\u{d8ebc}';
_21.3 = Adt50 { fld0: _9.0.fld0 };
(*_20) = _21.1;
(*_17) = '\u{991ec}';
(*_17) = '\u{bbb2a}';
_19 = _4 as f32;
_22 = _13 as isize;
(*_17) = '\u{47a1e}';
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_24 = (*_17);
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_20) = _21.1;
(*_17) = _24;
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_13 = (-29482_i16) | 9134_i16;
Goto(bb19)
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
RET = _1;
_3 = Adt19::Variant0 { fld0: true,fld1: 88100105865226147557638984499129421426_i128,fld2: 18116329335444976812_u64 };
_1 = RET;
_1 = RET;
_6 = -97_i8;
RET = _1;
_6 = (-121_i8) + 16_i8;
_4 = 127049026_i32 ^ 1729764593_i32;
_1 = RET;
place!(Field::<i128>(Variant(_3, 0), 1)) = 112329821772061012938086585410046993254_i128 << _4;
place!(Field::<u64>(Variant(_3, 0), 2)) = 13284466488368831445_u64 * 14480582781744095255_u64;
place!(Field::<bool>(Variant(_3, 0), 0)) = Field::<u64>(Variant(_3, 0), 2) == Field::<u64>(Variant(_3, 0), 2);
place!(Field::<bool>(Variant(_3, 0), 0)) = !true;
_4 = 746330323_i32 ^ (-2028305871_i32);
place!(Field::<i128>(Variant(_3, 0), 1)) = Field::<bool>(Variant(_3, 0), 0) as i128;
_5 = &_3;
_8.0 = Adt50 { fld0: 157297585309475046407115457645050937614_u128 };
match _8.0.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
157297585309475046407115457645050937614 => bb9,
_ => bb8
}
}
bb29 = {
_18 = &_3;
(*_17) = '\u{9e3c3}';
_9.0.fld0 = Field::<i128>(Variant((*_18), 0), 1) as u128;
_35 = (*_17);
_34 = -Field::<i128>(Variant((*_18), 0), 1);
Goto(bb30)
}
bb30 = {
_19 = _32 * _32;
(*_17) = _35;
_17 = &mut _35;
_13 = 13880_i16;
(*_17) = '\u{3d999}';
_8 = Move(_9);
(*_17) = '\u{62ad1}';
_30.0 = &mut _27;
(*_17) = '\u{d5bdd}';
(*_17) = '\u{4c7b9}';
place!(Field::<bool>(Variant(_3, 0), 0)) = !_15;
_13 = 481_i16;
_21.3 = Adt50 { fld0: _8.0.fld0 };
(*_17) = '\u{8094}';
_9.0.fld0 = _8.0.fld0 << _34;
_8.0 = Adt50 { fld0: _21.3.fld0 };
_30.2.1 = &(*_17);
(*_17) = '\u{e7d41}';
(*_17) = '\u{6e4f3}';
_9 = (Move(_21.3),);
_15 = Field::<bool>(Variant(_3, 0), 0);
(*_17) = '\u{cb2e2}';
_19 = _32;
_22 = !_31;
match _4 {
0 => bb31,
1 => bb32,
2 => bb33,
3 => bb34,
4 => bb35,
340282366920938463463374607430383241453 => bb37,
_ => bb36
}
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_21.2 = core::ptr::addr_of!(_22);
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = _21.1;
_9.0.fld0 = 28948_u16 as u128;
_27 = 23226_u16 as usize;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
place!(Field::<i128>(Variant(_3, 0), 1)) = _27 as i128;
(*_20) = _21.1;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_9 = (Move(_8.0),);
_8.0.fld0 = !_21.3.fld0;
_14 = &_21.2;
_15 = !Field::<bool>(Variant((*_18), 0), 0);
_9.0 = Adt50 { fld0: _8.0.fld0 };
(*_17) = _24;
(*_17) = _24;
(*_17) = _24;
_25 = &_21.2;
Goto(bb20)
}
bb34 = {
Return()
}
bb35 = {
_16.1 = Adt19::Variant0 { fld0: _15,fld1: Field::<i128>(Variant(_3, 0), 1),fld2: 8158640891720098179_u64 };
_13 = Field::<i16>(Variant(_16.3.2, 0), 4) | Field::<i16>(Variant(_16.3.2, 0), 4);
_16.3.0 = Field::<u32>(Variant(_16.3.2, 0), 0) + _16.3.1;
place!(Field::<char>(Variant(_16.3.2, 0), 1)) = (*_17);
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _1;
Goto(bb17)
}
bb36 = {
_12.1 = [11175672089579437695_u64,9633959153620731031_u64,7879700463196085334_u64,3019571979543582957_u64,15679772624361011751_u64,11097110697004078439_u64];
_1 = RET;
RET = _1;
_13 = (-30570_i16);
_4 = -151800431_i32;
_16.3.3 = 0_usize as f32;
place!(Field::<i128>(Variant(_3, 0), 1)) = -110874107459835463026357974848149147893_i128;
_9.0.fld0 = !_8.0.fld0;
_15 = true;
_8 = Move(_9);
_1 = RET;
_9.0.fld0 = !_8.0.fld0;
match _6 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb13,
88 => bb15,
_ => bb14
}
}
bb37 = {
(*_17) = '\u{5935e}';
_4 = (-1348766642_i32) + 458109570_i32;
_43 = (-6332337144637330731_i64) ^ 3806794355137588871_i64;
_7 = Move(_21.2);
_8.0 = Move(_9.0);
(*_20) = [Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2)];
_30.2.0 = core::ptr::addr_of_mut!(_40);
_5 = &_3;
_43 = (-7928022757961778874_i64) - (-6569630735569464349_i64);
(*_17) = '\u{10ea03}';
_42.0 = core::ptr::addr_of_mut!(_40);
_33 = core::ptr::addr_of!(_40);
(*_20) = [Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_18), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_5), 0), 2),Field::<u64>(Variant((*_5), 0), 2)];
(*_33) = _13 as f64;
match _13 {
0 => bb27,
1 => bb22,
2 => bb17,
3 => bb7,
4 => bb25,
5 => bb31,
481 => bb39,
_ => bb38
}
}
bb38 = {
_9.0.fld0 = _6 as u128;
_9.0.fld0 = 71972012219398496891279643062736201879_u128 - 185207671389660961578236477429743385926_u128;
_8 = (Move(_9.0),);
_9.0.fld0 = _8.0.fld0;
_8.0 = Move(_9.0);
_4 = 32580_i16 as i32;
RET = _1;
_12.3.fld0 = !_8.0.fld0;
_13 = (-15083_i16) >> Field::<i128>(Variant(_3, 0), 1);
_14 = &_7;
_8 = (Move(_12.3),);
Goto(bb11)
}
bb39 = {
_28.1 = Adt17::Variant2 { fld0: (*_33),fld1: 63437_u16,fld2: 163_u8 };
_14 = &_7;
(*_33) = Field::<f64>(Variant(_28.1, 2), 0) * Field::<f64>(Variant(_28.1, 2), 0);
(*_20) = _30.3;
place!(Field::<u64>(Variant(_3, 0), 2)) = !3483686972213069133_u64;
(*_17) = '\u{2e20a}';
_30.1.1 = Adt17::Variant1 { fld0: _34,fld1: Field::<u64>(Variant(_3, 0), 2),fld2: _19 };
Goto(bb40)
}
bb40 = {
(*_17) = '\u{fcfbf}';
(*_33) = -Field::<f64>(Variant(_28.1, 2), 0);
(*_33) = Field::<f64>(Variant(_28.1, 2), 0);
match _13 {
0 => bb12,
1 => bb41,
2 => bb42,
3 => bb43,
4 => bb44,
5 => bb45,
6 => bb46,
481 => bb48,
_ => bb47
}
}
bb41 = {
_19 = Field::<u64>(Variant(_3, 0), 2) as f32;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = _21.1;
_24 = (*_17);
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_17) = _24;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_32 = -_19;
_5 = Move(_18);
(*_17) = _24;
(*_20) = _21.1;
_29 = _22 ^ _22;
place!(Field::<u64>(Variant(_3, 0), 2)) = !7299504231752842334_u64;
_17 = &mut _24;
(*_17) = '\u{34238}';
(*_20) = _21.1;
_8 = (Move(_21.3),);
(*_20) = _21.1;
_29 = _15 as isize;
_21.3.fld0 = (*_17) as u128;
_8.0 = Move(_9.0);
match _4 {
0 => bb15,
1 => bb13,
2 => bb12,
3 => bb11,
4 => bb14,
5 => bb10,
340282366920938463463374607430383241453 => bb21,
_ => bb7
}
}
bb42 = {
_22 = -(-9223372036854775808_isize);
(*_17) = '\u{b9ddd}';
place!(Field::<u64>(Variant(_3, 0), 2)) = (*_17) as u64;
(*_17) = '\u{d8ebc}';
_21.3 = Adt50 { fld0: _9.0.fld0 };
(*_20) = _21.1;
(*_17) = '\u{991ec}';
(*_17) = '\u{bbb2a}';
_19 = _4 as f32;
_22 = _13 as isize;
(*_17) = '\u{47a1e}';
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_24 = (*_17);
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = _24;
(*_20) = _21.1;
(*_17) = _24;
_21.1 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_13 = (-29482_i16) | 9134_i16;
Goto(bb19)
}
bb43 = {
_8.0.fld0 = _21.3.fld0;
(*_20) = _21.1;
(*_17) = '\u{dc4ab}';
_31 = -_29;
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_20) = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
_21.3 = Adt50 { fld0: _8.0.fld0 };
(*_20) = _21.1;
_30.3 = [Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_3, 0), 2)];
(*_17) = '\u{10f11f}';
match _4 {
0 => bb14,
1 => bb22,
2 => bb23,
3 => bb24,
4 => bb25,
5 => bb26,
6 => bb27,
340282366920938463463374607430383241453 => bb29,
_ => bb28
}
}
bb44 = {
_12.3 = Adt50 { fld0: _9.0.fld0 };
_16.2 = 25897_u16 as i32;
_6 = 35_i8 | (-20_i8);
_8.0.fld0 = _12.3.fld0;
_16.3.1 = 4231729764_u32 << _12.3.fld0;
RET = _1;
_16.3.2 = Adt17::Variant0 { fld0: _16.3.1,fld1: RET,fld2: 9223372036854775807_isize,fld3: 0_usize,fld4: _13,fld5: 126_u8,fld6: _8.0.fld0 };
place!(Field::<u32>(Variant(_16.3.2, 0), 0)) = _6 as u32;
_6 = !66_i8;
_12.2 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_16.3.2, 0), 2)));
_16.3.0 = 9223372036854775807_isize as u32;
_1 = Field::<char>(Variant(_16.3.2, 0), 1);
place!(Field::<bool>(Variant(_3, 0), 0)) = _15;
_17 = &mut RET;
_16.0 = (*_17);
_16.3.1 = Field::<u32>(Variant(_16.3.2, 0), 0) + Field::<u32>(Variant(_16.3.2, 0), 0);
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _16.0;
(*_17) = Field::<char>(Variant(_16.3.2, 0), 1);
(*_17) = _1;
_9.0 = Adt50 { fld0: _8.0.fld0 };
_16.1 = Adt19::Variant2 { fld0: _4,fld1: 1_usize,fld2: 205_u8,fld3: _6,fld4: Field::<i128>(Variant(_3, 0), 1) };
_1 = (*_17);
place!(Field::<u32>(Variant(_16.3.2, 0), 0)) = _16.3.0 >> _4;
_12.2 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_16.3.2, 0), 2)));
match Field::<i16>(Variant(_16.3.2, 0), 4) {
340282366920938463463374607431768180886 => bb16,
_ => bb4
}
}
bb45 = {
Return()
}
bb46 = {
_6 = !26_i8;
_4 = (-1977208523_i32);
_6 = _8.0.fld0 as i8;
_4 = (-917722456_i32);
_1 = RET;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-155388707460859618820126471210140707219_i128) - (-107757502008485065991428165376559946323_i128);
_8.0 = Adt50 { fld0: 262644473571107444148430529037996287291_u128 };
RET = _1;
_1 = RET;
_8.0.fld0 = 547039474_u32 as u128;
_9.0 = Move(_8.0);
_8.0.fld0 = _9.0.fld0;
_9 = (Move(_8.0),);
_9.0.fld0 = !75531403001743826383562529803129082284_u128;
_8 = (Move(_9.0),);
place!(Field::<bool>(Variant(_3, 0), 0)) = false ^ true;
_9.0 = Adt50 { fld0: _8.0.fld0 };
place!(Field::<u64>(Variant(_3, 0), 2)) = !612792326982716199_u64;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-166848123227820650993176368348251613329_i128) & 87609242963281824578817854705294276864_i128;
Call(_7 = fn6(Move(_5), Move(_8.0), _6, _1, Field::<i128>(Variant(_3, 0), 1), _1, RET, RET, _6, _4, _9.0.fld0, Field::<i128>(Variant(_3, 0), 1)), ReturnTo(bb10), UnwindUnreachable())
}
bb47 = {
Return()
}
bb48 = {
_28.1 = _30.1.1;
(*_33) = _6 as f64;
(*_33) = 32581_u16 as f64;
(*_17) = '\u{2bb05}';
(*_20) = [Field::<u64>(Variant(_28.1, 1), 1),Field::<u64>(Variant(_28.1, 1), 1),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_30.1.1, 1), 1),Field::<u64>(Variant(_3, 0), 2),Field::<u64>(Variant(_28.1, 1), 1)];
Goto(bb49)
}
bb49 = {
(*_33) = _19 as f64;
_48 = _34 >= Field::<i128>(Variant((*_5), 0), 1);
place!(Field::<u64>(Variant(_30.1.1, 1), 1)) = 3704980710_u32 as u64;
_42.1 = Move(_30.2.1);
_9.0 = Move(_8.0);
_41 = _6;
_50 = [(*_17),(*_17),(*_17),(*_17),(*_17),(*_17),(*_17)];
_26 = [4274_u16];
_23 = Move(_33);
Goto(bb50)
}
bb50 = {
Call(_57 = dump_var(Move(_26), Move(_41), Move(_48), Move(_15)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_57 = dump_var(Move(_24), Move(_22), Move(_6), Move(_50)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: &'static Adt19,mut _2: Adt50,mut _3: i8,mut _4: char,mut _5: i128,mut _6: char,mut _7: char,mut _8: char,mut _9: i8,mut _10: i32,mut _11: u128,mut _12: i128) -> *const isize {
mir! {
type RET = *const isize;
let _13: u8;
let _14: char;
let _15: &'static mut [u64; 6];
let _16: i128;
let _17: (&'static char,);
let _18: f32;
let _19: [char; 7];
let _20: i32;
let _21: *const f64;
let _22: Adt40;
let _23: isize;
let _24: char;
let _25: char;
let _26: &'static f32;
let _27: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _28: [u64; 6];
let _29: *mut *const bool;
let _30: bool;
let _31: u128;
let _32: bool;
let _33: *const i64;
let _34: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _35: isize;
let _36: bool;
let _37: f32;
let _38: &'static mut *const i64;
let _39: isize;
let _40: [u16; 1];
let _41: &'static char;
let _42: (usize, bool);
let _43: isize;
let _44: f32;
let _45: [u64; 6];
let _46: i8;
let _47: u32;
let _48: &'static mut [u64; 6];
let _49: usize;
let _50: &'static mut usize;
let _51: isize;
let _52: *const Adt19;
let _53: isize;
let _54: i64;
let _55: &'static mut u128;
let _56: u32;
let _57: usize;
let _58: (*mut u16, Adt17);
let _59: *const (&'static char,);
let _60: bool;
let _61: *mut u16;
let _62: f64;
let _63: *const Adt19;
let _64: (&'static char,);
let _65: *const Adt17;
let _66: u64;
let _67: Adt40;
let _68: i128;
let _69: f64;
let _70: ();
let _71: ();
{
_2.fld0 = _11 << _5;
_5 = _12;
_12 = _5;
_9 = -_3;
_2 = Adt50 { fld0: _11 };
_11 = !_2.fld0;
_4 = _8;
_3 = !_9;
_7 = _6;
_8 = _7;
_6 = _7;
_6 = _4;
_14 = _8;
_5 = _12;
_12 = _5 ^ _5;
_2.fld0 = !_11;
_12 = !_5;
_8 = _4;
_7 = _4;
_5 = _3 as i128;
_5 = _12 >> _10;
_10 = -(-1441469775_i32);
_12 = !_5;
_7 = _14;
_8 = _4;
_10 = _5 as i32;
_2.fld0 = !_11;
Call(_11 = core::intrinsics::transmute(_12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !(-960938818_i32);
_13 = 94_u8 - 210_u8;
_16 = _5 << _2.fld0;
_13 = 47_u8;
_16 = !_5;
_17.0 = &_8;
_3 = _9;
_14 = _7;
_3 = _9;
_3 = _9;
_7 = _14;
_17.0 = &_6;
_10 = 703625893_i32;
_10 = (-311769648_i32) * 1986313130_i32;
_8 = _14;
_8 = _4;
_8 = _14;
_5 = !_12;
_12 = 1_usize as i128;
_2 = Adt50 { fld0: _11 };
_12 = !_5;
_10 = 284708417_i32 * (-1252695578_i32);
_2.fld0 = _11;
_4 = _6;
Call(_10 = fn7(Move(_17), _8, _14, _16, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.0 = &_14;
_19 = [_14,_14,_8,_6,_8,_8,_6];
_20 = !_10;
_11 = _2.fld0;
_11 = _2.fld0 - _2.fld0;
_16 = _3 as i128;
RET = core::ptr::addr_of!(_22.fld0);
(*RET) = false as isize;
(*RET) = !112_isize;
(*RET) = (-12_isize) * 9223372036854775807_isize;
_12 = _5 * _16;
_20 = _10;
_25 = _14;
_10 = _20;
(*RET) = !(-73_isize);
(*RET) = (-101_isize) & (-71_isize);
(*RET) = (-113_isize) * (-9223372036854775808_isize);
RET = core::ptr::addr_of!((*RET));
_12 = _5 * _5;
_20 = _10 ^ _10;
RET = core::ptr::addr_of!((*RET));
(*RET) = !9223372036854775807_isize;
_9 = true as i8;
_9 = _3 << _20;
Goto(bb3)
}
bb3 = {
(*RET) = (-9223372036854775808_isize) & (-10_isize);
(*RET) = _12 as isize;
(*RET) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
(*RET) = !(-106_isize);
(*RET) = 235435908764660994_u64 as isize;
(*RET) = 4_usize as isize;
(*RET) = (-9223372036854775808_isize) & 9223372036854775807_isize;
_17.0 = &_8;
_12 = _16 + _5;
_3 = _9;
(*RET) = -9223372036854775807_isize;
Call((*RET) = core::intrinsics::bswap(102_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = _4;
_22.fld0 = -9223372036854775807_isize;
_4 = _8;
_23 = (*RET) & (*RET);
_18 = 5_usize as f32;
(*RET) = _23 + _23;
(*RET) = _23 - _23;
_22.fld0 = _23 >> _11;
(*RET) = _20 as isize;
(*RET) = _23 ^ _23;
(*RET) = _13 as isize;
_16 = -_5;
_13 = 242_u8 >> _2.fld0;
_25 = _4;
_18 = 1788205220505624340_i64 as f32;
Goto(bb5)
}
bb5 = {
(*RET) = _8 as isize;
(*RET) = _12 as isize;
(*RET) = _23;
_27.0.0 = &_8;
_16 = _12 & _5;
_23 = (*RET);
(*RET) = _23 | _23;
_14 = _7;
_26 = &_18;
_2.fld0 = _11 | _11;
(*RET) = _23;
(*RET) = _23;
_2.fld0 = _11 + _11;
_9 = _3 ^ _3;
_18 = _3 as f32;
_24 = _4;
(*RET) = -_23;
(*RET) = 13885246575501911374_u64 as isize;
_2.fld0 = _11;
_25 = _8;
_25 = _24;
_30 = _18 != _18;
_4 = _25;
(*RET) = _23 >> _3;
_16 = !_12;
Goto(bb6)
}
bb6 = {
(*RET) = _23 >> _11;
(*RET) = _23;
RET = core::ptr::addr_of!(_23);
_5 = _16;
_18 = (*RET) as f32;
_8 = _6;
_6 = _4;
_17.0 = &_24;
(*RET) = _22.fld0;
_12 = _16 >> (*RET);
_31 = !_2.fld0;
(*RET) = _22.fld0 >> _10;
(*RET) = _22.fld0;
_24 = _14;
_4 = _25;
_2.fld0 = !_11;
(*RET) = _22.fld0 | _22.fld0;
_32 = (*RET) > (*RET);
(*RET) = _22.fld0;
(*RET) = -_22.fld0;
(*RET) = -_22.fld0;
_17.0 = &_7;
_18 = _20 as f32;
_27.0 = (Move(_17.0),);
(*RET) = !_22.fld0;
(*RET) = _22.fld0 * _22.fld0;
_18 = _20 as f32;
Goto(bb7)
}
bb7 = {
(*RET) = _22.fld0 * _22.fld0;
_34.3 = Adt50 { fld0: _2.fld0 };
_3 = _32 as i8;
_27.0.0 = &_8;
_8 = _4;
_9 = _3;
_35 = -(*RET);
_34.1 = [8343393715038106756_u64,18181403476204608950_u64,9486284723176689509_u64,938523851883981785_u64,2645455142091756421_u64,2996656348737645123_u64];
_34.2 = core::ptr::addr_of!((*RET));
(*RET) = _22.fld0;
_5 = _9 as i128;
_34.3 = Move(_2);
(*RET) = _35 ^ _22.fld0;
_20 = _10 + _10;
_34.0 = core::ptr::addr_of!(_27.0);
(*RET) = _22.fld0;
Goto(bb8)
}
bb8 = {
(*RET) = !_22.fld0;
Goto(bb9)
}
bb9 = {
_36 = _30 >= _30;
(*RET) = !_35;
_13 = 81_u8 >> (*RET);
_2 = Adt50 { fld0: _11 };
_17 = (Move(_27.0.0),);
_25 = _6;
_32 = _30;
(*RET) = _34.3.fld0 as isize;
_8 = _7;
_2 = Move(_34.3);
(*RET) = _35 - _35;
_36 = _32;
_35 = -(*RET);
_34.3.fld0 = !_11;
_34.3.fld0 = _11;
(*RET) = -_35;
_39 = (*RET);
_3 = -_9;
_2 = Adt50 { fld0: _11 };
_15 = &mut _34.1;
(*_15) = [16647482566556891138_u64,16079524598908535079_u64,7125723226894605427_u64,4167344194368979634_u64,10615066811421406465_u64,12839030804244411473_u64];
(*RET) = _39 - _35;
_24 = _6;
(*_15) = [18303838666111423374_u64,8480813933744350206_u64,7328141516855732220_u64,14836164693160660715_u64,10427321052218175590_u64,348262783092186505_u64];
(*RET) = _39 * _35;
(*_15) = [3748533650475604620_u64,10681697757574900840_u64,379632897425536931_u64,4606652404253631289_u64,1980654509065893869_u64,15263623203946988727_u64];
Call((*RET) = core::intrinsics::transmute(_39), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_15) = [8847675341103713705_u64,13820304505227714708_u64,15898805762256251646_u64,6383900487715322218_u64,15344161112766845385_u64,9806497969409483168_u64];
(*RET) = _39 + _39;
(*RET) = _35 & _39;
_41 = &_4;
(*_15) = [15042926853620148960_u64,7811705066518618625_u64,1556549807725299972_u64,7792590686016402037_u64,9088987110800489763_u64,3854308323172809848_u64];
_17 = (Move(_41),);
_4 = _6;
_3 = _9 | _9;
_37 = _18 * _18;
Goto(bb11)
}
bb11 = {
(*RET) = !_39;
_42.0 = 6_usize;
_14 = _24;
(*_15) = [5545622431840110497_u64,13145582951364905061_u64,18112329686720550238_u64,8643236738266237716_u64,17181676886206615249_u64,15787343368235287238_u64];
_41 = &_8;
_43 = 1394537890_u32 as isize;
_7 = (*_41);
(*RET) = _39 + _39;
(*_15) = [11591515651467649912_u64,11357363510455547982_u64,16536574364362732944_u64,1352345169918540107_u64,7148148564450852242_u64,7785528854060691015_u64];
(*RET) = _3 as isize;
(*RET) = _39;
(*RET) = 37483_u16 as isize;
(*RET) = -_35;
(*RET) = -_39;
_27.0.0 = &_8;
(*_15) = [764334931367828034_u64,8983288561928577066_u64,12692203258141483595_u64,13612507752335198354_u64,3358235591431349778_u64,4820507133600477141_u64];
_2.fld0 = !_11;
(*_15) = [3367440432232134448_u64,7485424729822133231_u64,3407408137251137158_u64,23992128852505582_u64,4146588899946253256_u64,14461207058472188843_u64];
match _42.0 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb4,
4 => bb8,
5 => bb12,
6 => bb15,
_ => bb14
}
}
bb12 = {
(*RET) = (-9223372036854775808_isize) & (-10_isize);
(*RET) = _12 as isize;
(*RET) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
(*RET) = !(-106_isize);
(*RET) = 235435908764660994_u64 as isize;
(*RET) = 4_usize as isize;
(*RET) = (-9223372036854775808_isize) & 9223372036854775807_isize;
_17.0 = &_8;
_12 = _16 + _5;
_3 = _9;
(*RET) = -9223372036854775807_isize;
Call((*RET) = core::intrinsics::bswap(102_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
(*RET) = _22.fld0 * _22.fld0;
_34.3 = Adt50 { fld0: _2.fld0 };
_3 = _32 as i8;
_27.0.0 = &_8;
_8 = _4;
_9 = _3;
_35 = -(*RET);
_34.1 = [8343393715038106756_u64,18181403476204608950_u64,9486284723176689509_u64,938523851883981785_u64,2645455142091756421_u64,2996656348737645123_u64];
_34.2 = core::ptr::addr_of!((*RET));
(*RET) = _22.fld0;
_5 = _9 as i128;
_34.3 = Move(_2);
(*RET) = _35 ^ _22.fld0;
_20 = _10 + _10;
_34.0 = core::ptr::addr_of!(_27.0);
(*RET) = _22.fld0;
Goto(bb8)
}
bb14 = {
(*RET) = !_22.fld0;
Goto(bb9)
}
bb15 = {
_12 = _16 << _3;
Goto(bb16)
}
bb16 = {
_39 = _23 & (*RET);
(*_15) = [5932416710821431445_u64,2305866258728036455_u64,11758112495605410345_u64,8355568007555859044_u64,14758461328187249455_u64,7629769203095378196_u64];
_36 = !_30;
_19 = [_25,(*_41),_14,(*_41),(*_41),(*_41),(*_41)];
(*RET) = _39 * _39;
(*RET) = _39 | _39;
(*_15) = [12661589424120054786_u64,2309440934979023073_u64,7277357683500322530_u64,5441676104536767053_u64,13348621997899800622_u64,10521895877425812080_u64];
_45 = (*_15);
(*_15) = _45;
_23 = _39 >> _3;
_12 = _13 as i128;
Goto(bb17)
}
bb17 = {
(*_15) = [1761774663821312988_u64,3425590125945491793_u64,1413245774569369833_u64,3356987509489319040_u64,3409626440438728799_u64,17774005940234397669_u64];
_2.fld0 = _11;
_48 = &mut (*_15);
(*RET) = _39 + _39;
_2.fld0 = _11 * _31;
_6 = (*_41);
(*_48) = _45;
_40 = [16306_u16];
_49 = !_42.0;
(*RET) = !_39;
_42.1 = _32;
Call(_51 = core::intrinsics::transmute((*RET)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_48) = [17518364838238365352_u64,17537910282539882723_u64,10071990014351587228_u64,13827426349991639366_u64,11166237710473822970_u64,15326643263857020554_u64];
_10 = _20 << _51;
_10 = _20 & _20;
(*RET) = _37 as isize;
(*_48) = [5938620748282185237_u64,15989882683547661484_u64,9233776756875779362_u64,4907393110163047394_u64,13277502973531525106_u64,12193947598594554464_u64];
_50 = &mut _49;
(*_48) = [9385130045426599396_u64,16478823848826695800_u64,7726738972215985758_u64,13247957747876928091_u64,1879805380841450207_u64,7389800979206363227_u64];
(*_48) = _45;
(*_48) = _45;
(*RET) = -_35;
_22.fld0 = (*RET) | _51;
_50 = &mut _42.0;
_46 = _10 as i8;
_3 = _9;
_40 = [64435_u16];
(*_48) = [17046192463998924700_u64,7957893919546357838_u64,345818887373599848_u64,9046881154131552782_u64,2763484631685590187_u64,17579729624063832589_u64];
_4 = (*_41);
(*_50) = 1_usize;
_51 = _23;
(*RET) = !_35;
(*_50) = 3_usize & 964466471723853894_usize;
_31 = _11 << (*RET);
_48 = Move(_15);
_39 = (*RET);
Goto(bb19)
}
bb19 = {
_32 = _30 & _30;
(*_50) = _10 as usize;
(*RET) = -_22.fld0;
(*RET) = _35 ^ _35;
_47 = !2244271484_u32;
_43 = (*RET) * (*RET);
_23 = _37 as isize;
_33 = core::ptr::addr_of!(_54);
(*RET) = _43;
_47 = 1348443718_u32 << (*RET);
(*_33) = (-1046709186189954495_i64) + (-2086365528491039317_i64);
_20 = _10;
(*_50) = !15118536045208568167_usize;
(*_33) = 4120903281901458802_i64 >> (*RET);
_44 = _18 * _18;
(*_33) = -(-1841075113150158880_i64);
_24 = _14;
_54 = (-4905424600988056728_i64) * (-8945487008353352121_i64);
_47 = !32691573_u32;
_53 = (*RET);
(*_50) = 16545073108239412441_usize;
(*RET) = -_22.fld0;
match (*_50) {
0 => bb15,
1 => bb10,
2 => bb6,
3 => bb9,
4 => bb20,
16545073108239412441 => bb22,
_ => bb21
}
}
bb20 = {
_10 = !(-960938818_i32);
_13 = 94_u8 - 210_u8;
_16 = _5 << _2.fld0;
_13 = 47_u8;
_16 = !_5;
_17.0 = &_8;
_3 = _9;
_14 = _7;
_3 = _9;
_3 = _9;
_7 = _14;
_17.0 = &_6;
_10 = 703625893_i32;
_10 = (-311769648_i32) * 1986313130_i32;
_8 = _14;
_8 = _4;
_8 = _14;
_5 = !_12;
_12 = 1_usize as i128;
_2 = Adt50 { fld0: _11 };
_12 = !_5;
_10 = 284708417_i32 * (-1252695578_i32);
_2.fld0 = _11;
_4 = _6;
Call(_10 = fn7(Move(_17), _8, _14, _16, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb21 = {
(*RET) = _22.fld0 * _22.fld0;
_34.3 = Adt50 { fld0: _2.fld0 };
_3 = _32 as i8;
_27.0.0 = &_8;
_8 = _4;
_9 = _3;
_35 = -(*RET);
_34.1 = [8343393715038106756_u64,18181403476204608950_u64,9486284723176689509_u64,938523851883981785_u64,2645455142091756421_u64,2996656348737645123_u64];
_34.2 = core::ptr::addr_of!((*RET));
(*RET) = _22.fld0;
_5 = _9 as i128;
_34.3 = Move(_2);
(*RET) = _35 ^ _22.fld0;
_20 = _10 + _10;
_34.0 = core::ptr::addr_of!(_27.0);
(*RET) = _22.fld0;
Goto(bb8)
}
bb22 = {
_19 = [_6,_8,(*_41),(*_41),(*_41),_24,_7];
(*RET) = _39 >> (*_33);
(*_50) = 2_usize * 5_usize;
(*_33) = -2780597586739380428_i64;
(*_50) = 9250_u16 as usize;
_10 = _20 >> (*_33);
(*RET) = -_43;
_7 = (*_41);
_13 = 209_u8 >> _5;
_14 = (*_41);
_55 = &mut _2.fld0;
(*_33) = (-1186461059371233423_i64) | (-4113554218672325510_i64);
_56 = _47;
(*_55) = _13 as u128;
_59 = core::ptr::addr_of!(_27.0);
(*_59) = (Move(_41),);
Goto(bb23)
}
bb23 = {
(*_59).0 = &_14;
_21 = core::ptr::addr_of!(_62);
_16 = _5;
_17.0 = Move((*_59).0);
(*_33) = 2187005760486784425_i64;
_55 = &mut _31;
(*_50) = !18152317200302202182_usize;
(*_59).0 = &_24;
(*_59).0 = &_4;
_57 = !(*_50);
(*_50) = _57;
(*_21) = 6065344217992175298_u64 as f64;
_9 = -_46;
_64 = (Move((*_59).0),);
_40 = [21545_u16];
(*_50) = _57;
_6 = _7;
_5 = _12 | _12;
(*_50) = _57 << (*_33);
_7 = _6;
RET = core::ptr::addr_of!((*RET));
Goto(bb24)
}
bb24 = {
Call(_70 = dump_var(Move(_13), Move(_5), Move(_39), Move(_46)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_70 = dump_var(Move(_11), Move(_23), Move(_31), Move(_20)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_70 = dump_var(Move(_49), Move(_9), Move(_42), Move(_3)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_70 = dump_var(Move(_30), Move(_12), Move(_25), Move(_40)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_70 = dump_var(Move(_51), Move(_10), _71, _71), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (&'static char,),mut _2: char,mut _3: char,mut _4: i128,mut _5: char) -> i32 {
mir! {
type RET = i32;
let _6: u32;
let _7: *mut u16;
let _8: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _9: &'static (u32, u32, Adt17, f32);
let _10: Adt60;
let _11: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _12: &'static Adt19;
let _13: *const f64;
let _14: isize;
let _15: isize;
let _16: *mut [char; 6];
let _17: i128;
let _18: *mut *mut *const bool;
let _19: &'static Adt19;
let _20: Adt85;
let _21: i8;
let _22: isize;
let _23: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _24: bool;
let _25: &'static mut *const i64;
let _26: i16;
let _27: i64;
let _28: isize;
let _29: *const isize;
let _30: bool;
let _31: [bool; 6];
let _32: Adt17;
let _33: isize;
let _34: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _35: u128;
let _36: &'static mut *const i64;
let _37: bool;
let _38: f64;
let _39: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _40: f32;
let _41: *const Adt50;
let _42: f64;
let _43: i16;
let _44: char;
let _45: Adt52;
let _46: char;
let _47: *mut u16;
let _48: f32;
let _49: &'static char;
let _50: (Adt50,);
let _51: u128;
let _52: usize;
let _53: &'static mut &'static *mut bool;
let _54: bool;
let _55: (*mut u16, Adt17);
let _56: u8;
let _57: char;
let _58: i32;
let _59: &'static mut [u64; 6];
let _60: i32;
let _61: &'static &'static mut u128;
let _62: u128;
let _63: u128;
let _64: (&'static char,);
let _65: (Adt50,);
let _66: isize;
let _67: u8;
let _68: u8;
let _69: u16;
let _70: *mut *mut *const bool;
let _71: u8;
let _72: &'static usize;
let _73: isize;
let _74: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _75: *const bool;
let _76: *mut *mut f64;
let _77: bool;
let _78: *const Adt50;
let _79: &'static &'static mut [u64; 6];
let _80: [u64; 6];
let _81: f64;
let _82: *const Adt17;
let _83: Adt19;
let _84: f32;
let _85: i32;
let _86: u128;
let _87: &'static (f64, &'static (u32, u32, Adt17, f32));
let _88: f32;
let _89: isize;
let _90: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _91: char;
let _92: isize;
let _93: *const &'static Adt19;
let _94: *const i64;
let _95: isize;
let _96: u32;
let _97: f64;
let _98: &'static mut [u64; 6];
let _99: f64;
let _100: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _101: *mut bool;
let _102: Adt17;
let _103: *mut u32;
let _104: [i8; 6];
let _105: (&'static char,);
let _106: ();
let _107: ();
{
_5 = _3;
Goto(bb1)
}
bb1 = {
_1.0 = &_2;
_2 = _5;
_6 = !2721706768_u32;
_6 = 3662370448_u32;
_6 = 440544114_u32 * 3754242045_u32;
_6 = 2720393366_u32 ^ 588663804_u32;
_2 = _5;
_8.2 = 660409624_i32 & 1638875989_i32;
_8.3.0 = _6 ^ _6;
_8.3.3 = _6 as f32;
_6 = !_8.3.0;
_1.0 = &_5;
_8.2 = 984042911_i32 | (-826699276_i32);
_8.3.3 = (-780196147672983638_i64) as f32;
_8.3.1 = _6 << _4;
_4 = 145_u8 as i128;
_1.0 = &_2;
_5 = _2;
_8.3.3 = 66_isize as f32;
_8.3.2 = Adt17::Variant1 { fld0: _4,fld1: 1952488535195953018_u64,fld2: _8.3.3 };
_8.0 = _2;
_2 = _3;
Goto(bb2)
}
bb2 = {
_8.2 = false as i32;
_1.0 = &_5;
_8.3.1 = _8.3.0 >> _8.3.0;
_8.2 = 677481082_i32;
_8.3.3 = Field::<f32>(Variant(_8.3.2, 1), 2) * Field::<f32>(Variant(_8.3.2, 1), 2);
_8.2 = 2131298356_i32;
_11.1.1 = Adt17::Variant0 { fld0: _6,fld1: _3,fld2: 9223372036854775807_isize,fld3: 16423384475108735223_usize,fld4: 27143_i16,fld5: 139_u8,fld6: 87958451999143468749616834453419666267_u128 };
_11.3 = [13508913562034016167_u64,3716408717223422147_u64,7970699140986511630_u64,14480796784357656379_u64,13126833912239340017_u64,16271185012564737230_u64];
_2 = _3;
_6 = Field::<u32>(Variant(_11.1.1, 0), 0);
place!(Field::<isize>(Variant(_11.1.1, 0), 2)) = !9223372036854775807_isize;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _8.0;
place!(Field::<usize>(Variant(_11.1.1, 0), 3)) = 4533103485472297342_usize + 16840892833596070322_usize;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _3;
_11.1.1 = Adt17::Variant0 { fld0: _8.3.1,fld1: _3,fld2: (-9223372036854775808_isize),fld3: 5627714117750645497_usize,fld4: 14867_i16,fld5: 243_u8,fld6: 13144789790397412560263682774559795303_u128 };
RET = !_8.2;
_11.2.1 = Move(_1.0);
_4 = Field::<i128>(Variant(_8.3.2, 1), 0) ^ Field::<i128>(Variant(_8.3.2, 1), 0);
_5 = _3;
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = 253357603374502356403363327626809664601_u128;
Call(_8.3.2 = fn8(Field::<u128>(Variant(_11.1.1, 0), 6), _2, Field::<char>(Variant(_11.1.1, 0), 1), Field::<u32>(Variant(_11.1.1, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _8.2;
_5 = _3;
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = Field::<u8>(Variant(_8.3.2, 0), 5);
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = Field::<i16>(Variant(_8.3.2, 0), 4) << _4;
_1.0 = &_2;
_11.0 = &mut place!(Field::<usize>(Variant(_8.3.2, 0), 3));
_14 = !9223372036854775807_isize;
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = _6;
_1.0 = &_3;
_11.3 = [3544669317398369938_u64,3337180589954344108_u64,6322135764599789117_u64,6386060722371819768_u64,4826818845821210152_u64,8418072502691372209_u64];
_11.2.1 = &_3;
_4 = (-42547612904784576477147119903431253284_i128) ^ (-132985475812772001283740682822677249948_i128);
match RET {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
2131298356 => bb11,
_ => bb10
}
}
bb4 = {
_8.2 = false as i32;
_1.0 = &_5;
_8.3.1 = _8.3.0 >> _8.3.0;
_8.2 = 677481082_i32;
_8.3.3 = Field::<f32>(Variant(_8.3.2, 1), 2) * Field::<f32>(Variant(_8.3.2, 1), 2);
_8.2 = 2131298356_i32;
_11.1.1 = Adt17::Variant0 { fld0: _6,fld1: _3,fld2: 9223372036854775807_isize,fld3: 16423384475108735223_usize,fld4: 27143_i16,fld5: 139_u8,fld6: 87958451999143468749616834453419666267_u128 };
_11.3 = [13508913562034016167_u64,3716408717223422147_u64,7970699140986511630_u64,14480796784357656379_u64,13126833912239340017_u64,16271185012564737230_u64];
_2 = _3;
_6 = Field::<u32>(Variant(_11.1.1, 0), 0);
place!(Field::<isize>(Variant(_11.1.1, 0), 2)) = !9223372036854775807_isize;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _8.0;
place!(Field::<usize>(Variant(_11.1.1, 0), 3)) = 4533103485472297342_usize + 16840892833596070322_usize;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _3;
_11.1.1 = Adt17::Variant0 { fld0: _8.3.1,fld1: _3,fld2: (-9223372036854775808_isize),fld3: 5627714117750645497_usize,fld4: 14867_i16,fld5: 243_u8,fld6: 13144789790397412560263682774559795303_u128 };
RET = !_8.2;
_11.2.1 = Move(_1.0);
_4 = Field::<i128>(Variant(_8.3.2, 1), 0) ^ Field::<i128>(Variant(_8.3.2, 1), 0);
_5 = _3;
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = 253357603374502356403363327626809664601_u128;
Call(_8.3.2 = fn8(Field::<u128>(Variant(_11.1.1, 0), 6), _2, Field::<char>(Variant(_11.1.1, 0), 1), Field::<u32>(Variant(_11.1.1, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_1.0 = &_2;
_2 = _5;
_6 = !2721706768_u32;
_6 = 3662370448_u32;
_6 = 440544114_u32 * 3754242045_u32;
_6 = 2720393366_u32 ^ 588663804_u32;
_2 = _5;
_8.2 = 660409624_i32 & 1638875989_i32;
_8.3.0 = _6 ^ _6;
_8.3.3 = _6 as f32;
_6 = !_8.3.0;
_1.0 = &_5;
_8.2 = 984042911_i32 | (-826699276_i32);
_8.3.3 = (-780196147672983638_i64) as f32;
_8.3.1 = _6 << _4;
_4 = 145_u8 as i128;
_1.0 = &_2;
_5 = _2;
_8.3.3 = 66_isize as f32;
_8.3.2 = Adt17::Variant1 { fld0: _4,fld1: 1952488535195953018_u64,fld2: _8.3.3 };
_8.0 = _2;
_2 = _3;
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
Return()
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = 79_u8 + 75_u8;
_2 = _3;
place!(Field::<isize>(Variant(_11.1.1, 0), 2)) = -_14;
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = Field::<i16>(Variant(_11.1.1, 0), 4) as u128;
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = _6 - _6;
_14 = 1_usize as isize;
_6 = Field::<u32>(Variant(_11.1.1, 0), 0);
_2 = Field::<char>(Variant(_11.1.1, 0), 1);
Goto(bb12)
}
bb12 = {
_3 = _2;
_6 = Field::<u32>(Variant(_11.1.1, 0), 0);
_14 = Field::<isize>(Variant(_11.1.1, 0), 2) + Field::<isize>(Variant(_11.1.1, 0), 2);
_3 = Field::<char>(Variant(_11.1.1, 0), 1);
_6 = Field::<u32>(Variant(_11.1.1, 0), 0) | Field::<u32>(Variant(_11.1.1, 0), 0);
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = !142840345112249483687380992409231845395_u128;
_5 = Field::<char>(Variant(_11.1.1, 0), 1);
_5 = Field::<char>(Variant(_11.1.1, 0), 1);
RET = Field::<isize>(Variant(_11.1.1, 0), 2) as i32;
_11.1.1 = Adt17::Variant0 { fld0: _6,fld1: _5,fld2: _14,fld3: 17415707798988460834_usize,fld4: (-8891_i16),fld5: 174_u8,fld6: 185796804797492398313489403150459085825_u128 };
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = 169160410489301909566438946709587109696_u128 - 186892286113004706471064254740124765995_u128;
_6 = Field::<u32>(Variant(_11.1.1, 0), 0) ^ Field::<u32>(Variant(_11.1.1, 0), 0);
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _5;
_15 = _14;
_14 = Field::<isize>(Variant(_11.1.1, 0), 2) + _15;
_2 = _5;
_15 = _14 + Field::<isize>(Variant(_11.1.1, 0), 2);
_4 = (-102000171849023544538866269236082253960_i128) << _15;
_6 = !Field::<u32>(Variant(_11.1.1, 0), 0);
_14 = RET as isize;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _2;
place!(Field::<usize>(Variant(_11.1.1, 0), 3)) = !3_usize;
_17 = _4;
_17 = !_4;
RET = true as i32;
Call(_4 = fn9(), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = _6 ^ _6;
_4 = _17 + _17;
_4 = _17 - _17;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _5;
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = _6;
_11.2.1 = &_2;
_14 = _15 << Field::<isize>(Variant(_11.1.1, 0), 2);
_1.0 = &_5;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _2;
_23.1.1 = Adt17::Variant0 { fld0: _6,fld1: _3,fld2: _15,fld3: Field::<usize>(Variant(_11.1.1, 0), 3),fld4: (-6866_i16),fld5: 112_u8,fld6: Field::<u128>(Variant(_11.1.1, 0), 6) };
_5 = _3;
_23.3 = _11.3;
_28 = 10218590801390344644_u64 as isize;
_11.3 = [1852986070255898788_u64,11581391974233716627_u64,1205529733132997343_u64,11753270738043289256_u64,7568858277377384403_u64,18095746726584702367_u64];
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = Field::<isize>(Variant(_23.1.1, 0), 2) as i16;
_11.3 = _23.3;
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = !185_u8;
place!(Field::<i16>(Variant(_23.1.1, 0), 4)) = Field::<char>(Variant(_23.1.1, 0), 1) as i16;
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = Field::<i16>(Variant(_23.1.1, 0), 4) * Field::<i16>(Variant(_23.1.1, 0), 4);
_2 = Field::<char>(Variant(_23.1.1, 0), 1);
_23.3 = [3564690079793163194_u64,1649666027910492913_u64,16397791918849675247_u64,7923398796985428465_u64,76587737497246599_u64,8446223773491854280_u64];
_24 = !true;
_23.2.1 = &_3;
Goto(bb14)
}
bb14 = {
_21 = _17 as i8;
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _2;
_28 = 23597_u16 as isize;
place!(Field::<isize>(Variant(_23.1.1, 0), 2)) = _15;
_23.0 = &mut place!(Field::<usize>(Variant(_23.1.1, 0), 3));
_5 = _3;
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = 3648_i16 << _4;
_3 = Field::<char>(Variant(_11.1.1, 0), 1);
_2 = _5;
_2 = _5;
_26 = Field::<i16>(Variant(_11.1.1, 0), 4);
_22 = _14 & _14;
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = _21 as i16;
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = !15_u8;
_17 = !_4;
Goto(bb15)
}
bb15 = {
place!(Field::<i16>(Variant(_11.1.1, 0), 4)) = _26 | _26;
_33 = _4 as isize;
_11.3 = [12751442376975170518_u64,10247457928174029849_u64,3387742970181226373_u64,15078837107972604709_u64,5808988579390755393_u64,12362766214970131116_u64];
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = 208_u8 * 229_u8;
_14 = _22;
place!(Field::<isize>(Variant(_11.1.1, 0), 2)) = _33 | _22;
_1.0 = &_2;
_22 = Field::<isize>(Variant(_11.1.1, 0), 2) << _6;
_11.2.1 = &_5;
Goto(bb16)
}
bb16 = {
_29 = core::ptr::addr_of!(_14);
(*_29) = _33 >> _22;
_32 = _11.1.1;
(*_29) = Field::<isize>(Variant(_32, 0), 2) + _22;
(*_29) = _22 | Field::<isize>(Variant(_32, 0), 2);
_32 = _11.1.1;
(*_29) = _24 as isize;
Goto(bb17)
}
bb17 = {
place!(Field::<isize>(Variant(_32, 0), 2)) = Field::<isize>(Variant(_11.1.1, 0), 2) + _22;
place!(Field::<isize>(Variant(_11.1.1, 0), 2)) = (*_29);
_28 = _22;
_34.0 = core::ptr::addr_of!(_1);
_30 = _24;
(*_29) = _15;
_34.3 = Adt50 { fld0: Field::<u128>(Variant(_11.1.1, 0), 6) };
_14 = _22;
Goto(bb18)
}
bb18 = {
(*_29) = _28 >> Field::<i16>(Variant(_11.1.1, 0), 4);
(*_29) = _28;
_34.1 = _11.3;
(*_29) = !Field::<isize>(Variant(_32, 0), 2);
_27 = 7739609575186505937_i64 ^ 678466000367125614_i64;
(*_29) = -_28;
_6 = Field::<u32>(Variant(_32, 0), 0) << (*_29);
(*_29) = _28 & Field::<isize>(Variant(_32, 0), 2);
_39.0.0 = &place!(Field::<char>(Variant(_32, 0), 1));
Goto(bb19)
}
bb19 = {
_37 = (*_29) > (*_29);
RET = _27 as i32;
_34.3.fld0 = Field::<u128>(Variant(_32, 0), 6);
place!(Field::<u32>(Variant(_32, 0), 0)) = _6 | _6;
(*_29) = _28 - _15;
(*_29) = Field::<isize>(Variant(_32, 0), 2);
(*_29) = _22 - _28;
_39.1 = _32;
_31 = [_37,_37,_37,_37,_37,_37];
_1.0 = &place!(Field::<char>(Variant(_32, 0), 1));
(*_29) = Field::<isize>(Variant(_32, 0), 2);
_3 = _5;
place!(Field::<u128>(Variant(_32, 0), 6)) = _34.3.fld0 << _28;
(*_29) = Field::<isize>(Variant(_32, 0), 2);
_37 = (*_29) != (*_29);
_33 = (*_29) ^ (*_29);
(*_29) = _33 ^ Field::<isize>(Variant(_32, 0), 2);
_41 = core::ptr::addr_of!(_34.3);
_39.2.fld0 = Field::<char>(Variant(_11.1.1, 0), 1);
(*_41).fld0 = Field::<u128>(Variant(_32, 0), 6) & Field::<u128>(Variant(_32, 0), 6);
place!(Field::<usize>(Variant(_11.1.1, 0), 3)) = _33 as usize;
Call((*_41).fld0 = core::intrinsics::bswap(Field::<u128>(Variant(_32, 0), 6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
(*_41) = Adt50 { fld0: Field::<u128>(Variant(_32, 0), 6) };
_11.1.1 = _32;
_11.2.1 = &place!(Field::<char>(Variant(_39.1, 0), 1));
_39.2.fld0 = Field::<char>(Variant(_11.1.1, 0), 1);
place!(Field::<usize>(Variant(_39.1, 0), 3)) = Field::<usize>(Variant(_32, 0), 3) + Field::<usize>(Variant(_11.1.1, 0), 3);
place!(Field::<usize>(Variant(_39.1, 0), 3)) = Field::<usize>(Variant(_32, 0), 3) - Field::<usize>(Variant(_32, 0), 3);
(*_41) = Adt50 { fld0: Field::<u128>(Variant(_11.1.1, 0), 6) };
_35 = !(*_41).fld0;
_28 = !(*_29);
_15 = (*_29);
_11.1.1 = _32;
Goto(bb21)
}
bb21 = {
_30 = !_37;
(*_41).fld0 = _21 as u128;
(*_41) = Adt50 { fld0: Field::<u128>(Variant(_32, 0), 6) };
(*_41).fld0 = Field::<u128>(Variant(_11.1.1, 0), 6) - _35;
place!(Field::<char>(Variant(_32, 0), 1)) = _3;
(*_41).fld0 = _27 as u128;
_26 = Field::<i16>(Variant(_32, 0), 4) & Field::<i16>(Variant(_11.1.1, 0), 4);
(*_41) = Adt50 { fld0: _35 };
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = Field::<u32>(Variant(_32, 0), 0);
place!(Field::<u128>(Variant(_32, 0), 6)) = (*_41).fld0 ^ (*_41).fld0;
_41 = core::ptr::addr_of!((*_41));
(*_41) = Adt50 { fld0: Field::<u128>(Variant(_32, 0), 6) };
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = Field::<char>(Variant(_11.1.1, 0), 1) as u128;
place!(Field::<u32>(Variant(_11.1.1, 0), 0)) = Field::<u32>(Variant(_32, 0), 0);
_45.fld6.3.1 = Field::<u32>(Variant(_39.1, 0), 0);
(*_41).fld0 = _6 as u128;
_45.fld6.3.0 = Field::<u32>(Variant(_39.1, 0), 0) ^ _45.fld6.3.1;
_41 = core::ptr::addr_of!((*_41));
_22 = 2697071206459696693_u64 as isize;
(*_29) = _28 >> Field::<u32>(Variant(_32, 0), 0);
(*_41) = Adt50 { fld0: Field::<u128>(Variant(_32, 0), 6) };
_45.fld0 = _30;
_46 = _2;
Goto(bb22)
}
bb22 = {
_50.0 = Move((*_41));
(*_41) = Adt50 { fld0: _50.0.fld0 };
(*_41).fld0 = _27 as u128;
_39.3 = &mut (*_41).fld0;
_45.fld6.0 = _39.2.fld0;
Goto(bb23)
}
bb23 = {
_14 = Field::<isize>(Variant(_32, 0), 2);
Goto(bb24)
}
bb24 = {
(*_29) = Field::<isize>(Variant(_39.1, 0), 2) << Field::<isize>(Variant(_11.1.1, 0), 2);
place!(Field::<u32>(Variant(_39.1, 0), 0)) = _45.fld6.3.0 << _28;
_45.fld7 = Field::<usize>(Variant(_11.1.1, 0), 3) as f64;
_45.fld2.3 = _21 as f32;
_4 = _17;
_45.fld6.1 = Adt19::Variant0 { fld0: _45.fld0,fld1: _17,fld2: 18323500138947284590_u64 };
(*_29) = -_33;
_39.0.0 = Move(_1.0);
_28 = -(*_29);
_45.fld3 = core::ptr::addr_of!(_27);
_45.fld6.2 = _45.fld2.3 as i32;
place!(Field::<usize>(Variant(_39.1, 0), 3)) = Field::<usize>(Variant(_11.1.1, 0), 3) ^ Field::<usize>(Variant(_32, 0), 3);
_49 = &_5;
_45.fld1 = _39.2.fld0;
_44 = (*_49);
_45.fld4.3 = Adt19::Variant0 { fld0: _45.fld0,fld1: _4,fld2: 183884832097559545_u64 };
Goto(bb25)
}
bb25 = {
_45.fld2.1 = _45.fld6.3.1 & Field::<u32>(Variant(_39.1, 0), 0);
(*_29) = Field::<u32>(Variant(_39.1, 0), 0) as isize;
_13 = core::ptr::addr_of!(_45.fld7);
place!(Field::<u8>(Variant(_39.1, 0), 5)) = _4 as u8;
(*_13) = (*_29) as f64;
_51 = Field::<u128>(Variant(_32, 0), 6) + Field::<u128>(Variant(_32, 0), 6);
place!(Field::<u128>(Variant(_32, 0), 6)) = _51 >> (*_29);
_43 = _26;
_39.3 = &mut place!(Field::<u128>(Variant(_39.1, 0), 6));
_36 = &mut _45.fld3;
Goto(bb26)
}
bb26 = {
_17 = _4 | _4;
_38 = (*_13) * (*_13);
_13 = core::ptr::addr_of!((*_13));
(*_13) = _38 + _38;
_4 = _17 + _17;
(*_36) = core::ptr::addr_of!(_27);
Goto(bb27)
}
bb27 = {
(*_13) = _38 + _38;
_4 = !_17;
_13 = core::ptr::addr_of!((*_13));
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = Field::<u8>(Variant(_32, 0), 5) << _26;
_52 = !Field::<usize>(Variant(_11.1.1, 0), 3);
place!(Field::<char>(Variant(_11.1.1, 0), 1)) = _3;
_32 = _11.1.1;
(*_13) = -_38;
(*_29) = -_33;
_22 = !(*_29);
(*_13) = _38 - _38;
_48 = Field::<usize>(Variant(_11.1.1, 0), 3) as f32;
(*_36) = core::ptr::addr_of!(_27);
(*_36) = core::ptr::addr_of!(_27);
(*_36) = core::ptr::addr_of!(_27);
place!(Field::<u8>(Variant(_32, 0), 5)) = !Field::<u8>(Variant(_11.1.1, 0), 5);
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = _51 - _35;
place!(Field::<u128>(Variant(_11.1.1, 0), 6)) = Field::<u128>(Variant(_32, 0), 6) * _50.0.fld0;
_57 = Field::<char>(Variant(_32, 0), 1);
(*_29) = _21 as isize;
(*_29) = _33;
Goto(bb28)
}
bb28 = {
_57 = (*_49);
_60 = 44058_u16 as i32;
(*_13) = _38 + _38;
_58 = !RET;
(*_13) = _26 as f64;
_1.0 = Move(_11.2.1);
_22 = _37 as isize;
(*_36) = core::ptr::addr_of!(_27);
_11.1.1 = Adt17::Variant2 { fld0: (*_13),fld1: 6119_u16,fld2: Field::<u8>(Variant(_32, 0), 5) };
(*_36) = core::ptr::addr_of!(_27);
(*_29) = -_28;
_37 = !_30;
_32 = Adt17::Variant0 { fld0: _6,fld1: (*_49),fld2: _15,fld3: _52,fld4: _43,fld5: Field::<u8>(Variant(_11.1.1, 2), 2),fld6: _50.0.fld0 };
(*_29) = _28;
(*_13) = _38;
_11.2.0 = core::ptr::addr_of_mut!((*_13));
(*_36) = core::ptr::addr_of!(_27);
_29 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_32, 0), 2)));
(*_13) = _38 + _38;
_7 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_11.1.1, 2), 1)));
(*_7) = 33287_u16 ^ 35108_u16;
Goto(bb29)
}
bb29 = {
(*_7) = 1942_u16 << (*_29);
(*_29) = _22 - _28;
(*_7) = 10172_u16 - 41757_u16;
_42 = -(*_13);
_62 = _51 >> (*_29);
_1.0 = &(*_49);
(*_36) = core::ptr::addr_of!(_27);
(*_36) = core::ptr::addr_of!(_27);
(*_13) = _27 as f64;
(*_29) = _33 | _33;
(*_7) = !3475_u16;
_40 = _48 + _48;
(*_29) = _22 & _28;
(*_13) = _38 + _42;
(*_36) = core::ptr::addr_of!(_27);
(*_7) = !44892_u16;
_11.1 = (Move(_7), _32);
place!(Field::<u8>(Variant(_32, 0), 5)) = Field::<u8>(Variant(_11.1.1, 0), 5) | Field::<u8>(Variant(_11.1.1, 0), 5);
(*_13) = Field::<usize>(Variant(_32, 0), 3) as f64;
_65.0 = Move(_50.0);
_64 = Move(_1);
(*_36) = core::ptr::addr_of!(_27);
place!(Field::<u32>(Variant(_32, 0), 0)) = _6 * _6;
Goto(bb30)
}
bb30 = {
_31 = [_30,_37,_37,_37,_30,_30];
_50 = (Move(_65.0),);
_5 = _44;
_30 = !_37;
(*_36) = core::ptr::addr_of!(_27);
_26 = _43 * Field::<i16>(Variant(_11.1.1, 0), 4);
(*_29) = _14;
_10 = Adt60::Variant2 { fld0: _30,fld1: 7119762857801601491_u64 };
Goto(bb31)
}
bb31 = {
place!(Field::<u64>(Variant(_10, 2), 1)) = 5508396938323435969_u64 | 10840357034844638585_u64;
_5 = _44;
_7 = Move(_11.1.0);
place!(Field::<u8>(Variant(_11.1.1, 0), 5)) = 57855_u16 as u8;
(*_29) = _15 & Field::<isize>(Variant(_11.1.1, 0), 2);
_11.1 = (Move(_7), _32);
_11.2.0 = core::ptr::addr_of_mut!((*_13));
(*_36) = core::ptr::addr_of!(_27);
(*_13) = _38 - _38;
_33 = (*_29) ^ (*_29);
_38 = (*_13);
_55 = (Move(_11.1.0), _11.1.1);
_66 = -_28;
_11.1.1 = _55.1;
(*_13) = Field::<u128>(Variant(_11.1.1, 0), 6) as f64;
_35 = !_50.0.fld0;
(*_13) = -_42;
Goto(bb32)
}
bb32 = {
_31 = [_37,_37,_30,_37,Field::<bool>(Variant(_10, 2), 0),Field::<bool>(Variant(_10, 2), 0)];
_26 = Field::<i16>(Variant(_55.1, 0), 4) * _43;
place!(Field::<usize>(Variant(_55.1, 0), 3)) = Field::<usize>(Variant(_11.1.1, 0), 3) >> (*_29);
(*_36) = core::ptr::addr_of!(_27);
_65 = Move(_50);
_3 = _5;
_33 = (*_29);
(*_36) = core::ptr::addr_of!(_27);
place!(Field::<isize>(Variant(_32, 0), 2)) = Field::<isize>(Variant(_11.1.1, 0), 2) << _14;
_50 = Move(_65);
_74.3 = &mut place!(Field::<u128>(Variant(_55.1, 0), 6));
Goto(bb33)
}
bb33 = {
_69 = !55136_u16;
_48 = _40 * _40;
(*_13) = (*_29) as f64;
_33 = (*_29);
_77 = _14 == (*_29);
_61 = &_74.3;
(*_29) = -Field::<isize>(Variant(_11.1.1, 0), 2);
_66 = !(*_29);
_44 = _46;
(*_36) = core::ptr::addr_of!(_27);
_47 = core::ptr::addr_of_mut!(_69);
_25 = Move(_36);
_59 = &mut _11.3;
_68 = Field::<u8>(Variant(_32, 0), 5) | Field::<u8>(Variant(_32, 0), 5);
_40 = (*_47) as f32;
(*_13) = _42;
Goto(bb34)
}
bb34 = {
(*_13) = _38 - _42;
_54 = (*_29) < (*_29);
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
_42 = (*_13);
_14 = (*_29) * _66;
_37 = _30;
_7 = core::ptr::addr_of_mut!((*_47));
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
_4 = _17;
_67 = _68;
_41 = core::ptr::addr_of!(_50.0);
place!(Field::<i16>(Variant(_32, 0), 4)) = -_26;
(*_41).fld0 = !_62;
_47 = core::ptr::addr_of_mut!((*_47));
(*_7) = 33903_u16 | 19536_u16;
_71 = _68 + _68;
_82 = core::ptr::addr_of!(_32);
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
(*_7) = 51364_u16 & 24600_u16;
(*_29) = -_33;
_75 = core::ptr::addr_of!(_37);
(*_7) = !37853_u16;
(*_41).fld0 = Field::<u8>(Variant((*_82), 0), 5) as u128;
(*_75) = _54;
_1 = (Move(_49),);
Goto(bb35)
}
bb35 = {
place!(Field::<u8>(Variant((*_82), 0), 5)) = _68;
place!(Field::<u32>(Variant((*_82), 0), 0)) = _6 + _6;
(*_75) = _30 ^ _54;
(*_41).fld0 = _21 as u128;
_73 = (*_29) >> Field::<i16>(Variant((*_82), 0), 4);
_46 = Field::<char>(Variant(_32, 0), 1);
place!(Field::<u128>(Variant((*_82), 0), 6)) = _62 << Field::<u8>(Variant(_32, 0), 5);
(*_29) = -_15;
(*_41) = Adt50 { fld0: Field::<u128>(Variant((*_82), 0), 6) };
(*_41).fld0 = Field::<u128>(Variant((*_82), 0), 6);
_41 = core::ptr::addr_of!(_65.0);
Goto(bb36)
}
bb36 = {
(*_13) = _42;
_75 = core::ptr::addr_of!((*_75));
_88 = _48 - _48;
(*_41).fld0 = !_62;
_42 = _38 * (*_13);
place!(Field::<char>(Variant((*_82), 0), 1)) = _44;
place!(Field::<u32>(Variant((*_82), 0), 0)) = _6 + _6;
Goto(bb37)
}
bb37 = {
place!(Field::<i16>(Variant((*_82), 0), 4)) = _26 | _26;
(*_29) = _28 << Field::<i16>(Variant((*_82), 0), 4);
place!(Field::<u32>(Variant(_32, 0), 0)) = (*_75) as u32;
(*_7) = 17803_u16 - 54025_u16;
_30 = (*_75);
(*_29) = _28;
(*_29) = -_22;
(*_29) = _73 - _15;
place!(Field::<i16>(Variant((*_82), 0), 4)) = _26 ^ _26;
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
place!(Field::<u128>(Variant((*_82), 0), 6)) = (*_41).fld0 - (*_41).fld0;
place!(Field::<u32>(Variant((*_82), 0), 0)) = !_6;
(*_13) = -_38;
place!(Field::<usize>(Variant((*_82), 0), 3)) = !_52;
place!(Field::<usize>(Variant(_32, 0), 3)) = Field::<i16>(Variant((*_82), 0), 4) as usize;
place!(Field::<usize>(Variant((*_82), 0), 3)) = !_52;
(*_7) = !50389_u16;
place!(Field::<char>(Variant((*_82), 0), 1)) = _57;
place!(Field::<i16>(Variant((*_82), 0), 4)) = !_43;
_5 = Field::<char>(Variant((*_82), 0), 1);
(*_13) = _38;
_89 = (*_29);
_88 = _17 as f32;
place!(Field::<u128>(Variant((*_82), 0), 6)) = (*_41).fld0 | (*_41).fld0;
_85 = _58;
(*_7) = 24953_u16;
Goto(bb38)
}
bb38 = {
(*_7) = 63838_u16 ^ 61960_u16;
place!(Field::<u8>(Variant((*_82), 0), 5)) = _67 * _67;
_90.3.fld0 = Field::<u128>(Variant((*_82), 0), 6);
(*_75) = _54;
place!(Field::<char>(Variant((*_82), 0), 1)) = _57;
place!(Field::<i16>(Variant((*_82), 0), 4)) = _43 | _43;
place!(Field::<char>(Variant((*_82), 0), 1)) = _57;
Goto(bb39)
}
bb39 = {
_84 = (*_7) as f32;
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
_90.2 = core::ptr::addr_of!((*_29));
(*_41).fld0 = Field::<usize>(Variant((*_82), 0), 3) as u128;
place!(Field::<usize>(Variant((*_82), 0), 3)) = _52 - _52;
(*_13) = _38 + _38;
(*_7) = (*_75) as u16;
place!(Field::<u8>(Variant((*_82), 0), 5)) = _84 as u8;
_81 = (*_13) * (*_13);
_97 = (*_13);
_65.0.fld0 = (*_75) as u128;
_30 = _54 ^ _54;
(*_13) = _38 * _97;
_84 = _88;
(*_41) = Adt50 { fld0: Field::<u128>(Variant((*_82), 0), 6) };
_75 = core::ptr::addr_of!((*_75));
Goto(bb40)
}
bb40 = {
place!(Field::<u8>(Variant((*_82), 0), 5)) = (*_41).fld0 as u8;
_32 = Adt17::Variant0 { fld0: _6,fld1: _57,fld2: _66,fld3: _52,fld4: _43,fld5: _68,fld6: _51 };
_74.0.0 = &place!(Field::<char>(Variant((*_82), 0), 1));
_78 = core::ptr::addr_of!((*_41));
(*_59) = [Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1),Field::<u64>(Variant(_10, 2), 1)];
Goto(bb41)
}
bb41 = {
_81 = -(*_13);
Goto(bb42)
}
bb42 = {
place!(Field::<u8>(Variant((*_82), 0), 5)) = Field::<i16>(Variant((*_82), 0), 4) as u8;
(*_78) = Adt50 { fld0: Field::<u128>(Variant((*_82), 0), 6) };
_74.2 = Adt34 { fld0: Field::<char>(Variant((*_82), 0), 1) };
(*_7) = 37035_u16;
(*_75) = (*_13) != (*_13);
place!(Field::<u32>(Variant((*_82), 0), 0)) = !_6;
Goto(bb43)
}
bb43 = {
(*_41) = Adt50 { fld0: Field::<u128>(Variant((*_82), 0), 6) };
place!(Field::<char>(Variant((*_82), 0), 1)) = _5;
(*_41).fld0 = Field::<u8>(Variant(_32, 0), 5) as u128;
match (*_7) {
0 => bb36,
1 => bb44,
2 => bb45,
3 => bb46,
4 => bb47,
37035 => bb49,
_ => bb48
}
}
bb44 = {
Return()
}
bb45 = {
_81 = -(*_13);
Goto(bb42)
}
bb46 = {
Return()
}
bb47 = {
(*_13) = _42;
_75 = core::ptr::addr_of!((*_75));
_88 = _48 - _48;
(*_41).fld0 = !_62;
_42 = _38 * (*_13);
place!(Field::<char>(Variant((*_82), 0), 1)) = _44;
place!(Field::<u32>(Variant((*_82), 0), 0)) = _6 + _6;
Goto(bb37)
}
bb48 = {
Return()
}
bb49 = {
place!(Field::<char>(Variant((*_82), 0), 1)) = _57;
(*_7) = 4922_u16 & 20547_u16;
place!(Field::<usize>(Variant((*_82), 0), 3)) = _52 ^ _52;
place!(Field::<char>(Variant((*_82), 0), 1)) = _3;
place!(Field::<char>(Variant((*_82), 0), 1)) = _57;
(*_41).fld0 = !Field::<u128>(Variant((*_82), 0), 6);
_100.0 = &mut place!(Field::<usize>(Variant((*_82), 0), 3));
_98 = &mut (*_59);
_105 = (Move(_74.0.0),);
Goto(bb50)
}
bb50 = {
Call(_106 = dump_var(Move(_52), Move(_33), Move(_37), Move(_3)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_106 = dump_var(Move(_14), Move(_26), Move(_27), Move(_46)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_106 = dump_var(Move(_6), Move(_15), Move(_58), Move(_44)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_106 = dump_var(Move(_31), Move(_28), Move(_21), Move(_30)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_106 = dump_var(Move(_24), Move(_77), Move(_51), _107), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u128,mut _2: char,mut _3: char,mut _4: u32) -> Adt17 {
mir! {
type RET = Adt17;
let _5: Adt34;
let _6: &'static *mut bool;
let _7: Adt85;
let _8: &'static mut [u64; 6];
let _9: char;
let _10: char;
let _11: ();
let _12: ();
{
_1 = 235466828500921589613078963978781680792_u128 + 74229373740061643623685108859328689299_u128;
_3 = _2;
_1 = 115501276919018373558475028079646010364_u128;
_2 = _3;
_3 = _2;
RET = Adt17::Variant0 { fld0: _4,fld1: _2,fld2: 98_isize,fld3: 3211560918657741769_usize,fld4: 20623_i16,fld5: 119_u8,fld6: _1 };
_2 = _3;
RET = Adt17::Variant0 { fld0: _4,fld1: _3,fld2: 9223372036854775807_isize,fld3: 15517143079814237080_usize,fld4: 15847_i16,fld5: 65_u8,fld6: _1 };
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
_5.fld0 = Field::<char>(Variant(RET, 0), 1);
place!(Field::<isize>(Variant(RET, 0), 2)) = (-9223372036854775808_isize);
place!(Field::<u32>(Variant(RET, 0), 0)) = _4 << _1;
_5.fld0 = _2;
place!(Field::<isize>(Variant(RET, 0), 2)) = !101_isize;
place!(Field::<char>(Variant(RET, 0), 1)) = _2;
place!(Field::<u128>(Variant(RET, 0), 6)) = _1 << Field::<u32>(Variant(RET, 0), 0);
place!(Field::<u8>(Variant(RET, 0), 5)) = !221_u8;
place!(Field::<u32>(Variant(RET, 0), 0)) = (-25777_i16) as u32;
_3 = _5.fld0;
_5.fld0 = _3;
place!(Field::<isize>(Variant(RET, 0), 2)) = (-9223372036854775808_isize) - (-102_isize);
_1 = Field::<u128>(Variant(RET, 0), 6) * Field::<u128>(Variant(RET, 0), 6);
place!(Field::<char>(Variant(RET, 0), 1)) = _2;
_2 = _5.fld0;
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
_2 = _5.fld0;
place!(Field::<i16>(Variant(RET, 0), 4)) = 7199_i16 ^ (-28699_i16);
place!(Field::<u32>(Variant(RET, 0), 0)) = _4 ^ _4;
Goto(bb1)
}
bb1 = {
_2 = _3;
place!(Field::<u8>(Variant(RET, 0), 5)) = 61_u8 | 60_u8;
place!(Field::<u32>(Variant(RET, 0), 0)) = _4 ^ _4;
place!(Field::<isize>(Variant(RET, 0), 2)) = (-9223372036854775808_isize) * 9223372036854775807_isize;
place!(Field::<u128>(Variant(RET, 0), 6)) = _1;
place!(Field::<usize>(Variant(RET, 0), 3)) = 16389426598693358902_usize & 1179650265832589713_usize;
_5 = Adt34 { fld0: _3 };
place!(Field::<isize>(Variant(RET, 0), 2)) = 9223372036854775807_isize >> Field::<u32>(Variant(RET, 0), 0);
place!(Field::<char>(Variant(RET, 0), 1)) = _3;
_4 = Field::<u32>(Variant(RET, 0), 0) * Field::<u32>(Variant(RET, 0), 0);
place!(Field::<u32>(Variant(RET, 0), 0)) = (-1490243331_i32) as u32;
_4 = !Field::<u32>(Variant(RET, 0), 0);
_9 = _2;
place!(Field::<i16>(Variant(RET, 0), 4)) = -(-16998_i16);
_5 = Adt34 { fld0: _9 };
_2 = _3;
place!(Field::<usize>(Variant(RET, 0), 3)) = 4398308700010200857_usize * 17322108060570517003_usize;
_10 = _2;
Goto(bb2)
}
bb2 = {
Call(_11 = dump_var(Move(_2), Move(_1), Move(_4), _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9() -> i128 {
mir! {
type RET = i128;
let _1: f32;
let _2: *const f64;
let _3: (&'static mut char, f32, Adt40, usize);
let _4: isize;
let _5: isize;
let _6: char;
let _7: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _8: bool;
let _9: usize;
let _10: f64;
let _11: *mut f64;
let _12: Adt50;
let _13: *const isize;
let _14: &'static char;
let _15: u8;
let _16: Adt40;
let _17: f32;
let _18: Adt52;
let _19: (*mut f64, &'static char);
let _20: Adt85;
let _21: char;
let _22: char;
let _23: Adt40;
let _24: f32;
let _25: [bool; 6];
let _26: f64;
let _27: isize;
let _28: f64;
let _29: &'static &'static mut [u64; 6];
let _30: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _31: char;
let _32: *const f64;
let _33: bool;
let _34: Adt34;
let _35: isize;
let _36: &'static (u32, u32, Adt17, f32);
let _37: Adt19;
let _38: (f64, &'static (u32, u32, Adt17, f32));
let _39: bool;
let _40: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _41: char;
let _42: (*mut f64, &'static char);
let _43: &'static *const *mut u16;
let _44: Adt17;
let _45: isize;
let _46: Adt52;
let _47: u32;
let _48: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _49: *mut u32;
let _50: bool;
let _51: char;
let _52: *mut bool;
let _53: isize;
let _54: &'static *mut bool;
let _55: *mut u16;
let _56: [char; 6];
let _57: i128;
let _58: [u32; 6];
let _59: usize;
let _60: *mut (f64, &'static (u32, u32, Adt17, f32));
let _61: *const Adt50;
let _62: u64;
let _63: u64;
let _64: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _65: Adt40;
let _66: f64;
let _67: *const bool;
let _68: (*mut u16, Adt17);
let _69: bool;
let _70: &'static (f64, &'static (u32, u32, Adt17, f32));
let _71: *const *mut u16;
let _72: (&'static mut char, f32, Adt40, usize);
let _73: i16;
let _74: (*mut u16, Adt17);
let _75: isize;
let _76: i16;
let _77: isize;
let _78: &'static (f64, &'static (u32, u32, Adt17, f32));
let _79: Adt19;
let _80: &'static mut usize;
let _81: isize;
let _82: [char; 1];
let _83: f32;
let _84: (Adt50,);
let _85: f64;
let _86: isize;
let _87: *mut u16;
let _88: f64;
let _89: i16;
let _90: bool;
let _91: (usize, bool);
let _92: &'static mut *const i64;
let _93: &'static (f64, &'static (u32, u32, Adt17, f32));
let _94: isize;
let _95: bool;
let _96: ();
let _97: ();
{
_1 = (-10608_i16) as f32;
RET = 19858658667380579971790672234765960127_i128 << 16_u16;
_1 = RET as f32;
RET = 7099271074324605207891543211408899483_i128 - 101850077463312521583210139738696498321_i128;
_1 = 15750015165809360593_u64 as f32;
RET = 35134599659976527453967029245333094342_i128 >> 194_u8;
_3.1 = (-55_i8) as f32;
_4 = 10870648437568853530_u64 as isize;
_3.3 = !5_usize;
Goto(bb1)
}
bb1 = {
_5 = (-25457_i16) as isize;
_1 = _3.1 - _3.1;
_4 = !_5;
_1 = _3.1 + _3.1;
_3.1 = _1 + _1;
_4 = (-1768139886_i32) as isize;
_3.1 = -_1;
_4 = _5 ^ _5;
RET = 127995922542190879898904039093005722828_i128 - (-82069439070430524940545778269002468342_i128);
_4 = _5 + _5;
_4 = _5 & _5;
_4 = _5 | _5;
_6 = '\u{103263}';
_3.0 = &mut _6;
_7.3 = Adt50 { fld0: 247561673197673804991648116148744186707_u128 };
_7.2 = core::ptr::addr_of!(_4);
_8 = !false;
_4 = (-52_i8) as isize;
_3.2.fld0 = RET as isize;
_7.3 = Adt50 { fld0: 295553453207957397849786029870496857642_u128 };
_4 = _5 >> _5;
_3.1 = -_1;
_11 = core::ptr::addr_of_mut!(_10);
_7.1 = [9456918141271376224_u64,5880308974974232039_u64,10917463742264416971_u64,11221146562179839669_u64,11777581504868436840_u64,15566129769359672382_u64];
match _7.3.fld0 {
295553453207957397849786029870496857642 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
(*_11) = 77_u8 as f64;
_3.2 = Adt40 { fld0: _5,fld1: Move(_11) };
match _7.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
295553453207957397849786029870496857642 => bb9,
_ => bb8
}
}
bb4 = {
Return()
}
bb5 = {
_5 = (-25457_i16) as isize;
_1 = _3.1 - _3.1;
_4 = !_5;
_1 = _3.1 + _3.1;
_3.1 = _1 + _1;
_4 = (-1768139886_i32) as isize;
_3.1 = -_1;
_4 = _5 ^ _5;
RET = 127995922542190879898904039093005722828_i128 - (-82069439070430524940545778269002468342_i128);
_4 = _5 + _5;
_4 = _5 & _5;
_4 = _5 | _5;
_6 = '\u{103263}';
_3.0 = &mut _6;
_7.3 = Adt50 { fld0: 247561673197673804991648116148744186707_u128 };
_7.2 = core::ptr::addr_of!(_4);
_8 = !false;
_4 = (-52_i8) as isize;
_3.2.fld0 = RET as isize;
_7.3 = Adt50 { fld0: 295553453207957397849786029870496857642_u128 };
_4 = _5 >> _5;
_3.1 = -_1;
_11 = core::ptr::addr_of_mut!(_10);
_7.1 = [9456918141271376224_u64,5880308974974232039_u64,10917463742264416971_u64,11221146562179839669_u64,11777581504868436840_u64,15566129769359672382_u64];
match _7.3.fld0 {
295553453207957397849786029870496857642 => bb3,
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
_3.3 = 4301453621644443777_usize;
match _7.3.fld0 {
0 => bb7,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
295553453207957397849786029870496857642 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_5 = (-25457_i16) as isize;
_1 = _3.1 - _3.1;
_4 = !_5;
_1 = _3.1 + _3.1;
_3.1 = _1 + _1;
_4 = (-1768139886_i32) as isize;
_3.1 = -_1;
_4 = _5 ^ _5;
RET = 127995922542190879898904039093005722828_i128 - (-82069439070430524940545778269002468342_i128);
_4 = _5 + _5;
_4 = _5 & _5;
_4 = _5 | _5;
_6 = '\u{103263}';
_3.0 = &mut _6;
_7.3 = Adt50 { fld0: 247561673197673804991648116148744186707_u128 };
_7.2 = core::ptr::addr_of!(_4);
_8 = !false;
_4 = (-52_i8) as isize;
_3.2.fld0 = RET as isize;
_7.3 = Adt50 { fld0: 295553453207957397849786029870496857642_u128 };
_4 = _5 >> _5;
_3.1 = -_1;
_11 = core::ptr::addr_of_mut!(_10);
_7.1 = [9456918141271376224_u64,5880308974974232039_u64,10917463742264416971_u64,11221146562179839669_u64,11777581504868436840_u64,15566129769359672382_u64];
match _7.3.fld0 {
295553453207957397849786029870496857642 => bb3,
_ => bb2
}
}
bb13 = {
_5 = (-25457_i16) as isize;
_1 = _3.1 - _3.1;
_4 = !_5;
_1 = _3.1 + _3.1;
_3.1 = _1 + _1;
_4 = (-1768139886_i32) as isize;
_3.1 = -_1;
_4 = _5 ^ _5;
RET = 127995922542190879898904039093005722828_i128 - (-82069439070430524940545778269002468342_i128);
_4 = _5 + _5;
_4 = _5 & _5;
_4 = _5 | _5;
_6 = '\u{103263}';
_3.0 = &mut _6;
_7.3 = Adt50 { fld0: 247561673197673804991648116148744186707_u128 };
_7.2 = core::ptr::addr_of!(_4);
_8 = !false;
_4 = (-52_i8) as isize;
_3.2.fld0 = RET as isize;
_7.3 = Adt50 { fld0: 295553453207957397849786029870496857642_u128 };
_4 = _5 >> _5;
_3.1 = -_1;
_11 = core::ptr::addr_of_mut!(_10);
_7.1 = [9456918141271376224_u64,5880308974974232039_u64,10917463742264416971_u64,11221146562179839669_u64,11777581504868436840_u64,15566129769359672382_u64];
match _7.3.fld0 {
295553453207957397849786029870496857642 => bb3,
_ => bb2
}
}
bb14 = {
(*_11) = 77_u8 as f64;
_3.2 = Adt40 { fld0: _5,fld1: Move(_11) };
match _7.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
295553453207957397849786029870496857642 => bb9,
_ => bb8
}
}
bb15 = {
_5 = !_4;
_15 = !58_u8;
_10 = _15 as f64;
_17 = 18478_u16 as f32;
_7.1 = [8745356407134095602_u64,4458576626553423427_u64,6486228191724883930_u64,4867926091104551072_u64,7568159232335657810_u64,8298119146995176244_u64];
_7.2 = core::ptr::addr_of!(_3.2.fld0);
_12 = Adt50 { fld0: _7.3.fld0 };
_13 = core::ptr::addr_of!(_18.fld4.1);
_7.2 = core::ptr::addr_of!((*_13));
_7.3 = Adt50 { fld0: _12.fld0 };
_16.fld0 = _3.2.fld0;
_18.fld7 = _10;
(*_13) = _4;
(*_13) = -_3.2.fld0;
_18.fld6.3.1 = 3554880688_u32;
_18.fld1 = '\u{c5032}';
_14 = &_18.fld1;
Goto(bb16)
}
bb16 = {
_17 = _18.fld6.3.1 as f32;
_18.fld6.0 = (*_14);
_16.fld1 = core::ptr::addr_of_mut!(_10);
_19.1 = Move(_14);
(*_13) = _5 << _5;
(*_13) = _3.2.fld0 ^ _16.fld0;
(*_13) = _4 | _5;
_2 = core::ptr::addr_of!(_10);
(*_13) = _4 - _4;
(*_2) = (-3_i8) as f64;
(*_2) = _18.fld7 * _18.fld7;
_18.fld6.3.0 = _18.fld6.3.1 * _18.fld6.3.1;
_18.fld2.1 = _18.fld6.3.1 << (*_13);
_14 = &_18.fld1;
_18.fld2.3 = _1;
_1 = _3.1 + _18.fld2.3;
_16.fld1 = core::ptr::addr_of_mut!((*_2));
Goto(bb17)
}
bb17 = {
_18.fld2.0 = _18.fld2.1 / _18.fld6.3.1;
_15 = 30_u8 & 113_u8;
_18.fld2.1 = _18.fld2.0 >> _18.fld4.1;
_11 = Move(_3.2.fld1);
_18.fld6.3.3 = _3.1 * _18.fld2.3;
_12 = Move(_7.3);
(*_13) = !_4;
_9 = _1 as usize;
(*_13) = _3.2.fld0 << _18.fld2.1;
_7.2 = Move(_13);
_3.1 = _18.fld2.1 as f32;
(*_2) = -_18.fld7;
_18.fld7 = _10 * (*_2);
_24 = _18.fld6.3.3 + _1;
_3.2.fld1 = core::ptr::addr_of_mut!((*_2));
_9 = _3.3 + _3.3;
_7.1 = [17911286658979965830_u64,13581948489091972143_u64,2256633156169431064_u64,14925565735402004805_u64,1986445678937453810_u64,17645693860320182370_u64];
(*_2) = _18.fld7 * _18.fld7;
_18.fld6.2 = !245117125_i32;
(*_2) = _18.fld7 - _18.fld7;
_23 = Adt40 { fld0: _5,fld1: Move(_16.fld1) };
_11 = core::ptr::addr_of_mut!((*_2));
(*_11) = (-21331_i16) as f64;
_16 = Adt40 { fld0: _4,fld1: Move(_23.fld1) };
_18.fld2.0 = _18.fld2.1;
(*_2) = _18.fld7;
_3.2.fld1 = core::ptr::addr_of_mut!(_10);
_24 = _3.1 + _3.1;
_18.fld4.3 = Adt19::Variant0 { fld0: _8,fld1: RET,fld2: 4990461318937783477_u64 };
Goto(bb18)
}
bb18 = {
(*_2) = _18.fld7 * _18.fld7;
_17 = _24 * _18.fld2.3;
_18.fld0 = _8;
_18.fld4.2 = [24832_u16];
_9 = !_3.3;
_23 = Adt40 { fld0: _18.fld4.1,fld1: Move(_3.2.fld1) };
_15 = 46_u8 ^ 59_u8;
_21 = (*_14);
(*_2) = -_18.fld7;
_10 = _15 as f64;
match _18.fld6.3.1 {
0 => bb11,
3554880688 => bb20,
_ => bb19
}
}
bb19 = {
Return()
}
bb20 = {
_3.2.fld0 = !_18.fld4.1;
(*_2) = _18.fld7;
place!(Field::<bool>(Variant(_18.fld4.3, 0), 0)) = _18.fld2.0 < _18.fld2.1;
_18.fld2.1 = !_18.fld2.0;
_7.1 = [11054139389206322913_u64,15538488749461001475_u64,7014529190487040482_u64,15165915500602908595_u64,36724662484291580_u64,15070054865870122008_u64];
_18.fld6.3.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_18.fld4.3, 0), 1),fld1: 6541700604520284923_u64,fld2: _3.1 };
_22 = (*_14);
(*_2) = _12.fld0 as f64;
_18.fld7 = -(*_2);
_26 = (*_2);
_18.fld2.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_18.fld4.3, 0), 1),fld1: 4147536161529008293_u64,fld2: _17 };
_18.fld6.3.1 = _18.fld2.1 * _18.fld6.3.0;
Call(place!(Field::<u64>(Variant(_18.fld4.3, 0), 2)) = fn10(Move(_23.fld1), (*_2), Move(_16), (*_14), Field::<i128>(Variant(_18.fld4.3, 0), 1)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
(*_2) = -_18.fld7;
_30.3.0 = _18.fld2.0;
_23.fld1 = core::ptr::addr_of_mut!((*_2));
_30.0 = (*_14);
_30.2 = _18.fld6.2 << _23.fld0;
_7.3 = Adt50 { fld0: _12.fld0 };
Goto(bb22)
}
bb22 = {
place!(Field::<f32>(Variant(_18.fld2.2, 1), 2)) = _1;
_4 = !_5;
_24 = Field::<f32>(Variant(_18.fld2.2, 1), 2);
_18.fld6.3.1 = _18.fld2.1 - _18.fld2.1;
_19.0 = Move(_11);
_3.2.fld1 = core::ptr::addr_of_mut!(_28);
_5 = (-79_i8) as isize;
_11 = core::ptr::addr_of_mut!((*_2));
_18.fld6.1 = Move(_18.fld4.3);
_7.1 = [Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2)];
Goto(bb23)
}
bb23 = {
_5 = !_18.fld4.1;
_4 = -_3.2.fld0;
(*_2) = _30.2 as f64;
_7.3 = Move(_12);
_30.3.2 = Adt17::Variant2 { fld0: (*_2),fld1: 48004_u16,fld2: _15 };
_34.fld0 = _21;
place!(Field::<u64>(Variant(_18.fld2.2, 1), 1)) = !Field::<u64>(Variant(_18.fld6.1, 0), 2);
_4 = _18.fld4.1 >> _30.2;
_21 = (*_14);
place!(Field::<u64>(Variant(_18.fld6.3.2, 1), 1)) = Field::<u64>(Variant(_18.fld6.1, 0), 2) * Field::<u64>(Variant(_18.fld6.1, 0), 2);
(*_2) = 37_i8 as f64;
_35 = _23.fld0 ^ _4;
_18.fld0 = Field::<bool>(Variant(_18.fld6.1, 0), 0);
place!(Field::<u16>(Variant(_30.3.2, 2), 1)) = 26981_u16 - 61518_u16;
place!(Field::<u64>(Variant(_18.fld2.2, 1), 1)) = !Field::<u64>(Variant(_18.fld6.1, 0), 2);
_18.fld2 = _18.fld6.3;
_7.3 = Adt50 { fld0: 225849027421975147519011957021123422652_u128 };
_11 = core::ptr::addr_of_mut!((*_2));
place!(Field::<i128>(Variant(_18.fld2.2, 1), 0)) = -Field::<i128>(Variant(_18.fld6.3.2, 1), 0);
_18.fld2.3 = _3.1 * Field::<f32>(Variant(_18.fld6.3.2, 1), 2);
(*_11) = Field::<f64>(Variant(_30.3.2, 2), 0) * Field::<f64>(Variant(_30.3.2, 2), 0);
(*_2) = Field::<u8>(Variant(_30.3.2, 2), 2) as f64;
_9 = _3.3 ^ _3.3;
Goto(bb24)
}
bb24 = {
_18.fld6.0 = (*_14);
(*_2) = _26;
_7.3 = Adt50 { fld0: 194612430846548532629170206767536824315_u128 };
(*_2) = -Field::<f64>(Variant(_30.3.2, 2), 0);
(*_2) = -Field::<f64>(Variant(_30.3.2, 2), 0);
place!(Field::<u8>(Variant(_30.3.2, 2), 2)) = !_15;
_30.0 = (*_14);
_18.fld4.3 = Move(_18.fld6.1);
place!(Field::<i128>(Variant(_18.fld6.3.2, 1), 0)) = _30.2 as i128;
place!(Field::<f64>(Variant(_30.3.2, 2), 0)) = (*_2) - (*_2);
(*_2) = Field::<f64>(Variant(_30.3.2, 2), 0) + Field::<f64>(Variant(_30.3.2, 2), 0);
_27 = _35 << Field::<i128>(Variant(_18.fld4.3, 0), 1);
_18.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_30.3.2, 2), 1)));
_18.fld6.3.2 = _30.3.2;
_37 = Move(_18.fld4.3);
(*_2) = Field::<f64>(Variant(_30.3.2, 2), 0) - Field::<f64>(Variant(_18.fld6.3.2, 2), 0);
_18.fld6 = (_30.0, Move(_37), _30.2, _18.fld2);
_12.fld0 = _7.3.fld0 % _7.3.fld0;
_11 = core::ptr::addr_of_mut!((*_2));
_3.1 = _1 + _18.fld2.3;
(*_11) = _18.fld6.2 as f64;
_18.fld2.2 = Adt17::Variant2 { fld0: (*_2),fld1: Field::<u16>(Variant(_30.3.2, 2), 1),fld2: Field::<u8>(Variant(_30.3.2, 2), 2) };
match _7.3.fld0 {
194612430846548532629170206767536824315 => bb26,
_ => bb25
}
}
bb25 = {
place!(Field::<f32>(Variant(_18.fld2.2, 1), 2)) = _1;
_4 = !_5;
_24 = Field::<f32>(Variant(_18.fld2.2, 1), 2);
_18.fld6.3.1 = _18.fld2.1 - _18.fld2.1;
_19.0 = Move(_11);
_3.2.fld1 = core::ptr::addr_of_mut!(_28);
_5 = (-79_i8) as isize;
_11 = core::ptr::addr_of_mut!((*_2));
_18.fld6.1 = Move(_18.fld4.3);
_7.1 = [Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2)];
Goto(bb23)
}
bb26 = {
(*_2) = _26 * _26;
_18.fld2 = _18.fld6.3;
_35 = _3.2.fld0 & _23.fld0;
_3.2.fld1 = core::ptr::addr_of_mut!((*_2));
_34 = Adt34 { fld0: (*_14) };
_18.fld6.3.2 = _18.fld2.2;
_7.3 = Move(_12);
place!(Field::<u64>(Variant(_18.fld2.2, 1), 1)) = !Field::<u64>(Variant(_18.fld6.3.2, 1), 1);
_22 = (*_14);
_4 = -_27;
_13 = core::ptr::addr_of!(_3.2.fld0);
_21 = _18.fld6.0;
(*_13) = _27 + _27;
_38.0 = Field::<f64>(Variant(_30.3.2, 2), 0);
_36 = &_18.fld6.3;
_26 = _30.2 as f64;
_37 = Adt19::Variant2 { fld0: _18.fld6.2,fld1: _3.3,fld2: Field::<u8>(Variant(_30.3.2, 2), 2),fld3: 17_i8,fld4: Field::<i128>(Variant((*_36).2, 1), 0) };
_33 = (*_13) > (*_13);
match _3.3 {
4301453621644443777 => bb27,
_ => bb24
}
}
bb27 = {
_18.fld6.3 = _18.fld2;
_25 = [Field::<bool>(Variant(_18.fld6.1, 0), 0),_18.fld0,_33,Field::<bool>(Variant(_18.fld6.1, 0), 0),Field::<bool>(Variant(_18.fld6.1, 0), 0),_33];
_30.3.1 = (-105_i8) as u32;
_16 = Move(_3.2);
(*_13) = _16.fld0;
_18.fld2.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_18.fld6.1, 0), 1),fld1: Field::<u64>(Variant(_18.fld6.3.2, 1), 1),fld2: _18.fld2.3 };
_18.fld7 = _38.0;
_18.fld4.1 = (*_13) + (*_13);
_18.fld7 = (*_2) * Field::<f64>(Variant(_30.3.2, 2), 0);
_38.1 = &_18.fld2;
_18.fld6.3.3 = _17 - Field::<f32>(Variant(_18.fld2.2, 1), 2);
Goto(bb28)
}
bb28 = {
_18.fld6.2 = Field::<i32>(Variant(_37, 2), 0);
_40.0 = (Move(_14),);
place!(Field::<u64>(Variant(_18.fld6.3.2, 1), 1)) = Field::<u64>(Variant(_18.fld2.2, 1), 1);
_18.fld6.3.1 = _18.fld2.0;
_3.2.fld0 = _35 ^ _16.fld0;
_32 = core::ptr::addr_of!(_38.0);
_44 = _30.3.2;
_19.0 = core::ptr::addr_of_mut!((*_2));
_23.fld0 = (*_13) >> (*_13);
_3.2.fld1 = core::ptr::addr_of_mut!((*_32));
match Field::<usize>(Variant(_37, 2), 1) {
0 => bb15,
1 => bb29,
2 => bb30,
3 => bb31,
4301453621644443777 => bb33,
_ => bb32
}
}
bb29 = {
_5 = (-25457_i16) as isize;
_1 = _3.1 - _3.1;
_4 = !_5;
_1 = _3.1 + _3.1;
_3.1 = _1 + _1;
_4 = (-1768139886_i32) as isize;
_3.1 = -_1;
_4 = _5 ^ _5;
RET = 127995922542190879898904039093005722828_i128 - (-82069439070430524940545778269002468342_i128);
_4 = _5 + _5;
_4 = _5 & _5;
_4 = _5 | _5;
_6 = '\u{103263}';
_3.0 = &mut _6;
_7.3 = Adt50 { fld0: 247561673197673804991648116148744186707_u128 };
_7.2 = core::ptr::addr_of!(_4);
_8 = !false;
_4 = (-52_i8) as isize;
_3.2.fld0 = RET as isize;
_7.3 = Adt50 { fld0: 295553453207957397849786029870496857642_u128 };
_4 = _5 >> _5;
_3.1 = -_1;
_11 = core::ptr::addr_of_mut!(_10);
_7.1 = [9456918141271376224_u64,5880308974974232039_u64,10917463742264416971_u64,11221146562179839669_u64,11777581504868436840_u64,15566129769359672382_u64];
match _7.3.fld0 {
295553453207957397849786029870496857642 => bb3,
_ => bb2
}
}
bb30 = {
(*_2) = _26 * _26;
_18.fld2 = _18.fld6.3;
_35 = _3.2.fld0 & _23.fld0;
_3.2.fld1 = core::ptr::addr_of_mut!((*_2));
_34 = Adt34 { fld0: (*_14) };
_18.fld6.3.2 = _18.fld2.2;
_7.3 = Move(_12);
place!(Field::<u64>(Variant(_18.fld2.2, 1), 1)) = !Field::<u64>(Variant(_18.fld6.3.2, 1), 1);
_22 = (*_14);
_4 = -_27;
_13 = core::ptr::addr_of!(_3.2.fld0);
_21 = _18.fld6.0;
(*_13) = _27 + _27;
_38.0 = Field::<f64>(Variant(_30.3.2, 2), 0);
_36 = &_18.fld6.3;
_26 = _30.2 as f64;
_37 = Adt19::Variant2 { fld0: _18.fld6.2,fld1: _3.3,fld2: Field::<u8>(Variant(_30.3.2, 2), 2),fld3: 17_i8,fld4: Field::<i128>(Variant((*_36).2, 1), 0) };
_33 = (*_13) > (*_13);
match _3.3 {
4301453621644443777 => bb27,
_ => bb24
}
}
bb31 = {
place!(Field::<f32>(Variant(_18.fld2.2, 1), 2)) = _1;
_4 = !_5;
_24 = Field::<f32>(Variant(_18.fld2.2, 1), 2);
_18.fld6.3.1 = _18.fld2.1 - _18.fld2.1;
_19.0 = Move(_11);
_3.2.fld1 = core::ptr::addr_of_mut!(_28);
_5 = (-79_i8) as isize;
_11 = core::ptr::addr_of_mut!((*_2));
_18.fld6.1 = Move(_18.fld4.3);
_7.1 = [Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2)];
Goto(bb23)
}
bb32 = {
(*_11) = 77_u8 as f64;
_3.2 = Adt40 { fld0: _5,fld1: Move(_11) };
match _7.3.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
295553453207957397849786029870496857642 => bb9,
_ => bb8
}
}
bb33 = {
_4 = !_18.fld4.1;
(*_2) = (*_32) * (*_32);
_9 = Field::<usize>(Variant(_37, 2), 1);
(*_32) = (*_2);
_42.0 = core::ptr::addr_of_mut!((*_32));
_19.0 = core::ptr::addr_of_mut!((*_32));
_46.fld2.1 = _30.3.0 ^ _30.3.0;
_46.fld6.3 = (_30.3.0, _18.fld2.1, _30.3.2, Field::<f32>(Variant(_18.fld2.2, 1), 2));
_18.fld2 = (_30.3.0, _46.fld2.1, _30.3.2, _18.fld6.3.3);
_42.0 = core::ptr::addr_of_mut!(_26);
_23.fld0 = _7.3.fld0 as isize;
(*_32) = Field::<u16>(Variant(_18.fld2.2, 2), 1) as f64;
_48.3 = _18.fld2;
(*_32) = (*_2) * (*_2);
_18.fld6.3.1 = !_30.3.0;
(*_32) = (*_2) - (*_2);
_42.1 = &_18.fld1;
place!(Field::<i8>(Variant(_37, 2), 3)) = 102_i8 | 42_i8;
_46.fld6.2 = -_18.fld6.2;
_18.fld6 = (_21, Move(_37), Field::<i32>(Variant(_37, 2), 0), _18.fld2);
(*_2) = (*_32) * (*_32);
Goto(bb34)
}
bb34 = {
(*_13) = _18.fld4.1;
_18.fld6.0 = _34.fld0;
(*_13) = -_4;
_44 = _30.3.2;
_30.3.3 = _18.fld2.3 + _48.3.3;
place!(Field::<u16>(Variant(_48.3.2, 2), 1)) = Field::<u16>(Variant(_44, 2), 1) * Field::<u16>(Variant(_30.3.2, 2), 1);
_30.0 = _18.fld1;
place!(Field::<i128>(Variant(_18.fld6.1, 2), 4)) = -RET;
_7.3.fld0 = 265851167252585672414716164613973246140_u128 - 34900133123565295042088693848543611143_u128;
_47 = _18.fld2.1 ^ _46.fld6.3.1;
_46.fld2.3 = -_30.3.3;
_46.fld4.3 = Adt19::Variant2 { fld0: _30.2,fld1: _3.3,fld2: Field::<u8>(Variant(_46.fld6.3.2, 2), 2),fld3: Field::<i8>(Variant(_18.fld6.1, 2), 3),fld4: Field::<i128>(Variant(_18.fld6.1, 2), 4) };
Goto(bb35)
}
bb35 = {
_1 = _46.fld2.3 - _46.fld2.3;
_48.3.2 = _46.fld6.3.2;
place!(Field::<usize>(Variant(_46.fld4.3, 2), 1)) = _7.3.fld0 as usize;
place!(Field::<usize>(Variant(_46.fld4.3, 2), 1)) = _9;
_52 = core::ptr::addr_of_mut!(_50);
(*_13) = -_27;
_48.1 = Move(_18.fld6.1);
_46.fld2 = _30.3;
place!(Field::<f64>(Variant(_46.fld2.2, 2), 0)) = (*_32) + (*_2);
_46.fld4.1 = (*_13) - (*_13);
(*_13) = _18.fld4.1;
place!(Field::<u8>(Variant(_18.fld6.3.2, 2), 2)) = !Field::<u8>(Variant(_44, 2), 2);
_54 = &_52;
_7.3.fld0 = 303865000200464581990963385751981188922_u128 << (*_13);
_21 = _34.fld0;
_18.fld1 = _34.fld0;
_49 = core::ptr::addr_of_mut!(_18.fld6.3.1);
_2 = core::ptr::addr_of!((*_2));
(*_2) = -(*_32);
_50 = _33 | _33;
_14 = &_22;
place!(Field::<u16>(Variant(_48.3.2, 2), 1)) = Field::<u16>(Variant(_44, 2), 1) | Field::<u16>(Variant(_46.fld2.2, 2), 1);
_46.fld4.2 = _18.fld4.2;
Goto(bb36)
}
bb36 = {
_9 = Field::<usize>(Variant(_46.fld4.3, 2), 1) - Field::<usize>(Variant(_46.fld4.3, 2), 1);
_46.fld4 = (Move(_18.fld4.0), (*_13), _18.fld4.2, Move(_48.1));
place!(Field::<i8>(Variant(_46.fld4.3, 2), 3)) = 57_i8 - 71_i8;
_25 = [(*_52),_8,(*_52),(*_52),(*_52),(*_52)];
_38 = ((*_2), Move(_36));
place!(Field::<i32>(Variant(_46.fld4.3, 2), 0)) = _46.fld6.2 + _18.fld6.2;
place!(Field::<u8>(Variant(_46.fld6.3.2, 2), 2)) = !Field::<u8>(Variant(_44, 2), 2);
_18.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_48.3.2, 2), 1)));
_7.3 = Adt50 { fld0: 165822028648384153925508833087307128824_u128 };
(*_32) = (*_2) * (*_2);
match Field::<usize>(Variant(_46.fld4.3, 2), 1) {
4301453621644443777 => bb37,
_ => bb19
}
}
bb37 = {
_46.fld7 = (*_2) + (*_2);
_46.fld2.1 = _18.fld7 as u32;
_46.fld0 = (*_13) != (*_13);
_58 = [_47,_18.fld2.1,(*_49),(*_49),_47,_48.3.1];
(*_2) = (*_32) * (*_32);
(*_32) = _46.fld7 * (*_2);
(*_2) = (*_32) * (*_32);
_36 = &_30.3;
_11 = Move(_23.fld1);
_7.3 = Adt50 { fld0: 98552280570648215681148088553122572702_u128 };
(*_49) = (*_52) as u32;
_40.0.0 = &(*_14);
_59 = (*_52) as usize;
_62 = 3445246427892585810_u64;
_67 = core::ptr::addr_of!((*_52));
(*_49) = (*_36).0 * _47;
_71 = core::ptr::addr_of!(_64.1.0);
_46.fld2.3 = (*_36).3 - (*_36).3;
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant((*_36).2, 2), 1)));
_18.fld4.3 = Move(_46.fld4.3);
(*_67) = (*_13) > (*_13);
_59 = Field::<usize>(Variant(_18.fld4.3, 2), 1);
match Field::<usize>(Variant(_18.fld4.3, 2), 1) {
0 => bb7,
1 => bb26,
2 => bb21,
3 => bb5,
4301453621644443777 => bb39,
_ => bb38
}
}
bb38 = {
_9 = Field::<usize>(Variant(_46.fld4.3, 2), 1) - Field::<usize>(Variant(_46.fld4.3, 2), 1);
_46.fld4 = (Move(_18.fld4.0), (*_13), _18.fld4.2, Move(_48.1));
place!(Field::<i8>(Variant(_46.fld4.3, 2), 3)) = 57_i8 - 71_i8;
_25 = [(*_52),_8,(*_52),(*_52),(*_52),(*_52)];
_38 = ((*_2), Move(_36));
place!(Field::<i32>(Variant(_46.fld4.3, 2), 0)) = _46.fld6.2 + _18.fld6.2;
place!(Field::<u8>(Variant(_46.fld6.3.2, 2), 2)) = !Field::<u8>(Variant(_44, 2), 2);
_18.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_48.3.2, 2), 1)));
_7.3 = Adt50 { fld0: 165822028648384153925508833087307128824_u128 };
(*_32) = (*_2) * (*_2);
match Field::<usize>(Variant(_46.fld4.3, 2), 1) {
4301453621644443777 => bb37,
_ => bb19
}
}
bb39 = {
(*_32) = -(*_2);
(*_52) = _33 & _46.fld0;
_35 = (-4432209347744545493_i64) as isize;
_18.fld6.3 = (*_36);
_25 = [(*_52),(*_52),(*_52),(*_52),(*_52),(*_52)];
(*_49) = (*_36).0;
_46.fld2.1 = Field::<u16>(Variant(_46.fld6.3.2, 2), 1) as u32;
_18.fld2.3 = (*_36).3;
_53 = (*_14) as isize;
_19.1 = &(*_14);
(*_32) = _46.fld4.1 as f64;
_18.fld6.2 = _46.fld6.2 | Field::<i32>(Variant(_18.fld4.3, 2), 0);
_38.1 = &(*_36);
(*_13) = _7.3.fld0 as isize;
_38.1 = &_46.fld6.3;
(*_32) = Field::<f64>(Variant((*_36).2, 2), 0) - (*_2);
(*_13) = !_4;
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant((*_36).2, 2), 1)));
_46.fld2 = ((*_36).1, (*_36).0, (*_36).2, (*_36).3);
(*_2) = -(*_32);
(*_49) = !_46.fld6.3.0;
Goto(bb40)
}
bb40 = {
(*_52) = _33 & _33;
_69 = (*_52) & (*_52);
(*_2) = (*_32);
_38.0 = -(*_2);
_18.fld6 = ((*_14), Move(_18.fld4.3), _46.fld6.2, _18.fld2);
(*_2) = (*_32);
_48.3.3 = -_30.3.3;
(*_13) = _46.fld4.1;
_46.fld6.3.0 = (*_49) - (*_36).0;
match _62 {
3445246427892585810 => bb41,
_ => bb13
}
}
bb41 = {
_4 = (*_13);
_68.1 = Adt17::Variant2 { fld0: (*_32),fld1: Field::<u16>(Variant(_48.3.2, 2), 1),fld2: Field::<u8>(Variant((*_36).2, 2), 2) };
(*_52) = !_33;
(*_52) = _33 & _69;
_46.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_68.1, 2), 1)));
place!(Field::<u16>(Variant(_46.fld2.2, 2), 1)) = (*_14) as u16;
_56 = [(*_14),(*_14),(*_14),(*_14),(*_14),(*_14)];
(*_32) = Field::<f64>(Variant((*_36).2, 2), 0) + Field::<f64>(Variant(_68.1, 2), 0);
_50 = _69 | _33;
_72.2 = Adt40 { fld0: (*_13),fld1: Move(_42.0) };
_46.fld6.1 = Move(_18.fld6.1);
_64.2.0 = core::ptr::addr_of_mut!(_18.fld7);
_68.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant((*_36).2, 2), 1)));
_45 = Field::<u16>(Variant((*_36).2, 2), 1) as isize;
_67 = core::ptr::addr_of!((*_52));
_68.1 = Adt17::Variant2 { fld0: (*_32),fld1: Field::<u16>(Variant((*_36).2, 2), 1),fld2: Field::<u8>(Variant((*_36).2, 2), 2) };
_76 = (-26826_i16) << (*_13);
(*_67) = (*_13) <= (*_13);
(*_13) = _4;
(*_52) = (*_2) < (*_32);
_68.1 = Adt17::Variant1 { fld0: RET,fld1: _62,fld2: (*_36).3 };
place!(Field::<f64>(Variant(_18.fld6.3.2, 2), 0)) = (*_2);
_30.0 = (*_14);
_57 = (*_14) as i128;
_18.fld6.1 = Move(_46.fld6.1);
(*_13) = _46.fld4.1 & _72.2.fld0;
Goto(bb42)
}
bb42 = {
_65 = Move(_72.2);
_31 = (*_14);
(*_32) = (*_2) - (*_2);
(*_2) = Field::<f64>(Variant((*_36).2, 2), 0);
(*_32) = Field::<f64>(Variant((*_36).2, 2), 0) + Field::<f64>(Variant(_46.fld2.2, 2), 0);
_3.2.fld1 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_36).2, 2), 0)));
_36 = &_18.fld2;
_22 = _34.fld0;
_48.3 = ((*_36).1, (*_49), (*_36).2, (*_36).3);
(*_71) = Move(_18.fld4.0);
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant((*_36).2, 2), 1)));
_75 = _18.fld4.1 << (*_13);
_16.fld1 = core::ptr::addr_of_mut!((*_2));
_63 = Field::<u64>(Variant(_68.1, 1), 1) << (*_13);
_75 = -(*_13);
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_46.fld2.2, 2), 1)));
place!(Field::<u16>(Variant(_48.3.2, 2), 1)) = !Field::<u16>(Variant((*_36).2, 2), 1);
_46.fld6.0 = _22;
_52 = core::ptr::addr_of_mut!((*_52));
Call((*_32) = core::intrinsics::transmute((*_13)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_53 = _75 + (*_13);
(*_52) = !_69;
_18.fld2.1 = (*_49) | (*_36).0;
_14 = Move(_19.1);
_70 = &_38;
_51 = _34.fld0;
(*_52) = _46.fld0;
_30 = (_31, Move(_18.fld6.1), _46.fld6.2, _46.fld2);
_46.fld4.3 = Move(_30.1);
_46.fld6.3.1 = !(*_49);
(*_32) = (*_2) + (*_2);
_72 = (Move(_3.0), _1, Move(_65), Field::<usize>(Variant(_46.fld4.3, 2), 1));
_30.1 = Move(_46.fld4.3);
_48.3.3 = (*_36).3;
_42 = (Move(_11), Move(_40.0.0));
_80 = &mut _59;
_38.0 = Field::<f64>(Variant((*_36).2, 2), 0) * (*_2);
_46.fld2.1 = (*_49) ^ (*_49);
_7.2 = Move(_13);
(*_71) = Move(_46.fld4.0);
_16.fld0 = _72.2.fld0 - _53;
match (*_80) {
0 => bb12,
1 => bb22,
2 => bb14,
3 => bb30,
4301453621644443777 => bb44,
_ => bb37
}
}
bb44 = {
_64.0 = Move(_80);
_46.fld5 = core::ptr::addr_of_mut!(_56);
_18.fld2.2 = _44;
_48 = (_46.fld6.0, Move(_30.1), Field::<i32>(Variant(_30.1, 2), 0), _18.fld6.3);
_18.fld4.3 = Move(_48.1);
_12 = Adt50 { fld0: _7.3.fld0 };
_46.fld7 = _3.3 as f64;
_46.fld2.2 = Adt17::Variant2 { fld0: _38.0,fld1: Field::<u16>(Variant(_18.fld6.3.2, 2), 1),fld2: Field::<u8>(Variant(_44, 2), 2) };
(*_2) = (*_49) as f64;
(*_32) = _72.3 as f64;
_46.fld6.0 = _34.fld0;
_64.1 = (Move(_68.0), _68.1);
_85 = (*_2);
(*_2) = Field::<f64>(Variant(_18.fld6.3.2, 2), 0) + Field::<f64>(Variant(_48.3.2, 2), 0);
_48 = (_31, Move(_18.fld4.3), _46.fld6.2, _30.3);
_46.fld2.1 = !(*_36).0;
match _62 {
0 => bb10,
1 => bb19,
2 => bb3,
3 => bb45,
4 => bb46,
3445246427892585810 => bb48,
_ => bb47
}
}
bb45 = {
place!(Field::<f32>(Variant(_18.fld2.2, 1), 2)) = _1;
_4 = !_5;
_24 = Field::<f32>(Variant(_18.fld2.2, 1), 2);
_18.fld6.3.1 = _18.fld2.1 - _18.fld2.1;
_19.0 = Move(_11);
_3.2.fld1 = core::ptr::addr_of_mut!(_28);
_5 = (-79_i8) as isize;
_11 = core::ptr::addr_of_mut!((*_2));
_18.fld6.1 = Move(_18.fld4.3);
_7.1 = [Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2),Field::<u64>(Variant(_18.fld6.1, 0), 2)];
Goto(bb23)
}
bb46 = {
Return()
}
bb47 = {
_9 = Field::<usize>(Variant(_46.fld4.3, 2), 1) - Field::<usize>(Variant(_46.fld4.3, 2), 1);
_46.fld4 = (Move(_18.fld4.0), (*_13), _18.fld4.2, Move(_48.1));
place!(Field::<i8>(Variant(_46.fld4.3, 2), 3)) = 57_i8 - 71_i8;
_25 = [(*_52),_8,(*_52),(*_52),(*_52),(*_52)];
_38 = ((*_2), Move(_36));
place!(Field::<i32>(Variant(_46.fld4.3, 2), 0)) = _46.fld6.2 + _18.fld6.2;
place!(Field::<u8>(Variant(_46.fld6.3.2, 2), 2)) = !Field::<u8>(Variant(_44, 2), 2);
_18.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_48.3.2, 2), 1)));
_7.3 = Adt50 { fld0: 165822028648384153925508833087307128824_u128 };
(*_32) = (*_2) * (*_2);
match Field::<usize>(Variant(_46.fld4.3, 2), 1) {
4301453621644443777 => bb37,
_ => bb19
}
}
bb48 = {
_18.fld4.3 = Move(_48.1);
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_18.fld2.2, 2), 1)));
_66 = (*_2);
_48.1 = Adt19::Variant2 { fld0: _46.fld6.2,fld1: Field::<usize>(Variant(_18.fld4.3, 2), 1),fld2: Field::<u8>(Variant(_46.fld2.2, 2), 2),fld3: Field::<i8>(Variant(_18.fld4.3, 2), 3),fld4: Field::<i128>(Variant(_68.1, 1), 0) };
place!(Field::<u8>(Variant(_30.3.2, 2), 2)) = Field::<u8>(Variant(_18.fld2.2, 2), 2);
_74.1 = Adt17::Variant0 { fld0: (*_49),fld1: _18.fld6.0,fld2: _18.fld4.1,fld3: _9,fld4: _76,fld5: Field::<u8>(Variant(_18.fld4.3, 2), 2),fld6: _12.fld0 };
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_44, 2), 1)));
_46.fld4.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_30.3.2, 2), 1)));
place!(Field::<f32>(Variant(_68.1, 1), 2)) = _48.2 as f32;
_46.fld4 = (Move((*_71)), _16.fld0, _18.fld4.2, Move(_48.1));
_82 = [_31];
(*_71) = Move(_46.fld4.0);
_83 = (*_36).3 - (*_36).3;
_18.fld6.3.0 = (*_49) << _3.2.fld0;
(*_52) = _69;
Call((*_2) = core::intrinsics::fmaf64(Field::<f64>(Variant(_18.fld6.3.2, 2), 0), Field::<f64>(Variant(_30.3.2, 2), 0), Field::<f64>(Variant(_48.3.2, 2), 0)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_69 = _50 < (*_52);
(*_32) = Field::<u16>(Variant(_18.fld6.3.2, 2), 1) as f64;
_48.1 = Move(_18.fld4.3);
_64.1.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_18.fld6.3.2, 2), 1)));
_79 = Adt19::Variant2 { fld0: _48.2,fld1: _9,fld2: Field::<u8>(Variant(_18.fld6.3.2, 2), 2),fld3: Field::<i8>(Variant(_46.fld4.3, 2), 3),fld4: RET };
place!(Field::<u8>(Variant(_74.1, 0), 5)) = Field::<u8>(Variant(_79, 2), 2) - Field::<u8>(Variant(_46.fld2.2, 2), 2);
_72.2.fld1 = core::ptr::addr_of_mut!(_10);
_23.fld1 = core::ptr::addr_of_mut!((*_2));
place!(Field::<u128>(Variant(_74.1, 0), 6)) = _63 as u128;
(*_49) = Field::<u8>(Variant(_48.1, 2), 2) as u32;
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_48.3.2, 2), 1)));
_46.fld6.3.1 = !_47;
_84 = (Move(_7.3),);
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_44, 2), 1)));
_18.fld2.3 = _46.fld2.3;
place!(Field::<u8>(Variant(_18.fld6.3.2, 2), 2)) = Field::<u8>(Variant(_46.fld2.2, 2), 2) - Field::<u8>(Variant(_30.3.2, 2), 2);
_18.fld4.2 = [Field::<u16>(Variant(_18.fld6.3.2, 2), 1)];
_56 = [Field::<char>(Variant(_74.1, 0), 1),_46.fld6.0,_18.fld6.0,_18.fld6.0,_22,_46.fld6.0];
_60 = core::ptr::addr_of_mut!(_38);
(*_49) = (*_36).0;
_88 = -(*_2);
(*_71) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_30.3.2, 2), 1)));
_40.0.0 = &_46.fld6.0;
_18.fld2.0 = (*_49) >> _46.fld4.1;
(*_49) = Field::<i128>(Variant(_79, 2), 4) as u32;
Goto(bb50)
}
bb50 = {
Call(_96 = dump_var(Move(_50), Move(_8), Move(_5), Move(_31)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_96 = dump_var(Move(_63), Move(_82), Move(_57), Move(_76)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_96 = dump_var(Move(_15), Move(_22), Move(_25), Move(_53)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_96 = dump_var(Move(_51), Move(_58), _97, _97), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *mut f64,mut _2: f64,mut _3: Adt40,mut _4: char,mut _5: i128) -> u64 {
mir! {
type RET = u64;
let _6: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _7: bool;
let _8: &'static i32;
let _9: (*mut f64, &'static char);
let _10: isize;
let _11: (u32, u32, Adt17, f32);
let _12: f32;
let _13: (Adt50,);
let _14: isize;
let _15: i16;
let _16: (u32, u32, Adt17, f32);
let _17: f32;
let _18: *const i64;
let _19: Adt19;
let _20: f32;
let _21: bool;
let _22: f32;
let _23: [u16; 1];
let _24: Adt40;
let _25: isize;
let _26: Adt60;
let _27: &'static &'static mut [u64; 6];
let _28: &'static *const isize;
let _29: &'static i32;
let _30: isize;
let _31: isize;
let _32: *mut [char; 6];
let _33: isize;
let _34: char;
let _35: i8;
let _36: f32;
let _37: u16;
let _38: u16;
let _39: &'static f32;
let _40: bool;
let _41: f64;
let _42: *mut *const bool;
let _43: Adt34;
let _44: i128;
let _45: isize;
let _46: f64;
let _47: u128;
let _48: ();
let _49: ();
{
RET = 10822127477712577340_u64 - 5974455290839797055_u64;
_2 = 17473451484928158524_usize as f64;
_5 = (-845983196787859627008848748286374989_i128);
_3.fld1 = core::ptr::addr_of_mut!(_2);
_3.fld0 = (-9223372036854775808_isize) ^ 106_isize;
_7 = _3.fld0 < _3.fld0;
_5 = 604398908_i32 as i128;
_7 = false;
_3 = Adt40 { fld0: 25_isize,fld1: Move(_1) };
_1 = Move(_3.fld1);
_3.fld1 = core::ptr::addr_of_mut!(_2);
_3.fld0 = 5_isize - (-9223372036854775808_isize);
_3.fld0 = (-9223372036854775808_isize);
_6.0.0 = &_4;
_5 = 94525612627683859521447414582206557000_i128;
_11.3 = (-108_i8) as f32;
_9.0 = Move(_1);
_3.fld1 = core::ptr::addr_of_mut!(_2);
_12 = _11.3 - _11.3;
_5 = !143450257148308076388636206653909878149_i128;
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _12 };
match _3.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463454151235394913435648 => bb9,
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
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
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
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _13.0.fld0 as f64;
(*_1) = Field::<i128>(Variant(_11.2, 1), 0) as f64;
(*_1) = RET as f64;
(*_1) = _11.1 as f64;
_11.0 = _11.1 >> _10;
_12 = _11.3;
Goto(bb14)
}
bb14 = {
_11.0 = !_11.1;
(*_1) = (-191779591_i32) as f64;
_5 = (-102_i8) as i128;
_16.3 = -_12;
(*_1) = 25154_i16 as f64;
_3 = Adt40 { fld0: _10,fld1: Move(_9.0) };
_16.1 = _11.1 | _11.0;
(*_1) = (-7203957207714809355_i64) as f64;
(*_1) = 1958007570470145347_i64 as f64;
_11.3 = Field::<f32>(Variant(_11.2, 1), 2);
(*_1) = 38586_u16 as f64;
(*_1) = (-22902_i16) as f64;
(*_1) = _16.1 as f64;
_16.0 = _11.0 ^ _16.1;
_17 = _12;
(*_1) = 27180_i16 as f64;
(*_1) = 5188527122052165987_i64 as f64;
(*_1) = 63727_u16 as f64;
_11.0 = _16.1;
(*_1) = 88_i8 as f64;
_19 = Adt19::Variant2 { fld0: 615368702_i32,fld1: 2_usize,fld2: 119_u8,fld3: 99_i8,fld4: _5 };
place!(Field::<i128>(Variant(_11.2, 1), 0)) = 1407402818_i32 as i128;
_9.0 = core::ptr::addr_of_mut!((*_1));
(*_1) = 22043_u16 as f64;
_9.0 = core::ptr::addr_of_mut!((*_1));
_11.0 = _13.0.fld0 as u32;
(*_1) = _11.3 as f64;
Goto(bb15)
}
bb15 = {
_16.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_11.2, 1), 0),fld1: Field::<u64>(Variant(_11.2, 1), 1),fld2: Field::<f32>(Variant(_11.2, 1), 2) };
(*_1) = _13.0.fld0 as f64;
(*_1) = 17_u8 as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_16.2 = _11.2;
place!(Field::<i32>(Variant(_19, 2), 0)) = (-5331730726073968901_i64) as i32;
(*_1) = _14 as f64;
place!(Field::<i128>(Variant(_16.2, 1), 0)) = _5;
_13.0.fld0 = !183622342752468479278137193333961030514_u128;
(*_1) = _13.0.fld0 as f64;
_16.0 = !_16.1;
(*_1) = Field::<u64>(Variant(_11.2, 1), 1) as f64;
(*_1) = 13553037591490302617_usize as f64;
place!(Field::<f32>(Variant(_16.2, 1), 2)) = _17 - _16.3;
place!(Field::<i128>(Variant(_16.2, 1), 0)) = _5 & _5;
(*_1) = (-101_i8) as f64;
_3.fld0 = !_10;
place!(Field::<i128>(Variant(_11.2, 1), 0)) = !Field::<i128>(Variant(_16.2, 1), 0);
_16.1 = _11.0 | _11.1;
_1 = core::ptr::addr_of_mut!((*_1));
match _14 {
0 => bb1,
1 => bb6,
2 => bb16,
3 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb16 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb17 = {
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _13.0.fld0 as f64;
(*_1) = Field::<i128>(Variant(_11.2, 1), 0) as f64;
(*_1) = RET as f64;
(*_1) = _11.1 as f64;
_11.0 = _11.1 >> _10;
_12 = _11.3;
Goto(bb14)
}
bb18 = {
Return()
}
bb19 = {
place!(Field::<u8>(Variant(_19, 2), 2)) = 29_u8 * 194_u8;
_11.0 = !_16.1;
_13.0.fld0 = 220973946803646412007759970854293906954_u128 & 281766974498160655746645710431763439135_u128;
_11 = (_16.1, _16.0, _16.2, Field::<f32>(Variant(_16.2, 1), 2));
place!(Field::<u8>(Variant(_19, 2), 2)) = !168_u8;
_16.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_11.2, 1), 0),fld1: Field::<u64>(Variant(_11.2, 1), 1),fld2: _16.3 };
(*_1) = (-181361786937361188_i64) as f64;
_15 = Field::<u64>(Variant(_16.2, 1), 1) as i16;
_7 = false;
place!(Field::<i8>(Variant(_19, 2), 3)) = !115_i8;
_24 = Adt40 { fld0: _10,fld1: Move(_3.fld1) };
place!(Field::<i128>(Variant(_16.2, 1), 0)) = !Field::<i128>(Variant(_11.2, 1), 0);
_24.fld1 = core::ptr::addr_of_mut!((*_1));
_20 = Field::<f32>(Variant(_11.2, 1), 2);
_12 = Field::<i32>(Variant(_19, 2), 0) as f32;
(*_1) = 442_u16 as f64;
_13.0.fld0 = 56642457418431718788804591673474405452_u128;
_3.fld0 = _10 - _24.fld0;
_3.fld1 = core::ptr::addr_of_mut!((*_1));
_24 = Move(_3);
match _14 {
0 => bb1,
1 => bb14,
2 => bb20,
340282366920938463454151235394913435648 => bb22,
_ => bb21
}
}
bb20 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb21 = {
Return()
}
bb22 = {
(*_1) = _5 as f64;
_21 = _7 ^ _7;
(*_1) = Field::<u8>(Variant(_19, 2), 2) as f64;
place!(Field::<i8>(Variant(_19, 2), 3)) = 97_i8;
_26 = Adt60::Variant2 { fld0: _21,fld1: RET };
(*_1) = _20 as f64;
_3 = Move(_24);
_10 = Field::<i8>(Variant(_19, 2), 3) as isize;
_11.2 = Adt17::Variant0 { fld0: _16.0,fld1: _4,fld2: _10,fld3: 10240725762716399325_usize,fld4: _15,fld5: Field::<u8>(Variant(_19, 2), 2),fld6: _13.0.fld0 };
_24 = Adt40 { fld0: _3.fld0,fld1: Move(_9.0) };
match Field::<i8>(Variant(_19, 2), 3) {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
97 => bb28,
_ => bb27
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_16.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_11.2, 1), 0),fld1: Field::<u64>(Variant(_11.2, 1), 1),fld2: Field::<f32>(Variant(_11.2, 1), 2) };
(*_1) = _13.0.fld0 as f64;
(*_1) = 17_u8 as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_16.2 = _11.2;
place!(Field::<i32>(Variant(_19, 2), 0)) = (-5331730726073968901_i64) as i32;
(*_1) = _14 as f64;
place!(Field::<i128>(Variant(_16.2, 1), 0)) = _5;
_13.0.fld0 = !183622342752468479278137193333961030514_u128;
(*_1) = _13.0.fld0 as f64;
_16.0 = !_16.1;
(*_1) = Field::<u64>(Variant(_11.2, 1), 1) as f64;
(*_1) = 13553037591490302617_usize as f64;
place!(Field::<f32>(Variant(_16.2, 1), 2)) = _17 - _16.3;
place!(Field::<i128>(Variant(_16.2, 1), 0)) = _5 & _5;
(*_1) = (-101_i8) as f64;
_3.fld0 = !_10;
place!(Field::<i128>(Variant(_11.2, 1), 0)) = !Field::<i128>(Variant(_16.2, 1), 0);
_16.1 = _11.0 | _11.1;
_1 = core::ptr::addr_of_mut!((*_1));
match _14 {
0 => bb1,
1 => bb6,
2 => bb16,
3 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb26 = {
Return()
}
bb27 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb28 = {
_13.0.fld0 = Field::<u128>(Variant(_11.2, 0), 6);
place!(Field::<usize>(Variant(_19, 2), 1)) = 7_usize;
(*_1) = Field::<i32>(Variant(_19, 2), 0) as f64;
(*_1) = _11.0 as f64;
match Field::<u128>(Variant(_11.2, 0), 6) {
0 => bb29,
1 => bb30,
56642457418431718788804591673474405452 => bb32,
_ => bb31
}
}
bb29 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb30 = {
Return()
}
bb31 = {
Return()
}
bb32 = {
place!(Field::<usize>(Variant(_11.2, 0), 3)) = Field::<usize>(Variant(_19, 2), 1) ^ Field::<usize>(Variant(_19, 2), 1);
match _14 {
0 => bb26,
1 => bb33,
2 => bb34,
3 => bb35,
340282366920938463454151235394913435648 => bb37,
_ => bb36
}
}
bb33 = {
Return()
}
bb34 = {
Return()
}
bb35 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb36 = {
_11.3 = _12 - Field::<f32>(Variant(_11.2, 1), 2);
_11.1 = 3734731048_u32 << _5;
_11.1 = (-19_i8) as u32;
_1 = core::ptr::addr_of_mut!(_2);
_11.2 = Adt17::Variant1 { fld0: _5,fld1: RET,fld2: _11.3 };
(*_1) = _11.1 as f64;
_9.1 = &_4;
(*_1) = _11.1 as f64;
(*_1) = _3.fld0 as f64;
(*_1) = _5 as f64;
_13.0.fld0 = 14834_u16 as u128;
(*_1) = 7_usize as f64;
place!(Field::<f32>(Variant(_11.2, 1), 2)) = -_11.3;
(*_1) = _5 as f64;
(*_1) = _11.1 as f64;
_10 = _3.fld0 | _3.fld0;
_14 = _3.fld0;
(*_1) = 6_usize as f64;
_6.0 = (Move(_9.1),);
(*_1) = _10 as f64;
match _14 {
0 => bb10,
1 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb37 = {
place!(Field::<u32>(Variant(_11.2, 0), 0)) = _11.0;
_23 = [14988_u16];
_22 = Field::<f32>(Variant(_16.2, 1), 2);
(*_1) = 46862_u16 as f64;
_11.0 = 11627_u16 as u32;
_12 = _22 + Field::<f32>(Variant(_16.2, 1), 2);
_14 = _3.fld0 & _3.fld0;
_25 = Field::<isize>(Variant(_11.2, 0), 2) * _24.fld0;
(*_1) = Field::<u8>(Variant(_11.2, 0), 5) as f64;
_8 = &place!(Field::<i32>(Variant(_19, 2), 0));
_20 = _16.3 + Field::<f32>(Variant(_16.2, 1), 2);
_24 = Move(_3);
place!(Field::<u32>(Variant(_11.2, 0), 0)) = _11.1;
place!(Field::<usize>(Variant(_19, 2), 1)) = Field::<usize>(Variant(_11.2, 0), 3) >> _24.fld0;
Goto(bb38)
}
bb38 = {
_7 = !Field::<bool>(Variant(_26, 2), 0);
_12 = _24.fld0 as f32;
_25 = Field::<isize>(Variant(_11.2, 0), 2);
_9.0 = core::ptr::addr_of_mut!((*_1));
_3 = Adt40 { fld0: Field::<isize>(Variant(_11.2, 0), 2),fld1: Move(_9.0) };
place!(Field::<isize>(Variant(_11.2, 0), 2)) = _10 | _14;
(*_1) = _24.fld0 as f64;
_5 = Field::<i128>(Variant(_16.2, 1), 0) << Field::<i128>(Variant(_19, 2), 4);
_16.1 = Field::<u32>(Variant(_11.2, 0), 0) - _11.0;
(*_1) = Field::<i8>(Variant(_19, 2), 3) as f64;
_12 = Field::<f32>(Variant(_16.2, 1), 2);
(*_1) = RET as f64;
_11.0 = !_16.0;
_25 = !Field::<isize>(Variant(_11.2, 0), 2);
place!(Field::<isize>(Variant(_11.2, 0), 2)) = -_25;
_20 = _16.3 * _12;
(*_1) = _14 as f64;
RET = _12 as u64;
place!(Field::<bool>(Variant(_26, 2), 0)) = _21 | _21;
_9.0 = core::ptr::addr_of_mut!((*_1));
_6.0.0 = &place!(Field::<char>(Variant(_11.2, 0), 1));
_31 = _14 | _25;
_12 = _5 as f32;
_12 = _16.1 as f32;
match Field::<i8>(Variant(_19, 2), 3) {
0 => bb13,
1 => bb18,
2 => bb35,
3 => bb8,
4 => bb5,
5 => bb6,
97 => bb39,
_ => bb7
}
}
bb39 = {
place!(Field::<u8>(Variant(_11.2, 0), 5)) = Field::<u8>(Variant(_19, 2), 2);
_24 = Adt40 { fld0: _31,fld1: Move(_9.0) };
(*_1) = _11.3 as f64;
place!(Field::<f32>(Variant(_16.2, 1), 2)) = Field::<u8>(Variant(_19, 2), 2) as f32;
(*_1) = Field::<u64>(Variant(_16.2, 1), 1) as f64;
place!(Field::<i128>(Variant(_16.2, 1), 0)) = _5;
match Field::<i8>(Variant(_19, 2), 3) {
0 => bb22,
1 => bb34,
97 => bb41,
_ => bb40
}
}
bb40 = {
Return()
}
bb41 = {
(*_1) = Field::<i8>(Variant(_19, 2), 3) as f64;
_21 = Field::<bool>(Variant(_26, 2), 0);
RET = !Field::<u64>(Variant(_26, 2), 1);
_20 = _22;
_9 = (Move(_24.fld1), Move(_6.0.0));
_16.0 = (*_1) as u32;
_16 = _11;
(*_1) = Field::<isize>(Variant(_16.2, 0), 2) as f64;
(*_1) = Field::<i128>(Variant(_19, 2), 4) as f64;
(*_1) = Field::<i8>(Variant(_19, 2), 3) as f64;
_29 = &(*_8);
_16 = (_11.1, _11.1, _11.2, _17);
_22 = (*_8) as f32;
(*_1) = _24.fld0 as f64;
place!(Field::<usize>(Variant(_19, 2), 1)) = Field::<usize>(Variant(_11.2, 0), 3);
_24.fld0 = _31 * Field::<isize>(Variant(_16.2, 0), 2);
place!(Field::<usize>(Variant(_16.2, 0), 3)) = Field::<u8>(Variant(_16.2, 0), 5) as usize;
_14 = _31;
(*_1) = Field::<usize>(Variant(_16.2, 0), 3) as f64;
_33 = _31 & Field::<isize>(Variant(_11.2, 0), 2);
(*_1) = 49674_u16 as f64;
_20 = _12 - _11.3;
_10 = Field::<isize>(Variant(_16.2, 0), 2) - _25;
_11.3 = _22;
(*_1) = RET as f64;
(*_1) = Field::<usize>(Variant(_11.2, 0), 3) as f64;
match Field::<i8>(Variant(_19, 2), 3) {
0 => bb1,
1 => bb2,
2 => bb15,
97 => bb43,
_ => bb42
}
}
bb42 = {
_13.0.fld0 = Field::<u128>(Variant(_11.2, 0), 6);
place!(Field::<usize>(Variant(_19, 2), 1)) = 7_usize;
(*_1) = Field::<i32>(Variant(_19, 2), 0) as f64;
(*_1) = _11.0 as f64;
match Field::<u128>(Variant(_11.2, 0), 6) {
0 => bb29,
1 => bb30,
56642457418431718788804591673474405452 => bb32,
_ => bb31
}
}
bb43 = {
_33 = -_24.fld0;
_35 = !Field::<i8>(Variant(_19, 2), 3);
Call(_32 = fn11(Move(_26), Move(_3), (*_29), Move(_1), Move(_19), Move(_13), Move(_9.0)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
_4 = Field::<char>(Variant(_11.2, 0), 1);
_20 = 6310475497798215119_i64 as f32;
match Field::<u128>(Variant(_16.2, 0), 6) {
56642457418431718788804591673474405452 => bb45,
_ => bb35
}
}
bb45 = {
_36 = _17 * _17;
_15 = -Field::<i16>(Variant(_11.2, 0), 4);
_4 = Field::<char>(Variant(_16.2, 0), 1);
place!(Field::<isize>(Variant(_16.2, 0), 2)) = _24.fld0 ^ _14;
_35 = _31 as i8;
_1 = core::ptr::addr_of_mut!(_2);
_17 = _12 + _16.3;
(*_1) = _5 as f64;
(*_1) = 1027200851_i32 as f64;
(*_1) = RET as f64;
_36 = _17 + _20;
place!(Field::<i16>(Variant(_11.2, 0), 4)) = !Field::<i16>(Variant(_16.2, 0), 4);
_14 = 19367_u16 as isize;
_1 = core::ptr::addr_of_mut!((*_1));
_3.fld0 = _31 | _25;
_16.3 = 6998_u16 as f32;
place!(Field::<u32>(Variant(_16.2, 0), 0)) = _11.1;
place!(Field::<usize>(Variant(_16.2, 0), 3)) = Field::<usize>(Variant(_11.2, 0), 3);
_13.0.fld0 = Field::<u128>(Variant(_11.2, 0), 6) | Field::<u128>(Variant(_11.2, 0), 6);
(*_1) = Field::<usize>(Variant(_11.2, 0), 3) as f64;
_34 = _4;
_30 = _24.fld0 >> _11.1;
_16.3 = -_12;
_7 = _22 != _36;
(*_1) = _13.0.fld0 as f64;
_36 = _16.3;
place!(Field::<usize>(Variant(_16.2, 0), 3)) = !Field::<usize>(Variant(_11.2, 0), 3);
_33 = _24.fld0 + Field::<isize>(Variant(_11.2, 0), 2);
(*_1) = Field::<isize>(Variant(_16.2, 0), 2) as f64;
Goto(bb46)
}
bb46 = {
(*_1) = Field::<u32>(Variant(_11.2, 0), 0) as f64;
_9.0 = core::ptr::addr_of_mut!((*_1));
_15 = Field::<i16>(Variant(_11.2, 0), 4) << _33;
place!(Field::<u8>(Variant(_11.2, 0), 5)) = _35 as u8;
_14 = _25;
_23 = [46084_u16];
(*_1) = _5 as f64;
(*_1) = _5 as f64;
_15 = Field::<i16>(Variant(_16.2, 0), 4);
(*_1) = (-2268172355383226691_i64) as f64;
(*_1) = Field::<i16>(Variant(_16.2, 0), 4) as f64;
_39 = &_22;
_16.0 = _11.0 | _16.1;
_23 = [33396_u16];
_7 = _21 | _21;
(*_1) = Field::<i16>(Variant(_11.2, 0), 4) as f64;
_25 = _34 as isize;
Goto(bb47)
}
bb47 = {
(*_1) = 1014379396_i32 as f64;
_3.fld1 = core::ptr::addr_of_mut!((*_1));
_24.fld1 = core::ptr::addr_of_mut!((*_1));
_37 = 921086815_i32 as u16;
_36 = _12 * (*_39);
(*_1) = _15 as f64;
_34 = _4;
place!(Field::<u128>(Variant(_11.2, 0), 6)) = !_13.0.fld0;
_38 = !_37;
place!(Field::<u32>(Variant(_16.2, 0), 0)) = Field::<char>(Variant(_11.2, 0), 1) as u32;
Goto(bb48)
}
bb48 = {
_17 = -_12;
_5 = 132957318909649147554343019188649361020_i128 ^ 103588140876547727252539664710538177348_i128;
place!(Field::<usize>(Variant(_11.2, 0), 3)) = Field::<usize>(Variant(_16.2, 0), 3) << Field::<u8>(Variant(_11.2, 0), 5);
match Field::<u128>(Variant(_16.2, 0), 6) {
0 => bb24,
1 => bb6,
2 => bb49,
56642457418431718788804591673474405452 => bb51,
_ => bb50
}
}
bb49 = {
Return()
}
bb50 = {
place!(Field::<u32>(Variant(_11.2, 0), 0)) = _11.0;
_23 = [14988_u16];
_22 = Field::<f32>(Variant(_16.2, 1), 2);
(*_1) = 46862_u16 as f64;
_11.0 = 11627_u16 as u32;
_12 = _22 + Field::<f32>(Variant(_16.2, 1), 2);
_14 = _3.fld0 & _3.fld0;
_25 = Field::<isize>(Variant(_11.2, 0), 2) * _24.fld0;
(*_1) = Field::<u8>(Variant(_11.2, 0), 5) as f64;
_8 = &place!(Field::<i32>(Variant(_19, 2), 0));
_20 = _16.3 + Field::<f32>(Variant(_16.2, 1), 2);
_24 = Move(_3);
place!(Field::<u32>(Variant(_11.2, 0), 0)) = _11.1;
place!(Field::<usize>(Variant(_19, 2), 1)) = Field::<usize>(Variant(_11.2, 0), 3) >> _24.fld0;
Goto(bb38)
}
bb51 = {
_6.0.0 = &place!(Field::<char>(Variant(_16.2, 0), 1));
(*_1) = _20 as f64;
_13.0.fld0 = !Field::<u128>(Variant(_11.2, 0), 6);
(*_1) = _38 as f64;
_16.0 = Field::<u32>(Variant(_11.2, 0), 0) >> _33;
_24.fld0 = -_30;
_24.fld1 = core::ptr::addr_of_mut!((*_1));
_24 = Move(_3);
(*_1) = Field::<u128>(Variant(_11.2, 0), 6) as f64;
_19 = Adt19::Variant2 { fld0: (-1355074670_i32),fld1: Field::<usize>(Variant(_11.2, 0), 3),fld2: Field::<u8>(Variant(_11.2, 0), 5),fld3: _35,fld4: _5 };
_1 = Move(_9.0);
place!(Field::<char>(Variant(_11.2, 0), 1)) = Field::<char>(Variant(_16.2, 0), 1);
_9.0 = core::ptr::addr_of_mut!(_2);
_1 = core::ptr::addr_of_mut!(_41);
_30 = _33 | _33;
place!(Field::<u32>(Variant(_11.2, 0), 0)) = Field::<i16>(Variant(_11.2, 0), 4) as u32;
(*_1) = _2 * _2;
_3.fld0 = _33 * _31;
place!(Field::<u128>(Variant(_16.2, 0), 6)) = Field::<u128>(Variant(_11.2, 0), 6) ^ Field::<u128>(Variant(_11.2, 0), 6);
_44 = _5 & _5;
_7 = _21;
Goto(bb52)
}
bb52 = {
Call(_48 = dump_var(Move(_31), Move(_34), Move(_33), Move(_15)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_48 = dump_var(Move(_23), Move(_21), Move(_14), Move(_10)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: Adt60,mut _2: Adt40,mut _3: i32,mut _4: *mut f64,mut _5: Adt19,mut _6: (Adt50,),mut _7: *mut f64) -> *mut [char; 6] {
mir! {
type RET = *mut [char; 6];
let _8: i128;
let _9: &'static mut [u64; 6];
let _10: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _11: &'static i32;
let _12: f32;
let _13: (*mut u16, isize, [u16; 1], Adt19);
let _14: &'static f32;
let _15: *mut *mut *const bool;
let _16: Adt19;
let _17: isize;
let _18: isize;
let _19: (*mut u16, isize, [u16; 1], Adt19);
let _20: isize;
let _21: char;
let _22: i32;
let _23: isize;
let _24: *const Adt50;
let _25: *mut u32;
let _26: f64;
let _27: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _28: isize;
let _29: isize;
let _30: &'static usize;
let _31: u32;
let _32: *const Adt17;
let _33: u128;
let _34: bool;
let _35: usize;
let _36: f32;
let _37: i32;
let _38: i8;
let _39: *const Adt50;
let _40: &'static Adt19;
let _41: *mut [char; 6];
let _42: i64;
let _43: &'static *mut bool;
let _44: (&'static mut char, f32, Adt40, usize);
let _45: char;
let _46: Adt34;
let _47: f64;
let _48: u8;
let _49: [u64; 6];
let _50: &'static i32;
let _51: &'static mut [u64; 6];
let _52: &'static f32;
let _53: &'static &'static mut u128;
let _54: char;
let _55: *const Adt50;
let _56: &'static mut char;
let _57: char;
let _58: &'static mut *const i64;
let _59: Adt17;
let _60: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _61: &'static mut usize;
let _62: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _63: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _64: &'static f32;
let _65: f32;
let _66: [char; 1];
let _67: i16;
let _68: *const bool;
let _69: Adt50;
let _70: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _71: Adt34;
let _72: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _73: u64;
let _74: Adt85;
let _75: &'static i32;
let _76: &'static mut &'static mut usize;
let _77: &'static Adt19;
let _78: u128;
let _79: char;
let _80: Adt40;
let _81: u128;
let _82: char;
let _83: *const &'static Adt19;
let _84: Adt60;
let _85: &'static mut usize;
let _86: &'static mut u128;
let _87: f32;
let _88: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _89: &'static *mut bool;
let _90: char;
let _91: *const &'static Adt19;
let _92: &'static *const *mut u16;
let _93: (&'static char,);
let _94: f64;
let _95: (&'static mut char, f32, Adt40, usize);
let _96: &'static mut u128;
let _97: Adt34;
let _98: *mut f64;
let _99: (*mut u16, isize, [u16; 1], Adt19);
let _100: f32;
let _101: *const Adt60;
let _102: isize;
let _103: &'static mut usize;
let _104: i32;
let _105: *const Adt50;
let _106: [char; 1];
let _107: i16;
let _108: (f64, &'static (u32, u32, Adt17, f32));
let _109: isize;
let _110: [char; 6];
let _111: f32;
let _112: i32;
let _113: &'static *const isize;
let _114: &'static mut char;
let _115: f64;
let _116: u64;
let _117: f32;
let _118: f64;
let _119: Adt17;
let _120: *const Adt17;
let _121: (u32, u32, Adt17, f32);
let _122: [u16; 1];
let _123: &'static mut u128;
let _124: &'static *mut bool;
let _125: f32;
let _126: *const f64;
let _127: bool;
let _128: char;
let _129: (usize, bool);
let _130: *const isize;
let _131: Adt60;
let _132: &'static mut u128;
let _133: (Adt50,);
let _134: (Adt50,);
let _135: [bool; 6];
let _136: *const *mut u16;
let _137: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _138: f64;
let _139: &'static mut &'static mut usize;
let _140: isize;
let _141: f32;
let _142: bool;
let _143: f64;
let _144: i64;
let _145: f32;
let _146: [u16; 1];
let _147: f32;
let _148: *const f64;
let _149: [u64; 6];
let _150: i8;
let _151: isize;
let _152: *const Adt19;
let _153: f64;
let _154: f32;
let _155: (u32, u32, Adt17, f32);
let _156: Adt40;
let _157: *mut *mut *const bool;
let _158: bool;
let _159: usize;
let _160: bool;
let _161: bool;
let _162: *const Adt17;
let _163: char;
let _164: *const Adt50;
let _165: *mut *mut *const bool;
let _166: ();
let _167: ();
{
place!(Field::<i32>(Variant(_5, 2), 0)) = _6.0.fld0 as i32;
match _6.0.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
56642457418431718788804591673474405452 => bb7,
_ => bb6
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
_2 = Adt40 { fld0: 39_isize,fld1: Move(_4) };
place!(Field::<usize>(Variant(_5, 2), 1)) = Field::<i8>(Variant(_5, 2), 3) as usize;
place!(Field::<i32>(Variant(_5, 2), 0)) = Field::<usize>(Variant(_5, 2), 1) as i32;
_6.0 = Adt50 { fld0: 139865542653515474803563253774163101685_u128 };
_6.0 = Adt50 { fld0: 169739407588484938734561789405318725128_u128 };
_4 = Move(_7);
place!(Field::<bool>(Variant(_1, 2), 0)) = false;
place!(Field::<u64>(Variant(_1, 2), 1)) = 13137804735889016509_u64 - 16784475122324493441_u64;
_6.0 = Adt50 { fld0: 183492013426021935544943220754016699848_u128 };
_7 = Move(_4);
_4 = Move(_7);
place!(Field::<i8>(Variant(_5, 2), 3)) = (-6545197108461448057_i64) as i8;
place!(Field::<u8>(Variant(_5, 2), 2)) = 152_u8 - 84_u8;
_3 = Field::<i32>(Variant(_5, 2), 0) - Field::<i32>(Variant(_5, 2), 0);
place!(Field::<i32>(Variant(_5, 2), 0)) = 60094_u16 as i32;
_2.fld0 = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<bool>(Variant(_1, 2), 0)) = false | true;
_10.3 = &mut _6.0.fld0;
_7 = Move(_4);
_10.2 = Adt34 { fld0: '\u{8b6d}' };
_10.0.0 = &_10.2.fld0;
place!(Field::<u8>(Variant(_5, 2), 2)) = 178_u8 + 111_u8;
Goto(bb8)
}
bb8 = {
_2.fld0 = (-127_isize) << Field::<u8>(Variant(_5, 2), 2);
_3 = Field::<i32>(Variant(_5, 2), 0) & Field::<i32>(Variant(_5, 2), 0);
_2.fld1 = Move(_7);
place!(Field::<u8>(Variant(_5, 2), 2)) = 37_u8;
place!(Field::<usize>(Variant(_5, 2), 1)) = 8972315276317427650_usize;
_2.fld0 = (-9223372036854775808_isize) >> Field::<u8>(Variant(_5, 2), 2);
_4 = Move(_2.fld1);
place!(Field::<usize>(Variant(_5, 2), 1)) = Field::<i8>(Variant(_5, 2), 3) as usize;
place!(Field::<u64>(Variant(_1, 2), 1)) = Field::<u8>(Variant(_5, 2), 2) as u64;
_13.1 = -_2.fld0;
match Field::<u8>(Variant(_5, 2), 2) {
0 => bb1,
1 => bb2,
37 => bb9,
_ => bb7
}
}
bb9 = {
_2.fld1 = Move(_4);
place!(Field::<i8>(Variant(_5, 2), 3)) = -(-73_i8);
place!(Field::<usize>(Variant(_5, 2), 1)) = 9474298385594988249_usize << _13.1;
_12 = (-4139555626194309840_i64) as f32;
_11 = &_3;
place!(Field::<u64>(Variant(_1, 2), 1)) = 3166917488487845659_u64 & 6223126500084863128_u64;
_12 = 7305412152340961215_i64 as f32;
_7 = Move(_2.fld1);
_14 = &_12;
_2.fld1 = Move(_7);
place!(Field::<u8>(Variant(_5, 2), 2)) = 196_u8 ^ 13_u8;
_4 = Move(_2.fld1);
place!(Field::<i32>(Variant(_5, 2), 0)) = (*_11);
_2 = Adt40 { fld0: _13.1,fld1: Move(_4) };
place!(Field::<u8>(Variant(_5, 2), 2)) = 201_u8 >> (*_11);
_7 = Move(_2.fld1);
_4 = Move(_7);
place!(Field::<i32>(Variant(_5, 2), 0)) = (*_11) - (*_11);
_16 = Move(_5);
place!(Field::<i128>(Variant(_16, 2), 4)) = 3373235786694753472109750103952930437_i128 + 59753539937346998803552751722054961149_i128;
place!(Field::<i8>(Variant(_16, 2), 3)) = Field::<u8>(Variant(_16, 2), 2) as i8;
_8 = Field::<i128>(Variant(_16, 2), 4) ^ Field::<i128>(Variant(_16, 2), 4);
_17 = (*_14) as isize;
Goto(bb10)
}
bb10 = {
place!(Field::<i32>(Variant(_16, 2), 0)) = (*_11) << (*_11);
_3 = Field::<i32>(Variant(_16, 2), 0);
place!(Field::<u8>(Variant(_16, 2), 2)) = !147_u8;
_10.2.fld0 = '\u{3037}';
_5 = Move(_16);
_12 = Field::<u64>(Variant(_1, 2), 1) as f32;
_16 = Move(_5);
_12 = (-7903406871889939050_i64) as f32;
_2.fld1 = Move(_4);
_18 = _17 ^ _13.1;
_4 = Move(_2.fld1);
_7 = Move(_4);
Goto(bb11)
}
bb11 = {
_13.2 = [25888_u16];
_17 = !_2.fld0;
place!(Field::<i32>(Variant(_16, 2), 0)) = -_3;
place!(Field::<i32>(Variant(_16, 2), 0)) = _3;
place!(Field::<i128>(Variant(_16, 2), 4)) = !_8;
_20 = Field::<u8>(Variant(_16, 2), 2) as isize;
_5 = Move(_16);
_3 = _12 as i32;
_27.1 = [Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1)];
_9 = &mut _27.1;
place!(Field::<bool>(Variant(_1, 2), 0)) = _17 < _2.fld0;
place!(Field::<i8>(Variant(_5, 2), 3)) = -(-92_i8);
(*_9) = [Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1)];
_2.fld1 = core::ptr::addr_of_mut!(_26);
place!(Field::<i8>(Variant(_5, 2), 3)) = 806299350_u32 as i8;
_2.fld1 = core::ptr::addr_of_mut!(_26);
(*_9) = [Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant(_1, 2), 1)];
Call(_19.2 = fn12(Move(_10.0.0), Move(_1), Move(_11), Move(_2), Move(_7), (*_9), Move(_9), Move(_5), Move(_14)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2.fld1 = core::ptr::addr_of_mut!(_26);
_28 = 174603406572586667873941330060771928821_u128 as isize;
_19.3 = Adt19::Variant2 { fld0: _3,fld1: 1_usize,fld2: 218_u8,fld3: 120_i8,fld4: _8 };
place!(Field::<usize>(Variant(_19.3, 2), 1)) = 13225612931571621336_usize + 15051651075556063881_usize;
_20 = !_17;
_17 = _18 & _20;
_17 = _18;
_10.0.0 = &_10.2.fld0;
_23 = _17 - _20;
_19.2 = [16898_u16];
_13.3 = Adt19::Variant0 { fld0: true,fld1: _8,fld2: 5321563535312747412_u64 };
_23 = _13.1 << _13.1;
_17 = _13.1 >> _13.1;
_20 = _18 | _18;
Goto(bb13)
}
bb13 = {
_18 = _20;
_13.3 = Adt19::Variant0 { fld0: true,fld1: Field::<i128>(Variant(_19.3, 2), 4),fld2: 11457302119117838290_u64 };
place!(Field::<i8>(Variant(_19.3, 2), 3)) = 7750439875626235358_i64 as i8;
place!(Field::<bool>(Variant(_13.3, 0), 0)) = !true;
_4 = core::ptr::addr_of_mut!(_26);
(*_4) = Field::<i8>(Variant(_19.3, 2), 3) as f64;
place!(Field::<u64>(Variant(_13.3, 0), 2)) = !9556712049499069183_u64;
_21 = _10.2.fld0;
_22 = _20 as i32;
_5 = Move(_13.3);
_13.1 = 211346660354761531936826071821147212997_u128 as isize;
_31 = 3867796776_u32 | 3724300790_u32;
(*_4) = Field::<i128>(Variant(_5, 0), 1) as f64;
(*_4) = 6778325841736790852_i64 as f64;
Call((*_4) = core::intrinsics::transmute(_18), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_4) = (-31099_i16) as f64;
_10.0.0 = &_21;
_19.3 = Move(_5);
_4 = Move(_2.fld1);
_26 = 281016055332481638320245690456640497013_u128 as f64;
_4 = core::ptr::addr_of_mut!(_26);
_11 = &_22;
(*_4) = 1_usize as f64;
_10.2.fld0 = _21;
_25 = core::ptr::addr_of_mut!(_31);
_26 = 10446740491070374606_usize as f64;
_20 = !_23;
_17 = _23 >> (*_11);
_11 = &_3;
(*_25) = 2028658583_u32;
_19.1 = _20;
_2 = Adt40 { fld0: _20,fld1: Move(_4) };
_8 = Field::<i128>(Variant(_19.3, 0), 1);
(*_25) = 626474048_u32 >> Field::<i128>(Variant(_19.3, 0), 1);
_13.1 = !_18;
_23 = -_13.1;
_10.1 = Adt17::Variant2 { fld0: _26,fld1: 25867_u16,fld2: 203_u8 };
(*_25) = 1572181753_u32;
_10.1 = Adt17::Variant0 { fld0: (*_25),fld1: _10.2.fld0,fld2: _19.1,fld3: 17838448015279554368_usize,fld4: 31460_i16,fld5: 227_u8,fld6: 273300829668613374735969011043897334017_u128 };
_2.fld1 = core::ptr::addr_of_mut!(_26);
_35 = !7_usize;
_7 = core::ptr::addr_of_mut!(_26);
match (*_25) {
0 => bb15,
1572181753 => bb17,
_ => bb16
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
(*_25) = !Field::<u32>(Variant(_10.1, 0), 0);
(*_7) = _8 as f64;
_13.3 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_19.3, 0), 0),fld1: _8,fld2: Field::<u64>(Variant(_19.3, 0), 2) };
_34 = Field::<bool>(Variant(_13.3, 0), 0);
Goto(bb18)
}
bb18 = {
_10.1 = Adt17::Variant0 { fld0: (*_25),fld1: _21,fld2: _18,fld3: _35,fld4: (-9090_i16),fld5: 139_u8,fld6: 255780548249191029283191413195422228306_u128 };
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) & Field::<u32>(Variant(_10.1, 0), 0);
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) ^ Field::<u32>(Variant(_10.1, 0), 0);
_31 = 112865495644287459082865941376683494624_u128 as u32;
_5 = Adt19::Variant0 { fld0: _34,fld1: Field::<i128>(Variant(_13.3, 0), 1),fld2: Field::<u64>(Variant(_19.3, 0), 2) };
_13.2 = _19.2;
_13.1 = _17 ^ _17;
_23 = _13.1;
(*_7) = _22 as f64;
_10.1 = Adt17::Variant0 { fld0: (*_25),fld1: _10.2.fld0,fld2: _18,fld3: _35,fld4: 2549_i16,fld5: 14_u8,fld6: 272162450596397423636642639325357811469_u128 };
_4 = core::ptr::addr_of_mut!((*_7));
(*_4) = _13.1 as f64;
(*_25) = !Field::<u32>(Variant(_10.1, 0), 0);
(*_4) = _22 as f64;
(*_25) = 269540098427332106515101048994681292282_u128 as u32;
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) | Field::<u32>(Variant(_10.1, 0), 0);
(*_4) = _12 as f64;
_40 = &_19.3;
Goto(bb19)
}
bb19 = {
_7 = core::ptr::addr_of_mut!((*_4));
_16 = Move(_13.3);
_44.1 = _12 - _12;
_34 = Field::<i128>(Variant((*_40), 0), 1) <= Field::<i128>(Variant((*_40), 0), 1);
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) + Field::<u32>(Variant(_10.1, 0), 0);
_13.1 = _23 + _18;
_37 = _10.2.fld0 as i32;
_13.3 = Move(_5);
_42 = (-3078178134055783484_i64) ^ 7280871149092283184_i64;
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) + Field::<u32>(Variant(_10.1, 0), 0);
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) | Field::<u32>(Variant(_10.1, 0), 0);
_37 = (-27788_i16) as i32;
place!(Field::<u64>(Variant(_13.3, 0), 2)) = 114_i8 as u64;
(*_7) = 3615_u16 as f64;
_44.2.fld0 = !_23;
place!(Field::<u8>(Variant(_10.1, 0), 5)) = 115_u8 * 122_u8;
Goto(bb20)
}
bb20 = {
(*_25) = Field::<u32>(Variant(_10.1, 0), 0);
_38 = (-9_i8) << _37;
_30 = &_35;
(*_7) = (*_30) as f64;
_44.3 = (*_30) >> Field::<i128>(Variant((*_40), 0), 1);
place!(Field::<u128>(Variant(_10.1, 0), 6)) = (*_4) as u128;
(*_4) = (-2296_i16) as f64;
_44.2.fld1 = core::ptr::addr_of_mut!(_26);
_11 = &_22;
_37 = _26 as i32;
_26 = _19.1 as f64;
_5 = Adt19::Variant2 { fld0: (*_11),fld1: (*_30),fld2: Field::<u8>(Variant(_10.1, 0), 5),fld3: _38,fld4: Field::<i128>(Variant((*_40), 0), 1) };
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) | Field::<u32>(Variant(_10.1, 0), 0);
_35 = _34 as usize;
place!(Field::<char>(Variant(_10.1, 0), 1)) = _10.2.fld0;
(*_4) = _35 as f64;
Goto(bb21)
}
bb21 = {
place!(Field::<bool>(Variant(_13.3, 0), 0)) = !Field::<bool>(Variant((*_40), 0), 0);
_44.0 = &mut _21;
place!(Field::<bool>(Variant(_13.3, 0), 0)) = (*_11) <= (*_11);
_45 = Field::<char>(Variant(_10.1, 0), 1);
_19.1 = -_44.2.fld0;
_10.2 = Adt34 { fld0: Field::<char>(Variant(_10.1, 0), 1) };
place!(Field::<u64>(Variant(_13.3, 0), 2)) = (*_4) as u64;
place!(Field::<i128>(Variant(_19.3, 0), 1)) = !Field::<i128>(Variant(_13.3, 0), 1);
(*_4) = 3831_i16 as f64;
(*_4) = Field::<i128>(Variant(_5, 2), 4) as f64;
place!(Field::<bool>(Variant(_16, 0), 0)) = _20 <= _18;
_52 = &_12;
place!(Field::<i32>(Variant(_5, 2), 0)) = (*_11) >> _13.1;
place!(Field::<u8>(Variant(_5, 2), 2)) = Field::<u8>(Variant(_10.1, 0), 5) & Field::<u8>(Variant(_10.1, 0), 5);
_57 = _45;
Goto(bb22)
}
bb22 = {
(*_25) = Field::<u32>(Variant(_10.1, 0), 0);
place!(Field::<bool>(Variant(_16, 0), 0)) = Field::<bool>(Variant((*_40), 0), 0) & Field::<bool>(Variant((*_40), 0), 0);
(*_25) = !Field::<u32>(Variant(_10.1, 0), 0);
_57 = _10.2.fld0;
(*_25) = !Field::<u32>(Variant(_10.1, 0), 0);
(*_4) = Field::<i128>(Variant(_5, 2), 4) as f64;
_54 = _10.2.fld0;
_44.1 = (*_52) + (*_52);
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) << (*_11);
(*_4) = _42 as f64;
(*_4) = (*_25) as f64;
_54 = _10.2.fld0;
_10.0.0 = &_57;
_60.2 = Adt34 { fld0: Field::<char>(Variant(_10.1, 0), 1) };
_10.2.fld0 = _57;
_20 = _18;
_49 = [Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
(*_4) = _42 as f64;
_34 = (*_25) == (*_25);
_42 = Field::<u128>(Variant(_10.1, 0), 6) as i64;
_10.3 = &mut place!(Field::<u128>(Variant(_10.1, 0), 6));
_46 = Adt34 { fld0: _57 };
Goto(bb23)
}
bb23 = {
(*_25) = 3506834882_u32;
(*_4) = (*_52) as f64;
(*_4) = Field::<i8>(Variant(_5, 2), 3) as f64;
_8 = -Field::<i128>(Variant(_5, 2), 4);
_31 = !4137468278_u32;
_36 = _8 as f32;
_18 = _19.1;
_60.2.fld0 = _54;
place!(Field::<i128>(Variant(_13.3, 0), 1)) = _8;
(*_25) = 2489030882_u32 << Field::<i32>(Variant(_5, 2), 0);
place!(Field::<usize>(Variant(_5, 2), 1)) = _35 * _35;
_44.2.fld1 = Move(_4);
_2.fld0 = -_44.2.fld0;
_20 = _17 << Field::<i128>(Variant(_19.3, 0), 1);
_38 = _8 as i8;
(*_25) = !297967712_u32;
(*_25) = _42 as u32;
place!(Field::<bool>(Variant(_16, 0), 0)) = Field::<bool>(Variant(_13.3, 0), 0);
Goto(bb24)
}
bb24 = {
(*_25) = _42 as u32;
(*_25) = 2030180312_u32 << (*_11);
_31 = 595749431_u32;
_63.2.0 = core::ptr::addr_of_mut!(_26);
(*_25) = 1927189765_u32 >> (*_11);
Goto(bb25)
}
bb25 = {
_60.1 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_16, 0), 1),fld1: Field::<u64>(Variant(_19.3, 0), 2),fld2: (*_52) };
_16 = Move(_5);
_1 = Adt60::Variant2 { fld0: _34,fld1: Field::<u64>(Variant((*_40), 0), 2) };
_38 = 24008_u16 as i8;
_19.2 = _13.2;
_65 = _35 as f32;
_14 = &(*_52);
_48 = Field::<u8>(Variant(_16, 2), 2) - Field::<u8>(Variant(_16, 2), 2);
_57 = _45;
_56 = &mut _54;
_47 = _26 * _26;
(*_56) = _45;
place!(Field::<f32>(Variant(_60.1, 1), 2)) = (*_52) - (*_52);
_18 = -_44.2.fld0;
(*_56) = _45;
_3 = (*_11) * (*_11);
_63.3 = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2)];
place!(Field::<bool>(Variant(_19.3, 0), 0)) = !_34;
_63.3 = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
_50 = &(*_11);
_9 = &mut _49;
_23 = _13.1 - _13.1;
_63.0 = &mut place!(Field::<usize>(Variant(_16, 2), 1));
_5 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_13.3, 0), 0),fld1: Field::<i128>(Variant(_13.3, 0), 1),fld2: Field::<u64>(Variant((*_40), 0), 2) };
Goto(bb26)
}
bb26 = {
_33 = 152107297227617139527316804791100451095_u128 * 148307879998178918763325211938457737585_u128;
_20 = _19.1;
(*_25) = !2126804402_u32;
_60.2 = Move(_46);
_63.3 = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_5, 0), 2)];
(*_25) = !2666002762_u32;
(*_9) = [Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_60.1, 1), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant((*_40), 0), 2)];
(*_9) = _63.3;
(*_56) = _60.2.fld0;
_24 = core::ptr::addr_of!(_69);
(*_9) = _63.3;
_44.2.fld1 = core::ptr::addr_of_mut!(_47);
(*_56) = _60.2.fld0;
_55 = core::ptr::addr_of!((*_24));
_2.fld0 = !_18;
(*_55) = Adt50 { fld0: _33 };
_50 = &_3;
Goto(bb27)
}
bb27 = {
_56 = &mut _60.2.fld0;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2)];
_44.2 = Adt40 { fld0: _18,fld1: Move(_2.fld1) };
(*_55) = Adt50 { fld0: _33 };
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
_46.fld0 = (*_56);
(*_24) = Adt50 { fld0: _33 };
(*_56) = _46.fld0;
(*_24) = Adt50 { fld0: _33 };
(*_56) = _46.fld0;
_68 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_19.3, 0), 0)));
(*_9) = [Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
(*_24) = Adt50 { fld0: _33 };
_72.2.1 = &(*_56);
_32 = core::ptr::addr_of!(_59);
(*_24).fld0 = _33;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_5, 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_13.3, 0), 2)];
_73 = (*_52) as u64;
Goto(bb28)
}
bb28 = {
(*_24) = Adt50 { fld0: _33 };
(*_68) = Field::<bool>(Variant(_5, 0), 0);
(*_32) = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: _23,fld3: _35,fld4: 13121_i16,fld5: _48,fld6: _33 };
_62.0.0 = Move(_72.2.1);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 >> Field::<isize>(Variant((*_32), 0), 2);
Call(place!(Field::<u128>(Variant((*_32), 0), 6)) = core::intrinsics::bswap((*_24).fld0), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
place!(Field::<isize>(Variant((*_32), 0), 2)) = _17 + _23;
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
place!(Field::<bool>(Variant(_19.3, 0), 0)) = Field::<usize>(Variant((*_32), 0), 3) >= Field::<usize>(Variant((*_32), 0), 3);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -15008_i16;
(*_24).fld0 = Field::<u128>(Variant(_59, 0), 6) | Field::<u128>(Variant((*_32), 0), 6);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0;
(*_24).fld0 = Field::<u128>(Variant((*_32), 0), 6) & Field::<u128>(Variant((*_32), 0), 6);
_67 = -Field::<i16>(Variant((*_32), 0), 4);
Goto(bb30)
}
bb30 = {
place!(Field::<isize>(Variant(_59, 0), 2)) = _18 - _23;
(*_32) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_5, 0), 1),fld1: Field::<u64>(Variant(_13.3, 0), 2),fld2: (*_52) };
Goto(bb31)
}
bb31 = {
place!(Field::<i128>(Variant((*_32), 1), 0)) = !_8;
(*_24) = Adt50 { fld0: _33 };
place!(Field::<i128>(Variant((*_32), 1), 0)) = !Field::<i128>(Variant(_19.3, 0), 1);
place!(Field::<f32>(Variant((*_32), 1), 2)) = (*_52) * (*_52);
place!(Field::<f32>(Variant((*_32), 1), 2)) = _36 * (*_52);
(*_24) = Adt50 { fld0: _33 };
_71 = Move(_46);
(*_32) = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: _23,fld3: _44.3,fld4: _67,fld5: _48,fld6: (*_24).fld0 };
(*_32) = Adt17::Variant2 { fld0: _26,fld1: 59709_u16,fld2: _48 };
(*_68) = Field::<bool>(Variant(_1, 2), 0) ^ Field::<bool>(Variant(_1, 2), 0);
(*_24) = Adt50 { fld0: _33 };
(*_24) = Adt50 { fld0: _33 };
(*_56) = _45;
_80.fld1 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_32), 2), 0)));
place!(Field::<f64>(Variant((*_32), 2), 0)) = Field::<u64>(Variant((*_40), 0), 2) as f64;
(*_24) = Adt50 { fld0: _33 };
(*_24) = Adt50 { fld0: _33 };
place!(Field::<f64>(Variant((*_32), 2), 0)) = _47 * _47;
(*_32) = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: _17,fld3: _35,fld4: _67,fld5: _48,fld6: (*_24).fld0 };
_80 = Adt40 { fld0: _23,fld1: Move(_44.2.fld1) };
place!(Field::<u64>(Variant(_19.3, 0), 2)) = Field::<u8>(Variant((*_32), 0), 5) as u64;
Goto(bb32)
}
bb32 = {
place!(Field::<u128>(Variant((*_32), 0), 6)) = _69.fld0 >> (*_25);
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67;
place!(Field::<u128>(Variant((*_32), 0), 6)) = Field::<u64>(Variant(_19.3, 0), 2) as u128;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _19.1 & _13.1;
place!(Field::<u8>(Variant((*_32), 0), 5)) = !_48;
_76 = &mut _63.0;
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
_18 = (*_50) as isize;
(*_68) = Field::<isize>(Variant((*_32), 0), 2) != Field::<isize>(Variant((*_32), 0), 2);
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67;
(*_25) = (*_52) as u32;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<i128>(Variant(_13.3, 0), 1) as u8;
(*_25) = Field::<u32>(Variant(_59, 0), 0);
_61 = &mut place!(Field::<usize>(Variant((*_32), 0), 3));
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 >> Field::<isize>(Variant((*_32), 0), 2);
(*_9) = [Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_13.3, 0), 2)];
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 & (*_24).fld0;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _26 as i16;
(*_68) = (*_61) <= (*_61);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 >> (*_24).fld0;
Goto(bb33)
}
bb33 = {
(*_61) = Field::<isize>(Variant((*_32), 0), 2) as usize;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _80.fld0;
_2 = Adt40 { fld0: Field::<isize>(Variant((*_32), 0), 2),fld1: Move(_80.fld1) };
_88.1 = Adt17::Variant2 { fld0: _47,fld1: 61207_u16,fld2: Field::<u8>(Variant((*_32), 0), 5) };
Goto(bb34)
}
bb34 = {
_45 = (*_56);
_94 = Field::<isize>(Variant((*_32), 0), 2) as f64;
(*_24).fld0 = Field::<u128>(Variant((*_32), 0), 6) * Field::<u128>(Variant((*_32), 0), 6);
Goto(bb35)
}
bb35 = {
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 + (*_24).fld0;
(*_61) = _44.3 - _44.3;
_35 = (*_61) + (*_61);
place!(Field::<u8>(Variant(_88.1, 2), 2)) = _47 as u8;
_39 = core::ptr::addr_of!((*_24));
place!(Field::<isize>(Variant((*_32), 0), 2)) = _13.1;
(*_68) = (*_24).fld0 > (*_24).fld0;
_13.3 = Move(_19.3);
(*_76) = &mut (*_61);
_5 = Move(_13.3);
_40 = &_5;
_70.1 = Adt17::Variant1 { fld0: _8,fld1: Field::<u64>(Variant((*_40), 0), 2),fld2: _65 };
(*_68) = Field::<bool>(Variant((*_40), 0), 0);
place!(Field::<u8>(Variant((*_32), 0), 5)) = !_48;
place!(Field::<u32>(Variant((*_32), 0), 0)) = Field::<u8>(Variant((*_32), 0), 5) as u32;
_84 = Move(_1);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
(*_39).fld0 = !Field::<u128>(Variant((*_32), 0), 6);
_64 = &_44.1;
_55 = core::ptr::addr_of!((*_39));
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<bool>(Variant((*_40), 0), 0) as isize;
Call(_8 = core::intrinsics::bswap(Field::<i128>(Variant(_5, 0), 1)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
(*_39).fld0 = Field::<u128>(Variant((*_32), 0), 6) >> Field::<u128>(Variant((*_32), 0), 6);
place!(Field::<u16>(Variant(_88.1, 2), 1)) = 4707_u16 + 39437_u16;
(*_56) = _45;
_72.3 = (*_9);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
(*_39).fld0 = Field::<u128>(Variant((*_32), 0), 6);
(*_68) = Field::<bool>(Variant((*_40), 0), 0) > Field::<bool>(Variant((*_40), 0), 0);
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
place!(Field::<u128>(Variant((*_32), 0), 6)) = Field::<u16>(Variant(_88.1, 2), 1) as u128;
Goto(bb37)
}
bb37 = {
_91 = core::ptr::addr_of!(_40);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _19.1 << (*_24).fld0;
place!(Field::<char>(Variant((*_32), 0), 1)) = _57;
(*_9) = _72.3;
(*_25) = Field::<u32>(Variant((*_32), 0), 0) * Field::<u32>(Variant((*_32), 0), 0);
_34 = (*_68);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 << (*_24).fld0;
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25);
place!(Field::<i16>(Variant((*_32), 0), 4)) = Field::<isize>(Variant((*_32), 0), 2) as i16;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
place!(Field::<i128>(Variant(_5, 0), 1)) = _8;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67 & _67;
(*_24).fld0 = !Field::<u128>(Variant((*_32), 0), 6);
(*_68) = Field::<bool>(Variant((*_40), 0), 0) <= Field::<bool>(Variant((*_40), 0), 0);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
_81 = (*_24).fld0 | (*_24).fld0;
_44.3 = _35;
Goto(bb38)
}
bb38 = {
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) - (*_25);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -_67;
place!(Field::<u32>(Variant((*_32), 0), 0)) = !(*_25);
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
_34 = Field::<bool>(Variant((*_40), 0), 0) < Field::<bool>(Variant((*_40), 0), 0);
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_5, 0), 2),Field::<u64>(Variant(_5, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
_19.3 = Adt19::Variant0 { fld0: Field::<bool>(Variant((*_40), 0), 0),fld1: Field::<i128>(Variant(_70.1, 1), 0),fld2: Field::<u64>(Variant((*_40), 0), 2) };
_19.3 = Move(_5);
(*_24).fld0 = Field::<u128>(Variant((*_32), 0), 6) + _81;
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_64) as u32;
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
(*_76) = &mut _35;
_98 = core::ptr::addr_of_mut!(_47);
(*_9) = [_73,Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_70.1, 1), 1)];
_61 = &mut _44.3;
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 ^ (*_24).fld0;
place!(Field::<u32>(Variant((*_32), 0), 0)) = _31 >> Field::<isize>(Variant((*_32), 0), 2);
(*_9) = [Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_70.1, 1), 1)];
Goto(bb39)
}
bb39 = {
place!(Field::<u8>(Variant((*_32), 0), 5)) = _48 >> (*_24).fld0;
(*_98) = _26 - _94;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67 >> (*_25);
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
(*_25) = Field::<char>(Variant((*_32), 0), 1) as u32;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67;
(*_76) = &mut (*_61);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
(*_98) = _94 * _94;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67 >> Field::<isize>(Variant((*_32), 0), 2);
place!(Field::<u8>(Variant((*_32), 0), 5)) = !_48;
place!(Field::<u8>(Variant((*_32), 0), 5)) = _48 & _48;
_73 = (*_14) as u64;
_72.2.1 = &(*_56);
_37 = (*_50) >> Field::<i16>(Variant((*_32), 0), 4);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _34 as isize;
_87 = _36 - (*_52);
Goto(bb40)
}
bb40 = {
_67 = Field::<i16>(Variant((*_32), 0), 4);
_72.1.1 = Adt17::Variant0 { fld0: Field::<u32>(Variant((*_32), 0), 0),fld1: (*_56),fld2: Field::<isize>(Variant((*_32), 0), 2),fld3: 7685390749023233899_usize,fld4: Field::<i16>(Variant((*_32), 0), 4),fld5: Field::<u8>(Variant((*_32), 0), 5),fld6: (*_24).fld0 };
(*_91) = &_19.3;
_75 = &(*_50);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_14) as u128;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _80.fld0 << (*_24).fld0;
(*_24).fld0 = _81;
_88.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
(*_25) = !Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<u8>(Variant(_72.1.1, 0), 5)) = Field::<u8>(Variant((*_32), 0), 5) & Field::<u8>(Variant((*_32), 0), 5);
place!(Field::<i128>(Variant(_70.1, 1), 0)) = Field::<u16>(Variant(_88.1, 2), 1) as i128;
_65 = Field::<f32>(Variant(_70.1, 1), 2) - (*_64);
_72.2 = (Move(_98), Move(_88.0.0));
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) << Field::<isize>(Variant((*_32), 0), 2);
_82 = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -_67;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2)];
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<isize>(Variant(_72.1.1, 0), 2) + Field::<isize>(Variant(_72.1.1, 0), 2);
(*_9) = _72.3;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_88.1, 2), 2) - Field::<u8>(Variant(_72.1.1, 0), 5);
(*_76) = Move(_61);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _23;
_14 = &_65;
Goto(bb41)
}
bb41 = {
(*_25) = !Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _94 as isize;
Goto(bb42)
}
bb42 = {
_99.1 = Field::<i16>(Variant((*_32), 0), 4) as isize;
place!(Field::<u32>(Variant((*_32), 0), 0)) = _45 as u32;
_88.2.fld0 = (*_56);
_70.2.fld0 = Field::<char>(Variant(_72.1.1, 0), 1);
place!(Field::<u128>(Variant(_72.1.1, 0), 6)) = !(*_24).fld0;
place!(Field::<f32>(Variant(_70.1, 1), 2)) = (*_14) - (*_14);
_90 = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<i128>(Variant(_19.3, 0), 1)) = _8 & _8;
_106 = [_45];
place!(Field::<u128>(Variant((*_32), 0), 6)) = Field::<u8>(Variant((*_32), 0), 5) as u128;
_107 = Field::<i16>(Variant((*_32), 0), 4) + Field::<i16>(Variant((*_32), 0), 4);
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) | (*_25);
_5 = Move(_19.3);
_100 = (*_14) + (*_14);
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) | (*_25);
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_72.1.1, 0), 5) & _48;
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0 ^ (*_24).fld0;
_72.1.1 = _70.1;
(*_56) = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<u8>(Variant(_88.1, 2), 2)) = Field::<i128>(Variant(_70.1, 1), 0) as u8;
_96 = &mut (*_24).fld0;
_19.2 = [Field::<u16>(Variant(_88.1, 2), 1)];
_95.2 = Adt40 { fld0: _20,fld1: Move(_72.2.0) };
_84 = Adt60::Variant2 { fld0: Field::<bool>(Variant(_5, 0), 0),fld1: Field::<u64>(Variant(_70.1, 1), 1) };
Call((*_9) = core::intrinsics::transmute(_72.3), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
_108.0 = _42 as f64;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _2.fld0;
_19.3 = Move(_5);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
Goto(bb44)
}
bb44 = {
_53 = &_96;
_88.1 = Adt17::Variant2 { fld0: _94,fld1: 17389_u16,fld2: Field::<u8>(Variant((*_32), 0), 5) };
_72.3 = (*_9);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _19.1 & _2.fld0;
_47 = _94;
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_96) | _81;
place!(Field::<u64>(Variant(_72.1.1, 1), 1)) = Field::<u64>(Variant(_84, 2), 1) & Field::<u64>(Variant(_84, 2), 1);
_18 = -Field::<isize>(Variant((*_32), 0), 2);
_78 = (*_96) & (*_96);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_96) & (*_96);
_2 = Adt40 { fld0: Field::<isize>(Variant((*_32), 0), 2),fld1: Move(_95.2.fld1) };
(*_25) = (*_14) as u32;
(*_9) = [Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_72.1.1, 1), 1),Field::<u64>(Variant(_84, 2), 1),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_84, 2), 1),Field::<u64>(Variant(_70.1, 1), 1)];
_99.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_88.1, 2), 1)));
_86 = Move(_96);
place!(Field::<u8>(Variant((*_32), 0), 5)) = _8 as u8;
_7 = core::ptr::addr_of_mut!(_108.0);
place!(Field::<i16>(Variant((*_32), 0), 4)) = _107 >> Field::<u32>(Variant((*_32), 0), 0);
(*_7) = Field::<f64>(Variant(_88.1, 2), 0) - _47;
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
_19.2 = [43663_u16];
_100 = -_36;
place!(Field::<u32>(Variant((*_32), 0), 0)) = Field::<u64>(Variant(_19.3, 0), 2) as u32;
Call(_109 = core::intrinsics::bswap(Field::<isize>(Variant((*_32), 0), 2)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<u64>(Variant(_70.1, 1), 1) as isize;
_9 = &mut _72.3;
_95.2.fld0 = Field::<isize>(Variant((*_32), 0), 2) << _107;
(*_56) = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<isize>(Variant((*_32), 0), 2)) = !_23;
_121.2 = _70.1;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67 ^ _107;
_77 = Move((*_91));
_46.fld0 = _57;
_95.1 = (*_14);
place!(Field::<u128>(Variant((*_32), 0), 6)) = !_78;
(*_9) = [Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_121.2, 1), 1),_73,Field::<u64>(Variant(_84, 2), 1),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_121.2, 1), 1)];
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_88.1, 2), 2) ^ Field::<u8>(Variant(_88.1, 2), 2);
Goto(bb46)
}
bb46 = {
_70.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
(*_9) = [Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_70.1, 1), 1)];
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78 | _78;
_70.1 = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: Field::<isize>(Variant((*_32), 0), 2),fld3: 12365128024566003236_usize,fld4: Field::<i16>(Variant((*_32), 0), 4),fld5: Field::<u8>(Variant((*_32), 0), 5),fld6: Field::<u128>(Variant((*_32), 0), 6) };
(*_7) = Field::<i128>(Variant(_19.3, 0), 1) as f64;
_114 = &mut _45;
_88.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
_42 = 3274122334116528791_i64 << Field::<u8>(Variant((*_32), 0), 5);
_78 = !Field::<u128>(Variant((*_32), 0), 6);
_88.1 = _121.2;
Goto(bb47)
}
bb47 = {
_2.fld0 = Field::<isize>(Variant((*_32), 0), 2) ^ Field::<isize>(Variant((*_32), 0), 2);
(*_91) = &_19.3;
Goto(bb48)
}
bb48 = {
place!(Field::<i16>(Variant((*_32), 0), 4)) = -Field::<i16>(Variant(_70.1, 0), 4);
place!(Field::<isize>(Variant((*_32), 0), 2)) = (*_50) as isize;
place!(Field::<u128>(Variant((*_32), 0), 6)) = 7892271264961887847_usize as u128;
place!(Field::<u128>(Variant((*_32), 0), 6)) = !_78;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_88.1, 1), 1)];
(*_56) = _57;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_70.1, 0), 5) | Field::<u8>(Variant(_70.1, 0), 5);
Goto(bb49)
}
bb49 = {
_31 = Field::<u32>(Variant((*_32), 0), 0) ^ Field::<u32>(Variant((*_32), 0), 0);
_80.fld1 = core::ptr::addr_of_mut!((*_7));
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78;
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<isize>(Variant(_70.1, 0), 2) | _13.1;
_13.0 = Move(_99.0);
_129 = (18118682628905682682_usize, Field::<bool>(Variant((*_40), 0), 0));
_25 = core::ptr::addr_of_mut!((*_25));
_41 = core::ptr::addr_of_mut!(_110);
_12 = (*_14) * (*_64);
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78 >> Field::<i16>(Variant((*_32), 0), 4);
_104 = _37;
_95.0 = &mut (*_56);
place!(Field::<u128>(Variant((*_32), 0), 6)) = Field::<u128>(Variant(_70.1, 0), 6) * _78;
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) ^ (*_25);
match _129.0 {
0 => bb13,
1 => bb34,
2 => bb50,
3 => bb51,
4 => bb52,
18118682628905682682 => bb54,
_ => bb53
}
}
bb50 = {
Return()
}
bb51 = {
Return()
}
bb52 = {
_70.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
(*_9) = [Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_70.1, 1), 1)];
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78 | _78;
_70.1 = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: Field::<isize>(Variant((*_32), 0), 2),fld3: 12365128024566003236_usize,fld4: Field::<i16>(Variant((*_32), 0), 4),fld5: Field::<u8>(Variant((*_32), 0), 5),fld6: Field::<u128>(Variant((*_32), 0), 6) };
(*_7) = Field::<i128>(Variant(_19.3, 0), 1) as f64;
_114 = &mut _45;
_88.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
_42 = 3274122334116528791_i64 << Field::<u8>(Variant((*_32), 0), 5);
_78 = !Field::<u128>(Variant((*_32), 0), 6);
_88.1 = _121.2;
Goto(bb47)
}
bb53 = {
(*_25) = Field::<u32>(Variant(_10.1, 0), 0);
_38 = (-9_i8) << _37;
_30 = &_35;
(*_7) = (*_30) as f64;
_44.3 = (*_30) >> Field::<i128>(Variant((*_40), 0), 1);
place!(Field::<u128>(Variant(_10.1, 0), 6)) = (*_4) as u128;
(*_4) = (-2296_i16) as f64;
_44.2.fld1 = core::ptr::addr_of_mut!(_26);
_11 = &_22;
_37 = _26 as i32;
_26 = _19.1 as f64;
_5 = Adt19::Variant2 { fld0: (*_11),fld1: (*_30),fld2: Field::<u8>(Variant(_10.1, 0), 5),fld3: _38,fld4: Field::<i128>(Variant((*_40), 0), 1) };
(*_25) = Field::<u32>(Variant(_10.1, 0), 0) | Field::<u32>(Variant(_10.1, 0), 0);
_35 = _34 as usize;
place!(Field::<char>(Variant(_10.1, 0), 1)) = _10.2.fld0;
(*_4) = _35 as f64;
Goto(bb21)
}
bb54 = {
(*_7) = _47 + _47;
place!(Field::<u32>(Variant((*_32), 0), 0)) = Field::<u8>(Variant((*_32), 0), 5) as u32;
(*_41) = [(*_114),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1)];
place!(Field::<u128>(Variant((*_32), 0), 6)) = !Field::<u128>(Variant(_70.1, 0), 6);
(*_41) = [(*_114),(*_114),Field::<char>(Variant((*_32), 0), 1),(*_114),Field::<char>(Variant(_70.1, 0), 1),(*_114)];
place!(Field::<u8>(Variant((*_32), 0), 5)) = _48 * Field::<u8>(Variant(_70.1, 0), 5);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_14) as u128;
place!(Field::<u128>(Variant((*_32), 0), 6)) = 31972_u16 as u128;
_93.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
_34 = !Field::<bool>(Variant((*_40), 0), 0);
_119 = _121.2;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _99.1;
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<isize>(Variant(_70.1, 0), 2) >> Field::<u8>(Variant((*_32), 0), 5);
place!(Field::<char>(Variant((*_32), 0), 1)) = _57;
(*_25) = _42 as u32;
place!(Field::<i16>(Variant((*_32), 0), 4)) = _107 | _107;
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
match _129.0 {
18118682628905682682 => bb55,
_ => bb11
}
}
bb55 = {
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
_130 = core::ptr::addr_of!(place!(Field::<isize>(Variant((*_32), 0), 2)));
_136 = core::ptr::addr_of!(_99.0);
(*_25) = Field::<u32>(Variant((*_32), 0), 0);
_5 = Move(_19.3);
_121.3 = _8 as f32;
place!(Field::<usize>(Variant(_70.1, 0), 3)) = _129.0;
Goto(bb56)
}
bb56 = {
(*_91) = &_5;
_77 = &(*_40);
place!(Field::<isize>(Variant((*_32), 0), 2)) = -_20;
(*_41) = [_70.2.fld0,Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),_82,Field::<char>(Variant((*_32), 0), 1)];
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
(*_136) = Move(_13.0);
(*_41) = [(*_114),(*_114),_70.2.fld0,(*_114),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1)];
(*_41) = [Field::<char>(Variant(_70.1, 0), 1),_57,Field::<char>(Variant((*_32), 0), 1),(*_114),_71.fld0,(*_114)];
place!(Field::<isize>(Variant((*_32), 0), 2)) = Field::<usize>(Variant(_70.1, 0), 3) as isize;
_111 = (*_14);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
_22 = (*_75) | (*_50);
match _129.0 {
0 => bb14,
1 => bb34,
2 => bb7,
3 => bb53,
4 => bb38,
5 => bb21,
6 => bb57,
18118682628905682682 => bb59,
_ => bb58
}
}
bb57 = {
_70.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
(*_9) = [Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_70.1, 1), 1),Field::<u64>(Variant(_19.3, 0), 2),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_121.2, 1), 1),Field::<u64>(Variant(_70.1, 1), 1)];
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78 | _78;
_70.1 = Adt17::Variant0 { fld0: (*_25),fld1: (*_56),fld2: Field::<isize>(Variant((*_32), 0), 2),fld3: 12365128024566003236_usize,fld4: Field::<i16>(Variant((*_32), 0), 4),fld5: Field::<u8>(Variant((*_32), 0), 5),fld6: Field::<u128>(Variant((*_32), 0), 6) };
(*_7) = Field::<i128>(Variant(_19.3, 0), 1) as f64;
_114 = &mut _45;
_88.0.0 = &place!(Field::<char>(Variant((*_32), 0), 1));
_42 = 3274122334116528791_i64 << Field::<u8>(Variant((*_32), 0), 5);
_78 = !Field::<u128>(Variant((*_32), 0), 6);
_88.1 = _121.2;
Goto(bb47)
}
bb58 = {
(*_25) = _42 as u32;
(*_25) = 2030180312_u32 << (*_11);
_31 = 595749431_u32;
_63.2.0 = core::ptr::addr_of_mut!(_26);
(*_25) = 1927189765_u32 >> (*_11);
Goto(bb25)
}
bb59 = {
(*_41) = [Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),_46.fld0,(*_114),Field::<char>(Variant((*_32), 0), 1),_90];
_130 = core::ptr::addr_of!(_20);
_115 = (*_7);
_126 = core::ptr::addr_of!((*_7));
(*_41) = [(*_114),_57,(*_114),Field::<char>(Variant((*_32), 0), 1),(*_114),Field::<char>(Variant((*_32), 0), 1)];
place!(Field::<char>(Variant((*_32), 0), 1)) = _70.2.fld0;
_110 = [(*_114),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),_57,Field::<char>(Variant((*_32), 0), 1)];
(*_41) = [Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),(*_114)];
_70.2 = Move(_88.2);
_117 = _111;
(*_7) = _115 * _94;
_97 = Adt34 { fld0: (*_114) };
_110 = [_82,(*_114),(*_114),_90,_90,(*_114)];
place!(Field::<isize>(Variant((*_32), 0), 2)) = _2.fld0 >> Field::<u8>(Variant((*_32), 0), 5);
place!(Field::<i16>(Variant((*_32), 0), 4)) = _67 * Field::<i16>(Variant(_70.1, 0), 4);
(*_114) = _90;
match Field::<usize>(Variant(_70.1, 0), 3) {
0 => bb22,
1 => bb9,
2 => bb44,
3 => bb60,
4 => bb61,
5 => bb62,
6 => bb63,
18118682628905682682 => bb65,
_ => bb64
}
}
bb60 = {
_2.fld0 = Field::<isize>(Variant((*_32), 0), 2) ^ Field::<isize>(Variant((*_32), 0), 2);
(*_91) = &_19.3;
Goto(bb48)
}
bb61 = {
_33 = 152107297227617139527316804791100451095_u128 * 148307879998178918763325211938457737585_u128;
_20 = _19.1;
(*_25) = !2126804402_u32;
_60.2 = Move(_46);
_63.3 = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_5, 0), 2)];
(*_25) = !2666002762_u32;
(*_9) = [Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_13.3, 0), 2),Field::<u64>(Variant(_60.1, 1), 1),Field::<u64>(Variant(_1, 2), 1),Field::<u64>(Variant((*_40), 0), 2)];
(*_9) = _63.3;
(*_56) = _60.2.fld0;
_24 = core::ptr::addr_of!(_69);
(*_9) = _63.3;
_44.2.fld1 = core::ptr::addr_of_mut!(_47);
(*_56) = _60.2.fld0;
_55 = core::ptr::addr_of!((*_24));
_2.fld0 = !_18;
(*_55) = Adt50 { fld0: _33 };
_50 = &_3;
Goto(bb27)
}
bb62 = {
Return()
}
bb63 = {
place!(Field::<i16>(Variant((*_32), 0), 4)) = -Field::<i16>(Variant(_70.1, 0), 4);
place!(Field::<isize>(Variant((*_32), 0), 2)) = (*_50) as isize;
place!(Field::<u128>(Variant((*_32), 0), 6)) = 7892271264961887847_usize as u128;
place!(Field::<u128>(Variant((*_32), 0), 6)) = !_78;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_88.1, 1), 1)];
(*_56) = _57;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_70.1, 0), 5) | Field::<u8>(Variant(_70.1, 0), 5);
Goto(bb49)
}
bb64 = {
Return()
}
bb65 = {
_120 = core::ptr::addr_of!(_121.2);
place!(Field::<isize>(Variant((*_32), 0), 2)) = -_20;
_55 = core::ptr::addr_of!(_134.0);
place!(Field::<char>(Variant((*_32), 0), 1)) = _90;
_102 = _42 as isize;
(*_114) = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<u64>(Variant((*_120), 1), 1)) = Field::<u64>(Variant((*_77), 0), 2) << _129.0;
(*_55) = Adt50 { fld0: _81 };
place!(Field::<isize>(Variant((*_32), 0), 2)) = _80.fld0 + (*_130);
(*_114) = Field::<char>(Variant((*_32), 0), 1);
_98 = core::ptr::addr_of_mut!(_138);
_122 = [51552_u16];
(*_41) = [_71.fld0,(*_114),Field::<char>(Variant((*_32), 0), 1),(*_114),(*_114),(*_114)];
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<i16>(Variant((*_32), 0), 4) as u8;
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant((*_40), 0), 1) ^ Field::<i128>(Variant((*_40), 0), 1);
_88.2.fld0 = (*_114);
_95.2 = Move(_2);
_19.0 = Move((*_136));
place!(Field::<i128>(Variant(_5, 0), 1)) = Field::<i128>(Variant((*_120), 1), 0);
match _129.0 {
0 => bb18,
1 => bb36,
2 => bb38,
3 => bb66,
4 => bb67,
18118682628905682682 => bb69,
_ => bb68
}
}
bb66 = {
Return()
}
bb67 = {
place!(Field::<isize>(Variant((*_32), 0), 2)) = _17 + _23;
(*_24) = Adt50 { fld0: Field::<u128>(Variant((*_32), 0), 6) };
place!(Field::<bool>(Variant(_19.3, 0), 0)) = Field::<usize>(Variant((*_32), 0), 3) >= Field::<usize>(Variant((*_32), 0), 3);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -15008_i16;
(*_24).fld0 = Field::<u128>(Variant(_59, 0), 6) | Field::<u128>(Variant((*_32), 0), 6);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_24).fld0;
(*_24).fld0 = Field::<u128>(Variant((*_32), 0), 6) & Field::<u128>(Variant((*_32), 0), 6);
_67 = -Field::<i16>(Variant((*_32), 0), 4);
Goto(bb30)
}
bb68 = {
(*_25) = 3506834882_u32;
(*_4) = (*_52) as f64;
(*_4) = Field::<i8>(Variant(_5, 2), 3) as f64;
_8 = -Field::<i128>(Variant(_5, 2), 4);
_31 = !4137468278_u32;
_36 = _8 as f32;
_18 = _19.1;
_60.2.fld0 = _54;
place!(Field::<i128>(Variant(_13.3, 0), 1)) = _8;
(*_25) = 2489030882_u32 << Field::<i32>(Variant(_5, 2), 0);
place!(Field::<usize>(Variant(_5, 2), 1)) = _35 * _35;
_44.2.fld1 = Move(_4);
_2.fld0 = -_44.2.fld0;
_20 = _17 << Field::<i128>(Variant(_19.3, 0), 1);
_38 = _8 as i8;
(*_25) = !297967712_u32;
(*_25) = _42 as u32;
place!(Field::<bool>(Variant(_16, 0), 0)) = Field::<bool>(Variant(_13.3, 0), 0);
Goto(bb24)
}
bb69 = {
(*_25) = Field::<u32>(Variant((*_32), 0), 0) >> Field::<u32>(Variant((*_32), 0), 0);
_103 = &mut _129.0;
(*_7) = _47 - _115;
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant(_5, 0), 1);
place!(Field::<u32>(Variant((*_32), 0), 0)) = !(*_25);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
(*_130) = Field::<u8>(Variant((*_32), 0), 5) as isize;
(*_25) = _70.2.fld0 as u32;
place!(Field::<i16>(Variant((*_32), 0), 4)) = Field::<i16>(Variant(_70.1, 0), 4) >> (*_103);
_70.2 = Move(_97);
(*_103) = Field::<usize>(Variant(_70.1, 0), 3) + Field::<usize>(Variant(_70.1, 0), 3);
_91 = core::ptr::addr_of!((*_91));
Goto(bb70)
}
bb70 = {
_135 = [Field::<bool>(Variant((*_77), 0), 0),Field::<bool>(Variant(_84, 2), 0),Field::<bool>(Variant((*_77), 0), 0),Field::<bool>(Variant((*_40), 0), 0),Field::<bool>(Variant((*_40), 0), 0),Field::<bool>(Variant((*_40), 0), 0)];
_126 = core::ptr::addr_of!((*_98));
(*_9) = [Field::<u64>(Variant((*_120), 1), 1),Field::<u64>(Variant((*_120), 1), 1),Field::<u64>(Variant((*_120), 1), 1),Field::<u64>(Variant((*_77), 0), 2),Field::<u64>(Variant((*_120), 1), 1),Field::<u64>(Variant((*_120), 1), 1)];
(*_136) = Move(_19.0);
(*_126) = (*_7);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
_112 = Field::<i16>(Variant((*_32), 0), 4) as i32;
place!(Field::<u64>(Variant((*_120), 1), 1)) = (*_98) as u64;
_51 = Move(_9);
(*_25) = !Field::<u32>(Variant((*_32), 0), 0);
(*_120) = _70.1;
_76 = &mut _103;
(*_98) = (*_7) + (*_7);
_79 = (*_114);
place!(Field::<i16>(Variant((*_32), 0), 4)) = Field::<u32>(Variant((*_120), 0), 0) as i16;
(*_114) = _46.fld0;
_78 = !(*_55).fld0;
_99.3 = Move(_5);
_17 = Field::<isize>(Variant((*_120), 0), 2) & Field::<isize>(Variant((*_120), 0), 2);
_111 = _95.1;
_142 = Field::<bool>(Variant(_99.3, 0), 0);
_95.3 = Field::<u32>(Variant((*_32), 0), 0) as usize;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant((*_120), 0), 5) >> Field::<u128>(Variant((*_120), 0), 6);
Call((*_7) = core::intrinsics::transmute((*_130)), ReturnTo(bb71), UnwindUnreachable())
}
bb71 = {
(*_120) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_99.3, 0), 1),fld1: Field::<u64>(Variant(_119, 1), 1),fld2: (*_14) };
(*_7) = (*_98) * (*_98);
match Field::<usize>(Variant(_70.1, 0), 3) {
0 => bb68,
1 => bb72,
18118682628905682682 => bb74,
_ => bb73
}
}
bb72 = {
(*_25) = Field::<u32>(Variant((*_32), 0), 0) - Field::<u32>(Variant((*_32), 0), 0);
_108.0 = _42 as f64;
place!(Field::<isize>(Variant((*_32), 0), 2)) = _2.fld0;
_19.3 = Move(_5);
(*_56) = Field::<char>(Variant((*_32), 0), 1);
Goto(bb44)
}
bb73 = {
(*_25) = Field::<u32>(Variant((*_32), 0), 0) >> Field::<u32>(Variant((*_32), 0), 0);
_103 = &mut _129.0;
(*_7) = _47 - _115;
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant(_5, 0), 1);
place!(Field::<u32>(Variant((*_32), 0), 0)) = !(*_25);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
(*_130) = Field::<u8>(Variant((*_32), 0), 5) as isize;
(*_25) = _70.2.fld0 as u32;
place!(Field::<i16>(Variant((*_32), 0), 4)) = Field::<i16>(Variant(_70.1, 0), 4) >> (*_103);
_70.2 = Move(_97);
(*_103) = Field::<usize>(Variant(_70.1, 0), 3) + Field::<usize>(Variant(_70.1, 0), 3);
_91 = core::ptr::addr_of!((*_91));
Goto(bb70)
}
bb74 = {
place!(Field::<u64>(Variant((*_120), 1), 1)) = 33700_u16 as u64;
place!(Field::<u64>(Variant((*_120), 1), 1)) = Field::<i128>(Variant((*_120), 1), 0) as u64;
_92 = &_136;
_80.fld0 = _95.2.fld0 | Field::<isize>(Variant((*_32), 0), 2);
_138 = Field::<i128>(Variant((*_120), 1), 0) as f64;
place!(Field::<u32>(Variant((*_32), 0), 0)) = (*_25) | (*_25);
place!(Field::<u128>(Variant((*_32), 0), 6)) = (*_55).fld0 & _81;
(*_55).fld0 = (*_130) as u128;
place!(Field::<i16>(Variant((*_32), 0), 4)) = -_67;
(*_55).fld0 = Field::<u128>(Variant((*_32), 0), 6);
_28 = Field::<isize>(Variant((*_32), 0), 2);
place!(Field::<f32>(Variant((*_120), 1), 2)) = (*_14) + (*_64);
_61 = &mut _95.3;
_125 = _65 - (*_14);
place!(Field::<isize>(Variant((*_32), 0), 2)) = _28 >> (*_55).fld0;
match Field::<usize>(Variant(_70.1, 0), 3) {
0 => bb16,
1 => bb34,
18118682628905682682 => bb76,
_ => bb75
}
}
bb75 = {
Return()
}
bb76 = {
(*_61) = Field::<usize>(Variant(_70.1, 0), 3) | Field::<usize>(Variant(_70.1, 0), 3);
place!(Field::<u64>(Variant((*_120), 1), 1)) = Field::<u64>(Variant(_88.1, 1), 1) + Field::<u64>(Variant(_88.1, 1), 1);
_93 = Move(_70.0);
(*_114) = Field::<char>(Variant((*_32), 0), 1);
Goto(bb77)
}
bb77 = {
Goto(bb78)
}
bb78 = {
place!(Field::<u64>(Variant((*_120), 1), 1)) = Field::<u64>(Variant(_119, 1), 1);
(*_25) = (*_114) as u32;
_145 = Field::<i128>(Variant((*_120), 1), 0) as f32;
place!(Field::<isize>(Variant((*_32), 0), 2)) = (*_130) << (*_130);
_88.3 = &mut (*_55).fld0;
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant(_99.3, 0), 1) & Field::<i128>(Variant(_99.3, 0), 1);
_76 = &mut _61;
(*_130) = -_23;
_93 = Move(_88.0);
(*_91) = &_99.3;
Goto(bb79)
}
bb79 = {
place!(Field::<u64>(Variant((*_120), 1), 1)) = !Field::<u64>(Variant((*_40), 0), 2);
match Field::<usize>(Variant(_70.1, 0), 3) {
0 => bb18,
1 => bb25,
2 => bb35,
3 => bb59,
4 => bb80,
5 => bb81,
6 => bb82,
18118682628905682682 => bb84,
_ => bb83
}
}
bb80 = {
_45 = (*_56);
_94 = Field::<isize>(Variant((*_32), 0), 2) as f64;
(*_24).fld0 = Field::<u128>(Variant((*_32), 0), 6) * Field::<u128>(Variant((*_32), 0), 6);
Goto(bb35)
}
bb81 = {
Return()
}
bb82 = {
(*_25) = Field::<u32>(Variant((*_32), 0), 0) >> Field::<u32>(Variant((*_32), 0), 0);
_103 = &mut _129.0;
(*_7) = _47 - _115;
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant(_5, 0), 1);
place!(Field::<u32>(Variant((*_32), 0), 0)) = !(*_25);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_114);
(*_130) = Field::<u8>(Variant((*_32), 0), 5) as isize;
(*_25) = _70.2.fld0 as u32;
place!(Field::<i16>(Variant((*_32), 0), 4)) = Field::<i16>(Variant(_70.1, 0), 4) >> (*_103);
_70.2 = Move(_97);
(*_103) = Field::<usize>(Variant(_70.1, 0), 3) + Field::<usize>(Variant(_70.1, 0), 3);
_91 = core::ptr::addr_of!((*_91));
Goto(bb70)
}
bb83 = {
place!(Field::<i16>(Variant((*_32), 0), 4)) = -Field::<i16>(Variant(_70.1, 0), 4);
place!(Field::<isize>(Variant((*_32), 0), 2)) = (*_50) as isize;
place!(Field::<u128>(Variant((*_32), 0), 6)) = 7892271264961887847_usize as u128;
place!(Field::<u128>(Variant((*_32), 0), 6)) = !_78;
(*_9) = [Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant((*_40), 0), 2),Field::<u64>(Variant(_88.1, 1), 1)];
(*_56) = _57;
place!(Field::<u8>(Variant((*_32), 0), 5)) = Field::<u8>(Variant(_70.1, 0), 5) | Field::<u8>(Variant(_70.1, 0), 5);
Goto(bb49)
}
bb84 = {
_56 = &mut (*_114);
_155 = (Field::<u32>(Variant((*_32), 0), 0), Field::<u32>(Variant((*_32), 0), 0), (*_120), (*_14));
match Field::<usize>(Variant(_70.1, 0), 3) {
0 => bb82,
1 => bb37,
2 => bb40,
3 => bb74,
18118682628905682682 => bb85,
_ => bb42
}
}
bb85 = {
place!(Field::<u128>(Variant((*_32), 0), 6)) = Field::<u128>(Variant(_70.1, 0), 6) ^ Field::<u128>(Variant(_70.1, 0), 6);
(*_76) = &mut place!(Field::<usize>(Variant(_70.1, 0), 3));
_80.fld1 = core::ptr::addr_of_mut!((*_7));
_102 = Field::<isize>(Variant((*_32), 0), 2) - Field::<isize>(Variant((*_32), 0), 2);
_73 = Field::<u64>(Variant((*_40), 0), 2);
_81 = Field::<u128>(Variant((*_32), 0), 6);
_102 = !(*_130);
(*_120) = Adt17::Variant2 { fld0: (*_7),fld1: 13088_u16,fld2: Field::<u8>(Variant((*_32), 0), 5) };
(*_98) = (*_7) + (*_7);
place!(Field::<f64>(Variant((*_120), 2), 0)) = (*_98);
_96 = Move(_88.3);
_2.fld0 = -(*_130);
place!(Field::<u128>(Variant((*_32), 0), 6)) = !_78;
(*_120) = _119;
(*_120) = Adt17::Variant2 { fld0: (*_98),fld1: 40056_u16,fld2: Field::<u8>(Variant((*_32), 0), 5) };
(*_41) = [_46.fld0,_88.2.fld0,(*_56),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),(*_56)];
_80.fld1 = core::ptr::addr_of_mut!((*_98));
_85 = Move((*_76));
Goto(bb86)
}
bb86 = {
(*_120) = _155.2;
place!(Field::<i128>(Variant((*_120), 1), 0)) = !Field::<i128>(Variant(_119, 1), 0);
place!(Field::<u32>(Variant((*_32), 0), 0)) = !_155.1;
(*_41) = [Field::<char>(Variant((*_32), 0), 1),(*_56),Field::<char>(Variant((*_32), 0), 1),(*_56),_82,(*_56)];
place!(Field::<u128>(Variant((*_32), 0), 6)) = _78 * _81;
_90 = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -_67;
(*_41) = [Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1),(*_56),(*_56),Field::<char>(Variant((*_32), 0), 1),Field::<char>(Variant((*_32), 0), 1)];
(*_120) = Adt17::Variant1 { fld0: Field::<i128>(Variant((*_40), 0), 1),fld1: Field::<u64>(Variant((*_40), 0), 2),fld2: (*_14) };
(*_25) = !Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<f32>(Variant(_88.1, 1), 2)) = -_36;
place!(Field::<u128>(Variant((*_32), 0), 6)) = _81 | _81;
(*_7) = (*_98) - _138;
_91 = core::ptr::addr_of!((*_91));
_28 = Field::<isize>(Variant((*_32), 0), 2) >> _18;
Goto(bb87)
}
bb87 = {
_99.1 = -Field::<isize>(Variant((*_32), 0), 2);
place!(Field::<u8>(Variant((*_32), 0), 5)) = _48 << Field::<i16>(Variant((*_32), 0), 4);
_82 = Field::<char>(Variant((*_32), 0), 1);
place!(Field::<i16>(Variant((*_32), 0), 4)) = -_67;
(*_98) = (*_25) as f64;
_108.0 = Field::<u8>(Variant((*_32), 0), 5) as f64;
_13.0 = Move((*_136));
_160 = Field::<bool>(Variant((*_40), 0), 0) & Field::<bool>(Variant((*_40), 0), 0);
place!(Field::<char>(Variant((*_32), 0), 1)) = (*_56);
Goto(bb88)
}
bb88 = {
(*_136) = Move(_13.0);
place!(Field::<i128>(Variant((*_120), 1), 0)) = Field::<i128>(Variant((*_40), 0), 1);
_128 = Field::<char>(Variant((*_32), 0), 1);
_121.1 = !Field::<u32>(Variant((*_32), 0), 0);
place!(Field::<u8>(Variant((*_32), 0), 5)) = _48;
_162 = Move(_120);
(*_130) = Field::<isize>(Variant((*_32), 0), 2);
_86 = &mut place!(Field::<u128>(Variant((*_32), 0), 6));
_146 = [34462_u16];
_88.3 = &mut _81;
(*_41) = [Field::<char>(Variant((*_32), 0), 1),(*_56),(*_56),_82,_82,_82];
_99.2 = [4673_u16];
_159 = 11015815805207716267_usize | 4_usize;
(*_86) = _104 as u128;
RET = core::ptr::addr_of_mut!((*_41));
place!(Field::<u8>(Variant((*_32), 0), 5)) = _28 as u8;
_48 = _38 as u8;
(*_56) = _71.fld0;
_140 = (*_130) + _17;
(*_25) = !Field::<u32>(Variant((*_32), 0), 0);
_17 = (*_130);
_13.3 = Move(_99.3);
_55 = core::ptr::addr_of!(_133.0);
(*_41) = [_128,Field::<char>(Variant((*_32), 0), 1),_79,(*_56),Field::<char>(Variant((*_32), 0), 1),(*_56)];
_80.fld0 = !(*_130);
Goto(bb89)
}
bb89 = {
Call(_166 = dump_var(Move(_38), Move(_17), Move(_73), Move(_159)), ReturnTo(bb90), UnwindUnreachable())
}
bb90 = {
Call(_166 = dump_var(Move(_28), Move(_106), Move(_34), Move(_33)), ReturnTo(bb91), UnwindUnreachable())
}
bb91 = {
Call(_166 = dump_var(Move(_112), Move(_79), Move(_110), Move(_129)), ReturnTo(bb92), UnwindUnreachable())
}
bb92 = {
Call(_166 = dump_var(Move(_48), Move(_81), Move(_90), Move(_102)), ReturnTo(bb93), UnwindUnreachable())
}
bb93 = {
Call(_166 = dump_var(Move(_37), Move(_146), Move(_57), Move(_20)), ReturnTo(bb94), UnwindUnreachable())
}
bb94 = {
Call(_166 = dump_var(Move(_128), Move(_23), _167, _167), ReturnTo(bb95), UnwindUnreachable())
}
bb95 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: &'static char,mut _2: Adt60,mut _3: &'static i32,mut _4: Adt40,mut _5: *mut f64,mut _6: [u64; 6],mut _7: &'static mut [u64; 6],mut _8: Adt19,mut _9: &'static f32) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _10: bool;
let _11: *mut *const bool;
let _12: u16;
let _13: i128;
let _14: f32;
let _15: i32;
let _16: i128;
let _17: f32;
let _18: bool;
let _19: &'static &'static mut u128;
let _20: &'static mut [u64; 6];
let _21: bool;
let _22: *const f64;
let _23: bool;
let _24: i16;
let _25: &'static *const *mut u16;
let _26: &'static f32;
let _27: &'static (f64, &'static (u32, u32, Adt17, f32));
let _28: u128;
let _29: isize;
let _30: &'static mut &'static *mut bool;
let _31: (*mut u16, isize, [u16; 1], Adt19);
let _32: char;
let _33: bool;
let _34: &'static mut u128;
let _35: &'static char;
let _36: (Adt50,);
let _37: &'static &'static mut [u64; 6];
let _38: &'static mut &'static mut usize;
let _39: &'static mut usize;
let _40: *mut *mut *const bool;
let _41: (usize, bool);
let _42: u8;
let _43: u64;
let _44: f32;
let _45: bool;
let _46: &'static (u32, u32, Adt17, f32);
let _47: ((&'static char,), &'static (f64, &'static (u32, u32, Adt17, f32)));
let _48: i16;
let _49: u32;
let _50: isize;
let _51: bool;
let _52: i16;
let _53: (f64, &'static (u32, u32, Adt17, f32));
let _54: &'static mut usize;
let _55: &'static f32;
let _56: i32;
let _57: &'static (u32, u32, Adt17, f32);
let _58: u128;
let _59: isize;
let _60: [char; 7];
let _61: f64;
let _62: char;
let _63: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _64: bool;
let _65: &'static f32;
let _66: f64;
let _67: *const i64;
let _68: i8;
let _69: i128;
let _70: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _71: f64;
let _72: [i8; 6];
let _73: char;
let _74: (usize, bool);
let _75: bool;
let _76: i128;
let _77: isize;
let _78: &'static Adt19;
let _79: f32;
let _80: f64;
let _81: &'static &'static mut u128;
let _82: char;
let _83: char;
let _84: isize;
let _85: *const f64;
let _86: i64;
let _87: &'static i32;
let _88: (&'static mut char, f32, Adt40, usize);
let _89: i8;
let _90: [char; 1];
let _91: i16;
let _92: *const *mut u16;
let _93: &'static &'static mut [u64; 6];
let _94: &'static *const isize;
let _95: f64;
let _96: ();
let _97: ();
{
place!(Field::<i128>(Variant(_8, 2), 4)) = -(-155319621018710662227620952210021175029_i128);
place!(Field::<usize>(Variant(_8, 2), 1)) = 1515051032133580545_usize;
place!(Field::<i32>(Variant(_8, 2), 0)) = Field::<bool>(Variant(_2, 2), 0) as i32;
_3 = &place!(Field::<i32>(Variant(_8, 2), 0));
place!(Field::<i32>(Variant(_8, 2), 0)) = (-738364266_i32) | 2099180035_i32;
_10 = Field::<bool>(Variant(_2, 2), 0);
RET = [2330_u16];
place!(Field::<usize>(Variant(_8, 2), 1)) = !1_usize;
_4 = Adt40 { fld0: (-9223372036854775808_isize),fld1: Move(_5) };
_5 = Move(_4.fld1);
place!(Field::<i8>(Variant(_8, 2), 3)) = (-9_i8) ^ (-88_i8);
place!(Field::<u8>(Variant(_8, 2), 2)) = 234_u8 * 134_u8;
_6 = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_12 = 40943_u16;
place!(Field::<i32>(Variant(_8, 2), 0)) = !2010584896_i32;
place!(Field::<i128>(Variant(_8, 2), 4)) = 108555866473674433363538546370402284586_i128;
_13 = !Field::<i128>(Variant(_8, 2), 4);
_4.fld0 = 47_isize;
place!(Field::<usize>(Variant(_8, 2), 1)) = 14147908097879467539_usize & 17941432851538827504_usize;
Goto(bb1)
}
bb1 = {
_4.fld1 = Move(_5);
_5 = Move(_4.fld1);
place!(Field::<usize>(Variant(_8, 2), 1)) = 7_usize >> _13;
_14 = (-7979224065020198688_i64) as f32;
place!(Field::<u64>(Variant(_2, 2), 1)) = 3284412310313282658_u64;
_13 = Field::<i128>(Variant(_8, 2), 4);
place!(Field::<usize>(Variant(_8, 2), 1)) = 2631175797056625074_usize;
_8 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_2, 2), 0),fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
_4.fld1 = Move(_5);
_4.fld0 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_12 = 5962_u16;
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2) | Field::<u64>(Variant(_8, 0), 2);
_14 = 287704243274509662611774364755516962964_u128 as f32;
RET = [_12];
_7 = &mut _6;
place!(Field::<bool>(Variant(_2, 2), 0)) = Field::<i128>(Variant(_8, 0), 1) < _13;
_10 = !Field::<bool>(Variant(_8, 0), 0);
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2)];
_15 = 1723103335_i32 ^ (-1463260058_i32);
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_16 = _13 | _13;
Goto(bb2)
}
bb2 = {
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
match Field::<i128>(Variant(_8, 0), 1) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
108555866473674433363538546370402284586 => bb11,
_ => bb10
}
}
bb3 = {
_4.fld1 = Move(_5);
_5 = Move(_4.fld1);
place!(Field::<usize>(Variant(_8, 2), 1)) = 7_usize >> _13;
_14 = (-7979224065020198688_i64) as f32;
place!(Field::<u64>(Variant(_2, 2), 1)) = 3284412310313282658_u64;
_13 = Field::<i128>(Variant(_8, 2), 4);
place!(Field::<usize>(Variant(_8, 2), 1)) = 2631175797056625074_usize;
_8 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_2, 2), 0),fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
_4.fld1 = Move(_5);
_4.fld0 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_12 = 5962_u16;
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2) | Field::<u64>(Variant(_8, 0), 2);
_14 = 287704243274509662611774364755516962964_u128 as f32;
RET = [_12];
_7 = &mut _6;
place!(Field::<bool>(Variant(_2, 2), 0)) = Field::<i128>(Variant(_8, 0), 1) < _13;
_10 = !Field::<bool>(Variant(_8, 0), 0);
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2)];
_15 = 1723103335_i32 ^ (-1463260058_i32);
(*_7) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_16 = _13 | _13;
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
place!(Field::<u64>(Variant(_8, 0), 2)) = Field::<u64>(Variant(_2, 2), 1);
_3 = &_15;
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_10 = !Field::<bool>(Variant(_8, 0), 0);
_16 = 221377305637377701243966031436471135218_u128 as i128;
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2);
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_17 = 175_u8 as f32;
_8 = Adt19::Variant0 { fld0: _10,fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
_12 = (*_3) as u16;
_4.fld0 = 47_isize;
_20 = &mut (*_7);
_15 = !(-368624928_i32);
_13 = 2_usize as i128;
_14 = 2297967517_u32 as f32;
_16 = _13;
(*_20) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_20) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
_5 = Move(_4.fld1);
match _4.fld0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb10,
47 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
(*_20) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
_7 = Move(_20);
_4 = Adt40 { fld0: 37_isize,fld1: Move(_5) };
_15 = 1152903790_i32;
place!(Field::<i128>(Variant(_8, 0), 1)) = (-4_i8) as i128;
place!(Field::<i128>(Variant(_8, 0), 1)) = _13 << _16;
place!(Field::<u64>(Variant(_8, 0), 2)) = _4.fld0 as u64;
_21 = _10 < Field::<bool>(Variant(_2, 2), 0);
place!(Field::<u64>(Variant(_8, 0), 2)) = !Field::<u64>(Variant(_2, 2), 1);
RET = [_12];
_5 = Move(_4.fld1);
match _4.fld0 {
0 => bb14,
37 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
place!(Field::<i128>(Variant(_8, 0), 1)) = _13;
place!(Field::<u64>(Variant(_2, 2), 1)) = !Field::<u64>(Variant(_8, 0), 2);
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2) ^ Field::<u64>(Variant(_8, 0), 2);
match _4.fld0 {
0 => bb6,
1 => bb13,
37 => bb18,
_ => bb17
}
}
bb17 = {
place!(Field::<u64>(Variant(_8, 0), 2)) = Field::<u64>(Variant(_2, 2), 1);
_3 = &_15;
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_10 = !Field::<bool>(Variant(_8, 0), 0);
_16 = 221377305637377701243966031436471135218_u128 as i128;
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2);
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
_17 = 175_u8 as f32;
_8 = Adt19::Variant0 { fld0: _10,fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
_12 = (*_3) as u16;
_4.fld0 = 47_isize;
_20 = &mut (*_7);
_15 = !(-368624928_i32);
_13 = 2_usize as i128;
_14 = 2297967517_u32 as f32;
_16 = _13;
(*_20) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2)];
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
(*_20) = [Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_8, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
_5 = Move(_4.fld1);
match _4.fld0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb10,
47 => bb13,
_ => bb12
}
}
bb18 = {
_13 = Field::<i128>(Variant(_8, 0), 1) - _16;
_18 = Field::<bool>(Variant(_8, 0), 0);
_15 = (-1795186472_i32) >> _4.fld0;
_4 = Adt40 { fld0: (-9223372036854775808_isize),fld1: Move(_5) };
place!(Field::<bool>(Variant(_2, 2), 0)) = _18;
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2) >> _15;
_4.fld0 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_23 = Field::<bool>(Variant(_8, 0), 0);
_15 = (-283698920_i32) | 108872611_i32;
_5 = Move(_4.fld1);
place!(Field::<bool>(Variant(_2, 2), 0)) = Field::<bool>(Variant(_8, 0), 0) ^ _23;
Call(place!(Field::<u64>(Variant(_8, 0), 2)) = core::intrinsics::bswap(Field::<u64>(Variant(_2, 2), 1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_4.fld1 = Move(_5);
_12 = 63094_u16 + 55484_u16;
_9 = &_17;
_5 = Move(_4.fld1);
_24 = '\u{18718}' as i16;
_14 = -(*_9);
_26 = &(*_9);
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2) & Field::<u64>(Variant(_8, 0), 2);
place!(Field::<u64>(Variant(_2, 2), 1)) = Field::<u64>(Variant(_8, 0), 2);
_17 = _14 * _14;
_4.fld1 = Move(_5);
place!(Field::<i128>(Variant(_8, 0), 1)) = _13 - _13;
place!(Field::<bool>(Variant(_2, 2), 0)) = _23 | _21;
_15 = (-1978837910_i32) * 167508373_i32;
_5 = Move(_4.fld1);
place!(Field::<i128>(Variant(_8, 0), 1)) = _13 * _16;
_31.0 = core::ptr::addr_of_mut!(_12);
place!(Field::<u64>(Variant(_2, 2), 1)) = !Field::<u64>(Variant(_8, 0), 2);
Goto(bb20)
}
bb20 = {
place!(Field::<bool>(Variant(_8, 0), 0)) = !Field::<bool>(Variant(_2, 2), 0);
place!(Field::<u64>(Variant(_8, 0), 2)) = (-25_i8) as u64;
RET = [_12];
_2 = Adt60::Variant2 { fld0: Field::<bool>(Variant(_8, 0), 0),fld1: Field::<u64>(Variant(_8, 0), 2) };
_29 = !_4.fld0;
_21 = Field::<i128>(Variant(_8, 0), 1) > _13;
_21 = !_18;
_31.1 = _4.fld0;
Goto(bb21)
}
bb21 = {
_21 = Field::<bool>(Variant(_2, 2), 0) != _10;
place!(Field::<i128>(Variant(_8, 0), 1)) = _13 | _16;
_4.fld1 = Move(_5);
_32 = '\u{5cd4d}';
Goto(bb22)
}
bb22 = {
_28 = 35_u8 as u128;
_31.1 = !_4.fld0;
_31.2 = [_12];
_12 = !50305_u16;
_1 = &_32;
place!(Field::<u64>(Variant(_8, 0), 2)) = Field::<u64>(Variant(_2, 2), 1) * Field::<u64>(Variant(_2, 2), 1);
place!(Field::<bool>(Variant(_8, 0), 0)) = (*_1) < (*_1);
_34 = &mut _28;
_29 = !_31.1;
_9 = &_14;
_31.3 = Adt19::Variant0 { fld0: _21,fld1: Field::<i128>(Variant(_8, 0), 1),fld2: Field::<u64>(Variant(_8, 0), 2) };
_36.0 = Adt50 { fld0: (*_34) };
_14 = _17;
(*_34) = !_36.0.fld0;
place!(Field::<i128>(Variant(_31.3, 0), 1)) = Field::<i128>(Variant(_8, 0), 1) * Field::<i128>(Variant(_8, 0), 1);
_17 = _14 - _14;
Goto(bb23)
}
bb23 = {
_21 = (*_1) != (*_1);
_8 = Adt19::Variant0 { fld0: _23,fld1: Field::<i128>(Variant(_31.3, 0), 1),fld2: Field::<u64>(Variant(_31.3, 0), 2) };
_31.2 = [_12];
(*_34) = _36.0.fld0 + _36.0.fld0;
_31.0 = core::ptr::addr_of_mut!(_12);
(*_34) = _36.0.fld0 | _36.0.fld0;
place!(Field::<u64>(Variant(_8, 0), 2)) = Field::<u64>(Variant(_31.3, 0), 2) - Field::<u64>(Variant(_31.3, 0), 2);
_21 = Field::<u64>(Variant(_8, 0), 2) <= Field::<u64>(Variant(_8, 0), 2);
(*_34) = _36.0.fld0 >> Field::<i128>(Variant(_31.3, 0), 1);
(*_34) = _31.1 as u128;
Goto(bb24)
}
bb24 = {
_31.1 = _4.fld0;
(*_34) = 1264619598_u32 as u128;
(*_34) = !_36.0.fld0;
(*_34) = _24 as u128;
(*_34) = _36.0.fld0 ^ _36.0.fld0;
_31.1 = 2015418961755277491_i64 as isize;
_12 = 34695_u16 & 16857_u16;
_31.0 = core::ptr::addr_of_mut!(_12);
(*_34) = _36.0.fld0;
(*_34) = _36.0.fld0 - _36.0.fld0;
_35 = &(*_1);
_17 = _14 * _14;
_13 = Field::<i128>(Variant(_31.3, 0), 1);
Call(RET = fn13(Move(_34), Field::<bool>(Variant(_8, 0), 0)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_31.3 = Move(_8);
_26 = &_14;
_17 = (*_26);
RET = [_12];
_26 = &_17;
_26 = Move(_9);
_13 = Field::<i128>(Variant(_31.3, 0), 1);
_1 = Move(_35);
_31.3 = Adt19::Variant0 { fld0: _21,fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
_9 = &_17;
_8 = Move(_31.3);
Call(place!(Field::<u64>(Variant(_8, 0), 2)) = core::intrinsics::transmute(_4.fld0), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_41.1 = _10 ^ Field::<bool>(Variant(_2, 2), 0);
_40 = core::ptr::addr_of_mut!(_11);
_13 = Field::<i128>(Variant(_8, 0), 1);
_2 = Adt60::Variant2 { fld0: Field::<bool>(Variant(_8, 0), 0),fld1: Field::<u64>(Variant(_8, 0), 2) };
_36.0 = Adt50 { fld0: 316460309008760667728456183425606307063_u128 };
_5 = Move(_4.fld1);
_10 = _41.1;
place!(Field::<bool>(Variant(_2, 2), 0)) = _23;
Goto(bb27)
}
bb27 = {
_31.3 = Move(_8);
_24 = -(-8607_i16);
RET = _31.2;
_17 = Field::<u64>(Variant(_2, 2), 1) as f32;
_43 = Field::<u64>(Variant(_31.3, 0), 2) * Field::<u64>(Variant(_2, 2), 1);
_33 = _10 < Field::<bool>(Variant(_2, 2), 0);
_10 = _41.1;
_24 = (-25289_i16) - (-30656_i16);
_32 = '\u{66fbe}';
_10 = _18;
_43 = Field::<i128>(Variant(_31.3, 0), 1) as u64;
place!(Field::<u64>(Variant(_2, 2), 1)) = _43;
_4.fld0 = _29 ^ _29;
_36.0 = Adt50 { fld0: 207901471756391691608937566627066055100_u128 };
_23 = _33;
_31.1 = _4.fld0 | _4.fld0;
_43 = !Field::<u64>(Variant(_2, 2), 1);
_44 = _14 - _17;
_4.fld1 = Move(_5);
_21 = _23;
_42 = 25_u8 | 63_u8;
_12 = 43000_u16 + 19068_u16;
Goto(bb28)
}
bb28 = {
_41.1 = !_23;
_44 = _14 - _17;
_31.0 = core::ptr::addr_of_mut!(_12);
_41 = Checked(1_usize * 16697303477582970688_usize);
_36.0.fld0 = 105914663462630893958905620111557394151_u128;
place!(Field::<bool>(Variant(_2, 2), 0)) = _33 & _18;
_45 = _29 < _29;
_34 = &mut _36.0.fld0;
(*_34) = !19672424339466056274407643595266469541_u128;
(*_34) = 9476335087461417206368157126395129781_u128 - 96805250011141665826555835667719536761_u128;
_1 = &_32;
_41.1 = _23 ^ _21;
(*_34) = !262362456862347508864856852616830740840_u128;
place!(Field::<u64>(Variant(_2, 2), 1)) = _43;
_2 = Adt60::Variant2 { fld0: _21,fld1: _43 };
Goto(bb29)
}
bb29 = {
_39 = &mut _41.0;
_47.0.0 = &(*_1);
_5 = Move(_4.fld1);
(*_34) = 2273210692174816588365881968193794971_u128;
_8 = Move(_31.3);
_26 = &_14;
match (*_34) {
2273210692174816588365881968193794971 => bb30,
_ => bb28
}
}
bb30 = {
(*_34) = 4282783907332440704391379905138189664_u128 ^ 26799927032574038423119903975933110964_u128;
(*_34) = 272721251272309684692188692565473949259_u128 << (*_39);
(*_39) = 10757338317331512851_usize * 4_usize;
(*_34) = 2715481345_u32 as u128;
_29 = _31.1 - _31.1;
_17 = (*_26);
(*_39) = 3_usize + 0_usize;
_33 = !Field::<bool>(Variant(_8, 0), 0);
(*_34) = !274132686958825766767808987167820987701_u128;
_48 = _24;
_4 = Adt40 { fld0: _29,fld1: Move(_5) };
_31.3 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_2, 2), 0),fld1: _16,fld2: Field::<u64>(Variant(_2, 2), 1) };
(*_34) = !177299955893156933031235092554789242672_u128;
_49 = 742681382_u32 + 2289083790_u32;
_8 = Move(_31.3);
Goto(bb31)
}
bb31 = {
_8 = Adt19::Variant2 { fld0: _15,fld1: (*_39),fld2: _42,fld3: 17_i8,fld4: _13 };
_14 = Field::<i32>(Variant(_8, 2), 0) as f32;
(*_39) = Field::<i32>(Variant(_8, 2), 0) as usize;
(*_34) = 309664450288112124476374158920306347377_u128 >> _42;
(*_34) = 207871212393839366749690709415083931568_u128;
_32 = '\u{ef632}';
(*_39) = 8726451634620108671_i64 as usize;
place!(Field::<i32>(Variant(_8, 2), 0)) = -_15;
_53.0 = _42 as f64;
(*_34) = !309089434723208456962458080178735817079_u128;
Goto(bb32)
}
bb32 = {
(*_34) = 104_i8 as u128;
_54 = &mut (*_39);
(*_54) = Field::<i128>(Variant(_8, 2), 4) as usize;
(*_34) = 50525250241314577082049067044807825398_u128;
_49 = _12 as u32;
_1 = Move(_47.0.0);
_49 = 3776040590_u32 >> (*_54);
(*_34) = 103329565006929837646174068323627588351_u128;
_26 = &_17;
(*_54) = Field::<usize>(Variant(_8, 2), 1);
(*_54) = !Field::<usize>(Variant(_8, 2), 1);
_50 = _31.1 - _4.fld0;
_54 = &mut place!(Field::<usize>(Variant(_8, 2), 1));
_35 = &_32;
(*_34) = _12 as u128;
_51 = !Field::<bool>(Variant(_2, 2), 0);
Goto(bb33)
}
bb33 = {
_31.3 = Adt19::Variant0 { fld0: _23,fld1: _13,fld2: Field::<u64>(Variant(_2, 2), 1) };
(*_34) = 211727015336323968972652848391161708231_u128;
_29 = _33 as isize;
(*_54) = 5257359373739027305_usize;
(*_54) = 10446064787190533640_usize & 10075575176819553144_usize;
(*_54) = 7_usize << _50;
_31.0 = core::ptr::addr_of_mut!(_12);
(*_54) = 2_usize ^ 4_usize;
_55 = &_44;
(*_34) = 218794090835114449544333586385622581369_u128 | 9148200129585411395751557001370627470_u128;
(*_34) = 58688095187813496291650955921883585616_u128;
(*_34) = _13 as u128;
_26 = Move(_55);
_45 = Field::<u64>(Variant(_31.3, 0), 2) <= _43;
(*_34) = 286233281371987098505004683417896499440_u128;
(*_54) = 5789979981945051030_usize | 0_usize;
place!(Field::<bool>(Variant(_31.3, 0), 0)) = _4.fld0 != _4.fld0;
_56 = _24 as i32;
_47.0.0 = &(*_35);
_61 = (-104_i8) as f64;
(*_34) = 43759498129797920137116142489968782947_u128 + 32821857968419953231358426280026368870_u128;
_58 = !(*_34);
Goto(bb34)
}
bb34 = {
(*_34) = !_58;
Goto(bb35)
}
bb35 = {
(*_34) = _58 << _31.1;
(*_34) = _58;
Goto(bb36)
}
bb36 = {
(*_34) = _58 - _58;
(*_34) = _58 << (*_54);
_64 = (*_54) > (*_54);
_26 = &_44;
(*_34) = _58 - _58;
_63.2 = core::ptr::addr_of!(_29);
_63.3 = Adt50 { fld0: (*_34) };
Goto(bb37)
}
bb37 = {
(*_34) = _63.3.fld0 ^ _58;
(*_34) = (*_54) as u128;
_5 = Move(_4.fld1);
_44 = _14 - _14;
_56 = _15;
_64 = _45;
_65 = &_14;
_70.3 = [_43,_43,_43,Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_31.3, 0), 2)];
(*_34) = _63.3.fld0 ^ _63.3.fld0;
_56 = -_15;
_70.2 = (Move(_5), Move(_47.0.0));
_53.0 = _61;
Goto(bb38)
}
bb38 = {
_31.1 = !_4.fld0;
_17 = _44;
_22 = core::ptr::addr_of!(_61);
place!(Field::<i128>(Variant(_31.3, 0), 1)) = _13 | _13;
_15 = (*_54) as i32;
_5 = core::ptr::addr_of_mut!((*_22));
(*_34) = !_58;
_69 = Field::<i128>(Variant(_31.3, 0), 1) ^ _16;
_4.fld1 = core::ptr::addr_of_mut!((*_22));
_63.2 = core::ptr::addr_of!(_50);
_69 = Field::<i128>(Variant(_31.3, 0), 1) * Field::<i128>(Variant(_31.3, 0), 1);
_47.0.0 = &(*_35);
(*_54) = 3715044722315023375_usize & 15311886387568614563_usize;
_69 = _13 | Field::<i128>(Variant(_31.3, 0), 1);
_70.3 = [_43,_43,_43,Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_31.3, 0), 2),Field::<u64>(Variant(_2, 2), 1)];
_71 = (*_22) - (*_5);
(*_5) = -_71;
_61 = _71 - _53.0;
Goto(bb39)
}
bb39 = {
_7 = &mut _70.3;
(*_5) = _71 * _71;
_18 = _23 | _51;
_9 = &(*_65);
_69 = !Field::<i128>(Variant(_31.3, 0), 1);
_55 = &(*_9);
_32 = '\u{e7355}';
_63.3 = Adt50 { fld0: (*_34) };
Call((*_34) = core::intrinsics::transmute(_63.3.fld0), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_42 = 82_u8 + 56_u8;
(*_5) = _71;
_64 = _23 | Field::<bool>(Variant(_31.3, 0), 0);
Goto(bb41)
}
bb41 = {
_38 = &mut _39;
(*_7) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_31.3, 0), 2),Field::<u64>(Variant(_31.3, 0), 2),Field::<u64>(Variant(_31.3, 0), 2)];
(*_54) = 2_usize;
(*_5) = _53.0 - _71;
Call((*_22) = core::intrinsics::fmaf64(_71, _71, _71), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
(*_22) = _71;
_19 = &_34;
_31.1 = _29;
_63.0 = core::ptr::addr_of!(_47.0);
_31.3 = Adt19::Variant2 { fld0: _56,fld1: (*_54),fld2: _42,fld3: 14_i8,fld4: _13 };
(*_54) = Field::<usize>(Variant(_31.3, 2), 1) ^ Field::<usize>(Variant(_31.3, 2), 1);
(*_7) = [_43,_43,Field::<u64>(Variant(_2, 2), 1),_43,_43,_43];
_20 = &mut (*_7);
(*_22) = _71;
_37 = &_7;
_48 = -_24;
_65 = &_44;
place!(Field::<bool>(Variant(_2, 2), 0)) = (*_65) < (*_65);
_63.3.fld0 = !(*_34);
(*_38) = &mut (*_54);
(*_34) = _63.3.fld0 | _63.3.fld0;
_5 = core::ptr::addr_of_mut!((*_22));
(*_38) = &mut place!(Field::<usize>(Variant(_31.3, 2), 1));
(*_22) = _71;
_4.fld0 = _29;
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),_43,_43,Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1)];
(*_34) = !_63.3.fld0;
_63.0 = core::ptr::addr_of!(_47.0);
_18 = _64;
Goto(bb43)
}
bb43 = {
_73 = _32;
_32 = _73;
(*_22) = -_53.0;
_76 = _69 >> _69;
(*_38) = Move(_54);
_63.1 = (*_20);
(*_22) = (-6612561604875046780_i64) as f64;
_62 = _73;
_7 = &mut _63.1;
_66 = -(*_22);
_60 = [_73,_32,_62,_32,_32,_32,_73];
(*_7) = (*_20);
(*_22) = _71;
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),_43,_43,_43,Field::<u64>(Variant(_2, 2), 1)];
_12 = 1965_u16 * 51835_u16;
(*_20) = (*_7);
Goto(bb44)
}
bb44 = {
(*_20) = [Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),_43,_43,_43];
(*_34) = _58 | _58;
(*_7) = (*_20);
(*_34) = _58 | _58;
(*_20) = (*_7);
(*_20) = [_43,_43,Field::<u64>(Variant(_2, 2), 1),Field::<u64>(Variant(_2, 2), 1),_43,_43];
Goto(bb45)
}
bb45 = {
_51 = !_10;
(*_7) = [Field::<u64>(Variant(_2, 2), 1),_43,Field::<u64>(Variant(_2, 2), 1),_43,_43,Field::<u64>(Variant(_2, 2), 1)];
_32 = _73;
_47.0 = (Move(_35),);
_20 = Move(_7);
RET = [_12];
(*_34) = _58;
_80 = (*_22) - (*_22);
(*_34) = !_58;
Goto(bb46)
}
bb46 = {
_35 = &_32;
(*_34) = _58 ^ _58;
_88.2.fld1 = core::ptr::addr_of_mut!((*_22));
(*_22) = _53.0 + _53.0;
_32 = _62;
(*_34) = _14 as u128;
_60 = [_73,_73,_32,_73,_62,_73,_73];
_12 = !40617_u16;
_5 = core::ptr::addr_of_mut!(_80);
_17 = 4102702121425058768_usize as f32;
_3 = &_56;
_74.1 = (*_3) <= (*_3);
_86 = 4707009598769353151_i64 | 2111432501166978851_i64;
(*_5) = (*_22);
(*_5) = (*_22) * (*_22);
_90 = [_62];
(*_34) = _23 as u128;
(*_5) = (*_22);
Goto(bb47)
}
bb47 = {
_24 = _48 ^ _48;
(*_5) = (*_22) * (*_22);
(*_5) = (*_22);
_35 = &_62;
_52 = _24 | _48;
_51 = (*_34) <= (*_34);
_42 = 127_u8 << (*_34);
_5 = Move(_4.fld1);
_29 = _50 | _50;
_10 = _18 ^ _21;
_32 = (*_35);
(*_34) = !_58;
_21 = (*_34) == (*_34);
place!(Field::<bool>(Variant(_2, 2), 0)) = (*_65) == (*_65);
place!(Field::<bool>(Variant(_2, 2), 0)) = (*_22) <= (*_22);
(*_34) = !_58;
_59 = -_50;
(*_22) = -_71;
_76 = (*_3) as i128;
Goto(bb48)
}
bb48 = {
(*_22) = _80;
_35 = &_32;
_71 = -(*_22);
(*_34) = _58 * _58;
Call(_76 = core::intrinsics::bswap(_69), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_69 = _76;
_86 = _52 as i64;
(*_34) = !_58;
_81 = &(*_19);
_10 = _18 == _33;
(*_34) = !_58;
(*_34) = !_58;
_18 = _29 <= _29;
_32 = _73;
(*_22) = 9771334165090327855_usize as f64;
_88.3 = 6910508904994356634_usize;
_33 = !_10;
_4 = Adt40 { fld0: _29,fld1: Move(_88.2.fld1) };
(*_38) = &mut _88.3;
(*_34) = _58;
Goto(bb50)
}
bb50 = {
Call(_96 = dump_var(Move(_45), Move(_58), Move(_64), Move(_59)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_96 = dump_var(Move(_12), Move(_18), Move(_76), Move(_21)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_96 = dump_var(Move(_32), Move(_50), Move(_51), Move(_62)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_96 = dump_var(Move(_90), Move(_60), Move(_69), Move(_49)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_96 = dump_var(Move(_56), _97, _97, _97), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: &'static mut u128,mut _2: bool) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _3: f64;
let _4: &'static mut [u64; 6];
let _5: (*mut u16, Adt17);
let _6: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _7: &'static f32;
let _8: &'static mut u128;
let _9: *mut [char; 6];
let _10: f32;
let _11: i16;
let _12: (*mut u16, isize, [u16; 1], Adt19);
let _13: i16;
let _14: &'static mut [u64; 6];
let _15: *mut *mut f64;
let _16: isize;
let _17: usize;
let _18: char;
let _19: isize;
let _20: &'static *const isize;
let _21: (&'static char,);
let _22: f64;
let _23: char;
let _24: *mut *mut *const bool;
let _25: i8;
let _26: isize;
let _27: i64;
let _28: (*mut u16, Adt17);
let _29: isize;
let _30: *const i64;
let _31: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _32: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _33: &'static *const isize;
let _34: i32;
let _35: u64;
let _36: isize;
let _37: i16;
let _38: i8;
let _39: &'static i32;
let _40: u8;
let _41: &'static &'static mut u128;
let _42: u8;
let _43: *mut f64;
let _44: *const Adt17;
let _45: &'static f32;
let _46: u16;
let _47: bool;
let _48: *const Adt60;
let _49: usize;
let _50: [i8; 6];
let _51: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _52: &'static mut *const i64;
let _53: Adt19;
let _54: f32;
let _55: [i8; 6];
let _56: [u16; 1];
let _57: (&'static mut char, f32, Adt40, usize);
let _58: (&'static char,);
let _59: char;
let _60: *mut [char; 6];
let _61: u32;
let _62: u64;
let _63: *mut bool;
let _64: [char; 6];
let _65: *mut f64;
let _66: (*mut u16, Adt17);
let _67: *const &'static Adt19;
let _68: &'static Adt19;
let _69: *const isize;
let _70: i8;
let _71: *mut u32;
let _72: *const isize;
let _73: i8;
let _74: i16;
let _75: Adt34;
let _76: &'static mut *const i64;
let _77: *const f64;
let _78: i128;
let _79: &'static mut *const i64;
let _80: char;
let _81: *const Adt60;
let _82: *const Adt60;
let _83: [char; 7];
let _84: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _85: &'static mut [u64; 6];
let _86: &'static mut [u64; 6];
let _87: &'static mut usize;
let _88: (*mut f64, &'static char);
let _89: isize;
let _90: f32;
let _91: i8;
let _92: i128;
let _93: &'static *const *mut u16;
let _94: Adt85;
let _95: &'static *const *mut u16;
let _96: Adt50;
let _97: isize;
let _98: f64;
let _99: *const Adt19;
let _100: &'static mut [u64; 6];
let _101: (*mut u16, isize, [u16; 1], Adt19);
let _102: isize;
let _103: &'static mut u128;
let _104: i64;
let _105: f32;
let _106: f32;
let _107: ();
let _108: ();
{
RET = [33260_u16];
_2 = 13756507309388950920_u64 != 9125519762852978310_u64;
_2 = false;
_2 = (-70_i8) > (-75_i8);
_3 = (-1845352469_i32) as f64;
_2 = true;
RET = [5863_u16];
RET = [33993_u16];
_3 = 97932991293459738643549730165175843619_i128 as f64;
_6.1.1 = Adt17::Variant2 { fld0: _3,fld1: 64726_u16,fld2: 201_u8 };
_6.3 = [8043432548939816858_u64,9282815031998083963_u64,4411926809702130564_u64,8747178762135364899_u64,274494002729393336_u64,7339028708104317345_u64];
_2 = _3 < _3;
place!(Field::<u16>(Variant(_6.1.1, 2), 1)) = 107191537476114935711926548452396173999_i128 as u16;
_6.1.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.1.1, 2), 1)));
place!(Field::<u16>(Variant(_6.1.1, 2), 1)) = (-343919369_i32) as u16;
_4 = &mut _6.3;
(*_4) = [13586621287267179817_u64,10739123378509100024_u64,1103540121586257927_u64,7612233250755066909_u64,4808867489861288567_u64,11940921413633584898_u64];
(*_4) = [13859616299602642114_u64,2264864732257821289_u64,4253795594525860400_u64,9247096718422427164_u64,7292759904767397053_u64,13976629724003763040_u64];
Call((*_4) = fn14(_2, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4) = [9099275316752552265_u64,1689719364741068326_u64,770584985596198485_u64,11376843781301235191_u64,18321683384846694647_u64,6316138467988643395_u64];
(*_4) = [13412361693980486942_u64,12113279685249037403_u64,6478337945258230457_u64,11556622652836073934_u64,10565153509128011029_u64,8426833434449198768_u64];
(*_4) = [5265060634070577202_u64,507689548014445622_u64,3958148897191159705_u64,13579871299775697288_u64,7990882064132689925_u64,6574469865508769694_u64];
(*_4) = [5494643732701293108_u64,8011304741059967896_u64,11933737492698526935_u64,13178576166678259205_u64,9577050507469006869_u64,14413330151831371228_u64];
(*_4) = [7313424603454821649_u64,9856115227228744505_u64,13905771896871476466_u64,3943298795722569243_u64,9676269319639779301_u64,8145433355164519640_u64];
_5.1 = Adt17::Variant0 { fld0: 772611903_u32,fld1: '\u{23593}',fld2: 2_isize,fld3: 10320789600949750719_usize,fld4: 24753_i16,fld5: 247_u8,fld6: 265799612372625982176312860038404398386_u128 };
(*_4) = [8937630159898101467_u64,430028176583093096_u64,8727952038441671874_u64,7848177389119565883_u64,12530089644938862458_u64,1991436026446384272_u64];
place!(Field::<u8>(Variant(_5.1, 0), 5)) = 206_u8 & 149_u8;
(*_4) = [1995339448220923818_u64,1733725374663180475_u64,8974075545746143422_u64,1841476412142292041_u64,3179974101864704041_u64,11655746373095654986_u64];
place!(Field::<char>(Variant(_5.1, 0), 1)) = '\u{c7936}';
_3 = 5003518678286813368_u64 as f64;
_10 = _3 as f32;
place!(Field::<usize>(Variant(_5.1, 0), 3)) = 4_usize << Field::<u8>(Variant(_5.1, 0), 5);
_12.2 = [49482_u16];
(*_4) = [11025424752807255236_u64,1343369599091660961_u64,14718132472118731396_u64,10964605061533108589_u64,1290672284300684828_u64,11322282793657715818_u64];
(*_4) = [1726052534187846481_u64,12284943466108212710_u64,12060114612615693480_u64,9166311209401500627_u64,14054619200899848322_u64,9131308812412336287_u64];
(*_4) = [9612022840697173983_u64,3272998788560127969_u64,15083870966626288993_u64,9968207911061149361_u64,17475326556249555346_u64,8617638975153480325_u64];
(*_4) = [10116514884702498851_u64,5041145073397264844_u64,7526631085897551457_u64,6694183272168402331_u64,6663141354960901581_u64,12536535428538200194_u64];
(*_4) = [13921565857791536140_u64,15762931247918533247_u64,14208739046025948061_u64,3757742646914102560_u64,8236966795507308500_u64,14235435719055924970_u64];
(*_4) = [2236778389309511081_u64,7203821627265948954_u64,10991461705921048280_u64,16164652010266186550_u64,3465727412445014655_u64,15414847536072854403_u64];
(*_4) = [10673056628889025763_u64,6170386728977499538_u64,17762712337509382893_u64,5456738172696206459_u64,3786853576421102195_u64,10705401922891603261_u64];
(*_4) = [15047168376595233204_u64,16074298749406347571_u64,3592092265969096753_u64,5387850886707036226_u64,14426484532480220411_u64,1505046581515752072_u64];
place!(Field::<u128>(Variant(_5.1, 0), 6)) = (-3409147310341006531_i64) as u128;
_5.1 = Adt17::Variant2 { fld0: _3,fld1: 29702_u16,fld2: 147_u8 };
Goto(bb2)
}
bb2 = {
_5.1 = Adt17::Variant2 { fld0: _3,fld1: 9065_u16,fld2: 243_u8 };
(*_4) = [11226727240293794843_u64,13815138726083184229_u64,16112205170164816154_u64,4329898568002655939_u64,1470064934338107508_u64,3349546348411379587_u64];
(*_4) = [15649137074748129677_u64,16119856463583694078_u64,5784299027530659871_u64,4250045410517429889_u64,5963171692083659961_u64,11582413383086341389_u64];
_12.2 = [37796_u16];
_13 = !(-24211_i16);
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_5.1, 2), 1)));
(*_4) = [16644091119167867735_u64,15077429932773745969_u64,3421611194391640065_u64,4266263569902645894_u64,1828565779681475105_u64,3824492313144074195_u64];
(*_4) = [6630076878351448216_u64,1675834522822450110_u64,1400081002655481346_u64,9721582648867304465_u64,3498495697026183093_u64,3580954723591547948_u64];
(*_4) = [5083344437895614469_u64,15672391871150830886_u64,15463585618530431568_u64,5838356307315406320_u64,15161515877172727693_u64,17584133642903832998_u64];
_12.1 = 9223372036854775807_isize | (-9223372036854775808_isize);
(*_4) = [10442679537626056223_u64,17695583420742283845_u64,6175383067411220428_u64,10428072083128379713_u64,9476981762007817539_u64,14985578723086790657_u64];
_5.1 = Adt17::Variant0 { fld0: 1942543882_u32,fld1: '\u{43c42}',fld2: _12.1,fld3: 47876486434911454_usize,fld4: _13,fld5: 36_u8,fld6: 266573610879297320020939959298018543535_u128 };
(*_4) = [531256757831413134_u64,1008580686333353399_u64,4588282295111474343_u64,14444014406162188566_u64,2061020983377006985_u64,743929756588391927_u64];
_12.2 = RET;
RET = [52134_u16];
(*_4) = [14013906936879612902_u64,8492287322366773226_u64,8840704643154099323_u64,18398405153082177653_u64,8626721196342541527_u64,11800222511231595452_u64];
Goto(bb3)
}
bb3 = {
place!(Field::<u32>(Variant(_5.1, 0), 0)) = 3930462449_u32 | 951432131_u32;
place!(Field::<i16>(Variant(_5.1, 0), 4)) = !_13;
_16 = '\u{cf3ac}' as isize;
_12.0 = Move(_5.0);
(*_4) = [10820963604833534343_u64,1266377363132834957_u64,3987815263097601499_u64,15461231605598024272_u64,8713998124882797595_u64,17607174247157786934_u64];
(*_4) = [15827682864689313673_u64,7838576132507374041_u64,10933895318325517275_u64,7112005440864559766_u64,7673133862953256713_u64,1564610274628539936_u64];
place!(Field::<usize>(Variant(_5.1, 0), 3)) = 4_usize + 7_usize;
_18 = '\u{50b6b}';
(*_4) = [16713817383619997017_u64,11740860869020870648_u64,14957230754973364504_u64,5053341834298702281_u64,17685004747782745530_u64,13110249910061534402_u64];
_10 = 45148602020517759925683992962946292448_u128 as f32;
_7 = &_10;
_10 = 27725072056693184735844896009072186524_i128 as f32;
(*_4) = [6289554359274623091_u64,17741989867497423595_u64,10115952659734795215_u64,7292346739217505312_u64,3507938727786434232_u64,16857657967875612103_u64];
(*_4) = [810946010946427973_u64,5075684499004906475_u64,5175757474533829618_u64,9498063757568341610_u64,4265812894507853514_u64,14967131216301776961_u64];
(*_4) = [2635660572343151002_u64,2384889422615168727_u64,4992949697334017958_u64,8525852007009780582_u64,2274180558121302265_u64,11351150310137668201_u64];
place!(Field::<u32>(Variant(_5.1, 0), 0)) = 1475175340_u32 + 2700482103_u32;
place!(Field::<i16>(Variant(_5.1, 0), 4)) = !_13;
(*_4) = [17918736363219190327_u64,16337963287840344556_u64,8912153871168768796_u64,11836700080408214630_u64,401194573499961442_u64,13152768704927336678_u64];
(*_4) = [6898801776669845568_u64,2689492295983655271_u64,15176127902769095684_u64,1967053468691491780_u64,18282430049718850130_u64,4484886160991567778_u64];
(*_4) = [11622793365331688939_u64,10804464855831681687_u64,14277714183270508954_u64,9263728397509766292_u64,10617277570450685182_u64,8560268565797942973_u64];
Goto(bb4)
}
bb4 = {
(*_4) = [14542694863243950722_u64,4226351997599457221_u64,612892826271435454_u64,13172256967949194144_u64,18335398553673678504_u64,7368488925616443900_u64];
_10 = 3_i8 as f32;
(*_4) = [5757663304897001670_u64,8839017982927628004_u64,6452498290471604884_u64,15012798092319940147_u64,14247253331840039328_u64,11850036377917822592_u64];
(*_4) = [13474189550449755683_u64,13519781474810207998_u64,807716050963133614_u64,15937357675919983752_u64,6300947190991226949_u64,12090887640828619613_u64];
_14 = &mut (*_4);
_10 = (-4_i8) as f32;
place!(Field::<u32>(Variant(_5.1, 0), 0)) = 3069652460_u32 ^ 965958778_u32;
(*_14) = [4564990167565605718_u64,5671946290689781138_u64,7069346923758502022_u64,1030514010944259815_u64,2818950341741961907_u64,12146385518358650877_u64];
_21.0 = &_18;
_12.1 = Field::<isize>(Variant(_5.1, 0), 2) | Field::<isize>(Variant(_5.1, 0), 2);
(*_14) = [17087753214863198565_u64,14336357578372771445_u64,878798604450665977_u64,2057988421846850141_u64,13575315366209489944_u64,4120041151412264964_u64];
(*_14) = [8744169751190287540_u64,16060600121005649850_u64,6224244035905820251_u64,9034608489532910208_u64,1059740060280590736_u64,17474979735370332727_u64];
(*_14) = [5480982503976133183_u64,394569453724233474_u64,11101897097299884305_u64,7972988045108380519_u64,576664516279693172_u64,10828360101342175373_u64];
place!(Field::<u32>(Variant(_5.1, 0), 0)) = 2215977141_u32 - 1410553045_u32;
(*_14) = [8766687408149812529_u64,14538053226114775500_u64,3204916589808301624_u64,12999868964213327070_u64,12046523827143859882_u64,5678558764367860138_u64];
RET = [34289_u16];
RET = [21136_u16];
_19 = Field::<isize>(Variant(_5.1, 0), 2) & Field::<isize>(Variant(_5.1, 0), 2);
(*_14) = [4880793545989292732_u64,11790622086971221718_u64,16385858832113195604_u64,6607365971099134036_u64,5051415434261228062_u64,12241201362819237182_u64];
_16 = _12.1;
_17 = Field::<usize>(Variant(_5.1, 0), 3) + Field::<usize>(Variant(_5.1, 0), 3);
Goto(bb5)
}
bb5 = {
_5.1 = Adt17::Variant1 { fld0: 127554453555560743127981432871977015475_i128,fld1: 7291848653384612250_u64,fld2: _10 };
(*_14) = [18252882116820538091_u64,13221879895663445067_u64,5639532697632659157_u64,9095745186050921964_u64,11149454248968603157_u64,10388505535149593742_u64];
(*_14) = [4052478383441717834_u64,7046663968669696014_u64,8355932705672226736_u64,581462661626358430_u64,9867404615869479207_u64,6678848159242509720_u64];
_14 = Move(_4);
_2 = _16 == _12.1;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = 38585554_u32 as f32;
_25 = !82_i8;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = _10 - _10;
Goto(bb6)
}
bb6 = {
_27 = -(-3843212574367796951_i64);
_2 = true;
_3 = 158235452846570724803469866440401968161_i128 as f64;
_11 = _13 * _13;
_2 = true;
_13 = _11 | _11;
_3 = 11981410874796313652_u64 as f64;
RET = [21782_u16];
_27 = (-55999767_i32) as i64;
_10 = -Field::<f32>(Variant(_5.1, 1), 2);
_2 = false & false;
place!(Field::<i128>(Variant(_5.1, 1), 0)) = (-122852316360964711324762162420871295889_i128) ^ (-82590123947960767313466656603710074897_i128);
_28.1 = Adt17::Variant0 { fld0: 1489722102_u32,fld1: _18,fld2: _19,fld3: _17,fld4: _13,fld5: 133_u8,fld6: 79203529835753353299364817760090485037_u128 };
_28.1 = Adt17::Variant2 { fld0: _3,fld1: 38179_u16,fld2: 157_u8 };
Goto(bb7)
}
bb7 = {
place!(Field::<u64>(Variant(_5.1, 1), 1)) = 16610964459981649583_u64 << _11;
place!(Field::<f64>(Variant(_28.1, 2), 0)) = _3 - _3;
place!(Field::<u8>(Variant(_28.1, 2), 2)) = 69_u8;
_7 = &place!(Field::<f32>(Variant(_5.1, 1), 2));
_23 = _18;
_12.3 = Adt19::Variant2 { fld0: (-1234915143_i32),fld1: _17,fld2: Field::<u8>(Variant(_28.1, 2), 2),fld3: _25,fld4: Field::<i128>(Variant(_5.1, 1), 0) };
place!(Field::<u16>(Variant(_28.1, 2), 1)) = 35279_u16 + 3027_u16;
_28.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_28.1, 2), 1)));
_26 = _16 & _12.1;
place!(Field::<f64>(Variant(_28.1, 2), 0)) = _3 - _3;
_29 = _16;
_13 = _11 >> _17;
_30 = core::ptr::addr_of!(_27);
_28 = (Move(_12.0), _5.1);
place!(Field::<i128>(Variant(_12.3, 2), 4)) = -Field::<i128>(Variant(_28.1, 1), 0);
_12.3 = Adt19::Variant3 { fld0: _2,fld1: _17,fld2: 4163519748_u32,fld3: _25,fld4: Move(_30),fld5: 258370061_i32,fld6: (*_30),fld7: 37631_u16 };
_12.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_12.3, 3), 7)));
place!(Field::<i32>(Variant(_12.3, 3), 5)) = !(-781674404_i32);
place!(Field::<*const i64>(Variant(_12.3, 3), 4)) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_12.3, 3), 6)));
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<u64>(Variant(_28.1, 1), 1) ^ Field::<u64>(Variant(_28.1, 1), 1);
place!(Field::<u32>(Variant(_12.3, 3), 2)) = 1432802956_u32 << Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<f32>(Variant(_28.1, 1), 2)) = -Field::<f32>(Variant(_5.1, 1), 2);
Call(_27 = core::intrinsics::transmute(Field::<i64>(Variant(_12.3, 3), 6)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = !Field::<usize>(Variant(_12.3, 3), 1);
_2 = Field::<bool>(Variant(_12.3, 3), 0);
_29 = !_12.1;
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<i32>(Variant(_12.3, 3), 5) as u64;
place!(Field::<u64>(Variant(_28.1, 1), 1)) = 64222_u16 as u64;
_19 = _29 + _26;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = -_10;
_31.3.fld0 = !129224972459203680160367576527255507261_u128;
RET = [51464_u16];
_28.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_12.3, 3), 7)));
place!(Field::<u64>(Variant(_28.1, 1), 1)) = Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<usize>(Variant(_12.3, 3), 1)) = _17;
_26 = _10 as isize;
_31.0 = core::ptr::addr_of!(_21);
_31.1 = [Field::<u64>(Variant(_5.1, 1), 1),Field::<u64>(Variant(_28.1, 1), 1),Field::<u64>(Variant(_5.1, 1), 1),Field::<u64>(Variant(_5.1, 1), 1),Field::<u64>(Variant(_5.1, 1), 1),Field::<u64>(Variant(_5.1, 1), 1)];
_12.1 = _19 >> _29;
place!(Field::<u16>(Variant(_12.3, 3), 7)) = _10 as u16;
place!(Field::<i8>(Variant(_12.3, 3), 3)) = !_25;
_18 = _23;
_28.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_12.3, 3), 7)));
_22 = _3;
_31.2 = core::ptr::addr_of!(_12.1);
place!(Field::<i32>(Variant(_12.3, 3), 5)) = _25 as i32;
place!(Field::<i8>(Variant(_12.3, 3), 3)) = -_25;
_18 = _23;
_22 = _3;
Goto(bb9)
}
bb9 = {
place!(Field::<f32>(Variant(_5.1, 1), 2)) = _12.1 as f32;
_33 = &_31.2;
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<i32>(Variant(_12.3, 3), 5) as u64;
_8 = &mut _31.3.fld0;
_32.3.0 = Field::<u32>(Variant(_12.3, 3), 2) ^ Field::<u32>(Variant(_12.3, 3), 2);
place!(Field::<i64>(Variant(_12.3, 3), 6)) = _27 | _27;
_16 = Field::<i32>(Variant(_12.3, 3), 5) as isize;
_12.2 = RET;
_22 = -_3;
_22 = _3;
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<u64>(Variant(_28.1, 1), 1) ^ Field::<u64>(Variant(_28.1, 1), 1);
_17 = !Field::<usize>(Variant(_12.3, 3), 1);
_5 = (Move(_28.0), _28.1);
_36 = _29;
(*_8) = 37706178393802898802441230797095910237_u128;
_28.0 = Move(_12.0);
(*_8) = !332790222871249300883064397568974774923_u128;
_28 = (Move(_5.0), _5.1);
place!(Field::<i128>(Variant(_28.1, 1), 0)) = !Field::<i128>(Variant(_5.1, 1), 0);
_26 = _12.1 + _12.1;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_12.3, 3), 7)));
(*_8) = 79630272031023640993866061279487027979_u128 & 183879011693447052228391164191072017411_u128;
Goto(bb10)
}
bb10 = {
_16 = _12.1;
_3 = Field::<u64>(Variant(_28.1, 1), 1) as f64;
_32.3.2 = _28.1;
_32.3.3 = Field::<i64>(Variant(_12.3, 3), 6) as f32;
(*_8) = 172386952184308112679003356157675705942_u128 >> _32.3.0;
RET = [Field::<u16>(Variant(_12.3, 3), 7)];
_32.3.0 = Field::<u32>(Variant(_12.3, 3), 2) + Field::<u32>(Variant(_12.3, 3), 2);
_34 = Field::<i32>(Variant(_12.3, 3), 5) - Field::<i32>(Variant(_12.3, 3), 5);
_32.1 = Adt19::Variant1 { fld0: Move(Field::<*const i64>(Variant(_12.3, 3), 4)) };
_17 = Field::<i64>(Variant(_12.3, 3), 6) as usize;
_12 = (Move(_28.0), _19, RET, Move(_32.1));
_12.0 = Move(_5.0);
_28.0 = Move(_12.0);
_22 = -_3;
(*_8) = 6978787398604434703445526562244322972_u128 * 96864765859861844968919193463940258034_u128;
(*_8) = _32.3.0 as u128;
_13 = _11 - _11;
(*_8) = 46392904244682640237200071317636420327_u128 & 180492048820671342699324923591336833877_u128;
_20 = &(*_33);
_17 = 1_usize * 2561526427461492378_usize;
(*_8) = !160974871105379245351577942703015649988_u128;
Goto(bb11)
}
bb11 = {
_32.1 = Adt19::Variant3 { fld0: _2,fld1: _17,fld2: _32.3.0,fld3: _25,fld4: Move(Field::<*const i64>(Variant(_12.3, 1), 0)),fld5: _34,fld6: _27,fld7: 593_u16 };
place!(Field::<*const i64>(Variant(_12.3, 1), 0)) = core::ptr::addr_of!(_27);
_32.2 = Field::<i32>(Variant(_32.1, 3), 5) ^ Field::<i32>(Variant(_32.1, 3), 5);
_17 = !Field::<usize>(Variant(_32.1, 3), 1);
(*_8) = 253994706031036506391352606839828425492_u128 * 57605245545193980085139995555101360094_u128;
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<i32>(Variant(_32.1, 3), 5) as u64;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
place!(Field::<u16>(Variant(_32.1, 3), 7)) = 33435_u16 | 14487_u16;
_12.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
_32.3.1 = !Field::<u32>(Variant(_32.1, 3), 2);
_22 = _3 - _3;
_11 = Field::<f32>(Variant(_28.1, 1), 2) as i16;
_21.0 = &_23;
place!(Field::<f32>(Variant(_28.1, 1), 2)) = _10 * _32.3.3;
_2 = _32.3.0 >= Field::<u32>(Variant(_32.1, 3), 2);
_32.3 = (Field::<u32>(Variant(_32.1, 3), 2), Field::<u32>(Variant(_32.1, 3), 2), _5.1, Field::<f32>(Variant(_5.1, 1), 2));
(*_8) = 278146692766063811761964959401938576530_u128 - 90291905763001869428215875755691186852_u128;
(*_8) = 88013609234476493882854218818199679295_u128 & 297945344143891566483128422019089304587_u128;
(*_8) = 211358899049189521447582746730567856602_u128 + 157438271037337155225339889010104888650_u128;
place!(Field::<i128>(Variant(_5.1, 1), 0)) = _2 as i128;
_32.3.3 = -_10;
(*_8) = Field::<i128>(Variant(_5.1, 1), 0) as u128;
Goto(bb12)
}
bb12 = {
place!(Field::<u32>(Variant(_32.1, 3), 2)) = _32.3.0 - _32.3.1;
(*_8) = _12.1 as u128;
(*_8) = 13073355409346244071775622882478237178_u128 - 275963547260935570510770940564813053981_u128;
_17 = _32.2 as usize;
place!(Field::<usize>(Variant(_32.1, 3), 1)) = _17 >> _32.3.0;
_17 = Field::<usize>(Variant(_32.1, 3), 1) >> _19;
_34 = _32.2;
_36 = _26 | _16;
place!(Field::<i64>(Variant(_32.1, 3), 6)) = Field::<u64>(Variant(_5.1, 1), 1) as i64;
(*_8) = 18000816337965075201960991261987459505_u128 | 307328251919287569318258585934091541637_u128;
(*_8) = 138645011472226694474906937979993973664_u128 + 202713385011171123508148226093140747694_u128;
_15 = core::ptr::addr_of_mut!(_43);
(*_15) = core::ptr::addr_of_mut!(_22);
_7 = &_10;
_12.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
(*_8) = !238669985762887790691106157766258834974_u128;
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_8) = 250255062376670640078682441823659767985_u128;
_28 = (Move(_5.0), _5.1);
_35 = !Field::<u64>(Variant(_28.1, 1), 1);
place!(Field::<u64>(Variant(_5.1, 1), 1)) = _35 << _36;
(*_8) = _25 as u128;
Goto(bb13)
}
bb13 = {
_40 = 207_u8 & 241_u8;
_44 = core::ptr::addr_of!(_28.1);
_32.3.0 = !_32.3.1;
_51.0 = &mut _17;
(*_43) = -_3;
(*_43) = _3;
(*_44) = Adt17::Variant0 { fld0: Field::<u32>(Variant(_32.1, 3), 2),fld1: _18,fld2: _12.1,fld3: Field::<usize>(Variant(_32.1, 3), 1),fld4: _13,fld5: _40,fld6: (*_8) };
_35 = Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 - _40;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) - _32.3.0;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) | _32.3.1;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
_36 = Field::<isize>(Variant((*_44), 0), 2) * Field::<isize>(Variant((*_44), 0), 2);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
(*_44) = _5.1;
_57.2.fld0 = _26 + _26;
Goto(bb14)
}
bb14 = {
(*_43) = -_3;
_58.0 = Move(_21.0);
_20 = Move(_33);
place!(Field::<u64>(Variant(_32.3.2, 1), 1)) = !Field::<u64>(Variant((*_44), 1), 1);
_54 = -Field::<f32>(Variant((*_44), 1), 2);
Call(_57.2.fld0 = core::intrinsics::transmute(Field::<u64>(Variant(_28.1, 1), 1)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_44) = Adt17::Variant0 { fld0: Field::<u32>(Variant(_32.1, 3), 2),fld1: _23,fld2: _57.2.fld0,fld3: Field::<usize>(Variant(_32.1, 3), 1),fld4: _13,fld5: _40,fld6: (*_8) };
place!(Field::<u64>(Variant(_32.3.2, 1), 1)) = Field::<u64>(Variant(_5.1, 1), 1) ^ _35;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8) ^ (*_8);
Call(place!(Field::<usize>(Variant((*_44), 0), 3)) = core::intrinsics::transmute(_27), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*_8) = Field::<u128>(Variant(_28.1, 0), 6) ^ Field::<u128>(Variant((*_44), 0), 6);
place!(Field::<isize>(Variant((*_44), 0), 2)) = _36;
(*_8) = !Field::<u128>(Variant((*_44), 0), 6);
(*_44) = _32.3.2;
_36 = _57.2.fld0 ^ _57.2.fld0;
place!(Field::<*const i64>(Variant(_12.3, 1), 0)) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_32.1, 3), 6)));
_57.3 = Field::<usize>(Variant(_32.1, 3), 1);
_39 = &_32.2;
_12.2 = [Field::<u16>(Variant(_32.1, 3), 7)];
Goto(bb17)
}
bb17 = {
_47 = _2;
(*_43) = _3;
(*_44) = _5.1;
(*_43) = -_3;
place!(Field::<i128>(Variant((*_44), 1), 0)) = Field::<u64>(Variant((*_44), 1), 1) as i128;
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_8) as f32;
(*_43) = Field::<i128>(Variant(_28.1, 1), 0) as f64;
(*_44) = _5.1;
(*_43) = _3;
place!(Field::<u64>(Variant((*_44), 1), 1)) = (*_43) as u64;
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) * (*_7);
place!(Field::<u64>(Variant((*_44), 1), 1)) = !Field::<u64>(Variant(_32.3.2, 1), 1);
place!(Field::<i128>(Variant((*_44), 1), 0)) = _13 as i128;
place!(Field::<*const i64>(Variant(_12.3, 1), 0)) = Move(Field::<*const i64>(Variant(_32.1, 3), 4));
_28.1 = _32.3.2;
Goto(bb18)
}
bb18 = {
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) - (*_7);
(*_43) = _3;
_28.1 = Adt17::Variant2 { fld0: _3,fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
_56 = [Field::<u16>(Variant((*_44), 2), 1)];
(*_44) = Adt17::Variant2 { fld0: (*_43),fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
(*_15) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_44), 2), 0)));
place!(Field::<u16>(Variant((*_44), 2), 1)) = Field::<u16>(Variant(_32.1, 3), 7) | Field::<u16>(Variant(_32.1, 3), 7);
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_5.1, 1), 0),fld1: Field::<u64>(Variant(_32.3.2, 1), 1),fld2: (*_7) };
(*_8) = 155770473797462877307981647322155258258_u128 & 168269793160497857135371925798499000283_u128;
_43 = core::ptr::addr_of_mut!(_3);
Goto(bb19)
}
bb19 = {
_42 = _40 * _40;
(*_43) = _22 - _22;
(*_43) = _22;
_57.3 = Field::<usize>(Variant(_32.1, 3), 1) | Field::<usize>(Variant(_32.1, 3), 1);
_54 = _32.2 as f32;
_3 = -_22;
_32.3.0 = (*_39) as u32;
_66 = Move(_28);
_66 = Move(_5);
_39 = &place!(Field::<i32>(Variant(_32.1, 3), 5));
(*_8) = 33122295933595110170975220161555977499_u128;
_52 = &mut place!(Field::<*const i64>(Variant(_12.3, 1), 0));
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_66.1, 1), 0),fld1: _35,fld2: (*_7) };
(*_52) = core::ptr::addr_of!(_27);
_32.3.2 = (*_44);
_45 = &(*_7);
(*_43) = _22 * _22;
_57.0 = &mut _23;
_57.2.fld1 = core::ptr::addr_of_mut!((*_43));
place!(Field::<i128>(Variant((*_44), 1), 0)) = Field::<i128>(Variant(_32.3.2, 1), 0);
place!(Field::<f32>(Variant((*_44), 1), 2)) = -(*_45);
_36 = _19;
(*_15) = core::ptr::addr_of_mut!((*_43));
_27 = !Field::<i64>(Variant(_32.1, 3), 6);
match (*_8) {
0 => bb5,
1 => bb15,
2 => bb7,
33122295933595110170975220161555977499 => bb20,
_ => bb8
}
}
bb20 = {
_38 = _25;
(*_43) = _38 as f64;
_57.0 = &mut _18;
place!(Field::<u64>(Variant(_66.1, 1), 1)) = Field::<u64>(Variant((*_44), 1), 1) + Field::<u64>(Variant((*_44), 1), 1);
_29 = _57.2.fld0;
(*_44) = _32.3.2;
_5.0 = Move(_66.0);
(*_8) = _47 as u128;
_25 = Field::<i8>(Variant(_32.1, 3), 3) * Field::<i8>(Variant(_32.1, 3), 3);
place!(Field::<i128>(Variant((*_44), 1), 0)) = Field::<i128>(Variant(_32.3.2, 1), 0);
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_32.3.2, 1), 0),fld1: Field::<u64>(Variant(_66.1, 1), 1),fld2: (*_7) };
place!(Field::<u64>(Variant((*_44), 1), 1)) = Field::<u64>(Variant(_66.1, 1), 1) | Field::<u64>(Variant(_66.1, 1), 1);
_9 = core::ptr::addr_of_mut!(_64);
_7 = &_32.3.3;
(*_43) = -_22;
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) + (*_45);
Goto(bb21)
}
bb21 = {
place!(Field::<u64>(Variant(_66.1, 1), 1)) = !Field::<u64>(Variant((*_44), 1), 1);
_22 = (*_43) - (*_43);
_66 = (Move(_5.0), (*_44));
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_8) = 109853216466138591073687801614142725466_u128 - 276149711221270300263462973339471561089_u128;
_60 = core::ptr::addr_of_mut!((*_9));
(*_9) = ['\u{7bd4}','\u{9908c}','\u{903e}','\u{778c2}','\u{ad74a}','\u{208ad}'];
(*_15) = core::ptr::addr_of_mut!((*_43));
_5.1 = (*_44);
_63 = core::ptr::addr_of_mut!(_2);
(*_44) = _5.1;
(*_43) = _22 * _22;
_9 = core::ptr::addr_of_mut!((*_9));
(*_44) = Adt17::Variant0 { fld0: _32.3.1,fld1: '\u{37197}',fld2: _26,fld3: _57.3,fld4: _13,fld5: _42,fld6: (*_8) };
_43 = core::ptr::addr_of_mut!((*_43));
place!(Field::<usize>(Variant((*_44), 0), 3)) = (*_63) as usize;
_25 = Field::<u128>(Variant((*_44), 0), 6) as i8;
(*_52) = core::ptr::addr_of!(_27);
place!(Field::<char>(Variant((*_44), 0), 1)) = '\u{89905}';
_39 = &_32.2;
place!(Field::<i16>(Variant((*_44), 0), 4)) = _13 + _11;
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 + _40;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) - Field::<u32>(Variant(_32.1, 3), 2);
place!(Field::<isize>(Variant((*_44), 0), 2)) = _57.2.fld0 - _57.2.fld0;
(*_15) = core::ptr::addr_of_mut!(_22);
Goto(bb22)
}
bb22 = {
place!(Field::<usize>(Variant((*_44), 0), 3)) = Field::<isize>(Variant((*_44), 0), 2) as usize;
(*_44) = _32.3.2;
_51.0 = &mut place!(Field::<usize>(Variant(_32.1, 3), 1));
_59 = '\u{70138}';
_69 = core::ptr::addr_of!(_57.2.fld0);
(*_52) = core::ptr::addr_of!(_27);
Goto(bb23)
}
bb23 = {
place!(Field::<f32>(Variant((*_44), 1), 2)) = -(*_45);
(*_8) = 291413758584254345839898086176254292511_u128 | 191045639282011658188795536246018040086_u128;
(*_44) = Adt17::Variant0 { fld0: 1384698193_u32,fld1: _59,fld2: (*_69),fld3: _57.3,fld4: _13,fld5: _40,fld6: (*_8) };
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),_59,Field::<char>(Variant(_28.1, 0), 1),Field::<char>(Variant((*_44), 0), 1)];
_78 = -Field::<i128>(Variant(_66.1, 1), 0);
_7 = &(*_45);
_22 = _3;
place!(Field::<usize>(Variant((*_44), 0), 3)) = (*_63) as usize;
(*_8) = Field::<i16>(Variant((*_44), 0), 4) as u128;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8) << (*_8);
_5.0 = core::ptr::addr_of_mut!(_46);
place!(Field::<u32>(Variant((*_44), 0), 0)) = !3209758589_u32;
(*_9) = [Field::<char>(Variant(_28.1, 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
place!(Field::<u32>(Variant((*_44), 0), 0)) = (*_63) as u32;
_51.1 = Move(_5);
(*_52) = core::ptr::addr_of!(_27);
place!(Field::<u128>(Variant((*_44), 0), 6)) = !(*_8);
Goto(bb24)
}
bb24 = {
place!(Field::<char>(Variant((*_44), 0), 1)) = _59;
place!(Field::<isize>(Variant(_28.1, 0), 2)) = (*_69) >> _16;
(*_52) = core::ptr::addr_of!(_27);
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8) | (*_8);
place!(Field::<i16>(Variant((*_44), 0), 4)) = _11;
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
_3 = Field::<u8>(Variant((*_44), 0), 5) as f64;
_56 = [65159_u16];
_20 = &_69;
_46 = 4221_u16;
place!(Field::<char>(Variant((*_44), 0), 1)) = _59;
place!(Field::<u128>(Variant((*_44), 0), 6)) = Field::<u32>(Variant((*_44), 0), 0) as u128;
_66.0 = core::ptr::addr_of_mut!(_46);
_51.2.0 = core::ptr::addr_of_mut!((*_43));
_29 = (*_69) >> Field::<u128>(Variant((*_44), 0), 6);
(*_43) = _3;
_75 = Adt34 { fld0: Field::<char>(Variant((*_44), 0), 1) };
match _46 {
4221 => bb25,
_ => bb1
}
}
bb25 = {
place!(Field::<u32>(Variant((*_44), 0), 0)) = 2728318335_u32 >> (*_69);
_87 = &mut place!(Field::<usize>(Variant((*_44), 0), 3));
place!(Field::<isize>(Variant((*_44), 0), 2)) = _26 << Field::<u32>(Variant((*_44), 0), 0);
(*_43) = _3 * _3;
place!(Field::<isize>(Variant((*_44), 0), 2)) = _29;
place!(Field::<i16>(Variant((*_44), 0), 4)) = (*_39) as i16;
(*_52) = core::ptr::addr_of!(_27);
_57.2 = Adt40 { fld0: Field::<isize>(Variant((*_44), 0), 2),fld1: Move((*_15)) };
place!(Field::<i16>(Variant((*_44), 0), 4)) = _11;
place!(Field::<u32>(Variant((*_44), 0), 0)) = (*_8) as u32;
(*_87) = !_57.3;
_50 = [_25,_38,_38,_25,_25,_38];
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_69) = Field::<isize>(Variant((*_44), 0), 2) ^ Field::<isize>(Variant((*_44), 0), 2);
_47 = (*_63) ^ (*_63);
place!(Field::<isize>(Variant((*_44), 0), 2)) = Field::<i128>(Variant(_51.1.1, 1), 0) as isize;
_19 = Field::<u64>(Variant(_66.1, 1), 1) as isize;
match _46 {
0 => bb10,
1 => bb11,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
4221 => bb32,
_ => bb31
}
}
bb26 = {
_40 = 207_u8 & 241_u8;
_44 = core::ptr::addr_of!(_28.1);
_32.3.0 = !_32.3.1;
_51.0 = &mut _17;
(*_43) = -_3;
(*_43) = _3;
(*_44) = Adt17::Variant0 { fld0: Field::<u32>(Variant(_32.1, 3), 2),fld1: _18,fld2: _12.1,fld3: Field::<usize>(Variant(_32.1, 3), 1),fld4: _13,fld5: _40,fld6: (*_8) };
_35 = Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 - _40;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) - _32.3.0;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) | _32.3.1;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
_36 = Field::<isize>(Variant((*_44), 0), 2) * Field::<isize>(Variant((*_44), 0), 2);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
(*_44) = _5.1;
_57.2.fld0 = _26 + _26;
Goto(bb14)
}
bb27 = {
_16 = _12.1;
_3 = Field::<u64>(Variant(_28.1, 1), 1) as f64;
_32.3.2 = _28.1;
_32.3.3 = Field::<i64>(Variant(_12.3, 3), 6) as f32;
(*_8) = 172386952184308112679003356157675705942_u128 >> _32.3.0;
RET = [Field::<u16>(Variant(_12.3, 3), 7)];
_32.3.0 = Field::<u32>(Variant(_12.3, 3), 2) + Field::<u32>(Variant(_12.3, 3), 2);
_34 = Field::<i32>(Variant(_12.3, 3), 5) - Field::<i32>(Variant(_12.3, 3), 5);
_32.1 = Adt19::Variant1 { fld0: Move(Field::<*const i64>(Variant(_12.3, 3), 4)) };
_17 = Field::<i64>(Variant(_12.3, 3), 6) as usize;
_12 = (Move(_28.0), _19, RET, Move(_32.1));
_12.0 = Move(_5.0);
_28.0 = Move(_12.0);
_22 = -_3;
(*_8) = 6978787398604434703445526562244322972_u128 * 96864765859861844968919193463940258034_u128;
(*_8) = _32.3.0 as u128;
_13 = _11 - _11;
(*_8) = 46392904244682640237200071317636420327_u128 & 180492048820671342699324923591336833877_u128;
_20 = &(*_33);
_17 = 1_usize * 2561526427461492378_usize;
(*_8) = !160974871105379245351577942703015649988_u128;
Goto(bb11)
}
bb28 = {
place!(Field::<usize>(Variant((*_44), 0), 3)) = Field::<isize>(Variant((*_44), 0), 2) as usize;
(*_44) = _32.3.2;
_51.0 = &mut place!(Field::<usize>(Variant(_32.1, 3), 1));
_59 = '\u{70138}';
_69 = core::ptr::addr_of!(_57.2.fld0);
(*_52) = core::ptr::addr_of!(_27);
Goto(bb23)
}
bb29 = {
_5.1 = Adt17::Variant1 { fld0: 127554453555560743127981432871977015475_i128,fld1: 7291848653384612250_u64,fld2: _10 };
(*_14) = [18252882116820538091_u64,13221879895663445067_u64,5639532697632659157_u64,9095745186050921964_u64,11149454248968603157_u64,10388505535149593742_u64];
(*_14) = [4052478383441717834_u64,7046663968669696014_u64,8355932705672226736_u64,581462661626358430_u64,9867404615869479207_u64,6678848159242509720_u64];
_14 = Move(_4);
_2 = _16 == _12.1;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = 38585554_u32 as f32;
_25 = !82_i8;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = _10 - _10;
Goto(bb6)
}
bb30 = {
_38 = _25;
(*_43) = _38 as f64;
_57.0 = &mut _18;
place!(Field::<u64>(Variant(_66.1, 1), 1)) = Field::<u64>(Variant((*_44), 1), 1) + Field::<u64>(Variant((*_44), 1), 1);
_29 = _57.2.fld0;
(*_44) = _32.3.2;
_5.0 = Move(_66.0);
(*_8) = _47 as u128;
_25 = Field::<i8>(Variant(_32.1, 3), 3) * Field::<i8>(Variant(_32.1, 3), 3);
place!(Field::<i128>(Variant((*_44), 1), 0)) = Field::<i128>(Variant(_32.3.2, 1), 0);
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_32.3.2, 1), 0),fld1: Field::<u64>(Variant(_66.1, 1), 1),fld2: (*_7) };
place!(Field::<u64>(Variant((*_44), 1), 1)) = Field::<u64>(Variant(_66.1, 1), 1) | Field::<u64>(Variant(_66.1, 1), 1);
_9 = core::ptr::addr_of_mut!(_64);
_7 = &_32.3.3;
(*_43) = -_22;
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) + (*_45);
Goto(bb21)
}
bb31 = {
_42 = _40 * _40;
(*_43) = _22 - _22;
(*_43) = _22;
_57.3 = Field::<usize>(Variant(_32.1, 3), 1) | Field::<usize>(Variant(_32.1, 3), 1);
_54 = _32.2 as f32;
_3 = -_22;
_32.3.0 = (*_39) as u32;
_66 = Move(_28);
_66 = Move(_5);
_39 = &place!(Field::<i32>(Variant(_32.1, 3), 5));
(*_8) = 33122295933595110170975220161555977499_u128;
_52 = &mut place!(Field::<*const i64>(Variant(_12.3, 1), 0));
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_66.1, 1), 0),fld1: _35,fld2: (*_7) };
(*_52) = core::ptr::addr_of!(_27);
_32.3.2 = (*_44);
_45 = &(*_7);
(*_43) = _22 * _22;
_57.0 = &mut _23;
_57.2.fld1 = core::ptr::addr_of_mut!((*_43));
place!(Field::<i128>(Variant((*_44), 1), 0)) = Field::<i128>(Variant(_32.3.2, 1), 0);
place!(Field::<f32>(Variant((*_44), 1), 2)) = -(*_45);
_36 = _19;
(*_15) = core::ptr::addr_of_mut!((*_43));
_27 = !Field::<i64>(Variant(_32.1, 3), 6);
match (*_8) {
0 => bb5,
1 => bb15,
2 => bb7,
33122295933595110170975220161555977499 => bb20,
_ => bb8
}
}
bb32 = {
(*_63) = !_47;
place!(Field::<u8>(Variant((*_44), 0), 5)) = !_40;
(*_69) = Field::<isize>(Variant((*_44), 0), 2);
_77 = core::ptr::addr_of!(_22);
(*_87) = _57.3 << Field::<isize>(Variant((*_44), 0), 2);
Goto(bb33)
}
bb33 = {
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_63) = _47;
_19 = (*_69) ^ _29;
(*_87) = _57.3 << (*_69);
_30 = core::ptr::addr_of!(_27);
_54 = (*_7) + (*_7);
_57.2.fld1 = core::ptr::addr_of_mut!(_22);
_51.2.1 = &place!(Field::<char>(Variant((*_44), 0), 1));
(*_8) = Field::<u128>(Variant((*_44), 0), 6);
place!(Field::<i16>(Variant((*_44), 0), 4)) = Field::<isize>(Variant((*_44), 0), 2) as i16;
_83 = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
_51.0 = &mut (*_87);
place!(Field::<i16>(Variant((*_44), 0), 4)) = -_13;
match _46 {
0 => bb27,
1 => bb14,
2 => bb20,
3 => bb22,
4 => bb5,
4221 => bb34,
_ => bb21
}
}
bb34 = {
(*_69) = Field::<u32>(Variant((*_44), 0), 0) as isize;
_49 = _57.3 + _57.3;
place!(Field::<u32>(Variant((*_44), 0), 0)) = 3017455505_u32;
(*_30) = (-894511915449523799_i64) ^ 7077019347362417846_i64;
match Field::<u32>(Variant((*_44), 0), 0) {
0 => bb5,
1 => bb31,
2 => bb35,
3 => bb36,
4 => bb37,
3017455505 => bb39,
_ => bb38
}
}
bb35 = {
_40 = 207_u8 & 241_u8;
_44 = core::ptr::addr_of!(_28.1);
_32.3.0 = !_32.3.1;
_51.0 = &mut _17;
(*_43) = -_3;
(*_43) = _3;
(*_44) = Adt17::Variant0 { fld0: Field::<u32>(Variant(_32.1, 3), 2),fld1: _18,fld2: _12.1,fld3: Field::<usize>(Variant(_32.1, 3), 1),fld4: _13,fld5: _40,fld6: (*_8) };
_35 = Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 - _40;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) - _32.3.0;
place!(Field::<u32>(Variant((*_44), 0), 0)) = Field::<u32>(Variant(_32.1, 3), 2) | _32.3.1;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
_36 = Field::<isize>(Variant((*_44), 0), 2) * Field::<isize>(Variant((*_44), 0), 2);
place!(Field::<char>(Variant((*_44), 0), 1)) = _18;
(*_44) = _5.1;
_57.2.fld0 = _26 + _26;
Goto(bb14)
}
bb36 = {
place!(Field::<u32>(Variant(_32.1, 3), 2)) = _32.3.0 - _32.3.1;
(*_8) = _12.1 as u128;
(*_8) = 13073355409346244071775622882478237178_u128 - 275963547260935570510770940564813053981_u128;
_17 = _32.2 as usize;
place!(Field::<usize>(Variant(_32.1, 3), 1)) = _17 >> _32.3.0;
_17 = Field::<usize>(Variant(_32.1, 3), 1) >> _19;
_34 = _32.2;
_36 = _26 | _16;
place!(Field::<i64>(Variant(_32.1, 3), 6)) = Field::<u64>(Variant(_5.1, 1), 1) as i64;
(*_8) = 18000816337965075201960991261987459505_u128 | 307328251919287569318258585934091541637_u128;
(*_8) = 138645011472226694474906937979993973664_u128 + 202713385011171123508148226093140747694_u128;
_15 = core::ptr::addr_of_mut!(_43);
(*_15) = core::ptr::addr_of_mut!(_22);
_7 = &_10;
_12.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
(*_8) = !238669985762887790691106157766258834974_u128;
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_8) = 250255062376670640078682441823659767985_u128;
_28 = (Move(_5.0), _5.1);
_35 = !Field::<u64>(Variant(_28.1, 1), 1);
place!(Field::<u64>(Variant(_5.1, 1), 1)) = _35 << _36;
(*_8) = _25 as u128;
Goto(bb13)
}
bb37 = {
(*_43) = -_3;
_58.0 = Move(_21.0);
_20 = Move(_33);
place!(Field::<u64>(Variant(_32.3.2, 1), 1)) = !Field::<u64>(Variant((*_44), 1), 1);
_54 = -Field::<f32>(Variant((*_44), 1), 2);
Call(_57.2.fld0 = core::intrinsics::transmute(Field::<u64>(Variant(_28.1, 1), 1)), ReturnTo(bb15), UnwindUnreachable())
}
bb38 = {
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) - (*_7);
(*_43) = _3;
_28.1 = Adt17::Variant2 { fld0: _3,fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
_56 = [Field::<u16>(Variant((*_44), 2), 1)];
(*_44) = Adt17::Variant2 { fld0: (*_43),fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
(*_15) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_44), 2), 0)));
place!(Field::<u16>(Variant((*_44), 2), 1)) = Field::<u16>(Variant(_32.1, 3), 7) | Field::<u16>(Variant(_32.1, 3), 7);
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_5.1, 1), 0),fld1: Field::<u64>(Variant(_32.3.2, 1), 1),fld2: (*_7) };
(*_8) = 155770473797462877307981647322155258258_u128 & 168269793160497857135371925798499000283_u128;
_43 = core::ptr::addr_of_mut!(_3);
Goto(bb19)
}
bb39 = {
(*_15) = core::ptr::addr_of_mut!((*_77));
(*_30) = 7759505714845270020_i64 + 8711170324230083265_i64;
(*_15) = core::ptr::addr_of_mut!((*_43));
_88.0 = core::ptr::addr_of_mut!((*_43));
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_43) = _3 * _3;
place!(Field::<i16>(Variant((*_44), 0), 4)) = !_11;
place!(Field::<u32>(Variant((*_44), 0), 0)) = !2155484899_u32;
place!(Field::<i128>(Variant(_66.1, 1), 0)) = _78;
_9 = core::ptr::addr_of_mut!((*_9));
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_63) = _49 == _49;
(*_43) = _3;
_5.1 = _51.1.1;
Goto(bb40)
}
bb40 = {
_13 = (*_30) as i16;
(*_63) = Field::<isize>(Variant((*_44), 0), 2) <= Field::<isize>(Variant((*_44), 0), 2);
(*_63) = _47 ^ _47;
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_15) = core::ptr::addr_of_mut!((*_43));
(*_15) = core::ptr::addr_of_mut!((*_77));
(*_43) = _3 + _3;
(*_69) = _19 << (*_30);
(*_69) = Field::<isize>(Variant((*_44), 0), 2) | _19;
place!(Field::<char>(Variant((*_44), 0), 1)) = _59;
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
(*_77) = -_3;
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
place!(Field::<char>(Variant((*_44), 0), 1)) = _59;
_76 = &mut (*_52);
place!(Field::<i16>(Variant((*_44), 0), 4)) = _11 + _13;
(*_30) = !(-6460990036564219864_i64);
(*_63) = Field::<i16>(Variant((*_44), 0), 4) >= Field::<i16>(Variant((*_44), 0), 4);
place!(Field::<i16>(Variant((*_44), 0), 4)) = Field::<u32>(Variant((*_44), 0), 0) as i16;
_57.2 = Adt40 { fld0: Field::<isize>(Variant((*_44), 0), 2),fld1: Move(_43) };
match _46 {
0 => bb41,
4221 => bb43,
_ => bb42
}
}
bb41 = {
(*_63) = !_47;
place!(Field::<u8>(Variant((*_44), 0), 5)) = !_40;
(*_69) = Field::<isize>(Variant((*_44), 0), 2);
_77 = core::ptr::addr_of!(_22);
(*_87) = _57.3 << Field::<isize>(Variant((*_44), 0), 2);
Goto(bb33)
}
bb42 = {
place!(Field::<usize>(Variant((*_44), 0), 3)) = Field::<isize>(Variant((*_44), 0), 2) as usize;
(*_44) = _32.3.2;
_51.0 = &mut place!(Field::<usize>(Variant(_32.1, 3), 1));
_59 = '\u{70138}';
_69 = core::ptr::addr_of!(_57.2.fld0);
(*_52) = core::ptr::addr_of!(_27);
Goto(bb23)
}
bb43 = {
(*_15) = Move(_88.0);
place!(Field::<isize>(Variant((*_44), 0), 2)) = (*_69) | _26;
place!(Field::<i16>(Variant((*_44), 0), 4)) = _13 & _11;
(*_8) = Field::<u128>(Variant((*_44), 0), 6);
_16 = (*_69) * (*_69);
place!(Field::<u8>(Variant((*_44), 0), 5)) = _42;
place!(Field::<i16>(Variant((*_44), 0), 4)) = _13 + _11;
_40 = Field::<u8>(Variant((*_44), 0), 5);
place!(Field::<u32>(Variant((*_44), 0), 0)) = 742284867_u32 + 3968235285_u32;
_21 = (Move(_51.2.1),);
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 - _42;
_33 = &(*_20);
(*_9) = [Field::<char>(Variant((*_44), 0), 1),_59,Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
(*_63) = !_47;
(*_69) = (*_63) as isize;
_30 = core::ptr::addr_of!((*_30));
_61 = Field::<i128>(Variant(_51.1.1, 1), 0) as u32;
(*_76) = core::ptr::addr_of!((*_30));
(*_76) = core::ptr::addr_of!((*_30));
place!(Field::<u128>(Variant((*_44), 0), 6)) = Field::<isize>(Variant((*_44), 0), 2) as u128;
(*_30) = _57.2.fld0 as i64;
(*_30) = (-598655956252230738_i64) | (-586735891835844087_i64);
place!(Field::<isize>(Variant((*_44), 0), 2)) = _19 >> Field::<u128>(Variant((*_44), 0), 6);
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
place!(Field::<u8>(Variant((*_44), 0), 5)) = _42 >> (*_69);
place!(Field::<f32>(Variant(_5.1, 1), 2)) = (*_45) * (*_7);
Goto(bb44)
}
bb44 = {
_33 = Move(_20);
_57.3 = _49;
(*_30) = 8849339502881202171_i64 - (-6936590116232557361_i64);
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8);
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8);
place!(Field::<u32>(Variant((*_44), 0), 0)) = _61 >> _57.3;
_61 = !Field::<u32>(Variant((*_44), 0), 0);
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),_75.fld0];
_70 = _25 << Field::<isize>(Variant((*_44), 0), 2);
_84.2 = core::ptr::addr_of!(_19);
_51.2.1 = &place!(Field::<char>(Variant((*_44), 0), 1));
place!(Field::<u32>(Variant((*_44), 0), 0)) = _61;
match _46 {
0 => bb21,
1 => bb41,
2 => bb26,
3 => bb9,
4 => bb8,
5 => bb45,
4221 => bb47,
_ => bb46
}
}
bb45 = {
place!(Field::<u32>(Variant((*_44), 0), 0)) = 2728318335_u32 >> (*_69);
_87 = &mut place!(Field::<usize>(Variant((*_44), 0), 3));
place!(Field::<isize>(Variant((*_44), 0), 2)) = _26 << Field::<u32>(Variant((*_44), 0), 0);
(*_43) = _3 * _3;
place!(Field::<isize>(Variant((*_44), 0), 2)) = _29;
place!(Field::<i16>(Variant((*_44), 0), 4)) = (*_39) as i16;
(*_52) = core::ptr::addr_of!(_27);
_57.2 = Adt40 { fld0: Field::<isize>(Variant((*_44), 0), 2),fld1: Move((*_15)) };
place!(Field::<i16>(Variant((*_44), 0), 4)) = _11;
place!(Field::<u32>(Variant((*_44), 0), 0)) = (*_8) as u32;
(*_87) = !_57.3;
_50 = [_25,_38,_38,_25,_25,_38];
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_69) = Field::<isize>(Variant((*_44), 0), 2) ^ Field::<isize>(Variant((*_44), 0), 2);
_47 = (*_63) ^ (*_63);
place!(Field::<isize>(Variant((*_44), 0), 2)) = Field::<i128>(Variant(_51.1.1, 1), 0) as isize;
_19 = Field::<u64>(Variant(_66.1, 1), 1) as isize;
match _46 {
0 => bb10,
1 => bb11,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
4221 => bb32,
_ => bb31
}
}
bb46 = {
place!(Field::<usize>(Variant((*_44), 0), 3)) = Field::<isize>(Variant((*_44), 0), 2) as usize;
(*_44) = _32.3.2;
_51.0 = &mut place!(Field::<usize>(Variant(_32.1, 3), 1));
_59 = '\u{70138}';
_69 = core::ptr::addr_of!(_57.2.fld0);
(*_52) = core::ptr::addr_of!(_27);
Goto(bb23)
}
bb47 = {
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8) << Field::<isize>(Variant((*_44), 0), 2);
(*_76) = core::ptr::addr_of!((*_30));
place!(Field::<char>(Variant((*_44), 0), 1)) = _75.fld0;
(*_30) = (-8993107429964530367_i64) >> (*_8);
_58.0 = &place!(Field::<char>(Variant((*_44), 0), 1));
_61 = Field::<i128>(Variant(_51.1.1, 1), 0) as u32;
(*_69) = Field::<isize>(Variant((*_44), 0), 2) + Field::<isize>(Variant((*_44), 0), 2);
_55 = [_70,_70,_70,_70,_70,_70];
(*_30) = (-1561107531458907792_i64) & (-6893962827788387512_i64);
_59 = Field::<char>(Variant((*_44), 0), 1);
place!(Field::<u32>(Variant((*_44), 0), 0)) = !_61;
place!(Field::<f32>(Variant(_51.1.1, 1), 2)) = Field::<i16>(Variant((*_44), 0), 4) as f32;
match _46 {
0 => bb24,
1 => bb29,
2 => bb30,
3 => bb39,
4 => bb5,
4221 => bb48,
_ => bb6
}
}
bb48 = {
(*_8) = Field::<u128>(Variant((*_44), 0), 6);
_51.2 = (Move((*_15)), Move(_58.0));
_106 = Field::<i16>(Variant((*_44), 0), 4) as f32;
_21.0 = &place!(Field::<char>(Variant((*_44), 0), 1));
(*_15) = core::ptr::addr_of_mut!((*_77));
place!(Field::<isize>(Variant((*_44), 0), 2)) = (*_69) * (*_69);
_88 = (Move((*_15)), Move(_21.0));
place!(Field::<u8>(Variant((*_44), 0), 5)) = _40 * _42;
_99 = core::ptr::addr_of!(_53);
(*_63) = _57.2.fld0 >= Field::<isize>(Variant((*_44), 0), 2);
place!(Field::<u32>(Variant((*_44), 0), 0)) = _61 << Field::<u8>(Variant((*_44), 0), 5);
match _46 {
0 => bb49,
1 => bb50,
2 => bb51,
3 => bb52,
4221 => bb54,
_ => bb53
}
}
bb49 = {
place!(Field::<usize>(Variant((*_44), 0), 3)) = Field::<isize>(Variant((*_44), 0), 2) as usize;
(*_44) = _32.3.2;
_51.0 = &mut place!(Field::<usize>(Variant(_32.1, 3), 1));
_59 = '\u{70138}';
_69 = core::ptr::addr_of!(_57.2.fld0);
(*_52) = core::ptr::addr_of!(_27);
Goto(bb23)
}
bb50 = {
place!(Field::<f32>(Variant((*_44), 1), 2)) = -(*_45);
(*_8) = 291413758584254345839898086176254292511_u128 | 191045639282011658188795536246018040086_u128;
(*_44) = Adt17::Variant0 { fld0: 1384698193_u32,fld1: _59,fld2: (*_69),fld3: _57.3,fld4: _13,fld5: _40,fld6: (*_8) };
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),_59,Field::<char>(Variant(_28.1, 0), 1),Field::<char>(Variant((*_44), 0), 1)];
_78 = -Field::<i128>(Variant(_66.1, 1), 0);
_7 = &(*_45);
_22 = _3;
place!(Field::<usize>(Variant((*_44), 0), 3)) = (*_63) as usize;
(*_8) = Field::<i16>(Variant((*_44), 0), 4) as u128;
place!(Field::<u128>(Variant((*_44), 0), 6)) = (*_8) << (*_8);
_5.0 = core::ptr::addr_of_mut!(_46);
place!(Field::<u32>(Variant((*_44), 0), 0)) = !3209758589_u32;
(*_9) = [Field::<char>(Variant(_28.1, 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
place!(Field::<u32>(Variant((*_44), 0), 0)) = (*_63) as u32;
_51.1 = Move(_5);
(*_52) = core::ptr::addr_of!(_27);
place!(Field::<u128>(Variant((*_44), 0), 6)) = !(*_8);
Goto(bb24)
}
bb51 = {
_32.1 = Adt19::Variant3 { fld0: _2,fld1: _17,fld2: _32.3.0,fld3: _25,fld4: Move(Field::<*const i64>(Variant(_12.3, 1), 0)),fld5: _34,fld6: _27,fld7: 593_u16 };
place!(Field::<*const i64>(Variant(_12.3, 1), 0)) = core::ptr::addr_of!(_27);
_32.2 = Field::<i32>(Variant(_32.1, 3), 5) ^ Field::<i32>(Variant(_32.1, 3), 5);
_17 = !Field::<usize>(Variant(_32.1, 3), 1);
(*_8) = 253994706031036506391352606839828425492_u128 * 57605245545193980085139995555101360094_u128;
place!(Field::<u64>(Variant(_5.1, 1), 1)) = Field::<i32>(Variant(_32.1, 3), 5) as u64;
_5.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
place!(Field::<u16>(Variant(_32.1, 3), 7)) = 33435_u16 | 14487_u16;
_12.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_32.1, 3), 7)));
_32.3.1 = !Field::<u32>(Variant(_32.1, 3), 2);
_22 = _3 - _3;
_11 = Field::<f32>(Variant(_28.1, 1), 2) as i16;
_21.0 = &_23;
place!(Field::<f32>(Variant(_28.1, 1), 2)) = _10 * _32.3.3;
_2 = _32.3.0 >= Field::<u32>(Variant(_32.1, 3), 2);
_32.3 = (Field::<u32>(Variant(_32.1, 3), 2), Field::<u32>(Variant(_32.1, 3), 2), _5.1, Field::<f32>(Variant(_5.1, 1), 2));
(*_8) = 278146692766063811761964959401938576530_u128 - 90291905763001869428215875755691186852_u128;
(*_8) = 88013609234476493882854218818199679295_u128 & 297945344143891566483128422019089304587_u128;
(*_8) = 211358899049189521447582746730567856602_u128 + 157438271037337155225339889010104888650_u128;
place!(Field::<i128>(Variant(_5.1, 1), 0)) = _2 as i128;
_32.3.3 = -_10;
(*_8) = Field::<i128>(Variant(_5.1, 1), 0) as u128;
Goto(bb12)
}
bb52 = {
place!(Field::<f32>(Variant((*_44), 1), 2)) = (*_7) - (*_7);
(*_43) = _3;
_28.1 = Adt17::Variant2 { fld0: _3,fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
_56 = [Field::<u16>(Variant((*_44), 2), 1)];
(*_44) = Adt17::Variant2 { fld0: (*_43),fld1: Field::<u16>(Variant(_32.1, 3), 7),fld2: _40 };
(*_15) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_44), 2), 0)));
place!(Field::<u16>(Variant((*_44), 2), 1)) = Field::<u16>(Variant(_32.1, 3), 7) | Field::<u16>(Variant(_32.1, 3), 7);
(*_44) = Adt17::Variant1 { fld0: Field::<i128>(Variant(_5.1, 1), 0),fld1: Field::<u64>(Variant(_32.3.2, 1), 1),fld2: (*_7) };
(*_8) = 155770473797462877307981647322155258258_u128 & 168269793160497857135371925798499000283_u128;
_43 = core::ptr::addr_of_mut!(_3);
Goto(bb19)
}
bb53 = {
_5.1 = Adt17::Variant1 { fld0: 127554453555560743127981432871977015475_i128,fld1: 7291848653384612250_u64,fld2: _10 };
(*_14) = [18252882116820538091_u64,13221879895663445067_u64,5639532697632659157_u64,9095745186050921964_u64,11149454248968603157_u64,10388505535149593742_u64];
(*_14) = [4052478383441717834_u64,7046663968669696014_u64,8355932705672226736_u64,581462661626358430_u64,9867404615869479207_u64,6678848159242509720_u64];
_14 = Move(_4);
_2 = _16 == _12.1;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = 38585554_u32 as f32;
_25 = !82_i8;
place!(Field::<f32>(Variant(_5.1, 1), 2)) = _10 - _10;
Goto(bb6)
}
bb54 = {
_89 = (*_69) >> Field::<u64>(Variant(_5.1, 1), 1);
place!(Field::<isize>(Variant((*_44), 0), 2)) = !(*_69);
_84.3.fld0 = !Field::<u128>(Variant((*_44), 0), 6);
(*_8) = (*_30) as u128;
_75 = Adt34 { fld0: Field::<char>(Variant((*_44), 0), 1) };
place!(Field::<u128>(Variant((*_44), 0), 6)) = _84.3.fld0 | _84.3.fld0;
(*_30) = 3795783593868250577_i64 + (-93932999015798455_i64);
(*_9) = [Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1),_59,Field::<char>(Variant((*_44), 0), 1),Field::<char>(Variant((*_44), 0), 1)];
Goto(bb55)
}
bb55 = {
Call(_107 = dump_var(Move(_25), Move(_18), Move(_55), Move(_64)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_107 = dump_var(Move(_26), Move(_36), Move(_23), Move(_49)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_107 = dump_var(Move(_50), Move(_70), Move(_78), Move(_38)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_107 = dump_var(Move(_47), Move(_61), Move(_13), _108), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: [u16; 1]) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _3: u8;
let _4: bool;
let _5: isize;
let _6: char;
let _7: *const isize;
let _8: &'static f32;
let _9: &'static mut char;
let _10: *const i64;
let _11: &'static usize;
let _12: f32;
let _13: &'static f32;
let _14: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _15: &'static mut u128;
let _16: *mut bool;
let _17: &'static *mut bool;
let _18: char;
let _19: f64;
let _20: *const bool;
let _21: isize;
let _22: usize;
let _23: isize;
let _24: bool;
let _25: *mut (f64, &'static (u32, u32, Adt17, f32));
let _26: u32;
let _27: &'static Adt19;
let _28: f32;
let _29: Adt40;
let _30: (usize, bool);
let _31: *mut *const bool;
let _32: Adt60;
let _33: i128;
let _34: (*mut f64, &'static char);
let _35: &'static usize;
let _36: char;
let _37: char;
let _38: bool;
let _39: f64;
let _40: Adt17;
let _41: u32;
let _42: f64;
let _43: f32;
let _44: u128;
let _45: u16;
let _46: isize;
let _47: u16;
let _48: f64;
let _49: isize;
let _50: isize;
let _51: f64;
let _52: u128;
let _53: &'static mut [u64; 6];
let _54: (*mut f64, &'static char);
let _55: i8;
let _56: isize;
let _57: (*mut f64, &'static char);
let _58: (*mut u16, isize, [u16; 1], Adt19);
let _59: f64;
let _60: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _61: usize;
let _62: &'static *const isize;
let _63: (f64, &'static (u32, u32, Adt17, f32));
let _64: isize;
let _65: Adt34;
let _66: usize;
let _67: i16;
let _68: u16;
let _69: ();
let _70: ();
{
_2 = [778_u16];
Goto(bb1)
}
bb1 = {
RET = [2316622545747768863_u64,16389819573748296531_u64,808879256619719359_u64,2423986083878349908_u64,5771722224127893219_u64,14751832712093211654_u64];
_1 = true;
RET = [6471388821923836080_u64,3931692462341405975_u64,18326417718635209264_u64,15080537220822827780_u64,7294572345794750546_u64,8849584728117921708_u64];
_1 = true ^ false;
_3 = !212_u8;
Goto(bb2)
}
bb2 = {
RET = [10807428392329229664_u64,6385851864347418866_u64,1305192329400206286_u64,12737088052234085450_u64,13178965319551414528_u64,16143315557792719504_u64];
RET = [5841736846587907098_u64,10436531089093342026_u64,4896063042261183993_u64,5341450785885573198_u64,6921933702258166563_u64,1959055052110131770_u64];
_2 = [58165_u16];
_4 = _1 == _1;
_1 = _4 >= _4;
_3 = 226_u8 >> 8767783496715301822_i64;
_2 = [29878_u16];
_4 = _1;
_1 = _4 | _4;
_2 = [44658_u16];
_1 = _4 ^ _4;
RET = [7989738247255563796_u64,5220062031977632621_u64,4474666022583586287_u64,15814923324272362713_u64,6360982502314045977_u64,16487527295304881327_u64];
RET = [638221424692937223_u64,6811385004321111849_u64,15336531846731113302_u64,4469767012600200783_u64,3041841546566317933_u64,5859386825346382681_u64];
Call(_3 = fn15(_2, _4, _4, _4, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [18230174622157755616_u64,9544024858253637254_u64,15109853220686775732_u64,12149641117114326129_u64,6323749566519683480_u64,7280694546421817423_u64];
_1 = _4 & _4;
_3 = 234_u8 | 99_u8;
_4 = _1 <= _1;
_3 = 94123114995917579695043808107458541430_u128 as u8;
_4 = !_1;
_4 = _1;
_2 = [31010_u16];
Goto(bb4)
}
bb4 = {
_5 = -(-9223372036854775808_isize);
RET = [4616487265842067665_u64,6532754810810491065_u64,1958085169459835868_u64,4010484872746119301_u64,15190710129225025557_u64,3016046858844379467_u64];
_7 = core::ptr::addr_of!(_5);
(*_7) = (-27_isize) << _3;
(*_7) = -9223372036854775807_isize;
_7 = core::ptr::addr_of!((*_7));
(*_7) = (-8_isize);
(*_7) = (-108_isize) & 9223372036854775807_isize;
(*_7) = !(-9223372036854775808_isize);
_7 = core::ptr::addr_of!((*_7));
(*_7) = (-3_isize);
_3 = 107_u8 * 50_u8;
Goto(bb5)
}
bb5 = {
(*_7) = (-1440637803323570181_i64) as isize;
_6 = '\u{c7f17}';
(*_7) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_7) = 9223372036854775807_isize >> _3;
(*_7) = (-56_isize);
(*_7) = !(-9223372036854775808_isize);
(*_7) = (-9223372036854775808_isize) ^ (-9_isize);
(*_7) = (-19_isize);
(*_7) = !9223372036854775807_isize;
_4 = _1;
RET = [4841980106259457568_u64,13558589098784549160_u64,10011706445166882149_u64,4726501074209596073_u64,13590985156167810914_u64,3201542331021608378_u64];
(*_7) = 9223372036854775807_isize & (-9223372036854775808_isize);
(*_7) = !9223372036854775807_isize;
(*_7) = 9223372036854775807_isize;
RET = [1059307383966302549_u64,14565311683222149689_u64,6185921483340621097_u64,5031500455798366625_u64,6943332628120530642_u64,2176658682260598862_u64];
(*_7) = 9223372036854775807_isize;
(*_7) = 43327313468893780752086373955498372631_i128 as isize;
_5 = -95_isize;
(*_7) = -9223372036854775807_isize;
(*_7) = 2198841287991649132_i64 as isize;
(*_7) = (-9223372036854775808_isize);
(*_7) = 9223372036854775807_isize | (-9223372036854775808_isize);
_6 = '\u{bba07}';
(*_7) = (-145626878090690341873396194077705355707_i128) as isize;
(*_7) = 9223372036854775807_isize;
match (*_7) {
0 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb6 = {
RET = [10807428392329229664_u64,6385851864347418866_u64,1305192329400206286_u64,12737088052234085450_u64,13178965319551414528_u64,16143315557792719504_u64];
RET = [5841736846587907098_u64,10436531089093342026_u64,4896063042261183993_u64,5341450785885573198_u64,6921933702258166563_u64,1959055052110131770_u64];
_2 = [58165_u16];
_4 = _1 == _1;
_1 = _4 >= _4;
_3 = 226_u8 >> 8767783496715301822_i64;
_2 = [29878_u16];
_4 = _1;
_1 = _4 | _4;
_2 = [44658_u16];
_1 = _4 ^ _4;
RET = [7989738247255563796_u64,5220062031977632621_u64,4474666022583586287_u64,15814923324272362713_u64,6360982502314045977_u64,16487527295304881327_u64];
RET = [638221424692937223_u64,6811385004321111849_u64,15336531846731113302_u64,4469767012600200783_u64,3041841546566317933_u64,5859386825346382681_u64];
Call(_3 = fn15(_2, _4, _4, _4, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = [18230174622157755616_u64,9544024858253637254_u64,15109853220686775732_u64,12149641117114326129_u64,6323749566519683480_u64,7280694546421817423_u64];
_1 = _4 & _4;
_3 = 234_u8 | 99_u8;
_4 = _1 <= _1;
_3 = 94123114995917579695043808107458541430_u128 as u8;
_4 = !_1;
_4 = _1;
_2 = [31010_u16];
Goto(bb4)
}
bb8 = {
(*_7) = (-103_isize) + (-82_isize);
_6 = '\u{c57d0}';
_5 = (-88_isize) * 30_isize;
(*_7) = 12081_u16 as isize;
(*_7) = (-109_isize);
(*_7) = 75_isize;
(*_7) = -(-9223372036854775808_isize);
(*_7) = _3 as isize;
(*_7) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_7) = 56_isize - 1_isize;
(*_7) = -(-9223372036854775808_isize);
(*_7) = !9223372036854775807_isize;
(*_7) = -(-9223372036854775808_isize);
_4 = !_1;
(*_7) = 116_isize;
_2 = [40669_u16];
(*_7) = (-9223372036854775808_isize);
RET = [11473290115177131405_u64,13999428139878693113_u64,15681635163098451210_u64,15998978268317026521_u64,195437866797256203_u64,6367623274717257657_u64];
(*_7) = 39997_u16 as isize;
(*_7) = 9223372036854775807_isize | (-9223372036854775808_isize);
Call((*_7) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = 9223372036854775807_isize + 9223372036854775807_isize;
(*_7) = 9223372036854775807_isize + (-9223372036854775808_isize);
_4 = _1;
(*_7) = (-9223372036854775808_isize);
_5 = !117_isize;
(*_7) = (-124_isize) * 9223372036854775807_isize;
(*_7) = (-67_isize) | (-9223372036854775808_isize);
(*_7) = -57_isize;
(*_7) = -9223372036854775807_isize;
_9 = &mut _6;
_3 = 165_u8 + 223_u8;
(*_9) = '\u{e7dba}';
(*_7) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_7) = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_7 = core::ptr::addr_of!((*_7));
Goto(bb10)
}
bb10 = {
_14.2.fld0 = (*_9);
RET = [7387030838042867117_u64,8916361833049482018_u64,10903620521218651929_u64,11251580091899357740_u64,13612878983065482268_u64,13162684683915313473_u64];
(*_7) = !28_isize;
Goto(bb11)
}
bb11 = {
(*_9) = _14.2.fld0;
(*_9) = _14.2.fld0;
(*_7) = 9223372036854775807_isize >> _3;
(*_9) = _14.2.fld0;
(*_7) = (-60_isize) << _3;
(*_7) = 9223372036854775807_isize << _3;
_9 = &mut _14.2.fld0;
(*_9) = '\u{156c0}';
(*_9) = '\u{36aaf}';
(*_7) = (-132531753535461142769230527911776244087_i128) as isize;
(*_7) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_7) = (-127521822860890020836608657906606119630_i128) as isize;
(*_9) = '\u{38341}';
Goto(bb12)
}
bb12 = {
(*_9) = '\u{36857}';
_3 = 4549920667754623876_i64 as u8;
(*_7) = 9223372036854775807_isize & (-9223372036854775808_isize);
(*_7) = (-9223372036854775808_isize) & (-62_isize);
(*_9) = '\u{8b78f}';
_16 = core::ptr::addr_of_mut!(_4);
(*_9) = '\u{268b6}';
(*_16) = !_1;
(*_16) = (*_7) <= (*_7);
_3 = 53_u8 * 95_u8;
(*_9) = '\u{c2360}';
(*_9) = '\u{7a93e}';
(*_7) = 58611537619995037264594796067953640453_i128 as isize;
(*_16) = _1;
(*_16) = _1 ^ _1;
(*_7) = 9223372036854775807_isize;
(*_7) = (-17_isize) >> _3;
(*_7) = (-9223372036854775808_isize);
(*_9) = '\u{906b2}';
(*_16) = _1;
match (*_7) {
0 => bb11,
1 => bb13,
2 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
}
bb13 = {
(*_9) = _14.2.fld0;
(*_9) = _14.2.fld0;
(*_7) = 9223372036854775807_isize >> _3;
(*_9) = _14.2.fld0;
(*_7) = (-60_isize) << _3;
(*_7) = 9223372036854775807_isize << _3;
_9 = &mut _14.2.fld0;
(*_9) = '\u{156c0}';
(*_9) = '\u{36aaf}';
(*_7) = (-132531753535461142769230527911776244087_i128) as isize;
(*_7) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_7) = (-127521822860890020836608657906606119630_i128) as isize;
(*_9) = '\u{38341}';
Goto(bb12)
}
bb14 = {
(*_7) = (-103_isize) + (-82_isize);
_6 = '\u{c57d0}';
_5 = (-88_isize) * 30_isize;
(*_7) = 12081_u16 as isize;
(*_7) = (-109_isize);
(*_7) = 75_isize;
(*_7) = -(-9223372036854775808_isize);
(*_7) = _3 as isize;
(*_7) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_7) = 56_isize - 1_isize;
(*_7) = -(-9223372036854775808_isize);
(*_7) = !9223372036854775807_isize;
(*_7) = -(-9223372036854775808_isize);
_4 = !_1;
(*_7) = 116_isize;
_2 = [40669_u16];
(*_7) = (-9223372036854775808_isize);
RET = [11473290115177131405_u64,13999428139878693113_u64,15681635163098451210_u64,15998978268317026521_u64,195437866797256203_u64,6367623274717257657_u64];
(*_7) = 39997_u16 as isize;
(*_7) = 9223372036854775807_isize | (-9223372036854775808_isize);
Call((*_7) = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
RET = [2316622545747768863_u64,16389819573748296531_u64,808879256619719359_u64,2423986083878349908_u64,5771722224127893219_u64,14751832712093211654_u64];
_1 = true;
RET = [6471388821923836080_u64,3931692462341405975_u64,18326417718635209264_u64,15080537220822827780_u64,7294572345794750546_u64,8849584728117921708_u64];
_1 = true ^ false;
_3 = !212_u8;
Goto(bb2)
}
bb16 = {
(*_7) = (-8071111229543170788_i64) as isize;
(*_16) = _1 & _1;
(*_7) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
Goto(bb17)
}
bb17 = {
(*_7) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_16) = _1 ^ _1;
(*_16) = _1 ^ _1;
(*_16) = _1 & _1;
(*_7) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
(*_9) = '\u{348aa}';
(*_9) = '\u{e1feb}';
(*_7) = !(-9223372036854775808_isize);
(*_7) = -(-9223372036854775808_isize);
(*_16) = _5 != (*_7);
Goto(bb18)
}
bb18 = {
(*_16) = !_1;
(*_9) = '\u{bdc0b}';
(*_9) = '\u{10c7e9}';
_18 = (*_9);
(*_9) = _18;
(*_9) = _18;
(*_9) = _18;
_16 = core::ptr::addr_of_mut!((*_16));
(*_9) = _18;
(*_9) = _18;
_17 = &_16;
_19 = _3 as f64;
(*_16) = !_1;
_16 = core::ptr::addr_of_mut!((*_16));
_4 = _1 != _1;
(*_7) = 56_isize;
_22 = 0_usize + 0_usize;
(*_16) = (*_7) == (*_7);
_4 = _1 | _1;
_29.fld0 = (-160176037048498003198555509843296988837_i128) as isize;
_19 = (-1332_i16) as f64;
_24 = (*_16);
_1 = (*_16);
_21 = (*_7);
(*_9) = _18;
(*_16) = _1;
match (*_7) {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb9,
5 => bb16,
6 => bb19,
56 => bb21,
_ => bb20
}
}
bb19 = {
RET = [2316622545747768863_u64,16389819573748296531_u64,808879256619719359_u64,2423986083878349908_u64,5771722224127893219_u64,14751832712093211654_u64];
_1 = true;
RET = [6471388821923836080_u64,3931692462341405975_u64,18326417718635209264_u64,15080537220822827780_u64,7294572345794750546_u64,8849584728117921708_u64];
_1 = true ^ false;
_3 = !212_u8;
Goto(bb2)
}
bb20 = {
(*_9) = _14.2.fld0;
(*_9) = _14.2.fld0;
(*_7) = 9223372036854775807_isize >> _3;
(*_9) = _14.2.fld0;
(*_7) = (-60_isize) << _3;
(*_7) = 9223372036854775807_isize << _3;
_9 = &mut _14.2.fld0;
(*_9) = '\u{156c0}';
(*_9) = '\u{36aaf}';
(*_7) = (-132531753535461142769230527911776244087_i128) as isize;
(*_7) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_7) = (-127521822860890020836608657906606119630_i128) as isize;
(*_9) = '\u{38341}';
Goto(bb12)
}
bb21 = {
(*_9) = _18;
(*_7) = _21 * _21;
(*_9) = _18;
(*_16) = (*_7) > (*_7);
(*_7) = !_29.fld0;
_18 = (*_9);
(*_9) = _18;
(*_16) = _1;
(*_7) = !_21;
(*_16) = _24 | _24;
Goto(bb22)
}
bb22 = {
_19 = 11113_u16 as f64;
(*_9) = _18;
_7 = core::ptr::addr_of!((*_7));
_24 = (*_16) >= (*_16);
_12 = 11600_u16 as f32;
(*_16) = _24 < _24;
(*_7) = !_21;
_9 = &mut _18;
Goto(bb23)
}
bb23 = {
_23 = _5 ^ (*_7);
_30.1 = (*_16) != _4;
_13 = &_12;
(*_7) = _21;
(*_16) = !_24;
_33 = (-61676013460482880029745941708261895350_i128);
(*_7) = _23 & _21;
_30 = Checked(_22 - _22);
_28 = (*_13) * (*_13);
_12 = _28 + _28;
(*_7) = 1987707702_i32 as isize;
(*_7) = _23;
_7 = core::ptr::addr_of!((*_7));
_23 = 1977370662_u32 as isize;
(*_16) = _24 > _24;
(*_9) = '\u{e796f}';
(*_7) = _29.fld0;
(*_9) = '\u{3c1f2}';
(*_9) = '\u{1444c}';
(*_7) = 13939_i16 as isize;
_26 = _3 as u32;
_34.0 = core::ptr::addr_of_mut!(_19);
_34.0 = core::ptr::addr_of_mut!(_19);
Call(_21 = core::intrinsics::bswap((*_7)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_29 = Adt40 { fld0: (*_7),fld1: Move(_34.0) };
(*_9) = '\u{7f410}';
(*_9) = '\u{bd1a2}';
_20 = core::ptr::addr_of!((*_16));
(*_7) = (-685012036276181563_i64) as isize;
_23 = (-75_i8) as isize;
(*_7) = !_21;
(*_9) = '\u{4835e}';
(*_9) = '\u{9416f}';
(*_20) = _24 >= _24;
_8 = &_28;
(*_7) = 565648880_i32 as isize;
_28 = _12 * _12;
(*_9) = '\u{6b37}';
(*_20) = _24;
(*_9) = '\u{ea475}';
_30 = Checked(_22 * _22);
(*_20) = _24 == _24;
(*_9) = '\u{3d2ae}';
(*_20) = _24;
_34.1 = &(*_9);
(*_20) = _30.0 < _22;
match _33 {
0 => bb20,
1 => bb18,
2 => bb16,
278606353460455583433628665723506316106 => bb26,
_ => bb25
}
}
bb25 = {
RET = [18230174622157755616_u64,9544024858253637254_u64,15109853220686775732_u64,12149641117114326129_u64,6323749566519683480_u64,7280694546421817423_u64];
_1 = _4 & _4;
_3 = 234_u8 | 99_u8;
_4 = _1 <= _1;
_3 = 94123114995917579695043808107458541430_u128 as u8;
_4 = !_1;
_4 = _1;
_2 = [31010_u16];
Goto(bb4)
}
bb26 = {
(*_7) = _29.fld0;
(*_16) = _24 ^ _24;
_11 = &_30.0;
_33 = 109866042765285266069157394087310522658_i128 ^ (-67925711892631262661861920842570061159_i128);
Goto(bb27)
}
bb27 = {
Goto(bb28)
}
bb28 = {
(*_7) = _23 | _21;
_31 = core::ptr::addr_of_mut!(_20);
(*_16) = !_24;
_30 = (_22, (*_16));
_21 = (-32220_i16) as isize;
_38 = (*_16) ^ (*_16);
_30.0 = _22 - _22;
_33 = (-3743903058742765350_i64) as i128;
(*_9) = '\u{e468a}';
_43 = -_12;
Call((*_9) = fn17(Move((*_31)), Move(_29.fld1), (*_7), Move(_8), Move(_13)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
(*_31) = core::ptr::addr_of!((*_16));
(*_7) = _21 ^ _21;
(*_16) = _24 < _30.1;
Call((*_7) = core::intrinsics::bswap(_23), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*_31) = core::ptr::addr_of!((*_16));
(*_31) = core::ptr::addr_of!((*_16));
(*_7) = _29.fld0;
(*_20) = !_1;
(*_7) = _29.fld0;
_4 = _38 ^ _38;
(*_7) = _29.fld0 & _23;
_38 = (*_16) | (*_16);
(*_9) = '\u{148bd}';
(*_7) = _23;
Goto(bb31)
}
bb31 = {
_3 = 36_u8 & 233_u8;
(*_20) = _24;
_16 = core::ptr::addr_of_mut!((*_20));
_36 = (*_9);
(*_7) = _23 + _29.fld0;
(*_9) = _36;
_44 = 176035371466800158678232037313097795376_u128 << (*_7);
_51 = _19 - _19;
(*_16) = _24;
_46 = (*_7) >> (*_7);
_4 = _30.1;
Goto(bb32)
}
bb32 = {
(*_20) = _38;
_30 = (_22, _24);
(*_7) = _44 as isize;
(*_31) = core::ptr::addr_of!((*_20));
_4 = !_24;
_12 = _43;
_50 = (*_7) & (*_7);
_45 = 45385_u16 << _50;
Goto(bb33)
}
bb33 = {
_32 = Adt60::Variant1 { fld0: _2,fld1: 561491714461688117_i64,fld2: Move((*_31)) };
_19 = -_51;
_53 = &mut RET;
_10 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_32, 1), 1)));
(*_7) = (-1639158243_i32) as isize;
(*_7) = _50;
_3 = 193_u8;
place!(Field::<[u16; 1]>(Variant(_32, 1), 0)) = [_45];
(*_31) = core::ptr::addr_of!(_30.1);
Goto(bb34)
}
bb34 = {
(*_9) = _36;
(*_7) = _3 as isize;
_7 = core::ptr::addr_of!((*_7));
(*_10) = !(-2669448077185944288_i64);
(*_53) = [11573438581642507208_u64,18196520401502770474_u64,16662129910203380337_u64,9579969197532835525_u64,13347390566551170399_u64,2729641882579069785_u64];
_39 = _30.0 as f64;
(*_53) = [18197267232149107605_u64,7621200502234836779_u64,1617352317051817769_u64,13037365495333048113_u64,5065228268547918348_u64,8096714153078652151_u64];
_8 = &_12;
(*_53) = [11089074898376236672_u64,2281152256529472855_u64,1281796182935870841_u64,5275396337325033218_u64,1965227568654771399_u64,3901560329456469078_u64];
_56 = _23 | _50;
Goto(bb35)
}
bb35 = {
_47 = _45 - _45;
_29.fld1 = core::ptr::addr_of_mut!(_51);
(*_31) = core::ptr::addr_of!(_24);
(*_20) = _4 > _38;
(*_10) = !2952003775596710925_i64;
match _3 {
0 => bb22,
1 => bb13,
2 => bb11,
3 => bb36,
193 => bb38,
_ => bb37
}
}
bb36 = {
(*_20) = _38;
_30 = (_22, _24);
(*_7) = _44 as isize;
(*_31) = core::ptr::addr_of!((*_20));
_4 = !_24;
_12 = _43;
_50 = (*_7) & (*_7);
_45 = 45385_u16 << _50;
Goto(bb33)
}
bb37 = {
_19 = 11113_u16 as f64;
(*_9) = _18;
_7 = core::ptr::addr_of!((*_7));
_24 = (*_16) >= (*_16);
_12 = 11600_u16 as f32;
(*_16) = _24 < _24;
(*_7) = !_21;
_9 = &mut _18;
Goto(bb23)
}
bb38 = {
_58.2 = [_47];
(*_53) = [631592804003337650_u64,2985056137961536510_u64,12791620701174041602_u64,18132401288709034774_u64,9810133014789124415_u64,5661072793629080716_u64];
(*_10) = (-8787512954349671424_i64) * (-7703509082805810522_i64);
(*_10) = -6033978675152211620_i64;
(*_20) = !_30.1;
_13 = &(*_8);
(*_7) = _50 >> _46;
_24 = !_4;
(*_7) = !_50;
(*_9) = _36;
(*_9) = _36;
_31 = core::ptr::addr_of_mut!((*_31));
(*_31) = core::ptr::addr_of!((*_20));
(*_20) = _30.1;
(*_7) = !_50;
(*_9) = _36;
(*_7) = !_23;
(*_10) = (*_20) as i64;
_36 = (*_9);
_55 = !(-98_i8);
(*_20) = _1;
match _3 {
0 => bb33,
1 => bb39,
193 => bb41,
_ => bb40
}
}
bb39 = {
RET = [2316622545747768863_u64,16389819573748296531_u64,808879256619719359_u64,2423986083878349908_u64,5771722224127893219_u64,14751832712093211654_u64];
_1 = true;
RET = [6471388821923836080_u64,3931692462341405975_u64,18326417718635209264_u64,15080537220822827780_u64,7294572345794750546_u64,8849584728117921708_u64];
_1 = true ^ false;
_3 = !212_u8;
Goto(bb2)
}
bb40 = {
_5 = 9223372036854775807_isize + 9223372036854775807_isize;
(*_7) = 9223372036854775807_isize + (-9223372036854775808_isize);
_4 = _1;
(*_7) = (-9223372036854775808_isize);
_5 = !117_isize;
(*_7) = (-124_isize) * 9223372036854775807_isize;
(*_7) = (-67_isize) | (-9223372036854775808_isize);
(*_7) = -57_isize;
(*_7) = -9223372036854775807_isize;
_9 = &mut _6;
_3 = 165_u8 + 223_u8;
(*_9) = '\u{e7dba}';
(*_7) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_7) = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_7 = core::ptr::addr_of!((*_7));
Goto(bb10)
}
bb41 = {
(*_10) = _26 as i64;
_49 = (*_7);
(*_9) = _36;
(*_31) = core::ptr::addr_of!((*_20));
(*_31) = Move(Field::<*const bool>(Variant(_32, 1), 2));
_60.3.fld0 = _44 & _44;
(*_31) = core::ptr::addr_of!(_30.1);
Goto(bb42)
}
bb42 = {
(*_53) = [3100589864608668379_u64,14231003645815512093_u64,14850579563048184025_u64,17930500081657514417_u64,1679260477546783432_u64,11241759144441978873_u64];
(*_9) = _36;
(*_10) = (-4146541597764461676_i64) + 6825151636556118947_i64;
(*_9) = _36;
(*_20) = _4 & _1;
(*_53) = [3571029115661987990_u64,5209305632354834306_u64,8689021776583509234_u64,13169564014008059594_u64,4739981869361642667_u64,16175851541527542286_u64];
(*_10) = (-134241862034974643_i64);
_8 = &_28;
_32 = Adt60::Variant1 { fld0: _58.2,fld1: 5324542590763408358_i64,fld2: Move((*_31)) };
_30.0 = _22 - _22;
_48 = _60.3.fld0 as f64;
(*_53) = [11293617250283879635_u64,4843152404033820581_u64,10662045775911573716_u64,4394303607402990915_u64,5550740135895486370_u64,16721658839821829296_u64];
place!(Field::<i64>(Variant(_32, 1), 1)) = -2246349250326279700_i64;
(*_31) = core::ptr::addr_of!(_38);
_57.0 = core::ptr::addr_of_mut!(_59);
Goto(bb43)
}
bb43 = {
(*_31) = core::ptr::addr_of!((*_20));
_47 = _45;
_58.2 = [_45];
(*_20) = (*_8) <= (*_13);
Goto(bb44)
}
bb44 = {
(*_9) = _36;
(*_7) = (*_13) as isize;
(*_20) = _30.1 & _24;
_34.1 = &_36;
(*_9) = _36;
(*_9) = _36;
(*_31) = core::ptr::addr_of!((*_20));
_47 = _45 | _45;
(*_7) = _26 as isize;
(*_20) = _4 != _30.1;
(*_53) = [12451935478974057892_u64,10543000842545564888_u64,13058041086415746522_u64,4147381893381150951_u64,9394451837300479053_u64,4094565369462559701_u64];
_37 = (*_9);
(*_53) = [5589601636430680168_u64,6087481382450406719_u64,16223478017299723467_u64,4696881557631347161_u64,119697518861780733_u64,17878250237867468643_u64];
(*_7) = 772938315_i32 as isize;
(*_7) = _56 | _50;
_29.fld0 = 7661_i16 as isize;
place!(Field::<[u16; 1]>(Variant(_32, 1), 0)) = _58.2;
(*_9) = _36;
_58.0 = core::ptr::addr_of_mut!(_47);
_5 = _3 as isize;
(*_31) = core::ptr::addr_of!((*_20));
Goto(bb45)
}
bb45 = {
_36 = (*_9);
_10 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_32, 1), 1)));
(*_31) = core::ptr::addr_of!(_30.1);
_61 = _30.0 ^ _30.0;
_58.2 = Field::<[u16; 1]>(Variant(_32, 1), 0);
(*_31) = Move(Field::<*const bool>(Variant(_32, 1), 2));
_54.0 = core::ptr::addr_of_mut!(_42);
_30 = (_22, _4);
(*_53) = [3966400069244384703_u64,5669291281026470913_u64,14312492204857985293_u64,11104674015509066613_u64,1262414677762804892_u64,14861314143218373548_u64];
(*_53) = [6910228038692748885_u64,1740524287040113572_u64,13171845429542340309_u64,12944255939445372744_u64,7637338479775844410_u64,1633962994215491389_u64];
(*_53) = [15978528517941142907_u64,2987083755355453081_u64,6545803383163366051_u64,6498872365180621412_u64,347721928728478263_u64,152343477933001482_u64];
_58.1 = (*_7);
(*_53) = [15241540055484634855_u64,9361851724506731321_u64,12165651828536358069_u64,8110260388231279503_u64,14039677364567453153_u64,11447154059078691_u64];
(*_7) = -_50;
(*_31) = core::ptr::addr_of!(_4);
_2 = _58.2;
(*_31) = core::ptr::addr_of!((*_20));
match _3 {
0 => bb1,
1 => bb12,
2 => bb19,
3 => bb46,
4 => bb47,
193 => bb49,
_ => bb48
}
}
bb46 = {
_47 = _45 - _45;
_29.fld1 = core::ptr::addr_of_mut!(_51);
(*_31) = core::ptr::addr_of!(_24);
(*_20) = _4 > _38;
(*_10) = !2952003775596710925_i64;
match _3 {
0 => bb22,
1 => bb13,
2 => bb11,
3 => bb36,
193 => bb38,
_ => bb37
}
}
bb47 = {
(*_9) = _14.2.fld0;
(*_9) = _14.2.fld0;
(*_7) = 9223372036854775807_isize >> _3;
(*_9) = _14.2.fld0;
(*_7) = (-60_isize) << _3;
(*_7) = 9223372036854775807_isize << _3;
_9 = &mut _14.2.fld0;
(*_9) = '\u{156c0}';
(*_9) = '\u{36aaf}';
(*_7) = (-132531753535461142769230527911776244087_i128) as isize;
(*_7) = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_7) = (-127521822860890020836608657906606119630_i128) as isize;
(*_9) = '\u{38341}';
Goto(bb12)
}
bb48 = {
RET = [10807428392329229664_u64,6385851864347418866_u64,1305192329400206286_u64,12737088052234085450_u64,13178965319551414528_u64,16143315557792719504_u64];
RET = [5841736846587907098_u64,10436531089093342026_u64,4896063042261183993_u64,5341450785885573198_u64,6921933702258166563_u64,1959055052110131770_u64];
_2 = [58165_u16];
_4 = _1 == _1;
_1 = _4 >= _4;
_3 = 226_u8 >> 8767783496715301822_i64;
_2 = [29878_u16];
_4 = _1;
_1 = _4 | _4;
_2 = [44658_u16];
_1 = _4 ^ _4;
RET = [7989738247255563796_u64,5220062031977632621_u64,4474666022583586287_u64,15814923324272362713_u64,6360982502314045977_u64,16487527295304881327_u64];
RET = [638221424692937223_u64,6811385004321111849_u64,15336531846731113302_u64,4469767012600200783_u64,3041841546566317933_u64,5859386825346382681_u64];
Call(_3 = fn15(_2, _4, _4, _4, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb49 = {
(*_53) = [8209885251763579386_u64,2046231264273792305_u64,11200142774795178717_u64,7674870657535852342_u64,8560689000581160403_u64,12205985223034941223_u64];
_30 = Checked(_61 - _61);
(*_7) = -_58.1;
_51 = _19 - _19;
_60.3.fld0 = _44 - _44;
_43 = (*_13);
_47 = _45;
_68 = _47 ^ _47;
(*_20) = (*_8) > (*_8);
_61 = _33 as usize;
(*_7) = _56 - _50;
_5 = _46 >> _44;
_33 = -(-32740313821439144017371629274215103868_i128);
Goto(bb50)
}
bb50 = {
Call(_69 = dump_var(Move(_1), Move(_45), Move(_30), Move(_4)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_69 = dump_var(Move(_50), Move(_56), Move(_47), Move(_33)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_69 = dump_var(Move(_26), Move(_55), Move(_18), Move(_36)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_69 = dump_var(Move(_21), _70, _70, _70), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u16; 1],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: [u16; 1]) -> u8 {
mir! {
type RET = u8;
let _6: f64;
let _7: &'static char;
let _8: &'static char;
let _9: &'static char;
let _10: &'static *mut bool;
let _11: f32;
let _12: i64;
let _13: u128;
let _14: [char; 6];
let _15: &'static mut usize;
let _16: Adt19;
let _17: isize;
let _18: &'static mut char;
let _19: i16;
let _20: Adt19;
let _21: [bool; 6];
let _22: char;
let _23: f32;
let _24: *mut [char; 6];
let _25: u16;
let _26: (usize, bool);
let _27: f32;
let _28: isize;
let _29: Adt60;
let _30: f32;
let _31: u16;
let _32: i16;
let _33: &'static char;
let _34: *const Adt19;
let _35: f64;
let _36: (*mut u16, isize, [u16; 1], Adt19);
let _37: isize;
let _38: isize;
let _39: *mut *mut *const bool;
let _40: *mut u32;
let _41: bool;
let _42: *const *mut u16;
let _43: u64;
let _44: bool;
let _45: *const (&'static char,);
let _46: usize;
let _47: bool;
let _48: u16;
let _49: isize;
let _50: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _51: char;
let _52: &'static mut &'static mut usize;
let _53: [i8; 6];
let _54: ();
let _55: ();
{
_4 = _3;
RET = 99_u8 ^ 202_u8;
_6 = 9506141953185035764_usize as f64;
_1 = [31794_u16];
_2 = !_4;
_5 = [30959_u16];
_1 = [61848_u16];
Goto(bb1)
}
bb1 = {
RET = 51_u8 | 121_u8;
_4 = _3 | _3;
_1 = [4103_u16];
_3 = _4;
_3 = RET < RET;
_5 = _1;
RET = 225_u8 << 285706560895771344499670658379794229708_u128;
_2 = _4 >= _4;
_1 = _5;
_4 = _2 <= _2;
_2 = _4 != _4;
_11 = 3225706462438638704_i64 as f32;
_1 = [12682_u16];
_4 = !_2;
_1 = [21261_u16];
_1 = [3737_u16];
_4 = _2 | _2;
_11 = 28849_u16 as f32;
_2 = _4 > _4;
_4 = _3;
_12 = 4851232107449427100_i64;
_2 = _4;
_5 = [43407_u16];
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
4851232107449427100 => bb8,
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
_4 = _3 != _2;
_14 = ['\u{72b0}','\u{78d72}','\u{79313}','\u{c181b}','\u{db4b9}','\u{74e1b}'];
_13 = 111741342167805941532646903518316276359_u128 + 311629694567607948293362379654375611836_u128;
_6 = (-123_i8) as f64;
_16 = Adt19::Variant0 { fld0: _4,fld1: (-24274670363020662031616016882355204175_i128),fld2: 743886514976015223_u64 };
_2 = _3 > _4;
_4 = !_2;
_3 = Field::<bool>(Variant(_16, 0), 0);
Goto(bb9)
}
bb9 = {
_17 = 2_isize ^ 119_isize;
_17 = (-82_isize);
_4 = _2 > Field::<bool>(Variant(_16, 0), 0);
_3 = _4;
_17 = 9223372036854775807_isize & 54_isize;
_6 = 419031431_i32 as f64;
_14 = ['\u{24be3}','\u{f78b0}','\u{53caa}','\u{85ee2}','\u{df7f5}','\u{3437d}'];
place!(Field::<u64>(Variant(_16, 0), 2)) = 2463840927359763650_u64;
_12 = (-125233707561988133895761630884954548563_i128) as i64;
_13 = !84748743089645082627441623526036331951_u128;
place!(Field::<i128>(Variant(_16, 0), 1)) = !(-108326034423360859189519914560685394611_i128);
_16 = Adt19::Variant0 { fld0: _2,fld1: (-109309236771128292771933764131969467269_i128),fld2: 10671477556738798019_u64 };
_21 = [_3,Field::<bool>(Variant(_16, 0), 0),Field::<bool>(Variant(_16, 0), 0),_4,_4,Field::<bool>(Variant(_16, 0), 0)];
_14 = ['\u{77f2a}','\u{73b64}','\u{3d7da}','\u{60bfd}','\u{2ed5}','\u{efc74}'];
place!(Field::<u64>(Variant(_16, 0), 2)) = RET as u64;
place!(Field::<bool>(Variant(_16, 0), 0)) = _2 | _4;
_19 = (-22850_i16) ^ (-9503_i16);
_22 = '\u{91dc3}';
Goto(bb10)
}
bb10 = {
_17 = 9223372036854775807_isize & (-9223372036854775808_isize);
_16 = Adt19::Variant0 { fld0: _4,fld1: (-113138884330674819903177900120200891972_i128),fld2: 16795231462455407131_u64 };
_1 = [15559_u16];
_9 = &_22;
place!(Field::<u64>(Variant(_16, 0), 2)) = 4576558699168119125_u64;
RET = 166_u8 - 22_u8;
_4 = _2 > _2;
place!(Field::<i128>(Variant(_16, 0), 1)) = (-81014072426915788326938465359920805525_i128) - 52234559361020840394954668170275801761_i128;
_21 = [_4,_2,_2,_4,_3,_2];
_12 = (-870033826226176444_i64) ^ (-6886334419646885698_i64);
RET = !46_u8;
_8 = &(*_9);
place!(Field::<bool>(Variant(_16, 0), 0)) = _4 < _4;
_13 = !34788686337894758539551644557581161043_u128;
_7 = Move(_9);
_23 = _11;
_17 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_7 = Move(_8);
_2 = !Field::<bool>(Variant(_16, 0), 0);
_9 = &_22;
place!(Field::<bool>(Variant(_16, 0), 0)) = !_2;
Goto(bb11)
}
bb11 = {
_25 = _17 as u16;
_14 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_7 = &(*_9);
_8 = &_22;
_11 = -_23;
_21 = [_4,Field::<bool>(Variant(_16, 0), 0),_4,_2,_2,_2];
_20 = Move(_16);
_9 = Move(_7);
_4 = _2 < Field::<bool>(Variant(_20, 0), 0);
_3 = _4 ^ _4;
_24 = core::ptr::addr_of_mut!(_14);
_16 = Move(_20);
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),_22];
Goto(bb12)
}
bb12 = {
Goto(bb13)
}
bb13 = {
_26.1 = (*_8) <= (*_8);
_23 = Field::<i128>(Variant(_16, 0), 1) as f32;
_22 = '\u{c7e08}';
_2 = _4 < _4;
_27 = -_11;
_7 = &_22;
_17 = 9223372036854775807_isize & (-114_isize);
(*_24) = [(*_7),(*_7),(*_7),(*_7),(*_7),_22];
_7 = Move(_8);
(*_24) = [_22,_22,_22,_22,_22,_22];
(*_24) = [_22,_22,_22,_22,_22,_22];
match Field::<u64>(Variant(_16, 0), 2) {
4576558699168119125 => bb15,
_ => bb14
}
}
bb14 = {
RET = 51_u8 | 121_u8;
_4 = _3 | _3;
_1 = [4103_u16];
_3 = _4;
_3 = RET < RET;
_5 = _1;
RET = 225_u8 << 285706560895771344499670658379794229708_u128;
_2 = _4 >= _4;
_1 = _5;
_4 = _2 <= _2;
_2 = _4 != _4;
_11 = 3225706462438638704_i64 as f32;
_1 = [12682_u16];
_4 = !_2;
_1 = [21261_u16];
_1 = [3737_u16];
_4 = _2 | _2;
_11 = 28849_u16 as f32;
_2 = _4 > _4;
_4 = _3;
_12 = 4851232107449427100_i64;
_2 = _4;
_5 = [43407_u16];
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
4851232107449427100 => bb8,
_ => bb7
}
}
bb15 = {
(*_24) = [_22,_22,_22,_22,_22,_22];
(*_24) = [_22,_22,_22,_22,_22,_22];
_21 = [_2,_2,_3,_3,_4,_4];
_13 = 148627417987613604535161051314515860902_u128 | 332734962101036764620565897586617544357_u128;
Call(_30 = fn16(Move(_24), _27, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_26 = (1_usize, _2);
_13 = 116068283154890582419028761086100503474_u128;
RET = 234_u8 | 122_u8;
match _26.0 {
0 => bb1,
2 => bb15,
3 => bb4,
4 => bb17,
5 => bb18,
1 => bb20,
_ => bb19
}
}
bb17 = {
(*_24) = [_22,_22,_22,_22,_22,_22];
(*_24) = [_22,_22,_22,_22,_22,_22];
_21 = [_2,_2,_3,_3,_4,_4];
_13 = 148627417987613604535161051314515860902_u128 | 332734962101036764620565897586617544357_u128;
Call(_30 = fn16(Move(_24), _27, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb18 = {
_25 = _17 as u16;
_14 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_7 = &(*_9);
_8 = &_22;
_11 = -_23;
_21 = [_4,Field::<bool>(Variant(_16, 0), 0),_4,_2,_2,_2];
_20 = Move(_16);
_9 = Move(_7);
_4 = _2 < Field::<bool>(Variant(_20, 0), 0);
_3 = _4 ^ _4;
_24 = core::ptr::addr_of_mut!(_14);
_16 = Move(_20);
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),_22];
Goto(bb12)
}
bb19 = {
_17 = 9223372036854775807_isize & (-9223372036854775808_isize);
_16 = Adt19::Variant0 { fld0: _4,fld1: (-113138884330674819903177900120200891972_i128),fld2: 16795231462455407131_u64 };
_1 = [15559_u16];
_9 = &_22;
place!(Field::<u64>(Variant(_16, 0), 2)) = 4576558699168119125_u64;
RET = 166_u8 - 22_u8;
_4 = _2 > _2;
place!(Field::<i128>(Variant(_16, 0), 1)) = (-81014072426915788326938465359920805525_i128) - 52234559361020840394954668170275801761_i128;
_21 = [_4,_2,_2,_4,_3,_2];
_12 = (-870033826226176444_i64) ^ (-6886334419646885698_i64);
RET = !46_u8;
_8 = &(*_9);
place!(Field::<bool>(Variant(_16, 0), 0)) = _4 < _4;
_13 = !34788686337894758539551644557581161043_u128;
_7 = Move(_9);
_23 = _11;
_17 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_7 = Move(_8);
_2 = !Field::<bool>(Variant(_16, 0), 0);
_9 = &_22;
place!(Field::<bool>(Variant(_16, 0), 0)) = !_2;
Goto(bb11)
}
bb20 = {
_2 = _26.1;
_29 = Adt60::Variant2 { fld0: _4,fld1: 10839460174520470471_u64 };
_26 = (2_usize, _4);
_26.0 = 7_usize >> _19;
_15 = &mut _26.0;
(*_15) = 0_usize ^ 7503320298448648057_usize;
_36.0 = core::ptr::addr_of_mut!(_31);
_34 = core::ptr::addr_of!(_36.3);
_7 = &_22;
(*_34) = Adt19::Variant2 { fld0: (-2146652581_i32),fld1: (*_15),fld2: RET,fld3: 90_i8,fld4: (-48519189203790882402550214149500319142_i128) };
place!(Field::<i128>(Variant((*_34), 2), 4)) = 67028939273119347290351182591452083201_i128;
place!(Field::<i32>(Variant((*_34), 2), 0)) = 890531601_i32 >> (*_15);
place!(Field::<u8>(Variant((*_34), 2), 2)) = !RET;
match Field::<i128>(Variant((*_34), 2), 4) {
0 => bb12,
1 => bb6,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb21,
67028939273119347290351182591452083201 => bb23,
_ => bb22
}
}
bb21 = {
_17 = 9223372036854775807_isize & (-9223372036854775808_isize);
_16 = Adt19::Variant0 { fld0: _4,fld1: (-113138884330674819903177900120200891972_i128),fld2: 16795231462455407131_u64 };
_1 = [15559_u16];
_9 = &_22;
place!(Field::<u64>(Variant(_16, 0), 2)) = 4576558699168119125_u64;
RET = 166_u8 - 22_u8;
_4 = _2 > _2;
place!(Field::<i128>(Variant(_16, 0), 1)) = (-81014072426915788326938465359920805525_i128) - 52234559361020840394954668170275801761_i128;
_21 = [_4,_2,_2,_4,_3,_2];
_12 = (-870033826226176444_i64) ^ (-6886334419646885698_i64);
RET = !46_u8;
_8 = &(*_9);
place!(Field::<bool>(Variant(_16, 0), 0)) = _4 < _4;
_13 = !34788686337894758539551644557581161043_u128;
_7 = Move(_9);
_23 = _11;
_17 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_7 = Move(_8);
_2 = !Field::<bool>(Variant(_16, 0), 0);
_9 = &_22;
place!(Field::<bool>(Variant(_16, 0), 0)) = !_2;
Goto(bb11)
}
bb22 = {
RET = 51_u8 | 121_u8;
_4 = _3 | _3;
_1 = [4103_u16];
_3 = _4;
_3 = RET < RET;
_5 = _1;
RET = 225_u8 << 285706560895771344499670658379794229708_u128;
_2 = _4 >= _4;
_1 = _5;
_4 = _2 <= _2;
_2 = _4 != _4;
_11 = 3225706462438638704_i64 as f32;
_1 = [12682_u16];
_4 = !_2;
_1 = [21261_u16];
_1 = [3737_u16];
_4 = _2 | _2;
_11 = 28849_u16 as f32;
_2 = _4 > _4;
_4 = _3;
_12 = 4851232107449427100_i64;
_2 = _4;
_5 = [43407_u16];
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
4851232107449427100 => bb8,
_ => bb7
}
}
bb23 = {
place!(Field::<i8>(Variant((*_34), 2), 3)) = 55_i8;
_36.2 = [_25];
place!(Field::<i128>(Variant((*_34), 2), 4)) = (-105445201653711447460316368792278147221_i128);
_31 = _25;
_6 = _17 as f64;
_16 = Move((*_34));
_38 = _17;
_24 = core::ptr::addr_of_mut!(_14);
_8 = &(*_7);
(*_15) = !Field::<usize>(Variant(_16, 2), 1);
(*_24) = [(*_8),(*_8),(*_7),(*_8),(*_7),(*_8)];
(*_15) = Field::<usize>(Variant(_16, 2), 1) >> _38;
Call((*_15) = core::intrinsics::bswap(Field::<usize>(Variant(_16, 2), 1)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_32 = _19;
(*_34) = Move(_16);
_19 = _32 * _32;
_12 = (-7820836702720973806_i64) & (-4170420692027461113_i64);
_25 = !_31;
place!(Field::<usize>(Variant(_36.3, 2), 1)) = _6 as usize;
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15) + (*_15);
(*_24) = [(*_7),(*_8),(*_7),(*_8),(*_8),(*_7)];
_36.2 = [_31];
match Field::<i128>(Variant((*_34), 2), 4) {
0 => bb25,
234837165267227016003058238639490064235 => bb27,
_ => bb26
}
}
bb25 = {
Return()
}
bb26 = {
_25 = _17 as u16;
_14 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_7 = &(*_9);
_8 = &_22;
_11 = -_23;
_21 = [_4,Field::<bool>(Variant(_16, 0), 0),_4,_2,_2,_2];
_20 = Move(_16);
_9 = Move(_7);
_4 = _2 < Field::<bool>(Variant(_20, 0), 0);
_3 = _4 ^ _4;
_24 = core::ptr::addr_of_mut!(_14);
_16 = Move(_20);
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),_22];
Goto(bb12)
}
bb27 = {
place!(Field::<usize>(Variant((*_34), 2), 1)) = _30 as usize;
place!(Field::<i32>(Variant((*_34), 2), 0)) = 1808931654_i32 >> Field::<usize>(Variant((*_34), 2), 1);
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15) << Field::<i32>(Variant((*_34), 2), 0);
_16 = Adt19::Variant0 { fld0: Field::<bool>(Variant(_29, 2), 0),fld1: Field::<i128>(Variant((*_34), 2), 4),fld2: 12779046597918046006_u64 };
place!(Field::<i32>(Variant((*_34), 2), 0)) = 10332303924531548517_u64 as i32;
place!(Field::<usize>(Variant((*_34), 2), 1)) = !(*_15);
(*_15) = !Field::<usize>(Variant((*_34), 2), 1);
place!(Field::<i32>(Variant((*_34), 2), 0)) = (-1760925282_i32);
(*_34) = Adt19::Variant2 { fld0: (-360849941_i32),fld1: (*_15),fld2: RET,fld3: (-41_i8),fld4: Field::<i128>(Variant(_16, 0), 1) };
_38 = !_17;
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15);
_14 = [(*_7),(*_7),(*_7),(*_8),(*_8),(*_8)];
(*_15) = Field::<usize>(Variant((*_34), 2), 1);
_41 = !Field::<bool>(Variant(_29, 2), 0);
place!(Field::<i128>(Variant((*_34), 2), 4)) = !Field::<i128>(Variant(_16, 0), 1);
match Field::<i128>(Variant(_16, 0), 1) {
0 => bb24,
1 => bb7,
2 => bb28,
3 => bb29,
4 => bb30,
234837165267227016003058238639490064235 => bb32,
_ => bb31
}
}
bb28 = {
_25 = _17 as u16;
_14 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_7 = &(*_9);
_8 = &_22;
_11 = -_23;
_21 = [_4,Field::<bool>(Variant(_16, 0), 0),_4,_2,_2,_2];
_20 = Move(_16);
_9 = Move(_7);
_4 = _2 < Field::<bool>(Variant(_20, 0), 0);
_3 = _4 ^ _4;
_24 = core::ptr::addr_of_mut!(_14);
_16 = Move(_20);
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),_22];
Goto(bb12)
}
bb29 = {
RET = 51_u8 | 121_u8;
_4 = _3 | _3;
_1 = [4103_u16];
_3 = _4;
_3 = RET < RET;
_5 = _1;
RET = 225_u8 << 285706560895771344499670658379794229708_u128;
_2 = _4 >= _4;
_1 = _5;
_4 = _2 <= _2;
_2 = _4 != _4;
_11 = 3225706462438638704_i64 as f32;
_1 = [12682_u16];
_4 = !_2;
_1 = [21261_u16];
_1 = [3737_u16];
_4 = _2 | _2;
_11 = 28849_u16 as f32;
_2 = _4 > _4;
_4 = _3;
_12 = 4851232107449427100_i64;
_2 = _4;
_5 = [43407_u16];
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
4851232107449427100 => bb8,
_ => bb7
}
}
bb30 = {
Return()
}
bb31 = {
_17 = 9223372036854775807_isize & (-9223372036854775808_isize);
_16 = Adt19::Variant0 { fld0: _4,fld1: (-113138884330674819903177900120200891972_i128),fld2: 16795231462455407131_u64 };
_1 = [15559_u16];
_9 = &_22;
place!(Field::<u64>(Variant(_16, 0), 2)) = 4576558699168119125_u64;
RET = 166_u8 - 22_u8;
_4 = _2 > _2;
place!(Field::<i128>(Variant(_16, 0), 1)) = (-81014072426915788326938465359920805525_i128) - 52234559361020840394954668170275801761_i128;
_21 = [_4,_2,_2,_4,_3,_2];
_12 = (-870033826226176444_i64) ^ (-6886334419646885698_i64);
RET = !46_u8;
_8 = &(*_9);
place!(Field::<bool>(Variant(_16, 0), 0)) = _4 < _4;
_13 = !34788686337894758539551644557581161043_u128;
_7 = Move(_9);
_23 = _11;
_17 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_7 = Move(_8);
_2 = !Field::<bool>(Variant(_16, 0), 0);
_9 = &_22;
place!(Field::<bool>(Variant(_16, 0), 0)) = !_2;
Goto(bb11)
}
bb32 = {
place!(Field::<i32>(Variant((*_34), 2), 0)) = 456173_i32;
_46 = _13 as usize;
place!(Field::<i32>(Variant((*_34), 2), 0)) = !(-213496788_i32);
place!(Field::<i128>(Variant((*_34), 2), 4)) = Field::<i128>(Variant(_16, 0), 1) >> Field::<usize>(Variant((*_34), 2), 1);
place!(Field::<i8>(Variant((*_34), 2), 3)) = (*_15) as i8;
match _13 {
0 => bb22,
1 => bb12,
2 => bb8,
3 => bb27,
4 => bb28,
5 => bb33,
116068283154890582419028761086100503474 => bb35,
_ => bb34
}
}
bb33 = {
(*_24) = [_22,_22,_22,_22,_22,_22];
(*_24) = [_22,_22,_22,_22,_22,_22];
_21 = [_2,_2,_3,_3,_4,_4];
_13 = 148627417987613604535161051314515860902_u128 | 332734962101036764620565897586617544357_u128;
Call(_30 = fn16(Move(_24), _27, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb34 = {
Return()
}
bb35 = {
place!(Field::<i128>(Variant((*_34), 2), 4)) = Field::<i128>(Variant(_16, 0), 1) >> Field::<i32>(Variant((*_34), 2), 0);
(*_24) = [(*_7),(*_7),(*_8),(*_7),(*_7),(*_8)];
_13 = 241086460625051174974315226498032825249_u128;
place!(Field::<i8>(Variant((*_34), 2), 3)) = 13_i8 & 106_i8;
place!(Field::<i128>(Variant(_16, 0), 1)) = _12 as i128;
_35 = _6 + _6;
_49 = _17;
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15);
_42 = core::ptr::addr_of!(_36.0);
place!(Field::<u8>(Variant((*_34), 2), 2)) = _25 as u8;
_44 = !_41;
place!(Field::<u64>(Variant(_29, 2), 1)) = !8704320953592450089_u64;
place!(Field::<u8>(Variant((*_34), 2), 2)) = RET << Field::<usize>(Variant((*_34), 2), 1);
match _13 {
0 => bb21,
1 => bb15,
2 => bb34,
3 => bb16,
4 => bb36,
5 => bb37,
241086460625051174974315226498032825249 => bb39,
_ => bb38
}
}
bb36 = {
_32 = _19;
(*_34) = Move(_16);
_19 = _32 * _32;
_12 = (-7820836702720973806_i64) & (-4170420692027461113_i64);
_25 = !_31;
place!(Field::<usize>(Variant(_36.3, 2), 1)) = _6 as usize;
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15) + (*_15);
(*_24) = [(*_7),(*_8),(*_7),(*_8),(*_8),(*_7)];
_36.2 = [_31];
match Field::<i128>(Variant((*_34), 2), 4) {
0 => bb25,
234837165267227016003058238639490064235 => bb27,
_ => bb26
}
}
bb37 = {
_25 = _17 as u16;
_14 = [(*_9),(*_9),(*_9),(*_9),(*_9),(*_9)];
_7 = &(*_9);
_8 = &_22;
_11 = -_23;
_21 = [_4,Field::<bool>(Variant(_16, 0), 0),_4,_2,_2,_2];
_20 = Move(_16);
_9 = Move(_7);
_4 = _2 < Field::<bool>(Variant(_20, 0), 0);
_3 = _4 ^ _4;
_24 = core::ptr::addr_of_mut!(_14);
_16 = Move(_20);
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
(*_24) = [(*_8),(*_8),(*_8),(*_8),(*_8),_22];
Goto(bb12)
}
bb38 = {
_17 = 9223372036854775807_isize & (-9223372036854775808_isize);
_16 = Adt19::Variant0 { fld0: _4,fld1: (-113138884330674819903177900120200891972_i128),fld2: 16795231462455407131_u64 };
_1 = [15559_u16];
_9 = &_22;
place!(Field::<u64>(Variant(_16, 0), 2)) = 4576558699168119125_u64;
RET = 166_u8 - 22_u8;
_4 = _2 > _2;
place!(Field::<i128>(Variant(_16, 0), 1)) = (-81014072426915788326938465359920805525_i128) - 52234559361020840394954668170275801761_i128;
_21 = [_4,_2,_2,_4,_3,_2];
_12 = (-870033826226176444_i64) ^ (-6886334419646885698_i64);
RET = !46_u8;
_8 = &(*_9);
place!(Field::<bool>(Variant(_16, 0), 0)) = _4 < _4;
_13 = !34788686337894758539551644557581161043_u128;
_7 = Move(_9);
_23 = _11;
_17 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_7 = Move(_8);
_2 = !Field::<bool>(Variant(_16, 0), 0);
_9 = &_22;
place!(Field::<bool>(Variant(_16, 0), 0)) = !_2;
Goto(bb11)
}
bb39 = {
_11 = _30 - _30;
_19 = _38 as i16;
_48 = !_25;
place!(Field::<i128>(Variant((*_34), 2), 4)) = Field::<i128>(Variant(_16, 0), 1) - Field::<i128>(Variant(_16, 0), 1);
place!(Field::<i32>(Variant((*_34), 2), 0)) = (-254440293_i32) << (*_15);
place!(Field::<usize>(Variant((*_34), 2), 1)) = Field::<u64>(Variant(_29, 2), 1) as usize;
(*_24) = [(*_7),(*_8),(*_8),(*_7),(*_8),(*_8)];
place!(Field::<i8>(Variant((*_34), 2), 3)) = _41 as i8;
RET = Field::<u8>(Variant((*_34), 2), 2) | Field::<u8>(Variant((*_34), 2), 2);
place!(Field::<i32>(Variant((*_34), 2), 0)) = 1724393168_i32 | (-2008728080_i32);
_50.3 = &mut _13;
(*_42) = core::ptr::addr_of_mut!(_31);
_43 = Field::<u64>(Variant(_29, 2), 1) & Field::<u64>(Variant(_29, 2), 1);
place!(Field::<i8>(Variant((*_34), 2), 3)) = 110_i8;
_48 = _2 as u16;
place!(Field::<i128>(Variant((*_34), 2), 4)) = Field::<i128>(Variant(_16, 0), 1);
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15) | (*_15);
place!(Field::<usize>(Variant((*_34), 2), 1)) = (*_15) << _48;
_14 = [(*_7),(*_7),(*_8),(*_7),(*_7),(*_8)];
place!(Field::<i32>(Variant((*_34), 2), 0)) = 2818036377_u32 as i32;
_20 = Move((*_34));
(*_34) = Adt19::Variant2 { fld0: Field::<i32>(Variant(_20, 2), 0),fld1: Field::<usize>(Variant(_20, 2), 1),fld2: Field::<u8>(Variant(_20, 2), 2),fld3: Field::<i8>(Variant(_20, 2), 3),fld4: Field::<i128>(Variant(_20, 2), 4) };
(*_42) = core::ptr::addr_of_mut!(_48);
_47 = Field::<u8>(Variant(_20, 2), 2) == Field::<u8>(Variant((*_34), 2), 2);
Goto(bb40)
}
bb40 = {
Call(_54 = dump_var(Move(_47), Move(_31), Move(_38), Move(_43)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_54 = dump_var(Move(_32), Move(_46), Move(_19), Move(_12)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_54 = dump_var(Move(_41), Move(_2), Move(_21), Move(_49)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *mut [char; 6],mut _2: f32,mut _3: Adt19) -> f32 {
mir! {
type RET = f32;
let _4: *mut *mut f64;
let _5: f32;
let _6: Adt19;
let _7: i16;
let _8: u8;
let _9: ();
let _10: ();
{
RET = _2 + _2;
place!(Field::<i128>(Variant(_3, 0), 1)) = 77400531471299379962636937537453949727_i128 * (-12952063840928250796085120949576580893_i128);
_2 = 20_i8 as f32;
match Field::<u64>(Variant(_3, 0), 2) {
4576558699168119125 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2 = RET;
place!(Field::<u64>(Variant(_3, 0), 2)) = 2438430705851474758_u64;
place!(Field::<bool>(Variant(_3, 0), 0)) = Field::<i128>(Variant(_3, 0), 1) >= Field::<i128>(Variant(_3, 0), 1);
RET = _2;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-80847079285072510533325209997887982525_i128) << Field::<u64>(Variant(_3, 0), 2);
_5 = _2 - _2;
_2 = _5;
RET = -_2;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-1369630886173061816453386839563648795_i128) & (-59324773172529263087114823114874962434_i128);
_5 = _2 - RET;
place!(Field::<i128>(Variant(_3, 0), 1)) = (-106812466197016180459196069431389145033_i128) | (-142492968151088916377658030804714042193_i128);
_5 = _2 * _2;
_6 = Move(_3);
place!(Field::<bool>(Variant(_6, 0), 0)) = false;
_2 = _5 - _5;
_7 = 25411_i16 << Field::<i128>(Variant(_6, 0), 1);
RET = _7 as f32;
_7 = 24549_i16 * (-18289_i16);
place!(Field::<bool>(Variant(_6, 0), 0)) = Field::<i128>(Variant(_6, 0), 1) != Field::<i128>(Variant(_6, 0), 1);
_3 = Move(_6);
_8 = !75_u8;
_5 = (-9223372036854775808_isize) as f32;
RET = _2 - _2;
Goto(bb3)
}
bb3 = {
Call(_9 = dump_var(Move(_8), _10, _10, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *const bool,mut _2: *mut f64,mut _3: isize,mut _4: &'static f32,mut _5: &'static f32) -> char {
mir! {
type RET = char;
let _6: char;
let _7: isize;
let _8: (*mut f64, &'static char);
let _9: *mut u16;
let _10: *const Adt60;
let _11: i16;
let _12: f64;
let _13: isize;
let _14: f64;
let _15: *const isize;
let _16: &'static usize;
let _17: *const i64;
let _18: char;
let _19: i8;
let _20: *mut *const bool;
let _21: isize;
let _22: i128;
let _23: &'static mut usize;
let _24: f32;
let _25: *const Adt60;
let _26: isize;
let _27: f32;
let _28: isize;
let _29: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _30: i64;
let _31: usize;
let _32: &'static (u32, u32, Adt17, f32);
let _33: &'static Adt19;
let _34: f64;
let _35: &'static mut u128;
let _36: char;
let _37: char;
let _38: *mut [char; 6];
let _39: bool;
let _40: f32;
let _41: *const Adt60;
let _42: *const Adt50;
let _43: &'static mut usize;
let _44: Adt19;
let _45: &'static mut char;
let _46: [char; 7];
let _47: bool;
let _48: u64;
let _49: &'static *const *mut u16;
let _50: *mut *mut f64;
let _51: i8;
let _52: *const Adt50;
let _53: *mut u16;
let _54: *const isize;
let _55: char;
let _56: f64;
let _57: i128;
let _58: *mut bool;
let _59: u16;
let _60: char;
let _61: bool;
let _62: isize;
let _63: &'static (char, Adt19, i32, (u32, u32, Adt17, f32));
let _64: f64;
let _65: &'static (u32, u32, Adt17, f32);
let _66: f32;
let _67: f64;
let _68: u16;
let _69: *const *mut u16;
let _70: &'static mut u128;
let _71: Adt40;
let _72: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _73: i128;
let _74: isize;
let _75: &'static mut [u64; 6];
let _76: *mut u32;
let _77: i64;
let _78: &'static (f64, &'static (u32, u32, Adt17, f32));
let _79: Adt85;
let _80: f64;
let _81: *mut f64;
let _82: char;
let _83: *mut *mut *const bool;
let _84: isize;
let _85: u8;
let _86: [char; 1];
let _87: f64;
let _88: (&'static mut char, f32, Adt40, usize);
let _89: isize;
let _90: Adt34;
let _91: Adt19;
let _92: Adt85;
let _93: f32;
let _94: f64;
let _95: *const i64;
let _96: isize;
let _97: i8;
let _98: bool;
let _99: char;
let _100: &'static *const isize;
let _101: Adt52;
let _102: Adt40;
let _103: *const f64;
let _104: bool;
let _105: &'static &'static mut [u64; 6];
let _106: u8;
let _107: &'static mut &'static *mut bool;
let _108: f32;
let _109: char;
let _110: [char; 1];
let _111: isize;
let _112: [char; 6];
let _113: Adt19;
let _114: f32;
let _115: *const bool;
let _116: &'static char;
let _117: isize;
let _118: &'static mut u128;
let _119: char;
let _120: [char; 6];
let _121: ();
let _122: ();
{
_3 = (-9223372036854775808_isize);
_7 = _3 & _3;
RET = '\u{b7d76}';
_7 = _3;
_6 = RET;
_6 = RET;
_3 = -_7;
_6 = RET;
_8.0 = Move(_2);
_2 = Move(_8.0);
Goto(bb1)
}
bb1 = {
_7 = 245839088503080572499992458301274968231_u128 as isize;
_8.0 = Move(_2);
_8.1 = &_6;
RET = _6;
_6 = RET;
_8.1 = &RET;
_2 = Move(_8.0);
_3 = !_7;
_7 = _3;
_8.0 = Move(_2);
_6 = RET;
_7 = _3 + _3;
_2 = Move(_8.0);
_7 = _3 + _3;
_6 = RET;
_8.0 = Move(_2);
_3 = !_7;
RET = _6;
_6 = RET;
RET = _6;
_8.1 = &_6;
Call(_11 = fn18(Move(_8), Move(_1), _3, _3, _3, _7, _6, _7, RET, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = RET;
RET = _6;
_8.1 = &RET;
_12 = _7 as f64;
_7 = _3 << _3;
_12 = 247_u8 as f64;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 131_u8 as f64;
_11 = (-3651122563638789313_i64) as i16;
(*_2) = 68_u8 as f64;
(*_2) = 144987118257922234699536836311618865165_i128 as f64;
(*_2) = 17422075417074694140_usize as f64;
_3 = _7 << _7;
(*_2) = 231741617849402977736963009379108161787_u128 as f64;
(*_2) = 1050208876_i32 as f64;
_8.0 = core::ptr::addr_of_mut!((*_2));
_7 = -_3;
(*_2) = (-3375438853764895654_i64) as f64;
Goto(bb3)
}
bb3 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb4 = {
(*_2) = _14 - _14;
(*_2) = -_14;
(*_15) = _21 << _3;
_8.1 = &_18;
_22 = (-38616576786554641923130232698862197707_i128) >> (*_15);
(*_2) = -_14;
RET = _6;
_2 = core::ptr::addr_of_mut!((*_2));
(*_15) = !_21;
(*_2) = _14 - _14;
(*_15) = _3 | _3;
(*_2) = _14;
_13 = _3 << _3;
(*_2) = -_14;
(*_2) = _14;
(*_2) = -_14;
(*_15) = 109_u8 as isize;
match _19 {
62 => bb5,
_ => bb2
}
}
bb5 = {
(*_2) = _14;
(*_2) = _14 - _14;
RET = _6;
_15 = core::ptr::addr_of!((*_15));
_15 = core::ptr::addr_of!((*_15));
(*_2) = _14;
(*_15) = !_7;
(*_2) = _14 + _14;
(*_15) = _3 - _3;
_28 = (*_15);
_29.0.0 = Move(_8.1);
_2 = core::ptr::addr_of_mut!((*_2));
_29.2 = Adt34 { fld0: _18 };
_26 = _7 & (*_15);
_27 = _19 as f32;
(*_2) = _14 + _14;
(*_15) = true as isize;
(*_15) = 44_u8 as isize;
(*_15) = 136_u8 as isize;
_20 = core::ptr::addr_of_mut!((*_20));
(*_15) = _26;
(*_2) = _14 - _14;
_14 = (*_2) + (*_2);
match _19 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
62 => bb10,
_ => bb9
}
}
bb6 = {
(*_2) = _14 - _14;
(*_2) = -_14;
(*_15) = _21 << _3;
_8.1 = &_18;
_22 = (-38616576786554641923130232698862197707_i128) >> (*_15);
(*_2) = -_14;
RET = _6;
_2 = core::ptr::addr_of_mut!((*_2));
(*_15) = !_21;
(*_2) = _14 - _14;
(*_15) = _3 | _3;
(*_2) = _14;
_13 = _3 << _3;
(*_2) = -_14;
(*_2) = _14;
(*_2) = -_14;
(*_15) = 109_u8 as isize;
match _19 {
62 => bb5,
_ => bb2
}
}
bb7 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb8 = {
_6 = RET;
RET = _6;
_8.1 = &RET;
_12 = _7 as f64;
_7 = _3 << _3;
_12 = 247_u8 as f64;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 131_u8 as f64;
_11 = (-3651122563638789313_i64) as i16;
(*_2) = 68_u8 as f64;
(*_2) = 144987118257922234699536836311618865165_i128 as f64;
(*_2) = 17422075417074694140_usize as f64;
_3 = _7 << _7;
(*_2) = 231741617849402977736963009379108161787_u128 as f64;
(*_2) = 1050208876_i32 as f64;
_8.0 = core::ptr::addr_of_mut!((*_2));
_7 = -_3;
(*_2) = (-3375438853764895654_i64) as f64;
Goto(bb3)
}
bb9 = {
_7 = 245839088503080572499992458301274968231_u128 as isize;
_8.0 = Move(_2);
_8.1 = &_6;
RET = _6;
_6 = RET;
_8.1 = &RET;
_2 = Move(_8.0);
_3 = !_7;
_7 = _3;
_8.0 = Move(_2);
_6 = RET;
_7 = _3 + _3;
_2 = Move(_8.0);
_7 = _3 + _3;
_6 = RET;
_8.0 = Move(_2);
_3 = !_7;
RET = _6;
_6 = RET;
RET = _6;
_8.1 = &_6;
Call(_11 = fn18(Move(_8), Move(_1), _3, _3, _3, _7, _6, _7, RET, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_12 = _14 - _14;
(*_2) = _14 - _14;
(*_2) = -_14;
_6 = _18;
match _19 {
0 => bb11,
62 => bb13,
_ => bb12
}
}
bb11 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb12 = {
_6 = RET;
RET = _6;
_8.1 = &RET;
_12 = _7 as f64;
_7 = _3 << _3;
_12 = 247_u8 as f64;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 131_u8 as f64;
_11 = (-3651122563638789313_i64) as i16;
(*_2) = 68_u8 as f64;
(*_2) = 144987118257922234699536836311618865165_i128 as f64;
(*_2) = 17422075417074694140_usize as f64;
_3 = _7 << _7;
(*_2) = 231741617849402977736963009379108161787_u128 as f64;
(*_2) = 1050208876_i32 as f64;
_8.0 = core::ptr::addr_of_mut!((*_2));
_7 = -_3;
(*_2) = (-3375438853764895654_i64) as f64;
Goto(bb3)
}
bb13 = {
(*_2) = _14 * _14;
(*_2) = _11 as f64;
(*_15) = !_26;
(*_15) = _26 | _26;
(*_15) = _11 as isize;
(*_15) = _6 as isize;
(*_15) = _26 >> _22;
_30 = 48956571_i32 as i64;
(*_15) = _3;
_20 = core::ptr::addr_of_mut!((*_20));
(*_15) = _21 ^ _26;
_5 = &_27;
(*_2) = -_14;
(*_15) = -_28;
_29.1 = Adt17::Variant2 { fld0: _14,fld1: 39551_u16,fld2: 101_u8 };
(*_15) = !_7;
_8.0 = Move(_2);
RET = _18;
place!(Field::<f64>(Variant(_29.1, 2), 0)) = _14 + _12;
_2 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_29.1, 2), 0)));
_2 = Move(_8.0);
_3 = (*_15) + (*_15);
_6 = _29.2.fld0;
_29.2 = Adt34 { fld0: _18 };
_29.1 = Adt17::Variant2 { fld0: _14,fld1: 38157_u16,fld2: 33_u8 };
match _19 {
0 => bb12,
1 => bb10,
62 => bb14,
_ => bb5
}
}
bb14 = {
(*_15) = _26 | _26;
_30 = (-5352851631432690990_i64);
_22 = 68699680246282474912024594524649466082_i128;
(*_15) = _18 as isize;
(*_20) = core::ptr::addr_of!(_39);
_3 = _28 ^ _28;
(*_1) = true ^ true;
(*_20) = core::ptr::addr_of!((*_1));
_34 = Field::<f64>(Variant(_29.1, 2), 0) * Field::<f64>(Variant(_29.1, 2), 0);
(*_1) = true;
_4 = &(*_5);
(*_15) = _26;
_19 = (-80_i8);
(*_15) = 15118113083317489903_usize as isize;
_5 = Move(_4);
(*_1) = !true;
_9 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_29.1, 2), 1)));
(*_1) = Field::<f64>(Variant(_29.1, 2), 0) > _12;
(*_9) = 19461_u16 | 5456_u16;
(*_15) = _6 as isize;
_6 = _29.2.fld0;
(*_9) = !3255_u16;
_12 = -_34;
(*_1) = _28 == _3;
_29.1 = Adt17::Variant0 { fld0: 2793719624_u32,fld1: _18,fld2: _26,fld3: 2_usize,fld4: _11,fld5: 96_u8,fld6: 264471498680067089424653149388926539829_u128 };
(*_1) = _26 == _7;
(*_15) = _3;
match _19 {
0 => bb1,
1 => bb10,
2 => bb4,
3 => bb15,
340282366920938463463374607431768211376 => bb17,
_ => bb16
}
}
bb15 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb16 = {
_6 = RET;
RET = _6;
_8.1 = &RET;
_12 = _7 as f64;
_7 = _3 << _3;
_12 = 247_u8 as f64;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 131_u8 as f64;
_11 = (-3651122563638789313_i64) as i16;
(*_2) = 68_u8 as f64;
(*_2) = 144987118257922234699536836311618865165_i128 as f64;
(*_2) = 17422075417074694140_usize as f64;
_3 = _7 << _7;
(*_2) = 231741617849402977736963009379108161787_u128 as f64;
(*_2) = 1050208876_i32 as f64;
_8.0 = core::ptr::addr_of_mut!((*_2));
_7 = -_3;
(*_2) = (-3375438853764895654_i64) as f64;
Goto(bb3)
}
bb17 = {
(*_1) = !true;
(*_15) = _6 as isize;
_40 = _27 - _27;
(*_15) = _28 >> _3;
(*_15) = Field::<isize>(Variant(_29.1, 0), 2) << Field::<isize>(Variant(_29.1, 0), 2);
(*_15) = _18 as isize;
(*_1) = _21 != _3;
_19 = (-126_i8) << _7;
_19 = 56_i8;
place!(Field::<isize>(Variant(_29.1, 0), 2)) = _28 * _3;
place!(Field::<usize>(Variant(_29.1, 0), 3)) = 2135183405045917844_usize;
(*_20) = core::ptr::addr_of!((*_1));
(*_1) = false;
(*_15) = !Field::<isize>(Variant(_29.1, 0), 2);
_12 = _34 + _34;
Goto(bb18)
}
bb18 = {
_16 = &place!(Field::<usize>(Variant(_29.1, 0), 3));
(*_1) = (*_15) < (*_15);
_47 = (*_1) > (*_1);
(*_15) = Field::<isize>(Variant(_29.1, 0), 2) | Field::<isize>(Variant(_29.1, 0), 2);
_39 = _47;
Goto(bb19)
}
bb19 = {
_45 = &mut _6;
_29.1 = Adt17::Variant0 { fld0: 522413615_u32,fld1: (*_45),fld2: (*_15),fld3: 11920873751945483797_usize,fld4: _11,fld5: 203_u8,fld6: 293475362566386461304535762442078155767_u128 };
_45 = &mut _18;
_20 = core::ptr::addr_of_mut!((*_20));
_2 = core::ptr::addr_of_mut!(_14);
(*_2) = _12 + _12;
(*_20) = core::ptr::addr_of!((*_1));
_17 = core::ptr::addr_of!(_30);
(*_17) = 585159803185726662_i64;
(*_20) = core::ptr::addr_of!((*_1));
_37 = (*_45);
(*_15) = _3;
(*_1) = _47;
(*_17) = (*_1) as i64;
match _22 {
68699680246282474912024594524649466082 => bb20,
_ => bb6
}
}
bb20 = {
(*_2) = _34;
(*_20) = core::ptr::addr_of!(_47);
(*_15) = Field::<isize>(Variant(_29.1, 0), 2) >> (*_17);
_12 = 106_u8 as f64;
_4 = &_40;
(*_15) = _28;
_29.2.fld0 = Field::<char>(Variant(_29.1, 0), 1);
(*_15) = -_26;
(*_2) = _34 - _34;
(*_2) = _34 - _34;
match _22 {
0 => bb19,
68699680246282474912024594524649466082 => bb22,
_ => bb21
}
}
bb21 = {
_7 = 245839088503080572499992458301274968231_u128 as isize;
_8.0 = Move(_2);
_8.1 = &_6;
RET = _6;
_6 = RET;
_8.1 = &RET;
_2 = Move(_8.0);
_3 = !_7;
_7 = _3;
_8.0 = Move(_2);
_6 = RET;
_7 = _3 + _3;
_2 = Move(_8.0);
_7 = _3 + _3;
_6 = RET;
_8.0 = Move(_2);
_3 = !_7;
RET = _6;
_6 = RET;
RET = _6;
_8.1 = &_6;
Call(_11 = fn18(Move(_8), Move(_1), _3, _3, _3, _7, _6, _7, RET, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb22 = {
_12 = (*_2);
_3 = (*_15) + (*_15);
RET = _37;
(*_15) = _28;
(*_15) = _19 as isize;
(*_17) = 2707934445279000262_i64 & (-646926687456703807_i64);
(*_20) = core::ptr::addr_of!((*_1));
_50 = core::ptr::addr_of_mut!(_8.0);
_56 = _22 as f64;
(*_15) = (*_17) as isize;
_57 = _22;
(*_20) = core::ptr::addr_of!((*_1));
(*_50) = Move(_2);
(*_20) = core::ptr::addr_of!((*_1));
(*_1) = _39 != _39;
_12 = -_34;
(*_1) = !_39;
_24 = (*_4) * (*_4);
(*_20) = core::ptr::addr_of!((*_1));
place!(Field::<isize>(Variant(_29.1, 0), 2)) = _3 * _26;
(*_20) = core::ptr::addr_of!((*_1));
(*_1) = _39;
(*_17) = (-5950416824275823818_i64);
Goto(bb23)
}
bb23 = {
place!(Field::<i16>(Variant(_29.1, 0), 4)) = _11 | _11;
(*_20) = core::ptr::addr_of!((*_1));
(*_20) = core::ptr::addr_of!(_39);
(*_17) = 217508063767621375_i64 << (*_15);
_36 = (*_45);
(*_50) = core::ptr::addr_of_mut!(_34);
_29.2 = Adt34 { fld0: RET };
(*_1) = !_47;
(*_1) = (*_15) == (*_15);
(*_50) = core::ptr::addr_of_mut!(_14);
place!(Field::<u8>(Variant(_29.1, 0), 5)) = 25_u8 >> _3;
(*_50) = core::ptr::addr_of_mut!(_34);
place!(Field::<usize>(Variant(_29.1, 0), 3)) = Field::<isize>(Variant(_29.1, 0), 2) as usize;
_19 = !(-77_i8);
_8.1 = &(*_45);
_59 = !59159_u16;
_47 = _28 >= _3;
(*_17) = (-4133646364824489915_i64) & 1092846661550319504_i64;
(*_1) = _47;
(*_15) = (*_1) as isize;
Goto(bb24)
}
bb24 = {
(*_50) = core::ptr::addr_of_mut!(_14);
_3 = _13;
(*_20) = core::ptr::addr_of!(_39);
_21 = (*_15);
_69 = core::ptr::addr_of!(_9);
(*_20) = core::ptr::addr_of!((*_1));
_21 = (*_15) | (*_15);
(*_17) = 8570032078855521926_i64 >> (*_15);
(*_50) = core::ptr::addr_of_mut!(_34);
(*_17) = 4303799276216814955_i64 >> (*_15);
(*_20) = core::ptr::addr_of!((*_1));
(*_15) = _28;
(*_69) = core::ptr::addr_of_mut!(_59);
place!(Field::<i16>(Variant(_29.1, 0), 4)) = 17082377727914938459_u64 as i16;
(*_17) = (*_9) as i64;
_72.3 = Adt50 { fld0: 3141250907297068183754818069363243201_u128 };
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_15) = _28 ^ Field::<isize>(Variant(_29.1, 0), 2);
match _57 {
0 => bb21,
1 => bb2,
68699680246282474912024594524649466082 => bb25,
_ => bb6
}
}
bb25 = {
(*_69) = core::ptr::addr_of_mut!((*_9));
_71.fld0 = !(*_15);
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_69) = core::ptr::addr_of_mut!((*_9));
_8.0 = core::ptr::addr_of_mut!(_34);
(*_17) = 5219306084613360957_i64;
_64 = _34;
(*_69) = core::ptr::addr_of_mut!((*_9));
_62 = !(*_15);
(*_50) = core::ptr::addr_of_mut!(_12);
_22 = _57 ^ _57;
(*_69) = core::ptr::addr_of_mut!((*_9));
_62 = (*_15) ^ Field::<isize>(Variant(_29.1, 0), 2);
_15 = core::ptr::addr_of!((*_15));
(*_20) = core::ptr::addr_of!((*_1));
Goto(bb26)
}
bb26 = {
(*_20) = core::ptr::addr_of!(_61);
_72.3 = Adt50 { fld0: 317465638048344187571120625950509743392_u128 };
(*_17) = !7577999820713854164_i64;
(*_1) = (*_15) > (*_15);
_23 = &mut place!(Field::<usize>(Variant(_29.1, 0), 3));
_73 = _57;
_60 = (*_45);
(*_17) = _57 as i64;
_31 = (*_23);
(*_9) = 13845996_i32 as u16;
_55 = _36;
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_1) = (*_15) > (*_15);
(*_50) = core::ptr::addr_of_mut!(_12);
_68 = !(*_9);
(*_15) = -_3;
_70 = &mut _72.3.fld0;
(*_17) = 5185014787503586490_i64 | 1067215898713866972_i64;
(*_1) = !_39;
_36 = (*_45);
(*_9) = !_68;
_35 = &mut (*_70);
(*_69) = core::ptr::addr_of_mut!((*_9));
match _57 {
0 => bb27,
1 => bb28,
2 => bb29,
68699680246282474912024594524649466082 => bb31,
_ => bb30
}
}
bb27 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb28 = {
_6 = RET;
RET = _6;
_8.1 = &RET;
_12 = _7 as f64;
_7 = _3 << _3;
_12 = 247_u8 as f64;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 131_u8 as f64;
_11 = (-3651122563638789313_i64) as i16;
(*_2) = 68_u8 as f64;
(*_2) = 144987118257922234699536836311618865165_i128 as f64;
(*_2) = 17422075417074694140_usize as f64;
_3 = _7 << _7;
(*_2) = 231741617849402977736963009379108161787_u128 as f64;
(*_2) = 1050208876_i32 as f64;
_8.0 = core::ptr::addr_of_mut!((*_2));
_7 = -_3;
(*_2) = (-3375438853764895654_i64) as f64;
Goto(bb3)
}
bb29 = {
place!(Field::<i16>(Variant(_29.1, 0), 4)) = _11 | _11;
(*_20) = core::ptr::addr_of!((*_1));
(*_20) = core::ptr::addr_of!(_39);
(*_17) = 217508063767621375_i64 << (*_15);
_36 = (*_45);
(*_50) = core::ptr::addr_of_mut!(_34);
_29.2 = Adt34 { fld0: RET };
(*_1) = !_47;
(*_1) = (*_15) == (*_15);
(*_50) = core::ptr::addr_of_mut!(_14);
place!(Field::<u8>(Variant(_29.1, 0), 5)) = 25_u8 >> _3;
(*_50) = core::ptr::addr_of_mut!(_34);
place!(Field::<usize>(Variant(_29.1, 0), 3)) = Field::<isize>(Variant(_29.1, 0), 2) as usize;
_19 = !(-77_i8);
_8.1 = &(*_45);
_59 = !59159_u16;
_47 = _28 >= _3;
(*_17) = (-4133646364824489915_i64) & 1092846661550319504_i64;
(*_1) = _47;
(*_15) = (*_1) as isize;
Goto(bb24)
}
bb30 = {
_7 = 245839088503080572499992458301274968231_u128 as isize;
_8.0 = Move(_2);
_8.1 = &_6;
RET = _6;
_6 = RET;
_8.1 = &RET;
_2 = Move(_8.0);
_3 = !_7;
_7 = _3;
_8.0 = Move(_2);
_6 = RET;
_7 = _3 + _3;
_2 = Move(_8.0);
_7 = _3 + _3;
_6 = RET;
_8.0 = Move(_2);
_3 = !_7;
RET = _6;
_6 = RET;
RET = _6;
_8.1 = &_6;
Call(_11 = fn18(Move(_8), Move(_1), _3, _3, _3, _7, _6, _7, RET, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb31 = {
_45 = &mut _55;
(*_1) = _3 >= (*_15);
(*_20) = core::ptr::addr_of!((*_1));
_17 = core::ptr::addr_of!((*_17));
(*_15) = _21 ^ _62;
(*_45) = RET;
_17 = core::ptr::addr_of!((*_17));
(*_35) = 106153145886243528516514595509788612138_u128 & 268473998053189344983692318455721337474_u128;
(*_1) = (*_23) > _31;
(*_17) = 4942823837482341226_i64 | (-1531305256206598723_i64);
_48 = _64 as u64;
_34 = 3973709920_u32 as f64;
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_50) = core::ptr::addr_of_mut!(_34);
_66 = -(*_4);
(*_23) = _31 << (*_15);
(*_1) = (*_23) < (*_23);
_71.fld1 = core::ptr::addr_of_mut!(_67);
(*_23) = _31 ^ _31;
(*_15) = _71.fld0 >> _3;
(*_9) = _12 as u16;
(*_17) = (-7879041831016051975_i64) | 2261817690696817281_i64;
(*_35) = 258865086107381251401081193867268847519_u128;
(*_1) = !_39;
(*_15) = _26 & _26;
(*_50) = core::ptr::addr_of_mut!(_80);
_46 = [(*_45),(*_45),_60,(*_45),(*_45),(*_45),_36];
_12 = _64 - _14;
_27 = (*_4);
Goto(bb32)
}
bb32 = {
(*_23) = _40 as usize;
(*_1) = _39 <= _47;
(*_69) = core::ptr::addr_of_mut!((*_9));
_23 = &mut _31;
(*_35) = (*_23) as u128;
_71.fld1 = core::ptr::addr_of_mut!(_56);
(*_1) = _47;
(*_15) = _3 << (*_23);
(*_9) = _68 * _68;
_88.0 = &mut (*_45);
_43 = &mut (*_23);
_44 = Adt19::Variant1 { fld0: Move(_17) };
(*_15) = _3 ^ _3;
(*_9) = !_68;
(*_20) = core::ptr::addr_of!((*_1));
(*_43) = 698591851437451648_usize;
match (*_43) {
0 => bb11,
698591851437451648 => bb33,
_ => bb20
}
}
bb33 = {
(*_43) = (*_9) as usize;
_23 = Move(_43);
(*_9) = _19 as u16;
(*_1) = _47 ^ _47;
_50 = core::ptr::addr_of_mut!((*_50));
_11 = (-30003_i16) + 22367_i16;
(*_9) = _68 - _68;
_54 = core::ptr::addr_of!(_89);
_9 = core::ptr::addr_of_mut!((*_9));
(*_54) = RET as isize;
(*_9) = _68 & _68;
(*_54) = (*_15) >> (*_15);
(*_20) = core::ptr::addr_of!((*_1));
(*_35) = 288096302852267985139598798029743837914_u128 - 329912202056562562341168753541820364022_u128;
(*_20) = core::ptr::addr_of!((*_1));
match _57 {
68699680246282474912024594524649466082 => bb35,
_ => bb34
}
}
bb34 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb35 = {
(*_54) = (*_35) as isize;
_57 = _22 << (*_15);
_88.2 = Adt40 { fld0: _26,fld1: Move(_71.fld1) };
(*_20) = core::ptr::addr_of!((*_1));
(*_54) = _28 - _3;
(*_35) = 85392448344043780798250535542150260116_u128;
(*_1) = _39;
(*_1) = _26 <= (*_15);
(*_9) = _68;
(*_35) = 98013085342306298070690676361593831750_u128 << (*_15);
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_54) = 275064196_i32 as isize;
(*_35) = 61900475989457354060830646373247483613_u128 >> (*_9);
_28 = _30 as isize;
(*_54) = (*_15) & (*_15);
(*_20) = core::ptr::addr_of!((*_1));
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_1) = (*_54) <= (*_15);
_51 = _19;
(*_9) = _68;
(*_15) = (*_54) ^ (*_54);
(*_9) = _68 & _68;
(*_15) = (*_1) as isize;
(*_35) = 148355732777489057508997245838970725648_u128 - 57700849026450930727271623054229383360_u128;
(*_9) = _68;
(*_54) = (*_15) >> (*_15);
_70 = Move(_35);
match _73 {
0 => bb36,
1 => bb37,
2 => bb38,
68699680246282474912024594524649466082 => bb40,
_ => bb39
}
}
bb36 = {
_16 = &place!(Field::<usize>(Variant(_29.1, 0), 3));
(*_1) = (*_15) < (*_15);
_47 = (*_1) > (*_1);
(*_15) = Field::<isize>(Variant(_29.1, 0), 2) | Field::<isize>(Variant(_29.1, 0), 2);
_39 = _47;
Goto(bb19)
}
bb37 = {
(*_20) = core::ptr::addr_of!(_61);
_72.3 = Adt50 { fld0: 317465638048344187571120625950509743392_u128 };
(*_17) = !7577999820713854164_i64;
(*_1) = (*_15) > (*_15);
_23 = &mut place!(Field::<usize>(Variant(_29.1, 0), 3));
_73 = _57;
_60 = (*_45);
(*_17) = _57 as i64;
_31 = (*_23);
(*_9) = 13845996_i32 as u16;
_55 = _36;
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_1) = (*_15) > (*_15);
(*_50) = core::ptr::addr_of_mut!(_12);
_68 = !(*_9);
(*_15) = -_3;
_70 = &mut _72.3.fld0;
(*_17) = 5185014787503586490_i64 | 1067215898713866972_i64;
(*_1) = !_39;
_36 = (*_45);
(*_9) = !_68;
_35 = &mut (*_70);
(*_69) = core::ptr::addr_of_mut!((*_9));
match _57 {
0 => bb27,
1 => bb28,
2 => bb29,
68699680246282474912024594524649466082 => bb31,
_ => bb30
}
}
bb38 = {
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (*_2) * (*_2);
(*_2) = _14 * _14;
(*_2) = _14;
(*_2) = _14 - _14;
(*_2) = _14 * _14;
_18 = _6;
(*_2) = -_14;
(*_2) = 5896621356109305705_u64 as f64;
_20 = core::ptr::addr_of_mut!(_1);
RET = _6;
_3 = _7 * _7;
_8.1 = &_6;
_21 = -_3;
_2 = core::ptr::addr_of_mut!((*_2));
_3 = _21 & _7;
(*_2) = -_14;
(*_2) = _14 * _14;
_15 = core::ptr::addr_of!(_13);
(*_2) = _14 * _14;
_19 = 62_i8;
(*_15) = _3;
match _19 {
62 => bb4,
_ => bb1
}
}
bb39 = {
place!(Field::<i16>(Variant(_29.1, 0), 4)) = _11 | _11;
(*_20) = core::ptr::addr_of!((*_1));
(*_20) = core::ptr::addr_of!(_39);
(*_17) = 217508063767621375_i64 << (*_15);
_36 = (*_45);
(*_50) = core::ptr::addr_of_mut!(_34);
_29.2 = Adt34 { fld0: RET };
(*_1) = !_47;
(*_1) = (*_15) == (*_15);
(*_50) = core::ptr::addr_of_mut!(_14);
place!(Field::<u8>(Variant(_29.1, 0), 5)) = 25_u8 >> _3;
(*_50) = core::ptr::addr_of_mut!(_34);
place!(Field::<usize>(Variant(_29.1, 0), 3)) = Field::<isize>(Variant(_29.1, 0), 2) as usize;
_19 = !(-77_i8);
_8.1 = &(*_45);
_59 = !59159_u16;
_47 = _28 >= _3;
(*_17) = (-4133646364824489915_i64) & 1092846661550319504_i64;
(*_1) = _47;
(*_15) = (*_1) as isize;
Goto(bb24)
}
bb40 = {
_48 = 4891045497068659845_u64 ^ 8448248216490998413_u64;
_88.3 = 6_usize * 2038345279364137966_usize;
(*_20) = core::ptr::addr_of!((*_1));
(*_69) = core::ptr::addr_of_mut!((*_9));
_8.0 = core::ptr::addr_of_mut!(_67);
(*_54) = -(*_15);
_90.fld0 = _60;
(*_15) = _19 as isize;
(*_54) = -_26;
_74 = _3;
(*_50) = core::ptr::addr_of_mut!(_14);
(*_9) = _68 >> (*_54);
(*_20) = core::ptr::addr_of!((*_1));
_4 = &_24;
_26 = _12 as isize;
(*_20) = core::ptr::addr_of!((*_1));
(*_15) = (*_54);
_37 = RET;
(*_20) = core::ptr::addr_of!((*_1));
_8.1 = &_37;
(*_20) = core::ptr::addr_of!(_98);
_68 = (*_9);
Goto(bb41)
}
bb41 = {
_88.3 = 5_usize & 4442154742668101977_usize;
(*_20) = core::ptr::addr_of!((*_1));
(*_1) = _64 <= _12;
_74 = (*_15) - (*_15);
_49 = &_69;
(*_1) = !_61;
_22 = -_57;
_7 = (*_54);
_87 = _12 - _64;
_83 = core::ptr::addr_of_mut!(_20);
_101.fld2.2 = Adt17::Variant0 { fld0: 1209914947_u32,fld1: RET,fld2: (*_15),fld3: _88.3,fld4: _11,fld5: 68_u8,fld6: 236739283714717404723193359238684228662_u128 };
(*_54) = !(*_15);
(*_83) = core::ptr::addr_of_mut!((*_20));
Goto(bb42)
}
bb42 = {
(*_50) = core::ptr::addr_of_mut!(_80);
_100 = &_54;
(*_20) = core::ptr::addr_of!(_98);
_91 = Adt19::Variant3 { fld0: (*_1),fld1: _88.3,fld2: 1205587574_u32,fld3: _19,fld4: Move(Field::<*const i64>(Variant(_44, 1), 0)),fld5: 1004756636_i32,fld6: _30,fld7: (*_9) };
(*_20) = core::ptr::addr_of!((*_1));
_67 = 214192428755103066548557824188025661321_u128 as f64;
_26 = !(*_15);
_104 = (*_1);
Goto(bb43)
}
bb43 = {
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_83) = core::ptr::addr_of_mut!((*_20));
place!(Field::<u32>(Variant(_101.fld2.2, 0), 0)) = 3763097949_u32 + 2414452008_u32;
(*_69) = core::ptr::addr_of_mut!((*_9));
_15 = core::ptr::addr_of!((*_15));
(*_20) = core::ptr::addr_of!((*_1));
(*_15) = (*_54);
_59 = Field::<u16>(Variant(_91, 3), 7);
_84 = (*_15) ^ (*_15);
(*_15) = (*_54);
(*_69) = core::ptr::addr_of_mut!((*_9));
place!(Field::<i32>(Variant(_91, 3), 5)) = !(-1097472612_i32);
(*_9) = Field::<u16>(Variant(_91, 3), 7) >> _89;
(*_1) = (*_9) != (*_9);
(*_9) = _30 as u16;
(*_1) = _39;
(*_20) = core::ptr::addr_of!((*_1));
(*_50) = core::ptr::addr_of_mut!(_14);
(*_1) = _3 == (*_54);
_101.fld2.0 = Field::<u32>(Variant(_101.fld2.2, 0), 0) | Field::<u32>(Variant(_101.fld2.2, 0), 0);
place!(Field::<i32>(Variant(_91, 3), 5)) = Field::<u32>(Variant(_101.fld2.2, 0), 0) as i32;
(*_15) = -(*_54);
(*_50) = core::ptr::addr_of_mut!(_56);
place!(Field::<i16>(Variant(_101.fld2.2, 0), 4)) = _11 + _11;
match _73 {
68699680246282474912024594524649466082 => bb45,
_ => bb44
}
}
bb44 = {
(*_20) = core::ptr::addr_of!(_61);
_72.3 = Adt50 { fld0: 317465638048344187571120625950509743392_u128 };
(*_17) = !7577999820713854164_i64;
(*_1) = (*_15) > (*_15);
_23 = &mut place!(Field::<usize>(Variant(_29.1, 0), 3));
_73 = _57;
_60 = (*_45);
(*_17) = _57 as i64;
_31 = (*_23);
(*_9) = 13845996_i32 as u16;
_55 = _36;
(*_69) = core::ptr::addr_of_mut!((*_9));
(*_1) = (*_15) > (*_15);
(*_50) = core::ptr::addr_of_mut!(_12);
_68 = !(*_9);
(*_15) = -_3;
_70 = &mut _72.3.fld0;
(*_17) = 5185014787503586490_i64 | 1067215898713866972_i64;
(*_1) = !_39;
_36 = (*_45);
(*_9) = !_68;
_35 = &mut (*_70);
(*_69) = core::ptr::addr_of_mut!((*_9));
match _57 {
0 => bb27,
1 => bb28,
2 => bb29,
68699680246282474912024594524649466082 => bb31,
_ => bb30
}
}
bb45 = {
(*_69) = core::ptr::addr_of_mut!((*_9));
_71.fld1 = Move((*_50));
(*_1) = _47;
_17 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_91, 3), 6)));
_80 = _87 - _87;
_88 = (Move(_45), (*_4), Move(_71), Field::<usize>(Variant(_101.fld2.2, 0), 3));
(*_20) = core::ptr::addr_of!((*_1));
_90 = Adt34 { fld0: _36 };
_101.fld6.3.1 = 204526807501028250869591956857741512721_u128 as u32;
(*_1) = !_61;
(*_83) = core::ptr::addr_of_mut!((*_20));
(*_15) = 203_u8 as isize;
(*_9) = !_68;
_53 = core::ptr::addr_of_mut!((*_9));
(*_69) = Move(_53);
_108 = (*_54) as f32;
_100 = &_15;
(*_69) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_91, 3), 7)));
(*_69) = core::ptr::addr_of_mut!((*_9));
_43 = &mut place!(Field::<usize>(Variant(_101.fld2.2, 0), 3));
(*_20) = core::ptr::addr_of!((*_1));
Call((*_9) = core::intrinsics::bswap(_59), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
(*_50) = Move(_88.2.fld1);
_96 = (*_54) << (*_9);
_88.0 = &mut _37;
(*_69) = core::ptr::addr_of_mut!((*_9));
_76 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_91, 3), 2)));
(*_9) = _59 * _68;
_27 = -(*_4);
_102 = Adt40 { fld0: _7,fld1: Move((*_50)) };
(*_1) = _47 | _61;
_2 = Move(_102.fld1);
(*_76) = 1796367244_u32 & 3222919247_u32;
(*_50) = Move(_2);
_100 = &_54;
Goto(bb47)
}
bb47 = {
_5 = Move(_4);
_7 = (*_54) * (*_54);
_69 = core::ptr::addr_of!((*_69));
(*_50) = core::ptr::addr_of_mut!(_34);
(*_15) = _3 | _7;
(*_43) = Field::<usize>(Variant(_91, 3), 1) - Field::<usize>(Variant(_91, 3), 1);
(*_76) = _12 as u32;
_61 = !(*_1);
(*_54) = (*_15) << (*_15);
(*_83) = core::ptr::addr_of_mut!((*_20));
_113 = Adt19::Variant3 { fld0: (*_1),fld1: (*_43),fld2: (*_76),fld3: _19,fld4: Move(Field::<*const i64>(Variant(_91, 3), 4)),fld5: Field::<i32>(Variant(_91, 3), 5),fld6: (*_17),fld7: Field::<u16>(Variant(_91, 3), 7) };
(*_15) = _88.1 as isize;
(*_50) = core::ptr::addr_of_mut!(_94);
(*_83) = core::ptr::addr_of_mut!((*_20));
(*_17) = !_30;
(*_9) = _108 as u16;
place!(Field::<usize>(Variant(_91, 3), 1)) = _80 as usize;
(*_83) = core::ptr::addr_of_mut!((*_20));
(*_43) = Field::<usize>(Variant(_91, 3), 1) * Field::<usize>(Variant(_91, 3), 1);
(*_15) = -_84;
(*_17) = Field::<i64>(Variant(_113, 3), 6);
match _73 {
0 => bb44,
1 => bb18,
2 => bb35,
3 => bb48,
4 => bb49,
68699680246282474912024594524649466082 => bb51,
_ => bb50
}
}
bb48 = {
(*_50) = Move(_88.2.fld1);
_96 = (*_54) << (*_9);
_88.0 = &mut _37;
(*_69) = core::ptr::addr_of_mut!((*_9));
_76 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_91, 3), 2)));
(*_9) = _59 * _68;
_27 = -(*_4);
_102 = Adt40 { fld0: _7,fld1: Move((*_50)) };
(*_1) = _47 | _61;
_2 = Move(_102.fld1);
(*_76) = 1796367244_u32 & 3222919247_u32;
(*_50) = Move(_2);
_100 = &_54;
Goto(bb47)
}
bb49 = {
(*_2) = _34;
(*_20) = core::ptr::addr_of!(_47);
(*_15) = Field::<isize>(Variant(_29.1, 0), 2) >> (*_17);
_12 = 106_u8 as f64;
_4 = &_40;
(*_15) = _28;
_29.2.fld0 = Field::<char>(Variant(_29.1, 0), 1);
(*_15) = -_26;
(*_2) = _34 - _34;
(*_2) = _34 - _34;
match _22 {
0 => bb19,
68699680246282474912024594524649466082 => bb22,
_ => bb21
}
}
bb50 = {
_48 = 4891045497068659845_u64 ^ 8448248216490998413_u64;
_88.3 = 6_usize * 2038345279364137966_usize;
(*_20) = core::ptr::addr_of!((*_1));
(*_69) = core::ptr::addr_of_mut!((*_9));
_8.0 = core::ptr::addr_of_mut!(_67);
(*_54) = -(*_15);
_90.fld0 = _60;
(*_15) = _19 as isize;
(*_54) = -_26;
_74 = _3;
(*_50) = core::ptr::addr_of_mut!(_14);
(*_9) = _68 >> (*_54);
(*_20) = core::ptr::addr_of!((*_1));
_4 = &_24;
_26 = _12 as isize;
(*_20) = core::ptr::addr_of!((*_1));
(*_15) = (*_54);
_37 = RET;
(*_20) = core::ptr::addr_of!((*_1));
_8.1 = &_37;
(*_20) = core::ptr::addr_of!(_98);
_68 = (*_9);
Goto(bb41)
}
bb51 = {
_94 = _12 + _87;
_44 = Adt19::Variant0 { fld0: _47,fld1: _22,fld2: _48 };
_110 = [RET];
_38 = core::ptr::addr_of_mut!(_112);
(*_17) = _11 as i64;
place!(Field::<i32>(Variant(_113, 3), 5)) = Field::<i32>(Variant(_91, 3), 5);
_88.2.fld1 = core::ptr::addr_of_mut!(_67);
(*_54) = (*_15) * (*_15);
(*_76) = Field::<u32>(Variant(_113, 3), 2) - Field::<u32>(Variant(_113, 3), 2);
(*_20) = core::ptr::addr_of!((*_1));
_81 = Move((*_50));
(*_50) = core::ptr::addr_of_mut!(_94);
(*_83) = core::ptr::addr_of_mut!((*_20));
_19 = Field::<i8>(Variant(_91, 3), 3);
(*_1) = !_47;
(*_15) = -(*_54);
(*_54) = !_88.2.fld0;
(*_43) = Field::<usize>(Variant(_91, 3), 1) + Field::<usize>(Variant(_91, 3), 1);
Goto(bb52)
}
bb52 = {
Call(_121 = dump_var(Move(_98), Move(_13), Move(_21), Move(_51)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_121 = dump_var(Move(_36), Move(_73), Move(_39), Move(_48)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_121 = dump_var(Move(_30), Move(_37), Move(_104), Move(_110)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_121 = dump_var(Move(_55), Move(_74), Move(_61), Move(_28)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_121 = dump_var(Move(_11), _122, _122, _122), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: (*mut f64, &'static char),mut _2: *const bool,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: char,mut _8: isize,mut _9: char,mut _10: isize,mut _11: char,mut _12: isize) -> i16 {
mir! {
type RET = i16;
let _13: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _14: f32;
let _15: i8;
let _16: &'static i32;
let _17: [u32; 6];
let _18: f64;
let _19: (usize, bool);
let _20: Adt17;
let _21: &'static mut u128;
let _22: &'static char;
let _23: Adt17;
let _24: i128;
let _25: bool;
let _26: isize;
let _27: *const Adt17;
let _28: bool;
let _29: bool;
let _30: isize;
let _31: u64;
let _32: i64;
let _33: isize;
let _34: &'static mut &'static *mut bool;
let _35: &'static *const isize;
let _36: &'static usize;
let _37: &'static mut usize;
let _38: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _39: *mut f64;
let _40: u8;
let _41: i64;
let _42: isize;
let _43: usize;
let _44: [u16; 1];
let _45: *const isize;
let _46: bool;
let _47: *const Adt19;
let _48: *const (&'static char,);
let _49: (&'static mut usize, (*mut u16, Adt17), (*mut f64, &'static char), [u64; 6]);
let _50: char;
let _51: char;
let _52: [char; 6];
let _53: ((&'static char,), Adt17, Adt34, &'static mut u128);
let _54: ();
let _55: ();
{
_10 = _8;
Call(_2 = fn19(_4, Move(_1.0), _11, _10, _9, _9, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
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
RET = 2784_i16;
_4 = -_3;
_13.3 = Adt50 { fld0: 194968153056670442989408304033824421166_u128 };
_13.2 = core::ptr::addr_of!(_6);
_9 = _7;
_13.2 = core::ptr::addr_of!(_5);
_15 = 65459_u16 as i8;
_5 = (-385598051007139718_i64) as isize;
_12 = !_10;
_4 = _10 << _6;
_13.2 = core::ptr::addr_of!(_4);
_8 = _15 as isize;
_12 = _10 >> _4;
_14 = (-13470036415962963476520101003366504103_i128) as f32;
_8 = _13.3.fld0 as isize;
_17 = [64249123_u32,2947339010_u32,1491025139_u32,242310270_u32,1791615784_u32,294305765_u32];
_4 = _10;
_6 = -_12;
_13.1 = [16164522143822545742_u64,1038696799919592216_u64,9594448836332412566_u64,14430603168679230689_u64,4044091574648763491_u64,12654384941017893660_u64];
_10 = -_12;
_3 = RET as isize;
_13.3 = Adt50 { fld0: 170865080140437214106421344793594041177_u128 };
_6 = 7_u8 as isize;
_3 = -_12;
_1.1 = &_9;
_5 = _12 + _3;
_14 = 6_usize as f32;
match RET {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
2784 => bb11,
_ => bb10
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
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
_ => bb4
}
}
bb10 = {
Return()
}
bb11 = {
_3 = _10 | _5;
_9 = _11;
_11 = _9;
_7 = _11;
_9 = _11;
_13.3 = Adt50 { fld0: 241975312456050300472654883378710973574_u128 };
_1.1 = &_11;
_15 = (-6_i8) << _10;
_18 = 6484_u16 as f64;
Goto(bb12)
}
bb12 = {
_1.1 = &_7;
_9 = _11;
_19.0 = 3015204500920809101_usize;
_15 = (-80_i8);
_7 = _9;
_3 = _10;
match _19.0 {
0 => bb4,
1 => bb2,
2 => bb13,
3 => bb14,
3015204500920809101 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
_ => bb4
}
}
bb15 = {
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
_ => bb4
}
}
bb16 = {
RET = 4712493716609871257_u64 as i16;
_6 = -_5;
_9 = _11;
_9 = _11;
_14 = RET as f32;
_22 = &_7;
_19.1 = true;
_2 = core::ptr::addr_of!(_19.1);
_22 = Move(_1.1);
Goto(bb17)
}
bb17 = {
_9 = _11;
_4 = _13.3.fld0 as isize;
Call(_13.3.fld0 = core::intrinsics::bswap(139632705585417384380215296631722157586_u128), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_1.0 = core::ptr::addr_of_mut!(_18);
(*_2) = false;
_22 = &_9;
_17 = [2857066487_u32,2057063496_u32,1412058697_u32,2350751337_u32,1812271053_u32,1189792524_u32];
(*_2) = true | true;
(*_2) = _5 != _5;
(*_2) = !true;
_11 = (*_22);
_1.1 = Move(_22);
_9 = _11;
_9 = _7;
(*_2) = _7 != _11;
_14 = RET as f32;
_10 = _19.0 as isize;
_11 = _7;
_8 = _6 >> _5;
(*_2) = true;
_18 = 54268_u16 as f64;
(*_2) = false & false;
match _19.0 {
0 => bb3,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
3015204500920809101 => bb25,
_ => bb24
}
}
bb19 = {
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
_ => bb4
}
}
bb20 = {
RET = 4712493716609871257_u64 as i16;
_6 = -_5;
_9 = _11;
_9 = _11;
_14 = RET as f32;
_22 = &_7;
_19.1 = true;
_2 = core::ptr::addr_of!(_19.1);
_22 = Move(_1.1);
Goto(bb17)
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
(*_2) = _3 != _8;
(*_2) = _5 >= _5;
Goto(bb26)
}
bb26 = {
(*_2) = _5 <= _8;
_19.1 = !false;
_23 = Adt17::Variant2 { fld0: _18,fld1: 7382_u16,fld2: 66_u8 };
(*_2) = _5 > _3;
_23 = Adt17::Variant1 { fld0: 121450452511603103745495584369935614139_i128,fld1: 17340605378625734020_u64,fld2: _14 };
_25 = (*_2) <= (*_2);
_22 = &_11;
place!(Field::<u64>(Variant(_23, 1), 1)) = (-116370052508997735518505224820986689651_i128) as u64;
_3 = _8;
_24 = Field::<u64>(Variant(_23, 1), 1) as i128;
Goto(bb27)
}
bb27 = {
_27 = core::ptr::addr_of!(_20);
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: Field::<f32>(Variant(_23, 1), 2) };
(*_2) = _25;
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: _14 };
place!(Field::<u64>(Variant((*_27), 1), 1)) = Field::<u64>(Variant(_23, 1), 1) & Field::<u64>(Variant(_23, 1), 1);
_25 = (*_2) | (*_2);
_21 = &mut _13.3.fld0;
RET = (-29022_i16);
(*_21) = !174507454317620944231790544222661546296_u128;
place!(Field::<f32>(Variant((*_27), 1), 2)) = _14 - _14;
place!(Field::<u64>(Variant((*_27), 1), 1)) = 1128060135_u32 as u64;
place!(Field::<f32>(Variant((*_27), 1), 2)) = _14 + Field::<f32>(Variant(_23, 1), 2);
_19 = Checked(3_usize - 3_usize);
_25 = _6 <= _6;
(*_21) = 161554052466525624372925548248748483820_u128 << _8;
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: _14 };
place!(Field::<u64>(Variant((*_27), 1), 1)) = !Field::<u64>(Variant(_23, 1), 1);
Goto(bb28)
}
bb28 = {
_12 = _3 * _3;
place!(Field::<f32>(Variant((*_27), 1), 2)) = Field::<f32>(Variant(_23, 1), 2) + _14;
(*_2) = _25;
_26 = 15583_u16 as isize;
(*_27) = Adt17::Variant0 { fld0: 556039230_u32,fld1: (*_22),fld2: _12,fld3: _19.0,fld4: RET,fld5: 244_u8,fld6: (*_21) };
place!(Field::<i16>(Variant((*_27), 0), 4)) = RET & RET;
(*_21) = !Field::<u128>(Variant((*_27), 0), 6);
(*_2) = Field::<isize>(Variant((*_27), 0), 2) <= _3;
place!(Field::<char>(Variant(_20, 0), 1)) = (*_22);
(*_2) = Field::<u128>(Variant((*_27), 0), 6) > Field::<u128>(Variant((*_27), 0), 6);
place!(Field::<i16>(Variant((*_27), 0), 4)) = -RET;
(*_21) = !Field::<u128>(Variant((*_27), 0), 6);
place!(Field::<isize>(Variant((*_27), 0), 2)) = _5 ^ _5;
place!(Field::<u32>(Variant((*_27), 0), 0)) = 3429724058_u32 | 1494783745_u32;
place!(Field::<u8>(Variant((*_27), 0), 5)) = 76_u8 | 112_u8;
place!(Field::<usize>(Variant((*_27), 0), 3)) = _19.0 - _19.0;
(*_27) = Adt17::Variant2 { fld0: _18,fld1: 36717_u16,fld2: 59_u8 };
_2 = core::ptr::addr_of!((*_2));
place!(Field::<u16>(Variant((*_27), 2), 1)) = _12 as u16;
place!(Field::<u8>(Variant((*_27), 2), 2)) = !234_u8;
Goto(bb29)
}
bb29 = {
place!(Field::<u16>(Variant((*_27), 2), 1)) = 15247_u16;
place!(Field::<u8>(Variant(_20, 2), 2)) = 222_u8 | 13_u8;
(*_2) = !_25;
(*_21) = !336237509092564389031610099837512200417_u128;
(*_21) = 231915995859923474764794295196879799720_u128 | 60316009071529852997393952494167863976_u128;
place!(Field::<u16>(Variant((*_27), 2), 1)) = !42103_u16;
_10 = _3 ^ _8;
place!(Field::<f64>(Variant((*_27), 2), 0)) = RET as f64;
place!(Field::<u8>(Variant((*_27), 2), 2)) = !67_u8;
place!(Field::<u16>(Variant((*_27), 2), 1)) = 50894_u16;
match Field::<u16>(Variant((*_27), 2), 1) {
0 => bb24,
1 => bb26,
2 => bb30,
3 => bb31,
50894 => bb33,
_ => bb32
}
}
bb30 = {
_7 = _9;
_13.3.fld0 = 144014609405889922156566912400993086510_u128 ^ 161758279084589515559733998274282156905_u128;
_13.3.fld0 = 17_u8 as u128;
_13.2 = core::ptr::addr_of!(_4);
_13.1 = [7821706185701691720_u64,6735258331259352276_u64,15717040300168906287_u64,2102538499927550896_u64,3096167175956531089_u64,9205848113803441975_u64];
_6 = _4 + _4;
_13.1 = [16488675075124086242_u64,17238921147053628192_u64,7481327762962479315_u64,17697703478127085779_u64,12741010803984620926_u64,12512608831765272678_u64];
RET = (-1890_i16);
RET = 17542_i16;
_4 = _12 - _6;
_7 = _11;
_11 = _9;
_10 = _6 * _8;
_6 = -_3;
_1.1 = &_11;
_13.3.fld0 = 107221245992602564272377262849022086574_u128 | 187328114022450401607964892139680359227_u128;
_6 = !_3;
_4 = _3;
_13.2 = core::ptr::addr_of!(_8);
_8 = true as isize;
match RET {
0 => bb2,
1 => bb3,
17542 => bb5,
_ => bb4
}
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
place!(Field::<u16>(Variant((*_27), 2), 1)) = 29349_u16;
place!(Field::<u16>(Variant((*_27), 2), 1)) = _24 as u16;
place!(Field::<u16>(Variant((*_27), 2), 1)) = !48795_u16;
place!(Field::<i128>(Variant(_23, 1), 0)) = !_24;
_29 = !(*_2);
place!(Field::<u16>(Variant((*_27), 2), 1)) = 63987_u16 - 10482_u16;
_33 = _12 >> _8;
_28 = (*_2) | _19.1;
match _15 {
0 => bb13,
1 => bb34,
2 => bb35,
3 => bb36,
4 => bb37,
340282366920938463463374607431768211376 => bb39,
_ => bb38
}
}
bb34 = {
Return()
}
bb35 = {
(*_2) = _5 <= _8;
_19.1 = !false;
_23 = Adt17::Variant2 { fld0: _18,fld1: 7382_u16,fld2: 66_u8 };
(*_2) = _5 > _3;
_23 = Adt17::Variant1 { fld0: 121450452511603103745495584369935614139_i128,fld1: 17340605378625734020_u64,fld2: _14 };
_25 = (*_2) <= (*_2);
_22 = &_11;
place!(Field::<u64>(Variant(_23, 1), 1)) = (-116370052508997735518505224820986689651_i128) as u64;
_3 = _8;
_24 = Field::<u64>(Variant(_23, 1), 1) as i128;
Goto(bb27)
}
bb36 = {
Return()
}
bb37 = {
place!(Field::<u16>(Variant((*_27), 2), 1)) = 15247_u16;
place!(Field::<u8>(Variant(_20, 2), 2)) = 222_u8 | 13_u8;
(*_2) = !_25;
(*_21) = !336237509092564389031610099837512200417_u128;
(*_21) = 231915995859923474764794295196879799720_u128 | 60316009071529852997393952494167863976_u128;
place!(Field::<u16>(Variant((*_27), 2), 1)) = !42103_u16;
_10 = _3 ^ _8;
place!(Field::<f64>(Variant((*_27), 2), 0)) = RET as f64;
place!(Field::<u8>(Variant((*_27), 2), 2)) = !67_u8;
place!(Field::<u16>(Variant((*_27), 2), 1)) = 50894_u16;
match Field::<u16>(Variant((*_27), 2), 1) {
0 => bb24,
1 => bb26,
2 => bb30,
3 => bb31,
50894 => bb33,
_ => bb32
}
}
bb38 = {
_27 = core::ptr::addr_of!(_20);
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: Field::<f32>(Variant(_23, 1), 2) };
(*_2) = _25;
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: _14 };
place!(Field::<u64>(Variant((*_27), 1), 1)) = Field::<u64>(Variant(_23, 1), 1) & Field::<u64>(Variant(_23, 1), 1);
_25 = (*_2) | (*_2);
_21 = &mut _13.3.fld0;
RET = (-29022_i16);
(*_21) = !174507454317620944231790544222661546296_u128;
place!(Field::<f32>(Variant((*_27), 1), 2)) = _14 - _14;
place!(Field::<u64>(Variant((*_27), 1), 1)) = 1128060135_u32 as u64;
place!(Field::<f32>(Variant((*_27), 1), 2)) = _14 + Field::<f32>(Variant(_23, 1), 2);
_19 = Checked(3_usize - 3_usize);
_25 = _6 <= _6;
(*_21) = 161554052466525624372925548248748483820_u128 << _8;
(*_27) = Adt17::Variant1 { fld0: _24,fld1: Field::<u64>(Variant(_23, 1), 1),fld2: _14 };
place!(Field::<u64>(Variant((*_27), 1), 1)) = !Field::<u64>(Variant(_23, 1), 1);
Goto(bb28)
}
bb39 = {
place!(Field::<u16>(Variant((*_27), 2), 1)) = !17514_u16;
place!(Field::<u8>(Variant((*_27), 2), 2)) = 231_u8;
_1.1 = &(*_22);
match Field::<u8>(Variant((*_27), 2), 2) {
0 => bb32,
231 => bb41,
_ => bb40
}
}
bb40 = {
Return()
}
bb41 = {
(*_27) = Adt17::Variant0 { fld0: 3127030666_u32,fld1: (*_22),fld2: _10,fld3: _19.0,fld4: RET,fld5: 52_u8,fld6: (*_21) };
place!(Field::<u8>(Variant((*_27), 0), 5)) = !5_u8;
place!(Field::<u32>(Variant((*_27), 0), 0)) = !1959069868_u32;
place!(Field::<isize>(Variant((*_27), 0), 2)) = _10;
place!(Field::<isize>(Variant((*_27), 0), 2)) = _10;
_22 = Move(_1.1);
(*_2) = _25;
place!(Field::<u128>(Variant((*_27), 0), 6)) = !(*_21);
(*_2) = !_28;
_1.1 = &place!(Field::<char>(Variant((*_27), 0), 1));
_8 = -_33;
(*_2) = Field::<isize>(Variant((*_27), 0), 2) == _10;
_12 = Field::<isize>(Variant(_20, 0), 2) >> Field::<isize>(Variant((*_27), 0), 2);
place!(Field::<u128>(Variant((*_27), 0), 6)) = (*_21) >> Field::<isize>(Variant((*_27), 0), 2);
place!(Field::<u32>(Variant((*_27), 0), 0)) = 1597524553_u32;
place!(Field::<u32>(Variant((*_27), 0), 0)) = Field::<i16>(Variant((*_27), 0), 4) as u32;
(*_2) = _29;
(*_2) = !_25;
place!(Field::<u8>(Variant((*_27), 0), 5)) = 116_u8 | 242_u8;
(*_27) = Adt17::Variant2 { fld0: _18,fld1: 34887_u16,fld2: 102_u8 };
place!(Field::<u8>(Variant((*_27), 2), 2)) = 131_u8 >> _33;
_36 = &_19.0;
place!(Field::<f64>(Variant((*_27), 2), 0)) = _24 as f64;
(*_27) = Adt17::Variant0 { fld0: 2558619834_u32,fld1: _9,fld2: _12,fld3: (*_36),fld4: RET,fld5: 101_u8,fld6: (*_21) };
place!(Field::<u32>(Variant((*_27), 0), 0)) = !2061820002_u32;
match Field::<i16>(Variant((*_27), 0), 4) {
0 => bb40,
1 => bb14,
340282366920938463463374607431768182434 => bb42,
_ => bb38
}
}
bb42 = {
place!(Field::<u8>(Variant((*_27), 0), 5)) = Field::<u64>(Variant(_23, 1), 1) as u8;
place!(Field::<usize>(Variant((*_27), 0), 3)) = (*_36);
Goto(bb43)
}
bb43 = {
match Field::<i16>(Variant((*_27), 0), 4) {
0 => bb44,
340282366920938463463374607431768182434 => bb46,
_ => bb45
}
}
bb44 = {
_9 = _11;
_4 = _13.3.fld0 as isize;
Call(_13.3.fld0 = core::intrinsics::bswap(139632705585417384380215296631722157586_u128), ReturnTo(bb18), UnwindUnreachable())
}
bb45 = {
Return()
}
bb46 = {
place!(Field::<isize>(Variant((*_27), 0), 2)) = -_33;
_5 = Field::<isize>(Variant((*_27), 0), 2);
place!(Field::<u128>(Variant((*_27), 0), 6)) = (*_21);
place!(Field::<char>(Variant((*_27), 0), 1)) = _9;
_31 = Field::<u64>(Variant(_23, 1), 1) & Field::<u64>(Variant(_23, 1), 1);
place!(Field::<u8>(Variant((*_27), 0), 5)) = 1700809623_i32 as u8;
place!(Field::<char>(Variant((*_27), 0), 1)) = _7;
place!(Field::<usize>(Variant((*_27), 0), 3)) = (*_36);
place!(Field::<i16>(Variant((*_27), 0), 4)) = RET;
_19.1 = !_28;
place!(Field::<u128>(Variant((*_27), 0), 6)) = (*_2) as u128;
place!(Field::<i16>(Variant((*_27), 0), 4)) = -RET;
_7 = _9;
place!(Field::<usize>(Variant((*_27), 0), 3)) = (*_36) ^ (*_36);
place!(Field::<isize>(Variant(_20, 0), 2)) = _12 | _33;
(*_21) = Field::<u128>(Variant((*_27), 0), 6);
place!(Field::<u8>(Variant((*_27), 0), 5)) = 82_u8 << (*_21);
_37 = &mut (*_36);
(*_2) = (*_21) != Field::<u128>(Variant((*_27), 0), 6);
Goto(bb47)
}
bb47 = {
_44 = [46483_u16];
_26 = Field::<isize>(Variant((*_27), 0), 2);
place!(Field::<u32>(Variant((*_27), 0), 0)) = 2250752079_u32 * 1692945361_u32;
place!(Field::<u32>(Variant((*_27), 0), 0)) = !3599456986_u32;
Goto(bb48)
}
bb48 = {
_38.1 = [_31,_31,Field::<u64>(Variant(_23, 1), 1),_31,_31,_31];
place!(Field::<u128>(Variant((*_27), 0), 6)) = Field::<u8>(Variant((*_27), 0), 5) as u128;
place!(Field::<isize>(Variant((*_27), 0), 2)) = _5;
place!(Field::<char>(Variant((*_27), 0), 1)) = _7;
_39 = core::ptr::addr_of_mut!(_18);
place!(Field::<isize>(Variant((*_27), 0), 2)) = _10 + _5;
place!(Field::<u128>(Variant((*_27), 0), 6)) = 3909754388018541557_i64 as u128;
place!(Field::<char>(Variant((*_27), 0), 1)) = _9;
_29 = !(*_2);
place!(Field::<u32>(Variant((*_27), 0), 0)) = !1231610420_u32;
(*_39) = (*_21) as f64;
place!(Field::<u8>(Variant((*_27), 0), 5)) = !43_u8;
_37 = &mut place!(Field::<usize>(Variant((*_27), 0), 3));
place!(Field::<u128>(Variant((*_27), 0), 6)) = (*_21);
(*_39) = Field::<i16>(Variant((*_27), 0), 4) as f64;
(*_39) = 59331_u16 as f64;
place!(Field::<u32>(Variant((*_27), 0), 0)) = 1771919335_u32 * 1460023864_u32;
place!(Field::<i16>(Variant((*_27), 0), 4)) = _14 as i16;
Goto(bb49)
}
bb49 = {
_15 = (-89_i8) & 88_i8;
place!(Field::<u128>(Variant((*_27), 0), 6)) = !(*_21);
_30 = !Field::<isize>(Variant((*_27), 0), 2);
_49.1.1 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_23, 1), 0),fld1: _31,fld2: Field::<f32>(Variant(_23, 1), 2) };
Goto(bb50)
}
bb50 = {
Call(_54 = dump_var(Move(_6), Move(_8), Move(_44), Move(_15)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_54 = dump_var(Move(_7), Move(_4), Move(_11), Move(_24)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_54 = dump_var(Move(_28), Move(_19), Move(_30), _55), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: *mut f64,mut _3: char,mut _4: isize,mut _5: char,mut _6: char,mut _7: char) -> *const bool {
mir! {
type RET = *const bool;
let _8: &'static (f64, &'static (u32, u32, Adt17, f32));
let _9: i128;
let _10: &'static &'static mut [u64; 6];
let _11: &'static *mut bool;
let _12: f64;
let _13: (*mut u16, Adt17);
let _14: (Adt50,);
let _15: Adt34;
let _16: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _17: char;
let _18: u8;
let _19: *const Adt60;
let _20: Adt50;
let _21: *mut [char; 6];
let _22: (char, Adt19, i32, (u32, u32, Adt17, f32));
let _23: u8;
let _24: *const isize;
let _25: char;
let _26: (Adt50,);
let _27: &'static mut *const i64;
let _28: i16;
let _29: Adt85;
let _30: f32;
let _31: *const &'static Adt19;
let _32: *mut (f64, &'static (u32, u32, Adt17, f32));
let _33: i8;
let _34: (*const (&'static char,), [u64; 6], *const isize, Adt50);
let _35: f32;
let _36: (Adt50,);
let _37: (&'static char,);
let _38: char;
let _39: isize;
let _40: *const f64;
let _41: f64;
let _42: f32;
let _43: &'static *mut bool;
let _44: f32;
let _45: u8;
let _46: isize;
let _47: u8;
let _48: ();
let _49: ();
{
_3 = _6;
_6 = _5;
_1 = 37744_u16 as isize;
_5 = _7;
_3 = _5;
_6 = _5;
_4 = _1 & _1;
_4 = !_1;
_5 = _7;
Goto(bb1)
}
bb1 = {
_9 = 26448099792668658822803909548622383797_i128 + (-160555354716700923568554266780586575296_i128);
_3 = _5;
_6 = _3;
_7 = _6;
_5 = _6;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 17209238367927771604_u64 as f64;
_6 = _7;
Goto(bb2)
}
bb2 = {
(*_2) = 2233421908_u32 as f64;
(*_2) = (-114_i8) as f64;
_5 = _6;
_9 = -13778011922585906810804534189362829895_i128;
(*_2) = _9 as f64;
_16.0 = _7;
_15.fld0 = _7;
(*_2) = 7609611318818193108_i64 as f64;
_16.3.2 = Adt17::Variant2 { fld0: (*_2),fld1: 25560_u16,fld2: 123_u8 };
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) * Field::<f64>(Variant(_16.3.2, 2), 0);
_16.3.0 = 2633765181_u32 << _4;
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
Goto(bb3)
}
bb3 = {
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_6 = _5;
_16.3.3 = 50457664271965597080543611335515262795_u128 as f32;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) * Field::<f64>(Variant(_16.3.2, 2), 0);
_18 = 4996690553048159292_u64 as u8;
Goto(bb4)
}
bb4 = {
_17 = _7;
_1 = _4 + _4;
_16.1 = Adt19::Variant0 { fld0: true,fld1: _9,fld2: 557730770228894012_u64 };
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_16.0 = _7;
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
RET = core::ptr::addr_of!(place!(Field::<bool>(Variant(_16.1, 0), 0)));
(*RET) = (*_2) != (*_2);
_4 = (-300998952_i32) as isize;
place!(Field::<u64>(Variant(_16.1, 0), 2)) = Field::<i128>(Variant(_16.1, 0), 1) as u64;
(*RET) = _3 > _6;
_7 = _6;
_4 = -_1;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) + Field::<f64>(Variant(_16.3.2, 2), 0);
_15.fld0 = _16.0;
_2 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_16.3.2, 2), 0)));
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
(*_2) = _12 - _12;
_16.2 = !(-104926071_i32);
_16.3.1 = _16.3.0 * _16.3.0;
(*RET) = _17 == _5;
(*_2) = _12 * _12;
(*_2) = _12 + _12;
_20 = Adt50 { fld0: 108114253785965581190276034807086137122_u128 };
_14.0.fld0 = _20.fld0;
_15 = Adt34 { fld0: _17 };
_15 = Adt34 { fld0: _16.0 };
Goto(bb5)
}
bb5 = {
(*_2) = _12 + _12;
(*RET) = true;
(*_2) = -_12;
RET = core::ptr::addr_of!((*RET));
match _14.0.fld0 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
108114253785965581190276034807086137122 => bb13,
_ => bb12
}
}
bb6 = {
_17 = _7;
_1 = _4 + _4;
_16.1 = Adt19::Variant0 { fld0: true,fld1: _9,fld2: 557730770228894012_u64 };
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_16.0 = _7;
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
RET = core::ptr::addr_of!(place!(Field::<bool>(Variant(_16.1, 0), 0)));
(*RET) = (*_2) != (*_2);
_4 = (-300998952_i32) as isize;
place!(Field::<u64>(Variant(_16.1, 0), 2)) = Field::<i128>(Variant(_16.1, 0), 1) as u64;
(*RET) = _3 > _6;
_7 = _6;
_4 = -_1;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) + Field::<f64>(Variant(_16.3.2, 2), 0);
_15.fld0 = _16.0;
_2 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_16.3.2, 2), 0)));
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
(*_2) = _12 - _12;
_16.2 = !(-104926071_i32);
_16.3.1 = _16.3.0 * _16.3.0;
(*RET) = _17 == _5;
(*_2) = _12 * _12;
(*_2) = _12 + _12;
_20 = Adt50 { fld0: 108114253785965581190276034807086137122_u128 };
_14.0.fld0 = _20.fld0;
_15 = Adt34 { fld0: _17 };
_15 = Adt34 { fld0: _16.0 };
Goto(bb5)
}
bb7 = {
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_6 = _5;
_16.3.3 = 50457664271965597080543611335515262795_u128 as f32;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) * Field::<f64>(Variant(_16.3.2, 2), 0);
_18 = 4996690553048159292_u64 as u8;
Goto(bb4)
}
bb8 = {
(*_2) = 2233421908_u32 as f64;
(*_2) = (-114_i8) as f64;
_5 = _6;
_9 = -13778011922585906810804534189362829895_i128;
(*_2) = _9 as f64;
_16.0 = _7;
_15.fld0 = _7;
(*_2) = 7609611318818193108_i64 as f64;
_16.3.2 = Adt17::Variant2 { fld0: (*_2),fld1: 25560_u16,fld2: 123_u8 };
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) * Field::<f64>(Variant(_16.3.2, 2), 0);
_16.3.0 = 2633765181_u32 << _4;
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
Goto(bb3)
}
bb9 = {
_9 = 26448099792668658822803909548622383797_i128 + (-160555354716700923568554266780586575296_i128);
_3 = _5;
_6 = _3;
_7 = _6;
_5 = _6;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 17209238367927771604_u64 as f64;
_6 = _7;
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
(*_2) = -_12;
(*RET) = (*_2) <= (*_2);
(*RET) = false;
_16.3.2 = Adt17::Variant1 { fld0: _9,fld1: Field::<u64>(Variant(_16.1, 0), 2),fld2: _16.3.3 };
_18 = 14_u8 << _16.3.0;
_22.3.2 = Adt17::Variant1 { fld0: Field::<i128>(Variant(_16.3.2, 1), 0),fld1: Field::<u64>(Variant(_16.3.2, 1), 1),fld2: _16.3.3 };
(*RET) = true;
(*RET) = true | true;
place!(Field::<bool>(Variant(_16.1, 0), 0)) = _16.0 >= _3;
_22.3.3 = _16.3.3 * _16.3.3;
_6 = _16.0;
place!(Field::<f32>(Variant(_16.3.2, 1), 2)) = _22.3.3;
(*RET) = _16.3.1 != _16.3.1;
_13.1 = Adt17::Variant0 { fld0: _16.3.1,fld1: _5,fld2: _1,fld3: 1_usize,fld4: 16955_i16,fld5: _18,fld6: _14.0.fld0 };
_14.0 = Move(_20);
place!(Field::<f32>(Variant(_16.3.2, 1), 2)) = -_22.3.3;
_22.0 = _17;
_2 = core::ptr::addr_of_mut!(_12);
(*RET) = true;
(*_2) = _14.0.fld0 as f64;
(*RET) = false;
match _14.0.fld0 {
0 => bb1,
1 => bb7,
2 => bb5,
3 => bb9,
4 => bb14,
108114253785965581190276034807086137122 => bb16,
_ => bb15
}
}
bb14 = {
_17 = _7;
_1 = _4 + _4;
_16.1 = Adt19::Variant0 { fld0: true,fld1: _9,fld2: 557730770228894012_u64 };
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_16.0 = _7;
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
RET = core::ptr::addr_of!(place!(Field::<bool>(Variant(_16.1, 0), 0)));
(*RET) = (*_2) != (*_2);
_4 = (-300998952_i32) as isize;
place!(Field::<u64>(Variant(_16.1, 0), 2)) = Field::<i128>(Variant(_16.1, 0), 1) as u64;
(*RET) = _3 > _6;
_7 = _6;
_4 = -_1;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) + Field::<f64>(Variant(_16.3.2, 2), 0);
_15.fld0 = _16.0;
_2 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_16.3.2, 2), 0)));
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
(*_2) = _12 - _12;
_16.2 = !(-104926071_i32);
_16.3.1 = _16.3.0 * _16.3.0;
(*RET) = _17 == _5;
(*_2) = _12 * _12;
(*_2) = _12 + _12;
_20 = Adt50 { fld0: 108114253785965581190276034807086137122_u128 };
_14.0.fld0 = _20.fld0;
_15 = Adt34 { fld0: _17 };
_15 = Adt34 { fld0: _16.0 };
Goto(bb5)
}
bb15 = {
_9 = 26448099792668658822803909548622383797_i128 + (-160555354716700923568554266780586575296_i128);
_3 = _5;
_6 = _3;
_7 = _6;
_5 = _6;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = 17209238367927771604_u64 as f64;
_6 = _7;
Goto(bb2)
}
bb16 = {
(*_2) = _22.3.3 as f64;
place!(Field::<u8>(Variant(_13.1, 0), 5)) = _18 - _18;
place!(Field::<u64>(Variant(_22.3.2, 1), 1)) = Field::<u64>(Variant(_16.3.2, 1), 1) ^ Field::<u64>(Variant(_16.1, 0), 2);
match Field::<u128>(Variant(_13.1, 0), 6) {
108114253785965581190276034807086137122 => bb18,
_ => bb17
}
}
bb17 = {
_17 = _7;
_1 = _4 + _4;
_16.1 = Adt19::Variant0 { fld0: true,fld1: _9,fld2: 557730770228894012_u64 };
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
_16.0 = _7;
(*_2) = -Field::<f64>(Variant(_16.3.2, 2), 0);
RET = core::ptr::addr_of!(place!(Field::<bool>(Variant(_16.1, 0), 0)));
(*RET) = (*_2) != (*_2);
_4 = (-300998952_i32) as isize;
place!(Field::<u64>(Variant(_16.1, 0), 2)) = Field::<i128>(Variant(_16.1, 0), 1) as u64;
(*RET) = _3 > _6;
_7 = _6;
_4 = -_1;
(*_2) = Field::<f64>(Variant(_16.3.2, 2), 0) + Field::<f64>(Variant(_16.3.2, 2), 0);
_15.fld0 = _16.0;
_2 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_16.3.2, 2), 0)));
_13.0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_16.3.2, 2), 1)));
(*_2) = _12 - _12;
_16.2 = !(-104926071_i32);
_16.3.1 = _16.3.0 * _16.3.0;
(*RET) = _17 == _5;
(*_2) = _12 * _12;
(*_2) = _12 + _12;
_20 = Adt50 { fld0: 108114253785965581190276034807086137122_u128 };
_14.0.fld0 = _20.fld0;
_15 = Adt34 { fld0: _17 };
_15 = Adt34 { fld0: _16.0 };
Goto(bb5)
}
bb18 = {
place!(Field::<usize>(Variant(_13.1, 0), 3)) = 5_usize * 1_usize;
(*_2) = Field::<u32>(Variant(_13.1, 0), 0) as f64;
place!(Field::<u8>(Variant(_13.1, 0), 5)) = 67_i8 as u8;
place!(Field::<bool>(Variant(_16.1, 0), 0)) = false ^ true;
_22.3.0 = _16.3.1 * _16.3.0;
place!(Field::<f32>(Variant(_22.3.2, 1), 2)) = Field::<f32>(Variant(_16.3.2, 1), 2);
_22.2 = !_16.2;
place!(Field::<i16>(Variant(_13.1, 0), 4)) = (-29090_i16) & 32220_i16;
_9 = -Field::<i128>(Variant(_22.3.2, 1), 0);
(*_2) = _16.3.1 as f64;
(*RET) = Field::<u8>(Variant(_13.1, 0), 5) == _18;
(*RET) = !false;
_5 = _17;
place!(Field::<u128>(Variant(_13.1, 0), 6)) = _14.0.fld0 / _14.0.fld0;
_22.3.0 = (-6088406978655016803_i64) as u32;
_22.3.1 = !_16.3.1;
(*RET) = !true;
(*RET) = false;
_16.0 = _3;
_16.2 = !_22.2;
place!(Field::<usize>(Variant(_13.1, 0), 3)) = 11273097727936452041_usize << Field::<i128>(Variant(_22.3.2, 1), 0);
place!(Field::<i16>(Variant(_13.1, 0), 4)) = _3 as i16;
_14.0.fld0 = Field::<u128>(Variant(_13.1, 0), 6);
(*_2) = _22.3.1 as f64;
Goto(bb19)
}
bb19 = {
_20 = Move(_14.0);
_26.0 = Move(_20);
_20.fld0 = !Field::<u128>(Variant(_13.1, 0), 6);
_20 = Move(_26.0);
_24 = core::ptr::addr_of!(_4);
_22.1 = Adt19::Variant2 { fld0: _22.2,fld1: Field::<usize>(Variant(_13.1, 0), 3),fld2: _18,fld3: (-33_i8),fld4: Field::<i128>(Variant(_16.3.2, 1), 0) };
(*RET) = false | true;
(*_24) = Field::<u8>(Variant(_22.1, 2), 2) as isize;
_26.0 = Adt50 { fld0: Field::<u128>(Variant(_13.1, 0), 6) };
(*_24) = Field::<isize>(Variant(_13.1, 0), 2) ^ Field::<isize>(Variant(_13.1, 0), 2);
(*_24) = _1 << Field::<u32>(Variant(_13.1, 0), 0);
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _20.fld0 as f64;
Goto(bb20)
}
bb20 = {
_25 = _15.fld0;
(*_2) = (*_24) as f64;
_14 = Move(_26);
_22.0 = _5;
_22.2 = 29164_u16 as i32;
(*_24) = _1 >> Field::<u8>(Variant(_22.1, 2), 2);
_16.3.1 = Field::<u32>(Variant(_13.1, 0), 0) * _22.3.0;
_17 = _7;
place!(Field::<bool>(Variant(_16.1, 0), 0)) = (*_2) == (*_2);
_22.1 = Move(_16.1);
(*_2) = _22.2 as f64;
(*_24) = -Field::<isize>(Variant(_13.1, 0), 2);
(*RET) = !Field::<bool>(Variant(_22.1, 0), 0);
_22.3.1 = !Field::<u32>(Variant(_13.1, 0), 0);
_7 = _17;
(*RET) = Field::<bool>(Variant(_22.1, 0), 0);
_22.2 = -_16.2;
(*_2) = Field::<u128>(Variant(_13.1, 0), 6) as f64;
(*RET) = Field::<bool>(Variant(_22.1, 0), 0) >= Field::<bool>(Variant(_22.1, 0), 0);
(*_2) = _22.2 as f64;
_16 = Move(_22);
_22.3.1 = !Field::<u32>(Variant(_13.1, 0), 0);
Goto(bb21)
}
bb21 = {
(*_24) = Field::<isize>(Variant(_13.1, 0), 2) - Field::<isize>(Variant(_13.1, 0), 2);
_15 = Adt34 { fld0: _5 };
_4 = !Field::<isize>(Variant(_13.1, 0), 2);
(*_24) = _1 | Field::<isize>(Variant(_13.1, 0), 2);
(*_24) = Field::<isize>(Variant(_13.1, 0), 2) ^ Field::<isize>(Variant(_13.1, 0), 2);
_9 = !Field::<i128>(Variant(_16.1, 0), 1);
place!(Field::<char>(Variant(_13.1, 0), 1)) = _15.fld0;
(*_24) = Field::<isize>(Variant(_13.1, 0), 2) & _1;
(*_2) = (-4541581364513606812_i64) as f64;
_22.3 = _16.3;
(*_24) = Field::<isize>(Variant(_13.1, 0), 2);
_34.2 = core::ptr::addr_of!(_1);
_36.0.fld0 = _20.fld0 + Field::<u128>(Variant(_13.1, 0), 6);
(*_24) = Field::<isize>(Variant(_13.1, 0), 2);
(*_24) = _1 & _1;
_22.1 = Move(_16.1);
(*_2) = (-112_i8) as f64;
_22.2 = _16.2 >> Field::<usize>(Variant(_13.1, 0), 3);
_30 = Field::<f32>(Variant(_22.3.2, 1), 2);
(*_24) = -_1;
Call(_39 = core::intrinsics::bswap((*_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_39 = (*_24);
(*_24) = Field::<isize>(Variant(_13.1, 0), 2);
(*_2) = Field::<usize>(Variant(_13.1, 0), 3) as f64;
place!(Field::<u64>(Variant(_22.1, 0), 2)) = (*_2) as u64;
_40 = core::ptr::addr_of!((*_2));
_40 = core::ptr::addr_of!((*_40));
RET = core::ptr::addr_of!(place!(Field::<bool>(Variant(_22.1, 0), 0)));
_26 = (Move(_36.0),);
_39 = _1 | (*_24);
place!(Field::<u32>(Variant(_13.1, 0), 0)) = _22.3.0 | _16.3.1;
_35 = Field::<f32>(Variant(_22.3.2, 1), 2);
_34.3.fld0 = _26.0.fld0 & _26.0.fld0;
_34.1 = [Field::<u64>(Variant(_22.1, 0), 2),Field::<u64>(Variant(_16.3.2, 1), 1),Field::<u64>(Variant(_16.3.2, 1), 1),Field::<u64>(Variant(_22.3.2, 1), 1),Field::<u64>(Variant(_16.3.2, 1), 1),Field::<u64>(Variant(_16.3.2, 1), 1)];
(*_2) = (-2454545535844401957_i64) as f64;
(*RET) = false | false;
(*_2) = _30 as f64;
_39 = _4;
_34.3.fld0 = !_26.0.fld0;
place!(Field::<f32>(Variant(_22.3.2, 1), 2)) = _16.2 as f32;
(*RET) = false | false;
Goto(bb23)
}
bb23 = {
Call(_48 = dump_var(Move(_7), Move(_39), Move(_3), Move(_4)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_48 = dump_var(Move(_25), _49, _49, _49), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{c1705}'), std::hint::black_box(1584561203_u32), std::hint::black_box(35_i8), std::hint::black_box((-1684_i16)), std::hint::black_box(183131218_i32), std::hint::black_box(8333875494033331885_i64), std::hint::black_box((-4550138847304011708318687128522081079_i128)), std::hint::black_box(7112762253097467619_usize), std::hint::black_box(3323953954479996740_u64), std::hint::black_box(17102_u16));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt17 {
Variant0{
fld0: u32,
fld1: char,
fld2: isize,
fld3: usize,
fld4: i16,
fld5: u8,
fld6: u128,

},
Variant1{
fld0: i128,
fld1: u64,
fld2: f32,

},
Variant2{
fld0: f64,
fld1: u16,
fld2: u8,

}}
#[derive(Debug)]
pub enum Adt19 {
Variant0{
fld0: bool,
fld1: i128,
fld2: u64,

},
Variant1{
fld0: *const i64,

},
Variant2{
fld0: i32,
fld1: usize,
fld2: u8,
fld3: i8,
fld4: i128,

},
Variant3{
fld0: bool,
fld1: usize,
fld2: u32,
fld3: i8,
fld4: *const i64,
fld5: i32,
fld6: i64,
fld7: u16,

}}
#[derive(Debug)]
pub struct Adt34 {
fld0: char,
}
#[derive(Debug)]
pub struct Adt40 {
fld0: isize,
fld1: *mut f64,
}
#[derive(Debug)]
pub struct Adt50 {
fld0: u128,
}
#[derive(Debug)]
pub struct Adt52 {
fld0: bool,
fld1: char,
fld2: (u32, u32, Adt17, f32),
fld3: *const i64,
fld4: (*mut u16, isize, [u16; 1], Adt19),
fld5: *mut [char; 6],
fld6: (char, Adt19, i32, (u32, u32, Adt17, f32)),
fld7: f64,
}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: bool,
fld1: u16,
fld2: *const *mut u16,
fld3: *const bool,
fld4: (*mut u16, isize, [u16; 1], Adt19),
fld5: *mut [char; 6],

},
Variant1{
fld0: [u16; 1],
fld1: i64,
fld2: *const bool,

},
Variant2{
fld0: bool,
fld1: u64,

}}
#[derive(Debug)]
pub enum Adt85 {
Variant0{
fld0: *mut bool,
fld1: *mut *const bool,
fld2: (Adt50,),
fld3: Adt17,
fld4: *mut u16,
fld5: [u16; 1],
fld6: u16,

},
Variant1{
fld0: (Adt50,),
fld1: [u32; 6],

}}

