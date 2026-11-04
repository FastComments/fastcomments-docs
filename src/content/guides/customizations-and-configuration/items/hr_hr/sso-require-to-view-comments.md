FastComments SSO (<a href="#sso">detalji ovdje</a>) pruža vašim korisnicima način za komentiranje bez potrebe da se prijave na drugu platformu.

Međutim, to samo po sebi ne osigurava vaše niti komentara, budući da su podaci o komentarima po zadanim postavkama javno dostupni – svatko tko može vidjeti stranicu može vidjeti i komentare.

Promjenom postavke možemo ograničiti dohvaćanje komentara osim ako to učini administrator ili valjani SSO korisnik.

#### Postavljanje bez koda

Možemo spriječiti pregledavanje i interakciju s našim nitima komentara, kada je SSO postavljen, stvaranjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagodbe</a>.

Pri tome, potražite SSO i naći ćete ovu opciju:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opcija Require SSO za prikaz komentara omogućena u pravilu prilagodbe, s izborom razine sigurnosti'; title='Require SSO za prikaz komentara' app-screenshot-end]

Omogućite ga i spremite pravilo prilagodbe.

#### Zaštiti samo određenu domenu ili stranicu

Da bismo zaštitili samo određenu domenu ili stranicu, jednostavno ćemo konfigurirati pravilo prilagodbe da to učini.

Na vrhu sučelja za prilagodbu naći ćemo dva unosa, Domain i URL ID.

Za zaštitu određene domene, unesite tu domenu u polje "domain".

Za zaštitu određene stranice, unesite URL stranice u polje "URL ID". Ako imate prilagođenu integraciju s FastComments, ovdje možete unijeti vrstu ID-a umjesto URL-a.

#### Razine sigurnosti

Kada zahtijevate SSO, trebate odlučiti želite li Simple SSO ili Secure SSO. Ako odaberete Simple SSO, oba su dopuštena, ali ako odaberete Secure SSO, sadržaj mora biti dohvaćen s Secure SSO payloadom hashiranim vašim API ključem kako bi se mogao pregledati.

Opcija razine sigurnosti pojavit će se kada odaberete "Require SSO To View Comments", i primjenjuje se samo dok je ta opcija uključena.

Postavke <a href="#sso-simple">Automatske provjere komentara putem Simple SSO i dopuštanja Simple SSO administratora</a> su odvojene i primjenjuju se neovisno o tome je li ova opcija uključena.

#### Zaštita izvan čitanja

Omogućavanje ove opcije zaštitit će stranicu ili domenu od komentiranja, osim ako korisnik nije prijavljen putem SSO.

#### Zamke

Korisnici koji su kreirali komentare prije vaše SSO integracije neće moći vidjeti te komentare, osim ako se ne prijave putem vaše SSO integracije.