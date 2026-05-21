The **FastComments - Comment Count** block renders a small comment count for a single page. Use it in blog post lists, product cards, or any template that links to a page with comments, so visitors can see how active each thread is before clicking through.

### Add the block

1. Abra o editor de temas do Shopify.
2. Abra o template onde você quer que a contagem apareça. Por exemplo, o template **Blog** (a lista de posts) ou uma seção de listagem de produtos.
3. Clique em **Adicionar bloco** na seção que renderiza cada item.
4. Em **Apps**, selecione **FastComments - Comment Count**.
5. Clique em **Salvar**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the count reads from. Leave blank to use the store's automatically-configured tenant. | (em branco) |
| Custom URL ID | Override the page identifier the count looks up. Use this when the count is on a different page from the FastComments block it tracks. | (detectado automaticamente) |

### How the count matches the comment thread

The Comment Count block uses the same auto-detection logic as the **FastComments** block:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

If you set a **Custom URL ID** on the **FastComments** block on a page, set the same Custom URL ID on the Comment Count block so they point at the same thread.

### Tips

- Counts for every item on the page are fetched in one request, so adding the block to every item in a long list has no extra round-trip cost.
- One Comment Count block per article or product in a listing is the expected usage; the block can be added as many times as you need.