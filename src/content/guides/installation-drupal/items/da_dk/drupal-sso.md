FastComments integreres med Drupals brugersystem via SSO, eller single-sign-on. Dine brugere logger ind på dit Drupal-websted, og modulet videresender deres identitet til FastComments automatisk. Ingen ekstra konti at oprette, ingen indledende synkronisering der skal køres.

Modulet understøtter tre SSO-tilstande, indstilles under `Administration > Configuration > Content > FastComments`.

### Ingen

Ingen SSO. Brugere kommenterer som gæster eller opretter en FastComments-konto. Brug dette, hvis dit websted er offentligt, og du ikke behøver at knytte kommentarer til Drupal-brugere.

### Simpel

Videregiver Drupal-brugerens navn, e-mail og avatar til FastComments uden serverside-verifikation. Intet API Secret nødvendigt. Godt til interne eller lavrisiko-websteder.

### Sikker (anbefalet)

Bruger [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) til at verificere hver brugeridentitet med FastComments. Dette er den tilstand, du skal vælge, når du har konfigureret et API Secret, og det er den eneste tilstand, der forhindrer en besøgende i at udgive sig for en anden bruger.

Brugerens identitet videregives til FastComments hver gang en bruger ser en kommentartråd. Der er ingen indledende eller løbende synkronisering, der skal køre.

<sup>(Valgfrit)</sup> Tilføj dine administratorer til [Brugere & Administratorer](https://fastcomments.com/auth/my-account/users) og moderatorer til [Kommentar-moderatorer](https://fastcomments.com/auth/my-account/moderate-comments/moderators) for at forbedre deres oplevelse og aktivere statistiksporing for moderatorer.

For en dybere gennemgang af, hvordan SSO fungerer, se [SSO-afsnittet](/guide-customizations-and-configuration.html#sso) i tilpasningsdokumentationen.

---