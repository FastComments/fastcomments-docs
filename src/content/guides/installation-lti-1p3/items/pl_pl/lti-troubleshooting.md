#### „Nie znaleziono tokenu rejestracji, token wygasł lub został już użyty"

Token w adresie URL rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pobierz go tutaj</a>) jest ważny przez 30 minut i może zostać użyty tylko raz. Jeśli Twój LMS potrzebował więcej czasu lub rejestracja została ponowiona po jej pomyślnym zakończeniu, token zostanie odrzucony. Wygeneruj świeży adres URL na stronie konfiguracji FastComments LTI 1.3 i zacznij od nowa.

#### „Platforma odrzuciła rejestrację"

Twój LMS odmówił przeprowadzenia uzgodnienia rejestracji. Najczęstsze przyczyny:

- **Narzędzie jest już zarejestrowane pod tą samą nazwą klienta.** Niektóre platformy (w szczególności D2L) odrzucają drugą rejestrację „FastComments", dopóki poprzednia nie zostanie usunięta. Usuń stare narzędzie w swoim LMS, a następnie spróbuj ponownie.
- **Błędne pole w LMS.** Upewnij się, że wkleiłeś adres URL w polu **registration / tool initiation registration endpoint**, a nie w polu launch URL lub login URL.
- **LMS faktycznie nie obsługuje Dynamic Registration.** Starsze wersje Moodle i Blackboard deklarują obsługę LTI 1.3, ale pozwalają tylko na ręczną konfigurację. Sprawdź dokumentację swojej platformy.

#### „Nie udało się pobrać konfiguracji platformy"

FastComments nie mógł odczytać dokumentu openid-configuration Twojego LMS. To rzadka sytuacja i zwykle oznacza, że LMS podał nieprawidłowo sformowany lub niedostępny adres URL discovery. Skontaktuj się z pomocą techniczną swojego LMS.

#### Uruchomienie pokazuje „Configuration not found"

Konfiguracja w FastComments została usunięta albo uruchomienie pochodziło z pary `iss`/`client_id`, której nie rozpoznajemy. Jeśli usunąłeś konfigurację i zarejestrowałeś ją ponownie, zaleć swojemu LMS usunięcie i ponowne dodanie narzędzia FastComments, aby otrzymało nowy client_id.

#### Uruchomienie pokazuje „Deployment not registered"

Uruchomiłeś FastComments z wdrożenia Brightspace/Moodle/Blackboard innego niż to, w którym nastąpiło pierwsze uruchomienie. FastComments przypina `deployment_id` przy pierwszym uruchomieniu jako weryfikację bezpieczeństwa. Aby dodać nowe wdrożenie pod tym samym klientem, skontaktuj się z pomocą techniczną — dodamy identyfikator wdrożenia do konfiguracji.

#### Uruchomienie pokazuje „Unsupported message_type"

LMS wysłał wiadomość LTI, której FastComments nie obsługuje (np. `LtiSubmissionReviewRequest`). FastComments obsługuje wyłącznie standardowe uruchomienie resource-link oraz przepływy deep-linking. Skontaktuj się z nami, jeśli potrzebujesz dodania konkretnego typu wiadomości.

#### Iframe nie zmienia rozmiaru

Większość LMS-ów automatycznie dopasowuje rozmiar ramek iframe LTI. Jeśli Twój tego nie robi, sprawdź, czy ustawienia uruchamiania LMS pozwalają narzędziu wysyłać zdarzenia postMessage do ramki nadrzędnej. FastComments wysyła zarówno wiadomości o zmianie rozmiaru w stylu Canvas (`lti.frameResize`), jak i zgodne ze specyfikacją IMS (`org.imsglobal.lti.frameResize`).