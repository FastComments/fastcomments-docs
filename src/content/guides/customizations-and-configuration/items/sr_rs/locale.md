[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Po podrazumevanju, FastComments će prikazati vidžet za komentare u lokalizaciji određenoј sistemom i pregledačem korisnika.

Kada korisnik komentariše ili se prijavi, ažuriramo njegovu poslednju korišćenu lokalizaciju i koristimo je i za slanje e‑mailova.

Ovo utiče na to kako je vidžet za komentarisanje preveden za korisnika. Lokalizacija se sastoji od jezika i regiona korisnika, tako da podešavanje lokalizacije obično menja jezik koji se prikazuje korisniku.

#### Preko UI-ja

Ovo se može definisati putem UI‑ja za prilagođavanje vidžeta. Pogledajte opciju „Locale / Language“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Padajući meni Locale / Language na stranici za prilagođavanje vidžeta koji se koristi za prepisivanje detektovane lokalizacije posetioca'; title='Promena lokalizacije / jezika' app-screenshot-end]

#### Preko koda

Ovo se može prepisati željenom lokalizacijom.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ručno definisanje lokalizacije korisnika'; code-example-end]

### Podržani jezici i kodovi lokalizacije

[Možete pronaći kompletnu listu podržanih jezika i odgovarajućih kodova lokalizacije ovde.](/guide-supported-languages.html#supported-languages)

### Napomena o SSO

Ako koristite SSO, možda ćete želeti da prosledite lokalizaciju korisnika u objektu korisnika, kako bi e‑mailovi i druge stvari bile pravilno lokalizovane za njih.