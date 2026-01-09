[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podršku za spoilere možemo omogućiti postavljanjem zastavice **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Kada se označi tekst i klikne na sada vidljivo dugme `SPOILER`, tekst će biti zamaskiran dok korisnik ne pređe mišem preko njega. Za tamni režim radimo isto, sa drugačijim
bojama koje bolje odgovaraju tamnom režimu.

Ovo je takođe kompatibilno sa WYSIWYG editorom.