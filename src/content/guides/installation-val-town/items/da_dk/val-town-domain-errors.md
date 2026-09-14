Når du slår `demo`-lejeren fra, kan widget'en nægte at indlæses med en autorisationsfejl. Dette skyldes, at FastComments ikke ved, at den skal tillade, at din konto bruges på det domæne.

[Go here to add your site to your account.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town er værd at kigge på igen her, fordi en val kan være tilgængelig på mere end ét værtsnavn:

- Hver HTTP-val har en lang standardendpoint, `<org>--<id>.web.val.run`.
- At gøre krav på et brugerdefineret subdomæne tilføjer `<name>.val.run`.
- Et [custom domain](https://docs.val.town/vals/http/custom-domains/) tilføjer et tredje.
- Grenene får deres egne URL'er.

Tilføj de værtsnavne, du faktisk leverer widget'en fra. Hvis du gør krav på et subdomæne efter opsætningen, skal du også tilføje det, ellers fungerer widget'en på den gamle URL og fejler på den nye.