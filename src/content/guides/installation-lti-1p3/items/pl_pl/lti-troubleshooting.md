#### "Token rejestracyjny nie znaleziony, wygasł lub został już użyty"

Token w Twoim URL rejestracyjnym (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uzyskaj go tutaj</a>) jest ważny przez 30 minut i może być użyty tylko raz. Jeśli Twój LMS potrzebował więcej czasu niż to, lub jeśli rejestracja została ponowiona po pomyślnym zakończeniu, token zostanie odrzucony. Wygeneruj nowy URL na stronie konfiguracji FastComments LTI 1.3 i zacznij od początku.

#### "Platform rejected registration"

Twój LMS odrzucił procedurę rejestracji. Najczęstsze przyczyny:

- **Tool already registered with the same client name.** Niektóre platformy (w szczególności D2L) odrzucają drugą rejestrację "FastComments" dopóki poprzednia nie zostanie usunięta. Usuń stare narzędzie w swoim LMS, a następnie spróbuj ponownie.
- **Wrong field in the LMS.** Upewnij się, że wkleiłeś URL do pola **registration / tool initiation registration endpoint**, a nie do pola launch URL lub login URL.
- **The LMS doesn't actually support Dynamic Registration.** Starsze wersje Moodle i Blackboard reklamują LTI 1.3, ale pozwalają tylko na ręczną konfigurację. Sprawdź dokumentację swojej platformy.

#### "Failed to fetch platform configuration"

FastComments nie mogło odczytać dokumentu openid-configuration Twojego LMS. To rzadkie i zazwyczaj oznacza, że LMS podał niepoprawny lub nieosiągalny discovery URL. Skontaktuj się ze wsparciem swojego LMS.

#### Launch shows "Configuration not found"

Albo konfiguracja w FastComments została usunięta, albo uruchomienie pochodziło z pary `iss`/`client_id`, której nie rozpoznajemy. Jeśli usunąłeś i ponownie zarejestrowałeś, poinstruuj swój LMS, aby usunął i ponownie dodał narzędzie FastComments, aby otrzymało nowe client_id.

#### Launch shows "Deployment not registered"

Uruchomiłeś FastComments z wdrożenia Brightspace/Moodle/Blackboard innego niż to, w którym zostało uruchomione po raz pierwszy. FastComments przypisuje `deployment_id` przy pierwszym uruchomieniu jako kontrolę bezpieczeństwa. Aby dodać nowe wdrożenie pod tym samym klientem, skontaktuj się z pomocą techniczną — dodamy identyfikator wdrożenia do konfiguracji.

#### Launch shows "Unsupported message_type"

LMS wysłał komunikat LTI, którego FastComments nie obsługuje (np. `LtiSubmissionReviewRequest`). FastComments obsługuje tylko standardowy resource-link launch oraz przepływy deep-linking. Skontaktuj się z nami, jeśli potrzebujesz dodania konkretnego typu komunikatu.

#### Iframe doesn't resize

Większość LMS automatycznie dopasowuje rozmiar iframe'ów LTI. Jeśli Twój tego nie robi, sprawdź, czy ustawienia uruchamiania LMS pozwalają narzędziu wysyłać zdarzenia postMessage do ramki nadrzędnej. FastComments wysyła zarówno komunikaty zmiany rozmiaru w stylu Canvas (`lti.frameResize`), jak i zgodne ze specyfikacją IMS (`org.imsglobal.lti.frameResize`).

---