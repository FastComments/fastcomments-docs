以下是我們常見遇到的一些症狀，以及常見的解決方案。

### "This is a demo" 訊息

當您從我們的首頁複製小工具程式碼時，會顯示此訊息，因為首頁使用示範租戶。要使用您的租戶，請從 [這裡](https://fastcomments.com/auth/my-account/get-acct-code) 複製小工具程式碼。

### "FastComments cannot load on this domain" 錯誤

FastComments 需要知道哪些網域是您擁有的，以便驗證與您的帳戶相關的請求。請參閱我們的[文件](/guide-multiple-sites.html#add-domains-to-account) 以了解如何解決此錯誤（只需將完全的子網域 + 網域新增到您的帳戶即可）。

請注意，這通常只會在試用期結束後發生。在試用期間，來自新網域的任何請求會自動新增到您的帳戶。

### 自訂安裝中已遷移的評論未顯示

通常這會發生在匯入的評論綁定到 `Page ID`，而您傳遞的是 URL（或沒有值，此時會預設為該頁面的 URL）。

您可以透過[匯出您的評論](https://fastcomments.com/auth/my-account/manage-data/export) 並查看 `URL ID` 欄（目前為第 `B` 欄）來進行除錯。

請確保您在 `URL ID` 欄看到的值，與您傳給小工具設定的 `urlId` 參數值相同。

如需進一步說明，請參閱我們的[評論如何與頁面與文章綁定的說明文件](/guide-customizations-and-configuration.html#url-id)。

如果仍無法解決，請[聯絡我們](https://fastcomments.com/auth/my-account/help)。

### 評論小工具未顯示

如果評論小工具未顯示，請檢查 Chrome 開發者主控台是否有錯誤。

對於大多數設定錯誤來說，只要評論小工具能夠載入，至少會在頁面上顯示錯誤。完全看不到通常表示有腳本錯誤。

### 設定未按預期運作

請嘗試使用我們的 [Chrome 擴充功能](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) 來查看傳遞給評論小工具的設定。如果仍然失敗，請截圖該 Chrome 擴充功能所顯示的內容，然後[聯絡我們](https://fastcomments.com/auth/my-account/help)。

### 相同 URL 但不同 Hash Bang 的評論遺失

預設情況下，FastComments 會使用頁面 URL 作為存放評論的「bucket」。如果您的 URL 包含 `#hashbangs`，且這些 `#hashbangs` 不應該成為識別評論串的識別符的一部分，我們可以簡單地忽略 hash bang 的值，例如：

[inline-code-attrs-start title = '忽略 Hash Bang 範例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

請注意，在進行此變更之後，現有的評論將需要執行一次遷移。[關於此部分，請聯絡我們。](https://fastcomments.com/auth/my-account/help)

### 影響小工具的 URL 查詢參數

預設情況下，FastComments 會使用頁面 URL 作為存放評論的「bucket」。如果您的 URL 包含不應該成為識別評論串的識別符一部分的查詢參數，我們可以簡單地忽略它們，例如：

[inline-code-attrs-start title = '忽略查詢參數'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

請注意，在進行此變更之後，現有的評論將需要執行一次遷移。[關於此部分，請聯絡我們。](https://fastcomments.com/auth/my-account/help)

### 未收到電子郵件

在 FastComments，我們投入大量工作以確保電子郵件的傳遞盡可能可靠。然而，有些電子郵件提供商確實很難穩定地投遞。請檢查您的垃圾郵件資料夾中是否有來自 fastcomments.com 的郵件。

如果您[聯絡我們](https://fastcomments.com/auth/my-account/help)，我們通常可以提供更多關於您可能未看到我們郵件的原因的資訊。

---