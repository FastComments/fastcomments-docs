[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

我们可以通过将 **enableSpoilers** 标志设置为 true 来启用剧透支持：

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

这也可以不通过代码完成。在小部件自定义页面，请查看 "启用剧透" 选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

当文本被高亮并点击现在可见的 `SPOILER` 按钮时，文本将被遮盖，直到用户将鼠标移到其上。对于暗色模式，我们采用相同的做法，但使用不同的
颜色以更好地匹配暗色模式。

这也与所见即所得编辑器兼容。