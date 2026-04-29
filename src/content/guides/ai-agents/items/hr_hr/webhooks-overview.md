Agent webhooks su HTTP pozivnici (callbacks) koje platforma šalje kada se izvršavanje agenta završi ili se promijeni stanje odobrenja. Koristite ih za prosljeđivanje aktivnosti agenta u vaše sustave — nadzorne ploče za moderaciju, zapise revizije, Slack kanale, alate za eskalaciju.

Konfigurirano pod karticom **Webhooks** na [stranici AI agenata](https://fastcomments.com/auth/my-account/ai-agents).

### Što se isporučuje

Četiri tipa događaja:

- **`trigger.succeeded`** - izvršavanje agenta je uspješno završeno.
- **`trigger.failed`** - izvršavanje agenta je završilo s greškom.
- **`approval.requested`** - akcija je stavljena u red čekanja za ljudsko odobrenje.
- **`approval.decided`** - odobrenje je odobreno, odbijeno ili je izvršenje neuspjelo.

Pogledajte [Webhook Events](#webhook-events) za točno kada se koji događaji pokreću, i [Webhook Payloads](#webhook-payloads) za shemu svakog od njih.

### Što se ne isporučuje

- **Per-tool-action webhooks.** Pokretanje koje poziva `pin_comment` ne pokreće zaseban webhook za pin — ta je akcija uključena u `trigger.succeeded` payload pokretanja. Ako želite isporuku po akciji, parsirajte niz `actions` u trigger payloadu.
- **Dropped triggers.** Trigger koji se ne dispatcha (prekoračenje budžeta, pogrešan opseg) ne pokreće webhook. Odbacivanja su vidljiva samo na [stranici Analitike](#analytics-page).
- **Replay-produced triggers.** Testna pokretanja ne šalju webhooks. Pogledajte [Test Runs (Replays)](#test-runs-replays).

### Konfiguracija

Za svaki webhook koji postavite:

- **URL** - HTTPS endpoint na koji se šalje POST.
- **Domain** - domena komentara na koju se ovaj webhook primjenjuje (vaš tenant može imati više domena). `*` odgovara svim domenama; `*.example.com` je glob; točna domena odgovara samo toj.
- **Events** - na koje od četiri vrste događaja se pretplaćujete.
- **Agents** - prazno za "sve agente", ili lista specifičnih ID-eva agenata za filtriranje.
- **Method** - POST ili PUT (zadano POST).
- **No-retry status codes** - HTTP odgovori koje treba tretirati kao konačne greške bez ponovnog pokušaja (npr. 410 Gone, 422 Unprocessable). Pogledajte [Webhook Retries](#webhook-retries).

Više webhookova može se pretplatiti na isti događaj — svaki se stavlja u red neovisno i isporučuje na svoj URL.

### Podudaranje po domeni

Događaj se isporučuje svim webhookovima čije polje `domain` odgovara domeni komentara. Podudaranje koristi jednostavan glob:

- Exact: `example.com` odgovara samo `example.com`.
- Wildcard star: `*` odgovara svakoj domeni.
- Subdomain glob: `*.example.com` odgovara `blog.example.com`, `forum.example.com`, ali ne i samom `example.com`.

Domena u payloadu je prvi nenulti rezultat iz: komentarevog `domain`, URL parsiran prema konfiguraciji domena vašeg tenanta, ili `Page` pretrage po `urlId`.

### Filtriranje po agentu

Polje **Agents** omogućava webhooku da se pretplati samo na određene agente. Prazno znači "svi agenti". Kada nije prazno, webhook se okida samo za agente s popisa.

Ovo je korisno kada imate jedan webhook za događaje moderacije, a drugi za događaje angažmana, koji oba usmjeravaju prema različitim downstream sustavima.

### Testno slanje

UI konfiguracije webhooka ima gumb **Test send** koji šalje lažni payload na URL kako biste mogli provjeriti povezivost, potpisivanje i kod odgovora vašeg endpointa bez čekanja stvarnog događaja.

### Zapisi isporuke

Svaka isporuka (i svaki ponovni pokušaj) završava na stranici [Webhook Delivery Logs](#webhook-logs) tako da možete vidjeti što se dogodilo na mreži.

### Potpisivanje

Svaki webhook je potpisan HMAC-SHA256 koristeći API secret vašeg tenanta. Pogledajte [Webhook Signing](#webhook-signing).

### Dostupnost

Agent webhooks zahtijevaju valjanu naplatu na tenantu. Koriste istu infrastrukturu potpisivanja/sekreta kao i vaši postojeći webhookovi za komentare — ako ste već integrirali webhookove za komentare (pogledajte [Vodič za Webhooks](/guide-webhooks.html)), integracija agent webhooks je iste forme s novim vrstama događaja.