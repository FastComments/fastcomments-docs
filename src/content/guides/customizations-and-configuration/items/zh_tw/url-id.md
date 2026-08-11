[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

在呈現評論串或發表評論時，FastComments 需要知道這些評論屬於哪個頁面、文章或產品。

為此，我們使用稱為「URL ID」的概念。它可以是識別碼，例如字串或數字，或是一個 URL。

預設情況下，如果未指定 urlId，系統會使用頁面 URL。我們會取得目前頁面的 URL，並清除常見的行銷參數或追蹤識別碼。

對於第三方整合（例如 WordPress），我們的外掛通常會使用代表目前所檢視資訊的識別碼作為 URL ID，例如文章/頁面的 ID。

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = '定義自訂 URL ID'; code-example-end]

本文件中我們常會提到的其中一項內容是 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>。

此 UI 可用於在不撰寫程式碼的情況下對評論小工具進行多項變更。

在建立自訂規則時，我們通常希望它套用至網站的所有頁面。然而，在某些情況下，我們可能想要在特定頁面上自訂評論小工具，無論是套用自訂樣式，或是讓該頁面的評論匿名。例如，你也可以讓某些頁面立即顯示即時評論，而在其他頁面則將評論隱藏在通知按鈕下。

這一切都可以透過此頁面的 URL ID 輸入欄位完成，欄位長相如下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='用於將自訂規則套用至單一頁面或類似 */blog/* 的模式的 URL ID 欄位'; title='Widget Customization 頁面中的 URL ID 輸入欄位' app-screenshot-end]

此欄位的值應與傳入評論小工具的 *urlId* 參數相符。如果希望自訂規則對 *urlId* 不敏感，請將此欄位留空或輸入 *。

自 2023 年起，widget 自訂中的 `URL ID` 欄位也支援模式！例如，你可以使用 `*/blog/*` 為部落格添加特定樣式，或使用 `*/store/*` 為商店添加特定樣式，同時使用相同的網域。

### Gotchas

1. 如果你的頁面包含雜湊參數（例如 example.com#page-1），預設情況下這些參數會成為 URL ID 的一部份。
2. 在遷移期間，例如從 WordPress 遷移至 Gatsby，可能需要在初次遷移後遷移 URL ID 評論值。如有需要，請與我們聯繫。