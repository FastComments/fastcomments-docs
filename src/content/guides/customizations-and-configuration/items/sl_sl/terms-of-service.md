FastComments vam omogoča, da od prvih komentatorjev zahtevate sprejem vaših pogojev uporabe, preden oddajo komentar.

Ko je omogočeno:
- **Anonimni uporabniki** bodo videli potrditveno polje TOS pri vsakem komentarju
- **Avtenticirani uporabniki** bodo videli potrditveno polje le pri njihovem prvem komentarju ali ko posodobite svoje TOS

### Konfiguracija

Pojdite na stran za prilagajanje gradnika in omogočite potrditveno polje "Zahtevaj sprejem pogojev uporabe". Ko je omogočeno, boste videli naslednje možnosti:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Pult s pogoji uporabe, ki prikazuje izbirnik načina besedila TOS in polje datuma zadnje posodobitve'; title='Možnosti pogojev uporabe' app-screenshot-end]

- **Način besedila TOS**: Privzeto potrditveno polje prikazuje "Strinjam se s pogoji uporabe in pravilnikom o zasebnosti" z povezavami do obeh dokumentov. Izberite "Prilagodi besedilo po jezikih", da zagotovite svoje besedilo za vsak jezik.
- **Datum zadnje posodobitve TOS**: Ko posodobite svoje pogoje uporabe, nastavite ta datum. Uporabniki, ki so sprejeli pred tem datumom, bodo morali sprejeti ponovno.

### Kako deluje

- Časovni žig sprejema TOS se shrani po uporabniku in po komentarju
- Ko uporabnik sprejme TOS, se datum zabeleži v njegovem uporabniškem profilu (po najemniku)
- Če nastavite datum "Zadnje posodobitve", ki je po datumu sprejema uporabnika, bo moral ponovno sprejeti
- Za anonimne uporabnike, ki jih ni mogoče slediti, se potrditveno polje pojavi pri vsaki oddaji komentarja