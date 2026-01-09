Nachdem Sie SAML in FastComments konfiguriert haben, müssen Sie FastComments als Service Provider in Ihrem Identitätsanbieter einrichten.

### Allgemeine IdP-Konfiguration

Die meisten Identitätsanbieter (IdP) benötigen die folgenden Informationen, um FastComments als SAML-Anwendung hinzuzufügen:

#### Erforderliche Informationen zum Service Provider

Diese Werte werden automatisch generiert und auf Ihrer FastComments SAML-Konfigurationsseite angezeigt:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Dies identifiziert Ihre FastComments-Instanz eindeutig

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Wohin Ihr IdP SAML-Antworten nach der Authentifizierung sendet

**SP Metadata URL** *(falls von Ihrem IdP unterstützt)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Stellt die vollständige SAML-Konfiguration im XML-Format bereit

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Direkter Link, um die SAML-Authentifizierung zu starten

### Erforderliche SAML-Attribute

Konfigurieren Sie Ihren Identitätsanbieter so, dass diese Attribute mit SAML-Antworten gesendet werden:

#### Wesentliche Attribute

**E-Mail-Adresse** *(erforderlich)*
- **Attribute Name**: `email`, `emailAddress`, oder `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Zweck**: Eindeutige Benutzeridentifikation und Benachrichtigungen
- **Format**: Gültige E-Mail-Adresse

#### Optionale Attribute

**Vorname**
- **Attribute Names**: `firstName`, `givenName`, oder `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Zweck**: Anzeigename des Benutzers

**Nachname**
- **Attribute Names**: `lastName`, `surname`, oder `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Zweck**: Anzeigename des Benutzers

**Rollen** *(wichtig für Zugriffskontrolle)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, oder benutzerdefinierte Attributnamen
- **Zweck**: Zuweisung von FastComments-Rollen und Berechtigungen
- **Format**: Array von Rollen-Strings oder kommaseparierte Werte

### Gängige Identitätsanbieter-Konfigurationen

#### Microsoft Azure AD

1. **Enterprise-Anwendung hinzufügen**
   - Suchen Sie nach "FastComments" oder erstellen Sie eine benutzerdefinierte SAML-Anwendung
   - Verwenden Sie die von FastComments bereitgestellten SP-Informationen

2. **Attribute konfigurieren**
   - E-Mail: `user.mail` oder `user.userprincipalname`
   - Vorname: `user.givenname`
   - Nachname: `user.surname`
   - Rollen: `user.assignedroles` oder Verzeichnisgruppen

#### Okta

1. **SAML-Anwendung erstellen**
   - Verwenden Sie "Create New App" und wählen Sie SAML 2.0
   - Konfigurieren Sie mit FastComments SP-Informationen

2. **Attributzuweisungen**
   - E-Mail: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Rollen: `user.groups` oder benutzerdefinierte Attribute

#### Google Workspace

1. **SAML-Anwendung hinzufügen**
   - Gehen Sie zu Apps > Web and mobile apps > Add App > Add custom SAML app
   - Konfigurieren Sie mit FastComments SP-Informationen

2. **Attributzuordnung**
   - E-Mail: Primäre E-Mail
   - Vorname: Vorname
   - Nachname: Nachname
   - Rollen: Gruppen oder benutzerdefinierte Attribute

#### Active Directory Federation Services (ADFS)

1. **Relying Party Trust hinzufügen**
   - Verwenden Sie die FastComments-Metadaten-URL oder die manuelle Konfiguration
   - Konfigurieren Sie die SP-Informationen wie angegeben

2. **Claim Rules**
   - E-Mail: Email Address claim
   - Name: Name ID claim
   - Rollen: Gruppenmitgliedschaft oder benutzerdefinierte Claims

### Flexibilität der Attributnamen

FastComments akzeptiert Rolleninformationen aus mehreren Attributnamen, um verschiedene IdP-Konfigurationen zu unterstützen:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Diese Flexibilität stellt die Kompatibilität mit verschiedenen Identitätsanbietern sicher, ohne bestimmte Namenskonventionen für Attribute zu erzwingen.

### Testen Ihrer Konfiguration

Nachdem Sie Ihren Identitätsanbieter konfiguriert haben:

1. Speichern Sie die IdP-Konfiguration
2. Testen Sie mit einem dedizierten Test-Benutzerkonto
3. Überprüfen Sie, ob die Attribute korrekt gesendet werden
4. Stellen Sie sicher, dass Rollen korrekt zugeordnet sind
5. Stellen Sie sicher, dass der Authentifizierungsfluss erfolgreich abgeschlossen wird

Die meisten Identitätsanbieter bieten SAML-Testtools an, um die Konfiguration zu validieren, bevor sie für Produktionsbenutzer bereitgestellt wird.