[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許使用者封鎖其他使用者。封鎖使用者會導致其評論被隱藏，阻止使用者之間的通知，等等。

可能需要停用此功能。可以這樣做：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = '停用封鎖'; code-example-end]

也可以不使用程式碼，透過小工具自訂 UI 來完成，這同時也能啟用正確的伺服器端驗證：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='在小工具自訂 UI 中的停用封鎖選項，可防止使用者互相封鎖'; title='停用封鎖' app-screenshot-end]