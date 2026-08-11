Der er to måder at bandlyse brugere fra at kommentere på dit site med FastComments.

Den første er, hvis du allerede kender deres e‑mail, kan du indtaste den på siden <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">bandlyste brugere</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste over bandlyste brugere under Moderer Kommentarer, med de bandlyste e-mailadresser og en knap til at tilføje en ny ban'; title='Siden for bandlyste brugere' app-screenshot-end]

Denne side kan tilgås via Moderer Kommentarer -> Bandlyste Brugere

Når vi skal bandlyse en bruger, kan vi vælge en type, enten Permanent eller Permanent Skyggeban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Ny ban-formular med et e-mailfelt og et valg af ban-typen Permanent eller Permanent Skyggeban'; title='Bandlysning af en bruger' app-screenshot-end]

Den anden måde at bandlyse en bruger på er ved at klikke på bandlysningsknappen, som er placeret på hver kommentar på siden Kommentar Moderation.

Når vi klikker på bandlysningsknappen, får du vist nogle muligheder, hvor vi kan angive bandlystypen og varigheden.

### E‑mail‑aliaser

Når du bandlyser en bruger via e‑mail, ignorerer FastComments automatisk `+`‑aliaser. For eksempel vil bandlysning af `user+alias@gmail.com` også bandlyse `user@gmail.com` og enhver anden `+`‑variation af den adresse, såsom `user+other@gmail.com`.

### Skyggebaner

En skyggeban er en type ban, der får det til at se ud som om brugerens kommentar eller stemme blev gemt succesfuldt, selvom den i virkeligheden ikke blev det. Dette kan være ønskeligt i visse situationer.

### Bandlysning via IP‑adresse

Medmindre en lejer ønsker at fravælge, understøtter FastComments bandlysning via IP ved at gemme en hash‑version af kommentatorens IP‑adresse.