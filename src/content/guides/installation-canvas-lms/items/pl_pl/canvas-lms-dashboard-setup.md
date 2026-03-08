#### Przejdź do konfiguracji Canvas LTI

Zaloguj się na swoje konto FastComments i przejdź do <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Moje konto > Konfiguracja Canvas LTI</a>.

#### Utwórz nową konfigurację LTI

Zaznacz pole wyboru **Włącz LTI**. Pojawią się pola konfiguracyjne:

- **Nazwa konfiguracji** - opcjonalna etykieta identyfikująca tę konfigurację (przydatne, jeśli łączysz kilka instancji Canvas).
- **Platform URL** - adres URL twojej instancji Canvas (np. `https://yourschool.instructure.com`). Możesz zostawić to pole puste na razie i uzupełnić je po utworzeniu klucza dewelopera.
- **ID klienta (Client ID)** - zostaw to pole puste na razie. Wypełnisz je po utworzeniu klucza dewelopera w Canvas.
- **ID wdrożenia (Deployment ID)** - zostaw to pole puste na razie.
- **Styl komentarzy** - wybierz między Komentarze, Collab Chat lub oba. Zobacz [Style komentowania](#canvas-lms-commenting-styles) po szczegóły.

Kliknij **Dodaj**, aby utworzyć konfigurację.

#### Skopiuj adresy URL konfiguracji

Po zapisaniu pojawią się trzy adresy URL:

- **Configuration URL** - wkleisz to w Canvas podczas tworzenia klucza dewelopera.
- **OIDC Login URL** - używany przez Canvas do procesu logowania LTI (automatycznie skonfigurowany przez Configuration URL).
- **Launch URL** - punkt końcowy wywoływany przez Canvas, gdy student otwiera FastComments (automatycznie skonfigurowany przez Configuration URL).

Skopiuj **Configuration URL**. Będziesz go potrzebować w następnym kroku.