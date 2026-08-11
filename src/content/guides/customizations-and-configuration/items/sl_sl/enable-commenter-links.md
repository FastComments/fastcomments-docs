[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Privzeto FastComments zahteva od uporabnika le njihov komentar, uporabniško ime in e‑pošto.

Vendar pa v nekaterih situacijah morda želite, da uporabnik pusti povezavo do svojega bloga ali spletnega mesta.

Lahko omogočimo prikaz dodatnega vnosnega polja za vnos URL-ja spletnega mesta uporabnika, tako da nastavimo zastavico **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Omogočanje povezav komentatorjev'; code-example-end]

Ko je URL naveden, se bo uporabniški račun posodobil in vse njihove uporabniške ime na vseh preteklih in prihodnjih komentarjih bo povezano s tem URL-jem.

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Stran za prilagajanje gradnika s potrditvijo potrditvenega polja povezav komentatorjev, ki doda polje za URL spletnega mesta v obrazec za komentar'; title='Omogočanje povezav komentatorjev' app-screenshot-end]