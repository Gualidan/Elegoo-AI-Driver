## How the project is compiled and flashed onto the board
Running cargo build produces a .elf file under target/avr-none/debug/elegoo-conqueror-ai-driven.elf then cargo run calls avrdude to flash the program onto the chips memory via the Optiboot bootloader. The program then takes over.

(31/10/26) Currently RAM usage is at 67 bytes: elegoo-conqueror-ai-driven.elf  :
section                         size       addr
.data                           0x42   0x800100
.text                          0x836        0x0
.bss                             0x1   0x800142
.comment                        0x35        0x0
.note.gnu.avr.deviceinfo        0x40        0x0
.debug_aranges                0x1038        0x0
.debug_info                  0x59204        0x0
.debug_abbrev                 0x116c        0x0
.debug_line                  0x2ae68        0x0
.debug_str                   0x34989        0x0
.debug_loc                   0x91527        0x0
.debug_ranges                 0x7338        0x0
.debug_line_str                 0x87        0x0
Total                       0x153d6d
