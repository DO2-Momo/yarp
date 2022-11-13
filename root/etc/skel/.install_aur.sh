install_icons() {
  echo "Installing we10x icons";
  
  git clone https://aur.archlinux.org/we10x-icon-theme-git.git;
  cd we10x-icon-theme-git;
  echo "$1" | makepkg -si && cd .. ; rm -rf we10x-icon-theme-git.git;
}

install_yay() {
  echo "Installing AUR package manager yay";
 
  git clone https://aur.archlinux.org/yay.git;
  cd yay;
  makepkg -si && cd .. ; rm -rf yay;
}

install_yay && install_icons && rm -rf $HOME/.install_aur.sh