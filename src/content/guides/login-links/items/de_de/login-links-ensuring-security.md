Da Anmeldelinks im Wesentlichen Passwörter sind, nehmen wir die Sicherheit sehr ernst.

Alle Anmeldelinks in unserem System sind so eingestellt, dass sie nach einer bestimmten Zeit ablaufen, und wir haben auch Mechanismen implementiert, um
das Erraten eines Anmeldelinks zu erkennen. Einige Anmeldelinks sind in mehrere Passwörter aufgeteilt, und wenn eines erraten wird,
wird das andere ungültig.

### Sicherheit im Vergleich zu Passwörtern

Bei den meisten Systemen, die ein Passwort erfordern, kann man den Passwort-vergessen-Mechanismus durchlaufen,
wenn man die E-Mail des Benutzers hat. Das bedeutet, wenn Sie Zugriff auf das E-Mail-Konto des Benutzers haben,
spielt es keine Rolle, ob das angegriffene System Passwörter oder magische Links verwendet.

### Sicherheit im Vergleich zu MFA

Anmeldelinks sind weniger sicher als MFA. FastComments unterstützt jetzt die Zwei-Faktor-Authentifizierung (2FA)
für Admin-Konten, um die Sicherheit zu erhöhen. Wenn 2FA aktiviert ist, ist sie auch bei Verwendung von Anmeldelinks erforderlich.