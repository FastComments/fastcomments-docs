---
FastComments authentifiziert Anfragen an Ihr Konto, um sicherzustellen, dass sie von Ihrer Website stammen. Dies ist der Grund, warum
wir wissen müssen, auf welcher Website bzw. welchen Websites Sie FastComments installieren möchten.

FastComments unterstützt die Authentifizierung über Domains sowie Subdomains.

Nehmen wir die Seite `https://example.com`. In diesem Fall ist `example.com` die Domain. `example.com` unterstützt sowohl `example.com` als auch `www.example.com`. Wir nennen das "www" die "Subdomain".

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

#### Über die API

Domains können auch [über die DomainConfigs API](/guide-api.html#domain-config-structure) hinzugefügt und konfiguriert werden.

---