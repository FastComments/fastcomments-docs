**Template ID:** `tos_enforcer`

Moderator template je preporučena polazna tačka ako vam je cilj smanjiti ručno moderisanje. Pregleda nove i označene komentare i primenjuje pravila vaše zajednice.

Većinu vremena ćete želeti da **dopunite ugrađeni prompt** konkretnim primerima šta vaš sajt dozvoljava, a šta ne. Platformina sopstvena politika eskalacije (upozori pre banovanja, pretraži memoriju pre banovanja) je već ugrađena u sistemski prompt koji agent prima, tako da je ne morate ponavljati.

### Triggers

- **New comment posted** (`COMMENT_ADD`) - agent analizira svaki novi komentar.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - agent ponovo procenjuje komentar koji su drugi korisnici označili.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - korisno za tenant-e sa pre-moderacijom gde agent objavljuje čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasati, zakačiti, zaključavati, dodeljivati značke, niti slati imejl — prompt je namerno ograničen.

### Recommended additions before going live

- **Set [Community Guidelines](#community-guidelines).** Dve-tri rečenice pisane politike su dovoljne; agent ih primenjuje pri svakom izvršavanju.
- **Gate `ban_user` behind [approval](#approval-workflow).** Ovo je podrazumevano uključeno u EU regionu (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i preporučuje se svuda.
- **Consider also gating `mark_comment_spam` behind approval** ako imate nizak obim, ali visok rizik sadržaja.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Odobravanje lošeg komentara stavlja ga pred čitaoce; ograničite to dok agent ne stekne poverenje kroz dry-run.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** u [Context Options](#context-options). Model će mnogo ređe upozoravati kada može da vidi da je neko dugogodišnji korisnik dobre volje.

### Recommended dry-run window

Pokrenite ovaj template u [dry-run](#dry-run-mode) režimu najmanje nedelju dana na stvarnom saobraćaju pre nego što ga prebacite na Enabled. Koristite [Test Runs (Replays)](#test-runs-replays) da takođe pregledate rezultate za prethodnih 30 dana.

---