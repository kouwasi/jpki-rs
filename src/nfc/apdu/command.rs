use crate::nfc::apdu;
use crate::nfc::apdu::ins;

pub struct Command {
    cla: u8,
    ins: u8,
    p1: u8,
    p2: u8,
    le: Option<u8>,
    payload: Option<Vec<u8>>,
}

impl Command {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: None,
            payload: None,
        }
    }

    pub fn new_with_le(cla: u8, ins: u8, p1: u8, p2: u8, le: u8) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: Some(le),
            payload: None,
        }
    }

    pub fn new_with_payload(cla: u8, ins: u8, p1: u8, p2: u8, payload: Vec<u8>) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: None,
            payload: Some(payload),
        }
    }

    pub fn new_with_payload_le(cla: u8, ins: u8, p1: u8, p2: u8, le: u8, payload: Vec<u8>) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: Some(le),
            payload: Some(payload),
        }
    }

    pub fn select_file(p1: u8, p2: u8, payload: Vec<u8>) -> Self {
        match payload.len() {
            0 => Self::new(apdu::CLA_DEFAULT, ins::SELECT_FILE, p1, p2),
            _ => Self::new_with_payload(apdu::CLA_DEFAULT, ins::SELECT_FILE, p1, p2, payload),
        }
    }

    pub fn read_binary(p1: u8, p2: u8, le: u8) -> Self {
        Self::new_with_le(apdu::CLA_DEFAULT, ins::READ_BINARY, p1, p2, le)
    }

    pub fn verify(p2: u8, payload: Vec<u8>) -> Self {
        match payload.len() {
            0 => Self::new(apdu::CLA_DEFAULT, ins::VERIFY, 0x00, p2),
            _ => Self::new_with_payload(apdu::CLA_DEFAULT, ins::VERIFY, 0x00, p2, payload),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let Self {
            cla,
            ins,
            p1,
            p2,
            le,
            payload,
        } = self;

        let mut buffer: Vec<u8> = vec![cla, ins, p1, p2];
        if let Some(mut p) = payload {
            buffer.push(p.len() as u8);
            buffer.append(&mut p);
        }

        if let Some(l) = le {
            buffer.push(l);
        }

        buffer
    }
}