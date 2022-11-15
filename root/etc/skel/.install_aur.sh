install_icons() {
  echo "--- Installing we10x icons ---";
  yay -S we10x-icon-theme-git --noconfirm;
}

install_pamac() {
  echo "--- Installing pamac aur & pacman wrapper ---";
  yay -S pamac-aur --noconfirm;
}

install_yay() {
  echo "--- Installing AUR package manager yay ---";
 
  git clone https://aur.archlinux.org/yay.git;
  cd yay;
  makepkg -si --noconfirm && cd .. && rm -rf yay &&

  install_pamac
  install_icons
}

install_yay; 

#rm -rf $HOME/.install_aur.sh