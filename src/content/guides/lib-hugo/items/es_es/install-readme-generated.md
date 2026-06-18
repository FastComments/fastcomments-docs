Elija una de las dos formas estándar para añadir un componente de tema de Hugo.

### Opción A: Módulo de Hugo (recomendado)

Desde la raíz de su sitio:

```bash
hugo mod init github.com/you/your-site   # solo si su sitio aún no es un módulo
hugo mod get github.com/FastComments/fastcomments-hugo
```

Then add the import to your `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opción B: Componente de tema (submódulo Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Then reference it from your `hugo.toml`. List it alongside your own theme; later entries win, so keep your theme first:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```