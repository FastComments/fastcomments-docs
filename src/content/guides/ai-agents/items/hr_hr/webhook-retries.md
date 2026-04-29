Agent webhookovi pokušavaju ponovno u slučaju neuspjeha. Dostava je **pošalji-i-zaboravi iz perspektive agenta** - neuspjela dostava ne blokira izvršavanje agenta niti poništava bilo kakve radnje - a red čekanja + cron asinkrono upravljaju ponovnim pokušajima.

### Model reda čekanja

Svaki događaj se stavlja u red **jednom po odgovarajućem webhooku**. Dakle, ako imate tri webhooka pretplaćena na `trigger.succeeded` za određenog agenta + domenu, platforma stavlja tri isporuke u red; svaka se isporučuje i ponovno pokušava neovisno. Neuspjeh na jednom webhooku nikada ne utječe na ostale.

### Što se ponovno pokušava

Dostava se ponovno pokušava kada:

- HTTP zahtjev **se ne dovrši** (neuspjeh DNS-a, veza odbijena, timeout).
- HTTP odgovor ima bilo koji status koji nije 2xx i koji nije na konfiguriranoj listi **kodova statusa bez ponovnog pokušaja**.

Dostava se **ne pokušava ponovno** kada:

- kod odgovora je `2xx` (uspjeh).
- kod odgovora je na konfiguriranoj listi **kodova statusa bez ponovnog pokušaja**. Zadano, ova lista je prazna - svaki odgovor koji nije 2xx se ponovno pokušava.

### Konfiguriranje kodova bez ponovnog pokušaja

Obrazac konfiguracije webhooka ima polje **Kodovi statusa bez ponovnog pokušaja** (višestruka vrijednost). Uobičajeni unosi:

- `410` - Gone. Vaš endpoint je trajno premješten ili resurs je nestao. Ponovno pokušavanje samo troši propusnost s obje strane.
- `422` - Unprocessable Entity. Vaš endpoint je razumio payload, ali ga je smatrao nevažećim. Ponovno pokušavanje s istim payloadom dat će isti odgovor.
- `400` - Bad Request, u istom smislu.

Dodavanje koda ovdje znači: kada endpoint vrati taj kod, označi dostavu kao failed-terminal i prestani s ponovnim pokušajima.

### Raspored ponovnih pokušaja

Pozadinski radnik se pokreće svake nekoliko sekundi i obrađuje sve isporuke čije je vrijeme sljedećeg pokušaja prošlo.

Nakon svakog neuspjeha, vrijeme sljedećeg pokušaja se pomiče unaprijed s **linearbackoff-om**: čekanje raste kao `60 seconds * attempt count` (tako da pokušaj 1 čeka 1 minutu, pokušaj 2 čeka 2 minute, i tako dalje).

Nakon 99 neuspjelih pokušaja (ili 3 u lokalnom razvoju), odustaje se od dostave i uklanja iz reda. Unosi u dnevniku isporuke ipak opstaju i ostaju vidljivi na stranici [Dnevnici isporuke webhookova](#webhook-logs) dok ne istekne njihov rok trajanja.

### Idempotentnost na vašoj strani

Budući da pokušavamo ponovno, vaš endpoint **mora biti idempotentan**. Isti `triggerId` (ili `approvalId`) može stići više puta. Vaš endpoint treba:

- Koristiti jedinstveni ključ (`triggerId` za trigger događaje, `approvalId` za approval događaje) kao token za deduplikaciju.
- Prihvatiti duplicirane isporuke bez problema (vratiti 200 drugi put).

Neidempotentan endpoint će na kraju dvaput obraditi neke dostave, osobito tijekom privremenih prekida gdje jedan timeout ponovno pokuša 30 sekundi kasnije, ali je izvorni zahtjev zapravo uspio.

### Redoslijed

Isporuke **nisu strogo poredane**. `trigger.succeeded` i downstream `approval.requested` (iz iste izvedbe) mogu stići u bilo kojem redoslijedu ako se jedan ponovno pokuša, a drugi ne. Vaš endpoint ne bi trebao pretpostavljati kauzalni redoslijed.

Ako vam treba redoslijed, koristite vremenske oznake - `occurredAt` na omotnici, plus `createdAt` triggera/approvala u bloku podataka - za rekonstrukciju redoslijeda na vašoj strani.

### Čišćenje

Isporuke se uklanjaju iz reda čim uspiju ili dosegnu najviši broj pokušaja. Platforma ne zadržava terminalno neuspjele isporuke u samom redu; trajni zapis svakog pokušaja živi u [Dnevnici isporuke webhookova](#webhook-logs).

### Gdje pogledati kada ponovni pokušaji zakažu

Stranica [Dnevnici isporuke webhookova](#webhook-logs) je mjesto gdje možete vidjeti zašto webhook ne uspijeva. Uobičajeni uzroci:

- **Neuspjeh DNS rezolucije** - URL je pogrešan ili domena je nestala.
- **TLS pogreške** - certifikat vašeg endpointa je nevaljan ili istekao.
- **Veza odbijena / timeout** - vaš endpoint je nedostupan.
- **5xx odgovori** - vaš endpoint je dostupan ali je došlo do pogreške. Tijek odgovora (skraćen) se zapisuje.
- **4xx odgovori** - vaš endpoint je odbio payload. Ako je to namjerno, dodajte kod na listu **Kodova statusa bez ponovnog pokušaja**.

### Pauziranje neispravnog webhooka

Ako webhook stalno ne uspijeva, najjednostavnije rješenje je izbrisati ga (ili privremeno očistiti njegovu listu pretplata na događaje). Platforma ne onemogućuje automatski webhookove koji ne uspijevaju - oni nastavljaju s ponovnim pokušajima dok se ne odustane od dostave.