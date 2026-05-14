#### Sakai

Sakai obsługuje Dynamiczną Rejestrację LTI 1.3 w wydaniach z LTI Advantage. Z poziomu Obszaru administracyjnego:

1. Zaloguj się jako administrator Sakai i otwórz **Obszar administracyjny**.
2. Wybierz **Narzędzia zewnętrzne** > **Zainstaluj narzędzie LTI 1.3**.
3. Wklej adres rejestracyjny FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uzyskaj go tutaj</a>) i wyślij.
4. Zatwierdź narzędzie po zakończeniu procedury nawiązywania połączenia (handshake).

Narzędzie pojawi się następnie w **Narzędziach zewnętrznych** i może być dodane do witryn przez ich opiekunów.

#### Schoology

Instancje Schoology Enterprise obsługują LTI 1.3, ale dostępność rejestracji dynamicznej różni się w zależności od wdrożenia. Skonsultuj się ze swoim opiekunem konta Schoology.

Jeśli rejestracja dynamiczna nie jest dostępna na Twojej instancji Schoology, będziesz musiał skonfigurować integrację ręcznie, używając następujących punktów końcowych:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Po otrzymaniu od Schoology identyfikatora klienta (Client ID) i identyfikatora wdrożenia (Deployment ID), skontaktuj się z pomocą techniczną FastComments, aby zarejestrować konfigurację na swojej instancji.

#### Inne platformy LTI 1.3

Każdy LMS, który przestrzega specyfikacji IMS LTI 1.3 Advantage, powinien działać z tym samym adresem rejestracyjnym (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uzyskaj go tutaj</a>). Szukaj ustawienia oznaczonego jako "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" lub podobnego.

Jeśli Twoja platforma obsługuje jedynie ręczną konfigurację LTI 1.3, użyj czterech punktów końcowych wymienionych w sekcji Schoology powyżej i skontaktuj się z pomocą techniczną, aby dokończyć konfigurację.