Der er to måder at udelukke brugere fra at kommentere på dit site med FastComments.

Den første er, hvis du allerede kender deres e-mail, kan du indtaste den på <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">blokerede brugere</a> siden.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Denne side kan åbnes via Moderér kommentarer -> Blokerede brugere

Når vi udelukker en bruger, kan vi vælge en type: enten Permanent eller Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Den anden måde at udelukke en bruger på er ved at klikke på blokér-knappen, som findes på hver kommentar på Kommentarmoderation-siden.

Når du klikker på blokér-knappen, vises nogle valgmuligheder, hvor du kan angive type og varighed af blokeringen.

### E-mail-aliaser

Når man udelukker en bruger via e-mail, ignorerer FastComments automatisk `+` aliaser. For eksempel vil udelukkelse af `user+alias@gmail.com` også udelukke `user@gmail.com` og enhver anden `+` variation af den adresse, såsom `user+other@gmail.com`.

### Shadow Bans

En shadow-ban er en type udelukkelse, der får det til at se ud som om brugerens kommentar eller stemme blev gemt succesfuldt, mens den i virkeligheden ikke blev det. Dette kan være ønskeligt i visse situationer.

### Udelukkelse via IP-adresse

Medmindre en tenant ønsker at fravælge det, understøtter FastComments blokering via IP ved at gemme en hashet version af kommentatorens IP-adresse.