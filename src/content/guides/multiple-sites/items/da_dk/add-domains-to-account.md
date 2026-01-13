FastComments autentificerer forespørgsler til din konto for at sikre, at de kommer fra dit websted. Dette er grunden til, at vi har brug for at vide, hvilket websted, eller hvilke websteder, du ønsker at installere FastComments på.

FastComments understøtter autentificering via domæne samt underdomæner.

Lad os tage sitet `https://example.com`. I dette tilfælde er "`example.com`" domænet. `example.com` understøtter både `example.com` og `www.example.com`. Vi kalder "www" for "underdomænet".

For eksempel:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Hvis du bruger en blogging-platform, og du får et underdomæne, skal du tilføje det **fulde domæne inklusive underdomænet** til din konto, for eksempel: `cats.blogger.com`.

Vi kan tilføje domæner til vores konto ved at besøge siden `My Domains` og klikke `Add a Domain` nederst:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

I prøveperioden bliver **domæner automatisk tilføjet til din konto**, når forespørgsler kommer fra de pågældende domæner. Efter denne tid skal de dog tilføjes eksplicit af sikkerhedsmæssige årsager. Du bør modtage en e-mail, når denne automatiske funktion finder sted.

Du behøver **ikke** at tilføje `localhost` til lokal udvikling - det er tilladt som standard.

#### Via API'en

Domæner kan også tilføjes og konfigureres [via DomainConfigs API](/guide-api.html#domain-config-structure).