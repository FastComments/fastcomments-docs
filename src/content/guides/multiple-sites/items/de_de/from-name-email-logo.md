Manchmal muss FastComments Ihren Nutzern E-Mails schicken, insbesondere wenn Sie kein Secure SSO verwenden.

Beispiele hierfür sind die Verifizierung ihres Kontos oder ihrer Aktivität, wenn sie zum ersten Mal kommentieren. FastComments
sendet ihnen außerdem Benachrichtigungen zu Antworten auf ihre Kommentare.

Wenn FastComments Ihren Nutzern E-Mails sendet, verwenden wir standardmäßig als Absendernamen `FastComments Robot` und als Absender-E-Mail `noreply@fastcomments.com`.

Wir verwenden außerdem unser eigenes Logo in der Fußzeile dieser E-Mails.

Wenn Sie FastComments Flex oder Pro haben, kann dies pro Domain auf der Seite "Meine Domains" angepasst werden:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Anpassen von Absendername, E-Mail und Logo' app-screenshot-end]

Beim Anpassen des Logos, das in E-Mails angezeigt wird, stellen Sie sicher, dass die Größe, die Sie hochladen, der Größe entspricht, die Sie in der Fußzeile der E-Mail anzeigen möchten.

### Beim Anpassen der `From Domain`

Wenn Sie die `From Domain` anpassen, müssen E-Mail-Anbieter und -Clients wissen, dass FastComments berechtigt ist, E-Mails in Ihrem Namen zu versenden. Andernfalls führt das Festlegen der `From Domain` ohne Befolgung der nachstehenden Schritte sehr wahrscheinlich dazu, dass E-Mails im Spam landen.

#### 1. SPF einrichten

Damit FastComments sicher E-Mails im Namen Ihrer Domain senden kann, fügen Sie einen SPF-Eintrag hinzu, der dies erlaubt.

Stellen Sie sicher, dass SPF-Einträge vorhanden sind, die es `mail.fastcomments.com` und `sib.fastcomments.com` erlauben, E-Mails im Namen Ihrer Domain zu versenden.

Weitere Informationen dazu finden Sie hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM einrichten

Zusätzlich zu SPF sollten Sie DKIM einrichten. Sobald Ihre DNS-Konfiguration bereit ist, können Sie auf der Seite mit den Domain-Einstellungen auf "Erweiterte Optionen anzeigen" klicken, um die DKIM-Einstellungen pro Domain anzuzeigen.

Sie können auch [die API aufrufen](/guide-api.html#domain-config-structure), um die DKIM-Konfiguration festzulegen.

### Abmelde-Links

Wenn Sie SSO verwenden, können die in E-Mails und Benachrichtigungen verwendeten Abmeldefunktionen [über die DomainConfigs API](/guide-api.html#domain-config-structure) angepasst werden.