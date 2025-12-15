FastComments authenticates requests to your account to see that they are coming from your site. This is why
we need to know which site, or sites, you want to install FastComments on.

FastComments supports authentication by means of domain, as well as subdomains.

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

#### Via The API

Domains can also be added and configured [via the DomainConfigs API](/guide-api.html#domain-config-structure).
