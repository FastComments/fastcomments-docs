---
Standardmäßig verfügt FastComments über eine trainierbare Spam-Erkennung.

Wenn Sie Kommentare moderieren und sie als **Spam** markieren, oder automatisch als **Spam** erkannte Kommentare als **Nicht Spam** markieren, lernt das Spam-Erkennungssystem aus diesen Aktionen, um genauer zu bestimmen, was Sie als Spam betrachten.

Als **Spam** markierte Kommentare werden nicht automatisch genehmigt und erscheinen daher nicht, bis sie ausdrücklich als **Nicht Spam** markiert werden.

Die Spam-Erkennung kann über die Seite mit den Einstellungen zur Kommentar-Moderation deaktiviert werden.

### Verschiedene Spam-Detektoren

FastComments unterstützt drei Möglichkeiten zur Spam-Erkennung:

1. Ein traditioneller Naïve-Bayes-Klassifikator, der kontinuierlich trainiert wird und der über alle FastComments.com tenants hinweg gemeinsam genutzt wird.
2. Ein traditioneller Naïve-Bayes-Klassifikator, der kontinuierlich trainiert wird und für Ihren tenant **isoliert** ist.
3. Verwendung von ChatGPT 4.

Jeder hat Zugriff auf die gemeinsam genutzten und isolierten Naïve-Bayes-Klassifikatoren.

Die ChatGPT 4-Option kann auf der Seite mit den Einstellungen zur Kommentar-Moderation ausgewählt werden, wenn Sie auf Flex billing sind, da sie nach verwendeten Tokens abgerechnet wird.

### Vertrauensfaktor

FastComments passt den Spam-Filter für einen Benutzer daran an, wie vertrauenswürdig er für die jeweilige Site ist.

Wenn Administratoren beispielsweise viele ihrer Kommentare angepinnt haben, ist der Nutzer wahrscheinlich sehr vertrauenswürdig. Oder wenn er schon lange Mitglied der Site ist und viele Kommentare hat, kann auch sein Vertrauensfaktor hoch sein.

### SSO

Von SSO-Nutzern verfasste Kommentare können als Spam betrachtet und entsprechend überprüft werden. Die Ausnahme ist, wenn der SSO-Nutzer dieselbe E-Mail-Adresse hat wie ein Tenant-Benutzer, der eine oder mehrere der folgenden Berechtigungen besitzt:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO-Nutzer mit diesen Berechtigungen werden nicht auf Spam überprüft.

### Wiederholte Nachrichten

FastComments erkennt und verhindert wiederholte Nachrichten. Es erkennt außerdem wiederholte Nachrichten, die sehr ähnlich sind, um Spam zu verhindern. Dies kann nicht deaktiviert werden, da es verhindert, dass unsere Plattform für Missbrauch verwendet wird. Wenn Sie einen hohen Vertrauensfaktor haben, wird dies bei der Verhinderung wiederholter Nachrichten berücksichtigt.

---