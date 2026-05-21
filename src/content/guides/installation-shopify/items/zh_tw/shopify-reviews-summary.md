---
**FastComments - 評論摘要** 區塊會顯示頁面的彙總星級評分與評論細分。將它與產品範本上的 **FastComments** 區塊搭配使用，形成標準的評論版面：上方為摘要，下方則是評論表單與評論列表。

### 先決條件：設定 評等與評論

評論摘要區塊會顯示您為商店設定的評分問題。請先完成這些設定：

1. 在 Shopify 管理後台開啟 FastComments 應用程式。
2. 點選 **評等與評論小幫手** 磚塊（或直接開啟 [評等與評論小幫手](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)）。
3. 新增您希望每位評論者回答的問題（整體星級、「尺寸如何」等）。

如果沒有設定任何問題，摘要區塊就沒有可彙總的內容。

### 新增區塊

1. 開啟 Shopify 主題編輯器。
2. 開啟 **Product** 範本（或您想放置摘要的頁面範本）。
3. 在頁面區段上方、**FastComments** 區塊上方，點選 **新增區塊**。
4. 在 **Apps** 底下，選取 **FastComments - Reviews Summary**。
5. 如果尚未新增，請在同一頁面較下方加入一個 **FastComments** 區塊，讓訪客可以留下評論。
6. 點選 **儲存**。

### 設定

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | 覆蓋摘要所讀取的 FastComments 租戶。留空則使用商店自動設定的租戶。 | (空白) |
| Custom URL ID | 覆蓋摘要要彙總的頁面識別 ID。當摘要所在頁面與其所反映的 FastComments 區塊不同時使用此項。 | (自動偵測) |

### 摘要如何對應評論

評論摘要區塊使用與 **FastComments** 區塊相同的自動偵測邏輯：

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

對於一般的產品頁面，摘要與評論串會自動共享 URL ID，無需額外設定。

### 提示

- 摘要為唯讀。若要收集評論，您需要在同一頁面放置 **FastComments** 區塊。
- 若在收集評論後於評等與評論小幫手中變更評分問題，摘要會依新的問題集重新計算。

---