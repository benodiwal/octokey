# 🐙 OctoKey

OctoKey is a tentacular tool to manage your GitHub SSH keys with ease and style! 🎉

## 🌟 Features

- 🔑 Add new SSH keys for GitHub
- 🔄 Switch between existing SSH keys
- 🔍 Check your current GitHub user
- 📋 List all available SSH keys
- 🐙 Octopus-approved UI (we promise it's not fishy)

## 🚀 Installation

1. Make sure you have Rust and Cargo installed. If not, get them from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
   ```
   git clone git@github.com:benodiwal/octokey.git
   cd octokey
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. The binary will be available at `target/release/octokey`. You can move it to a directory in your PATH for easy access:
   ```
   sudo mv target/release/octokey /usr/local/bin/
   ```

## 🎮 Usage

OctoKey is easy to use with its intuitive command-line interface:

### Add a new SSH key

```
octokey add my_new_key --email your_email@example.com
```

### Switch to a different SSH key

```
octokey switch existing_key_name
```

### Check your current GitHub user

```
octokey check
```

### List all available SSH keys

```
octokey list
```

## 📜 License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

Made with ❤️ by Sachin
