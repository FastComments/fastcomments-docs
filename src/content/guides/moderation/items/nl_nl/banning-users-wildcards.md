Het is mogelijk om gebruikers te blokkeren die bepaalde e-mailproviders gebruiken via wildcards.

Als je bijvoorbeeld merkt dat alle reacties van **@bademail.com** spam zijn, kun je die hele e-mailprovider blokkeren door "*@bademail.com" in te vullen in het e-mailinvoerveld bij het toevoegen van een geblokkeerde gebruiker.

Let op de "*" vóór de @ in het e-mailadres.

### Subdomeinen

Een domeinblokkade geldt ook voor elk subdomein van dat domein. Het blokkeren van `*@bademail.com` blokkeert ook
`someone@mail.bademail.com` en `someone@eu.mail.bademail.com`, dus het is niet nodig om voor elk subdomein een aparte blokkade toe te voegen.

Als je alleen een specifiek subdomein wilt blokkeren, vul dan dat subdomein in, bijvoorbeeld `*@mail.bademail.com`. Die blokkade
heeft geen invloed op `someone@bademail.com`.

### Een domein blokkeren vanuit een reactie

Je hoeft het patroon niet zelf in te typen. Wanneer je een gebruiker blokkeert vanuit een reactie op de pagina Moderate Comments, bevat het
blokkeervenster een selectievakje "Ban All @domain Users" dat dezelfde `*@domain`-blokkade aanmaakt voor het e-maildomein van de reactiegever.

### Ondersteunde patronen

De enige ondersteunde wildcardvorm is een enkele `*` in plaats van het volledige naamgedeelte, gevolgd door `@` en een domein. Andere vormen
worden geweigerd wanneer je ze probeert op te slaan:

- `*@*.bademail.com` is niet nodig, omdat `*@bademail.com` al subdomeinen dekt.
- `name*@bademail.com` en `*bademail.com` worden niet ondersteund.