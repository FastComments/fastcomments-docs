FastComments SSO (<a href="#sso">detalji ovdje</a>) omogućuje vašim korisnicima da komentiraju bez potrebe za prijavom na drugu platformu.

Međutim, samo to ne štiti vaše niti komentara, jer su prema zadanim postavkama podaci komentara javno dostupne informacije - svatko tko može pregledati stranicu može vidjeti komentare.

Promjenom jedne postavke možemo ograničiti dohvaćanje komentara osim ako to ne radi administrator ili valjani SSO korisnik.

#### Postavljanje bez koda

Možemo spriječiti pregledavanje i interakciju s našim nitima komentara, kad je SSO postavljen, stvaranjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagodbe</a>.

Pri tome potražite SSO i naći ćete ovu opciju:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Omogućite je i spremite pravilo prilagodbe.

#### Zaštita samo određenog domena ili stranice

Da biste zaštitili samo određenu domenu ili stranicu, jednostavno konfiguriramo pravilo prilagodbe da to učini.

Na vrhu sučelja za prilagodbu pronaći ćemo dva unosa, Domena i URL ID.

Za zaštitu određene domene unesite odgovarajuću domenu u polje "domain".

Za zaštitu određene stranice unesite URL stranice u polje "URL ID". Ako imate prilagođenu integraciju s FastComments, ovdje umjesto URL-a možete unijeti vrstu ID-a.

#### Razine sigurnosti

Pri zahtjevu za SSO trebate odlučiti zahtijevate li Simple SSO ili Secure SSO. Ako zahtijevate Simple SSO, oba su dopuštena, ali ako zahtijevate Secure SSO tada
se sadržaj mora dohvatiti sa Secure SSO payload-om heširanim vašim API ključem kako bi bio vidljiv.

Opcija razine sigurnosti pojavit će se kada odaberete "Require SSO To View Comments".

#### Zaštita iznad samog čitanja

Omogućavanje ove opcije zaštitit će stranicu ili domenu od komentiranja osim ako korisnik nije prijavljen putem SSO-a.

#### Napomene

Svi korisnici koji su kreirali komentare prije vaše SSO integracije neće ih moći vidjeti, osim ako se ne prijave putem vaše SSO integracije.