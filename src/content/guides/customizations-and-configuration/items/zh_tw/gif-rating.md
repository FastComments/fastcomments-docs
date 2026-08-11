---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

預設情況下，FastComments 評論小工具會將 `gif rating` 設為 `pg`。

可用的選項有 `g`、`pg`、`pg-13` 與 `r`。

這可以在程式碼中或透過 UI 設定。以下是在程式碼中的設定方式：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = '設定 Gif 評分'; code-example-end]

在 UI 中，只要未勾選 `Disable Image Uploads?`，您就會在 `Gif Picker Rating` 下找到此設定。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='小工具自訂頁面上的 Gif Picker Rating 下拉選單，提供 g、pg、pg-13 和 r'; title='設定 Gif 評分' app-screenshot-end]

---