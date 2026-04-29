**ID predloška:** `tos_enforcer`

Predložak Moderator je preporučena polazna tačka ako vam je cilj smanjenje ručnog opterećenja moderacije. Pregleda nove i označene komentare i primenjuje pravila vaše zajednice.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

U gotovo svim slučajevima ćete želeti da **dopunite ovaj prompt** konkretnim primerima šta vaš sajt dozvoljava, a šta ne. Platformina sopstvena politika eskalacije (upozoriti pre zabrane, pretražiti memoriju pre zabrane) već je ugrađena u sistemski prompt koji agent prima, tako da je ne morate ponavljati.

### Okidači

- **Novi komentar objavljen** (`COMMENT_ADD`) - agent pregleda svaki novi komentar.
- **Komentar prelazi prag za označavanje** (`COMMENT_FLAG_THRESHOLD`, podrazumevani prag: 3) - agent ponovo procenjuje komentar koji su označili drugi korisnici.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - koristan za tenant-e koji rade sa pre-moderacijom gde agent pušta čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasati, zakačiti, zaključavati, dodeljivati značke ili slati email - prompt je namerno sužen.

### Preporučeni dodaci pre puštanja u rad

- **Postavite [Pravila zajednice](#community-guidelines).** Nekoliko rečenica pisane politike je dovoljno; agent ih primenjuje pri svakom pokretanju.
- **Ograničite `ban_user` iza [odobrenja](#approval-workflow).** Ovo je podrazumevano uključeno u EU regionu (vidi [Usklađenost sa EU DSA članom 17](#eu-dsa-compliance)) i preporučeno svuda.
- **Razmotrite i ograničavanje `mark_comment_spam` iza odobrenja** ako imate nizak obim ali visok stejk sadržaja.
- **Ograničite `mark_comment_approved` iza odobrenja ako koristite pre-moderaciju.** Odobravanje lošeg komentara stavlja ga pred čitaoce; ograničite to dok agent ne stekne poverenje kroz probni režim.
- **Označite "Uključi faktor poverenja autora komentara, starost naloga, istoriju zabrana i nedavne komentare"** u [Opcijama konteksta](#context-options). Model će upozoravati znatno ređe kada može da vidi da je neko dugogodišnji korisnik dobronameran.

### Preporučeni probni period

Pokrenite ovaj predložak u [probnom režimu (dry-run)](#dry-run-mode) najmanje nedelju dana protiv vašeg stvarnog saobraćaja pre nego što ga prebacite u Enabled. Koristite [Test pokretanja (Replays)](#test-runs-replays) da takođe pregledate poslednjih 30 dana.

---