Изаберите један од два стандардна начина да додате компоненту теме за Hugo.

### Опција A: Hugo модул (препоручено)

Из корена вашег сајта:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Затим додајте увоз у ваш `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Опција B: Компонента теме (Git подмодул)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Затим га наведите у вашем `hugo.toml`. Наведите га заједно са вашом темом; каснији уноси побјеђују, зато ставите вашу тему прву:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```