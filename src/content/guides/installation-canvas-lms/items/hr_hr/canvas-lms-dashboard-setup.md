#### Navigirajte do Canvas LTI konfiguracije

Prijavite se na svoj FastComments račun i idite na <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Moj račun > Canvas LTI konfiguracija</a>.

#### Kreirajte novu LTI konfiguraciju

Označite potvrdni okvir **Omogući LTI**. Pojavit će se polja za konfiguraciju:

- **Naziv konfiguracije** - neobavezna oznaka za identificiranje ove konfiguracije (korisno ako povežete više Canvas instanci).
- **Platform URL** - URL vaše Canvas instance (npr. `https://yourschool.instructure.com`). Možete ovo za sada ostaviti prazno i popuniti ga nakon stvaranja Developer Key.
- **Client ID** - za sada ostavite prazno. Popunit ćete ga nakon stvaranja Developer Key u Canvasu.
- **Deployment ID** - za sada ostavite prazno.
- **Stil komentiranja** - odaberite između Komentara, Collab Chat ili Oboje. Pogledajte [Commenting Styles](#canvas-lms-commenting-styles) za detalje.

Kliknite **Dodaj** za stvaranje konfiguracije.

#### Kopirajte URL-ove konfiguracije

Nakon spremanja pojavit će se tri URL-a:

- **Configuration URL** - zalijepit ćete ga u Canvas prilikom stvaranja Developer Key.
- **OIDC Login URL** - Canvas koristi za LTI prijavni tijek (automatski konfigurirano putem Configuration URL).
- **Launch URL** - endpoint koji Canvas poziva kada student otvori FastComments (automatski konfigurirano putem Configuration URL).

Kopirajte **Configuration URL**. Trebat će vam u sljedećem koraku.