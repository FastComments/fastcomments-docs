对于本地开发，可以使用像 [ngrok](https://ngrok.com/) 这样的工具。

为了简化保持系统安全的流程，本地开发遵循与设置和保护其他环境相同的流程。 

### 第 1 步：将 "localhost" 添加到您帐户中的域名。

在 [此处](https://fastcomments.com/auth/my-account/configure-domains) 将 "localhost" 添加为域名。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 第 2 步：选择一个 API 密钥

我们将为您的域添加 webhook 配置，因此需要一个 API 密钥。您可以在 [这里](https://fastcomments.com/auth/my-account/api-secret) 完成该操作。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

在 "Associate with domain" 下 - 选择您的 "localhost" 域。

**注意：或者，您可以为所有测试活动和预发布（staging）环境使用一个 API Secret。只需为 "All Domains" 添加一个 API Secret，并给它一个类似 "test" 的名称。**

确保您已为生产域定义了 API Secret。所有其他域的事件将使用通配符（测试）密钥。

### 第 3 步：添加您的 Webhook

在运行 ngrok 或类似工具时，在 [此处](https://fastcomments.com/auth/my-account/manage-data/webhooks) 为 "localhost" 设置值。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

当点击 `Send Test Payload` 时，我们会发送两个测试事件以检查您是否验证了 API 密钥。

一旦验证通过，点击 `Save`。

### 第 4 步：添加评论

现在您可以添加、编辑或删除评论，并应该会看到我们使用您的测试 API 密钥通过事件调用您的本地开发机器。事件可能最多有 30 秒的延迟
才能到达您的机器。