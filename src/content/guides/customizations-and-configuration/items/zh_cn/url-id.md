[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

在呈现评论线程或发布评论时，FastComments 需要知道这些评论属于哪个页面、文章或产品。

为此，我们使用所谓的 "URL ID"。它可以是一个标识符（例如字符串或数字），也可以是一个 URL。

默认情况下，如果未指定 urlId，它将变为页面 URL。我们会获取当前页面的 URL，并清理它以移除任何常见的营销参数或跟踪标识。

在第三方集成（例如 WordPress）的情况下，我们的插件通常会使用表示当前正在查看信息的标识符作为 URL ID，例如文章/页面 id。

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

我们在本文档中经常会提到 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">小部件自定义界面</a>。

该界面可用于在不使用代码的情况下对评论小部件进行许多更改。

在创建自定义规则时，我们通常希望它应用于网站的所有页面。但是，在某些情况下，我们希望对特定页面的评论小部件进行自定义，例如应用自定义样式，或使该页面的评论匿名。你也可以例如让某些页面的实时评论立即显示，而在其他页面将其隐藏在通知按钮下。

这些都可以通过该页面上的 URL ID 输入字段实现，该字段看起来如下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

该字段中的值应与传入评论小部件的 *urlId* 参数匹配。如果你希望自定义规则对 *urlId* 不敏感，请将此字段留空或输入 *。

自 2023 年起，小部件自定义中的 `URL ID` 字段现在也接受模式！例如，你可以使用 `*/blog/*` 为你的博客添加特定样式，使用 `*/store/*` 为你的商店添加特定样式，同时仍使用相同的域名。

### 注意事项

1. 如果你的页面有哈希参数（像 example.com#page-1） - 默认情况下，这将成为 URL ID 的一部分。
2. 在迁移期间，例如从 WordPress 到 Gatsby，你可能需要在初始迁移之后迁移 URL ID 的评论值。为此，请联系我们。