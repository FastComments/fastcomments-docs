[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

S FastComments je vse besedilo v pripomočku za komentarje prilagodljivo.

Lahko prepišete posamezen del besedila, kot je gumb za pošiljanje, ali vso besedilo v celotnem pripomočku za komentarje.

Privzeto je besedilo v pripomočku za komentarje prevedeno glede na uporabnikovo lokalno nastavitve. Vendar lahko besedilo prepišemo, če smo prepričani, da naša baza uporabnikov uporablja enako lokalno/jezikovno nastavitve, na primer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Vse prilagodljive prevode lahko najdete <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">tukaj</a> pod zavihkom "napredne možnosti".

Vendar obstaja preprostejši način, prek uporabniškega vmesnika za prilagajanje pripomočka. Tam lahko preprosto najdemo besedilo, ki se prikaže v pripomočku za komentarje v lokalni nastavitvi EN_US, in določimo nadomestilo.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Plošča po meri besedila z izbranim nizom pripomočka iz spustnega menija in poljem za nadomestno besedilo'; title='Po meri besedilo' app-screenshot-end]

Vsi prepisani prevodi trenutno vplivajo na vse lokalne nastavitve.