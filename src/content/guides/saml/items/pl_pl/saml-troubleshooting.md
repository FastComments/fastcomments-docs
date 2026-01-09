Ten przewodnik obejmuje typowe problemy z uwierzytelnianiem SAML i ich rozwiązania.

### Problemy z certyfikatami i bezpieczeństwem

#### Błąd nieprawidłowego certyfikatu

**Objawy**:
- błąd "Certificate validation failed"
- Użytkownicy nie mogą dokończyć uwierzytelniania SAML
- Odpowiedzi SAML są odrzucane

**Typowe przyczyny**:
- Format certyfikatu jest nieprawidłowy
- Certyfikat wygasł
- Podano niewłaściwy certyfikat
- Dodatkowe znaki lub spacje w certyfikacie

**Rozwiązania**:
1. **Zweryfikuj format certyfikatu**:
   - Upewnij się, że certyfikat zawiera markery `-----BEGIN CERTIFICATE-----` i `-----END CERTIFICATE-----`
   - Usuń dodatkowe spacje lub przerwy w linii
   - Skopiuj certyfikat bezpośrednio z metadanych IdP lub konfiguracji

2. **Sprawdź ważność certyfikatu**:
   - Zweryfikuj, że certyfikat nie wygasł
   - Potwierdź, że certyfikat jest dla właściwego IdP
   - Użyj narzędzi online do walidacji certyfikatów, aby sprawdzić format

3. **Ponownie pobierz certyfikat**:
   - Pobierz świeży certyfikat z IdP
   - Użyj URL metadanych IdP, jeśli jest dostępny
   - Potwierdź, że certyfikat odpowiada bieżącej konfiguracji IdP

#### Weryfikacja podpisu nie powiodła się

**Objawy**:
- błędy walidacji podpisu asercji SAML
- Uwierzytelnianie nie powodzi się po zalogowaniu w IdP
- komunikaty o błędzie "Invalid signature"

**Rozwiązania**:
1. **Niedopasowanie algorytmu**:
   - Sprawdź, czy algorytm podpisu w FastComments odpowiada IdP
   - Wypróbuj różne algorytmy podpisu (SHA-256, SHA-1, SHA-512)
   - Zweryfikuj, czy algorytm skrótu odpowiada konfiguracji IdP

2. **Problemy z certyfikatem**:
   - Upewnij się, że poprawny certyfikat podpisujący jest skonfigurowany
   - Zweryfikuj, że certyfikat odpowiada kluczowi prywatnemu używanemu przez IdP
   - Sprawdź, czy nie zaszła rotacja certyfikatów w IdP

### Problemy konfiguracyjne

#### Nieprawidłowe Entity ID lub ACS URL

**Objawy**:
- IdP zgłasza "Unknown Service Provider"
- Odpowiedzi SAML trafiają na niewłaściwy endpoint
- Uwierzytelnianie nie zostaje zakończone

**Rozwiązania**:
1. **Zweryfikuj informacje SP**:
   - Skopiuj dokładne Entity ID z konfiguracji FastComments
   - Upewnij się, że ACS URL ma format: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Sprawdź literówki w tenant ID

2. **Konfiguracja IdP**:
   - Zaktualizuj IdP o poprawne Entity ID SP
   - Skonfiguruj właściwy ACS/Reply URL
   - Zweryfikuj ustawienia wiązania IdP (preferowany HTTP-POST)

#### Brakujące lub nieprawidłowe atrybuty

**Objawy**:
- Użytkownicy tworzeni bez odpowiednich ról
- Brak informacji w profilu użytkownika
- błędy "Email required"

**Rozwiązania**:
1. **Atrybut e-mail**:
   - Upewnij się, że IdP wysyła atrybut e-mail
   - Sprawdź mapowanie nazwy atrybutu (email, emailAddress itp.)
   - Zweryfikuj, że wartość e-mail jest prawidłowym adresem e-mail

2. **Atrybuty ról**:
   - Potwierdź, że IdP wysyła informacje o rolach/grupach
   - Sprawdź, czy nazwy atrybutów ról odpowiadają oczekiwaniom FastComments
   - Zweryfikuj, że wartości ról dokładnie odpowiadają nazwom ról w FastComments

3. **Format atrybutów**:
   - Przetestuj zarówno format tablicowy, jak i wartości rozdzielone przecinkami dla ról
   - Upewnij się, że wartości atrybutów nie mają dodatkowych spacji
   - Sprawdź wrażliwość na wielkość liter w nazwach ról

### Problemy z przebiegiem uwierzytelniania

#### Pętla przekierowań

**Objawy**:
- Przeglądarka nieustannie przekierowuje między FastComments a IdP
- Uwierzytelnianie nigdy się nie kończy
- W narzędziach deweloperskich przeglądarki widać wiele przekierowań

**Rozwiązania**:
1. **Sprawdź konfigurację SP**:
   - Zweryfikuj, że Entity ID dokładnie pasuje do konfiguracji IdP
   - Upewnij się, że ACS URL jest poprawnie skonfigurowany w IdP
   - Sprawdź, czy w URL nie ma zakończeń ukośnikiem

2. **Problemy z sesją**:
   - Wyczyść ciasteczka przeglądarki i spróbuj ponownie
   - Przetestuj w oknie incognito/prywatnym
   - Sprawdź ustawienia limitu czasu sesji

#### Dostęp odrzucony po uwierzytelnieniu

**Objawy**:
- Uwierzytelnianie SAML się powiodło
- Użytkownik zostaje przekierowany do FastComments
- Wyświetlany jest komunikat "Access denied" lub błąd uprawnień

**Rozwiązania**:
1. **Przypisanie ról**:
   - Zweryfikuj, że użytkownik ma odpowiednie role w IdP
   - Sprawdź, czy atrybut roli jest wysyłany w odpowiedzi SAML
   - Potwierdź, że nazwy ról dokładnie odpowiadają wymaganiom FastComments

2. **Ograniczenia pakietu**:
   - Zweryfikuj, że konto ma plan Flex lub Pro
   - Sprawdź, czy funkcja SAML jest włączona dla pakietu
   - Skontaktuj się z supportem, jeśli pakiet obejmuje SAML, ale funkcja jest niedostępna

### Problemy specyficzne dla dostawcy tożsamości

#### Microsoft Azure AD

**Typowe problemy**:
- Przypisania ról aplikacji nie pojawiają się w tokenach
- Roszczenia (claims) nie są wysyłane poprawnie
- Wymagania dotyczące przypisania użytkownika

**Rozwiązania**:
- Sprawdź przypisanie użytkownika do aplikacji FastComments
- Zweryfikuj, że role aplikacji są poprawnie skonfigurowane
- Upewnij się, że mapowanie roszczeń zawiera wymagane atrybuty

#### Okta

**Typowe problemy**:
- Filtry grup nie działają poprawnie
- Deklaracje atrybutów są źle skonfigurowane
- Problemy z przypisaniem aplikacji

**Rozwiązania**:
- Przejrzyj konfigurację deklaracji atrybutów
- Sprawdź przypisanie grup i reguły filtrowania
- Zweryfikuj, że aplikacja jest przypisana do odpowiednich użytkowników/grup

#### Google Workspace

**Typowe problemy**:
- Niestandardowe atrybuty nie mapują się poprawnie
- Członkostwo w grupach nie jest wysyłane
- Błędy konfiguracji aplikacji SAML

**Rozwiązania**:
- Skonfiguruj niestandardowe schematy dla atrybutów ról
- Sprawdź propagację członkostwa w grupach
- Zweryfikuj mapowanie atrybutów aplikacji SAML

### Problemy sieciowe i łączności

#### Błędy przekroczenia czasu (timeout)

**Objawy**:
- Proces uwierzytelniania kończy się przekroczeniem czasu
- błędy "Request timeout" lub podobne
- Powolny przebieg uwierzytelniania

**Rozwiązania**:
1. **Łączność sieciowa**:
   - Sprawdź reguły zapory, aby umożliwić komunikację z FastComments
   - Zweryfikuj rozwiązywanie DNS dla fastcomments.com
   - Przetestuj łączność sieciową z IdP do FastComments

2. **Problemy z wydajnością**:
   - Sprawdź czasy odpowiedzi IdP
   - Zweryfikuj, czy walidacja łańcucha certyfikatów nie jest powolna
   - Rozważ opóźnienia sieciowe między IdP a użytkownikami

#### Problemy SSL/TLS

**Objawy**:
- Ostrzeżenia dotyczące certyfikatów podczas uwierzytelniania
- Niepowodzenia podczas uzgadniania SSL handshake
- błędy "Secure connection failed"

**Rozwiązania**:
- Upewnij się, że wszystkie endpointy SAML używają HTTPS
- Sprawdź ważność certyfikatów dla wszystkich zaangażowanych domen
- Zweryfikuj kompatybilność wersji TLS

### Debugowanie i logowanie

#### Włączanie informacji debugowania

1. **Narzędzia deweloperskie przeglądarki**:
   - Monitoruj kartę Network podczas przebiegu SAML
   - Sprawdź Console pod kątem błędów JavaScript
   - Zbadaj żądania POST SAML (jeśli widoczne)

2. **Logowanie IdP**:
   - Włącz debugowanie SAML w swoim IdP
   - Przejrzyj logi IdP pod kątem szczegółów żądań/odpowiedzi SAML
   - Sprawdź problemy z mapowaniem atrybutów

#### Typowe komunikaty w logach

**Logi FastComments**:
- "SAML config not found" - SAML nie jest włączony lub jest źle skonfigurowany
- "Invalid certificate" - Walidacja certyfikatu nie powiodła się
- "Missing email attribute" - Wymagany atrybut e-mail nie został dostarczony w odpowiedzi SAML

**Logi IdP**:
- "Unknown service provider" - Niedopasowanie Entity ID
- "Invalid ACS URL" - Nieprawidłowy Assertion Consumer Service URL
- "User not assigned" - Użytkownik nie ma przypisanego dostępu do aplikacji SAML

### Uzyskiwanie pomocy

#### Informacje do zebrania

Kontaktując się z supportem, dostarcz:
- Dokładne komunikaty o błędach i znaczniki czasu
- Szczegóły konfiguracji SAML (bez danych wrażliwych)
- Typ i wersję IdP
- Kroki do odtworzenia problemu
- Informacje o przeglądarce i sieci

#### Support FastComments

W kwestiach związanych z SAML:
1. Użyj [support portal](https://fastcomments.com/auth/my-account/help)
2. Dołącz tenant ID i e-maile dotkniętych użytkowników
3. Podaj komunikaty o błędach i szczegóły konfiguracji
4. Określ typ IdP i podejście konfiguracyjne

#### Support IdP

W przypadku problemów specyficznych dla IdP:
- Zapoznaj się z dokumentacją IdP dotyczącą rozwiązywania problemów z SAML
- Skorzystaj z kanałów wsparcia IdP w przypadku problemów konfiguracyjnych
- Wykorzystaj fora społeczności IdP dla typowych problemów

### Wskazówki zapobiegawcze

#### Najlepsze praktyki

1. **Dokładne testy**:
   - Testuj zmiany konfiguracji w środowisku nieprodukcyjnym
   - Zweryfikuj przy użyciu wielu użytkowników testowych
   - Dokumentuj działające konfiguracje

2. **Regularny monitoring**:
   - Skonfiguruj monitorowanie niepowodzeń uwierzytelniania SAML
   - Przeglądaj daty wygaśnięcia certyfikatów
   - Monitoruj zmiany konfiguracji IdP

3. **Dokumentacja**:
   - Utrzymuj dokumentację konfiguracji SAML
   - Dokumentuj wszelkie niestandardowe konfiguracje lub obejścia
   - Przechowuj dane kontaktowe administratorów IdP

#### Proaktywne utrzymanie

1. **Zarządzanie certyfikatami**:
   - Monitoruj daty wygaśnięcia certyfikatów
   - Planuj procedury rotacji certyfikatów
   - Testuj aktualizacje certyfikatów przed wygaśnięciem

2. **Przeglądy konfiguracji**:
   - Regularnie przeglądaj konfigurację SAML
   - Zweryfikuj, że konfiguracja IdP pozostaje aktualna
   - Aktualizuj dokumentację w miarę wprowadzania zmian