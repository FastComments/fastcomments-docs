Gdy SAML jest włączony w FastComments, system automatycznie generuje informacje o Dostawcy Usługi (SP), które musisz skonfigurować w swoim dostawcy tożsamości.

### Dostęp do informacji o Dostawcy Usługi

Informacje o SP są wyświetlane na stronie konfiguracji SAML po włączeniu uwierzytelniania SAML. Informacje te zawierają wszystkie szczegóły, których potrzebuje twój dostawca tożsamości, aby ustanowić relację zaufania SAML.

### Punkty końcowe Dostawcy Usługi

#### Identyfikator encji SP / Odbiorca
**Cel**: Jednoznaczne identyfikowanie twojej instancji FastComments jako dostawcy usługi  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Zastosowanie**: Skonfiguruj to jako Entity ID lub Audience w swoim IdP

Ten identyfikator zapewnia, że odpowiedzi SAML są przeznaczone dla konkretnego najemcy FastComments i zapobiega akceptowaniu odpowiedzi SAML przez inne instancje.

#### Assertion Consumer Service (ACS) URL
**Cel**: Punkt końcowy, na który twój IdP wysyła odpowiedzi SAML po uwierzytelnieniu użytkownika  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Zastosowanie**: Skonfiguruj to jako ACS URL lub Reply URL w swoim IdP

To miejsce, do którego użytkownicy są przekierowywani po pomyślnym uwierzytelnieniu przez dostawcę tożsamości, wraz z asercją SAML zawierającą informacje o użytkowniku.

#### URL metadanych SP
**Cel**: Dostarcza pełną konfigurację SAML w standardowym formacie XML  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Zastosowanie**: Niektóre IdP mogą automatycznie importować konfigurację przy użyciu tego URL

URL metadanych zawiera wszystkie niezbędne informacje o SP w formacie XML, co ułatwia automatyczną konfigurację kompatybilnych dostawców tożsamości.

#### URL logowania SAML
**Cel**: Bezpośredni link do inicjowania uwierzytelniania SAML dla twojego najemcy  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Zastosowanie**: Udostępnij użytkownikom bezpośredni link do uwierzytelniania SAML lub przetestuj przepływ

Możesz użyć tego URL, aby przetestować uwierzytelnianie SAML lub zapewnić użytkownikom bezpośredni link do logowania przez SAML.

### Obsługa powiązań (SAML Binding)

FastComments obsługuje następujące powiązania SAML:

#### HTTP-POST Binding
- **Główna metoda**: Najczęściej stosowane powiązanie dla odpowiedzi SAML  
- **Bezpieczeństwo**: Odpowiedź SAML jest przesyłana za pomocą HTTP POST do ACS URL  
- **Zastosowanie**: Zalecane dla wdrożeń produkcyjnych

#### HTTP-Redirect Binding
- **Metoda alternatywna**: Odpowiedź SAML wysyłana za pomocą przekierowania HTTP  
- **Ograniczenia**: Ograniczony rozmiar ładunku z powodu ograniczeń długości URL  
- **Zastosowanie**: Obsługiwane, ale preferowany jest HTTP-POST

### Polityka Name ID

FastComments konfiguruje następującą politykę Name ID w żądaniach SAML:

- **Domyślny format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternatywne formaty**: Persistent, Transient, Unspecified (konfigurowalne)  
- **Wymóg**: Adres e-mail jest używany jako podstawowy identyfikator użytkownika

### Atrybuty żądania SAML

Podczas inicjowania uwierzytelniania SAML, FastComments wysyła żądania o następujących cechach:

#### Podpisywanie żądań
- **Status**: Opcjonalne (konfigurowalne)  
- **Algorytm**: Zgodny z skonfigurowanym algorytmem podpisu  
- **Certyfikat**: Używa certyfikatu specyficznego dla najemcy, jeśli podpisywanie żądań jest włączone

#### Żądane atrybuty
FastComments żąda następujących atrybutów w AuthnRequests SAML:

- **Email**: Wymagane do identyfikacji użytkownika  
- **First Name**: Opcjonalne do celów wyświetlania  
- **Last Name**: Opcjonalne do celów wyświetlania  
- **Roles/Groups**: Opcjonalne do kontroli dostępu i uprawnień

### Kopiowanie informacji SP

Strona konfiguracji SAML udostępnia pola, które można kliknąć, aby automatycznie skopiować informacje o SP do schowka:

1. Kliknij dowolne pole z informacją o SP (Entity ID, ACS URL itp.)  
2. Wartość zostanie automatycznie skopiowana do schowka  
3. Wklej wartość w konfiguracji dostawcy tożsamości  
4. Krótkie podświetlenie wskazuje pomyślne skopiowanie

Ułatwia to dokładne przeniesienie informacji SP do twojego IdP bez błędów w pisaniu.

### Informacje o certyfikacie SP

#### Zastosowanie certyfikatu
- **Cel**: Szyfruje komunikację i potwierdza tożsamość SP  
- **Rotacja**: Certyfikaty są zarządzane automatycznie przez FastComments  
- **Dostęp**: Certyfikaty publiczne są dostępne przez URL metadanych

#### Szczegóły certyfikatu
- **Algorytm**: RSA-2048 lub wyższy  
- **Ważność**: Certyfikaty są automatycznie odnawiane przed wygaśnięciem  
- **Dystrybucja**: Dostępne poprzez standardowe metadane SAML

### Rozwiązywanie problemów z konfiguracją SP

Jeśli twój dostawca tożsamości zgłasza problemy z informacjami SP:

1. **Zweryfikuj URL-e**: Upewnij się, że wszystkie URL-e używają HTTPS i zawierają prawidłowe ID najemcy  
2. **Sprawdź metadane**: Użyj URL metadanych, aby zweryfikować konfigurację  
3. **Przetestuj łączność**: Upewnij się, że twój IdP może osiągnąć punkty końcowe FastComments  
4. **Zwaliduj format**: Potwierdź, że twój IdP obsługuje format informacji SP

Typowe problemy obejmują:
- Nieprawidłowe ID najemcy w URL-ach  
- Problemy z łącznością sieciową między IdP a FastComments  
- IdP oczekujący innych formatów URL lub dodatkowych opcji konfiguracyjnych