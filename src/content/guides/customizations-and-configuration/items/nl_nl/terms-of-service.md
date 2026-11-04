Met FastComments kun je vereisen dat eerste keer commentaargevers je Servicevoorwaarden accepteren voordat ze een reactie plaatsen.

Wanneer ingeschakeld:
- **Anonieme gebruikers** zien bij elke reactie een TOS-selectievakje
- **Geverifieerde gebruikers** zien het selectievakje alleen bij hun eerste reactie, of wanneer je je TOS bijwerkt

### Configuratie

Navigeer naar de widget-aanpassingspagina en vink het selectievakje "Acceptatie Servicevoorwaarden vereisen" aan. Eenmaal ingeschakeld zie je de volgende opties:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Servicevoorwaarden-paneel met de TOS-tekstmoduskiezer en het veld voor de datum van laatste update'; title='Servicevoorwaarden Opties' app-screenshot-end]

- **TOS-tekstmodus**: Standaard toont het selectievakje "I agree to the Terms of Service and Privacy Policy" met links naar beide documenten. Selecteer "Tekst per taal aanpassen" om je eigen tekst per taal op te geven.
- **TOS Datum Laatst Bijgewerkt**: Wanneer je je Servicevoorwaarden bijwerkt, stel je deze datum in. Gebruikers die vóór deze datum hebben geaccepteerd, moeten opnieuw accepteren.

### Hoe Het Werkt

- De TOS-acceptatietijdstempel wordt per gebruiker en per reactie opgeslagen
- Wanneer een gebruiker de TOS accepteert, wordt de datum vastgelegd in zijn gebruikersprofiel (per tenant)
- Als je een "Laatst Bijgewerkt"-datum instelt die na de acceptatiedatum van de gebruiker ligt, moet hij opnieuw accepteren
- Voor anonieme gebruikers die niet kunnen worden gevolgd, verschijnt het selectievakje bij elke reactie