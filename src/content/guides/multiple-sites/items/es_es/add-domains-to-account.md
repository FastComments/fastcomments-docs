FastComments autentica las solicitudes a su cuenta para ver que provienen de su sitio. Por eso necesitamos saber qué sitio o sitios desea instalar FastComments.

FastComments admite la autenticación mediante dominio, así como subdominios.

Tomemos el sitio `https://example.com`. En este caso, "`example.com`" es el dominio. `example.com` admite tanto `example.com` como `www.example.com`. Llamaremos al "www" el "subdominio".

Por ejemplo:

- Para permitir solo `blog.example.com`:
  - Añada `blog.example.com` a sus dominios.
- Para permitir `www.example.com`, `somesite.example.com` y `example.com`:
  - Añada `example.com` a sus dominios.
  - Esto se factura como **un dominio** asociado a su cuenta.
- Ahora puede añadir subdominios comodín, por ejemplo *myname.vercel.app.
  - Esto se factura como **un dominio** asociado a su cuenta.

Si estuviera usando una plataforma de blogs y le asignaran un subdominio, querría añadir el **dominio completo incluido el subdominio** a su cuenta, por ejemplo: `cats.blogger.com`.

Podemos añadir dominios a nuestra cuenta visitando la página `My Domains` y haciendo clic en `Add a Domain` al final:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Página My Domains que muestra los dominios en la cuenta, con el botón Añadir un dominio al final'; title='La página My Domains' app-screenshot-end]

Durante el período de prueba, **los dominios se añaden automáticamente a su cuenta** cuando las solicitudes provienen de dichos dominios. Sin embargo, después de este tiempo deben añadirse explícitamente por motivos de seguridad. Debería recibir un correo electrónico cuando ocurra este comportamiento automatizado.

No tiene que añadir `localhost` para desarrollo local; está permitido por defecto.

#### A través de la API

Los dominios también pueden añadirse y configurarse [a través de la API DomainConfigs](/guide-api.html#domain-config-structure).