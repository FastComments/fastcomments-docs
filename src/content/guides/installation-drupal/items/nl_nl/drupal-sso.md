FastComments integreert met het gebruikerssysteem van Drupal via SSO, of single-sign-on. Je gebruikers melden zich aan op je Drupal-site en de module geeft hun identiteit automatisch door aan FastComments. Geen extra accounts om aan te maken, geen initiële synchronisatie die uitgevoerd moet worden.

De module ondersteunt drie SSO-modi, in te stellen onder `Administration > Configuration > Content > FastComments`.

### None

Geen SSO. Gebruikers reageren als gast of maken een FastComments-account aan. Gebruik dit als je site publiek is en je reacties niet aan Drupal-gebruikers hoeft te koppelen.

### Simple

Geeft de naam, het e-mailadres en de avatar van de Drupal-gebruiker door aan FastComments zonder server-side verificatie. Geen API Secret nodig. Geschikt voor interne of laag-risico sites.

### Secure (aanbevolen)

Gebruikt [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) om elke gebruikersidentiteit te verifiëren met FastComments. Dit is de modus die je wilt wanneer je een API Secret hebt geconfigureerd, en het is de enige modus die voorkomt dat een bezoeker zich voordoet als een andere gebruiker.

De gebruikersidentiteit wordt elke keer dat een gebruiker een commentaarthread bekijkt aan FastComments doorgegeven. Er hoeft geen initiële of doorlopende synchronisatie uitgevoerd te worden.

<sup>(Optioneel)</sup> Voeg je beheerders toe aan [Users & Administrators](https://fastcomments.com/auth/my-account/users) en moderatoren aan [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) om hun ervaring te verbeteren en statistiekregistratie voor moderatoren in te schakelen.

Voor een diepere blik op hoe SSO werkt, zie de [SSO-sectie](/guide-customizations-and-configuration.html#sso) van de aanpassingsdocumentatie.