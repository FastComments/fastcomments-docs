En betaling kan fejle, fordi et kort er udløbet, blevet udskiftet, ikke har dækning, eller fordi banken har blokeret beløbet. FastComments giver dig tid til at rette det, før noget holder op med at virke, og fortæller de personer, der kan rette det.

### Hvad der sker

1. **Beløbet fejler.** Fakturaen forbliver `Unpaid` i [Billing History](https://fastcomments.com/auth/my-account/billing-history) med en `Pay Now`-knap. På en fast plan kan Stripe forsøge beløbet igen. På Flex forsøger FastComments enhver ubetalt faktura, der er ældre end fristperioden, igen hver nat.
2. **En advarsel sendes.** Hver nat gennemgår FastComments hver konto. Hvis en Flex-faktura stadig er ubetalt efter fristperioden, og gentagelsen fejler, eller et abonnement på en fast plan er udløbet, fordi det ikke kunne opkræves, får hver bruger med `Billing Admin`-tilladelsen e-mailen `Your FastComments Tenant Will Be Deactivated Soon!` med datoen, og kontoen planlægges til deaktivering én fristperiode senere. Widgetten fungerer fortsat i mellemtiden.
3. **Deaktivering.** Hvis intet er ændret ved den planlagte dato, deaktiveres kontoen: kommentarer skjules på dit websted, og besøgende ser `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Billing-administratorer får e-mail igen. Dine data gemmes.

Fristperioden er én uge for de fleste konti og to uger for konti, der er over et år gamle. På Flex gælder den to gange, én gang før advarslen og én gang mellem advarslen og deaktiveringen, så der er mindst to uger mellem en mislykket opkrævning og skjulning af kommentarer.

### Sådan retter du det

Gem et fungerende kort på [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-siden. Det alene genaktiverer kontoen med det samme og annullerer enhver planlagt deaktivering; den ubetalte faktura opkræves derefter med det nye kort, straks på en fast plan og under den natlige kontrol på Flex. Du kan også betale fakturaen med det samme med `Pay Now` i Billing History.

Hvis kortet på fil er i orden, og fejlen var midlertidig, behøver du ikke gøre noget: den natlige kontrol bemærker den betalte faktura eller det aktive abonnement, annullerer deaktiveringen og sender e-mail til billing-administratorerne som bekræftelse. En allerede deaktiveret konto genaktiveres på samme måde.

### Flere fakturer på én gang

Hvis dit kort var ugyldigt i et par måneder og derefter blev rettet, opkræves hver ubetalte måned separat, så du kan se flere beløb på samme dag. Tjek faktureringsperioden på hver faktura i Billing History: de gælder forskellige måneder, ikke dubletter.

### Ser du stadig en faktureringsadvarsel efter betaling?

Advarslen tilhører en bestemt konto. Hvis du har betalt, men et websted stadig viser `billing information is out of date`, bruger widgetten på det websted sandsynligvis en anden konto end den, du betalte på. Dette sker, når et websted blev oprettet to gange, eller når der er én konto på `fastcomments.com` og en anden på EU-implementeringen på `eu.fastcomments.com`, som faktureres separat. Sammenlign `tenantId` i koden på din side med den under `Settings -> API/SSO`, eller [kontakt support](https://fastcomments.com/auth/my-account/help), så retter vi det.