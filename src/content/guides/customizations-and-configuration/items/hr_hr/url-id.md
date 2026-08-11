[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Pri prikazivanju niza komentara ili ostavljanju komentara, FastComments mora znati kojoj stranici, članku ili proizvodu ti komentari pripadaju.

Za to koristimo nešto što nazivamo "URL ID". To je ili identifikator, poput stringa ili broja, ili URL.

Prema zadanim postavkama, ako ne navedete urlId, on će postati URL stranice. Uzeti ćemo trenutni URL stranice i očistiti ga kako bismo uklonili sve uobičajene marketinške parametre ili identifikatore praćenja.

U slučaju integracija trećih strana, poput WordPressa, naš dodatak će obično koristiti identifikator koji predstavlja trenutno prikazane informacije kao URL ID, na primjer ID članka/stranice.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definiranje prilagođenog URL ID-a'; code-example-end]

Jedna stvar na koju ćemo često upućivati u ovom dokumentu je <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Korisničko sučelje za prilagodbu widgeta</a>.

Ovo sučelje može se koristiti za mnoge promjene widgeta za komentare bez korištenja koda.

Kada stvaramo pravilo prilagodbe, često ćemo željeti da se primijeni na sve stranice naše web stranice. Međutim, u nekim slučajevima želimo prilagoditi widget za komentare na određenoj stranici, bilo da primijenimo prilagođeni stil ili da učinimo komentare za tu stranicu anonimnim. Također, na primjer, možete imati da se živi komentari odmah prikazuju na nekim stranicama, dok se na drugima skrivaju ispod tipki za obavijesti.

Sve je to moguće putem polja za unos URL ID-a na ovoj stranici, koje izgleda ovako:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Polje URL ID-a koje se koristi za ograničavanje pravila prilagodbe na jednu stranicu ili na uzorak poput */blog/*'; title='Unos URL ID-a na stranici za prilagodbu widgeta' app-screenshot-end]

Vrijednost u ovom polju treba odgovarati parametru *urlId* koji se prosljeđuje widgetu za komentare. Ako želite da vaše pravilo prilagodbe bude neovisno o *urlId*, ostavite ovo polje prazno ili unesite *.

Od 2023. godine polje `URL ID` u prilagodbi widgeta sada također prihvaća uzorke! Na primjer, možete imati `*/blog/*` za dodavanje stilova specifičnih za vaš blog i `*/store/*` za stilove specifične za vašu trgovinu, sve dok koristite istu domenu.

### Zamke

1. Ako vaša stranica ima hash parametre (kao example.com#page-1) - to će po zadanim postavkama postati dio URL ID-a.
2. Tijekom migracija, na primjer s WordPressa na Gatsby, možda ćete morati migrirati vrijednosti URL ID-a komentara nakon početne migracije. Za to nas kontaktirajte.

---