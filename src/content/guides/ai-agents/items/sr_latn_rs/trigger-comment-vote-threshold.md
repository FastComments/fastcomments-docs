Pokreće se kada ukupan broj glasova komentara dostigne podešeni prag. Neto glasovi su `votesUp - votesDown`.

### Potrebna konfiguracija

- **Vote threshold** - ceo broj >= 1. Okidač se aktivira na glas koji dovede neto glasove tačno do ovog broja.

Ako je prag 10 i komentar pređe sa 9 na 10 neto glasova, okidač se aktivira jednom. Ako glas potom podigne broj sa 10 na 11, okidač se **ne** aktivira ponovo - ne ponovo pokreće za svaki dodatni glas preko praga.

### Kontekst koji agent prima

- Komentar, sa trenutnim brojem glasova.
- **vote direction** (`up` or `down`) glasa koji je izazvao prelazak praga.
- Opcionalno thread / user history / page context prema podešavanju.

### Napomene

- Komentar koji poraste do 10, padne nazad na 9, i ponovo poraste na 10 aktiviraće okidač dva puta. Ne postoji stanje „aktivirano jednom“ po komentaru - ako vam treba ta semantika, naterajte agenta da zabeleži [memory note](#tools-overview) pri prvom pokretanju i proveri je pri narednim pokretanjima.
- Prag je uvek **neto** broj glasova, a ne sirovi broj pozitivnih glasova. Komentar sa 12 pozitivnih i 2 negativna ima neto 10 i aktivira okidač; onaj sa 10 pozitivnih i 0 negativnih takođe aktivira.
- Prelazi praga izazvani isključivo negativnim glasom su mogući - komentar koji padne sa 11 na 10 zbog negativnog glasa takođe aktivira okidač. Parametar `voteDirection` u kontekstu govori agentu iz kog smera je došlo do prelaska praga.

### Uobičajena upotreba

- **Prikvačivanje (Pinning)** - [Top Comment Pinner template](#template-top-comment-pinner) je izgrađen oko ovog okidača.
- **Promocija / radni tokovi istaknutih komentara** - pošaljite događaj putem [Webhooks](#webhooks-overview) kako bi eksterni sistem mogao promovisati komentar na drugim mestima na vašem sajtu.
- **Praćenje angažmana** - zabeležite memorijsku napomenu o korisniku koji je napisao komentar tako da drugi agenti znaju da je proizveo popularan sadržaj.

### Podešavanje

Pravi prag zavisi od zajednice. Pratite [Run History](#run-history) nekoliko dana sa niskim pragom (5) da vidite koliko često se aktivira. Povećavajte prag dok učestalost aktiviranja ne odgovara željenom ritmu.