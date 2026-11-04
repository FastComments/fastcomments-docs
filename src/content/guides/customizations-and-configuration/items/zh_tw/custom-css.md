[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 的設計宗旨是可被自訂。基於安全考量，留言小工具本身是在 iframe 中執行的，因此若要套用自訂樣式，您必須採用以下兩種方法之一。

第一種方法最簡單，也是我們推薦的方式，就是使用[小工具自訂頁面](https://fastcomments.com/auth/my-account/customize-widget)。

在小工具自訂頁面中，請查看「Show Advanced Options」區段，其下方有一個標示為「Custom CSS」的區域：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='小工具自訂頁面中 Show Advanced Options 下方的 Custom CSS 編輯器'; title='Custom CSS 輸入區域' app-screenshot-end]

這種方法有以下好處：
1. 輸入的 CSS 會在傳送給使用者之前被壓縮（minify），並在編輯介面中保持格式一致。
2. 您可以享有小工具自訂介面的所有好處，例如輕鬆地為不同網站設定不同的留言小工具樣式。
3. 當我們對留言小工具進行變更時，您的自訂樣式會作為我們發布流程的一部分進行測試。

第二種方法是在小工具設定中指定 **customCSS** 參數，如下所示：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = '傳遞自訂 CSS'; code-example-end]

然而，這種方式有*一些限制*：
1. 由於標頭大小的限制，能傳遞的自訂 CSS 數量有限，超過上限我們的伺服器將會拒絕請求。
2. 您必須在自己的基礎架構與建置系統中管理自訂 CSS。不過這也可能是優點而非缺點。
3. 在這種使用情境下，自訂 CSS 會在網路上額外傳輸**兩次**，因為它必須先傳送到我們的伺服器，然後再透過 iframe 內容傳回。不過對大多數的負載大小而言，這並不明顯。
4. 常見的最佳化做法是壓縮 CSS 以減少網路傳輸量，但採用這種方法時，您必須自行處理。
5. 當我們進行變更時，您的自訂 CSS 不會被測試。

### 外部 CSS 檔案

您可以透過 `@import` 告訴小工具去擷取外部檔案！

建議將 `@import` 放在自訂規則中。這樣一來，如果我們需要對留言小工具進行變更，就可以使用我們的自動化工具來驗證您的設定。例如，您可以在小工具自訂介面中建立一條自訂規則，點擊 `Advanced`，然後在 `Custom CSS` 中輸入：

    @import url(https://example.com/styles.css);

#### 在程式碼中 - 不建議

您也可以透過 `customCSS` 屬性載入外部 CSS 檔案：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = '外部 CSS 檔案'; code-example-end]

不過請記住，如果您這樣做，我們將無法測試您的 CSS。

### 使用者個人資料彈出視窗樣式

使用者個人資料彈出視窗也可以使用自訂 CSS 進行樣式設定。不過，為了確保自訂樣式能套用於使用者個人資料，所有 CSS 選擇器都必須以 `.user-profile` 作為前綴。若沒有此前綴，自訂樣式將不會套用於使用者個人資料彈出視窗。

例如：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = '使用者個人資料 CSS'; code-example-end]

### 向下相容性

在 FastComments，我們知道客戶會自訂留言小工具。這正是我們的設計理念——我們最不希望的就是我們的產品造成您產品在設計上的不一致。

由於這是我們產品的重要一環，我們有一套建置流程，讓我們能在每次發布時，針對個別客戶審查留言小工具的變更。

如果我們發現輕微的問題，我們會更新您的帳戶以確保發布順利進行。如果我們發現重大的破壞性變更，這讓我們能夠暫停發布。