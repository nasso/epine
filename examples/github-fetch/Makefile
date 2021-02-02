##
## EPITECH PROJECT, 2021
## libmy
## File description:
## Makefile automatically generated using Epine!
##

NAME = libmy.a

all: libmy.a hello
.PHONY: all

EPINE_CC___NAME__SRCS := $(shell find -path './src/*.c')
EPINE_CC___NAME__CFLAGS := -Wall -Wextra -pedantic $(if DEBUG,-g3)
EPINE_CC___NAME__LDLIBS :=
EPINE_CC___NAME__LDFLAGS :=
EPINE_CC___NAME__OBJS := $(EPINE_CC___NAME__SRCS:.c=.o)
EPINE_CC___NAME__CFLAGS += -Iinclude
EPINE_CC___NAME__LDFLAGS += -L. -L./lib
$(NAME): $(EPINE_CC___NAME__OBJS)
	$(AR) rc $@ $(EPINE_CC___NAME__OBJS)
$(EPINE_CC___NAME__OBJS): %.o: %.c
	$(CC) $(CFLAGS) $(EPINE_CC___NAME__CFLAGS) -c -o $@ $<

EPINE_CC_hello_SRCS := main.c
EPINE_CC_hello_CFLAGS := -Wall -Wextra -pedantic $(if DEBUG,-g3)
EPINE_CC_hello_LDLIBS :=
EPINE_CC_hello_LDFLAGS :=
EPINE_CC_hello_OBJS := $(EPINE_CC_hello_SRCS:.c=.o)
EPINE_CC_hello_CFLAGS += -Iinclude
EPINE_CC_hello_LDLIBS += -lmy
EPINE_CC_hello_LDFLAGS += -L. -L./lib
hello $(EPINE_CC_hello_OBJS): libmy.a
hello: $(EPINE_CC_hello_OBJS)
	$(CC) -o $@ $(EPINE_CC_hello_OBJS) $(EPINE_CC_hello_LDLIBS) $(EPINE_CC_hello_LDFLAGS)
$(EPINE_CC_hello_OBJS): %.o: %.c
	$(CC) $(CFLAGS) $(EPINE_CC_hello_CFLAGS) -c -o $@ $<

EPINE_CC_unit_tests_SRCS := $(shell find -path './tests/*.c')
EPINE_CC_unit_tests_CFLAGS := -Wall -Wextra -pedantic $(if DEBUG,-g3)
EPINE_CC_unit_tests_LDLIBS :=
EPINE_CC_unit_tests_LDFLAGS :=
EPINE_CC_unit_tests_OBJS := $(EPINE_CC_unit_tests_SRCS:.c=.o)
EPINE_CC_unit_tests_CFLAGS += -Iinclude
EPINE_CC_unit_tests_LDLIBS += -l:$(NAME) -lcriterion
EPINE_CC_unit_tests_LDFLAGS += -L.
unit_tests $(EPINE_CC_unit_tests_OBJS): $(NAME)
unit_tests: $(EPINE_CC_unit_tests_OBJS)
	$(CC) -o $@ $(EPINE_CC_unit_tests_OBJS) $(EPINE_CC_unit_tests_LDLIBS) $(EPINE_CC_unit_tests_LDFLAGS)
$(EPINE_CC_unit_tests_OBJS): %.o: %.c
	$(CC) $(CFLAGS) $(EPINE_CC_unit_tests_CFLAGS) -c -o $@ $<

tests_run: unit_tests
	./unit_tests $(ARGS)
.PHONY: tests_run

clean:
	$(RM) $(EPINE_CC___NAME__OBJS) $(EPINE_CC_hello_OBJS) $(EPINE_CC_unit_tests_OBJS)
.PHONY: clean

fclean:
	$(RM) $(EPINE_CC___NAME__OBJS) $(EPINE_CC_hello_OBJS) $(EPINE_CC_unit_tests_OBJS)
	$(RM) $(NAME) hello
	$(RM) unit_tests
.PHONY: fclean

re: fclean all
.PHONY: re
