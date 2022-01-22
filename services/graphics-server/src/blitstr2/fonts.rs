// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![allow(dead_code)]
pub mod bold;
pub mod emoji;
pub mod ja;
pub mod kr;
pub mod mono;
pub mod regular;
pub mod small;
pub mod zh;

const DEFAULT_KERN: u8 = 1;

// Font data is stored as CODEPOINTS and GLYPHS arrays. CODEPOINTS holds sorted
// Unicode codepoints for characters included in the font, and GLYPHS holds
// 16*16px sprites (pixels packed in row-major order, LSB of first word is top
// left pixel of sprite). The order of codepoints and glyphs is the same, but,
// each codepoint is one u32 word long while each glyph is eight u32 words
// long. So, to find a glyph we do:
//  1. Binary search CODEPOINTS for the codepoint of interest
//  2. Multiply the codepoint index by 8, yielding an offset into GLYPHS
//  3. Slice 8 u32 words from GLYPHS starting at the offset

/// Struct to hold sprite pixel reference and associated metadata for glyphs
#[derive(Copy, Clone, Debug)]
pub struct GlyphSprite {
    pub glyph: &'static [u32],
    pub wide: u8,
    pub high: u8,
    pub kern: u8,
    // the original character
    pub ch: char,
    // invert rendering for the character - for copy/paste selection regions
    pub invert: bool,
    // drawn an insertion point after this character
    pub insert: bool,
    // 2x flag for the back-end rendering (wide/high should be pre-computed to match this)
    pub double: bool,
}

pub fn small_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match small::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= small::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &small::glyphs()[offset..end],
                    wide: small::WIDTHS[n],
                    high: small::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn regular_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match regular::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= regular::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &regular::glyphs()[offset..end],
                    wide: regular::WIDTHS[n],
                    high: regular::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn large_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match small::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= small::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &small::glyphs()[offset..end],
                    wide: small::WIDTHS[n] * 2,
                    high: small::MAX_HEIGHT * 2,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: true,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn extra_large_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match regular::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= regular::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &regular::glyphs()[offset..end],
                    wide: regular::WIDTHS[n] * 2,
                    high: regular::MAX_HEIGHT * 2,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: true,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn bold_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match bold::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= bold::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &bold::glyphs()[offset..end],
                    wide: bold::WIDTHS[n],
                    high: bold::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn mono_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match mono::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= mono::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &mono::glyphs()[offset..end],
                    wide: mono::WIDTHS[n],
                    high: mono::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn emoji_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match emoji::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= emoji::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &emoji::glyphs()[offset..end],
                    wide: emoji::MAX_HEIGHT, // yes, use height for wide
                    high: emoji::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn emoji_large_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match emoji::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= emoji::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &emoji::glyphs()[offset..end],
                    wide: emoji::MAX_HEIGHT * 2, // yes, use height for wide
                    high: emoji::MAX_HEIGHT * 2,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: true,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn zh_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match zh::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= zh::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &zh::glyphs()[offset..end],
                    wide: zh::MAX_HEIGHT, // yes, use height for wide
                    high: zh::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn ja_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match ja::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= ja::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &ja::glyphs()[offset..end],
                    wide: ja::MAX_HEIGHT, // yes, use height for wide
                    high: ja::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn kr_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match kr::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= kr::glyphs().len() {
                true => Ok(GlyphSprite {
                    glyph: &kr::glyphs()[offset..end],
                    wide: kr::MAX_HEIGHT, // yes, use height for wide
                    high: kr::MAX_HEIGHT,
                    kern: DEFAULT_KERN,
                    ch,
                    invert: false,
                    insert: false,
                    double: false,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}
