Vsaka bremenitev, bodisi obnova paketa, Flex mesec ali sprememba paketa, ustvari račun. Računi ob plačilu služijo tudi kot potrdila.

### Iskanje računov

Odprite [Billing History](https://fastcomments.com/auth/my-account/billing-history) ali kliknite `View Invoices` na strani Billing Info. Tabela prikazuje vsak račun z datumom, obdobjem obračunavanja, številko računa, opisom, skupno vsoto, statusom (`Paid`, `Unpaid`, `Draft` ali `Void`) in datumom zapadlosti, deset na stran, najnovejši na vrhu.

Za vsak račun:

- `View` odpre račun na Stripe gostovani strani, kjer ga lahko tudi prenesete.
- `PDF` neposredno prenese PDF.
- `Pay Now` se prikaže namesto `View` pri neplačanem računu in vas pripelje na Stripe plačilno stran za ta račun, kjer ga lahko neposredno plačate.

Računi so ob ustvarjanju poslani tudi po e-pošti na vaš obračunski e-naslov. Če želite, da gredo v finančni nabiralnik namesto k lastniku računa, nastavite `Billing Email` v [Invoice Settings](#invoice-settings).

### Kaj je na računu

Račun prikazuje ime in naslov za obračunavanje ter ID za DDV / davek iz [Invoice Settings](#invoice-settings), postavke, morebitni predplačni [kredit](#credits), ki je bil uporabljen, in znesek bremenitve na vašo kartico. Flex računi navajajo eno postavko za vsako vrsto uporabe, glejte [How Flex Usage Is Measured](#how-flex-usage-is-measured). Za obračunske naslove v Evropski uniji so Flex računi izstavljeni z 0 % DDV in s podpisom `Reverse Charge Transaction: Customer to pay VAT`.

Spremembe nastavitev računov veljajo za račune, ustvarjene po shranjevanju. Že izstavljenega računa ni mogoče urejati, zato nastavite podatke podjetja in ID za DDV / davek pred naslednjim datumom obračunavanja.

### Računi obračunani drugje

Če vaše obračunavanje ureja zunanji partner, na primer račun, dogovorjen s podporo, so vam računi poslani za plačilo namesto bremenitve kartice in se morda ne prikazujejo v Billing History. Računi, nameščeni prek Shopify ali BigCommerce, se obračunajo prek te platforme, stran Billing Info pa povezuje na stran paketa tam. Podračuni, obračunani prek starševskega računa, nimajo lastnih računov; glejte [When You Are Charged](#when-you-are-charged).