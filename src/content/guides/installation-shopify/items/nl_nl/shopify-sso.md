Het **FastComments**-blok ondersteunt single sign-on, zodat je Shopify-klanten kunnen reageren als zichzelf zonder een apart FastComments-account aan te maken.

### Hoe het werkt

Wanneer een bezoeker die is ingelogd op je winkel een pagina opent met het **FastComments**-blok:

1. Het blok detecteert het Shopify `customer`-object.
2. Het stuurt de naam en het e-mailadres van de klant naar FastComments via een ondertekend app proxy-verzoek.
3. FastComments maakt een gebruiker aan of koppelt deze op basis van `shopify-{customerId}`, zodat dezelfde klant altijd wordt gemapt naar dezelfde FastComments-gebruiker, over sessies en herinstallaties heen.
4. De naam van de bezoeker verschijnt bij diens reacties. Er wordt niet opnieuw gevraagd in te loggen.

Als de bezoeker niet is ingelogd op de winkel, valt het blok terug op anoniem reageren (of de FastComments-aanmeldflow, afhankelijk van je widgetconfiguratie).

### SSO uitschakelen

SSO staat standaard aan voor elk **FastComments**-blok. Om het uit te schakelen voor een specifiek blok:

1. Open de Shopify theme editor.
2. Open de template die het blok bevat en klik op het blok om het te selecteren.
3. Vink **SSO** uit.
4. Klik op **Save**.

Schakel SSO uit als je wilt dat commenters een aparte identiteit kiezen voor het gesprek. Bijvoorbeeld een interne community-pagina waar medewerkers reageren onder een andere weergavenaam.

### Wat FastComments ontvangt

De SSO-payload die per klant wordt verzonden bevat:

- Een gebruikers-ID afgeleid van het Shopify-klant-ID (`shopify-{customerId}`).
- Het e-mailadres van de klant (gebruikt om de gebruiker te identificeren; niet openbaar weergegeven).
- De weergavenaam van de klant (gebruikt als naam van de reactie-auteur).

Er worden geen orderhistorie-, betalings- of adresgegevens verzonden. De payload wordt server-side ondertekend; de browser van de klant ziet nooit een credential.

### Inlog- en uitloglinks

Wanneer SSO is ingeschakeld, verwijzen de inlog- en uitloglinks van de reactiewidget naar `/account/login` en `/account/logout`, de standaard Shopify-klantenaccountroutes. Er hoeft niets geconfigureerd te worden. De links werken voor elke winkel met klantaccounts ingeschakeld.