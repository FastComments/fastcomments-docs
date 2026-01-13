---
FastComments аутентифицирует запросы к вашей учётной записи, чтобы убедиться, что они приходят с вашего сайта. Это причина, по которой
нам нужно знать, на каком сайте или сайтах вы хотите установить FastComments.

FastComments поддерживает аутентификацию по домену, а также по поддоменам.

Возьмём сайт `https://example.com`. В этом случае, "`example.com`" — это домен. `example.com` поддерживает как `example.com`, так и `www.example.com`. Мы будем называть "www" «поддоменом».

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Если вы используете платформу для блогов и вам выдали поддомен, вам следует
добавить **полный домен, включая поддомен**, в вашу учётную запись, например: `cats.blogger.com`.

Мы можем добавить домены в нашу учётную запись, перейдя на страницу `My Domains` и нажав `Add a Domain` внизу:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

В течение пробного периода **домены автоматически добавляются в вашу учётную запись**, когда запросы приходят с указанных доменов. Однако,
после этого они должны быть добавлены явно в целях безопасности. Вы должны получить электронное письмо, когда это автоматическое действие произойдёт.

Вам **не** нужно добавлять `localhost` для локальной разработки — он разрешён по умолчанию.

#### Via The API

Домены также можно добавлять и настраивать [via the DomainConfigs API](/guide-api.html#domain-config-structure).

---