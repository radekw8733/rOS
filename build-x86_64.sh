#!/bin/sh

mkdir -p build/kernel
mkdir -p image/EFI/BOOT
if [ "$1" = "--release" ]; then
    cargo build --target x86_64-unknown-none --features="x86_64 limine" --release
    cp -v target/x86_64-unknown-none/release/rOS build/kernel/kernel.elf
else
    cargo build --target x86_64-unknown-none --features="x86_64 limine"
    cp -v target/x86_64-unknown-none/debug/rOS build/kernel/kernel.elf
fi

if [ ! -d "build/limine" ]; then
    git clone https://github.com/limine-bootloader/limine.git build/limine --branch=v7.x-binary --depth=1
    cc build/limine/limine.c -o build/limine/limine
fi
cp -v build/kernel/kernel.elf limine.cfg build/limine/limine-bios.sys build/limine/limine-bios-cd.bin build/limine/limine-uefi-cd.bin image/
cp -v build/limine/BOOTX64.EFI image/EFI/BOOT/
xorriso -as mkisofs -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label image -o rOS.iso
./build/limine/limine bios-install rOS.iso