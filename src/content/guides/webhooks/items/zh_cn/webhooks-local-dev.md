对于本地开发，请使用类似 [ngrok](https://ngrok.com/) 的工具。

为了简化系统安全管理，本地开发遵循与设置和保护其他环境相同的流程。 

### 第 1 步：将 "localhost" 添加到你账户的域名中。

将 "localhost" 添加为域名，操作请见 [此处](https://fastcomments.com/auth/my-account/configure-domains)。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 第 2 步：选择一个 API Key

我们将为你的域名添加 webhook 配置，因此需要一个 API key。你可以在 [此处](https://fastcomments.com/auth/my-account/api-secret) 创建。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

在 “Associate with domain” 下 - 选择你的 “localhost” 域名。

**注意：或者，你可以为所有测试活动和预发布环境使用一个 API Secret。只需为 "All Domains" 添加一个 API Secret，并给它起名为 "test"。**

确保为你的生产域定义了 API Secret。其他所有域的事件将使用通配（测试）密钥。

### 第 3 步：添加你的 Webhook

在运行 ngrok 或类似工具时，在 [此处](https://fastcomments.com/auth/my-account/manage-data/webhooks) 为 “localhost” 设置值。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

当点击 `Send Test Payload` 时，我们会发送两个测试事件来检查你是否验证了 API key。

验证通过后，点击 `Save`。

### 第 4 步：添加一条评论

现在你可以添加、编辑或删除评论，并应看到我们使用你的测试 API key 将事件发送到本地开发机器。事件到达你的机器可能会有最多 30 秒的延迟。

---