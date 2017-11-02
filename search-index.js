var searchIndex = {};
searchIndex["assembler"] = {"doc":"","items":[],"paths":[]};
searchIndex["core"] = {"doc":"Core utilities","items":[[0,"isa","core","Instruction set types",null,null],[4,"OpCode","core::isa","A MIPS opcode",null,null],[13,"addi","","Add (immediate)",0,null],[13,"addiu","","Add (immediate, unsigned)",0,null],[13,"andi","","Bitwise and (immediate)",0,null],[13,"beq","","Branch if equal",0,null],[13,"bne","","Branch if not equal",0,null],[13,"lbu","","Load byte (unsigned)",0,null],[13,"lhu","","Load half-word (unsigned)",0,null],[13,"lui","","Load upper (immediate)",0,null],[13,"lw","","Load word",0,null],[13,"ori","","Bitwise or (immediate)",0,null],[13,"sb","","Store byte",0,null],[13,"sh","","Store half-word",0,null],[13,"slti","","Set to 1 if less than (immediate)",0,null],[13,"sw","","Store word",0,null],[13,"j","","Unconditional jump",0,null],[13,"jal","","Jump and link",0,null],[4,"Instruction32","","A 32-bit MIPS instruction",null,null],[13,"R","","A Register type instruction",1,null],[12,"src1","core::isa::Instruction32","Source 1",1,null],[12,"src2","","Source 2",1,null],[12,"dst","","Destination",1,null],[12,"shift","","Shift about",1,null],[12,"func","","Function code",1,null],[13,"I","core::isa","An Immediate instruction (32-bit)",1,null],[12,"opcode","core::isa::Instruction32","OpCode",1,null],[12,"src","","Source",1,null],[12,"dst","","Destination",1,null],[12,"imm","","Immediate value",1,null],[13,"J","core::isa","A Jump type instruction, used for jumps",1,null],[12,"opcode","core::isa::Instruction32","OpCode",1,null],[12,"offset","","Offset from pc",1,null],[4,"FunctionCode","core::isa","R type function code",null,null],[13,"add","","Add",2,null],[13,"addu","","Add (unsigned)",2,null],[13,"and","","Bitwise and",2,null],[13,"div","","Divide",2,null],[13,"divu","","Divide (unsigned)",2,null],[13,"mfhi","","Move from high",2,null],[13,"mflo","","Move from low",2,null],[13,"mult","","Multiply",2,null],[13,"multu","","Multiply (unsigned)",2,null],[13,"nor","","Nor",2,null],[13,"xor","","Bitwise Exclusive Or",2,null],[13,"or","","Bitwise Or",2,null],[13,"slt","","Set to 1 if less than",2,null],[13,"sltu","","Set to 1 if less than (unsigned)",2,null],[13,"sll","","Logical Shift left",2,null],[13,"srl","","Logical Shift right (0-extended)",2,null],[13,"sra","","Arithmetic Shift Right (sign-extended)",2,null],[13,"sub","","Subtract",2,null],[13,"subu","","Subtract (unsigned)",2,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"opcode"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"opcode"}],"output":{"name":"bool"}}],[11,"from","","",0,{"inputs":[{"name":"u8"}],"output":{"name":"self"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"instruction32"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"instruction32"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"instruction32"}],"output":{"name":"bool"}}],[11,"from","","",1,{"inputs":[{"name":"u32"}],"output":{"name":"instruction32"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"functioncode"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"functioncode"}],"output":{"name":"bool"}}],[11,"from","","",2,{"inputs":[{"name":"u8"}],"output":{"name":"self"}}],[0,"components","core","Components of a MIPS systems",null,null],[3,"Core","core::components","A MIPS core",null,null],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"core"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"core"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"core"}],"output":{"name":"bool"}}],[11,"with_memory_size","","Create a new core with a specified memory size",3,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"step","","Step the processor one cycle",3,{"inputs":[{"name":"self"}],"output":null}],[11,"pc","","Get a copy of the program counter",3,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"ir","","Get the instruction Register",3,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"register","","Get a copy of the Processor's `RegisterFile`",3,{"inputs":[{"name":"self"}],"output":{"name":"registerfile"}}],[11,"float_registers","","Get a copy of the Processor's float `RegisterFile`",3,{"inputs":[{"name":"self"}],"output":{"name":"registerfile"}}],[11,"is_halted","","Is the `Core` halted?",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"memory","","Get a copy of the `Core`'s memoy",3,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"halt","","Halt the `Core`",3,{"inputs":[{"name":"self"}],"output":null}],[11,"reset","","Reset the `Core`",3,{"inputs":[{"name":"self"}],"output":null}],[11,"overflow","","Is the overflow flag set",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[6,"Core32","","32-bit MIPS `Core`",null,null],[6,"Core64","","64-bit MIPS `Core`",null,null]],"paths":[[4,"OpCode"],[4,"Instruction32"],[4,"FunctionCode"],[3,"Core"]]};
searchIndex["simulator"] = {"doc":"","items":[],"paths":[]};
initSearch(searchIndex);
