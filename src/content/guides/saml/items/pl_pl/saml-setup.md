Skonfigurowanie uwierzytelniania SAML w FastComments wymaga zarówno konfiguracji w panelu administracyjnym, jak i ustawień w dostawcy tożsamości.

### Wymagania wstępne

Przed skonfigurowaniem SAML upewnij się, że posiadasz:

- Plan FastComments Flex lub Pro (SAML nie jest dostępny w planie Creators)
- Dostęp administracyjny do konta FastComments
- Dostęp administracyjny do dostawcy tożsamości
- Metadane SAML lub informacje o certyfikacie Twojego IdP

### Uzyskiwanie dostępu do konfiguracji SAML

1. Zaloguj się do swojego [panelu administracyjnego FastComments](https://fastcomments.com/auth/my-account)
2. Przejdź do **Ustawień API/SSO** w lewym pasku bocznym
3. Kliknij przycisk **Konfiguracja SAML**

Jeśli nie widzisz przycisku Konfiguracja SAML, sprawdź, czy:
- Twoje konto ma wymagany pakiet (Flex lub Pro)
- Masz uprawnienia administracyjne
- Twoje konto użytkownika ma role API Admin lub Admin Admin

### Podstawowa konfiguracja SAML

#### Włączenie uwierzytelniania SAML

1. Zaznacz pole wyboru **Włącz uwierzytelnianie SAML**
2. To aktywuje SAML dla Twojego tenanta i udostępni pola konfiguracji

#### Wymagane pola

**IdP Single Sign-On URL** *(Wymagane)*
- URL, pod który użytkownicy zostaną przekierowani w celu uwierzytelnienia
- Zazwyczaj dostarczany przez Twojego dostawcę tożsamości
- Przykład: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Wymagane)*
- Certyfikat publiczny od Twojego dostawcy tożsamości
- Używany do weryfikacji autentyczności odpowiedzi SAML
- Musi zawierać pełny certyfikat z znacznikami BEGIN/END
- Przykładowy format:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Opcjonalne pola

**IdP Entity ID / Issuer**
- Identyfikuje Twojego dostawcę tożsamości
- Jeśli pozostawione puste, domyślnie używany jest adres URL FastComments
- Powinien odpowiadać issuerowi skonfigurowanemu w Twoim IdP

### Zaawansowana konfiguracja

#### Ustawienia zabezpieczeń

**Signature Algorithm**
- Domyślnie SHA-256 (zalecane)
- Opcje: SHA-1, SHA-256, SHA-512
- Powinno odpowiadać konfiguracji Twojego IdP

**Digest Algorithm**
- Domyślnie SHA-256 (zalecane)
- Używany do obliczania skrótu w odpowiedziach SAML
- Powinno odpowiadać konfiguracji Twojego IdP

**Name ID Format**
- Domyślnie format adresu e-mail
- Określa, w jaki sposób identyfikatory użytkowników są formatowane
- Typowe opcje: Email Address, Persistent, Transient

#### Szyfrowanie (opcjonalnie)

**Private Key for Decryption**
- Potrzebne tylko, jeśli Twój IdP szyfruje asercje SAML
- Wklej swój klucz prywatny używany do odszyfrowania
- W większości wdrożeń szyfrowanie asercji nie jest wymagane

### Zapisywanie konfiguracji

1. Sprawdź wszystkie ustawienia pod kątem poprawności
2. Kliknij **Zapisz konfigurację SAML**
3. System zweryfikuje Twoją konfigurację
4. Jeśli wszystko się powiedzie, zobaczysz komunikat potwierdzający

### Kolejne kroki

Po zapisaniu konfiguracji SAML w FastComments:

1. Skonfiguruj swojego dostawcę tożsamości, używając informacji dotyczących Service Provider
2. Przetestuj przebieg uwierzytelniania
3. Skonfiguruj role użytkowników i uprawnienia w razie potrzeby

Informacje o Service Provider potrzebne do konfiguracji IdP będą wyświetlane po włączeniu SAML.