SSO, oder Single Sign-On, ist eine Sammlung von Konventionen, die es Ihnen oder Ihren Nutzern ermöglicht, FastComments zu verwenden, ohne ein weiteres Konto erstellen zu müssen.

Vorausgesetzt, Sie erlauben keine anonymen Kommentare, ist ein Konto erforderlich, um mit FastComments zu kommentieren. Wir machen diesen Anmeldeprozess sehr einfach – der Nutzer hinterlässt beim Kommentieren einfach seine E-Mail.
Wir verstehen jedoch, dass selbst das für manche Seiten zusätzliche Reibung bedeutet, die sie vermeiden möchten.

Wir können diese Reibung verringern, indem wir nur einen einzigen Anmeldeablauf für Ihre gesamte Website haben.

### Wie bekomme ich es?
Alle Kontotypen haben derzeit Zugriff auf SSO. Die maximale Anzahl an SSO-Nutzern hängt jedoch von Ihrem Paket ab. Wie bei anderen Funktionen bieten die Pro-Pläne und höher direkten Entwicklungssupport.

Vergleichen wir die Optionen und gehen dann ins Detail zu jeder einzelnen.

### Benutzer- und Kommentar-Migrationen

Bei der Migration von einer Plattform mit SSO wie Disqus haben Sie bereits Benutzer und deren Kommentare.

Kommentare werden im Rahmen Ihrer Migration importiert, entweder über die API, unser Import-UI oder den Kundensupport. Das Import-UI wird bevorzugt, wenn es die Plattform unterstützt, von der Sie migrieren, da es Fehlerbehandlung, Avatar- und Medienextraktion und Uploads sowie ein Batch-Job-Überwachungssystem integriert.

Die Benutzer selbst werden automatisch hinzugefügt, wenn Kommentar-Threads zum ersten Mal angesehen werden. Alternativ können sie vorab über die API hinzugefügt werden, aber diese Arbeit bietet nicht viele Vorteile.

Wenn Kommentare importiert werden und SSO-Benutzer nicht manuell über die API hinzugefügt werden, werden die Kommentare beim ersten Erstellen des Benutzerkontos automatisch dem Benutzerkonto zugewiesen, wenn dieser einen Kommentar-Thread ansieht. Sie können dann die Kommentare bearbeiten, verwalten und löschen, die sie ursprünglich verfasst haben.

Die automatische Migration erfolgt per E-Mail oder Benutzername. Einige Plattformen liefern beim Export keine E-Mails, wie Disqus, daher greifen wir in diesem Fall auf den Benutzernamen zurück.
- Solange Sie einen übereinstimmenden Benutzernamen und eine E-Mail im SSO-Payload übergeben, fügen wir die E-Mail den einzelnen Kommentarobjekten hinzu, damit Benachrichtigungen und Erwähnungen funktionieren.

Wenn gewünscht ist, Kommentare und Benutzer gleichzeitig zu importieren, arbeiten Sie mit dem Support zusammen, um die Kommentare nach dem Import der Benutzer über die API den jeweiligen Benutzerkonten zuzuordnen.

Zusammengefasst ist der einfachste Migrationspfad:

1. Importieren Sie Kommentare.
   1. Avatare und andere Medien werden automatisch migriert, wenn das Import-UI unter `Manage Data -> Imports` verwendet wird.
2. Richten Sie Secure SSO oder Simple SSO ein.
3. Lassen Sie die Migration pro Benutzer automatisch beim ersten Login ablaufen.
   1. Dies fügt der Seitenladezeit normalerweise weniger als eine Sekunde hinzu, wenn der Benutzer weniger als 50k Kommentare hat.

### WordPress-Benutzer
Wenn Sie unser <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress-Plugin</a> verwenden, müssen Sie keinen Code schreiben! Gehen Sie einfach zur Admin-Seite des Plugins, klicken Sie auf SSO-Einstellungen und dann auf Aktivieren.

Dies führt Sie durch einen Ein-Klick-Assistenten, der Ihren API-Schlüssel erstellt, ihn an Ihre WordPress-Installation sendet und SSO aktiviert. Wir haben dies für Sie in einen einzelnen Klick zusammengefasst.

Beachten Sie, dass Sie beim ersten Installieren des Plugins den Einrichtungsprozess abschließen müssen, bevor Sie die Admin-Seite mit der Schaltfläche SSO-Einstellungen sehen.

#### WordPress SSO - Moderatoren

Beachten Sie, dass derzeit das "Moderator"-Abzeichen neben Ihren Moderatoren, wenn diese mit dem FastComments WordPress-Plugin kommentieren, nur angezeigt wird, wenn sie zusätzlich als Moderator im FastComments-Dashboard hinzugefügt wurden und ihre E-Mail verifiziert ist.

### Benutzerdefinierte Integrationen

Für benutzerdefinierte Integrationen gibt es zwei Optionen.

### Option Eins - Secure SSO

Mit Secure SSO weiß FastComments, dass der kommentierende, abstimmende und lesende Nutzer ein echter Nutzer auf Ihrer Seite ist.

Solange Sie eine gültige Payload erzeugen, hat der Nutzer immer ein nahtloses Kommentiererlebnis.

Bei Secure SSO wird die SSO-Payload **serverseitig** unter Verwendung von HMAC-Authentifizierung erzeugt und dann dem Widget auf dem **Client** übergeben.

Bei Secure SSO ist das Benutzerkonto **völlig getrennt** vom übrigen FastComments-Benutzerstamm. Das bedeutet, wenn wir zwei Partner haben,
Firma A und Firma B, kann jeder einen SSO-Benutzer mit dem Benutzernamen "Bob" haben.

#### Voraussetzungen
- Grundlegende Kenntnisse in der Backend-Entwicklung.
- Grundlegende Kenntnisse im Umgang mit geheimen API-Schlüsseln.
- Grundlegende Kenntnisse in API-Entwicklung oder serverseitigem Rendering.

#### Vorteile
- Sicher.
- Nahtloses Kommentiererlebnis.

#### Nachteile
- Erfordert Backend-Entwicklung.

#### Aktualisierung von Benutzerdaten

Bei Secure SSO aktualisieren wir den Benutzer bei jedem Übergeben der SSO-User-Payload mit den neuesten Informationen. Zum Beispiel, wenn
der Benutzer den Benutzernamen `X` hat und Sie in der SSO-Payload `Y` übergeben, wird sein Benutzername zu `Y`.

Wenn Sie mit diesem Ansatz Werte entfernen möchten, setzen Sie sie auf `null` (nicht `undefined`).

#### Secure SSO API

Wir bieten auch eine API zur Interaktion mit den SSO-Benutzern. Siehe [die Dokumentation](/guide-api.html#sso-user-structure).

Beachten Sie, dass bei Verwendung von Secure SSO Benutzer beim Laden der Seite automatisch im Hintergrund erstellt werden. Sie müssen Ihre Benutzer nicht in großen Mengen importieren.

### Option Zwei - Simple SSO

Die Alternative zu Secure SSO besteht darin, dem Kommentar-Widget einfach die Benutzerinformationen zu übergeben.

Bei Simple SSO ist das Angeben einer E-Mail nicht erforderlich, jedoch werden ohne diese ihre Kommentare als "Unverified" angezeigt.

<sup>Hinweis!</sup> Ab Anfang 2022 müssen Benutzernamen bei Simple SSO nicht mehr über alle FastComments.com hinweg eindeutig sein.

Idealerweise sollte Simple SSO nur gewählt werden, wenn Sie auf einer Plattform entwickeln, die keinen Backend-Zugriff bietet.

#### Voraussetzungen
- Grundlegende Kenntnisse in der clientseitigen Entwicklung.
- Kenntnis mindestens der E-Mail des Nutzers.

#### Vorteile
- Einfach.
- Alle Aktivitäten werden weiterhin verifiziert.
- Der Nutzer gibt niemals seinen Benutzernamen oder seine E-Mail ein.

#### Nachteile
- Weniger sicher als Secure SSO, da die Client-seitige Payload manipuliert werden könnte, um beliebig als ein anderer Nutzer aufzutreten.

#### Simple SSO API

Benutzer, die automatisch über den Simple SSO-Fluss erstellt wurden, werden als `SSOUser`-Objekte gespeichert. Sie können über die `SSOUser`-API auf sie zugreifen und sie verwalten. Siehe [die Dokumentation](/guide-api.html#sso-user-structure).