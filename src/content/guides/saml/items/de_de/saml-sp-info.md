Wenn SAML in FastComments aktiviert ist, generiert das System automatisch Service Provider (SP)-Informationen, die Sie in Ihrem Identitätsanbieter konfigurieren müssen.

### Zugriff auf Service Provider-Informationen

Die SP-Informationen werden auf Ihrer SAML-Konfigurationsseite angezeigt, nachdem die SAML-Authentifizierung aktiviert wurde. Diese Informationen enthalten alle Details, die Ihr Identitätsanbieter benötigt, um die SAML-Vertrauensbeziehung herzustellen.

### Service Provider-Endpunkte

#### SP Entity ID / Audience
**Zweck**: Identifiziert eindeutig Ihre FastComments-Instanz als Service Provider  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Verwendung**: Konfigurieren Sie dies als Entity ID oder Audience in Ihrem IdP

Dieser Bezeichner stellt sicher, dass SAML-Antworten für Ihren spezifischen FastComments-Mandanten bestimmt sind und verhindert, dass SAML-Antworten von anderen Instanzen akzeptiert werden.

#### Assertion Consumer Service (ACS) URL
**Zweck**: Der Endpunkt, an den Ihr IdP SAML-Antworten nach der Benutzer-Authentifizierung sendet  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Verwendung**: Konfigurieren Sie dies als ACS-URL oder Reply URL in Ihrem IdP

Hierhin werden Benutzer nach erfolgreicher Authentifizierung bei Ihrem Identitätsanbieter zusammen mit der SAML-Assertion mit Benutzerinformationen umgeleitet.

#### SP Metadata URL
**Zweck**: Stellt die vollständige SAML-Konfiguration im standardmäßigen XML-Format bereit  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Verwendung**: Einige IdPs können die Konfiguration automatisch über diese URL importieren

Die Metadata-URL enthält alle notwendigen SP-Informationen im XML-Format, wodurch es einfach ist, kompatible Identitätsanbieter automatisch zu konfigurieren.

#### SAML Login URL
**Zweck**: Direkter Link zum Starten der SAML-Authentifizierung für Ihren Mandanten  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Verwendung**: Leiten Sie Benutzer direkt zur SAML-Authentifizierung oder testen Sie den Ablauf

Sie können diese URL verwenden, um die SAML-Authentifizierung zu testen oder Benutzern einen direkten Link zur Anmeldung über SAML bereitzustellen.

### Unterstützte SAML-Bindings

FastComments unterstützt die folgenden SAML-Bindings:

#### HTTP-POST Binding
- **Hauptmethode**: Häufigste Bindung für SAML-Antworten  
- **Sicherheit**: SAML-Antwort wird per HTTP POST an die ACS-URL gesendet  
- **Verwendung**: Für Produktionsumgebungen empfohlen

#### HTTP-Redirect Binding
- **Alternative Methode**: SAML-Antwort wird per HTTP-Redirect gesendet  
- **Einschränkungen**: Begrenzte Nutzlastgröße aufgrund von URL-Längenbeschränkungen  
- **Verwendung**: Unterstützt, aber HTTP-POST wird bevorzugt

### Name ID-Richtlinie

FastComments konfiguriert die folgende Name ID-Richtlinie in SAML-Anfragen:

- **Standardformat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative Formate**: Persistent, Transient, Unspecified (konfigurierbar)  
- **Anforderung**: Die E-Mail-Adresse wird als primärer Benutzerbezeichner verwendet

### SAML-Anfrageattribute

Beim Starten der SAML-Authentifizierung sendet FastComments Anfragen mit diesen Merkmalen:

#### Anfragensignierung
- **Status**: Optional (konfigurierbar)  
- **Algorithmus**: Entspricht dem konfigurierten Signaturalgorithmus  
- **Zertifikat**: Verwendet mandantenspezifisches Zertifikat, wenn die Anfragensignierung aktiviert ist

#### Angeforderte Attribute
FastComments fordert die folgenden Attribute in SAML AuthnRequests an:

- **Email**: Erforderlich zur Benutzeridentifizierung  
- **First Name**: Optional für Anzeigezwecke  
- **Last Name**: Optional für Anzeigezwecke  
- **Roles/Groups**: Optional für Zugriffskontrolle und Berechtigungen

### Kopieren von SP-Informationen

Die SAML-Konfigurationsseite bietet klickbare Felder, die SP-Informationen automatisch in Ihre Zwischenablage kopieren:

1. Klicken Sie auf ein beliebiges SP-Informationsfeld (Entity ID, ACS URL usw.)  
2. Der Wert wird automatisch in Ihre Zwischenablage kopiert  
3. Fügen Sie den Wert in die Konfiguration Ihres Identitätsanbieters ein  
4. Eine kurze Hervorhebung zeigt erfolgreiches Kopieren an

So können Sie die SP-Informationen ohne Tippfehler genau an Ihren IdP übertragen.

### SP-Zertifikatsinformationen

#### Zertifikatverwendung
- **Zweck**: Verschlüsselt die Kommunikation und verifiziert die SP-Identität  
- **Rotation**: Zertifikate werden automatisch von FastComments verwaltet  
- **Zugriff**: Öffentliche Zertifikate sind über die Metadata-URL verfügbar

#### Zertifikatdetails
- **Algorithmus**: RSA-2048 or higher  
- **Gültigkeit**: Zertifikate werden automatisch vor dem Ablaufdatum erneuert  
- **Verteilung**: Über standardmäßige SAML-Metadaten verfügbar

### Fehlerbehebung bei SP-Konfiguration

Wenn Ihr Identitätsanbieter Probleme mit SP-Informationen meldet:

1. **URLs überprüfen**: Stellen Sie sicher, dass alle URLs HTTPS verwenden und die korrekte Mandanten-ID enthalten  
2. **Metadata prüfen**: Verwenden Sie die Metadata-URL, um die Konfiguration zu überprüfen  
3. **Konnektivität testen**: Stellen Sie sicher, dass Ihr IdP die FastComments-Endpunkte erreichen kann  
4. **Format validieren**: Bestätigen Sie, dass Ihr IdP das Format der SP-Informationen unterstützt

Häufige Probleme sind:
- Falsche Mandanten-ID in den URLs  
- Netzwerkverbindungsprobleme zwischen IdP und FastComments  
- IdP erwartet andere URL-Formate oder zusätzliche Konfigurationsoptionen