[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Som standard vil FastComments gengive kommentarfunktionen i den lokalitet, der er bestemt af brugerens system og browser.

Når en bruger kommenterer eller logger ind, opdaterer vi deres sidst brugte lokalitet og bruger den også til at sende e‑mails.

Dette påvirker, hvordan kommentarfunktionen oversættes for brugeren. En lokalitet består af brugerens sprog og region, så konfiguration af lokalitet vil normalt ændre det sprog, der vises til brugeren.

#### Via UI'en

Dette kan defineres via widget‑tilpasnings‑UI'en. Se indstillingen "Locale / Language" option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Locale / Language‑rullemenu på widget‑tilpasningssiden, der bruges til at tilsidesætte den besøgendes registrerede lokalitet'; title='Ændring af lokalitet / sprog' app-screenshot-end]

#### Via kode

Dette kan tilsidesættes med en ønsket lokalitet.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manuel definition af brugerens lokalitet'; code-example-end]

### Understøttede sprog og lokalitetskoder

[Du kan finde den komplette liste over understøttede sprog og de tilsvarende lokalitetskoder her.](/guide-supported-languages.html#supported-languages)

### SSO‑bemærkning

Hvis du bruger SSO, vil du måske sende brugerens lokalitet i brugerobjektet, så e‑mails og andre ting lokales korrekt for dem.