[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 設計上允許自訂。為了安全，評論元件本身在 iframe 中執行，因此要套用自訂樣式必須採取下列兩種方法之一。

第一種，也是最簡單且我們推薦的方法，是使用 [小工具自訂頁面](https://fastcomments.com/auth/my-account/customize-widget)。

在小工具自訂頁面，請查看「顯示進階選項」區段，在該區段下有一個標示為「Custom CSS」的區域：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

這種方法有一些好處：
1. 輸入的 CSS 在傳送給使用者之前會被壓縮，且在編輯介面中保持一致的格式。
2. 您可以獲得小工具自訂介面的所有好處，例如可以為不同網站輕鬆地以不同方式自訂評論元件。
3. 當我們對評論元件進行變更時，您的自訂樣式會作為我們發佈流程的一部分進行測試。

第二種方法是在小工具設定中指定 **customCSS** 參數，如下：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

然而，這有一些*限制*：
1. 由於標頭大小的限制，可以傳遞的自訂 CSS 大小有限，超過時我們的伺服器會拒絕該請求。
2. 您必須在自己的基礎設施與建置系統中管理自訂 CSS。這也可能是優點而非缺點。
3. 在此情況下，傳送自訂 CSS 會增加額外的網路負擔，因為必須先傳到我們的伺服器，然後再在 iframe 內容中傳回，總共 **兩次**。不過對於大多數載入量大小而言，這不會太明顯。
4. 常見的優化是將 CSS 最小化（minify）以減少網路傳輸大小，但採用此方法時需由您自行處理。
5. 當我們進行變更時，您的自訂 CSS 不會被我們測試。

### External CSS Files

您可以透過使用 `@import` 指示小工具擷取外部檔案！

建議將 `@import` 放在自訂規則中。這樣一來，如果我們需要對評論元件進行變更，就可以使用自動化工具驗證您的設定。例如，您會在小工具自訂介面中建立一個自訂規則，點選 `Advanced`，並在 `Custom CSS` 中輸入：

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

您也可以透過 `customCSS` 屬性載入外部 CSS 檔案：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

但請記住，如果您這樣做，我們將無法對您的 CSS 進行測試。 

### User Profile Modal Styling

使用者個人檔案快顯視窗也可以用自訂 CSS 進行樣式設定。然而，為了確保自訂樣式能套用到使用者個人檔案，所有 CSS 選擇器必須以 `.user-profile` 為前綴。若沒有此一前綴，自訂樣式將會在使用者個人檔案快顯視窗中被忽略。

例如：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

在 FastComments，我們知道客戶會自訂評論元件。這是我們的設計意圖 — 我們最不希望的是讓我們的產品造成您產品中的設計不一致。

由於這是我們產品的重要部分，我們有一個建置管線，可以在每次發佈時針對每個客戶檢視評論元件的變更。

如果我們發現小問題，我們會更新您的帳戶以確保發佈順利進行；如果發現重大破壞性變更，這也讓我們能夠中止發佈。

---