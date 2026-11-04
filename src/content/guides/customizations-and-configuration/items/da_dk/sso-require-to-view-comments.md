FastComments SSO (<a href="#sso">detaljer her</a>) giver dine brugere en måde at kommentere på uden at skulle logge ind på en anden platform.

Men dette alene sikrer ikke dine kommentarfærdigheder, da kommentardata som standard er offentligt tilgængelige oplysninger – enhver, der kan se siden, kan se kommentarerne.

Ved at ændre en indstilling kan vi begrænse, at kommentarer hentes, medmindre det er af en administrator eller en gyldig SSO-bruger.

#### Opsætning uden kode

Vi kan forhindre visning og interaktion med vores kommentarfærdigheder, når SSO er opsat, ved at oprette en <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">tilpasningsregel</a>.

Når du gør det, så søg efter SSO, og du vil finde denne indstilling:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Kræv SSO for at se kommentarer-indstillingen aktiveret i en tilpasningsregel, med valg af sikkerhedsniveau'; title='Kræv SSO for at se kommentarer' app-screenshot-end]

Aktiver den og gem tilpasningsreglen.

#### Beskyt kun et bestemt domæne eller en side

For kun at beskytte et bestemt domæne eller en side, vil vi blot konfigurere tilpasningsreglen til at gøre det.

Øverst i tilpasnings‑UI’en vil vi finde to inputfelter, Domæne og URL‑ID.

For kun at beskytte et bestemt domæne, indtast det pågældende domæne i feltet "domain".

For at beskytte en bestemt side, indtast en side‑URL i feltet "URL ID". Hvis du har en tilpasset integration med FastComments, kan du indtaste en form for ID her i stedet for en URL.

#### Sikkerhedsniveauer

Når du kræver SSO, vil du beslutte, om du kræver Simple SSO eller Secure SSO. Hvis du kræver Simple SSO, så er begge tilladt, men hvis du kræver Secure SSO, så
skal indholdet hentes med en Secure SSO‑payload, der er hash'et med din API‑nøgle, for at kunne vises.

Sikkerhedsniveau‑indstillingen vises, når du vælger "Require SSO To View Comments", og træder kun i kraft, mens den indstilling er slået til.

De <a href="#sso-simple">Automatisk bekræftelse af kommentarer via Simple SSO og tillad Simple SSO‑administratorer</a> indstillinger er separate og gælder uanset om denne indstilling er slået til eller ej.

#### Beskyttelse ud over læsning

Aktivering af denne indstilling vil beskytte siden eller domænet mod at blive kommenteret på, medmindre brugeren er logget ind via SSO.

#### Faldgruber

Alle brugere, der har oprettet kommentarer før din SSO-integration, vil ikke kunne se dem, medmindre de logger ind via din SSO-integration.