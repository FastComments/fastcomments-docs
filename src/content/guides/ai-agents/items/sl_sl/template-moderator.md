**ID predloge:** `tos_enforcer`

Predloga Moderator je priporočeno izhodišče, če je vaš cilj zmanjšati ročno obremenitev moderiranja. Pregleduje nove in označene komentarje ter uveljavlja pravila vaše skupnosti.

Skoraj vedno boste želeli **dopolniti vgrajen poziv (prompt)** s konkretnimi primeri, kaj vaš splet naredi in ne dovoli. Platformina lastna politika eskalacije (opozori pred začasno prepovedjo, preveri spomin pred prepovedjo) je že vgrajena v sistemski poziv, ki ga agent prejme, zato je ni treba ponavljati.

### Sprožilci

- **New comment posted** (`COMMENT_ADD`) - agent pregleda vsak nov komentar.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - agent ponovno ovrednoti komentar, ki so ga označili drugi uporabniki.

### Dovoljena orodja

- [`mark_comment_approved`](#tools-overview) - uporabno za najemnike s predmoderacijo, kjer agent sprosti čiste komentarje in skrije preostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne more objavljati komentarjev, glasovati, pripenjati, zaklepati, podeljevati značk ali pošiljati e-pošte - poziv je namerno omejen.

### Priporočene dopolnitve pred prehodom v živo

- **Nastavite [Pravila skupnosti (Community Guidelines)](#community-guidelines).** Zadostuje nekaj povedi pisane politike; agent jo upošteva ob vsakem zagonu.
- **Omejite `ban_user` z [odobritvijo](#approval-workflow).** To je privzeto vklopljeno v regiji EU (glejte [EU DSA Article 17 Compliance](#eu-dsa-compliance)) in je priporočljivo povsod.
- **Razmislite tudi o omejitvi `mark_comment_spam` z odobritvijo**, če imate majhen obseg, a visok tveganje vsebine.
- **Omejite `mark_comment_approved` z odobritvijo, če uporabljate predmoderacijo.** Odobritev slabega komentarja ga prikaže bralcem; omejite to možnost, dokler agent ne pridobi zaupanja skozi suhi zagon.
- **Označite "Vključi faktor zaupanja avtorja komentarja, starost računa, zgodovino prepovedi in nedavne komentarje"** v [Možnosti konteksta (Context Options)](#context-options). Model bo opozarjal veliko manj agresivno, ko bo videl, da je nekdo dolgoletni uporabnik dobre vere.

### Priporočeno obdobje suhega zagona

Zaženite to predlogo v [dry-run](#dry-run-mode) vsaj en teden na vašem resničnem prometu, preden preklopite na Omogočeno. Uporabite [Test Runs (Replays)](#test-runs-replays) za predogled tudi na podatkih iz prejšnjih 30 dni.

---