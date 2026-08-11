[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

默认情况下，FastComments 评论小部件会将 `gif rating` 设置为 `pg`。

可用的选项有 `g`、`pg`、`pg-13` 和 `r`。

可以在代码中或通过 UI 设置。代码中可以这样做：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = '设置 Gif 评级'; code-example-end]

在 UI 中，只要未选中 `Disable Image Uploads?`，您可以在 `Gif Picker Rating` 下找到它。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='小部件自定义页面上的 Gif Picker Rating 下拉菜单，提供 g、pg、pg-13 和 r'; title='设置 Gif 评级' app-screenshot-end]