#### Navigeer naar LTI 1.3-configuratie

Meld u aan bij FastComments en ga naar <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uw LTI 1.3-configuratiepagina</a>.

Als uw account nog geen LTI-toegang heeft, ziet u "LTI not enabled for this account" - neem contact op met support om het op uw plan te activeren.

#### Kies een platform (optioneel)

Onder **Dynamische registratie-URL genereren**, gebruikt u de vervolgkeuzelijst **Platform** om FastComments te vertellen met welk LMS u verbinding maakt:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Andere LTI 1.3-platform

U kunt het ook op **Automatisch detecteren** laten staan. Het platform wordt tijdens registratie uit de openid-configuration van uw LMS gelezen; de vervolgkeuzelijst vult alleen het weergavelabel voor de resulterende configuratie.

#### URL genereren

Klik op **Genereer URL**. FastComments maakt een eenmalige registratietoken aan en toont u een URL die er als volgt uitziet:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopieer deze. Deze URL:

- Is **eenmalig** - zodra uw LMS er met succes een oproep naar doet, wordt het registratietoken verbruikt.
- Vervalt na **30 minuten** als het niet wordt gebruikt.
- Moet privé worden gehouden - iedereen met de URL kan binnen die 30 minuten een tool registreren voor uw tenant.

#### Bestaande configuraties

Zodra een registratie succesvol is voltooid, verschijnt de nieuwe configuratie in de tabel **Bestaande configuraties** op dezelfde pagina, met Platform, Issuer, Client ID en Status. U kunt configuraties uit deze tabel verwijderen als u ze ooit wilt de-registreren.