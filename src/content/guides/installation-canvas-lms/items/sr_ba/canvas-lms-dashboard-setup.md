#### Navigate to Canvas LTI Config

Prijavite se na svoj FastComments nalog i idite na <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Moj nalog > Canvas LTI konfiguracija</a>.

#### Create a New LTI Configuration

Označite polje **Enable LTI**. Pojaviće se polja konfiguracije:

- **Configuration Name** - opciona oznaka za identifikaciju ove konfiguracije (korisno ako povežete više Canvas instanci).
- **Platform URL** - URL vaše Canvas instance (e.g. `https://yourschool.instructure.com`). Možete ovo ostaviti prazno za sada i popuniti ga nakon kreiranja Developer Key.
- **Client ID** - za sada ostavite prazno. Popunićete ga nakon kreiranja Developer Key u Canvasu.
- **Deployment ID** - za sada ostavite prazno.
- **Comment Style** - izaberite između Comments, Collab Chat, ili Both. Pogledajte [Commenting Styles](#canvas-lms-commenting-styles) za detalje.

Kliknite **Add** da kreirate konfiguraciju.

#### Copy the Configuration URLs

Nakon spremanja, pojaviće se tri URL-a:

- **Configuration URL** - zalijepit ćete ovo u Canvas prilikom kreiranja Developer Key.
- **OIDC Login URL** - Canvas koristi za LTI prijavni tok (automatski konfigurisan putem Configuration URL).
- **Launch URL** - endpoint koji Canvas poziva kada student otvori FastComments (automatski konfigurisan putem Configuration URL).

Kopirajte **Configuration URL**. Trebaće vam u sljedećem koraku.