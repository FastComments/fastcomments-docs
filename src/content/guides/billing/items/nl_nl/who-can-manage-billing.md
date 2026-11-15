Facturatie is beperkt tot twee soorten gebruikers:

- De `Account Owner`, de gebruiker die het account heeft aangemaakt.
- Elke gebruiker met de `Billing Admin` permissie.

Zij zijn de enige gebruikers die het plan kunnen wijzigen, een kaart kunnen opslaan of vervangen, `Invoice Settings` kunnen openen, Flex spend limits kunnen instellen en tegoed kunnen toevoegen. Iedereen anders die het probeert, ziet `Only the account owner or a billing admin can change the plan or payment method.`

Facturatie-e-mails, zoals de herinnering halverwege de proefperiode, waarschuwingen bij mislukte betalingen en deactiveringsmeldingen, worden verzonden naar gebruikers met de `Billing Admin` permissie. Geef de permissie aan wie de e-mails moet ontvangen, bijvoorbeeld iemand van de financiële afdeling, en deze persoon ontvangt de e-mails zonder verdere toegang tot het dashboard nodig te hebben. De herinneringen die worden verzonden nadat een proefperiode is geëindigd zonder factuurgegevens, gaan naar elke dashboardgebruiker op het account. Elk van deze e-mails bevat een uitschrijflink; een gebruiker die zich afmeldt voor accountmeldingen ontvangt ze niet meer.

Facturen zelf gaan naar de `Billing Email` in [Invoice Settings](#invoice-settings), die helemaal niet toe hoeft te behoren aan een dashboardgebruiker.

De [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) pagina vereist in plaats daarvan de `Analytics Admin` permissie, omdat het gebruik toont in plaats van betalingsgegevens.

Om de permissie te verlenen of te verwijderen, open je de `Users` pagina en bewerk je de gebruiker. Zie [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Houd bij Flex rekening met het feit dat elke extra admin een gefactureerde seat is, zie [Admin and Moderator Seats](#admin-and-moderator-seats).

Als je bent ingelogd en `Billing Info` helemaal niet ziet, of het stuurt je terug naar de startpagina, dan ben je ingelogd met een commenter-account in plaats van een dashboardgebruiker. Meld je af en meld je aan met het account dat eigenaar is van de site.