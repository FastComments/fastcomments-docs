Da die Anmelde-Links im Grunde Passwörter sind, nehmen wir die Sicherheit sehr ernst.

Alle Anmelde-Links in unserem System sind so eingestellt, dass sie nach einer bestimmten Zeitspanne ablaufen, und wir haben außerdem Mechanismen implementiert, um das Erraten eines Anmelde-Links zu erkennen.
Manche Anmelde-Links sind in mehrere Passwörter aufgeteilt, und wenn eines davon erraten wird,
wird das andere ungültig.

### Sicherheit im Vergleich zu Passwörtern

Bei den meisten Systemen, die ein Passwort erfordern, kann man über einen "Passwort vergessen"-Mechanismus gehen
wenn man die E-Mail-Adresse des Nutzers hat. Das bedeutet, wenn Sie Zugriff auf das E-Mail-Konto des Nutzers haben,
spielt es keine Rolle, ob das angegriffene System Passwörter oder Magic Links verwendet.

### Neue IP-Anmeldebenachrichtigungen

Wenn eine Anmeldung von einer IP-Adresse erfolgt, die für ein bestimmtes Konto bisher nicht gesehen wurde, sendet FastComments eine Sicherheitsbenachrichtigung per E-Mail
mit dem ungefähren Standort und der IP-Adresse. Dies hilft Nutzern, unautorisierten Zugriff zu erkennen. Beachten Sie, dass FastComments nicht
unveränderte IP-Adressen speichert — es wird aus Sicherheitsgründen nur eine verschleierte Form gespeichert.

### Backup-E-Mail zur Kontowiederherstellung

Wenn Sie den Zugriff auf Ihre primäre E-Mail verlieren, können Sie eine verifizierte Backup-E-Mail verwenden, um Ihr Konto wiederherzustellen. Ihre Backup-E-Mail funktioniert
mit allen Anmeldeflüssen. Sie können sie auf der Seite "Benutzername vergessen" eingeben, sie beim Login per Magic Link verwenden oder sie in das
Benutzername/E-Mail-Feld für die Passwort-Anmeldung eingeben.

Um eine Backup-E-Mail einzurichten, gehen Sie zu [Kontodetails](https://fastcomments.com/auth/my-account/edit-details) und klicken Sie
**Backup-E-Mail definieren**. Ihre Backup-E-Mail wird nur zur Kontowiederherstellung verwendet und erhält keine Benachrichtigungen.

### Sicherheit im Vergleich zu MFA

Anmelde-Links sind weniger sicher als MFA. FastComments unterstützt jetzt die Zwei-Faktor-Authentifizierung (2FA)
für Administratorkonten, um erhöhte Sicherheit zu bieten. Wenn 2FA aktiviert ist, ist sie auch bei Verwendung von Anmelde-Links erforderlich.