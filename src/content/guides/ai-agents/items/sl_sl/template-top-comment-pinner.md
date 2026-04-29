**ID predloge:** `top_comment_pinner`

Pripenjač najvišjih komentarjev spremlja vrhnje komentarje, ki presežejo prag glasov, in jih pripne — zamenja karkoli je bilo prej pripeto v isti niti.

Vgrajen poziv agentu mu ukaže, naj preskoči odgovore (pripenjanje deluje na nitih, zato je pripenjanje odgovora redko koristno) in naj filtrira promocijske vsebine (tako agent ne bo okrepil priljubljenega link-spama).

### Sprožilci

- **Komentar preseže prag glasov** (`COMMENT_VOTE_THRESHOLD`, privzeti prag glasov: 10).

Sprožilec se aktivira, ko neto glasovi komentarja (`up - down`) dosežejo konfiguriran prag. Prilagodite številko na obrazcu za urejanje glede na to, kako aktivne so vaše niti — 10 je razumen privzeti za zmerno aktivne strani.

### Dovoljena orodja

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pripenjanje ni uničujoče — lahko ga takoj razveljavite — zato ta predloga običajno deluje brez odobritev.

### Priporočene dodatke pred objavo

- **Označite "Vključi nadrejen komentar in prejšnje odgovore v isti niti"** v [Context Options](#context-options). Brez konteksta niti agent ne more zanesljivo ugotoviti, ali je že pripet komentar, ki ga je treba odpeti.
- **Prilagodite prag glasov** glede na vašo stran. Na prometnih nitih se 10 zgodi preveč pogosto; na mirnih nitih se 10 morda nikoli ne zgodi.
- **Razmislite o omejitvi po URL**, če želite pripete komentarje le v določenih razdelkih vaše strani — na primer v novičarskih nitih, ne pa v nitih z obvestili.

### Opomba o podvojenem pripenjanju

Poziv agenta mu ukaže, naj najprej odpeče, preden pripne, vendar če model preskoči ta korak, platforma sama ne uveljavlja pravila en pripet na nit (lahko imate več). Če je podvojeno pripenjanje problem na vaši strani, omejite `pin_comment` z odobritvijo in jih vsakega pregledajte — ali pa napišite strožji poziv.