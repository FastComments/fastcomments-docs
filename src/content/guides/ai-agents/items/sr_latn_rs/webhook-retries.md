Agent vebhukovi ponavljaju pokušaje u slučaju greške. Isporuka je **fire-and-forget iz perspektive agenta** - neuspešna isporuka ne blokira izvršavanje agenta niti poništava bilo koje akcije - a red + cron asinhrono upravlja ponovnim pokušajima.

### Model reda

Svaki događaj se stavlja u red **jednom po odgovarajućem vebhuku**. Dakle, ako imate tri vebhuka pretplaćena na `trigger.succeeded` za dati agent + domen, platforma stavlja tri isporuke u red; svaka se isporučuje i ponavlja nezavisno. Greška na jednom vebhuku nikada ne utiče na ostale.

### Šta se ponavlja

Isporuka se ponavlja kada:

- HTTP zahtev **ne bude dovršen** (DNS greška, odbijena veza, istek vremena).
- HTTP kod odgovora je bilo koji ne-2xx status koji nije na konfigurisanoj listi **No-retry status codes**.

Isporuka se **ne ponavlja** kada:

- Kod odgovora je `2xx` (uspeh).
- Kod odgovora je na konfigurisanoj listi **No-retry status codes**. Podrazumevano je ova lista prazna - svaki ne-2xx se ponavlja.

### Konfigurisanje kodova bez ponavljanja

Forma za konfiguraciju vebhuka ima polje **No-retry status codes** (viševrednosno). Uobičajeni unosi:

- `410` - Gone. Vaš endpoint je trajno premešten ili resurs više ne postoji. Ponavljanje samo rasipava propusnost obe strane.
- `422` - Unprocessable Entity. Vaš endpoint je razumeo sadržaj ali ga je smatrao nevažećim. Ponavljanje sa istim sadržajem će dati isti odgovor.
- `400` - Bad Request, u istom duhu.

Dodavanje koda ovde znači: kada endpoint vrati taj kod, isporuka se označava kao trajno neuspešna i prestaje se sa ponavljanjem.

### Raspored ponavljanja

Pozadinski radnik radi svakih nekoliko sekundi i obrađuje sve isporuke čije je vreme sledećeg pokušaja prošlo.

Posle svake greške, vreme sledećeg pokušaja se pomera unapred sa **linearnim backoff**: čekanje raste kao `60 seconds * attempt count` (tako pokušaj 1 čeka 1 minut, pokušaj 2 čeka 2 minuta, i tako dalje).

Posle 99 neuspešnih pokušaja (ili 3 u lokalnom razvoju), isporuka se odustaje i uklanja iz reda. Unosi u dnevniku isporuke ostaju sačuvani i vidljivi na stranici [Webhook Delivery Logs](#webhook-logs) dok ne isteknu.

### Idempotencija na vašoj strani

Pošto ponavljamo pokušaje, vaš endpoint **mora biti idempotentan**. Isti `triggerId` (ili `approvalId`) može stići više puta. Vaš endpoint treba da:

- Koristi jedinstveni ključ (`triggerId` za događaje okidača, `approvalId` za događaje odobrenja) kao token za deduplikaciju.
- Graciozno prihvata duplikate isporuka (vrati 200 drugi put).

Neidempotentan endpoint će na kraju dvostruko obraditi neke isporuke, posebno tokom prolaznih prekida gde jedan istek vremena ponavlja 30 sekundi kasnije, ali je prvobitni zahtev zapravo uspeo.

### Redosled

Isporuke **nisu strogo uređene**. `trigger.succeeded` i nizvodni `approval.requested` (iz istog pokretanja) mogu stići u bilo kom redosledu ako se jedan ponavlja a drugi ne. Vaš endpoint ne treba da pretpostavlja uzročni redosled.

Ako vam treba redosled, koristite vremenske oznake - `occurredAt` na omotu, plus `createdAt` okidača/odobrenja u bloku podataka - da rekonstruišete redosled na svojoj strani.

### Čišćenje

Isporuke se uklanjaju iz reda čim ili uspeju ili dostignu ograničenje pokušaja. Platforma ne zadržava trajno neuspešne isporuke u samom redu; trajni zapis svakog pokušaja živi na stranici [Webhook Delivery Logs](#webhook-logs).

### Gde gledati kada ponavljanja ne uspevaju

Stranica [Webhook Delivery Logs](#webhook-logs) je mesto za videti zašto vebhuk ne uspeva. Uobičajeni uzroci:

- **Neuspeh DNS rezolucije** - URL je pogrešan ili domen više ne postoji.
- **TLS greške** - sertifikat vašeg endpointa je nevažeći ili istekao.
- **Odbijena veza / istek vremena** - vaš endpoint je nedostupan.
- **5xx odgovori** - vaš endpoint radi ali je došlo do greške. Telo odgovora (skraćeno) se beleži.
- **4xx odgovori** - vaš endpoint je odbio sadržaj. Ako je ovo namerno, dodajte kod u **No-retry status codes**.

### Pauziranje neispravnog vebhuka

Ako vebhuk dosledno ne uspeva, najčistije rešenje je da ga obrišete (ili privremeno ispraznite njegovu listu pretplata na događaje). Platforma ne automatski onemogućava neuspešne vebhuke - oni nastavljaju da ponavljaju pokušaje dok se ne odustane od isporuke.