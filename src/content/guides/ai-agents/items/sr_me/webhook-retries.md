Agent webhookovi ponovo pokušavaju pri grešci. Dostava je **fire-and-forget sa stanovišta agenta** - neuspešna dostava ne blokira izvršavanje agenta niti poništava bilo kakve akcije - a red čekanja + cron pokreću ponovne pokušaje asinkrono.

### Model reda

Svaki događaj se stavlja u red **po jednom za svaki odgovarajući webhook**. Dakle, ako imate tri webhooka pretplaćena na `trigger.succeeded` za određenog agenta + domen, platforma stavlja u red tri dostave; svaka se isporučuje i ponovo pokušava nezavisno. Kvar na jednom webhooku nikada ne utiče na ostale.

### Šta se ponovo pokušava

Dostava se ponovo pokušava kada:

- HTTP zahtev **se ne kompletira** (greška DNS-a, konekcija odbijena, istek vremena (timeout)).
- HTTP statusni kod odgovora je bilo koji ne-`2xx` status koji nije u podešenom spisku **statusni kodovi bez ponovnog pokušaja**.

Dostava se **ne ponavlja** kada:

- Kod odgovora je `2xx` (uspeh).
- Kod odgovora se nalazi u podešenom spisku **statusni kodovi bez ponovnog pokušaja**. Po podrazumevanoj vrednosti ovaj spisak je prazan - svaki ne-`2xx` se ponovo pokušava.

### Konfigurisanje kodova bez ponovnog pokušaja

Forma za konfiguraciju webhooka ima polje **statusni kodovi bez ponovnog pokušaja** (višestruka vrednost). Uobičajeni unosi:

- `410` - Gone. Vaš endpoint je trajno premješten ili resurs više ne postoji. Ponovno pokušavanje samo troši propusni opseg obe strane.
- `422` - Unprocessable Entity. Vaš endpoint je razumio payload ali ga je smatrao nevažećim. Ponovno slanje istog payload-a će dati isti odgovor.
- `400` - Bad Request, u istom smislu.

Dodavanje koda ovde znači: kada endpoint vrati taj kod, označi dostavu kao failed-terminal i prestani sa ponovnim pokušajima.

### Raspored ponovnog pokušaja

Pozadinski radnik se pokreće svakih nekoliko sekundi i obrađuje sve dostave čije je vreme sledećeg pokušaja prošlo.

Nakon svakog neuspjeha, vreme sledećeg pokušaja se pomera napred sa **linear backoff**: čekanje raste kao `60 seconds * attempt count` (tako da pokušaj 1 čeka 1 minut, pokušaj 2 čeka 2 minute, i tako dalje).

Nakon 99 neuspješnih pokušaja (ili 3 u lokalnom razvoju), odustaje se od dostave i uklanja iz reda. Unosi u dnevniku dostave ipak ostaju i ostaju vidljivi na stranici [Dnevnici isporuke webhooka](#webhook-logs) dok ne istekne njihov rok.

### Idempotentnost na vašoj strani

Pošto ponavljamo pokušaje, vaš endpoint **mora biti idempotentan**. Isti `triggerId` (ili `approvalId`) može stići više puta. Vaš endpoint bi trebao:

- Koristite jedinstveni ključ (`triggerId` za trigger događaje, `approvalId` za approval događaje) kao token za deduplikaciju.
- Prihvatite duplikate isporuka bez problema (vratite 200 drugi put).

Ne-idempotentan endpoint će na kraju dvaput obraditi neke dostave, posebno tokom privremenih prekida gde jedan timeout ponovo pokušava 30 sekundi kasnije dok je originalni zahtev zapravo uspeo.

### Redoslijed

Dostave **nisu strogo poredane**. `trigger.succeeded` i jedan downstream `approval.requested` (iz iste izvedbe) mogu stići u bilo kojem redu ako se jedan ponovi, a drugi ne. Vaš endpoint ne bi trebao pretpostavljati uzročni redoslijed.

Ako vam treba redoslijed, koristite vremenske oznake - `occurredAt` na koverti (envelope), plus trigger/approval `createdAt` u data bloku - da rekonstrušete red kod sebe.

### Čišćenje

Dostave se uklanjaju iz reda čim uspiju ili dostignu kapu pokušaja. Platforma ne zadržava terminalno neuspjele dostave u samom redu; trajni zapis svakog pokušaja živi na stranici [Dnevnici isporuke webhooka](#webhook-logs).

### Gde pogledati kada ponovni pokušaji ne uspiju

Stranica [Dnevnici isporuke webhooka](#webhook-logs) je mjesto da vidite zašto webhook ne radi. Uobičajeni uzroci:

- **Greška pri DNS rezoluciji** - URL je pogrešan ili domen više ne postoji.
- **TLS greške** - sertifikat vašeg endpointa je nevažeći ili je istekao.
- **Konekcija odbijena / timeout** - vaš endpoint je nedostupan.
- **5xx odgovori** - vaš endpoint je dostupan ali je javio grešku. Tijelo odgovora (skraćeno) je zabilježeno.
- **4xx odgovori** - vaš endpoint je odbio payload. Ako je to namjerno, dodajte kod u **statusni kodovi bez ponovnog pokušaja**.

### Pauziranje nezdravog webhooka

Ako webhook stalno ne uspijeva, najčistije rješenje je obrisati ga (ili privremeno očistiti njegovu listu pretplate na događaje). Platforma ne onemogućava automatski webhookove koji ne uspijevaju - oni nastavljaju sa ponovnim pokušajima dok se od dostave ne odustane.