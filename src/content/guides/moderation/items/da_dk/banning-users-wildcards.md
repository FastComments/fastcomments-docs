Det er muligt at blokere brugere fra bestemte e‑mailudbydere ved hjælp af jokertegn.

For eksempel, hvis du opdager at alle kommentarer fra **@bademail.com** er spam, kan du blot blokere hele e‑mailudbyderen ved at indtaste "*@bademail.com" i e‑mail‑inputfeltet, når du tilføjer en blokeret bruger.

Bemærk "*" før @ i e‑mailen.

### Subdomains

Et domæneforbud dækker også alle underdomæner til det pågældende domæne. At blokere `*@bademail.com` blokerer også `someone@mail.bademail.com` og `someone@eu.mail.bademail.com`, så der er ingen grund til at tilføje et separat forbud for hvert underdomæne.

Hvis du kun vil blokere et specifikt underdomæne, indtast da det underdomæne i stedet, for eksempel `*@mail.bademail.com`. Dette forbud påvirker ikke `someone@bademail.com`.

### Banning a Domain From a Comment

Du behøver ikke at skrive mønsteret selv. Når du blokerer en bruger fra en kommentar på siden Moderate Comments, har forbudsdialogen en afkrydsningsboks "Ban All @domain Users", som opretter det samme `*@domain` forbud for kommentatorens e‑mail‑domæne.

### Supported Patterns

Den eneste understøttede jokertegnsform er et enkelt `*` i stedet for hele navnedelen, efterfulgt af `@` og et domæne. Andre former afvises, når du forsøger at gemme dem:

- `*@*.bademail.com` er ikke nødvendigt, fordi `*@bademail.com` allerede dækker underdomæner.
- `name*@bademail.com` og `*bademail.com` understøttes ikke.