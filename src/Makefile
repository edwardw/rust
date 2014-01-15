TOP := libstd
DEPS := $(wildcard $(TOP)/*.rs) $(wildcard $(TOP)/num/*.rs)
DEPS += $(wildcard $(TOP)/rt/*.rs) $(wildcard $(TOP)/fmt/*.rs)
DEPS += $(wildcard $(TOP)/unstable/*.rs) $(wildcard $(TOP)/io/*.rs)
RLIB_STD := libstd-64b22b00-0.0.rlib
SRC_TAR := bare-metal-src.tar

all: $(RLIB_STD)

$(RLIB_STD): $(DEPS)
	rustc --target x86_64-linux-gnu $(TOP)/bare-metal.rs --out-dir . -O -Z no-landing-pads --dep-info deps

clean:
	rm -f *.o
	rm -f *.bc
	rm -f $(RLIB_STD)
	rm -f deps
	rm -f $(SRC_TAR)

tar: $(RLIB_STD)
	tar -cvf $(SRC_TAR) $(patsubst $(RLIB_STD):%,%,$(shell cat deps))
