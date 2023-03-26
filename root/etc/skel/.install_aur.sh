install_yay () {
  echo "--- Installing AUR package manager yay ---";
 
  git clone https://aur.archlinux.org/yay.git;
  cd yay;
  makepkg -si --noconfirm && cd .. && rm -rf yay;
}

install_google_chrome () {
  echo "--- Installing google-chrome ---";
  
  yay -S google-chrome --noconfirm
}


install_pamac () {
  echo "--- Installing pamac aur & pacman wrapper ---";
  
  yay -S pamac-aur --noconfirm
}

install_yay;
install_google_chrome;
install_pamac;

exit;

rm -rf $HOME/install_aur.sh;