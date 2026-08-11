[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

對於驗證，FastComments 依賴於瀏覽器中啟用第三方 Cookie。若未啟用，使用者必須留下電子郵件才能發表評論（除非隱藏了電子郵件輸入欄位），且他們的評論預設會顯示為未驗證。

為了解決此問題，您可以啟用第三方 Cookie 繞過。 

啟用此設定後，會出現一個小彈出視窗，顯示使用者正在登入的訊息。此彈出視窗會在使用者與評論小工具互動時顯示，例如留下評論時。

我們可以在程式碼中將 **enableThirdPartyCookieBypass** 旗標設為 true 來達成：

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = '啟用第三方 Cookie 繞過'; code-example-end]

我們也可以透過 Widget 自訂介面設定此項，位於 `Enable Third-Party Cookie Popup` 之下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='已勾選「啟用第三方 Cookie 彈出視窗」的 Widget 自訂頁面'; title='啟用第三方 Cookie 繞過' app-screenshot-end]