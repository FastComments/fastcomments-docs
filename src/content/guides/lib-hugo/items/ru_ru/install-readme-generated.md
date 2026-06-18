Выберите один из двух стандартных способов добавить компонент темы Hugo.

### Вариант A: Модуль Hugo (рекомендуется)

Из корня вашего сайта:

```bash
hugo mod init github.com/you/your-site   # только если ваш сайт ещё не является модулем
hugo mod get github.com/FastComments/fastcomments-hugo
```

Затем добавьте импорт в ваш `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Вариант B: Компонент темы (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Затем укажите его в вашем `hugo.toml`. Перечислите его вместе с вашей темой; побеждает более поздняя запись, поэтому ставьте вашу тему первой:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```