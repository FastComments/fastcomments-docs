[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許使用者封鎖其他使用者。封鎖某位使用者會導致其評論
被隱藏、阻止雙方之間的通知，等等。

有時可能希望停用此功能。可以如此操作：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

也可以透過 Widget 自訂介面在不寫程式碼的情況下完成，並同時啟用適當的伺服器端驗證：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---