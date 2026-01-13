Po skonfigurowaniu SAML w FastComments, musisz dodać FastComments jako Dostawcę Usług (Service Provider) w swoim dostawcy tożsamości.

### Ogólna konfiguracja IdP

Większość dostawców tożsamości wymaga następujących informacji, aby dodać FastComments jako aplikację SAML:

#### Wymagane informacje o Service Provider

Te wartości są automatycznie generowane i wyświetlane na stronie konfiguracji SAML w FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- To jednoznacznie identyfikuje Twoją instancję FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Miejsce, do którego Twój IdP wysyła odpowiedzi SAML po uwierzytelnieniu

**SP Metadata URL** *(jeśli obsługiwane przez Twojego IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Zapewnia kompletną konfigurację SAML w formacie XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Bezpośredni link do zainicjowania uwierzytelniania SAML

### Wymagane atrybuty SAML

Skonfiguruj swojego dostawcę tożsamości, aby wysyłał te atrybuty w odpowiedziach SAML:

#### Podstawowe atrybuty

**Email Address** *(Wymagane)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Cel**: Jednoznaczna identyfikacja użytkownika i powiadomienia
- **Format**: Prawidłowy adres e-mail

#### Opcjonalne atrybuty

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Cel**: Imię wyświetlane użytkownika

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Cel**: Nazwisko wyświetlane użytkownika

**Roles** *(Ważne dla kontroli dostępu)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Cel**: Przypisywanie ról i uprawnień w FastComments
- **Format**: Tablica łańcuchów ról lub wartości rozdzielone przecinkami

### Częste konfiguracje dostawców tożsamości

#### Microsoft Azure AD

1. **Dodaj Enterprise Application**
   - Wyszukaj "FastComments" lub utwórz niestandardową aplikację SAML
   - Użyj informacji SP dostarczonych przez FastComments

2. **Skonfiguruj atrybuty**
   - Email: `user.mail` lub `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` lub grupy katalogowe

#### Okta

1. **Utwórz aplikację SAML**
   - Wybierz "Create New App" i SAML 2.0
   - Skonfiguruj przy użyciu informacji SP od FastComments

2. **Deklaracje atrybutów**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` lub niestandardowe atrybuty

#### Google Workspace

1. **Dodaj aplikację SAML**
   - Przejdź do Apps > Web and mobile apps > Add App > Add custom SAML app
   - Skonfiguruj przy użyciu informacji SP od FastComments

2. **Mapowanie atrybutów**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups lub niestandardowe atrybuty

#### Active Directory Federation Services (ADFS)

1. **Dodaj Relying Party Trust**
   - Użyj URL metadanych FastComments lub konfiguracji ręcznej
   - Skonfiguruj informacje SP zgodnie z dostarczonymi danymi

2. **Reguły roszczeń (Claim Rules)**
   - Email: roszczenie Email Address
   - Name: roszczenie Name ID
   - Roles: członkostwo w grupie lub niestandardowe roszczenia

### Elastyczność nazw atrybutów

FastComments akceptuje informacje o rolach z wielu nazw atrybutów, aby dopasować się do różnych konfiguracji IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ta elastyczność zapewnia zgodność z różnymi dostawcami tożsamości bez wymogu stosowania konkretnych konwencji nazewniczych atrybutów.

### Testowanie konfiguracji

Po skonfigurowaniu dostawcy tożsamości:

1. Zapisz konfigurację IdP
2. Przetestuj przy użyciu dedykowanego konta testowego
3. Zweryfikuj, czy atrybuty są wysyłane poprawnie
4. Sprawdź, czy role są prawidłowo mapowane
5. Upewnij się, że proces uwierzytelniania kończy się pomyślnie

Większość dostawców tożsamości oferuje narzędzia do testowania SAML, aby zweryfikować konfigurację przed wdrożeniem dla użytkowników produkcyjnych.