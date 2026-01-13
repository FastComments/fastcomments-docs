FastComments SSO (<a href="#sso">detalji ovde</a>) pruža vašim korisnicima način da komentarišu bez potrebe da se prijavljuju na drugu platformu.

Međutim, samo ovo ne osigurava vaše niti komentara, pošto su po defaultu podaci komentara javno dostupne informacije - bilo ko ko može da pregleda stranicu može videti i komentare.

#### No-Code Setup

Možemo onemogućiti pregledanje i interakciju sa našim nitima komentara, kada je SSO podešen, kreiranjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagođavanja</a>.

Kada to uradite, pretražite SSO i naći ćete ovu opciju:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Omogućite je i sačuvajte pravilo prilagođavanja.

#### Only Protect a Certain Domain or Page

Da biste zaštitili samo određeni domen ili stranicu, jednostavno ćemo podesiti pravilo prilagođavanja da to uradi.

Na vrhu korisničkog interfejsa za prilagođavanje naći ćemo dva unosa, Domain i URL ID.

Da biste zaštitili samo određeni domen, unesite taj domen u polje "domain".

Da biste zaštitili određenu stranicu, unesite URL stranice u polje "URL ID". Ako imate prilagođenu integraciju sa FastComments, možete ovde uneti tip ID-a umesto URL-a.

#### Security Levels

Kada zahtevate SSO, treba da odlučite da li zahtevate Simple SSO ili Secure SSO. Ako zahtevate Simple SSO, oba su dozvoljena, ali ako zahtevate Secure SSO, sadržaj mora biti preuzet sa Secure SSO payload-om heširanim vašim API key-jem da bi bio vidljiv.

Opcija nivoa bezbednosti će se pojaviti kada izaberete "Require SSO To View Comments".

#### Protection Beyond Reading

Ako je ova opcija omogućena, stranica ili domen će biti zaštićeni od komentarisanja osim ako korisnik nije prijavljen preko SSO.

#### Napomene

Svi korisnici koji su kreirali komentare pre vaše SSO integracije neće moći da ih vide, osim ako se ne prijave putem vaše SSO integracije.