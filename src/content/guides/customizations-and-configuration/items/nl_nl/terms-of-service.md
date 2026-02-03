FastComments stelt je in staat om gebruikers die voor het eerst reageren te verplichten je Servicevoorwaarden te accepteren voordat ze een opmerking indienen.

Wanneer ingeschakeld:
- **Anonieme gebruikers** zien elke keer dat ze reageren een selectievakje voor de Servicevoorwaarden
- **Geauthenticeerde gebruikers** zien het selectievakje alleen bij hun eerste reactie, of wanneer je je Servicevoorwaarden bijwerkt

### Servicevoorwaarden inschakelen

Ga naar de pagina voor het aanpassen van de widget en schakel het selectievakje "Require Terms of Service acceptance" in:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### De tekst van de Servicevoorwaarden aanpassen

Standaard toont het selectievakje "I agree to the Terms of Service and Privacy Policy" met links naar beide documenten. Je kunt deze tekst per locale aanpassen indien nodig:

1. Selecteer "Customize text per locale"
2. Selecteer de locale in de vervolgkeuzelijst en voer je aangepaste tekst in

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Je Servicevoorwaarden bijwerken

Wanneer je je Servicevoorwaarden bijwerkt, stel je de datum "Last Updated" in. Gebruikers die de Servicevoorwaarden vóór deze datum hebben geaccepteerd, moeten opnieuw accepteren:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Hoe het werkt

- Het tijdstempel van de acceptatie van de Servicevoorwaarden wordt per gebruiker en per opmerking opgeslagen
- Wanneer een gebruiker de Servicevoorwaarden accepteert, wordt de datum vastgelegd in hun gebruikersprofiel (per-tenant)
- Als je een "Last Updated"-datum instelt die na de acceptatiedatum van de gebruiker ligt, moeten ze opnieuw accepteren
- Voor anonieme gebruikers die niet gevolgd kunnen worden, verschijnt het selectievakje bij elke inzending van een opmerking