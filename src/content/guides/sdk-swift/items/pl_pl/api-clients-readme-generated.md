The FastComments SDK zapewnia trzy klienty API:

### PublicAPI - Metody bezpieczne dla klienta

The `PublicAPI` zawiera metody, które są bezpieczne do wywoływania z kodu po stronie klienta (aplikacje iOS/macOS). Te metody:
- Nie wymagają klucza API
- Mogą używać tokenów SSO do uwierzytelniania
- Podlegają limitom szybkości żądań na użytkownika/urządzenie
- Nadają się do aplikacji skierowanych do użytkowników końcowych

**Przykładowe zastosowanie**: Pobieranie i tworzenie komentarzy w twojej aplikacji iOS

### DefaultAPI - Metody po stronie serwera

The `DefaultAPI` zawiera metody uwierzytelnione, które wymagają klucza API. Te metody:
- Wymagają twojego klucza API FastComments
- Powinny być WYŁĄCZNIE wywoływane z kodu po stronie serwera
- Zapewniają pełny dostęp do twoich danych FastComments
- Podlegają limitom żądań na dzierżawcę (tenant)

**Przykładowe zastosowanie**: Operacje administracyjne, eksport masowy danych, zarządzanie użytkownikami

### ModerationAPI - Metody panelu moderatora

The `ModerationAPI` zawiera metody, które napędzają panel moderatora. Te metody obejmują:
- **Moderacja komentarzy** - listowanie, zliczanie, wyszukiwanie, pobieranie logów i eksport komentarzy
- **Działania moderacyjne** - usuwanie/przywracanie komentarzy, zgłaszanie, ustawianie statusu do przeglądu/spam/zaakceptowania, zarządzanie głosami oraz ponowne otwieranie/zamykanie wątków
- **Bany** - zablokowanie użytkownika w komentarzu, cofanie banów, pobieranie podsumowań przed banem, sprawdzanie statusu bana i preferencji oraz odczytywanie liczby zablokowanych użytkowników
- **Odznaki i zaufanie** - przyznawanie/usuwanie odznak, listowanie odznak ręcznych, odczyt/ustawianie współczynnika zaufania użytkownika oraz odczyt wewnętrznego profilu użytkownika

Każda metoda `ModerationAPI` akceptuje parametr sso, dzięki czemu moderatorzy mogą być uwierzytelnieni za pomocą SSO.

**Przykładowe zastosowanie**: Tworzenie doświadczenia moderacji dla moderatorów twojej społeczności

**WAŻNE**: Nigdy nie ujawniaj swojego klucza API w kodzie po stronie klienta. Klucze API powinny być używane wyłącznie po stronie serwera.