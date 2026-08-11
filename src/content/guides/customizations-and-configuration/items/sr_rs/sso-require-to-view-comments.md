FastComments SSO (<a href="#sso">detalji ovde</a>) omogućava vašim korisnicima da komentarišu bez potrebe da se prijave na drugu platformu.

Međutim, ovo samo po sebi ne štiti vaše niti komentara, jer su podaci o komentarima po podrazumevanom javno dostupni – svako ko može da vidi stranicu može da vidi i komentare.

Promenom jedne postavke možemo ograničiti preuzimanje komentara osim ako to ne radi administrator ili validni SSO korisnik.

#### No-Code Setup

Možemo sprečiti pregled i interakciju sa našim nitima komentara, kada je SSO podešen, tako što ćemo kreirati <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilo prilagođavanja</a>.

Pri tome, potražite SSO i naći ćete ovu opciju:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opcija „Zahtevajte SSO“ za pregled komentara omogućena u pravilu prilagođavanja, uz izbor nivoa sigurnosti'; title='Zahtevajte SSO za pregled komentara' app-screenshot-end]

Omogućite je i sačuvajte pravilo prilagođavanja.

#### Only Protect a Certain Domain or Page

Da biste zaštitili samo određeni domen ili stranicu, jednostavno ćemo podesiti pravilo prilagođavanja da to uradi.

Na vrhu UI‑ja za prilagođavanje naći ćete dva polja, Domen i URL ID.

Da biste zaštitili određeni domen, unesite taj domen u polje "domain".

Da biste zaštitili određenu stranicu, unesite URL stranice u polje "URL ID". Ako imate prilagođenu integraciju sa FastComments, ovde možete uneti tip ID‑ja umesto URL‑a.

#### Security Levels

Kada zahtevate SSO, treba da odlučite da li želite jednostavni SSO ili siguran SSO. Ako izaberete jednostavni SSO, oba su dozvoljena, ali ako izaberete siguran SSO, sadržaj mora biti preuzet uz Secure SSO payload koji je hash‑ovan vašim API ključem da bi se mogao pregledati.

Opcija nivoa sigurnosti će se pojaviti kada izaberete "Require SSO To View Comments".

#### Protection Beyond Reading

Omogućavanje ove opcije zaštitiće stranicu ili domen od komentarisanja, osim ako korisnik nije prijavljen putem SSO.

#### Gotchas

Korisnici koji su kreirali komentare pre vaše SSO integracije neće moći da ih vide, osim ako se ne prijave putem vaše SSO integracije.