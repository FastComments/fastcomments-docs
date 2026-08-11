---
By default, each user can submit up to `5 comments` in the same minute.

This is tracked by user id, anon user id, and ip address (hashed).

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Polje največ komentarjev na minuto na strani za prilagajanje gradnika, privzeto nastavljeno na 5'; title='Omejevanje števila komentarjev na uporabnika' app-screenshot-end]

Upoštevajte, da če uporabljate API za ustvarjanje komentarjev, boste morda želeli v zahtevo našemu strežniku poslati izvirni `ip` naslov uporabnika, da se omejitev hitrosti uporabi
po uporabniku in ne globalno za vaš račun.
---