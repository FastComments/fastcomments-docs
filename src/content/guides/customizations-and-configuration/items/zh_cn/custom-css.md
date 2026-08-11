[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 旨在实现高度自定义。出于安全考虑，评论小部件本身运行在 iframe 中，因此要应用自定义样式，需要遵循以下两种方法之一。

第一种，也是我们首选且最简便的方法，是使用[小部件自定义页面](https://fastcomments.com/auth/my-account/customize-widget)。

在小部件自定义页面，查看 “显示高级选项” 部分，其中有一个标记为 “自定义 CSS” 的区域：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='在小部件自定义页面的 “显示高级选项” 下的自定义 CSS 编辑器'; title='自定义 CSS 输入区域' app-screenshot-end]

此方法有以下优势：
1. 输入的 CSS 在发送给用户之前会被压缩，编辑 UI 中的格式保持一致。
2. 您可以获得小部件自定义 UI 的全部好处，例如可以为不同站点轻松定制评论小部件。
3. 当我们对评论小部件进行更改时，您的自定义样式将作为我们发布流程的一部分进行测试。

第二种方法是通过小部件配置中的 **customCSS** 参数指定，如下所示：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = '传递自定义 CSS'; code-example-end]

然而，这种方式有 *限制*：
1. 由于请求头的大小限制，传递的自定义 CSS 有上限，超过后我们的服务器会拒绝请求。
2. 您必须在自己的基础设施和构建系统中管理自定义 CSS。这有时也可能是一个优势。
3. 在此使用场景下，网络上会 **两次** 发送自定义 CSS：一次发送到我们的服务器，随后再返回到 iframe 内容中。不过对于大多数负载大小，这几乎感觉不到。
4. 常见的优化是对 CSS 进行压缩以减小网络传输大小，但使用此方法时您需要自行处理。
5. 当我们进行更改时，您的自定义 CSS 将不会被测试。

### 外部 CSS 文件

您可以使用 `@import` 让小部件加载外部文件！

建议将 `@import` 放在自定义规则中。这样，如果我们需要对评论小部件进行更改，就可以使用自动化工具验证您的设置。例如，您可以在“小部件自定义 UI” 中创建一条自定义规则，点击 `高级`，并在 `自定义 CSS` 中输入：

    @import url(https://example.com/styles.css);

#### 在代码中 - 不推荐

您也可以通过 `customCSS` 属性加载外部 CSS 文件：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = '外部 CSS 文件'; code-example-end]

但请记住，如果这样做，您的 CSS 将无法由我们进行测试。

### 用户资料模态框样式

用户资料模态框也可以使用自定义 CSS 进行样式化。不过，为确保自定义样式能够应用于用户资料，所有 CSS 选择器必须以 `.user-profile` 为前缀。没有此前缀，自定义样式将被忽略。

例如：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = '用户资料 CSS'; code-example-end]

### 向后兼容性

在 FastComments，我们知道客户会自定义评论小部件。这本就是设计初衷——我们最不希望的是我们的产品导致您产品的设计不一致。

由于这是我们产品的重要组成部分，我们拥有一条构建流水线，能够在每次发布时对评论小部件的每位客户的更改进行审查。

如果我们发现小问题，会更新您的账户以确保发布顺利进行。如果出现重大破坏性更改，这将使我们能够暂停发布。