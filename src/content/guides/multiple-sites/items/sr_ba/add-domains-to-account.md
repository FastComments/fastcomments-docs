FastComments аутентификује захтјеве према вашем налогу да би утврдио да долазе са вашег сајта. Због тога нам треба знати на који(е) сајт(ове) желите инсталирати FastComments.

FastComments подржава аутентификацију по домену, као и по поддоменима.

Let's take the site `https://example.com`. In this case, "`example.com`" is the domain. `example.com` supports both `example.com`, and `www.example.com`. We'll call the "www" the "subdomain".

На пример:

- Да бисте дозволили само `blog.example.com`:
  - Додајте `blog.example.com` у своје домене.
- Да бисте дозволили `www.example.com`, `somesite.example.com`, и `example.com`:
  - Додајте `example.com` у своје домене.
  - Ово се наплаћује као **један домен** повезан са вашим налогом.
- Сада можете додати wildcard поддомене, на примјер *myname.vercel.app. 
  - Ово се наплаћује као **један домен** повезан са вашим налогом.

If you were using a blogging platform, and you were given a subdomain, you would want
to add the **full domain including the subdomain** to your account, for example: `cats.blogger.com`.

Домене можемо додати на наш налог посјетом страници `My Domains` и кликом на `Add a Domain` при дну:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Током пробног периода, **домене се аутоматски додају на ваш налог** када захтјеви долазе са тих домена. Међутим,
након тог периода морају бити експлицитно додани ради сигурности. Требало би да примите е-пошту када се ова аутоматска појава догоди.

Не морате додавати `localhost` за локални развој - дозвољен је подразумјевано.

#### Преко API-ја

Домене такође можете додати и конфигурисати [преко DomainConfigs API](/guide-api.html#domain-config-structure).