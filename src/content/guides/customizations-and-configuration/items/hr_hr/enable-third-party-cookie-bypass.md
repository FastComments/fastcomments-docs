[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za autentifikaciju, FastComments ovisi o tome da su kolačići treće strane omogućeni u vašem pregledniku. Bez njih, korisnici će uvijek morati
ostaviti svoju e-poštu da bi komentirali (osim ako je polje za unos e-pošte skriveno), i njihovi će komentari uvijek prikazivati kao neprovjereni (po defaultu).

Da biste to zaobišli, možete omogućiti zaobilaženje kolačića treće strane. 

Kada je ovo podešenje omogućeno, pojavit će se mali popup koji prikazuje poruku da se korisnik prijavljuje. Ovaj popup
se prikazuje kad god korisnik interagira s widgetom komentara; na primjer, kada ostavi komentar.

Ovo možemo napraviti u kodu postavljanjem zastavice **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ovo također možemo postaviti putem sučelja za prilagodbu widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]