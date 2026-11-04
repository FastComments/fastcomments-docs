SSO, of single-sign-on, is een reeks conventies waarmee jij of je gebruikers FastComments kunnen gebruiken zonder nog een account aan te maken.

Ervan uitgaande dat je anoniem reageren niet toestaat, is een account vereist om te reageren met FastComments. We maken dit registratieproces heel eenvoudig - de gebruiker laat alleen zijn e-mailadres achter bij het reageren.
We begrijpen echter dat zelfs dat extra wrijving is die sommige sites willen vermijden.

We kunnen die wrijving verminderen door slechts één inlogflow voor je hele site te hebben.

### Hoe krijg ik het?
Alle accounttypes hebben momenteel toegang tot SSO. Het maximale aantal SSO-gebruikers varieert echter afhankelijk van je pakket. Zoals bij andere functies, bieden de Pro-plannen en hoger directe ontwikkelondersteuning.

Laten we de opties vergelijken en vervolgens op de details van elke optie ingaan.

### Gebruikers- en Commentaarmigraties

Bij het migreren van een platform met SSO zoals Disqus heb je al gebruikers en hun reacties.

Reacties worden geïmporteerd als onderdeel van je migratie, via de API, onze Import UI of klantenservice. De Import UI heeft de voorkeur als deze het platform ondersteunt waarvan je
migreert, omdat deze foutafhandeling, avatar- en media-extractie en uploads, en een batchtaak-monitoringsysteem bevat.

De gebruikers zelf worden automatisch toegevoegd wanneer ze voor het eerst een commentaarthread bekijken. Als alternatief kunnen ze vooraf worden toegevoegd via de API, maar dit werk heeft niet
veel voordelen.

Als reacties worden geïmporteerd en SSO-gebruikers niet handmatig via de API worden toegevoegd, dan worden reacties automatisch gemigreerd naar het account van de gebruiker de eerste
keer dat het wordt aangemaakt wanneer ze een commentaarthread bekijken. Ze kunnen vervolgens de reacties die ze oorspronkelijk hebben geschreven beheren, bewerken en verwijderen.

De automatische migratie gebeurt via e-mail of gebruikersnaam. Sommige platforms verstrekken geen e-mailadressen bij export, zoals Disqus, dus vallen we in dat geval terug op de gebruikersnaam.
- Zolang je een overeenkomende gebruikersnaam en een e-mailadres in de SSO-payload doorgeeft, voegen we het e-mailadres toe aan de individuele commentaarobjecten zodat meldingen en vermeldingen werken.

Als het gewenst is om je reacties en gebruikers tegelijk te importeren, werk dan samen met support om de reacties over te zetten naar de respectieve accounts van de gebruikers nadat de gebruikers zijn geïmporteerd
via de API.

Samenvattend is het makkelijkste pad voor de migratie dus:

1. Importeer reacties.
   1. Avatars en andere media worden automatisch gemigreerd als je de Import UI gebruikt in `Manage Data -> Imports`.
2. Stel Secure of Simple SSO in.
3. Laat de migratie per gebruiker automatisch gebeuren wanneer ze voor het eerst inloggen.
   1. Dit voegt meestal minder dan een seconde toe aan de laadtijd van de pagina als de gebruiker minder dan 50k reacties heeft.

### WordPress-gebruikers
Als je onze <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress-plugin</a> gebruikt, hoef je geen code te schrijven! Ga gewoon naar de Admin-pagina van de plugin, klik op SSO Settings en vervolgens op Enable.

Dit brengt je naar een wizard met één knop die je API-sleutel aanmaakt, deze naar je WordPress-installatie stuurt en SSO inschakelt. We hebben dit voor je geconsolideerd tot één klik op een knop.

Houd er rekening mee dat als je de plugin voor het eerst installeert, je eerst het installatieproces moet voltooien voordat je de admin-pagina met de SSO Settings-knop ziet.

#### WordPress SSO - Moderators

Houd er rekening mee dat momenteel, om het "Moderator"-badge naast je moderators te tonen wanneer ze reageren met de FastComments WordPress-plugin,
ze ook als Moderator moeten worden toegevoegd in het FastComments-dashboard en hun e-mailadres geverifieerd moet zijn.

### Aangepaste integraties

Voor aangepaste integraties zijn er twee opties.

### Optie één - Secure SSO

Met Secure SSO weet FastComments dat de gebruiker die reageert, stemt en reacties leest een echte gebruiker op je site is.

Zolang je een geldige payload aanmaakt, heeft de gebruiker altijd een naadloze commentaarervaring.

Met Secure SSO wordt de SSO-payload **server-side** aangemaakt met HMAC-authenticatie en vervolgens doorgegeven aan de widget op de **client**.

Met Secure SSO is het account van de gebruiker **volledig gescheiden** van de rest van de FastComments-gebruikersbasis. Dit betekent dat als we twee partners hebben,
Bedrijf A en Bedrijf B, elk een SSO-gebruiker kan hebben met de gebruikersnaam "Bob".

#### Vereisten
- Enige basiskennis van backend-ontwikkeling.
- Enige basiskennis van het omgaan met geheime API-sleutels.
- Enige basiskennis van API-ontwikkeling of server-side rendering.

#### Voordelen
- Veilig.
- Naadloze commentaarervaring.

#### Nadelen
- Vereist backend-ontwikkeling.

#### Gebruikersgegevens bijwerken

Met Secure SSO werken we bij elke keer dat je de sso-gebruikerspayload doorgeeft de gebruiker bij met de nieuwste informatie. Bijvoorbeeld, als
de gebruiker een gebruikersnaam `X` heeft en je `Y` doorgeeft in de SSO-payload, wordt de gebruikersnaam `Y`.

Als je waarden wilt verwijderen met deze aanpak, stel ze dan in op `null` (niet `undefined`).

#### Secure SSO API

We bieden ook een API voor interactie met de SSO-gebruikers. Zie [de documentatie](/guide-api.html#sso-user-structure).

Houd er rekening mee dat bij het gebruik van Secure SSO gebruikers automatisch op de achtergrond worden aangemaakt bij het laden van de pagina. Je hoeft je gebruikers niet bulksgewijs te importeren.

### Optie twee - Simple SSO

Het alternatief voor Secure SSO is om simpelweg de gebruikersinformatie door te geven aan de commentaarwidget.

Het verstrekken van een e-mailadres bij Simple SSO is niet vereist, maar zonder dit worden hun reacties getoond als "Unverified".

<sup>Let op!</sup> Vanaf begin 2022 hoeven gebruikersnamen met Simple SSO niet uniek te zijn binnen heel FastComments.com.

Idealiter zou Simple SSO alleen gekozen moeten worden bij het ontwikkelen op een platform dat geen backend-toegang biedt.

#### Vereisten
- Enige basiskennis van client-side ontwikkeling.
- Je moet ten minste het e-mailadres van de gebruiker weten.

#### Voordelen
- Eenvoudig.
- Alle activiteit wordt nog steeds geverifieerd.
- De gebruiker voert nooit zijn gebruikersnaam of e-mailadres in.

#### Nadelen
- Minder veilig dan Secure SSO, omdat de client-side payload vervalst zou kunnen worden om elke gebruiker te worden.

#### Simple SSO API

Gebruikers die automatisch worden aangemaakt via de Simple SSO-flow worden opgeslagen als `SSOUser`-objecten. Ze kunnen worden geraadpleegd en beheerd via de `SSOUser` API. Zie [de documentatie](/guide-api.html#sso-user-structure).