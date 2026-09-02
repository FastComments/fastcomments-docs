Het is mogelijk om gebruikers te verbannen die bepaalde e‑mailproviders gebruiken met behulp van wildcards.

Voorbeeld, als je merkt dat alle reacties van **@bademail.com** spam zijn, kun je eenvoudig die hele e‑mailprovider verbannen door "*@bademail.com" in te voeren in het e‑mail invoerveld bij het toevoegen van een verbannen gebruiker.

Let op het "*" vóór de @ in het e‑mailadres.

### Subdomains

Een domeinverbod dekt ook elk subdomein van dat domein. Het verbannen van `*@bademail.com` verbiedt ook `someone@mail.bademail.com` en `someone@eu.mail.bademail.com`, dus er is geen noodzaak om een apart verbod voor elk subdomein toe te voegen.

Als je alleen een specifiek subdomein wilt verbannen, voer dan dat subdomein in, bijvoorbeeld `*@mail.bademail.com`. Dat verbod heeft geen invloed op `someone@bademail.com`.

### Banning a Domain From a Comment

Je hoeft het patroon niet zelf in te typen. Wanneer je een gebruiker verbant vanuit een reactie op de pagina Moderate Comments, bevat het verboddialoogvenster een selectievakje "Ban All @domain Users" dat hetzelfde `*@domain` verbod maakt voor het e‑maildomein van de reageerder.

### Supported Patterns

De enige ondersteunde wildcard‑vorm is een enkel `*` in plaats van het volledige naamgedeelte, gevolgd door `@` en een domein. Andere vormen worden afgewezen wanneer je probeert ze op te slaan:

- `*@*.bademail.com` is niet nodig, omdat `*@bademail.com` al subdomeinen dekt.
- `name*@bademail.com` en `*bademail.com` worden niet ondersteund.

---