Изберете един от двата стандартни начина за добавяне на компонент за тема на Hugo.

### Опция A: Hugo модул (препоръчително)

От корена на вашия сайт:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

След това добавете импорта във вашия `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Опция B: Компонент на тема (Git подмодул)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

След това го посочете във вашия `hugo.toml`. Избройте го заедно с вашата тема; по-късните записи имат предимство, затова поставете вашата тема първа:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```