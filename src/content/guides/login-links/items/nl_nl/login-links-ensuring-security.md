Aangezien de inloglinks in wezen wachtwoorden zijn, nemen we de beveiliging zeer serieus.

Alle inloglinks in ons systeem verlopen na een bepaalde tijd, en we hebben ook mechanismen om het raden van een inloglink te detecteren. Sommige inloglinks zijn opgesplitst in meerdere wachtwoorden, en als er één wordt geraden, wordt de andere ongeldig gemaakt.

### Beveiliging vergeleken met wachtwoorden

Bij de meeste systemen die een wachtwoord vereisen, kun je via een 'Wachtwoord vergeten'-mechanisme gaan als je het e‑mailadres van de gebruiker hebt. Dit betekent dat als je toegang hebt tot het e-mailaccount van de gebruiker, het niet uitmaakt of het aangevallen systeem wachtwoorden of inloglinks gebruikt.

### Nieuwe IP-inlogmeldingen

Wanneer er wordt ingelogd vanaf een IP-adres dat nog niet eerder is gezien voor een bepaald account, stuurt FastComments een beveiligingswaarschuwing per e-mail met de geschatte locatie en het IP-adres. Dit helpt gebruikers ongeautoriseerde toegang te detecteren. Let op dat FastComments geen ruwe IP-adressen opslaat — er wordt alleen een geobfusceerde vorm opgeslagen voor beveiligingsdoeleinden.

### Back-up e-mail voor accountherstel

Als je de toegang tot je primaire e-mail verliest, kun je een geverifieerde back-up e-mail gebruiken om je account te herstellen. Je back-up e-mail werkt met alle inlogstromen. Je kunt deze invoeren op de 'gebruikersnaam vergeten'-pagina, gebruiken bij inloggen met een inloglink, of invoeren in het gebruikersnaam/e-mailveld voor wachtwoordinlog.

Om een back-up e-mail in te stellen, ga naar [Account Details](https://fastcomments.com/auth/my-account/edit-details) en klik op **Back-up e-mail definiëren**. Je back-up e-mail wordt alleen gebruikt voor accountherstel en ontvangt geen meldingen.

### Beveiliging vergeleken met MFA

Inloglinks zijn minder veilig dan MFA. FastComments ondersteunt nu tweefactorauthenticatie (2FA) voor beheerdersaccounts om extra beveiliging te bieden. Wanneer 2FA is ingeschakeld, is deze verplicht, zelfs bij gebruik van inloglinks.