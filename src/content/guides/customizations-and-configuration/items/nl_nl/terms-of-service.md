FastComments stelt je in staat om gebruikers die voor het eerst reageren te verplichten je Servicevoorwaarden te accepteren voordat ze een reactie indienen.

When enabled:
- **Anonieme gebruikers** zullen een TOS-selectievakje zien elke keer dat ze reageren
- **Aangemelde gebruikers** zullen het selectievakje alleen bij hun eerste reactie zien, of wanneer je je TOS bijwerkt

### Configuratie

Ga naar de pagina voor het aanpassen van de widget en schakel het selectievakje "Acceptatie van de Servicevoorwaarden verplichten" in. Zodra ingeschakeld zie je de volgende opties:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS-tekstmodus**: Standaard toont het selectievakje "Ik ga akkoord met de Servicevoorwaarden en het Privacybeleid" met links naar beide documenten. Selecteer "Tekst per taal aanpassen" om voor elke taal je eigen tekst op te geven.
- **TOS Laatst Bijgewerkt Datum**: Wanneer je je Servicevoorwaarden bijwerkt, stel deze datum in. Gebruikers die vóór deze datum hebben geaccepteerd, zullen opnieuw moeten accepteren.

### Hoe het werkt

- De TOS-acceptatietijdstempel wordt per gebruiker en per reactie opgeslagen
- Wanneer een gebruiker de TOS accepteert, wordt de datum vastgelegd op hun gebruikersprofiel (per-tenant)
- Als je een "Laatst bijgewerkt"-datum instelt die later is dan de acceptatiedatum van de gebruiker, moeten ze opnieuw accepteren
- Voor anonieme gebruikers die niet gevolgd kunnen worden, verschijnt het selectievakje bij iedere reactieverzending