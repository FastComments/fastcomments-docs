[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za overjanje FastComments potrebuje, da so v vašem brskalniku omogočeni piškotki tretjih oseb. Brez njih bodo uporabniki vedno morali
pustiti svoj e-poštni naslov za komentiranje (razen če je polje za vnos e-pošte skrito), njihovi komentarji pa bodo privzeto vedno prikazani kot nepreverjeni.

Da se temu izognete, lahko omogočite obhod tretjih piškotkov. 

Ko je ta nastavitev omogočena, se bo pojavilo majhno pojavno okno, ki prikazuje sporočilo, da poteka prijava uporabnika. To pojavno okno
se prikaže vsakič, ko uporabnik komunicira z widgetom za komentarje; na primer, če odda komentar.

To lahko naredimo v kodi tako, da zastavico **enableThirdPartyCookieBypass** nastavimo na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

To lahko tudi nastavite preko uporabniškega vmesnika za prilagajanje widgeta, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---