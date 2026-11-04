**ID šablona:** `tos_enforcer`

Moderator šablon je preporučena polazna tačka ako je vaš cilj smanjenje ručnog opterećenja moderacije. On pregleda nove i prijavljene komentare i primenjuje pravila vaše zajednice.

Skoro uvek ćete želeti da **obogatite ugrađeni prompt** konkretnim primerima onoga što vaš sajt dozvoljava, a što ne dozvoljava. Sopstvena politika platforme za eskalaciju (upozorenje pre banovanja, pretraga memorije pre banovanja) je već ugrađena u sistemski prompt koji agent prima, tako da je ne morate ponavljati.

### Okidači

- **Novi komentar objavljen** (`COMMENT_ADD`) - agent pregleda svaki novi komentar.
- **Komentar prelazi prag prijave** (`COMMENT_FLAG_THRESHOLD`, podrazumevani prag: 3) - agent ponovo procenjuje komentar koji su drugi korisnici prijavili.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - korisno za tenant-e sa pre-moderacijom gde agent objavljuje čiste komentare i sakriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može da objavljuje komentare, glasa, kači, zaključava, dodeljuje bedževe niti šalje imejl - prompt je namerno uzak.

### Preporučene dopune pre objavljivanja

- **Postavite [Smernice zajednice](#community-guidelines).** Nekoliko rečenica pisane politike je dovoljno; agent ih primenjuje pri svakom pokretanju.
- **Ograničite `ban_user` kroz [odobravanje](#approval-workflow).** Ovo je podrazumevano uključeno u EU regionu (vidi [EU DSA Član 17 Usklađenost](#eu-dsa-compliance)) i preporučeno svuda.
- **Razmotrite i ograničavanje `mark_comment_spam` kroz odobravanje** ako imate sadržaj malog obima ali visokog rizika.
- **Ograničite `mark_comment_approved` kroz odobravanje ako koristite pre-moderaciju.** Odobravanje lošeg komentara stavlja ga pred čitaoce; ograničite ga dok agent ne stekne poverenje kroz dry-run.
- **Čekirajte "Uključi faktor poverenja komentatora, starost naloga, istoriju banovanja i nedavne komentare"** u [Context Options](#context-options). Model će mnogo manje agresivno upozoravati kada vidi da je neko dugogodišnji korisnik dobre volje.

### Preporučeni dry-run period

Pokrenite ovaj šablon u [dry-run](#dry-run-mode) režimu najmanje nedelju dana na vašem stvarnom saobraćaju pre prebacivanja na Enabled. Koristite [Test Runs (Replays)](#test-runs-replays) da pregledate i prethodnih 30 dana.