FastComments 会验证发送到您账户的请求，以确认这些请求来自您的网站。这就是为什么
我们需要知道您要在哪个网站（或哪些网站）上安装 FastComments。

FastComments 支持基于域名以及子域名的认证。

Let's take the site `https://example.com`. In this case, "`example.com`" is the domain. `example.com` supports both `example.com`, and `www.example.com`. We'll call the "www" the "subdomain".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

If you were using a blogging platform, and you were given a subdomain, you would want
to add the **full domain including the subdomain** to your account, for example: `cats.blogger.com`.

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

During the trial period, **domains are automatically added to your account** when requests come from said domains. However,
after this time they must be added explicitly for security. You should receive an email when this automated behavior occurs.

You do **not** have to add `localhost` for local development - it is allowed by default.

#### 通过 API

Domains can also be added and configured [通过 DomainConfigs API](/guide-api.html#domain-config-structure).