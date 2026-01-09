[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

在呈現留言串或發表留言時，FastComments 需要知道那些留言屬於哪個頁面、文章或產品。

為此，我們使用一個稱為 "URL ID" 的東西。它可以是像字串或數字之類的識別碼，或是一個 URL。

預設情況下，如果您沒有指定 urlId，它將成為頁面 URL。我們會取得當前頁面的 URL，並清理它以移除任何常見的行銷參數或追蹤識別碼。

在第三方整合的情況下，例如 WordPress，我們的外掛通常會使用代表當前正在檢視資訊的識別碼作為 URL ID，例如文章/頁面的 id。

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

本文檔中我們經常會參考的一項功能是 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>。

此介面可用於在不撰寫程式碼的情況下對留言元件進行多項變更。

在建立自訂規則時，我們通常希望它套用到網站上的所有頁面。然而，在某些情況下，我們只想在特定頁面上自訂留言元件，可能是套用自訂樣式，或是讓該頁面的留言匿名。舉例來說，您也可以讓某些頁面的即時留言立即顯示，而在其他頁面則將其隱藏在通知按鈕下。

這些都可以透過此頁面的 URL ID 輸入欄位實現，其外觀如下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

此欄位中的值應與傳入留言元件的 *urlId* 參數相符。如果您希望自訂規則與 *urlId* 無關，請將此欄位留空或輸入 *。

從 2023 年起，widget 自訂中的 `URL ID` 欄位現在也接受模式！例如，您可以使用 `*/blog/*` 為您的部落格加入特定樣式，或使用 `*/store/*` 為您的商店加入特定樣式，而兩者仍可共用相同的網域。

### 注意事項

1. 如果您的頁面具有 hash 參數（例如 example.com#page-1）— 預設情況下，這會成為 URL ID 的一部分。
2. 在進行遷移時，例如從 WordPress 遷移到 Gatsby，您可能需要在初始遷移後遷移 URL ID 的留言值。若需要協助，請與我們聯繫。