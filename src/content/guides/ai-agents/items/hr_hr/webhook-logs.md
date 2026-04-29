Svaki webhook agenta ima svoj vlastiti zapisnik dostave. Dostupan je s [popisa webhookova](https://fastcomments.com/auth/my-account/ai-agents/webhooks) putem gumba **Logs** na svakom redu webhooka.

### Što se nalazi na stranici

Paginirana tablica s jednim redom po pokušaju dostave:

| Stupac | Značenje |
|---|---|
| When | Kada je pokušaj napravljen. |
| Event | Tip događaja (`trigger.succeeded`, `approval.requested`, itd.). |
| Status | Status dostave. |
| StatusCode | HTTP statusni kod koji je vratio vaš endpoint, kada je dostupan. |
| Description | Kratki opis ishoda. Za neuspjele dostave gdje nije primljen HTTP odgovor, osnovna poruka o pogrešci pohranjena je kao `{error: <message>}`. |

Stranica podržava samo paginaciju - nema filtera po statusu, tipu događaja ili rasponu datuma.

### Što možete učiniti iz zapisnika

- **Odlučiti trebaju li neki statusni kodovi biti u Statusni kodovi bez ponovnog pokušaja.** Ako vidite da vaš endpoint vraća isti `4xx` iznova i iznova, to je jak signal da ga želite dodati u **Statusni kodovi bez ponovnog pokušaja** kako bi platforma prestala s ponovnim pokušajima.

### Informacije o pogreškama

Kad dostava zakaže bez HTTP odgovora (DNS neuspjeh, veza odbijena, prekoračenje vremena, TLS pogreška itd.), neobrađena poruka o pogrešci bilježi se kao `{error: <message>}`. Platforma ih ne kategorizira u imenovane kante poput `TIMEOUT` ili `DNS_ERROR` - pročitajte poruku o pogrešci izravno da vidite što se dogodilo.

Za HTTP odgovore, stupac StatusCode prikazuje kod koji je vratio vaš endpoint. Uobičajeni slučajevi:

- **Svi pokušaji: `401` ili `403`** - vaš endpoint odbacuje potpis. Provjerite da izračunavate HMAC preko `${timestamp}.${body}` i koristite ispravan tajni ključ. Vidi [Webhook Signing](#webhook-signing).
- **Svi pokušaji: `422`** - vaš endpoint smatra da je payload nevaljan. Ili popravite endpoint, ili dodajte `422` u **Statusni kodovi bez ponovnog pokušaja** ako je odbacivanje očekivano za neke događaje.

### Kontekst po isporuci

Svaki unos u zapisniku sadrži:

- `webhookId` - koja konfiguracija webhooka je proizvela ovu dostavu.
- `agentId` - o kojem agentu se radi za dostavu.
- `triggerId` ili `approvalId` - temeljni zapis.
- `domain` - podudarani domenski naziv.

Možete ih koristiti za korelaciju neuspjele dostave s pokretanjem na koje se odnosi u [Run History](#run-history).

### Zadržavanje

Unosi `AgentSyncLog` imaju jedinstveni TTL od 1 godine na `createdAt` bez obzira na ishod - uspješne i neuspjele dostave čuvaju se istu duljinu vremena. Nakon isteka zadržavanja, zapis u zapisniku nestaje.

Ako vam treba dugoročna revizija, održiv obrazac je: neka **sam endpoint** pohranjuje dostave koje primi. To odvaja vaš revizijski zapis od politike zadržavanja platforme.

### Testno slanje

Gumb **Test send** u obrascu konfiguracije webhooka upisuje lažnu dostavu u istu tablicu zapisnika tako da možete provjeriti povezanost end-to-end prije oslanjanja na stvarne događaje. Testne dostave su jasno označene u zapisniku kako ne bi zagađivale statistiku grešaka u produkciji.

### Vidi također

- [Pregled Webhookova](#webhooks-overview).
- [Ponavljanja webhooka](#webhook-retries) za semantiku ponovnih pokušaja koja pokreće ove zapisnike.
- [Potpisivanje webhooka](#webhook-signing) za način verifikacije na vašem endpointu.