Odaberite jedan od dva standardna načina za dodavanje komponente Hugo teme.

### Opcija A: Hugo modul (preporučeno)

Iz korijena vašeg sajta:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Zatim dodajte uvoz u svoj `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opcija B: Komponenta teme (Git podmodul)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Zatim se pozovite na nju iz svog `hugo.toml`. Navedite je zajedno s vašom temom; kasniji unosi imaju prednost, stoga stavite svoju temu prvu:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```