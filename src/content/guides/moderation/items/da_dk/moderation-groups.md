---
Moderatorer kan placeres i grupper for at moderere forskellige sider eller indholdskategorier.

Når en moderator tilhører en eller flere grupper, vil de kun se kommentarer fra de grupper på siden Moderer kommentarer.

For eksempel, lad os antage at vi driver et site, der viser videoer efter kategori. Vi vil måske have forskellige moderatorer for Kat, Hund og Papegøje videoer, så [lad os tilføje de grupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Moderationsgrupper liste med Kat, Hund og Papegøje grupper oprettet for hver videokategori'; title='Moderationsgrupper siden' app-screenshot-end]

Når vi tilføjer en moderator, har vi nu mulighed for at vælge en eller flere grupper, som moderatoren skal tilhøre:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Tilføj en moderator formular med gruppevælgeren, der bruges til at tildele moderator til en eller flere grupper'; title='Tilføjelse af en moderator og valg af en gruppe' app-screenshot-end]

Endelig skal kommentarer knyttes til en eller flere grupper, så de korrekte moderatorer kan se dem.

Dette kan opsættes ved at [tilføje nogle grupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) og derefter angive de tilsvarende `Moderation Group` id'er i kommentarfunktionen,
[såsom beskrevet her](/guide-customizations-and-configuration.html#moderation-group-ids).

---