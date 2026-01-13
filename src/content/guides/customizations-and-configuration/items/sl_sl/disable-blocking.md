[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Privzeto FastComments omogoča uporabnikom, da blokirajo druge uporabnike. Blokiranje uporabnika bo povzročilo, da bodo njegovi komentarji prikriti, preprečilo bo obvestila med uporabnikoma in podobno.

Morda boste želeli onemogočiti to funkcionalnost. To lahko storite tako:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

To je mogoče narediti tudi brez kode, kar omogoča tudi pravilno validacijo na strežniku, prek Widget Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]