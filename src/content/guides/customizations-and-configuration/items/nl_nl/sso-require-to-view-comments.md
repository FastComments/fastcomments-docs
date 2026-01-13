FastComments SSO (<a href="#sso">meer informatie</a>) biedt uw gebruikers een manier om te reageren zonder in te hoeven loggen op een ander platform.

Dit alleen beveiligt echter uw reactiedraden niet, aangezien standaard reactiedata openbare informatie is - iedereen die de pagina kan bekijken, kan de reacties zien.

Door een instelling te wijzigen, kunnen we voorkomen dat reacties worden opgehaald, tenzij dit gebeurt door een beheerder of een geldige SSO-gebruiker.

#### No-Code Setup

We kunnen het bekijken en interactie hebben met onze reactiedraden voorkomen, wanneer SSO is ingesteld, door een <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">aanpassingsregel</a> te maken.

Wanneer u dat doet, zoek naar SSO, en u zult deze optie vinden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Schakel deze in en sla de aanpassingsregel op.

#### Only Protect a Certain Domain or Page

Om alleen een bepaald domein of pagina te beschermen, configureren we simpelweg de aanpassingsregel daarvoor.

Bovenaan de aanpassings-UI vinden we twee invoervelden, Domain en URL ID.

Om alleen een specifiek domein te beschermen, voert u het betreffende domein in het veld "domain" in.

Om een specifieke pagina te beschermen, voert u een pagina-URL in het veld "URL ID" in. Als u een custom integratie met FastComments heeft, kunt u hier in plaats van een URL een type ID invoeren.

#### Security Levels

Wanneer u SSO verplicht stelt, moet u beslissen of u Simple SSO of Secure SSO vereist. Als u Simple SSO vereist, zijn beide toegestaan, maar als u Secure SSO vereist, moet de content worden opgehaald met een Secure SSO-payload die met uw API key is gehasht om bekeken te kunnen worden.

De optie voor het beveiligingsniveau zal verschijnen wanneer u "Require SSO To View Comments" selecteert.

#### Protection Beyond Reading

Het inschakelen van deze optie beschermt de pagina of het domein tegen het plaatsen van reacties, tenzij de gebruiker is ingelogd via SSO.

#### Gotchas

Gebruikers die reacties hebben gemaakt voordat uw SSO-integratie actief was, kunnen deze niet zien, tenzij ze inloggen via uw SSO-integratie.