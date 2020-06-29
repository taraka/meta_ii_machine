use std::io::{self, Read, Write};
use std::error::Error;
use std::convert::TryInto;
use num_enum::TryFromPrimitive;
use std::fs::File;
use std::env;

#[derive(TryFromPrimitive, Debug)]
#[repr(u8)]
enum Opcode {
    ADR = 1,
    TST = 2,
    BF = 3,
    ID = 4,
    BE = 5,
    CL = 6,
    CI = 7,
    OUT = 8,
    CLL = 9,
    BT = 10,
    SET = 11,
    R = 12,
    END = 13,
    STR = 14,
    NUM = 15,
    LB = 16,
    GN1 = 17,
    GN2 = 18,
    B = 19,
}

struct VM {
    header: [u8; 8],
    code: Vec<u8>,
    ip: usize,
    addrsize: usize,
    switch: bool,
    last_value: String,
    input: Vec<u8>,
    stack: Vec<Vec<u8>>,
    next_label: u32,
    output: String,
    output_label: bool
}

impl VM {
    fn new(code: Vec<u8>, input: Vec<u8>) -> VM {
        let vm = VM {
            header: code[0..8].try_into().expect("Header was wrong size"),
            code: code[8..].to_vec(),
            ip: 0,
            addrsize: (*code.get(7).expect("Unable to get addrsize")) as usize,
            switch: false,
            last_value: String::new(),
            input,
            stack: vec![vec![0, 0, 0 ,0 ,0 ,0 ,0 ,0 ,0], Vec::new(), Vec::new()], // first empty stack frame
            next_label: 0,
            output: String::new(),
            output_label: false,
        };

        vm.validate_header();

        return vm;
    }

    fn validate_header(&self) {
        if self.header != [46, 109, 101, 116, 97, 0, 0, 8] {
            panic!("Headers wasn't valid");
        }
    }

    fn run(&mut self) {
        //println!("{}: {:?}", self.code.len(), self.code);
        //println!("{:?}", &self.code[700..710]);
        loop {
            //println!("ip: {}, {}", self.ip, self.code.get(self.ip).unwrap());

            match self.get_current_opcode() {
                Opcode::ADR => self.adr(),
                Opcode::TST => self.tst(),
                Opcode::BF => self.bf(),
                Opcode::CL => self.cl(),
                Opcode::OUT => self.out(),
                Opcode::BT => self.bt(),
                Opcode::STR => self.str(),
                Opcode::CI => self.ci(),
                Opcode::R => self.r(),
                Opcode::BE => self.be(),
                Opcode::CLL => self.cll(),
                Opcode::SET => self.set(),
                Opcode::ID => self.id(),
                Opcode::LB => self.lb(),
                Opcode::GN1 => self.gn1(),
                Opcode::GN2 => self.gn2(),
                Opcode::END => self.end(),
                Opcode::NUM => self.num(),
                Opcode::B => self.b(),
            }
        }
    }

    fn get_current_opcode(&self) -> Opcode {
        (*self.code.get(self.ip).unwrap()).try_into().expect("Unable to read opcode")
    }

    fn adr(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
        // println!("{}", addr);
        self.ip = addr;
    }

    fn tst(&mut self) {
        self.ip += 1;
        let string = self.consume_string();
        //println!("Testing for string: {:?}\nInput: {:?}", string, String::from_utf8_lossy(&self.input[0..string.len()]));
        self.consume_input_whitespace();

        if self.input[0..string.len()] == *string.as_bytes() {
            self.switch = true;
            self.input.drain(0..string.len());
            self.last_value = string;
        }
        else {
            self.switch = false;
        }

    }

    fn bf(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
        if !self.switch {
            self.ip = addr;
        }
    }

    fn cl(&mut self) {
        self.ip += 1;
        let string = self.consume_string();
        self.output.push_str(&format!("{} ", string)[..]);
    }

    fn out(&mut self) {
        self.ip += 1;
        if !self.output_label {
            print!("\t");
        }
        print!("{}\n", self.output);
        self.output = String::new();
        self.output_label = false;
    }

    fn bt(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
        if self.switch {
            self.ip = addr;
        }
    }

    fn str(&mut self) {
        self.ip += 1;
        self.consume_input_whitespace();

        if *self.input.first().unwrap() as char == '\'' {
            let mut string = String::new();

            self.switch = true;
            self.input.remove(0);
            string.push('\'');

            let mut c = *self.input.first().unwrap() as char;

            while c != '\'' {
                string.push(c);
                self.input.remove(0);
                c = *self.input.first().unwrap() as char;
            }

            self.input.remove(0);
            string.push('\'');
            self.last_value = string;
        }
        else {
            self.switch = false;
        }
    }

    fn ci(&mut self) {
        self.ip += 1;

        self.output.push_str(&format!("{} ", self.last_value)[..]);
    }

    fn r(&mut self) {
        //println!("Returning {:?}", self.stack);
        self.ip += 1;

        let return_frame = self.stack.get(self.stack.len() - 3 ).unwrap();
        let framesize = *return_frame.first().unwrap();
        let mut addr_bytes: [u8;8] = [0,0,0,0,0,0,0,0];
        addr_bytes.copy_from_slice(&return_frame[1..]);
        let addr = usize::from_le_bytes(addr_bytes);

        if framesize == 0 {
            self.end();
        }

        if framesize == 3 {
            self.stack.pop();
            self.stack.pop();
            self.stack.pop();
        }
        else {
            self.stack.pop();
            let len = self.stack.len();
            self.stack.get_mut(len - 1 ).unwrap().drain(..);
            self.stack.get_mut(len - 2 ).unwrap().drain(..);
        }

        self.ip = addr;
        //println!("new Stack {:?}", self.stack);
    }

    fn be(&mut self) {
        self.ip += 1;
        if !self.switch {
            std::process::exit(1);
        }
    }

    fn cll(&mut self) {
        //println!("Calling {:?}", self.stack);
        self.ip += 1;
        let addr = self.get_addr();
        let mut framesize: u8 = 1;

        // Are the top too cells empty
        if self.stack.get(self.stack.len() -1 ).unwrap().len() != 0 ||
            self.stack.get(self.stack.len() - 2 ).unwrap().len() != 0 {
            self.stack.push(Vec::new());
            self.stack.push(Vec::new());
            framesize = 3;
        }

        self.stack.push(Vec::new());
        let len = self.stack.len();
        self.stack.get_mut(len - 3 ).unwrap().push(framesize);
        self.stack.get_mut(len - 3 ).unwrap().append(&mut self.ip.to_le_bytes().to_vec());
        self.ip = addr;
    }

    fn set(&mut self) {
        self.ip += 1;
        self.switch = true;
    }

    fn id(&mut self) {
        self.ip += 1;
        self.consume_input_whitespace();

        if (*self.input.first().unwrap() as char).is_ascii_alphabetic() {
            self.switch = true;

            let mut string = String::new();

            loop {
                let c = *self.input.first().unwrap() as char;
                if !c.is_ascii_alphanumeric() {
                    break;
                }
                string.push(c);
                self.input.remove(0);

            }
            self.last_value = string;
        }
        else {
            self.switch = false;
        }
    }

    fn lb(&mut self) {
        self.ip += 1;
        self.output_label = true;
    }

    fn gn1(&mut self) {
        self.ip += 1;
        let stack_i = self.stack.len() -2;

        if self.stack.get(stack_i).unwrap().len() == 0 {
            // Mutable so it can be drained in to the cell
            let mut new_label = self.new_label();
            // Probably be a better way to do this, but this was quick
            unsafe { self.stack.get_mut(stack_i).unwrap().append(new_label.as_mut_vec()); }
        }

        self.output.push_str(&format!("{} ", String::from_utf8((*self.stack.get(stack_i).unwrap()).clone()).unwrap())[..]);
    }

    fn gn2(&mut self) {
        self.ip += 1;
        let stack_i = self.stack.len() - 1;

        if self.stack.get(stack_i).unwrap().len() == 0 {
            // Mutable so it can be drained in to the cell
            let mut new_label = self.new_label();
            // Probably be a better way to do this, but this was quick
            unsafe { self.stack.get_mut(stack_i).unwrap().append(new_label.as_mut_vec()); }
        }

        self.output.push_str(&format!("{} ", String::from_utf8((*self.stack.get(stack_i).unwrap()).clone()).unwrap())[..]);
    }

    fn end(&mut self) {
        std::process::exit(0);
    }

    fn b(&mut self) {
        self.ip += 1;
        self.ip = self.get_addr();
    }

    fn num(&mut self) {
        // Todo... but not used in the metacompiler
        self.ip += 1;
    }

    fn get_addr(&mut self) -> usize {
        let mut bytes: [u8;8] = [0,0,0,0,0,0,0,0];
        bytes.copy_from_slice( &self.code[self.ip..(self.ip + self.addrsize)]);
        let addr = usize::from_le_bytes(bytes);
        self.ip += self.addrsize;
        //println!("Got addr: {}", addr);
        return addr;
    }

    fn get_input_string(&mut self) -> String {
        let mut string =  String::new();
        let mut offset: usize = 0;

        loop {
            let c = *self.code.get(self.ip + offset).unwrap();
            if c != 0 {
                string.push(c as char);
            }
            offset += 1;
            if c == 0 {
                break;
            }
        }

        //println!("Got string: {}", string);

        return string;
    }

    fn consume_string(&mut self) -> String {
        let mut string =  String::new();

        loop {
            let c = *self.code.get(self.ip).unwrap();
            if c != 0 {
                string.push(c as char);
            }
            self.ip += 1;
            if c == 0 {
                break;
            }
        }

        //println!("Got string: {}", string);

        return string;
    }

    fn consume_input_whitespace(&mut self) {
        while " \t\n\r".contains(*self.input.first().unwrap_or(&b'a') as char) {
            //Could use a better way to do this as this will copy all the elements
            self.input.remove(0);
        }
    }

    fn new_label(&mut self) -> String {
        let char = ((self.next_label / 100) as u8 + 65) as char;
        let num = self.next_label % 100;
        self.next_label += 1;

        return format!("{}{}", char, num);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(args.get(1).expect("Please provide a program file"))?;

    let mut code_bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut code_bytes)?;

    let mut input_bytes: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut input_bytes)?;

    let mut vm = VM::new(code_bytes, input_bytes);

    vm.run();

    Ok(())
}
