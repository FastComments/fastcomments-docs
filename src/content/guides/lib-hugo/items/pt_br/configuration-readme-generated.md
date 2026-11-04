Todas as opções do widget FastComments são definidas em `[params.fastcomments]` no `hugo.toml`, e podem ser sobrescritas por página no front matter em `[fastcomments]`. Precedência, da menor para a maior: parâmetros do site, front matter da página, parâmetros do shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter de uma página
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Quando nem `url` nem `urlId` são fornecidos, `url` assume como padrão o permalink da página, para que os tópicos de comentários permaneçam vinculados a uma URL estável.

### Residência de dados na UE

Clientes da UE definem `region = "eu"` para rotear o widget para `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Observação sobre o formato das chaves

O Hugo converte para minúsculas todas as chaves no `hugo.toml` e no front matter, mas os widgets do FastComments exigem chaves em camelCase (`tenantId`, `hasDarkBackground`). Este componente restaura automaticamente o formato correto para todas as opções de nível superior conhecidas, então escreva as opções em sua forma camelCase normal. Chaves aninhadas dentro de um valor de objeto (por exemplo, as chaves de um mapa `translations`, ou campos de `pageReactConfig`) não são restauradas. Configure essas opções pela UI de personalização do painel do FastComments.