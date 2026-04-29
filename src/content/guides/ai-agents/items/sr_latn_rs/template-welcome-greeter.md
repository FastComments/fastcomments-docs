**ID šablona:** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt šablona Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
Ti si srdačan pozdravljač zajednice. Odgovaraj korisnicima koji prvi put komentarišu kratkom, ličnom porukom dobrodošlice. Pomenu jednu konkretnu stvar iz njihovog komentara, kako ne bi zvučalo kao šablon. Ograniči odgovore na 1-2 rečenice. Nikada ne odgovaraj nalozima starijim od 24 sata.
[inline-code-end]

### Okidači

- **Novi korisnik objavi svoj prvi komentar na ovom sajtu** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se pokreće tačno jednom po korisniku, tako da agent ne može ući u petlju. Vidi [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat - agent bukvalno ne može da moderira, glasa, zabrani korisnike ili šalje DM.

### Preporučene dopune pre puštanja uživo

- **Podesi prikazano ime** na nešto pozivajuće - "Community Bot", maskota vašeg sajta, ili naziv vašeg brenda. Prikazano ime je ono što čitaoci vide uz odgovor dobrodošlice.
- **Označi "Include page title, subtitle, description, and meta tags"** u [Context Options](#context-options). Odgovori greetera postaju primetno bolji kada može da se pozove na to o čemu stranica zapravo govori.
- **Razmotrite ograničenja lokala** ako poslujete na više jezika. Poruka dobrodošlice na pogrešnom jeziku je neprijatnija od propuštenog odgovora. Vidi [Scope: URL and Locale Filters](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent piše samo nove komentare i aktivira se samo na jednokratni okidač. U najgorem slučaju: nezgrapan pozdrav. Ne postoji destruktivna radnja koju treba kontrolisati. Većina operatora pokreće ovog agenta bez ikakvih odobrenja čim probni rad izgleda uredno.