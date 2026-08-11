[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

在渲染评论线程或留下评论时，FastComments 需要知道这些评论属于哪个页面、文章或产品。

为此，我们使用一种称为 “URL ID” 的标识。它可以是标识符（如字符串或数字），也可以是一个 URL。

默认情况下，如果未指定 urlId，它将使用页面的 URL。我们会获取当前页面的 URL，并清除其中的常见营销参数或跟踪标识符。

在第三方集成（如 WordPress）的情况下，我们的插件通常会使用代表当前查看信息的标识符作为 URL ID，例如文章/页面的 ID。

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = '定义自定义 URL ID'; code-example-end]

本文档中我们经常会提到 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">小部件自定义 UI</a>。

该 UI 可用于在无需编写代码的情况下对评论小部件进行多种更改。

创建自定义规则时，我们通常希望它适用于站点的所有页面。然而，在某些情况下，我们希望对特定页面的评论小部件进行自定义，例如应用自定义样式，或将该页面的评论设为匿名。您也可以例如让某些页面的实时评论立即显示，而在其他页面则通过通知按钮隐藏。

这全部可以通过此页面上的 URL ID 输入字段实现，示例如下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='用于将自定义规则限定到单个页面或模式（如 */blog/*）的 URL ID 字段'; title='小部件自定义页面中的 URL ID 输入' app-screenshot-end]

此字段的值应与传入评论小部件的 *urlId* 参数匹配。如果希望自定义规则对 *urlId* 不敏感，请将此字段留空或输入 *。

自 2023 年起，widget 自定义中的 `URL ID` 字段也支持模式！例如，您可以使用 `*/blog/*` 为博客添加特定样式，使用 `*/store/*` 为商店添加特定样式，且仍使用同一域名。

### 注意事项

1. 如果您的页面包含哈希参数（如 example.com#page-1），默认情况下它们会成为 URL ID 的一部分。
2. 在迁移期间，例如从 WordPress 迁移到 Gatsby，您可能需要在初始迁移后迁移 URL ID 评论值。为此，请联系我们。