[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će od korisnika tražiti samo njihov komentar, korisničko ime i e-poštu.

Međutim, u nekim situacijama možda ćete htjeti da korisnik ostavi poveznicu na vlastiti blog ili web-stranicu.

Možemo omogućiti prikaz dodatnog polja za unos URL-a korisnikove web-stranice postavljanjem zastavice **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Kada je taj URL naveden, korisnički račun će biti ažuriran i svi njihovi korisnički nazivi na svim prošlim i budućim komentarima vodit će na taj URL.

Ovo se može prilagoditi bez pisanja koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]