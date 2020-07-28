use OpCode::{PUSH, PLUS, MUL};

pub enum OpCode {
    PUSH,
    PLUS,
    MUL
}

pub struct Code {
    pub kind: OpCode,
    pub val: i32
}

impl Code {
    pub fn new(kind: OpCode) -> Code {
        Code {
            kind,
            val: 0
        }
    }

    pub fn new_val(kind: OpCode, val: i32) -> Code {
        Code {
            kind,
            val
        }
    }
}

pub fn new_push(val: i32) -> Code {
    Code::new_val(PUSH, val)
}

pub fn new_plus() -> Code {
    Code::new(PLUS)
}

pub fn new_mul() -> Code {
    Code::new(MUL)
}


pub struct VM {
    code: Vec<Code>,
    stack: Vec<i32>
}

impl VM {
    pub fn new(code: Vec<Code>) -> VM {
        VM {
            code,
            stack: Vec::new()
        }
    }

    pub fn run(&mut self) -> Option<i32> {
        self.stack = Vec::new();

        for c in &self.code {
            match c.kind {
                PUSH => {
                    self.stack.push(c.val)
                },
                MUL => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    if let (Some(r), Some(l)) = (right, left) {
                        self.stack.push(r*l);
                    }
                },
                PLUS => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    if let (Some(r), Some(l)) = (right, left) {
                        self.stack.push(r+l);
                    }
                }
            }
        }

        match self.stack.last() {
            None => None,
            Some(v) => Some(*v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show_vm_res(r: &Option<i32>) {
        print!("VM stack (top): ");
        match r {
            None => println!("{}", "empty"),
            Some(t) => println!("{}", t)
        }
    }

    #[test]
    fn test_vm_1() {
            let vc = vec![new_push(1), new_push(2), new_push(3), new_mul(), new_plus()];
            let res = VM::new(vc).run();

            show_vm_res(&res);
        assert_eq!(Some(7), res);
    }

    #[test]
    fn test_vm_2() {
        let vc = vec![new_push(2), new_push(3), new_push(5), new_plus(), new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(Some(16), res);
    }

    #[test]
    fn test_vm_3() {
        let vc = vec![];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_4() {
        let vc = vec![new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_5() {
        let vc = vec![new_plus()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_6() {
        let vc = vec![new_push(1), new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_7() {
        let vc = vec![new_push(1)];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(Some(1), res);
    }
}