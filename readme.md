# Minecraft Yggdrasil API in Rust

This project is a Rust implementation of the Minecraft Yggdrasil API.

Implements [authlib-injector](https://github.com/yushijinhun/authlib-injector)

It provides endpoints for user authentication, session management, and profile handling.

## Features

- User authentication
- Session management
- Profile handling
- RSA key generation and signing
- Configurable via config files

## Getting Started

### Prerequisites

- Rust and Cargo installed
- PostgreSQL installed (You have to change the code to use another database)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/zrll12/yggdrasil-apirs.git
    cd yggdrasil-apirs
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```
   Then binary files may be found in the `target/release` directory.

### Running the Application

1. Start by running the binary file. This will generate all the necessary files
2. Edit configs under `config` directory. Check the wiki for more information
3. Add a authenticate source to you launcher. Endpoints are the root of the server
4. Add [authlib-injector](https://github.com/yushijinhun/authlib-injector) or [MultiLogin](https://github.com/CaaMoe/MultiLogin) to your server. (Config for MultiLogin is here: [yggdrasil.yml](https://github.com/zrll12/yggdrasil-apirs/blob/master/yggdrasil.yml))