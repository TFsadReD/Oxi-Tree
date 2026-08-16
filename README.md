<div align="center">
<h1>Утилита oxi-tree 🌳</h1>
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
</div>

**oxi-tree** — это быстрая и лёгкая консольная утилита для визуализации структуры директорий и файлов в виде дерева, написанная на Rust.

---

## 🚀 Особенности

* ⚡ **Высокая скорость:** быстрый обход файловой системы благодаря оптимизациям Rust
* 🔍 **Контроль глубины:** поддержка ограничения уровня вложенности папок
* 📦 **Простая установка на Windows:** готовый `.msi` инсталлятор, который автоматически добавляет утилиту в системный `PATH`
* 🛡️ **Надёжность:** минимальное количество зависимостей и безопасная работа с памятью

---

## 📸 Пример работы

```bash
.
├── 📁 src/
│   ├── 📄 main.rs
│   ├── 📄 tree.rs
├── 📄 .gitignore
├── 📄 Cargo.lock
├── 📄 Cargo.toml
├── 📄 LICENSE
├── 📄 README.md
└── 1 📁  |  7 📄
```

---

## 📥 Установка

### Windows (через `.msi` инсталлятор)
1. Скачайте последнюю версию `oxi-tree-x86_64.msi` со страницы [Releases](https://github.com/TFsadReD/Oxi-Tree/releases)
2. Запустите инсталлятор и пройдите шаги установки
3. Откройте **новое** окно PowerShell или CMD и введите команду:
    ```cmd
    oxi-tree
    ```

### Из исходников (через Cargo)

Если у вас установлен Rust:

```bash
git clone https://github.com/TFsadReD/Oxi-Tree.git
cd oxi-tree
cargo install --path .
```

---

## 💻 Использование

- Запуск обхода в текущей папке:

    ```bash
    oxi-tree
    ```

- Запуск для указанного пути:

    ```bash
    oxi-tree C:\Users\Username
    ```

---

## 📜 Лицензия

Проект распространяется под лицензией **Apache-2.0**