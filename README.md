<div align="center">
    <h1>Oxi-Tree 🌳</h1>
<img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=for-the-badge" alt="License" />
</div>

<p align="center">
  <b>Language / Язык:</b>
  <a href="#english">English</a> • <a href="#russian">Русский</a>
</p>

---

<a id="english"></a>
## English README.md

**Oxi-Tree** is a fast and lightweight command-line tool written in Rust for visualizing directory and file structures as a tree.

> 🦀 **Rust Learning Project:** This is my first hands-on experience diving into Rust! I built Oxi-Tree to practice working with the filesystem, gain overall development experience, and create `.msi` installers for Windows. If you find this tool useful or want to support my Rust learning journey, giving this repository a ⭐️ **Star** would mean a lot!

---

## 🚀 Features

* ⚡ **High performance:** fast directory traversal powered by Rust optimizations
* 📦 **Easy Windows installation:** ready-to-use `.msi` installer that automatically adds the utility to your system `PATH`
* 🛡️ **Reliable:** minimal dependencies and memory-safe architecture

---

## 📸 Usage Example

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

## 📥 Installation

### Windows (via `.msi` installer)

1. Download the latest `oxi-tree-x86_64.msi` from the [Releases](https://github.com/TFsadReD/Oxi-Tree/releases) page.
2. Run the installer and complete the setup steps.
3. Open a **new** PowerShell or CMD window and run:
    ```cmd
    oxi-tree
    ```


### From Source (via Cargo)

If you have Rust installed:

```bash
git clone https://github.com/TFsadReD/Oxi-Tree.git
cd oxi-tree
cargo install --path .
```

---

## 💻 Usage

* Run in the current directory:
    ```bash
    oxi-tree
    ```


* Run for a specific path:
    ```bash
    oxi-tree C:\Users\Username
    ```

---

## 📜 License

Distributed under the **Apache-2.0** License.


---

<a id="russian"></a>
## Русский README.md


**Oxi-Tree** — это быстрая и лёгкая консольная утилита для визуализации структуры директорий и файлов в виде дерева, написанная на Rust.

> 🦀 **Пет-проект для изучения Rust:** Это мой первый практический опыт погружения в Rust! Я написал Oxi-Tree, чтобы на практике разобраться с файловой системой, набраться опыта в разработке и сборкой `.msi` инсталляторов под Windows. Если инструмент показался вам полезным или вы хотите поддержать мой путь в освоении Rust — буду очень благодарен за ⭐️ **звёздочку репозиторию**!

---

## 🚀 Особенности

* ⚡ **Высокая скорость:** быстрый обход файловой системы благодаря оптимизациям Rust
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
