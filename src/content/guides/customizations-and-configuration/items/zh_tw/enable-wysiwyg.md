---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 的格式化功能是透過在文字周圍加入像 `<b></b>` 這類可見的標籤來完成。點擊工具列
或使用快速鍵會為你執行這個動作。不過，有些社群可能希望選擇在沒有可見標籤的情況下使用格式化功能。這稱為啟用
WYSIWYG（所見即所得）編輯器。這個編輯器的外觀與預設編輯器完全相同，不同之處在於它會載入一些
額外的程式碼，讓使用者可以在不產生可見標籤的情況下將文字加粗、加底線等。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

這也可以在不撰寫程式碼的情況下完成。在小工具自訂頁面，請查看「啟用進階格式化」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---