SAML (Security Assertion Markup Language) ist ein auf XML basierender offener Standard zum Austausch von Authentifizierungs- und Autorisierungsdaten zwischen Parteien, insbesondere zwischen einem Identitätsanbieter (IdP) und einem Dienstanbieter (SP).

### Wie SAML funktioniert

SAML ermöglicht Single Sign-On (SSO), indem es Benutzern erlaubt, sich einmal bei ihrem Identitätsanbieter zu authentifizieren und dann auf mehrere Anwendungen zuzugreifen, ohne Anmeldeinformationen erneut eingeben zu müssen. Wenn ein Benutzer versucht, auf FastComments zuzugreifen:

1. **Authentifizierungsanfrage**: FastComments leitet den Benutzer zu Ihrem Identitätsanbieter weiter
2. **Benutzer-Authentifizierung**: Der Benutzer authentifiziert sich bei Ihrem IdP (z. B. Active Directory, Okta, Azure AD)
3. **SAML-Antwort**: Der IdP sendet eine signierte SAML-Assertion zurück an FastComments
4. **Benutzerzugang**: FastComments prüft die Assertion und gewährt dem authentifizierten Benutzer Zugriff

### Vorteile von SAML

- **Erhöhte Sicherheit**: Zentralisierte Authentifizierung reduziert passwortbezogene Sicherheitsrisiken
- **Verbessertes Benutzererlebnis**: Benutzer melden sich einmal an und greifen nahtlos auf mehrere Anwendungen zu
- **Compliance**: Hilft, regulatorische Anforderungen für Zugriffskontrolle und Prüfpfade zu erfüllen
- **Administrative Kontrolle**: IT-Administratoren behalten die zentrale Benutzerverwaltung

### Unterstützung für SAML 2.0

FastComments implementiert SAML 2.0, die am weitesten verbreitete Version des SAML-Standards. Unsere Implementierung unterstützt:

- HTTP-POST- und HTTP-Redirect-Bindings
- Signierte SAML-Antworten und -Assertions
- Verschlüsselte Assertions (optional)
- Mehrere Signatur- und Digest-Algorithmen
- Verschiedene Name-Identifier-Formate