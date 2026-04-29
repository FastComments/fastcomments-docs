An **odobrenje** je pozvana alatka koja je stavljena u red i koja zahtijeva da je osoba odobri ili odbije prije nego što platforma to izvrši.

### Konfigurisanje odobrenja

Na obrascu za uređivanje agenta, sekcija **Odobrenja** navodi svaku alatku koju agent smije pozvati (allowlist) i omogućava vam da označite one koje moraju biti pregledane od strane čovjeka. Neoznačene alatke se izvršavaju odmah. Označene alatke se stavljaju u red.

Alatke koje nisu dozvoljene se *odmah odbijaju*, ne stavljaju u red - odobrenja se primjenjuju samo na alatke koje su inače dozvoljene.

### Šta se dešava kada se aktivira ograničena akcija

1. Agent izabere poziv alatke (npr. `ban_user`) sa argumentima, obrazloženjem i stepenom povjerenja.
2. Umjesto izvršavanja, platforma stavlja odobrenje u red u stanju `PENDING` sa imenom alatke, argumentima, obrazloženjem, stepenom povjerenja i snimkom konteksta koji opisuje okidač koji je to proizveo.
3. Obavještenja se šalju recenzentima (vidi [Approval Notifications](#approval-notifications)).
4. Izvršavanje agenta se završava i bilježi - akcija je prikazana sa **Pending approval** u [Run Detail View](#run-detail-view).

### Pregledanje odobrenja

Inbox za odobrenja na [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) navodi odobrenja u stanju pending, odobrena, odbijena i ona kod kojih je izvršenje zakazalo. Za svako:

- **Ime alatke i argumenti** - tačno ono što agent želi da uradi.
- **Razmišljanje agenta** - obrazloženje koje je agent dao.
- **Povjerenje** - samoprocijenjeni stepen povjerenja agenta, od 0.0 do 1.0.
- **Snimak konteksta** - komentar, stranica i korisnik na koje akcija cilja.

Dvije dugmad: **Odobri** i **Odbaci**. Klik na Odobri zapravo izvršava alatku; Odbaci odbacuje zahtjev.

### Stanja statusa odobrenja

Odobrenje prolazi kroz ova stanja:

| State | Meaning |
|---|---|
| `PENDING` | Čeka ljudsku odluku. |
| `APPROVED` | Čovjek je odobrio i akcija je izvršena. |
| `REJECTED` | Čovjek je odbio. Akcija nije izvršena. |
| `EXECUTION_FAILED` | Čovjek je odobrio ali izvršenje je zakašnjelo (npr. meta komentar je već obrisan). |
| `EXECUTING` | Privremeno: čovjek je kliknuo Odobri i akcija se izvršava. Koristi se za serijalizaciju istovremenih klikova Odobri tako da alatka sa stvarnim sporednim efektima nikada ne bude izvršena dva puta. |

Stanje `EXECUTING` je bitno kada dva recenzenta istovremeno kliknu Odobri - jedan „pobijedi“, drugi vidi da je odobrenje već promijenilo stanje.

### Šta se briše

Pending odobrenja ostaju pending dok se ne odluči. Automatski ističu nakon **90 days** od kreiranja. Odobrenja u bilo kojem drugom stanju takođe se brišu nakon 90 days radi održavanja prostora za pohranu.

Polja odobrenja "decided by" / "decided at" / "executed at" / "execution result" se popunjavaju kako se odobrenje kreće kroz stanja - sve je vidljivo u prikazu detalja u inboxu.

### Integracija webhook-a

Dva webhook događaja se aktiviraju kako se odobrenja mijenjaju:

- **`approval.requested`** - pri umetnutom PENDING.
- **`approval.decided`** - pri prelasku u APPROVED, REJECTED, ili EXECUTION_FAILED.

Oba su potpisana kao i svaki drugi webhook. Vidi [Webhook Events](#webhook-events) i [Webhook Payloads](#webhook-payloads).

### Ojačavanje: provera poznatih alata

Odobrenja ne dozvoljavaju stavljanje u red bilo kojeg imena alatke koje nije prepoznato kao alatka agenta. Ovo je provjera odbrane u dubinu: čak i ako neki budući kod proslijedi LLM‑generisano ime alatke u tok odobravanja, taj string nikada ne može završiti kao stavka u redu odobrenja. Skup poznatih imena alatki isti je skup naveden u [Tools Reference](#tools-overview).

### Uobičajeni obrasci ograničavanja

- **Sasvim novi agent za moderaciju** - ograničite `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Pratite inbox nekoliko sedmica, zatim uklonite ograničenja za alatke sa malim brojem grešaka.
- **Dugoročni agent za moderaciju** - zadržite `ban_user` i sve ireverzibilne akcije (`deleteAllUsersComments`, `banIP`) stalno ograničene.
- **EU region** - `ban_user` je zaključan zbog Article 17 bez obzira šta označite. Vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Šta odobrenja **ne rade**

- Ne odlažu ostale pozive alatki agenta. Izvršavanje agenta može pozvati više alatki, i samo ograničene se stavljaju u red - ostale se izvršavaju normalno.
- Ne poništavaju izvršavanje agenta ako čovjek odbije. Deo izvršavanja koji nije ograničen već je urađen.