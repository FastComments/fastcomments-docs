Kada agent stavi odobrenje u red, platforma obaveštava recenzente putem e‑maila. Dva podešavanja na obrascu za uređivanje kontrolišu ovo: **ko** se obaveštava i **koliko često**.

### Ko: režim obaveštavanja

Dva režima:

- **Svi administratori i moderatori** (podrazumevano) – svaki vlasnik naloga, super administrator i administrator moderatora komentara na tenantu je kandidat za recenzenta.
- **Specifični korisnici** – ručno odaberite listu iz dual‑list birača na obrascu za uređivanje.

U svakom slučaju, kandidat za recenzenta mora imati nalog na tenantu i važeću e‑mail adresu da bi primao obaveštenja.

### Koliko često: učestalost po korisniku

Svaki kandidat‑recenzent **svojim profilom** postavlja ličnu učestalost obaveštenja za odobrenja agenata:

- **Odmah** (podrazumevano) – jedan e‑mail po čekajućem odobrenju, poslat čim se odobrenje kreira.
- **Svakog sata** – jedan sažeti e‑mail po satu koji sumira sva odobrenja stavljena u red u tom satu.
- **Dnevno** – jedan sažeti e‑mail na svakih 24 sata.
- **Onemogućeno** – nema e‑mailova. Korisnik i dalje može pregledati odobrenja putem UI‑ja inboxa; samo ne dobija obaveštenja.

Korisnik menja ovo podešavanje na svom profilu, a ne na obrascu za uređivanje agenta. Ovo je namerno – jedan tenant može imati deset agenata, i moderator ne bi trebalo da mora da postavlja svoju preferiranu učestalost za svakog agenta zasebno.

### Cron poslovi koji generišu sažetke

- **`hourly-agent-approval-digest`** – pokreće se svakog sata, grupiše odobrenja stavljena u red od poslednjeg sažetka svakog korisnika, šalje jedan e‑mail po korisniku.
- **`daily-agent-approval-digest`** – isto, dnevno.
- **`agent-approval-reaper`** – uklanja odobrenja starija od 90 dana, bez obzira na stanje.

Cron‑ovi za satni i dnevni sažetak su ograničeni po primaocu: korisnik sa učestalošću po satu obrađuje se od strane satnog crona i preskače se od strane dnevnog (i obrnuto). Korisnici sa učestalošću „odmah“ se obaveštavaju putem koda za kreiranje odobrenja, a ne putem cron‑ova.

### Stanje deduplikacije

Platforma prati koji su korisnici već poslali e‑mail o svakom odobrenju. Kada je korisnik obavešten (odmah ili u sažetku), neće mu se ponovo šalje e‑mail za isto odobrenje – čak i ako promeni učestalost sa „odmah“ na „dnevno“ usred ciklusa.

### Odobravanje iz e‑maila

Svaki e‑mail obaveštenja sadrži link za jednoklikni potpisani login koji vodi recenzenta direktno na stranicu detalja odobrenja, već autentifikovanog. Oni mogu odobriti, odbiti ili otvoriti tok [Refine Prompts](#refining-prompts) odatle.

### Šta ako ne postoje administratori

Ako je `notifyMode` postavljen na `All admins and moderators`, a tenant nema super administratore, administratore moderatora komentara ili vlasnike naloga sa važećim e‑mailovima, platforma zabeleži upozorenje i odobrenje i dalje ulazi u red – samo niko ne bude obavešten o tome. Biće u inboxu dok neko ne pogleda.

Ako je `notifyMode` postavljen na `Specific users`, a vi niste odabrali nijednog korisnika, rezultat je isti.

### Šta ako su obaveštenja o naplati onemogućena

[Budget Alerts](#budget-alerts) – e‑mailovi vezani za budžet – idu billing administratorima **bez obzira na ličnu preferenciju obaveštavanja**. Ovo je namerno: prekoračenja budžeta utiču na trošak, i vlasnik naplate mora da bude obavešten.

Obaveštenja o odobrenjima poštuju samo ličnu postavku učestalosti odobrenja agenata. Ne proveravaju širu opciju odjave od admin obaveštenja – korisnik koji se odjavio od admin obaveštenja i dalje će primati e‑mailove o odobrenjima ako je na listi recenzenata, osim ako je njegova učestalost odobrenja agenata postavljena na **Onemogućeno**.

### Takođe pogledajte

- [Approval Workflow](#approval-workflow) za ceo životni ciklus odobrenja.
- [Refining Prompts](#refining-prompts) za radni tok „Stalno odobravam istu vrstu greške“.