[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

一些文字，例如標題或訊息，可以呈現在留言數量下方但登入狀態文字上方。

我們稱之為「標頭」，預設為隱藏。

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

此項設定可在小工具自訂頁面中的「進階選項」下，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; title='Specifying Header HTML' app-screenshot-end]