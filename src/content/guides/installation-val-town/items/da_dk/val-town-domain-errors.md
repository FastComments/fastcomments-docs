Når du slår `demo`-tenanten fra, kan widgetten nægte at indlæse med en autorisationsfejl. Det skyldes, at FastComments ikke ved, at din konto skal have lov til at blive brugt på det domæne.

[Gå hertil for at tilføje dit site til din konto.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town er et andet blik værd her, fordi en val kan være tilgængelig på mere end ét hostname:

- Hver HTTP-val har et langt standardendpoint, `<org>--<id>.web.val.run`.
- At gøre krav på et brugerdefineret subdomæne tilføjer `<name>.val.run`.
- Et [brugerdefineret domæne](https://docs.val.town/vals/http/custom-domains/) tilføjer et tredje.
- Branches får deres egne URL'er.

Tilføj de hostnames, du faktisk serverer widgetten fra. Hvis du gør krav på et subdomæne, efter du har sat tingene op, så tilføj det også, ellers virker widgetten på den gamle URL og fejler på den nye.