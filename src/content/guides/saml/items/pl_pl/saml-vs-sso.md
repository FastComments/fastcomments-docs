FastComments oferuje zarówno uwierzytelnianie SSO, jak i SAML. Zrozumienie różnic pomoże Ci wybrać odpowiednie podejście dla Twojej organizacji.

### Simple/Secure SSO w środowisku produkcyjnym

FastComments oferuje dwa różne przepływy SSO do uwierzytelniania w widgetcie komentarzy za pośrednictwem Twojej strony.
To różni się od SAML i nie wymaga SAML. Zamiast tego Simple SSO wymaga po prostu przekazania obiektu do
widgetu komentarzy, podczas gdy Secure SSO robi to samo plus hashuje ładunek przy użyciu API key.

SAML, z drugiej strony, uwierzytelnia użytkownika do całego produktu (na podstawie ich uprawnień) *as well as* widgetu komentarzy
(jeśli mają włączone pliki cookie stron trzecich dla naszej domeny).

### SAML Authentication

SAML to korporacyjny protokół uwierzytelniania, który zapewnia bardziej solidne możliwości bezpieczeństwa i integracji:

- **Implementation**: Wymaga konfiguracji Identity Provider (IdP) i wymiany certyfikatów
- **Security**: Używa podpisanych asercji XML i obsługuje szyfrowanie
- **Use Case**: Idealne dla przedsiębiorstw z istniejącą infrastrukturą SAML (Active Directory, Okta, itp.)
- **Setup Complexity**: Bardziej złożone - wymaga konfiguracji IdP i zarządzania certyfikatami
- **Enterprise Features**: Zaawansowane mapowanie ról, scentralizowane zarządzanie użytkownikami, ścieżki audytu

### When to Choose SAML

Rozważ uwierzytelnianie SAML, jeśli Twoja organizacja:

- Już korzysta z kompatybilnego z SAML dostawcy tożsamości (Okta, Azure AD, ADFS, itp.)
- Wymaga korporacyjnego poziomu bezpieczeństwa i zgodności
- Potrzebuje scentralizowanego zarządzania użytkownikami i kontroli dostępu
- Ma wiele aplikacji korzystających z SAML do uwierzytelniania
- Wymaga szczegółowych ścieżek audytu i raportowania bezpieczeństwa

### When to Choose Simple or Secure SSO

Nasze rozwiązania SSO ukierunkowane na widget mogą być wystarczające, jeśli:

- Masz własny system uwierzytelniania
- Potrzebujesz szybkiej implementacji przy minimalnej konfiguracji
- Nie wymagasz integracji z korporacyjnym dostawcą tożsamości
- Chcesz kontrolować dane użytkownika bezpośrednio z aplikacji
- Masz prostsze wymagania dotyczące bezpieczeństwa

Simple i Secure SSO są powszechnie używane dla portali internetowych, blogów, itp., gdzie użytkownik już posiada konto *through your site or app*
ale niekoniecznie korzysta z SAML.