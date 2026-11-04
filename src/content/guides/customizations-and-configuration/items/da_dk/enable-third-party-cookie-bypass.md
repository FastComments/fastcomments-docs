[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Til autentificering er FastComments afhængig af, at tredjepartscookies er aktiveret i din browser. Uden dem skal brugerne altid efterlade deres e-mail for at kommentere (medmindre e-mail-inputfeltet er skjult), og deres kommentarer vil altid blive vist som uverificerede (som standard).

For at omgå dette kan du aktivere tredjepartscookie-bypass.

Når denne indstilling er aktiveret, vil det medføre en lille popup, der viser en besked om, at brugeren bliver logget ind. Denne popup vises, når brugeren interagerer med kommentar-widget'en; for eksempel, hvis de efterlader en kommentar.

Vi kan gøre dette i kode ved at sætte flaget **enableThirdPartyCookieBypass** til true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Aktivering af tredjepartscookie-bypass'; code-example-end]

Vi kan også konfigurere dette via Widget Customization UI, under `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Widget-tilpasningsside med afkrydsningsfeltet Enable Third-Party Cookie Popup markeret'; title='Aktivering af tredjepartscookie-bypass' app-screenshot-end]