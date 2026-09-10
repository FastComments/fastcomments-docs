A coding agent such as Claude Code, Cursor, or an MCP-based assistant can set up FastComments for you without you filling in the signup form. This is useful when you ask an agent to "add comments to my site" and you do not have an account yet.

### Hoe het werkt

1. De agent maakt een nieuw account aan en ontvangt een API‑sleutel en een claim‑link. De API‑sleutel werkt meteen, zodat de agent het account kan configureren en de widget op je site kan installeren.  
2. De agent geeft je de claim‑link. Open deze in je browser, log in of maak een login aan, en bevestig de claim. Het account is dan van jou: je beheert het, de facturering en de API‑sleutels via het dashboard. De pagina toont de API‑sleutel die de agent bezit, zodat je deze kunt intrekken als je de agent, of wie het ook uitvoert, geen toegang meer wilt geven.  
3. Als niemand de claim‑link binnen 72 uur opent, worden het account en de sleutel verwijderd. Vraag de agent om een nieuwe aan te maken.

Totdat het is geclaimd, heeft het account dezelfde limieten als een normale gratis proefperiode.

### Als je al een account hebt

Elke login bezit één account. Als je een claim‑link opent terwijl je bent ingelogd op een bestaand account, laat de pagina je kiezen:

- **Koppel aan mijn account** maakt van het nieuwe account een beheerde tenant van het account waarin je bent ingelogd. Dit vereist een betaald abonnement met white‑labeling, en het gebruik van de nieuwe tenant wordt gefactureerd aan jouw account.  
- **Log uit en claim met een andere login** logt je uit en brengt je terug naar de claim‑pagina zodat je deze kunt claimen met een andere login.

### Voor agent‑auteurs

De agent‑instructies op [fastcomments.com/agents.md](https://fastcomments.com/agents.md) beschrijven de account‑aanmaak‑call, de velden in de respons, en hoe je de claim‑link aan de persoon kunt overhandigen waarvoor je werkt. De call vereist geen API‑sleutel en is per IP‑adres rate‑gelimiteerd.