Svaki agent webhook ima svoj zapisnik isporuke. Dostupan je sa [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) putem dugmeta **Zapisi** na svakom redu webhook-a.

### Šta se nalazi na stranici

Paginisana tabela sa jednim redom po pokušaju isporuke:

| Column | Meaning |
|---|---|
| When | Kada se pokušaj dogodio. |
| Event | Tip događaja (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | Status isporuke. |
| StatusCode | HTTP status kod koji je vratio vaš endpoint, kada je dostupan. |
| Description | Kratak opis ishoda. Za neuspešne isporuke gde nije primljen HTTP odgovor, osnovna poruka o grešci je sačuvana kao `{error: <message>}`. |

Stranica podržava samo paginaciju - nema filtera po statusu, tipu događaja ili vremenskom opsegu.

### Šta možete uraditi iz zapisnika

- **Odlučiti da li status kod treba da bude u No-retry.** Ako vidite da vaš endpoint stalno vraća isti `4xx`, to je jak signal da želite da ga dodate u **Status kodovi bez ponovnog pokušaja** tako da platforma prestane sa ponovnim pokušajima.

### Informacije o greškama

Kada isporuka ne uspe bez HTTP odgovora (DNS greška, konekcija odbijena, istekao timeout, TLS greška, itd.), sirova poruka o grešci se beleži kao `{error: <message>}`. Platforma ne kategorizuje ove poruke u imenovane kategorije poput `TIMEOUT` ili `DNS_ERROR` - pročitajte poruku o grešci direktno da vidite šta se desilo.

Za HTTP odgovore, kolona StatusCode prikazuje kod koji je vratio vaš endpoint. Uobičajeni slučajevi:

- **Svi pokušaji: `401` ili `403`** - vaš endpoint odbacuje potpis. Proverite da li računate HMAC preko `${timestamp}.${body}` i koristite ispravan secret. Pogledajte [Webhook Signing](#webhook-signing).
- **Svi pokušaji: `422`** - vaš endpoint smatra da je payload nevalidan. Ili ispravite vaš endpoint, ili dodajte `422` u **Status kodovi bez ponovnog pokušaja** ako je odbijanje očekivano za neke događaje.

### Kontekst po isporuci

Svaki zapis u zapisniku nosi:

- `webhookId` - koja webhook konfiguracija je proizvela ovu isporuku.
- `agentId` - o kojem agentu je isporuka.
- `triggerId` or `approvalId` - osnovni zapis.
- `domain` - podudaran domen.

Možete ih koristiti da povežete neuspešnu isporuku sa izvršenjem na koje se odnosi u [Run History](#run-history).

### Retencija

`AgentSyncLog` unosi imaju jedinstveni TTL od 1 godine na polju `createdAt` bez obzira na ishod - uspešne i neuspešne isporuke se čuvaju istu dužinu vremena. Nakon isteka retencije, zapis u zapisniku nestaje.

Ako vam treba dugoročna revizija, održivi obrazac je: neka sam **endpoint** sačuva isporuke koje primi. To odvaja vaš audit log od politike zadržavanja platforme.

### Test send

Dugme **Testno slanje** na formi konfiguracije webhook-a upisuje lažnu isporuku u istu tabelu zapisnika tako da možete verifikovati konektivnost end-to-end pre nego što se oslonite na stvarne događaje. Test isporuke su jasno označene u zapisniku kako ne bi zagađivale statistiku neuspeha u produkciji.

### Pogledajte i

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) za semantiku ponovnih pokušaja koja upravlja ovim zapisnicima.
- [Webhook Signing](#webhook-signing) za način verifikacije na vašem endpoint-u.