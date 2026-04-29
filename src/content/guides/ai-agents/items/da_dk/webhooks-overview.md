Agent webhooks er HTTP-callbacks, som platformen udløser, når en agentkørsel fuldføres eller en godkendelse ændrer tilstand. Brug dem til at videresende agentaktivitet til dine egne systemer - moderationsdashboarder, revisionslogfiler, Slack-kanaler, eskaleringsværktøjer.

Konfigureres under fanen **Webhooks** på [AI Agents-siden](https://fastcomments.com/auth/my-account/ai-agents).

### Hvad leveres

Fire begivenhedstyper:

- **`trigger.succeeded`** - en agentkørsel fuldførtes succesfuldt.
- **`trigger.failed`** - en agentkørsel fejlede.
- **`approval.requested`** - en handling blev køet til manuel godkendelse.
- **`approval.decided`** - en godkendelse blev godkendt, afvist eller eksekvering fejlede.

Se [Webhook Events](#webhook-events) for hvilke begivenheder der udløses hvornår, og [Webhook Payloads](#webhook-payloads) for skemaet for hver.

### Hvad der ikke leveres

- **Per-tool-action webhooks.** En kørsel, der kalder `pin_comment`, udløser ikke et separat webhook for pinningen - handlingen er inkluderet i kørsels `trigger.succeeded` payload. Hvis du ønsker levering pr. handling, analyser `actions`-arrayet i trigger-payloaden.
- **Dropped triggers.** En trigger, der ikke dispatches (over budget, forkert scope), udløser ikke et webhook. Drops er kun synlige i [Analytics page](#analytics-page).
- **Replay-produced triggers.** Testkørsler udløser ikke webhooks. Se [Test Runs (Replays)](#test-runs-replays).

### Konfiguration

For hver webhook du konfigurerer:

- **URL** - HTTPS-endpointet der skal POST'es til.
- **Domain** - den kommentardomæne, som dette webhook gælder for (din tenant kan være vært for flere domæner). `*` matcher alle domæner; `*.example.com` er en glob; et eksakt domæne matcher kun det.
- **Events** - hvilke af de fire begivenhedstyper der skal abonneres på.
- **Agents** - tomt for 'alle agenter', eller en liste over specifikke agent-ID'er at filtrere efter.
- **Method** - POST eller PUT (standard POST).
- **No-retry status codes** - HTTP-responskoder, der bør behandles som endelige fejl og ikke forsøges igen (f.eks. 410 Gone, 422 Unprocessable). Se [Webhook Retries](#webhook-retries).

Flere webhooks kan abonnere på den samme begivenhed - hver køes uafhængigt og leveres til sin egen URL.

### Pr. domænematching

En given begivenhed leveres til **hver webhook hvis `domain`-felt matcher kommentarens domæne**. Matchningen bruger en simpel glob:

- Eksakt: `example.com` matcher kun `example.com`.
- Wildcard-stjerne: `*` matcher alle domæner.
- Subdomæne-glob: `*.example.com` matcher `blog.example.com`, `forum.example.com`, men ikke `example.com` selv.

Domænet på en payload er det første ikke-nul resultat fra: kommentarens `domain`, URL'en parset i forhold til din tenants domænekonfiguration, eller `Page`-opslag via `urlId`.

### Pr. agentfiltrering

Feltet **Agents** lader en webhook abonnere kun på bestemte agenter. Tomt betyder 'alle agenter'. Når det ikke er tomt, udløses webhook'en kun for agenter i listen.

Dette er nyttigt, når du har én webhook til moderationbegivenheder og en anden til engagement-begivenheder, som begge rutes til forskellige downstream-systemer.

### Test send

Webhook-konfigurations-UI'en har en **Test send**-knap, der POST'er en falsk payload til URL'en, så du kan verificere forbindelsen, signeringen og dit endpoints responskode uden at vente på en reel begivenhed.

### Leveringslogfiler

Hver levering (og hvert genforsøg) lander på siden [Webhook Delivery Logs](#webhook-logs), så du kan se, hvad der skete på forbindelsen.

### Signering

Hver webhook signeres med HMAC-SHA256 ved hjælp af din tenants API-secret. Se [Webhook Signing](#webhook-signing).

### Berettigelse

Agent-webhooks kræver gyldig fakturering på din tenant. De bruger samme signerings-/secret-infrastruktur som dine eksisterende kommentar-webhooks - hvis du allerede har integreret kommentar-webhooks (se [Webhooks-guiden](/guide-webhooks.html)), er agent-webhook-integrationen af samme form med nye begivenhedstyper.