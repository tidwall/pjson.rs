#![allow(unused)]

// Copyright 2021 Joshua J Baker. All rights reserved.
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file.

// Bit flags passed to the "info" parameter of the iter function which
// provides additional information about the data

/// the data is a JSON String
pub const STRING: usize = 1 << 1;
/// the data is a JSON Number
pub const NUMBER: usize = 1 << 2;
/// the data is a JSON True
pub const TRUE: usize = 1 << 3;
/// the data is a JSON False
pub const FALSE: usize = 1 << 4;
/// the data is a JSON NUll
pub const NULL: usize = 1 << 5;
/// the data is a JSON Object (open or close character)
pub const OBJECT: usize = 1 << 6;
/// the data is a JSON Array (open or close character)
pub const ARRAY: usize = 1 << 7;
/// the data is a JSON comma character ','
pub const COMMA: usize = 1 << 8;
/// the data is a JSON colon character ':'
pub const COLON: usize = 1 << 9;
/// the data is the start of the JSON document
pub const START: usize = 1 << 10;
/// the data is the end of the JSON document
pub const END: usize = 1 << 11;
/// the data is an open character (Object or Array, '{' or '[')
pub const OPEN: usize = 1 << 12;
/// the data is an close character (Object or Array, '}' or ']')
pub const CLOSE: usize = 1 << 13;
/// the data is a JSON Object key
pub const KEY: usize = 1 << 14;
/// the data is a JSON Object or Array value
pub const VALUE: usize = 1 << 15;
/// the data is a String with at least one escape character ('\')
pub const ESCAPED: usize = 1 << 16;
/// the data is a signed Number (has a '-' prefix)
pub const SIGN: usize = 1 << 17;
/// the data is a Number has a dot (radix point)
pub const DOT: usize = 1 << 18;
/// the data is a Number in scientific notation (has 'E' or 'e')
pub const E: usize = 1 << 19;

pub const UNCHECKED: usize = 1 << 1;

/// Parse JSON. The iter function is a callback that fires for every element in
/// the JSON document. Elements include all values and tokens. The 'start' and
/// 'end' params are the start and end indexes of their respective element,
/// such that json[start..end] will equal the complete element data. The 'info'
/// param provides extra information about the element data.
///
/// Returning 0 from 'iter' will stop the parsing.
/// Returning 1 from 'iter' will continue the parsing.
/// Returning -1 from 'iter' will skip all children elements in the
/// current Object or Array, which only applies when the 'info' for current
/// element has the Open bit set, otherwise it effectively works like
/// returning 1.
///
/// This operation returns zero or a negative value if an error
/// occured. This value represents the position that the parser was at when it
/// discovered the error. To get the true offset multiple this value by -1.
///
/// This operation returns a positive value when successful. If the 'iter'
/// stopped early then this value will be the position the parser was at when
/// it stopped, otherwise the value will be equal the length of the original
/// json document.
///
/// The following example prints every JSON String Value in the document:
///
/// ```
/// fn main() {
///     let json = br#"
///     {
///       "name": {"first": "Tom", "last": "Anderson"},
///       "age":37,
///       "children": ["Sara","Alex","Jack"],
///       "fav.movie": "Deer Hunter",
///       "friends": [
///     	{"first": "Dale", "last": "Murphy", "age": 44, "nets": ["ig", "fb", "tw"]},
///     	{"first": "Roger", "last": "Craig", "age": 68, "nets": ["fb", "tw"]},
///     	{"first": "Jane", "last": "Murphy", "age": 47, "nets": ["ig", "tw"]}
///       ]
///     }
///    "#;
///
///    pjson::parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
///        if info & (pjson::STRING | pjson::VALUE) == pjson::STRING | pjson::VALUE {
///            let el = String::from_utf8(json[start..end].to_vec()).unwrap();
///            println!("{}", el);
///        }
///        1
///    });
/// }
/// // output:
/// // "Tom"
/// // "Anderson"
/// // "Sara"
/// // "Alex"
/// // "Jack"
/// // "Deer Hunter"
/// // "Dale"
/// // "Murphy"
/// // "ig"
/// // "fb"
/// // "tw"
/// // "Roger"
/// // "Craig"
/// // "fb"
/// // "tw"
/// // "Jane"
/// // "Murphy"
/// // "ig"
/// // "tw"
/// ```
pub fn parse<F>(json: &[u8], opts: usize, iter: F) -> i64
where
    F: FnMut(usize, usize, usize) -> i64,
{
    let mut f = iter;
    let (i, ok, _) = vdoc(json, 0, opts, &mut f, false);
    if !ok {
        i as i64 * -1
    } else {
        i as i64
    }
}

const CHWS: u8 = 1 << 1;
const CHNUM: u8 = 1 << 2;
const CHSTRTOK: u8 = 1 << 3;
const CHSQUASH: u8 = 1 << 4;
const CHOPEN: u8 = 1 << 5;
const CHCLOSE: u8 = 1 << 6;

static CHTABLE: [u8; 256] = {
    let mut table = [0; 256];
    table[b'\t' as usize] |= CHWS;
    table[b'\n' as usize] |= CHWS;
    table[b'\r' as usize] |= CHWS;
    table[b' ' as usize] |= CHWS;

    table[b'0' as usize] |= CHNUM;
    table[b'1' as usize] |= CHNUM;
    table[b'2' as usize] |= CHNUM;
    table[b'3' as usize] |= CHNUM;
    table[b'4' as usize] |= CHNUM;
    table[b'5' as usize] |= CHNUM;
    table[b'6' as usize] |= CHNUM;
    table[b'7' as usize] |= CHNUM;
    table[b'8' as usize] |= CHNUM;
    table[b'9' as usize] |= CHNUM;

    table[0x00] |= CHSTRTOK;
    table[0x01] |= CHSTRTOK;
    table[0x02] |= CHSTRTOK;
    table[0x03] |= CHSTRTOK;
    table[0x04] |= CHSTRTOK;
    table[0x05] |= CHSTRTOK;
    table[0x06] |= CHSTRTOK;
    table[0x07] |= CHSTRTOK;
    table[0x08] |= CHSTRTOK;
    table[0x09] |= CHSTRTOK;
    table[0x0A] |= CHSTRTOK;
    table[0x0B] |= CHSTRTOK;
    table[0x0C] |= CHSTRTOK;
    table[0x0D] |= CHSTRTOK;
    table[0x0E] |= CHSTRTOK;
    table[0x0F] |= CHSTRTOK;
    table[0x10] |= CHSTRTOK;
    table[0x11] |= CHSTRTOK;
    table[0x12] |= CHSTRTOK;
    table[0x13] |= CHSTRTOK;
    table[0x14] |= CHSTRTOK;
    table[0x15] |= CHSTRTOK;
    table[0x16] |= CHSTRTOK;
    table[0x17] |= CHSTRTOK;
    table[0x18] |= CHSTRTOK;
    table[0x19] |= CHSTRTOK;
    table[0x1A] |= CHSTRTOK;
    table[0x1B] |= CHSTRTOK;
    table[0x1C] |= CHSTRTOK;
    table[0x1D] |= CHSTRTOK;
    table[0x1E] |= CHSTRTOK;
    table[0x1F] |= CHSTRTOK;
    table[b'"' as usize] |= CHSTRTOK;
    table[b'\\' as usize] |= CHSTRTOK;


    table[b'"' as usize] |= CHSQUASH;
    table[b'{' as usize] |= CHSQUASH|CHOPEN;
    table[b'[' as usize] |= CHSQUASH|CHOPEN;
    table[b'}' as usize] |= CHSQUASH|CHCLOSE;
    table[b']' as usize] |= CHSQUASH|CHCLOSE;

    table
};

fn isws(ch: u8) -> bool {
    // ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r'
    CHTABLE[ch as usize] & CHWS == CHWS
}

fn isnum(ch: u8) -> bool {
    // ch >= b'0' && ch <= b'9'
    CHTABLE[ch as usize] & CHNUM == CHNUM
}

fn isstrtok(ch: u8) -> bool {
    // ch < b' ' || ch == b'"' || ch == b'\\'
    CHTABLE[ch as usize] & CHSTRTOK == CHSTRTOK
}

fn vdoc<F>(json: &[u8], i: usize, opts: usize, f: &mut F, skip: bool) -> (usize, bool, bool)
where
    F: FnMut(usize, usize, usize) -> i64,
{
    let (mut i, ok, stop) = vany(json, i, opts, START, f, skip);
    if stop {
        return (i, ok, stop);
    }
    while i < json.len() {
        if isws(json[i]) {
            i += 1;
            continue;
        }
        return (i, false, true);
    }
    return (i, true, false);
}

// squash an object or array and return the next index after the '{' or '['.
fn squash(json: &[u8], mut i: usize) -> usize {
    // opening character has been already parsed
    let mut depth = 1;
    let mut ch: usize = 0;
    'outer: loop {
        'tok: loop {
            while i + 8 < json.len() {
                for _ in 0..8 {
                    ch = unsafe { *json.get_unchecked(i) } as usize;
                    if CHTABLE[ch]&CHSQUASH == CHSQUASH {
                        break 'tok;
                    }
                    i += 1;
                }
            }
            while i < json.len() {
                ch = json[i] as usize;
                if CHTABLE[ch]&CHSQUASH == CHSQUASH {    
                    break 'tok;
                }
                i += 1;
            }
            break 'outer;
        }
        if ch as u8 == b'"' {
            i += 1;
            let s = i;
            loop {
                'quote: loop {
                    while i + 8 < json.len() {
                        for _ in 0..8 {
                            if unsafe { *json.get_unchecked(i) } == b'"' {
                                break 'quote;
                            }
                            i += 1;
                        }
                    }
                    while i < json.len() {
                        if json[i] == b'"' {
                            break 'quote;
                        }
                        i += 1;
                    }
                    break 'outer;
                }
                // look for an escaped slash
                if json[i-1] == b'\\' {
                    let mut n = 0;
                    let mut j = i - 2;
                    while j > s-1  {
                        if json[j] != b'\\' {
                            break;
                        }
                        n += 1;
                        j -= 1;
                    }
                    if n%2 == 0 {
                        i += 1;
                        continue;
                    }
                }
                break;
            }
        } else if CHTABLE[ch]&CHOPEN == CHOPEN {
            depth += 1;
        } else if CHTABLE[ch]&CHCLOSE == CHCLOSE {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
        i += 1;
    }
    return i;
}

fn vany<F>(
    json: &[u8],
    mut i: usize,
    opts: usize,
    mut dinfo: usize,
    f: &mut F,
    skip: bool,
) -> (usize, bool, bool)
where
    F: FnMut(usize, usize, usize) -> i64,
{
    while i < json.len() {
        if isws(json[i]) {
            i += 1;
            continue;
        }
        let mark = i;
        let mut info = 0;
        let ok;
        let stop;
        if json[i] == b'"' {
            let (i_, info_, ok_, stop_) = vstring(json, i + 1);
            i = i_;
            info = info_;
            ok = ok_;
            stop = stop_;
            info |= STRING;
        } else if json[i] == b'{' {
            let mut oskip = skip;
            if !skip {
                let r = f(i, i + 1, OBJECT | OPEN | dinfo);
                if r == 0 {
                    return (i, true, true);
                }
                if r == -1 {
                    oskip = true;
                }
            }
            if opts&UNCHECKED==UNCHECKED && oskip {
                i = squash(json, i + 1);
            } else {
                let (i_, ok_, stop_) = vobject(json, i + 1, opts, f, oskip);
                i = i_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
            }
            if !skip {
                if dinfo & START == START {
                    dinfo = dinfo ^ START; // TODO: IS THIS RIGHT? ¯\_(ツ)_/¯
                    dinfo |= END;
                }
                if f(i - 1, i, OBJECT | CLOSE | dinfo) == 0 {
                    return (i, true, true);
                }
            }
            return (i, true, false);
        } else if json[i] == b'[' {
            let mut oskip = skip;
            if !skip {
                let r = f(i, i + 1, ARRAY | OPEN | dinfo);
                if r == 0 {
                    return (i, true, true);
                }
                if r == -1 {
                    oskip = true;
                }
            }
            if opts&UNCHECKED==UNCHECKED && oskip {
                i = squash(json, i + 1);
            } else {
                let (i_, ok_, stop_) = varray(json, i + 1, opts, f, oskip);
                i = i_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
            }
            if !skip {
                if dinfo & START == START {
                    dinfo = dinfo ^ START; // TODO: IS THIS RIGHT? ¯\_(ツ)_/¯
                    dinfo |= END
                }
                if f(i - 1, i, ARRAY | CLOSE | dinfo) == 0 {
                    return (i, true, true);
                }
            }
            return (i, true, false);
        } else if json[i] == b'-' || isnum(json[i]) {
            let (i_, info_, ok_, stop_) = vnumber(json, i + 1);
            i = i_;
            info = info_;
            ok = ok_;
            stop = stop_;
            info |= NUMBER;
        } else if json[i] == b't' {
            let (i_, ok_, stop_) = vtrue(json, i + 1);
            i = i_;
            ok = ok_;
            stop = stop_;
            info |= TRUE;
        } else if json[i] == b'n' {
            let (i_, ok_, stop_) = vnull(json, i + 1);
            i = i_;
            ok = ok_;
            stop = stop_;
            info |= NULL;
        } else if json[i] == b'f' {
            let (i_, ok_, stop_) = vfalse(json, i + 1);
            i = i_;
            ok = ok_;
            stop = stop_;
            info |= FALSE;
        } else {
            return (i, false, true);
        }
        if stop {
            return (i, ok, stop);
        }
        if !skip {
            if dinfo & START == START {
                dinfo |= END;
            }
            if f(mark, i, info | dinfo) == 0 {
                return (i, true, true);
            }
        }
        return (i, ok, stop);
    }
    return (i, false, true);
}

fn vobject<F>(json: &[u8], mut i: usize, opts: usize, f: &mut F, skip: bool) -> (usize, bool, bool)
where
    F: FnMut(usize, usize, usize) -> i64,
{
    while i < json.len() {
        if isws(json[i]) {
            i += 1;
            continue;
        }
        if json[i] == b'}' {
            return (i + 1, true, false);
        }
        if json[i] == b'"' {
            'key: loop {
                let mark = i;
                let info;
                let mut ok;
                let mut stop;
                let (i_, info_, ok_, stop_) = vstring(json, i + 1);
                i = i_;
                info = info_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
                if !skip {
                    if f(mark, i, info | KEY | STRING) == 0 {
                        return (i, true, true);
                    }
                }
                let (i_, ok_, stop_) = vcolon(json, i);
                i = i_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
                if !skip {
                    if f(i - 1, i, COLON) == 0 {
                        return (i, true, true);
                    }
                }
                let (i_, ok_, stop_) = vany(json, i, opts, VALUE, f, skip);
                i = i_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
                let (i_, ok_, stop_) = vcomma(json, i, b'}');
                i = i_;
                ok = ok_;
                stop = stop_;
                if stop {
                    return (i, ok, stop);
                }
                if json[i] == b'}' {
                    return (i + 1, true, false);
                }
                if !skip {
                    if f(i, i + 1, COMMA) == 0 {
                        return (i, true, true);
                    }
                }
                i += 1;
                while i < json.len() {
                    if isws(json[i]) {
                        i += 1;
                        continue;
                    }
                    if json[i] == b'"' {
                        continue 'key;
                    }
                    break;
                }
                break;
            }
        }
        break;
    }
    return (i, false, true);
}

fn varray<F>(json: &[u8], mut i: usize, opts: usize, f: &mut F, skip: bool) -> (usize, bool, bool)
where
    F: FnMut(usize, usize, usize) -> i64,
{
    while i < json.len() {
        if isws(json[i]) {
            i += 1;
            continue;
        }
        if json[i] == b']' {
            return (i + 1, true, false);
        }
        while i < json.len() {
            if isws(json[i]) {
                i += 1;
                continue;
            }
            let mut ok;
            let mut stop;
            let (i_, ok_, stop_) = vany(json, i, opts, VALUE, f, skip);
            i = i_;
            ok = ok_;
            stop = stop_;
            if stop {
                return (i, ok, stop);
            }
            let (i_, ok_, stop_) = vcomma(json, i, b']');
            i = i_;
            ok = ok_;
            stop = stop_;
            if stop {
                return (i, ok, stop);
            }
            if json[i] == b']' {
                return (i + 1, true, false);
            }
            if !skip {
                if f(i, i + 1, COMMA) == 0 {
                    return (i, true, true);
                }
            }
            i += 1;
        }
    }
    return (i, false, true);
}

fn vstring(json: &[u8], mut i: usize) -> (usize, usize, bool, bool) {
    let mut info: usize = 0;
    'outer: loop {
        'tok: loop {
            while i + 8 < json.len() {
                for _ in 0..8 {
                    // SAFETY: the call is made safe because the bounds were
                    // checked in the parent while loop condition.
                    if isstrtok(unsafe { *json.get_unchecked(i) }) {
                        break 'tok;
                    }
                    i += 1;
                }
            }
            while i < json.len() {
                if isstrtok(json[i]) {
                    break 'tok;
                }
                i += 1;
            }
            break 'outer;
        }
        if json[i] == b'"' {
            return (i + 1, info, true, false);
        }
        if json[i] < b' ' {
            return (i, info, false, true);
        }
        if json[i] == b'\\' {
            info |= ESCAPED;
            i += 1;
            if i == json.len() {
                return (i, info, false, true);
            }
            match json[i] {
                b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
                b'u' => {
                    for _ in 0..4 {
                        i += 1;
                        if i == json.len() {
                            return (i, info, false, true);
                        }
                        if !((json[i] >= b'0' && json[i] <= b'9')
                            || (json[i] >= b'a' && json[i] <= b'f')
                            || (json[i] >= b'A' && json[i] <= b'F'))
                        {
                            return (i, info, false, true);
                        }
                    }
                }
                _ => {
                    return (i, info, false, true);
                }
            }
        }
        i += 1;
    }
    return (i, info, false, true);
}

fn vnumber(json: &[u8], mut i: usize) -> (usize, usize, bool, bool) {
    let mut info: usize = 0;

    i -= 1; // go back one byte

    if json[i] == b'-' {
        info |= SIGN;
        i += 1;
        if i == json.len() || !isnum(json[i]) {
            return (i, info, false, true);
        }
    }

    'significand: loop {
        if json[i] == b'0' {
            i += 1;
        } else {
            while i < json.len() {
                if !isnum(json[i]) {
                    break 'significand;
                }
                i += 1;
            }
        }
        if i == json.len() {
            return (i, info, true, false);
        }
        break;
    }

    'base: loop {
        if json[i] == b'.' {
            info |= DOT;
            i += 1;
            if i == json.len() {
                return (i, info, false, true);
            }
            if !isnum(json[i]) {
                return (i, info, false, true);
            }
            i += 1;
            while i + 4 < json.len() {
                for _ in 0..4 {
                    // SAFETY: the call is made safe because the bounds were
                    // checked in the parent while loop condition.
                    if !isnum(unsafe { *json.get_unchecked(i) }) {
                        break 'base;
                    }
                    i += 1;
                }
            }
            while i < json.len() {
                if !isnum(json[i]) {
                    break 'base;
                }
                i += 1;
            }
        }
        if i == json.len() {
            return (i, info, true, false);
        }
        break;
    }

    // 'exponent: loop {
    if json[i] == b'e' || json[i] == b'E' {
        info |= E;
        i += 1;
        if i == json.len() {
            return (i, info, false, true);
        }
        if json[i] == b'+' || json[i] == b'-' {
            i += 1;
        }
        if i == json.len() {
            return (i, info, false, true);
        }
        if !isnum(json[i]) {
            return (i, info, false, true);
        }
        i += 1;
        while i < json.len() {
            if !isnum(json[i]) {
                break;
            }
            i += 1;
        }
    }
    return (i, info, true, false);
    // }
}

fn vtrue(json: &[u8], i: usize) -> (usize, bool, bool) {
    if i + 3 <= json.len() {
        if json[i] == b'r' && json[i + 1] == b'u' && json[i + 2] == b'e' {
            return (i + 3, true, false);
        }
    }
    return (i, false, true);
}

fn vnull(json: &[u8], i: usize) -> (usize, bool, bool) {
    if i + 3 <= json.len() {
        if json[i] == b'u' && json[i + 1] == b'l' && json[i + 2] == b'l' {
            return (i + 3, true, false);
        }
    }
    return (i, false, true);
}

fn vfalse(json: &[u8], i: usize) -> (usize, bool, bool) {
    if i + 4 <= json.len() {
        if json[i] == b'a' && json[i + 1] == b'l' && json[i + 2] == b's' && json[i + 3] == b'e' {
            return (i + 4, true, false);
        }
    }
    return (i, false, true);
}

fn vcolon(json: &[u8], mut i: usize) -> (usize, bool, bool) {
    while i < json.len() {
        if json[i] == b':' {
            return (i + 1, true, false);
        }
        if !isws(json[i]) {
            break;
        }
        i += 1;
    }
    return (i, false, true);
}

fn vcomma(json: &[u8], mut i: usize, end: u8) -> (usize, bool, bool) {
    while i < json.len() {
        if json[i] == b',' {
            return (i, true, false);
        }
        if json[i] == end {
            return (i, true, false);
        }
        if !isws(json[i]) {
            break;
        }
        i += 1;
    }
    return (i, false, true);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    use std::io::Write;

    fn frag(json: &[u8], start: usize, end: usize) -> String {
        return String::from_utf8(json[start..end].to_vec()).unwrap();
    }

    #[test]
    fn iters() {
        ///////////////
        let json = br#" { "hello" : [ 1, 2, 3 ], "jello" : [ 4, 5, 6 ] } "#;
        let mut out = String::new();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return -1;
        });
        assert_eq!(out, "{}");
        let mut out = String::new();
        parse(json, UNCHECKED, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return -1;
        });
        assert_eq!(out, "{}");
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return 0;
        });
        assert_eq!(out, "{");
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return 0;
        });
        assert_eq!(out, "{");
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & KEY == KEY {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello""#);
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & KEY == KEY {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello""#);
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & COLON == COLON {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":"#);
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & COLON == COLON {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":"#);
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            if info & COMMA == COMMA {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"#);
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            if info & COMMA == COMMA {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"#);
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"jello":[]}"#);
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"jello":[]}"#);
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            if info & (CLOSE | OBJECT) == CLOSE | OBJECT {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"jello":[]}"#);
        out.clear();
        parse(json, UNCHECKED, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (OPEN | ARRAY) == OPEN | ARRAY {
                return -1;
            }
            if info & (CLOSE | OBJECT) == CLOSE | OBJECT {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"{"hello":[],"jello":[]}"#);
        ///////////////
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (OBJECT | START) == OBJECT | START {
                out.push_str(&frag(json, start, end));
            }
            return 0;
        });
        assert_eq!(out, r#"{"#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (OBJECT | START | END) == OBJECT | START | END {
                out.push_str(&frag(json, start, end));
            }
            return 0;
        });
        assert_eq!(out, r#""#);

        let json = br#" [ 1,2,3 ] "#;
        out.clear();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return 0;
        });
        assert_eq!(out, r#"["#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & COMMA == COMMA {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"[1,"#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return -1;
        });
        assert_eq!(out, r#"[]"#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            if info & (ARRAY | CLOSE) == ARRAY | CLOSE {
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"[1,2,3]"#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (ARRAY | START) == ARRAY | START {
                out.push_str(&frag(json, start, end));
            }
            return 0;
        });
        assert_eq!(out, r#"["#);

        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (ARRAY | START | END) == ARRAY | START | END {
                out.push_str(&frag(json, start, end));
            }
            return 0;
        });
        assert_eq!(out, r#""#);

        let json = br#" true "#;
        out.clear();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return 0;
        });
        assert_eq!(out, r#"true"#);

        let json = br#" true "#;
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (START | END) == START | END {
                out.push_str(&frag(json, start, end));
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#"true"#);

        let json = br#"{  "hi\nthere": "yo" }"#;
        out.clear();
        parse(json, 0, |start: usize, end: usize, info: usize| -> i64 {
            if info & (KEY) == KEY {
                out.push_str(&frag(json, start, end));
                return 0;
            }
            return 1;
        });
        assert_eq!(out, r#""hi\nthere""#);

        let json = br#" { "a" : "b" , "c" : [ 1 , 2 , 3 ] } "#;
        out.clear();
        let mut index = 0;
        let expect = [
            START | OPEN | OBJECT,
            KEY | STRING,
            COLON,
            VALUE | STRING,
            COMMA,
            KEY | STRING,
            COLON,
            VALUE | OPEN | ARRAY,
            VALUE | NUMBER,
            COMMA,
            VALUE | NUMBER,
            COMMA,
            VALUE | NUMBER,
            VALUE | CLOSE | ARRAY,
            END | CLOSE | OBJECT,
        ];
        parse(json, 0, |_: usize, _: usize, info: usize| -> i64 {
            assert_eq!(expect[index], info);
            index += 1;
            return 1;
        });
        if index != 15 {
            panic!("!");
        }
    }
    #[test]
    fn simples() {
        assert_eq!(parse_simple(br#" 10 "#), NUMBER | START | END);
        assert_eq!(parse_simple(br#" 10.0 "#), NUMBER | DOT | START | END);
        assert_eq!(
            parse_simple(br#" -0.0 "#),
            NUMBER | SIGN | DOT | START | END
        );
        assert_eq!(
            parse_simple(br#" -1230.1230e10 "#),
            NUMBER | SIGN | DOT | E | START | END
        );
        assert_eq!(
            parse_simple(br#" -1230.1230e-10 "#),
            NUMBER | SIGN | DOT | E | START | END
        );
        assert_eq!(
            parse_simple(br#" 1230.1230e+10 "#),
            NUMBER | DOT | E | START | END
        );
        assert_eq!(
            parse_simple(br#" -1230e10 "#),
            NUMBER | SIGN | E | START | END
        );
        assert_eq!(parse_simple(br#" 1230e10 "#), NUMBER | E | START | END);
        assert_eq!(parse_simple(br#" "" "#), STRING | START | END);
        assert_eq!(parse_simple(br#" "hello" "#), STRING | START | END);
        assert_eq!(
            parse_simple(br#" "hell\no" "#),
            STRING | ESCAPED | START | END
        );
        assert_eq!(
            parse_simple(br#" "hell\no" "#),
            STRING | ESCAPED | START | END
        );
        assert_eq!(parse_simple(br#" true "#), TRUE | START | END);
        assert_eq!(parse_simple(br#" false "#), FALSE | START | END);
        assert_eq!(parse_simple(br#" null "#), NULL | START | END);
    }

    fn parse_simple(json: &[u8]) -> usize {
        let mut oinfo: usize = 0;
        let ret = parse(json, 0, |_: usize, _: usize, info: usize| -> i64 {
            oinfo = info;
            -1
        });
        assert_eq!(ret as usize, json.len());
        oinfo
    }

    fn testvalid(json: &[u8], valid: bool) {
        let ret = parse(json, 0, |_: usize, _: usize, _: usize| -> i64 { 1 });
        assert_eq!(valid, ret > 0);
    }

    const JSON1: &str = r#"{
        "widget": {
            "debug": "on",
            "window": {
                "title": "Sample Konfabulator Widget",
                "name": "main_window",
                "width": 500,
                "height": 500
            },
            "image": {
                "src": "Images/Sun.png",
                "hOffset": 250,
                "vOffset": 250,
                "alignment": "center"
            },
            "text": {
                "data": "Click Here",
                "size": 36,
                "style": "bold",
                "vOffset": 100,
                "alignment": "center",
                "onMouseUp": "sun1.opacity = (sun1.opacity / 100) * 90;"
            }
        }
    }"#;

    const JSON2: &str = r#"
    {
        "tagged": "OK",
        "Tagged": "KO",
        "NotTagged": true,
        "unsettable": 101,
        "Nested": {
            "Yellow": "Green",
            "yellow": "yellow"
        },
        "nestedTagged": {
            "Green": "Green",
            "Map": {
                "this": "that",
                "and": "the other thing"
            },
            "Ints": {
                "Uint": 99,
                "Uint16": 16,
                "Uint32": 32,
                "Uint64": 65
            },
            "Uints": {
                "int": -99,
                "Int": -98,
                "Int16": -16,
                "Int32": -32,
                "int64": -64,
                "Int64": -65
            },
            "Uints": {
                "Float32": 32.32,
                "Float64": 64.64
            },
            "Byte": 254,
            "Bool": true
        },
        "LeftOut": "you shouldn't be here",
        "SelfPtr": {"tagged":"OK","nestedTagged":{"Ints":{"Uint32":32}}},
        "SelfSlice": [{"tagged":"OK","nestedTagged":{"Ints":{"Uint32":32}}}],
        "SelfSlicePtr": [{"tagged":"OK","nestedTagged":{"Ints":{"Uint32":32}}}],
        "SelfPtrSlice": [{"tagged":"OK","nestedTagged":{"Ints":{"Uint32":32}}}],
        "interface": "Tile38 Rocks!",
        "Interface": "Please Download",
        "Array": [0,2,3,4,5],
        "time": "2017-05-07T13:24:43-07:00",
        "Binary": "R0lGODlhPQBEAPeo",
        "NonBinary": [9,3,100,115]
    }
    "#;

    #[test]
    fn valid_basic() {
        testvalid(br#"false"#, true);
        testvalid(br#"fals0"#, false);
        testvalid(br#"-\n"#, false);
        testvalid(br#"0"#, true);
        testvalid(br#"00"#, false);
        testvalid(br#"-00"#, false);
        testvalid(br#"-."#, false);
        testvalid(br#"0.0"#, true);
        testvalid(br#"10.0"#, true);
        testvalid(br#"10e1"#, true);
        testvalid(br#"10EE"#, false);
        testvalid(br#"10E-"#, false);
        testvalid(br#"10E+"#, false);
        testvalid(br#"10E+1a"#, false);
        testvalid(br#"10E123"#, true);
        testvalid(br#"10E-123"#, true);
        testvalid(br#"10E-0123"#, true);
        testvalid(br#""#, false);
        testvalid(br#" "#, false);
        testvalid(br#"{}"#, true);
        testvalid(br#"{"#, false);
        testvalid(br#"-"#, false);
        testvalid(br#"-1"#, true);
        testvalid(br#"-1."#, false);
        testvalid(br#"-1.0"#, true);
        testvalid(br#" -1.0"#, true);
        testvalid(br#" -1.0 "#, true);
        testvalid(br#"-1.0 "#, true);
        testvalid(br#"-1.0 i"#, false);
        testvalid(br#"-1.0 i"#, false);
        testvalid(br#"true"#, true);
        testvalid(br#" true"#, true);
        testvalid(br#" true "#, true);
        testvalid(br#" True "#, false);
        testvalid(br#" tru"#, false);
        testvalid(br#"false"#, true);
        testvalid(br#" false"#, true);
        testvalid(br#" false "#, true);
        testvalid(br#" False "#, false);
        testvalid(br#" fals"#, false);
        testvalid(br#"null"#, true);
        testvalid(br#" null"#, true);
        testvalid(br#" null "#, true);
        testvalid(br#" Null "#, false);
        testvalid(br#" nul"#, false);
        testvalid(br#" []"#, true);
        testvalid(br#" [true]"#, true);
        testvalid(br#" [ true, null ]"#, true);
        testvalid(br#" [ true,]"#, false);
        testvalid(br#"{"hello":"world"}"#, true);
        testvalid(br#"{ "hello": "world" }"#, true);
        testvalid(br#"{ "hello": "world", }"#, false);
        testvalid(br#"{"a":"b",}"#, false);
        testvalid(br#"{"a":"b","a"}"#, false);
        testvalid(br#"{"a":"b","a":}"#, false);
        testvalid(br#"{"a":"b","a":1}"#, true);
        testvalid(br#"{"a":"b",2"1":2}"#, false);
        testvalid(br#"{"a":"b","a": 1, "c":{"hi":"there"} }"#, true);
        testvalid(
            br#"{"a":"b","a": 1, "c":{"hi":"there", "easy":["going",{"mixed":"bag"}]} }"#,
            true,
        );
        testvalid(br#""""#, true);
        testvalid(br#"""#, false);
        testvalid(br#""\n""#, true);
        testvalid(br#""\""#, false);
        testvalid(br#""\\""#, true);
        testvalid(br#""a\\b""#, true);
        testvalid(br#""a\\b\\\"a""#, true);
        testvalid(br#""a\\b\\\uFFAAa""#, true);
        testvalid(br#""a\\b\\\uFFAZa""#, false);
        testvalid(br#""a\\b\\\uFFA""#, false);
        testvalid(br#""hello world\"#, false);
        testvalid(br#""hello world\i"#, false);
        testvalid(br#""hello world\u8"#, false);
        testvalid(br#"[1"#, false);
        testvalid(br#"[1,"#, false);
        testvalid(br#"{"hi":"ya""#, false);
        testvalid(br#"{"hi"#, false);
        testvalid(br#"{123:123}"#, false);
        testvalid(br#"123.a123"#, false);
        testvalid(br#"123.123e"#, false);
        testvalid(JSON1.as_bytes(), true);
        testvalid(JSON2.as_bytes(), true);
        let mut a: Vec<u8> = br#""hello"#.iter().cloned().collect();
        a.push(0);
        a.append(&mut br#"world""#.iter().cloned().collect());
        testvalid(&a, false)
    }

    fn testreturnvalue(json: &[u8], expect: i64) {
        let ret = parse(json, 0, |_: usize, _: usize, _: usize| -> i64 { 1 });
        assert_eq!(ret, expect);
    }

    #[test]
    fn returnvalue_basic() {
        testreturnvalue(br#"false"#, 5);
        testreturnvalue(br#"false "#, 6);
        testreturnvalue(br#" false "#, 7);
        testreturnvalue(br#""#, 0);
        testreturnvalue(br#" "#, -1);
        testreturnvalue(br#" a"#, -1);
        testreturnvalue(br#" {"hel\y" : 1}"#, -7);
    }

    fn ugly(src: &[u8]) -> Vec<u8> {
        let mut dst = Vec::new();
        let mut i = 0;
        while i < src.len() {
            if src[i] > b' ' {
                dst.push(src[i]);
                if src[i] == b'"' {
                    i += 1;
                    while i < src.len() {
                        dst.push(src[i]);
                        if src[i] == b'"' {
                            let mut j = i - 1;
                            loop {
                                if src[j] != b'\\' {
                                    break;
                                }
                                j -= 1;
                            }
                            if (i - j) & 1 != 0 {
                                break;
                            }
                        }
                        i += 1;
                    }
                }
            }
            i += 1;
        }
        dst
    }

    #[test]
    fn ugly_test() {
        let json_in = r#" { "\"a\"a" : "b\\b" , "c\nc" : [ 1 , 2 , 3 ] } "#;
        let json_out = r#"{"\"a\"a":"b\\b","c\nc":[1,2,3]}"#;
        assert_eq!(
            String::from_utf8(ugly(json_in.as_bytes())).unwrap(),
            json_out,
        );
    }

    fn test_file(path: &str) {
        println!("{}", path);
        let contents = fs::read_to_string(path).unwrap();
        let json = contents.as_bytes();
        let mut out = String::new();
        parse(json, 0, |start: usize, end: usize, _: usize| -> i64 {
            out.push_str(&frag(json, start, end));
            return 1;
        });
        assert_eq!(String::from_utf8(ugly(&json)).unwrap(), out,);
    }

    #[test]
    fn test_files() {
        for file in fs::read_dir("testfiles").unwrap() {
            test_file(&file.unwrap().path().into_os_string().into_string().unwrap());
        }
    }

    #[test]
    #[ignore]
    fn bench() {
        let mut path = String::new();
        for (key, value) in std::env::vars() {
            if key == "PJSON_BENCH_FILE" {
                path = String::from(value);
                break;
            }
        }
        if path != "" {
            bench_file(&path);
        }
    }

    fn bench_file(path: &str) {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let json = contents.as_bytes();
        print!("running benchmark: {}: ", path);
        std::io::stdout().flush().unwrap();
        let mut total = 0;
        let start = std::time::Instant::now();
        while total < 100 * 1024 * 1024 {
            parse(json, 0, |_: usize, _: usize, _: usize| -> i64 { -1 });
            total += json.len();
        }
        println!(
            "{:.2} GB/sec",
            (total as f64 / start.elapsed().as_secs_f64() / 1024.0 / 1024.0 / 1024.0),
        );
    }

    const EXAMPLE: &[u8] = br#"
	    {
	      "name": {"first": "Tom", "last": "Anderson"},
	      "age":37,
	      "children": ["Sara","Alex","Jack"],
	      "fav.movie": "Deer Hunter",
	      "friends": [
	    	{"first": "Dale", "last": "Murphy", "age": 44, "nets": ["ig", "fb", "tw"]},
	    	{"first": "Roger", "last": "Craig", "age": 68, "nets": ["fb", "tw"]},
	    	{"first": "Jane", "last": "Murphy", "age": 47, "nets": ["ig", "tw"]}
	      ]
	    }
	   "#;

    #[test]
    fn test_unchecked() {
        parse(EXAMPLE, UNCHECKED, |start: usize, end: usize, _: usize| -> i64 {
            println!("{}", unsafe {
                std::str::from_utf8_unchecked(&EXAMPLE[start..end])
            });
            1
        });
    }
}
