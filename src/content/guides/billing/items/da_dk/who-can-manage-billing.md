Fakturering er begrænset til to typer brugere:

- `Account Owner`, brugeren som oprettede kontoen.
- Enhver bruger med `Billing Admin`-tilladelsen.

De er de eneste brugere, der kan ændre planen, gemme eller erstatte et kort, åbne `Invoice Settings`, angive Flex-forbrugsgrænser og tilføje kredit. Alle andre, der forsøger, ser `Only the account owner or a billing admin can change the plan or payment method.`

Fakturerings-e-mails, såsom påmindelsen midt i prøveperioden, advarsler om mislykkede betalinger og deaktiveringsmeddelelser, sendes til brugere med `Billing Admin`-tilladelsen. Giv tilladelsen til den, der skal modtage dem, for eksempel en person i økonomiafdelingen, og vedkommende vil modtage e-mails uden at have brug for anden adgang til dashboardet. Påmindelserne, der sendes efter en prøveperiode er udløbet uden faktureringsoplysninger, går til alle dashboard-brugere på kontoen. Hver af disse e-mails har et afmeldingslink; en bruger, der melder sig fra kontoens notifikationer, stopper med at modtage dem.

Fakturaer selv sendes til `Billing Email` i [Invoice Settings](#invoice-settings), som slet ikke behøver at tilhøre en dashboard-bruger.

Siden [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) kræver i stedet `Analytics Admin`-tilladelsen, da den viser forbrug frem for betalingsdetaljer.

For at tildele eller fjerne tilladelsen skal du åbne `Users`-siden og redigere brugeren. Se [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Husk på Flex, at hver ekstra administrator er en faktureret plads, se [Admin and Moderator Seats](#admin-and-moderator-seats).

Hvis du er logget ind og slet ikke kan se `Billing Info`, eller det sender dig tilbage til forsiden, er du logget ind som en kommentator-konto frem for en dashboard-bruger. Log ud og log ind med den konto, der ejer webstedet.