---
[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

您可能会注意到，评论小部件可以使用 Tenant ID 为 "demo"，例如：

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

这仅用于试用和玩转评论小部件。在生产环境中，您应传入您的 Tenant ID，如下所示：

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

您可以在评论小部件的 <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">账户代码片段</a> 中找到已应用的 Tenant ID。

还可以在 [API 凭证页面](https://fastcomments.com/auth/my-account/api-secret) 找到您的 Tenant ID 并管理您的 API 密钥。

从此之后，如果您已登录 FastComments，代码示例将使用您的真实 Tenant ID（如果您在 https://fastcomments.com 上已登录）。
---