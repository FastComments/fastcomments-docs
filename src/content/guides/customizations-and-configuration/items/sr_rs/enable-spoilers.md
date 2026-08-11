[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Možemo omogućiti podršku za spoiler postavljanjem **enableSpoilers** zastavice na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Omogućavanje Spoilera'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju „Enable Spoilers“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Stranica za prilagođavanje widgeta sa označenim poljem Enable Spoilers da bi se dodalo dugme SPOILER u editor'; title='Omogući Spoilere' app-screenshot-end]

Kada je tekst označen, a sada vidljivo dugme `SPOILER` kliknuto, tekst će biti maskiran dok korisnik ne pređe mišem preko njega. Za tamni režim radimo isto, sa različitim bojama koje bolje odgovaraju tamnom režimu.

Ovo je takođe kompatibilno sa WYSIWYG editorom.