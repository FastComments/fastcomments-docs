Оберіть один із двох стандартних способів додати компонент теми Hugo.

### Варіант A: Hugo Module (рекомендовано)

У кореневому каталозі вашого сайту:

```bash
hugo mod init github.com/you/your-site   # лише якщо ваш сайт ще не є модулем
hugo mod get github.com/FastComments/fastcomments-hugo
```

Потім додайте імпорт до вашого `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Варіант B: Компонент теми (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Потім посилайтеся на нього у вашому `hugo.toml`. Перелічіть його поряд із вашою темою; пізніші записи мають пріоритет, тому тримайте вашу тему першою:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```