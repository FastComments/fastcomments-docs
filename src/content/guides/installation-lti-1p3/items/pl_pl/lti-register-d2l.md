D2L Brightspace udostępnia Dynamic Registration za pośrednictwem interfejsu administracyjnego LTI Advantage. Wymagany jest dostęp administratora.

#### Otwórz ekran rejestracji

1. Zaloguj się do swojej instancji Brightspace jako administrator.
2. Przejdź do **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknij **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Wklej URL

Zobaczysz formularz rejestracji. Kluczowe pole to **punkt końcowy rejestracji inicjacji narzędzia** (w niektórych wersjach Brightspace oznaczane jako "URL rejestracji inicjacji narzędzia").

Wklej adres rejestracyjny FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pobierz go tutaj</a>) do tego pola. Pozostaw pozostałe pola puste - są automatycznie wypełniane przez FastComments podczas wymiany rejestracyjnej.

Kliknij **Register**.

#### Zatwierdź narzędzie

Brightspace otworzy wyskakujące okno, które komunikuje się z FastComments, wymieni klucze i pokaże ekran potwierdzenia. Wyskakujące okno zamknie się automatycznie po zakończeniu rejestracji.

Nowe narzędzie pojawi się na liście narzędzi LTI Advantage. Domyślnie Brightspace oznacza nowe narzędzia jako **wyłączone** - przestaw przełącznik na **włączone**, aby Twoje kursy mogły z niego korzystać.

#### Dodaj wdrożenie

W Brightspace narzędzia LTI wymagają **wdrożenia**, zanim będą mogły być używane w kursach:

1. Otwórz nowo zarejestrowane narzędzie FastComments.
2. Kliknij **View Deployments** > **New Deployment**.
3. Nadaj wdrożeniu nazwę (np. "FastComments - All Courses"), wybierz jednostki organizacyjne, w których ma być dostępne, i zapisz.

Po pierwszym uruchomieniu poprzez to wdrożenie FastComments przypisze `deployment_id` do swojego rekordu konfiguracyjnego - kolejne uruchomienia z innego wdrożenia dla tego samego klienta zostaną odrzucone, chyba że ponownie zarejestrujesz.