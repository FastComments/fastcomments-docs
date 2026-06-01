Wybierz jedną z dwóch standardowych metod dodania komponentu motywu Hugo.

### Opcja A: Moduł Hugo (zalecane)

Z katalogu głównego swojej witryny:

```bash
hugo mod init github.com/you/your-site   # tylko jeśli twoja witryna nie jest jeszcze modułem
hugo mod get github.com/FastComments/fastcomments-hugo
```

Następnie dodaj import do pliku `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opcja B: Komponent motywu (podmoduł Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Następnie odwołaj się do niego w `hugo.toml`. Umieść go obok własnego motywu; późniejsze wpisy mają pierwszeństwo, więc umieść swój motyw jako pierwszy:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```