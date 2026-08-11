FastComments SSO (<a href="#sso">detaljer her</a>) giver dine brugere en måde at kommentere på uden at skulle logge ind på en anden platform.

Dette alene sikrer dog ikke dine kommentarfærdigheder, da kommentardata som standard er offentligt tilgængelige – enhver, der kan se siden, kan også se kommentarerne.

Ved at ændre en indstilling kan vi begrænse, at kommentarer hentes, medmindre det er af en administrator eller en gyldig SSO‑bruger.

#### No-Code Setup

Vi kan forhindre visning og interaktion med vores kommentarfærdigheder, når SSO er opsat, ved at oprette en <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">tilpasningsregel</a>.

Når du gør det, så søg efter SSO, og du vil finde denne mulighed:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Kræv SSO for at se kommentarer-indstillingen aktiveret i en tilpasningsregel, med valg af sikkerhedsniveau'; title='Kræv SSO for at se kommentarer' app-screenshot-end]

Aktivér den og gem tilpasningsreglen.

#### Only Protect a Certain Domain or Page

For kun at beskytte et bestemt domæne eller en side, konfigurerer vi blot tilpasningsreglen til at gøre det.

Øverst i tilpasnings‑UI’en finder vi to inputfelter, Domæne og URL‑ID.

For kun at beskytte et specifikt domæne, indtast det pågældende domæne i feltet “domain”.

For at beskytte en specifik side, indtast en side‑URL i feltet “URL ID”. Hvis du har en brugerdefineret integration med FastComments, kan du i stedet indtaste en type ID her i stedet for en URL.

#### Security Levels

Når du kræver SSO, vil du gerne beslutte, om du kræver Simple SSO eller Secure SSO. Hvis du kræver Simple SSO, er begge tilladt, men hvis du kræver Secure SSO, skal indholdet hentes med en Secure SSO‑payload, der er hash‑et med din API‑nøgle, for at kunne vises.

Muligheden for sikkerhedsniveau vises, når du vælger “Require SSO To View Comments”.

#### Protection Beyond Reading

Aktivering af denne mulighed vil beskytte siden eller domænet mod at blive kommenteret på, medmindre brugeren er logget ind via SSO.

#### Gotchas

Alle brugere, der har oprettet kommentarer før din SSO‑integration, vil ikke kunne se dem, medmindre de logger ind via din SSO‑integration.