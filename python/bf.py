import sys


instruction_pointer = 0
data_pointer = 0

memory = [0] * 30000

if len(sys.argv) != 2:
    print("supply the file with the source")
    exit(1)

with open(sys.argv[1]) as fp:
    program_memory = fp.read()

program_size = len(program_memory)

def panic(message: str):
    print("PANIC", message)
    print(f"{instruction_pointer=} {data_pointer=}")
    print(program_memory[instruction_pointer - 50: instruction_pointer + 50])
    exit(1)

while instruction_pointer < len(program_memory):
    instruction = program_memory[instruction_pointer]

    if instruction == ">":
        # Increment the data pointer by one (to point to the next cell to the right). 
        data_pointer += 1
        if data_pointer > len(memory):
            panic(f"memory access after {memory} bytes")

    elif instruction == "<":
        # Decrement the data pointer by one (to point to the next cell to the left). 
        data_pointer -= 1
        if data_pointer < 0:
            panic(f"memory access at less than zero addr")

    elif instruction == "+":
        # Increment the byte at the data pointer by one. 
        memory[data_pointer] += 1

    elif instruction == "-":
        # Decrement the byte at the data pointer by one. 
        memory[data_pointer] -= 1

    elif instruction == ".":
        # Output the byte at the data pointer. 
        print(chr(memory[data_pointer]), end="")

    elif instruction == ",":
        # Accept one byte of input, storing its value in the byte at the data pointer. 
        memory[data_pointer] = ord(sys.stdin.read(1))
        sys.stdin.flush()

    elif instruction == "[":
        # If the byte at the data pointer is zero, then instead of moving the 
        # instruction pointer forward to the next command, jump it forward to the 
        # command after the matching ] command. 
        if memory[data_pointer] == 0:
            nesting_count = 0
            while True:
                if program_memory[instruction_pointer] == "]":
                    nesting_count -= 1
                if program_memory[instruction_pointer] == "[":
                    nesting_count += 1

                if nesting_count == 0: break

                instruction_pointer += 1

    elif instruction == "]":
        # If the byte at the data pointer is nonzero, then instead of moving the 
        # instruction pointer forward to the next command, jump it back to the 
        # command after the matching [ command.
        if memory[data_pointer] != 0:
            nesting_count = 0
            while True:
                if program_memory[instruction_pointer] == "[":
                    nesting_count -= 1
                if program_memory[instruction_pointer] == "]":
                    nesting_count += 1
                
                if nesting_count == 0: break
                    
                instruction_pointer -= 1

    instruction_pointer += 1


