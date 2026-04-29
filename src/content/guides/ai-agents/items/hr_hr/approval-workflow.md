An **odobrenje** je pozvana naredba alata stavljena u red koja zahtijeva da čovjek odobri ili odbije prije nego što je platforma izvrši.

### Konfiguriranje odobrenja

Na obrascu za uređivanje agenta, odjeljak **Odobrenja** navodi svaki alat koji agent smije pozivati (lista dopuštenih) i omogućuje vam označiti one koji moraju biti pregledani od strane čovjeka. Alati koji nisu označeni izvršavaju se odmah. Označeni alati stavljaju se u red.

Alati koji nisu dopušteni *odbijaju se odmah*, ne stavljaju se u red - odobrenja se primjenjuju samo na alate koji su inače dopušteni.

### Što se događa kada se pokrene ograničena radnja

1. Agent bira poziv alata (npr. `ban_user`) s argumentima, obrazloženjem i povjerenjem.
2. Umjesto izvršavanja, platforma stavlja odobrenje u red u stanju `PENDING` s nazivom alata, argumentima, obrazloženjem, povjerenjem i snimkom konteksta koja opisuje okidač koji ga je stvorio.
3. Obavijesti se šalju recenzentima (vidi [Obavijesti o odobrenju](#approval-notifications)).
4. Pokretanje agenta se dovršava i bilježi - radnja se prikazuje s **Čeka odobrenje** u [Prikazu detalja pokretanja](#run-detail-view).

### Pregled odobrenja

Inbox za odobrenja na [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) navodi odobrenja u stanju na čekanju, odobrena, odbijena i s neuspjelim izvršenjem. Za svako:

- **Naziv alata i argumenti** - točno ono što agent želi učiniti.
- **Obrazloženje agenta** - opravdanje koje je agent naveo.
- **Povjerenje** - samoprocijenjeno povjerenje agenta, od 0.0 do 1.0.
- **Snimka konteksta** - komentar, stranica i korisnik na koje se radnja odnosi.

Dvije tipke: **Odobri** i **Odbaci**. Odobri zapravo izvršava alat; Odbaci odbacuje zahtjev.

### Stanja odobrenja

Odobrenje prolazi kroz sljedeća stanja:

| Stanje | Značenje |
|---|---|
| `PENDING` | Čeka ljudsku odluku. |
| `APPROVED` | Ljudsko odobrenje; radnja je izvršena. |
| `REJECTED` | Ljudsko odbijanje. Radnja nije izvršena. |
| `EXECUTION_FAILED` | Ljudsko odobrenje je dano, ali izvršenje je nije uspjelo (npr. ciljani komentar je već izbrisan). |
| `EXECUTING` | Privremeno: osoba je kliknula Odobri i radnja je u tijeku. Koristi se za serijalizaciju istovremenih klikova Odobri kako bi alat sa stvarnim nuspojavama nikada ne bio pokrenut dvaput. |

Stanje `EXECUTING` je važno kada dva recenzenta istovremeno kliknu Odobri - jedan "pobjeđuje", a drugi vidi da je odobrenje već promijenilo stanje.

### Što se čisti

Odobrenja u stanju na čekanju ostaju na čekanju dok se ne odluči. Oni automatski istječu **90 dana** nakon kreiranja. Odobrenja u bilo kojem drugom stanju također se brišu nakon 90 dana radi urednosti pohrane.

Polja odobrenja "odlučio/la" / "odlučeno u" / "izvršeno u" / "rezultat izvršenja" popunjavaju se kako se odobrenje kreće kroz stanja - sve je vidljivo u prikazu detalja u inboxu.

### Integracija putem webhooka

Dva webhook događaja se aktiviraju kako se odobrenja mijenjaju:

- **`approval.requested`** - pri umetku u PENDING.
- **`approval.decided`** - pri prijelazu u APPROVED, REJECTED ili EXECUTION_FAILED.

Oba su potpisana kao i svi ostali webhookovi. Vidi [Webhook Events](#webhook-events) i [Webhook Payloads](#webhook-payloads).

### Ojačavanje: provjera poznatog alata

Odobrenja odbijaju stavljanje u red bilo kojeg naziva alata koji nije prepoznat alat agenta. Ovo je provjera obrane u dubinu: čak i ako neki budući kod proslijedi LLM-om generirani naziv alata u tijek odobrenja, taj niz nikada ne može završiti kao odobrenje u redu. Skup poznatih naziva alata isti je skup naveden u [Referenci alata](#tools-overview).

### Uobičajeni obrasci ograničavanja

- **Potpuno nov agent za moderaciju** - zaštitite `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Pratite inbox nekoliko tjedana, zatim uklonite ograničenje za alate s malom učestalošću pogrešaka.
- **Dugoročni agent za moderaciju** - zadržite `ban_user` i sve nepovratne radnje (`deleteAllUsersComments`, `banIP`) uvijek zaštićene.
- **Regija EU** - `ban_user` je zaključan zbog Članka 17 bez obzira što označite. Vidi [Usklađenost s EU DSA člankom 17](#eu-dsa-compliance).

### Što odobrenja ne rade

- Ne odgađaju ostale pozive alata agenta. Pokretanje agenta može pozvati više alata, i samo ograničeni alati idu u red - ostali se izvršavaju normalno.
- Ne poništavaju pokretanje agenta ako čovjek odbije. Neograničeni dio pokretanja je već izvršen.