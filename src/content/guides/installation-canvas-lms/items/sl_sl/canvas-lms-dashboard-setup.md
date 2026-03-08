#### Pojdite na Canvas LTI Config

Prijavite se v svoj FastComments račun in pojdite na <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Označite polje za preverjanje **Enable LTI**. Pojavila se bodo polja konfiguracije:

- **Configuration Name** - izbirna oznaka za identifikacijo te konfiguracije (uporabno, če povežete več instanc Canvas).
- **Platform URL** - URL vaše instance Canvas (npr. `https://yourschool.instructure.com`). To lahko za zdaj pustite prazno in ga izpolnite po ustvarjanju Developer Key.
- **Client ID** - za zdaj pustite prazno. Izpolnili ga boste po ustvarjanju Developer Key v Canvasu.
- **Deployment ID** - za zdaj pustite prazno.
- **Comment Style** - izberite med Comments, Collab Chat ali Both. Oglejte si [Slogi komentiranja](#canvas-lms-commenting-styles) za podrobnosti.

Kliknite **Add**, da ustvarite konfiguracijo.

#### Copy the Configuration URLs

Po shranjevanju se bodo prikazali trije URL-ji:

- **Configuration URL** - ta boste prilepili v Canvas pri ustvarjanju Developer Key.
- **OIDC Login URL** - Canvas ga uporablja za LTI prijavni tok (samodejno konfigurirano prek Configuration URL).
- **Launch URL** - končna točka, ki jo Canvas pokliče, ko študent odpre FastComments (samodejno konfigurirano prek Configuration URL).

Kopirajte **Configuration URL**. Potrebovali ga boste v naslednjem koraku.