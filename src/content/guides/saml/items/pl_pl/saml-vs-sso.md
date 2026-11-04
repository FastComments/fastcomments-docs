FastComments oferuje zarówno uwierzytelnianie SSO, jak i SAML. Zrozumienie różnic pomoże Ci wybrać odpowiednie podejście dla Twojej organizacji.

### Prosty/Bezpieczny SSO w produkcji

FastComments oferuje dwa różne przepływy SSO do uwierzytelniania w widżecie komentarzy za pośrednictwem Twojej witryny.
To różni się od SAML i nie wymaga SAML. Zamiast tego Prosty SSO wymaga jedynie przekazania obiektu do
widżetu komentarzy, natomiast Bezpieczny SSO robi to plus hashowanie ładunku kluczem API.

SAML z kolei uwierzytelnia użytkownika w całym produkcie (na podstawie jego uprawnień) *a także* w widżecie komentarzy
(jeśli ma włączone pliki cookie stron trzecich dla naszej domeny).

### Uwierzytelnianie SAML

SAML to korporacyjny protokół uwierzytelniania, który zapewnia bardziej solidne bezpieczeństwo i możliwości integracji:

- **Implementacja**: Wymaga konfiguracji dostawcy tożsamości (IdP) i wymiany certyfikatów
- **Bezpieczeństwo**: Używa podpisanych asercji XML i obsługuje szyfrowanie
- **Przypadek użycia**: Idealny dla przedsiębiorstw z istniejącą infrastrukturą SAML (Active Directory, Okta itp.)
- **Złożoność konfiguracji**: Bardziej wymagający - wymaga konfiguracji IdP i zarządzania certyfikatami
- **Funkcje korporacyjne**: Zaawansowane mapowanie ról, scentralizowane zarządzanie użytkownikami, ślady audytu

### Kiedy wybrać SAML

Rozważ uwierzytelnianie SAML, jeśli Twoja organizacja:

- Już korzysta z dostawcy tożsamości zgodnego z SAML (Okta, Azure AD, ADFS itp.)
- Wymaga korporacyjnego poziomu bezpieczeństwa i zgodności
- Potrzebuje scentralizowanego zarządzania użytkownikami i kontroli dostępu
- Ma wiele aplikacji używających SAML do uwierzytelniania
- Wymaga szczegółowych śladów audytu i raportowania bezpieczeństwa

### Kiedy wybrać Prosty lub Bezpieczny SSO

Nasze rozwiązania SSO skupione na widżecie mogą być wystarczające, jeśli:

- Masz własny system uwierzytelniania
- Potrzebujesz szybkiej implementacji przy minimalnej konfiguracji
- Nie wymagasz integracji z korporacyjnym dostawcą tożsamości
- Chcesz kontrolować dane użytkowników bezpośrednio ze swojej aplikacji
- Masz prostsze wymagania dotyczące bezpieczeństwa

Prosty i Bezpieczny SSO są powszechnie używane w portalach internetowych, blogach itp., gdzie użytkownik ma już konto *za pośrednictwem Twojej witryny lub aplikacji*
ale niekoniecznie korzysta z SAML.