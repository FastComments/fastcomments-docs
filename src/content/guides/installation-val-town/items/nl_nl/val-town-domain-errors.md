Once you switch off the `demo` tenant, the widget may refuse to load with an authorization error. This is because FastComments doesn't know it's supposed to allow your account to be used on that domain.

[Ga hierheen om je site aan je account toe te voegen.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town verdient hier een tweede blik, omdat een val via meer dan één hostnaam bereikbaar kan zijn:

- Elke HTTP val heeft een lange standaard endpoint, `<org>--<id>.web.val.run`.
- Het claimen van een aangepast subdomein voegt `<name>.val.run` toe.
- Een [aangepast domein](https://docs.val.town/vals/http/custom-domains/) voegt een derde toe.
- Branches krijgen hun eigen URL's.

Voeg de hostnamen toe waarvan je de widget daadwerkelijk bedient. Als je een subdomein claimt nadat je alles hebt ingesteld, voeg dat dan ook toe, anders werkt de widget op de oude URL en faalt op de nieuwe.