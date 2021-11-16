use std::marker::PhantomData;

use crate::nfc;
use crate::nfc::apdu;

const SELECT_P1_DF: u8 = 0x04;
const SELECT_P1_EF: u8 = 0x02;
const SELECT_P2: u8 = 0x0C;

const VERIFY_P2: u8 = 0x80;

const SIGN_CLA: u8 = 0x80;
const SIGN_INS: u8 = 0x2A;
const SIGN_P1: u8 = 0x00;
const SIGN_P2: u8 = 0x80;

pub struct Card<T, Ctx>
where
    T: nfc::Card<Ctx>,
    Ctx: Copy,
{
    delegate: Box<T>,
    _ctx: PhantomData<Ctx>,
}

impl<T, Ctx> Card<T, Ctx>
where
    T: nfc::Card<Ctx>,
    Ctx: Copy,
{
    pub fn new(delegate: Box<T>) -> Self {
        Self {
            delegate,
            _ctx: PhantomData,
        }
    }

    pub fn select_df(&self, ctx: Ctx, name: Vec<u8>) -> Result<(), apdu::Error> {
        self.delegate
            .handle(
                ctx,
                apdu::Command::select_file(SELECT_P1_DF, SELECT_P2, name),
            )
            .into_result()
            .map(|_| ())
    }

    pub fn select_ef(&self, ctx: Ctx, id: Vec<u8>) -> Result<(), apdu::Error> {
        self.delegate
            .handle(ctx, apdu::Command::select_file(SELECT_P1_EF, SELECT_P2, id))
            .into_result()
            .map(|_| ())
    }

    pub fn read(&self, ctx: Ctx, len: Option<u16>) -> Result<Vec<u8>, apdu::Error>
    where
        Ctx: Copy,
    {
        let mut pos: u16 = 0;
        let mut buf: Vec<u8> = Vec::new();

        while match len {
            Some(l) => pos < l,
            None => true,
        } {
            let [p1, p2] = pos.to_be_bytes();
            let le: u8 = match len {
                Some(l) => match l - pos > 0xFF {
                    true => 0,
                    _ => (l & 0xFF) as u8,
                },
                _ => 0,
            };

            let mut fragment = self
                .delegate
                .handle(ctx, apdu::Command::read_binary(p1, p2, le))
                .into_result()?;

            let length = fragment.len();

            buf.append(&mut fragment);
            pos += length as u16;

            if (length as u8) < le {
                break;
            }
        }

        Ok(buf)
    }

    pub fn verify(&self, ctx: Ctx, pin: Vec<u8>) -> Result<(), apdu::Error> {
        self.delegate
            .handle(ctx, apdu::Command::verify(VERIFY_P2, pin))
            .into_result()
            .map(|_| ())
    }

    pub fn sign(&self, ctx: Ctx, digest: Vec<u8>) -> Result<Vec<u8>, apdu::Error> {
        self.delegate
            .handle(
                ctx,
                apdu::Command::new_with_payload_le(SIGN_CLA, SIGN_INS, SIGN_P1, SIGN_P2, 0, digest),
            )
            .into_result()
    }
}