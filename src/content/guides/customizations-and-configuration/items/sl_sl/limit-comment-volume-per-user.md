Privzeto lahko vsak uporabnik odda največ `5 komentarjev` v isti minuti.

To se sledi po uporabniškem id, anonimnem uporabniškem id in ip naslovu (zgostljenem).

To je mogoče prilagoditi brez kode, na strani za prilagoditev pripomočka:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Polje za največje število komentarjev na minuto na strani za prilagoditev pripomočka, privzeto nastavljeno na 5'; title='Omejevanje količine komentarjev na uporabnika' app-screenshot-end]

Upoštevajte, da če uporabljate API za ustvarjanje komentarjev, boste morda želeli v zahtevi posredovati izvirni `ip` naslov uporabnika našemu zaledju, da se omejitev hitrosti uporabi na uporabnika in ne globalno na vaš račun.