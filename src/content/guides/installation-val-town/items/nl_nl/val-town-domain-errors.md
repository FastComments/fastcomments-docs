Zodra je de `demo` tenant uitschakelt, kan de widget weigeren te laden met een autorisatiefout. Dit komt omdat FastComments niet weet dat jouw account op dat domein gebruikt mag worden.

[Ga hierheen om je site aan je account toe te voegen.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town verdient hier een tweede blik, omdat een val op meer dan één hostnaam bereikbaar kan zijn:

- Elke HTTP-val heeft een lang standaard endpoint, `<org>--<id>.web.val.run`.
- Het claimen van een aangepast subdomein voegt `<name>.val.run` toe.
- Een [custom domain](https://docs.val.town/vals/http/custom-domains/) voegt een derde toe.
- Branches krijgen hun eigen URL's.

Voeg alle hostnames toe waarvan je de widget daadwerkelijk serveert. Als je na de initiële configuratie een subdomein claimt, voeg die dan ook toe, anders werkt de widget op de oude URL en faalt deze op de nieuwe.