Vælg en af de to standardmetoder til at tilføje en Hugo-temakomponent.

### Valgmulighed A: Hugo-modul (anbefalet)

Fra dit sites rod:

```bash
hugo mod init github.com/you/your-site   # kun hvis dit site ikke allerede er et modul
hugo mod get github.com/FastComments/fastcomments-hugo
```

Tilføj derefter importen til din `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Valgmulighed B: Temakomponent (Git-submodul)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Referér derefter til det fra din `hugo.toml`. Angiv det sammen med dit eget tema; senere elementer har forrang, så hold dit tema først:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```