/* Memory Spaces Definitions, 512K flash bank 1, 64K ram */
/* No bootloader present */

MEMORY
{
  rom (rx)  : ORIGIN = 0x000000001a000000, LENGTH = 0x0000000000040000
  prog (rx) : ORIGIN = 0x000000001a040000, LENGTH = 0x0000000000040000
  ram (rwx) : ORIGIN = 0x0000000010000000, LENGTH = 0x0000000000008000
}

MPU_MIN_ALIGN = 8K;
