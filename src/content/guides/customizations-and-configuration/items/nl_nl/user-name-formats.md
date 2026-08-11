---
Standaard toont FastComments de naam van de gebruiker zoals deze is ingevoerd, of zoals deze via SSO aan ons is doorgegeven.

Het kan echter wenselijk zijn om de naam van de gebruiker te maskeren of op een andere manier weer te geven. Bijvoorbeeld, als de naam van de gebruiker Allen Rex is, wil je misschien alleen "Allen R." tonen.

Dit kan zonder code worden gedaan in de Widget Customisatie UI, onder de instelling genaamd `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commentator Naamindeling dropdown geopend met keuzes zoals Capitalize, Last Initial en All Initials'; title='Naamindeling wijzigen' app-screenshot-end]

De beschikbare indelingen zijn:

- Capitalize (toon voorbeeldgebruiker als Example User)
- Last Initial (toon Example User als Example U.)
- All Initials (toon Example User als E. U.)
- Show "Anonymous"

Het effect van het wijzigen hiervan is onmiddellijk. Gebruikers zullen nog steeds hun volledige gebruikersnaam bovenaan het reactiegebied zien, voor zichzelf, maar hun reacties zullen de aangepaste gebruikersnaam tonen.

Gebruikersnamen worden server-side gemaskeerd om gebruikers te beschermen.
---