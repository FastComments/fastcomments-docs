[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za autentifikaciju, FastComments zavisi od toga da su kolačići trećih strana omogućeni u vašem pregledaču. Bez njih, korisnici će uvek morati da
ostave svoj email da bi komentarisali (osim ako je polje za unos email-a sakriveno), i njihovi komentari će uvek biti prikazani kao neverifikovani (po podrazumevanju).

Da biste zaobišli ovo, možete omogućiti zaobilaženje kolačića trećih strana.

Kada je ovo podešavanje omogućeno, pojaviće se mali popup koji prikazuje poruku da se korisnik prijavljuje. Ovaj popup
se prikazuje kad god korisnik interaguje sa widgetom za komentare; na primer, ako ostavi komentar.

Ovo možemo uraditi u kodu postavljanjem zastavice **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ovo takođe možemo podesiti putem UI-ja za prilagođavanje widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Stranica za prilagođavanje widgeta sa označenim poljem Enable Third-Party Cookie Popup'; title='Omogućavanje zaobilaženja kolačića trećih strana' app-screenshot-end]

---