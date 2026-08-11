[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

For authentication, FastComments depends on third party cookies to be enabled in your browser. Without them, users will always have to
leave their email to comment (unless the email input field is hidden), and their comments will always show as unverified (by default).

Til godkendelse er FastComments afhængig af, at tredjeparts‑cookies er aktiveret i din browser. Uden dem skal brugerne altid indtaste deres e‑mail for at kommentere (medmindre e‑mail‑feltet er skjult), og deres kommentarer vil altid blive vist som uverificerede (som standard).

To get around this, you can enable the third party cookie bypass. 

For at omgå dette kan du aktivere tredjeparts‑cookie‑omgåelsen. 

When this setting is enabled, it will cause a small popup that shows a message saying the user is being logged in. This popup
shows whenever the user interacts with the comment widget; for example, if they leave a comment.

Når denne indstilling er aktiveret, vil den udløse en lille pop‑up, der viser en besked om, at brugeren logger ind. Denne pop‑up vises, når brugeren interagerer med kommentarfunktionen; for eksempel når de efterlader en kommentar.

We can do this in code by setting the **enableThirdPartyCookieBypass** flag to true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Aktivering af tredjeparts cookie-omgåelse'; code-example-end]

We can also set this up via the Widget Customization UI, under `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Widget-tilpasningsside med afkrydsningsfeltet Aktiver tredjeparts cookie-popup markeret'; title='Aktivering af tredjeparts cookie-omgåelse' app-screenshot-end]