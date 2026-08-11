---
默认情况下，FastComments 会将链接渲染为如下形式： [https://exmaple.com](https://exmaple.com) - 其中链接 URL 会变成可点击的 HTML 锚点标签。

某些站点可能希望禁用此功能，例如以防止诈骗者。我们通过将 `Comment HTML Rendering Option` 设置为 `Links as Text` 来实现此功能。

此设置可在无需编写代码的情况下进行自定义，可在小部件自定义页面上针对整个域或单个页面进行设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; alt='在小部件自定义的高级选项中，将 Comment HTML Rendering Option 设置为 Links as Text'; title='将链接渲染为文本' app-screenshot-end]
---