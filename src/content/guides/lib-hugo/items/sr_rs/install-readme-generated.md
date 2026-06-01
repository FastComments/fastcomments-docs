Изаберите један од два стандардна начина да додате компоненту теме за Hugo.

### Опција A: Hugo модул (препоручено)

Из корена вашег сајта:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Затим додајте import у ваш `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Опција B: Компонента теме (Git подмодул)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Затим референцирајте то из вашег `hugo.toml`. Наведите га поред ваше теме; каснији уноси имају предност, па ставите вашу тему прву:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```