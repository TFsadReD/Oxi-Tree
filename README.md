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
* ⚙️ **Flexible CLI configuration:** easily customize execution parameters with built-in command-line arguments

---

## 📸 Usage Example

```bash
> oxi-tree
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

## 🚩 List of Flags

| Short | Long | Parameter | Description |
| --- | --- | --- | --- |
| `-d` | `--depth` | `<number>` | Limit directory traversal depth (default: 2) |
| `-nd` | `--no-depth` | — | Remove crawl depth limits (unlimited traversal) |
| `-a` | `--all` | — | Include hidden files and folders (starting with `.`) |
| `-h` | `--help` | — | Display help information and exit |
| `-D` | `--dirs-only` | — | Display directories only (hide files) |
| `-v` | `--version` | — | Show current application version |
| `-e` | `--ext` | `<extension>` | Filter files by extension (e.g., -e rs or -e .rs) |
| `-U` | `--unsorted` | — | Disable natural sorting (traverse entries as-is for speed) |
| `-s` | `--size` | — | Display file sizes next to their names |

---

## 📥 Installation

### Via Cargo *(Recommended ⭐)*

**Oxi-Tree** is published on [crates.io](https://crates.io/crates/oxi-tree). If you have Rust and Cargo installed, you can install the tool with a single command:

```bash
cargo install oxi-tree
```

> ⚠️ **Note:** Installing via Cargo is the **strongly recommended** method. The Windows `.msi` installer may trigger false positive warnings from Windows Defender or antivirus software due to the lack of a paid digital code-signing certificate.

### Windows (via `.msi` installer)

1. Download the latest `oxi-tree-x86_64.msi` from the [Releases](https://www.google.com/search?q=https://github.com/TFsadReD/Oxi-Tree/releases) page.
2. Run the installer and complete the setup steps.
3. Open a **new** PowerShell or CMD window and run:
    ```cmd
    oxi-tree
    ```

---

## 🗺️ Roadmap

- [x] 📁 **`-D` / `--dirs-only`**: Display directories only (hide files)
- [x] 🏷️ **`-v` / `--version`**: Show current application version
- [x] 🎯 **`-e` / `--ext <ext>`**: Filter files by specific extension (e.g., `-e rs`)
- [x] ⚡ **`-U` / `--unsorted`**: Disable natural sorting for maximum traversal speed in huge directories
- [x] 📊 **`-s` / `--size`**: Display file sizes next to their names
- [ ] 🔀 **`-S` / `--sort <by>`**: Sort items by parameter (`name`, `size`, `date`, `ext`)
- [ ] 💾 **`-o` / `--output <file>`**: Save generated directory tree directly to a file
- [ ] ⚖️ **`-mn` / `--min-size <size>`**, **`-mx` / `--max-size <size>`**: Filter files by size boundaries (e.g., `--min-size 1MB`)

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
* ⚙️ **Гибкая настройка CLI:** гибкая настройка всех параметров выполнения прямо из командной строки

---

## 📸 Пример работы

```bash
> oxi-tree
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

## 🚩 Список Флагов

| Короткий | Полный | Параметр | Описание |
| --- | --- | --- | --- |
| `-d` | `--depth` | `<число>` | Ограничить глубину обхода директорий (по умолчанию: 2) |
| `-nd` | `--no-depth` | — | Снять ограничение глубины обхода |
| `-a` | `--all` | — | Отображать скрытые файлы и папки (начинающиеся с `.`) |
| `-h` | `--help` | — | Показать справочную информацию |
| `-D` | `--dirs-only` | — | Отображение только папок (без файлов) |
| `-v` | `--version` | — | Вывод текущей версии утилиты |
| `-e` | `--ext` | `<расширение>` | Фильтровать файлы по расширению (например, -e rs или -e .rs) |
| `-U` | `--unsorted` | — | Отключить естественную сортировку (для максимальной скорости) |
| `-s` | `--size` | — | Отображение размера файлов |

---

## 📥 Установка

### Через Cargo (Рекомендуемый способ ⭐)

Утилита опубликована на [crates.io](https://crates.io/crates/oxi-tree). Если у вас установлен Rust и менеджер пакетов Cargo, вы можете установить `oxi-tree` всего одной командой:

```bash
cargo install oxi-tree
```

> ⚠️ **Примечание:** Установка через Cargo — это **наиболее предпочтительный способ**. `.msi` инсталлятор для Windows может вызывать ложные срабатывания Windows Defender или других антивирусов из-за отсутствия платной цифровой подписи (Code Signing Certificate).

### Windows (через `.msi` инсталлятор)

1. Скачайте последнюю версию `oxi-tree-x86_64.msi` со страницы [Releases](https://www.google.com/search?q=https://github.com/TFsadReD/Oxi-Tree/releases)
2. Запустите инсталлятор и пройдите шаги установки
3. Откройте **новое** окно PowerShell или CMD и введите команду:
    ```cmd
    oxi-tree
    ```

---

## 🗺️ Планы на будущее (Roadmap)

- [x] 📁 **`-D` / `--dirs-only`**: Отображение только папок (без файлов)
- [x] 🏷️ **`-v` / `--version`**: Вывод текущей версии утилиты
- [x] 🎯 **`-e` / `--ext <расширение>`**: Фильтрация файлов по расширению (например, `-e rs`)
- [x] ⚡ **`-U` / `--unsorted`**: Отключение естественной сортировки для максимальной скорости обхода огромных директорий
- [x] 📊 **`-s` / `--size`**: Отображение размера файлов
- [ ] 🔀 **`-S` / `--sort <критерий>`**: Сортировка элементов по параметру (`name`, `size`, `date`, `ext`)
- [ ] 💾 **`-o` / `--output <файл>`**: Сохранение вывода дерева напрямую в файл
- [ ] ⚖️ **`-mn` / `--min-size <размер>`**, **`-mx` / `--max-size <размер>`**: Фильтрация файлов по размеру (например, `--min-size 1MB`)

---

## 📜 Лицензия

Проект распространяется под лицензией **Apache-2.0**
