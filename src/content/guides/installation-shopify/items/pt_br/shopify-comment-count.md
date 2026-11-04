O bloco **FastComments - Comment Count** exibe uma pequena contagem de comentários para uma única página. Use-o em listas de posts de blog, cartões de produtos ou qualquer template que linka para uma página com comentários, para que os visitantes possam ver o quão ativo cada thread está antes de clicar.

### Adicionar o bloco

1. Abra o editor de temas da Shopify.
2. Abra o template onde deseja que a contagem apareça. Por exemplo, o template **Blog** (a lista de posts) ou uma seção de listagem de produtos.
3. Clique em **Add block** na seção que renderiza cada item.
4. Em **Apps**, selecione **FastComments - Comment Count**.
5. Clique em **Save**.

### Configurações

| Configuração | O que faz | Padrão |
|---|---|---|
| Tenant ID (opcional) | Sobrescreve de qual tenant do FastComments a contagem lê. Deixe em branco para usar o tenant configurado automaticamente da loja. | (em branco) |
| Custom URL ID | Sobrescreve o identificador de página que a contagem busca. Use isso quando a contagem estiver em uma página diferente do bloco FastComments que ela rastreia. | (detectado automaticamente) |

### Como a contagem corresponde ao thread de comentários

O bloco Comment Count usa a mesma lógica de detecção automática do bloco **FastComments**:

- Template de post de blog: `shopify-article-{article.id}`
- Template de produto: `shopify-product-{product.id}`
- Outros templates: o caminho da requisição

Se você definir um **Custom URL ID** no bloco **FastComments** em uma página, defina o mesmo Custom URL ID no bloco Comment Count para que apontem para o mesmo thread.

### Dicas

- As contagens de todos os itens da página são buscadas em uma única requisição, então adicionar o bloco a cada item em uma lista longa não tem custo extra de ida e volta.
- Um bloco Comment Count por artigo ou produto em uma listagem é o uso esperado; o bloco pode ser adicionado quantas vezes você precisar.