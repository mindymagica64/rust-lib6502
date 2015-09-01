use lib6502::types;


mod lib6502 {
	struct Registers {
		sp: Address,
		prg_counter: Address,
		acc: Byte,
		x: Byte,
		y: Byte,
		status: Byte
	}

	impl Default for Registers {
		fn default() -> Registers {
			Registers {
				sp = 0x0100,
				prg_counter = 0x8000,
				acc = 0,
				x = 0,
				y = 0,
				status = StatusFlags::bit5
			}
		}

		impl Registers {
			fn update_status(&mut self, status_op: StatusOps, flag: StatusFlags) {
				match status_op {
					StatusOps::Set => status = status | flag,
					StatusOps::Clear => status = status & ~flag;
				}
			}
		}
	}

	enum StatusOps {
		Set,
		Clear
	}

	enum StatusFlags{
		Carry = 1;
		Zero = 1 << 1;
		Interrupt = 1 << 2;
		Dec = 1 << 3;
		Brk = 1 << 4;
		Bit5 = 1 << 5;
		Overflow = 1 << 6;
		Sign =  1 << 7;
	}
		
}