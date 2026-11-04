Wenn SAML in FastComments aktiviert ist, generiert das System automatisch Service-Provider-Informationen (SP), die Sie in Ihrem Identitätsanbieter konfigurieren müssen.

### Zugriff auf Service-Provider-Informationen

Die SP-Informationen werden auf Ihrer SAML-Konfigurationsseite angezeigt, nachdem die SAML-Authentifizierung aktiviert wurde. Diese Informationen enthalten alle Details, die Ihr Identitätsanbieter benötigt, um die SAML-Vertrauensbeziehung herzustellen.

### Service-Provider-Endpunkte

#### SP Entity ID / Audience
**Zweck**: Identifiziert Ihre FastComments-Instanz eindeutig als Service Provider
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Verwendung**: Konfigurieren Sie dies als Entity ID oder Audience in Ihrem IdP

Dieser Bezeichner stellt sicher, dass SAML-Antworten für Ihren spezifischen FastComments-Tenant bestimmt sind und verhindert, dass SAML-Antworten von anderen Instanzen akzeptiert werden.

#### Assertion Consumer Service (ACS) URL
**Zweck**: Der Endpunkt, an den Ihr IdP SAML-Antworten nach der Benutzerauthentifizierung sendet
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Verwendung**: Konfigurieren Sie dies als ACS-URL oder Reply-URL in Ihrem IdP

Hier werden Benutzer nach erfolgreicher Authentifizierung bei Ihrem Identitätsanbieter zusammen mit der SAML-Assertion mit Benutzerinformationen weitergeleitet.

#### SP Metadata URL
**Zweck**: Stellt die vollständige SAML-Konfiguration im standardmäßigen XML-Format bereit
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Verwendung**: Einige IdPs können die Konfiguration automatisch über diese URL importieren

Die Metadata-URL enthält alle notwendigen SP-Informationen im XML-Format, was die automatische Konfiguration kompatibler Identitätsanbieter erleichtert.

#### SAML Login URL
**Zweck**: Direkter Link zum Starten der SAML-Authentifizierung für Ihren Tenant
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Verwendung**: Verlinken Sie Benutzer direkt zur SAML-Authentifizierung oder testen Sie den Ablauf

Sie können diese URL verwenden, um die SAML-Authentifizierung zu testen oder Benutzern einen direkten Link zur Anmeldung über SAML bereitzustellen.

### Unterstützung von SAML-Bindings

FastComments unterstützt die folgenden SAML-Bindings:

#### HTTP-POST Binding
- **Primäre Methode**: Das häufigste Binding für SAML-Antworten
- **Sicherheit**: Die SAML-Antwort wird per HTTP POST an die ACS-URL gesendet
- **Verwendung**: Empfohlen für Produktionsumgebungen

#### HTTP-Redirect Binding
- **Alternative Methode**: SAML-Antwort wird per HTTP-Redirect gesendet
- **Einschränkungen**: Begrenzte Payload-Größe aufgrund von URL-Längenbeschränkungen
- **Verwendung**: Wird unterstützt, HTTP-POST wird jedoch bevorzugt

### Name ID Policy

FastComments konfiguriert die folgende Name-ID-Richtlinie in SAML-Anfragen:

- **Standardformat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternative Formate**: Persistent, Transient, Unspecified (konfigurierbar)
- **Anforderung**: Die E-Mail-Adresse wird als primärer Benutzeridentifikator verwendet

### SAML-Anfrageattribute

Beim Start der SAML-Authentifizierung sendet FastComments Anfragen mit den folgenden Eigenschaften:

#### Anfrage-Signierung
- **Status**: Optional (konfigurierbar)
- **Algorithmus**: Entspricht dem konfigurierten Signaturalgorithmus
- **Zertifikat**: Verwendet ein tenant-spezifisches Zertifikat, wenn die Anfrage-Signierung aktiviert ist

#### Angeforderte Attribute
FastComments fordert die folgenden Attribute in SAML AuthnRequests an:

- **Email**: Erforderlich zur Benutzeridentifikation
- **First Name**: Optional für Anzeigezwecke  
- **Last Name**: Optional für Anzeigezwecke
- **Roles/Groups**: Optional für Zugriffskontrolle und Berechtigungen

### Kopieren von SP-Informationen

Die SAML-Konfigurationsseite bietet anklickbare Felder, die SP-Informationen automatisch in Ihre Zwischenablage kopieren:

1. Klicken Sie auf ein beliebiges SP-Informationsfeld (Entity ID, ACS URL usw.)
2. Der Wert wird automatisch in Ihre Zwischenablage kopiert
3. Fügen Sie den Wert in die Konfiguration Ihres Identitätsanbieters ein
4. Eine kurze Hervorhebung zeigt das erfolgreiche Kopieren an

Dies erleichtert die genaue Übertragung der SP-Informationen an Ihren IdP ohne Tippfehler.

### SP-Zertifikatsinformationen

#### Zertifikatsverwendung
- **Zweck**: Verschlüsselt die Kommunikation und verifiziert die SP-Identität
- **Rotation**: Zertifikate werden automatisch von FastComments verwaltet
- **Zugriff**: Öffentliche Zertifikate sind über die Metadata-URL verfügbar

#### Zertifikatsdetails
- **Algorithmus**: RSA-2048 oder höher
- **Gültigkeit**: Zertifikate werden vor Ablauf automatisch erneuert
- **Verteilung**: Verfügbar über standardmäßige SAML-Metadaten

### Fehlerbehebung bei der SP-Konfiguration

Wenn Ihr Identitätsanbieter Probleme mit den SP-Informationen meldet:

1. **URLs überprüfen**: Stellen Sie sicher, dass alle URLs HTTPS verwenden und die korrekte Tenant-ID enthalten
2. **Metadaten prüfen**: Verwenden Sie die Metadata-URL, um die Konfiguration zu überprüfen
3. **Konnektivität testen**: Stellen Sie sicher, dass Ihr IdP die FastComments-Endpunkte erreichen kann
4. **Format validieren**: Bestätigen Sie, dass Ihr IdP das Format der SP-Informationen unterstützt

Häufige Probleme sind:
- Falsche Tenant-ID in den URLs
- Netzwerkverbindungsprobleme zwischen IdP und FastComments
- IdP erwartet andere URL-Formate oder zusätzliche Konfigurationsoptionen