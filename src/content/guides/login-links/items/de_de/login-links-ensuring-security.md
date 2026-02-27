---
Da die Login-Links im Grunde Passwörter sind, nehmen wir die Sicherheit sehr ernst.

All login links in our system are set to expire after a certain period of time, and we also have mechanisms in place to detect
the guessing of a login link. Some login links are split into multiple passwords, and if one is guessed,
the other will be invalidated.

### Sicherheit im Vergleich zu Passwörtern

Bei den meisten Systemen, die ein Passwort erfordern, können Sie den 'Passwort vergessen'-Mechanismus
nutzen, wenn Sie die E-Mail-Adresse des Benutzers haben. Das bedeutet, wenn Sie Zugriff auf das E-Mail-Konto des Benutzers haben,
spielt es keine Rolle, ob das angegriffene System Passwörter oder Magic Links verwendet.

### Neue IP-Anmeldebenachrichtigungen

When a login occurs from an IP address that has not been seen before for a given account, FastComments sends a security alert email
with the approximate location and IP address. This helps users detect unauthorized access. Note that FastComments does not store
raw IP addresses — only an obfuscated form is stored for security purposes.

### Sicherheit im Vergleich zu MFA

Login-Links sind weniger sicher als MFA. FastComments unterstützt jetzt die Zwei-Faktor-Authentifizierung (2FA)
für Administratorkonten, um die Sicherheit zu erhöhen. Wenn 2FA aktiviert ist, ist sie auch bei der Verwendung von Login-Links erforderlich.

---