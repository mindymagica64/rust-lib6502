use lib6502::types;
use lib6502::Register;
use lib6502::Memory;
use lib6502::Core;

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
		fn load_bytes(&mut self, data) {
			std::slice::bytes::copy_memory(&data[..], &env.memory.rom[..]);
			env.registers.program_counter = (env.memory.read(WithAddress(0xfffd)) as Address) << 8 | (env.memory.read(WithAddress(0xfffc)) as Address);
		}

		fn run_program(&mut self) {
			if(!interactive){
				while !finished {
				 	Core::nextState(env);
				 }
			} else{
				Core::nextState(env);
			}
		}

		fn eval(&mut self, block) {
			env.memory.rom = [0x0; 0x8000]
			std::slice::bytes::copy_memory(&block[..], &memory.rom[..]);
			env.registers.program_counter = 0x8000;
			while !finished {
				Core::nextState(env);
			}
		}	
	}
}
