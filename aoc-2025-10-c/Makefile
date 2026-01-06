ACTION ?= main
CC ?= gcc
OUTPUT=$(ACTION).out
C_ACCUMULATIVE_HASH_SET_DIR := $$(pwd)/../c-bindings/c-accumulative-hash-set/dylib
OBJECTS ?= \
	utils/log.o \
	utils/numbers.o \
	utils/str.o \
	parse/bracket.o \
	parse/line.o \
	types/combinations.o \
	types/order.o \
	types/scenario.o \
	types/solution.o \
	types/validate.o \
	types/vector.o \
	solve/bisect.o \
	solve/brute.o \
	solve/divide.o \
	solve/common.o \
	solve/mask.o \

OBJECTS_WITHOUT_ACTION = $(filter-out $(ACTION).o, $(OBJECTS))
DFLAGS = $(foreach flag,$(FLAGS),-D$(flag))

	
build: clean $(C_ACCUMULATIVE_HASH_SET_DIR)/libc_accumulative_hash_set.so $(OBJECTS)
	@echo Building \\x1b[1m$(OUTPUT)\\x1b[0m with "\\x1b[1m$(DFLAGS)\\x1b[0m"...
	@$(CC) $(OBJECTS) $(DFLAGS) $(ACTION).c \
		-lm \
		-I$(C_ACCUMULATIVE_HASH_SET_DIR) \
		-L$(C_ACCUMULATIVE_HASH_SET_DIR) \
		-lc_accumulative_hash_set \
		-Wl,-rpath,$(C_ACCUMULATIVE_HASH_SET_DIR) \
		-o $(OUTPUT)

$(C_ACCUMULATIVE_HASH_SET_DIR)/libc_accumulative_hash_set.so:
	@$(MAKE) -C ../c-bindings/c-accumulative-hash-set build

%.o: %.c
	@echo Building \\x1b[1m$@\\x1b[0m with \\x1b[1m$(DFLAGS)\\x1b[0m...
	@$(CC) $(DFLAGS) -c $< -o $@

run_only:
	@./$(OUTPUT)

run: build run_only

test: ACTION:=tests try_accumulative_hash_set
test: FLAGS?=UNIT_TEST
test:
test: clean
	@for action in $(ACTION); \
	do \
		echo Running test for \\x1b[1m$$action\\x1b[0m... && \
		ACTION=$$action FLAGS="$(FLAGS)" OBJECTS="$(OBJECTS_WITHOUT_ACTION)" $(MAKE) run && \
		echo \\x1b[32mFinished test\\x1b[0m for \\x1b[1m$$action\\x1b[0m.; \
	done

clean:
	@rm -f $(OBJECTS)
