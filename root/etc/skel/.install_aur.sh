install_yay () {
  echo "--- Installing AUR package manager yay ---";
 
  git clone https://aur.archlinux.org/yay.git;
  cd yay;
  makepkg -si --noconfirm && cd .. && rm -rf yay 

}

# uncomment to install yay
# install_yay;


install_pamac () {
  echo "--- Installing pamac aur & pacman wrapper ---";
  
  yay -S pamac-aur --noconfirm

}

# uncomment to install pamac
# install_pamac;


rm -rf ./install_aur.sh;