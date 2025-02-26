use std::io;

use std::io::Write;

use std::io::BufRead;
use std::io::Read;

use lz4_flex::frame::FrameDecoder;
use lz4_flex::frame::FrameEncoder;

pub fn bufread2writer_enc<R, W>(mut rdr: R, wtr: W) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
{
    let mut enc: FrameEncoder<_> = FrameEncoder::new(wtr);
    io::copy(&mut rdr, &mut enc)?;
    let mut iwtr: W = enc.finish()?;
    iwtr.flush()
}

pub fn stdin2stdout_enc() -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let ol = o.lock();

    bufread2writer_enc(il, ol)
}

pub fn bufread2writer_dec<R, W>(rdr: R, mut wtr: W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let mut dec: FrameDecoder<_> = FrameDecoder::new(rdr);
    io::copy(&mut dec, &mut wtr)?;
    wtr.flush()
}

pub fn stdin2stdout_dec() -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let ol = o.lock();

    bufread2writer_dec(il, ol)
}

pub fn stdin2stdout_b(encode: bool) -> Result<(), io::Error> {
    match encode {
        true => stdin2stdout_enc(),
        false => stdin2stdout_dec(),
    }
}

pub fn stdin2stdout() -> Result<(), io::Error> {
    stdin2stdout_b(false)
}
