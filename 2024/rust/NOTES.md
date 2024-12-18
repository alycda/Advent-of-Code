TheNuProjectContributors.vscode-nushell-lang

docker run -it -v ${PWD}:/home/aoc --rm ghcr.io/nushell/nushell 

cargo install nu --locked

chsh -s $(which nu)

`$ nu`

curl -sS https://starship.rs/install.sh | sh
mkdir ~/.cache/starship
starship init nu | save -f ~/.cache/starship/init.nu