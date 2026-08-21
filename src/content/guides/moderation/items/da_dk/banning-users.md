Der er to måder at bandlyse brugere fra at kommentere på dit site med FastComments.

Den første er, hvis du allerede kender deres e‑mail, kan du indtaste den på <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">bandlyste brugere</a>-siden.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste over bandlyste brugere under Moderer kommentarer, med de bandlyste e-mailadresser og en knap til at tilføje en ny bandlysning'; title='Siden for bandlyste brugere' app-screenshot-end]

Denne side kan tilgås via Moderer kommentarer -> Bandlyste brugere

Når vi skal bandlyse en bruger, kan vi vælge en type, enten Permanent eller Permanent Skyggeban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Ny bandlysningsformular med et e-mailfelt og et valg af bandlystype: Permanent eller Permanent Skyggeban'; title='Bandlysning af en bruger' app-screenshot-end]

Den anden måde at bandlyse en bruger på er ved at klikke på bandlysningsknappen, der er placeret på hver kommentar på siden for kommentar‑moderation.

Når vi klikker på bandlysningsknappen, får du vist nogle muligheder, hvor vi kan angive bandlystypen og varigheden.

### E‑mail‑aliaser

Når du bandlyser en bruger via e‑mail, ignorerer FastComments automatisk `+`‑aliaser. For eksempel vil bandlysning af `user+alias@gmail.com` også bandlyse `user@gmail.com` og enhver anden `+`‑variation af den adresse, såsom `user+other@gmail.com`.

### Skyggebaner

En skyggeban er en type bandlysning, der får det til at fremstå, at brugerens kommentar eller stemme blev gemt korrekt, selvom den i virkeligheden ikke blev det. Dette kan være ønskeligt i visse situationer.

### Bandlysning via IP‑adresse

Medmindre en lejer ønsker at fravælge, understøtter FastComments bandlysning via IP ved at gemme en hash‑version af kommentatorens IP‑adresse.

### Søgning blandt bandlyste brugere

Når din liste vokser ud over en side eller to, kan du indsnævre den med søgelinjen over tabellen.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Søgelinje på siden for bandlyste brugere med en \'Søg efter\' dropdown, en \'Match\' dropdown og et \'Værdi\' input'; title='Søgning blandt bandlyste brugere' app-screenshot-end]

Der er tre kontroller:

- **Search By** vælger, hvilket felt der skal søges i: Any Field, Email, Name, Banned By eller Banned For Saying. De sidste fire svarer til kolonnerne med samme navn i tabellen.
- **Match** vælger, hvordan der skal sammenlignes. **Contains** finder din værdi hvor som helst i feltet, og **Equals** matcher hele feltet.
- **Value** er den tekst, der skal søges efter.

Alle felter matches uden hensyntagen til store/små bogstaver, så søgning efter `SPAMMER@EXAMPLE.COM` finder en bandlysning gemt som `spammer@example.com`.

Et par ting, der er værd at vide:

- **Banned For Saying** søger i teksten af den kommentar, der fik brugeren bandlyst. Sådan finder du alle, der er bandlyst for en bestemt sætning.
- **Banned By** søger i navnet på den moderator, der udstedte bandlysningen, hvilket er nyttigt for at gennemgå en anden moderators beslutninger.
- Wildcard‑bandlysninger gemmes med deres `*`, så en **Contains**‑søgning efter `bademail.com` finder en `*@bademail.com`‑bandlysning.
- **Name** matcher navnet vist i kolonnen Name, så den finder en bruger selvom de har ændret deres navn siden bandlysningen, og selvom du oprettede bandlysningen ved at indtaste en e‑mailadresse og intet navn blev registreret på det tidspunkt. Navnet, der er registreret på bandlysningen, matcher også, så søgning efter enten det gamle eller det aktuelle navn virker.
- **Any Field** søger i e‑mail, navn, banned‑by‑moderator og den bandlyste kommentartekst samlet.

Din søgning er en del af sidens URL, så du kan dele en filtreret liste med andre moderatorer på samme måde som du deler andre moderations‑links. Sideinddeling gennem resultater bevarer søgningen, en ny søgning bringer dig tilbage til første side, og **Clear** vender tilbage til den fulde liste.