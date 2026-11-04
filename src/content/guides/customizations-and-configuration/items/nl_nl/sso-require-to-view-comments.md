FastComments SSO (<a href="#sso">details hier</a>) biedt je gebruikers een manier om te reageren zonder dat ze hoeven in te loggen op een ander platform.

Dit alleen beveiligt je commentaarthreads echter niet, omdat commentaargegevens standaard publiek beschikbare informatie zijn - iedereen die de pagina kan bekijken, kan de reacties bekijken.

Door een instelling te wijzigen, kunnen we voorkomen dat reacties worden opgehaald, tenzij dit door een beheerder of geldige SSO-gebruiker gebeurt.

#### No-Code Setup

We kunnen voorkomen dat onze commentaarthreads worden bekeken en dat er interactie mee plaatsvindt wanneer SSO is ingesteld, door een <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">customization rule</a> aan te maken.

Zoek bij het doen daarvan naar SSO, en je vindt deze optie:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Optie Vereis SSO om reacties te bekijken ingeschakeld in een customization rule, met de keuze voor beveiligingsniveau'; title='Vereis SSO Om Reacties Te Bekijken' app-screenshot-end]

Schakel deze in en sla de customization rule op.

#### Alleen een Bepaald Domein of Pagina Beveiligen

Om alleen een bepaald domein of pagina te beveiligen, configureren we de customization rule eenvoudigweg om dit te doen.

Bovenaan de customization-UI vinden we twee invoervelden: Domain en URL ID.

Om alleen een bepaald domein te beveiligen, voer je het betreffende domein in het veld "domain" in.

Om een bepaalde pagina te beveiligen, voer je een pagina-URL in het veld "URL ID" in. Als je een aangepaste integratie met FastComments hebt, kun je hier in plaats van een URL een soort ID invoeren.

#### Beveiligingsniveaus

Wanneer je SSO vereist, moet je beslissen of je Simple SSO of Secure SSO vereist. Als je Simple SSO vereist, zijn beide toegestaan, maar als je Secure SSO vereist, dan moet de content worden opgehaald met een Secure SSO-payload die is gehasht met je API-sleutel om bekeken te kunnen worden.

De optie voor het beveiligingsniveau verschijnt wanneer je "Require SSO To View Comments" selecteert, en heeft alleen effect terwijl die optie aan staat.

De instellingen <a href="#sso-simple">Auto Verify Comments via Simple SSO en Allow Simple SSO Admins</a> zijn apart, en gelden ongeacht of deze optie aan staat.

#### Bescherming Buiten het Lezen

Het inschakelen van deze optie beschermt de pagina of het domein tegen reacties, tenzij de gebruiker is ingelogd via SSO.

#### Aandachtspunten

Gebruikers die reacties hebben geplaatst vóór je SSO-integratie kunnen deze niet zien, tenzij ze inloggen via je SSO-integratie.