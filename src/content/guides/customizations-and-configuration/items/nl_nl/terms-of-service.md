FastComments stelt je in staat om van eerste keer commentatoren te eisen dat ze je Servicevoorwaarden accepteren voordat ze een reactie indienen.

Wanneer ingeschakeld:
- **Anonieme gebruikers** zien elke keer dat ze reageren een TOS-selectievakje
- **Geauthenticeerde gebruikers** zien het selectievakje alleen bij hun eerste reactie, of wanneer je je TOS bijwerkt

### Configuration

Ga naar de widget-aanpassingspagina en schakel het selectievakje "Vereis acceptatie van Servicevoorwaarden" in. Zodra dit is ingeschakeld, zie je de volgende opties:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Servicevoorwaardenpaneel dat de selector voor TOS-tekstmodus en het veld met de laatst bijgewerkte datum toont'; title='Opties voor Servicevoorwaarden' app-screenshot-end]

- **TOS Tekstmodus**: Standaard toont het selectievakje "Ik ga akkoord met de Servicevoorwaarden en het Privacybeleid" met links naar beide documenten. Selecteer "Tekst per locale aanpassen" om je eigen tekst voor elke taal te geven.
- **TOS Laatst Bijgewerkt Datum**: Wanneer je je Servicevoorwaarden bijwerkt, stel je deze datum in. Gebruikers die vóór deze datum hebben geaccepteerd, moeten opnieuw accepteren.

### How It Works

- Het tijdstempel van de TOS-acceptatie wordt per gebruiker en per reactie opgeslagen
- Wanneer een gebruiker de TOS accepteert, wordt de datum op hun gebruikersprofiel (per tenant) vastgelegd
- Als je een "Laatst Bijgewerkt" datum instelt die later is dan de acceptatiedatum van de gebruiker, moeten ze opnieuw accepteren
- Voor anonieme gebruikers die niet kunnen worden gevolgd, verschijnt het selectievakje bij elke reactie-indiening

---