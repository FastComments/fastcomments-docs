Svaki agent webhook ima svoj dnevnik isporuke. Dostupan sa [stranice liste webhookova](https://fastcomments.com/auth/my-account/ai-agents/webhooks) putem dugmeta **Logs** u redu svakog webhooka.

### Šta se nalazi na stranici

Paginirana tabela sa jednim redom po pokušaju isporuke:

| Kolona | Značenje |
|---|---|
| When | Kada se pokušaj desio. |
| Event | Tip događaja (`trigger.succeeded`, `approval.requested`, itd.). |
| Status | Status isporuke. |
| StatusCode | HTTP status kod koji je vratio vaš endpoint, kada je dostupan. |
| Description | Kratak opis ishoda. Za neuspele isporuke gde nije primljen HTTP odgovor, osnovna poruka o grešci se čuva kao `{error: <message>}`. |

Stranica podržava samo paginaciju - nema filtera po statusu, tipu događaja ni datumskom opsegu.

### Šta možete da radite iz logova

- **Odlučite da li neki status kod treba da bude u No-retry.** Ako vidite da vaš endpoint stalno vraća isti `4xx`, to je jak signal da ga dodate u **No-retry status codes** kako platforma ne bi nastavila sa ponovnim pokušajima.

### Informacije o greškama

Kada isporuka ne uspe bez HTTP odgovora (DNS greška, odbijena konekcija, timeout, TLS greška, itd.), sirova poruka o grešci se zapisuje kao `{error: <message>}`. Platforma ih ne kategorizuje u imenovane kategorije poput `TIMEOUT` ili `DNS_ERROR` - pročitajte poruku o grešci direktno da biste videli šta se desilo.

Za HTTP odgovore, kolona StatusCode prikazuje kod koji je vratio vaš endpoint. Česti slučajevi:

- **Svi pokušaji: `401` ili `403`** - vaš endpoint odbija potpis. Proverite da računate HMAC preko `${timestamp}.${body}` i da koristite pravi secret. Vidite [Webhook Signing](#webhook-signing).
- **Svi pokušaji: `422`** - vaš endpoint smatra da je payload nevalidan. Ili ispravite svoj endpoint, ili dodajte `422` u **No-retry status codes** ako je odbijanje očekivano za neke događaje.

### Kontekst po isporuci

Svaki unos u logu sadrži:

- `webhookId` - koja webhook konfiguracija je proizvela ovu isporuku.
- `agentId` - o kom agentu je reč.
- `triggerId` ili `approvalId` - osnovni zapis.
- `domain` - podudarni domen.

Možete ih koristiti da povežete neuspelu isporuku sa povezanim izvršavanjem u [Run History](#run-history).

### Zadržavanje podataka

Unosi u `AgentSyncLog` imaju fiksni TTL od 1 godine na `createdAt` bez obzira na ishod - uspešne i neuspele isporuke se čuvaju isto vreme. Nakon isteka retencije, unos u logu nestaje.

Ako vam treba dugoročna revizija, održiv obrazac je: neka **sam endpoint** trajno čuva isporuke koje prima. Time se vaš revizioni log odvaja od politike zadržavanja platforme.

### Test slanje

Dugme **Test send** u obrascu za konfiguraciju webhooka upisuje lažnu isporuku u istu tabelu loga kako biste proverili konektivnost od kraja do kraja pre nego što se oslonite na prave događaje. Test isporuke su jasno označene u logu kako ne bi zagađivale statistike grešaka u produkciji.

### Vidite takođe

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) za semantiku ponovnih pokušaja koja pokreće ove logove.
- [Webhook Signing](#webhook-signing) za to kako da verifikujete na svom endpointu.