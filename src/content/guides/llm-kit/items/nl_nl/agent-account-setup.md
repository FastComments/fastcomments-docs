Een coding agent zoals Claude Code, Cursor, of een MCP-gebaseerde assistent kan FastComments voor je instellen zonder dat je het registratieformulier hoeft in te vullen. Dit is handig wanneer je een agent vraagt om "comments toe te voegen aan mijn site" en je nog geen account hebt.

### Hoe het werkt

1. De agent maakt een nieuw account aan en ontvangt een API key en een claim link. De API key werkt meteen, dus de agent kan het account configureren en de widget op je site installeren.
2. De agent geeft je de claim link. Open deze in je browser, log in of maak een login aan, en bevestig de claim. Het account is dan van jou: je beheert het, de facturering ervan, en de API keys vanuit het dashboard. De pagina toont de API key die de agent heeft, zodat je deze kunt intrekken als je niet langer wilt dat de agent, of wie deze ook beheert, toegang heeft.
3. Als niemand de claim link binnen 72 uur opent, worden het account en de key verwijderd. Vraag de agent om een nieuw account aan te maken.

Tot het geclaimd is, gelden voor het account dezelfde limieten als voor een normale gratis proefperiode.

### Als je al een account hebt

Elke login heeft één account. Als je een claim link opent terwijl je bent ingelogd op een bestaand account, kun je op de pagina kiezen:

- **Koppel aan mijn account** maakt van het nieuwe account een beheerde tenant van het account waarop je bent ingelogd. Dit vereist een betaald abonnement met white labeling, en het gebruik van de nieuwe tenant wordt gefactureerd op jouw account.
- **Log uit en claim met een andere login** logt je uit en brengt je terug naar de claim pagina zodat je deze kunt claimen met een andere login.

### Voor agent auteurs

De agent instructies op [fastcomments.com/agents.md](https://fastcomments.com/agents.md) beschrijven de account creation call, de velden in de response, en hoe je de claim link overhandigt aan de persoon voor wie je werkt. De call heeft geen API key nodig en is rate limited per IP address.