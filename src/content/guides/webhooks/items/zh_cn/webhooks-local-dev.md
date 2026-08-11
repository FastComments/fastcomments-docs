对于本地开发，请使用类似 [ngrok](https://ngrok.com/) 的工具。

为了简化系统安全的维护，本地开发遵循与设置和保护其他环境相同的流程。 

### 步骤 1：在您的账户中添加 “localhost” 到域名列表。

在此处将 “localhost” [添加为域名](https://fastcomments.com/auth/my-account/configure-domains)。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='账户设置中添加域名的表单，域名字段已输入 localhost'; title='添加 localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 步骤 2：选择 API 密钥

我们需要为您的域名添加 webhook 配置，因此需要一个 API 密钥。[您可以在此处创建。](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='新建 API 密钥表单，关联域名设置为 localhost，密钥名称为 Testing'; title='添加 Testing API 密钥'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

在 “Associate with domain”（关联到域名）下，选择您的 “localhost” 域名。

**注意：** 另外，您可以为所有测试活动和预发布环境使用同一个 API Secret。只需为 “All Domains”（所有域）添加一个 API Secret，并将其命名为 “test”。  

确保已为您的生产域名定义了 API Secret。其他所有域名的事件将使用通配符（测试）密钥。

### 步骤 3：添加您的 Webhook

在运行 ngrok 或类似工具时，在 [此处](https://fastcomments.com/auth/my-account/manage-data/webhooks) 为 “localhost” 设置值。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhook 管理页面，已选择 localhost 域名，并在评论创建端点中填写了 ngrok URL'; title='添加 Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

点击 `Send Test Payload` 时，我们会发送两个测试事件，以检查您是否验证了 API 密钥。

验证通过后，点击 `Save`。

### 步骤 4：添加评论

现在您可以添加、编辑或删除评论，并应看到我们使用您的测试 API 密钥调用本地开发机器发送事件。事件到达您的机器可能会有最多 30 秒的延迟。