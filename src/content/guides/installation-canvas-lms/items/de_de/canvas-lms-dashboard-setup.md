#### Navigieren Sie zu Canvas LTI Config

Melden Sie sich bei Ihrem FastComments-Konto an und gehen Sie zu <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Neue LTI-Konfiguration erstellen

Aktivieren Sie das Kontrollkästchen **Enable LTI**. Die Konfigurationsfelder werden angezeigt:

- **Configuration Name** - eine optionale Bezeichnung zur Identifizierung dieser Konfiguration (nützlich, wenn Sie mehrere Canvas-Instanzen verbinden).
- **Platform URL** - die URL Ihrer Canvas-Instanz (z. B. `https://yourschool.instructure.com`). Sie können dieses Feld vorerst leer lassen und es nach dem Erstellen des Developer Key ausfüllen.
- **Client ID** - lassen Sie dieses Feld vorerst leer. Sie werden es nach dem Erstellen des Developer Key in Canvas ausfüllen.
- **Deployment ID** - lassen Sie dieses Feld vorerst leer.
- **Comment Style** - wählen Sie zwischen Comments, Collab Chat oder Both. Siehe [Commenting Styles](#canvas-lms-commenting-styles) für Details.

Klicken Sie auf **Add**, um die Konfiguration zu erstellen.

#### Konfigurations-URLs kopieren

Nach dem Speichern erscheinen drei URLs:

- **Configuration URL** - fügen Sie dies in Canvas ein, wenn Sie den Developer Key erstellen.
- **OIDC Login URL** - von Canvas für den LTI-Anmeldefluss verwendet (wird automatisch über die Configuration URL konfiguriert).
- **Launch URL** - der Endpunkt, den Canvas aufruft, wenn ein Studierender FastComments öffnet (wird automatisch über die Configuration URL konfiguriert).

Kopieren Sie die **Configuration URL**. Sie benötigen sie im nächsten Schritt.