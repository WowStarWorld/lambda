use crate::bytecode::builder::BytecodeBuilder;
use crate::bytecode::reader::BytecodeReader;


#[derive(Debug, Clone)]
pub enum Bytecode {
    // 元信息和基本操作
    Metadata { source_file: String }, // 文件的元信息
    Nop, // 无操作

    // 常量池操作
    Constant(String), // 常量池的内容
    LoadConst(usize), // 从常量池加载常量 #index: 常量值在常量池中的索引

    // 对象操作
    GetObject(usize), // 获取 object class 的实例 #index: 类名在常量池中的索引
    NewObject(usize), // 创建一个新的对象实例  #index: 类名在常量池中的索引

    // 栈操作
    Load, // 加载一个对象到栈顶
    Store(usize), // 将栈顶元素存储到局部变量
    LoadLocal(usize), // 从局部变量加载到栈顶
    Pop, // 弹出栈顶元素
    Dup, // 复制栈顶元素
    Swap, // 交换栈顶两个元素

    // 函数调用
    Invoke(usize), // 调用函数 #index: 函数名在常量池中的索引
    Return, // 返回栈顶的对象

    // 控制流
    Jump(usize), // 无条件跳转
    JumpIfTrue(usize), // 条件跳转（真）
    JumpIfFalse(usize), // 条件跳转（假）

    // 字段操作
    GetField(usize), // 获取字段值
    SetField(usize), // 设数字字段值

    // 类型转换和检查
    CheckCast(usize), // 类型检查和转换  #index: 类名在常量池中的索引
    InstanceOf(usize), // 检查对象是否为指定类型的实例  #index: 类名在常量池中的索引

    // 异常处理
    Throw, // 抛出异常
}

type Code = u8;

impl Bytecode {
    pub fn get_code(&self) -> Code {
        match self {
            Bytecode::Metadata { .. } => 0x00,
            Bytecode::Nop => 0x01,
            Bytecode::Constant(_) => 0x02,
            Bytecode::LoadConst(_) => 0x03,
            Bytecode::GetObject(_) => 0x04,
            Bytecode::NewObject(_) => 0x05,
            Bytecode::Load => 0x06,
            Bytecode::Store(_) => 0x07,
            Bytecode::LoadLocal(_) => 0x08,
            Bytecode::Pop => 0x09,
            Bytecode::Dup => 0x0A,
            Bytecode::Swap => 0x0B,
            Bytecode::Invoke(_) => 0x0C,
            Bytecode::Return => 0x0D,
            Bytecode::Jump(_) => 0x0E,
            Bytecode::JumpIfTrue(_) => 0x0F,
            Bytecode::JumpIfFalse(_) => 0x10,
            Bytecode::GetField(_) => 0x11,
            Bytecode::SetField(_) => 0x12,
            Bytecode::CheckCast(_) => 0x13,
            Bytecode::InstanceOf(_) => 0x14,
            Bytecode::Throw => 0x15,
        }
    }

    pub fn write_code(&self, builder: &mut BytecodeBuilder) {
        builder.write_u8(self.get_code());
    }

    pub fn read_code(reader: &mut BytecodeReader) -> Option<Code> {
        reader.read_u8()
    }

    pub fn read(reader: &mut BytecodeReader) -> Option<Self> {
        let code = Self::read_code(reader)?;
        match code {
            0x00 => {
                let source_file = reader.read_string()?;
                Some(Bytecode::Metadata { source_file })
            },
            0x01 => Some(Bytecode::Nop),
            0x02 => {
                let value = reader.read_string()?;
                Some(Bytecode::Constant(value))
            },
            0x03 => {
                let index = reader.read_usize()?;
                Some(Bytecode::LoadConst(index))
            },
            0x04 => {
                let index = reader.read_usize()?;
                Some(Bytecode::GetObject(index))
            },
            0x05 => {
                let index = reader.read_usize()?;
                Some(Bytecode::NewObject(index))
            },
            0x06 => Some(Bytecode::Load),
            0x07 => {
                let index = reader.read_usize()?;
                Some(Bytecode::Store(index))
            },
            0x08 => {
                let index = reader.read_usize()?;
                Some(Bytecode::LoadLocal(index))
            },
            0x09 => Some(Bytecode::Pop),
            0x0A => Some(Bytecode::Dup),
            0x0B => Some(Bytecode::Swap),
            0x0C => {
                let index = reader.read_usize()?;
                Some(Bytecode::Invoke(index))
            },
            0x0D => Some(Bytecode::Return),
            0x0E => {
                let offset = reader.read_usize()?;
                Some(Bytecode::Jump(offset))
            },
            0x0F => {
                let offset = reader.read_usize()?;
                Some(Bytecode::JumpIfTrue(offset))
            },
            0x10 => {
                let offset = reader.read_usize()?;
                Some(Bytecode::JumpIfFalse(offset))
            },
            0x11 => {
                let index = reader.read_usize()?;
                Some(Bytecode::GetField(index))
            },
            0x12 => {
                let index = reader.read_usize()?;
                Some(Bytecode::SetField(index))
            },
            0x13 => {
                let index = reader.read_usize()?;
                Some(Bytecode::CheckCast(index))
            },
            0x14 => {
                let index = reader.read_usize()?;
                Some(Bytecode::InstanceOf(index))
            },
            0x15 => Some(Bytecode::Throw),
            _ => None,
        }
    }

    pub fn write(&self, builder: &mut BytecodeBuilder) {
        self.write_code(builder);
        match self {
            Bytecode::Metadata { source_file } => {
                builder.write_string(source_file);
            },
            Bytecode::Nop => {},
            Bytecode::Constant(value) => {
                builder.write_string(value);
            },
            Bytecode::LoadConst(index) => {
                builder.write_usize(*index);
            },
            Bytecode::GetObject(index) => {
                builder.write_usize(*index);
            },
            Bytecode::NewObject(index) => {
                builder.write_usize(*index);
            },
            Bytecode::Load => {},
            Bytecode::Store(index) => {
                builder.write_usize(*index);
            },
            Bytecode::LoadLocal(index) => {
                builder.write_usize(*index);
            },
            Bytecode::Pop => {},
            Bytecode::Dup => {},
            Bytecode::Swap => {},
            Bytecode::Invoke(index) => {
                builder.write_usize(*index);
            },
            Bytecode::Return => {},
            Bytecode::Jump(offset) => {
                builder.write_usize(*offset);
            },
            Bytecode::JumpIfTrue(offset) => {
                builder.write_usize(*offset);
            },
            Bytecode::JumpIfFalse(offset) => {
                builder.write_usize(*offset);
            },
            Bytecode::GetField(index) => {
                builder.write_usize(*index);
            },
            Bytecode::SetField(index) => {
                builder.write_usize(*index);
            },
            Bytecode::CheckCast(index) => {
                builder.write_usize(*index);
            },
            Bytecode::InstanceOf(index) => {
                builder.write_usize(*index);
            },
            Bytecode::Throw => {},
        }
    }
}
