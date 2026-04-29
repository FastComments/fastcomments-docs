Sproži se, ko neto število glasov komentarja doseže konfigurirani prag. Neto glasovi so `votesUp - votesDown`.

### Zahtevana konfiguracija

- **Prag glasov** - celo število >= 1. Sprožilec se sproži ob glasovanju, ki pripelje neto glasove natanko do te vrednosti.

Če je prag 10 in komentar gre iz 9 na 10 neto glasov, se sprožilec sproži enkrat. Če glas nato poveča na 11, se sprožilec **ne** sproži znova - ne sproži se ob vsakem dodatnem glasu nad pragom.

### Kontekst, ki ga agent prejme

- Komentar z aktualnimi števili glasov.
- **smer glasovanja** (`up` ali `down`) glasovanja, ki je sprožilo prehod praga.
- Neobvezna zgodovina niti / uporabnika / kontekst strani, kot je konfigurirano.

### Pomembno

- Komentar, ki se povzpne na 10, pade nazaj na 9 in se znova povzpne na 10, bo sprožil sprožilec dvakrat. Ni stanja "enkrat sproženo" na posamezen komentar - če potrebujete takšno semantiko, naj agent ob prvem zagonu zabeleži [opombo v pomnilniku](#tools-overview) in jo preveri pri naslednjih zagonih.
- Prag je vedno **neto** število glasov, ne surovih pozitivnih glasov. Komentar z 12 za in 2 proti ima neto 10 in sproži sprožilec; komentar z 10 za in 0 proti prav tako sproži.
- Možni so tudi prehodi samo zaradi negativnega glasovanja - komentar, ki zaradi negativnega glasu pade s 11 na 10, prav tako sproži sprožilec. Parameter `voteDirection` v kontekstu pove agentu, iz katere smeri je prišel prehod praga.

### Pogoste uporabe

- **Pripenjanje** - predloga [Top Comment Pinner template](#template-top-comment-pinner) je zgrajena okoli tega sprožilca.
- **Promocija / poteki dela za izpostavljene komentarje** - pošljite dogodek preko [Webhooks](#webhooks-overview), da lahko zunanji sistem promovira komentar drugje na vaši strani.
- **Sledenje angažiranosti** - zabeležite opombo v pomnilniku o uporabniku, ki je napisal komentar, da bodo drugi agenti vedeli, da je ustvaril priljubljeno vsebino.

### Prilagajanje

Pravilen prag je odvisen od skupnosti. Spremljajte [Run History](#run-history) nekaj dni pri nizkem pragu (5), da vidite, kako pogosto se sproži. Povišajte prag, dokler stopnja sprožitev ne ustreza ritmu, ki ga želite.