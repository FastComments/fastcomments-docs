FastComments integreert met het gebruikerssysteem van Drupal via SSO, of single-sign-on. Uw gebruikers melden zich aan op uw Drupal-site, en de module geeft hun identiteit automatisch door aan FastComments. Geen extra accounts aan te maken, geen initiële synchronisatie uit te voeren.

De module ondersteunt drie SSO-modi, ingesteld onder `Administration > Configuration > Content > FastComments`.

### None

Geen SSO. Gebruikers plaatsen opmerkingen als gasten of maken een FastComments-account aan. Gebruik dit als uw site openbaar is en u reacties niet aan Drupal-gebruikers hoeft te koppelen.

### Simple

Geeft de naam, het e-mailadres en de avatar van de Drupal-gebruiker door aan FastComments zonder verificatie aan de serverzijde. Er is geen API Secret nodig. Goed voor interne of laag-risico sites.

### Secure (recommended)

Gebruikt [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) om elke gebruikersidentiteit te verifiëren met FastComments. Dit is de modus die u wilt gebruiken wanneer u een API Secret hebt geconfigureerd, en het is de enige modus die voorkomt dat een bezoeker zich voordoet als een andere gebruiker.

De gebruikersidentiteit wordt aan FastComments doorgegeven telkens wanneer een gebruiker een discussiedraad bekijkt. Er is geen initiële of continue synchronisatie die uitgevoerd moet worden.

<sup>(Optioneel)</sup> Voeg uw beheerders toe aan [Users & Administrators](https://fastcomments.com/auth/my-account/users) en moderators aan [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) om hun ervaring te verbeteren en het bijhouden van statistieken voor moderators mogelijk te maken.

Voor een diepere blik op hoe SSO werkt, zie de [SSO section](/guide-customizations-and-configuration.html#sso) van de customisatiedocumentatie.