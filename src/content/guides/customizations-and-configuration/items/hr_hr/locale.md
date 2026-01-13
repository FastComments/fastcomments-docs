[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazati widget za komentare u lokalizaciji koju određuju postavke korisnikovog sustava i preglednika.

Kad korisnik ostavi komentar ili se prijavi, ažuriramo zadnju korištenu lokalizaciju i koristimo je i za slanje e-poruka.

To utječe na način na koji je widget za komentare preveden za korisnika. Lokalizacija se sastoji od jezika i regije korisnika, pa će konfiguriranje lokalizacije obično
promijeniti jezik koji se koristi za prikaz teksta korisniku.

#### Putem korisničkog sučelja

To se može postaviti putem sučelja za prilagodbu widgeta. Pogledajte opciju "Lokalizacija / Jezik":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Putem koda

Ovo se može nadjačati željenom lokalizacijom.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Podržani jezici i kodovi lokalizacija

[Puni popis podržanih jezika i odgovarajućih kodova lokalizacija možete pronaći ovdje.](/guide-supported-languages.html#supported-languages)

### Napomena o SSO

Ako koristite SSO, možda ćete htjeti proslijediti lokalizaciju korisnika u objektu user, kako bi e-poruke i ostalo bili ispravno lokalizirani za njih.