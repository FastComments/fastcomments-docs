**Template ID:** `tos_enforcer`

Moderator šablon je preporučena polazna tačka ako je vaš cilj smanjenje ručnog opterećenja moderacije. On pregledava nove i označene komentare i primenjuje pravila vaše zajednice.

Gotovo uvek ćete želeti da **dopunite ugrađeni prompt** konkretnim primerima šta vaš sajt dozvoljava, a šta ne. Politika eskalacije same platforme (upozori pre zabrane, pretraži memoriju pre zabranjivanja) već je ugrađena u sistemski prompt koji agent prima, pa nije potrebno da je ponavljate.

### Triggers

- **Novi komentar postavljen** (`COMMENT_ADD`) – agent gleda svaki novi komentar.
- **Komentar pređe prag za označavanje** (`COMMENT_FLAG_THRESHOLD`, podrazumevani prag: 3) – agent ponovo procenjuje komentar koji su drugi korisnici označili.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) – korisno za pre-moderacione najamnike gde agent objavljuje čiste komentare i sakriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može da objavljuje komentare, glasa, zakači, zaključava, dodeljuje značke ili šalje e‑mail – prompt je namerno sužan.

### Recommended additions before going live

- **Postavite [Community Guidelines](#community-guidelines).** Nekoliko rečenica pisane politike je dovoljno; agent je primenjuje pri svakom pokretanju.
- **Postavite `ban_user` iza [odobrenja](#approval-workflow).** Ovo je podrazumevano uključeno u EU regionu (vidite [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i preporučuje se svuda.
- **Razmotrite takođe postavljanje `mark_comment_spam` iza odobrenja** ako imate sadržaj niskog obima, ali visokog značaja.
- **Postavite `mark_comment_approved` iza odobrenja ako koristite pre-moderaciju.** Odobravanje lošeg komentara stavlja ga pred čitaoce; postavite ga iza odobrenja dok agent ne stekne poverenje kroz dry‑run.
- **Označite „Uključi faktor poverenja komentatora, starost naloga, istoriju zabrana i nedavne komentare“** u [Context Options](#context-options). Model će upozoravati mnogo manje agresivno kada vidi da je neko dugogodišnji korisnik dobre volje.

### Recommended dry-run window

Pokrenite ovaj šablon u [dry-run](#dry-run-mode) najmanje nedelju dana na vašem stvarnom saobraćaju pre nego što ga prebacite na Enabled. Koristite [Test Runs (Replays)](#test-runs-replays) da takođe pregledate poslednjih 30 dana.