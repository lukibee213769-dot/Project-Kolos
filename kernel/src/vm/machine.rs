use std::collections::HashMap;
use super::instruction::Instruction;
use super::value::Value;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub return_ip: usize,
    pub locals: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum VmError {
    IpOutOfRange,
    StackUnderflow(&'static str),
    DivisionByZero,
    ModuloByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    ArgumentMismatch { name: String, expected: usize, got: usize },
    TypeMismatch(&'static str),
}

pub struct Vm {
    pub bytecode: Vec<Instruction>,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    pub functions: HashMap<String, (Vec<String>, usize)>,
    pub call_stack: Vec<CallFrame>,
    pub output_log: Vec<String>,
}

impl Vm {
    pub fn new(bytecode: Vec<Instruction>) -> Self {
        Self {
            bytecode,
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
            output_log: Vec::new(),
        }
    }

    pub fn step(&mut self) -> Result<Option<()>, VmError> {
        if self.ip >= self.bytecode.len() {
            return Err(VmError::IpOutOfRange);
        }

        let instr = self.bytecode[self.ip].clone();
        self.ip += 1;

        match instr {
            Instruction::Push(v) => self.stack.push(v),
            Instruction::Pop => {
                self.stack.pop().ok_or(VmError::StackUnderflow("POP"))?;
            }
            Instruction::Dup => {
                let top = self.stack.last().cloned().ok_or(VmError::StackUnderflow("DUP"))?;
                self.stack.push(top);
            }
            Instruction::Store(name) => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("STORE"))?;
                if let Some(frame) = self.call_stack.last_mut() {
                    frame.locals.insert(name, val);
                } else {
                    self.globals.insert(name, val);
                }
            }
            Instruction::Load(name) => {
                let val = if let Some(frame) = self.call_stack.last() {
                    if let Some(v) = frame.locals.get(&name) {
                        Some(v.clone())
                    } else {
                        self.globals.get(&name).cloned()
                    }
                } else {
                    self.globals.get(&name).cloned()
                };

                match val {
                    Some(v) => self.stack.push(v),
                    None => return Err(VmError::UndefinedVariable(name)),
                }
            }
            Instruction::DefFn { name, params, target_ip } => {
                self.functions.insert(name, (params, target_ip));
            }
            Instruction::Call { name, argc } => {
                let (params, target_ip) = self.functions.get(&name)
                    .cloned()
                    .ok_or_else(|| VmError::UndefinedFunction(name.clone()))?;

                if argc != params.len() {
                    return Err(VmError::ArgumentMismatch {
                        name,
                        expected: params.len(),
                        got: argc,
                    });
                }

                let mut args = Vec::with_capacity(argc);
                for _ in 0..argc {
                    args.push(self.stack.pop().ok_or(VmError::StackUnderflow("CALL args"))?);
                }
                args.reverse();

                let mut locals = HashMap::new();
                for (param, val) in params.into_iter().zip(args.into_iter()) {
                    locals.insert(param, val);
                }

                self.call_stack.push(CallFrame {
                    return_ip: self.ip,
                    locals,
                });
                self.ip = target_ip;
            }
            Instruction::Ret => {
                let ret_val = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(frame) = self.call_stack.pop() {
                    self.ip = frame.return_ip;
                    self.stack.push(ret_val);
                } else {
                    self.stack.push(ret_val);
                    return Ok(None);
                }
            }
            Instruction::Add => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("ADD"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("ADD"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x + y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                    (Value::Str(x), Value::Str(y)) => self.stack.push(Value::Str(format!("{}{}", x, y))),
                    _ => return Err(VmError::TypeMismatch("ADD")),
                }
            }
            Instruction::Sub => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("SUB"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("SUB"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x - y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                    _ => return Err(VmError::TypeMismatch("SUB")),
                }
            }
            Instruction::Mul => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("MUL"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("MUL"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x * y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                    _ => return Err(VmError::TypeMismatch("MUL")),
                }
            }
            Instruction::Div => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("DIV"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("DIV"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        if y == 0 { return Err(VmError::DivisionByZero); }
                        self.stack.push(Value::Int(x / y));
                    }
                    (Value::Float(x), Value::Float(y)) => {
                        if y == 0.0 { return Err(VmError::DivisionByZero); }
                        self.stack.push(Value::Float(x / y));
                    }
                    _ => return Err(VmError::TypeMismatch("DIV")),
                }
            }
            Instruction::Pow => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("POW"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("POW"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x.pow(y as u32))),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x.powf(y))),
                    _ => return Err(VmError::TypeMismatch("POW")),
                }
            }
            Instruction::Mod => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("MOD"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("MOD"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        if y == 0 { return Err(VmError::ModuloByZero); }
                        self.stack.push(Value::Int(x % y));
                    }
                    _ => return Err(VmError::TypeMismatch("MOD")),
                }
            }
            Instruction::Neg => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("NEG"))?;
                match val {
                    Value::Int(x) => self.stack.push(Value::Int(-x)),
                    Value::Float(x) => self.stack.push(Value::Float(-x)),
                    _ => return Err(VmError::TypeMismatch("NEG")),
                }
            }
            Instruction::Not => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("NOT"))?;
                self.stack.push(Value::Bool(!val.is_truthy()));
            }
            Instruction::Eq => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("EQ"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("EQ"))?;
                self.stack.push(Value::Bool(a == b));
            }
            Instruction::Ne => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("NE"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("NE"))?;
                self.stack.push(Value::Bool(a != b));
            }
            Instruction::Lt => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("LT"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("LT"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x < y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x < y)),
                    _ => return Err(VmError::TypeMismatch("LT")),
                }
            }
            Instruction::Le => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("LE"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("LE"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x <= y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x <= y)),
                    _ => return Err(VmError::TypeMismatch("LE")),
                }
            }
            Instruction::Gt => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("GT"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("GT"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x > y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x > y)),
                    _ => return Err(VmError::TypeMismatch("GT")),
                }
            }
            Instruction::Ge => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("GE"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("GE"))?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x >= y)),
                    (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x >= y)),
                    _ => return Err(VmError::TypeMismatch("GE")),
                }
            }
            Instruction::And => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("AND"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("AND"))?;
                self.stack.push(Value::Bool(a.is_truthy() && b.is_truthy()));
            }
            Instruction::Or => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow("OR"))?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow("OR"))?;
                self.stack.push(Value::Bool(a.is_truthy() || b.is_truthy()));
            }
            Instruction::Jmp(target) => self.ip = target,
            Instruction::Jz(target) => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("JZ"))?;
                if !val.is_truthy() { self.ip = target; }
            }
            Instruction::Jnz(target) => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("JNZ"))?;
                if val.is_truthy() { self.ip = target; }
            }
            Instruction::Print => {
                let val = self.stack.pop().ok_or(VmError::StackUnderflow("PRINT"))?;
                let s = format!("{}", val);
                self.output_log.push(s.clone());
                println!("{}", s);
            }
            Instruction::Halt => return Ok(None),
        }

        Ok(Some(()))
    }

    pub fn run(&mut self) -> Result<Option<Value>, VmError> {
        while let Some(()) = self.step()? {}
        Ok(self.stack.last().cloned())
    }
}
