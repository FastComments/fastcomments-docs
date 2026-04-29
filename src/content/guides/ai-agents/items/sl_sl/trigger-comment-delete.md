Sproži se, ko je komentar izbrisan.

### Kontekst, ki ga prejme agent

- Komentar, ki je bil pravkar izbrisan - besedilo, avtor, stran.
- Izbirno nit / zgodovina uporabnika / kontekst strani, kot je konfigurirano.

### Pomembno

- Sproži se tako pri **mehkih izbrisih** (kjer je komentar skrit, vendar ohranjen za revizijo) kot pri **trdih izbrisih** (kjer je komentar popolnoma odstranjen). Obdelovalec sprožilca razreši komentar iz cevovoda kaskadnega brisanja; kar agent vidi, je zadnje znano stanje.
- Ko je komentar popolnoma izbrisan, orodja, ki ciljajo nanj (`pin_comment`, `mark_comment_spam`, itd.) za ta ID komentarja ne bodo delovala.

### Pogoste uporabe

- **Posredovanje revizije preko [Webhooks](#webhooks-overview)** - sproži dogodek `trigger.succeeded`, da zunanji sistem zabeleži, kaj je bilo izbrisano.
- **Zapis v spomin** - agentu omogoči, da zabeleži [opombo v spominu](#tools-overview) o vzorcu izbrisa (izbrisani komentar je bil uporabnikov tretji v 24 urah itd.).
- **Učinki med nitmi** - zaznajte, kdaj izbris spremeni strukturo niti, ki jo je agent prej povzel, in premislite, ali jo je treba ponovno povzeti.

### Opomba o stroških delovanja

Če imate spletno mesto z velikim številom izbrisov (intenzivno človeško moderiranje), se lahko ta sprožilec pogosto aktivira.