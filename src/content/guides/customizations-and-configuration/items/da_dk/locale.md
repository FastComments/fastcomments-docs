[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Som standard viser FastComments kommenteringswidgeten i den locale, der bestemmes af brugerens system og browser.

Når en bruger kommenterer eller logger ind, opdaterer vi deres sidst anvendte locale og bruger denne til også at sende e-mails.

Dette påvirker, hvordan kommenteringswidgeten oversættes for brugeren. Locale består af brugerens sprog og region, så konfiguration af locale vil normalt ændre det sprog, der bruges til at vise tekst for brugeren.

#### Via brugergrænsefladen

Dette kan defineres ved hjælp af widgetens tilpasnings-UI. Se indstillingen "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via kode

Dette kan tilsidesættes med en ønsket locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Understøttede sprog og locale-koder

[Du kan finde den komplette liste over understøttede sprog og de tilsvarende locale-koder her.](/guide-supported-languages.html#supported-languages)

### Bemærkning vedr. SSO

Hvis du bruger SSO, vil du måske sende brugerens locale i brugerobjektet, så e-mails og andre ting bliver korrekt lokaliseret for dem.