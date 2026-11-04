Agent veb-hukovi su HTTP povratni pozivi koje platforma pokreće kada se agent run završi ili kada se stanje odobrenja promeni. Koristite ih da prosledite aktivnost agenata u svoje sisteme - moderation dashboards, audit logove, Slack kanale, alate za eskalaciju.

Konfigurišu se pod tabom **Webhooks** na [AI Agents stranici](https://fastcomments.com/auth/my-account/ai-agents).

### Šta se dostavlja

Četiri tipa događaja:

- **`trigger.succeeded`** - agent run je uspešno završen.
- **`trigger.failed`** - agent run je doživeo grešku.
- **`approval.requested`** - akcija je stavljena u red za ljudsko odobrenje.
- **`approval.decided`** - odobrenje je odobreno, odbijeno ili je izvršavanje propalo.

Pogledajte [Webhook Events](#webhook-events) za to koji se događaji kada okidaju, i [Webhook Payloads](#webhook-payloads) za šemu svakog od njih.

### Šta se ne dostavlja

- **Veb-hukovi po pojedinačnoj tool akciji.** Run koji poziva `pin_comment` ne okida poseban veb-huk za to pinovanje - akcija je uključena u payload run-a `trigger.succeeded`. Ako želite dostavu po akciji, parsirajte `actions` niz u trigger payload-u.
- **Odbačeni triggeri.** Trigger koji se ne otprema (preko budžeta, pogrešan scope) ne okida veb-huk. Odbacivanja su vidljiva samo na [Analytics stranici](#analytics-page).
- **Triggeri proizvedeni replay-em.** Test runovi ne okidaju veb-hukove. Pogledajte [Test Runs (Replays)](#test-runs-replays).

### Konfiguracija

Za svaki veb-huk postavljate:

- **URL** - HTTPS endpoint na koji se radi POST.
- **Domain** - domen komentara na koji se ovaj veb-huk odnosi (vaš tenant može hostovati više domena). `*` odgovara svim domenima; `*.example.com` je glob; tačan domen odgovara samo tom jednom.
- **Events** - na koje od četiri tipa događaja se pretplatite.
- **Agents** - prazno za "sve agente", ili lista konkretnih agent ID-jeva za filtriranje.
- **Method** - POST ili PUT (podrazumevano POST).
- **No-retry status codes** - HTTP statusni kodovi koji se tretiraju kao konačne greške, bez ponovnih pokušaja (npr. 410 Gone, 422 Unprocessable). Pogledajte [Webhook Retries](#webhook-retries).

Više veb-hukova može biti pretplaćeno na isti događaj - svaki se nezavisno stavlja u red i dostavlja na svoj URL.

### Podudaranje po domenu

Dati događaj se dostavlja **svakom veb-huku čije `domain` polje odgovara domenu komentara**. Podudaranje koristi jednostavan glob:

- Tačno: `example.com` odgovara samo `example.com`.
- Džoker zvezdica: `*` odgovara svakom domenu.
- Subdomain glob: `*.example.com` odgovara `blog.example.com`, `forum.example.com`, ali ne i samom `example.com`.

Domen u payload-u je prvi ne-null rezultat od: `domain` polja komentara, URL-a parsiranog prema konfiguraciji domena vašeg tenanta, ili `Page` pretrage po `urlId`.

### Filtriranje po agentu

Polje **Agents** omogućava da se veb-huk pretplati samo na određene agente. Prazno znači "svi agenti". Kada nije prazno, veb-huk se okida samo za agente sa liste.

Ovo je korisno kada imate jedan veb-huk za moderation događaje i drugi za engagement događaje, pri čemu oba usmeravaju ka različitim downstream sistemima.

### Test send

UI za konfiguraciju veb-huka ima dugme **Test send** koje šalje lažni payload na URL, tako da možete proveriti konektivnost, potpisivanje i statusni kod odgovora vašeg endpointa bez čekanja pravog događaja.

### Delivery logovi

Svaka dostava (i svaki ponovni pokušaj) završava na stranici [Webhook Delivery Logs](#webhook-logs), tako da možete videti šta se desilo na žici.

### Potpisivanje

Svaki veb-huk je potpisan sa HMAC-SHA256 koristeći API secret vašeg tenanta. Pogledajte [Webhook Signing](#webhook-signing).

### Uslovi korišćenja

Agent veb-hukovi zahtevaju važeće naplatne podatke na tenantu. Koriste istu infrastrukturu potpisivanja/secret-a kao vaši postojeći veb-hukovi za komentare - ako ste već integrisali comment veb-hukove (pogledajte [Webhooks vodič](/guide-webhooks.html)), integracija agent veb-hukova je istog oblika, samo sa novim tipovima događaja.