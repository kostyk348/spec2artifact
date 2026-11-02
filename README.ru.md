<div align="center">

# spec2artifact

### LLM пишет спеку. Алгебра пишет артефакт.

**Детерминированный рендерер на Rust: ~90 токенов спеки превращаются в полный
структурированный артефакт — данные, отчёты, таблицы, заготовки кода — за 0 LLM-токенов.**

[![Release](https://img.shields.io/github/v/release/kostyk348/spec2artifact?sort=semver&color=0ea5e9&label=release)](https://github.com/kostyk348/spec2artifact/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/kostyk348/spec2artifact/ci.yml?branch=master&label=CI)](https://github.com/kostyk348/spec2artifact/actions/workflows/ci.yml)
[![tests](https://img.shields.io/badge/tests-24%20passing-22c55e)](#разработка)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](CONTRIBUTING.md)

[English](README.md) · **Русский**

</div>

---

## Что это такое?

Современная практика часто просит LLM выдать весь артефакт целиком. Для *структурированных*
артефактов (данные, конфиги, документы фиксированной формы, заготовки кода) это пустая трата
токенов: модель тратит генерацию на то, что программа посчитает бесплатно.

`spec2artifact` разделяет работу:

| роль | кто | цена |
|---|---|---|
| **что произвести** (спека) | LLM | мало, ~90 токенов, **не зависит от размера артефакта** |
| **как отрендерить** (тело) | детерминированный рендерер | микросекунды, **0 LLM-токенов** |

**Ничего принципиально нового.** «Разделение контента и представления» (шаблоны, DSL, кодогенерация),
data-to-text/NLG, генерация с JSON-schema и grammar-constrained decoding уже существуют. Этот
инструмент упаковывает *измеренный* выигрыш плюс **вариантные спеки**: форма спеки меняется
вместе с типом задачи, а рендерер точен по построению.

## Вариантные спеки

| kind | что заполняет LLM | что производит рендерер |
|---|---|---|
| `records` | n, seed, категории + пулы имён, диапазоны | n записей (jsonl / csv / json) |
| `report` | заголовок, секции, буллеты | Markdown |
| `table` | колонки, строки или пулы | Markdown-таблица |
| `code` | язык, модуль, импорты, сигнатуры+доки | заготовка кода |
| `list` | заголовок, пункты, отмеченные | список с галочками |
| `dataset` | n, алфавит, длина, правило | синтетический бенчмарк с ИЗВЕСТной меткой |
| `prose` | тема + тезисы абзацев, тон | читаемый template-NLG текст |

## Измерено (живой замер, один сабагент = один вызов LLM)

Генерация каталога из 60 записей:

| путь | время | выходные токены |
|---|---|---|
| LLM пишет весь каталог | 18.0 с | ~1054 |
| LLM пишет спеку, инструмент рендерит | 8.1 с | ~90 + рендер |

**На 91% меньше токенов, в 2.2 раза быстрее** (оверхед вызова LLM доминирует на этом размере).
Поскольку спека постоянна при любом `n`, выигрыш растёт дальше. Скорость рендерера, замерено
на Rust-бинарнике (один вызов процесса, release-сборка, вместе со стартом процесса):

```
n=60     ~0.9 ms    ~960 токенов, которые LLM так и не написал
n=600    ~2.0 ms    ~9726
n=6000   ~13.7 ms   ~98788
n=60000  ~128 ms    ~988k
```

## Использование

```bash
spec2artifact kinds
spec2artifact template records     # скелет ВАРИАНТНОЙ спеки для LLM
spec2artifact validate spec.json
spec2artifact render spec.json -o out.jsonl
```

Как библиотека Rust:

```rust
use spec2artifact::{spec, render};

let mut s = spec::spec_template("records").unwrap();
s["n"] = 1000.into();
let artifact = render::render(&s).unwrap();
assert_eq!(artifact.lines().count(), 1000);
```

Как MCP-инструмент: `spec2artifact mcp` (stdio JSON-RPC; инструменты: `spec_kinds`,
`spec_template`, `spec_validate`, `spec_render`). Манифест инструментов заморожен —
`spec2artifact fingerprint` печатает его стабильный sha256, чтобы дрейф кэша промптов
агента был заметен:

```bash
$ spec2artifact fingerprint
tools=4 sha256=…
```

## Установка

```bash
cargo install --path .          # из чекаута
# либо готовый бинарник из Releases
```

Никаких зависимостей, кроме тулчейна Rust (1.75+); бинарник — один статический файл ~1 МБ.

## Детерминизм

Та же спека (включая `seed`) → байт-в-байт тот же артефакт, на любой машине, в любом релизе.
RNG — SplitMix64, сидируемый прямо из спеки; поток не зависит от платформы.
Важно: сиды **не** совместимы с убранной Python-реализацией (Mersenne Twister) —
та же спека, другой (но столь же детерминированный) поток.

## Разработка

```bash
cargo fmt --check                          # форматирование
cargo clippy --all-targets -- -D warnings  # линты, нулевая толерантность
cargo test                                 # 24 теста: рендереры, валидация, CLI, MCP over stdio
cargo build --release                      # бинарник
```

Чек-лист для PR — в [CONTRIBUTING.md](CONTRIBUTING.md), история релизов — в
[CHANGELOG.md](CHANGELOG.md).

## Оговорки

* Результат ровно настолько богат, насколько богата спека; открытая изобретательность живёт
  в (маленькой) спеке.
* Лучше всего подходит для **структурированных** артефактов. Для свободной прозы спека должна
  быть богатой, а это съедает токены.
* Детерминизм: та же спека (включая `seed`) → тот же артефакт.
