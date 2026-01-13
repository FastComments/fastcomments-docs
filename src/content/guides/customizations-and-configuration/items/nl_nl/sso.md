SSO, of single sign-on, is een reeks conventies waarmee jij of je gebruikers FastComments kunnen gebruiken zonder een extra account te hoeven aanmaken.

Als je geen anonieme reacties toestaat, is een account vereist om met FastComments te reageren. We maken dit aanmeldproces erg eenvoudig - de gebruiker laat gewoon zijn e-mail achter wanneer hij reageert.
We begrijpen echter dat zelfs dat extra frictie kan zijn die sommige sites willen vermijden.

We kunnen die frictie verminderen door slechts één inlogstroom voor je gehele site te gebruiken.

### How do I get it?
Alle accounttypen krijgen momenteel toegang tot SSO. Het maximale aantal SSO-gebruikers varieert echter afhankelijk van je pakket. Zoals bij andere functies bieden de Pro-plannen en hoger directe ontwikkelaarsondersteuning.

Laten we de opties vergelijken en daarna de details van elk bespreken.

### User and Comment Migrations

Bij migratie vanaf een platform met SSO zoals Disqus heb je al gebruikers en hun reacties.

Reacties worden geïmporteerd als onderdeel van je migratie, hetzij via de API, onze Import UI, of via de klantenservice. De Import UI heeft de voorkeur als het het platform ondersteunt waarvan je migreert, omdat het foutafhandeling, avatar- en media-extractie en uploads, en een batchjob-monitoringssysteem bevat.

De gebruikers zelf worden automatisch toegevoegd bij het voor het eerst bekijken van reactiedraden. Als alternatief kunnen ze vooraf worden toegevoegd via de API, maar dit werk heeft niet veel voordelen.

Als reacties worden geïmporteerd en SSO-gebruikers niet handmatig via de API worden toegevoegd, dan worden reacties automatisch gemigreerd naar het account van de gebruiker de eerste keer dat dat account wordt aangemaakt wanneer ze een reactiedraad bekijken. Ze kunnen dan de reacties die ze oorspronkelijk hebben geschreven beheren, bewerken en verwijderen.

De automatische migratie gebeurt via e-mailadres of gebruikersnaam. Sommige platforms geven bij export geen e-mails, zoals Disqus, dus in dat geval vallen we terug op de gebruikersnaam.
- Zolang je een overeenkomende gebruikersnaam doorgeeft, en een e-mail in de SSO-payload, voegen we de e-mail toe aan de individuele reactie-objecten zodat meldingen en vermeldingen werken.

Als je je reacties en gebruikers tegelijk wilt importeren, werk dan samen met support om de reacties naar de respectieve accounts van de gebruikers te migreren nadat gebruikers via de API zijn geïmporteerd.

Samengevat is het gemakkelijkste pad voor de migratie:

1. Importeer reacties.
   1. Avatars en andere media worden automatisch gemigreerd als je de Import UI gebruikt in `Manage Data -> Imports`.
2. Stel Secure of Simple SSO in.
3. Laat de migratie per gebruiker automatisch plaatsvinden wanneer zij voor het eerst inloggen.
   1. Dit voegt meestal minder dan een seconde toe aan de laadtijd van de pagina als de gebruiker minder dan 50k reacties heeft.

### WordPress Users
Als je onze <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress-plugin</a> gebruikt, hoef je geen code te schrijven! Ga gewoon naar de adminpagina van de plugin, klik op SSO-instellingen en schakel het in.

Dit brengt je naar een wizard met één klik die je API-sleutel aanmaakt, deze naar je WordPress-installatie stuurt en SSO inschakelt. We hebben dit voor je samengevoegd tot één knop.

Let op: als je de plugin voor de eerste keer installeert, moet je het installatieproces doorlopen voordat je de adminpagina met de knop SSO-instellingen ziet.

#### WordPress SSO - Moderators

Let erop dat op dit moment, om het "Moderator"-badge naast je moderators te tonen wanneer ze reageren met de FastComments WordPress-plugin,
ze ook als Moderator moeten zijn toegevoegd in het FastComments-dashboard en hun e-mailadres geverifieerd moet zijn.

### Custom Integrations

Voor aangepaste integraties zijn er twee opties.

### Option One - Secure SSO

Met Secure SSO weet FastComments dat de gebruiker die reageert, stemt en reacties leest een echte gebruiker op jouw site is.

Zolang je een geldig payload aanmaakt, zal de gebruiker altijd een naadloze ervaring bij het reageren hebben.

Bij Secure SSO wordt de SSO-payload **server-side** aangemaakt met HMAC-authenticatie en vervolgens doorgegeven aan de widget op de **client**.

Bij Secure SSO is het account van de gebruiker **volledig gescheiden** van de rest van de FastComments-gebruikersbasis. Dit betekent dat als we twee partners hebben
Bedrijf A en Bedrijf B, elk een SSO-gebruiker met de gebruikersnaam "Bob" kunnen hebben.

#### Requirements
- Enige basiskennis van backend-ontwikkeling.
- Enige basiskennis van het omgaan met geheime API-sleutels.
- Enige basiskennis van API-ontwikkeling of server-side rendering.

#### Pros
- Veilig.
- Naadloze ervaring bij het reageren.

#### Cons
- Vereist backend-ontwikkeling.

#### Updating User Data

Bij Secure SSO zullen we elke keer dat je de SSO-gebruikerspayload doorgeeft, hun gebruiker bijwerken met de nieuwste informatie. Bijvoorbeeld, als
de gebruiker de gebruikersnaam `X` heeft, en je geeft `Y` door in de SSO-payload, zal hun gebruikersnaam `Y` worden.

Als je waarden wilt verwijderen met deze aanpak, stel ze dan in op `null` (niet `undefined`).

#### Secure SSO API

We bieden ook een API voor interactie met de SSO-gebruikers. Zie [the docs](/guide-api.html#sso-user-structure).

Let op dat bij gebruik van Secure SSO gebruikers automatisch achter de schermen worden aangemaakt bij het laden van de pagina. Je hoeft je gebruikers niet in bulk te importeren.

### Option Two - Simple SSO

Het alternatief voor Secure SSO is simpelweg de gebruikersinformatie naar de reactie-widget te sturen.

Het doorgeven van een e-mail met Simple SSO is niet vereist, maar zonder deze zal hun reactie als "Unverified" worden weergegeven.

<sup>Opmerking!</sup> Vanaf begin 2022 hoeven gebruikersnamen bij Simple SSO niet uniek te zijn over heel FastComments.com.

Idealiter kies je Simple SSO alleen bij ontwikkeling op een platform dat geen backend-toegang biedt.

#### Requirements
- Enige basiskennis van client-side ontwikkeling.
- Je moet ten minste het e-mailadres van de gebruiker kennen.

#### Pros
- Eenvoudig.
- Alle activiteit wordt nog steeds geverifieerd.
- De gebruiker voert nooit zijn/haar gebruikersnaam of e-mailadres in.

#### Cons
- Minder veilig dan Secure SSO omdat de client-side payload geconstrueerd zou kunnen worden om zich als elke gebruiker voor te doen.

#### Simple SSO API

Gebruikers die automatisch worden aangemaakt via de Simple SSO-stroom worden opgeslagen als `SSOUser`-objecten. Ze kunnen worden benaderd en beheerd via de `SSOUser` API. Zie [the docs](/guide-api.html#sso-user-structure).