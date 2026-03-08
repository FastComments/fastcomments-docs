#### Navigeer naar Canvas LTI Config

Meld u aan bij uw FastComments-account en ga naar <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Maak een nieuwe LTI-configuratie

Vink het selectievakje **Enable LTI** aan. De configuratievelden verschijnen:

- **Configuration Name** - een optioneel label om deze configuratie te identificeren (handig als u meerdere Canvas-instanties verbindt).
- **Platform URL** - de URL van uw Canvas-instantie (bijv. `https://yourschool.instructure.com`). u kunt dit voorlopig leeg laten en invullen nadat u de Developer Key heeft gemaakt.
- **Client ID** - laat dit voorlopig leeg. u vult dit in nadat u de Developer Key in Canvas heeft aangemaakt.
- **Deployment ID** - laat dit voorlopig leeg.
- **Comment Style** - kies tussen Comments, Collab Chat, of Both. Zie [Commenting Styles](#canvas-lms-commenting-styles) voor details.

Klik op **Add** om de configuratie te maken.

#### Kopieer de Configuration URLs

Na het opslaan verschijnen drie URL's:

- **Configuration URL** - u plakt dit in Canvas wanneer u de Developer Key maakt.
- **OIDC Login URL** - gebruikt door Canvas voor de LTI-aanmeldingsstroom (automatisch geconfigureerd via de Configuration URL).
- **Launch URL** - het eindpunt dat Canvas aanroept wanneer een student FastComments opent (automatisch geconfigureerd via de Configuration URL).

Kopieer de **Configuration URL**. u heeft deze nodig in de volgende stap.