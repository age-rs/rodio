//! Decodes samples from an audio file.

use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use std::time::Duration;

use Source;

mod flac;
mod vorbis;
mod wav;

/// Source of audio samples from decoding a file.
///
/// Supports WAV, Vorbis and Flac.
pub struct Decoder<R>(DecoderImpl<R>) where R: Read + Seek;

enum DecoderImpl<R>
    where R: Read + Seek
{
    Wav(wav::WavDecoder<R>),
    Vorbis(vorbis::VorbisDecoder<R>),
    Flac(flac::FlacDecoder<R>),
}

impl<R> Decoder<R>
    where R: Read + Seek + Send + 'static
{
    /// Builds a new decoder.
    ///
    /// Attempts to automatically detect the format of the source of data.
    pub fn new(data: R) -> Result<Decoder<R>, DecoderError> {
        let data = match wav::WavDecoder::new(data) {
            Err(data) => data,
            Ok(decoder) => {
                return Ok(Decoder(DecoderImpl::Wav(decoder)));
            }
        };

        let data = match flac::FlacDecoder::new(data) {
            Err(data) => data,
            Ok(decoder) => {
                return Ok(Decoder(DecoderImpl::Flac(decoder)));
            }
        };

        if let Ok(decoder) = vorbis::VorbisDecoder::new(data) {
            return Ok(Decoder(DecoderImpl::Vorbis(decoder)));
        }

        Err(DecoderError::UnrecognizedFormat)
    }
}

impl<R> Iterator for Decoder<R>
    where R: Read + Seek
{
    type Item = i16;

    #[inline]
    fn next(&mut self) -> Option<i16> {
        match self.0 {
            DecoderImpl::Wav(ref mut source) => source.next(),
            DecoderImpl::Vorbis(ref mut source) => source.next(),
            DecoderImpl::Flac(ref mut source) => source.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.size_hint(),
            DecoderImpl::Vorbis(ref source) => source.size_hint(),
            DecoderImpl::Flac(ref source) => source.size_hint(),
        }
    }
}

impl<R> Source for Decoder<R>
    where R: Read + Seek
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.current_frame_len(),
            DecoderImpl::Vorbis(ref source) => source.current_frame_len(),
            DecoderImpl::Flac(ref source) => source.current_frame_len(),
        }
    }

    #[inline]
    fn channels(&self) -> u16 {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.channels(),
            DecoderImpl::Vorbis(ref source) => source.channels(),
            DecoderImpl::Flac(ref source) => source.channels(),
        }
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.samples_rate(),
            DecoderImpl::Vorbis(ref source) => source.samples_rate(),
            DecoderImpl::Flac(ref source) => source.samples_rate(),
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.total_duration(),
            DecoderImpl::Vorbis(ref source) => source.total_duration(),
            DecoderImpl::Flac(ref source) => source.total_duration(),
        }
    }
}

/// Error that can happen when creating a decoder.
#[derive(Debug, Clone)]
pub enum DecoderError {
    /// The format of the data has not been recognized.
    UnrecognizedFormat,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DecoderError::UnrecognizedFormat => write!(f, "Unrecognized format"),
        }
    }
}

impl Error for DecoderError {
    fn description(&self) -> &str {
        match self {
            &DecoderError::UnrecognizedFormat => "Unrecognized format",
        }
    }
}
