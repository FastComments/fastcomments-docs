Agent webhooks zijn HTTP-callbacks die door het platform worden afgevuurd wanneer een uitvoering van een agent voltooid is of wanneer een goedkeuring van status verandert. Gebruik ze om agent-activiteit door te sturen naar uw eigen systemen - moderatie-dashboards, auditlogs, Slack-kanalen, escalatiehulpmiddelen.

Geconfigureerd onder het **Webhooks**-tabblad op de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents).

### Wat wordt geleverd

Vier gebeurtenistypen:

- **`trigger.succeeded`** - een agentuitvoering is succesvol voltooid.
- **`trigger.failed`** - een agentuitvoering heeft een fout opgeleverd.
- **`approval.requested`** - een actie is in de wachtrij gezet voor menselijke goedkeuring.
- **`approval.decided`** - een goedkeuring is goedgekeurd, afgewezen of de uitvoering is gefaald.

Zie [Webhook-evenementen](#webhook-events) voor welke gebeurtenissen wanneer afgevuurd worden, en [Webhook Payloads](#webhook-payloads) voor het schema van elk van hen.

### Wat niet wordt geleverd

- **Per-tool-action webhooks.** Een run die `pin_comment` aanroept vuurt geen aparte webhook af voor het pinnnen - de actie is opgenomen in de `trigger.succeeded`-payload van de run. Als u levering per actie wilt, parseer dan de `actions` array in de trigger-payload.
- **Niet-verzonden triggers.** Een trigger die niet wordt gedispatched (budget overschreden, verkeerde scope) vuurt geen webhook af. Drops zijn alleen zichtbaar in de [Analytics-pagina](#analytics-page).
- **Triggers geproduceerd door replays.** Testruns vuren geen webhooks af. Zie [Test Runs (Replays)](#test-runs-replays).

### Configuratie

Voor elke webhook die u instelt:

- **URL** - het HTTPS-endpoint waarnaar POST wordt gedaan.
- **Domain** - het comment-domein waarop deze webhook van toepassing is (uw tenant kan meerdere domeinen hosten). `*` matcht alle domeinen; `*.example.com` is een glob; een exact domein matcht alleen dat domein.
- **Events** - op welke van de vier gebeurtenistypen u zich wilt abonneren.
- **Agents** - leeg voor "all agents", of een lijst met specifieke agent-ID's om op te filteren.
- **Method** - POST of PUT (standaard POST).
- **No-retry status codes** - HTTP-responscodes die als terminale fouten moeten worden behandeld en niet opnieuw geprobeerd mogen worden (bijv. 410 Gone, 422 Unprocessable). Zie [Webhook Retries](#webhook-retries).

Meerdere webhooks kunnen zich abonneren op hetzelfde event - elk wordt onafhankelijk in de wachtrij gezet en naar zijn eigen URL afgeleverd.

### Per-domein matching

Een gegeven event wordt afgeleverd aan **elke webhook waarvan het `domain`-veld overeenkomt met het domein van de opmerking**. De matching gebruikt een eenvoudige glob:

- Exact: `example.com` matcht alleen `example.com`.
- Wildcard star: `*` matcht elk domein.
- Subdomain glob: `*.example.com` matcht `blog.example.com`, `forum.example.com`, maar niet `example.com` zelf.

Het domein in een payload is het eerste niet-null resultaat van: de `domain` van de opmerking, de URL geparsed tegen de domeinconfiguratie van uw tenant, of de `Page`-opvraging via `urlId`.

### Per-agent filtering

Het **Agents**-veld laat een webhook alleen op bepaalde agents abonneren. Leeg betekent "all agents". Wanneer niet-leeg, vuurt de webhook alleen voor agents in de lijst.

Dit is nuttig wanneer u één webhook heeft voor moderatiegebeurtenissen en een andere voor engagementgebeurtenissen, die elk naar verschillende downstreamsystemen routeren.

### Testverzending

De webhook-config UI heeft een **Test send**-knop die een nep-payload naar de URL post zodat u connectiviteit, signing en de responscode van uw endpoint kunt verifiëren zonder te wachten op een echt event.

### Leveringslogs

Elke levering (en elke retry) verschijnt in de [Webhook Delivery Logs](#webhook-logs)-pagina zodat u kunt zien wat er op het netwerk is gebeurd.

### Ondertekening

Elke webhook wordt ondertekend met HMAC-SHA256 met gebruik van uw tenant's API secret. Zie [Webhook Signing](#webhook-signing).

### Geschiktheid

Agent webhooks vereisen geldige billing op de tenant. Ze gebruiken dezelfde signing/secret-infrastructuur als uw bestaande comment webhooks - als u comment webhooks al heeft geïntegreerd (zie de [Webhooks-gids](/guide-webhooks.html)), heeft de agent-webhookintegratie dezelfde vorm maar met nieuwe gebeurtenistypen.

---