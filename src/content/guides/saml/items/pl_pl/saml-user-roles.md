FastComments mapuje role użytkowników SAML na wewnętrzne uprawnienia, umożliwiając kontrolę dostępu opartą na rolach dla Twojej organizacji.

### FastComments Role System

FastComments używa systemu uprawnień opartego na rolach, gdzie użytkownicy mogą mieć jedną lub więcej ról, które określają ich poziomy dostępu i możliwości.

### Available FastComments Roles

#### Administrative Roles

**fc-account-owner**
- **Permissions**: Pełny dostęp administracyjny
- **Capabilities**: Wszystkie funkcje, zarządzanie płatnościami, zarządzanie użytkownikami
- **Use Case**: Główni administratorzy konta i właściciele

**fc-admin-admin**  
- **Permissions**: Dostęp administracyjny do większości funkcji
- **Capabilities**: Zarządzanie użytkownikami, konfiguracja, moderacja. **Może zarządzać innymi administratorami.**
- **Use Case**: Administratorzy drugiego poziomu i personel IT

**fc-billing-admin**
- **Permissions**: Zarządzanie rozliczeniami i subskrypcjami
- **Capabilities**: Metody płatności, faktury, zmiany subskrypcji
- **Use Case**: Członkowie zespołu finansowego i osoby kontaktowe ds. rozliczeń

#### Specialized Roles

**fc-analytics-admin**
- **Permissions**: Dostęp do analityki i raportów
- **Capabilities**: Przeglądanie statystyk serwisu, danych zaangażowania użytkowników
- **Use Case**: Zespoły marketingowe i analitycy danych

**fc-api-admin**
- **Permissions**: Dostęp do API i zarządzanie nim
- **Capabilities**: Dane uwierzytelniające API, konfiguracja webhooków
- **Use Case**: Programiści i integratorzy techniczni

**fc-moderator**
- **Permissions**: Możliwości moderacji komentarzy
- **Capabilities**: Zatwierdzanie/odrzucanie komentarzy, zarządzanie spamem
- **Use Case**: Moderatorzy społeczności i menedżerowie treści

### Role Mapping Configuration

#### SAML Attribute Sources

FastComments akceptuje informacje o rolach z różnych nazw atrybutów SAML, aby zapewnić kompatybilność z różnymi dostawcami tożsamości:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Role Format Support

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Identity Provider Role Configuration

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - Zdefiniuj role FastComments w aplikacji Azure AD
   - Przypisz użytkowników do odpowiednich ról aplikacji
   - Skonfiguruj roszczenia, aby uwzględniały przypisane role

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - Utwórz grupy odpowiadające nazwom ról FastComments
   - Przypisz użytkowników do odpowiednich grup
   - Skonfiguruj deklaracje atrybutów

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - Utwórz jednostki organizacyjne lub grupy
   - Nazwij grupy z prefiksami ról FastComments
   - Skonfiguruj mapowanie atrybutów

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Default User Behavior

#### Users Without Roles

When a SAML user has no roles or unrecognized roles:
- Użytkownik jest tworzony jako standardowy komentujący
- Nie przyznaje się dostępu administracyjnego
- Może publikować i zarządzać własnymi komentarzami
- Nie ma dostępu do funkcji panelu administracyjnego

#### Role Inheritance

- Użytkownicy mogą mieć jednocześnie wiele ról
- Uprawnienia sumują się (stosuje się najwyższy poziom uprawnień)
- Zmiany ról w IdP są odzwierciedlane przy następnym logowaniu

### Managing SAML Users

#### User Creation

When a user logs in via SAML for the first time:
1. **User Account**: Automatycznie tworzone konto z adresem e-mail jako identyfikatorem
2. **Role Assignment**: Role przypisywane na podstawie atrybutów SAML
3. **Profile Information**: Imię/nazwisko wypełniane, jeśli są dostarczone
4. **Permission Activation**: Role stają się aktywne natychmiastowo

#### Role Updates

Existing SAML users receive role updates:
1. **Login Trigger**: Aktualizacje ról następują podczas każdego logowania SAML
2. **Immediate Effect**: Nowe uprawnienia obowiązują natychmiast
3. **Role Removal**: Usunięte role są automatycznie wycofywane
4. **Audit Trail**: Zmiany ról są rejestrowane w logach audytu

### Custom Role Mapping

#### Enterprise Customization

For enterprise customers with specific requirements:
- Nazwy niestandardowe ról mogą być mapowane na uprawnienia FastComments
- Można wdrożyć złożone hierarchie ról
- Można skonfigurować kontrolę dostępu specyficzną dla działów

Contact FastComments support for custom role mapping configurations.

#### Role Validation

FastComments validates incoming roles:
- Nieznane role są ignorowane (nie odrzucane)
- Błędnie sformatowane atrybuty ról są rejestrowane do celów rozwiązywania problemów
- Użytkownicy zachowują istniejące role, jeśli asercja SAML nie zawiera informacji o rolach

### Best Practices

#### Role Management

1. **Principle of Least Privilege**: Przypisuj minimalnie niezbędne uprawnienia
2. **Regular Auditing**: Regularnie przeglądaj role użytkowników i dostęp  
3. **Clear Naming**: Używaj opisowych nazw grup w swoim IdP
4. **Documentation**: Prowadź dokumentację przypisań ról

#### Security Considerations

1. **Role Attributes**: Upewnij się, że atrybuty ról są odpowiednio zabezpieczone w odpowiedziach SAML
2. **Attribute Validation**: Sprawdź, że tylko upoważnione systemy mogą przypisywać role
3. **Access Reviews**: Regularnie przeglądaj przypisania ról administracyjnych
4. **Monitoring**: Monitoruj zmiany ról i działania administracyjne

### Troubleshooting Role Issues

#### Common Problems

**Roles Not Applied**:
- Sprawdź, czy nazwy atrybutów SAML odpowiadają obsługiwanym formatom
- Zweryfikuj, czy IdP wysyła informacje o rolach
- Potwierdź, czy wartości ról dokładnie odpowiadają nazwom ról FastComments

**Access Denied**:
- Zweryfikuj, czy użytkownik ma odpowiednią rolę przypisaną w IdP
- Sprawdź pisownię roli i wielkość liter
- Potwierdź, że rola jest poprawnie sformatowana w odpowiedzi SAML

**Missing Permissions**:
- Przejrzyj definicje ról i wymagane uprawnienia
- Sprawdź konfliktujące przypisania ról
- Sprawdź, czy użytkownik zalogował się po zmianach ról

---