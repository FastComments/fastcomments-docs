**ID predloge:** `tos_enforcer`

Predloga Moderator je priporočeno izhodišče, če je vaš cilj zmanjšati obremenitev ročnega moderiranja. Pregleduje nove in označene komentarje ter uporablja pravila vaše skupnosti.

### Vgrajen začetni poziv

[inline-code-attrs-start title = 'Začetni poziv predloge Moderator'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Skoraj vedno boste želeli ta poziv **razširiti** s konkretnimi primeri, kaj vaša stran dovoljuje in česa ne. Platformina politika za eskalacijo (opozorilo pred prepovedjo, iskanje v spominu pred prepovedjo) je že vključena v sistemski poziv, ki ga agent prejme, zato je ni treba ponavljati.

### Sprožilci

- **Objavljen nov komentar** (`COMMENT_ADD`) - agent pregleda vsak nov komentar.
- **Komentar preseže prag označitev** (`COMMENT_FLAG_THRESHOLD`, privzeti prag: 3) - agent znova oceni komentar, ki so ga označili drugi uporabniki.

### Dovoljena orodja

- [`mark_comment_approved`](#tools-overview) - koristno za najemnike s pred-moderiranjem, kjer agent objavi čiste komentarje in skrije ostale.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ne more objavljati komentarjev, glasovati, pripenjati, zakleniti, podeljevati značk ali pošiljati e-pošte - poziv je namerno ozko omejen.

### Priporočene dopolnitve pred objavo

- **Določite [Smernice skupnosti](#community-guidelines).** Zadoščajo nekaj stavkov pisne politike; agent jih uporablja pri vsakem zagonu.
- **Omejite `ban_user` z [odobritvijo](#approval-workflow).** To je privzeto vključeno v regiji EU (glejte [Skladnost z EU DSA člen 17](#eu-dsa-compliance)) in je priporočeno povsod.
- **Razmislite tudi o omejitvi `mark_comment_spam` z odobritvijo**, če imate vsebino z nizkim volumnom, vendar visokim tveganjem.
- **Omejite `mark_comment_approved` z odobritvijo, če uporabljate pred-moderiranje.** Potrditev slabega komentarja ga postavi pred bralce; omejite to možnost, dokler agent s preizkusnim zagonom ne pridobi zaupanja.
- **Označite "Vključi faktor zaupanja komentatorja, starost računa, zgodovino prepovedi in nedavne komentarje"** v [Možnosti konteksta](#context-options). Model bo veliko manj agresivno opozarjal, ko bo videl, da je nekdo dolgoletni uporabnik v dobri veri.

### Priporočeno obdobje preizkusnega zagona

Zaženite to predlogo v [dry-run](#dry-run-mode) vsaj en teden na dejanskem prometu, preden jo preklopite na Omogočeno. Uporabite [Test Runs (Replays)](#test-runs-replays) tudi za predogled za zadnjih 30 dni.

---