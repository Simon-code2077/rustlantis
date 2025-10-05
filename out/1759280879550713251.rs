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
pub fn fn0(mut _1: u128,mut _2: u8) -> isize {
mir! {
type RET = isize;
let _3: i128;
let _4: bool;
let _5: isize;
let _6: f64;
let _7: i64;
let _8: &'static &'static (i32, (usize, u128, f32), f64);
let _9: *mut u32;
let _10: u128;
let _11: [i32; 4];
let _12: bool;
let _13: isize;
let _14: char;
let _15: ([i128; 4], &'static [char; 4]);
let _16: [i32; 2];
let _17: *mut [i16; 4];
let _18: bool;
let _19: *const [usize; 3];
let _20: isize;
let _21: *const isize;
let _22: [u32; 8];
let _23: i32;
let _24: f64;
let _25: isize;
let _26: *mut [i16; 4];
let _27: ();
let _28: ();
{
_2 = 96_u8;
_1 = !82889043068346254669421014956661606874_u128;
_4 = _2 == _2;
RET = !(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_2 = 206_u8;
_3 = (-130611383652957053259769213873133231192_i128);
_1 = RET as u128;
_1 = 180860670368777204225484853027112836209_u128 >> RET;
_7 = (-1822212428347050771_i64) << _1;
_7 = -3023506089453020145_i64;
RET = 9223372036854775807_isize & (-9223372036854775808_isize);
_1 = 321665688693897491385699631085862363288_u128 - 322539453877064964461417557840442193937_u128;
_5 = 35_i8 as isize;
_7 = !6553410455229905248_i64;
_3 = (-143426010313543495003565310708742992997_i128) << _2;
_6 = _2 as f64;
_10 = _1;
_3 = 6_usize as i128;
_4 = true ^ false;
_4 = _10 != _1;
_6 = (-89_i8) as f64;
Call(_1 = fn1(_2, _4, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _10;
_3 = _6 as i128;
_10 = _1 & _1;
_4 = true;
_10 = !_1;
_4 = _1 == _10;
_10 = 5698437826000139081_u64 as u128;
_1 = _10 << _3;
RET = _5 + _5;
_11 = [2044463640_i32,(-488569277_i32),(-66310939_i32),1124601348_i32];
_7 = -5679076152518133759_i64;
_10 = _1 * _1;
RET = _5 ^ _5;
_13 = RET;
match _2 {
0 => bb2,
1 => bb3,
206 => bb5,
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
_2 = 142_u8 & 200_u8;
_5 = RET + _13;
_12 = _4;
_7 = !3464057944993228168_i64;
_14 = '\u{2e217}';
_4 = !_12;
_10 = _1;
_14 = '\u{108293}';
_15.0 = [_3,_3,_3,_3];
_13 = _7 as isize;
_4 = _12;
_2 = 122_u8 >> _5;
_4 = _12 | _12;
_14 = '\u{10142b}';
_16 = [(-629580737_i32),(-1452327029_i32)];
_7 = (-6942066624778649194_i64) & 6211041488975067508_i64;
_2 = !83_u8;
RET = _5 - _5;
_14 = '\u{eea31}';
Goto(bb6)
}
bb6 = {
_20 = RET >> _5;
_18 = !_4;
_13 = -_20;
_15.0 = [_3,_3,_3,_3];
_12 = _20 < _20;
_1 = !_10;
_11 = [(-285222392_i32),1194766086_i32,(-50068148_i32),(-1140626651_i32)];
_12 = _4;
_10 = _6 as u128;
_2 = 68_u8;
_7 = 3306830079775190517_i64 | 7496709565947656899_i64;
_12 = _18 > _4;
_20 = RET;
_6 = 637608959_u32 as f64;
_13 = _20;
_18 = RET != _13;
_1 = _10 ^ _10;
_20 = _13 >> _7;
_11 = [1765751143_i32,194019307_i32,1058436195_i32,225106076_i32];
match _2 {
0 => bb2,
68 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_21 = core::ptr::addr_of!(_5);
(*_21) = !RET;
RET = (*_21);
_4 = _18;
(*_21) = 9770751525325125878_u64 as isize;
(*_21) = _7 as isize;
_23 = (-1132660303_i32);
(*_21) = !_13;
_10 = !_1;
_24 = _6 - _6;
_3 = !22664247689879919530926345934293248476_i128;
_6 = _24 * _24;
(*_21) = _20 - RET;
_22 = [3041352968_u32,3912180829_u32,581184375_u32,4129316035_u32,3006689527_u32,730123188_u32,327086838_u32,316630896_u32];
(*_21) = !_20;
(*_21) = 2612919993_u32 as isize;
_2 = !246_u8;
_25 = RET;
RET = _25 ^ _13;
(*_21) = _20 & RET;
Goto(bb9)
}
bb9 = {
Call(_27 = dump_var(Move(_20), Move(_11), Move(_13), Move(_16)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_27 = dump_var(Move(_23), Move(_3), Move(_10), Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u8,mut _2: bool,mut _3: isize) -> u128 {
mir! {
type RET = u128;
let _4: bool;
let _5: i128;
let _6: isize;
let _7: &'static Adt25;
let _8: ([u16; 5], *mut char, i16, [i16; 4]);
let _9: i128;
let _10: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _11: [i16; 5];
let _12: &'static f64;
let _13: *mut *mut char;
let _14: usize;
let _15: u32;
let _16: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _17: *const u64;
let _18: u8;
let _19: u16;
let _20: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _21: &'static mut usize;
let _22: char;
let _23: &'static u8;
let _24: &'static mut Adt42;
let _25: &'static [char; 4];
let _26: &'static [char; 4];
let _27: Adt84;
let _28: i32;
let _29: i64;
let _30: u32;
let _31: char;
let _32: *mut &'static Adt25;
let _33: f64;
let _34: Adt18;
let _35: bool;
let _36: &'static &'static (i32, (usize, u128, f32), f64);
let _37: u64;
let _38: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _39: i16;
let _40: Adt78;
let _41: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _42: u64;
let _43: f32;
let _44: bool;
let _45: usize;
let _46: *mut &'static Adt25;
let _47: Adt18;
let _48: isize;
let _49: bool;
let _50: i128;
let _51: char;
let _52: *mut *mut char;
let _53: [char; 8];
let _54: bool;
let _55: &'static mut usize;
let _56: &'static (i32, (usize, u128, f32), f64);
let _57: i64;
let _58: Adt25;
let _59: u32;
let _60: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _61: [i128; 5];
let _62: i8;
let _63: f64;
let _64: &'static i128;
let _65: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _66: char;
let _67: i128;
let _68: [u128; 5];
let _69: &'static f64;
let _70: [char; 6];
let _71: *mut [i16; 4];
let _72: *const u64;
let _73: bool;
let _74: isize;
let _75: char;
let _76: [char; 6];
let _77: i8;
let _78: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _79: u128;
let _80: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _81: [i16; 5];
let _82: *const [char; 4];
let _83: isize;
let _84: &'static i128;
let _85: u8;
let _86: u64;
let _87: i64;
let _88: isize;
let _89: [char; 8];
let _90: usize;
let _91: f64;
let _92: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _93: char;
let _94: f64;
let _95: [i32; 1];
let _96: [u32; 8];
let _97: Adt62;
let _98: u32;
let _99: u8;
let _100: &'static [char; 4];
let _101: u8;
let _102: isize;
let _103: &'static i128;
let _104: *mut f64;
let _105: Adt25;
let _106: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _107: isize;
let _108: *mut f64;
let _109: u16;
let _110: i16;
let _111: ([i128; 4], &'static [char; 4]);
let _112: i64;
let _113: *mut char;
let _114: [isize; 8];
let _115: f64;
let _116: i64;
let _117: f64;
let _118: [i32; 2];
let _119: &'static mut usize;
let _120: f32;
let _121: &'static u8;
let _122: [isize; 8];
let _123: char;
let _124: usize;
let _125: f32;
let _126: Adt68;
let _127: f32;
let _128: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _129: ();
let _130: ();
{
_1 = 196_u8;
RET = 312827245798779085205329786693116900386_u128 * 307264727926592667206767906949065350335_u128;
_3 = 16_isize >> RET;
RET = 25832012071847573591022122320896759525_u128 & 316651449750880189031581294801915225614_u128;
_2 = false;
_2 = !true;
RET = 99050700933465166957966943031641686873_u128 - 88665767863069218756366526641285584893_u128;
_1 = 79_u8;
RET = 57039690961168845996825144271309694218_u128 + 109530937560593121742936679927762534817_u128;
RET = 105840683620874957098110129051916774454_i128 as u128;
_2 = _1 >= _1;
_2 = !false;
_1 = 163_u8 * 188_u8;
RET = 3_i8 as u128;
RET = !286863101387498848095280791784450384685_u128;
RET = 17351898045503429437_u64 as u128;
_1 = 54_u8;
RET = 292281427598530923462738549084939746224_u128;
_3 = !(-15_isize);
match _1 {
0 => bb1,
54 => bb3,
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
_4 = _2 == _2;
_2 = _4 ^ _4;
_1 = 193_u8 - 198_u8;
RET = 292995212820425173286638479725759053764_u128 & 35811633809191876438008157290289522199_u128;
_3 = (-9223372036854775808_isize);
RET = 329401817403175778923281065722326930460_u128 | 327572006731337827389485581981826290070_u128;
_5 = 26424_u16 as i128;
RET = _5 as u128;
_5 = 130742612060581427376927802808577883874_i128 >> _1;
_1 = !209_u8;
_3 = _5 as isize;
_4 = _1 == _1;
_5 = 105183176671241979906090407065160298226_i128 >> _1;
_2 = _4;
_5 = (-160394475878601761481547173289233431640_i128) + 66374950254644930870827523691225428099_i128;
_2 = _4;
_5 = (-68569899120244783911867263596948558825_i128);
_4 = _3 == _3;
_4 = !_2;
_3 = 9223372036854775807_isize * (-9223372036854775808_isize);
RET = !212324901460727700457275411885364640169_u128;
_3 = 9223372036854775807_isize - (-9223372036854775808_isize);
_6 = _3 | _3;
match _5 {
0 => bb2,
271712467800693679551507343834819652631 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
_6 = _3;
_8.0 = [54604_u16,51617_u16,59073_u16,61958_u16,21001_u16];
_6 = _3 & _3;
_10.1.3 = [22023_i16,(-30088_i16),6483_i16,3033_i16];
_10.1.2 = 23814_i16 ^ 28020_i16;
_10.1.3 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2];
_8.3 = [_10.1.2,_10.1.2,_10.1.2,_10.1.2];
Goto(bb6)
}
bb6 = {
_8.3 = _10.1.3;
_9 = RET as i128;
_5 = -_9;
_3 = _6;
_8.0 = [58987_u16,36846_u16,9388_u16,38553_u16,61709_u16];
_10.1.2 = _1 as i16;
_8.2 = !_10.1.2;
Goto(bb7)
}
bb7 = {
_10.1.3 = [_8.2,_10.1.2,_8.2,_10.1.2];
_10.1.0 = [12307_u16,31686_u16,8472_u16,24252_u16,26289_u16];
_10.1.0 = [50957_u16,39495_u16,57824_u16,38198_u16,20605_u16];
_8.2 = -_10.1.2;
_10.1.2 = RET as i16;
_6 = _3;
_8.3 = _10.1.3;
_6 = (-973658085_i32) as isize;
_3 = _6 & _6;
_9 = _5 << RET;
_8.2 = _10.1.2;
RET = !192277153846467024028787620813187719504_u128;
_6 = _3 - _3;
RET = !298837571476049138533779511868478846959_u128;
_10.1.0 = [53941_u16,58291_u16,58002_u16,18121_u16,47285_u16];
RET = 194282199950650236479897505231940576049_u128 + 187096311918570507686985568713950165708_u128;
_2 = !_4;
_5 = _9 ^ _9;
_10.1.3 = [_8.2,_10.1.2,_10.1.2,_10.1.2];
_10.1.0 = _8.0;
_11 = [_8.2,_8.2,_8.2,_10.1.2,_8.2];
_10.0 = _8.3;
_11 = [_10.1.2,_8.2,_10.1.2,_8.2,_10.1.2];
_4 = _2 | _2;
_10.1.0 = [21654_u16,5441_u16,39853_u16,11703_u16,49629_u16];
_4 = _9 < _9;
_10.1.2 = _8.2 * _8.2;
_1 = 32_u8 >> _6;
_6 = -_3;
Goto(bb8)
}
bb8 = {
_1 = 163_u8 + 69_u8;
_8.0 = _10.1.0;
_10.1.0 = _8.0;
_16.0 = [0_usize,4540894032673410415_usize,6_usize];
_8.3 = [_10.1.2,_8.2,_8.2,_8.2];
_1 = !208_u8;
_16.1.0 = _8.3;
_16.1.0 = _10.0;
_14 = _4 as usize;
_16.1.1.0 = [34601_u16,9125_u16,54273_u16,14061_u16,25586_u16];
_10.1.3 = _10.0;
_8.0 = [49605_u16,6632_u16,8388_u16,32807_u16,46520_u16];
_10.1.0 = [2542_u16,50730_u16,56065_u16,44155_u16,51070_u16];
_20.2 = &mut _14;
_15 = 82_i8 as u32;
_21 = Move(_20.2);
_19 = 28826_u16 ^ 36942_u16;
RET = 17403444112719444243_usize as u128;
_4 = !_2;
_18 = _1;
_20.0.1.3 = _8.3;
Goto(bb9)
}
bb9 = {
_16.1.1.2 = !_8.2;
_10.1.1 = core::ptr::addr_of_mut!(_22);
_20.0.1.0 = [_19,_19,_19,_19,_19];
_3 = -_6;
_13 = core::ptr::addr_of_mut!(_10.1.1);
_22 = '\u{ca2}';
_8.0 = _10.1.0;
_20.0.0 = [_16.1.1.2,_8.2,_10.1.2,_8.2];
(*_13) = core::ptr::addr_of_mut!(_22);
_23 = &_1;
_16.0 = [11716057727050741762_usize,1_usize,5_usize];
_16.1.1.2 = _8.2;
(*_13) = core::ptr::addr_of_mut!(_22);
(*_13) = core::ptr::addr_of_mut!(_22);
_20.0.1.0 = [_19,_19,_19,_19,_19];
(*_13) = core::ptr::addr_of_mut!(_22);
_8 = (_16.1.1.0, Move((*_13)), _10.1.2, _10.0);
_8.3 = [_16.1.1.2,_16.1.1.2,_8.2,_8.2];
(*_13) = core::ptr::addr_of_mut!(_22);
_10.1.0 = _20.0.1.0;
_16.1.1.3 = _10.1.3;
Call(_16.1.1.0 = core::intrinsics::transmute(_11), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = 1462546636_u32 << (*_23);
_16.1.1.1 = core::ptr::addr_of_mut!(_22);
_20.0.1.1 = core::ptr::addr_of_mut!(_22);
_18 = _15 as u8;
(*_13) = Move(_20.0.1.1);
_16.1.1 = (_8.0, Move((*_13)), _8.2, _8.3);
_20.0.1.2 = 6339479909708188988_u64 as i16;
_16.1.1.2 = _10.1.2 >> _5;
RET = _3 as u128;
Goto(bb11)
}
bb11 = {
_29 = 6445608337132430687_i64 << _5;
(*_13) = core::ptr::addr_of_mut!(_22);
_5 = _9;
(*_13) = Move(_16.1.1.1);
_20.0.1.1 = core::ptr::addr_of_mut!(_22);
(*_13) = Move(_20.0.1.1);
(*_13) = core::ptr::addr_of_mut!(_22);
(*_13) = Move(_8.1);
_15 = 408514721_u32;
_8.1 = Move((*_13));
(*_13) = core::ptr::addr_of_mut!(_22);
_20.1 = [_22,_22,_22,_22,_22,_22,_22,_22];
_13 = core::ptr::addr_of_mut!((*_13));
(*_13) = core::ptr::addr_of_mut!(_22);
_20.1 = [_22,_22,_22,_22,_22,_22,_22,_22];
_16.1.1.0 = _8.0;
_10.0 = [_16.1.1.2,_20.0.1.2,_16.1.1.2,_16.1.1.2];
(*_13) = core::ptr::addr_of_mut!(_22);
_20.0.1.2 = _16.1.1.2 ^ _16.1.1.2;
_15 = !1143219539_u32;
_8.1 = core::ptr::addr_of_mut!(_22);
_10.1.1 = Move(_8.1);
_8.1 = Move((*_13));
_9 = _5 + _5;
_10.1.0 = [_19,_19,_19,_19,_19];
_30 = !_15;
_20.0.1 = (_16.1.1.0, Move(_8.1), _8.2, _8.3);
_10.0 = [_16.1.1.2,_8.2,_16.1.1.2,_16.1.1.2];
Goto(bb12)
}
bb12 = {
_9 = _30 as i128;
_8.0 = [_19,_19,_19,_19,_19];
(*_13) = core::ptr::addr_of_mut!(_22);
(*_13) = core::ptr::addr_of_mut!(_31);
_29 = -(-987944251028896072_i64);
_11 = [_20.0.1.2,_8.2,_16.1.1.2,_16.1.1.2,_10.1.2];
(*_13) = core::ptr::addr_of_mut!(_31);
_10.1.2 = _20.0.1.2 * _20.0.1.2;
(*_13) = core::ptr::addr_of_mut!(_31);
_22 = '\u{49634}';
_10.1.0 = [_19,_19,_19,_19,_19];
_20.0.1.0 = [_19,_19,_19,_19,_19];
RET = (-8_i8) as u128;
_16.1.1.3 = [_16.1.1.2,_10.1.2,_20.0.1.2,_16.1.1.2];
_20.0.1.3 = [_8.2,_20.0.1.2,_16.1.1.2,_16.1.1.2];
_20.0.0 = [_10.1.2,_16.1.1.2,_20.0.1.2,_16.1.1.2];
_4 = _16.1.1.2 < _16.1.1.2;
_8 = Move(_20.0.1);
Call(_16.1.1.1 = fn2(Move((*_13)), Move(_13), _19, Move(_8.1), _8.2, Move(_23), _5, (*_23), _29, _22), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.0.1.1 = core::ptr::addr_of_mut!(_31);
_20.0.1.2 = _15 as i16;
_4 = _3 > _6;
_28 = -(-1866113583_i32);
_30 = _15 * _15;
_5 = 5_i8 as i128;
_20.0.1 = (_16.1.1.0, Move(_16.1.1.1), _16.1.1.2, _16.1.1.3);
_8 = (_10.1.0, Move(_20.0.1.1), _16.1.1.2, _10.0);
_33 = _8.2 as f64;
_3 = _6;
_12 = &_33;
_20.0.1.2 = _30 as i16;
_10.1.1 = core::ptr::addr_of_mut!(_31);
_20.0.1 = (_16.1.1.0, Move(_8.1), _8.2, _8.3);
_16.0 = [7_usize,1_usize,3_usize];
_18 = _1 << _3;
_8.1 = core::ptr::addr_of_mut!(_22);
_10.2 = Adt25::Variant0 { fld0: (*_12),fld1: _30 };
_30 = 15610805396163724119_u64 as u32;
_20.0.2 = Move(_10.2);
_16.1.1.3 = _8.3;
_20.0.1.2 = _29 as i16;
_16.1.1 = (_20.0.1.0, Move(_20.0.1.1), _8.2, _20.0.0);
_13 = core::ptr::addr_of_mut!(_8.1);
_16.1 = (_8.3, Move(_8), Move(_20.0.2));
Goto(bb14)
}
bb14 = {
(*_13) = core::ptr::addr_of_mut!(_31);
_8 = Move(_16.1.1);
_8.1 = core::ptr::addr_of_mut!(_31);
_13 = core::ptr::addr_of_mut!((*_13));
(*_13) = core::ptr::addr_of_mut!(_22);
_8.2 = -_10.1.2;
_22 = '\u{36654}';
_16.1.1 = (_20.0.1.0, Move((*_13)), _10.1.2, _10.0);
place!(Field::<u32>(Variant(_16.1.2, 0), 1)) = _15 ^ _30;
_11 = [_10.1.2,_8.2,_16.1.1.2,_16.1.1.2,_16.1.1.2];
_19 = _5 as u16;
_16.1.0 = [_8.2,_10.1.2,_8.2,_8.2];
_10.1.2 = _20.0.1.2 * _16.1.1.2;
_5 = _30 as i128;
_16.1.2 = Adt25::Variant0 { fld0: (*_12),fld1: _15 };
place!(Field::<u32>(Variant(_16.1.2, 0), 1)) = _30 >> _3;
_20.0 = Move(_16.1);
(*_13) = Move(_20.0.1.1);
_10.2 = Adt25::Variant0 { fld0: (*_12),fld1: Field::<u32>(Variant(_20.0.2, 0), 1) };
_16.1.0 = [_8.2,_10.1.2,_10.1.2,_20.0.1.2];
_20.0 = Move(_10);
_10 = (_20.0.0, Move(_8), Move(_20.0.2));
_29 = (-6910051504820336431_i64) * (-7848672654666260761_i64);
_16.0 = [4_usize,4868217797104405093_usize,10318254230044594974_usize];
_10.0 = [_10.1.2,_10.1.2,_20.0.1.2,_20.0.1.2];
(*_13) = Move(_10.1.1);
_8.3 = _20.0.0;
_23 = &_1;
Goto(bb15)
}
bb15 = {
(*_13) = core::ptr::addr_of_mut!(_22);
_15 = Field::<u32>(Variant(_10.2, 0), 1);
_16.1.1.0 = [_19,_19,_19,_19,_19];
_16.1.1.1 = core::ptr::addr_of_mut!(_22);
_15 = 11483046104889539379_usize as u32;
_9 = _10.1.2 as i128;
_7 = &_10.2;
_16.1.1.2 = _10.1.2;
(*_13) = core::ptr::addr_of_mut!(_31);
_16.1.2 = Move(_10.2);
(*_13) = core::ptr::addr_of_mut!(_22);
_10 = (_20.0.0, Move(_20.0.1), Move(_16.1.2));
(*_13) = core::ptr::addr_of_mut!(_31);
(*_13) = core::ptr::addr_of_mut!(_22);
_37 = 14408551699639141738_u64 | 5650926306791941131_u64;
(*_13) = Move(_16.1.1.1);
_42 = _10.1.2 as u64;
_20.0.1 = (_16.1.1.0, Move((*_13)), _10.1.2, _20.0.0);
_8.0 = _20.0.1.0;
_44 = (*_12) >= (*_12);
Call(_42 = core::intrinsics::bswap(_37), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
(*_13) = core::ptr::addr_of_mut!(_31);
_34 = Adt18::Variant1 { fld0: _2,fld1: _22,fld2: _3,fld3: _29,fld4: _37,fld5: (*_12) };
_20.0.2 = Adt25::Variant0 { fld0: (*_12),fld1: Field::<u32>(Variant(_10.2, 0), 1) };
_35 = !Field::<bool>(Variant(_34, 1), 0);
_32 = core::ptr::addr_of_mut!(_7);
_10.1.0 = _16.1.1.0;
_37 = !Field::<u64>(Variant(_34, 1), 4);
_35 = (*_12) != (*_12);
_13 = core::ptr::addr_of_mut!((*_13));
_42 = Field::<i64>(Variant(_34, 1), 3) as u64;
_16.1.1 = (_10.1.0, Move(_20.0.1.1), _20.0.1.2, _20.0.1.3);
(*_32) = &_20.0.2;
place!(Field::<i64>(Variant(_34, 1), 3)) = -_29;
(*_32) = &_10.2;
_16.1.1.1 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_10.0 = [_10.1.2,_10.1.2,_16.1.1.2,_16.1.1.2];
_31 = _22;
Goto(bb17)
}
bb17 = {
_16.1 = Move(_10);
_17 = core::ptr::addr_of!(_37);
(*_13) = core::ptr::addr_of_mut!(_31);
RET = 125199057210588510809818411640016208135_u128 - 35140352442359098724793997515276347692_u128;
(*_13) = core::ptr::addr_of_mut!(_22);
(*_17) = !Field::<u64>(Variant(_34, 1), 4);
_23 = &_18;
_45 = 2_usize;
(*_17) = _8.0[_45] as u64;
_20.0.0[_45] = _20.0.1.2;
_10.1.0[_45] = !_8.0[_45];
_8.1 = core::ptr::addr_of_mut!(_31);
_20.2 = &mut _45;
Goto(bb18)
}
bb18 = {
_47 = _34;
_20.1 = [_22,_22,Field::<char>(Variant(_47, 1), 1),_31,Field::<char>(Variant(_47, 1), 1),_22,Field::<char>(Variant(_47, 1), 1),Field::<char>(Variant(_47, 1), 1)];
_28 = (*_23) as i32;
place!(Field::<isize>(Variant(_34, 1), 2)) = Field::<isize>(Variant(_47, 1), 2) * Field::<isize>(Variant(_47, 1), 2);
_10.1.0 = [_19,_19,_19,_19,_19];
(*_17) = !_42;
Goto(bb19)
}
bb19 = {
(*_17) = _42 - Field::<u64>(Variant(_34, 1), 4);
_10.2 = Adt25::Variant0 { fld0: (*_12),fld1: _30 };
_20.0 = (_8.3, Move(_16.1.1), Move(_10.2));
(*_13) = core::ptr::addr_of_mut!(_31);
_13 = core::ptr::addr_of_mut!((*_13));
_10.0 = [_20.0.1.2,_20.0.1.2,_20.0.1.2,_20.0.1.2];
_10.1.1 = core::ptr::addr_of_mut!(_22);
_4 = (*_17) < (*_17);
(*_32) = &_16.1.2;
place!(Field::<i64>(Variant(_47, 1), 3)) = Field::<i64>(Variant(_34, 1), 3) - _29;
_33 = RET as f64;
_16.1.1.0 = _20.0.1.0;
_16.1.1.2 = -_20.0.1.2;
(*_13) = core::ptr::addr_of_mut!(_31);
(*_32) = &_20.0.2;
_10.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_7), 0), 0),fld1: Field::<u32>(Variant((*_7), 0), 1) };
_53 = [_22,_31,_22,Field::<char>(Variant(_34, 1), 1),Field::<char>(Variant(_34, 1), 1),_31,Field::<char>(Variant(_47, 1), 1),_22];
_29 = Field::<i64>(Variant(_47, 1), 3) - Field::<i64>(Variant(_34, 1), 3);
_2 = _35;
(*_17) = _19 as u64;
Goto(bb20)
}
bb20 = {
_16.1 = (_8.3, Move(_20.0.1), Move(_20.0.2));
(*_32) = &_16.1.2;
_34 = _47;
_18 = _1;
(*_32) = &_10.2;
(*_17) = Field::<u64>(Variant(_47, 1), 4);
_20.0.1 = Move(_16.1.1);
(*_17) = !Field::<u64>(Variant(_47, 1), 4);
place!(Field::<u32>(Variant(_10.2, 0), 1)) = Field::<u32>(Variant(_16.1.2, 0), 1) << _28;
(*_13) = Move(_10.1.1);
(*_17) = _30 as u64;
_50 = _9 & _9;
Goto(bb21)
}
bb21 = {
_52 = core::ptr::addr_of_mut!(_20.0.1.1);
place!(Field::<f64>(Variant(_16.1.2, 0), 0)) = (-10_i8) as f64;
_55 = Move(_20.2);
(*_17) = _42;
(*_13) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_8.0 = [_19,_19,_19,_19,_19];
Call(place!(Field::<isize>(Variant(_47, 1), 2)) = core::intrinsics::bswap(_6), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_20.0.0 = [_20.0.1.2,_20.0.1.2,_20.0.1.2,_20.0.1.2];
_16.1.1 = (_20.0.1.0, Move((*_13)), _20.0.1.2, _10.0);
_16.1.2 = Move(_10.2);
_20.0.1 = (_8.0, Move(_16.1.1.1), _16.1.1.2, _8.3);
(*_17) = _42;
(*_32) = &_16.1.2;
_6 = _3;
_8 = (_16.1.1.0, Move((*_52)), _16.1.1.2, _16.1.1.3);
_58 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_7), 0), 0),fld1: Field::<u32>(Variant((*_7), 0), 1) };
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_20.0 = (_16.1.0, Move(_8), Move(_58));
Goto(bb23)
}
bb23 = {
_60.2 = Move(_20.0.2);
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
(*_32) = &_60.2;
(*_13) = core::ptr::addr_of_mut!(_22);
_44 = _4 ^ _4;
Goto(bb24)
}
bb24 = {
_16.1.1.0 = _10.1.0;
_60.1.0 = [_19,_19,_19,_19,_19];
(*_17) = Field::<u64>(Variant(_47, 1), 4) | Field::<u64>(Variant(_47, 1), 4);
_8.1 = core::ptr::addr_of_mut!(_22);
(*_32) = &_16.1.2;
(*_17) = !Field::<u64>(Variant(_47, 1), 4);
(*_32) = &_60.2;
(*_17) = _42 & _42;
(*_32) = &_16.1.2;
(*_13) = core::ptr::addr_of_mut!(_22);
_54 = Field::<u32>(Variant((*_7), 0), 1) < Field::<u32>(Variant((*_7), 0), 1);
place!(Field::<isize>(Variant(_47, 1), 2)) = _3 * _3;
_4 = !_54;
_16.1.1.2 = _20.0.1.2 & _20.0.1.2;
(*_17) = Field::<u64>(Variant(_34, 1), 4);
(*_32) = &_60.2;
(*_32) = &_16.1.2;
(*_13) = core::ptr::addr_of_mut!(_22);
(*_13) = core::ptr::addr_of_mut!(_31);
_13 = core::ptr::addr_of_mut!((*_52));
Goto(bb25)
}
bb25 = {
_49 = _44;
place!(Field::<bool>(Variant(_47, 1), 0)) = _4;
(*_32) = &_60.2;
(*_13) = core::ptr::addr_of_mut!(_31);
place!(Field::<f64>(Variant(_34, 1), 5)) = 58_i8 as f64;
_20.0.1.2 = _16.1.1.2 ^ _16.1.1.2;
_68 = [RET,RET,RET,RET,RET];
_8.2 = _16.1.1.2;
_9 = Field::<isize>(Variant(_47, 1), 2) as i128;
_50 = Field::<f64>(Variant((*_7), 0), 0) as i128;
_20.0.2 = Move(_16.1.2);
_62 = !(-109_i8);
(*_13) = core::ptr::addr_of_mut!(_22);
_49 = !Field::<bool>(Variant(_47, 1), 0);
Goto(bb26)
}
bb26 = {
(*_52) = core::ptr::addr_of_mut!(_22);
_57 = _29 | _29;
_20.0.0 = [_8.2,_8.2,_16.1.1.2,_20.0.1.2];
(*_52) = core::ptr::addr_of_mut!(_66);
_51 = _31;
_35 = Field::<u32>(Variant((*_7), 0), 1) == Field::<u32>(Variant((*_7), 0), 1);
_37 = _42 ^ _42;
_10.1.1 = Move(_8.1);
_60.1.2 = _8.2;
_8.0 = _20.0.1.0;
Goto(bb27)
}
bb27 = {
_48 = Field::<isize>(Variant(_47, 1), 2) | Field::<isize>(Variant(_34, 1), 2);
(*_52) = core::ptr::addr_of_mut!(_51);
_67 = !_50;
place!(Field::<u64>(Variant(_47, 1), 4)) = (*_17);
_8.2 = -_20.0.1.2;
(*_32) = &_20.0.2;
(*_52) = Move(_10.1.1);
_4 = !_35;
_60.1 = (_10.1.0, Move((*_52)), _8.2, _20.0.0);
(*_17) = _42 - Field::<u64>(Variant(_47, 1), 4);
(*_32) = &_60.2;
(*_17) = !_42;
place!(Field::<u32>(Variant(_20.0.2, 0), 1)) = Field::<u32>(Variant((*_7), 0), 1) & Field::<u32>(Variant((*_7), 0), 1);
_12 = &place!(Field::<f64>(Variant((*_7), 0), 0));
(*_32) = &_20.0.2;
Call(_30 = core::intrinsics::transmute(_51), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_8.3 = [_16.1.1.2,_20.0.1.2,_8.2,_60.1.2];
_20.0 = (_8.3, Move(_60.1), Move(_60.2));
Goto(bb29)
}
bb29 = {
_51 = Field::<char>(Variant(_34, 1), 1);
_60.1.2 = _8.2;
(*_17) = !Field::<u64>(Variant(_47, 1), 4);
RET = !277172536565362799636392604347064725508_u128;
_20.1 = [_22,_31,Field::<char>(Variant(_34, 1), 1),_31,_51,_22,_51,Field::<char>(Variant(_34, 1), 1)];
_68 = [RET,RET,RET,RET,RET];
_10.1 = (_20.0.1.0, Move((*_52)), _16.1.1.2, _8.3);
_33 = Field::<f64>(Variant(_20.0.2, 0), 0) - Field::<f64>(Variant(_20.0.2, 0), 0);
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_22 = Field::<char>(Variant(_47, 1), 1);
_3 = !Field::<isize>(Variant(_34, 1), 2);
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_63 = (*_17) as f64;
_17 = core::ptr::addr_of!((*_17));
_59 = (*_17) as u32;
_61 = [_9,_67,_9,_67,_67];
_10 = (_16.1.1.3, Move(_20.0.1), Move(_20.0.2));
(*_17) = !_42;
_20.0.1.0 = _16.1.1.0;
_8.3 = [_10.1.2,_8.2,_8.2,_60.1.2];
place!(Field::<isize>(Variant(_47, 1), 2)) = (*_17) as isize;
_60.1.1 = core::ptr::addr_of_mut!(_51);
_60 = (_20.0.0, Move(_10.1), Move(_10.2));
(*_32) = &_60.2;
Goto(bb30)
}
bb30 = {
(*_17) = _42 ^ Field::<u64>(Variant(_47, 1), 4);
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_66 = _31;
(*_17) = Field::<u32>(Variant((*_7), 0), 1) as u64;
(*_17) = Field::<u64>(Variant(_47, 1), 4) ^ _42;
_81 = [_60.1.2,_60.1.2,_60.1.2,_60.1.2,_16.1.1.2];
_78 = &_60.1;
Goto(bb31)
}
bb31 = {
_20.0.1.0 = (*_78).0;
_46 = core::ptr::addr_of_mut!((*_32));
_63 = Field::<f64>(Variant((*_7), 0), 0);
place!(Field::<u32>(Variant(_60.2, 0), 1)) = _59 >> (*_78).2;
_16.1.1 = Move(_60.1);
RET = !9435061335556359263294470603320981618_u128;
_19 = _31 as u16;
_20.1 = [_31,_31,Field::<char>(Variant(_47, 1), 1),_31,Field::<char>(Variant(_34, 1), 1),_51,Field::<char>(Variant(_47, 1), 1),_22];
_60.1.2 = _16.1.1.2;
RET = _62 as u128;
_79 = RET << (*_17);
_44 = Field::<bool>(Variant(_47, 1), 0);
_48 = -Field::<isize>(Variant(_47, 1), 2);
_10.1.1 = Move((*_52));
_17 = core::ptr::addr_of!((*_17));
(*_52) = core::ptr::addr_of_mut!(_66);
_10.1.2 = _8.2 ^ _60.1.2;
_10.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_7), 0), 0),fld1: Field::<u32>(Variant(_60.2, 0), 1) };
_6 = _10.1.2 as isize;
(*_32) = &_10.2;
_79 = RET >> Field::<u32>(Variant(_10.2, 0), 1);
Goto(bb32)
}
bb32 = {
(*_32) = &_60.2;
(*_52) = Move(_16.1.1.1);
_70 = [_31,_22,_22,_51,Field::<char>(Variant(_34, 1), 1),_51];
_16.1.1.0 = [_19,_19,_19,_19,_19];
_52 = core::ptr::addr_of_mut!(_10.1.1);
(*_52) = core::ptr::addr_of_mut!(_31);
_22 = Field::<char>(Variant(_47, 1), 1);
_75 = _66;
_18 = (*_17) as u8;
(*_32) = &_10.2;
_39 = _10.1.2 - _10.1.2;
_71 = core::ptr::addr_of_mut!(_10.1.3);
_88 = _6;
_20.0.1.2 = _10.1.2;
(*_71) = _8.3;
(*_32) = &_60.2;
_10.1 = (_20.0.1.0, Move(_20.0.1.1), _8.2, _16.1.0);
_30 = _28 as u32;
_48 = -_6;
_16.1.1.0 = _8.0;
_91 = Field::<f64>(Variant(_47, 1), 5) * Field::<f64>(Variant((*_7), 0), 0);
_15 = Field::<u32>(Variant((*_7), 0), 1) | Field::<u32>(Variant((*_7), 0), 1);
Goto(bb33)
}
bb33 = {
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_92 = ((*_71), Move(_10.1), Move(_60.2));
_47 = Adt18::Variant1 { fld0: _2,fld1: Field::<char>(Variant(_34, 1), 1),fld2: _48,fld3: _57,fld4: (*_17),fld5: _33 };
_20.0.1.3 = _16.1.1.3;
(*_71) = [_39,_39,_39,_20.0.1.2];
Goto(bb34)
}
bb34 = {
_50 = _9;
_89 = [_51,Field::<char>(Variant(_34, 1), 1),_51,Field::<char>(Variant(_47, 1), 1),_51,_22,Field::<char>(Variant(_34, 1), 1),_66];
(*_52) = core::ptr::addr_of_mut!(_66);
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_60.1.1 = Move((*_52));
(*_32) = &_92.2;
(*_17) = Field::<u64>(Variant(_34, 1), 4) | Field::<u64>(Variant(_47, 1), 4);
_60.1.2 = _19 as i16;
_8.0 = _92.1.0;
(*_52) = core::ptr::addr_of_mut!(_93);
_87 = _57 >> _88;
_16.1 = Move(_92);
(*_17) = Field::<u64>(Variant(_47, 1), 4) | Field::<u64>(Variant(_47, 1), 4);
(*_52) = core::ptr::addr_of_mut!(_22);
_20.0 = Move(_16.1);
_32 = Move(_46);
_43 = _91 as f32;
_74 = !Field::<isize>(Variant(_34, 1), 2);
place!(Field::<bool>(Variant(_47, 1), 0)) = _39 > _39;
_77 = _62;
_81 = _11;
place!(Field::<f64>(Variant(_10.2, 0), 0)) = Field::<f64>(Variant(_47, 1), 5) + Field::<f64>(Variant(_20.0.2, 0), 0);
_11 = [_8.2,_39,_39,_39,_39];
_68 = [_79,_79,_79,_79,_79];
(*_52) = core::ptr::addr_of_mut!(_75);
Goto(bb35)
}
bb35 = {
_49 = _6 < _6;
(*_52) = core::ptr::addr_of_mut!(_31);
_8.2 = (*_17) as i16;
place!(Field::<f64>(Variant(_10.2, 0), 0)) = Field::<f64>(Variant(_47, 1), 5) + Field::<f64>(Variant(_47, 1), 5);
_2 = Field::<bool>(Variant(_47, 1), 0) ^ _49;
(*_71) = [_39,_39,_20.0.1.2,_39];
Goto(bb36)
}
bb36 = {
_59 = Field::<u32>(Variant(_10.2, 0), 1);
(*_71) = _10.0;
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_16.1.1.3 = _8.3;
(*_71) = [_39,_20.0.1.2,_20.0.1.2,_8.2];
_16.1.1.1 = core::ptr::addr_of_mut!(_75);
_83 = _88;
_86 = (*_17) | (*_17);
place!(Field::<bool>(Variant(_47, 1), 0)) = !_2;
_18 = _51 as u8;
(*_71) = [_20.0.1.2,_39,_8.2,_39];
(*_17) = Field::<char>(Variant(_34, 1), 1) as u64;
place!(Field::<i64>(Variant(_34, 1), 3)) = !_87;
_20.1 = [_51,_51,Field::<char>(Variant(_34, 1), 1),Field::<char>(Variant(_34, 1), 1),_66,_31,_66,Field::<char>(Variant(_47, 1), 1)];
_92.1.2 = _44 as i16;
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_34, 1), 1)));
_16.1.1.0 = [_19,_19,_19,_19,_19];
_10.0 = [_39,_39,_8.2,_39];
_67 = _50 << _59;
(*_17) = _86 << _39;
(*_71) = _10.0;
_102 = _39 as isize;
_102 = Field::<isize>(Variant(_47, 1), 2);
_60.1.1 = core::ptr::addr_of_mut!(_51);
_71 = core::ptr::addr_of_mut!(_8.3);
_12 = &_33;
Goto(bb37)
}
bb37 = {
_92.0 = _10.0;
_43 = (*_12) as f32;
_20.1 = [Field::<char>(Variant(_34, 1), 1),Field::<char>(Variant(_34, 1), 1),_31,Field::<char>(Variant(_47, 1), 1),Field::<char>(Variant(_34, 1), 1),_66,_22,Field::<char>(Variant(_34, 1), 1)];
(*_52) = core::ptr::addr_of_mut!(_22);
_71 = core::ptr::addr_of_mut!((*_71));
(*_17) = Field::<u64>(Variant(_34, 1), 4);
(*_17) = _43 as u64;
_53 = _89;
(*_71) = _16.1.1.3;
_81 = [_39,_39,_20.0.1.2,_20.0.1.2,_39];
_85 = !_1;
(*_17) = _86 & _86;
Goto(bb38)
}
bb38 = {
_61 = [_9,_67,_67,_9,_67];
(*_71) = [_92.1.2,_39,_39,_39];
(*_17) = Field::<u64>(Variant(_47, 1), 4);
_62 = _77;
(*_52) = core::ptr::addr_of_mut!(_31);
(*_71) = _92.0;
_16.1.1 = Move(_20.0.1);
Goto(bb39)
}
bb39 = {
(*_52) = core::ptr::addr_of_mut!(_93);
_20.0.1.0 = [_19,_19,_19,_19,_19];
place!(Field::<f64>(Variant(_47, 1), 5)) = _91;
_92 = ((*_71), Move(_16.1.1), Move(_10.2));
_69 = &_33;
(*_17) = Field::<u64>(Variant(_47, 1), 4) ^ _86;
_53 = [Field::<char>(Variant(_47, 1), 1),_75,_66,_51,Field::<char>(Variant(_34, 1), 1),_22,_75,_66];
_33 = _63 * Field::<f64>(Variant(_92.2, 0), 0);
(*_71) = [_92.1.2,_39,_92.1.2,_39];
place!(Field::<isize>(Variant(_47, 1), 2)) = _43 as isize;
_94 = _63;
(*_52) = core::ptr::addr_of_mut!(_75);
_102 = _6;
_99 = _1 << (*_17);
_94 = -_63;
_103 = &_5;
_73 = (*_17) != (*_17);
_98 = _15;
_49 = _2 & Field::<bool>(Variant(_47, 1), 0);
_60 = Move(_92);
_60.1.2 = 14571908227340010479_usize as i16;
_16.1.1 = (_60.1.0, Move((*_52)), _8.2, (*_71));
(*_52) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
Goto(bb40)
}
bb40 = {
_20.0.1.0 = [_19,_19,_19,_19,_19];
_57 = _87;
_94 = _77 as f64;
_20.0.1.1 = core::ptr::addr_of_mut!(_22);
Goto(bb41)
}
bb41 = {
_69 = Move(_12);
_8.0 = [_19,_19,_19,_19,_19];
_112 = _87 << _57;
_31 = _22;
(*_71) = _16.1.1.3;
_8.1 = core::ptr::addr_of_mut!(_31);
_34 = Adt18::Variant1 { fld0: _2,fld1: _31,fld2: _88,fld3: _112,fld4: (*_17),fld5: Field::<f64>(Variant(_47, 1), 5) };
Goto(bb42)
}
bb42 = {
(*_71) = [_39,_39,_8.2,_8.2];
_10.0 = (*_71);
_12 = &place!(Field::<f64>(Variant(_60.2, 0), 0));
(*_17) = _62 as u64;
place!(Field::<u32>(Variant(_20.0.2, 0), 1)) = _98 * _15;
(*_71) = [_39,_39,_39,_39];
_37 = Field::<u64>(Variant(_34, 1), 4) - _86;
place!(Field::<u64>(Variant(_47, 1), 4)) = !(*_17);
_92.1.0 = _60.1.0;
_44 = (*_12) <= (*_12);
_93 = _66;
_98 = Field::<u32>(Variant(_60.2, 0), 1) - Field::<u32>(Variant(_20.0.2, 0), 1);
_60.1.2 = _39;
(*_17) = Field::<u64>(Variant(_47, 1), 4) | Field::<u64>(Variant(_34, 1), 4);
(*_71) = [_39,_60.1.2,_39,_60.1.2];
_10.1.1 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_109 = _19;
(*_17) = _67 as u64;
_92.1.1 = Move(_8.1);
_90 = 4_usize;
_84 = &(*_103);
place!(Field::<isize>(Variant(_34, 1), 2)) = _48;
Goto(bb43)
}
bb43 = {
(*_17) = !Field::<u64>(Variant(_47, 1), 4);
_107 = _6;
_20.1 = _89;
match _90 {
0 => bb13,
1 => bb28,
4 => bb44,
_ => bb23
}
}
bb44 = {
_118 = [_28,_28];
Goto(bb45)
}
bb45 = {
place!(Field::<u64>(Variant(_34, 1), 4)) = (*_17) << (*_17);
_48 = _90 as isize;
(*_52) = core::ptr::addr_of_mut!(_75);
_92.1.3 = [_60.1.2,_16.1.1.2,_60.1.2,_39];
_95 = [_28];
_61[_90] = _43 as i128;
_28 = 722244862_i32;
_116 = (*_84) as i64;
place!(Field::<bool>(Variant(_47, 1), 0)) = _49;
_113 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_47, 1), 1)));
_92.1.0[_90] = _60.1.0[_90] ^ _20.0.1.0[_90];
_70[_90] = (*_113);
_53[_90] = Field::<char>(Variant(_34, 1), 1);
_111.0 = [_67,_67,_50,_67];
_19 = Field::<i64>(Variant(_34, 1), 3) as u16;
_95 = [_28];
_10.1.3 = (*_71);
_75 = _89[_90];
_74 = _66 as isize;
_20.2 = &mut _90;
_109 = _19 | _19;
(*_17) = Field::<u64>(Variant(_47, 1), 4);
(*_71) = _60.0;
_46 = Move(_32);
_65 = Move(_78);
place!(Field::<u32>(Variant(_60.2, 0), 1)) = Field::<u32>(Variant(_20.0.2, 0), 1) >> _60.1.2;
(*_17) = Field::<u64>(Variant(_47, 1), 4) & Field::<u64>(Variant(_34, 1), 4);
Goto(bb46)
}
bb46 = {
_58 = Move(_20.0.2);
Goto(bb47)
}
bb47 = {
_37 = !Field::<u64>(Variant(_47, 1), 4);
place!(Field::<f64>(Variant(_47, 1), 5)) = (*_12) * (*_12);
_122 = [_102,_88,_83,_83,_107,_107,Field::<isize>(Variant(_34, 1), 2),_107];
(*_71) = [_60.1.2,_39,_39,_60.1.2];
_101 = !_99;
_7 = &_60.2;
match _28 {
0 => bb48,
1 => bb49,
722244862 => bb51,
_ => bb50
}
}
bb48 = {
_58 = Move(_20.0.2);
Goto(bb47)
}
bb49 = {
_1 = 163_u8 + 69_u8;
_8.0 = _10.1.0;
_10.1.0 = _8.0;
_16.0 = [0_usize,4540894032673410415_usize,6_usize];
_8.3 = [_10.1.2,_8.2,_8.2,_8.2];
_1 = !208_u8;
_16.1.0 = _8.3;
_16.1.0 = _10.0;
_14 = _4 as usize;
_16.1.1.0 = [34601_u16,9125_u16,54273_u16,14061_u16,25586_u16];
_10.1.3 = _10.0;
_8.0 = [49605_u16,6632_u16,8388_u16,32807_u16,46520_u16];
_10.1.0 = [2542_u16,50730_u16,56065_u16,44155_u16,51070_u16];
_20.2 = &mut _14;
_15 = 82_i8 as u32;
_21 = Move(_20.2);
_19 = 28826_u16 ^ 36942_u16;
RET = 17403444112719444243_usize as u128;
_4 = !_2;
_18 = _1;
_20.0.1.3 = _8.3;
Goto(bb9)
}
bb50 = {
_49 = _6 < _6;
(*_52) = core::ptr::addr_of_mut!(_31);
_8.2 = (*_17) as i16;
place!(Field::<f64>(Variant(_10.2, 0), 0)) = Field::<f64>(Variant(_47, 1), 5) + Field::<f64>(Variant(_47, 1), 5);
_2 = Field::<bool>(Variant(_47, 1), 0) ^ _49;
(*_71) = [_39,_39,_20.0.1.2,_39];
Goto(bb36)
}
bb51 = {
_76 = [Field::<char>(Variant(_34, 1), 1),_75,Field::<char>(Variant(_34, 1), 1),(*_113),(*_113),(*_113)];
Goto(bb52)
}
bb52 = {
Call(_129 = dump_var(Move(_98), Move(_116), Move(_15), Move(_87)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_129 = dump_var(Move(_5), Move(_44), Move(_4), Move(_79)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_129 = dump_var(Move(_73), Move(_39), Move(_11), Move(_118)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_129 = dump_var(Move(_2), Move(_22), Move(_14), Move(_51)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_129 = dump_var(Move(_29), Move(_53), Move(_37), Move(_31)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_129 = dump_var(Move(_109), Move(_30), Move(_62), Move(_3)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_129 = dump_var(Move(_61), Move(_102), Move(_28), Move(_66)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_129 = dump_var(Move(_57), Move(_77), Move(_76), _130), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: *mut char,mut _2: *mut *mut char,mut _3: u16,mut _4: *mut char,mut _5: i16,mut _6: &'static u8,mut _7: i128,mut _8: u8,mut _9: i64,mut _10: char) -> *mut char {
mir! {
type RET = *mut char;
let _11: (isize, u8, i128, i64);
let _12: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _13: bool;
let _14: [u32; 8];
let _15: bool;
let _16: &'static Adt25;
let _17: u16;
let _18: *const [char; 4];
let _19: isize;
let _20: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _21: i16;
let _22: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _23: [u16; 5];
let _24: isize;
let _25: &'static [i128; 4];
let _26: isize;
let _27: *mut f64;
let _28: *mut f64;
let _29: u16;
let _30: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _31: f64;
let _32: char;
let _33: &'static u8;
let _34: bool;
let _35: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _36: i16;
let _37: *mut u32;
let _38: *mut &'static Adt25;
let _39: bool;
let _40: [i128; 6];
let _41: [i32; 1];
let _42: &'static u8;
let _43: [i32; 4];
let _44: u32;
let _45: &'static mut usize;
let _46: *const f64;
let _47: *const [usize; 3];
let _48: [i16; 4];
let _49: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _50: (isize, u8, i128, i64);
let _51: &'static (i32, (usize, u128, f32), f64);
let _52: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _53: i64;
let _54: isize;
let _55: *const [usize; 3];
let _56: f64;
let _57: f64;
let _58: isize;
let _59: bool;
let _60: isize;
let _61: isize;
let _62: &'static mut u16;
let _63: &'static mut usize;
let _64: isize;
let _65: *const [usize; 3];
let _66: isize;
let _67: &'static [i128; 4];
let _68: u8;
let _69: &'static f64;
let _70: ([u16; 5], *mut char, i16, [i16; 4]);
let _71: [char; 4];
let _72: char;
let _73: f32;
let _74: *mut &'static Adt25;
let _75: [char; 8];
let _76: &'static mut usize;
let _77: [char; 4];
let _78: *const [usize; 3];
let _79: i64;
let _80: *const [usize; 3];
let _81: i128;
let _82: *mut u32;
let _83: isize;
let _84: f64;
let _85: &'static mut usize;
let _86: &'static mut usize;
let _87: i8;
let _88: bool;
let _89: (&'static (i32, (usize, u128, f32), f64), f32);
let _90: Adt84;
let _91: bool;
let _92: *const isize;
let _93: usize;
let _94: ([i32; 1], &'static mut usize, Adt24);
let _95: *mut *mut char;
let _96: (i32, (usize, u128, f32), f64);
let _97: u32;
let _98: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _99: bool;
let _100: [u128; 5];
let _101: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _102: u8;
let _103: f64;
let _104: isize;
let _105: i64;
let _106: [u8; 2];
let _107: isize;
let _108: *mut f64;
let _109: u16;
let _110: [u64; 7];
let _111: i16;
let _112: *const f64;
let _113: char;
let _114: isize;
let _115: isize;
let _116: i64;
let _117: f32;
let _118: u32;
let _119: char;
let _120: [usize; 3];
let _121: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _122: &'static [i128; 4];
let _123: *const [usize; 3];
let _124: u128;
let _125: bool;
let _126: *const u64;
let _127: isize;
let _128: u8;
let _129: [u16; 5];
let _130: *const [usize; 3];
let _131: u128;
let _132: &'static [i128; 4];
let _133: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _134: *mut f64;
let _135: i128;
let _136: u8;
let _137: f64;
let _138: f64;
let _139: &'static (i32, (usize, u128, f32), f64);
let _140: [u8; 2];
let _141: f64;
let _142: i16;
let _143: [i16; 5];
let _144: *const [usize; 3];
let _145: u64;
let _146: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _147: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _148: isize;
let _149: Adt78;
let _150: f32;
let _151: f32;
let _152: [i64; 7];
let _153: *const u64;
let _154: [i128; 4];
let _155: [i32; 1];
let _156: isize;
let _157: [i16; 4];
let _158: &'static Adt25;
let _159: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _160: i8;
let _161: &'static mut Adt42;
let _162: [i16; 5];
let _163: [i32; 4];
let _164: &'static &'static (i32, (usize, u128, f32), f64);
let _165: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _166: *const f64;
let _167: ([i32; 1], &'static mut usize, Adt24);
let _168: isize;
let _169: Adt78;
let _170: Adt68;
let _171: [i64; 7];
let _172: *mut f64;
let _173: u32;
let _174: &'static Adt25;
let _175: f64;
let _176: [i16; 5];
let _177: &'static [char; 4];
let _178: ();
let _179: ();
{
_5 = (-4587_i16);
_6 = &_8;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768206869 => bb9,
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
_14 = [3249028639_u32,358778434_u32,1157846735_u32,1917166833_u32,2064960539_u32,954750898_u32,3286165439_u32,3420125420_u32];
_12.1.3 = [_5,_5,_5,_5];
_11.3 = _9 + _9;
_7 = 50323508109879513210748951218852165083_i128;
_12.1.2 = _5 ^ _5;
_10 = '\u{abed0}';
_12.1.1 = core::ptr::addr_of_mut!(_10);
_5 = _12.1.2 * _12.1.2;
RET = core::ptr::addr_of_mut!(_10);
(*RET) = '\u{f2c0f}';
(*RET) = '\u{e6c08}';
_11.2 = _7 << (*_6);
(*RET) = '\u{2722f}';
_12.1.0 = [_3,_3,_3,_3,_3];
Call(_13 = fn3(Move(_6), Move(_4), Move(_1), Move(_12.1), Move(_2), Move(RET), (*RET)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = core::ptr::addr_of_mut!(_10);
(*RET) = '\u{4557d}';
_10 = '\u{12bfa}';
_12.1.0 = [_3,_3,_3,_3,_3];
_12.1.1 = core::ptr::addr_of_mut!((*RET));
_15 = !_13;
(*RET) = '\u{eb9aa}';
(*RET) = '\u{69833}';
_2 = core::ptr::addr_of_mut!(_1);
_11 = ((-9223372036854775808_isize), _8, _7, _9);
(*RET) = '\u{7da0d}';
(*_2) = Move(RET);
(*_2) = core::ptr::addr_of_mut!(_10);
_12.0 = [_5,_5,_5,_5];
_13 = _15;
(*_1) = '\u{d038a}';
_10 = '\u{51b22}';
(*_2) = Move(_12.1.1);
_8 = _11.1;
_12.1.3 = [_5,_5,_5,_5];
Goto(bb11)
}
bb11 = {
_4 = core::ptr::addr_of_mut!(_10);
(*_4) = '\u{58645}';
_15 = _5 != _5;
_8 = _11.1;
match _11.0 {
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_11.0 = !80_isize;
_17 = _3;
_2 = core::ptr::addr_of_mut!((*_2));
(*_4) = '\u{6283b}';
_4 = core::ptr::addr_of_mut!((*_4));
RET = core::ptr::addr_of_mut!((*_4));
(*_4) = '\u{659d9}';
(*_4) = '\u{9c9bd}';
_12.1.3 = [_5,_5,_5,_5];
_6 = &_11.1;
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_1) = '\u{105fef}';
(*_2) = core::ptr::addr_of_mut!((*_4));
_17 = 320407819705834325031444334225081978455_u128 as u16;
_7 = -_11.2;
_12.0 = [_5,_5,_5,_5];
Goto(bb14)
}
bb14 = {
_3 = _17;
(*_1) = '\u{aaee2}';
_12.1.1 = core::ptr::addr_of_mut!((*_4));
_1 = Move(_4);
(*_2) = core::ptr::addr_of_mut!(_10);
RET = core::ptr::addr_of_mut!((*_1));
_9 = _11.3;
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_2) = Move(_12.1.1);
_13 = (*_6) >= (*_6);
_4 = core::ptr::addr_of_mut!((*RET));
(*_4) = '\u{1bc18}';
(*_4) = '\u{493b5}';
(*RET) = '\u{1c104}';
_20.2.0 = [_5,_5,_5,_5];
_6 = &_8;
(*_2) = core::ptr::addr_of_mut!((*RET));
(*_2) = core::ptr::addr_of_mut!((*RET));
(*_2) = Move(RET);
_19 = _11.0 & _11.0;
(*_2) = core::ptr::addr_of_mut!(_10);
_11.3 = _9 ^ _9;
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_2) = core::ptr::addr_of_mut!((*_1));
Goto(bb15)
}
bb15 = {
_22.0.1 = (_12.1.0, Move((*_2)), _5, _12.0);
_11 = (_19, (*_6), _7, _9);
(*_2) = core::ptr::addr_of_mut!(_10);
_12.1 = Move(_22.0.1);
_22.0.1.1 = core::ptr::addr_of_mut!((*_1));
_12.1.3 = _20.2.0;
_22.0.1.3 = [_5,_5,_5,_12.1.2];
(*_1) = '\u{db6ba}';
_9 = 9829165604859622449_usize as i64;
_22.0.0 = [_12.1.2,_12.1.2,_5,_12.1.2];
_11.1 = (*_6) ^ (*_6);
_22.0.1 = Move(_12.1);
(*_1) = '\u{10fca2}';
(*_2) = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!((*_2));
_12.1.1 = core::ptr::addr_of_mut!((*_1));
_20.2.1.1 = core::ptr::addr_of_mut!((*_1));
_17 = _22.0.1.2 as u16;
_20.2.1.0 = [_17,_17,_17,_17,_17];
_11.0 = _19 - _19;
_11.2 = _17 as i128;
(*_2) = core::ptr::addr_of_mut!((*_1));
_12.0 = [_5,_22.0.1.2,_5,_5];
Goto(bb16)
}
bb16 = {
_9 = (-82_i8) as i64;
_26 = 9496722105136850004_usize as isize;
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_2) = Move(_12.1.1);
_22.0.1 = (_20.2.1.0, Move((*_2)), _5, _22.0.0);
_23 = [_17,_17,_17,_17,_17];
(*_2) = core::ptr::addr_of_mut!(_10);
_22.0.1.2 = -_5;
(*_1) = '\u{e3008}';
Goto(bb17)
}
bb17 = {
_22.0.1.0 = [_17,_17,_17,_17,_17];
(*_1) = '\u{8f4f3}';
(*_2) = core::ptr::addr_of_mut!((*_1));
Goto(bb18)
}
bb18 = {
_23 = _20.2.1.0;
(*_1) = '\u{4500b}';
_15 = _22.0.1.2 != _22.0.1.2;
RET = core::ptr::addr_of_mut!((*_1));
_14 = [3868049263_u32,1104026430_u32,2537826230_u32,3472958703_u32,3747544433_u32,1391622466_u32,786301723_u32,3716959334_u32];
(*_2) = core::ptr::addr_of_mut!((*_1));
_11.2 = _7 | _7;
(*_2) = core::ptr::addr_of_mut!((*RET));
(*_2) = Move(RET);
_28 = core::ptr::addr_of_mut!(_20.0);
_21 = _7 as i16;
(*_2) = core::ptr::addr_of_mut!(_32);
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_28) = (*_6) as f64;
(*_1) = _10;
(*_1) = _10;
(*_28) = 24296253281601345091425396185651368800_u128 as f64;
_40 = [_11.2,_11.2,_7,_11.2,_11.2,_11.2];
_24 = _11.0;
Goto(bb19)
}
bb19 = {
(*_28) = (*_6) as f64;
(*_1) = _10;
(*_1) = _10;
_42 = Move(_6);
_40 = [_7,_11.2,_11.2,_11.2,_11.2,_7];
(*_1) = _10;
(*_2) = core::ptr::addr_of_mut!((*_1));
_3 = _17 ^ _17;
(*_1) = _10;
_11.3 = !_9;
_20.2.1.0 = [_3,_17,_3,_3,_17];
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_28) = _22.0.1.2 as f64;
(*_28) = 114_i8 as f64;
(*_28) = _21 as f64;
Goto(bb20)
}
bb20 = {
_20.2.1.2 = _21;
(*_2) = Move(_20.2.1.1);
_12.0 = [_21,_21,_22.0.1.2,_5];
_19 = -_24;
_33 = &_11.1;
_12.1.0 = [_17,_3,_3,_17,_17];
_43 = [1790045093_i32,(-578083404_i32),1191098503_i32,1282825397_i32];
_37 = core::ptr::addr_of_mut!(_44);
_22.0.1 = (_12.1.0, Move((*_2)), _20.2.1.2, _22.0.0);
_3 = _10 as u16;
(*_37) = 2800308993_u32 ^ 1157351470_u32;
_31 = -(*_28);
_37 = core::ptr::addr_of_mut!((*_37));
(*_2) = Move(_4);
_49.1.1 = core::ptr::addr_of_mut!(_10);
_42 = &_11.1;
_12.1 = Move(_22.0.1);
(*_2) = core::ptr::addr_of_mut!(_10);
Goto(bb21)
}
bb21 = {
_12.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
RET = Move((*_2));
_52.0.1.0 = [_17,_3,_17,_17,_3];
_11.1 = !_8;
_43 = [(-149156448_i32),(-1423895266_i32),(-720653384_i32),1590403912_i32];
_12.1 = (_20.2.1.0, Move(RET), _5, _22.0.0);
(*_2) = core::ptr::addr_of_mut!(_10);
Goto(bb22)
}
bb22 = {
(*_1) = _32;
(*_28) = -Field::<f64>(Variant(_12.2, 0), 0);
_12.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
_50.0 = _19 & _11.0;
(*_2) = Move(_12.1.1);
(*_28) = _5 as f64;
_24 = 11713046984991838233_u64 as isize;
Goto(bb23)
}
bb23 = {
_22.0.1.1 = core::ptr::addr_of_mut!(_32);
_22.0.1 = (_12.1.0, Move((*_2)), _5, _12.0);
(*_28) = _31 + _31;
_52.0 = (_20.2.0, Move(_22.0.1), Move(_12.2));
_12.1.1 = Move(_52.0.1.1);
place!(Field::<f64>(Variant(_52.0.2, 0), 0)) = (*_28) * (*_28);
_33 = &_8;
(*_2) = core::ptr::addr_of_mut!(_10);
(*_28) = Field::<f64>(Variant(_52.0.2, 0), 0);
_52.0.1.1 = core::ptr::addr_of_mut!((*_1));
_32 = (*_1);
_49.0 = [_52.0.1.2,_5,_5,_12.1.2];
(*_28) = -_31;
(*_37) = !Field::<u32>(Variant(_52.0.2, 0), 1);
Goto(bb24)
}
bb24 = {
_3 = _17 - _17;
_43 = [(-1975415264_i32),81465027_i32,(-451701705_i32),376554300_i32];
_22.0.1.3 = [_5,_52.0.1.2,_52.0.1.2,_52.0.1.2];
(*_2) = core::ptr::addr_of_mut!((*_1));
_58 = !_19;
_30 = &_12.1;
_57 = -(*_28);
_40 = [_11.2,_7,_11.2,_11.2,_11.2,_11.2];
_16 = &_52.0.2;
_53 = _9;
(*_37) = Field::<u32>(Variant((*_16), 0), 1) ^ Field::<u32>(Variant((*_16), 0), 1);
_49.1.3 = _52.0.1.3;
_49.1.2 = (*_30).2;
_12 = (_52.0.1.3, Move(_52.0.1), Move(_52.0.2));
_20.2.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
_8 = !_11.1;
_1 = core::ptr::addr_of_mut!((*_1));
_11.0 = _26;
Goto(bb25)
}
bb25 = {
(*_28) = -_31;
_24 = _58 | _19;
(*_1) = _32;
_39 = (*_37) != (*_37);
_22.0.1.2 = _49.1.2 & _12.1.2;
(*_2) = Move(_49.1.1);
_22.0.1.2 = _49.1.2 & _12.1.2;
_50.2 = 7569016205212815808277414391858840830_u128 as i128;
(*_28) = Field::<f64>(Variant(_12.2, 0), 0);
_2 = core::ptr::addr_of_mut!((*_2));
_20.2.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
(*_2) = core::ptr::addr_of_mut!(_32);
_22.0.1.3 = [_12.1.2,_20.2.1.2,_5,_22.0.1.2];
Goto(bb26)
}
bb26 = {
_52.0.1.1 = core::ptr::addr_of_mut!((*_1));
_52.1 = [(*_1),(*_1),(*_1),(*_1),(*_1),(*_1),(*_1),(*_1)];
(*_2) = Move(_12.1.1);
_23 = [_3,_17,_3,_3,_3];
_40 = [_7,_11.2,_50.2,_50.2,_11.2,_7];
RET = core::ptr::addr_of_mut!(_32);
_37 = core::ptr::addr_of_mut!((*_37));
_19 = _50.0;
_6 = &_11.1;
(*RET) = _10;
_39 = (*_37) <= (*_37);
_54 = _7 as isize;
_11.1 = _8;
(*_37) = !Field::<u32>(Variant(_20.2.2, 0), 1);
_46 = core::ptr::addr_of!((*_28));
_24 = _50.0;
_20.2.1.1 = Move((*_2));
(*_2) = core::ptr::addr_of_mut!((*RET));
_1 = core::ptr::addr_of_mut!(_10);
_22.0.1.3 = [_12.1.2,_22.0.1.2,_22.0.1.2,_22.0.1.2];
_20.2.0 = [_12.1.2,_22.0.1.2,_12.1.2,_21];
_24 = 13879751015940722432_u64 as isize;
Call((*_28) = core::intrinsics::transmute(_20.2.0), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_37) = _11.1 as u32;
(*_37) = Field::<u32>(Variant(_12.2, 0), 1);
Goto(bb28)
}
bb28 = {
(*RET) = (*_1);
_48 = [_12.1.2,_12.1.2,_12.1.2,_22.0.1.2];
_11.1 = _8 * _8;
place!(Field::<f64>(Variant(_12.2, 0), 0)) = (*_28);
_14 = [(*_37),(*_37),(*_37),(*_37),(*_37),Field::<u32>(Variant(_20.2.2, 0), 1),(*_37),(*_37)];
_70.1 = core::ptr::addr_of_mut!((*RET));
(*_2) = core::ptr::addr_of_mut!((*RET));
_50.2 = _11.2 ^ _11.2;
_27 = core::ptr::addr_of_mut!((*_28));
(*_1) = _10;
_12.1.3 = _52.0.0;
_52.0.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
(*_37) = Field::<u32>(Variant(_20.2.2, 0), 1);
_58 = 4002331174639921394_u64 as isize;
_14 = [(*_37),(*_37),_44,(*_37),Field::<u32>(Variant(_12.2, 0), 1),(*_37),(*_37),(*_37)];
_66 = (*_1) as isize;
_33 = &_11.1;
_52.1 = [(*_1),(*_1),(*RET),(*_1),(*RET),(*_1),(*_1),(*_1)];
Goto(bb29)
}
bb29 = {
_54 = 14937863040264460978_usize as isize;
(*_28) = _19 as f64;
_52.0.1.0 = _12.1.0;
(*_2) = core::ptr::addr_of_mut!((*RET));
place!(Field::<u32>(Variant(_12.2, 0), 1)) = (*_37) & (*_37);
_20.2.0 = [_12.1.2,_22.0.1.2,_49.1.2,_12.1.2];
(*_28) = _57 + Field::<f64>(Variant(_52.0.2, 0), 0);
(*RET) = _10;
_70.2 = _3 as i16;
(*_28) = Field::<f64>(Variant(_20.2.2, 0), 0) * Field::<f64>(Variant(_52.0.2, 0), 0);
(*_2) = core::ptr::addr_of_mut!((*RET));
place!(Field::<u32>(Variant(_12.2, 0), 1)) = (*_37);
_58 = _54;
_22.0.1 = (_20.2.1.0, Move((*_2)), _5, _12.1.3);
_72 = (*RET);
_22.0.1 = (_20.2.1.0, Move(_20.2.1.1), _49.1.2, _12.1.3);
(*_2) = core::ptr::addr_of_mut!((*RET));
(*_1) = _10;
_2 = core::ptr::addr_of_mut!(_20.2.1.1);
_11.2 = _7 >> (*_37);
(*RET) = _72;
(*_2) = Move(_70.1);
Goto(bb30)
}
bb30 = {
_43 = [1045872636_i32,698504245_i32,(-14994203_i32),(-946939550_i32)];
_54 = _39 as isize;
(*_37) = Field::<u32>(Variant(_52.0.2, 0), 1) >> Field::<u32>(Variant(_20.2.2, 0), 1);
_12.1.1 = core::ptr::addr_of_mut!((*RET));
_68 = (*_33) & _11.1;
_11.0 = _19 & _54;
_40 = [_7,_11.2,_50.2,_50.2,_50.2,_11.2];
_20.2.1 = (_23, Move(RET), _5, _22.0.1.3);
_62 = &mut _17;
(*_37) = Field::<u32>(Variant(_52.0.2, 0), 1) & Field::<u32>(Variant(_20.2.2, 0), 1);
_34 = _39 ^ _13;
_10 = _72;
(*_2) = core::ptr::addr_of_mut!(_32);
_22.0.2 = Adt25::Variant0 { fld0: (*_28),fld1: (*_37) };
_73 = _50.2 as f32;
_22.1 = [_72,_32,_72,_10,_72,_10,_32,_32];
_50 = (_19, (*_33), _11.2, _9);
_22.0.1.0 = [(*_62),(*_62),(*_62),_3,(*_62)];
(*_28) = Field::<f64>(Variant(_12.2, 0), 0);
_52.1 = [_32,_10,_10,_10,_72,_10,_10,_32];
_49.1.1 = Move((*_2));
(*_37) = !Field::<u32>(Variant(_12.2, 0), 1);
_60 = _24 << (*_62);
Goto(bb31)
}
bb31 = {
_72 = _32;
(*_37) = Field::<u32>(Variant(_20.2.2, 0), 1) ^ Field::<u32>(Variant(_52.0.2, 0), 1);
_88 = _15 & _15;
_74 = core::ptr::addr_of_mut!(_16);
_4 = core::ptr::addr_of_mut!(_10);
(*_74) = &_12.2;
(*_2) = core::ptr::addr_of_mut!((*_4));
_89.1 = _73 - _73;
_28 = core::ptr::addr_of_mut!((*_28));
_20.2.1.3 = [_70.2,_22.0.1.2,_70.2,_20.2.1.2];
(*_74) = &_52.0.2;
_89.1 = _73 - _73;
_52.0.1.0 = [(*_62),(*_62),(*_62),(*_62),(*_62)];
_20.2.0 = _20.2.1.3;
(*_2) = core::ptr::addr_of_mut!((*_4));
_1 = core::ptr::addr_of_mut!((*_4));
_12.1.1 = core::ptr::addr_of_mut!((*_1));
_70.0 = _12.1.0;
_58 = _19;
(*_28) = Field::<f64>(Variant((*_16), 0), 0);
(*_4) = _32;
(*_62) = 2016151728_i32 as u16;
Goto(bb32)
}
bb32 = {
(*_62) = _3;
(*_4) = _72;
place!(Field::<u32>(Variant(_52.0.2, 0), 1)) = (*_37);
(*_74) = &_12.2;
_20.2.2 = Move(_22.0.2);
(*_37) = _89.1 as u32;
_10 = _72;
_82 = Move(_37);
_36 = (*_62) as i16;
_37 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant((*_16), 0), 1)));
_56 = 69_i8 as f64;
(*_28) = Field::<f64>(Variant((*_16), 0), 0);
(*_28) = -Field::<f64>(Variant((*_16), 0), 0);
_52.0.1.2 = !_5;
(*_62) = _73 as u16;
(*_74) = &_20.2.2;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = _72;
_70.0 = [(*_62),(*_62),(*_62),(*_62),(*_62)];
_22.0.2 = Move(_20.2.2);
_12.0 = _20.2.1.3;
_52.0.1.1 = core::ptr::addr_of_mut!(_10);
Goto(bb33)
}
bb33 = {
(*_74) = &_22.0.2;
_18 = core::ptr::addr_of!(_71);
_75 = [(*_4),_72,(*_4),(*_4),_72,(*_4),(*_4),(*_4)];
(*_62) = !_3;
_91 = Field::<f64>(Variant((*_16), 0), 0) >= (*_28);
_26 = _11.0;
_72 = (*_4);
_15 = Field::<u32>(Variant((*_16), 0), 1) > Field::<u32>(Variant((*_16), 0), 1);
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
_84 = Field::<f64>(Variant((*_16), 0), 0);
(*_18) = [(*_4),_32,(*_4),(*_4)];
(*_4) = _72;
Goto(bb34)
}
bb34 = {
(*_74) = &_52.0.2;
Goto(bb35)
}
bb35 = {
_70.0 = _12.1.0;
_54 = _11.0 >> (*_62);
_60 = _66;
_33 = &_68;
_42 = &(*_33);
_12.1.2 = _5 & _36;
_91 = Field::<f64>(Variant((*_16), 0), 0) == Field::<f64>(Variant((*_16), 0), 0);
(*_74) = &_12.2;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
_49.1.0 = _20.2.1.0;
_52.1 = [(*_4),(*_4),(*_4),_72,(*_4),(*_4),(*_4),(*_4)];
(*_28) = Field::<f64>(Variant((*_16), 0), 0) * Field::<f64>(Variant(_22.0.2, 0), 0);
(*_4) = _32;
(*_62) = _3 * _3;
(*_62) = !_3;
Goto(bb36)
}
bb36 = {
(*_74) = &_52.0.2;
_52.0.1.1 = Move(_12.1.1);
_9 = _11.3 & _11.3;
_12.0 = [_5,_52.0.1.2,_49.1.2,_21];
_81 = _7 | _11.2;
_32 = (*_4);
(*_28) = Field::<f64>(Variant((*_16), 0), 0) * Field::<f64>(Variant(_52.0.2, 0), 0);
(*_18) = [_72,(*_4),(*_4),(*_4)];
(*_62) = 1453501994_i32 as u16;
(*_62) = !_3;
_41 = [364815417_i32];
_57 = Field::<f64>(Variant((*_16), 0), 0) - (*_28);
_12.1 = (_23, Move((*_2)), _20.2.1.2, _20.2.1.3);
(*_74) = &_12.2;
Goto(bb37)
}
bb37 = {
(*_28) = (-22_i8) as f64;
_69 = &(*_28);
(*_4) = _32;
(*_2) = core::ptr::addr_of_mut!(_32);
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_4) = _72;
(*_2) = core::ptr::addr_of_mut!((*_4));
_20.2.0 = [_70.2,_20.2.1.2,_5,_49.1.2];
_57 = (*_69) * Field::<f64>(Variant((*_16), 0), 0);
_14 = [Field::<u32>(Variant((*_16), 0), 1),(*_37),_44,Field::<u32>(Variant((*_16), 0), 1),Field::<u32>(Variant((*_16), 0), 1),(*_37),Field::<u32>(Variant((*_16), 0), 1),(*_37)];
(*_62) = _3 * _3;
_98.1.1 = Move(_22.0.1);
_88 = _15 ^ _91;
(*_28) = 9650600014342179481_u64 as f64;
Goto(bb38)
}
bb38 = {
(*_28) = _73 as f64;
(*_74) = &_22.0.2;
_49.2 = Move(_22.0.2);
(*_18) = [_32,_72,(*_4),(*_4)];
_70.2 = _50.0 as i16;
_15 = _88 | _34;
_107 = !_26;
_100 = [229658971250642161250320526602115199937_u128,188602038818769588343615205788689552421_u128,131039957949134954863714353483628443067_u128,244096568658470925726862363773840469673_u128,274329232919424267064387545130018034016_u128];
_12 = (_49.0, Move(_49.1), Move(_49.2));
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
_49.1.3 = _12.1.3;
Goto(bb39)
}
bb39 = {
(*_2) = core::ptr::addr_of_mut!((*_4));
_70.0 = _20.2.1.0;
_22.0.2 = Move(_52.0.2);
Goto(bb40)
}
bb40 = {
_22.0.1 = (_12.1.0, Move((*_2)), _98.1.1.2, _48);
(*_4) = _32;
(*_2) = core::ptr::addr_of_mut!((*_4));
_98.1 = (_22.0.0, Move(_20.2.1), Move(_12.2));
_98.0 = [13654272148549509991_usize,5_usize,4_usize];
_61 = _11.0;
Goto(bb41)
}
bb41 = {
_20.2 = Move(_98.1);
(*_74) = &_20.2.2;
_91 = _88;
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_28) = Field::<f64>(Variant((*_16), 0), 0);
_93 = 1745265268391948581_usize;
_79 = !_11.3;
(*_4) = _32;
(*_4) = _32;
_83 = -_61;
_11.3 = _9 * _9;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
_52.2 = &mut _93;
(*_74) = &_22.0.2;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
(*_4) = _72;
(*_28) = -Field::<f64>(Variant((*_16), 0), 0);
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
_70.3 = _48;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
(*_62) = _3;
_100 = [238277642309658519780807693401176245092_u128,100855030127182399644225054476433525781_u128,303369118439360050340201707536179613263_u128,296281010581756423287148319377741025297_u128,163463243257016851068390746607068485460_u128];
_96.1.0 = 17772732878747087756_u64 as usize;
(*_74) = &_20.2.2;
(*_18) = [(*_4),(*_4),(*_4),_72];
Goto(bb42)
}
bb42 = {
_29 = _3 >> _50.2;
(*_2) = Move(_12.1.1);
_111 = _52.0.1.2;
_21 = _52.0.1.2 >> (*_62);
(*_2) = core::ptr::addr_of_mut!((*_4));
_42 = Move(_33);
_33 = Move(_6);
_52.0.1.3 = [_5,_22.0.1.2,_70.2,_111];
Goto(bb43)
}
bb43 = {
(*_2) = core::ptr::addr_of_mut!((*_4));
Call(_109 = core::intrinsics::transmute((*_62)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
_16 = &_22.0.2;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
Goto(bb45)
}
bb45 = {
(*_4) = _32;
_99 = !_91;
_12.0 = [_21,_111,_70.2,_21];
(*_18) = [(*_4),(*_4),_10,(*_4)];
_70.3 = [_36,_70.2,_21,_21];
_105 = _50.3 ^ _11.3;
(*_62) = _29;
(*_4) = _72;
Goto(bb46)
}
bb46 = {
place!(Field::<u32>(Variant(_20.2.2, 0), 1)) = Field::<u32>(Variant((*_16), 0), 1);
_106 = [_50.1,_68];
_96.1.1 = 126013678672960021054605358721799535587_u128 ^ 194860951445139844863572174817844940653_u128;
_64 = _26;
(*_18) = [(*_4),(*_4),(*_4),_72];
Goto(bb47)
}
bb47 = {
(*_28) = -Field::<f64>(Variant(_22.0.2, 0), 0);
_33 = &_50.1;
_98.1 = (_49.0, Move(_20.2.1), Move(_20.2.2));
_94.0 = [1309678987_i32];
(*_62) = _29 * _29;
(*_4) = _72;
_63 = &mut _96.1.0;
_49.1.1 = core::ptr::addr_of_mut!((*_4));
(*_63) = 4565301575998813640_usize;
_78 = core::ptr::addr_of!(_98.0);
place!(Field::<f64>(Variant(_22.0.2, 0), 0)) = (*_28) * _57;
_55 = core::ptr::addr_of!((*_78));
_59 = _91 ^ _34;
(*_2) = core::ptr::addr_of_mut!((*_4));
_12.1.1 = core::ptr::addr_of_mut!((*_4));
(*_74) = &_98.1.2;
_6 = &(*_33);
_100 = [144521340006632771169627803648105994031_u128,119940504364897112992110897054183302840_u128,117936568178658654245734112200194540908_u128,216136916611354898243290392029481370354_u128,102476139121822600446809188788339446507_u128];
_56 = Field::<f64>(Variant((*_16), 0), 0);
(*_78) = [(*_63),(*_63),(*_63)];
Goto(bb48)
}
bb48 = {
_82 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant((*_16), 0), 1)));
(*_63) = _54 as usize;
_80 = core::ptr::addr_of!((*_78));
_103 = Field::<f64>(Variant((*_16), 0), 0) + (*_28);
_70 = (_12.1.0, Move((*_2)), _5, _52.0.1.3);
(*_74) = &_22.0.2;
(*_80) = [(*_63),(*_63),(*_63)];
(*_4) = _72;
_31 = 14132474181529167334_u64 as f64;
(*_28) = Field::<f64>(Variant((*_16), 0), 0) - Field::<f64>(Variant(_22.0.2, 0), 0);
(*_80) = [(*_63),(*_63),(*_63)];
(*_78) = [(*_63),(*_63),(*_63)];
(*_63) = !2_usize;
_98.1.0 = _12.0;
(*_18) = [(*_4),(*_4),_32,(*_4)];
(*_28) = _56 - Field::<f64>(Variant((*_16), 0), 0);
(*_2) = Move(_52.0.1.1);
_20.2.1.2 = 15_i8 as i16;
(*_28) = Field::<f64>(Variant((*_16), 0), 0) - _103;
(*_62) = !_109;
(*_74) = &_98.1.2;
_113 = (*_4);
Goto(bb49)
}
bb49 = {
(*_2) = core::ptr::addr_of_mut!((*_4));
_52.0.2 = Move(_98.1.2);
(*_2) = core::ptr::addr_of_mut!((*_4));
_98.0 = [(*_63),(*_63),(*_63)];
(*_2) = core::ptr::addr_of_mut!((*_4));
_20.2 = Move(_22.0);
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_63) = 7_usize;
_92 = core::ptr::addr_of!(_60);
_60 = _54 & _61;
_66 = _64;
(*_28) = -Field::<f64>(Variant(_20.2.2, 0), 0);
_114 = _50.0;
_22.0.1.2 = !_21;
_97 = Field::<u32>(Variant(_52.0.2, 0), 1) - Field::<u32>(Variant(_52.0.2, 0), 1);
_81 = _7 + _11.2;
_27 = Move(_28);
(*_92) = _15 as isize;
_86 = &mut (*_63);
match (*_86) {
0 => bb29,
1 => bb31,
2 => bb3,
3 => bb48,
4 => bb7,
5 => bb6,
7 => bb51,
_ => bb50
}
}
bb50 = {
_22.0.1 = (_12.1.0, Move((*_2)), _5, _12.0);
_11 = (_19, (*_6), _7, _9);
(*_2) = core::ptr::addr_of_mut!(_10);
_12.1 = Move(_22.0.1);
_22.0.1.1 = core::ptr::addr_of_mut!((*_1));
_12.1.3 = _20.2.0;
_22.0.1.3 = [_5,_5,_5,_12.1.2];
(*_1) = '\u{db6ba}';
_9 = 9829165604859622449_usize as i64;
_22.0.0 = [_12.1.2,_12.1.2,_5,_12.1.2];
_11.1 = (*_6) ^ (*_6);
_22.0.1 = Move(_12.1);
(*_1) = '\u{10fca2}';
(*_2) = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!((*_2));
_12.1.1 = core::ptr::addr_of_mut!((*_1));
_20.2.1.1 = core::ptr::addr_of_mut!((*_1));
_17 = _22.0.1.2 as u16;
_20.2.1.0 = [_17,_17,_17,_17,_17];
_11.0 = _19 - _19;
_11.2 = _17 as i128;
(*_2) = core::ptr::addr_of_mut!((*_1));
_12.0 = [_5,_22.0.1.2,_5,_5];
Goto(bb16)
}
bb51 = {
_71 = [(*_4),(*_4),(*_4),(*_4)];
(*_4) = _32;
(*_4) = _113;
(*_62) = !_29;
_22.0.1.1 = core::ptr::addr_of_mut!(_72);
_31 = _84 + _103;
match (*_86) {
0 => bb52,
1 => bb53,
2 => bb54,
3 => bb55,
4 => bb56,
7 => bb58,
_ => bb57
}
}
bb52 = {
_11.0 = !80_isize;
_17 = _3;
_2 = core::ptr::addr_of_mut!((*_2));
(*_4) = '\u{6283b}';
_4 = core::ptr::addr_of_mut!((*_4));
RET = core::ptr::addr_of_mut!((*_4));
(*_4) = '\u{659d9}';
(*_4) = '\u{9c9bd}';
_12.1.3 = [_5,_5,_5,_5];
_6 = &_11.1;
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_1) = '\u{105fef}';
(*_2) = core::ptr::addr_of_mut!((*_4));
_17 = 320407819705834325031444334225081978455_u128 as u16;
_7 = -_11.2;
_12.0 = [_5,_5,_5,_5];
Goto(bb14)
}
bb53 = {
(*_2) = core::ptr::addr_of_mut!((*_4));
_70.0 = _20.2.1.0;
_22.0.2 = Move(_52.0.2);
Goto(bb40)
}
bb54 = {
Return()
}
bb55 = {
Return()
}
bb56 = {
RET = core::ptr::addr_of_mut!(_10);
(*RET) = '\u{4557d}';
_10 = '\u{12bfa}';
_12.1.0 = [_3,_3,_3,_3,_3];
_12.1.1 = core::ptr::addr_of_mut!((*RET));
_15 = !_13;
(*RET) = '\u{eb9aa}';
(*RET) = '\u{69833}';
_2 = core::ptr::addr_of_mut!(_1);
_11 = ((-9223372036854775808_isize), _8, _7, _9);
(*RET) = '\u{7da0d}';
(*_2) = Move(RET);
(*_2) = core::ptr::addr_of_mut!(_10);
_12.0 = [_5,_5,_5,_5];
_13 = _15;
(*_1) = '\u{d038a}';
_10 = '\u{51b22}';
(*_2) = Move(_12.1.1);
_8 = _11.1;
_12.1.3 = [_5,_5,_5,_5];
Goto(bb11)
}
bb57 = {
_52.0.1.1 = core::ptr::addr_of_mut!((*_1));
_52.1 = [(*_1),(*_1),(*_1),(*_1),(*_1),(*_1),(*_1),(*_1)];
(*_2) = Move(_12.1.1);
_23 = [_3,_17,_3,_3,_3];
_40 = [_7,_11.2,_50.2,_50.2,_11.2,_7];
RET = core::ptr::addr_of_mut!(_32);
_37 = core::ptr::addr_of_mut!((*_37));
_19 = _50.0;
_6 = &_11.1;
(*RET) = _10;
_39 = (*_37) <= (*_37);
_54 = _7 as isize;
_11.1 = _8;
(*_37) = !Field::<u32>(Variant(_20.2.2, 0), 1);
_46 = core::ptr::addr_of!((*_28));
_24 = _50.0;
_20.2.1.1 = Move((*_2));
(*_2) = core::ptr::addr_of_mut!((*RET));
_1 = core::ptr::addr_of_mut!(_10);
_22.0.1.3 = [_12.1.2,_22.0.1.2,_22.0.1.2,_22.0.1.2];
_20.2.0 = [_12.1.2,_22.0.1.2,_12.1.2,_21];
_24 = 13879751015940722432_u64 as isize;
Call((*_28) = core::intrinsics::transmute(_20.2.0), ReturnTo(bb27), UnwindUnreachable())
}
bb58 = {
_87 = _105 as i8;
_133.2.2 = Adt25::Variant0 { fld0: _31,fld1: _97 };
_36 = -_70.2;
(*_2) = core::ptr::addr_of_mut!(_10);
_75 = [(*_4),(*_4),(*_4),(*_4),(*_4),(*_4),(*_4),(*_4)];
_90 = Adt84::Variant0 { fld0: Field::<f64>(Variant(_133.2.2, 0), 0),fld1: Move(_27) };
(*_78) = [(*_86),(*_86),(*_86)];
_133.2.1.1 = core::ptr::addr_of_mut!((*_4));
_104 = (*_92) | (*_92);
(*_86) = 7712603518698522867_usize << (*_62);
(*_92) = -_83;
_22 = (Move(_20.2), _52.1, Move(_86));
_49.1.3 = [_36,_21,_36,_21];
(*_2) = Move(_133.2.1.1);
_91 = !_15;
_20.2.0 = [_21,_21,_21,_21];
_133.2.1.0 = [(*_62),(*_62),_109,(*_62),(*_62)];
_49.1 = (_98.1.1.0, Move((*_2)), _22.0.1.2, _98.1.0);
(*_18) = [(*_4),(*_4),(*_4),_113];
Goto(bb59)
}
bb59 = {
_125 = _59;
(*_62) = !_29;
_12.1.0 = [(*_62),_29,(*_62),(*_62),(*_62)];
(*_78) = [2_usize,3_usize,5_usize];
_133.2.1.2 = _22.0.1.2 & _52.0.1.2;
_77 = (*_18);
_57 = -Field::<f64>(Variant(_133.2.2, 0), 0);
(*_4) = _72;
_140 = _106;
(*_2) = core::ptr::addr_of_mut!((*_4));
_55 = Move(_78);
place!(Field::<f64>(Variant(_90, 0), 0)) = Field::<f64>(Variant(_133.2.2, 0), 0) * Field::<f64>(Variant(_52.0.2, 0), 0);
_114 = -_50.0;
(*_18) = _77;
_12.1 = (_70.0, Move((*_2)), _133.2.1.2, _52.0.1.3);
place!(Field::<u32>(Variant(_52.0.2, 0), 1)) = _31 as u32;
(*_62) = !_29;
_4 = core::ptr::addr_of_mut!((*_4));
Goto(bb60)
}
bb60 = {
_11.1 = 211197655_i32 as u8;
_133.2.2 = Move(_52.0.2);
_84 = -_31;
(*_74) = &_22.0.2;
_31 = -Field::<f64>(Variant((*_16), 0), 0);
_22.2 = Move(_63);
(*_18) = _77;
(*_4) = _113;
(*_2) = Move(_12.1.1);
(*_4) = _32;
_131 = 7975278761251140605_u64 as u128;
_49.1 = (_52.0.1.0, Move((*_2)), _12.1.2, _12.1.3);
_47 = Move(_80);
(*_74) = &_133.2.2;
_63 = Move(_52.2);
_127 = !(*_92);
Goto(bb61)
}
bb61 = {
_52.0 = Move(_22.0);
_64 = !_83;
(*_92) = _66 & _19;
_22.0.2 = Move(_52.0.2);
(*_74) = &_22.0.2;
_64 = _87 as isize;
_38 = core::ptr::addr_of_mut!((*_74));
_59 = !_125;
_98.1.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_16), 0), 0),fld1: Field::<u32>(Variant((*_16), 0), 1) };
place!(Field::<*mut f64>(Variant(_90, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_16), 0), 0)));
_138 = _11.2 as f64;
_4 = core::ptr::addr_of_mut!((*_4));
_22.0.1.3 = _52.0.1.3;
Goto(bb62)
}
bb62 = {
_81 = _11.2 & _11.2;
_151 = _89.1 * _73;
_130 = core::ptr::addr_of!(_120);
_12 = (_70.3, Move(_70), Move(_22.0.2));
(*_74) = &_133.2.2;
(*_92) = -_107;
_157 = _12.1.3;
_156 = !_127;
(*_18) = _77;
(*_130) = [3_usize,5283045078816615563_usize,3_usize];
_47 = core::ptr::addr_of!((*_130));
_49.2 = Move(_98.1.2);
_108 = Move(Field::<*mut f64>(Variant(_90, 0), 1));
(*_2) = core::ptr::addr_of_mut!((*_4));
(*_74) = &_49.2;
Call(_81 = core::intrinsics::bswap(_50.2), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
(*_4) = _32;
_38 = core::ptr::addr_of_mut!((*_74));
(*_47) = [5_usize,13556047826124749449_usize,2206394682302042676_usize];
_20.3 = core::ptr::addr_of!(_145);
_110 = [15080624719646814230_u64,2370836472617611866_u64,3835379028262815769_u64,15697667602770965818_u64,18221110969604947881_u64,8765990287932471810_u64,18176159281238775834_u64];
place!(Field::<u32>(Variant(_12.2, 0), 1)) = _114 as u32;
(*_2) = core::ptr::addr_of_mut!(_72);
_114 = _11.2 as isize;
(*_38) = &_133.2.2;
(*_4) = _32;
_135 = _11.2;
(*_47) = [1_usize,5027196711386059844_usize,16437190101312105803_usize];
(*_130) = [16924156767860703755_usize,4724317524847522756_usize,1_usize];
(*_38) = &_49.2;
_11.3 = _105 * _79;
Goto(bb64)
}
bb64 = {
(*_18) = _77;
_70.3 = _22.0.1.3;
(*_92) = _127 - _104;
_133.3 = core::ptr::addr_of!(_145);
_79 = _11.3 ^ _11.3;
_118 = Field::<u32>(Variant((*_16), 0), 1);
_133.3 = core::ptr::addr_of!(_145);
_100 = [_131,_131,_131,_131,_131];
_162 = [_5,_21,_52.0.1.2,_5,_21];
_159.0.1.1 = Move(_52.0.1.1);
(*_2) = core::ptr::addr_of_mut!(_10);
_158 = Move((*_38));
_98.1.1.1 = core::ptr::addr_of_mut!(_72);
_26 = _73 as isize;
(*_18) = [(*_4),(*_4),(*_4),(*_4)];
(*_18) = [_32,(*_4),(*_4),(*_4)];
Goto(bb65)
}
bb65 = {
(*_38) = &_133.2.2;
RET = Move(_49.1.1);
_22.0.1.0 = [(*_62),(*_62),(*_62),(*_62),(*_62)];
_54 = (*_92);
_168 = -(*_92);
(*_18) = _77;
_49.1 = Move(_12.1);
_137 = Field::<f64>(Variant(_90, 0), 0) * _84;
_159.0.1.2 = !_133.2.1.2;
_5 = -_133.2.1.2;
_174 = &(*_16);
(*_130) = [1_usize,1_usize,11765819745215017580_usize];
_131 = 299333343306540724144870228863766140718_u128 >> (*_33);
(*_74) = &_12.2;
(*_4) = _72;
_133.2 = (_20.2.0, Move(_98.1.1), Move(_49.2));
_78 = core::ptr::addr_of!((*_130));
Goto(bb66)
}
bb66 = {
Call(_178 = dump_var(Move(_87), Move(_14), Move(_66), Move(_7)), ReturnTo(bb67), UnwindUnreachable())
}
bb67 = {
Call(_178 = dump_var(Move(_59), Move(_64), Move(_83), Move(_93)), ReturnTo(bb68), UnwindUnreachable())
}
bb68 = {
Call(_178 = dump_var(Move(_3), Move(_61), Move(_13), Move(_100)), ReturnTo(bb69), UnwindUnreachable())
}
bb69 = {
Call(_178 = dump_var(Move(_40), Move(_77), Move(_88), Move(_157)), ReturnTo(bb70), UnwindUnreachable())
}
bb70 = {
Call(_178 = dump_var(Move(_54), Move(_39), Move(_26), Move(_156)), ReturnTo(bb71), UnwindUnreachable())
}
bb71 = {
Call(_178 = dump_var(Move(_17), Move(_168), Move(_114), Move(_53)), ReturnTo(bb72), UnwindUnreachable())
}
bb72 = {
Call(_178 = dump_var(Move(_8), Move(_5), Move(_36), Move(_21)), ReturnTo(bb73), UnwindUnreachable())
}
bb73 = {
Call(_178 = dump_var(Move(_34), Move(_79), Move(_23), Move(_110)), ReturnTo(bb74), UnwindUnreachable())
}
bb74 = {
Call(_178 = dump_var(Move(_68), Move(_127), Move(_44), _179), ReturnTo(bb75), UnwindUnreachable())
}
bb75 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static u8,mut _2: *mut char,mut _3: *mut char,mut _4: ([u16; 5], *mut char, i16, [i16; 4]),mut _5: *mut *mut char,mut _6: *mut char,mut _7: char) -> bool {
mir! {
type RET = bool;
let _8: &'static [char; 4];
let _9: u128;
let _10: isize;
let _11: f64;
let _12: (isize, u8, i128, i64);
let _13: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _14: *const [usize; 3];
let _15: i64;
let _16: i16;
let _17: u16;
let _18: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _19: isize;
let _20: [u16; 5];
let _21: *const i8;
let _22: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _23: f32;
let _24: isize;
let _25: &'static (i32, (usize, u128, f32), f64);
let _26: f32;
let _27: *mut &'static Adt25;
let _28: *const f64;
let _29: ([u16; 5], *mut char, i16, [i16; 4]);
let _30: *const isize;
let _31: [i64; 7];
let _32: u128;
let _33: isize;
let _34: bool;
let _35: u32;
let _36: &'static f64;
let _37: u16;
let _38: ([u16; 5], *mut char, i16, [i16; 4]);
let _39: bool;
let _40: [i32; 2];
let _41: f64;
let _42: char;
let _43: &'static mut Adt42;
let _44: isize;
let _45: [isize; 8];
let _46: *mut &'static Adt25;
let _47: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _48: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _49: isize;
let _50: bool;
let _51: [i32; 1];
let _52: u16;
let _53: &'static [i128; 4];
let _54: i16;
let _55: [char; 6];
let _56: *const f64;
let _57: f64;
let _58: *mut [i16; 4];
let _59: *const isize;
let _60: u32;
let _61: &'static mut usize;
let _62: &'static mut u16;
let _63: char;
let _64: i16;
let _65: char;
let _66: i64;
let _67: i128;
let _68: isize;
let _69: char;
let _70: u32;
let _71: isize;
let _72: isize;
let _73: &'static mut Adt42;
let _74: *mut u32;
let _75: bool;
let _76: isize;
let _77: ();
let _78: ();
{
_4.1 = core::ptr::addr_of_mut!(_7);
_4.2 = (-10330_i16) << 20863_i16;
_4.0 = [17468_u16,23518_u16,58819_u16,4616_u16,19616_u16];
Call(_4 = fn4(Move(_6), Move(_5), Move(_3), _7, Move(_2)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = '\u{3ad26}';
_4.2 = 16233_i16 << (-78028202305983204146634919182601741421_i128);
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = core::ptr::addr_of_mut!(_7);
_7 = '\u{e00b6}';
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
(*_5) = Move(_3);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
_9 = 42721237927530497940784401028029516522_u128 & 42321859793835945998855795080888313241_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
_4.2 = (-16419_i16) + 17595_i16;
(*_3) = '\u{a1b48}';
_7 = '\u{b35bd}';
(*_5) = Move(_3);
RET = _7 == _7;
_4.0 = [24579_u16,60549_u16,19867_u16,13641_u16,59550_u16];
_3 = core::ptr::addr_of_mut!(_7);
(*_3) = '\u{afcec}';
(*_5) = core::ptr::addr_of_mut!((*_3));
Goto(bb2)
}
bb2 = {
(*_3) = '\u{61ee1}';
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{7995e}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{c6328}';
(*_3) = '\u{100c21}';
_3 = core::ptr::addr_of_mut!((*_3));
_5 = core::ptr::addr_of_mut!(_4.1);
Goto(bb3)
}
bb3 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_13 = &_4;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{aa925}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{5d325}';
_11 = (-28108822844306166170129638935202363338_i128) as f64;
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-3901075474078457919_i64);
Goto(bb4)
}
bb4 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_6 = Move((*_5));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{eecc3}';
_11 = 11425194381384392518398081762114688664_i128 as f64;
(*_3) = '\u{566d}';
_12.0 = 9223372036854775807_isize;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb5 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_13 = &_4;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{aa925}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{5d325}';
_11 = (-28108822844306166170129638935202363338_i128) as f64;
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-3901075474078457919_i64);
Goto(bb4)
}
bb6 = {
(*_3) = '\u{61ee1}';
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{7995e}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{c6328}';
(*_3) = '\u{100c21}';
_3 = core::ptr::addr_of_mut!((*_3));
_5 = core::ptr::addr_of_mut!(_4.1);
Goto(bb3)
}
bb7 = {
_7 = '\u{3ad26}';
_4.2 = 16233_i16 << (-78028202305983204146634919182601741421_i128);
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = core::ptr::addr_of_mut!(_7);
_7 = '\u{e00b6}';
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
(*_5) = Move(_3);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
_9 = 42721237927530497940784401028029516522_u128 & 42321859793835945998855795080888313241_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
_4.2 = (-16419_i16) + 17595_i16;
(*_3) = '\u{a1b48}';
_7 = '\u{b35bd}';
(*_5) = Move(_3);
RET = _7 == _7;
_4.0 = [24579_u16,60549_u16,19867_u16,13641_u16,59550_u16];
_3 = core::ptr::addr_of_mut!(_7);
(*_3) = '\u{afcec}';
(*_5) = core::ptr::addr_of_mut!((*_3));
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_18 = &_4;
(*_3) = '\u{13f9b}';
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{1c053}';
(*_3) = '\u{83f0}';
_16 = -(*_18).2;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{10f482}';
Goto(bb10)
}
bb10 = {
(*_3) = '\u{73084}';
_7 = '\u{71f47}';
_9 = !182126950241624816625927898058082992472_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-1377590484887864539_i64);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{da23e}';
(*_3) = '\u{bb756}';
(*_3) = '\u{6b9d2}';
_9 = 271619999823854656973442966658261969133_u128 | 146758375065607618603682072405231149280_u128;
_4.0 = [7805_u16,16670_u16,7596_u16,37292_u16,20441_u16];
_3 = core::ptr::addr_of_mut!((*_3));
_12 = (9223372036854775807_isize, 153_u8, (-78563182141358482260324332961336235246_i128), (-2375212462813864141_i64));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = Move(_6);
Goto(bb11)
}
bb11 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_24 = _12.0 << (*_13).2;
(*_3) = '\u{105708}';
(*_3) = '\u{3edbe}';
_11 = _12.2 as f64;
_11 = 1836400933_i32 as f64;
_15 = -_12.3;
_17 = 44779_u16 ^ 21479_u16;
(*_3) = '\u{a9685}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{104b80}';
(*_5) = core::ptr::addr_of_mut!(_7);
_2 = Move(_3);
_1 = &_12.1;
(*_5) = core::ptr::addr_of_mut!(_7);
_19 = _12.0 ^ _24;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_13).2,(*_18).2,(*_18).2,(*_18).2];
match _12.1 {
153 => bb12,
_ => bb6
}
}
bb12 = {
_23 = _9 as f32;
_29.1 = Move((*_5));
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_18).2,(*_18).2,(*_18).2,(*_18).2];
_17 = !40488_u16;
_3 = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!((*_3));
_6 = core::ptr::addr_of_mut!((*_3));
_26 = _9 as f32;
(*_6) = '\u{960ea}';
Goto(bb13)
}
bb13 = {
_12.0 = _24 << (*_18).2;
_13 = Move(_18);
(*_6) = '\u{894cc}';
(*_3) = '\u{fb8ef}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_7 = '\u{1b3a}';
(*_3) = '\u{566fd}';
match (*_1) {
0 => bb7,
153 => bb15,
_ => bb14
}
}
bb14 = {
_7 = '\u{3ad26}';
_4.2 = 16233_i16 << (-78028202305983204146634919182601741421_i128);
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = core::ptr::addr_of_mut!(_7);
_7 = '\u{e00b6}';
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
(*_5) = Move(_3);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
_9 = 42721237927530497940784401028029516522_u128 & 42321859793835945998855795080888313241_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
_4.2 = (-16419_i16) + 17595_i16;
(*_3) = '\u{a1b48}';
_7 = '\u{b35bd}';
(*_5) = Move(_3);
RET = _7 == _7;
_4.0 = [24579_u16,60549_u16,19867_u16,13641_u16,59550_u16];
_3 = core::ptr::addr_of_mut!(_7);
(*_3) = '\u{afcec}';
(*_5) = core::ptr::addr_of_mut!((*_3));
Goto(bb2)
}
bb15 = {
match (*_1) {
0 => bb13,
1 => bb11,
2 => bb14,
153 => bb16,
_ => bb6
}
}
bb16 = {
_31 = [_12.3,_12.3,_12.3,_15,_12.3,_12.3,_15];
(*_3) = '\u{2ca76}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_9 = 134624736590047101394345681144381073096_u128 + 239860460928619421554911239377562374911_u128;
_20 = [_17,_17,_17,_17,_17];
_29.3 = [_4.2,_4.2,_4.2,_16];
_4.3 = [_4.2,_16,_16,_16];
_10 = _19 ^ _12.0;
_29.0 = [_17,_17,_17,_17,_17];
_29 = Move(_4);
(*_3) = '\u{20690}';
_16 = _29.2 & _29.2;
(*_5) = core::ptr::addr_of_mut!((*_3));
_30 = core::ptr::addr_of!(_10);
_6 = core::ptr::addr_of_mut!((*_3));
_12 = ((*_30), 242_u8, (-87030753221202171127743378175857564964_i128), _15);
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_5) = Move(_3);
(*_30) = _12.0 | _12.0;
(*_6) = '\u{c8006}';
_35 = _12.1 as u32;
_12.3 = !_15;
Goto(bb17)
}
bb17 = {
(*_6) = '\u{c5075}';
_31 = [_12.3,_15,_15,_12.3,_12.3,_15,_12.3];
_23 = _26 * _26;
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_6) = '\u{a4377}';
_12.0 = _29.2 as isize;
(*_5) = core::ptr::addr_of_mut!((*_6));
_34 = !RET;
_32 = _9 << (*_30);
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_30) = _12.0;
_10 = _24;
(*_5) = core::ptr::addr_of_mut!((*_6));
_12 = ((*_30), 149_u8, (-10346355241878958179297112551379731247_i128), _15);
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_6) = '\u{d9963}';
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_30) = _12.0 + _24;
_3 = core::ptr::addr_of_mut!((*_6));
Goto(bb18)
}
bb18 = {
(*_3) = '\u{16d3c}';
(*_30) = -_12.0;
_26 = _23;
(*_30) = _24 | _19;
(*_6) = '\u{7435f}';
(*_6) = '\u{2b69c}';
_28 = core::ptr::addr_of!(_11);
(*_28) = _32 as f64;
(*_28) = (*_30) as f64;
_38 = (_29.0, Move((*_5)), _16, _29.3);
_7 = '\u{8d16a}';
(*_28) = _23 as f64;
_7 = '\u{65eae}';
(*_28) = 17322875675320150942_u64 as f64;
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_30) = _24 | _24;
(*_6) = '\u{f2f47}';
_4 = (_29.0, Move(_38.1), _16, _38.3);
match _12.1 {
0 => bb7,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
149 => bb25,
_ => bb24
}
}
bb19 = {
_7 = '\u{3ad26}';
_4.2 = 16233_i16 << (-78028202305983204146634919182601741421_i128);
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = core::ptr::addr_of_mut!(_7);
_7 = '\u{e00b6}';
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
(*_5) = Move(_3);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_3 = core::ptr::addr_of_mut!(_7);
_9 = 42721237927530497940784401028029516522_u128 & 42321859793835945998855795080888313241_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
_4.2 = (-16419_i16) + 17595_i16;
(*_3) = '\u{a1b48}';
_7 = '\u{b35bd}';
(*_5) = Move(_3);
RET = _7 == _7;
_4.0 = [24579_u16,60549_u16,19867_u16,13641_u16,59550_u16];
_3 = core::ptr::addr_of_mut!(_7);
(*_3) = '\u{afcec}';
(*_5) = core::ptr::addr_of_mut!((*_3));
Goto(bb2)
}
bb20 = {
_31 = [_12.3,_12.3,_12.3,_15,_12.3,_12.3,_15];
(*_3) = '\u{2ca76}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_9 = 134624736590047101394345681144381073096_u128 + 239860460928619421554911239377562374911_u128;
_20 = [_17,_17,_17,_17,_17];
_29.3 = [_4.2,_4.2,_4.2,_16];
_4.3 = [_4.2,_16,_16,_16];
_10 = _19 ^ _12.0;
_29.0 = [_17,_17,_17,_17,_17];
_29 = Move(_4);
(*_3) = '\u{20690}';
_16 = _29.2 & _29.2;
(*_5) = core::ptr::addr_of_mut!((*_3));
_30 = core::ptr::addr_of!(_10);
_6 = core::ptr::addr_of_mut!((*_3));
_12 = ((*_30), 242_u8, (-87030753221202171127743378175857564964_i128), _15);
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_5) = Move(_3);
(*_30) = _12.0 | _12.0;
(*_6) = '\u{c8006}';
_35 = _12.1 as u32;
_12.3 = !_15;
Goto(bb17)
}
bb21 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_13 = &_4;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{aa925}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{5d325}';
_11 = (-28108822844306166170129638935202363338_i128) as f64;
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-3901075474078457919_i64);
Goto(bb4)
}
bb22 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_6 = Move((*_5));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{eecc3}';
_11 = 11425194381384392518398081762114688664_i128 as f64;
(*_3) = '\u{566d}';
_12.0 = 9223372036854775807_isize;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb23 = {
_12.0 = _24 << (*_18).2;
_13 = Move(_18);
(*_6) = '\u{894cc}';
(*_3) = '\u{fb8ef}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_7 = '\u{1b3a}';
(*_3) = '\u{566fd}';
match (*_1) {
0 => bb7,
153 => bb15,
_ => bb14
}
}
bb24 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_24 = _12.0 << (*_13).2;
(*_3) = '\u{105708}';
(*_3) = '\u{3edbe}';
_11 = _12.2 as f64;
_11 = 1836400933_i32 as f64;
_15 = -_12.3;
_17 = 44779_u16 ^ 21479_u16;
(*_3) = '\u{a9685}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{104b80}';
(*_5) = core::ptr::addr_of_mut!(_7);
_2 = Move(_3);
_1 = &_12.1;
(*_5) = core::ptr::addr_of_mut!(_7);
_19 = _12.0 ^ _24;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_13).2,(*_18).2,(*_18).2,(*_18).2];
match _12.1 {
153 => bb12,
_ => bb6
}
}
bb25 = {
(*_6) = '\u{4901}';
(*_28) = 57_i8 as f64;
(*_6) = '\u{dbbb9}';
(*_28) = _32 as f64;
(*_6) = '\u{c3f09}';
(*_30) = _12.0;
_38 = (_4.0, Move((*_5)), _4.2, _4.3);
(*_5) = core::ptr::addr_of_mut!((*_6));
_19 = !(*_30);
_9 = !_32;
(*_5) = core::ptr::addr_of_mut!((*_6));
_44 = (*_30);
(*_30) = -_44;
(*_30) = !_19;
_13 = &_29;
(*_6) = '\u{d9965}';
_4.1 = core::ptr::addr_of_mut!((*_6));
_4.1 = core::ptr::addr_of_mut!((*_6));
(*_6) = '\u{8c03a}';
(*_5) = Move(_2);
(*_6) = '\u{b4d46}';
(*_5) = Move(_29.1);
_33 = (*_30);
_2 = core::ptr::addr_of_mut!((*_6));
(*_30) = _33;
_38.2 = (*_13).2 | _4.2;
_38.2 = (*_13).2 << _35;
(*_2) = '\u{37d02}';
Goto(bb26)
}
bb26 = {
_4.3 = (*_13).3;
(*_30) = _24 & _33;
_20 = [_17,_17,_17,_17,_17];
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_28) = 14817780389318502414_u64 as f64;
match _12.1 {
0 => bb8,
1 => bb24,
2 => bb16,
3 => bb18,
4 => bb11,
5 => bb27,
6 => bb28,
149 => bb30,
_ => bb29
}
}
bb27 = {
_31 = [_12.3,_12.3,_12.3,_15,_12.3,_12.3,_15];
(*_3) = '\u{2ca76}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_9 = 134624736590047101394345681144381073096_u128 + 239860460928619421554911239377562374911_u128;
_20 = [_17,_17,_17,_17,_17];
_29.3 = [_4.2,_4.2,_4.2,_16];
_4.3 = [_4.2,_16,_16,_16];
_10 = _19 ^ _12.0;
_29.0 = [_17,_17,_17,_17,_17];
_29 = Move(_4);
(*_3) = '\u{20690}';
_16 = _29.2 & _29.2;
(*_5) = core::ptr::addr_of_mut!((*_3));
_30 = core::ptr::addr_of!(_10);
_6 = core::ptr::addr_of_mut!((*_3));
_12 = ((*_30), 242_u8, (-87030753221202171127743378175857564964_i128), _15);
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_5) = Move(_3);
(*_30) = _12.0 | _12.0;
(*_6) = '\u{c8006}';
_35 = _12.1 as u32;
_12.3 = !_15;
Goto(bb17)
}
bb28 = {
match (*_1) {
0 => bb13,
1 => bb11,
2 => bb14,
153 => bb16,
_ => bb6
}
}
bb29 = {
(*_3) = '\u{73084}';
_7 = '\u{71f47}';
_9 = !182126950241624816625927898058082992472_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-1377590484887864539_i64);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{da23e}';
(*_3) = '\u{bb756}';
(*_3) = '\u{6b9d2}';
_9 = 271619999823854656973442966658261969133_u128 | 146758375065607618603682072405231149280_u128;
_4.0 = [7805_u16,16670_u16,7596_u16,37292_u16,20441_u16];
_3 = core::ptr::addr_of_mut!((*_3));
_12 = (9223372036854775807_isize, 153_u8, (-78563182141358482260324332961336235246_i128), (-2375212462813864141_i64));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = Move(_6);
Goto(bb11)
}
bb30 = {
(*_30) = -_12.0;
(*_28) = 1404727897853437523_u64 as f64;
(*_28) = _17 as f64;
_12 = ((*_30), 161_u8, 34996405790139954263655659271696713140_i128, _15);
(*_2) = '\u{f929e}';
_31 = [_12.3,_15,_15,_12.3,_15,_15,_12.3];
(*_6) = '\u{d1599}';
(*_30) = 2_usize as isize;
(*_30) = (-15_i8) as isize;
_50 = !RET;
(*_6) = '\u{78046}';
_7 = '\u{8965c}';
_12.2 = (-1186135223_i32) as i128;
(*_28) = _26 as f64;
(*_6) = '\u{6fb79}';
(*_30) = 16299587026290374217_u64 as isize;
(*_28) = _12.2 as f64;
match _12.1 {
0 => bb16,
1 => bb2,
2 => bb20,
3 => bb31,
4 => bb32,
5 => bb33,
161 => bb35,
_ => bb34
}
}
bb31 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_24 = _12.0 << (*_13).2;
(*_3) = '\u{105708}';
(*_3) = '\u{3edbe}';
_11 = _12.2 as f64;
_11 = 1836400933_i32 as f64;
_15 = -_12.3;
_17 = 44779_u16 ^ 21479_u16;
(*_3) = '\u{a9685}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{104b80}';
(*_5) = core::ptr::addr_of_mut!(_7);
_2 = Move(_3);
_1 = &_12.1;
(*_5) = core::ptr::addr_of_mut!(_7);
_19 = _12.0 ^ _24;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_13).2,(*_18).2,(*_18).2,(*_18).2];
match _12.1 {
153 => bb12,
_ => bb6
}
}
bb32 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_6 = Move((*_5));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{eecc3}';
_11 = 11425194381384392518398081762114688664_i128 as f64;
(*_3) = '\u{566d}';
_12.0 = 9223372036854775807_isize;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb33 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
_24 = _12.0 << (*_13).2;
(*_3) = '\u{105708}';
(*_3) = '\u{3edbe}';
_11 = _12.2 as f64;
_11 = 1836400933_i32 as f64;
_15 = -_12.3;
_17 = 44779_u16 ^ 21479_u16;
(*_3) = '\u{a9685}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{104b80}';
(*_5) = core::ptr::addr_of_mut!(_7);
_2 = Move(_3);
_1 = &_12.1;
(*_5) = core::ptr::addr_of_mut!(_7);
_19 = _12.0 ^ _24;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_13).2,(*_18).2,(*_18).2,(*_18).2];
match _12.1 {
153 => bb12,
_ => bb6
}
}
bb34 = {
_23 = _9 as f32;
_29.1 = Move((*_5));
(*_5) = core::ptr::addr_of_mut!(_7);
_4.3 = [(*_18).2,(*_18).2,(*_18).2,(*_18).2];
_17 = !40488_u16;
_3 = core::ptr::addr_of_mut!(_7);
_4.1 = core::ptr::addr_of_mut!((*_3));
_6 = core::ptr::addr_of_mut!((*_3));
_26 = _9 as f32;
(*_6) = '\u{960ea}';
Goto(bb13)
}
bb35 = {
(*_5) = core::ptr::addr_of_mut!((*_6));
_29 = (_4.0, Move((*_5)), _38.2, _38.3);
_19 = (*_28) as isize;
(*_30) = 13257379923148720965_u64 as isize;
(*_28) = _17 as f64;
_48 = &_38;
(*_5) = core::ptr::addr_of_mut!((*_6));
_38.0 = [_17,_17,_17,_17,_17];
(*_30) = -_12.0;
_41 = -(*_28);
_51 = [2090934494_i32];
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_28) = _9 as f64;
_58 = core::ptr::addr_of_mut!((*_48).3);
_42 = (*_6);
match _12.1 {
0 => bb20,
1 => bb36,
2 => bb37,
161 => bb39,
_ => bb38
}
}
bb36 = {
_12.0 = _24 << (*_18).2;
_13 = Move(_18);
(*_6) = '\u{894cc}';
(*_3) = '\u{fb8ef}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_7 = '\u{1b3a}';
(*_3) = '\u{566fd}';
match (*_1) {
0 => bb7,
153 => bb15,
_ => bb14
}
}
bb37 = {
_4.3 = (*_13).3;
(*_30) = _24 & _33;
_20 = [_17,_17,_17,_17,_17];
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_28) = 14817780389318502414_u64 as f64;
match _12.1 {
0 => bb8,
1 => bb24,
2 => bb16,
3 => bb18,
4 => bb11,
5 => bb27,
6 => bb28,
149 => bb30,
_ => bb29
}
}
bb38 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_13 = &_4;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{aa925}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{5d325}';
_11 = (-28108822844306166170129638935202363338_i128) as f64;
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-3901075474078457919_i64);
Goto(bb4)
}
bb39 = {
(*_6) = _42;
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_28) = _12.1 as f64;
(*_6) = _42;
(*_6) = _42;
(*_30) = (*_28) as isize;
_49 = (*_30);
(*_6) = _42;
_59 = core::ptr::addr_of!(_44);
(*_59) = _35 as isize;
_31 = [_12.3,_12.3,_12.3,_12.3,_15,_12.3,_12.3];
(*_5) = Move(_38.1);
_60 = !_35;
(*_59) = (*_30) - (*_30);
_49 = (*_28) as isize;
_5 = core::ptr::addr_of_mut!((*_5));
_4.1 = core::ptr::addr_of_mut!((*_6));
Goto(bb40)
}
bb40 = {
_54 = (*_48).2 & (*_48).2;
(*_5) = core::ptr::addr_of_mut!((*_6));
(*_28) = _41;
(*_59) = (*_30);
_52 = _17 & _17;
(*_6) = _42;
Goto(bb41)
}
bb41 = {
_13 = &_4;
(*_30) = (*_59);
(*_28) = _41;
_19 = (*_59) - (*_59);
_33 = -(*_59);
_69 = _42;
_66 = _15;
(*_6) = _42;
_16 = (*_48).2 | _29.2;
_64 = (*_13).2;
_45 = [(*_59),(*_30),(*_30),_10,(*_30),_10,(*_59),(*_59)];
_5 = core::ptr::addr_of_mut!((*_5));
_63 = (*_6);
_36 = &(*_28);
_12.3 = _15 | _15;
_10 = -(*_59);
_68 = _17 as isize;
(*_30) = (*_59) ^ (*_59);
_26 = _12.1 as f32;
match _12.1 {
0 => bb42,
1 => bb43,
161 => bb45,
_ => bb44
}
}
bb42 = {
(*_3) = '\u{73084}';
_7 = '\u{71f47}';
_9 = !182126950241624816625927898058082992472_u128;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-1377590484887864539_i64);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{da23e}';
(*_3) = '\u{bb756}';
(*_3) = '\u{6b9d2}';
_9 = 271619999823854656973442966658261969133_u128 | 146758375065607618603682072405231149280_u128;
_4.0 = [7805_u16,16670_u16,7596_u16,37292_u16,20441_u16];
_3 = core::ptr::addr_of_mut!((*_3));
_12 = (9223372036854775807_isize, 153_u8, (-78563182141358482260324332961336235246_i128), (-2375212462813864141_i64));
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = Move(_6);
Goto(bb11)
}
bb43 = {
_12.0 = _24 << (*_18).2;
_13 = Move(_18);
(*_6) = '\u{894cc}';
(*_3) = '\u{fb8ef}';
(*_5) = core::ptr::addr_of_mut!((*_3));
_7 = '\u{1b3a}';
(*_3) = '\u{566fd}';
match (*_1) {
0 => bb7,
153 => bb15,
_ => bb14
}
}
bb44 = {
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_3));
_13 = &_4;
(*_5) = core::ptr::addr_of_mut!(_7);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{aa925}';
_6 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{5d325}';
_11 = (-28108822844306166170129638935202363338_i128) as f64;
(*_5) = core::ptr::addr_of_mut!((*_3));
_12.3 = -(-3901075474078457919_i64);
Goto(bb4)
}
bb45 = {
_31 = [_12.3,_15,_12.3,_12.3,_12.3,_12.3,_66];
_59 = core::ptr::addr_of!((*_30));
_71 = _44 ^ (*_30);
(*_28) = _41 - _41;
_50 = !RET;
(*_6) = _69;
Goto(bb46)
}
bb46 = {
(*_59) = (*_28) as isize;
(*_6) = _42;
_59 = core::ptr::addr_of!(_49);
(*_6) = _63;
Goto(bb47)
}
bb47 = {
Goto(bb48)
}
bb48 = {
_13 = &_29;
(*_30) = 1_usize as isize;
_29.0 = _38.0;
_54 = (*_48).2;
_23 = -_26;
_4.2 = _54 << (*_59);
_31 = [_12.3,_66,_12.3,_15,_12.3,_66,_66];
(*_28) = _52 as f64;
Call(_37 = core::intrinsics::transmute((*_48).2), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_39 = RET | RET;
_4.2 = -_54;
(*_30) = (*_59);
_23 = _26 * _26;
_57 = -(*_28);
(*_59) = (*_30);
_70 = _35 + _35;
Goto(bb50)
}
bb50 = {
Call(_77 = dump_var(Move(_71), Move(_24), Move(_7), Move(_42)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_77 = dump_var(Move(_19), Move(_49), Move(_45), Move(_16)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_77 = dump_var(Move(_63), Move(_64), Move(_70), Move(_34)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_77 = dump_var(Move(_44), Move(_20), Move(_17), Move(_31)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: *mut char,mut _2: *mut *mut char,mut _3: *mut char,mut _4: char,mut _5: *mut char) -> ([u16; 5], *mut char, i16, [i16; 4]) {
mir! {
type RET = ([u16; 5], *mut char, i16, [i16; 4]);
let _6: i64;
let _7: u8;
let _8: [i16; 5];
let _9: Adt78;
let _10: Adt68;
let _11: bool;
let _12: f32;
let _13: [u64; 7];
let _14: &'static u8;
let _15: f64;
let _16: ();
let _17: ();
{
_5 = core::ptr::addr_of_mut!(_4);
(*_5) = '\u{b52f3}';
(*_5) = '\u{5d913}';
_4 = '\u{45590}';
(*_5) = '\u{62c45}';
_2 = core::ptr::addr_of_mut!(_1);
(*_5) = '\u{70006}';
_5 = core::ptr::addr_of_mut!((*_5));
_8 = [17226_i16,(-17668_i16),(-30141_i16),(-31909_i16),7493_i16];
(*_2) = core::ptr::addr_of_mut!((*_5));
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_2) = Move(_5);
(*_2) = core::ptr::addr_of_mut!(_4);
(*_1) = '\u{34144}';
(*_1) = '\u{200de}';
(*_2) = core::ptr::addr_of_mut!((*_1));
Call(_7 = fn5(Move((*_2))), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_2) = core::ptr::addr_of_mut!(_4);
(*_1) = '\u{26863}';
(*_1) = '\u{bd40d}';
_10.fld0.0 = [3537_i16,(-31630_i16),15893_i16,(-8581_i16)];
_10.fld0.1.0 = [13637_u16,43679_u16,51938_u16,41084_u16,62809_u16];
_6 = (-1323937914944277502_i64);
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{e81ae}';
(*_2) = core::ptr::addr_of_mut!((*_1));
_3 = core::ptr::addr_of_mut!((*_1));
(*_2) = Move(_3);
_11 = false;
_7 = 123_u8 * 246_u8;
(*_2) = core::ptr::addr_of_mut!(_4);
(*_1) = '\u{36add}';
RET.3 = [2109_i16,14010_i16,12488_i16,(-24466_i16)];
_5 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{4f74c}';
(*_2) = core::ptr::addr_of_mut!((*_1));
(*_2) = core::ptr::addr_of_mut!((*_1));
RET = (_10.fld0.1.0, Move(_5), 30701_i16, _10.fld0.0);
(*_1) = '\u{67728}';
(*_1) = '\u{93e0a}';
(*_1) = '\u{f9d}';
(*_1) = '\u{8816b}';
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(Move(_6), Move(_8), _17, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut char) -> u8 {
mir! {
type RET = u8;
let _2: *mut [i16; 4];
let _3: [u64; 1];
let _4: &'static f64;
let _5: *const u64;
let _6: f32;
let _7: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _8: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _9: *mut u32;
let _10: f32;
let _11: &'static mut usize;
let _12: &'static mut u16;
let _13: *const [usize; 3];
let _14: [usize; 3];
let _15: i32;
let _16: isize;
let _17: isize;
let _18: &'static Adt25;
let _19: i8;
let _20: f64;
let _21: char;
let _22: &'static u8;
let _23: &'static u8;
let _24: &'static u8;
let _25: isize;
let _26: &'static &'static (i32, (usize, u128, f32), f64);
let _27: (isize, u8, i128, i64);
let _28: isize;
let _29: &'static mut usize;
let _30: u8;
let _31: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _32: &'static u8;
let _33: bool;
let _34: i64;
let _35: f32;
let _36: i32;
let _37: &'static [char; 4];
let _38: i16;
let _39: bool;
let _40: i8;
let _41: i8;
let _42: *const i8;
let _43: [u64; 7];
let _44: f32;
let _45: *mut &'static Adt25;
let _46: isize;
let _47: *const i8;
let _48: bool;
let _49: char;
let _50: f32;
let _51: Adt25;
let _52: &'static mut Adt42;
let _53: i16;
let _54: char;
let _55: isize;
let _56: bool;
let _57: bool;
let _58: u8;
let _59: Adt25;
let _60: *const f64;
let _61: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _62: u16;
let _63: u32;
let _64: [char; 8];
let _65: u32;
let _66: f32;
let _67: f64;
let _68: u32;
let _69: u32;
let _70: f32;
let _71: ();
let _72: ();
{
RET = 163_u8 & 61_u8;
RET = 219_u8 * 242_u8;
RET = 205_u8 | 14_u8;
RET = '\u{b85b}' as u8;
RET = 203_u8 >> 20079491561942847719880284419225982771_i128;
RET = 223_u8 & 68_u8;
RET = 224_u8 * 217_u8;
RET = 187_u8 * 35_u8;
RET = 51759281664116293353751502794124864598_i128 as u8;
RET = 55_u8 ^ 73_u8;
_3 = [4796062789560674354_u64];
_3 = [16652755457811989153_u64];
_3 = [13461782346206309207_u64];
RET = (-64_i8) as u8;
RET = !109_u8;
RET = 5873274729294858015_i64 as u8;
RET = !36_u8;
_3 = [15175358658013718314_u64];
_3 = [9673849055045518515_u64];
Goto(bb1)
}
bb1 = {
RET = !198_u8;
RET = 215_u8 | 224_u8;
RET = 53_u8;
_3 = [6332011529521739044_u64];
_3 = [13861165776251896148_u64];
RET = 81_u8 | 42_u8;
RET = 1339145090_u32 as u8;
RET = 123_u8 << 2514832513_u32;
RET = 229_u8 + 57_u8;
_3 = [1700802861968388401_u64];
_3 = [1593360114214916912_u64];
RET = !153_u8;
_3 = [591365573513511627_u64];
_6 = RET as f32;
_7.1.2 = (-21384_i16) | (-11632_i16);
_7.1.3 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_2 = core::ptr::addr_of_mut!(_7.0);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.0 = [5429_u16,914_u16,31031_u16,7719_u16,41371_u16];
Goto(bb2)
}
bb2 = {
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_3 = [15708347575814504860_u64];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = _7.1.3;
_7.1.3 = (*_2);
_7.1.3 = (*_2);
(*_2) = _7.1.3;
_10 = -_6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_6 = _10 * _10;
_6 = _10 * _10;
_1 = Move(_7.1.1);
Goto(bb3)
}
bb3 = {
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.0 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.2 = 15143_i16;
(*_2) = _7.1.3;
Goto(bb4)
}
bb4 = {
_6 = _10 + _10;
_7.1.3 = (*_2);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
RET = 74_u8 >> _7.1.2;
_6 = -_10;
_10 = _6 - _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
_10 = _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_2 = core::ptr::addr_of_mut!((*_2));
_1 = Move(_7.1.1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
Call(_9 = fn6((*_2), (*_2), Move(_2), (*_2), (*_2), (*_2), Move(_1), (*_2), (*_2), (*_2), (*_2), (*_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = _10;
_16 = !(-9223372036854775808_isize);
_13 = core::ptr::addr_of!(_14);
(*_13) = [17561425076022480620_usize,0_usize,4175514522492093815_usize];
_13 = core::ptr::addr_of!((*_13));
(*_13) = [4_usize,3_usize,10293231080265187496_usize];
(*_13) = [0_usize,1_usize,1_usize];
match _7.1.2 {
15143 => bb7,
_ => bb6
}
}
bb6 = {
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.0 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.2 = 15143_i16;
(*_2) = _7.1.3;
Goto(bb4)
}
bb7 = {
_19 = 15605979927937170986_u64 as i8;
(*_13) = [6_usize,6422134536975842499_usize,2_usize];
(*_13) = [3719304976281715101_usize,2_usize,2_usize];
(*_13) = [9754791463703612492_usize,1981135645286896756_usize,13632346625841917684_usize];
(*_13) = [6_usize,2_usize,6123543866851276854_usize];
(*_13) = [16027515513480852922_usize,6040401291462433807_usize,18025944993396093867_usize];
(*_13) = [17669727797872409207_usize,16793990623533631569_usize,2_usize];
(*_13) = [3_usize,13790369281041463929_usize,3_usize];
_14 = [7_usize,14006696631068706688_usize,2_usize];
(*_13) = [15741337361199016437_usize,18267813313450878762_usize,10246712115265432751_usize];
_15 = 1620211998_i32;
(*_13) = [9236115909233197839_usize,2_usize,1_usize];
_19 = (-57_i8) << RET;
(*_13) = [6133583276273247571_usize,6_usize,2_usize];
_1 = core::ptr::addr_of_mut!(_21);
(*_1) = '\u{25c2}';
(*_13) = [2_usize,731400636654423420_usize,7932040666823664970_usize];
(*_13) = [4_usize,3_usize,1_usize];
(*_13) = [3_usize,17524573888072418415_usize,3034422039682867011_usize];
(*_1) = '\u{f4507}';
(*_1) = '\u{764ac}';
_6 = _10 - _10;
(*_13) = [3272077946491160918_usize,16495564519840818152_usize,2_usize];
(*_1) = '\u{e451d}';
match _7.1.2 {
0 => bb1,
15143 => bb8,
_ => bb3
}
}
bb8 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb9 = {
RET = !198_u8;
RET = 215_u8 | 224_u8;
RET = 53_u8;
_3 = [6332011529521739044_u64];
_3 = [13861165776251896148_u64];
RET = 81_u8 | 42_u8;
RET = 1339145090_u32 as u8;
RET = 123_u8 << 2514832513_u32;
RET = 229_u8 + 57_u8;
_3 = [1700802861968388401_u64];
_3 = [1593360114214916912_u64];
RET = !153_u8;
_3 = [591365573513511627_u64];
_6 = RET as f32;
_7.1.2 = (-21384_i16) | (-11632_i16);
_7.1.3 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_2 = core::ptr::addr_of_mut!(_7.0);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.0 = [5429_u16,914_u16,31031_u16,7719_u16,41371_u16];
Goto(bb2)
}
bb10 = {
(*_1) = '\u{d9c5f}';
_23 = &RET;
(*_1) = '\u{b10b8}';
(*_13) = [1_usize,5_usize,2323421530294814911_usize];
_25 = !_16;
(*_1) = '\u{24807}';
Call(_10 = core::intrinsics::transmute((*_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19 = (*_1) as i8;
_21 = '\u{6640e}';
Goto(bb12)
}
bb12 = {
_27.2 = (-70924966076742258978093349209453186337_i128) - (-132045140950821792670899716566727218406_i128);
(*_1) = '\u{98b20}';
(*_13) = [7_usize,0_usize,3_usize];
_27.0 = _17;
(*_1) = '\u{464b0}';
_27 = (_17, (*_23), (-19374937631514608986101455025157847380_i128), (-8459494508343893705_i64));
_27.2 = (-146988257961291902356677528078742171269_i128) - 70882878841403689013966970454077981377_i128;
(*_1) = '\u{65bdc}';
(*_1) = '\u{4320e}';
(*_13) = [7_usize,10452795848678309220_usize,4_usize];
(*_13) = [12711198795369370411_usize,4_usize,6085087652531581263_usize];
(*_13) = [7_usize,14375050943909234433_usize,5_usize];
(*_13) = [4_usize,6_usize,1_usize];
_17 = !_27.0;
_30 = (*_23) & (*_23);
(*_13) = [6962546827361715208_usize,17842466944074968152_usize,9242830190750216212_usize];
(*_1) = '\u{d7225}';
_10 = _6 + _6;
(*_1) = '\u{5922}';
Call(_27.2 = core::intrinsics::bswap((-52054528398143887859706367083412543771_i128)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_27.2 = 10224578598796472805142158597243917275_i128;
(*_1) = '\u{3ede9}';
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!(_7.1.3);
(*_13) = [18151273042943057063_usize,13415503235237757125_usize,14655215383304052321_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.0;
match _27.3 {
0 => bb7,
1 => bb4,
340282366920938463454915112923424317751 => bb15,
_ => bb14
}
}
bb14 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb15 = {
(*_13) = [7_usize,2_usize,10840537768423918469_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_1) = '\u{16929}';
(*_1) = '\u{2c950}';
match _27.3 {
0 => bb9,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
5 => bb20,
340282366920938463454915112923424317751 => bb22,
_ => bb21
}
}
bb16 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb17 = {
_27.2 = 10224578598796472805142158597243917275_i128;
(*_1) = '\u{3ede9}';
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!(_7.1.3);
(*_13) = [18151273042943057063_usize,13415503235237757125_usize,14655215383304052321_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.0;
match _27.3 {
0 => bb7,
1 => bb4,
340282366920938463454915112923424317751 => bb15,
_ => bb14
}
}
bb18 = {
_27.2 = (-70924966076742258978093349209453186337_i128) - (-132045140950821792670899716566727218406_i128);
(*_1) = '\u{98b20}';
(*_13) = [7_usize,0_usize,3_usize];
_27.0 = _17;
(*_1) = '\u{464b0}';
_27 = (_17, (*_23), (-19374937631514608986101455025157847380_i128), (-8459494508343893705_i64));
_27.2 = (-146988257961291902356677528078742171269_i128) - 70882878841403689013966970454077981377_i128;
(*_1) = '\u{65bdc}';
(*_1) = '\u{4320e}';
(*_13) = [7_usize,10452795848678309220_usize,4_usize];
(*_13) = [12711198795369370411_usize,4_usize,6085087652531581263_usize];
(*_13) = [7_usize,14375050943909234433_usize,5_usize];
(*_13) = [4_usize,6_usize,1_usize];
_17 = !_27.0;
_30 = (*_23) & (*_23);
(*_13) = [6962546827361715208_usize,17842466944074968152_usize,9242830190750216212_usize];
(*_1) = '\u{d7225}';
_10 = _6 + _6;
(*_1) = '\u{5922}';
Call(_27.2 = core::intrinsics::bswap((-52054528398143887859706367083412543771_i128)), ReturnTo(bb13), UnwindUnreachable())
}
bb19 = {
_6 = _10 + _10;
_7.1.3 = (*_2);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
RET = 74_u8 >> _7.1.2;
_6 = -_10;
_10 = _6 - _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
_10 = _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_2 = core::ptr::addr_of_mut!((*_2));
_1 = Move(_7.1.1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
Call(_9 = fn6((*_2), (*_2), Move(_2), (*_2), (*_2), (*_2), Move(_1), (*_2), (*_2), (*_2), (*_2), (*_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb20 = {
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_3 = [15708347575814504860_u64];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = _7.1.3;
_7.1.3 = (*_2);
_7.1.3 = (*_2);
(*_2) = _7.1.3;
_10 = -_6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_6 = _10 * _10;
_6 = _10 * _10;
_1 = Move(_7.1.1);
Goto(bb3)
}
bb21 = {
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.0 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.2 = 15143_i16;
(*_2) = _7.1.3;
Goto(bb4)
}
bb22 = {
(*_13) = [3978240733069497044_usize,10826293370713260969_usize,3472280982075838881_usize];
_15 = (-680699521_i32);
_33 = false ^ false;
_25 = _17 & _27.0;
_28 = _27.0 & _25;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_13) = [54947689575263814_usize,0_usize,8853650830603271237_usize];
(*_13) = [11984500229398524388_usize,2_usize,0_usize];
(*_13) = [6363570777503934079_usize,3_usize,3136993907177365084_usize];
(*_2) = _7.0;
_34 = _27.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_1) = '\u{8a70a}';
(*_1) = '\u{58dfe}';
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_13) = [12364536750151318012_usize,16700958658832232676_usize,15833585491490987128_usize];
_30 = 4655101181973560043_u64 as u8;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_31 = &_7.1;
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
match _34 {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
340282366920938463454915112923424317751 => bb29,
_ => bb28
}
}
bb23 = {
_27.2 = 10224578598796472805142158597243917275_i128;
(*_1) = '\u{3ede9}';
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!(_7.1.3);
(*_13) = [18151273042943057063_usize,13415503235237757125_usize,14655215383304052321_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.0;
match _27.3 {
0 => bb7,
1 => bb4,
340282366920938463454915112923424317751 => bb15,
_ => bb14
}
}
bb24 = {
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.0 = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.2 = 15143_i16;
(*_2) = _7.1.3;
Goto(bb4)
}
bb25 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb26 = {
_19 = (*_1) as i8;
_21 = '\u{6640e}';
Goto(bb12)
}
bb27 = {
_19 = 15605979927937170986_u64 as i8;
(*_13) = [6_usize,6422134536975842499_usize,2_usize];
(*_13) = [3719304976281715101_usize,2_usize,2_usize];
(*_13) = [9754791463703612492_usize,1981135645286896756_usize,13632346625841917684_usize];
(*_13) = [6_usize,2_usize,6123543866851276854_usize];
(*_13) = [16027515513480852922_usize,6040401291462433807_usize,18025944993396093867_usize];
(*_13) = [17669727797872409207_usize,16793990623533631569_usize,2_usize];
(*_13) = [3_usize,13790369281041463929_usize,3_usize];
_14 = [7_usize,14006696631068706688_usize,2_usize];
(*_13) = [15741337361199016437_usize,18267813313450878762_usize,10246712115265432751_usize];
_15 = 1620211998_i32;
(*_13) = [9236115909233197839_usize,2_usize,1_usize];
_19 = (-57_i8) << RET;
(*_13) = [6133583276273247571_usize,6_usize,2_usize];
_1 = core::ptr::addr_of_mut!(_21);
(*_1) = '\u{25c2}';
(*_13) = [2_usize,731400636654423420_usize,7932040666823664970_usize];
(*_13) = [4_usize,3_usize,1_usize];
(*_13) = [3_usize,17524573888072418415_usize,3034422039682867011_usize];
(*_1) = '\u{f4507}';
(*_1) = '\u{764ac}';
_6 = _10 - _10;
(*_13) = [3272077946491160918_usize,16495564519840818152_usize,2_usize];
(*_1) = '\u{e451d}';
match _7.1.2 {
0 => bb1,
15143 => bb8,
_ => bb3
}
}
bb28 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb29 = {
_2 = core::ptr::addr_of_mut!((*_2));
_27.0 = _28;
(*_1) = '\u{fec54}';
(*_13) = [7_usize,2_usize,6_usize];
(*_13) = [3_usize,11033055595370082519_usize,5_usize];
(*_13) = [6_usize,11374227522307160957_usize,13714440907885176965_usize];
(*_13) = [1_usize,6_usize,11140077451370238725_usize];
(*_13) = [0_usize,7_usize,17171599995858482934_usize];
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
(*_2) = _7.0;
_28 = _17 * _27.0;
_22 = &(*_23);
_22 = &_30;
(*_13) = [4_usize,6_usize,14618177241397420313_usize];
Goto(bb30)
}
bb30 = {
_14 = [1873949313341050491_usize,6895909811310577435_usize,4527496572902448513_usize];
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_36 = _15 - _15;
_35 = _10 * _6;
(*_13) = [16630651953161981661_usize,7_usize,1_usize];
_35 = _6 + _6;
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_33 = true & false;
(*_13) = [16906375190613419342_usize,7_usize,290767275117369073_usize];
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
(*_1) = '\u{c3cf9}';
(*_13) = [1_usize,7002806683290400602_usize,3_usize];
_14 = [0_usize,7_usize,60434219966119903_usize];
_27 = (_28, (*_22), (-1312938083652168907394293881715258087_i128), _34);
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_33 = _27.3 >= _34;
(*_1) = '\u{1e9c8}';
_15 = !_36;
_38 = !(*_31).2;
_42 = core::ptr::addr_of!(_41);
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
(*_42) = _19;
_30 = (*_23);
(*_2) = _7.0;
(*_1) = '\u{505eb}';
match _27.2 {
0 => bb7,
338969428837286294555980313550052953369 => bb32,
_ => bb31
}
}
bb31 = {
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_3 = [15708347575814504860_u64];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = _7.1.3;
_7.1.3 = (*_2);
_7.1.3 = (*_2);
(*_2) = _7.1.3;
_10 = -_6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_6 = _10 * _10;
_6 = _10 * _10;
_1 = Move(_7.1.1);
Goto(bb3)
}
bb32 = {
(*_1) = '\u{656e6}';
(*_1) = '\u{af8a1}';
(*_2) = [(*_31).2,(*_31).2,_7.1.2,(*_31).2];
(*_13) = [7_usize,4072411427652522715_usize,5690623959611536069_usize];
_25 = !_17;
_32 = &(*_23);
(*_13) = [7683450846538934951_usize,4_usize,2_usize];
(*_2) = _7.0;
_22 = &(*_32);
(*_13) = [1_usize,13216023039309558014_usize,1540479152744101315_usize];
_43 = [14664065620889883914_u64,10002921637797923942_u64,6533102631588998663_u64,762801152380768839_u64,109153742619059328_u64,12366474276336203338_u64,12751960295660061672_u64];
(*_13) = [1385499382249845737_usize,9001375296037251859_usize,4_usize];
_39 = _34 == _27.3;
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
match _7.1.2 {
0 => bb6,
15143 => bb33,
_ => bb14
}
}
bb33 = {
(*_13) = [1_usize,0_usize,4703697048673624620_usize];
(*_42) = _19 + _19;
_32 = &_30;
(*_1) = '\u{2366f}';
(*_13) = [7657159275486513789_usize,8441312491069160401_usize,5414852245420635521_usize];
match _27.2 {
0 => bb14,
1 => bb16,
2 => bb26,
3 => bb7,
4 => bb24,
5 => bb34,
338969428837286294555980313550052953369 => bb36,
_ => bb35
}
}
bb34 = {
_27.2 = 10224578598796472805142158597243917275_i128;
(*_1) = '\u{3ede9}';
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!(_7.1.3);
(*_13) = [18151273042943057063_usize,13415503235237757125_usize,14655215383304052321_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.0;
match _27.3 {
0 => bb7,
1 => bb4,
340282366920938463454915112923424317751 => bb15,
_ => bb14
}
}
bb35 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb36 = {
(*_42) = _19 * _19;
(*_2) = [(*_31).2,_38,(*_31).2,(*_31).2];
(*_1) = '\u{f79}';
_10 = -_35;
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
(*_13) = [3666736400513114150_usize,2_usize,7_usize];
(*_1) = '\u{cf2ee}';
(*_13) = [13584000920008556956_usize,7_usize,16658965570420816132_usize];
(*_13) = [6177295623468392472_usize,0_usize,13981367290760886409_usize];
match _27.3 {
0 => bb37,
1 => bb38,
2 => bb39,
3 => bb40,
4 => bb41,
340282366920938463454915112923424317751 => bb43,
_ => bb42
}
}
bb37 = {
_6 = _10 + _10;
_7.1.3 = (*_2);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
RET = 74_u8 >> _7.1.2;
_6 = -_10;
_10 = _6 - _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
_10 = _6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_2 = core::ptr::addr_of_mut!((*_2));
_1 = Move(_7.1.1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
Call(_9 = fn6((*_2), (*_2), Move(_2), (*_2), (*_2), (*_2), Move(_1), (*_2), (*_2), (*_2), (*_2), (*_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb38 = {
_27.2 = 10224578598796472805142158597243917275_i128;
(*_1) = '\u{3ede9}';
_7.1.1 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!(_7.1.3);
(*_13) = [18151273042943057063_usize,13415503235237757125_usize,14655215383304052321_usize];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.0;
match _27.3 {
0 => bb7,
1 => bb4,
340282366920938463454915112923424317751 => bb15,
_ => bb14
}
}
bb39 = {
_27.2 = (-70924966076742258978093349209453186337_i128) - (-132045140950821792670899716566727218406_i128);
(*_1) = '\u{98b20}';
(*_13) = [7_usize,0_usize,3_usize];
_27.0 = _17;
(*_1) = '\u{464b0}';
_27 = (_17, (*_23), (-19374937631514608986101455025157847380_i128), (-8459494508343893705_i64));
_27.2 = (-146988257961291902356677528078742171269_i128) - 70882878841403689013966970454077981377_i128;
(*_1) = '\u{65bdc}';
(*_1) = '\u{4320e}';
(*_13) = [7_usize,10452795848678309220_usize,4_usize];
(*_13) = [12711198795369370411_usize,4_usize,6085087652531581263_usize];
(*_13) = [7_usize,14375050943909234433_usize,5_usize];
(*_13) = [4_usize,6_usize,1_usize];
_17 = !_27.0;
_30 = (*_23) & (*_23);
(*_13) = [6962546827361715208_usize,17842466944074968152_usize,9242830190750216212_usize];
(*_1) = '\u{d7225}';
_10 = _6 + _6;
(*_1) = '\u{5922}';
Call(_27.2 = core::intrinsics::bswap((-52054528398143887859706367083412543771_i128)), ReturnTo(bb13), UnwindUnreachable())
}
bb40 = {
(*_1) = '\u{656e6}';
(*_1) = '\u{af8a1}';
(*_2) = [(*_31).2,(*_31).2,_7.1.2,(*_31).2];
(*_13) = [7_usize,4072411427652522715_usize,5690623959611536069_usize];
_25 = !_17;
_32 = &(*_23);
(*_13) = [7683450846538934951_usize,4_usize,2_usize];
(*_2) = _7.0;
_22 = &(*_32);
(*_13) = [1_usize,13216023039309558014_usize,1540479152744101315_usize];
_43 = [14664065620889883914_u64,10002921637797923942_u64,6533102631588998663_u64,762801152380768839_u64,109153742619059328_u64,12366474276336203338_u64,12751960295660061672_u64];
(*_13) = [1385499382249845737_usize,9001375296037251859_usize,4_usize];
_39 = _34 == _27.3;
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
match _7.1.2 {
0 => bb6,
15143 => bb33,
_ => bb14
}
}
bb41 = {
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_3 = [15708347575814504860_u64];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = _7.1.3;
_7.1.3 = (*_2);
_7.1.3 = (*_2);
(*_2) = _7.1.3;
_10 = -_6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_6 = _10 * _10;
_6 = _10 * _10;
_1 = Move(_7.1.1);
Goto(bb3)
}
bb42 = {
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_7.1.1 = Move(_1);
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_3 = [15708347575814504860_u64];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = _7.1.3;
(*_2) = _7.1.3;
_7.1.3 = (*_2);
_7.1.3 = (*_2);
(*_2) = _7.1.3;
_10 = -_6;
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
(*_2) = [_7.1.2,_7.1.2,_7.1.2,_7.1.2];
_6 = _10 * _10;
_6 = _10 * _10;
_1 = Move(_7.1.1);
Goto(bb3)
}
bb43 = {
(*_1) = '\u{2d198}';
(*_42) = _19 * _19;
_10 = _35 + _35;
(*_2) = _7.0;
_28 = !_27.0;
_46 = 18320586435755381071_u64 as isize;
_21 = '\u{7b0a7}';
_3 = [10332068115637150767_u64];
(*_42) = _34 as i8;
_10 = _35 * _35;
_48 = (*_42) < (*_42);
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_35 = -_10;
RET = !(*_32);
(*_13) = [12333275319978939663_usize,4_usize,6759152286770800851_usize];
(*_13) = [13453505588943010752_usize,9825876227590894345_usize,13154269299638911_usize];
Goto(bb44)
}
bb44 = {
(*_13) = [1750972433841275298_usize,3_usize,7_usize];
_7.1.0 = [49139_u16,25079_u16,56183_u16,34547_u16,7133_u16];
_21 = '\u{446f0}';
(*_42) = _19;
_28 = _17 & _25;
_44 = _10 + _35;
_13 = core::ptr::addr_of!((*_13));
(*_2) = _7.0;
(*_1) = '\u{5913e}';
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_35 = _10 * _44;
(*_1) = '\u{bba4c}';
_14 = [5_usize,0_usize,1_usize];
_7.0 = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_50 = _44 - _10;
(*_42) = _19 * _19;
(*_2) = _7.0;
_15 = _36;
_27.3 = -_34;
(*_13) = [17173125338744792027_usize,5_usize,7_usize];
_27.1 = 10172_u16 as u8;
(*_1) = '\u{30fac}';
_49 = (*_1);
Goto(bb45)
}
bb45 = {
_47 = core::ptr::addr_of!((*_42));
_10 = _44 - _44;
(*_47) = _28 as i8;
(*_42) = -_19;
(*_13) = [8061211053285276220_usize,2_usize,11641773464357636116_usize];
_2 = core::ptr::addr_of_mut!((*_2));
_55 = -_27.0;
_13 = core::ptr::addr_of!((*_13));
_48 = !_33;
(*_1) = _49;
_17 = _55 >> (*_32);
_10 = _35 + _35;
(*_42) = _44 as i8;
_38 = (*_31).2;
_24 = Move(_32);
(*_1) = _49;
(*_42) = !_19;
(*_2) = _7.0;
_1 = core::ptr::addr_of_mut!((*_1));
(*_42) = _19 + _19;
_56 = _27.0 < _27.0;
_27.0 = _17 << (*_42);
_15 = _36 << _36;
(*_1) = _49;
match _38 {
0 => bb44,
1 => bb25,
15143 => bb46,
_ => bb20
}
}
bb46 = {
RET = _30 & _30;
_61.1.1.1 = Move(_7.1.1);
_7.0 = (*_2);
(*_42) = 1_usize as i8;
(*_42) = _19 + _19;
_33 = _48 > _48;
_61.1.1.2 = (*_31).2 ^ (*_31).2;
match _34 {
0 => bb32,
1 => bb24,
2 => bb27,
340282366920938463454915112923424317751 => bb47,
_ => bb38
}
}
bb47 = {
_61.1.1.0 = [2061_u16,50276_u16,54736_u16,8890_u16,13106_u16];
(*_1) = _49;
_27 = (_55, RET, 131952519239744529082194205090725989854_i128, _34);
(*_13) = [10795585831069361883_usize,15837174682931185667_usize,12322844688582479463_usize];
(*_42) = _19;
(*_42) = _34 as i8;
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
_9 = core::ptr::addr_of_mut!(_63);
(*_1) = _49;
_57 = _56;
(*_13) = [0_usize,2_usize,4_usize];
(*_9) = 2925793929_u32 | 3064561417_u32;
(*_9) = (*_42) as u32;
_32 = &_27.1;
(*_2) = [(*_31).2,(*_31).2,(*_31).2,(*_31).2];
(*_1) = _49;
_20 = (*_9) as f64;
(*_2) = [(*_31).2,_38,(*_31).2,(*_31).2];
_3 = [15730845057139201382_u64];
match _27.2 {
131952519239744529082194205090725989854 => bb49,
_ => bb48
}
}
bb48 = {
(*_13) = [6_usize,3_usize,14907700727819298727_usize];
(*_1) = '\u{4555c}';
(*_1) = '\u{efb5f}';
(*_13) = [7_usize,3_usize,5_usize];
_7.1.1 = core::ptr::addr_of_mut!((*_1));
(*_1) = '\u{de582}';
(*_13) = [6_usize,7376885791630700124_usize,1035680907249312543_usize];
(*_13) = [18285283497670001330_usize,7754006128792675451_usize,5_usize];
_17 = _16 >> _19;
(*_13) = [4_usize,6484131094007302125_usize,8792285527444120371_usize];
_7.1.0 = [34442_u16,53233_u16,38329_u16,38509_u16,51338_u16];
_6 = -_10;
(*_1) = '\u{ad93d}';
match _7.1.2 {
0 => bb6,
1 => bb4,
15143 => bb10,
_ => bb9
}
}
bb49 = {
_33 = _57 | _39;
_65 = _27.2 as u32;
_19 = !(*_42);
_34 = _27.3 << (*_9);
_27.3 = _34 * _34;
_61.1.1.3 = [_61.1.1.2,(*_31).2,(*_31).2,(*_31).2];
_34 = _35 as i64;
_58 = !(*_32);
(*_13) = [6_usize,6_usize,6_usize];
(*_13) = [7_usize,11813702897602056279_usize,6_usize];
Goto(bb50)
}
bb50 = {
Call(_71 = dump_var(Move(_63), Move(_17), Move(_56), Move(_46)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_71 = dump_var(Move(_28), Move(_21), Move(_49), Move(_43)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_71 = dump_var(Move(_3), Move(_14), Move(_34), Move(_65)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_71 = dump_var(Move(_33), _72, _72, _72), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i16; 4],mut _2: [i16; 4],mut _3: *mut [i16; 4],mut _4: [i16; 4],mut _5: [i16; 4],mut _6: [i16; 4],mut _7: *mut char,mut _8: [i16; 4],mut _9: [i16; 4],mut _10: [i16; 4],mut _11: [i16; 4],mut _12: [i16; 4]) -> *mut u32 {
mir! {
type RET = *mut u32;
let _13: *mut [i16; 4];
let _14: f32;
let _15: u128;
let _16: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _17: isize;
let _18: f64;
let _19: char;
let _20: usize;
let _21: &'static f64;
let _22: isize;
let _23: isize;
let _24: bool;
let _25: f64;
let _26: isize;
let _27: (&'static Adt25, char, *mut char);
let _28: [u64; 7];
let _29: [u32; 8];
let _30: Adt62;
let _31: bool;
let _32: [i128; 4];
let _33: u8;
let _34: *mut u32;
let _35: &'static &'static (i32, (usize, u128, f32), f64);
let _36: [isize; 8];
let _37: &'static u8;
let _38: f64;
let _39: f64;
let _40: *const [char; 4];
let _41: i64;
let _42: &'static mut usize;
let _43: isize;
let _44: isize;
let _45: f64;
let _46: &'static mut Adt42;
let _47: [i64; 7];
let _48: f64;
let _49: i8;
let _50: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _51: usize;
let _52: isize;
let _53: &'static mut usize;
let _54: &'static mut Adt42;
let _55: bool;
let _56: Adt78;
let _57: isize;
let _58: [char; 8];
let _59: i16;
let _60: [u32; 8];
let _61: isize;
let _62: f64;
let _63: char;
let _64: Adt68;
let _65: &'static mut u16;
let _66: *mut char;
let _67: isize;
let _68: char;
let _69: *mut &'static Adt25;
let _70: u64;
let _71: char;
let _72: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _73: &'static mut usize;
let _74: bool;
let _75: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _76: (i32, (usize, u128, f32), f64);
let _77: f64;
let _78: u32;
let _79: bool;
let _80: f32;
let _81: isize;
let _82: u64;
let _83: isize;
let _84: f64;
let _85: &'static mut usize;
let _86: usize;
let _87: [u64; 7];
let _88: *const [usize; 3];
let _89: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _90: f64;
let _91: i16;
let _92: bool;
let _93: ([i128; 4], &'static [char; 4]);
let _94: f32;
let _95: [i32; 1];
let _96: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _97: &'static [char; 4];
let _98: &'static mut Adt42;
let _99: *mut char;
let _100: (&'static Adt25, char, *mut char);
let _101: u64;
let _102: Adt78;
let _103: i64;
let _104: *const f64;
let _105: [i16; 4];
let _106: isize;
let _107: ();
let _108: ();
{
_10 = [(-23851_i16),(-421_i16),(-4879_i16),22641_i16];
_12 = [656_i16,13879_i16,31963_i16,(-28258_i16)];
_12 = [(-1402_i16),(-394_i16),12477_i16,(-27944_i16)];
_12 = [19949_i16,(-7967_i16),(-28912_i16),(-16753_i16)];
_13 = core::ptr::addr_of_mut!(_5);
_5 = [(-26275_i16),(-8317_i16),20399_i16,(-13340_i16)];
_4 = [19317_i16,28070_i16,(-31277_i16),6255_i16];
_6 = [(-13862_i16),16778_i16,(-21726_i16),16289_i16];
_9 = [21661_i16,23205_i16,3703_i16,27722_i16];
_9 = [(-8520_i16),11446_i16,30966_i16,15715_i16];
_4 = [13616_i16,28870_i16,2804_i16,20734_i16];
_3 = core::ptr::addr_of_mut!((*_13));
(*_13) = [20164_i16,5374_i16,(-13326_i16),8017_i16];
_13 = core::ptr::addr_of_mut!((*_13));
_4 = (*_13);
_6 = [(-9970_i16),25705_i16,5959_i16,(-15880_i16)];
(*_13) = [(-21430_i16),3766_i16,16710_i16,12928_i16];
_10 = [(-3995_i16),2714_i16,(-5991_i16),(-20524_i16)];
_3 = core::ptr::addr_of_mut!((*_13));
_6 = [23925_i16,(-29325_i16),(-28693_i16),(-15185_i16)];
(*_13) = [1458_i16,(-17766_i16),(-9867_i16),(-20383_i16)];
_11 = [(-25092_i16),7987_i16,6549_i16,10807_i16];
_8 = [(-22650_i16),(-30350_i16),(-26768_i16),(-15966_i16)];
Goto(bb1)
}
bb1 = {
(*_13) = _2;
_10 = _8;
(*_13) = [22624_i16,(-17759_i16),(-2527_i16),9167_i16];
(*_13) = [(-22663_i16),11834_i16,30214_i16,(-21807_i16)];
Goto(bb2)
}
bb2 = {
(*_13) = _11;
_10 = _12;
(*_13) = _11;
_15 = !256407808980369931734713201234919284702_u128;
(*_13) = [(-30931_i16),30890_i16,28192_i16,(-19724_i16)];
(*_13) = _12;
(*_13) = _10;
_12 = (*_13);
_9 = [(-17782_i16),(-1764_i16),(-16030_i16),(-4578_i16)];
_12 = [(-22678_i16),16194_i16,22094_i16,(-14484_i16)];
_6 = [(-24617_i16),(-24195_i16),(-20523_i16),(-7353_i16)];
_18 = 65_i8 as f64;
(*_13) = [30281_i16,(-29008_i16),4404_i16,26215_i16];
_1 = (*_13);
(*_13) = _4;
(*_13) = [26961_i16,9381_i16,6066_i16,16473_i16];
_3 = core::ptr::addr_of_mut!((*_13));
_17 = -(-125_isize);
_20 = 8006802234788278215_usize ^ 1382458766748075416_usize;
(*_13) = [13151_i16,21628_i16,(-24841_i16),(-4295_i16)];
_18 = 41_i8 as f64;
(*_13) = [7308_i16,22433_i16,26665_i16,29334_i16];
_6 = [11182_i16,(-14381_i16),6890_i16,(-508_i16)];
Goto(bb3)
}
bb3 = {
(*_13) = _10;
_18 = (-16296_i16) as f64;
(*_13) = [5928_i16,29806_i16,29184_i16,(-15844_i16)];
_7 = core::ptr::addr_of_mut!(_19);
(*_7) = '\u{6cb2c}';
(*_13) = [30482_i16,26121_i16,9190_i16,(-15295_i16)];
_24 = (*_7) > (*_7);
(*_13) = _6;
_11 = [(-31460_i16),30568_i16,7077_i16,31962_i16];
_12 = [(-23770_i16),17165_i16,(-25817_i16),(-5535_i16)];
(*_7) = '\u{3406f}';
Call(_25 = fn7((*_13), Move(_7), (*_7), (*_7), (*_7), (*_13), Move(_3), (*_7), _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = [14400_i16,5406_i16,20385_i16,(-30432_i16)];
_11 = [(-26652_i16),31122_i16,(-27926_i16),(-31291_i16)];
(*_13) = [13599_i16,(-7524_i16),(-20968_i16),15068_i16];
(*_13) = [9437_i16,30491_i16,22018_i16,(-22221_i16)];
_3 = core::ptr::addr_of_mut!(_8);
_15 = 125854269006199230438973030808159745363_u128 + 306210989510612335676130373237023686622_u128;
(*_13) = [(-1056_i16),10898_i16,(-22175_i16),18912_i16];
(*_3) = [(-21025_i16),20115_i16,3556_i16,(-13916_i16)];
(*_3) = [20418_i16,(-8552_i16),(-4603_i16),(-4344_i16)];
_8 = [3815_i16,(-23605_i16),11492_i16,26579_i16];
(*_13) = [(-5315_i16),11134_i16,5717_i16,14388_i16];
(*_13) = (*_3);
(*_3) = (*_13);
(*_13) = [(-6408_i16),31579_i16,(-7107_i16),(-20158_i16)];
Goto(bb5)
}
bb5 = {
_19 = '\u{22c2e}';
_22 = _17 - _17;
_1 = (*_3);
(*_13) = [28805_i16,(-10528_i16),27763_i16,(-11332_i16)];
_13 = core::ptr::addr_of_mut!((*_13));
(*_13) = (*_3);
_27.1 = _19;
_12 = [15737_i16,23482_i16,11816_i16,25005_i16];
(*_13) = (*_3);
(*_3) = [27285_i16,24085_i16,(-23301_i16),(-20685_i16)];
_10 = [(-8915_i16),92_i16,24898_i16,14828_i16];
(*_3) = [818_i16,17512_i16,3472_i16,(-23958_i16)];
Goto(bb6)
}
bb6 = {
(*_3) = [14658_i16,(-11981_i16),14595_i16,(-21785_i16)];
(*_3) = [(-32557_i16),23908_i16,(-28213_i16),(-5528_i16)];
_28 = [14130136189126302378_u64,5670149390990284456_u64,1433955076644364203_u64,13391128242843679489_u64,13435683884553623468_u64,10469027735538725536_u64,5611524238045871391_u64];
(*_13) = _9;
_20 = !11767889240394134075_usize;
(*_13) = [(-11208_i16),7330_i16,(-17825_i16),26735_i16];
(*_13) = [(-30533_i16),7372_i16,(-6962_i16),(-18670_i16)];
(*_3) = (*_13);
_25 = 102530078427259025931500155102969107356_i128 as f64;
(*_3) = _6;
_9 = [9475_i16,14737_i16,(-6996_i16),(-6839_i16)];
_15 = 50080237798866085969503480290255658266_u128 << _22;
_20 = 5_usize & 8810979135705182397_usize;
Goto(bb7)
}
bb7 = {
(*_3) = [5423_i16,17731_i16,15049_i16,7764_i16];
_19 = _27.1;
_10 = (*_13);
_29 = [3373241055_u32,2062767479_u32,2574877927_u32,439087010_u32,2522256197_u32,4092047499_u32,3638708421_u32,902445193_u32];
_5 = [(-17698_i16),12146_i16,(-28802_i16),29223_i16];
(*_3) = [18300_i16,(-1124_i16),4829_i16,20454_i16];
_27.2 = core::ptr::addr_of_mut!(_27.1);
(*_3) = [(-2357_i16),7176_i16,(-6817_i16),192_i16];
(*_13) = [22136_i16,5684_i16,(-31514_i16),(-651_i16)];
_21 = &_18;
(*_3) = [(-25136_i16),23670_i16,(-27120_i16),28007_i16];
_33 = 155_u8 >> _17;
_14 = 61_i8 as f32;
(*_13) = [(-18002_i16),(-11965_i16),(-970_i16),(-18566_i16)];
(*_13) = [32437_i16,(-510_i16),792_i16,13857_i16];
(*_13) = [10941_i16,(-23341_i16),1141_i16,(-16289_i16)];
(*_13) = [27502_i16,(-31384_i16),27742_i16,(-31903_i16)];
Goto(bb8)
}
bb8 = {
(*_3) = [(-24731_i16),(-9115_i16),9197_i16,(-2290_i16)];
(*_13) = [(-9641_i16),22428_i16,(-26273_i16),23849_i16];
_7 = core::ptr::addr_of_mut!(_19);
(*_7) = _27.1;
_26 = _22;
(*_7) = _27.1;
(*_7) = _27.1;
(*_3) = [10395_i16,(-23766_i16),(-16888_i16),25019_i16];
(*_3) = [10967_i16,23422_i16,(-858_i16),359_i16];
(*_3) = [(-19353_i16),(-5117_i16),(-9046_i16),(-58_i16)];
(*_3) = [28194_i16,(-3901_i16),(-23929_i16),(-30664_i16)];
(*_7) = _27.1;
(*_7) = _27.1;
(*_3) = [31060_i16,25707_i16,1238_i16,(-30447_i16)];
(*_7) = _27.1;
_9 = (*_13);
(*_13) = (*_3);
_28 = [10382127650436089118_u64,7389852899738381594_u64,11301098516718498060_u64,17774169008413271364_u64,2892301975491098002_u64,10164892385010387396_u64,8685815757105404070_u64];
(*_7) = _27.1;
_24 = false;
(*_13) = (*_3);
Goto(bb9)
}
bb9 = {
(*_13) = [24994_i16,(-4368_i16),30304_i16,(-16350_i16)];
_17 = _22 ^ _26;
(*_7) = _27.1;
_5 = [(-25739_i16),(-26718_i16),(-28210_i16),17421_i16];
_36 = [_26,_26,_17,_22,_22,_22,_17,_17];
(*_7) = _27.1;
(*_3) = [(-20179_i16),(-30447_i16),30661_i16,16294_i16];
_17 = _22;
(*_13) = [2768_i16,27192_i16,(-28425_i16),18138_i16];
_37 = &_33;
_2 = [30561_i16,20007_i16,(-8079_i16),(-29112_i16)];
(*_13) = [2277_i16,(-4179_i16),16710_i16,25932_i16];
(*_7) = _27.1;
(*_13) = _9;
Goto(bb10)
}
bb10 = {
_38 = (*_21) - (*_21);
(*_7) = _27.1;
_36 = [_26,_26,_22,_17,_26,_22,_17,_22];
(*_13) = [(-30548_i16),12668_i16,26117_i16,(-8089_i16)];
_33 = 1_u8 >> _26;
_21 = &_25;
(*_3) = [(-6431_i16),30522_i16,(-1144_i16),31771_i16];
_36 = [_26,_26,_17,_26,_26,_17,_26,_17];
_3 = core::ptr::addr_of_mut!((*_3));
(*_13) = [(-11887_i16),(-18637_i16),1841_i16,16945_i16];
_14 = _33 as f32;
(*_13) = [(-32297_i16),(-8320_i16),3573_i16,(-13189_i16)];
(*_13) = (*_3);
(*_7) = _27.1;
(*_3) = [(-20197_i16),(-16408_i16),17958_i16,30698_i16];
(*_3) = [3333_i16,654_i16,(-20668_i16),(-17126_i16)];
(*_3) = (*_13);
(*_13) = (*_3);
(*_3) = (*_13);
(*_3) = [15862_i16,(-18977_i16),3808_i16,(-11803_i16)];
Goto(bb11)
}
bb11 = {
(*_3) = [7470_i16,22961_i16,(-30134_i16),16345_i16];
_7 = core::ptr::addr_of_mut!((*_7));
Call(_33 = core::intrinsics::transmute(_24), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_3) = [9063_i16,(-21310_i16),28356_i16,(-20312_i16)];
_14 = _33 as f32;
(*_7) = _27.1;
_21 = &_18;
(*_13) = [(-22173_i16),(-26143_i16),4360_i16,(-7641_i16)];
(*_3) = [27724_i16,17324_i16,(-29736_i16),(-25183_i16)];
(*_13) = [13954_i16,14854_i16,(-26726_i16),(-32478_i16)];
(*_13) = [(-6314_i16),21112_i16,12639_i16,3854_i16];
_39 = (*_21) * (*_21);
_9 = [(-23599_i16),(-22984_i16),23364_i16,(-17138_i16)];
(*_7) = _27.1;
(*_13) = [(-32720_i16),(-23371_i16),(-16934_i16),(-21044_i16)];
(*_7) = _27.1;
Goto(bb13)
}
bb13 = {
_43 = 40553_u16 as isize;
_45 = (*_21) + (*_21);
(*_13) = [(-24785_i16),(-5516_i16),(-6893_i16),14212_i16];
(*_13) = [(-19129_i16),(-9181_i16),(-32555_i16),(-13225_i16)];
(*_3) = _10;
_25 = -(*_21);
(*_3) = [(-1355_i16),9427_i16,19770_i16,6124_i16];
(*_7) = _27.1;
_24 = true;
(*_7) = _27.1;
Goto(bb14)
}
bb14 = {
_15 = 196276338709236081669914855815331630431_u128 & 185467561879816663042175733105126620379_u128;
(*_7) = _27.1;
_11 = [3648_i16,22944_i16,16967_i16,1955_i16];
(*_7) = _27.1;
(*_13) = [18919_i16,(-9707_i16),25330_i16,30895_i16];
_32 = [71540822503491971872051521284577705295_i128,101933132600193414166447321088641240726_i128,(-164834652500525282992303749690762549535_i128),(-122897163902656639420181928456477885334_i128)];
(*_7) = _27.1;
(*_3) = [11139_i16,(-15064_i16),13599_i16,11879_i16];
Goto(bb15)
}
bb15 = {
(*_3) = [(-19839_i16),25105_i16,26007_i16,18495_i16];
(*_13) = [(-8722_i16),27666_i16,17421_i16,27293_i16];
(*_13) = [22675_i16,(-12466_i16),28531_i16,(-2684_i16)];
(*_7) = _27.1;
_10 = [(-22781_i16),(-21684_i16),(-20166_i16),16696_i16];
(*_7) = _27.1;
(*_7) = _27.1;
(*_7) = _27.1;
_27.2 = core::ptr::addr_of_mut!((*_7));
_13 = core::ptr::addr_of_mut!(_4);
_20 = 13709264354698777950_usize >> _33;
(*_3) = [(-21538_i16),(-22664_i16),(-1309_i16),(-4961_i16)];
(*_7) = _27.1;
_17 = _43;
Goto(bb16)
}
bb16 = {
(*_7) = _27.1;
_5 = [21827_i16,(-31359_i16),18813_i16,20399_i16];
(*_7) = _27.1;
_31 = (*_7) != (*_7);
_50.2 = Adt25::Variant0 { fld0: (*_21),fld1: 499196334_u32 };
place!(Field::<f64>(Variant(_50.2, 0), 0)) = _18;
_20 = 7_usize | 12224887886817593353_usize;
_31 = (*_7) < (*_7);
(*_3) = (*_13);
place!(Field::<u32>(Variant(_50.2, 0), 1)) = 3208210971_u32 ^ 1905420721_u32;
_42 = &mut _20;
(*_13) = [13784_i16,(-27225_i16),(-3325_i16),(-27196_i16)];
_26 = _22 ^ _22;
(*_13) = [(-28325_i16),1391_i16,22806_i16,631_i16];
(*_13) = _10;
_49 = (-107_i8);
_23 = _26 >> (*_42);
_50.0 = [(-1403_i16),1583_i16,(-12247_i16),(-73_i16)];
_34 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_50.2, 0), 1)));
(*_7) = _27.1;
_21 = &_25;
(*_42) = 10889615625137536791_usize | 2_usize;
(*_3) = [28320_i16,(-5444_i16),(-238_i16),(-5723_i16)];
(*_3) = _12;
Goto(bb17)
}
bb17 = {
(*_3) = [(-5799_i16),(-5392_i16),(-11238_i16),(-25212_i16)];
(*_42) = 10996390470060580576_usize >> (*_34);
_48 = (*_21) - (*_21);
(*_42) = _31 as usize;
_57 = _23;
_2 = [8970_i16,11561_i16,32434_i16,28221_i16];
_29 = [(*_34),(*_34),(*_34),(*_34),(*_34),Field::<u32>(Variant(_50.2, 0), 1),(*_34),(*_34)];
Call(_44 = core::intrinsics::bswap(_23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_13) = [3286_i16,(-24792_i16),12878_i16,(-9880_i16)];
Goto(bb19)
}
bb19 = {
_34 = core::ptr::addr_of_mut!((*_34));
_43 = !_23;
(*_13) = (*_3);
(*_7) = _27.1;
(*_7) = _27.1;
_50.1.3 = (*_13);
(*_42) = !7_usize;
(*_34) = !1586650384_u32;
(*_34) = 1019307209_u32 | 3322310400_u32;
(*_42) = 2_usize - 6541455839222735366_usize;
(*_34) = 2652396702_u32;
(*_7) = _27.1;
(*_13) = [(-28680_i16),541_i16,(-4903_i16),(-15696_i16)];
match Field::<u32>(Variant(_50.2, 0), 1) {
0 => bb1,
2652396702 => bb21,
_ => bb20
}
}
bb20 = {
(*_3) = [5423_i16,17731_i16,15049_i16,7764_i16];
_19 = _27.1;
_10 = (*_13);
_29 = [3373241055_u32,2062767479_u32,2574877927_u32,439087010_u32,2522256197_u32,4092047499_u32,3638708421_u32,902445193_u32];
_5 = [(-17698_i16),12146_i16,(-28802_i16),29223_i16];
(*_3) = [18300_i16,(-1124_i16),4829_i16,20454_i16];
_27.2 = core::ptr::addr_of_mut!(_27.1);
(*_3) = [(-2357_i16),7176_i16,(-6817_i16),192_i16];
(*_13) = [22136_i16,5684_i16,(-31514_i16),(-651_i16)];
_21 = &_18;
(*_3) = [(-25136_i16),23670_i16,(-27120_i16),28007_i16];
_33 = 155_u8 >> _17;
_14 = 61_i8 as f32;
(*_13) = [(-18002_i16),(-11965_i16),(-970_i16),(-18566_i16)];
(*_13) = [32437_i16,(-510_i16),792_i16,13857_i16];
(*_13) = [10941_i16,(-23341_i16),1141_i16,(-16289_i16)];
(*_13) = [27502_i16,(-31384_i16),27742_i16,(-31903_i16)];
Goto(bb8)
}
bb21 = {
(*_13) = _50.1.3;
(*_34) = !195393124_u32;
_33 = !207_u8;
_55 = (*_34) != Field::<u32>(Variant(_50.2, 0), 1);
(*_7) = _27.1;
(*_7) = _27.1;
(*_13) = [19748_i16,(-32023_i16),28859_i16,(-22284_i16)];
_61 = _43 | _23;
(*_34) = 814779243_u32;
_50.1.1 = core::ptr::addr_of_mut!((*_7));
(*_34) = (*_42) as u32;
(*_7) = _27.1;
_4 = [(-14173_i16),12793_i16,13841_i16,(-6107_i16)];
_64.fld0.1.1 = core::ptr::addr_of_mut!((*_7));
(*_13) = [(-13500_i16),(-30986_i16),25431_i16,(-25184_i16)];
_32 = [160368643573980321881600887247766851398_i128,119759475873430285764230690786907524310_i128,136668725084250118101341310590588450018_i128,(-39594657688838535243691603078194222177_i128)];
Goto(bb22)
}
bb22 = {
_10 = (*_13);
_33 = 145_u8;
_44 = _57 & _17;
(*_3) = [30407_i16,6960_i16,(-24587_i16),14525_i16];
_61 = _43 * _44;
_49 = 51_i8 << (*_42);
_51 = !(*_42);
(*_7) = _27.1;
_2 = _11;
_50.1.0 = [53806_u16,37312_u16,557_u16,56983_u16,26510_u16];
_12 = [(-13868_i16),(-11761_i16),(-13362_i16),25959_i16];
Goto(bb23)
}
bb23 = {
_31 = _55 | _55;
(*_13) = [(-7072_i16),10052_i16,(-31674_i16),21946_i16];
_12 = [(-27433_i16),(-32228_i16),2341_i16,(-6031_i16)];
_50.1.1 = core::ptr::addr_of_mut!((*_7));
_27.2 = core::ptr::addr_of_mut!((*_7));
(*_7) = _27.1;
_64.fld0.2 = Move(_50.2);
(*_34) = Field::<u32>(Variant(_64.fld0.2, 0), 1) - Field::<u32>(Variant(_64.fld0.2, 0), 1);
_11 = (*_13);
_27.1 = (*_7);
(*_3) = [19918_i16,26879_i16,(-31084_i16),(-8763_i16)];
(*_42) = _51;
(*_34) = Field::<u32>(Variant(_64.fld0.2, 0), 1) << _61;
_62 = _14 as f64;
Call((*_34) = core::intrinsics::transmute((*_7)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_13) = _5;
(*_42) = _51 + _51;
(*_7) = _27.1;
(*_42) = _51 >> _57;
_50.2 = Adt25::Variant0 { fld0: (*_21),fld1: Field::<u32>(Variant(_64.fld0.2, 0), 1) };
(*_3) = (*_13);
_49 = 11_i8 & 58_i8;
(*_7) = _27.1;
(*_7) = _27.1;
_6 = [(-25648_i16),20189_i16,7524_i16,(-21344_i16)];
_41 = -(-8584774802160038848_i64);
_18 = _39;
(*_42) = _51 + _51;
_64.fld0.1 = (_50.1.0, Move(_27.2), (-4906_i16), (*_13));
(*_7) = _27.1;
_66 = core::ptr::addr_of_mut!(_19);
(*_3) = [_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
_39 = Field::<f64>(Variant(_64.fld0.2, 0), 0) + _45;
_64.fld2 = core::ptr::addr_of!(_62);
_12 = [_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
_71 = (*_7);
_16 = &_64.fld0.1;
_9 = [_64.fld0.1.2,(*_16).2,(*_16).2,(*_16).2];
match (*_16).2 {
0 => bb25,
340282366920938463463374607431768206550 => bb27,
_ => bb26
}
}
bb25 = {
_38 = (*_21) - (*_21);
(*_7) = _27.1;
_36 = [_26,_26,_22,_17,_26,_22,_17,_22];
(*_13) = [(-30548_i16),12668_i16,26117_i16,(-8089_i16)];
_33 = 1_u8 >> _26;
_21 = &_25;
(*_3) = [(-6431_i16),30522_i16,(-1144_i16),31771_i16];
_36 = [_26,_26,_17,_26,_26,_17,_26,_17];
_3 = core::ptr::addr_of_mut!((*_3));
(*_13) = [(-11887_i16),(-18637_i16),1841_i16,16945_i16];
_14 = _33 as f32;
(*_13) = [(-32297_i16),(-8320_i16),3573_i16,(-13189_i16)];
(*_13) = (*_3);
(*_7) = _27.1;
(*_3) = [(-20197_i16),(-16408_i16),17958_i16,30698_i16];
(*_3) = [3333_i16,654_i16,(-20668_i16),(-17126_i16)];
(*_3) = (*_13);
(*_13) = (*_3);
(*_3) = (*_13);
(*_3) = [15862_i16,(-18977_i16),3808_i16,(-11803_i16)];
Goto(bb11)
}
bb26 = {
_34 = core::ptr::addr_of_mut!((*_34));
_43 = !_23;
(*_13) = (*_3);
(*_7) = _27.1;
(*_7) = _27.1;
_50.1.3 = (*_13);
(*_42) = !7_usize;
(*_34) = !1586650384_u32;
(*_34) = 1019307209_u32 | 3322310400_u32;
(*_42) = 2_usize - 6541455839222735366_usize;
(*_34) = 2652396702_u32;
(*_7) = _27.1;
(*_13) = [(-28680_i16),541_i16,(-4903_i16),(-15696_i16)];
match Field::<u32>(Variant(_50.2, 0), 1) {
0 => bb1,
2652396702 => bb21,
_ => bb20
}
}
bb27 = {
(*_7) = _27.1;
(*_3) = _4;
_50.1.1 = core::ptr::addr_of_mut!((*_7));
match (*_16).2 {
0 => bb9,
1 => bb16,
340282366920938463463374607431768206550 => bb28,
_ => bb10
}
}
bb28 = {
_9 = [(*_16).2,_64.fld0.1.2,(*_16).2,(*_16).2];
(*_3) = [(*_16).2,(*_16).2,(*_16).2,(*_16).2];
(*_13) = [(*_16).2,(*_16).2,(*_16).2,(*_16).2];
_18 = (*_21);
_9 = [(*_16).2,(*_16).2,(*_16).2,(*_16).2];
(*_13) = [(*_16).2,(*_16).2,(*_16).2,(*_16).2];
_50 = ((*_3), Move(_64.fld0.1), Move(_64.fld0.2));
(*_13) = (*_3);
_68 = (*_7);
(*_42) = !_51;
_71 = (*_7);
_64.fld0 = Move(_50);
_53 = Move(_42);
(*_3) = [_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
_18 = (*_21) + Field::<f64>(Variant(_64.fld0.2, 0), 0);
Goto(bb29)
}
bb29 = {
(*_7) = _71;
_27.1 = (*_7);
(*_7) = _71;
_47 = [_41,_41,_41,_41,_41,_41,_41];
_50 = Move(_64.fld0);
_76.1.1 = !_15;
(*_13) = (*_3);
_73 = &mut _51;
(*_13) = [_50.1.2,_50.1.2,_50.1.2,_50.1.2];
_76.1.0 = _76.1.1 as usize;
_48 = (*_21);
_26 = _44;
_25 = _39 - _39;
_64.fld0.1.0 = [32605_u16,44536_u16,55868_u16,8015_u16,29121_u16];
(*_73) = 220014323_i32 as usize;
_45 = -_39;
(*_3) = [_50.1.2,_50.1.2,_50.1.2,_50.1.2];
_5 = [_50.1.2,_50.1.2,_50.1.2,_50.1.2];
_44 = !_57;
_64.fld0.1.2 = !_50.1.2;
_42 = Move(_73);
_26 = _61 + _61;
_73 = &mut _76.1.0;
match _50.1.2 {
0 => bb25,
1 => bb2,
340282366920938463463374607431768206550 => bb30,
_ => bb13
}
}
bb30 = {
(*_73) = 1_usize + 1_usize;
(*_13) = [_50.1.2,_64.fld0.1.2,_50.1.2,_64.fld0.1.2];
(*_13) = [_50.1.2,_50.1.2,_50.1.2,_64.fld0.1.2];
_12 = [_64.fld0.1.2,_50.1.2,_50.1.2,_64.fld0.1.2];
(*_3) = [_50.1.2,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
(*_7) = _71;
_32 = [(-166980179208676788754014117076340399348_i128),156019270525663393741276250614692482377_i128,37045214714452642482245345644504674443_i128,97675437473075814623349719983611675705_i128];
_2 = [_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2,_50.1.2];
(*_73) = !5_usize;
_77 = _38 * _25;
_6 = [_50.1.2,_64.fld0.1.2,_50.1.2,_64.fld0.1.2];
_74 = !_55;
match _50.1.2 {
0 => bb14,
1 => bb10,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
6 => bb35,
340282366920938463463374607431768206550 => bb37,
_ => bb36
}
}
bb31 = {
(*_3) = [9063_i16,(-21310_i16),28356_i16,(-20312_i16)];
_14 = _33 as f32;
(*_7) = _27.1;
_21 = &_18;
(*_13) = [(-22173_i16),(-26143_i16),4360_i16,(-7641_i16)];
(*_3) = [27724_i16,17324_i16,(-29736_i16),(-25183_i16)];
(*_13) = [13954_i16,14854_i16,(-26726_i16),(-32478_i16)];
(*_13) = [(-6314_i16),21112_i16,12639_i16,3854_i16];
_39 = (*_21) * (*_21);
_9 = [(-23599_i16),(-22984_i16),23364_i16,(-17138_i16)];
(*_7) = _27.1;
(*_13) = [(-32720_i16),(-23371_i16),(-16934_i16),(-21044_i16)];
(*_7) = _27.1;
Goto(bb13)
}
bb32 = {
_38 = (*_21) - (*_21);
(*_7) = _27.1;
_36 = [_26,_26,_22,_17,_26,_22,_17,_22];
(*_13) = [(-30548_i16),12668_i16,26117_i16,(-8089_i16)];
_33 = 1_u8 >> _26;
_21 = &_25;
(*_3) = [(-6431_i16),30522_i16,(-1144_i16),31771_i16];
_36 = [_26,_26,_17,_26,_26,_17,_26,_17];
_3 = core::ptr::addr_of_mut!((*_3));
(*_13) = [(-11887_i16),(-18637_i16),1841_i16,16945_i16];
_14 = _33 as f32;
(*_13) = [(-32297_i16),(-8320_i16),3573_i16,(-13189_i16)];
(*_13) = (*_3);
(*_7) = _27.1;
(*_3) = [(-20197_i16),(-16408_i16),17958_i16,30698_i16];
(*_3) = [3333_i16,654_i16,(-20668_i16),(-17126_i16)];
(*_3) = (*_13);
(*_13) = (*_3);
(*_3) = (*_13);
(*_3) = [15862_i16,(-18977_i16),3808_i16,(-11803_i16)];
Goto(bb11)
}
bb33 = {
(*_13) = [3286_i16,(-24792_i16),12878_i16,(-9880_i16)];
Goto(bb19)
}
bb34 = {
_15 = 196276338709236081669914855815331630431_u128 & 185467561879816663042175733105126620379_u128;
(*_7) = _27.1;
_11 = [3648_i16,22944_i16,16967_i16,1955_i16];
(*_7) = _27.1;
(*_13) = [18919_i16,(-9707_i16),25330_i16,30895_i16];
_32 = [71540822503491971872051521284577705295_i128,101933132600193414166447321088641240726_i128,(-164834652500525282992303749690762549535_i128),(-122897163902656639420181928456477885334_i128)];
(*_7) = _27.1;
(*_3) = [11139_i16,(-15064_i16),13599_i16,11879_i16];
Goto(bb15)
}
bb35 = {
(*_13) = _11;
_10 = _12;
(*_13) = _11;
_15 = !256407808980369931734713201234919284702_u128;
(*_13) = [(-30931_i16),30890_i16,28192_i16,(-19724_i16)];
(*_13) = _12;
(*_13) = _10;
_12 = (*_13);
_9 = [(-17782_i16),(-1764_i16),(-16030_i16),(-4578_i16)];
_12 = [(-22678_i16),16194_i16,22094_i16,(-14484_i16)];
_6 = [(-24617_i16),(-24195_i16),(-20523_i16),(-7353_i16)];
_18 = 65_i8 as f64;
(*_13) = [30281_i16,(-29008_i16),4404_i16,26215_i16];
_1 = (*_13);
(*_13) = _4;
(*_13) = [26961_i16,9381_i16,6066_i16,16473_i16];
_3 = core::ptr::addr_of_mut!((*_13));
_17 = -(-125_isize);
_20 = 8006802234788278215_usize ^ 1382458766748075416_usize;
(*_13) = [13151_i16,21628_i16,(-24841_i16),(-4295_i16)];
_18 = 41_i8 as f64;
(*_13) = [7308_i16,22433_i16,26665_i16,29334_i16];
_6 = [11182_i16,(-14381_i16),6890_i16,(-508_i16)];
Goto(bb3)
}
bb36 = {
(*_3) = [14658_i16,(-11981_i16),14595_i16,(-21785_i16)];
(*_3) = [(-32557_i16),23908_i16,(-28213_i16),(-5528_i16)];
_28 = [14130136189126302378_u64,5670149390990284456_u64,1433955076644364203_u64,13391128242843679489_u64,13435683884553623468_u64,10469027735538725536_u64,5611524238045871391_u64];
(*_13) = _9;
_20 = !11767889240394134075_usize;
(*_13) = [(-11208_i16),7330_i16,(-17825_i16),26735_i16];
(*_13) = [(-30533_i16),7372_i16,(-6962_i16),(-18670_i16)];
(*_3) = (*_13);
_25 = 102530078427259025931500155102969107356_i128 as f64;
(*_3) = _6;
_9 = [9475_i16,14737_i16,(-6996_i16),(-6839_i16)];
_15 = 50080237798866085969503480290255658266_u128 << _22;
_20 = 5_usize & 8810979135705182397_usize;
Goto(bb7)
}
bb37 = {
_67 = _44 >> _61;
(*_13) = [_64.fld0.1.2,_50.1.2,_64.fld0.1.2,_50.1.2];
(*_3) = [_50.1.2,_50.1.2,_64.fld0.1.2,_64.fld0.1.2];
_74 = !_55;
RET = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_50.2, 0), 1)));
_83 = _67 | _57;
(*_7) = _27.1;
(*_3) = [_64.fld0.1.2,_50.1.2,_50.1.2,_50.1.2];
(*_73) = 15854105202221847756_usize & 3_usize;
(*RET) = 4225213309_u32 - 1684656086_u32;
_64.fld0.1.1 = core::ptr::addr_of_mut!((*_7));
_21 = &_18;
_50.1 = (_64.fld0.1.0, Move(_66), _64.fld0.1.2, (*_3));
_17 = _49 as isize;
_33 = !202_u8;
(*_7) = _27.1;
_70 = 15887225745913467267_u64;
_81 = !_26;
_21 = &_38;
_29 = [(*RET),(*RET),(*RET),(*RET),(*RET),(*RET),(*RET),(*RET)];
_61 = !_17;
(*_13) = [_64.fld0.1.2,_64.fld0.1.2,_50.1.2,_64.fld0.1.2];
(*RET) = _67 as u32;
(*RET) = 2038161011_u32 - 2729672056_u32;
(*_3) = [_50.1.2,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
(*_73) = 2_usize << _83;
_64.fld0.1.3 = _1;
_64.fld0.0 = [_50.1.2,_50.1.2,_64.fld0.1.2,_64.fld0.1.2];
Goto(bb38)
}
bb38 = {
_21 = &_45;
(*_3) = [_50.1.2,_50.1.2,_50.1.2,_64.fld0.1.2];
(*RET) = 3466979089_u32 | 1003953437_u32;
_44 = _26 ^ _67;
(*_7) = _68;
_60 = [(*RET),(*RET),(*RET),(*RET),(*RET),(*RET),(*RET),(*RET)];
_53 = &mut (*_73);
_67 = _44 + _44;
(*_7) = _71;
_70 = _15 as u64;
_1 = [_64.fld0.1.2,_64.fld0.1.2,_50.1.2,_64.fld0.1.2];
_28 = [_70,_70,_70,_70,_70,_70,_70];
(*_53) = 1719651310098375682_usize;
_22 = -_44;
_64.fld0.1.0 = [52093_u16,53149_u16,26740_u16,55319_u16,17345_u16];
_61 = _67 * _26;
_63 = (*_7);
(*RET) = 1718378828_u32 - 919501793_u32;
_61 = _22 << _22;
_9 = [_50.1.2,_50.1.2,_64.fld0.1.2,_50.1.2];
(*RET) = 1115204057_u32;
_27.1 = (*_7);
(*_3) = _5;
_64.fld1 = [_33,_33,_33,_33];
(*RET) = 21163509_u32 << _61;
(*_53) = 6_usize;
(*_3) = [_64.fld0.1.2,_64.fld0.1.2,_50.1.2,_50.1.2];
(*RET) = 1216055258_u32 >> _61;
(*_13) = [_50.1.2,_50.1.2,_64.fld0.1.2,_64.fld0.1.2];
Goto(bb39)
}
bb39 = {
_24 = (*RET) >= (*RET);
_50.1.0 = [12683_u16,50203_u16,43473_u16,11995_u16,31425_u16];
(*_7) = _68;
_19 = _63;
_16 = &_50.1;
_11 = [_50.1.2,(*_16).2,(*_16).2,(*_16).2];
(*RET) = !955096420_u32;
(*_53) = !0_usize;
(*_3) = [(*_16).2,(*_16).2,(*_16).2,_50.1.2];
_90 = (*_21) * (*_21);
_34 = core::ptr::addr_of_mut!((*RET));
_59 = (*_16).2;
(*_34) = 29394574_u32;
_87 = [_70,_70,_70,_70,_70,_70,_70];
_89 = ((*_13), Move(_50.1), Move(_50.2));
_89.1.3 = [_89.1.2,_64.fld0.1.2,_64.fld0.1.2,_89.1.2];
_58 = [_19,_71,(*_7),(*_7),(*_7),_68,(*_7),(*_7)];
Goto(bb40)
}
bb40 = {
_43 = !_61;
_6 = [_89.1.2,_64.fld0.1.2,_64.fld0.1.2,_89.1.2];
(*RET) = _14 as u32;
_61 = _24 as isize;
(*_3) = [_64.fld0.1.2,_59,_59,_59];
_64.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_89.2, 0), 0)));
_50.1 = Move(_64.fld0.1);
(*RET) = _33 as u32;
_64.fld0.1.0 = [29440_u16,9519_u16,56339_u16,60246_u16,9193_u16];
(*RET) = Field::<u32>(Variant(_89.2, 0), 1) - Field::<u32>(Variant(_89.2, 0), 1);
(*_13) = [_89.1.2,_59,_50.1.2,_59];
_89.0 = [_89.1.2,_59,_50.1.2,_59];
_74 = _26 > _81;
(*RET) = Field::<u32>(Variant(_89.2, 0), 1);
_50 = (_11, Move(_89.1), Move(_89.2));
_90 = (*_21) + (*_21);
(*_7) = _27.1;
_72 = &_50.1;
_78 = !Field::<u32>(Variant(_50.2, 0), 1);
_73 = Move(_53);
_82 = (-119174858650203410754253098928657797639_i128) as u64;
(*_13) = [(*_72).2,(*_72).2,(*_72).2,(*_72).2];
Goto(bb41)
}
bb41 = {
_32 = [24619849618348623174601232583690256372_i128,161576082852171851517539347009611634411_i128,(-41781789348539628372377085054507564175_i128),101940335387296208581556619937556513447_i128];
_64.fld0 = (_6, Move(_50.1), Move(_50.2));
_34 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_64.fld0.2, 0), 1)));
(*_3) = (*_13);
(*_7) = _27.1;
(*_13) = [_59,_64.fld0.1.2,_64.fld0.1.2,_64.fld0.1.2];
_71 = (*_7);
_33 = 206_u8 >> _61;
_9 = [_59,_64.fld0.1.2,_59,_59];
(*_3) = [_64.fld0.1.2,_59,_59,_59];
(*_34) = !_78;
Goto(bb42)
}
bb42 = {
(*_13) = _8;
_31 = !_24;
_43 = (*_21) as isize;
Goto(bb43)
}
bb43 = {
_28 = [_70,_70,_82,_70,_70,_70,_70];
_64.fld0.1.2 = !_59;
Goto(bb44)
}
bb44 = {
(*_7) = _63;
_92 = !_31;
_27.2 = core::ptr::addr_of_mut!((*_7));
(*_3) = [_64.fld0.1.2,_59,_64.fld0.1.2,_59];
(*_3) = _12;
_17 = _61;
_52 = -_67;
_11 = [_64.fld0.1.2,_59,_59,_59];
(*_34) = _78 * _78;
(*_3) = [_64.fld0.1.2,_59,_64.fld0.1.2,_59];
_89.1.1 = core::ptr::addr_of_mut!((*_7));
(*_34) = !_78;
_67 = -_61;
(*_13) = (*_3);
_50.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant(_64.fld0.2, 0), 0),fld1: (*_34) };
(*_13) = [_59,_59,_59,_64.fld0.1.2];
Goto(bb45)
}
bb45 = {
_4 = [_64.fld0.1.2,_59,_59,_59];
_14 = (*_34) as f32;
(*_34) = Field::<u32>(Variant(_50.2, 0), 1) - Field::<u32>(Variant(_50.2, 0), 1);
_95 = [(-1868643407_i32)];
(*_7) = _71;
_27.0 = &_64.fld0.2;
_75 = &_64.fld0.1;
(*_13) = (*_75).3;
_50.1 = ((*_75).0, Move(_89.1.1), (*_75).2, _10);
(*_34) = _78 ^ Field::<u32>(Variant(_50.2, 0), 1);
_62 = _33 as f64;
Goto(bb46)
}
bb46 = {
_43 = 45915_u16 as isize;
(*_7) = _63;
_64.fld0.2 = Move(_50.2);
_72 = &(*_75);
_64.fld0.1.3 = [(*_72).2,(*_72).2,(*_72).2,(*_75).2];
_75 = &_50.1;
_11 = [(*_75).2,(*_72).2,(*_72).2,_59];
_94 = -_14;
(*_13) = _9;
_49 = 91_i8;
Goto(bb47)
}
bb47 = {
(*_3) = [(*_72).2,(*_72).2,(*_72).2,(*_75).2];
_75 = &_64.fld0.1;
_9 = [_64.fld0.1.2,(*_75).2,(*_75).2,(*_72).2];
_86 = 5413825812529473070_usize;
Goto(bb48)
}
bb48 = {
_64.fld1 = [_33,_33,_33,_33];
_10 = [(*_75).2,(*_75).2,(*_72).2,(*_72).2];
_64.fld0.1.0 = [24484_u16,6868_u16,57510_u16,37440_u16,18582_u16];
(*_7) = _63;
_91 = _50.1.2 ^ (*_75).2;
_100 = (Move(_27.0), (*_7), Move(_64.fld0.1.1));
_100.1 = _68;
_64.fld0.1.2 = _61 as i16;
_24 = _31 ^ _92;
_93.0 = _32;
_81 = _61 * _17;
match _86 {
0 => bb38,
1 => bb49,
2 => bb50,
5413825812529473070 => bb52,
_ => bb51
}
}
bb49 = {
(*_3) = [5423_i16,17731_i16,15049_i16,7764_i16];
_19 = _27.1;
_10 = (*_13);
_29 = [3373241055_u32,2062767479_u32,2574877927_u32,439087010_u32,2522256197_u32,4092047499_u32,3638708421_u32,902445193_u32];
_5 = [(-17698_i16),12146_i16,(-28802_i16),29223_i16];
(*_3) = [18300_i16,(-1124_i16),4829_i16,20454_i16];
_27.2 = core::ptr::addr_of_mut!(_27.1);
(*_3) = [(-2357_i16),7176_i16,(-6817_i16),192_i16];
(*_13) = [22136_i16,5684_i16,(-31514_i16),(-651_i16)];
_21 = &_18;
(*_3) = [(-25136_i16),23670_i16,(-27120_i16),28007_i16];
_33 = 155_u8 >> _17;
_14 = 61_i8 as f32;
(*_13) = [(-18002_i16),(-11965_i16),(-970_i16),(-18566_i16)];
(*_13) = [32437_i16,(-510_i16),792_i16,13857_i16];
(*_13) = [10941_i16,(-23341_i16),1141_i16,(-16289_i16)];
(*_13) = [27502_i16,(-31384_i16),27742_i16,(-31903_i16)];
Goto(bb8)
}
bb50 = {
_43 = !_61;
_6 = [_89.1.2,_64.fld0.1.2,_64.fld0.1.2,_89.1.2];
(*RET) = _14 as u32;
_61 = _24 as isize;
(*_3) = [_64.fld0.1.2,_59,_59,_59];
_64.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_89.2, 0), 0)));
_50.1 = Move(_64.fld0.1);
(*RET) = _33 as u32;
_64.fld0.1.0 = [29440_u16,9519_u16,56339_u16,60246_u16,9193_u16];
(*RET) = Field::<u32>(Variant(_89.2, 0), 1) - Field::<u32>(Variant(_89.2, 0), 1);
(*_13) = [_89.1.2,_59,_50.1.2,_59];
_89.0 = [_89.1.2,_59,_50.1.2,_59];
_74 = _26 > _81;
(*RET) = Field::<u32>(Variant(_89.2, 0), 1);
_50 = (_11, Move(_89.1), Move(_89.2));
_90 = (*_21) + (*_21);
(*_7) = _27.1;
_72 = &_50.1;
_78 = !Field::<u32>(Variant(_50.2, 0), 1);
_73 = Move(_53);
_82 = (-119174858650203410754253098928657797639_i128) as u64;
(*_13) = [(*_72).2,(*_72).2,(*_72).2,(*_72).2];
Goto(bb41)
}
bb51 = {
_4 = [_64.fld0.1.2,_59,_59,_59];
_14 = (*_34) as f32;
(*_34) = Field::<u32>(Variant(_50.2, 0), 1) - Field::<u32>(Variant(_50.2, 0), 1);
_95 = [(-1868643407_i32)];
(*_7) = _71;
_27.0 = &_64.fld0.2;
_75 = &_64.fld0.1;
(*_13) = (*_75).3;
_50.1 = ((*_75).0, Move(_89.1.1), (*_75).2, _10);
(*_34) = _78 ^ Field::<u32>(Variant(_50.2, 0), 1);
_62 = _33 as f64;
Goto(bb46)
}
bb52 = {
_78 = Field::<u32>(Variant(_64.fld0.2, 0), 1);
_85 = &mut _86;
_53 = &mut (*_85);
(*_7) = _71;
(*_3) = [_64.fld0.1.2,_91,_64.fld0.1.2,_64.fld0.1.2];
_48 = -_62;
(*_3) = _4;
(*_13) = (*_3);
_64.fld0.1.0 = [55126_u16,49868_u16,29206_u16,15537_u16,20869_u16];
_28 = [_82,_70,_70,_70,_70,_70,_82];
Goto(bb53)
}
bb53 = {
Call(_107 = dump_var(Move(_57), Move(_63), Move(_17), Move(_47)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_107 = dump_var(Move(_43), Move(_59), Move(_26), Move(_20)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_107 = dump_var(Move(_68), Move(_44), Move(_1), Move(_23)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_107 = dump_var(Move(_22), Move(_91), Move(_60), Move(_49)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_107 = dump_var(Move(_9), Move(_29), Move(_24), Move(_55)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_107 = dump_var(Move(_2), Move(_41), Move(_81), Move(_11)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_107 = dump_var(Move(_74), Move(_6), _108, _108), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i16; 4],mut _2: *mut char,mut _3: char,mut _4: char,mut _5: char,mut _6: [i16; 4],mut _7: *mut [i16; 4],mut _8: char,mut _9: [i16; 4]) -> f64 {
mir! {
type RET = f64;
let _10: bool;
let _11: f32;
let _12: char;
let _13: *const [usize; 3];
let _14: bool;
let _15: i16;
let _16: *const f64;
let _17: isize;
let _18: f32;
let _19: isize;
let _20: (isize, u8, i128, i64);
let _21: f64;
let _22: f64;
let _23: [isize; 8];
let _24: f32;
let _25: &'static mut u16;
let _26: &'static &'static (i32, (usize, u128, f32), f64);
let _27: &'static (i32, (usize, u128, f32), f64);
let _28: bool;
let _29: isize;
let _30: u16;
let _31: (&'static (i32, (usize, u128, f32), f64), f32);
let _32: &'static Adt25;
let _33: isize;
let _34: char;
let _35: *mut f64;
let _36: [char; 8];
let _37: isize;
let _38: f32;
let _39: *mut f64;
let _40: [u8; 2];
let _41: isize;
let _42: u16;
let _43: *mut &'static Adt25;
let _44: f64;
let _45: bool;
let _46: Adt68;
let _47: &'static [i128; 4];
let _48: u128;
let _49: i32;
let _50: ([i32; 1], &'static mut usize, Adt24);
let _51: (&'static Adt25, char, *mut char);
let _52: *const isize;
let _53: &'static &'static (i32, (usize, u128, f32), f64);
let _54: isize;
let _55: *const f64;
let _56: isize;
let _57: u128;
let _58: *mut &'static Adt25;
let _59: Adt62;
let _60: [i32; 1];
let _61: &'static &'static (i32, (usize, u128, f32), f64);
let _62: f64;
let _63: isize;
let _64: &'static i128;
let _65: f64;
let _66: [i32; 4];
let _67: &'static i128;
let _68: i16;
let _69: *const [usize; 3];
let _70: (i32, (usize, u128, f32), f64);
let _71: *mut &'static Adt25;
let _72: isize;
let _73: isize;
let _74: [u8; 5];
let _75: char;
let _76: &'static u8;
let _77: isize;
let _78: u16;
let _79: i32;
let _80: *mut u32;
let _81: i128;
let _82: i64;
let _83: f64;
let _84: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _85: *mut [i16; 4];
let _86: (i32, (usize, u128, f32), f64);
let _87: *const isize;
let _88: &'static mut Adt42;
let _89: (&'static (i32, (usize, u128, f32), f64), f32);
let _90: isize;
let _91: isize;
let _92: *const isize;
let _93: &'static [char; 4];
let _94: bool;
let _95: ();
let _96: ();
{
_3 = _5;
RET = 39097_u16 as f64;
_7 = core::ptr::addr_of_mut!(_1);
_4 = _8;
_9 = [(-27211_i16),(-23000_i16),(-5941_i16),(-20349_i16)];
(*_7) = _9;
_1 = [9047_i16,(-20464_i16),7625_i16,(-20244_i16)];
(*_7) = _6;
(*_7) = [(-4737_i16),(-11550_i16),3925_i16,(-21661_i16)];
_8 = _5;
(*_7) = _9;
_2 = core::ptr::addr_of_mut!(_8);
(*_7) = _9;
(*_2) = _5;
RET = (-1_isize) as f64;
_4 = (*_2);
_7 = core::ptr::addr_of_mut!((*_7));
(*_2) = _3;
Goto(bb1)
}
bb1 = {
_6 = [(-25605_i16),21909_i16,2286_i16,21035_i16];
(*_7) = _6;
(*_2) = _5;
(*_2) = _3;
(*_7) = [(-1542_i16),24272_i16,(-28563_i16),23188_i16];
(*_2) = _5;
_5 = (*_2);
_6 = [4648_i16,(-27096_i16),(-16386_i16),6200_i16];
_10 = true;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _4;
_12 = (*_2);
(*_2) = _4;
(*_2) = _4;
Goto(bb2)
}
bb2 = {
(*_7) = [20501_i16,19236_i16,24459_i16,30045_i16];
(*_7) = [(-31046_i16),19858_i16,(-31102_i16),9745_i16];
(*_7) = [(-18181_i16),(-25403_i16),(-22701_i16),(-8663_i16)];
(*_2) = _5;
_18 = 1516780765_u32 as f32;
(*_2) = _5;
(*_2) = _5;
(*_7) = [13741_i16,(-30269_i16),(-165_i16),(-22107_i16)];
(*_7) = _9;
(*_2) = _4;
(*_2) = _5;
(*_7) = [(-30631_i16),(-23441_i16),26086_i16,(-7552_i16)];
(*_7) = _6;
_5 = _3;
(*_2) = _4;
(*_7) = [14690_i16,(-17186_i16),(-19848_i16),(-1129_i16)];
(*_2) = _5;
(*_7) = [19484_i16,6456_i16,(-13247_i16),(-21516_i16)];
_12 = _3;
(*_2) = _3;
Goto(bb3)
}
bb3 = {
_8 = _5;
(*_7) = [5916_i16,26666_i16,9045_i16,(-30346_i16)];
(*_2) = _5;
(*_7) = [(-479_i16),26891_i16,27941_i16,(-30347_i16)];
(*_7) = [(-6874_i16),(-22833_i16),12662_i16,4633_i16];
Goto(bb4)
}
bb4 = {
(*_7) = [(-21071_i16),(-13209_i16),18841_i16,(-3604_i16)];
(*_7) = [6331_i16,(-3672_i16),26802_i16,(-20130_i16)];
(*_2) = _12;
(*_2) = _3;
_8 = _5;
_18 = 8_i8 as f32;
Goto(bb5)
}
bb5 = {
(*_7) = [(-31504_i16),(-17592_i16),(-28392_i16),23921_i16];
_14 = _10;
(*_2) = _12;
Goto(bb6)
}
bb6 = {
(*_7) = _9;
_9 = [5538_i16,16932_i16,17835_i16,(-23649_i16)];
(*_2) = _3;
(*_7) = [14309_i16,26147_i16,31038_i16,(-6618_i16)];
_6 = (*_7);
_10 = (*_2) == (*_2);
_6 = [(-1661_i16),(-19525_i16),19465_i16,(-20392_i16)];
(*_2) = _4;
_3 = (*_2);
(*_2) = _4;
(*_2) = _12;
(*_2) = _12;
Goto(bb7)
}
bb7 = {
RET = (-1644148156_i32) as f64;
_20 = (9223372036854775807_isize, 61_u8, 4682310127538735198724811408345862738_i128, 791570465939433460_i64);
(*_7) = _6;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _12;
Goto(bb8)
}
bb8 = {
_18 = RET as f32;
(*_7) = [7598_i16,(-27170_i16),25462_i16,(-22878_i16)];
RET = _20.0 as f64;
_23 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_11 = -_18;
_15 = _20.1 as i16;
_22 = RET + RET;
(*_2) = _12;
(*_7) = _6;
_16 = core::ptr::addr_of!(_22);
(*_7) = _6;
(*_16) = _11 as f64;
(*_7) = _9;
(*_7) = [_15,_15,_15,_15];
(*_2) = _5;
_20.3 = -(-9020384678877454938_i64);
match _20.2 {
0 => bb5,
1 => bb2,
4682310127538735198724811408345862738 => bb10,
_ => bb9
}
}
bb9 = {
_8 = _5;
(*_7) = [5916_i16,26666_i16,9045_i16,(-30346_i16)];
(*_2) = _5;
(*_7) = [(-479_i16),26891_i16,27941_i16,(-30347_i16)];
(*_7) = [(-6874_i16),(-22833_i16),12662_i16,4633_i16];
Goto(bb4)
}
bb10 = {
(*_2) = _5;
(*_16) = RET * RET;
RET = 13559363756355212381_u64 as f64;
(*_2) = _3;
(*_7) = [_15,_15,_15,_15];
(*_16) = RET + RET;
_21 = (*_16);
(*_2) = _5;
(*_7) = _9;
(*_16) = _21 * _21;
(*_7) = [_15,_15,_15,_15];
_20.3 = 8889993108151474442_i64 ^ (-7274769373169187383_i64);
(*_7) = [_15,_15,_15,_15];
(*_7) = _9;
_2 = core::ptr::addr_of_mut!(_5);
_2 = core::ptr::addr_of_mut!((*_2));
Goto(bb11)
}
bb11 = {
_19 = _20.0;
(*_7) = _6;
_21 = (*_16);
_20 = (_19, 196_u8, 73389032990637857524004529411297643187_i128, (-8092520715524269590_i64));
(*_2) = _3;
match _20.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb12,
5 => bb13,
6 => bb14,
196 => bb16,
_ => bb15
}
}
bb12 = {
_6 = [(-25605_i16),21909_i16,2286_i16,21035_i16];
(*_7) = _6;
(*_2) = _5;
(*_2) = _3;
(*_7) = [(-1542_i16),24272_i16,(-28563_i16),23188_i16];
(*_2) = _5;
_5 = (*_2);
_6 = [4648_i16,(-27096_i16),(-16386_i16),6200_i16];
_10 = true;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _4;
_12 = (*_2);
(*_2) = _4;
(*_2) = _4;
Goto(bb2)
}
bb13 = {
_8 = _5;
(*_7) = [5916_i16,26666_i16,9045_i16,(-30346_i16)];
(*_2) = _5;
(*_7) = [(-479_i16),26891_i16,27941_i16,(-30347_i16)];
(*_7) = [(-6874_i16),(-22833_i16),12662_i16,4633_i16];
Goto(bb4)
}
bb14 = {
(*_7) = [20501_i16,19236_i16,24459_i16,30045_i16];
(*_7) = [(-31046_i16),19858_i16,(-31102_i16),9745_i16];
(*_7) = [(-18181_i16),(-25403_i16),(-22701_i16),(-8663_i16)];
(*_2) = _5;
_18 = 1516780765_u32 as f32;
(*_2) = _5;
(*_2) = _5;
(*_7) = [13741_i16,(-30269_i16),(-165_i16),(-22107_i16)];
(*_7) = _9;
(*_2) = _4;
(*_2) = _5;
(*_7) = [(-30631_i16),(-23441_i16),26086_i16,(-7552_i16)];
(*_7) = _6;
_5 = _3;
(*_2) = _4;
(*_7) = [14690_i16,(-17186_i16),(-19848_i16),(-1129_i16)];
(*_2) = _5;
(*_7) = [19484_i16,6456_i16,(-13247_i16),(-21516_i16)];
_12 = _3;
(*_2) = _3;
Goto(bb3)
}
bb15 = {
RET = (-1644148156_i32) as f64;
_20 = (9223372036854775807_isize, 61_u8, 4682310127538735198724811408345862738_i128, 791570465939433460_i64);
(*_7) = _6;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _12;
Goto(bb8)
}
bb16 = {
(*_7) = [_15,_15,_15,_15];
(*_16) = _20.0 as f64;
_22 = _21;
(*_2) = _8;
_15 = !(-14350_i16);
Goto(bb17)
}
bb17 = {
_3 = (*_2);
_18 = _11;
Call(_33 = fn8(_21, Move(_2), (*_2), Move(_16), _20.2, (*_2), (*_2), (*_2), (*_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_1 = [_15,_15,_15,_15];
_20 = (_19, 160_u8, 6183580206544564464500765759854559877_i128, (-8748569869844816314_i64));
_10 = _14 | _14;
_36 = [_8,_12,_4,_5,_12,_3,_4,_8];
Goto(bb19)
}
bb19 = {
_34 = _5;
_35 = core::ptr::addr_of_mut!(_22);
(*_35) = RET + _21;
_35 = core::ptr::addr_of_mut!((*_35));
_20.3 = 2278325773092310305_i64;
_5 = _3;
(*_7) = [_15,_15,_15,_15];
_14 = (*_35) > (*_35);
(*_7) = _9;
_31.1 = _18 * _18;
_11 = _19 as f32;
_20.3 = 1309429763427379502_i64 * 1557751642973449626_i64;
_10 = _14 ^ _14;
_8 = _5;
(*_35) = _21 + _21;
_10 = _14;
(*_7) = [_15,_15,_15,_15];
_42 = 27450_u16 - 51302_u16;
_11 = _31.1 * _31.1;
_7 = core::ptr::addr_of_mut!((*_7));
(*_35) = _21 * _21;
(*_7) = _9;
_21 = _15 as f64;
RET = -(*_35);
(*_35) = -RET;
_28 = _14 | _14;
Goto(bb20)
}
bb20 = {
_20.3 = (-6672344990858401803_i64) ^ (-4928915653066827063_i64);
(*_35) = _21;
_4 = _8;
_10 = !_28;
_20 = (_19, 77_u8, 28665710202142011194387902953921281990_i128, 1445125775337048263_i64);
(*_7) = [_15,_15,_15,_15];
_1 = [_15,_15,_15,_15];
(*_7) = [_15,_15,_15,_15];
_46.fld2 = core::ptr::addr_of!(RET);
_1 = [_15,_15,_15,_15];
_18 = 47980820715274126456484813576894178451_u128 as f32;
(*_7) = [_15,_15,_15,_15];
_37 = _20.2 as isize;
_34 = _12;
_10 = !_28;
_44 = (-108_i8) as f64;
_20.1 = 218_u8 + 10_u8;
(*_7) = _9;
(*_7) = [_15,_15,_15,_15];
match _20.3 {
0 => bb18,
1 => bb10,
2 => bb3,
3 => bb6,
4 => bb16,
1445125775337048263 => bb22,
_ => bb21
}
}
bb21 = {
_34 = _5;
_35 = core::ptr::addr_of_mut!(_22);
(*_35) = RET + _21;
_35 = core::ptr::addr_of_mut!((*_35));
_20.3 = 2278325773092310305_i64;
_5 = _3;
(*_7) = [_15,_15,_15,_15];
_14 = (*_35) > (*_35);
(*_7) = _9;
_31.1 = _18 * _18;
_11 = _19 as f32;
_20.3 = 1309429763427379502_i64 * 1557751642973449626_i64;
_10 = _14 ^ _14;
_8 = _5;
(*_35) = _21 + _21;
_10 = _14;
(*_7) = [_15,_15,_15,_15];
_42 = 27450_u16 - 51302_u16;
_11 = _31.1 * _31.1;
_7 = core::ptr::addr_of_mut!((*_7));
(*_35) = _21 * _21;
(*_7) = _9;
_21 = _15 as f64;
RET = -(*_35);
(*_35) = -RET;
_28 = _14 | _14;
Goto(bb20)
}
bb22 = {
(*_35) = -_21;
_15 = 94292962481138884216141270735988012380_u128 as i16;
_5 = _34;
_8 = _12;
(*_7) = [_15,_15,_15,_15];
_46.fld0.1.3 = [_15,_15,_15,_15];
(*_35) = RET - RET;
_46.fld0.2 = Adt25::Variant0 { fld0: (*_35),fld1: 542140280_u32 };
(*_7) = _9;
_20.0 = -_37;
_4 = _12;
_6 = [_15,_15,_15,_15];
(*_35) = -Field::<f64>(Variant(_46.fld0.2, 0), 0);
Goto(bb23)
}
bb23 = {
_46.fld0.1.3 = [_15,_15,_15,_15];
(*_7) = [_15,_15,_15,_15];
_40 = [_20.1,_20.1];
(*_7) = [_15,_15,_15,_15];
_2 = core::ptr::addr_of_mut!(_12);
_54 = 126_i8 as isize;
_3 = (*_2);
_38 = _18 + _11;
place!(Field::<f64>(Variant(_46.fld0.2, 0), 0)) = 53545558922321475148134768830681084327_u128 as f64;
_25 = &mut _42;
(*_2) = _4;
(*_7) = _9;
_46.fld2 = core::ptr::addr_of!((*_35));
_31.1 = _38;
_41 = _37 * _54;
_39 = core::ptr::addr_of_mut!(_44);
Goto(bb24)
}
bb24 = {
_46.fld1 = [_20.1,_20.1,_20.1,_20.1];
(*_2) = _34;
_46.fld0.1.0 = [(*_25),(*_25),(*_25),(*_25),(*_25)];
(*_25) = 46933_u16;
place!(Field::<f64>(Variant(_46.fld0.2, 0), 0)) = (*_35);
Goto(bb25)
}
bb25 = {
(*_2) = _4;
_7 = core::ptr::addr_of_mut!((*_7));
_46.fld0.1.2 = _15 * _15;
(*_2) = _34;
_46.fld0.1.1 = core::ptr::addr_of_mut!(_51.1);
_20.1 = 125_u8 << _41;
_46.fld0.0 = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
_5 = _12;
(*_7) = [_46.fld0.1.2,_15,_15,_46.fld0.1.2];
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
(*_2) = _8;
(*_25) = 4337794350343163436_u64 as u16;
_18 = _20.3 as f32;
_5 = _8;
_55 = core::ptr::addr_of!((*_39));
(*_2) = _3;
(*_55) = (*_35) + (*_35);
_22 = (*_55) * (*_39);
_16 = core::ptr::addr_of!((*_35));
(*_35) = (-730747747_i32) as f64;
(*_2) = _8;
(*_55) = -RET;
_28 = !_10;
(*_39) = (*_35);
_62 = _15 as f64;
place!(Field::<u32>(Variant(_46.fld0.2, 0), 1)) = 3252324869_u32 - 3912921325_u32;
match _20.2 {
28665710202142011194387902953921281990 => bb26,
_ => bb2
}
}
bb26 = {
(*_35) = -RET;
_20.2 = 152106334280150427154829141632433102275_i128 | (-104135842452541348745632678350631052317_i128);
(*_35) = -(*_39);
_20 = (_37, 27_u8, 102586884008701113716132727587114284387_i128, 3438219557936001408_i64);
(*_39) = (*_35);
_11 = (*_39) as f32;
(*_39) = -RET;
(*_2) = _8;
_62 = _20.3 as f64;
(*_7) = _6;
(*_2) = _8;
(*_35) = (*_39);
_51.0 = &_46.fld0.2;
_20.1 = 51_u8 - 6_u8;
(*_39) = (*_35) + (*_35);
(*_35) = _18 as f64;
Goto(bb27)
}
bb27 = {
(*_35) = -(*_39);
(*_39) = (*_35) * (*_35);
_28 = !_10;
(*_39) = (*_35) * _22;
_9 = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
_12 = _8;
_35 = core::ptr::addr_of_mut!(_70.2);
_34 = (*_2);
(*_2) = _3;
_57 = 287435510859057980148060873066885940070_u128;
(*_7) = [_15,_46.fld0.1.2,_15,_46.fld0.1.2];
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
_35 = core::ptr::addr_of_mut!((*_39));
(*_25) = 44824_u16;
(*_35) = _20.3 as f64;
_60 = [1743587374_i32];
(*_39) = 931031238_i32 as f64;
_49 = (-1288097319_i32) * (-327931567_i32);
_45 = !_28;
(*_25) = !14611_u16;
_58 = core::ptr::addr_of_mut!(_51.0);
_71 = core::ptr::addr_of_mut!((*_58));
(*_2) = _8;
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_15,_15];
_65 = -_22;
_20.2 = (*_39) as i128;
(*_25) = 44657_u16 << _20.1;
match _20.3 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
3438219557936001408 => bb34,
_ => bb33
}
}
bb28 = {
(*_35) = -RET;
_20.2 = 152106334280150427154829141632433102275_i128 | (-104135842452541348745632678350631052317_i128);
(*_35) = -(*_39);
_20 = (_37, 27_u8, 102586884008701113716132727587114284387_i128, 3438219557936001408_i64);
(*_39) = (*_35);
_11 = (*_39) as f32;
(*_39) = -RET;
(*_2) = _8;
_62 = _20.3 as f64;
(*_7) = _6;
(*_2) = _8;
(*_35) = (*_39);
_51.0 = &_46.fld0.2;
_20.1 = 51_u8 - 6_u8;
(*_39) = (*_35) + (*_35);
(*_35) = _18 as f64;
Goto(bb27)
}
bb29 = {
(*_2) = _4;
_7 = core::ptr::addr_of_mut!((*_7));
_46.fld0.1.2 = _15 * _15;
(*_2) = _34;
_46.fld0.1.1 = core::ptr::addr_of_mut!(_51.1);
_20.1 = 125_u8 << _41;
_46.fld0.0 = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
_5 = _12;
(*_7) = [_46.fld0.1.2,_15,_15,_46.fld0.1.2];
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2];
(*_2) = _8;
(*_25) = 4337794350343163436_u64 as u16;
_18 = _20.3 as f32;
_5 = _8;
_55 = core::ptr::addr_of!((*_39));
(*_2) = _3;
(*_55) = (*_35) + (*_35);
_22 = (*_55) * (*_39);
_16 = core::ptr::addr_of!((*_35));
(*_35) = (-730747747_i32) as f64;
(*_2) = _8;
(*_55) = -RET;
_28 = !_10;
(*_39) = (*_35);
_62 = _15 as f64;
place!(Field::<u32>(Variant(_46.fld0.2, 0), 1)) = 3252324869_u32 - 3912921325_u32;
match _20.2 {
28665710202142011194387902953921281990 => bb26,
_ => bb2
}
}
bb30 = {
(*_7) = [(-31504_i16),(-17592_i16),(-28392_i16),23921_i16];
_14 = _10;
(*_2) = _12;
Goto(bb6)
}
bb31 = {
_6 = [(-25605_i16),21909_i16,2286_i16,21035_i16];
(*_7) = _6;
(*_2) = _5;
(*_2) = _3;
(*_7) = [(-1542_i16),24272_i16,(-28563_i16),23188_i16];
(*_2) = _5;
_5 = (*_2);
_6 = [4648_i16,(-27096_i16),(-16386_i16),6200_i16];
_10 = true;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _4;
_12 = (*_2);
(*_2) = _4;
(*_2) = _4;
Goto(bb2)
}
bb32 = {
(*_7) = [20501_i16,19236_i16,24459_i16,30045_i16];
(*_7) = [(-31046_i16),19858_i16,(-31102_i16),9745_i16];
(*_7) = [(-18181_i16),(-25403_i16),(-22701_i16),(-8663_i16)];
(*_2) = _5;
_18 = 1516780765_u32 as f32;
(*_2) = _5;
(*_2) = _5;
(*_7) = [13741_i16,(-30269_i16),(-165_i16),(-22107_i16)];
(*_7) = _9;
(*_2) = _4;
(*_2) = _5;
(*_7) = [(-30631_i16),(-23441_i16),26086_i16,(-7552_i16)];
(*_7) = _6;
_5 = _3;
(*_2) = _4;
(*_7) = [14690_i16,(-17186_i16),(-19848_i16),(-1129_i16)];
(*_2) = _5;
(*_7) = [19484_i16,6456_i16,(-13247_i16),(-21516_i16)];
_12 = _3;
(*_2) = _3;
Goto(bb3)
}
bb33 = {
(*_7) = [(-21071_i16),(-13209_i16),18841_i16,(-3604_i16)];
(*_7) = [6331_i16,(-3672_i16),26802_i16,(-20130_i16)];
(*_2) = _12;
(*_2) = _3;
_8 = _5;
_18 = 8_i8 as f32;
Goto(bb5)
}
bb34 = {
_4 = (*_2);
_64 = &_20.2;
_70.1.0 = 7692387899241580599_usize + 2_usize;
(*_25) = 31149_u16 | 63973_u16;
(*_7) = _46.fld0.1.3;
(*_2) = _3;
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_46.fld0.1.2,_15];
_32 = &_46.fld0.2;
match _19 {
0 => bb1,
1 => bb17,
2 => bb35,
9223372036854775807 => bb37,
_ => bb36
}
}
bb35 = {
(*_7) = [(-31504_i16),(-17592_i16),(-28392_i16),23921_i16];
_14 = _10;
(*_2) = _12;
Goto(bb6)
}
bb36 = {
(*_35) = -RET;
_20.2 = 152106334280150427154829141632433102275_i128 | (-104135842452541348745632678350631052317_i128);
(*_35) = -(*_39);
_20 = (_37, 27_u8, 102586884008701113716132727587114284387_i128, 3438219557936001408_i64);
(*_39) = (*_35);
_11 = (*_39) as f32;
(*_39) = -RET;
(*_2) = _8;
_62 = _20.3 as f64;
(*_7) = _6;
(*_2) = _8;
(*_35) = (*_39);
_51.0 = &_46.fld0.2;
_20.1 = 51_u8 - 6_u8;
(*_39) = (*_35) + (*_35);
(*_35) = _18 as f64;
Goto(bb27)
}
bb37 = {
(*_2) = _8;
_51 = (Move(_32), (*_2), Move(_2));
_60 = [_49];
_29 = _37 >> _20.0;
(*_25) = !39607_u16;
_70.1.2 = -_38;
_68 = _15 * _15;
Goto(bb38)
}
bb38 = {
_68 = -_46.fld0.1.2;
(*_25) = _46.fld0.1.2 as u16;
(*_58) = &_46.fld0.2;
_52 = core::ptr::addr_of!(_63);
_10 = _14;
_51.2 = core::ptr::addr_of_mut!(_75);
_66 = [_49,_49,_49,_49];
_56 = _19 & _37;
(*_52) = _41 & _41;
(*_7) = _46.fld0.1.3;
(*_39) = -_22;
_32 = &_46.fld0.2;
_70.1 = (17796269553530751539_usize, _57, _31.1);
_72 = Field::<u32>(Variant((*_32), 0), 1) as isize;
_24 = _38;
_17 = (*_52);
(*_52) = !_29;
(*_7) = _9;
(*_39) = Field::<f64>(Variant((*_32), 0), 0) * Field::<f64>(Variant((*_32), 0), 0);
(*_39) = Field::<f64>(Variant((*_32), 0), 0);
_43 = Move(_71);
(*_39) = Field::<f64>(Variant((*_32), 0), 0) - Field::<f64>(Variant((*_32), 0), 0);
(*_25) = 21402_u16 >> (*_52);
(*_52) = _72;
(*_58) = Move(_32);
(*_39) = _22 + _62;
(*_25) = 31901_u16;
Goto(bb39)
}
bb39 = {
(*_39) = Field::<f64>(Variant(_46.fld0.2, 0), 0) + _65;
(*_58) = &_46.fld0.2;
Goto(bb40)
}
bb40 = {
(*_7) = _9;
_33 = _14 as isize;
_46.fld2 = core::ptr::addr_of!((*_39));
(*_7) = [_68,_68,_68,_46.fld0.1.2];
_44 = -Field::<f64>(Variant(_46.fld0.2, 0), 0);
_32 = &_46.fld0.2;
(*_58) = Move(_32);
_73 = _49 as isize;
_60 = [_49];
_80 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_46.fld0.2, 0), 1)));
(*_39) = (-123_i8) as f64;
place!(Field::<u32>(Variant(_46.fld0.2, 0), 1)) = 186238747_u32;
_15 = _12 as i16;
_11 = _38 + _24;
match _20.3 {
0 => bb41,
1 => bb42,
2 => bb43,
3 => bb44,
3438219557936001408 => bb46,
_ => bb45
}
}
bb41 = {
_6 = [(-25605_i16),21909_i16,2286_i16,21035_i16];
(*_7) = _6;
(*_2) = _5;
(*_2) = _3;
(*_7) = [(-1542_i16),24272_i16,(-28563_i16),23188_i16];
(*_2) = _5;
_5 = (*_2);
_6 = [4648_i16,(-27096_i16),(-16386_i16),6200_i16];
_10 = true;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _5;
(*_2) = _4;
_12 = (*_2);
(*_2) = _4;
(*_2) = _4;
Goto(bb2)
}
bb42 = {
(*_2) = _5;
(*_16) = RET * RET;
RET = 13559363756355212381_u64 as f64;
(*_2) = _3;
(*_7) = [_15,_15,_15,_15];
(*_16) = RET + RET;
_21 = (*_16);
(*_2) = _5;
(*_7) = _9;
(*_16) = _21 * _21;
(*_7) = [_15,_15,_15,_15];
_20.3 = 8889993108151474442_i64 ^ (-7274769373169187383_i64);
(*_7) = [_15,_15,_15,_15];
(*_7) = _9;
_2 = core::ptr::addr_of_mut!(_5);
_2 = core::ptr::addr_of_mut!((*_2));
Goto(bb11)
}
bb43 = {
(*_7) = [20501_i16,19236_i16,24459_i16,30045_i16];
(*_7) = [(-31046_i16),19858_i16,(-31102_i16),9745_i16];
(*_7) = [(-18181_i16),(-25403_i16),(-22701_i16),(-8663_i16)];
(*_2) = _5;
_18 = 1516780765_u32 as f32;
(*_2) = _5;
(*_2) = _5;
(*_7) = [13741_i16,(-30269_i16),(-165_i16),(-22107_i16)];
(*_7) = _9;
(*_2) = _4;
(*_2) = _5;
(*_7) = [(-30631_i16),(-23441_i16),26086_i16,(-7552_i16)];
(*_7) = _6;
_5 = _3;
(*_2) = _4;
(*_7) = [14690_i16,(-17186_i16),(-19848_i16),(-1129_i16)];
(*_2) = _5;
(*_7) = [19484_i16,6456_i16,(-13247_i16),(-21516_i16)];
_12 = _3;
(*_2) = _3;
Goto(bb3)
}
bb44 = {
(*_35) = -RET;
_20.2 = 152106334280150427154829141632433102275_i128 | (-104135842452541348745632678350631052317_i128);
(*_35) = -(*_39);
_20 = (_37, 27_u8, 102586884008701113716132727587114284387_i128, 3438219557936001408_i64);
(*_39) = (*_35);
_11 = (*_39) as f32;
(*_39) = -RET;
(*_2) = _8;
_62 = _20.3 as f64;
(*_7) = _6;
(*_2) = _8;
(*_35) = (*_39);
_51.0 = &_46.fld0.2;
_20.1 = 51_u8 - 6_u8;
(*_39) = (*_35) + (*_35);
(*_35) = _18 as f64;
Goto(bb27)
}
bb45 = {
(*_7) = [_15,_15,_15,_15];
(*_16) = _20.0 as f64;
_22 = _21;
(*_2) = _8;
_15 = !(-14350_i16);
Goto(bb17)
}
bb46 = {
_75 = _51.1;
_35 = core::ptr::addr_of_mut!(_70.2);
(*_58) = &_46.fld0.2;
_5 = _12;
(*_35) = _65 * _22;
_35 = core::ptr::addr_of_mut!(_44);
(*_39) = -_70.2;
(*_39) = _70.2 - _22;
_73 = _72 - _17;
(*_7) = [_15,_46.fld0.1.2,_68,_46.fld0.1.2];
(*_39) = _62 + Field::<f64>(Variant(_46.fld0.2, 0), 0);
(*_52) = (-63_i8) as isize;
_17 = _72;
_84.1.3 = [_46.fld0.1.2,_68,_15,_46.fld0.1.2];
_67 = &(*_64);
(*_7) = [_46.fld0.1.2,_68,_15,_46.fld0.1.2];
(*_7) = [_46.fld0.1.2,_15,_68,_46.fld0.1.2];
_11 = _31.1 + _24;
Goto(bb47)
}
bb47 = {
_40 = [_20.1,_20.1];
_55 = Move(_46.fld2);
(*_52) = _73 & _56;
_2 = core::ptr::addr_of_mut!(_12);
_86.1.1 = _57 & _57;
match (*_25) {
0 => bb48,
31901 => bb50,
_ => bb49
}
}
bb48 = {
(*_35) = -RET;
_20.2 = 152106334280150427154829141632433102275_i128 | (-104135842452541348745632678350631052317_i128);
(*_35) = -(*_39);
_20 = (_37, 27_u8, 102586884008701113716132727587114284387_i128, 3438219557936001408_i64);
(*_39) = (*_35);
_11 = (*_39) as f32;
(*_39) = -RET;
(*_2) = _8;
_62 = _20.3 as f64;
(*_7) = _6;
(*_2) = _8;
(*_35) = (*_39);
_51.0 = &_46.fld0.2;
_20.1 = 51_u8 - 6_u8;
(*_39) = (*_35) + (*_35);
(*_35) = _18 as f64;
Goto(bb27)
}
bb49 = {
(*_7) = [20501_i16,19236_i16,24459_i16,30045_i16];
(*_7) = [(-31046_i16),19858_i16,(-31102_i16),9745_i16];
(*_7) = [(-18181_i16),(-25403_i16),(-22701_i16),(-8663_i16)];
(*_2) = _5;
_18 = 1516780765_u32 as f32;
(*_2) = _5;
(*_2) = _5;
(*_7) = [13741_i16,(-30269_i16),(-165_i16),(-22107_i16)];
(*_7) = _9;
(*_2) = _4;
(*_2) = _5;
(*_7) = [(-30631_i16),(-23441_i16),26086_i16,(-7552_i16)];
(*_7) = _6;
_5 = _3;
(*_2) = _4;
(*_7) = [14690_i16,(-17186_i16),(-19848_i16),(-1129_i16)];
(*_2) = _5;
(*_7) = [19484_i16,6456_i16,(-13247_i16),(-21516_i16)];
_12 = _3;
(*_2) = _3;
Goto(bb3)
}
bb50 = {
_87 = Move(_52);
(*_39) = _86.1.1 as f64;
_48 = _86.1.1 << _63;
_45 = _29 == _63;
(*_7) = [_46.fld0.1.2,_15,_15,_15];
_52 = core::ptr::addr_of!(_41);
(*_52) = _20.0 + _63;
_8 = _5;
_84.2 = Adt25::Variant0 { fld0: _22,fld1: Field::<u32>(Variant(_46.fld0.2, 0), 1) };
(*_2) = _51.1;
place!(Field::<f64>(Variant(_46.fld0.2, 0), 0)) = _70.2;
(*_7) = _46.fld0.0;
_11 = _18 * _70.1.2;
(*_7) = [_68,_15,_68,_46.fld0.1.2];
(*_52) = _45 as isize;
_55 = core::ptr::addr_of!((*_39));
(*_7) = [_68,_15,_46.fld0.1.2,_68];
_54 = (*_52) & (*_52);
(*_7) = [_46.fld0.1.2,_46.fld0.1.2,_68,_46.fld0.1.2];
_39 = core::ptr::addr_of_mut!((*_39));
_84.2 = Move(_46.fld0.2);
(*_2) = _51.1;
Goto(bb51)
}
bb51 = {
Call(_95 = dump_var(Move(_57), Move(_29), Move(_34), Move(_6)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_95 = dump_var(Move(_73), Move(_49), Move(_23), Move(_28)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_95 = dump_var(Move(_20), Move(_9), Move(_3), Move(_41)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_95 = dump_var(Move(_56), Move(_8), Move(_19), Move(_33)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_95 = dump_var(Move(_68), Move(_37), _96, _96), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: f64,mut _2: *mut char,mut _3: char,mut _4: *const f64,mut _5: i128,mut _6: char,mut _7: char,mut _8: char,mut _9: char) -> isize {
mir! {
type RET = isize;
let _10: Adt24;
let _11: bool;
let _12: bool;
let _13: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _14: u32;
let _15: f32;
let _16: u8;
let _17: char;
let _18: u8;
let _19: isize;
let _20: isize;
let _21: &'static u8;
let _22: i8;
let _23: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _24: *mut *mut char;
let _25: isize;
let _26: usize;
let _27: &'static mut u16;
let _28: *const [char; 4];
let _29: *const [char; 4];
let _30: &'static &'static (i32, (usize, u128, f32), f64);
let _31: i128;
let _32: ([u16; 5], *mut char, i16, [i16; 4]);
let _33: f64;
let _34: i16;
let _35: u16;
let _36: &'static Adt25;
let _37: isize;
let _38: [u64; 7];
let _39: [i16; 4];
let _40: [char; 6];
let _41: *const [char; 4];
let _42: *mut u32;
let _43: (i32, (usize, u128, f32), f64);
let _44: Adt25;
let _45: bool;
let _46: [u8; 4];
let _47: [i16; 5];
let _48: *mut f64;
let _49: &'static mut u16;
let _50: *mut char;
let _51: i8;
let _52: bool;
let _53: &'static [char; 4];
let _54: &'static [i128; 4];
let _55: u32;
let _56: *const u64;
let _57: *mut *mut char;
let _58: &'static [i128; 4];
let _59: f32;
let _60: *const i8;
let _61: &'static f64;
let _62: isize;
let _63: *mut u32;
let _64: char;
let _65: u128;
let _66: *const u64;
let _67: u128;
let _68: (usize, u128, f32);
let _69: isize;
let _70: char;
let _71: [i128; 6];
let _72: i64;
let _73: bool;
let _74: [u128; 5];
let _75: isize;
let _76: &'static mut u16;
let _77: Adt62;
let _78: &'static Adt25;
let _79: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _80: &'static (i32, (usize, u128, f32), f64);
let _81: i32;
let _82: Adt84;
let _83: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _84: *const [char; 4];
let _85: f64;
let _86: [i128; 4];
let _87: i16;
let _88: isize;
let _89: *mut char;
let _90: &'static mut usize;
let _91: ();
let _92: ();
{
_7 = _8;
_3 = _7;
_6 = _9;
_3 = _9;
_9 = _3;
_6 = _7;
_1 = 1975008233_i32 as f64;
Call(_1 = fn9(Move(_2), _9, _7, _8, Move(_4), _7, _5, _7, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 55_i8 as f64;
_3 = _9;
RET = -120_isize;
_1 = 11457779218500704114_u64 as f64;
_1 = (-81_i8) as f64;
Goto(bb2)
}
bb2 = {
_9 = _3;
_4 = core::ptr::addr_of!(_1);
(*_4) = 16343698594948327080_u64 as f64;
RET = -(-9223372036854775808_isize);
(*_4) = (-3719863987373377333_i64) as f64;
_4 = core::ptr::addr_of!((*_4));
_2 = core::ptr::addr_of_mut!(_8);
(*_4) = 126568382940529916769715777619775684207_u128 as f64;
_9 = (*_2);
(*_2) = _7;
(*_2) = _3;
(*_4) = 58975_u16 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb3 = {
(*_4) = 20990_i16 as f64;
(*_4) = 6661066854910268605_i64 as f64;
_4 = core::ptr::addr_of!((*_4));
(*_4) = 1935757689_i32 as f64;
Call((*_4) = core::intrinsics::transmute(RET), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_2) = _6;
(*_4) = 2310050472037924462_usize as f64;
(*_2) = _7;
_4 = core::ptr::addr_of!((*_4));
(*_4) = 78_u8 as f64;
(*_4) = 16_u8 as f64;
(*_2) = _9;
_2 = core::ptr::addr_of_mut!(_3);
_7 = (*_2);
_1 = 5581_i16 as f64;
(*_2) = _9;
(*_4) = (-41_i16) as f64;
(*_2) = _8;
(*_2) = _7;
_15 = 5_usize as f32;
_6 = (*_2);
(*_2) = _9;
(*_2) = _8;
(*_2) = _7;
(*_4) = 59_i8 as f64;
(*_2) = _6;
(*_2) = _8;
(*_4) = 8648719884910110580_u64 as f64;
(*_2) = _7;
(*_4) = 105138364597235666955070994441412969823_u128 as f64;
match _5 {
0 => bb5,
1 => bb6,
73389032990637857524004529411297643187 => bb8,
_ => bb7
}
}
bb5 = {
(*_4) = 20990_i16 as f64;
(*_4) = 6661066854910268605_i64 as f64;
_4 = core::ptr::addr_of!((*_4));
(*_4) = 1935757689_i32 as f64;
Call((*_4) = core::intrinsics::transmute(RET), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_9 = _3;
_4 = core::ptr::addr_of!(_1);
(*_4) = 16343698594948327080_u64 as f64;
RET = -(-9223372036854775808_isize);
(*_4) = (-3719863987373377333_i64) as f64;
_4 = core::ptr::addr_of!((*_4));
_2 = core::ptr::addr_of_mut!(_8);
(*_4) = 126568382940529916769715777619775684207_u128 as f64;
_9 = (*_2);
(*_2) = _7;
(*_2) = _3;
(*_4) = 58975_u16 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb7 = {
_1 = 55_i8 as f64;
_3 = _9;
RET = -120_isize;
_1 = 11457779218500704114_u64 as f64;
_1 = (-81_i8) as f64;
Goto(bb2)
}
bb8 = {
(*_2) = _8;
(*_2) = _9;
_16 = 3_u8 << _5;
(*_2) = _9;
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
73389032990637857524004529411297643187 => bb10,
_ => bb9
}
}
bb9 = {
_9 = _3;
_4 = core::ptr::addr_of!(_1);
(*_4) = 16343698594948327080_u64 as f64;
RET = -(-9223372036854775808_isize);
(*_4) = (-3719863987373377333_i64) as f64;
_4 = core::ptr::addr_of!((*_4));
_2 = core::ptr::addr_of_mut!(_8);
(*_4) = 126568382940529916769715777619775684207_u128 as f64;
_9 = (*_2);
(*_2) = _7;
(*_2) = _3;
(*_4) = 58975_u16 as f64;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb10 = {
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _6;
(*_2) = _9;
(*_2) = _9;
(*_4) = (-37_i8) as f64;
(*_2) = _8;
(*_2) = _6;
(*_4) = 90_i8 as f64;
_12 = !false;
(*_2) = _7;
Goto(bb11)
}
bb11 = {
(*_2) = _9;
_19 = _5 as isize;
(*_2) = _8;
(*_4) = 30236_i16 as f64;
_17 = _3;
(*_4) = _15 as f64;
(*_2) = _6;
(*_4) = (-5099655321451443681_i64) as f64;
(*_4) = 32588_u16 as f64;
(*_4) = 1664713168_i32 as f64;
_16 = (*_2) as u8;
(*_2) = _7;
_2 = core::ptr::addr_of_mut!((*_2));
_20 = -_19;
(*_2) = _17;
_2 = core::ptr::addr_of_mut!((*_2));
(*_4) = 2669343888666459771_u64 as f64;
(*_4) = _16 as f64;
(*_4) = (-1134980402_i32) as f64;
(*_2) = _17;
(*_2) = _9;
Goto(bb12)
}
bb12 = {
(*_2) = _6;
(*_2) = _9;
(*_4) = _15 as f64;
_14 = 679360131_u32 * 3494643435_u32;
_25 = _19 + _20;
(*_2) = _8;
(*_4) = _25 as f64;
(*_4) = _15 as f64;
(*_2) = _8;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _17;
_2 = core::ptr::addr_of_mut!((*_2));
(*_4) = 8303378649315291744_i64 as f64;
_26 = _16 as usize;
(*_4) = 23764_u16 as f64;
(*_4) = 71051357485699665124906590015943814125_u128 as f64;
_22 = (-14_i8) >> _25;
(*_2) = _6;
_16 = 436677596356293674686079145731026633_u128 as u8;
(*_4) = _15 as f64;
(*_4) = 16570693016275668350_u64 as f64;
Goto(bb13)
}
bb13 = {
_22 = (-3_i8) >> _14;
(*_2) = _8;
(*_4) = _15 as f64;
_16 = 116_u8 >> _20;
_25 = -_19;
_12 = true;
(*_2) = _17;
(*_2) = _9;
(*_4) = _26 as f64;
(*_2) = _6;
match _5 {
0 => bb4,
1 => bb10,
2 => bb8,
73389032990637857524004529411297643187 => bb15,
_ => bb14
}
}
bb14 = {
_1 = 55_i8 as f64;
_3 = _9;
RET = -120_isize;
_1 = 11457779218500704114_u64 as f64;
_1 = (-81_i8) as f64;
Goto(bb2)
}
bb15 = {
(*_4) = 244196738752115382454789431139096472814_u128 as f64;
(*_4) = 1917135542_i32 as f64;
_14 = 970015697_u32 * 189376456_u32;
(*_2) = _7;
_20 = 68659128561136013667253383283207212755_u128 as isize;
(*_2) = _17;
(*_2) = _17;
_11 = !_12;
(*_2) = _9;
(*_4) = _15 as f64;
_15 = _5 as f32;
_24 = core::ptr::addr_of_mut!(_2);
_2 = core::ptr::addr_of_mut!((*_2));
_19 = _25 & _25;
(*_24) = core::ptr::addr_of_mut!(_9);
Goto(bb16)
}
bb16 = {
_20 = !_25;
(*_24) = core::ptr::addr_of_mut!((*_2));
match _5 {
0 => bb4,
73389032990637857524004529411297643187 => bb18,
_ => bb17
}
}
bb17 = {
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _6;
(*_2) = _9;
(*_2) = _9;
(*_4) = (-37_i8) as f64;
(*_2) = _8;
(*_2) = _6;
(*_4) = 90_i8 as f64;
_12 = !false;
(*_2) = _7;
Goto(bb11)
}
bb18 = {
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _17;
(*_4) = (-12415_i16) as f64;
_7 = (*_2);
_21 = &_16;
(*_4) = 947757737_i32 as f64;
(*_2) = _8;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_4) = 15800971291736395355_u64 as f64;
_4 = core::ptr::addr_of!((*_4));
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _17;
(*_4) = 14595_u16 as f64;
(*_2) = _8;
_32.1 = core::ptr::addr_of_mut!((*_2));
(*_2) = _17;
_8 = (*_2);
_19 = !_25;
_34 = (-32308_i16) & (-22071_i16);
(*_2) = _3;
_33 = _5 as f64;
_32.3 = [_34,_34,_34,_34];
match _5 {
0 => bb19,
73389032990637857524004529411297643187 => bb21,
_ => bb20
}
}
bb19 = {
_22 = (-3_i8) >> _14;
(*_2) = _8;
(*_4) = _15 as f64;
_16 = 116_u8 >> _20;
_25 = -_19;
_12 = true;
(*_2) = _17;
(*_2) = _9;
(*_4) = _26 as f64;
(*_2) = _6;
match _5 {
0 => bb4,
1 => bb10,
2 => bb8,
73389032990637857524004529411297643187 => bb15,
_ => bb14
}
}
bb20 = {
_20 = !_25;
(*_24) = core::ptr::addr_of_mut!((*_2));
match _5 {
0 => bb4,
73389032990637857524004529411297643187 => bb18,
_ => bb17
}
}
bb21 = {
(*_2) = _3;
_8 = (*_2);
(*_24) = Move(_32.1);
(*_4) = _33 * _33;
Goto(bb22)
}
bb22 = {
(*_24) = core::ptr::addr_of_mut!(_7);
(*_2) = _9;
(*_4) = 55161_u16 as f64;
(*_24) = core::ptr::addr_of_mut!((*_2));
_32.2 = _34;
Goto(bb23)
}
bb23 = {
(*_4) = _34 as f64;
(*_4) = -_33;
(*_24) = core::ptr::addr_of_mut!((*_2));
_22 = (-91_i8) & 16_i8;
_8 = (*_2);
_5 = (-133466814869235746224431913946203528493_i128) - 148809178041766035894632365496660817817_i128;
_32.3 = [_32.2,_32.2,_32.2,_34];
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _9;
Goto(bb24)
}
bb24 = {
_1 = _33 + _33;
(*_2) = _8;
_39 = [_34,_34,_32.2,_32.2];
(*_4) = (*_21) as f64;
_35 = 33635_u16 - 28851_u16;
_16 = 20_u8 ^ 114_u8;
(*_2) = _8;
_32.1 = core::ptr::addr_of_mut!((*_2));
(*_4) = _33 * _33;
(*_2) = _17;
(*_4) = _5 as f64;
(*_2) = _8;
_18 = _16;
_32.2 = _34 ^ _34;
Goto(bb25)
}
bb25 = {
(*_2) = _9;
_32.0 = [_35,_35,_35,_35,_35];
(*_4) = _33 - _33;
_38 = [11232447685547567902_u64,15457323777698070589_u64,7480432804856500246_u64,17919348789092279687_u64,5951709334793212346_u64,15519549620364580468_u64,1563019631391120597_u64];
_39 = [_32.2,_34,_32.2,_34];
(*_4) = _19 as f64;
(*_2) = _3;
(*_24) = core::ptr::addr_of_mut!((*_2));
_32.1 = core::ptr::addr_of_mut!((*_2));
(*_24) = core::ptr::addr_of_mut!((*_2));
_23 = &_32;
(*_4) = -_33;
(*_4) = _33 * _33;
(*_2) = _17;
Goto(bb26)
}
bb26 = {
(*_4) = _33 - _33;
(*_24) = core::ptr::addr_of_mut!(_6);
_43.1.0 = !_26;
_27 = &mut _35;
(*_27) = !27637_u16;
_33 = (*_4) * (*_4);
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _7;
(*_24) = core::ptr::addr_of_mut!((*_2));
_9 = (*_2);
_43.1.0 = _16 as usize;
(*_4) = _33;
Goto(bb27)
}
bb27 = {
_43.2 = (*_4) * (*_4);
_45 = _11;
(*_24) = core::ptr::addr_of_mut!(_7);
_45 = (*_23).2 <= (*_23).2;
_46 = [_18,_18,_18,_16];
_6 = (*_2);
Goto(bb28)
}
bb28 = {
_32.0 = [(*_27),(*_27),(*_27),(*_27),(*_27)];
_7 = _8;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_27) = 52112_u16 >> (*_23).2;
_40 = [(*_2),(*_2),(*_2),(*_2),_9,(*_2)];
(*_27) = !4093_u16;
_37 = _19;
(*_27) = !52726_u16;
(*_27) = 17195_u16 - 54152_u16;
(*_2) = _9;
(*_2) = _9;
(*_27) = _16 as u16;
(*_2) = _6;
(*_4) = _33;
(*_2) = _8;
_31 = _5 << (*_23).2;
(*_24) = core::ptr::addr_of_mut!((*_2));
_22 = !55_i8;
_38 = [11849211355910469681_u64,1055159976331824816_u64,13373070687078157241_u64,1927164383385147999_u64,18037928148953881245_u64,17518983473013591081_u64,16610600333154865672_u64];
(*_4) = _43.2 * _43.2;
(*_4) = _22 as f64;
(*_2) = _9;
(*_4) = _43.2;
_42 = core::ptr::addr_of_mut!(_14);
_17 = (*_2);
(*_4) = _43.2 - _43.2;
Goto(bb29)
}
bb29 = {
_14 = (*_23).2 as u32;
(*_2) = _3;
_47 = [(*_23).2,(*_23).2,(*_23).2,(*_23).2,(*_23).2];
(*_4) = -_33;
(*_42) = _15 as u32;
(*_2) = _3;
Goto(bb30)
}
bb30 = {
_3 = (*_2);
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_27) = 8145_u16 << (*_23).2;
_38 = [10272605655716234917_u64,8868346556538135052_u64,106102402731613840_u64,11795392498596451009_u64,8424110116929962929_u64,566646570719845763_u64,5025157603308211308_u64];
_12 = (*_23).2 == (*_23).2;
(*_2) = _9;
Goto(bb31)
}
bb31 = {
_11 = !_45;
(*_2) = _9;
(*_2) = _3;
_40 = [_7,(*_2),(*_2),(*_2),_17,(*_2)];
(*_24) = core::ptr::addr_of_mut!((*_2));
_32.0 = [(*_27),(*_27),(*_27),(*_27),(*_27)];
(*_2) = _3;
(*_24) = Move(_32.1);
(*_24) = core::ptr::addr_of_mut!(_8);
_25 = _20 & RET;
_46 = [_16,_18,_16,_18];
(*_2) = _7;
(*_2) = _3;
(*_42) = 3206113418_u32 | 4133891304_u32;
(*_2) = _17;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _17;
(*_2) = _9;
(*_2) = _3;
Goto(bb32)
}
bb32 = {
Goto(bb33)
}
bb33 = {
_51 = (*_27) as i8;
(*_4) = _33 * _43.2;
(*_27) = 12569_u16 << (*_23).2;
_49 = &mut (*_27);
(*_42) = 3516508251_u32;
_52 = !_11;
(*_49) = (-1326922941_i32) as u16;
_49 = Move(_27);
match (*_42) {
0 => bb29,
1 => bb4,
2 => bb24,
3 => bb34,
4 => bb35,
5 => bb36,
6 => bb37,
3516508251 => bb39,
_ => bb38
}
}
bb34 = {
(*_4) = 244196738752115382454789431139096472814_u128 as f64;
(*_4) = 1917135542_i32 as f64;
_14 = 970015697_u32 * 189376456_u32;
(*_2) = _7;
_20 = 68659128561136013667253383283207212755_u128 as isize;
(*_2) = _17;
(*_2) = _17;
_11 = !_12;
(*_2) = _9;
(*_4) = _15 as f64;
_15 = _5 as f32;
_24 = core::ptr::addr_of_mut!(_2);
_2 = core::ptr::addr_of_mut!((*_2));
_19 = _25 & _25;
(*_24) = core::ptr::addr_of_mut!(_9);
Goto(bb16)
}
bb35 = {
(*_4) = 20990_i16 as f64;
(*_4) = 6661066854910268605_i64 as f64;
_4 = core::ptr::addr_of!((*_4));
(*_4) = 1935757689_i32 as f64;
Call((*_4) = core::intrinsics::transmute(RET), ReturnTo(bb4), UnwindUnreachable())
}
bb36 = {
(*_2) = _3;
_8 = (*_2);
(*_24) = Move(_32.1);
(*_4) = _33 * _33;
Goto(bb22)
}
bb37 = {
(*_2) = _9;
_19 = _5 as isize;
(*_2) = _8;
(*_4) = 30236_i16 as f64;
_17 = _3;
(*_4) = _15 as f64;
(*_2) = _6;
(*_4) = (-5099655321451443681_i64) as f64;
(*_4) = 32588_u16 as f64;
(*_4) = 1664713168_i32 as f64;
_16 = (*_2) as u8;
(*_2) = _7;
_2 = core::ptr::addr_of_mut!((*_2));
_20 = -_19;
(*_2) = _17;
_2 = core::ptr::addr_of_mut!((*_2));
(*_4) = 2669343888666459771_u64 as f64;
(*_4) = _16 as f64;
(*_4) = (-1134980402_i32) as f64;
(*_2) = _17;
(*_2) = _9;
Goto(bb12)
}
bb38 = {
_32.0 = [(*_27),(*_27),(*_27),(*_27),(*_27)];
_7 = _8;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_27) = 52112_u16 >> (*_23).2;
_40 = [(*_2),(*_2),(*_2),(*_2),_9,(*_2)];
(*_27) = !4093_u16;
_37 = _19;
(*_27) = !52726_u16;
(*_27) = 17195_u16 - 54152_u16;
(*_2) = _9;
(*_2) = _9;
(*_27) = _16 as u16;
(*_2) = _6;
(*_4) = _33;
(*_2) = _8;
_31 = _5 << (*_23).2;
(*_24) = core::ptr::addr_of_mut!((*_2));
_22 = !55_i8;
_38 = [11849211355910469681_u64,1055159976331824816_u64,13373070687078157241_u64,1927164383385147999_u64,18037928148953881245_u64,17518983473013591081_u64,16610600333154865672_u64];
(*_4) = _43.2 * _43.2;
(*_4) = _22 as f64;
(*_2) = _9;
(*_4) = _43.2;
_42 = core::ptr::addr_of_mut!(_14);
_17 = (*_2);
(*_4) = _43.2 - _43.2;
Goto(bb29)
}
bb39 = {
(*_2) = _3;
(*_4) = -_43.2;
(*_4) = _43.2;
_21 = &_18;
_48 = core::ptr::addr_of_mut!((*_4));
_9 = (*_2);
(*_2) = _3;
_33 = -(*_48);
_46 = [(*_21),(*_21),(*_21),(*_21)];
(*_42) = !150469943_u32;
(*_48) = _33 * _33;
_6 = (*_2);
Goto(bb40)
}
bb40 = {
(*_48) = _33 + _33;
_43.1.1 = !112246620533437673404503163845719716348_u128;
(*_42) = !3587298987_u32;
_8 = _9;
_7 = _17;
(*_2) = _7;
(*_2) = _7;
(*_2) = _7;
_52 = _12;
(*_42) = _43.1.0 as u32;
_19 = _25 & _37;
(*_4) = (*_23).2 as f64;
(*_24) = core::ptr::addr_of_mut!((*_2));
_15 = 110535986288739116_i64 as f32;
(*_4) = _43.2 + _33;
(*_2) = _7;
(*_2) = _9;
_17 = (*_2);
(*_4) = _43.2;
(*_4) = _43.2;
_1 = _33 - _43.2;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _6;
(*_2) = _6;
(*_4) = _18 as f64;
(*_24) = core::ptr::addr_of_mut!((*_2));
_25 = !_19;
Goto(bb41)
}
bb41 = {
(*_4) = _43.2 - _33;
(*_2) = _3;
_6 = (*_2);
(*_4) = _33 * _33;
(*_2) = _9;
Goto(bb42)
}
bb42 = {
_60 = core::ptr::addr_of!(_22);
(*_42) = !1245500205_u32;
(*_60) = 12586_u16 as i8;
_43.1.2 = (*_60) as f32;
_48 = core::ptr::addr_of_mut!((*_4));
(*_42) = !3914136272_u32;
_55 = _26 as u32;
(*_4) = -_33;
_38 = [1612764241184669724_u64,4557353786992320980_u64,11407129387221101259_u64,508630558076562942_u64,4468824045882233994_u64,17295849843375762236_u64,15271859943963580139_u64];
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_42) = _55 >> _20;
(*_60) = -_51;
(*_4) = _26 as f64;
(*_42) = _55 - _55;
_38 = [7681237831509359699_u64,16072901680622181596_u64,17718815028887193466_u64,8301462473049348937_u64,9142962522255637472_u64,15523001510019127203_u64,6163773616295249594_u64];
(*_24) = core::ptr::addr_of_mut!(_6);
_12 = _11;
_64 = (*_2);
(*_2) = _3;
Goto(bb43)
}
bb43 = {
(*_2) = _64;
_21 = &_16;
(*_60) = _51;
(*_60) = _51;
_43.0 = _11 as i32;
(*_42) = _55 & _55;
_25 = _37 * _19;
(*_24) = core::ptr::addr_of_mut!((*_2));
_43.2 = _33 - (*_4);
(*_60) = !_51;
_45 = _12;
_44 = Adt25::Variant0 { fld0: _43.2,fld1: (*_42) };
(*_60) = _51 >> _37;
_65 = _43.1.1;
_9 = (*_2);
_4 = core::ptr::addr_of!((*_4));
_39 = [(*_23).2,(*_23).2,(*_23).2,_32.2];
(*_24) = core::ptr::addr_of_mut!((*_2));
_43.1 = (_26, _65, _15);
(*_2) = _8;
(*_2) = _7;
(*_60) = _51 >> (*_21);
Goto(bb44)
}
bb44 = {
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_4) = _43.2;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_4) = -_33;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_2) = _9;
(*_4) = _43.2;
(*_24) = core::ptr::addr_of_mut!(_17);
(*_60) = _51 << _31;
Goto(bb45)
}
bb45 = {
_68.0 = _26;
(*_24) = core::ptr::addr_of_mut!((*_2));
_59 = -_43.1.2;
_43.1.1 = _65;
_48 = core::ptr::addr_of_mut!(_1);
_67 = _43.1.1 | _65;
Goto(bb46)
}
bb46 = {
(*_24) = core::ptr::addr_of_mut!((*_2));
_62 = _20 * _19;
(*_2) = _9;
_17 = _6;
_44 = Adt25::Variant0 { fld0: (*_4),fld1: _55 };
(*_2) = _3;
(*_24) = core::ptr::addr_of_mut!((*_2));
(*_42) = Field::<u32>(Variant(_44, 0), 1) >> (*_21);
_48 = core::ptr::addr_of_mut!(_33);
(*_48) = (*_4) * (*_4);
(*_24) = core::ptr::addr_of_mut!((*_2));
_36 = &_44;
_31 = _5 - _5;
_79.1.2 = Move(_44);
(*_4) = -_33;
_47 = [(*_23).2,_34,(*_23).2,(*_23).2,(*_23).2];
(*_60) = -_51;
(*_60) = _31 as i8;
(*_60) = !_51;
_75 = _19 + _62;
Goto(bb47)
}
bb47 = {
(*_2) = _7;
Goto(bb48)
}
bb48 = {
_48 = core::ptr::addr_of_mut!((*_48));
_38 = [9894474998881754323_u64,2097745720539621542_u64,986478893872753745_u64,8915784722912480853_u64,13746207685544294605_u64,12775491754344560642_u64,15054109280151626861_u64];
(*_60) = _51;
_52 = (*_48) > (*_48);
_43.1.2 = _15 + _15;
(*_24) = core::ptr::addr_of_mut!((*_2));
_81 = (*_48) as i32;
(*_4) = -(*_48);
Goto(bb49)
}
bb49 = {
(*_2) = _9;
_21 = &_18;
(*_24) = core::ptr::addr_of_mut!((*_2));
_15 = -_43.1.2;
_5 = _31 >> _81;
_63 = core::ptr::addr_of_mut!((*_42));
Goto(bb50)
}
bb50 = {
Call(_91 = dump_var(Move(_9), Move(_25), Move(_26), Move(_16)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_91 = dump_var(Move(_64), Move(_65), Move(_6), Move(_18)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_91 = dump_var(Move(_40), Move(_7), Move(_47), Move(_5)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_91 = dump_var(Move(_75), Move(_38), Move(_81), Move(_20)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_91 = dump_var(Move(_22), Move(_62), _92, _92), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut char,mut _2: char,mut _3: char,mut _4: char,mut _5: *const f64,mut _6: char,mut _7: i128,mut _8: char,mut _9: char,mut _10: char) -> f64 {
mir! {
type RET = f64;
let _11: u128;
let _12: *mut f64;
let _13: u32;
let _14: [i128; 5];
let _15: char;
let _16: *mut &'static Adt25;
let _17: *const isize;
let _18: isize;
let _19: [char; 6];
let _20: Adt25;
let _21: *mut char;
let _22: char;
let _23: isize;
let _24: bool;
let _25: usize;
let _26: (isize, u8, i128, i64);
let _27: f32;
let _28: *mut *mut char;
let _29: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _30: u64;
let _31: Adt78;
let _32: *mut *mut char;
let _33: isize;
let _34: i8;
let _35: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _36: ();
let _37: ();
{
_2 = _6;
_8 = _2;
_8 = _2;
_10 = _9;
_3 = _10;
_10 = _3;
RET = 3051994308467037158_u64 as f64;
_3 = _9;
_11 = !177254726238455586871922021100632481437_u128;
_5 = core::ptr::addr_of!(RET);
(*_5) = 201_u8 as f64;
_11 = 86_i8 as u128;
(*_5) = 31803425_u32 as f64;
RET = 4919523410876777716_usize as f64;
(*_5) = 0_usize as f64;
_2 = _4;
_2 = _4;
(*_5) = (-81_isize) as f64;
_5 = core::ptr::addr_of!((*_5));
_13 = 2268374883_u32;
(*_5) = (-110_i8) as f64;
_2 = _3;
(*_5) = (-4728431018904926233_i64) as f64;
match _7 {
0 => bb1,
1 => bb2,
73389032990637857524004529411297643187 => bb4,
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
_6 = _3;
_5 = core::ptr::addr_of!((*_5));
(*_5) = 58636_u16 as f64;
(*_5) = 54681_u16 as f64;
_7 = 41045131410824654684414769049138642042_i128 & 156770754330955330198826526805806162341_i128;
(*_5) = (-109_i8) as f64;
(*_5) = _13 as f64;
_12 = core::ptr::addr_of_mut!((*_5));
_14 = [_7,_7,_7,_7,_7];
_7 = (-146105721912459285089356344562191273503_i128) - (-60997231526378118632386864749297205902_i128);
(*_12) = _13 as f64;
(*_5) = 59069_u16 as f64;
match _13 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
2268374883 => bb11,
_ => bb10
}
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
_11 = 252385528622446795850750576695116790088_u128 | 33786613493250183224868524772664237146_u128;
(*_5) = 96_i8 as f64;
(*_5) = 808018876_i32 as f64;
_7 = (-169074428418643255701331231652126253962_i128) * 40745932384922798284456848361742301049_i128;
_12 = core::ptr::addr_of_mut!((*_5));
(*_12) = (-9223372036854775808_isize) as f64;
(*_5) = 12784_i16 as f64;
(*_5) = _7 as f64;
(*_5) = 7658666715454189028_u64 as f64;
(*_5) = 41954_u16 as f64;
_9 = _8;
RET = (-622420468_i32) as f64;
_4 = _10;
_5 = core::ptr::addr_of!((*_5));
(*_5) = 55178_u16 as f64;
RET = 1076534531_i32 as f64;
(*_5) = _11 as f64;
_10 = _9;
_6 = _9;
(*_5) = 2847_u16 as f64;
(*_5) = 3075487436670091304_u64 as f64;
(*_5) = 6391816040731783828_u64 as f64;
RET = (-1014734554_i32) as f64;
(*_5) = _13 as f64;
_1 = core::ptr::addr_of_mut!(_3);
(*_5) = (-4165277731038321585_i64) as f64;
Goto(bb12)
}
bb12 = {
(*_1) = _10;
_7 = _10 as i128;
_1 = core::ptr::addr_of_mut!((*_1));
_3 = _10;
(*_1) = _4;
_10 = _8;
(*_5) = 5277029373040360008_i64 as f64;
match _13 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb13,
4 => bb14,
2268374883 => bb16,
_ => bb15
}
}
bb13 = {
_11 = 252385528622446795850750576695116790088_u128 | 33786613493250183224868524772664237146_u128;
(*_5) = 96_i8 as f64;
(*_5) = 808018876_i32 as f64;
_7 = (-169074428418643255701331231652126253962_i128) * 40745932384922798284456848361742301049_i128;
_12 = core::ptr::addr_of_mut!((*_5));
(*_12) = (-9223372036854775808_isize) as f64;
(*_5) = 12784_i16 as f64;
(*_5) = _7 as f64;
(*_5) = 7658666715454189028_u64 as f64;
(*_5) = 41954_u16 as f64;
_9 = _8;
RET = (-622420468_i32) as f64;
_4 = _10;
_5 = core::ptr::addr_of!((*_5));
(*_5) = 55178_u16 as f64;
RET = 1076534531_i32 as f64;
(*_5) = _11 as f64;
_10 = _9;
_6 = _9;
(*_5) = 2847_u16 as f64;
(*_5) = 3075487436670091304_u64 as f64;
(*_5) = 6391816040731783828_u64 as f64;
RET = (-1014734554_i32) as f64;
(*_5) = _13 as f64;
_1 = core::ptr::addr_of_mut!(_3);
(*_5) = (-4165277731038321585_i64) as f64;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_17 = core::ptr::addr_of!(_18);
_15 = (*_1);
match _13 {
0 => bb17,
2268374883 => bb19,
_ => bb18
}
}
bb17 = {
_6 = _3;
_5 = core::ptr::addr_of!((*_5));
(*_5) = 58636_u16 as f64;
(*_5) = 54681_u16 as f64;
_7 = 41045131410824654684414769049138642042_i128 & 156770754330955330198826526805806162341_i128;
(*_5) = (-109_i8) as f64;
(*_5) = _13 as f64;
_12 = core::ptr::addr_of_mut!((*_5));
_14 = [_7,_7,_7,_7,_7];
_7 = (-146105721912459285089356344562191273503_i128) - (-60997231526378118632386864749297205902_i128);
(*_12) = _13 as f64;
(*_5) = 59069_u16 as f64;
match _13 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
2268374883 => bb11,
_ => bb10
}
}
bb18 = {
Return()
}
bb19 = {
(*_5) = _11 as f64;
(*_1) = _8;
RET = 42_u8 as f64;
(*_17) = (*_5) as isize;
(*_17) = 80_isize;
(*_17) = (-27_isize) & 9223372036854775807_isize;
(*_17) = (-78_isize) >> _11;
(*_5) = _13 as f64;
_1 = core::ptr::addr_of_mut!((*_1));
(*_5) = (-122_i8) as f64;
(*_1) = _10;
(*_1) = _9;
_22 = _6;
(*_5) = 7343822066503429319_u64 as f64;
_3 = _9;
(*_1) = _22;
match _13 {
0 => bb20,
1 => bb21,
2268374883 => bb23,
_ => bb22
}
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
(*_17) = (-9223372036854775808_isize) >> _13;
_23 = (*_17) << (*_17);
(*_5) = _11 as f64;
(*_5) = 55198_u16 as f64;
(*_5) = 7444766172624498167_i64 as f64;
(*_17) = !_23;
Call(_9 = fn10((*_1), Move(_5), (*_17), Move(_1), Move(_12), (*_17)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_5 = core::ptr::addr_of!(RET);
(*_5) = (*_17) as f64;
(*_5) = 46_u8 as f64;
_24 = (*_17) != (*_17);
(*_17) = 6399_u16 as isize;
_26.2 = _7 * _7;
(*_17) = _23 | _23;
Goto(bb25)
}
bb25 = {
_5 = core::ptr::addr_of!((*_5));
(*_17) = _23 | _23;
_26.2 = _7 << _18;
_1 = core::ptr::addr_of_mut!(_22);
(*_5) = _11 as f64;
(*_17) = -_23;
_26.3 = 7_usize as i64;
_26.3 = 7416181669515726079_i64;
Goto(bb26)
}
bb26 = {
(*_17) = _23 >> _26.2;
(*_5) = _11 as f64;
_26.1 = !1_u8;
(*_5) = (-737539559_i32) as f64;
_25 = _13 as usize;
(*_17) = _23 + _23;
(*_1) = _8;
_21 = core::ptr::addr_of_mut!((*_1));
_3 = _22;
_27 = 29733_u16 as f32;
_25 = !12921994246953420394_usize;
(*_17) = _23;
(*_17) = _23 * _23;
match _13 {
0 => bb1,
1 => bb16,
2 => bb11,
2268374883 => bb28,
_ => bb27
}
}
bb27 = {
_5 = core::ptr::addr_of!((*_5));
(*_17) = _23 | _23;
_26.2 = _7 << _18;
_1 = core::ptr::addr_of_mut!(_22);
(*_5) = _11 as f64;
(*_17) = -_23;
_26.3 = 7_usize as i64;
_26.3 = 7416181669515726079_i64;
Goto(bb26)
}
bb28 = {
(*_5) = (*_17) as f64;
_26 = ((*_17), 201_u8, _7, (-4877849883774101114_i64));
Goto(bb29)
}
bb29 = {
_21 = core::ptr::addr_of_mut!((*_1));
(*_17) = _23 & _23;
_26.0 = (*_17);
_29.0 = [14934_i16,24589_i16,(-178_i16),(-25361_i16)];
_15 = _9;
_21 = core::ptr::addr_of_mut!((*_1));
_29.1.3 = [22694_i16,23979_i16,27190_i16,(-21831_i16)];
(*_1) = _15;
_9 = (*_1);
_13 = !2329983559_u32;
_19 = [(*_1),(*_1),(*_1),(*_1),(*_1),(*_1)];
_34 = !(-35_i8);
(*_5) = _26.1 as f64;
_5 = core::ptr::addr_of!((*_5));
_30 = 18182244415063393383_u64 | 17717067543906029082_u64;
_15 = (*_1);
_29.1.0 = [36895_u16,41291_u16,18412_u16,50502_u16,10913_u16];
(*_17) = _26.0 << _26.1;
Goto(bb30)
}
bb30 = {
Call(_36 = dump_var(Move(_2), Move(_4), Move(_22), Move(_24)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_36 = dump_var(Move(_23), Move(_11), Move(_8), Move(_10)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_36 = dump_var(Move(_6), Move(_18), _37, _37), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: char,mut _2: *const f64,mut _3: isize,mut _4: *mut char,mut _5: *mut f64,mut _6: isize) -> char {
mir! {
type RET = char;
let _7: *const u64;
let _8: [u16; 5];
let _9: *const u64;
let _10: isize;
let _11: i16;
let _12: i128;
let _13: i8;
let _14: f32;
let _15: &'static mut Adt42;
let _16: [u32; 8];
let _17: &'static mut Adt42;
let _18: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _19: Adt84;
let _20: f64;
let _21: &'static i128;
let _22: *const i8;
let _23: &'static f64;
let _24: f32;
let _25: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _26: *mut [i16; 4];
let _27: u128;
let _28: &'static Adt25;
let _29: *mut u32;
let _30: u16;
let _31: char;
let _32: *mut [i16; 4];
let _33: isize;
let _34: [u8; 5];
let _35: isize;
let _36: isize;
let _37: bool;
let _38: f64;
let _39: &'static &'static (i32, (usize, u128, f32), f64);
let _40: *mut char;
let _41: char;
let _42: [i32; 1];
let _43: *const f64;
let _44: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _45: f32;
let _46: i32;
let _47: char;
let _48: *const i8;
let _49: isize;
let _50: [u8; 2];
let _51: i8;
let _52: bool;
let _53: *const [usize; 3];
let _54: f32;
let _55: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _56: [char; 4];
let _57: &'static Adt25;
let _58: [u128; 5];
let _59: Adt25;
let _60: f64;
let _61: [i128; 6];
let _62: i8;
let _63: [i128; 6];
let _64: bool;
let _65: isize;
let _66: isize;
let _67: bool;
let _68: bool;
let _69: u128;
let _70: u64;
let _71: &'static mut u16;
let _72: *const u64;
let _73: bool;
let _74: *const isize;
let _75: &'static mut u16;
let _76: &'static (i32, (usize, u128, f32), f64);
let _77: Adt78;
let _78: [i128; 5];
let _79: char;
let _80: char;
let _81: isize;
let _82: *mut [i16; 4];
let _83: ();
let _84: ();
{
_3 = _6 | _6;
RET = _1;
_6 = _3;
_8 = [23117_u16,5915_u16,37375_u16,46744_u16,59809_u16];
RET = _1;
_8 = [4675_u16,28423_u16,59934_u16,27437_u16,19862_u16];
RET = _1;
Call(_6 = fn11(Move(_4), Move(_5), Move(_2), _1, _1, _3, _3, _3, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _1;
_4 = core::ptr::addr_of_mut!(RET);
_1 = (*_4);
(*_4) = _1;
(*_4) = _1;
_1 = (*_4);
(*_4) = _1;
_6 = _3 & _3;
_6 = !_3;
(*_4) = _1;
(*_4) = _1;
_10 = 185_u8 as isize;
(*_4) = _1;
Goto(bb2)
}
bb2 = {
_8 = [44981_u16,29198_u16,29385_u16,41988_u16,37228_u16];
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
_4 = core::ptr::addr_of_mut!(_1);
_10 = 18075540975798213981878873775779459869_u128 as isize;
Goto(bb3)
}
bb3 = {
_11 = !18208_i16;
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
Goto(bb4)
}
bb4 = {
RET = (*_4);
_12 = !47037688844750246262737488373488002308_i128;
Goto(bb5)
}
bb5 = {
_13 = -(-113_i8);
_1 = RET;
_14 = 210_u8 as f32;
Goto(bb6)
}
bb6 = {
(*_4) = RET;
_11 = 3088_i16 ^ (-9337_i16);
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
_3 = 1753891522_u32 as isize;
_2 = core::ptr::addr_of!(_18.0);
_13 = 6_i8;
_5 = core::ptr::addr_of_mut!((*_2));
(*_2) = 2850900271830363118_i64 as f64;
(*_2) = _10 as f64;
(*_2) = _14 as f64;
(*_2) = 10373165439785735615_u64 as f64;
(*_4) = RET;
(*_2) = _12 as f64;
(*_4) = RET;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = RET;
_23 = &(*_2);
(*_2) = _14 as f64;
_18.0 = 76480192_u32 as f64;
_21 = &_12;
_19 = Adt84::Variant0 { fld0: (*_2),fld1: Move(_5) };
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_4) = RET;
(*_4) = RET;
_18.2.0 = [_11,_11,_11,_11];
_1 = RET;
_14 = _13 as f32;
_18.2.1.1 = core::ptr::addr_of_mut!((*_4));
_18.2.1 = (_8, Move(_4), _11, _18.2.0);
_4 = core::ptr::addr_of_mut!(_1);
_18.2.0 = [_11,_11,_11,_11];
_24 = _14 + _14;
_5 = core::ptr::addr_of_mut!((*_2));
(*_4) = RET;
(*_5) = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
(*_2) = Field::<f64>(Variant(_19, 0), 0);
_3 = !_6;
RET = (*_4);
_18.0 = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
(*_2) = Field::<f64>(Variant(_19, 0), 0);
_5 = core::ptr::addr_of_mut!((*_2));
(*_2) = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
place!(Field::<*mut f64>(Variant(_19, 0), 1)) = core::ptr::addr_of_mut!((*_2));
(*_2) = 2595690792_u32 as f64;
(*_2) = Field::<f64>(Variant(_19, 0), 0);
(*_4) = RET;
(*_2) = Field::<f64>(Variant(_19, 0), 0);
match _13 {
6 => bb8,
_ => bb3
}
}
bb8 = {
(*_2) = Field::<f64>(Variant(_19, 0), 0);
Goto(bb9)
}
bb9 = {
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
Goto(bb10)
}
bb10 = {
_16 = [2751415884_u32,2021403598_u32,503310206_u32,2529304027_u32,2880789857_u32,3626447354_u32,1120653504_u32,2315154649_u32];
_14 = _24;
(*_2) = -Field::<f64>(Variant(_19, 0), 0);
_20 = _6 as f64;
_13 = 61_i8;
_18.2.1 = (_8, Move(_4), _11, _18.2.0);
(*_2) = Field::<f64>(Variant(_19, 0), 0) * _20;
_8 = _18.2.1.0;
_5 = core::ptr::addr_of_mut!((*_2));
_20 = (*_5) * (*_2);
place!(Field::<*mut f64>(Variant(_19, 0), 1)) = core::ptr::addr_of_mut!((*_2));
_3 = _6;
(*_2) = _20 - _20;
_2 = core::ptr::addr_of!((*_2));
_4 = core::ptr::addr_of_mut!(RET);
(*_2) = -_20;
(*_4) = _1;
Call((*_2) = fn12(Move(_18.2.1), Move(_19), (*_4), _8, (*_4), (*_21), (*_4), (*_4), (*_4), (*_4), Move(_2), Move(_5)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = (*_21) as f64;
_26 = core::ptr::addr_of_mut!(_18.2.0);
(*_26) = [_11,_11,_11,_11];
(*_4) = _1;
(*_4) = _1;
_16 = [1331655806_u32,1087629061_u32,665419795_u32,846563252_u32,2288468015_u32,1301331296_u32,2133009499_u32,1979357714_u32];
_18.2.1 = (_8, Move(_4), _11, (*_26));
_32 = core::ptr::addr_of_mut!((*_26));
(*_32) = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
_4 = Move(_18.2.1.1);
_2 = core::ptr::addr_of!(_18.0);
(*_26) = _18.2.1.3;
(*_2) = _20 + _20;
_5 = core::ptr::addr_of_mut!((*_2));
(*_2) = -_20;
(*_26) = [_18.2.1.2,_11,_18.2.1.2,_18.2.1.2];
_31 = _1;
_18.2.2 = Adt25::Variant0 { fld0: (*_2),fld1: 982939424_u32 };
_12 = 68933131647572286348521688299192724671_i128 | 24180054280615694079054559247481174316_i128;
_30 = !53184_u16;
_18.2.1.2 = _11 << _6;
_1 = RET;
_18.2.2 = Adt25::Variant0 { fld0: (*_2),fld1: 1643399510_u32 };
place!(Field::<f64>(Variant(_18.2.2, 0), 0)) = 4_usize as f64;
(*_26) = _18.2.1.3;
match _13 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb12,
4 => bb13,
5 => bb14,
61 => bb16,
_ => bb15
}
}
bb12 = {
RET = (*_4);
_12 = !47037688844750246262737488373488002308_i128;
Goto(bb5)
}
bb13 = {
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
Goto(bb10)
}
bb14 = {
_11 = !18208_i16;
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
Goto(bb4)
}
bb15 = {
(*_4) = RET;
(*_4) = RET;
_18.2.0 = [_11,_11,_11,_11];
_1 = RET;
_14 = _13 as f32;
_18.2.1.1 = core::ptr::addr_of_mut!((*_4));
_18.2.1 = (_8, Move(_4), _11, _18.2.0);
_4 = core::ptr::addr_of_mut!(_1);
_18.2.0 = [_11,_11,_11,_11];
_24 = _14 + _14;
_5 = core::ptr::addr_of_mut!((*_2));
(*_4) = RET;
(*_5) = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
(*_2) = Field::<f64>(Variant(_19, 0), 0);
_3 = !_6;
RET = (*_4);
_18.0 = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
(*_2) = Field::<f64>(Variant(_19, 0), 0);
_5 = core::ptr::addr_of_mut!((*_2));
(*_2) = Field::<f64>(Variant(_19, 0), 0) - Field::<f64>(Variant(_19, 0), 0);
place!(Field::<*mut f64>(Variant(_19, 0), 1)) = core::ptr::addr_of_mut!((*_2));
(*_2) = 2595690792_u32 as f64;
(*_2) = Field::<f64>(Variant(_19, 0), 0);
(*_4) = RET;
(*_2) = Field::<f64>(Variant(_19, 0), 0);
match _13 {
6 => bb8,
_ => bb3
}
}
bb16 = {
_18.2.1.1 = Move(_4);
_18.2.1.3 = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
_22 = core::ptr::addr_of!(_13);
_4 = Move(_18.2.1.1);
(*_26) = _18.2.1.3;
_34 = [93_u8,207_u8,51_u8,97_u8,111_u8];
(*_26) = [_11,_18.2.1.2,_18.2.1.2,_18.2.1.2];
Goto(bb17)
}
bb17 = {
(*_2) = -_20;
_18.2.2 = Adt25::Variant0 { fld0: (*_2),fld1: 2595838446_u32 };
(*_26) = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
(*_2) = _30 as f64;
(*_2) = _20;
_36 = 3660830423_u32 as isize;
Call(_6 = core::intrinsics::transmute((*_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_2) = _20 + _20;
place!(Field::<f64>(Variant(_18.2.2, 0), 0)) = _12 as f64;
_38 = (*_2);
(*_22) = (-10_i8) * (-14_i8);
(*_2) = Field::<f64>(Variant(_18.2.2, 0), 0) + Field::<f64>(Variant(_18.2.2, 0), 0);
_34 = [247_u8,144_u8,106_u8,154_u8,50_u8];
(*_2) = -_38;
(*_26) = [_11,_18.2.1.2,_18.2.1.2,_18.2.1.2];
(*_26) = _18.2.1.3;
_36 = -_3;
_35 = !_6;
_29 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_18.2.2, 0), 1)));
_40 = core::ptr::addr_of_mut!(_1);
(*_26) = _18.2.1.3;
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_29) = _11 as u32;
(*_2) = 5_usize as f64;
_38 = (*_2);
(*_40) = RET;
_41 = (*_40);
_37 = false;
(*_29) = (-1986513771_i32) as u32;
_28 = &_18.2.2;
(*_40) = _41;
_43 = core::ptr::addr_of!(place!(Field::<f64>(Variant((*_28), 0), 0)));
Goto(bb19)
}
bb19 = {
_42 = [(-1091485291_i32)];
(*_29) = 2677632884_u32 & 2484528373_u32;
(*_40) = _31;
(*_26) = _18.2.1.3;
(*_40) = RET;
(*_22) = (*_29) as i8;
(*_29) = !1345691471_u32;
_32 = core::ptr::addr_of_mut!((*_26));
(*_29) = 2519461525_u32;
_27 = 264104551489025571256163413684689564633_u128 & 277624517223707233790877626151395786512_u128;
match (*_29) {
0 => bb5,
1 => bb12,
2519461525 => bb20,
_ => bb14
}
}
bb20 = {
(*_32) = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
_46 = 1744681505_i32 - 612157467_i32;
(*_29) = 3899274090_u32 * 3048361300_u32;
(*_40) = RET;
(*_32) = [_18.2.1.2,_18.2.1.2,_11,_18.2.1.2];
_46 = (-1566104926_i32) & (-1910764830_i32);
_1 = _41;
(*_29) = 11418451157477612733_u64 as u32;
_35 = _18.2.1.2 as isize;
_23 = &place!(Field::<f64>(Variant((*_28), 0), 0));
_38 = -Field::<f64>(Variant((*_28), 0), 0);
_31 = (*_40);
(*_2) = Field::<f64>(Variant(_18.2.2, 0), 0) * (*_43);
(*_22) = 56_i8 << _6;
(*_2) = Field::<f64>(Variant((*_28), 0), 0) - (*_43);
Goto(bb21)
}
bb21 = {
(*_22) = 11_i8 << _35;
_32 = core::ptr::addr_of_mut!((*_32));
(*_40) = RET;
(*_29) = 9055214613347026982_u64 as u32;
_34 = [235_u8,101_u8,22_u8,150_u8,30_u8];
_48 = core::ptr::addr_of!((*_22));
(*_48) = (-9_i8) - (-65_i8);
(*_32) = _18.2.1.3;
(*_2) = (*_23);
Goto(bb22)
}
bb22 = {
_30 = 6849_u16 ^ 17881_u16;
(*_48) = (-12_i8) ^ (-106_i8);
(*_32) = _18.2.1.3;
(*_2) = (*_43) * (*_43);
_49 = _35 >> (*_48);
_2 = Move(_43);
(*_22) = 98_i8 | (-3_i8);
(*_26) = _18.2.1.3;
(*_26) = _18.2.1.3;
_31 = (*_40);
_47 = (*_40);
(*_22) = (-58_i8) - 38_i8;
_29 = core::ptr::addr_of_mut!((*_29));
_18.2.1 = (_8, Move(_4), _11, (*_26));
Call((*_22) = fn19(_47, Move(_28), Field::<f64>(Variant((*_28), 0), 0), Move(_26), Move(_18.2.1), (*_40), (*_29)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_40) = RET;
_47 = (*_40);
_50 = [72_u8,220_u8];
_29 = core::ptr::addr_of_mut!((*_29));
(*_22) = 4_usize as i8;
place!(Field::<u32>(Variant(_18.2.2, 0), 1)) = _12 as u32;
_37 = false | true;
(*_40) = RET;
(*_29) = 531837263_u32 - 2379160726_u32;
Goto(bb24)
}
bb24 = {
(*_40) = RET;
(*_22) = !(-65_i8);
(*_22) = 14_i8;
_5 = core::ptr::addr_of_mut!((*_23));
(*_22) = _37 as i8;
_36 = -_35;
_37 = (*_22) < (*_22);
_18.2.1.2 = 188_u8 as i16;
(*_29) = (*_23) as u32;
(*_29) = !389163627_u32;
(*_22) = 68_i8;
(*_22) = (-120_i8);
_22 = core::ptr::addr_of!((*_22));
(*_22) = -117_i8;
_56 = [(*_40),(*_40),(*_40),(*_40)];
_8 = [_30,_30,_30,_30,_30];
(*_29) = 3889931925_u32;
_57 = &_18.2.2;
(*_22) = -(-38_i8);
_43 = core::ptr::addr_of!((*_5));
match (*_29) {
0 => bb20,
1 => bb17,
2 => bb15,
3 => bb4,
4 => bb23,
3889931925 => bb26,
_ => bb25
}
}
bb25 = {
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_40) = RET;
_47 = (*_40);
_50 = [72_u8,220_u8];
_29 = core::ptr::addr_of_mut!((*_29));
(*_22) = 4_usize as i8;
place!(Field::<u32>(Variant(_18.2.2, 0), 1)) = _12 as u32;
_37 = false | true;
(*_40) = RET;
(*_29) = 531837263_u32 - 2379160726_u32;
Goto(bb24)
}
bb26 = {
_46 = (*_43) as i32;
(*_22) = (-122_i8) >> (*_29);
(*_29) = (*_5) as u32;
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
_31 = (*_40);
(*_22) = (-20_i8) >> _12;
Goto(bb27)
}
bb27 = {
(*_29) = 3154790566_u32 >> (*_22);
_1 = RET;
_45 = _24;
(*_22) = 13_u8 as i8;
Goto(bb28)
}
bb28 = {
(*_22) = 4_usize as i8;
(*_29) = 1859507990_u32 | 3743526900_u32;
(*_22) = 10497387600846530490_u64 as i8;
_18.0 = (*_43);
(*_29) = !336120945_u32;
_52 = _37;
_18.2.1.3 = _18.2.0;
(*_22) = 87_u8 as i8;
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_22) = 16_i8;
(*_22) = (-30_i8) & (-116_i8);
(*_29) = 1817510704_u32 ^ 459652554_u32;
Goto(bb29)
}
bb29 = {
(*_40) = _47;
(*_22) = -66_i8;
_2 = Move(_43);
(*_29) = !2450479826_u32;
(*_40) = _47;
Goto(bb30)
}
bb30 = {
_60 = (*_5) + Field::<f64>(Variant((*_57), 0), 0);
(*_29) = 1223073681_u32 ^ 893114167_u32;
_30 = 1915_u16;
match _30 {
1915 => bb32,
_ => bb31
}
}
bb31 = {
_42 = [(-1091485291_i32)];
(*_29) = 2677632884_u32 & 2484528373_u32;
(*_40) = _31;
(*_26) = _18.2.1.3;
(*_40) = RET;
(*_22) = (*_29) as i8;
(*_29) = !1345691471_u32;
_32 = core::ptr::addr_of_mut!((*_26));
(*_29) = 2519461525_u32;
_27 = 264104551489025571256163413684689564633_u128 & 277624517223707233790877626151395786512_u128;
match (*_29) {
0 => bb5,
1 => bb12,
2519461525 => bb20,
_ => bb14
}
}
bb32 = {
_16 = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),Field::<u32>(Variant(_18.2.2, 0), 1),(*_29)];
(*_29) = 733844844_u32;
(*_40) = _47;
_23 = &_18.0;
_34 = [253_u8,135_u8,20_u8,53_u8,95_u8];
_1 = _47;
(*_29) = 250265865_u32;
(*_22) = (-47_i8) * 108_i8;
match (*_29) {
0 => bb33,
1 => bb34,
2 => bb35,
3 => bb36,
250265865 => bb38,
_ => bb37
}
}
bb33 = {
_11 = !18208_i16;
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
Goto(bb4)
}
bb34 = {
_8 = [44981_u16,29198_u16,29385_u16,41988_u16,37228_u16];
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
(*_4) = _1;
_4 = core::ptr::addr_of_mut!(_1);
_10 = 18075540975798213981878873775779459869_u128 as isize;
Goto(bb3)
}
bb35 = {
(*_2) = -_20;
_18.2.2 = Adt25::Variant0 { fld0: (*_2),fld1: 2595838446_u32 };
(*_26) = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
(*_2) = _30 as f64;
(*_2) = _20;
_36 = 3660830423_u32 as isize;
Call(_6 = core::intrinsics::transmute((*_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb36 = {
(*_22) = 4_usize as i8;
(*_29) = 1859507990_u32 | 3743526900_u32;
(*_22) = 10497387600846530490_u64 as i8;
_18.0 = (*_43);
(*_29) = !336120945_u32;
_52 = _37;
_18.2.1.3 = _18.2.0;
(*_22) = 87_u8 as i8;
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_22) = 16_i8;
(*_22) = (-30_i8) & (-116_i8);
(*_29) = 1817510704_u32 ^ 459652554_u32;
Goto(bb29)
}
bb37 = {
RET = (*_4);
_12 = !47037688844750246262737488373488002308_i128;
Goto(bb5)
}
bb38 = {
_33 = _49 | _6;
_29 = core::ptr::addr_of_mut!((*_29));
_37 = (*_22) > (*_22);
_67 = !_37;
_18.0 = Field::<f64>(Variant((*_57), 0), 0) + (*_5);
_61 = [_12,_12,_12,_12,_12,_12];
(*_29) = 2957576861_u32 | 3885713488_u32;
_68 = (*_5) < Field::<f64>(Variant((*_57), 0), 0);
_18.2.1.0 = [_30,_30,_30,_30,_30];
_31 = (*_40);
match _30 {
0 => bb4,
1 => bb2,
2 => bb37,
3 => bb39,
1915 => bb41,
_ => bb40
}
}
bb39 = {
(*_4) = RET;
_11 = 3088_i16 ^ (-9337_i16);
(*_4) = RET;
(*_4) = RET;
(*_4) = RET;
_3 = 1753891522_u32 as isize;
_2 = core::ptr::addr_of!(_18.0);
_13 = 6_i8;
_5 = core::ptr::addr_of_mut!((*_2));
(*_2) = 2850900271830363118_i64 as f64;
(*_2) = _10 as f64;
(*_2) = _14 as f64;
(*_2) = 10373165439785735615_u64 as f64;
(*_4) = RET;
(*_2) = _12 as f64;
(*_4) = RET;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = RET;
_23 = &(*_2);
(*_2) = _14 as f64;
_18.0 = 76480192_u32 as f64;
_21 = &_12;
_19 = Adt84::Variant0 { fld0: (*_2),fld1: Move(_5) };
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb7), UnwindUnreachable())
}
bb40 = {
(*_2) = Field::<f64>(Variant(_19, 0), 0);
Goto(bb9)
}
bb41 = {
_54 = -_14;
(*_29) = _30 as u32;
(*_22) = (-102_i8) & 76_i8;
_59 = Move(_18.2.2);
_22 = core::ptr::addr_of!((*_22));
RET = (*_40);
_26 = core::ptr::addr_of_mut!(_18.2.1.3);
_36 = 6591539080983695453_i64 as isize;
(*_26) = _18.2.0;
(*_29) = Field::<u32>(Variant(_59, 0), 1);
(*_29) = Field::<u32>(Variant(_59, 0), 1);
_64 = (*_22) != (*_22);
(*_40) = _31;
(*_29) = _52 as u32;
(*_40) = _31;
(*_26) = _18.2.0;
match _30 {
0 => bb6,
1 => bb9,
2 => bb24,
1915 => bb43,
_ => bb42
}
}
bb42 = {
(*_2) = Field::<f64>(Variant(_19, 0), 0);
Goto(bb9)
}
bb43 = {
_4 = core::ptr::addr_of_mut!((*_40));
_73 = _64;
(*_4) = RET;
_18.0 = _38 + _60;
(*_22) = 41_i8 + 70_i8;
_57 = &_59;
_33 = _46 as isize;
_75 = &mut _30;
(*_22) = 114_i8 ^ 63_i8;
(*_40) = _41;
_3 = _49 ^ _6;
_69 = _27 * _27;
_73 = _64 | _67;
place!(Field::<u32>(Variant(_59, 0), 1)) = (*_29);
_7 = core::ptr::addr_of!(_70);
_48 = core::ptr::addr_of!((*_22));
(*_40) = _41;
(*_7) = 13295953110471998197_u64 & 11513581065835852724_u64;
match (*_75) {
0 => bb24,
1 => bb2,
2 => bb13,
3 => bb44,
4 => bb45,
1915 => bb47,
_ => bb46
}
}
bb44 = {
_42 = [(-1091485291_i32)];
(*_29) = 2677632884_u32 & 2484528373_u32;
(*_40) = _31;
(*_26) = _18.2.1.3;
(*_40) = RET;
(*_22) = (*_29) as i8;
(*_29) = !1345691471_u32;
_32 = core::ptr::addr_of_mut!((*_26));
(*_29) = 2519461525_u32;
_27 = 264104551489025571256163413684689564633_u128 & 277624517223707233790877626151395786512_u128;
match (*_29) {
0 => bb5,
1 => bb12,
2519461525 => bb20,
_ => bb14
}
}
bb45 = {
_18.2.1.1 = Move(_4);
_18.2.1.3 = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
_22 = core::ptr::addr_of!(_13);
_4 = Move(_18.2.1.1);
(*_26) = _18.2.1.3;
_34 = [93_u8,207_u8,51_u8,97_u8,111_u8];
(*_26) = [_11,_18.2.1.2,_18.2.1.2,_18.2.1.2];
Goto(bb17)
}
bb46 = {
_18.2.1.1 = core::ptr::addr_of_mut!((*_40));
(*_40) = RET;
_47 = (*_40);
_50 = [72_u8,220_u8];
_29 = core::ptr::addr_of_mut!((*_29));
(*_22) = 4_usize as i8;
place!(Field::<u32>(Variant(_18.2.2, 0), 1)) = _12 as u32;
_37 = false | true;
(*_40) = RET;
(*_29) = 531837263_u32 - 2379160726_u32;
Goto(bb24)
}
bb47 = {
(*_48) = 108_i8 * (-2_i8);
(*_48) = 19_i8 >> (*_75);
(*_40) = _47;
(*_29) = Field::<u32>(Variant(_59, 0), 1);
(*_22) = (-58_i8);
_72 = core::ptr::addr_of!((*_7));
(*_22) = (-36_i8);
(*_7) = !5639734606122194579_u64;
(*_22) = 64_i8 & 97_i8;
match (*_75) {
0 => bb1,
1 => bb45,
2 => bb34,
1915 => bb48,
_ => bb28
}
}
bb48 = {
RET = (*_40);
(*_75) = 14497_u16;
(*_75) = (*_40) as u16;
(*_40) = _41;
(*_26) = [_11,_18.2.1.2,_11,_11];
_71 = &mut (*_75);
(*_40) = RET;
(*_22) = 72_i8;
(*_26) = [_18.2.1.2,_11,_11,_18.2.1.2];
(*_71) = 8413_u16 * 7841_u16;
_12 = (-63608149441990662106286548443965319834_i128) & 94190084690399302774267579074787415834_i128;
_36 = _3 | _6;
_2 = core::ptr::addr_of!(place!(Field::<f64>(Variant((*_57), 0), 0)));
(*_71) = _45 as u16;
(*_7) = 9615728785673400643_u64 - 178853463863056253_u64;
(*_22) = 48_i8 - (-80_i8);
_34 = [215_u8,101_u8,231_u8,237_u8,240_u8];
(*_71) = 26365_u16;
(*_29) = Field::<u32>(Variant(_59, 0), 1);
_71 = Move(_75);
(*_26) = _18.2.0;
_2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_59, 0), 0)));
Goto(bb49)
}
bb49 = {
(*_22) = 1_i8 ^ (-91_i8);
(*_29) = _64 as u32;
(*_22) = -40_i8;
_18.2.1.2 = _11 + _11;
(*_22) = (-45_i8) << (*_29);
_67 = _52 | _73;
_78 = [_12,_12,_12,_12,_12];
_35 = -_3;
(*_26) = [_18.2.1.2,_18.2.1.2,_18.2.1.2,_18.2.1.2];
(*_26) = [_18.2.1.2,_11,_18.2.1.2,_11];
(*_29) = Field::<u32>(Variant(_59, 0), 1) + Field::<u32>(Variant(_59, 0), 1);
(*_7) = 3991314459238047098_u64 ^ 10685010918755985792_u64;
(*_29) = Field::<u32>(Variant(_59, 0), 1) << _3;
(*_26) = [_11,_18.2.1.2,_18.2.1.2,_11];
(*_29) = Field::<u32>(Variant(_59, 0), 1);
(*_7) = 3059003997655588407_u64;
Goto(bb50)
}
bb50 = {
Call(_83 = dump_var(Move(_1), Move(_78), Move(_34), Move(_56)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_83 = dump_var(Move(_11), Move(_68), Move(_10), Move(_8)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_83 = dump_var(Move(_70), Move(_41), Move(_49), Move(_12)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(Move(_30), Move(_37), Move(_33), Move(_16)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut char,mut _2: *mut f64,mut _3: *const f64,mut _4: char,mut _5: char,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: char) -> isize {
mir! {
type RET = isize;
let _11: [u8; 5];
let _12: [i128; 5];
let _13: i64;
let _14: ();
let _15: ();
{
_9 = !_7;
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!(_10);
_10 = _5;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _5;
(*_1) = _5;
(*_1) = _5;
_4 = (*_1);
(*_1) = _5;
RET = _7 << _6;
(*_1) = _5;
(*_1) = _5;
(*_1) = _5;
(*_1) = _5;
(*_1) = _4;
(*_1) = _5;
(*_1) = _4;
(*_1) = _4;
(*_1) = _5;
(*_1) = _4;
(*_1) = _5;
(*_1) = _4;
(*_1) = _4;
(*_1) = _4;
(*_1) = _4;
(*_1) = _5;
_5 = (*_1);
(*_1) = _5;
(*_1) = _5;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(Move(_9), Move(_6), Move(_5), _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: ([u16; 5], *mut char, i16, [i16; 4]),mut _2: Adt84,mut _3: char,mut _4: [u16; 5],mut _5: char,mut _6: i128,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: *const f64,mut _12: *mut f64) -> f64 {
mir! {
type RET = f64;
let _13: isize;
let _14: f32;
let _15: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _16: &'static f64;
let _17: f64;
let _18: char;
let _19: f64;
let _20: *mut f64;
let _21: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _22: &'static &'static (i32, (usize, u128, f32), f64);
let _23: u16;
let _24: i16;
let _25: i32;
let _26: isize;
let _27: f64;
let _28: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _29: &'static i128;
let _30: char;
let _31: u128;
let _32: &'static [i128; 4];
let _33: i32;
let _34: f64;
let _35: char;
let _36: *mut u32;
let _37: i8;
let _38: f32;
let _39: ();
let _40: ();
{
_10 = _3;
_9 = _5;
_8 = _3;
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_2, 0), 0)));
_1.2 = 32220_i16;
_13 = -24_isize;
_7 = _5;
_14 = 783405322620473643_i64 as f32;
Call(place!(Field::<f64>(Variant(_2, 0), 0)) = fn13(Move(_12), Move(_1), Move(_11), _3, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = 18001541519573653296_u64 as f32;
_7 = _10;
_13 = 94_isize & (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_1.3 = [(-30119_i16),(-21580_i16),6720_i16,13161_i16];
RET = Field::<f64>(Variant(_2, 0), 0) * Field::<f64>(Variant(_2, 0), 0);
_7 = _10;
_4 = [29546_u16,57493_u16,4097_u16,39651_u16,22609_u16];
_10 = _3;
_5 = _9;
_14 = _6 as f32;
_6 = (-137137039797722650827621743904025121862_i128) - (-3864063277276831269311588512773611816_i128);
RET = Field::<f64>(Variant(_2, 0), 0);
_6 = -(-153378169753905218006369043235497087223_i128);
_7 = _8;
_3 = _10;
_16 = &RET;
_14 = (-15639_i16) as f32;
_7 = _9;
_3 = _10;
Goto(bb3)
}
bb3 = {
_4 = [63334_u16,58836_u16,15481_u16,27768_u16,51250_u16];
_1.0 = _4;
_1.2 = 12810_i16;
_1.1 = core::ptr::addr_of_mut!(_5);
_1.2 = -9704_i16;
place!(Field::<f64>(Variant(_2, 0), 0)) = -(*_16);
_1.3 = [_1.2,_1.2,_1.2,_1.2];
_13 = true as isize;
_16 = &place!(Field::<f64>(Variant(_2, 0), 0));
_16 = &RET;
_12 = core::ptr::addr_of_mut!((*_16));
_5 = _8;
_1.2 = 20611_i16 >> _13;
_5 = _10;
_19 = (*_12) - (*_16);
_9 = _5;
_18 = _10;
Goto(bb4)
}
bb4 = {
_23 = 29178_u16;
_1.0 = [_23,_23,_23,_23,_23];
_1.2 = 28784_i16;
place!(Field::<f64>(Variant(_2, 0), 0)) = _13 as f64;
_14 = 254_u8 as f32;
_24 = _1.2;
_5 = _10;
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!((*_12));
_24 = _1.2 + _1.2;
_25 = 1680551644_i32;
Goto(bb5)
}
bb5 = {
_13 = 26_isize - 34_isize;
_24 = !_1.2;
_27 = 3162266030_u32 as f64;
_8 = _9;
_1.1 = core::ptr::addr_of_mut!(_18);
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!((*_12));
_1.1 = core::ptr::addr_of_mut!(_3);
_10 = _7;
Call(_26 = core::intrinsics::bswap(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = [_23,_23,_23,_23,_23];
_17 = -(*_12);
match _1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
28784 => bb10,
_ => bb9
}
}
bb7 = {
_13 = 26_isize - 34_isize;
_24 = !_1.2;
_27 = 3162266030_u32 as f64;
_8 = _9;
_1.1 = core::ptr::addr_of_mut!(_18);
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!((*_12));
_1.1 = core::ptr::addr_of_mut!(_3);
_10 = _7;
Call(_26 = core::intrinsics::bswap(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_23 = 29178_u16;
_1.0 = [_23,_23,_23,_23,_23];
_1.2 = 28784_i16;
place!(Field::<f64>(Variant(_2, 0), 0)) = _13 as f64;
_14 = 254_u8 as f32;
_24 = _1.2;
_5 = _10;
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!((*_12));
_24 = _1.2 + _1.2;
_25 = 1680551644_i32;
Goto(bb5)
}
bb9 = {
_14 = 18001541519573653296_u64 as f32;
_7 = _10;
_13 = 94_isize & (-9223372036854775808_isize);
Goto(bb2)
}
bb10 = {
place!(Field::<f64>(Variant(_2, 0), 0)) = (*_16) * _27;
_27 = (*_12);
_1.3 = [_24,_24,_24,_1.2];
_28.1 = [_9,_10,_8,_9,_5,_18,_10,_9];
_9 = _5;
Goto(bb11)
}
bb11 = {
_28.0.0 = [_1.2,_24,_24,_1.2];
_27 = (*_12) - (*_12);
_19 = (*_12);
_1.2 = _24;
_3 = _8;
_28.1 = [_8,_7,_8,_9,_7,_10,_3,_5];
RET = _27 + _17;
Goto(bb12)
}
bb12 = {
_7 = _9;
_31 = 73502041288780014405640390148721634601_u128 & 104978839198915098128701428333928146717_u128;
_6 = (-131999007678844703863374281605459460371_i128) & (-50146867262019421111522317829017814882_i128);
_18 = _9;
place!(Field::<f64>(Variant(_2, 0), 0)) = RET;
_14 = _24 as f32;
match _25 {
0 => bb13,
1 => bb14,
2 => bb15,
1680551644 => bb17,
_ => bb16
}
}
bb13 = {
_28.0.0 = [_1.2,_24,_24,_1.2];
_27 = (*_12) - (*_12);
_19 = (*_12);
_1.2 = _24;
_3 = _8;
_28.1 = [_8,_7,_8,_9,_7,_10,_3,_5];
RET = _27 + _17;
Goto(bb12)
}
bb14 = {
_4 = [63334_u16,58836_u16,15481_u16,27768_u16,51250_u16];
_1.0 = _4;
_1.2 = 12810_i16;
_1.1 = core::ptr::addr_of_mut!(_5);
_1.2 = -9704_i16;
place!(Field::<f64>(Variant(_2, 0), 0)) = -(*_16);
_1.3 = [_1.2,_1.2,_1.2,_1.2];
_13 = true as isize;
_16 = &place!(Field::<f64>(Variant(_2, 0), 0));
_16 = &RET;
_12 = core::ptr::addr_of_mut!((*_16));
_5 = _8;
_1.2 = 20611_i16 >> _13;
_5 = _10;
_19 = (*_12) - (*_16);
_9 = _5;
_18 = _10;
Goto(bb4)
}
bb15 = {
_14 = 18001541519573653296_u64 as f32;
_7 = _10;
_13 = 94_isize & (-9223372036854775808_isize);
Goto(bb2)
}
bb16 = {
_13 = 26_isize - 34_isize;
_24 = !_1.2;
_27 = 3162266030_u32 as f64;
_8 = _9;
_1.1 = core::ptr::addr_of_mut!(_18);
place!(Field::<*mut f64>(Variant(_2, 0), 1)) = core::ptr::addr_of_mut!((*_12));
_1.1 = core::ptr::addr_of_mut!(_3);
_10 = _7;
Call(_26 = core::intrinsics::bswap(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb17 = {
_5 = _18;
_30 = _7;
_2 = Adt84::Variant0 { fld0: RET,fld1: Move(_12) };
_33 = _25 * _25;
_11 = core::ptr::addr_of!(_34);
(*_11) = _27;
_28.0.1.1 = core::ptr::addr_of_mut!(_8);
(*_11) = Field::<f64>(Variant(_2, 0), 0) * _27;
_24 = _1.2 ^ _1.2;
(*_11) = Field::<f64>(Variant(_2, 0), 0);
_23 = 31761_u16 - 25982_u16;
(*_11) = -_27;
_10 = _8;
Call((*_11) = core::intrinsics::transmute(_26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
place!(Field::<f64>(Variant(_2, 0), 0)) = RET;
(*_11) = RET;
_1.1 = core::ptr::addr_of_mut!(_18);
_4 = _1.0;
_1.1 = core::ptr::addr_of_mut!(_30);
_1 = (_4, Move(_28.0.1.1), _24, _28.0.0);
_17 = (*_11) + (*_11);
_23 = (*_11) as u16;
_28.0.1 = (_4, Move(_1.1), _24, _28.0.0);
_37 = _23 as i8;
_28.0.2 = Adt25::Variant0 { fld0: (*_11),fld1: 4258985383_u32 };
_37 = (-64_i8);
_11 = core::ptr::addr_of!((*_11));
(*_11) = _17;
match _25 {
1680551644 => bb20,
_ => bb19
}
}
bb19 = {
_7 = _9;
_31 = 73502041288780014405640390148721634601_u128 & 104978839198915098128701428333928146717_u128;
_6 = (-131999007678844703863374281605459460371_i128) & (-50146867262019421111522317829017814882_i128);
_18 = _9;
place!(Field::<f64>(Variant(_2, 0), 0)) = RET;
_14 = _24 as f32;
match _25 {
0 => bb13,
1 => bb14,
2 => bb15,
1680551644 => bb17,
_ => bb16
}
}
bb20 = {
_18 = _30;
place!(Field::<f64>(Variant(_2, 0), 0)) = (*_11) + (*_11);
RET = (*_11);
_7 = _18;
_37 = 77_i8 << _23;
_29 = &_6;
(*_11) = RET + _17;
_13 = 100_isize ^ 9223372036854775807_isize;
_25 = _33;
_36 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_28.0.2, 0), 1)));
Goto(bb21)
}
bb21 = {
Call(_39 = dump_var(Move(_6), Move(_18), Move(_33), Move(_9)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(Move(_8), Move(_37), Move(_4), Move(_26)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_39 = dump_var(Move(_13), _40, _40, _40), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut f64,mut _2: ([u16; 5], *mut char, i16, [i16; 4]),mut _3: *const f64,mut _4: char,mut _5: char) -> f64 {
mir! {
type RET = f64;
let _6: (usize, u128, f32);
let _7: f64;
let _8: &'static &'static (i32, (usize, u128, f32), f64);
let _9: char;
let _10: i32;
let _11: i64;
let _12: [i32; 4];
let _13: f64;
let _14: &'static mut Adt42;
let _15: char;
let _16: f32;
let _17: ([u16; 5], *mut char, i16, [i16; 4]);
let _18: [u64; 1];
let _19: i32;
let _20: ([i128; 4], &'static [char; 4]);
let _21: u64;
let _22: Adt42;
let _23: u32;
let _24: [u128; 5];
let _25: *mut [i16; 4];
let _26: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _27: f64;
let _28: isize;
let _29: *mut [i16; 4];
let _30: i8;
let _31: Adt68;
let _32: [i16; 4];
let _33: f64;
let _34: isize;
let _35: f32;
let _36: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _37: *mut f64;
let _38: f32;
let _39: char;
let _40: i128;
let _41: *const f64;
let _42: i128;
let _43: i16;
let _44: bool;
let _45: char;
let _46: [i32; 4];
let _47: char;
let _48: Adt25;
let _49: isize;
let _50: &'static f64;
let _51: isize;
let _52: (&'static (i32, (usize, u128, f32), f64), f32);
let _53: [i128; 6];
let _54: char;
let _55: [i32; 4];
let _56: &'static [i128; 4];
let _57: u8;
let _58: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _59: *const [char; 4];
let _60: [i64; 7];
let _61: [i32; 4];
let _62: *const u64;
let _63: *mut u32;
let _64: *mut f64;
let _65: u8;
let _66: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _67: u16;
let _68: f64;
let _69: u8;
let _70: f32;
let _71: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _72: u16;
let _73: *const [char; 4];
let _74: bool;
let _75: [u8; 4];
let _76: isize;
let _77: &'static Adt25;
let _78: isize;
let _79: i16;
let _80: i64;
let _81: [u64; 7];
let _82: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _83: [u8; 2];
let _84: isize;
let _85: usize;
let _86: isize;
let _87: Adt25;
let _88: bool;
let _89: *mut f64;
let _90: [u32; 8];
let _91: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _92: Adt42;
let _93: *const [usize; 3];
let _94: u64;
let _95: [i128; 5];
let _96: ();
let _97: ();
{
_2.2 = 30873_u16 as i16;
_4 = _5;
RET = 5776_u16 as f64;
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 1098615539_i32 as f64;
_2.2 = (-1173_i16);
RET = 44_u8 as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_6.0 = !14296255888820680421_usize;
_2.3 = [_2.2,_2.2,_2.2,_2.2];
(*_1) = 21986381111648101178893519463746984831_i128 as f64;
_1 = core::ptr::addr_of_mut!(_7);
_7 = RET * RET;
_6.2 = (-14_isize) as f32;
RET = -(*_1);
(*_1) = -RET;
_4 = _5;
(*_1) = 9223372036854775807_isize as f64;
(*_1) = RET - RET;
(*_1) = -RET;
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768210283 => bb7,
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
(*_1) = RET;
(*_1) = RET;
(*_1) = RET + RET;
(*_1) = RET * RET;
(*_1) = RET;
_9 = _5;
(*_1) = RET;
_2.3 = [_2.2,_2.2,_2.2,_2.2];
_6.1 = !241106653917228646598995031279016107014_u128;
(*_1) = RET + RET;
_5 = _9;
(*_1) = RET;
RET = (*_1);
_15 = _5;
(*_1) = RET;
(*_1) = RET + RET;
_12 = [(-1462854821_i32),(-848683278_i32),(-1489651896_i32),106192595_i32];
match _2.2 {
0 => bb1,
340282366920938463463374607431768210283 => bb8,
_ => bb4
}
}
bb8 = {
(*_1) = RET;
(*_1) = 54912_u16 as f64;
(*_1) = -RET;
_3 = core::ptr::addr_of!((*_1));
(*_3) = 66_i8 as f64;
_18 = [9898949470257739313_u64];
(*_1) = (-9223372036854775808_isize) as f64;
(*_1) = RET;
_5 = _4;
(*_1) = RET;
match _2.2 {
0 => bb7,
1 => bb9,
2 => bb10,
340282366920938463463374607431768210283 => bb12,
_ => bb11
}
}
bb9 = {
(*_1) = RET;
(*_1) = RET;
(*_1) = RET + RET;
(*_1) = RET * RET;
(*_1) = RET;
_9 = _5;
(*_1) = RET;
_2.3 = [_2.2,_2.2,_2.2,_2.2];
_6.1 = !241106653917228646598995031279016107014_u128;
(*_1) = RET + RET;
_5 = _9;
(*_1) = RET;
RET = (*_1);
_15 = _5;
(*_1) = RET;
(*_1) = RET + RET;
_12 = [(-1462854821_i32),(-848683278_i32),(-1489651896_i32),106192595_i32];
match _2.2 {
0 => bb1,
340282366920938463463374607431768210283 => bb8,
_ => bb4
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_17 = (_2.0, Move(_2.1), _2.2, _2.3);
_9 = _5;
(*_1) = RET + RET;
_2 = (_17.0, Move(_17.1), _17.2, _17.3);
_20.0 = [134962017053997254619722854717848994379_i128,(-96718049558498900921074784284994550101_i128),152811289466862937003375993989489188007_i128,88395643831863436321805234081874095538_i128];
_1 = core::ptr::addr_of_mut!((*_1));
_11 = (-49614598186807018_i64);
_12 = [1909862671_i32,2126358089_i32,(-1582065036_i32),(-207822074_i32)];
_6.2 = 1521021064077488653_u64 as f32;
_4 = _15;
_12 = [(-1158454254_i32),(-1563029104_i32),229984128_i32,(-663692279_i32)];
_16 = _6.2 * _6.2;
match _11 {
340282366920938463463324992833581404438 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_10 = 1063581046_i32;
_7 = RET - RET;
_7 = RET;
(*_1) = RET;
_18 = [10113559523334841539_u64];
_2.1 = core::ptr::addr_of_mut!(_9);
(*_1) = RET + RET;
(*_1) = RET * RET;
(*_1) = _17.2 as f64;
_6 = (2_usize, 294257426614099815938745176324372502316_u128, _16);
_6 = (1909605135367620292_usize, 318938835119192181591291215269122560141_u128, _16);
_10 = (-1053685067_i32) ^ (-1245185784_i32);
_7 = -RET;
_21 = _6.1 as u64;
_11 = 676473032713093324_i64;
_2.0 = _17.0;
(*_1) = RET;
_18 = [_21];
_2.3 = _17.3;
(*_1) = _11 as f64;
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_25 = core::ptr::addr_of_mut!(_2.3);
_17.2 = _6.2 as i16;
match _6.0 {
0 => bb11,
1 => bb4,
2 => bb15,
3 => bb16,
4 => bb17,
1909605135367620292 => bb19,
_ => bb18
}
}
bb15 = {
(*_1) = RET;
(*_1) = RET;
(*_1) = RET + RET;
(*_1) = RET * RET;
(*_1) = RET;
_9 = _5;
(*_1) = RET;
_2.3 = [_2.2,_2.2,_2.2,_2.2];
_6.1 = !241106653917228646598995031279016107014_u128;
(*_1) = RET + RET;
_5 = _9;
(*_1) = RET;
RET = (*_1);
_15 = _5;
(*_1) = RET;
(*_1) = RET + RET;
_12 = [(-1462854821_i32),(-848683278_i32),(-1489651896_i32),106192595_i32];
match _2.2 {
0 => bb1,
340282366920938463463374607431768210283 => bb8,
_ => bb4
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
(*_1) = RET - RET;
(*_1) = RET + RET;
_10 = 962592290_i32;
_21 = 11185245180820301863_u64 << _17.2;
_4 = _15;
_17 = (_2.0, Move(_2.1), _2.2, (*_25));
(*_1) = RET + RET;
(*_25) = [_17.2,_2.2,_2.2,_2.2];
(*_25) = [_17.2,_2.2,_2.2,_17.2];
Goto(bb20)
}
bb20 = {
(*_1) = RET;
_17.3 = [_2.2,_17.2,_2.2,_17.2];
_19 = !_10;
(*_25) = [_17.2,_17.2,_2.2,_17.2];
(*_25) = [_2.2,_17.2,_17.2,_17.2];
(*_25) = [_2.2,_17.2,_17.2,_17.2];
_23 = 2980223128_u32;
(*_1) = _6.1 as f64;
_25 = core::ptr::addr_of_mut!((*_25));
_18 = [_21];
_6.0 = 0_usize;
(*_1) = RET;
_21 = 15497493844871994541_u64;
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_25) = _17.3;
_1 = core::ptr::addr_of_mut!((*_1));
(*_25) = [_2.2,_2.2,_2.2,_17.2];
_30 = _6.0 as i8;
_6.0 = 7_usize >> _19;
_31.fld0.1 = (_2.0, Move(_17.1), _17.2, (*_25));
_15 = _4;
_30 = !(-36_i8);
(*_25) = [_17.2,_2.2,_31.fld0.1.2,_31.fld0.1.2];
_21 = !4100046011862825078_u64;
_16 = _6.2 - _6.2;
(*_25) = [_17.2,_2.2,_17.2,_17.2];
Call((*_1) = fn14((*_25), _2.2, Move(_31.fld0.1), _24), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
(*_1) = -RET;
_2.3 = [_17.2,_2.2,_2.2,_17.2];
(*_25) = [_2.2,_17.2,_2.2,_17.2];
(*_25) = [_17.2,_2.2,_17.2,_17.2];
_17.3 = (*_25);
(*_25) = [_17.2,_2.2,_2.2,_2.2];
(*_1) = _11 as f64;
_10 = _15 as i32;
(*_1) = RET - RET;
_13 = (*_1);
(*_1) = _13 * _13;
_18 = [_21];
_17.3 = [_2.2,_2.2,_17.2,_2.2];
(*_1) = -RET;
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_1) = (-4840978080384181222228807688852437088_i128) as f64;
(*_1) = _13 + RET;
_30 = _6.1 as i8;
_17.2 = _2.2;
Call((*_25) = core::intrinsics::transmute(_6.0), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Goto(bb23)
}
bb23 = {
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_25 = core::ptr::addr_of_mut!((*_25));
(*_1) = RET + _13;
_29 = core::ptr::addr_of_mut!((*_25));
_28 = (-9223372036854775808_isize);
_13 = _23 as f64;
_35 = -_6.2;
_13 = (*_1) * (*_1);
_34 = _28 - _28;
(*_1) = RET;
(*_1) = _13 * _13;
_3 = core::ptr::addr_of!(_13);
_3 = core::ptr::addr_of!((*_3));
Goto(bb24)
}
bb24 = {
_17.3 = (*_29);
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_3) = (*_1);
_43 = _2.2 ^ _2.2;
_31.fld0.1.1 = core::ptr::addr_of_mut!(_45);
_32 = [_43,_2.2,_43,_2.2];
(*_1) = (*_3) + _13;
_17.3 = (*_29);
(*_29) = [_17.2,_43,_43,_43];
(*_3) = (*_1);
_18 = [_21];
Goto(bb25)
}
bb25 = {
(*_25) = [_2.2,_43,_43,_43];
_39 = _15;
_34 = -_28;
(*_25) = [_17.2,_17.2,_43,_43];
_30 = 58_u8 as i8;
(*_1) = -(*_3);
(*_25) = [_43,_2.2,_17.2,_43];
_2.3 = _17.3;
_41 = core::ptr::addr_of!((*_3));
(*_41) = _34 as f64;
_31.fld0.1.3 = [_43,_43,_43,_2.2];
(*_3) = _7;
(*_3) = (*_1);
_31.fld0.0 = [_43,_43,_17.2,_43];
_23 = 2803870303_u32 & 2186833308_u32;
Goto(bb26)
}
bb26 = {
_13 = _7 + (*_1);
_2.2 = _17.2;
(*_3) = (*_1) + (*_1);
_50 = &(*_1);
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_3) = 9076_u16 as f64;
_2.3 = [_43,_2.2,_17.2,_43];
_52.1 = _6.2 - _16;
_49 = _34 * _34;
(*_3) = (*_1);
_31.fld1 = [29_u8,79_u8,233_u8,113_u8];
(*_1) = (*_3) - (*_3);
(*_3) = (*_1) * (*_1);
_44 = true ^ true;
_31.fld2 = Move(_41);
(*_25) = [_17.2,_17.2,_17.2,_17.2];
_33 = (*_3) - (*_1);
_20.0 = [(-85641483029626928285026856308576654170_i128),(-67963582552039895764707594974293390453_i128),(-42508199517601815595210185199838584773_i128),167969262946378980099181570187244271635_i128];
Goto(bb27)
}
bb27 = {
(*_1) = 191_u8 as f64;
_53 = [66683023549388883174500839493822373008_i128,(-137121756262648390015169953265834801249_i128),80614261476102951219386724560607641330_i128,115206761234546888999235941989934896265_i128,(-112801634392464585329693196736206578572_i128),(-6051865019076557211015528040488507546_i128)];
match _6.1 {
0 => bb9,
1 => bb4,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
318938835119192181591291215269122560141 => bb34,
_ => bb33
}
}
bb28 = {
_13 = _7 + (*_1);
_2.2 = _17.2;
(*_3) = (*_1) + (*_1);
_50 = &(*_1);
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_3) = 9076_u16 as f64;
_2.3 = [_43,_2.2,_17.2,_43];
_52.1 = _6.2 - _16;
_49 = _34 * _34;
(*_3) = (*_1);
_31.fld1 = [29_u8,79_u8,233_u8,113_u8];
(*_1) = (*_3) - (*_3);
(*_3) = (*_1) * (*_1);
_44 = true ^ true;
_31.fld2 = Move(_41);
(*_25) = [_17.2,_17.2,_17.2,_17.2];
_33 = (*_3) - (*_1);
_20.0 = [(-85641483029626928285026856308576654170_i128),(-67963582552039895764707594974293390453_i128),(-42508199517601815595210185199838584773_i128),167969262946378980099181570187244271635_i128];
Goto(bb27)
}
bb29 = {
(*_1) = RET;
(*_1) = RET;
(*_1) = RET + RET;
(*_1) = RET * RET;
(*_1) = RET;
_9 = _5;
(*_1) = RET;
_2.3 = [_2.2,_2.2,_2.2,_2.2];
_6.1 = !241106653917228646598995031279016107014_u128;
(*_1) = RET + RET;
_5 = _9;
(*_1) = RET;
RET = (*_1);
_15 = _5;
(*_1) = RET;
(*_1) = RET + RET;
_12 = [(-1462854821_i32),(-848683278_i32),(-1489651896_i32),106192595_i32];
match _2.2 {
0 => bb1,
340282366920938463463374607431768210283 => bb8,
_ => bb4
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
Return()
}
bb34 = {
_54 = _15;
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_4 = _9;
(*_25) = _31.fld0.0;
(*_25) = _17.3;
_31.fld1 = [10_u8,110_u8,74_u8,235_u8];
_40 = (-157988294144712543779577030443013854759_i128);
_17.1 = core::ptr::addr_of_mut!(_47);
match _6.1 {
0 => bb27,
318938835119192181591291215269122560141 => bb36,
_ => bb35
}
}
bb35 = {
(*_25) = [_2.2,_43,_43,_43];
_39 = _15;
_34 = -_28;
(*_25) = [_17.2,_17.2,_43,_43];
_30 = 58_u8 as i8;
(*_1) = -(*_3);
(*_25) = [_43,_2.2,_17.2,_43];
_2.3 = _17.3;
_41 = core::ptr::addr_of!((*_3));
(*_41) = _34 as f64;
_31.fld0.1.3 = [_43,_43,_43,_2.2];
(*_3) = _7;
(*_3) = (*_1);
_31.fld0.0 = [_43,_43,_17.2,_43];
_23 = 2803870303_u32 & 2186833308_u32;
Goto(bb26)
}
bb36 = {
(*_25) = [_2.2,_43,_43,_43];
(*_25) = [_2.2,_2.2,_17.2,_43];
_31.fld0.1 = Move(_17);
_17.1 = core::ptr::addr_of_mut!(_54);
_42 = _40;
_2.1 = core::ptr::addr_of_mut!(_5);
_16 = _6.1 as f32;
(*_25) = _31.fld0.1.3;
_31.fld0.2 = Adt25::Variant0 { fld0: (*_3),fld1: _23 };
(*_25) = _31.fld0.0;
_23 = Field::<u32>(Variant(_31.fld0.2, 0), 1) >> _6.0;
(*_25) = _31.fld0.1.3;
_9 = _15;
_36 = &_2;
(*_1) = -(*_3);
(*_1) = (*_3) + (*_3);
_61 = [_19,_10,_10,_10];
(*_25) = _31.fld0.0;
(*_1) = (*_3) * _33;
_46 = _12;
match (*_36).2 {
0 => bb37,
340282366920938463463374607431768210283 => bb39,
_ => bb38
}
}
bb37 = {
Return()
}
bb38 = {
Return()
}
bb39 = {
(*_1) = (*_3) + (*_3);
_2.1 = core::ptr::addr_of_mut!(_15);
(*_1) = (*_3) - _33;
(*_3) = (*_1);
(*_1) = (*_3) + Field::<f64>(Variant(_31.fld0.2, 0), 0);
_53 = [_42,_42,_42,_42,_40,_40];
(*_1) = (*_3);
_45 = _54;
_5 = _54;
_48 = Move(_31.fld0.2);
(*_25) = [(*_36).2,(*_36).2,(*_36).2,(*_36).2];
Goto(bb40)
}
bb40 = {
_60 = [_11,_11,_11,_11,_11,_11,_11];
(*_25) = [(*_36).2,(*_36).2,(*_36).2,_2.2];
(*_3) = _23 as f64;
_31.fld2 = core::ptr::addr_of!((*_3));
_34 = -_49;
_31.fld0.1.2 = _2.2 + (*_36).2;
_46 = [_10,_10,_19,_19];
(*_3) = 4_u8 as f64;
_17.0 = [41558_u16,23695_u16,5611_u16,37917_u16,13077_u16];
_68 = (*_1) * (*_1);
_32 = [(*_36).2,(*_36).2,(*_36).2,(*_36).2];
Call((*_1) = core::intrinsics::fmaf64((*_3), Field::<f64>(Variant(_48, 0), 0), _68), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
(*_3) = _21 as f64;
(*_3) = (*_1) * (*_1);
(*_1) = (*_3);
_63 = core::ptr::addr_of_mut!(_23);
(*_1) = (*_3) * (*_3);
(*_3) = _7;
_72 = 26785_u16 - 28563_u16;
(*_1) = _30 as f64;
(*_25) = [(*_36).2,(*_36).2,_2.2,(*_36).2];
match (*_36).2 {
0 => bb28,
1 => bb39,
2 => bb20,
3 => bb18,
4 => bb14,
5 => bb42,
6 => bb43,
340282366920938463463374607431768210283 => bb45,
_ => bb44
}
}
bb42 = {
Return()
}
bb43 = {
Return()
}
bb44 = {
Return()
}
bb45 = {
_24 = [_6.1,_6.1,_6.1,_6.1,_6.1];
(*_3) = (*_1);
(*_25) = [(*_36).2,(*_36).2,(*_36).2,(*_36).2];
_10 = _19;
(*_25) = _31.fld0.0;
_75 = _31.fld1;
(*_63) = Field::<u32>(Variant(_48, 0), 1) - Field::<u32>(Variant(_48, 0), 1);
(*_25) = _31.fld0.1.3;
(*_25) = _31.fld0.1.3;
_43 = _30 as i16;
Call(_31.fld0.1 = fn17((*_3), Move(_25), (*_36).2, Move(_17.1), Move(_3), Move(_2)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_2.3 = _31.fld0.1.3;
_37 = core::ptr::addr_of_mut!((*_1));
(*_1) = -_33;
_63 = core::ptr::addr_of_mut!((*_63));
(*_1) = -_33;
_2.1 = core::ptr::addr_of_mut!(_5);
(*_63) = Field::<u32>(Variant(_48, 0), 1) * Field::<u32>(Variant(_48, 0), 1);
(*_1) = _31.fld0.1.2 as f64;
_17.1 = core::ptr::addr_of_mut!(_4);
_2 = (_17.0, Move(_31.fld0.1.1), _31.fld0.1.2, _31.fld0.1.3);
(*_63) = Field::<u32>(Variant(_48, 0), 1) + Field::<u32>(Variant(_48, 0), 1);
_31.fld0.1.2 = !_2.2;
_76 = _2.2 as isize;
Goto(bb47)
}
bb47 = {
(*_63) = Field::<u32>(Variant(_48, 0), 1) ^ Field::<u32>(Variant(_48, 0), 1);
_2.2 = _31.fld0.1.2 * _31.fld0.1.2;
_16 = _52.1;
_39 = _15;
_82.0.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant(_48, 0), 0),fld1: (*_63) };
_82.0.1.2 = _31.fld0.1.2 << Field::<u32>(Variant(_82.0.2, 0), 1);
(*_63) = !Field::<u32>(Variant(_82.0.2, 0), 1);
_31.fld0.1.2 = _2.2 * _2.2;
_82.0.1.1 = Move(_17.1);
_47 = _54;
_31.fld0.1 = (_2.0, Move(_82.0.1.1), _2.2, _2.3);
_17.3 = [_82.0.1.2,_43,_82.0.1.2,_43];
(*_63) = Field::<u32>(Variant(_82.0.2, 0), 1) >> _6.1;
(*_63) = Field::<u32>(Variant(_82.0.2, 0), 1);
_41 = core::ptr::addr_of!((*_1));
(*_1) = Field::<f64>(Variant(_48, 0), 0);
(*_63) = Field::<u32>(Variant(_82.0.2, 0), 1) | Field::<u32>(Variant(_48, 0), 1);
(*_63) = Field::<u32>(Variant(_82.0.2, 0), 1) >> _31.fld0.1.2;
(*_1) = _68;
_84 = _10 as isize;
place!(Field::<f64>(Variant(_82.0.2, 0), 0)) = (*_1);
Call((*_1) = core::intrinsics::transmute(_28), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
_2.1 = core::ptr::addr_of_mut!(_54);
Call((*_1) = core::intrinsics::fmaf64(Field::<f64>(Variant(_82.0.2, 0), 0), _33, Field::<f64>(Variant(_48, 0), 0)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_79 = _2.2;
_65 = 241_u8 * 245_u8;
_69 = _65 + _65;
(*_1) = Field::<f64>(Variant(_82.0.2, 0), 0) * _33;
(*_1) = Field::<f64>(Variant(_82.0.2, 0), 0) * Field::<f64>(Variant(_82.0.2, 0), 0);
_1 = core::ptr::addr_of_mut!(_13);
_38 = _35 * _35;
_82.0.1.1 = Move(_31.fld0.1.1);
(*_1) = Field::<f64>(Variant(_48, 0), 0) * _33;
_83 = [_65,_69];
(*_63) = !Field::<u32>(Variant(_82.0.2, 0), 1);
_44 = Field::<f64>(Variant(_48, 0), 0) >= (*_1);
_6.2 = _16 - _52.1;
_80 = _11;
_82.0.1.3 = [_2.2,_2.2,_79,_43];
(*_1) = Field::<f64>(Variant(_82.0.2, 0), 0);
_23 = Field::<u32>(Variant(_82.0.2, 0), 1);
_2.3 = [_82.0.1.2,_2.2,_82.0.1.2,_31.fld0.1.2];
_64 = core::ptr::addr_of_mut!((*_1));
_83 = [_65,_69];
_94 = _21 << (*_63);
_60 = [_11,_80,_11,_11,_11,_11,_11];
(*_1) = Field::<f64>(Variant(_48, 0), 0);
_81 = [_94,_94,_94,_21,_94,_21,_94];
_7 = _94 as f64;
Goto(bb50)
}
bb50 = {
Call(_96 = dump_var(Move(_12), Move(_84), Move(_45), Move(_72)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_96 = dump_var(Move(_30), Move(_53), Move(_75), Move(_65)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_96 = dump_var(Move(_76), Move(_44), Move(_60), Move(_42)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_96 = dump_var(Move(_11), Move(_61), Move(_10), Move(_69)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_96 = dump_var(Move(_24), Move(_80), Move(_28), Move(_47)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [i16; 4],mut _2: i16,mut _3: ([u16; 5], *mut char, i16, [i16; 4]),mut _4: [u128; 5]) -> f64 {
mir! {
type RET = f64;
let _5: f64;
let _6: u128;
let _7: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _8: [i16; 4];
let _9: Adt25;
let _10: isize;
let _11: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _12: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _13: f32;
let _14: f64;
let _15: [i16; 4];
let _16: bool;
let _17: &'static mut u16;
let _18: u64;
let _19: *const [char; 4];
let _20: &'static u8;
let _21: i16;
let _22: f32;
let _23: &'static mut usize;
let _24: f32;
let _25: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _26: i64;
let _27: Adt78;
let _28: ([i128; 4], &'static [char; 4]);
let _29: &'static [char; 4];
let _30: char;
let _31: i8;
let _32: isize;
let _33: isize;
let _34: char;
let _35: *mut *mut char;
let _36: f32;
let _37: (usize, u128, f32);
let _38: &'static [i128; 4];
let _39: *mut char;
let _40: &'static mut u16;
let _41: f64;
let _42: ();
let _43: ();
{
RET = (-117_i8) as f64;
_3.2 = -_2;
_1 = _3.3;
_3.3 = [_3.2,_3.2,_2,_2];
_4 = [313225163246111550921242869635976204150_u128,266354136889626127080988230770926146312_u128,277061760292104987853451876825797297463_u128,238921020340552854746790372403356927222_u128,27869457404457523008079142160670976803_u128];
_1 = [_2,_2,_3.2,_2];
RET = (-8182515767866315573_i64) as f64;
_2 = _3.2;
_3.0 = [20620_u16,16650_u16,27633_u16,33605_u16,29817_u16];
_3.0 = [12035_u16,13727_u16,16251_u16,14917_u16,21369_u16];
_3.3 = [_3.2,_2,_3.2,_2];
_2 = _3.2 >> _3.2;
_3.0 = [20601_u16,49459_u16,64904_u16,12949_u16,16965_u16];
_3.2 = _2 * _2;
_3.0 = [57403_u16,20420_u16,6583_u16,1859_u16,61887_u16];
_3.2 = _2;
_8 = [_3.2,_3.2,_2,_3.2];
Goto(bb1)
}
bb1 = {
_3.3 = [_3.2,_3.2,_2,_3.2];
_5 = -RET;
_9 = Adt25::Variant0 { fld0: RET,fld1: 2626541205_u32 };
_1 = [_3.2,_2,_2,_2];
_3.0 = [48860_u16,42894_u16,56479_u16,37528_u16,24637_u16];
_3.0 = [47803_u16,9182_u16,60021_u16,51220_u16,65521_u16];
_3.3 = _8;
_4 = [221798161108881641650711724642736728325_u128,10824889763761317262371632456044370290_u128,247621504503247805829331808523676443490_u128,234813334091858794286352286789605305161_u128,171719742961837188588349069284072667096_u128];
_5 = RET + Field::<f64>(Variant(_9, 0), 0);
_6 = !286617932414087710986080894805819240091_u128;
_5 = Field::<f64>(Variant(_9, 0), 0) - RET;
place!(Field::<u32>(Variant(_9, 0), 1)) = 1434484249_u32;
RET = _5;
_11.1 = (_8, Move(_3), Move(_9));
_3.1 = Move(_11.1.1.1);
_11.1.2 = Adt25::Variant0 { fld0: RET,fld1: 1940803377_u32 };
_6 = 15069192676800934696201518084619841165_u128 >> _11.1.1.2;
_3.2 = _11.1.1.2 + _2;
_11.1.1.2 = !_3.2;
_11.0 = [9537056315641415847_usize,3410711734310705366_usize,15088719319511434401_usize];
_8 = _11.1.1.3;
_11.1.1.0 = [29487_u16,9263_u16,14871_u16,59349_u16,64993_u16];
_11.1.1.1 = Move(_3.1);
_3 = Move(_11.1.1);
_4 = [_6,_6,_6,_6,_6];
Goto(bb2)
}
bb2 = {
_12 = &_3;
_11.1.1.3 = _1;
_1 = [(*_12).2,(*_12).2,(*_12).2,(*_12).2];
_1 = [(*_12).2,(*_12).2,(*_12).2,(*_12).2];
_11.1.1.3 = [(*_12).2,(*_12).2,(*_12).2,(*_12).2];
_11.1.1 = ((*_12).0, Move(_3.1), (*_12).2, (*_12).3);
Goto(bb3)
}
bb3 = {
place!(Field::<u32>(Variant(_11.1.2, 0), 1)) = 1928058976_u32 ^ 868622268_u32;
_6 = 228618878689513161628269830108505027851_u128;
_11.0 = [484409902914985393_usize,17203724312639447786_usize,5_usize];
_5 = Field::<f64>(Variant(_11.1.2, 0), 0) * Field::<f64>(Variant(_11.1.2, 0), 0);
Call(_11.1.2 = fn15((*_12).0, Move(_11.1.1.1), _1, (*_12).0, (*_12).0, (*_12).2, (*_12).0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.1.1.3 = [_3.2,(*_12).2,(*_12).2,(*_12).2];
_11.1.1.2 = (*_12).2 | (*_12).2;
_6 = !233580501604495779237940042610883370214_u128;
_5 = Field::<f64>(Variant(_11.1.2, 0), 0) + Field::<f64>(Variant(_11.1.2, 0), 0);
_11.1.1.0 = (*_12).0;
_22 = (-202297982_i32) as f32;
_11.1.1.2 = '\u{156d0}' as i16;
_13 = _22 + _22;
_9 = Move(_11.1.2);
_22 = _13;
_10 = !45_isize;
_21 = (*_12).2 * (*_12).2;
_22 = _13 * _13;
Goto(bb5)
}
bb5 = {
_1 = [(*_12).2,(*_12).2,_21,_2];
_11.1.1.0 = (*_12).0;
_24 = _22;
Call(_21 = core::intrinsics::bswap((*_12).2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14 = 3_usize as f64;
_11.0 = [4_usize,4_usize,9758201383109643206_usize];
_4 = [_6,_6,_6,_6,_6];
_11.1.1.0 = [40332_u16,35663_u16,25621_u16,23547_u16,49535_u16];
_1 = (*_12).3;
_11.1.1.0 = [62192_u16,24329_u16,27093_u16,40795_u16,52595_u16];
_11.1.1.3 = (*_12).3;
_24 = -_22;
_4 = [_6,_6,_6,_6,_6];
_11.1.1.3 = (*_12).3;
_2 = -(*_12).2;
_15 = [(*_12).2,(*_12).2,(*_12).2,_21];
_25.2.2 = Adt25::Variant0 { fld0: _5,fld1: Field::<u32>(Variant(_9, 0), 1) };
Goto(bb7)
}
bb7 = {
_15 = [(*_12).2,(*_12).2,(*_12).2,(*_12).2];
place!(Field::<f64>(Variant(_25.2.2, 0), 0)) = -_5;
_25.2.0 = _1;
_25.2.1.0 = [34716_u16,50588_u16,1306_u16,11708_u16,63419_u16];
_18 = _5 as u64;
Goto(bb8)
}
bb8 = {
_25.0 = -_5;
_25.2.1.2 = _21;
_21 = (*_12).2 ^ (*_12).2;
_26 = !5782232944835755884_i64;
_16 = !true;
_26 = 8537638009546795680_i64;
_22 = _24 * _13;
_11.1.1.0 = [10316_u16,16458_u16,65385_u16,35471_u16,19293_u16];
place!(Field::<u32>(Variant(_9, 0), 1)) = Field::<u32>(Variant(_25.2.2, 0), 1);
_28.0 = [(-41535294834510742058993437186509158651_i128),(-142127607022441075906025572914668994743_i128),(-109216220775019209502313139989757218008_i128),141035321720891577761901340237657486286_i128];
Goto(bb9)
}
bb9 = {
_14 = Field::<f64>(Variant(_25.2.2, 0), 0) - Field::<f64>(Variant(_25.2.2, 0), 0);
_21 = _2 + (*_12).2;
_24 = _22 * _22;
_25.2.1.3 = _8;
_15 = [_21,(*_12).2,(*_12).2,_2];
_3.2 = _25.2.1.2 & _21;
_11.1.2 = Adt25::Variant0 { fld0: _14,fld1: Field::<u32>(Variant(_9, 0), 1) };
_11.1.1.0 = [9833_u16,25086_u16,34602_u16,64711_u16,54632_u16];
_11.0 = [1_usize,7_usize,3_usize];
_25.3 = core::ptr::addr_of!(_18);
place!(Field::<f64>(Variant(_25.2.2, 0), 0)) = -_5;
place!(Field::<u32>(Variant(_25.2.2, 0), 1)) = !Field::<u32>(Variant(_11.1.2, 0), 1);
match _26 {
0 => bb1,
1 => bb8,
2 => bb6,
8537638009546795680 => bb10,
_ => bb4
}
}
bb10 = {
RET = 114_i8 as f64;
_26 = 7_usize as i64;
_2 = -_21;
_34 = '\u{a0678}';
_31 = 126_i8;
_25.2.1.1 = core::ptr::addr_of_mut!(_34);
_3 = Move(_25.2.1);
_35 = core::ptr::addr_of_mut!(_3.1);
_30 = _34;
place!(Field::<f64>(Variant(_11.1.2, 0), 0)) = _2 as f64;
_1 = _11.1.1.3;
_14 = Field::<f64>(Variant(_25.2.2, 0), 0) - _5;
place!(Field::<f64>(Variant(_9, 0), 0)) = Field::<f64>(Variant(_25.2.2, 0), 0) * _14;
_25.2.1.0 = [50351_u16,37409_u16,13417_u16,3067_u16,34449_u16];
_11.1.0 = [_11.1.1.2,_3.2,_21,_2];
_25.2.1 = (_3.0, Move((*_35)), _21, _11.1.1.3);
_37.1 = _6 & _6;
_36 = _24 + _22;
_18 = 2090868704293539206_u64 + 103290607015661910_u64;
(*_35) = Move(_25.2.1.1);
_22 = -_24;
_25.2.1.1 = core::ptr::addr_of_mut!(_30);
match _31 {
0 => bb6,
1 => bb3,
126 => bb12,
_ => bb11
}
}
bb11 = {
_3.3 = [_3.2,_3.2,_2,_3.2];
_5 = -RET;
_9 = Adt25::Variant0 { fld0: RET,fld1: 2626541205_u32 };
_1 = [_3.2,_2,_2,_2];
_3.0 = [48860_u16,42894_u16,56479_u16,37528_u16,24637_u16];
_3.0 = [47803_u16,9182_u16,60021_u16,51220_u16,65521_u16];
_3.3 = _8;
_4 = [221798161108881641650711724642736728325_u128,10824889763761317262371632456044370290_u128,247621504503247805829331808523676443490_u128,234813334091858794286352286789605305161_u128,171719742961837188588349069284072667096_u128];
_5 = RET + Field::<f64>(Variant(_9, 0), 0);
_6 = !286617932414087710986080894805819240091_u128;
_5 = Field::<f64>(Variant(_9, 0), 0) - RET;
place!(Field::<u32>(Variant(_9, 0), 1)) = 1434484249_u32;
RET = _5;
_11.1 = (_8, Move(_3), Move(_9));
_3.1 = Move(_11.1.1.1);
_11.1.2 = Adt25::Variant0 { fld0: RET,fld1: 1940803377_u32 };
_6 = 15069192676800934696201518084619841165_u128 >> _11.1.1.2;
_3.2 = _11.1.1.2 + _2;
_11.1.1.2 = !_3.2;
_11.0 = [9537056315641415847_usize,3410711734310705366_usize,15088719319511434401_usize];
_8 = _11.1.1.3;
_11.1.1.0 = [29487_u16,9263_u16,14871_u16,59349_u16,64993_u16];
_11.1.1.1 = Move(_3.1);
_3 = Move(_11.1.1);
_4 = [_6,_6,_6,_6,_6];
Goto(bb2)
}
bb12 = {
_11.1.1.1 = Move((*_35));
_32 = !_10;
_11.1.1 = (_3.0, Move(_25.2.1.1), _25.2.1.2, _3.3);
(*_35) = Move(_11.1.1.1);
_7 = &_3;
_7 = Move(_12);
_37.0 = 15729857745019276594_usize & 15406122765311980137_usize;
_37 = (11650703528791490056_usize, _6, _24);
_34 = _30;
(*_35) = core::ptr::addr_of_mut!(_30);
_25.2.1.2 = _11.1.1.2 << Field::<u32>(Variant(_11.1.2, 0), 1);
RET = Field::<f64>(Variant(_25.2.2, 0), 0);
_3.3 = _25.2.0;
place!(Field::<u32>(Variant(_25.2.2, 0), 1)) = Field::<u32>(Variant(_9, 0), 1) | Field::<u32>(Variant(_9, 0), 1);
Goto(bb13)
}
bb13 = {
Call(_42 = dump_var(Move(_31), Move(_18), Move(_34), Move(_2)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_42 = dump_var(Move(_26), Move(_30), Move(_21), _43), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u16; 5],mut _2: *mut char,mut _3: [i16; 4],mut _4: [u16; 5],mut _5: [u16; 5],mut _6: i16,mut _7: [u16; 5]) -> Adt25 {
mir! {
type RET = Adt25;
let _8: u128;
let _9: isize;
let _10: [i32; 1];
let _11: &'static mut usize;
let _12: char;
let _13: (isize, u8, i128, i64);
let _14: Adt78;
let _15: bool;
let _16: f32;
let _17: f64;
let _18: *mut [i16; 4];
let _19: *const f64;
let _20: i8;
let _21: *mut u32;
let _22: Adt42;
let _23: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _24: Adt68;
let _25: &'static [i128; 4];
let _26: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _27: *mut *mut char;
let _28: isize;
let _29: isize;
let _30: &'static Adt25;
let _31: [i16; 4];
let _32: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _33: i8;
let _34: [usize; 3];
let _35: [u128; 5];
let _36: u64;
let _37: bool;
let _38: i128;
let _39: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _40: i128;
let _41: bool;
let _42: [i128; 4];
let _43: i64;
let _44: i16;
let _45: (isize, u8, i128, i64);
let _46: (isize, u8, i128, i64);
let _47: u8;
let _48: *mut &'static Adt25;
let _49: f32;
let _50: usize;
let _51: *mut [i16; 4];
let _52: [u64; 7];
let _53: *mut &'static Adt25;
let _54: &'static Adt25;
let _55: *mut *mut char;
let _56: [i64; 7];
let _57: [i128; 6];
let _58: char;
let _59: u128;
let _60: &'static i128;
let _61: (&'static Adt25, char, *mut char);
let _62: u32;
let _63: isize;
let _64: i128;
let _65: f32;
let _66: char;
let _67: isize;
let _68: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _69: i128;
let _70: ();
let _71: ();
{
_5 = _7;
_1 = [35461_u16,56435_u16,26281_u16,55292_u16,43444_u16];
_7 = _1;
_8 = 13674441875517542857995279904592243237_u128 & 217492548183578162283679282199220185272_u128;
Goto(bb1)
}
bb1 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb2 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb3 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
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
_7 = [6487_u16,9380_u16,17961_u16,16519_u16,34393_u16];
_13.2 = 102028598688397943480276834708659565287_i128 >> _6;
_13 = (_9, 188_u8, 11938083623219275945308116215066008237_i128, (-5457785980205929217_i64));
_13.3 = 5671754391830271120_i64 ^ 7067881303336858555_i64;
_8 = !213476572721647163104520530491108517893_u128;
_4 = [49525_u16,25217_u16,18342_u16,8860_u16,35045_u16];
_5 = [22655_u16,53536_u16,38633_u16,27897_u16,57637_u16];
_13.3 = true as i64;
_9 = _13.0;
_13.1 = !31_u8;
_12 = '\u{7aca6}';
match _13.2 {
0 => bb6,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
11938083623219275945308116215066008237 => bb14,
_ => bb13
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
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb12 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb13 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb14 = {
_7 = _5;
_10 = [(-2098602250_i32)];
_15 = _13.2 == _13.2;
_3 = [_6,_6,_6,_6];
_15 = true | false;
_13.0 = _9 >> _13.2;
_9 = _13.0;
_15 = _13.2 >= _13.2;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = '\u{ca603}';
(*_2) = '\u{c02fa}';
(*_2) = '\u{e68e6}';
_13.3 = 4235003509087645823_i64 >> _9;
(*_2) = '\u{6d5a9}';
Goto(bb15)
}
bb15 = {
(*_2) = '\u{2a95d}';
(*_2) = '\u{e8841}';
_6 = 4180_i16;
_4 = [35440_u16,63770_u16,26168_u16,28308_u16,11892_u16];
_12 = '\u{fcf5c}';
(*_2) = '\u{b0dd3}';
(*_2) = '\u{5eaba}';
(*_2) = '\u{f95c3}';
_7 = _5;
_16 = 1_usize as f32;
(*_2) = '\u{61215}';
_17 = 7493626123593347938_usize as f64;
_12 = '\u{f978b}';
_2 = core::ptr::addr_of_mut!((*_2));
_16 = _13.3 as f32;
(*_2) = '\u{5a5ad}';
(*_2) = '\u{93b90}';
_9 = _13.0 & _13.0;
_13.1 = 124_u8 * 198_u8;
(*_2) = '\u{10d1fd}';
_17 = 8179599393212277280_u64 as f64;
_9 = _13.0 + _13.0;
_1 = _7;
(*_2) = '\u{a6169}';
_13.0 = _13.2 as isize;
(*_2) = '\u{ce547}';
match _13.2 {
0 => bb14,
1 => bb9,
2 => bb10,
3 => bb16,
11938083623219275945308116215066008237 => bb18,
_ => bb17
}
}
bb16 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb17 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb18 = {
_2 = core::ptr::addr_of_mut!((*_2));
_3 = [_6,_6,_6,_6];
_19 = core::ptr::addr_of!(_17);
_8 = 7678234948025902815628739992343079043_u128;
_23.2.1.2 = _6 >> _13.2;
(*_19) = _23.2.1.2 as f64;
(*_2) = '\u{9e252}';
(*_2) = '\u{c66b8}';
(*_19) = 96_i8 as f64;
(*_19) = _13.3 as f64;
(*_2) = '\u{ec579}';
_15 = _9 <= _9;
(*_19) = _8 as f64;
_13.2 = (*_2) as i128;
_3 = [_23.2.1.2,_23.2.1.2,_23.2.1.2,_23.2.1.2];
(*_2) = '\u{42490}';
(*_2) = '\u{b60be}';
_24.fld0.1.1 = core::ptr::addr_of_mut!((*_2));
(*_19) = 611580958_i32 as f64;
_20 = (-100_i8) >> _9;
_5 = [22719_u16,21974_u16,6529_u16,13386_u16,60487_u16];
_19 = core::ptr::addr_of!((*_19));
_9 = _13.0 - _13.0;
_4 = _5;
_15 = !true;
_8 = (*_19) as u128;
(*_2) = '\u{6f85d}';
match _6 {
0 => bb19,
1 => bb20,
2 => bb21,
3 => bb22,
4180 => bb24,
_ => bb23
}
}
bb19 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
_7 = [6487_u16,9380_u16,17961_u16,16519_u16,34393_u16];
_13.2 = 102028598688397943480276834708659565287_i128 >> _6;
_13 = (_9, 188_u8, 11938083623219275945308116215066008237_i128, (-5457785980205929217_i64));
_13.3 = 5671754391830271120_i64 ^ 7067881303336858555_i64;
_8 = !213476572721647163104520530491108517893_u128;
_4 = [49525_u16,25217_u16,18342_u16,8860_u16,35045_u16];
_5 = [22655_u16,53536_u16,38633_u16,27897_u16,57637_u16];
_13.3 = true as i64;
_9 = _13.0;
_13.1 = !31_u8;
_12 = '\u{7aca6}';
match _13.2 {
0 => bb6,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
11938083623219275945308116215066008237 => bb14,
_ => bb13
}
}
bb24 = {
(*_19) = 6913214320015245274_usize as f64;
_23.0 = (*_19) * (*_19);
(*_19) = -_23.0;
_8 = !266258814301054266763359351178622893_u128;
(*_19) = _23.0 - _23.0;
(*_19) = _23.0;
(*_2) = '\u{8979c}';
_9 = -_13.0;
Call(_22 = fn16((*_19), (*_2), Move(_19), (*_2), Move(_24.fld0.1.1), _20), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_24.fld0.1.0 = [63101_u16,17755_u16,6914_u16,61636_u16,8590_u16];
_24.fld0.1 = (_7, Move(_2), _23.2.1.2, _3);
Goto(bb26)
}
bb26 = {
_13.2 = Field::<i128>(Variant(_22, 1), 2) + Field::<i128>(Variant(_22, 1), 2);
_25 = &place!(Field::<[i128; 4]>(Variant(_22, 1), 3));
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = [_6,_23.2.1.2,_24.fld0.1.2,_24.fld0.1.2];
_1 = [9373_u16,11832_u16,30470_u16,30476_u16,35013_u16];
_23.2.0 = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_23.2.1.2];
_18 = core::ptr::addr_of_mut!(_3);
_24.fld0.0 = (*_18);
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = (*_18);
(*_18) = [_24.fld0.1.2,_23.2.1.2,_23.2.1.2,_24.fld0.1.2];
_23.2.1.2 = _24.fld0.1.2;
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb27)
}
bb27 = {
_13.3 = _16 as i64;
_24.fld0.2 = Adt25::Variant0 { fld0: _17,fld1: 1030216915_u32 };
match Field::<i128>(Variant(_22, 1), 2) {
0 => bb1,
1 => bb17,
2 => bb3,
3 => bb10,
4 => bb20,
154667553499189329909394543776361417237 => bb29,
_ => bb28
}
}
bb28 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb29 = {
_24.fld0.1 = (_1, Move(_2), _23.2.1.2, (*_18));
(*_18) = [_24.fld0.1.2,_6,_23.2.1.2,_23.2.1.2];
(*_18) = _24.fld0.1.3;
_2 = core::ptr::addr_of_mut!(_12);
_16 = _13.2 as f32;
_13.0 = _9;
_6 = -_23.2.1.2;
_24.fld1 = [_13.1,_13.1,_13.1,_13.1];
_17 = Field::<f64>(Variant(_24.fld0.2, 0), 0) * _23.0;
(*_2) = '\u{5c6c9}';
match Field::<i128>(Variant(_22, 1), 2) {
0 => bb30,
1 => bb31,
2 => bb32,
3 => bb33,
154667553499189329909394543776361417237 => bb35,
_ => bb34
}
}
bb30 = {
Return()
}
bb31 = {
Return()
}
bb32 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb33 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb34 = {
Return()
}
bb35 = {
_9 = 13038340590079883110_usize as isize;
_8 = !148646511057258179488669267471201258599_u128;
_24.fld0.1.0 = _7;
_13.3 = (-2047344196623416248_i64) + (-3311647350964429590_i64);
_6 = _9 as i16;
(*_2) = '\u{f98c3}';
_24.fld0.2 = Adt25::Variant0 { fld0: _23.0,fld1: 2986994724_u32 };
_28 = -_13.0;
RET = Adt25::Variant0 { fld0: _23.0,fld1: 4156939241_u32 };
_24.fld0.1.2 = _6;
(*_18) = [_23.2.1.2,_24.fld0.1.2,_23.2.1.2,_23.2.1.2];
Goto(bb36)
}
bb36 = {
_24.fld2 = core::ptr::addr_of!(_17);
(*_2) = '\u{a1895}';
(*_2) = '\u{108b18}';
place!(Field::<i128>(Variant(_22, 1), 2)) = _13.2 & _13.2;
(*_18) = [_23.2.1.2,_23.2.1.2,_6,_6];
(*_18) = [_23.2.1.2,_6,_23.2.1.2,_23.2.1.2];
_31 = [_23.2.1.2,_23.2.1.2,_24.fld0.1.2,_23.2.1.2];
_5 = [27091_u16,62023_u16,20926_u16,24873_u16,5163_u16];
_23.2.1.0 = [45634_u16,31214_u16,8240_u16,56979_u16,46167_u16];
(*_18) = [_23.2.1.2,_23.2.1.2,_23.2.1.2,_24.fld0.1.2];
(*_18) = [_23.2.1.2,_23.2.1.2,_23.2.1.2,_24.fld0.1.2];
_13 = (_9, 111_u8, Field::<i128>(Variant(_22, 1), 2), 7485692997154862967_i64);
(*_18) = Field::<[i16; 4]>(Variant(_22, 1), 0);
(*_18) = [_23.2.1.2,_6,_23.2.1.2,_24.fld0.1.2];
_2 = core::ptr::addr_of_mut!((*_2));
_16 = 21511_u16 as f32;
_23.2.1 = Move(_24.fld0.1);
(*_2) = '\u{6e410}';
_1 = _7;
_24.fld0.1.1 = Move(_23.2.1.1);
_24.fld0.2 = Adt25::Variant0 { fld0: Field::<f64>(Variant(RET, 0), 0),fld1: 3259234677_u32 };
_34 = [0_usize,10127299774865214874_usize,2_usize];
_35 = [_8,_8,_8,_8,_8];
match _13.3 {
0 => bb37,
1 => bb38,
7485692997154862967 => bb40,
_ => bb39
}
}
bb37 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb38 = {
_13.2 = Field::<i128>(Variant(_22, 1), 2) + Field::<i128>(Variant(_22, 1), 2);
_25 = &place!(Field::<[i128; 4]>(Variant(_22, 1), 3));
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = [_6,_23.2.1.2,_24.fld0.1.2,_24.fld0.1.2];
_1 = [9373_u16,11832_u16,30470_u16,30476_u16,35013_u16];
_23.2.0 = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_23.2.1.2];
_18 = core::ptr::addr_of_mut!(_3);
_24.fld0.0 = (*_18);
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = (*_18);
(*_18) = [_24.fld0.1.2,_23.2.1.2,_23.2.1.2,_24.fld0.1.2];
_23.2.1.2 = _24.fld0.1.2;
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb27)
}
bb39 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb40 = {
_23.2.1 = (_1, Move(_2), _6, (*_18));
(*_18) = [_23.2.1.2,_6,_6,_6];
_13.0 = _28 - _28;
(*_18) = [_23.2.1.2,_6,_23.2.1.2,_6];
_23.2.1 = (_4, Move(_24.fld0.1.1), _6, (*_18));
_24.fld0.1.1 = Move(_23.2.1.1);
_23.2.1 = (_5, Move(_24.fld0.1.1), _6, (*_18));
_36 = 5342323504042380083_u64 << _13.2;
_24.fld0.1.1 = core::ptr::addr_of_mut!(_12);
(*_18) = [_23.2.1.2,_6,_6,_6];
_15 = true;
_21 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(RET, 0), 1)));
(*_18) = _24.fld0.0;
(*_21) = !3158349622_u32;
(*_18) = [_6,_6,_23.2.1.2,_23.2.1.2];
_13.1 = 121_u8 << _13.0;
(*_21) = 2986345735_u32;
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = _16 as u32;
_24.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_24.fld0.2, 0), 0)));
_13 = (_28, 250_u8, Field::<i128>(Variant(_22, 1), 2), (-5434064433441008342_i64));
_18 = core::ptr::addr_of_mut!((*_18));
place!(Field::<[i32; 1]>(Variant(_22, 1), 1)) = [(-1882142272_i32)];
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = (*_21) + (*_21);
(*_21) = Field::<u32>(Variant(_24.fld0.2, 0), 1);
(*_18) = [_23.2.1.2,_6,_6,_23.2.1.2];
_30 = &_24.fld0.2;
(*_18) = [_23.2.1.2,_23.2.1.2,_6,_23.2.1.2];
match _13.3 {
0 => bb10,
1 => bb12,
2 => bb41,
3 => bb42,
340282366920938463457940542998327203114 => bb44,
_ => bb43
}
}
bb41 = {
Return()
}
bb42 = {
_13.2 = Field::<i128>(Variant(_22, 1), 2) + Field::<i128>(Variant(_22, 1), 2);
_25 = &place!(Field::<[i128; 4]>(Variant(_22, 1), 3));
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = [_6,_23.2.1.2,_24.fld0.1.2,_24.fld0.1.2];
_1 = [9373_u16,11832_u16,30470_u16,30476_u16,35013_u16];
_23.2.0 = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_23.2.1.2];
_18 = core::ptr::addr_of_mut!(_3);
_24.fld0.0 = (*_18);
place!(Field::<[i16; 4]>(Variant(_22, 1), 0)) = (*_18);
(*_18) = [_24.fld0.1.2,_23.2.1.2,_23.2.1.2,_24.fld0.1.2];
_23.2.1.2 = _24.fld0.1.2;
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb27)
}
bb43 = {
Return()
}
bb44 = {
(*_21) = Field::<u32>(Variant((*_30), 0), 1) ^ Field::<u32>(Variant((*_30), 0), 1);
place!(Field::<i128>(Variant(_22, 1), 2)) = _13.2 & _13.2;
_5 = [55403_u16,32888_u16,59344_u16,4829_u16,50600_u16];
_24.fld0.1.0 = [473_u16,16924_u16,63124_u16,18037_u16,32699_u16];
_17 = Field::<f64>(Variant((*_30), 0), 0);
_39.1.1 = Move(_23.2.1);
match _13.3 {
0 => bb11,
1 => bb28,
2 => bb30,
3 => bb45,
4 => bb46,
340282366920938463457940542998327203114 => bb48,
_ => bb47
}
}
bb45 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb46 = {
_8 = 255688212943834707776832146173058898283_u128;
_1 = _5;
_1 = [9872_u16,51559_u16,48343_u16,54462_u16,30480_u16];
_7 = [24530_u16,44237_u16,39587_u16,812_u16,52636_u16];
_10 = [1539076855_i32];
_6 = !(-9892_i16);
_1 = [63856_u16,6227_u16,25546_u16,17758_u16,19868_u16];
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = [(-1561642219_i32)];
_9 = (-9223372036854775808_isize);
_4 = _1;
_3 = [_6,_6,_6,_6];
_4 = [28572_u16,15047_u16,367_u16,30441_u16,26134_u16];
_5 = _7;
_1 = [50569_u16,45812_u16,10383_u16,60010_u16,17083_u16];
_1 = _7;
Goto(bb2)
}
bb47 = {
_24.fld0.1 = (_1, Move(_2), _23.2.1.2, (*_18));
(*_18) = [_24.fld0.1.2,_6,_23.2.1.2,_23.2.1.2];
(*_18) = _24.fld0.1.3;
_2 = core::ptr::addr_of_mut!(_12);
_16 = _13.2 as f32;
_13.0 = _9;
_6 = -_23.2.1.2;
_24.fld1 = [_13.1,_13.1,_13.1,_13.1];
_17 = Field::<f64>(Variant(_24.fld0.2, 0), 0) * _23.0;
(*_2) = '\u{5c6c9}';
match Field::<i128>(Variant(_22, 1), 2) {
0 => bb30,
1 => bb31,
2 => bb32,
3 => bb33,
154667553499189329909394543776361417237 => bb35,
_ => bb34
}
}
bb48 = {
(*_21) = Field::<u32>(Variant((*_30), 0), 1) << _28;
_33 = _20 << _13.3;
_23.2.1.1 = core::ptr::addr_of_mut!(_12);
_39.1.1.0 = _1;
(*_21) = Field::<u32>(Variant((*_30), 0), 1) + Field::<u32>(Variant((*_30), 0), 1);
_13.3 = (-6082149593518746089_i64);
_39.1.2 = Move(RET);
_38 = Field::<i128>(Variant(_22, 1), 2);
_39.1.1 = (_24.fld0.1.0, Move(_23.2.1.1), _6, _31);
(*_21) = Field::<u32>(Variant((*_30), 0), 1) * Field::<u32>(Variant((*_30), 0), 1);
match _13.1 {
0 => bb34,
1 => bb49,
2 => bb50,
3 => bb51,
4 => bb52,
5 => bb53,
6 => bb54,
250 => bb56,
_ => bb55
}
}
bb49 = {
_7 = _5;
_10 = [(-2098602250_i32)];
_15 = _13.2 == _13.2;
_3 = [_6,_6,_6,_6];
_15 = true | false;
_13.0 = _9 >> _13.2;
_9 = _13.0;
_15 = _13.2 >= _13.2;
_2 = core::ptr::addr_of_mut!(_12);
(*_2) = '\u{ca603}';
(*_2) = '\u{c02fa}';
(*_2) = '\u{e68e6}';
_13.3 = 4235003509087645823_i64 >> _9;
(*_2) = '\u{6d5a9}';
Goto(bb15)
}
bb50 = {
Return()
}
bb51 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb52 = {
Return()
}
bb53 = {
Return()
}
bb54 = {
Return()
}
bb55 = {
Return()
}
bb56 = {
_30 = &_39.1.2;
_23.0 = Field::<f64>(Variant((*_30), 0), 0) * Field::<f64>(Variant(_24.fld0.2, 0), 0);
(*_18) = [_6,_39.1.1.2,_39.1.1.2,_39.1.1.2];
_23.2 = (_39.1.1.3, Move(_39.1.1), Move(_24.fld0.2));
_39.0 = [207258694416048888_usize,5_usize,7_usize];
_16 = _33 as f32;
_23.3 = core::ptr::addr_of!(_36);
_24.fld0.1.2 = -_23.2.1.2;
Goto(bb57)
}
bb57 = {
(*_18) = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_23.2.1.2];
(*_18) = Field::<[i16; 4]>(Variant(_22, 1), 0);
_19 = core::ptr::addr_of!(place!(Field::<f64>(Variant((*_30), 0), 0)));
_39.1 = Move(_23.2);
_37 = _16 == _16;
_24.fld0.1 = (_4, Move(_39.1.1.1), _6, _24.fld0.0);
_23.2.1.3 = [_39.1.1.2,_6,_6,_39.1.1.2];
_36 = !2236045721697319312_u64;
_23.2.1 = (_1, Move(_24.fld0.1.1), _39.1.1.2, (*_18));
_15 = _33 >= _33;
_34 = [4_usize,5_usize,10547508359648992010_usize];
_24.fld0.1.1 = core::ptr::addr_of_mut!(_12);
(*_18) = [_39.1.1.2,_6,_39.1.1.2,_23.2.1.2];
_23.2 = (_24.fld0.1.3, Move(_24.fld0.1), Move(_39.1.2));
_30 = &_23.2.2;
_46.1 = _13.1 * _13.1;
_24.fld0.1 = (_4, Move(_23.2.1.1), _6, _39.1.0);
_23.2.1.0 = [22976_u16,19807_u16,9438_u16,27369_u16,24040_u16];
_5 = [31697_u16,52494_u16,32845_u16,1907_u16,45949_u16];
(*_21) = _12 as u32;
(*_18) = [_6,_6,_6,_39.1.1.2];
place!(Field::<[i32; 1]>(Variant(_22, 1), 1)) = _10;
_28 = Field::<f64>(Variant(_23.2.2, 0), 0) as isize;
(*_21) = !Field::<u32>(Variant((*_30), 0), 1);
_23.2.0 = [_23.2.1.2,_6,_39.1.1.2,_6];
Goto(bb58)
}
bb58 = {
_45.2 = !Field::<i128>(Variant(_22, 1), 2);
_2 = core::ptr::addr_of_mut!(_12);
_46.3 = !_13.3;
(*_21) = _36 as u32;
_39.1.1 = (_4, Move(_2), _23.2.1.2, _24.fld0.0);
_39.1 = ((*_18), Move(_24.fld0.1), Move(_23.2.2));
(*_21) = Field::<u32>(Variant(_39.1.2, 0), 1) | Field::<u32>(Variant(_39.1.2, 0), 1);
place!(Field::<f64>(Variant(_39.1.2, 0), 0)) = _23.0 + _23.0;
(*_21) = Field::<u32>(Variant(_39.1.2, 0), 1) + Field::<u32>(Variant(_39.1.2, 0), 1);
_40 = _38 + Field::<i128>(Variant(_22, 1), 2);
_24.fld0.1.1 = core::ptr::addr_of_mut!(_12);
_23.2.1 = Move(_39.1.1);
_42 = (*_25);
(*_21) = Field::<u32>(Variant(_39.1.2, 0), 1) ^ Field::<u32>(Variant(_39.1.2, 0), 1);
_39.1.1 = (_4, Move(_23.2.1.1), _23.2.1.2, _31);
_23.2.1 = (_1, Move(_39.1.1.1), _6, _31);
_10 = [190425930_i32];
(*_18) = [_39.1.1.2,_23.2.1.2,_39.1.1.2,_23.2.1.2];
_46 = _13;
_40 = _38;
_33 = _20 + _20;
_46.0 = _9;
_39.1.1.1 = core::ptr::addr_of_mut!(_12);
_51 = core::ptr::addr_of_mut!(_3);
_50 = !8225424674477988808_usize;
(*_51) = Field::<[i16; 4]>(Variant(_22, 1), 0);
_45.3 = _16 as i64;
Goto(bb59)
}
bb59 = {
_23.2.1.2 = _6 - _6;
_23.2 = ((*_18), Move(_39.1.1), Move(_39.1.2));
(*_18) = [_6,_6,_6,_23.2.1.2];
_1 = [36041_u16,57111_u16,47244_u16,27227_u16,11573_u16];
_29 = _13.0 + _13.0;
_45.1 = !_46.1;
(*_18) = Field::<[i16; 4]>(Variant(_22, 1), 0);
(*_21) = Field::<u32>(Variant(_23.2.2, 0), 1) * Field::<u32>(Variant(_23.2.2, 0), 1);
(*_18) = [_6,_6,_6,_23.2.1.2];
_43 = -_45.3;
(*_18) = Field::<[i16; 4]>(Variant(_22, 1), 0);
_24.fld0 = ((*_18), Move(_23.2.1), Move(_23.2.2));
RET = Move(_24.fld0.2);
_36 = !7798753293464277941_u64;
_49 = -_16;
_39.1 = ((*_18), Move(_24.fld0.1), Move(RET));
_23.2.1 = (_1, Move(_39.1.1.1), _6, (*_18));
(*_18) = [_39.1.1.2,_23.2.1.2,_39.1.1.2,_23.2.1.2];
_10 = [(-1854568375_i32)];
_24.fld1 = [_13.1,_13.1,_45.1,_13.1];
_39.0 = _34;
(*_18) = [_39.1.1.2,_23.2.1.2,_6,_23.2.1.2];
(*_18) = [_6,_6,_39.1.1.2,_6];
_23.2.1.3 = _39.1.0;
_4 = _1;
_5 = _7;
_50 = _33 as usize;
_36 = 29999_u16 as u64;
Goto(bb60)
}
bb60 = {
_49 = _16 - _16;
_24.fld0.1 = (_23.2.1.0, Move(_23.2.1.1), _23.2.1.2, _24.fld0.0);
_23.2.1 = Move(_24.fld0.1);
(*_18) = Field::<[i16; 4]>(Variant(_22, 1), 0);
_24.fld0.1 = (_1, Move(_23.2.1.1), _23.2.1.2, (*_18));
_23.2.1.3 = (*_18);
_13.0 = _29;
place!(Field::<i128>(Variant(_22, 1), 2)) = _13.2;
_24.fld0.1.0 = _39.1.1.0;
_30 = &_39.1.2;
match _46.1 {
0 => bb26,
1 => bb7,
2 => bb33,
250 => bb61,
_ => bb14
}
}
bb61 = {
_27 = core::ptr::addr_of_mut!(_24.fld0.1.1);
_48 = core::ptr::addr_of_mut!(_30);
(*_27) = core::ptr::addr_of_mut!(_12);
_5 = [30790_u16,59638_u16,47550_u16,49395_u16,53451_u16];
(*_18) = _23.2.0;
(*_27) = core::ptr::addr_of_mut!(_12);
_23.2 = ((*_18), Move(_24.fld0.1), Move(_39.1.2));
_49 = -_16;
(*_48) = &_23.2.2;
_13.2 = _45.2 << _46.2;
_39 = (_34, Move(_23.2));
_23.2.1 = Move(_39.1.1);
_7 = [22649_u16,64818_u16,64911_u16,2831_u16,54478_u16];
_25 = &_42;
Goto(bb62)
}
bb62 = {
_24.fld0.1.0 = [22942_u16,55312_u16,41075_u16,61269_u16,35622_u16];
(*_27) = core::ptr::addr_of_mut!(_12);
place!(Field::<[i32; 1]>(Variant(_22, 1), 1)) = _10;
(*_27) = core::ptr::addr_of_mut!(_58);
(*_27) = core::ptr::addr_of_mut!(_12);
_39.1.1.3 = (*_18);
_39.0 = [_50,_50,_50];
(*_27) = core::ptr::addr_of_mut!(_58);
_25 = &place!(Field::<[i128; 4]>(Variant(_22, 1), 3));
(*_48) = &_39.1.2;
_17 = Field::<f64>(Variant((*_30), 0), 0);
_16 = _49;
_48 = core::ptr::addr_of_mut!((*_48));
_47 = !_13.1;
_35 = [_8,_8,_8,_8,_8];
_46.3 = _50 as i64;
_19 = core::ptr::addr_of!(_17);
_37 = _15;
_23.2.2 = Move(_39.1.2);
_24.fld0.1.2 = _6 ^ _6;
(*_27) = core::ptr::addr_of_mut!(_61.1);
(*_19) = _23.0;
match _13.1 {
0 => bb19,
1 => bb63,
250 => bb65,
_ => bb64
}
}
bb63 = {
(*_21) = Field::<u32>(Variant((*_30), 0), 1) << _28;
_33 = _20 << _13.3;
_23.2.1.1 = core::ptr::addr_of_mut!(_12);
_39.1.1.0 = _1;
(*_21) = Field::<u32>(Variant((*_30), 0), 1) + Field::<u32>(Variant((*_30), 0), 1);
_13.3 = (-6082149593518746089_i64);
_39.1.2 = Move(RET);
_38 = Field::<i128>(Variant(_22, 1), 2);
_39.1.1 = (_24.fld0.1.0, Move(_23.2.1.1), _6, _31);
(*_21) = Field::<u32>(Variant((*_30), 0), 1) * Field::<u32>(Variant((*_30), 0), 1);
match _13.1 {
0 => bb34,
1 => bb49,
2 => bb50,
3 => bb51,
4 => bb52,
5 => bb53,
6 => bb54,
250 => bb56,
_ => bb55
}
}
bb64 = {
Return()
}
bb65 = {
(*_27) = core::ptr::addr_of_mut!(_61.1);
(*_19) = Field::<f64>(Variant(_23.2.2, 0), 0) * _23.0;
_13.0 = -_29;
_17 = 1494432075_i32 as f64;
(*_27) = core::ptr::addr_of_mut!(_12);
_39.1.1.0 = [61691_u16,40795_u16,58381_u16,41399_u16,2832_u16];
_41 = _37;
place!(Field::<i128>(Variant(_22, 1), 2)) = _13.2 | _38;
_56 = [_45.3,_45.3,_43,_46.3,_45.3,_43,_43];
(*_27) = core::ptr::addr_of_mut!(_12);
_54 = &_23.2.2;
match _13.1 {
0 => bb52,
1 => bb7,
2 => bb51,
3 => bb9,
4 => bb5,
5 => bb28,
250 => bb67,
_ => bb66
}
}
bb66 = {
_6 = (-16451_i16);
_9 = (-48_isize) >> _8;
_3 = [_6,_6,_6,_6];
_10 = [(-1675646130_i32)];
_6 = 2615161373_u32 as i16;
_8 = 306339619359669570655359574057326377129_u128;
_4 = [36070_u16,9207_u16,16856_u16,45778_u16,28013_u16];
_6 = (-21585_i16) - (-16118_i16);
_3 = [_6,_6,_6,_6];
_5 = _4;
_12 = '\u{11438}';
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
306339619359669570655359574057326377129 => bb7,
_ => bb6
}
}
bb67 = {
_64 = _12 as i128;
_24.fld0.2 = Adt25::Variant0 { fld0: _23.0,fld1: Field::<u32>(Variant((*_54), 0), 1) };
_19 = core::ptr::addr_of!((*_19));
place!(Field::<i128>(Variant(_22, 1), 2)) = _38 ^ _45.2;
(*_19) = Field::<f64>(Variant((*_54), 0), 0) - Field::<f64>(Variant((*_54), 0), 0);
Goto(bb68)
}
bb68 = {
_45.1 = _46.1 ^ _13.1;
_24.fld0.1.3 = _3;
_39.1.1.1 = core::ptr::addr_of_mut!(_58);
Goto(bb69)
}
bb69 = {
_61.1 = _12;
_34 = [_50,_50,_50];
(*_19) = Field::<f64>(Variant((*_54), 0), 0) * Field::<f64>(Variant(_23.2.2, 0), 0);
_23.3 = core::ptr::addr_of!(_36);
_10 = Field::<[i32; 1]>(Variant(_22, 1), 1);
(*_27) = Move(_23.2.1.1);
_30 = &_24.fld0.2;
_40 = (-1078875414_i32) as i128;
RET = Adt25::Variant0 { fld0: (*_19),fld1: Field::<u32>(Variant((*_54), 0), 1) };
Goto(bb70)
}
bb70 = {
Call(_70 = dump_var(Move(_8), Move(_42), Move(_50), Move(_7)), ReturnTo(bb71), UnwindUnreachable())
}
bb71 = {
Call(_70 = dump_var(Move(_3), Move(_46), Move(_1), Move(_35)), ReturnTo(bb72), UnwindUnreachable())
}
bb72 = {
Call(_70 = dump_var(Move(_12), Move(_5), Move(_6), Move(_47)), ReturnTo(bb73), UnwindUnreachable())
}
bb73 = {
Call(_70 = dump_var(Move(_33), Move(_36), Move(_13), _71), ReturnTo(bb74), UnwindUnreachable())
}
bb74 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: f64,mut _2: char,mut _3: *const f64,mut _4: char,mut _5: *mut char,mut _6: i8) -> Adt42 {
mir! {
type RET = Adt42;
let _7: i32;
let _8: &'static &'static (i32, (usize, u128, f32), f64);
let _9: f32;
let _10: &'static i128;
let _11: [i32; 4];
let _12: *mut f64;
let _13: bool;
let _14: &'static f64;
let _15: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _16: f64;
let _17: [u16; 5];
let _18: f32;
let _19: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _20: *mut *mut char;
let _21: [char; 8];
let _22: [u16; 5];
let _23: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _24: Adt68;
let _25: bool;
let _26: *mut [i16; 4];
let _27: [i16; 4];
let _28: *mut &'static Adt25;
let _29: [isize; 8];
let _30: isize;
let _31: f64;
let _32: isize;
let _33: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _34: Adt42;
let _35: char;
let _36: f64;
let _37: *const f64;
let _38: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _39: u128;
let _40: *const [usize; 3];
let _41: [u8; 2];
let _42: char;
let _43: f64;
let _44: [i32; 1];
let _45: *const i8;
let _46: &'static [i128; 4];
let _47: &'static mut Adt42;
let _48: isize;
let _49: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _50: f64;
let _51: f64;
let _52: [i32; 2];
let _53: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _54: *const u64;
let _55: &'static i128;
let _56: isize;
let _57: ([i128; 4], &'static [char; 4]);
let _58: [i128; 5];
let _59: f64;
let _60: isize;
let _61: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _62: [i16; 4];
let _63: *const i8;
let _64: (&'static Adt25, char, *mut char);
let _65: &'static mut Adt42;
let _66: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _67: [i64; 7];
let _68: f32;
let _69: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _70: *mut *mut char;
let _71: isize;
let _72: *const u64;
let _73: i64;
let _74: f32;
let _75: f32;
let _76: Adt24;
let _77: (usize, u128, f32);
let _78: isize;
let _79: &'static mut Adt42;
let _80: usize;
let _81: [i16; 4];
let _82: isize;
let _83: [i16; 4];
let _84: (isize, u8, i128, i64);
let _85: *mut f64;
let _86: isize;
let _87: u64;
let _88: &'static u8;
let _89: &'static u8;
let _90: u32;
let _91: isize;
let _92: &'static mut usize;
let _93: char;
let _94: *const [char; 4];
let _95: [usize; 3];
let _96: *mut char;
let _97: f64;
let _98: Adt84;
let _99: bool;
let _100: i64;
let _101: &'static (i32, (usize, u128, f32), f64);
let _102: &'static (i32, (usize, u128, f32), f64);
let _103: isize;
let _104: f64;
let _105: f32;
let _106: [u32; 8];
let _107: bool;
let _108: f64;
let _109: Adt42;
let _110: &'static mut Adt42;
let _111: &'static [i128; 4];
let _112: u64;
let _113: f64;
let _114: *const isize;
let _115: *const [char; 4];
let _116: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _117: ();
let _118: ();
{
_6 = 15_i8 | (-1_i8);
_6 = (-45_i8);
_3 = core::ptr::addr_of!(_1);
(*_3) = 1546976181984845834_i64 as f64;
_5 = core::ptr::addr_of_mut!(_2);
(*_5) = _4;
(*_3) = 96684641789432707272094379868059333642_u128 as f64;
_6 = (-52_i8);
(*_5) = _4;
(*_3) = 40181741620890462186820073074290182969_u128 as f64;
(*_5) = _4;
(*_3) = (-9223372036854775808_isize) as f64;
(*_5) = _4;
(*_3) = 3130451156882225641_i64 as f64;
(*_5) = _4;
(*_3) = 2326548614355346090_u64 as f64;
(*_5) = _4;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 16987904817192458007_usize as f64;
(*_5) = _4;
(*_5) = _4;
(*_3) = _6 as f64;
(*_3) = 26928_u16 as f64;
Goto(bb1)
}
bb1 = {
(*_3) = 45926_u16 as f64;
(*_3) = (-14165096211498493103865333755677794582_i128) as f64;
(*_5) = _4;
_13 = (*_5) >= _4;
(*_5) = _4;
_3 = core::ptr::addr_of!((*_3));
(*_5) = _4;
(*_3) = 23780_u16 as f64;
_1 = (-16145_i16) as f64;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 121316646552230304905542558979845225606_u128 as f64;
(*_3) = (-32800370937100605856525282291451775009_i128) as f64;
(*_3) = 167_u8 as f64;
_14 = &(*_3);
_11 = [(-1487698082_i32),(-853527390_i32),555039015_i32,(-228050408_i32)];
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211404 => bb7,
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
(*_3) = 2693219305_u32 as f64;
(*_3) = (-7102660313145136862_i64) as f64;
(*_5) = _4;
(*_3) = (-960438963_i32) as f64;
match _6 {
340282366920938463463374607431768211404 => bb8,
_ => bb3
}
}
bb8 = {
(*_3) = 51794292492215364111148782147651326785_i128 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_3) = 12256_u16 as f64;
(*_5) = _4;
_11 = [(-1018165777_i32),1329196089_i32,460040269_i32,1452580612_i32];
_3 = core::ptr::addr_of!((*_3));
_12 = core::ptr::addr_of_mut!((*_3));
(*_3) = 9223372036854775807_isize as f64;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 23_u8 as f64;
Goto(bb9)
}
bb9 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_9 = _6 as f32;
(*_3) = 62412977095293585203989038898244249846_i128 as f64;
(*_5) = _4;
(*_3) = 3_usize as f64;
(*_3) = 4566_i16 as f64;
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = _4;
_12 = core::ptr::addr_of_mut!((*_3));
(*_3) = _7 as f64;
_3 = core::ptr::addr_of!((*_3));
_4 = (*_5);
(*_5) = _4;
(*_5) = _4;
_18 = _9;
_6 = 12801596049274371031_u64 as i8;
(*_3) = 39817_u16 as f64;
(*_5) = _4;
Goto(bb12)
}
bb12 = {
(*_5) = _4;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 58_u8 as f64;
(*_5) = _4;
_9 = 3895320912_u32 as f32;
(*_3) = 271577054993167853144689934378245847866_u128 as f64;
(*_3) = 3595392902025322588_usize as f64;
_22 = [28120_u16,22519_u16,57896_u16,64450_u16,40244_u16];
_17 = _22;
_4 = (*_5);
(*_5) = _4;
(*_3) = _7 as f64;
(*_3) = 62320_u16 as f64;
(*_3) = 30298108761984924769994212085164769975_i128 as f64;
(*_5) = _4;
(*_5) = _4;
_21 = [(*_5),_2,(*_5),_4,_2,(*_5),(*_5),(*_5)];
Goto(bb13)
}
bb13 = {
_24.fld1 = [70_u8,14_u8,204_u8,96_u8];
(*_3) = (-8762143703947074945_i64) as f64;
_24.fld2 = core::ptr::addr_of!((*_3));
(*_3) = 220695738777653239331253042217875721331_u128 as f64;
_24.fld0.1.0 = [55932_u16,58529_u16,60326_u16,12921_u16,8877_u16];
(*_3) = 171_u8 as f64;
(*_5) = _4;
(*_5) = _4;
_7 = !(-1823518154_i32);
_24.fld0.0 = [19889_i16,12938_i16,11315_i16,20591_i16];
(*_5) = _4;
(*_5) = _4;
(*_3) = 9936_i16 as f64;
(*_3) = 248499620368388777162410304450398709184_u128 as f64;
_24.fld0.1 = (_17, Move(_5), 26706_i16, _24.fld0.0);
(*_3) = _24.fld0.1.2 as f64;
_5 = core::ptr::addr_of_mut!(_4);
(*_3) = _18 as f64;
(*_5) = _2;
_20 = core::ptr::addr_of_mut!(_24.fld0.1.1);
_24.fld0.1.0 = _22;
_4 = _2;
(*_5) = _2;
(*_5) = _2;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!((*_5));
_4 = _2;
match _24.fld0.1.2 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
26706 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
(*_3) = 45926_u16 as f64;
(*_3) = (-14165096211498493103865333755677794582_i128) as f64;
(*_5) = _4;
_13 = (*_5) >= _4;
(*_5) = _4;
_3 = core::ptr::addr_of!((*_3));
(*_5) = _4;
(*_3) = 23780_u16 as f64;
_1 = (-16145_i16) as f64;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 121316646552230304905542558979845225606_u128 as f64;
(*_3) = (-32800370937100605856525282291451775009_i128) as f64;
(*_3) = 167_u8 as f64;
_14 = &(*_3);
_11 = [(-1487698082_i32),(-853527390_i32),555039015_i32,(-228050408_i32)];
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211404 => bb7,
_ => bb6
}
}
bb17 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb18 = {
Return()
}
bb19 = {
(*_3) = _18 as f64;
_16 = -(*_3);
_24.fld0.1.0 = [22793_u16,34141_u16,59496_u16,59597_u16,64331_u16];
_22 = _24.fld0.1.0;
(*_20) = Move(_5);
(*_3) = 34541_u16 as f64;
_29 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = _16 + _16;
_18 = _9 - _9;
(*_3) = _16 - _16;
(*_3) = _16;
match _24.fld0.1.2 {
0 => bb1,
1 => bb20,
2 => bb21,
26706 => bb23,
_ => bb22
}
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
(*_5) = _4;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 58_u8 as f64;
(*_5) = _4;
_9 = 3895320912_u32 as f32;
(*_3) = 271577054993167853144689934378245847866_u128 as f64;
(*_3) = 3595392902025322588_usize as f64;
_22 = [28120_u16,22519_u16,57896_u16,64450_u16,40244_u16];
_17 = _22;
_4 = (*_5);
(*_5) = _4;
(*_3) = _7 as f64;
(*_3) = 62320_u16 as f64;
(*_3) = 30298108761984924769994212085164769975_i128 as f64;
(*_5) = _4;
(*_5) = _4;
_21 = [(*_5),_2,(*_5),_4,_2,(*_5),(*_5),(*_5)];
Goto(bb13)
}
bb23 = {
_25 = !_13;
_24.fld0.2 = Adt25::Variant0 { fld0: (*_3),fld1: 1674228067_u32 };
_20 = core::ptr::addr_of_mut!(_5);
(*_20) = core::ptr::addr_of_mut!(_4);
_3 = core::ptr::addr_of!((*_3));
(*_5) = _2;
(*_3) = Field::<f64>(Variant(_24.fld0.2, 0), 0) * _16;
(*_5) = _2;
_18 = -_9;
(*_20) = core::ptr::addr_of_mut!((*_5));
(*_3) = Field::<f64>(Variant(_24.fld0.2, 0), 0);
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!(_4);
(*_5) = _2;
Goto(bb24)
}
bb24 = {
(*_20) = Move(_24.fld0.1.1);
_2 = _4;
_24.fld0.2 = Adt25::Variant0 { fld0: (*_3),fld1: 3644075572_u32 };
_32 = !(-9223372036854775808_isize);
_31 = (*_3) + (*_3);
(*_3) = -_31;
_18 = (*_3) as f32;
(*_20) = core::ptr::addr_of_mut!(_4);
_24.fld0.1 = (_17, Move((*_20)), (-23778_i16), _24.fld0.0);
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = core::ptr::addr_of_mut!(_4);
(*_5) = _2;
match _24.fld0.1.2 {
0 => bb22,
1 => bb2,
2 => bb20,
3 => bb23,
4 => bb16,
5 => bb25,
6 => bb26,
340282366920938463463374607431768187678 => bb28,
_ => bb27
}
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
(*_20) = core::ptr::addr_of_mut!(_35);
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = 3508378215_u32 >> _24.fld0.1.2;
_24.fld0.1.0 = [41292_u16,61424_u16,63060_u16,23909_u16,43191_u16];
(*_3) = -_31;
(*_5) = _4;
(*_3) = _31 * _16;
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = 160137845_u32 * 3926319887_u32;
(*_3) = _31 - Field::<f64>(Variant(_24.fld0.2, 0), 0);
match _24.fld0.1.2 {
0 => bb14,
1 => bb27,
2 => bb26,
3 => bb9,
4 => bb22,
340282366920938463463374607431768187678 => bb29,
_ => bb13
}
}
bb29 = {
_25 = _13;
(*_5) = _2;
_37 = core::ptr::addr_of!(_16);
_35 = _4;
_24.fld0.0 = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2];
_24.fld0.1.0 = [15712_u16,45379_u16,63252_u16,22_u16,50260_u16];
_27 = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2];
(*_5) = _2;
Goto(bb30)
}
bb30 = {
_24.fld0.1.2 = _6 as i16;
_33 = &_24.fld0.1;
(*_3) = -(*_37);
(*_20) = Move(_24.fld0.1.1);
(*_3) = (*_37) * (*_37);
Goto(bb31)
}
bb31 = {
_24.fld0.1.0 = _22;
(*_20) = core::ptr::addr_of_mut!(_42);
(*_3) = (*_37) * (*_37);
_21 = [_4,_35,_35,_2,_4,_4,_4,_4];
(*_37) = (*_3) * (*_3);
(*_5) = _2;
_6 = _13 as i8;
(*_37) = (*_3) * _1;
(*_37) = (*_3);
_18 = -_9;
_9 = _18;
_27 = _24.fld0.0;
(*_37) = (-4905606724093915289_i64) as f64;
_39 = !52166436084429250451678054449131215017_u128;
(*_5) = _2;
(*_3) = _31;
_13 = !_25;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!(_4);
(*_3) = (*_37) * (*_37);
_13 = (*_33).2 >= (*_33).2;
_44 = [_7];
Goto(bb32)
}
bb32 = {
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = 721631861_u32;
(*_3) = (*_37) + (*_37);
_36 = 97063120075351560391192645496208211156_i128 as f64;
_4 = _2;
_18 = _9;
_2 = (*_5);
_18 = _9;
_7 = -(-1854907551_i32);
_39 = (*_3) as u128;
(*_20) = core::ptr::addr_of_mut!((*_5));
_51 = (*_37);
(*_5) = _2;
(*_5) = _42;
(*_37) = Field::<f64>(Variant(_24.fld0.2, 0), 0) * (*_3);
(*_20) = core::ptr::addr_of_mut!((*_5));
(*_3) = (*_37) + _16;
_53.1.0 = [55755_u16,22731_u16,7331_u16,48326_u16,16550_u16];
_24.fld0.0 = [_24.fld0.1.2,(*_33).2,(*_33).2,(*_33).2];
match Field::<u32>(Variant(_24.fld0.2, 0), 1) {
0 => bb13,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
6 => bb38,
721631861 => bb40,
_ => bb39
}
}
bb33 = {
_24.fld0.1.0 = _22;
(*_20) = core::ptr::addr_of_mut!(_42);
(*_3) = (*_37) * (*_37);
_21 = [_4,_35,_35,_2,_4,_4,_4,_4];
(*_37) = (*_3) * (*_3);
(*_5) = _2;
_6 = _13 as i8;
(*_37) = (*_3) * _1;
(*_37) = (*_3);
_18 = -_9;
_9 = _18;
_27 = _24.fld0.0;
(*_37) = (-4905606724093915289_i64) as f64;
_39 = !52166436084429250451678054449131215017_u128;
(*_5) = _2;
(*_3) = _31;
_13 = !_25;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!(_4);
(*_3) = (*_37) * (*_37);
_13 = (*_33).2 >= (*_33).2;
_44 = [_7];
Goto(bb32)
}
bb34 = {
_24.fld0.1.2 = _6 as i16;
_33 = &_24.fld0.1;
(*_3) = -(*_37);
(*_20) = Move(_24.fld0.1.1);
(*_3) = (*_37) * (*_37);
Goto(bb31)
}
bb35 = {
_9 = _6 as f32;
(*_3) = 62412977095293585203989038898244249846_i128 as f64;
(*_5) = _4;
(*_3) = 3_usize as f64;
(*_3) = 4566_i16 as f64;
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = _4;
_12 = core::ptr::addr_of_mut!((*_3));
(*_3) = _7 as f64;
_3 = core::ptr::addr_of!((*_3));
_4 = (*_5);
(*_5) = _4;
(*_5) = _4;
_18 = _9;
_6 = 12801596049274371031_u64 as i8;
(*_3) = 39817_u16 as f64;
(*_5) = _4;
Goto(bb12)
}
bb36 = {
Return()
}
bb37 = {
Return()
}
bb38 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb39 = {
_24.fld1 = [70_u8,14_u8,204_u8,96_u8];
(*_3) = (-8762143703947074945_i64) as f64;
_24.fld2 = core::ptr::addr_of!((*_3));
(*_3) = 220695738777653239331253042217875721331_u128 as f64;
_24.fld0.1.0 = [55932_u16,58529_u16,60326_u16,12921_u16,8877_u16];
(*_3) = 171_u8 as f64;
(*_5) = _4;
(*_5) = _4;
_7 = !(-1823518154_i32);
_24.fld0.0 = [19889_i16,12938_i16,11315_i16,20591_i16];
(*_5) = _4;
(*_5) = _4;
(*_3) = 9936_i16 as f64;
(*_3) = 248499620368388777162410304450398709184_u128 as f64;
_24.fld0.1 = (_17, Move(_5), 26706_i16, _24.fld0.0);
(*_3) = _24.fld0.1.2 as f64;
_5 = core::ptr::addr_of_mut!(_4);
(*_3) = _18 as f64;
(*_5) = _2;
_20 = core::ptr::addr_of_mut!(_24.fld0.1.1);
_24.fld0.1.0 = _22;
_4 = _2;
(*_5) = _2;
(*_5) = _2;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!((*_5));
_4 = _2;
match _24.fld0.1.2 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
26706 => bb19,
_ => bb18
}
}
bb40 = {
(*_5) = _35;
_53.0 = [(*_33).2,(*_33).2,(*_33).2,(*_33).2];
(*_37) = (*_3) * (*_3);
(*_20) = core::ptr::addr_of_mut!((*_5));
(*_5) = _42;
_50 = -(*_3);
(*_37) = (*_3) - (*_3);
_36 = _32 as f64;
(*_3) = (*_37);
_24.fld0.1.2 = !29493_i16;
_42 = (*_5);
(*_37) = -(*_3);
_53.1.1 = core::ptr::addr_of_mut!((*_5));
(*_37) = (*_3) - (*_3);
(*_37) = _32 as f64;
_3 = core::ptr::addr_of!((*_3));
_43 = (*_3);
_12 = core::ptr::addr_of_mut!((*_37));
place!(Field::<f64>(Variant(_24.fld0.2, 0), 0)) = (*_3) - (*_3);
(*_20) = Move(_53.1.1);
(*_12) = 77607413244216667146035549781323329631_i128 as f64;
_37 = core::ptr::addr_of!((*_37));
(*_20) = core::ptr::addr_of_mut!(_2);
place!(Field::<f64>(Variant(_24.fld0.2, 0), 0)) = _24.fld0.1.2 as f64;
_56 = _32 << _32;
Goto(bb41)
}
bb41 = {
(*_12) = (*_3) - (*_3);
_48 = _56;
_41 = [126_u8,82_u8];
(*_12) = (*_3);
(*_12) = (*_3) + (*_3);
_61.1.1 = (_17, Move((*_20)), _24.fld0.1.2, _27);
Goto(bb42)
}
bb42 = {
(*_3) = _48 as f64;
_61.1.1.0 = [31102_u16,42430_u16,35729_u16,39957_u16,3388_u16];
(*_12) = -_50;
match Field::<u32>(Variant(_24.fld0.2, 0), 1) {
721631861 => bb44,
_ => bb43
}
}
bb43 = {
_24.fld0.1.0 = _22;
(*_20) = core::ptr::addr_of_mut!(_42);
(*_3) = (*_37) * (*_37);
_21 = [_4,_35,_35,_2,_4,_4,_4,_4];
(*_37) = (*_3) * (*_3);
(*_5) = _2;
_6 = _13 as i8;
(*_37) = (*_3) * _1;
(*_37) = (*_3);
_18 = -_9;
_9 = _18;
_27 = _24.fld0.0;
(*_37) = (-4905606724093915289_i64) as f64;
_39 = !52166436084429250451678054449131215017_u128;
(*_5) = _2;
(*_3) = _31;
_13 = !_25;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!(_4);
(*_3) = (*_37) * (*_37);
_13 = (*_33).2 >= (*_33).2;
_44 = [_7];
Goto(bb32)
}
bb44 = {
_62 = [_24.fld0.1.2,_61.1.1.2,_24.fld0.1.2,_61.1.1.2];
_30 = 8592392483894313268_i64 as isize;
_26 = core::ptr::addr_of_mut!((*_33).3);
_61.1.0 = [_61.1.1.2,_61.1.1.2,_24.fld0.1.2,_24.fld0.1.2];
_53.1.1 = core::ptr::addr_of_mut!(_35);
Call((*_3) = core::intrinsics::transmute((*_26)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
(*_3) = (*_12) + (*_12);
(*_12) = (*_3);
_53.1.0 = _17;
_50 = (*_3);
_60 = _32 & _56;
(*_3) = Field::<u32>(Variant(_24.fld0.2, 0), 1) as f64;
_52 = [_7,_7];
(*_20) = core::ptr::addr_of_mut!(_35);
(*_5) = _42;
(*_20) = Move(_61.1.1.1);
(*_20) = core::ptr::addr_of_mut!(_4);
_41 = [81_u8,205_u8];
(*_20) = core::ptr::addr_of_mut!(_2);
(*_3) = (-164454895058933040690474670241849937435_i128) as f64;
_53.0 = (*_33).3;
_41 = [178_u8,242_u8];
_53.1 = (_24.fld0.1.0, Move((*_20)), _61.1.1.2, (*_26));
_45 = core::ptr::addr_of!(_6);
(*_20) = core::ptr::addr_of_mut!(_2);
(*_12) = _18 as f64;
_30 = _48;
_61.0 = [1_usize,5541152064012870484_usize,5_usize];
(*_3) = (*_12);
(*_12) = -(*_3);
_37 = core::ptr::addr_of!((*_12));
_37 = Move(_24.fld2);
match Field::<u32>(Variant(_24.fld0.2, 0), 1) {
0 => bb14,
1 => bb46,
2 => bb47,
721631861 => bb49,
_ => bb48
}
}
bb46 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb47 = {
_9 = _6 as f32;
(*_3) = 62412977095293585203989038898244249846_i128 as f64;
(*_5) = _4;
(*_3) = 3_usize as f64;
(*_3) = 4566_i16 as f64;
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = _4;
_12 = core::ptr::addr_of_mut!((*_3));
(*_3) = _7 as f64;
_3 = core::ptr::addr_of!((*_3));
_4 = (*_5);
(*_5) = _4;
(*_5) = _4;
_18 = _9;
_6 = 12801596049274371031_u64 as i8;
(*_3) = 39817_u16 as f64;
(*_5) = _4;
Goto(bb12)
}
bb48 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb49 = {
(*_5) = _4;
(*_20) = core::ptr::addr_of_mut!((*_5));
(*_5) = _35;
(*_20) = core::ptr::addr_of_mut!((*_5));
_66.0.1.3 = (*_26);
(*_3) = _43 + (*_12);
_61.1 = ((*_26), Move(_53.1), Move(_24.fld0.2));
place!(Field::<u32>(Variant(_61.1.2, 0), 1)) = 1640979903_u32;
_16 = (*_3) + (*_3);
_5 = Move(_61.1.1.1);
_53.1.3 = [_61.1.1.2,_24.fld0.1.2,_24.fld0.1.2,_61.1.1.2];
_66.0.1.2 = _24.fld0.1.2 | _24.fld0.1.2;
_6 = !48_i8;
(*_3) = (*_12) - (*_12);
(*_12) = _60 as f64;
_53.2 = Move(_61.1.2);
_59 = _1;
(*_12) = (*_3) + (*_3);
_66.0.1.0 = [5290_u16,15879_u16,36398_u16,30948_u16,5934_u16];
(*_3) = _66.0.1.2 as f64;
_66.0.2 = Move(_53.2);
_1 = (*_45) as f64;
Call(_63 = core::intrinsics::arith_offset(_45, 9223372036854775807_isize), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
_6 = (-54_i8) >> _39;
(*_12) = _59;
(*_45) = 113_i8 - 9_i8;
_20 = core::ptr::addr_of_mut!((*_20));
_35 = _42;
(*_12) = _9 as f64;
_50 = (*_12);
_45 = core::ptr::addr_of!((*_45));
_64.1 = _2;
_50 = _59;
(*_3) = -_59;
_61.1.1 = (_66.0.1.0, Move((*_20)), _24.fld0.1.2, (*_26));
_14 = &(*_12);
_74 = _9 - _9;
_70 = Move(_20);
match Field::<u32>(Variant(_66.0.2, 0), 1) {
0 => bb1,
1 => bb34,
2 => bb9,
1640979903 => bb52,
_ => bb51
}
}
bb51 = {
Return()
}
bb52 = {
_31 = -(*_3);
(*_3) = _50;
(*_3) = (*_14) - _31;
_24.fld0.1 = (_66.0.1.0, Move(_61.1.1.1), _61.1.1.2, _53.0);
_64.2 = core::ptr::addr_of_mut!(_64.1);
(*_3) = (*_45) as f64;
(*_45) = 198_u8 as i8;
(*_12) = _50;
(*_45) = (-113_i8) - (-89_i8);
_53 = (_27, Move(_24.fld0.1), Move(_66.0.2));
(*_3) = (*_12) - _50;
_51 = _31 - (*_12);
_61.1 = (_27, Move(_53.1), Move(_53.2));
(*_3) = (*_12) * _51;
_24.fld0.2 = Move(_61.1.2);
place!(Field::<u32>(Variant(_24.fld0.2, 0), 1)) = 5291726_u32 + 2587382023_u32;
_57.0 = [(-77269942419601956265462537248177458479_i128),113529667157475336243982378099398304515_i128,(-151316554649273992029367409877315147265_i128),(-33849033010577845314256335518733839380_i128)];
_61.1.0 = [_66.0.1.2,_66.0.1.2,_61.1.1.2,_66.0.1.2];
_61.1.1 = (_66.0.1.0, Move(_64.2), _66.0.1.2, _53.0);
_50 = (*_3) - (*_3);
_75 = -_18;
(*_12) = 47055_u16 as f64;
_40 = core::ptr::addr_of!(_61.0);
(*_45) = (-6_i8) | 64_i8;
Goto(bb53)
}
bb53 = {
_49 = &_61.1.1;
_70 = core::ptr::addr_of_mut!((*_49).1);
(*_3) = Field::<u32>(Variant(_24.fld0.2, 0), 1) as f64;
_61.1.0 = (*_49).3;
(*_40) = [7_usize,4_usize,7_usize];
_66.0.0 = (*_49).3;
(*_3) = _51;
Goto(bb54)
}
bb54 = {
_74 = 6294600634223861454_u64 as f32;
_66.0.1.1 = Move(_61.1.1.1);
_24.fld2 = Move(_3);
(*_12) = -_51;
(*_40) = [16285151322881998946_usize,16923344888524435528_usize,11320415712647975396_usize];
_23 = &_66.0.1;
(*_40) = [6048051530580900691_usize,1_usize,5104948113945082382_usize];
_43 = (-6188324786347200846_i64) as f64;
_24.fld0.1.1 = Move(_66.0.1.1);
_67 = [675972790716860726_i64,(-8535829568657392701_i64),(-2447538642497168805_i64),(-2875711752426380411_i64),(-1248861833102308165_i64),(-8279531121641417894_i64),5880418263304393559_i64];
_61.1.1.2 = _66.0.1.2;
(*_40) = [9273561098262536646_usize,2_usize,2_usize];
_36 = -(*_12);
Call((*_45) = core::intrinsics::bswap((-120_i8)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
_24.fld0.1.0 = (*_23).0;
_41 = [189_u8,170_u8];
_41 = [14_u8,159_u8];
_66.0.0 = (*_23).3;
(*_45) = 110_i8 * 125_i8;
_53.1.3 = [(*_23).2,_61.1.1.2,(*_23).2,(*_23).2];
Goto(bb56)
}
bb56 = {
_27 = [(*_23).2,(*_23).2,(*_23).2,(*_23).2];
_64.2 = core::ptr::addr_of_mut!(_4);
_53.1.1 = core::ptr::addr_of_mut!(_2);
_64.0 = &_24.fld0.2;
(*_45) = 74_i8;
(*_40) = [1661289655231451218_usize,12173654512526923328_usize,0_usize];
_77.0 = _42 as usize;
_61.1.2 = Adt25::Variant0 { fld0: (*_12),fld1: Field::<u32>(Variant(_24.fld0.2, 0), 1) };
Goto(bb57)
}
bb57 = {
_54 = core::ptr::addr_of!(_87);
(*_12) = -Field::<f64>(Variant(_61.1.2, 0), 0);
(*_54) = 15663269332369253960_u64 - 9280127224942847035_u64;
_61.1.1.1 = core::ptr::addr_of_mut!(_64.1);
_41 = [99_u8,244_u8];
_17 = [28647_u16,12355_u16,16301_u16,11236_u16,19846_u16];
_61.1.1.3 = _53.1.3;
(*_45) = !0_i8;
_85 = core::ptr::addr_of_mut!((*_12));
_53 = Move(_61.1);
_26 = core::ptr::addr_of_mut!(_27);
_64.1 = _42;
_4 = _64.1;
_83 = (*_26);
(*_40) = [_77.0,_77.0,_77.0];
_66.0.1 = Move(_53.1);
_53.1.2 = _66.0.1.2 & _66.0.1.2;
(*_54) = 13845825205942405826_u64;
_16 = _31 - _36;
_72 = core::ptr::addr_of!((*_54));
_80 = !_77.0;
_74 = Field::<u32>(Variant(_24.fld0.2, 0), 1) as f32;
_61.1.2 = Adt25::Variant0 { fld0: (*_85),fld1: Field::<u32>(Variant(_24.fld0.2, 0), 1) };
(*_45) = (-67_i8) ^ 117_i8;
_24.fld1 = [55_u8,223_u8,32_u8,194_u8];
Call((*_54) = core::intrinsics::bswap(5673104405913865777_u64), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
_24.fld0.1.2 = _66.0.1.2;
_16 = _1 - _51;
(*_40) = [_80,_80,_77.0];
(*_54) = 8329608594685380636_u64 >> _60;
(*_54) = !3112051788140833618_u64;
_53.1.1 = core::ptr::addr_of_mut!(_2);
(*_45) = 123_i8 | (-107_i8);
_6 = _39 as i8;
(*_12) = _77.0 as f64;
_75 = -_74;
(*_54) = 8954385050870850937_u64 ^ 12350710705476683648_u64;
_42 = _2;
Goto(bb59)
}
bb59 = {
_16 = _31;
(*_26) = _53.0;
(*_45) = (-68_i8);
(*_40) = [_80,_80,_77.0];
(*_26) = [_66.0.1.2,_53.1.2,_66.0.1.2,_53.1.2];
(*_12) = Field::<f64>(Variant(_53.2, 0), 0) + _31;
(*_12) = _60 as f64;
_66.2 = &mut _80;
_66.0.1.1 = core::ptr::addr_of_mut!(_64.1);
(*_26) = _66.0.1.3;
_14 = &place!(Field::<f64>(Variant(_24.fld0.2, 0), 0));
Goto(bb60)
}
bb60 = {
_73 = 3074890610537703192_i64;
(*_54) = _60 as u64;
_49 = &_66.0.1;
(*_54) = 5718346026749630757_u64 & 5658298709854208670_u64;
(*_40) = [_77.0,_77.0,_77.0];
_24.fld0 = (_53.0, Move(_66.0.1), Move(_61.1.2));
_11 = [_7,_7,_7,_7];
(*_26) = _53.0;
_84.2 = 109776552975672041510150147116302428896_i128 * (-116950563454584272804770132772549839038_i128);
_24.fld0.1 = (_17, Move(_53.1.1), _53.1.2, (*_26));
_90 = Field::<u32>(Variant(_24.fld0.2, 0), 1);
(*_12) = _31 * Field::<f64>(Variant(_24.fld0.2, 0), 0);
_24.fld0.0 = (*_26);
(*_26) = _24.fld0.0;
_58 = [_84.2,_84.2,_84.2,_84.2,_84.2];
_95 = [_77.0,_77.0,_77.0];
_30 = _90 as isize;
_4 = _64.1;
(*_45) = _84.2 as i8;
_64.2 = core::ptr::addr_of_mut!(_4);
(*_26) = _24.fld0.1.3;
(*_12) = Field::<u32>(Variant(_53.2, 0), 1) as f64;
(*_45) = 121_i8 >> (*_54);
_61.1.1.3 = [_53.1.2,_53.1.2,_24.fld0.1.2,_53.1.2];
Goto(bb61)
}
bb61 = {
(*_26) = [_24.fld0.1.2,_24.fld0.1.2,_53.1.2,_24.fld0.1.2];
_71 = _60;
(*_45) = -(-67_i8);
_22 = [55654_u16,56346_u16,55819_u16,29381_u16,64086_u16];
_53.1.3 = [_53.1.2,_24.fld0.1.2,_24.fld0.1.2,_53.1.2];
_99 = _13;
(*_45) = 117_i8 * (-74_i8);
_12 = core::ptr::addr_of_mut!((*_12));
_57.0 = [_84.2,_84.2,_84.2,_84.2];
_61.1.0 = [_24.fld0.1.2,_24.fld0.1.2,_53.1.2,_24.fld0.1.2];
_24.fld2 = core::ptr::addr_of!((*_12));
(*_12) = -_51;
_66.0.2 = Move(_24.fld0.2);
_66.0.1.1 = core::ptr::addr_of_mut!(_42);
_86 = !_71;
_82 = _56 | _30;
_24.fld0.1 = (_17, Move(_66.0.1.1), _53.1.2, _66.0.0);
_43 = _59 + _50;
(*_40) = _95;
(*_26) = [_24.fld0.1.2,_24.fld0.1.2,_24.fld0.1.2,_53.1.2];
_53 = ((*_26), Move(_24.fld0.1), Move(_66.0.2));
(*_45) = 46_i8 + (-90_i8);
(*_45) = (-83_i8) | (-93_i8);
(*_12) = _36 * _59;
match _73 {
0 => bb62,
3074890610537703192 => bb64,
_ => bb63
}
}
bb62 = {
Return()
}
bb63 = {
_24.fld0.1.0 = _22;
(*_20) = core::ptr::addr_of_mut!(_42);
(*_3) = (*_37) * (*_37);
_21 = [_4,_35,_35,_2,_4,_4,_4,_4];
(*_37) = (*_3) * (*_3);
(*_5) = _2;
_6 = _13 as i8;
(*_37) = (*_3) * _1;
(*_37) = (*_3);
_18 = -_9;
_9 = _18;
_27 = _24.fld0.0;
(*_37) = (-4905606724093915289_i64) as f64;
_39 = !52166436084429250451678054449131215017_u128;
(*_5) = _2;
(*_3) = _31;
_13 = !_25;
(*_5) = _2;
(*_20) = core::ptr::addr_of_mut!(_4);
(*_3) = (*_37) * (*_37);
_13 = (*_33).2 >= (*_33).2;
_44 = [_7];
Goto(bb32)
}
bb64 = {
_53.1.2 = (-22971_i16) - (-3610_i16);
_14 = &(*_12);
_62 = [_53.1.2,_53.1.2,_53.1.2,_53.1.2];
_27 = [_53.1.2,_53.1.2,_53.1.2,_53.1.2];
_91 = -_48;
_46 = &_57.0;
(*_45) = 13_i8;
_61 = (_95, Move(_53));
_92 = &mut _77.0;
_24.fld0.1 = Move(_61.1.1);
Goto(bb65)
}
bb65 = {
_61.1.1.0 = [38162_u16,28152_u16,32861_u16,49338_u16,7368_u16];
_66.0 = ((*_26), Move(_24.fld0.1), Move(_61.1.2));
(*_45) = _66.0.1.2 as i8;
(*_12) = _50 - _51;
_90 = Field::<u32>(Variant(_66.0.2, 0), 1);
(*_92) = _73 as usize;
_3 = core::ptr::addr_of!(_50);
(*_12) = (*_3) + (*_3);
_61.1.0 = (*_26);
_66.0.1.2 = (-26826_i16);
(*_12) = _59 * (*_3);
(*_26) = [_66.0.1.2,_66.0.1.2,_66.0.1.2,_66.0.1.2];
(*_3) = _1 - (*_12);
(*_54) = !68433120855044113_u64;
(*_40) = [(*_92),(*_92),(*_92)];
(*_54) = 2880553871042602987_u64 ^ 8354457167675768121_u64;
(*_3) = 75_u8 as f64;
(*_26) = [_66.0.1.2,_66.0.1.2,_66.0.1.2,_66.0.1.2];
(*_92) = 11243996052510569078_usize;
_33 = &_66.0.1;
_97 = (*_12) + (*_12);
_103 = 24006_u16 as isize;
_53.1 = ((*_33).0, Move(_64.2), (*_33).2, (*_33).3);
match (*_33).2 {
0 => bb59,
1 => bb19,
2 => bb11,
3 => bb61,
4 => bb32,
5 => bb66,
6 => bb67,
340282366920938463463374607431768184630 => bb69,
_ => bb68
}
}
bb66 = {
Return()
}
bb67 = {
_18 = (*_3) as f32;
(*_5) = _4;
(*_5) = _4;
(*_3) = _18 as f64;
_12 = core::ptr::addr_of_mut!((*_3));
(*_5) = _4;
(*_12) = (-44_isize) as f64;
_1 = 1859564811_i32 as f64;
(*_3) = 9223372036854775807_isize as f64;
_9 = _18 + _18;
_17 = [34360_u16,10148_u16,47393_u16,56014_u16,40784_u16];
(*_5) = _4;
_21 = [(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),_2];
_7 = !(-125816905_i32);
(*_3) = 345_i16 as f64;
match _6 {
340282366920938463463374607431768211404 => bb11,
_ => bb10
}
}
bb68 = {
Return()
}
bb69 = {
(*_92) = !14499312155754224386_usize;
_70 = core::ptr::addr_of_mut!((*_33).1);
(*_40) = [(*_92),(*_92),(*_92)];
_24.fld0.1.3 = [(*_33).2,(*_33).2,_53.1.2,(*_33).2];
_16 = _7 as f64;
_5 = core::ptr::addr_of_mut!(_93);
(*_5) = _42;
_53.1.3 = [(*_33).2,(*_33).2,(*_33).2,(*_33).2];
_105 = _74;
_53.0 = [(*_33).2,(*_33).2,(*_33).2,(*_33).2];
(*_54) = !1656061805087597522_u64;
(*_92) = 3939689593664069575_usize >> (*_33).2;
(*_5) = _64.1;
_59 = _43 - _31;
_84 = (_60, 208_u8, 154667553499189329909394543776361417237_i128, _73);
_61 = (_95, Move(_66.0));
_61.1.1.1 = core::ptr::addr_of_mut!((*_5));
_98 = Adt84::Variant0 { fld0: _43,fld1: Move(_85) };
(*_3) = _74 as f64;
_64.1 = (*_5);
_66.0.1.3 = [_53.1.2,_53.1.2,_53.1.2,_53.1.2];
(*_5) = _64.1;
Goto(bb70)
}
bb70 = {
_61.1.1.1 = core::ptr::addr_of_mut!((*_5));
(*_12) = -_97;
_21 = [_64.1,(*_5),(*_5),(*_5),(*_5),(*_5),(*_5),(*_5)];
(*_3) = (*_12) * (*_12);
_109 = Adt42::Variant1 { fld0: (*_26),fld1: _44,fld2: _84.2,fld3: (*_46) };
_106 = [_90,Field::<u32>(Variant(_61.1.2, 0), 1),_90,Field::<u32>(Variant(_61.1.2, 0), 1),Field::<u32>(Variant(_61.1.2, 0), 1),_90,Field::<u32>(Variant(_61.1.2, 0), 1),_90];
(*_26) = _61.1.0;
_10 = &place!(Field::<i128>(Variant(_109, 1), 2));
(*_5) = _35;
_81 = (*_26);
(*_54) = !12291405309382534449_u64;
(*_12) = -_50;
(*_40) = _95;
_33 = &_61.1.1;
_104 = _39 as f64;
(*_5) = _4;
(*_5) = _42;
place!(Field::<f64>(Variant(_98, 0), 0)) = (*_12) - (*_3);
_60 = _84.3 as isize;
_73 = _7 as i64;
Goto(bb71)
}
bb71 = {
(*_3) = -(*_12);
_66.0.1 = ((*_33).0, Move(_53.1.1), (*_33).2, _24.fld0.0);
(*_26) = (*_33).3;
_100 = _84.2 as i64;
(*_5) = _35;
Call((*_92) = core::intrinsics::bswap(1_usize), ReturnTo(bb72), UnwindUnreachable())
}
bb72 = {
(*_5) = _2;
_24.fld0.1.0 = [32850_u16,24221_u16,18380_u16,58186_u16,9697_u16];
(*_12) = (*_3) + _50;
_90 = (*_10) as u32;
_82 = _60;
(*_12) = -(*_3);
_64.1 = (*_5);
_24.fld0.1.2 = (*_33).2;
(*_45) = (*_33).2 as i8;
_78 = _84.0 >> _73;
_66.0.1.0 = [52439_u16,43855_u16,50834_u16,36193_u16,24871_u16];
_3 = core::ptr::addr_of!((*_12));
_66.0.1.3 = [(*_33).2,(*_33).2,(*_33).2,(*_33).2];
_2 = _93;
RET = _109;
_43 = (*_12) - (*_12);
_66.2 = &mut (*_92);
(*_3) = (*_10) as f64;
(*_5) = _35;
_50 = -(*_12);
_73 = _100 >> (*_45);
_53.1.1 = core::ptr::addr_of_mut!((*_5));
(*_5) = _4;
(*_3) = Field::<f64>(Variant(_98, 0), 0) + _43;
(*_12) = _1 * _97;
(*_54) = 2109028991455460246_u64 | 8962596384105499905_u64;
(*_12) = -_1;
Goto(bb73)
}
bb73 = {
Call(_117 = dump_var(Move(_90), Move(_87), Move(_81), Move(_62)), ReturnTo(bb74), UnwindUnreachable())
}
bb74 = {
Call(_117 = dump_var(Move(_80), Move(_41), Move(_35), Move(_4)), ReturnTo(bb75), UnwindUnreachable())
}
bb75 = {
Call(_117 = dump_var(Move(_17), Move(_106), Move(_11), Move(_39)), ReturnTo(bb76), UnwindUnreachable())
}
bb76 = {
Call(_117 = dump_var(Move(_42), Move(_30), Move(_29), Move(_6)), ReturnTo(bb77), UnwindUnreachable())
}
bb77 = {
Call(_117 = dump_var(Move(_93), Move(_100), Move(_95), Move(_58)), ReturnTo(bb78), UnwindUnreachable())
}
bb78 = {
Call(_117 = dump_var(Move(_56), Move(_103), _118, _118), ReturnTo(bb79), UnwindUnreachable())
}
bb79 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: f64,mut _2: *mut [i16; 4],mut _3: i16,mut _4: *mut char,mut _5: *const f64,mut _6: ([u16; 5], *mut char, i16, [i16; 4])) -> ([u16; 5], *mut char, i16, [i16; 4]) {
mir! {
type RET = ([u16; 5], *mut char, i16, [i16; 4]);
let _7: bool;
let _8: isize;
let _9: *const f64;
let _10: [i16; 4];
let _11: [i32; 4];
let _12: f64;
let _13: u64;
let _14: &'static (i32, (usize, u128, f32), f64);
let _15: i128;
let _16: [char; 6];
let _17: (i32, (usize, u128, f32), f64);
let _18: *mut u32;
let _19: u8;
let _20: i32;
let _21: isize;
let _22: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _23: isize;
let _24: &'static [i128; 4];
let _25: isize;
let _26: &'static mut Adt42;
let _27: [u8; 4];
let _28: [char; 4];
let _29: bool;
let _30: Adt24;
let _31: &'static (i32, (usize, u128, f32), f64);
let _32: char;
let _33: *mut u32;
let _34: &'static mut usize;
let _35: ([u16; 5], *mut char, i16, [i16; 4]);
let _36: u16;
let _37: u128;
let _38: [u32; 8];
let _39: &'static &'static (i32, (usize, u128, f32), f64);
let _40: *const u64;
let _41: ([i128; 4], &'static [char; 4]);
let _42: u64;
let _43: char;
let _44: [i32; 4];
let _45: &'static mut usize;
let _46: u32;
let _47: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _48: u8;
let _49: [i32; 1];
let _50: i128;
let _51: u64;
let _52: &'static f64;
let _53: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _54: isize;
let _55: *mut &'static Adt25;
let _56: [isize; 8];
let _57: i64;
let _58: i64;
let _59: ([i32; 1], &'static mut usize, Adt24);
let _60: *mut u32;
let _61: &'static mut u16;
let _62: i8;
let _63: *const isize;
let _64: &'static u8;
let _65: *mut *mut char;
let _66: u64;
let _67: ([u16; 5], *mut char, i16, [i16; 4]);
let _68: isize;
let _69: *mut f64;
let _70: bool;
let _71: i32;
let _72: i128;
let _73: [char; 6];
let _74: bool;
let _75: i64;
let _76: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _77: isize;
let _78: *mut *mut char;
let _79: [i16; 5];
let _80: isize;
let _81: Adt84;
let _82: (&'static (i32, (usize, u128, f32), f64), f32);
let _83: &'static mut Adt42;
let _84: *const isize;
let _85: u32;
let _86: &'static &'static (i32, (usize, u128, f32), f64);
let _87: i16;
let _88: u128;
let _89: [i128; 4];
let _90: [u8; 5];
let _91: &'static i128;
let _92: Adt18;
let _93: [u8; 4];
let _94: isize;
let _95: u16;
let _96: &'static i128;
let _97: *mut char;
let _98: &'static u8;
let _99: *mut char;
let _100: bool;
let _101: &'static Adt25;
let _102: ();
let _103: ();
{
Goto(bb1)
}
bb1 = {
RET.3 = _6.3;
_3 = !_6.2;
_6.0 = [20301_u16,6778_u16,62093_u16,10142_u16,50956_u16];
_5 = core::ptr::addr_of!(_1);
_6.2 = (-81096771222829383020329407818747136588_i128) as i16;
_6.3 = [_3,_6.2,_3,_6.2];
_5 = core::ptr::addr_of!((*_5));
RET.2 = _3 & _3;
(*_5) = 39_isize as f64;
(*_5) = 3462157587815737871_usize as f64;
_9 = core::ptr::addr_of!(_1);
(*_9) = 204_u8 as f64;
(*_9) = 3598101099677466376_u64 as f64;
(*_5) = 96427014_u32 as f64;
(*_5) = 13309792493722570311_u64 as f64;
(*_5) = _3 as f64;
(*_5) = 1414647125_i32 as f64;
(*_5) = 99_i8 as f64;
(*_5) = 6_usize as f64;
_7 = !true;
RET.2 = _3 >> _6.2;
(*_5) = 100533984918966926560506812517768127551_u128 as f64;
RET.3 = [RET.2,_3,_6.2,RET.2];
_7 = true & false;
_2 = core::ptr::addr_of_mut!(RET.3);
Goto(bb2)
}
bb2 = {
(*_2) = _6.3;
_6.0 = [37356_u16,37620_u16,41214_u16,42084_u16,46461_u16];
RET.1 = Move(_4);
(*_5) = 4_usize as f64;
_4 = Move(_6.1);
_13 = 2812986823739653616_u64 >> _3;
RET.3 = _6.3;
(*_5) = RET.2 as f64;
_6.1 = Move(_4);
_9 = core::ptr::addr_of!((*_5));
(*_9) = 126619817494553622972024215396105691768_i128 as f64;
(*_2) = [RET.2,_3,RET.2,RET.2];
_12 = (*_5);
(*_2) = [RET.2,RET.2,RET.2,_6.2];
RET.2 = _6.2 + _3;
(*_5) = (-7215593151832051266_i64) as f64;
_15 = (-105739430169054147055672348330232127699_i128) | 82778824704871744956375053252649039142_i128;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [_3,_6.2,_6.2,_6.2];
_12 = -_1;
(*_2) = _6.3;
(*_5) = 3327342624664812492_i64 as f64;
_16 = ['\u{595a1}','\u{9caa9}','\u{11472}','\u{126ee}','\u{c859a}','\u{502ed}'];
(*_2) = _6.3;
Goto(bb3)
}
bb3 = {
_4 = Move(_6.1);
_11 = [(-945847680_i32),1277243248_i32,1422323643_i32,61440690_i32];
(*_5) = _12;
(*_2) = _6.3;
_15 = (-167617029622527460356661877689913132954_i128);
_5 = core::ptr::addr_of!((*_5));
RET.0 = [24460_u16,7751_u16,8677_u16,9970_u16,36351_u16];
(*_5) = -_12;
(*_5) = 9223372036854775807_isize as f64;
Goto(bb4)
}
bb4 = {
_17.1.1 = 324083096654908623619538390623882904140_u128 - 243709460541812413212385314311505353677_u128;
(*_5) = -_12;
(*_5) = RET.2 as f64;
_4 = Move(RET.1);
match _15 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
172665337298411003106712729741855078502 => bb12,
_ => bb11
}
}
bb5 = {
_4 = Move(_6.1);
_11 = [(-945847680_i32),1277243248_i32,1422323643_i32,61440690_i32];
(*_5) = _12;
(*_2) = _6.3;
_15 = (-167617029622527460356661877689913132954_i128);
_5 = core::ptr::addr_of!((*_5));
RET.0 = [24460_u16,7751_u16,8677_u16,9970_u16,36351_u16];
(*_5) = -_12;
(*_5) = 9223372036854775807_isize as f64;
Goto(bb4)
}
bb6 = {
(*_2) = _6.3;
_6.0 = [37356_u16,37620_u16,41214_u16,42084_u16,46461_u16];
RET.1 = Move(_4);
(*_5) = 4_usize as f64;
_4 = Move(_6.1);
_13 = 2812986823739653616_u64 >> _3;
RET.3 = _6.3;
(*_5) = RET.2 as f64;
_6.1 = Move(_4);
_9 = core::ptr::addr_of!((*_5));
(*_9) = 126619817494553622972024215396105691768_i128 as f64;
(*_2) = [RET.2,_3,RET.2,RET.2];
_12 = (*_5);
(*_2) = [RET.2,RET.2,RET.2,_6.2];
RET.2 = _6.2 + _3;
(*_5) = (-7215593151832051266_i64) as f64;
_15 = (-105739430169054147055672348330232127699_i128) | 82778824704871744956375053252649039142_i128;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [_3,_6.2,_6.2,_6.2];
_12 = -_1;
(*_2) = _6.3;
(*_5) = 3327342624664812492_i64 as f64;
_16 = ['\u{595a1}','\u{9caa9}','\u{11472}','\u{126ee}','\u{c859a}','\u{502ed}'];
(*_2) = _6.3;
Goto(bb3)
}
bb7 = {
RET.3 = _6.3;
_3 = !_6.2;
_6.0 = [20301_u16,6778_u16,62093_u16,10142_u16,50956_u16];
_5 = core::ptr::addr_of!(_1);
_6.2 = (-81096771222829383020329407818747136588_i128) as i16;
_6.3 = [_3,_6.2,_3,_6.2];
_5 = core::ptr::addr_of!((*_5));
RET.2 = _3 & _3;
(*_5) = 39_isize as f64;
(*_5) = 3462157587815737871_usize as f64;
_9 = core::ptr::addr_of!(_1);
(*_9) = 204_u8 as f64;
(*_9) = 3598101099677466376_u64 as f64;
(*_5) = 96427014_u32 as f64;
(*_5) = 13309792493722570311_u64 as f64;
(*_5) = _3 as f64;
(*_5) = 1414647125_i32 as f64;
(*_5) = 99_i8 as f64;
(*_5) = 6_usize as f64;
_7 = !true;
RET.2 = _3 >> _6.2;
(*_5) = 100533984918966926560506812517768127551_u128 as f64;
RET.3 = [RET.2,_3,_6.2,RET.2];
_7 = true & false;
_2 = core::ptr::addr_of_mut!(RET.3);
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
(*_5) = 2398046682832066172_i64 as f64;
_20 = 862898285_i32;
(*_5) = 1022287939_u32 as f64;
_2 = core::ptr::addr_of_mut!((*_2));
_11 = [_20,_20,_20,_20];
_2 = core::ptr::addr_of_mut!(_10);
RET.3 = [RET.2,RET.2,_3,RET.2];
(*_2) = [_3,_3,_6.2,_6.2];
_27 = [25_u8,121_u8,109_u8,71_u8];
(*_5) = _12 + _12;
(*_2) = [RET.2,RET.2,_3,_3];
_25 = 31_isize >> _13;
(*_2) = [RET.2,_3,_3,_3];
_21 = -_25;
(*_5) = -_12;
_23 = _25;
(*_5) = _12 + _12;
(*_5) = _13 as f64;
_13 = 3629417204216802041_u64;
(*_2) = [_6.2,_3,RET.2,RET.2];
(*_2) = [RET.2,_3,_3,_6.2];
_5 = Move(_9);
Goto(bb13)
}
bb13 = {
(*_2) = [RET.2,_3,RET.2,_6.2];
(*_2) = [RET.2,RET.2,_6.2,RET.2];
_17.1.1 = !263426593955257046032028010758516421317_u128;
_11 = [_20,_20,_20,_20];
(*_2) = RET.3;
_8 = _21 - _21;
match _20 {
0 => bb12,
862898285 => bb14,
_ => bb6
}
}
bb14 = {
(*_2) = RET.3;
_17.0 = _20 | _20;
_2 = core::ptr::addr_of_mut!(_10);
_17.0 = _20 * _20;
_6.1 = Move(_4);
match _20 {
0 => bb7,
1 => bb6,
862898285 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
_29 = !_7;
_15 = 50898414648518667945792932393237358012_i128;
_6.1 = core::ptr::addr_of_mut!(_32);
_19 = 97_u8 << _25;
_5 = core::ptr::addr_of!(_12);
(*_5) = _17.1.1 as f64;
(*_2) = [RET.2,_3,RET.2,_6.2];
(*_5) = _1 - _1;
(*_2) = [_6.2,RET.2,_3,_6.2];
(*_5) = -_1;
(*_5) = _1 - _1;
Call((*_2) = core::intrinsics::transmute(_13), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
(*_2) = [RET.2,RET.2,RET.2,RET.2];
(*_5) = _1 * _1;
_17.1.2 = (-3004212171380993327_i64) as f32;
_7 = !_29;
(*_2) = [RET.2,RET.2,_3,_3];
_35.2 = _6.2 >> _8;
_4 = core::ptr::addr_of_mut!(_32);
_25 = _17.0 as isize;
_21 = !_8;
(*_4) = '\u{358d0}';
(*_5) = _13 as f64;
(*_5) = _1 * _1;
_35 = (_6.0, Move(_6.1), RET.2, _10);
RET.0 = [53769_u16,35925_u16,6977_u16,61947_u16,57449_u16];
_23 = _17.1.2 as isize;
(*_4) = '\u{5c299}';
_17.1.0 = _35.2 as usize;
(*_4) = '\u{46971}';
Goto(bb18)
}
bb18 = {
_17.1.0 = _13 as usize;
Goto(bb19)
}
bb19 = {
(*_5) = _1 - _1;
(*_2) = [_6.2,_35.2,_6.2,RET.2];
_15 = 65195372372373228741877737019421538984_i128;
(*_4) = '\u{d41d9}';
_23 = _21;
_35.0 = _6.0;
_27 = [_19,_19,_19,_19];
_22 = &_35;
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_9 = core::ptr::addr_of!((*_5));
(*_5) = 30431_u16 as f64;
_37 = _17.1.1;
RET = ((*_22).0, Move(_4), (*_22).2, (*_2));
(*_5) = _1 * _1;
_20 = _17.0 + _17.0;
_32 = '\u{6df2d}';
RET.3 = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_17.2 = 964473533_u32 as f64;
Call(_17.1.1 = core::intrinsics::transmute(_11), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
RET.2 = (*_22).2 | (*_22).2;
(*_5) = _17.2 + _17.2;
_35.3 = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_32 = '\u{df19e}';
RET = (_35.0, Move(_35.1), (*_22).2, (*_2));
_6.3 = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_44 = [_17.0,_20,_17.0,_20];
(*_5) = (-58_i8) as f64;
_25 = _8;
_40 = core::ptr::addr_of!(_13);
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
(*_5) = _1 + _17.2;
(*_5) = 3509634967_u32 as f64;
(*_5) = (-1937993317583253276_i64) as f64;
match (*_40) {
0 => bb5,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
3629417204216802041 => bb27,
_ => bb26
}
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
(*_2) = [RET.2,RET.2,RET.2,RET.2];
(*_5) = _1 * _1;
_17.1.2 = (-3004212171380993327_i64) as f32;
_7 = !_29;
(*_2) = [RET.2,RET.2,_3,_3];
_35.2 = _6.2 >> _8;
_4 = core::ptr::addr_of_mut!(_32);
_25 = _17.0 as isize;
_21 = !_8;
(*_4) = '\u{358d0}';
(*_5) = _13 as f64;
(*_5) = _1 * _1;
_35 = (_6.0, Move(_6.1), RET.2, _10);
RET.0 = [53769_u16,35925_u16,6977_u16,61947_u16,57449_u16];
_23 = _17.1.2 as isize;
(*_4) = '\u{5c299}';
_17.1.0 = _35.2 as usize;
(*_4) = '\u{46971}';
Goto(bb18)
}
bb24 = {
(*_2) = [RET.2,_3,RET.2,_6.2];
(*_2) = [RET.2,RET.2,_6.2,RET.2];
_17.1.1 = !263426593955257046032028010758516421317_u128;
_11 = [_20,_20,_20,_20];
(*_2) = RET.3;
_8 = _21 - _21;
match _20 {
0 => bb12,
862898285 => bb14,
_ => bb6
}
}
bb25 = {
Return()
}
bb26 = {
RET.3 = _6.3;
_3 = !_6.2;
_6.0 = [20301_u16,6778_u16,62093_u16,10142_u16,50956_u16];
_5 = core::ptr::addr_of!(_1);
_6.2 = (-81096771222829383020329407818747136588_i128) as i16;
_6.3 = [_3,_6.2,_3,_6.2];
_5 = core::ptr::addr_of!((*_5));
RET.2 = _3 & _3;
(*_5) = 39_isize as f64;
(*_5) = 3462157587815737871_usize as f64;
_9 = core::ptr::addr_of!(_1);
(*_9) = 204_u8 as f64;
(*_9) = 3598101099677466376_u64 as f64;
(*_5) = 96427014_u32 as f64;
(*_5) = 13309792493722570311_u64 as f64;
(*_5) = _3 as f64;
(*_5) = 1414647125_i32 as f64;
(*_5) = 99_i8 as f64;
(*_5) = 6_usize as f64;
_7 = !true;
RET.2 = _3 >> _6.2;
(*_5) = 100533984918966926560506812517768127551_u128 as f64;
RET.3 = [RET.2,_3,_6.2,RET.2];
_7 = true & false;
_2 = core::ptr::addr_of_mut!(RET.3);
Goto(bb2)
}
bb27 = {
(*_40) = 7026758851714719446_u64 & 12985450451205418642_u64;
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_13 = 16065802568434045072_u64;
(*_2) = _6.3;
match (*_40) {
0 => bb25,
1 => bb11,
2 => bb28,
3 => bb29,
16065802568434045072 => bb31,
_ => bb30
}
}
bb28 = {
RET.3 = _6.3;
_3 = !_6.2;
_6.0 = [20301_u16,6778_u16,62093_u16,10142_u16,50956_u16];
_5 = core::ptr::addr_of!(_1);
_6.2 = (-81096771222829383020329407818747136588_i128) as i16;
_6.3 = [_3,_6.2,_3,_6.2];
_5 = core::ptr::addr_of!((*_5));
RET.2 = _3 & _3;
(*_5) = 39_isize as f64;
(*_5) = 3462157587815737871_usize as f64;
_9 = core::ptr::addr_of!(_1);
(*_9) = 204_u8 as f64;
(*_9) = 3598101099677466376_u64 as f64;
(*_5) = 96427014_u32 as f64;
(*_5) = 13309792493722570311_u64 as f64;
(*_5) = _3 as f64;
(*_5) = 1414647125_i32 as f64;
(*_5) = 99_i8 as f64;
(*_5) = 6_usize as f64;
_7 = !true;
RET.2 = _3 >> _6.2;
(*_5) = 100533984918966926560506812517768127551_u128 as f64;
RET.3 = [RET.2,_3,_6.2,RET.2];
_7 = true & false;
_2 = core::ptr::addr_of_mut!(RET.3);
Goto(bb2)
}
bb29 = {
Return()
}
bb30 = {
(*_2) = [RET.2,RET.2,RET.2,RET.2];
(*_5) = _1 * _1;
_17.1.2 = (-3004212171380993327_i64) as f32;
_7 = !_29;
(*_2) = [RET.2,RET.2,_3,_3];
_35.2 = _6.2 >> _8;
_4 = core::ptr::addr_of_mut!(_32);
_25 = _17.0 as isize;
_21 = !_8;
(*_4) = '\u{358d0}';
(*_5) = _13 as f64;
(*_5) = _1 * _1;
_35 = (_6.0, Move(_6.1), RET.2, _10);
RET.0 = [53769_u16,35925_u16,6977_u16,61947_u16,57449_u16];
_23 = _17.1.2 as isize;
(*_4) = '\u{5c299}';
_17.1.0 = _35.2 as usize;
(*_4) = '\u{46971}';
Goto(bb18)
}
bb31 = {
(*_5) = _1 - _1;
_1 = -(*_5);
_8 = _17.1.1 as isize;
_17.1.2 = _15 as f32;
_16 = [_32,_32,_32,_32,_32,_32];
(*_5) = _1 + _1;
_41.0 = [_15,_15,_15,_15];
(*_40) = 13854220050556542766_u64 - 4428472034933925391_u64;
_9 = core::ptr::addr_of!(_12);
_31 = &_17;
(*_9) = _17.2 * (*_31).2;
_37 = (*_31).0 as u128;
(*_9) = (*_31).2 - (*_31).2;
_45 = &mut (*_31).1.0;
_47.0.1 = ((*_22).0, Move(RET.1), _6.2, (*_2));
_14 = Move(_31);
_6.0 = [4552_u16,2393_u16,27417_u16,61882_u16,5236_u16];
_47.0.1.2 = !(*_22).2;
(*_5) = -_1;
(*_2) = [(*_22).2,(*_22).2,(*_22).2,_47.0.1.2];
_6.0 = [13574_u16,35337_u16,39098_u16,35949_u16,49084_u16];
_53.1.2 = (*_22).2;
Goto(bb32)
}
bb32 = {
(*_40) = 16122936077644906245_u64 ^ 14413044364256980340_u64;
(*_2) = _47.0.1.3;
(*_5) = _1 * _1;
_53.1.2 = !(*_22).2;
_53.1.2 = (*_22).2 & (*_22).2;
_47.0.1.1 = core::ptr::addr_of_mut!(_32);
(*_2) = _47.0.1.3;
_28 = [_32,_32,_32,_32];
Goto(bb33)
}
bb33 = {
_33 = core::ptr::addr_of_mut!(_46);
_35.0 = _6.0;
(*_33) = 1808480759_u32 * 2210162239_u32;
_41.1 = &_28;
(*_40) = _37 as u64;
Goto(bb34)
}
bb34 = {
(*_33) = !4122315859_u32;
(*_40) = 13551647276701211145_u64 | 15596960677864469079_u64;
_3 = _25 as i16;
(*_2) = [(*_22).2,(*_22).2,_47.0.1.2,_3];
(*_2) = [(*_22).2,(*_22).2,(*_22).2,_3];
_53.1.1 = core::ptr::addr_of_mut!(_32);
(*_40) = 3189187036529692626_u64;
_29 = !_7;
_49 = [_20];
_35.1 = core::ptr::addr_of_mut!(_32);
_5 = core::ptr::addr_of!((*_5));
_35.3 = (*_2);
(*_40) = 4408067782562351583_u64 ^ 6979972527926911595_u64;
_48 = (-108_i8) as u8;
RET = (_35.0, Move(_47.0.1.1), _6.2, (*_2));
(*_2) = _35.3;
_59.0 = [_20];
(*_40) = !10457463194689343467_u64;
(*_2) = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_53.0 = [(*_22).2,(*_22).2,(*_22).2,(*_22).2];
_53.0 = [(*_22).2,_6.2,(*_22).2,_3];
_12 = (*_40) as f64;
_49 = _59.0;
match _15 {
0 => bb3,
1 => bb10,
65195372372373228741877737019421538984 => bb36,
_ => bb35
}
}
bb35 = {
Return()
}
bb36 = {
_6.1 = core::ptr::addr_of_mut!(_32);
_35.2 = _48 as i16;
(*_5) = (-87_i8) as f64;
(*_33) = 1247402863_u32 ^ 2604987944_u32;
_48 = !_19;
_47.1 = [_32,_32,_32,_32,_32,_32,_32,_32];
(*_40) = 51689_u16 as u64;
_6.3 = [_53.1.2,_53.1.2,_47.0.1.2,_53.1.2];
_47.0.0 = [_53.1.2,_3,_6.2,_3];
(*_40) = 11610626453989613359_u64 + 8123438744383918854_u64;
match _15 {
0 => bb31,
65195372372373228741877737019421538984 => bb38,
_ => bb37
}
}
bb37 = {
Return()
}
bb38 = {
RET.1 = Move(_53.1.1);
(*_5) = _1 - _1;
_54 = _25 >> (*_40);
(*_2) = [_3,_3,_53.1.2,_53.1.2];
_53.1.0 = [25105_u16,51879_u16,46254_u16,32108_u16,51936_u16];
(*_33) = 1776057537_u32;
_18 = core::ptr::addr_of_mut!((*_33));
(*_2) = [_47.0.1.2,_35.2,RET.2,_6.2];
_11 = _44;
_66 = _13 & (*_40);
_6.2 = !_53.1.2;
(*_2) = [RET.2,_3,_3,_47.0.1.2];
_35.2 = _3;
Goto(bb39)
}
bb39 = {
(*_33) = 19020283_u32 + 980421097_u32;
_65 = core::ptr::addr_of_mut!(RET.1);
(*_65) = core::ptr::addr_of_mut!(_43);
(*_65) = Move(_6.1);
(*_5) = (*_45) as f64;
_9 = core::ptr::addr_of!((*_5));
_58 = _20 as i64;
(*_33) = _58 as u32;
match _15 {
0 => bb16,
1 => bb26,
2 => bb25,
3 => bb4,
4 => bb21,
5 => bb40,
6 => bb41,
65195372372373228741877737019421538984 => bb43,
_ => bb42
}
}
bb40 = {
_4 = Move(_6.1);
_11 = [(-945847680_i32),1277243248_i32,1422323643_i32,61440690_i32];
(*_5) = _12;
(*_2) = _6.3;
_15 = (-167617029622527460356661877689913132954_i128);
_5 = core::ptr::addr_of!((*_5));
RET.0 = [24460_u16,7751_u16,8677_u16,9970_u16,36351_u16];
(*_5) = -_12;
(*_5) = 9223372036854775807_isize as f64;
Goto(bb4)
}
bb41 = {
RET.3 = _6.3;
_3 = !_6.2;
_6.0 = [20301_u16,6778_u16,62093_u16,10142_u16,50956_u16];
_5 = core::ptr::addr_of!(_1);
_6.2 = (-81096771222829383020329407818747136588_i128) as i16;
_6.3 = [_3,_6.2,_3,_6.2];
_5 = core::ptr::addr_of!((*_5));
RET.2 = _3 & _3;
(*_5) = 39_isize as f64;
(*_5) = 3462157587815737871_usize as f64;
_9 = core::ptr::addr_of!(_1);
(*_9) = 204_u8 as f64;
(*_9) = 3598101099677466376_u64 as f64;
(*_5) = 96427014_u32 as f64;
(*_5) = 13309792493722570311_u64 as f64;
(*_5) = _3 as f64;
(*_5) = 1414647125_i32 as f64;
(*_5) = 99_i8 as f64;
(*_5) = 6_usize as f64;
_7 = !true;
RET.2 = _3 >> _6.2;
(*_5) = 100533984918966926560506812517768127551_u128 as f64;
RET.3 = [RET.2,_3,_6.2,RET.2];
_7 = true & false;
_2 = core::ptr::addr_of_mut!(RET.3);
Goto(bb2)
}
bb42 = {
(*_2) = RET.3;
_17.0 = _20 | _20;
_2 = core::ptr::addr_of_mut!(_10);
_17.0 = _20 * _20;
_6.1 = Move(_4);
match _20 {
0 => bb7,
1 => bb6,
862898285 => bb16,
_ => bb15
}
}
bb43 = {
_47.0.2 = Adt25::Variant0 { fld0: (*_5),fld1: _46 };
_66 = (*_40) | _13;
(*_40) = _32 as u64;
_6.0 = [33683_u16,44627_u16,52897_u16,14354_u16,22447_u16];
(*_9) = _1;
(*_5) = -_1;
(*_65) = core::ptr::addr_of_mut!(_32);
(*_65) = Move(_35.1);
_10 = _53.0;
_35.3 = (*_2);
_59.1 = &mut (*_45);
_60 = core::ptr::addr_of_mut!((*_33));
_53.1.3 = _47.0.0;
(*_33) = !Field::<u32>(Variant(_47.0.2, 0), 1);
_53.0 = [_3,_6.2,_35.2,_35.2];
_24 = &_41.0;
(*_65) = core::ptr::addr_of_mut!(_43);
(*_33) = !Field::<u32>(Variant(_47.0.2, 0), 1);
(*_40) = _20 as u64;
_70 = (*_40) < (*_40);
match _15 {
0 => bb41,
1 => bb30,
2 => bb3,
3 => bb7,
4 => bb36,
65195372372373228741877737019421538984 => bb45,
_ => bb44
}
}
bb44 = {
_17.1.0 = _13 as usize;
Goto(bb19)
}
bb45 = {
_37 = 133265871341942392570073359956996247488_u128;
(*_40) = (*_5) as u64;
_32 = '\u{275e}';
_70 = (*_40) >= (*_40);
(*_65) = core::ptr::addr_of_mut!(_43);
(*_33) = _54 as u32;
(*_5) = _58 as f64;
(*_2) = _47.0.1.3;
(*_40) = !_66;
_25 = _8;
_75 = _58 * _58;
_47.0.1 = Move(RET);
(*_2) = [_3,_53.1.2,_35.2,_47.0.1.2];
_73 = _16;
_74 = _70;
Call(_47.0.1.0 = fn18(_10, Move(_2), Move(_41), Move(_47.0.2), Move(_47.0.1.1), Move(_5), Move(_9), (*_5), (*_33), (*_5)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_2 = core::ptr::addr_of_mut!(_6.3);
(*_40) = _66 ^ _66;
Goto(bb47)
}
bb47 = {
_77 = 70_i8 as isize;
_67.3 = [_35.2,_6.2,_53.1.2,_3];
_57 = _37 as i64;
(*_65) = core::ptr::addr_of_mut!(_43);
_51 = _20 as u64;
Goto(bb48)
}
bb48 = {
_37 = (*_33) as u128;
(*_65) = core::ptr::addr_of_mut!(_43);
_10 = _6.3;
_37 = 83275505314322475234406918485654910367_u128 & 285681303511900319507083251484989217069_u128;
(*_33) = 1548388691_u32 * 3155892359_u32;
(*_2) = [_53.1.2,_3,_3,_53.1.2];
Goto(bb49)
}
bb49 = {
_69 = core::ptr::addr_of_mut!(_1);
(*_33) = 2125293152_u32 | 760386386_u32;
(*_33) = 1935019654_u32 ^ 3656214834_u32;
_47.2 = Move(_59.1);
(*_69) = -_12;
(*_2) = [_3,_3,_6.2,_35.2];
_67.2 = _15 as i16;
(*_40) = _66;
(*_65) = core::ptr::addr_of_mut!(_43);
(*_69) = _12 - _12;
_4 = core::ptr::addr_of_mut!(_32);
_18 = core::ptr::addr_of_mut!((*_33));
_53.1 = (_6.0, Move(_4), _35.2, (*_2));
(*_33) = 2966891908_u32;
_6 = Move(_53.1);
Goto(bb50)
}
bb50 = {
(*_69) = _20 as f64;
(*_40) = _51 ^ _66;
(*_65) = core::ptr::addr_of_mut!(_43);
Goto(bb51)
}
bb51 = {
_64 = &_19;
(*_40) = _51;
(*_33) = 802037564_u32 * 663757191_u32;
_64 = &_48;
(*_65) = core::ptr::addr_of_mut!(_32);
_59.1 = Move(_45);
_63 = core::ptr::addr_of!(_68);
_97 = core::ptr::addr_of_mut!(_43);
_76 = &_6;
_50 = _15;
_43 = _32;
_53.1 = ((*_76).0, Move((*_65)), (*_76).2, (*_76).3);
_93 = [(*_64),(*_64),(*_64),(*_64)];
_35.3 = (*_2);
(*_65) = Move(_53.1.1);
_5 = core::ptr::addr_of!((*_69));
_35.2 = (*_76).2;
RET.3 = [(*_76).2,(*_76).2,(*_76).2,(*_76).2];
(*_33) = 914853989_u32 ^ 381045768_u32;
match _15 {
0 => bb52,
1 => bb53,
2 => bb54,
65195372372373228741877737019421538984 => bb56,
_ => bb55
}
}
bb52 = {
(*_69) = _20 as f64;
(*_40) = _51 ^ _66;
(*_65) = core::ptr::addr_of_mut!(_43);
Goto(bb51)
}
bb53 = {
(*_5) = _1 - _1;
_1 = -(*_5);
_8 = _17.1.1 as isize;
_17.1.2 = _15 as f32;
_16 = [_32,_32,_32,_32,_32,_32];
(*_5) = _1 + _1;
_41.0 = [_15,_15,_15,_15];
(*_40) = 13854220050556542766_u64 - 4428472034933925391_u64;
_9 = core::ptr::addr_of!(_12);
_31 = &_17;
(*_9) = _17.2 * (*_31).2;
_37 = (*_31).0 as u128;
(*_9) = (*_31).2 - (*_31).2;
_45 = &mut (*_31).1.0;
_47.0.1 = ((*_22).0, Move(RET.1), _6.2, (*_2));
_14 = Move(_31);
_6.0 = [4552_u16,2393_u16,27417_u16,61882_u16,5236_u16];
_47.0.1.2 = !(*_22).2;
(*_5) = -_1;
(*_2) = [(*_22).2,(*_22).2,(*_22).2,_47.0.1.2];
_6.0 = [13574_u16,35337_u16,39098_u16,35949_u16,49084_u16];
_53.1.2 = (*_22).2;
Goto(bb32)
}
bb54 = {
RET.1 = Move(_53.1.1);
(*_5) = _1 - _1;
_54 = _25 >> (*_40);
(*_2) = [_3,_3,_53.1.2,_53.1.2];
_53.1.0 = [25105_u16,51879_u16,46254_u16,32108_u16,51936_u16];
(*_33) = 1776057537_u32;
_18 = core::ptr::addr_of_mut!((*_33));
(*_2) = [_47.0.1.2,_35.2,RET.2,_6.2];
_11 = _44;
_66 = _13 & (*_40);
_6.2 = !_53.1.2;
(*_2) = [RET.2,_3,_3,_47.0.1.2];
_35.2 = _3;
Goto(bb39)
}
bb55 = {
(*_5) = 2398046682832066172_i64 as f64;
_20 = 862898285_i32;
(*_5) = 1022287939_u32 as f64;
_2 = core::ptr::addr_of_mut!((*_2));
_11 = [_20,_20,_20,_20];
_2 = core::ptr::addr_of_mut!(_10);
RET.3 = [RET.2,RET.2,_3,RET.2];
(*_2) = [_3,_3,_6.2,_6.2];
_27 = [25_u8,121_u8,109_u8,71_u8];
(*_5) = _12 + _12;
(*_2) = [RET.2,RET.2,_3,_3];
_25 = 31_isize >> _13;
(*_2) = [RET.2,_3,_3,_3];
_21 = -_25;
(*_5) = -_12;
_23 = _25;
(*_5) = _12 + _12;
(*_5) = _13 as f64;
_13 = 3629417204216802041_u64;
(*_2) = [_6.2,_3,RET.2,RET.2];
(*_2) = [RET.2,_3,_3,_6.2];
_5 = Move(_9);
Goto(bb13)
}
bb56 = {
(*_5) = -_12;
_53.2 = Adt25::Variant0 { fld0: (*_69),fld1: _46 };
_4 = core::ptr::addr_of_mut!(_32);
_27 = _93;
_10 = [_35.2,(*_76).2,(*_76).2,_6.2];
RET.0 = [27169_u16,25915_u16,7311_u16,30066_u16,53557_u16];
(*_4) = _43;
(*_40) = _51 - _66;
(*_63) = _54 - _23;
_67.3 = (*_76).3;
RET.2 = !(*_76).2;
Goto(bb57)
}
bb57 = {
Call(_102 = dump_var(Move(_10), Move(_48), Move(_43), Move(_49)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_102 = dump_var(Move(_25), Move(_68), Move(_16), Move(_77)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_102 = dump_var(Move(_3), Move(_46), Move(_74), Move(_54)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_102 = dump_var(Move(_50), Move(_11), Move(_21), Move(_58)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_102 = dump_var(Move(_23), Move(_29), _103, _103), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [i16; 4],mut _2: *mut [i16; 4],mut _3: ([i128; 4], &'static [char; 4]),mut _4: Adt25,mut _5: *mut char,mut _6: *const f64,mut _7: *const f64,mut _8: f64,mut _9: u32,mut _10: f64) -> [u16; 5] {
mir! {
type RET = [u16; 5];
let _11: (isize, u8, i128, i64);
let _12: *mut char;
let _13: u8;
let _14: Adt25;
let _15: *const [char; 4];
let _16: *mut &'static Adt25;
let _17: &'static Adt25;
let _18: Adt84;
let _19: isize;
let _20: [isize; 8];
let _21: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _22: &'static u8;
let _23: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25);
let _24: &'static [char; 4];
let _25: &'static Adt25;
let _26: &'static f64;
let _27: *const [char; 4];
let _28: f64;
let _29: [i32; 1];
let _30: [u64; 1];
let _31: bool;
let _32: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _33: u16;
let _34: char;
let _35: *const u64;
let _36: &'static ([u16; 5], *mut char, i16, [i16; 4]);
let _37: (i32, (usize, u128, f32), f64);
let _38: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _39: i64;
let _40: isize;
let _41: bool;
let _42: *const [char; 4];
let _43: char;
let _44: i16;
let _45: char;
let _46: &'static mut u16;
let _47: f64;
let _48: f32;
let _49: u64;
let _50: isize;
let _51: i16;
let _52: *const i8;
let _53: u128;
let _54: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _55: [u128; 5];
let _56: f32;
let _57: [i16; 4];
let _58: *const i8;
let _59: *const f64;
let _60: bool;
let _61: (f64, &'static &'static (i32, (usize, u128, f32), f64), ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), *const u64);
let _62: isize;
let _63: &'static mut usize;
let _64: char;
let _65: char;
let _66: [i64; 7];
let _67: u16;
let _68: isize;
let _69: &'static u8;
let _70: usize;
let _71: i32;
let _72: &'static &'static (i32, (usize, u128, f32), f64);
let _73: isize;
let _74: ();
let _75: ();
{
place!(Field::<f64>(Variant(_4, 0), 0)) = _8 * _10;
_11.1 = 12343775452182599803_u64 as u8;
Goto(bb1)
}
bb1 = {
_11.0 = Field::<u32>(Variant(_4, 0), 1) as isize;
RET = [21702_u16,44107_u16,28351_u16,48652_u16,34985_u16];
_11 = ((-101_isize), 28_u8, (-42552442264489933849711237144136171760_i128), 7429348266733782782_i64);
_11.1 = 26_u8 - 175_u8;
_4 = Adt25::Variant0 { fld0: _10,fld1: _9 };
_11.1 = false as u8;
_6 = core::ptr::addr_of!(_10);
_11 = ((-58_isize), 66_u8, 102333054316329591887938917824296337114_i128, (-620687534867055403_i64));
_8 = (*_6);
Call(place!(Field::<f64>(Variant(_4, 0), 0)) = core::intrinsics::transmute(_11.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb3 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb4 = {
(*_2) = [(-25121_i16),(-21593_i16),(-29856_i16),7445_i16];
(*_6) = _10 * Field::<f64>(Variant(_4, 0), 0);
_11.1 = _13;
(*_2) = [(-2737_i16),840_i16,11649_i16,20829_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10 - Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-5318_i16),29780_i16,(-19446_i16),(-18223_i16)];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_13 = !_11.1;
(*_6) = (-11_i8) as f64;
match _11.3 {
0 => bb5,
340282366920938463458279246748991405776 => bb7,
_ => bb6
}
}
bb5 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb6 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb7 = {
(*_6) = 1_usize as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_19 = _11.0 >> Field::<u32>(Variant(_4, 0), 1);
(*_2) = [25308_i16,(-10767_i16),32676_i16,20510_i16];
(*_2) = [27378_i16,248_i16,11317_i16,21814_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) * Field::<f64>(Variant(_4, 0), 0);
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = _10 * Field::<f64>(Variant(_4, 0), 0);
(*_2) = [25954_i16,28356_i16,(-10689_i16),15444_i16];
_2 = core::ptr::addr_of_mut!((*_2));
_11.3 = false as i64;
(*_2) = [(-21337_i16),28634_i16,(-9679_i16),21769_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) - Field::<f64>(Variant(_4, 0), 0);
_11.1 = _13 - _13;
(*_2) = [(-9557_i16),(-28253_i16),4078_i16,4512_i16];
(*_2) = [(-17203_i16),19620_i16,29268_i16,(-6170_i16)];
_11 = (_19, _13, (-10837005960097383111400161904926249068_i128), (-4485196597188802197_i64));
(*_2) = [(-17901_i16),9692_i16,11341_i16,(-12329_i16)];
_1 = [13603_i16,18311_i16,13975_i16,26787_i16];
_11.0 = _19 - _19;
(*_2) = [23645_i16,(-13428_i16),22653_i16,(-28458_i16)];
(*_2) = [19348_i16,(-24299_i16),(-16151_i16),(-18918_i16)];
_4 = Adt25::Variant0 { fld0: (*_6),fld1: _9 };
(*_2) = [5169_i16,17703_i16,(-6820_i16),29419_i16];
_1 = [2478_i16,(-26743_i16),16151_i16,(-11078_i16)];
_17 = &_4;
(*_6) = Field::<f64>(Variant((*_17), 0), 0) * Field::<f64>(Variant(_4, 0), 0);
match _11.3 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463458889410834579409259 => bb9,
_ => bb8
}
}
bb8 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb9 = {
_11.3 = 8337960739654736130_i64 | 4688588442697612528_i64;
match _11.2 {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb11,
329445360960841080351974445526841962388 => bb13,
_ => bb12
}
}
bb10 = {
_11.0 = Field::<u32>(Variant(_4, 0), 1) as isize;
RET = [21702_u16,44107_u16,28351_u16,48652_u16,34985_u16];
_11 = ((-101_isize), 28_u8, (-42552442264489933849711237144136171760_i128), 7429348266733782782_i64);
_11.1 = 26_u8 - 175_u8;
_4 = Adt25::Variant0 { fld0: _10,fld1: _9 };
_11.1 = false as u8;
_6 = core::ptr::addr_of!(_10);
_11 = ((-58_isize), 66_u8, 102333054316329591887938917824296337114_i128, (-620687534867055403_i64));
_8 = (*_6);
Call(place!(Field::<f64>(Variant(_4, 0), 0)) = core::intrinsics::transmute(_11.3), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
(*_6) = 1_usize as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_19 = _11.0 >> Field::<u32>(Variant(_4, 0), 1);
(*_2) = [25308_i16,(-10767_i16),32676_i16,20510_i16];
(*_2) = [27378_i16,248_i16,11317_i16,21814_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) * Field::<f64>(Variant(_4, 0), 0);
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = _10 * Field::<f64>(Variant(_4, 0), 0);
(*_2) = [25954_i16,28356_i16,(-10689_i16),15444_i16];
_2 = core::ptr::addr_of_mut!((*_2));
_11.3 = false as i64;
(*_2) = [(-21337_i16),28634_i16,(-9679_i16),21769_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) - Field::<f64>(Variant(_4, 0), 0);
_11.1 = _13 - _13;
(*_2) = [(-9557_i16),(-28253_i16),4078_i16,4512_i16];
(*_2) = [(-17203_i16),19620_i16,29268_i16,(-6170_i16)];
_11 = (_19, _13, (-10837005960097383111400161904926249068_i128), (-4485196597188802197_i64));
(*_2) = [(-17901_i16),9692_i16,11341_i16,(-12329_i16)];
_1 = [13603_i16,18311_i16,13975_i16,26787_i16];
_11.0 = _19 - _19;
(*_2) = [23645_i16,(-13428_i16),22653_i16,(-28458_i16)];
(*_2) = [19348_i16,(-24299_i16),(-16151_i16),(-18918_i16)];
_4 = Adt25::Variant0 { fld0: (*_6),fld1: _9 };
(*_2) = [5169_i16,17703_i16,(-6820_i16),29419_i16];
_1 = [2478_i16,(-26743_i16),16151_i16,(-11078_i16)];
_17 = &_4;
(*_6) = Field::<f64>(Variant((*_17), 0), 0) * Field::<f64>(Variant(_4, 0), 0);
match _11.3 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463458889410834579409259 => bb9,
_ => bb8
}
}
bb12 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb13 = {
_19 = _11.0 ^ _11.0;
(*_2) = [(-29931_i16),28821_i16,27599_i16,23121_i16];
Goto(bb14)
}
bb14 = {
(*_2) = [(-1756_i16),29855_i16,14568_i16,(-10107_i16)];
(*_2) = [25131_i16,23814_i16,(-18140_i16),18170_i16];
(*_2) = [(-17863_i16),(-8807_i16),11726_i16,(-1266_i16)];
_11 = (_19, _13, (-28107532715291865787694950801677802805_i128), 426288484224461826_i64);
(*_2) = [31767_i16,28635_i16,22511_i16,22151_i16];
_25 = &_4;
(*_6) = Field::<f64>(Variant((*_17), 0), 0);
(*_6) = Field::<f64>(Variant((*_25), 0), 0) + Field::<f64>(Variant((*_25), 0), 0);
match _11.2 {
0 => bb15,
1 => bb16,
312174834205646597675679656630090408651 => bb18,
_ => bb17
}
}
bb15 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb16 = {
(*_6) = 1_usize as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_19 = _11.0 >> Field::<u32>(Variant(_4, 0), 1);
(*_2) = [25308_i16,(-10767_i16),32676_i16,20510_i16];
(*_2) = [27378_i16,248_i16,11317_i16,21814_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) * Field::<f64>(Variant(_4, 0), 0);
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = _10 * Field::<f64>(Variant(_4, 0), 0);
(*_2) = [25954_i16,28356_i16,(-10689_i16),15444_i16];
_2 = core::ptr::addr_of_mut!((*_2));
_11.3 = false as i64;
(*_2) = [(-21337_i16),28634_i16,(-9679_i16),21769_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) - Field::<f64>(Variant(_4, 0), 0);
_11.1 = _13 - _13;
(*_2) = [(-9557_i16),(-28253_i16),4078_i16,4512_i16];
(*_2) = [(-17203_i16),19620_i16,29268_i16,(-6170_i16)];
_11 = (_19, _13, (-10837005960097383111400161904926249068_i128), (-4485196597188802197_i64));
(*_2) = [(-17901_i16),9692_i16,11341_i16,(-12329_i16)];
_1 = [13603_i16,18311_i16,13975_i16,26787_i16];
_11.0 = _19 - _19;
(*_2) = [23645_i16,(-13428_i16),22653_i16,(-28458_i16)];
(*_2) = [19348_i16,(-24299_i16),(-16151_i16),(-18918_i16)];
_4 = Adt25::Variant0 { fld0: (*_6),fld1: _9 };
(*_2) = [5169_i16,17703_i16,(-6820_i16),29419_i16];
_1 = [2478_i16,(-26743_i16),16151_i16,(-11078_i16)];
_17 = &_4;
(*_6) = Field::<f64>(Variant((*_17), 0), 0) * Field::<f64>(Variant(_4, 0), 0);
match _11.3 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463458889410834579409259 => bb9,
_ => bb8
}
}
bb17 = {
_11.0 = Field::<u32>(Variant(_4, 0), 1) as isize;
RET = [21702_u16,44107_u16,28351_u16,48652_u16,34985_u16];
_11 = ((-101_isize), 28_u8, (-42552442264489933849711237144136171760_i128), 7429348266733782782_i64);
_11.1 = 26_u8 - 175_u8;
_4 = Adt25::Variant0 { fld0: _10,fld1: _9 };
_11.1 = false as u8;
_6 = core::ptr::addr_of!(_10);
_11 = ((-58_isize), 66_u8, 102333054316329591887938917824296337114_i128, (-620687534867055403_i64));
_8 = (*_6);
Call(place!(Field::<f64>(Variant(_4, 0), 0)) = core::intrinsics::transmute(_11.3), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
(*_2) = [(-32533_i16),(-7910_i16),26260_i16,(-192_i16)];
match _11.2 {
0 => bb3,
1 => bb19,
312174834205646597675679656630090408651 => bb21,
_ => bb20
}
}
bb19 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb20 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb21 = {
_2 = core::ptr::addr_of_mut!((*_2));
_7 = core::ptr::addr_of!((*_6));
_23.2 = Move(_4);
(*_7) = Field::<f64>(Variant(_23.2, 0), 0);
(*_7) = _9 as f64;
_30 = [15706745467769636346_u64];
(*_6) = Field::<f64>(Variant(_23.2, 0), 0) * Field::<f64>(Variant(_23.2, 0), 0);
(*_6) = 17215_i16 as f64;
_25 = &_23.2;
_11 = (_19, _13, 167009058038605842100014702315195614727_i128, 7328429934789586567_i64);
match _11.3 {
0 => bb8,
1 => bb12,
2 => bb7,
3 => bb22,
4 => bb23,
5 => bb24,
6 => bb25,
7328429934789586567 => bb27,
_ => bb26
}
}
bb22 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb23 = {
_11.3 = 8337960739654736130_i64 | 4688588442697612528_i64;
match _11.2 {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb11,
329445360960841080351974445526841962388 => bb13,
_ => bb12
}
}
bb24 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb25 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb26 = {
(*_6) = 1_usize as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_19 = _11.0 >> Field::<u32>(Variant(_4, 0), 1);
(*_2) = [25308_i16,(-10767_i16),32676_i16,20510_i16];
(*_2) = [27378_i16,248_i16,11317_i16,21814_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) * Field::<f64>(Variant(_4, 0), 0);
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = _10 * Field::<f64>(Variant(_4, 0), 0);
(*_2) = [25954_i16,28356_i16,(-10689_i16),15444_i16];
_2 = core::ptr::addr_of_mut!((*_2));
_11.3 = false as i64;
(*_2) = [(-21337_i16),28634_i16,(-9679_i16),21769_i16];
(*_6) = Field::<f64>(Variant(_4, 0), 0) - Field::<f64>(Variant(_4, 0), 0);
_11.1 = _13 - _13;
(*_2) = [(-9557_i16),(-28253_i16),4078_i16,4512_i16];
(*_2) = [(-17203_i16),19620_i16,29268_i16,(-6170_i16)];
_11 = (_19, _13, (-10837005960097383111400161904926249068_i128), (-4485196597188802197_i64));
(*_2) = [(-17901_i16),9692_i16,11341_i16,(-12329_i16)];
_1 = [13603_i16,18311_i16,13975_i16,26787_i16];
_11.0 = _19 - _19;
(*_2) = [23645_i16,(-13428_i16),22653_i16,(-28458_i16)];
(*_2) = [19348_i16,(-24299_i16),(-16151_i16),(-18918_i16)];
_4 = Adt25::Variant0 { fld0: (*_6),fld1: _9 };
(*_2) = [5169_i16,17703_i16,(-6820_i16),29419_i16];
_1 = [2478_i16,(-26743_i16),16151_i16,(-11078_i16)];
_17 = &_4;
(*_6) = Field::<f64>(Variant((*_17), 0), 0) * Field::<f64>(Variant(_4, 0), 0);
match _11.3 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463458889410834579409259 => bb9,
_ => bb8
}
}
bb27 = {
_26 = &_8;
_11.0 = _19 & _19;
_2 = core::ptr::addr_of_mut!((*_2));
_4 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_25), 0), 0),fld1: Field::<u32>(Variant((*_25), 0), 1) };
_16 = core::ptr::addr_of_mut!(_17);
_6 = core::ptr::addr_of!((*_26));
(*_16) = &(*_25);
match _11.2 {
0 => bb25,
1 => bb11,
2 => bb26,
3 => bb19,
4 => bb5,
5 => bb18,
6 => bb13,
167009058038605842100014702315195614727 => bb28,
_ => bb17
}
}
bb28 = {
_22 = &_13;
(*_16) = Move(_25);
(*_2) = [(-10106_i16),3965_i16,(-13424_i16),(-28371_i16)];
(*_2) = [22984_i16,13227_i16,13488_i16,6902_i16];
(*_2) = [(-28624_i16),(-15455_i16),(-31430_i16),(-30774_i16)];
(*_16) = &_23.2;
(*_2) = [12705_i16,(-28998_i16),28826_i16,(-24474_i16)];
(*_2) = [13906_i16,1255_i16,(-9594_i16),(-10494_i16)];
_33 = (-52_i8) as u16;
_23.1.3 = [(-30656_i16),27826_i16,(-23589_i16),(-15327_i16)];
(*_16) = &_4;
_20 = [_11.0,_11.0,_19,_11.0,_11.0,_11.0,_11.0,_11.0];
_2 = core::ptr::addr_of_mut!(_23.0);
(*_2) = [7233_i16,31446_i16,(-17634_i16),(-26230_i16)];
_37.1.1 = '\u{93f02}' as u128;
(*_2) = _1;
_37.1.2 = 1720529431_i32 as f32;
Goto(bb29)
}
bb29 = {
(*_16) = &_23.2;
_16 = core::ptr::addr_of_mut!((*_16));
(*_2) = [27010_i16,13006_i16,26610_i16,(-19574_i16)];
match _11.2 {
0 => bb26,
1 => bb14,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
6 => bb34,
167009058038605842100014702315195614727 => bb36,
_ => bb35
}
}
bb30 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb31 = {
_26 = &_8;
_11.0 = _19 & _19;
_2 = core::ptr::addr_of_mut!((*_2));
_4 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_25), 0), 0),fld1: Field::<u32>(Variant((*_25), 0), 1) };
_16 = core::ptr::addr_of_mut!(_17);
_6 = core::ptr::addr_of!((*_26));
(*_16) = &(*_25);
match _11.2 {
0 => bb25,
1 => bb11,
2 => bb26,
3 => bb19,
4 => bb5,
5 => bb18,
6 => bb13,
167009058038605842100014702315195614727 => bb28,
_ => bb17
}
}
bb32 = {
_19 = _11.0 ^ _11.0;
(*_2) = [(-29931_i16),28821_i16,27599_i16,23121_i16];
Goto(bb14)
}
bb33 = {
_11.0 = Field::<u32>(Variant(_4, 0), 1) as isize;
RET = [21702_u16,44107_u16,28351_u16,48652_u16,34985_u16];
_11 = ((-101_isize), 28_u8, (-42552442264489933849711237144136171760_i128), 7429348266733782782_i64);
_11.1 = 26_u8 - 175_u8;
_4 = Adt25::Variant0 { fld0: _10,fld1: _9 };
_11.1 = false as u8;
_6 = core::ptr::addr_of!(_10);
_11 = ((-58_isize), 66_u8, 102333054316329591887938917824296337114_i128, (-620687534867055403_i64));
_8 = (*_6);
Call(place!(Field::<f64>(Variant(_4, 0), 0)) = core::intrinsics::transmute(_11.3), ReturnTo(bb2), UnwindUnreachable())
}
bb34 = {
_11.3 = 8337960739654736130_i64 | 4688588442697612528_i64;
match _11.2 {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb11,
329445360960841080351974445526841962388 => bb13,
_ => bb12
}
}
bb35 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb36 = {
(*_2) = [14268_i16,(-18601_i16),(-30187_i16),9777_i16];
_43 = '\u{8c7a7}';
_30 = [12449948002279478609_u64];
_23.1.3 = [(-14299_i16),(-39_i16),27301_i16,(-4813_i16)];
_28 = Field::<f64>(Variant((*_17), 0), 0) - Field::<f64>(Variant((*_17), 0), 0);
_33 = !19955_u16;
_26 = &_10;
_23.1.0 = RET;
(*_2) = [1059_i16,(-12282_i16),30966_i16,16489_i16];
(*_16) = &_4;
(*_2) = [3494_i16,8907_i16,(-8058_i16),18781_i16];
(*_2) = [(-9500_i16),(-4763_i16),(-15087_i16),21065_i16];
(*_16) = &_23.2;
_37.2 = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant((*_17), 0), 0);
_13 = _11.1 + _11.1;
(*_2) = [25770_i16,28827_i16,9916_i16,(-14010_i16)];
(*_2) = [(-7837_i16),23466_i16,17202_i16,(-23436_i16)];
(*_16) = &_4;
_31 = false | true;
Goto(bb37)
}
bb37 = {
(*_16) = &_23.2;
_1 = [81_i16,(-23958_i16),(-11213_i16),9790_i16];
_29 = [(-294961811_i32)];
_12 = core::ptr::addr_of_mut!(_34);
(*_16) = &_4;
(*_2) = [(-16254_i16),13388_i16,18057_i16,(-20552_i16)];
_37.0 = (-272361417_i32);
place!(Field::<u32>(Variant(_23.2, 0), 1)) = Field::<u32>(Variant((*_17), 0), 1);
(*_2) = [19262_i16,(-32718_i16),(-27326_i16),5909_i16];
(*_12) = _43;
_47 = Field::<f64>(Variant((*_17), 0), 0) * Field::<f64>(Variant((*_17), 0), 0);
(*_12) = _43;
(*_16) = &_23.2;
(*_2) = _1;
(*_2) = [(-7333_i16),(-4946_i16),27939_i16,(-12291_i16)];
_48 = -_37.1.2;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [(-94_i16),29992_i16,14398_i16,17841_i16];
Goto(bb38)
}
bb38 = {
(*_12) = _43;
_23.2 = Move(_4);
(*_2) = _1;
(*_12) = _43;
(*_2) = [23115_i16,22169_i16,6681_i16,12635_i16];
_16 = core::ptr::addr_of_mut!((*_16));
(*_12) = _43;
_35 = core::ptr::addr_of!(_49);
(*_35) = _11.0 as u64;
_23.1.2 = 23231_i16 + (-16226_i16);
_46 = &mut _33;
(*_12) = _43;
Goto(bb39)
}
bb39 = {
(*_46) = 31095_u16 << (*_35);
(*_35) = 15519129895021927470_u64 << (*_46);
_10 = -_28;
(*_35) = 7248881869010062225_u64;
Goto(bb40)
}
bb40 = {
(*_2) = [_23.1.2,_23.1.2,_23.1.2,_23.1.2];
_9 = !Field::<u32>(Variant(_23.2, 0), 1);
match (*_35) {
7248881869010062225 => bb42,
_ => bb41
}
}
bb41 = {
_19 = _11.0 ^ _11.0;
(*_2) = [(-29931_i16),28821_i16,27599_i16,23121_i16];
Goto(bb14)
}
bb42 = {
_43 = (*_12);
place!(Field::<u32>(Variant(_23.2, 0), 1)) = _9;
_11.1 = _13 >> (*_46);
_13 = _11.1 ^ _11.1;
_12 = core::ptr::addr_of_mut!((*_12));
(*_2) = [_23.1.2,_23.1.2,_23.1.2,_23.1.2];
(*_12) = _43;
_35 = core::ptr::addr_of!((*_35));
(*_35) = 12682407867603757390_u64 >> _11.3;
_37.2 = Field::<f64>(Variant(_23.2, 0), 0);
(*_2) = [_23.1.2,_23.1.2,_23.1.2,_23.1.2];
(*_12) = _43;
(*_46) = _47 as u16;
(*_2) = _1;
_40 = (*_35) as isize;
(*_2) = [_23.1.2,_23.1.2,_23.1.2,_23.1.2];
_11.2 = !(-9256556168534832850683192626914345989_i128);
(*_12) = _43;
Call((*_35) = core::intrinsics::bswap(4228211593920586581_u64), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_13 = _11.1 - _11.1;
(*_46) = !7268_u16;
_53 = !_37.1.1;
_55 = [_53,_37.1.1,_53,_53,_53];
(*_35) = 15849877726787059483_u64;
_23.1 = (RET, Move(_12), (-17142_i16), (*_2));
_11.3 = _37.2 as i64;
_2 = core::ptr::addr_of_mut!((*_2));
_36 = &_23.1;
(*_46) = 7861_u16;
_56 = -_37.1.2;
_12 = core::ptr::addr_of_mut!(_43);
match (*_46) {
0 => bb15,
1 => bb35,
2 => bb34,
3 => bb44,
4 => bb45,
7861 => bb47,
_ => bb46
}
}
bb44 = {
RET = [55451_u16,54005_u16,2213_u16,18453_u16,26535_u16];
_11.3 = (-5095360682776805680_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _8;
_11.0 = !6_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 101_isize ^ (-9223372036854775808_isize);
(*_6) = _11.2 as f64;
_10 = _8 + _8;
_13 = Field::<u32>(Variant(_4, 0), 1) as u8;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_11.0 = 9223372036854775807_isize & 9223372036854775807_isize;
(*_6) = Field::<f64>(Variant(_4, 0), 0);
_6 = core::ptr::addr_of!(_8);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = _10;
_1 = [13260_i16,22821_i16,30965_i16,6839_i16];
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
_10 = _11.3 as f64;
(*_6) = -Field::<f64>(Variant(_4, 0), 0);
_8 = _13 as f64;
Goto(bb3)
}
bb45 = {
_19 = _11.0 ^ _11.0;
(*_2) = [(-29931_i16),28821_i16,27599_i16,23121_i16];
Goto(bb14)
}
bb46 = {
(*_12) = _43;
_23.2 = Move(_4);
(*_2) = _1;
(*_12) = _43;
(*_2) = [23115_i16,22169_i16,6681_i16,12635_i16];
_16 = core::ptr::addr_of_mut!((*_16));
(*_12) = _43;
_35 = core::ptr::addr_of!(_49);
(*_35) = _11.0 as u64;
_23.1.2 = 23231_i16 + (-16226_i16);
_46 = &mut _33;
(*_12) = _43;
Goto(bb39)
}
bb47 = {
(*_46) = 59319_u16 + 62145_u16;
_61.2.0 = [(*_36).2,(*_36).2,(*_36).2,(*_36).2];
_61.2.0 = [(*_36).2,(*_36).2,(*_36).2,(*_36).2];
_61.2.1 = ((*_36).0, Move(_12), (*_36).2, (*_2));
(*_35) = 15743172064489470066_u64 | 12221898671047862888_u64;
_35 = core::ptr::addr_of!((*_35));
(*_46) = !12957_u16;
Goto(bb48)
}
bb48 = {
_61.2 = ((*_36).3, Move(_23.1), Move(_23.2));
(*_46) = !43819_u16;
(*_35) = !4598187812151299861_u64;
(*_35) = 1993081585286556309_u64 & 16684866927927839657_u64;
_61.0 = (-35_i8) as f64;
_50 = _13 as isize;
(*_2) = _61.2.0;
_45 = _34;
(*_46) = 53788_u16;
_44 = _61.2.1.2 << _50;
(*_16) = &_61.2.2;
(*_2) = _1;
_20 = [_50,_50,_50,_50,_50,_40,_50,_50];
_61.2.1.3 = [_44,_44,_61.2.1.2,_44];
(*_35) = Field::<f64>(Variant((*_17), 0), 0) as u64;
_4 = Adt25::Variant0 { fld0: _47,fld1: Field::<u32>(Variant((*_17), 0), 1) };
(*_46) = !117_u16;
(*_35) = _31 as u64;
_64 = _43;
_65 = _45;
_23.1.1 = Move(_61.2.1.1);
(*_2) = [_44,_44,_44,_61.2.1.2];
(*_2) = [_44,_61.2.1.2,_44,_44];
_53 = _37.1.1;
match _61.2.1.2 {
340282366920938463463374607431768194314 => bb50,
_ => bb49
}
}
bb49 = {
_10 = (*_6) - (*_6);
(*_6) = 1787008090_i32 as f64;
_7 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
_9 = !Field::<u32>(Variant(_4, 0), 1);
(*_6) = _10 + _10;
(*_6) = _10;
_3.0 = [_11.2,_11.2,_11.2,_11.2];
(*_6) = 17019_u16 as f64;
_1 = [(-3116_i16),(-21846_i16),(-18923_i16),490_i16];
(*_6) = -_10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _11.2 as f64;
_11.0 = !(-98_isize);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _10 - _10;
place!(Field::<f64>(Variant(_4, 0), 0)) = _10 * (*_6);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0) * _10;
_11.2 = '\u{63d83}' as i128;
_2 = core::ptr::addr_of_mut!(_1);
(*_6) = Field::<f64>(Variant(_4, 0), 0) + Field::<f64>(Variant(_4, 0), 0);
(*_6) = Field::<f64>(Variant(_4, 0), 0);
(*_2) = [(-20135_i16),29745_i16,(-839_i16),(-32176_i16)];
(*_2) = [(-724_i16),5953_i16,14053_i16,15221_i16];
(*_6) = (-2099883987_i32) as f64;
Goto(bb4)
}
bb50 = {
(*_2) = [_44,_44,_44,_44];
_66 = [_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3];
_69 = &_11.1;
_23.1.3 = (*_2);
_16 = core::ptr::addr_of_mut!((*_16));
_23.2 = Move(_61.2.2);
(*_46) = Field::<u32>(Variant(_4, 0), 1) as u16;
(*_16) = &_23.2;
(*_35) = 7558518397738143802_u64 * 3453303774223380105_u64;
(*_35) = 5319260218479573131_u64 + 12413035012788468311_u64;
_11.1 = _13 * _13;
Goto(bb51)
}
bb51 = {
Call(_74 = dump_var(Move(_49), Move(_20), Move(_45), Move(_64)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_74 = dump_var(Move(_55), Move(_19), Move(_66), Move(_44)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_74 = dump_var(Move(_33), Move(_9), Move(_43), _75), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: char,mut _2: &'static Adt25,mut _3: f64,mut _4: *mut [i16; 4],mut _5: ([u16; 5], *mut char, i16, [i16; 4]),mut _6: char,mut _7: u32) -> i8 {
mir! {
type RET = i8;
let _8: ([i128; 4], &'static [char; 4]);
let _9: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _10: f32;
let _11: &'static &'static (i32, (usize, u128, f32), f64);
let _12: [char; 8];
let _13: *mut &'static Adt25;
let _14: char;
let _15: *mut [i16; 4];
let _16: [i16; 5];
let _17: Adt25;
let _18: Adt18;
let _19: f64;
let _20: *mut [i16; 4];
let _21: bool;
let _22: f64;
let _23: i128;
let _24: bool;
let _25: [i32; 2];
let _26: char;
let _27: &'static mut usize;
let _28: char;
let _29: bool;
let _30: &'static [i128; 4];
let _31: bool;
let _32: Adt68;
let _33: *mut f64;
let _34: bool;
let _35: [i128; 4];
let _36: char;
let _37: Adt24;
let _38: i32;
let _39: *const [char; 4];
let _40: *mut *mut char;
let _41: f32;
let _42: i16;
let _43: [i32; 2];
let _44: *const u64;
let _45: &'static mut Adt42;
let _46: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _47: i32;
let _48: bool;
let _49: &'static Adt25;
let _50: *const [char; 4];
let _51: char;
let _52: u128;
let _53: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _54: (([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25), [char; 8], &'static mut usize);
let _55: *mut [i16; 4];
let _56: *const [usize; 3];
let _57: i64;
let _58: *mut &'static Adt25;
let _59: *mut *mut char;
let _60: usize;
let _61: *mut *mut char;
let _62: f32;
let _63: *const [usize; 3];
let _64: [u64; 1];
let _65: &'static mut usize;
let _66: *const f64;
let _67: u8;
let _68: isize;
let _69: i32;
let _70: [u16; 5];
let _71: *mut &'static Adt25;
let _72: bool;
let _73: [u16; 5];
let _74: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25));
let _75: i64;
let _76: [u64; 1];
let _77: i8;
let _78: *const isize;
let _79: isize;
let _80: usize;
let _81: ([i32; 1], &'static mut usize, Adt24);
let _82: Adt42;
let _83: [char; 4];
let _84: isize;
let _85: [u128; 5];
let _86: char;
let _87: &'static f64;
let _88: &'static (i32, (usize, u128, f32), f64);
let _89: &'static mut ([i32; 1], &'static mut usize, Adt24);
let _90: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _91: &'static mut Adt42;
let _92: *mut &'static mut ([i32; 1], &'static mut usize, Adt24);
let _93: &'static i128;
let _94: isize;
let _95: f32;
let _96: *const u64;
let _97: Adt18;
let _98: bool;
let _99: f32;
let _100: usize;
let _101: f64;
let _102: *mut *mut char;
let _103: isize;
let _104: i16;
let _105: u64;
let _106: isize;
let _107: i32;
let _108: Adt25;
let _109: ();
let _110: ();
{
_8.0 = [128995197899420730004159648182871711516_i128,(-148524642723735281549662309020916435389_i128),(-111251637414160531768590952815633316713_i128),(-167548969190446695724391324701410485106_i128)];
_5.3 = [_5.2,_5.2,_5.2,_5.2];
_3 = 8_i8 as f64;
RET = 34417_u16 as i8;
_5.2 = _7 as i16;
_5.1 = core::ptr::addr_of_mut!(_6);
_5.3 = [_5.2,_5.2,_5.2,_5.2];
Goto(bb1)
}
bb1 = {
_5.0 = [27206_u16,54029_u16,54483_u16,484_u16,25986_u16];
_5.0 = [47892_u16,26248_u16,63061_u16,64533_u16,54529_u16];
_3 = (-155868042356433180845149981388474074106_i128) as f64;
_1 = _6;
Goto(bb2)
}
bb2 = {
_8.0 = [(-19919833344780945514458882728823379883_i128),(-145492523642875918414771330933666784689_i128),15349347877190094975102085999622580997_i128,(-87226113267505346413520203760530470521_i128)];
RET = _7 as i8;
_3 = (-4658446005740998451_i64) as f64;
_5.2 = -(-388_i16);
_7 = 1743468795_u32 - 3688128472_u32;
_8.0 = [92072795071609718838549795266511528242_i128,104462241986171062433215708829567737734_i128,(-98543600535016674546030090455501135436_i128),(-70547819371412484452554302945065828488_i128)];
_6 = _1;
_5.3 = [_5.2,_5.2,_5.2,_5.2];
_3 = 156138306510245347058837900666242454473_u128 as f64;
_3 = (-7807516795429389556_i64) as f64;
_4 = core::ptr::addr_of_mut!(_5.3);
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_5.3 = [_5.2,_5.2,_5.2,_5.2];
_9.1 = [_1,_6,_1,_6,_6,_1,_1,_6];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_1 = _6;
_9.0.0 = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
(*_4) = _9.0.0;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_3 = 92_u8 as f64;
_9.0.2 = Adt25::Variant0 { fld0: _3,fld1: _7 };
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_8.0 = [(-96405119759364323552470148538323817156_i128),(-62181903386182152097266786026904656071_i128),73128155864930113140711328734232083025_i128,(-158985205199295084353705039114331705753_i128)];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
Call((*_4) = core::intrinsics::transmute(_9.0.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.0.1.3 = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.1.3;
_4 = core::ptr::addr_of_mut!((*_4));
_12 = [_1,_1,_6,_6,_6,_6,_6,_6];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_5.1 = core::ptr::addr_of_mut!(_6);
_1 = _6;
(*_4) = _9.0.1.3;
_1 = _6;
_14 = _1;
_9.0.1.0 = [44228_u16,1175_u16,63419_u16,50501_u16,40652_u16];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.1.3;
_4 = core::ptr::addr_of_mut!((*_4));
_5.1 = core::ptr::addr_of_mut!(_6);
(*_4) = _9.0.0;
_4 = core::ptr::addr_of_mut!((*_4));
_9.0.1.1 = core::ptr::addr_of_mut!(_1);
_14 = _1;
_12 = [_1,_6,_1,_14,_1,_14,_14,_14];
Call((*_4) = core::intrinsics::transmute(_9.0.1.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9.0.1.0 = _5.0;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_17 = Move(_9.0.2);
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_7 = 17542934844819050192_usize as u32;
_3 = Field::<f64>(Variant(_17, 0), 0);
_17 = Adt25::Variant0 { fld0: _3,fld1: _7 };
_5.3 = _9.0.1.3;
_9.0.1.2 = _5.2 ^ _5.2;
_9.0 = ((*_4), Move(_5), Move(_17));
_2 = &_9.0.2;
(*_4) = _9.0.0;
_16 = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_9.0.0 = (*_4);
_6 = _14;
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_5.1 = core::ptr::addr_of_mut!(_14);
_9.0.1.1 = Move(_5.1);
_1 = _14;
_14 = _1;
_9.0.1.1 = core::ptr::addr_of_mut!(_14);
Call(_7 = core::intrinsics::transmute(Field::<u32>(Variant((*_2), 0), 1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5.0 = [51965_u16,35483_u16,51800_u16,51410_u16,43728_u16];
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_1 = _6;
_5.1 = core::ptr::addr_of_mut!(_14);
_6 = _14;
(*_4) = _9.0.1.3;
_5.2 = _9.0.1.2 * _9.0.1.2;
(*_4) = [_5.2,_5.2,_9.0.1.2,_5.2];
_5.3 = _9.0.0;
(*_4) = [_9.0.1.2,_9.0.1.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_16 = [_5.2,_5.2,_5.2,_9.0.1.2,_5.2];
_9.1 = [_14,_14,_14,_6,_1,_6,_1,_14];
RET = !104_i8;
(*_4) = _9.0.0;
_17 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_2), 0), 0),fld1: Field::<u32>(Variant((*_2), 0), 1) };
(*_4) = _9.0.0;
Goto(bb6)
}
bb6 = {
_5.2 = _9.0.1.2 & _9.0.1.2;
_7 = Field::<u32>(Variant((*_2), 0), 1) + Field::<u32>(Variant((*_2), 0), 1);
_5.1 = core::ptr::addr_of_mut!(_1);
_9.1 = [_1,_6,_6,_14,_6,_6,_14,_6];
_12 = [_1,_1,_6,_1,_1,_6,_14,_1];
_9.0.1 = Move(_5);
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_3 = RET as f64;
_5.0 = [42113_u16,23138_u16,23837_u16,20318_u16,13549_u16];
_2 = &_17;
_7 = !Field::<u32>(Variant((*_2), 0), 1);
_2 = &_9.0.2;
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_5.0 = _9.0.1.0;
_5 = (_9.0.1.0, Move(_9.0.1.1), _9.0.1.2, _9.0.0);
_12 = _9.1;
_8.0 = [(-152613010573887257366525177035296778934_i128),169081065513418357408663118454746127583_i128,(-7088374265602652312674978981147847880_i128),102494612452598162129949154119122976966_i128];
Goto(bb7)
}
bb7 = {
_19 = _3;
place!(Field::<f64>(Variant(_17, 0), 0)) = -_3;
(*_4) = [_9.0.1.2,_9.0.1.2,_5.2,_5.2];
_9.0.1.1 = core::ptr::addr_of_mut!(_6);
_21 = Field::<u32>(Variant((*_2), 0), 1) == Field::<u32>(Variant((*_2), 0), 1);
_22 = 3204738024197360096_i64 as f64;
_9.0.2 = Move(_17);
_9.0.1 = (_5.0, Move(_5.1), _5.2, (*_4));
_1 = _14;
place!(Field::<f64>(Variant(_9.0.2, 0), 0)) = _9.0.1.2 as f64;
Goto(bb8)
}
bb8 = {
_9.0.2 = Adt25::Variant0 { fld0: _19,fld1: _7 };
_5.2 = _9.0.1.2;
_22 = Field::<f64>(Variant(_9.0.2, 0), 0) * Field::<f64>(Variant(_9.0.2, 0), 0);
_9.0.0 = (*_4);
_5.3 = [_9.0.1.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_9.0.1.2,_9.0.1.2];
_9.0.2 = Adt25::Variant0 { fld0: _22,fld1: _7 };
_21 = false ^ true;
_5 = (_9.0.1.0, Move(_9.0.1.1), _9.0.1.2, _9.0.1.3);
_20 = core::ptr::addr_of_mut!((*_4));
(*_20) = _9.0.1.3;
(*_4) = [_5.2,_9.0.1.2,_9.0.1.2,_5.2];
_15 = core::ptr::addr_of_mut!((*_4));
(*_15) = [_5.2,_9.0.1.2,_5.2,_5.2];
(*_4) = _9.0.0;
(*_4) = _9.0.1.3;
_26 = _6;
_22 = _3;
_6 = _14;
_8.0 = [(-68540132064795131224358124749592126306_i128),51657326649515085640451813009954427744_i128,(-122905251717194826604876928207429271700_i128),106075054543428144169631111998248139877_i128];
Goto(bb9)
}
bb9 = {
_32.fld0.1.2 = _9.0.1.2 >> _9.0.1.2;
_9.0.1 = Move(_5);
_1 = _6;
(*_4) = _9.0.1.3;
(*_4) = _9.0.1.3;
place!(Field::<f64>(Variant(_9.0.2, 0), 0)) = _3;
_32.fld1 = [12_u8,232_u8,22_u8,255_u8];
_28 = _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_9.0.0 = [_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_34 = _28 > _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2];
_17 = Move(_9.0.2);
Goto(bb10)
}
bb10 = {
(*_4) = _9.0.0;
_41 = 6726742312598037573747166855129386797_i128 as f32;
_10 = -_41;
_17 = Adt25::Variant0 { fld0: _3,fld1: _7 };
(*_4) = _9.0.0;
(*_4) = [_9.0.1.2,_32.fld0.1.2,_9.0.1.2,_9.0.1.2];
(*_4) = [_9.0.1.2,_9.0.1.2,_32.fld0.1.2,_9.0.1.2];
_5.0 = _9.0.1.0;
(*_4) = _9.0.0;
_32.fld0.0 = (*_4);
(*_4) = _32.fld0.0;
_10 = Field::<u32>(Variant(_17, 0), 1) as f32;
(*_4) = _9.0.1.3;
(*_4) = [_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2];
_29 = RET >= RET;
_34 = !_21;
_9.0.1.1 = core::ptr::addr_of_mut!(_36);
_32.fld0 = ((*_4), Move(_9.0.1), Move(_17));
_43 = [(-805625815_i32),1534230805_i32];
_9.0.1.2 = _32.fld0.1.2 << Field::<u32>(Variant(_32.fld0.2, 0), 1);
_2 = &_32.fld0.2;
_32.fld0.1.1 = core::ptr::addr_of_mut!(_26);
_25 = [(-511683133_i32),1946858414_i32];
_35 = [77709770895859744632909968378790623988_i128,20300564115825232384166092408454171227_i128,40264820654182293515640896506025183532_i128,(-135148999007547109232870842532859784569_i128)];
_33 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant((*_2), 0), 0)));
(*_4) = [_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2];
(*_4) = [_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2];
Goto(bb11)
}
bb11 = {
_32.fld0.1.2 = Field::<u32>(Variant((*_2), 0), 1) as i16;
_41 = _10 - _10;
_40 = core::ptr::addr_of_mut!(_32.fld0.1.1);
_23 = !(-75094044560271019642033770745532281605_i128);
_30 = &_8.0;
_40 = core::ptr::addr_of_mut!((*_40));
Goto(bb12)
}
bb12 = {
_3 = _22;
_5.3 = _32.fld0.1.3;
_9.0 = ((*_4), Move(_32.fld0.1), Move(_32.fld0.2));
(*_4) = _32.fld0.0;
(*_40) = core::ptr::addr_of_mut!(_1);
_17 = Move(_9.0.2);
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
(*_40) = Move(_9.0.1.1);
_3 = _19 - _22;
_32.fld0.1.2 = _9.0.1.2 | _9.0.1.2;
(*_40) = core::ptr::addr_of_mut!(_28);
_9.0.1.1 = core::ptr::addr_of_mut!(_36);
_9.0.0 = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_3 = Field::<f64>(Variant(_17, 0), 0) - _19;
_31 = !_34;
_6 = _1;
_28 = _6;
Goto(bb13)
}
bb13 = {
_42 = _32.fld0.1.2 | _9.0.1.2;
_32.fld0.1 = Move(_9.0.1);
_9.0.1 = Move(_32.fld0.1);
_36 = _28;
(*_40) = core::ptr::addr_of_mut!(_26);
_3 = Field::<f64>(Variant(_17, 0), 0) + Field::<f64>(Variant(_17, 0), 0);
(*_40) = core::ptr::addr_of_mut!(_1);
(*_4) = [_42,_42,_42,_42];
_33 = core::ptr::addr_of_mut!(_3);
_4 = Move(_15);
_26 = _28;
Goto(bb14)
}
bb14 = {
(*_33) = 12132649843479798872_u64 as f64;
_49 = &_17;
_9.0.2 = Move(_17);
(*_40) = core::ptr::addr_of_mut!(_28);
_35 = [_23,_23,_23,_23];
_29 = _31;
_34 = _21;
(*_33) = -_19;
(*_33) = _19 - _19;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0);
_8.0 = [_23,_23,_23,_23];
_3 = _22 * Field::<f64>(Variant(_9.0.2, 0), 0);
_5.2 = (-2321409686276916134_i64) as i16;
Call((*_33) = core::intrinsics::transmute(_9.0.0), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_33) = _22 * Field::<f64>(Variant(_9.0.2, 0), 0);
_35 = _8.0;
_48 = _31;
(*_40) = core::ptr::addr_of_mut!(_28);
_12 = _9.1;
_52 = 3_usize as u128;
_22 = (*_33) - (*_33);
_18 = Adt18::Variant1 { fld0: _34,fld1: _28,fld2: (-9223372036854775808_isize),fld3: (-5000565933771674610_i64),fld4: 16182130320237442061_u64,fld5: (*_33) };
(*_33) = _19 + _22;
place!(Field::<u64>(Variant(_18, 1), 4)) = 16716774821383212201_u64 - 40256736386119954_u64;
(*_33) = _22;
_3 = Field::<f64>(Variant(_18, 1), 5) * _22;
_22 = -(*_33);
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) * _22;
_28 = _6;
_32.fld0.2 = Move(_9.0.2);
_5.3 = [_42,_42,_42,_42];
_32.fld0.1.3 = _9.0.1.3;
(*_40) = core::ptr::addr_of_mut!(_28);
place!(Field::<u64>(Variant(_18, 1), 4)) = RET as u64;
_48 = (*_33) >= (*_33);
(*_33) = _22 * _22;
_32.fld0.1.0 = _9.0.1.0;
_9.0.2 = Adt25::Variant0 { fld0: (*_33),fld1: Field::<u32>(Variant(_32.fld0.2, 0), 1) };
_6 = _36;
Goto(bb16)
}
bb16 = {
_54.0.0 = [_5.2,_42,_5.2,_5.2];
(*_40) = core::ptr::addr_of_mut!(_26);
Goto(bb17)
}
bb17 = {
_54.1 = [_28,_28,_28,_14,_1,_28,_36,_36];
_54.0.1.1 = core::ptr::addr_of_mut!(_28);
(*_40) = Move(_54.0.1.1);
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - _22;
(*_33) = _22;
(*_33) = Field::<f64>(Variant(_18, 1), 5) - _22;
_54.0.1.1 = core::ptr::addr_of_mut!(_51);
_52 = 293790340142412204638441061647967483648_u128 & 212274106920629372237179234056382668522_u128;
_55 = core::ptr::addr_of_mut!(_54.0.1.3);
_32.fld0.1 = Move(_9.0.1);
_9.0.1.2 = _42;
_54.0.1.3 = [_32.fld0.1.2,_42,_5.2,_9.0.1.2];
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
place!(Field::<isize>(Variant(_18, 1), 2)) = 9223372036854775807_isize - (-57_isize);
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
_28 = _26;
_32.fld0.1.2 = _42;
_59 = core::ptr::addr_of_mut!((*_40));
(*_55) = _5.3;
(*_55) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_59) = core::ptr::addr_of_mut!(_36);
(*_40) = core::ptr::addr_of_mut!(_6);
place!(Field::<bool>(Variant(_18, 1), 0)) = !_29;
(*_33) = _22 + _22;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) + _19;
Call(_10 = core::intrinsics::transmute(Field::<u32>(Variant(_9.0.2, 0), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_33) = Field::<f64>(Variant(_18, 1), 5) * Field::<f64>(Variant(_9.0.2, 0), 0);
(*_40) = core::ptr::addr_of_mut!(_51);
place!(Field::<bool>(Variant(_18, 1), 0)) = _48;
_61 = core::ptr::addr_of_mut!((*_40));
_16 = [_32.fld0.1.2,_32.fld0.1.2,_5.2,_32.fld0.1.2,_32.fld0.1.2];
(*_61) = core::ptr::addr_of_mut!(_51);
_34 = _48;
(*_33) = _41 as f64;
_5.1 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_18, 1), 1)));
_54.0.1.0 = [37208_u16,27449_u16,13069_u16,32070_u16,4513_u16];
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - Field::<f64>(Variant(_9.0.2, 0), 0);
(*_40) = core::ptr::addr_of_mut!(_14);
(*_40) = core::ptr::addr_of_mut!(_26);
_6 = _14;
_5.0 = _32.fld0.1.0;
_33 = core::ptr::addr_of_mut!((*_33));
(*_40) = core::ptr::addr_of_mut!(_28);
_19 = Field::<u64>(Variant(_18, 1), 4) as f64;
(*_40) = core::ptr::addr_of_mut!(_1);
_20 = Move(_4);
(*_55) = [_9.0.1.2,_9.0.1.2,_32.fld0.1.2,_9.0.1.2];
_22 = (*_33) - _3;
_32.fld0.0 = [_9.0.1.2,_42,_9.0.1.2,_9.0.1.2];
_5.2 = _9.0.1.2;
(*_55) = _5.3;
_41 = Field::<u32>(Variant(_32.fld0.2, 0), 1) as f32;
_51 = _28;
Goto(bb19)
}
bb19 = {
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - Field::<f64>(Variant(_9.0.2, 0), 0);
place!(Field::<u64>(Variant(_18, 1), 4)) = 16136850397432294050_u64 << _5.2;
(*_55) = _9.0.0;
_9.0.1.2 = _32.fld0.1.2;
(*_33) = -_22;
_58 = core::ptr::addr_of_mut!(_2);
(*_40) = Move(_54.0.1.1);
_13 = core::ptr::addr_of_mut!((*_58));
(*_13) = &_9.0.2;
(*_13) = &_32.fld0.2;
_21 = !Field::<bool>(Variant(_18, 1), 0);
_9.0.1.3 = [_5.2,_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2];
_36 = _1;
_4 = core::ptr::addr_of_mut!(_32.fld0.1.3);
_9.0.2 = Move(_32.fld0.2);
(*_58) = &_9.0.2;
(*_33) = -_22;
_38 = 1787938288_i32;
_44 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_18, 1), 4)));
_34 = Field::<bool>(Variant(_18, 1), 0) & _48;
(*_44) = _34 as u64;
place!(Field::<u32>(Variant(_9.0.2, 0), 1)) = _7;
_58 = core::ptr::addr_of_mut!((*_58));
Goto(bb20)
}
bb20 = {
_31 = _21 | Field::<bool>(Variant(_18, 1), 0);
_15 = Move(_55);
_25 = [_38,_38];
place!(Field::<i64>(Variant(_18, 1), 3)) = 5434182468435116316_i64;
(*_44) = !801666051947008295_u64;
_9.0.1 = (_5.0, Move((*_40)), _42, _54.0.0);
_15 = core::ptr::addr_of_mut!(_9.0.1.3);
(*_33) = Field::<f64>(Variant((*_2), 0), 0);
(*_40) = core::ptr::addr_of_mut!(_51);
_71 = core::ptr::addr_of_mut!((*_58));
(*_15) = [_32.fld0.1.2,_9.0.1.2,_5.2,_32.fld0.1.2];
_25 = _43;
_34 = !_31;
_60 = 15677975437168204543_usize << (*_44);
(*_40) = core::ptr::addr_of_mut!(_36);
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_5.2,_42];
(*_44) = 17678358539119015243_u64;
_32.fld0.1 = Move(_5);
(*_15) = (*_4);
(*_40) = Move(_9.0.1.1);
(*_4) = [_32.fld0.1.2,_9.0.1.2,_42,_42];
(*_33) = -_22;
match (*_44) {
0 => bb6,
1 => bb13,
2 => bb4,
17678358539119015243 => bb22,
_ => bb21
}
}
bb21 = {
_5.0 = [51965_u16,35483_u16,51800_u16,51410_u16,43728_u16];
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_1 = _6;
_5.1 = core::ptr::addr_of_mut!(_14);
_6 = _14;
(*_4) = _9.0.1.3;
_5.2 = _9.0.1.2 * _9.0.1.2;
(*_4) = [_5.2,_5.2,_9.0.1.2,_5.2];
_5.3 = _9.0.0;
(*_4) = [_9.0.1.2,_9.0.1.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_16 = [_5.2,_5.2,_5.2,_9.0.1.2,_5.2];
_9.1 = [_14,_14,_14,_6,_1,_6,_1,_14];
RET = !104_i8;
(*_4) = _9.0.0;
_17 = Adt25::Variant0 { fld0: Field::<f64>(Variant((*_2), 0), 0),fld1: Field::<u32>(Variant((*_2), 0), 1) };
(*_4) = _9.0.0;
Goto(bb6)
}
bb22 = {
place!(Field::<i64>(Variant(_18, 1), 3)) = !2878253669127277148_i64;
RET = -(-30_i8);
_19 = -(*_33);
(*_33) = _23 as f64;
_54.2 = &mut _60;
(*_15) = [_42,_32.fld0.1.2,_9.0.1.2,_9.0.1.2];
_5.0 = _9.0.1.0;
(*_40) = core::ptr::addr_of_mut!(_36);
(*_33) = Field::<f64>(Variant((*_2), 0), 0);
_74.1.0 = [_32.fld0.1.2,_9.0.1.2,_9.0.1.2,_32.fld0.1.2];
match (*_44) {
0 => bb7,
1 => bb14,
2 => bb23,
3 => bb24,
4 => bb25,
5 => bb26,
17678358539119015243 => bb28,
_ => bb27
}
}
bb23 = {
_9.0.1.0 = _5.0;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_17 = Move(_9.0.2);
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_7 = 17542934844819050192_usize as u32;
_3 = Field::<f64>(Variant(_17, 0), 0);
_17 = Adt25::Variant0 { fld0: _3,fld1: _7 };
_5.3 = _9.0.1.3;
_9.0.1.2 = _5.2 ^ _5.2;
_9.0 = ((*_4), Move(_5), Move(_17));
_2 = &_9.0.2;
(*_4) = _9.0.0;
_16 = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_9.0.0 = (*_4);
_6 = _14;
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_5.1 = core::ptr::addr_of_mut!(_14);
_9.0.1.1 = Move(_5.1);
_1 = _14;
_14 = _1;
_9.0.1.1 = core::ptr::addr_of_mut!(_14);
Call(_7 = core::intrinsics::transmute(Field::<u32>(Variant((*_2), 0), 1)), ReturnTo(bb5), UnwindUnreachable())
}
bb24 = {
(*_33) = 12132649843479798872_u64 as f64;
_49 = &_17;
_9.0.2 = Move(_17);
(*_40) = core::ptr::addr_of_mut!(_28);
_35 = [_23,_23,_23,_23];
_29 = _31;
_34 = _21;
(*_33) = -_19;
(*_33) = _19 - _19;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0);
_8.0 = [_23,_23,_23,_23];
_3 = _22 * Field::<f64>(Variant(_9.0.2, 0), 0);
_5.2 = (-2321409686276916134_i64) as i16;
Call((*_33) = core::intrinsics::transmute(_9.0.0), ReturnTo(bb15), UnwindUnreachable())
}
bb25 = {
_9.0.2 = Adt25::Variant0 { fld0: _19,fld1: _7 };
_5.2 = _9.0.1.2;
_22 = Field::<f64>(Variant(_9.0.2, 0), 0) * Field::<f64>(Variant(_9.0.2, 0), 0);
_9.0.0 = (*_4);
_5.3 = [_9.0.1.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_9.0.1.2,_9.0.1.2];
_9.0.2 = Adt25::Variant0 { fld0: _22,fld1: _7 };
_21 = false ^ true;
_5 = (_9.0.1.0, Move(_9.0.1.1), _9.0.1.2, _9.0.1.3);
_20 = core::ptr::addr_of_mut!((*_4));
(*_20) = _9.0.1.3;
(*_4) = [_5.2,_9.0.1.2,_9.0.1.2,_5.2];
_15 = core::ptr::addr_of_mut!((*_4));
(*_15) = [_5.2,_9.0.1.2,_5.2,_5.2];
(*_4) = _9.0.0;
(*_4) = _9.0.1.3;
_26 = _6;
_22 = _3;
_6 = _14;
_8.0 = [(-68540132064795131224358124749592126306_i128),51657326649515085640451813009954427744_i128,(-122905251717194826604876928207429271700_i128),106075054543428144169631111998248139877_i128];
Goto(bb9)
}
bb26 = {
_5.0 = [27206_u16,54029_u16,54483_u16,484_u16,25986_u16];
_5.0 = [47892_u16,26248_u16,63061_u16,64533_u16,54529_u16];
_3 = (-155868042356433180845149981388474074106_i128) as f64;
_1 = _6;
Goto(bb2)
}
bb27 = {
_32.fld0.1.2 = _9.0.1.2 >> _9.0.1.2;
_9.0.1 = Move(_5);
_1 = _6;
(*_4) = _9.0.1.3;
(*_4) = _9.0.1.3;
place!(Field::<f64>(Variant(_9.0.2, 0), 0)) = _3;
_32.fld1 = [12_u8,232_u8,22_u8,255_u8];
_28 = _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_9.0.0 = [_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_34 = _28 > _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2];
_17 = Move(_9.0.2);
Goto(bb10)
}
bb28 = {
(*_4) = [_42,_9.0.1.2,_32.fld0.1.2,_42];
(*_15) = [_42,_42,_42,_32.fld0.1.2];
_54.0.1.3 = (*_15);
_21 = _31 ^ Field::<bool>(Variant(_18, 1), 0);
(*_33) = _19 + Field::<f64>(Variant(_18, 1), 5);
_74.1.1.0 = [6918_u16,41385_u16,60756_u16,42661_u16,54853_u16];
_35 = _8.0;
(*_40) = core::ptr::addr_of_mut!(_36);
_63 = core::ptr::addr_of!(_74.0);
_32.fld0.1.2 = _9.0.1.2 * _42;
(*_63) = [143836011346276908_usize,6388777238277108105_usize,4905841707651887584_usize];
_65 = Move(_54.2);
_36 = _6;
(*_33) = -_22;
place!(Field::<f64>(Variant(_18, 1), 5)) = (*_33) - (*_33);
(*_15) = (*_4);
(*_40) = core::ptr::addr_of_mut!(_14);
_31 = (*_33) == (*_33);
(*_63) = [4_usize,13823472926035803822_usize,1287724985470434851_usize];
_26 = _6;
(*_4) = [_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_44) = 16596029380351753573_u64 - 7652961118697340357_u64;
_74.1.1.0 = [50633_u16,37402_u16,49093_u16,60784_u16,24635_u16];
_24 = _21 < _31;
_52 = 44661102113497589401776740657508770303_u128;
Goto(bb29)
}
bb29 = {
(*_40) = core::ptr::addr_of_mut!(_51);
_42 = RET as i16;
_64 = [(*_44)];
(*_15) = [_9.0.1.2,_42,_9.0.1.2,_32.fld0.1.2];
_32.fld0.0 = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_44) = 7547857520701082995_u64 | 6781074991647396335_u64;
_1 = _26;
_1 = _14;
(*_40) = core::ptr::addr_of_mut!(_28);
_23 = (-30837613353199200986030150234599279682_i128);
(*_33) = -_19;
_72 = (*_33) > (*_33);
(*_15) = [_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_61 = Move(_59);
_32.fld0.0 = (*_15);
_54.0.1.3 = [_32.fld0.1.2,_32.fld0.1.2,_42,_9.0.1.2];
_54.1 = [_1,_14,_26,_28,_28,_51,Field::<char>(Variant(_18, 1), 1),Field::<char>(Variant(_18, 1), 1)];
_74.1.1.1 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_18, 1), 1)));
(*_40) = core::ptr::addr_of_mut!(_1);
_59 = core::ptr::addr_of_mut!((*_40));
match _52 {
0 => bb6,
1 => bb2,
44661102113497589401776740657508770303 => bb30,
_ => bb25
}
}
bb30 = {
(*_40) = core::ptr::addr_of_mut!(_36);
(*_15) = (*_4);
_36 = _26;
(*_44) = 15673051582992695463_u64 | 4350082223571711515_u64;
_9.0.0 = (*_15);
_5.2 = _32.fld0.1.2 + _32.fld0.1.2;
_73 = [18830_u16,32053_u16,30775_u16,63321_u16,22654_u16];
(*_15) = [_5.2,_5.2,_32.fld0.1.2,_5.2];
(*_44) = !8113153616196363323_u64;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
Goto(bb31)
}
bb31 = {
_54.0.1 = Move(_32.fld0.1);
_32.fld0.1.2 = _5.2 << _9.0.1.2;
(*_63) = [0_usize,4_usize,11239948375155575349_usize];
_58 = core::ptr::addr_of_mut!((*_71));
_54.1 = _12;
_56 = core::ptr::addr_of!((*_63));
_24 = _72;
(*_40) = core::ptr::addr_of_mut!(_28);
_29 = _24;
(*_40) = Move(_54.0.1.1);
_32.fld0.0 = (*_15);
_50 = core::ptr::addr_of!(_83);
(*_15) = _74.1.0;
(*_50) = [_28,_1,_36,_1];
(*_4) = _32.fld0.0;
_68 = Field::<isize>(Variant(_18, 1), 2) & Field::<isize>(Variant(_18, 1), 2);
_54.1 = [_1,Field::<char>(Variant(_18, 1), 1),_51,_26,Field::<char>(Variant(_18, 1), 1),_6,_51,Field::<char>(Variant(_18, 1), 1)];
_62 = _41;
_84 = _68;
(*_56) = [1_usize,4236801820652418565_usize,5_usize];
match _23 {
0 => bb6,
1 => bb27,
2 => bb30,
3 => bb4,
4 => bb32,
309444753567739262477344457197168931774 => bb34,
_ => bb33
}
}
bb32 = {
_32.fld0.1.2 = _9.0.1.2 >> _9.0.1.2;
_9.0.1 = Move(_5);
_1 = _6;
(*_4) = _9.0.1.3;
(*_4) = _9.0.1.3;
place!(Field::<f64>(Variant(_9.0.2, 0), 0)) = _3;
_32.fld1 = [12_u8,232_u8,22_u8,255_u8];
_28 = _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_9.0.0 = [_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_34 = _28 > _26;
(*_4) = [_32.fld0.1.2,_32.fld0.1.2,_9.0.1.2,_32.fld0.1.2];
_17 = Move(_9.0.2);
Goto(bb10)
}
bb33 = {
_54.1 = [_28,_28,_28,_14,_1,_28,_36,_36];
_54.0.1.1 = core::ptr::addr_of_mut!(_28);
(*_40) = Move(_54.0.1.1);
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - _22;
(*_33) = _22;
(*_33) = Field::<f64>(Variant(_18, 1), 5) - _22;
_54.0.1.1 = core::ptr::addr_of_mut!(_51);
_52 = 293790340142412204638441061647967483648_u128 & 212274106920629372237179234056382668522_u128;
_55 = core::ptr::addr_of_mut!(_54.0.1.3);
_32.fld0.1 = Move(_9.0.1);
_9.0.1.2 = _42;
_54.0.1.3 = [_32.fld0.1.2,_42,_5.2,_9.0.1.2];
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
place!(Field::<isize>(Variant(_18, 1), 2)) = 9223372036854775807_isize - (-57_isize);
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
_28 = _26;
_32.fld0.1.2 = _42;
_59 = core::ptr::addr_of_mut!((*_40));
(*_55) = _5.3;
(*_55) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_59) = core::ptr::addr_of_mut!(_36);
(*_40) = core::ptr::addr_of_mut!(_6);
place!(Field::<bool>(Variant(_18, 1), 0)) = !_29;
(*_33) = _22 + _22;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) + _19;
Call(_10 = core::intrinsics::transmute(Field::<u32>(Variant(_9.0.2, 0), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb34 = {
_9.0.1.2 = _54.0.1.2;
(*_15) = [_32.fld0.1.2,_32.fld0.1.2,_54.0.1.2,_5.2];
(*_50) = [_6,_6,_6,_28];
_79 = -_68;
(*_63) = [7370483076606736148_usize,4_usize,17095202908664238008_usize];
(*_40) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_18, 1), 1)));
(*_33) = _52 as f64;
place!(Field::<char>(Variant(_18, 1), 1)) = _51;
_31 = _24;
_74.1.1.0 = _73;
(*_4) = (*_15);
_9.0.1.0 = [58949_u16,8278_u16,11422_u16,11217_u16,21252_u16];
_68 = Field::<isize>(Variant(_18, 1), 2) & _79;
(*_40) = core::ptr::addr_of_mut!(_51);
_41 = _10 + _10;
(*_15) = [_32.fld0.1.2,_9.0.1.2,_5.2,_32.fld0.1.2];
_24 = !_29;
_5 = (_54.0.1.0, Move((*_40)), _42, (*_15));
Call((*_44) = core::intrinsics::transmute(_84), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
(*_44) = 3716112854257260898_u64 ^ 5628110282991393860_u64;
_57 = -Field::<i64>(Variant(_18, 1), 3);
(*_40) = core::ptr::addr_of_mut!(_36);
(*_63) = [15274222554681567411_usize,9545754934429338356_usize,5_usize];
_74.1.1.3 = [_9.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_54.0.1.2];
(*_33) = -_19;
(*_40) = core::ptr::addr_of_mut!(_28);
_50 = core::ptr::addr_of!((*_50));
(*_4) = (*_15);
(*_50) = [Field::<char>(Variant(_18, 1), 1),_6,_26,_26];
_54.0 = ((*_4), Move(_5), Move(_9.0.2));
_71 = Move(_58);
_56 = core::ptr::addr_of!((*_63));
_32.fld0.1.3 = (*_15);
(*_33) = Field::<f64>(Variant(_18, 1), 5) - Field::<f64>(Variant(_18, 1), 5);
(*_63) = [5_usize,13200697596096656353_usize,14225643630924650025_usize];
_32.fld0.0 = (*_4);
(*_4) = (*_15);
_44 = core::ptr::addr_of!((*_44));
(*_4) = (*_15);
(*_33) = Field::<f64>(Variant(_18, 1), 5);
_23 = -137721738326121451620248290320445818848_i128;
place!(Field::<bool>(Variant(_18, 1), 0)) = (*_33) <= (*_33);
_74.1.0 = [_32.fld0.1.2,_9.0.1.2,_54.0.1.2,_32.fld0.1.2];
match _52 {
0 => bb31,
1 => bb2,
2 => bb29,
3 => bb36,
44661102113497589401776740657508770303 => bb38,
_ => bb37
}
}
bb36 = {
_3 = _22;
_5.3 = _32.fld0.1.3;
_9.0 = ((*_4), Move(_32.fld0.1), Move(_32.fld0.2));
(*_4) = _32.fld0.0;
(*_40) = core::ptr::addr_of_mut!(_1);
_17 = Move(_9.0.2);
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
(*_40) = Move(_9.0.1.1);
_3 = _19 - _22;
_32.fld0.1.2 = _9.0.1.2 | _9.0.1.2;
(*_40) = core::ptr::addr_of_mut!(_28);
_9.0.1.1 = core::ptr::addr_of_mut!(_36);
_9.0.0 = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_3 = Field::<f64>(Variant(_17, 0), 0) - _19;
_31 = !_34;
_6 = _1;
_28 = _6;
Goto(bb13)
}
bb37 = {
_54.1 = [_28,_28,_28,_14,_1,_28,_36,_36];
_54.0.1.1 = core::ptr::addr_of_mut!(_28);
(*_40) = Move(_54.0.1.1);
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - _22;
(*_33) = _22;
(*_33) = Field::<f64>(Variant(_18, 1), 5) - _22;
_54.0.1.1 = core::ptr::addr_of_mut!(_51);
_52 = 293790340142412204638441061647967483648_u128 & 212274106920629372237179234056382668522_u128;
_55 = core::ptr::addr_of_mut!(_54.0.1.3);
_32.fld0.1 = Move(_9.0.1);
_9.0.1.2 = _42;
_54.0.1.3 = [_32.fld0.1.2,_42,_5.2,_9.0.1.2];
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
place!(Field::<isize>(Variant(_18, 1), 2)) = 9223372036854775807_isize - (-57_isize);
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
_28 = _26;
_32.fld0.1.2 = _42;
_59 = core::ptr::addr_of_mut!((*_40));
(*_55) = _5.3;
(*_55) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_59) = core::ptr::addr_of_mut!(_36);
(*_40) = core::ptr::addr_of_mut!(_6);
place!(Field::<bool>(Variant(_18, 1), 0)) = !_29;
(*_33) = _22 + _22;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) + _19;
Call(_10 = core::intrinsics::transmute(Field::<u32>(Variant(_9.0.2, 0), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb38 = {
_54.0.1.2 = _32.fld0.1.2 & _32.fld0.1.2;
_3 = -_19;
_15 = core::ptr::addr_of_mut!((*_15));
(*_4) = [_54.0.1.2,_54.0.1.2,_42,_9.0.1.2];
_41 = _62 + _62;
_5.3 = [_54.0.1.2,_54.0.1.2,_54.0.1.2,_54.0.1.2];
(*_63) = [17021066792152350235_usize,7288028155577824457_usize,3_usize];
(*_44) = 6073761142161382908_u64 * 3324903081952765497_u64;
(*_63) = [6_usize,1_usize,8350671769501805708_usize];
Call(_75 = core::intrinsics::bswap(_57), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
place!(Field::<u64>(Variant(_18, 1), 4)) = 6996190477380719463_u64 ^ 12304058929954448709_u64;
(*_33) = -_22;
_9.0.1.0 = _73;
_56 = core::ptr::addr_of!((*_63));
(*_4) = [_54.0.1.2,_54.0.1.2,_9.0.1.2,_32.fld0.1.2];
_9.0.2 = Adt25::Variant0 { fld0: (*_33),fld1: _7 };
_6 = _14;
_8.1 = &(*_50);
_35 = [_23,_23,_23,_23];
_94 = Field::<isize>(Variant(_18, 1), 2);
_32.fld2 = core::ptr::addr_of!((*_33));
(*_15) = (*_4);
_9.0.1.0 = [48050_u16,22939_u16,53377_u16,62705_u16,273_u16];
place!(Field::<f64>(Variant(_9.0.2, 0), 0)) = (*_33);
match _38 {
0 => bb14,
1 => bb10,
2 => bb40,
3 => bb41,
4 => bb42,
1787938288 => bb44,
_ => bb43
}
}
bb40 = {
_54.0.1.2 = _32.fld0.1.2 & _32.fld0.1.2;
_3 = -_19;
_15 = core::ptr::addr_of_mut!((*_15));
(*_4) = [_54.0.1.2,_54.0.1.2,_42,_9.0.1.2];
_41 = _62 + _62;
_5.3 = [_54.0.1.2,_54.0.1.2,_54.0.1.2,_54.0.1.2];
(*_63) = [17021066792152350235_usize,7288028155577824457_usize,3_usize];
(*_44) = 6073761142161382908_u64 * 3324903081952765497_u64;
(*_63) = [6_usize,1_usize,8350671769501805708_usize];
Call(_75 = core::intrinsics::bswap(_57), ReturnTo(bb39), UnwindUnreachable())
}
bb41 = {
_9.0.1.0 = _5.0;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_17 = Move(_9.0.2);
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_7 = 17542934844819050192_usize as u32;
_3 = Field::<f64>(Variant(_17, 0), 0);
_17 = Adt25::Variant0 { fld0: _3,fld1: _7 };
_5.3 = _9.0.1.3;
_9.0.1.2 = _5.2 ^ _5.2;
_9.0 = ((*_4), Move(_5), Move(_17));
_2 = &_9.0.2;
(*_4) = _9.0.0;
_16 = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_9.0.0 = (*_4);
_6 = _14;
(*_4) = [_9.0.1.2,_9.0.1.2,_9.0.1.2,_9.0.1.2];
_5.1 = core::ptr::addr_of_mut!(_14);
_9.0.1.1 = Move(_5.1);
_1 = _14;
_14 = _1;
_9.0.1.1 = core::ptr::addr_of_mut!(_14);
Call(_7 = core::intrinsics::transmute(Field::<u32>(Variant((*_2), 0), 1)), ReturnTo(bb5), UnwindUnreachable())
}
bb42 = {
_8.0 = [(-19919833344780945514458882728823379883_i128),(-145492523642875918414771330933666784689_i128),15349347877190094975102085999622580997_i128,(-87226113267505346413520203760530470521_i128)];
RET = _7 as i8;
_3 = (-4658446005740998451_i64) as f64;
_5.2 = -(-388_i16);
_7 = 1743468795_u32 - 3688128472_u32;
_8.0 = [92072795071609718838549795266511528242_i128,104462241986171062433215708829567737734_i128,(-98543600535016674546030090455501135436_i128),(-70547819371412484452554302945065828488_i128)];
_6 = _1;
_5.3 = [_5.2,_5.2,_5.2,_5.2];
_3 = 156138306510245347058837900666242454473_u128 as f64;
_3 = (-7807516795429389556_i64) as f64;
_4 = core::ptr::addr_of_mut!(_5.3);
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_5.3 = [_5.2,_5.2,_5.2,_5.2];
_9.1 = [_1,_6,_1,_6,_6,_1,_1,_6];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_1 = _6;
_9.0.0 = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
(*_4) = _9.0.0;
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_3 = 92_u8 as f64;
_9.0.2 = Adt25::Variant0 { fld0: _3,fld1: _7 };
(*_4) = [_5.2,_5.2,_5.2,_5.2];
_8.0 = [(-96405119759364323552470148538323817156_i128),(-62181903386182152097266786026904656071_i128),73128155864930113140711328734232083025_i128,(-158985205199295084353705039114331705753_i128)];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
Call((*_4) = core::intrinsics::transmute(_9.0.0), ReturnTo(bb3), UnwindUnreachable())
}
bb43 = {
_9.0.1.3 = [_5.2,_5.2,_5.2,_5.2];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.1.3;
_4 = core::ptr::addr_of_mut!((*_4));
_12 = [_1,_1,_6,_6,_6,_6,_6,_6];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.0;
_5.1 = core::ptr::addr_of_mut!(_6);
_1 = _6;
(*_4) = _9.0.1.3;
_1 = _6;
_14 = _1;
_9.0.1.0 = [44228_u16,1175_u16,63419_u16,50501_u16,40652_u16];
(*_4) = [_5.2,_5.2,_5.2,_5.2];
(*_4) = _9.0.1.3;
_4 = core::ptr::addr_of_mut!((*_4));
_5.1 = core::ptr::addr_of_mut!(_6);
(*_4) = _9.0.0;
_4 = core::ptr::addr_of_mut!((*_4));
_9.0.1.1 = core::ptr::addr_of_mut!(_1);
_14 = _1;
_12 = [_1,_6,_1,_14,_1,_14,_14,_14];
Call((*_4) = core::intrinsics::transmute(_9.0.1.3), ReturnTo(bb4), UnwindUnreachable())
}
bb44 = {
(*_44) = !5508287441793435253_u64;
(*_44) = 6188236064191806627_u64 + 18150979130793858282_u64;
_58 = core::ptr::addr_of_mut!(_2);
_54.0.1.3 = [_54.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
_5.1 = core::ptr::addr_of_mut!(_86);
_81.2 = Adt24::Variant1 { fld0: _52,fld1: _23,fld2: Move(_32.fld2),fld3: _18 };
_5.3 = [_54.0.1.2,_32.fld0.1.2,_54.0.1.2,_54.0.1.2];
(*_56) = [5829037125937308401_usize,17263824308653206179_usize,11652225226850783956_usize];
Goto(bb45)
}
bb45 = {
(*_50) = [_14,_28,_36,_51];
(*_56) = [5_usize,6_usize,18006836336972503935_usize];
(*_4) = [_54.0.1.2,_54.0.1.2,_9.0.1.2,_9.0.1.2];
Goto(bb46)
}
bb46 = {
(*_56) = [17472622234706875153_usize,1856044113391417169_usize,7464971446688142591_usize];
(*_50) = [_6,_36,Field::<char>(Variant(Field::<Adt18>(Variant(_81.2, 1), 3), 1), 1),Field::<char>(Variant(Field::<Adt18>(Variant(_81.2, 1), 3), 1), 1)];
_70 = [46735_u16,5770_u16,8212_u16,30246_u16,10475_u16];
_32.fld2 = Move(Field::<*const f64>(Variant(_81.2, 1), 2));
(*_58) = &_54.0.2;
(*_4) = [_54.0.1.2,_9.0.1.2,_54.0.1.2,_54.0.1.2];
(*_56) = [2_usize,11657729661354142312_usize,7_usize];
(*_15) = [_54.0.1.2,_32.fld0.1.2,_32.fld0.1.2,_42];
(*_58) = &_9.0.2;
(*_58) = &_54.0.2;
(*_40) = core::ptr::addr_of_mut!(_6);
(*_63) = [13443757665873491716_usize,10357190752343741606_usize,16294362317181962589_usize];
(*_4) = (*_15);
_59 = core::ptr::addr_of_mut!((*_40));
_48 = !Field::<bool>(Variant(_18, 1), 0);
_76 = _64;
_30 = &_35;
_39 = core::ptr::addr_of!(_83);
Goto(bb47)
}
bb47 = {
_25 = [_38,_38];
_42 = _57 as i16;
_70 = [24978_u16,34391_u16,24263_u16,7133_u16,56993_u16];
_8.0 = [_23,_23,Field::<i128>(Variant(_81.2, 1), 1),_23];
_55 = core::ptr::addr_of_mut!(_5.3);
(*_15) = [_54.0.1.2,_9.0.1.2,_54.0.1.2,_32.fld0.1.2];
(*_40) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(place!(Field::<Adt18>(Variant(_81.2, 1), 3)), 1), 1)));
(*_63) = [5_usize,17738595113953357045_usize,10551608808073834900_usize];
(*_58) = &_9.0.2;
(*_50) = [_28,_6,_14,Field::<char>(Variant(_18, 1), 1)];
(*_44) = Field::<u64>(Variant(Field::<Adt18>(Variant(_81.2, 1), 3), 1), 4) - Field::<u64>(Variant(Field::<Adt18>(Variant(_81.2, 1), 3), 1), 4);
match Field::<u128>(Variant(_81.2, 1), 0) {
0 => bb35,
44661102113497589401776740657508770303 => bb49,
_ => bb48
}
}
bb48 = {
_54.1 = [_28,_28,_28,_14,_1,_28,_36,_36];
_54.0.1.1 = core::ptr::addr_of_mut!(_28);
(*_40) = Move(_54.0.1.1);
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) - _22;
(*_33) = _22;
(*_33) = Field::<f64>(Variant(_18, 1), 5) - _22;
_54.0.1.1 = core::ptr::addr_of_mut!(_51);
_52 = 293790340142412204638441061647967483648_u128 & 212274106920629372237179234056382668522_u128;
_55 = core::ptr::addr_of_mut!(_54.0.1.3);
_32.fld0.1 = Move(_9.0.1);
_9.0.1.2 = _42;
_54.0.1.3 = [_32.fld0.1.2,_42,_5.2,_9.0.1.2];
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
place!(Field::<isize>(Variant(_18, 1), 2)) = 9223372036854775807_isize - (-57_isize);
(*_33) = -Field::<f64>(Variant(_9.0.2, 0), 0);
_28 = _26;
_32.fld0.1.2 = _42;
_59 = core::ptr::addr_of_mut!((*_40));
(*_55) = _5.3;
(*_55) = [_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2,_32.fld0.1.2];
(*_59) = core::ptr::addr_of_mut!(_36);
(*_40) = core::ptr::addr_of_mut!(_6);
place!(Field::<bool>(Variant(_18, 1), 0)) = !_29;
(*_33) = _22 + _22;
(*_33) = Field::<f64>(Variant(_9.0.2, 0), 0) + _19;
Call(_10 = core::intrinsics::transmute(Field::<u32>(Variant(_9.0.2, 0), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb49 = {
(*_44) = Field::<f64>(Variant((*_2), 0), 0) as u64;
_74.1.1.1 = Move((*_40));
_103 = _36 as isize;
_47 = _38 & _38;
_74.1 = ((*_55), Move(_54.0.1), Move(_9.0.2));
(*_40) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_18, 1), 1)));
_67 = (*_44) as u8;
(*_33) = -Field::<f64>(Variant(Field::<Adt18>(Variant(_81.2, 1), 3), 1), 5);
_18 = Field::<Adt18>(Variant(_81.2, 1), 3);
Goto(bb50)
}
bb50 = {
Call(_109 = dump_var(Move(_103), Move(_94), Move(_43), Move(_84)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_109 = dump_var(Move(_67), Move(_23), Move(_52), Move(_21)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_109 = dump_var(Move(_60), Move(_35), Move(_83), Move(_73)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_109 = dump_var(Move(_72), Move(_75), Move(_57), Move(_42)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_109 = dump_var(Move(_1), Move(_26), Move(_6), _110), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(120508309236050819795618741645747787140_u128), std::hint::black_box(157_u8));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt18 {
Variant0{
fld0: bool,
fld1: char,
fld2: isize,
fld3: u16,
fld4: (usize, u128, f32),
fld5: u64,
fld6: u128,
fld7: i128,

},
Variant1{
fld0: bool,
fld1: char,
fld2: isize,
fld3: i64,
fld4: u64,
fld5: f64,

}}
#[derive(Debug)]
pub enum Adt24 {
Variant0{
fld0: bool,
fld1: *const f64,
fld2: u64,
fld3: u32,
fld4: Adt18,
fld5: i32,
fld6: i64,
fld7: f64,

},
Variant1{
fld0: u128,
fld1: i128,
fld2: *const f64,
fld3: Adt18,

}}
#[derive(Debug)]
pub enum Adt25 {
Variant0{
fld0: f64,
fld1: u32,

},
Variant1{
fld0: *const u64,
fld1: Adt24,
fld2: Adt18,
fld3: u16,
fld4: *const f64,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt42 {
Variant0{
fld0: (i32, (usize, u128, f32), f64),

},
Variant1{
fld0: [i16; 4],
fld1: [i32; 1],
fld2: i128,
fld3: [i128; 4],

}}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: bool,
fld1: f32,
fld2: isize,
fld3: *const i8,
fld4: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25),
fld5: [u8; 5],

},
Variant1{
fld0: (isize, u8, i128, i64),
fld1: [i32; 1],
fld2: isize,

},
Variant2{
fld0: Adt18,
fld1: [u32; 8],
fld2: u8,
fld3: *const [usize; 3],
fld4: [u64; 1],
fld5: [i32; 1],
fld6: i64,
fld7: Adt24,

}}
#[derive(Debug)]
pub struct Adt68 {
fld0: ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25),
fld1: [u8; 4],
fld2: *const f64,
}
#[derive(Debug)]
pub enum Adt78 {
Variant0{
fld0: [u128; 5],
fld1: [u8; 4],
fld2: *mut u32,
fld3: f32,
fld4: [usize; 3],
fld5: *mut [i16; 4],

},
Variant1{
fld0: [i128; 6],
fld1: *mut [i16; 4],
fld2: isize,
fld3: f32,
fld4: [i32; 1],
fld5: ([u16; 5], *mut char, i16, [i16; 4]),
fld6: usize,
fld7: [usize; 3],

},
Variant2{
fld0: [char; 6],
fld1: char,
fld2: *const isize,
fld3: i64,
fld4: *const f64,
fld5: [i16; 4],

}}
#[derive(Debug)]
pub enum Adt84 {
Variant0{
fld0: f64,
fld1: *mut f64,

},
Variant1{
fld0: [char; 4],
fld1: ([usize; 3], ([i16; 4], ([u16; 5], *mut char, i16, [i16; 4]), Adt25)),
fld2: u64,
fld3: [u16; 5],
fld4: (usize, u128, f32),
fld5: [isize; 8],
fld6: i64,
fld7: Adt18,

},
Variant2{
fld0: [i128; 4],
fld1: [i32; 2],
fld2: [char; 4],

}}

