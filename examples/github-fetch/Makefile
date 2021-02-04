MyGKrellm: CC = g++

MyGKrellm_CPPFLAGS := -MD -MP
MyGKrellm_CPPFLAGS += -I./include
MyGKrellm_CPPFLAGS += -I./lib/libjzon/include
MyGKrellm_LDLIBS := -lsfml-graphics
MyGKrellm_LDLIBS += -lsfml-window
MyGKrellm_LDLIBS += -lsfml-system
MyGKrellm_LDLIBS += -ljzon
MyGKrellm_LDFLAGS := -L./lib
MyGKrellm_SRCS := $(shell find -path './src/*.cpp')
MyGKrellm_CXXFLAGS := -Wall
MyGKrellm_CXXFLAGS += -Wextra
MyGKrellm_OBJS := $(filter %.cpp,$(MyGKrellm_SRCS))
MyGKrellm_OBJS := $(MyGKrellm_OBJS:.cpp=.o)
MyGKrellm_DEPS := $(MyGKrellm_OBJS:.o=.d)
MyGKrellm $(MyGKrellm_OBJS): ./lib/libjzon.a
MyGKrellm: $(MyGKrellm_OBJS)
$(MyGKrellm_OBJS): %.o: %.cpp
-include $(MyGKrellm_DEPS)
clean:
	$(RM) $(MyGKrellm_DEPS) $(MyGKrellm_OBJS)
.PHONY: clean
