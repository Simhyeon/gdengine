LIB		:= ../libs
SRC		:= ./
SRCS    := $(wildcard $(SRC)/*.r4d)
LIBS    := $(patsubst $(SRC)/%.r4d,$(LIB)/%.r4f,$(SRCS))

all : $(LIBS)

$(LIB)/%.r4f: $(SRC)/%.r4d
	@rad $< -f $@ --discard -A
