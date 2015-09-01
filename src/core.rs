use lib6502::types;

mod lib6502 {
	mod Core {
		fn nextState(&mut env: Environment)  {
			let opcode = env.memory.read(WithAddress(env.registers.program_counter));
			env.registers.program_counter += 1;
			match opcode {
				Opcodes::ADCImmediate => Instructions::ADC(AddressCalculator::Immediate, env),
				Opcodes::ADCZeroPage => Instructions::ADC(AddressCalculator::ZeroPage, env),
				Opcodes::ADCZeroPageX => Instructions::ADC(AddressCalculator::ZeroPageX, env),
				Opcodes::ADCAbsolute => env.registers.program_counter += 1; Instructions::ADC(AddressCalculator::Absolute, env),
				Opcodes::ADCAbsoluteX => env.registers.program_counter += 1; Instructions::ADC(AddressCalculator::AbsoluteX, env),
				Opcodes::ADCAbsoluteY => env.registers.program_counter += 1; Instructions::ADC(AddressCalculator::AbsoluteY, env),
				Opcodes::ADCIndirectX => Instructions::ADC(AddressCalculator::IndirectX, env),
				Opcodes::ADCIndirectY => Instructions::ADC(AddressCalculator::IndirectY, env),
			}
		}
		struct AddressCalculator {
			Immediate: (Fn(Environment) -> Address),
			ZeroPage: (Fn(Environment) -> Address),
			ZeroPageX: (Fn(Environment) -> Address),
			Absolute: (Fn(Environment) -> Address),
			AbsoluteX: (Fn(Environment) -> Address),
			AbsoluteY: (Fn(Environment) -> Address),
			IndirectX: (Fn(Environment) -> Address),
			IndirectY: (Fn(Environment) -> Address)
		}

		impl AddressCalculator {
			fn Immediate(env) {
				env.registers.program_counter;
			}

			fn ZeroPage(env) {
				env.memory.read(WithAddress(env.registers.program_counter));
			}

			fn ZeroPageX(env) {
				(ZeroPage(env) + env.registers.x) & 0xffu16;
			}

			fn Absolute(env) {
				env.memory.read(WithUpperAndLower(env.memory.read(WithAddress(env.registers.program_counter)), env.memory.read(WithAddress(env.registers.program_counter))));
			}

			fn AbsoluteX(env) {
				Absolute(env) + env.registers.x;
			}

			fn AbsoluteY(env) {
				Absolute(env) + env.registers.y;
			}

		}
		mod Instructions {
			fn ADC(addrcalc: AddressCalculator, &mut env: Environment) {
				let operand = env.memory.read(addrcalc(env));
				let prevacc = env.registers.accum;
				match operand.checked_add(env.registers.accum) {
					Some(val) => env.registers.accum = val; env.registers.update_status(StatusOps::ClearOverflow),
					None => env.registers.accum += operand; env.registers.update_status(StatusOps::SetOverflow)
				}
				if((operand as u16 + prevacc as u16) & (1 << 8) == 1){
					env.registers.update_status(StatusOps::SetCarry);
				} else{
					env.registers.update_status(StatusOps::ClearCarry);
				}
				if(env.registers.accum << 7 == 1){
					env.registers.update_status(StatusOps::SetSign);
				} else{
					env.registers.update_status(StatusOps::ClearSign);
				}
				if(env.registers.accum == 0){
					env.registers.update_status(StatusOps::SetZero);
				} else{
					env.registers.update_status(StatusOps::ClearZero);
				}
			}

			fn AND(addrcalc: Add)
		}
	}
}