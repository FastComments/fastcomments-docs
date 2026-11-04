Gdy SAML jest włączone w FastComments, system automatycznie generuje informacje Service Provider (SP), które musisz skonfigurować w swoim dostawcy tożsamości.

### Dostęp do informacji Service Provider

Informacje SP są wyświetlane na stronie konfiguracji SAML po włączeniu uwierzytelniania SAML. Informacje te zawierają wszystkie szczegóły, których dostawca tożsamości potrzebuje do ustanowienia relacji zaufania SAML.

### Punkty końcowe Service Provider

#### SP Entity ID / Audience
**Cel**: Unikalnie identyfikuje Twoją instancję FastComments jako dostawcę usług
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Użycie**: Skonfiguruj to jako Entity ID lub Audience w swoim IdP

Ten identyfikator zapewnia, że odpowiedzi SAML są przeznaczone dla Twojej konkretnej instancji (tenanta) FastComments i zapobiega akceptowaniu odpowiedzi SAML przez inne instancje.

#### Assertion Consumer Service (ACS) URL
**Cel**: Punkt końcowy, do którego Twój IdP wysyła odpowiedzi SAML po uwierzytelnieniu użytkownika
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Użycie**: Skonfiguruj to jako ACS URL lub Reply URL w swoim IdP

To tutaj użytkownicy są przekierowywani po pomyślnym uwierzytelnieniu u Twojego dostawcy tożsamości, wraz z asercją SAML zawierającą informacje o użytkowniku.

#### SP Metadata URL
**Cel**: Zapewnia pełną konfigurację SAML w standardowym formacie XML
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Użycie**: Niektóre IdP mogą automatycznie importować konfigurację za pomocą tego URL

URL metadanych zawiera wszystkie niezbędne informacje SP w formacie XML, co ułatwia automatyczną konfigurację kompatybilnych dostawców tożsamości.

#### SAML Login URL
**Cel**: Bezpośredni link do zainicjowania uwierzytelniania SAML dla Twojego tenanta
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Użycie**: Linkuj użytkowników bezpośrednio do uwierzytelniania SAML lub testuj przepływ

Możesz użyć tego URL do przetestowania uwierzytelniania SAML lub udostępnić użytkownikom bezpośredni link do logowania przez SAML.

### Obsługa powiązań SAML

FastComments obsługuje następujące powiązania SAML:

#### HTTP-POST Binding
- **Metoda podstawowa**: Najczęściej używane powiązanie dla odpowiedzi SAML
- **Bezpieczeństwo**: Odpowiedź SAML jest wysyłana przez HTTP POST do ACS URL
- **Użycie**: Zalecane dla wdrożeń produkcyjnych

#### HTTP-Redirect Binding
- **Metoda alternatywna**: Odpowiedź SAML wysyłana przez przekierowanie HTTP
- **Ograniczenia**: Ograniczony rozmiar ładunku ze względu na ograniczenia długości URL
- **Użycie**: Obsługiwane, ale preferowany jest HTTP-POST

### Name ID Policy

FastComments konfiguruje następującą politykę Name ID w żądaniach SAML:

- **Format domyślny**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Formaty alternatywne**: Persistent, Transient, Unspecified (konfigurowalne)
- **Wymaganie**: Adres e-mail jest używany jako główny identyfikator użytkownika

### Atrybuty żądań SAML

Podczas inicjowania uwierzytelniania SAML FastComments wysyła żądania o następujących cechach:

#### Podpisywanie żądań
- **Status**: Opcjonalne (konfigurowalne)
- **Algorytm**: Zgodny ze skonfigurowanym algorytmem podpisu
- **Certyfikat**: Używa certyfikatu specyficznego dla tenanta, jeśli podpisywanie żądań jest włączone

#### Żądane atrybuty
FastComments żąda następujących atrybutów w żądaniach SAML AuthnRequests:

- **Email**: Wymagany do identyfikacji użytkownika
- **Imię**: Opcjonalne, do celów wyświetlania
- **Nazwisko**: Opcjonalne, do celów wyświetlania
- **Role/Grupy**: Opcjonalne, do kontroli dostępu i uprawnień

### Kopiowanie informacji SP

Strona konfiguracji SAML udostępnia pola klikalne, które automatycznie kopiują informacje SP do schowka:

1. Kliknij dowolne pole informacji SP (Entity ID, ACS URL itd.)
2. Wartość jest automatycznie kopiowana do schowka
3. Wklej wartość do konfiguracji swojego dostawcy tożsamości
4. Krótkie podświetlenie wskazuje pomyślne skopiowanie

Ułatwia to dokładne przeniesienie informacji SP do Twojego IdP bez błędów przy wpisywaniu.

### Informacje o certyfikacie SP

#### Użycie certyfikatu
- **Cel**: Szyfruje komunikację i weryfikuje tożsamość SP
- **Rotacja**: Certyfikaty są automatycznie zarządzane przez FastComments
- **Dostęp**: Certyfikaty publiczne są dostępne przez URL metadanych

#### Szczegóły certyfikatu
- **Algorytm**: RSA-2048 lub wyższy
- **Ważność**: Certyfikaty są automatycznie odnawiane przed wygaśnięciem
- **Dystrybucja**: Dostępne przez standardowe metadane SAML

### Rozwiązywanie problemów z konfiguracją SP

Jeśli Twój dostawca tożsamości zgłasza problemy z informacjami SP:

1. **Zweryfikuj URL-e**: Upewnij się, że wszystkie URL-e używają HTTPS i zawierają poprawny identyfikator tenanta
2. **Sprawdź metadane**: Użyj URL metadanych, aby zweryfikować konfigurację
3. **Przetestuj łączność**: Upewnij się, że Twój IdP może dotrzeć do punktów końcowych FastComments
4. **Zweryfikuj format**: Potwierdź, że Twój IdP obsługuje format informacji SP

Typowe problemy obejmują:
- Nieprawidłowy identyfikator tenanta w URL-ach
- Problemy z łącznością sieciową między IdP a FastComments
- IdP oczekujący innych formatów URL lub dodatkowych opcji konfiguracji