Okida se kada neto broj glasova komentara dosegne konfigurirani prag. Neto glasovi su `votesUp - votesDown`.

### Potrebna konfiguracija

- **Prag glasova** - cijeli broj >= 1. Okidač se pokreće na glas koji dovede neto glasove točno do tog broja.

Ako je prag 10 i komentar ide s 9 na 10 neto glasova, okidač se pokreće jednom. Ako glas onda poveća broj s 10 na 11, okidač se **ne** pokreće ponovno - ne ponavlja se za svaki dodatni glas iznad praga.

### Kontekst koji agent prima

- Komentar s trenutnim brojem glasova.
- **smjer glasovanja** (`up` or `down`) glasa koji je izazvao prelazak praga.
- Opcionalna povijest teme / korisnika / kontekst stranice prema konfiguraciji.

### Napomena

- Komentar koji dođe do 10, spusti se na 9 i ponovno poraste na 10 pokrenut će okidač dva puta. Ne postoji po-komentar stanje "već pokrenuto" - ako trebate takvu semantiku, neka agent zabilježi [bilješku u memoriji](#tools-overview) pri prvom izvršenju i provjeri je pri sljedećim izvršenjima.
- Prag je uvijek **neto** broj glasova, ne ukupan broj pozitivnih glasova. Komentar s 12 pozitivnih i 2 negativna ima neto 10 i pokreće okidač; onaj s 10 pozitivnih i 0 negativnih također pokreće.
- Mogući su prijelazi izazvani samo negativnim glasovima - komentar koji zbog negativnog glasa padne s 11 na 10 također pokreće okidač. Parametar `voteDirection` u kontekstu govori agentu iz kojeg je smjera došlo prelazak praga.

### Uobičajene upotrebe

- **Pričvršćivanje** - predložak [Top Comment Pinner](#template-top-comment-pinner) je izgrađen oko ovog okidača.
- **Promocija / tijekovi rada za istaknute komentare** - pošaljite događaj putem [Webhooks](#webhooks-overview) kako bi vanjski sustav mogao promovirati komentar negdje drugdje na vašoj stranici.
- **Praćenje angažmana** - zabilježite bilješku u memoriji o korisniku koji je napisao komentar kako bi drugi agenti znali da je proizveo popularan sadržaj.

### Podešavanje

Pravi prag ovisi o zajednici. Promatrajte [Povijest izvođenja](#run-history) nekoliko dana s niskim pragom (5) kako biste vidjeli koliko često se okidač aktivira. Povećajte prag dok se učestalost aktiviranja ne podudara s ritmom koji zapravo želite.