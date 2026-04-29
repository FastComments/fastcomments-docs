**ID predloge:** `welcome_greeter`

Pozdravitelj dobrodošlice odgovarja toplo komentatorjem, ki komentirajo prvič. To je najmanj tvegana predloga (brez destruktivnih orodij) in dober prvi agent za zagon v živo.

### Vgrajen začetni poziv

[inline-code-attrs-start title = 'Začetni poziv predloge Pozdravitelj dobrodošlice'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Sprožilci

- **Nov uporabnik objavi svoj prvi komentar na tej strani** (`NEW_USER_FIRST_COMMENT`).

Ta dogodek se sproži natančno enkrat na uporabnika, zato agent ne more v zanko. Oglejte si [Sprožilec: Nov uporabnik prvi komentar](#trigger-new-user-first-comment).

### Dovoljena orodja

- [`write_comment`](#tools-overview)

To je edino orodje - agent dobesedno ne more moderirati, glasovati, blokirati ali pošiljati zasebnih sporočil (DM).

### Priporočene dopolnitve pred zagonom v živo

- **Nastavite prikazno ime** na nekaj vabljivega - "Community Bot", maskota vaše strani ali ime vaše znamke. Prikazno ime je tisto, kar bralci vidijo pritrjeno ob pozdravnem odgovoru.
- **Označite "Vključi naslov strani, podnaslov, opis in meta oznake"** v [Context Options](#context-options). Odgovori pozdravitelja postanejo opazno boljši, kadar lahko sklicuje, o čem stran dejansko govori.
- **Upoštevajte omejitve lokalizacije**, če delujete v več jezikih. Pozdrav v napačnem jeziku je bolj moteč kot izpuščen odgovor. Oglejte si [Obseg: Filtri URL in lokalizacije](#scope-url-locale).

### Zakaj odobritve niso potrebne

Agent piše samo nove komentarje in le ob enkratnem sprožilcu. V najslabšem primeru: neroden pozdrav. Ni nobenega destruktivnega dejanja, ki bi ga bilo treba omejevati z odobritvijo. Večina upravljavcev uporablja tega brez kakršnih koli odobritev, ko poskusni zagon izgleda brezhibno.

---