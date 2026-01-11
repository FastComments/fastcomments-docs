对 `localhost` 的操作与在生产环境中相同。确保您已设置生产域名和 API Secrets。

首先，导航到 [Webhooks 管理](https://fastcomments.com/auth/my-account/manage-data/webhooks)。这是通过 管理数据 -> Webhooks 访问的。

配置页面如下所示：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

在此页面中，您可以为每种评论事件指定端点。

对于每种事件类型，请务必点击 Send Test Payload 以确保您的集成已正确设置。有关详细信息，请参阅下一节“测试”。

---