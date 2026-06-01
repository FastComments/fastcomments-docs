Kies een van de twee standaardmanieren om een Hugo-themacomponent toe te voegen.

### Optie A: Hugo-module (aanbevolen)

Vanaf de root van je site:

```bash
hugo mod init github.com/you/your-site   # alleen als je site nog geen module is
hugo mod get github.com/FastComments/fastcomments-hugo
```

Voeg vervolgens de import toe aan je `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Optie B: Thema-component (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Verwijs er vervolgens naar in je `hugo.toml`. Vermeld het naast je eigen thema; latere vermeldingen winnen, dus houd je thema als eerste:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```