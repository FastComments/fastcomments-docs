FastComments uwierzytelnia żądania kierowane do Twojego konta, aby sprawdzić, czy pochodzą z Twojej strony. Z tego powodu
musimy wiedzieć, na której stronie, lub stronach, chcesz zainstalować FastComments.

FastComments obsługuje uwierzytelnianie za pomocą domeny, jak również subdomen.

Załóżmy stronę `https://example.com`. W tym przypadku, "`example.com`" jest domeną. `example.com` obsługuje zarówno `example.com`, jak i `www.example.com`. Nazwiemy "www" "subdomeną".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Jeśli używasz platformy blogowej i przydzielono Ci subdomenę, powinieneś dodać **pełną domenę razem z subdomeną** do swojego konta, na przykład: `cats.blogger.com`.

Domeny można dodać do konta, odwiedzając stronę `My Domains` i klikając `Add a Domain` na dole:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

During the trial period, **domains are automatically added to your account** when requests come from said domains. However,
after this time they must be added explicitly for security. You should receive an email when this automated behavior occurs.

You do **not** have to add `localhost` for local development - it is allowed by default.

#### Za pomocą API

Domeny można także dodawać i konfigurować [za pomocą API DomainConfigs](/guide-api.html#domain-config-structure).