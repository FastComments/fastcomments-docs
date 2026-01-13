---
Standaard toont FastComments de naam van de gebruiker zoals deze is ingevoerd, of zoals deze via SSO aan ons is doorgegeven.

Het kan echter wenselijk zijn om de naam van de gebruiker te maskeren of op een andere manier weer te geven. Bijvoorbeeld, als de naam van de gebruiker Allen Rex is, wilt u misschien alleen "Allen R." tonen.

Dit kan zonder code worden gedaan in de Widget Customization UI, onder de instelling genaamd `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

De beschikbare formaten zijn:

- Capitalize (toont Example User als Example User)
- Last Initial (toont Example User als Example U.)
- All Initials (toont Example User als E. U.)
- Show "Anonymous"

De wijziging is onmiddellijk van kracht. Gebruikers zullen hun volledige gebruikersnaam nog steeds boven het opmerkingengebied voor zichzelf zien, maar bij hun opmerkingen wordt de gewijzigde gebruikersnaam weergegeven.

Gebruikersnamen worden aan de serverzijde gemaskeerd om gebruikers te beschermen.

---