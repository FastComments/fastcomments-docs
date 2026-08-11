[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će tražiti od korisnika samo njihov komentar, korisničko ime i email.

Međutim, u nekim situacijama možda ćete želeti da korisnik ostavi link ka svom blogu ili veb sajtu.

Možemo omogućiti prikaz dodatnog polja za unos URL-a veb sajta korisnika tako što ćemo postaviti zastavicu **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Kada se navedeni URL unese, nalog korisnika će biti ažuriran i sve njihove korisničke nazive na svim prošlim i budućim komentarima će biti povezani na ovaj URL.

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Stranica za prilagođavanje widgeta sa čekiranim poljem za linkove komentatora kako bi se dodalo polje za URL veb sajta u formu za komentar'; title='Omogućavanje linkova komentatora' app-screenshot-end]

---