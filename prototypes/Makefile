BUILD_DIR	?= ./build

# default-target = "hello"
NAME		:= hello

# standard targets
.PHONY: all
all: $(NAME)

.PHONY: clean
clean:

.PHONY: fclean
fclean: clean

.PHONY: re
re: fclean all

$(NAME): hello

# # target hello (binary)
# #   name = "hello"
# #   src = ["./src/**/*.c"]
# #   include = ["./include"]
# hello_SRC := $(wildcard ./src/**/*.c)
# hello_OBJ := $(hello_SRC:%=$(BUILD_DIR)/%.o)
# hello_DEPS := $(hello_DEPS:.o=.d)
# hello_INC := ./include
#
# SRC		:= $(wildcard ./src/**/*.c)
# INCLUDE	:= $(wildcard ./include)
#
# hello: $(wildcard ./src/**/*.c)
