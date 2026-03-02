Plugin'et understøtter tre SSO-tilstande til at integrere Moodle-brugerkonti med FastComments.

#### Sikker SSO (Anbefalet)

Brugerdata underskrives på serversiden med HMAC-SHA256 ved hjælp af dit API Secret. Dette er den mest sikre mulighed og anbefales til produktionsbrug.

Med Sikker SSO:

- Brugernavne, e-mails og avatarer sendes sikkert til FastComments.
- Moodle-siteadministratorer synkroniseres automatisk som FastComments-moderatorer.
- Brugere kan ikke udgive sig for andre konti.
- Kræver, at **API Secret** er konfigureret i plugin-indstillingerne.

#### Simpel SSO

Brugerdata (navn, e-mail, avatar) sendes klient-side uden en kryptografisk signatur. Dette er nemmere at sætte op, da det ikke kræver et API Secret, men det er mindre sikkert, fordi brugerdata ikke verificeres på serversiden.

#### Ingen

Ingen SSO-integration. Brugere kommenterer anonymt eller skal logge ind på FastComments separat. Brug dette, hvis du ikke ønsker, at Moodle-brugerkonti skal være knyttet til kommentarer.