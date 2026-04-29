**ID predloška:** `tos_enforcer`

Predložak Moderator je preporučena polazna točka ako vam je cilj smanjiti opterećenje ručnog moderiranja. Pregledava nove i označene komentare te primjenjuje pravila vaše zajednice.

Gotovo uvijek ćete htjeti **nadograditi ugrađeni prompt** konkretnim primjerima što vaš sajt dopušta, a što ne. Sama platforma ima ugrađenu politiku eskalacije (upozoriti prije zabrane, pretražiti memoriju prije zabrane) koja je već uključena u sistemski prompt koji agent prima, tako da je ne morate ponavljati.

### Okidači

- **New comment posted** (`COMMENT_ADD`) - agent pregleda svaki novi komentar.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, zadani prag: 3) - agent ponovo evaluira komentar koji su drugi korisnici prijavili.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - korisno za zakupce koji koriste pre-moderaciju gdje agent objavljuje čiste komentare i skriva ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne može objavljivati komentare, glasovati, prikvačiti, zaključavati, dodjeljivati značke ili slati e-poštu - prompt je namjerno ograničen.

### Preporučeni dodaci prije puštanja uživo

- **Postavite [Smjernice zajednice](#community-guidelines).** Dovoljno je nekoliko rečenica pisane politike; agent ih primjenjuje pri svakom izvršavanju.
- **Zahtijevajte odobrenje za `ban_user` putem [odobrenja](#approval-workflow).** Ovo je prema zadanim postavkama uključeno u EU regiji (vidi [Usklađenost s člankom 17. EU DSA](#eu-dsa-compliance)) i preporučuje se svugdje.
- **Razmotrite i zahtijevanje odobrenja za `mark_comment_spam`** ako imate malo prometa, ali visokorizičan sadržaj.
- **Zahtijevajte odobrenje za `mark_comment_approved` ako koristite pre-moderaciju.** Odobravanje lošeg komentara izlaže ga čitateljima; zahtijevajte odobrenje dok agent ne stekne povjerenje kroz probni način.
- **Označite "Uključi faktor povjerenja komentatora, starost računa, povijest zabrana i nedavne komentare"** u [Opcijama konteksta](#context-options). Model će upozoravati puno manje agresivno kad može vidjeti da je netko dugogodišnji korisnik u dobroj vjeri.

### Preporučeni probni period

Pokrenite ovaj predložak u [probnom načinu](#dry-run-mode) najmanje tjedan dana na stvarnom prometu prije nego što ga uključite. Upotrijebite [Testna pokretanja (Replays)](#test-runs-replays) da biste također pregledali prethodnih 30 dana.

---