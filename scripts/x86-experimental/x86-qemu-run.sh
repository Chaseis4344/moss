
mkdir -p build/isofiles/boot/grub/
if RUSTFLAGS=-Aunused cargo build  --target=x86_64-unknown-none; then

  cp target/x86_64-unknown-none/debug/moss build/isofiles/boot/moss.bin
  cp src/arch/x86_64/boot/grub.cfg build/isofiles/boot/grub/grub.cfg
  if grub-file --is-x86-multiboot2 build/isofiles/boot/moss.bin; then
    echo "building ISO"
    cd build/
    grub-mkrescue -o moss.iso isofiles
    echo "Running QEMU"
    qemu-system-x86_64 -s -S -cdrom moss.iso &
    cd ../
  else
    echo "not mulitboot2 compliant, aborting build"
  fi 
else
  echo "Bulid failed exiting"
fi



