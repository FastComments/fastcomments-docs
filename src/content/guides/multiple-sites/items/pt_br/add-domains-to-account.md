FastComments autentica as solicitações para sua conta para verificar se elas estão vindo do seu site. É por isso que
precisamos saber em qual(is) site(s) você quer instalar o FastComments.

FastComments suporta autenticação por domínio, assim como por subdomínios.

Vamos considerar o site `https://example.com`. Neste caso, "`example.com`" é o domínio. `example.com` suporta tanto `example.com`, quanto `www.example.com`. Chamaremos o "www" de "subdomínio".

Por exemplo:

- Para permitir apenas `blog.example.com`:
  - Adicione `blog.example.com` aos seus domínios.
- Para permitir `www.example.com`, `somesite.example.com`, e `example.com`:
  - Adicione `example.com` aos seus domínios.
  - Isso é cobrado como tendo **um domínio** associado à sua conta.
- Você também pode adicionar subdomínios curinga, por exemplo *myname.vercel.app. 
  - Isso é cobrado como tendo **um domínio** associado à sua conta.

Se você estivesse usando uma plataforma de blog e lhe fosse fornecido um subdomínio, você deveria
adicionar o **domínio completo incluindo o subdomínio** à sua conta, por exemplo: `cats.blogger.com`.

Podemos adicionar domínios à nossa conta visitando a página `My Domains` e clicando em `Add a Domain` na parte inferior:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Durante o período de avaliação, **os domínios são adicionados automaticamente à sua conta** quando solicitações vêm desses domínios. Entretanto,
após esse período eles devem ser adicionados explicitamente por segurança. Você deve receber um e-mail quando esse comportamento automatizado ocorrer.

Você **não** precisa adicionar `localhost` para desenvolvimento local - ele é permitido por padrão.

#### Via a API

Os domínios também podem ser adicionados e configurados [via the DomainConfigs API](/guide-api.html#domain-config-structure).