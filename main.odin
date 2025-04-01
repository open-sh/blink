#+feature dynamic-literals
package main

import "core:fmt"
import "core:os"
import "core:os/os2"

import vmem "core:mem/virtual"

main :: proc() {

	fmt.println("Player tauz \n")

	arena: vmem.Arena
	arena_err := vmem.arena_init_growing(&arena)
	arena_alloc := vmem.arena_allocator(&arena)

	tauzLyrics, tauzLyrics_ok := os.read_entire_file("tauz.txt", arena_alloc)
	ensure(tauzLyrics_ok)


	command: [dynamic]string = {"cowsay", string(tauzLyrics)}
	_, stdout, _, _ := os2.process_exec(os2.Process_Desc{command = command[:]}, context.allocator)

	fmt.println(string(stdout))

	/* for i, merda in string(tauzLyrics) {
		fmt.println(i)
	} */
}
