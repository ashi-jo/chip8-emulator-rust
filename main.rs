fn main() {
    println!("Hello World!");
}

// Specifications

// 1. A 64x32 monochrome display, drawn to via sprites that are always 8 pixels wide and between 1 and 16 pixels tall
// 2. Sixteen 8-bit general purpose registers, referred to as V0 thru VF. VF also doubles as the flag register for overflow operations
// 3. 16-bit program counter
// 4. Single 16-bit register used as a pointer for memory access, called the I Register
// 5. 4KB RAM
// 6. 16-bit stack used for calling and returning from subroutines
// 7. 16-key keyboard input
// 8. Two special registers which decrease each frame and trigger upon reaching zero:

//Delay timer: Used for time-based game events
//Sound timer: Used to trigger the audio beep