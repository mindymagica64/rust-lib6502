mod lib6502 {
	enum Opcodes {
		ADCImmediate = 0x69,
		ADCZeroPage = 0x65,
		ADCZeroPageX = 0x75,
		ADCAbsolute = 0x6d,
		ADCAbsoluteX = 0x7d,
		ADCAbsoluteY = 0x79,
		ADCIndirectX = 0x61,
		ADCIndirectY = 0x71,
		ANDImmediate = 0x29,
		ANDZeroPage = 0x25,
		ANDZeroPageX = 0x35,
		ANDAbsolute = 0x2d,
		ANDAbsoluteX = 0x3d,
		ANDAbsoluteY = 0x39,
		ANDIndirectX = 0x21,
		ANDIndirectY = 0x31
	}
}