An **odobrenje** je poziv alata stavljen u red čekanja koji zahtijeva da osoba odobri ili odbije prije nego što ga platforma izvrši.

### Konfigurisanje odobrenja

Na formi za uređivanje agenta, sekcija **Odobrenja** navodi svaki alat koji agent smije pozivati (dozvoljena lista) i omogućava vam da označite one koji moraju biti pregledani od strane osobe. Neoznačeni alati se izvršavaju odmah. Označeni alati se stavljaju u red čekanja.

Zabranjeni alati se *odmah odbijaju*, ne stavljaju u red čekanja - odobrenja se primjenjuju samo na alate koji su inače dozvoljeni.

### Šta se dešava kada se pokrene ograničena radnja

1. Agent izabere poziv alata (npr. `ban_user`) sa argumentima, opravdanjem i povjerenjem.
2. Umjesto izvršavanja, platforma stavlja odobrenje u red čekanja sa statusom `PENDING`, zajedno sa imenom alata, argumentima, opravdanjem, povjerenjem i snimkom konteksta koji opisuje okidač koji ga je proizveo.
3. Notifikacije se šalju recenzentima (vidi [Notifikacije odobrenja](#approval-notifications)).
4. Izvršavanje agenta se završava i bilježi - radnja je prikazana sa **Pending approval** u [Prikazu detalja izvršavanja](#run-detail-view).

### Pregledanje odobrenja

Pretinac odobrenja na [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) navodi odobrenja koja su u statusu čekanja, odobrena, odbijena i ona za koja je izvršenje neuspješno. Za svako:

- **Ime alata i argumenti** - tačno ono što agent želi da uradi.
- **Obrazloženje agenta** - opravdanje koje je agent dostavio.
- **Povjerenje** - samoprocijenjeno povjerenje agenta, od 0.0 do 1.0.
- **Snimak konteksta** - komentar, stranica i korisnik na koje se radnja odnosi.

Dvije dugmad: **Odobri** i **Odbij**. Odobri zapravo izvršava alat; Odbij odbacuje.

### Stati odobrenja

Odobrenje prolazi kroz ove statuse:

| State | Meaning |
|---|---|
| `PENDING` | Čeka ljudsku odluku. |
| `APPROVED` | Osoba je odobrila i radnja je izvršena. |
| `REJECTED` | Osoba je odbila. Radnja nije izvršena. |
| `EXECUTION_FAILED` | Osoba je odobrila, ali izvršenje je neuspjelo (npr., ciljani komentar je već obrisan). |
| `EXECUTING` | Privremeno stanje: osoba je kliknula Odobri i radnja se izvršava. Koristi se za serijalizaciju istovremenih klikova Odobri kako bi alat sa stvarnim sporednim efektima nikada nije bio izvršen dvaput. |

Stanje `EXECUTING` je važno kada dva recenzenta istovremeno kliknu Odobri - jedan "pobjeđuje", drugi vidi da je odobrenje već promijenilo stanje.

### Šta se briše

Odobrenja u statusu čekanja ostaju u čekanju dok se ne donese odluka. Automatski ističu nakon **90 dana** od kreiranja. Odobrenja u bilo kojem drugom statusu takođe se brišu nakon 90 dana radi urednosti skladištenja.

Polja odobrenja "odlučio" / "odlučeno u" / "izvršeno u" / "rezultat izvršenja" se popunjavaju kako se odobrenje kreće kroz statuse - sve je vidljivo u prikazu detalja pretinca.

### Integracija webhooks

Dva webhook događaja se okidaju dok se odobrenja mijenjaju:

- **`approval.requested`** - pri umetku u stanje PENDING.
- **`approval.decided`** - pri tranziciji u APPROVED, REJECTED, ili EXECUTION_FAILED.

Oba su potpisana kao i svaki drugi webhook. Vidi [Webhook Events](#webhook-events) i [Webhook Payloads](#webhook-payloads).

### Ojačavanje: provjera poznatih alata

Odobrenja odbijaju da stave u red bilo koje ime alata koje nije prepoznati agentov alat. Ovo je provjera dubinske odbrane: čak i ako neki budući put prolaza koda proslijedi ime alata izvedeno od LLM-a u tok odobravanja, taj niz nikada ne može završiti kao odobrenje u redu čekanja. Skup poznatih imena alata je isti skup naveden u [Referenci alata](#tools-overview).

### Uobičajeni obrasci ograničavanja

- **Potpuno novi agent za moderaciju** - ograničite `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Pratite pretinac nekoliko sedmica, zatim uklonite ograničenja sa alata koji rijetko prave greške.
- **Dugoročni agent za moderaciju** - zadržite `ban_user` i sve nepovratne radnje (`deleteAllUsersComments`, `banIP`) ograničenima zauvijek.
- **EU regija** - `ban_user` je uključen po članu 17 bez obzira na to šta označite. Vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Šta odobrenja **ne rade**

- Ne odlažu druge pozive alata agenta. Izvršavanje agenta može pozvati više alata, i samo oni koji su ograničeni se stavljaju u red čekanja - ostali se izvršavaju normalno.
- Ne poništavaju izvršavanje agenta ako osoba odbije. Deo izvršavanja koji nije bio ograničen je već završen.