use std::str::FromStr;
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::BigInt;
use crate::bytecode::bytecode::Bytecode;

pub struct BytecodeReader {
    pub bytes: Vec<u8>,
    pub position: usize,
}

impl BytecodeReader {
    
    pub fn new(bytecode: Vec<u8>) -> Self { BytecodeReader { bytes: bytecode, position: 0 } }
    
    pub fn has_next(&self) -> bool { self.position < self.bytes.len() }
    
    pub fn read_u8(&mut self) -> Option<u8> {
        if !self.has_next() {
            None
        } else {
            let value = self.bytes[self.position];
            self.position += 1;
            Some(value)
        }
    }
    
    pub fn read_i8(&mut self) -> Option<i8> { Some(self.read_u8()? as i8) }
    
    pub fn read_u16(&mut self) -> Option<u16> {
        if !self.has_next() || self.position + 1 >= self.bytes.len() {
            None
        } else {
            let value = u16::from_be_bytes([
                self.bytes[self.position],
                self.bytes[self.position + 1],
            ]);
            self.position += 2;
            Some(value)
        }
    }
    
    pub fn read_i16(&mut self) -> Option<i16> { Some(self.read_u16()? as i16) }
    
    pub fn read_u32(&mut self) -> Option<u32> {
        if !self.has_next() || self.position + 3 >= self.bytes.len() {
            None
        } else {
            let value = u32::from_be_bytes([
                self.bytes[self.position],
                self.bytes[self.position + 1],
                self.bytes[self.position + 2],
                self.bytes[self.position + 3],
            ]);
            self.position += 4;
            Some(value)
        }
    }
    
    pub fn read_i32(&mut self) -> Option<i32> { Some(self.read_u32()? as i32) }
    
    pub fn read_u64(&mut self) -> Option<u64> {
        if !self.has_next() || self.position + 7 >= self.bytes.len() {
            None
        } else {
            let value = u64::from_be_bytes([
                self.bytes[self.position],
                self.bytes[self.position + 1],
                self.bytes[self.position + 2],
                self.bytes[self.position + 3],
                self.bytes[self.position + 4],
                self.bytes[self.position + 5],
                self.bytes[self.position + 6],
                self.bytes[self.position + 7],
            ]);
            self.position += 8;
            Some(value)
        }
    }
    
    pub fn read_i64(&mut self) -> Option<i64> { Some(self.read_u64()? as i64) }
    
    pub fn read_u128(&mut self) -> Option<u128> {
        if !self.has_next() || self.position + 15 >= self.bytes.len() {
            None
        } else {
            let value = u128::from_be_bytes([
                self.bytes[self.position],
                self.bytes[self.position + 1],
                self.bytes[self.position + 2],
                self.bytes[self.position + 3],
                self.bytes[self.position + 4],
                self.bytes[self.position + 5],
                self.bytes[self.position + 6],
                self.bytes[self.position + 7],
                self.bytes[self.position + 8],
                self.bytes[self.position + 9],
                self.bytes[self.position + 10],
                self.bytes[self.position + 11],
                self.bytes[self.position + 12],
                self.bytes[self.position + 13],
                self.bytes[self.position + 14],
                self.bytes[self.position + 15],
            ]);
            self.position += 16;
            Some(value)
        }
    }
    
    pub fn read_i128(&mut self) -> Option<i128> { Some(self.read_u128()? as i128) }
    
    pub fn read_bool(&mut self) -> Option<bool> {
        let value = self.read_u8()?;
        Some(value != 0)
    }
    
    pub fn read_f32(&mut self) -> Option<f32> {
        let bits = self.read_u32()?;
        Some(f32::from_bits(bits))
    }
    
    pub fn read_f64(&mut self) -> Option<f64> {
        let bits = self.read_u64()?;
        Some(f64::from_bits(bits))
    }
    
    pub fn read_isize(&mut self) -> Option<isize> {
        let value = self.read_u64()?;
        Some(value as isize)
    }
    
    pub fn read_usize(&mut self) -> Option<usize> {
        let value = self.read_u64()?;
        Some(value as usize)
    }
    
    pub fn read_char(&mut self) -> Option<char> {
        let length = self.read_usize()?;
        if length > self.bytes.len() - self.position {
            return None; // 长度超出边界
        }
        let bytes = &self.bytes[self.position..self.position + length];
        self.position += length;
        String::from_utf8(bytes.to_vec()).ok().and_then(|s| s.chars().next())
    }
    
    pub fn read_string(&mut self) -> Option<String> {
        let length = self.read_usize()?;
        if length > self.bytes.len() - self.position {
            return None; // 长度超出边界
        }
        let bytes = &self.bytes[self.position..self.position + length];
        self.position += length;
        Some(String::from_utf8_lossy(bytes).to_string())
    }
    
    pub fn read_big_decimal(&mut self) -> Option<BigDecimal> {
        Some(BigDecimal::from_str(self.read_string()?.as_str()).ok()?)
    }

    pub fn read_big_int(&mut self) -> Option<BigInt> {
        Some(BigInt::from_str(self.read_string()?.as_str()).ok()?)
    }
    
    pub fn read_bytecode(&mut self) -> Option<Bytecode> {
        Bytecode::read(self)
    }
    
    pub fn read_vec<T, F>(&mut self, read_element: F) -> Option<Vec<T>>
    where
        F: Fn(&mut BytecodeReader) -> Option<T>,
    {
        let length = self.read_usize()?;
        let mut vec = Vec::with_capacity(length);
        for _ in 0..length {
            if let Some(element) = read_element(self) {
                vec.push(element);
            } else {
                return None; // 读取元素失败
            }
        }
        Some(vec)
    }
}

impl Iterator for BytecodeReader {
    type Item = Bytecode;
    fn next(&mut self) -> Option<Bytecode> { self.read_bytecode() }
}
