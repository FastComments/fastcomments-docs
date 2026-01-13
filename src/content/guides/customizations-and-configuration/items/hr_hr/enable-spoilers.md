[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Možemo omogućiti podršku za spoilere postavljanjem zastavice **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Ovo se može učiniti i bez koda. Na stranici za prilagodbu widgeta pogledajte opciju "Omogući spoilere".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Kada je tekst označen, i sada vidljivi `SPOILER` gumb kliknut, tekst će biti zamaskiran dok korisnik ne prijeđe mišem preko njega. Za tamni način rada radimo isto, s različitim
bojama koje bolje odgovaraju tamnom načinu rada.

Ovo je također kompatibilno s WYSIWYG uređivačem.