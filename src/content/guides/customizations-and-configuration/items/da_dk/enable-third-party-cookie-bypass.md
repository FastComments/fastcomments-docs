[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

For authentication, FastComments depends on third party cookies to be enabled in your browser. Without them, users will always have to
leave their email to comment (unless the email input field is hidden), and their comments will always show as unverified (by default).

Til autentificering er FastComments afhængig af, at tredjepartscookies er aktiveret i din browser. Uden dem vil brugere altid skulle
angive deres e-mail for at kommentere (medmindre e-mailinputfeltet er skjult), og deres kommentarer vil som standard altid blive vist som uverificerede.

To get around this, you can enable the third party cookie bypass.

For at omgå dette kan du aktivere omgåelse af tredjepartscookies.

When this setting is enabled, it will cause a small popup that shows a message saying the user is being logged in. This popup
shows whenever the user interacts with the comment widget; for example, if they leave a comment.

Når denne indstilling er aktiveret, vil den udløse et lille popup-vindue, der viser en besked om, at brugeren logges ind. Dette popupvindue
vises hver gang brugeren interagerer med kommentarboksen; for eksempel hvis de efterlader en kommentar.

We can do this in code by setting the **enableThirdPartyCookieBypass** flag to true:

Det kan vi gøre i koden ved at sætte flaget **enableThirdPartyCookieBypass** til true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

We can also set this up via the Widget Customization UI, under `Enable Third-Party Cookie Popup`:

Vi kan også sætte dette op via Widget-tilpasnings-UI'en under `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---