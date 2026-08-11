[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za autentifikaciju, FastComments zavisi od toga da su kolačići treće strane omogućeni u vašem pregledaču. Bez njih, korisnici će uvek morati da ostave svoj email da bi komentarisali (osim ako je polje za unos email-a sakriveno), i njihovi komentari će uvek biti prikazani kao neverifikovani (podrazumevano).

Da biste zaobišli ovo, možete omogućiti zaobilaženje kolačića treće strane. 

Kada je ovo podešavanje omogućeno, pojaviće se mali popup koji prikazuje poruku da se korisnik prijavljuje. Ovaj popup se prikazuje kad god korisnik interaguje sa widgetom za komentare; na primer, ako ostavi komentar.

Možemo to uraditi u kodu postavljanjem zastavice **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Omogućavanje zaobilaženja kolačića treće strane'; code-example-end]

Ovo takođe možemo podesiti putem UI-ja za prilagođavanje widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Stranica za prilagođavanje widgeta sa označenim poljem Omogući popup za kolačiće treće strane'; title='Omogućavanje zaobilaženja kolačića treće strane' app-screenshot-end]