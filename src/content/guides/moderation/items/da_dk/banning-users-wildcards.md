Det er muligt at bande brugere fra bestemte e-mail-udbydere ved hjælp af wildcards.

For eksempel, hvis du opdager, at alle kommentarer fra **@bademail.com** er spam, kan du simpelthen bande
hele den e-mail-udbyder ved at indtaste "*@bademail.com" i e-mail-inputfeltet, når du tilføjer en bandlyst bruger.

Bemærk "*" før @ i e-mailen.

### Subdomæner

En domænebannelse dækker også alle subdomæner af det domæne. At bande `*@bademail.com` bandler også
`someone@mail.bademail.com` og `someone@eu.mail.bademail.com`, så der er ikke behov for at tilføje en separat bannelse for hvert subdomæne.

Hvis du kun vil bande et bestemt subdomæne, skal du indtaste det subdomæne i stedet, for eksempel `*@mail.bademail.com`. Den bannelse
påvirker ikke `someone@bademail.com`.

### Bannelse af et domæne fra en kommentar

Du behøver ikke selv at skrive mønsteret. Når du bandler en bruger fra en kommentar på siden Moderate Comments, har bannelsesdialogen
et afkrydsningsfelt "Ban All @domain Users", der opretter den samme `*@domain` bannelse for kommentarens e-mail-domæne.

### Understøttede mønstre

Den eneste understøttede wildcard-form er en enkelt `*` i stedet for hele navnedelen, efterfulgt af `@` og et domæne. Andre former
afvises, når du forsøger at gemme dem:

- `*@*.bademail.com` er ikke nødvendigt, fordi `*@bademail.com` allerede dækker subdomæner.
- `name*@bademail.com` og `*bademail.com` understøttes ikke.