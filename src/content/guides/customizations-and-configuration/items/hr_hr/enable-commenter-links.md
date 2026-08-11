[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će tražiti od korisnika samo njihov komentar, korisničko ime i e‑mail adresu.

Međutim, u nekim situacijama možda ćete željeti da korisnik ostavi poveznicu na svoj blog ili web stranicu.

Možemo omogućiti prikaz dodatnog polja za unos URL‑a web stranice korisnika postavljanjem zastavice **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Omogućavanje veza komentatora'; code-example-end]

Kada se navedeni URL unese, korisnički račun će se ažurirati i sve njihove korisničko ime na svim prošlim i budućim komentarima će biti povezano s tim URL‑om.

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Stranica prilagodbe widgeta s označenim potvrdnim okvirom za veze komentatora kako bi se dodalo polje za URL web stranice u obrazac za komentar'; title='Omogućavanje veza komentatora' app-screenshot-end]