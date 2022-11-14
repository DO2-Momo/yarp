install_icons() {
  echo "Installing we10x icons";
  
  git clone https://aur.archlinux.org/we10x-icon-theme-git.git;
  cd we10x-icon-theme-git;
  makepkg -si && cd .. ; rm -rf we10x-icon-theme-git.git;
}

install_pamac() {
  echo "Installing pamac aur & pacman wrapper";
  
  git clone https://aur.archlinux.org/pamac-aur.git;
  cd pamac-aur;
  makepkg -si && cd .. ; rm -rf pamac-aur;
}

install_yay() {
  echo "Installing AUR package manager yay";
 
  git clone https://aur.archlinux.org/yay.git;
  cd yay;
  makepkg -si && cd .. ; rm -rf yay;
}

install_yay && install_pamac && install_icons 

#rm -rf $HOME/.install_aur.sh