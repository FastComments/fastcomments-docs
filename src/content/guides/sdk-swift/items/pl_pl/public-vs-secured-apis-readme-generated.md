---
FastComments SDK zapewnia dwa rodzaje punktów końcowych API:

### PublicAPI - Punkty końcowe bezpieczne dla klienta

The `PublicAPI` contains endpoints that are safe to call from client-side code (aplikacje iOS/macOS). Te punkty końcowe:
- Nie wymagają API key
- Mogą używać SSO tokens do uwierzytelniania
- Podlegają ograniczeniom na użytkownika/urządzenie
- Nadają się do aplikacji skierowanych do użytkowników końcowych

**Przykładowy przypadek użycia**: Pobieranie i tworzenie komentarzy w twojej aplikacji iOS

### DefaultAPI - Punkty końcowe po stronie serwera

The `DefaultAPI` contains authenticated endpoints that require an API key. Te punkty końcowe:
- Wymagają twojego FastComments API key
- Należy WYŁĄCZNIE wywoływać je z kodu po stronie serwera
- Zapewniają pełny dostęp do twoich danych FastComments
- Podlegają ograniczeniom na tenant

**Przykładowy przypadek użycia**: operacje administracyjne, eksport masowych danych, narzędzia do moderacji

**WAŻNE**: Nigdy nie ujawniaj swojego API key w kodzie po stronie klienta. API keys powinny być używane wyłącznie po stronie serwera.
---