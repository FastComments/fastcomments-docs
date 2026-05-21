The **FastComments** 區塊是主要的留言小工具。將它加入部落格文章範本、商品範本，或任何你想要討論串或即時聊天的頁面。

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
| Tenant ID (optional) | 覆蓋此區塊所渲染的 FastComments 租戶。留空以使用商店自動設定的租戶。可在 fastcomments.com/auth/my-account/api-secret 找到手動的租戶 ID。 | （空白） |
| SSO | 在留言前自動將訪客以其 Shopify 客戶帳號登入。請參見 [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso)。 | 開啟 |
| Commenting Style | 使用 **Threaded** 以支援巢狀回覆與投票，或使用 **Streaming** 以實時顯示像聊天訊息流的留言。 | Threaded |
| Custom URL ID | 覆蓋自動偵測的頁面識別碼。當你希望兩個 URL 共用同一討論串時使用此項。 | （自動偵測） |

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