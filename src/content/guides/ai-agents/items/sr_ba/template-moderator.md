**Template ID:** `tos_enforcer`

Šablon moderatora je preporučeni početni izbor ako vam je cilj smanjiti ručno moderiranje. Pregleda nove i označene komentare i primjenjuje pravila vaše zajednice.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt šablona moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Gotovo uvijek ćete htjeti **dopuniti ovaj prompt** konkretnim primjerima šta vaša stranica dozvoljava, a šta ne. Platformina vlastita politika eskalacije (upozori prije zabrane, provjeri memoriju prije zabrane) je već ugrađena u sistemski prompt koji agent prima, tako da je ne trebate ponavljati.

### Okidači

- **Novi komentar poslat** (`COMMENT_ADD`) - agent provjerava svaki novi komentar.
- **Komentar prelazi prag oznaka** (`COMMENT_FLAG_THRESHOLD`, zadani prag: 3) - agent ponovo procjenjuje komentar koji su drugi korisnici označili.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - koristan za tenant-e sa pred-moderiranjem gdje agent objavljuje čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasati, zakačiti, zaključavati, dodjeljivati značke ili slati email - prompt je namjerno sužen.

### Preporučene dopune prije puštanja u rad

- **Postavite [Smjernice zajednice](#community-guidelines).** Nekoliko rečenica pisane politike je dovoljno; agent ih primjenjuje pri svakom pokretanju.
- **Stavite `ban_user` iza [odobrenja](#approval-workflow).** Ovo je podrazumevano uključeno u EU regiji (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i preporučuje se svuda.
- **Razmotrite i stavljanje `mark_comment_spam` iza odobrenja** ako imate mali obim ali visok rizik sadržaja.
- **Stavite `mark_comment_approved` iza odobrenja ako koristite pred-moderiranje.** Odobravanje lošeg komentara stavlja ga pred čitaoce; stavite ga iza odobrenja dok agent ne stekne povjerenje kroz dry-run.
- **Označite opciju "Include commenter's trust factor, account age, ban history, and recent comments" u [Opcijama konteksta](#context-options).** Model će znatno rjeđe upozoravati kada može vidjeti da je neko dugogodišnji korisnik dobre namjere.

### Preporučeno razdoblje dry-run-a

Pokrenite ovaj šablon u [dry-run](#dry-run-mode) modu najmanje jednu sedmicu na stvarnom prometu prije nego ga prebacite na Omogućeno. Koristite [Test Runs (Replays)](#test-runs-replays) da takođe pregledate ponašanje na osnovu prethodnih 30 dana.

---