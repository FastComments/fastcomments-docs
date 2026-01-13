[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Po podrazumevanoj vrednosti, FastComments će prikazati widget za komentare u lokalu koji je određen podešavanjima sistema i pregledača korisnika.

Kada korisnik ostavi komentar ili se prijavi, ažuriramo njihov poslednji korišćeni lokal i koristimo ga i za slanje e‑poruka.

Ovo utiče na način na koji je widget za komentare preveden za korisnika. Lokal se sastoji od jezika korisnika i regiona, tako da konfiguracija lokala će
obično promeniti jezik koji se koristi za prikaz teksta korisniku.

#### Preko korisničkog interfejsa

Ovo se može definisati koristeći interfejs za prilagođavanje widgeta. Pogledajte opciju "Lokal / Jezik":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Putem koda

Ovo se može prebrisati željenim lokalom.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Podržani jezici i kodovi lokala

[Kompletan spisak podržanih jezika i odgovarajućih kodova lokala možete pronaći ovde.](/guide-supported-languages.html#supported-languages)

### Napomena za SSO

Ako koristite SSO, možda ćete želeti da prosledite lokal korisnika u user object, tako da su e‑poruke i druge stvari pravilno lokalizovane za njih.

---