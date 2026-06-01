Izaberite jedan od dva standardna načina za dodavanje komponente Hugo teme.

### Opcija A: Hugo modul (preporučeno)

Iz korena vašeg sajta:

```bash
hugo mod init github.com/you/your-site   # samo ako vaš sajt već nije modul
hugo mod get github.com/FastComments/fastcomments-hugo
```

Zatim dodajte import u vaš `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opcija B: Komponenta teme (Git podmodul)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Zatim ga referencirajte u svom `hugo.toml`. Navedite ga pored svoje teme; kasniji unosi imaju prednost, zato stavite svoju temu prvu:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```