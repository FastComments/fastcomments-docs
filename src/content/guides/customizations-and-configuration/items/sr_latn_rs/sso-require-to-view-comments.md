FastComments SSO (<a href="#sso">detalji ovde</a>) pruža vašim korisnicima način da komentarišu bez potrebe da se prijave na drugu platformu.

Međutim, ovo samo po sebi ne osigurava vaše niti komentara, pošto su podaci o komentarima podrazumevano javno dostupni – bilo ko ko može da vidi stranicu može da vidi i komentare.

Promenom jednog podešavanja možemo ograničiti preuzimanje komentara osim ako to ne radi administrator ili validni SSO korisnik.

#### Podešavanje bez koda

Možemo sprečiti pregled i interakciju sa našim nitima komentara, kada je SSO postavljen, kreiranjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagođavanja</a>.

Prilikom toga, potražite SSO i naći ćete ovu opciju:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opcija „Zahtevaj SSO za pregled komentara“ omogućena u pravilu prilagođavanja, sa izborom nivoa sigurnosti'; title='Zahtevaj SSO za pregled komentara' app-screenshot-end]

Omogućite je i sačuvajte pravilo prilagođavanja.

#### Zaštiti samo određeni domen ili stranicu

Da biste zaštitili samo određeni domen ili stranicu, jednostavno ćemo podesiti pravilo prilagođavanja da to uradi.

Na vrhu UI‑ja za prilagođavanje naći ćemo dva polja, Domen i URL ID.

Da biste zaštitili samo određeni domen, unesite taj domen u polje „domain“.

Da biste zaštitili određenu stranicu, unesite URL stranice u polje „URL ID“. Ako imate prilagođenu integraciju sa FastComments, ovde možete uneti tip ID‑ja umesto URL‑a.

#### Nivoi sigurnosti

Kada zahtevate SSO, treba da odlučite da li zahtevate Simple SSO ili Secure SSO. Ako zahtevate Simple SSO, oba su dozvoljena, ali ako zahtevate Secure SSO, sadržaj mora biti preuzet sa Secure SSO payload‑om hash‑ovanom vašim API ključem da bi se mogao pregledati.

Opcija nivoa sigurnosti će se pojaviti kada izaberete "Require SSO To View Comments".

#### Zaštita izvan čitanja

Omogućavanje ove opcije zaštitiće stranicu ili domen od komentarisanja osim ako korisnik nije prijavljen putem SSO.

#### Zamke

Korisnici koji su kreirali komentare pre vaše SSO integracije neće moći da ih vide, osim ako se ne prijave putem vaše SSO integracije.