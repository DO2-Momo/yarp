# This script installs the grub booloader and enables 

#
# Users & Passwords
#

# Set Root password
echo -e "$2\\n$2" | passwd 

# Add User & Set Password (same as root)
useradd -m -G wheel -s /bin/bash $1
echo -e "$2\\n$2" | passwd $1 


#
# Install & configure bootloader
#
grub-install \
  --target=x86_64-efi \
  --efi-directory=/boot/efi \
  --bootloader-id=GRUB \
  --removable \
  --recheck

grub-mkconfig -o /boot/grub/grub.cfg


#
# Enable services here
#

# Enable Network manager service
systemctl enable NetworkManager.service


# Set Machine's Hostname 
echo $3 > /etc/hostname

# Install AUR 
# echo "$2" | su - $1 -c "source /home/$1/.install_aur.sh $2"

# Remove self
rm -f /install