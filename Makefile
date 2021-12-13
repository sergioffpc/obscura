ERLANG_PATH ?= $(shell erl -eval 'io:format("~s", [lists:concat([code:root_dir(), "/erts-", erlang:system_info(version), "/include"])])' -s init stop -noshell)
CFLAGS += -Wall -Wextra -Wno-unused-parameter -I$(ERLANG_PATH)
LDFLAGS += -Wl,--no-as-needed -lX11 -lXext

.PHONY: all clean

all: priv/x11.so

priv/x11.so: src/x11.c
	$(CC) $(CFLAGS) -fPIC -shared $(LDFLAGS) -o $@ src/x11.c

clean:
	$(RM) priv/x11.so