The **FastComments - Счётчик комментариев** block renders a small comment count for a single page. Use it in blog post lists, product cards, or any template that links to a page with comments, so visitors can see how active each thread is before clicking through.

### Add the block

1. Open the Shopify theme editor.
2. Open the template where you want the count to appear. For example, the **Blog** template (the post list) or a product listing section.
3. Click **Add block** in the section that renders each item.
4. Under **Apps**, select **FastComments - Comment Count**.
5. Click **Save**.

### Settings

| Параметр | Что делает | По умолчанию |
|---|---|---|
| Tenant ID (optional) | Переопределяет, из какого тенанта FastComments считывается счётчик. Оставьте пустым, чтобы использовать автоматически настроенный для магазина тенант. | (blank) |
| Custom URL ID | Переопределяет идентификатор страницы, по которому выполняется поиск счётчика. Используйте это, когда счётчик находится на другой странице, чем блок FastComments, за которым он отслеживает комментарии. | (auto-detected) |

### How the count matches the comment thread

The Comment Count block uses the same auto-detection logic as the **FastComments** block:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

If you set a **Custom URL ID** on the **FastComments** block on a page, set the same Custom URL ID on the Comment Count block so they point at the same thread.

### Tips

- Counts for every item on the page are fetched in one request, so adding the block to every item in a long list has no extra round-trip cost.
- One Comment Count block per article or product in a listing is the expected usage; the block can be added as many times as you need.