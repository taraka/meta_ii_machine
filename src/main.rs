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
    addrsize: usize
}

impl VM {
    fn new(input: Vec<u8>) -> VM {
        let vm = VM {
            header: input[0..8].try_into().expect("Header was wrong size"),
            code: input[8..].to_vec(),
            ip: 0,
            addrsize: (*input.get(7).expect("Unable to get addrsize")) as usize
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
        loop {
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
                _ => panic!("Unknown opcode {:#?}", self.get_current_opcode())
            }
        }
    }

    fn get_current_opcode(&self) -> Opcode {
        (*self.code.get(self.ip).unwrap()).try_into().expect("Unable to read opcode")
    }

    fn adr(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
    }

    fn tst(&mut self) {
        self.ip += 1;
        let string = self.get_string();
    }

    fn bf(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
    }

    fn cl(&mut self) {
        self.ip += 1;
        let string = self.get_string();
    }

    fn out(&mut self) {
        self.ip += 1;
    }

    fn bt(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
    }

    fn str(&mut self) {
        self.ip += 1;
    }

    fn ci(&mut self) {
        self.ip += 1;
    }

    fn r(&mut self) {
        self.ip += 1;
    }

    fn be(&mut self) {
        self.ip += 1;
    }

    fn cll(&mut self) {
        self.ip += 1;
        let addr = self.get_addr();
    }

    fn set(&mut self) {
        self.ip += 1;
    }

    fn id(&mut self) {
        self.ip += 1;
    }

    fn lb(&mut self) {
        self.ip += 1;
    }

    fn gn1(&mut self) {
        self.ip += 1;
    }

    fn gn2(&mut self) {
        self.ip += 1;
    }

    fn end(&mut self) {
        std::process::exit(0);
    }

    fn get_addr(&mut self) -> usize {
        let addr = usize::from_le_bytes(self.code[self.ip..(self.ip+self.addrsize)].try_into().expect("Failed raising address"));
        self.ip += self.addrsize;
        //println!("Got addr: {}", addr);
        return addr;
    }

    fn get_string(&mut self) -> String {
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
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(args.get(1).expect("Please provide a program file"))?;
    let mut code_bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut code_bytes)?;

    let mut vm = VM::new(code_bytes);
    //println!("{:?}", vm.code);
    vm.run();

    Ok(())
}
