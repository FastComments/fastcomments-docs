**Template ID:** `tos_enforcer`

Predložak moderatora je preporučena polazna tačka ako vam je cilj smanjenje ručnog opterećenja moderacije. On pregledava nove i označene komentare i primjenjuje pravila vaše zajednice.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
Vi ste agent za sprovođenje uslova korišćenja. Pregledajte svaki novi komentar u odnosu na pravila zajednice. Označite jasan spam ili kršenje pravila. Izdajte upozorenja za prvokazna granična sadržaja. Eskalirajte odluke o zabrani samo za ponovljena ili teška kršenja. Ako je komentar očigledno u skladu sa pravilima, odobrite ga kako bi postao vidljiv (relevantno za tenant-e sa pre-moderacijom). Izbjegavajte političke ili subjektivne debate, fokusirajte se na pravila kako su napisana.
[inline-code-end]

Skoro uvijek ćete htjeti da **dopunite ovaj prompt** konkretnim primjerima šta vaš sajt dozvoljava, a šta ne. Platformina vlastita politika eskalacije (upozoriti prije zabrane, pretražiti memoriju prije zabrane) već je ugrađena u sistemski prompt koji agent prima, tako da nije potrebno da je ponavljate.

### Okidači

- **Objavljen je novi komentar** (`COMMENT_ADD`) - agent pregleda svaki novi komentar.
- **Komentar prelazi prag zastavica** (`COMMENT_FLAG_THRESHOLD`, zadani prag: 3) - agent ponovo procjenjuje komentar koji su drugi korisnici označili.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - koristan za tenant-e sa pre-moderacijom gdje agent pušta čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasati, zakačiti, zaključavati, dodjeljivati značke, niti slati e-poštu - prompt je namjerno sužen.

### Preporučena dopuna prije puštanja u rad

- **Postavite [Pravila zajednice](#community-guidelines).** Nekoliko rečenica napisane politike je dovoljno; agent ih primjenjuje pri svakom pokretanju.
- **Ograničite `ban_user` kroz [odobrenje](#approval-workflow).** Ovo je podrazumijevano uključeno u EU regionu (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)) i preporučuje se svuda.
- **Razmotrite takođe ograničavanje `mark_comment_spam` kroz odobrenje** ako imate mali obim ali visok rizik.
- **Ograničite `mark_comment_approved` kroz odobrenje ako koristite pre-moderaciju.** Odobravanje lošeg komentara stavlja ga pred čitaoce; ograničite ga dok agent ne stekne povjerenje kroz probni rad.
- **Označite "Include commenter's trust factor, account age, ban history, and recent comments"** u [Context Options](#context-options). Model će upozoravati mnogo manje agresivno kada može vidjeti da je neko dugogodišnji, dobronamjeran korisnik.

### Preporučeni probni period (dry-run)

Pokrenite ovaj predložak u [dry-run](#dry-run-mode) modu najmanje nedjelju dana protiv vašeg stvarnog saobraćaja prije nego ga prebacite na Omogućen. Koristite [Test Runs (Replays)](#test-runs-replays) da takođe pregledate zadnjih 30 dana.