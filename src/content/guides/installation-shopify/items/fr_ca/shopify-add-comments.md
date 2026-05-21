Le bloc **FastComments** est le widget principal de commentaires. Ajoutez-le aux modèles d'articles de blog, aux modèles de produit ou à toute autre page où vous souhaitez un fil de discussion ou un clavardage en direct.

### Add the block

1. Ouvrez l'éditeur de thème Shopify (**Online Store > Themes > Customize**).
2. Choisissez le modèle sur lequel vous voulez des commentaires : **Blog post**, **Product**, ou tout autre modèle de page ou de section.
3. Dans la section où vous voulez que les commentaires apparaissent, cliquez sur **Add block**.
4. Sous **Apps**, sélectionnez **FastComments**.
5. Cliquez sur **Save**.

Le bloc apparaît immédiatement. Il n'y a pas de Tenant ID à entrer ; le tenant de votre boutique est configuré automatiquement lors de l'installation de l'application.

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