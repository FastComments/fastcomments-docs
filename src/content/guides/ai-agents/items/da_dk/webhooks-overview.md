Agent webhooks er HTTP-callbacks, som platformen udsender, når en agent-kørsel afsluttes, eller en godkendelse ændrer tilstand. Brug dem til at videresende agent-aktivitet til dine egne systemer - moderationsdashboards, auditlogs, Slack-kanaler, eskalationsværktøjer.

Konfigureres under fanen **Webhooks** på [AI Agents-siden](https://fastcomments.com/auth/my-account/ai-agents).

### Hvad der leveres

Fire hændelsestyper:

- **`trigger.succeeded`** - en agent-kørsel blev gennemført med succes.
- **`trigger.failed`** - en agent-kørsel fejlede.
- **`approval.requested`** - en handling blev sat i kø til menneskelig godkendelse.
- **`approval.decided`** - en godkendelse blev godkendt, afvist, eller eksekveringen fejlede.

Se [Webhook Events](#webhook-events) for, hvilke hændelser der udløses hvornår, og [Webhook Payloads](#webhook-payloads) for skemaet for hver.

### Hvad der ikke leveres

- **Webhooks pr. værktøjshandling.** En kørsel, der kalder `pin_comment`, udløser ikke en separat webhook for selve pin-handlingen - handlingen er inkluderet i kørselens `trigger.succeeded`-payload. Hvis du ønsker levering pr. handling, skal du parse `actions`-arrayet i trigger-payloaden.
- **Droppede triggers.** En trigger, der ikke afsendes (over budget, forkert scope), udløser ikke en webhook. Drops er kun synlige på [Analytics-siden](#analytics-page).
- **Replay-producerede triggers.** Testkørsler udløser ikke webhooks. Se [Test Runs (Replays)](#test-runs-replays).

### Konfiguration

For hver webhook angiver du:

- **URL** - HTTPS-endepunktet, der skal POST'es til.
- **Domain** - det kommentardomæne, denne webhook gælder for (din tenant kan hoste flere domæner). `*` matcher alle domæner; `*.example.com` er en glob; et præcist domæne matcher kun det pågældende domæne.
- **Events** - hvilke af de fire hændelsestyper, der skal abonneres på.
- **Agents** - tomt for "alle agenter", eller en liste over specifikke agent-ID'er at filtrere efter.
- **Method** - POST eller PUT (standard POST).
- **No-retry status codes** - HTTP-svarcodes, der skal behandles som endelige fejl og ikke forsøges igen (f.eks. 410 Gone, 422 Unprocessable). Se [Webhook Retries](#webhook-retries).

Flere webhooks kan abonnere på samme hændelse - hver enkelt sættes i kø uafhængigt og leveres til sin egen URL.

### Matching pr. domæne

En given hændelse leveres til **enhver webhook, hvis `domain`-felt matcher kommentarens domæne**. Matchingen bruger en simpel glob:

- Præcis: `example.com` matcher kun `example.com`.
- Wildcard-stjerne: `*` matcher alle domæner.
- Subdomæne-glob: `*.example.com` matcher `blog.example.com`, `forum.example.com`, men ikke `example.com` selv.

Domænet i en payload er det første ikke-null resultat af: kommentarens `domain`, URL'en parset mod din tenants domænekonfiguration, eller `Page`-opslaget via `urlId`.

### Filtrering pr. agent

Feltet **Agents** gør det muligt for en webhook kun at abonnere på bestemte agenter. Tomt betyder "alle agenter". Når feltet ikke er tomt, udløses webhooken kun for agenter på listen.

Dette er nyttigt, når du har én webhook til moderationshændelser og en anden til engagementshændelser, som begge router til forskellige downstream-systemer.

### Testsend

Webhook-konfigurations-UI har en **Test send**-knap, som poster en falsk payload til URL'en, så du kan verificere forbindelse, signering og dit endepunkts svarkode uden at vente på en rigtig hændelse.

### Leveringslogs

Hver levering (og hvert retry) lander på [Webhook Delivery Logs](#webhook-logs)-siden, så du kan se, hvad der skete på kablet.

### Signering

Hver webhook signeres med HMAC-SHA256 ved brug af din tenants API-secret. Se [Webhook Signing](#webhook-signing).

### Berettigelse

Agent-webhooks kræver gyldig fakturering på tenanten. De bruger samme signerings-/secret-infrastruktur som dine eksisterende kommentar-webhooks - hvis du allerede har integreret kommentar-webhooks (se [Webhooks-guiden](/guide-webhooks.html)), har agent-webhook-integrationen samme form med nye hændelsestyper.