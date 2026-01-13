[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

預設情況下，FastComments 評論小工具會將 `gif rating` 設為 `pg`。

可用的選項有 `g`、`pg`、`pg-13` 與 `r`。

此設定可在程式碼中或透過 UI 設定。在程式碼中可以這樣做：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

在 UI 中，您會在 `Gif Picker Rating` 下找到此設定，只要 `Disable Image Uploads?` 未被勾選。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]