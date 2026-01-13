[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

可輸入於評論輸入欄位的最大字元數可由 **maxCommentCharacterLength** 參數限制。

預設值為 2000。

像是圖片 URL 等不會包含在長度計算中。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

這可以在無需撰寫程式碼的情況下，在小工具自訂頁面上進行：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---