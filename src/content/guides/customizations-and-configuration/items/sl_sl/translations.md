[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

S FastComments je vse besedilo v pripomočku za komentarje prilagodljivo.

Lahko preglasite en sam del besedila, na primer gumb za oddajo, ali pa vse besedilo v celotnem pripomočku za komentarje.

Privzeto se besedilo v pripomočku za komentarje prevaja glede na lokalizacijo uporabnika. Vendar lahko besedilo preglasimo, če smo prepričani,
da večina naših uporabnikov uporablja isto lokalizacijo/jezik, na primer:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Prilagojeno besedilo'; code-example-end]

Vse prilagodljive prevode najdete <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">tukaj</a> pod zavihek "napredne možnosti".

Obstaja pa lažji način, prek uporabniškega vmesnika za prilagajanje pripomočka. Tam preprosto poiščemo besedilo, ki se prikazuje v pripomočku za komentiranje v lokalizaciji EN_US, in določimo nadomestilo.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Prilagojeno besedilo' app-screenshot-end]

Vse spremembe prevodov trenutno vplivajo na vse lokalizacije.