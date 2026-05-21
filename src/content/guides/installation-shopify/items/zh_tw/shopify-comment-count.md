The **FastComments - Comment Count** block 會在單一頁面上呈現一個小型的留言數。將它用在部落格文章列表、商品卡片或任何連到有留言之頁面的範本中，讓訪客在點入之前就能看到每個討論串的活躍程度。

### Add the block

1. 開啟 Shopify 主題編輯器。
2. 開啟你想要顯示計數的範本。例如，**Blog** 範本（文章列表）或商品列表區段。
3. 在呈現每個項目的區段中，點選 **Add block**。
4. 在 **Apps** 底下，選取 **FastComments - Comment Count**。
5. 點選 **Save**。

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | 覆寫 Comment Count 要讀取的 FastComments 租戶。留空以使用商店自動設定的租戶。 | (空白) |
| Custom URL ID | 覆寫計數要查詢的頁面識別碼。當計數位於與其追蹤之 FastComments 區塊不同的頁面時使用此項。 | (自動偵測) |

### How the count matches the comment thread

Comment Count 區塊使用與 **FastComments** 區塊相同的自動偵測邏輯：

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

如果你在某頁面的 **FastComments** 區塊上設定了 **Custom URL ID**，請在 Comment Count 區塊上設定相同的 Custom URL ID，以便它們指向相同的討論串。

### Tips

- 頁面上每個項目的計數會在一次請求中取得，因此在長列表的每個項目都加入該區塊不會產生額外的往返成本。
- 在列表中，每篇文章或每個商品使用一個 Comment Count 區塊是預期的用法；你可以視需要重複新增此區塊。