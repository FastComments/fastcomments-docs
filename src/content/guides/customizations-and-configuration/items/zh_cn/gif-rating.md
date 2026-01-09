[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

默认情况下，FastComments 评论小部件会将 `gif rating` 设置为 `pg`。

可用的选项有 `g`、`pg`、`pg-13` 和 `r`。

可以在代码中或通过 UI 进行设置。我们可以在代码中这样做：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

在 UI 中，只要未勾选 `Disable Image Uploads?`，您就可以在 `Gif Picker Rating` 下找到此设置。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]