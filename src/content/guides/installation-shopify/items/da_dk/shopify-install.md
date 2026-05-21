### Installer fra Shopify App Store

1. Åbn [FastComments-opføringen på Shopify App Store](https://apps.shopify.com/fastcomments).
2. Klik på **Tilføj app** og vælg den plan, du ønsker under installationsforløbet.
3. Shopify omdirigerer dig tilbage til FastComments-administrationen inde i Shopify, når installationen er færdig.

Det er hele installationen. Der er intet, du skal indsætte i dine tema-filer.

### Hvad der bliver opsat for dig

Installationen kører alt det, du ellers ville gøre manuelt:

- En FastComments-tenant oprettes for din butik og linkes til dit shop-domæne.
- Din butiks URL tilføjes tenantens autoriserede domæner, så kommentarer indlæses uden domænefejl.
- Et `fastcomments.tenant_id` shop metafield skrives, så hver blok ved, hvilken tenant den skal rendre imod.
- Single sign-on for dine Shopify-kunder aktiveres som standard.
- Fakturering kører via Shopify Managed Pricing. Gebyrer vises på din almindelige Shopify-regning. Opgrader, nedgrader eller annuller under **Indstillinger > Apps og salgskanaler > FastComments** i din Shopify-admin.

Hvis din butik allerede var FastComments-kunde, før du installerede appen, genbruger installationen den eksisterende tenant i stedet for at oprette en ny.

### Den indlejrede administration

Når du åbner FastComments-appen fra din Shopify-admin, lander du på et dashboard med fliser, der med ét klik åbner hele FastComments-backend'en:

- **Dashboard**: kontoindstillinger, forbrug og abonnementsdetaljer.
- **Moderation Queue**: godkend, afvis og svar på kommentarer på tværs af din butik.
- **Customize**: juster widget-farver, skrifttyper, moderationsregler og konfiguration.
- **Ratings & Reviews Helper**: opsæt stjernebedømmelser og anmeldelsesspørgsmål, hvis du vil bruge Reviews Summary-blokken.

Hver flise åbner FastComments med et engangs-loginlink, så du ikke behøver en separat login.

### Næste: tilføj blokke til din butik

Åbn din Shopify-temaredigerer (**Online Store > Themes > Customize**), åbn den skabelon, du vil tilføje kommentarer eller anmeldelser til, og klik på **Add block**. FastComments-blokkene vises under **Apps**. Resten af denne guide gennemgår hver enkelt.