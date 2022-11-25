FROM ubuntu:22.04
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt -y install build-essential
RUN apt-get -y install curl zsh git libssl-dev pkg-config
RUN yes | sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN chsh -s $(which zsh)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
