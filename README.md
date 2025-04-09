# Sibadi Web
Репозиторий содержит веб-приложение, разработанное на Rust, для университета
[**Сибади**](https://sibadi.org/)

## Приложение в магазинах
- [ ] App Store
- [ ] Play Market

## Сборка
Установка [**Rust**](https://www.rust-lang.org/)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Установка Dioxus CLI
```bash
cargo install dioxus-cli
```

Зависимости для сборки проекта на Linux | WSL
```bash
sudo apt update && \
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

Сборка проекта
```bash
dx build
```

Отладка проекта
```bash
dx serve
```

## Тесты
### Для компонентов
```bash
cargo test
```

### Для модулей
Установка [**wasm-pack**](https://github.com/rustwasm/wasm-pack)
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Запустить тесты можно с помощью
```bash
wasm-pack test --headless --firefox
```

Или для Chrome
```bash
wasm-pack test --headless --chrome
```

## Использованные библиотеки и фреймворки
- [**Dioxus**](https://dioxuslabs.com/)
- [**reqwest**](https://crates.io/crates/reqwest/)
- [**serde**](https://serde.rs/)
- [**serde_json**](https://crates.io/crates/serde_json)
- [**web-sys**](https://crates.io/crates/web-sys)

## Использованные библиотеки и фреймворки для тестов
- [**dioxus-ssr**](https://crates.io/crates/dioxus-ssr)
- [**pretty_assertions**](https://crates.io/crates/pretty_assertions)
- [**wasm-bindgen-test**](https://crates.io/crates/wasm-bindgen-test)
