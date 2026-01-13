FastComments fungerer med websteder kun for medlemmer ved at bruge det, der kaldes SSO, eller single-sign-on. Dine medlemmer logger ind på dit WordPress-site, men behøver ikke at bekymre sig om at oprette en konto hos FastComments eller logge ind med sociale medier for at kunne kommentere. Hvis dine medlemmer (inklusive administratorer) er logget ind på dit WordPress-site, vil de kunne kommentere. Dine administratorer og moderatorer vil også kunne moderere kommentarer direkte fra dine WordPress-blogindlæg.

<sup>(Valgfrit)</sup> Husk også at tilføje dine administratorer til [Brugere & administratorer](https://fastcomments.com/auth/my-account/users) og moderatorer til [Kommentarmoderatorer](https://fastcomments.com/auth/my-account/moderate-comments/moderators) for at forbedre deres oplevelse og aktivere statistiksporing for moderatorer.

SSO kan aktiveres ved at gå til plugin-dashboardet og klikke på "SSO-indstillinger".

Du skal først aktivere funktionen "Alle kan registrere sig" på dit websted.

Alle brugeroplysninger overføres problemfrit og sikkert til FastComments hver gang en bruger ser en kommentartråd via [HMAC](https://en.wikipedia.org/wiki/HMAC).

Der er ingen initial eller kontinuerlig synkronisering, der skal køres for at kopiere dine medlemmer over til FastComments-serverne. Dette gøres automatisk, når de ser kommentartråde ved at åbne et blogindlæg.

## Navne og Avatarer

Plugin'et opdaterer automatisk brugerens visningsnavn og avatar på alle deres kommentarer i FastComments, når de ser enhver kommentartråd. Avatarer understøttes via Gravatar eller ethvert avatar-håndteringsplugin i WordPress, da plugin'et vil kalde `get_avatar_url`.

---