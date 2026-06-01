Todas as opções do widget FastComments são definidas em `[params.fastcomments]` em `hugo.toml`, e podem ser substituídas por página no front matter em `[fastcomments]`. Precedência, do menor para o maior: parâmetros do site, front matter da página, parâmetros do shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### Residência de dados da UE

Clientes da UE definem `region = "eu"` para direcionar o widget para `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Observação sobre o padrão de maiúsculas/minúsculas das chaves

Hugo converte todas as chaves para letras minúsculas em `hugo.toml` e no front matter, mas os widgets do FastComments exigem chaves em camelCase (`tenantId`, `hasDarkBackground`). Este componente restaura automaticamente a capitalização correta para toda opção de nível superior conhecida, portanto escreva as opções em seu formato camelCase normal. Chaves aninhadas dentro de um valor de objeto (por exemplo, as chaves de um mapa `translations`, ou campos de `pageReactConfig`) não são restauradas. Configure essas através da interface de personalização (dashboard) do FastComments em vez disso.