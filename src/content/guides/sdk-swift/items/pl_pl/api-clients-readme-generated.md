SDK FastComments udostępnia trzech klientów API:

### PublicAPI - Metody bezpieczne dla klienta

`PublicAPI` zawiera metody, które można bezpiecznie wywoływać z kodu po stronie klienta (aplikacje iOS/macOS). Te metody:
- Nie wymagają klucza API
- Mogą używać tokenów SSO do uwierzytelniania
- Są ograniczone limitem żądań na użytkownika/urządzenie
- Nadają się do aplikacji przeznaczonych dla użytkowników końcowych

**Przykładowy przypadek użycia**: Pobieranie i tworzenie komentarzy w aplikacji iOS

### DefaultAPI - Metody po stronie serwera

`DefaultAPI` zawiera metody uwierzytelniane, które wymagają klucza API. Te metody:
- Wymagają klucza API FastComments
- Powinny być wywoływane WYŁĄCZNIE z kodu po stronie serwera
- Zapewniają pełny dostęp do danych FastComments
- Są ograniczone limitem żądań na najemcę (tenant)

**Przykładowy przypadek użycia**: Operacje administracyjne, masowy eksport danych, zarządzanie użytkownikami

### ModerationAPI - Metody panelu moderatora

`ModerationAPI` oferuje rozbudowany zestaw API do moderacji na żywo i szybkiej moderacji. Każda metoda `ModerationAPI` przyjmuje parametr `sso` i może uwierzytelniać się za pomocą SSO lub ciasteczka sesji FastComments.com.

**Przykładowy przypadek użycia**: Budowanie doświadczenia moderacji dla moderatorów Twojej społeczności

**WAŻNE**: Nigdy nie ujawniaj klucza API w kodzie po stronie klienta. Klucze API powinny być używane tylko po stronie serwera.