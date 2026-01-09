[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

對於身份驗證，FastComments 依賴瀏覽器啟用第三方 Cookie。若未啟用，使用者將始終需要
留下電子郵件才能評論（除非電子郵件輸入欄位被隱藏），且他們的評論預設會顯示為未驗證。

為了解決此問題，您可以啟用第三方 Cookie 繞過功能。 

啟用此設定時，會出現一個小彈出視窗，顯示使用者正在登入的訊息。這個彈出視窗
會在使用者與評論小工具互動時顯示；例如，當他們發表評論時。

我們可以在程式碼中透過將 **enableThirdPartyCookieBypass** flag 設為 true 來達成此目的：

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

我們也可以在小工具自訂 UI（Widget Customization UI）中設定，路徑為 `Enable Third-Party Cookie Popup`：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---