[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Kada se prikazuje nit komentara, ili se ostavlja komentar, FastComments mora znati kojoj stranici, članku ili proizvodu ti komentari pripadaju.

Da bismo to uradili, koristimo nešto što nazivamo "URL ID". To je ili identifikator, poput stringa ili broja, ili URL.

Po defaultu, ako ne navedete urlId, on će postati URL stranice. Uzeće trenutni URL stranice i očistiti ga kako bi uklonio bilo koje uobičajene marketinške parametre ili identifikatore za praćenje.

U slučaju integracija trećih strana, kao što je WordPress, naš plugin će obično koristiti identifikator koji predstavlja informacije koje se trenutno pregledaju kao URL ID, na primer id članka/stranice.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Jedna stvar na koju ćemo često pozivati u ovom dokumentu je <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>.

Ovaj UI se može koristiti za izvođenje mnogih promena na widgetu za komentare bez korišćenja koda.

Kada kreirate pravilo za prilagođavanje, često ćemo želeti da se ono primeni na sve stranice na našem sajtu. Međutim, u nekim slučajevima želimo da prilagodimo widget za komentare na određenoj stranici, bilo da primenimo prilagođeni stil, ili možda napravimo komentare za tu stranicu anonimnim. Takođe, na primer, možete imati uživo komentare koji se odmah pojavljuju na nekim stranicama, dok su na drugim skriveni iza dugmadi za obaveštavanje.

Sve je to moguće preko URL ID polja za unos na ovoj stranici, koje izgleda kao što sledi:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Vrednost u ovom polju treba da odgovara *urlId* parametru koji se prosleđuje u comment widget. Ako želite da vaše pravilo za prilagođavanje bude agnostično prema *urlId*, ostavite ovo polje prazno ili unesite *.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### Napomene

1. Ako vaša stranica ima hash parametre (kao example.com#page-1) - to će postati deo URL ID-a, po defaultu.
2. Tokom migracija, na primer iz WordPress-a u Gatsby, možda ćete morati da migrirate vrednosti komentara URL ID nakon početne migracije. U tom slučaju, obratite nam se.