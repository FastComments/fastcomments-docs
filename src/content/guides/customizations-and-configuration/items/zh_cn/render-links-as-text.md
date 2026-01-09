---
默认情况下，FastComments 会像这样渲染链接： [https://exmaple.com](https://exmaple.com) - 链接 URL 会变成一个可点击的
HTML 锚点标签。

有些站点可能希望禁用此功能，例如为了阻止诈骗者。我们通过将 `Comment HTML Rendering Option` 设置为 `Links as Text` 来提供此选项。

这可以在小部件自定义页面上无须编写代码即可为整个域或某个页面进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]

---