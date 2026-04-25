FastComments integriert sich in Drupals Benutzersystem über SSO, also Single Sign-On. Ihre Nutzer melden sich auf Ihrer Drupal-Seite an, und das Modul übermittelt deren Identität automatisch an FastComments. Keine zusätzlichen Konten zu erstellen, kein anfänglicher Sync erforderlich.

Das Modul unterstützt drei SSO-Modi, die unter `Administration > Configuration > Content > FastComments` eingestellt werden.

### Keine

Kein SSO. Nutzer kommentieren als Gäste oder erstellen ein FastComments-Konto. Verwenden Sie dies, wenn Ihre Seite öffentlich ist und Sie Kommentare nicht an Drupal-Benutzer binden müssen.

### Einfach

Übermittelt den Drupal-Benutzernamen, die E-Mail und das Avatar an FastComments ohne serverseitige Verifizierung. Kein API Secret nötig. Gut für interne oder gering riskante Seiten.

### Sicher (empfohlen)

Verwendet [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC), um jede Benutzeridentität gegenüber FastComments zu verifizieren. Dies ist der Modus, den Sie verwenden sollten, wenn ein API Secret konfiguriert ist, und er ist der einzige Modus, der verhindert, dass ein Besucher einen anderen Nutzer vortäuscht.

Die Benutzeridentität wird an FastComments übermittelt, jedes Mal, wenn ein Nutzer einen Kommentarthread ansieht. Es gibt keinen anfänglichen oder kontinuierlichen Sync, der ausgeführt werden muss.

<sup>(Optional)</sup> Fügen Sie Ihre Administratoren zu [Benutzer & Administratoren](https://fastcomments.com/auth/my-account/users) und Moderatoren zu [Kommentarmoderatoren](https://fastcomments.com/auth/my-account/moderate-comments/moderators) hinzu, um deren Experience zu verbessern und die Statistikverfolgung für Moderatoren zu aktivieren.

Für einen tieferen Einblick, wie SSO funktioniert, siehe den [SSO-Abschnitt](/guide-customizations-and-configuration.html#sso) der Anpassungsdokumentation.

---