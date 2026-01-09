FastComments は、リクエストがあなたのサイトから来ていることを確認するためにアカウントへのリクエストを認証します。このため
FastComments をインストールしたいサイト（またはサイト群）を把握する必要があります。

FastComments はドメインおよびサブドメインによる認証をサポートします。

サイト `https://example.com` を例に取りましょう。この場合、`example.com` はドメインです。`example.com` は `example.com` と `www.example.com` の両方をサポートします。ここでは "www" を「サブドメイン」と呼びます。

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

#### API 経由

ドメインは [DomainConfigs API 経由](/guide-api.html#domain-config-structure) でも追加・設定できます。

---