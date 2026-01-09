---
Moderatorer kan placeres i grupper for at moderere forskellige sider eller kategorier af indhold.

Når en Moderator tilhører en eller flere grupper, vil de kun se kommentarer fra disse grupper på siden Moderér Kommentarer.

For eksempel, lad os sige, at vi driver et site, der viser videoer efter kategori. Vi vil måske have forskellige moderatorer for katte-, hunde- og papegøjevideoer, så [lad os tilføje de grupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='Siden Moderationsgrupper' app-screenshot-end]

Når vi tilføjer en moderator, har vi nu mulighed for at vælge en eller flere grupper, som moderatoren vil tilhøre:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Tilføjelse af en moderator og valg af en gruppe' app-screenshot-end]

Endelig skal kommentarer knyttes til en eller flere grupper, så de korrekte moderatorer kan se dem.

Dette kan opsættes ved at [tilføje nogle grupper](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) og derefter angive de tilsvarende `Moderation Group` ids i kommentar-widgeten,
[som beskrevet her](/guide-customizations-and-configuration.html#moderation-group-ids).

---