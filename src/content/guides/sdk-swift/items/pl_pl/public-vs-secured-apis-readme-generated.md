FastComments SDK udostępnia dwa typy punktów końcowych API:

### PublicAPI - Punkty końcowe bezpieczne dla klienta

`PublicAPI` zawiera punkty końcowe, które można bezpiecznie wywoływać z kodu po stronie klienta (aplikacje iOS/macOS). Te punkty końcowe:
- Nie wymagają klucza API
- Mogą używać tokenów SSO do uwierzytelniania
- Są ograniczone limitem żądań na użytkownika/urządzenie
- Nadają się do aplikacji przeznaczonych dla użytkowników końcowych

**Przykładowy przypadek użycia**: Pobieranie i tworzenie komentarzy w aplikacji iOS

### DefaultAPI - Punkty końcowe po stronie serwera

`DefaultAPI` zawiera punkty końcowe wymagające uwierzytelnienia kluczem API. Te punkty końcowe:
- Wymagają klucza API FastComments
- Powinny być wywoływane WYŁĄCZNIE z kodu po stronie serwera
- Zapewniają pełny dostęp do danych FastComments
- Są ograniczone limitem żądań na najemcę (tenant)

**Przykładowy przypadek użycia**: Operacje administracyjne, masowy eksport danych, narzędzia moderacji

**WAŻNE**: Nigdy nie ujawniaj klucza API w kodzie po stronie klienta. Klucze API powinny być używane wyłącznie po stronie serwera.