Agent webhooks ponovo pokušavaju isporuku pri neuspjehu. Isporuka je **send-and-forget iz perspektive agenta** — neuspjela isporuka ne blokira izvršenje agenta niti poništava bilo koje radnje — a red i cron asinhrono pokreću ponovne pokušaje.

### Model reda

Svaki događaj se stavlja u red **jednom po svakom odgovarajućem webhooku**. Dakle, ako imate tri webhooka pretplaćena na `trigger.succeeded` za datog agenta + domen, platforma stavlja u red tri isporuke; svaka se isporučuje i ponovo pokušava nezavisno. Neuspjeh na jednom webhooku nikad ne utiče na druge.

### Šta se ponovo pokušava

Isporuka se ponovo pokušava kada:

- HTTP zahtjev **se ne završi** (DNS greška, konekcija odbijena, timeout).
- HTTP odgovor ima bilo koji status koji nije 2xx i koji NIJE na konfiguriranoj listi **No-retry status codes**.

Isporuka se **ne ponavlja** kada:

- kod odgovora je `2xx` (uspjeh).
- kod odgovora je na konfiguriranoj listi **No-retry status codes**. Po defaultu je ta lista prazna — svaki ne-2xx se ponovo pokušava.

### Konfigurisanje kodova bez ponovnog pokušaja

Forma za konfiguraciju webhooka ima polje **No-retry status codes** (više vrijednosti). Uobičajeni unos:

- `410` - Gone. Vaš endpoint je trajno premješten ili resurs je izgubljen. Ponovno pokušavanje samo troši bandwidth obje strane.
- `422` - Unprocessable Entity. Vaš endpoint je razumio payload ali ga je smatrao nevažećim. Ponovno slanje istog payload-a daje isti odgovor.
- `400` - Bad Request, u istom duhu.

Dodavanje koda ovdje znači: kada endpoint vrati taj kod, označi isporuku kao failed-terminal i prestani sa ponovnim pokušajima.

### Raspored ponovnih pokušaja

Pozadinski worker radi svakih nekoliko sekundi i obrađuje sve isporuke čije je vrijeme sljedećeg pokušaja prošlo.

Nakon svakog neuspjeha, vrijeme sljedećeg pokušaja se pomjera unaprijed sa **linear backoff**: čekanje raste kao `60 seconds * attempt count` (dakle pokušaj 1 čeka 1 minutu, pokušaj 2 čeka 2 minute, i tako dalje).

Nakon 99 neuspješnih pokušaja (ili 3 u lokalnom razvoju), odustaje se od isporuke i uklanja se iz reda. Stavke u zapisniku isporuka ipak ostaju i vidljive su na stranici [Zapisnici isporuke webhookova](#webhook-logs) dok ne isteknu.

### Idempotentnost na vašoj strani

Pošto ponovo pokušavamo, vaš endpoint **mora biti idempotentan**. Isti `triggerId` (ili `approvalId`) može stići više puta. Vaš endpoint treba:

- Koristiti jedinstveni ključ (`triggerId` za trigger događaje, `approvalId` za approval događaje) kao token za dedupliranje.
- Prihvatiti duplikatne isporuke graciozno (vratiti 200 drugi put).

Neidempotentan endpoint će na kraju dvostruko obraditi neke isporuke, naročito tokom privremenih problema gdje jedan timeout ponovi pokušaj nakon 30 sekundi, ali je originalni zahtjev zapravo uspio.

### Redoslijed

Isporuke **nisu strogo poredane**. `trigger.succeeded` i naknadni `approval.requested` (iz istog run-a) mogu stići u bilo kojem redoslijedu ako se jedan ponovo pokuša a drugi ne. Vaš endpoint ne bi trebao pretpostavljati uzročno-posljedični redoslijed.

Ako vam treba redoslijed, koristite vremenske oznake - `occurredAt` na koverti, plus trigger/approval `createdAt` u data bloku - da rekonstruirate redoslijed na vašoj strani.

### Čišćenje

Isporuke se uklanjaju iz reda čim ili uspiju ili dosegnu limit pokušaja. Platforma ne zadržava terminalno neuspjele isporuke u samom redu; trajni zapis svakog pokušaja živi u [Zapisnici isporuke webhookova](#webhook-logs) stranici.

### Gdje gledati kada ponovni pokušaji ne uspiju

Stranica [Zapisnici isporuke webhookova](#webhook-logs) je mjesto gdje možete vidjeti zašto webhook ne uspijeva. Uobičajeni uzroci:

- **DNS greška pri rješavanju** - URL je pogrešan ili domen je nestao.
- **TLS greške** - sertifikat vašeg endpointa je nevažeći ili istekao.
- **Konekcija odbijena / timeout** - vaš endpoint je nedostupan.
- **5xx odgovori** - vaš endpoint radi ali je došlo do greške. Tjelo odgovora (skraćeno) je zabilježeno.
- **4xx odgovori** - vaš endpoint je odbio payload. Ako je to namjerno, dodajte taj kod u **No-retry status codes**.

### Pauzirajte neispravan webhook

Ako webhook konstantno neuspijeva, najčistije rješenje je obrisati ga (ili privremeno očistiti listu pretplate na događaje). Platforma ne onemogućava automatski webhookove koji ne uspijevaju - nastavljaju sa ponovnim pokušajima dok se ne odustane od isporuke.