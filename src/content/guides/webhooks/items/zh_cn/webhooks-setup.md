对 `localhost` 采取与生产环境相同的步骤。请确保已设置生产域名和 API Secrets。

首先，导航到 [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks)。可通过 管理数据 -> Webhooks 访问。

配置页面如下所示：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

在此页面，您可以为每种评论事件指定端点。

针对每种事件，请务必点击 发送测试负载 (Send Test Payload) 以确保您已正确设置集成。有关详细信息，请参阅下一节 “测试”。