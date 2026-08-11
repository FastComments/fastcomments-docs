[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 設計為可自訂。評論小工具本身因安全考量在 iframe 中執行，因此若要套用自訂樣式，必須遵循以下兩種方法之一。

第一種，也是最簡單且我們首選的方法，是使用[小工具自訂頁面](https://fastcomments.com/auth/my-account/customize-widget)。

在小工具自訂頁面，請查看「顯示進階選項」區段，其下有一個標示為「Custom CSS」的區域：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='在小工具自訂頁面的「顯示進階選項」下的自訂 CSS 編輯器'; title='自訂 CSS 輸入區域' app-screenshot-end]

此方法有以下好處：
1. 輸入的 CSS 會在送出給使用者之前被壓縮，且編輯介面的格式保持一致。
2. 您可獲得小工具自訂 UI 的所有好處，例如能輕鬆為不同網站自訂不同的評論小工具。
3. 當我們對評論小工具進行變更時，您的自訂樣式將作為我們發布流程的一部分進行測試。

第二種方法是於小工具設定中指定 **customCSS** 參數，如下所示：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

然而，這種方式有 *限制*：
1. 由於標頭大小的限制，傳遞的自訂 CSS 數量有上限，超過後我們的伺服器會拒絕請求。
2. 您必須在自己的基礎建設與建置系統中管理自訂 CSS。這也可能是一個優勢而非劣勢。
3. 在此情境下，會額外產生兩次傳送自訂 CSS 的網路開銷：一次傳送至我們的伺服器，然後再回傳至 iframe 內容。但對於大多數負載大小而言，影響不大。
4. 常見的最佳化方式是壓縮 CSS 以減少網路傳輸大小，但使用此方法時您必須自行處理。
5. 當我們進行變更時，您的自訂 CSS 不會被測試。

### External CSS Files

您可以使用 `@import` 讓小工具取得外部檔案！

建議將 `@import` 放在自訂規則中。如此一來，若我們需要對評論小工具進行變更，便能使用自動化工具驗證您的設定。例如，您可以在小工具自訂 UI 中建立自訂規則，點選 `Advanced`，並在 `Custom CSS` 中輸入：

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

您也可以透過 `customCSS` 屬性載入外部 CSS 檔案：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

但請記住，若這樣做，我們將無法測試您的 CSS。 

### User Profile Modal Styling

使用者個人資料彈出視窗也可以使用自訂 CSS 進行樣式設定。然而，為確保自訂樣式套用於使用者個人資料，所有 CSS 選擇器必須以 `.user-profile` 為前綴。若未加此前綴，對使用者個人資料彈出視窗的自訂樣式將被忽略。

例如：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

在 FastComments，我們了解客戶會自訂評論小工具。這本就是設計初衷——我們最不希望的是，我們的產品在您的產品中造成設計不一致。

鑑於這是我們產品的重要部分，我們擁有一條建置管線，能在每次發布時針對每位客戶審查評論小工具的變更。

若我們發現小問題，會更新您的帳號以確保發布順利進行。若發現重大破壞性變更，我們則會暫停發布。