FastComments bietet sowohl SSO- als auch SAML-Authentifizierung. Das Verständnis der Unterschiede hilft Ihnen, den richtigen Ansatz für Ihre Organisation auszuwählen.

### Simple/Secure SSO Optionen

FastComments bietet zwei verschiedene SSO-Flows zur Authentifizierung im Kommentar-Widget über Ihre Website.
Dies unterscheidet sich von SAML und erfordert kein SAML. Stattdessen benötigt Simple SSO lediglich das Übergeben eines Objekts an
das Kommentar-Widget, während Secure SSO dies zusätzlich durch das Hashen der Nutzlast mit einem API-Schlüssel absichert.

SAML hingegen authentifiziert den Benutzer für das gesamte Produkt (basierend auf seinen Berechtigungen) *sowie* für das Kommentar-Widget
(wenn für unsere Domain Cookies von Drittanbietern aktiviert sind).

### SAML-Authentifizierung

SAML ist ein unternehmensweites Authentifizierungsprotokoll, das robustere Sicherheits- und Integrationsmöglichkeiten bietet:

- **Implementierung**: Erfordert Konfiguration des Identity Providers (IdP) und Zertifikatsaustausch
- **Sicherheit**: Verwendet signierte XML-Assertions und unterstützt Verschlüsselung
- **Anwendungsfall**: Ideal für Unternehmen mit bestehender SAML-Infrastruktur (Active Directory, Okta, etc.)
- **Einrichtungsaufwand**: Aufwändiger - erfordert IdP-Konfiguration und Zertifikatsverwaltung
- **Enterprise-Funktionen**: Erweiterte Rollen-Zuordnung, zentrale Benutzerverwaltung, Prüfundsprotokolle

### Wann Sie SAML wählen sollten

Erwägen Sie die SAML-Authentifizierung, wenn Ihre Organisation:

- Bereits einen SAML-kompatiblen Identity Provider verwendet (Okta, Azure AD, ADFS, etc.)
- Enterprise-Grade-Sicherheit und Compliance benötigt
- Zentrale Benutzerverwaltung und Zugriffskontrolle benötigt
- Mehrere Anwendungen hat, die SAML für die Authentifizierung verwenden
- Detaillierte Prüfundsprotokolle und Sicherheitsberichte benötigt

### Wann Sie Simple oder Secure SSO wählen sollten

Unsere widget-fokussierten SSO-Lösungen sind möglicherweise ausreichend, wenn Sie:

- Ein eigenes Authentifizierungssystem haben
- Eine schnelle Implementierung mit minimaler Einrichtung benötigen
- Keine Integration mit einem Enterprise-Identity-Provider benötigen
- Benutzerdaten direkt aus Ihrer Anwendung steuern möchten
- Einfachere Sicherheitsanforderungen haben

Simple und Secure SSO werden häufig für Online-Portale, Blogs usw. verwendet, bei denen der Benutzer bereits ein Konto *über Ihre Website oder App* hat,
aber nicht unbedingt SAML verwendet.