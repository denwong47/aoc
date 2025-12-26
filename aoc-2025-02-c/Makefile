ACTION ?= main
OUTPUT=$(ACTION).out
DFLAGS = $(foreach flag,$(FLAGS),-D$(flag))

build:
	gcc -o $(OUTPUT) $(DFLAGS) ./$(ACTION).c -lm

run_only:
	@./$(OUTPUT)

run: build run_only

test: ACTION:=func
test: FLAGS?=UNIT_TEST
test:
	@for action in $(ACTION); \
	do \
		echo Running test for \\x1b[1m$$action\\x1b[0m... && \
		ACTION=$$action FLAGS="$(FLAGS)" $(MAKE) run && \
		echo \\x1b[32mFinished test\\x1b[0m for \\x1b[1m$$action\\x1b[0m.; \
	done
