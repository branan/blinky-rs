all:: $(OUTDIR)/libteensyrs.a

$(OUTDIR)/libteensyrs.a: $(OUTDIR)/sysinit.o $(OUTDIR)/crt0.o
	arm-none-eabi-ar rvs $@ $(OUTDIR)/sysinit.o $(OUTDIR)/crt0.o

$(OUTDIR)/sysinit.o: sysinit.c
	arm-none-eabi-gcc -Wall -fno-common -ffreestanding -mcpu=cortex-m4 -mthumb -O2 -I. -c $< -o $@

$(OUTDIR)/crt0.o: crt0.s
	arm-none-eabi-as -mcpu=cortex-m4 -o $@ $<
