**ID šablona:** `tos_enforcer`

Moderator šablon je preporučena početna tačka ako vam je cilj smanjiti ručno moderisanje. Pregleda nove i označene komentare i primenjuje pravila vaše zajednice.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt šablona moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

You will almost always want to **augment this prompt** with concrete examples of what your site does and does not allow. The platform's own escalation policy (warn before ban, search memory before banning) is already baked into the system prompt the agent receives, so you do not need to repeat it.

### Okidači

- **Novi komentar objavljen** (`COMMENT_ADD`) - agent pregleda svaki novi komentar.
- **Komentar prelazi prag označavanja** (`COMMENT_FLAG_THRESHOLD`, podrazumevani prag: 3) - agent ponovo ocenjuje komentar koji su označili drugi korisnici.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - koristan za tenant-e sa pre-moderacijom gde agent objavljuje čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasati, prikvačiti, zaključavati, dodeljivati značke ili slati e-poštu - prompt je namerno ograničen.

### Preporučeni dodaci pre puštanja u rad

- **Podesite [Smernice zajednice](#community-guidelines).** Dovoljno je nekoliko rečenica pisane politike; agent ih primenjuje pri svakom pokretanju.
- **Zahtevajte [odobrenje](#approval-workflow) za `ban_user`.** Ovo je podrazumevano uključeno u EU regionu (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i preporučuje se svuda.
- **Razmislite i o tome da zahtevate odobrenje za `mark_comment_spam`** ako imate mali obim, ali sadržaje visokog rizika.
- **Zahtevajte odobrenje za `mark_comment_approved` ako koristite pre-moderaciju.** Odobravanje lošeg komentara dovodi do toga da bude vidljiv čitaocima; zahtevajte odobrenje dok agent ne stekne poverenje kroz dry-run.
- **Označite "Uključi faktor poverenja komentatora, starost naloga, istoriju zabrana i nedavne komentare"** u [Opcijama konteksta](#context-options). Model će znatno ređe upozoravati kada može da vidi da je neko dugogodišnji korisnik u dobroj nameri.

### Preporučeni period rada u dry-run režimu

Pokrenite ovaj šablon u [dry-run](#dry-run-mode) režimu najmanje nedelju dana protiv vašeg stvarnog saobraćaja pre nego što ga prebacite na Omogućeno. Koristite [Test pokretanja (Replays)](#test-runs-replays) da takođe pregledate za prethodnih 30 dana.

---