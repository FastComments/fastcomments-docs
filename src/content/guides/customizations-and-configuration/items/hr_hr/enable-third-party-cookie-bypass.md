[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za autentifikaciju, FastComments ovisi o tome da su kolačići treće strane omogućeni u vašem pregledniku. Bez njih, korisnici će uvijek morati
ostaviti svoju e‑mail adresu za komentar (osim ako je polje za unos e‑maila skriveno), i njihovi će komentari uvijek biti prikazani kao neprovjereni (prema zadanim postavkama).

Da biste to zaobišli, možete omogućiti zaobilaženje kolačića treće strane. 

Kada je ova postavka omogućena, pojavit će se mali skočni prozor koji prikazuje poruku da se korisnik prijavljuje. Ovaj skočni prozor
prikazuje se kad god korisnik komunicira s widgetom za komentare; na primjer, ako ostavi komentar.

Ovo možemo učiniti u kodu postavljanjem zastavice **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Omogućavanje zaobilaženja kolačića treće strane'; code-example-end]

Ovo također možete postaviti putem sučelja za prilagodbu widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Stranica prilagodbe widgeta s označenim potvrdnim okvirom Omogući skočni prozor kolačića treće strane'; title='Omogućavanje zaobilaženja kolačića treće strane' app-screenshot-end]