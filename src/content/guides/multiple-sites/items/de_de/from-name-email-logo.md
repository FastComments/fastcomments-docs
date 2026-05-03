Manchmal muss FastComments E-Mails an Ihre Nutzer senden, insbesondere wenn Sie kein Secure SSO verwenden.

Beispiele dafür sind das Verifizieren ihres Kontos oder ihrer Aktivität, wenn sie zum ersten Mal einen Kommentar abgeben. FastComments
wird ihnen auch Benachrichtigungen über Antworten auf ihre Kommentare schicken.

Wenn FastComments Ihren Nutzern E-Mails sendet, verwenden wir standardmäßig den Absendernamen und die E-Mail `FastComments Robot` und `noreply@fastcomments.com`.

Wir verwenden außerdem unser eigenes Logo in der Fußzeile dieser E-Mails.

Wenn Sie FastComments Flex oder Pro haben, kann dies alles pro Domain über die "Meine Domains"-Seite angepasst werden:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Beim Anpassen des in E-Mails angezeigten Logos stellen Sie sicher, dass die hochgeladene Größe der Größe entspricht, die Sie in der Fußzeile der E-Mail anzeigen möchten.

### Beim Anpassen der `From Domain`

Wenn Sie die `From Domain` anpassen, müssen E-Mail-Anbieter und -Clients wissen, dass FastComments berechtigt ist, im Namen Ihrer Domain E-Mails zu senden. Andernfalls
führt das Festlegen der `From Domain` ohne Befolgung der nachstehenden Schritte wahrscheinlich dazu, dass E-Mails im Spam landen.

#### 1. SPF einrichten

Damit FastComments sicher E-Mails im Namen Ihrer Domain senden kann, fügen Sie einen SPF-Eintrag hinzu, der uns dies erlaubt.

Stellen Sie sicher, dass SPF-Einträge vorhanden sind, die erlauben, dass `mail.fastcomments.com` und `sib.fastcomments.com` E-Mails im Namen Ihrer Domain senden.

Weitere Informationen dazu finden Sie hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM einrichten

Zusätzlich zu SPF sollten Sie DKIM einrichten. Sobald Ihre DNS-Konfiguration bereit ist, können Sie auf der Domain-Konfigurationsseite auf "Show Advanced" klicken,
um die DKIM-Einstellungen pro Domain anzuzeigen.

Sie können auch die [API aufrufen](/guide-api.html#domain-config-structure), um die DKIM-Konfiguration festzulegen.

### Abmelde-Links

Wenn SSO verwendet wird, können die in E-Mails und Benachrichtigungen verwendeten Abmeldefunktionen [über die DomainConfigs API](/guide-api.html#domain-config-structure) angepasst werden.

### E-Mail-Link-Verschleierung

Wenn die Reputation der Domain Ihrer Website dazu führt, dass Benachrichtigungs-E-Mails im Spam landen, können Sie die Schaltflächen "view comment" über `fastcomments.com` umleiten, anstatt direkt auf Ihre Seite zu verlinken. Mailanbieter bewerten jeden Link im E-Mail-Text anhand der Reputation des Ziels, sodass nackte Links zur Spam-Bewertung beitragen, wenn Ihre Domain markiert wird, unabhängig davon, wie sauber Ihre Versandkonfiguration ist.

Aktivieren Sie dies unter "Show Advanced" auf der Seite "Meine Domains" im Abschnitt "Email Link Obfuscation". Die Einstellung ist pro Domain.

Wenn aktiviert, werden Links in Erwähnungs-, Antwort-, Neue-Kommentar-, abonnierte-Seite-, Profilkommentar- und Digest-E-Mails in kurze Tokens umgeschrieben, die beim Anklicken auf die ursprüngliche Seite weiterleiten. Das Ziel ist an Ihren Tenant gebunden: Der Redirect leitet nur an URLs weiter, deren Host mit einer Ihrer konfigurierten Domains übereinstimmt, und Tokens laufen automatisch nach 30 Tagen ab.

Das Erlebnis nach dem Anklicken bleibt unverändert. Leser landen weiterhin auf Ihrer Seite, wobei der Kommentar in die Ansicht gescrollt wird.