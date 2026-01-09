---
預設情況下，FastComments 會將連結渲染為如下形式： [https://exmaple.com](https://exmaple.com) - 連結 URL 會變成可點擊的
HTML 錨點標籤。

有些網站可能想要停用此功能，例如為了阻止詐騙者。我們提供的方法是將 `Comment HTML Rendering Option` 設為 `Links as Text`。

此設定可在不撰寫程式碼的情況下自訂，於 Widget 自訂頁面上針對整個網域，或特定頁面：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]

---