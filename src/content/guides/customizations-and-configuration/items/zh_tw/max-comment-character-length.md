---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

可在評論輸入欄位中輸入的最大字元數可透過 **maxCommentCharacterLength** 參數加以限制。

預設值為 2000。

像是圖片 URL 之類的內容不會計入長度計算。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = '限制評論長度'; code-example-end]

這可以在小工具自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='小工具自訂頁面上的最大評論大小欄位，用於限制評論可包含的字元數'; title='限制評論長度' app-screenshot-end]

---