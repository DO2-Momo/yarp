echo -e "$2\\n$2" | passwd 
useradd -m -G wheel -s /bin/bash $1
echo -e "$2\\n$2" | passwd $1 

ls /boot/efi

grub-install \
  --target=x86_64-efi \
  --efi-directory=/boot/efi \
  --bootloader-id=GRUB \
  --removable \
  --recheck

grub-mkconfig -o /boot/grub/grub.cfg

systemctl enable NetworkManager.service
systemctl enable lightdm.service

git clone https://aur.archlinux.org/yay.git
cd yay;

makepkg -si

cd ..

yay -S dracula-gtk-theme

rm -f /install.sh