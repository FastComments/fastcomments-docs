FastComments SSO (<a href="#sso">detaljer her</a>) giver dine brugere mulighed for at kommentere uden at skulle logge ind på en anden platform.

Denne funktion alene sikrer dog ikke dine kommentartråde, da kommentardata som standard er offentligt tilgængelig information - enhver, der kan se siden, kan se kommentarerne.

Ved at ændre en indstilling kan vi forhindre, at kommentarer hentes, medmindre det er af en administrator eller en gyldig SSO-bruger.

#### No-Code Setup

Vi kan forhindre visning og interaktion med vores kommentartråde, når SSO er opsat, ved at oprette en <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">tilpasningsregel</a>.

Søg efter SSO, og du vil finde denne mulighed:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Aktivér den og gem tilpasningsreglen.

#### Only Protect a Certain Domain or Page

For kun at beskytte et bestemt domæne eller en bestemt side, konfigurerer vi simpelthen tilpasningsreglen til dette.

Øverst i tilpasnings-UI'et finder vi to inputfelter, Domain og URL ID.

For kun at beskytte et bestemt domæne, indtast det pågældende domæne i feltet "domain".

For at beskytte en bestemt side, indtast en side-URL i feltet "URL ID". Hvis du har en brugerdefineret integration med FastComments, kan du i stedet for en URL indtaste en type ID her.

#### Security Levels

Når du kræver SSO, skal du beslutte, om du kræver Simple SSO eller Secure SSO. Hvis du kræver Simple SSO, så er begge tilladt, men hvis du kræver Secure SSO, skal indholdet hentes med et Secure SSO-payload, der er hashet med din API-nøgle for at kunne blive vist.

Sikkerhedsniveauindstillingen vises, når du vælger "Require SSO To View Comments".

#### Protection Beyond Reading

Aktivering af denne indstilling vil beskytte siden eller domænet fra at blive kommenteret på, medmindre brugeren er logget ind via SSO.

#### Gotchas

Brugere, der oprettede kommentarer før din SSO-integration, vil ikke kunne se dem, medmindre de logger ind via din SSO-integration.