#
# Install & configure bootloader
#

if [ "$3" == "false" ];
then
  grub-install \
    --target=x86_64-efi \
    --efi-directory=/boot/efi \
    --bootloader-id=DO2OS \
    --recheck \
    --removable
else 
  grub-install --target=i386-pc $4
fi

grub-mkconfig -o /boot/grub/grub.cfg


#
# Users & Passwords
#

# Set Root password
echo -e "$2\\n$2" | passwd 

# Add User & Set Password (same as root)
useradd -m -G wheel -s /bin/bash $1
echo -e "$2\\n$2" | passwd $1 


# Generate Locale
locale-gen

# Create desktop directories
mkdir /home/$1/Desktop;
mkdir /home/$1/Downloads;
mkdir /home/$1/Documents;
mkdir /home/$1/Pictures;
mkdir /home/$1/Videos;
mkdir /home/$1/Music;

chown -R $1 /home/$1

#
# Enable services here
#
systemctl enable NetworkManager.service
systemctl enable sddm.service

# Install AUR 
echo "$2" | su - $1 -c "source /home/$1/.install_aur.sh $2"


# Remove self
rm -f /install.sh
