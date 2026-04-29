Agent memory je tenant-ograničen, **deljeni** key-value pool kojem svaki agent u vašem tenantu može da čita i piše. Postoji da bi agenti mogli da prenose kontekst između izvršavanja.

### Zašto memorija postoji

LLM kontekst je po izvršavanju. Bez memorije, agent koji izda upozorenje korisniku nema način da zna za to upozorenje sledeći put kad vidi istog korisnika. Platformina politika eskalacije - "opomeni pre zabrane" - zavisi od toga da agent može da pronađe prethodno upozorenje. Memorija je ono što to omogućava.

### Dva tipa memorije

- **WARNING** - piše se automatski kao deo toka [`warn_user`](#tool-warn-user). Agent ne piše zapise tipa `WARNING` ručno; oni su sporedni efekat opominjanja korisnika.
- **NOTE** - piše se pomoću [`save_memory`](#tools-overview). Opšti kontekst koji agent želi da budući agenti znaju.

Politika eskalacije konkretno traži zapise tipa `WARNING` kada odlučuje da li je zabrana opravdana.

### Tenant-ograničeno, deljeno među agentima

Svi agenti u vašem tenantu dele **jedan memorijski pool**. Beleška koju sačuva Agent A vidljiva je Agentu B kroz pozive `search_memory`. Ovo je namerno - želite da beleške trijažnog agenta informišu odluke moderatorskog agenta.

`tenantId` se postavlja od strane izvršioca iz samog agenta-tenanta - nikada iz LLM argumenata - tako da su curenja memorije između tenanta nemoguća po konstrukciji.

### Šta se nalazi u zapisu memorije

Svaki unos u memoriji sadrži:

- **Ko ga je napisao**, i kada.
- **O kome je** - korisnik kojeg ova memorija opisuje. Agent to ne može da izmisli; platforma to popunjava automatski iz onoga što je pokrenulo agenta.
- **Skriveni signal alternativnog naloga** - platforma takođe beleži (privatno) IP otisak komentara odakle potiče, tako da buduće pretrage memorije mogu da prikažu beleške o drugim nalozima koji su postovali sa iste IP adrese. Otisak se nikada ne prikazuje agentu ili LLM-u.
- **Sama beleška** - do 2000 karaktera slobodnog teksta.
- **Tagovi** za pretragu - do 10 kratkih tagova.
- **Tip** - ili upozorenje ili opšta beleška.
- **Opcioni link ka komentaru** - ako je memorija vezana za konkretan komentar.

### Ponašanje pretrage

[`search_memory`](#tools-overview) vraća do 25 zapisa, sortirano od najnovijih ka najstarijim, automatski ograničeno na (korisnika koji je pokrenuo) ILI (druge naloge na IP-u koji je pokrenuo). Rezultati su takođe ograničeni na ukupno 8000 karaktera kroz sav vraćeni sadržaj - stariji unosi se izbacuju ako se limit dostigne.

Agent ne prosleđuje `userId` ili `targetIpHash`. Oba se postavljaju od strane izvršioca.

### Persistencija

Memorija nema **TTL**. Zapisi traju dok se eksplicitno ne uklone. WARNING zapisi o korisniku namerno se nikada ne obrišu automatski - istorija eskalacija mora biti moguće pronaći neograničeno ili platformina provera "pretraga pre zabrane" nema smisla.

Tri načina na koja se memorija uklanja:

- Moderator obriše osnovni komentar - sva memorija vezana za taj komentar se kaskadno izbriše.
- Korisnik se obriše - svi zapisi memorije o tom korisniku se uklanjaju u istoj transakciji.
- Vaš tenant se obriše.

Danas ne postoji administratorski UI za brisanje pojedinačnih zapisa memorije.

### Memorija u dry-run režimu

Dry-run agenti NE pišu memoriju. Ovo je po dizajnu: hipotetičke odluke dry-run agenta ne bi trebalo da zagađuju deljeni memorijski pool. Čitanje kroz `search_memory` radi normalno u dry-run režimu - agent može da vidi prave memorije iz live agenata - samo ne može da ih dopuni.

### Memorija u replay-ovima

Isto kao dry-run: replay agenti ne pišu memoriju. Replays su samo pregled. Pogledajte [Test Runs (Replays)](#test-runs-replays).

### Rezime ograničenja

| Ograničenje | Vrednost |
|---|---|
| Maksimalna dužina sadržaja memorije | 2000 karaktera |
| Maksimalna dužina taga memorije | 64 karaktera |
| Maksimalan broj tagova memorije | 10 |
| Maksimalna dužina upita memorije | 200 karaktera |
| Limit rezultata pretrage memorije | 25 zapisa |
| Ukupni limit sadržaja pretrage memorije | 8000 karaktera |

### Pogledajte takođe

- [Tool: save_memory](#tools-overview) za upis.
- [Tool: search_memory](#tools-overview) za čitanje.
- [Tool: warn_user](#tool-warn-user) - jedini alat koji piše memoriju tipa WARNING.
- [Tool: ban_user](#tool-ban-user) - sistemski prompt zahteva pozivanje `search_memory` pre ovog.