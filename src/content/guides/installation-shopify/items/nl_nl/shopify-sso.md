De **FastComments**-blok ondersteunt single sign-on, zodat je Shopify-klanten als zichzelf kunnen reageren zonder een apart FastComments-account te maken.

### Hoe het werkt

When a visitor who is logged into your store opens a page with the **FastComments** block:

1. The block detects the Shopify `customer` object.
2. It sends the customer's name and email to FastComments through a signed app proxy request.
3. FastComments creates or matches a user keyed by `shopify-{customerId}`, so the same customer always maps to the same FastComments user across sessions and re-installs.
4. The visitor's name shows up on their comments. They are not prompted to log in again.

Als de bezoeker niet is ingelogd in de winkel, valt het blok terug op anonieme reacties (of de FastComments-aanmeldingsflow, afhankelijk van je widgetconfiguratie).

### SSO uitschakelen

SSO staat standaard aan voor elk **FastComments**-blok. Om het uit te schakelen voor een specifiek blok:

1. Open de thema-editor van Shopify.
2. Open de template die het blok bevat en klik op het blok om het te selecteren.
3. Haal het vinkje weg bij **SSO**.
4. Klik op **Opslaan**.

Schakel SSO uit als je wilt dat reageerders een aparte identiteit kiezen voor het gesprek. Bijvoorbeeld een interne communitypagina waar medewerkers onder een andere weergavenaam reageren.

### Wat FastComments ontvangt

The SSO payload sent for each customer contains:

- Een gebruikers-ID afgeleid van de Shopify-klant-ID (`shopify-{customerId}`).
- Het e-mailadres van de klant (gebruikt om de gebruiker te identificeren; niet openbaar weergegeven).
- De weergavenaam van de klant (gebruikt als naam van de auteur bij opmerkingen).

Er worden geen bestelgeschiedenis-, betalings- of adresgegevens verzonden. De payload wordt server-side ondertekend; de browser van de klant ziet nooit de inloggegevens.

### Aanmeld- en afmeldlinks

When SSO is on, the comment widget's sign-in and sign-out links point at `/account/login` and `/account/logout`, the standard Shopify customer account routes. There is nothing to configure. The links work for any store with customer accounts enabled.