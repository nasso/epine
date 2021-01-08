binary "hello"
    file { match "./src/*.c" }
    include "./include"

action "ptdr"
    run "echo lol"

--[[
%.c: %.o
    gcc -c grhosegbvhnkis gvhfnuikgh nrfuo hgvropqhg vuozqgvr


%.s: %.o
    as hgriousqhngikrfngopmrfjzqsougfhzqukgrhjfdzqghrdfs


%.java: %.class
    javac ghnrziqhrndfilsghnbrdiklnvqrfdsnbvirfmdn

hello: SRC := $(wildcard ./src/*.c)
hello: OBJ := $(SRC:.c=.o)
hello: $(OBJ)
hello: CFLAGS := -I./include
hello:
    gcc -W -Wall -Wextra $(CFLAGS) $(OBJ)
hello: SRC := $(wildcard ./src/*.java)
hello: OBJ := $(SRC:.java=.class)
hello: $(OBJ)

]]
