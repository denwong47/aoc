ACTION ?= main
CC ?= gcc
OUTPUT=$(ACTION).out
OBJECTS ?= compare.o consolidate.o func.o parse.o range.o utils.o
OBJECTS_WITHOUT_ACTION = $(filter-out $(ACTION).o, $(OBJECTS))
DFLAGS = $(foreach flag,$(FLAGS),-D$(flag))

build: clean $(OBJECTS)
	@echo Building \\x1b[1m$(OUTPUT)\\x1b[0m with "\\x1b[1m$(DFLAGS)\\x1b[0m"...
	@$(CC) $(OBJECTS) $(DFLAGS) -o $(OUTPUT) $(ACTION).c

%.o: %.c
	@echo Building \\x1b[1m$@\\x1b[0m with \\x1b[1m$(DFLAGS)\\x1b[0m...
	@$(CC) -o $(OUTPUT) $(DFLAGS) -c $< -o $@

run_only:
	@./$(OUTPUT)

run: build run_only

test: ACTION:=tests
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
	rm -f $(OBJECTS)
