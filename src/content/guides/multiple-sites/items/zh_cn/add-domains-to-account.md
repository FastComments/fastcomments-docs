FastComments 对请求进行身份验证，以确认它们来自您的站点。这就是为什么我们需要了解您想要在其上安装 FastComments 的站点或站点集合。

FastComments 支持通过域名以及子域名进行身份验证。

让我们以站点 `https://example.com` 为例。在这种情况下，"`example.com`" 是域名。`example.com` 同时支持 `example.com` 和 `www.example.com`。我们将 “www” 称为 “子域名”。

例如：

- 只允许 `blog.example.com`：
  - 将 `blog.example.com` 添加到您的域名列表中。
- 允许 `www.example.com`、`somesite.example.com` 和 `example.com`：
  - 将 `example.com` 添加到您的域名列表中。
  - 这将计为 **一个域名** 与您的账户关联。
- 您现在可以添加通配符子域名，例如 *myname.vercel.app。
  - 这也计为 **一个域名** 与您的账户关联。

如果您使用的是博客平台，并且获得了一个子域名，您需要将 **包括子域名的完整域名** 添加到您的账户，例如：`cats.blogger.com`。

我们可以通过访问 `My Domains` 页面并点击底部的 `Add a Domain` 来向账户添加域名：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='账户上列出域名的“我的域名”页面，底部有“添加域名”按钮'; title='我的域名页面' app-screenshot-end]

在试用期间，**当请求来自这些域名时，域名会自动添加到您的账户**。然而，试用期结束后，出于安全考虑，必须手动添加。系统在执行此自动行为时会向您发送电子邮件。

您 **不需要** 为本地开发添加 `localhost`——默认已被允许。

#### 通过 API

域名也可以通过 [DomainConfigs API](/guide-api.html#domain-config-structure) 添加和配置。