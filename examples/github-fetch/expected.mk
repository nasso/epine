CFLAGS = -g3

all: libmy.a
.PHONY: all

EPINE_CC_libmy_a_SRCS := ./src/my_putstr.c ./src/my_printf.c
EPINE_CC_libmy_a_CFLAGS := -Wall -Wextra -pedantic
EPINE_CC_libmy_a_LDLIBS :=
EPINE_CC_libmy_a_LDFLAGS :=
EPINE_CC_libmy_a_OBJS := $(EPINE_CC_libmy_a_SRCS:.c=.o)
EPINE_CC_libmy_a_CFLAGS += -Iinclude
EPINE_CC_libmy_a_CFLAGS += -DMY_ALLOW_MALLOC -DMY_ALLOW_FREE -DMY_FAKE_MALLOC_FAILURE=16
libmy.a: $(EPINE_CC_libmy_a_OBJS)
	$(AR) rc $@ $(EPINE_CC_libmy_a_OBJS)
$(EPINE_CC_libmy_a_OBJS): %.o: %.c
	$(CC) $(CFLAGS) $(EPINE_CC_libmy_a_CFLAGS) -c -o $@ $<

EPINE_CC_unit_tests_SRCS := tests/test.c
EPINE_CC_unit_tests_CFLAGS := -Wall -Wextra -pedantic
EPINE_CC_unit_tests_LDLIBS :=
EPINE_CC_unit_tests_LDFLAGS :=
EPINE_CC_unit_tests_OBJS := $(EPINE_CC_unit_tests_SRCS:.c=.o)
EPINE_CC_unit_tests_CFLAGS += -Iinclude
EPINE_CC_unit_tests_LDLIBS += -lmy -lcriterion
EPINE_CC_unit_tests_LDFLAGS += -L.
unit_tests $(EPINE_CC_unit_tests_OBJS): libmy.a
unit_tests: $(EPINE_CC_unit_tests_OBJS)
	$(CC) -o $@ $(EPINE_CC_unit_tests_OBJS) $(EPINE_CC_unit_tests_LDLIBS) $(EPINE_CC_unit_tests_LDFLAGS)
$(EPINE_CC_unit_tests_OBJS): %.o: %.c
	$(CC) $(CFLAGS) $(EPINE_CC_unit_tests_CFLAGS) -c -o $@ $<

tests_run: unit_tests
	./unit_tests
.PHONY: tests_run

clean:
	$(RM) $(EPINE_CC_libmy_a_OBJS) $(EPINE_CC_unit_tests_OBJS)
.PHONY: clean

fclean:
	$(RM) $(EPINE_CC_libmy_a_OBJS) $(EPINE_CC_unit_tests_OBJS)
	$(RM) libmy.a unit_tests
.PHONY: fclean

re: fclean all
.PHONY: re
