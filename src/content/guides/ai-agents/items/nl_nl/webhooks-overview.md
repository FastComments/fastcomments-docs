Agent-webhooks zijn HTTP-callbacks die door het platform worden afgevuurd wanneer een agent-run voltooid is of een goedkeuring van status verandert. Gebruik ze om agent-activiteit door te sturen naar je eigen systemen - moderatiedashboards, auditlogs, Slack-kanalen, escalatietools.

Te configureren onder het tabblad **Webhooks** op de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents).

### Wat wordt geleverd

Vier gebeurtenistypes:

- **`trigger.succeeded`** - een agent-run is succesvol voltooid.
- **`trigger.failed`** - een agent-run is misgegaan.
- **`approval.requested`** - een actie is in de wachtrij geplaatst voor menselijke goedkeuring.
- **`approval.decided`** - een goedkeuring is goedgekeurd, afgewezen, of de uitvoering is misgegaan.

Zie [Webhook Events](#webhook-events) voor welke gebeurtenissen wanneer afvuren, en [Webhook Payloads](#webhook-payloads) voor het schema van elk.

### Wat niet wordt geleverd

- **Webhooks per tool-actie.** Een run die `pin_comment` aanroept vuurt geen aparte webhook af voor de pin - de actie is opgenomen in de `trigger.succeeded`-payload van de run. Als je levering per actie wilt, parseer dan de `actions`-array in de trigger-payload.
- **Genegeerde triggers.** Een trigger die niet wordt verzonden (over budget, verkeerde scope) vuurt geen webhook af. Genegeerde triggers zijn alleen zichtbaar op de [Analytics-pagina](#analytics-page).
- **Triggers geproduceerd door replays.** Testruns vuren geen webhooks af. Zie [Test Runs (Replays)](#test-runs-replays).

### Configuratie

Voor elke webhook stel je in:

- **URL** - het HTTPS-eindpunt waarnaar wordt gepost.
- **Domein** - het commentaardomein waarop deze webhook van toepassing is (je tenant kan meerdere domeinen hosten). `*` komt overeen met alle domeinen; `*.example.com` is een glob; een exact domein komt alleen met dat ene domein overeen.
- **Events** - op welke van de vier gebeurtenistypes je wilt abonneren.
- **Agents** - leeg voor "alle agents", of een lijst met specifieke agent-ID's om te filteren.
- **Method** - POST of PUT (standaard POST).
- **No-retry statuscodes** - HTTP-responscodes die als definitieve fouten moeten worden behandeld en niet opnieuw worden geprobeerd (bijv. 410 Gone, 422 Unprocessable). Zie [Webhook Retries](#webhook-retries).

Meerdere webhooks kunnen op dezelfde gebeurtenis abonneren - elke webhook staat onafhankelijk in de wachtrij en wordt geleverd aan zijn eigen URL.

### Matching per domein

Een bepaalde gebeurtenis wordt geleverd aan **elke webhook waarvan het veld `domain` overeenkomt met het domein van het commentaar**. De matching gebruikt een eenvoudige glob:

- Exact: `example.com` komt alleen overeen met `example.com`.
- Wildcard-ster: `*` komt overeen met elk domein.
- Subdomein-glob: `*.example.com` komt overeen met `blog.example.com`, `forum.example.com`, maar niet met `example.com` zelf.

Het domein in een payload is het eerste niet-null resultaat van: het `domain` van het commentaar, de URL geparsed tegen de domeinconfiguratie van je tenant, of de `Page`-lookup op `urlId`.

### Filtering per agent

Het veld **Agents** laat een webhook alleen op bepaalde agents abonneren. Leeg betekent "alle agents". Wanneer het niet leeg is, vuurt de webhook alleen af voor agents in de lijst.

Dit is handig wanneer je één webhook hebt voor moderatiegebeurtenissen en een andere voor engagementgebeurtenissen, die beide naar verschillende downstream-systemen routeren.

### Testverzending

De webhookconfiguratie-UI heeft een knop **Test send** die een nep-payload naar de URL post, zodat je connectiviteit, ondertekening en de responscodes van je eindpunt kunt verifiëren zonder op een echte gebeurtenis te wachten.

### Leveringslogs

Elke levering (en elke herpoging) komt terecht op de pagina [Webhook Delivery Logs](#webhook-logs), zodat je kunt zien wat er over de draad is gegaan.

### Ondertekening

Elke webhook wordt ondertekend met HMAC-SHA256 met behulp van het API-geheim van je tenant. Zie [Webhook Signing](#webhook-signing).

### Geschiktheid

Agent-webhooks vereisen geldige facturering op de tenant. Ze gebruiken dezelfde ondertekenings-/geheiminfrastructuur als je bestaande commentaarwebhooks - als je commentaarwebhooks al hebt geïntegreerd (zie de [Webhooks-gids](/guide-webhooks.html)), heeft de agent-webhookintegratie dezelfde vorm met nieuwe gebeurtenistypes.