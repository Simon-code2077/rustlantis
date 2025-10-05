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
pub fn fn0(mut _1: u64,mut _2: u8,mut _3: u32,mut _4: i8,mut _5: i16,mut _6: u128,mut _7: i64,mut _8: i128,mut _9: usize) -> Adt26 {
mir! {
type RET = Adt26;
let _10: (u16,);
let _11: f32;
let _12: *const &'static mut [u32; 3];
let _13: i16;
let _14: ();
let _15: ();
{
RET.fld4 = 154649711533669404426412949506069756492_u128;
_7 = 8415892863344071140_i64 << RET.fld4;
_6 = RET.fld4;
_10.0 = 28943_u16 - 22283_u16;
_9 = 4_usize >> _10.0;
_2 = 16_i8 as u8;
_1 = 1630404096921863785_u64 >> _6;
RET.fld1 = _10.0;
Call(RET.fld5 = fn1(_7, RET.fld1, _7, _10.0, _1, RET.fld1, _6, _10, _10.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 69109396244435885489675634697807839706_i128;
Goto(bb2)
}
bb2 = {
_1 = 4808098835155446258_u64;
RET.fld2.0 = !RET.fld1;
_4 = 71_i8;
_5 = 9160_i16;
RET.fld3 = _9;
RET.fld1 = RET.fld2.0 >> Field::<isize>(Variant(RET.fld5, 0), 0);
_5 = !32055_i16;
RET.fld0 = _1 ^ _1;
RET.fld0 = '\u{c558b}' as u64;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(Move(_4), Move(_2), Move(_5), Move(_8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i64,mut _2: u16,mut _3: i64,mut _4: u16,mut _5: u64,mut _6: u16,mut _7: u128,mut _8: (u16,),mut _9: u16) -> Adt21 {
mir! {
type RET = Adt21;
let _10: *const &'static (f32, i8, i128);
let _11: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _12: f64;
let _13: f64;
let _14: u128;
let _15: Adt81;
let _16: &'static (f32, i8, i128);
let _17: [i16; 1];
let _18: u64;
let _19: u128;
let _20: i64;
let _21: (u16, Adt21, *mut [i16; 1]);
let _22: f32;
let _23: u128;
let _24: ([u16; 5], i64);
let _25: isize;
let _26: *const &'static char;
let _27: u128;
let _28: *mut u16;
let _29: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _30: &'static mut u32;
let _31: Adt52;
let _32: u64;
let _33: Adt81;
let _34: isize;
let _35: ([u16; 5], i64);
let _36: char;
let _37: *mut *mut u16;
let _38: (char, i64, Adt17);
let _39: f64;
let _40: isize;
let _41: (*const &'static (f32, i8, i128),);
let _42: (u32,);
let _43: *mut [i16; 1];
let _44: &'static mut &'static mut u32;
let _45: &'static mut isize;
let _46: f64;
let _47: i8;
let _48: isize;
let _49: u128;
let _50: isize;
let _51: i64;
let _52: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _53: [i64; 2];
let _54: [u32; 3];
let _55: i32;
let _56: &'static mut isize;
let _57: isize;
let _58: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _59: *mut [i16; 1];
let _60: f32;
let _61: i32;
let _62: &'static mut isize;
let _63: i16;
let _64: isize;
let _65: i16;
let _66: f64;
let _67: [char; 1];
let _68: f32;
let _69: u128;
let _70: i16;
let _71: &'static (char, i64, Adt17);
let _72: [i64; 2];
let _73: &'static [i128; 8];
let _74: *mut i32;
let _75: (*const &'static (f32, i8, i128),);
let _76: &'static i16;
let _77: char;
let _78: *mut *mut u16;
let _79: char;
let _80: &'static mut u32;
let _81: isize;
let _82: *mut Adt52;
let _83: ();
let _84: ();
{
_7 = !81173794589950127421317986406874390844_u128;
_8 = (_9,);
_8 = (_9,);
_11.0.2 = [_4];
RET = Adt21::Variant0 { fld0: (-9223372036854775808_isize) };
_11.1 = _9 > _9;
_4 = _2 + _9;
_11.1 = true & true;
Call(RET = fn2(_5, _11.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _1 + _1;
_13 = _3 as f64;
_12 = -_13;
_11.0.1 = _11.0.2;
_11.1 = !false;
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize;
_17 = [2769_i16];
_15.fld2.0 = 105_u8 as f32;
_2 = _7 as u16;
_15.fld1.1.0 = '\u{f5155}';
_5 = !8174805704914599330_u64;
_11.1 = _4 > _4;
RET = Adt21::Variant2 { fld0: 471646013_u32,fld1: _15.fld1.1.0,fld2: _15.fld2.0,fld3: _12,fld4: 0_usize };
_15.fld4.1 = _7 & _7;
_15.fld2.1 = _13 + _12;
place!(Field::<f32>(Variant(RET, 2), 2)) = _15.fld2.0 * _15.fld2.0;
_15.fld1.1.1 = !_3;
_15.fld1.0 = (_4,);
_5 = 3074248605687561168_u64 * 649934934544298147_u64;
_11.0.2 = [_4];
Goto(bb2)
}
bb2 = {
_15.fld1.1.0 = Field::<char>(Variant(RET, 2), 1);
_18 = _5 - _5;
_15.fld4.1 = _7;
_3 = _15.fld1.1.1 * _15.fld1.1.1;
_18 = _5 << _4;
place!(Field::<f32>(Variant(RET, 2), 2)) = _15.fld2.0 - _15.fld2.0;
place!(Field::<u32>(Variant(RET, 2), 0)) = !3830490230_u32;
_15.fld0 = !_11.1;
_11.1 = _13 < _12;
_21.0 = _15.fld1.0.0 + _6;
_20 = (-1446420901_i32) as i64;
_21.2 = core::ptr::addr_of_mut!(_17);
_1 = _3 | _15.fld1.1.1;
_15.fld2.2 = Field::<u32>(Variant(RET, 2), 0) as i64;
_21.2 = core::ptr::addr_of_mut!(_17);
_8.0 = _21.0;
RET = Adt21::Variant1 { fld0: _3,fld1: 0_usize,fld2: _8,fld3: 116_u8 };
Call(place!(Field::<u8>(Variant(RET, 1), 3)) = core::intrinsics::bswap(29_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = _15.fld4.1 * _15.fld4.1;
_15.fld2.1 = _12 + _13;
_11.0.2 = [_15.fld1.0.0];
_5 = !_18;
_15.fld1.1.0 = '\u{10d3be}';
_12 = -_15.fld2.1;
_11.1 = _15.fld0 ^ _15.fld0;
_15.fld2.0 = 8855_i16 as f32;
_2 = Field::<(u16,)>(Variant(RET, 1), 2).0 >> _5;
_12 = _15.fld2.1 - _15.fld2.1;
_19 = !_14;
Goto(bb4)
}
bb4 = {
_17 = [24211_i16];
_20 = _15.fld1.1.1 + Field::<i64>(Variant(RET, 1), 0);
_15.fld1.1.1 = _3 | _1;
_1 = _15.fld1.1.1;
_18 = !_5;
_21.0 = _6;
Goto(bb5)
}
bb5 = {
_24.1 = 9223372036854775807_isize as i64;
_24.0 = [_8.0,_8.0,_6,_9,_21.0];
_25 = 67_isize << _15.fld1.1.1;
_6 = !_2;
_23 = _8.0 as u128;
_6 = _15.fld1.0.0 + _2;
_11.3 = &mut _25;
_15.fld1.1.2 = Adt17::Variant3 { fld0: _15.fld4.1,fld1: _15.fld1.1.0,fld2: 2764071841_u32,fld3: _12,fld4: _18,fld5: (-330172153_i32),fld6: _15.fld1.1.1,fld7: 42534738126810399022536187069807885114_i128 };
Goto(bb6)
}
bb6 = {
_15.fld5 = Adt52::Variant0 { fld0: (-9223372036854775808_isize) };
_17 = [(-25988_i16)];
_11.1 = Field::<i64>(Variant(_15.fld1.1.2, 3), 6) == _1;
RET = Adt21::Variant2 { fld0: 276967838_u32,fld1: _15.fld1.1.0,fld2: _15.fld2.0,fld3: _13,fld4: 4_usize };
_11.0.1 = _11.0.2;
_21.1 = Adt21::Variant1 { fld0: _15.fld1.1.1,fld1: 11839083640155282862_usize,fld2: _15.fld1.0,fld3: 147_u8 };
place!(Field::<usize>(Variant(RET, 2), 4)) = 4531397909204960287_usize ^ 11630838129767740978_usize;
_21.2 = core::ptr::addr_of_mut!(_17);
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
place!(Field::<(u16,)>(Variant(_21.1, 1), 2)).0 = 179_u8 as u16;
_21.1 = Adt21::Variant1 { fld0: _15.fld1.1.1,fld1: Field::<usize>(Variant(RET, 2), 4),fld2: _8,fld3: 148_u8 };
_28 = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant(_21.1, 1), 2)).0);
_9 = (*_28) >> (*_28);
_9 = (*_28);
_15.fld1.0 = Field::<(u16,)>(Variant(_21.1, 1), 2);
place!(Field::<f32>(Variant(RET, 2), 2)) = _15.fld2.1 as f32;
Call(_5 = core::intrinsics::bswap(_18), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_21.1 = Adt21::Variant2 { fld0: 4284973572_u32,fld1: Field::<char>(Variant(_15.fld1.1.2, 3), 1),fld2: Field::<f32>(Variant(RET, 2), 2),fld3: _12,fld4: Field::<usize>(Variant(RET, 2), 4) };
_23 = _14 & Field::<u128>(Variant(_15.fld1.1.2, 3), 0);
_15.fld4.1 = Field::<u128>(Variant(_15.fld1.1.2, 3), 0);
_7 = _19 + _19;
_33.fld1.0.0 = _6 | _6;
_11.0.1 = [_33.fld1.0.0];
Call(_29.2.3 = core::intrinsics::transmute(_15.fld1.1.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_33.fld4.0 = core::ptr::addr_of_mut!(_28);
_15.fld2 = (Field::<f32>(Variant(RET, 2), 2), _12, _15.fld1.1.1);
_33.fld1.1.1 = _1;
_27 = _7 | _23;
_33.fld4 = (Move(_15.fld4.0), _27);
_33.fld1.1.1 = _5 as i64;
place!(Field::<isize>(Variant(_15.fld5, 0), 0)) = !48_isize;
_8.0 = Field::<char>(Variant(RET, 2), 1) as u16;
_33.fld0 = _11.1;
_4 = _29.2.3 as u16;
_28 = core::ptr::addr_of_mut!(_33.fld1.0.0);
_33.fld2.1 = Field::<f64>(Variant(RET, 2), 3) - Field::<f64>(Variant(_15.fld1.1.2, 3), 3);
_22 = _3 as f32;
_39 = Field::<f64>(Variant(_15.fld1.1.2, 3), 3) + Field::<f64>(Variant(RET, 2), 3);
_38.1 = !_15.fld1.1.1;
place!(Field::<i64>(Variant(_15.fld1.1.2, 3), 6)) = _15.fld2.2 >> (*_28);
_20 = Field::<i64>(Variant(_15.fld1.1.2, 3), 6) << (*_28);
Call(place!(Field::<u128>(Variant(_15.fld1.1.2, 3), 0)) = core::intrinsics::transmute(_23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11.0.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_15.fld1.1.2, 3), 5)));
_11.0.2 = [(*_28)];
(*_28) = _9;
place!(Field::<i64>(Variant(_15.fld1.1.2, 3), 6)) = _33.fld0 as i64;
place!(Field::<u32>(Variant(_15.fld1.1.2, 3), 2)) = !_29.2.3;
place!(Field::<i64>(Variant(_15.fld1.1.2, 3), 6)) = (-946635136_i32) as i64;
Call(_29.2.0 = core::intrinsics::bswap(1804799004_i32), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<i64>(Variant(_15.fld1.1.2, 3), 6)) = !_1;
(*_28) = _2 & _2;
_37 = Move(_33.fld4.0);
_15.fld1.1.2 = Adt17::Variant0 { fld0: _39,fld1: _15.fld1.1.0,fld2: Field::<isize>(Variant(_15.fld5, 0), 0),fld3: _33.fld4.1,fld4: 2942_i16,fld5: Field::<f32>(Variant(_21.1, 2), 2) };
_33.fld4 = (Move(_37), _27);
(*_28) = !_9;
_33.fld1.1.0 = Field::<char>(Variant(RET, 2), 1);
_29.2.1 = core::ptr::addr_of!(RET);
Goto(bb11)
}
bb11 = {
(*_28) = (-21528_i16) as u16;
(*_28) = _15.fld1.0.0 << _1;
(*_28) = (-87_i8) as u16;
_34 = Field::<isize>(Variant(_15.fld5, 0), 0) << _7;
_15.fld4 = (Move(_33.fld4.0), Field::<u128>(Variant(_15.fld1.1.2, 0), 3));
_35 = (_24.0, _20);
place!(Field::<f32>(Variant(RET, 2), 2)) = -Field::<f32>(Variant(_21.1, 2), 2);
(*_28) = _6 - _15.fld1.0.0;
_33.fld1.1.2 = Adt17::Variant1 { fld0: 9_u8 };
_21.1 = Adt21::Variant0 { fld0: _34 };
(*_28) = _9 - _9;
_38.1 = _20;
Goto(bb12)
}
bb12 = {
_11.1 = _33.fld0 | _33.fld0;
_33.fld1.0.0 = _6 - _2;
Goto(bb13)
}
bb13 = {
(*_28) = 26354_i16 as u16;
_29.2.3 = 2975107396_u32 & 195336175_u32;
place!(Field::<usize>(Variant(RET, 2), 4)) = 8105755844408882445_usize - 5_usize;
place!(Field::<f64>(Variant(RET, 2), 3)) = _15.fld2.1 * _33.fld2.1;
_33.fld1.1.2 = Adt17::Variant0 { fld0: _33.fld2.1,fld1: Field::<char>(Variant(_15.fld1.1.2, 0), 1),fld2: Field::<isize>(Variant(_21.1, 0), 0),fld3: _15.fld4.1,fld4: (-7883_i16),fld5: _15.fld2.0 };
_4 = _22 as u16;
_15.fld0 = !_11.1;
_42.0 = _29.2.3 << _15.fld1.1.1;
_33.fld1.1.2 = Adt17::Variant1 { fld0: 137_u8 };
(*_28) = _2;
_46 = _15.fld2.1 + Field::<f64>(Variant(_15.fld1.1.2, 0), 0);
_15.fld2.2 = Field::<usize>(Variant(RET, 2), 4) as i64;
_15.fld1.0 = ((*_28),);
_33.fld2.0 = _1 as f32;
_15.fld2.0 = _22;
_23 = _19 - _14;
(*_28) = (-55149549559419066736108042169949116949_i128) as u16;
_20 = _18 as i64;
Goto(bb14)
}
bb14 = {
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
(*_28) = !_6;
_50 = _34;
_28 = core::ptr::addr_of_mut!((*_28));
_27 = Field::<usize>(Variant(RET, 2), 4) as u128;
_33.fld1.0 = (_6,);
_37 = core::ptr::addr_of_mut!(_28);
place!(Field::<isize>(Variant(_21.1, 0), 0)) = (-794909653_i32) as isize;
_43 = Move(_21.2);
_47 = _42.0 as i8;
place!(Field::<isize>(Variant(_15.fld5, 0), 0)) = -_34;
_33.fld2 = (_15.fld2.0, _46, _35.1);
_37 = Move(_15.fld4.0);
_52.2.1 = core::ptr::addr_of!(_29.1);
_35.0 = [_4,(*_28),_2,(*_28),(*_28)];
_38.0 = _33.fld1.1.0;
_21.0 = (*_28) * (*_28);
_55 = 1483961364_i32 - (-2126714146_i32);
_52.1 = [Field::<char>(Variant(_15.fld1.1.2, 0), 1)];
_30 = &mut _42.0;
(*_28) = !_2;
(*_30) = _29.2.3 * _29.2.3;
_48 = Field::<usize>(Variant(RET, 2), 4) as isize;
_29.2.2 = Move(_30);
_45 = &mut place!(Field::<isize>(Variant(_21.1, 0), 0));
_58.2.0 = _55 * _55;
Call((*_45) = core::intrinsics::bswap(_34), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_45) = (*_28) as isize;
(*_28) = !_15.fld1.0.0;
_32 = _5;
Goto(bb16)
}
bb16 = {
(*_45) = _50;
(*_45) = -_34;
_15.fld1.0 = ((*_28),);
RET = Adt21::Variant0 { fld0: (*_45) };
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
(*_45) = !Field::<isize>(Variant(RET, 0), 0);
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
(*_45) = !_50;
place!(Field::<i16>(Variant(_15.fld1.1.2, 0), 4)) = !26915_i16;
(*_45) = _50 | _50;
_17 = [Field::<i16>(Variant(_15.fld1.1.2, 0), 4)];
_33.fld1 = (_15.fld1.0, Move(_15.fld1.1));
_33.fld1.0 = _15.fld1.0;
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
_35.0 = [_9,(*_28),(*_28),_6,(*_28)];
_17 = [Field::<i16>(Variant(_33.fld1.1.2, 0), 4)];
_38 = (_33.fld1.1.0, _35.1, Move(_33.fld1.1.2));
_11.0.3 = core::ptr::addr_of_mut!(_29.2.0);
(*_28) = _4 + _9;
(*_45) = _34 & Field::<isize>(Variant(RET, 0), 0);
_52.2.2 = &mut _29.2.3;
_58.2.1 = core::ptr::addr_of!(_58.1);
_15.fld1.0 = ((*_28),);
Goto(bb17)
}
bb17 = {
_11.0.1 = [(*_28)];
_56 = &mut (*_45);
(*_56) = -Field::<isize>(Variant(_15.fld5, 0), 0);
_15.fld1.1 = Move(_38);
_63 = Field::<i16>(Variant(_15.fld1.1.2, 0), 4) + Field::<i16>(Variant(_15.fld1.1.2, 0), 4);
_57 = _47 as isize;
_15.fld2.2 = _33.fld1.1.1;
_44 = &mut _52.2.2;
(*_56) = _57 + Field::<isize>(Variant(_15.fld5, 0), 0);
(*_56) = Field::<isize>(Variant(RET, 0), 0);
_53 = [_15.fld1.1.1,_15.fld1.1.1];
_11.0.0 = &_15.fld1.1;
_15.fld1.1.1 = !_35.1;
_38.1 = -_33.fld2.2;
_11.0.1 = _11.0.2;
_11.1 = _33.fld0 & _15.fld0;
(*_56) = _57 * Field::<isize>(Variant(_15.fld5, 0), 0);
_36 = _15.fld1.1.0;
place!(Field::<isize>(Variant(_15.fld1.1.2, 0), 2)) = (*_56);
_60 = -_15.fld2.0;
_30 = Move((*_44));
_15.fld1.0.0 = _6;
_8.0 = (*_28);
_65 = _63 | _63;
_61 = _58.2.0;
Goto(bb18)
}
bb18 = {
_17 = [_63];
_15.fld1.0.0 = !(*_28);
_58.2.0 = _55;
_67 = _52.1;
_13 = _15.fld2.1;
(*_56) = _50;
_6 = !(*_28);
_47 = -123_i8;
(*_28) = _8.0 << _2;
(*_28) = _47 as u16;
_12 = _33.fld2.1;
_48 = -(*_56);
place!(Field::<isize>(Variant(_15.fld1.1.2, 0), 2)) = !(*_56);
_15.fld2 = (_60, _12, _35.1);
_33.fld1.1 = Move(_15.fld1.1);
_58.2.0 = _65 as i32;
_31 = Move(_15.fld5);
_58.2.0 = -_61;
(*_28) = _2;
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
_33.fld4 = (Move(_37), _19);
(*_56) = _34;
(*_56) = Field::<isize>(Variant(RET, 0), 0) + Field::<isize>(Variant(_31, 0), 0);
_63 = _65 | _65;
place!(Field::<isize>(Variant(_31, 0), 0)) = _57;
(*_28) = _22 as u16;
_15.fld1.1.1 = !_15.fld2.2;
(*_28) = _15.fld1.0.0;
_35.0 = [_15.fld1.0.0,_9,(*_28),(*_28),(*_28)];
Goto(bb19)
}
bb19 = {
_47 = _38.1 as i8;
_11.1 = _15.fld0;
_17 = [_63];
_15.fld4 = Move(_33.fld4);
_33.fld5 = Move(_31);
_39 = _12 * _33.fld2.1;
(*_28) = _15.fld1.0.0;
(*_56) = !Field::<isize>(Variant(_33.fld5, 0), 0);
_40 = !(*_56);
_33.fld3 = Adt64::Variant1 { fld0: _8,fld1: _35.0,fld2: _11.0.1,fld3: _17,fld4: Move(_33.fld1.1.2) };
_22 = Field::<f32>(Variant(Field::<Adt17>(Variant(_33.fld3, 1), 4), 0), 5) + _60;
(*_28) = 1877403001_u32 as u16;
_62 = &mut (*_56);
(*_28) = !_9;
_27 = _15.fld2.2 as u128;
_48 = !_40;
_49 = !_27;
_20 = -_35.1;
_15.fld1.0 = ((*_28),);
_51 = _61 as i64;
_15.fld1.1.0 = Field::<char>(Variant(Field::<Adt17>(Variant(_33.fld3, 1), 4), 0), 1);
(*_28) = _6;
(*_62) = _57 | _57;
Goto(bb20)
}
bb20 = {
_40 = (*_62) << _6;
_57 = (*_62) & (*_62);
_5 = !_18;
_31 = Adt52::Variant0 { fld0: (*_62) };
_54 = [908919753_u32,3274054846_u32,2682732045_u32];
_74 = core::ptr::addr_of_mut!(_58.2.0);
_15.fld1.1 = (_33.fld1.1.0, _38.1, Move(Field::<Adt17>(Variant(_33.fld3, 1), 4)));
_76 = &_63;
(*_62) = Field::<isize>(Variant(_33.fld5, 0), 0);
(*_62) = _57;
(*_62) = _57 << _33.fld2.2;
Goto(bb21)
}
bb21 = {
_71 = &_15.fld1.1;
_55 = (*_74) ^ (*_74);
_33.fld1.1.1 = (*_71).1 << (*_74);
_38.0 = Field::<char>(Variant((*_71).2, 0), 1);
_6 = !(*_28);
_33.fld1.1 = Move(_15.fld1.1);
_55 = (*_74) - (*_74);
_15.fld1.1.1 = -_20;
_33.fld1.1.1 = -_15.fld1.1.1;
_24.0 = Field::<[u16; 5]>(Variant(_33.fld3, 1), 1);
_68 = _22 - _22;
_71 = &_33.fld1.1;
_33.fld2.1 = _46 * Field::<f64>(Variant(_33.fld1.1.2, 0), 0);
_15.fld4.0 = core::ptr::addr_of_mut!(_28);
_17 = [(*_76)];
(*_62) = -Field::<isize>(Variant(_33.fld1.1.2, 0), 2);
(*_62) = !Field::<isize>(Variant((*_71).2, 0), 2);
_1 = (*_71).1 >> (*_28);
_70 = -(*_76);
Goto(bb22)
}
bb22 = {
_64 = (*_62);
_51 = !(*_71).1;
Call((*_62) = core::intrinsics::transmute(Field::<isize>(Variant((*_71).2, 0), 2)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
(*_74) = _18 as i32;
_15.fld2.2 = (*_71).1;
_15.fld1 = Move(_33.fld1);
place!(Field::<[u16; 5]>(Variant(_33.fld3, 1), 1)) = [_4,_15.fld1.0.0,_9,_15.fld1.0.0,Field::<(u16,)>(Variant(_33.fld3, 1), 0).0];
(*_28) = !_15.fld1.0.0;
_24 = (Field::<[u16; 5]>(Variant(_33.fld3, 1), 1), _51);
_38.2 = Adt17::Variant1 { fld0: 98_u8 };
place!(Field::<isize>(Variant(RET, 0), 0)) = (*_62) - _40;
_24 = _35;
_15.fld1.1.2 = Adt17::Variant3 { fld0: _27,fld1: _38.0,fld2: 4275710567_u32,fld3: _33.fld2.1,fld4: _18,fld5: (*_74),fld6: _15.fld1.1.1,fld7: (-164195938910176285012995101085201881287_i128) };
(*_62) = Field::<isize>(Variant(_33.fld5, 0), 0) - _40;
_33.fld1.0.0 = _15.fld1.0.0 ^ _15.fld1.0.0;
_76 = &_65;
(*_28) = Field::<(u16,)>(Variant(_33.fld3, 1), 0).0 >> (*_62);
_81 = _20 as isize;
Goto(bb24)
}
bb24 = {
Call(_83 = dump_var(Move(_36), Move(_42), Move(_3), Move(_35)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_83 = dump_var(Move(_20), Move(_67), Move(_51), Move(_65)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_83 = dump_var(Move(_19), Move(_53), Move(_2), Move(_50)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_83 = dump_var(Move(_17), Move(_49), Move(_55), Move(_81)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_83 = dump_var(Move(_54), Move(_7), Move(_61), Move(_5)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u64,mut _2: bool) -> Adt21 {
mir! {
type RET = Adt21;
let _3: &'static i16;
let _4: (*mut u16,);
let _5: isize;
let _6: i64;
let _7: i16;
let _8: usize;
let _9: i64;
let _10: (f32, i8, i128);
let _11: ((f32, i8, i128), usize, &'static (f32, i8, i128));
let _12: u16;
let _13: f64;
let _14: u8;
let _15: bool;
let _16: isize;
let _17: *const &'static (f32, i8, i128);
let _18: &'static mut isize;
let _19: i8;
let _20: isize;
let _21: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _22: bool;
let _23: &'static [i128; 8];
let _24: &'static Adt66;
let _25: f64;
let _26: isize;
let _27: (u16,);
let _28: char;
let _29: *const i64;
let _30: &'static mut [u32; 3];
let _31: f32;
let _32: [char; 1];
let _33: &'static mut (*mut u16,);
let _34: i8;
let _35: i64;
let _36: [u8; 8];
let _37: (u16, Adt21, *mut [i16; 1]);
let _38: &'static (f32, i8, i128);
let _39: f64;
let _40: ((f32, i8, i128), usize, &'static (f32, i8, i128));
let _41: *mut i32;
let _42: u8;
let _43: ();
let _44: ();
{
_2 = false;
_1 = 757861854_u32 as u64;
_2 = _1 == _1;
_2 = true;
_2 = _1 < _1;
_2 = true ^ false;
_1 = !10009611159227467741_u64;
_1 = 9486070914285414715_u64 * 15607809853945457118_u64;
_2 = _1 == _1;
_1 = 460_u16 as u64;
_2 = true ^ true;
_2 = _1 != _1;
_1 = 12962833867223158216_u64;
_2 = false;
_1 = !7914088139591463927_u64;
_1 = 2190218646993331735_u64;
RET = Adt21::Variant0 { fld0: 19_isize };
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize;
_2 = false | false;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
2190218646993331735 => bb9,
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
_2 = false;
_2 = _1 >= _1;
place!(Field::<isize>(Variant(RET, 0), 0)) = (-60_i8) as isize;
_1 = '\u{4ffb3}' as u64;
_1 = _2 as u64;
_1 = 4449259692040906139_u64 * 7905308484522099217_u64;
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize;
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize;
_2 = !false;
_2 = false;
_1 = !2873616740155768960_u64;
_1 = 13152964889182225420_u64;
_2 = !false;
_2 = !false;
_2 = Field::<isize>(Variant(RET, 0), 0) < Field::<isize>(Variant(RET, 0), 0);
_1 = 63_u8 as u64;
_2 = false;
_2 = !false;
place!(Field::<isize>(Variant(RET, 0), 0)) = 9223372036854775807_isize | (-45_isize);
RET = Adt21::Variant0 { fld0: (-59_isize) };
_6 = 4436837778977850150_i64;
_5 = -96_isize;
_2 = !false;
Call(_5 = core::intrinsics::transmute(_6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<isize>(Variant(RET, 0), 0)) = -_5;
_1 = 9865369150690012484_u64;
RET = Adt21::Variant0 { fld0: _5 };
_5 = Field::<isize>(Variant(RET, 0), 0) << Field::<isize>(Variant(RET, 0), 0);
_5 = Field::<isize>(Variant(RET, 0), 0);
_1 = !141441803877839919_u64;
_6 = 2600666553720821976_i64;
_6 = (-6543732046079598888_i64);
_1 = 1336142150162812851_u64 * 9346178232356283400_u64;
_5 = Field::<isize>(Variant(RET, 0), 0) & Field::<isize>(Variant(RET, 0), 0);
_2 = _6 > _6;
place!(Field::<isize>(Variant(RET, 0), 0)) = _5;
_6 = 8743470531943261477_i64 >> Field::<isize>(Variant(RET, 0), 0);
_5 = !Field::<isize>(Variant(RET, 0), 0);
_5 = Field::<isize>(Variant(RET, 0), 0);
RET = Adt21::Variant0 { fld0: _5 };
_5 = Field::<isize>(Variant(RET, 0), 0);
_7 = (-24612_i16);
_1 = 10132597557591327268_u64 & 16795370521614304031_u64;
place!(Field::<isize>(Variant(RET, 0), 0)) = _7 as isize;
_7 = -24070_i16;
place!(Field::<isize>(Variant(RET, 0), 0)) = _5;
_1 = !9572069256393085320_u64;
_1 = 8129588624776536271_u64 * 5319449024841235772_u64;
_3 = &_7;
_5 = (-53828303247782858630797313965027775998_i128) as isize;
_1 = 13332738216510195177_u64 * 4099886527508634894_u64;
Goto(bb11)
}
bb11 = {
Goto(bb12)
}
bb12 = {
_2 = _1 < _1;
_6 = (-336876784097769447_i64);
_10.0 = _6 as f32;
_10.0 = 36468_u16 as f32;
_5 = Field::<isize>(Variant(RET, 0), 0) | Field::<isize>(Variant(RET, 0), 0);
_11.0 = (_10.0, (-120_i8), (-81940724429766590903473027790703574202_i128));
_4.0 = core::ptr::addr_of_mut!(_12);
_11.0.1 = 6_i8;
_11.1 = 6_usize;
_4.0 = core::ptr::addr_of_mut!(_12);
_5 = !Field::<isize>(Variant(RET, 0), 0);
_10.1 = -_11.0.1;
_1 = _11.1 as u64;
_8 = _11.1;
match _6 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb11,
340282366920938463463037730647670442009 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<isize>(Variant(RET, 0), 0)) = -_5;
_11.0.2 = (-139521119499198310168207638277108637069_i128);
_14 = 77_u8 >> (*_3);
_11.1 = _8 | _8;
_6 = (-8716896506138434638_i64) + (-867716359612577683_i64);
_15 = _2;
_10 = _11.0;
_11.0.1 = _10.1 ^ _10.1;
place!(Field::<isize>(Variant(RET, 0), 0)) = _5 + _5;
_7 = 2386_i16 - (-25433_i16);
_13 = _11.1 as f64;
Call(_11.1 = fn3(), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_11.2 = &_11.0;
_11.2 = &_10;
_5 = _13 as isize;
_2 = !_15;
_9 = -_6;
_11.1 = _8 * _8;
_11.0.2 = _10.2 ^ _10.2;
_18 = &mut place!(Field::<isize>(Variant(RET, 0), 0));
_7 = (-22138_i16);
_10.2 = _11.0.2;
(*_18) = 3395742763_u32 as isize;
(*_18) = _5 ^ _5;
_17 = core::ptr::addr_of!(_11.2);
_13 = _14 as f64;
(*_17) = &_11.0;
(*_18) = _5 + _5;
(*_18) = !_5;
Goto(bb16)
}
bb16 = {
(*_17) = &_10;
(*_18) = _14 as isize;
(*_18) = -_5;
_5 = (*_18) << (*_18);
_17 = core::ptr::addr_of!((*_17));
(*_18) = _5;
_14 = 138_u8 << (*_18);
(*_18) = !_5;
_14 = 27_u8 * 144_u8;
_10.2 = _11.0.2 & _11.0.2;
_21.1 = ['\u{105cb3}'];
_11.2 = &_11.0;
(*_17) = &_10;
_7 = !(-2381_i16);
(*_17) = &_11.0;
(*_17) = &_10;
(*_18) = _5 - _5;
(*_18) = !_5;
match _8 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb12,
6 => bb17,
_ => bb14
}
}
bb17 = {
(*_17) = &_11.0;
(*_17) = &_10;
_12 = 10605_u16;
(*_18) = _14 as isize;
(*_18) = _5 * _5;
(*_17) = &_11.0;
Goto(bb18)
}
bb18 = {
(*_18) = -_5;
(*_17) = &_10;
(*_18) = _5;
(*_18) = _5 ^ _5;
_11.0.0 = _10.0;
_14 = !189_u8;
_25 = _13;
_19 = _10.1;
_11.2 = &_11.0;
(*_18) = _5 | _5;
(*_18) = !_5;
match _8 {
0 => bb2,
1 => bb19,
2 => bb20,
6 => bb22,
_ => bb21
}
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
_25 = _13 * _13;
(*_17) = &_10;
(*_18) = _5 & _5;
_11.1 = _8 + _8;
_5 = (*_18);
_29 = core::ptr::addr_of!(_9);
(*_29) = _6 ^ _6;
_11.0.2 = _15 as i128;
(*_18) = -_5;
Goto(bb23)
}
bb23 = {
_11.0 = (_10.0, _19, _10.2);
_12 = 18727_u16 << (*_18);
(*_17) = &_11.0;
Goto(bb24)
}
bb24 = {
(*_29) = _6 & _6;
(*_29) = _6 ^ _6;
(*_17) = &_10;
(*_29) = _6 * _6;
(*_17) = &_11.0;
_13 = _25 - _25;
_1 = _2 as u64;
(*_17) = &_10;
_21.1 = ['\u{c0bbe}'];
(*_17) = &_11.0;
(*_17) = &_10;
(*_29) = _6;
_36 = [_14,_14,_14,_14,_14,_14,_14,_14];
(*_17) = &_11.0;
(*_18) = _5 | _5;
_32 = _21.1;
_9 = _2 as i64;
_40.0 = _10;
Goto(bb25)
}
bb25 = {
Call(_43 = dump_var(Move(_15), Move(_6), Move(_14), Move(_19)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_43 = dump_var(Move(_12), Move(_7), _44, _44), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3() -> usize {
mir! {
type RET = usize;
let _1: f64;
let _2: Adt64;
let _3: f32;
let _4: *mut i32;
let _5: ((u16,), (char, i64, Adt17));
let _6: isize;
let _7: [u128; 7];
let _8: isize;
let _9: &'static Adt26;
let _10: i128;
let _11: i16;
let _12: (u32,);
let _13: *mut Adt52;
let _14: u128;
let _15: isize;
let _16: *mut *mut u16;
let _17: (f32, i8, i128);
let _18: char;
let _19: *const &'static (f32, i8, i128);
let _20: isize;
let _21: [i128; 8];
let _22: [u128; 7];
let _23: isize;
let _24: bool;
let _25: char;
let _26: f32;
let _27: &'static mut *const *const char;
let _28: *mut u16;
let _29: bool;
let _30: i8;
let _31: i8;
let _32: &'static mut *const *const char;
let _33: *mut u16;
let _34: (*mut u16,);
let _35: bool;
let _36: &'static Adt26;
let _37: f64;
let _38: [u16; 5];
let _39: isize;
let _40: *mut *mut u16;
let _41: [u16; 5];
let _42: isize;
let _43: f64;
let _44: isize;
let _45: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _46: bool;
let _47: [u8; 8];
let _48: Adt26;
let _49: f32;
let _50: [u16; 1];
let _51: isize;
let _52: u8;
let _53: bool;
let _54: (char, i64, Adt17);
let _55: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _56: char;
let _57: &'static mut &'static mut u32;
let _58: &'static Adt66;
let _59: f32;
let _60: f64;
let _61: isize;
let _62: *const &'static (f32, i8, i128);
let _63: *mut u16;
let _64: *mut i64;
let _65: *mut u16;
let _66: usize;
let _67: Adt17;
let _68: ();
let _69: ();
{
RET = !4615810970012656516_usize;
RET = 13078475547554494942_u64 as usize;
_1 = 38681888931225674108784021171111120335_i128 as f64;
RET = !7_usize;
RET = 219649782601929446292708670398961094295_u128 as usize;
_1 = 208639287339004047283638561811326642772_u128 as f64;
RET = !6605145283144108443_usize;
_1 = (-9223372036854775808_isize) as f64;
RET = 6_usize;
_1 = 12788622019474269939_u64 as f64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
6 => bb7,
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
_3 = (-148117590_i32) as f32;
RET = !5680437090035818286_usize;
_3 = 14477769815875785316_u64 as f32;
_1 = 14777130667379635872_u64 as f64;
_6 = (-602873071379593242_i64) as isize;
_5.1.0 = '\u{7c5b3}';
Call(_5.1.1 = fn4(_6, _5.1.0, RET, RET, _3, _5.1.0, _6, _6, _6, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5.0.0 = RET as u16;
_7 = [12838944454321223983698531310907532197_u128,209612334945062869600043091350067266476_u128,218463264623353131433779274763573189469_u128,6678269131729254444109682043065214754_u128,118569438825204184275980995928575939085_u128,222775945632281171630569768689644575257_u128,293076019186881815985168246762687614643_u128];
_5.0.0 = !8890_u16;
_5.1.2 = Adt17::Variant2 { fld0: 21806888074175818016548971020894053980_u128,fld1: _5.1.0,fld2: 175_u8,fld3: (-33_i8),fld4: _3,fld5: _5.1.1 };
place!(Field::<i8>(Variant(_5.1.2, 2), 3)) = (-17_i8) << _5.1.1;
_7 = [52867647960710647062971586873137876424_u128,66405766236795067730749861255079102600_u128,34959223751513199732697526101868831277_u128,131152897552030507832251211408342313525_u128,102812958997369917517979662906250760142_u128,197867877932360034825182323296109839030_u128,134347976883893363574632967515427041051_u128];
place!(Field::<i64>(Variant(_5.1.2, 2), 5)) = _5.1.1;
_5.1.1 = Field::<i64>(Variant(_5.1.2, 2), 5);
_3 = Field::<f32>(Variant(_5.1.2, 2), 4) + Field::<f32>(Variant(_5.1.2, 2), 4);
_8 = -_6;
Goto(bb9)
}
bb9 = {
_7 = [142310266507281743263104831189416505984_u128,223303075281442847070196893247986638227_u128,319298254274367739474416395557205311958_u128,214333918989555655360784492845770641073_u128,126458428583823285783838173088333574132_u128,321685938232622202066707224025961364628_u128,135548588998085429527543917222926669375_u128];
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = !302710275190394831479638157492456650265_u128;
_5.0 = (57423_u16,);
_5.0.0 = 53618081762218141729690649520514890690_i128 as u16;
place!(Field::<u8>(Variant(_5.1.2, 2), 2)) = (-505876894_i32) as u8;
_5.0 = (16343_u16,);
RET = !17973515445910170571_usize;
_5.1.1 = !Field::<i64>(Variant(_5.1.2, 2), 5);
_5.1.0 = Field::<char>(Variant(_5.1.2, 2), 1);
_10 = _5.1.0 as i128;
_1 = Field::<u128>(Variant(_5.1.2, 2), 0) as f64;
place!(Field::<u8>(Variant(_5.1.2, 2), 2)) = !13_u8;
_7 = [Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0)];
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = !1896473162065598494592511814237959068_u128;
place!(Field::<i64>(Variant(_5.1.2, 2), 5)) = 29071_i16 as i64;
_5.1.2 = Adt17::Variant1 { fld0: 176_u8 };
_5.1.1 = (-7997499327127916069_i64) - 1223115346382749853_i64;
_10 = 123501582164133133951307154708191024798_i128 << _6;
place!(Field::<u8>(Variant(_5.1.2, 1), 0)) = 20_u8;
_11 = 11626_i16 >> _5.1.1;
_3 = (-1840348366_i32) as f32;
_5.1.0 = '\u{10d079}';
_5.1.0 = '\u{c2efe}';
_5.1.0 = '\u{ab95e}';
place!(Field::<u8>(Variant(_5.1.2, 1), 0)) = 167_u8 - 232_u8;
_7 = [24153864267070996243158279459323906724_u128,23724626104687042207673923086629122557_u128,199640833008808087885402421986060144811_u128,59433501395236738474677251402447884426_u128,68189792977069309727278669712491715334_u128,8837108662288988609736153149773796615_u128,330129497450954503856599760245350184357_u128];
match _5.0.0 {
0 => bb8,
1 => bb2,
2 => bb6,
3 => bb4,
16343 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_5.1.2 = Adt17::Variant2 { fld0: 202402963025908063820317086736282956595_u128,fld1: _5.1.0,fld2: 186_u8,fld3: (-15_i8),fld4: _3,fld5: _5.1.1 };
_14 = !75164601641781114527217729942064386815_u128;
RET = !6_usize;
place!(Field::<u8>(Variant(_5.1.2, 2), 2)) = _5.0.0 as u8;
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = _14 + _14;
place!(Field::<i8>(Variant(_5.1.2, 2), 3)) = (-128_i8);
_6 = _5.1.0 as isize;
_6 = !_8;
Goto(bb12)
}
bb12 = {
_5.0 = (426_u16,);
_12 = (4131598744_u32,);
RET = !4_usize;
match Field::<i8>(Variant(_5.1.2, 2), 3) {
0 => bb11,
1 => bb7,
2 => bb4,
3 => bb13,
4 => bb14,
340282366920938463463374607431768211328 => bb16,
_ => bb15
}
}
bb13 = {
_5.0.0 = RET as u16;
_7 = [12838944454321223983698531310907532197_u128,209612334945062869600043091350067266476_u128,218463264623353131433779274763573189469_u128,6678269131729254444109682043065214754_u128,118569438825204184275980995928575939085_u128,222775945632281171630569768689644575257_u128,293076019186881815985168246762687614643_u128];
_5.0.0 = !8890_u16;
_5.1.2 = Adt17::Variant2 { fld0: 21806888074175818016548971020894053980_u128,fld1: _5.1.0,fld2: 175_u8,fld3: (-33_i8),fld4: _3,fld5: _5.1.1 };
place!(Field::<i8>(Variant(_5.1.2, 2), 3)) = (-17_i8) << _5.1.1;
_7 = [52867647960710647062971586873137876424_u128,66405766236795067730749861255079102600_u128,34959223751513199732697526101868831277_u128,131152897552030507832251211408342313525_u128,102812958997369917517979662906250760142_u128,197867877932360034825182323296109839030_u128,134347976883893363574632967515427041051_u128];
place!(Field::<i64>(Variant(_5.1.2, 2), 5)) = _5.1.1;
_5.1.1 = Field::<i64>(Variant(_5.1.2, 2), 5);
_3 = Field::<f32>(Variant(_5.1.2, 2), 4) + Field::<f32>(Variant(_5.1.2, 2), 4);
_8 = -_6;
Goto(bb9)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_14 = _8 as u128;
_6 = RET as isize;
_10 = _12.0 as i128;
_7 = [Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),_14,Field::<u128>(Variant(_5.1.2, 2), 0),_14,Field::<u128>(Variant(_5.1.2, 2), 0)];
_14 = !Field::<u128>(Variant(_5.1.2, 2), 0);
_6 = _5.1.1 as isize;
place!(Field::<char>(Variant(_5.1.2, 2), 1)) = _5.1.0;
_11 = -(-18520_i16);
_12 = (3259018840_u32,);
_12.0 = 2278827030_u32 * 3179349666_u32;
_11 = Field::<i64>(Variant(_5.1.2, 2), 5) as i16;
_5.1.2 = Adt17::Variant1 { fld0: 71_u8 };
Goto(bb17)
}
bb17 = {
_5.1.1 = (-1944064008110979060_i64) ^ 7782022857844866609_i64;
_15 = _6 ^ _8;
_7 = [_14,_14,_14,_14,_14,_14,_14];
_5.1.1 = 1616496139047479614_i64;
RET = !5_usize;
_6 = _15 + _15;
_12.0 = 2202259257_u32 >> _15;
_5.1.1 = -(-7231166842749179737_i64);
_5.0 = (10712_u16,);
_5.1.1 = 7354973948337647062_i64;
_18 = _5.1.0;
_5.1.2 = Adt17::Variant3 { fld0: _14,fld1: _18,fld2: _12.0,fld3: _1,fld4: 14435314528714054117_u64,fld5: 604733720_i32,fld6: _5.1.1,fld7: _10 };
_7 = [Field::<u128>(Variant(_5.1.2, 3), 0),_14,Field::<u128>(Variant(_5.1.2, 3), 0),_14,_14,Field::<u128>(Variant(_5.1.2, 3), 0),_14];
_17.1 = true as i8;
_17.2 = _10 + _10;
_1 = _3 as f64;
_4 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_5.1.2, 3), 5)));
match _5.1.1 {
0 => bb15,
1 => bb16,
2 => bb18,
7354973948337647062 => bb20,
_ => bb19
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
(*_4) = (-1731281383_i32) - (-815543151_i32);
_22 = [_14,Field::<u128>(Variant(_5.1.2, 3), 0),Field::<u128>(Variant(_5.1.2, 3), 0),_14,_14,Field::<u128>(Variant(_5.1.2, 3), 0),_14];
place!(Field::<i64>(Variant(_5.1.2, 3), 6)) = _5.1.1 >> (*_4);
(*_4) = !1719127307_i32;
Goto(bb21)
}
bb21 = {
(*_4) = (-336007809_i32) | 1916008340_i32;
_15 = _6 | _6;
_6 = -_15;
_1 = -Field::<f64>(Variant(_5.1.2, 3), 3);
_15 = !_6;
(*_4) = 203_u8 as i32;
place!(Field::<u32>(Variant(_5.1.2, 3), 2)) = _12.0 << (*_4);
(*_4) = _5.0.0 as i32;
match _5.0.0 {
10712 => bb22,
_ => bb20
}
}
bb22 = {
(*_4) = _5.1.1 as i32;
(*_4) = (-1806713764_i32) ^ 2037464118_i32;
_15 = _6 - _6;
_10 = Field::<i128>(Variant(_5.1.2, 3), 7) ^ Field::<i128>(Variant(_5.1.2, 3), 7);
_6 = _15 + _15;
place!(Field::<i128>(Variant(_5.1.2, 3), 7)) = _17.2 | _10;
(*_4) = 1887087994_i32 - 1675137254_i32;
(*_4) = -(-1757925966_i32);
_5.1.2 = Adt17::Variant0 { fld0: _1,fld1: _18,fld2: _15,fld3: _14,fld4: _11,fld5: _3 };
_5.0 = (6758_u16,);
_3 = Field::<f32>(Variant(_5.1.2, 0), 5) + Field::<f32>(Variant(_5.1.2, 0), 5);
_15 = Field::<isize>(Variant(_5.1.2, 0), 2) << Field::<isize>(Variant(_5.1.2, 0), 2);
RET = 4619198318727756696_usize;
_29 = !false;
_5.1.0 = Field::<char>(Variant(_5.1.2, 0), 1);
_17.0 = _3 + _3;
_28 = core::ptr::addr_of_mut!(_5.0.0);
_16 = core::ptr::addr_of_mut!(_28);
_24 = Field::<isize>(Variant(_5.1.2, 0), 2) > _6;
_29 = (*_28) == (*_28);
_24 = _29 & _29;
match (*_28) {
0 => bb1,
1 => bb9,
2 => bb18,
3 => bb23,
4 => bb24,
5 => bb25,
6758 => bb27,
_ => bb26
}
}
bb23 = {
Return()
}
bb24 = {
_3 = (-148117590_i32) as f32;
RET = !5680437090035818286_usize;
_3 = 14477769815875785316_u64 as f32;
_1 = 14777130667379635872_u64 as f64;
_6 = (-602873071379593242_i64) as isize;
_5.1.0 = '\u{7c5b3}';
Call(_5.1.1 = fn4(_6, _5.1.0, RET, RET, _3, _5.1.0, _6, _6, _6, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb25 = {
_5.0 = (426_u16,);
_12 = (4131598744_u32,);
RET = !4_usize;
match Field::<i8>(Variant(_5.1.2, 2), 3) {
0 => bb11,
1 => bb7,
2 => bb4,
3 => bb13,
4 => bb14,
340282366920938463463374607431768211328 => bb16,
_ => bb15
}
}
bb26 = {
Return()
}
bb27 = {
_25 = _5.1.0;
_17.0 = _3;
(*_16) = core::ptr::addr_of_mut!((*_28));
_14 = _5.1.1 as u128;
(*_16) = core::ptr::addr_of_mut!((*_28));
(*_16) = core::ptr::addr_of_mut!((*_28));
_26 = _17.0;
_6 = Field::<isize>(Variant(_5.1.2, 0), 2) - Field::<isize>(Variant(_5.1.2, 0), 2);
(*_16) = core::ptr::addr_of_mut!((*_28));
(*_16) = core::ptr::addr_of_mut!((*_28));
_26 = _17.0;
(*_16) = core::ptr::addr_of_mut!((*_28));
(*_28) = 29774_u16 - 1212_u16;
(*_16) = core::ptr::addr_of_mut!((*_28));
_21 = [_17.2,_17.2,_10,_17.2,_10,_17.2,_17.2,_17.2];
_10 = _17.2 >> _6;
_25 = Field::<char>(Variant(_5.1.2, 0), 1);
(*_16) = core::ptr::addr_of_mut!((*_28));
Goto(bb28)
}
bb28 = {
_26 = _17.0;
_34 = (Move((*_16)),);
(*_16) = core::ptr::addr_of_mut!(_5.0.0);
(*_16) = core::ptr::addr_of_mut!((*_28));
_35 = _15 == _15;
(*_16) = core::ptr::addr_of_mut!(_5.0.0);
_33 = core::ptr::addr_of_mut!((*_28));
(*_16) = Move(_33);
_37 = -Field::<f64>(Variant(_5.1.2, 0), 0);
match _5.1.1 {
0 => bb6,
1 => bb16,
2 => bb11,
3 => bb29,
4 => bb30,
7354973948337647062 => bb32,
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
(*_4) = (-1731281383_i32) - (-815543151_i32);
_22 = [_14,Field::<u128>(Variant(_5.1.2, 3), 0),Field::<u128>(Variant(_5.1.2, 3), 0),_14,_14,Field::<u128>(Variant(_5.1.2, 3), 0),_14];
place!(Field::<i64>(Variant(_5.1.2, 3), 6)) = _5.1.1 >> (*_4);
(*_4) = !1719127307_i32;
Goto(bb21)
}
bb32 = {
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),_14,Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3)];
_23 = _15 << _15;
_33 = Move((*_16));
place!(Field::<u128>(Variant(_5.1.2, 0), 3)) = _14 - _14;
_3 = _17.0 - _26;
_17.2 = Field::<u128>(Variant(_5.1.2, 0), 3) as i128;
_38 = [_5.0.0,_5.0.0,_5.0.0,_5.0.0,_5.0.0];
_31 = !_17.1;
(*_16) = core::ptr::addr_of_mut!(_5.0.0);
_23 = _15 ^ _15;
_5.1.1 = (-786449335537631553_i64) + 6531743820072285831_i64;
(*_16) = core::ptr::addr_of_mut!((*_28));
Goto(bb33)
}
bb33 = {
_17.2 = _10;
_30 = _17.1 << _23;
(*_28) = !61627_u16;
_40 = Move(_16);
_11 = Field::<i16>(Variant(_5.1.2, 0), 4) | Field::<i16>(Variant(_5.1.2, 0), 4);
_12 = (3975569750_u32,);
(*_28) = 33358_u16 - 37318_u16;
(*_28) = _30 as u16;
(*_28) = 11288_u16 & 1306_u16;
_12.0 = 3755882442_u32 * 3529944504_u32;
_24 = _35;
place!(Field::<u128>(Variant(_5.1.2, 0), 3)) = _14 | _14;
(*_28) = 36235_u16;
_42 = 11731626160261497064_u64 as isize;
_20 = _12.0 as isize;
_17 = (Field::<f32>(Variant(_5.1.2, 0), 5), _30, _10);
(*_28) = 15494_u16 << _23;
(*_28) = 24334_u16;
_23 = _6 + _6;
_41 = _38;
_17.2 = _10 << _30;
_17.1 = _30;
Goto(bb34)
}
bb34 = {
_31 = !_17.1;
(*_28) = !14078_u16;
_21 = [_10,_17.2,_17.2,_17.2,_10,_17.2,_10,_10];
_30 = _35 as i8;
(*_28) = 49230_u16;
_22 = _7;
_5.1.1 = 567742349049707524_i64 ^ (-4582702520035606468_i64);
_12 = (3815417881_u32,);
_30 = _31 & _31;
_34.0 = core::ptr::addr_of_mut!((*_28));
place!(Field::<isize>(Variant(_5.1.2, 0), 2)) = 238_u8 as isize;
(*_28) = 48606_u16 ^ 4165_u16;
(*_28) = 56408_u16;
_16 = core::ptr::addr_of_mut!(_28);
(*_28) = 15812_u16;
_45.1.2 = &_17;
match (*_28) {
0 => bb28,
1 => bb4,
15812 => bb36,
_ => bb35
}
}
bb35 = {
Return()
}
bb36 = {
(*_28) = 8714_u16 * 12773_u16;
_34.0 = core::ptr::addr_of_mut!((*_28));
(*_28) = 13313_u16 + 55852_u16;
(*_16) = core::ptr::addr_of_mut!((*_28));
(*_16) = core::ptr::addr_of_mut!((*_28));
_12.0 = !4022965775_u32;
(*_28) = 19898_u16;
(*_28) = 3622_u16;
_46 = _24;
(*_28) = 20926_u16;
_5.0 = (52516_u16,);
_45.0 = Adt21::Variant0 { fld0: _23 };
(*_16) = core::ptr::addr_of_mut!((*_28));
(*_16) = core::ptr::addr_of_mut!((*_28));
_45.1.1 = RET;
(*_16) = core::ptr::addr_of_mut!((*_28));
match (*_28) {
0 => bb10,
52516 => bb37,
_ => bb25
}
}
bb37 = {
_11 = Field::<i16>(Variant(_5.1.2, 0), 4) + Field::<i16>(Variant(_5.1.2, 0), 4);
(*_16) = core::ptr::addr_of_mut!((*_28));
_48.fld2 = (_5.0.0,);
_16 = core::ptr::addr_of_mut!((*_16));
match (*_28) {
52516 => bb39,
_ => bb38
}
}
bb38 = {
_17.2 = _10;
_30 = _17.1 << _23;
(*_28) = !61627_u16;
_40 = Move(_16);
_11 = Field::<i16>(Variant(_5.1.2, 0), 4) | Field::<i16>(Variant(_5.1.2, 0), 4);
_12 = (3975569750_u32,);
(*_28) = 33358_u16 - 37318_u16;
(*_28) = _30 as u16;
(*_28) = 11288_u16 & 1306_u16;
_12.0 = 3755882442_u32 * 3529944504_u32;
_24 = _35;
place!(Field::<u128>(Variant(_5.1.2, 0), 3)) = _14 | _14;
(*_28) = 36235_u16;
_42 = 11731626160261497064_u64 as isize;
_20 = _12.0 as isize;
_17 = (Field::<f32>(Variant(_5.1.2, 0), 5), _30, _10);
(*_28) = 15494_u16 << _23;
(*_28) = 24334_u16;
_23 = _6 + _6;
_41 = _38;
_17.2 = _10 << _30;
_17.1 = _30;
Goto(bb34)
}
bb39 = {
_41 = [(*_28),(*_28),(*_28),(*_28),(*_28)];
_33 = Move((*_16));
_48 = Adt26 { fld0: 5139360038410507660_u64,fld1: _5.0.0,fld2: _5.0,fld3: _45.1.1,fld4: _14,fld5: _45.0 };
(*_16) = core::ptr::addr_of_mut!(_48.fld1);
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),_14,Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),_48.fld4,Field::<u128>(Variant(_5.1.2, 0), 3),_48.fld4];
_34.0 = core::ptr::addr_of_mut!(_48.fld1);
_47 = [19_u8,70_u8,190_u8,234_u8,65_u8,172_u8,167_u8,192_u8];
_31 = _6 as i8;
(*_28) = _5.0.0 + _48.fld2.0;
(*_28) = _48.fld2.0;
_36 = &_48;
_1 = _37 * _37;
(*_16) = core::ptr::addr_of_mut!(_48.fld2.0);
(*_16) = core::ptr::addr_of_mut!((*_28));
Goto(bb40)
}
bb40 = {
_48.fld1 = (*_28) - (*_28);
place!(Field::<isize>(Variant(_45.0, 0), 0)) = Field::<isize>(Variant((*_36).fld5, 0), 0) | Field::<isize>(Variant((*_36).fld5, 0), 0);
_5.1.0 = _25;
_6 = Field::<isize>(Variant((*_36).fld5, 0), 0) - Field::<isize>(Variant((*_36).fld5, 0), 0);
_48.fld4 = _14;
_16 = Move(_40);
_26 = (*_36).fld0 as f32;
_40 = core::ptr::addr_of_mut!(_34.0);
_39 = _10 as isize;
_11 = 182_u8 as i16;
_31 = -_30;
place!(Field::<u128>(Variant(_5.1.2, 0), 3)) = _14 >> (*_36).fld0;
_26 = Field::<f32>(Variant(_5.1.2, 0), 5);
place!(Field::<f32>(Variant(_5.1.2, 0), 5)) = _26 * _3;
(*_40) = core::ptr::addr_of_mut!((*_28));
(*_40) = core::ptr::addr_of_mut!((*_28));
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),_14,Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3)];
_19 = core::ptr::addr_of!(_45.1.2);
_48.fld1 = (*_28);
_53 = _35;
_41 = [(*_28),(*_36).fld2.0,(*_36).fld2.0,(*_36).fld2.0,(*_36).fld2.0];
_23 = -Field::<isize>(Variant((*_36).fld5, 0), 0);
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3)];
_12.0 = 4112572755_u32 << (*_36).fld0;
_1 = _3 as f64;
Goto(bb41)
}
bb41 = {
(*_40) = core::ptr::addr_of_mut!(_48.fld2.0);
_48.fld0 = 11705515966319832857_u64 << Field::<isize>(Variant(_48.fld5, 0), 0);
(*_40) = core::ptr::addr_of_mut!((*_28));
_5.1.0 = _25;
_43 = Field::<f64>(Variant(_5.1.2, 0), 0) + _1;
_17.1 = _30;
_29 = _46;
_45.1.0.0 = (*_28) as f32;
_48.fld4 = !Field::<u128>(Variant(_5.1.2, 0), 3);
_40 = core::ptr::addr_of_mut!((*_40));
_9 = Move(_36);
_55.0.2 = [(*_28)];
_5.1.0 = _18;
_9 = &_48;
_5.0 = (*_9).fld2;
_54 = (Field::<char>(Variant(_5.1.2, 0), 1), _5.1.1, Move(_5.1.2));
_14 = _54.1 as u128;
_8 = _6 << (*_9).fld4;
_53 = !_29;
_38 = [_48.fld2.0,(*_28),(*_28),(*_9).fld2.0,_48.fld1];
_15 = Field::<isize>(Variant((*_9).fld5, 0), 0) >> Field::<isize>(Variant((*_9).fld5, 0), 0);
_54.1 = _5.1.1 ^ _5.1.1;
_54.2 = Adt17::Variant2 { fld0: (*_9).fld4,fld1: _54.0,fld2: 79_u8,fld3: _17.1,fld4: _3,fld5: _5.1.1 };
_55.0.3 = Move(_4);
_37 = _43 - _43;
match (*_9).fld1 {
0 => bb29,
1 => bb42,
2 => bb43,
3 => bb44,
52516 => bb46,
_ => bb45
}
}
bb42 = {
_48.fld1 = (*_28) - (*_28);
place!(Field::<isize>(Variant(_45.0, 0), 0)) = Field::<isize>(Variant((*_36).fld5, 0), 0) | Field::<isize>(Variant((*_36).fld5, 0), 0);
_5.1.0 = _25;
_6 = Field::<isize>(Variant((*_36).fld5, 0), 0) - Field::<isize>(Variant((*_36).fld5, 0), 0);
_48.fld4 = _14;
_16 = Move(_40);
_26 = (*_36).fld0 as f32;
_40 = core::ptr::addr_of_mut!(_34.0);
_39 = _10 as isize;
_11 = 182_u8 as i16;
_31 = -_30;
place!(Field::<u128>(Variant(_5.1.2, 0), 3)) = _14 >> (*_36).fld0;
_26 = Field::<f32>(Variant(_5.1.2, 0), 5);
place!(Field::<f32>(Variant(_5.1.2, 0), 5)) = _26 * _3;
(*_40) = core::ptr::addr_of_mut!((*_28));
(*_40) = core::ptr::addr_of_mut!((*_28));
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),_14,Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3)];
_19 = core::ptr::addr_of!(_45.1.2);
_48.fld1 = (*_28);
_53 = _35;
_41 = [(*_28),(*_36).fld2.0,(*_36).fld2.0,(*_36).fld2.0,(*_36).fld2.0];
_23 = -Field::<isize>(Variant((*_36).fld5, 0), 0);
_22 = [Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3),Field::<u128>(Variant(_5.1.2, 0), 3)];
_12.0 = 4112572755_u32 << (*_36).fld0;
_1 = _3 as f64;
Goto(bb41)
}
bb43 = {
Return()
}
bb44 = {
_3 = (-148117590_i32) as f32;
RET = !5680437090035818286_usize;
_3 = 14477769815875785316_u64 as f32;
_1 = 14777130667379635872_u64 as f64;
_6 = (-602873071379593242_i64) as isize;
_5.1.0 = '\u{7c5b3}';
Call(_5.1.1 = fn4(_6, _5.1.0, RET, RET, _3, _5.1.0, _6, _6, _6, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb45 = {
_5.0 = (426_u16,);
_12 = (4131598744_u32,);
RET = !4_usize;
match Field::<i8>(Variant(_5.1.2, 2), 3) {
0 => bb11,
1 => bb7,
2 => bb4,
3 => bb13,
4 => bb14,
340282366920938463463374607431768211328 => bb16,
_ => bb15
}
}
bb46 = {
(*_40) = core::ptr::addr_of_mut!((*_9).fld2.0);
_8 = !Field::<isize>(Variant((*_9).fld5, 0), 0);
(*_40) = core::ptr::addr_of_mut!((*_28));
place!(Field::<f32>(Variant(_54.2, 2), 4)) = _45.1.0.0;
_6 = !Field::<isize>(Variant(_45.0, 0), 0);
_8 = (-888935284_i32) as isize;
_45.1.1 = (*_9).fld3 * (*_9).fld3;
_60 = -_37;
_42 = Field::<isize>(Variant((*_9).fld5, 0), 0) - Field::<isize>(Variant((*_9).fld5, 0), 0);
_48.fld0 = !12488473542799730022_u64;
_5.1.2 = Adt17::Variant2 { fld0: (*_9).fld4,fld1: _18,fld2: 12_u8,fld3: _31,fld4: _3,fld5: _54.1 };
_40 = core::ptr::addr_of_mut!((*_40));
_45.1.0.0 = 237776726_i32 as f32;
_14 = (*_9).fld4;
_55.0.1 = [(*_9).fld2.0];
place!(Field::<i64>(Variant(_54.2, 2), 5)) = -_5.1.1;
(*_40) = core::ptr::addr_of_mut!((*_9).fld1);
_48.fld4 = Field::<u128>(Variant(_5.1.2, 2), 0) << Field::<u128>(Variant(_5.1.2, 2), 0);
_49 = _10 as f32;
place!(Field::<f32>(Variant(_5.1.2, 2), 4)) = _49;
_11 = (-1898_i16) * (-14592_i16);
_38 = _41;
(*_40) = core::ptr::addr_of_mut!((*_28));
_55.1 = !_24;
_25 = Field::<char>(Variant(_54.2, 2), 1);
place!(Field::<f32>(Variant(_5.1.2, 2), 4)) = _49 + _49;
place!(Field::<f32>(Variant(_54.2, 2), 4)) = -_49;
match (*_9).fld1 {
0 => bb15,
1 => bb29,
52516 => bb48,
_ => bb47
}
}
bb47 = {
_7 = [142310266507281743263104831189416505984_u128,223303075281442847070196893247986638227_u128,319298254274367739474416395557205311958_u128,214333918989555655360784492845770641073_u128,126458428583823285783838173088333574132_u128,321685938232622202066707224025961364628_u128,135548588998085429527543917222926669375_u128];
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = !302710275190394831479638157492456650265_u128;
_5.0 = (57423_u16,);
_5.0.0 = 53618081762218141729690649520514890690_i128 as u16;
place!(Field::<u8>(Variant(_5.1.2, 2), 2)) = (-505876894_i32) as u8;
_5.0 = (16343_u16,);
RET = !17973515445910170571_usize;
_5.1.1 = !Field::<i64>(Variant(_5.1.2, 2), 5);
_5.1.0 = Field::<char>(Variant(_5.1.2, 2), 1);
_10 = _5.1.0 as i128;
_1 = Field::<u128>(Variant(_5.1.2, 2), 0) as f64;
place!(Field::<u8>(Variant(_5.1.2, 2), 2)) = !13_u8;
_7 = [Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0)];
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = !1896473162065598494592511814237959068_u128;
place!(Field::<i64>(Variant(_5.1.2, 2), 5)) = 29071_i16 as i64;
_5.1.2 = Adt17::Variant1 { fld0: 176_u8 };
_5.1.1 = (-7997499327127916069_i64) - 1223115346382749853_i64;
_10 = 123501582164133133951307154708191024798_i128 << _6;
place!(Field::<u8>(Variant(_5.1.2, 1), 0)) = 20_u8;
_11 = 11626_i16 >> _5.1.1;
_3 = (-1840348366_i32) as f32;
_5.1.0 = '\u{10d079}';
_5.1.0 = '\u{c2efe}';
_5.1.0 = '\u{ab95e}';
place!(Field::<u8>(Variant(_5.1.2, 1), 0)) = 167_u8 - 232_u8;
_7 = [24153864267070996243158279459323906724_u128,23724626104687042207673923086629122557_u128,199640833008808087885402421986060144811_u128,59433501395236738474677251402447884426_u128,68189792977069309727278669712491715334_u128,8837108662288988609736153149773796615_u128,330129497450954503856599760245350184357_u128];
match _5.0.0 {
0 => bb8,
1 => bb2,
2 => bb6,
3 => bb4,
16343 => bb11,
_ => bb10
}
}
bb48 = {
_45.1.0.2 = Field::<isize>(Variant((*_9).fld5, 0), 0) as i128;
(*_40) = core::ptr::addr_of_mut!((*_28));
_5.0.0 = _11 as u16;
_10 = (*_9).fld3 as i128;
_55.0.1 = [(*_28)];
match (*_9).fld1 {
0 => bb1,
1 => bb7,
2 => bb14,
3 => bb33,
4 => bb49,
5 => bb50,
6 => bb51,
52516 => bb53,
_ => bb52
}
}
bb49 = {
Return()
}
bb50 = {
_14 = _8 as u128;
_6 = RET as isize;
_10 = _12.0 as i128;
_7 = [Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),Field::<u128>(Variant(_5.1.2, 2), 0),_14,Field::<u128>(Variant(_5.1.2, 2), 0),_14,Field::<u128>(Variant(_5.1.2, 2), 0)];
_14 = !Field::<u128>(Variant(_5.1.2, 2), 0);
_6 = _5.1.1 as isize;
place!(Field::<char>(Variant(_5.1.2, 2), 1)) = _5.1.0;
_11 = -(-18520_i16);
_12 = (3259018840_u32,);
_12.0 = 2278827030_u32 * 3179349666_u32;
_11 = Field::<i64>(Variant(_5.1.2, 2), 5) as i16;
_5.1.2 = Adt17::Variant1 { fld0: 71_u8 };
Goto(bb17)
}
bb51 = {
_5.0 = (426_u16,);
_12 = (4131598744_u32,);
RET = !4_usize;
match Field::<i8>(Variant(_5.1.2, 2), 3) {
0 => bb11,
1 => bb7,
2 => bb4,
3 => bb13,
4 => bb14,
340282366920938463463374607431768211328 => bb16,
_ => bb15
}
}
bb52 = {
Return()
}
bb53 = {
_5.1.0 = _54.0;
place!(Field::<char>(Variant(_54.2, 2), 1)) = _54.0;
_21 = [_17.2,_45.1.0.2,_17.2,_17.2,_45.1.0.2,_17.2,_17.2,_45.1.0.2];
_5.0.0 = (*_28) / (*_9).fld1;
_4 = Move(_55.0.3);
(*_40) = core::ptr::addr_of_mut!((*_9).fld1);
_1 = _60 + _60;
place!(Field::<u128>(Variant(_5.1.2, 2), 0)) = _14 & _48.fld4;
_62 = core::ptr::addr_of!((*_19));
(*_40) = Move(_33);
_38 = [(*_9).fld1,_5.0.0,(*_28),(*_9).fld2.0,(*_9).fld2.0];
_65 = core::ptr::addr_of_mut!((*_28));
(*_40) = core::ptr::addr_of_mut!((*_65));
_8 = Field::<isize>(Variant((*_9).fld5, 0), 0);
_40 = core::ptr::addr_of_mut!((*_40));
_5.1.0 = Field::<char>(Variant(_54.2, 2), 1);
Goto(bb54)
}
bb54 = {
Call(_68 = dump_var(Move(_6), Move(_18), Move(_24), Move(_12)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_68 = dump_var(Move(_23), Move(_35), Move(_53), Move(_25)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_68 = dump_var(Move(_46), Move(_20), Move(_38), Move(_8)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_68 = dump_var(Move(_31), _69, _69, _69), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: char,mut _3: usize,mut _4: usize,mut _5: f32,mut _6: char,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> i64 {
mir! {
type RET = i64;
let _11: u128;
let _12: bool;
let _13: *mut i64;
let _14: *mut i64;
let _15: *const &'static char;
let _16: *const *const char;
let _17: isize;
let _18: *mut Adt52;
let _19: bool;
let _20: u128;
let _21: &'static [i128; 8];
let _22: char;
let _23: [u32; 6];
let _24: u32;
let _25: i64;
let _26: f64;
let _27: bool;
let _28: u8;
let _29: (u32,);
let _30: u64;
let _31: &'static char;
let _32: (u16,);
let _33: &'static mut (*mut u16,);
let _34: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _35: f32;
let _36: (*mut u16,);
let _37: f32;
let _38: u8;
let _39: f32;
let _40: isize;
let _41: char;
let _42: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _43: char;
let _44: [u16; 5];
let _45: (*const &'static (f32, i8, i128),);
let _46: *mut &'static char;
let _47: u32;
let _48: (u32,);
let _49: char;
let _50: &'static mut *const *const char;
let _51: ();
let _52: ();
{
_7 = _1 | _1;
_3 = !_4;
_10 = -_9;
_11 = 320246483530705280980476987616740214758_u128 << _8;
_7 = -_10;
_1 = !_8;
_5 = _11 as f32;
_7 = _9 << _11;
Goto(bb1)
}
bb1 = {
RET = !4092669005163364680_i64;
RET = !1802283129296403097_i64;
_8 = _1 << _11;
_5 = (-1253684487_i32) as f32;
_2 = _6;
_11 = 155195192269923477095542088884875875550_u128 >> _8;
_12 = _7 < _10;
_7 = !_8;
_12 = _11 > _11;
_3 = _4 * _4;
_13 = core::ptr::addr_of_mut!(RET);
(*_13) = 33003_u16 as i64;
(*_13) = 2120628498894259741_i64 & (-2060353600268439082_i64);
(*_13) = 249_u8 as i64;
(*_13) = (-6619652680697529612_i64) ^ (-845424196154788406_i64);
(*_13) = 8707567191376580788_i64 | 3805880480745485910_i64;
_2 = _6;
(*_13) = (-8257150371211998195_i64);
(*_13) = -(-703099804503598531_i64);
_9 = _8 & _7;
(*_13) = -2260430056473806447_i64;
Goto(bb2)
}
bb2 = {
_11 = 26641807393369079624426932912785520220_u128 << RET;
(*_13) = _6 as i64;
_14 = core::ptr::addr_of_mut!((*_13));
_3 = !_4;
(*_13) = _4 as i64;
Call(_4 = fn5(_10, Move(_13), Move(_14), (*_13), (*_13), (*_13), _9, _9, (*_13), (*_13)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _6;
_12 = _9 >= _9;
_6 = _2;
_14 = core::ptr::addr_of_mut!(RET);
_7 = 212_u8 as isize;
_14 = core::ptr::addr_of_mut!((*_14));
_2 = _6;
_1 = _12 as isize;
_3 = !_4;
(*_14) = (-6750218053343514791_i64);
(*_14) = 1275844237_u32 as i64;
(*_14) = 3030261507477983409_i64 * 7824253785396623729_i64;
_17 = _1 - _9;
_13 = core::ptr::addr_of_mut!((*_14));
_17 = !_9;
Goto(bb4)
}
bb4 = {
(*_13) = (-948390968155220862_i64);
(*_13) = (-2403972056567947587_i64);
_7 = 3130124306_u32 as isize;
(*_14) = (-78483559802572039_i64) + 5068632651429880_i64;
_19 = (*_14) > (*_14);
_7 = 620131509_i32 as isize;
_2 = _6;
(*_14) = (-3538527089303850440_i64);
Goto(bb5)
}
bb5 = {
_22 = _6;
(*_14) = 2073501331_i32 as i64;
_17 = _1 - _1;
(*_14) = 137079310390931387919848953254523080752_i128 as i64;
(*_14) = (-14171_i16) as i64;
_22 = _2;
_25 = (*_14) ^ (*_14);
_20 = _11 | _11;
(*_14) = _25 << _1;
(*_14) = _25;
_22 = _6;
_7 = _9 << _17;
(*_14) = _25 - _25;
(*_14) = 1242033492_i32 as i64;
_23 = [415166450_u32,2442922707_u32,4115098656_u32,2327273110_u32,3402669937_u32,3645455197_u32];
(*_14) = _25 + _25;
_2 = _6;
(*_14) = -_25;
Goto(bb6)
}
bb6 = {
_1 = _7;
_2 = _22;
_26 = _5 as f64;
(*_14) = _25 >> _7;
_24 = 739272506_u32 << (*_14);
(*_14) = _25;
(*_14) = _26 as i64;
RET = _25 >> _24;
_25 = _4 as i64;
_27 = _1 < _8;
(*_14) = _25;
_7 = _1 | _9;
_4 = 87_i8 as usize;
(*_14) = _25 << _24;
_7 = 6500476414874877936_u64 as isize;
_27 = !_12;
_14 = core::ptr::addr_of_mut!((*_14));
(*_14) = !_25;
(*_14) = (-113225038578654318304060337307782213715_i128) as i64;
_28 = 77_u8 ^ 205_u8;
(*_14) = _25 - _25;
_22 = _2;
(*_14) = _25 ^ _25;
_25 = -(*_14);
(*_14) = _25 >> _17;
_24 = 3293317411_u32 >> (*_14);
(*_14) = _25 * _25;
Goto(bb7)
}
bb7 = {
_8 = _17;
(*_14) = -_25;
(*_14) = !_25;
_34.1.0 = (_5, (-32_i8), 4745801547400449482174227504737675818_i128);
_34.1.0 = (_5, 47_i8, 27677953475069400813197325999816683700_i128);
_34.1.1 = !_3;
_34.2 = &_34.1.0;
_19 = _1 > _8;
(*_14) = _25 - _25;
Goto(bb8)
}
bb8 = {
_1 = _17 >> (*_14);
(*_14) = -_25;
_8 = _34.1.0.1 as isize;
_17 = -_1;
(*_14) = _25 * _25;
(*_14) = _26 as i64;
_34.0 = Adt21::Variant2 { fld0: _24,fld1: _2,fld2: _34.1.0.0,fld3: _26,fld4: _34.1.1 };
(*_14) = _25 + _25;
_32 = (47817_u16,);
_7 = !_17;
_38 = _28;
_24 = !Field::<u32>(Variant(_34.0, 2), 0);
_32.0 = !52689_u16;
_37 = _1 as f32;
place!(Field::<char>(Variant(_34.0, 2), 1)) = _6;
_29 = (_24,);
(*_14) = _25 >> _17;
RET = _11 as i64;
match _34.1.0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
27677953475069400813197325999816683700 => bb10,
_ => bb9
}
}
bb9 = {
_2 = _6;
_12 = _9 >= _9;
_6 = _2;
_14 = core::ptr::addr_of_mut!(RET);
_7 = 212_u8 as isize;
_14 = core::ptr::addr_of_mut!((*_14));
_2 = _6;
_1 = _12 as isize;
_3 = !_4;
(*_14) = (-6750218053343514791_i64);
(*_14) = 1275844237_u32 as i64;
(*_14) = 3030261507477983409_i64 * 7824253785396623729_i64;
_17 = _1 - _9;
_13 = core::ptr::addr_of_mut!((*_14));
_17 = !_9;
Goto(bb4)
}
bb10 = {
_34.1.0.0 = _34.1.0.1 as f32;
_29 = (Field::<u32>(Variant(_34.0, 2), 0),);
_4 = _34.1.1 ^ _34.1.1;
_41 = _22;
_4 = _3;
_42.1.2 = &_34.1.0;
place!(Field::<usize>(Variant(_34.0, 2), 4)) = _19 as usize;
_36.0 = core::ptr::addr_of_mut!(_32.0);
_34.0 = Adt21::Variant2 { fld0: _29.0,fld1: _22,fld2: _37,fld3: _26,fld4: _3 };
_44 = [_32.0,_32.0,_32.0,_32.0,_32.0];
Goto(bb11)
}
bb11 = {
_2 = _6;
(*_14) = !_25;
_42.0 = Adt21::Variant2 { fld0: Field::<u32>(Variant(_34.0, 2), 0),fld1: Field::<char>(Variant(_34.0, 2), 1),fld2: Field::<f32>(Variant(_34.0, 2), 2),fld3: Field::<f64>(Variant(_34.0, 2), 3),fld4: Field::<usize>(Variant(_34.0, 2), 4) };
_7 = _17 >> _1;
(*_14) = _25 >> _3;
(*_14) = !_25;
_44 = [_32.0,_32.0,_32.0,_32.0,_32.0];
_42.1.0 = _34.1.0;
(*_14) = _25;
_47 = !_29.0;
_30 = 18253215817212383126_u64 | 3451611058477996722_u64;
place!(Field::<f32>(Variant(_42.0, 2), 2)) = Field::<f64>(Variant(_34.0, 2), 3) as f32;
(*_14) = 8316_i16 as i64;
_23 = [_29.0,_29.0,Field::<u32>(Variant(_34.0, 2), 0),_29.0,Field::<u32>(Variant(_42.0, 2), 0),Field::<u32>(Variant(_42.0, 2), 0)];
(*_14) = _25 & _25;
(*_14) = _25 & _25;
_2 = Field::<char>(Variant(_42.0, 2), 1);
_38 = !_28;
_10 = _1;
_29 = (Field::<u32>(Variant(_34.0, 2), 0),);
_2 = _22;
(*_14) = !_25;
_34.1.0.1 = _42.1.0.1;
(*_14) = _25 ^ _25;
_32 = (5259_u16,);
_34.1 = (_42.1.0, _4, Move(_42.1.2));
_34.0 = Adt21::Variant1 { fld0: (*_14),fld1: _4,fld2: _32,fld3: _38 };
Goto(bb12)
}
bb12 = {
_34.2 = &_42.1.0;
_34.0 = Adt21::Variant0 { fld0: _8 };
_42.1.0.0 = _34.1.0.0 - _37;
(*_14) = Field::<f64>(Variant(_42.0, 2), 3) as i64;
_25 = _11 as i64;
_7 = -_1;
_42.1.2 = &_34.1.0;
(*_14) = _7 as i64;
Goto(bb13)
}
bb13 = {
Call(_51 = dump_var(Move(_29), Move(_1), Move(_22), Move(_38)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_51 = dump_var(Move(_17), Move(_11), Move(_2), Move(_6)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_51 = dump_var(Move(_12), Move(_20), Move(_24), Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(Move(_27), _52, _52, _52), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: *mut i64,mut _3: *mut i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: isize,mut _8: isize,mut _9: i64,mut _10: i64) -> usize {
mir! {
type RET = usize;
let _11: [i64; 2];
let _12: [char; 1];
let _13: [char; 1];
let _14: *mut i64;
let _15: isize;
let _16: char;
let _17: &'static Adt26;
let _18: &'static mut *const *const char;
let _19: u64;
let _20: isize;
let _21: &'static mut (*mut u16,);
let _22: *const Adt21;
let _23: *mut &'static mut &'static mut u32;
let _24: bool;
let _25: isize;
let _26: u128;
let _27: Adt81;
let _28: i32;
let _29: isize;
let _30: bool;
let _31: &'static mut *const *const char;
let _32: u8;
let _33: f64;
let _34: usize;
let _35: u128;
let _36: f64;
let _37: bool;
let _38: f32;
let _39: (*mut u16, &'static (char, i64, Adt17));
let _40: &'static Adt26;
let _41: (u16, Adt21, *mut [i16; 1]);
let _42: (f32, f64, i64);
let _43: isize;
let _44: *mut i32;
let _45: u32;
let _46: &'static mut &'static mut u32;
let _47: char;
let _48: isize;
let _49: f32;
let _50: [i16; 1];
let _51: [char; 1];
let _52: u16;
let _53: i64;
let _54: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _55: f32;
let _56: i64;
let _57: &'static mut *const *const char;
let _58: f64;
let _59: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _60: i16;
let _61: Adt66;
let _62: isize;
let _63: isize;
let _64: isize;
let _65: f32;
let _66: *mut Adt52;
let _67: [i128; 8];
let _68: [u32; 3];
let _69: u8;
let _70: f64;
let _71: &'static mut *const *const char;
let _72: [u8; 8];
let _73: bool;
let _74: &'static (char, i64, Adt17);
let _75: bool;
let _76: char;
let _77: [u8; 7];
let _78: i16;
let _79: *mut [i16; 1];
let _80: &'static mut (*mut u16,);
let _81: bool;
let _82: *const i64;
let _83: u32;
let _84: i8;
let _85: &'static Adt26;
let _86: Adt52;
let _87: i16;
let _88: &'static mut &'static mut u32;
let _89: i16;
let _90: Adt26;
let _91: u128;
let _92: u64;
let _93: *const char;
let _94: Adt81;
let _95: &'static mut u32;
let _96: &'static Adt66;
let _97: ();
let _98: ();
{
_1 = _7 | _7;
_8 = 2000181907_u32 as isize;
_8 = _1 & _7;
_8 = _1;
_8 = _7;
_10 = (-165275751813636101563919421890489962512_i128) as i64;
_3 = core::ptr::addr_of_mut!(_5);
RET = 1880558984215236590_usize + 13984863235500550422_usize;
_11 = [_10,(*_3)];
(*_3) = !_9;
(*_3) = _6 ^ _9;
(*_3) = (-29608_i16) as i64;
_6 = 21617_i16 as i64;
(*_3) = !_9;
_12 = ['\u{10cc6e}'];
(*_3) = _9;
_12 = ['\u{8472e}'];
_2 = Move(_3);
_10 = _6 | _9;
_9 = _4 >> _8;
_4 = -_9;
_12 = ['\u{4c479}'];
_8 = !_1;
_2 = core::ptr::addr_of_mut!(_6);
(*_2) = 186_u8 as i64;
Call((*_2) = fn6(_1, _11, Move(_2), _1, _8, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _8;
_14 = core::ptr::addr_of_mut!(_5);
_12 = ['\u{82d27}'];
Call((*_14) = fn7(_9, _10, Move(_14)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = core::ptr::addr_of_mut!(_10);
(*_14) = _5;
_16 = '\u{c0735}';
(*_14) = _5 - _5;
(*_14) = 61642_u16 as i64;
_9 = -_5;
_15 = _7 - _7;
_16 = '\u{85aeb}';
RET = 17730217612097632021_usize + 59551034406743521_usize;
(*_14) = _9 ^ _6;
(*_14) = _16 as i64;
(*_14) = _6 + _9;
(*_14) = _5;
_4 = (*_14) & (*_14);
_19 = 13482148317782916342_u64;
(*_14) = !_9;
_15 = -_7;
_15 = _8;
(*_14) = _4 * _5;
(*_14) = _5 + _4;
match _19 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
13482148317782916342 => bb11,
_ => bb10
}
}
bb3 = {
_1 = _8;
_14 = core::ptr::addr_of_mut!(_5);
_12 = ['\u{82d27}'];
Call((*_14) = fn7(_9, _10, Move(_14)), ReturnTo(bb2), UnwindUnreachable())
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
_13 = _12;
_10 = _4 << _5;
(*_14) = !_6;
_4 = (*_14) ^ (*_14);
(*_14) = _9;
_13 = [_16];
(*_14) = _4;
_1 = _8 * _8;
(*_14) = !_5;
_19 = 132173722211596225_u64;
_2 = core::ptr::addr_of_mut!((*_14));
(*_2) = _4 ^ _9;
_25 = _1;
Goto(bb12)
}
bb12 = {
_6 = (*_2) | (*_14);
(*_2) = _6;
_26 = !64922589080886240775671843293818280858_u128;
(*_2) = _9 + _6;
_27.fld2.0 = 1690258075_u32 as f32;
_28 = 1779096499_i32 & (-1526646647_i32);
(*_2) = !_4;
(*_2) = -_4;
(*_14) = _6 ^ _4;
_25 = -_15;
_27.fld1.1.0 = _16;
_3 = Move(_14);
_29 = _25 + _25;
_27.fld1.0.0 = 51904_u16 << _4;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = _5;
(*_14) = _10;
(*_14) = _27.fld1.0.0 as i64;
_27.fld0 = (*_14) >= (*_14);
_27.fld4.1 = _26 ^ _26;
_20 = -_15;
Goto(bb13)
}
bb13 = {
_5 = _10 & (*_14);
_5 = 104_u8 as i64;
(*_14) = _27.fld2.0 as i64;
_27.fld2.1 = _27.fld1.0.0 as f64;
(*_14) = _10 + _10;
(*_14) = _10 >> _6;
_27.fld2.1 = _27.fld4.1 as f64;
(*_14) = _10 * _6;
(*_14) = _6;
_27.fld1.0.0 = 45843_u16 - 64608_u16;
_27.fld1.1.2 = Adt17::Variant2 { fld0: _27.fld4.1,fld1: _27.fld1.1.0,fld2: 219_u8,fld3: (-111_i8),fld4: _27.fld2.0,fld5: (*_14) };
_4 = (*_14) ^ (*_14);
_27.fld4.1 = _26 * Field::<u128>(Variant(_27.fld1.1.2, 2), 0);
(*_14) = Field::<i64>(Variant(_27.fld1.1.2, 2), 5) - _10;
_27.fld2.1 = 1784367599_u32 as f64;
_30 = _27.fld0;
_35 = _27.fld4.1 * Field::<u128>(Variant(_27.fld1.1.2, 2), 0);
_1 = _29 & _15;
match _19 {
132173722211596225 => bb14,
_ => bb5
}
}
bb14 = {
place!(Field::<u8>(Variant(_27.fld1.1.2, 2), 2)) = 211_u8 ^ 104_u8;
_27.fld5 = Adt52::Variant0 { fld0: _29 };
(*_14) = _6;
_36 = _27.fld2.1;
_32 = !Field::<u8>(Variant(_27.fld1.1.2, 2), 2);
_24 = _27.fld0;
_11 = [(*_14),_4];
_28 = !1034357001_i32;
_1 = _29 & _29;
_5 = (*_14);
(*_14) = _5 >> _4;
_38 = Field::<f32>(Variant(_27.fld1.1.2, 2), 4) + _27.fld2.0;
_27.fld1.1.2 = Adt17::Variant0 { fld0: _36,fld1: _16,fld2: _1,fld3: _35,fld4: (-18426_i16),fld5: _38 };
_30 = _5 <= (*_14);
_27.fld1.1.1 = (*_14) + _6;
(*_14) = !_27.fld1.1.1;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb5,
5 => bb9,
132173722211596225 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
_27.fld2 = (_38, Field::<f64>(Variant(_27.fld1.1.2, 0), 0), (*_14));
_4 = (*_14) * _10;
_27.fld4.0 = core::ptr::addr_of_mut!(_39.0);
(*_14) = -_6;
_1 = Field::<isize>(Variant(_27.fld1.1.2, 0), 2) << _27.fld1.1.1;
_12 = [Field::<char>(Variant(_27.fld1.1.2, 0), 1)];
(*_14) = _27.fld1.1.1 * _10;
_3 = core::ptr::addr_of_mut!((*_14));
(*_14) = _10 | _6;
_10 = 2826190900_u32 as i64;
Goto(bb17)
}
bb17 = {
_28 = -(-709751225_i32);
_1 = (*_14) as isize;
_20 = 24_i8 as isize;
_20 = _1;
_38 = Field::<f32>(Variant(_27.fld1.1.2, 0), 5) + _27.fld2.0;
place!(Field::<i16>(Variant(_27.fld1.1.2, 0), 4)) = 16782_i16 | 6285_i16;
_3 = core::ptr::addr_of_mut!(_9);
_34 = RET;
_27.fld0 = _30;
(*_14) = -_6;
(*_14) = _28 as i64;
_13 = _12;
_3 = core::ptr::addr_of_mut!((*_14));
_41.0 = _20 as u16;
_28 = -1401607859_i32;
match _19 {
0 => bb18,
132173722211596225 => bb20,
_ => bb19
}
}
bb18 = {
_6 = (*_2) | (*_14);
(*_2) = _6;
_26 = !64922589080886240775671843293818280858_u128;
(*_2) = _9 + _6;
_27.fld2.0 = 1690258075_u32 as f32;
_28 = 1779096499_i32 & (-1526646647_i32);
(*_2) = !_4;
(*_2) = -_4;
(*_14) = _6 ^ _4;
_25 = -_15;
_27.fld1.1.0 = _16;
_3 = Move(_14);
_29 = _25 + _25;
_27.fld1.0.0 = 51904_u16 << _4;
_14 = core::ptr::addr_of_mut!(_9);
(*_14) = _5;
(*_14) = _10;
(*_14) = _27.fld1.0.0 as i64;
_27.fld0 = (*_14) >= (*_14);
_27.fld4.1 = _26 ^ _26;
_20 = -_15;
Goto(bb13)
}
bb19 = {
Return()
}
bb20 = {
_39.1 = &_27.fld1.1;
_25 = -_1;
(*_14) = -_27.fld1.1.1;
(*_14) = _27.fld1.1.1 | _5;
(*_14) = _27.fld1.1.1;
_19 = _27.fld0 as u64;
_27.fld4.1 = !Field::<u128>(Variant(_27.fld1.1.2, 0), 3);
_37 = !_24;
_45 = _36 as u32;
(*_14) = _27.fld1.1.1;
_20 = _25 - _25;
_11 = [(*_14),(*_14)];
_30 = _24;
_42.0 = -_38;
RET = !_34;
_43 = _1;
(*_14) = (-79_i8) as i64;
_41.1 = Adt21::Variant0 { fld0: Field::<isize>(Variant(_27.fld5, 0), 0) };
place!(Field::<isize>(Variant(_27.fld5, 0), 0)) = _20;
(*_14) = _34 as i64;
(*_14) = _27.fld2.2 ^ _5;
Goto(bb21)
}
bb21 = {
_27.fld2 = (_38, _36, (*_14));
_42.1 = _19 as f64;
place!(Field::<u128>(Variant(_27.fld1.1.2, 0), 3)) = !_26;
_42.1 = -Field::<f64>(Variant(_27.fld1.1.2, 0), 0);
_43 = Field::<isize>(Variant(_27.fld5, 0), 0) << (*_14);
_16 = _27.fld1.1.0;
_27.fld1.0 = (_41.0,);
_30 = !_24;
(*_14) = !_4;
_20 = -Field::<isize>(Variant(_41.1, 0), 0);
_14 = core::ptr::addr_of_mut!((*_14));
_42 = (_38, Field::<f64>(Variant(_27.fld1.1.2, 0), 0), (*_14));
_27.fld1.0.0 = _41.0 ^ _41.0;
(*_14) = _27.fld2.2 | _5;
_27.fld2.0 = _38 + _42.0;
(*_14) = _27.fld1.1.1;
Goto(bb22)
}
bb22 = {
_27.fld2.0 = -_42.0;
(*_14) = _27.fld2.2 & _42.2;
place!(Field::<isize>(Variant(_27.fld5, 0), 0)) = _43 + _43;
(*_14) = _27.fld1.1.1;
_15 = Field::<isize>(Variant(_27.fld1.1.2, 0), 2) & _8;
(*_14) = _42.2 * _5;
Goto(bb23)
}
bb23 = {
_36 = _27.fld2.1 + _27.fld2.1;
_54.0 = Adt21::Variant0 { fld0: Field::<isize>(Variant(_27.fld5, 0), 0) };
(*_14) = _42.2 & _27.fld1.1.1;
_27.fld0 = _30;
_35 = !_27.fld4.1;
place!(Field::<i16>(Variant(_27.fld1.1.2, 0), 4)) = _38 as i16;
_27.fld1.1.1 = (*_14);
(*_14) = _6 & _6;
_42.2 = (*_14) & (*_14);
place!(Field::<f32>(Variant(_27.fld1.1.2, 0), 5)) = -_38;
_22 = core::ptr::addr_of!(_41.1);
(*_14) = 3356711853277162363955761824335253963_i128 as i64;
_47 = _27.fld1.1.0;
_41.2 = core::ptr::addr_of_mut!(_50);
_54.1.0.1 = _19 as i8;
_51 = _12;
place!(Field::<isize>(Variant((*_22), 0), 0)) = Field::<isize>(Variant(_27.fld5, 0), 0) & _1;
place!(Field::<f32>(Variant(_27.fld1.1.2, 0), 5)) = -_42.0;
_12 = [_16];
Goto(bb24)
}
bb24 = {
place!(Field::<isize>(Variant((*_22), 0), 0)) = !_1;
place!(Field::<isize>(Variant((*_22), 0), 0)) = _19 as isize;
place!(Field::<isize>(Variant((*_22), 0), 0)) = _8 * Field::<isize>(Variant(_54.0, 0), 0);
_41.2 = core::ptr::addr_of_mut!(_50);
_55 = _42.0 + _38;
_26 = _27.fld4.1;
place!(Field::<isize>(Variant((*_22), 0), 0)) = !Field::<isize>(Variant(_27.fld1.1.2, 0), 2);
Goto(bb25)
}
bb25 = {
_10 = _4;
_36 = _27.fld2.1 + _42.1;
_59.1 = (*_22);
Goto(bb26)
}
bb26 = {
(*_14) = _10 << Field::<isize>(Variant((*_22), 0), 0);
place!(Field::<isize>(Variant((*_22), 0), 0)) = !Field::<isize>(Variant(_27.fld1.1.2, 0), 2);
_54.0 = Adt21::Variant0 { fld0: _43 };
(*_14) = _55 as i64;
place!(Field::<isize>(Variant((*_22), 0), 0)) = !Field::<isize>(Variant(_27.fld5, 0), 0);
(*_22) = Adt21::Variant0 { fld0: _43 };
Goto(bb27)
}
bb27 = {
_37 = _24;
_59.2.2 = &mut _45;
_35 = _27.fld4.1 & _26;
(*_14) = _42.2 + _27.fld2.2;
_15 = !Field::<isize>(Variant((*_22), 0), 0);
_30 = !_27.fld0;
_56 = (*_14) | (*_14);
_54.1.0.2 = !63939657967844460000314263737383752578_i128;
_7 = -Field::<isize>(Variant((*_22), 0), 0);
_4 = _5 + (*_14);
(*_14) = _54.1.0.2 as i64;
place!(Field::<isize>(Variant((*_22), 0), 0)) = _43 - Field::<isize>(Variant(_27.fld5, 0), 0);
(*_14) = _27.fld1.1.1 >> Field::<isize>(Variant((*_22), 0), 0);
place!(Field::<isize>(Variant((*_22), 0), 0)) = Field::<isize>(Variant(_54.0, 0), 0) | _1;
place!(Field::<u128>(Variant(_27.fld1.1.2, 0), 3)) = Field::<i16>(Variant(_27.fld1.1.2, 0), 4) as u128;
place!(Field::<isize>(Variant((*_22), 0), 0)) = _29 ^ _43;
_33 = _27.fld2.1 * _42.1;
_53 = (*_14) + (*_14);
_59.2.1 = Move(_22);
_64 = Field::<isize>(Variant(_41.1, 0), 0) | _15;
_58 = _36 * _27.fld2.1;
_59.2.1 = core::ptr::addr_of!(_54.0);
_53 = Field::<i16>(Variant(_27.fld1.1.2, 0), 4) as i64;
Goto(bb28)
}
bb28 = {
_41.0 = _27.fld1.0.0 >> Field::<isize>(Variant(_41.1, 0), 0);
_27.fld2.1 = -_33;
_15 = _43;
_29 = !_7;
_19 = 6238579088631931942_u64;
place!(Field::<u128>(Variant(_27.fld1.1.2, 0), 3)) = !_26;
_1 = _64 ^ Field::<isize>(Variant(_41.1, 0), 0);
(*_14) = _34 as i64;
_22 = Move(_59.2.1);
_3 = Move(_2);
_2 = Move(_3);
_50 = [Field::<i16>(Variant(_27.fld1.1.2, 0), 4)];
_44 = core::ptr::addr_of_mut!(_59.2.0);
_27.fld2 = (_55, _42.1, _42.2);
(*_44) = _28;
_7 = -Field::<isize>(Variant(_41.1, 0), 0);
(*_14) = _42.2 << _25;
_52 = _41.0 | _27.fld1.0.0;
_27.fld1.0.0 = _36 as u16;
_39.0 = core::ptr::addr_of_mut!(_27.fld1.0.0);
Goto(bb29)
}
bb29 = {
_13 = _51;
_27.fld1.0 = (_52,);
_27.fld2.1 = -_33;
_63 = _27.fld1.0.0 as isize;
_54.1.0 = (_27.fld2.0, 23_i8, (-136495123246851842469914816176720861694_i128));
_24 = _30 == _37;
_37 = _30;
(*_44) = _28 | _28;
_15 = _32 as isize;
(*_14) = _54.1.0.2 as i64;
(*_14) = _10 - _6;
Goto(bb30)
}
bb30 = {
(*_44) = _28 - _28;
_13 = _12;
_54.1.0 = (_55, 17_i8, 141288927444355215101789340157968704300_i128);
place!(Field::<char>(Variant(_27.fld1.1.2, 0), 1)) = _16;
_24 = _30;
(*_44) = _28 * _28;
(*_44) = _28 * _28;
_7 = _63 + Field::<isize>(Variant(_54.0, 0), 0);
place!(Field::<char>(Variant(_27.fld1.1.2, 0), 1)) = _47;
place!(Field::<i16>(Variant(_27.fld1.1.2, 0), 4)) = (-7684_i16) + (-9204_i16);
_27.fld2.0 = _32 as f32;
place!(Field::<i16>(Variant(_27.fld1.1.2, 0), 4)) = 3158_i16;
place!(Field::<char>(Variant(_27.fld1.1.2, 0), 1)) = _27.fld1.1.0;
(*_14) = _5;
_38 = -_55;
_10 = !_4;
_44 = core::ptr::addr_of_mut!((*_44));
Call(_59.2.1 = core::intrinsics::arith_offset(_22, (-9223372036854775808_isize)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_49 = Field::<f32>(Variant(_27.fld1.1.2, 0), 5) * _55;
(*_44) = _64 as i32;
_34 = RET | RET;
_1 = _47 as isize;
_55 = _49 * _42.0;
_19 = _54.1.0.1 as u64;
place!(Field::<isize>(Variant(_41.1, 0), 0)) = _58 as isize;
_52 = _27.fld1.0.0 + _27.fld1.0.0;
_65 = _55;
Goto(bb32)
}
bb32 = {
(*_44) = _28 * _28;
_6 = (*_14) ^ _4;
place!(Field::<isize>(Variant(_59.1, 0), 0)) = Field::<isize>(Variant(_27.fld1.1.2, 0), 2) * Field::<isize>(Variant(_54.0, 0), 0);
_27.fld1.0.0 = _54.1.0.2 as u16;
(*_14) = _6 + _6;
_42 = (_38, _27.fld2.1, _4);
Goto(bb33)
}
bb33 = {
_42.1 = Field::<i16>(Variant(_27.fld1.1.2, 0), 4) as f64;
_41.1 = Adt21::Variant1 { fld0: (*_14),fld1: _34,fld2: _27.fld1.0,fld3: _32 };
_74 = &_27.fld1.1;
_14 = core::ptr::addr_of_mut!((*_14));
place!(Field::<(u16,)>(Variant(_41.1, 1), 2)).0 = !_52;
place!(Field::<f64>(Variant(_27.fld1.1.2, 0), 0)) = _36;
(*_44) = _28;
(*_14) = -(*_74).1;
(*_14) = (*_74).1 + (*_74).1;
_22 = core::ptr::addr_of!(_54.0);
(*_14) = _56;
_54.2 = &_54.1.0;
_66 = core::ptr::addr_of_mut!(_27.fld5);
_67 = [_54.1.0.2,_54.1.0.2,_54.1.0.2,_54.1.0.2,_54.1.0.2,_54.1.0.2,_54.1.0.2,_54.1.0.2];
place!(Field::<isize>(Variant((*_22), 0), 0)) = Field::<char>(Variant((*_74).2, 0), 1) as isize;
(*_22) = Adt21::Variant0 { fld0: _7 };
_51 = _13;
(*_14) = (*_74).1 >> (*_74).1;
(*_14) = (*_74).1 + _27.fld1.1.1;
_27.fld0 = !_37;
place!(Field::<isize>(Variant((*_66), 0), 0)) = Field::<isize>(Variant((*_22), 0), 0) ^ Field::<isize>(Variant((*_22), 0), 0);
match _54.1.0.1 {
17 => bb35,
_ => bb34
}
}
bb34 = {
_14 = core::ptr::addr_of_mut!(_10);
(*_14) = _5;
_16 = '\u{c0735}';
(*_14) = _5 - _5;
(*_14) = 61642_u16 as i64;
_9 = -_5;
_15 = _7 - _7;
_16 = '\u{85aeb}';
RET = 17730217612097632021_usize + 59551034406743521_usize;
(*_14) = _9 ^ _6;
(*_14) = _16 as i64;
(*_14) = _6 + _9;
(*_14) = _5;
_4 = (*_14) & (*_14);
_19 = 13482148317782916342_u64;
(*_14) = !_9;
_15 = -_7;
_15 = _8;
(*_14) = _4 * _5;
(*_14) = _5 + _4;
match _19 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
13482148317782916342 => bb11,
_ => bb10
}
}
bb35 = {
place!(Field::<isize>(Variant((*_22), 0), 0)) = _7 | Field::<isize>(Variant((*_66), 0), 0);
_66 = core::ptr::addr_of_mut!((*_66));
(*_66) = Adt52::Variant2 { fld0: Move(_27.fld4.0),fld1: _13 };
(*_44) = _28 | _28;
(*_22) = Adt21::Variant2 { fld0: 2793781814_u32,fld1: (*_74).0,fld2: _55,fld3: _27.fld2.1,fld4: RET };
_2 = Move(_14);
place!(Field::<u32>(Variant((*_22), 2), 0)) = 633344057_u32 - 1616065050_u32;
(*_44) = _27.fld2.1 as i32;
(*_22) = Adt21::Variant1 { fld0: _6,fld1: _34,fld2: Field::<(u16,)>(Variant(_41.1, 1), 2),fld3: Field::<u8>(Variant(_41.1, 1), 3) };
(*_44) = _28 * _28;
_22 = core::ptr::addr_of!((*_22));
place!(Field::<*mut *mut u16>(Variant((*_66), 2), 0)) = core::ptr::addr_of_mut!(_39.0);
place!(Field::<(u16,)>(Variant((*_22), 1), 2)) = (_41.0,);
place!(Field::<u8>(Variant((*_22), 1), 3)) = _36 as u8;
_4 = Field::<i64>(Variant((*_22), 1), 0);
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = !Field::<(u16,)>(Variant(_41.1, 1), 2).0;
_12 = [(*_74).0];
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = !_41.0;
(*_44) = -_28;
place!(Field::<usize>(Variant((*_22), 1), 1)) = Field::<usize>(Variant(_41.1, 1), 1) + _34;
_6 = !(*_74).1;
place!(Field::<u8>(Variant(_54.0, 1), 3)) = _32 + Field::<u8>(Variant(_41.1, 1), 3);
(*_22) = Adt21::Variant1 { fld0: _42.2,fld1: _34,fld2: Field::<(u16,)>(Variant(_41.1, 1), 2),fld3: Field::<u8>(Variant(_41.1, 1), 3) };
place!(Field::<(u16,)>(Variant((*_22), 1), 2)) = (Field::<(u16,)>(Variant(_41.1, 1), 2).0,);
place!(Field::<*mut *mut u16>(Variant((*_66), 2), 0)) = core::ptr::addr_of_mut!(_39.0);
match Field::<i16>(Variant((*_74).2, 0), 4) {
0 => bb31,
1 => bb24,
2 => bb17,
3 => bb7,
4 => bb8,
5 => bb6,
6 => bb36,
3158 => bb38,
_ => bb37
}
}
bb36 = {
(*_44) = _28 * _28;
_6 = (*_14) ^ _4;
place!(Field::<isize>(Variant(_59.1, 0), 0)) = Field::<isize>(Variant(_27.fld1.1.2, 0), 2) * Field::<isize>(Variant(_54.0, 0), 0);
_27.fld1.0.0 = _54.1.0.2 as u16;
(*_14) = _6 + _6;
_42 = (_38, _27.fld2.1, _4);
Goto(bb33)
}
bb37 = {
Return()
}
bb38 = {
_81 = Field::<isize>(Variant((*_74).2, 0), 2) == Field::<isize>(Variant((*_74).2, 0), 2);
place!(Field::<i64>(Variant((*_22), 1), 0)) = !(*_74).1;
_78 = 2912987319_u32 as i16;
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = _41.0 ^ _41.0;
place!(Field::<i64>(Variant((*_22), 1), 0)) = !(*_74).1;
place!(Field::<usize>(Variant((*_22), 1), 1)) = _34;
_79 = core::ptr::addr_of_mut!(_50);
place!(Field::<(u16,)>(Variant((*_22), 1), 2)) = Field::<(u16,)>(Variant(_41.1, 1), 2);
place!(Field::<usize>(Variant((*_22), 1), 1)) = !Field::<usize>(Variant(_41.1, 1), 1);
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = Field::<(u16,)>(Variant(_41.1, 1), 2).0 << Field::<isize>(Variant((*_74).2, 0), 2);
_15 = _25 & Field::<isize>(Variant((*_74).2, 0), 2);
place!(Field::<*mut *mut u16>(Variant((*_66), 2), 0)) = core::ptr::addr_of_mut!(_39.0);
match _54.1.0.1 {
17 => bb39,
_ => bb3
}
}
bb39 = {
place!(Field::<usize>(Variant((*_22), 1), 1)) = Field::<usize>(Variant(_41.1, 1), 1) & Field::<usize>(Variant(_41.1, 1), 1);
_27.fld1.0.0 = !Field::<(u16,)>(Variant((*_22), 1), 2).0;
_63 = _29 << (*_74).1;
_54.1.0.2 = 89388374984418492506414399613384577496_i128;
_60 = -Field::<i16>(Variant((*_74).2, 0), 4);
place!(Field::<[char; 1]>(Variant((*_66), 2), 1)) = [(*_74).0];
place!(Field::<[char; 1]>(Variant((*_66), 2), 1)) = [Field::<char>(Variant((*_74).2, 0), 1)];
(*_79) = [Field::<i16>(Variant((*_74).2, 0), 4)];
place!(Field::<[char; 1]>(Variant((*_66), 2), 1)) = [(*_74).0];
Goto(bb40)
}
bb40 = {
place!(Field::<*mut *mut u16>(Variant((*_66), 2), 0)) = core::ptr::addr_of_mut!(_39.0);
_14 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant((*_22), 1), 0)));
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = _41.0 << Field::<i64>(Variant(_41.1, 1), 0);
_76 = Field::<char>(Variant((*_74).2, 0), 1);
place!(Field::<i64>(Variant((*_22), 1), 0)) = (*_74).1;
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = !_27.fld1.0.0;
place!(Field::<[char; 1]>(Variant((*_66), 2), 1)) = [(*_74).0];
_59.2.3 = 2657541217_u32 * 392612099_u32;
place!(Field::<(u16,)>(Variant((*_22), 1), 2)).0 = _41.0 >> Field::<i64>(Variant((*_22), 1), 0);
place!(Field::<u8>(Variant((*_22), 1), 3)) = Field::<u8>(Variant(_41.1, 1), 3);
(*_66) = Adt52::Variant0 { fld0: Field::<isize>(Variant((*_74).2, 0), 2) };
_41.1 = Adt21::Variant1 { fld0: (*_74).1,fld1: Field::<usize>(Variant((*_22), 1), 1),fld2: Field::<(u16,)>(Variant((*_22), 1), 2),fld3: Field::<u8>(Variant((*_22), 1), 3) };
place!(Field::<(u16,)>(Variant(_41.1, 1), 2)).0 = !Field::<(u16,)>(Variant((*_22), 1), 2).0;
(*_22) = Adt21::Variant0 { fld0: Field::<isize>(Variant((*_66), 0), 0) };
_27.fld1.1.1 = _6;
_54.1.1 = RET;
_54.1.1 = !_34;
_89 = -Field::<i16>(Variant((*_74).2, 0), 4);
_32 = Field::<u8>(Variant(_41.1, 1), 3) * Field::<u8>(Variant(_41.1, 1), 3);
(*_79) = [Field::<i16>(Variant((*_74).2, 0), 4)];
place!(Field::<isize>(Variant((*_66), 0), 0)) = -Field::<isize>(Variant((*_22), 0), 0);
_54.1.2 = &_54.1.0;
_74 = Move(_39.1);
place!(Field::<(u16,)>(Variant(_41.1, 1), 2)) = (_52,);
(*_79) = [_89];
(*_79) = [_78];
match _54.1.0.1 {
0 => bb23,
1 => bb15,
2 => bb11,
3 => bb17,
4 => bb37,
5 => bb19,
6 => bb41,
17 => bb43,
_ => bb42
}
}
bb41 = {
Return()
}
bb42 = {
Return()
}
bb43 = {
_27.fld0 = _24;
(*_79) = [_78];
_11 = [_6,_4];
Goto(bb44)
}
bb44 = {
(*_79) = [_89];
place!(Field::<isize>(Variant((*_66), 0), 0)) = _15 >> _63;
place!(Field::<isize>(Variant(_54.0, 0), 0)) = _33 as isize;
_54.2 = Move(_54.1.2);
_90.fld4 = _26 << Field::<isize>(Variant((*_66), 0), 0);
(*_66) = Adt52::Variant0 { fld0: _64 };
_48 = Field::<isize>(Variant((*_66), 0), 0) >> Field::<i64>(Variant(_41.1, 1), 0);
(*_79) = [Field::<i16>(Variant(_27.fld1.1.2, 0), 4)];
_3 = core::ptr::addr_of_mut!(_9);
Call((*_44) = core::intrinsics::transmute(Field::<char>(Variant(_27.fld1.1.2, 0), 1)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
(*_79) = [Field::<i16>(Variant(_27.fld1.1.2, 0), 4)];
_75 = !_27.fld0;
place!(Field::<isize>(Variant((*_66), 0), 0)) = _27.fld2.1 as isize;
_94.fld1.1 = (_76, (*_3), Move(_27.fld1.1.2));
_9 = _10 & _5;
(*_79) = [_89];
_94.fld1.1.0 = _47;
(*_44) = -_28;
(*_44) = _28 | _28;
match _54.1.0.1 {
0 => bb7,
1 => bb46,
2 => bb47,
3 => bb48,
17 => bb50,
_ => bb49
}
}
bb46 = {
(*_79) = [_89];
place!(Field::<isize>(Variant((*_66), 0), 0)) = _15 >> _63;
place!(Field::<isize>(Variant(_54.0, 0), 0)) = _33 as isize;
_54.2 = Move(_54.1.2);
_90.fld4 = _26 << Field::<isize>(Variant((*_66), 0), 0);
(*_66) = Adt52::Variant0 { fld0: _64 };
_48 = Field::<isize>(Variant((*_66), 0), 0) >> Field::<i64>(Variant(_41.1, 1), 0);
(*_79) = [Field::<i16>(Variant(_27.fld1.1.2, 0), 4)];
_3 = core::ptr::addr_of_mut!(_9);
Call((*_44) = core::intrinsics::transmute(Field::<char>(Variant(_27.fld1.1.2, 0), 1)), ReturnTo(bb45), UnwindUnreachable())
}
bb47 = {
Return()
}
bb48 = {
place!(Field::<u8>(Variant(_27.fld1.1.2, 2), 2)) = 211_u8 ^ 104_u8;
_27.fld5 = Adt52::Variant0 { fld0: _29 };
(*_14) = _6;
_36 = _27.fld2.1;
_32 = !Field::<u8>(Variant(_27.fld1.1.2, 2), 2);
_24 = _27.fld0;
_11 = [(*_14),_4];
_28 = !1034357001_i32;
_1 = _29 & _29;
_5 = (*_14);
(*_14) = _5 >> _4;
_38 = Field::<f32>(Variant(_27.fld1.1.2, 2), 4) + _27.fld2.0;
_27.fld1.1.2 = Adt17::Variant0 { fld0: _36,fld1: _16,fld2: _1,fld3: _35,fld4: (-18426_i16),fld5: _38 };
_30 = _5 <= (*_14);
_27.fld1.1.1 = (*_14) + _6;
(*_14) = !_27.fld1.1.1;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb5,
5 => bb9,
132173722211596225 => bb16,
_ => bb15
}
}
bb49 = {
_27.fld2 = (_38, Field::<f64>(Variant(_27.fld1.1.2, 0), 0), (*_14));
_4 = (*_14) * _10;
_27.fld4.0 = core::ptr::addr_of_mut!(_39.0);
(*_14) = -_6;
_1 = Field::<isize>(Variant(_27.fld1.1.2, 0), 2) << _27.fld1.1.1;
_12 = [Field::<char>(Variant(_27.fld1.1.2, 0), 1)];
(*_14) = _27.fld1.1.1 * _10;
_3 = core::ptr::addr_of_mut!((*_14));
(*_14) = _10 | _6;
_10 = 2826190900_u32 as i64;
Goto(bb17)
}
bb50 = {
(*_22) = Adt21::Variant2 { fld0: _59.2.3,fld1: _47,fld2: _42.0,fld3: _36,fld4: Field::<usize>(Variant(_41.1, 1), 1) };
(*_22) = Adt21::Variant2 { fld0: _59.2.3,fld1: _27.fld1.1.0,fld2: _65,fld3: _27.fld2.1,fld4: Field::<usize>(Variant(_41.1, 1), 1) };
place!(Field::<usize>(Variant((*_22), 2), 4)) = RET + _34;
_9 = _27.fld1.1.1;
_59.2.2 = &mut place!(Field::<u32>(Variant((*_22), 2), 0));
place!(Field::<f64>(Variant(_94.fld1.1.2, 0), 0)) = Field::<f64>(Variant((*_22), 2), 3);
_27.fld2.2 = (*_3) * (*_3);
_27.fld1.1.2 = Move(_94.fld1.1.2);
Goto(bb51)
}
bb51 = {
Call(_97 = dump_var(Move(_16), Move(_75), Move(_63), Move(_13)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_97 = dump_var(Move(_29), Move(_50), Move(_11), Move(_60)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_97 = dump_var(Move(_19), Move(_10), Move(_9), Move(_25)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_97 = dump_var(Move(_48), Move(_1), Move(_24), Move(_4)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_97 = dump_var(Move(_76), Move(_5), Move(_32), Move(_12)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_97 = dump_var(Move(_81), _98, _98, _98), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: [i64; 2],mut _3: *mut i64,mut _4: isize,mut _5: isize,mut _6: i64) -> i64 {
mir! {
type RET = i64;
let _7: [u16; 5];
let _8: ();
let _9: ();
{
_6 = !(-6676559960011905685_i64);
RET = _6 - _6;
_6 = RET >> _5;
Goto(bb1)
}
bb1 = {
_3 = core::ptr::addr_of_mut!(RET);
(*_3) = _6 + _6;
(*_3) = _6 | _6;
(*_3) = 64_u8 as i64;
(*_3) = _6;
(*_3) = 2334656993_u32 as i64;
(*_3) = _6 * _6;
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(Move(_6), Move(_5), _9, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i64,mut _2: i64,mut _3: *mut i64) -> i64 {
mir! {
type RET = i64;
let _4: *mut &'static mut &'static mut u32;
let _5: i32;
let _6: f64;
let _7: isize;
let _8: f32;
let _9: char;
let _10: u64;
let _11: isize;
let _12: ();
let _13: ();
{
RET = !_1;
_3 = core::ptr::addr_of_mut!(_1);
(*_3) = _2;
(*_3) = !RET;
(*_3) = 74_i8 as i64;
(*_3) = -RET;
_1 = _2 ^ _2;
(*_3) = RET;
(*_3) = 3898_i16 as i64;
(*_3) = 3423613166_u32 as i64;
(*_3) = RET + RET;
(*_3) = RET >> _2;
_1 = -RET;
(*_3) = (-9223372036854775808_isize) as i64;
(*_3) = RET;
_3 = core::ptr::addr_of_mut!((*_3));
Call((*_3) = core::intrinsics::bswap(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_3) = RET | _2;
(*_3) = -RET;
(*_3) = 13766641668966587339451178250260649338_i128 as i64;
(*_3) = _2 - RET;
(*_3) = RET;
(*_3) = (-85_i8) as i64;
(*_3) = -RET;
_5 = 221911896_i32 & (-2010726636_i32);
(*_3) = -_2;
(*_3) = 15991486890350847682_u64 as i64;
(*_3) = 57459_u16 as i64;
Goto(bb2)
}
bb2 = {
(*_3) = RET;
(*_3) = RET >> _2;
(*_3) = RET - RET;
(*_3) = !RET;
_1 = 7_usize as i64;
(*_3) = RET;
(*_3) = 19619622325018750435346488321106812460_i128 as i64;
(*_3) = RET * RET;
(*_3) = 10_u8 as i64;
(*_3) = RET;
(*_3) = 339629087528557665154013392191567427198_u128 as i64;
(*_3) = -RET;
(*_3) = RET - RET;
Goto(bb3)
}
bb3 = {
(*_3) = RET | _2;
Goto(bb4)
}
bb4 = {
(*_3) = RET & _2;
(*_3) = RET | RET;
(*_3) = RET << _2;
(*_3) = RET << _2;
(*_3) = RET + RET;
_5 = 52011262_i32 | (-1078702872_i32);
(*_3) = -_2;
(*_3) = 7123277258691337796_u64 as i64;
(*_3) = RET;
(*_3) = -RET;
(*_3) = RET;
_6 = _5 as f64;
(*_3) = RET;
(*_3) = _2 + RET;
(*_3) = _2 << _5;
(*_3) = _6 as i64;
Goto(bb5)
}
bb5 = {
(*_3) = RET >> RET;
Call((*_3) = fn8(Move(_3), _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = !_2;
_3 = core::ptr::addr_of_mut!(_1);
(*_3) = RET * _2;
(*_3) = RET;
(*_3) = -RET;
(*_3) = -RET;
_6 = 2251979709_u32 as f64;
(*_3) = RET - RET;
(*_3) = RET * RET;
(*_3) = RET & RET;
_2 = (*_3) ^ (*_3);
(*_3) = _2 + _2;
(*_3) = -_2;
(*_3) = _2;
(*_3) = RET ^ RET;
(*_3) = _2;
(*_3) = _2 >> RET;
_5 = _6 as i32;
(*_3) = _2;
(*_3) = _2;
(*_3) = _2 - RET;
(*_3) = 40427_u16 as i64;
Goto(bb7)
}
bb7 = {
_5 = (-765830663_i32) << _2;
(*_3) = !_2;
(*_3) = 55_i8 as i64;
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb8)
}
bb8 = {
(*_3) = _2 + RET;
_9 = '\u{87b83}';
Call((*_3) = core::intrinsics::bswap(RET), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_3) = 55779583315931770962123219460244271566_u128 as i64;
(*_3) = 292492947691788023740948792079440606602_u128 as i64;
_2 = (*_3) << _5;
RET = -_2;
(*_3) = 62941_u16 as i64;
(*_3) = -RET;
_8 = 9_i8 as f32;
(*_3) = -_2;
_10 = 135329539956765233309835290497906688666_i128 as u64;
(*_3) = RET;
(*_3) = -_2;
(*_3) = !RET;
_10 = 151649599212652872711734473088938468690_u128 as u64;
_1 = RET >> _2;
_9 = '\u{93334}';
(*_3) = RET ^ RET;
(*_3) = _2 ^ _2;
(*_3) = RET;
_7 = (-32_isize) | (-9223372036854775808_isize);
_3 = core::ptr::addr_of_mut!((*_3));
_8 = 85268425082948309919847547133437240439_u128 as f32;
(*_3) = _2 * RET;
_9 = '\u{6f5be}';
(*_3) = RET;
Goto(bb10)
}
bb10 = {
Call(_12 = dump_var(Move(_2), Move(_5), Move(_10), _13), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *mut i64,mut _2: f64) -> i64 {
mir! {
type RET = i64;
let _3: [u32; 4];
let _4: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _5: *const *const char;
let _6: f64;
let _7: u8;
let _8: i32;
let _9: *mut u16;
let _10: u32;
let _11: u16;
let _12: i64;
let _13: &'static Adt66;
let _14: [i16; 1];
let _15: &'static (f32, i8, i128);
let _16: &'static (f32, i8, i128);
let _17: isize;
let _18: Adt26;
let _19: &'static (f32, i8, i128);
let _20: *mut u16;
let _21: bool;
let _22: u64;
let _23: usize;
let _24: *const *const char;
let _25: *const char;
let _26: f64;
let _27: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _28: i8;
let _29: [i16; 1];
let _30: &'static mut *const *const char;
let _31: u128;
let _32: &'static char;
let _33: ();
let _34: ();
{
_4.1 = ['\u{7b3ff}'];
Call(_4.2.0 = fn9(Move(_1), _2, _4.1, _4.1, _2, _4.1, _2, _2, _4.1, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [3190953012_u32,1564588876_u32,2948244649_u32,3783293419_u32];
RET = 9122819073626746398_i64;
_2 = 444541429_u32 as f64;
_4.2.3 = 4119027572_u32;
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 894867647598339665_i64 | (-4207737756344369692_i64);
(*_1) = 6151540232862038057_i64 | (-2706107598023574111_i64);
(*_1) = 1145798298529738478_i64 * 5201274932782560618_i64;
(*_1) = 4797044314369479401_i64 >> _4.2.0;
(*_1) = 636413839778360113_i64 | (-2462181210541403459_i64);
(*_1) = 14_u8 as i64;
_4.2.2 = &mut _4.2.3;
(*_1) = 9056113559751304999_i64;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = 1649685118_i32 as f64;
(*_1) = (-8120124698631088774_i64);
Goto(bb2)
}
bb2 = {
(*_1) = -(-8206826957744907634_i64);
(*_1) = (-4953380073618767540_i64) + (-8047378251649912322_i64);
(*_1) = (-5328347733908903165_i64) << 1932_i16;
(*_1) = 7987669629320218785_i64 | (-168412819510850446_i64);
(*_1) = 7419659574282901480_i64;
(*_1) = -601876320748281621_i64;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 7253785566592750451_i64;
(*_1) = 3426765023437415541_i64 << 7_usize;
_7 = 125_u8 << (*_1);
(*_1) = (-8164781089156207058_i64);
_6 = -_2;
(*_1) = (-4309704754997178097_i64) + (-9058727092526303435_i64);
_6 = _2 * _2;
_2 = _6 + _6;
_3 = [1504373543_u32,1722481201_u32,122877781_u32,3152586194_u32];
_3 = [1804047425_u32,141838783_u32,933943274_u32,1790303168_u32];
(*_1) = (-2226579725838246416_i64);
(*_1) = (-7181087714842491474_i64);
match (*_1) {
340282366920938463456193519716925719982 => bb4,
_ => bb3
}
}
bb3 = {
_3 = [3190953012_u32,1564588876_u32,2948244649_u32,3783293419_u32];
RET = 9122819073626746398_i64;
_2 = 444541429_u32 as f64;
_4.2.3 = 4119027572_u32;
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 894867647598339665_i64 | (-4207737756344369692_i64);
(*_1) = 6151540232862038057_i64 | (-2706107598023574111_i64);
(*_1) = 1145798298529738478_i64 * 5201274932782560618_i64;
(*_1) = 4797044314369479401_i64 >> _4.2.0;
(*_1) = 636413839778360113_i64 | (-2462181210541403459_i64);
(*_1) = 14_u8 as i64;
_4.2.2 = &mut _4.2.3;
(*_1) = 9056113559751304999_i64;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = 1649685118_i32 as f64;
(*_1) = (-8120124698631088774_i64);
Goto(bb2)
}
bb4 = {
(*_1) = 4163613605922110958_i64 - 6901914761369284117_i64;
(*_1) = (-2207468693013184313_i64);
_10 = 3822605451_u32 + 2558856000_u32;
(*_1) = (-7902469095439858554_i64);
_8 = (-1559313580_i32) >> (*_1);
(*_1) = !7514891256354783481_i64;
(*_1) = 28395_i16 as i64;
_8 = (-300540533_i32) * 1720708528_i32;
(*_1) = 6176545250454716507_i64 & 5882750369230102867_i64;
_3 = [_10,_10,_10,_10];
(*_1) = (-3969576216038621138_i64) & (-3549539779777644470_i64);
Goto(bb5)
}
bb5 = {
(*_1) = (-8537508582074533633_i64);
_12 = (*_1) | (*_1);
_3 = [_10,_10,_10,_10];
_7 = 236_u8 & 58_u8;
(*_1) = _12 >> _7;
(*_1) = _12 ^ _12;
(*_1) = _12 + _12;
(*_1) = 3903717519445885032_u64 as i64;
_2 = _6;
_11 = !8704_u16;
(*_1) = _12 ^ _12;
(*_1) = _12 << _12;
(*_1) = _12;
(*_1) = _12 & _12;
(*_1) = !_12;
(*_1) = _10 as i64;
(*_1) = _12 & _12;
_8 = 1884932981_i32 + (-1118439838_i32);
(*_1) = _12;
(*_1) = !_12;
_12 = (*_1);
(*_1) = _12;
_14 = [21617_i16];
(*_1) = _12;
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb6)
}
bb6 = {
(*_1) = _12;
(*_1) = -_12;
(*_1) = (-85593315658508696594860847315552447214_i128) as i64;
_14 = [(-32165_i16)];
_9 = core::ptr::addr_of_mut!(_11);
(*_1) = _12 + _12;
(*_1) = !_12;
_8 = 1628289576_i32 ^ 1338562763_i32;
(*_9) = 18902_u16;
(*_1) = _12;
(*_9) = 32438_u16;
_6 = _2;
(*_9) = false as u16;
(*_1) = _12;
(*_1) = _12;
(*_9) = 5350813139493943727_u64 as u16;
_14 = [6771_i16];
(*_9) = 13663_u16 >> (*_1);
Goto(bb7)
}
bb7 = {
_20 = Move(_9);
RET = (-111_i8) as i64;
_12 = (*_1) ^ (*_1);
(*_1) = _12;
(*_1) = _12 ^ _12;
_18.fld2 = (_11,);
_20 = core::ptr::addr_of_mut!(_18.fld1);
Goto(bb8)
}
bb8 = {
_22 = !14961667364911312108_u64;
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = (-56_isize) as u16;
(*_20) = _18.fld2.0 & _11;
Goto(bb9)
}
bb9 = {
_17 = (-9223372036854775808_isize);
(*_20) = _11;
_17 = !9223372036854775807_isize;
(*_20) = _18.fld2.0 ^ _11;
_22 = 16547187166539792285_u64;
(*_1) = !_12;
_14 = [25103_i16];
(*_20) = _18.fld2.0 ^ _18.fld2.0;
(*_20) = _11 ^ _11;
match _22 {
0 => bb8,
1 => bb10,
16547187166539792285 => bb12,
_ => bb11
}
}
bb10 = {
_22 = !14961667364911312108_u64;
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = (-56_isize) as u16;
(*_20) = _18.fld2.0 & _11;
Goto(bb9)
}
bb11 = {
_20 = Move(_9);
RET = (-111_i8) as i64;
_12 = (*_1) ^ (*_1);
(*_1) = _12;
(*_1) = _12 ^ _12;
_18.fld2 = (_11,);
_20 = core::ptr::addr_of_mut!(_18.fld1);
Goto(bb8)
}
bb12 = {
_10 = 2397433214_u32 << _18.fld2.0;
(*_1) = _12;
_3 = [_10,_10,_10,_10];
_18.fld3 = 6849034089406169682_usize & 5_usize;
(*_1) = _12 * _12;
_27.2.3 = _10;
(*_1) = _12;
(*_20) = 79_i8 as u16;
_24 = core::ptr::addr_of!(_25);
_10 = _27.2.3 | _27.2.3;
(*_20) = !_18.fld2.0;
(*_1) = -_12;
(*_1) = _12 & _12;
match _22 {
0 => bb1,
1 => bb5,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
16547187166539792285 => bb19,
_ => bb18
}
}
bb13 = {
_20 = Move(_9);
RET = (-111_i8) as i64;
_12 = (*_1) ^ (*_1);
(*_1) = _12;
(*_1) = _12 ^ _12;
_18.fld2 = (_11,);
_20 = core::ptr::addr_of_mut!(_18.fld1);
Goto(bb8)
}
bb14 = {
_22 = !14961667364911312108_u64;
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = (-56_isize) as u16;
(*_20) = _18.fld2.0 & _11;
Goto(bb9)
}
bb15 = {
(*_1) = (-8537508582074533633_i64);
_12 = (*_1) | (*_1);
_3 = [_10,_10,_10,_10];
_7 = 236_u8 & 58_u8;
(*_1) = _12 >> _7;
(*_1) = _12 ^ _12;
(*_1) = _12 + _12;
(*_1) = 3903717519445885032_u64 as i64;
_2 = _6;
_11 = !8704_u16;
(*_1) = _12 ^ _12;
(*_1) = _12 << _12;
(*_1) = _12;
(*_1) = _12 & _12;
(*_1) = !_12;
(*_1) = _10 as i64;
(*_1) = _12 & _12;
_8 = 1884932981_i32 + (-1118439838_i32);
(*_1) = _12;
(*_1) = !_12;
_12 = (*_1);
(*_1) = _12;
_14 = [21617_i16];
(*_1) = _12;
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb6)
}
bb16 = {
_3 = [3190953012_u32,1564588876_u32,2948244649_u32,3783293419_u32];
RET = 9122819073626746398_i64;
_2 = 444541429_u32 as f64;
_4.2.3 = 4119027572_u32;
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 894867647598339665_i64 | (-4207737756344369692_i64);
(*_1) = 6151540232862038057_i64 | (-2706107598023574111_i64);
(*_1) = 1145798298529738478_i64 * 5201274932782560618_i64;
(*_1) = 4797044314369479401_i64 >> _4.2.0;
(*_1) = 636413839778360113_i64 | (-2462181210541403459_i64);
(*_1) = 14_u8 as i64;
_4.2.2 = &mut _4.2.3;
(*_1) = 9056113559751304999_i64;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = 1649685118_i32 as f64;
(*_1) = (-8120124698631088774_i64);
Goto(bb2)
}
bb17 = {
_20 = Move(_9);
RET = (-111_i8) as i64;
_12 = (*_1) ^ (*_1);
(*_1) = _12;
(*_1) = _12 ^ _12;
_18.fld2 = (_11,);
_20 = core::ptr::addr_of_mut!(_18.fld1);
Goto(bb8)
}
bb18 = {
(*_1) = 4163613605922110958_i64 - 6901914761369284117_i64;
(*_1) = (-2207468693013184313_i64);
_10 = 3822605451_u32 + 2558856000_u32;
(*_1) = (-7902469095439858554_i64);
_8 = (-1559313580_i32) >> (*_1);
(*_1) = !7514891256354783481_i64;
(*_1) = 28395_i16 as i64;
_8 = (-300540533_i32) * 1720708528_i32;
(*_1) = 6176545250454716507_i64 & 5882750369230102867_i64;
_3 = [_10,_10,_10,_10];
(*_1) = (-3969576216038621138_i64) & (-3549539779777644470_i64);
Goto(bb5)
}
bb19 = {
_18.fld2.0 = (*_20) | (*_20);
(*_20) = !_18.fld2.0;
(*_1) = !_12;
(*_1) = _8 as i64;
_8 = 1013011485_i32 | 616235040_i32;
(*_1) = _10 as i64;
_21 = (*_1) != (*_1);
_27.2.0 = _17 as i32;
_9 = core::ptr::addr_of_mut!((*_20));
(*_20) = _10 as u16;
_30 = &mut _24;
(*_20) = _18.fld2.0 & _18.fld2.0;
(*_20) = _18.fld2.0;
_7 = _18.fld3 as u8;
(*_20) = !_11;
(*_20) = _18.fld2.0 + _18.fld2.0;
_18.fld4 = 203475107804231167913668927684000452460_u128 * 153677561914963945218220974140799717196_u128;
_3 = [_27.2.3,_10,_10,_10];
_27.1 = Adt21::Variant0 { fld0: _17 };
(*_1) = _12 << (*_20);
_29 = [(-22882_i16)];
(*_20) = !_18.fld2.0;
Goto(bb20)
}
bb20 = {
Call(_33 = dump_var(Move(_22), Move(_29), Move(_21), Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_33 = dump_var(Move(_14), _34, _34, _34), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut i64,mut _2: f64,mut _3: [char; 1],mut _4: [char; 1],mut _5: f64,mut _6: [char; 1],mut _7: f64,mut _8: f64,mut _9: [char; 1],mut _10: f64,mut _11: f64) -> i32 {
mir! {
type RET = i32;
let _12: u128;
let _13: i8;
let _14: f64;
let _15: *const i64;
let _16: &'static mut u32;
let _17: f32;
let _18: &'static mut *const *const char;
let _19: char;
let _20: char;
let _21: i8;
let _22: u32;
let _23: isize;
let _24: *const i64;
let _25: u32;
let _26: (char, i64, Adt17);
let _27: isize;
let _28: i8;
let _29: Adt55;
let _30: char;
let _31: &'static mut u32;
let _32: i8;
let _33: isize;
let _34: i8;
let _35: *const &'static mut [u32; 3];
let _36: Adt52;
let _37: i64;
let _38: &'static Adt26;
let _39: char;
let _40: &'static [i128; 8];
let _41: *const i64;
let _42: i8;
let _43: (u32,);
let _44: *const Adt21;
let _45: isize;
let _46: (*mut u16,);
let _47: ([u16; 5], i64);
let _48: isize;
let _49: isize;
let _50: &'static Adt66;
let _51: i16;
let _52: &'static mut &'static mut u32;
let _53: ([u16; 5], i64);
let _54: u64;
let _55: char;
let _56: *const char;
let _57: i64;
let _58: Adt81;
let _59: &'static i16;
let _60: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _61: i32;
let _62: u64;
let _63: &'static (f32, i8, i128);
let _64: *const &'static char;
let _65: bool;
let _66: *const Adt21;
let _67: (*mut *mut u16, u128);
let _68: f32;
let _69: isize;
let _70: f32;
let _71: isize;
let _72: &'static (f32, i8, i128);
let _73: i64;
let _74: f64;
let _75: i8;
let _76: &'static char;
let _77: Adt21;
let _78: u8;
let _79: (*mut u16,);
let _80: ((u16,), (char, i64, Adt17));
let _81: usize;
let _82: Adt52;
let _83: *mut &'static char;
let _84: [u16; 5];
let _85: u64;
let _86: i64;
let _87: isize;
let _88: *const *const char;
let _89: i64;
let _90: bool;
let _91: *mut Adt52;
let _92: *mut Adt52;
let _93: &'static i16;
let _94: i16;
let _95: u32;
let _96: i32;
let _97: u32;
let _98: i128;
let _99: [i128; 2];
let _100: ();
let _101: ();
{
_8 = 6_usize as f64;
RET = (-478772104_i32);
_10 = _2;
RET = 715422409_i32 >> 9223372036854775807_isize;
_6 = ['\u{b00f1}'];
_5 = (-8572_i16) as f64;
_8 = _11;
_8 = 21814_i16 as f64;
_12 = 8197614750065601197216821131603905809_u128 - 156337003311992065678491264875416621075_u128;
_12 = !192682986432613921163568085306282801478_u128;
RET = (-954974211_i32) >> _12;
RET = 766489485_i32 ^ (-1428552967_i32);
_3 = _6;
_4 = ['\u{105c3e}'];
_13 = -53_i8;
_7 = _10;
_5 = _2 - _2;
_9 = ['\u{d51d1}'];
_4 = _6;
_8 = _11 - _5;
_8 = _5 + _2;
_6 = _3;
_6 = ['\u{f4129}'];
_5 = -_8;
Call(_14 = fn10(_8, _7, _10, Move(_1), _9, _10, _5, _8, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 1633727442_u32 as i32;
_17 = 3_usize as f32;
RET = 1944778485_i32 << _13;
_9 = ['\u{fb91e}'];
_10 = _8 - _5;
_9 = _6;
_7 = 86_u8 as f64;
_9 = ['\u{2694}'];
_9 = ['\u{b9535}'];
RET = 1560405551_i32 >> _13;
_7 = _5 + _8;
_14 = _7 * _7;
Goto(bb2)
}
bb2 = {
_4 = ['\u{f36b3}'];
_6 = ['\u{f70b}'];
_10 = -_5;
_17 = 3788639210_u32 as f32;
_3 = _6;
_14 = 16877_i16 as f64;
_7 = -_5;
_21 = '\u{1313d}' as i8;
_20 = '\u{82990}';
_22 = !1841918076_u32;
_19 = _20;
_8 = _5;
Goto(bb3)
}
bb3 = {
_12 = 279380062910817942268186281682810064932_u128;
_5 = _7 * _10;
_7 = _10 + _8;
_2 = _5;
_3 = [_19];
_2 = _7;
_16 = &mut _22;
(*_16) = 1565370380_u32 * 3202636842_u32;
(*_16) = 2498670057_u32;
(*_16) = 1670448046_u32 ^ 3222196664_u32;
_23 = 83_isize;
(*_16) = 3314293469_u32;
(*_16) = !2460612494_u32;
(*_16) = 2946965067_u32;
(*_16) = _12 as u32;
(*_16) = 1605444212_u32 & 262299224_u32;
_13 = _21 * _21;
_17 = _21 as f32;
RET = 430182948_i32 << (*_16);
_9 = [_19];
_3 = _9;
(*_16) = 4288733865_u32 - 1980621002_u32;
Goto(bb4)
}
bb4 = {
_14 = _8 - _8;
(*_16) = !2334156405_u32;
_19 = _20;
(*_16) = !2944523742_u32;
_11 = _2 - _2;
_1 = core::ptr::addr_of_mut!(_26.1);
(*_1) = (-5481191280274842019_i64) ^ 2853611579919953731_i64;
RET = (-126351122_i32);
(*_16) = 1376968856_u32 & 102983958_u32;
(*_16) = RET as u32;
_25 = (*_16);
_26.1 = 880204572450145437_i64 - 2562173438304373038_i64;
_28 = _21 >> (*_1);
(*_1) = RET as i64;
_12 = 15252876933730495077_usize as u128;
match _23 {
0 => bb1,
1 => bb2,
83 => bb5,
_ => bb3
}
}
bb5 = {
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb6)
}
bb6 = {
(*_16) = _25 + _25;
_7 = _14;
RET = (-690130273_i32) >> (*_16);
(*_1) = 47135_u16 as i64;
(*_1) = 6302366435732128707_u64 as i64;
(*_1) = (-3137087201741052439_i64) >> (*_16);
(*_1) = 3297023958207938672_i64 | (-392851250929410990_i64);
_3 = [_19];
(*_16) = _25 - _25;
_26.1 = (-2418102882692409822_i64) - (-8505427614144546796_i64);
_7 = _11 * _14;
(*_1) = -4168179705104398037_i64;
_15 = core::ptr::addr_of!((*_1));
(*_16) = _25 * _25;
(*_1) = !(-7593836935098645895_i64);
_17 = _12 as f32;
_6 = _4;
(*_1) = (-5793910342976971826_i64) ^ (-8181935863831024392_i64);
_16 = &mut _25;
(*_1) = (-9028499170491443372_i64);
(*_16) = 3311444979_u32;
(*_1) = 582059520413105261_i64;
_17 = (*_1) as f32;
(*_16) = (-5214_i16) as u32;
Goto(bb7)
}
bb7 = {
(*_16) = 2328555731_u32;
_9 = [_19];
_3 = _6;
(*_16) = 5662468452606708606_u64 as u32;
match (*_1) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
582059520413105261 => bb16,
_ => bb15
}
}
bb8 = {
(*_16) = _25 + _25;
_7 = _14;
RET = (-690130273_i32) >> (*_16);
(*_1) = 47135_u16 as i64;
(*_1) = 6302366435732128707_u64 as i64;
(*_1) = (-3137087201741052439_i64) >> (*_16);
(*_1) = 3297023958207938672_i64 | (-392851250929410990_i64);
_3 = [_19];
(*_16) = _25 - _25;
_26.1 = (-2418102882692409822_i64) - (-8505427614144546796_i64);
_7 = _11 * _14;
(*_1) = -4168179705104398037_i64;
_15 = core::ptr::addr_of!((*_1));
(*_16) = _25 * _25;
(*_1) = !(-7593836935098645895_i64);
_17 = _12 as f32;
_6 = _4;
(*_1) = (-5793910342976971826_i64) ^ (-8181935863831024392_i64);
_16 = &mut _25;
(*_1) = (-9028499170491443372_i64);
(*_16) = 3311444979_u32;
(*_1) = 582059520413105261_i64;
_17 = (*_1) as f32;
(*_16) = (-5214_i16) as u32;
Goto(bb7)
}
bb9 = {
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb6)
}
bb10 = {
_14 = _8 - _8;
(*_16) = !2334156405_u32;
_19 = _20;
(*_16) = !2944523742_u32;
_11 = _2 - _2;
_1 = core::ptr::addr_of_mut!(_26.1);
(*_1) = (-5481191280274842019_i64) ^ 2853611579919953731_i64;
RET = (-126351122_i32);
(*_16) = 1376968856_u32 & 102983958_u32;
(*_16) = RET as u32;
_25 = (*_16);
_26.1 = 880204572450145437_i64 - 2562173438304373038_i64;
_28 = _21 >> (*_1);
(*_1) = RET as i64;
_12 = 15252876933730495077_usize as u128;
match _23 {
0 => bb1,
1 => bb2,
83 => bb5,
_ => bb3
}
}
bb11 = {
_12 = 279380062910817942268186281682810064932_u128;
_5 = _7 * _10;
_7 = _10 + _8;
_2 = _5;
_3 = [_19];
_2 = _7;
_16 = &mut _22;
(*_16) = 1565370380_u32 * 3202636842_u32;
(*_16) = 2498670057_u32;
(*_16) = 1670448046_u32 ^ 3222196664_u32;
_23 = 83_isize;
(*_16) = 3314293469_u32;
(*_16) = !2460612494_u32;
(*_16) = 2946965067_u32;
(*_16) = _12 as u32;
(*_16) = 1605444212_u32 & 262299224_u32;
_13 = _21 * _21;
_17 = _21 as f32;
RET = 430182948_i32 << (*_16);
_9 = [_19];
_3 = _9;
(*_16) = 4288733865_u32 - 1980621002_u32;
Goto(bb4)
}
bb12 = {
_4 = ['\u{f36b3}'];
_6 = ['\u{f70b}'];
_10 = -_5;
_17 = 3788639210_u32 as f32;
_3 = _6;
_14 = 16877_i16 as f64;
_7 = -_5;
_21 = '\u{1313d}' as i8;
_20 = '\u{82990}';
_22 = !1841918076_u32;
_19 = _20;
_8 = _5;
Goto(bb3)
}
bb13 = {
RET = 1633727442_u32 as i32;
_17 = 3_usize as f32;
RET = 1944778485_i32 << _13;
_9 = ['\u{fb91e}'];
_10 = _8 - _5;
_9 = _6;
_7 = 86_u8 as f64;
_9 = ['\u{2694}'];
_9 = ['\u{b9535}'];
RET = 1560405551_i32 >> _13;
_7 = _5 + _8;
_14 = _7 * _7;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
(*_1) = 23998_i16 as i64;
_27 = _23 - _23;
_11 = (*_16) as f64;
(*_1) = _28 as i64;
_26.2 = Adt17::Variant2 { fld0: _12,fld1: _20,fld2: 50_u8,fld3: _28,fld4: _17,fld5: (*_1) };
(*_1) = Field::<i64>(Variant(_26.2, 2), 5) | Field::<i64>(Variant(_26.2, 2), 5);
match _23 {
83 => bb17,
_ => bb7
}
}
bb17 = {
_11 = _14 - _14;
(*_16) = 3542875823_u32;
_26.1 = Field::<i64>(Variant(_26.2, 2), 5) - Field::<i64>(Variant(_26.2, 2), 5);
(*_1) = Field::<i64>(Variant(_26.2, 2), 5);
(*_1) = !Field::<i64>(Variant(_26.2, 2), 5);
(*_16) = 2769104152_u32 << _27;
Goto(bb18)
}
bb18 = {
(*_16) = !2166279645_u32;
place!(Field::<char>(Variant(_26.2, 2), 1)) = _19;
_28 = _13 << (*_16);
(*_1) = Field::<i64>(Variant(_26.2, 2), 5) - Field::<i64>(Variant(_26.2, 2), 5);
(*_16) = !2676208382_u32;
(*_1) = Field::<i64>(Variant(_26.2, 2), 5) & Field::<i64>(Variant(_26.2, 2), 5);
(*_1) = -Field::<i64>(Variant(_26.2, 2), 5);
(*_1) = Field::<i64>(Variant(_26.2, 2), 5) ^ Field::<i64>(Variant(_26.2, 2), 5);
_3 = [_20];
_30 = _19;
_4 = [_19];
(*_1) = -Field::<i64>(Variant(_26.2, 2), 5);
_10 = _27 as f64;
_10 = _17 as f64;
_14 = -_7;
_19 = _20;
_21 = _28 & _28;
_36 = Adt52::Variant0 { fld0: _27 };
_24 = core::ptr::addr_of!((*_1));
_14 = _23 as f64;
_2 = _11 * _5;
(*_16) = 2873039900_u32 - 3534140882_u32;
(*_1) = Field::<i64>(Variant(_26.2, 2), 5);
_26.1 = Field::<i64>(Variant(_26.2, 2), 5);
Goto(bb19)
}
bb19 = {
(*_16) = 2676499126_u32 * 356098189_u32;
(*_16) = 2638168721_u32 ^ 3050630560_u32;
_4 = [_30];
place!(Field::<f32>(Variant(_26.2, 2), 4)) = _17;
_3 = [Field::<char>(Variant(_26.2, 2), 1)];
place!(Field::<i64>(Variant(_26.2, 2), 5)) = (*_1) + _26.1;
_32 = _28 | _28;
_17 = _2 as f32;
(*_16) = 1335922085_u32 - 2025530312_u32;
_36 = Adt52::Variant0 { fld0: _27 };
(*_16) = 3345398869_u32 * 3633870970_u32;
(*_16) = _30 as u32;
match _23 {
0 => bb20,
83 => bb22,
_ => bb21
}
}
bb20 = {
(*_16) = _25 + _25;
_7 = _14;
RET = (-690130273_i32) >> (*_16);
(*_1) = 47135_u16 as i64;
(*_1) = 6302366435732128707_u64 as i64;
(*_1) = (-3137087201741052439_i64) >> (*_16);
(*_1) = 3297023958207938672_i64 | (-392851250929410990_i64);
_3 = [_19];
(*_16) = _25 - _25;
_26.1 = (-2418102882692409822_i64) - (-8505427614144546796_i64);
_7 = _11 * _14;
(*_1) = -4168179705104398037_i64;
_15 = core::ptr::addr_of!((*_1));
(*_16) = _25 * _25;
(*_1) = !(-7593836935098645895_i64);
_17 = _12 as f32;
_6 = _4;
(*_1) = (-5793910342976971826_i64) ^ (-8181935863831024392_i64);
_16 = &mut _25;
(*_1) = (-9028499170491443372_i64);
(*_16) = 3311444979_u32;
(*_1) = 582059520413105261_i64;
_17 = (*_1) as f32;
(*_16) = (-5214_i16) as u32;
Goto(bb7)
}
bb21 = {
(*_16) = _25 + _25;
_7 = _14;
RET = (-690130273_i32) >> (*_16);
(*_1) = 47135_u16 as i64;
(*_1) = 6302366435732128707_u64 as i64;
(*_1) = (-3137087201741052439_i64) >> (*_16);
(*_1) = 3297023958207938672_i64 | (-392851250929410990_i64);
_3 = [_19];
(*_16) = _25 - _25;
_26.1 = (-2418102882692409822_i64) - (-8505427614144546796_i64);
_7 = _11 * _14;
(*_1) = -4168179705104398037_i64;
_15 = core::ptr::addr_of!((*_1));
(*_16) = _25 * _25;
(*_1) = !(-7593836935098645895_i64);
_17 = _12 as f32;
_6 = _4;
(*_1) = (-5793910342976971826_i64) ^ (-8181935863831024392_i64);
_16 = &mut _25;
(*_1) = (-9028499170491443372_i64);
(*_16) = 3311444979_u32;
(*_1) = 582059520413105261_i64;
_17 = (*_1) as f32;
(*_16) = (-5214_i16) as u32;
Goto(bb7)
}
bb22 = {
(*_16) = 9277314662383390694_usize as u32;
(*_16) = 8900_u16 as u32;
place!(Field::<f32>(Variant(_26.2, 2), 4)) = _17 + _17;
(*_1) = Field::<isize>(Variant(_36, 0), 0) as i64;
(*_16) = !3706810716_u32;
(*_1) = RET as i64;
(*_16) = 965029945_u32;
_23 = _27;
(*_16) = 3402312835_u32;
_13 = 232_u8 as i8;
RET = (-28944_i16) as i32;
Goto(bb23)
}
bb23 = {
(*_1) = Field::<i64>(Variant(_26.2, 2), 5) + Field::<i64>(Variant(_26.2, 2), 5);
(*_16) = false as u32;
(*_1) = -Field::<i64>(Variant(_26.2, 2), 5);
_5 = -_11;
(*_1) = -Field::<i64>(Variant(_26.2, 2), 5);
_37 = (*_1) + (*_1);
(*_1) = _37;
_3 = [_19];
(*_16) = 1878893341_u32;
RET = -(-1738464400_i32);
_7 = _11 - _2;
Call((*_1) = core::intrinsics::transmute(_23), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_1) = _37;
_43 = ((*_16),);
_24 = Move(_15);
(*_1) = Field::<f32>(Variant(_26.2, 2), 4) as i64;
_10 = _11 - _7;
(*_16) = _43.0 + _43.0;
(*_16) = _43.0 * _43.0;
_26.0 = _19;
RET = !647615746_i32;
_34 = _32 & _21;
(*_16) = _43.0 & _43.0;
_15 = core::ptr::addr_of!(_26.1);
_47.1 = (*_15) & (*_1);
(*_15) = _47.1 & _47.1;
(*_15) = 6684627687791946421_u64 as i64;
(*_15) = _47.1 & _47.1;
_41 = core::ptr::addr_of!((*_1));
(*_16) = _43.0;
(*_16) = _43.0 / _43.0;
(*_15) = _47.1;
place!(Field::<u8>(Variant(_26.2, 2), 2)) = 162_u8 | 95_u8;
Call((*_16) = core::intrinsics::bswap(_43.0), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
(*_15) = !_47.1;
_15 = core::ptr::addr_of!((*_15));
place!(Field::<char>(Variant(_26.2, 2), 1)) = _30;
_23 = Field::<char>(Variant(_26.2, 2), 1) as isize;
(*_15) = _47.1 * _47.1;
_47.1 = (*_15) + (*_1);
_26.1 = RET as i64;
_13 = Field::<i8>(Variant(_26.2, 2), 3) >> (*_1);
(*_1) = _47.1 << _34;
(*_16) = _43.0 + _43.0;
Call((*_16) = core::intrinsics::bswap(_43.0), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_13 = false as i8;
(*_16) = !_43.0;
(*_16) = _43.0 | _43.0;
_9 = [_20];
_41 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_26.2, 2), 5)));
_26.2 = Adt17::Variant0 { fld0: _10,fld1: _26.0,fld2: _27,fld3: _12,fld4: 16090_i16,fld5: _17 };
_41 = Move(_15);
_7 = -Field::<f64>(Variant(_26.2, 0), 0);
Call((*_16) = core::intrinsics::bswap(_43.0), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_1) = _37;
(*_1) = _47.1;
place!(Field::<char>(Variant(_26.2, 0), 1)) = _19;
(*_16) = _43.0 << (*_1);
(*_16) = _43.0;
(*_16) = !_43.0;
_14 = -_7;
_11 = _7 - _14;
_7 = _14;
Call(_5 = core::intrinsics::transmute((*_1)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_20 = Field::<char>(Variant(_26.2, 0), 1);
_53.0 = [38517_u16,1532_u16,28593_u16,51605_u16,26126_u16];
place!(Field::<isize>(Variant(_26.2, 0), 2)) = Field::<isize>(Variant(_36, 0), 0) << (*_1);
_41 = core::ptr::addr_of!((*_1));
_47 = (_53.0, (*_1));
Goto(bb29)
}
bb29 = {
_37 = -(*_41);
_30 = _20;
(*_1) = 13144_u16 as i64;
(*_1) = -_37;
(*_1) = _34 as i64;
_47.0 = _53.0;
_52 = &mut _16;
_10 = _7;
_51 = (-8255_i16) - 18903_i16;
(*_52) = &mut _43.0;
_20 = _26.0;
_37 = _26.1 + (*_1);
_54 = !4079787893382872579_u64;
(*_1) = _47.1;
Goto(bb30)
}
bb30 = {
place!(Field::<f64>(Variant(_26.2, 0), 0)) = (-26269474541913961963795841314954883906_i128) as f64;
_15 = core::ptr::addr_of!((*_1));
_5 = _14 * _11;
_20 = _26.0;
(*_15) = -_47.1;
(*_15) = _47.1 + _47.1;
_53 = (_47.0, (*_1));
_1 = core::ptr::addr_of_mut!((*_1));
_33 = Field::<isize>(Variant(_26.2, 0), 2);
_53 = (_47.0, _47.1);
_48 = -_33;
_15 = core::ptr::addr_of!((*_15));
_2 = _10;
_17 = Field::<f32>(Variant(_26.2, 0), 5);
_42 = _34 << (*_15);
_30 = _20;
(*_15) = _47.1 >> _47.1;
(*_1) = -_53.1;
(*_1) = _53.1 & _53.1;
place!(Field::<isize>(Variant(_26.2, 0), 2)) = _48 + _33;
_53.0 = [61929_u16,25746_u16,32833_u16,37650_u16,49099_u16];
_39 = _20;
_45 = _33 & _48;
_2 = _11 - _7;
_8 = -_2;
_1 = core::ptr::addr_of_mut!((*_1));
_42 = -_32;
_47.0 = [25750_u16,5886_u16,4741_u16,29393_u16,64204_u16];
_21 = _34 + _34;
(*_1) = _47.1;
Goto(bb31)
}
bb31 = {
_56 = core::ptr::addr_of!(_26.0);
(*_1) = _37;
(*_56) = Field::<char>(Variant(_26.2, 0), 1);
(*_56) = _20;
(*_56) = _19;
(*_1) = !_53.1;
_54 = _14 as u64;
(*_1) = _53.1;
_32 = _21 & _21;
_13 = _32 ^ _42;
_2 = _8;
(*_1) = _47.1 & _47.1;
place!(Field::<u128>(Variant(_26.2, 0), 3)) = !_12;
_14 = _11;
Goto(bb32)
}
bb32 = {
_8 = -_14;
_31 = Move((*_52));
_26.0 = _19;
_51 = -15685_i16;
_48 = -_45;
(*_1) = _37 & _53.1;
_53.0 = _47.0;
(*_56) = _19;
(*_1) = _47.1 | _47.1;
_49 = Field::<isize>(Variant(_26.2, 0), 2);
_2 = _8 * _14;
_17 = Field::<f32>(Variant(_26.2, 0), 5) * Field::<f32>(Variant(_26.2, 0), 5);
place!(Field::<f32>(Variant(_26.2, 0), 5)) = _17 - _17;
(*_1) = _47.1 ^ _53.1;
_26.2 = Adt17::Variant3 { fld0: _12,fld1: (*_56),fld2: 3557938359_u32,fld3: _14,fld4: _54,fld5: RET,fld6: (*_1),fld7: (-16289945014322169037803889801545526497_i128) };
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _47.1 >> _48;
(*_1) = Field::<i64>(Variant(_26.2, 3), 6) - Field::<i64>(Variant(_26.2, 3), 6);
(*_1) = _47.1 + _47.1;
Call((*_56) = fn11(_23, Move(_1), Move(_36), Field::<char>(Variant(_26.2, 3), 1), Move(_56), Move(_41)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_58.fld4.0 = core::ptr::addr_of_mut!(_46.0);
_58.fld2 = (_17, _11, Field::<i64>(Variant(_26.2, 3), 6));
_55 = _39;
_58.fld1.1.0 = Field::<char>(Variant(_26.2, 3), 1);
_46.0 = core::ptr::addr_of_mut!(_58.fld1.0.0);
place!(Field::<char>(Variant(_26.2, 3), 1)) = _39;
_17 = _58.fld2.0 + _58.fld2.0;
_6 = [_30];
Goto(bb34)
}
bb34 = {
_15 = core::ptr::addr_of!(_57);
_48 = _45 + _49;
_12 = !Field::<u128>(Variant(_26.2, 3), 0);
_12 = !Field::<u128>(Variant(_26.2, 3), 0);
_7 = -_58.fld2.1;
_33 = _48 ^ _45;
(*_15) = _47.1 | _26.1;
_47.1 = (*_15);
Goto(bb35)
}
bb35 = {
_60.1.0.2 = !148678842241298393840167723194034231592_i128;
_58.fld1.1.0 = _19;
_14 = _5;
(*_15) = _53.1;
_19 = _30;
(*_15) = _53.1;
place!(Field::<u64>(Variant(_26.2, 3), 4)) = false as u64;
(*_15) = _58.fld2.2 ^ _26.1;
Goto(bb36)
}
bb36 = {
_60.1.1 = 1_usize ^ 13279029722697465835_usize;
_39 = _58.fld1.1.0;
_5 = _58.fld2.1 * Field::<f64>(Variant(_26.2, 3), 3);
(*_15) = Field::<f64>(Variant(_26.2, 3), 3) as i64;
_53 = _47;
_53.0 = [31158_u16,63111_u16,8771_u16,3534_u16,15342_u16];
_60.1.0.0 = -_17;
_13 = _21 << Field::<i64>(Variant(_26.2, 3), 6);
_8 = _12 as f64;
place!(Field::<u64>(Variant(_26.2, 3), 4)) = _54;
_24 = core::ptr::addr_of!((*_15));
Goto(bb37)
}
bb37 = {
_34 = _13;
_44 = core::ptr::addr_of!(_60.0);
_24 = Move(_15);
(*_44) = Adt21::Variant2 { fld0: 348522742_u32,fld1: _58.fld1.1.0,fld2: _60.1.0.0,fld3: _5,fld4: _60.1.1 };
_58.fld1.0 = (38062_u16,);
_58.fld1.0 = (42974_u16,);
_60.1.0 = (Field::<f32>(Variant((*_44), 2), 2), _34, 88993905783359123597209592939077221712_i128);
_8 = _12 as f64;
_59 = &_51;
_39 = Field::<char>(Variant((*_44), 2), 1);
place!(Field::<f64>(Variant((*_44), 2), 3)) = _58.fld1.0.0 as f64;
place!(Field::<usize>(Variant((*_44), 2), 4)) = _60.1.1;
(*_44) = Adt21::Variant1 { fld0: _58.fld2.2,fld1: _60.1.1,fld2: _58.fld1.0,fld3: 21_u8 };
place!(Field::<(u16,)>(Variant(_60.0, 1), 2)).0 = _58.fld1.0.0 + _58.fld1.0.0;
RET = Field::<i32>(Variant(_26.2, 3), 5) + Field::<i32>(Variant(_26.2, 3), 5);
place!(Field::<usize>(Variant((*_44), 1), 1)) = _60.1.1;
_68 = _58.fld2.0 - _17;
Goto(bb38)
}
bb38 = {
_60.1.0.0 = _34 as f32;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_58.fld1.0.0,);
_56 = core::ptr::addr_of!(place!(Field::<char>(Variant(_26.2, 3), 1)));
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_58.fld1.0.0,);
place!(Field::<u8>(Variant((*_44), 1), 3)) = 203_u8 + 193_u8;
Goto(bb39)
}
bb39 = {
place!(Field::<usize>(Variant((*_44), 1), 1)) = _60.1.1 | _60.1.1;
place!(Field::<usize>(Variant((*_44), 1), 1)) = Field::<u64>(Variant(_26.2, 3), 4) as usize;
_58.fld0 = true ^ false;
_23 = _49 | _49;
place!(Field::<usize>(Variant((*_44), 1), 1)) = 3769050718_u32 as usize;
_48 = _49;
place!(Field::<i64>(Variant((*_44), 1), 0)) = _58.fld2.2 - _57;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)).0 = _58.fld1.0.0 * _58.fld1.0.0;
_49 = _48 ^ _45;
_28 = -_21;
place!(Field::<usize>(Variant((*_44), 1), 1)) = !_60.1.1;
(*_56) = _20;
place!(Field::<usize>(Variant((*_44), 1), 1)) = _60.1.1 + _60.1.1;
_28 = _13;
place!(Field::<usize>(Variant((*_44), 1), 1)) = _60.1.1;
(*_56) = _30;
(*_44) = Adt21::Variant2 { fld0: 2557519259_u32,fld1: (*_56),fld2: _17,fld3: Field::<f64>(Variant(_26.2, 3), 3),fld4: _60.1.1 };
place!(Field::<char>(Variant((*_44), 2), 1)) = _19;
_68 = 4261037531_u32 as f32;
(*_44) = Adt21::Variant1 { fld0: _26.1,fld1: _60.1.1,fld2: _58.fld1.0,fld3: 57_u8 };
match Field::<(u16,)>(Variant((*_44), 1), 2).0 {
0 => bb4,
42974 => bb40,
_ => bb19
}
}
bb40 = {
_58.fld2 = (_60.1.0.0, _11, Field::<i64>(Variant((*_44), 1), 0));
place!(Field::<u8>(Variant((*_44), 1), 3)) = !220_u8;
place!(Field::<u8>(Variant((*_44), 1), 3)) = 10_u8 * 114_u8;
place!(Field::<u8>(Variant((*_44), 1), 3)) = !86_u8;
_53.0 = _47.0;
_1 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant((*_44), 1), 0)));
place!(Field::<u32>(Variant(_26.2, 3), 2)) = 902315546_u32;
_67.0 = core::ptr::addr_of_mut!(_46.0);
_45 = -_23;
place!(Field::<i64>(Variant((*_44), 1), 0)) = _21 as i64;
(*_44) = Adt21::Variant1 { fld0: _57,fld1: _60.1.1,fld2: _58.fld1.0,fld3: 112_u8 };
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_58.fld1.0.0,);
_20 = _26.0;
place!(Field::<i64>(Variant((*_44), 1), 0)) = _53.1 & _57;
(*_56) = _20;
Call(place!(Field::<u8>(Variant((*_44), 1), 3)) = core::intrinsics::transmute(_32), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
place!(Field::<i64>(Variant((*_44), 1), 0)) = _53.1;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)).0 = _58.fld1.0.0 / _58.fld1.0.0;
_56 = core::ptr::addr_of!((*_56));
place!(Field::<usize>(Variant((*_44), 1), 1)) = _60.1.1;
(*_44) = Adt21::Variant1 { fld0: _37,fld1: _60.1.1,fld2: _58.fld1.0,fld3: 186_u8 };
place!(Field::<usize>(Variant((*_44), 1), 1)) = !_60.1.1;
place!(Field::<u8>(Variant((*_44), 1), 3)) = 59_u8;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_58.fld1.0.0,);
(*_44) = Adt21::Variant2 { fld0: Field::<u32>(Variant(_26.2, 3), 2),fld1: (*_56),fld2: _17,fld3: _2,fld4: _60.1.1 };
RET = Field::<i32>(Variant(_26.2, 3), 5) + Field::<i32>(Variant(_26.2, 3), 5);
(*_56) = Field::<char>(Variant((*_44), 2), 1);
place!(Field::<f32>(Variant((*_44), 2), 2)) = _60.1.0.2 as f32;
match _60.1.0.2 {
0 => bb13,
1 => bb42,
88993905783359123597209592939077221712 => bb44,
_ => bb43
}
}
bb42 = {
_56 = core::ptr::addr_of!(_26.0);
(*_1) = _37;
(*_56) = Field::<char>(Variant(_26.2, 0), 1);
(*_56) = _20;
(*_56) = _19;
(*_1) = !_53.1;
_54 = _14 as u64;
(*_1) = _53.1;
_32 = _21 & _21;
_13 = _32 ^ _42;
_2 = _8;
(*_1) = _47.1 & _47.1;
place!(Field::<u128>(Variant(_26.2, 0), 3)) = !_12;
_14 = _11;
Goto(bb32)
}
bb43 = {
_60.1.0.2 = !148678842241298393840167723194034231592_i128;
_58.fld1.1.0 = _19;
_14 = _5;
(*_15) = _53.1;
_19 = _30;
(*_15) = _53.1;
place!(Field::<u64>(Variant(_26.2, 3), 4)) = false as u64;
(*_15) = _58.fld2.2 ^ _26.1;
Goto(bb36)
}
bb44 = {
place!(Field::<f32>(Variant((*_44), 2), 2)) = _58.fld2.0 * _17;
place!(Field::<usize>(Variant((*_44), 2), 4)) = _60.1.1 & _60.1.1;
place!(Field::<f32>(Variant((*_44), 2), 2)) = _60.1.0.0;
place!(Field::<f64>(Variant((*_44), 2), 3)) = -_10;
_75 = _28;
_20 = Field::<char>(Variant((*_44), 2), 1);
_60.0 = Adt21::Variant2 { fld0: Field::<u32>(Variant(_26.2, 3), 2),fld1: (*_56),fld2: _58.fld2.0,fld3: _58.fld2.1,fld4: _60.1.1 };
place!(Field::<f32>(Variant((*_44), 2), 2)) = _58.fld2.0 + _17;
_80.1.0 = Field::<char>(Variant((*_44), 2), 1);
place!(Field::<usize>(Variant((*_44), 2), 4)) = !_60.1.1;
place!(Field::<u32>(Variant((*_44), 2), 0)) = Field::<usize>(Variant((*_44), 2), 4) as u32;
Goto(bb45)
}
bb45 = {
place!(Field::<usize>(Variant((*_44), 2), 4)) = _60.1.1 - _60.1.1;
_13 = !_34;
place!(Field::<usize>(Variant((*_44), 2), 4)) = _26.1 as usize;
place!(Field::<char>(Variant((*_44), 2), 1)) = (*_56);
_67.0 = core::ptr::addr_of_mut!(_79.0);
place!(Field::<f32>(Variant((*_44), 2), 2)) = _17;
_74 = _11 * _14;
_32 = !_34;
place!(Field::<usize>(Variant((*_44), 2), 4)) = !_60.1.1;
place!(Field::<usize>(Variant((*_44), 2), 4)) = !_60.1.1;
_76 = &place!(Field::<char>(Variant((*_44), 2), 1));
_83 = core::ptr::addr_of_mut!(_76);
place!(Field::<u32>(Variant(_60.0, 2), 0)) = _58.fld1.0.0 as u32;
place!(Field::<f32>(Variant((*_44), 2), 2)) = _17 * _60.1.0.0;
place!(Field::<char>(Variant((*_44), 2), 1)) = (*_56);
place!(Field::<f32>(Variant((*_44), 2), 2)) = _34 as f32;
place!(Field::<f64>(Variant((*_44), 2), 3)) = _14 + _10;
_47.0 = _53.0;
_33 = Field::<u128>(Variant(_26.2, 3), 0) as isize;
Goto(bb46)
}
bb46 = {
_88 = core::ptr::addr_of!(_56);
_19 = (*_56);
(*_44) = Adt21::Variant0 { fld0: _45 };
_1 = core::ptr::addr_of_mut!(_73);
(*_1) = _60.1.0.2 as i64;
place!(Field::<isize>(Variant(_60.0, 0), 0)) = _23 << _60.1.0.2;
_67.1 = _12 >> (*_59);
(*_56) = _30;
place!(Field::<isize>(Variant((*_44), 0), 0)) = _17 as isize;
place!(Field::<isize>(Variant((*_44), 0), 0)) = _48;
(*_1) = 41_u8 as i64;
_71 = -_49;
_60.1.1 = _60.1.0.2 as usize;
(*_88) = core::ptr::addr_of!((*_56));
(*_88) = core::ptr::addr_of!((*_56));
_6 = [(*_56)];
(*_44) = Adt21::Variant2 { fld0: Field::<u32>(Variant(_26.2, 3), 2),fld1: _39,fld2: _60.1.0.0,fld3: Field::<f64>(Variant(_26.2, 3), 3),fld4: _60.1.1 };
_37 = Field::<f32>(Variant((*_44), 2), 2) as i64;
_26.2 = Adt17::Variant1 { fld0: 30_u8 };
(*_44) = Adt21::Variant1 { fld0: _47.1,fld1: _60.1.1,fld2: _58.fld1.0,fld3: 234_u8 };
place!(Field::<(u16,)>(Variant((*_44), 1), 2)).0 = _58.fld1.0.0 ^ _58.fld1.0.0;
place!(Field::<u8>(Variant((*_44), 1), 3)) = 127_u8 - 217_u8;
_80.0.0 = Field::<(u16,)>(Variant((*_44), 1), 2).0;
_58.fld1.1.2 = Adt17::Variant1 { fld0: Field::<u8>(Variant(_60.0, 1), 3) };
(*_1) = Field::<i64>(Variant((*_44), 1), 0) ^ Field::<i64>(Variant((*_44), 1), 0);
Goto(bb47)
}
bb47 = {
(*_83) = &_58.fld1.1.0;
(*_44) = Adt21::Variant1 { fld0: (*_1),fld1: _60.1.1,fld2: _80.0,fld3: Field::<u8>(Variant(_58.fld1.1.2, 1), 0) };
(*_83) = &_80.1.0;
_58.fld2 = (_60.1.0.0, _7, (*_1));
place!(Field::<u8>(Variant((*_44), 1), 3)) = 303458228_u32 as u8;
(*_88) = core::ptr::addr_of!((*_76));
place!(Field::<u8>(Variant((*_44), 1), 3)) = Field::<u8>(Variant(_58.fld1.1.2, 1), 0) & Field::<u8>(Variant(_58.fld1.1.2, 1), 0);
_65 = _58.fld0;
_77 = Adt21::Variant2 { fld0: 914496572_u32,fld1: (*_56),fld2: _17,fld3: _2,fld4: Field::<usize>(Variant((*_44), 1), 1) };
_93 = &_51;
place!(Field::<u8>(Variant((*_44), 1), 3)) = Field::<u8>(Variant(_58.fld1.1.2, 1), 0);
place!(Field::<u8>(Variant(_26.2, 1), 0)) = !Field::<u8>(Variant(_60.0, 1), 3);
(*_1) = -Field::<i64>(Variant((*_44), 1), 0);
_58.fld1.1.0 = (*_56);
_96 = _60.1.0.2 as i32;
_95 = 3622274619_u32;
(*_83) = &_19;
_21 = _75 >> _60.1.0.1;
_58.fld2 = (_60.1.0.0, _7, Field::<i64>(Variant((*_44), 1), 0));
place!(Field::<u8>(Variant((*_44), 1), 3)) = !Field::<u8>(Variant(_58.fld1.1.2, 1), 0);
_26 = ((*_76), Field::<i64>(Variant((*_44), 1), 0), Move(_58.fld1.1.2));
_58.fld4.1 = _67.1 & _67.1;
place!(Field::<i64>(Variant((*_44), 1), 0)) = (*_1);
(*_83) = &_58.fld1.1.0;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_80.0.0,);
_96 = RET;
_69 = _49;
match _60.1.0.2 {
0 => bb26,
1 => bb33,
2 => bb29,
3 => bb12,
4 => bb21,
88993905783359123597209592939077221712 => bb48,
_ => bb9
}
}
bb48 = {
place!(Field::<usize>(Variant((*_44), 1), 1)) = !_60.1.1;
place!(Field::<i64>(Variant((*_44), 1), 0)) = !(*_1);
place!(Field::<(u16,)>(Variant((*_44), 1), 2)).0 = !_80.0.0;
place!(Field::<(u16,)>(Variant((*_44), 1), 2)).0 = _80.0.0;
Goto(bb49)
}
bb49 = {
place!(Field::<u8>(Variant((*_44), 1), 3)) = Field::<u8>(Variant(_26.2, 1), 0);
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_58.fld1.0.0,);
(*_83) = &(*_56);
place!(Field::<(u16,)>(Variant(_60.0, 1), 2)).0 = _80.0.0 % _58.fld1.0.0;
place!(Field::<i64>(Variant((*_44), 1), 0)) = _95 as i64;
_98 = _60.1.0.2;
(*_88) = core::ptr::addr_of!(_55);
place!(Field::<(u16,)>(Variant((*_44), 1), 2)) = (_80.0.0,);
_71 = _49 ^ _23;
(*_88) = core::ptr::addr_of!((*_76));
(*_44) = Adt21::Variant0 { fld0: _71 };
Goto(bb50)
}
bb50 = {
Call(_100 = dump_var(Move(_27), Move(_57), Move(_20), Move(_53)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_100 = dump_var(Move(_55), Move(_12), Move(_19), Move(_30)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_100 = dump_var(Move(_43), Move(_98), Move(_73), Move(_34)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_100 = dump_var(Move(_3), Move(_32), Move(_4), Move(_37)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_100 = dump_var(Move(_71), Move(_75), Move(_39), _101), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: *mut i64,mut _5: [char; 1],mut _6: f64,mut _7: f64,mut _8: f64,mut _9: [char; 1]) -> f64 {
mir! {
type RET = f64;
let _10: [u16; 5];
let _11: *mut i64;
let _12: (&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32);
let _13: &'static Adt66;
let _14: isize;
let _15: (u16,);
let _16: *mut [i16; 1];
let _17: bool;
let _18: Adt26;
let _19: ();
let _20: ();
{
RET = -_7;
_6 = -_7;
_6 = _1 * RET;
_5 = _9;
_5 = ['\u{1d5c8}'];
_7 = (-14_isize) as f64;
_2 = (-7051560828900308999_i64) as f64;
_9 = ['\u{1d576}'];
_3 = _1;
RET = _1;
_11 = Move(_4);
_10 = [47743_u16,33350_u16,56441_u16,51050_u16,29775_u16];
_10 = [47902_u16,4768_u16,21528_u16,61058_u16,32567_u16];
_1 = -_8;
_4 = Move(_11);
_11 = Move(_4);
Goto(bb1)
}
bb1 = {
_5 = ['\u{e1004}'];
_3 = -_1;
_12.2 = [21918_u16];
_14 = 9223372036854775807_isize << 5_usize;
_3 = _8;
_12.1 = [10909_u16];
RET = -_6;
_6 = _3 * RET;
_4 = Move(_11);
_7 = 5_usize as f64;
_11 = Move(_4);
_8 = RET;
_4 = Move(_11);
_11 = Move(_4);
_3 = _8 + _6;
_6 = -RET;
_9 = ['\u{3ea3}'];
_3 = -_6;
_10 = [62849_u16,15891_u16,3512_u16,62962_u16,58368_u16];
_14 = 9223372036854775807_isize;
_1 = _6 * _8;
_12.1 = [26566_u16];
_1 = -_6;
_18.fld5 = Adt21::Variant0 { fld0: _14 };
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(Move(_10), Move(_14), _20, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: *mut i64,mut _3: Adt52,mut _4: char,mut _5: *const char,mut _6: *const i64) -> char {
mir! {
type RET = char;
let _7: u8;
let _8: &'static char;
let _9: *mut *mut u16;
let _10: f64;
let _11: &'static mut [u32; 3];
let _12: char;
let _13: &'static mut (*mut u16,);
let _14: [char; 1];
let _15: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _16: char;
let _17: bool;
let _18: (*mut u16,);
let _19: *const *const char;
let _20: [u32; 4];
let _21: u8;
let _22: &'static (f32, i8, i128);
let _23: f32;
let _24: i64;
let _25: Adt81;
let _26: &'static Adt26;
let _27: *mut *mut u16;
let _28: (*mut *mut u16, u128);
let _29: *const i64;
let _30: isize;
let _31: bool;
let _32: f32;
let _33: *const i64;
let _34: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _35: i16;
let _36: &'static mut u32;
let _37: i64;
let _38: u64;
let _39: i128;
let _40: &'static mut &'static mut u32;
let _41: &'static mut u32;
let _42: i8;
let _43: char;
let _44: f64;
let _45: u16;
let _46: *const &'static (f32, i8, i128);
let _47: isize;
let _48: u16;
let _49: &'static mut &'static mut u32;
let _50: [u8; 8];
let _51: f32;
let _52: u32;
let _53: &'static mut [u32; 3];
let _54: ([u16; 5], i64);
let _55: u64;
let _56: f32;
let _57: u8;
let _58: *const &'static (f32, i8, i128);
let _59: *const i64;
let _60: *mut &'static mut &'static mut u32;
let _61: *const char;
let _62: *mut &'static char;
let _63: &'static i16;
let _64: isize;
let _65: usize;
let _66: f32;
let _67: *mut i64;
let _68: bool;
let _69: *const &'static mut [u32; 3];
let _70: f64;
let _71: *const Adt21;
let _72: f64;
let _73: &'static mut (*mut u16,);
let _74: u8;
let _75: char;
let _76: Adt52;
let _77: &'static i16;
let _78: &'static mut isize;
let _79: [i16; 1];
let _80: char;
let _81: f64;
let _82: bool;
let _83: f32;
let _84: *const *const char;
let _85: *mut [i16; 1];
let _86: bool;
let _87: usize;
let _88: i64;
let _89: (&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32);
let _90: &'static (char, i64, Adt17);
let _91: Adt81;
let _92: ();
let _93: ();
{
_5 = core::ptr::addr_of!(RET);
(*_5) = _4;
_5 = core::ptr::addr_of!((*_5));
(*_5) = _4;
_1 = Field::<isize>(Variant(_3, 0), 0);
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_4 = (*_5);
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
Goto(bb1)
}
bb1 = {
_7 = 59_u8 + 176_u8;
(*_5) = _4;
place!(Field::<isize>(Variant(_3, 0), 0)) = 6763879686294512683_i64 as isize;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_8 = &(*_5);
_1 = Field::<isize>(Variant(_3, 0), 0) | Field::<isize>(Variant(_3, 0), 0);
(*_5) = _4;
place!(Field::<isize>(Variant(_3, 0), 0)) = 5_usize as isize;
_4 = (*_5);
_12 = RET;
(*_5) = _12;
(*_5) = _12;
(*_5) = _4;
(*_5) = _4;
_8 = &_4;
Goto(bb2)
}
bb2 = {
_1 = Field::<isize>(Variant(_3, 0), 0) >> _7;
(*_5) = (*_8);
(*_5) = (*_8);
(*_5) = (*_8);
(*_5) = (*_8);
place!(Field::<isize>(Variant(_3, 0), 0)) = _1;
(*_5) = (*_8);
_14 = [(*_5)];
(*_5) = (*_8);
_15.0.2 = [16987_u16];
_15.0.1 = [36630_u16];
_15.1 = (*_8) == (*_8);
Call((*_5) = fn12(Move(_2), (*_8), Move(_3), Move(_6), (*_8), Move(_8), (*_8), _15.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = 9223372036854775807_isize + (-62_isize);
(*_5) = _12;
_7 = (-59834924027478284575344276654988485872_i128) as u8;
(*_5) = _12;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_15.3 = &mut _1;
(*_5) = _4;
_10 = 12025418572701152969_u64 as f64;
_15.0.2 = [62149_u16];
_16 = _4;
_15.0.1 = [35481_u16];
_14 = [(*_5)];
_8 = &(*_5);
(*_5) = _4;
(*_5) = _12;
RET = _12;
_14 = [(*_5)];
(*_5) = _12;
_15.1 = (*_5) >= (*_5);
(*_5) = _16;
(*_5) = _16;
(*_5) = _16;
_3 = Adt52::Variant0 { fld0: 117_isize };
_15.1 = true;
(*_5) = _16;
Goto(bb4)
}
bb4 = {
_10 = _7 as f64;
_9 = core::ptr::addr_of_mut!(_18.0);
(*_5) = _12;
place!(Field::<isize>(Variant(_3, 0), 0)) = 35_isize;
_8 = &_16;
_19 = core::ptr::addr_of!(_5);
_21 = _7 ^ _7;
_12 = (*_8);
Goto(bb5)
}
bb5 = {
(*_5) = _12;
_9 = core::ptr::addr_of_mut!((*_9));
_15.3 = &mut place!(Field::<isize>(Variant(_3, 0), 0));
_9 = core::ptr::addr_of_mut!((*_9));
(*_19) = core::ptr::addr_of!((*_8));
(*_19) = core::ptr::addr_of!((*_5));
_19 = core::ptr::addr_of!((*_19));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_5);
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_8);
_16 = RET;
(*_19) = core::ptr::addr_of!(RET);
_4 = (*_5);
(*_5) = _12;
(*_19) = core::ptr::addr_of!((*_5));
_2 = core::ptr::addr_of_mut!(_24);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_2) = 2673768108395255991_i64 * (-3378677082840194307_i64);
_20 = [1225058017_u32,2620937502_u32,2091464876_u32,1864409239_u32];
(*_2) = (*_5) as i64;
_25.fld0 = _7 < _7;
(*_2) = 14853161902131065245_u64 as i64;
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
Goto(bb6)
}
bb6 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb7 = {
_28.0 = core::ptr::addr_of_mut!(_18.0);
(*_5) = _4;
(*_5) = _16;
_23 = (-9715385631484237822887297207142236668_i128) as f32;
_20 = [3592729127_u32,3422586711_u32,2708800684_u32,2682378039_u32];
(*_19) = core::ptr::addr_of!((*_5));
_9 = core::ptr::addr_of_mut!(_18.0);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_5) = _12;
(*_19) = core::ptr::addr_of!((*_5));
_28.1 = 1_usize as u128;
(*_5) = _16;
Goto(bb8)
}
bb8 = {
_6 = core::ptr::addr_of!((*_2));
_5 = core::ptr::addr_of!((*_5));
_24 = 10563_i16 as i64;
(*_5) = _12;
(*_6) = -(-4713334988697206284_i64);
(*_2) = 1062326536_u32 as i64;
_25.fld4.1 = _28.1 | _28.1;
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
(*_5) = _16;
_25.fld2.0 = _23 * _23;
_19 = core::ptr::addr_of!((*_19));
_28.1 = (*_2) as u128;
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!(_12);
(*_19) = core::ptr::addr_of!((*_5));
_28 = (Move(_9), _25.fld4.1);
Goto(bb9)
}
bb9 = {
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_14 = [(*_5)];
(*_19) = core::ptr::addr_of!((*_5));
_25.fld2.2 = (*_2);
(*_19) = core::ptr::addr_of!((*_5));
(*_2) = _25.fld2.2 | _25.fld2.2;
_2 = core::ptr::addr_of_mut!((*_2));
(*_5) = RET;
(*_5) = _4;
(*_2) = 36725842716917327610150643814671846993_i128 as i64;
(*_5) = _4;
_30 = (-67_isize);
_34.2.0 = _25.fld2.0 as i32;
_33 = core::ptr::addr_of!((*_2));
_28.1 = _25.fld4.1 & _25.fld4.1;
(*_2) = _25.fld2.2 | _25.fld2.2;
(*_19) = core::ptr::addr_of!((*_5));
_15.3 = &mut _30;
(*_5) = _12;
Goto(bb10)
}
bb10 = {
Goto(bb11)
}
bb11 = {
(*_2) = -_25.fld2.2;
(*_2) = -_25.fld2.2;
(*_19) = core::ptr::addr_of!((*_5));
(*_2) = -_25.fld2.2;
_17 = (*_2) < (*_2);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_5) = _4;
_25.fld1.0.0 = 24720_u16;
(*_2) = _25.fld2.2 & _25.fld2.2;
(*_2) = _25.fld2.2 * _25.fld2.2;
_5 = core::ptr::addr_of!((*_5));
(*_2) = _25.fld2.2 ^ _25.fld2.2;
_43 = (*_5);
_34.2.3 = 260816448_u32;
_25.fld2 = (_23, _10, (*_2));
_41 = &mut _34.2.3;
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
(*_41) = 1184039322_u32;
(*_5) = _4;
_25.fld2.2 = (*_2) & (*_2);
(*_19) = core::ptr::addr_of!((*_5));
match (*_41) {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
1184039322 => bb20,
_ => bb19
}
}
bb12 = {
Goto(bb11)
}
bb13 = {
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_14 = [(*_5)];
(*_19) = core::ptr::addr_of!((*_5));
_25.fld2.2 = (*_2);
(*_19) = core::ptr::addr_of!((*_5));
(*_2) = _25.fld2.2 | _25.fld2.2;
_2 = core::ptr::addr_of_mut!((*_2));
(*_5) = RET;
(*_5) = _4;
(*_2) = 36725842716917327610150643814671846993_i128 as i64;
(*_5) = _4;
_30 = (-67_isize);
_34.2.0 = _25.fld2.0 as i32;
_33 = core::ptr::addr_of!((*_2));
_28.1 = _25.fld4.1 & _25.fld4.1;
(*_2) = _25.fld2.2 | _25.fld2.2;
(*_19) = core::ptr::addr_of!((*_5));
_15.3 = &mut _30;
(*_5) = _12;
Goto(bb10)
}
bb14 = {
_1 = Field::<isize>(Variant(_3, 0), 0) >> _7;
(*_5) = (*_8);
(*_5) = (*_8);
(*_5) = (*_8);
(*_5) = (*_8);
place!(Field::<isize>(Variant(_3, 0), 0)) = _1;
(*_5) = (*_8);
_14 = [(*_5)];
(*_5) = (*_8);
_15.0.2 = [16987_u16];
_15.0.1 = [36630_u16];
_15.1 = (*_8) == (*_8);
Call((*_5) = fn12(Move(_2), (*_8), Move(_3), Move(_6), (*_8), Move(_8), (*_8), _15.1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_28.0 = core::ptr::addr_of_mut!(_18.0);
(*_5) = _4;
(*_5) = _16;
_23 = (-9715385631484237822887297207142236668_i128) as f32;
_20 = [3592729127_u32,3422586711_u32,2708800684_u32,2682378039_u32];
(*_19) = core::ptr::addr_of!((*_5));
_9 = core::ptr::addr_of_mut!(_18.0);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_5) = _12;
(*_19) = core::ptr::addr_of!((*_5));
_28.1 = 1_usize as u128;
(*_5) = _16;
Goto(bb8)
}
bb16 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb17 = {
_7 = 59_u8 + 176_u8;
(*_5) = _4;
place!(Field::<isize>(Variant(_3, 0), 0)) = 6763879686294512683_i64 as isize;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_8 = &(*_5);
_1 = Field::<isize>(Variant(_3, 0), 0) | Field::<isize>(Variant(_3, 0), 0);
(*_5) = _4;
place!(Field::<isize>(Variant(_3, 0), 0)) = 5_usize as isize;
_4 = (*_5);
_12 = RET;
(*_5) = _12;
(*_5) = _12;
(*_5) = _4;
(*_5) = _4;
_8 = &_4;
Goto(bb2)
}
bb18 = {
_10 = _7 as f64;
_9 = core::ptr::addr_of_mut!(_18.0);
(*_5) = _12;
place!(Field::<isize>(Variant(_3, 0), 0)) = 35_isize;
_8 = &_16;
_19 = core::ptr::addr_of!(_5);
_21 = _7 ^ _7;
_12 = (*_8);
Goto(bb5)
}
bb19 = {
_1 = 9223372036854775807_isize + (-62_isize);
(*_5) = _12;
_7 = (-59834924027478284575344276654988485872_i128) as u8;
(*_5) = _12;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_15.3 = &mut _1;
(*_5) = _4;
_10 = 12025418572701152969_u64 as f64;
_15.0.2 = [62149_u16];
_16 = _4;
_15.0.1 = [35481_u16];
_14 = [(*_5)];
_8 = &(*_5);
(*_5) = _4;
(*_5) = _12;
RET = _12;
_14 = [(*_5)];
(*_5) = _12;
_15.1 = (*_5) >= (*_5);
(*_5) = _16;
(*_5) = _16;
(*_5) = _16;
_3 = Adt52::Variant0 { fld0: 117_isize };
_15.1 = true;
(*_5) = _16;
Goto(bb4)
}
bb20 = {
(*_2) = 252995431_i32 as i64;
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_16 = _4;
_43 = (*_5);
_12 = (*_5);
(*_5) = _16;
(*_2) = -_25.fld2.2;
(*_41) = !760274660_u32;
_27 = core::ptr::addr_of_mut!(_18.0);
_6 = Move(_33);
(*_2) = _25.fld2.2;
(*_2) = _25.fld2.2 - _25.fld2.2;
(*_41) = !1248442501_u32;
_25.fld4.1 = _28.1 * _28.1;
(*_27) = core::ptr::addr_of_mut!(_25.fld1.0.0);
_18.0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_2) = _25.fld2.2 - _25.fld2.2;
_35 = (-5965_i16) << (*_2);
(*_5) = _12;
_44 = _25.fld2.1 - _10;
_28 = Move(_25.fld4);
match _25.fld1.0.0 {
24720 => bb22,
_ => bb21
}
}
bb21 = {
(*_5) = _12;
_9 = core::ptr::addr_of_mut!((*_9));
_15.3 = &mut place!(Field::<isize>(Variant(_3, 0), 0));
_9 = core::ptr::addr_of_mut!((*_9));
(*_19) = core::ptr::addr_of!((*_8));
(*_19) = core::ptr::addr_of!((*_5));
_19 = core::ptr::addr_of!((*_19));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_5);
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_8);
_16 = RET;
(*_19) = core::ptr::addr_of!(RET);
_4 = (*_5);
(*_5) = _12;
(*_19) = core::ptr::addr_of!((*_5));
_2 = core::ptr::addr_of_mut!(_24);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_2) = 2673768108395255991_i64 * (-3378677082840194307_i64);
_20 = [1225058017_u32,2620937502_u32,2091464876_u32,1864409239_u32];
(*_2) = (*_5) as i64;
_25.fld0 = _7 < _7;
(*_2) = 14853161902131065245_u64 as i64;
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
Goto(bb6)
}
bb22 = {
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
_15.2 = &mut _41;
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
_48 = _25.fld1.0.0;
_21 = !_7;
(*_19) = core::ptr::addr_of!((*_5));
_25.fld0 = (*_2) >= _24;
_28.0 = core::ptr::addr_of_mut!((*_27));
_25.fld0 = !_17;
(*_27) = core::ptr::addr_of_mut!(_48);
Goto(bb23)
}
bb23 = {
(*_19) = core::ptr::addr_of!((*_5));
(*_2) = _25.fld2.2 - _25.fld2.2;
(*_2) = 6_usize as i64;
(*_5) = RET;
_9 = core::ptr::addr_of_mut!(_18.0);
_33 = Move(_6);
(*_9) = core::ptr::addr_of_mut!(_48);
(*_27) = core::ptr::addr_of_mut!(_48);
_38 = 9832219498437605352_u64 * 5532451162234978753_u64;
_28 = (Move(_27), 202611523720056149160910625136977380791_u128);
match _25.fld1.0.0 {
0 => bb10,
1 => bb19,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
24720 => bb29,
_ => bb28
}
}
bb24 = {
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
_15.2 = &mut _41;
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
_48 = _25.fld1.0.0;
_21 = !_7;
(*_19) = core::ptr::addr_of!((*_5));
_25.fld0 = (*_2) >= _24;
_28.0 = core::ptr::addr_of_mut!((*_27));
_25.fld0 = !_17;
(*_27) = core::ptr::addr_of_mut!(_48);
Goto(bb23)
}
bb25 = {
_1 = 9223372036854775807_isize + (-62_isize);
(*_5) = _12;
_7 = (-59834924027478284575344276654988485872_i128) as u8;
(*_5) = _12;
(*_5) = _4;
(*_5) = _4;
(*_5) = _4;
_15.3 = &mut _1;
(*_5) = _4;
_10 = 12025418572701152969_u64 as f64;
_15.0.2 = [62149_u16];
_16 = _4;
_15.0.1 = [35481_u16];
_14 = [(*_5)];
_8 = &(*_5);
(*_5) = _4;
(*_5) = _12;
RET = _12;
_14 = [(*_5)];
(*_5) = _12;
_15.1 = (*_5) >= (*_5);
(*_5) = _16;
(*_5) = _16;
(*_5) = _16;
_3 = Adt52::Variant0 { fld0: 117_isize };
_15.1 = true;
(*_5) = _16;
Goto(bb4)
}
bb26 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb27 = {
(*_5) = _12;
_9 = core::ptr::addr_of_mut!((*_9));
_15.3 = &mut place!(Field::<isize>(Variant(_3, 0), 0));
_9 = core::ptr::addr_of_mut!((*_9));
(*_19) = core::ptr::addr_of!((*_8));
(*_19) = core::ptr::addr_of!((*_5));
_19 = core::ptr::addr_of!((*_19));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_5);
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_12 = (*_8);
_16 = RET;
(*_19) = core::ptr::addr_of!(RET);
_4 = (*_5);
(*_5) = _12;
(*_19) = core::ptr::addr_of!((*_5));
_2 = core::ptr::addr_of_mut!(_24);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_2) = 2673768108395255991_i64 * (-3378677082840194307_i64);
_20 = [1225058017_u32,2620937502_u32,2091464876_u32,1864409239_u32];
(*_2) = (*_5) as i64;
_25.fld0 = _7 < _7;
(*_2) = 14853161902131065245_u64 as i64;
(*_9) = core::ptr::addr_of_mut!(_25.fld1.0.0);
Goto(bb6)
}
bb28 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb29 = {
(*_19) = core::ptr::addr_of!((*_5));
_25.fld2.1 = _7 as f64;
(*_2) = _25.fld2.2 | _25.fld2.2;
(*_5) = RET;
_4 = RET;
(*_5) = RET;
(*_5) = RET;
_25.fld1.1.0 = RET;
(*_19) = core::ptr::addr_of!((*_5));
_25.fld1.1.0 = _4;
(*_19) = core::ptr::addr_of!((*_5));
_39 = 121877659683509427047756844161641192479_i128 >> (*_2);
_56 = _44 as f32;
(*_19) = core::ptr::addr_of!((*_5));
_55 = _38 << _35;
(*_5) = _4;
(*_2) = _10 as i64;
_25.fld0 = _17;
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_8 = &(*_5);
_25.fld4.0 = Move(_9);
_5 = core::ptr::addr_of!(_4);
(*_2) = !_25.fld2.2;
match _48 {
24720 => bb31,
_ => bb30
}
}
bb30 = {
Goto(bb11)
}
bb31 = {
_31 = _25.fld0 | _15.1;
(*_2) = _25.fld2.2 - _25.fld2.2;
_25.fld4 = (Move(_28.0), _28.1);
_25.fld2 = (_56, _44, (*_2));
_52 = 3393998873_u32 + 3522942112_u32;
_7 = !_21;
_59 = Move(_33);
(*_5) = (*_8);
_24 = _25.fld2.2;
_25.fld1.1.2 = Adt17::Variant1 { fld0: _21 };
_6 = core::ptr::addr_of!(_25.fld2.2);
(*_6) = (*_2) << (*_2);
(*_19) = core::ptr::addr_of!((*_5));
_42 = (-32_i8);
(*_5) = (*_8);
(*_2) = (*_6) ^ (*_6);
(*_5) = (*_8);
_36 = &mut _52;
(*_2) = (*_6) & (*_6);
_12 = (*_5);
(*_2) = (*_6) - (*_6);
_25.fld4.0 = core::ptr::addr_of_mut!(_18.0);
_50 = [Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_7,Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_21];
(*_5) = _16;
_15.1 = (*_6) >= (*_2);
Goto(bb32)
}
bb32 = {
(*_5) = _12;
_19 = core::ptr::addr_of!((*_19));
(*_19) = core::ptr::addr_of!(_12);
(*_2) = !(*_6);
_6 = core::ptr::addr_of!((*_2));
(*_2) = _25.fld2.2 - _25.fld2.2;
_54.1 = !(*_2);
_23 = _25.fld2.0;
_13 = &mut _18;
_43 = _4;
_28.0 = core::ptr::addr_of_mut!((*_13).0);
(*_13).0 = core::ptr::addr_of_mut!(_45);
Goto(bb33)
}
bb33 = {
(*_36) = 1703539042_u32 - 2641727134_u32;
(*_2) = _54.1 << (*_36);
_70 = _10;
_56 = _23;
(*_2) = _54.1;
(*_13).0 = core::ptr::addr_of_mut!(_48);
Goto(bb34)
}
bb34 = {
(*_36) = _35 as u32;
(*_13).0 = core::ptr::addr_of_mut!(_45);
(*_19) = core::ptr::addr_of!((*_8));
(*_13).0 = core::ptr::addr_of_mut!(_45);
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_57 = _7;
_20 = [(*_36),(*_36),(*_36),(*_36)];
(*_36) = 332372617_u32;
_56 = -_23;
_72 = _56 as f64;
_21 = _57;
_15.0.2 = [_48];
_10 = _25.fld2.1;
_61 = core::ptr::addr_of!((*_5));
_25.fld4 = Move(_28);
_7 = _25.fld4.1 as u8;
_48 = _25.fld1.0.0 >> (*_2);
_37 = _24 | (*_2);
_19 = core::ptr::addr_of!((*_19));
_67 = Move(_2);
_25.fld1.1.1 = _24 - _25.fld2.2;
_25.fld1.0 = (_48,);
(*_36) = 3398712239012758476_usize as u32;
(*_36) = !3459222837_u32;
(*_13).0 = core::ptr::addr_of_mut!(_48);
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_32 = _56;
(*_13).0 = core::ptr::addr_of_mut!(_48);
Goto(bb35)
}
bb35 = {
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_59 = core::ptr::addr_of!(_25.fld1.1.1);
(*_59) = !_37;
_21 = _57 ^ _7;
_64 = -(-74_isize);
_25.fld4.0 = core::ptr::addr_of_mut!((*_13).0);
(*_59) = _25.fld2.2 ^ _24;
_68 = _15.1;
(*_59) = _37 + _37;
_47 = !_64;
(*_59) = -_37;
_63 = &_35;
_49 = &mut _36;
(*_13).0 = core::ptr::addr_of_mut!(_48);
_7 = !_21;
Goto(bb36)
}
bb36 = {
_74 = !_57;
_17 = (*_59) < _24;
_10 = _72;
_24 = (*_59) * (*_59);
_19 = core::ptr::addr_of!((*_19));
(*_13).0 = core::ptr::addr_of_mut!(_48);
_15.2 = &mut (*_49);
_37 = -_24;
_25.fld2.0 = _23;
place!(Field::<u8>(Variant(_25.fld1.1.2, 1), 0)) = 4_usize as u8;
_2 = core::ptr::addr_of_mut!((*_59));
(*_19) = core::ptr::addr_of!((*_8));
_65 = 6_usize;
(*_2) = _25.fld4.1 as i64;
_66 = _25.fld2.0 - _25.fld2.0;
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_39 = (-54225224897582107967373791925403490912_i128) * (-49959610997882278990634508574308444029_i128);
(*_19) = core::ptr::addr_of!((*_5));
_25.fld1.1.0 = _12;
_40 = Move(_15.2);
_76 = Adt52::Variant0 { fld0: _64 };
(*_59) = _37 << (*_63);
_2 = core::ptr::addr_of_mut!((*_59));
(*_2) = _24 | _54.1;
_27 = core::ptr::addr_of_mut!((*_13).0);
_15.1 = (*_59) != _37;
match _65 {
0 => bb27,
1 => bb14,
6 => bb38,
_ => bb37
}
}
bb37 = {
_31 = _25.fld0 | _15.1;
(*_2) = _25.fld2.2 - _25.fld2.2;
_25.fld4 = (Move(_28.0), _28.1);
_25.fld2 = (_56, _44, (*_2));
_52 = 3393998873_u32 + 3522942112_u32;
_7 = !_21;
_59 = Move(_33);
(*_5) = (*_8);
_24 = _25.fld2.2;
_25.fld1.1.2 = Adt17::Variant1 { fld0: _21 };
_6 = core::ptr::addr_of!(_25.fld2.2);
(*_6) = (*_2) << (*_2);
(*_19) = core::ptr::addr_of!((*_5));
_42 = (-32_i8);
(*_5) = (*_8);
(*_2) = (*_6) ^ (*_6);
(*_5) = (*_8);
_36 = &mut _52;
(*_2) = (*_6) & (*_6);
_12 = (*_5);
(*_2) = (*_6) - (*_6);
_25.fld4.0 = core::ptr::addr_of_mut!(_18.0);
_50 = [Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_7,Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_21];
(*_5) = _16;
_15.1 = (*_6) >= (*_2);
Goto(bb32)
}
bb38 = {
_38 = _64 as u64;
(*_27) = core::ptr::addr_of_mut!(_45);
(*_27) = core::ptr::addr_of_mut!(_25.fld1.0.0);
(*_2) = _37 + _37;
_66 = _25.fld2.0;
(*_13).0 = core::ptr::addr_of_mut!(_48);
_77 = &(*_63);
_25.fld5 = Move(_76);
_23 = _56 - _32;
_28 = Move(_25.fld4);
(*_19) = core::ptr::addr_of!(_75);
(*_59) = !_37;
_59 = core::ptr::addr_of!((*_59));
_56 = _23 + _32;
_15.2 = Move(_49);
match _28.1 {
0 => bb21,
1 => bb32,
2 => bb23,
3 => bb7,
4 => bb30,
5 => bb39,
6 => bb40,
202611523720056149160910625136977380791 => bb42,
_ => bb41
}
}
bb39 = {
(*_36) = _35 as u32;
(*_13).0 = core::ptr::addr_of_mut!(_45);
(*_19) = core::ptr::addr_of!((*_8));
(*_13).0 = core::ptr::addr_of_mut!(_45);
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_57 = _7;
_20 = [(*_36),(*_36),(*_36),(*_36)];
(*_36) = 332372617_u32;
_56 = -_23;
_72 = _56 as f64;
_21 = _57;
_15.0.2 = [_48];
_10 = _25.fld2.1;
_61 = core::ptr::addr_of!((*_5));
_25.fld4 = Move(_28);
_7 = _25.fld4.1 as u8;
_48 = _25.fld1.0.0 >> (*_2);
_37 = _24 | (*_2);
_19 = core::ptr::addr_of!((*_19));
_67 = Move(_2);
_25.fld1.1.1 = _24 - _25.fld2.2;
_25.fld1.0 = (_48,);
(*_36) = 3398712239012758476_usize as u32;
(*_36) = !3459222837_u32;
(*_13).0 = core::ptr::addr_of_mut!(_48);
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_32 = _56;
(*_13).0 = core::ptr::addr_of_mut!(_48);
Goto(bb35)
}
bb40 = {
(*_19) = core::ptr::addr_of!((*_5));
_25.fld2.1 = _7 as f64;
(*_2) = _25.fld2.2 | _25.fld2.2;
(*_5) = RET;
_4 = RET;
(*_5) = RET;
(*_5) = RET;
_25.fld1.1.0 = RET;
(*_19) = core::ptr::addr_of!((*_5));
_25.fld1.1.0 = _4;
(*_19) = core::ptr::addr_of!((*_5));
_39 = 121877659683509427047756844161641192479_i128 >> (*_2);
_56 = _44 as f32;
(*_19) = core::ptr::addr_of!((*_5));
_55 = _38 << _35;
(*_5) = _4;
(*_2) = _10 as i64;
_25.fld0 = _17;
(*_19) = core::ptr::addr_of!((*_5));
(*_19) = core::ptr::addr_of!((*_5));
_8 = &(*_5);
_25.fld4.0 = Move(_9);
_5 = core::ptr::addr_of!(_4);
(*_2) = !_25.fld2.2;
match _48 {
24720 => bb31,
_ => bb30
}
}
bb41 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb42 = {
(*_59) = Field::<u8>(Variant(_25.fld1.1.2, 1), 0) as i64;
_19 = core::ptr::addr_of!((*_19));
_84 = core::ptr::addr_of!((*_19));
(*_13).0 = core::ptr::addr_of_mut!(_48);
_82 = _15.1 ^ _68;
(*_5) = RET;
Goto(bb43)
}
bb43 = {
(*_5) = RET;
_14 = [(*_5)];
match _42 {
0 => bb16,
1 => bb44,
340282366920938463463374607431768211424 => bb46,
_ => bb45
}
}
bb44 = {
_74 = !_57;
_17 = (*_59) < _24;
_10 = _72;
_24 = (*_59) * (*_59);
_19 = core::ptr::addr_of!((*_19));
(*_13).0 = core::ptr::addr_of_mut!(_48);
_15.2 = &mut (*_49);
_37 = -_24;
_25.fld2.0 = _23;
place!(Field::<u8>(Variant(_25.fld1.1.2, 1), 0)) = 4_usize as u8;
_2 = core::ptr::addr_of_mut!((*_59));
(*_19) = core::ptr::addr_of!((*_8));
_65 = 6_usize;
(*_2) = _25.fld4.1 as i64;
_66 = _25.fld2.0 - _25.fld2.0;
(*_13).0 = core::ptr::addr_of_mut!(_25.fld1.0.0);
_39 = (-54225224897582107967373791925403490912_i128) * (-49959610997882278990634508574308444029_i128);
(*_19) = core::ptr::addr_of!((*_5));
_25.fld1.1.0 = _12;
_40 = Move(_15.2);
_76 = Adt52::Variant0 { fld0: _64 };
(*_59) = _37 << (*_63);
_2 = core::ptr::addr_of_mut!((*_59));
(*_2) = _24 | _54.1;
_27 = core::ptr::addr_of_mut!((*_13).0);
_15.1 = (*_59) != _37;
match _65 {
0 => bb27,
1 => bb14,
6 => bb38,
_ => bb37
}
}
bb45 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb46 = {
_35 = (-31200_i16) | 14361_i16;
_64 = Field::<isize>(Variant(_25.fld5, 0), 0);
_32 = _35 as f32;
(*_13).0 = core::ptr::addr_of_mut!(_48);
_28.0 = core::ptr::addr_of_mut!((*_13).0);
(*_5) = _16;
_45 = _25.fld2.1 as u16;
_73 = &mut (*_13);
(*_84) = core::ptr::addr_of!((*_5));
_85 = core::ptr::addr_of_mut!(_79);
(*_5) = _25.fld1.1.0;
(*_84) = Move(_61);
(*_19) = core::ptr::addr_of!(RET);
(*_19) = core::ptr::addr_of!((*_5));
(*_85) = [_35];
_7 = Field::<u8>(Variant(_25.fld1.1.2, 1), 0);
_85 = core::ptr::addr_of_mut!((*_85));
_25.fld4.1 = !_28.1;
_51 = _21 as f32;
_15.0.2 = [_25.fld1.0.0];
match _28.1 {
0 => bb42,
1 => bb18,
2 => bb13,
3 => bb44,
4 => bb5,
5 => bb29,
6 => bb47,
202611523720056149160910625136977380791 => bb49,
_ => bb48
}
}
bb47 = {
_25.fld1.0.0 = 16052_i16 as u16;
_12 = (*_5);
_25.fld4.0 = Move(_9);
(*_19) = core::ptr::addr_of!((*_5));
(*_5) = _4;
(*_19) = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb48 = {
_31 = _25.fld0 | _15.1;
(*_2) = _25.fld2.2 - _25.fld2.2;
_25.fld4 = (Move(_28.0), _28.1);
_25.fld2 = (_56, _44, (*_2));
_52 = 3393998873_u32 + 3522942112_u32;
_7 = !_21;
_59 = Move(_33);
(*_5) = (*_8);
_24 = _25.fld2.2;
_25.fld1.1.2 = Adt17::Variant1 { fld0: _21 };
_6 = core::ptr::addr_of!(_25.fld2.2);
(*_6) = (*_2) << (*_2);
(*_19) = core::ptr::addr_of!((*_5));
_42 = (-32_i8);
(*_5) = (*_8);
(*_2) = (*_6) ^ (*_6);
(*_5) = (*_8);
_36 = &mut _52;
(*_2) = (*_6) & (*_6);
_12 = (*_5);
(*_2) = (*_6) - (*_6);
_25.fld4.0 = core::ptr::addr_of_mut!(_18.0);
_50 = [Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_7,Field::<u8>(Variant(_25.fld1.1.2, 1), 0),_7,_21,_21];
(*_5) = _16;
_15.1 = (*_6) >= (*_2);
Goto(bb32)
}
bb49 = {
_15.0.1 = [_25.fld1.0.0];
_20 = [3259351622_u32,864685110_u32,3699281435_u32,113042284_u32];
_81 = _47 as f64;
(*_19) = core::ptr::addr_of!(_25.fld1.1.0);
_21 = _50[_65] - _57;
place!(Field::<isize>(Variant(_25.fld5, 0), 0)) = -_47;
(*_59) = _24;
(*_73).0 = core::ptr::addr_of_mut!(_48);
_42 = -41_i8;
(*_59) = !_24;
_25.fld2.1 = _44;
_67 = core::ptr::addr_of_mut!((*_59));
(*_19) = core::ptr::addr_of!(_75);
Goto(bb50)
}
bb50 = {
Call(_92 = dump_var(Move(_21), Move(_64), Move(_57), Move(_52)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_92 = dump_var(Move(_30), Move(_20), Move(_38), Move(_43)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_92 = dump_var(Move(_74), Move(_12), Move(_55), Move(_16)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_92 = dump_var(Move(_1), Move(_50), Move(_79), Move(_14)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: *mut i64,mut _2: char,mut _3: Adt52,mut _4: *const i64,mut _5: char,mut _6: &'static char,mut _7: char,mut _8: bool) -> char {
mir! {
type RET = char;
let _9: Adt52;
let _10: bool;
let _11: char;
let _12: bool;
let _13: *const char;
let _14: f64;
let _15: *mut u16;
let _16: [u16; 1];
let _17: &'static mut isize;
let _18: &'static (char, i64, Adt17);
let _19: u32;
let _20: &'static i16;
let _21: char;
let _22: char;
let _23: i128;
let _24: [u16; 5];
let _25: (u16,);
let _26: char;
let _27: f64;
let _28: f32;
let _29: Adt81;
let _30: isize;
let _31: &'static char;
let _32: *mut Adt52;
let _33: i8;
let _34: *mut [i16; 1];
let _35: (*mut u16, &'static (char, i64, Adt17));
let _36: f32;
let _37: &'static mut isize;
let _38: u16;
let _39: i16;
let _40: *const *const char;
let _41: i64;
let _42: isize;
let _43: &'static mut u32;
let _44: i32;
let _45: [u32; 6];
let _46: bool;
let _47: f32;
let _48: &'static [i128; 8];
let _49: bool;
let _50: isize;
let _51: bool;
let _52: *const &'static mut [u32; 3];
let _53: isize;
let _54: isize;
let _55: *mut i64;
let _56: &'static mut (*mut u16,);
let _57: *const *const char;
let _58: isize;
let _59: isize;
let _60: &'static mut *const *const char;
let _61: f32;
let _62: Adt21;
let _63: [u8; 7];
let _64: [u8; 8];
let _65: usize;
let _66: *const char;
let _67: &'static Adt26;
let _68: Adt21;
let _69: Adt55;
let _70: Adt26;
let _71: *mut i32;
let _72: isize;
let _73: [i16; 1];
let _74: ([u16; 5], i64);
let _75: (*mut *mut u16, u128);
let _76: *mut Adt52;
let _77: isize;
let _78: Adt55;
let _79: *mut u16;
let _80: *mut i64;
let _81: f64;
let _82: *mut Adt52;
let _83: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _84: Adt17;
let _85: [char; 1];
let _86: *mut i64;
let _87: isize;
let _88: u64;
let _89: bool;
let _90: ();
let _91: ();
{
RET = _5;
_7 = _2;
_8 = RET > _2;
_2 = _7;
_5 = _2;
_5 = RET;
place!(Field::<isize>(Variant(_3, 0), 0)) = 9223372036854775807_isize >> (-22_i8);
_8 = RET < _5;
_9 = Adt52::Variant0 { fld0: Field::<isize>(Variant(_3, 0), 0) };
_8 = false;
place!(Field::<isize>(Variant(_3, 0), 0)) = (-97720605219669898409668632294870435603_i128) as isize;
_3 = Move(_9);
_6 = &RET;
_2 = (*_6);
_9 = Move(_3);
place!(Field::<isize>(Variant(_9, 0), 0)) = 3_isize | (-9223372036854775808_isize);
_3 = Adt52::Variant0 { fld0: Field::<isize>(Variant(_9, 0), 0) };
_3 = Move(_9);
place!(Field::<isize>(Variant(_3, 0), 0)) = 2050350131_u32 as isize;
_9 = Move(_3);
_3 = Move(_9);
_2 = _7;
_2 = (*_6);
Call(_2 = fn13((*_6), (*_6), Field::<isize>(Variant(_3, 0), 0), (*_6), (*_6), Move(_3), (*_6)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb2 = {
(*_13) = _5;
_12 = !_8;
(*_13) = _7;
place!(Field::<isize>(Variant(_3, 0), 0)) = 40869172383598904678999488440246988097_u128 as isize;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
(*_13) = _7;
(*_13) = _5;
_3 = Move(_9);
(*_13) = _7;
(*_13) = RET;
(*_13) = _5;
_7 = (*_13);
_16 = [367_u16];
(*_13) = _7;
_8 = !_12;
_14 = (-17_i8) as f64;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
_11 = (*_13);
Goto(bb3)
}
bb3 = {
(*_13) = RET;
(*_13) = _7;
_16 = [62223_u16];
_6 = &(*_13);
(*_13) = _11;
RET = (*_13);
_8 = !_12;
_7 = (*_13);
(*_13) = RET;
(*_13) = _5;
(*_13) = _7;
(*_13) = _7;
_22 = _2;
_6 = &RET;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
(*_13) = _22;
(*_13) = (*_6);
_23 = (-49783611509215622604813325762153057758_i128) << (-1027888384_i32);
_13 = core::ptr::addr_of!((*_13));
(*_13) = (*_6);
_9 = Adt52::Variant0 { fld0: 94_isize };
Goto(bb4)
}
bb4 = {
(*_13) = (*_6);
_15 = core::ptr::addr_of_mut!(_25.0);
(*_15) = !35899_u16;
(*_15) = 49190_u16;
_8 = !_12;
(*_15) = !20907_u16;
(*_15) = 36541_u16 - 47111_u16;
(*_13) = (*_6);
RET = _7;
(*_13) = _11;
(*_13) = _7;
(*_13) = RET;
RET = (*_13);
(*_13) = _7;
(*_13) = _5;
_25 = (50782_u16,);
(*_15) = 35633_u16 << _23;
(*_15) = 328_u16 << _23;
(*_15) = 60855_u16 | 39777_u16;
Goto(bb5)
}
bb5 = {
_15 = core::ptr::addr_of_mut!((*_15));
_29.fld4.1 = 274873624505435047808571785452993127290_u128 * 156507761182751477976572878348508546447_u128;
(*_13) = _7;
_12 = (*_15) < (*_15);
_29.fld1.0.0 = (*_15);
(*_13) = _7;
_28 = 430026108622557275_u64 as f32;
(*_13) = _5;
_29.fld2 = (_28, _14, (-2748432223949134666_i64));
_26 = (*_13);
(*_15) = _29.fld1.0.0 - _29.fld1.0.0;
_29.fld4.0 = core::ptr::addr_of_mut!(_15);
_29.fld2 = (_28, _14, 7333122324958586836_i64);
_10 = _8 ^ _12;
_29.fld1.1.0 = (*_13);
_15 = core::ptr::addr_of_mut!((*_15));
_14 = _29.fld2.1 - _29.fld2.1;
_29.fld4.1 = 156333306741974299527940186240669636368_u128;
match _29.fld2.2 {
7333122324958586836 => bb7,
_ => bb6
}
}
bb6 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb7 = {
(*_15) = _29.fld1.0.0 | _29.fld1.0.0;
_27 = -_29.fld2.1;
_13 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb8 = {
(*_13) = _5;
_36 = _28 * _28;
(*_15) = _29.fld1.0.0 - _29.fld1.0.0;
_19 = !880684900_u32;
_33 = 3_i8;
(*_13) = _22;
(*_13) = _11;
_2 = (*_13);
(*_13) = _22;
(*_15) = _28 as u16;
(*_13) = _29.fld1.1.0;
(*_13) = _26;
_35.0 = core::ptr::addr_of_mut!((*_15));
place!(Field::<isize>(Variant(_9, 0), 0)) = 742807884_i32 as isize;
(*_15) = _29.fld1.0.0 ^ _29.fld1.0.0;
_29.fld2.0 = _36;
_35.0 = core::ptr::addr_of_mut!((*_15));
_29.fld2.1 = -_27;
_7 = RET;
_6 = &_11;
_19 = 268321170_u32;
_29.fld1.1.2 = Adt17::Variant1 { fld0: 23_u8 };
match _29.fld2.2 {
0 => bb9,
7333122324958586836 => bb11,
_ => bb10
}
}
bb9 = {
_15 = core::ptr::addr_of_mut!((*_15));
_29.fld4.1 = 274873624505435047808571785452993127290_u128 * 156507761182751477976572878348508546447_u128;
(*_13) = _7;
_12 = (*_15) < (*_15);
_29.fld1.0.0 = (*_15);
(*_13) = _7;
_28 = 430026108622557275_u64 as f32;
(*_13) = _5;
_29.fld2 = (_28, _14, (-2748432223949134666_i64));
_26 = (*_13);
(*_15) = _29.fld1.0.0 - _29.fld1.0.0;
_29.fld4.0 = core::ptr::addr_of_mut!(_15);
_29.fld2 = (_28, _14, 7333122324958586836_i64);
_10 = _8 ^ _12;
_29.fld1.1.0 = (*_13);
_15 = core::ptr::addr_of_mut!((*_15));
_14 = _29.fld2.1 - _29.fld2.1;
_29.fld4.1 = 156333306741974299527940186240669636368_u128;
match _29.fld2.2 {
7333122324958586836 => bb7,
_ => bb6
}
}
bb10 = {
(*_13) = (*_6);
_15 = core::ptr::addr_of_mut!(_25.0);
(*_15) = !35899_u16;
(*_15) = 49190_u16;
_8 = !_12;
(*_15) = !20907_u16;
(*_15) = 36541_u16 - 47111_u16;
(*_13) = (*_6);
RET = _7;
(*_13) = _11;
(*_13) = _7;
(*_13) = RET;
RET = (*_13);
(*_13) = _7;
(*_13) = _5;
_25 = (50782_u16,);
(*_15) = 35633_u16 << _23;
(*_15) = 328_u16 << _23;
(*_15) = 60855_u16 | 39777_u16;
Goto(bb5)
}
bb11 = {
(*_15) = _29.fld1.0.0 + _29.fld1.0.0;
_38 = (*_15) + (*_15);
_36 = -_29.fld2.0;
_21 = (*_6);
match _19 {
0 => bb1,
268321170 => bb12,
_ => bb9
}
}
bb12 = {
_30 = !Field::<isize>(Variant(_9, 0), 0);
_28 = -_36;
(*_15) = _29.fld1.0.0;
_40 = core::ptr::addr_of!(_13);
(*_15) = _29.fld1.0.0 << _38;
(*_13) = (*_6);
(*_13) = (*_6);
_29.fld1.1.0 = _21;
_1 = core::ptr::addr_of_mut!(_29.fld1.1.1);
(*_1) = _29.fld2.2 ^ _29.fld2.2;
_43 = &mut _19;
_41 = (*_1) * (*_1);
(*_40) = core::ptr::addr_of!((*_13));
(*_15) = !_38;
(*_1) = _41 | _41;
(*_15) = _38;
(*_13) = (*_6);
Goto(bb13)
}
bb13 = {
_29.fld4.1 = _23 as u128;
_3 = Move(_9);
_45 = [(*_43),(*_43),(*_43),(*_43),(*_43),(*_43)];
_26 = _2;
(*_43) = 87870150_u32 | 2840969243_u32;
_29.fld1.1.2 = Adt17::Variant3 { fld0: _29.fld4.1,fld1: (*_13),fld2: (*_43),fld3: _14,fld4: 2310710488839112016_u64,fld5: 1790234595_i32,fld6: (*_1),fld7: _23 };
_27 = -_29.fld2.1;
(*_40) = core::ptr::addr_of!((*_13));
place!(Field::<i32>(Variant(_29.fld1.1.2, 3), 5)) = -1997129306_i32;
_23 = Field::<i128>(Variant(_29.fld1.1.2, 3), 7) + Field::<i128>(Variant(_29.fld1.1.2, 3), 7);
(*_13) = Field::<char>(Variant(_29.fld1.1.2, 3), 1);
_4 = core::ptr::addr_of!((*_1));
_24 = [(*_15),(*_15),(*_15),(*_15),_38];
(*_43) = 29796_i16 as u32;
_27 = _33 as f64;
(*_13) = (*_6);
(*_4) = _41 | Field::<i64>(Variant(_29.fld1.1.2, 3), 6);
(*_15) = _38;
(*_40) = core::ptr::addr_of!((*_6));
_22 = (*_13);
_9 = Move(_3);
(*_15) = _38 & _38;
_29.fld5 = Move(_9);
match _29.fld2.2 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb14,
7333122324958586836 => bb16,
_ => bb15
}
}
bb14 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb15 = {
(*_13) = (*_6);
_15 = core::ptr::addr_of_mut!(_25.0);
(*_15) = !35899_u16;
(*_15) = 49190_u16;
_8 = !_12;
(*_15) = !20907_u16;
(*_15) = 36541_u16 - 47111_u16;
(*_13) = (*_6);
RET = _7;
(*_13) = _11;
(*_13) = _7;
(*_13) = RET;
RET = (*_13);
(*_13) = _7;
(*_13) = _5;
_25 = (50782_u16,);
(*_15) = 35633_u16 << _23;
(*_15) = 328_u16 << _23;
(*_15) = 60855_u16 | 39777_u16;
Goto(bb5)
}
bb16 = {
(*_43) = Field::<u32>(Variant(_29.fld1.1.2, 3), 2) >> (*_1);
place!(Field::<u32>(Variant(_29.fld1.1.2, 3), 2)) = (*_43);
(*_43) = Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
(*_43) = Field::<u32>(Variant(_29.fld1.1.2, 3), 2) * Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
_16 = [(*_15)];
(*_1) = _41 | _41;
(*_15) = _38;
_29.fld1.1.1 = _29.fld2.2;
_45 = [(*_43),(*_43),(*_43),(*_43),(*_43),(*_43)];
place!(Field::<i128>(Variant(_29.fld1.1.2, 3), 7)) = 0_usize as i128;
(*_43) = _23 as u32;
(*_15) = !_38;
_16 = [(*_15)];
_35.0 = Move(_15);
_49 = _12;
_17 = &mut _30;
(*_43) = !Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
(*_40) = core::ptr::addr_of!(place!(Field::<char>(Variant(_29.fld1.1.2, 3), 1)));
(*_13) = (*_6);
(*_43) = _33 as u32;
_25 = (_38,);
(*_40) = core::ptr::addr_of!((*_6));
_54 = (*_17);
_29.fld1.1.0 = (*_6);
(*_43) = Field::<u32>(Variant(_29.fld1.1.2, 3), 2) + Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
_39 = -(-9045_i16);
match (*_1) {
7333122324958586836 => bb17,
_ => bb13
}
}
bb17 = {
_33 = _29.fld1.0.0 as i8;
(*_40) = core::ptr::addr_of!((*_13));
(*_40) = core::ptr::addr_of!(_2);
(*_17) = Field::<isize>(Variant(_29.fld5, 0), 0) >> (*_43);
Call((*_1) = core::intrinsics::transmute(_29.fld2.2), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_17) = Field::<i64>(Variant(_29.fld1.1.2, 3), 6) as isize;
_53 = !(*_17);
_26 = (*_6);
_25.0 = _38;
(*_17) = (*_13) as isize;
(*_17) = _53 | _53;
(*_43) = !Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
(*_43) = !Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
(*_17) = _39 as isize;
_12 = !_10;
_11 = (*_13);
(*_13) = _11;
(*_40) = core::ptr::addr_of!((*_13));
_29.fld2.0 = _36 * _28;
(*_17) = !_53;
_50 = -(*_17);
(*_40) = core::ptr::addr_of!(_29.fld1.1.0);
match _29.fld2.2 {
0 => bb15,
1 => bb16,
2 => bb17,
3 => bb8,
7333122324958586836 => bb19,
_ => bb12
}
}
bb19 = {
_36 = _29.fld2.0;
_15 = core::ptr::addr_of_mut!(_38);
_9 = Move(_29.fld5);
_35.0 = core::ptr::addr_of_mut!((*_15));
_11 = (*_13);
Goto(bb20)
}
bb20 = {
_8 = !_49;
_60 = &mut _40;
_17 = &mut _50;
(*_15) = _29.fld1.0.0;
(*_1) = Field::<i64>(Variant(_29.fld1.1.2, 3), 6) << (*_15);
_29.fld0 = (*_17) <= (*_17);
(*_15) = !_29.fld1.0.0;
(*_17) = _53 << (*_15);
_29.fld1.1.1 = !Field::<i64>(Variant(_29.fld1.1.2, 3), 6);
(*_1) = Field::<i64>(Variant(_29.fld1.1.2, 3), 6) - Field::<i64>(Variant(_29.fld1.1.2, 3), 6);
(*_17) = !_53;
_44 = Field::<i32>(Variant(_29.fld1.1.2, 3), 5) << (*_43);
_1 = core::ptr::addr_of_mut!((*_1));
(*_13) = _2;
Goto(bb21)
}
bb21 = {
place!(Field::<f64>(Variant(_29.fld1.1.2, 3), 3)) = 15992251743138260269_u64 as f64;
(*_1) = _41;
Goto(bb22)
}
bb22 = {
_16 = [(*_15)];
_7 = (*_13);
_2 = (*_13);
_44 = 1_usize as i32;
_59 = (*_17) * (*_17);
Goto(bb23)
}
bb23 = {
_42 = (*_17) ^ (*_17);
(*_13) = _5;
_53 = -_54;
_49 = !_29.fld0;
_29.fld5 = Move(_9);
_42 = (*_17) - (*_17);
(*_17) = _59 ^ _42;
(*_43) = !Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
(*_60) = core::ptr::addr_of!(_13);
(*_1) = -_41;
(*_17) = _42;
(*_15) = _25.0;
_39 = (-2457_i16);
(*_15) = _25.0 + _29.fld1.0.0;
(*_13) = Field::<char>(Variant(_29.fld1.1.2, 3), 1);
place!(Field::<i32>(Variant(_29.fld1.1.2, 3), 5)) = _44 + _44;
_29.fld2.0 = Field::<u128>(Variant(_29.fld1.1.2, 3), 0) as f32;
_29.fld1.1.0 = _7;
match _29.fld2.2 {
0 => bb1,
1 => bb8,
2 => bb18,
3 => bb24,
7333122324958586836 => bb26,
_ => bb25
}
}
bb24 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb25 = {
(*_15) = _29.fld1.0.0 | _29.fld1.0.0;
_27 = -_29.fld2.1;
_13 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb26 = {
(*_13) = _5;
place!(Field::<u128>(Variant(_29.fld1.1.2, 3), 0)) = _29.fld4.1 + _29.fld4.1;
_9 = Adt52::Variant0 { fld0: (*_17) };
place!(Field::<char>(Variant(_29.fld1.1.2, 3), 1)) = (*_13);
_29.fld1.0.0 = !(*_15);
(*_43) = !Field::<u32>(Variant(_29.fld1.1.2, 3), 2);
_43 = &mut place!(Field::<u32>(Variant(_29.fld1.1.2, 3), 2));
(*_60) = core::ptr::addr_of!(_13);
_66 = Move(_13);
_23 = (-118426104239563340133634867338124246009_i128) * (-25882757352927326135547110948661090000_i128);
_38 = !_25.0;
(*_1) = _23 as i64;
_22 = _2;
_33 = _14 as i8;
(*_15) = _25.0;
(*_60) = core::ptr::addr_of!(_66);
_70.fld2 = ((*_15),);
_58 = _59 & (*_17);
(*_1) = -_41;
RET = _7;
match _39 {
0 => bb27,
1 => bb28,
2 => bb29,
340282366920938463463374607431768208999 => bb31,
_ => bb30
}
}
bb27 = {
_33 = _29.fld1.0.0 as i8;
(*_40) = core::ptr::addr_of!((*_13));
(*_40) = core::ptr::addr_of!(_2);
(*_17) = Field::<isize>(Variant(_29.fld5, 0), 0) >> (*_43);
Call((*_1) = core::intrinsics::transmute(_29.fld2.2), ReturnTo(bb18), UnwindUnreachable())
}
bb28 = {
(*_13) = (*_6);
_15 = core::ptr::addr_of_mut!(_25.0);
(*_15) = !35899_u16;
(*_15) = 49190_u16;
_8 = !_12;
(*_15) = !20907_u16;
(*_15) = 36541_u16 - 47111_u16;
(*_13) = (*_6);
RET = _7;
(*_13) = _11;
(*_13) = _7;
(*_13) = RET;
RET = (*_13);
(*_13) = _7;
(*_13) = _5;
_25 = (50782_u16,);
(*_15) = 35633_u16 << _23;
(*_15) = 328_u16 << _23;
(*_15) = 60855_u16 | 39777_u16;
Goto(bb5)
}
bb29 = {
(*_15) = _29.fld1.0.0 | _29.fld1.0.0;
_27 = -_29.fld2.1;
_13 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb30 = {
(*_13) = _5;
_12 = !_8;
(*_13) = _7;
place!(Field::<isize>(Variant(_3, 0), 0)) = 40869172383598904678999488440246988097_u128 as isize;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
(*_13) = _7;
(*_13) = _5;
_3 = Move(_9);
(*_13) = _7;
(*_13) = RET;
(*_13) = _5;
_7 = (*_13);
_16 = [367_u16];
(*_13) = _7;
_8 = !_12;
_14 = (-17_i8) as f64;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
_11 = (*_13);
Goto(bb3)
}
bb31 = {
_12 = !_49;
_37 = &mut (*_17);
(*_1) = _41;
(*_37) = _58 >> (*_15);
_70.fld1 = (*_15);
match _39 {
340282366920938463463374607431768208999 => bb32,
_ => bb15
}
}
bb32 = {
_70.fld5 = Adt21::Variant1 { fld0: (*_1),fld1: 7_usize,fld2: _70.fld2,fld3: 1_u8 };
_36 = _28 * _28;
(*_43) = 1314233469_u32;
(*_60) = core::ptr::addr_of!(_66);
_47 = _36 - _28;
_70.fld0 = !9109120350746443706_u64;
_55 = Move(_1);
(*_43) = !267163398_u32;
_70.fld4 = 148033228663842577217875354299846790937_u128 - 332203655572744283600705221467882002934_u128;
_53 = _21 as isize;
(*_43) = 3058353108_u32;
Goto(bb33)
}
bb33 = {
_64 = [34_u8,156_u8,57_u8,204_u8,109_u8,251_u8,173_u8,81_u8];
place!(Field::<isize>(Variant(_9, 0), 0)) = (*_37) - (*_37);
(*_37) = _33 as isize;
(*_43) = !3993775234_u32;
_66 = core::ptr::addr_of!(_7);
_63 = [102_u8,120_u8,198_u8,109_u8,181_u8,134_u8,165_u8];
(*_66) = _26;
(*_66) = _2;
_36 = _47 * _47;
(*_43) = 2845488604_u32;
_59 = Field::<isize>(Variant(_9, 0), 0);
(*_37) = _58 << _25.0;
(*_66) = _5;
(*_60) = core::ptr::addr_of!(_66);
_68 = Adt21::Variant1 { fld0: Field::<i64>(Variant(_70.fld5, 1), 0),fld1: 6_usize,fld2: Field::<(u16,)>(Variant(_70.fld5, 1), 2),fld3: 212_u8 };
(*_66) = RET;
_64 = [13_u8,170_u8,5_u8,83_u8,55_u8,160_u8,91_u8,230_u8];
_2 = (*_66);
match (*_43) {
0 => bb15,
1 => bb34,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
6 => bb39,
2845488604 => bb41,
_ => bb40
}
}
bb34 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb35 = {
(*_15) = _29.fld1.0.0 | _29.fld1.0.0;
_27 = -_29.fld2.1;
_13 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb36 = {
_8 = !_49;
_60 = &mut _40;
_17 = &mut _50;
(*_15) = _29.fld1.0.0;
(*_1) = Field::<i64>(Variant(_29.fld1.1.2, 3), 6) << (*_15);
_29.fld0 = (*_17) <= (*_17);
(*_15) = !_29.fld1.0.0;
(*_17) = _53 << (*_15);
_29.fld1.1.1 = !Field::<i64>(Variant(_29.fld1.1.2, 3), 6);
(*_1) = Field::<i64>(Variant(_29.fld1.1.2, 3), 6) - Field::<i64>(Variant(_29.fld1.1.2, 3), 6);
(*_17) = !_53;
_44 = Field::<i32>(Variant(_29.fld1.1.2, 3), 5) << (*_43);
_1 = core::ptr::addr_of_mut!((*_1));
(*_13) = _2;
Goto(bb21)
}
bb37 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb38 = {
_16 = [(*_15)];
_7 = (*_13);
_2 = (*_13);
_44 = 1_usize as i32;
_59 = (*_17) * (*_17);
Goto(bb23)
}
bb39 = {
(*_13) = _5;
_12 = !_8;
(*_13) = _7;
place!(Field::<isize>(Variant(_3, 0), 0)) = 40869172383598904678999488440246988097_u128 as isize;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
(*_13) = _7;
(*_13) = _5;
_3 = Move(_9);
(*_13) = _7;
(*_13) = RET;
(*_13) = _5;
_7 = (*_13);
_16 = [367_u16];
(*_13) = _7;
_8 = !_12;
_14 = (-17_i8) as f64;
(*_13) = RET;
(*_13) = _7;
(*_13) = RET;
_11 = (*_13);
Goto(bb3)
}
bb40 = {
(*_13) = RET;
(*_13) = _7;
_16 = [62223_u16];
_6 = &(*_13);
(*_13) = _11;
RET = (*_13);
_8 = !_12;
_7 = (*_13);
(*_13) = RET;
(*_13) = _5;
(*_13) = _7;
(*_13) = _7;
_22 = _2;
_6 = &RET;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
(*_13) = _22;
(*_13) = (*_6);
_23 = (-49783611509215622604813325762153057758_i128) << (-1027888384_i32);
_13 = core::ptr::addr_of!((*_13));
(*_13) = (*_6);
_9 = Adt52::Variant0 { fld0: 94_isize };
Goto(bb4)
}
bb41 = {
(*_43) = _14 as u32;
(*_66) = _2;
(*_60) = core::ptr::addr_of!(_66);
_71 = core::ptr::addr_of_mut!(_44);
_76 = core::ptr::addr_of_mut!(_3);
(*_37) = _59 | Field::<isize>(Variant(_9, 0), 0);
(*_15) = _70.fld2.0;
(*_66) = _2;
(*_71) = (-1290538262_i32);
(*_15) = _49 as u16;
_31 = &(*_66);
(*_66) = _21;
(*_66) = _21;
(*_71) = (-549310170_i32) | (-1776815639_i32);
(*_66) = _11;
(*_60) = core::ptr::addr_of!(_13);
(*_76) = Move(_9);
(*_60) = core::ptr::addr_of!(_66);
place!(Field::<usize>(Variant(_70.fld5, 1), 1)) = !5_usize;
_70.fld5 = Adt21::Variant1 { fld0: Field::<i64>(Variant(_68, 1), 0),fld1: 7_usize,fld2: _70.fld2,fld3: 171_u8 };
(*_37) = (*_66) as isize;
(*_66) = _22;
_58 = Field::<isize>(Variant((*_76), 0), 0);
_34 = core::ptr::addr_of_mut!(_73);
_75.0 = core::ptr::addr_of_mut!(_35.0);
_4 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_70.fld5, 1), 0)));
Goto(bb42)
}
bb42 = {
(*_34) = [_39];
(*_43) = !4068088495_u32;
(*_43) = 1032273072_u32 - 2724725851_u32;
place!(Field::<isize>(Variant((*_76), 0), 0)) = _44 as isize;
(*_34) = [_39];
(*_66) = _5;
match _39 {
0 => bb1,
1 => bb2,
2 => bb33,
3 => bb43,
4 => bb44,
5 => bb45,
340282366920938463463374607431768208999 => bb47,
_ => bb46
}
}
bb43 = {
_16 = [(*_15)];
_7 = (*_13);
_2 = (*_13);
_44 = 1_usize as i32;
_59 = (*_17) * (*_17);
Goto(bb23)
}
bb44 = {
_8 = (*_6) >= (*_6);
_7 = _5;
_2 = (*_6);
_5 = (*_6);
_7 = (*_6);
_10 = (*_6) == (*_6);
RET = _7;
_7 = RET;
_2 = RET;
_12 = _8 ^ _10;
_6 = &_7;
_9 = Adt52::Variant0 { fld0: (-74_isize) };
RET = (*_6);
_5 = (*_6);
_13 = core::ptr::addr_of!((*_6));
RET = (*_13);
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
_12 = !_10;
_3 = Adt52::Variant0 { fld0: 9223372036854775807_isize };
place!(Field::<isize>(Variant(_9, 0), 0)) = !9223372036854775807_isize;
_7 = RET;
_13 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb45 = {
_33 = _29.fld1.0.0 as i8;
(*_40) = core::ptr::addr_of!((*_13));
(*_40) = core::ptr::addr_of!(_2);
(*_17) = Field::<isize>(Variant(_29.fld5, 0), 0) >> (*_43);
Call((*_1) = core::intrinsics::transmute(_29.fld2.2), ReturnTo(bb18), UnwindUnreachable())
}
bb46 = {
(*_15) = _29.fld1.0.0 | _29.fld1.0.0;
_27 = -_29.fld2.1;
_13 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb47 = {
(*_15) = Field::<(u16,)>(Variant(_70.fld5, 1), 2).0 & Field::<(u16,)>(Variant(_70.fld5, 1), 2).0;
(*_37) = _42 + _59;
(*_43) = !3976302497_u32;
(*_4) = Field::<i64>(Variant(_68, 1), 0);
place!(Field::<isize>(Variant((*_76), 0), 0)) = (*_37) + (*_37);
_54 = (*_37) >> _58;
(*_15) = !Field::<(u16,)>(Variant(_70.fld5, 1), 2).0;
(*_37) = Field::<isize>(Variant((*_76), 0), 0) & Field::<isize>(Variant((*_76), 0), 0);
place!(Field::<isize>(Variant((*_76), 0), 0)) = (*_37) << (*_37);
_83.1.0.0 = _36 - _47;
_74 = (_24, (*_4));
(*_43) = !2398771758_u32;
(*_60) = core::ptr::addr_of!(_13);
_81 = _14;
place!(Field::<u8>(Variant(_68, 1), 3)) = 245_u8 - 102_u8;
(*_76) = Adt52::Variant0 { fld0: (*_37) };
_83.1.0.2 = !_23;
_70.fld5 = Adt21::Variant2 { fld0: (*_43),fld1: (*_66),fld2: _83.1.0.0,fld3: _81,fld4: 2_usize };
(*_60) = core::ptr::addr_of!(_13);
(*_43) = Field::<u32>(Variant(_70.fld5, 2), 0);
(*_66) = _5;
place!(Field::<isize>(Variant((*_76), 0), 0)) = _83.1.0.2 as isize;
_81 = _27 - _14;
(*_37) = _59;
Goto(bb48)
}
bb48 = {
_75.1 = !_70.fld4;
Goto(bb49)
}
bb49 = {
(*_60) = core::ptr::addr_of!(_66);
_83.1.1 = 3_usize;
_79 = core::ptr::addr_of_mut!((*_15));
Goto(bb50)
}
bb50 = {
Call(_90 = dump_var(Move(_49), Move(_41), Move(_8), Move(_39)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_90 = dump_var(Move(_59), Move(_25), Move(_33), Move(_24)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_90 = dump_var(Move(_5), Move(_54), Move(_45), Move(_30)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_90 = dump_var(Move(_50), Move(_58), Move(_74), Move(_21)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: char,mut _3: isize,mut _4: char,mut _5: char,mut _6: Adt52,mut _7: char) -> char {
mir! {
type RET = char;
let _8: isize;
let _9: ((f32, i8, i128), usize, &'static (f32, i8, i128));
let _10: Adt17;
let _11: f64;
let _12: isize;
let _13: f64;
let _14: &'static mut &'static mut u32;
let _15: (&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32);
let _16: *mut &'static mut &'static mut u32;
let _17: char;
let _18: [u32; 3];
let _19: (u16,);
let _20: *mut i32;
let _21: i128;
let _22: &'static mut u32;
let _23: &'static mut *const *const char;
let _24: bool;
let _25: &'static mut (*mut u16,);
let _26: &'static mut &'static mut u32;
let _27: u8;
let _28: i16;
let _29: Adt55;
let _30: (u16,);
let _31: f32;
let _32: isize;
let _33: char;
let _34: (i32, *const Adt21, &'static mut u32, u32);
let _35: *mut &'static mut &'static mut u32;
let _36: u8;
let _37: isize;
let _38: f64;
let _39: *const Adt21;
let _40: u16;
let _41: *const *const char;
let _42: *mut *mut u16;
let _43: isize;
let _44: char;
let _45: isize;
let _46: char;
let _47: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _48: char;
let _49: i32;
let _50: f32;
let _51: isize;
let _52: char;
let _53: i8;
let _54: f32;
let _55: [i16; 1];
let _56: i32;
let _57: [u32; 6];
let _58: &'static Adt26;
let _59: i16;
let _60: Adt64;
let _61: bool;
let _62: f32;
let _63: f64;
let _64: *mut u16;
let _65: char;
let _66: (char, i64, Adt17);
let _67: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _68: f64;
let _69: &'static i16;
let _70: isize;
let _71: &'static mut u32;
let _72: (i32, *const Adt21, &'static mut u32, u32);
let _73: char;
let _74: (*const &'static (f32, i8, i128),);
let _75: char;
let _76: (char, i64, Adt17);
let _77: [u32; 6];
let _78: isize;
let _79: *mut *mut u16;
let _80: i64;
let _81: f32;
let _82: [i128; 8];
let _83: &'static mut isize;
let _84: (u16,);
let _85: &'static mut &'static mut u32;
let _86: isize;
let _87: *const i64;
let _88: ([u16; 5], i64);
let _89: isize;
let _90: &'static Adt26;
let _91: char;
let _92: char;
let _93: u128;
let _94: &'static char;
let _95: [i64; 2];
let _96: *const i64;
let _97: *const *const char;
let _98: [u128; 7];
let _99: f64;
let _100: Adt66;
let _101: f32;
let _102: bool;
let _103: *const i64;
let _104: u16;
let _105: (u16,);
let _106: *mut &'static char;
let _107: [u32; 3];
let _108: char;
let _109: *mut i32;
let _110: f32;
let _111: f32;
let _112: char;
let _113: *mut u16;
let _114: isize;
let _115: &'static Adt66;
let _116: f64;
let _117: bool;
let _118: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _119: &'static i16;
let _120: u8;
let _121: i16;
let _122: [u16; 5];
let _123: u64;
let _124: f32;
let _125: ();
let _126: ();
{
_2 = _1;
_8 = -_3;
_7 = _1;
RET = _5;
_2 = _4;
_5 = _4;
_8 = _3 & Field::<isize>(Variant(_6, 0), 0);
_6 = Adt52::Variant0 { fld0: _3 };
_9.0.2 = (-11470_i16) as i128;
_9.0.0 = (-74_i8) as f32;
place!(Field::<isize>(Variant(_6, 0), 0)) = _8 | _8;
_9.0.2 = 64098266453675396320527965519409901003_i128 << Field::<isize>(Variant(_6, 0), 0);
_9.1 = 9378863422170305962_u64 as usize;
_9.0.0 = _9.1 as f32;
_11 = (-596420447_i32) as f64;
_8 = Field::<isize>(Variant(_6, 0), 0) & Field::<isize>(Variant(_6, 0), 0);
Goto(bb1)
}
bb1 = {
_12 = Field::<isize>(Variant(_6, 0), 0);
_8 = _3 ^ Field::<isize>(Variant(_6, 0), 0);
_9.0.1 = 106_i8 & (-126_i8);
_9.0.1 = (-40_i8) | 45_i8;
_13 = _11 - _11;
Call(_4 = fn14(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = true as isize;
_10 = Adt17::Variant3 { fld0: 1402536121959522685414508224268260445_u128,fld1: _1,fld2: 2227474564_u32,fld3: _13,fld4: 10919572977467540938_u64,fld5: 628428799_i32,fld6: 106495430187236247_i64,fld7: _9.0.2 };
_15.2 = [9083_u16];
RET = _5;
place!(Field::<u128>(Variant(_10, 3), 0)) = (-4348111277914211829_i64) as u128;
_18 = [2768835103_u32,2606232953_u32,1277528643_u32];
place!(Field::<isize>(Variant(_6, 0), 0)) = 50590_u16 as isize;
_15.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_10, 3), 5)));
_9.2 = &_9.0;
_2 = _1;
_15.2 = [60097_u16];
_9.0.1 = (-121_i8) >> _9.0.2;
Goto(bb3)
}
bb3 = {
_17 = RET;
place!(Field::<f64>(Variant(_10, 3), 3)) = _11 * _13;
place!(Field::<u32>(Variant(_10, 3), 2)) = _9.0.1 as u32;
_15.2 = [35478_u16];
place!(Field::<i64>(Variant(_10, 3), 6)) = -(-8363774925696686314_i64);
_8 = _13 as isize;
_9.1 = 6_usize >> Field::<isize>(Variant(_6, 0), 0);
_19 = (3657_u16,);
_10 = Adt17::Variant2 { fld0: 245995255968940195616967830791718860768_u128,fld1: _7,fld2: 32_u8,fld3: _9.0.1,fld4: _9.0.0,fld5: 8582646160854281475_i64 };
place!(Field::<isize>(Variant(_6, 0), 0)) = -_8;
match _19.0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
3657 => bb9,
_ => bb8
}
}
bb4 = {
_12 = true as isize;
_10 = Adt17::Variant3 { fld0: 1402536121959522685414508224268260445_u128,fld1: _1,fld2: 2227474564_u32,fld3: _13,fld4: 10919572977467540938_u64,fld5: 628428799_i32,fld6: 106495430187236247_i64,fld7: _9.0.2 };
_15.2 = [9083_u16];
RET = _5;
place!(Field::<u128>(Variant(_10, 3), 0)) = (-4348111277914211829_i64) as u128;
_18 = [2768835103_u32,2606232953_u32,1277528643_u32];
place!(Field::<isize>(Variant(_6, 0), 0)) = 50590_u16 as isize;
_15.3 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_10, 3), 5)));
_9.2 = &_9.0;
_2 = _1;
_15.2 = [60097_u16];
_9.0.1 = (-121_i8) >> _9.0.2;
Goto(bb3)
}
bb5 = {
_12 = Field::<isize>(Variant(_6, 0), 0);
_8 = _3 ^ Field::<isize>(Variant(_6, 0), 0);
_9.0.1 = 106_i8 & (-126_i8);
_9.0.1 = (-40_i8) | 45_i8;
_13 = _11 - _11;
Call(_4 = fn14(), ReturnTo(bb2), UnwindUnreachable())
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
_19.0 = 46474_u16;
_18 = [3055281464_u32,4107792009_u32,901044342_u32];
_11 = _13;
match _19.0 {
0 => bb1,
1 => bb7,
2 => bb10,
46474 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_9.0.0 = _9.0.2 as f32;
_8 = !_3;
_20 = Move(_15.3);
_15.3 = Move(_20);
_5 = _17;
_24 = !false;
_17 = RET;
_9.0.0 = -Field::<f32>(Variant(_10, 2), 4);
place!(Field::<u128>(Variant(_10, 2), 0)) = 308679535767327100614543797123828432530_u128;
_2 = RET;
place!(Field::<i8>(Variant(_10, 2), 3)) = !_9.0.1;
_21 = _9.0.2 ^ _9.0.2;
_1 = RET;
_20 = Move(_15.3);
_24 = _21 == _9.0.2;
place!(Field::<i64>(Variant(_10, 2), 5)) = (-1539097530074091628_i64) + (-6333041800414443637_i64);
_13 = _11 * _11;
place!(Field::<char>(Variant(_10, 2), 1)) = _2;
_19 = (26897_u16,);
place!(Field::<i64>(Variant(_10, 2), 5)) = 5545165876596471730_i64;
_10 = Adt17::Variant1 { fld0: 226_u8 };
_15.3 = Move(_20);
RET = _4;
_11 = _13 - _13;
match _19.0 {
0 => bb1,
1 => bb13,
2 => bb14,
3 => bb15,
26897 => bb17,
_ => bb16
}
}
bb13 = {
_12 = Field::<isize>(Variant(_6, 0), 0);
_8 = _3 ^ Field::<isize>(Variant(_6, 0), 0);
_9.0.1 = 106_i8 & (-126_i8);
_9.0.1 = (-40_i8) | 45_i8;
_13 = _11 - _11;
Call(_4 = fn14(), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_19.0 = 46474_u16;
_18 = [3055281464_u32,4107792009_u32,901044342_u32];
_11 = _13;
match _19.0 {
0 => bb1,
1 => bb7,
2 => bb10,
46474 => bb12,
_ => bb11
}
}
bb16 = {
_12 = Field::<isize>(Variant(_6, 0), 0);
_8 = _3 ^ Field::<isize>(Variant(_6, 0), 0);
_9.0.1 = 106_i8 & (-126_i8);
_9.0.1 = (-40_i8) | 45_i8;
_13 = _11 - _11;
Call(_4 = fn14(), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_2 = _5;
_2 = _1;
Goto(bb18)
}
bb18 = {
_9.0.0 = 1296939396038896723_u64 as f32;
_20 = Move(_15.3);
_12 = Field::<isize>(Variant(_6, 0), 0);
_18 = [250472391_u32,3733023906_u32,1611593657_u32];
_1 = _4;
_15.2 = [_19.0];
_9.0.2 = _21;
_9.0.1 = 44_i8 * 80_i8;
_27 = 236_u8;
_9.0.1 = _9.0.0 as i8;
_28 = -10372_i16;
_21 = _9.0.2 ^ _9.0.2;
place!(Field::<u8>(Variant(_10, 1), 0)) = !_27;
_21 = _9.0.2;
_4 = RET;
_9.1 = Field::<isize>(Variant(_6, 0), 0) as usize;
_3 = _9.0.2 as isize;
_19.0 = 20614_u16;
place!(Field::<isize>(Variant(_6, 0), 0)) = _19.0 as isize;
_30.0 = _19.0;
_34.3 = 2960464753_u32 << _27;
_9.1 = _34.3 as usize;
_15.3 = core::ptr::addr_of_mut!(_34.0);
_31 = _9.0.0;
match _27 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb7,
236 => bb19,
_ => bb5
}
}
bb19 = {
_15.1 = [_30.0];
_9.1 = 8720374485163355637_usize >> _21;
_15.3 = core::ptr::addr_of_mut!(_34.0);
_37 = _12;
_9.0.2 = _21 | _21;
_34.2 = &mut _34.3;
place!(Field::<isize>(Variant(_6, 0), 0)) = _3 + _3;
match _30.0 {
0 => bb1,
1 => bb8,
2 => bb9,
3 => bb16,
20614 => bb20,
_ => bb10
}
}
bb20 = {
_27 = Field::<u8>(Variant(_10, 1), 0) + Field::<u8>(Variant(_10, 1), 0);
_2 = RET;
_32 = _24 as isize;
place!(Field::<isize>(Variant(_6, 0), 0)) = _9.0.1 as isize;
_32 = _12 & _12;
_40 = _30.0;
_3 = _12;
_40 = _30.0 & _19.0;
_27 = Field::<u8>(Variant(_10, 1), 0) - Field::<u8>(Variant(_10, 1), 0);
_21 = _30.0 as i128;
_19.0 = _30.0 & _40;
_13 = _11;
_9.1 = 4_usize;
_4 = _17;
_28 = 11372_i16 << _21;
_15.3 = Move(_20);
_31 = _9.0.0 + _9.0.0;
_31 = _28 as f32;
_37 = _9.0.2 as isize;
_10 = Adt17::Variant1 { fld0: _27 };
_38 = -_11;
_31 = _9.0.0;
_15.1 = [_40];
place!(Field::<isize>(Variant(_6, 0), 0)) = _37 * _37;
_36 = !Field::<u8>(Variant(_10, 1), 0);
match _30.0 {
0 => bb7,
1 => bb2,
2 => bb5,
20614 => bb22,
_ => bb21
}
}
bb21 = {
_19.0 = 46474_u16;
_18 = [3055281464_u32,4107792009_u32,901044342_u32];
_11 = _13;
match _19.0 {
0 => bb1,
1 => bb7,
2 => bb10,
46474 => bb12,
_ => bb11
}
}
bb22 = {
_12 = _37;
_38 = _28 as f64;
_19.0 = !_40;
_19.0 = _40 * _40;
_32 = -_12;
_45 = 66541186610303340079829224431367912578_u128 as isize;
_33 = _17;
Goto(bb23)
}
bb23 = {
_4 = _7;
_15.2 = [_19.0];
_38 = Field::<isize>(Variant(_6, 0), 0) as f64;
_13 = _38;
_12 = _8 | Field::<isize>(Variant(_6, 0), 0);
_36 = !_27;
_1 = _7;
_9.1 = 17851421651850963344_usize | 7_usize;
_30 = (_40,);
_47.1.0.1 = 470763106_u32 as i8;
_30.0 = _40;
_37 = Field::<isize>(Variant(_6, 0), 0);
_31 = _9.0.0;
_44 = _17;
_20 = core::ptr::addr_of_mut!(_49);
Goto(bb24)
}
bb24 = {
(*_20) = 261170372_i32 | (-2146497474_i32);
_30 = (_19.0,);
_46 = _17;
_47.1.0.2 = _9.0.2 + _9.0.2;
_47.1.0.1 = _9.0.1 - _9.0.1;
(*_20) = _31 as i32;
(*_20) = (-1239935681_i32) - (-1461770927_i32);
(*_20) = -154739841_i32;
_31 = -_9.0.0;
Goto(bb25)
}
bb25 = {
(*_20) = _38 as i32;
_49 = 522468856_i32;
_47.1.0.0 = -_9.0.0;
_54 = _38 as f32;
_21 = _47.1.0.2 >> _49;
_9.0.1 = _47.1.0.1 + _47.1.0.1;
_47.1 = (_9.0, _9.1, Move(_9.2));
(*_20) = 2140257419_i32 >> Field::<isize>(Variant(_6, 0), 0);
_48 = _17;
_28 = (-31253_i16) + (-26519_i16);
_9.1 = _47.1.1;
(*_20) = !(-625458863_i32);
_47.0 = Adt21::Variant0 { fld0: _37 };
_17 = _46;
(*_20) = _9.1 as i32;
_57 = [357557514_u32,2443670459_u32,364658385_u32,4220823872_u32,2829430991_u32,2723968243_u32];
_1 = RET;
_19 = _30;
_43 = _40 as isize;
_55 = [_28];
_56 = (*_20) & (*_20);
Goto(bb26)
}
bb26 = {
_32 = _54 as isize;
(*_20) = -_56;
_47.1.0.1 = -_9.0.1;
(*_20) = _56 >> Field::<isize>(Variant(_47.0, 0), 0);
(*_20) = -_56;
_48 = _17;
(*_20) = _56 + _56;
(*_20) = _56 & _56;
_9.1 = !_47.1.1;
_5 = RET;
Goto(bb27)
}
bb27 = {
_2 = _7;
(*_20) = _56 & _56;
_32 = -Field::<isize>(Variant(_47.0, 0), 0);
(*_20) = 433517960_u32 as i32;
_12 = -Field::<isize>(Variant(_47.0, 0), 0);
_40 = _19.0;
_20 = core::ptr::addr_of_mut!((*_20));
_20 = core::ptr::addr_of_mut!((*_20));
_39 = core::ptr::addr_of!(_47.0);
_19.0 = _30.0 ^ _40;
Goto(bb28)
}
bb28 = {
_53 = _9.0.1 >> Field::<isize>(Variant((*_39), 0), 0);
_15.3 = core::ptr::addr_of_mut!((*_20));
(*_39) = Adt21::Variant1 { fld0: 7554862237528087105_i64,fld1: _9.1,fld2: _19,fld3: _36 };
_17 = _5;
place!(Field::<(u16,)>(Variant((*_39), 1), 2)) = (_19.0,);
place!(Field::<i64>(Variant((*_39), 1), 0)) = 2560589760039938206_i64 | (-8847228665517995594_i64);
_62 = -_54;
_37 = Field::<isize>(Variant(_6, 0), 0) >> Field::<(u16,)>(Variant((*_39), 1), 2).0;
place!(Field::<usize>(Variant((*_39), 1), 1)) = _47.1.1;
_27 = Field::<u8>(Variant((*_39), 1), 3) & Field::<u8>(Variant((*_39), 1), 3);
(*_20) = -_56;
place!(Field::<usize>(Variant((*_39), 1), 1)) = _9.1 << Field::<i64>(Variant((*_39), 1), 0);
place!(Field::<(u16,)>(Variant((*_39), 1), 2)).0 = !_40;
place!(Field::<u8>(Variant((*_39), 1), 3)) = !_36;
place!(Field::<(u16,)>(Variant((*_39), 1), 2)) = _30;
(*_39) = Adt21::Variant0 { fld0: _37 };
place!(Field::<isize>(Variant((*_39), 0), 0)) = _37 ^ _32;
_12 = _47.1.1 as isize;
place!(Field::<isize>(Variant((*_39), 0), 0)) = !_37;
(*_20) = 11366174938400416649_u64 as i32;
_47.1.0.1 = 222016147189826803511646068684351860318_u128 as i8;
_15.1 = [_19.0];
place!(Field::<isize>(Variant((*_39), 0), 0)) = _12 & _37;
place!(Field::<isize>(Variant((*_39), 0), 0)) = -Field::<isize>(Variant(_6, 0), 0);
Goto(bb29)
}
bb29 = {
_36 = Field::<u8>(Variant(_10, 1), 0) & Field::<u8>(Variant(_10, 1), 0);
_43 = Field::<isize>(Variant((*_39), 0), 0) + Field::<isize>(Variant((*_39), 0), 0);
(*_39) = Adt21::Variant2 { fld0: 1128727402_u32,fld1: _1,fld2: _54,fld3: _11,fld4: _47.1.1 };
place!(Field::<u32>(Variant((*_39), 2), 0)) = Field::<usize>(Variant((*_39), 2), 4) as u32;
place!(Field::<char>(Variant((*_39), 2), 1)) = _4;
place!(Field::<f32>(Variant((*_39), 2), 2)) = _54;
_30 = (_40,);
(*_20) = _56 - _56;
place!(Field::<f64>(Variant((*_39), 2), 3)) = _38 * _13;
_66.2 = Adt17::Variant0 { fld0: Field::<f64>(Variant((*_39), 2), 3),fld1: Field::<char>(Variant((*_39), 2), 1),fld2: _43,fld3: 145396420556414955916740748272316696770_u128,fld4: _28,fld5: Field::<f32>(Variant((*_39), 2), 2) };
place!(Field::<usize>(Variant((*_39), 2), 4)) = _47.1.1 + _9.1;
_5 = Field::<char>(Variant((*_39), 2), 1);
place!(Field::<char>(Variant((*_39), 2), 1)) = _5;
_47.1.1 = !Field::<usize>(Variant((*_39), 2), 4);
Goto(bb30)
}
bb30 = {
place!(Field::<f32>(Variant((*_39), 2), 2)) = -_62;
_9.0.2 = _47.1.0.2 >> Field::<u32>(Variant((*_39), 2), 0);
_9.0.2 = _47.1.0.2;
place!(Field::<usize>(Variant((*_39), 2), 4)) = _47.1.1 & _47.1.1;
place!(Field::<usize>(Variant((*_39), 2), 4)) = _53 as usize;
_13 = Field::<f64>(Variant((*_39), 2), 3) * Field::<f64>(Variant((*_39), 2), 3);
(*_39) = Adt21::Variant2 { fld0: 77666417_u32,fld1: _33,fld2: Field::<f32>(Variant(_66.2, 0), 5),fld3: _38,fld4: _47.1.1 };
_47.0 = Adt21::Variant1 { fld0: 3371273675788570237_i64,fld1: _9.1,fld2: _30,fld3: _27 };
_67.1 = _53 > _53;
(*_39) = Adt21::Variant1 { fld0: (-8452881314884648782_i64),fld1: _47.1.1,fld2: _19,fld3: _27 };
place!(Field::<usize>(Variant((*_39), 1), 1)) = _67.1 as usize;
(*_39) = Adt21::Variant1 { fld0: 8309667779297824531_i64,fld1: _47.1.1,fld2: _19,fld3: _36 };
place!(Field::<i64>(Variant((*_39), 1), 0)) = 5887272144829613941_i64 - (-6687146976215496971_i64);
_53 = 3078778497_u32 as i8;
place!(Field::<(u16,)>(Variant((*_39), 1), 2)) = (_19.0,);
(*_20) = _56;
_61 = Field::<(u16,)>(Variant((*_39), 1), 2).0 > Field::<(u16,)>(Variant((*_39), 1), 2).0;
_51 = Field::<i16>(Variant(_66.2, 0), 4) as isize;
place!(Field::<usize>(Variant((*_39), 1), 1)) = _9.1 << (*_20);
_47.2 = &_9.0;
_64 = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant((*_39), 1), 2)).0);
_59 = Field::<i16>(Variant(_66.2, 0), 4);
place!(Field::<u8>(Variant((*_39), 1), 3)) = _36 >> (*_20);
Goto(bb31)
}
bb31 = {
(*_39) = Adt21::Variant0 { fld0: _32 };
place!(Field::<isize>(Variant((*_39), 0), 0)) = _67.1 as isize;
Goto(bb32)
}
bb32 = {
_47.1.2 = &_9.0;
_55 = [_59];
_65 = _5;
_9.0.2 = (-2420761105114763791_i64) as i128;
place!(Field::<isize>(Variant((*_39), 0), 0)) = Field::<isize>(Variant(_66.2, 0), 2);
_43 = -Field::<isize>(Variant((*_39), 0), 0);
(*_39) = Adt21::Variant1 { fld0: (-8513903691750502176_i64),fld1: _47.1.1,fld2: _30,fld3: _27 };
(*_39) = Adt21::Variant2 { fld0: 2670538012_u32,fld1: _5,fld2: Field::<f32>(Variant(_66.2, 0), 5),fld3: _13,fld4: _47.1.1 };
_53 = _9.0.1 | _9.0.1;
_5 = _48;
place!(Field::<f32>(Variant((*_39), 2), 2)) = Field::<f32>(Variant(_66.2, 0), 5);
_11 = _53 as f64;
place!(Field::<f64>(Variant((*_39), 2), 3)) = -_38;
place!(Field::<char>(Variant((*_39), 2), 1)) = _33;
(*_20) = _56;
_66 = (Field::<char>(Variant((*_39), 2), 1), 7635917153340911360_i64, Move(_10));
_32 = -_8;
_67.0.0 = &_66;
_47.1.2 = &_47.1.0;
place!(Field::<u32>(Variant((*_39), 2), 0)) = 1300227308_u32 | 2911533582_u32;
_70 = _43;
_47.1.0.1 = Field::<u32>(Variant((*_39), 2), 0) as i8;
Goto(bb33)
}
bb33 = {
(*_39) = Adt21::Variant2 { fld0: 422482543_u32,fld1: _17,fld2: _62,fld3: _13,fld4: _9.1 };
match _66.1 {
0 => bb4,
7635917153340911360 => bb35,
_ => bb34
}
}
bb34 = {
_12 = Field::<isize>(Variant(_6, 0), 0);
_8 = _3 ^ Field::<isize>(Variant(_6, 0), 0);
_9.0.1 = 106_i8 & (-126_i8);
_9.0.1 = (-40_i8) | 45_i8;
_13 = _11 - _11;
Call(_4 = fn14(), ReturnTo(bb2), UnwindUnreachable())
}
bb35 = {
place!(Field::<usize>(Variant((*_39), 2), 4)) = 3259162013_u32 as usize;
(*_20) = _56;
_47.1.0.1 = _53;
place!(Field::<char>(Variant((*_39), 2), 1)) = _7;
_33 = Field::<char>(Variant((*_39), 2), 1);
place!(Field::<f64>(Variant((*_39), 2), 3)) = _66.1 as f64;
place!(Field::<u32>(Variant((*_39), 2), 0)) = 3668244428_u32 | 1562812393_u32;
(*_39) = Adt21::Variant0 { fld0: _43 };
_73 = _33;
_67.0.3 = core::ptr::addr_of_mut!((*_20));
_18 = [1384736321_u32,1035418544_u32,1231303815_u32];
place!(Field::<isize>(Variant((*_39), 0), 0)) = _70 << _21;
_40 = !_19.0;
_9.0.1 = _53;
Goto(bb36)
}
bb36 = {
_69 = &_59;
(*_39) = Adt21::Variant0 { fld0: Field::<isize>(Variant(_6, 0), 0) };
_10 = Move(_66.2);
_72.0 = (*_20) & (*_20);
_67.3 = &mut place!(Field::<isize>(Variant(_47.0, 0), 0));
(*_20) = _72.0 * _72.0;
_5 = _17;
_67.3 = &mut _43;
_56 = _62 as i32;
_78 = _70 * _37;
_56 = _53 as i32;
_8 = _70 << (*_20);
_64 = core::ptr::addr_of_mut!(_30.0);
match _66.1 {
0 => bb20,
7635917153340911360 => bb37,
_ => bb12
}
}
bb37 = {
_76.0 = _44;
_67.0.1 = [_40];
(*_20) = _72.0 * _56;
(*_64) = _19.0;
(*_20) = _72.0 + _56;
_9.0.1 = -_53;
(*_64) = _40 + _40;
_75 = _44;
place!(Field::<u8>(Variant(_10, 1), 0)) = 214070491182183977618225381060427635122_u128 as u8;
_72.1 = Move(_39);
_15.3 = core::ptr::addr_of_mut!((*_20));
(*_64) = _13 as u16;
_59 = -_28;
_11 = _13 + _13;
Goto(bb38)
}
bb38 = {
_79 = core::ptr::addr_of_mut!(_64);
_62 = _54;
_28 = _59;
(*_79) = core::ptr::addr_of_mut!((*_64));
_67.0.2 = [(*_64)];
_78 = Field::<isize>(Variant(_6, 0), 0) << _8;
(*_64) = _40 * _19.0;
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_64) = _19.0 - _40;
(*_64) = !_40;
_46 = _5;
_54 = _62 * _62;
_82 = [_21,_21,_21,_21,_21,_21,_21,_21];
_86 = _37 | _37;
_36 = _27 | _27;
_42 = core::ptr::addr_of_mut!((*_79));
(*_20) = _72.0;
(*_64) = _19.0;
_76.1 = !_66.1;
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_64) = _40;
Call(_36 = core::intrinsics::bswap(_27), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
(*_64) = _40 << (*_20);
(*_79) = core::ptr::addr_of_mut!(_30.0);
_7 = RET;
_48 = _75;
(*_64) = !_19.0;
(*_79) = core::ptr::addr_of_mut!(_19.0);
_81 = -_62;
_89 = !Field::<isize>(Variant(_6, 0), 0);
(*_64) = _53 as u16;
_31 = _54 - _81;
(*_79) = core::ptr::addr_of_mut!(_84.0);
(*_79) = core::ptr::addr_of_mut!(_30.0);
_88.0 = [_30.0,_19.0,(*_64),(*_64),(*_64)];
RET = _1;
place!(Field::<u8>(Variant(_10, 1), 0)) = _36 + _36;
(*_20) = _56 << _8;
(*_79) = core::ptr::addr_of_mut!(_19.0);
_49 = _56 >> (*_64);
match _66.1 {
0 => bb34,
1 => bb22,
7635917153340911360 => bb40,
_ => bb35
}
}
bb40 = {
(*_64) = _40;
(*_79) = core::ptr::addr_of_mut!((*_64));
_68 = _13 - _38;
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_20) = !_56;
_54 = _31 * _31;
_3 = Field::<isize>(Variant(_6, 0), 0) - _8;
_3 = _37 >> _70;
_9.1 = 16255186921964209218_usize >> _21;
_7 = _17;
_86 = !_3;
(*_64) = !_30.0;
_30 = (_19.0,);
(*_20) = _72.0 - _72.0;
_88.1 = _66.1;
(*_79) = core::ptr::addr_of_mut!((*_64));
_17 = _48;
_38 = 2116454226555160066536728671306286363_u128 as f64;
match _66.1 {
0 => bb5,
1 => bb19,
2 => bb28,
3 => bb10,
4 => bb41,
7635917153340911360 => bb43,
_ => bb42
}
}
bb41 = {
(*_20) = 261170372_i32 | (-2146497474_i32);
_30 = (_19.0,);
_46 = _17;
_47.1.0.2 = _9.0.2 + _9.0.2;
_47.1.0.1 = _9.0.1 - _9.0.1;
(*_20) = _31 as i32;
(*_20) = (-1239935681_i32) - (-1461770927_i32);
(*_20) = -154739841_i32;
_31 = -_9.0.0;
Goto(bb25)
}
bb42 = {
_17 = RET;
place!(Field::<f64>(Variant(_10, 3), 3)) = _11 * _13;
place!(Field::<u32>(Variant(_10, 3), 2)) = _9.0.1 as u32;
_15.2 = [35478_u16];
place!(Field::<i64>(Variant(_10, 3), 6)) = -(-8363774925696686314_i64);
_8 = _13 as isize;
_9.1 = 6_usize >> Field::<isize>(Variant(_6, 0), 0);
_19 = (3657_u16,);
_10 = Adt17::Variant2 { fld0: 245995255968940195616967830791718860768_u128,fld1: _7,fld2: 32_u8,fld3: _9.0.1,fld4: _9.0.0,fld5: 8582646160854281475_i64 };
place!(Field::<isize>(Variant(_6, 0), 0)) = -_8;
match _19.0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
3657 => bb9,
_ => bb8
}
}
bb43 = {
_9.0 = (_31, _53, _21);
_76.1 = _21 as i64;
_52 = _65;
_72.3 = 1067521928_u32;
_67.1 = _24 | _61;
_76 = (RET, _88.1, Move(_10));
_62 = -_81;
(*_64) = _72.3 as u16;
_63 = _13;
_19.0 = _11 as u16;
(*_20) = -_72.0;
_71 = &mut _72.3;
_56 = 326883835296650686189409076242568866989_u128 as i32;
(*_64) = _30.0 | _30.0;
_103 = core::ptr::addr_of!(_80);
(*_79) = core::ptr::addr_of_mut!((*_64));
_19 = _30;
_11 = _68 + _68;
_10 = Move(_76.2);
_92 = _66.0;
_14 = &mut _71;
(*_103) = _66.1 | _66.1;
_95 = [(*_103),(*_103)];
_67.2 = &mut (*_14);
_52 = _44;
Goto(bb44)
}
bb44 = {
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_64) = _40 << _8;
(*_103) = _88.1 & _66.1;
_102 = _67.1;
(*_64) = !_30.0;
_101 = -_62;
(*_20) = _56 << _78;
_109 = core::ptr::addr_of_mut!((*_20));
(*_79) = core::ptr::addr_of_mut!(_84.0);
_83 = &mut _8;
_79 = core::ptr::addr_of_mut!((*_79));
_105.0 = (*_20) as u16;
(*_64) = _105.0;
_37 = -(*_83);
(*_83) = !_78;
_86 = -(*_83);
(*_79) = core::ptr::addr_of_mut!((*_64));
match _76.1 {
0 => bb12,
7635917153340911360 => bb45,
_ => bb41
}
}
bb45 = {
_111 = -_54;
_92 = _5;
(*_83) = (*_103) as isize;
(*_83) = _70 >> _3;
(*_79) = core::ptr::addr_of_mut!(_30.0);
(*_20) = _56 - _56;
(*_79) = core::ptr::addr_of_mut!(_104);
(*_103) = _9.1 as i64;
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_79) = core::ptr::addr_of_mut!((*_64));
_62 = _54 * _9.0.0;
_114 = _65 as isize;
(*_20) = _56;
_77 = [3727405295_u32,3506198332_u32,1103148143_u32,589664242_u32,2584687445_u32,3556214170_u32];
_28 = _59 * _59;
(*_64) = _84.0 ^ _84.0;
_67.0.2 = _15.1;
(*_79) = core::ptr::addr_of_mut!(_40);
place!(Field::<isize>(Variant(_6, 0), 0)) = (*_83);
_59 = -_28;
_31 = _9.0.0;
_9.2 = &_9.0;
(*_83) = 1978695852_u32 as isize;
_105 = _30;
match _88.1 {
0 => bb29,
1 => bb35,
2 => bb34,
3 => bb30,
4 => bb39,
7635917153340911360 => bb46,
_ => bb41
}
}
bb46 = {
_88.1 = (*_103) ^ (*_103);
Goto(bb47)
}
bb47 = {
(*_64) = _84.0;
_67.0.3 = core::ptr::addr_of_mut!((*_20));
(*_64) = !_105.0;
(*_83) = Field::<isize>(Variant(_6, 0), 0) << _37;
RET = _7;
Goto(bb48)
}
bb48 = {
(*_20) = _56 - _56;
_87 = core::ptr::addr_of!((*_103));
_67.0.1 = [_84.0];
(*_87) = (*_20) as i64;
_88.0 = [_104,(*_64),_104,_104,(*_64)];
_66.0 = _17;
_52 = _66.0;
_53 = (*_20) as i8;
(*_64) = !_84.0;
(*_87) = _88.1;
(*_79) = core::ptr::addr_of_mut!((*_64));
(*_20) = _56 << (*_64);
(*_64) = _104 ^ _84.0;
_121 = _28 | _59;
_122 = _88.0;
_81 = _31 - _101;
(*_87) = _88.1 - _88.1;
_117 = _67.1;
_81 = _31 + _111;
_109 = Move(_67.0.3);
(*_64) = 17961792505713153077_u64 as u16;
match _66.1 {
0 => bb8,
1 => bb16,
2 => bb13,
3 => bb35,
7635917153340911360 => bb50,
_ => bb49
}
}
bb49 = {
Return()
}
bb50 = {
_66 = (_17, (*_87), Move(_10));
_15 = (Move(_67.0.0), _67.0.1, _67.0.1, Move(_20));
_87 = core::ptr::addr_of!((*_103));
_26 = Move(_67.2);
_20 = core::ptr::addr_of_mut!(_56);
(*_79) = core::ptr::addr_of_mut!((*_64));
_103 = core::ptr::addr_of!((*_103));
(*_64) = _84.0 | _104;
_35 = core::ptr::addr_of_mut!(_14);
(*_79) = core::ptr::addr_of_mut!(_30.0);
(*_20) = _21 as i32;
_42 = core::ptr::addr_of_mut!((*_79));
(*_20) = -_49;
_65 = _66.0;
_55 = [_121];
_118.1.0 = (_31, _9.0.1, _9.0.2);
(*_64) = !_84.0;
_84.0 = (*_64) + (*_64);
_51 = _37;
_118.1.0.0 = _111;
(*_42) = core::ptr::addr_of_mut!(_104);
_77 = _57;
Goto(bb51)
}
bb51 = {
Call(_125 = dump_var(Move(_61), Move(_86), Move(_55), Move(_46)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_125 = dump_var(Move(_8), Move(_114), Move(_65), Move(_102)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_125 = dump_var(Move(_49), Move(_28), Move(_3), Move(_56)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_125 = dump_var(Move(_84), Move(_7), Move(_37), Move(_48)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_125 = dump_var(Move(_89), Move(_33), Move(_73), Move(_24)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_125 = dump_var(Move(_5), Move(_44), Move(_121), Move(_18)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_125 = dump_var(Move(_30), Move(_117), Move(_1), Move(_36)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14() -> char {
mir! {
type RET = char;
let _1: &'static mut *const *const char;
let _2: bool;
let _3: bool;
let _4: Adt55;
let _5: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _6: i16;
let _7: *mut i32;
let _8: f32;
let _9: [u16; 5];
let _10: isize;
let _11: bool;
let _12: f64;
let _13: [u32; 3];
let _14: &'static (char, i64, Adt17);
let _15: bool;
let _16: f64;
let _17: f32;
let _18: (*mut u16,);
let _19: i128;
let _20: *mut u16;
let _21: *const char;
let _22: i16;
let _23: char;
let _24: f64;
let _25: (i32, *const Adt21, &'static mut u32, u32);
let _26: usize;
let _27: bool;
let _28: isize;
let _29: *const &'static (f32, i8, i128);
let _30: [i64; 2];
let _31: *const &'static (f32, i8, i128);
let _32: *const &'static char;
let _33: char;
let _34: u32;
let _35: &'static i16;
let _36: char;
let _37: f64;
let _38: *mut u16;
let _39: f32;
let _40: [u32; 3];
let _41: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _42: i8;
let _43: (u16, Adt21, *mut [i16; 1]);
let _44: bool;
let _45: u8;
let _46: &'static (f32, i8, i128);
let _47: [u32; 4];
let _48: [i16; 1];
let _49: Adt55;
let _50: [i128; 2];
let _51: &'static mut (*mut u16,);
let _52: u16;
let _53: &'static (f32, i8, i128);
let _54: u128;
let _55: bool;
let _56: bool;
let _57: i32;
let _58: isize;
let _59: char;
let _60: usize;
let _61: Adt55;
let _62: u64;
let _63: bool;
let _64: char;
let _65: &'static mut (*mut u16,);
let _66: *mut [i16; 1];
let _67: *const *const char;
let _68: ((u16,), (char, i64, Adt17));
let _69: char;
let _70: *mut &'static char;
let _71: &'static mut [u32; 3];
let _72: bool;
let _73: [u8; 7];
let _74: isize;
let _75: u32;
let _76: u128;
let _77: [u16; 1];
let _78: u64;
let _79: Adt66;
let _80: &'static mut (*mut u16,);
let _81: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _82: f32;
let _83: &'static mut *const *const char;
let _84: u32;
let _85: char;
let _86: [char; 1];
let _87: ((u16,), (char, i64, Adt17));
let _88: Adt52;
let _89: f64;
let _90: *const *const char;
let _91: char;
let _92: [u8; 7];
let _93: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _94: Adt55;
let _95: Adt66;
let _96: u16;
let _97: &'static mut isize;
let _98: f64;
let _99: bool;
let _100: bool;
let _101: isize;
let _102: Adt64;
let _103: u8;
let _104: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _105: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _106: f32;
let _107: Adt26;
let _108: f64;
let _109: bool;
let _110: &'static mut isize;
let _111: i8;
let _112: bool;
let _113: ([u16; 5], i64);
let _114: bool;
let _115: ([u16; 5], i64);
let _116: f64;
let _117: *mut u16;
let _118: (u16,);
let _119: *mut i32;
let _120: &'static Adt66;
let _121: f64;
let _122: ();
let _123: ();
{
_2 = !true;
_3 = _2 < _2;
_3 = _2;
RET = '\u{f9cff}';
_2 = !_3;
RET = '\u{69495}';
_3 = _2;
_2 = RET != RET;
_3 = !_2;
_3 = _2 <= _2;
_2 = !_3;
_3 = _2;
_2 = !_3;
RET = '\u{a71df}';
_3 = !_2;
_3 = RET > RET;
_3 = _2 | _2;
_3 = _2;
_3 = !_2;
_3 = !_2;
RET = '\u{92d0f}';
_2 = !_3;
_3 = _2 > _2;
_3 = _2 < _2;
Goto(bb1)
}
bb1 = {
RET = '\u{524ed}';
_3 = !_2;
RET = '\u{10bcc8}';
_2 = !_3;
RET = '\u{5a792}';
RET = '\u{c6e8f}';
_3 = _2 < _2;
_3 = !_2;
_3 = _2 ^ _2;
RET = '\u{9b353}';
_3 = !_2;
_6 = _3 as i16;
RET = '\u{1c5b1}';
_5.1 = [RET];
_6 = !(-2952_i16);
_5.2.0 = (-2118224673_i32) << _6;
_2 = !_3;
_3 = _2 <= _2;
Goto(bb2)
}
bb2 = {
_3 = _2 ^ _2;
_5.2.0 = 311970836_i32 + (-159951304_i32);
_5.1 = [RET];
_6 = !1426_i16;
_5.2.3 = 2481039071_u32 >> _5.2.0;
_7 = core::ptr::addr_of_mut!(_5.2.0);
(*_7) = (-7595158907083056344_i64) as i32;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = !1997994622_i32;
_5.2.3 = 2828250823_u32 - 717407249_u32;
(*_7) = 42944_u16 as i32;
(*_7) = 1725389451_i32;
(*_7) = 1969976166_i32 ^ (-47761056_i32);
(*_7) = 291352823_i32 ^ 2077060970_i32;
_5.2.2 = &mut _5.2.3;
(*_7) = 979225504_i32 << _6;
_3 = _2;
_9 = [15180_u16,20715_u16,13253_u16,41952_u16,63377_u16];
(*_7) = 1499141028_i32 << _6;
(*_7) = 1511378994_i32 * (-125362036_i32);
(*_7) = 25609_u16 as i32;
_2 = _3;
(*_7) = !1214296412_i32;
_11 = !_3;
Call((*_7) = core::intrinsics::transmute(RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = 13102213884326889505_usize as f32;
(*_7) = RET as i32;
_7 = core::ptr::addr_of_mut!((*_7));
Goto(bb4)
}
bb4 = {
(*_7) = 321657136950637478527471567628512487540_u128 as i32;
_2 = _3;
(*_7) = 9564583179204991701_u64 as i32;
_11 = !_2;
(*_7) = (-850349263_i32) ^ 931442138_i32;
(*_7) = -744876547_i32;
_9 = [16366_u16,17466_u16,34312_u16,49343_u16,17749_u16];
_6 = -6522_i16;
(*_7) = 1435205111_i32;
_6 = 3104_i16 >> (*_7);
(*_7) = 1459676366_i32 ^ 240592418_i32;
_13 = [1940354356_u32,2777270756_u32,3327788054_u32];
(*_7) = 2126819131_u32 as i32;
_15 = _3;
(*_7) = 203554557_i32 & 804368711_i32;
_6 = 23468_i16;
(*_7) = (-1151794629_i32) | 220691013_i32;
(*_7) = (-181421768_i32) | (-772050476_i32);
(*_7) = _3 as i32;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = (-507171435_i32) - 938070975_i32;
_6 = !30501_i16;
_16 = 12983228450556921877_u64 as f64;
(*_7) = 772438454_i32 | (-419359577_i32);
(*_7) = _16 as i32;
Call(_6 = core::intrinsics::bswap(10603_i16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_7) = -892007292_i32;
(*_7) = (-1868895603_i32) - 898329965_i32;
(*_7) = -2008794462_i32;
_10 = -(-9223372036854775808_isize);
_3 = _15;
(*_7) = (-1083787219_i32) ^ 1735762840_i32;
_7 = core::ptr::addr_of_mut!((*_7));
_17 = 89_u8 as f32;
(*_7) = 13807_u16 as i32;
(*_7) = (-1871491029_i32) ^ (-80729489_i32);
(*_7) = -387999509_i32;
(*_7) = _10 as i32;
_3 = _11;
(*_7) = 1177012875_i32 + (-180925174_i32);
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = _16 as i32;
(*_7) = 4491736009432892640_u64 as i32;
(*_7) = 1058528757_i32 & (-878861792_i32);
RET = '\u{4994d}';
(*_7) = (-484294778_i32) & (-1725101340_i32);
(*_7) = (-2035655215_i32) - (-64274590_i32);
_17 = _8 - _8;
(*_7) = 746861166_i32 & (-898239803_i32);
Goto(bb6)
}
bb6 = {
_3 = _15;
(*_7) = -895124328_i32;
(*_7) = -504623836_i32;
_22 = _6 * _6;
_16 = (*_7) as f64;
(*_7) = (-1761191467_i32) ^ (-23403307_i32);
_2 = _11 | _15;
(*_7) = 215302115_u32 as i32;
(*_7) = -(-2074728970_i32);
_19 = 140_u8 as i128;
(*_7) = (-690598749_i32) | (-1011711982_i32);
_12 = _19 as f64;
(*_7) = -(-1859550479_i32);
Goto(bb7)
}
bb7 = {
_25.3 = 4280405680_u32 - 4055274363_u32;
(*_7) = !1549874071_i32;
(*_7) = 1690461746_i32;
_26 = 0_usize;
(*_7) = (-117_i8) as i32;
(*_7) = 1500695836_i32 >> _19;
_9[_26] = !21227_u16;
(*_7) = 433774285_i32 >> _25.3;
(*_7) = (-1707420071_i32);
_21 = core::ptr::addr_of!(RET);
(*_21) = '\u{d0765}';
_23 = (*_21);
(*_7) = (-1224518539_i32) << _13[_26];
RET = _23;
(*_7) = (-1772760277_i32) | (-87551862_i32);
_20 = core::ptr::addr_of_mut!(_9[_26]);
(*_20) = 29300_u16 >> (*_7);
(*_20) = 42457_u16 >> (*_7);
(*_7) = (-1337291559_i32) << (*_20);
(*_21) = _23;
_27 = _15;
match _13[_26] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1940354356 => bb9,
_ => bb8
}
}
bb8 = {
_8 = 13102213884326889505_usize as f32;
(*_7) = RET as i32;
_7 = core::ptr::addr_of_mut!((*_7));
Goto(bb4)
}
bb9 = {
Goto(bb10)
}
bb10 = {
_12 = _16 * _16;
_34 = !_13[_26];
_8 = (*_20) as f32;
(*_20) = 48871_u16 - 13087_u16;
(*_21) = _23;
_24 = (-6850148194431542436_i64) as f64;
Goto(bb11)
}
bb11 = {
_25.0 = (*_7);
_30 = [7298416560073177635_i64,3541206739843766513_i64];
_9[_26] = !23706_u16;
_25.2 = &mut _13[_26];
(*_21) = _23;
(*_21) = _23;
_18.0 = core::ptr::addr_of_mut!((*_20));
_18 = (Move(_20),);
_6 = _19 as i16;
(*_21) = _23;
_21 = core::ptr::addr_of!(_33);
_35 = &_6;
_11 = !_15;
(*_7) = _25.0;
(*_21) = RET;
match _30[_26] {
0 => bb10,
1 => bb4,
2 => bb6,
7298416560073177635 => bb13,
_ => bb12
}
}
bb12 = {
_3 = _15;
(*_7) = -895124328_i32;
(*_7) = -504623836_i32;
_22 = _6 * _6;
_16 = (*_7) as f64;
(*_7) = (-1761191467_i32) ^ (-23403307_i32);
_2 = _11 | _15;
(*_7) = 215302115_u32 as i32;
(*_7) = -(-2074728970_i32);
_19 = 140_u8 as i128;
(*_7) = (-690598749_i32) | (-1011711982_i32);
_12 = _19 as f64;
(*_7) = -(-1859550479_i32);
Goto(bb7)
}
bb13 = {
_3 = _27;
_12 = 2825026893175468694_u64 as f64;
_37 = _9[_26] as f64;
RET = (*_21);
(*_21) = RET;
(*_21) = _23;
(*_21) = _23;
_23 = (*_21);
_25.2 = &mut _34;
(*_21) = _23;
(*_21) = RET;
_30 = [(-1725490164761997409_i64),6011569295117251749_i64];
_28 = !_10;
(*_7) = !_25.0;
_25.0 = (*_7) * (*_7);
Goto(bb14)
}
bb14 = {
_35 = &_22;
_35 = &_6;
(*_21) = _23;
(*_7) = _25.0 - _25.0;
_19 = 169507432083091351244090512962693482664_i128 ^ (-36818028818553506450644958975207304136_i128);
(*_21) = _23;
_9[_26] = !65233_u16;
_9[_26] = !64848_u16;
_41.2.0 = -(*_7);
_38 = core::ptr::addr_of_mut!(_43.0);
Goto(bb15)
}
bb15 = {
_41.2.1 = core::ptr::addr_of!(_43.1);
(*_38) = _9[_26];
_37 = _12 - _24;
(*_21) = _23;
_26 = !2_usize;
_9 = [(*_38),(*_38),(*_38),(*_38),(*_38)];
_36 = RET;
(*_38) = (-55_i8) as u16;
_26 = _10 as usize;
(*_21) = _36;
(*_7) = _25.0 << (*_35);
(*_21) = _23;
(*_38) = 8410_u16;
(*_38) = 31261_u16;
(*_7) = !_41.2.0;
(*_7) = -_25.0;
(*_7) = _25.0 << (*_35);
_41.2.2 = Move(_25.2);
Goto(bb16)
}
bb16 = {
(*_21) = RET;
match (*_38) {
0 => bb1,
1 => bb12,
2 => bb7,
3 => bb4,
4 => bb5,
31261 => bb17,
_ => bb10
}
}
bb17 = {
_10 = _28 >> (*_7);
_41.2.3 = _25.3 << (*_7);
_42 = 104_i8;
(*_21) = _36;
(*_21) = _23;
(*_7) = _12 as i32;
match (*_38) {
31261 => bb18,
_ => bb9
}
}
bb18 = {
_18.0 = Move(_38);
_40 = [_41.2.3,_41.2.3,_25.3];
(*_21) = RET;
(*_7) = _24 as i32;
_41.2.2 = &mut _41.2.3;
_42 = 121_i8 - (-122_i8);
(*_7) = _25.0;
_28 = -_10;
(*_7) = _25.0 << _28;
_25.1 = core::ptr::addr_of!(_43.1);
_55 = !_15;
_25.2 = &mut _25.3;
(*_21) = RET;
_19 = 12239747114031509954000633311576930740_i128 + (-57688707383996817907619721223852103756_i128);
_44 = !_27;
_6 = _22 >> (*_7);
_37 = _12;
(*_21) = RET;
(*_7) = (-81837563_i32) >> _28;
_20 = Move(_18.0);
_38 = core::ptr::addr_of_mut!(_52);
_8 = _17 * _17;
(*_21) = _36;
(*_21) = _23;
match _43.0 {
0 => bb1,
1 => bb14,
2 => bb3,
3 => bb19,
31261 => bb21,
_ => bb20
}
}
bb19 = {
_41.2.1 = core::ptr::addr_of!(_43.1);
(*_38) = _9[_26];
_37 = _12 - _24;
(*_21) = _23;
_26 = !2_usize;
_9 = [(*_38),(*_38),(*_38),(*_38),(*_38)];
_36 = RET;
(*_38) = (-55_i8) as u16;
_26 = _10 as usize;
(*_21) = _36;
(*_7) = _25.0 << (*_35);
(*_21) = _23;
(*_38) = 8410_u16;
(*_38) = 31261_u16;
(*_7) = !_41.2.0;
(*_7) = -_25.0;
(*_7) = _25.0 << (*_35);
_41.2.2 = Move(_25.2);
Goto(bb16)
}
bb20 = {
_3 = _15;
(*_7) = -895124328_i32;
(*_7) = -504623836_i32;
_22 = _6 * _6;
_16 = (*_7) as f64;
(*_7) = (-1761191467_i32) ^ (-23403307_i32);
_2 = _11 | _15;
(*_7) = 215302115_u32 as i32;
(*_7) = -(-2074728970_i32);
_19 = 140_u8 as i128;
(*_7) = (-690598749_i32) | (-1011711982_i32);
_12 = _19 as f64;
(*_7) = -(-1859550479_i32);
Goto(bb7)
}
bb21 = {
(*_21) = _23;
(*_7) = !693293690_i32;
(*_7) = 1642783492_i32;
(*_21) = _23;
(*_38) = _43.0;
(*_7) = 133_u8 as i32;
_35 = &_22;
_6 = _17 as i16;
_47 = [2939045491_u32,2541249355_u32,582811975_u32,2793653250_u32];
(*_7) = 1864786718_i32 >> (*_38);
_24 = -_37;
_58 = _10;
(*_7) = 2068531854_i32 << _10;
(*_21) = _36;
Call((*_38) = fn15(Move(_35), (*_7), Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
(*_7) = !810460874_i32;
_20 = core::ptr::addr_of_mut!((*_38));
_15 = !_2;
(*_7) = (-725805589_i32);
_50 = [_19,_19];
_28 = _10;
(*_20) = _43.0 / _43.0;
(*_7) = (-1270273514_i32) | 885097671_i32;
_18 = (Move(_20),);
_48 = [_22];
Goto(bb23)
}
bb23 = {
(*_7) = (-843687465_i32) >> (*_38);
(*_7) = _26 as i32;
_35 = &_6;
_2 = _44;
_51 = &mut _18;
_8 = _17 * _17;
match _43.0 {
0 => bb14,
1 => bb11,
2 => bb19,
3 => bb13,
31261 => bb25,
_ => bb24
}
}
bb24 = {
_3 = _15;
(*_7) = -895124328_i32;
(*_7) = -504623836_i32;
_22 = _6 * _6;
_16 = (*_7) as f64;
(*_7) = (-1761191467_i32) ^ (-23403307_i32);
_2 = _11 | _15;
(*_7) = 215302115_u32 as i32;
(*_7) = -(-2074728970_i32);
_19 = 140_u8 as i128;
(*_7) = (-690598749_i32) | (-1011711982_i32);
_12 = _19 as f64;
(*_7) = -(-1859550479_i32);
Goto(bb7)
}
bb25 = {
_43.0 = (*_38) & (*_38);
(*_51).0 = core::ptr::addr_of_mut!((*_38));
_68.0 = (_52,);
_43.1 = Adt21::Variant2 { fld0: 2748571311_u32,fld1: RET,fld2: _17,fld3: _16,fld4: _26 };
_35 = &_22;
place!(Field::<u32>(Variant(_43.1, 2), 0)) = 155164144_u32 * 525968027_u32;
(*_51) = (Move(_38),);
_6 = -(*_35);
Goto(bb26)
}
bb26 = {
place!(Field::<usize>(Variant(_43.1, 2), 4)) = !_26;
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
_39 = _8;
_68.1.0 = Field::<char>(Variant(_43.1, 2), 1);
Call(_68.1.1 = core::intrinsics::transmute(_28), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_19 = -(-49897473074592856522578180951272508100_i128);
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_7) = 249_u8 as i32;
_43.0 = !_52;
_38 = core::ptr::addr_of_mut!(_52);
_48 = [(*_35)];
_63 = _3;
_20 = core::ptr::addr_of_mut!((*_38));
_17 = -Field::<f32>(Variant(_43.1, 2), 2);
_9 = [(*_20),(*_38),(*_20),(*_20),(*_20)];
(*_20) = _43.0;
(*_51).0 = core::ptr::addr_of_mut!((*_20));
(*_51).0 = core::ptr::addr_of_mut!((*_38));
_30 = [_68.1.1,_68.1.1];
_71 = &mut _40;
(*_20) = _8 as u16;
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0)];
(*_51).0 = Move(_20);
(*_51) = (Move(_38),);
_56 = (*_35) >= (*_35);
(*_7) = _26 as i32;
_57 = (*_7) & (*_7);
Goto(bb28)
}
bb28 = {
(*_51).0 = core::ptr::addr_of_mut!(_52);
_58 = _10 * _28;
_68.0.0 = _52 + _43.0;
_54 = 129944082879094229474092774481839986900_u128 * 187739424266720122810745821661491086018_u128;
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0)];
(*_7) = !_57;
_52 = _68.0.0;
(*_7) = _57 + _57;
_3 = _37 == _16;
_43.1 = Adt21::Variant2 { fld0: 993365291_u32,fld1: _33,fld2: _17,fld3: _37,fld4: _26 };
Goto(bb29)
}
bb29 = {
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
_55 = !_63;
_50 = [_19,_19];
(*_71) = [3364059024_u32,4187708383_u32,436858106_u32];
_42 = -(-40_i8);
_68.1.2 = Adt17::Variant2 { fld0: _54,fld1: _33,fld2: 124_u8,fld3: _42,fld4: _8,fld5: _68.1.1 };
_47 = [2805258750_u32,2861328173_u32,2309649789_u32,3510500530_u32];
Goto(bb30)
}
bb30 = {
(*_7) = _58 as i32;
_2 = !_56;
_59 = _33;
_48 = [(*_35)];
(*_71) = [1434028126_u32,1535638259_u32,1243640510_u32];
_43.2 = core::ptr::addr_of_mut!(_48);
_73 = [78_u8,58_u8,80_u8,73_u8,109_u8,127_u8,4_u8];
_52 = !_43.0;
_19 = (-105961156946296561785628246655073102246_i128);
_45 = 230_u8 - 144_u8;
_20 = core::ptr::addr_of_mut!(_43.0);
match _19 {
0 => bb20,
234321209974641901677746360776695109210 => bb32,
_ => bb31
}
}
bb31 = {
_19 = -(-49897473074592856522578180951272508100_i128);
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_7) = 249_u8 as i32;
_43.0 = !_52;
_38 = core::ptr::addr_of_mut!(_52);
_48 = [(*_35)];
_63 = _3;
_20 = core::ptr::addr_of_mut!((*_38));
_17 = -Field::<f32>(Variant(_43.1, 2), 2);
_9 = [(*_20),(*_38),(*_20),(*_20),(*_20)];
(*_20) = _43.0;
(*_51).0 = core::ptr::addr_of_mut!((*_20));
(*_51).0 = core::ptr::addr_of_mut!((*_38));
_30 = [_68.1.1,_68.1.1];
_71 = &mut _40;
(*_20) = _8 as u16;
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0)];
(*_51).0 = Move(_20);
(*_51) = (Move(_38),);
_56 = (*_35) >= (*_35);
(*_7) = _26 as i32;
_57 = (*_7) & (*_7);
Goto(bb28)
}
bb32 = {
(*_20) = _68.0.0 << (*_7);
(*_51) = (Move(_20),);
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_71) = [3534627825_u32,395965988_u32,845088465_u32];
place!(Field::<u8>(Variant(_68.1.2, 2), 2)) = _45 >> _43.0;
_39 = Field::<f32>(Variant(_43.1, 2), 2);
_64 = Field::<char>(Variant(_68.1.2, 2), 1);
place!(Field::<i8>(Variant(_68.1.2, 2), 3)) = _42 >> (*_7);
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
_76 = Field::<i64>(Variant(_68.1.2, 2), 5) as u128;
place!(Field::<u8>(Variant(_68.1.2, 2), 2)) = !_45;
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
_54 = !_76;
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
_67 = core::ptr::addr_of!(_21);
place!(Field::<f32>(Variant(_68.1.2, 2), 4)) = _39 - Field::<f32>(Variant(_43.1, 2), 2);
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
_21 = core::ptr::addr_of!(_59);
_9 = [_68.0.0,_68.0.0,_52,_43.0,_68.0.0];
_59 = Field::<char>(Variant(_68.1.2, 2), 1);
(*_71) = [2983588519_u32,580071510_u32,543010101_u32];
Goto(bb33)
}
bb33 = {
(*_67) = core::ptr::addr_of!((*_21));
_62 = 13810678014921448694_u64 + 901931452127825577_u64;
(*_67) = core::ptr::addr_of!((*_21));
(*_21) = _33;
_81.2.3 = 2192087334_u32;
_58 = !_10;
place!(Field::<char>(Variant(_43.1, 2), 1)) = _36;
_26 = Field::<i8>(Variant(_68.1.2, 2), 3) as usize;
(*_7) = _57 >> Field::<i64>(Variant(_68.1.2, 2), 5);
(*_67) = core::ptr::addr_of!((*_21));
_43.0 = _68.0.0;
place!(Field::<f32>(Variant(_68.1.2, 2), 4)) = _17;
(*_67) = core::ptr::addr_of!((*_21));
match _81.2.3 {
0 => bb12,
1 => bb30,
2 => bb20,
3 => bb19,
4 => bb23,
5 => bb34,
6 => bb35,
2192087334 => bb37,
_ => bb36
}
}
bb34 = {
(*_7) = -892007292_i32;
(*_7) = (-1868895603_i32) - 898329965_i32;
(*_7) = -2008794462_i32;
_10 = -(-9223372036854775808_isize);
_3 = _15;
(*_7) = (-1083787219_i32) ^ 1735762840_i32;
_7 = core::ptr::addr_of_mut!((*_7));
_17 = 89_u8 as f32;
(*_7) = 13807_u16 as i32;
(*_7) = (-1871491029_i32) ^ (-80729489_i32);
(*_7) = -387999509_i32;
(*_7) = _10 as i32;
_3 = _11;
(*_7) = 1177012875_i32 + (-180925174_i32);
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = _16 as i32;
(*_7) = 4491736009432892640_u64 as i32;
(*_7) = 1058528757_i32 & (-878861792_i32);
RET = '\u{4994d}';
(*_7) = (-484294778_i32) & (-1725101340_i32);
(*_7) = (-2035655215_i32) - (-64274590_i32);
_17 = _8 - _8;
(*_7) = 746861166_i32 & (-898239803_i32);
Goto(bb6)
}
bb35 = {
_3 = _27;
_12 = 2825026893175468694_u64 as f64;
_37 = _9[_26] as f64;
RET = (*_21);
(*_21) = RET;
(*_21) = _23;
(*_21) = _23;
_23 = (*_21);
_25.2 = &mut _34;
(*_21) = _23;
(*_21) = RET;
_30 = [(-1725490164761997409_i64),6011569295117251749_i64];
_28 = !_10;
(*_7) = !_25.0;
_25.0 = (*_7) * (*_7);
Goto(bb14)
}
bb36 = {
_12 = _16 * _16;
_34 = !_13[_26];
_8 = (*_20) as f32;
(*_20) = 48871_u16 - 13087_u16;
(*_21) = _23;
_24 = (-6850148194431542436_i64) as f64;
Goto(bb11)
}
bb37 = {
(*_51).0 = core::ptr::addr_of_mut!(_52);
_78 = Field::<u8>(Variant(_68.1.2, 2), 2) as u64;
(*_21) = _23;
_69 = (*_21);
_16 = _37;
(*_7) = _57;
_55 = _12 < _24;
(*_51).0 = core::ptr::addr_of_mut!(_52);
(*_7) = _8 as i32;
(*_71) = [_81.2.3,_81.2.3,_81.2.3];
(*_21) = _69;
_85 = Field::<char>(Variant(_43.1, 2), 1);
(*_7) = -_57;
(*_7) = _78 as i32;
(*_7) = -_57;
_81.2.0 = (*_7) - (*_7);
(*_71) = [_81.2.3,_81.2.3,_81.2.3];
_83 = &mut _67;
(*_7) = _57 << _26;
(*_83) = core::ptr::addr_of!(_21);
(*_21) = Field::<char>(Variant(_43.1, 2), 1);
Call((*_7) = core::intrinsics::transmute(_81.2.3), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
_87.1.0 = (*_21);
(*_7) = (*_35) as i32;
_81.2.2 = &mut _81.2.3;
(*_71) = [577316597_u32,523431110_u32,2888559072_u32];
(*_21) = _87.1.0;
Goto(bb39)
}
bb39 = {
_87.1.1 = -Field::<i64>(Variant(_68.1.2, 2), 5);
(*_7) = -_57;
_93.0.1 = [_68.0.0];
_87.0 = (_68.0.0,);
_87.1.2 = Adt17::Variant2 { fld0: _76,fld1: _33,fld2: _45,fld3: Field::<i8>(Variant(_68.1.2, 2), 3),fld4: Field::<f32>(Variant(_68.1.2, 2), 4),fld5: _68.1.1 };
(*_21) = RET;
place!(Field::<u8>(Variant(_87.1.2, 2), 2)) = _45;
(*_7) = _57 ^ _57;
_93.0.3 = core::ptr::addr_of_mut!((*_7));
(*_21) = Field::<char>(Variant(_87.1.2, 2), 1);
_89 = Field::<f64>(Variant(_43.1, 2), 3);
(*_71) = [3500579933_u32,2905294125_u32,2902768985_u32];
(*_21) = Field::<char>(Variant(_68.1.2, 2), 1);
(*_71) = [1242127973_u32,3624727466_u32,501847106_u32];
(*_7) = _57 * _57;
_28 = Field::<i8>(Variant(_68.1.2, 2), 3) as isize;
(*_83) = core::ptr::addr_of!(_21);
(*_51).0 = core::ptr::addr_of_mut!(_52);
_7 = core::ptr::addr_of_mut!((*_7));
Goto(bb40)
}
bb40 = {
(*_83) = core::ptr::addr_of!(_21);
_93.0.0 = &_87.1;
Goto(bb41)
}
bb41 = {
_87.1 = ((*_21), Field::<i64>(Variant(_68.1.2, 2), 5), Move(_68.1.2));
_85 = _33;
_2 = _27 ^ _56;
_59 = RET;
_76 = _54;
_30 = [_68.1.1,_68.1.1];
(*_21) = _85;
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
(*_21) = _64;
_87.1.2 = Adt17::Variant3 { fld0: _76,fld1: (*_21),fld2: 3619227329_u32,fld3: _12,fld4: _78,fld5: (*_7),fld6: _68.1.1,fld7: _19 };
(*_21) = _36;
(*_51).0 = core::ptr::addr_of_mut!(_96);
place!(Field::<u32>(Variant(_43.1, 2), 0)) = !233425285_u32;
Goto(bb42)
}
bb42 = {
(*_7) = Field::<i32>(Variant(_87.1.2, 3), 5);
_22 = (*_7) as i16;
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0)];
_93.0.1 = [_87.0.0];
_86 = [_33];
(*_83) = core::ptr::addr_of!(_21);
_39 = Field::<f32>(Variant(_43.1, 2), 2) + _17;
place!(Field::<usize>(Variant(_43.1, 2), 4)) = _26 & _26;
match _19 {
0 => bb1,
1 => bb28,
2 => bb30,
234321209974641901677746360776695109210 => bb44,
_ => bb43
}
}
bb43 = {
_25.3 = 4280405680_u32 - 4055274363_u32;
(*_7) = !1549874071_i32;
(*_7) = 1690461746_i32;
_26 = 0_usize;
(*_7) = (-117_i8) as i32;
(*_7) = 1500695836_i32 >> _19;
_9[_26] = !21227_u16;
(*_7) = 433774285_i32 >> _25.3;
(*_7) = (-1707420071_i32);
_21 = core::ptr::addr_of!(RET);
(*_21) = '\u{d0765}';
_23 = (*_21);
(*_7) = (-1224518539_i32) << _13[_26];
RET = _23;
(*_7) = (-1772760277_i32) | (-87551862_i32);
_20 = core::ptr::addr_of_mut!(_9[_26]);
(*_20) = 29300_u16 >> (*_7);
(*_20) = 42457_u16 >> (*_7);
(*_7) = (-1337291559_i32) << (*_20);
(*_21) = _23;
_27 = _15;
match _13[_26] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1940354356 => bb9,
_ => bb8
}
}
bb44 = {
_107.fld5 = _43.1;
(*_83) = core::ptr::addr_of!(_21);
_12 = (*_7) as f64;
(*_51).0 = core::ptr::addr_of_mut!(_87.0.0);
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_107.fld5, 2), 0)];
_96 = _87.0.0 & _43.0;
(*_21) = _68.1.0;
_107.fld3 = Field::<usize>(Variant(_107.fld5, 2), 4);
(*_21) = RET;
(*_21) = _33;
(*_7) = Field::<i32>(Variant(_87.1.2, 3), 5) & Field::<i32>(Variant(_87.1.2, 3), 5);
(*_7) = _57 << _107.fld3;
_68.1.2 = Adt17::Variant1 { fld0: _45 };
_93.3 = &mut _58;
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
(*_51).0 = core::ptr::addr_of_mut!(_87.0.0);
_97 = &mut _28;
(*_71) = [Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0),Field::<u32>(Variant(_43.1, 2), 0)];
_26 = _107.fld3;
_99 = _2 & _56;
_104.1 = _86;
(*_51).0 = core::ptr::addr_of_mut!(_107.fld2.0);
(*_83) = core::ptr::addr_of!(_21);
(*_7) = Field::<i32>(Variant(_87.1.2, 3), 5) & Field::<i32>(Variant(_87.1.2, 3), 5);
(*_97) = -_10;
_102 = Adt64::Variant1 { fld0: _68.0,fld1: _9,fld2: _93.0.1,fld3: _48,fld4: Move(_68.1.2) };
_105.0 = Adt21::Variant1 { fld0: Field::<i64>(Variant(_87.1.2, 3), 6),fld1: _107.fld3,fld2: Field::<(u16,)>(Variant(_102, 1), 0),fld3: Field::<u8>(Variant(Field::<Adt17>(Variant(_102, 1), 4), 1), 0) };
match Field::<i128>(Variant(_87.1.2, 3), 7) {
0 => bb18,
1 => bb24,
234321209974641901677746360776695109210 => bb46,
_ => bb45
}
}
bb45 = {
RET = '\u{524ed}';
_3 = !_2;
RET = '\u{10bcc8}';
_2 = !_3;
RET = '\u{5a792}';
RET = '\u{c6e8f}';
_3 = _2 < _2;
_3 = !_2;
_3 = _2 ^ _2;
RET = '\u{9b353}';
_3 = !_2;
_6 = _3 as i16;
RET = '\u{1c5b1}';
_5.1 = [RET];
_6 = !(-2952_i16);
_5.2.0 = (-2118224673_i32) << _6;
_2 = !_3;
_3 = _2 <= _2;
Goto(bb2)
}
bb46 = {
(*_97) = _10 * _10;
(*_97) = !_10;
_44 = !_3;
(*_21) = _69;
_107.fld1 = _43.0 | Field::<(u16,)>(Variant(_102, 1), 0).0;
place!(Field::<u32>(Variant(_107.fld5, 2), 0)) = Field::<u32>(Variant(_43.1, 2), 0) << (*_97);
_43.0 = _96 & Field::<(u16,)>(Variant(_105.0, 1), 2).0;
_89 = _24 * _12;
_86 = _104.1;
_21 = core::ptr::addr_of!(place!(Field::<char>(Variant(_107.fld5, 2), 1)));
_87.1.2 = Adt17::Variant2 { fld0: _76,fld1: _68.1.0,fld2: Field::<u8>(Variant(Field::<Adt17>(Variant(_102, 1), 4), 1), 0),fld3: _42,fld4: _8,fld5: _68.1.1 };
_105.0 = Adt21::Variant2 { fld0: Field::<u32>(Variant(_107.fld5, 2), 0),fld1: _69,fld2: Field::<f32>(Variant(_43.1, 2), 2),fld3: _89,fld4: Field::<usize>(Variant(_107.fld5, 2), 4) };
(*_7) = _57;
(*_21) = Field::<char>(Variant(_43.1, 2), 1);
(*_83) = core::ptr::addr_of!(_21);
place!(Field::<i64>(Variant(_87.1.2, 2), 5)) = _68.1.1 | _68.1.1;
_84 = Field::<usize>(Variant(_107.fld5, 2), 4) as u32;
_93.0.1 = [_43.0];
(*_21) = Field::<char>(Variant(_43.1, 2), 1);
_84 = Field::<u32>(Variant(_107.fld5, 2), 0) | Field::<u32>(Variant(_107.fld5, 2), 0);
(*_7) = -_57;
(*_51).0 = core::ptr::addr_of_mut!(_68.0.0);
(*_97) = _10 ^ _10;
_87.1.0 = (*_21);
(*_83) = core::ptr::addr_of!(_21);
(*_71) = [Field::<u32>(Variant(_105.0, 2), 0),Field::<u32>(Variant(_107.fld5, 2), 0),Field::<u32>(Variant(_107.fld5, 2), 0)];
match _19 {
234321209974641901677746360776695109210 => bb48,
_ => bb47
}
}
bb47 = {
place!(Field::<usize>(Variant(_43.1, 2), 4)) = !_26;
(*_51).0 = core::ptr::addr_of_mut!(_43.0);
_39 = _8;
_68.1.0 = Field::<char>(Variant(_43.1, 2), 1);
Call(_68.1.1 = core::intrinsics::transmute(_28), ReturnTo(bb27), UnwindUnreachable())
}
bb48 = {
_54 = Field::<u128>(Variant(_87.1.2, 2), 0);
(*_7) = _57 * _57;
_35 = &_6;
place!(Field::<usize>(Variant(_107.fld5, 2), 4)) = !_26;
(*_51).0 = core::ptr::addr_of_mut!(_52);
_68.0 = (_107.fld1,);
_111 = !_42;
(*_7) = _57 ^ _57;
_72 = !_63;
Goto(bb49)
}
bb49 = {
(*_83) = core::ptr::addr_of!(_21);
_74 = (*_97) >> (*_35);
(*_21) = _59;
(*_83) = core::ptr::addr_of!(_21);
(*_97) = !_10;
Goto(bb50)
}
bb50 = {
Call(_122 = dump_var(Move(_74), Move(_72), Move(_86), Move(_11)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_122 = dump_var(Move(_78), Move(_42), Move(_28), Move(_52)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_122 = dump_var(Move(_69), Move(_34), Move(_56), Move(_22)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_122 = dump_var(Move(_63), Move(_111), Move(_9), Move(_26)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_122 = dump_var(Move(_19), Move(_85), Move(_44), Move(_58)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_122 = dump_var(Move(_13), Move(_84), Move(_2), _123), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static i16,mut _2: i32,mut _3: *const char) -> u16 {
mir! {
type RET = u16;
let _4: *mut *mut u16;
let _5: f32;
let _6: [char; 1];
let _7: i64;
let _8: *const Adt21;
let _9: *const &'static mut [u32; 3];
let _10: *mut &'static char;
let _11: u16;
let _12: i8;
let _13: f64;
let _14: char;
let _15: f32;
let _16: &'static Adt66;
let _17: bool;
let _18: char;
let _19: (u16,);
let _20: bool;
let _21: Adt66;
let _22: *const *const char;
let _23: u64;
let _24: *const &'static mut [u32; 3];
let _25: ([u16; 5], i64);
let _26: u128;
let _27: i32;
let _28: *mut i64;
let _29: f64;
let _30: (i32, *const Adt21, &'static mut u32, u32);
let _31: f64;
let _32: f64;
let _33: *const &'static char;
let _34: u128;
let _35: char;
let _36: u128;
let _37: &'static [i128; 8];
let _38: ();
let _39: ();
{
RET = 5695756990114076756_i64 as u16;
RET = 9704_u16;
_5 = (-113_isize) as f32;
_5 = _2 as f32;
_5 = RET as f32;
_5 = 173_u8 as f32;
_5 = 245332218256014682079137910499585624043_u128 as f32;
_2 = -869422084_i32;
_5 = (-126_i8) as f32;
_7 = -4506466113856557645_i64;
_2 = !966561678_i32;
_7 = 7036217502777522403_i64;
_6 = ['\u{19532}'];
_7 = (-5245091754855509553_i64);
RET = 52783_u16 * 27742_u16;
Call(_8 = fn16(Move(_3), _2, RET, _5, _5, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-1198933194602582380_i64) * (-3442876523039960781_i64);
_5 = 24_isize as f32;
_2 = true as i32;
_11 = RET - RET;
RET = !_11;
_6 = ['\u{b7a25}'];
RET = !_11;
RET = (-106_i8) as u16;
_5 = 18085757328028598496289731463204144817_u128 as f32;
_12 = '\u{4720c}' as i8;
RET = !_11;
_11 = RET;
_12 = (-5_i8);
_7 = !(-1447158575873334674_i64);
match _12 {
0 => bb2,
340282366920938463463374607431768211451 => bb4,
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
_2 = (-2102495759_i32) | 1378705569_i32;
_2 = 2048784574_i32 & (-1198786995_i32);
_2 = 1969777837_i32;
match _2 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
1969777837 => bb9,
_ => bb8
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_7 = (-1198933194602582380_i64) * (-3442876523039960781_i64);
_5 = 24_isize as f32;
_2 = true as i32;
_11 = RET - RET;
RET = !_11;
_6 = ['\u{b7a25}'];
RET = !_11;
RET = (-106_i8) as u16;
_5 = 18085757328028598496289731463204144817_u128 as f32;
_12 = '\u{4720c}' as i8;
RET = !_11;
_11 = RET;
_12 = (-5_i8);
_7 = !(-1447158575873334674_i64);
match _12 {
0 => bb2,
340282366920938463463374607431768211451 => bb4,
_ => bb3
}
}
bb8 = {
Return()
}
bb9 = {
_7 = 1062330207249011569_i64 + (-7137087762307048108_i64);
_3 = core::ptr::addr_of!(_14);
(*_3) = '\u{b8f86}';
(*_3) = '\u{b1a0}';
(*_3) = '\u{1547c}';
(*_3) = '\u{619e0}';
_5 = _7 as f32;
(*_3) = '\u{d9b5e}';
_5 = (-7590_i16) as f32;
(*_3) = '\u{105169}';
(*_3) = '\u{10ce51}';
(*_3) = '\u{cdc59}';
(*_3) = '\u{10af23}';
(*_3) = '\u{af5a}';
_13 = (-9223372036854775808_isize) as f64;
(*_3) = '\u{b1625}';
(*_3) = '\u{3dbc2}';
(*_3) = '\u{e638b}';
(*_3) = '\u{ed7ec}';
(*_3) = '\u{17809}';
(*_3) = '\u{f5a41}';
_15 = _5;
_19 = (RET,);
(*_3) = '\u{662b0}';
(*_3) = '\u{a6acc}';
Goto(bb10)
}
bb10 = {
_15 = 2186382180_u32 as f32;
_5 = _15 - _15;
(*_3) = '\u{f3067}';
RET = _11 << _7;
_3 = core::ptr::addr_of!((*_3));
_17 = RET <= _11;
_11 = RET | RET;
(*_3) = '\u{262e9}';
_7 = !8698487083515193992_i64;
_18 = (*_3);
_13 = _7 as f64;
(*_3) = _18;
(*_3) = _18;
_20 = (*_3) < (*_3);
(*_3) = _18;
(*_3) = _18;
(*_3) = _18;
Goto(bb11)
}
bb11 = {
RET = _19.0 >> _2;
(*_3) = _18;
_22 = core::ptr::addr_of!(_3);
(*_3) = _18;
(*_22) = core::ptr::addr_of!(_14);
_17 = _11 != _19.0;
(*_3) = _18;
_23 = _5 as u64;
_27 = _17 as i32;
(*_3) = _18;
(*_22) = core::ptr::addr_of!(_18);
_18 = _14;
(*_3) = _14;
(*_3) = _14;
(*_3) = _14;
(*_22) = core::ptr::addr_of!((*_3));
_29 = -_13;
(*_22) = core::ptr::addr_of!((*_3));
(*_3) = _14;
(*_22) = core::ptr::addr_of!((*_3));
_30.0 = (*_3) as i32;
match _2 {
0 => bb3,
1 => bb12,
2 => bb13,
1969777837 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_2 = (-2102495759_i32) | 1378705569_i32;
_2 = 2048784574_i32 & (-1198786995_i32);
_2 = 1969777837_i32;
match _2 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
1969777837 => bb9,
_ => bb8
}
}
bb15 = {
(*_3) = _14;
(*_3) = _14;
(*_22) = core::ptr::addr_of!((*_3));
(*_3) = _14;
(*_3) = _14;
(*_3) = _14;
_30.1 = Move(_8);
_12 = -99_i8;
_19.0 = _11 - _11;
(*_3) = _14;
(*_22) = core::ptr::addr_of!(_14);
(*_22) = core::ptr::addr_of!(_14);
_25.1 = _7;
_15 = 62920516605914032625629749037021877904_i128 as f32;
_18 = (*_3);
(*_3) = _18;
(*_3) = _18;
_14 = _18;
(*_3) = _18;
(*_3) = _18;
_19.0 = !RET;
(*_22) = core::ptr::addr_of!((*_3));
(*_3) = _18;
(*_22) = core::ptr::addr_of!((*_3));
_23 = 9070829589456674513_u64;
_14 = _18;
Goto(bb16)
}
bb16 = {
(*_3) = _18;
(*_3) = _18;
(*_3) = _18;
Goto(bb17)
}
bb17 = {
_19 = (RET,);
_30.3 = 4249848239_u32;
_6 = [(*_3)];
(*_22) = core::ptr::addr_of!((*_3));
(*_3) = _18;
(*_22) = core::ptr::addr_of!((*_3));
_22 = core::ptr::addr_of!((*_22));
RET = !_11;
_7 = _25.1 - _25.1;
(*_3) = _18;
(*_22) = core::ptr::addr_of!((*_3));
(*_22) = core::ptr::addr_of!(_35);
_17 = _30.0 != _27;
(*_3) = _18;
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(Move(_17), Move(_20), Move(_11), Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(Move(_12), Move(_23), _39, _39), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const char,mut _2: i32,mut _3: u16,mut _4: f32,mut _5: f32,mut _6: i64) -> *const Adt21 {
mir! {
type RET = *const Adt21;
let _7: i8;
let _8: &'static Adt26;
let _9: [u32; 4];
let _10: isize;
let _11: (u16, Adt21, *mut [i16; 1]);
let _12: u64;
let _13: [i64; 2];
let _14: [char; 1];
let _15: *mut i32;
let _16: char;
let _17: Adt64;
let _18: isize;
let _19: char;
let _20: *mut u16;
let _21: Adt26;
let _22: i32;
let _23: *mut [i16; 1];
let _24: f64;
let _25: isize;
let _26: isize;
let _27: u16;
let _28: Adt26;
let _29: f32;
let _30: &'static Adt26;
let _31: i8;
let _32: [i64; 2];
let _33: u128;
let _34: isize;
let _35: u128;
let _36: &'static Adt26;
let _37: [i128; 2];
let _38: i64;
let _39: i8;
let _40: &'static mut [u32; 3];
let _41: *const &'static char;
let _42: *mut i64;
let _43: u128;
let _44: [u128; 7];
let _45: &'static mut *const *const char;
let _46: *const &'static (f32, i8, i128);
let _47: u128;
let _48: (i32, *const Adt21, &'static mut u32, u32);
let _49: *mut [i16; 1];
let _50: &'static mut isize;
let _51: &'static mut *const *const char;
let _52: char;
let _53: Adt64;
let _54: Adt64;
let _55: &'static i16;
let _56: i128;
let _57: [u32; 4];
let _58: f64;
let _59: char;
let _60: [u8; 8];
let _61: [u128; 7];
let _62: (*mut u16, &'static (char, i64, Adt17));
let _63: *mut [i16; 1];
let _64: &'static char;
let _65: isize;
let _66: (*mut *mut u16, u128);
let _67: u128;
let _68: (u16,);
let _69: *mut Adt52;
let _70: bool;
let _71: *mut &'static char;
let _72: Adt81;
let _73: [char; 1];
let _74: &'static mut isize;
let _75: u16;
let _76: bool;
let _77: [char; 1];
let _78: ((u16,), (char, i64, Adt17));
let _79: *mut i64;
let _80: f64;
let _81: char;
let _82: &'static mut *const *const char;
let _83: [u8; 7];
let _84: [u16; 1];
let _85: bool;
let _86: isize;
let _87: *mut &'static mut &'static mut u32;
let _88: Adt55;
let _89: *mut &'static mut &'static mut u32;
let _90: char;
let _91: isize;
let _92: (u16, Adt21, *mut [i16; 1]);
let _93: Adt81;
let _94: *const char;
let _95: f32;
let _96: *const char;
let _97: f64;
let _98: ((u16,), (char, i64, Adt17));
let _99: Adt21;
let _100: [i128; 2];
let _101: &'static (f32, i8, i128);
let _102: &'static (char, i64, Adt17);
let _103: &'static mut *const *const char;
let _104: *const &'static char;
let _105: u128;
let _106: char;
let _107: isize;
let _108: bool;
let _109: f64;
let _110: *const &'static (f32, i8, i128);
let _111: &'static mut isize;
let _112: (*mut u16,);
let _113: bool;
let _114: &'static mut u32;
let _115: Adt52;
let _116: [i64; 2];
let _117: f32;
let _118: (&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32);
let _119: (*const &'static (f32, i8, i128),);
let _120: bool;
let _121: (char, i64, Adt17);
let _122: &'static (char, i64, Adt17);
let _123: u64;
let _124: u16;
let _125: (*const &'static char, Adt21, (i32, *const Adt21, &'static mut u32, u32));
let _126: f64;
let _127: u64;
let _128: *const &'static (f32, i8, i128);
let _129: u16;
let _130: (u16,);
let _131: Adt17;
let _132: i128;
let _133: isize;
let _134: *const i64;
let _135: *mut Adt52;
let _136: *mut u16;
let _137: ();
let _138: ();
{
_3 = 31209_u16 + 20850_u16;
_3 = !18457_u16;
_5 = _4;
_5 = _4 + _4;
_5 = _4 + _4;
_5 = _4;
_5 = 56967480757823234544671396970352594826_u128 as f32;
_7 = 36_i8 ^ 56_i8;
_3 = 49640_u16;
_3 = !13391_u16;
_7 = -(-126_i8);
_4 = _5;
_3 = !52497_u16;
_6 = _3 as i64;
_10 = 41_isize * 9223372036854775807_isize;
_11.0 = _3 << _7;
_2 = 2012317374_i32;
_10 = -9223372036854775807_isize;
_4 = -_5;
_4 = _3 as f32;
Call(_4 = fn17(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _5 * _5;
_6 = -(-745237574738892670_i64);
_9 = [2951835639_u32,3753588946_u32,1557785644_u32,4162221749_u32];
_4 = 1009576893_u32 as f32;
_12 = 13360844506743826917_u64 * 7877277799714372203_u64;
_11.1 = Adt21::Variant0 { fld0: _10 };
Goto(bb2)
}
bb2 = {
_11.1 = Adt21::Variant0 { fld0: _10 };
RET = core::ptr::addr_of!(_11.1);
_9 = [835331379_u32,2911844770_u32,3384105120_u32,3650211497_u32];
_3 = _11.0 * _11.0;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 * _10;
_7 = (-59_i8) + 109_i8;
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
2012317374 => bb10,
_ => bb9
}
}
bb3 = {
_4 = _5 * _5;
_6 = -(-745237574738892670_i64);
_9 = [2951835639_u32,3753588946_u32,1557785644_u32,4162221749_u32];
_4 = 1009576893_u32 as f32;
_12 = 13360844506743826917_u64 * 7877277799714372203_u64;
_11.1 = Adt21::Variant0 { fld0: _10 };
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
place!(Field::<isize>(Variant((*RET), 0), 0)) = _5 as isize;
_7 = 107_i8;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 + _10;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _2 as isize;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10;
_9 = [3469784379_u32,1795248676_u32,3663052903_u32,1224916630_u32];
_2 = 213678752_i32 >> _6;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 + _10;
place!(Field::<isize>(Variant((*RET), 0), 0)) = !_10;
_6 = (-8813456355299753219_i64) - 7346227964808486672_i64;
(*RET) = Adt21::Variant0 { fld0: _10 };
_4 = _11.0 as f32;
_21.fld3 = 15188671728324697663_usize;
RET = core::ptr::addr_of!((*RET));
(*RET) = Adt21::Variant0 { fld0: _10 };
Goto(bb11)
}
bb11 = {
place!(Field::<isize>(Variant((*RET), 0), 0)) = -_10;
_21.fld4 = 246430938910596106834567752153748222548_u128 * 312387973868239106284296979098143281576_u128;
_13 = [_6,_6];
_20 = core::ptr::addr_of_mut!(_3);
_12 = 4375224736557659289_u64 | 9456714624451186562_u64;
_18 = Field::<isize>(Variant((*RET), 0), 0) | Field::<isize>(Variant((*RET), 0), 0);
_14 = ['\u{73e03}'];
_25 = Field::<isize>(Variant((*RET), 0), 0);
_22 = _2;
_15 = core::ptr::addr_of_mut!(_2);
_10 = Field::<isize>(Variant((*RET), 0), 0);
(*_15) = _21.fld3 as i32;
(*_15) = _22;
(*_15) = _22 & _22;
_22 = (*_15);
_4 = -_5;
(*_15) = _22 ^ _22;
(*_15) = _22 ^ _22;
(*RET) = Adt21::Variant0 { fld0: _25 };
_5 = -_4;
_21.fld4 = 213216986456086893401954876891335234521_u128 - 305194660161176974937644939441339423034_u128;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _18 | _25;
_28.fld4 = _21.fld4 ^ _21.fld4;
Call((*_20) = core::intrinsics::transmute(_11.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_28.fld4 = _21.fld4;
_28.fld2 = ((*_20),);
_21.fld0 = _12 - _12;
(*_15) = !_22;
_20 = core::ptr::addr_of_mut!((*_20));
place!(Field::<isize>(Variant((*RET), 0), 0)) = -_25;
_29 = -_5;
_2 = _22 - _22;
(*RET) = Adt21::Variant0 { fld0: _18 };
_19 = '\u{fd1b4}';
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 << (*_15);
_3 = _11.0 >> _2;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 | _18;
(*_20) = _11.0 >> (*_15);
place!(Field::<isize>(Variant((*RET), 0), 0)) = _18;
Goto(bb13)
}
bb13 = {
(*_15) = _22 + _22;
(*_15) = -_22;
(*_20) = !_11.0;
_5 = _29 - _29;
_29 = _5 * _4;
Call((*_15) = core::intrinsics::transmute(_22), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = !Field::<isize>(Variant((*RET), 0), 0);
_14 = [_19];
(*RET) = Adt21::Variant1 { fld0: _6,fld1: _21.fld3,fld2: _28.fld2,fld3: 118_u8 };
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = ((*_20),);
match Field::<usize>(Variant((*RET), 1), 1) {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb4,
4 => bb15,
5 => bb16,
6 => bb17,
15188671728324697663 => bb19,
_ => bb18
}
}
bb15 = {
Return()
}
bb16 = {
_11.1 = Adt21::Variant0 { fld0: _10 };
RET = core::ptr::addr_of!(_11.1);
_9 = [835331379_u32,2911844770_u32,3384105120_u32,3650211497_u32];
_3 = _11.0 * _11.0;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _10 * _10;
_7 = (-59_i8) + 109_i8;
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
2012317374 => bb10,
_ => bb9
}
}
bb17 = {
Return()
}
bb18 = {
_4 = _5 * _5;
_6 = -(-745237574738892670_i64);
_9 = [2951835639_u32,3753588946_u32,1557785644_u32,4162221749_u32];
_4 = 1009576893_u32 as f32;
_12 = 13360844506743826917_u64 * 7877277799714372203_u64;
_11.1 = Adt21::Variant0 { fld0: _10 };
Goto(bb2)
}
bb19 = {
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = (*_20) | (*_20);
_32 = _13;
_31 = (*_20) as i8;
_20 = core::ptr::addr_of_mut!((*_20));
place!(Field::<u8>(Variant((*RET), 1), 3)) = _5 as u8;
(*_15) = _22 ^ _22;
place!(Field::<u8>(Variant((*RET), 1), 3)) = !235_u8;
place!(Field::<(u16,)>(Variant(_11.1, 1), 2)).0 = (*_20) & (*_20);
place!(Field::<i64>(Variant(_11.1, 1), 0)) = _6;
_21.fld5 = (*RET);
_35 = _21.fld4;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (Field::<(u16,)>(Variant(_21.fld5, 1), 2).0,);
(*_15) = _22;
place!(Field::<i64>(Variant((*RET), 1), 0)) = _6 | _6;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = ((*_20),);
_28.fld2 = Field::<(u16,)>(Variant(_21.fld5, 1), 2);
_20 = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_11.0,);
_16 = _19;
(*_15) = _22;
place!(Field::<u8>(Variant((*RET), 1), 3)) = Field::<u8>(Variant(_21.fld5, 1), 3) | Field::<u8>(Variant(_21.fld5, 1), 3);
place!(Field::<i64>(Variant((*RET), 1), 0)) = Field::<i64>(Variant(_21.fld5, 1), 0) - Field::<i64>(Variant(_21.fld5, 1), 0);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_3,);
place!(Field::<u8>(Variant((*RET), 1), 3)) = Field::<u8>(Variant(_21.fld5, 1), 3) << Field::<i64>(Variant((*RET), 1), 0);
place!(Field::<i64>(Variant((*RET), 1), 0)) = Field::<i64>(Variant(_21.fld5, 1), 0) + Field::<i64>(Variant(_21.fld5, 1), 0);
_21.fld3 = Field::<usize>(Variant((*RET), 1), 1) | Field::<usize>(Variant((*RET), 1), 1);
match Field::<usize>(Variant((*RET), 1), 1) {
0 => bb1,
1 => bb6,
15188671728324697663 => bb20,
_ => bb18
}
}
bb20 = {
place!(Field::<i64>(Variant((*RET), 1), 0)) = _6;
place!(Field::<usize>(Variant(_21.fld5, 1), 1)) = Field::<usize>(Variant((*RET), 1), 1);
(*_15) = -_22;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _28.fld2.0 - _28.fld2.0;
place!(Field::<usize>(Variant((*RET), 1), 1)) = Field::<usize>(Variant(_21.fld5, 1), 1) * Field::<usize>(Variant(_21.fld5, 1), 1);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (Field::<(u16,)>(Variant(_21.fld5, 1), 2).0,);
_27 = Field::<(u16,)>(Variant((*RET), 1), 2).0 >> Field::<usize>(Variant((*RET), 1), 1);
place!(Field::<i64>(Variant((*RET), 1), 0)) = !_6;
place!(Field::<u8>(Variant((*RET), 1), 3)) = (*_15) as u8;
_28.fld5 = Adt21::Variant0 { fld0: _10 };
(*RET) = _28.fld5;
(*_15) = -_22;
place!(Field::<usize>(Variant(_21.fld5, 1), 1)) = !_21.fld3;
(*_15) = _35 as i32;
place!(Field::<isize>(Variant(_28.fld5, 0), 0)) = Field::<isize>(Variant((*RET), 0), 0);
(*RET) = Adt21::Variant1 { fld0: _6,fld1: _21.fld3,fld2: _28.fld2,fld3: Field::<u8>(Variant(_21.fld5, 1), 3) };
Goto(bb21)
}
bb21 = {
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = Field::<(u16,)>(Variant(_21.fld5, 1), 2).0 | _27;
(*_15) = _22 * _22;
place!(Field::<u8>(Variant((*RET), 1), 3)) = !Field::<u8>(Variant(_21.fld5, 1), 3);
place!(Field::<usize>(Variant((*RET), 1), 1)) = !_21.fld3;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = !_27;
place!(Field::<u8>(Variant((*RET), 1), 3)) = 23003775499552121179152929954076723399_i128 as u8;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_28.fld2.0,);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = Field::<(u16,)>(Variant(_21.fld5, 1), 2);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = _28.fld2;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_27,);
_28.fld0 = _21.fld0 - _12;
(*RET) = _21.fld5;
_28.fld1 = !_27;
(*RET) = Adt21::Variant0 { fld0: _18 };
_28.fld3 = _3 as usize;
(*_15) = _22 - _22;
place!(Field::<isize>(Variant((*RET), 0), 0)) = Field::<usize>(Variant(_21.fld5, 1), 1) as isize;
_37 = [12362541715902243404775072484768918599_i128,(-77736975963980056253376285797149230426_i128)];
_5 = _4 - _29;
_38 = _6;
place!(Field::<isize>(Variant((*RET), 0), 0)) = _25;
_42 = core::ptr::addr_of_mut!(_38);
(*_15) = _22 * _22;
(*RET) = _21.fld5;
_11.1 = Adt21::Variant1 { fld0: (*_42),fld1: Field::<usize>(Variant(_21.fld5, 1), 1),fld2: Field::<(u16,)>(Variant(_21.fld5, 1), 2),fld3: Field::<u8>(Variant(_21.fld5, 1), 3) };
Call(_18 = core::intrinsics::transmute(Field::<i64>(Variant((*RET), 1), 0)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
place!(Field::<usize>(Variant((*RET), 1), 1)) = _18 as usize;
place!(Field::<i64>(Variant((*RET), 1), 0)) = (*_42) + (*_42);
place!(Field::<usize>(Variant((*RET), 1), 1)) = _31 as usize;
place!(Field::<i64>(Variant((*RET), 1), 0)) = -(*_42);
place!(Field::<usize>(Variant((*RET), 1), 1)) = (-16880_i16) as usize;
place!(Field::<i64>(Variant((*RET), 1), 0)) = Field::<i64>(Variant(_21.fld5, 1), 0) - (*_42);
place!(Field::<isize>(Variant(_28.fld5, 0), 0)) = !_10;
place!(Field::<i64>(Variant((*RET), 1), 0)) = (*_42) + (*_42);
place!(Field::<usize>(Variant((*RET), 1), 1)) = _28.fld3 << _3;
(*RET) = _21.fld5;
place!(Field::<i64>(Variant((*RET), 1), 0)) = (*_42) >> (*_15);
_7 = _31 ^ _31;
place!(Field::<usize>(Variant((*RET), 1), 1)) = Field::<usize>(Variant(_21.fld5, 1), 1) * _28.fld3;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (Field::<(u16,)>(Variant(_21.fld5, 1), 2).0,);
place!(Field::<usize>(Variant((*RET), 1), 1)) = false as usize;
(*_42) = _19 as i64;
place!(Field::<usize>(Variant((*RET), 1), 1)) = Field::<usize>(Variant(_21.fld5, 1), 1) ^ Field::<usize>(Variant(_21.fld5, 1), 1);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _28.fld2.0;
place!(Field::<i64>(Variant(_21.fld5, 1), 0)) = Field::<i64>(Variant((*RET), 1), 0) >> Field::<usize>(Variant((*RET), 1), 1);
(*_15) = _22 ^ _22;
Goto(bb23)
}
bb23 = {
_21.fld1 = _5 as u16;
place!(Field::<u8>(Variant((*RET), 1), 3)) = _21.fld0 as u8;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_28.fld2.0,);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _28.fld1;
_33 = _21.fld4;
(*RET) = Adt21::Variant0 { fld0: Field::<isize>(Variant(_28.fld5, 0), 0) };
(*RET) = _21.fld5;
Goto(bb24)
}
bb24 = {
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _28.fld2.0 + _27;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = false as u16;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _27;
_21.fld2 = (Field::<(u16,)>(Variant((*RET), 1), 2).0,);
(*_42) = Field::<i64>(Variant((*RET), 1), 0) * Field::<i64>(Variant((*RET), 1), 0);
(*_42) = Field::<i64>(Variant((*RET), 1), 0);
(*_15) = Field::<(u16,)>(Variant((*RET), 1), 2).0 as i32;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = 3024333486_u32 as u16;
place!(Field::<usize>(Variant((*RET), 1), 1)) = !_21.fld3;
place!(Field::<i64>(Variant((*RET), 1), 0)) = (*_42);
_26 = !_25;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (Field::<(u16,)>(Variant(_21.fld5, 1), 2).0,);
_48.3 = _16 as u32;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = true as u16;
(*RET) = _21.fld5;
(*_15) = _22;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = _28.fld2.0 ^ _27;
Goto(bb25)
}
bb25 = {
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_11.0,);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_27,);
(*_42) = !Field::<i64>(Variant((*RET), 1), 0);
place!(Field::<(u16,)>(Variant(_21.fld5, 1), 2)) = Field::<(u16,)>(Variant((*RET), 1), 2);
(*_42) = Field::<i64>(Variant((*RET), 1), 0) & Field::<i64>(Variant((*RET), 1), 0);
place!(Field::<usize>(Variant(_11.1, 1), 1)) = !_28.fld3;
(*_42) = true as i64;
_48.2 = &mut _48.3;
_21.fld2.0 = Field::<(u16,)>(Variant((*RET), 1), 2).0;
(*_42) = Field::<i64>(Variant((*RET), 1), 0) ^ Field::<i64>(Variant((*RET), 1), 0);
place!(Field::<usize>(Variant((*RET), 1), 1)) = _28.fld3 ^ _28.fld3;
_21.fld4 = _33;
place!(Field::<u8>(Variant((*RET), 1), 3)) = !Field::<u8>(Variant(_21.fld5, 1), 3);
place!(Field::<u8>(Variant((*RET), 1), 3)) = !Field::<u8>(Variant(_21.fld5, 1), 3);
(*_15) = _22 * _22;
(*RET) = _21.fld5;
place!(Field::<u8>(Variant((*RET), 1), 3)) = Field::<u8>(Variant(_21.fld5, 1), 3) - Field::<u8>(Variant(_21.fld5, 1), 3);
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = !_28.fld2.0;
(*RET) = _21.fld5;
place!(Field::<u8>(Variant((*RET), 1), 3)) = _28.fld0 as u8;
_28.fld3 = Field::<usize>(Variant((*RET), 1), 1) << (*_42);
_14 = [_16];
(*_42) = Field::<i64>(Variant((*RET), 1), 0);
(*RET) = _21.fld5;
(*RET) = _28.fld5;
_28.fld3 = _35 as usize;
Goto(bb26)
}
bb26 = {
_26 = Field::<isize>(Variant((*RET), 0), 0) * Field::<isize>(Variant((*RET), 0), 0);
_59 = _19;
_28 = Adt26 { fld0: _21.fld0,fld1: _3,fld2: _21.fld2,fld3: Field::<usize>(Variant(_21.fld5, 1), 1),fld4: _35,fld5: (*RET) };
_60 = [Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3),Field::<u8>(Variant(_21.fld5, 1), 3)];
_21.fld1 = !_28.fld2.0;
(*RET) = _21.fld5;
_58 = _28.fld0 as f64;
_21 = _28;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)).0 = 2426797437_u32 as u16;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = _28.fld2;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = (_21.fld2.0,);
_50 = &mut _10;
place!(Field::<u8>(Variant((*RET), 1), 3)) = !226_u8;
place!(Field::<(u16,)>(Variant((*RET), 1), 2)) = _28.fld2;
_56 = _58 as i128;
place!(Field::<u8>(Variant((*RET), 1), 3)) = 2818939957_u32 as u8;
Call(place!(Field::<i64>(Variant((*RET), 1), 0)) = core::intrinsics::bswap((*_42)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_11.0 = !Field::<(u16,)>(Variant((*RET), 1), 2).0;
(*RET) = Adt21::Variant0 { fld0: (*_50) };
(*_15) = _22;
(*RET) = Adt21::Variant2 { fld0: 306338138_u32,fld1: _19,fld2: _5,fld3: _58,fld4: _28.fld3 };
_20 = core::ptr::addr_of_mut!(_28.fld2.0);
_31 = _7;
_21.fld2.0 = (*_20) | (*_20);
(*RET) = Adt21::Variant0 { fld0: (*_50) };
(*_42) = _6 & _6;
_57 = [1112365454_u32,2789295317_u32,3885116309_u32,714090824_u32];
_37 = [_56,_56];
place!(Field::<isize>(Variant((*RET), 0), 0)) = (*_50) + (*_50);
(*_20) = !_21.fld1;
_59 = _16;
(*_20) = !_27;
(*_42) = _6;
(*_50) = 47_u8 as isize;
place!(Field::<isize>(Variant((*RET), 0), 0)) = 149_u8 as isize;
_59 = _19;
_12 = _7 as u64;
_28.fld2 = (_11.0,);
(*_20) = !_21.fld2.0;
_20 = core::ptr::addr_of_mut!((*_20));
(*_15) = _22 | _22;
place!(Field::<isize>(Variant((*RET), 0), 0)) = (*_50) + Field::<isize>(Variant(_21.fld5, 0), 0);
Goto(bb28)
}
bb28 = {
(*_20) = _21.fld2.0 >> (*_15);
_44 = [_33,_35,_35,_21.fld4,_28.fld4,_35,_35];
(*_42) = _6 | _6;
Goto(bb29)
}
bb29 = {
(*_20) = _21.fld2.0 + _21.fld2.0;
(*_15) = _22 + _22;
(*_42) = _6;
(*_50) = 7047_i16 as isize;
Call((*_42) = core::intrinsics::bswap(_6), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*RET) = _28.fld5;
place!(Field::<isize>(Variant((*RET), 0), 0)) = !Field::<isize>(Variant(_21.fld5, 0), 0);
(*_20) = _58 as u16;
_47 = _33 * _21.fld4;
_66.1 = _47 << (*_20);
(*_42) = _6;
place!(Field::<isize>(Variant((*RET), 0), 0)) = (*_50);
(*_42) = 3241674092_u32 as i64;
_32 = [(*_42),(*_42)];
_21.fld2.0 = (*_20);
place!(Field::<isize>(Variant((*RET), 0), 0)) = Field::<isize>(Variant(_21.fld5, 0), 0);
_15 = core::ptr::addr_of_mut!((*_15));
(*_20) = !_11.0;
place!(Field::<isize>(Variant((*RET), 0), 0)) = (*_50);
(*_20) = _28.fld1 | _27;
_72.fld2.0 = Field::<isize>(Variant((*RET), 0), 0) as f32;
_65 = _21.fld3 as isize;
Goto(bb31)
}
bb31 = {
place!(Field::<isize>(Variant((*RET), 0), 0)) = (*_50) << (*_15);
_6 = (*_42) - (*_42);
(*RET) = Adt21::Variant0 { fld0: _26 };
_25 = (*_50);
_34 = !Field::<isize>(Variant((*RET), 0), 0);
place!(Field::<isize>(Variant(_21.fld5, 0), 0)) = Field::<isize>(Variant((*RET), 0), 0) >> (*_15);
_12 = _21.fld0;
(*RET) = _21.fld5;
_70 = _28.fld2.0 < (*_20);
_11.1 = Adt21::Variant2 { fld0: 3202898531_u32,fld1: _16,fld2: _29,fld3: _58,fld4: _21.fld3 };
(*RET) = Adt21::Variant2 { fld0: 1034247817_u32,fld1: _19,fld2: _29,fld3: _58,fld4: _21.fld3 };
(*_15) = Field::<f32>(Variant((*RET), 2), 2) as i32;
place!(Field::<f64>(Variant((*RET), 2), 3)) = _56 as f64;
(*_15) = (-2183_i16) as i32;
place!(Field::<u32>(Variant((*RET), 2), 0)) = _27 as u32;
place!(Field::<f64>(Variant((*RET), 2), 3)) = -_58;
place!(Field::<u32>(Variant(_11.1, 2), 0)) = !3118889147_u32;
place!(Field::<u32>(Variant((*RET), 2), 0)) = _21.fld0 as u32;
(*_15) = 169_u8 as i32;
_60 = [179_u8,72_u8,61_u8,245_u8,17_u8,122_u8,41_u8,187_u8];
Call(_57 = core::intrinsics::transmute(_32), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
place!(Field::<f64>(Variant((*RET), 2), 3)) = _58 - _58;
place!(Field::<usize>(Variant((*RET), 2), 4)) = _21.fld3 | _28.fld3;
(*_15) = _22 & _22;
place!(Field::<f64>(Variant((*RET), 2), 3)) = _58 + _58;
_72.fld5 = Adt52::Variant0 { fld0: _26 };
place!(Field::<f32>(Variant(_11.1, 2), 2)) = _66.1 as f32;
_68.0 = !(*_20);
_72.fld1.0 = ((*_20),);
(*_20) = _68.0 << (*_15);
_32 = [(*_42),(*_42)];
place!(Field::<f32>(Variant((*RET), 2), 2)) = Field::<usize>(Variant((*RET), 2), 4) as f32;
place!(Field::<u32>(Variant((*RET), 2), 0)) = 3727552588_u32 | 368147836_u32;
place!(Field::<f32>(Variant((*RET), 2), 2)) = _5 + _4;
(*_50) = Field::<isize>(Variant(_72.fld5, 0), 0) & _34;
_6 = (*_42);
place!(Field::<f64>(Variant(_11.1, 2), 3)) = _58 - _58;
place!(Field::<u32>(Variant((*RET), 2), 0)) = 1556208334_u32 >> (*_20);
_12 = _28.fld0 | _28.fld0;
place!(Field::<usize>(Variant((*RET), 2), 4)) = _21.fld3;
_47 = _21.fld4;
place!(Field::<f64>(Variant((*RET), 2), 3)) = -_58;
place!(Field::<f32>(Variant((*RET), 2), 2)) = _5 * _29;
place!(Field::<char>(Variant((*RET), 2), 1)) = _19;
place!(Field::<f32>(Variant((*RET), 2), 2)) = -_72.fld2.0;
_62.0 = core::ptr::addr_of_mut!((*_20));
_58 = Field::<f64>(Variant((*RET), 2), 3) + Field::<f64>(Variant((*RET), 2), 3);
place!(Field::<f64>(Variant((*RET), 2), 3)) = _58;
_37 = [_56,_56];
Call(place!(Field::<usize>(Variant((*RET), 2), 4)) = core::intrinsics::bswap(_28.fld3), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
(*_20) = _21.fld2.0;
place!(Field::<f32>(Variant(_11.1, 2), 2)) = _4 * _5;
(*_15) = _22;
place!(Field::<char>(Variant((*RET), 2), 1)) = _19;
place!(Field::<usize>(Variant((*RET), 2), 4)) = _28.fld3 + _21.fld3;
place!(Field::<f32>(Variant((*RET), 2), 2)) = -_72.fld2.0;
(*RET) = Adt21::Variant1 { fld0: (*_42),fld1: _21.fld3,fld2: _72.fld1.0,fld3: 228_u8 };
(*RET) = Adt21::Variant1 { fld0: (*_42),fld1: _21.fld3,fld2: _72.fld1.0,fld3: 90_u8 };
(*RET) = Adt21::Variant0 { fld0: (*_50) };
(*RET) = _21.fld5;
_47 = _33;
_28.fld0 = _21.fld0 * _12;
(*_20) = !_28.fld1;
(*_20) = (*_42) as u16;
_33 = _7 as u128;
Goto(bb34)
}
bb34 = {
_72.fld1.1.2 = Adt17::Variant0 { fld0: _58,fld1: _19,fld2: (*_50),fld3: _28.fld4,fld4: 29822_i16,fld5: _72.fld2.0 };
_8 = &_28;
_28.fld0 = !_21.fld0;
_21.fld4 = !(*_8).fld4;
(*_20) = !(*_8).fld1;
_65 = !Field::<isize>(Variant((*RET), 0), 0);
(*_42) = _6;
_35 = (*_8).fld4 - (*_8).fld4;
_34 = -(*_50);
(*_50) = _31 as isize;
(*_50) = !Field::<isize>(Variant((*RET), 0), 0);
(*_50) = Field::<isize>(Variant((*RET), 0), 0);
Goto(bb35)
}
bb35 = {
(*_15) = -_22;
_72.fld2 = (Field::<f32>(Variant(_72.fld1.1.2, 0), 5), Field::<f64>(Variant(_72.fld1.1.2, 0), 0), (*_42));
(*_15) = _22 * _22;
_79 = core::ptr::addr_of_mut!((*_42));
(*_50) = Field::<isize>(Variant((*RET), 0), 0) << Field::<isize>(Variant((*RET), 0), 0);
_28.fld5 = Adt21::Variant1 { fld0: (*_42),fld1: (*_8).fld3,fld2: _72.fld1.0,fld3: 147_u8 };
_65 = Field::<isize>(Variant((*RET), 0), 0) * (*_50);
(*_42) = (*_50) as i64;
(*_15) = _22 | _22;
_28.fld3 = Field::<usize>(Variant(_28.fld5, 1), 1) * _21.fld3;
(*_42) = !_6;
_74 = &mut _25;
(*RET) = Adt21::Variant0 { fld0: (*_50) };
_85 = (*_50) < Field::<isize>(Variant((*RET), 0), 0);
(*_15) = _47 as i32;
(*_20) = !_72.fld1.0.0;
Goto(bb36)
}
bb36 = {
(*RET) = Adt21::Variant0 { fld0: (*_50) };
_14 = [_19];
_52 = Field::<char>(Variant(_72.fld1.1.2, 0), 1);
_44 = [_35,(*_8).fld4,(*_8).fld4,Field::<u128>(Variant(_72.fld1.1.2, 0), 3),_66.1,(*_8).fld4,(*_8).fld4];
_30 = &_21;
_90 = _59;
_21.fld1 = (*_20);
(*_20) = _72.fld1.0.0;
_21.fld0 = _12;
_83 = [95_u8,169_u8,23_u8,64_u8,233_u8,188_u8,240_u8];
_72.fld1.1.2 = Adt17::Variant0 { fld0: _72.fld2.1,fld1: _90,fld2: (*_50),fld3: (*_30).fld4,fld4: 2032_i16,fld5: _5 };
_74 = &mut place!(Field::<isize>(Variant((*RET), 0), 0));
(*_42) = _90 as i64;
(*_20) = (*_30).fld2.0;
(*_20) = !(*_8).fld1;
(*_42) = _6 * _72.fld2.2;
_74 = &mut (*_50);
_27 = (*_30).fld2.0 * (*_30).fld2.0;
_72.fld2.2 = (*_42);
(*_15) = _22 + _22;
(*_15) = Field::<isize>(Variant(_21.fld5, 0), 0) as i32;
_72.fld1.0 = (*_30).fld2;
Goto(bb37)
}
bb37 = {
_77 = _14;
(*_42) = -_72.fld2.2;
_73 = [_16];
_22 = (*_15) & (*_15);
_22 = -(*_15);
Goto(bb38)
}
bb38 = {
_90 = _19;
_92.1 = Adt21::Variant2 { fld0: 2274251499_u32,fld1: Field::<char>(Variant(_72.fld1.1.2, 0), 1),fld2: _72.fld2.0,fld3: _72.fld2.1,fld4: (*_30).fld3 };
(*_42) = _72.fld2.2;
_21.fld2.0 = (*_8).fld1 ^ (*_8).fld1;
_93.fld2.0 = _5 + Field::<f32>(Variant(_72.fld1.1.2, 0), 5);
_78.0 = (_3,);
_72.fld2 = (_29, _58, (*_42));
_36 = &_21;
(*_74) = Field::<isize>(Variant((*_30).fld5, 0), 0) ^ Field::<isize>(Variant((*_30).fld5, 0), 0);
_72.fld1.1.0 = Field::<char>(Variant(_72.fld1.1.2, 0), 1);
_9 = [3483385943_u32,2651828796_u32,1555158278_u32,102616188_u32];
place!(Field::<isize>(Variant(_72.fld5, 0), 0)) = (*_74) + Field::<isize>(Variant((*_36).fld5, 0), 0);
_93.fld2.1 = _58;
(*_42) = Field::<i64>(Variant(_28.fld5, 1), 0) ^ _72.fld2.2;
_20 = Move(_62.0);
_72.fld4.0 = core::ptr::addr_of_mut!(_20);
place!(Field::<u32>(Variant(_92.1, 2), 0)) = !2593329956_u32;
place!(Field::<u8>(Variant(_28.fld5, 1), 3)) = 199_u8 >> Field::<usize>(Variant(_28.fld5, 1), 1);
_66 = (Move(_72.fld4.0), (*_36).fld4);
place!(Field::<(u16,)>(Variant(_28.fld5, 1), 2)) = _78.0;
_84 = [(*_8).fld1];
_30 = &_28;
(*_15) = _22;
(*_74) = Field::<usize>(Variant((*_30).fld5, 1), 1) as isize;
Goto(bb39)
}
bb39 = {
_86 = Field::<isize>(Variant((*_36).fld5, 0), 0) + Field::<isize>(Variant((*_36).fld5, 0), 0);
_14 = _77;
_72.fld4.1 = !(*_8).fld4;
_85 = _70 ^ _70;
(*_42) = Field::<i64>(Variant((*_30).fld5, 1), 0);
(*_15) = _22 ^ _22;
place!(Field::<u32>(Variant(_92.1, 2), 0)) = 1889423642_u32;
_69 = core::ptr::addr_of_mut!(_93.fld5);
_91 = (*_30).fld0 as isize;
Goto(bb40)
}
bb40 = {
(*_69) = Move(_72.fld5);
(*_42) = (*_30).fld3 as i64;
place!(Field::<isize>(Variant(_72.fld1.1.2, 0), 2)) = Field::<isize>(Variant((*_36).fld5, 0), 0) ^ Field::<isize>(Variant((*_36).fld5, 0), 0);
place!(Field::<f32>(Variant(_92.1, 2), 2)) = _5 - _93.fld2.0;
(*_74) = Field::<isize>(Variant((*_36).fld5, 0), 0) << (*_42);
_78.0.0 = (*_30).fld2.0 >> Field::<isize>(Variant((*_36).fld5, 0), 0);
(*_42) = Field::<i64>(Variant((*_30).fld5, 1), 0);
_93.fld1.1.0 = _16;
(*_74) = Field::<isize>(Variant((*_69), 0), 0) | Field::<isize>(Variant((*_69), 0), 0);
_81 = _72.fld1.1.0;
_43 = !(*_30).fld4;
_92.1 = Adt21::Variant1 { fld0: (*_42),fld1: _28.fld3,fld2: (*_36).fld2,fld3: Field::<u8>(Variant((*_30).fld5, 1), 3) };
_84 = [(*_8).fld1];
_98.0 = Field::<(u16,)>(Variant((*_30).fld5, 1), 2);
_22 = (*_15);
place!(Field::<i64>(Variant(_28.fld5, 1), 0)) = _72.fld2.1 as i64;
(*_15) = _22 >> (*_30).fld3;
_93.fld4 = Move(_66);
_91 = -(*_74);
_30 = &(*_36);
place!(Field::<usize>(Variant(_92.1, 1), 1)) = !_28.fld3;
Goto(bb41)
}
bb41 = {
_13 = [(*_42),Field::<i64>(Variant(_28.fld5, 1), 0)];
_19 = _16;
_60 = [Field::<u8>(Variant(_92.1, 1), 3),Field::<u8>(Variant(_28.fld5, 1), 3),Field::<u8>(Variant(_92.1, 1), 3),Field::<u8>(Variant(_92.1, 1), 3),Field::<u8>(Variant(_28.fld5, 1), 3),Field::<u8>(Variant(_28.fld5, 1), 3),Field::<u8>(Variant(_28.fld5, 1), 3),Field::<u8>(Variant(_28.fld5, 1), 3)];
_93.fld4.1 = (*_15) as u128;
_37 = [_56,_56];
place!(Field::<i16>(Variant(_72.fld1.1.2, 0), 4)) = (-25006_i16) >> (*_74);
(*_42) = _72.fld2.2 ^ Field::<i64>(Variant(_92.1, 1), 0);
place!(Field::<(u16,)>(Variant(_92.1, 1), 2)) = (*_30).fld2;
_93.fld2.2 = (*_42) ^ Field::<i64>(Variant(_28.fld5, 1), 0);
_21.fld3 = _28.fld3 | Field::<usize>(Variant(_28.fld5, 1), 1);
(*_74) = Field::<isize>(Variant((*_69), 0), 0) * Field::<isize>(Variant((*_69), 0), 0);
_39 = _7 - _31;
(*_42) = Field::<i64>(Variant(_92.1, 1), 0) - _93.fld2.2;
_64 = &place!(Field::<char>(Variant(_72.fld1.1.2, 0), 1));
Goto(bb42)
}
bb42 = {
_28.fld2 = ((*_8).fld1,);
(*_74) = !Field::<isize>(Variant((*_69), 0), 0);
_21.fld0 = _28.fld0 << (*_15);
_72.fld1.1.2 = Adt17::Variant1 { fld0: Field::<u8>(Variant(_28.fld5, 1), 3) };
place!(Field::<isize>(Variant((*_69), 0), 0)) = Field::<isize>(Variant((*_30).fld5, 0), 0) - (*_74);
_92.0 = (*_36).fld1;
(*_15) = _22 | _22;
(*_15) = _22 * _22;
_93.fld1.0.0 = (*_30).fld1 - (*_36).fld1;
place!(Field::<(u16,)>(Variant(_92.1, 1), 2)).0 = !(*_36).fld1;
_80 = -_58;
(*_69) = Adt52::Variant2 { fld0: Move(_93.fld4.0),fld1: _14 };
_97 = _80 + _80;
_66.0 = core::ptr::addr_of_mut!(_112.0);
_64 = &_59;
_72.fld1.1.0 = (*_64);
(*_74) = Field::<isize>(Variant((*_36).fld5, 0), 0) ^ Field::<isize>(Variant((*_36).fld5, 0), 0);
_115 = Adt52::Variant2 { fld0: Move(Field::<*mut *mut u16>(Variant((*_69), 2), 0)),fld1: Field::<[char; 1]>(Variant((*_69), 2), 1) };
_76 = Field::<isize>(Variant((*_30).fld5, 0), 0) < Field::<isize>(Variant((*_30).fld5, 0), 0);
_81 = (*_64);
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_62.0);
_81 = (*_64);
place!(Field::<[char; 1]>(Variant((*_69), 2), 1)) = [_90];
_115 = Adt52::Variant2 { fld0: Move(Field::<*mut *mut u16>(Variant((*_69), 2), 0)),fld1: Field::<[char; 1]>(Variant((*_69), 2), 1) };
place!(Field::<isize>(Variant(_21.fld5, 0), 0)) = _91;
_65 = (*_15) as isize;
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_112.0);
_15 = core::ptr::addr_of_mut!((*_15));
_22 = (*_15);
Goto(bb43)
}
bb43 = {
_93.fld1.0.0 = (*_36).fld1;
(*_69) = Move(_115);
_98.1.2 = Adt17::Variant0 { fld0: _97,fld1: (*_64),fld2: (*_74),fld3: (*_36).fld4,fld4: (-20442_i16),fld5: _4 };
place!(Field::<[char; 1]>(Variant((*_69), 2), 1)) = [(*_64)];
(*_15) = _22 ^ _22;
_93.fld4 = (Move(Field::<*mut *mut u16>(Variant(_93.fld5, 2), 0)), (*_36).fld4);
Call((*_42) = core::intrinsics::bswap(_93.fld2.2), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
(*_15) = _22 >> (*_74);
_112 = (Move(_20),);
place!(Field::<u128>(Variant(_98.1.2, 0), 3)) = (*_8).fld4;
_31 = Field::<u8>(Variant(_92.1, 1), 3) as i8;
(*_69) = Adt52::Variant2 { fld0: Move(_93.fld4.0),fld1: _77 };
_93.fld0 = _85 & _85;
_22 = -(*_15);
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_20);
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_112.0);
_57 = _9;
place!(Field::<[char; 1]>(Variant((*_69), 2), 1)) = [(*_64)];
(*_74) = -_91;
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_112.0);
Goto(bb45)
}
bb45 = {
(*_15) = _22 - _22;
place!(Field::<[char; 1]>(Variant((*_69), 2), 1)) = [(*_64)];
_93.fld4.0 = core::ptr::addr_of_mut!(_20);
(*_69) = Adt52::Variant2 { fld0: Move(_93.fld4.0),fld1: _73 };
_20 = core::ptr::addr_of_mut!((*_30).fld2.0);
_98.1.2 = Adt17::Variant1 { fld0: Field::<u8>(Variant(_72.fld1.1.2, 1), 0) };
(*_69) = Adt52::Variant0 { fld0: (*_74) };
_72.fld1.0.0 = (*_36).fld1;
_72.fld4.0 = Move(_66.0);
_70 = _85;
_42 = Move(_79);
_96 = core::ptr::addr_of!(_98.1.0);
_65 = !Field::<isize>(Variant((*_69), 0), 0);
place!(Field::<isize>(Variant((*_69), 0), 0)) = (*_74);
_121.2 = Move(_72.fld1.1.2);
place!(Field::<isize>(Variant((*_69), 0), 0)) = Field::<isize>(Variant(_21.fld5, 0), 0);
Goto(bb46)
}
bb46 = {
_72.fld2 = _93.fld2;
_112.0 = core::ptr::addr_of_mut!(_98.0.0);
_72.fld2 = (_93.fld2.0, _93.fld2.1, _6);
_66.1 = (*_36).fld4 >> Field::<isize>(Variant((*_69), 0), 0);
_124 = !(*_36).fld2.0;
(*_15) = !_22;
_61 = _44;
_90 = (*_64);
_93.fld4.1 = !(*_30).fld4;
Goto(bb47)
}
bb47 = {
(*_96) = (*_64);
place!(Field::<isize>(Variant((*_69), 0), 0)) = -(*_74);
_85 = _93.fld0;
(*_69) = Adt52::Variant2 { fld0: Move(_72.fld4.0),fld1: _14 };
Goto(bb48)
}
bb48 = {
_72.fld4 = (Move(Field::<*mut *mut u16>(Variant((*_69), 2), 0)), _66.1);
_98.1.2 = Adt17::Variant3 { fld0: _72.fld4.1,fld1: (*_64),fld2: 3606346709_u32,fld3: _80,fld4: _21.fld0,fld5: (*_15),fld6: Field::<i64>(Variant(_28.fld5, 1), 0),fld7: _56 };
_66 = Move(_72.fld4);
_72.fld1.1 = (_90, _38, Move(_121.2));
_118.0 = &_72.fld1.1;
_9 = [2069824746_u32,4034740369_u32,3878264246_u32,3259439247_u32];
(*_69) = Adt52::Variant2 { fld0: Move(_66.0),fld1: _77 };
(*_74) = _91;
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_62.0);
_24 = _31 as f64;
_93.fld1.0.0 = (*_20) | (*_8).fld1;
_78.0.0 = _93.fld0 as u16;
_99 = Adt21::Variant1 { fld0: _93.fld2.2,fld1: _28.fld3,fld2: (*_36).fld2,fld3: Field::<u8>(Variant(_72.fld1.1.2, 1), 0) };
Goto(bb49)
}
bb49 = {
_118.1 = [(*_36).fld1];
(*_96) = (*_64);
_122 = &_72.fld1.1;
_67 = !_66.1;
place!(Field::<*mut *mut u16>(Variant((*_69), 2), 0)) = core::ptr::addr_of_mut!(_20);
(*_15) = _22;
_93.fld1 = Move(_72.fld1);
_98.1.0 = Field::<char>(Variant(_98.1.2, 3), 1);
_78.1.0 = (*_96);
(*_69) = Adt52::Variant0 { fld0: (*_74) };
place!(Field::<(u16,)>(Variant(_99, 1), 2)).0 = (*_30).fld1;
_59 = (*_96);
_93.fld4.0 = core::ptr::addr_of_mut!(_112.0);
_68.0 = (*_36).fld1 >> (*_30).fld4;
_129 = (*_30).fld2.0 | (*_36).fld2.0;
_121.0 = _16;
_13 = [Field::<i64>(Variant(_98.1.2, 3), 6),Field::<i64>(Variant(_28.fld5, 1), 0)];
_98.1 = (_90, Field::<i64>(Variant(_99, 1), 0), Move(_93.fld1.1.2));
_122 = &_98.1;
_73 = _14;
_133 = Field::<isize>(Variant((*_69), 0), 0);
_31 = _39 | _7;
Goto(bb50)
}
bb50 = {
Call(_137 = dump_var(Move(_129), Move(_31), Move(_26), Move(_6)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_137 = dump_var(Move(_90), Move(_60), Move(_67), Move(_35)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_137 = dump_var(Move(_39), Move(_83), Move(_56), Move(_37)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_137 = dump_var(Move(_43), Move(_52), Move(_9), Move(_68)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_137 = dump_var(Move(_84), Move(_27), Move(_22), Move(_10)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_137 = dump_var(Move(_34), Move(_16), Move(_124), Move(_38)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_137 = dump_var(Move(_47), _138, _138, _138), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17() -> f32 {
mir! {
type RET = f32;
let _1: i64;
let _2: &'static (char, i64, Adt17);
let _3: *mut [i16; 1];
let _4: [u32; 3];
let _5: &'static mut *const *const char;
let _6: u8;
let _7: i64;
let _8: f64;
let _9: bool;
let _10: i128;
let _11: &'static [i128; 8];
let _12: (*mut u16, &'static (char, i64, Adt17));
let _13: i32;
let _14: [u8; 8];
let _15: f32;
let _16: &'static Adt26;
let _17: &'static mut (*mut u16,);
let _18: &'static char;
let _19: u8;
let _20: [i64; 2];
let _21: *mut u16;
let _22: isize;
let _23: [u128; 7];
let _24: *const i64;
let _25: isize;
let _26: f64;
let _27: u64;
let _28: bool;
let _29: f32;
let _30: i32;
let _31: u64;
let _32: char;
let _33: &'static Adt66;
let _34: *mut *mut u16;
let _35: *mut *mut u16;
let _36: [u16; 1];
let _37: ();
let _38: ();
{
RET = 161_u8 as f32;
RET = 884_u16 as f32;
_1 = 469523565658479965_i64;
RET = 22822_u16 as f32;
_1 = -(-224534381620668748_i64);
_1 = (-8153369181782320272_i64) - (-3971439762298792857_i64);
_1 = 2138178908225798370_i64 << 805355677_u32;
RET = 4184652225_u32 as f32;
_1 = 6051834197309947246_i64 | (-3699358029247662994_i64);
RET = (-16066343_i32) as f32;
_1 = -(-3302548849166963663_i64);
_1 = 477_u16 as i64;
RET = 14199918341375369289_usize as f32;
_1 = (-2467339036072008978_i64) >> 28_u8;
RET = (-73_i8) as f32;
_1 = (-7714422026384626560_i64);
RET = 226_u8 as f32;
RET = 44215_u16 as f32;
RET = 40068_u16 as f32;
_1 = 611232710_u32 as i64;
_1 = (-4033166703727981198_i64) & 3958222678120970615_i64;
_1 = 8285841168548220680_i64 & 5080195405490780791_i64;
RET = 24440_i16 as f32;
RET = 2_usize as f32;
RET = (-1600479078_i32) as f32;
_1 = (-8448073241903474595_i64);
RET = _1 as f32;
RET = 26299_u16 as f32;
RET = (-94428990899076525229689481688364999672_i128) as f32;
RET = 13386165950857369503_u64 as f32;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454926534189864736861 => bb6,
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
RET = 220992691347908834819619762437841834483_u128 as f32;
_1 = 6415_i16 as i64;
RET = 1090078383_i32 as f32;
RET = (-9223372036854775808_isize) as f32;
RET = 3480973462434570717_usize as f32;
RET = 3040625908_u32 as f32;
RET = _1 as f32;
RET = 9223372036854775807_isize as f32;
_1 = 5721380873182944597_i64;
RET = 430259233_u32 as f32;
match _1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5721380873182944597 => bb13,
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
RET = (-127119517150273560505925503498312998764_i128) as f32;
_1 = (-5460424408201739843_i64);
RET = 11434_i16 as f32;
Goto(bb14)
}
bb14 = {
RET = (-103991723234366100673110897126758119666_i128) as f32;
_1 = 64_u8 as i64;
RET = 155_u8 as f32;
RET = (-378291542_i32) as f32;
RET = 1201731192_u32 as f32;
_1 = (-7522366499470544928_i64);
_1 = (-5179553317904021771_i64);
RET = _1 as f32;
_1 = 8168352368019781258_i64 & 2999815779813249865_i64;
RET = 35375_u16 as f32;
RET = _1 as f32;
RET = 30_i8 as f32;
RET = 9223372036854775807_isize as f32;
_4 = [3159705561_u32,543876083_u32,1641832108_u32];
_4 = [4174906033_u32,2586648257_u32,2373020315_u32];
_4 = [3888159498_u32,415300298_u32,2395442281_u32];
RET = (-1551332685_i32) as f32;
RET = 229_u8 as f32;
Goto(bb15)
}
bb15 = {
RET = 83_i8 as f32;
_4 = [3642300685_u32,752940995_u32,738151356_u32];
_1 = (-5954415158287530283_i64);
_4 = [2869995360_u32,2947261523_u32,2926598144_u32];
RET = 15627815364541382724_u64 as f32;
_1 = 5826621435610704056_i64;
_1 = 8490023403039106854_i64 | (-3286482849842655314_i64);
_4 = [582679233_u32,3015042528_u32,2503422053_u32];
RET = 2225632254_u32 as f32;
_4 = [939739093_u32,3471903150_u32,2027221282_u32];
_1 = 6701058034000528984_i64 & 3701409791907138623_i64;
RET = 287003417185283996078486140872538399493_u128 as f32;
_4 = [1998369626_u32,2461725933_u32,112192884_u32];
_6 = 163657935_i32 as u8;
Goto(bb16)
}
bb16 = {
_4 = [3591568671_u32,3323017174_u32,163125728_u32];
RET = 962475294_u32 as f32;
_1 = 972182566359312641_i64;
RET = 324147675459529338528331598785035115524_u128 as f32;
_4 = [1358802056_u32,2394257460_u32,241317925_u32];
_4 = [811056689_u32,3538251217_u32,2367056021_u32];
_7 = !_1;
_7 = _1 >> _6;
_6 = !50_u8;
RET = 1835_u16 as f32;
_8 = 12518_u16 as f64;
_7 = !_1;
_1 = _7 & _7;
_6 = (-119211128430639624260897536616073434529_i128) as u8;
RET = 37660_u16 as f32;
Goto(bb17)
}
bb17 = {
Goto(bb18)
}
bb18 = {
_7 = !_1;
_9 = false;
_1 = !_7;
_7 = 753519455_u32 as i64;
_7 = _1;
_7 = 9674023117584699156_u64 as i64;
Goto(bb19)
}
bb19 = {
_10 = -(-157982742112526291507081365183142425194_i128);
_8 = 8704276848302626068_usize as f64;
RET = 318085102_u32 as f32;
_1 = _7 | _7;
_1 = (-9223372036854775808_isize) as i64;
_1 = _6 as i64;
_6 = !250_u8;
_6 = 145_u8 - 11_u8;
Goto(bb20)
}
bb20 = {
_10 = 228848826716754118037821231851438411073_u128 as i128;
_9 = _10 < _10;
_4 = [2317675168_u32,743592696_u32,4204907184_u32];
_8 = (-30_i8) as f64;
_6 = 3_usize as u8;
_4 = [3410377021_u32,3250144854_u32,503841335_u32];
_6 = 25_u8;
_1 = !_7;
_10 = RET as i128;
RET = 3_usize as f32;
_1 = _7;
_4 = [3589564874_u32,3425650657_u32,2036845832_u32];
_9 = !false;
_8 = RET as f64;
_4 = [2679619456_u32,2696696278_u32,4279332512_u32];
_4 = [3401980003_u32,907222917_u32,4121241823_u32];
_6 = 159_u8 & 68_u8;
_4 = [2191542797_u32,3962012271_u32,1614137307_u32];
RET = 2986977546_u32 as f32;
_4 = [1108502712_u32,1153744820_u32,2831849992_u32];
_10 = (-534843425_i32) as i128;
_13 = 30725_u16 as i32;
RET = 9223372036854775807_isize as f32;
_1 = _7 - _7;
_6 = 74_u8 + 108_u8;
_6 = 101_u8 - 132_u8;
RET = (-46_i8) as f32;
_6 = !60_u8;
Call(_10 = fn18(_13, _7, _8, _9, _4, _4, _4, _13, _4), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_6 = _13 as u8;
_6 = 4728817838251042598_u64 as u8;
_8 = 1303458869_u32 as f64;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = _13 != _13;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_13 = 96065283_i32 | 1039377354_i32;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_15 = RET * RET;
_8 = 44974_u16 as f64;
_10 = (-164686540390404805719111244322539710161_i128) << _1;
_9 = true | false;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_4 = [2181417206_u32,319391004_u32,3060977336_u32];
_8 = (-17640_i16) as f64;
Goto(bb22)
}
bb22 = {
RET = -_15;
_6 = 199_u8;
_8 = _13 as f64;
_15 = RET;
RET = _15;
_13 = (-2113233060_i32);
Goto(bb23)
}
bb23 = {
_7 = _1;
_7 = 9223372036854775807_isize as i64;
_4 = [1369131389_u32,181005035_u32,588827862_u32];
_13 = 73_isize as i32;
_7 = _1 ^ _1;
_1 = -_7;
_10 = 2124099253178757842149181393475332682_i128 >> _6;
_1 = !_7;
_13 = (-1493759370_i32) - 809817437_i32;
Goto(bb24)
}
bb24 = {
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = (-37691013163797321103491041572387541640_i128);
_4 = [571763252_u32,2482307258_u32,2097387142_u32];
_4 = [4097372837_u32,416855978_u32,367153297_u32];
_6 = 236_u8 & 129_u8;
_4 = [1109112774_u32,322706552_u32,2888283984_u32];
_7 = 7200_u16 as i64;
_4 = [3135313760_u32,1039389200_u32,3830923527_u32];
RET = _15 - _15;
_13 = (-893800331_i32) | (-1550776300_i32);
_7 = _1;
_19 = 16808746745892653923129510778983471864_u128 as u8;
_4 = [454666193_u32,1300624016_u32,3460765764_u32];
_19 = _6 << _1;
_15 = RET;
_9 = _15 > RET;
_10 = 1398237985_u32 as i128;
_6 = _19;
_6 = _19 + _19;
_10 = 91380870908745306634545030267082684644_i128;
_13 = (-566799187_i32) + 805537422_i32;
_20 = [_1,_7];
_7 = _1 + _1;
_8 = (-111_i8) as f64;
_7 = -_1;
match _10 {
0 => bb2,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
91380870908745306634545030267082684644 => bb32,
_ => bb31
}
}
bb25 = {
Return()
}
bb26 = {
RET = -_15;
_6 = 199_u8;
_8 = _13 as f64;
_15 = RET;
RET = _15;
_13 = (-2113233060_i32);
Goto(bb23)
}
bb27 = {
_6 = _13 as u8;
_6 = 4728817838251042598_u64 as u8;
_8 = 1303458869_u32 as f64;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = _13 != _13;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_13 = 96065283_i32 | 1039377354_i32;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_15 = RET * RET;
_8 = 44974_u16 as f64;
_10 = (-164686540390404805719111244322539710161_i128) << _1;
_9 = true | false;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_4 = [2181417206_u32,319391004_u32,3060977336_u32];
_8 = (-17640_i16) as f64;
Goto(bb22)
}
bb28 = {
_10 = 228848826716754118037821231851438411073_u128 as i128;
_9 = _10 < _10;
_4 = [2317675168_u32,743592696_u32,4204907184_u32];
_8 = (-30_i8) as f64;
_6 = 3_usize as u8;
_4 = [3410377021_u32,3250144854_u32,503841335_u32];
_6 = 25_u8;
_1 = !_7;
_10 = RET as i128;
RET = 3_usize as f32;
_1 = _7;
_4 = [3589564874_u32,3425650657_u32,2036845832_u32];
_9 = !false;
_8 = RET as f64;
_4 = [2679619456_u32,2696696278_u32,4279332512_u32];
_4 = [3401980003_u32,907222917_u32,4121241823_u32];
_6 = 159_u8 & 68_u8;
_4 = [2191542797_u32,3962012271_u32,1614137307_u32];
RET = 2986977546_u32 as f32;
_4 = [1108502712_u32,1153744820_u32,2831849992_u32];
_10 = (-534843425_i32) as i128;
_13 = 30725_u16 as i32;
RET = 9223372036854775807_isize as f32;
_1 = _7 - _7;
_6 = 74_u8 + 108_u8;
_6 = 101_u8 - 132_u8;
RET = (-46_i8) as f32;
_6 = !60_u8;
Call(_10 = fn18(_13, _7, _8, _9, _4, _4, _4, _13, _4), ReturnTo(bb21), UnwindUnreachable())
}
bb29 = {
Return()
}
bb30 = {
Return()
}
bb31 = {
_4 = [3591568671_u32,3323017174_u32,163125728_u32];
RET = 962475294_u32 as f32;
_1 = 972182566359312641_i64;
RET = 324147675459529338528331598785035115524_u128 as f32;
_4 = [1358802056_u32,2394257460_u32,241317925_u32];
_4 = [811056689_u32,3538251217_u32,2367056021_u32];
_7 = !_1;
_7 = _1 >> _6;
_6 = !50_u8;
RET = 1835_u16 as f32;
_8 = 12518_u16 as f64;
_7 = !_1;
_1 = _7 & _7;
_6 = (-119211128430639624260897536616073434529_i128) as u8;
RET = 37660_u16 as f32;
Goto(bb17)
}
bb32 = {
_1 = _7 << _6;
Goto(bb33)
}
bb33 = {
_22 = 90_isize;
Goto(bb34)
}
bb34 = {
RET = _22 as f32;
_14 = [_19,_6,_6,_6,_6,_6,_19,_6];
_9 = false;
_7 = _1 + _1;
_20 = [_7,_1];
_9 = false & false;
_14 = [_19,_19,_6,_6,_6,_6,_19,_6];
_4 = [3494677041_u32,187401689_u32,398465162_u32];
_20 = [_7,_1];
_1 = _22 as i64;
_7 = _1 >> _19;
_4 = [1609636928_u32,192326579_u32,3899334460_u32];
_15 = 3386_i16 as f32;
_13 = (-960768481_i32) & (-1749738495_i32);
_13 = (-1191474186_i32) & (-577582869_i32);
match _22 {
0 => bb35,
1 => bb36,
2 => bb37,
3 => bb38,
90 => bb40,
_ => bb39
}
}
bb35 = {
Return()
}
bb36 = {
_10 = -(-157982742112526291507081365183142425194_i128);
_8 = 8704276848302626068_usize as f64;
RET = 318085102_u32 as f32;
_1 = _7 | _7;
_1 = (-9223372036854775808_isize) as i64;
_1 = _6 as i64;
_6 = !250_u8;
_6 = 145_u8 - 11_u8;
Goto(bb20)
}
bb37 = {
RET = (-103991723234366100673110897126758119666_i128) as f32;
_1 = 64_u8 as i64;
RET = 155_u8 as f32;
RET = (-378291542_i32) as f32;
RET = 1201731192_u32 as f32;
_1 = (-7522366499470544928_i64);
_1 = (-5179553317904021771_i64);
RET = _1 as f32;
_1 = 8168352368019781258_i64 & 2999815779813249865_i64;
RET = 35375_u16 as f32;
RET = _1 as f32;
RET = 30_i8 as f32;
RET = 9223372036854775807_isize as f32;
_4 = [3159705561_u32,543876083_u32,1641832108_u32];
_4 = [4174906033_u32,2586648257_u32,2373020315_u32];
_4 = [3888159498_u32,415300298_u32,2395442281_u32];
RET = (-1551332685_i32) as f32;
RET = 229_u8 as f32;
Goto(bb15)
}
bb38 = {
RET = 220992691347908834819619762437841834483_u128 as f32;
_1 = 6415_i16 as i64;
RET = 1090078383_i32 as f32;
RET = (-9223372036854775808_isize) as f32;
RET = 3480973462434570717_usize as f32;
RET = 3040625908_u32 as f32;
RET = _1 as f32;
RET = 9223372036854775807_isize as f32;
_1 = 5721380873182944597_i64;
RET = 430259233_u32 as f32;
match _1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5721380873182944597 => bb13,
_ => bb12
}
}
bb39 = {
Return()
}
bb40 = {
Goto(bb41)
}
bb41 = {
RET = _15;
_24 = core::ptr::addr_of!(_7);
_10 = 155146861520200939036162098936110496895_i128;
_15 = RET - RET;
(*_24) = _1;
(*_24) = _1 & _1;
_14 = [_6,_6,_6,_6,_19,_6,_6,_6];
_4 = [2591643615_u32,1254901417_u32,1237989993_u32];
_24 = core::ptr::addr_of!(_1);
(*_24) = _7 << _6;
_4 = [3256112285_u32,3107597930_u32,3895385758_u32];
Goto(bb42)
}
bb42 = {
(*_24) = 88199362_u32 as i64;
(*_24) = _7 - _7;
match _10 {
0 => bb5,
1 => bb9,
2 => bb43,
3 => bb44,
4 => bb45,
5 => bb46,
155146861520200939036162098936110496895 => bb48,
_ => bb47
}
}
bb43 = {
Return()
}
bb44 = {
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = (-37691013163797321103491041572387541640_i128);
_4 = [571763252_u32,2482307258_u32,2097387142_u32];
_4 = [4097372837_u32,416855978_u32,367153297_u32];
_6 = 236_u8 & 129_u8;
_4 = [1109112774_u32,322706552_u32,2888283984_u32];
_7 = 7200_u16 as i64;
_4 = [3135313760_u32,1039389200_u32,3830923527_u32];
RET = _15 - _15;
_13 = (-893800331_i32) | (-1550776300_i32);
_7 = _1;
_19 = 16808746745892653923129510778983471864_u128 as u8;
_4 = [454666193_u32,1300624016_u32,3460765764_u32];
_19 = _6 << _1;
_15 = RET;
_9 = _15 > RET;
_10 = 1398237985_u32 as i128;
_6 = _19;
_6 = _19 + _19;
_10 = 91380870908745306634545030267082684644_i128;
_13 = (-566799187_i32) + 805537422_i32;
_20 = [_1,_7];
_7 = _1 + _1;
_8 = (-111_i8) as f64;
_7 = -_1;
match _10 {
0 => bb2,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
91380870908745306634545030267082684644 => bb32,
_ => bb31
}
}
bb45 = {
_1 = _7 << _6;
Goto(bb33)
}
bb46 = {
RET = 220992691347908834819619762437841834483_u128 as f32;
_1 = 6415_i16 as i64;
RET = 1090078383_i32 as f32;
RET = (-9223372036854775808_isize) as f32;
RET = 3480973462434570717_usize as f32;
RET = 3040625908_u32 as f32;
RET = _1 as f32;
RET = 9223372036854775807_isize as f32;
_1 = 5721380873182944597_i64;
RET = 430259233_u32 as f32;
match _1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5721380873182944597 => bb13,
_ => bb12
}
}
bb47 = {
Return()
}
bb48 = {
(*_24) = !_7;
_10 = !(-47153652380362399572013495724556120508_i128);
_7 = (*_24);
(*_24) = _7;
_27 = 3160390366404851769_u64 - 6367382934341126179_u64;
(*_24) = _7 >> _6;
_28 = _9 | _9;
(*_24) = !_7;
(*_24) = _7 | _7;
_22 = 9223372036854775807_isize;
(*_24) = _7 | _7;
(*_24) = (-30633_i16) as i64;
Goto(bb49)
}
bb49 = {
(*_24) = 51845_u16 as i64;
(*_24) = _7 * _7;
(*_24) = _19 as i64;
(*_24) = !_7;
_4 = [2583342527_u32,2417619496_u32,3393707990_u32];
_25 = _22 & _22;
_31 = !_27;
(*_24) = _7;
_30 = -_13;
_23 = [190398796104680923965689089182594234764_u128,298950019610179703173791402102812403489_u128,177456615075589165517138878136650177954_u128,264821216634438775192140357808315459127_u128,63786168222935153017240513156842709254_u128,174039362791986028397138416285365788662_u128,143603276846904107754854616894516447447_u128];
(*_24) = _10 as i64;
_15 = RET * RET;
(*_24) = _28 as i64;
(*_24) = _7 - _7;
_32 = '\u{b4811}';
_26 = _8 * _8;
Goto(bb50)
}
bb50 = {
Call(_37 = dump_var(Move(_23), Move(_31), Move(_19), Move(_10)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_37 = dump_var(Move(_32), Move(_28), Move(_30), Move(_13)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_37 = dump_var(Move(_1), _38, _38, _38), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: i32,mut _2: i64,mut _3: f64,mut _4: bool,mut _5: [u32; 3],mut _6: [u32; 3],mut _7: [u32; 3],mut _8: i32,mut _9: [u32; 3]) -> i128 {
mir! {
type RET = i128;
let _10: bool;
let _11: f64;
let _12: [i64; 2];
let _13: u32;
let _14: isize;
let _15: i8;
let _16: &'static [i128; 8];
let _17: &'static mut [u32; 3];
let _18: &'static Adt66;
let _19: f64;
let _20: u16;
let _21: [u128; 7];
let _22: u64;
let _23: char;
let _24: &'static Adt66;
let _25: f32;
let _26: bool;
let _27: *mut *mut u16;
let _28: [i128; 2];
let _29: char;
let _30: isize;
let _31: *const &'static mut [u32; 3];
let _32: u128;
let _33: i32;
let _34: bool;
let _35: &'static mut &'static mut u32;
let _36: &'static mut *const *const char;
let _37: char;
let _38: Adt26;
let _39: [u32; 4];
let _40: isize;
let _41: *mut Adt52;
let _42: f64;
let _43: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _44: u128;
let _45: f32;
let _46: u64;
let _47: i16;
let _48: (Adt21, ((f32, i8, i128), usize, &'static (f32, i8, i128)), &'static (f32, i8, i128));
let _49: char;
let _50: [i64; 2];
let _51: i8;
let _52: [u32; 6];
let _53: *const *const char;
let _54: [u16; 1];
let _55: u16;
let _56: *const Adt21;
let _57: char;
let _58: &'static mut *const *const char;
let _59: isize;
let _60: *mut &'static mut &'static mut u32;
let _61: isize;
let _62: &'static mut u32;
let _63: &'static [i128; 8];
let _64: f32;
let _65: *mut &'static char;
let _66: *mut *mut u16;
let _67: &'static mut (*mut u16,);
let _68: &'static mut *const *const char;
let _69: Adt17;
let _70: [u128; 7];
let _71: i128;
let _72: (i32, *const Adt21, &'static mut u32, u32);
let _73: f32;
let _74: bool;
let _75: &'static mut *const *const char;
let _76: [u16; 5];
let _77: &'static Adt66;
let _78: &'static mut u32;
let _79: [i16; 1];
let _80: bool;
let _81: i128;
let _82: i16;
let _83: f64;
let _84: bool;
let _85: *const char;
let _86: bool;
let _87: i32;
let _88: *const Adt21;
let _89: [i64; 2];
let _90: [i64; 2];
let _91: i64;
let _92: &'static mut *const *const char;
let _93: isize;
let _94: *mut &'static char;
let _95: &'static mut [u32; 3];
let _96: *const &'static (f32, i8, i128);
let _97: *mut Adt52;
let _98: ();
let _99: ();
{
_7 = [523177292_u32,2208123348_u32,2675521280_u32];
_9 = [3510242609_u32,754302567_u32,293019802_u32];
_8 = (-41_i8) as i32;
_5 = [2629203029_u32,1065618087_u32,2600303095_u32];
Call(_1 = core::intrinsics::transmute(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb2 = {
_9 = _5;
_4 = !_10;
_6 = [563747978_u32,1396117982_u32,1528905849_u32];
_11 = 4223537568_u32 as f64;
_14 = (-9223372036854775808_isize) * 12_isize;
_2 = (-8433856114651553486_i64) << _1;
_12 = [_2,_2];
_4 = _1 == _8;
_7 = [133957931_u32,3072633119_u32,4159749135_u32];
_13 = (-80_i8) as u32;
_11 = _3 * _3;
Call(_1 = fn19(RET, _5, _7, _14, _2, RET, _2, _14, _6, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = (-55_i8) ^ (-55_i8);
_8 = _1 >> _1;
_4 = _1 >= _1;
_11 = _3 + _3;
_11 = _3 * _3;
_3 = _11;
_7 = [_13,_13,_13];
_7 = _9;
_7 = [_13,_13,_13];
_5 = [_13,_13,_13];
_17 = &mut _6;
(*_17) = [_13,_13,_13];
_14 = 21_isize >> _1;
_7 = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = _9;
_3 = -_11;
_13 = 3844059166_u32;
_11 = -_3;
(*_17) = _9;
(*_17) = _9;
_17 = &mut _5;
match _13 {
3844059166 => bb5,
_ => bb4
}
}
bb4 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb5 = {
_3 = -_11;
_19 = _3;
(*_17) = _9;
_20 = 50760_u16 << _8;
(*_17) = _7;
_10 = _14 < _14;
Goto(bb6)
}
bb6 = {
(*_17) = [_13,_13,_13];
_4 = _10 & _10;
_25 = _19 as f32;
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_1 = -_8;
_10 = _4 == _4;
Goto(bb7)
}
bb7 = {
(*_17) = [_13,_13,_13];
_22 = 14374961332881730524_usize as u64;
_23 = '\u{2aa39}';
_21 = [169130814486385235339329137107153090477_u128,98471182587818266112126722393243967295_u128,98709228025082609574004772553356862329_u128,254607784531155381265593886828379078453_u128,61737305829248568738155396869407892266_u128,213366304204633370457176214938938460952_u128,229913545145825447273352203557730927699_u128];
_23 = '\u{10fe23}';
_1 = _8;
_26 = _4 & _4;
(*_17) = [_13,_13,_13];
_22 = 13842105383507128695_u64 | 10127055004485659855_u64;
(*_17) = [_13,_13,_13];
_25 = _22 as f32;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_28 = [RET,RET];
(*_17) = [_13,_13,_13];
_29 = _23;
_15 = (-16_i8) ^ 34_i8;
(*_17) = [_13,_13,_13];
_21 = [256411417398686078486563946481053468176_u128,223654437691604514642688111269778716638_u128,215802333770219853541020863090187378450_u128,121520689319724494534936272077113327729_u128,1018785259364670590296606084317315489_u128,231328729243889442935848209823979766741_u128,48013897247366650010231729007823346286_u128];
_12 = [_2,_2];
_12 = [_2,_2];
RET = _19 as i128;
_3 = _19 - _11;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = _9;
_11 = _3 * _3;
Call(_14 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = &mut _9;
_29 = _23;
(*_17) = [_13,_13,_13];
_10 = _26;
(*_17) = _7;
_26 = _10 == _10;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_23 = _29;
_7 = (*_17);
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = _7;
(*_17) = [_13,_13,_13];
_31 = core::ptr::addr_of!(_17);
_13 = 3054095170_u32;
_21 = [285684203272725686454906387502220697435_u128,35366017260733626648643868824337787738_u128,304269455082414894750792673728721270946_u128,97207808522426047515814805118189615593_u128,126615809395249725442699770749310765469_u128,328569682181258913675048169354002551423_u128,260551653212415744363565448261387942183_u128];
_14 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_31) = &mut _7;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_11 = _3 - _3;
(*_17) = [_13,_13,_13];
_20 = _2 as u16;
match _13 {
3054095170 => bb10,
_ => bb9
}
}
bb9 = {
(*_17) = [_13,_13,_13];
_22 = 14374961332881730524_usize as u64;
_23 = '\u{2aa39}';
_21 = [169130814486385235339329137107153090477_u128,98471182587818266112126722393243967295_u128,98709228025082609574004772553356862329_u128,254607784531155381265593886828379078453_u128,61737305829248568738155396869407892266_u128,213366304204633370457176214938938460952_u128,229913545145825447273352203557730927699_u128];
_23 = '\u{10fe23}';
_1 = _8;
_26 = _4 & _4;
(*_17) = [_13,_13,_13];
_22 = 13842105383507128695_u64 | 10127055004485659855_u64;
(*_17) = [_13,_13,_13];
_25 = _22 as f32;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_28 = [RET,RET];
(*_17) = [_13,_13,_13];
_29 = _23;
_15 = (-16_i8) ^ 34_i8;
(*_17) = [_13,_13,_13];
_21 = [256411417398686078486563946481053468176_u128,223654437691604514642688111269778716638_u128,215802333770219853541020863090187378450_u128,121520689319724494534936272077113327729_u128,1018785259364670590296606084317315489_u128,231328729243889442935848209823979766741_u128,48013897247366650010231729007823346286_u128];
_12 = [_2,_2];
_12 = [_2,_2];
RET = _19 as i128;
_3 = _19 - _11;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = _9;
_11 = _3 * _3;
Call(_14 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
RET = (-154406904852293531650878368796681842146_i128) + 29431138906420435313206175220579129810_i128;
(*_17) = [_13,_13,_13];
_32 = 218730490290510621065915034689590435517_u128 >> _1;
_10 = _26 ^ _4;
_33 = _1 | _8;
_22 = _15 as u64;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_29 = _23;
(*_17) = [_13,_13,_13];
match _13 {
3054095170 => bb11,
_ => bb9
}
}
bb11 = {
_32 = 49300760893091700034626446482785085953_u128 << _1;
(*_17) = [_13,_13,_13];
_11 = _3 * _3;
_25 = 14321149358903422633_usize as f32;
RET = _2 as i128;
_38.fld2.0 = _10 as u16;
_3 = _11 + _19;
_38.fld5 = Adt21::Variant2 { fld0: _13,fld1: _23,fld2: _25,fld3: _3,fld4: 17605836393576758088_usize };
_43.2.0 = _8 << _38.fld2.0;
_43.2.2 = &mut place!(Field::<u32>(Variant(_38.fld5, 2), 0));
_20 = !12288_u16;
_13 = !3845518096_u32;
(*_17) = [_13,_13,_13];
_22 = 14644782002542628050_u64 - 14882179093874033485_u64;
_12 = [_2,_2];
_22 = 2858897729699350302_u64 >> _1;
Goto(bb12)
}
bb12 = {
_43.2.0 = !_8;
_23 = _29;
(*_17) = [_13,_13,_13];
_43.2.0 = -_8;
_39 = [_13,_13,_13,_13];
(*_17) = [_13,_13,_13];
Goto(bb13)
}
bb13 = {
_43.1 = [_23];
_3 = -_11;
_14 = (-111_isize) >> _22;
_30 = _14 & _14;
_26 = _10 & _4;
_32 = 255786384425407744056974978925534904899_u128 >> _33;
_15 = !(-88_i8);
_42 = _3 + _11;
Goto(bb14)
}
bb14 = {
_13 = 3985670260_u32 >> _43.2.0;
(*_17) = [_13,_13,_13];
_10 = !_4;
_40 = -_30;
(*_17) = [_13,_13,_13];
_34 = _30 < _30;
(*_17) = [_13,_13,_13];
_19 = _42 + _42;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_48.1.0.1 = -_15;
(*_17) = [_13,_13,_13];
_42 = 1_usize as f64;
Goto(bb15)
}
bb15 = {
_43.2.3 = !_13;
(*_17) = [_13,_13,_13];
(*_17) = [_43.2.3,_13,_13];
_37 = _23;
_35 = &mut _43.2.2;
_23 = _37;
_48.1.0.1 = _32 as i8;
_22 = 9697089156231981547_u64;
RET = !67207577169260667826740778595874728270_i128;
_49 = _29;
(*_17) = [_13,_13,_13];
_15 = _48.1.0.1;
Call(_48.1.0.0 = core::intrinsics::transmute(_29), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_51 = _25 as i8;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_3 = _19;
(*_17) = [_13,_13,_13];
_25 = _48.1.0.0 + _48.1.0.0;
(*_17) = [_13,_13,_13];
_37 = _49;
_2 = 1185155252334889522_i64 ^ 8706196909875734509_i64;
_52 = [_13,_13,_13,_13,_13,_13];
Goto(bb17)
}
bb17 = {
(*_35) = &mut _13;
(*_17) = [189861198_u32,3305254286_u32,3735987937_u32];
_4 = _26 | _34;
_45 = _25 - _48.1.0.0;
_2 = _19 as i64;
_59 = _26 as isize;
(*_17) = [2508704608_u32,4132275082_u32,3130011773_u32];
(*_17) = [1217175314_u32,4090020131_u32,2259386383_u32];
_40 = _30 + _30;
RET = (-105593334216697673773835261198581855670_i128);
(*_17) = [380666336_u32,2790028121_u32,505088671_u32];
_60 = core::ptr::addr_of_mut!(_35);
_45 = _48.1.0.0 * _25;
(*_17) = [84146435_u32,3410429907_u32,1874893047_u32];
_2 = (-6502407471570815529_i64) >> _1;
Goto(bb18)
}
bb18 = {
_8 = _33 << _15;
_48.1.1 = 13397103950674166106_usize & 12117304534406273104_usize;
(*_17) = [1484433853_u32,460223646_u32,3084995447_u32];
(*_17) = [3524757934_u32,503175975_u32,2104706582_u32];
_47 = !(-28286_i16);
_15 = -_48.1.0.1;
Goto(bb19)
}
bb19 = {
_33 = _1 - _1;
_56 = core::ptr::addr_of!(_48.0);
_19 = _11;
(*_56) = Adt21::Variant2 { fld0: 1868247474_u32,fld1: _49,fld2: _25,fld3: _3,fld4: _48.1.1 };
(*_56) = Adt21::Variant2 { fld0: 3726849572_u32,fld1: _37,fld2: _48.1.0.0,fld3: _3,fld4: _48.1.1 };
place!(Field::<usize>(Variant((*_56), 2), 4)) = _48.1.1 - _48.1.1;
_48.1.0.2 = RET - RET;
place!(Field::<char>(Variant((*_56), 2), 1)) = _37;
(*_17) = [1638425114_u32,1421111135_u32,2125006053_u32];
place!(Field::<usize>(Variant((*_56), 2), 4)) = _8 as usize;
(*_56) = Adt21::Variant2 { fld0: 630967316_u32,fld1: _23,fld2: _25,fld3: _3,fld4: _48.1.1 };
place!(Field::<usize>(Variant((*_56), 2), 4)) = !_48.1.1;
place!(Field::<u32>(Variant((*_56), 2), 0)) = RET as u32;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _48.1.0.0 - _25;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _19;
_3 = Field::<f64>(Variant((*_56), 2), 3);
place!(Field::<u32>(Variant((*_56), 2), 0)) = _47 as u32;
place!(Field::<f64>(Variant((*_56), 2), 3)) = -_19;
_45 = Field::<f32>(Variant((*_56), 2), 2) * _25;
_48.1.2 = &_48.1.0;
_57 = Field::<char>(Variant((*_56), 2), 1);
_52 = [Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0)];
(*_17) = [Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0)];
place!(Field::<u32>(Variant((*_56), 2), 0)) = 2970859839_u32 ^ 1449505369_u32;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _3;
match _22 {
0 => bb20,
1 => bb21,
2 => bb22,
3 => bb23,
9697089156231981547 => bb25,
_ => bb24
}
}
bb20 = {
(*_17) = [_13,_13,_13];
_4 = _10 & _10;
_25 = _19 as f32;
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_1 = -_8;
_10 = _4 == _4;
Goto(bb7)
}
bb21 = {
_43.1 = [_23];
_3 = -_11;
_14 = (-111_isize) >> _22;
_30 = _14 & _14;
_26 = _10 & _4;
_32 = 255786384425407744056974978925534904899_u128 >> _33;
_15 = !(-88_i8);
_42 = _3 + _11;
Goto(bb14)
}
bb22 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb23 = {
_17 = &mut _9;
_29 = _23;
(*_17) = [_13,_13,_13];
_10 = _26;
(*_17) = _7;
_26 = _10 == _10;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_23 = _29;
_7 = (*_17);
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = _7;
(*_17) = [_13,_13,_13];
_31 = core::ptr::addr_of!(_17);
_13 = 3054095170_u32;
_21 = [285684203272725686454906387502220697435_u128,35366017260733626648643868824337787738_u128,304269455082414894750792673728721270946_u128,97207808522426047515814805118189615593_u128,126615809395249725442699770749310765469_u128,328569682181258913675048169354002551423_u128,260551653212415744363565448261387942183_u128];
_14 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
(*_31) = &mut _7;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_11 = _3 - _3;
(*_17) = [_13,_13,_13];
_20 = _2 as u16;
match _13 {
3054095170 => bb10,
_ => bb9
}
}
bb24 = {
(*_17) = [_13,_13,_13];
_22 = 14374961332881730524_usize as u64;
_23 = '\u{2aa39}';
_21 = [169130814486385235339329137107153090477_u128,98471182587818266112126722393243967295_u128,98709228025082609574004772553356862329_u128,254607784531155381265593886828379078453_u128,61737305829248568738155396869407892266_u128,213366304204633370457176214938938460952_u128,229913545145825447273352203557730927699_u128];
_23 = '\u{10fe23}';
_1 = _8;
_26 = _4 & _4;
(*_17) = [_13,_13,_13];
_22 = 13842105383507128695_u64 | 10127055004485659855_u64;
(*_17) = [_13,_13,_13];
_25 = _22 as f32;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_28 = [RET,RET];
(*_17) = [_13,_13,_13];
_29 = _23;
_15 = (-16_i8) ^ 34_i8;
(*_17) = [_13,_13,_13];
_21 = [256411417398686078486563946481053468176_u128,223654437691604514642688111269778716638_u128,215802333770219853541020863090187378450_u128,121520689319724494534936272077113327729_u128,1018785259364670590296606084317315489_u128,231328729243889442935848209823979766741_u128,48013897247366650010231729007823346286_u128];
_12 = [_2,_2];
_12 = [_2,_2];
RET = _19 as i128;
_3 = _19 - _11;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = _9;
_11 = _3 * _3;
Call(_14 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb25 = {
(*_17) = [Field::<u32>(Variant(_48.0, 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant(_48.0, 2), 0)];
place!(Field::<u32>(Variant((*_56), 2), 0)) = 2803291342_u32 ^ 4049079495_u32;
(*_17) = [Field::<u32>(Variant(_48.0, 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0)];
(*_17) = [Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0)];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _48.1.0.0 * _45;
(*_17) = [Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0),Field::<u32>(Variant((*_56), 2), 0)];
place!(Field::<f64>(Variant((*_56), 2), 3)) = _14 as f64;
_56 = core::ptr::addr_of!((*_56));
_50 = [_2,_2];
_62 = &mut place!(Field::<u32>(Variant((*_56), 2), 0));
_51 = _15;
place!(Field::<char>(Variant((*_56), 2), 1)) = _57;
_46 = _45 as u64;
(*_60) = &mut _62;
_52 = [2924203002_u32,378266542_u32,2866558227_u32,582827133_u32,1563195263_u32,4163222887_u32];
_46 = _22 ^ _22;
_60 = core::ptr::addr_of_mut!((*_60));
place!(Field::<f64>(Variant((*_56), 2), 3)) = _11 * _11;
_46 = _22 - _22;
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_45;
place!(Field::<char>(Variant((*_56), 2), 1)) = _57;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 3_usize & 17885350346882690997_usize;
Goto(bb26)
}
bb26 = {
_20 = _1 as u16;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _11 - _11;
(*_17) = [3682987297_u32,2135088545_u32,3113022708_u32];
place!(Field::<usize>(Variant((*_56), 2), 4)) = 6_usize >> _32;
_54 = [_20];
(*_17) = [1883890528_u32,1488745468_u32,3047033641_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 + _45;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 * _45;
match _22 {
0 => bb14,
9697089156231981547 => bb27,
_ => bb18
}
}
bb27 = {
(*_17) = [816227704_u32,3633545431_u32,556514590_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 * _25;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 1_usize + 4_usize;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 + _45;
_64 = RET as f32;
match _22 {
9697089156231981547 => bb28,
_ => bb19
}
}
bb28 = {
place!(Field::<f64>(Variant((*_56), 2), 3)) = -_11;
_19 = _11;
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_45;
_31 = core::ptr::addr_of!((*_31));
place!(Field::<f32>(Variant((*_56), 2), 2)) = 217_u8 as f32;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 + _45;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 6374435841001778824_usize + 2470037867061407488_usize;
place!(Field::<f64>(Variant((*_56), 2), 3)) = Field::<f32>(Variant((*_56), 2), 2) as f64;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 3_usize | 9379594808335596936_usize;
(*_17) = [3510109541_u32,1836984681_u32,3750393572_u32];
place!(Field::<f64>(Variant((*_56), 2), 3)) = _3;
(*_17) = [742063563_u32,2137753244_u32,4088455554_u32];
_26 = _10 >= _34;
place!(Field::<char>(Variant((*_56), 2), 1)) = _29;
(*_17) = [4038260594_u32,2054404721_u32,2895869184_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _1 as f32;
place!(Field::<char>(Variant((*_56), 2), 1)) = _29;
place!(Field::<char>(Variant((*_56), 2), 1)) = _57;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _19 * _3;
place!(Field::<usize>(Variant((*_56), 2), 4)) = !7875272489352555338_usize;
_55 = _20;
_37 = Field::<char>(Variant((*_56), 2), 1);
place!(Field::<usize>(Variant((*_56), 2), 4)) = 2_usize;
_70 = [_32,_32,_32,_32,_32,_32,_32];
_26 = _10;
match Field::<usize>(Variant((*_56), 2), 4) {
0 => bb13,
1 => bb12,
2 => bb30,
_ => bb29
}
}
bb29 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb30 = {
(*_17) = [2535680556_u32,3961057228_u32,3405843879_u32];
(*_17) = [3496311921_u32,17369110_u32,4277605609_u32];
(*_17) = [2429966514_u32,3013714381_u32,2490985306_u32];
(*_17) = [3325764227_u32,569143202_u32,2564985419_u32];
(*_17) = [1520175287_u32,3778342560_u32,2272919234_u32];
place!(Field::<char>(Variant((*_56), 2), 1)) = _23;
place!(Field::<usize>(Variant((*_56), 2), 4)) = !7297524250798632694_usize;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 12131955847280276659_usize >> _59;
_69 = Adt17::Variant3 { fld0: _32,fld1: Field::<char>(Variant((*_56), 2), 1),fld2: 3252180131_u32,fld3: Field::<f64>(Variant((*_56), 2), 3),fld4: _46,fld5: _1,fld6: _2,fld7: RET };
place!(Field::<char>(Variant((*_56), 2), 1)) = Field::<char>(Variant(_69, 3), 1);
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_25;
_39 = [1753117053_u32,3525208683_u32,1623869978_u32,4236892272_u32];
place!(Field::<char>(Variant((*_56), 2), 1)) = Field::<char>(Variant(_69, 3), 1);
_2 = Field::<i64>(Variant(_69, 3), 6) & Field::<i64>(Variant(_69, 3), 6);
Goto(bb31)
}
bb31 = {
_33 = -Field::<i32>(Variant(_69, 3), 5);
(*_17) = [1048108575_u32,1393637940_u32,263220035_u32];
_72.0 = _8 & _33;
_34 = !_10;
match RET {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
234689032704240789689539346233186355786 => bb37,
_ => bb36
}
}
bb32 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb33 = {
_3 = -_11;
_19 = _3;
(*_17) = _9;
_20 = 50760_u16 << _8;
(*_17) = _7;
_10 = _14 < _14;
Goto(bb6)
}
bb34 = {
(*_17) = [_13,_13,_13];
_4 = _10 & _10;
_25 = _19 as f32;
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_1 = -_8;
_10 = _4 == _4;
Goto(bb7)
}
bb35 = {
_8 = (-27139_i16) as i32;
_10 = !_4;
_1 = _8;
RET = (-155165149164296252966388822643947214983_i128);
_9 = [3778036332_u32,2599256105_u32,1965972448_u32];
_12 = [_2,_2];
RET = (-112705466649273677330858774501121284841_i128) - (-41530418155718918770325330484888077237_i128);
_11 = (-51_i8) as f64;
_3 = _1 as f64;
_4 = !_10;
_4 = _10;
_8 = _1 ^ _1;
RET = 2361802978568999909249459963850993183_i128 ^ 133663578934294331264773254550040743481_i128;
_7 = _6;
_4 = _10 & _10;
_14 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_4 = !_10;
_6 = [4045983665_u32,2073308916_u32,3704923857_u32];
_4 = !_10;
_5 = [2797405321_u32,3003548630_u32,1349304173_u32];
_1 = 3241314741_u32 as i32;
_8 = !_1;
_1 = _8;
Goto(bb2)
}
bb36 = {
_20 = _1 as u16;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _11 - _11;
(*_17) = [3682987297_u32,2135088545_u32,3113022708_u32];
place!(Field::<usize>(Variant((*_56), 2), 4)) = 6_usize >> _32;
_54 = [_20];
(*_17) = [1883890528_u32,1488745468_u32,3047033641_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 + _45;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 * _45;
match _22 {
0 => bb14,
9697089156231981547 => bb27,
_ => bb18
}
}
bb37 = {
_61 = _59;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 9355672552479372372_usize | 15320883964003170934_usize;
(*_17) = [2167491873_u32,2890635145_u32,2986856211_u32];
place!(Field::<i32>(Variant(_69, 3), 5)) = _33;
_37 = Field::<char>(Variant((*_56), 2), 1);
place!(Field::<usize>(Variant((*_56), 2), 4)) = 6929469767017860904_usize + 12494213659844506526_usize;
_20 = !_55;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 2473138506833056420_usize << _40;
_8 = _1 & _72.0;
(*_17) = [3914700733_u32,4213346825_u32,1110554388_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45;
_82 = -_47;
match _22 {
0 => bb38,
1 => bb39,
2 => bb40,
3 => bb41,
9697089156231981547 => bb43,
_ => bb42
}
}
bb38 = {
(*_17) = [_13,_13,_13];
_22 = 14374961332881730524_usize as u64;
_23 = '\u{2aa39}';
_21 = [169130814486385235339329137107153090477_u128,98471182587818266112126722393243967295_u128,98709228025082609574004772553356862329_u128,254607784531155381265593886828379078453_u128,61737305829248568738155396869407892266_u128,213366304204633370457176214938938460952_u128,229913545145825447273352203557730927699_u128];
_23 = '\u{10fe23}';
_1 = _8;
_26 = _4 & _4;
(*_17) = [_13,_13,_13];
_22 = 13842105383507128695_u64 | 10127055004485659855_u64;
(*_17) = [_13,_13,_13];
_25 = _22 as f32;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_28 = [RET,RET];
(*_17) = [_13,_13,_13];
_29 = _23;
_15 = (-16_i8) ^ 34_i8;
(*_17) = [_13,_13,_13];
_21 = [256411417398686078486563946481053468176_u128,223654437691604514642688111269778716638_u128,215802333770219853541020863090187378450_u128,121520689319724494534936272077113327729_u128,1018785259364670590296606084317315489_u128,231328729243889442935848209823979766741_u128,48013897247366650010231729007823346286_u128];
_12 = [_2,_2];
_12 = [_2,_2];
RET = _19 as i128;
_3 = _19 - _11;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = _9;
_11 = _3 * _3;
Call(_14 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb39 = {
(*_17) = [816227704_u32,3633545431_u32,556514590_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 * _25;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 1_usize + 4_usize;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 + _45;
_64 = RET as f32;
match _22 {
9697089156231981547 => bb28,
_ => bb19
}
}
bb40 = {
(*_17) = [_13,_13,_13];
_4 = _10 & _10;
_25 = _19 as f32;
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_1 = -_8;
_10 = _4 == _4;
Goto(bb7)
}
bb41 = {
_3 = -_11;
_19 = _3;
(*_17) = _9;
_20 = 50760_u16 << _8;
(*_17) = _7;
_10 = _14 < _14;
Goto(bb6)
}
bb42 = {
(*_35) = &mut _13;
(*_17) = [189861198_u32,3305254286_u32,3735987937_u32];
_4 = _26 | _34;
_45 = _25 - _48.1.0.0;
_2 = _19 as i64;
_59 = _26 as isize;
(*_17) = [2508704608_u32,4132275082_u32,3130011773_u32];
(*_17) = [1217175314_u32,4090020131_u32,2259386383_u32];
_40 = _30 + _30;
RET = (-105593334216697673773835261198581855670_i128);
(*_17) = [380666336_u32,2790028121_u32,505088671_u32];
_60 = core::ptr::addr_of_mut!(_35);
_45 = _48.1.0.0 * _25;
(*_17) = [84146435_u32,3410429907_u32,1874893047_u32];
_2 = (-6502407471570815529_i64) >> _1;
Goto(bb18)
}
bb43 = {
_4 = Field::<usize>(Variant((*_56), 2), 4) > Field::<usize>(Variant((*_56), 2), 4);
place!(Field::<f64>(Variant(_69, 3), 3)) = Field::<f64>(Variant((*_56), 2), 3);
place!(Field::<char>(Variant((*_56), 2), 1)) = _49;
_54 = [_20];
place!(Field::<f64>(Variant((*_56), 2), 3)) = _3 * _11;
_20 = _4 as u16;
place!(Field::<usize>(Variant((*_56), 2), 4)) = _20 as usize;
_31 = core::ptr::addr_of!((*_31));
_11 = -Field::<f64>(Variant((*_56), 2), 3);
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_64;
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_45;
(*_17) = [3298823884_u32,3575644444_u32,2822307568_u32];
place!(Field::<f32>(Variant((*_56), 2), 2)) = _45 * _25;
_59 = 1481136722_u32 as isize;
match RET {
0 => bb29,
1 => bb35,
2 => bb14,
3 => bb39,
4 => bb44,
234689032704240789689539346233186355786 => bb46,
_ => bb45
}
}
bb44 = {
(*_35) = &mut _13;
(*_17) = [189861198_u32,3305254286_u32,3735987937_u32];
_4 = _26 | _34;
_45 = _25 - _48.1.0.0;
_2 = _19 as i64;
_59 = _26 as isize;
(*_17) = [2508704608_u32,4132275082_u32,3130011773_u32];
(*_17) = [1217175314_u32,4090020131_u32,2259386383_u32];
_40 = _30 + _30;
RET = (-105593334216697673773835261198581855670_i128);
(*_17) = [380666336_u32,2790028121_u32,505088671_u32];
_60 = core::ptr::addr_of_mut!(_35);
_45 = _48.1.0.0 * _25;
(*_17) = [84146435_u32,3410429907_u32,1874893047_u32];
_2 = (-6502407471570815529_i64) >> _1;
Goto(bb18)
}
bb45 = {
(*_17) = [_13,_13,_13];
_22 = 14374961332881730524_usize as u64;
_23 = '\u{2aa39}';
_21 = [169130814486385235339329137107153090477_u128,98471182587818266112126722393243967295_u128,98709228025082609574004772553356862329_u128,254607784531155381265593886828379078453_u128,61737305829248568738155396869407892266_u128,213366304204633370457176214938938460952_u128,229913545145825447273352203557730927699_u128];
_23 = '\u{10fe23}';
_1 = _8;
_26 = _4 & _4;
(*_17) = [_13,_13,_13];
_22 = 13842105383507128695_u64 | 10127055004485659855_u64;
(*_17) = [_13,_13,_13];
_25 = _22 as f32;
(*_17) = [_13,_13,_13];
(*_17) = [_13,_13,_13];
_28 = [RET,RET];
(*_17) = [_13,_13,_13];
_29 = _23;
_15 = (-16_i8) ^ 34_i8;
(*_17) = [_13,_13,_13];
_21 = [256411417398686078486563946481053468176_u128,223654437691604514642688111269778716638_u128,215802333770219853541020863090187378450_u128,121520689319724494534936272077113327729_u128,1018785259364670590296606084317315489_u128,231328729243889442935848209823979766741_u128,48013897247366650010231729007823346286_u128];
_12 = [_2,_2];
_12 = [_2,_2];
RET = _19 as i128;
_3 = _19 - _11;
(*_17) = [_13,_13,_13];
(*_17) = _9;
(*_17) = _9;
_11 = _3 * _3;
Call(_14 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb46 = {
_64 = Field::<f32>(Variant((*_56), 2), 2);
(*_17) = [3289022367_u32,2119180861_u32,4193709045_u32];
place!(Field::<usize>(Variant((*_56), 2), 4)) = 4_usize >> _2;
_44 = !Field::<u128>(Variant(_69, 3), 0);
(*_17) = [1395733489_u32,3532671982_u32,505247254_u32];
_26 = _4 | _34;
place!(Field::<f32>(Variant((*_56), 2), 2)) = Field::<f64>(Variant((*_56), 2), 3) as f32;
place!(Field::<usize>(Variant((*_56), 2), 4)) = !2_usize;
(*_17) = [2493676441_u32,1260361334_u32,3049330962_u32];
place!(Field::<u32>(Variant(_69, 3), 2)) = 426916661_u32 - 1124564179_u32;
_3 = Field::<f64>(Variant((*_56), 2), 3) + Field::<f64>(Variant((*_56), 2), 3);
(*_17) = [Field::<u32>(Variant(_69, 3), 2),Field::<u32>(Variant(_69, 3), 2),Field::<u32>(Variant(_69, 3), 2)];
place!(Field::<usize>(Variant((*_56), 2), 4)) = 3_usize - 10712631388905105878_usize;
_76 = [_20,_20,_55,_55,_20];
(*_17) = [Field::<u32>(Variant(_69, 3), 2),Field::<u32>(Variant(_69, 3), 2),Field::<u32>(Variant(_69, 3), 2)];
place!(Field::<f64>(Variant((*_56), 2), 3)) = _11 + _19;
(*_35) = &mut place!(Field::<u32>(Variant(_69, 3), 2));
_76 = [_55,_20,_55,_55,_20];
_73 = Field::<f32>(Variant((*_56), 2), 2);
_42 = Field::<f64>(Variant((*_56), 2), 3) + Field::<f64>(Variant((*_56), 2), 3);
match _22 {
0 => bb39,
9697089156231981547 => bb47,
_ => bb8
}
}
bb47 = {
_34 = _10 & _4;
place!(Field::<usize>(Variant((*_56), 2), 4)) = 3_usize;
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_64;
_3 = RET as f64;
place!(Field::<char>(Variant((*_56), 2), 1)) = _23;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _73 - _73;
_11 = Field::<f64>(Variant((*_56), 2), 3) - Field::<f64>(Variant((*_56), 2), 3);
_31 = core::ptr::addr_of!((*_31));
_1 = !_8;
_23 = Field::<char>(Variant((*_56), 2), 1);
_32 = _44;
place!(Field::<f64>(Variant((*_56), 2), 3)) = _42 * _42;
_1 = -_8;
place!(Field::<f32>(Variant((*_56), 2), 2)) = -_45;
_23 = Field::<char>(Variant((*_56), 2), 1);
place!(Field::<f64>(Variant((*_56), 2), 3)) = _19 + _42;
_29 = Field::<char>(Variant((*_56), 2), 1);
_78 = Move((*_35));
_73 = -Field::<f32>(Variant((*_56), 2), 2);
place!(Field::<f64>(Variant((*_56), 2), 3)) = -_11;
_85 = core::ptr::addr_of!(place!(Field::<char>(Variant((*_56), 2), 1)));
Goto(bb48)
}
bb48 = {
_85 = core::ptr::addr_of!(place!(Field::<char>(Variant((*_56), 2), 1)));
(*_85) = _37;
place!(Field::<f32>(Variant((*_56), 2), 2)) = _25 * _73;
_72.1 = Move(_56);
(*_17) = [3818948969_u32,42533883_u32,1600627703_u32];
_34 = _4;
(*_17) = [2019946178_u32,118263408_u32,1970819358_u32];
_3 = _42;
_32 = _44;
_64 = _25;
_19 = -_11;
(*_85) = _49;
(*_85) = _37;
_55 = _20 + _20;
_51 = _15 | _15;
_59 = !_30;
(*_17) = [17387378_u32,1601957174_u32,1737084367_u32];
_47 = _2 as i16;
_71 = RET << _33;
_1 = _72.0 + _8;
Goto(bb49)
}
bb49 = {
_46 = _22 | _22;
_30 = _14;
(*_85) = _29;
(*_85) = _37;
(*_85) = _37;
_89 = _50;
(*_17) = [1804203099_u32,1936224490_u32,2597277677_u32];
_33 = _1;
_88 = Move(_72.1);
_52 = [3712096821_u32,969494285_u32,3464131126_u32,2863327673_u32,1127896395_u32,2804445212_u32];
_85 = core::ptr::addr_of!((*_85));
_86 = _34 <= _10;
(*_17) = [398007236_u32,565187607_u32,1657471740_u32];
_45 = _15 as f32;
_79 = [_47];
_22 = !_46;
_26 = _10;
_55 = 3794396133_u32 as u16;
_90 = _89;
_28 = [_71,_71];
_1 = _8 << _22;
_47 = _22 as i16;
Goto(bb50)
}
bb50 = {
Call(_98 = dump_var(Move(_7), Move(_39), Move(_26), Move(_12)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_98 = dump_var(Move(_76), Move(_22), Move(_5), Move(_79)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_98 = dump_var(Move(_10), Move(_28), Move(_90), Move(_44)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_98 = dump_var(Move(_37), Move(_51), Move(_86), Move(_50)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_98 = dump_var(Move(_61), Move(_1), Move(_47), Move(_30)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_98 = dump_var(Move(_52), Move(_14), Move(_2), _99), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i128,mut _2: [u32; 3],mut _3: [u32; 3],mut _4: isize,mut _5: i64,mut _6: i128,mut _7: i64,mut _8: isize,mut _9: [u32; 3],mut _10: isize) -> i32 {
mir! {
type RET = i32;
let _11: *const i64;
let _12: i32;
let _13: i16;
let _14: Adt21;
let _15: Adt52;
let _16: isize;
let _17: i8;
let _18: i128;
let _19: isize;
let _20: usize;
let _21: i128;
let _22: *const &'static char;
let _23: u32;
let _24: [i128; 8];
let _25: bool;
let _26: ((f32, i8, i128), usize, &'static (f32, i8, i128));
let _27: isize;
let _28: i32;
let _29: (*mut u16, &'static (char, i64, Adt17));
let _30: *const &'static (f32, i8, i128);
let _31: u8;
let _32: [u8; 8];
let _33: [i128; 2];
let _34: char;
let _35: *mut u16;
let _36: &'static Adt26;
let _37: Adt64;
let _38: u8;
let _39: i128;
let _40: i8;
let _41: f32;
let _42: &'static i16;
let _43: bool;
let _44: char;
let _45: u8;
let _46: (*const &'static char, [char; 1], (i32, *const Adt21, &'static mut u32, u32));
let _47: f32;
let _48: usize;
let _49: ((&'static (char, i64, Adt17), [u16; 1], [u16; 1], *mut i32), bool, &'static mut &'static mut u32, &'static mut isize);
let _50: *const &'static char;
let _51: u128;
let _52: char;
let _53: i8;
let _54: isize;
let _55: isize;
let _56: bool;
let _57: u64;
let _58: (u16, Adt21, *mut [i16; 1]);
let _59: bool;
let _60: &'static mut &'static mut u32;
let _61: &'static mut u32;
let _62: *mut [i16; 1];
let _63: isize;
let _64: u16;
let _65: ();
let _66: ();
{
_9 = [3687335760_u32,3942125814_u32,1061255292_u32];
_5 = _7 ^ _7;
_3 = _9;
_6 = -_1;
_2 = [4010839287_u32,2392432761_u32,2280493754_u32];
_5 = _7 | _7;
RET = -518921110_i32;
_2 = _3;
_3 = _2;
_3 = [3710348024_u32,4146819843_u32,2990232661_u32];
_11 = core::ptr::addr_of!(_5);
Goto(bb1)
}
bb1 = {
(*_11) = RET as i64;
(*_11) = _7;
(*_11) = _7 << _6;
(*_11) = 28_u8 as i64;
(*_11) = _7 & _7;
_4 = _8 * _8;
(*_11) = '\u{23eb8}' as i64;
(*_11) = -_7;
(*_11) = _7;
_9 = _2;
(*_11) = 31980_u16 as i64;
Goto(bb2)
}
bb2 = {
(*_11) = _7 | _7;
Goto(bb3)
}
bb3 = {
(*_11) = _10 as i64;
_9 = _3;
(*_11) = 84_i8 as i64;
(*_11) = _7 & _7;
(*_11) = _7 & _7;
(*_11) = _7 * _7;
(*_11) = _7 & _7;
(*_11) = 167_u8 as i64;
Call((*_11) = core::intrinsics::bswap(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_11) = 2672633361022068028_usize as i64;
_12 = 237230189414149296248232992482716097247_u128 as i32;
_8 = -_4;
Call(RET = core::intrinsics::transmute(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17 = _12 as i8;
(*_11) = _7 + _7;
_7 = !(*_11);
_6 = _1;
_3 = [3517482099_u32,647795036_u32,2564683870_u32];
(*_11) = _7 << RET;
(*_11) = _7;
(*_11) = _1 as i64;
_17 = (-63_i8) + 50_i8;
(*_11) = -_7;
_9 = _3;
(*_11) = '\u{672de}' as i64;
(*_11) = _7;
_14 = Adt21::Variant0 { fld0: _4 };
_5 = _7 * _7;
_1 = _6 << _8;
_4 = !_10;
(*_11) = true as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = Field::<isize>(Variant(_14, 0), 0) as i64;
(*_11) = _7 | _7;
Goto(bb6)
}
bb6 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb7 = {
_16 = _19;
_5 = _7 + _7;
(*_11) = 172581406794431396320148077541550368394_u128 as i64;
_18 = _1;
(*_11) = _7;
(*_11) = _7 << _13;
(*_11) = _7 << _18;
(*_11) = _7 * _7;
_21 = _18 ^ _18;
(*_11) = !_7;
_18 = 39645_u16 as i128;
(*_11) = _17 as i64;
(*_11) = _7;
_6 = _12 as i128;
_23 = !1018452922_u32;
_20 = 4_usize & 5_usize;
(*_11) = _7 & _7;
Call(_20 = core::intrinsics::transmute((*_11)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19 = (*_11) as isize;
_10 = Field::<isize>(Variant(_14, 0), 0) - Field::<isize>(Variant(_14, 0), 0);
(*_11) = -_7;
_14 = Adt21::Variant0 { fld0: _10 };
_6 = _21;
place!(Field::<isize>(Variant(_14, 0), 0)) = _16;
(*_11) = _7;
(*_11) = _7;
_16 = _10 >> _19;
(*_11) = _23 as i64;
(*_11) = _7 * _7;
_23 = 2838629778_u32;
place!(Field::<isize>(Variant(_14, 0), 0)) = _10 ^ _19;
_3 = _2;
_17 = -30_i8;
_17 = (-40_i8);
_3 = [_23,_23,_23];
_13 = 17517_i16 | (-27419_i16);
_23 = 2035233509_u32;
_26.0.0 = _23 as f32;
_4 = _10 ^ _16;
(*_11) = -_7;
_16 = -Field::<isize>(Variant(_14, 0), 0);
_1 = 35_u8 as i128;
match _17 {
340282366920938463463374607431768211416 => bb10,
_ => bb9
}
}
bb9 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb10 = {
_9 = _2;
_10 = -_4;
(*_11) = _7;
_9 = _2;
_23 = 3520791509_u32;
(*_11) = _7;
_3 = _9;
Goto(bb11)
}
bb11 = {
_26.0.0 = 31320_u16 as f32;
_27 = true as isize;
(*_11) = 135_u8 as i64;
(*_11) = !_7;
_8 = _20 as isize;
_15 = Adt52::Variant0 { fld0: Field::<isize>(Variant(_14, 0), 0) };
(*_11) = !_7;
_26.1 = _20 >> _6;
(*_11) = _7;
(*_11) = 220_u8 as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = _7 + _7;
_26.0.2 = 108741512612772220110545406070816912092_u128 as i128;
_6 = 61_u8 as i128;
(*_11) = !_7;
_19 = _4 << (*_11);
_26.0.1 = _17;
(*_11) = 32141314742081570744097943348649145101_u128 as i64;
(*_11) = _7 * _7;
_28 = _12;
_7 = (*_11) - (*_11);
_26.2 = &_26.0;
_5 = _21 as i64;
Call((*_11) = core::intrinsics::bswap(_7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<isize>(Variant(_14, 0), 0)) = Field::<isize>(Variant(_15, 0), 0) ^ _19;
_30 = core::ptr::addr_of!(_26.2);
(*_11) = _7;
place!(Field::<isize>(Variant(_15, 0), 0)) = Field::<isize>(Variant(_14, 0), 0) & Field::<isize>(Variant(_14, 0), 0);
_18 = _21;
_10 = Field::<isize>(Variant(_15, 0), 0) * Field::<isize>(Variant(_15, 0), 0);
place!(Field::<isize>(Variant(_14, 0), 0)) = _10;
_32 = [245_u8,62_u8,108_u8,187_u8,112_u8,21_u8,224_u8,184_u8];
(*_11) = _7 << _17;
(*_11) = _7 + _7;
match _23 {
0 => bb7,
1 => bb13,
3520791509 => bb15,
_ => bb14
}
}
bb13 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb14 = {
_17 = _12 as i8;
(*_11) = _7 + _7;
_7 = !(*_11);
_6 = _1;
_3 = [3517482099_u32,647795036_u32,2564683870_u32];
(*_11) = _7 << RET;
(*_11) = _7;
(*_11) = _1 as i64;
_17 = (-63_i8) + 50_i8;
(*_11) = -_7;
_9 = _3;
(*_11) = '\u{672de}' as i64;
(*_11) = _7;
_14 = Adt21::Variant0 { fld0: _4 };
_5 = _7 * _7;
_1 = _6 << _8;
_4 = !_10;
(*_11) = true as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = Field::<isize>(Variant(_14, 0), 0) as i64;
(*_11) = _7 | _7;
Goto(bb6)
}
bb15 = {
_19 = _23 as isize;
_34 = '\u{12633}';
_24 = [_21,_18,_21,_6,_18,_18,_21,_21];
_27 = Field::<isize>(Variant(_14, 0), 0) ^ Field::<isize>(Variant(_15, 0), 0);
_34 = '\u{a628e}';
_32 = [159_u8,92_u8,213_u8,11_u8,59_u8,214_u8,230_u8,0_u8];
_12 = _28;
_13 = -15573_i16;
_25 = true;
_26.0.1 = _17;
_26.0.2 = 17687_u16 as i128;
_30 = core::ptr::addr_of!((*_30));
(*_11) = _7;
_18 = Field::<isize>(Variant(_14, 0), 0) as i128;
(*_11) = _7 ^ _7;
(*_11) = _7;
_26.0.1 = _27 as i8;
_8 = _27 << _26.0.1;
(*_11) = _20 as i64;
place!(Field::<isize>(Variant(_15, 0), 0)) = Field::<isize>(Variant(_14, 0), 0);
_12 = RET ^ _28;
place!(Field::<isize>(Variant(_14, 0), 0)) = _4 << _10;
(*_11) = _7 & _7;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_27 = _10 << (*_11);
match _17 {
0 => bb14,
1 => bb10,
2 => bb16,
3 => bb17,
340282366920938463463374607431768211416 => bb19,
_ => bb18
}
}
bb16 = {
_17 = _12 as i8;
(*_11) = _7 + _7;
_7 = !(*_11);
_6 = _1;
_3 = [3517482099_u32,647795036_u32,2564683870_u32];
(*_11) = _7 << RET;
(*_11) = _7;
(*_11) = _1 as i64;
_17 = (-63_i8) + 50_i8;
(*_11) = -_7;
_9 = _3;
(*_11) = '\u{672de}' as i64;
(*_11) = _7;
_14 = Adt21::Variant0 { fld0: _4 };
_5 = _7 * _7;
_1 = _6 << _8;
_4 = !_10;
(*_11) = true as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = Field::<isize>(Variant(_14, 0), 0) as i64;
(*_11) = _7 | _7;
Goto(bb6)
}
bb17 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb18 = {
(*_11) = _10 as i64;
_9 = _3;
(*_11) = 84_i8 as i64;
(*_11) = _7 & _7;
(*_11) = _7 & _7;
(*_11) = _7 * _7;
(*_11) = _7 & _7;
(*_11) = 167_u8 as i64;
Call((*_11) = core::intrinsics::bswap(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb19 = {
_27 = !_8;
_14 = Adt21::Variant0 { fld0: _10 };
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
(*_11) = _7 ^ _7;
match _17 {
340282366920938463463374607431768211416 => bb21,
_ => bb20
}
}
bb20 = {
(*_11) = _10 as i64;
_9 = _3;
(*_11) = 84_i8 as i64;
(*_11) = _7 & _7;
(*_11) = _7 & _7;
(*_11) = _7 * _7;
(*_11) = _7 & _7;
(*_11) = 167_u8 as i64;
Call((*_11) = core::intrinsics::bswap(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb21 = {
_4 = Field::<isize>(Variant(_14, 0), 0) + _27;
(*_11) = 147_u8 as i64;
_31 = 112_u8 & 27_u8;
(*_11) = !_7;
(*_11) = _13 as i64;
(*_11) = _7;
_26.0.1 = _17 - _17;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25 = false;
_6 = _25 as i128;
_39 = !_18;
_16 = Field::<isize>(Variant(_15, 0), 0) << _27;
(*_11) = _7;
_33 = [_39,_39];
_11 = core::ptr::addr_of!((*_11));
_28 = !RET;
_46.1 = [_34];
Goto(bb22)
}
bb22 = {
_30 = core::ptr::addr_of!((*_30));
_9 = _3;
_12 = _13 as i32;
_43 = !_25;
_46.2.2 = &mut _23;
_47 = _26.1 as f32;
_10 = _26.0.0 as isize;
_9 = [2538721106_u32,1520065732_u32,2575719410_u32];
_32 = [_31,_31,_31,_31,_31,_31,_31,_31];
(*_11) = 8045437336002643512_u64 as i64;
_1 = _21 >> Field::<isize>(Variant(_15, 0), 0);
_46.1 = [_34];
_43 = Field::<isize>(Variant(_14, 0), 0) > _16;
_8 = _4 >> _39;
_4 = !Field::<isize>(Variant(_14, 0), 0);
_33 = [_39,_1];
_26.0 = (_47, _17, _1);
_49.0.2 = [33143_u16];
match _17 {
0 => bb6,
1 => bb18,
2 => bb23,
3 => bb24,
4 => bb25,
340282366920938463463374607431768211416 => bb27,
_ => bb26
}
}
bb23 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb24 = {
(*_11) = _7 | _7;
Goto(bb3)
}
bb25 = {
_17 = _12 as i8;
(*_11) = _7 + _7;
_7 = !(*_11);
_6 = _1;
_3 = [3517482099_u32,647795036_u32,2564683870_u32];
(*_11) = _7 << RET;
(*_11) = _7;
(*_11) = _1 as i64;
_17 = (-63_i8) + 50_i8;
(*_11) = -_7;
_9 = _3;
(*_11) = '\u{672de}' as i64;
(*_11) = _7;
_14 = Adt21::Variant0 { fld0: _4 };
_5 = _7 * _7;
_1 = _6 << _8;
_4 = !_10;
(*_11) = true as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = Field::<isize>(Variant(_14, 0), 0) as i64;
(*_11) = _7 | _7;
Goto(bb6)
}
bb26 = {
(*_11) = RET as i64;
(*_11) = _7;
(*_11) = _7 << _6;
(*_11) = 28_u8 as i64;
(*_11) = _7 & _7;
_4 = _8 * _8;
(*_11) = '\u{23eb8}' as i64;
(*_11) = -_7;
(*_11) = _7;
_9 = _2;
(*_11) = 31980_u16 as i64;
Goto(bb2)
}
bb27 = {
_42 = &_13;
_28 = -_12;
match _17 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
340282366920938463463374607431768211416 => bb34,
_ => bb33
}
}
bb28 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb29 = {
_17 = _12 as i8;
(*_11) = _7 + _7;
_7 = !(*_11);
_6 = _1;
_3 = [3517482099_u32,647795036_u32,2564683870_u32];
(*_11) = _7 << RET;
(*_11) = _7;
(*_11) = _1 as i64;
_17 = (-63_i8) + 50_i8;
(*_11) = -_7;
_9 = _3;
(*_11) = '\u{672de}' as i64;
(*_11) = _7;
_14 = Adt21::Variant0 { fld0: _4 };
_5 = _7 * _7;
_1 = _6 << _8;
_4 = !_10;
(*_11) = true as i64;
_11 = core::ptr::addr_of!((*_11));
(*_11) = Field::<isize>(Variant(_14, 0), 0) as i64;
(*_11) = _7 | _7;
Goto(bb6)
}
bb30 = {
_27 = !_8;
_14 = Adt21::Variant0 { fld0: _10 };
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
(*_11) = _7 ^ _7;
match _17 {
340282366920938463463374607431768211416 => bb21,
_ => bb20
}
}
bb31 = {
_19 = _23 as isize;
_34 = '\u{12633}';
_24 = [_21,_18,_21,_6,_18,_18,_21,_21];
_27 = Field::<isize>(Variant(_14, 0), 0) ^ Field::<isize>(Variant(_15, 0), 0);
_34 = '\u{a628e}';
_32 = [159_u8,92_u8,213_u8,11_u8,59_u8,214_u8,230_u8,0_u8];
_12 = _28;
_13 = -15573_i16;
_25 = true;
_26.0.1 = _17;
_26.0.2 = 17687_u16 as i128;
_30 = core::ptr::addr_of!((*_30));
(*_11) = _7;
_18 = Field::<isize>(Variant(_14, 0), 0) as i128;
(*_11) = _7 ^ _7;
(*_11) = _7;
_26.0.1 = _27 as i8;
_8 = _27 << _26.0.1;
(*_11) = _20 as i64;
place!(Field::<isize>(Variant(_15, 0), 0)) = Field::<isize>(Variant(_14, 0), 0);
_12 = RET ^ _28;
place!(Field::<isize>(Variant(_14, 0), 0)) = _4 << _10;
(*_11) = _7 & _7;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_27 = _10 << (*_11);
match _17 {
0 => bb14,
1 => bb10,
2 => bb16,
3 => bb17,
340282366920938463463374607431768211416 => bb19,
_ => bb18
}
}
bb32 = {
(*_11) = _7;
_13 = (-7864_i16) & 15546_i16;
_19 = !_10;
(*_11) = _7 >> Field::<isize>(Variant(_14, 0), 0);
(*_11) = _7 >> _1;
(*_11) = !_7;
_5 = _7 ^ _7;
_9 = [4282216555_u32,468421478_u32,3189697168_u32];
(*_11) = _7;
_6 = 18226270246184483754_u64 as i128;
_2 = _9;
_2 = [2481782968_u32,491954154_u32,160596516_u32];
(*_11) = _7;
_18 = _1 << _10;
(*_11) = '\u{e1dbf}' as i64;
_19 = -_4;
Goto(bb7)
}
bb33 = {
_4 = Field::<isize>(Variant(_14, 0), 0) + _27;
(*_11) = 147_u8 as i64;
_31 = 112_u8 & 27_u8;
(*_11) = !_7;
(*_11) = _13 as i64;
(*_11) = _7;
_26.0.1 = _17 - _17;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25 = false;
_6 = _25 as i128;
_39 = !_18;
_16 = Field::<isize>(Variant(_15, 0), 0) << _27;
(*_11) = _7;
_33 = [_39,_39];
_11 = core::ptr::addr_of!((*_11));
_28 = !RET;
_46.1 = [_34];
Goto(bb22)
}
bb34 = {
_40 = _26.0.1 - _17;
_49.0.3 = core::ptr::addr_of_mut!(_12);
_17 = _40 << Field::<isize>(Variant(_14, 0), 0);
_24 = [_18,_39,_1,_1,_18,_39,_39,_39];
_51 = 289988330391619061828655698058090359189_u128 * 264618159064999227647101977560965612995_u128;
(*_11) = _7;
_40 = _17;
_44 = _34;
_49.1 = !_43;
_27 = _1 as isize;
_21 = _1 | _18;
(*_11) = _7 & _7;
_54 = _4 + _16;
(*_11) = _7;
_32 = [_31,_31,_31,_31,_31,_31,_31,_31];
_52 = _44;
_41 = _47 + _47;
_51 = !271982049726018000970093683782489114344_u128;
_9 = _2;
_29.0 = core::ptr::addr_of_mut!(_58.0);
_24 = [_21,_26.0.2,_26.0.2,_21,_21,_21,_39,_26.0.2];
_43 = !_49.1;
Goto(bb35)
}
bb35 = {
(*_11) = _7;
_6 = _21 ^ _21;
_55 = (*_11) as isize;
_45 = _31 & _31;
_58.1 = Adt21::Variant0 { fld0: _4 };
_18 = _6;
(*_11) = _18 as i64;
_52 = _44;
_46.2.0 = RET & RET;
_49.0.1 = [57776_u16];
RET = _16 as i32;
_46.2.1 = core::ptr::addr_of!(_14);
_26.1 = _52 as usize;
_49.2 = &mut _46.2.2;
_47 = (*_42) as f32;
(*_11) = _26.0.2 as i64;
(*_11) = 65283_u16 as i64;
RET = _28 >> _26.0.2;
_27 = Field::<isize>(Variant(_58.1, 0), 0) | _16;
_34 = _44;
_48 = _20;
(*_11) = !_7;
place!(Field::<isize>(Variant(_58.1, 0), 0)) = (*_11) as isize;
_55 = _18 as isize;
_26.0.2 = _45 as i128;
Goto(bb36)
}
bb36 = {
Call(_65 = dump_var(Move(_17), Move(_51), Move(_33), Move(_20)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_65 = dump_var(Move(_28), Move(_18), Move(_52), Move(_24)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_65 = dump_var(Move(_21), Move(_10), Move(_45), Move(_32)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_65 = dump_var(Move(_40), Move(_31), Move(_55), Move(_39)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_65 = dump_var(Move(_13), Move(_8), _66, _66), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(6991282720085458004_u64), std::hint::black_box(221_u8), std::hint::black_box(2699498041_u32), std::hint::black_box((-66_i8)), std::hint::black_box((-28801_i16)), std::hint::black_box(247065618586610739512720254042165199427_u128), std::hint::black_box((-2686475020433183561_i64)), std::hint::black_box((-124145462537865171108106049310340730072_i128)), std::hint::black_box(17657552053415438332_usize));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug)]
pub enum Adt17 {
Variant0{
fld0: f64,
fld1: char,
fld2: isize,
fld3: u128,
fld4: i16,
fld5: f32,

},
Variant1{
fld0: u8,

},
Variant2{
fld0: u128,
fld1: char,
fld2: u8,
fld3: i8,
fld4: f32,
fld5: i64,

},
Variant3{
fld0: u128,
fld1: char,
fld2: u32,
fld3: f64,
fld4: u64,
fld5: i32,
fld6: i64,
fld7: i128,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt21 {
Variant0{
fld0: isize,

},
Variant1{
fld0: i64,
fld1: usize,
fld2: (u16,),
fld3: u8,

},
Variant2{
fld0: u32,
fld1: char,
fld2: f32,
fld3: f64,
fld4: usize,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt26 {
fld0: u64,
fld1: u16,
fld2: (u16,),
fld3: usize,
fld4: u128,
fld5: Adt21,
}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: isize,

},
Variant1{
fld0: *const Adt21,
fld1: char,
fld2: Adt26,
fld3: [i128; 8],

},
Variant2{
fld0: *mut *mut u16,
fld1: [char; 1],

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: [i128; 2],
fld1: *mut i64,

},
Variant1{
fld0: f64,
fld1: char,
fld2: *const char,
fld3: *mut u16,
fld4: [u32; 4],
fld5: f32,
fld6: i64,
fld7: Adt26,

}}
#[derive(Debug)]
pub enum Adt64 {
Variant0{
fld0: ([u16; 5], i64),
fld1: f32,
fld2: usize,
fld3: (f32, f64, i64),
fld4: [u16; 5],
fld5: u128,
fld6: (u32,),
fld7: *const char,

},
Variant1{
fld0: (u16,),
fld1: [u16; 5],
fld2: [u16; 1],
fld3: [i16; 1],
fld4: Adt17,

}}
#[derive(Debug)]
pub enum Adt66 {
Variant0{
fld0: [i16; 1],
fld1: ((u16,), (char, i64, Adt17)),
fld2: ([u16; 5], i64),

},
Variant1{
fld0: [i128; 8],
fld1: u64,
fld2: [u32; 6],
fld3: (char, i64, Adt17),
fld4: u16,

},
Variant2{
fld0: bool,
fld1: [u32; 6],
fld2: Adt26,
fld3: Adt64,
fld4: [u128; 7],
fld5: i32,
fld6: *mut i64,

},
Variant3{
fld0: *mut *mut u16,
fld1: u8,
fld2: (u16,),

}}
#[derive(Debug)]
pub struct Adt81 {
fld0: bool,
fld1: ((u16,), (char, i64, Adt17)),
fld2: (f32, f64, i64),
fld3: Adt64,
fld4: (*mut *mut u16, u128),
fld5: Adt52,
}

