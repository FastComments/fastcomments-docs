[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Podrazumevano, FastComments će prikazati vidžet za komentare u lokalizaciji koju određuje sistem i pregledač korisnika.

Kada korisnik komentariše ili se prijavi, ažuriramo njegovu poslednju korišćenu lokalizaciju i koristimo je i za slanje e‑mailova.

Ovo utiče na to kako je vidžet za komentarisanje preveden za korisnika. Lokalizacija se sastoji od jezika i regiona korisnika, tako da podešavanje lokalizacije obično menja jezik koji se koristi za prikaz teksta korisniku.

#### Putem UI-ja

Ovo se može definisati pomoću UI-ja za prilagođavanje vidžeta. Pogledajte opciju „Locale / Language“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Padajući meni Locale / Language na stranici za prilagođavanje vidžeta koji se koristi za prepisivanje otkrivene lokalizacije posetioca'; title='Promena lokalizacije / jezika' app-screenshot-end]

#### Putem koda

Ovo se može prepisati željenom lokalizacijom.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ručno definisanje korisnikove lokalizacije'; code-example-end]

### Podržani jezici i kodovi lokalizacije

[Možete pronaći kompletnu listu podržanih jezika i odgovarajućih kodova lokalizacije ovde.](/guide-supported-languages.html#supported-languages)

### Napomena o SSO

Ako koristite SSO, možda ćete želeti da prosledite korisnikovu lokalizaciju u objektu korisnika, kako bi e‑mailovi i druge stvari bile pravilno lokalizovane za njih.

---