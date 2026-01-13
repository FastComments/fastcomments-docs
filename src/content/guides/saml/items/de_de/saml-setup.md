Das Einrichten der SAML-Authentifizierung in FastComments erfordert sowohl eine Konfiguration in Ihrem Admin-Dashboard als auch eine Einrichtung in Ihrem Identitätsanbieter.

### Voraussetzungen

Bevor Sie SAML konfigurieren, stellen Sie sicher, dass Sie:

- Einen FastComments Flex- oder Pro-Plan haben (SAML ist im Creators-Plan nicht verfügbar)
- Administrativen Zugriff auf Ihr FastComments-Konto haben
- Administrativen Zugriff auf Ihren Identitätsanbieter haben
- Die SAML-Metadaten oder Zertifikatsinformationen Ihres IdP vorliegen haben

### Zugriff auf die SAML-Konfiguration

1. Melden Sie sich bei Ihrem [FastComments Admin-Dashboard](https://fastcomments.com/auth/my-account) an
2. Navigieren Sie in der linken Seitenleiste zu **API/SSO-Einstellungen**
3. Klicken Sie auf die **SAML-Konfiguration**-Schaltfläche

Wenn Sie die Schaltfläche zur SAML-Konfiguration nicht sehen, prüfen Sie:
- Ihr Konto verfügt über das erforderliche Paket (Flex oder Pro)
- Sie haben administrative Berechtigungen
- Ihr Benutzer verfügt über die Rollen API Admin oder Admin Admin

### Grundlegende SAML-Konfiguration

#### SAML-Authentifizierung aktivieren

1. Aktivieren Sie das Kontrollkästchen **Enable SAML Authentication**
2. Dies aktiviert SAML für Ihren Mandanten und macht die Konfigurationsfelder verfügbar

#### Erforderliche Felder

**IdP Single Sign-On URL** *(Erforderlich)*
- Die URL, auf die Benutzer zur Authentifizierung weitergeleitet werden
- Wird normalerweise von Ihrem Identitätsanbieter bereitgestellt
- Beispiel: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Erforderlich)*
- Das öffentliche Zertifikat Ihres Identitätsanbieters
- Wird verwendet, um die Authentizität von SAML-Antworten zu überprüfen
- Muss das vollständige Zertifikat mit BEGIN/END-Markierungen enthalten
- Beispiel-Format:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Optionale Felder

**IdP Entity ID / Issuer**
- Identifiziert Ihren Identitätsanbieter
- Wenn leer gelassen, wird standardmäßig Ihre FastComments-URL verwendet
- Sollte mit dem im IdP konfigurierten Issuer übereinstimmen

### Erweiterte Konfiguration

#### Sicherheitseinstellungen

**Signature Algorithm**
- Standardmäßig SHA-256 (empfohlen)
- Optionen: SHA-1, SHA-256, SHA-512
- Sollte mit der Konfiguration Ihres IdP übereinstimmen

**Digest Algorithm**
- Standardmäßig SHA-256 (empfohlen)
- Wird für die Digest-Berechnung in SAML-Antworten verwendet
- Sollte mit der Konfiguration Ihres IdP übereinstimmen

**Name ID Format**
- Standardmäßig das Format E-Mail-Adresse
- Bestimmt, wie Benutzerkennungen formatiert werden
- Gängige Optionen: E-Mail-Adresse, Persistent, Transient

#### Verschlüsselung (optional)

**Private Key for Decryption**
- Nur erforderlich, wenn Ihr IdP SAML-Assertions verschlüsselt
- Fügen Sie hier Ihren privaten Schlüssel zur Entschlüsselung ein
- Bei den meisten Bereitstellungen ist die Assertion-Verschlüsselung nicht erforderlich

### Konfiguration speichern

1. Überprüfen Sie alle Einstellungen auf Richtigkeit
2. Klicken Sie auf **Save SAML Configuration**
3. Das System wird Ihre Konfiguration validieren
4. Bei Erfolg sehen Sie eine Bestätigungsnachricht

### Nächste Schritte

Nachdem Sie Ihre FastComments-SAML-Konfiguration gespeichert haben:

1. Konfigurieren Sie Ihren Identitätsanbieter mit den Service-Provider-Informationen
2. Testen Sie den Authentifizierungsablauf
3. Richten Sie Benutzerrollen und Berechtigungen nach Bedarf ein

Die für die Konfiguration Ihres IdP benötigten Service-Provider-Informationen werden angezeigt, sobald SAML aktiviert ist.