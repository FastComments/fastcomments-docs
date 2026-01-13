[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za autentifikaciju, FastComments zavisi od toga da su u vašem pregledaču omogućeni kolačići treće strane. Bez njih, korisnici će uvek morati
da ostave svoj email da bi komentarisali (osim ako je polje za email sakriveno), i njihovi komentari će po podrazumevanoj postavci uvek biti prikazani kao neverifikovani.

Da biste zaobišli ovo, možete omogućiti zaobilaženje kolačića treće strane. 

Kada je ova postavka omogućena, to će izazvati mali iskačući prozor koji prikazuje poruku da se korisnik prijavljuje. Ovaj popup
se prikazuje kad god korisnik stupi u interakciju sa widgetom za komentare; na primer, ako ostavi komentar.

Ovo možemo uraditi u kodu podešavanjem zastavice **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ovo takođe možemo podesiti putem korisničkog interfejsa za prilagođavanje widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]