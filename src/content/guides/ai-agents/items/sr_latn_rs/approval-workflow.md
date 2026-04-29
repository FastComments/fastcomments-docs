Jedno **odobrenje** je poziv alata stavljen u red čekanja koji zahteva da ga čovek odobri ili odbije pre nego što ga platforma izvrši.

### Konfigurisanje odobrenja

Na formularu za uređivanje agenta, sekcija **Odobrenja** navodi svaki alat koji agent sme da pozove (lista dozvoljenih) i omogućava vam da označite one koji moraju biti pregledani od strane čoveka. Neoznačeni alati se izvršavaju odmah. Označeni alati se stavljaju u red čekanja.

Alati koji nisu dozvoljeni se *odbacuju odmah*, ne stavljaju u red čekanja - odobrenja se primenjuju samo na alate koji su inače dozvoljeni.

### Šta se dešava kada se aktivira kontrolisana akcija

1. Agent bira poziv alata (npr. `ban_user`) sa argumentima, obrazloženjem i stepenom poverenja.
2. Umesto izvršavanja, platforma stavlja odobrenje u red čekanja u stanju `PENDING` sa imenom alata, argumentima, obrazloženjem, stepenom poverenja i snimkom konteksta koji opisuje okidač koji ga je proizveo.
3. Obaveštenja se šalju recenzentima (pogledajte [Obaveštenja o odobrenjima](#approval-notifications)).
4. Izvršavanje agenta se završava i beleži - akcija se prikazuje sa **Na čekanju odobrenja** u [Pregledu detalja izvršavanja](#run-detail-view).

### Pregled odobrenja

Pretinac za odobrenja na [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) prikazuje odobrenja koja su na čekanju, odobrena, odbijena i ona kod kojih je izvršenje neuspešno. Za svako:

- **Naziv alata i argumenti** - tačno ono što agent želi da uradi.
- **Obrazloženje agenta** - opravdanje koje je agent dao.
- **Poverenje** - samoprocena poverenja agenta, od 0.0 do 1.0.
- **Snimak konteksta** - komentar, stranica i korisnik na koje je akcija usmerena.

Dva dugmeta: **Odobri** i **Odbaci**. **Odobri** zapravo izvršava alat; **Odbaci** odbacuje zahtev.

### Stanja odobrenja

Odobrenje prolazi kroz sledeća stanja:

| Stanje | Značenje |
|---|---|
| `PENDING` | Na čekanju ljudske odluke. |
| `APPROVED` | Čovek je odobrio i akcija je izvršena. |
| `REJECTED` | Čovek je odbio. Akcija nije izvršena. |
| `EXECUTION_FAILED` | Čovek je odobrio, ali je izvršenje neuspešno (npr. ciljani komentar je već obrisan). |
| `EXECUTING` | Tranzitno: čovek je kliknuo Odobri i akcija se izvršava. Koristi se za serijalizaciju istovremenih klikova na odobri tako da alat sa stvarnim sporednim efektima nikada ne bude pokrenut dva puta. |

Stanje `EXECUTING` je važno kada dva recenzenta istovremeno kliknu na Odobri - jedan "pobeđuje", drugi vidi da je odobrenje već promenilo stanje.

### Šta se čisti

Odobrenja na čekanju ostaju na čekanju dok se ne donese odluka. Automatski ističu nakon **90 dana** od kreiranja. Odobrenja u bilo kom drugom stanju takođe se brišu nakon 90 dana radi higijene skladišta.

Polja odobrenja "decided by" / "decided at" / "executed at" / "execution result" se popunjavaju kako odobrenje prolazi kroz stanja - sve je vidljivo u prikazu detalja pretinca.

### Integracija webhook-a

Dva webhook događaja se okidaju dok se odobrenja menjaju stanje:

- **`approval.requested`** - pri umetanju u `PENDING`.
- **`approval.decided`** - pri prelazu u `APPROVED`, `REJECTED` ili `EXECUTION_FAILED`.

Oba su potpisana kao i svaki drugi webhook. Pogledajte [Webhook Events](#webhook-events) i [Webhook Payloads](#webhook-payloads).

### Ojačavanje: kontrola poznatih alata

Odobrenja odbijaju da stave u red čekanja bilo koje ime alata koje nije prepoznat alat agenta. Ovo je provera odbrane u dubinu: čak i ako neki budući kod prosledi LLM-om izvedeno ime alata u tok odobrenja, taj niz nikada neće završiti kao pozvano odobrenje. Skup poznatih imena alata je isti skup naveden u [Tools Reference](#tools-overview).

### Uobičajeni obrasci ograničavanja

- **Potpuno novi agent za moderaciju** - ograničite `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Pratite pretinac nekoliko nedelja, pa uklonite ograničenja sa alata sa niskom stopom grešaka.
- **Dugoročni agent za moderaciju** - zadržite `ban_user` i sve nepovratne akcije (`deleteAllUsersComments`, `banIP`) ograničene zauvek.
- **EU region** - `ban_user` je zaključan zbog člana 17 bez obzira šta označite. Pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Šta odobrenja **ne** rade

- Ne odlažu ostale pozive alata agenta. Izvršavanje agenta može pozvati više alata, i samo oni pod kontrolom se stavljaju u red čekanja - ostali se izvršavaju normalno.
- Ne poništavaju izvršavanje agenta ako čovek odbije. Deo izvršavanja koji nije pod kontrolom je već završen.

---