**ID predloge:** `top_comment_pinner`

Pripenjač najboljših komentarjev spremlja komentarje najvišje ravni, ki presegajo prag glasov, in jih pripne - pri čemer nadomesti karkoli je bilo prej pripeto v isti nitki.

### Vgrajeni začetni poziv

[inline-code-attrs-start title = 'Začetni poziv predloge Pripenjanje najboljšega komentarja'; type='text' inline-code-attrs-end]
[inline-code-start]
Pripneš najboljše komentarje najvišje ravni v nitki. Ko komentar doseže prag glasov, ga pripni, če je vsebina ustrezna in ni promocijske narave. Najprej odstrani vsak prej pripet komentar v isti nitki. Ne pripenjaj odgovorov, le komentarjev najvišje ravni.
[inline-code-end]

Navedba "ne pripenjaj odgovorov" je pomembna: pripenjanje deluje na ravni nitk, zato je pripenjanje odgovora redko uporabno. Filter "nepromocijsko" preprečuje agentu, da bi okrepil priljubljen komentar, ki je spam s povezavami.

### Sprožilci

- **Komentar preseže prag glasov** (`COMMENT_VOTE_THRESHOLD`, privzeti prag glasov: 10).

Sprožilec se sproži, ko neto glasovi komentarja (`up - down`) dosežejo konfigurirani prag. Prilagodite številko v obrazcu za urejanje glede na to, kako aktivne so vaše nitke - 10 je smiselna privzeta vrednost za zmerno aktivne strani.

### Dovoljena orodja

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pripenjanje ni uničujoče - ga je mogoče takoj razveljaviti - zato ta predloga običajno deluje brez odobritev.

### Priporočene dopolnitve pred objavo v živo

- **Označite "Vključi nadrejen komentar in predhodne odgovore v isti nitki"** v [Context Options](#context-options). Brez konteksta nitke agent ne more zanesljivo ugotoviti, ali je že pripet komentar, ki ga je treba odstraniti.
- **Prilagodite prag glasov** za vašo stran. Na prometnih nitkah se 10 zgodi preveč pogosto; na mirnih nitkah se 10 morda nikoli ne zgodi.
- **Razmislite o omejitvi po URL**, če želite pripete komentarje le v določenih delih vaše strani - na primer v novičnih nitkah, ne pa v nitkah z obvestili.

### Opomba o podvojenem pripenjanju

Poziv agenta mu ukaže, naj najprej odstrani pripet komentar, preden pripne novega, vendar če model ta korak spregleda, platforma sama ne uveljavlja pravila enega pripetega komentarja na nitko (lahko jih je več). Če je podvojeno pripenjanje problem na vaši strani, omejite uporabo `pin_comment` z odobritvijo in preglejte vsak primer - ali pa napišite bolj natančen poziv.