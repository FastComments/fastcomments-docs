[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Pri prikazivanju niza komentara ili ostavljanju komentara, FastComments mora da zna kojoj stranici, članku ili proizvodu ti komentari pripadaju.

Za to koristimo nešto što nazivamo „URL ID“. To je ili identifikator, poput niza znakova ili broja, ili URL.

Podrazumevano, ako ne navedete urlId, on će postati URL stranice. Uzimaćemo trenutni URL stranice i očistiti ga da ukloni sve uobičajene marketinške parametre ili identifikatore praćenja.

U slučaju integracija trećih strana, poput WordPress-a, naš dodatak će obično koristiti identifikator koji predstavlja trenutne informacije koje se pregledaju kao URL ID, na primer ID članka/strane.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definisanje prilagođenog URL ID-a'; code-example-end]

Jedna stvar na koju ćemo često ukazivati u ovom dokumentu je <a href="https://fastcomments.com/auth/my-account/customize-widget/new">UI za prilagođavanje widgeta</a>.

Ovaj UI se može koristiti za mnoge izmene widgeta za komentare bez korišćenja koda.

Kada kreiramo pravilo prilagođavanja, često ćemo želeti da se primeni na sve stranice našeg sajta. Međutim, u nekim slučajevima želimo da prilagodimo widget za komentare na određenoj stranici, bilo da primenimo prilagođeni stil ili možda učinimo komentare na toj stranici anonimnim. Takođe, na primer, možete imati da se živi komentari odmah prikazuju na nekim stranicama, dok su na drugim skriveni iza dugmeta za obaveštenja.

Sve ovo je moguće putem polja za unos URL ID-a na ovoj stranici, koje izgleda ovako:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Polje URL ID koje se koristi za ograničavanje pravila prilagođavanja na jednu stranicu, ili na šablon kao što je */blog/*'; title='Unos URL ID-a na stranici za prilagođavanje widgeta' app-screenshot-end]

Vrednost u ovom polju treba da se podudara sa parametrom *urlId* prosleđenim widgetu za komentare. Ako želite da vaše pravilo prilagođavanja bude neutralno prema *urlId*, ostavite ovo polje prazno ili unesite *.

Od 2023. godine, polje `URL ID` u prilagođavanju widgeta sada takođe prihvata obrasce! Na primer, možete imati `*/blog/*` da dodate stil specifičan za vaš blog i `*/store/*` da imate stil specifičan za vašu prodavnicu, sve dok koristite isti domen.

### Zamke

1. Ako vaša stranica ima hash parametre (kao example.com#page-1) – to će podrazumevano postati deo URL ID-a.  
2. Tokom migracija, na primer sa WordPress-a na Gatsby, možda ćete morati da migrirate vrednosti URL ID komentara nakon početne migracije. Za to nas kontaktirajte.