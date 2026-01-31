下面列出一些我们常见的问题及常见解决方法。 

### “这是演示” 消息

当您从我们的主页复制了小部件代码（主页使用的是我们的演示租户）时，会显示此消息。要使用您的租户，请从[这里](https://fastcomments.com/auth/my-account/get-acct-code)复制小部件代码。

### “FastComments cannot load on this domain” 错误

FastComments 需要知道哪些域名属于您，以便对与您帐户相关的请求进行身份验证。[请查阅我们的文档](/guide-multiple-sites.html#add-domains-to-account) 以了解如何解决此错误（只需将确切的子域 + 域添加到您的帐户中）。

请注意，这种情况通常只会在试用期结束后发生。在试用期间，来自新域的任何请求会自动添加到您的帐户中。

### 自定义安装后迁移的评论未显示

通常发生在导入的评论与 `Page ID` 关联，而您传递的是 URL（或未传值，此时默认使用页面 URL）。

您可以通过[导出您的评论](https://fastcomments.com/auth/my-account/manage-data/export)并查看 `URL ID` 列（当前为列 `B`）来调试此问题。

确保您在 `URL ID` 列中看到的值与您在小部件配置中作为 `urlId` 参数传递的值相同。

有关详细说明，请阅读我们的[评论如何与页面和文章关联 文档](/guide-customizations-and-configuration.html#url-id)。

如果仍然无法解决，请[联系我们](https://fastcomments.com/auth/my-account/help)。

### 评论小部件未显示

如果评论小部件未显示，请检查 Chrome 开发者控制台中的错误。

对于大多数配置错误，如果评论小部件能够加载，至少会在页面上显示错误。什么也看不到通常表明存在脚本错误。

### 期望的配置未按预期工作

尝试使用我们的[Chrome 扩展](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)查看传递给评论小部件的配置。如果仍然无法解决，请截取 Chrome 扩展显示内容的屏幕截图并[联系我们](https://fastcomments.com/auth/my-account/help)。

### 相同 URL 但不同 Hash Bang 时评论缺失

默认情况下，FastComments 会使用页面 URL 作为存储评论的“桶”。如果您的 URL 包含 `#hashbangs`，而这些 `#hashbangs` 不应成为标识评论线程的标识符的一部分，我们可以简单地忽略 hash bang 值，例如：

[inline-code-attrs-start title = '忽略 Hash Bang 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

请注意，在进行此更改后，现有评论必须进行迁移。[为此，请联系我们。](https://fastcomments.com/auth/my-account/help)

### URL 查询参数影响小部件

默认情况下，FastComments 会使用页面 URL 作为存储评论的“桶”。如果您的 URL 包含不应成为标识评论线程的标识符一部分的查询参数，我们可以简单地忽略它们，例如：

[inline-code-attrs-start title = '忽略查询参数'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

请注意，在进行此更改后，现有评论必须进行迁移。[为此，请联系我们。](https://fastcomments.com/auth/my-account/help)

### 未收到电子邮件

在 FastComments，我们投入大量工作以确保邮件投递尽可能可靠。然而，一些邮件服务提供商的投递可靠性 notoriously 有问题。请检查您来自 fastcomments.com 的邮件是否被放入了垃圾邮件文件夹。

如果您[联系我们](https://fastcomments.com/auth/my-account/help)，我们通常可以提供更多关于您可能未看到我们邮件的原因的见解。

---