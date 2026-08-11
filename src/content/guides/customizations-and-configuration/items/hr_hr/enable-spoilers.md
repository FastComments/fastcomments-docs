[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Možemo omogućiti podršku za spoiler postavljanjem zastavice **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Omogućavanje Spoilera'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte opciju "Enable Spoilers" opciju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Stranica za prilagodbu widgeta s označenim potvrdnim okvirom Enable Spoilers kako bi se dodalo SPOILER dugme u editor'; title='Omogući Spoilere' app-screenshot-end]

Kada je tekst označen, a sada vidljivo dugme `SPOILER` kliknuto, tekst će biti maskiran dok korisnik ne prijeđe mišem preko njega. Za tamni način rada radimo isto, s različitim bojama koje bolje odgovaraju tamnom načinu rada.

Ovo je također kompatibilno s WYSIWYG editorom.