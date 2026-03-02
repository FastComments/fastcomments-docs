Wtyczka obsługuje trzy tryby SSO do integracji kont użytkowników Moodle z FastComments.

#### Bezpieczne SSO (Zalecane)

Dane użytkownika są podpisywane po stronie serwera przy użyciu HMAC-SHA256 z Twoim **API Secret**. Jest to najbezpieczniejsza opcja i zalecana do użycia w środowisku produkcyjnym.

W przypadku Bezpiecznego SSO:

- Nazwy użytkowników, adresy e-mail i awatary są przesyłane bezpiecznie do FastComments.
- Administratorzy strony Moodle są automatycznie synchronizowani jako moderatorzy FastComments.
- Użytkownicy nie mogą podszywać się pod inne konta.
- Wymaga skonfigurowania **API Secret** w ustawieniach wtyczki.

#### Proste SSO

Dane użytkownika (nazwa, e-mail, awatar) są wysyłane po stronie klienta bez podpisu kryptograficznego. Jest to łatwiejsze do skonfigurowania, ponieważ nie wymaga **API Secret**, ale jest mniej bezpieczne, ponieważ dane użytkownika nie są weryfikowane po stronie serwera.

#### Brak

Brak integracji SSO. Użytkownicy komentują anonimowo lub muszą osobno zalogować się do FastComments. Użyj tego, jeśli nie chcesz, aby konta użytkowników Moodle były powiązane z komentarzami.