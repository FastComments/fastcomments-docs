[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 小部件会按垂直方向调整大小以适应所有可见评论。分页通过当前页面末尾的 "View Next" 按钮实现，因为我们发现这种交互对大多数用户来说是最为舒适的。

但是，在某些情况下更偏好使用无限滚动。例如，我们在 Stream Chat 产品中使用此功能。

我们可以通过将 **enableInfiniteScrolling** 标志设置为 true 来隐藏 "View Next" 按钮并切换到无限滚动：

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

这还需要添加自定义 CSS。为 `.comments` 选择器添加自定义 CSS 以启用滚动，例如：

[inline-code-attrs-start title = '启用无限滚动'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

下面是一个完整的工作示例：

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

在上例中我们使用了 `customCSS` 属性，但出于性能原因，建议改为使用 Widget Configuration UI。 [查看自定义 CSS 文档。](/guide-customizations-and-configuration.html#custom-css)

---