[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments od uporabnika zahteval le njihov komentar, uporabniško ime in e-poštni naslov.

Vendar pa boste v nekaterih primerih morda želeli, da uporabnik vnese povezavo do svojega bloga ali spletne strani.

Prikaz dodatnega polja za vnos URL-ja uporabnikove spletne strani lahko omogočimo z nastavitvijo zastavice **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Ko je omenjeni URL vnesen, bo uporabniški račun posodobljen in njihovo uporabniško ime pri vseh preteklih in prihodnjih komentarjih bo povezano na ta URL.

To lahko prilagodite brez kode, na strani za prilagajanje pripomočka:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]