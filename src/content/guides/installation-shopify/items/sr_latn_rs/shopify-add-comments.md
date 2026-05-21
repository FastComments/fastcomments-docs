Blok **FastComments** je glavni vidžet za komentare. Dodajte ga u šablone objava na blogu, šablone proizvoda ili bilo koju drugu stranicu na kojoj želite nit razgovora ili live chat.

### Add the block

1. Open the Shopify theme editor (**Online Store > Themes > Customize**).
2. Pick the template you want comments on: **Blog post**, **Product**, or any other page or section template.
3. In the section where you want comments to appear, click **Add block**.
4. Under **Apps**, select **FastComments**.
5. Click **Save**.

The block appears immediately. There is no Tenant ID to enter; your store's tenant is wired up automatically when you install the app.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the block renders against. Leave blank to use the store's automatically-configured tenant. Find a manual tenant ID at fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Auto-logs the visitor in as their Shopify customer account before commenting. See [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** for nested replies and votes, or **Streaming** for a real-time chat feed. | Threaded |
| Custom URL ID | Override the auto-detected page identifier. Use this when you want two URLs to share one comment thread. | (auto-detected) |

### How the page identifier is chosen

Each comment thread is keyed by a URL ID. The block picks one automatically:

- **Blog post template:** `shopify-article-{article.id}`, which is stable across slug and title changes.
- **Product template:** `shopify-product-{product.id}`, which is stable across slug and title changes.
- **Other templates:** the request path.

If you set **Custom URL ID**, that value is used instead. Use the same Custom URL ID across multiple blocks (for example, on a localized variant of a product page) to share one comment thread.

### Threaded vs Streaming

**Threaded** is the default. Visitors reply to each other, vote, and moderation tools work as expected. Best for blog posts and product reviews.

**Streaming** drops the threading and shows new comments in real time as they're posted, like a chat feed. Best for product launches, live events, and community pages.

### Multiple blocks on the same page

The block can be added more than once to the same template. For example, a Reviews Summary at the top of a product page and a FastComments block at the bottom. The blocks share a URL ID, so the summary reflects the comments below.

### Tips

- The block hides itself in the theme editor preview with a yellow notice if it can't find a tenant. If that appears in your live store, reinstall the FastComments app.
- For a product page, the FastComments block doubles as your product reviews widget. Pair it with **FastComments - Reviews Summary** for a star-rating summary at the top of the page.