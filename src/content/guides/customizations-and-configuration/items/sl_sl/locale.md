[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Privzeto bo FastComments upodobil pripomoček za komentarje v lokalni nastavitvi, določeni s sistemom in brskalnikom uporabnika.

Ko uporabnik komentira ali se prijavi, posodobimo njegovo zadnjo uporabljeno lokalno nastavitev in to uporabimo tudi pri pošiljanju e-pošte.

To vpliva na to, kako je pripomoček za komentarje preveden za uporabnika. Lokalna nastavitev (locale) vključuje jezik in regijo uporabnika, zato bo spreminjanje lokalne nastavitve običajno spremenilo jezik, v katerem se besedilo prikaže uporabniku.

#### Preko uporabniškega vmesnika

To lahko določite z uporabo vmesnika za prilagajanje pripomočka. Oglejte si možnost "Lokalna nastavitev / Jezik":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Preko kode

To lahko preglasite z želeno lokalno nastavitev.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Podprti jeziki in kodi lokalov

[Celoten seznam podprtih jezikov in pripadajočih kod lokalov najdete tukaj.](/guide-supported-languages.html#supported-languages)

### Opomba za SSO

Če uporabljate SSO, boste morda želeli posredovati uporabnikovo lokalno nastavitev v objektu uporabnika, tako da bodo e-pošta in druge vsebine zanje pravilno lokalizirane.