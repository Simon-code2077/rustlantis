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
pub fn fn0(mut _1: u8,mut _2: u16,mut _3: u128,mut _4: i8,mut _5: usize) -> bool {
mir! {
type RET = bool;
let _6: &'static usize;
let _7: f32;
let _8: &'static mut *const i16;
let _9: [i128; 2];
let _10: &'static mut *mut char;
let _11: f32;
let _12: f32;
let _13: isize;
let _14: (Adt23, u128);
let _15: bool;
let _16: &'static mut u8;
let _17: Adt55;
let _18: (*mut *mut i16, i32, Adt55, *mut i16);
let _19: [i32; 1];
let _20: u8;
let _21: f64;
let _22: Adt50;
let _23: &'static f64;
let _24: char;
let _25: ([char; 4], u8, [char; 4]);
let _26: f32;
let _27: *mut [u32; 7];
let _28: u16;
let _29: isize;
let _30: Adt41;
let _31: &'static i8;
let _32: i8;
let _33: Adt50;
let _34: [usize; 7];
let _35: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _36: &'static mut *const i16;
let _37: char;
let _38: Adt75;
let _39: bool;
let _40: &'static u8;
let _41: *const i16;
let _42: &'static mut *mut char;
let _43: i64;
let _44: u8;
let _45: &'static &'static mut *mut char;
let _46: isize;
let _47: bool;
let _48: char;
let _49: bool;
let _50: &'static mut *const i16;
let _51: isize;
let _52: f64;
let _53: i8;
let _54: char;
let _55: Adt55;
let _56: *mut *mut i16;
let _57: bool;
let _58: (*mut *mut i16, i32, Adt55, *mut i16);
let _59: i64;
let _60: f64;
let _61: &'static mut &'static usize;
let _62: *mut char;
let _63: *const u128;
let _64: bool;
let _65: i8;
let _66: [bool; 2];
let _67: [i8; 8];
let _68: isize;
let _69: bool;
let _70: *mut char;
let _71: u32;
let _72: &'static mut *const i16;
let _73: isize;
let _74: i128;
let _75: i64;
let _76: bool;
let _77: isize;
let _78: isize;
let _79: &'static mut *const i16;
let _80: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _81: Adt50;
let _82: bool;
let _83: &'static &'static mut u8;
let _84: char;
let _85: f32;
let _86: [u32; 7];
let _87: f32;
let _88: char;
let _89: (&'static mut &'static f64, i16, usize, [char; 4]);
let _90: *mut u32;
let _91: (&'static mut &'static f64, i16, usize, [char; 4]);
let _92: u8;
let _93: bool;
let _94: [i128; 2];
let _95: *mut ([char; 4], u8, [char; 4]);
let _96: isize;
let _97: f64;
let _98: ([char; 4], [u8; 2], &'static mut isize, char);
let _99: (*const i16,);
let _100: [i8; 8];
let _101: u64;
let _102: &'static mut *mut char;
let _103: u16;
let _104: isize;
let _105: f64;
let _106: ();
let _107: ();
{
_2 = (-27_i8) as u16;
RET = false;
_5 = 1_usize;
_3 = 271617625627181497464986805348453936284_u128 & 199961901436906481077904557111387453132_u128;
_5 = !18182563832862991497_usize;
_5 = 2069588141_i32 as usize;
_1 = 244_u8 << _2;
_6 = &_5;
_3 = 229628253792897966095393015971398495520_u128;
Call(_3 = fn1(Move(_6)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 29333601724486459471081408090021894338_u128;
_6 = &_5;
_4 = (-49262815793280249639730107075571650467_i128) as i8;
_2 = !17173_u16;
_3 = 167643413390808723414036499746310923383_u128;
_1 = !188_u8;
_2 = _1 as u16;
_7 = _2 as f32;
_7 = (-89780976593755345192108803725540457790_i128) as f32;
_1 = (-2664908531758020864_i64) as u8;
_4 = (-92_i8) ^ (-9_i8);
RET = (*_6) == (*_6);
_5 = '\u{51074}' as usize;
RET = !false;
RET = false;
RET = _3 != _3;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
167643413390808723414036499746310923383 => bb9,
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
_5 = 1_usize * 16530043105129799024_usize;
_4 = _2 as i8;
match _3 {
0 => bb6,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
167643413390808723414036499746310923383 => bb11,
_ => bb10
}
}
bb10 = {
_3 = 29333601724486459471081408090021894338_u128;
_6 = &_5;
_4 = (-49262815793280249639730107075571650467_i128) as i8;
_2 = !17173_u16;
_3 = 167643413390808723414036499746310923383_u128;
_1 = !188_u8;
_2 = _1 as u16;
_7 = _2 as f32;
_7 = (-89780976593755345192108803725540457790_i128) as f32;
_1 = (-2664908531758020864_i64) as u8;
_4 = (-92_i8) ^ (-9_i8);
RET = (*_6) == (*_6);
_5 = '\u{51074}' as usize;
RET = !false;
RET = false;
RET = _3 != _3;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
167643413390808723414036499746310923383 => bb9,
_ => bb8
}
}
bb11 = {
_1 = 176_u8 << _4;
_2 = _4 as u16;
_7 = _4 as f32;
_2 = _1 as u16;
_2 = 46197_u16 << _5;
_1 = 121_u8 ^ 62_u8;
_4 = RET as i8;
_1 = (-604625897_i32) as u8;
_9 = [(-95068405686388014110507174035964045427_i128),(-18736146812454372437836273523614108198_i128)];
_7 = _4 as f32;
_7 = (-136201836198993167708992762395791099336_i128) as f32;
_3 = 171051449196327361768842478972433380708_u128 - 230606691812711645380875668280738849043_u128;
_2 = 48896_u16;
Goto(bb12)
}
bb12 = {
_1 = 48_u8 << _3;
match _2 {
0 => bb1,
48896 => bb13,
_ => bb9
}
}
bb13 = {
_9 = [127689832705099924452367954787084311811_i128,57803606926304565598745751402529341771_i128];
_4 = -(-30_i8);
_2 = 59200_u16 ^ 32514_u16;
_1 = 205_u8;
RET = !true;
_1 = 156_u8 | 80_u8;
_9 = [(-6438917688947269531513532678922560368_i128),125907372414126620812466801608362150403_i128];
_7 = _2 as f32;
_9 = [(-107278618696380365164271783453183545502_i128),(-99232482879387268304391611199633892140_i128)];
_12 = -_7;
_1 = _4 as u8;
Goto(bb14)
}
bb14 = {
_5 = 4_usize ^ 6300934819909514440_usize;
_1 = 149_u8;
_7 = _12 * _12;
_3 = 24485673631001864149228116554191071903_u128;
RET = false | false;
_7 = _12 * _12;
_14.0 = Adt23::Variant2 { fld0: _2,fld1: 90290979514922555239953553244683210327_i128 };
_4 = (-2128749298603358531210197471594342484_i128) as i8;
place!(Field::<i128>(Variant(_14.0, 2), 1)) = 23085658441873675779373649510135900048_i128 & 3760319278926443474747067945774680464_i128;
RET = !false;
_13 = 6582391505924029780_i64 as isize;
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
RET = true | false;
_2 = Field::<u16>(Variant(_14.0, 2), 0) & Field::<u16>(Variant(_14.0, 2), 0);
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
_5 = (-586557958_i32) as usize;
RET = Field::<i128>(Variant(_14.0, 2), 1) == Field::<i128>(Variant(_14.0, 2), 1);
_12 = -_7;
place!(Field::<i128>(Variant(_14.0, 2), 1)) = (-87979656104509080751507903443186758255_i128) ^ 159117479864128842694448192553627843734_i128;
_16 = &mut _1;
_11 = _12;
(*_16) = 25_u8;
(*_16) = 142_u8 * 162_u8;
(*_16) = !102_u8;
(*_16) = 2294626590_u32 as u8;
match _3 {
24485673631001864149228116554191071903 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
(*_16) = 183_u8 >> _3;
(*_16) = 186_u8 | 191_u8;
(*_16) = 89_u8 << _4;
_20 = !(*_16);
_11 = _12 * _7;
_15 = RET;
(*_16) = '\u{679d3}' as u8;
_18.1 = 1398286140343793360_u64 as i32;
_19 = [_18.1];
(*_16) = _5 as u8;
(*_16) = !_20;
RET = !_15;
_5 = 6_usize << (*_16);
(*_16) = _20 >> _5;
_11 = -_7;
_7 = _3 as f32;
_2 = 12867320992554539766_u64 as u16;
(*_16) = _20 >> _2;
(*_16) = _18.1 as u8;
(*_16) = _11 as u8;
(*_16) = 838730204047508741_u64 as u8;
(*_16) = _20;
Goto(bb17)
}
bb17 = {
_18.0 = core::ptr::addr_of_mut!(_18.3);
_14.1 = _3 | _3;
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
(*_16) = !_20;
(*_16) = !_20;
(*_16) = !_20;
(*_16) = _20;
(*_16) = _20 + _20;
_21 = _12 as f64;
(*_16) = _13 as u8;
(*_16) = !_20;
_25.1 = (*_16) >> (*_16);
Call(_3 = core::intrinsics::transmute(Field::<i128>(Variant(_14.0, 2), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_22.fld1 = _19;
(*_16) = !_25.1;
(*_16) = _15 as u8;
(*_16) = _25.1;
_19 = _22.fld1;
(*_16) = _20;
(*_16) = _20;
_26 = _12 + _7;
(*_16) = _25.1 >> _18.1;
(*_16) = 9319691303487307325_u64 as u8;
_13 = _4 as isize;
_21 = _14.1 as f64;
_25.0 = ['\u{b7697}','\u{28940}','\u{9abdd}','\u{287dd}'];
(*_16) = _25.1;
_23 = &_21;
(*_16) = _25.1;
(*_16) = _25.1;
(*_16) = _25.1 - _20;
_18.0 = core::ptr::addr_of_mut!(_18.3);
Goto(bb19)
}
bb19 = {
_18.0 = core::ptr::addr_of_mut!(_18.3);
_25.1 = _4 as u8;
(*_16) = _25.1 + _20;
_16 = &mut _20;
(*_16) = _25.1;
_21 = _18.1 as f64;
(*_16) = _4 as u8;
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
_7 = Field::<u16>(Variant(_14.0, 2), 0) as f32;
Goto(bb20)
}
bb20 = {
_3 = _14.1 | _14.1;
(*_16) = _15 as u8;
_21 = _4 as f64;
(*_16) = _4 as u8;
(*_16) = _25.1 * _25.1;
_29 = '\u{4d3aa}' as isize;
_24 = '\u{145fb}';
(*_16) = _25.1 >> Field::<i128>(Variant(_14.0, 2), 1);
(*_16) = _7 as u8;
Goto(bb21)
}
bb21 = {
(*_16) = _25.1 ^ _25.1;
(*_16) = !_25.1;
_11 = _12 + _12;
(*_16) = _25.1 >> _13;
_22.fld0 = [_24,_24,_24,_24];
(*_16) = Field::<i128>(Variant(_14.0, 2), 1) as u8;
_5 = _15 as usize;
(*_16) = Field::<u16>(Variant(_14.0, 2), 0) as u8;
_5 = _4 as usize;
_25.1 = (*_16);
_25 = (_22.fld0, (*_16), _22.fld0);
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
_9 = [Field::<i128>(Variant(_14.0, 2), 1),Field::<i128>(Variant(_14.0, 2), 1)];
(*_16) = !_25.1;
_26 = _4 as f32;
_33.fld0 = [_24,_24,_24,_24];
_25.1 = (*_16) | (*_16);
Call((*_16) = core::intrinsics::transmute(_15), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_33.fld0 = [_24,_24,_24,_24];
_11 = -_12;
(*_16) = _24 as u8;
(*_16) = 3988654961_u32 as u8;
(*_16) = _25.1 * _25.1;
_25 = (_33.fld0, (*_16), _33.fld0);
_33.fld0 = [_24,_24,_24,_24];
_25 = (_22.fld0, (*_16), _22.fld0);
_3 = (-17168_i16) as u128;
_7 = -_11;
_30 = Adt41::Variant1 { fld0: RET,fld1: 12471889565788222125_u64,fld2: _21,fld3: _4,fld4: 1919183135_u32 };
(*_16) = !_25.1;
_4 = !Field::<i8>(Variant(_30, 1), 3);
_14.0 = Adt23::Variant0 { fld0: 525978564_u32,fld1: Field::<f64>(Variant(_30, 1), 2),fld2: _13 };
(*_16) = _25.1;
_5 = 11286465753924248874_usize >> (*_16);
(*_16) = !_25.1;
_34 = [_5,_5,_5,_5,_5,_5,_5];
_31 = &place!(Field::<i8>(Variant(_30, 1), 3));
place!(Field::<bool>(Variant(_30, 1), 0)) = (*_31) < (*_31);
(*_16) = !_25.1;
(*_16) = _11 as u8;
place!(Field::<u32>(Variant(_30, 1), 4)) = !2660668474_u32;
(*_16) = _25.1 + _25.1;
Goto(bb23)
}
bb23 = {
(*_16) = _12 as u8;
_33.fld0 = [_24,_24,_24,_24];
(*_16) = _25.1 - _25.1;
Goto(bb24)
}
bb24 = {
(*_16) = _25.1 & _25.1;
_22.fld0 = _33.fld0;
_34 = [_5,_5,_5,_5,_5,_5,_5];
_33.fld1 = _19;
_19 = [_18.1];
Goto(bb25)
}
bb25 = {
_18.0 = core::ptr::addr_of_mut!(_18.3);
_16 = &mut _25.1;
(*_16) = 247_u8;
place!(Field::<u64>(Variant(_30, 1), 1)) = 11035336516905393748_u64 ^ 5379230188451894964_u64;
_35.0 = Move(_30);
(*_16) = Field::<f64>(Variant(_14.0, 0), 1) as u8;
_23 = &place!(Field::<f64>(Variant(_35.0, 1), 2));
(*_16) = 124_u8 << _5;
(*_16) = 154_u8 | 247_u8;
(*_16) = 129_u8 * 213_u8;
(*_16) = 88_u8 << _5;
_27 = core::ptr::addr_of_mut!(_35.1.1);
_11 = -_12;
_33.fld1 = [_18.1];
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
Goto(bb26)
}
bb26 = {
_7 = _26;
_35.2 = &(*_23);
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
_39 = !_15;
_32 = Field::<i8>(Variant(_35.0, 1), 3) | _4;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
_48 = _24;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
place!(Field::<f64>(Variant(_14.0, 0), 1)) = (*_23) * (*_23);
(*_16) = 122_u8 << _5;
Goto(bb27)
}
bb27 = {
_18.1 = (-388975207_i32);
_35.0 = Adt41::Variant1 { fld0: RET,fld1: 8141158351372859526_u64,fld2: Field::<f64>(Variant(_14.0, 0), 1),fld3: _32,fld4: 1748226045_u32 };
(*_27) = [2496199920_u32,3629682583_u32,2755392458_u32,1168527609_u32,1185651629_u32,3883523184_u32,379673178_u32];
_11 = _26 * _7;
Call(_32 = core::intrinsics::bswap(Field::<i8>(Variant(_35.0, 1), 3)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_46 = Field::<f64>(Variant(_14.0, 0), 1) as isize;
_48 = _24;
(*_27) = [1973617637_u32,649481173_u32,1241914677_u32,1186764702_u32,3199568348_u32,121999988_u32,2489375576_u32];
_48 = _24;
(*_27) = [3619667710_u32,4027522895_u32,2219805963_u32,762876013_u32,2338940817_u32,329922358_u32,2020754936_u32];
(*_27) = [3450423119_u32,2705910058_u32,3946204195_u32,1843241769_u32,396730911_u32,1290584638_u32,1984535122_u32];
(*_16) = 210_u8 * 221_u8;
_13 = _46;
place!(Field::<u32>(Variant(_14.0, 0), 0)) = 2125900975_u32;
_47 = (*_16) != (*_16);
_43 = (-7723646549954052274_i64);
_22.fld1 = [_18.1];
(*_27) = [Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0)];
_14.0 = Adt23::Variant2 { fld0: _2,fld1: (-16529823566389734196446494736890952050_i128) };
_47 = (*_16) == (*_16);
(*_27) = [150076374_u32,1566426641_u32,1990210862_u32,3305239493_u32,1547921896_u32,4030749620_u32,302469723_u32];
_53 = _32 ^ _32;
_49 = !_15;
_15 = Field::<bool>(Variant(_35.0, 1), 0) ^ Field::<bool>(Variant(_35.0, 1), 0);
(*_16) = 98_u8;
_49 = !_47;
_35.1.1 = [1783531060_u32,1178380474_u32,2390265037_u32,1134272833_u32,3354162769_u32,1104686728_u32,2416579046_u32];
_44 = (*_16) | (*_16);
_9 = [(-65974238839684910815617967421501308985_i128),117500650949412120725131620720879427990_i128];
_51 = -_46;
Goto(bb29)
}
bb29 = {
_39 = (*_16) == (*_16);
place!(Field::<u16>(Variant(_14.0, 2), 0)) = _2 * _2;
_11 = -_12;
_43 = _47 as i64;
_54 = _24;
_56 = core::ptr::addr_of_mut!(_18.3);
_31 = &place!(Field::<i8>(Variant(_35.0, 1), 3));
(*_27) = [2519923587_u32,2404096734_u32,3085335816_u32,2377376760_u32,3224842904_u32,46787688_u32,547731498_u32];
_46 = _11 as isize;
_2 = Field::<u16>(Variant(_14.0, 2), 0);
_15 = !_49;
(*_16) = _44;
(*_16) = _44;
(*_16) = _44 * _44;
_14.0 = Adt23::Variant0 { fld0: 1415921061_u32,fld1: Field::<f64>(Variant(_35.0, 1), 2),fld2: _46 };
(*_16) = (*_31) as u8;
_46 = Field::<isize>(Variant(_14.0, 0), 2) | _13;
_27 = core::ptr::addr_of_mut!((*_27));
place!(Field::<u32>(Variant(_14.0, 0), 0)) = (*_31) as u32;
_28 = _26 as u16;
Goto(bb30)
}
bb30 = {
_61 = &mut _6;
_64 = _49 ^ _15;
_46 = Field::<isize>(Variant(_14.0, 0), 2);
place!(Field::<f64>(Variant(_14.0, 0), 1)) = Field::<f64>(Variant(_35.0, 1), 2);
_39 = !_64;
place!(Field::<u32>(Variant(_35.0, 1), 4)) = (*_31) as u32;
_58.0 = core::ptr::addr_of_mut!((*_56));
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4)];
_23 = &place!(Field::<f64>(Variant(_35.0, 1), 2));
_15 = (*_31) < _53;
place!(Field::<bool>(Variant(_35.0, 1), 0)) = !RET;
match _18.1 {
0 => bb31,
1 => bb32,
2 => bb33,
340282366920938463463374607431379236249 => bb35,
_ => bb34
}
}
bb31 = {
_18.0 = core::ptr::addr_of_mut!(_18.3);
_16 = &mut _25.1;
(*_16) = 247_u8;
place!(Field::<u64>(Variant(_30, 1), 1)) = 11035336516905393748_u64 ^ 5379230188451894964_u64;
_35.0 = Move(_30);
(*_16) = Field::<f64>(Variant(_14.0, 0), 1) as u8;
_23 = &place!(Field::<f64>(Variant(_35.0, 1), 2));
(*_16) = 124_u8 << _5;
(*_16) = 154_u8 | 247_u8;
(*_16) = 129_u8 * 213_u8;
(*_16) = 88_u8 << _5;
_27 = core::ptr::addr_of_mut!(_35.1.1);
_11 = -_12;
_33.fld1 = [_18.1];
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
Goto(bb26)
}
bb32 = {
Return()
}
bb33 = {
_33.fld0 = [_24,_24,_24,_24];
_11 = -_12;
(*_16) = _24 as u8;
(*_16) = 3988654961_u32 as u8;
(*_16) = _25.1 * _25.1;
_25 = (_33.fld0, (*_16), _33.fld0);
_33.fld0 = [_24,_24,_24,_24];
_25 = (_22.fld0, (*_16), _22.fld0);
_3 = (-17168_i16) as u128;
_7 = -_11;
_30 = Adt41::Variant1 { fld0: RET,fld1: 12471889565788222125_u64,fld2: _21,fld3: _4,fld4: 1919183135_u32 };
(*_16) = !_25.1;
_4 = !Field::<i8>(Variant(_30, 1), 3);
_14.0 = Adt23::Variant0 { fld0: 525978564_u32,fld1: Field::<f64>(Variant(_30, 1), 2),fld2: _13 };
(*_16) = _25.1;
_5 = 11286465753924248874_usize >> (*_16);
(*_16) = !_25.1;
_34 = [_5,_5,_5,_5,_5,_5,_5];
_31 = &place!(Field::<i8>(Variant(_30, 1), 3));
place!(Field::<bool>(Variant(_30, 1), 0)) = (*_31) < (*_31);
(*_16) = !_25.1;
(*_16) = _11 as u8;
place!(Field::<u32>(Variant(_30, 1), 4)) = !2660668474_u32;
(*_16) = _25.1 + _25.1;
Goto(bb23)
}
bb34 = {
_7 = _26;
_35.2 = &(*_23);
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
_39 = !_15;
_32 = Field::<i8>(Variant(_35.0, 1), 3) | _4;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
_48 = _24;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
place!(Field::<f64>(Variant(_14.0, 0), 1)) = (*_23) * (*_23);
(*_16) = 122_u8 << _5;
Goto(bb27)
}
bb35 = {
_2 = _28 * _28;
_54 = _24;
(*_16) = _44 << (*_31);
_18.1 = -1489096439_i32;
Goto(bb36)
}
bb36 = {
_66 = [_64,_15];
(*_16) = _44 ^ _44;
_30 = Adt41::Variant1 { fld0: _47,fld1: 6090088300005926460_u64,fld2: (*_23),fld3: (*_31),fld4: Field::<u32>(Variant(_35.0, 1), 4) };
(*_16) = _44 & _44;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0)];
place!(Field::<u32>(Variant(_30, 1), 4)) = Field::<u32>(Variant(_14.0, 0), 0);
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
_68 = _46 + _46;
_11 = -_12;
_27 = core::ptr::addr_of_mut!((*_27));
_35.0 = Adt41::Variant1 { fld0: _47,fld1: 18409310555122311593_u64,fld2: Field::<f64>(Variant(_30, 1), 2),fld3: _53,fld4: Field::<u32>(Variant(_30, 1), 4) };
_51 = _46 << (*_16);
_68 = _51 & Field::<isize>(Variant(_14.0, 0), 2);
(*_16) = 160945050803989996242035448219927132810_i128 as u8;
(*_16) = !_44;
_7 = _11 * _12;
_67 = [_53,_53,_53,_53,_53,Field::<i8>(Variant(_35.0, 1), 3),_53,Field::<i8>(Variant(_35.0, 1), 3)];
_62 = core::ptr::addr_of_mut!(_24);
(*_16) = _18.1 as u8;
Goto(bb37)
}
bb37 = {
(*_62) = _54;
Goto(bb38)
}
bb38 = {
(*_27) = [Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_14.0, 0), 0)];
_60 = _5 as f64;
_40 = &(*_16);
(*_16) = _44 + _44;
(*_62) = _48;
_51 = !Field::<isize>(Variant(_14.0, 0), 2);
(*_16) = _44;
(*_16) = Field::<f64>(Variant(_35.0, 1), 2) as u8;
(*_27) = [Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_14.0, 0), 0),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
(*_16) = _44 - _44;
_15 = Field::<bool>(Variant(_30, 1), 0) <= _64;
place!(Field::<u32>(Variant(_35.0, 1), 4)) = (*_62) as u32;
(*_62) = _48;
_35.1.0 = &_21;
(*_16) = _44;
(*_16) = _44;
_14.0 = Adt23::Variant2 { fld0: _28,fld1: 50449691309625168970059063630486446314_i128 };
_24 = _54;
(*_62) = _54;
_24 = _54;
_42 = &mut _62;
(*_42) = core::ptr::addr_of_mut!(_54);
_70 = core::ptr::addr_of_mut!(_37);
(*_16) = _44;
Goto(bb39)
}
bb39 = {
(*_70) = _24;
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
(*_16) = _44 << _32;
_59 = _43 >> Field::<u32>(Variant(_30, 1), 4);
place!(Field::<u64>(Variant(_30, 1), 1)) = !15659717933766844356_u64;
(*_42) = core::ptr::addr_of_mut!((*_70));
place!(Field::<i128>(Variant(_14.0, 2), 1)) = _5 as i128;
(*_42) = Move(_70);
(*_16) = !_44;
(*_42) = core::ptr::addr_of_mut!(_37);
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
_52 = -_60;
(*_42) = core::ptr::addr_of_mut!(_24);
_32 = _53 ^ Field::<i8>(Variant(_35.0, 1), 3);
(*_16) = _44;
_60 = Field::<f64>(Variant(_35.0, 1), 2) + _52;
Goto(bb40)
}
bb40 = {
_29 = _68 | _51;
Goto(bb41)
}
bb41 = {
(*_16) = _44;
_80.2.0 = Adt23::Variant2 { fld0: _28,fld1: Field::<i128>(Variant(_14.0, 2), 1) };
place!(Field::<f64>(Variant(_30, 1), 2)) = Field::<u64>(Variant(_30, 1), 1) as f64;
_18.1 = Field::<u32>(Variant(_30, 1), 4) as i32;
place!(Field::<u64>(Variant(_30, 1), 1)) = Field::<i128>(Variant(_14.0, 2), 1) as u64;
(*_16) = _44 - _44;
_30 = Adt41::Variant1 { fld0: _15,fld1: 12172638779357681725_u64,fld2: _52,fld3: _32,fld4: Field::<u32>(Variant(_35.0, 1), 4) };
_82 = !_64;
_19 = _22.fld1;
(*_16) = _44;
Goto(bb42)
}
bb42 = {
_51 = _46;
_46 = _68 + _68;
(*_16) = _48 as u8;
(*_16) = _44;
place!(Field::<u64>(Variant(_35.0, 1), 1)) = 4773296708228982086_u64 * 3858719484684720034_u64;
_5 = _32 as usize;
Goto(bb43)
}
bb43 = {
_64 = _46 >= _46;
_64 = !Field::<bool>(Variant(_35.0, 1), 0);
_60 = 20486_i16 as f64;
_4 = !Field::<i8>(Variant(_30, 1), 3);
_14 = (_80.2.0, _3);
(*_42) = core::ptr::addr_of_mut!(_84);
(*_42) = core::ptr::addr_of_mut!(_54);
_58.1 = _18.1 - _18.1;
_12 = _7;
_59 = _43 * _43;
_19 = [_58.1];
_10 = &mut (*_42);
_57 = _64;
_35.3 = Adt41::Variant1 { fld0: _39,fld1: Field::<u64>(Variant(_35.0, 1), 1),fld2: Field::<f64>(Variant(_35.0, 1), 2),fld3: _32,fld4: Field::<u32>(Variant(_35.0, 1), 4) };
Goto(bb44)
}
bb44 = {
(*_16) = _44 ^ _44;
(*_27) = [Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_35.3, 1), 4)];
_28 = Field::<u16>(Variant(_80.2.0, 2), 0);
(*_10) = core::ptr::addr_of_mut!(_37);
_80.0 = !_2;
_4 = Field::<i8>(Variant(_30, 1), 3) << _29;
(*_10) = core::ptr::addr_of_mut!(_24);
_29 = _46 ^ _51;
_81.fld1 = _19;
(*_10) = core::ptr::addr_of_mut!(_48);
(*_10) = core::ptr::addr_of_mut!(_24);
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_30, 1), 4)];
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_35.3, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
_56 = core::ptr::addr_of_mut!((*_56));
_54 = _37;
(*_10) = core::ptr::addr_of_mut!(_37);
_81.fld0 = _33.fld0;
_78 = _29 + _13;
_52 = Field::<f64>(Variant(_30, 1), 2) * Field::<f64>(Variant(_35.3, 1), 2);
_35.0 = Move(_35.3);
Goto(bb45)
}
bb45 = {
place!(Field::<bool>(Variant(_35.0, 1), 0)) = Field::<bool>(Variant(_30, 1), 0);
_21 = Field::<u64>(Variant(_35.0, 1), 1) as f64;
_67 = [_32,_32,_4,_4,_4,_32,Field::<i8>(Variant(_35.0, 1), 3),_32];
(*_10) = core::ptr::addr_of_mut!(_37);
(*_10) = core::ptr::addr_of_mut!(_54);
(*_56) = core::ptr::addr_of_mut!(_89.1);
_18.1 = _58.1 >> (*_16);
_33.fld0 = [_37,_48,_54,_48];
_26 = _12;
place!(Field::<u64>(Variant(_30, 1), 1)) = Field::<u64>(Variant(_35.0, 1), 1) + Field::<u64>(Variant(_35.0, 1), 1);
place!(Field::<u32>(Variant(_30, 1), 4)) = !Field::<u32>(Variant(_35.0, 1), 4);
_44 = (*_16) | (*_16);
_41 = core::ptr::addr_of!(_89.1);
_10 = Move(_42);
(*_41) = (-25229_i16) & 8721_i16;
_11 = _12 + _26;
Goto(bb46)
}
bb46 = {
_73 = _78 - _68;
(*_16) = _44;
(*_56) = core::ptr::addr_of_mut!((*_41));
(*_16) = Field::<i128>(Variant(_14.0, 2), 1) as u8;
_89.2 = _5;
(*_41) = (-21604_i16) ^ (-15703_i16);
_70 = core::ptr::addr_of_mut!(_37);
Goto(bb47)
}
bb47 = {
(*_41) = !(-6348_i16);
Goto(bb48)
}
bb48 = {
_63 = core::ptr::addr_of!(_3);
_35.0 = Adt41::Variant1 { fld0: _15,fld1: Field::<u64>(Variant(_30, 1), 1),fld2: Field::<f64>(Variant(_30, 1), 2),fld3: _4,fld4: Field::<u32>(Variant(_30, 1), 4) };
_10 = &mut _70;
_11 = _12 + _12;
(*_16) = Field::<f64>(Variant(_30, 1), 2) as u8;
(*_41) = (-2643_i16);
_35.1.0 = &_52;
(*_61) = &_89.2;
(*_41) = 17740_i16;
(*_56) = core::ptr::addr_of_mut!((*_41));
_84 = _24;
(*_27) = [Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_35.0, 1), 4),Field::<u32>(Variant(_30, 1), 4),Field::<u32>(Variant(_35.0, 1), 4)];
(*_56) = core::ptr::addr_of_mut!(_91.1);
(*_16) = !_44;
_80.3 = core::ptr::addr_of_mut!(_35.1.0);
_24 = _48;
match (*_41) {
0 => bb49,
17740 => bb51,
_ => bb50
}
}
bb49 = {
(*_16) = _44;
_80.2.0 = Adt23::Variant2 { fld0: _28,fld1: Field::<i128>(Variant(_14.0, 2), 1) };
place!(Field::<f64>(Variant(_30, 1), 2)) = Field::<u64>(Variant(_30, 1), 1) as f64;
_18.1 = Field::<u32>(Variant(_30, 1), 4) as i32;
place!(Field::<u64>(Variant(_30, 1), 1)) = Field::<i128>(Variant(_14.0, 2), 1) as u64;
(*_16) = _44 - _44;
_30 = Adt41::Variant1 { fld0: _15,fld1: 12172638779357681725_u64,fld2: _52,fld3: _32,fld4: Field::<u32>(Variant(_35.0, 1), 4) };
_82 = !_64;
_19 = _22.fld1;
(*_16) = _44;
Goto(bb42)
}
bb50 = {
_1 = 176_u8 << _4;
_2 = _4 as u16;
_7 = _4 as f32;
_2 = _1 as u16;
_2 = 46197_u16 << _5;
_1 = 121_u8 ^ 62_u8;
_4 = RET as i8;
_1 = (-604625897_i32) as u8;
_9 = [(-95068405686388014110507174035964045427_i128),(-18736146812454372437836273523614108198_i128)];
_7 = _4 as f32;
_7 = (-136201836198993167708992762395791099336_i128) as f32;
_3 = 171051449196327361768842478972433380708_u128 - 230606691812711645380875668280738849043_u128;
_2 = 48896_u16;
Goto(bb12)
}
bb51 = {
(*_16) = !_44;
_22.fld1 = _19;
(*_56) = core::ptr::addr_of_mut!((*_41));
_74 = -Field::<i128>(Variant(_14.0, 2), 1);
_21 = _73 as f64;
_35.1.0 = &_21;
(*_16) = _26 as u8;
_89.3 = _22.fld0;
_7 = _26 + _26;
_91.0 = &mut _35.1.0;
(*_41) = (-10924_i16) << (*_63);
place!(Field::<f64>(Variant(_30, 1), 2)) = -_21;
_98.1 = [(*_16),(*_16)];
(*_61) = &_5;
_2 = (*_63) as u16;
_26 = Field::<i128>(Variant(_14.0, 2), 1) as f32;
(*_10) = core::ptr::addr_of_mut!(_88);
(*_61) = &_89.2;
(*_61) = &_5;
_99.0 = core::ptr::addr_of!((*_41));
_45 = &_10;
_88 = _84;
_18.0 = core::ptr::addr_of_mut!((*_56));
_72 = &mut _99.0;
_40 = &_44;
_98.3 = _37;
_87 = -_26;
Goto(bb52)
}
bb52 = {
Call(_106 = dump_var(Move(_44), Move(_24), Move(_34), Move(_39)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_106 = dump_var(Move(_46), Move(_48), Move(_82), Move(_5)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_106 = dump_var(Move(_78), Move(_9), Move(_4), Move(_66)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_106 = dump_var(Move(_25), Move(_37), Move(_49), Move(_15)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_106 = dump_var(Move(_67), Move(_32), Move(_59), _107), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static usize) -> u128 {
mir! {
type RET = u128;
let _2: &'static mut i8;
let _3: &'static usize;
let _4: char;
let _5: (*mut *mut i16, i32, Adt55, *mut i16);
let _6: f64;
let _7: isize;
let _8: u8;
let _9: &'static mut *const i16;
let _10: isize;
let _11: (&'static f64, [u32; 7]);
let _12: usize;
let _13: (&'static mut &'static f64, i16, usize, [char; 4]);
let _14: &'static i8;
let _15: &'static u8;
let _16: &'static mut *mut char;
let _17: *const char;
let _18: char;
let _19: f32;
let _20: [i8; 8];
let _21: bool;
let _22: &'static mut &'static f64;
let _23: *const &'static mut &'static f64;
let _24: f64;
let _25: *mut u32;
let _26: &'static mut &'static usize;
let _27: isize;
let _28: bool;
let _29: Adt44;
let _30: *mut *mut i16;
let _31: *mut char;
let _32: f32;
let _33: &'static mut *const i16;
let _34: *const char;
let _35: *mut &'static mut u8;
let _36: char;
let _37: *const char;
let _38: f64;
let _39: [bool; 2];
let _40: f32;
let _41: [usize; 7];
let _42: f32;
let _43: f64;
let _44: &'static mut u8;
let _45: &'static mut isize;
let _46: Adt28;
let _47: u32;
let _48: char;
let _49: &'static &'static mut *mut char;
let _50: *mut ([char; 4], u8, [char; 4]);
let _51: &'static mut u8;
let _52: i8;
let _53: char;
let _54: isize;
let _55: *mut *mut i16;
let _56: char;
let _57: &'static mut *const i16;
let _58: *mut char;
let _59: *const u128;
let _60: isize;
let _61: isize;
let _62: isize;
let _63: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _64: bool;
let _65: i8;
let _66: &'static mut *const i16;
let _67: &'static bool;
let _68: char;
let _69: [i64; 3];
let _70: isize;
let _71: [i64; 3];
let _72: u64;
let _73: i32;
let _74: &'static mut *mut char;
let _75: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _76: *mut &'static f64;
let _77: *mut char;
let _78: *const u128;
let _79: Adt41;
let _80: u64;
let _81: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _82: &'static mut &'static f64;
let _83: [i8; 8];
let _84: u64;
let _85: (*mut &'static f64, &'static mut isize);
let _86: Adt23;
let _87: i16;
let _88: f64;
let _89: u128;
let _90: char;
let _91: u8;
let _92: (&'static f64, [u32; 7]);
let _93: u64;
let _94: Adt41;
let _95: i8;
let _96: (*mut *mut i16, i32, Adt55, *mut i16);
let _97: char;
let _98: Adt55;
let _99: i128;
let _100: char;
let _101: isize;
let _102: f64;
let _103: &'static mut *mut char;
let _104: &'static usize;
let _105: &'static bool;
let _106: *mut ([char; 4], u8, [char; 4]);
let _107: &'static mut u16;
let _108: f64;
let _109: *mut &'static mut u8;
let _110: i128;
let _111: &'static mut i8;
let _112: (*const i16,);
let _113: ();
let _114: ();
{
RET = 116709232889594278453633095221857155539_u128 >> 6935513183606365689_i64;
RET = !69974263312478167455802757558097231434_u128;
RET = 292088255619862815205440800601917632589_u128;
RET = (-60_i8) as u128;
RET = '\u{e37e}' as u128;
RET = 270482766252531011311442655903386256150_u128 >> (-53_isize);
RET = 284932447393160339111298389964267610219_u128;
RET = !119469905883754123568597467798507608206_u128;
RET = (-66_i8) as u128;
RET = 129284610118315639998797810986904896853_i128 as u128;
RET = !233319382825925995884622027496473323981_u128;
RET = 312469652420597284244867950177221779724_u128 << (-1784853335363323292_i64);
RET = !335501168679781143054463609562091573355_u128;
RET = 203479372291960955653346746825872446518_u128 - 63979733382476375720214073386900326059_u128;
_4 = '\u{2cc4b}';
_4 = '\u{17405}';
RET = 151346196232343652251512572451267756038_u128 - 75677495313049277893858498807280948925_u128;
_5.1 = 1917622838_i32 - 2086927809_i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
Call(_5.3 = fn2(Move(_5.0), _4, _4, _4, _5.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5.1 = (-1944311965_i32) & 1475794740_i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = !293897226484798638041909357778124232313_u128;
_4 = '\u{283b1}';
RET = (-104_i8) as u128;
RET = 61082_u16 as u128;
_5.1 = -(-489218948_i32);
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = 37876_u16 as f64;
_6 = 52_i8 as f64;
_4 = '\u{f3255}';
RET = _5.1 as u128;
_6 = _5.1 as f64;
_4 = '\u{9d649}';
_7 = 9223372036854775807_isize;
_8 = (-17_i8) as u8;
_5.1 = (-1209349003_i32) + 1861443364_i32;
_5.1 = 87917241_i32 ^ 1526652244_i32;
_4 = '\u{d76e2}';
_8 = 117_u8;
_4 = '\u{4189b}';
RET = 41927277313298623167457198704393361288_u128 << _7;
_6 = RET as f64;
_5.1 = (-848158252_i32) * 127680458_i32;
_7 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_5.1 = 16160341132908984231_u64 as i32;
_6 = (-16754_i16) as f64;
RET = 317169769846452383917739169064218506089_u128 * 302056282750334647095203431853150284590_u128;
_8 = !101_u8;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = _8 as f64;
_6 = (-133135802254258935934262364068162891574_i128) as f64;
_4 = '\u{2abec}';
_8 = !196_u8;
_8 = !179_u8;
_8 = (-168792887645502231018798898699238740728_i128) as u8;
_7 = 13298954090965501704_usize as isize;
_8 = 907229993503352111_i64 as u8;
_5.1 = 2859577243178982226_i64 as i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = 322299490769627051252771286461739453391_u128 ^ 269719337051412988805101286385117821424_u128;
_7 = 111_isize << _8;
_4 = '\u{c0e3a}';
_4 = '\u{daf8a}';
_5.0 = core::ptr::addr_of_mut!(_5.3);
_5.1 = _8 as i32;
_7 = _5.1 as isize;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = !277437881710179596371742702892652286485_u128;
_5.1 = 1493027896_i32;
_11.1 = [93722097_u32,13899943_u32,1459842874_u32,3046965510_u32,981383583_u32,4253396209_u32,3950568162_u32];
Goto(bb3)
}
bb3 = {
_6 = 4_usize as f64;
_8 = !201_u8;
_11.1 = [919091326_u32,4269959389_u32,1842239608_u32,2816023404_u32,2935110727_u32,524867596_u32,864929380_u32];
_11.1 = [1361364866_u32,2971020841_u32,224325958_u32,1343701754_u32,3390971777_u32,3294597018_u32,277902066_u32];
_8 = 52_u8 - 116_u8;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_11.0 = &_6;
_7 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_11.1 = [1859008776_u32,3208038630_u32,1581424432_u32,508129541_u32,214643060_u32,3941652186_u32,1307454538_u32];
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = 5268410896445268582_u64 as f64;
_13.1 = 12539_i16 * 17466_i16;
_8 = !193_u8;
_12 = 3554300349_u32 as usize;
_1 = &_12;
_5.3 = core::ptr::addr_of_mut!(_13.1);
_10 = !_7;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_3 = Move(_1);
_17 = core::ptr::addr_of!(_4);
_11.1 = [3687014002_u32,1045845545_u32,3922164086_u32,2963404578_u32,1832900051_u32,278452404_u32,620043358_u32];
(*_17) = '\u{49cd2}';
(*_17) = '\u{b8b9c}';
Goto(bb4)
}
bb4 = {
(*_17) = '\u{eca70}';
_5.1 = (-87447955420071554337156203539997649622_i128) as i32;
_13.0 = &mut _11.0;
(*_17) = '\u{7c6ac}';
_18 = (*_17);
_10 = _7;
_5.0 = core::ptr::addr_of_mut!(_5.3);
(*_17) = _18;
_3 = &_12;
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
_8 = 35_u8 >> _7;
_13.2 = (*_3);
(*_17) = _18;
(*_17) = _18;
_20 = [(-122_i8),(-103_i8),(-116_i8),(-63_i8),96_i8,(-37_i8),(-127_i8),48_i8];
(*_17) = _18;
_12 = 49596_u16 as usize;
(*_17) = _18;
_13.2 = _12;
Goto(bb5)
}
bb5 = {
_22 = Move(_13.0);
_5.2 = Adt55::Variant0 { fld0: Move(_17),fld1: 16242369449924368833_u64,fld2: RET };
_17 = core::ptr::addr_of!(_4);
(*_17) = _18;
Goto(bb6)
}
bb6 = {
_20 = [15_i8,(-38_i8),76_i8,(-81_i8),(-66_i8),77_i8,82_i8,(-82_i8)];
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
(*_17) = _18;
_21 = false | true;
(*_17) = _18;
_18 = (*_17);
_28 = !_21;
(*_17) = _18;
_19 = Field::<u128>(Variant(_5.2, 0), 2) as f32;
(*_17) = _18;
_34 = Move(Field::<*const char>(Variant(_5.2, 0), 0));
_6 = _10 as f64;
_1 = &_13.2;
_7 = _10;
_18 = (*_17);
(*_17) = _18;
(*_17) = _18;
_15 = &_8;
(*_17) = _18;
_10 = Field::<u128>(Variant(_5.2, 0), 2) as isize;
Goto(bb7)
}
bb7 = {
place!(Field::<*const char>(Variant(_5.2, 0), 0)) = core::ptr::addr_of!((*_17));
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
(*_17) = _18;
(*_17) = _18;
_37 = core::ptr::addr_of!((*_17));
_27 = _7;
_1 = &_12;
(*_17) = _18;
Call(place!(Field::<u128>(Variant(_5.2, 0), 2)) = core::intrinsics::bswap(RET), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb9 = {
_7 = _5.1 as isize;
_36 = (*_17);
(*_17) = _18;
_6 = Field::<u128>(Variant(_5.2, 0), 2) as f64;
_43 = _6 + _6;
_42 = _19 + _19;
_44 = &mut _8;
_6 = _43 + _43;
_28 = _21;
_31 = core::ptr::addr_of_mut!((*_17));
(*_31) = _36;
(*_17) = _18;
(*_44) = (*_1) as u8;
Goto(bb10)
}
bb10 = {
(*_44) = 164_u8 * 26_u8;
(*_17) = _18;
_41 = [(*_1),(*_1),(*_1),(*_1),(*_1),(*_1),(*_1)];
_46.fld3 = core::ptr::addr_of_mut!(_46.fld1);
_46.fld2 = -_42;
(*_44) = 121_u8 | 249_u8;
_20 = [3_i8,68_i8,(-15_i8),(-67_i8),48_i8,37_i8,(-31_i8),105_i8];
_46.fld2 = _19 * _19;
_46.fld4 = _13.2 & (*_1);
(*_17) = _36;
_6 = _43;
(*_17) = _36;
(*_17) = _36;
_47 = 3449862816547569485_i64 as u32;
_34 = core::ptr::addr_of!((*_17));
(*_44) = !203_u8;
_24 = _43;
(*_44) = 233_u8 ^ 208_u8;
Goto(bb11)
}
bb11 = {
(*_17) = _36;
(*_17) = _36;
_35 = core::ptr::addr_of_mut!(_44);
_19 = _42 + _46.fld2;
_13.1 = (-32313_i16);
(*_44) = _46.fld4 as u8;
(*_17) = _36;
_25 = core::ptr::addr_of_mut!(_47);
_52 = -(-90_i8);
match _13.1 {
0 => bb9,
340282366920938463463374607431768179143 => bb13,
_ => bb12
}
}
bb12 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb13 = {
(*_25) = 2450895769_u32 + 1178755096_u32;
_32 = RET as f32;
_30 = core::ptr::addr_of_mut!(_5.3);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_34 = core::ptr::addr_of!((*_17));
(*_25) = 1950735064_u32;
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
RET = (*_44) as u128;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_54 = _27 << (*_1);
_41 = [(*_1),(*_1),(*_1),(*_1),(*_1),_46.fld4,(*_1)];
(*_44) = 236_u8;
(*_25) = _43 as u32;
(*_34) = _18;
_35 = core::ptr::addr_of_mut!((*_35));
(*_44) = 190_u8 << _52;
(*_44) = !148_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_17) = _36;
(*_44) = 91_u8 * 157_u8;
_7 = _54 * _54;
Goto(bb14)
}
bb14 = {
place!(Field::<u64>(Variant(_5.2, 0), 1)) = !7649284136009148794_u64;
_43 = _6 - _6;
_46.fld1 = (*_25) - (*_25);
_18 = (*_17);
_45 = &mut _7;
_2 = &mut _52;
(*_44) = 72_u8 - 224_u8;
(*_2) = _28 as i8;
_56 = _18;
(*_44) = 53433_u16 as u8;
(*_45) = (*_2) as isize;
_13.2 = (*_25) as usize;
_38 = _6 + _6;
(*_45) = _19 as isize;
(*_2) = !(-114_i8);
match _13.1 {
0 => bb1,
1 => bb10,
2 => bb12,
3 => bb15,
340282366920938463463374607431768179143 => bb17,
_ => bb16
}
}
bb15 = {
_7 = _5.1 as isize;
_36 = (*_17);
(*_17) = _18;
_6 = Field::<u128>(Variant(_5.2, 0), 2) as f64;
_43 = _6 + _6;
_42 = _19 + _19;
_44 = &mut _8;
_6 = _43 + _43;
_28 = _21;
_31 = core::ptr::addr_of_mut!((*_17));
(*_31) = _36;
(*_17) = _18;
(*_44) = (*_1) as u8;
Goto(bb10)
}
bb16 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb17 = {
_37 = core::ptr::addr_of!((*_17));
(*_45) = _54;
_46.fld1 = _5.1 as u32;
_46 = Adt28 { fld0: 28317_u16,fld1: (*_25),fld2: _19,fld3: Move(_25),fld4: (*_1) };
_46.fld0 = 22700_u16 ^ 43399_u16;
_60 = (*_44) as isize;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_37 = core::ptr::addr_of!((*_37));
_24 = _38;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_45) = _54 * _54;
place!(Field::<u128>(Variant(_5.2, 0), 2)) = RET + RET;
(*_2) = _46.fld0 as i8;
RET = _13.1 as u128;
_26 = &mut _3;
(*_26) = &_46.fld4;
place!(Field::<u64>(Variant(_5.2, 0), 1)) = !17102244462457300660_u64;
Goto(bb18)
}
bb18 = {
_17 = core::ptr::addr_of!((*_17));
_18 = (*_37);
(*_2) = 85_i8;
_25 = core::ptr::addr_of_mut!(_46.fld1);
_61 = (*_45);
_21 = _28 & _28;
_38 = -_24;
_6 = _42 as f64;
(*_44) = !140_u8;
(*_17) = _18;
(*_17) = _56;
match (*_2) {
0 => bb10,
1 => bb14,
2 => bb3,
3 => bb17,
85 => bb20,
_ => bb19
}
}
bb19 = {
(*_25) = 2450895769_u32 + 1178755096_u32;
_32 = RET as f32;
_30 = core::ptr::addr_of_mut!(_5.3);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_34 = core::ptr::addr_of!((*_17));
(*_25) = 1950735064_u32;
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
RET = (*_44) as u128;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_54 = _27 << (*_1);
_41 = [(*_1),(*_1),(*_1),(*_1),(*_1),_46.fld4,(*_1)];
(*_44) = 236_u8;
(*_25) = _43 as u32;
(*_34) = _18;
_35 = core::ptr::addr_of_mut!((*_35));
(*_44) = 190_u8 << _52;
(*_44) = !148_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_17) = _36;
(*_44) = 91_u8 * 157_u8;
_7 = _54 * _54;
Goto(bb14)
}
bb20 = {
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_25) = _47;
(*_25) = _47 * _47;
(*_26) = &(*_1);
(*_25) = _47 ^ _47;
_14 = &(*_2);
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_44) = 0_u8;
_40 = -_42;
_47 = (*_25) ^ (*_25);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_58 = core::ptr::addr_of_mut!((*_37));
(*_44) = 50_u8;
(*_37) = _18;
Goto(bb21)
}
bb21 = {
_42 = _46.fld2 - _46.fld2;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_25 = core::ptr::addr_of_mut!((*_25));
_46 = Adt28 { fld0: 56539_u16,fld1: _47,fld2: _42,fld3: Move(_25),fld4: (*_1) };
RET = (*_1) as u128;
_60 = (*_45) & (*_45);
_43 = _46.fld0 as f64;
(*_37) = _36;
(*_45) = _10 * _60;
(*_45) = _61;
(*_37) = _18;
_62 = _46.fld2 as isize;
(*_44) = 118_u8 >> (*_45);
(*_44) = !231_u8;
(*_2) = (-6_i8) + 126_i8;
match _46.fld0 {
0 => bb18,
1 => bb10,
2 => bb17,
56539 => bb22,
_ => bb13
}
}
bb22 = {
_13.2 = (*_1);
(*_26) = &_13.2;
(*_44) = 206_u8 & 110_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_45) = (*_37) as isize;
(*_26) = &(*_1);
_51 = &mut (*_44);
(*_2) = 82_i8;
(*_35) = Move(_51);
_5.1 = !809988388_i32;
_4 = _56;
(*_2) = -96_i8;
(*_45) = _54 << _62;
(*_26) = &_46.fld4;
_73 = !_5.1;
(*_37) = _56;
_70 = (*_45) << (*_45);
_30 = core::ptr::addr_of_mut!((*_30));
(*_45) = _61;
(*_45) = _60;
Goto(bb23)
}
bb23 = {
(*_37) = _56;
_56 = (*_37);
_28 = _46.fld0 >= _46.fld0;
(*_37) = _36;
_59 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_5.2, 0), 2)));
(*_45) = !_70;
_34 = core::ptr::addr_of!((*_37));
_13.1 = 14446_i16 << (*_45);
(*_45) = -_54;
(*_45) = _60 | _60;
(*_2) = (*_45) as i8;
(*_2) = 154813986953201688010496941789252999419_i128 as i8;
(*_37) = _56;
_5.3 = core::ptr::addr_of_mut!(_13.1);
_26 = &mut _1;
(*_2) = -(-66_i8);
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_26) = &_13.2;
_46.fld3 = core::ptr::addr_of_mut!(_46.fld1);
_68 = (*_37);
(*_2) = (-33_i8);
(*_2) = -71_i8;
_72 = (*_2) as u64;
(*_26) = &_46.fld4;
_36 = (*_37);
_12 = !_46.fld4;
_10 = (*_45);
(*_37) = _36;
_58 = Move(_31);
_63.1.0 = &_43;
match _46.fld0 {
0 => bb14,
1 => bb24,
56539 => bb26,
_ => bb25
}
}
bb24 = {
(*_25) = 2450895769_u32 + 1178755096_u32;
_32 = RET as f32;
_30 = core::ptr::addr_of_mut!(_5.3);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_34 = core::ptr::addr_of!((*_17));
(*_25) = 1950735064_u32;
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
RET = (*_44) as u128;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_54 = _27 << (*_1);
_41 = [(*_1),(*_1),(*_1),(*_1),(*_1),_46.fld4,(*_1)];
(*_44) = 236_u8;
(*_25) = _43 as u32;
(*_34) = _18;
_35 = core::ptr::addr_of_mut!((*_35));
(*_44) = 190_u8 << _52;
(*_44) = !148_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_17) = _36;
(*_44) = 91_u8 * 157_u8;
_7 = _54 * _54;
Goto(bb14)
}
bb25 = {
_5.1 = (-1944311965_i32) & 1475794740_i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = !293897226484798638041909357778124232313_u128;
_4 = '\u{283b1}';
RET = (-104_i8) as u128;
RET = 61082_u16 as u128;
_5.1 = -(-489218948_i32);
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = 37876_u16 as f64;
_6 = 52_i8 as f64;
_4 = '\u{f3255}';
RET = _5.1 as u128;
_6 = _5.1 as f64;
_4 = '\u{9d649}';
_7 = 9223372036854775807_isize;
_8 = (-17_i8) as u8;
_5.1 = (-1209349003_i32) + 1861443364_i32;
_5.1 = 87917241_i32 ^ 1526652244_i32;
_4 = '\u{d76e2}';
_8 = 117_u8;
_4 = '\u{4189b}';
RET = 41927277313298623167457198704393361288_u128 << _7;
_6 = RET as f64;
_5.1 = (-848158252_i32) * 127680458_i32;
_7 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb2)
}
bb26 = {
_13.0 = &mut _63.1.0;
(*_59) = RET + RET;
_23 = core::ptr::addr_of!(_13.0);
_69 = [3926367177934178759_i64,4997005387271238656_i64,(-1928747452706951317_i64)];
(*_45) = _10 & _60;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_13.1 = 1479_i16;
(*_59) = RET + RET;
(*_2) = 11_i8;
_71 = [(-8777067329115332459_i64),7625273932175900968_i64,7112264300231502217_i64];
(*_26) = &_13.2;
(*_45) = !_62;
place!(Field::<u128>(Variant(_5.2, 0), 2)) = _13.2 as u128;
(*_26) = &_12;
_55 = Move(_30);
(*_37) = _18;
_62 = (*_45) & (*_45);
(*_59) = RET | RET;
(*_45) = _13.1 as isize;
(*_59) = RET - RET;
_5.1 = 88_u8 as i32;
_37 = core::ptr::addr_of!(_53);
(*_37) = _4;
_75.1.0 = &_24;
_13.1 = (*_37) as i16;
match (*_2) {
0 => bb27,
1 => bb28,
2 => bb29,
11 => bb31,
_ => bb30
}
}
bb27 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb28 = {
_5.1 = 16160341132908984231_u64 as i32;
_6 = (-16754_i16) as f64;
RET = 317169769846452383917739169064218506089_u128 * 302056282750334647095203431853150284590_u128;
_8 = !101_u8;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = _8 as f64;
_6 = (-133135802254258935934262364068162891574_i128) as f64;
_4 = '\u{2abec}';
_8 = !196_u8;
_8 = !179_u8;
_8 = (-168792887645502231018798898699238740728_i128) as u8;
_7 = 13298954090965501704_usize as isize;
_8 = 907229993503352111_i64 as u8;
_5.1 = 2859577243178982226_i64 as i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = 322299490769627051252771286461739453391_u128 ^ 269719337051412988805101286385117821424_u128;
_7 = 111_isize << _8;
_4 = '\u{c0e3a}';
_4 = '\u{daf8a}';
_5.0 = core::ptr::addr_of_mut!(_5.3);
_5.1 = _8 as i32;
_7 = _5.1 as isize;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = !277437881710179596371742702892652286485_u128;
_5.1 = 1493027896_i32;
_11.1 = [93722097_u32,13899943_u32,1459842874_u32,3046965510_u32,981383583_u32,4253396209_u32,3950568162_u32];
Goto(bb3)
}
bb29 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb30 = {
_6 = 4_usize as f64;
_8 = !201_u8;
_11.1 = [919091326_u32,4269959389_u32,1842239608_u32,2816023404_u32,2935110727_u32,524867596_u32,864929380_u32];
_11.1 = [1361364866_u32,2971020841_u32,224325958_u32,1343701754_u32,3390971777_u32,3294597018_u32,277902066_u32];
_8 = 52_u8 - 116_u8;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_11.0 = &_6;
_7 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_11.1 = [1859008776_u32,3208038630_u32,1581424432_u32,508129541_u32,214643060_u32,3941652186_u32,1307454538_u32];
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = 5268410896445268582_u64 as f64;
_13.1 = 12539_i16 * 17466_i16;
_8 = !193_u8;
_12 = 3554300349_u32 as usize;
_1 = &_12;
_5.3 = core::ptr::addr_of_mut!(_13.1);
_10 = !_7;
_5.0 = core::ptr::addr_of_mut!(_5.3);
_3 = Move(_1);
_17 = core::ptr::addr_of!(_4);
_11.1 = [3687014002_u32,1045845545_u32,3922164086_u32,2963404578_u32,1832900051_u32,278452404_u32,620043358_u32];
(*_17) = '\u{49cd2}';
(*_17) = '\u{b8b9c}';
Goto(bb4)
}
bb31 = {
(*_59) = RET | RET;
(*_26) = &_46.fld4;
(*_26) = &_13.2;
_61 = _70 + _10;
_58 = core::ptr::addr_of_mut!((*_37));
_58 = core::ptr::addr_of_mut!((*_58));
(*_37) = _56;
(*_26) = &_12;
_13.0 = &mut _75.1.0;
_6 = _38 * _43;
_28 = !_21;
(*_2) = (-107_i8) | 103_i8;
(*_45) = !_61;
_85.1 = &mut (*_45);
_77 = core::ptr::addr_of_mut!(_56);
(*_77) = (*_37);
_30 = core::ptr::addr_of_mut!(_5.3);
(*_37) = (*_77);
(*_26) = &_13.2;
(*_37) = _36;
(*_2) = (-10_i8) | 125_i8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_26) = &_46.fld4;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_2) = -(-8_i8);
match _46.fld0 {
56539 => bb33,
_ => bb32
}
}
bb32 = {
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_25) = _47;
(*_25) = _47 * _47;
(*_26) = &(*_1);
(*_25) = _47 ^ _47;
_14 = &(*_2);
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_44) = 0_u8;
_40 = -_42;
_47 = (*_25) ^ (*_25);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_58 = core::ptr::addr_of_mut!((*_37));
(*_44) = 50_u8;
(*_37) = _18;
Goto(bb21)
}
bb33 = {
_42 = _19 + _46.fld2;
(*_59) = RET;
(*_77) = (*_37);
(*_30) = core::ptr::addr_of_mut!(_87);
(*_30) = core::ptr::addr_of_mut!(_87);
(*_26) = &_12;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_2) = 116_i8;
(*_59) = !RET;
_18 = (*_37);
_59 = core::ptr::addr_of!((*_59));
_55 = Move(_5.0);
_36 = (*_37);
(*_77) = (*_37);
(*_77) = (*_37);
Goto(bb34)
}
bb34 = {
_83 = [(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2),(*_2)];
_86 = Adt23::Variant2 { fld0: _46.fld0,fld1: (-169159117246103167190040144571642617547_i128) };
_45 = &mut _10;
(*_45) = _27 ^ _61;
(*_77) = (*_37);
_71 = [7057549842270007870_i64,(-1365366995962741870_i64),3076501508453815204_i64];
_21 = !_28;
(*_30) = core::ptr::addr_of_mut!(_87);
_91 = (*_37) as u8;
_64 = (*_45) >= (*_45);
_42 = _46.fld2;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_35) = &mut _91;
(*_59) = RET;
_41 = [_13.2,_13.2,_46.fld4,_46.fld4,_46.fld4,_12,_13.2];
(*_2) = 105_i8 | (-54_i8);
(*_59) = (*_44) as u128;
_71 = [(-8519593410536731856_i64),(-4354709669251372555_i64),(-3258830514563948448_i64)];
(*_45) = _61;
_78 = core::ptr::addr_of!((*_59));
match _46.fld0 {
0 => bb35,
1 => bb36,
56539 => bb38,
_ => bb37
}
}
bb35 = {
_7 = _5.1 as isize;
_36 = (*_17);
(*_17) = _18;
_6 = Field::<u128>(Variant(_5.2, 0), 2) as f64;
_43 = _6 + _6;
_42 = _19 + _19;
_44 = &mut _8;
_6 = _43 + _43;
_28 = _21;
_31 = core::ptr::addr_of_mut!((*_17));
(*_31) = _36;
(*_17) = _18;
(*_44) = (*_1) as u8;
Goto(bb10)
}
bb36 = {
place!(Field::<*const char>(Variant(_5.2, 0), 0)) = core::ptr::addr_of!((*_17));
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
(*_17) = _18;
(*_17) = _18;
_37 = core::ptr::addr_of!((*_17));
_27 = _7;
_1 = &_12;
(*_17) = _18;
Call(place!(Field::<u128>(Variant(_5.2, 0), 2)) = core::intrinsics::bswap(RET), ReturnTo(bb8), UnwindUnreachable())
}
bb37 = {
(*_59) = RET | RET;
(*_26) = &_46.fld4;
(*_26) = &_13.2;
_61 = _70 + _10;
_58 = core::ptr::addr_of_mut!((*_37));
_58 = core::ptr::addr_of_mut!((*_58));
(*_37) = _56;
(*_26) = &_12;
_13.0 = &mut _75.1.0;
_6 = _38 * _43;
_28 = !_21;
(*_2) = (-107_i8) | 103_i8;
(*_45) = !_61;
_85.1 = &mut (*_45);
_77 = core::ptr::addr_of_mut!(_56);
(*_77) = (*_37);
_30 = core::ptr::addr_of_mut!(_5.3);
(*_37) = (*_77);
(*_26) = &_13.2;
(*_37) = _36;
(*_2) = (-10_i8) | 125_i8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_26) = &_46.fld4;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_2) = -(-8_i8);
match _46.fld0 {
56539 => bb33,
_ => bb32
}
}
bb38 = {
_74 = &mut _77;
(*_44) = 189_u8 ^ 125_u8;
(*_78) = !RET;
_79 = Adt41::Variant1 { fld0: _64,fld1: Field::<u64>(Variant(_5.2, 0), 1),fld2: _6,fld3: (*_2),fld4: _47 };
_88 = _6;
(*_44) = 94_u8 ^ 150_u8;
(*_59) = RET - RET;
_96.2 = Adt55::Variant1 { fld0: Move((*_74)),fld1: Move(_46.fld3),fld2: (*_45),fld3: _46.fld0,fld4: _13.1 };
place!(Field::<isize>(Variant(_96.2, 1), 2)) = 3246006472581318125_i64 as isize;
_77 = core::ptr::addr_of_mut!(_97);
place!(Field::<*mut char>(Variant(_96.2, 1), 0)) = core::ptr::addr_of_mut!((*_77));
(*_59) = RET;
(*_44) = 118448778872277652438574902000194095068_i128 as u8;
(*_45) = _61;
_89 = !(*_59);
_96.0 = core::ptr::addr_of_mut!(_96.3);
(*_77) = (*_37);
(*_26) = &_46.fld4;
_72 = !Field::<u64>(Variant(_5.2, 0), 1);
_32 = _46.fld2 + _42;
(*_37) = (*_77);
match Field::<u16>(Variant(_96.2, 1), 3) {
0 => bb14,
1 => bb24,
2 => bb30,
3 => bb15,
4 => bb39,
5 => bb40,
56539 => bb42,
_ => bb41
}
}
bb39 = {
_22 = Move(_13.0);
_5.2 = Adt55::Variant0 { fld0: Move(_17),fld1: 16242369449924368833_u64,fld2: RET };
_17 = core::ptr::addr_of!(_4);
(*_17) = _18;
Goto(bb6)
}
bb40 = {
place!(Field::<*const char>(Variant(_5.2, 0), 0)) = core::ptr::addr_of!((*_17));
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
(*_17) = _18;
(*_17) = _18;
_37 = core::ptr::addr_of!((*_17));
_27 = _7;
_1 = &_12;
(*_17) = _18;
Call(place!(Field::<u128>(Variant(_5.2, 0), 2)) = core::intrinsics::bswap(RET), ReturnTo(bb8), UnwindUnreachable())
}
bb41 = {
_17 = core::ptr::addr_of!((*_17));
_18 = (*_37);
(*_2) = 85_i8;
_25 = core::ptr::addr_of_mut!(_46.fld1);
_61 = (*_45);
_21 = _28 & _28;
_38 = -_24;
_6 = _42 as f64;
(*_44) = !140_u8;
(*_17) = _18;
(*_17) = _56;
match (*_2) {
0 => bb10,
1 => bb14,
2 => bb3,
3 => bb17,
85 => bb20,
_ => bb19
}
}
bb42 = {
(*_44) = Field::<bool>(Variant(_79, 1), 0) as u8;
(*_37) = (*_77);
_96 = (Move(_55), _73, Move(_5.2), Move((*_30)));
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_77) = _53;
(*_45) = _61 ^ _61;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_59) = RET;
_87 = _13.1;
_31 = core::ptr::addr_of_mut!((*_37));
(*_44) = 9_u8 * 15_u8;
_48 = (*_31);
(*_44) = !10_u8;
_5 = (Move(_96.0), _96.1, Move(_96.2), Move(_96.3));
_65 = (*_2) | (*_2);
_27 = (*_45) + (*_45);
(*_45) = _27 | _27;
(*_30) = core::ptr::addr_of_mut!(_13.1);
match _46.fld0 {
0 => bb34,
1 => bb7,
2 => bb21,
3 => bb16,
4 => bb43,
5 => bb44,
56539 => bb46,
_ => bb45
}
}
bb43 = {
_8 = _13.1 as u8;
_39 = [_28,_28];
Goto(bb9)
}
bb44 = {
_37 = core::ptr::addr_of!((*_17));
(*_45) = _54;
_46.fld1 = _5.1 as u32;
_46 = Adt28 { fld0: 28317_u16,fld1: (*_25),fld2: _19,fld3: Move(_25),fld4: (*_1) };
_46.fld0 = 22700_u16 ^ 43399_u16;
_60 = (*_44) as isize;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_37 = core::ptr::addr_of!((*_37));
_24 = _38;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_45) = _54 * _54;
place!(Field::<u128>(Variant(_5.2, 0), 2)) = RET + RET;
(*_2) = _46.fld0 as i8;
RET = _13.1 as u128;
_26 = &mut _3;
(*_26) = &_46.fld4;
place!(Field::<u64>(Variant(_5.2, 0), 1)) = !17102244462457300660_u64;
Goto(bb18)
}
bb45 = {
_22 = Move(_13.0);
_5.2 = Adt55::Variant0 { fld0: Move(_17),fld1: 16242369449924368833_u64,fld2: RET };
_17 = core::ptr::addr_of!(_4);
(*_17) = _18;
Goto(bb6)
}
bb46 = {
_56 = _4;
_46.fld2 = _32 * _42;
_13.3 = [_18,(*_37),(*_31),(*_77)];
_12 = 6926049080348010982_i64 as usize;
(*_26) = &_13.2;
(*_31) = (*_77);
(*_30) = core::ptr::addr_of_mut!(_87);
(*_77) = (*_37);
(*_44) = 127_u8 * 161_u8;
place!(Field::<i128>(Variant(_86, 2), 1)) = 89248636563618127238127470806276009679_i128;
(*_44) = 7_u8 ^ 179_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_44) = 179_u8 | 56_u8;
(*_37) = (*_77);
match Field::<u16>(Variant(_86, 2), 0) {
0 => bb18,
1 => bb47,
2 => bb48,
56539 => bb50,
_ => bb49
}
}
bb47 = {
(*_25) = 2450895769_u32 + 1178755096_u32;
_32 = RET as f32;
_30 = core::ptr::addr_of_mut!(_5.3);
(*_30) = core::ptr::addr_of_mut!(_13.1);
_34 = core::ptr::addr_of!((*_17));
(*_25) = 1950735064_u32;
_5.3 = core::ptr::addr_of_mut!(_13.1);
(*_30) = core::ptr::addr_of_mut!(_13.1);
RET = (*_44) as u128;
(*_30) = core::ptr::addr_of_mut!(_13.1);
_54 = _27 << (*_1);
_41 = [(*_1),(*_1),(*_1),(*_1),(*_1),_46.fld4,(*_1)];
(*_44) = 236_u8;
(*_25) = _43 as u32;
(*_34) = _18;
_35 = core::ptr::addr_of_mut!((*_35));
(*_44) = 190_u8 << _52;
(*_44) = !148_u8;
(*_30) = core::ptr::addr_of_mut!(_13.1);
(*_17) = _36;
(*_44) = 91_u8 * 157_u8;
_7 = _54 * _54;
Goto(bb14)
}
bb48 = {
place!(Field::<*const char>(Variant(_5.2, 0), 0)) = core::ptr::addr_of!((*_17));
_13.3 = [(*_17),(*_17),(*_17),(*_17)];
(*_17) = _18;
(*_17) = _18;
_37 = core::ptr::addr_of!((*_17));
_27 = _7;
_1 = &_12;
(*_17) = _18;
Call(place!(Field::<u128>(Variant(_5.2, 0), 2)) = core::intrinsics::bswap(RET), ReturnTo(bb8), UnwindUnreachable())
}
bb49 = {
_5.1 = (-1944311965_i32) & 1475794740_i32;
_5.0 = core::ptr::addr_of_mut!(_5.3);
RET = !293897226484798638041909357778124232313_u128;
_4 = '\u{283b1}';
RET = (-104_i8) as u128;
RET = 61082_u16 as u128;
_5.1 = -(-489218948_i32);
_5.0 = core::ptr::addr_of_mut!(_5.3);
_6 = 37876_u16 as f64;
_6 = 52_i8 as f64;
_4 = '\u{f3255}';
RET = _5.1 as u128;
_6 = _5.1 as f64;
_4 = '\u{9d649}';
_7 = 9223372036854775807_isize;
_8 = (-17_i8) as u8;
_5.1 = (-1209349003_i32) + 1861443364_i32;
_5.1 = 87917241_i32 ^ 1526652244_i32;
_4 = '\u{d76e2}';
_8 = 117_u8;
_4 = '\u{4189b}';
RET = 41927277313298623167457198704393361288_u128 << _7;
_6 = RET as f64;
_5.1 = (-848158252_i32) * 127680458_i32;
_7 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb2)
}
bb50 = {
_81.0 = (*_44) as u16;
RET = _89 + _89;
_90 = (*_77);
_2 = &mut place!(Field::<i8>(Variant(_79, 1), 3));
(*_44) = 147_u8;
(*_30) = core::ptr::addr_of_mut!(_87);
(*_45) = _62 - _27;
(*_45) = -_27;
(*_26) = &_12;
(*_26) = &_13.2;
_55 = core::ptr::addr_of_mut!((*_30));
(*_44) = 199_u8 ^ 51_u8;
(*_45) = _27;
(*_45) = _27 >> _46.fld0;
_81.2.1 = _89 - RET;
(*_77) = _53;
_88 = _6 + _6;
_28 = _64;
_96.2 = Move(_5.2);
(*_44) = Field::<i128>(Variant(_86, 2), 1) as u8;
Goto(bb51)
}
bb51 = {
Call(_113 = dump_var(Move(_36), Move(_73), Move(_27), Move(_10)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_113 = dump_var(Move(_60), Move(_48), Move(_54), Move(_90)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_113 = dump_var(Move(_52), Move(_47), Move(_20), Move(_41)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_113 = dump_var(Move(_53), Move(_18), Move(_71), Move(_8)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_113 = dump_var(Move(_21), Move(_97), _114, _114), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: *mut *mut i16,mut _2: char,mut _3: char,mut _4: char,mut _5: i32) -> *mut i16 {
mir! {
type RET = *mut i16;
let _6: isize;
let _7: i8;
let _8: isize;
let _9: *const &'static mut u8;
let _10: [u32; 7];
let _11: *const u128;
let _12: *mut char;
let _13: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _14: *mut [u32; 7];
let _15: [u8; 7];
let _16: Adt43;
let _17: &'static mut isize;
let _18: *const &'static bool;
let _19: i128;
let _20: isize;
let _21: Adt55;
let _22: f32;
let _23: bool;
let _24: u128;
let _25: &'static mut &'static usize;
let _26: bool;
let _27: Adt55;
let _28: f32;
let _29: *mut u32;
let _30: char;
let _31: isize;
let _32: bool;
let _33: f64;
let _34: u32;
let _35: *const u128;
let _36: u64;
let _37: [u8; 7];
let _38: &'static mut *const i16;
let _39: (&'static mut &'static f64, i16, usize, [char; 4]);
let _40: (*mut &'static f64, &'static mut isize);
let _41: [i64; 3];
let _42: *mut i16;
let _43: &'static mut isize;
let _44: u128;
let _45: &'static mut &'static f64;
let _46: Adt55;
let _47: bool;
let _48: &'static mut isize;
let _49: isize;
let _50: u128;
let _51: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _52: Adt28;
let _53: i16;
let _54: u64;
let _55: i16;
let _56: char;
let _57: [i8; 8];
let _58: isize;
let _59: *mut char;
let _60: Adt50;
let _61: &'static f64;
let _62: &'static i8;
let _63: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _64: [bool; 2];
let _65: f64;
let _66: Adt50;
let _67: isize;
let _68: f32;
let _69: Adt43;
let _70: &'static usize;
let _71: &'static mut u8;
let _72: *const char;
let _73: *mut *mut &'static f64;
let _74: *mut *mut &'static f64;
let _75: isize;
let _76: Adt28;
let _77: &'static f32;
let _78: bool;
let _79: u32;
let _80: &'static mut &'static f64;
let _81: *mut char;
let _82: f64;
let _83: *const char;
let _84: i64;
let _85: f64;
let _86: isize;
let _87: &'static i8;
let _88: *mut &'static mut u8;
let _89: ();
let _90: ();
{
_4 = _2;
_1 = core::ptr::addr_of_mut!(RET);
_4 = _2;
_4 = _3;
_4 = _2;
_6 = 15777452927575301805_u64 as isize;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = _4;
_5 = (-1261743049_i32) * 718273532_i32;
_1 = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_1));
Call(_3 = fn3(_6, _4, Move(_1), _2, _2, _5, _2, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
Goto(bb2)
}
bb2 = {
_6 = _8 ^ _8;
_6 = -_8;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = _8;
_12 = core::ptr::addr_of_mut!(_2);
(*_12) = _4;
(*_12) = _4;
_8 = _6 + _6;
_3 = (*_12);
_5 = !(-1694154169_i32);
_6 = _8 - _8;
_10 = [4146271869_u32,320517727_u32,1333883338_u32,337601195_u32,2355789052_u32,1997212937_u32,902674184_u32];
_13.2.1 = 243402504226303969623843524992417658051_u128 * 296579316464576967183825253997513512593_u128;
_5 = !(-349562868_i32);
_13.2.0 = Adt23::Variant2 { fld0: 12876_u16,fld1: 90107934117188065837313355771822594460_i128 };
(*_12) = _4;
_10 = [3898956438_u32,125256147_u32,3950468781_u32,147893687_u32,3835759742_u32,3515801477_u32,2299224033_u32];
(*_12) = _4;
_14 = core::ptr::addr_of_mut!(_10);
(*_14) = [2899211422_u32,1279739008_u32,2322114809_u32,2017990713_u32,98600023_u32,3930083878_u32,4293025349_u32];
_13.2.0 = Adt23::Variant1 { fld0: _6 };
(*_12) = _3;
(*_12) = _3;
(*_12) = _4;
match _7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
116 => bb10,
_ => bb9
}
}
bb3 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
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
(*_14) = [2739776147_u32,4222780328_u32,519230747_u32,1589021063_u32,3461930954_u32,2143133505_u32,902420215_u32];
(*_14) = [173585283_u32,1022536633_u32,293418607_u32,3745189610_u32,3404067510_u32,503288006_u32,1247900686_u32];
(*_14) = [2509296551_u32,874199952_u32,1324480649_u32,821155282_u32,3538009023_u32,967132247_u32,1627270076_u32];
(*_12) = _4;
(*_14) = [2746301391_u32,4130264687_u32,3018222579_u32,2403696476_u32,292330260_u32,1403020809_u32,3310950973_u32];
match _7 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb9,
6 => bb11,
116 => bb13,
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
(*_12) = _3;
(*_12) = _3;
match _7 {
0 => bb7,
1 => bb14,
2 => bb15,
116 => bb17,
_ => bb16
}
}
bb14 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
Goto(bb2)
}
bb17 = {
_13.2.0 = Adt23::Variant1 { fld0: _6 };
(*_12) = _4;
_10 = [2444981497_u32,603659319_u32,1417931787_u32,716463057_u32,2436222463_u32,2820402124_u32,3561492970_u32];
_4 = (*_12);
(*_14) = [131051902_u32,2502836456_u32,1664566942_u32,3312953357_u32,555644641_u32,176460448_u32,1551140800_u32];
(*_12) = _4;
match _7 {
0 => bb7,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
116 => bb24,
_ => bb23
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
Goto(bb2)
}
bb21 = {
(*_12) = _3;
(*_12) = _3;
match _7 {
0 => bb7,
1 => bb14,
2 => bb15,
116 => bb17,
_ => bb16
}
}
bb22 = {
_6 = _8 ^ _8;
_6 = -_8;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = _8;
_12 = core::ptr::addr_of_mut!(_2);
(*_12) = _4;
(*_12) = _4;
_8 = _6 + _6;
_3 = (*_12);
_5 = !(-1694154169_i32);
_6 = _8 - _8;
_10 = [4146271869_u32,320517727_u32,1333883338_u32,337601195_u32,2355789052_u32,1997212937_u32,902674184_u32];
_13.2.1 = 243402504226303969623843524992417658051_u128 * 296579316464576967183825253997513512593_u128;
_5 = !(-349562868_i32);
_13.2.0 = Adt23::Variant2 { fld0: 12876_u16,fld1: 90107934117188065837313355771822594460_i128 };
(*_12) = _4;
_10 = [3898956438_u32,125256147_u32,3950468781_u32,147893687_u32,3835759742_u32,3515801477_u32,2299224033_u32];
(*_12) = _4;
_14 = core::ptr::addr_of_mut!(_10);
(*_14) = [2899211422_u32,1279739008_u32,2322114809_u32,2017990713_u32,98600023_u32,3930083878_u32,4293025349_u32];
_13.2.0 = Adt23::Variant1 { fld0: _6 };
(*_12) = _3;
(*_12) = _3;
(*_12) = _4;
match _7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
116 => bb10,
_ => bb9
}
}
bb23 = {
Return()
}
bb24 = {
_3 = (*_12);
_13.1 = &mut _7;
(*_12) = _4;
(*_12) = _3;
_17 = &mut _8;
(*_14) = [2070953271_u32,755778519_u32,1796408888_u32,2527748780_u32,3090305740_u32,846722700_u32,2707915610_u32];
_17 = &mut place!(Field::<isize>(Variant(_13.2.0, 1), 0));
_6 = -(*_17);
_20 = _6;
_6 = (*_12) as isize;
(*_14) = [1867701216_u32,3510537471_u32,308196398_u32,890034713_u32,1989858628_u32,4046348119_u32,4273318493_u32];
(*_12) = _4;
(*_12) = _3;
(*_14) = [2711144253_u32,2734855194_u32,3551186832_u32,792801884_u32,2961491037_u32,1121403849_u32,964953533_u32];
(*_12) = _3;
(*_17) = _6;
Goto(bb25)
}
bb25 = {
(*_12) = _4;
(*_12) = _3;
Call(_20 = core::intrinsics::bswap((*_17)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_14) = [544112637_u32,741725311_u32,2145179406_u32,3216169545_u32,3351880597_u32,617713537_u32,3548446227_u32];
(*_12) = _4;
(*_14) = [1924216931_u32,4211956392_u32,1966522753_u32,801752505_u32,1437555460_u32,4023981656_u32,3837149040_u32];
(*_17) = !_20;
(*_12) = _4;
(*_17) = _20;
(*_17) = _20;
(*_17) = _20;
(*_17) = _6 & _6;
(*_12) = _4;
(*_12) = _3;
_22 = 8_i8 as f32;
(*_17) = 999713875_u32 as isize;
(*_12) = _3;
_19 = !158381596793103659003633508593224790225_i128;
_17 = &mut _20;
(*_17) = _6 ^ _6;
Goto(bb27)
}
bb27 = {
(*_14) = [1063605353_u32,1821390566_u32,2481377946_u32,2529647806_u32,634857723_u32,2773120494_u32,1732820952_u32];
_2 = _4;
(*_14) = [2573066208_u32,4255544453_u32,4258181866_u32,1514440543_u32,3280764554_u32,1942687230_u32,1393756895_u32];
(*_17) = 14921_i16 as isize;
_17 = &mut _6;
(*_12) = _3;
(*_12) = _3;
(*_12) = _4;
_4 = (*_12);
(*_12) = _3;
(*_14) = [703223193_u32,399725870_u32,3067281950_u32,2966433118_u32,2549884061_u32,2147685271_u32,829287449_u32];
(*_12) = _4;
_4 = (*_12);
(*_17) = 26_isize * (-9223372036854775808_isize);
(*_14) = [87564825_u32,3649196915_u32,390306564_u32,1524124489_u32,3138621903_u32,1385134583_u32,3404576017_u32];
(*_12) = _3;
_26 = (*_17) <= (*_17);
Goto(bb28)
}
bb28 = {
_5 = -465158092_i32;
(*_14) = [3434791396_u32,929118940_u32,3003587393_u32,2009142480_u32,2152882709_u32,380055200_u32,735162822_u32];
(*_17) = -9223372036854775807_isize;
(*_17) = -9223372036854775807_isize;
(*_14) = [3970981954_u32,2851411023_u32,1794190691_u32,855940933_u32,3455025023_u32,1051426117_u32,2585375315_u32];
(*_14) = [536408300_u32,218783630_u32,249422250_u32,1813988851_u32,391594420_u32,3862534459_u32,609920377_u32];
(*_12) = _3;
_14 = core::ptr::addr_of_mut!((*_14));
(*_14) = [546307203_u32,3213865340_u32,1458767706_u32,619516314_u32,3185896760_u32,1081285624_u32,2553039008_u32];
_15 = [21_u8,75_u8,118_u8,132_u8,61_u8,203_u8,50_u8];
_31 = _22 as isize;
(*_12) = _3;
(*_14) = [4172158114_u32,1099465025_u32,2710774752_u32,1314491024_u32,2673821978_u32,2342419382_u32,3167835696_u32];
(*_12) = _3;
_4 = (*_12);
(*_17) = _31;
_14 = core::ptr::addr_of_mut!((*_14));
_28 = _22 - _22;
_11 = core::ptr::addr_of!(_24);
(*_12) = _4;
(*_12) = _4;
(*_14) = [2932981077_u32,3026227785_u32,3083258454_u32,1973872681_u32,798502137_u32,2726794839_u32,517549539_u32];
Goto(bb29)
}
bb29 = {
_29 = core::ptr::addr_of_mut!(_34);
(*_14) = [1338962701_u32,3295013891_u32,3577845479_u32,2051771312_u32,20781556_u32,3607288671_u32,819531305_u32];
(*_29) = 4097137682_u32 ^ 3036096106_u32;
(*_11) = !17882977860392991348837729652007563662_u128;
_3 = (*_12);
(*_11) = 339574609721171887512558589487852519407_u128 - 163956111876358222729565759655028644955_u128;
_10 = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
Call((*_17) = core::intrinsics::transmute(_31), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*_12) = _3;
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_11) = 13227215290222447049051968165487898465_u128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_12) = _3;
(*_29) = 2175752495_u32 - 243762288_u32;
(*_1) = core::ptr::addr_of_mut!(_39.1);
(*RET) = 66_i8 as i16;
match (*_11) {
0 => bb1,
1 => bb2,
2 => bb5,
13227215290222447049051968165487898465 => bb32,
_ => bb31
}
}
bb31 = {
_3 = _2;
_6 = (-2_isize) & 8_isize;
_1 = core::ptr::addr_of_mut!(RET);
_8 = 50159_u16 as isize;
_3 = _4;
_2 = _4;
_3 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_6 = (-121_i8) as isize;
_6 = !_8;
_8 = -_6;
_7 = 116_i8;
_10 = [2447819062_u32,3275799740_u32,652641360_u32,3462912764_u32,2121145484_u32,3598995095_u32,1645307415_u32];
_3 = _2;
_6 = _8 ^ _8;
_10 = [2694324571_u32,3958225841_u32,1229029485_u32,3677814950_u32,1300895685_u32,1736819271_u32,292777322_u32];
Goto(bb2)
}
bb32 = {
(*_11) = !290397202935046106636396195387173546063_u128;
(*_11) = 222720098691521881329104249699250861225_u128 - 315915660860754738042031352243479520417_u128;
(*_29) = 3687805623_u32 + 3874666753_u32;
(*RET) = (*_17) as i16;
_27 = Adt55::Variant1 { fld0: Move(_12),fld1: Move(_29),fld2: (*_17),fld3: 31106_u16,fld4: (*RET) };
_12 = core::ptr::addr_of_mut!(_3);
Goto(bb33)
}
bb33 = {
(*_12) = _2;
(*RET) = Field::<i16>(Variant(_27, 1), 4) ^ Field::<i16>(Variant(_27, 1), 4);
_12 = core::ptr::addr_of_mut!((*_12));
Goto(bb34)
}
bb34 = {
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_12) = _2;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
(*_12) = _4;
(*_12) = _4;
(*_17) = -_31;
(*RET) = !Field::<i16>(Variant(_27, 1), 4);
_39.3 = [(*_12),(*_12),_2,(*_12)];
(*RET) = _34 as i16;
_15 = [155_u8,131_u8,66_u8,172_u8,47_u8,171_u8,24_u8];
(*_17) = Field::<isize>(Variant(_27, 1), 2) >> (*_11);
(*_17) = -Field::<isize>(Variant(_27, 1), 2);
(*_11) = !90154693768229257757022579625809189802_u128;
(*_11) = 248060798078703681714271380215463876268_u128;
(*RET) = 160_u8 as i16;
(*_11) = _28 as u128;
_32 = (*_11) >= (*_11);
(*_17) = _31;
(*_1) = core::ptr::addr_of_mut!(_39.1);
(*RET) = Field::<i16>(Variant(_27, 1), 4) ^ Field::<i16>(Variant(_27, 1), 4);
(*_12) = _2;
(*RET) = -Field::<i16>(Variant(_27, 1), 4);
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
(*_12) = _4;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
_22 = _28;
(*_12) = _2;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb35)
}
bb35 = {
(*_1) = core::ptr::addr_of_mut!((*RET));
place!(Field::<i16>(Variant(_27, 1), 4)) = _22 as i16;
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_11) = 254686761069397552923726966200168144891_u128;
(*_17) = !Field::<isize>(Variant(_27, 1), 2);
(*RET) = (*_17) as i16;
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_17) = !Field::<isize>(Variant(_27, 1), 2);
_44 = !(*_11);
_11 = core::ptr::addr_of!((*_11));
_41 = [(-7308193323001696346_i64),(-5385091591013990346_i64),(-3663742788568593754_i64)];
(*_12) = _4;
(*_17) = Field::<isize>(Variant(_27, 1), 2) & Field::<isize>(Variant(_27, 1), 2);
_19 = 6502274724334902864384412917031241523_i128 << (*_17);
(*_1) = core::ptr::addr_of_mut!((*RET));
(*RET) = Field::<i16>(Variant(_27, 1), 4);
(*_11) = _44 ^ _44;
Goto(bb36)
}
bb36 = {
_39.3 = [(*_12),(*_12),(*_12),(*_12)];
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
(*RET) = Field::<i16>(Variant(_27, 1), 4);
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_17) = Field::<isize>(Variant(_27, 1), 2) * Field::<isize>(Variant(_27, 1), 2);
_11 = core::ptr::addr_of!((*_11));
(*_1) = core::ptr::addr_of_mut!((*RET));
_12 = core::ptr::addr_of_mut!((*_12));
(*_11) = _44;
_33 = 59781_u16 as f64;
(*RET) = _34 as i16;
_19 = (*_11) as i128;
(*RET) = Field::<i16>(Variant(_27, 1), 4) | Field::<i16>(Variant(_27, 1), 4);
(*_11) = !_44;
_12 = core::ptr::addr_of_mut!((*_12));
_39.3 = [(*_12),(*_12),_3,(*_12)];
(*_17) = Field::<isize>(Variant(_27, 1), 2) >> (*RET);
(*_1) = core::ptr::addr_of_mut!((*RET));
Call(_36 = core::intrinsics::bswap(17662865023545898665_u64), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
(*_11) = !_44;
(*_17) = Field::<isize>(Variant(_27, 1), 2) + Field::<isize>(Variant(_27, 1), 2);
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
_53 = !(*RET);
(*_17) = Field::<isize>(Variant(_27, 1), 2);
(*_1) = core::ptr::addr_of_mut!(_39.1);
(*RET) = !_53;
_21 = Adt55::Variant1 { fld0: Move(Field::<*mut char>(Variant(_27, 1), 0)),fld1: Move(Field::<*mut u32>(Variant(_27, 1), 1)),fld2: (*_17),fld3: 9259_u16,fld4: (*RET) };
(*RET) = Field::<i16>(Variant(_27, 1), 4);
_51.3 = Adt41::Variant1 { fld0: _32,fld1: 11623085317377826114_u64,fld2: _33,fld3: (-52_i8),fld4: _34 };
Goto(bb38)
}
bb38 = {
_33 = Field::<f64>(Variant(_51.3, 1), 2) - Field::<f64>(Variant(_51.3, 1), 2);
(*_12) = _4;
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_21, 1), 4)));
_47 = !Field::<bool>(Variant(_51.3, 1), 0);
(*_11) = _44;
_52.fld4 = !552356893792543483_usize;
_41 = [(-5562569133866792689_i64),8159933925033538433_i64,5673421330139599613_i64];
_24 = !_44;
_52.fld3 = Move(Field::<*mut u32>(Variant(_21, 1), 1));
_1 = core::ptr::addr_of_mut!((*_1));
(*_11) = _34 as u128;
_61 = &_33;
(*_17) = (*RET) as isize;
_34 = Field::<u32>(Variant(_51.3, 1), 4);
(*RET) = Field::<i16>(Variant(_27, 1), 4) << (*_11);
_51.1.1 = [_34,_34,_34,_34,_34,_34,_34];
(*RET) = _39.1;
place!(Field::<*mut u32>(Variant(_27, 1), 1)) = core::ptr::addr_of_mut!(_34);
Goto(bb39)
}
bb39 = {
_35 = core::ptr::addr_of!((*_11));
(*_17) = _31;
_63.2 = &(*_61);
_51.1 = (Move(_61), (*_14));
_48 = &mut (*_17);
_28 = _22 * _22;
_59 = core::ptr::addr_of_mut!((*_12));
(*_11) = _44;
(*_11) = _44 >> _53;
Goto(bb40)
}
bb40 = {
_51.2 = Move(_63.2);
(*_48) = -_31;
(*_59) = _4;
(*_14) = [Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4),_34,_34,Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4)];
_52 = Adt28 { fld0: 24326_u16,fld1: Field::<u32>(Variant(_51.3, 1), 4),fld2: _22,fld3: Move(Field::<*mut u32>(Variant(_27, 1), 1)),fld4: 16629197636353453625_usize };
place!(Field::<u32>(Variant(_51.3, 1), 4)) = _34 & _52.fld1;
_64 = [_32,Field::<bool>(Variant(_51.3, 1), 0)];
_51.1.1 = [Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4),_34,_34,Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4),Field::<u32>(Variant(_51.3, 1), 4)];
(*_11) = _44;
_57 = [(-61_i8),(-50_i8),(-39_i8),23_i8,(-45_i8),(-10_i8),(-43_i8),50_i8];
(*RET) = _33 as i16;
Goto(bb41)
}
bb41 = {
_66.fld0 = _39.3;
(*RET) = _22 as i16;
(*_48) = _31 >> (*RET);
(*_1) = core::ptr::addr_of_mut!((*RET));
_65 = _33 - Field::<f64>(Variant(_51.3, 1), 2);
_26 = _65 >= _65;
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_59) = _4;
_54 = 2876693739612264844_u64 ^ 4435869450111084023_u64;
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_27, 1), 4)));
_3 = _4;
match _52.fld4 {
16629197636353453625 => bb42,
_ => bb37
}
}
bb42 = {
_35 = core::ptr::addr_of!((*_11));
_21 = Adt55::Variant1 { fld0: Move(_59),fld1: Move(_52.fld3),fld2: (*_48),fld3: _52.fld0,fld4: (*RET) };
_40.1 = &mut _31;
_60.fld1 = [_5];
(*RET) = -_39.1;
RET = core::ptr::addr_of_mut!((*RET));
(*_48) = Field::<isize>(Variant(_27, 1), 2) >> _52.fld0;
(*_12) = _2;
Goto(bb43)
}
bb43 = {
(*_48) = !Field::<isize>(Variant(_21, 1), 2);
(*_35) = _44 << _39.1;
(*_48) = Field::<isize>(Variant(_27, 1), 2);
_61 = &_33;
_76.fld3 = core::ptr::addr_of_mut!(_52.fld1);
(*_48) = -Field::<isize>(Variant(_27, 1), 2);
(*_14) = _51.1.1;
_45 = &mut _61;
_39 = (Move(_45), (*RET), _52.fld4, _66.fld0);
_29 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_51.3, 1), 4)));
(*_11) = _44 & _44;
_16 = Adt43::Variant2 { fld0: 131_u8,fld1: _39.2,fld2: Move((*_1)) };
_65 = _33 - Field::<f64>(Variant(_51.3, 1), 2);
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
_48 = Move(_40.1);
_10 = _51.1.1;
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_27, 1), 4)));
(*_14) = _51.1.1;
(*_29) = !_52.fld1;
_3 = _2;
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = _4;
(*_1) = core::ptr::addr_of_mut!((*RET));
_67 = Field::<isize>(Variant(_27, 1), 2);
(*_29) = _52.fld1;
_10 = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_12) = _2;
_56 = (*_12);
_49 = Field::<isize>(Variant(_21, 1), 2) & Field::<isize>(Variant(_27, 1), 2);
(*_12) = _56;
match Field::<u16>(Variant(_21, 1), 3) {
0 => bb41,
1 => bb17,
2 => bb3,
3 => bb31,
4 => bb5,
5 => bb14,
6 => bb34,
24326 => bb44,
_ => bb8
}
}
bb44 = {
_75 = _49 >> Field::<usize>(Variant(_16, 2), 1);
_3 = _2;
_52.fld0 = Field::<u16>(Variant(_21, 1), 3);
_26 = _47;
(*RET) = _54 as i16;
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_11) = _44;
(*_11) = !_44;
_76.fld4 = Field::<usize>(Variant(_16, 2), 1) % _52.fld4;
(*_12) = _56;
(*_12) = _56;
(*RET) = Field::<i16>(Variant(_21, 1), 4) ^ Field::<i16>(Variant(_21, 1), 4);
_65 = Field::<f64>(Variant(_51.3, 1), 2);
_4 = (*_12);
(*_1) = Move(Field::<*mut i16>(Variant(_16, 2), 2));
(*_14) = [(*_29),Field::<u32>(Variant(_51.3, 1), 4),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_14) = _51.1.1;
_72 = core::ptr::addr_of!((*_12));
_59 = core::ptr::addr_of_mut!((*_72));
Goto(bb45)
}
bb45 = {
_52.fld3 = core::ptr::addr_of_mut!((*_29));
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_21, 1), 4)));
(*_72) = _4;
_40.1 = &mut _67;
(*_12) = _56;
_63.0 = Adt41::Variant1 { fld0: Field::<bool>(Variant(_51.3, 1), 0),fld1: _54,fld2: _33,fld3: (-27_i8),fld4: (*_29) };
(*_14) = [(*_29),(*_29),(*_29),(*_29),Field::<u32>(Variant(_63.0, 1), 4),(*_29),(*_29)];
_37 = [35_u8,32_u8,255_u8,127_u8,7_u8,6_u8,1_u8];
match Field::<u16>(Variant(_21, 1), 3) {
0 => bb34,
1 => bb42,
2 => bb46,
3 => bb47,
4 => bb48,
5 => bb49,
6 => bb50,
24326 => bb52,
_ => bb51
}
}
bb46 = {
_75 = _49 >> Field::<usize>(Variant(_16, 2), 1);
_3 = _2;
_52.fld0 = Field::<u16>(Variant(_21, 1), 3);
_26 = _47;
(*RET) = _54 as i16;
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_11) = _44;
(*_11) = !_44;
_76.fld4 = Field::<usize>(Variant(_16, 2), 1) % _52.fld4;
(*_12) = _56;
(*_12) = _56;
(*RET) = Field::<i16>(Variant(_21, 1), 4) ^ Field::<i16>(Variant(_21, 1), 4);
_65 = Field::<f64>(Variant(_51.3, 1), 2);
_4 = (*_12);
(*_1) = Move(Field::<*mut i16>(Variant(_16, 2), 2));
(*_14) = [(*_29),Field::<u32>(Variant(_51.3, 1), 4),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_14) = _51.1.1;
_72 = core::ptr::addr_of!((*_12));
_59 = core::ptr::addr_of_mut!((*_72));
Goto(bb45)
}
bb47 = {
(*_12) = _4;
(*_12) = _3;
Call(_20 = core::intrinsics::bswap((*_17)), ReturnTo(bb26), UnwindUnreachable())
}
bb48 = {
_35 = core::ptr::addr_of!((*_11));
_21 = Adt55::Variant1 { fld0: Move(_59),fld1: Move(_52.fld3),fld2: (*_48),fld3: _52.fld0,fld4: (*RET) };
_40.1 = &mut _31;
_60.fld1 = [_5];
(*RET) = -_39.1;
RET = core::ptr::addr_of_mut!((*RET));
(*_48) = Field::<isize>(Variant(_27, 1), 2) >> _52.fld0;
(*_12) = _2;
Goto(bb43)
}
bb49 = {
Return()
}
bb50 = {
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_12) = _2;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
(*_12) = _4;
(*_12) = _4;
(*_17) = -_31;
(*RET) = !Field::<i16>(Variant(_27, 1), 4);
_39.3 = [(*_12),(*_12),_2,(*_12)];
(*RET) = _34 as i16;
_15 = [155_u8,131_u8,66_u8,172_u8,47_u8,171_u8,24_u8];
(*_17) = Field::<isize>(Variant(_27, 1), 2) >> (*_11);
(*_17) = -Field::<isize>(Variant(_27, 1), 2);
(*_11) = !90154693768229257757022579625809189802_u128;
(*_11) = 248060798078703681714271380215463876268_u128;
(*RET) = 160_u8 as i16;
(*_11) = _28 as u128;
_32 = (*_11) >= (*_11);
(*_17) = _31;
(*_1) = core::ptr::addr_of_mut!(_39.1);
(*RET) = Field::<i16>(Variant(_27, 1), 4) ^ Field::<i16>(Variant(_27, 1), 4);
(*_12) = _2;
(*RET) = -Field::<i16>(Variant(_27, 1), 4);
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
(*_12) = _4;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
_22 = _28;
(*_12) = _2;
(*_14) = [_34,_34,_34,_34,_34,_34,_34];
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb35)
}
bb51 = {
Return()
}
bb52 = {
(*_1) = core::ptr::addr_of_mut!((*RET));
(*_29) = Field::<u32>(Variant(_63.0, 1), 4) * _34;
_65 = Field::<f64>(Variant(_63.0, 1), 2);
_81 = core::ptr::addr_of_mut!((*_12));
_63.1.0 = &place!(Field::<f64>(Variant(_63.0, 1), 2));
(*_12) = _56;
_40.0 = core::ptr::addr_of_mut!(_63.1.0);
_78 = !Field::<bool>(Variant(_51.3, 1), 0);
place!(Field::<u16>(Variant(_27, 1), 3)) = _75 as u16;
_15 = [131_u8,105_u8,168_u8,28_u8,212_u8,0_u8,61_u8];
_2 = (*_12);
(*_14) = _51.1.1;
(*_12) = _4;
_76.fld1 = (-93_i8) as u32;
_23 = _78;
_21 = Adt55::Variant1 { fld0: Move(_12),fld1: Move(_76.fld3),fld2: _75,fld3: _52.fld0,fld4: _53 };
(*_14) = [Field::<u32>(Variant(_51.3, 1), 4),(*_29),Field::<u32>(Variant(_63.0, 1), 4),(*_29),(*_29),(*_29),(*_29)];
(*_14) = [_34,(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_21, 1), 4)));
(*RET) = _53;
_35 = core::ptr::addr_of!((*_11));
place!(Field::<isize>(Variant(_27, 1), 2)) = _23 as isize;
(*_14) = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
Goto(bb53)
}
bb53 = {
Call(_89 = dump_var(Move(_2), Move(_64), Move(_20), Move(_5)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_89 = dump_var(Move(_54), Move(_24), Move(_31), Move(_37)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_89 = dump_var(Move(_78), Move(_53), Move(_49), Move(_19)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_89 = dump_var(Move(_7), Move(_26), Move(_44), _90), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: char,mut _3: *mut *mut i16,mut _4: char,mut _5: char,mut _6: i32,mut _7: char,mut _8: char) -> char {
mir! {
type RET = char;
let _9: i32;
let _10: Adt44;
let _11: Adt43;
let _12: ([char; 4], [u8; 2], &'static mut isize, char);
let _13: i128;
let _14: isize;
let _15: (*mut *mut i16, i32, Adt55, *mut i16);
let _16: *mut &'static f64;
let _17: *mut &'static mut u8;
let _18: isize;
let _19: &'static f64;
let _20: &'static mut isize;
let _21: [u8; 7];
let _22: isize;
let _23: Adt28;
let _24: *const &'static mut u8;
let _25: [char; 4];
let _26: [i128; 2];
let _27: [u8; 7];
let _28: u16;
let _29: bool;
let _30: Adt43;
let _31: i8;
let _32: i32;
let _33: *const i16;
let _34: f32;
let _35: bool;
let _36: char;
let _37: char;
let _38: isize;
let _39: [i64; 3];
let _40: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _41: *mut &'static mut u8;
let _42: &'static bool;
let _43: [i32; 1];
let _44: ([char; 4], u8, [char; 4]);
let _45: (u128, usize, [i32; 1]);
let _46: f64;
let _47: *mut i16;
let _48: isize;
let _49: [bool; 2];
let _50: bool;
let _51: f32;
let _52: (*const i16,);
let _53: &'static u8;
let _54: char;
let _55: u64;
let _56: [i128; 2];
let _57: i32;
let _58: &'static mut *const i16;
let _59: &'static mut &'static usize;
let _60: (*mut *mut i16, i32, Adt55, *mut i16);
let _61: u64;
let _62: char;
let _63: i64;
let _64: [u128; 2];
let _65: &'static &'static mut *mut char;
let _66: [char; 4];
let _67: i128;
let _68: bool;
let _69: Adt28;
let _70: *const &'static mut &'static f64;
let _71: u16;
let _72: [i32; 1];
let _73: f32;
let _74: i128;
let _75: ([char; 4], u8, [char; 4]);
let _76: [u32; 7];
let _77: [u32; 7];
let _78: usize;
let _79: (*mut &'static f64, &'static mut isize);
let _80: i16;
let _81: (*mut *mut i16, i32, Adt55, *mut i16);
let _82: i64;
let _83: &'static mut i8;
let _84: &'static mut i8;
let _85: *const &'static bool;
let _86: &'static mut i8;
let _87: u8;
let _88: isize;
let _89: u32;
let _90: (*mut &'static f64, &'static mut isize);
let _91: &'static f64;
let _92: &'static i8;
let _93: u8;
let _94: u64;
let _95: ();
let _96: ();
{
_4 = _8;
RET = _5;
_2 = _4;
RET = _2;
_9 = -_6;
_8 = RET;
_7 = _4;
_8 = _5;
_2 = RET;
Call(_11 = fn4(Move(_3), _8, _7, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _5;
_1 = Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
_1 = -Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
_7 = _2;
_1 = Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0) * Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
place!(Field::<f32>(Variant(_11, 3), 1)) = (-31273_i16) as f32;
_7 = _2;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 61399_u16,fld1: 105351821701949088957179214205575908479_i128 };
_8 = _7;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _8 as i128;
_2 = _4;
_5 = _4;
Goto(bb2)
}
bb2 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb3 = {
_8 = _4;
_6 = _9 + _9;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13 - _13;
_8 = _5;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 17978_u16,fld1: _13 };
_3 = core::ptr::addr_of_mut!(_15.3);
_13 = -Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
place!(Field::<u16>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 0)) = 26940_i16 as u16;
_15.0 = core::ptr::addr_of_mut!((*_3));
_14 = _1 & _1;
Goto(bb4)
}
bb4 = {
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
_15.1 = !_6;
place!(Field::<i8>(Variant(_11, 3), 3)) = (-113_i8) + 62_i8;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 39853_u16,fld1: _13 };
_1 = _14 - _14;
_5 = _4;
_5 = _2;
_4 = _8;
_12.2 = &mut _1;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = 214526942960101464740791846450830845058_u128 as i128;
_7 = RET;
_12.3 = _5;
_14 = 9223372036854775807_isize << _15.1;
_18 = _13 as isize;
_15.0 = core::ptr::addr_of_mut!((*_3));
RET = _12.3;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = -_13;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_12.1 = [202_u8,30_u8];
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_8 = _4;
_12.1 = [153_u8,212_u8];
_12.0 = [_8,_4,_12.3,_8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _5;
_12.2 = &mut place!(Field::<isize>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 1), 0));
_8 = _2;
Goto(bb5)
}
bb5 = {
_2 = _4;
_9 = _6;
_2 = _12.3;
_3 = core::ptr::addr_of_mut!((*_3));
_4 = _12.3;
_15.0 = core::ptr::addr_of_mut!((*_3));
_18 = _14 ^ _14;
_8 = _5;
_21 = [252_u8,127_u8,87_u8,17_u8,115_u8,139_u8,82_u8];
_2 = RET;
_12.1 = [249_u8,94_u8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _2;
_15.1 = _6;
_20 = &mut _14;
(*_20) = _18;
(*_20) = -_18;
_18 = !(*_20);
_12.2 = Move(_20);
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb6)
}
bb6 = {
RET = _5;
_12.2 = &mut _18;
_23.fld2 = _13 as f32;
_15.1 = _6;
_23.fld1 = !10226818_u32;
_4 = _12.3;
_12.1 = [17_u8,132_u8];
_2 = RET;
_23.fld0 = !38515_u16;
_12.0 = [_12.3,_4,_7,_8];
_2 = _8;
_23.fld0 = !18507_u16;
_13 = 15738468650867183732266409067749643526_i128;
_23.fld2 = _15.1 as f32;
_8 = _5;
match _13 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
15738468650867183732266409067749643526 => bb11,
_ => bb10
}
}
bb7 = {
_2 = _4;
_9 = _6;
_2 = _12.3;
_3 = core::ptr::addr_of_mut!((*_3));
_4 = _12.3;
_15.0 = core::ptr::addr_of_mut!((*_3));
_18 = _14 ^ _14;
_8 = _5;
_21 = [252_u8,127_u8,87_u8,17_u8,115_u8,139_u8,82_u8];
_2 = RET;
_12.1 = [249_u8,94_u8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _2;
_15.1 = _6;
_20 = &mut _14;
(*_20) = _18;
(*_20) = -_18;
_18 = !(*_20);
_12.2 = Move(_20);
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb6)
}
bb8 = {
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
_15.1 = !_6;
place!(Field::<i8>(Variant(_11, 3), 3)) = (-113_i8) + 62_i8;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 39853_u16,fld1: _13 };
_1 = _14 - _14;
_5 = _4;
_5 = _2;
_4 = _8;
_12.2 = &mut _1;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = 214526942960101464740791846450830845058_u128 as i128;
_7 = RET;
_12.3 = _5;
_14 = 9223372036854775807_isize << _15.1;
_18 = _13 as isize;
_15.0 = core::ptr::addr_of_mut!((*_3));
RET = _12.3;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = -_13;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_12.1 = [202_u8,30_u8];
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_8 = _4;
_12.1 = [153_u8,212_u8];
_12.0 = [_8,_4,_12.3,_8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _5;
_12.2 = &mut place!(Field::<isize>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 1), 0));
_8 = _2;
Goto(bb5)
}
bb9 = {
_8 = _4;
_6 = _9 + _9;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13 - _13;
_8 = _5;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 17978_u16,fld1: _13 };
_3 = core::ptr::addr_of_mut!(_15.3);
_13 = -Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
place!(Field::<u16>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 0)) = 26940_i16 as u16;
_15.0 = core::ptr::addr_of_mut!((*_3));
_14 = _1 & _1;
Goto(bb4)
}
bb10 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb11 = {
_8 = _2;
_12.0 = [_2,_2,_5,_5];
_13 = (-6273218062534240678512721120109841103_i128) >> _9;
_12.1 = [166_u8,67_u8];
_25 = [_12.3,_4,_8,_2];
_15.0 = core::ptr::addr_of_mut!((*_3));
_21 = [196_u8,48_u8,245_u8,151_u8,68_u8,71_u8,10_u8];
_23.fld0 = _23.fld1 as u16;
_5 = _12.3;
_6 = _15.1 | _9;
_23.fld1 = 3293171505_u32 * 4169162515_u32;
_9 = !_6;
_20 = Move(_12.2);
RET = _2;
_21 = [54_u8,32_u8,144_u8,147_u8,201_u8,239_u8,8_u8];
Goto(bb12)
}
bb12 = {
_2 = _7;
_26 = [_13,_13];
_15.1 = -_9;
_23.fld1 = 400641993_u32 | 2239645557_u32;
_23.fld2 = 224508361245332671899110538987075848244_u128 as f32;
_6 = -_9;
_25 = [_4,_7,_4,_5];
_27 = _21;
_27 = [241_u8,189_u8,85_u8,159_u8,187_u8,63_u8,127_u8];
_23.fld4 = _23.fld1 as usize;
_25 = [_5,_12.3,_12.3,_8];
_23.fld4 = 6876904583351740064_usize - 6_usize;
_13 = (-31552071260008147416042559150803648008_i128) ^ 146979005277860489697742389498933022962_i128;
_28 = _8 as u16;
_15.1 = _6 >> _6;
_23.fld1 = !2756520392_u32;
_22 = _12.3 as isize;
Goto(bb13)
}
bb13 = {
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_28 = _23.fld0;
_4 = _7;
_21 = [247_u8,11_u8,33_u8,14_u8,81_u8,180_u8,47_u8];
_25 = [_8,_8,RET,_4];
_21 = [192_u8,142_u8,170_u8,54_u8,10_u8,23_u8,206_u8];
_7 = _5;
_3 = core::ptr::addr_of_mut!((*_3));
_12.2 = &mut _22;
_23.fld2 = 187182019303229145467790343178611136_u128 as f32;
_15.0 = core::ptr::addr_of_mut!((*_3));
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_29 = false | false;
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!(_15.3);
_9 = _29 as i32;
_26 = [_13,_13];
_28 = _23.fld2 as u16;
_4 = _8;
_31 = -10_i8;
_25 = _12.0;
_21 = [233_u8,93_u8,95_u8,4_u8,130_u8,41_u8,184_u8];
Goto(bb14)
}
bb14 = {
_29 = _6 == _6;
_15.0 = core::ptr::addr_of_mut!((*_3));
_15.0 = core::ptr::addr_of_mut!((*_3));
_13 = (-121512128548139117273960627478327617126_i128) + (-18222423426461441048868910170673225001_i128);
_12.1 = [124_u8,100_u8];
_3 = core::ptr::addr_of_mut!((*_3));
_13 = 237_u8 as i128;
_12.0 = _25;
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_2 = _8;
_5 = _4;
_23.fld1 = 4077182113_u32;
_8 = _2;
_12.3 = _4;
_32 = _15.1 + _15.1;
_4 = _2;
_23.fld2 = _23.fld1 as f32;
_12.3 = _2;
_15.1 = _32;
_23.fld2 = (-86_isize) as f32;
_10 = Adt44::Variant0 { fld0: _29,fld1: _26,fld2: Move(_23),fld3: _31,fld4: 6166_i16,fld5: _25,fld6: 16317120971512545649_u64,fld7: _23.fld2 };
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_23 = Move(Field::<Adt28>(Variant(_10, 0), 2));
_34 = _23.fld2;
_4 = _2;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_26 = Field::<[i128; 2]>(Variant(_10, 0), 1);
match _23.fld1 {
0 => bb7,
4077182113 => bb16,
_ => bb15
}
}
bb15 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb16 = {
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_4 = _7;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_23.fld2 = _34;
_23.fld2 = _34 - Field::<f32>(Variant(_10, 0), 7);
_23.fld2 = _34;
_12.0 = [_8,_4,RET,_4];
_34 = -_23.fld2;
_8 = _5;
place!(Field::<[char; 4]>(Variant(_10, 0), 5)) = [_12.3,_7,_2,_4];
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
place!(Field::<u64>(Variant(_10, 0), 6)) = 9921265609556374014_u64;
_12.0 = [_4,_7,_5,_4];
_2 = _7;
place!(Field::<[char; 4]>(Variant(_10, 0), 5)) = _12.0;
place!(Field::<u64>(Variant(_10, 0), 6)) = 186_u8 as u64;
place!(Field::<i8>(Variant(_10, 0), 3)) = _2 as i8;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
Goto(bb17)
}
bb17 = {
place!(Field::<u64>(Variant(_10, 0), 6)) = 7840590128054409278_u64 + 10321878472527521224_u64;
match _23.fld1 {
4077182113 => bb19,
_ => bb18
}
}
bb18 = {
_2 = _7;
_26 = [_13,_13];
_15.1 = -_9;
_23.fld1 = 400641993_u32 | 2239645557_u32;
_23.fld2 = 224508361245332671899110538987075848244_u128 as f32;
_6 = -_9;
_25 = [_4,_7,_4,_5];
_27 = _21;
_27 = [241_u8,189_u8,85_u8,159_u8,187_u8,63_u8,127_u8];
_23.fld4 = _23.fld1 as usize;
_25 = [_5,_12.3,_12.3,_8];
_23.fld4 = 6876904583351740064_usize - 6_usize;
_13 = (-31552071260008147416042559150803648008_i128) ^ 146979005277860489697742389498933022962_i128;
_28 = _8 as u16;
_15.1 = _6 >> _6;
_23.fld1 = !2756520392_u32;
_22 = _12.3 as isize;
Goto(bb13)
}
bb19 = {
_35 = !Field::<bool>(Variant(_10, 0), 0);
_13 = -(-7975655044702855422951321619881036035_i128);
_26 = [_13,_13];
_8 = RET;
_8 = RET;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld4 = Field::<u64>(Variant(_10, 0), 6) as usize;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(_23.fld1);
_8 = _5;
place!(Field::<i16>(Variant(_10, 0), 4)) = (-368598272902903747_i64) as i16;
_12.0 = _25;
Goto(bb20)
}
bb20 = {
_26 = Field::<[i128; 2]>(Variant(_10, 0), 1);
_27 = _21;
_12.0 = [_8,_12.3,_7,_12.3];
_26 = [_13,_13];
_37 = _7;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_40.1.1 = [_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1];
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_7 = _5;
_20 = Move(_12.2);
place!(Field::<bool>(Variant(_10, 0), 0)) = _29;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_27 = [7_u8,255_u8,175_u8,50_u8,94_u8,214_u8,76_u8];
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_23.fld4 = Field::<Adt28>(Variant(_10, 0), 2).fld4 << _9;
_40.1.1 = [_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1];
_15.0 = Move(_3);
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld2 = _23.fld2;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld0 = !_28;
_3 = core::ptr::addr_of_mut!(_15.3);
_38 = 84_isize >> _32;
_42 = &_29;
_6 = _32;
_31 = Field::<i8>(Variant(_10, 0), 3);
_12.2 = &mut _38;
_13 = 7852632443816075351310656871792127468_i128 & (-193587330658598770850682075743516705_i128);
Goto(bb21)
}
bb21 = {
_13 = Field::<i8>(Variant(_10, 0), 3) as i128;
place!(Field::<u64>(Variant(_10, 0), 6)) = !5314409531170227642_u64;
_6 = 166895865764309684313325617876861618626_u128 as i32;
_15.1 = _32 << Field::<i8>(Variant(_10, 0), 3);
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1);
_13 = (-111936576675012393311155712088580551115_i128) >> _15.1;
_44.1 = 55_u8 ^ 28_u8;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld2 = -_34;
_15.1 = _32 ^ _32;
_45.1 = _23.fld4 ^ Field::<Adt28>(Variant(_10, 0), 2).fld4;
place!(Field::<f32>(Variant(_10, 0), 7)) = _23.fld2 - _34;
_21 = [_44.1,_44.1,_44.1,_44.1,_44.1,_44.1,_44.1];
_39 = [(-7571389374304719252_i64),(-8550783549934849335_i64),423632659002117664_i64];
_23.fld0 = _28 - Field::<Adt28>(Variant(_10, 0), 2).fld0;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1 = _23.fld1;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_44 = (_12.0, 162_u8, _25);
_27 = _21;
_21 = [_44.1,_44.1,_44.1,_44.1,_44.1,_44.1,_44.1];
place!(Field::<i16>(Variant(_10, 0), 4)) = (-32749_i16) ^ 14645_i16;
_32 = _15.1 << _15.1;
_12.3 = _7;
_3 = Move(_15.0);
_45.0 = 200417106553311485598003869860978982202_u128 & 109799133607749859496063555926607432258_u128;
_28 = _23.fld0;
_21 = [_44.1,_44.1,_44.1,_44.1,_44.1,_44.1,_44.1];
_15.1 = !_32;
_15.0 = core::ptr::addr_of_mut!(_47);
match Field::<Adt28>(Variant(_10, 0), 2).fld1 {
0 => bb22,
4077182113 => bb24,
_ => bb23
}
}
bb22 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb23 = {
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _13;
_15.1 = !_6;
place!(Field::<i8>(Variant(_11, 3), 3)) = (-113_i8) + 62_i8;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 39853_u16,fld1: _13 };
_1 = _14 - _14;
_5 = _4;
_5 = _2;
_4 = _8;
_12.2 = &mut _1;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = 214526942960101464740791846450830845058_u128 as i128;
_7 = RET;
_12.3 = _5;
_14 = 9223372036854775807_isize << _15.1;
_18 = _13 as isize;
_15.0 = core::ptr::addr_of_mut!((*_3));
RET = _12.3;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = -_13;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_12.1 = [202_u8,30_u8];
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant1 { fld0: _14 };
_8 = _4;
_12.1 = [153_u8,212_u8];
_12.0 = [_8,_4,_12.3,_8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _5;
_12.2 = &mut place!(Field::<isize>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 1), 0));
_8 = _2;
Goto(bb5)
}
bb24 = {
_34 = _31 as f32;
_3 = core::ptr::addr_of_mut!(_15.3);
Goto(bb25)
}
bb25 = {
_23.fld0 = (-9223372036854775808_isize) as u16;
_45.2 = [_15.1];
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld2 = Field::<u64>(Variant(_10, 0), 6) as f32;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld4 = (*_42) as usize;
_36 = _2;
place!(Field::<Adt28>(Variant(_10, 0), 2)) = Adt28 { fld0: _28,fld1: _23.fld1,fld2: Field::<f32>(Variant(_10, 0), 7),fld3: Move(_23.fld3),fld4: _45.1 };
_46 = Field::<i8>(Variant(_10, 0), 3) as f64;
_23.fld0 = _28 * _28;
_23.fld3 = core::ptr::addr_of_mut!(place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1);
_12.3 = _2;
_49 = [(*_42),(*_42)];
Goto(bb26)
}
bb26 = {
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
place!(Field::<i8>(Variant(_10, 0), 3)) = !_31;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_34 = Field::<f32>(Variant(_10, 0), 7);
match _44.1 {
0 => bb1,
1 => bb11,
2 => bb24,
162 => bb27,
_ => bb19
}
}
bb27 = {
place!(Field::<i8>(Variant(_10, 0), 3)) = -_31;
_12.3 = _2;
_8 = _12.3;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_51 = _46 as f32;
_49 = [(*_42),(*_42)];
_29 = !Field::<bool>(Variant(_10, 0), 0);
_15.0 = core::ptr::addr_of_mut!((*_3));
_12.3 = _5;
Goto(bb28)
}
bb28 = {
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_44.2 = [_5,_36,_5,_4];
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
place!(Field::<bool>(Variant(_10, 0), 0)) = _35;
_3 = core::ptr::addr_of_mut!((*_3));
_40.0 = Adt41::Variant1 { fld0: _35,fld1: Field::<u64>(Variant(_10, 0), 6),fld2: _46,fld3: Field::<i8>(Variant(_10, 0), 3),fld4: Field::<Adt28>(Variant(_10, 0), 2).fld1 };
_47 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_47) = (-9846_i16) * 16292_i16;
Goto(bb29)
}
bb29 = {
_9 = _32 * _15.1;
(*_47) = Field::<f64>(Variant(_40.0, 1), 2) as i16;
_33 = core::ptr::addr_of!((*_47));
(*_33) = _36 as i16;
(*_33) = (-27022_i16) * (-28327_i16);
_3 = core::ptr::addr_of_mut!(_15.3);
(*_3) = core::ptr::addr_of_mut!((*_47));
(*_33) = (-29590_i16) ^ 7036_i16;
_37 = _2;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1 = _23.fld1;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_40.0, 1), 4)));
_57 = _32 >> _15.1;
_44.0 = Field::<[char; 4]>(Variant(_10, 0), 5);
_48 = _36 as isize;
_43 = [_57];
_32 = _15.1;
_44.0 = Field::<[char; 4]>(Variant(_10, 0), 5);
_40.3 = Move(_40.0);
_33 = core::ptr::addr_of!((*_33));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
match _44.1 {
0 => bb16,
162 => bb31,
_ => bb30
}
}
bb30 = {
place!(Field::<i8>(Variant(_10, 0), 3)) = -_31;
_12.3 = _2;
_8 = _12.3;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_51 = _46 as f32;
_49 = [(*_42),(*_42)];
_29 = !Field::<bool>(Variant(_10, 0), 0);
_15.0 = core::ptr::addr_of_mut!((*_3));
_12.3 = _5;
Goto(bb28)
}
bb31 = {
(*_33) = 12724_i16;
(*_33) = (-2132_i16) | (-22511_i16);
_19 = &_46;
(*_3) = Move(_47);
(*_33) = 26962_i16;
(*_33) = Field::<u32>(Variant(_40.3, 1), 4) as i16;
_35 = !_29;
_35 = Field::<bool>(Variant(_10, 0), 0) | Field::<bool>(Variant(_10, 0), 0);
match _44.1 {
0 => bb32,
1 => bb33,
2 => bb34,
162 => bb36,
_ => bb35
}
}
bb32 = {
_2 = _5;
_1 = Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
_1 = -Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
_7 = _2;
_1 = Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0) * Field::<isize>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 1), 0);
place!(Field::<f32>(Variant(_11, 3), 1)) = (-31273_i16) as f32;
_7 = _2;
place!(Field::<Adt23>(Variant(_11, 3), 2)) = Adt23::Variant2 { fld0: 61399_u16,fld1: 105351821701949088957179214205575908479_i128 };
_8 = _7;
place!(Field::<i128>(Variant(place!(Field::<Adt23>(Variant(_11, 3), 2)), 2), 1)) = _8 as i128;
_2 = _4;
_5 = _4;
Goto(bb2)
}
bb33 = {
_2 = _4;
_9 = _6;
_2 = _12.3;
_3 = core::ptr::addr_of_mut!((*_3));
_4 = _12.3;
_15.0 = core::ptr::addr_of_mut!((*_3));
_18 = _14 ^ _14;
_8 = _5;
_21 = [252_u8,127_u8,87_u8,17_u8,115_u8,139_u8,82_u8];
_2 = RET;
_12.1 = [249_u8,94_u8];
_3 = core::ptr::addr_of_mut!((*_3));
_8 = _2;
_15.1 = _6;
_20 = &mut _14;
(*_20) = _18;
(*_20) = -_18;
_18 = !(*_20);
_12.2 = Move(_20);
_23.fld3 = core::ptr::addr_of_mut!(_23.fld1);
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb6)
}
bb34 = {
_35 = !Field::<bool>(Variant(_10, 0), 0);
_13 = -(-7975655044702855422951321619881036035_i128);
_26 = [_13,_13];
_8 = RET;
_8 = RET;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld4 = Field::<u64>(Variant(_10, 0), 6) as usize;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(_23.fld1);
_8 = _5;
place!(Field::<i16>(Variant(_10, 0), 4)) = (-368598272902903747_i64) as i16;
_12.0 = _25;
Goto(bb20)
}
bb35 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb36 = {
place!(Field::<f64>(Variant(_40.3, 1), 2)) = (*_19) - (*_19);
_43 = [_9];
match _44.1 {
0 => bb34,
1 => bb27,
2 => bb19,
3 => bb32,
4 => bb6,
162 => bb38,
_ => bb37
}
}
bb37 = {
_9 = _32 * _15.1;
(*_47) = Field::<f64>(Variant(_40.0, 1), 2) as i16;
_33 = core::ptr::addr_of!((*_47));
(*_33) = _36 as i16;
(*_33) = (-27022_i16) * (-28327_i16);
_3 = core::ptr::addr_of_mut!(_15.3);
(*_3) = core::ptr::addr_of_mut!((*_47));
(*_33) = (-29590_i16) ^ 7036_i16;
_37 = _2;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1 = _23.fld1;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_40.0, 1), 4)));
_57 = _32 >> _15.1;
_44.0 = Field::<[char; 4]>(Variant(_10, 0), 5);
_48 = _36 as isize;
_43 = [_57];
_32 = _15.1;
_44.0 = Field::<[char; 4]>(Variant(_10, 0), 5);
_40.3 = Move(_40.0);
_33 = core::ptr::addr_of!((*_33));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
match _44.1 {
0 => bb16,
162 => bb31,
_ => bb30
}
}
bb38 = {
_31 = _13 as i8;
_42 = &place!(Field::<bool>(Variant(_40.3, 1), 0));
(*_33) = (-16595_i16) * (-20289_i16);
_21 = _27;
(*_33) = 3109_i16;
_54 = _5;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1 = Field::<u32>(Variant(_40.3, 1), 4) - _23.fld1;
(*_3) = core::ptr::addr_of_mut!((*_33));
_23.fld3 = core::ptr::addr_of_mut!(place!(Field::<Adt28>(Variant(_10, 0), 2)).fld1);
_40.0 = Move(_40.3);
_12.1 = [_44.1,_44.1];
_58 = &mut _33;
_10 = Adt44::Variant0 { fld0: _35,fld1: _26,fld2: Move(_23),fld3: _31,fld4: 14722_i16,fld5: _44.0,fld6: Field::<u64>(Variant(_40.0, 1), 1),fld7: _34 };
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
Goto(bb39)
}
bb39 = {
_44.1 = 240_u8;
_12.2 = &mut _48;
_5 = _36;
_21 = _27;
_60.0 = Move(_3);
_66 = [_12.3,_5,RET,RET];
Goto(bb40)
}
bb40 = {
_16 = core::ptr::addr_of_mut!(_19);
(*_16) = &place!(Field::<f64>(Variant(_40.0, 1), 2));
_60.3 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_69.fld1 = _31 as u32;
_23 = Adt28 { fld0: _28,fld1: _69.fld1,fld2: _51,fld3: Move(Field::<Adt28>(Variant(_10, 0), 2).fld3),fld4: Field::<Adt28>(Variant(_10, 0), 2).fld4 };
place!(Field::<u64>(Variant(_10, 0), 6)) = Field::<u64>(Variant(_40.0, 1), 1) + Field::<u64>(Variant(_40.0, 1), 1);
_46 = (*_19) * (*_19);
_73 = Field::<f32>(Variant(_10, 0), 7) + Field::<Adt28>(Variant(_10, 0), 2).fld2;
_69 = Adt28 { fld0: Field::<Adt28>(Variant(_10, 0), 2).fld0,fld1: _23.fld1,fld2: Field::<f32>(Variant(_10, 0), 7),fld3: Move(_23.fld3),fld4: _45.1 };
_28 = _46 as u16;
place!(Field::<i8>(Variant(_10, 0), 3)) = !_31;
_23.fld1 = _69.fld1 ^ _69.fld1;
_46 = (*_19) - Field::<f64>(Variant(_40.0, 1), 2);
Goto(bb41)
}
bb41 = {
_44.0 = [_36,_37,_37,_36];
_40.3 = Move(_40.0);
_39 = [(-4832291866135021306_i64),4800052203042500251_i64,(-3147744418160604068_i64)];
_15.1 = (-6775_i16) as i32;
_61 = Field::<u64>(Variant(_10, 0), 6) | Field::<u64>(Variant(_10, 0), 6);
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(_69.fld1);
_73 = _23.fld2 + Field::<Adt28>(Variant(_10, 0), 2).fld2;
_45.2 = [_32];
_64 = [_45.0,_45.0];
_67 = _32 as i128;
_73 = _51;
_66 = _25;
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
_54 = _12.3;
(*_16) = &_46;
_73 = _23.fld1 as f32;
_3 = core::ptr::addr_of_mut!(_15.3);
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
_15.3 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_63 = 8513041087885156996_i64 ^ 7272422353388932907_i64;
_40.1.0 = &(*_19);
_76 = [_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_23.fld1,_69.fld1];
_13 = _67 * _67;
_35 = (*_19) < (*_19);
Goto(bb42)
}
bb42 = {
_13 = -_67;
_15.3 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_69.fld1 = _23.fld1 & _23.fld1;
_54 = _2;
_55 = _61 * _61;
_61 = Field::<u64>(Variant(_10, 0), 6) - Field::<u64>(Variant(_10, 0), 6);
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
_40.1.0 = &place!(Field::<f64>(Variant(_40.3, 1), 2));
Goto(bb43)
}
bb43 = {
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_55 = _57 as u64;
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
_79 = (Move(_16), Move(_12.2));
(*_3) = Move(_60.3);
_39 = [_63,_63,_63];
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_25 = [_8,_12.3,_36,_54];
_40.1 = (Move(_19), _76);
_25 = [RET,RET,_2,_36];
(*_3) = core::ptr::addr_of_mut!(_80);
_29 = _35 ^ Field::<bool>(Variant(_10, 0), 0);
_75.0 = _44.2;
(*_3) = core::ptr::addr_of_mut!(_80);
_74 = _13;
_42 = &_29;
match Field::<Adt28>(Variant(_10, 0), 2).fld1 {
0 => bb6,
1 => bb37,
2 => bb15,
3 => bb33,
4077182113 => bb44,
_ => bb21
}
}
bb44 = {
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
place!(Field::<u64>(Variant(_10, 0), 6)) = _55 & _55;
place!(Field::<f32>(Variant(_10, 0), 7)) = _45.0 as f32;
_40.1.0 = &_46;
place!(Field::<bool>(Variant(_10, 0), 0)) = !(*_42);
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_51 = -_73;
_12.0 = Field::<[char; 4]>(Variant(_10, 0), 5);
_66 = [_54,_5,_8,_2];
_72 = _45.2;
_7 = _8;
place!(Field::<bool>(Variant(_10, 0), 0)) = (*_42) & (*_42);
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(_80);
_2 = _5;
_40.1.1 = [_69.fld1,_69.fld1,_69.fld1,_69.fld1,_23.fld1,_69.fld1,_69.fld1];
(*_58) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_10, 0), 4)));
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld3 = core::ptr::addr_of_mut!(_69.fld1);
_47 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_50 = (*_42) ^ (*_42);
_53 = &_44.1;
place!(Field::<Adt28>(Variant(_10, 0), 2)).fld4 = _69.fld4;
(*_58) = core::ptr::addr_of!((*_47));
_45.1 = _23.fld0 as usize;
(*_3) = core::ptr::addr_of_mut!((*_47));
Goto(bb45)
}
bb45 = {
_89 = _23.fld1 & _23.fld1;
(*_3) = core::ptr::addr_of_mut!((*_47));
_40.1.1 = [_89,_69.fld1,_89,_69.fld1,_89,_69.fld1,_89];
Goto(bb46)
}
bb46 = {
place!(Field::<u64>(Variant(_10, 0), 6)) = _67 as u64;
_4 = _36;
place!(Field::<[i128; 2]>(Variant(_10, 0), 1)) = _26;
_45.1 = _69.fld4;
(*_47) = 12172_i16 << _31;
_19 = &_46;
_23.fld3 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_40.3, 1), 4)));
(*_47) = (-15667_i16) & (-5509_i16);
(*_47) = 8885_i16 | (-5777_i16);
(*_3) = core::ptr::addr_of_mut!((*_47));
_23.fld2 = _55 as f32;
_52 = (Move((*_58)),);
_46 = Field::<f64>(Variant(_40.3, 1), 2);
match (*_53) {
0 => bb47,
1 => bb48,
240 => bb50,
_ => bb49
}
}
bb47 = {
place!(Field::<i8>(Variant(_10, 0), 3)) = -_31;
_12.3 = _2;
_8 = _12.3;
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
(*_3) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 0), 4)));
_51 = _46 as f32;
_49 = [(*_42),(*_42)];
_29 = !Field::<bool>(Variant(_10, 0), 0);
_15.0 = core::ptr::addr_of_mut!((*_3));
_12.3 = _5;
Goto(bb28)
}
bb48 = {
_2 = _7;
_26 = [_13,_13];
_15.1 = -_9;
_23.fld1 = 400641993_u32 | 2239645557_u32;
_23.fld2 = 224508361245332671899110538987075848244_u128 as f32;
_6 = -_9;
_25 = [_4,_7,_4,_5];
_27 = _21;
_27 = [241_u8,189_u8,85_u8,159_u8,187_u8,63_u8,127_u8];
_23.fld4 = _23.fld1 as usize;
_25 = [_5,_12.3,_12.3,_8];
_23.fld4 = 6876904583351740064_usize - 6_usize;
_13 = (-31552071260008147416042559150803648008_i128) ^ 146979005277860489697742389498933022962_i128;
_28 = _8 as u16;
_15.1 = _6 >> _6;
_23.fld1 = !2756520392_u32;
_22 = _12.3 as isize;
Goto(bb13)
}
bb49 = {
_13 = Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1) ^ Field::<i128>(Variant(Field::<Adt23>(Variant(_11, 3), 2), 2), 1);
_4 = _8;
_5 = _7;
_2 = _5;
_9 = !_6;
Goto(bb3)
}
bb50 = {
_90.0 = core::ptr::addr_of_mut!(_19);
Goto(bb51)
}
bb51 = {
Call(_95 = dump_var(Move(_21), Move(_50), Move(_4), Move(_14)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_95 = dump_var(Move(_44), Move(_89), Move(_31), Move(_27)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_95 = dump_var(Move(_9), Move(_43), Move(_38), Move(_35)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_95 = dump_var(Move(_13), Move(_39), Move(_45), Move(_5)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_95 = dump_var(Move(_76), Move(_29), Move(_72), Move(_18)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_95 = dump_var(Move(_2), _96, _96, _96), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: *mut *mut i16,mut _2: char,mut _3: char,mut _4: i32) -> Adt43 {
mir! {
type RET = Adt43;
let _5: char;
let _6: *mut char;
let _7: &'static mut *mut i16;
let _8: [u64; 7];
let _9: bool;
let _10: bool;
let _11: f32;
let _12: char;
let _13: isize;
let _14: *const &'static bool;
let _15: [usize; 7];
let _16: &'static i32;
let _17: [u128; 2];
let _18: &'static i8;
let _19: &'static mut *const i16;
let _20: f32;
let _21: u64;
let _22: [u8; 2];
let _23: *const char;
let _24: f32;
let _25: &'static mut *mut char;
let _26: u8;
let _27: char;
let _28: &'static u8;
let _29: isize;
let _30: Adt28;
let _31: f32;
let _32: char;
let _33: isize;
let _34: *mut char;
let _35: i8;
let _36: isize;
let _37: &'static mut isize;
let _38: &'static u8;
let _39: *mut u32;
let _40: (*mut *mut i16, i32, Adt55, *mut i16);
let _41: char;
let _42: Adt43;
let _43: char;
let _44: &'static mut u16;
let _45: isize;
let _46: u128;
let _47: Adt50;
let _48: (Adt23, u128);
let _49: *mut *mut i16;
let _50: *mut &'static mut u8;
let _51: [u128; 2];
let _52: bool;
let _53: &'static mut &'static usize;
let _54: *mut u32;
let _55: &'static mut &'static f64;
let _56: isize;
let _57: Adt28;
let _58: f64;
let _59: u16;
let _60: *mut &'static f64;
let _61: *const &'static bool;
let _62: &'static mut *const i16;
let _63: &'static mut &'static f64;
let _64: [bool; 2];
let _65: isize;
let _66: *mut *mut &'static f64;
let _67: bool;
let _68: (*mut &'static f64, &'static mut isize);
let _69: Adt75;
let _70: *mut *mut &'static f64;
let _71: char;
let _72: isize;
let _73: &'static mut isize;
let _74: bool;
let _75: &'static bool;
let _76: *mut char;
let _77: *const &'static bool;
let _78: isize;
let _79: u16;
let _80: &'static &'static mut u8;
let _81: u32;
let _82: &'static mut *const i16;
let _83: u64;
let _84: f32;
let _85: i128;
let _86: usize;
let _87: &'static mut *const i16;
let _88: &'static &'static mut *mut char;
let _89: i64;
let _90: Adt55;
let _91: (Adt23, u128);
let _92: char;
let _93: usize;
let _94: &'static mut *mut char;
let _95: ();
let _96: ();
{
_2 = _3;
_3 = _2;
Goto(bb1)
}
bb1 = {
_2 = _3;
_3 = _2;
_6 = core::ptr::addr_of_mut!(_2);
_6 = core::ptr::addr_of_mut!((*_6));
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_4 = (-1895466868_i32) << (-29691_i16);
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_4 = 2956447240_u32 as i32;
(*_6) = _3;
_8 = [3409390856582492615_u64,1473666853851910555_u64,15996845707372943287_u64,3456189180170697435_u64,11934429165006653618_u64,8440947725169802275_u64,16861484112254472693_u64];
_9 = !false;
Goto(bb2)
}
bb2 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_8 = [14120019655826477840_u64,11727150017712968920_u64,2626749002719111407_u64,15055792989576853712_u64,9110911911850534246_u64,9472750845889128999_u64,4816462085864602859_u64];
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_10 = _3 >= (*_6);
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
Goto(bb3)
}
bb3 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12 = _3;
_11 = 1043810688_u32 as f32;
_5 = _2;
_6 = core::ptr::addr_of_mut!(_2);
(*_6) = _3;
_13 = 38439774503555201387085177922832612858_i128 as isize;
(*_6) = _3;
_11 = 21541_i16 as f32;
_9 = _10 & _10;
_15 = [2_usize,7_usize,10144285350877186279_usize,3603280982803407934_usize,7658623565001846431_usize,13871377499826416115_usize,15779021935845343307_usize];
_2 = _12;
(*_6) = _3;
(*_6) = _3;
_17 = [271347765945266320987084131153585117835_u128,77871480023544049772173897937898596178_u128];
(*_6) = _12;
_9 = (*_6) == (*_6);
(*_6) = _5;
_6 = core::ptr::addr_of_mut!((*_6));
(*_6) = _12;
(*_6) = _12;
_16 = &_4;
_8 = [4474353839798336379_u64,16237173244817941910_u64,9449642984028817774_u64,1009577513801251761_u64,8853025160030427205_u64,17465390778916943704_u64,10035701421786073183_u64];
_20 = _11;
_9 = _10;
(*_6) = _3;
_21 = 3131305945409183261_u64 ^ 17536441746420773186_u64;
(*_6) = _5;
(*_6) = _5;
_15 = [1_usize,7_usize,7_usize,2_usize,3_usize,2_usize,877089929672617325_usize];
Goto(bb5)
}
bb5 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb6 = {
(*_6) = _5;
_22 = [253_u8,6_u8];
(*_6) = _5;
(*_6) = _3;
_22 = [3_u8,86_u8];
match _21 {
0 => bb7,
3928788315817448533 => bb9,
_ => bb8
}
}
bb7 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb8 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb9 = {
(*_6) = _5;
_5 = _3;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _3;
_24 = _11;
_23 = core::ptr::addr_of!(_2);
(*_23) = _5;
(*_6) = _12;
(*_6) = _3;
_10 = !_9;
(*_6) = _5;
(*_6) = _12;
(*_6) = _3;
_11 = -_20;
_21 = 16184933427735390726_u64 | 13261376371212823650_u64;
(*_6) = _12;
(*_6) = _12;
_24 = -_11;
_11 = _24;
_22 = [239_u8,165_u8];
_25 = &mut _6;
(*_25) = core::ptr::addr_of_mut!(_5);
(*_25) = core::ptr::addr_of_mut!(_12);
_10 = _9 ^ _9;
(*_25) = core::ptr::addr_of_mut!(_3);
_4 = 865898306_i32 * 105941003_i32;
Goto(bb10)
}
bb10 = {
_20 = _24 - _24;
(*_25) = core::ptr::addr_of_mut!(_12);
_20 = 46950_u16 as f32;
_10 = !_9;
_4 = 2083376636_i32;
(*_25) = core::ptr::addr_of_mut!(_5);
(*_25) = core::ptr::addr_of_mut!(_3);
_26 = !157_u8;
_27 = _5;
_24 = _20 * _11;
(*_25) = core::ptr::addr_of_mut!(_3);
(*_25) = core::ptr::addr_of_mut!(_12);
(*_25) = core::ptr::addr_of_mut!(_5);
_10 = _9;
_30.fld4 = 0_usize;
_21 = 9517637678940557483_u64;
_13 = _30.fld4 as isize;
_22 = [_26,_26];
_30.fld3 = core::ptr::addr_of_mut!(_30.fld1);
_30.fld2 = _11 + _24;
_28 = &_26;
_29 = (-165010369392842743842295552245285870133_i128) as isize;
_30.fld1 = 497133543_u32 >> _13;
Goto(bb11)
}
bb11 = {
(*_25) = core::ptr::addr_of_mut!(_3);
_9 = _10;
_2 = _3;
_10 = _9 ^ _9;
_22 = [(*_28),(*_28)];
_22 = [(*_28),(*_28)];
_23 = core::ptr::addr_of!(_12);
(*_23) = _27;
(*_23) = _3;
_32 = (*_23);
(*_23) = _27;
_3 = (*_23);
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!((*_23));
Goto(bb12)
}
bb12 = {
_11 = (-52138243696397217124107813591786900172_i128) as f32;
_10 = _9 | _9;
_31 = _24;
(*_23) = _5;
_33 = 157768594031490520407377968629866377501_i128 as isize;
_17 = [30832294654515359726121699872753461752_u128,286176437916086037972614248289986567344_u128];
(*_23) = _5;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_23) = _32;
_30.fld2 = -_20;
_34 = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!((*_34));
_17 = [174703604359456540020784535310684648697_u128,153246832258085360759496538651961613153_u128];
Goto(bb13)
}
bb13 = {
(*_23) = _3;
(*_23) = _2;
(*_23) = _2;
(*_23) = _3;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!(_2);
_35 = 54_i8 ^ 9_i8;
(*_25) = Move(_34);
(*_25) = core::ptr::addr_of_mut!((*_23));
match _30.fld4 {
0 => bb14,
_ => bb7
}
}
bb14 = {
_15 = [_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4];
_32 = (*_23);
_33 = _29;
_4 = (-573735186_i32);
_30.fld0 = !37751_u16;
(*_23) = _5;
_30.fld4 = _4 as usize;
_40.1 = _4 - _4;
_29 = _30.fld4 as isize;
_5 = (*_23);
(*_25) = core::ptr::addr_of_mut!((*_23));
_8 = [_21,_21,_21,_21,_21,_21,_21];
_21 = 12543698166630127397_u64;
_18 = &_35;
(*_25) = core::ptr::addr_of_mut!((*_23));
_34 = core::ptr::addr_of_mut!(_2);
(*_23) = (*_34);
(*_34) = _12;
_41 = (*_23);
(*_23) = (*_34);
_30.fld0 = 24428_i16 as u16;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_34) = _12;
(*_23) = (*_34);
_44 = &mut _30.fld0;
match _4 {
0 => bb12,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
340282366920938463463374607431194476270 => bb22,
_ => bb21
}
}
bb15 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb16 = {
_11 = (-52138243696397217124107813591786900172_i128) as f32;
_10 = _9 | _9;
_31 = _24;
(*_23) = _5;
_33 = 157768594031490520407377968629866377501_i128 as isize;
_17 = [30832294654515359726121699872753461752_u128,286176437916086037972614248289986567344_u128];
(*_23) = _5;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_23) = _32;
_30.fld2 = -_20;
_34 = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!((*_34));
_17 = [174703604359456540020784535310684648697_u128,153246832258085360759496538651961613153_u128];
Goto(bb13)
}
bb17 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb18 = {
(*_6) = _5;
_22 = [253_u8,6_u8];
(*_6) = _5;
(*_6) = _3;
_22 = [3_u8,86_u8];
match _21 {
0 => bb7,
3928788315817448533 => bb9,
_ => bb8
}
}
bb19 = {
(*_6) = _5;
_5 = _3;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _3;
_24 = _11;
_23 = core::ptr::addr_of!(_2);
(*_23) = _5;
(*_6) = _12;
(*_6) = _3;
_10 = !_9;
(*_6) = _5;
(*_6) = _12;
(*_6) = _3;
_11 = -_20;
_21 = 16184933427735390726_u64 | 13261376371212823650_u64;
(*_6) = _12;
(*_6) = _12;
_24 = -_11;
_11 = _24;
_22 = [239_u8,165_u8];
_25 = &mut _6;
(*_25) = core::ptr::addr_of_mut!(_5);
(*_25) = core::ptr::addr_of_mut!(_12);
_10 = _9 ^ _9;
(*_25) = core::ptr::addr_of_mut!(_3);
_4 = 865898306_i32 * 105941003_i32;
Goto(bb10)
}
bb20 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb21 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb22 = {
(*_23) = (*_34);
(*_25) = core::ptr::addr_of_mut!(_2);
(*_23) = (*_34);
(*_23) = (*_34);
(*_34) = (*_23);
match _4 {
0 => bb13,
1 => bb23,
340282366920938463463374607431194476270 => bb25,
_ => bb24
}
}
bb23 = {
_15 = [_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4];
_32 = (*_23);
_33 = _29;
_4 = (-573735186_i32);
_30.fld0 = !37751_u16;
(*_23) = _5;
_30.fld4 = _4 as usize;
_40.1 = _4 - _4;
_29 = _30.fld4 as isize;
_5 = (*_23);
(*_25) = core::ptr::addr_of_mut!((*_23));
_8 = [_21,_21,_21,_21,_21,_21,_21];
_21 = 12543698166630127397_u64;
_18 = &_35;
(*_25) = core::ptr::addr_of_mut!((*_23));
_34 = core::ptr::addr_of_mut!(_2);
(*_23) = (*_34);
(*_34) = _12;
_41 = (*_23);
(*_23) = (*_34);
_30.fld0 = 24428_i16 as u16;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_34) = _12;
(*_23) = (*_34);
_44 = &mut _30.fld0;
match _4 {
0 => bb12,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
340282366920938463463374607431194476270 => bb22,
_ => bb21
}
}
bb24 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb25 = {
(*_25) = core::ptr::addr_of_mut!((*_34));
_43 = (*_34);
(*_34) = (*_23);
_29 = !_33;
_40.2 = Adt55::Variant0 { fld0: Move(_23),fld1: _21,fld2: 110614274088989181371105440634663026827_u128 };
_31 = -_24;
_25 = &mut _34;
_5 = _3;
(*_25) = core::ptr::addr_of_mut!(_32);
_38 = Move(_28);
place!(Field::<u128>(Variant(_40.2, 0), 2)) = 302504635260875425694369443654352869272_u128;
_21 = Field::<u64>(Variant(_40.2, 0), 1) ^ Field::<u64>(Variant(_40.2, 0), 1);
_47.fld0 = [_43,_5,_2,_12];
(*_25) = core::ptr::addr_of_mut!(_43);
(*_44) = 51630_u16 & 12149_u16;
(*_25) = core::ptr::addr_of_mut!(_27);
place!(Field::<u64>(Variant(_40.2, 0), 1)) = _10 as u64;
match Field::<u128>(Variant(_40.2, 0), 2) {
0 => bb26,
1 => bb27,
2 => bb28,
3 => bb29,
302504635260875425694369443654352869272 => bb31,
_ => bb30
}
}
bb26 = {
_11 = (-52138243696397217124107813591786900172_i128) as f32;
_10 = _9 | _9;
_31 = _24;
(*_23) = _5;
_33 = 157768594031490520407377968629866377501_i128 as isize;
_17 = [30832294654515359726121699872753461752_u128,286176437916086037972614248289986567344_u128];
(*_23) = _5;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_23) = _32;
_30.fld2 = -_20;
_34 = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!((*_34));
_17 = [174703604359456540020784535310684648697_u128,153246832258085360759496538651961613153_u128];
Goto(bb13)
}
bb27 = {
(*_6) = _5;
_22 = [253_u8,6_u8];
(*_6) = _5;
(*_6) = _3;
_22 = [3_u8,86_u8];
match _21 {
0 => bb7,
3928788315817448533 => bb9,
_ => bb8
}
}
bb28 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb29 = {
(*_6) = _5;
_22 = [253_u8,6_u8];
(*_6) = _5;
(*_6) = _3;
_22 = [3_u8,86_u8];
match _21 {
0 => bb7,
3928788315817448533 => bb9,
_ => bb8
}
}
bb30 = {
(*_6) = _5;
_5 = _3;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _3;
_24 = _11;
_23 = core::ptr::addr_of!(_2);
(*_23) = _5;
(*_6) = _12;
(*_6) = _3;
_10 = !_9;
(*_6) = _5;
(*_6) = _12;
(*_6) = _3;
_11 = -_20;
_21 = 16184933427735390726_u64 | 13261376371212823650_u64;
(*_6) = _12;
(*_6) = _12;
_24 = -_11;
_11 = _24;
_22 = [239_u8,165_u8];
_25 = &mut _6;
(*_25) = core::ptr::addr_of_mut!(_5);
(*_25) = core::ptr::addr_of_mut!(_12);
_10 = _9 ^ _9;
(*_25) = core::ptr::addr_of_mut!(_3);
_4 = 865898306_i32 * 105941003_i32;
Goto(bb10)
}
bb31 = {
place!(Field::<u64>(Variant(_40.2, 0), 1)) = _21;
_36 = -_29;
_4 = _40.1 << (*_18);
(*_25) = core::ptr::addr_of_mut!(_5);
_13 = _29 ^ _29;
(*_44) = 47689_u16;
_48.1 = Field::<u128>(Variant(_40.2, 0), 2) << (*_44);
_52 = !_10;
_45 = _29 | _13;
(*_25) = core::ptr::addr_of_mut!(_43);
_36 = -_33;
_23 = core::ptr::addr_of!(_12);
(*_44) = !60788_u16;
_5 = (*_23);
(*_23) = _3;
(*_44) = 957_u16;
(*_25) = core::ptr::addr_of_mut!(_41);
_21 = 14913971322967318817_usize as u64;
_57.fld1 = 2097697805_u32;
place!(Field::<u128>(Variant(_40.2, 0), 2)) = _48.1 ^ _48.1;
_22 = [_26,_26];
_35 = 16_i8 + 37_i8;
(*_44) = 62881_u16 - 51695_u16;
_40.0 = core::ptr::addr_of_mut!(_40.3);
(*_23) = _43;
_37 = &mut _29;
place!(Field::<u64>(Variant(_40.2, 0), 1)) = !_21;
match _57.fld1 {
0 => bb16,
1 => bb30,
2 => bb10,
3 => bb32,
4 => bb33,
2097697805 => bb35,
_ => bb34
}
}
bb32 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb33 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb34 = {
_12 = _3;
_11 = 1043810688_u32 as f32;
_5 = _2;
_6 = core::ptr::addr_of_mut!(_2);
(*_6) = _3;
_13 = 38439774503555201387085177922832612858_i128 as isize;
(*_6) = _3;
_11 = 21541_i16 as f32;
_9 = _10 & _10;
_15 = [2_usize,7_usize,10144285350877186279_usize,3603280982803407934_usize,7658623565001846431_usize,13871377499826416115_usize,15779021935845343307_usize];
_2 = _12;
(*_6) = _3;
(*_6) = _3;
_17 = [271347765945266320987084131153585117835_u128,77871480023544049772173897937898596178_u128];
(*_6) = _12;
_9 = (*_6) == (*_6);
(*_6) = _5;
_6 = core::ptr::addr_of_mut!((*_6));
(*_6) = _12;
(*_6) = _12;
_16 = &_4;
_8 = [4474353839798336379_u64,16237173244817941910_u64,9449642984028817774_u64,1009577513801251761_u64,8853025160030427205_u64,17465390778916943704_u64,10035701421786073183_u64];
_20 = _11;
_9 = _10;
(*_6) = _3;
_21 = 3131305945409183261_u64 ^ 17536441746420773186_u64;
(*_6) = _5;
(*_6) = _5;
_15 = [1_usize,7_usize,7_usize,2_usize,3_usize,2_usize,877089929672617325_usize];
Goto(bb5)
}
bb35 = {
_10 = (*_44) > (*_44);
_57.fld4 = !13335640629607973976_usize;
(*_44) = 49164_u16 << (*_37);
(*_23) = _3;
(*_25) = core::ptr::addr_of_mut!(_41);
_20 = _26 as f32;
(*_37) = _13 | _36;
(*_23) = _3;
(*_25) = core::ptr::addr_of_mut!(_32);
(*_44) = 49192_u16;
_57.fld3 = core::ptr::addr_of_mut!(_57.fld1);
(*_25) = core::ptr::addr_of_mut!(_2);
(*_44) = 5636_u16;
_46 = _48.1 - Field::<u128>(Variant(_40.2, 0), 2);
(*_25) = core::ptr::addr_of_mut!((*_23));
_27 = (*_23);
_33 = _13 ^ (*_37);
_20 = (*_44) as f32;
place!(Field::<u128>(Variant(_40.2, 0), 2)) = _26 as u128;
_37 = &mut _33;
_36 = !(*_37);
_47.fld0 = [(*_23),_12,(*_23),(*_23)];
_40.2 = Adt55::Variant1 { fld0: Move((*_25)),fld1: Move(_57.fld3),fld2: _45,fld3: (*_44),fld4: 19123_i16 };
Goto(bb36)
}
bb36 = {
(*_44) = Field::<u16>(Variant(_40.2, 1), 3);
(*_44) = 92731781935720801429886314240750106988_i128 as u16;
_59 = (-76778686993856155493326692895264341635_i128) as u16;
(*_37) = _36;
_39 = core::ptr::addr_of_mut!(_57.fld1);
_57.fld4 = !9253341435854560583_usize;
_58 = (*_39) as f64;
(*_44) = 5837220643493517290_i64 as u16;
(*_23) = _2;
_36 = (*_37);
match (*_39) {
2097697805 => bb38,
_ => bb37
}
}
bb37 = {
_11 = (-52138243696397217124107813591786900172_i128) as f32;
_10 = _9 | _9;
_31 = _24;
(*_23) = _5;
_33 = 157768594031490520407377968629866377501_i128 as isize;
_17 = [30832294654515359726121699872753461752_u128,286176437916086037972614248289986567344_u128];
(*_23) = _5;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_23) = _32;
_30.fld2 = -_20;
_34 = core::ptr::addr_of_mut!((*_23));
(*_25) = core::ptr::addr_of_mut!((*_34));
_17 = [174703604359456540020784535310684648697_u128,153246832258085360759496538651961613153_u128];
Goto(bb13)
}
bb38 = {
(*_23) = _32;
(*_23) = _32;
(*_39) = 2621725460_u32 - 420620653_u32;
_72 = _45 + (*_37);
(*_37) = (*_23) as isize;
_21 = 7158224352778285918_u64 ^ 10438520362827020137_u64;
_2 = _12;
Goto(bb39)
}
bb39 = {
(*_23) = _43;
(*_37) = _13 & Field::<isize>(Variant(_40.2, 1), 2);
(*_39) = _41 as u32;
(*_37) = _36 - _72;
_28 = &_26;
_21 = _35 as u64;
_54 = core::ptr::addr_of_mut!((*_39));
(*_39) = 2658029051_u32;
(*_39) = !3759091527_u32;
Goto(bb40)
}
bb40 = {
(*_23) = _41;
(*_44) = !Field::<u16>(Variant(_40.2, 1), 3);
_4 = _46 as i32;
(*_37) = -Field::<isize>(Variant(_40.2, 1), 2);
(*_44) = Field::<u16>(Variant(_40.2, 1), 3) * Field::<u16>(Variant(_40.2, 1), 3);
_57.fld0 = (*_44) << (*_44);
_49 = core::ptr::addr_of_mut!(_40.3);
(*_39) = !4285888648_u32;
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
(*_37) = _72;
match Field::<u16>(Variant(_40.2, 1), 3) {
0 => bb35,
1 => bb2,
2 => bb33,
3 => bb38,
4 => bb41,
5 => bb42,
6 => bb43,
5636 => bb45,
_ => bb44
}
}
bb41 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb42 = {
(*_23) = _32;
(*_23) = _32;
(*_39) = 2621725460_u32 - 420620653_u32;
_72 = _45 + (*_37);
(*_37) = (*_23) as isize;
_21 = 7158224352778285918_u64 ^ 10438520362827020137_u64;
_2 = _12;
Goto(bb39)
}
bb43 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb44 = {
(*_6) = _5;
_22 = [253_u8,6_u8];
(*_6) = _5;
(*_6) = _3;
_22 = [3_u8,86_u8];
match _21 {
0 => bb7,
3928788315817448533 => bb9,
_ => bb8
}
}
bb45 = {
(*_44) = _57.fld0;
_48.0 = Adt23::Variant1 { fld0: (*_37) };
_51 = _17;
_24 = _11 - _20;
_5 = (*_23);
_20 = _31;
(*_23) = _2;
Call(_57.fld2 = core::intrinsics::transmute(_43), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
(*_39) = 1887137499_u32;
_39 = core::ptr::addr_of_mut!((*_39));
_1 = core::ptr::addr_of_mut!((*_49));
_56 = (*_37);
_15 = [_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4];
Goto(bb47)
}
bb47 = {
_34 = Move(Field::<*mut char>(Variant(_40.2, 1), 0));
(*_1) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
(*_39) = !2359937704_u32;
_57.fld3 = Move(Field::<*mut u32>(Variant(_40.2, 1), 1));
(*_37) = (*_23) as isize;
place!(Field::<*mut char>(Variant(_40.2, 1), 0)) = core::ptr::addr_of_mut!(_43);
_72 = _45 - (*_37);
(*_39) = 1629562248_u32;
_51 = _17;
_8 = [_21,_21,_21,_21,_21,_21,_21];
_5 = (*_23);
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
match (*_39) {
0 => bb9,
1 => bb48,
2 => bb49,
3 => bb50,
4 => bb51,
5 => bb52,
6 => bb53,
1629562248 => bb55,
_ => bb54
}
}
bb48 = {
(*_39) = 1887137499_u32;
_39 = core::ptr::addr_of_mut!((*_39));
_1 = core::ptr::addr_of_mut!((*_49));
_56 = (*_37);
_15 = [_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4,_57.fld4];
Goto(bb47)
}
bb49 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb50 = {
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
(*_6) = _3;
_5 = (*_6);
(*_6) = _3;
_12 = (*_6);
(*_6) = _12;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _12;
_8 = [7251604931694582827_u64,8844659012619501817_u64,3285513450167406935_u64,4950024139319246530_u64,4893026647631178653_u64,4491989828923658745_u64,4226258632304920024_u64];
(*_6) = _5;
(*_6) = _5;
(*_6) = _3;
(*_6) = _5;
_9 = (*_6) == (*_6);
_13 = !9223372036854775807_isize;
(*_6) = _5;
(*_6) = _5;
(*_6) = _5;
Call((*_6) = fn5(Move(_1), _4, Move(_6), _5, _5, _12, _3, _5, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb51 = {
_15 = [_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4,_30.fld4];
_32 = (*_23);
_33 = _29;
_4 = (-573735186_i32);
_30.fld0 = !37751_u16;
(*_23) = _5;
_30.fld4 = _4 as usize;
_40.1 = _4 - _4;
_29 = _30.fld4 as isize;
_5 = (*_23);
(*_25) = core::ptr::addr_of_mut!((*_23));
_8 = [_21,_21,_21,_21,_21,_21,_21];
_21 = 12543698166630127397_u64;
_18 = &_35;
(*_25) = core::ptr::addr_of_mut!((*_23));
_34 = core::ptr::addr_of_mut!(_2);
(*_23) = (*_34);
(*_34) = _12;
_41 = (*_23);
(*_23) = (*_34);
_30.fld0 = 24428_i16 as u16;
(*_25) = core::ptr::addr_of_mut!((*_23));
(*_34) = _12;
(*_23) = (*_34);
_44 = &mut _30.fld0;
match _4 {
0 => bb12,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
340282366920938463463374607431194476270 => bb22,
_ => bb21
}
}
bb52 = {
(*_6) = _5;
_5 = _3;
(*_6) = _12;
(*_6) = _12;
(*_6) = _5;
(*_6) = _3;
_24 = _11;
_23 = core::ptr::addr_of!(_2);
(*_23) = _5;
(*_6) = _12;
(*_6) = _3;
_10 = !_9;
(*_6) = _5;
(*_6) = _12;
(*_6) = _3;
_11 = -_20;
_21 = 16184933427735390726_u64 | 13261376371212823650_u64;
(*_6) = _12;
(*_6) = _12;
_24 = -_11;
_11 = _24;
_22 = [239_u8,165_u8];
_25 = &mut _6;
(*_25) = core::ptr::addr_of_mut!(_5);
(*_25) = core::ptr::addr_of_mut!(_12);
_10 = _9 ^ _9;
(*_25) = core::ptr::addr_of_mut!(_3);
_4 = 865898306_i32 * 105941003_i32;
Goto(bb10)
}
bb53 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb54 = {
(*_6) = _3;
_9 = _10;
(*_6) = _12;
(*_6) = _3;
_3 = (*_6);
(*_6) = _3;
_15 = [0_usize,3914796352864647056_usize,4_usize,14236868325519273645_usize,15741645461947773748_usize,12517950146247721782_usize,7_usize];
(*_6) = _3;
(*_6) = _3;
_6 = core::ptr::addr_of_mut!(_5);
_6 = core::ptr::addr_of_mut!(_2);
_2 = _5;
(*_6) = _5;
(*_6) = _5;
_4 = 498851775_i32 << _13;
(*_6) = _5;
(*_6) = _5;
_11 = -_20;
_3 = _12;
(*_6) = _3;
(*_6) = _5;
_21 = 3928788315817448533_u64;
(*_6) = _12;
Goto(bb6)
}
bb55 = {
(*_23) = _2;
(*_39) = 3317011486_u32 >> _48.1;
_48.1 = _46 + _46;
(*_37) = (-2355641775812336787364313596704653677_i128) as isize;
_74 = _56 >= (*_37);
(*_23) = _2;
(*_44) = !_57.fld0;
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
_70 = core::ptr::addr_of_mut!(_60);
_79 = (*_44);
_51 = _17;
_59 = (*_44) << _56;
_47.fld1 = [_40.1];
(*_37) = !_13;
match Field::<u16>(Variant(_40.2, 1), 3) {
5636 => bb56,
_ => bb42
}
}
bb56 = {
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
_39 = core::ptr::addr_of_mut!((*_39));
(*_37) = (*_44) as isize;
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
_38 = &(*_28);
(*_49) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_40.2, 1), 4)));
_27 = (*_23);
(*_39) = !1836405639_u32;
(*_44) = (-3068723401590339908_i64) as u16;
_71 = (*_23);
_39 = Move(_57.fld3);
(*_37) = 7372888853907423543_i64 as isize;
_40.2 = Adt55::Variant1 { fld0: Move(_34),fld1: Move(_39),fld2: _13,fld3: (*_44),fld4: (-23288_i16) };
_65 = !Field::<isize>(Variant(_48.0, 1), 0);
_7 = &mut (*_49);
_4 = (-115003323_i32) + (-1939235258_i32);
(*_44) = !_59;
(*_23) = _41;
(*_23) = _41;
(*_44) = _59;
_71 = (*_23);
_46 = _58 as u128;
(*_37) = Field::<isize>(Variant(_48.0, 1), 0) - _72;
Goto(bb57)
}
bb57 = {
_89 = !(-4965702010504267428_i64);
(*_37) = _65 - _56;
(*_37) = _35 as isize;
Goto(bb58)
}
bb58 = {
_75 = &_10;
_31 = _35 as f32;
(*_23) = _5;
_81 = _57.fld1 << (*_28);
RET = Adt43::Variant3 { fld0: Move(_54),fld1: _57.fld2,fld2: _48.0,fld3: _35 };
_54 = core::ptr::addr_of_mut!(_81);
(*_44) = !_59;
(*_23) = _71;
_72 = _65 >> (*_44);
(*_44) = _57.fld0 >> _65;
_91.1 = _58 as u128;
_91 = _48;
_83 = !_21;
(*_54) = _31 as u32;
place!(Field::<i8>(Variant(RET, 3), 3)) = !_35;
(*_37) = _83 as isize;
_22 = [(*_28),(*_28)];
(*_37) = _65;
(*_44) = (*_75) as u16;
place!(Field::<isize>(Variant(place!(Field::<Adt23>(Variant(RET, 3), 2)), 1), 0)) = 71279792997195252443910399690416175365_i128 as isize;
_44 = &mut _57.fld0;
(*_37) = _72 - _56;
_2 = _12;
(*_37) = _36 ^ _13;
(*_23) = _3;
(*_37) = _48.1 as isize;
(*_44) = _59 | _79;
(*_23) = _3;
_79 = (*_44) ^ (*_44);
Goto(bb59)
}
bb59 = {
Call(_95 = dump_var(Move(_27), Move(_5), Move(_46), Move(_13)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_95 = dump_var(Move(_29), Move(_59), Move(_71), Move(_43)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_95 = dump_var(Move(_12), Move(_41), Move(_52), Move(_17)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_95 = dump_var(Move(_2), Move(_36), Move(_79), Move(_65)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Call(_95 = dump_var(Move(_89), Move(_45), _96, _96), ReturnTo(bb64), UnwindUnreachable())
}
bb64 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *mut *mut i16,mut _2: i32,mut _3: *mut char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char) -> char {
mir! {
type RET = char;
let _11: isize;
let _12: ([char; 4], u8, [char; 4]);
let _13: *const u128;
let _14: char;
let _15: &'static mut *mut i16;
let _16: *mut &'static mut u8;
let _17: Adt44;
let _18: *mut *mut &'static f64;
let _19: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _20: Adt44;
let _21: i32;
let _22: isize;
let _23: isize;
let _24: i128;
let _25: u32;
let _26: *mut char;
let _27: bool;
let _28: &'static mut *const i16;
let _29: [bool; 2];
let _30: (*mut *mut i16, i32, Adt55, *mut i16);
let _31: u128;
let _32: bool;
let _33: [i128; 2];
let _34: *const &'static mut u8;
let _35: char;
let _36: &'static mut &'static f64;
let _37: u16;
let _38: isize;
let _39: isize;
let _40: u16;
let _41: f64;
let _42: (&'static f64, [u32; 7]);
let _43: u8;
let _44: &'static mut u8;
let _45: char;
let _46: Adt75;
let _47: &'static mut &'static usize;
let _48: *mut &'static f64;
let _49: [bool; 2];
let _50: &'static &'static mut *mut char;
let _51: isize;
let _52: i32;
let _53: usize;
let _54: usize;
let _55: f32;
let _56: bool;
let _57: f32;
let _58: ([char; 4], u8, [char; 4]);
let _59: u128;
let _60: f64;
let _61: char;
let _62: u32;
let _63: *const i16;
let _64: Adt41;
let _65: i16;
let _66: (Adt28, &'static mut u16);
let _67: &'static u8;
let _68: [i8; 8];
let _69: u64;
let _70: [i128; 2];
let _71: isize;
let _72: Adt44;
let _73: i64;
let _74: u32;
let _75: isize;
let _76: &'static &'static mut u8;
let _77: &'static mut *mut char;
let _78: u64;
let _79: ();
let _80: ();
{
_7 = _5;
_9 = _10;
_8 = _5;
_2 = (-1547581515_i32);
_7 = _10;
_8 = _5;
_4 = _5;
_6 = _4;
_11 = 1851957422580593679_u64 as isize;
_10 = _7;
_12.0 = [_9,_4,_7,_5];
_7 = _9;
RET = _6;
_7 = _6;
_12.2 = [_6,RET,_5,_9];
_12.1 = 233_u8;
_2 = (-163608542_i32);
_8 = RET;
_12.1 = 201_u8 & 185_u8;
_12.2 = [_4,RET,_5,RET];
_10 = _7;
_5 = _4;
RET = _9;
_10 = _6;
_2 = 1980455930_i32;
_14 = _8;
Call(_1 = fn6(Move(_3), _12, _2, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 69_isize;
_4 = _14;
_12.1 = 132_u8 + 52_u8;
_8 = _6;
_12.2 = [_4,RET,_14,RET];
_3 = core::ptr::addr_of_mut!(_14);
(*_3) = _10;
(*_3) = _5;
(*_3) = _5;
(*_3) = _4;
(*_3) = _6;
RET = _9;
_4 = _7;
(*_3) = RET;
(*_3) = _9;
(*_3) = _5;
(*_3) = _8;
(*_3) = _7;
_7 = (*_3);
_12.0 = [(*_3),_10,(*_3),_8];
_4 = _14;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1980455930 => bb8,
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
(*_3) = _9;
(*_3) = _10;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
_6 = (*_3);
_2 = 1568999038_i32;
_9 = (*_3);
(*_3) = _4;
_4 = _9;
(*_3) = _7;
_5 = _8;
RET = _14;
(*_3) = _10;
Goto(bb9)
}
bb9 = {
(*_3) = _10;
_12.2 = _12.0;
(*_3) = RET;
_8 = (*_3);
(*_3) = _9;
_5 = RET;
_13 = core::ptr::addr_of!(_19.2.1);
(*_3) = _10;
(*_13) = 285537725152172399508833229384758433476_u128 >> _11;
match _11 {
0 => bb8,
69 => bb11,
_ => bb10
}
}
bb10 = {
(*_3) = _9;
(*_3) = _10;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
_6 = (*_3);
_2 = 1568999038_i32;
_9 = (*_3);
(*_3) = _4;
_4 = _9;
(*_3) = _7;
_5 = _8;
RET = _14;
(*_3) = _10;
Goto(bb9)
}
bb11 = {
_7 = (*_3);
_8 = (*_3);
(*_3) = _6;
_18 = core::ptr::addr_of_mut!(_19.3);
(*_3) = _10;
(*_13) = 236140031984238455190064257470061814640_u128 & 9833072888213786638822469066908898624_u128;
(*_13) = 317588573591270636476802402735768311458_u128 - 249321320054092253957136665240104342662_u128;
_19.2.0 = Adt23::Variant1 { fld0: _11 };
(*_13) = !64829962250314796160912321614936733663_u128;
(*_13) = 115286234516574696961623029379762409398_u128 + 266976720549518562716322859814785594022_u128;
(*_3) = RET;
(*_3) = RET;
(*_13) = 51066764618599828299969421525845314066_u128 * 2441113576421613364161203575200713322_u128;
_18 = core::ptr::addr_of_mut!((*_18));
(*_13) = 135914635059944646915449541541366815654_u128 * 216335701984518013976766845877763926021_u128;
(*_3) = _6;
match Field::<isize>(Variant(_19.2.0, 1), 0) {
0 => bb3,
1 => bb10,
2 => bb12,
3 => bb13,
4 => bb14,
69 => bb16,
_ => bb15
}
}
bb12 = {
(*_3) = _9;
(*_3) = _10;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
_6 = (*_3);
_2 = 1568999038_i32;
_9 = (*_3);
(*_3) = _4;
_4 = _9;
(*_3) = _7;
_5 = _8;
RET = _14;
(*_3) = _10;
Goto(bb9)
}
bb13 = {
Return()
}
bb14 = {
(*_3) = _9;
(*_3) = _10;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
_6 = (*_3);
_2 = 1568999038_i32;
_9 = (*_3);
(*_3) = _4;
_4 = _9;
(*_3) = _7;
_5 = _8;
RET = _14;
(*_3) = _10;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
(*_13) = 238154131898921021949204470224454739625_u128;
(*_13) = 216772318808533324449559451265796189928_u128 ^ 67051322085667344590712370808219994763_u128;
RET = _7;
_12.2 = [_8,(*_3),(*_3),(*_3)];
_3 = core::ptr::addr_of_mut!((*_3));
(*_13) = 30202252001690664640260035299617052818_u128 << _11;
match _11 {
0 => bb7,
69 => bb18,
_ => bb17
}
}
bb17 = {
Return()
}
bb18 = {
_14 = _7;
_3 = core::ptr::addr_of_mut!((*_3));
_12.1 = 113_u8;
(*_3) = _6;
(*_13) = !146466712971604314247510670094674534070_u128;
(*_13) = 192491345597079798328906692086018012814_u128 * 319913532347656440517493758967938518427_u128;
(*_13) = Field::<isize>(Variant(_19.2.0, 1), 0) as u128;
(*_3) = _5;
(*_3) = _10;
_4 = _9;
(*_13) = (-8745877767386424862_i64) as u128;
_7 = (*_3);
(*_13) = 241846537892007731514956426788002459507_u128;
(*_3) = _7;
(*_3) = _10;
_25 = !3280963658_u32;
match (*_13) {
0 => bb8,
1 => bb6,
2 => bb11,
3 => bb16,
241846537892007731514956426788002459507 => bb19,
_ => bb13
}
}
bb19 = {
_12.2 = [_5,_10,_9,(*_3)];
_11 = !Field::<isize>(Variant(_19.2.0, 1), 0);
RET = (*_3);
(*_3) = _9;
_7 = (*_3);
(*_3) = _5;
(*_13) = 145334095757553796953343305958120891111_u128 + 163054460737935724912253234009906820103_u128;
_23 = -_11;
(*_3) = _10;
_21 = _2 | _2;
(*_3) = _5;
(*_3) = _10;
(*_13) = !229252899696615881419253613868670941647_u128;
match _2 {
0 => bb9,
1 => bb3,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
1568999038 => bb26,
_ => bb25
}
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
(*_13) = 238154131898921021949204470224454739625_u128;
(*_13) = 216772318808533324449559451265796189928_u128 ^ 67051322085667344590712370808219994763_u128;
RET = _7;
_12.2 = [_8,(*_3),(*_3),(*_3)];
_3 = core::ptr::addr_of_mut!((*_3));
(*_13) = 30202252001690664640260035299617052818_u128 << _11;
match _11 {
0 => bb7,
69 => bb18,
_ => bb17
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
Return()
}
bb26 = {
(*_3) = _5;
_23 = -Field::<isize>(Variant(_19.2.0, 1), 0);
_26 = core::ptr::addr_of_mut!((*_3));
(*_3) = _6;
(*_13) = 204900570648336018032707396885950473579_u128;
_27 = true;
_10 = (*_3);
(*_3) = _5;
(*_13) = 14861737991410437076_u64 as u128;
(*_3) = _8;
_19.0 = 56271_u16 - 57942_u16;
(*_13) = 310671586923593372058061592801403798159_u128 & 246442482891242814177404634271109260147_u128;
(*_13) = _27 as u128;
_18 = core::ptr::addr_of_mut!((*_18));
(*_3) = _10;
_19.0 = 8411_u16 * 48806_u16;
(*_3) = _4;
(*_13) = !281799365085892798455947106685172738992_u128;
(*_13) = 311392031413430679859271190592626653859_u128 * 323502587809661117757119716759752384938_u128;
_23 = -_11;
_22 = _11 ^ _23;
(*_13) = 15144_i16 as u128;
_27 = (*_3) == _5;
(*_3) = _6;
_5 = _4;
(*_13) = 14745708825347057913_u64 as u128;
match _2 {
0 => bb10,
1 => bb21,
2 => bb13,
1568999038 => bb27,
_ => bb17
}
}
bb27 = {
(*_3) = _7;
_24 = -(-151259263244045288074206624741380437757_i128);
(*_13) = 88199031064121509191199238740850782931_u128 << Field::<isize>(Variant(_19.2.0, 1), 0);
_21 = _2;
_24 = 85684748276423500691714515556000832284_i128 * (-122952010922601256228356463951478592067_i128);
(*_13) = 94757645553551701363129472351499394624_u128 - 97593109421622060847787790437023325970_u128;
(*_13) = !93795371006036626283556357033418178848_u128;
_14 = RET;
(*_3) = _4;
(*_3) = _8;
_24 = 47895893241224036341229425245315458522_i128 << (*_13);
RET = (*_3);
_9 = (*_3);
_5 = _6;
_11 = !Field::<isize>(Variant(_19.2.0, 1), 0);
(*_3) = _4;
_10 = (*_3);
(*_13) = 101351217501089712802339971179480749813_u128 >> _21;
(*_13) = 302889836722287217128905540049712318324_u128 | 41694433312516875386349053095812503289_u128;
(*_13) = 202420801610730019555714024957464359666_u128;
Goto(bb28)
}
bb28 = {
(*_13) = _19.0 as u128;
_32 = !_27;
_25 = 1556483633_u32;
(*_13) = 126598178363770875025829840582563992196_u128;
_5 = (*_3);
_35 = (*_3);
_13 = core::ptr::addr_of!((*_13));
_21 = -_2;
Goto(bb29)
}
bb29 = {
_25 = 449533304_u32 + 1347618798_u32;
_26 = core::ptr::addr_of_mut!((*_3));
(*_13) = 103977046925056519833573297388373245420_u128 >> Field::<isize>(Variant(_19.2.0, 1), 0);
_1 = core::ptr::addr_of_mut!(_30.3);
(*_3) = _35;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
(*_13) = 284683145774218285023965986764999882471_u128 << _25;
(*_13) = 191309119921533528090078980642728329762_u128 >> _19.0;
(*_3) = _10;
_30.1 = _21 | _21;
_1 = core::ptr::addr_of_mut!((*_1));
(*_3) = _10;
(*_13) = (*_3) as u128;
(*_13) = 167979644754025878065532527601539272466_u128;
(*_13) = _32 as u128;
(*_3) = _7;
(*_3) = _9;
_18 = core::ptr::addr_of_mut!((*_18));
_33 = [_24,_24];
(*_13) = 224811437322711707183897597046053652280_u128;
(*_3) = _8;
(*_3) = _6;
(*_13) = _12.1 as u128;
_30.1 = _21 - _21;
_37 = _19.0 & _19.0;
(*_13) = !198982795765329778674742905917591836790_u128;
(*_3) = _35;
_27 = (*_13) >= (*_13);
_6 = (*_3);
match Field::<isize>(Variant(_19.2.0, 1), 0) {
0 => bb26,
1 => bb12,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
69 => bb35,
_ => bb34
}
}
bb30 = {
_11 = 69_isize;
_4 = _14;
_12.1 = 132_u8 + 52_u8;
_8 = _6;
_12.2 = [_4,RET,_14,RET];
_3 = core::ptr::addr_of_mut!(_14);
(*_3) = _10;
(*_3) = _5;
(*_3) = _5;
(*_3) = _4;
(*_3) = _6;
RET = _9;
_4 = _7;
(*_3) = RET;
(*_3) = _9;
(*_3) = _5;
(*_3) = _8;
(*_3) = _7;
_7 = (*_3);
_12.0 = [(*_3),_10,(*_3),_8];
_4 = _14;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1980455930 => bb8,
_ => bb7
}
}
bb31 = {
(*_3) = _10;
_12.2 = _12.0;
(*_3) = RET;
_8 = (*_3);
(*_3) = _9;
_5 = RET;
_13 = core::ptr::addr_of!(_19.2.1);
(*_3) = _10;
(*_13) = 285537725152172399508833229384758433476_u128 >> _11;
match _11 {
0 => bb8,
69 => bb11,
_ => bb10
}
}
bb32 = {
(*_3) = _5;
_23 = -Field::<isize>(Variant(_19.2.0, 1), 0);
_26 = core::ptr::addr_of_mut!((*_3));
(*_3) = _6;
(*_13) = 204900570648336018032707396885950473579_u128;
_27 = true;
_10 = (*_3);
(*_3) = _5;
(*_13) = 14861737991410437076_u64 as u128;
(*_3) = _8;
_19.0 = 56271_u16 - 57942_u16;
(*_13) = 310671586923593372058061592801403798159_u128 & 246442482891242814177404634271109260147_u128;
(*_13) = _27 as u128;
_18 = core::ptr::addr_of_mut!((*_18));
(*_3) = _10;
_19.0 = 8411_u16 * 48806_u16;
(*_3) = _4;
(*_13) = !281799365085892798455947106685172738992_u128;
(*_13) = 311392031413430679859271190592626653859_u128 * 323502587809661117757119716759752384938_u128;
_23 = -_11;
_22 = _11 ^ _23;
(*_13) = 15144_i16 as u128;
_27 = (*_3) == _5;
(*_3) = _6;
_5 = _4;
(*_13) = 14745708825347057913_u64 as u128;
match _2 {
0 => bb10,
1 => bb21,
2 => bb13,
1568999038 => bb27,
_ => bb17
}
}
bb33 = {
Return()
}
bb34 = {
Return()
}
bb35 = {
(*_3) = _7;
(*_13) = 61294864100676078283906108393839818500_u128 << _12.1;
_19.0 = _37 ^ _37;
(*_3) = _9;
_2 = -_21;
_39 = _23;
(*_3) = _8;
_10 = (*_3);
(*_13) = 313182405826983927541989243464180192666_u128 >> _23;
(*_13) = 301482079179107728162453016618116256841_u128 * 268470300451487335409279747339634148378_u128;
(*_3) = _7;
(*_3) = _8;
_4 = (*_3);
match Field::<isize>(Variant(_19.2.0, 1), 0) {
69 => bb36,
_ => bb8
}
}
bb36 = {
(*_3) = _35;
(*_13) = 40589145340610784480118892256473625625_u128;
_30.0 = core::ptr::addr_of_mut!((*_1));
_7 = (*_3);
(*_3) = _8;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _7;
(*_3) = _4;
(*_3) = _9;
_31 = !(*_13);
_35 = (*_3);
(*_3) = RET;
_9 = (*_3);
_43 = _12.1;
match (*_13) {
40589145340610784480118892256473625625 => bb37,
_ => bb15
}
}
bb37 = {
_29 = [_32,_27];
_40 = _19.0 ^ _19.0;
_3 = core::ptr::addr_of_mut!(_8);
(*_13) = _31 * _31;
(*_3) = _10;
_31 = (*_13) ^ (*_13);
_33 = [_24,_24];
_8 = _7;
_13 = core::ptr::addr_of!((*_13));
_12.1 = _43 ^ _43;
(*_13) = _31;
_19.0 = !_40;
_25 = 3721346594_u32 << _31;
match _43 {
0 => bb10,
1 => bb17,
113 => bb38,
_ => bb27
}
}
bb38 = {
(*_3) = _10;
(*_3) = _4;
(*_13) = _31;
_42.1 = [_25,_25,_25,_25,_25,_25,_25];
(*_13) = !_31;
(*_13) = _31 - _31;
(*_3) = _14;
_14 = (*_3);
(*_13) = 7_usize as u128;
_40 = _19.0 * _19.0;
_14 = (*_3);
_19.0 = _40 & _40;
(*_3) = _14;
(*_13) = _32 as u128;
_37 = !_40;
_18 = core::ptr::addr_of_mut!((*_18));
_12.1 = (-9152916054989763315_i64) as u8;
_49 = _29;
_44 = &mut _12.1;
Goto(bb39)
}
bb39 = {
_41 = 12_i8 as f64;
_19.2.0 = Adt23::Variant2 { fld0: _19.0,fld1: _24 };
(*_13) = _31 & _31;
(*_44) = !_43;
match _43 {
0 => bb40,
1 => bb41,
113 => bb43,
_ => bb42
}
}
bb40 = {
(*_3) = _9;
(*_3) = _10;
_12.2 = [(*_3),(*_3),(*_3),(*_3)];
_6 = (*_3);
_2 = 1568999038_i32;
_9 = (*_3);
(*_3) = _4;
_4 = _9;
(*_3) = _7;
_5 = _8;
RET = _14;
(*_3) = _10;
Goto(bb9)
}
bb41 = {
(*_3) = _7;
(*_13) = 61294864100676078283906108393839818500_u128 << _12.1;
_19.0 = _37 ^ _37;
(*_3) = _9;
_2 = -_21;
_39 = _23;
(*_3) = _8;
_10 = (*_3);
(*_13) = 313182405826983927541989243464180192666_u128 >> _23;
(*_13) = 301482079179107728162453016618116256841_u128 * 268470300451487335409279747339634148378_u128;
(*_3) = _7;
(*_3) = _8;
_4 = (*_3);
match Field::<isize>(Variant(_19.2.0, 1), 0) {
69 => bb36,
_ => bb8
}
}
bb42 = {
Return()
}
bb43 = {
(*_44) = (*_3) as u8;
_30.1 = !_2;
(*_13) = _31 ^ _31;
_21 = _2;
_40 = 5083427706992319096_usize as u16;
(*_3) = _10;
(*_3) = _5;
(*_3) = _9;
_24 = Field::<i128>(Variant(_19.2.0, 2), 1) - Field::<i128>(Variant(_19.2.0, 2), 1);
_21 = _23 as i32;
place!(Field::<i128>(Variant(_19.2.0, 2), 1)) = 11060893061107406074_usize as i128;
_31 = (*_13);
_44 = &mut _43;
(*_3) = _14;
_56 = _27 ^ _32;
_4 = (*_3);
match (*_44) {
0 => bb11,
1 => bb34,
113 => bb45,
_ => bb44
}
}
bb44 = {
Return()
}
bb45 = {
(*_44) = _11 as u8;
_39 = _22 << (*_13);
(*_44) = 94_u8;
_30.0 = core::ptr::addr_of_mut!((*_1));
(*_44) = 98_u8 | 54_u8;
_14 = (*_3);
(*_13) = _31 & _31;
_29 = [_27,_56];
_57 = Field::<u16>(Variant(_19.2.0, 2), 0) as f32;
Goto(bb46)
}
bb46 = {
(*_44) = _19.0 as u8;
_7 = (*_3);
_9 = (*_3);
_34 = core::ptr::addr_of!(_44);
_8 = _10;
(*_13) = _31 >> (*_44);
(*_44) = 95_u8;
_7 = _14;
_58.1 = (*_44);
(*_34) = &mut _58.1;
(*_44) = !63_u8;
(*_3) = _7;
(*_13) = _31 - _31;
_29 = _49;
(*_44) = 173_u8;
_56 = _27 ^ _27;
(*_3) = _6;
(*_13) = _31;
_5 = (*_3);
_5 = (*_3);
Goto(bb47)
}
bb47 = {
(*_13) = _31 << _37;
(*_3) = _10;
_16 = core::ptr::addr_of_mut!((*_34));
_7 = (*_3);
_35 = (*_3);
(*_44) = 123_u8;
(*_44) = 89_u8 & 114_u8;
(*_44) = !67_u8;
RET = (*_3);
(*_44) = !203_u8;
(*_1) = core::ptr::addr_of_mut!(_65);
(*_1) = core::ptr::addr_of_mut!(_65);
(*_1) = core::ptr::addr_of_mut!(_65);
_7 = (*_3);
(*_1) = core::ptr::addr_of_mut!(_65);
(*_44) = 88_u8;
(*_1) = core::ptr::addr_of_mut!(_65);
_51 = -_39;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = core::ptr::addr_of_mut!(_65);
_66.0.fld2 = _57;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = core::ptr::addr_of_mut!(_65);
(*_1) = core::ptr::addr_of_mut!(_65);
_66.0.fld1 = _25;
Goto(bb48)
}
bb48 = {
_54 = 18020707113602182471_usize;
_33 = [_24,_24];
_7 = (*_3);
(*_3) = _4;
_61 = (*_3);
(*_13) = _31 * _31;
_13 = core::ptr::addr_of!((*_13));
_61 = (*_3);
(*_1) = core::ptr::addr_of_mut!(_65);
_13 = core::ptr::addr_of!((*_13));
_65 = 25366_i16 >> _19.0;
(*_44) = _25 as u8;
(*_1) = core::ptr::addr_of_mut!(_65);
_21 = _30.1 >> Field::<u16>(Variant(_19.2.0, 2), 0);
_30.0 = core::ptr::addr_of_mut!((*_1));
_26 = core::ptr::addr_of_mut!(_9);
(*_26) = RET;
_65 = (-8061_i16) | 17323_i16;
(*_44) = _65 as u8;
(*_44) = 181_u8 | 248_u8;
_30.0 = core::ptr::addr_of_mut!((*_1));
(*_1) = core::ptr::addr_of_mut!(_65);
(*_13) = (*_44) as u128;
(*_1) = core::ptr::addr_of_mut!(_65);
(*_44) = !77_u8;
_7 = _9;
Goto(bb49)
}
bb49 = {
_49 = [_56,_32];
_18 = core::ptr::addr_of_mut!((*_18));
_75 = _22 * _39;
(*_44) = 128_u8;
_9 = _4;
_49 = _29;
(*_44) = _21 as u8;
(*_13) = _31;
(*_26) = (*_3);
(*_3) = (*_26);
_62 = _25 << (*_44);
(*_26) = RET;
(*_3) = (*_26);
_2 = _21 ^ _21;
_38 = _51;
Goto(bb50)
}
bb50 = {
Call(_79 = dump_var(Move(_8), Move(_37), Move(_21), Move(_38)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_79 = dump_var(Move(_25), Move(_24), Move(_4), Move(_23)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_79 = dump_var(Move(_7), Move(_65), Move(_10), Move(_35)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_79 = dump_var(Move(_49), Move(_56), Move(_2), Move(_43)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_79 = dump_var(Move(_29), _80, _80, _80), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut char,mut _2: ([char; 4], u8, [char; 4]),mut _3: i32,mut _4: char) -> *mut *mut i16 {
mir! {
type RET = *mut *mut i16;
let _5: i128;
let _6: char;
let _7: u16;
let _8: &'static mut u16;
let _9: u32;
let _10: i16;
let _11: *const u128;
let _12: bool;
let _13: *mut i16;
let _14: isize;
let _15: i8;
let _16: isize;
let _17: &'static u8;
let _18: [usize; 7];
let _19: i32;
let _20: f32;
let _21: u32;
let _22: &'static mut *mut i16;
let _23: *mut *mut &'static f64;
let _24: (Adt23, u128);
let _25: &'static mut isize;
let _26: char;
let _27: [usize; 7];
let _28: u8;
let _29: *mut ([char; 4], u8, [char; 4]);
let _30: [bool; 2];
let _31: (Adt28, &'static mut u16);
let _32: &'static mut isize;
let _33: &'static mut i8;
let _34: *const &'static bool;
let _35: f64;
let _36: i16;
let _37: u8;
let _38: [bool; 2];
let _39: *const &'static mut &'static f64;
let _40: *mut char;
let _41: &'static mut isize;
let _42: &'static mut &'static f64;
let _43: i8;
let _44: *mut *mut &'static f64;
let _45: char;
let _46: &'static mut i8;
let _47: &'static mut &'static f64;
let _48: (u128, usize, [i32; 1]);
let _49: *mut &'static mut u8;
let _50: char;
let _51: bool;
let _52: isize;
let _53: ([char; 4], [u8; 2], &'static mut isize, char);
let _54: isize;
let _55: *mut *mut &'static f64;
let _56: &'static mut &'static usize;
let _57: char;
let _58: ([char; 4], [u8; 2], &'static mut isize, char);
let _59: isize;
let _60: u16;
let _61: Adt75;
let _62: &'static mut *mut char;
let _63: &'static usize;
let _64: *const &'static mut &'static f64;
let _65: &'static u8;
let _66: *mut u32;
let _67: *mut [u32; 7];
let _68: u64;
let _69: char;
let _70: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _71: ();
let _72: ();
{
_2.0 = [_4,_4,_4,_4];
_5 = 98434516823185513557776363810798234066_i128 & 108647567248617954036763070385666926315_i128;
_4 = '\u{cef27}';
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
1980455930 => bb6,
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
_4 = '\u{86ebf}';
_2.0 = [_4,_4,_4,_4];
_5 = (-18607_i16) as i128;
_4 = '\u{363cc}';
_2.2 = [_4,_4,_4,_4];
_3 = (-1781819686_i32);
_1 = core::ptr::addr_of_mut!(_4);
(*_1) = '\u{ed32e}';
(*_1) = '\u{10ec08}';
(*_1) = '\u{e5407}';
(*_1) = '\u{67567}';
(*_1) = '\u{a0283}';
(*_1) = '\u{bf35f}';
_5 = 62959528041521128864304272449716929608_i128 + 119408774882307853977178052543750160009_i128;
(*_1) = '\u{1d296}';
(*_1) = '\u{99c8f}';
(*_1) = '\u{5a3cd}';
(*_1) = '\u{5fed0}';
(*_1) = '\u{64f39}';
(*_1) = '\u{4d7d8}';
match _3 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607429986391770 => bb12,
_ => bb11
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
_2.0 = [(*_1),_4,(*_1),(*_1)];
_6 = (*_1);
match _3 {
0 => bb11,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
340282366920938463463374607429986391770 => bb13,
_ => bb7
}
}
bb13 = {
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
_2.2 = _2.0;
_2.2 = _2.0;
(*_1) = _6;
(*_1) = _6;
_3 = 1987779390_i32 << _2.1;
_9 = !4190361271_u32;
(*_1) = _6;
_3 = false as i32;
(*_1) = _6;
_7 = 50016_u16 * 29562_u16;
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
_8 = &mut _7;
_3 = -1694528088_i32;
(*_8) = 35334_u16;
(*_8) = 34137_u16 & 4781_u16;
Call((*_1) = fn7((*_8), Move(_8), Move(_1), (*_8), _2.0, (*_8), (*_8), (*_8), _2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_2.0 = _2.2;
_10 = 30809_i16 >> _5;
_1 = core::ptr::addr_of_mut!(_4);
(*_1) = _6;
_3 = (-414605412_i32) - (-123543422_i32);
_12 = false;
(*_1) = _6;
(*_1) = _6;
Goto(bb15)
}
bb15 = {
_10 = !(-9761_i16);
(*_1) = _6;
_6 = (*_1);
(*_1) = _6;
(*_1) = _6;
_4 = _6;
_3 = 2036139986_i32 >> _2.1;
(*_1) = _6;
_13 = core::ptr::addr_of_mut!(_10);
(*_1) = _6;
_12 = false | true;
_2.1 = 123_u8 << (*_13);
(*_1) = _6;
_15 = -(-17_i8);
(*_1) = _6;
Goto(bb16)
}
bb16 = {
(*_13) = (-13779_i16) ^ (-26874_i16);
_2.1 = _9 as u8;
(*_13) = _15 as i16;
Goto(bb17)
}
bb17 = {
_2.1 = !172_u8;
(*_1) = _6;
_3 = (-717227042_i32);
_4 = _6;
(*_13) = 2908_i16 - (-321_i16);
_2.2 = [(*_1),_6,(*_1),(*_1)];
_16 = (-109_isize) - 9223372036854775807_isize;
(*_13) = (-6841_i16) - 8523_i16;
(*_1) = _6;
(*_13) = 15246_i16;
(*_1) = _6;
_18 = [13333748676410530463_usize,7_usize,9394381421230409245_usize,854071596776110267_usize,4574786228934843551_usize,3_usize,1734270680105120354_usize];
_20 = _16 as f32;
(*_1) = _6;
(*_13) = (-29303_i16);
match (*_13) {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
340282366920938463463374607431768182153 => bb25,
_ => bb24
}
}
bb18 = {
Return()
}
bb19 = {
_10 = !(-9761_i16);
(*_1) = _6;
_6 = (*_1);
(*_1) = _6;
(*_1) = _6;
_4 = _6;
_3 = 2036139986_i32 >> _2.1;
(*_1) = _6;
_13 = core::ptr::addr_of_mut!(_10);
(*_1) = _6;
_12 = false | true;
_2.1 = 123_u8 << (*_13);
(*_1) = _6;
_15 = -(-17_i8);
(*_1) = _6;
Goto(bb16)
}
bb20 = {
_4 = '\u{86ebf}';
_2.0 = [_4,_4,_4,_4];
_5 = (-18607_i16) as i128;
_4 = '\u{363cc}';
_2.2 = [_4,_4,_4,_4];
_3 = (-1781819686_i32);
_1 = core::ptr::addr_of_mut!(_4);
(*_1) = '\u{ed32e}';
(*_1) = '\u{10ec08}';
(*_1) = '\u{e5407}';
(*_1) = '\u{67567}';
(*_1) = '\u{a0283}';
(*_1) = '\u{bf35f}';
_5 = 62959528041521128864304272449716929608_i128 + 119408774882307853977178052543750160009_i128;
(*_1) = '\u{1d296}';
(*_1) = '\u{99c8f}';
(*_1) = '\u{5a3cd}';
(*_1) = '\u{5fed0}';
(*_1) = '\u{64f39}';
(*_1) = '\u{4d7d8}';
match _3 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607429986391770 => bb12,
_ => bb11
}
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
(*_13) = !(-19128_i16);
(*_13) = (-1630_i16) | 28049_i16;
(*_1) = _6;
(*_13) = (-15225_i16) + (-14258_i16);
(*_13) = 6776_i16;
(*_13) = !32353_i16;
(*_13) = (-6903_i16) << _15;
(*_13) = 2_usize as i16;
_17 = &_2.1;
(*_1) = _6;
_3 = (-1050453474_i32);
(*_13) = !9389_i16;
_2.0 = _2.2;
_18 = [7_usize,3_usize,3568188524844888135_usize,1513200299358156823_usize,13708794656348786358_usize,10205901038069471127_usize,16953037751468349324_usize];
(*_13) = _16 as i16;
(*_1) = _6;
(*_13) = (-18541_i16);
(*_13) = 1_usize as i16;
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
(*_13) = (-16523_i16) << (*_17);
Goto(bb26)
}
bb26 = {
(*_13) = !18016_i16;
(*_1) = _6;
(*_13) = 8585_i16;
_14 = !_16;
_4 = _6;
(*_1) = _6;
_5 = (*_1) as i128;
_24.0 = Adt23::Variant1 { fld0: _16 };
(*_13) = (*_17) as i16;
_2.2 = [(*_1),(*_1),(*_1),(*_1)];
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
_24.1 = 78028627857321196802915333646132296412_u128;
(*_13) = 21480_i16 ^ (-10851_i16);
_2.1 = _14 as u8;
_6 = (*_1);
_27 = [2_usize,717348138081528589_usize,10417222007027851433_usize,4903194775495388488_usize,0_usize,8291355010343901719_usize,0_usize];
_4 = _6;
_19 = _3 + _3;
Goto(bb27)
}
bb27 = {
(*_1) = _6;
_12 = true;
(*_13) = (-31488_i16);
(*_1) = _6;
(*_1) = _6;
_24.1 = 38899_u16 as u128;
_31.0.fld0 = !293_u16;
_29 = core::ptr::addr_of_mut!(_2);
_31.1 = &mut _31.0.fld0;
(*_1) = _6;
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
match (*_13) {
0 => bb20,
1 => bb14,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
340282366920938463463374607431768179968 => bb33,
_ => bb32
}
}
bb28 = {
Return()
}
bb29 = {
_10 = !(-9761_i16);
(*_1) = _6;
_6 = (*_1);
(*_1) = _6;
(*_1) = _6;
_4 = _6;
_3 = 2036139986_i32 >> _2.1;
(*_1) = _6;
_13 = core::ptr::addr_of_mut!(_10);
(*_1) = _6;
_12 = false | true;
_2.1 = 123_u8 << (*_13);
(*_1) = _6;
_15 = -(-17_i8);
(*_1) = _6;
Goto(bb16)
}
bb30 = {
(*_13) = (-13779_i16) ^ (-26874_i16);
_2.1 = _9 as u8;
(*_13) = _15 as i16;
Goto(bb17)
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
_16 = !Field::<isize>(Variant(_24.0, 1), 0);
RET = core::ptr::addr_of_mut!(_13);
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_29).2 = (*_29).0;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_1) = _6;
_6 = (*_1);
(*_29).2 = [_4,(*_1),(*_1),_6];
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_1) = _6;
(*_29).0 = [_6,(*_1),(*_1),(*_1)];
(*_29).2 = [(*_1),_4,(*_1),(*_1)];
(*_29).2 = [(*_1),_6,(*_1),(*_1)];
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_29).1 = 68_u8 - 153_u8;
_9 = 1062656878_u32 + 1455968762_u32;
(*_13) = _24.1 as i16;
place!(Field::<isize>(Variant(_24.0, 1), 0)) = !_16;
_19 = !_3;
(*_1) = _6;
match _3 {
0 => bb20,
1 => bb6,
2 => bb16,
3 => bb4,
340282366920938463463374607430717757982 => bb35,
_ => bb34
}
}
bb34 = {
(*_13) = (-13779_i16) ^ (-26874_i16);
_2.1 = _9 as u8;
(*_13) = _15 as i16;
Goto(bb17)
}
bb35 = {
_35 = (*_13) as f64;
(*_13) = (-23827_i16);
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
_25 = &mut place!(Field::<isize>(Variant(_24.0, 1), 0));
_25 = &mut _16;
(*_13) = (-22500_i16) ^ 283_i16;
_32 = &mut (*_25);
(*_1) = _6;
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
_6 = (*_1);
(*_29).1 = 207_u8 >> (*_32);
(*_29).1 = 81_u8;
(*_29).2 = (*_29).0;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
_33 = &mut _15;
(*_29).2 = [(*_1),(*_1),(*_1),_4];
(*_29).2 = (*_29).0;
(*_29).1 = !13_u8;
match _3 {
0 => bb25,
1 => bb23,
2 => bb18,
3 => bb4,
4 => bb30,
5 => bb13,
6 => bb36,
340282366920938463463374607430717757982 => bb38,
_ => bb37
}
}
bb36 = {
Return()
}
bb37 = {
(*_13) = !(-19128_i16);
(*_13) = (-1630_i16) | 28049_i16;
(*_1) = _6;
(*_13) = (-15225_i16) + (-14258_i16);
(*_13) = 6776_i16;
(*_13) = !32353_i16;
(*_13) = (-6903_i16) << _15;
(*_13) = 2_usize as i16;
_17 = &_2.1;
(*_1) = _6;
_3 = (-1050453474_i32);
(*_13) = !9389_i16;
_2.0 = _2.2;
_18 = [7_usize,3_usize,3568188524844888135_usize,1513200299358156823_usize,13708794656348786358_usize,10205901038069471127_usize,16953037751468349324_usize];
(*_13) = _16 as i16;
(*_1) = _6;
(*_13) = (-18541_i16);
(*_13) = 1_usize as i16;
(*_1) = _6;
(*_1) = _6;
(*_1) = _6;
(*_13) = (-16523_i16) << (*_17);
Goto(bb26)
}
bb38 = {
(*_29).1 = 138_u8;
_28 = _2.1 & (*_29).1;
(*_13) = _19 as i16;
(*_1) = _6;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_29).1 = _28 >> (*_33);
_20 = (*_13) as f32;
Goto(bb39)
}
bb39 = {
_2.1 = _28;
(*_29).0 = (*_29).2;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*RET) = core::ptr::addr_of_mut!((*_13));
_26 = (*_1);
_32 = &mut _14;
(*_29).1 = _28;
Goto(bb40)
}
bb40 = {
(*_1) = _26;
match _3 {
0 => bb8,
340282366920938463463374607430717757982 => bb42,
_ => bb41
}
}
bb41 = {
_2.0 = _2.2;
_10 = 30809_i16 >> _5;
_1 = core::ptr::addr_of_mut!(_4);
(*_1) = _6;
_3 = (-414605412_i32) - (-123543422_i32);
_12 = false;
(*_1) = _6;
(*_1) = _6;
Goto(bb15)
}
bb42 = {
_29 = core::ptr::addr_of_mut!((*_29));
(*_32) = 4_usize as isize;
(*_29).0 = (*_29).2;
(*_13) = 17872_i16;
(*_1) = _26;
(*_13) = 25239_i16 + 1769_i16;
(*_32) = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
(*_29).0 = _2.2;
_32 = Move(_25);
(*_13) = 27682_i16;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_13) = _35 as i16;
(*_29).1 = !_28;
(*_29).1 = _28 ^ _28;
(*_29).1 = 15083260529018290636_u64 as u8;
(*_33) = _12 as i8;
(*_33) = 78_i8 & (-30_i8);
_46 = &mut (*_33);
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_29).2 = [(*_1),(*_1),_4,(*_1)];
(*_1) = _26;
(*_29).2 = [(*_1),(*_1),_4,(*_1)];
Goto(bb43)
}
bb43 = {
(*_46) = _5 as i8;
(*_46) = !80_i8;
(*RET) = core::ptr::addr_of_mut!((*_13));
_53.1 = [(*_29).1,(*_29).1];
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_1) = _6;
_11 = core::ptr::addr_of!(_48.0);
_12 = (*_46) > (*_46);
(*_29).0 = (*_29).2;
_43 = (*_46) | (*_46);
(*_11) = 205800383522207591237507005871903144600_u128 | 150815045097616510771427890793346968171_u128;
_36 = (*_13) + (*_13);
match _3 {
0 => bb7,
1 => bb40,
2 => bb14,
3 => bb17,
340282366920938463463374607430717757982 => bb44,
_ => bb13
}
}
bb44 = {
(*_1) = _6;
(*_1) = _6;
(*_29).1 = _9 as u8;
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_1) = _26;
_37 = _2.1 + (*_29).1;
(*_29).1 = _5 as u8;
_48.2 = [_3];
(*_11) = (*_13) as u128;
_50 = (*_1);
_17 = &_28;
(*_11) = _19 as u128;
(*_46) = (*_11) as i8;
(*_1) = _6;
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
(*_11) = !14709163575378397379694195845485880924_u128;
(*_1) = _26;
(*_29).1 = _28;
(*_29).1 = !(*_17);
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_1) = _26;
(*_29).1 = (*_17);
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_46) = _20 as i8;
_12 = (*_17) == (*_17);
(*_29).2 = [(*_1),(*_1),(*_1),(*_1)];
Goto(bb45)
}
bb45 = {
(*_29).1 = (*_17);
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_46) = _43 * _43;
(*RET) = core::ptr::addr_of_mut!((*_13));
(*_13) = -_36;
(*_13) = _36 * _36;
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_29).1 = (*_17) ^ (*_17);
_5 = (-2154312779834647909_i64) as i128;
(*_29).1 = 52266_u16 as u8;
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_29).0 = (*_29).2;
(*RET) = core::ptr::addr_of_mut!((*_13));
_58.3 = (*_1);
(*_46) = -_43;
_22 = &mut (*RET);
(*_29).2 = [(*_1),_4,_50,(*_1)];
(*_22) = core::ptr::addr_of_mut!(_36);
(*_29).0 = [(*_1),(*_1),_50,(*_1)];
_48.1 = 7_usize * 5_usize;
_51 = !_12;
(*_29).2 = (*_29).0;
(*_46) = _43;
match _3 {
0 => bb26,
1 => bb46,
340282366920938463463374607430717757982 => bb48,
_ => bb47
}
}
bb46 = {
Return()
}
bb47 = {
Return()
}
bb48 = {
(*_22) = core::ptr::addr_of_mut!(_36);
_30 = [_51,_12];
_53.0 = (*_29).2;
(*_29).0 = [(*_1),(*_1),(*_1),(*_1)];
(*_11) = _20 as u128;
(*_29).1 = (*_17) - (*_17);
(*_29).2 = _53.0;
(*_29).0 = [(*_1),(*_1),_26,_4];
(*_46) = _43;
(*_29).2 = _2.0;
(*_46) = _43;
(*_11) = 251007722438249401052727930423577389087_u128;
Goto(bb49)
}
bb49 = {
(*_46) = 40388_u16 as i8;
_63 = &_48.1;
(*_1) = _26;
(*_46) = _43 - _43;
(*_1) = _58.3;
(*_29).2 = [(*_1),_6,(*_1),_50];
(*_29).2 = (*_29).0;
(*_29).2 = (*_29).0;
_53.0 = [(*_1),(*_1),(*_1),(*_1)];
(*_29) = (_53.0, _37, _53.0);
(*_29).0 = (*_29).2;
_58.3 = (*_1);
_54 = (-60_isize) << (*_63);
(*_1) = _58.3;
_53.2 = &mut _54;
_40 = core::ptr::addr_of_mut!(_26);
(*_46) = _9 as i8;
_65 = &(*_29).1;
Goto(bb50)
}
bb50 = {
Call(_71 = dump_var(Move(_5), Move(_6), Move(_26), Move(_48)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_71 = dump_var(Move(_28), Move(_9), Move(_43), Move(_7)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_71 = dump_var(Move(_37), Move(_10), Move(_18), Move(_19)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u16,mut _2: &'static mut u16,mut _3: *mut char,mut _4: u16,mut _5: [char; 4],mut _6: u16,mut _7: u16,mut _8: u16,mut _9: ([char; 4], u8, [char; 4])) -> char {
mir! {
type RET = char;
let _10: &'static bool;
let _11: *mut &'static mut u8;
let _12: u32;
let _13: isize;
let _14: f32;
let _15: &'static mut *mut i16;
let _16: &'static mut &'static usize;
let _17: bool;
let _18: &'static f32;
let _19: (*mut &'static f64, &'static mut isize);
let _20: &'static &'static mut *mut char;
let _21: f64;
let _22: f64;
let _23: *mut &'static f64;
let _24: [i32; 1];
let _25: [u32; 7];
let _26: &'static f64;
let _27: *mut &'static f64;
let _28: u64;
let _29: *mut &'static f64;
let _30: char;
let _31: char;
let _32: i32;
let _33: ([char; 4], [u8; 2], &'static mut isize, char);
let _34: char;
let _35: i16;
let _36: (u128, usize, [i32; 1]);
let _37: bool;
let _38: f32;
let _39: &'static mut isize;
let _40: f64;
let _41: f64;
let _42: i16;
let _43: isize;
let _44: &'static bool;
let _45: isize;
let _46: isize;
let _47: bool;
let _48: isize;
let _49: &'static mut *mut i16;
let _50: *mut &'static f64;
let _51: [char; 4];
let _52: Adt23;
let _53: (*const i16,);
let _54: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _55: u128;
let _56: f32;
let _57: &'static mut u8;
let _58: isize;
let _59: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _60: f32;
let _61: (*mut *mut i16, i32, Adt55, *mut i16);
let _62: (*mut &'static f64, &'static mut isize);
let _63: f64;
let _64: char;
let _65: u128;
let _66: [u8; 7];
let _67: &'static &'static mut u8;
let _68: [u32; 7];
let _69: bool;
let _70: u128;
let _71: char;
let _72: bool;
let _73: Adt75;
let _74: char;
let _75: i16;
let _76: *const &'static mut &'static f64;
let _77: &'static mut u16;
let _78: f32;
let _79: Adt43;
let _80: u8;
let _81: char;
let _82: u64;
let _83: &'static &'static mut u8;
let _84: &'static usize;
let _85: &'static mut *mut char;
let _86: f32;
let _87: *mut &'static mut u8;
let _88: i32;
let _89: f32;
let _90: (&'static mut &'static f64, i16, usize, [char; 4]);
let _91: usize;
let _92: *const char;
let _93: u8;
let _94: ();
let _95: ();
{
_7 = _4 >> _4;
_9.2 = ['\u{10b8fe}','\u{e172e}','\u{292c3}','\u{dcb72}'];
Goto(bb1)
}
bb1 = {
_9.2 = ['\u{31109}','\u{28daf}','\u{820ee}','\u{471b3}'];
RET = '\u{b4dc0}';
_9.2 = _9.0;
_9.2 = [RET,RET,RET,RET];
_9.2 = [RET,RET,RET,RET];
_1 = _4;
_1 = (-1286242055_i32) as u16;
_3 = core::ptr::addr_of_mut!(RET);
(*_3) = '\u{50c83}';
_9.2 = [(*_3),(*_3),(*_3),(*_3)];
(*_3) = '\u{92ee5}';
_2 = &mut _6;
(*_3) = '\u{7e07d}';
_1 = (*_2);
(*_2) = _7 * _7;
RET = '\u{8359d}';
(*_2) = !_7;
(*_2) = _7;
(*_2) = (-1133151123_i32) as u16;
_8 = (*_2);
_9.1 = 56_u8 << (*_2);
(*_3) = '\u{af7ae}';
_5 = [(*_3),(*_3),(*_3),(*_3)];
_9.0 = [(*_3),(*_3),(*_3),(*_3)];
(*_2) = !_7;
(*_2) = _4 ^ _7;
(*_3) = '\u{92e38}';
Call((*_3) = fn8(Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_2) = _1 << _7;
_3 = core::ptr::addr_of_mut!(RET);
(*_2) = _7;
(*_2) = _1 ^ _7;
(*_2) = _7 + _7;
(*_3) = '\u{ca30a}';
(*_3) = '\u{3cfef}';
(*_3) = '\u{34e5c}';
(*_2) = !_7;
(*_3) = '\u{24ac1}';
(*_3) = '\u{3df5}';
_14 = (*_2) as f32;
_4 = !_8;
(*_3) = '\u{f20cc}';
Goto(bb3)
}
bb3 = {
_13 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
RET = '\u{ece4b}';
_14 = 104_i8 as f32;
(*_2) = _7 & _7;
(*_3) = '\u{1cb1e}';
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = '\u{880e5}';
(*_2) = _4 >> _13;
(*_3) = '\u{b78d4}';
_7 = (*_2);
(*_2) = _7 >> _7;
_1 = !(*_2);
_5 = [(*_3),(*_3),RET,(*_3)];
(*_3) = '\u{336d0}';
(*_2) = _1 + _7;
(*_3) = '\u{75b53}';
_9.2 = [(*_3),(*_3),(*_3),(*_3)];
Goto(bb4)
}
bb4 = {
_18 = &_14;
(*_3) = '\u{e5a62}';
(*_3) = '\u{a4bd}';
(*_2) = 138508543225231273402906825623379508096_i128 as u16;
_3 = core::ptr::addr_of_mut!((*_3));
_8 = 312022574177333180414800160160522419493_u128 as u16;
_2 = &mut _4;
(*_2) = !_7;
(*_3) = '\u{f347b}';
_14 = (*_2) as f32;
(*_3) = '\u{572f7}';
(*_2) = _1;
_17 = (*_2) <= (*_2);
(*_2) = _1 >> _7;
(*_2) = _1 * _7;
(*_3) = '\u{f0acc}';
(*_3) = '\u{4b4c3}';
(*_2) = _1;
(*_3) = '\u{fce96}';
Goto(bb5)
}
bb5 = {
_14 = (-7397111892040283784_i64) as f32;
(*_2) = !_1;
_5 = [(*_3),RET,(*_3),(*_3)];
_2 = &mut _1;
(*_3) = '\u{713be}';
(*_3) = '\u{7621d}';
(*_2) = 2682830402_u32 as u16;
_3 = core::ptr::addr_of_mut!((*_3));
(*_2) = _7;
(*_3) = '\u{41a19}';
(*_2) = _7 & _8;
(*_3) = '\u{3ac0f}';
_21 = (-47_i8) as f64;
(*_3) = '\u{ea9d}';
(*_3) = '\u{c6157}';
_22 = _21 + _21;
_7 = (*_2);
(*_2) = !_7;
(*_2) = !_7;
(*_3) = '\u{5ef8}';
_21 = _22 + _22;
Goto(bb6)
}
bb6 = {
(*_3) = '\u{1612c}';
(*_3) = '\u{87eda}';
_13 = 9223372036854775807_isize - (-9223372036854775808_isize);
_25 = [2495101765_u32,3784721028_u32,399885060_u32,1164820200_u32,2683096726_u32,3311959115_u32,1018460993_u32];
(*_3) = '\u{a7b74}';
(*_3) = '\u{79db1}';
(*_3) = '\u{deea2}';
_25 = [615873679_u32,1196788911_u32,3390493544_u32,113327113_u32,2094371979_u32,1520428107_u32,3913931140_u32];
_10 = &_17;
_9.0 = [(*_3),(*_3),(*_3),(*_3)];
(*_3) = '\u{7ea86}';
_30 = (*_3);
_24 = [(-306905747_i32)];
(*_2) = _22 as u16;
(*_2) = 3897590037_u32 as u16;
_19.1 = &mut _13;
(*_3) = _30;
(*_3) = _30;
(*_3) = _30;
_30 = (*_3);
(*_2) = _7 << _9.1;
(*_3) = _30;
Goto(bb7)
}
bb7 = {
(*_3) = _30;
(*_3) = _30;
(*_2) = 55_i8 as u16;
(*_2) = _7 * _7;
(*_3) = _30;
_9.2 = [(*_3),(*_3),(*_3),_30];
_12 = 893290904_u32 & 3268699747_u32;
Goto(bb8)
}
bb8 = {
(*_3) = _30;
(*_2) = !_7;
(*_2) = (-2587601624848206298_i64) as u16;
_33.3 = (*_3);
_30 = (*_3);
(*_2) = _8 >> _12;
(*_3) = _33.3;
_12 = 2319893390_u32 >> (*_2);
(*_2) = !_7;
_26 = &_21;
_24 = [(-1671408188_i32)];
Goto(bb9)
}
bb9 = {
_23 = core::ptr::addr_of_mut!(_26);
_37 = (*_10) > (*_10);
_9.2 = [(*_3),(*_3),(*_3),(*_3)];
RET = _33.3;
_32 = _7 as i32;
(*_3) = _33.3;
(*_23) = &_22;
_21 = (*_26);
(*_2) = _7 ^ _7;
Goto(bb10)
}
bb10 = {
_27 = core::ptr::addr_of_mut!((*_23));
(*_23) = &_21;
_41 = -(*_26);
_40 = (*_26) - (*_26);
(*_2) = _7 >> _12;
(*_3) = _30;
_25 = [_12,_12,_12,_12,_12,_12,_12];
_31 = (*_3);
_39 = Move(_19.1);
_36.1 = !4_usize;
_19.0 = core::ptr::addr_of_mut!((*_23));
_24 = [_32];
(*_3) = _33.3;
(*_3) = _33.3;
(*_2) = !_7;
_35 = !(-32666_i16);
_5 = _9.0;
(*_2) = _8 >> _32;
_41 = (*_26) - (*_26);
_26 = &_41;
(*_3) = _33.3;
_36.0 = !323080008042814543882428815706131029936_u128;
_8 = _14 as u16;
Call(_41 = core::intrinsics::fmaf64(_40, _22, _40), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_23) = &_40;
(*_2) = !_7;
_37 = (*_10);
Goto(bb12)
}
bb12 = {
(*_2) = 47617292781202603546319699174211573026_i128 as u16;
_12 = !4127946891_u32;
(*_23) = &_41;
_12 = 3832845931_u32 + 3291876760_u32;
(*_3) = _30;
_17 = _37;
_32 = -983861356_i32;
_26 = &_22;
(*_2) = _7 << _12;
(*_3) = _31;
(*_2) = _8 ^ _7;
(*_23) = &_21;
_14 = _36.1 as f32;
RET = _31;
Goto(bb13)
}
bb13 = {
_36 = (160603167235743650840541903604569075589_u128, 0_usize, _24);
_5 = [_30,RET,(*_3),_31];
_36.2 = [_32];
_51 = [(*_3),(*_3),(*_3),(*_3)];
_34 = (*_3);
(*_2) = _7 & _7;
Goto(bb14)
}
bb14 = {
(*_2) = _7 >> _9.1;
_48 = (-9223372036854775808_isize) >> (*_2);
(*_3) = _33.3;
_27 = core::ptr::addr_of_mut!((*_23));
_3 = core::ptr::addr_of_mut!(_33.3);
(*_23) = &_22;
_47 = (*_2) < (*_2);
_55 = !_36.0;
_9.2 = _5;
_17 = _47;
(*_3) = RET;
_48 = _47 as isize;
_54.2 = &(*_26);
Goto(bb15)
}
bb15 = {
_46 = _48;
(*_3) = RET;
(*_23) = &_21;
_38 = _14 - _14;
_45 = _46 + _48;
_54.1.0 = Move((*_23));
_29 = core::ptr::addr_of_mut!(_54.2);
(*_23) = &_40;
_33.0 = [(*_3),(*_3),(*_3),_31];
(*_23) = &_41;
(*_2) = _7;
_25 = [_12,_12,_12,_12,_12,_12,_12];
_29 = core::ptr::addr_of_mut!((*_23));
Goto(bb16)
}
bb16 = {
(*_3) = _30;
(*_23) = &_40;
_33.1 = [_9.1,_9.1];
_2 = &mut _8;
_33.3 = RET;
(*_23) = Move(_54.2);
(*_23) = &_41;
(*_3) = _31;
(*_23) = &_40;
_31 = (*_3);
(*_23) = &_21;
_30 = (*_3);
(*_3) = _34;
match _36.1 {
1 => bb5,
2 => bb6,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
0 => bb22,
_ => bb21
}
}
bb17 = {
(*_3) = _30;
(*_2) = !_7;
(*_2) = (-2587601624848206298_i64) as u16;
_33.3 = (*_3);
_30 = (*_3);
(*_2) = _8 >> _12;
(*_3) = _33.3;
_12 = 2319893390_u32 >> (*_2);
(*_2) = !_7;
_26 = &_21;
_24 = [(-1671408188_i32)];
Goto(bb9)
}
bb18 = {
(*_3) = '\u{1612c}';
(*_3) = '\u{87eda}';
_13 = 9223372036854775807_isize - (-9223372036854775808_isize);
_25 = [2495101765_u32,3784721028_u32,399885060_u32,1164820200_u32,2683096726_u32,3311959115_u32,1018460993_u32];
(*_3) = '\u{a7b74}';
(*_3) = '\u{79db1}';
(*_3) = '\u{deea2}';
_25 = [615873679_u32,1196788911_u32,3390493544_u32,113327113_u32,2094371979_u32,1520428107_u32,3913931140_u32];
_10 = &_17;
_9.0 = [(*_3),(*_3),(*_3),(*_3)];
(*_3) = '\u{7ea86}';
_30 = (*_3);
_24 = [(-306905747_i32)];
(*_2) = _22 as u16;
(*_2) = 3897590037_u32 as u16;
_19.1 = &mut _13;
(*_3) = _30;
(*_3) = _30;
(*_3) = _30;
_30 = (*_3);
(*_2) = _7 << _9.1;
(*_3) = _30;
Goto(bb7)
}
bb19 = {
_36 = (160603167235743650840541903604569075589_u128, 0_usize, _24);
_5 = [_30,RET,(*_3),_31];
_36.2 = [_32];
_51 = [(*_3),(*_3),(*_3),(*_3)];
_34 = (*_3);
(*_2) = _7 & _7;
Goto(bb14)
}
bb20 = {
(*_2) = 47617292781202603546319699174211573026_i128 as u16;
_12 = !4127946891_u32;
(*_23) = &_41;
_12 = 3832845931_u32 + 3291876760_u32;
(*_3) = _30;
_17 = _37;
_32 = -983861356_i32;
_26 = &_22;
(*_2) = _7 << _12;
(*_3) = _31;
(*_2) = _8 ^ _7;
(*_23) = &_21;
_14 = _36.1 as f32;
RET = _31;
Goto(bb13)
}
bb21 = {
(*_23) = &_40;
(*_2) = !_7;
_37 = (*_10);
Goto(bb12)
}
bb22 = {
_59.3 = Move(_29);
(*_2) = !_7;
_59.2.1 = _55;
(*_2) = !_7;
_59.0 = (*_2) | (*_2);
_2 = &mut _59.0;
_41 = _21 - (*_26);
_47 = _17 & _17;
_9 = (_33.0, 227_u8, _5);
(*_23) = &_40;
_45 = _46;
_17 = (*_2) < (*_2);
_40 = _41 * _21;
_61.0 = core::ptr::addr_of_mut!(_61.3);
_36.0 = !_55;
(*_23) = &_21;
_9.1 = _35 as u8;
_46 = _45 + _45;
(*_2) = _7;
(*_2) = _12 as u16;
(*_23) = &_40;
_23 = Move(_27);
_25 = [_12,_12,_12,_12,_12,_12,_12];
_54.2 = &(*_26);
match _36.1 {
1 => bb2,
2 => bb13,
3 => bb15,
4 => bb5,
5 => bb8,
6 => bb16,
0 => bb24,
_ => bb23
}
}
bb23 = {
(*_3) = _30;
(*_2) = !_7;
(*_2) = (-2587601624848206298_i64) as u16;
_33.3 = (*_3);
_30 = (*_3);
(*_2) = _8 >> _12;
(*_3) = _33.3;
_12 = 2319893390_u32 >> (*_2);
(*_2) = !_7;
_26 = &_21;
_24 = [(-1671408188_i32)];
Goto(bb9)
}
bb24 = {
_42 = -_35;
(*_2) = _7 + _7;
_43 = _46 >> (*_2);
_62.1 = &mut _45;
(*_3) = _30;
_19.1 = &mut _48;
(*_3) = _31;
_44 = &_17;
_54.1.1 = _25;
_5 = [(*_3),_34,(*_3),_33.3];
_65 = _55 << (*_2);
match _36.1 {
1 => bb2,
2 => bb10,
3 => bb8,
4 => bb22,
0 => bb25,
_ => bb6
}
}
bb25 = {
_36.2 = [_32];
_7 = (*_26) as u16;
_54.2 = &_41;
(*_2) = _42 as u16;
_42 = _35 * _35;
_60 = _14;
_12 = 1607425356_u32 | 3612827626_u32;
(*_2) = !_7;
(*_3) = _34;
_9.0 = [(*_3),(*_3),RET,(*_3)];
_18 = &_60;
(*_2) = 770267614396159179_i64 as u16;
(*_3) = RET;
_19.0 = core::ptr::addr_of_mut!(_54.2);
match _36.1 {
1 => bb24,
0 => bb26,
_ => bb5
}
}
bb26 = {
_61.3 = core::ptr::addr_of_mut!(_42);
_37 = (*_44);
Goto(bb27)
}
bb27 = {
_35 = _42;
_66 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_54.1.0 = &(*_26);
_54.1.1 = _25;
_14 = -(*_18);
_70 = (-4314053256350252761_i64) as u128;
_32 = 1264272048_i32 & (-1969390620_i32);
_33.2 = &mut _46;
_69 = (*_44) & (*_44);
_50 = core::ptr::addr_of_mut!(_54.1.0);
(*_50) = Move(_54.2);
(*_2) = _7 ^ _7;
match _36.1 {
1 => bb18,
2 => bb28,
3 => bb29,
0 => bb31,
_ => bb30
}
}
bb28 = {
_36 = (160603167235743650840541903604569075589_u128, 0_usize, _24);
_5 = [_30,RET,(*_3),_31];
_36.2 = [_32];
_51 = [(*_3),(*_3),(*_3),(*_3)];
_34 = (*_3);
(*_2) = _7 & _7;
Goto(bb14)
}
bb29 = {
(*_2) = _1 << _7;
_3 = core::ptr::addr_of_mut!(RET);
(*_2) = _7;
(*_2) = _1 ^ _7;
(*_2) = _7 + _7;
(*_3) = '\u{ca30a}';
(*_3) = '\u{3cfef}';
(*_3) = '\u{34e5c}';
(*_2) = !_7;
(*_3) = '\u{24ac1}';
(*_3) = '\u{3df5}';
_14 = (*_2) as f32;
_4 = !_8;
(*_3) = '\u{f20cc}';
Goto(bb3)
}
bb30 = {
(*_2) = 47617292781202603546319699174211573026_i128 as u16;
_12 = !4127946891_u32;
(*_23) = &_41;
_12 = 3832845931_u32 + 3291876760_u32;
(*_3) = _30;
_17 = _37;
_32 = -983861356_i32;
_26 = &_22;
(*_2) = _7 << _12;
(*_3) = _31;
(*_2) = _8 ^ _7;
(*_23) = &_21;
_14 = _36.1 as f32;
RET = _31;
Goto(bb13)
}
bb31 = {
_74 = (*_3);
(*_50) = &(*_26);
_54.1 = (Move(_26), _25);
_41 = _22 + _40;
_62 = (Move(_19.0), Move(_19.1));
_69 = !_17;
Goto(bb32)
}
bb32 = {
_51 = [(*_3),(*_3),_34,(*_3)];
_19 = (Move(_23), Move(_33.2));
_17 = (*_2) < (*_2);
(*_50) = &_22;
_52 = Adt23::Variant2 { fld0: (*_2),fld1: (-40005119824380600099604730252500735914_i128) };
(*_3) = RET;
_26 = &_40;
_29 = core::ptr::addr_of_mut!((*_50));
_65 = _70 + _36.0;
(*_50) = Move(_26);
_36.1 = 17320598617171209996_usize;
(*_50) = &_40;
(*_50) = &_22;
_19.0 = core::ptr::addr_of_mut!((*_50));
_58 = _43 - _43;
(*_3) = RET;
_44 = &_47;
match _36.1 {
0 => bb26,
1 => bb15,
2 => bb8,
3 => bb5,
4 => bb33,
17320598617171209996 => bb35,
_ => bb34
}
}
bb33 = {
(*_3) = _30;
(*_2) = !_7;
(*_2) = (-2587601624848206298_i64) as u16;
_33.3 = (*_3);
_30 = (*_3);
(*_2) = _8 >> _12;
(*_3) = _33.3;
_12 = 2319893390_u32 >> (*_2);
(*_2) = !_7;
_26 = &_21;
_24 = [(-1671408188_i32)];
Goto(bb9)
}
bb34 = {
(*_2) = _7 >> _9.1;
_48 = (-9223372036854775808_isize) >> (*_2);
(*_3) = _33.3;
_27 = core::ptr::addr_of_mut!((*_23));
_3 = core::ptr::addr_of_mut!(_33.3);
(*_23) = &_22;
_47 = (*_2) < (*_2);
_55 = !_36.0;
_9.2 = _5;
_17 = _47;
(*_3) = RET;
_48 = _47 as isize;
_54.2 = &(*_26);
Goto(bb15)
}
bb35 = {
_33.3 = _30;
_54.3 = Adt41::Variant1 { fld0: (*_44),fld1: 1541428044182464837_u64,fld2: _40,fld3: 43_i8,fld4: _12 };
(*_2) = _7 << _65;
_63 = Field::<f64>(Variant(_54.3, 1), 2) + _41;
_12 = Field::<u32>(Variant(_54.3, 1), 4);
place!(Field::<u64>(Variant(_54.3, 1), 1)) = 7710072681036684670_u64 & 14496828533663479661_u64;
_60 = _38 + _14;
(*_2) = _33.3 as u16;
_36 = (_70, 2188588818907096893_usize, _24);
(*_2) = _36.1 as u16;
match _36.1 {
2188588818907096893 => bb37,
_ => bb36
}
}
bb36 = {
_35 = _42;
_66 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_54.1.0 = &(*_26);
_54.1.1 = _25;
_14 = -(*_18);
_70 = (-4314053256350252761_i64) as u128;
_32 = 1264272048_i32 & (-1969390620_i32);
_33.2 = &mut _46;
_69 = (*_44) & (*_44);
_50 = core::ptr::addr_of_mut!(_54.1.0);
(*_50) = Move(_54.2);
(*_2) = _7 ^ _7;
match _36.1 {
1 => bb18,
2 => bb28,
3 => bb29,
0 => bb31,
_ => bb30
}
}
bb37 = {
place!(Field::<i8>(Variant(_54.3, 1), 3)) = -(-106_i8);
place!(Field::<i128>(Variant(_52, 2), 1)) = 13042505099744885287515513442125547133_i128;
_27 = Move(_50);
_62.0 = core::ptr::addr_of_mut!(_54.1.0);
_61.1 = _32 | _32;
_33.3 = _31;
(*_2) = _7 * Field::<u16>(Variant(_52, 2), 0);
(*_3) = _74;
_19.1 = &mut _43;
(*_2) = Field::<u16>(Variant(_52, 2), 0) | _7;
_47 = _37;
_78 = _38;
_7 = !(*_2);
_44 = &place!(Field::<bool>(Variant(_54.3, 1), 0));
(*_3) = _74;
_29 = core::ptr::addr_of_mut!(_54.1.0);
_71 = (*_3);
(*_3) = RET;
_53.0 = core::ptr::addr_of!(_35);
match _36.1 {
2188588818907096893 => bb38,
_ => bb31
}
}
bb38 = {
_19.0 = core::ptr::addr_of_mut!((*_29));
_61.1 = _32;
(*_3) = _74;
(*_2) = _36.1 as u16;
(*_29) = &_40;
_80 = _9.1 & _9.1;
_26 = Move((*_29));
(*_29) = &_22;
_15 = &mut _61.3;
(*_29) = &_21;
_47 = !_69;
(*_2) = Field::<u16>(Variant(_52, 2), 0) | _7;
(*_2) = _7;
_69 = (*_44);
(*_29) = &place!(Field::<f64>(Variant(_54.3, 1), 2));
(*_15) = core::ptr::addr_of_mut!(_35);
_32 = Field::<i8>(Variant(_54.3, 1), 3) as i32;
_25 = [Field::<u32>(Variant(_54.3, 1), 4),_12,_12,_12,Field::<u32>(Variant(_54.3, 1), 4),Field::<u32>(Variant(_54.3, 1), 4),_12];
_58 = !65_isize;
place!(Field::<i128>(Variant(_52, 2), 1)) = (-164757819724064107762924324858758151491_i128);
match _36.1 {
0 => bb28,
2188588818907096893 => bb40,
_ => bb39
}
}
bb39 = {
_59.3 = Move(_29);
(*_2) = !_7;
_59.2.1 = _55;
(*_2) = !_7;
_59.0 = (*_2) | (*_2);
_2 = &mut _59.0;
_41 = _21 - (*_26);
_47 = _17 & _17;
_9 = (_33.0, 227_u8, _5);
(*_23) = &_40;
_45 = _46;
_17 = (*_2) < (*_2);
_40 = _41 * _21;
_61.0 = core::ptr::addr_of_mut!(_61.3);
_36.0 = !_55;
(*_23) = &_21;
_9.1 = _35 as u8;
_46 = _45 + _45;
(*_2) = _7;
(*_2) = _12 as u16;
(*_23) = &_40;
_23 = Move(_27);
_25 = [_12,_12,_12,_12,_12,_12,_12];
_54.2 = &(*_26);
match _36.1 {
1 => bb2,
2 => bb13,
3 => bb15,
4 => bb5,
5 => bb8,
6 => bb16,
0 => bb24,
_ => bb23
}
}
bb40 = {
_57 = &mut _80;
(*_57) = _55 as u8;
_33.2 = &mut _58;
_33.0 = [(*_3),_34,(*_3),(*_3)];
(*_15) = core::ptr::addr_of_mut!(_42);
(*_57) = _78 as u8;
_36.2 = [_32];
_36.2 = [_32];
match _36.1 {
0 => bb1,
2188588818907096893 => bb41,
_ => bb27
}
}
bb41 = {
(*_15) = core::ptr::addr_of_mut!(_35);
(*_29) = &_41;
Call(_36.1 = core::intrinsics::bswap(11435515042113973695_usize), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Goto(bb43)
}
bb43 = {
(*_2) = Field::<u16>(Variant(_52, 2), 0) ^ _7;
(*_29) = &_22;
(*_29) = &place!(Field::<f64>(Variant(_54.3, 1), 2));
(*_2) = _65 as u16;
(*_3) = _71;
_23 = Move(_19.0);
_57 = &mut _9.1;
_79 = Adt43::Variant1 { fld0: (*_2),fld1: _5,fld2: Move(_3),fld3: Move(_53.0) };
(*_29) = &_63;
_53.0 = core::ptr::addr_of!(_75);
(*_57) = 19_u8 - 219_u8;
(*_2) = _7 & Field::<u16>(Variant(_52, 2), 0);
(*_29) = &_22;
_3 = core::ptr::addr_of_mut!(_71);
_75 = (*_2) as i16;
(*_29) = &place!(Field::<f64>(Variant(_54.3, 1), 2));
(*_29) = &_21;
(*_29) = &_41;
(*_3) = _30;
(*_29) = &_40;
place!(Field::<*mut char>(Variant(_79, 1), 2)) = core::ptr::addr_of_mut!((*_3));
(*_2) = Field::<u16>(Variant(_79, 1), 0) << _75;
place!(Field::<*mut char>(Variant(_79, 1), 2)) = core::ptr::addr_of_mut!((*_3));
_88 = _32 ^ _32;
_87 = core::ptr::addr_of_mut!(_57);
_63 = -_41;
_68 = [Field::<u32>(Variant(_54.3, 1), 4),_12,_12,_12,Field::<u32>(Variant(_54.3, 1), 4),_12,_12];
(*_3) = _74;
match Field::<i128>(Variant(_52, 2), 1) {
0 => bb39,
175524547196874355700450282573010059965 => bb45,
_ => bb44
}
}
bb44 = {
_57 = &mut _80;
(*_57) = _55 as u8;
_33.2 = &mut _58;
_33.0 = [(*_3),_34,(*_3),(*_3)];
(*_15) = core::ptr::addr_of_mut!(_42);
(*_57) = _78 as u8;
_36.2 = [_32];
_36.2 = [_32];
match _36.1 {
0 => bb1,
2188588818907096893 => bb41,
_ => bb27
}
}
bb45 = {
(*_2) = _7;
(*_57) = 12_u8;
(*_57) = _75 as u8;
(*_2) = _7 | Field::<u16>(Variant(_52, 2), 0);
_50 = core::ptr::addr_of_mut!((*_29));
_70 = Field::<u64>(Variant(_54.3, 1), 1) as u128;
_62.1 = Move(_19.1);
(*_15) = core::ptr::addr_of_mut!(_42);
(*_29) = &place!(Field::<f64>(Variant(_54.3, 1), 2));
_23 = core::ptr::addr_of_mut!((*_29));
(*_57) = 227_u8;
_55 = _65;
_70 = !_65;
Goto(bb46)
}
bb46 = {
(*_15) = core::ptr::addr_of_mut!(_42);
place!(Field::<u32>(Variant(_54.3, 1), 4)) = _12 | _12;
_72 = _69 ^ _17;
_81 = (*_3);
place!(Field::<bool>(Variant(_54.3, 1), 0)) = !_37;
match (*_57) {
0 => bb47,
227 => bb49,
_ => bb48
}
}
bb47 = {
(*_3) = '\u{1612c}';
(*_3) = '\u{87eda}';
_13 = 9223372036854775807_isize - (-9223372036854775808_isize);
_25 = [2495101765_u32,3784721028_u32,399885060_u32,1164820200_u32,2683096726_u32,3311959115_u32,1018460993_u32];
(*_3) = '\u{a7b74}';
(*_3) = '\u{79db1}';
(*_3) = '\u{deea2}';
_25 = [615873679_u32,1196788911_u32,3390493544_u32,113327113_u32,2094371979_u32,1520428107_u32,3913931140_u32];
_10 = &_17;
_9.0 = [(*_3),(*_3),(*_3),(*_3)];
(*_3) = '\u{7ea86}';
_30 = (*_3);
_24 = [(-306905747_i32)];
(*_2) = _22 as u16;
(*_2) = 3897590037_u32 as u16;
_19.1 = &mut _13;
(*_3) = _30;
(*_3) = _30;
(*_3) = _30;
_30 = (*_3);
(*_2) = _7 << _9.1;
(*_3) = _30;
Goto(bb7)
}
bb48 = {
(*_3) = _30;
(*_3) = _30;
(*_2) = 55_i8 as u16;
(*_2) = _7 * _7;
(*_3) = _30;
_9.2 = [(*_3),(*_3),(*_3),_30];
_12 = 893290904_u32 & 3268699747_u32;
Goto(bb8)
}
bb49 = {
_53.0 = core::ptr::addr_of!(_42);
_33.3 = _74;
(*_2) = _7;
(*_29) = &_41;
Goto(bb50)
}
bb50 = {
Call(_94 = dump_var(Move(_80), Move(_30), Move(_51), Move(_25)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_94 = dump_var(Move(_47), Move(_48), Move(_69), Move(_31)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_94 = dump_var(Move(_75), Move(_42), Move(_7), Move(_35)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_94 = dump_var(Move(_34), Move(_46), Move(_32), Move(_68)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_94 = dump_var(Move(_6), Move(_43), Move(_65), Move(_45)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *mut char) -> char {
mir! {
type RET = char;
let _2: isize;
let _3: char;
let _4: char;
let _5: f64;
let _6: isize;
let _7: bool;
let _8: f32;
let _9: f32;
let _10: i128;
let _11: isize;
let _12: bool;
let _13: &'static &'static mut *mut char;
let _14: i32;
let _15: (*const i16,);
let _16: char;
let _17: i128;
let _18: (u128, usize, [i32; 1]);
let _19: [usize; 7];
let _20: char;
let _21: bool;
let _22: char;
let _23: *mut i16;
let _24: i8;
let _25: i16;
let _26: *const &'static mut &'static f64;
let _27: i64;
let _28: isize;
let _29: bool;
let _30: *mut char;
let _31: f32;
let _32: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _33: isize;
let _34: char;
let _35: usize;
let _36: u8;
let _37: [u8; 2];
let _38: isize;
let _39: Adt44;
let _40: char;
let _41: isize;
let _42: *mut &'static mut u8;
let _43: &'static i8;
let _44: f64;
let _45: u32;
let _46: [u8; 2];
let _47: *mut &'static f64;
let _48: char;
let _49: usize;
let _50: &'static mut isize;
let _51: f32;
let _52: isize;
let _53: &'static f64;
let _54: *const &'static mut &'static f64;
let _55: [i32; 1];
let _56: [u32; 7];
let _57: *const &'static mut &'static f64;
let _58: *const i16;
let _59: &'static mut *mut char;
let _60: *mut i16;
let _61: i8;
let _62: bool;
let _63: &'static f64;
let _64: char;
let _65: bool;
let _66: u64;
let _67: char;
let _68: (Adt23, u128);
let _69: i64;
let _70: *const char;
let _71: char;
let _72: ([char; 4], [u8; 2], &'static mut isize, char);
let _73: &'static mut &'static f64;
let _74: usize;
let _75: f64;
let _76: &'static mut *const u128;
let _77: isize;
let _78: (Adt23, u128);
let _79: i128;
let _80: &'static f64;
let _81: f32;
let _82: char;
let _83: Adt55;
let _84: *mut [u32; 7];
let _85: isize;
let _86: isize;
let _87: &'static mut isize;
let _88: i32;
let _89: isize;
let _90: f32;
let _91: isize;
let _92: u128;
let _93: char;
let _94: &'static mut u16;
let _95: [i32; 1];
let _96: char;
let _97: [u128; 2];
let _98: char;
let _99: isize;
let _100: isize;
let _101: isize;
let _102: *mut u32;
let _103: Adt55;
let _104: ([char; 4], [u8; 2], &'static mut isize, char);
let _105: &'static mut &'static f64;
let _106: isize;
let _107: isize;
let _108: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _109: Adt43;
let _110: f32;
let _111: char;
let _112: u16;
let _113: i32;
let _114: ([char; 4], u8, [char; 4]);
let _115: f32;
let _116: i8;
let _117: [bool; 2];
let _118: i128;
let _119: [i128; 2];
let _120: isize;
let _121: *const &'static mut u8;
let _122: [char; 4];
let _123: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _124: *const i16;
let _125: *const u128;
let _126: (*mut *mut i16, i32, Adt55, *mut i16);
let _127: f64;
let _128: ();
let _129: ();
{
RET = '\u{d8ffd}';
_1 = core::ptr::addr_of_mut!(_3);
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
Goto(bb1)
}
bb1 = {
_2 = 1_usize as isize;
_6 = 14300471359452079403_u64 as isize;
(*_1) = RET;
_6 = !_2;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
(*_1) = RET;
_4 = (*_1);
(*_1) = _4;
_2 = _6;
(*_1) = _4;
_4 = RET;
Goto(bb2)
}
bb2 = {
Call(_12 = fn9(Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = !_6;
RET = _3;
_8 = (-90_i8) as f32;
_12 = false;
_7 = !_12;
_4 = RET;
_1 = core::ptr::addr_of_mut!(_3);
_4 = (*_1);
_3 = _4;
(*_1) = RET;
(*_1) = RET;
_10 = (-35961875729857688865285046307063613147_i128);
_10 = (-31520143031231547561863749416809185366_i128) >> _6;
_5 = 175147978003405228058239168317053123843_u128 as f64;
(*_1) = _4;
Goto(bb4)
}
bb4 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = (*_1);
_6 = _7 as isize;
_12 = (*_1) == (*_1);
(*_1) = RET;
(*_1) = _4;
(*_1) = _4;
(*_1) = _4;
_3 = _4;
(*_1) = _4;
(*_1) = RET;
Goto(bb5)
}
bb5 = {
_5 = 7_usize as f64;
(*_1) = _4;
(*_1) = RET;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = RET;
_8 = _2 as f32;
(*_1) = _4;
Call(_2 = fn10(_7, (*_1), Move(_1), _6, _7, _8, (*_1), (*_1), _6, (*_1), (*_1)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6 = _11;
_14 = (-732772660_i32);
_16 = _3;
_1 = core::ptr::addr_of_mut!(_16);
(*_1) = _3;
(*_1) = _4;
_3 = (*_1);
_11 = !_2;
(*_1) = _4;
(*_1) = _4;
(*_1) = RET;
(*_1) = _3;
_2 = _11;
_1 = core::ptr::addr_of_mut!((*_1));
_14 = !(-694205366_i32);
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _3;
_18.1 = _2 as usize;
_18.0 = !208687907104226624326599679813848706713_u128;
(*_1) = _3;
(*_1) = _3;
Goto(bb7)
}
bb7 = {
_22 = (*_1);
_7 = _8 <= _8;
_8 = (-26553_i16) as f32;
_6 = _2 << _18.1;
_9 = _8 - _8;
_18.1 = 14659775977872971793_usize + 6954667047057479838_usize;
(*_1) = _22;
_18.2 = [_14];
(*_1) = _22;
(*_1) = RET;
Goto(bb8)
}
bb8 = {
_3 = (*_1);
(*_1) = RET;
_16 = RET;
_6 = -_2;
_5 = _9 as f64;
_16 = _22;
_5 = _2 as f64;
(*_1) = _3;
(*_1) = RET;
(*_1) = _4;
_4 = (*_1);
_6 = _2 & _2;
_6 = !_11;
_3 = (*_1);
_23 = core::ptr::addr_of_mut!(_25);
_24 = !(-74_i8);
(*_23) = 18842_i16 << _24;
(*_23) = -21403_i16;
(*_23) = 4489_i16;
(*_1) = _3;
_2 = _6;
_18.0 = _10 as u128;
_18.1 = (*_23) as usize;
(*_23) = _14 as i16;
(*_23) = -(-7944_i16);
_16 = _3;
Goto(bb9)
}
bb9 = {
_17 = _10 << (*_23);
(*_23) = -18727_i16;
(*_23) = 13143145191434529193_u64 as i16;
_16 = _4;
(*_1) = _3;
(*_1) = RET;
_23 = core::ptr::addr_of_mut!(_25);
(*_23) = 12335_i16 << _6;
_24 = _10 as i8;
_8 = _9 * _9;
_30 = Move(_1);
_7 = !_12;
_28 = _2 + _11;
_16 = _22;
_12 = !_7;
_7 = _12;
_21 = _7;
_18.1 = 2960685355095536167_usize;
Goto(bb10)
}
bb10 = {
_18.1 = 2_usize ^ 0_usize;
_1 = core::ptr::addr_of_mut!(_22);
_20 = (*_1);
_19 = [_18.1,_18.1,_18.1,_18.1,_18.1,_18.1,_18.1];
_21 = !_12;
_14 = _18.0 as i32;
(*_1) = RET;
_32.2.1 = _18.0 << _18.0;
RET = (*_1);
_32.1 = &mut _24;
(*_23) = 3679_i16 | 10140_i16;
Goto(bb11)
}
bb11 = {
_30 = core::ptr::addr_of_mut!((*_1));
_32.0 = !61192_u16;
(*_30) = _4;
(*_30) = _20;
Call(_11 = core::intrinsics::bswap(_28), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_23) = -(-11006_i16);
_18.0 = 68_u8 as u128;
(*_1) = RET;
_22 = _16;
_34 = _22;
_25 = 21177_i16;
_35 = !_18.1;
(*_1) = RET;
_9 = _8 - _8;
RET = (*_1);
(*_1) = _16;
(*_1) = RET;
(*_23) = _10 as i16;
_36 = 202_u8;
_19 = [_18.1,_18.1,_18.1,_18.1,_35,_18.1,_18.1];
(*_23) = 2156804495_u32 as i16;
(*_23) = 3389_i16;
(*_1) = _4;
Goto(bb13)
}
bb13 = {
_4 = (*_1);
_32.0 = 45736_u16 << _18.0;
_4 = (*_1);
(*_23) = (-29925_i16) + (-15659_i16);
_15.0 = core::ptr::addr_of!((*_23));
(*_1) = RET;
(*_1) = _4;
_14 = 138107710_i32 << _17;
(*_1) = _3;
(*_23) = -11167_i16;
(*_1) = _16;
(*_23) = _5 as i16;
Goto(bb14)
}
bb14 = {
_14 = 704336254_i32 + 1717153218_i32;
(*_23) = (-7912_i16) << _2;
(*_23) = (-5563_i16) | 17742_i16;
(*_23) = -(-3998_i16);
_4 = (*_1);
_32.0 = 27823_u16 ^ 50926_u16;
_27 = _5 as i64;
_41 = _28 - _6;
(*_1) = RET;
_32.0 = 854_u16 | 25638_u16;
_4 = (*_1);
(*_23) = !16985_i16;
_41 = _28;
Call(_33 = core::intrinsics::transmute(_28), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_23) = 12426_i16 * (-17254_i16);
_28 = _41 | _33;
_32.2.0 = Adt23::Variant2 { fld0: _32.0,fld1: _10 };
match _36 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb16,
202 => bb18,
_ => bb17
}
}
bb16 = {
_17 = _10 << (*_23);
(*_23) = -18727_i16;
(*_23) = 13143145191434529193_u64 as i16;
_16 = _4;
(*_1) = _3;
(*_1) = RET;
_23 = core::ptr::addr_of_mut!(_25);
(*_23) = 12335_i16 << _6;
_24 = _10 as i8;
_8 = _9 * _9;
_30 = Move(_1);
_7 = !_12;
_28 = _2 + _11;
_16 = _22;
_12 = !_7;
_7 = _12;
_21 = _7;
_18.1 = 2960685355095536167_usize;
Goto(bb10)
}
bb17 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = (*_1);
_6 = _7 as isize;
_12 = (*_1) == (*_1);
(*_1) = RET;
(*_1) = _4;
(*_1) = _4;
(*_1) = _4;
_3 = _4;
(*_1) = _4;
(*_1) = RET;
Goto(bb5)
}
bb18 = {
(*_1) = _16;
(*_23) = (-27581_i16);
(*_23) = _14 as i16;
_29 = !_7;
_9 = _8;
_45 = 102_i8 as u32;
(*_23) = -(-9918_i16);
_45 = 1117625629_u32;
_46 = [_36,_36];
(*_1) = _20;
(*_23) = 17306_i16;
_31 = _14 as f32;
_2 = _28 * _11;
(*_23) = _7 as i16;
(*_1) = _16;
(*_23) = 11965_i16 - 660_i16;
_27 = (-2496926694750136282_i64) | (-7481321558993342211_i64);
_18.2 = [_14];
_18.2 = [_14];
(*_1) = RET;
_11 = _2 ^ _2;
_33 = _2 >> _35;
match _36 {
0 => bb2,
202 => bb20,
_ => bb19
}
}
bb19 = {
_17 = _10 << (*_23);
(*_23) = -18727_i16;
(*_23) = 13143145191434529193_u64 as i16;
_16 = _4;
(*_1) = _3;
(*_1) = RET;
_23 = core::ptr::addr_of_mut!(_25);
(*_23) = 12335_i16 << _6;
_24 = _10 as i8;
_8 = _9 * _9;
_30 = Move(_1);
_7 = !_12;
_28 = _2 + _11;
_16 = _22;
_12 = !_7;
_7 = _12;
_21 = _7;
_18.1 = 2960685355095536167_usize;
Goto(bb10)
}
bb20 = {
_35 = _36 as usize;
_22 = _34;
_18.2 = [_14];
(*_1) = RET;
(*_23) = 32063_i16;
_44 = _5 - _5;
(*_1) = RET;
RET = (*_1);
(*_23) = -(-3114_i16);
(*_23) = _14 as i16;
_52 = -_2;
_20 = (*_1);
_41 = _28 ^ _52;
_52 = _11 - _41;
_49 = !_18.1;
(*_23) = !17854_i16;
Goto(bb21)
}
bb21 = {
_51 = _9 + _8;
(*_1) = _3;
(*_1) = _16;
_6 = _2 ^ _11;
_34 = _4;
_32.0 = !Field::<u16>(Variant(_32.2.0, 2), 0);
_4 = (*_1);
_18.2 = [_14];
(*_1) = _3;
(*_23) = 15915_i16;
(*_23) = !(-25952_i16);
_32.0 = Field::<u16>(Variant(_32.2.0, 2), 0);
_41 = _52 | _6;
_58 = Move(_15.0);
_30 = core::ptr::addr_of_mut!(_16);
_15 = (Move(_58),);
(*_1) = (*_30);
(*_30) = RET;
Goto(bb22)
}
bb22 = {
_40 = (*_30);
place!(Field::<u16>(Variant(_32.2.0, 2), 0)) = _32.0 >> _41;
_62 = _12;
(*_30) = (*_1);
(*_23) = _36 as i16;
(*_23) = -(-20793_i16);
(*_30) = _34;
_64 = _20;
(*_30) = _22;
(*_1) = (*_30);
_18.1 = _49 * _49;
_18.0 = _29 as u128;
_25 = Field::<u16>(Variant(_32.2.0, 2), 0) as i16;
_18.1 = !_49;
_60 = core::ptr::addr_of_mut!((*_23));
(*_23) = (-5322_i16) >> Field::<u16>(Variant(_32.2.0, 2), 0);
(*_23) = 18588_i16 - 25523_i16;
_40 = (*_30);
(*_23) = 13690_i16;
_55 = [_14];
_68 = (_32.2.0, _32.2.1);
(*_30) = (*_1);
match (*_23) {
0 => bb6,
13690 => bb24,
_ => bb23
}
}
bb23 = {
_17 = _10 << (*_23);
(*_23) = -18727_i16;
(*_23) = 13143145191434529193_u64 as i16;
_16 = _4;
(*_1) = _3;
(*_1) = RET;
_23 = core::ptr::addr_of_mut!(_25);
(*_23) = 12335_i16 << _6;
_24 = _10 as i8;
_8 = _9 * _9;
_30 = Move(_1);
_7 = !_12;
_28 = _2 + _11;
_16 = _22;
_12 = !_7;
_7 = _12;
_21 = _7;
_18.1 = 2960685355095536167_usize;
Goto(bb10)
}
bb24 = {
_1 = core::ptr::addr_of_mut!(_40);
_12 = _7;
_37 = [_36,_36];
_17 = _51 as i128;
_1 = core::ptr::addr_of_mut!((*_1));
_14 = (-1141450792_i32) << Field::<u16>(Variant(_32.2.0, 2), 0);
(*_1) = (*_30);
_3 = (*_1);
_17 = Field::<i128>(Variant(_32.2.0, 2), 1);
_18.0 = _33 as u128;
_23 = core::ptr::addr_of_mut!((*_23));
(*_23) = (-4424_i16) | (-12775_i16);
(*_1) = _16;
_49 = _18.1 | _18.1;
_61 = 122_i8 & (-53_i8);
_36 = 75_u8 | 222_u8;
_11 = _2 - _6;
(*_1) = _34;
_65 = (*_1) == (*_30);
_22 = _64;
Goto(bb25)
}
bb25 = {
(*_1) = (*_30);
match _45 {
0 => bb19,
1117625629 => bb27,
_ => bb26
}
}
bb26 = {
_1 = core::ptr::addr_of_mut!(_40);
_12 = _7;
_37 = [_36,_36];
_17 = _51 as i128;
_1 = core::ptr::addr_of_mut!((*_1));
_14 = (-1141450792_i32) << Field::<u16>(Variant(_32.2.0, 2), 0);
(*_1) = (*_30);
_3 = (*_1);
_17 = Field::<i128>(Variant(_32.2.0, 2), 1);
_18.0 = _33 as u128;
_23 = core::ptr::addr_of_mut!((*_23));
(*_23) = (-4424_i16) | (-12775_i16);
(*_1) = _16;
_49 = _18.1 | _18.1;
_61 = 122_i8 & (-53_i8);
_36 = 75_u8 | 222_u8;
_11 = _2 - _6;
(*_1) = _34;
_65 = (*_1) == (*_30);
_22 = _64;
Goto(bb25)
}
bb27 = {
_50 = &mut _6;
(*_23) = !(-30293_i16);
(*_30) = (*_1);
_1 = core::ptr::addr_of_mut!((*_30));
(*_30) = RET;
(*_30) = RET;
_48 = (*_30);
(*_30) = _3;
(*_23) = _40 as i16;
(*_50) = -_28;
(*_23) = -24240_i16;
(*_23) = (-23556_i16) & 22048_i16;
_62 = Field::<u16>(Variant(_68.0, 2), 0) >= Field::<u16>(Variant(_32.2.0, 2), 0);
_56 = [_45,_45,_45,_45,_45,_45,_45];
_63 = &_5;
_80 = &(*_63);
_38 = 10685494600389653104_u64 as isize;
_60 = core::ptr::addr_of_mut!((*_23));
_71 = (*_30);
(*_50) = _61 as isize;
_73 = &mut _80;
_44 = _61 as f64;
_12 = _62 >= _62;
(*_23) = (*_63) as i16;
_70 = core::ptr::addr_of!(_16);
(*_73) = &_44;
(*_23) = _27 as i16;
Goto(bb28)
}
bb28 = {
(*_70) = RET;
_74 = _18.1 & _49;
_54 = core::ptr::addr_of!(_73);
(*_50) = -_52;
_52 = !(*_50);
_77 = (*_50) & (*_50);
(*_54) = &mut _63;
_60 = core::ptr::addr_of_mut!((*_23));
place!(Field::<i128>(Variant(_68.0, 2), 1)) = _10 * _17;
(*_73) = &_44;
_32.3 = core::ptr::addr_of_mut!((*_73));
(*_23) = (-12046_i16) & (-4953_i16);
(*_30) = _4;
(*_73) = &_5;
_32.1 = &mut _61;
match _45 {
0 => bb29,
1 => bb30,
1117625629 => bb32,
_ => bb31
}
}
bb29 = {
_17 = _10 << (*_23);
(*_23) = -18727_i16;
(*_23) = 13143145191434529193_u64 as i16;
_16 = _4;
(*_1) = _3;
(*_1) = RET;
_23 = core::ptr::addr_of_mut!(_25);
(*_23) = 12335_i16 << _6;
_24 = _10 as i8;
_8 = _9 * _9;
_30 = Move(_1);
_7 = !_12;
_28 = _2 + _11;
_16 = _22;
_12 = !_7;
_7 = _12;
_21 = _7;
_18.1 = 2960685355095536167_usize;
Goto(bb10)
}
bb30 = {
_4 = (*_1);
_32.0 = 45736_u16 << _18.0;
_4 = (*_1);
(*_23) = (-29925_i16) + (-15659_i16);
_15.0 = core::ptr::addr_of!((*_23));
(*_1) = RET;
(*_1) = _4;
_14 = 138107710_i32 << _17;
(*_1) = _3;
(*_23) = -11167_i16;
(*_1) = _16;
(*_23) = _5 as i16;
Goto(bb14)
}
bb31 = {
(*_23) = 12426_i16 * (-17254_i16);
_28 = _41 | _33;
_32.2.0 = Adt23::Variant2 { fld0: _32.0,fld1: _10 };
match _36 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb16,
202 => bb18,
_ => bb17
}
}
bb32 = {
(*_73) = &_44;
place!(Field::<i128>(Variant(_68.0, 2), 1)) = _10 & _17;
_52 = _10 as isize;
(*_30) = _40;
(*_73) = &_5;
_72.0 = [(*_30),(*_30),RET,RET];
_78.0 = _32.2.0;
_15.0 = core::ptr::addr_of!((*_23));
(*_50) = -_33;
(*_30) = _22;
(*_73) = &_44;
_81 = _49 as f32;
place!(Field::<u16>(Variant(_78.0, 2), 0)) = Field::<u16>(Variant(_68.0, 2), 0) ^ Field::<u16>(Variant(_32.2.0, 2), 0);
(*_30) = _22;
(*_30) = _22;
_34 = (*_30);
_72.0 = [(*_30),(*_30),_22,_20];
(*_50) = _77 | _41;
_18.2 = [_14];
_26 = Move(_54);
(*_50) = _36 as isize;
(*_50) = _74 as isize;
_71 = (*_30);
(*_50) = _77;
(*_73) = &_5;
_75 = _18.0 as f64;
match _45 {
0 => bb31,
1 => bb9,
2 => bb29,
1117625629 => bb34,
_ => bb33
}
}
bb33 = {
_18.1 = 2_usize ^ 0_usize;
_1 = core::ptr::addr_of_mut!(_22);
_20 = (*_1);
_19 = [_18.1,_18.1,_18.1,_18.1,_18.1,_18.1,_18.1];
_21 = !_12;
_14 = _18.0 as i32;
(*_1) = RET;
_32.2.1 = _18.0 << _18.0;
RET = (*_1);
_32.1 = &mut _24;
(*_23) = 3679_i16 | 10140_i16;
Goto(bb11)
}
bb34 = {
_89 = _51 as isize;
_22 = (*_30);
_91 = _18.0 as isize;
(*_30) = _64;
(*_50) = _11 & _33;
_58 = core::ptr::addr_of!(_25);
(*_73) = &_44;
(*_30) = _34;
_44 = _75 + _75;
(*_50) = _14 as isize;
_47 = core::ptr::addr_of_mut!((*_73));
_62 = Field::<u16>(Variant(_78.0, 2), 0) <= Field::<u16>(Variant(_68.0, 2), 0);
RET = (*_30);
_16 = RET;
(*_73) = &_75;
(*_73) = &_5;
_94 = &mut place!(Field::<u16>(Variant(_32.2.0, 2), 0));
(*_73) = &_44;
_93 = (*_30);
Goto(bb35)
}
bb35 = {
_35 = _74 - _49;
_87 = &mut _77;
(*_23) = !(-19662_i16);
(*_73) = &_75;
_78.0 = Adt23::Variant2 { fld0: (*_94),fld1: _10 };
(*_23) = 5829_i16;
_57 = core::ptr::addr_of!(_73);
_34 = (*_30);
_66 = 13151154239934789748_u64 ^ 14164479926929361723_u64;
Goto(bb36)
}
bb36 = {
_18.1 = !_35;
Goto(bb37)
}
bb37 = {
(*_23) = (-22551_i16) | (-32593_i16);
_14 = -(-1313675837_i32);
_21 = _62;
(*_87) = (*_50) | (*_50);
(*_73) = &_44;
(*_94) = Field::<u16>(Variant(_78.0, 2), 0);
(*_30) = _48;
_18.0 = _68.1 - _68.1;
(*_73) = &_5;
(*_73) = &_44;
_55 = [_14];
(*_73) = &_75;
_98 = (*_30);
_82 = (*_30);
_100 = !(*_87);
_54 = core::ptr::addr_of!((*_57));
(*_94) = !Field::<u16>(Variant(_78.0, 2), 0);
(*_30) = _3;
_25 = (-12628_i16) ^ 23743_i16;
(*_50) = _62 as isize;
_92 = !_68.1;
Goto(bb38)
}
bb38 = {
_33 = (*_87);
(*_87) = (*_50) - (*_50);
_16 = _22;
(*_87) = -(*_50);
(*_73) = &_5;
_47 = core::ptr::addr_of_mut!((*_73));
_7 = _62;
_15.0 = core::ptr::addr_of!((*_23));
_106 = _49 as isize;
(*_23) = _45 as i16;
match _45 {
0 => bb9,
1 => bb32,
2 => bb5,
3 => bb26,
4 => bb39,
1117625629 => bb41,
_ => bb40
}
}
bb39 = {
_40 = (*_30);
place!(Field::<u16>(Variant(_32.2.0, 2), 0)) = _32.0 >> _41;
_62 = _12;
(*_30) = (*_1);
(*_23) = _36 as i16;
(*_23) = -(-20793_i16);
(*_30) = _34;
_64 = _20;
(*_30) = _22;
(*_1) = (*_30);
_18.1 = _49 * _49;
_18.0 = _29 as u128;
_25 = Field::<u16>(Variant(_32.2.0, 2), 0) as i16;
_18.1 = !_49;
_60 = core::ptr::addr_of_mut!((*_23));
(*_23) = (-5322_i16) >> Field::<u16>(Variant(_32.2.0, 2), 0);
(*_23) = 18588_i16 - 25523_i16;
_40 = (*_30);
(*_23) = 13690_i16;
_55 = [_14];
_68 = (_32.2.0, _32.2.1);
(*_30) = (*_1);
match (*_23) {
0 => bb6,
13690 => bb24,
_ => bb23
}
}
bb40 = {
_18.1 = !_35;
Goto(bb37)
}
bb41 = {
(*_87) = !(*_50);
(*_23) = 25313_i16 | (-5673_i16);
(*_94) = !Field::<u16>(Variant(_78.0, 2), 0);
(*_87) = -_100;
(*_94) = _36 as u16;
(*_47) = &_44;
(*_30) = _22;
(*_47) = &_5;
_101 = (*_87) << (*_87);
_18 = (_92, _35, _55);
_8 = _31;
(*_73) = &_75;
(*_50) = (*_87) * (*_87);
_107 = (*_50) + (*_50);
_54 = core::ptr::addr_of!((*_54));
_23 = core::ptr::addr_of_mut!((*_23));
(*_87) = _45 as isize;
_104.2 = &mut _2;
_100 = (*_50);
(*_23) = !20049_i16;
_41 = (*_50) & (*_50);
_87 = &mut _100;
_98 = _71;
_75 = _44;
_26 = core::ptr::addr_of!((*_54));
(*_73) = &_5;
(*_30) = _82;
match _45 {
0 => bb32,
1 => bb17,
2 => bb16,
3 => bb4,
4 => bb12,
5 => bb30,
1117625629 => bb43,
_ => bb42
}
}
bb42 = {
_51 = _9 + _8;
(*_1) = _3;
(*_1) = _16;
_6 = _2 ^ _11;
_34 = _4;
_32.0 = !Field::<u16>(Variant(_32.2.0, 2), 0);
_4 = (*_1);
_18.2 = [_14];
(*_1) = _3;
(*_23) = 15915_i16;
(*_23) = !(-25952_i16);
_32.0 = Field::<u16>(Variant(_32.2.0, 2), 0);
_41 = _52 | _6;
_58 = Move(_15.0);
_30 = core::ptr::addr_of_mut!(_16);
_15 = (Move(_58),);
(*_1) = (*_30);
(*_30) = RET;
Goto(bb22)
}
bb43 = {
_17 = _10;
(*_87) = _41 | (*_50);
(*_94) = !Field::<u16>(Variant(_68.0, 2), 0);
_18 = (_92, _35, _55);
_53 = &_5;
_108.1 = (Move((*_73)), _56);
(*_94) = Field::<u16>(Variant(_68.0, 2), 0);
_104.1 = _37;
(*_50) = (*_87);
_63 = &_44;
_35 = !_18.1;
_47 = core::ptr::addr_of_mut!(_53);
(*_30) = _48;
Goto(bb44)
}
bb44 = {
(*_23) = 8836_i16 | 7344_i16;
_49 = _18.1;
(*_94) = Field::<u16>(Variant(_68.0, 2), 0) + Field::<u16>(Variant(_78.0, 2), 0);
(*_47) = &(*_63);
(*_50) = (*_87) + (*_87);
(*_94) = Field::<u16>(Variant(_68.0, 2), 0) | Field::<u16>(Variant(_68.0, 2), 0);
(*_30) = _4;
(*_26) = &mut (*_47);
(*_26) = &mut _63;
_68.1 = !_92;
(*_50) = -_101;
_88 = _14 + _14;
match _45 {
0 => bb28,
1 => bb18,
2 => bb23,
3 => bb4,
4 => bb42,
5 => bb22,
6 => bb45,
1117625629 => bb47,
_ => bb46
}
}
bb45 = {
(*_1) = (*_30);
match _45 {
0 => bb19,
1117625629 => bb27,
_ => bb26
}
}
bb46 = {
_18.1 = !_35;
Goto(bb37)
}
bb47 = {
(*_94) = Field::<u16>(Variant(_78.0, 2), 0) * Field::<u16>(Variant(_68.0, 2), 0);
_62 = _7 ^ _7;
_83 = Adt55::Variant0 { fld0: Move(_70),fld1: _66,fld2: _92 };
(*_73) = &_75;
(*_94) = Field::<u16>(Variant(_78.0, 2), 0);
_72.1 = [_36,_36];
(*_50) = (*_87) >> (*_23);
_113 = -_14;
_44 = _75 + _75;
(*_73) = &_5;
(*_87) = (*_23) as isize;
(*_87) = _101 & (*_50);
_99 = -_33;
(*_50) = _113 as isize;
_87 = &mut _107;
_79 = _10;
(*_87) = _11 ^ _41;
_18.0 = _66 as u128;
(*_23) = (-30159_i16) * (-18131_i16);
Goto(bb48)
}
bb48 = {
_56 = [_45,_45,_45,_45,_45,_45,_45];
_72.3 = (*_30);
_104.0 = _72.0;
(*_23) = 6143_i16 << (*_87);
_108.1 = (Move((*_73)), _56);
_108.3 = Adt41::Variant1 { fld0: _12,fld1: Field::<u64>(Variant(_83, 0), 1),fld2: _44,fld3: (-34_i8),fld4: _45 };
match Field::<u32>(Variant(_108.3, 1), 4) {
0 => bb4,
1 => bb45,
2 => bb3,
1117625629 => bb50,
_ => bb49
}
}
bb49 = {
(*_1) = (*_30);
match _45 {
0 => bb19,
1117625629 => bb27,
_ => bb26
}
}
bb50 = {
(*_23) = (-20555_i16) * 2291_i16;
(*_30) = _71;
(*_87) = _41;
_46 = [_36,_36];
_123.2 = &_5;
_71 = (*_30);
(*_94) = Field::<u16>(Variant(_78.0, 2), 0) * Field::<u16>(Variant(_78.0, 2), 0);
(*_30) = _82;
_113 = _14 & _88;
_91 = (*_87) | (*_87);
_95 = [_14];
(*_26) = &mut _123.2;
(*_87) = _99;
_115 = _51 - _51;
_114.0 = [(*_30),_82,(*_30),(*_30)];
_37 = _72.1;
_27 = 2929936677111389080_i64 - (-8702703983355747975_i64);
(*_30) = _40;
_104.3 = _82;
(*_50) = (*_94) as isize;
Goto(bb51)
}
bb51 = {
Call(_128 = dump_var(Move(_88), Move(_28), Move(_25), Move(_46)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_128 = dump_var(Move(_16), Move(_91), Move(_65), Move(_100)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_128 = dump_var(Move(_37), Move(_55), Move(_101), Move(_77)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_128 = dump_var(Move(_12), Move(_62), Move(_79), Move(_113)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_128 = dump_var(Move(_49), Move(_4), Move(_56), Move(_27)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_128 = dump_var(Move(_24), Move(_93), Move(_6), Move(_64)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_128 = dump_var(Move(_19), Move(_95), Move(_106), Move(_38)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_128 = dump_var(Move(_45), _129, _129, _129), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut char) -> bool {
mir! {
type RET = bool;
let _2: (&'static mut &'static f64, i16, usize, [char; 4]);
let _3: *mut u32;
let _4: f32;
let _5: bool;
let _6: isize;
let _7: bool;
let _8: &'static &'static mut u8;
let _9: bool;
let _10: &'static i32;
let _11: &'static i32;
let _12: ();
let _13: ();
{
_2.2 = 9191193774275203586_usize | 15917850842647596296_usize;
_2.1 = !(-22199_i16);
_2.3 = ['\u{8f443}','\u{f0b33}','\u{cf751}','\u{5a40}'];
_2.1 = 10897_i16;
RET = _2.2 < _2.2;
_2.2 = 7_usize >> _2.1;
_2.3 = ['\u{1d9f0}','\u{b6448}','\u{749c0}','\u{b4eba}'];
_2.2 = 5845930730803272909_usize >> _2.1;
RET = true & true;
_2.2 = 189086233857141688503637051192377360989_u128 as usize;
RET = !false;
_4 = 12751_u16 as f32;
RET = !false;
_2.1 = 30710_i16 >> _2.2;
_2.1 = 6021_i16;
_2.1 = (-15783_i16);
_2.1 = -29691_i16;
_2.1 = (-24229_i16) << _2.2;
RET = _2.1 < _2.1;
_4 = _2.1 as f32;
_4 = 195_u8 as f32;
_2.2 = 3_usize;
_2.3 = ['\u{9f965}','\u{af3e9}','\u{f1e4c}','\u{985cc}'];
RET = !false;
_4 = _2.2 as f32;
_2.1 = (-18561_i16);
Goto(bb1)
}
bb1 = {
_5 = RET;
_2.1 = 17850_i16 + (-1658_i16);
_5 = RET | RET;
match _2.2 {
3 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = _5 | _5;
_4 = (-105_i8) as f32;
_2.2 = 0_usize;
_2.2 = 5_usize;
RET = !_5;
_2.2 = 7_usize;
_2.2 = 3_usize << _2.1;
RET = _5 | _5;
_6 = 9223372036854775807_isize;
_5 = RET ^ RET;
_2.1 = -(-32491_i16);
_4 = 55_u8 as f32;
_7 = !_5;
_2.2 = !7_usize;
_2.3 = ['\u{adbdf}','\u{ff41}','\u{f2670}','\u{c3dfa}'];
_2.2 = 3_usize + 6_usize;
_6 = 9223372036854775807_isize;
_7 = _5;
_7 = RET & RET;
Goto(bb4)
}
bb4 = {
_5 = !_7;
_5 = !RET;
_5 = !_7;
_5 = _6 == _6;
_2.3 = ['\u{69acd}','\u{91b54}','\u{39c46}','\u{5c245}'];
_9 = _7 > _5;
_4 = (-92528934957622296829738531788581080201_i128) as f32;
_7 = RET ^ _9;
_4 = 5484272830483651691_i64 as f32;
_2.1 = 7684_i16;
_4 = 3326743876_u32 as f32;
_4 = 210_u8 as f32;
_9 = _5 < RET;
_2.1 = (-20479_i16) + 773_i16;
_5 = _7 < _7;
_6 = (-112_isize) - 7_isize;
RET = _5 < _5;
_2.3 = ['\u{7019a}','\u{ad194}','\u{c6ff8}','\u{de093}'];
Goto(bb5)
}
bb5 = {
Call(_12 = dump_var(Move(_5), Move(_6), _13, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: bool,mut _2: char,mut _3: *mut char,mut _4: isize,mut _5: bool,mut _6: f32,mut _7: char,mut _8: char,mut _9: isize,mut _10: char,mut _11: char) -> isize {
mir! {
type RET = isize;
let _12: f32;
let _13: [i8; 8];
let _14: u8;
let _15: *mut char;
let _16: &'static mut &'static usize;
let _17: [i128; 2];
let _18: Adt55;
let _19: isize;
let _20: *const u128;
let _21: bool;
let _22: i128;
let _23: *mut i16;
let _24: [i128; 2];
let _25: char;
let _26: Adt28;
let _27: bool;
let _28: *const i16;
let _29: isize;
let _30: f64;
let _31: &'static f64;
let _32: [bool; 2];
let _33: &'static mut i8;
let _34: isize;
let _35: i16;
let _36: isize;
let _37: bool;
let _38: [char; 4];
let _39: i16;
let _40: usize;
let _41: i16;
let _42: ([char; 4], u8, [char; 4]);
let _43: [u128; 2];
let _44: isize;
let _45: [u64; 7];
let _46: bool;
let _47: *const char;
let _48: ([char; 4], [u8; 2], &'static mut isize, char);
let _49: isize;
let _50: &'static &'static mut *mut char;
let _51: *const i16;
let _52: f64;
let _53: i64;
let _54: [u32; 7];
let _55: f32;
let _56: f64;
let _57: [char; 4];
let _58: [u8; 2];
let _59: (*mut &'static f64, &'static mut isize);
let _60: *const u128;
let _61: *const i16;
let _62: isize;
let _63: i128;
let _64: *mut *mut &'static f64;
let _65: u128;
let _66: u32;
let _67: Adt43;
let _68: ([char; 4], [u8; 2], &'static mut isize, char);
let _69: &'static mut &'static f64;
let _70: f32;
let _71: f32;
let _72: i32;
let _73: &'static usize;
let _74: bool;
let _75: &'static &'static mut u8;
let _76: u8;
let _77: i32;
let _78: &'static mut i8;
let _79: f64;
let _80: usize;
let _81: u64;
let _82: &'static &'static mut u8;
let _83: isize;
let _84: u64;
let _85: isize;
let _86: [u32; 7];
let _87: Adt55;
let _88: &'static mut &'static f64;
let _89: isize;
let _90: u128;
let _91: char;
let _92: f32;
let _93: f64;
let _94: [char; 4];
let _95: i8;
let _96: *const &'static mut &'static f64;
let _97: Adt44;
let _98: &'static mut u16;
let _99: &'static mut *mut i16;
let _100: [bool; 2];
let _101: *mut char;
let _102: i16;
let _103: bool;
let _104: *mut ([char; 4], u8, [char; 4]);
let _105: i8;
let _106: usize;
let _107: [i8; 8];
let _108: u128;
let _109: char;
let _110: ();
let _111: ();
{
_4 = _9;
_5 = _9 <= _4;
_7 = _2;
_3 = core::ptr::addr_of_mut!(_11);
(*_3) = _8;
(*_3) = _2;
(*_3) = _8;
Call((*_3) = fn11(_9, _4, _1, Move(_3), _10, _7, _4, _2, _7, _10, _6, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _4 as f32;
_12 = _6 * _6;
_11 = _10;
_3 = core::ptr::addr_of_mut!(_11);
(*_3) = _8;
RET = (*_3) as isize;
(*_3) = _8;
_15 = core::ptr::addr_of_mut!((*_3));
(*_3) = _7;
Goto(bb2)
}
bb2 = {
_17 = [(-64216177373399762467441826137483026822_i128),78983677177798920162658344137890973902_i128];
_9 = RET ^ _4;
_19 = _4;
(*_3) = _10;
_14 = !58_u8;
_12 = _6 * _6;
_10 = (*_3);
_22 = (-163037567447376295962962821674380943172_i128) ^ (-62362037912603541103359731130194055821_i128);
_17 = [_22,_22];
_7 = (*_3);
Goto(bb3)
}
bb3 = {
_6 = 13246_i16 as f32;
_2 = (*_3);
_24 = [_22,_22];
(*_3) = _7;
(*_3) = _8;
_21 = !_5;
_9 = _19 >> _19;
_22 = _14 as i128;
Goto(bb4)
}
bb4 = {
_2 = (*_3);
(*_3) = _7;
(*_3) = _7;
_10 = (*_3);
(*_3) = _7;
(*_3) = _7;
_25 = _8;
_14 = 63_u8 - 75_u8;
(*_3) = _2;
(*_3) = _7;
_10 = (*_3);
_3 = core::ptr::addr_of_mut!((*_3));
RET = 1103032607_u32 as isize;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
(*_3) = _25;
(*_3) = _8;
_22 = 128648183_u32 as i128;
_9 = _4 | RET;
_11 = _2;
(*_3) = _7;
_27 = _21 | _1;
(*_3) = _7;
_8 = _2;
(*_3) = _7;
Goto(bb5)
}
bb5 = {
(*_3) = _7;
_30 = _9 as f64;
(*_3) = _8;
(*_3) = _10;
_29 = _9;
Goto(bb6)
}
bb6 = {
(*_3) = _25;
(*_3) = _25;
Goto(bb7)
}
bb7 = {
RET = _19;
_29 = _19 - RET;
(*_3) = _10;
_10 = _25;
(*_3) = _25;
_26.fld4 = 29140_u16 as usize;
_19 = _4;
(*_3) = _10;
(*_3) = _10;
_11 = _8;
(*_3) = _8;
Goto(bb8)
}
bb8 = {
(*_3) = _10;
_3 = Move(_15);
_15 = core::ptr::addr_of_mut!(_7);
Goto(bb9)
}
bb9 = {
_31 = &_30;
_21 = _1 & _5;
_13 = [(-59_i8),71_i8,(-93_i8),65_i8,102_i8,92_i8,(-128_i8),7_i8];
Goto(bb10)
}
bb10 = {
(*_15) = _8;
_30 = (-23857_i16) as f64;
_11 = (*_15);
(*_15) = _8;
(*_15) = _25;
_3 = core::ptr::addr_of_mut!((*_15));
(*_3) = _11;
_26.fld1 = (-354583226_i32) as u32;
_27 = _21;
(*_3) = _25;
_21 = (*_3) == _11;
(*_15) = _10;
_26.fld0 = 38262_u16 >> _22;
(*_15) = _8;
_32 = [_27,_27];
(*_15) = _10;
(*_15) = _8;
_10 = (*_15);
(*_15) = _11;
_35 = -19489_i16;
_34 = _29 + _4;
Goto(bb11)
}
bb11 = {
_24 = [_22,_22];
_26.fld4 = 4_usize - 7_usize;
_24 = _17;
_38 = [_10,_10,(*_15),(*_15)];
(*_15) = _8;
_9 = 52_i8 as isize;
_14 = (-7660243511055639114_i64) as u8;
_28 = core::ptr::addr_of!(_39);
(*_28) = _22 as i16;
_36 = _34 + RET;
_26.fld1 = 574454372_u32 * 805062135_u32;
(*_28) = _30 as i16;
_39 = _35 & _35;
_26.fld2 = (-128_i8) as f32;
(*_15) = _8;
_40 = _26.fld4 + _26.fld4;
_7 = _10;
_14 = 118_u8 & 255_u8;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
_26.fld1 = _14 as u32;
_8 = (*_15);
_17 = _24;
(*_28) = !_35;
_42.2 = _38;
_26.fld4 = _40 ^ _40;
(*_28) = _26.fld0 as i16;
Goto(bb12)
}
bb12 = {
_41 = (*_28) + (*_28);
_13 = [26_i8,(-105_i8),117_i8,(-87_i8),(-50_i8),(-3_i8),(-99_i8),30_i8];
(*_28) = _41 | _41;
(*_28) = _41;
_28 = core::ptr::addr_of!((*_28));
_13 = [118_i8,(-26_i8),(-113_i8),3_i8,101_i8,(-1_i8),98_i8,2_i8];
_19 = !_9;
_23 = core::ptr::addr_of_mut!((*_28));
(*_28) = _41 >> _34;
_42 = (_38, _14, _38);
(*_15) = _8;
(*_15) = _25;
(*_28) = _41 + _41;
_44 = _34;
_21 = (*_28) <= (*_28);
Goto(bb13)
}
bb13 = {
(*_28) = _26.fld0 as i16;
(*_15) = _10;
_28 = core::ptr::addr_of!((*_28));
_36 = RET >> (*_28);
_48.2 = &mut _4;
(*_15) = _25;
(*_28) = _26.fld4 as i16;
_26.fld4 = !_40;
_45 = [14288257719763809865_u64,10184476356569121276_u64,13159509921534384790_u64,5622052451590449803_u64,5010995270396126810_u64,11909701408381327309_u64,9566812735920436083_u64];
_8 = (*_15);
(*_15) = _10;
(*_28) = _35;
_49 = -_34;
(*_28) = _41 * _35;
(*_28) = -_41;
_29 = !_49;
_52 = _30;
_5 = _42.1 == _14;
_29 = _26.fld1 as isize;
(*_15) = _8;
_13 = [86_i8,95_i8,65_i8,54_i8,116_i8,39_i8,39_i8,50_i8];
_48.0 = _38;
_22 = _26.fld0 as i128;
_32 = [_27,_21];
(*_15) = _25;
Goto(bb14)
}
bb14 = {
(*_28) = 254914440182771939845236540284066671816_u128 as i16;
(*_15) = _11;
(*_15) = _11;
_8 = (*_15);
_49 = _29 ^ _44;
_53 = (-2469682280720681021_i64) ^ 5025360862382178280_i64;
_26.fld1 = 1144807171_u32 ^ 972665860_u32;
_28 = core::ptr::addr_of!((*_28));
_45 = [11918568466035081982_u64,3472412581852881961_u64,1466020302787262698_u64,12938444604486364639_u64,10125945752322977930_u64,5541469164344142482_u64,15076360925110048100_u64];
_10 = (*_15);
(*_15) = _2;
_26.fld0 = 12943421106888201028_u64 as u16;
(*_28) = _42.1 as i16;
_21 = _27 | _1;
_3 = core::ptr::addr_of_mut!(_2);
(*_3) = (*_15);
_49 = _44 >> (*_28);
_47 = core::ptr::addr_of!((*_15));
(*_3) = (*_47);
(*_3) = (*_15);
(*_15) = _10;
_8 = (*_15);
(*_15) = (*_3);
(*_28) = _35;
_56 = _53 as f64;
_36 = -_29;
(*_28) = _41 + _41;
_25 = (*_3);
Call((*_28) = core::intrinsics::transmute(_41), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_51 = core::ptr::addr_of!((*_28));
_37 = _27 & _21;
(*_3) = (*_15);
(*_3) = (*_15);
(*_51) = !_35;
(*_15) = (*_3);
_47 = core::ptr::addr_of!((*_3));
_22 = -(-130263043715370551674954865410616237306_i128);
(*_51) = _35;
_42.1 = _14 + _14;
(*_28) = !_41;
_55 = -_12;
_2 = _10;
_42.2 = [(*_3),(*_3),(*_3),(*_3)];
(*_3) = (*_15);
(*_15) = (*_3);
(*_28) = -_41;
_18 = Adt55::Variant1 { fld0: Move(_15),fld1: Move(_26.fld3),fld2: _44,fld3: _26.fld0,fld4: (*_28) };
_52 = _30 * _56;
_26.fld3 = Move(Field::<*mut u32>(Variant(_18, 1), 1));
_15 = core::ptr::addr_of_mut!((*_3));
_26.fld1 = 3409149918_u32 >> (*_28);
_58 = [_42.1,_42.1];
(*_28) = -Field::<i16>(Variant(_18, 1), 4);
(*_15) = _10;
Goto(bb16)
}
bb16 = {
_56 = -_52;
_1 = !_37;
(*_3) = _8;
(*_3) = _11;
(*_3) = _8;
_31 = &_52;
_54 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1];
(*_3) = _11;
(*_3) = _8;
_48.2 = &mut _34;
place!(Field::<*mut u32>(Variant(_18, 1), 1)) = Move(_26.fld3);
_48.3 = (*_3);
_10 = _25;
(*_28) = (*_3) as i16;
_7 = (*_3);
_59.1 = &mut _9;
_59.1 = &mut RET;
(*_28) = Field::<i16>(Variant(_18, 1), 4);
(*_28) = Field::<i16>(Variant(_18, 1), 4);
(*_28) = Field::<i16>(Variant(_18, 1), 4) + _41;
_56 = -(*_31);
_36 = _49 * _49;
(*_3) = _11;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
_26.fld2 = -_55;
place!(Field::<u16>(Variant(_18, 1), 3)) = _26.fld0;
_31 = &_30;
_48.1 = [_14,_14];
_39 = _40 as i16;
Goto(bb17)
}
bb17 = {
(*_3) = _7;
_47 = core::ptr::addr_of!((*_3));
(*_28) = Field::<i16>(Variant(_18, 1), 4) - _41;
_26.fld3 = Move(Field::<*mut u32>(Variant(_18, 1), 1));
_48.2 = &mut _29;
(*_28) = _41 ^ Field::<i16>(Variant(_18, 1), 4);
_43 = [188873424317438482191170238660585729761_u128,272164046260191948182731375443255726867_u128];
(*_47) = _8;
(*_47) = _11;
(*_47) = _11;
_48 = (_42.2, _58, Move(_59.1), _25);
_49 = 20_i8 as isize;
(*_47) = _8;
(*_28) = 1122168996_i32 as i16;
(*_3) = _8;
place!(Field::<u16>(Variant(_18, 1), 3)) = _26.fld0 >> _44;
_17 = [_22,_22];
Goto(bb18)
}
bb18 = {
_57 = [(*_3),(*_3),_11,_25];
_42.0 = [(*_3),(*_3),(*_3),(*_3)];
(*_28) = _41;
(*_3) = _10;
_42.2 = [_48.3,(*_3),_48.3,(*_3)];
_2 = _7;
_20 = core::ptr::addr_of!(_65);
_8 = (*_3);
_63 = _26.fld4 as i128;
Goto(bb19)
}
bb19 = {
_35 = (*_28) & (*_28);
_42.1 = !_14;
_37 = !_21;
_27 = _21;
(*_20) = 333245695313913704987418400005285228535_u128;
(*_20) = _12 as u128;
_59.0 = core::ptr::addr_of_mut!(_31);
_41 = (*_28);
_15 = core::ptr::addr_of_mut!((*_3));
_38 = _42.2;
(*_20) = (*_15) as u128;
(*_15) = _48.3;
_35 = (*_28) & (*_28);
_42 = (_38, _14, _57);
_60 = core::ptr::addr_of!((*_20));
(*_60) = _53 as u128;
_63 = _22 & _22;
_24 = [_63,_63];
(*_28) = _41 ^ _35;
(*_28) = _26.fld1 as i16;
_26.fld4 = _40 + _40;
_26.fld0 = _53 as u16;
(*_60) = 121046956460892427445250459692470796253_u128;
Goto(bb20)
}
bb20 = {
_11 = (*_15);
(*_28) = !Field::<i16>(Variant(_18, 1), 4);
_70 = _40 as f32;
(*_28) = _41;
_71 = _12 + _12;
(*_20) = !297240126000576611573423823290985759286_u128;
_68.0 = _42.0;
_10 = (*_3);
(*_28) = _35 & _35;
Goto(bb21)
}
bb21 = {
_1 = _21;
_26.fld1 = 949061048_u32 - 515355313_u32;
_67 = Adt43::Variant2 { fld0: _14,fld1: _26.fld4,fld2: Move(_23) };
(*_15) = _48.3;
(*_15) = _48.3;
_59.0 = core::ptr::addr_of_mut!(_31);
(*_20) = 221623874116529836497671051113619627593_u128 + 194098182760058522778993943410984851037_u128;
place!(Field::<u16>(Variant(_18, 1), 3)) = _26.fld0 & _26.fld0;
_65 = 149904208648509561890948024217436376472_u128 + 45189159882325271941021107132321544395_u128;
_64 = core::ptr::addr_of_mut!(_59.0);
_46 = _1;
(*_15) = _8;
(*_28) = _41 >> (*_20);
_71 = _70;
(*_28) = Field::<i16>(Variant(_18, 1), 4);
Goto(bb22)
}
bb22 = {
(*_15) = _48.3;
(*_64) = core::ptr::addr_of_mut!(_31);
(*_64) = core::ptr::addr_of_mut!(_31);
_42.1 = Field::<u8>(Variant(_67, 2), 0) * Field::<u8>(Variant(_67, 2), 0);
_44 = !Field::<isize>(Variant(_18, 1), 2);
_31 = &_56;
(*_28) = _41;
(*_20) = 148267938808989217477320183189844188513_u128 | 40830060113925073545778856537317749829_u128;
(*_3) = _11;
(*_28) = _35 << (*_20);
_22 = _63;
(*_20) = !161905141310610293171495351888810772631_u128;
_42.2 = [_7,(*_3),(*_3),_7];
(*_20) = 296153595869965557919703643507636284274_u128 - 162917965005138618318539032450757798464_u128;
(*_3) = _11;
_21 = _37;
_61 = core::ptr::addr_of!((*_28));
(*_64) = core::ptr::addr_of_mut!(_31);
_6 = _55;
place!(Field::<*mut u32>(Variant(_18, 1), 1)) = Move(_26.fld3);
(*_3) = _11;
_42.2 = _48.0;
_35 = -(*_28);
_7 = _10;
_40 = 92_i8 as usize;
_17 = _24;
Goto(bb23)
}
bb23 = {
(*_20) = 214470267048303802830597637299575971168_u128 << (*_28);
_37 = _5 & _5;
_57 = [(*_3),(*_3),(*_3),(*_3)];
_68.0 = [(*_3),(*_3),(*_3),_10];
_45 = [8071421123021645919_u64,12365610693963075184_u64,2916762473135900793_u64,4592925534671889431_u64,338793203477180868_u64,18382261169684861107_u64,4104389195685413618_u64];
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _11;
_64 = core::ptr::addr_of_mut!((*_64));
(*_3) = _7;
(*_28) = Field::<i16>(Variant(_18, 1), 4) - _35;
(*_64) = core::ptr::addr_of_mut!(_31);
(*_28) = _42.1 as i16;
_73 = &_40;
(*_28) = _35 << (*_20);
_55 = _71 + _70;
(*_28) = (*_73) as i16;
(*_28) = _35 | _41;
_48.3 = _11;
Call((*_20) = core::intrinsics::bswap(325585385873530601572739685493614770592_u128), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_42 = (_68.0, _14, _38);
place!(Field::<usize>(Variant(_67, 2), 1)) = (*_73) + (*_73);
_72 = -356266828_i32;
Goto(bb25)
}
bb25 = {
_15 = core::ptr::addr_of_mut!((*_3));
_35 = (*_28) >> (*_28);
_69 = &mut _31;
_54 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1];
(*_28) = _35 * _35;
(*_28) = _35 & _35;
_79 = _56;
(*_3) = _10;
_19 = _36;
Goto(bb26)
}
bb26 = {
(*_28) = _79 as i16;
(*_20) = 162839312636195559923779075108354531677_u128;
_76 = !_42.1;
(*_20) = 12656901204414411771228056122653894186_u128 << Field::<u8>(Variant(_67, 2), 0);
(*_3) = _8;
(*_28) = _35 + _35;
_48.2 = &mut _44;
_68.1 = _58;
_37 = _27;
(*_3) = _25;
_42.0 = [(*_3),(*_3),(*_3),(*_3)];
_1 = !_27;
_23 = core::ptr::addr_of_mut!((*_28));
_39 = _35;
Goto(bb27)
}
bb27 = {
(*_69) = &_30;
(*_64) = core::ptr::addr_of_mut!((*_69));
_26.fld2 = -_70;
_64 = core::ptr::addr_of_mut!((*_64));
(*_20) = 197228437287749074088573946654876057808_u128;
(*_3) = _25;
(*_64) = core::ptr::addr_of_mut!((*_69));
match (*_20) {
0 => bb22,
197228437287749074088573946654876057808 => bb29,
_ => bb28
}
}
bb28 = {
_24 = [_22,_22];
_26.fld4 = 4_usize - 7_usize;
_24 = _17;
_38 = [_10,_10,(*_15),(*_15)];
(*_15) = _8;
_9 = 52_i8 as isize;
_14 = (-7660243511055639114_i64) as u8;
_28 = core::ptr::addr_of!(_39);
(*_28) = _22 as i16;
_36 = _34 + RET;
_26.fld1 = 574454372_u32 * 805062135_u32;
(*_28) = _30 as i16;
_39 = _35 & _35;
_26.fld2 = (-128_i8) as f32;
(*_15) = _8;
_40 = _26.fld4 + _26.fld4;
_7 = _10;
_14 = 118_u8 & 255_u8;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
_26.fld1 = _14 as u32;
_8 = (*_15);
_17 = _24;
(*_28) = !_35;
_42.2 = _38;
_26.fld4 = _40 ^ _40;
(*_28) = _26.fld0 as i16;
Goto(bb12)
}
bb29 = {
_6 = -_26.fld2;
_59.1 = &mut place!(Field::<isize>(Variant(_18, 1), 2));
_32 = [_37,_1];
(*_23) = -_35;
_81 = 10216700326841787149_u64 << (*_28);
(*_69) = &_52;
(*_28) = -_35;
(*_69) = &_79;
(*_20) = 327625637972677283933818683831265149610_u128 & 17245692442501617870502610621694566762_u128;
Goto(bb30)
}
bb30 = {
_47 = core::ptr::addr_of!(_25);
Goto(bb31)
}
bb31 = {
(*_69) = &_30;
(*_20) = 98652299735198975165419026036671922493_u128 + 112924955994061476355917521950551774524_u128;
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_28) = _35 + _35;
_91 = _48.3;
(*_64) = core::ptr::addr_of_mut!((*_69));
_58 = _68.1;
place!(Field::<usize>(Variant(_67, 2), 1)) = !_26.fld4;
(*_69) = &_52;
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_3) = _25;
Goto(bb32)
}
bb32 = {
_61 = core::ptr::addr_of!((*_28));
(*_28) = _37 as i16;
(*_20) = 75550193727919689171345362765066008816_u128 << _26.fld0;
_10 = (*_3);
(*_64) = core::ptr::addr_of_mut!((*_69));
_55 = -_70;
(*_20) = _26.fld1 as u128;
_74 = _5;
(*_47) = (*_3);
(*_20) = !315957862136240979807804069468935647753_u128;
_93 = -_52;
_61 = core::ptr::addr_of!((*_28));
_61 = core::ptr::addr_of!((*_28));
(*_64) = core::ptr::addr_of_mut!((*_69));
_59.1 = &mut _36;
_58 = [Field::<u8>(Variant(_67, 2), 0),_76];
(*_28) = _35 | _35;
(*_47) = (*_3);
_68.2 = &mut _49;
_83 = -_19;
_45 = [_81,_81,_81,_81,_81,_81,_81];
(*_3) = (*_47);
(*_47) = (*_3);
_10 = (*_47);
(*_20) = 200361962282384584910868027815507009571_u128;
_99 = &mut place!(Field::<*mut i16>(Variant(_67, 2), 2));
match (*_20) {
0 => bb15,
1 => bb11,
2 => bb33,
3 => bb34,
4 => bb35,
5 => bb36,
200361962282384584910868027815507009571 => bb38,
_ => bb37
}
}
bb33 = {
(*_69) = &_30;
(*_20) = 98652299735198975165419026036671922493_u128 + 112924955994061476355917521950551774524_u128;
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_28) = _35 + _35;
_91 = _48.3;
(*_64) = core::ptr::addr_of_mut!((*_69));
_58 = _68.1;
place!(Field::<usize>(Variant(_67, 2), 1)) = !_26.fld4;
(*_69) = &_52;
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_3) = _25;
Goto(bb32)
}
bb34 = {
_1 = _21;
_26.fld1 = 949061048_u32 - 515355313_u32;
_67 = Adt43::Variant2 { fld0: _14,fld1: _26.fld4,fld2: Move(_23) };
(*_15) = _48.3;
(*_15) = _48.3;
_59.0 = core::ptr::addr_of_mut!(_31);
(*_20) = 221623874116529836497671051113619627593_u128 + 194098182760058522778993943410984851037_u128;
place!(Field::<u16>(Variant(_18, 1), 3)) = _26.fld0 & _26.fld0;
_65 = 149904208648509561890948024217436376472_u128 + 45189159882325271941021107132321544395_u128;
_64 = core::ptr::addr_of_mut!(_59.0);
_46 = _1;
(*_15) = _8;
(*_28) = _41 >> (*_20);
_71 = _70;
(*_28) = Field::<i16>(Variant(_18, 1), 4);
Goto(bb22)
}
bb35 = {
(*_3) = _7;
_30 = _9 as f64;
(*_3) = _8;
(*_3) = _10;
_29 = _9;
Goto(bb6)
}
bb36 = {
(*_28) = 254914440182771939845236540284066671816_u128 as i16;
(*_15) = _11;
(*_15) = _11;
_8 = (*_15);
_49 = _29 ^ _44;
_53 = (-2469682280720681021_i64) ^ 5025360862382178280_i64;
_26.fld1 = 1144807171_u32 ^ 972665860_u32;
_28 = core::ptr::addr_of!((*_28));
_45 = [11918568466035081982_u64,3472412581852881961_u64,1466020302787262698_u64,12938444604486364639_u64,10125945752322977930_u64,5541469164344142482_u64,15076360925110048100_u64];
_10 = (*_15);
(*_15) = _2;
_26.fld0 = 12943421106888201028_u64 as u16;
(*_28) = _42.1 as i16;
_21 = _27 | _1;
_3 = core::ptr::addr_of_mut!(_2);
(*_3) = (*_15);
_49 = _44 >> (*_28);
_47 = core::ptr::addr_of!((*_15));
(*_3) = (*_47);
(*_3) = (*_15);
(*_15) = _10;
_8 = (*_15);
(*_15) = (*_3);
(*_28) = _35;
_56 = _53 as f64;
_36 = -_29;
(*_28) = _41 + _41;
_25 = (*_3);
Call((*_28) = core::intrinsics::transmute(_41), ReturnTo(bb15), UnwindUnreachable())
}
bb37 = {
_11 = (*_15);
(*_28) = !Field::<i16>(Variant(_18, 1), 4);
_70 = _40 as f32;
(*_28) = _41;
_71 = _12 + _12;
(*_20) = !297240126000576611573423823290985759286_u128;
_68.0 = _42.0;
_10 = (*_3);
(*_28) = _35 & _35;
Goto(bb21)
}
bb38 = {
_70 = _55 + _71;
_42.0 = [(*_47),(*_3),(*_47),(*_47)];
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_3) = (*_47);
(*_69) = &_56;
Goto(bb39)
}
bb39 = {
(*_64) = core::ptr::addr_of_mut!((*_69));
_26.fld0 = !49321_u16;
_39 = _35;
_89 = !_19;
(*_69) = &_79;
_11 = (*_3);
(*_28) = _19 as i16;
_73 = &_26.fld4;
_90 = !(*_20);
match (*_20) {
0 => bb40,
1 => bb41,
2 => bb42,
200361962282384584910868027815507009571 => bb44,
_ => bb43
}
}
bb40 = {
_1 = _21;
_26.fld1 = 949061048_u32 - 515355313_u32;
_67 = Adt43::Variant2 { fld0: _14,fld1: _26.fld4,fld2: Move(_23) };
(*_15) = _48.3;
(*_15) = _48.3;
_59.0 = core::ptr::addr_of_mut!(_31);
(*_20) = 221623874116529836497671051113619627593_u128 + 194098182760058522778993943410984851037_u128;
place!(Field::<u16>(Variant(_18, 1), 3)) = _26.fld0 & _26.fld0;
_65 = 149904208648509561890948024217436376472_u128 + 45189159882325271941021107132321544395_u128;
_64 = core::ptr::addr_of_mut!(_59.0);
_46 = _1;
(*_15) = _8;
(*_28) = _41 >> (*_20);
_71 = _70;
(*_28) = Field::<i16>(Variant(_18, 1), 4);
Goto(bb22)
}
bb41 = {
_11 = (*_15);
(*_28) = !Field::<i16>(Variant(_18, 1), 4);
_70 = _40 as f32;
(*_28) = _41;
_71 = _12 + _12;
(*_20) = !297240126000576611573423823290985759286_u128;
_68.0 = _42.0;
_10 = (*_3);
(*_28) = _35 & _35;
Goto(bb21)
}
bb42 = {
_24 = [_22,_22];
_26.fld4 = 4_usize - 7_usize;
_24 = _17;
_38 = [_10,_10,(*_15),(*_15)];
(*_15) = _8;
_9 = 52_i8 as isize;
_14 = (-7660243511055639114_i64) as u8;
_28 = core::ptr::addr_of!(_39);
(*_28) = _22 as i16;
_36 = _34 + RET;
_26.fld1 = 574454372_u32 * 805062135_u32;
(*_28) = _30 as i16;
_39 = _35 & _35;
_26.fld2 = (-128_i8) as f32;
(*_15) = _8;
_40 = _26.fld4 + _26.fld4;
_7 = _10;
_14 = 118_u8 & 255_u8;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
_26.fld1 = _14 as u32;
_8 = (*_15);
_17 = _24;
(*_28) = !_35;
_42.2 = _38;
_26.fld4 = _40 ^ _40;
(*_28) = _26.fld0 as i16;
Goto(bb12)
}
bb43 = {
(*_3) = _25;
(*_3) = _25;
Goto(bb7)
}
bb44 = {
(*_3) = (*_47);
(*_99) = core::ptr::addr_of_mut!((*_28));
(*_99) = core::ptr::addr_of_mut!(_35);
_68.3 = (*_3);
(*_64) = core::ptr::addr_of_mut!((*_69));
_98 = &mut _26.fld0;
_91 = (*_3);
_10 = (*_3);
_40 = (*_73) << (*_28);
match (*_20) {
0 => bb45,
1 => bb46,
2 => bb47,
3 => bb48,
200361962282384584910868027815507009571 => bb50,
_ => bb49
}
}
bb45 = {
_31 = &_30;
_21 = _1 & _5;
_13 = [(-59_i8),71_i8,(-93_i8),65_i8,102_i8,92_i8,(-128_i8),7_i8];
Goto(bb10)
}
bb46 = {
_2 = (*_3);
(*_3) = _7;
(*_3) = _7;
_10 = (*_3);
(*_3) = _7;
(*_3) = _7;
_25 = _8;
_14 = 63_u8 - 75_u8;
(*_3) = _2;
(*_3) = _7;
_10 = (*_3);
_3 = core::ptr::addr_of_mut!((*_3));
RET = 1103032607_u32 as isize;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld1);
(*_3) = _25;
(*_3) = _8;
_22 = 128648183_u32 as i128;
_9 = _4 | RET;
_11 = _2;
(*_3) = _7;
_27 = _21 | _1;
(*_3) = _7;
_8 = _2;
(*_3) = _7;
Goto(bb5)
}
bb47 = {
(*_3) = _7;
_30 = _9 as f64;
(*_3) = _8;
(*_3) = _10;
_29 = _9;
Goto(bb6)
}
bb48 = {
(*_3) = _25;
(*_3) = _25;
Goto(bb7)
}
bb49 = {
_41 = (*_28) + (*_28);
_13 = [26_i8,(-105_i8),117_i8,(-87_i8),(-50_i8),(-3_i8),(-99_i8),30_i8];
(*_28) = _41 | _41;
(*_28) = _41;
_28 = core::ptr::addr_of!((*_28));
_13 = [118_i8,(-26_i8),(-113_i8),3_i8,101_i8,(-1_i8),98_i8,2_i8];
_19 = !_9;
_23 = core::ptr::addr_of_mut!((*_28));
(*_28) = _41 >> _34;
_42 = (_38, _14, _38);
(*_15) = _8;
(*_15) = _25;
(*_28) = _41 + _41;
_44 = _34;
_21 = (*_28) <= (*_28);
Goto(bb13)
}
bb50 = {
(*_99) = Move(_23);
(*_99) = core::ptr::addr_of_mut!((*_28));
(*_98) = 47158_u16 | 60839_u16;
_80 = !(*_73);
(*_98) = 29349_u16 * 49735_u16;
(*_47) = _91;
_48.2 = &mut _83;
(*_99) = core::ptr::addr_of_mut!((*_28));
_16 = &mut _73;
(*_69) = &_52;
(*_16) = &_40;
_89 = _19;
(*_69) = &_79;
(*_69) = &_52;
(*_3) = (*_47);
(*_16) = &_80;
_37 = !_46;
(*_3) = (*_47);
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_64) = core::ptr::addr_of_mut!((*_69));
(*_28) = _35;
(*_98) = 6750_u16;
Goto(bb51)
}
bb51 = {
Call(_110 = dump_var(Move(_63), Move(_25), Move(_39), Move(_42)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_110 = dump_var(Move(_10), Move(_38), Move(_41), Move(_27)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_110 = dump_var(Move(_45), Move(_36), Move(_54), Move(_74)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_110 = dump_var(Move(_80), Move(_43), Move(_83), Move(_14)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_110 = dump_var(Move(_17), Move(_11), Move(_81), Move(_57)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_110 = dump_var(Move(_7), Move(_22), Move(_24), Move(_35)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: *mut char,mut _5: char,mut _6: char,mut _7: isize,mut _8: char,mut _9: char,mut _10: char,mut _11: f32,mut _12: char) -> char {
mir! {
type RET = char;
let _13: *const &'static mut u8;
let _14: f64;
let _15: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _16: &'static &'static mut *mut char;
let _17: bool;
let _18: (*const i16,);
let _19: (u128, usize, [i32; 1]);
let _20: f32;
let _21: (*mut *mut i16, i32, Adt55, *mut i16);
let _22: isize;
let _23: isize;
let _24: i128;
let _25: f32;
let _26: *const u128;
let _27: *mut [u32; 7];
let _28: usize;
let _29: *const &'static mut &'static f64;
let _30: [i64; 3];
let _31: isize;
let _32: &'static mut u8;
let _33: f32;
let _34: f64;
let _35: (&'static f64, [u32; 7]);
let _36: char;
let _37: f64;
let _38: f32;
let _39: &'static usize;
let _40: *const &'static mut u8;
let _41: [i8; 8];
let _42: Adt28;
let _43: f32;
let _44: i32;
let _45: i32;
let _46: isize;
let _47: &'static f32;
let _48: char;
let _49: f64;
let _50: &'static i32;
let _51: u8;
let _52: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _53: u16;
let _54: [usize; 7];
let _55: *const u128;
let _56: &'static mut *const i16;
let _57: *mut *mut i16;
let _58: (&'static f64, [u32; 7]);
let _59: bool;
let _60: i8;
let _61: &'static mut i8;
let _62: u32;
let _63: isize;
let _64: f64;
let _65: isize;
let _66: &'static mut u8;
let _67: bool;
let _68: isize;
let _69: bool;
let _70: isize;
let _71: i8;
let _72: *mut ([char; 4], u8, [char; 4]);
let _73: f64;
let _74: i32;
let _75: isize;
let _76: f32;
let _77: u128;
let _78: ([char; 4], [u8; 2], &'static mut isize, char);
let _79: f64;
let _80: f64;
let _81: *mut *mut &'static f64;
let _82: *mut [u32; 7];
let _83: isize;
let _84: [u128; 2];
let _85: i128;
let _86: Adt41;
let _87: *mut i16;
let _88: char;
let _89: &'static mut &'static usize;
let _90: Adt50;
let _91: *const &'static mut &'static f64;
let _92: &'static usize;
let _93: (*mut &'static f64, &'static mut isize);
let _94: usize;
let _95: f64;
let _96: &'static mut *mut char;
let _97: i16;
let _98: &'static i32;
let _99: i32;
let _100: &'static f64;
let _101: *const i16;
let _102: [bool; 2];
let _103: &'static mut *const i16;
let _104: [i64; 3];
let _105: Adt44;
let _106: &'static &'static mut u8;
let _107: &'static mut &'static f64;
let _108: char;
let _109: f64;
let _110: &'static u8;
let _111: Adt44;
let _112: bool;
let _113: i128;
let _114: f32;
let _115: &'static i8;
let _116: *mut &'static f64;
let _117: &'static i32;
let _118: f64;
let _119: isize;
let _120: f32;
let _121: ();
let _122: ();
{
_5 = _6;
_8 = _6;
_3 = true;
_4 = core::ptr::addr_of_mut!(_12);
_3 = true;
(*_4) = _10;
(*_4) = _10;
_7 = _1;
(*_4) = _8;
Call(RET = fn12(_10, Move(_4), (*_4), (*_4)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _10;
_6 = _5;
_12 = _6;
_9 = _10;
_9 = _10;
_9 = RET;
_9 = _10;
_14 = 10_u8 as f64;
_11 = 24_u8 as f32;
_6 = RET;
_4 = core::ptr::addr_of_mut!(_8);
_9 = _5;
Goto(bb2)
}
bb2 = {
_12 = (*_4);
(*_4) = _6;
_12 = (*_4);
(*_4) = _12;
_12 = (*_4);
(*_4) = _10;
(*_4) = _9;
_14 = 304494331184664791195166030728799240126_u128 as f64;
_12 = (*_4);
(*_4) = _9;
(*_4) = _12;
(*_4) = _12;
(*_4) = _9;
_2 = _1 ^ _7;
_3 = (*_4) == _9;
(*_4) = _6;
(*_4) = _10;
Goto(bb3)
}
bb3 = {
_5 = (*_4);
_11 = 218461571243155750836388973310970682750_u128 as f32;
_7 = _14 as isize;
(*_4) = _12;
(*_4) = _6;
_15.2.0 = Adt23::Variant0 { fld0: 2644999162_u32,fld1: _14,fld2: _7 };
_6 = (*_4);
_10 = RET;
_7 = _1;
(*_4) = _9;
_15.0 = !32647_u16;
(*_4) = _9;
_10 = _5;
Goto(bb4)
}
bb4 = {
_17 = _3;
_6 = (*_4);
_15.2.1 = !74438877047899955335439230249178276191_u128;
_2 = Field::<isize>(Variant(_15.2.0, 0), 2) >> Field::<isize>(Variant(_15.2.0, 0), 2);
(*_4) = _12;
(*_4) = _6;
place!(Field::<u32>(Variant(_15.2.0, 0), 0)) = !737024970_u32;
(*_4) = RET;
_15.0 = 3141_u16 << _15.2.1;
_10 = (*_4);
_8 = _9;
place!(Field::<f64>(Variant(_15.2.0, 0), 1)) = _15.0 as f64;
place!(Field::<f64>(Variant(_15.2.0, 0), 1)) = -_14;
_19.2 = [(-1164244169_i32)];
_9 = RET;
_8 = _10;
_19.0 = _15.2.1;
(*_4) = _10;
_4 = core::ptr::addr_of_mut!((*_4));
RET = (*_4);
_15.2.0 = Adt23::Variant0 { fld0: 1158128327_u32,fld1: _14,fld2: _2 };
(*_4) = _9;
(*_4) = _12;
(*_4) = _5;
_15.2.1 = _19.0;
(*_4) = _12;
_20 = _11 * _11;
Goto(bb5)
}
bb5 = {
_15.2.0 = Adt23::Variant2 { fld0: _15.0,fld1: 126799496613711845161169719274243449791_i128 };
(*_4) = _12;
(*_4) = _6;
place!(Field::<u16>(Variant(_15.2.0, 2), 0)) = _15.0;
(*_4) = RET;
(*_4) = _6;
_22 = _2;
RET = (*_4);
_2 = 7_usize as isize;
(*_4) = RET;
_11 = _20 * _20;
_15.2.0 = Adt23::Variant2 { fld0: _15.0,fld1: 154408312143249871917095321648311321344_i128 };
_24 = 92063322675244917537367926563385020642_i128 | 150954998157326124471193127798693980396_i128;
_19.2 = [118720885_i32];
(*_4) = _5;
_15.2.1 = _19.0;
_21.0 = core::ptr::addr_of_mut!(_21.3);
_22 = 23773_i16 as isize;
_20 = _11 * _11;
(*_4) = _5;
Goto(bb6)
}
bb6 = {
(*_4) = _5;
(*_4) = _5;
_23 = _3 as isize;
_21.1 = -406846423_i32;
(*_4) = _9;
_19.1 = 6_usize << _24;
Goto(bb7)
}
bb7 = {
_24 = 2414623402267404299_i64 as i128;
(*_4) = RET;
_8 = _9;
(*_4) = _10;
_19.0 = _15.2.1 >> Field::<u16>(Variant(_15.2.0, 2), 0);
_2 = _23 << _22;
_21.1 = 600530548_i32 >> _19.0;
(*_4) = RET;
_24 = !21173399195974417198217206458026037265_i128;
_15.2.1 = _19.0 + _19.0;
_11 = 65_i8 as f32;
place!(Field::<i128>(Variant(_15.2.0, 2), 1)) = _24;
(*_4) = _6;
_26 = core::ptr::addr_of!(_19.0);
(*_4) = RET;
(*_4) = RET;
(*_26) = _15.2.1 | _15.2.1;
_31 = _7 << _2;
_9 = (*_4);
_1 = _31;
(*_26) = !_15.2.1;
_7 = 4256729409_u32 as isize;
_30 = [(-7784043319410979551_i64),5811584050154053254_i64,6730369942756843764_i64];
Goto(bb8)
}
bb8 = {
(*_4) = _12;
_12 = (*_4);
_28 = !_19.1;
(*_26) = _15.2.1;
_4 = core::ptr::addr_of_mut!((*_4));
Goto(bb9)
}
bb9 = {
_10 = (*_4);
(*_4) = _10;
_7 = _2 - _22;
_34 = _14;
(*_4) = _9;
_31 = _2;
_11 = _20;
_2 = _23;
Goto(bb10)
}
bb10 = {
(*_4) = _12;
(*_4) = _10;
(*_26) = _15.2.1;
_19.2 = [_21.1];
(*_26) = _15.2.1 - _15.2.1;
(*_4) = _10;
(*_26) = _15.2.1;
_19.1 = _28;
_5 = (*_4);
Goto(bb11)
}
bb11 = {
_21.0 = core::ptr::addr_of_mut!(_21.3);
(*_4) = _9;
(*_4) = _9;
_36 = (*_4);
(*_4) = _36;
_9 = (*_4);
_38 = -_20;
_37 = -_14;
_15.2.1 = !(*_26);
_12 = _8;
_7 = _37 as isize;
_28 = _19.1;
(*_4) = _5;
_31 = _1;
_17 = (*_4) <= (*_4);
(*_26) = _15.2.1;
(*_4) = _10;
_34 = Field::<u16>(Variant(_15.2.0, 2), 0) as f64;
_35.0 = &_14;
(*_4) = _6;
_25 = 332543656_u32 as f32;
(*_26) = _15.2.1 << _23;
(*_4) = _5;
_41 = [106_i8,12_i8,(-31_i8),121_i8,75_i8,10_i8,14_i8,60_i8];
Goto(bb12)
}
bb12 = {
(*_4) = _10;
(*_4) = _12;
_21.1 = -30219342_i32;
_8 = _9;
_25 = -_20;
_5 = _12;
_19.1 = _28 ^ _28;
_42.fld3 = core::ptr::addr_of_mut!(_42.fld1);
_19.2 = [_21.1];
_44 = Field::<i128>(Variant(_15.2.0, 2), 1) as i32;
Goto(bb13)
}
bb13 = {
(*_26) = _15.2.1 << _23;
(*_26) = _15.2.1;
(*_26) = _15.2.1 * _15.2.1;
(*_4) = _9;
(*_26) = !_15.2.1;
(*_26) = _15.2.1 | _15.2.1;
(*_26) = _15.2.1;
_11 = _20 * _20;
(*_4) = _36;
(*_26) = _15.2.1;
_19.2 = [_44];
_46 = _31 << (*_26);
(*_26) = _15.2.1;
(*_4) = RET;
(*_4) = _9;
(*_4) = _10;
_42.fld0 = !_15.0;
_34 = _14 + _14;
_35.1 = [1158901492_u32,4001161713_u32,6274745_u32,1538167814_u32,2387600143_u32,2628606621_u32,1260621147_u32];
(*_4) = _10;
(*_4) = _10;
_15.2.0 = Adt23::Variant2 { fld0: _42.fld0,fld1: _24 };
_42.fld3 = core::ptr::addr_of_mut!(_42.fld1);
Goto(bb14)
}
bb14 = {
(*_4) = _12;
_43 = _20 - _11;
_17 = _3;
_52.2 = (_15.2.0, (*_26));
_12 = (*_4);
_19.1 = _28 | _28;
(*_26) = _15.2.1 << _15.2.1;
(*_26) = _52.2.1;
place!(Field::<u16>(Variant(_15.2.0, 2), 0)) = !Field::<u16>(Variant(_52.2.0, 2), 0);
(*_26) = _15.2.1 & _52.2.1;
(*_4) = RET;
_28 = _19.1 << _1;
(*_26) = 163_u8 as u128;
place!(Field::<i128>(Variant(_15.2.0, 2), 1)) = _42.fld0 as i128;
(*_4) = RET;
place!(Field::<u16>(Variant(_15.2.0, 2), 0)) = _15.0 * Field::<u16>(Variant(_52.2.0, 2), 0);
(*_4) = RET;
_51 = 80_u8 + 240_u8;
_32 = &mut _51;
(*_32) = 165_u8 << _15.2.1;
(*_4) = _36;
(*_4) = _9;
_42.fld3 = core::ptr::addr_of_mut!(_42.fld1);
_2 = _46;
(*_4) = _36;
Goto(bb15)
}
bb15 = {
(*_4) = RET;
_47 = &_38;
(*_26) = _15.2.1 * _52.2.1;
_40 = core::ptr::addr_of!(_32);
(*_26) = _15.2.1;
Goto(bb16)
}
bb16 = {
(*_26) = _52.2.1;
_45 = _21.1 & _21.1;
_46 = Field::<i128>(Variant(_52.2.0, 2), 1) as isize;
(*_4) = RET;
_19.0 = _15.2.1 | _52.2.1;
_57 = core::ptr::addr_of_mut!(_21.3);
(*_4) = _9;
(*_26) = _19.1 as u128;
Goto(bb17)
}
bb17 = {
_46 = !_1;
(*_26) = _15.2.1 + _15.2.1;
RET = (*_4);
_52.2.0 = Adt23::Variant2 { fld0: _42.fld0,fld1: _24 };
place!(Field::<i128>(Variant(_52.2.0, 2), 1)) = 2432252593269765156_i64 as i128;
_2 = _31;
RET = (*_4);
(*_32) = _3 as u8;
(*_32) = !218_u8;
place!(Field::<i128>(Variant(_52.2.0, 2), 1)) = -_24;
(*_26) = _15.2.1 & _15.2.1;
_15.3 = core::ptr::addr_of_mut!(_35.0);
Goto(bb18)
}
bb18 = {
_6 = RET;
_48 = (*_4);
_15.0 = Field::<u16>(Variant(_52.2.0, 2), 0) - _42.fld0;
(*_32) = 239_u8;
(*_4) = _10;
_4 = core::ptr::addr_of_mut!((*_4));
_7 = Field::<u16>(Variant(_15.2.0, 2), 0) as isize;
_22 = _31 & _2;
(*_32) = 34_u8 * 18_u8;
_35.0 = &_34;
_55 = core::ptr::addr_of!(_15.2.1);
(*_4) = _48;
_52.0 = !Field::<u16>(Variant(_15.2.0, 2), 0);
_10 = _12;
_43 = (*_47);
(*_4) = _12;
(*_55) = (*_26) ^ (*_26);
(*_4) = _6;
Goto(bb19)
}
bb19 = {
_9 = (*_4);
(*_55) = (*_26) | (*_26);
(*_26) = !(*_55);
(*_4) = _36;
(*_55) = (*_26) + (*_26);
_39 = &_19.1;
(*_26) = (*_55) ^ (*_55);
(*_26) = !(*_55);
Goto(bb20)
}
bb20 = {
(*_4) = _12;
(*_4) = _12;
_47 = &_11;
_57 = core::ptr::addr_of_mut!((*_57));
_42.fld2 = (*_47) * (*_47);
(*_55) = !_19.0;
(*_55) = (*_26) + (*_26);
_19.0 = (*_55) - (*_55);
(*_26) = (*_55);
_45 = -_44;
_36 = (*_4);
_49 = -_14;
(*_32) = (*_39) as u8;
_58.1 = _35.1;
_65 = !_31;
_35.1 = [3087587382_u32,969547694_u32,2431220906_u32,2722683173_u32,62830448_u32,3070908157_u32,3400313703_u32];
_58.0 = &_14;
(*_4) = _12;
(*_32) = 243_u8;
_59 = (*_55) == (*_55);
(*_26) = (*_55) - (*_55);
_50 = &_44;
_15.2.1 = !(*_26);
place!(Field::<i128>(Variant(_52.2.0, 2), 1)) = _24 >> (*_55);
_15.2 = (_52.2.0, (*_26));
_13 = Move(_40);
Goto(bb21)
}
bb21 = {
_67 = _59 & _59;
(*_4) = _5;
_69 = !_67;
(*_55) = (*_26) | _19.0;
(*_26) = (*_55) | (*_55);
_68 = 4228_i16 as isize;
(*_4) = RET;
(*_4) = _6;
_8 = RET;
(*_55) = (*_26) << (*_26);
RET = (*_4);
Goto(bb22)
}
bb22 = {
_3 = _59;
_38 = 4109656458499897229_i64 as f32;
(*_55) = (*_26);
_13 = core::ptr::addr_of!(_32);
_13 = core::ptr::addr_of!(_32);
_2 = !_22;
_31 = _46 << (*_26);
Call(_37 = core::intrinsics::fmaf64(_49, _14, _49), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_35.0 = &_14;
(*_32) = 70_u8 >> (*_55);
_26 = core::ptr::addr_of!((*_55));
_27 = core::ptr::addr_of_mut!(_58.1);
(*_4) = _10;
(*_26) = _19.0 >> _46;
_57 = core::ptr::addr_of_mut!((*_57));
(*_55) = _49 as u128;
_37 = _34;
(*_32) = 208_u8 + 61_u8;
(*_4) = _48;
RET = _9;
_62 = 4167393265_u32;
match _62 {
0 => bb1,
1 => bb17,
2 => bb24,
3 => bb25,
4 => bb26,
4167393265 => bb28,
_ => bb27
}
}
bb24 = {
(*_4) = _12;
(*_4) = _10;
(*_26) = _15.2.1;
_19.2 = [_21.1];
(*_26) = _15.2.1 - _15.2.1;
(*_4) = _10;
(*_26) = _15.2.1;
_19.1 = _28;
_5 = (*_4);
Goto(bb11)
}
bb25 = {
(*_26) = _52.2.1;
_45 = _21.1 & _21.1;
_46 = Field::<i128>(Variant(_52.2.0, 2), 1) as isize;
(*_4) = RET;
_19.0 = _15.2.1 | _52.2.1;
_57 = core::ptr::addr_of_mut!(_21.3);
(*_4) = _9;
(*_26) = _19.1 as u128;
Goto(bb17)
}
bb26 = {
_8 = _10;
_6 = _5;
_12 = _6;
_9 = _10;
_9 = _10;
_9 = RET;
_9 = _10;
_14 = 10_u8 as f64;
_11 = 24_u8 as f32;
_6 = RET;
_4 = core::ptr::addr_of_mut!(_8);
_9 = _5;
Goto(bb2)
}
bb27 = {
_15.2.0 = Adt23::Variant2 { fld0: _15.0,fld1: 126799496613711845161169719274243449791_i128 };
(*_4) = _12;
(*_4) = _6;
place!(Field::<u16>(Variant(_15.2.0, 2), 0)) = _15.0;
(*_4) = RET;
(*_4) = _6;
_22 = _2;
RET = (*_4);
_2 = 7_usize as isize;
(*_4) = RET;
_11 = _20 * _20;
_15.2.0 = Adt23::Variant2 { fld0: _15.0,fld1: 154408312143249871917095321648311321344_i128 };
_24 = 92063322675244917537367926563385020642_i128 | 150954998157326124471193127798693980396_i128;
_19.2 = [118720885_i32];
(*_4) = _5;
_15.2.1 = _19.0;
_21.0 = core::ptr::addr_of_mut!(_21.3);
_22 = 23773_i16 as isize;
_20 = _11 * _11;
(*_4) = _5;
Goto(bb6)
}
bb28 = {
_46 = -_1;
_19.2 = [(*_50)];
(*_55) = _19.0;
(*_27) = _35.1;
place!(Field::<u16>(Variant(_52.2.0, 2), 0)) = _42.fld0 - _15.0;
Goto(bb29)
}
bb29 = {
_41 = [75_i8,99_i8,(-51_i8),95_i8,14_i8,(-124_i8),(-84_i8),(-113_i8)];
_12 = (*_4);
_42.fld1 = !_62;
_37 = _49 + _34;
_15.0 = Field::<u16>(Variant(_52.2.0, 2), 0) ^ _52.0;
_64 = _37 * _34;
_11 = _43 + _25;
(*_27) = _35.1;
(*_4) = RET;
(*_32) = 162_u8 << (*_55);
_42.fld4 = (-1269082384671388160_i64) as usize;
_36 = (*_4);
(*_27) = [_42.fld1,_62,_42.fld1,_42.fld1,_42.fld1,_42.fld1,_42.fld1];
(*_4) = _5;
_52.2.1 = (*_55);
_78.3 = (*_4);
(*_27) = [_62,_42.fld1,_62,_62,_62,_42.fld1,_42.fld1];
(*_32) = !255_u8;
_76 = _42.fld2;
_78.3 = (*_4);
(*_32) = 21_u8 >> (*_39);
RET = (*_4);
_62 = _42.fld1 - _42.fld1;
(*_55) = _52.2.1;
(*_4) = _10;
(*_32) = 71_u8 + 78_u8;
_9 = (*_4);
(*_27) = [_42.fld1,_62,_62,_62,_62,_62,_62];
Goto(bb30)
}
bb30 = {
(*_55) = _52.2.1 - _19.0;
(*_55) = _42.fld2 as u128;
_80 = _37 * _37;
(*_55) = _52.2.1;
(*_27) = _35.1;
_78.1 = [(*_32),(*_32)];
_15.2.1 = !_19.0;
_6 = (*_4);
_48 = _10;
(*_4) = _48;
_79 = Field::<i128>(Variant(_52.2.0, 2), 1) as f64;
(*_27) = [_62,_42.fld1,_42.fld1,_62,_62,_62,_62];
(*_55) = (*_4) as u128;
(*_32) = Field::<i128>(Variant(_52.2.0, 2), 1) as u8;
_21.1 = -_45;
_52.2 = (_15.2.0, _19.0);
_66 = Move((*_13));
(*_55) = !_19.0;
_73 = _79;
place!(Field::<u16>(Variant(_52.2.0, 2), 0)) = !_42.fld0;
Goto(bb31)
}
bb31 = {
place!(Field::<u16>(Variant(_15.2.0, 2), 0)) = _15.0 ^ _52.0;
(*_27) = [_62,_42.fld1,_42.fld1,_62,_62,_42.fld1,_62];
_42.fld1 = !_62;
_52.2.1 = (*_39) as u128;
_35.1 = [_62,_62,_62,_62,_62,_42.fld1,_62];
_19.0 = (*_55) ^ (*_55);
_5 = _12;
Goto(bb32)
}
bb32 = {
_49 = (-30635_i16) as f64;
(*_55) = _19.0;
_70 = -_31;
(*_4) = _10;
Call(_46 = core::intrinsics::transmute((*_39)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_27 = core::ptr::addr_of_mut!((*_27));
(*_4) = _9;
_74 = (*_50);
(*_27) = [_62,_62,_42.fld1,_42.fld1,_62,_42.fld1,_42.fld1];
_81 = core::ptr::addr_of_mut!(_52.3);
_3 = _59 ^ _69;
(*_81) = core::ptr::addr_of_mut!(_58.0);
_7 = _31;
_82 = Move(_27);
_57 = core::ptr::addr_of_mut!((*_57));
_82 = core::ptr::addr_of_mut!(_35.1);
(*_82) = _58.1;
_43 = _20 * _42.fld2;
_26 = core::ptr::addr_of!((*_55));
(*_82) = [_62,_42.fld1,_42.fld1,_42.fld1,_62,_42.fld1,_42.fld1];
(*_26) = _19.0;
_65 = (*_39) as isize;
_78.3 = _9;
_31 = _70 * _7;
(*_81) = core::ptr::addr_of_mut!(_58.0);
_83 = -_7;
(*_81) = core::ptr::addr_of_mut!(_35.0);
_52.2.0 = Adt23::Variant0 { fld0: _42.fld1,fld1: _73,fld2: _7 };
_48 = (*_4);
Goto(bb34)
}
bb34 = {
_68 = _7 >> _83;
_36 = (*_4);
(*_26) = (*_4) as u128;
(*_81) = core::ptr::addr_of_mut!(_58.0);
_21.0 = core::ptr::addr_of_mut!((*_57));
_68 = !_23;
(*_4) = _78.3;
(*_4) = _78.3;
(*_82) = [_62,_62,Field::<u32>(Variant(_52.2.0, 0), 0),_42.fld1,Field::<u32>(Variant(_52.2.0, 0), 0),_42.fld1,Field::<u32>(Variant(_52.2.0, 0), 0)];
(*_4) = _10;
(*_81) = core::ptr::addr_of_mut!(_35.0);
_17 = _59 < _67;
_86 = Adt41::Variant1 { fld0: _3,fld1: 13582100588019845640_u64,fld2: _79,fld3: 36_i8,fld4: Field::<u32>(Variant(_52.2.0, 0), 0) };
_82 = core::ptr::addr_of_mut!((*_82));
(*_81) = core::ptr::addr_of_mut!(_58.0);
Goto(bb35)
}
bb35 = {
(*_26) = _19.0 << _2;
(*_55) = _19.0 | _19.0;
(*_4) = _9;
_14 = _73;
_33 = _11 * _43;
_82 = core::ptr::addr_of_mut!((*_82));
_15.2.1 = _33 as u128;
_52.2 = (_15.2.0, (*_55));
(*_81) = core::ptr::addr_of_mut!(_58.0);
_15.3 = core::ptr::addr_of_mut!(_58.0);
Goto(bb36)
}
bb36 = {
_35.0 = &place!(Field::<f64>(Variant(_86, 1), 2));
_70 = !_83;
(*_4) = _5;
_71 = (-22_i8) & 64_i8;
place!(Field::<f64>(Variant(_86, 1), 2)) = _79 - _79;
(*_81) = core::ptr::addr_of_mut!(_35.0);
_4 = core::ptr::addr_of_mut!((*_4));
_53 = (*_50) as u16;
(*_82) = [_62,_42.fld1,Field::<u32>(Variant(_86, 1), 4),_62,_62,Field::<u32>(Variant(_86, 1), 4),_62];
_24 = Field::<i128>(Variant(_15.2.0, 2), 1) - Field::<i128>(Variant(_15.2.0, 2), 1);
(*_4) = _5;
_43 = _11;
(*_4) = _12;
Goto(bb37)
}
bb37 = {
(*_82) = [Field::<u32>(Variant(_86, 1), 4),Field::<u32>(Variant(_86, 1), 4),_42.fld1,_42.fld1,_62,_42.fld1,Field::<u32>(Variant(_86, 1), 4)];
_90.fld1 = [(*_50)];
(*_4) = _6;
_40 = Move(_13);
_15.0 = _71 as u16;
(*_82) = [Field::<u32>(Variant(_86, 1), 4),_42.fld1,_42.fld1,_62,_62,_62,_42.fld1];
(*_57) = core::ptr::addr_of_mut!(_97);
(*_81) = core::ptr::addr_of_mut!(_58.0);
_85 = Field::<i128>(Variant(_52.2.0, 2), 1) & Field::<i128>(Variant(_15.2.0, 2), 1);
(*_55) = !_19.0;
(*_55) = !_52.2.1;
_19 = ((*_55), _28, _90.fld1);
(*_57) = core::ptr::addr_of_mut!(_97);
_8 = _6;
Goto(bb38)
}
bb38 = {
_89 = &mut _39;
(*_57) = core::ptr::addr_of_mut!(_97);
_81 = core::ptr::addr_of_mut!((*_81));
(*_82) = _58.1;
(*_55) = _52.2.1 & _52.2.1;
_15.1 = &mut _71;
_47 = &_43;
(*_4) = _5;
(*_4) = _78.3;
place!(Field::<f64>(Variant(_86, 1), 2)) = _79;
_97 = (*_50) as i16;
(*_89) = &_42.fld4;
(*_4) = _6;
_84 = [(*_55),(*_55)];
(*_4) = _10;
_45 = (*_47) as i32;
Goto(bb39)
}
bb39 = {
(*_55) = _19.0;
_15.3 = Move((*_81));
(*_55) = 897530123441805544_i64 as u128;
_41 = [(-43_i8),(-28_i8),103_i8,49_i8,(-110_i8),(-31_i8),86_i8,(-116_i8)];
(*_55) = _52.2.1 & _52.2.1;
_70 = Field::<i128>(Variant(_15.2.0, 2), 1) as isize;
(*_81) = core::ptr::addr_of_mut!(_58.0);
_64 = Field::<f64>(Variant(_86, 1), 2);
(*_89) = &_19.1;
_42.fld2 = Field::<f64>(Variant(_86, 1), 2) as f32;
_79 = _64 + _73;
_15.2.0 = Adt23::Variant1 { fld0: _83 };
(*_81) = core::ptr::addr_of_mut!(_58.0);
_24 = _85;
_78.2 = &mut _1;
_70 = !_83;
place!(Field::<u64>(Variant(_86, 1), 1)) = 16655370264684517747_u64 | 1275078481391727223_u64;
(*_81) = Move(_15.3);
_10 = (*_4);
Call((*_55) = core::intrinsics::bswap(_52.2.1), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_21.3 = core::ptr::addr_of_mut!(_97);
(*_89) = &_28;
_95 = _14;
(*_81) = core::ptr::addr_of_mut!(_35.0);
place!(Field::<bool>(Variant(_86, 1), 0)) = _3 & _59;
_42.fld1 = !_62;
_45 = _21.1 << _31;
(*_82) = [_42.fld1,_62,Field::<u32>(Variant(_86, 1), 4),_42.fld1,_62,_42.fld1,Field::<u32>(Variant(_86, 1), 4)];
_22 = _31 ^ _46;
(*_4) = _12;
_96 = &mut _4;
place!(Field::<bool>(Variant(_86, 1), 0)) = Field::<isize>(Variant(_15.2.0, 1), 0) < _46;
(*_89) = &_42.fld4;
_43 = _76;
_11 = Field::<u32>(Variant(_86, 1), 4) as f32;
(*_55) = _6 as u128;
(*_55) = _45 as u128;
Goto(bb41)
}
bb41 = {
(*_96) = core::ptr::addr_of_mut!(_6);
_53 = 7955018559345926628_i64 as u16;
_83 = (*_55) as isize;
(*_89) = &_19.1;
_6 = _9;
_6 = _9;
Goto(bb42)
}
bb42 = {
(*_89) = &_28;
(*_82) = _58.1;
_52.1 = Move(_15.1);
Goto(bb43)
}
bb43 = {
_57 = core::ptr::addr_of_mut!((*_57));
_18.0 = core::ptr::addr_of!(_97);
(*_89) = &_19.1;
_53 = !Field::<u16>(Variant(_52.2.0, 2), 0);
_60 = -59_i8;
(*_89) = &_42.fld4;
_61 = &mut _60;
(*_81) = core::ptr::addr_of_mut!(_35.0);
_15.2.0 = Adt23::Variant1 { fld0: _7 };
_7 = _70;
_56 = &mut _18.0;
(*_89) = &_19.1;
(*_89) = &_42.fld4;
(*_89) = &_19.1;
(*_56) = core::ptr::addr_of!(_97);
_36 = _6;
(*_89) = &_42.fld4;
(*_81) = core::ptr::addr_of_mut!(_58.0);
_75 = _70 ^ Field::<isize>(Variant(_15.2.0, 1), 0);
(*_81) = core::ptr::addr_of_mut!(_35.0);
_44 = _45;
_5 = _6;
(*_89) = &_28;
(*_55) = _19.0 | _19.0;
(*_56) = core::ptr::addr_of!(_97);
Goto(bb44)
}
bb44 = {
(*_61) = !(-54_i8);
(*_55) = _19.0 & _19.0;
_27 = core::ptr::addr_of_mut!((*_82));
(*_27) = _58.1;
(*_96) = core::ptr::addr_of_mut!(_10);
_58.1 = [Field::<u32>(Variant(_86, 1), 4),_42.fld1,_62,Field::<u32>(Variant(_86, 1), 4),_42.fld1,_42.fld1,Field::<u32>(Variant(_86, 1), 4)];
_78.3 = _8;
(*_56) = core::ptr::addr_of!(_97);
_99 = _44;
_63 = _70 * Field::<isize>(Variant(_15.2.0, 1), 0);
_58 = (Move(_35.0), (*_27));
(*_82) = [_62,Field::<u32>(Variant(_86, 1), 4),_42.fld1,Field::<u32>(Variant(_86, 1), 4),_62,_42.fld1,_62];
(*_61) = (-16_i8);
(*_82) = [Field::<u32>(Variant(_86, 1), 4),_42.fld1,Field::<u32>(Variant(_86, 1), 4),Field::<u32>(Variant(_86, 1), 4),Field::<u32>(Variant(_86, 1), 4),_42.fld1,_62];
match (*_61) {
340282366920938463463374607431768211440 => bb46,
_ => bb45
}
}
bb45 = {
(*_4) = _12;
_12 = (*_4);
_28 = !_19.1;
(*_26) = _15.2.1;
_4 = core::ptr::addr_of_mut!((*_4));
Goto(bb9)
}
bb46 = {
(*_89) = &_19.1;
_93.1 = Move(_78.2);
_54 = [_28,_28,_28,_28,_28,_19.1,_28];
(*_55) = _52.2.1 << _99;
_70 = Field::<isize>(Variant(_15.2.0, 1), 0) * _63;
(*_89) = &_28;
_112 = _67 ^ _69;
_109 = _64 - Field::<f64>(Variant(_86, 1), 2);
(*_57) = core::ptr::addr_of_mut!(_97);
(*_96) = core::ptr::addr_of_mut!(_36);
_101 = Move((*_56));
_99 = -_44;
(*_57) = core::ptr::addr_of_mut!(_97);
_21.1 = _69 as i32;
match (*_61) {
0 => bb40,
1 => bb19,
2 => bb3,
3 => bb14,
340282366920938463463374607431768211440 => bb48,
_ => bb47
}
}
bb47 = {
_35.0 = &_14;
(*_32) = 70_u8 >> (*_55);
_26 = core::ptr::addr_of!((*_55));
_27 = core::ptr::addr_of_mut!(_58.1);
(*_4) = _10;
(*_26) = _19.0 >> _46;
_57 = core::ptr::addr_of_mut!((*_57));
(*_55) = _49 as u128;
_37 = _34;
(*_32) = 208_u8 + 61_u8;
(*_4) = _48;
RET = _9;
_62 = 4167393265_u32;
match _62 {
0 => bb1,
1 => bb17,
2 => bb24,
3 => bb25,
4 => bb26,
4167393265 => bb28,
_ => bb27
}
}
bb48 = {
(*_89) = &_42.fld4;
(*_55) = !_19.0;
(*_96) = core::ptr::addr_of_mut!(_9);
_45 = _48 as i32;
_104 = [(-1604860911646767312_i64),(-8392993817487450808_i64),(-720840644463237738_i64)];
(*_61) = 114_i8;
_44 = _99 ^ _99;
_31 = _97 as isize;
place!(Field::<f64>(Variant(_86, 1), 2)) = -_73;
_94 = _28 ^ _19.1;
_42.fld1 = _62 - Field::<u32>(Variant(_86, 1), 4);
(*_55) = _52.2.1 & _19.0;
place!(Field::<isize>(Variant(_15.2.0, 1), 0)) = _97 as isize;
(*_89) = &_19.1;
(*_57) = core::ptr::addr_of_mut!(_97);
(*_55) = _19.0;
Goto(bb49)
}
bb49 = {
_36 = _12;
_47 = &_33;
(*_57) = core::ptr::addr_of_mut!(_97);
place!(Field::<f64>(Variant(_86, 1), 2)) = _95;
(*_89) = &_94;
_76 = (*_47);
(*_61) = (-36_i8);
_70 = !_7;
_8 = _78.3;
_35.0 = &_95;
_93.0 = core::ptr::addr_of_mut!(_35.0);
(*_57) = core::ptr::addr_of_mut!(_97);
(*_61) = 35_i8 ^ 45_i8;
(*_96) = core::ptr::addr_of_mut!(_6);
_38 = _76 + (*_47);
_87 = Move((*_57));
_15.0 = Field::<u16>(Variant(_52.2.0, 2), 0) << _85;
(*_57) = core::ptr::addr_of_mut!(_97);
(*_55) = !_52.2.1;
(*_96) = core::ptr::addr_of_mut!(_6);
(*_57) = core::ptr::addr_of_mut!(_97);
_78.1 = [255_u8,253_u8];
(*_82) = _58.1;
(*_55) = _112 as u128;
_78.2 = &mut _75;
_115 = &(*_61);
place!(Field::<i128>(Variant(_52.2.0, 2), 1)) = _85 | _85;
Goto(bb50)
}
bb50 = {
Call(_121 = dump_var(Move(_23), Move(_67), Move(_41), Move(_63)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_121 = dump_var(Move(_36), Move(_74), Move(_12), Move(_59)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_121 = dump_var(Move(_45), Move(_99), Move(_5), Move(_7)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_121 = dump_var(Move(_53), Move(_44), Move(_60), Move(_8)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_121 = dump_var(Move(_17), Move(_71), Move(_62), Move(_83)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_121 = dump_var(Move(_6), Move(_69), Move(_31), _122), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: char,mut _2: *mut char,mut _3: char,mut _4: char) -> char {
mir! {
type RET = char;
let _5: Adt75;
let _6: usize;
let _7: (&'static f64, [u32; 7]);
let _8: *mut i16;
let _9: &'static mut *mut char;
let _10: [char; 4];
let _11: f32;
let _12: i64;
let _13: [u8; 7];
let _14: ([char; 4], [u8; 2], &'static mut isize, char);
let _15: [i32; 1];
let _16: isize;
let _17: u64;
let _18: *mut ([char; 4], u8, [char; 4]);
let _19: *mut char;
let _20: i16;
let _21: *const &'static mut &'static f64;
let _22: *const &'static mut &'static f64;
let _23: u128;
let _24: u8;
let _25: isize;
let _26: [u32; 7];
let _27: bool;
let _28: u128;
let _29: isize;
let _30: [u64; 7];
let _31: &'static mut isize;
let _32: isize;
let _33: char;
let _34: *const u128;
let _35: f64;
let _36: &'static i8;
let _37: [i8; 8];
let _38: char;
let _39: Adt43;
let _40: [i64; 3];
let _41: u64;
let _42: &'static mut &'static f64;
let _43: i16;
let _44: *mut *mut &'static f64;
let _45: char;
let _46: u32;
let _47: f64;
let _48: u16;
let _49: f32;
let _50: [u8; 7];
let _51: Adt44;
let _52: f32;
let _53: &'static mut *mut char;
let _54: char;
let _55: u64;
let _56: &'static i8;
let _57: *const i16;
let _58: *mut u32;
let _59: [bool; 2];
let _60: i32;
let _61: bool;
let _62: &'static mut *const i16;
let _63: *mut i16;
let _64: isize;
let _65: &'static mut *mut char;
let _66: &'static mut &'static f64;
let _67: f64;
let _68: isize;
let _69: [u32; 7];
let _70: *mut *mut i16;
let _71: *const &'static mut u8;
let _72: &'static mut u8;
let _73: (Adt23, u128);
let _74: &'static u8;
let _75: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _76: bool;
let _77: u16;
let _78: *const &'static mut &'static f64;
let _79: &'static f64;
let _80: char;
let _81: &'static i8;
let _82: *mut &'static mut u8;
let _83: ();
let _84: ();
{
RET = _1;
Goto(bb1)
}
bb1 = {
_3 = _4;
_4 = _1;
_3 = RET;
RET = _3;
RET = _4;
_2 = core::ptr::addr_of_mut!(RET);
(*_2) = _3;
(*_2) = _1;
_4 = (*_2);
_1 = (*_2);
(*_2) = _4;
(*_2) = _1;
RET = _4;
(*_2) = _1;
(*_2) = _3;
_4 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _3;
(*_2) = _1;
(*_2) = _4;
(*_2) = _1;
Goto(bb2)
}
bb2 = {
(*_2) = _1;
Goto(bb3)
}
bb3 = {
(*_2) = _4;
RET = _4;
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
_7.1 = [3633857374_u32,2261709482_u32,4173049679_u32,927512049_u32,3919086351_u32,589154205_u32,3763536965_u32];
(*_2) = _1;
(*_2) = _1;
(*_2) = _1;
Goto(bb4)
}
bb4 = {
(*_2) = _1;
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
_6 = 1_usize << (-1291_i16);
(*_2) = _1;
(*_2) = _1;
(*_2) = _4;
(*_2) = _1;
_3 = (*_2);
(*_2) = _3;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
(*_2) = _4;
_4 = (*_2);
_10 = [(*_2),_3,(*_2),(*_2)];
_11 = 4765032234921359092_i64 as f32;
(*_2) = _3;
Goto(bb5)
}
bb5 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = core::ptr::addr_of_mut!(_4);
(*_2) = _1;
(*_2) = _1;
_4 = RET;
_1 = (*_2);
_14.1 = [223_u8,222_u8];
(*_2) = _3;
(*_2) = _3;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _3;
_14.3 = (*_2);
_9 = &mut _2;
_14.3 = _1;
_15 = [490158380_i32];
(*_9) = core::ptr::addr_of_mut!(_1);
_13 = [205_u8,208_u8,222_u8,85_u8,29_u8,201_u8,157_u8];
(*_9) = core::ptr::addr_of_mut!(_4);
_4 = _14.3;
_14.0 = _10;
Goto(bb7)
}
bb7 = {
_14.1 = [37_u8,234_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_3);
_3 = _14.3;
(*_9) = core::ptr::addr_of_mut!(_3);
_11 = (-7144317400790035960_i64) as f32;
_4 = _14.3;
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [148_u8,84_u8,28_u8,16_u8,238_u8,24_u8,62_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
Goto(bb8)
}
bb8 = {
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [204_u8,221_u8,35_u8,1_u8,200_u8,148_u8,250_u8];
(*_9) = core::ptr::addr_of_mut!(_1);
Goto(bb9)
}
bb9 = {
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_9) = core::ptr::addr_of_mut!(RET);
_17 = 4374446042167788607_u64;
_15 = [1257627876_i32];
(*_9) = core::ptr::addr_of_mut!(_4);
_6 = true as usize;
(*_9) = core::ptr::addr_of_mut!(RET);
_7.1 = [582240372_u32,1851813668_u32,2287089504_u32,1307914586_u32,2896443425_u32,4280024524_u32,352211468_u32];
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_4);
_14.1 = [143_u8,100_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
match _17 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
4374446042167788607 => bb11,
_ => bb10
}
}
bb10 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
(*_9) = core::ptr::addr_of_mut!(_1);
RET = _1;
_12 = 3577027911452989056_i64;
_19 = Move((*_9));
_3 = _1;
_20 = (-13122_i16);
RET = _3;
_3 = RET;
_8 = core::ptr::addr_of_mut!(_20);
(*_8) = -(-28626_i16);
_2 = core::ptr::addr_of_mut!(_3);
(*_8) = (-18076_i16);
(*_2) = _1;
_19 = core::ptr::addr_of_mut!((*_2));
(*_19) = RET;
(*_19) = _1;
_6 = (-47_i8) as usize;
RET = (*_19);
_14.0 = _10;
(*_8) = !29079_i16;
_1 = (*_2);
(*_2) = _14.3;
(*_2) = _1;
(*_2) = _14.3;
_23 = 292462837578287223937453422629572686104_u128 ^ 35038203241014700900929284886847875049_u128;
_7.1 = [3389566842_u32,955241809_u32,1370273753_u32,144894125_u32,1392155953_u32,4280871742_u32,2972709900_u32];
(*_2) = RET;
Goto(bb12)
}
bb12 = {
(*_8) = 8_u8 as i16;
(*_2) = _1;
(*_2) = RET;
(*_2) = _14.3;
(*_2) = RET;
_1 = (*_2);
_20 = 16037_i16;
(*_8) = 19660_i16 ^ (-10327_i16);
_7.1 = [3180947303_u32,3136718083_u32,3515151373_u32,1648349260_u32,3187326781_u32,3298923271_u32,3886238476_u32];
(*_8) = (-75_i16) | (-26545_i16);
_14.0 = _10;
_9 = &mut _19;
(*_9) = Move(_2);
(*_9) = core::ptr::addr_of_mut!(_14.3);
(*_8) = (-14331_i16) >> _6;
(*_9) = core::ptr::addr_of_mut!(_14.3);
_16 = (-9223372036854775808_isize) << (*_8);
_16 = _4 as isize;
(*_8) = 7648_i16 - (-17226_i16);
Goto(bb13)
}
bb13 = {
_12 = -7191005332132091127_i64;
(*_9) = core::ptr::addr_of_mut!(_3);
_14.0 = _10;
_14.1 = [162_u8,14_u8];
_23 = !166100137134321923739750840536453021221_u128;
(*_8) = 1422_i16;
(*_9) = core::ptr::addr_of_mut!(RET);
Goto(bb14)
}
bb14 = {
(*_8) = (-24552_i16) ^ (-19265_i16);
_27 = (*_8) == (*_8);
_14.3 = _4;
_28 = !_23;
_4 = _1;
(*_8) = (-25973_i16) >> _6;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = 150420372_u32 as i16;
_14.0 = [RET,_1,_1,_3];
(*_8) = -(-6782_i16);
(*_8) = (-1774514715_i32) as i16;
_25 = _16 + _16;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_14.3);
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = !4452_i16;
_2 = core::ptr::addr_of_mut!(RET);
match _17 {
0 => bb3,
1 => bb15,
4374446042167788607 => bb17,
_ => bb16
}
}
bb15 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb17 = {
(*_9) = core::ptr::addr_of_mut!((*_2));
(*_2) = _3;
_29 = (-10517490914071934770370691857609410225_i128) as isize;
(*_8) = !8481_i16;
_23 = !_28;
(*_2) = _3;
(*_8) = !4291_i16;
_11 = 4246635568_u32 as f32;
(*_2) = _4;
(*_2) = _14.3;
_13 = [124_u8,187_u8,210_u8,42_u8,65_u8,168_u8,160_u8];
_30 = [_17,_17,_17,_17,_17,_17,_17];
_15 = [1986693743_i32];
_13 = [112_u8,38_u8,108_u8,84_u8,149_u8,99_u8,2_u8];
_14.1 = [43_u8,224_u8];
(*_9) = Move(_2);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_8) = (-22119_i16);
_31 = &mut _29;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_3);
match (*_8) {
0 => bb15,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb12,
5 => bb16,
340282366920938463463374607431768189337 => bb19,
_ => bb18
}
}
bb18 = {
_14.1 = [37_u8,234_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_3);
_3 = _14.3;
(*_9) = core::ptr::addr_of_mut!(_3);
_11 = (-7144317400790035960_i64) as f32;
_4 = _14.3;
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [148_u8,84_u8,28_u8,16_u8,238_u8,24_u8,62_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
Goto(bb8)
}
bb19 = {
_13 = [82_u8,38_u8,94_u8,169_u8,122_u8,61_u8,33_u8];
(*_8) = (-147442031957103591597083021740612654191_i128) as i16;
_12 = 6246032833937427619_i64 - (-2805339755625877461_i64);
_24 = 38_u8 ^ 197_u8;
(*_9) = core::ptr::addr_of_mut!(RET);
(*_31) = _16 & _25;
_17 = 17790457830900563884_u64 >> (*_31);
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(_4);
_14.1 = [_24,_24];
(*_9) = core::ptr::addr_of_mut!(RET);
(*_31) = !_16;
_11 = 18104_u16 as f32;
_20 = 166000706_i32 as i16;
Goto(bb20)
}
bb20 = {
_28 = _23;
(*_8) = _24 as i16;
_33 = _1;
(*_8) = (-405500908_i32) as i16;
(*_8) = 11868_i16 >> (*_31);
(*_31) = _25 << (*_8);
_14.1 = [_24,_24];
_26 = _7.1;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_31) = !_25;
(*_8) = 19358_i16 ^ 4397_i16;
_34 = core::ptr::addr_of!(_23);
(*_9) = core::ptr::addr_of_mut!(_4);
_14.0 = [_33,_1,_33,RET];
_33 = _4;
_37 = [4_i8,(-29_i8),44_i8,(-79_i8),(-99_i8),125_i8,16_i8,25_i8];
_26 = [1737261810_u32,942211518_u32,1486034883_u32,2436115692_u32,3140342545_u32,102663699_u32,1132930663_u32];
_14.2 = &mut _16;
_23 = (-109212691168811769046897360812022161567_i128) as u128;
_40 = [_12,_12,_12];
_40 = [_12,_12,_12];
(*_31) = (*_34) as isize;
_14.1 = [_24,_24];
(*_8) = (-9337_i16) ^ 1052_i16;
Goto(bb21)
}
bb21 = {
(*_8) = -24048_i16;
(*_34) = !_28;
(*_31) = _25 | _25;
(*_8) = (-2588_i16);
_12 = (-60934852490926276367363649911042952010_i128) as i64;
(*_8) = 114_i8 as i16;
(*_8) = 29376_i16;
_10 = [_14.3,RET,_33,_4];
(*_34) = !_28;
_27 = (*_31) >= _25;
(*_9) = core::ptr::addr_of_mut!(_14.3);
_8 = core::ptr::addr_of_mut!(_20);
_15 = [595366523_i32];
match (*_8) {
0 => bb7,
1 => bb8,
2 => bb20,
3 => bb22,
4 => bb23,
5 => bb24,
6 => bb25,
29376 => bb27,
_ => bb26
}
}
bb22 = {
(*_8) = (-24552_i16) ^ (-19265_i16);
_27 = (*_8) == (*_8);
_14.3 = _4;
_28 = !_23;
_4 = _1;
(*_8) = (-25973_i16) >> _6;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = 150420372_u32 as i16;
_14.0 = [RET,_1,_1,_3];
(*_8) = -(-6782_i16);
(*_8) = (-1774514715_i32) as i16;
_25 = _16 + _16;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_14.3);
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = !4452_i16;
_2 = core::ptr::addr_of_mut!(RET);
match _17 {
0 => bb3,
1 => bb15,
4374446042167788607 => bb17,
_ => bb16
}
}
bb23 = {
(*_9) = core::ptr::addr_of_mut!(_1);
RET = _1;
_12 = 3577027911452989056_i64;
_19 = Move((*_9));
_3 = _1;
_20 = (-13122_i16);
RET = _3;
_3 = RET;
_8 = core::ptr::addr_of_mut!(_20);
(*_8) = -(-28626_i16);
_2 = core::ptr::addr_of_mut!(_3);
(*_8) = (-18076_i16);
(*_2) = _1;
_19 = core::ptr::addr_of_mut!((*_2));
(*_19) = RET;
(*_19) = _1;
_6 = (-47_i8) as usize;
RET = (*_19);
_14.0 = _10;
(*_8) = !29079_i16;
_1 = (*_2);
(*_2) = _14.3;
(*_2) = _1;
(*_2) = _14.3;
_23 = 292462837578287223937453422629572686104_u128 ^ 35038203241014700900929284886847875049_u128;
_7.1 = [3389566842_u32,955241809_u32,1370273753_u32,144894125_u32,1392155953_u32,4280871742_u32,2972709900_u32];
(*_2) = RET;
Goto(bb12)
}
bb24 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb25 = {
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_9) = core::ptr::addr_of_mut!(RET);
_17 = 4374446042167788607_u64;
_15 = [1257627876_i32];
(*_9) = core::ptr::addr_of_mut!(_4);
_6 = true as usize;
(*_9) = core::ptr::addr_of_mut!(RET);
_7.1 = [582240372_u32,1851813668_u32,2287089504_u32,1307914586_u32,2896443425_u32,4280024524_u32,352211468_u32];
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_4);
_14.1 = [143_u8,100_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
match _17 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
4374446042167788607 => bb11,
_ => bb10
}
}
bb26 = {
_2 = core::ptr::addr_of_mut!(_4);
(*_2) = _1;
(*_2) = _1;
_4 = RET;
_1 = (*_2);
_14.1 = [223_u8,222_u8];
(*_2) = _3;
(*_2) = _3;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _3;
_14.3 = (*_2);
_9 = &mut _2;
_14.3 = _1;
_15 = [490158380_i32];
(*_9) = core::ptr::addr_of_mut!(_1);
_13 = [205_u8,208_u8,222_u8,85_u8,29_u8,201_u8,157_u8];
(*_9) = core::ptr::addr_of_mut!(_4);
_4 = _14.3;
_14.0 = _10;
Goto(bb7)
}
bb27 = {
(*_34) = _28 << (*_31);
(*_31) = _25 * _25;
(*_9) = core::ptr::addr_of_mut!(RET);
(*_31) = (*_34) as isize;
(*_31) = _25 & _25;
_47 = (-1776764363_i32) as f64;
(*_9) = core::ptr::addr_of_mut!(_4);
_20 = 27222_i16 * (-6016_i16);
(*_31) = _25 * _25;
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [_24,_24,_24,_24,_24,_24,_24];
(*_9) = core::ptr::addr_of_mut!(_45);
(*_9) = core::ptr::addr_of_mut!(_3);
_7.1 = _26;
(*_31) = _25 * _25;
_12 = (-205678909014848648_i64);
(*_31) = _25;
(*_8) = (-17584_i16) >> (*_34);
_32 = (-61_i8) as isize;
match _12 {
0 => bb25,
1 => bb26,
2 => bb23,
3 => bb28,
4 => bb29,
5 => bb30,
6 => bb31,
340282366920938463463168928522753362808 => bb33,
_ => bb32
}
}
bb28 = {
(*_8) = (-24552_i16) ^ (-19265_i16);
_27 = (*_8) == (*_8);
_14.3 = _4;
_28 = !_23;
_4 = _1;
(*_8) = (-25973_i16) >> _6;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = 150420372_u32 as i16;
_14.0 = [RET,_1,_1,_3];
(*_8) = -(-6782_i16);
(*_8) = (-1774514715_i32) as i16;
_25 = _16 + _16;
(*_9) = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_14.3);
(*_9) = core::ptr::addr_of_mut!(_1);
(*_8) = !4452_i16;
_2 = core::ptr::addr_of_mut!(RET);
match _17 {
0 => bb3,
1 => bb15,
4374446042167788607 => bb17,
_ => bb16
}
}
bb29 = {
_3 = _4;
_4 = _1;
_3 = RET;
RET = _3;
RET = _4;
_2 = core::ptr::addr_of_mut!(RET);
(*_2) = _3;
(*_2) = _1;
_4 = (*_2);
_1 = (*_2);
(*_2) = _4;
(*_2) = _1;
RET = _4;
(*_2) = _1;
(*_2) = _3;
_4 = (*_2);
(*_2) = _3;
(*_2) = _1;
(*_2) = _3;
(*_2) = _1;
(*_2) = _4;
(*_2) = _1;
Goto(bb2)
}
bb30 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb31 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb32 = {
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [204_u8,221_u8,35_u8,1_u8,200_u8,148_u8,250_u8];
(*_9) = core::ptr::addr_of_mut!(_1);
Goto(bb9)
}
bb33 = {
_14.0 = [_3,_14.3,RET,_4];
RET = _4;
(*_8) = 10809_i16 >> (*_34);
(*_9) = core::ptr::addr_of_mut!(_45);
_2 = core::ptr::addr_of_mut!(RET);
(*_8) = (-21906_i16);
_14.0 = _10;
(*_9) = core::ptr::addr_of_mut!((*_2));
(*_34) = _28 << (*_31);
(*_31) = (-509950400_i32) as isize;
(*_9) = core::ptr::addr_of_mut!((*_2));
(*_8) = 18855_i16 >> _24;
(*_31) = _32 * _32;
Goto(bb34)
}
bb34 = {
_45 = (*_2);
_57 = core::ptr::addr_of!((*_8));
(*_34) = _28;
(*_31) = _25 >> (*_57);
(*_8) = (-31765_i16) ^ 11901_i16;
match _12 {
0 => bb11,
1 => bb14,
2 => bb27,
340282366920938463463168928522753362808 => bb36,
_ => bb35
}
}
bb35 = {
_2 = core::ptr::addr_of_mut!(_4);
(*_2) = _1;
(*_2) = _1;
_4 = RET;
_1 = (*_2);
_14.1 = [223_u8,222_u8];
(*_2) = _3;
(*_2) = _3;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _3;
_14.3 = (*_2);
_9 = &mut _2;
_14.3 = _1;
_15 = [490158380_i32];
(*_9) = core::ptr::addr_of_mut!(_1);
_13 = [205_u8,208_u8,222_u8,85_u8,29_u8,201_u8,157_u8];
(*_9) = core::ptr::addr_of_mut!(_4);
_4 = _14.3;
_14.0 = _10;
Goto(bb7)
}
bb36 = {
_28 = !(*_34);
(*_2) = _14.3;
_37 = [23_i8,(-77_i8),62_i8,(-45_i8),(-14_i8),(-98_i8),104_i8,(-21_i8)];
(*_8) = 2159_i16;
_55 = _17 << (*_34);
_3 = _33;
(*_9) = Move(_2);
_43 = !(*_8);
_48 = 33762_u16 & 55657_u16;
_58 = core::ptr::addr_of_mut!(_46);
_8 = core::ptr::addr_of_mut!((*_8));
Goto(bb37)
}
bb37 = {
(*_8) = _43 + _43;
(*_8) = _43 & _43;
(*_58) = 1136439157_u32;
(*_9) = core::ptr::addr_of_mut!(_33);
(*_34) = !_28;
(*_58) = 3546398341_u32;
_23 = !_28;
(*_58) = !3833825415_u32;
(*_34) = _28 | _28;
(*_31) = -_25;
(*_9) = core::ptr::addr_of_mut!(_54);
(*_9) = core::ptr::addr_of_mut!(_33);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_34) = _28 - _28;
_14.0 = _10;
_59 = [_27,_27];
(*_9) = core::ptr::addr_of_mut!(_1);
_10 = [_1,_45,_4,_14.3];
match _12 {
0 => bb1,
1 => bb18,
2 => bb17,
3 => bb4,
4 => bb20,
5 => bb33,
340282366920938463463168928522753362808 => bb38,
_ => bb19
}
}
bb38 = {
(*_34) = _28;
(*_9) = core::ptr::addr_of_mut!(_3);
_4 = RET;
_37 = [(-82_i8),113_i8,62_i8,13_i8,(-92_i8),(-83_i8),(-99_i8),(-92_i8)];
(*_9) = core::ptr::addr_of_mut!(_45);
(*_34) = _28 & _28;
(*_31) = _27 as isize;
RET = _3;
_58 = core::ptr::addr_of_mut!((*_58));
(*_31) = !_25;
match _12 {
340282366920938463463168928522753362808 => bb39,
_ => bb12
}
}
bb39 = {
_54 = _1;
(*_58) = 3016164313_u32 << _48;
_50 = [_24,_24,_24,_24,_24,_24,_24];
_53 = Move(_9);
(*_8) = _43;
(*_31) = _25 + _25;
(*_34) = _28 & _28;
_55 = !_17;
_49 = _11 - _11;
_20 = _43 << (*_58);
_52 = _49 - _49;
_1 = _3;
_14.2 = &mut (*_31);
Goto(bb40)
}
bb40 = {
(*_34) = _28 >> _24;
_3 = RET;
_32 = -_25;
_30 = [_55,_55,_17,_55,_17,_55,_55];
(*_8) = -_43;
_54 = _45;
(*_58) = 3375236486_u32 * 3667307333_u32;
_4 = _54;
_64 = _32 << (*_34);
_17 = _25 as u64;
_7.0 = &_47;
_26 = [(*_58),(*_58),(*_58),(*_58),(*_58),(*_58),(*_58)];
(*_34) = _27 as u128;
_33 = _3;
_14.2 = &mut _64;
_35 = _47 + _47;
_7.0 = &_35;
_63 = core::ptr::addr_of_mut!((*_8));
(*_8) = _43 | _43;
_35 = _32 as f64;
_41 = !_17;
match _12 {
340282366920938463463168928522753362808 => bb42,
_ => bb41
}
}
bb41 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb42 = {
(*_34) = _28;
_62 = &mut _57;
(*_8) = _43;
(*_8) = _55 as i16;
_72 = &mut _24;
(*_34) = _28 | _28;
(*_8) = _43;
_50 = [(*_72),(*_72),(*_72),(*_72),(*_72),(*_72),(*_72)];
(*_34) = _28 + _28;
(*_72) = 133_u8 * 239_u8;
_55 = _41;
_14.1 = [(*_72),(*_72)];
RET = _3;
(*_58) = 945727682_u32;
(*_58) = 505109139_u32 + 3242477698_u32;
_58 = core::ptr::addr_of_mut!(_46);
_59 = [_27,_27];
_2 = core::ptr::addr_of_mut!(_33);
(*_62) = core::ptr::addr_of!((*_8));
Goto(bb43)
}
bb43 = {
_55 = _41 << _17;
(*_58) = 1434208845_u32;
(*_58) = 1212292914_u32;
_50 = _13;
(*_72) = 13_u8;
_75.3 = Adt41::Variant1 { fld0: _27,fld1: _55,fld2: _35,fld3: 20_i8,fld4: (*_58) };
(*_8) = _43 | _43;
(*_8) = _12 as i16;
_26 = [(*_58),(*_58),(*_58),(*_58),_46,(*_58),(*_58)];
_4 = (*_2);
Goto(bb44)
}
bb44 = {
(*_62) = core::ptr::addr_of!((*_8));
_69 = [(*_58),Field::<u32>(Variant(_75.3, 1), 4),(*_58),(*_58),_46,(*_58),(*_58)];
(*_8) = _43;
_68 = _25 + _25;
_14.2 = &mut _25;
(*_72) = 56_u8 << _55;
(*_8) = _43;
_7.0 = &place!(Field::<f64>(Variant(_75.3, 1), 2));
(*_8) = _43 * _43;
Goto(bb45)
}
bb45 = {
(*_62) = core::ptr::addr_of!((*_8));
(*_58) = Field::<u32>(Variant(_75.3, 1), 4) % Field::<u32>(Variant(_75.3, 1), 4);
_79 = Move(_7.0);
(*_34) = (*_8) as u128;
match _12 {
0 => bb7,
1 => bb46,
2 => bb47,
3 => bb48,
4 => bb49,
340282366920938463463168928522753362808 => bb51,
_ => bb50
}
}
bb46 = {
_14.1 = [37_u8,234_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
(*_9) = core::ptr::addr_of_mut!(_4);
(*_9) = core::ptr::addr_of_mut!(_3);
_3 = _14.3;
(*_9) = core::ptr::addr_of_mut!(_3);
_11 = (-7144317400790035960_i64) as f32;
_4 = _14.3;
(*_9) = core::ptr::addr_of_mut!(RET);
_13 = [148_u8,84_u8,28_u8,16_u8,238_u8,24_u8,62_u8];
(*_9) = core::ptr::addr_of_mut!(_3);
(*_9) = core::ptr::addr_of_mut!(RET);
Goto(bb8)
}
bb47 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb48 = {
_12 = -7191005332132091127_i64;
(*_9) = core::ptr::addr_of_mut!(_3);
_14.0 = _10;
_14.1 = [162_u8,14_u8];
_23 = !166100137134321923739750840536453021221_u128;
(*_8) = 1422_i16;
(*_9) = core::ptr::addr_of_mut!(RET);
Goto(bb14)
}
bb49 = {
(*_2) = _4;
RET = _4;
(*_2) = _3;
(*_2) = _1;
(*_2) = _1;
_7.1 = [3633857374_u32,2261709482_u32,4173049679_u32,927512049_u32,3919086351_u32,589154205_u32,3763536965_u32];
(*_2) = _1;
(*_2) = _1;
(*_2) = _1;
Goto(bb4)
}
bb50 = {
(*_2) = _4;
_4 = (*_2);
(*_2) = _3;
(*_2) = _3;
Call((*_2) = fn13(_6, Move(_2), _7.1, _6, _10, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb51 = {
(*_58) = Field::<u32>(Variant(_75.3, 1), 4) & Field::<u32>(Variant(_75.3, 1), 4);
place!(Field::<bool>(Variant(_75.3, 1), 0)) = (*_58) >= (*_58);
(*_62) = core::ptr::addr_of!((*_8));
(*_58) = _17 as u32;
(*_62) = core::ptr::addr_of!((*_8));
(*_8) = _43 >> (*_58);
_37 = [127_i8,100_i8,8_i8,(-88_i8),85_i8,79_i8,(-76_i8),(-36_i8)];
(*_62) = core::ptr::addr_of!((*_8));
(*_58) = Field::<u32>(Variant(_75.3, 1), 4);
(*_8) = _43;
(*_62) = core::ptr::addr_of!((*_8));
(*_58) = !Field::<u32>(Variant(_75.3, 1), 4);
(*_2) = _54;
_49 = _11;
_80 = (*_2);
_8 = core::ptr::addr_of_mut!((*_8));
(*_58) = Field::<u32>(Variant(_75.3, 1), 4) >> (*_8);
_76 = (*_72) != (*_72);
(*_34) = _28 >> (*_72);
_54 = (*_2);
(*_72) = 81_u8 & 164_u8;
(*_62) = core::ptr::addr_of!((*_8));
(*_62) = core::ptr::addr_of!((*_8));
_11 = _48 as f32;
_38 = (*_2);
(*_62) = core::ptr::addr_of!((*_8));
(*_8) = _43 - _43;
Goto(bb52)
}
bb52 = {
Call(_83 = dump_var(Move(_64), Move(_76), Move(_3), Move(_25)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(Move(_59), Move(_37), Move(_12), Move(_10)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_83 = dump_var(Move(_23), Move(_43), Move(_27), Move(_17)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_83 = dump_var(Move(_24), Move(_32), Move(_29), Move(_68)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_83 = dump_var(Move(_26), Move(_28), Move(_41), _84), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: usize,mut _2: *mut char,mut _3: [u32; 7],mut _4: usize,mut _5: [char; 4],mut _6: char) -> char {
mir! {
type RET = char;
let _7: (*const i16,);
let _8: (*const i16,);
let _9: i32;
let _10: &'static i8;
let _11: &'static u8;
let _12: i32;
let _13: u128;
let _14: isize;
let _15: char;
let _16: char;
let _17: ([char; 4], [u8; 2], &'static mut isize, char);
let _18: char;
let _19: &'static mut *mut char;
let _20: i64;
let _21: &'static mut *mut i16;
let _22: [u8; 7];
let _23: usize;
let _24: (&'static f64, [u32; 7]);
let _25: i64;
let _26: Adt43;
let _27: *mut &'static f64;
let _28: char;
let _29: isize;
let _30: (Adt28, &'static mut u16);
let _31: bool;
let _32: isize;
let _33: (&'static mut &'static f64, i16, usize, [char; 4]);
let _34: [u8; 7];
let _35: usize;
let _36: char;
let _37: bool;
let _38: i128;
let _39: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _40: *const u128;
let _41: [i8; 8];
let _42: [char; 4];
let _43: &'static u8;
let _44: [bool; 2];
let _45: *const &'static mut &'static f64;
let _46: bool;
let _47: &'static mut u8;
let _48: [char; 4];
let _49: *mut &'static f64;
let _50: char;
let _51: (*mut &'static f64, &'static mut isize);
let _52: [u8; 2];
let _53: [char; 4];
let _54: usize;
let _55: [i64; 3];
let _56: &'static i8;
let _57: *mut ([char; 4], u8, [char; 4]);
let _58: *mut &'static f64;
let _59: *mut i16;
let _60: *const u128;
let _61: isize;
let _62: *mut &'static mut u8;
let _63: [i128; 2];
let _64: u64;
let _65: u8;
let _66: isize;
let _67: (*mut &'static f64, &'static mut isize);
let _68: (*mut *mut i16, i32, Adt55, *mut i16);
let _69: (*const i16,);
let _70: bool;
let _71: *mut &'static f64;
let _72: &'static i8;
let _73: u128;
let _74: Adt23;
let _75: u8;
let _76: isize;
let _77: &'static bool;
let _78: [u128; 2];
let _79: isize;
let _80: (Adt28, &'static mut u16);
let _81: isize;
let _82: &'static bool;
let _83: &'static mut isize;
let _84: (Adt28, &'static mut u16);
let _85: Adt44;
let _86: f64;
let _87: *mut ([char; 4], u8, [char; 4]);
let _88: u8;
let _89: isize;
let _90: isize;
let _91: f32;
let _92: isize;
let _93: i16;
let _94: *mut u32;
let _95: (*mut *mut i16, i32, Adt55, *mut i16);
let _96: i32;
let _97: (&'static mut &'static f64, i16, usize, [char; 4]);
let _98: &'static i8;
let _99: u16;
let _100: isize;
let _101: isize;
let _102: [char; 4];
let _103: f32;
let _104: &'static mut *mut i16;
let _105: bool;
let _106: i64;
let _107: isize;
let _108: (&'static mut &'static f64, i16, usize, [char; 4]);
let _109: [char; 4];
let _110: &'static mut u8;
let _111: &'static usize;
let _112: &'static mut *mut i16;
let _113: bool;
let _114: (*mut &'static f64, &'static mut isize);
let _115: *const &'static mut &'static f64;
let _116: (*mut &'static f64, &'static mut isize);
let _117: f32;
let _118: *mut *mut &'static f64;
let _119: i128;
let _120: [u8; 7];
let _121: &'static mut *mut i16;
let _122: isize;
let _123: isize;
let _124: usize;
let _125: isize;
let _126: [i8; 8];
let _127: f64;
let _128: *const &'static mut &'static f64;
let _129: [u32; 7];
let _130: *mut *mut i16;
let _131: f32;
let _132: isize;
let _133: isize;
let _134: &'static mut &'static usize;
let _135: i128;
let _136: i32;
let _137: &'static bool;
let _138: &'static mut *mut char;
let _139: char;
let _140: [u32; 7];
let _141: [u32; 7];
let _142: f64;
let _143: *const &'static mut &'static f64;
let _144: u128;
let _145: &'static mut u8;
let _146: &'static mut u8;
let _147: [u32; 7];
let _148: f32;
let _149: *mut &'static mut u8;
let _150: i16;
let _151: *mut ([char; 4], u8, [char; 4]);
let _152: *mut *mut &'static f64;
let _153: ();
let _154: ();
{
_2 = core::ptr::addr_of_mut!(RET);
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
_5 = [(*_2),(*_2),(*_2),(*_2)];
(*_2) = _6;
_6 = (*_2);
Goto(bb1)
}
bb1 = {
(*_2) = _6;
_12 = (-1714943203_i32) & (-1993714786_i32);
_6 = (*_2);
(*_2) = _6;
_4 = _1 | _1;
(*_2) = _6;
(*_2) = _6;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
_1 = _4 ^ _4;
(*_2) = _6;
_9 = _12 >> _4;
_1 = _4 - _4;
_5 = [(*_2),(*_2),(*_2),(*_2)];
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
RET = _6;
(*_2) = _6;
_5 = [(*_2),(*_2),RET,(*_2)];
Goto(bb2)
}
bb2 = {
_12 = (-125506510494878811668308973739272411163_i128) as i32;
(*_2) = _6;
(*_2) = _6;
_5 = [(*_2),(*_2),(*_2),(*_2)];
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
_9 = _12 | _12;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
(*_2) = _6;
_6 = (*_2);
_14 = !9223372036854775807_isize;
(*_2) = _6;
Call(_8.0 = fn14(Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = 9223372036854775807_isize & (-96_isize);
RET = _6;
_16 = RET;
Goto(bb4)
}
bb4 = {
_12 = _9 & _9;
_17.2 = &mut _14;
_22 = [89_u8,239_u8,184_u8,69_u8,189_u8,156_u8,24_u8];
_17.0 = [RET,_6,RET,_16];
_7.0 = Move(_8.0);
RET = _6;
_18 = RET;
_18 = RET;
_2 = core::ptr::addr_of_mut!(_6);
_13 = 200160737482318827662176138587599013890_u128 >> _1;
(*_2) = _18;
_23 = !_4;
(*_2) = RET;
Goto(bb5)
}
bb5 = {
_25 = 11010632911820651253_u64 as i64;
(*_2) = RET;
RET = _6;
_22 = [184_u8,122_u8,194_u8,13_u8,154_u8,78_u8,67_u8];
(*_2) = RET;
_17.3 = (*_2);
_15 = _16;
_28 = (*_2);
RET = _17.3;
Goto(bb6)
}
bb6 = {
_19 = &mut _2;
_20 = 66_u8 as i64;
_29 = 89_isize >> _23;
_8.0 = Move(_7.0);
(*_19) = core::ptr::addr_of_mut!(_15);
_7 = (Move(_8.0),);
_13 = 321026061158264048161694210690459342977_u128 & 329722979056097783601504063704115165119_u128;
(*_19) = core::ptr::addr_of_mut!(_28);
Goto(bb7)
}
bb7 = {
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_15);
_17.0 = [_18,_17.3,RET,_28];
_8.0 = Move(_7.0);
_18 = RET;
_3 = [1449641107_u32,4011963268_u32,2168489173_u32,1104059164_u32,112368571_u32,2480552062_u32,945858528_u32];
(*_19) = core::ptr::addr_of_mut!(_6);
_29 = _6 as isize;
_30.0.fld2 = _20 as f32;
(*_19) = core::ptr::addr_of_mut!(_15);
(*_19) = core::ptr::addr_of_mut!(_18);
Goto(bb8)
}
bb8 = {
(*_19) = core::ptr::addr_of_mut!(_15);
_30.0.fld3 = core::ptr::addr_of_mut!(_30.0.fld1);
(*_19) = core::ptr::addr_of_mut!(_16);
_30.0.fld0 = !51397_u16;
_4 = _1 + _1;
_3 = [1799128828_u32,981448841_u32,3765121080_u32,785688199_u32,1258885267_u32,3469469903_u32,1400770107_u32];
(*_19) = core::ptr::addr_of_mut!(_18);
_29 = (-9223372036854775808_isize);
_24.1 = _3;
_9 = _12;
_33.3 = [_16,_16,RET,_6];
(*_19) = core::ptr::addr_of_mut!(_16);
_3 = _24.1;
Goto(bb9)
}
bb9 = {
_30.0.fld3 = core::ptr::addr_of_mut!(_30.0.fld1);
_32 = -_29;
(*_19) = core::ptr::addr_of_mut!(_6);
_30.0.fld0 = 60836_u16 | 37253_u16;
_31 = true ^ true;
(*_19) = core::ptr::addr_of_mut!(_6);
_12 = _9;
_31 = true;
_29 = _32;
_15 = _28;
_32 = _29;
_35 = _13 as usize;
_33.3 = _5;
_7 = (Move(_8.0),);
_4 = !_1;
(*_19) = core::ptr::addr_of_mut!(_36);
Goto(bb10)
}
bb10 = {
(*_19) = core::ptr::addr_of_mut!(RET);
_25 = -_20;
_3 = _24.1;
_33.2 = (-58223316084648526008349275404316536047_i128) as usize;
_30.0.fld3 = core::ptr::addr_of_mut!(_30.0.fld1);
_36 = _18;
_17.1 = [149_u8,231_u8];
_17.3 = _18;
_34 = [29_u8,57_u8,73_u8,40_u8,120_u8,209_u8,215_u8];
_30.1 = &mut _30.0.fld0;
_3 = [1094584286_u32,3913185078_u32,2629919961_u32,1913725402_u32,1973464239_u32,3578421618_u32,3889146024_u32];
(*_19) = core::ptr::addr_of_mut!(_17.3);
_39.2.1 = _31 as u128;
Goto(bb11)
}
bb11 = {
_39.0 = 36648_u16;
(*_19) = core::ptr::addr_of_mut!(RET);
_18 = _16;
_29 = _32 + _32;
_8.0 = core::ptr::addr_of!(_33.1);
_39.0 = _25 as u16;
(*_19) = core::ptr::addr_of_mut!(_15);
_37 = _31;
(*_19) = core::ptr::addr_of_mut!(_36);
Goto(bb12)
}
bb12 = {
_20 = _25;
(*_19) = core::ptr::addr_of_mut!(_28);
RET = _15;
_38 = !(-39992679182309037422688199704845548568_i128);
(*_19) = core::ptr::addr_of_mut!(_6);
(*_19) = core::ptr::addr_of_mut!(_17.3);
(*_19) = core::ptr::addr_of_mut!(_17.3);
(*_19) = core::ptr::addr_of_mut!(_18);
(*_19) = core::ptr::addr_of_mut!(_6);
_6 = _16;
_25 = _20;
_7.0 = core::ptr::addr_of!(_33.1);
_12 = _9;
(*_19) = core::ptr::addr_of_mut!(_15);
_31 = _37 ^ _37;
_33.1 = 19113_i16 << _4;
_44 = [_37,_31];
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_18);
Goto(bb13)
}
bb13 = {
(*_19) = core::ptr::addr_of_mut!(_6);
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_16);
_44 = [_31,_37];
_37 = _31 & _31;
(*_19) = core::ptr::addr_of_mut!(RET);
_33.1 = 20426_i16 + (-29085_i16);
_32 = _29;
_13 = _39.2.1;
(*_19) = core::ptr::addr_of_mut!(_36);
RET = _6;
_33.3 = [_17.3,RET,_15,_6];
_16 = _36;
(*_19) = core::ptr::addr_of_mut!(_18);
Goto(bb14)
}
bb14 = {
_37 = _31;
RET = _17.3;
_25 = _32 as i64;
_12 = _9 ^ _9;
_48 = [_6,_28,_18,_28];
_41 = [(-31_i8),(-89_i8),1_i8,(-34_i8),24_i8,(-10_i8),(-119_i8),(-53_i8)];
_17.1 = [133_u8,57_u8];
(*_19) = core::ptr::addr_of_mut!(_15);
_22 = [75_u8,180_u8,117_u8,31_u8,102_u8,61_u8,57_u8];
_51.1 = &mut _29;
_51.1 = &mut _32;
_48 = [_18,RET,_6,RET];
_22 = [157_u8,158_u8,91_u8,184_u8,169_u8,100_u8,73_u8];
_9 = _12 >> _12;
_42 = [_18,RET,_6,_28];
_33.1 = -364_i16;
_8.0 = core::ptr::addr_of!(_33.1);
_37 = _31 | _31;
(*_19) = core::ptr::addr_of_mut!(_36);
_6 = _16;
(*_19) = core::ptr::addr_of_mut!(_28);
_3 = [2511058367_u32,1888615899_u32,3352093799_u32,2754961104_u32,3085845036_u32,2758881841_u32,1770650983_u32];
_51.1 = Move(_17.2);
(*_19) = core::ptr::addr_of_mut!(_50);
_15 = _6;
_17.1 = [4_u8,64_u8];
Goto(bb15)
}
bb15 = {
_55 = [_20,_20,_25];
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_15);
_20 = !_25;
_50 = RET;
(*_19) = core::ptr::addr_of_mut!(_16);
(*_19) = core::ptr::addr_of_mut!(_28);
_37 = _9 < _12;
_35 = _1;
_33.1 = (-1734_i16);
_13 = _39.2.1;
_42 = _17.0;
_42 = _5;
_13 = _39.2.1;
_44 = [_37,_37];
_59 = core::ptr::addr_of_mut!(_33.1);
_31 = _37;
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_18);
RET = _50;
Goto(bb16)
}
bb16 = {
_53 = [_50,_50,RET,_50];
_15 = _16;
_9 = _12;
_21 = &mut _59;
_26 = Adt43::Variant1 { fld0: _39.0,fld1: _33.3,fld2: Move((*_19)),fld3: Move(_8.0) };
_63 = [_38,_38];
_8 = (Move(_7.0),);
(*_21) = core::ptr::addr_of_mut!(_33.1);
match _33.1 {
0 => bb1,
340282366920938463463374607431768209722 => bb17,
_ => bb10
}
}
bb17 = {
_1 = _35 - _35;
_60 = core::ptr::addr_of!(_39.2.1);
_28 = _50;
_7 = (Move(Field::<*const i16>(Variant(_26, 1), 3)),);
Goto(bb18)
}
bb18 = {
(*_60) = !_13;
(*_21) = core::ptr::addr_of_mut!(_33.1);
_19 = &mut place!(Field::<*mut char>(Variant(_26, 1), 2));
_34 = _22;
(*_19) = core::ptr::addr_of_mut!(RET);
_20 = !_25;
_33.2 = _37 as usize;
(*_21) = core::ptr::addr_of_mut!(_33.1);
(*_21) = core::ptr::addr_of_mut!(_33.1);
_33.1 = (-15628_i16);
(*_21) = core::ptr::addr_of_mut!(_33.1);
(*_21) = core::ptr::addr_of_mut!(_33.1);
_17.0 = _5;
_34 = [113_u8,62_u8,200_u8,4_u8,101_u8,103_u8,253_u8];
_52 = _17.1;
_17.0 = [_16,RET,_17.3,_50];
(*_60) = _13 << _33.2;
(*_19) = core::ptr::addr_of_mut!(_36);
(*_60) = _13;
_63 = [_38,_38];
(*_60) = _13;
(*_60) = !_13;
_65 = 228_u8 ^ 129_u8;
_42 = [_28,RET,_16,_15];
RET = _6;
Goto(bb19)
}
bb19 = {
_55 = [_25,_25,_20];
_20 = 91_i8 as i64;
_28 = _17.3;
_40 = core::ptr::addr_of!((*_60));
Goto(bb20)
}
bb20 = {
(*_40) = _13 ^ _13;
(*_19) = core::ptr::addr_of_mut!(_15);
_64 = !12999035087284314336_u64;
_50 = _17.3;
(*_60) = _33.2 as u128;
_66 = 54_isize + (-9223372036854775808_isize);
(*_60) = _13 << _12;
(*_21) = core::ptr::addr_of_mut!(_33.1);
_48 = [_18,RET,_18,_50];
_8 = Move(_7);
_36 = _6;
_65 = 168_u8 >> (*_60);
_42 = [_50,RET,_36,_6];
_31 = _37 | _37;
_47 = &mut _65;
_41 = [37_i8,106_i8,42_i8,115_i8,85_i8,0_i8,(-9_i8),(-31_i8)];
(*_19) = core::ptr::addr_of_mut!(_17.3);
(*_21) = core::ptr::addr_of_mut!(_33.1);
(*_47) = 6_u8 & 172_u8;
Goto(bb21)
}
bb21 = {
(*_47) = 156_u8 - 60_u8;
Goto(bb22)
}
bb22 = {
_69 = (Move(_8.0),);
_3 = [2048678514_u32,2151096535_u32,4026498955_u32,2651025169_u32,2379214427_u32,2710887428_u32,1658186626_u32];
(*_19) = core::ptr::addr_of_mut!(_28);
_31 = !_37;
_68.3 = Move((*_21));
_7 = (Move(_69.0),);
_6 = RET;
_62 = core::ptr::addr_of_mut!(_47);
_66 = _39.0 as isize;
(*_47) = _4 as u8;
match _33.1 {
340282366920938463463374607431768195828 => bb24,
_ => bb23
}
}
bb23 = {
_12 = _9 & _9;
_17.2 = &mut _14;
_22 = [89_u8,239_u8,184_u8,69_u8,189_u8,156_u8,24_u8];
_17.0 = [RET,_6,RET,_16];
_7.0 = Move(_8.0);
RET = _6;
_18 = RET;
_18 = RET;
_2 = core::ptr::addr_of_mut!(_6);
_13 = 200160737482318827662176138587599013890_u128 >> _1;
(*_2) = _18;
_23 = !_4;
(*_2) = RET;
Goto(bb5)
}
bb24 = {
_2 = core::ptr::addr_of_mut!(_16);
(*_19) = core::ptr::addr_of_mut!((*_2));
_6 = (*_2);
(*_19) = core::ptr::addr_of_mut!(_18);
_68.3 = core::ptr::addr_of_mut!(_33.1);
_52 = _17.1;
(*_47) = 32_u8 >> (*_60);
_54 = _35 >> (*_60);
(*_60) = _13 ^ _13;
_60 = core::ptr::addr_of!((*_60));
(*_47) = 84_u8 >> _9;
_11 = &(*_47);
(*_60) = _13;
(*_47) = 7_u8 & 229_u8;
_8 = Move(_7);
(*_2) = _17.3;
RET = (*_2);
(*_60) = _13 * _13;
(*_2) = _18;
Goto(bb25)
}
bb25 = {
(*_19) = core::ptr::addr_of_mut!((*_2));
(*_19) = core::ptr::addr_of_mut!(RET);
(*_19) = core::ptr::addr_of_mut!((*_2));
_81 = _66;
(*_2) = _18;
_84.0.fld2 = _20 as f32;
_63 = [_38,_38];
_17.0 = [(*_2),(*_2),(*_2),_36];
(*_47) = !67_u8;
(*_60) = _13 + _13;
(*_2) = _15;
_82 = &_37;
_48 = [(*_2),_15,(*_2),(*_2)];
_68.3 = core::ptr::addr_of_mut!(_33.1);
_80.0.fld0 = _39.0 >> _54;
(*_2) = _50;
Goto(bb26)
}
bb26 = {
(*_60) = !_13;
_68.1 = _9 * _12;
_77 = &(*_82);
_80.0.fld2 = _84.0.fld2;
_80.0.fld0 = !_39.0;
Goto(bb27)
}
bb27 = {
_78 = [(*_60),(*_60)];
_84.0.fld4 = _33.2 | _1;
_36 = (*_2);
_82 = &_31;
_79 = _66 | _81;
_76 = _66;
_68.1 = _9;
_35 = !_54;
(*_60) = _13 + _13;
_80.0.fld1 = 2891134472_u32 | 3585706162_u32;
_51.1 = &mut _81;
_39.2.0 = Adt23::Variant1 { fld0: _79 };
(*_60) = !_13;
_50 = (*_2);
_35 = _54 - _84.0.fld4;
(*_2) = _18;
(*_47) = !47_u8;
_2 = core::ptr::addr_of_mut!(_16);
_69.0 = Move(_8.0);
(*_2) = _17.3;
(*_60) = _13;
(*_60) = _13;
(*_2) = _28;
Goto(bb28)
}
bb28 = {
(*_60) = !_13;
_59 = core::ptr::addr_of_mut!(_33.1);
(*_2) = _28;
(*_60) = !_13;
_6 = (*_2);
_52 = _17.1;
_84.0.fld4 = !_35;
(*_2) = _15;
(*_19) = core::ptr::addr_of_mut!((*_2));
(*_19) = core::ptr::addr_of_mut!(_18);
Goto(bb29)
}
bb29 = {
(*_59) = (-31424_i16) ^ 13485_i16;
_31 = !(*_77);
(*_59) = (-61_i8) as i16;
(*_47) = !197_u8;
(*_47) = 161_u8;
_73 = (*_60) | (*_60);
(*_60) = _73;
(*_2) = _6;
_77 = &_31;
_63 = [_38,_38];
(*_2) = _28;
(*_2) = _28;
_48 = [(*_2),_18,(*_2),(*_2)];
_17.2 = &mut place!(Field::<isize>(Variant(_39.2.0, 1), 0));
(*_60) = _73 << _35;
_94 = core::ptr::addr_of_mut!(_84.0.fld1);
_68.3 = Move(_59);
_95.0 = core::ptr::addr_of_mut!(_68.3);
_80.0.fld4 = (*_77) as usize;
(*_47) = _38 as u8;
(*_94) = !_80.0.fld1;
_95.3 = core::ptr::addr_of_mut!(_93);
_4 = _33.2 | _54;
(*_94) = (*_77) as u32;
_12 = _9 ^ _68.1;
(*_19) = core::ptr::addr_of_mut!((*_2));
Goto(bb30)
}
bb30 = {
_80.0.fld0 = !3968_u16;
(*_60) = (*_47) as u128;
(*_47) = 182_u8;
_7.0 = core::ptr::addr_of!(_93);
(*_2) = _18;
_68.0 = core::ptr::addr_of_mut!(_59);
(*_47) = 161_u8 >> (*_94);
_4 = _35 - _35;
_79 = _76 >> (*_94);
Goto(bb31)
}
bb31 = {
(*_2) = _6;
(*_60) = _13;
_46 = !(*_77);
Goto(bb32)
}
bb32 = {
_13 = (*_60) << _9;
_17.3 = (*_2);
(*_19) = core::ptr::addr_of_mut!((*_2));
(*_2) = _6;
_60 = core::ptr::addr_of!((*_60));
_53 = [(*_2),(*_2),(*_2),(*_2)];
_100 = _79;
_64 = !16821016373045445428_u64;
_7.0 = core::ptr::addr_of!(_33.1);
(*_47) = 63_u8 - 183_u8;
_22 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
_68.0 = core::ptr::addr_of_mut!(_95.3);
(*_19) = core::ptr::addr_of_mut!((*_2));
_95.2 = Adt55::Variant1 { fld0: Move((*_19)),fld1: Move(_94),fld2: _100,fld3: _80.0.fld0,fld4: _33.1 };
_79 = -_66;
(*_60) = _25 as u128;
(*_2) = _17.3;
(*_60) = _13;
(*_60) = _13;
place!(Field::<[char; 4]>(Variant(_26, 1), 1)) = _53;
_8.0 = core::ptr::addr_of!(_97.1);
_84.0.fld0 = !Field::<u16>(Variant(_26, 1), 0);
(*_2) = _6;
_75 = (*_47) + (*_47);
Call((*_60) = core::intrinsics::transmute(_38), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
(*_60) = _13;
(*_2) = RET;
place!(Field::<u16>(Variant(_95.2, 1), 3)) = _84.0.fld0 - Field::<u16>(Variant(_26, 1), 0);
(*_47) = _75 * _75;
(*_47) = _75 - _75;
(*_62) = &mut _75;
_95.1 = _9 << _35;
_43 = &(*_47);
_55 = [_25,_25,_20];
_78 = [_13,(*_60)];
(*_47) = 115_u8 + 88_u8;
_77 = &_37;
(*_60) = _13 - _13;
_62 = core::ptr::addr_of_mut!((*_62));
_94 = core::ptr::addr_of_mut!(_84.0.fld1);
_104 = &mut _68.3;
(*_47) = !33_u8;
(*_60) = !_13;
_5 = [(*_2),(*_2),(*_2),(*_2)];
_94 = core::ptr::addr_of_mut!((*_94));
Goto(bb34)
}
bb34 = {
(*_94) = _80.0.fld1 - _80.0.fld1;
place!(Field::<*const i16>(Variant(_26, 1), 3)) = Move(_7.0);
(*_2) = _28;
(*_104) = core::ptr::addr_of_mut!(_97.1);
_17.2 = &mut place!(Field::<isize>(Variant(_95.2, 1), 2));
_19 = &mut _2;
_4 = _54;
_74 = Adt23::Variant2 { fld0: Field::<u16>(Variant(_26, 1), 0),fld1: _38 };
_80.0.fld4 = _35 << _54;
_84.0.fld3 = core::ptr::addr_of_mut!((*_94));
(*_19) = core::ptr::addr_of_mut!(_6);
(*_104) = core::ptr::addr_of_mut!(_97.1);
_59 = core::ptr::addr_of_mut!(_33.1);
(*_59) = (-15022_i16);
(*_104) = core::ptr::addr_of_mut!((*_59));
_97.3 = [_15,_16,RET,RET];
_92 = _35 as isize;
_77 = &_46;
_50 = _6;
_113 = _80.0.fld4 < _35;
Goto(bb35)
}
bb35 = {
_84.1 = &mut place!(Field::<u16>(Variant(_74, 2), 0));
(*_47) = 82_u8 - 199_u8;
_80.0 = Move(_84.0);
_17 = (Field::<[char; 4]>(Variant(_26, 1), 1), _52, Move(_51.1), _16);
(*_60) = _13 >> _35;
_117 = _80.0.fld2 * _80.0.fld2;
_91 = -_117;
(*_94) = _80.0.fld1 | _80.0.fld1;
_101 = _64 as isize;
(*_60) = !_13;
_84.0.fld2 = -_91;
(*_59) = _50 as i16;
(*_60) = !_13;
place!(Field::<[char; 4]>(Variant(_26, 1), 1)) = _33.3;
(*_19) = core::ptr::addr_of_mut!(_28);
_15 = _50;
_104 = &mut _59;
_1 = (*_60) as usize;
(*_60) = !_13;
_5 = [_15,_16,_15,_18];
_97.1 = _33.1 - _33.1;
(*_60) = _73 * _73;
Goto(bb36)
}
bb36 = {
(*_104) = core::ptr::addr_of_mut!(_97.1);
(*_104) = core::ptr::addr_of_mut!(_108.1);
(*_94) = _28 as u32;
_107 = -_92;
(*_60) = _80.0.fld4 as u128;
(*_47) = 242_u8;
(*_104) = core::ptr::addr_of_mut!(_97.1);
(*_104) = core::ptr::addr_of_mut!(_97.1);
(*_104) = core::ptr::addr_of_mut!(_97.1);
(*_19) = core::ptr::addr_of_mut!(_36);
_7 = Move(_69);
(*_60) = _13 ^ _13;
_70 = (*_77) ^ (*_77);
match (*_47) {
242 => bb38,
_ => bb37
}
}
bb37 = {
_37 = _31;
RET = _17.3;
_25 = _32 as i64;
_12 = _9 ^ _9;
_48 = [_6,_28,_18,_28];
_41 = [(-31_i8),(-89_i8),1_i8,(-34_i8),24_i8,(-10_i8),(-119_i8),(-53_i8)];
_17.1 = [133_u8,57_u8];
(*_19) = core::ptr::addr_of_mut!(_15);
_22 = [75_u8,180_u8,117_u8,31_u8,102_u8,61_u8,57_u8];
_51.1 = &mut _29;
_51.1 = &mut _32;
_48 = [_18,RET,_6,RET];
_22 = [157_u8,158_u8,91_u8,184_u8,169_u8,100_u8,73_u8];
_9 = _12 >> _12;
_42 = [_18,RET,_6,_28];
_33.1 = -364_i16;
_8.0 = core::ptr::addr_of!(_33.1);
_37 = _31 | _31;
(*_19) = core::ptr::addr_of_mut!(_36);
_6 = _16;
(*_19) = core::ptr::addr_of_mut!(_28);
_3 = [2511058367_u32,1888615899_u32,3352093799_u32,2754961104_u32,3085845036_u32,2758881841_u32,1770650983_u32];
_51.1 = Move(_17.2);
(*_19) = core::ptr::addr_of_mut!(_50);
_15 = _6;
_17.1 = [4_u8,64_u8];
Goto(bb15)
}
bb38 = {
(*_47) = !181_u8;
_120 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
_96 = _9 + _12;
_89 = _100;
_103 = -_91;
_70 = _37 == (*_77);
place!(Field::<*const i16>(Variant(_26, 1), 3)) = core::ptr::addr_of!(_108.1);
_36 = _18;
_24.1 = [(*_94),_80.0.fld1,(*_94),(*_94),(*_94),_84.0.fld1,(*_94)];
_118 = core::ptr::addr_of_mut!(_71);
(*_19) = core::ptr::addr_of_mut!(_36);
_90 = _100 >> (*_60);
(*_47) = 73_u8 ^ 10_u8;
_103 = _97.1 as f32;
_110 = &mut (*_47);
(*_110) = 54_u8 * 63_u8;
(*_62) = Move(_110);
_100 = _92 + _89;
_94 = core::ptr::addr_of_mut!(_80.0.fld1);
_46 = !_37;
(*_94) = !_84.0.fld1;
(*_19) = core::ptr::addr_of_mut!(_17.3);
(*_104) = core::ptr::addr_of_mut!(_97.1);
_105 = _96 >= _12;
_99 = !_80.0.fld0;
Goto(bb39)
}
bb39 = {
_67.1 = &mut _92;
(*_104) = core::ptr::addr_of_mut!(_97.1);
(*_94) = !_84.0.fld1;
_99 = !_80.0.fld0;
_89 = _107 * _100;
(*_94) = _84.0.fld1 >> _100;
(*_94) = !_84.0.fld1;
_125 = _90 >> (*_60);
_82 = &_105;
_52 = [209_u8,19_u8];
(*_60) = _12 as u128;
_96 = _80.0.fld4 as i32;
_20 = (*_82) as i64;
(*_19) = core::ptr::addr_of_mut!(_50);
_17.2 = &mut _101;
(*_94) = _84.0.fld1 * _84.0.fld1;
(*_60) = !_13;
_76 = -_89;
_80.0 = Adt28 { fld0: _99,fld1: _84.0.fld1,fld2: _117,fld3: Move(_94),fld4: _33.2 };
(*_19) = core::ptr::addr_of_mut!(_28);
_119 = _38;
_3 = [_80.0.fld1,_80.0.fld1,_80.0.fld1,_84.0.fld1,_84.0.fld1,_80.0.fld1,_84.0.fld1];
_91 = _103;
(*_60) = _13;
_108.2 = _4;
_105 = !_70;
_34 = [69_u8,10_u8,34_u8,10_u8,99_u8,153_u8,29_u8];
Goto(bb40)
}
bb40 = {
_1 = !_35;
(*_19) = core::ptr::addr_of_mut!(_15);
_33.1 = _97.1;
_80.0.fld3 = core::ptr::addr_of_mut!(_80.0.fld1);
_69.0 = core::ptr::addr_of!(_93);
_88 = 244_u8 & 119_u8;
_43 = &_88;
(*_60) = _20 as u128;
(*_19) = core::ptr::addr_of_mut!(_6);
_38 = 56_i8 as i128;
_84.0.fld0 = _99 >> (*_60);
(*_19) = core::ptr::addr_of_mut!(_36);
(*_60) = _13;
(*_19) = core::ptr::addr_of_mut!(_36);
_35 = !_33.2;
_50 = _28;
(*_19) = core::ptr::addr_of_mut!(_16);
(*_104) = core::ptr::addr_of_mut!(_93);
_83 = &mut _90;
_18 = _17.3;
(*_19) = core::ptr::addr_of_mut!(_17.3);
_5 = _97.3;
_22 = [(*_43),(*_43),(*_43),(*_43),(*_43),(*_43),(*_43)];
(*_104) = core::ptr::addr_of_mut!(_108.1);
Goto(bb41)
}
bb41 = {
_83 = &mut _89;
_131 = _91 * _80.0.fld2;
(*_19) = core::ptr::addr_of_mut!(RET);
(*_60) = _20 as u128;
_20 = !_25;
(*_62) = &mut (*_43);
(*_83) = _100;
_97.1 = _33.1 & _33.1;
_7 = (Move(_8.0),);
_108.1 = _84.0.fld0 as i16;
(*_83) = _100 + _107;
_100 = (*_83) ^ (*_83);
_99 = _84.0.fld0 << (*_60);
_114.1 = &mut (*_83);
(*_60) = _13 ^ _13;
_77 = &_31;
(*_19) = core::ptr::addr_of_mut!(_17.3);
Goto(bb42)
}
bb42 = {
_41 = [45_i8,107_i8,(-111_i8),(-8_i8),38_i8,(-38_i8),(-126_i8),(-28_i8)];
place!(Field::<*mut char>(Variant(_26, 1), 2)) = core::ptr::addr_of_mut!(_6);
_77 = Move(_82);
_126 = _41;
RET = _17.3;
_7 = (Move(Field::<*const i16>(Variant(_26, 1), 3)),);
_8.0 = core::ptr::addr_of!(_108.1);
_84.0.fld3 = core::ptr::addr_of_mut!(_80.0.fld1);
(*_104) = core::ptr::addr_of_mut!(_97.1);
_99 = !_84.0.fld0;
_102 = [_6,_6,_36,_36];
(*_60) = _13 & _13;
_22 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
_111 = &_1;
_93 = _108.1 + _108.1;
_54 = (*_111) >> (*_111);
_46 = _113;
_4 = (*_111) - _80.0.fld4;
(*_19) = core::ptr::addr_of_mut!(_36);
(*_19) = core::ptr::addr_of_mut!(_15);
(*_104) = core::ptr::addr_of_mut!(_97.1);
place!(Field::<*const i16>(Variant(_26, 1), 3)) = Move(_8.0);
_122 = _125;
Goto(bb43)
}
bb43 = {
_120 = [(*_47),(*_47),(*_47),(*_47),(*_47),(*_47),(*_47)];
(*_19) = Move(Field::<*mut char>(Variant(_26, 1), 2));
(*_60) = _73;
_121 = Move(_104);
_116.1 = &mut _66;
(*_60) = !_13;
(*_60) = (*_47) as u128;
(*_19) = core::ptr::addr_of_mut!(_36);
_140 = _24.1;
_1 = _25 as usize;
_102 = _5;
_114.1 = &mut _107;
Goto(bb44)
}
bb44 = {
_109 = [_50,_16,RET,_6];
Goto(bb45)
}
bb45 = {
(*_19) = core::ptr::addr_of_mut!(_139);
_99 = _84.0.fld0 & _84.0.fld0;
_40 = core::ptr::addr_of!((*_60));
_131 = _80.0.fld2 * _80.0.fld2;
_28 = _16;
_7 = (Move(Field::<*const i16>(Variant(_26, 1), 3)),);
_142 = _125 as f64;
_86 = _142 + _142;
_135 = _38 | _38;
_12 = (*_47) as i32;
_8 = (Move(_69.0),);
(*_40) = _13 - _73;
_84.0.fld4 = _33.2;
Goto(bb46)
}
bb46 = {
_146 = Move((*_62));
_80.0 = Move(_84.0);
_84.0.fld3 = Move(_80.0.fld3);
_55 = [_20,_25,_20];
_144 = (*_60) - (*_60);
_7 = (Move(_8.0),);
_8 = (Move(_7.0),);
_139 = _15;
_80.0 = Adt28 { fld0: _99,fld1: 2529084516_u32,fld2: _91,fld3: Move(_84.0.fld3),fld4: _54 };
_51.1 = Move(_67.1);
_25 = _38 as i64;
_63 = [_135,_135];
_84.0.fld4 = !_54;
_123 = !_125;
_99 = _80.0.fld0 & _80.0.fld0;
place!(Field::<*const i16>(Variant(_26, 1), 3)) = Move(_8.0);
_8.0 = core::ptr::addr_of!(_33.1);
_1 = _25 as usize;
_105 = _31;
_108.1 = _93 - _93;
_106 = _25 & _20;
_97.1 = _99 as i16;
Goto(bb47)
}
bb47 = {
_9 = _96;
_124 = _80.0.fld4;
place!(Field::<*const i16>(Variant(_26, 1), 3)) = Move(_8.0);
_132 = _76 ^ _123;
_125 = _76 * _122;
_70 = _46 | _113;
(*_19) = core::ptr::addr_of_mut!(_18);
(*_19) = core::ptr::addr_of_mut!(_36);
_43 = Move(_11);
_100 = (*_40) as isize;
_108.3 = [_50,_18,_16,_6];
_3 = [_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1];
_70 = (*_40) == (*_60);
_135 = _38 << _76;
_80.0.fld4 = _108.2 ^ _84.0.fld4;
_127 = _86;
match _80.0.fld1 {
0 => bb48,
1 => bb49,
2 => bb50,
2529084516 => bb52,
_ => bb51
}
}
bb48 = {
_78 = [(*_60),(*_60)];
_84.0.fld4 = _33.2 | _1;
_36 = (*_2);
_82 = &_31;
_79 = _66 | _81;
_76 = _66;
_68.1 = _9;
_35 = !_54;
(*_60) = _13 + _13;
_80.0.fld1 = 2891134472_u32 | 3585706162_u32;
_51.1 = &mut _81;
_39.2.0 = Adt23::Variant1 { fld0: _79 };
(*_60) = !_13;
_50 = (*_2);
_35 = _54 - _84.0.fld4;
(*_2) = _18;
(*_47) = !47_u8;
_2 = core::ptr::addr_of_mut!(_16);
_69.0 = Move(_8.0);
(*_2) = _17.3;
(*_60) = _13;
(*_60) = _13;
(*_2) = _28;
Goto(bb28)
}
bb49 = {
_69 = (Move(_8.0),);
_3 = [2048678514_u32,2151096535_u32,4026498955_u32,2651025169_u32,2379214427_u32,2710887428_u32,1658186626_u32];
(*_19) = core::ptr::addr_of_mut!(_28);
_31 = !_37;
_68.3 = Move((*_21));
_7 = (Move(_69.0),);
_6 = RET;
_62 = core::ptr::addr_of_mut!(_47);
_66 = _39.0 as isize;
(*_47) = _4 as u8;
match _33.1 {
340282366920938463463374607431768195828 => bb24,
_ => bb23
}
}
bb50 = {
_55 = [_20,_20,_25];
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_15);
_20 = !_25;
_50 = RET;
(*_19) = core::ptr::addr_of_mut!(_16);
(*_19) = core::ptr::addr_of_mut!(_28);
_37 = _9 < _12;
_35 = _1;
_33.1 = (-1734_i16);
_13 = _39.2.1;
_42 = _17.0;
_42 = _5;
_13 = _39.2.1;
_44 = [_37,_37];
_59 = core::ptr::addr_of_mut!(_33.1);
_31 = _37;
(*_19) = core::ptr::addr_of_mut!(_28);
(*_19) = core::ptr::addr_of_mut!(_18);
RET = _50;
Goto(bb16)
}
bb51 = {
_12 = _9 & _9;
_17.2 = &mut _14;
_22 = [89_u8,239_u8,184_u8,69_u8,189_u8,156_u8,24_u8];
_17.0 = [RET,_6,RET,_16];
_7.0 = Move(_8.0);
RET = _6;
_18 = RET;
_18 = RET;
_2 = core::ptr::addr_of_mut!(_6);
_13 = 200160737482318827662176138587599013890_u128 >> _1;
(*_2) = _18;
_23 = !_4;
(*_2) = RET;
Goto(bb5)
}
bb52 = {
_129 = [_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1,_80.0.fld1];
_85 = Adt44::Variant0 { fld0: _70,fld1: _63,fld2: Move(_80.0),fld3: (-38_i8),fld4: _93,fld5: _97.3,fld6: _64,fld7: _103 };
_12 = _96;
place!(Field::<bool>(Variant(_85, 0), 0)) = _37 ^ _37;
_133 = Field::<Adt28>(Variant(_85, 0), 2).fld0 as isize;
(*_60) = _144 - _13;
_96 = _142 as i32;
place!(Field::<u64>(Variant(_85, 0), 6)) = _64;
_76 = _108.1 as isize;
_37 = _46 | _113;
_80.0.fld4 = Field::<Adt28>(Variant(_85, 0), 2).fld4;
place!(Field::<Adt28>(Variant(_85, 0), 2)).fld3 = core::ptr::addr_of_mut!(_84.0.fld1);
(*_19) = core::ptr::addr_of_mut!(_17.3);
_6 = _17.3;
_149 = Move(_62);
place!(Field::<[i128; 2]>(Variant(_85, 0), 1)) = [_135,_135];
_48 = [_139,RET,_17.3,_18];
Goto(bb53)
}
bb53 = {
Call(_153 = dump_var(Move(_101), Move(_107), Move(_4), Move(_3)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_153 = dump_var(Move(_65), Move(_119), Move(_135), Move(_55)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_153 = dump_var(Move(_32), Move(_54), Move(_15), Move(_37)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_153 = dump_var(Move(_75), Move(_13), Move(_14), Move(_36)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_153 = dump_var(Move(_64), Move(_9), Move(_123), Move(_106)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_153 = dump_var(Move(_125), Move(_129), Move(_28), Move(_122)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_153 = dump_var(Move(_133), Move(_140), Move(_124), Move(_38)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_153 = dump_var(Move(_70), Move(_31), Move(_16), Move(_23)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_153 = dump_var(Move(_42), Move(_105), Move(_139), Move(_81)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_153 = dump_var(Move(_89), _154, _154, _154), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *mut char) -> *const i16 {
mir! {
type RET = *const i16;
let _2: usize;
let _3: *mut u32;
let _4: isize;
let _5: &'static mut isize;
let _6: &'static mut &'static usize;
let _7: &'static mut &'static f64;
let _8: isize;
let _9: i128;
let _10: *const u128;
let _11: *mut &'static mut u8;
let _12: &'static mut &'static usize;
let _13: usize;
let _14: isize;
let _15: usize;
let _16: ([char; 4], [u8; 2], &'static mut isize, char);
let _17: char;
let _18: &'static mut u16;
let _19: *const &'static mut u8;
let _20: isize;
let _21: usize;
let _22: [bool; 2];
let _23: bool;
let _24: f32;
let _25: Adt28;
let _26: *mut char;
let _27: u16;
let _28: u128;
let _29: isize;
let _30: &'static mut &'static usize;
let _31: &'static f64;
let _32: f32;
let _33: f32;
let _34: Adt43;
let _35: i128;
let _36: i128;
let _37: &'static mut &'static usize;
let _38: char;
let _39: isize;
let _40: u128;
let _41: &'static mut *mut char;
let _42: [i64; 3];
let _43: isize;
let _44: &'static &'static mut u8;
let _45: *const &'static bool;
let _46: isize;
let _47: char;
let _48: &'static mut *const u128;
let _49: &'static mut *mut char;
let _50: *mut i16;
let _51: *const &'static bool;
let _52: &'static mut i8;
let _53: &'static f64;
let _54: i64;
let _55: i16;
let _56: u32;
let _57: *const u128;
let _58: ();
let _59: ();
{
_2 = 1_usize >> 44318079240968129287089474149259607487_i128;
_2 = 14347007273244699120_usize << (-1088711522_i32);
_2 = 58885_u16 as usize;
_2 = 1_usize ^ 461376307652367939_usize;
_2 = 1_usize;
_2 = !4_usize;
_2 = 7375206998268826653_usize * 1_usize;
_2 = 0_usize & 2_usize;
_2 = 13506240720410442415_usize | 13704476158297254565_usize;
_2 = 2_usize - 1312574487409318682_usize;
_2 = 5531617689722520229_u64 as usize;
_4 = 29_isize;
_5 = &mut _4;
(*_5) = (-9223372036854775808_isize);
(*_5) = 6405135298094063114_i64 as isize;
(*_5) = (-37_isize) * (-9223372036854775808_isize);
Call((*_5) = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_5) = -(-9223372036854775808_isize);
(*_5) = (-52_isize) & (-63_isize);
(*_5) = 9223372036854775807_isize << _2;
(*_5) = (-9223372036854775808_isize);
(*_5) = _2 as isize;
(*_5) = (-9223372036854775808_isize);
(*_5) = -9223372036854775807_isize;
(*_5) = '\u{bb4b8}' as isize;
(*_5) = (-9223372036854775808_isize);
(*_5) = 16_isize;
match (*_5) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
16 => bb9,
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
(*_5) = 9223372036854775807_isize | (-43_isize);
(*_5) = _2 as isize;
(*_5) = 9223372036854775807_isize - (-9223372036854775808_isize);
(*_5) = !(-9223372036854775808_isize);
(*_5) = 9223372036854775807_isize | 33_isize;
(*_5) = (-9223372036854775808_isize);
(*_5) = !(-9223372036854775808_isize);
_8 = (*_5) << (*_5);
(*_5) = -_8;
(*_5) = _8 | _8;
(*_5) = _8;
(*_5) = _8 | _8;
_8 = !(*_5);
(*_5) = _8 ^ _8;
(*_5) = _8 ^ _8;
(*_5) = _8 | _8;
(*_5) = _8 << _8;
_8 = (*_5);
_9 = (-49000960240948798105054708674249521461_i128);
(*_5) = _8;
_8 = (*_5) << (*_5);
(*_5) = 1062578187_i32 as isize;
(*_5) = _8;
(*_5) = _8;
Goto(bb10)
}
bb10 = {
(*_5) = _8 & _8;
(*_5) = (-118_i8) as isize;
(*_5) = !_8;
(*_5) = _8 & _8;
_13 = _2 | _2;
(*_5) = 159223137102859098858409482718504899910_u128 as isize;
(*_5) = _8 << _8;
(*_5) = _8 * _8;
(*_5) = _8 << _13;
(*_5) = _8 | _8;
(*_5) = _8 + _8;
Goto(bb11)
}
bb11 = {
(*_5) = _8;
_13 = _2 | _2;
_14 = (*_5) | (*_5);
(*_5) = 210014075082251877994316946848216691163_u128 as isize;
(*_5) = !_14;
(*_5) = _14 & _8;
_13 = _2;
_16.3 = '\u{4ea2f}';
_16.1 = [24_u8,170_u8];
(*_5) = 17679776367926510271_u64 as isize;
(*_5) = 4773565934796495322_i64 as isize;
_15 = _2 << _14;
(*_5) = _15 as isize;
_20 = -(*_5);
match _9 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
291281406679989665358319898757518689995 => bb16,
_ => bb15
}
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
Return()
}
bb16 = {
_21 = 5847283032313203414_i64 as usize;
_21 = _15 << (*_5);
(*_5) = _20 * _14;
_9 = (-108179687004978600818209472946445842177_i128) ^ (-87841254644498397409197680640346524098_i128);
Goto(bb17)
}
bb17 = {
_13 = (-23435_i16) as usize;
_8 = (*_5) >> (*_5);
(*_5) = !_8;
_16.2 = &mut (*_5);
_20 = _8;
Goto(bb18)
}
bb18 = {
_9 = (-76598387310280779309013782594904382931_i128) - (-162887884714621768172113505571894904000_i128);
_17 = _16.3;
Goto(bb19)
}
bb19 = {
_21 = _15;
_23 = true;
_14 = !_8;
_24 = _14 as f32;
_22 = [_23,_23];
_22 = [_23,_23];
_21 = _15 * _15;
_14 = -_8;
_13 = _15;
_16.2 = &mut _20;
_25.fld2 = _24 - _24;
Goto(bb20)
}
bb20 = {
_16.3 = _17;
_16.0 = [_16.3,_17,_16.3,_16.3];
_24 = _25.fld2 - _25.fld2;
_21 = _13;
_25.fld4 = _17 as usize;
_25.fld3 = core::ptr::addr_of_mut!(_25.fld1);
_16.0 = [_17,_17,_17,_16.3];
_13 = _2;
_22 = [_23,_23];
_21 = _15 - _15;
_25.fld3 = core::ptr::addr_of_mut!(_25.fld1);
_3 = core::ptr::addr_of_mut!(_25.fld1);
(*_3) = 4674042_u32 ^ 1546192987_u32;
(*_3) = 681455013_u32;
match (*_3) {
0 => bb21,
681455013 => bb23,
_ => bb22
}
}
bb21 = {
Return()
}
bb22 = {
(*_5) = _8;
_13 = _2 | _2;
_14 = (*_5) | (*_5);
(*_5) = 210014075082251877994316946848216691163_u128 as isize;
(*_5) = !_14;
(*_5) = _14 & _8;
_13 = _2;
_16.3 = '\u{4ea2f}';
_16.1 = [24_u8,170_u8];
(*_5) = 17679776367926510271_u64 as isize;
(*_5) = 4773565934796495322_i64 as isize;
_15 = _2 << _14;
(*_5) = _15 as isize;
_20 = -(*_5);
match _9 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
291281406679989665358319898757518689995 => bb16,
_ => bb15
}
}
bb23 = {
(*_3) = 2545109717_u32 * 1751029052_u32;
_25.fld0 = 34084_u16 & 64797_u16;
(*_3) = _16.3 as u32;
(*_3) = 2197406141_u32 | 1177273616_u32;
_26 = core::ptr::addr_of_mut!(_16.3);
_10 = core::ptr::addr_of!(_28);
(*_10) = 329614840910170018566869893859585831098_u128 * 282217041945138893702754547902486025890_u128;
(*_10) = !146789313697147592266640071744966633355_u128;
(*_10) = 12_i8 as u128;
(*_10) = 164055702696084290599278201422487709711_u128;
(*_3) = _15 as u32;
(*_26) = _17;
(*_3) = 1245167541_u32 >> _14;
_1 = Move(_26);
(*_10) = 229797398408156645908132184959192615277_u128 + 157182762450919871367128748993673848978_u128;
(*_3) = (-129811894387694610_i64) as u32;
(*_10) = 62_i8 as u128;
(*_10) = !245915364615860234898261959093655971695_u128;
(*_10) = 263611159682095890910790980314698591802_u128 * 333363441825697068307682151550907009487_u128;
_25.fld3 = core::ptr::addr_of_mut!((*_3));
Call((*_10) = fn15(Move(_3), _21, _13, Move(_10), Move(_1)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_2 = _21 - _21;
_17 = _16.3;
_32 = _25.fld2;
_26 = core::ptr::addr_of_mut!(_17);
_5 = &mut _14;
(*_26) = _16.3;
(*_5) = _8 * _8;
(*_5) = _8 + _8;
_33 = _25.fld0 as f32;
_29 = (*_5);
(*_5) = -_8;
_28 = 193764532149302326299098523754699996391_u128;
_2 = !_21;
(*_5) = _29 ^ _29;
(*_5) = _29 & _29;
(*_5) = _29;
(*_26) = _16.3;
_9 = 121960320639815789614219729396782250908_i128;
_28 = !47703651482561800509316837793986232795_u128;
(*_5) = _29;
(*_26) = _16.3;
(*_26) = _16.3;
_18 = &mut _25.fld0;
(*_26) = _16.3;
_27 = (*_18) & (*_18);
(*_5) = _8 + _29;
(*_5) = _29;
Goto(bb25)
}
bb25 = {
(*_26) = _16.3;
(*_26) = _16.3;
_8 = (*_5);
(*_18) = _27;
match _9 {
0 => bb26,
1 => bb27,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
121960320639815789614219729396782250908 => bb34,
_ => bb33
}
}
bb26 = {
Return()
}
bb27 = {
(*_3) = 2545109717_u32 * 1751029052_u32;
_25.fld0 = 34084_u16 & 64797_u16;
(*_3) = _16.3 as u32;
(*_3) = 2197406141_u32 | 1177273616_u32;
_26 = core::ptr::addr_of_mut!(_16.3);
_10 = core::ptr::addr_of!(_28);
(*_10) = 329614840910170018566869893859585831098_u128 * 282217041945138893702754547902486025890_u128;
(*_10) = !146789313697147592266640071744966633355_u128;
(*_10) = 12_i8 as u128;
(*_10) = 164055702696084290599278201422487709711_u128;
(*_3) = _15 as u32;
(*_26) = _17;
(*_3) = 1245167541_u32 >> _14;
_1 = Move(_26);
(*_10) = 229797398408156645908132184959192615277_u128 + 157182762450919871367128748993673848978_u128;
(*_3) = (-129811894387694610_i64) as u32;
(*_10) = 62_i8 as u128;
(*_10) = !245915364615860234898261959093655971695_u128;
(*_10) = 263611159682095890910790980314698591802_u128 * 333363441825697068307682151550907009487_u128;
_25.fld3 = core::ptr::addr_of_mut!((*_3));
Call((*_10) = fn15(Move(_3), _21, _13, Move(_10), Move(_1)), ReturnTo(bb24), UnwindUnreachable())
}
bb28 = {
(*_5) = _8;
_13 = _2 | _2;
_14 = (*_5) | (*_5);
(*_5) = 210014075082251877994316946848216691163_u128 as isize;
(*_5) = !_14;
(*_5) = _14 & _8;
_13 = _2;
_16.3 = '\u{4ea2f}';
_16.1 = [24_u8,170_u8];
(*_5) = 17679776367926510271_u64 as isize;
(*_5) = 4773565934796495322_i64 as isize;
_15 = _2 << _14;
(*_5) = _15 as isize;
_20 = -(*_5);
match _9 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
291281406679989665358319898757518689995 => bb16,
_ => bb15
}
}
bb29 = {
Return()
}
bb30 = {
Return()
}
bb31 = {
(*_5) = _8;
_13 = _2 | _2;
_14 = (*_5) | (*_5);
(*_5) = 210014075082251877994316946848216691163_u128 as isize;
(*_5) = !_14;
(*_5) = _14 & _8;
_13 = _2;
_16.3 = '\u{4ea2f}';
_16.1 = [24_u8,170_u8];
(*_5) = 17679776367926510271_u64 as isize;
(*_5) = 4773565934796495322_i64 as isize;
_15 = _2 << _14;
(*_5) = _15 as isize;
_20 = -(*_5);
match _9 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
291281406679989665358319898757518689995 => bb16,
_ => bb15
}
}
bb32 = {
(*_5) = _8 & _8;
(*_5) = (-118_i8) as isize;
(*_5) = !_8;
(*_5) = _8 & _8;
_13 = _2 | _2;
(*_5) = 159223137102859098858409482718504899910_u128 as isize;
(*_5) = _8 << _8;
(*_5) = _8 * _8;
(*_5) = _8 << _13;
(*_5) = _8 | _8;
(*_5) = _8 + _8;
Goto(bb11)
}
bb33 = {
_13 = (-23435_i16) as usize;
_8 = (*_5) >> (*_5);
(*_5) = !_8;
_16.2 = &mut (*_5);
_20 = _8;
Goto(bb18)
}
bb34 = {
_13 = (*_5) as usize;
(*_18) = !_27;
(*_18) = _27 * _27;
(*_26) = _16.3;
(*_18) = !_27;
(*_26) = _16.3;
(*_5) = _28 as isize;
_10 = core::ptr::addr_of!(_28);
_13 = _2;
(*_5) = _8;
_21 = 2242686386_u32 as usize;
(*_26) = _16.3;
(*_5) = _15 as isize;
(*_18) = _27 - _27;
Goto(bb35)
}
bb35 = {
(*_5) = _8 >> _29;
(*_10) = 178121806953815622220761955642841292069_u128 * 201835063768520684503024515906174323982_u128;
(*_10) = (*_5) as u128;
(*_26) = _16.3;
(*_18) = _27;
(*_10) = !43894238949803363848645821117429630669_u128;
(*_5) = _29 + _29;
(*_26) = _16.3;
(*_26) = _16.3;
(*_18) = _27 >> _13;
(*_26) = _16.3;
(*_10) = 268399216536733452951000408048491952223_u128;
(*_10) = 112866338279196245584861594689605611182_u128 | 269255795019344383896581543652829805036_u128;
(*_5) = _8 << _29;
(*_5) = _8 | _29;
(*_10) = !9054353892818625171399470719611908239_u128;
(*_10) = 165762183764924615928882806467511109957_u128 - 327630199184900607922752039316812629350_u128;
(*_10) = !184946332547899711294210118460450714248_u128;
(*_5) = (*_10) as isize;
(*_18) = _27;
(*_26) = _16.3;
_9 = -152879829588708181547316766501713566643_i128;
_33 = _24;
_38 = (*_26);
(*_5) = _15 as isize;
_26 = core::ptr::addr_of_mut!((*_26));
(*_18) = _27;
Goto(bb36)
}
bb36 = {
(*_10) = 125581269823691327077012316048367516717_u128 ^ 45176053320478819673666541095485314382_u128;
(*_10) = 169190614288099544468437110624443365404_u128;
(*_26) = _38;
Goto(bb37)
}
bb37 = {
(*_26) = _16.3;
(*_10) = (-13917_i16) as u128;
(*_10) = 285888790176338622011394671196254939507_u128 * 252905646655566908832269418380613262478_u128;
(*_10) = 315976383486260478036390645342236695953_u128 | 240715362715977577710999608623787895215_u128;
_16.2 = &mut (*_5);
_26 = core::ptr::addr_of_mut!((*_26));
(*_10) = _15 as u128;
_28 = 164256942760190041701897326086881236341_u128 | 335738739825885483714366858614154490640_u128;
_42 = [6239299079047700944_i64,5828904610726624461_i64,6787185289527786034_i64];
_41 = &mut _26;
(*_41) = core::ptr::addr_of_mut!(_17);
(*_18) = !_27;
_33 = _24 * _32;
(*_41) = core::ptr::addr_of_mut!(_16.3);
_9 = (-37974569549978322332118778695073225507_i128);
match _9 {
0 => bb20,
1 => bb38,
2 => bb39,
3 => bb40,
302307797370960141131255828736694985949 => bb42,
_ => bb41
}
}
bb38 = {
Return()
}
bb39 = {
Return()
}
bb40 = {
_16.3 = _17;
_16.0 = [_16.3,_17,_16.3,_16.3];
_24 = _25.fld2 - _25.fld2;
_21 = _13;
_25.fld4 = _17 as usize;
_25.fld3 = core::ptr::addr_of_mut!(_25.fld1);
_16.0 = [_17,_17,_17,_16.3];
_13 = _2;
_22 = [_23,_23];
_21 = _15 - _15;
_25.fld3 = core::ptr::addr_of_mut!(_25.fld1);
_3 = core::ptr::addr_of_mut!(_25.fld1);
(*_3) = 4674042_u32 ^ 1546192987_u32;
(*_3) = 681455013_u32;
match (*_3) {
0 => bb21,
681455013 => bb23,
_ => bb22
}
}
bb41 = {
_13 = (-23435_i16) as usize;
_8 = (*_5) >> (*_5);
(*_5) = !_8;
_16.2 = &mut (*_5);
_20 = _8;
Goto(bb18)
}
bb42 = {
_46 = (-17551_i16) as isize;
(*_41) = core::ptr::addr_of_mut!(_38);
(*_18) = _27;
(*_41) = core::ptr::addr_of_mut!(_38);
(*_41) = core::ptr::addr_of_mut!(_16.3);
_5 = &mut _8;
(*_41) = core::ptr::addr_of_mut!(_17);
(*_5) = _29 + _29;
match _9 {
0 => bb43,
302307797370960141131255828736694985949 => bb45,
_ => bb44
}
}
bb43 = {
(*_26) = _16.3;
(*_10) = (-13917_i16) as u128;
(*_10) = 285888790176338622011394671196254939507_u128 * 252905646655566908832269418380613262478_u128;
(*_10) = 315976383486260478036390645342236695953_u128 | 240715362715977577710999608623787895215_u128;
_16.2 = &mut (*_5);
_26 = core::ptr::addr_of_mut!((*_26));
(*_10) = _15 as u128;
_28 = 164256942760190041701897326086881236341_u128 | 335738739825885483714366858614154490640_u128;
_42 = [6239299079047700944_i64,5828904610726624461_i64,6787185289527786034_i64];
_41 = &mut _26;
(*_41) = core::ptr::addr_of_mut!(_17);
(*_18) = !_27;
_33 = _24 * _32;
(*_41) = core::ptr::addr_of_mut!(_16.3);
_9 = (-37974569549978322332118778695073225507_i128);
match _9 {
0 => bb20,
1 => bb38,
2 => bb39,
3 => bb40,
302307797370960141131255828736694985949 => bb42,
_ => bb41
}
}
bb44 = {
Return()
}
bb45 = {
(*_10) = 189303441840992478041651281834460192486_u128;
(*_18) = !_27;
(*_41) = core::ptr::addr_of_mut!(_38);
_48 = &mut _10;
_5 = &mut _46;
(*_18) = _27;
_35 = _9 * _9;
_28 = !277033375077038291982419412235200245197_u128;
(*_5) = _29 << _13;
(*_18) = _27 & _27;
(*_18) = _27 ^ _27;
_1 = core::ptr::addr_of_mut!(_47);
_9 = (-1918990506_i32) as i128;
(*_41) = core::ptr::addr_of_mut!((*_1));
(*_18) = _9 as u16;
(*_1) = _17;
(*_1) = _38;
_33 = _28 as f32;
Goto(bb46)
}
bb46 = {
_28 = 237142494942336573073663730184505481826_u128 * 278490982696261344722972130219880736532_u128;
_36 = _35 >> _13;
(*_48) = core::ptr::addr_of!(_28);
_40 = _28 << (*_5);
(*_5) = _29 - _29;
(*_18) = !_27;
(*_5) = _29 - _29;
(*_18) = _27;
(*_1) = _16.3;
(*_5) = -_29;
Goto(bb47)
}
bb47 = {
(*_1) = _17;
Goto(bb48)
}
bb48 = {
(*_1) = _16.3;
Call((*_5) = core::intrinsics::bswap(_29), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
(*_5) = _29 >> (*_18);
(*_1) = _16.3;
_18 = &mut _27;
_1 = core::ptr::addr_of_mut!((*_1));
(*_48) = core::ptr::addr_of!(_40);
_33 = _24 * _24;
(*_5) = _29 * _29;
(*_1) = _16.3;
(*_48) = core::ptr::addr_of!(_28);
(*_48) = core::ptr::addr_of!(_28);
(*_48) = core::ptr::addr_of!(_28);
(*_48) = core::ptr::addr_of!(_40);
(*_1) = _16.3;
_49 = &mut (*_41);
_15 = _2 << (*_5);
(*_18) = 55441_u16 + 7768_u16;
(*_49) = core::ptr::addr_of_mut!(_16.3);
(*_1) = _17;
(*_1) = _17;
(*_1) = _38;
Goto(bb50)
}
bb50 = {
(*_1) = _16.3;
(*_1) = _17;
(*_49) = core::ptr::addr_of_mut!((*_1));
(*_18) = 18173_u16;
(*_5) = _29 - _29;
(*_18) = 18262_u16 >> (*_5);
(*_18) = 60617_u16;
(*_48) = core::ptr::addr_of!(_28);
_5 = Move(_16.2);
_54 = !4718157486386671522_i64;
RET = core::ptr::addr_of!(_55);
(*RET) = -(-7378_i16);
(*_18) = 24587_u16 & 13701_u16;
(*_1) = _38;
(*_49) = core::ptr::addr_of_mut!((*_1));
(*_48) = core::ptr::addr_of!(_28);
(*RET) = (-2108884155_i32) as i16;
(*_1) = _17;
(*_48) = core::ptr::addr_of!(_28);
Goto(bb51)
}
bb51 = {
Call(_58 = dump_var(Move(_14), Move(_55), Move(_4), Move(_8)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_58 = dump_var(Move(_40), Move(_47), Move(_22), Move(_15)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_58 = dump_var(Move(_54), Move(_9), Move(_36), Move(_35)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *mut u32,mut _2: usize,mut _3: usize,mut _4: *const u128,mut _5: *mut char) -> u128 {
mir! {
type RET = u128;
let _6: f64;
let _7: (&'static mut &'static f64, i16, usize, [char; 4]);
let _8: *const u128;
let _9: f64;
let _10: isize;
let _11: bool;
let _12: f64;
let _13: *const &'static mut &'static f64;
let _14: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _15: (u128, usize, [i32; 1]);
let _16: bool;
let _17: bool;
let _18: *mut *mut &'static f64;
let _19: u16;
let _20: ([char; 4], u8, [char; 4]);
let _21: isize;
let _22: i8;
let _23: u8;
let _24: u16;
let _25: &'static mut *mut char;
let _26: *const &'static mut &'static f64;
let _27: f32;
let _28: f64;
let _29: char;
let _30: bool;
let _31: *const &'static bool;
let _32: ([char; 4], [u8; 2], &'static mut isize, char);
let _33: &'static mut &'static f64;
let _34: &'static mut *const u128;
let _35: [usize; 7];
let _36: [u64; 7];
let _37: [u64; 7];
let _38: bool;
let _39: f64;
let _40: isize;
let _41: [i128; 2];
let _42: &'static mut u8;
let _43: ();
let _44: ();
{
_2 = 2044976833_i32 as usize;
_3 = _2 | _2;
RET = 269464484226211852004102069474131140528_u128 << _3;
_7.1 = -(-8834_i16);
_6 = 0_u8 as f64;
_4 = core::ptr::addr_of!(RET);
(*_4) = 112420083093397539942914689808012701212_u128 - 301009543906312328712112398380263006957_u128;
(*_4) = !40952452119702959054058882567170237608_u128;
(*_4) = '\u{d7a1d}' as u128;
(*_4) = !139224756316662084523719063745872550190_u128;
(*_4) = 155689978176293734740299663454431102635_u128;
_8 = core::ptr::addr_of!((*_4));
(*_8) = !140447123453109876039192006419667036209_u128;
_7.3 = ['\u{d085}','\u{b342b}','\u{f2640}','\u{d3f9c}'];
_8 = Move(_4);
_9 = _6;
RET = 169501449576903209446538999456459700740_u128;
_9 = _6;
_6 = _9 + _9;
_9 = _6 + _6;
_7.2 = _3 >> _3;
Goto(bb1)
}
bb1 = {
_4 = core::ptr::addr_of!(RET);
_8 = core::ptr::addr_of!((*_4));
(*_8) = _3 as u128;
(*_8) = 337792696347417099264999159015128154232_u128;
(*_8) = 188963888871199076296998609637842855429_u128 >> _2;
(*_4) = 229956874600172488178750731193546746522_u128;
(*_4) = 58634972747725448775178972218544787649_u128 + 199227132112003873399817310593145811732_u128;
_2 = 83990573006301482659586802663096241855_i128 as usize;
(*_4) = 112825868589602734077670731478002993698_u128 & 72446607588747241116345880215182636478_u128;
(*_4) = 67496949603486073308015425314240999443_u128 | 63253749774037583138292638653128692917_u128;
_7.1 = 9533_i16;
(*_4) = '\u{84c0b}' as u128;
_11 = false;
match _7.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9533 => bb10,
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
(*_4) = 41355707413578433514040373480229874516_u128;
_3 = !_7.2;
_12 = -_9;
_7.3 = ['\u{cfd5c}','\u{1b29a}','\u{2719f}','\u{b1386}'];
_14.2 = &_9;
(*_4) = 256564797799695845757962840317875297593_u128 + 223636460565123329880149507261494908028_u128;
_10 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
(*_4) = 47397297008927878296118859352422709940_u128 & 68286019050560919301338058896399525517_u128;
RET = 115303609669241904737918958177695793888_u128;
RET = !226722944823562254568318187447372656351_u128;
(*_4) = 57225098393916779266130740246121851736_u128 - 241097014413683577608224725030925235592_u128;
_14.1.1 = [1207012524_u32,1363379361_u32,1256056172_u32,1729354929_u32,2656559043_u32,1130194682_u32,4280442714_u32];
_3 = _7.2 >> (*_4);
(*_4) = 223660708796747518215027774048326159552_u128;
(*_4) = (-7598594296951866793_i64) as u128;
(*_4) = 154000072180084896451129340594890192473_u128 * 103652665393043446367606568457145589896_u128;
_2 = _3 ^ _3;
Goto(bb11)
}
bb11 = {
_14.1.0 = &_6;
_2 = _3;
_12 = _6;
_15.1 = !_2;
(*_4) = !91060101523266695245301604215415068430_u128;
_7.2 = 2741375679_u32 as usize;
_8 = core::ptr::addr_of!((*_4));
_10 = (-9223372036854775808_isize) - (-111_isize);
match _7.1 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
9533 => bb12,
_ => bb10
}
}
bb12 = {
(*_8) = !235168420643486555344878750080857982917_u128;
(*_8) = !281399196074723738859882221227873268581_u128;
_7.0 = &mut _14.2;
_15.1 = !_7.2;
(*_8) = 239476334222730986071039995907347477854_u128 + 231732508094461195992976816326238506800_u128;
_10 = 9223372036854775807_isize ^ 15_isize;
Goto(bb13)
}
bb13 = {
(*_4) = 322372890631145963546638134148106610911_u128;
_15.1 = !_2;
(*_4) = _2 as u128;
_2 = !_15.1;
(*_4) = 187876214150475392200917621390161194445_u128 & 326462579691933779426034411461046788974_u128;
_15.0 = (*_4);
_15.2 = [(-1939375702_i32)];
_11 = (*_4) < (*_4);
(*_4) = _15.0;
_15.0 = RET >> (*_4);
_17 = (*_4) <= (*_4);
_16 = (*_4) > (*_4);
_20 = (_7.3, 227_u8, _7.3);
_21 = _10 | _10;
_9 = _12 * _12;
_13 = core::ptr::addr_of!(_7.0);
_3 = !_15.1;
(*_4) = _15.0;
_15.0 = (*_4) >> _7.2;
_20.1 = (-1850723293_i32) as u8;
_23 = _20.1 + _20.1;
Goto(bb14)
}
bb14 = {
(*_4) = 57357_u16 as u128;
_19 = 26584_u16 - 45517_u16;
(*_4) = _3 as u128;
(*_4) = _15.0;
(*_4) = !_15.0;
(*_4) = _15.0 * _15.0;
_7.3 = _20.0;
(*_4) = !_15.0;
_4 = Move(_8);
_15.2 = [(-1371414387_i32)];
_8 = core::ptr::addr_of!(_15.0);
_6 = _12;
(*_8) = RET | RET;
(*_8) = _12 as u128;
(*_8) = RET;
(*_8) = 3219217772_u32 as u128;
RET = (*_8) >> (*_8);
(*_8) = RET;
_6 = (*_8) as f64;
match _7.1 {
0 => bb8,
1 => bb10,
2 => bb12,
9533 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
(*_8) = RET | RET;
_20.1 = _23;
(*_8) = _3 as u128;
(*_8) = _19 as u128;
match _7.1 {
0 => bb17,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
9533 => bb25,
_ => bb24
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
Return()
}
bb21 = {
Return()
}
bb22 = {
_4 = core::ptr::addr_of!(RET);
_8 = core::ptr::addr_of!((*_4));
(*_8) = _3 as u128;
(*_8) = 337792696347417099264999159015128154232_u128;
(*_8) = 188963888871199076296998609637842855429_u128 >> _2;
(*_4) = 229956874600172488178750731193546746522_u128;
(*_4) = 58634972747725448775178972218544787649_u128 + 199227132112003873399817310593145811732_u128;
_2 = 83990573006301482659586802663096241855_i128 as usize;
(*_4) = 112825868589602734077670731478002993698_u128 & 72446607588747241116345880215182636478_u128;
(*_4) = 67496949603486073308015425314240999443_u128 | 63253749774037583138292638653128692917_u128;
_7.1 = 9533_i16;
(*_4) = '\u{84c0b}' as u128;
_11 = false;
match _7.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9533 => bb10,
_ => bb9
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
(*_8) = RET & RET;
_22 = !89_i8;
_27 = _7.1 as f32;
(*_8) = RET >> _15.1;
_20.2 = ['\u{a4d3c}','\u{baf67}','\u{3f0dc}','\u{8ec25}'];
(*_8) = RET >> _7.2;
_24 = _19;
_2 = _27 as usize;
RET = _27 as u128;
_5 = core::ptr::addr_of_mut!(_29);
(*_8) = _7.1 as u128;
(*_5) = '\u{fc27f}';
Goto(bb26)
}
bb26 = {
(*_8) = RET << RET;
(*_5) = '\u{596a4}';
_9 = (-991415153_i32) as f64;
_25 = &mut _5;
_7.2 = (-1310201699_i32) as usize;
_7.2 = _23 as usize;
_20.1 = 3667828935395733215_i64 as u8;
_13 = core::ptr::addr_of!((*_13));
_27 = 8939726097544925240_i64 as f32;
_3 = _7.2 ^ _15.1;
(*_25) = core::ptr::addr_of_mut!(_29);
_19 = _24;
(*_8) = RET | RET;
_32.2 = &mut _10;
(*_8) = !RET;
(*_8) = RET;
(*_8) = RET << _3;
_29 = '\u{96db6}';
(*_8) = RET * RET;
(*_25) = core::ptr::addr_of_mut!(_29);
Goto(bb27)
}
bb27 = {
_32.3 = _29;
_28 = -_6;
RET = (*_8) ^ (*_8);
_2 = _3 * _3;
(*_8) = _15.1 as u128;
_8 = core::ptr::addr_of!((*_8));
(*_25) = core::ptr::addr_of_mut!(_29);
_32.1 = [_20.1,_23];
RET = !(*_8);
(*_25) = core::ptr::addr_of_mut!(_29);
(*_8) = _27 as u128;
_20.0 = [_32.3,_29,_32.3,_32.3];
_34 = &mut _4;
_30 = !_17;
_35 = [_2,_15.1,_2,_7.2,_2,_15.1,_3];
(*_25) = core::ptr::addr_of_mut!(_29);
Call(_3 = fn16(Move(_13), Move((*_13)), Move((*_34)), Move((*_25)), _30, _32.1, Move(_1), Move(_8)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_39 = _12 * _28;
_9 = (-734990669128346557_i64) as f64;
_21 = -9223372036854775807_isize;
_8 = core::ptr::addr_of!(RET);
(*_8) = _15.0 ^ _15.0;
(*_8) = _15.0 ^ _15.0;
match _7.1 {
0 => bb20,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
9533 => bb35,
_ => bb34
}
}
bb29 = {
Return()
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
_4 = core::ptr::addr_of!(RET);
_8 = core::ptr::addr_of!((*_4));
(*_8) = _3 as u128;
(*_8) = 337792696347417099264999159015128154232_u128;
(*_8) = 188963888871199076296998609637842855429_u128 >> _2;
(*_4) = 229956874600172488178750731193546746522_u128;
(*_4) = 58634972747725448775178972218544787649_u128 + 199227132112003873399817310593145811732_u128;
_2 = 83990573006301482659586802663096241855_i128 as usize;
(*_4) = 112825868589602734077670731478002993698_u128 & 72446607588747241116345880215182636478_u128;
(*_4) = 67496949603486073308015425314240999443_u128 | 63253749774037583138292638653128692917_u128;
_7.1 = 9533_i16;
(*_4) = '\u{84c0b}' as u128;
_11 = false;
match _7.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9533 => bb10,
_ => bb9
}
}
bb35 = {
(*_8) = _24 as u128;
(*_8) = _15.0;
_15.2 = [1977451462_i32];
Call((*_8) = core::intrinsics::bswap(_15.0), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
(*_8) = _21 as u128;
(*_8) = _15.0 >> _2;
_32.0 = [_29,_32.3,_29,_32.3];
_22 = 118_i8 | 124_i8;
_20.0 = [_32.3,_29,_29,_32.3];
(*_8) = !_15.0;
_20.0 = [_32.3,_29,_32.3,_32.3];
(*_8) = _15.0;
_20.1 = !_23;
_20.2 = _20.0;
_6 = _12 + _39;
(*_8) = !_15.0;
_15.1 = _2;
(*_8) = _15.0 << _15.1;
_42 = &mut _23;
_7.3 = _20.0;
Goto(bb37)
}
bb37 = {
Call(_43 = dump_var(Move(_10), Move(_3), Move(_30), Move(_19)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_43 = dump_var(Move(_20), Move(_23), Move(_16), Move(_15)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const &'static mut &'static f64,mut _2: &'static mut &'static f64,mut _3: *const u128,mut _4: *mut char,mut _5: bool,mut _6: [u8; 2],mut _7: *mut u32,mut _8: *const u128) -> usize {
mir! {
type RET = usize;
let _9: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _10: u32;
let _11: f64;
let _12: f64;
let _13: bool;
let _14: bool;
let _15: *const u128;
let _16: isize;
let _17: char;
let _18: &'static mut *mut i16;
let _19: Adt55;
let _20: &'static mut isize;
let _21: *mut u32;
let _22: f32;
let _23: usize;
let _24: &'static mut &'static f64;
let _25: *const &'static mut &'static f64;
let _26: [u128; 2];
let _27: f32;
let _28: *mut u32;
let _29: *mut *mut i16;
let _30: usize;
let _31: isize;
let _32: *const &'static mut &'static f64;
let _33: isize;
let _34: f64;
let _35: &'static mut *const i16;
let _36: u16;
let _37: Adt75;
let _38: f64;
let _39: &'static mut u16;
let _40: f32;
let _41: f32;
let _42: i64;
let _43: u128;
let _44: isize;
let _45: i8;
let _46: u128;
let _47: i64;
let _48: (*mut &'static f64, &'static mut isize);
let _49: usize;
let _50: &'static usize;
let _51: usize;
let _52: isize;
let _53: Adt43;
let _54: isize;
let _55: &'static f32;
let _56: &'static usize;
let _57: u32;
let _58: &'static &'static mut *mut char;
let _59: &'static mut *const u128;
let _60: f32;
let _61: char;
let _62: u32;
let _63: u32;
let _64: f32;
let _65: char;
let _66: isize;
let _67: Adt23;
let _68: u8;
let _69: &'static mut *mut i16;
let _70: &'static mut *const u128;
let _71: f32;
let _72: char;
let _73: *const &'static bool;
let _74: f64;
let _75: *const u128;
let _76: isize;
let _77: (&'static mut &'static f64, i16, usize, [char; 4]);
let _78: *mut u32;
let _79: (*mut &'static f64, &'static mut isize);
let _80: *mut *mut &'static f64;
let _81: Adt55;
let _82: isize;
let _83: f32;
let _84: bool;
let _85: u32;
let _86: &'static mut &'static f64;
let _87: *mut i16;
let _88: [i64; 3];
let _89: &'static mut &'static f64;
let _90: f64;
let _91: i16;
let _92: f32;
let _93: u8;
let _94: [i128; 2];
let _95: char;
let _96: u8;
let _97: Adt28;
let _98: Adt43;
let _99: i16;
let _100: u32;
let _101: i64;
let _102: (*const i16,);
let _103: char;
let _104: isize;
let _105: isize;
let _106: ();
let _107: ();
{
RET = 1_usize - 6_usize;
_6 = [160_u8,118_u8];
RET = 162_u8 as usize;
RET = 11132752779434888733_usize * 7_usize;
_3 = Move(_8);
_8 = Move(_3);
_3 = Move(_8);
_5 = false;
RET = !11182653017058563226_usize;
RET = 10589976377256321358_usize & 446570070279251685_usize;
_5 = RET == RET;
RET = 1805238118987556172_usize;
_8 = Move(_3);
_3 = Move(_8);
_8 = Move(_3);
_9.2.1 = 262433127252594866384560297288517860469_u128 | 87283109379772702795977219634000998740_u128;
_9.0 = !52822_u16;
_3 = core::ptr::addr_of!(_9.2.1);
(*_3) = (-7142378299824267843_i64) as u128;
(*_3) = 21549835081797444788038284799575801318_u128 & 52996102492628592979769524750010000317_u128;
(*_3) = 278295604516569983229758662210797065598_u128 - 41636281098250128142315334785788179852_u128;
Goto(bb1)
}
bb1 = {
(*_3) = 227156505733119936252728905811159227825_u128 - 142222106096972863992154226459282866670_u128;
_9.2.1 = !30090230694997678110184930082491224840_u128;
(*_3) = 98532762314881522729927258316032804250_u128 + 40189431201684665785960954581875728801_u128;
(*_3) = 259303053707161664942675084147608983136_u128 & 14925933155805069640486201558924034201_u128;
(*_3) = 123776589194404053120281237458162781346_u128;
_9.0 = 29776_u16 ^ 59888_u16;
(*_3) = 331266059685941020543173349893529038798_u128 * 48213599666913713106412212771770080224_u128;
RET = 9223372036854775807_isize as usize;
(*_3) = 183180720155221875503049345228193604911_u128;
(*_3) = 148316457772322120906742896914998409146_u128 ^ 104390313346453132891167224331722852168_u128;
(*_3) = 107897478094515560333843634151006159239_u128 << RET;
_5 = true;
(*_3) = 35279953004461035752172438501749393432_u128 | 26729502073224730703555844862410693078_u128;
(*_3) = 161384032591895636254941669665545934336_u128;
(*_3) = 80386038473102069560152994563580279896_u128 - 289269168722953141022703553792702544439_u128;
RET = 5298441983860587885_usize;
(*_3) = _9.0 as u128;
(*_3) = !15170417218377549744340510084749632962_u128;
(*_3) = 228150860326761594664705112389932747626_u128;
(*_3) = 116261346319208853582371808707768741785_u128 - 20373054500706192648085636061017026486_u128;
_6 = [206_u8,78_u8];
(*_3) = 188675462396358199996625137902209636157_u128 << _9.0;
(*_3) = !339656433363946658708027115598481653406_u128;
RET = 5_usize * 7402728958086811995_usize;
_10 = !3256686755_u32;
_11 = 31122_i16 as f64;
_9.2.0 = Adt23::Variant0 { fld0: _10,fld1: _11,fld2: (-9223372036854775808_isize) };
place!(Field::<u32>(Variant(_9.2.0, 0), 0)) = (*_3) as u32;
(*_3) = (-102367145395758287835424186572230484189_i128) as u128;
_6 = [248_u8,228_u8];
Goto(bb2)
}
bb2 = {
_5 = (*_3) >= (*_3);
(*_3) = 189474351180828298196363474848574376495_u128 + 272098196297663596034677204038045960476_u128;
(*_3) = !130610137494335478370385252816759356539_u128;
_7 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9.2.0, 0), 0)));
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = 8507607489670297852_u64 as u32;
Call((*_3) = fn17(Move(_4), Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_3) = 285168570789889139598936509302220520226_u128 | 300293528225425637282853643184214891659_u128;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 233440285035008844455601990691063236750_u128;
place!(Field::<isize>(Variant(_9.2.0, 0), 2)) = _11 as isize;
Goto(bb4)
}
bb4 = {
_9.0 = 49085_u16 - 29047_u16;
(*_3) = 234742854385077659709790588680301680293_u128;
_11 = Field::<f64>(Variant(_9.2.0, 0), 1) - Field::<f64>(Variant(_9.2.0, 0), 1);
(*_7) = _10;
(*_7) = _10 ^ _10;
(*_3) = 313076184828881048555957122797829269350_u128 + 247855867178091975647060463431114475378_u128;
(*_3) = 107448009723745686051015219842236851320_u128;
match (*_3) {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
107448009723745686051015219842236851320 => bb9,
_ => bb8
}
}
bb5 = {
(*_3) = 285168570789889139598936509302220520226_u128 | 300293528225425637282853643184214891659_u128;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 233440285035008844455601990691063236750_u128;
place!(Field::<isize>(Variant(_9.2.0, 0), 2)) = _11 as isize;
Goto(bb4)
}
bb6 = {
_5 = (*_3) >= (*_3);
(*_3) = 189474351180828298196363474848574376495_u128 + 272098196297663596034677204038045960476_u128;
(*_3) = !130610137494335478370385252816759356539_u128;
_7 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9.2.0, 0), 0)));
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = 8507607489670297852_u64 as u32;
Call((*_3) = fn17(Move(_4), Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
(*_3) = 227156505733119936252728905811159227825_u128 - 142222106096972863992154226459282866670_u128;
_9.2.1 = !30090230694997678110184930082491224840_u128;
(*_3) = 98532762314881522729927258316032804250_u128 + 40189431201684665785960954581875728801_u128;
(*_3) = 259303053707161664942675084147608983136_u128 & 14925933155805069640486201558924034201_u128;
(*_3) = 123776589194404053120281237458162781346_u128;
_9.0 = 29776_u16 ^ 59888_u16;
(*_3) = 331266059685941020543173349893529038798_u128 * 48213599666913713106412212771770080224_u128;
RET = 9223372036854775807_isize as usize;
(*_3) = 183180720155221875503049345228193604911_u128;
(*_3) = 148316457772322120906742896914998409146_u128 ^ 104390313346453132891167224331722852168_u128;
(*_3) = 107897478094515560333843634151006159239_u128 << RET;
_5 = true;
(*_3) = 35279953004461035752172438501749393432_u128 | 26729502073224730703555844862410693078_u128;
(*_3) = 161384032591895636254941669665545934336_u128;
(*_3) = 80386038473102069560152994563580279896_u128 - 289269168722953141022703553792702544439_u128;
RET = 5298441983860587885_usize;
(*_3) = _9.0 as u128;
(*_3) = !15170417218377549744340510084749632962_u128;
(*_3) = 228150860326761594664705112389932747626_u128;
(*_3) = 116261346319208853582371808707768741785_u128 - 20373054500706192648085636061017026486_u128;
_6 = [206_u8,78_u8];
(*_3) = 188675462396358199996625137902209636157_u128 << _9.0;
(*_3) = !339656433363946658708027115598481653406_u128;
RET = 5_usize * 7402728958086811995_usize;
_10 = !3256686755_u32;
_11 = 31122_i16 as f64;
_9.2.0 = Adt23::Variant0 { fld0: _10,fld1: _11,fld2: (-9223372036854775808_isize) };
place!(Field::<u32>(Variant(_9.2.0, 0), 0)) = (*_3) as u32;
(*_3) = (-102367145395758287835424186572230484189_i128) as u128;
_6 = [248_u8,228_u8];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_6 = [223_u8,119_u8];
(*_3) = !125861356111713674486213007679247323317_u128;
(*_3) = Field::<isize>(Variant(_9.2.0, 0), 2) as u128;
(*_3) = 18532010970692589330473522607424433543_u128;
(*_3) = 224456185448455476792921061150057360068_u128;
_8 = core::ptr::addr_of!((*_3));
_7 = core::ptr::addr_of_mut!((*_7));
(*_3) = 13819405156886412098027274669967233632_u128 * 99803331881876004170158840568261601518_u128;
(*_7) = _10;
(*_7) = _10 - _10;
(*_7) = _10;
_13 = !_5;
(*_3) = !119215375727266710001296217184830397601_u128;
Goto(bb10)
}
bb10 = {
(*_3) = 46595293123745158514789270016462378945_u128 + 41553003149848661691291046712432487246_u128;
(*_7) = _10 << (*_3);
(*_7) = _10 - _10;
_14 = _13 | _5;
(*_7) = _10;
(*_3) = 167057741024550906396758249476962219418_i128 as u128;
place!(Field::<f64>(Variant(_9.2.0, 0), 1)) = _11 * _11;
(*_7) = _10;
_6 = [239_u8,211_u8];
(*_3) = _5 as u128;
(*_3) = 257784014195056392737202076583466924972_u128 << (*_7);
(*_7) = Field::<isize>(Variant(_9.2.0, 0), 2) as u32;
(*_7) = _10;
_9.2.0 = Adt23::Variant0 { fld0: _10,fld1: _11,fld2: 9223372036854775807_isize };
_9.2.0 = Adt23::Variant0 { fld0: _10,fld1: _11,fld2: 9223372036854775807_isize };
Goto(bb11)
}
bb11 = {
(*_3) = (-1326691962_i32) as u128;
(*_3) = !226083014183447045141668316437786064573_u128;
Goto(bb12)
}
bb12 = {
(*_3) = !214196117199574364102612628168539270226_u128;
(*_3) = 139650680042428574111657790447365319995_u128 * 107173707997260417413937116010894484403_u128;
(*_3) = 61696551875352920061245397796005054329_u128 ^ 78162893505689562098405268128452064945_u128;
_9.2.0 = Adt23::Variant1 { fld0: 9223372036854775807_isize };
(*_3) = (-16_i8) as u128;
_9.2.0 = Adt23::Variant2 { fld0: _9.0,fld1: (-107319313560642764481279500283945080091_i128) };
(*_3) = RET as u128;
(*_3) = 329488018998856220790801213290149836782_u128 ^ 214638759903656523103299202950591743976_u128;
(*_3) = 159320553890586047354049751422449459868_u128 ^ 311054659796919604631923259085636855294_u128;
(*_3) = !315047912010325931689237944463617309818_u128;
_17 = '\u{aab5}';
_22 = 8352356845685309410_u64 as f32;
_4 = core::ptr::addr_of_mut!(_17);
(*_3) = !258545649765403996142690056216894984219_u128;
_4 = core::ptr::addr_of_mut!((*_4));
_13 = !_14;
_16 = 14595811104013587693_u64 as isize;
(*_3) = !246176433501683088363404836855989545945_u128;
(*_4) = '\u{2c370}';
(*_3) = 239686442177214931463509700463379590358_u128;
Goto(bb13)
}
bb13 = {
_9.0 = Field::<u16>(Variant(_9.2.0, 2), 0) & Field::<u16>(Variant(_9.2.0, 2), 0);
(*_3) = 198767797305928278312533396440391105463_u128 * 257148842769577980104627384491620575419_u128;
_21 = core::ptr::addr_of_mut!(_10);
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = -52710099018582940358237625366920928536_i128;
(*_21) = !340097544_u32;
(*_21) = (-3600253944641687942_i64) as u32;
(*_21) = (*_3) as u32;
(*_21) = 3593783577_u32;
(*_4) = '\u{95ce8}';
_6 = [73_u8,191_u8];
_15 = core::ptr::addr_of!((*_3));
_4 = core::ptr::addr_of_mut!((*_4));
(*_15) = 208631775487199802437207475513845209048_u128 << Field::<i128>(Variant(_9.2.0, 2), 1);
(*_15) = 43323333168302330225646525785751038971_u128 ^ 198311888602561066457857853044117708708_u128;
(*_21) = 428282607_u32 & 1110263952_u32;
_12 = _11 - _11;
_7 = core::ptr::addr_of_mut!((*_21));
(*_3) = 166981269920951321109454393040769441701_u128;
_6 = [16_u8,72_u8];
(*_7) = 4065541016_u32 >> (*_3);
(*_3) = (*_4) as u128;
(*_21) = _16 as u32;
(*_3) = (-119_i8) as u128;
(*_4) = '\u{d0d7}';
(*_3) = !205778815397016631857899204866254102420_u128;
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = _13 as i128;
Goto(bb14)
}
bb14 = {
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = _16 as i128;
(*_3) = 76052434585673663618251562710614326678_u128 | 318900086728863416570010959116700269205_u128;
(*_4) = '\u{ff678}';
_22 = _16 as f32;
(*_3) = 288597593447800146346382405246847633229_u128 >> Field::<i128>(Variant(_9.2.0, 2), 1);
(*_4) = '\u{10c8b5}';
(*_4) = '\u{a0cd1}';
(*_4) = '\u{9711e}';
(*_21) = !3508852912_u32;
(*_4) = '\u{efda}';
Goto(bb15)
}
bb15 = {
(*_21) = 458067880_u32 | 2341676434_u32;
(*_3) = 28067051162759827556103087788221580176_u128;
_5 = _13;
_27 = _22 - _22;
(*_4) = '\u{1021a5}';
(*_4) = '\u{c65a9}';
place!(Field::<u16>(Variant(_9.2.0, 2), 0)) = !_9.0;
_17 = '\u{d424f}';
(*_4) = '\u{14d19}';
(*_3) = !243355779150983031005469316818616925041_u128;
(*_4) = '\u{d7b50}';
_5 = (*_3) <= _9.2.1;
(*_3) = !34418157858211924027895397362896938714_u128;
_28 = core::ptr::addr_of_mut!((*_21));
_9.0 = Field::<u16>(Variant(_9.2.0, 2), 0);
(*_4) = '\u{e5658}';
(*_28) = 2658729130_u32 >> (*_3);
Goto(bb16)
}
bb16 = {
(*_4) = '\u{80c0b}';
_14 = !_5;
(*_28) = !3919741495_u32;
_8 = core::ptr::addr_of!((*_3));
(*_3) = 72443380141473578799985891748982941986_u128;
(*_21) = 1030118845_u32 >> RET;
(*_21) = 3194497719_u32 >> (*_3);
_3 = core::ptr::addr_of!((*_3));
(*_4) = '\u{95426}';
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = 40554555668656110409857191267645451125_i128 ^ 92309306850799350517796394963260787512_i128;
(*_4) = '\u{8005a}';
match (*_3) {
0 => bb7,
1 => bb10,
2 => bb14,
3 => bb11,
72443380141473578799985891748982941986 => bb18,
_ => bb17
}
}
bb17 = {
(*_3) = !214196117199574364102612628168539270226_u128;
(*_3) = 139650680042428574111657790447365319995_u128 * 107173707997260417413937116010894484403_u128;
(*_3) = 61696551875352920061245397796005054329_u128 ^ 78162893505689562098405268128452064945_u128;
_9.2.0 = Adt23::Variant1 { fld0: 9223372036854775807_isize };
(*_3) = (-16_i8) as u128;
_9.2.0 = Adt23::Variant2 { fld0: _9.0,fld1: (-107319313560642764481279500283945080091_i128) };
(*_3) = RET as u128;
(*_3) = 329488018998856220790801213290149836782_u128 ^ 214638759903656523103299202950591743976_u128;
(*_3) = 159320553890586047354049751422449459868_u128 ^ 311054659796919604631923259085636855294_u128;
(*_3) = !315047912010325931689237944463617309818_u128;
_17 = '\u{aab5}';
_22 = 8352356845685309410_u64 as f32;
_4 = core::ptr::addr_of_mut!(_17);
(*_3) = !258545649765403996142690056216894984219_u128;
_4 = core::ptr::addr_of_mut!((*_4));
_13 = !_14;
_16 = 14595811104013587693_u64 as isize;
(*_3) = !246176433501683088363404836855989545945_u128;
(*_4) = '\u{2c370}';
(*_3) = 239686442177214931463509700463379590358_u128;
Goto(bb13)
}
bb18 = {
(*_3) = _9.0 as u128;
(*_3) = (*_21) as u128;
_19 = Adt55::Variant1 { fld0: Move(_4),fld1: Move(_28),fld2: _16,fld3: _9.0,fld4: (-11368_i16) };
(*_3) = !122682796144784350497930304140154481686_u128;
place!(Field::<u16>(Variant(_9.2.0, 2), 0)) = _9.0 | Field::<u16>(Variant(_19, 1), 3);
(*_21) = 151814901_u32 & 2833978292_u32;
(*_21) = 4076886477_u32 >> Field::<u16>(Variant(_9.2.0, 2), 0);
(*_21) = (*_3) as u32;
(*_3) = 16891714608916000390661959196221350184_u128 & 110755380502815558151809530895458971780_u128;
_9.2.0 = Adt23::Variant2 { fld0: _9.0,fld1: (-127095627010213511233948868228885363324_i128) };
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = 1917979491593475115_i64 as i128;
_16 = -Field::<isize>(Variant(_19, 1), 2);
_23 = RET >> (*_3);
(*_3) = 18471365733823551852728477444469341362_u128;
_6 = [182_u8,71_u8];
(*_21) = !1279161918_u32;
(*_21) = !2925655469_u32;
_4 = core::ptr::addr_of_mut!(_17);
place!(Field::<isize>(Variant(_19, 1), 2)) = _23 as isize;
(*_4) = '\u{5a258}';
_22 = -_27;
(*_4) = '\u{f7bb2}';
_30 = _23;
(*_4) = '\u{90a2}';
(*_21) = _12 as u32;
(*_21) = 1179406953_u32;
match (*_3) {
0 => bb6,
1 => bb13,
2 => bb19,
3 => bb20,
4 => bb21,
18471365733823551852728477444469341362 => bb23,
_ => bb22
}
}
bb19 = {
(*_3) = !214196117199574364102612628168539270226_u128;
(*_3) = 139650680042428574111657790447365319995_u128 * 107173707997260417413937116010894484403_u128;
(*_3) = 61696551875352920061245397796005054329_u128 ^ 78162893505689562098405268128452064945_u128;
_9.2.0 = Adt23::Variant1 { fld0: 9223372036854775807_isize };
(*_3) = (-16_i8) as u128;
_9.2.0 = Adt23::Variant2 { fld0: _9.0,fld1: (-107319313560642764481279500283945080091_i128) };
(*_3) = RET as u128;
(*_3) = 329488018998856220790801213290149836782_u128 ^ 214638759903656523103299202950591743976_u128;
(*_3) = 159320553890586047354049751422449459868_u128 ^ 311054659796919604631923259085636855294_u128;
(*_3) = !315047912010325931689237944463617309818_u128;
_17 = '\u{aab5}';
_22 = 8352356845685309410_u64 as f32;
_4 = core::ptr::addr_of_mut!(_17);
(*_3) = !258545649765403996142690056216894984219_u128;
_4 = core::ptr::addr_of_mut!((*_4));
_13 = !_14;
_16 = 14595811104013587693_u64 as isize;
(*_3) = !246176433501683088363404836855989545945_u128;
(*_4) = '\u{2c370}';
(*_3) = 239686442177214931463509700463379590358_u128;
Goto(bb13)
}
bb20 = {
_9.0 = 49085_u16 - 29047_u16;
(*_3) = 234742854385077659709790588680301680293_u128;
_11 = Field::<f64>(Variant(_9.2.0, 0), 1) - Field::<f64>(Variant(_9.2.0, 0), 1);
(*_7) = _10;
(*_7) = _10 ^ _10;
(*_3) = 313076184828881048555957122797829269350_u128 + 247855867178091975647060463431114475378_u128;
(*_3) = 107448009723745686051015219842236851320_u128;
match (*_3) {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
107448009723745686051015219842236851320 => bb9,
_ => bb8
}
}
bb21 = {
(*_3) = 285168570789889139598936509302220520226_u128 | 300293528225425637282853643184214891659_u128;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 233440285035008844455601990691063236750_u128;
place!(Field::<isize>(Variant(_9.2.0, 0), 2)) = _11 as isize;
Goto(bb4)
}
bb22 = {
(*_3) = 227156505733119936252728905811159227825_u128 - 142222106096972863992154226459282866670_u128;
_9.2.1 = !30090230694997678110184930082491224840_u128;
(*_3) = 98532762314881522729927258316032804250_u128 + 40189431201684665785960954581875728801_u128;
(*_3) = 259303053707161664942675084147608983136_u128 & 14925933155805069640486201558924034201_u128;
(*_3) = 123776589194404053120281237458162781346_u128;
_9.0 = 29776_u16 ^ 59888_u16;
(*_3) = 331266059685941020543173349893529038798_u128 * 48213599666913713106412212771770080224_u128;
RET = 9223372036854775807_isize as usize;
(*_3) = 183180720155221875503049345228193604911_u128;
(*_3) = 148316457772322120906742896914998409146_u128 ^ 104390313346453132891167224331722852168_u128;
(*_3) = 107897478094515560333843634151006159239_u128 << RET;
_5 = true;
(*_3) = 35279953004461035752172438501749393432_u128 | 26729502073224730703555844862410693078_u128;
(*_3) = 161384032591895636254941669665545934336_u128;
(*_3) = 80386038473102069560152994563580279896_u128 - 289269168722953141022703553792702544439_u128;
RET = 5298441983860587885_usize;
(*_3) = _9.0 as u128;
(*_3) = !15170417218377549744340510084749632962_u128;
(*_3) = 228150860326761594664705112389932747626_u128;
(*_3) = 116261346319208853582371808707768741785_u128 - 20373054500706192648085636061017026486_u128;
_6 = [206_u8,78_u8];
(*_3) = 188675462396358199996625137902209636157_u128 << _9.0;
(*_3) = !339656433363946658708027115598481653406_u128;
RET = 5_usize * 7402728958086811995_usize;
_10 = !3256686755_u32;
_11 = 31122_i16 as f64;
_9.2.0 = Adt23::Variant0 { fld0: _10,fld1: _11,fld2: (-9223372036854775808_isize) };
place!(Field::<u32>(Variant(_9.2.0, 0), 0)) = (*_3) as u32;
(*_3) = (-102367145395758287835424186572230484189_i128) as u128;
_6 = [248_u8,228_u8];
Goto(bb2)
}
bb23 = {
_13 = _5;
(*_21) = 3144132095_u32 & 3395335648_u32;
_16 = Field::<isize>(Variant(_19, 1), 2) << (*_3);
(*_3) = !323522436242769958253329384288093439232_u128;
(*_21) = 1859596591_u32 | 1281875673_u32;
(*_4) = '\u{31a77}';
(*_3) = 20941037070624694596340655226466424813_u128;
_28 = core::ptr::addr_of_mut!((*_21));
(*_28) = !1415399079_u32;
(*_28) = 398557066_u32 & 4090533193_u32;
(*_3) = (-527658680579507048_i64) as u128;
(*_21) = !3482740262_u32;
(*_4) = '\u{9f81a}';
(*_3) = 148998083482724931612935742582463177456_u128 - 299667400249047646942823983202458011487_u128;
(*_4) = '\u{d64e9}';
(*_4) = '\u{5d2ea}';
_11 = _12 * _12;
(*_4) = '\u{865e9}';
Call(_31 = core::intrinsics::transmute(Field::<isize>(Variant(_19, 1), 2)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
(*_21) = 34312208_u32;
Goto(bb25)
}
bb25 = {
(*_3) = 245405545325976104243759515035989356780_u128 & 3206258734314155666541313457972029160_u128;
(*_3) = !185447160267491766768868060715998072976_u128;
match (*_21) {
34312208 => bb27,
_ => bb26
}
}
bb26 = {
_6 = [223_u8,119_u8];
(*_3) = !125861356111713674486213007679247323317_u128;
(*_3) = Field::<isize>(Variant(_9.2.0, 0), 2) as u128;
(*_3) = 18532010970692589330473522607424433543_u128;
(*_3) = 224456185448455476792921061150057360068_u128;
_8 = core::ptr::addr_of!((*_3));
_7 = core::ptr::addr_of_mut!((*_7));
(*_3) = 13819405156886412098027274669967233632_u128 * 99803331881876004170158840568261601518_u128;
(*_7) = _10;
(*_7) = _10 - _10;
(*_7) = _10;
_13 = !_5;
(*_3) = !119215375727266710001296217184830397601_u128;
Goto(bb10)
}
bb27 = {
(*_4) = '\u{e597}';
place!(Field::<isize>(Variant(_19, 1), 2)) = !_31;
(*_4) = '\u{3db86}';
(*_3) = Field::<i128>(Variant(_9.2.0, 2), 1) as u128;
(*_4) = '\u{ec7d4}';
(*_21) = !435858754_u32;
(*_21) = 4250109986_u32 + 2213444020_u32;
(*_3) = 185952735246956805145662337746203068865_u128;
(*_21) = 1657085344_u32;
_31 = (*_4) as isize;
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = 13429666098778809888037869514149174749_i128;
(*_4) = '\u{1de06}';
_28 = core::ptr::addr_of_mut!((*_21));
_14 = _13 ^ _5;
(*_3) = 201006945049505587816377849503496958626_u128;
(*_3) = 67463221124069242629109279191514904624_u128;
(*_21) = 30064_i16 as u32;
(*_4) = '\u{3ebf6}';
_9.2.0 = Adt23::Variant0 { fld0: (*_21),fld1: _11,fld2: _16 };
_9.2.0 = Adt23::Variant0 { fld0: (*_21),fld1: _11,fld2: Field::<isize>(Variant(_19, 1), 2) };
place!(Field::<f64>(Variant(_9.2.0, 0), 1)) = _11 + _11;
(*_3) = _23 as u128;
(*_21) = !Field::<u32>(Variant(_9.2.0, 0), 0);
place!(Field::<i16>(Variant(_19, 1), 4)) = (-26969_i16) | (-13230_i16);
(*_3) = 41629359416272439614781679933234784813_u128 << _30;
(*_3) = 330872002127908300329260367112123910122_u128 | 98290366653166863601597344004702270788_u128;
(*_3) = 194125466824927336262495617489498864381_u128;
_39 = &mut place!(Field::<u16>(Variant(_19, 1), 3));
Goto(bb28)
}
bb28 = {
_11 = _12;
(*_3) = !30936960453239886156228253967064616660_u128;
_33 = _16 ^ _16;
(*_39) = (-7967529699499799760_i64) as u16;
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0) + Field::<u32>(Variant(_9.2.0, 0), 0);
_6 = [169_u8,176_u8];
_41 = _30 as f32;
_9.2.0 = Adt23::Variant0 { fld0: (*_21),fld1: _11,fld2: _33 };
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0) + Field::<u32>(Variant(_9.2.0, 0), 0);
(*_4) = '\u{e944c}';
(*_4) = '\u{748ab}';
(*_39) = _9.0 - _9.0;
(*_3) = 154054836123569445132331326508399257531_u128 & 197048807836983768756580457012864863777_u128;
(*_3) = 55100481205340008432748149480752455597_u128 | 266763042385478334825191108809070799930_u128;
Goto(bb29)
}
bb29 = {
(*_21) = !Field::<u32>(Variant(_9.2.0, 0), 0);
_5 = _13;
(*_39) = _9.0 << _33;
place!(Field::<u32>(Variant(_9.2.0, 0), 0)) = !_10;
(*_3) = _30 as u128;
(*_39) = !_9.0;
Goto(bb30)
}
bb30 = {
_20 = &mut _16;
(*_21) = _12 as u32;
(*_3) = !214131516286083824402412213431922765060_u128;
(*_4) = '\u{89372}';
(*_20) = Field::<isize>(Variant(_9.2.0, 0), 2) & _33;
(*_39) = _9.0 - _9.0;
(*_4) = '\u{66d8f}';
(*_20) = Field::<isize>(Variant(_9.2.0, 0), 2) | _31;
_17 = '\u{32c1f}';
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0);
(*_21) = !Field::<u32>(Variant(_9.2.0, 0), 0);
(*_21) = _9.2.1 as u32;
(*_39) = !_9.0;
(*_4) = '\u{ad4a9}';
(*_21) = (-31768_i16) as u32;
Goto(bb31)
}
bb31 = {
(*_4) = '\u{28c68}';
(*_39) = _9.0 - _9.0;
(*_39) = 192_u8 as u16;
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0);
_9.2.1 = 57289444987750195871400366962677118012_u128 >> (*_20);
(*_20) = Field::<f64>(Variant(_9.2.0, 0), 1) as isize;
_20 = &mut _33;
_47 = (-1379921251677630130_i64) - 9113209362541086609_i64;
_6 = [77_u8,16_u8];
(*_20) = Field::<isize>(Variant(_9.2.0, 0), 2) - Field::<isize>(Variant(_9.2.0, 0), 2);
_9.2.1 = (-54_i8) as u128;
_23 = RET;
(*_20) = !Field::<isize>(Variant(_9.2.0, 0), 2);
_3 = Move(_8);
_48.1 = &mut (*_20);
_9.0 = (*_4) as u16;
_23 = !_30;
(*_21) = _12 as u32;
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0) | Field::<u32>(Variant(_9.2.0, 0), 0);
(*_4) = '\u{d2cdb}';
(*_21) = _12 as u32;
Goto(bb32)
}
bb32 = {
(*_4) = '\u{85cb7}';
_36 = (*_39) ^ (*_39);
_22 = _27 + _41;
(*_21) = 120307442880662975030707125962770299630_i128 as u32;
_13 = _23 == _23;
_9.2.1 = 191731153231994836508144417178747829794_u128 + 126631611872097772751192847607217725555_u128;
_45 = 3_i8 - 124_i8;
_9.1 = &mut _45;
(*_4) = '\u{99e}';
(*_39) = !_36;
_54 = -Field::<isize>(Variant(_9.2.0, 0), 2);
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0) ^ Field::<u32>(Variant(_9.2.0, 0), 0);
_4 = core::ptr::addr_of_mut!((*_4));
(*_39) = _36 >> (*_21);
(*_4) = '\u{c1fe4}';
(*_39) = !_36;
(*_39) = _9.0;
_42 = _47;
_7 = core::ptr::addr_of_mut!((*_21));
_56 = &_23;
(*_7) = Field::<u32>(Variant(_9.2.0, 0), 0);
(*_4) = '\u{5e7d7}';
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0);
(*_4) = '\u{85087}';
Goto(bb33)
}
bb33 = {
(*_4) = '\u{46c2f}';
(*_21) = Field::<u32>(Variant(_9.2.0, 0), 0) | Field::<u32>(Variant(_9.2.0, 0), 0);
_56 = &_30;
_42 = _47 | _47;
_57 = _13 as u32;
_26 = [_9.2.1,_9.2.1];
(*_21) = _57;
_46 = _42 as u128;
_55 = &_27;
(*_21) = _57;
(*_4) = '\u{1d246}';
(*_4) = '\u{25a3f}';
(*_4) = '\u{30244}';
_14 = (*_21) <= (*_21);
_13 = _14 <= _14;
_57 = !(*_21);
(*_4) = '\u{94ef6}';
Call(_38 = core::intrinsics::fmaf64(Field::<f64>(Variant(_9.2.0, 0), 1), _12, Field::<f64>(Variant(_9.2.0, 0), 1)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
(*_4) = '\u{7f858}';
(*_4) = '\u{887a7}';
_23 = (*_4) as usize;
_40 = _22 * _41;
(*_21) = _40 as u32;
(*_4) = '\u{c0bea}';
_41 = (*_55);
_9.2.0 = Adt23::Variant2 { fld0: (*_39),fld1: (-149537575328338140291672038214263341073_i128) };
(*_39) = !_36;
Goto(bb35)
}
bb35 = {
(*_21) = _57 + _57;
(*_39) = _36 << (*_21);
(*_21) = _57 & _57;
_9.0 = (*_39) & (*_39);
(*_21) = (-1203763135_i32) as u32;
_51 = (*_56) * (*_56);
_9.2.0 = Adt23::Variant0 { fld0: (*_21),fld1: _12,fld2: _54 };
_54 = Field::<isize>(Variant(_9.2.0, 0), 2) << (*_39);
(*_4) = '\u{21fa7}';
_34 = _42 as f64;
(*_21) = !_57;
(*_4) = '\u{b7463}';
(*_39) = _9.0 | _9.0;
_31 = Field::<isize>(Variant(_9.2.0, 0), 2);
_31 = _54 + _54;
_52 = _31;
(*_4) = '\u{2a57f}';
(*_21) = _57 << (*_39);
(*_4) = '\u{e1f46}';
(*_39) = !_9.0;
_62 = !(*_21);
_56 = &RET;
(*_21) = _62 >> (*_39);
_55 = &_22;
(*_39) = _9.0;
_10 = _62 - _62;
Goto(bb36)
}
bb36 = {
(*_21) = _62 + _62;
_14 = _13;
_59 = &mut _3;
_4 = core::ptr::addr_of_mut!((*_4));
_46 = 40_i8 as u128;
(*_39) = _9.0 ^ _9.0;
_40 = (*_55) - (*_55);
(*_4) = '\u{cbac4}';
_63 = !(*_21);
_14 = (*_21) >= (*_21);
(*_21) = !_62;
_39 = &mut _9.0;
_17 = '\u{43664}';
(*_39) = _36;
_13 = _5;
(*_59) = core::ptr::addr_of!(_46);
(*_39) = !_36;
_54 = _31 | _31;
_43 = _46;
_49 = 8229824336102783983_u64 as usize;
_61 = (*_4);
(*_59) = core::ptr::addr_of!(_46);
(*_59) = Move(_15);
_27 = (*_55);
(*_59) = core::ptr::addr_of!(_43);
_49 = (*_56) >> (*_21);
(*_39) = !_36;
(*_21) = _43 as u32;
Goto(bb37)
}
bb37 = {
_38 = -_12;
(*_21) = 18288455290670955418_u64 as u32;
_63 = (*_21);
_42 = _47;
(*_39) = _36;
_40 = (*_55);
(*_4) = _61;
(*_39) = _36;
(*_21) = _62;
_27 = -(*_55);
_60 = 7480762754156503093_u64 as f32;
_65 = (*_4);
Goto(bb38)
}
bb38 = {
(*_4) = _61;
_31 = _49 as isize;
(*_4) = _65;
(*_21) = _62;
Goto(bb39)
}
bb39 = {
_68 = 247_u8 >> (*_21);
_27 = (*_55) - _22;
(*_4) = _65;
(*_59) = core::ptr::addr_of!(_46);
(*_21) = _62 | _62;
(*_21) = _62 | _62;
(*_21) = _62 | _62;
(*_39) = _36;
(*_21) = (-26979_i16) as u32;
_22 = _27 - _41;
(*_39) = _36 ^ _36;
_75 = Move((*_59));
Goto(bb40)
}
bb40 = {
_38 = _12;
(*_21) = !_62;
_4 = core::ptr::addr_of_mut!((*_4));
(*_21) = _62;
_51 = (-1419617522_i32) as usize;
_7 = Move(_21);
_20 = &mut _54;
(*_39) = (-12666875673012109467424977581611197422_i128) as u16;
_51 = 1770606907_i32 as usize;
_77.2 = (*_56) << _62;
_60 = _43 as f32;
(*_4) = _61;
(*_4) = _65;
_79.1 = &mut (*_20);
_11 = _34 - _12;
(*_39) = !_36;
(*_39) = _36 ^ _36;
_66 = -_31;
_62 = !_10;
_41 = -_27;
Goto(bb41)
}
bb41 = {
(*_39) = (*_4) as u16;
(*_39) = 12706466816346233964_u64 as u16;
_38 = -_12;
_67 = Adt23::Variant2 { fld0: (*_39),fld1: (-151560946947846459775621835463257824967_i128) };
_83 = _41 - _41;
(*_4) = _61;
_15 = core::ptr::addr_of!(_43);
Goto(bb42)
}
bb42 = {
(*_39) = 17565211280847271523_u64 as u16;
_30 = _12 as usize;
_4 = core::ptr::addr_of_mut!(_61);
_67 = Adt23::Variant2 { fld0: (*_39),fld1: (-82360878581718423483985625356976691114_i128) };
(*_4) = _65;
_34 = 2173801467674565914_u64 as f64;
(*_15) = _46 * _46;
_87 = core::ptr::addr_of_mut!(_77.1);
_70 = &mut _75;
(*_87) = (-194_i16);
_29 = core::ptr::addr_of_mut!(_87);
(*_4) = _65;
_18 = &mut (*_29);
_51 = 562264105_i32 as usize;
_50 = &(*_56);
_79.1 = Move(_20);
_66 = _31 | _52;
(*_70) = core::ptr::addr_of!((*_15));
(*_39) = _77.1 as u16;
(*_70) = Move(_15);
_90 = _77.1 as f64;
(*_4) = _65;
_23 = _49;
match _77.1 {
0 => bb1,
1 => bb25,
2 => bb41,
3 => bb8,
4 => bb43,
5 => bb44,
6 => bb45,
340282366920938463463374607431768211262 => bb47,
_ => bb46
}
}
bb43 = {
_9.0 = Field::<u16>(Variant(_9.2.0, 2), 0) & Field::<u16>(Variant(_9.2.0, 2), 0);
(*_3) = 198767797305928278312533396440391105463_u128 * 257148842769577980104627384491620575419_u128;
_21 = core::ptr::addr_of_mut!(_10);
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = -52710099018582940358237625366920928536_i128;
(*_21) = !340097544_u32;
(*_21) = (-3600253944641687942_i64) as u32;
(*_21) = (*_3) as u32;
(*_21) = 3593783577_u32;
(*_4) = '\u{95ce8}';
_6 = [73_u8,191_u8];
_15 = core::ptr::addr_of!((*_3));
_4 = core::ptr::addr_of_mut!((*_4));
(*_15) = 208631775487199802437207475513845209048_u128 << Field::<i128>(Variant(_9.2.0, 2), 1);
(*_15) = 43323333168302330225646525785751038971_u128 ^ 198311888602561066457857853044117708708_u128;
(*_21) = 428282607_u32 & 1110263952_u32;
_12 = _11 - _11;
_7 = core::ptr::addr_of_mut!((*_21));
(*_3) = 166981269920951321109454393040769441701_u128;
_6 = [16_u8,72_u8];
(*_7) = 4065541016_u32 >> (*_3);
(*_3) = (*_4) as u128;
(*_21) = _16 as u32;
(*_3) = (-119_i8) as u128;
(*_4) = '\u{d0d7}';
(*_3) = !205778815397016631857899204866254102420_u128;
place!(Field::<i128>(Variant(_9.2.0, 2), 1)) = _13 as i128;
Goto(bb14)
}
bb44 = {
(*_3) = 245405545325976104243759515035989356780_u128 & 3206258734314155666541313457972029160_u128;
(*_3) = !185447160267491766768868060715998072976_u128;
match (*_21) {
34312208 => bb27,
_ => bb26
}
}
bb45 = {
(*_21) = _62 + _62;
_14 = _13;
_59 = &mut _3;
_4 = core::ptr::addr_of_mut!((*_4));
_46 = 40_i8 as u128;
(*_39) = _9.0 ^ _9.0;
_40 = (*_55) - (*_55);
(*_4) = '\u{cbac4}';
_63 = !(*_21);
_14 = (*_21) >= (*_21);
(*_21) = !_62;
_39 = &mut _9.0;
_17 = '\u{43664}';
(*_39) = _36;
_13 = _5;
(*_59) = core::ptr::addr_of!(_46);
(*_39) = !_36;
_54 = _31 | _31;
_43 = _46;
_49 = 8229824336102783983_u64 as usize;
_61 = (*_4);
(*_59) = core::ptr::addr_of!(_46);
(*_59) = Move(_15);
_27 = (*_55);
(*_59) = core::ptr::addr_of!(_43);
_49 = (*_56) >> (*_21);
(*_39) = !_36;
(*_21) = _43 as u32;
Goto(bb37)
}
bb46 = {
(*_3) = 285168570789889139598936509302220520226_u128 | 300293528225425637282853643184214891659_u128;
_3 = core::ptr::addr_of!((*_3));
(*_3) = 233440285035008844455601990691063236750_u128;
place!(Field::<isize>(Variant(_9.2.0, 0), 2)) = _11 as isize;
Goto(bb4)
}
bb47 = {
place!(Field::<u16>(Variant(_67, 2), 0)) = (*_39) + (*_39);
(*_70) = core::ptr::addr_of!(_46);
(*_4) = _17;
(*_39) = Field::<u16>(Variant(_67, 2), 0) + Field::<u16>(Variant(_67, 2), 0);
_51 = !_49;
_44 = -_31;
Goto(bb48)
}
bb48 = {
_21 = Move(_7);
(*_70) = core::ptr::addr_of!(_46);
(*_18) = core::ptr::addr_of_mut!(_91);
(*_4) = _17;
(*_18) = core::ptr::addr_of_mut!(_77.1);
_41 = _83;
(*_4) = _65;
RET = (*_39) as usize;
(*_18) = core::ptr::addr_of_mut!(_77.1);
_67 = Adt23::Variant1 { fld0: _66 };
(*_18) = core::ptr::addr_of_mut!(_99);
_20 = &mut _66;
(*_18) = core::ptr::addr_of_mut!(_91);
(*_4) = _17;
_69 = &mut (*_18);
_21 = core::ptr::addr_of_mut!(_62);
_8 = core::ptr::addr_of!(_43);
(*_69) = core::ptr::addr_of_mut!(_91);
_101 = _77.1 as i64;
_83 = _41 - _22;
(*_8) = _46 * _46;
_88 = [_47,_42,_42];
_38 = (*_20) as f64;
(*_8) = _46;
Goto(bb49)
}
bb49 = {
(*_20) = !_44;
Goto(bb50)
}
bb50 = {
Call(_106 = dump_var(Move(_46), Move(_44), Move(_43), Move(_54)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_106 = dump_var(Move(_47), Move(_14), Move(_16), Move(_57)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_106 = dump_var(Move(_62), Move(_49), Move(_23), Move(_6)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_106 = dump_var(Move(_61), Move(_65), Move(_31), Move(_13)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut char,mut _2: *const &'static mut &'static f64) -> u128 {
mir! {
type RET = u128;
let _3: [i64; 3];
let _4: (&'static f64, [u32; 7]);
let _5: *mut [u32; 7];
let _6: (&'static mut &'static f64, i16, usize, [char; 4]);
let _7: *mut *mut &'static f64;
let _8: *mut i16;
let _9: u16;
let _10: &'static mut *mut char;
let _11: *const i16;
let _12: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _13: &'static mut isize;
let _14: &'static f64;
let _15: isize;
let _16: &'static mut *mut char;
let _17: Adt75;
let _18: f32;
let _19: u128;
let _20: &'static mut &'static f64;
let _21: &'static usize;
let _22: i16;
let _23: f32;
let _24: isize;
let _25: char;
let _26: (*mut *mut i16, i32, Adt55, *mut i16);
let _27: isize;
let _28: &'static i32;
let _29: u8;
let _30: isize;
let _31: f64;
let _32: i64;
let _33: f32;
let _34: f32;
let _35: (u128, usize, [i32; 1]);
let _36: isize;
let _37: u8;
let _38: u8;
let _39: [i8; 8];
let _40: isize;
let _41: u16;
let _42: &'static f64;
let _43: &'static i32;
let _44: [i8; 8];
let _45: Adt28;
let _46: [u32; 7];
let _47: u128;
let _48: u128;
let _49: &'static mut isize;
let _50: &'static mut *mut i16;
let _51: usize;
let _52: *mut char;
let _53: i32;
let _54: [usize; 7];
let _55: *const char;
let _56: f32;
let _57: &'static bool;
let _58: bool;
let _59: &'static mut *const u128;
let _60: i128;
let _61: i128;
let _62: [bool; 2];
let _63: (*mut &'static f64, &'static mut isize);
let _64: *const char;
let _65: Adt50;
let _66: &'static i32;
let _67: i8;
let _68: char;
let _69: [usize; 7];
let _70: &'static mut *const i16;
let _71: Adt41;
let _72: isize;
let _73: (*const i16,);
let _74: &'static mut isize;
let _75: ();
let _76: ();
{
RET = 28150826880106639601621424522170183446_u128 ^ 231543331350286409311999116221067069466_u128;
RET = 327191362247772319935788907958121428656_u128 >> 193_u8;
RET = 94958129882680843741539015586546130344_u128 - 213692369557531635724944404156322874330_u128;
RET = 138742248869055789412259817604966890682_u128 << (-2082746024_i32);
RET = 245646738813033033245919542975985231956_u128 & 138190496061503347186816367180504704889_u128;
RET = 239215117411229884933712213394404306953_u128 + 238108351418164525159050567261960736825_u128;
RET = 256039171823140739307642076148464556464_u128;
_3 = [(-1257608233164724011_i64),(-4387625170404968596_i64),(-936820462261590209_i64)];
_4.1 = [597478378_u32,1125125577_u32,2991154312_u32,2098347519_u32,2674469561_u32,1197590096_u32,4271473155_u32];
_4.1 = [3368250021_u32,446623366_u32,2522567846_u32,2916775703_u32,2397775117_u32,1807306142_u32,2150516009_u32];
_4.1 = [1298709224_u32,343587574_u32,1586642461_u32,431865647_u32,2522128112_u32,1367339420_u32,1613876787_u32];
RET = 129364352235227173894438947148136758857_u128;
RET = 99082329846004940602190630095238783360_u128 >> (-975521762_i32);
_3 = [2100527187257653701_i64,(-4743330195930682122_i64),7316121082914364164_i64];
_3 = [6708968357049612218_i64,(-1518432984959884262_i64),(-3521958600506801679_i64)];
Goto(bb1)
}
bb1 = {
_3 = [(-7926477423842726560_i64),1233952608386070253_i64,(-7142727857550161162_i64)];
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = [189859929_u32,221944804_u32,3814040312_u32,1870347167_u32,1284614710_u32,4057965493_u32,3469303461_u32];
(*_5) = [2943733380_u32,3325953451_u32,2168853767_u32,4081769515_u32,2655301303_u32,3593800804_u32,4219423475_u32];
(*_5) = [4108421178_u32,816331508_u32,1850057768_u32,251287181_u32,312334215_u32,3777482596_u32,4131033790_u32];
_4.1 = [409419776_u32,2333658489_u32,706006340_u32,2424505438_u32,4201283322_u32,943073909_u32,878843454_u32];
_6.2 = '\u{ac949}' as usize;
(*_5) = [1347559909_u32,1666905328_u32,2639007430_u32,4232822179_u32,3388249802_u32,904952731_u32,742041857_u32];
(*_5) = [2315047399_u32,2827977166_u32,1145681768_u32,2556103937_u32,1939270312_u32,716973243_u32,2797802959_u32];
(*_5) = [3960607760_u32,2503980883_u32,3092933276_u32,4259435660_u32,2981712714_u32,4071284571_u32,1329668104_u32];
_8 = core::ptr::addr_of_mut!(_6.1);
RET = 122401005302358171226967882227188139841_u128 | 17607175463409253951205714012417251975_u128;
(*_5) = [2846532366_u32,2379707412_u32,3372517983_u32,2340460741_u32,865980321_u32,3246629596_u32,2006943177_u32];
(*_5) = [1925517036_u32,1420208574_u32,773633234_u32,645130599_u32,3691166129_u32,4154793830_u32,3106919474_u32];
(*_8) = -(-13221_i16);
(*_8) = 29149_i16;
_10 = &mut _1;
match (*_8) {
0 => bb2,
29149 => bb4,
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
(*_8) = false as i16;
_8 = core::ptr::addr_of_mut!((*_8));
(*_5) = [3953529149_u32,180305231_u32,3334228354_u32,2412264168_u32,368451256_u32,496418063_u32,766081666_u32];
_6.3 = ['\u{8158f}','\u{ebff7}','\u{12771}','\u{210c5}'];
(*_8) = 31576_i16 - (-9733_i16);
_6.1 = (-12900_i16) * (-16776_i16);
_6.1 = (-16600_i16) ^ 29579_i16;
_5 = core::ptr::addr_of_mut!((*_5));
_4.1 = [1941970202_u32,1626641330_u32,515150881_u32,3742718914_u32,2271132614_u32,1431346192_u32,2220795998_u32];
(*_5) = [1451063694_u32,1362918274_u32,1488079429_u32,1485246873_u32,348379270_u32,1332453741_u32,4052063750_u32];
(*_5) = [830018034_u32,1833527195_u32,3885026639_u32,2375664010_u32,957528891_u32,2744635666_u32,2364967024_u32];
(*_5) = [3618802164_u32,490685873_u32,2537051177_u32,829417556_u32,1384956217_u32,3929390213_u32,3899253313_u32];
_6.1 = -29270_i16;
(*_5) = [3541037437_u32,3073165490_u32,148734110_u32,467515396_u32,2414180476_u32,1900541914_u32,3890907567_u32];
(*_5) = [456744883_u32,2286932888_u32,1418055598_u32,4180506464_u32,2640686807_u32,1065138332_u32,3628750711_u32];
_9 = 43367_u16 * 35830_u16;
Call((*_8) = fn18(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_8) = (-14595_i16);
(*_5) = [2777079944_u32,2955787902_u32,684846459_u32,2020370294_u32,3061577572_u32,1384082831_u32,2150463589_u32];
(*_8) = 2905_i16 >> _6.2;
(*_5) = [1432319275_u32,676393889_u32,2161218352_u32,1602436155_u32,1248579248_u32,2867162179_u32,1499894816_u32];
(*_5) = [3922057735_u32,2119754504_u32,1979015763_u32,715744075_u32,1284257859_u32,1449299544_u32,3710467908_u32];
(*_5) = [4243192884_u32,2228981570_u32,1055976211_u32,2305054910_u32,2079811195_u32,3262420475_u32,2690860896_u32];
_6.2 = !422410774961155540_usize;
(*_8) = 26588_i16 << RET;
(*_5) = [3869461039_u32,1968406383_u32,2746060468_u32,294591540_u32,444339464_u32,2540136588_u32,2067604585_u32];
(*_8) = 3521_i16 & 11962_i16;
Goto(bb6)
}
bb6 = {
(*_5) = [3570457490_u32,4063133141_u32,3064059968_u32,2596606671_u32,2425889844_u32,636666024_u32,105643986_u32];
(*_8) = 14837_i16 >> _9;
(*_8) = (-32393_i16) ^ 6927_i16;
(*_8) = !28948_i16;
_15 = -9223372036854775807_isize;
Goto(bb7)
}
bb7 = {
(*_5) = [909198781_u32,3385208419_u32,3962834304_u32,705245369_u32,158087558_u32,3312426954_u32,3658870105_u32];
(*_8) = 31528_i16 - (-14738_i16);
_13 = &mut _15;
(*_5) = [446289459_u32,22146605_u32,3856190106_u32,723234815_u32,1598867504_u32,1998851097_u32,1565878228_u32];
(*_8) = 3668_i16;
_9 = 58208_u16 ^ 26473_u16;
(*_13) = 9223372036854775807_isize;
_5 = core::ptr::addr_of_mut!((*_5));
(*_8) = 28539_i16 + (-6432_i16);
(*_13) = 9223372036854775807_isize | (-33_isize);
(*_13) = 9223372036854775807_isize - 9223372036854775807_isize;
(*_8) = !(-16472_i16);
(*_8) = (-17566_i16);
RET = 47506029297625201085750511074265772913_u128 >> (*_8);
(*_8) = (-19351_i16) - (-20167_i16);
(*_13) = (-5_isize) ^ (-9223372036854775808_isize);
(*_5) = [1764227821_u32,3139014320_u32,4005990703_u32,591011883_u32,3140282779_u32,3923103315_u32,3618689149_u32];
(*_8) = !(-20215_i16);
Goto(bb8)
}
bb8 = {
(*_8) = '\u{5d009}' as i16;
(*_13) = !(-9223372036854775808_isize);
(*_13) = 9223372036854775807_isize | 9223372036854775807_isize;
_6.3 = ['\u{ced38}','\u{fde4c}','\u{251dc}','\u{5311c}'];
(*_13) = 9223372036854775807_isize << _9;
(*_8) = (-28096_i16) * 17222_i16;
(*_13) = (-95_isize) << (*_8);
(*_13) = (-86_isize) | (-9223372036854775808_isize);
(*_8) = 21227_i16;
(*_5) = [1742035123_u32,808227848_u32,3795078813_u32,4097122872_u32,1366999388_u32,3458558514_u32,887960603_u32];
(*_8) = 7678_i16 - (-32713_i16);
(*_5) = [3479925790_u32,1552130036_u32,1979994274_u32,870762834_u32,966624241_u32,3922271974_u32,1181492616_u32];
(*_5) = [1916579572_u32,2540050812_u32,2692265973_u32,2255209806_u32,2254093965_u32,2973135775_u32,3177367190_u32];
Goto(bb9)
}
bb9 = {
(*_5) = [1342927535_u32,1356006250_u32,568149988_u32,918603481_u32,2152792170_u32,1511558615_u32,2737971828_u32];
_4.1 = [2869626726_u32,4190750279_u32,3839210599_u32,2763294925_u32,257610459_u32,4079417837_u32,2599722692_u32];
(*_13) = 17_u8 as isize;
(*_5) = [3241846216_u32,592679077_u32,3984828150_u32,3735855413_u32,890866136_u32,3535754740_u32,4065495742_u32];
(*_5) = [4224718153_u32,1996412249_u32,2906016088_u32,29177286_u32,3068621360_u32,1351979883_u32,104801583_u32];
(*_5) = [2912921162_u32,3860950451_u32,2572975344_u32,1993739022_u32,3015281497_u32,995951016_u32,1974032778_u32];
(*_13) = -88_isize;
(*_8) = !(-13763_i16);
(*_13) = 104_isize >> (*_8);
(*_13) = _9 as isize;
(*_8) = 12968_i16 >> (*_13);
(*_8) = 793_i16 << (*_13);
_19 = (-2160272344854557985_i64) as u128;
_22 = !(*_8);
(*_5) = [4189717819_u32,3195289251_u32,586527886_u32,2979652365_u32,2921639122_u32,2729438674_u32,981508936_u32];
(*_13) = 9223372036854775807_isize << (*_8);
_16 = &mut (*_10);
(*_8) = _22;
(*_8) = -_22;
(*_13) = !9223372036854775807_isize;
(*_5) = [996694921_u32,1843755357_u32,11089008_u32,628168101_u32,2661472678_u32,2292247220_u32,1466485664_u32];
(*_5) = [2730531513_u32,2509026212_u32,1319758657_u32,2192092150_u32,4195012446_u32,273848216_u32,1841550179_u32];
(*_8) = _22;
(*_13) = 9223372036854775807_isize & (-9223372036854775808_isize);
Goto(bb10)
}
bb10 = {
(*_8) = _9 as i16;
_25 = '\u{ccae0}';
_24 = (*_13);
(*_16) = core::ptr::addr_of_mut!(_25);
(*_5) = [3067769285_u32,3384680999_u32,4280276175_u32,2850463926_u32,1493256556_u32,3726842415_u32,2933429861_u32];
(*_8) = -_22;
(*_5) = [3315136127_u32,2395199485_u32,1745014508_u32,554613751_u32,3543198063_u32,3671978988_u32,2237565358_u32];
_3 = [2255402449002730937_i64,(-967280277026119323_i64),4137033206691185955_i64];
Goto(bb11)
}
bb11 = {
(*_8) = -_22;
(*_5) = [342129426_u32,474670154_u32,1832353215_u32,3252886614_u32,2826240217_u32,1358906843_u32,3722180409_u32];
(*_8) = _22 * _22;
(*_13) = _6.1 as isize;
_27 = -(*_13);
_26.3 = core::ptr::addr_of_mut!((*_8));
Goto(bb12)
}
bb12 = {
(*_5) = [4242801610_u32,3042350264_u32,3643417019_u32,1230166354_u32,2824446890_u32,2024131681_u32,3235585126_u32];
_6.1 = !_22;
(*_8) = _22;
(*_16) = core::ptr::addr_of_mut!(_25);
(*_16) = core::ptr::addr_of_mut!(_25);
(*_5) = [1580903323_u32,2884360757_u32,2990582034_u32,1246979548_u32,232424622_u32,4262032584_u32,975181225_u32];
_10 = Move(_16);
(*_5) = [1634686224_u32,2077729167_u32,4001958556_u32,2980662251_u32,2908403575_u32,2010087536_u32,3254212087_u32];
(*_5) = [1245783241_u32,1318538730_u32,3279291752_u32,2504344387_u32,1203696089_u32,3413594667_u32,1059428155_u32];
_32 = 6017170461511208334_i64;
(*_5) = [3315936903_u32,526579480_u32,2971726884_u32,2562986065_u32,4206041929_u32,2151689084_u32,335483798_u32];
Goto(bb13)
}
bb13 = {
(*_5) = [3467256610_u32,2163548328_u32,638489370_u32,3941152331_u32,646206489_u32,446677011_u32,3126555633_u32];
_26.3 = core::ptr::addr_of_mut!((*_8));
(*_5) = [337990319_u32,572430433_u32,3849711598_u32,2399214370_u32,3661349758_u32,2495448513_u32,2737458343_u32];
(*_13) = _27;
(*_13) = RET as isize;
_23 = _32 as f32;
(*_5) = [3701968625_u32,278898661_u32,915549169_u32,1995143778_u32,2476759075_u32,3650488825_u32,4000539648_u32];
(*_5) = [534968145_u32,672048528_u32,3877573795_u32,2209849723_u32,3743327357_u32,1437599600_u32,605856783_u32];
(*_8) = _6.2 as i16;
(*_8) = _22 >> (*_13);
(*_8) = _22;
_37 = 50_u8 >> (*_8);
(*_8) = -_22;
_32 = (-8099948425569853012_i64) << (*_8);
_9 = 30661_u16;
_21 = &_6.2;
_5 = core::ptr::addr_of_mut!((*_5));
(*_8) = false as i16;
(*_5) = [1466832500_u32,2853110423_u32,517302330_u32,3421628378_u32,1189648355_u32,2489208529_u32,405321802_u32];
_34 = _23 + _23;
_29 = _37 | _37;
_38 = _37 | _37;
_25 = '\u{b0c06}';
_26.3 = core::ptr::addr_of_mut!((*_8));
Goto(bb14)
}
bb14 = {
(*_5) = [373083347_u32,3258116172_u32,4032789685_u32,3835489356_u32,792405118_u32,2351808553_u32,1689819093_u32];
_36 = (*_13) & (*_13);
(*_13) = 111_i8 as isize;
_18 = _34;
(*_13) = _24 & _27;
_29 = _38 ^ _38;
(*_5) = [3891549158_u32,228634456_u32,23396198_u32,947745610_u32,1458858583_u32,3342909031_u32,3466643715_u32];
(*_8) = _22;
_33 = _18;
(*_13) = _27;
match _9 {
30661 => bb16,
_ => bb15
}
}
bb15 = {
(*_5) = [909198781_u32,3385208419_u32,3962834304_u32,705245369_u32,158087558_u32,3312426954_u32,3658870105_u32];
(*_8) = 31528_i16 - (-14738_i16);
_13 = &mut _15;
(*_5) = [446289459_u32,22146605_u32,3856190106_u32,723234815_u32,1598867504_u32,1998851097_u32,1565878228_u32];
(*_8) = 3668_i16;
_9 = 58208_u16 ^ 26473_u16;
(*_13) = 9223372036854775807_isize;
_5 = core::ptr::addr_of_mut!((*_5));
(*_8) = 28539_i16 + (-6432_i16);
(*_13) = 9223372036854775807_isize | (-33_isize);
(*_13) = 9223372036854775807_isize - 9223372036854775807_isize;
(*_8) = !(-16472_i16);
(*_8) = (-17566_i16);
RET = 47506029297625201085750511074265772913_u128 >> (*_8);
(*_8) = (-19351_i16) - (-20167_i16);
(*_13) = (-5_isize) ^ (-9223372036854775808_isize);
(*_5) = [1764227821_u32,3139014320_u32,4005990703_u32,591011883_u32,3140282779_u32,3923103315_u32,3618689149_u32];
(*_8) = !(-20215_i16);
Goto(bb8)
}
bb16 = {
_39 = [44_i8,(-56_i8),28_i8,(-41_i8),21_i8,127_i8,(-35_i8),114_i8];
(*_5) = [3504282478_u32,994257552_u32,2595087693_u32,1324770433_u32,3112574319_u32,2007254420_u32,2821455815_u32];
(*_5) = [2691160940_u32,872230087_u32,729395487_u32,3832979531_u32,601891122_u32,109059797_u32,976485275_u32];
_6.3 = [_25,_25,_25,_25];
(*_8) = _22;
_12.1.1 = (*_5);
(*_13) = _27;
(*_5) = [2253066399_u32,1884045729_u32,3031872306_u32,2877973744_u32,3896177617_u32,1437412651_u32,460914107_u32];
_33 = _18 + _18;
(*_8) = _32 as i16;
(*_5) = [3289100644_u32,4102922090_u32,3002405483_u32,1023754102_u32,4041133258_u32,2141896476_u32,3234928321_u32];
_36 = (*_13) >> (*_21);
_30 = _37 as isize;
(*_8) = _22;
(*_5) = [342420017_u32,2630209996_u32,3114741789_u32,3996011204_u32,737492750_u32,480270495_u32,3134659962_u32];
(*_5) = [1996959624_u32,2204856319_u32,4048270778_u32,3711787981_u32,1063371096_u32,128008230_u32,1328924648_u32];
(*_8) = -_22;
(*_8) = _22 & _22;
_35.0 = _19 ^ RET;
(*_13) = _30 | _27;
_31 = (*_21) as f64;
(*_8) = (*_21) as i16;
Goto(bb17)
}
bb17 = {
_4.0 = &_31;
_12.1.0 = &_31;
(*_13) = _36;
(*_13) = _27 << RET;
_6.1 = _22 ^ _22;
_45.fld4 = (*_21) | (*_21);
(*_8) = _22 & _22;
_14 = &_31;
(*_8) = !_22;
(*_13) = !_36;
(*_5) = [1519970234_u32,3538609609_u32,3210527893_u32,151986543_u32,3422125986_u32,848199373_u32,2618640727_u32];
_18 = _33 - _33;
_47 = RET | _35.0;
(*_13) = _36 * _36;
_35.1 = (*_21) * (*_21);
(*_5) = _12.1.1;
_6.2 = _35.1;
_45.fld2 = _18 - _23;
_31 = (-36_i8) as f64;
_24 = (*_13) ^ (*_13);
Goto(bb18)
}
bb18 = {
_45.fld2 = _35.1 as f32;
_45.fld1 = 2361714560_u32 * 2420999474_u32;
(*_5) = _12.1.1;
_12.3 = Adt41::Variant1 { fld0: true,fld1: 5190404331452301565_u64,fld2: _31,fld3: 68_i8,fld4: _45.fld1 };
(*_8) = _22 | _22;
place!(Field::<u64>(Variant(_12.3, 1), 1)) = 4732876853616365074_u64 * 5628710840955486577_u64;
(*_8) = _31 as i16;
_51 = _35.1;
_3 = [_32,_32,_32];
_6.0 = &mut _14;
_19 = _47 & _35.0;
_36 = (*_8) as isize;
(*_8) = _22 - _22;
_19 = !_35.0;
(*_8) = _22 - _22;
_12.1 = Move(_4);
_6.1 = !_22;
Goto(bb19)
}
bb19 = {
_54 = [_35.1,_6.2,_35.1,_51,_6.2,_6.2,_51];
_42 = &place!(Field::<f64>(Variant(_12.3, 1), 2));
(*_13) = _24 ^ _27;
_45.fld1 = Field::<u32>(Variant(_12.3, 1), 4) + Field::<u32>(Variant(_12.3, 1), 4);
_48 = _9 as u128;
(*_8) = _22 + _22;
_20 = Move(_6.0);
(*_8) = _22;
(*_13) = _24;
_48 = RET;
(*_8) = _22;
_58 = true;
_13 = &mut _30;
(*_8) = _22 | _22;
_50 = &mut _8;
(*_13) = _24;
(*_5) = [Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,_45.fld1,_45.fld1,_45.fld1];
_45.fld2 = _18;
(*_5) = [_45.fld1,_45.fld1,_45.fld1,_45.fld1,_45.fld1,Field::<u32>(Variant(_12.3, 1), 4),_45.fld1];
_35.2 = [(-699509180_i32)];
(*_5) = [Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,_45.fld1,Field::<u32>(Variant(_12.3, 1), 4)];
_12.1.0 = &(*_42);
(*_5) = [_45.fld1,_45.fld1,_45.fld1,_45.fld1,Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),_45.fld1];
match _9 {
0 => bb14,
1 => bb20,
2 => bb21,
3 => bb22,
30661 => bb24,
_ => bb23
}
}
bb20 = {
(*_8) = false as i16;
_8 = core::ptr::addr_of_mut!((*_8));
(*_5) = [3953529149_u32,180305231_u32,3334228354_u32,2412264168_u32,368451256_u32,496418063_u32,766081666_u32];
_6.3 = ['\u{8158f}','\u{ebff7}','\u{12771}','\u{210c5}'];
(*_8) = 31576_i16 - (-9733_i16);
_6.1 = (-12900_i16) * (-16776_i16);
_6.1 = (-16600_i16) ^ 29579_i16;
_5 = core::ptr::addr_of_mut!((*_5));
_4.1 = [1941970202_u32,1626641330_u32,515150881_u32,3742718914_u32,2271132614_u32,1431346192_u32,2220795998_u32];
(*_5) = [1451063694_u32,1362918274_u32,1488079429_u32,1485246873_u32,348379270_u32,1332453741_u32,4052063750_u32];
(*_5) = [830018034_u32,1833527195_u32,3885026639_u32,2375664010_u32,957528891_u32,2744635666_u32,2364967024_u32];
(*_5) = [3618802164_u32,490685873_u32,2537051177_u32,829417556_u32,1384956217_u32,3929390213_u32,3899253313_u32];
_6.1 = -29270_i16;
(*_5) = [3541037437_u32,3073165490_u32,148734110_u32,467515396_u32,2414180476_u32,1900541914_u32,3890907567_u32];
(*_5) = [456744883_u32,2286932888_u32,1418055598_u32,4180506464_u32,2640686807_u32,1065138332_u32,3628750711_u32];
_9 = 43367_u16 * 35830_u16;
Call((*_8) = fn18(), ReturnTo(bb5), UnwindUnreachable())
}
bb21 = {
(*_8) = '\u{5d009}' as i16;
(*_13) = !(-9223372036854775808_isize);
(*_13) = 9223372036854775807_isize | 9223372036854775807_isize;
_6.3 = ['\u{ced38}','\u{fde4c}','\u{251dc}','\u{5311c}'];
(*_13) = 9223372036854775807_isize << _9;
(*_8) = (-28096_i16) * 17222_i16;
(*_13) = (-95_isize) << (*_8);
(*_13) = (-86_isize) | (-9223372036854775808_isize);
(*_8) = 21227_i16;
(*_5) = [1742035123_u32,808227848_u32,3795078813_u32,4097122872_u32,1366999388_u32,3458558514_u32,887960603_u32];
(*_8) = 7678_i16 - (-32713_i16);
(*_5) = [3479925790_u32,1552130036_u32,1979994274_u32,870762834_u32,966624241_u32,3922271974_u32,1181492616_u32];
(*_5) = [1916579572_u32,2540050812_u32,2692265973_u32,2255209806_u32,2254093965_u32,2973135775_u32,3177367190_u32];
Goto(bb9)
}
bb22 = {
(*_5) = [373083347_u32,3258116172_u32,4032789685_u32,3835489356_u32,792405118_u32,2351808553_u32,1689819093_u32];
_36 = (*_13) & (*_13);
(*_13) = 111_i8 as isize;
_18 = _34;
(*_13) = _24 & _27;
_29 = _38 ^ _38;
(*_5) = [3891549158_u32,228634456_u32,23396198_u32,947745610_u32,1458858583_u32,3342909031_u32,3466643715_u32];
(*_8) = _22;
_33 = _18;
(*_13) = _27;
match _9 {
30661 => bb16,
_ => bb15
}
}
bb23 = {
(*_5) = [1342927535_u32,1356006250_u32,568149988_u32,918603481_u32,2152792170_u32,1511558615_u32,2737971828_u32];
_4.1 = [2869626726_u32,4190750279_u32,3839210599_u32,2763294925_u32,257610459_u32,4079417837_u32,2599722692_u32];
(*_13) = 17_u8 as isize;
(*_5) = [3241846216_u32,592679077_u32,3984828150_u32,3735855413_u32,890866136_u32,3535754740_u32,4065495742_u32];
(*_5) = [4224718153_u32,1996412249_u32,2906016088_u32,29177286_u32,3068621360_u32,1351979883_u32,104801583_u32];
(*_5) = [2912921162_u32,3860950451_u32,2572975344_u32,1993739022_u32,3015281497_u32,995951016_u32,1974032778_u32];
(*_13) = -88_isize;
(*_8) = !(-13763_i16);
(*_13) = 104_isize >> (*_8);
(*_13) = _9 as isize;
(*_8) = 12968_i16 >> (*_13);
(*_8) = 793_i16 << (*_13);
_19 = (-2160272344854557985_i64) as u128;
_22 = !(*_8);
(*_5) = [4189717819_u32,3195289251_u32,586527886_u32,2979652365_u32,2921639122_u32,2729438674_u32,981508936_u32];
(*_13) = 9223372036854775807_isize << (*_8);
_16 = &mut (*_10);
(*_8) = _22;
(*_8) = -_22;
(*_13) = !9223372036854775807_isize;
(*_5) = [996694921_u32,1843755357_u32,11089008_u32,628168101_u32,2661472678_u32,2292247220_u32,1466485664_u32];
(*_5) = [2730531513_u32,2509026212_u32,1319758657_u32,2192092150_u32,4195012446_u32,273848216_u32,1841550179_u32];
(*_8) = _22;
(*_13) = 9223372036854775807_isize & (-9223372036854775808_isize);
Goto(bb10)
}
bb24 = {
(*_13) = -_24;
_31 = (*_42);
(*_50) = core::ptr::addr_of_mut!(_22);
_45.fld2 = _29 as f32;
_49 = &mut (*_13);
_12.1.0 = &_31;
_40 = (*_49);
(*_49) = _24 | _27;
(*_5) = [_45.fld1,_45.fld1,_45.fld1,_45.fld1,_45.fld1,_45.fld1,_45.fld1];
(*_5) = [Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,Field::<u32>(Variant(_12.3, 1), 4),Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,_45.fld1];
_51 = _6.1 as usize;
(*_50) = core::ptr::addr_of_mut!(_22);
_3 = [_32,_32,_32];
_54 = [_45.fld4,_6.2,_51,_51,_51,_51,_51];
match _9 {
30661 => bb25,
_ => bb14
}
}
bb25 = {
_12.0 = Adt41::Variant1 { fld0: _58,fld1: Field::<u64>(Variant(_12.3, 1), 1),fld2: (*_42),fld3: 127_i8,fld4: _45.fld1 };
(*_50) = core::ptr::addr_of_mut!(_22);
_25 = '\u{bb493}';
(*_49) = _24 * _24;
Call(_41 = core::intrinsics::transmute(_22), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_36 = (*_49) * (*_49);
(*_5) = [Field::<u32>(Variant(_12.3, 1), 4),_45.fld1,_45.fld1,_45.fld1,Field::<u32>(Variant(_12.0, 1), 4),_45.fld1,Field::<u32>(Variant(_12.3, 1), 4)];
_31 = _32 as f64;
_50 = &mut _26.3;
_61 = 47235378580827600859345784153589988728_i128;
(*_5) = _12.1.1;
match _9 {
0 => bb1,
1 => bb9,
2 => bb8,
3 => bb27,
30661 => bb29,
_ => bb28
}
}
bb27 = {
(*_8) = -_22;
(*_5) = [342129426_u32,474670154_u32,1832353215_u32,3252886614_u32,2826240217_u32,1358906843_u32,3722180409_u32];
(*_8) = _22 * _22;
(*_13) = _6.1 as isize;
_27 = -(*_13);
_26.3 = core::ptr::addr_of_mut!((*_8));
Goto(bb12)
}
bb28 = {
_3 = [(-7926477423842726560_i64),1233952608386070253_i64,(-7142727857550161162_i64)];
_5 = core::ptr::addr_of_mut!(_4.1);
(*_5) = [189859929_u32,221944804_u32,3814040312_u32,1870347167_u32,1284614710_u32,4057965493_u32,3469303461_u32];
(*_5) = [2943733380_u32,3325953451_u32,2168853767_u32,4081769515_u32,2655301303_u32,3593800804_u32,4219423475_u32];
(*_5) = [4108421178_u32,816331508_u32,1850057768_u32,251287181_u32,312334215_u32,3777482596_u32,4131033790_u32];
_4.1 = [409419776_u32,2333658489_u32,706006340_u32,2424505438_u32,4201283322_u32,943073909_u32,878843454_u32];
_6.2 = '\u{ac949}' as usize;
(*_5) = [1347559909_u32,1666905328_u32,2639007430_u32,4232822179_u32,3388249802_u32,904952731_u32,742041857_u32];
(*_5) = [2315047399_u32,2827977166_u32,1145681768_u32,2556103937_u32,1939270312_u32,716973243_u32,2797802959_u32];
(*_5) = [3960607760_u32,2503980883_u32,3092933276_u32,4259435660_u32,2981712714_u32,4071284571_u32,1329668104_u32];
_8 = core::ptr::addr_of_mut!(_6.1);
RET = 122401005302358171226967882227188139841_u128 | 17607175463409253951205714012417251975_u128;
(*_5) = [2846532366_u32,2379707412_u32,3372517983_u32,2340460741_u32,865980321_u32,3246629596_u32,2006943177_u32];
(*_5) = [1925517036_u32,1420208574_u32,773633234_u32,645130599_u32,3691166129_u32,4154793830_u32,3106919474_u32];
(*_8) = -(-13221_i16);
(*_8) = 29149_i16;
_10 = &mut _1;
match (*_8) {
0 => bb2,
29149 => bb4,
_ => bb3
}
}
bb29 = {
_35.0 = _47 + _47;
place!(Field::<bool>(Variant(_12.3, 1), 0)) = Field::<bool>(Variant(_12.0, 1), 0);
(*_50) = core::ptr::addr_of_mut!(_22);
(*_49) = _36 >> _36;
_57 = &_58;
(*_5) = _12.1.1;
_13 = &mut _36;
(*_49) = (*_13);
_44 = [17_i8,(-109_i8),(-3_i8),(-127_i8),69_i8,39_i8,(-117_i8),100_i8];
_20 = &mut _12.1.0;
(*_13) = _51 as isize;
_23 = _18;
_24 = !(*_49);
_21 = &_51;
(*_13) = (*_49) - (*_49);
(*_20) = &(*_42);
_44 = [(-59_i8),55_i8,65_i8,71_i8,(-115_i8),(-50_i8),(-7_i8),(-73_i8)];
_4.0 = &(*_42);
(*_50) = core::ptr::addr_of_mut!(_6.1);
(*_50) = core::ptr::addr_of_mut!(_22);
(*_13) = (*_49) >> _24;
RET = _19 - _35.0;
_35.1 = (*_21);
Goto(bb30)
}
bb30 = {
Call(_75 = dump_var(Move(_36), Move(_39), Move(_19), Move(_61)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_75 = dump_var(Move(_41), Move(_47), Move(_29), Move(_30)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_75 = dump_var(Move(_22), Move(_58), Move(_27), Move(_54)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> i16 {
mir! {
type RET = i16;
let _1: char;
let _2: isize;
let _3: [i8; 8];
let _4: &'static mut *const i16;
let _5: Adt75;
let _6: Adt43;
let _7: (u128, usize, [i32; 1]);
let _8: u16;
let _9: (Adt41, (&'static f64, [u32; 7]), &'static f64, Adt41);
let _10: isize;
let _11: i128;
let _12: bool;
let _13: i8;
let _14: *mut *mut &'static f64;
let _15: isize;
let _16: &'static mut u16;
let _17: &'static mut *mut i16;
let _18: &'static u8;
let _19: *mut *mut &'static f64;
let _20: &'static mut *const u128;
let _21: &'static &'static mut u8;
let _22: &'static mut i8;
let _23: *const &'static bool;
let _24: &'static mut i8;
let _25: *mut &'static f64;
let _26: (*const i16,);
let _27: f32;
let _28: Adt44;
let _29: char;
let _30: char;
let _31: &'static mut *mut char;
let _32: f32;
let _33: *mut [u32; 7];
let _34: u64;
let _35: *const &'static bool;
let _36: f64;
let _37: f32;
let _38: i64;
let _39: *mut i16;
let _40: f32;
let _41: i16;
let _42: &'static mut *mut char;
let _43: u8;
let _44: u64;
let _45: char;
let _46: isize;
let _47: bool;
let _48: [i64; 3];
let _49: i16;
let _50: [u8; 7];
let _51: i64;
let _52: ([char; 4], [u8; 2], &'static mut isize, char);
let _53: *mut *mut &'static f64;
let _54: u8;
let _55: bool;
let _56: f32;
let _57: char;
let _58: *const char;
let _59: (Adt28, &'static mut u16);
let _60: ([char; 4], u8, [char; 4]);
let _61: *const char;
let _62: &'static mut i8;
let _63: char;
let _64: *mut [u32; 7];
let _65: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _66: &'static mut &'static usize;
let _67: char;
let _68: [bool; 2];
let _69: (Adt28, &'static mut u16);
let _70: &'static mut u8;
let _71: isize;
let _72: &'static mut *mut i16;
let _73: char;
let _74: u64;
let _75: Adt28;
let _76: bool;
let _77: *const &'static mut u8;
let _78: ();
let _79: ();
{
RET = 6162_i16 | (-11792_i16);
RET = 27961_i16 + 30294_i16;
RET = (-24732_i16);
RET = 4481_i16 | 12680_i16;
RET = 28387_i16 ^ 17511_i16;
RET = 55_u8 as i16;
RET = (-708_i16) << 32589_i16;
RET = 20901_i16;
RET = (-22335_i16);
_1 = '\u{d5849}';
_1 = '\u{c4c50}';
_1 = '\u{95ad}';
RET = 32483_i16;
RET = (-27415_i16) >> 1777397691_u32;
RET = 12413_i16;
_2 = 215271433777707469301692721060857256258_u128 as isize;
_1 = '\u{e847a}';
_3 = [(-47_i8),91_i8,(-75_i8),(-95_i8),(-1_i8),14_i8,66_i8,16_i8];
RET = 25715_i16;
Goto(bb1)
}
bb1 = {
_2 = (-9223372036854775808_isize) << RET;
_1 = '\u{cb268}';
_3 = [(-113_i8),28_i8,(-79_i8),102_i8,(-35_i8),(-31_i8),1_i8,(-123_i8)];
RET = 5728_i16;
_1 = '\u{53b7b}';
RET = !507_i16;
RET = (-14117_i16);
_3 = [(-75_i8),(-115_i8),(-39_i8),(-105_i8),(-112_i8),81_i8,(-9_i8),87_i8];
RET = (-32730_i16);
RET = (-29048_i16) - 4692_i16;
_1 = '\u{efb27}';
RET = !25953_i16;
_2 = !9223372036854775807_isize;
_7.0 = 261105143501327913663675587060821464992_u128 ^ 89299678224469461494131017335843795257_u128;
_7.2 = [(-1559897563_i32)];
_3 = [(-121_i8),9_i8,21_i8,65_i8,56_i8,73_i8,(-52_i8),(-99_i8)];
_8 = !47997_u16;
_1 = '\u{9316a}';
_9.1.1 = [561072127_u32,350838682_u32,1758186737_u32,1767037538_u32,880645120_u32,3866996030_u32,206684834_u32];
_7.1 = 3_usize + 1_usize;
_9.1.1 = [1624188133_u32,2494294151_u32,3960368838_u32,2636320064_u32,101021771_u32,2863285230_u32,53883174_u32];
_9.1.1 = [3833965636_u32,1075709006_u32,4026250840_u32,1015347182_u32,474177284_u32,2335802519_u32,1919541624_u32];
_3 = [(-62_i8),119_i8,53_i8,(-118_i8),(-42_i8),63_i8,108_i8,98_i8];
_2 = 12_isize << _7.1;
Goto(bb2)
}
bb2 = {
RET = !(-7343_i16);
_1 = '\u{4cb5e}';
_2 = (-9223372036854775808_isize);
_10 = _7.0 as isize;
_9.1.1 = [3742702607_u32,1722792379_u32,1080178645_u32,843029360_u32,3275512143_u32,1447935261_u32,4246603865_u32];
_2 = !_10;
_7.1 = 0_usize ^ 0_usize;
_7.1 = 0_usize + 1_usize;
_7.1 = 11676437392152287379_usize * 3_usize;
_7.2 = [(-1443861659_i32)];
_1 = '\u{f1e50}';
_7.2 = [985344935_i32];
RET = (-22803_i16) << _7.0;
_1 = '\u{adc32}';
_7.1 = 1_usize * 2_usize;
_11 = 4718352153664806184_u64 as i128;
Goto(bb3)
}
bb3 = {
_8 = !33925_u16;
_9.1.1 = [297214684_u32,3609278471_u32,3996857564_u32,3166687158_u32,3264634448_u32,216016671_u32,1301820776_u32];
_11 = 68679707421942222796389627865791728345_i128 << RET;
_7.2 = [123201280_i32];
_12 = false ^ false;
_13 = 2235443339320733675_i64 as i8;
_7.1 = 6521392472774211888_usize + 12260598675995442791_usize;
_7.2 = [1671317839_i32];
_9.1.1 = [3540737451_u32,1099125497_u32,2732095683_u32,1825475296_u32,3267842278_u32,861439386_u32,2780166694_u32];
_9.1.1 = [4294680105_u32,1115338561_u32,2832764165_u32,1660076788_u32,2725903846_u32,4113156777_u32,2473457060_u32];
Goto(bb4)
}
bb4 = {
_13 = _7.0 as i8;
_7.1 = 11542297924273351906_usize + 5_usize;
_13 = -123_i8;
_2 = !_10;
_15 = -_10;
_10 = _15 ^ _15;
_16 = &mut _8;
(*_16) = 14780_u16;
(*_16) = !46300_u16;
(*_16) = !35787_u16;
(*_16) = 559099724_u32 as u16;
(*_16) = !62565_u16;
_11 = (-93357376863633909280375491964203083439_i128) - (-141274825185249757862037287146770856637_i128);
(*_16) = !24729_u16;
(*_16) = 57329_u16 | 33708_u16;
(*_16) = _13 as u16;
_2 = -_10;
(*_16) = 1911018465718920393_u64 as u16;
_9.1.1 = [1554159550_u32,141310168_u32,3810931266_u32,3397324535_u32,2202783010_u32,1969534813_u32,1031661457_u32];
_12 = !false;
(*_16) = 1794_u16 << _10;
Goto(bb5)
}
bb5 = {
(*_16) = 56522_u16;
_12 = false;
_10 = _15 & _2;
(*_16) = _1 as u16;
(*_16) = 39297_u16;
(*_16) = 33776_u16;
_7.2 = [(-1977262720_i32)];
match (*_16) {
0 => bb6,
1 => bb7,
2 => bb8,
33776 => bb10,
_ => bb9
}
}
bb6 = {
_13 = _7.0 as i8;
_7.1 = 11542297924273351906_usize + 5_usize;
_13 = -123_i8;
_2 = !_10;
_15 = -_10;
_10 = _15 ^ _15;
_16 = &mut _8;
(*_16) = 14780_u16;
(*_16) = !46300_u16;
(*_16) = !35787_u16;
(*_16) = 559099724_u32 as u16;
(*_16) = !62565_u16;
_11 = (-93357376863633909280375491964203083439_i128) - (-141274825185249757862037287146770856637_i128);
(*_16) = !24729_u16;
(*_16) = 57329_u16 | 33708_u16;
(*_16) = _13 as u16;
_2 = -_10;
(*_16) = 1911018465718920393_u64 as u16;
_9.1.1 = [1554159550_u32,141310168_u32,3810931266_u32,3397324535_u32,2202783010_u32,1969534813_u32,1031661457_u32];
_12 = !false;
(*_16) = 1794_u16 << _10;
Goto(bb5)
}
bb7 = {
_8 = !33925_u16;
_9.1.1 = [297214684_u32,3609278471_u32,3996857564_u32,3166687158_u32,3264634448_u32,216016671_u32,1301820776_u32];
_11 = 68679707421942222796389627865791728345_i128 << RET;
_7.2 = [123201280_i32];
_12 = false ^ false;
_13 = 2235443339320733675_i64 as i8;
_7.1 = 6521392472774211888_usize + 12260598675995442791_usize;
_7.2 = [1671317839_i32];
_9.1.1 = [3540737451_u32,1099125497_u32,2732095683_u32,1825475296_u32,3267842278_u32,861439386_u32,2780166694_u32];
_9.1.1 = [4294680105_u32,1115338561_u32,2832764165_u32,1660076788_u32,2725903846_u32,4113156777_u32,2473457060_u32];
Goto(bb4)
}
bb8 = {
RET = !(-7343_i16);
_1 = '\u{4cb5e}';
_2 = (-9223372036854775808_isize);
_10 = _7.0 as isize;
_9.1.1 = [3742702607_u32,1722792379_u32,1080178645_u32,843029360_u32,3275512143_u32,1447935261_u32,4246603865_u32];
_2 = !_10;
_7.1 = 0_usize ^ 0_usize;
_7.1 = 0_usize + 1_usize;
_7.1 = 11676437392152287379_usize * 3_usize;
_7.2 = [(-1443861659_i32)];
_1 = '\u{f1e50}';
_7.2 = [985344935_i32];
RET = (-22803_i16) << _7.0;
_1 = '\u{adc32}';
_7.1 = 1_usize * 2_usize;
_11 = 4718352153664806184_u64 as i128;
Goto(bb3)
}
bb9 = {
_2 = (-9223372036854775808_isize) << RET;
_1 = '\u{cb268}';
_3 = [(-113_i8),28_i8,(-79_i8),102_i8,(-35_i8),(-31_i8),1_i8,(-123_i8)];
RET = 5728_i16;
_1 = '\u{53b7b}';
RET = !507_i16;
RET = (-14117_i16);
_3 = [(-75_i8),(-115_i8),(-39_i8),(-105_i8),(-112_i8),81_i8,(-9_i8),87_i8];
RET = (-32730_i16);
RET = (-29048_i16) - 4692_i16;
_1 = '\u{efb27}';
RET = !25953_i16;
_2 = !9223372036854775807_isize;
_7.0 = 261105143501327913663675587060821464992_u128 ^ 89299678224469461494131017335843795257_u128;
_7.2 = [(-1559897563_i32)];
_3 = [(-121_i8),9_i8,21_i8,65_i8,56_i8,73_i8,(-52_i8),(-99_i8)];
_8 = !47997_u16;
_1 = '\u{9316a}';
_9.1.1 = [561072127_u32,350838682_u32,1758186737_u32,1767037538_u32,880645120_u32,3866996030_u32,206684834_u32];
_7.1 = 3_usize + 1_usize;
_9.1.1 = [1624188133_u32,2494294151_u32,3960368838_u32,2636320064_u32,101021771_u32,2863285230_u32,53883174_u32];
_9.1.1 = [3833965636_u32,1075709006_u32,4026250840_u32,1015347182_u32,474177284_u32,2335802519_u32,1919541624_u32];
_3 = [(-62_i8),119_i8,53_i8,(-118_i8),(-42_i8),63_i8,108_i8,98_i8];
_2 = 12_isize << _7.1;
Goto(bb2)
}
bb10 = {
(*_16) = 63754_u16 | 27140_u16;
_7.0 = 76932796897558388584840041078552124518_u128 & 122910777221645384029975598437356418231_u128;
(*_16) = !54747_u16;
(*_16) = 28506_u16 << _10;
_2 = _10 & _10;
(*_16) = 33452_u16 & 54799_u16;
(*_16) = 435_u16 | 5512_u16;
(*_16) = 21606_u16;
(*_16) = 62595_u16;
(*_16) = 38756_u16 & 43963_u16;
_12 = true & true;
(*_16) = 5325_u16;
(*_16) = 57388_u16 << _2;
_2 = _10 << (*_16);
(*_16) = 44054_u16;
Goto(bb11)
}
bb11 = {
(*_16) = 38681_u16;
_22 = &mut _13;
(*_22) = _7.1 as i8;
(*_16) = !11995_u16;
(*_22) = -75_i8;
(*_22) = (-1187096546_i32) as i8;
(*_22) = (-90_i8);
(*_22) = (-66_i8) ^ (-21_i8);
(*_16) = !43827_u16;
RET = 27726_i16 | 4423_i16;
(*_16) = 45213_u16 ^ 43570_u16;
_12 = (*_16) > (*_16);
_7.0 = 281367326702073795689038105069506412639_u128 | 260586194199483550348062429687164824874_u128;
(*_16) = 25821_u16 * 10198_u16;
Call((*_22) = fn19(_10, (*_16), (*_16), _10, Move(_16), (*_16), _7, _1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7.0 = 320115436090530492543351772026086594750_u128 >> _15;
_3 = [(*_22),(*_22),(*_22),(*_22),(*_22),(*_22),(*_22),(*_22)];
_19 = core::ptr::addr_of_mut!(_25);
Goto(bb13)
}
bb13 = {
_7.2 = [(-995752156_i32)];
(*_22) = 73_i8 | (-83_i8);
_19 = core::ptr::addr_of_mut!((*_19));
(*_22) = 101_i8 & (-48_i8);
(*_22) = (-56_i8) + 46_i8;
(*_22) = _7.1 as i8;
(*_22) = (-18_i8);
_14 = core::ptr::addr_of_mut!((*_19));
(*_22) = (-101_i8) & (-25_i8);
_26.0 = core::ptr::addr_of!(RET);
(*_22) = -(-62_i8);
(*_22) = 0_i8 * 102_i8;
(*_22) = 1562101834_i32 as i8;
_24 = &mut (*_22);
_7.2 = [444675716_i32];
(*_24) = 69_i8;
(*_24) = (-89_i8) - (-50_i8);
RET = 27531_i16;
_15 = _10;
_27 = 3977562697203925682_u64 as f32;
RET = 4689_i16 << (*_24);
(*_24) = (-77_i8) * (-122_i8);
_7.1 = !7_usize;
(*_24) = -(-105_i8);
_4 = &mut _26.0;
_9.1.1 = [4101496201_u32,2442376790_u32,3978380830_u32,3402082878_u32,1654888431_u32,3319323952_u32,3514414413_u32];
(*_24) = 84_i8 ^ (-92_i8);
Call(_7.1 = core::intrinsics::transmute(_2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_24) = 81_i8 * (-75_i8);
Goto(bb15)
}
bb15 = {
RET = -25898_i16;
(*_4) = core::ptr::addr_of!(RET);
_30 = _1;
(*_24) = (-108_i8) - (-43_i8);
Goto(bb16)
}
bb16 = {
RET = 3665_i16 & (-12268_i16);
(*_4) = core::ptr::addr_of!(RET);
_1 = _30;
_30 = _1;
_12 = (*_24) >= (*_24);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_24) = (-47_i8) & (-1_i8);
(*_4) = core::ptr::addr_of!(RET);
(*_24) = -66_i8;
_2 = !_10;
(*_24) = 79_i8 - (-89_i8);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_24) = !(-62_i8);
(*_24) = !(-1_i8);
_34 = 12065937027616725632_u64 >> (*_24);
_29 = _1;
_1 = _30;
Goto(bb17)
}
bb17 = {
(*_4) = core::ptr::addr_of!(RET);
_24 = Move(_22);
RET = 5566_i16 + (-17860_i16);
_34 = _12 as u64;
_7.1 = 7_usize | 1_usize;
(*_4) = core::ptr::addr_of!(RET);
_36 = _7.0 as f64;
_2 = _10 >> _10;
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_29 = _1;
Goto(bb18)
}
bb18 = {
_12 = _2 > _15;
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_1 = _30;
_12 = _2 < _2;
_27 = _7.1 as f32;
_37 = -_27;
_34 = !1858574753025553295_u64;
(*_4) = core::ptr::addr_of!(RET);
_39 = core::ptr::addr_of_mut!(RET);
_1 = _30;
(*_4) = core::ptr::addr_of!((*_39));
(*_39) = (-2377_i16) ^ 13436_i16;
_19 = core::ptr::addr_of_mut!((*_14));
_1 = _29;
(*_39) = (-22203_i16) >> _7.0;
(*_39) = 14757_i16;
(*_4) = core::ptr::addr_of!((*_39));
Goto(bb19)
}
bb19 = {
_19 = core::ptr::addr_of_mut!((*_14));
(*_4) = core::ptr::addr_of!((*_39));
_3 = [65_i8,(-35_i8),(-96_i8),(-46_i8),119_i8,(-19_i8),(-27_i8),88_i8];
_43 = 146_u8;
_7.0 = 61981546414885649690759452458549198193_u128 << (*_39);
_17 = &mut _39;
(*_17) = core::ptr::addr_of_mut!(_41);
_11 = 124711921585666141463279385839996830690_i128 & (-38290444348962114857928878269066359842_i128);
_41 = RET;
(*_17) = core::ptr::addr_of_mut!(_41);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_15 = _10;
_12 = false ^ true;
_9.0 = Adt41::Variant1 { fld0: _12,fld1: _34,fld2: _36,fld3: 58_i8,fld4: 449171740_u32 };
match _43 {
0 => bb17,
1 => bb14,
2 => bb3,
3 => bb5,
146 => bb21,
_ => bb20
}
}
bb20 = {
_12 = _2 > _15;
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_1 = _30;
_12 = _2 < _2;
_27 = _7.1 as f32;
_37 = -_27;
_34 = !1858574753025553295_u64;
(*_4) = core::ptr::addr_of!(RET);
_39 = core::ptr::addr_of_mut!(RET);
_1 = _30;
(*_4) = core::ptr::addr_of!((*_39));
(*_39) = (-2377_i16) ^ 13436_i16;
_19 = core::ptr::addr_of_mut!((*_14));
_1 = _29;
(*_39) = (-22203_i16) >> _7.0;
(*_39) = 14757_i16;
(*_4) = core::ptr::addr_of!((*_39));
Goto(bb19)
}
bb21 = {
_48 = [(-4515323207837328634_i64),(-8505360803683871864_i64),(-2598496484202184429_i64)];
(*_4) = core::ptr::addr_of!(_41);
_3 = [56_i8,(-46_i8),16_i8,2_i8,(-41_i8),77_i8,(-83_i8),(-118_i8)];
(*_17) = core::ptr::addr_of_mut!(RET);
(*_17) = core::ptr::addr_of_mut!(RET);
(*_4) = core::ptr::addr_of!(RET);
_11 = 61182956665334812056916019789395226399_i128 * 152383205544166963921495592695589012144_i128;
_2 = _15 << _7.1;
(*_17) = core::ptr::addr_of_mut!(_49);
_7.2 = [1597560173_i32];
_38 = -(-8957211248093857057_i64);
RET = -_41;
(*_17) = core::ptr::addr_of_mut!(RET);
_11 = (-125694952394708639207226260565551160138_i128) + 155268967526525943970405351726414885065_i128;
_18 = &_43;
(*_17) = core::ptr::addr_of_mut!(RET);
(*_4) = core::ptr::addr_of!(_49);
_7.1 = 5347894478035010389_usize >> _15;
_37 = _27 - _27;
_47 = Field::<bool>(Variant(_9.0, 1), 0) | _12;
_50 = [(*_18),(*_18),(*_18),(*_18),(*_18),(*_18),(*_18)];
_1 = _29;
(*_17) = core::ptr::addr_of_mut!(RET);
(*_17) = core::ptr::addr_of_mut!(_41);
_14 = core::ptr::addr_of_mut!((*_14));
_44 = _34 * _34;
(*_17) = core::ptr::addr_of_mut!(_41);
match (*_18) {
0 => bb5,
1 => bb13,
2 => bb14,
146 => bb22,
_ => bb4
}
}
bb22 = {
(*_4) = core::ptr::addr_of!(RET);
(*_17) = core::ptr::addr_of_mut!(RET);
_33 = core::ptr::addr_of_mut!(_9.1.1);
(*_33) = [332783821_u32,1071187131_u32,1072611950_u32,4212232909_u32,3019242901_u32,3350959816_u32,2589195159_u32];
(*_4) = core::ptr::addr_of!(_49);
_9.1.0 = &_36;
(*_17) = core::ptr::addr_of_mut!(_41);
(*_19) = core::ptr::addr_of_mut!(_9.1.0);
_14 = core::ptr::addr_of_mut!((*_19));
_52.0 = [_29,_1,_1,_1];
_36 = Field::<f64>(Variant(_9.0, 1), 2) - Field::<f64>(Variant(_9.0, 1), 2);
(*_17) = core::ptr::addr_of_mut!(RET);
_51 = _38 >> (*_18);
(*_19) = core::ptr::addr_of_mut!((*_25));
Goto(bb23)
}
bb23 = {
(*_33) = [3872657369_u32,1778366413_u32,1475198850_u32,2996207191_u32,2528174825_u32,3368413696_u32,1406123284_u32];
(*_17) = core::ptr::addr_of_mut!(_41);
Goto(bb24)
}
bb24 = {
(*_25) = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_19) = core::ptr::addr_of_mut!((*_25));
_45 = _30;
(*_19) = core::ptr::addr_of_mut!((*_25));
_9.1.0 = &_36;
(*_25) = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_4) = core::ptr::addr_of!(_49);
(*_17) = core::ptr::addr_of_mut!(_41);
(*_19) = core::ptr::addr_of_mut!((*_25));
_29 = _45;
_2 = -_10;
(*_19) = core::ptr::addr_of_mut!((*_25));
_32 = _37 - _27;
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_17) = core::ptr::addr_of_mut!(RET);
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_4) = core::ptr::addr_of!(RET);
(*_25) = &_36;
(*_17) = core::ptr::addr_of_mut!(_49);
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_19) = core::ptr::addr_of_mut!((*_25));
_12 = _36 != Field::<f64>(Variant(_9.0, 1), 2);
_38 = _51 >> RET;
(*_25) = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_25) = &_36;
(*_25) = &place!(Field::<f64>(Variant(_9.0, 1), 2));
match (*_18) {
0 => bb10,
1 => bb11,
2 => bb25,
3 => bb26,
4 => bb27,
146 => bb29,
_ => bb28
}
}
bb25 = {
(*_4) = core::ptr::addr_of!(RET);
_24 = Move(_22);
RET = 5566_i16 + (-17860_i16);
_34 = _12 as u64;
_7.1 = 7_usize | 1_usize;
(*_4) = core::ptr::addr_of!(RET);
_36 = _7.0 as f64;
_2 = _10 >> _10;
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_29 = _1;
Goto(bb18)
}
bb26 = {
(*_16) = 63754_u16 | 27140_u16;
_7.0 = 76932796897558388584840041078552124518_u128 & 122910777221645384029975598437356418231_u128;
(*_16) = !54747_u16;
(*_16) = 28506_u16 << _10;
_2 = _10 & _10;
(*_16) = 33452_u16 & 54799_u16;
(*_16) = 435_u16 | 5512_u16;
(*_16) = 21606_u16;
(*_16) = 62595_u16;
(*_16) = 38756_u16 & 43963_u16;
_12 = true & true;
(*_16) = 5325_u16;
(*_16) = 57388_u16 << _2;
_2 = _10 << (*_16);
(*_16) = 44054_u16;
Goto(bb11)
}
bb27 = {
_7.0 = 320115436090530492543351772026086594750_u128 >> _15;
_3 = [(*_22),(*_22),(*_22),(*_22),(*_22),(*_22),(*_22),(*_22)];
_19 = core::ptr::addr_of_mut!(_25);
Goto(bb13)
}
bb28 = {
RET = !(-7343_i16);
_1 = '\u{4cb5e}';
_2 = (-9223372036854775808_isize);
_10 = _7.0 as isize;
_9.1.1 = [3742702607_u32,1722792379_u32,1080178645_u32,843029360_u32,3275512143_u32,1447935261_u32,4246603865_u32];
_2 = !_10;
_7.1 = 0_usize ^ 0_usize;
_7.1 = 0_usize + 1_usize;
_7.1 = 11676437392152287379_usize * 3_usize;
_7.2 = [(-1443861659_i32)];
_1 = '\u{f1e50}';
_7.2 = [985344935_i32];
RET = (-22803_i16) << _7.0;
_1 = '\u{adc32}';
_7.1 = 1_usize * 2_usize;
_11 = 4718352153664806184_u64 as i128;
Goto(bb3)
}
bb29 = {
(*_17) = core::ptr::addr_of_mut!(_49);
(*_25) = &_36;
Goto(bb30)
}
bb30 = {
(*_33) = [96239609_u32,2088321328_u32,2556464805_u32,604031792_u32,3388265738_u32,4279281161_u32,4143501909_u32];
_7.1 = !18108414179006322562_usize;
(*_4) = core::ptr::addr_of!(_49);
_40 = _41 as f32;
(*_17) = core::ptr::addr_of_mut!(_41);
(*_17) = core::ptr::addr_of_mut!(_49);
match _41 {
14757 => bb32,
_ => bb31
}
}
bb31 = {
(*_17) = core::ptr::addr_of_mut!(_49);
(*_25) = &_36;
Goto(bb30)
}
bb32 = {
_9.1.0 = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_33) = [771319106_u32,4074013485_u32,3921849693_u32,4010817141_u32,297284108_u32,1096491123_u32,1977392337_u32];
(*_4) = core::ptr::addr_of!(_49);
match (*_18) {
0 => bb20,
146 => bb33,
_ => bb16
}
}
bb33 = {
_9.3 = Adt41::Variant1 { fld0: _47,fld1: _44,fld2: Field::<f64>(Variant(_9.0, 1), 2),fld3: 58_i8,fld4: 2257024425_u32 };
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_17) = core::ptr::addr_of_mut!(_49);
(*_33) = [3501632509_u32,883212928_u32,1599182442_u32,158601844_u32,2913495597_u32,647410464_u32,391883659_u32];
(*_25) = &place!(Field::<f64>(Variant(_9.3, 1), 2));
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(_41);
(*_25) = &_36;
_59.0.fld3 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9.3, 1), 4)));
_49 = RET ^ _41;
_59.0.fld1 = !2880185039_u32;
_44 = Field::<u64>(Variant(_9.3, 1), 1) << _51;
_47 = _12 ^ Field::<bool>(Variant(_9.3, 1), 0);
place!(Field::<bool>(Variant(_9.0, 1), 0)) = _47 | _12;
place!(Field::<u64>(Variant(_9.3, 1), 1)) = _12 as u64;
Goto(bb34)
}
bb34 = {
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_17) = core::ptr::addr_of_mut!(_41);
(*_4) = core::ptr::addr_of!(_49);
(*_33) = [_59.0.fld1,_59.0.fld1,_59.0.fld1,_59.0.fld1,_59.0.fld1,_59.0.fld1,_59.0.fld1];
_36 = _10 as f64;
(*_4) = core::ptr::addr_of!(_41);
_11 = (-69644729340096698807008951841089980798_i128) + 19103196115038264075480576484310171353_i128;
(*_19) = core::ptr::addr_of_mut!(_9.1.0);
_11 = !(-144683207574602129214415680549690356288_i128);
_9.3 = Adt41::Variant1 { fld0: _47,fld1: _44,fld2: _36,fld3: (-45_i8),fld4: _59.0.fld1 };
place!(Field::<f64>(Variant(_9.0, 1), 2)) = _32 as f64;
(*_4) = core::ptr::addr_of!(_49);
_40 = _32;
place!(Field::<i8>(Variant(_9.0, 1), 3)) = 126_i8 + (-49_i8);
_45 = _1;
_55 = Field::<u64>(Variant(_9.3, 1), 1) <= Field::<u64>(Variant(_9.3, 1), 1);
_9.2 = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_4) = core::ptr::addr_of!(RET);
Goto(bb35)
}
bb35 = {
_30 = _45;
RET = _49;
_60.0 = [_30,_1,_45,_1];
(*_33) = [_59.0.fld1,Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),_59.0.fld1,_59.0.fld1,Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4)];
_30 = _29;
(*_17) = core::ptr::addr_of_mut!(_49);
_59.0.fld4 = 1372566916_i32 as usize;
_60 = (_52.0, (*_18), _52.0);
_1 = _45;
_52.3 = _45;
_49 = _32 as i16;
match (*_18) {
146 => bb37,
_ => bb36
}
}
bb36 = {
(*_33) = [3872657369_u32,1778366413_u32,1475198850_u32,2996207191_u32,2528174825_u32,3368413696_u32,1406123284_u32];
(*_17) = core::ptr::addr_of_mut!(_41);
Goto(bb24)
}
bb37 = {
(*_19) = core::ptr::addr_of_mut!(_9.1.0);
_1 = _30;
_48 = [_38,_51,_38];
_60.0 = [_45,_1,_45,_52.3];
_9.3 = Adt41::Variant1 { fld0: Field::<bool>(Variant(_9.0, 1), 0),fld1: _44,fld2: Field::<f64>(Variant(_9.0, 1), 2),fld3: Field::<i8>(Variant(_9.0, 1), 3),fld4: _59.0.fld1 };
_11 = _12 as i128;
(*_17) = core::ptr::addr_of_mut!(_49);
_46 = Field::<f64>(Variant(_9.0, 1), 2) as isize;
(*_19) = core::ptr::addr_of_mut!((*_25));
_64 = core::ptr::addr_of_mut!((*_33));
_65.2.0 = Adt23::Variant1 { fld0: _10 };
_65.2.0 = Adt23::Variant2 { fld0: 37314_u16,fld1: _11 };
place!(Field::<f64>(Variant(_9.3, 1), 2)) = _36 + _36;
Goto(bb38)
}
bb38 = {
_1 = _30;
match (*_18) {
0 => bb21,
1 => bb39,
2 => bb40,
3 => bb41,
4 => bb42,
146 => bb44,
_ => bb43
}
}
bb39 = {
(*_4) = core::ptr::addr_of!(RET);
_24 = Move(_22);
RET = 5566_i16 + (-17860_i16);
_34 = _12 as u64;
_7.1 = 7_usize | 1_usize;
(*_4) = core::ptr::addr_of!(RET);
_36 = _7.0 as f64;
_2 = _10 >> _10;
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(RET);
_29 = _1;
Goto(bb18)
}
bb40 = {
(*_17) = core::ptr::addr_of_mut!(_49);
(*_25) = &_36;
Goto(bb30)
}
bb41 = {
(*_33) = [96239609_u32,2088321328_u32,2556464805_u32,604031792_u32,3388265738_u32,4279281161_u32,4143501909_u32];
_7.1 = !18108414179006322562_usize;
(*_4) = core::ptr::addr_of!(_49);
_40 = _41 as f32;
(*_17) = core::ptr::addr_of_mut!(_41);
(*_17) = core::ptr::addr_of_mut!(_49);
match _41 {
14757 => bb32,
_ => bb31
}
}
bb42 = {
_13 = _7.0 as i8;
_7.1 = 11542297924273351906_usize + 5_usize;
_13 = -123_i8;
_2 = !_10;
_15 = -_10;
_10 = _15 ^ _15;
_16 = &mut _8;
(*_16) = 14780_u16;
(*_16) = !46300_u16;
(*_16) = !35787_u16;
(*_16) = 559099724_u32 as u16;
(*_16) = !62565_u16;
_11 = (-93357376863633909280375491964203083439_i128) - (-141274825185249757862037287146770856637_i128);
(*_16) = !24729_u16;
(*_16) = 57329_u16 | 33708_u16;
(*_16) = _13 as u16;
_2 = -_10;
(*_16) = 1911018465718920393_u64 as u16;
_9.1.1 = [1554159550_u32,141310168_u32,3810931266_u32,3397324535_u32,2202783010_u32,1969534813_u32,1031661457_u32];
_12 = !false;
(*_16) = 1794_u16 << _10;
Goto(bb5)
}
bb43 = {
_9.3 = Adt41::Variant1 { fld0: _47,fld1: _44,fld2: Field::<f64>(Variant(_9.0, 1), 2),fld3: 58_i8,fld4: 2257024425_u32 };
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_17) = core::ptr::addr_of_mut!(_49);
(*_33) = [3501632509_u32,883212928_u32,1599182442_u32,158601844_u32,2913495597_u32,647410464_u32,391883659_u32];
(*_25) = &place!(Field::<f64>(Variant(_9.3, 1), 2));
(*_4) = core::ptr::addr_of!(RET);
(*_4) = core::ptr::addr_of!(_41);
(*_25) = &_36;
_59.0.fld3 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9.3, 1), 4)));
_49 = RET ^ _41;
_59.0.fld1 = !2880185039_u32;
_44 = Field::<u64>(Variant(_9.3, 1), 1) << _51;
_47 = _12 ^ Field::<bool>(Variant(_9.3, 1), 0);
place!(Field::<bool>(Variant(_9.0, 1), 0)) = _47 | _12;
place!(Field::<u64>(Variant(_9.3, 1), 1)) = _12 as u64;
Goto(bb34)
}
bb44 = {
(*_33) = [_59.0.fld1,Field::<u32>(Variant(_9.3, 1), 4),_59.0.fld1,_59.0.fld1,_59.0.fld1,Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4)];
_65.0 = !62721_u16;
_16 = &mut _65.0;
(*_19) = core::ptr::addr_of_mut!((*_25));
_60.2 = [_30,_45,_52.3,_52.3];
_33 = core::ptr::addr_of_mut!((*_33));
(*_16) = _7.1 as u16;
(*_33) = [_59.0.fld1,Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),_59.0.fld1,_59.0.fld1,_59.0.fld1];
match (*_18) {
0 => bb41,
1 => bb2,
2 => bb16,
3 => bb45,
4 => bb46,
5 => bb47,
146 => bb49,
_ => bb48
}
}
bb45 = {
(*_16) = 63754_u16 | 27140_u16;
_7.0 = 76932796897558388584840041078552124518_u128 & 122910777221645384029975598437356418231_u128;
(*_16) = !54747_u16;
(*_16) = 28506_u16 << _10;
_2 = _10 & _10;
(*_16) = 33452_u16 & 54799_u16;
(*_16) = 435_u16 | 5512_u16;
(*_16) = 21606_u16;
(*_16) = 62595_u16;
(*_16) = 38756_u16 & 43963_u16;
_12 = true & true;
(*_16) = 5325_u16;
(*_16) = 57388_u16 << _2;
_2 = _10 << (*_16);
(*_16) = 44054_u16;
Goto(bb11)
}
bb46 = {
_13 = _7.0 as i8;
_7.1 = 11542297924273351906_usize + 5_usize;
_13 = -123_i8;
_2 = !_10;
_15 = -_10;
_10 = _15 ^ _15;
_16 = &mut _8;
(*_16) = 14780_u16;
(*_16) = !46300_u16;
(*_16) = !35787_u16;
(*_16) = 559099724_u32 as u16;
(*_16) = !62565_u16;
_11 = (-93357376863633909280375491964203083439_i128) - (-141274825185249757862037287146770856637_i128);
(*_16) = !24729_u16;
(*_16) = 57329_u16 | 33708_u16;
(*_16) = _13 as u16;
_2 = -_10;
(*_16) = 1911018465718920393_u64 as u16;
_9.1.1 = [1554159550_u32,141310168_u32,3810931266_u32,3397324535_u32,2202783010_u32,1969534813_u32,1031661457_u32];
_12 = !false;
(*_16) = 1794_u16 << _10;
Goto(bb5)
}
bb47 = {
(*_33) = [3872657369_u32,1778366413_u32,1475198850_u32,2996207191_u32,2528174825_u32,3368413696_u32,1406123284_u32];
(*_17) = core::ptr::addr_of_mut!(_41);
Goto(bb24)
}
bb48 = {
(*_17) = core::ptr::addr_of_mut!(_49);
(*_25) = &_36;
Goto(bb30)
}
bb49 = {
_60 = (_52.0, (*_18), _52.0);
_11 = (-9172281469067635721092078295929356971_i128) ^ (-1726845743261685895095989380521016079_i128);
_41 = 2114211049_i32 as i16;
place!(Field::<u32>(Variant(_9.3, 1), 4)) = _11 as u32;
_7.1 = _59.0.fld4 ^ _59.0.fld4;
(*_4) = core::ptr::addr_of!(RET);
_57 = _29;
(*_19) = core::ptr::addr_of_mut!((*_25));
_69.1 = &mut (*_16);
place!(Field::<u32>(Variant(_9.0, 1), 4)) = !Field::<u32>(Variant(_9.3, 1), 4);
_68 = [Field::<bool>(Variant(_9.3, 1), 0),_47];
(*_33) = [Field::<u32>(Variant(_9.0, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.0, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4),Field::<u32>(Variant(_9.3, 1), 4)];
_45 = _52.3;
_11 = 2608177810831255819547838224884135597_i128 & 75662661580009908176594911395437931459_i128;
(*_25) = &place!(Field::<f64>(Variant(_9.3, 1), 2));
(*_25) = &_36;
_7.0 = 74167844572397023011354204216860706361_u128 << Field::<i8>(Variant(_9.0, 1), 3);
_70 = &mut (*_18);
_60.2 = [_52.3,_30,_45,_52.3];
(*_25) = &place!(Field::<f64>(Variant(_9.0, 1), 2));
(*_19) = core::ptr::addr_of_mut!((*_25));
(*_4) = core::ptr::addr_of!(RET);
_52.1 = [(*_70),(*_70)];
_75.fld4 = !_7.1;
Goto(bb50)
}
bb50 = {
Call(_78 = dump_var(Move(_3), Move(_15), Move(_55), Move(_41)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_78 = dump_var(Move(_43), Move(_2), Move(_11), Move(_48)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_78 = dump_var(Move(_44), Move(_29), Move(_8), Move(_50)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_78 = dump_var(Move(_7), Move(_60), _79, _79), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: u16,mut _3: u16,mut _4: isize,mut _5: &'static mut u16,mut _6: u16,mut _7: (u128, usize, [i32; 1]),mut _8: char) -> i8 {
mir! {
type RET = i8;
let _9: char;
let _10: (u16, &'static mut i8, (Adt23, u128), *mut &'static f64);
let _11: isize;
let _12: i8;
let _13: isize;
let _14: &'static bool;
let _15: u8;
let _16: (Adt23, u128);
let _17: Adt44;
let _18: &'static mut isize;
let _19: &'static mut *mut char;
let _20: bool;
let _21: *mut ([char; 4], u8, [char; 4]);
let _22: &'static mut u8;
let _23: [u128; 2];
let _24: usize;
let _25: &'static mut u16;
let _26: char;
let _27: Adt75;
let _28: &'static mut *const i16;
let _29: Adt43;
let _30: char;
let _31: *mut &'static mut u8;
let _32: u32;
let _33: isize;
let _34: isize;
let _35: &'static mut u8;
let _36: &'static mut i8;
let _37: &'static f32;
let _38: (*const i16,);
let _39: *mut *mut &'static f64;
let _40: &'static i32;
let _41: ([char; 4], [u8; 2], &'static mut isize, char);
let _42: u8;
let _43: i64;
let _44: i16;
let _45: char;
let _46: i64;
let _47: Adt55;
let _48: [u64; 7];
let _49: i8;
let _50: Adt44;
let _51: isize;
let _52: u64;
let _53: (*mut &'static f64, &'static mut isize);
let _54: f64;
let _55: f64;
let _56: [i32; 1];
let _57: f32;
let _58: *const u128;
let _59: usize;
let _60: f32;
let _61: f32;
let _62: f32;
let _63: f64;
let _64: *mut &'static mut u8;
let _65: (*const i16,);
let _66: &'static i32;
let _67: char;
let _68: f64;
let _69: f32;
let _70: [u8; 7];
let _71: ();
let _72: ();
{
_2 = _3;
_7.1 = 3_usize - 6_usize;
_3 = _2;
RET = -(-116_i8);
_8 = '\u{b6e53}';
_7.2 = [(-639101440_i32)];
_1 = _4 << _4;
_7.1 = 8566677089296939317_i64 as usize;
_7.1 = 1_usize * 4862485378755748379_usize;
_9 = _8;
_3 = _2;
_10.0 = !_2;
_8 = _9;
_10.2.1 = _7.0;
_10.1 = &mut RET;
_10.0 = _2 - _6;
_5 = &mut _10.0;
(*_5) = _2 >> _1;
_7.1 = 2_usize * 6351334993850784140_usize;
(*_5) = _2;
_13 = _1;
_4 = _13;
(*_5) = _2 << _4;
Goto(bb1)
}
bb1 = {
(*_5) = !_6;
_11 = _4 & _13;
(*_5) = _3;
_13 = _7.0 as isize;
_4 = _1 + _11;
(*_5) = _2 >> _4;
(*_5) = _6 >> _4;
(*_5) = _11 as u16;
(*_5) = _3;
(*_5) = !_6;
_8 = _9;
(*_5) = 254_u8 as u16;
(*_5) = _2 & _6;
(*_5) = _3 + _2;
_8 = _9;
(*_5) = _6;
(*_5) = _2 - _6;
_5 = &mut _6;
_12 = 8_i8 * (-8_i8);
_2 = 19298_i16 as u16;
(*_5) = 10288_i16 as u16;
_5 = &mut _2;
_9 = _8;
Call((*_5) = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb3 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb4 = {
place!(Field::<u16>(Variant(_16.0, 2), 0)) = !_3;
(*_5) = !Field::<u16>(Variant(_16.0, 2), 0);
_8 = _9;
_18 = &mut _4;
(*_5) = Field::<u16>(Variant(_16.0, 2), 0);
(*_18) = (*_5) as isize;
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) * Field::<u16>(Variant(_16.0, 2), 0);
(*_5) = _3 ^ Field::<u16>(Variant(_16.0, 2), 0);
_8 = _9;
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - _3;
(*_18) = -_11;
(*_18) = -_11;
_8 = _9;
(*_18) = Field::<i128>(Variant(_16.0, 2), 1) as isize;
_24 = !_7.1;
_7.1 = (*_5) as usize;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 90967160640561868460027140603626452729_i128 << (*_5);
(*_5) = !Field::<u16>(Variant(_16.0, 2), 0);
_8 = _9;
(*_18) = !_1;
match _7.0 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
161235668802368273983664633528775268895 => bb13,
_ => bb12
}
}
bb5 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb6 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb7 = {
(*_5) = !_6;
_11 = _4 & _13;
(*_5) = _3;
_13 = _7.0 as isize;
_4 = _1 + _11;
(*_5) = _2 >> _4;
(*_5) = _6 >> _4;
(*_5) = _11 as u16;
(*_5) = _3;
(*_5) = !_6;
_8 = _9;
(*_5) = 254_u8 as u16;
(*_5) = _2 & _6;
(*_5) = _3 + _2;
_8 = _9;
(*_5) = _6;
(*_5) = _2 - _6;
_5 = &mut _6;
_12 = 8_i8 * (-8_i8);
_2 = 19298_i16 as u16;
(*_5) = 10288_i16 as u16;
_5 = &mut _2;
_9 = _8;
Call((*_5) = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
_25 = &mut (*_5);
(*_25) = _3;
_16.0 = Adt23::Variant2 { fld0: (*_25),fld1: (-169616760055562952620623850298757254556_i128) };
_24 = _20 as usize;
_23 = [_7.0,_7.0];
(*_18) = _1;
match _7.0 {
0 => bb14,
1 => bb15,
2 => bb16,
161235668802368273983664633528775268895 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb16 = {
(*_5) = !_6;
_11 = _4 & _13;
(*_5) = _3;
_13 = _7.0 as isize;
_4 = _1 + _11;
(*_5) = _2 >> _4;
(*_5) = _6 >> _4;
(*_5) = _11 as u16;
(*_5) = _3;
(*_5) = !_6;
_8 = _9;
(*_5) = 254_u8 as u16;
(*_5) = _2 & _6;
(*_5) = _3 + _2;
_8 = _9;
(*_5) = _6;
(*_5) = _2 - _6;
_5 = &mut _6;
_12 = 8_i8 * (-8_i8);
_2 = 19298_i16 as u16;
(*_5) = 10288_i16 as u16;
_5 = &mut _2;
_9 = _8;
Call((*_5) = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb18 = {
(*_25) = _3 << _13;
(*_25) = !_3;
_16.0 = Adt23::Variant2 { fld0: (*_25),fld1: 147259732561710024304896247559352035571_i128 };
_16.0 = Adt23::Variant2 { fld0: (*_25),fld1: (-22676982587449630598996921680966630024_i128) };
(*_18) = _12 as isize;
_16.1 = _12 as u128;
(*_25) = Field::<u16>(Variant(_16.0, 2), 0);
(*_18) = _13;
(*_18) = _11;
(*_25) = _3 << (*_18);
_25 = &mut place!(Field::<u16>(Variant(_16.0, 2), 0));
(*_18) = _11;
(*_25) = _3 & _3;
_26 = _8;
(*_25) = _7.0 as u16;
_11 = (*_18);
(*_18) = _11;
(*_18) = _13 * _13;
(*_18) = _11 ^ _13;
(*_18) = !_11;
_26 = _9;
_30 = _9;
(*_25) = _3 * _3;
_30 = _9;
(*_25) = 12910773474331712637_u64 as u16;
_8 = _9;
(*_18) = 1029104795_u32 as isize;
_20 = !false;
match _7.0 {
0 => bb16,
1 => bb19,
161235668802368273983664633528775268895 => bb21,
_ => bb20
}
}
bb19 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb20 = {
Return()
}
bb21 = {
_8 = _26;
_32 = !2094250008_u32;
(*_25) = 3146365360397823733_i64 as u16;
(*_18) = _7.1 as isize;
(*_18) = _11 >> _32;
_24 = _7.1 + _7.1;
_5 = &mut (*_25);
(*_5) = !_3;
_13 = (*_18);
(*_18) = -_13;
_9 = _30;
_33 = (*_18) ^ (*_18);
(*_18) = (-1261139145_i32) as isize;
(*_18) = !_1;
_13 = -(*_18);
_18 = &mut _13;
_3 = !(*_5);
(*_18) = _33 * _33;
(*_5) = 77_u8 as u16;
(*_5) = !_3;
(*_5) = _32 as u16;
match _7.0 {
0 => bb15,
161235668802368273983664633528775268895 => bb23,
_ => bb22
}
}
bb22 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb23 = {
(*_5) = _3;
(*_18) = _33 * _33;
(*_5) = !_3;
_12 = 61_i8;
_7.1 = _20 as usize;
(*_18) = _33;
(*_5) = _7.0 as u16;
(*_5) = _3;
_34 = (*_18) | (*_18);
_36 = &mut _12;
(*_18) = _34 - _33;
_9 = _30;
(*_5) = _3;
(*_5) = _3 << (*_18);
(*_5) = (-2982748416491372781_i64) as u16;
_9 = _26;
(*_18) = 7203558971015001283_u64 as isize;
(*_5) = !_3;
(*_18) = _33;
(*_36) = (-85_i8) & (-25_i8);
(*_5) = _3 + _3;
(*_18) = (*_36) as isize;
(*_18) = _9 as isize;
match _7.0 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb17,
4 => bb9,
5 => bb22,
6 => bb24,
161235668802368273983664633528775268895 => bb26,
_ => bb25
}
}
bb24 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb25 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb26 = {
(*_5) = !_3;
(*_5) = _3 & _3;
(*_5) = _3 << _34;
(*_5) = _3;
(*_18) = _1;
(*_18) = _20 as isize;
(*_18) = _34;
(*_5) = _3 + _3;
_15 = 7_u8;
(*_18) = _33 * _11;
_15 = _9 as u8;
_18 = &mut _11;
(*_18) = _34 * _33;
(*_18) = _33 & _33;
(*_18) = 856002323419553404_u64 as isize;
(*_5) = _3 & _3;
_23 = [_7.0,_7.0];
(*_18) = _34;
_18 = &mut _34;
(*_36) = (-53_i8);
(*_18) = (*_36) as isize;
_26 = _8;
(*_18) = _33 | _33;
_7.2 = [(-1330519487_i32)];
(*_5) = _3 + _3;
match (*_36) {
0 => bb3,
1 => bb27,
2 => bb28,
3 => bb29,
340282366920938463463374607431768211403 => bb31,
_ => bb30
}
}
bb27 = {
Return()
}
bb28 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb29 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb30 = {
_8 = _26;
_32 = !2094250008_u32;
(*_25) = 3146365360397823733_i64 as u16;
(*_18) = _7.1 as isize;
(*_18) = _11 >> _32;
_24 = _7.1 + _7.1;
_5 = &mut (*_25);
(*_5) = !_3;
_13 = (*_18);
(*_18) = -_13;
_9 = _30;
_33 = (*_18) ^ (*_18);
(*_18) = (-1261139145_i32) as isize;
(*_18) = !_1;
_13 = -(*_18);
_18 = &mut _13;
_3 = !(*_5);
(*_18) = _33 * _33;
(*_5) = 77_u8 as u16;
(*_5) = !_3;
(*_5) = _32 as u16;
match _7.0 {
0 => bb15,
161235668802368273983664633528775268895 => bb23,
_ => bb22
}
}
bb31 = {
(*_5) = _3;
_33 = -(*_18);
(*_5) = _3 | _3;
(*_36) = 140676162498971049509316777762393868242_i128 as i8;
(*_18) = !_33;
(*_5) = !_3;
(*_5) = _3 & _3;
(*_36) = -(-21_i8);
(*_36) = (-31_i8);
match (*_36) {
0 => bb15,
340282366920938463463374607431768211425 => bb33,
_ => bb32
}
}
bb32 = {
(*_5) = (-24135_i16) as u16;
_13 = !_11;
_9 = _8;
(*_5) = _3 ^ _3;
Goto(bb3)
}
bb33 = {
(*_36) = 106_i8 | 64_i8;
(*_5) = _3;
(*_18) = _33 << (*_5);
_8 = _26;
_18 = &mut _33;
(*_5) = !_3;
(*_36) = (-76_i8);
_24 = !_7.1;
(*_5) = _3 + _3;
(*_5) = !_3;
(*_18) = _1;
(*_36) = !127_i8;
_41.1 = [_15,_15];
(*_5) = !_3;
(*_5) = !_3;
_1 = (*_36) as isize;
Call((*_36) = core::intrinsics::bswap(55_i8), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_24 = !_7.1;
(*_5) = _3;
(*_5) = _3 * _3;
(*_18) = -_1;
(*_36) = 109_i8 - (-107_i8);
(*_5) = _3;
_7.2 = [494328316_i32];
_42 = _15;
(*_5) = _30 as u16;
_7.2 = [1370039671_i32];
(*_36) = (-121_i8) << (*_5);
(*_18) = _1;
Goto(bb35)
}
bb35 = {
(*_36) = 49_i8;
(*_5) = _3 >> _3;
(*_18) = -_1;
_15 = !_42;
(*_5) = _3 - _3;
(*_36) = !(-3_i8);
(*_5) = _3;
_1 = -(*_18);
(*_18) = _1 & _1;
Goto(bb36)
}
bb36 = {
(*_5) = (-149714459094745746607617672781971246610_i128) as u16;
(*_36) = 1084370738098636938_u64 as i8;
(*_36) = -(-127_i8);
(*_5) = _3;
_42 = _15;
_45 = _9;
(*_36) = 23_i8 & (-32_i8);
(*_18) = _1 >> (*_36);
match _7.0 {
0 => bb32,
1 => bb26,
2 => bb37,
3 => bb38,
161235668802368273983664633528775268895 => bb40,
_ => bb39
}
}
bb37 = {
_8 = _26;
_32 = !2094250008_u32;
(*_25) = 3146365360397823733_i64 as u16;
(*_18) = _7.1 as isize;
(*_18) = _11 >> _32;
_24 = _7.1 + _7.1;
_5 = &mut (*_25);
(*_5) = !_3;
_13 = (*_18);
(*_18) = -_13;
_9 = _30;
_33 = (*_18) ^ (*_18);
(*_18) = (-1261139145_i32) as isize;
(*_18) = !_1;
_13 = -(*_18);
_18 = &mut _13;
_3 = !(*_5);
(*_18) = _33 * _33;
(*_5) = 77_u8 as u16;
(*_5) = !_3;
(*_5) = _32 as u16;
match _7.0 {
0 => bb15,
161235668802368273983664633528775268895 => bb23,
_ => bb22
}
}
bb38 = {
_7.0 = 161235668802368273983664633528775268895_u128;
(*_5) = _3 << _12;
_16.0 = Adt23::Variant2 { fld0: (*_5),fld1: 143339139086719811142862462840853059566_i128 };
(*_5) = Field::<u16>(Variant(_16.0, 2), 0) - Field::<u16>(Variant(_16.0, 2), 0);
_20 = false | true;
(*_5) = !_3;
_12 = (-24126_i16) as i8;
place!(Field::<i128>(Variant(_16.0, 2), 1)) = 15781424732674720768698136155755302310_i128 - 16197956495834974873464968505746939349_i128;
Goto(bb4)
}
bb39 = {
Return()
}
bb40 = {
_20 = !false;
_46 = (-5234867652025357292_i64) ^ 3096904796981642398_i64;
_35 = &mut _42;
_7.0 = 310709505150823123077333822715562658844_u128 ^ 304787729862557067662995938119860320627_u128;
_1 = (*_18) | (*_18);
(*_35) = _15 + _15;
_7.2 = [(-2031721992_i32)];
(*_5) = _3;
_49 = 23645_i16 as i8;
(*_18) = _24 as isize;
(*_36) = _49;
(*_5) = !_3;
_14 = &_20;
Goto(bb41)
}
bb41 = {
(*_35) = !_15;
Call((*_5) = core::intrinsics::bswap(_3), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
(*_18) = _1;
(*_35) = _3 as u8;
_41.0 = [_26,_45,_26,_26];
(*_35) = _15 + _15;
_44 = 31484_i16 << (*_18);
(*_35) = !_15;
(*_35) = _7.0 as u8;
_48 = [17925810096035258486_u64,13475172751080303516_u64,12683169565041869710_u64,17070175301513925060_u64,2376793590542747068_u64,47195678476659810_u64,1688732969860232203_u64];
(*_36) = _49;
(*_18) = -_1;
_41.1 = [(*_35),(*_35)];
_7.0 = 236708040100337088607893961547735376225_u128 | 339212840721402658821343896128787540724_u128;
(*_18) = _1 + _1;
_49 = _46 as i8;
(*_5) = _32 as u16;
_54 = (*_35) as f64;
Goto(bb43)
}
bb43 = {
(*_18) = _1 | _1;
_30 = _26;
_41.0 = [_26,_30,_9,_30];
_25 = &mut _3;
(*_25) = 15474885592119318218_u64 as u16;
(*_35) = !_15;
(*_18) = _1 & _1;
(*_25) = (*_5);
(*_35) = _15;
(*_36) = _49;
(*_5) = !(*_25);
_46 = -(-7524417011651431_i64);
(*_5) = (*_25) & (*_25);
(*_18) = _1 | _1;
_39 = core::ptr::addr_of_mut!(_53.0);
(*_35) = !_15;
_32 = !626615911_u32;
_54 = 3764224371396715986_u64 as f64;
_54 = _7.0 as f64;
(*_18) = _44 as isize;
(*_36) = _49 * _49;
(*_25) = (*_5) ^ (*_5);
(*_25) = !(*_5);
(*_35) = _9 as u8;
_56 = _7.2;
_38.0 = core::ptr::addr_of!(_44);
(*_18) = (*_14) as isize;
Goto(bb44)
}
bb44 = {
_43 = _46 ^ _46;
_41.3 = _45;
(*_36) = _49 >> (*_25);
(*_18) = _1 << _7.0;
(*_35) = (*_18) as u8;
(*_25) = _1 as u16;
_15 = !(*_35);
(*_25) = (*_5);
(*_36) = _49;
(*_35) = _15 - _15;
_46 = _43;
(*_18) = _1 * _1;
Goto(bb45)
}
bb45 = {
_41.2 = &mut _1;
(*_35) = _15 + _15;
(*_18) = !(-9223372036854775808_isize);
_32 = !1381581312_u32;
(*_25) = (*_5) >> (*_5);
(*_25) = (*_5) + (*_5);
_24 = _7.1 | _7.1;
_58 = core::ptr::addr_of!(_7.0);
(*_58) = 173013278419065477933122159083018506177_u128;
(*_25) = _24 as u16;
(*_36) = _49 >> (*_35);
Goto(bb46)
}
bb46 = {
_53.1 = &mut (*_18);
(*_35) = _15;
(*_36) = _49 << (*_5);
_51 = (-9223372036854775808_isize);
(*_25) = (*_5) * (*_5);
_31 = core::ptr::addr_of_mut!(_35);
_35 = &mut _15;
_36 = &mut _49;
(*_58) = 195764041346244062063641654940159934022_u128;
(*_36) = (-117_i8) * (-88_i8);
(*_25) = (*_36) as u16;
(*_5) = !(*_25);
_8 = _9;
(*_25) = (*_5) | (*_5);
_7.1 = !_24;
(*_36) = !(-32_i8);
(*_5) = (*_25);
(*_36) = _7.1 as i8;
_54 = _24 as f64;
(*_58) = !45369580610932242529792279693672065407_u128;
(*_36) = 50_i8 ^ 93_i8;
(*_5) = !(*_25);
(*_36) = -(-40_i8);
Goto(bb47)
}
bb47 = {
(*_35) = 183_u8;
(*_25) = (*_5);
(*_58) = 7565389912862611102669439749760018212_u128 >> (*_25);
(*_35) = _51 as u8;
(*_25) = (*_5) & (*_5);
(*_36) = (-89_i8);
(*_25) = !(*_5);
Call((*_58) = core::intrinsics::transmute(_41.0), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
_63 = _54 * _54;
(*_25) = !(*_5);
_20 = true;
_22 = &mut (*_35);
(*_25) = (*_5) + (*_5);
_28 = &mut _38.0;
(*_36) = -(-87_i8);
(*_25) = (*_5);
_58 = core::ptr::addr_of!((*_58));
(*_28) = core::ptr::addr_of!(_44);
(*_22) = _63 as u8;
(*_28) = core::ptr::addr_of!(_44);
Call((*_58) = core::intrinsics::transmute(_41.0), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_5 = &mut (*_25);
(*_28) = core::ptr::addr_of!(_44);
(*_22) = !182_u8;
(*_28) = core::ptr::addr_of!(_44);
_65 = (Move((*_28)),);
(*_22) = (*_58) as u8;
(*_5) = 40288_u16 >> _46;
(*_36) = (-61_i8);
(*_36) = (-64_i8) << _46;
(*_58) = 106701123591947911371163080047293302608_u128 | 265178261759718857243112165754664675174_u128;
(*_5) = 14250_u16 << _44;
(*_58) = !251787269889434855557025326619540064520_u128;
(*_22) = 235_u8 << (*_58);
(*_22) = 200_u8 + 30_u8;
(*_5) = 12803_u16 - 7878_u16;
_59 = _24 >> (*_22);
(*_58) = 48314058279124755600782535595747593572_u128;
(*_36) = -(-118_i8);
(*_31) = Move(_22);
(*_5) = 2561_u16;
_67 = _45;
(*_5) = !53226_u16;
(*_58) = !269601166767823742353484168640925791567_u128;
(*_36) = (-97_i8) + (-19_i8);
(*_58) = 76846919252044149868959433341505478716_u128 << _44;
_9 = _41.3;
_45 = _30;
_44 = (-24036_i16);
Goto(bb50)
}
bb50 = {
Call(_71 = dump_var(Move(_43), Move(_56), Move(_49), Move(_20)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_71 = dump_var(Move(_15), Move(_51), Move(_30), Move(_7)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_71 = dump_var(Move(_8), Move(_32), Move(_46), Move(_24)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_71 = dump_var(Move(_1), Move(_2), Move(_4), _72), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(88_u8), std::hint::black_box(59907_u16), std::hint::black_box(231255479429240907168300003444997420089_u128), std::hint::black_box(84_i8), std::hint::black_box(6_usize));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt23 {
Variant0{
fld0: u32,
fld1: f64,
fld2: isize,

},
Variant1{
fld0: isize,

},
Variant2{
fld0: u16,
fld1: i128,

}}
#[derive(Debug)]
pub struct Adt28 {
fld0: u16,
fld1: u32,
fld2: f32,
fld3: *mut u32,
fld4: usize,
}
#[derive(Debug)]
pub enum Adt41 {
Variant0{
fld0: *mut u32,
fld1: *const char,
fld2: ([char; 4], u8, [char; 4]),
fld3: *mut char,
fld4: u8,
fld5: u32,

},
Variant1{
fld0: bool,
fld1: u64,
fld2: f64,
fld3: i8,
fld4: u32,

}}
#[derive(Debug)]
pub enum Adt43 {
Variant0{
fld0: Adt28,
fld1: *const u128,
fld2: u16,
fld3: [i128; 2],
fld4: [u128; 2],
fld5: ([char; 4], u8, [char; 4]),
fld6: i64,
fld7: *mut i16,

},
Variant1{
fld0: u16,
fld1: [char; 4],
fld2: *mut char,
fld3: *const i16,

},
Variant2{
fld0: u8,
fld1: usize,
fld2: *mut i16,

},
Variant3{
fld0: *mut u32,
fld1: f32,
fld2: Adt23,
fld3: i8,

}}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: bool,
fld1: [i128; 2],
fld2: Adt28,
fld3: i8,
fld4: i16,
fld5: [char; 4],
fld6: u64,
fld7: f32,

},
Variant1{
fld0: [i32; 1],
fld1: u128,
fld2: *const char,
fld3: f64,

}}
#[derive(Debug)]
pub struct Adt50 {
fld0: [char; 4],
fld1: [i32; 1],
fld2: Adt44,
}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: *const char,
fld1: u64,
fld2: u128,

},
Variant1{
fld0: *mut char,
fld1: *mut u32,
fld2: isize,
fld3: u16,
fld4: i16,

}}
#[derive(Debug)]
pub enum Adt75 {
Variant0{
fld0: [u32; 7],
fld1: [i8; 8],
fld2: Adt43,
fld3: [i32; 1],

},
Variant1{
fld0: *const u128,
fld1: [i32; 1],
fld2: Adt55,
fld3: *mut u32,
fld4: Adt44,
fld5: [bool; 2],

},
Variant2{
fld0: (*const i16,),
fld1: Adt23,

}}

