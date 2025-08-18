use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::BigInt;
use crate::bytecode::bytecode::Bytecode;

pub struct BytecodeBuilder {
    pub bytes: Vec<u8>,
}

impl BytecodeBuilder {

    pub fn new() -> Self { BytecodeBuilder { bytes: Vec::new() } }

    pub fn write_i8(&mut self, value: i8) { self.bytes.push(value as u8); }
    pub fn write_u8(&mut self, value: u8) { self.bytes.push(value); }

    pub fn write_i16(&mut self, value: i16) {
        self.write_i8((value >> 8) as i8);
        self.write_i8(value as i8);
    }
    pub fn write_u16(&mut self, value: u16) {
        self.write_u8((value >> 8) as u8);
        self.write_u8(value as u8);
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_i16((value >> 16) as i16);
        self.write_i16(value as i16);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_u16((value >> 16) as u16);
        self.write_u16(value as u16);
    }

    pub fn write_i64(&mut self, value: i64) {
        self.write_i32((value >> 32) as i32);
        self.write_i32(value as i32);
    }

    pub fn write_u64(&mut self, value: u64) {
        self.write_u32((value >> 32) as u32);
        self.write_u32(value as u32);
    }

    pub fn write_i128(&mut self, value: i128) {
        self.write_i64((value >> 64) as i64);
        self.write_i64(value as i64);
    }

    pub fn write_u128(&mut self, value: u128) {
        self.write_u64((value >> 64) as u64);
        self.write_u64(value as u64);
    }

    pub fn write_bool(&mut self, value: bool) { self.write_u8(if value { 1 } else { 0 }); }

    pub fn write_f32(&mut self, value: f32) {
        let bits = value.to_bits();
        self.write_u32(bits);
    }

    pub fn write_f64(&mut self, value: f64) {
        let bits = value.to_bits();
        self.write_u64(bits);
    }

    pub fn write_isize(&mut self, value: isize) {
        self.write_u64(value as u64);
    }

    pub fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }

    pub fn write_char(&mut self, value: char) {
        let mut bytes = [0; 4];
        let len = value.encode_utf8(&mut bytes).len();
        self.write_usize(len); // 字符长度
        for &byte in &bytes[..len] {
            self.write_u8(byte); // 字符字节
        }
    }

    pub fn write_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        let length = bytes.len();
        self.write_usize(length); // 字符串长度
        for &byte in bytes {
            self.write_u8(byte); // 字符串字节
        }
    }

    pub fn write_big_decimal(&mut self, value: BigDecimal) {
        self.write_string(value.to_string().as_str())
    }

    pub fn write_big_int(&mut self, value: BigInt) {
        self.write_string(value.to_string().as_str())
    }

    pub fn write_bytecode(&mut self, bytecode: Bytecode) {
        bytecode.write(self);
    }

    pub fn write_vec<T, F>(&mut self, vec: &Vec<T>, write_element: F)
    where
        F: Fn(&mut BytecodeBuilder, &T),
    {
        self.write_usize(vec.len());
        for element in vec {
            write_element(self, element);
        }
    }
    
}
