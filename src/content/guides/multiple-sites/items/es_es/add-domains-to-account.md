FastComments autentica las solicitudes a su cuenta para verificar que provienen de su sitio. Por eso necesitamos saber en qué sitio, o sitios, desea instalar FastComments.

FastComments admite la autenticación por dominio, así como por subdominios.

Tomemos el sitio `https://example.com`. En este caso, "`example.com`" es el dominio. `example.com` admite tanto `example.com` como `www.example.com`. Llamaremos "www" el "subdominio".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Si estuviera usando una plataforma de blogs y le hubieran asignado un subdominio, querría añadir el **dominio completo incluyendo el subdominio** a su cuenta, por ejemplo: `cats.blogger.com`.

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Durante el período de prueba, **los dominios se añaden automáticamente a su cuenta** cuando las solicitudes provienen de dichos dominios. Sin embargo, después de este periodo deben añadirse explícitamente por razones de seguridad. Debería recibir un correo electrónico cuando ocurra este comportamiento automatizado.

You do **not** have to add `localhost` for local development - it is allowed by default.

#### A través de la API

Domains can also be added and configured [via the DomainConfigs API](/guide-api.html#domain-config-structure).