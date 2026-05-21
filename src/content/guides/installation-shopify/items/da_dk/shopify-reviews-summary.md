The **FastComments - Anmeldelsesoversigt** blok viser en samlet stjernerating og opdeling af anmeldelser for en side. Kombiner den med **FastComments**-blokken på produktskabeloner for den standard reviews-layout: oversigt øverst, anmeldelsesformular og anmeldelser nedenfor.

### Forudsætning: opsæt Vurderinger & Anmeldelser

Anmeldelsesoversigten viser de vurderingsspørgsmål, du har konfigureret for din butik. Opsæt dem først:

1. Åbn FastComments-appen i din Shopify-administrator.
2. Klik på flisen **Ratings & Reviews Helper** (eller åbn [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) direkte).
3. Tilføj de spørgsmål, du ønsker, at hver anmelder skal besvare (samlet stjernerating, "hvordan var pasformen", osv.).

Uden konfigurerede spørgsmål har oversigtsblokken intet at aggregere.

### Tilføj blokken

1. Åbn Shopify-temaeditoren.
2. Åbn skabelonen **Produkt** (eller den sideskabelon, hvor du vil have oversigten).
3. Klik på **Tilføj blok** nær toppen af sidesektionen, over hvor **FastComments**-blokken vil være.
4. Under **Apps** vælger du **FastComments - Reviews Summary**.
5. Tilføj en **FastComments**-blok længere nede på den samme side, hvis du ikke allerede har en, så besøgende kan efterlade anmeldelser.
6. Klik på **Gem**.

### Indstillinger

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Overskriv hvilken FastComments-tenant oversigten læser fra. Lad stå tom for at bruge butikkens automatisk-konfigurerede tenant. | (tom) |
| Custom URL ID | Overskriv sideidentifikatoren, som oversigten aggregerer imod. Brug dette, når oversigten ligger på en anden side end den FastComments-blok, den skal afspejle. | (automatisk registreret) |

### Hvordan oversigten matcher anmeldelserne

Anmeldelsesoversigten bruger samme autoudtækningslogik som **FastComments**-blokken:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

For en normal produktside deler oversigten og kommentartråden automatisk et URL-ID, uden behov for konfiguration.

### Tips

- Oversigten er skrivebeskyttet. For at indsamle anmeldelser skal du have en **FastComments**-blok på samme side.
- Hvis du ændrer vurderingsspørgsmål i Ratings & Reviews Helper efter at have indsamlet anmeldelser, genberegner oversigten i forhold til det nye sæt spørgsmål.