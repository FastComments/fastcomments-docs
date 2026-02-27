Aangezien de inloglinks in wezen wachtwoorden zijn, nemen we de beveiliging zeer serieus.

Alle inloglinks in ons systeem worden ingesteld om na een bepaalde periode te verlopen, en we hebben ook mechanismen om te detecteren
het raden van een inloglink. Sommige inloglinks zijn opgesplitst in meerdere wachtwoorden, en als er één wordt geraden,
zal de andere ongeldig worden gemaakt.

### Beveiliging vergeleken met wachtwoorden

Bij de meeste systemen die een wachtwoord vereisen, kun je via een 'Wachtwoord vergeten'-mechanisme gaan
als je het e-mailadres van de gebruiker hebt. Dit betekent dat, als je toegang hebt tot het e-mailaccount van de gebruiker,
het niet uitmaakt of het systeem dat wordt aangevallen wachtwoorden of magische links gebruikt.

### Nieuwe IP-inlogmeldingen

Wanneer een login plaatsvindt vanaf een IP address dat nog niet eerder is gezien voor een bepaald account, stuurt FastComments een beveiligingswaarschuwingsmail
met de geschatte locatie en het IP address. Dit helpt gebruikers ongeautoriseerde toegang te detecteren. Let op dat FastComments niet
raw IP addresses opslaat — er wordt alleen een obfuscated form opgeslagen voor beveiligingsdoeleinden.

### Beveiliging vergeleken met MFA

Inloglinks zijn minder veilig dan MFA. FastComments ondersteunt nu twee-factor-authenticatie (2FA)
voor beheeraccounts om verbeterde beveiliging te bieden. Wanneer 2FA is ingeschakeld, is het vereist, zelfs bij gebruik van inloglinks.