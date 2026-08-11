[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Za avtentikacijo FastComments zahteva, da so v vašem brskalniku omogočeni piškotki tretjih oseb. Brez njih bodo uporabniki vedno morali
pustiti svoj e‑mail, da komentirajo (razen če je polje za e‑mail skrito), in njihovi komentarji bodo privzeto prikazani kot neverificirani.

Da to zaobidete, lahko omogočite zaobidenje piškotkov tretjih oseb. 

Ko je ta nastavitev omogočena, bo povzročila majhno pojavno okno, ki prikaže sporočilo, da je uporabnik prijavljen. To pojavno okno
se prikaže vsakič, ko uporabnik sodeluje z gradnikom za komentarje; na primer, če pusti komentar.

To lahko storimo v kodi tako, da nastavimo zastavico **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Omogočanje zaobidenja piškotkov tretjih oseb'; code-example-end]

To lahko nastavite tudi prek uporabniškega vmesnika za prilagajanje gradnika, pod `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Stran prilagajanja gradnika z označeno potrditveno polje Omogoči pojavno okno piškotkov tretjih oseb'; title='Omogočanje zaobidenja piškotkov tretjih oseb' app-screenshot-end]

---