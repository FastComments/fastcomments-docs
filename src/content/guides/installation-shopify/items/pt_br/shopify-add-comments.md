O bloco **FastComments** é o widget principal de comentários. Adicione-o a templates de posts de blog, templates de produtos ou qualquer outra página onde você queira um tópico de discussão ou um chat ao vivo.

### Adicionar o bloco

1. Abra o editor de temas da Shopify (**Online Store > Themes > Customize**).
2. Escolha o template onde você quer comentários: **Blog post**, **Product** ou qualquer outro template de página ou seção.
3. Na seção onde você quer que os comentários apareçam, clique em **Add block**.
4. Em **Apps**, selecione **FastComments**.
5. Clique em **Save**.

O bloco aparece imediatamente. Não há Tenant ID para inserir; o tenant da sua loja é configurado automaticamente quando você instala o app.

### Configurações

| Configuração | O que faz | Padrão |
|---|---|---|
| Tenant ID (opcional) | Substitui contra qual tenant do FastComments o bloco será renderizado. Deixe em branco para usar o tenant configurado automaticamente da loja. Encontre um tenant ID manual em fastcomments.com/auth/my-account/api-secret. | (em branco) |
| SSO | Faz login automático do visitante com a conta de cliente Shopify dele antes de comentar. Veja [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** para respostas aninhadas e votos, ou **Streaming** para um feed de chat em tempo real. | Threaded |
| Custom URL ID | Substitui o identificador de página detectado automaticamente. Use isso quando quiser que duas URLs compartilhem o mesmo tópico de comentários. | (detectado automaticamente) |

### Como o identificador de página é escolhido

Cada tópico de comentários é identificado por um URL ID. O bloco escolhe um automaticamente:

- **Template de post de blog:** `shopify-article-{article.id}`, que é estável mesmo com mudanças de slug e título.
- **Template de produto:** `shopify-product-{product.id}`, que é estável mesmo com mudanças de slug e título.
- **Outros templates:** o caminho da requisição.

Se você definir **Custom URL ID**, esse valor será usado em vez disso. Use o mesmo Custom URL ID em vários blocos (por exemplo, em uma variante localizada de uma página de produto) para compartilhar um único tópico de comentários.

### Threaded vs Streaming

**Threaded** é o padrão. Os visitantes respondem uns aos outros, votam, e as ferramentas de moderação funcionam como esperado. Ideal para posts de blog e avaliações de produtos.

**Streaming** elimina o encadeamento e mostra novos comentários em tempo real conforme são publicados, como um feed de chat. Ideal para lançamentos de produtos, eventos ao vivo e páginas de comunidade.

### Vários blocos na mesma página

O bloco pode ser adicionado mais de uma vez ao mesmo template. Por exemplo, um Resumo de Avaliações no topo de uma página de produto e um bloco FastComments na parte inferior. Os blocos compartilham um URL ID, então o resumo reflete os comentários abaixo.

### Dicas

- O bloco se oculta na pré-visualização do editor de temas com um aviso amarelo se não conseguir encontrar um tenant. Se isso aparecer na sua loja ao vivo, reinstale o app FastComments.
- Para uma página de produto, o bloco FastComments também funciona como seu widget de avaliações de produto. Combine-o com **FastComments - Reviews Summary** para um resumo com classificação por estrelas no topo da página.