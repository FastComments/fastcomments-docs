[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Prema zadatim podešavanjima, FastComments će od korisnika tražiti samo njihov komentar, korisničko ime i e-poštu.

Međutim, u nekim situacijama možda ćete želeti da korisnik ostavi link ka svom blogu ili veb-sajtu.

Možemo omogućiti prikaz dodatnog polja za unos kako bi korisnik ostavio URL svog sajta tako što ćemo postaviti zastavicu **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Kada se taj URL navede, nalog korisnika će biti ažuriran i sva njihova korisnička imena na svim prošlim i budućim komentarima biće povezana sa ovim URL-om.

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---