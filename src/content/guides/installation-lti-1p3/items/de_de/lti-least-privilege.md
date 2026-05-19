Die FastComments LTI 1.3-Integration folgt dem Prinzip der minimalen Rechtevergabe: Sie verwendet nur die Launch-Claims, die erforderlich sind, um den Benutzer zu identifizieren, Kommentare dem richtigen Kurs und der richtigen Ressource zuzuordnen und rollenbasierte Berechtigungen anzuwenden.

Der Rest dieser Seite ordnet jede Claim, die die Integration verwendet, jeden LTI Advantage-Dienst, den sie nicht anfordert, und jede Kategorie von Daten, die sie nicht sammelt. Sicherheits- und Beschaffungsgutachter können Antworten direkt aus den untenstehenden Tabellen entnehmen.

## Vom LMS empfangene Datenelemente

Jeder LTI 1.3-Launch enthält ein vom LMS signiertes JWT. FastComments extrahiert die folgenden Claims aus diesem JWT und verwendet nichts anderes:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifiziert den Benutzer konsistent über Launches hinweg, sodass dieselbe Person auf denselben FastComments-SSO-Benutzer abgebildet wird | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Die neben den Kommentaren des Benutzers angezeigte Attribution | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Kontoabgleich, Benachrichtigungen, Moderation, Support-Korrespondenz | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Wird neben den Kommentaren des Benutzers angezeigt | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Bestimmt, ob der Benutzer Administrator, Instructor (Moderator) oder Lernender ist | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Verknüpft den Kommentar-Thread mit dem korrekten LMS-Kurs | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Ordnet Kommentare der richtigen Aktivität oder Tool-Platzierung innerhalb des Kurses zu | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Leitet den Launch an die korrekte FastComments-Tenant-Konfiguration weiter | Yes | Yes, on the FastComments LTI configuration record |

## Bei der Registrierung deklarierte Claims und Scopes

Während der LTI 1.3 Dynamic Registration registriert sich FastComments mit `scope: ""` (keine zusätzlichen OAuth-Scopes) und deklariert nur diese OpenID Connect-Claims:

`iss`, `sub`, `name`, `email`, `picture`

Es registriert zwei Nachrichtentypen:

- `LtiResourceLinkRequest` - der standardmäßige Kurs-Launch in FastComments.
- `LtiDeepLinkingRequest` - ermöglicht Dozenten, das FastComments-Tool innerhalb eines Kurses zu platzieren.

Vom LMS werden keine zusätzlichen Zugriffstoken angefordert.

## Nicht angeforderte LTI Advantage-Dienste

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | Die Integration benötigt kein Kursverzeichnis; die Benutzeridentität wird mit jedem Launch übermittelt |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | Die Integration ist nicht für das Notenbuch ausgelegt |
| Deep Linking beyond the standard placement return | No additional data | Deep Linking wird nur für die Platzierung des Tools durch Dozenten verwendet; es werden keine Kursinhalte aufgelistet |

## Nicht gesammelte Daten

Neben LTI selbst fordert FastComments vom LMS oder Benutzer nicht die folgenden Daten an oder empfängt sie nicht:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Zugriffsbeschränkungen

- FastComments erhält Daten nur im Rahmen eines autorisierten LTI 1.3-Launches, der mit den registrierten Schlüsseln des LMS signiert ist. Die Integration ruft das LMS nicht zur zusätzlichen Informationsgewinnung auf.
- Launch-Token sind einmalig verwendbar und kurzlebig. Wiederholte oder abgelaufene Token werden abgewiesen.
- LMS-Administratoren legen fest, wo das Tool in ihrer Plattform bereitgestellt wird. D2L Brightspace unterstützt beispielsweise deployment-spezifische Einschränkungen auf Organisationseinheiten und deployment-spezifische Sicherheitseinstellungen, wodurch Administratoren das Tool auf bestimmte Kurse oder Organisationseinheiten beschränken können, anstatt es global verfügbar zu machen. Moodle, Blackboard, Sakai und Schoology bieten in ihren LTI 1.3-Implementierungen äquivalente deployment-spezifische Steuerungen.

## Speicherung und Aufbewahrung

FastComments bewahrt LTI-abgeleitete Daten für die Dauer des aktiven Kommentardienstes und gemäß den vom Kunden konfigurierten Aufbewahrungseinstellungen auf. Kommentardaten werden in der Produktion in verschlüsseltem Speicher (encrypted-at-rest) abgelegt. Bei Kontokündigung oder schriftlicher Löschanfrage löscht oder anonymisiert FastComments Kundendaten gemäß der geltenden Vereinbarung.

Für vollständige Details zur Speicherung und Datenverarbeitung siehe die <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments-Datenschutzerklärung</a>.

## Überprüfungsrhythmus

Jedes neue LTI-Feature, das zusätzliche Claims, Scopes oder LTI Advantage-Dienste erfordern würde, wird vor der Veröffentlichung überprüft, um zu bestätigen, dass der angeforderte Zugriff notwendig und verhältnismäßig zum ausgelieferten Feature ist.

## Kurze Erklärung für Sicherheitsfragebögen

> FastComments wendet das Prinzip der minimalen Rechtevergabe und Datenminimierung auf seine LTI 1.3-Integration an. Die Integration verwendet nur die LTI-Launch-Claims, die zur Authentifizierung des Benutzers (`sub`, `name`, `email`, `picture`) erforderlich sind, um seine Rolle zu bestimmen und den Kurs sowie die Ressource zu identifizieren, zu der die Kommentare gehören. FastComments fordert weder Names and Role Provisioning Services noch Assignment and Grade Services, keine Notenbuchdaten, Anwesenheitsdaten, vollständige Teilnehmerlisten oder Administratorzugriff auf das LMS an. LMS-Administratoren behalten die Kontrolle darüber, für welche Organisationseinheiten, Kurse und Deployments das Tool verfügbar ist.