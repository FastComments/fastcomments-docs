FastComments mapuje role użytkowników SAML na wewnętrzne uprawnienia, umożliwiając kontrolę dostępu opartą na rolach w Twojej organizacji.

### System ról FastComments

FastComments korzysta z systemu uprawnień opartego na rolach, w którym użytkownicy mogą mieć jedną lub więcej ról, które określają ich poziomy dostępu i możliwości.

### Dostępne role FastComments

#### Role administracyjne

**fc-account-owner**
- **Uprawnienia**: Pełny dostęp administracyjny
- **Możliwości**: Wszystkie funkcje, zarządzanie rozliczeniami, zarządzanie użytkownikami
- **Przypadek użycia**: Główni administratorzy i właściciele kont

**fc-admin-admin**  
- **Uprawnienia**: Dostęp administracyjny do większości funkcji
- **Możliwości**: Zarządzanie użytkownikami, konfiguracja, moderacja. **Może administrować innymi administratorami.**
- **Przypadek użycia**: Administratorzy drugiego szczebla i personel IT

**fc-billing-admin**
- **Uprawnienia**: Zarządzanie rozliczeniami i subskrypcjami
- **Możliwości**: Metody płatności, faktury, zmiany subskrypcji
- **Przypadek użycia**: Członkowie zespołu finansowego i osoby kontaktowe ds. rozliczeń

#### Role specjalistyczne

**fc-analytics-admin**
- **Uprawnienia**: Dostęp do analityki i raportowania
- **Możliwości**: Przeglądanie statystyk witryny, danych o zaangażowaniu użytkowników
- **Przypadek użycia**: Zespoły marketingowe i analitycy danych

**fc-api-admin**
- **Uprawnienia**: Dostęp do API i jego zarządzanie
- **Możliwości**: Poświadczenia API, konfiguracja webhooków
- **Przypadek użycia**: Programiści i integratorzy techniczni

**fc-moderator**
- **Uprawnienia**: Możliwości moderacji komentarzy
- **Możliwości**: Zatwierdzanie/odrzucanie komentarzy, zarządzanie spamem
- **Przypadek użycia**: Moderatorzy społeczności i menedżerowie treści

### Konfiguracja mapowania ról

#### Źródła atrybutów SAML

FastComments akceptuje informacje o rolach z różnych nazw atrybutów SAML, aby zapewnić zgodność z różnymi dostawcami tożsamości:

**Standardowe nazwy atrybutów**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Atrybuty Microsoft/ADFS**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Obsługiwane formaty ról

**Format tablicowy** *(Preferowany)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format rozdzielany przecinkami**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format pojedynczej roli**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Konfiguracja ról u dostawcy tożsamości

#### Microsoft Azure AD

1. **Konfiguracja ról aplikacji**:
   - Zdefiniuj role FastComments w swojej aplikacji Azure AD
   - Przypisz użytkowników do odpowiednich ról aplikacji
   - Skonfiguruj claims tak, aby zawierały przypisane role

2. **Mapowanie atrybutów**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Przypisanie grup**:
   - Utwórz grupy odpowiadające nazwom ról FastComments
   - Przypisz użytkowników do odpowiednich grup
   - Skonfiguruj instrukcje atrybutów

2. **Instrukcja atrybutu**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mapowanie grup**:
   - Utwórz jednostki organizacyjne lub grupy
   - Nadaj grupom nazwy z prefiksami ról FastComments
   - Skonfiguruj mapowanie atrybutów

2. **Atrybuty niestandardowe**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Domyślne zachowanie użytkowników

#### Użytkownicy bez ról

Gdy użytkownik SAML nie ma ról ani ma role nierozpoznane:
- Użytkownik jest tworzony jako standardowy komentujący
- Nie otrzymuje dostępu administracyjnego
- Może publikować i zarządzać własnymi komentarzami
- Nie ma dostępu do funkcji panelu administracyjnego

#### Dziedziczenie ról

- Użytkownicy mogą mieć wiele ról jednocześnie
- Uprawnienia są kumulatywne (stosuje się najwyższy poziom uprawnień)
- Zmiany ról w IdP są odzwierciedlane przy następnym logowaniu

### Zarządzanie użytkownikami SAML

#### Tworzenie użytkownika

Gdy użytkownik loguje się przez SAML po raz pierwszy:
1. **Konto użytkownika**: Automatycznie tworzone z adresem e-mail jako identyfikatorem
2. **Przypisanie ról**: Role stosowane na podstawie atrybutów SAML
3. **Informacje o profilu**: Imię/nazwisko uzupełniane, jeśli zostały podane
4. **Aktywacja uprawnień**: Role stają się aktywne natychmiast

#### Aktualizacje ról

Istniejący użytkownicy SAML otrzymują aktualizacje ról:
1. **Wyzwalacz logowania**: Aktualizacje ról następują przy każdym logowaniu SAML
2. **Natychmiastowy efekt**: Nowe uprawnienia stosowane są natychmiast
3. **Usunięcie roli**: Usunięte role są automatycznie cofane
4. **Ślad audytu**: Zmiany ról są rejestrowane w dziennikach audytu

### Niestandardowe mapowanie ról

#### Dostosowanie dla przedsiębiorstw

Dla klientów enterprise ze specyficznymi wymaganiami:
- Niestandardowe nazwy ról mogą być mapowane na uprawnienia FastComments
- Można wdrożyć złożone hierarchie ról
- Można skonfigurować kontrole dostępu specyficzne dla działów

Skontaktuj się z pomocą techniczną FastComments w sprawie niestandardowych konfiguracji mapowania ról.

#### Walidacja ról

FastComments waliduje przychodzące role:
- Nierozpoznane role są ignorowane (nie odrzucane)
- Nieprawidłowo sformatowane atrybuty ról są rejestrowane w celu rozwiązywania problemów
- Użytkownicy zachowują istniejące role, jeśli asercja SAML nie zawiera informacji o rolach

### Najlepsze praktyki

#### Zarządzanie rolami

1. **Zasada najmniejszych uprawnień**: Przypisuj minimalne niezbędne uprawnienia
2. **Regularne audyty**: Okresowo przeglądaj role i dostępy użytkowników  
3. **Jasne nazewnictwo**: Używaj opisowych nazw grup w swoim IdP
4. **Dokumentacja**: Prowadź dokumentację przypisań ról

#### Kwestie bezpieczeństwa

1. **Atrybuty ról**: Upewnij się, że atrybuty ról są odpowiednio zabezpieczone w odpowiedziach SAML
2. **Walidacja atrybutów**: Zweryfikuj, że tylko autoryzowane systemy mogą przypisywać role
3. **Przeglądy dostępu**: Regularnie przeglądaj przypisania ról administracyjnych
4. **Monitorowanie**: Monitoruj zmiany ról i działania administracyjne

### Rozwiązywanie problemów z rolami

#### Częste problemy

**Role nie są stosowane**:
- Sprawdź, czy nazwy atrybutów SAML odpowiadają obsługiwanym formatom
- Zweryfikuj, czy IdP wysyła informacje o rolach
- Upewnij się, że wartości ról dokładnie odpowiadają nazwom ról FastComments

**Odmowa dostępu**:
- Zweryfikuj, czy użytkownik ma przypisaną odpowiednią rolę w IdP
- Sprawdź pisownię roli i wielkość liter
- Upewnij się, że rola jest poprawnie sformatowana w odpowiedzi SAML

**Brakujące uprawnienia**:
- Przejrzyj definicje ról i wymagane uprawnienia
- Sprawdź, czy nie ma konfliktowych przypisań ról
- Zweryfikuj, czy użytkownik zalogował się po zmianach ról