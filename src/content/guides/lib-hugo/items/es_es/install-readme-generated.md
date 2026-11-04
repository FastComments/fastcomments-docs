Elige una de las dos formas estándar de añadir un componente de tema de Hugo.

### Opción A: Módulo de Hugo (recomendado)

Desde la raíz de tu sitio:

```bash
hugo mod init github.com/you/your-site   # solo si tu sitio aún no es un módulo
hugo mod get github.com/FastComments/fastcomments-hugo
```

Luego añade la importación a tu `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opción B: Componente de tema (submódulo de Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Luego referéncialo desde tu `hugo.toml`. Colócalo junto a tu propio tema; las entradas posteriores tienen prioridad, así que mantén tu tema primero:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```