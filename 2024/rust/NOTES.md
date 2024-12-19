TheNuProjectContributors.vscode-nushell-lang

docker run -it -v ${PWD}:/home/aoc --rm ghcr.io/nushell/nushell 

cargo install nu --locked

chsh -s $(which nu)

`$ nu`

curl -sS https://starship.rs/install.sh | sh

$nu.env-path