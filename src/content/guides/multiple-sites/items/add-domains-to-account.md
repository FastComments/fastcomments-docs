All requests to load the comment widget are authenticated to see that they are coming from your site. This is why
FastComments needs to know which site, or sites, you want to install FastComments on.

FastComments supports authentication by means of domain, as well as subdomains.

Let's take the site `https://example.com`. In this case, "`example.com`" is the domain. `example.com` supports both `example.com`, and `www.example.com`. We'll call the "www" the "subdomain".

If this site were to be a FastComments customer, they would add "example.com" to their account,
as this would support "example.com" as well as all subdomains (www, and others).

However, if you were using a blogging platform, and you were given a subdomain, you would want
to add the **full domain including the subdomain** to your account, for example: `cats.blogger.com`.

We can add domains to our account by visiting the "My Domains page" and clicking "Add a Domain" at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

During the trial period, **domains are automatically added to your account** when requests come from said domains. However,
after this time they must be added explicitly for security.

#### Via The API

Domains can also be added and configured [via the DomainConfigs API](/guide-api.html#domain-config-structure).
