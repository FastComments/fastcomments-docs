De pagina [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), bereikbaar via de knop `Invoice Settings` op Billing Info, bepaalt wat op je facturen staat en waar ze naartoe worden gestuurd. Alleen de `Account Owner` of een `Billing Admin` kan deze openen.

- `Company Name`: de naam van je account zoals getoond in het dashboard, e-mails en teamuitnodigingen. Dit is niet de factuurnaam.
- `Billing Email`: waar facturen en factureringsmeldingen worden gestuurd. Stel dit in op een finance- of administratiepostbus als de accounteigenaar ze niet moet ontvangen. Dit wijzigt niet het inlog-e-mailadres van een gebruiker.
- `Invoice Currency`: de valuta waarin je facturen worden uitgegeven. Zie [Currencies](#currencies).
- `Billing Address`: de `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` en `Country` die op facturen worden afgedrukt. Dezelfde velden verschijnen op het Billing Info-formulier wanneer je een kaart opslaat; beide schrijven naar dezelfde plek.
- `VAT / Tax ID`: optioneel. Wordt op je facturen afgedrukt.

Klik op `Save Changes`. De pagina bevestigt met `Invoice settings updated successfully.` en de wijzigingen gelden voor de volgende factuur. Bestaande facturen worden niet gewijzigd.

### VAT And Tax IDs

Selecteer eerst je facturerings-`Country` voordat je het ID invoert, want het land bepaalt welk type ID het is: een EU-btw-nummer voor elke EU-lidstaat, een Brits btw-nummer voor het Verenigd Koninkrijk, een ABN voor Australië, een GST/HST-nummer voor Canada, een EIN voor de Verenigde Staten, enzovoort voor de andere landen die Stripe ondersteunt.

Het ID wordt gevalideerd wanneer je opslaat. Als het niet geldig is voor het geselecteerde land, meldt de pagina dat en wordt het ID niet opgeslagen, terwijl je andere instellingen dat wel worden. Als je land er niet een is waarvoor we een btw-ID kunnen registreren, vertelt de pagina je dat ook.

Als je een EU-factuuradres hebt, worden Flex-facturen uitgegeven met 0% btw als een reverse charge-transactie, en zodra je btw-nummer is opgeslagen, staat het erop, zodat je accountant de btw aan jouw kant kan afhandelen.