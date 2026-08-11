FastComments autentica solicitações à sua conta para garantir que elas vêm do seu site. Por isso,
precisamos saber qual site, ou sites, você deseja instalar o FastComments.

FastComments oferece suporte à autenticação por domínio, bem como por subdomínios.

Vamos usar o site `https://example.com`. Nesse caso, "`example.com`" é o domínio. `example.com` suporta tanto `example.com`, quanto `www.example.com`. Chamaremos o "www" de "subdomínio".

Para exemplo:

- Para permitir apenas `blog.example.com`:
  - Adicione `blog.example.com` aos seus domínios.
- Para permitir `www.example.com`, `somesite.example.com` e `example.com`:
  - Adicione `example.com` aos seus domínios.
  - Isso é cobrado como **um domínio** associado à sua conta.
- Agora você pode adicionar subdomínios curinga, por exemplo *myname.vercel.app.
  - Isso é cobrado como **um domínio** associado à sua conta.

Se você estiver usando uma plataforma de blogs e recebeu um subdomínio, deverá
adicionar o **domínio completo incluindo o subdomínio** à sua conta, por exemplo: `cats.blogger.com`.

Podemos adicionar domínios à nossa conta visitando a página `My Domains` e clicando em `Add a Domain` na parte inferior:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Página My Domains listando os domínios na conta, com o botão Add a Domain na parte inferior'; title='A página My Domains' app-screenshot-end]

Durante o período de teste, **os domínios são adicionados automaticamente à sua conta** quando as solicitações vêm desses domínios. No entanto,
após esse período eles devem ser adicionados explicitamente por questões de segurança. Você receberá um e‑mail quando esse comportamento automatizado ocorrer.

Você **não** precisa adicionar `localhost` para desenvolvimento local – ele é permitido por padrão.

#### Via API

Os domínios também podem ser adicionados e configurados [via the DomainConfigs API](/guide-api.html#domain-config-structure).