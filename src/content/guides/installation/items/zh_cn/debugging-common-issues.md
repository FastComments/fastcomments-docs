以下是我们经常遇到的一些症状和常见解决方案。

### "This is a demo"消息

当您从我们使用演示租户的主页复制小部件代码时会显示此消息。要使用您的租户，请从[这里](https://fastcomments.com/auth/my-account/get-acct-code)复制小部件代码。

### "FastComments cannot load on this domain"错误

FastComments需要知道哪些域名属于您，以验证与您账户相关的请求。[查看我们的文档](/guide-multiple-sites.html#add-domains-to-account)了解如何解决此错误（只需将确切的子域名+域名添加到您的账户）。

请注意，这应该只在试用期结束后发生。在试用期内，来自新域名的任何请求都会自动添加到您的账户。

### 自定义安装中迁移的评论未显示

通常这发生在导入的评论与`Page ID`关联，而您传递的是URL（或未传递值，在这种情况下默认使用页面URL）时。

您可以通过[导出评论](https://fastcomments.com/auth/my-account/manage-data/export)并查看`URL ID`列（目前是列`B`）来调试此问题。

确保您在`URL ID`列中看到的值与您传递给小部件配置的`urlId`参数的值相同。

有关更多说明，请阅读我们的[评论如何与页面和文章关联的文档](/guide-customizations-and-configuration.html#url-id)。

如果都失败了，[请联系我们](https://fastcomments.com/auth/my-account/help)。

### 评论小部件未显示

如果评论小部件未显示，请检查Chrome开发者控制台是否有错误。

对于大多数错误配置，评论小部件至少会在页面上显示错误（如果它能够加载）。什么都看不到通常表示有脚本错误。

### 所需配置未按预期工作

尝试我们的[Chrome扩展程序](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)查看传递给评论小部件的配置。如果都失败了，截取Chrome扩展程序显示的内容并[联系我们](https://fastcomments.com/auth/my-account/help)。

### 具有不同哈希标记的相同URL上的评论缺失

默认情况下，FastComments将使用页面URL作为存储评论的"桶"。如果您的URL包含`#hashbangs`，而这些`#hashbangs`不应该是标识评论线程的标识符的一部分，我们可以简单地忽略哈希标记值，例如：

[inline-code-attrs-start title = '忽略哈希标记示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

请注意，进行此更改后，需要对现有评论进行迁移。[为此，请联系我们。](https://fastcomments.com/auth/my-account/help)

### URL查询参数影响小部件

默认情况下，FastComments将使用页面URL作为存储评论的"桶"。如果您的URL包含不应该是标识评论线程的标识符一部分的查询参数，我们可以简单地忽略它们，例如：

[inline-code-attrs-start title = '忽略查询参数'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

请注意，进行此更改后，需要对现有评论进行迁移。[为此，请联系我们。](https://fastcomments.com/auth/my-account/help)

### 未收到电子邮件

在FastComments，我们投入大量工作以确保电子邮件传递尽可能可靠。但是，一些电子邮件提供商以难以可靠传递而闻名。请检查您的垃圾邮件文件夹中是否有来自fastcomments.com的邮件。

如果您[联系我们](https://fastcomments.com/auth/my-account/help)，我们通常可以提供更多关于您为什么可能没有收到我们电子邮件的信息。
