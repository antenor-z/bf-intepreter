package main

import (
	"fmt"
	"os"
)

func check(err error, message string) {
	if err != nil {
		panic(message + ": " + err.Error())
	}
}

const MEMORY_SIZE int = 30000

type bf_program struct {
	program_memory      []byte
	memory              []byte
	instruction_pointer int
	data_pointer        int
}

func main() {
	if len(os.Args) != 2 {
		panic("supply the file with the source")
	}

	bf := bf_program{
		program_memory:      make([]byte, MEMORY_SIZE),
		memory:              make([]byte, MEMORY_SIZE),
		instruction_pointer: 0,
		data_pointer:        0,
	}

	fp, err := os.Open(os.Args[1])
	check(err, "error while opening file")

	_, err = fp.Read(bf.program_memory)
	check(err, "error while reading file")

	for bf.instruction_pointer < len(bf.program_memory) {
		instruction := bf.program_memory[bf.instruction_pointer]

		if instruction == '>' {
			// Increment the data pointer by one (to point to the next cell to the right).
			bf.data_pointer += 1
			if bf.data_pointer > len(bf.memory) {
				panic("memory access after end of memory")
			}

		} else if instruction == '<' {
			// Decrement the data pointer by one (to point to the next cell to the left).
			bf.data_pointer -= 1
			if bf.data_pointer < 0 {
				panic("memory access at less than zero addr")
			}

		} else if instruction == '+' {
			// Increment the byte at the data pointer by one.
			bf.memory[bf.data_pointer] += 1

		} else if instruction == '-' {
			// Decrement the byte at the data pointer by one.
			bf.memory[bf.data_pointer] -= 1

		} else if instruction == '.' {
			// Output the byte at the data pointer.
			print(string(bf.memory[bf.data_pointer]))

		} else if instruction == ',' {
			// Accept one byte of input, storing its value in the byte at the data pointer.
			_, err := fmt.Scanf("%c", &bf.memory[bf.data_pointer])
			check(err, "error on struction ','")

		} else if instruction == '[' {
			// If the byte at the data pointer is zero, then instead of moving the
			// instruction pointer forward to the next command, jump it forward to the
			// command after the matching ] command.
			if bf.memory[bf.data_pointer] == 0 {
				var nesting_count int = 0
				for {
					if bf.program_memory[bf.instruction_pointer] == ']' {
						nesting_count -= 1
					}
					if bf.program_memory[bf.instruction_pointer] == '[' {
						nesting_count += 1
					}

					if nesting_count == 0 {
						break
					}

					bf.instruction_pointer += 1
				}
			}

		} else if instruction == ']' {
			// If the byte at the data pointer is nonzero, then instead of moving the
			// instruction pointer forward to the next command, jump it back to the
			// command after the matching [ command.
			if bf.memory[bf.data_pointer] != 0 {
				var nesting_count int = 0
				for {
					if bf.program_memory[bf.instruction_pointer] == '[' {
						nesting_count -= 1
					}
					if bf.program_memory[bf.instruction_pointer] == ']' {
						nesting_count += 1
					}

					if nesting_count == 0 {
						break
					}

					bf.instruction_pointer -= 1
				}
			}
		}

		bf.instruction_pointer += 1
	}

}
