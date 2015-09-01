use lib6502::types;

mod lib6502{

	enum MemoryModes {
		WithAddress(Address),
		WithUpperAndLower(Byte, Byte)
	}

	pub struct Memory {
	    data: [Byte; 0x8000]
	    rom: [Byte; 0x8000]
	}

	impl Memory {
		fn read(&self, addr: MemoryModes) -> Byte {
			match addr {
				MemoryModes::WithAddress(a) => mem_read(a),
				MemoryModes::WithUpperAndLower(u, l) => mem_read(((u as Address) << 8 | (l as Address)));
			}
		}

		fn write(&mut self, addr: MemoryModes, value: Byte) -> Byte {
			match addr {
				MemoryModes::WithAddress(a) => mem_write(a, value),
				MemoryModes::WithUpperAndLower(u, l) => mem_write(((u as Address) << 8 | (l as Address)), value);
			}
		}

		fn mem_read(&self, addr: Address) -> Byte {
			//if its a mirrored value, pass it through
			if(addr >= 0x2008 && addr < 0x4000){
				data[0x2000 + addr % 8];
			} else if(addr > 0x7fff) {
				rom[0x8000 - addr];
			} else{
				data[addr];
			}
		}

		fn mem_write(&mut self, addr: Address, value: Byte) {
			if(addr >= 0x2008 && addr < 0x4000){
				data[0x2000 + addr % 8] = value;
			} else if(addr > 0x7fff){
				rom[0x8000 - addr]
			} 
			else {
				data[addr] = value;
			}	
		}

	}
}
