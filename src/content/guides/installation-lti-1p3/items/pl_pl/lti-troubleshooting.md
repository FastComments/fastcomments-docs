#### "Token rejestracyjny nie znaleziony, wygasł lub został już użyty"

Token w URL rejestracyjnym jest ważny przez 30 minut i może być użyty tylko raz. Jeśli Twój LMS potrzebował więcej czasu, lub jeśli rejestracja została ponowiona po zakończeniu sukcesem, token zostanie odrzucony. Wygeneruj świeży URL na stronie konfiguracji FastComments LTI 1.3 i zacznij od nowa.

#### "Platforma odrzuciła rejestrację"

Twój LMS odrzucił procedurę rejestracji. Najczęstsze przyczyny:

- **Narzędzie już zarejestrowane pod taką samą nazwą klienta.** Niektóre platformy (w szczególności D2L) odrzucają drugą rejestrację "FastComments" dopóki poprzednia nie zostanie usunięta. Usuń stare narzędzie w swoim LMS, a następnie spróbuj ponownie.
- **Złe pole w LMS.** Upewnij się, że wkleiłeś URL do pola **registration / tool initiation registration endpoint**, a nie do pola launch URL lub login URL.
- **LMS w rzeczywistości nie obsługuje Dynamic Registration.** Starsze wersje Moodle i Blackboard deklarują LTI 1.3, ale pozwalają jedynie na konfigurację ręczną. Sprawdź dokumentację swojej platformy.

#### "Nie udało się pobrać konfiguracji platformy"

FastComments nie był w stanie odczytać dokumentu openid-configuration Twojego LMS. To rzadkie i zwykle oznacza, że LMS podał sfałszowany lub niedostępny discovery URL. Skontaktuj się z pomocą techniczną LMS.

#### Uruchomienie pokazuje "Nie znaleziono konfiguracji"

Albo konfiguracja w FastComments została usunięta, albo uruchomienie pochodziło z pary `iss`/`client_id`, której nie rozpoznajemy. Jeśli usunąłeś i ponownie zarejestrowałeś, poproś swój LMS o usunięcie i ponowne dodanie narzędzia FastComments, aby otrzymało nowe client_id.

#### Uruchomienie pokazuje "Nie zarejestrowano wdrożenia"

Uruchomiłeś FastComments z wdrożenia Brightspace/Moodle/Blackboard innego niż to, w którym zostało uruchomione po raz pierwszy. FastComments przypisuje `deployment_id` przy pierwszym uruchomieniu jako mechanizm zabezpieczający. Aby dodać nowe wdrożenie pod tym samym klientem, skontaktuj się z supportem — dodamy deployment ID do konfiguracji.

#### Uruchomienie pokazuje "Nieobsługiwany message_type"

LMS wysłał wiadomość LTI, której FastComments nie obsługuje (np. `LtiSubmissionReviewRequest`). FastComments obsługuje tylko standardowe resource-link launch oraz przepływy deep-linking. Skontaktuj się z nami, jeśli potrzebujesz dodania konkretnego typu wiadomości.

#### Iframe nie zmienia rozmiaru

Większość LMS-ów automatycznie dopasowuje rozmiar iframe'ów LTI. Jeśli Twój tego nie robi, sprawdź, czy ustawienia uruchomienia LMS pozwalają narzędziu wysyłać zdarzenia postMessage do ramki nadrzędnej. FastComments wysyła zarówno komunikaty w stylu Canvas (`lti.frameResize`), jak i zgodne ze specyfikacją IMS (`org.imsglobal.lti.frameResize`) dotyczące zmiany rozmiaru.