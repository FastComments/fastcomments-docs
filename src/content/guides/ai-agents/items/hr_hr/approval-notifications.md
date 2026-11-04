Kada agent stavi odobrenje u red, platforma obavještava recenzente putem e-pošte. Dvije postavke na obrascu za uređivanje kontroliraju ovo: **tko** se obavještava i **koliko često**.

### Tko: način obavještavanja

Dva načina:

- **Svi administratori i moderatori** (zadano) - svaki vlasnik računa, super admin i admin moderator komentara na tenantu je kandidat za recenzenta.
- **Određeni korisnici** - ručno odaberite popis putem dual-list pickera na obrascu za uređivanje.

U svakom slučaju, kandidat-recenzent mora imati račun na tenantu i valjanu adresu e-pošte za primanje obavještenja.

### Koliko često: frekvencija po korisniku

**Vlastiti profil** svakog kandidata-recenzenta postavlja njegovu osobnu frekvenciju obavještenja za odobrenja agenata:

- **Odmah** (zadano) - jedna e-poruka po čekajućem odobrenju, poslana čim se odobrenje stvori.
- **Satno** - jedna sažeta e-poruka po satu koja sažima sva odobrenja stavljena u red tijekom tog sata.
- **Dnevno** - jedna sažeta e-poruka svaka 24 sata.
- **Onemogućeno** - bez e-poruka. Korisnik i dalje može pregledavati odobrenja putem inbox UI-ja; jednostavno ne dobiva obavještenja.

Korisnik mijenja ovu postavku na svom vlastitom profilu, a ne na obrascu za uređivanje agenta. To je namjerno - jedan tenant može imati deset agenata, a moderator ne bi trebao morati postavljati svoju preferiranu frekvenciju na svakom agentu zasebno.

### Cron poslovi koji pokreću sažetke

- **`hourly-agent-approval-digest`** - prolazi svaki sat, grupira odobrenja stavljena u red od posljednjeg sažetka svakog korisnika, šalje jednu e-poruku po korisniku.
- **`daily-agent-approval-digest`** - isto, dnevno.
- **`agent-approval-reaper`** - uklanja odobrenja starija od 90 dana bez obzira na stanje.

Satni i dnevni sažetak cronovi su opsegom vezani po primatelju: korisnik sa satnom frekvencijom obrađuje se satnim cronom i preskače dnevnim (i obrnuto). Korisnici s odmah-frekvencijom obavještavaju se putem koda za stvaranje odobrenja, a ne putem cronova.

### Stanje deduplikacije

Platforma prati koji su korisnici već obavješteni e-poštom o svakom odobrenju. Jednom kada je korisnik obavješten (odmah ili u sažetku), neće ponovno dobiti e-poruku za isto odobrenje - čak i ako promijeni frekvenciju s odmah na dnevno usred ciklusa.

### Odobravanje iz e-pošte

Svaka e-poruka s obavještenjem sadrži jedno-klik link za prijavu s potpisom koji recenzenta vodi izravno na stranicu s detaljima odobrenja, već autentificiranog. Odatle može odobriti, odbiti ili otvoriti tijek [Refine Prompts](#refining-prompts).

### Što ako nema administratora

Ako je `notifyMode` `All admins and moderators`, ali tenant nema super admine, admine moderatore komentara ili vlasnike računa s valjanom e-poštom, platforma zapisuje upozorenje i odobrenje se i dalje stavlja u red - samo nitko ne dobiva obavještenje o njemu. Ostajat će u inboxu dok ga netko slučajno ne pogleda.

Ako je `notifyMode` `Specific users`, ali niste odabrali nijednog korisnika, isti ishod.

### Što ako su obavještenja o naplati onemogućena

[Budget Alerts](#budget-alerts) - e-poruke vezane uz proračun - idu adminima za naplatu **bez obzira na preferenciju obavještenja po korisniku**. To je namjerno: prekoračenja proračuna utječu na troškove, a vlasnik naplate to treba znati.

Obavještenja o odobrenjima poštuju samo postavku frekvencije odobrenja agenata po korisniku. Ne provjeravaju širu odjavu od admin obavještenja - korisnik koji se odjavio od admin obavještenja i dalje će primati e-poruke o odobrenjima ako je na popisu recenzenata, osim ako je njegova frekvencija odobrenja agenata postavljena na **Onemogućeno**.

### Vidi također

- [Approval Workflow](#approval-workflow) za cijeli životni ciklus odobrenja.
- [Refining Prompts](#refining-prompts) za tijek "stalno odobravam istu vrstu greške".