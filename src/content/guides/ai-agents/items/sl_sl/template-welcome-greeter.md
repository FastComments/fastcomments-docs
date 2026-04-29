**ID predloge:** `welcome_greeter`

Pozdravitelj dobrodošlice toplo odgovori komentatorjem, ki komentirajo prvič. Je predloga z najnižjim tveganjem (brez destruktivnih orodij) in dober prvi agent za zagon v živo.

### Sprožilci

- **Nov uporabnik objavi svoj prvi komentar na tem mestu** (`NEW_USER_FIRST_COMMENT`).

Dogodek se sproži natančno enkrat na uporabnika, zato agent ne more zagnati zanke. Glejte [Sprožilec: Nov uporabnik - prvi komentar](#trigger-new-user-first-comment).

### Dovoljena orodja

- [`write_comment`](#tools-overview)

To je edino orodje - agent dobesedno ne more moderirati, glasovati, blokirati (ban) ali poslati zasebnega sporočila (DM).

### Priporočene nastavitve pred objavo v živo

- **Nastavite prikazno ime** na nekaj vabilnega - "Community Bot", maskota vaše strani ali ime vaše znamke. Prikazno ime je tisto, kar bralci vidijo pripeto ob pozdravnem odgovoru.
- **Označite "Vključi naslov strani, podnaslov, opis in meta oznake"** v [Opcije konteksta](#context-options). Odgovori pozdravitelja postanejo opazno boljši, ko se lahko sklicujejo na to, o čem stran dejansko govori.
- **Upoštevajte omejitve glede lokalne nastavitve** če delate v več jezikih. Pozdravni odgovor v napačnem jeziku je bolj moteč kot izpuščen odgovor. Glejte [Obseg: Filtri URL in lokalnih nastavitev](#scope-url-locale).

### Zakaj odobritve niso potrebne

Agent samo piše nove komentarje in le ob enkratnem sprožilcu. Najslabše: neroden pozdrav. Ni nič destruktivnega, kar bi bilo treba omejevati. Večina upravljavcev to zažene brez odobritev, ko poskusni zagon izgleda v redu.