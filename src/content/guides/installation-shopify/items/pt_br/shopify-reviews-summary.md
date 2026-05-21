The **FastComments - Reviews Summary** block mostra uma classificação agregada por estrelas e a distribuição das avaliações de uma página. Combine-o com o bloco **FastComments** em templates de produto para o layout padrão de avaliações: resumo no topo, formulário de avaliação e avaliações abaixo.

### Pré-requisito: configurar Ratings & Reviews

O bloco Reviews Summary exibe as perguntas de avaliação que você configurou para sua loja. Configure-as primeiro:

1. Abra o app FastComments no seu admin do Shopify.
2. Clique no bloco **Ratings & Reviews Helper** (ou abra [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) diretamente).
3. Adicione as perguntas que você quer que cada avaliador responda (classificação geral por estrelas, "como foi o caimento", etc.).

Sem perguntas configuradas, o bloco de resumo não tem nada para agregar.

### Adicionar o bloco

1. Abra o editor de tema do Shopify.
2. Abra o template **Product** (ou o template de página onde você quer o resumo).
3. Clique em **Add block** perto do topo da seção da página, acima de onde o bloco **FastComments** ficará.
4. Em **Apps**, selecione **FastComments - Reviews Summary**.
5. Adicione um bloco **FastComments** mais abaixo na mesma página se você ainda não tiver um, para que os visitantes possam deixar avaliações.
6. Clique em **Save**.

### Configurações

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Sobrescreve qual tenant do FastComments o resumo lê. Deixe em branco para usar o tenant configurado automaticamente pela loja. | (em branco) |
| Custom URL ID | Sobrescreve o identificador de página contra o qual o resumo agrega. Use isto quando o resumo estiver em uma página diferente do bloco FastComments que ele reflete. | (detectado automaticamente) |

### Como o resumo corresponde às avaliações

O bloco Reviews Summary usa a mesma lógica de detecção automática que o bloco **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Para uma página de produto normal, o resumo e o thread de comentários compartilham um URL ID automaticamente, sem necessidade de configuração.

### Dicas

- O resumo é somente leitura. Para coletar avaliações, você precisa de um bloco **FastComments** na mesma página.
- Se você alterar as perguntas de avaliação no Ratings & Reviews Helper após coletar avaliações, o resumo será recalculado com base no novo conjunto de perguntas.