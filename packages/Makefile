CC := arm-none-eabi-gcc
AS := arm-none-eabi-as
OBJCP := arm-none-eabi-objcopy
INC = -I./$(TARGET)/include -I./include -I../os/arch/f401/include/ -I./lib/include
OUTDIR := ./$(TARGET)/build

CFLAGS = -O0  \
	 -g -mcpu=cortex-m4 -mthumb \
	 -nostartfiles -ffunction-sections  -Wall -Wextra -mfloat-abi=soft -static -nostdlib  \

SRC:=$(wildcard ./$(TARGET)/src/**/*.c) $(wildcard ./$(TARGET)/src/*.c)
OBJ:=$(wildcard ./$(TARGET)/build/*.o)

all: $(TARGET)

$(TARGET): compile_c build_platform_f401 build_platform_f401 link convert_elf 


compile_c: $(SRC)
	echo "Executing compile_c..."
	mkdir -p ./$(TARGET)/build
	$(CC) $(CFLAGS) $(INC) -c $^ 
	mv *.o $(OUTDIR)

build_platform_f401:
	echo "Executing build_platform_f401..."
	make -C ../os/arch/f401 HWFP=0
	mv ../os/arch/f401/build/*.o ./$(TARGET)/build

link:
	echo "Executing link..."
	make -C ./linkscripts TARGET=$(TARGET)

convert_elf:
	echo "Executing convert_elf..."
	$(OBJCP) $(OUTDIR)/$(TARGET) $(OUTDIR)/$(TARGET)_binary -O binary

clean:
	rm -f $(OUTDIR)/*.o 
