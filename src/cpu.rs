use lib6502::types;
use lib6502::Register;
use lib6502::Memory;
use lib6502::AddressCalculator;
use lib6502::Instructions;

mod lib6502 {

	struct Environment {
		registers: Registers,
		memory: Memory
	}

	struct CPU {
		env: Environment,
		interactive: false,
		finished: false,
	}

	impl CPU {
		fn loadBytes(&mut self, data) {
			std::slice::bytes::copy_memory(&data[..], &env.memory.rom[..]);
			registers.program_counter = (memory.read(WithAddress(0xfffd)) as Address) << 8 | (memory.read(WithAddress(0xfffc)) as Address);
		}

		fn run_program(&mut self) {
			if(!interactive){
				while !finished {
				 	Instructions::nextState(env);
				 }
			} else{
				Instructions::nextState(env);
			}
		}

		fn eval(&mut self, block) {
			memory.rom = [0x0; 0x8000]
			std::slice::bytes::copy_memory(&block[..], &memory.rom[..]);
			registers.program_counter = 0x8000;
			while !finished {
				Instructions::nextState(env);
			}
		}	
	}
}
