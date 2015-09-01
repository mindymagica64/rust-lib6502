use lib6502::types;
use lib6502::Environment;
use lib6502::Opcodes;
use lib6502::MemoryModes;

mod lib6502 {
	mod Core {
		fn nextState(&mut env: Environment)  {
			let opcode = env.memory.read(MemoryModes::WithAddress(env.registers.program_counter));
			env.registers.program_counter += 1;
			match opcode {
				Opcodes::ADCImmediate => env.registers.program_counter += 1; instructions::ADC(address_calculators::Immediate, env),
				Opcodes::ADCZeroPage => env.registers.program_counter += 1; instructions::ADC(address_calculators::ZeroPage, env),
				Opcodes::ADCZeroPageX => env.registers.program_counter += 1; instructions::ADC(address_calculators::ZeroPageX, env),
				Opcodes::ADCAbsolute => env.registers.program_counter += 2; instructions::ADC(address_calculators::Absolute, env),
				Opcodes::ADCAbsoluteX => env.registers.program_counter += 2; instructions::ADC(address_calculators::AbsoluteX, env),
				Opcodes::ADCAbsoluteY => env.registers.program_counter += 2; instructions::ADC(address_calculators::AbsoluteY, env),
				Opcodes::ADCIndirectX => env.registers.program_counter += 1; instructions::ADC(address_calculators::IndirectX, env),
				Opcodes::ADCIndirectY => env.registers.program_counter += 1; instructions::ADC(address_calculators::IndirectY, env),
				Opcodes::ANDImmediate => env.registers.program_counter += 1; instructions::AND(address_calculators::Immediate, env),
				Opcodes::ANDZeroPage => env.registers.program_counter += 1; instructions::AND(address_calculators::ZeroPage, env),
				Opcodes::ANDZeroPageX => env.registers.program_counter += 1; instructions::AND(address_calculators::ZeroPageX, env),
				Opcodes::ANDAbsolute => env.registers.program_counter += 2; instructions::AND(address_calculators::Absolute, env),
				Opcodes::ANDAbsoluteX => env.registers.program_counter += 2; instructions::AND(address_calculators::AbsoluteX, env),
				Opcodes::ANDAbsoluteY => env.registers.program_counter += 2; instructions::AND(address_calculators::AbsoluteY, env),
				Opcodes::ANDIndirectX => env.registers.program_counter += 1; instructions::AND(address_calculators::IndirectX, env),
				Opcodes::ANDIndirectY => env.registers.program_counter += 1; instructions::AND(address_calculators::IndirectY, env),
			}
		}
		

		mod address_calculators {
			pub trait AddressCalculator {
				fn addr(&self, env: Environment) -> Address;
			}
			pub struct Immediate;
			pub struct ZeroPage;
			pub struct ZeroPageX;
			pub struct Absolute;
			pub struct AbsoluteX;
			pub struct AbsoluteY;
			pub struct IndirectX;
			pub struct IndirectY;
			impl AddressCalculator for Immediate {
				fn addr(&self, env: Environment) -> Address {
					env.registers.program_counter - 1
				}
			}
			impl AddressCalculator for ZeroPage {
				fn addr(&self, env: Environment) -> Address {
					env.memory.read(MemoryModes::WithAddress(env.memory.program_counter - 1)) as Address
				}
			}
			impl AddressCalculator for ZeroPageX {
				fn addr(&self, env: Environment) -> Address {
					((env.memory.read(MemoryModes::WithAddress(env.memory.program_counter - 1)) + env.registers.x) & 0xffu16) as Address
				}
			} 

			impl AddressCalculator for Absolute {
				fn addr(&self, env: Environment) -> Address {
					env.memory.read(MemoryModes::WithAddress(env.registers.program_counter - 1)) as Address << 8
						| env.memory.read(MemoryModes::WithAddress(env.registers.program_counter - 2)) as Address
				}
			}

			impl AddressCalculator for AbsoluteX {
				fn addr(&self, env: Environment) -> Address {
					Absolute.addr(env) + env.registers.x
				}
			}

			impl AddressCalculator for AbsoluteY {
				fn addr(&self, env: Environment) -> Address {
					Absolute.addr(env) + env.registers.y
				}
			}

			impl AddressCalculator for IndirectX {
				fn addr(&self, env: Environment) -> Address {
					((env.registers.x + env.memory,read(env.registers.program_counter - 1)) & 0xffu16) as Address
				}
			}

			impl AddressCalculator for IndirectY {
				fn addr(&self, env: Environment) -> Address {
					let lsb_pos = env.memory.read(MemoryModes::WithAddress(env.registers.program_counter - 1));
					let msb_pos = lsb_pos + 1;
					env.memory.read(MemoryModes::WithAddress(msb_pos)) as Address << 8 | env.memory.read(MemoryModes::WithAddress(lsb_pos)) as Address 
						+ env.registers.y
				}
			}
		}

		mod instructions {
			fn ADC(addrcalc: address_calculators::AddressCalculator, &mut env: Environment) {
				let operand = env.memory.read(addrcalc.addr(env));
				let prevacc = env.registers.accum;
				match operand.checked_add(env.registers.accum) {
					Some(val) => env.registers.accum = val; env.registers.update_status(StatusOps::Clear, StatusFlags::Overflow),
					None => env.registers.accum += operand; env.registers.update_status(StatusOps::Set, StatusFlags::Overflow)
				}
				if((operand as u16 + prevacc as u16) & (1 << 8) == 1){
					env.registers.update_status(StatusOps::Set, StatusFlags::Carry);
				} else{
					env.registers.update_status(StatusOps::Clear, StatusFlags::Carry);
				}
				if(env.registers.accum << 7 == 1){
					env.registers.update_status(StatusOps::Set, StatusFlags::Sign);
				} else{
					env.registers.update_status(StatusOps::Clear, StatusFlags::Sign);
				}
				if(env.registers.accum == 0){
					env.registers.update_status(StatusOps::SetZero);
				} else{
					env.registers.update_status(StatusOps::ClearZero);
				}
			}

			fn AND(addrcalc: address_calculators::AddressCalculator, &mut env: Environment) {
				env.registers.accum &= env.memory.read(addrcalc.addr(env));
				if(env.registers.accum << 7 == 1){
					env.registers.update_status(StatusOps::Set, StatusFlags::Sign);
				} else{
					env.registers.update_status(StatusOps::Clear, StatusFlags::Sign);
				}
				if(env.registers.accum == 0){
					env.registers.update_status(StatusOps::SetZero);
				} else{
					env.registers.update_status(StatusOps::ClearZero);
				}
			} 
		}
	}
}