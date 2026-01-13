[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments 旨在支持自定义。评论组件本身出于安全原因运行在 iframe 中，因此要应用
自定义样式，必须遵循以下两种方法之一。

第一种，也是最简单且我们推荐的方法，是使用 [小部件自定义页面](https://fastcomments.com/auth/my-account/customize-widget)。

在小部件自定义页面，查看“显示高级选项”部分，其下有一个标为“自定义 CSS”的区域：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

这种方法有一些好处：
1. 输入的 CSS 在发送给用户之前会被压缩，而且在编辑界面中的格式保持一致。
2. 您可以获得小部件自定义界面的所有优点，例如可以为不同网站轻松地对评论组件进行不同的自定义。
3. 当我们对评论组件做出更改时，您的自定义样式将作为我们发布流程的一部分得到测试。

第二种方法是在小部件配置中指定 **customCSS** 参数，如下：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

但是，这有一些*限制*：
1. 由于请求头的大小限制，可传递的自定义 CSS 有一定限制，超过该限制我们的服务器将拒绝请求。
2. 您必须在自己的基础设施和构建系统中管理自定义 CSS。这也可能是一个优点而非缺点。
3. 在这种用例中，发送自定义 CSS 会产生额外的网络开销，因为它需要先发送到我们的服务器，然后再在 iframe 内容中返回，**会被发送两次**。不过对于大多数负载大小而言，这并不明显。
4. 常见的优化是对 CSS 进行压缩以减少网络传输大小，但使用此方法时您需要自行处理该操作。
5. 当我们进行更改时，您的自定义 CSS 将不会被测试。

### External CSS Files

您可以通过使用 `@import` 告诉小部件获取外部文件！

建议将 `@import` 放在自定义规则中。这样，如果我们需要对评论组件进行更改，我们可以使用我们的自动化
工具来验证您的设置。因此例如，您可以在小部件自定义 UI 中创建一个自定义规则，单击 `Advanced`，并在 `Custom CSS` 中输入：

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

您也可以通过 `customCSS` 属性加载外部 CSS 文件：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

但是，请记住，如果您这样做，我们将无法对您的 CSS 进行测试。 

### User Profile Modal Styling

用户资料模态也可以使用自定义 CSS 进行样式化。然而，为确保自定义样式应用于用户资料，所有 CSS 选择器必须以 `.user-profile` 为前缀。没有此前缀，用户资料模态的自定义样式将被忽略。

例如：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

在 FastComments，我们知道客户会自定义评论组件。这是刻意为之——我们最不希望的是我们的产品导致设计
在您的产品中出现不一致。

由于这是我们产品的重要部分，我们有一套构建流水线，允许我们在每次发布时针对每个客户审查对评论组件的更改。

如果我们发现小问题，我们会更新您的账户以确保发布顺利。如果我们发现重大破坏性更改，这使我们能够暂停发布。

---