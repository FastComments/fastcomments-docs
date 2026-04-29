Svaki agent webhook ima svoj sopstveni zapisnik isporuka. Dostupan je sa [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) preko dugmeta **Logovi** u svakom redu webhook-a.

### Šta se nalazi na stranici

Paginirana tabela sa po jednim redom za svaki pokušaj isporuke:

| Column | Meaning |
|---|---|
| When | Kada se pokušaj dogodio. |
| Event | Tip događaja (`trigger.succeeded`, `approval.requested`, itd.). |
| Status | Status isporuke. |
| StatusCode | HTTP status kod koji je vratio vaš endpoint, kada je dostupan. |
| Description | Kratak opis rezultata. Za neuspešne isporuke gde nije primljen HTTP odgovor, osnovna poruka o grešci se čuva kao `{error: <message>}`. |

Stranica podržava samo paginaciju - nema filtera po statusu, tipu događaja ili vremenskom opsegu.

### Šta možete uraditi iz zapisnika

- **Odlučiti da li status kod treba da bude u No-retry.** Ako vidite da vaš endpoint stalno vraća isti `4xx`, to je snažan signal da želite da ga dodate u **Status kodovi bez ponovnog pokušaja** tako da platforma prestane da pokušava ponovo.

### Informacije o greškama

Kada isporuka ne uspe bez HTTP odgovora (DNS greška, veza odbijena, timeout, TLS greška, itd.), sirova poruka o grešci se beleži kao `{error: <message>}`. Platforma ne kategorizuje ove greške u imenovane grupe poput `TIMEOUT` ili `DNS_ERROR` - pročitajte poruku o grešci direktno da vidite šta se dogodilo.

Za HTTP odgovore, kolona StatusCode pokazuje kod koji je vratio vaš endpoint. Uobičajeni slučajevi:

- **Svi pokušaji: `401` ili `403`** - vaš endpoint odbacuje potpis. Proverite da li računате HMAC nad `${timestamp}.${body}` i da li koristite pravi secret. Pogledajte [Webhook Signing](#webhook-signing).
- **Svi pokušaji: `422`** - vaš endpoint smatra da je payload nevažeći. Ili ispravite vaš endpoint, ili dodajte `422` u **Status kodovi bez ponovnog pokušaja** ako je odbijanje očekivano za neke događaje.

### Kontekst po isporuci

Svaki zapis nosi:

- `webhookId` - koja webhook konfiguracija je proizvela ovu isporuku.
- `agentId` - na koji agent se isporuka odnosi.
- `triggerId` or `approvalId` - osnovni zapis.
- `domain` - poklapani domen.

Možete ih koristiti da povežete neuspešnu isporuku sa izvršavanjem na koje se odnosi u [Run History](#run-history).

### Zadržavanje

`AgentSyncLog` unosi imaju jedinstven rok čuvanja od 1 godine počevši od `createdAt` bez obzira na ishod - uspešne i neuspešne isporuke se čuvaju isti vremenski period. Nakon isteka roka, zapisnik se briše.

Ako vam je potreban dugoročni audit, održivi obrazac je: neka sam **endpoint** persistiira isporuke koje prima. To odvajanje čini vaš audit log nezavisnim od politike zadržavanja platforme.

### Test send

Dugme **Test send** u formi za konfiguraciju webhook-a upisuje lažnu isporuku u istu tabelu zapisnika tako da možete verifikovati povezivost end-to-end pre oslanjanja na stvarne događaje. Test isporuke su jasno označene u zapisniku kako ne bi zagađivale statistiku neuspeha u produkciji.

### Pogledajte takođe

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) za semantiku ponovnih pokušaja koja stoji iza ovih zapisnika.
- [Webhook Signing](#webhook-signing) za način verifikacije na vašem endpoint-u.