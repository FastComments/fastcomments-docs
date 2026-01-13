FastComments аутентифицирует запросы к вашей учётной записи, чтобы убедиться, что они поступают с вашего сайта. Именно поэтому нам нужно знать, на каком сайте или сайтах вы хотите установить FastComments.

FastComments поддерживает аутентификацию по домену, а также по поддоменам.

Давайте возьмём сайт `https://example.com`. В этом случае "`example.com`" — это домен. `example.com` поддерживает как `example.com`, так и `www.example.com`. Мы будем называть "www" поддоменом.

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

#### Via The API

Domains can also be added and configured [via the DomainConfigs API](/guide-api.html#domain-config-structure).