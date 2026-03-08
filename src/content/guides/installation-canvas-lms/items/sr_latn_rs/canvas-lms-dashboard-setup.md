#### Idite na Canvas LTI Config

Prijavite se na svoj FastComments nalog i idite na <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Kreirajte novu LTI konfiguraciju

Označite polje za potvrdu **Enable LTI**. Pojaviće se polja za konfiguraciju:

- **Configuration Name** - an optional label to identify this configuration (useful if you connect multiple Canvas instances).
- **Platform URL** - your Canvas instance URL (e.g. `https://yourschool.instructure.com`). You can leave this blank for now and fill it in after creating the Developer Key.
- **Client ID** - leave this blank for now. You will fill it in after creating the Developer Key in Canvas.
- **Deployment ID** - leave this blank for now.
- **Comment Style** - choose between Comments, Collab Chat, or Both. See [Commenting Styles](#canvas-lms-commenting-styles) for details.

Kliknite **Add** da kreirate konfiguraciju.

#### Kopirajte Configuration URL-ove

Nakon čuvanja pojaviće se tri URL-a:

- **Configuration URL** - you will paste this into Canvas when creating the Developer Key.
- **OIDC Login URL** - used by Canvas for the LTI login flow (automatically configured via the Configuration URL).
- **Launch URL** - the endpoint Canvas calls when a student opens FastComments (automatically configured via the Configuration URL).

Kopirajte **Configuration URL**. Trebaće vam u narednom koraku.