---
对 `localhost` 按照生产环境的相同步骤进行操作。确保已设置生产域名和 API 密钥。

首先，前往 [Webhooks 管理页面](https://fastcomments.com/auth/my-account/manage-data/webhooks)。该页面可通过 “管理数据 -> Webhooks” 访问。

该页面列出您账户中的所有 webhook：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks 管理页面列出每个 webhook 及其 URL、事件、域、方法、状态和排队的事件计数'; title='Webhooks 列表'; cacheBuster = 'v4' app-screenshot-end]

点击 **新建 Webhook** 以添加。每个 webhook 包含一个 URL、一个评论事件（创建、更新或删除）、一个域名和一个 HTTP 方法：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='新建 webhook 表单，包含 URL、事件、域和 HTTP 方法字段以及发送测试负载'; title='新建 Webhook'; cacheBuster = 'v4' app-screenshot-end]

每个 webhook 独立发送。您可以将相同的事件发送到多个端点，并且作用域为 **所有域** 的 webhook 会接收来自所有域的评论，即使同一事件在特定域上已有 webhook。相同的 URL、事件和域名不能重复添加。

保存前，点击 **发送测试负载** 以检查端点是否接受已签名的请求。详情请参见下一节 “测试”。

在列表中，您可以编辑、禁用、重新启用或删除 webhook。禁用后会保留排队的事件，直至重新启用；删除则会丢弃这些事件。

也可以通过 API 创建 webhook，例如使用 Zapier。这些 webhook 会在同一列表中显示，来源标记为 **API**。请参阅通过 API 管理 webhook。

---