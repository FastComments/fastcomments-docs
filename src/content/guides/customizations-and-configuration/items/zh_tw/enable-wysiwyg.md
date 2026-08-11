---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 的格式化功能是透過在文字周圍加入可見的錨點標籤（例如 `<b></b>`）來實現。點擊工具列
或使用快捷鍵會為您自動完成此操作。然而，某些社群可能希望使用不帶錨點標籤的格式化。這稱為啟用
WYSIWYG（所見即所得）編輯器。此編輯器與預設編輯器外觀完全相同，只是會載入一些
額外的程式碼，允許使用者在不顯示錨點標籤的情況下加粗、加底線等。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = '啟用 WYSIWYG 編輯'; code-example-end]

這也可以不寫程式碼完成。在小工具自訂頁面中，請查看「啟用進階格式化」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='已勾選「啟用進階格式化」核取方塊以開啟 WYSIWYG 編輯器的小工具自訂頁面'; title='啟用 WYSIWYG' app-screenshot-end]

---