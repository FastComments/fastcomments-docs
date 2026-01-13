---
为使该插件正常工作，您的 WordPress 数据库和您的 FastComments 帐户中都会保存一个令牌。当插件向我们的服务器发出请求时，它会提供
该令牌。

您可以在[此处](https://fastcomments.com/auth/my-account/manage-data/integrations)查看所有已授权到您 FastComments 帐户的集成。

所有通信均通过 HTTPS 进行。

所有通信均为从您的 WordPress 服务器*发出*到 FastComments.com，包括作为通过
[轮询](https://en.wikipedia.org/wiki/Polling_(computer_science)) 从您 WordPress 安装中的 [cron](https://developer.wordpress.org/plugins/cron/) 设置实现的*回传*到您 WordPress 安装的同步。

---