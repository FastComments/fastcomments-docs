Agent memorija je opsegom stanara (tenant-scoped), **zajednički** ključ-vrijednost spremnik kojem svaki agent u vašem tenant-u može čitati i pisati. Postoji kako bi agenti mogli prenositi kontekst između izvođenja.

### Zašto memorija postoji

Kontext LLM-a je po izvođenju. Bez memorije, agent koji upozori korisnika nema način da sazna za to upozorenje sljedeći put kad vidi istog korisnika. Politika eskalacije platforme - "warn before banning" - ovisi o tome da agent može pronaći prethodno upozorenje. Memorija je ono što to omogućuje.

### Dvije vrste memorije

- **WARNING** - zapisuje se automatski kao dio toka [`warn_user`](#tool-warn-user). Agent ne piše `WARNING` zapise ručno; oni su nuspojava upozoravanja korisnika.
- **NOTE** - zapisuje se pomoću [`save_memory`](#tools-overview). Opći kontekst koji agent želi da budući agenti znaju.

Politika eskalacije posebno traži `WARNING` zapise pri odlučivanju je li zabrana opravdana.

### Opseg stanara, dijeljeno među agentima

Svi agenti u vašem tenant-u dijele **jedan spremnik memorije**. Napomena spremljena od Agenta A vidljiva je pozivima `search_memory` Agenta B. To je namjerno - želite da bilješke agenta za trijažu informiraju odluke agenta moderatora.

`tenantId` postavlja izvršitelj iz vlastitog tenanta agenta - nikada iz argumenata LLM-a - pa su curenja memorije između tenant-a nemoguća po konstrukciji.

### Što sadrži zapis memorije

Svaki unos memorije sadrži:

- **Koji ga je agent napisao**, i kada.
- **O kome je** - korisnik kojeg ova memorija opisuje. Agent to ne može izmišljati; platforma to automatski popunjava iz onoga što je pokrenulo agenta.
- **Skriveni signal alt-naloga** - platforma također (privatno) bilježi IP otisak poruke koja je inicirala zapis, tako da buduće pretrage memorije mogu izložiti bilješke o drugim računima koji objavljuju s iste IP adrese. Otisak nikada nije prikazan agentu ili LLM-u.
- **Sama bilješka** - do 2000 znakova slobodnog teksta.
- **Oznake** za dohvaćanje - do 10 kratkih oznaka.
- **Vrsta** - ili upozorenje ili opća bilješka.
- **Opcionalna poveznica na komentar** - ako je memorija vezana uz određeni komentar.

### Ponašanje pretrage

[`search_memory`](#tools-overview) vraća do 25 zapisa, sortirano po najnovijem-prvo, automatski ograničeno na (korisnika koji je pokrenuo) ILI (druge račune na IP-u koji je pokrenuo). Rezultati su također ograničeni ukupno na 8000 znakova kroz sav vraćeni sadržaj - stariji unosi se odbacuju ako se dosegne ograničenje.

Agent ne prosljeđuje `userId` ili `targetIpHash`. Oba postavlja izvršitelj.

### Trajnost

Memorija nema **TTL**. Zapisi ostaju dok ih se izričito ne ukloni. WARNING zapisi o korisniku namjerno se nikad ne brišu automatski - povijest eskalacije mora se moći pronaći neograničeno ili provjera platforme "pretraži prije zabrane" nema smisla.

Tri načina uklanjanja memorije:

- Moderator izbriše osnovni komentar - svaka memorija vezana uz taj komentar se kaskadno uklanja.
- Korisnik se obriše - svi zapisi memorije o tom korisniku uklanjaju se u istom transakciji.
- Vaš tenant se obriše.

Trenutno ne postoji administratorsko sučelje za brisanje pojedinačnih zapisa memorije.

### Memorija u dry-run načinu

Agenti u dry-run načinu **ne** pišu memoriju. To je po dizajnu: hipotetske odluke dry-run agenta ne bi smjele zagađivati zajednički spremnik memorije. Čitanje putem `search_memory` radi normalno u dry-run načinu - agent može vidjeti stvarne memorije iz živih agenata - samo ne može dodavati u njih.

### Memorija u replayima

Isto kao i dry-run: replay agenti ne pišu memoriju. Replay-i su samo za pregled. Vidi [Test Runs (Replays)](#test-runs-replays).

### Sažetak ograničenja

| Ograničenje | Vrijednost |
|---|---|
| Maksimalna duljina sadržaja memorije | 2000 znakova |
| Maksimalna duljina oznake memorije | 64 znaka |
| Maksimalan broj oznaka memorije | 10 |
| Maksimalna duljina upita memorije | 200 znakova |
| Ograničenje rezultata pretrage memorije | 25 zapisa |
| Ukupni limit sadržaja pretrage memorije | 8000 znakova |

### Pogledaj također

- [Alat: save_memory](#tools-overview) za pisanje.
- [Alat: search_memory](#tools-overview) za čitanje.
- [Tool: warn_user](#tool-warn-user) - jedini alat koji piše memoriju vrste WARNING.
- [Tool: ban_user](#tool-ban-user) - sustavni prompt zahtijeva pozivanje `search_memory` prije ovoga.