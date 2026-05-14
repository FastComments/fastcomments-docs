D2L Brightspace exposes Dynamic Registration through the LTI Advantage admin interface. You will need administrator access.

#### Otwórz ekran rejestracji

1. Zaloguj się do swojej instancji Brightspace jako administrator.
2. Przejdź do **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknij **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Wklej adres URL

Zobaczysz formularz rejestracji. Kluczowe pole to **Tool initiation registration endpoint** (w niektórych wersjach Brightspace pole to jest oznaczone jako "Tool Initiation Registration URL").

Wklej FastComments registration URL do tego pola. Pozostałe pola pozostaw puste - są wypełniane automatycznie przez FastComments podczas wymiany rejestracyjnej.

Kliknij **Register**.

#### Zatwierdź narzędzie

Brightspace otworzy wyskakujące okno, które porozumie się z FastComments, wymieni klucze i pokaże ekran potwierdzenia. Okno zamknie się automatycznie po zakończeniu rejestracji.

Nowe narzędzie pojawi się na liście narzędzi LTI Advantage. Domyślnie Brightspace oznacza nowe narzędzia jako **disabled** - przestaw przełącznik na **enabled**, aby kursy mogły z niego korzystać.

#### Dodaj deployment

W Brightspace narzędzia LTI potrzebują **deployment**, zanim będą mogły być używane w kursach:

1. Otwórz nowo zarejestrowane narzędzie FastComments.
2. Kliknij **View Deployments** > **New Deployment**.
3. Nadaj deploymentowi nazwę (np. "FastComments - All Courses"), wybierz jednostki organizacyjne (org units), w których ma być dostępny, i zapisz.

Po pierwszym uruchomieniu przez ten deployment, FastComments przypnie `deployment_id` do swojego rekordu konfiguracyjnego - kolejne uruchomienia z innego deploymentu dla tego samego klienta zostaną odrzucone, chyba że zarejestrujesz ponownie.