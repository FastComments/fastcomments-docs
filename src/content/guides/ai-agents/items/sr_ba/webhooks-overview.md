Agent webhooks su HTTP povratni pozivi koje platforma šalje kada se izvršavanje agenta završi ili kada se stanje odobrenja promijeni. Koristite ih da proslijedite aktivnost agenta u vaše sisteme - nadzorne ploče za moderaciju, zapisnike audita, Slack kanale, alate za eskalaciju.

Konfiguriše se pod karticom **Webhookovi** na [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### Šta se dostavlja

Četiri tipa događaja:

- **`trigger.succeeded`** - izvršavanje agenta je uspješno završeno.
- **`trigger.failed`** - izvršavanje agenta je završeno greškom.
- **`approval.requested`** - akcija je stavljena u red za ljudsko odobrenje.
- **`approval.decided`** - odobrenje je odobreno, odbijeno, ili je izvršavanje zašlo u grešku.

Pogledajte [Webhook Events](#webhook-events) za to kada se koji događaj pokreće, i [Webhook Payloads](#webhook-payloads) za šemu svakog od njih.

### Šta se ne dostavlja

- **Per-tool-action webhooks.** Izvršavanje koje poziva `pin_comment` ne šalje poseban webhook za pin — akcija je uključena u `trigger.succeeded` payload pokretanja. Ako želite isporuku po akciji, parsirajte niz `actions` u trigger payload-u.
- **Dropped triggers.** Trigger koji se ne dispečuje (preko budžeta, pogrešan opseg) ne šalje webhook. Dropovi su vidljivi samo na [Analytics page](#analytics-page).
- **Replay-produced triggers.** Test pokretanja ne šalju webhooks. Pogledajte [Test Runs (Replays)](#test-runs-replays).

### Konfiguracija

Za svaki webhook koji podesite:

- **URL** - HTTPS endpoint na koji se šalje POST.
- **Domain** - domen komentara na koji se ovaj webhook odnosi (vaš tenant može hostovati više domena). `*` odgovara svim domenima; `*.example.com` je glob; tačan domen odgovara samo tom jednom.
- **Events** - koje od četiri vrste događaja pratiti.
- **Agents** - prazno za "svi agenti", ili lista određenih ID-eva agenata za filtriranje.
- **Method** - POST ili PUT (podrazumijevano POST).
- **No-retry status codes** - HTTP status kodovi koji se tretiraju kao terminalne greške i ne pokušavaju ponovo (npr. 410 Gone, 422 Unprocessable). Pogledajte [Webhook Retries](#webhook-retries).

Više webhooks može da se pretplati na isti događaj - svaki se stavlja u svoj red i isporučuje na sopstveni URL.

### Podudaranje po domenu

Događaj se dostavlja svim webhookovima čije polje `domain` odgovara domenu komentara. Podudaranje koristi jednostavan glob:

- Tačno: `example.com` odgovara samo `example.com`.
- Wildcard star: `*` odgovara svakom domenu.
- Subdomain glob: `*.example.com` odgovara `blog.example.com`, `forum.example.com`, ali ne i samom `example.com`.

Domen u payload-u je prvi nenulti rezultat od: komentara `domain`, URL-a parsiranog prema konfiguraciji domena vašeg tenant-a, ili `Page` lookup-a po `urlId`.

### Filtriranje po agentu

Polje **Agents** omogućava webhooku da se pretplati samo na određene agente. Prazno znači "svi agenti". Kada nije prazno, webhook se pokreće samo za agente iz liste.

Ovo je korisno kada imate jedan webhook za događaje moderacije i drugi za engagement događaje, oba usmjerena ka različitim downstream sistemima.

### Test send

UI konfiguracije webhooka ima dugme **Test send** koje šalje lažni payload na URL kako biste mogli provjeriti konektivnost, potpisivanje i kod odgovora vašeg endpointa bez čekanja pravog događaja.

### Delivery logs

Svaka isporuka (i svaki pokušaj ponovnog slanja) završava na stranici [Webhook Delivery Logs](#webhook-logs) tako da možete vidjeti šta se desilo na mreži.

### Signing

Svaki webhook je potpisan HMAC-SHA256 koristeći API secret vašeg tenant-a. Pogledajte [Webhook Signing](#webhook-signing).

### Uslovi

Agent webhooks zahtijevaju važeće naplativanje na tenant-u. Koriste istu infrastrukturu potpisivanja/sekreta kao i vaši postojeći comment webhooks - ako ste već integrisali comment webhooks (pogledajte [Webhooks guide](/guide-webhooks.html)), integracija agent webhooks ima isti oblik sa novim tipovima događaja.