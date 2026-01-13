Damit das Plugin funktioniert, wird ein Token in Ihrer WordPress-Datenbank und auch in Ihrem FastComments-Konto gespeichert. Wenn das Plugin Anfragen an unsere Server stellt, übermittelt es
dieses Token.

Sie können alle Integrationen, die für Ihr FastComments-Konto autorisiert sind, [hier](https://fastcomments.com/auth/my-account/manage-data/integrations) einsehen.

Die gesamte Kommunikation erfolgt über HTTPS.

Alle Kommunikation erfolgt *ausgehend* von Ihrem WordPress-Server *an* FastComments.com, einschließlich der Synchronisierung *zurück* zu Ihrer WordPress-Installation, da sie implementiert
über [Polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) aus einer [Cron](https://developer.wordpress.org/plugins/cron/) Einrichtung in Ihrer WordPress-Installation.