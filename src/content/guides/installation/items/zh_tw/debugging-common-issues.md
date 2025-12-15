以下是我們經常遇到的一些症狀和常見解決方案。

### 「This is a demo」訊息

當您從我們使用示範租戶的首頁複製小工具程式碼時會顯示此訊息。要使用您的租戶，請從[這裡](https://fastcomments.com/auth/my-account/get-acct-code)複製小工具程式碼。

### 「FastComments cannot load on this domain」錯誤

FastComments需要知道哪些網域屬於您，以驗證與您帳戶相關的請求。[查看我們的文檔](/guide-multiple-sites.html#add-domains-to-account)了解如何解決此錯誤（只需將確切的子網域+網域添加到您的帳戶）。

請注意，這應該只在試用期結束後發生。在試用期內，來自新網域的任何請求都會自動添加到您的帳戶。

### 自訂安裝中遷移的評論未顯示

通常這發生在匯入的評論與`Page ID`關聯，而您傳遞的是URL（或未傳遞值，在這種情況下預設使用頁面URL）時。

您可以透過[匯出評論](https://fastcomments.com/auth/my-account/manage-data/export)並查看`URL ID`欄（目前是欄`B`）來除錯此問題。

確保您在`URL ID`欄中看到的值與您傳遞給小工具配置的`urlId`參數的值相同。

有關更多說明，請閱讀我們的[評論如何與頁面和文章關聯的文檔](/guide-customizations-and-configuration.html#url-id)。

如果都失敗了，[請聯繫我們](https://fastcomments.com/auth/my-account/help)。

### 評論小工具未顯示

如果評論小工具未顯示，請檢查Chrome開發者控制台是否有錯誤。

對於大多數錯誤配置，評論小工具至少會在頁面上顯示錯誤（如果它能夠載入）。什麼都看不到通常表示有腳本錯誤。

### 所需配置未按預期工作

嘗試我們的[Chrome擴充功能](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)查看傳遞給評論小工具的配置。如果都失敗了，截取Chrome擴充功能顯示的內容並[聯繫我們](https://fastcomments.com/auth/my-account/help)。

### 具有不同雜湊標記的相同URL上的評論缺失

預設情況下，FastComments將使用頁面URL作為儲存評論的「桶」。如果您的URL包含`#hashbangs`，而這些`#hashbangs`不應該是標識評論串的標識符的一部分，我們可以簡單地忽略雜湊標記值，例如：

[inline-code-attrs-start title = '忽略雜湊標記範例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

請注意，進行此更改後，需要對現有評論進行遷移。[為此，請聯繫我們。](https://fastcomments.com/auth/my-account/help)

### URL查詢參數影響小工具

預設情況下，FastComments將使用頁面URL作為儲存評論的「桶」。如果您的URL包含不應該是標識評論串的標識符一部分的查詢參數，我們可以簡單地忽略它們，例如：

[inline-code-attrs-start title = '忽略查詢參數'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

請注意，進行此更改後，需要對現有評論進行遷移。[為此，請聯繫我們。](https://fastcomments.com/auth/my-account/help)

### 未收到電子郵件

在FastComments，我們投入大量工作以確保電子郵件傳遞盡可能可靠。但是，一些電子郵件提供商以難以可靠傳遞而聞名。請檢查您的垃圾郵件資料夾中是否有來自fastcomments.com的郵件。

如果您[聯繫我們](https://fastcomments.com/auth/my-account/help)，我們通常可以提供更多關於您為什麼可能沒有收到我們電子郵件的資訊。
