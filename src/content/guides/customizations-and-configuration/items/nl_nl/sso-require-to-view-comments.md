FastComments SSO (<a href="#sso">details hier</a>) biedt uw gebruikers een manier om te reageren zonder zich bij een ander platform te hoeven aanmelden.

Echter, dit alleen maakt uw discussies niet veilig, aangezien standaard commentaargegevens openbaar beschikbare informatie zijn - iedereen die de pagina kan bekijken, kan de opmerkingen zien.

Door een instelling te wijzigen, kunnen we beperken dat opmerkingen worden opgehaald, tenzij dit gebeurt door een beheerder of een geldige SSO‑gebruiker.

#### Instelling zonder code

We kunnen het bekijken en interactie met onze discussies voorkomen, wanneer SSO is ingesteld, door een <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">aanpassingsregel</a> te maken.

Zoek bij het uitvoeren hiervan naar SSO, en u zult deze optie vinden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Optie Vereis SSO om opmerkingen te bekijken ingeschakeld in een aanpassingsregel, met de keuze voor beveiligingsniveau'; title='Vereis SSO om opmerkingen te bekijken' app-screenshot-end]

Schakel deze in en sla de aanpassingsregel op.

#### Bescherm alleen een bepaald domein of pagina

Om alleen een bepaald domein of een bepaalde pagina te beschermen, configureren we eenvoudigweg de aanpassingsregel hiervoor.

Bovenaan de aanpassings‑UI vinden we twee invoervelden, Domein en URL‑ID.

Om alleen een specifiek domein te beschermen, voert u het betreffende domein in het veld "domain" in.

Om een specifieke pagina te beschermen, voert u een pagin URL in het veld "URL ID" in. Als u een aangepaste integratie met FastComments heeft, kunt u hier in plaats van een URL een type ID invoeren.

#### Beveiligingsniveaus

Wanneer SSO vereist is, wilt u beslissen of u Simple SSO of Secure SSO vereist. Als u Simple SSO vereist, dan zijn beide toegestaan, maar als u Secure SSO vereist, moet de inhoud worden opgehaald met een Secure SSO payload die is gehasht met uw API‑sleutel om bekeken te kunnen worden.

De optie voor beveiligingsniveau verschijnt wanneer u "Vereis SSO om opmerkingen te bekijken" selecteert.

#### Bescherming verder dan lezen

Het inschakelen van deze optie beschermt de pagina of het domein tegen reacties, tenzij de gebruiker via SSO is aangemeld.

#### Valstrikken

Gebruikers die opmerkingen hebben gemaakt vóór uw SSO‑integratie, kunnen deze niet meer zien, tenzij ze zich aanmelden via uw SSO‑integratie.