---
FastComments godkender anmodninger til din konto for at sikre, at de kommer fra dit websted. Derfor har vi brug for at vide, hvilket websted eller hvilke websteder du vil installere FastComments på.

FastComments understøtter godkendelse via domæne såvel som subdomæner.

Lad os tage webstedet `https://example.com`. I dette tilfælde er "`example.com`" domænet. `example.com` understøtter både `example.com` og `www.example.com`. Vi vil kalde "www" for "subdomænet".

For eksempel:

- For kun at tillade `blog.example.com`:
  - Tilføj `blog.example.com` til dine domæner.
- For at tillade `www.example.com`, `somesite.example.com` og `example.com`:
  - Tilføj `example.com` til dine domæner.
  - Dette faktureres som **ét domæne** tilknyttet din konto.
- Du kan nu tilføje wildcard-subdomæner, for eksempel *myname.vercel.app.
  - Dette faktureres som **ét domæne** tilknyttet din konto.

Hvis du brugte en blogplatform, og du fik tildelt et subdomæne, vil du gerne tilføje **det fulde domæne inklusive subdomænet** til din konto, for eksempel: `cats.blogger.com`.

Vi kan tilføje domæner til vores konto ved at besøge siden `My Domains` og klikke på `Add a Domain` nederst:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='My Domains-side, der viser domænerne på kontoen, med knappen Add a Domain nederst'; title='My Domains-siden' app-screenshot-end]

I prøveperioden **tilføjes domæner automatisk til din konto**, når anmodninger kommer fra de pågældende domæner. Efter denne periode skal de dog tilføjes eksplicit af sikkerhedsmæssige årsager. Du vil modtage en e‑mail, når denne automatiske handling forekommer.

Du behøver **ikke** at tilføje `localhost` for lokal udvikling – det er tilladt som standard.

#### Via API'en

Domæner kan også tilføjes og konfigureres [via DomainConfigs API](/guide-api.html#domain-config-structure).

---