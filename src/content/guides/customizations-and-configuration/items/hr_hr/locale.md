[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazati widget za komentare u lokalizaciji određenu sustavom i preglednikom korisnika.

Kad korisnik komentira ili se prijavi, ažuriramo njegovu posljednju korištenu lokalizaciju i koristimo je i za slanje e‑mailova.

Ovo utječe na način na koji je widget za komentare preveden za korisnika. Lokalizacija se sastoji od jezika i regije korisnika, pa će podešavanje lokalizacije obično promijeniti jezik koji se koristi za prikaz teksta korisniku.

#### Putem UI-ja

Ovo se može definirati putem UI-ja za prilagodbu widgeta. Pogledajte opciju "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Padajući izbornik Locale / Language na stranici za prilagodbu widgeta koji se koristi za nadjačavanje otkrivene lokalizacije posjetitelja'; title='Promjena Locale / Language' app-screenshot-end]

#### Putem koda

Ovo se može nadjačati željenom lokalizacijom.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ručno definiranje korisnikove lokalizacije'; code-example-end]

### Podržani jezici i kodovi lokalizacije

[Možete pronaći potpuni popis podržanih jezika i odgovarajućih kodova lokalizacije ovdje.](/guide-supported-languages.html#supported-languages)

### Napomena o SSO

Ako koristite SSO, možda ćete htjeti proslijediti korisnikovu lokalizaciju u objektu korisnika, kako bi e‑mailovi i ostale stvari bile pravilno lokalizirane za njih.

---