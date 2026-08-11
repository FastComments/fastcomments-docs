[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Privzeto bo FastComments prikazal pripomoček za komentarje v lokalizaciji, ki jo določi uporabnikov sistem in brskalnik.

Ko uporabnik komentira ali se prijavi, posodobimo njegovo zadnjo uporabljeno lokalizacijo in jo uporabimo tudi za pošiljanje e-pošte.

To vpliva na to, kako je pripomoček za komentarje preveden za uporabnika. Lokalizacija sestavlja uporabnikov jezik in regija, zato bo nastavitev lokalizacije običajno spremenila jezik, ki se uporablja za prikaz besedila uporabniku.

#### Preko uporabniškega vmesnika

To je mogoče določiti prek uporabniškega vmesnika za prilagajanje pripomočka. Oglejte si možnost "Lokalizacija / Jezik":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Spustni meni Lokalizacija / Jezik na strani za prilagajanje pripomočka, ki se uporablja za preglasitev zaznane lokalizacije obiskovalca'; title='Spreminjanje lokalizacije / jezika' app-screenshot-end]

#### Preko kode

To je mogoče preglasiti z želeno lokalizacijo.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Ročno določanje uporabnikove lokalizacije'; code-example-end]

### Podprti jeziki in kode lokalizacije

[Tukaj lahko najdete popoln seznam podprtih jezikov in ustreznih kod lokalizacije.](/guide-supported-languages.html#supported-languages)

### Opomba o SSO

Če uporabljate SSO, boste morda želeli v uporabniškem objektu posredovati uporabnikovo lokalizacijo, da bodo e-poštna sporočila in druge stvari pravilno lokalizirane zanj.

---