# Decentralized Netflix Workshop

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/scrtlabs/dNetflix)

## Prepare for the workshop
To come more prepared to the workshop, we encourage you to set up you environment beforehand:
* Installations:
    * Install Rust (https://www.rust-lang.org/tools/install)
    * Build tools:
        ```bash
        # Linux 
        sudo apt update && sudo apt install build-essential

        # Mac
        xcode-select --install

        # Windows
        # Install WSL (https://learn.microsoft.com/en-us/windows/wsl/install) and then do the Linux part above ^
        ```
    * Make sure to have an IDE that supports Rust
        * VSCode (https://code.visualstudio.com/download) + rust-analyzer extension (https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) or
        * CLion (https://www.jetbrains.com/help/clion/installation-guide.html#requirements) or
        * Other IDE of your choice
* Get the latest LocalSecret image (*Apple silicon [M1/M2] users will have to work on a remote environment [Gitpod], so you can skip this part*):
    ```bash
    docker pull ghcr.io/scrtlabs/localsecret:v1.4.0-beta.11
    ```
* Create a free account on an IPFS provider:
    * Pinata (https://www.pinata.cloud/) or
    * Infura (https://infura.io/product/ipfs) or
    * Other IPFS provider of your choice