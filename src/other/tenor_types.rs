#![allow(non_camel_case_types)]

use std::fmt;


#[derive(Debug)]
pub enum ContentFilter {
    off,
    low,
    medium,
    high
}

impl fmt::Display for ContentFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// More information here https://developers.google.com/tenor/guides/response-objects-and-errors#media-object
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MediaFilter {
    preview,
    gif,
    mediumgif,
    tinygif,
    nanogif,

    mp4,
    loopedmp4,
    tinymp4,
    nanomp4,

    webm,
    tinywebm,
    nanowebm,

    webp_transparent,
    tinywebp_transparent,
    nanowebp_transparent,

    gif_transparent,
    tinygif_transparent,
    nanogif_transparent
}

impl fmt::Display for MediaFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ArRange {
    all,
    wide,
    standard
}

impl fmt::Display for ArRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}