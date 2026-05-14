**Używasz Moodle?** Publikujemy także dedykowaną wtyczkę Moodle dla FastComments z głębszą integracją niż LTI 1.3 (hooki synchronizacji ocen, rozbudowane raportowanie aktywności, natywne UI ustawień Moodle). Zobacz <a href="/guide-installation-moodle.html" target="_blank">poradnik instalacji wtyczki Moodle</a>. Schemat LTI 1.3 poniżej jest właściwy, jeśli chcesz pojedynczej rejestracji obejmującej też inne LMS-y, albo jeśli administrator Moodle nie zainstaluje wtyczek firm trzecich.

Moodle 4.0+ obsługuje dynamiczną rejestrację LTI 1.3 poprzez wtyczkę Narzędzie zewnętrzne.

#### Otwórz ekran zarządzania narzędziem

1. Zaloguj się do Moodle jako administrator witryny.
2. Przejdź do **Administracja witryną** > **Wtyczki** > **Moduły aktywności** > **Narzędzie zewnętrzne** > **Zarządzaj narzędziami**.

#### Wklej adres URL

Zobaczysz kartę oznaczoną **Adres URL narzędzia**. Wklej adres rejestracyjny FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uzyskaj go tutaj</a>) do pola tekstowego i kliknij **Dodaj LTI Advantage**.

Moodle otworzy ekran rejestracji pokazujący tożsamość narzędzia i żądane uprawnienia. Sprawdź i kliknij **Aktywuj** (lub **Zarejestruj**, w zależności od wersji Moodle).

Okno dialogowe zamknie się po zakończeniu rejestracji; nowe narzędzie FastComments pojawi się na liście **Narzędzia** ze statusem **Aktywny**.

#### Udostępnij

Domyślnie Moodle dodaje nowe narzędzia do listy „Narzędzia kursu”, ale nie pokazuje ich w wyborze aktywności. Aby udostępnić FastComments w całym kursie:

1. Kliknij ikonę zębatki na kafelku FastComments.
2. W sekcji **Użycie konfiguracji narzędzia** wybierz **Pokaż w wyborze aktywności i jako wstępnie skonfigurowane narzędzie**.
3. Zapisz.

Prowadzący mogą teraz dodać FastComments do dowolnego kursu poprzez **Dodaj aktywność lub zasób** > **FastComments**.