#### Sakai

Sakai obsługuje Dynamiczną Rejestrację LTI 1.3 w wydaniach z LTI Advantage. Z poziomu **Administration Workspace**:

1. Zaloguj się jako administrator Sakai i otwórz **Administration Workspace**.
2. Wybierz **External Tools** > **Install LTI 1.3 Tool**.
3. Wklej adres rejestracyjny FastComments i zatwierdź.
4. Zatwierdź narzędzie po zakończeniu procesu nawiązywania połączenia (handshake).

Narzędzie pojawi się wtedy w **External Tools** i może być dodane do witryn przez ich opiekunów.

#### Schoology

Instancje Schoology Enterprise obsługują LTI 1.3, ale dostępność Dynamic Registration zależy od wdrożenia. Skontaktuj się ze swoim opiekunem konta Schoology.

Jeżeli Dynamic Registration nie jest dostępna w Twojej instancji Schoology, musisz skonfigurować integrację ręcznie, używając następujących endpointów:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Po otrzymaniu od Schoology identyfikatora klienta (Client ID) i identyfikatora wdrożenia (Deployment ID), skontaktuj się z pomocą techniczną FastComments, aby zarejestrować konfigurację na swoim tenantcie.

#### Inne platformy LTI 1.3

Każdy LMS, który przestrzega specyfikacji IMS LTI 1.3 Advantage, powinien działać z tym samym adresem rejestracyjnym. Szukaj ustawienia oznaczonego jako "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" lub podobnego.

Jeżeli Twoja platforma obsługuje jedynie ręczną konfigurację LTI 1.3, użyj czterech endpointów wymienionych w sekcji dotyczącej Schoology powyżej i skontaktuj się z pomocą techniczną, aby sfinalizować.