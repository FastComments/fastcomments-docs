[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

我们可以通过将 **enableSpoilers** 标志设置为 true 来启用剧透支持：

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = '启用剧透功能'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面中，请参阅“Enable Spoilers”选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='小部件自定义页面，勾选了 Enable Spoilers 复选框以在编辑器中添加 SPOILER 按钮'; title='启用剧透功能' app-screenshot-end]

当文本被选中并点击此时可见的 `SPOILER` 按钮后，文本将被遮蔽，直到用户将鼠标悬停在上面。在深色模式下我们采用相同的处理方式，但使用更适合深色模式的不同颜色。

这也与所见即所得（WYSIWYG）编辑器兼容。