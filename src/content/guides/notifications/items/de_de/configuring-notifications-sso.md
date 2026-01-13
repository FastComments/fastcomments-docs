Für SSO gibt es die folgende Konfiguration, die für Benachrichtigungen zu berücksichtigen ist:

- Ob der Benutzer sich für Benachrichtigungen angemeldet hat.
  - Dies geschieht durch Setzen des Flags `optedInNotifications` auf `true` oder `false` im `SSOUser`-Objekt.
  - Dies kann über die API gesetzt werden.
  - Wenn Sie außerdem einen Wert für dieses Flag im Payload übergeben, wird es automatisch aktualisiert, wenn der Benutzer einen Kommentar-Thread lädt.
- Ob sich der Benutzer für **subscription**-Benachrichtigungen angemeldet hat.
  - Dies geschieht durch Setzen des Flags `optedInSubscriptionNotifications` auf `true` oder `false` im `SSOUser`-Objekt.
  - Dies kann über die API gesetzt werden.
  - Wenn Sie außerdem einen Wert für dieses Flag im Payload übergeben, wird es automatisch aktualisiert, wenn der Benutzer einen Kommentar-Thread lädt.
- Festlegen ihrer E-Mail-Adresse.
  - Ist diese nicht vorhanden, können wir keine E-Mail-basierten Benachrichtigungen senden.
- Ob Abmelde-Links in E-Mails deaktiviert werden sollen.
  - Dies geschieht über das Flag `disableUnsubscribeLinks` im `Tenant`-Objekt.
  - Dies kann über die API gesetzt werden.
- Ob ein benutzerdefinierter Abmelde-Link verwendet werden soll.
  - Dies geschieht über die Eigenschaft `footerUnsubscribeURL` im `DomainConfig`-Objekt.
  - Dies kann über die API gesetzt werden.
  - Sie sollten auch in Betracht ziehen, die entsprechenden Abmelde-Header über `emailHeaders` im selben Objekt zu setzen.