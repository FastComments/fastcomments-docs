**FastComments - Comment Count** 區塊會為單一頁面顯示一個小的留言數量。請將它用於部落格文章列表、產品卡片，或任何連結到有留言頁面的範本，讓訪客在點擊進入前就能看到每個討論串的活躍程度。

### 新增區塊

1. 開啟 Shopify 佈景主題編輯器。
2. 開啟您希望顯示數量的範本。例如 **Blog** 範本(文章列表)或產品列表區塊。
3. 在呈現每個項目的區段中，點擊 **Add block**。
4. 在 **Apps** 下方，選擇 **FastComments - Comment Count**。
5. 點擊 **Save**。

### 設定

| 設定 | 功能說明 | 預設值 |
|---|---|---|
| Tenant ID(選填) | 覆寫留言數量所讀取的 FastComments 租戶。留空則使用商店自動設定的租戶。 | (空白) |
| Custom URL ID | 覆寫留言數量查詢的頁面識別碼。當留言數量區塊與其追蹤的 FastComments 區塊位於不同頁面時使用此設定。 | (自動偵測) |

### 留言數量如何對應留言討論串

Comment Count 區塊使用與 **FastComments** 區塊相同的自動偵測邏輯：

- 部落格文章範本:`shopify-article-{article.id}`
- 產品範本:`shopify-product-{product.id}`
- 其他範本：請求路徑

如果您在頁面上的 **FastComments** 區塊設定了 **Custom URL ID**,請在 Comment Count 區塊上設定相同的 Custom URL ID,讓兩者指向同一個討論串。

### 提示

- 頁面上每個項目的數量會在一次請求中全部取得，因此在長列表的每個項目中加入此區塊不會產生額外的來回請求成本。
- 在列表中每篇文章或每個產品使用一個 Comment Count 區塊是預期的用法；您可以視需要新增任意數量的區塊。