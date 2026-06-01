Escolha uma das duas maneiras padrão de adicionar um componente de tema do Hugo.

### Opção A: Módulo do Hugo (recomendado)

A partir do diretório raiz do seu site:

```bash
hugo mod init github.com/you/your-site   # apenas se o seu site ainda não for um módulo
hugo mod get github.com/FastComments/fastcomments-hugo
```

Em seguida, adicione a importação ao seu `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opção B: Componente de tema (submódulo Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Em seguida, referencie-o a partir do seu `hugo.toml`. Liste-o junto com o seu próprio tema; entradas posteriores têm prioridade, portanto mantenha seu tema primeiro:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```