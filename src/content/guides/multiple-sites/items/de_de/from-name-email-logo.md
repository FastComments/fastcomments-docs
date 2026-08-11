Manchmal muss FastComments Ihren Benutzern E‑Mails senden, insbesondere wenn Sie kein Secure SSO verwenden.

Beispiele hierfür sind die Verifizierung ihres Kontos oder ihrer Aktivität, wenn sie zum ersten Mal kommentieren. FastComments
sendet ihnen außerdem Benachrichtigungen über Antworten auf ihre Kommentare.

Wenn FastComments Ihren Benutzern E‑Mails sendet, verwenden wir standardmäßig den Absendernamen und die E‑Mail‑Adresse `FastComments Robot` und `noreply@fastcomments.com`.

Wir verwenden außerdem unser eigenes Logo im Footer dieser E‑Mails.

Wenn Sie FastComments Flex oder Pro besitzen, kann all dies pro Domain über die „My Domains“-Seite angepasst werden:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Per-Domain-E-Mail-Einstellungsformular mit den Feldern Absendername, Absender-E-Mail und Logo-Upload'; title='Anpassen von Absendername, E-Mail und Logo' app-screenshot-end]

Wenn Sie das in E‑Mails angezeigte Logo anpassen, stellen Sie sicher, dass die hochgeladene Größe exakt der Größe entspricht, die Sie im Footer der E‑Mail anzeigen möchten.

### Beim Anpassen der `From Domain`

Wenn Sie die `From Domain` anpassen, müssen E‑Mail‑Provider und -Clients wissen, dass FastComments autorisiert ist, E‑Mails in Ihrem Namen zu versenden. Andernfalls führt das Definieren der `From Domain` ohne Befolgung der nachstehenden Schritte wahrscheinlich dazu, dass E‑Mails im Spam‑Ordner landen.

#### 1. SPF einrichten

Damit FastComments sicher E‑Mails in Ihrem Namen senden kann, fügen Sie einen SPF‑Eintrag hinzu, der uns dies erlaubt.

Stellen Sie sicher, dass SPF‑Einträge vorhanden sind, die `mail.fastcomments.com` und `sib.fastcomments.com` erlauben, E‑Mails in Ihrem Namen zu versenden.

Weitere Informationen dazu finden Sie hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM einrichten

Zusätzlich zu SPF sollten Sie DKIM einrichten. Sobald Ihre DNS‑Konfiguration bereit ist, können Sie auf **„Erweiterte Optionen anzeigen“** auf der Seite für Domain‑Konfigurationen klicken, um die DKIM‑Einstellungen pro Domain anzuzeigen.

Sie können auch die [API aufrufen](/guide-api.html#domain-config-structure), um die DKIM‑Konfiguration festzulegen.

### Abmeldelinks

Bei Verwendung von SSO können die Abmeldefunktionen in E‑Mails und Benachrichtigungen über die [DomainConfigs‑API](/guide-api.html#domain-config-structure) angepasst werden.

### E‑Mail‑Link‑Verschleierung

Wenn die Reputation Ihrer Domain dazu führt, dass Benachrichtigungs‑E‑Mails im Spam‑Ordner landen, können Sie die Schaltflächen „Kommentar anzeigen“ über `fastcomments.com` leiten, anstatt direkt zu Ihrer Seite zu verlinken. Mailbox‑Provider bewerten jeden Link im E‑Mail‑Text anhand der Reputation des Ziels; wenn Ihre Domain markiert ist, erhöhen reine Links den Spam‑Score, unabhängig davon, wie sauber Ihre Versand‑Konfiguration ist.

Aktivieren Sie diese Option unter **„Erweiterte Optionen anzeigen“** auf der My Domains‑Seite im Abschnitt „E‑Mail‑Link‑Verschleierung“. Die Einstellung gilt pro Domain.

Wenn aktiviert, werden Links in Erwähnungen, Antworten, neuen Kommentaren, abonnierten Seiten, Profil‑Kommentaren und Digest‑E‑Mails in kurze Tokens umgewandelt, die beim Klicken zur Originalseite weiterleiten. Das Ziel ist an Ihren Mandanten gebunden: Die Weiterleitung erfolgt nur zu URLs, deren Host zu einer Ihrer konfigurierten Domains gehört, und Tokens verfallen automatisch nach 30 Tagen.

Das Klick‑Erlebnis bleibt unverändert. Leser landen weiterhin auf Ihrer Seite, wobei der Kommentar in den Sichtbereich gescrollt wird.