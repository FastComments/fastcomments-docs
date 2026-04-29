Sproži se, ko števec označitev komentarja doseže **natanko** nastavljen prag.

### Zahtevana konfiguracija

- **Prag označitev** - celo število >= 1. Sprožilec se aktivira v trenutku, ko `flagCount === flagThreshold`. Ne sproži se znova na nadaljnjih označitvah čez prag.

Če je prag 3 in trije uporabniki označijo komentar, se agent sproži enkrat ob tretji označitvi. Četrta, peta ali šesta označitev ga **ne** sproži ponovno.

### Kontekst, ki ga prejme agent

- Označeni komentar.
- Neobvezno kontekst pogovora / zgodovina uporabnika / kontekst strani, kot je konfigurirano.
- Število označitev je v bloku komentarja kot `Flag Count: N`.

### Pomembno

- Sprožilec se aktivira le, ko komentar prečka prag od spodaj preko poti obravnave označitev na platformi (kjer je `didIncrement === true`). Neposredni zapisi v DB, ki nastavijo `flagCount` na vrednost praga, ga ne sprožijo; označitve nad pragom ga prav tako ne sprožijo ponovno.
- Ne vključuje, kdo je označil komentar - označitve so agentu anonimne. Če želite preveriti uporabnike, ki so označili, jih pridobite iz lastnih podatkov.
- Zamuda sprožilca (glejte [Odloženi sprožilci](#trigger-deferred-delay)) je pri tem sprožilcu *močno* priporočena - označitve pogosto prihajajo v valovih med razgreto debato, in majhna zamuda pusti sliko bolj stabilno, preden agent ukrepa.

### Pogoste uporabe

- **Pregled moderacije** - označeni komentar je kanoničen signal "ljudje menijo, da bi to lahko bilo sporno". [Predloga moderatorja](#template-moderator) je privzeto naročena na ta sprožilec s pragom označitev 3.
- **Razširitev vrste predmoderacije** - agent izvede začetni pregled in ali označi komentar za moderacijo (z `mark_comment_reviewed`) ali pa ga še dodatno eskalira.
- **Preprečevanje brigadiranja** - kombinirajte ta sprožilec s [kontekstom zgodovine uporabnika](#context-options) in dovolite agentu, da vidi pretekle prepovedi/znake podvojenih vsebin, preden ukrepa.

### Priporočila za kombiniranje

Naročite se na **oboje** `COMMENT_ADD` in `COMMENT_FLAG_THRESHOLD`, če želite moderacijski agent, ki ujame očitne primere na prvi pogled in ponovno oceni vprašljive primere, ko se označitve naberejo. Oba dogodka se sprožita neodvisno - agent se bo zagnal dvakrat, če sta bila oba naročena in sta oba sprožena, vendar drugi zagon vidi sedaj označeno stanje.