Für die lokale Entwicklung verwenden Sie ein Tool wie [ngrok](https://ngrok.com/).

Um die Sicherheit des Systems zu vereinfachen, folgt die lokale Entwicklung dem gleichen Prozess wie das Einrichten und Sichern anderer Umgebungen. 

### Schritt 1: Fügen Sie „localhost“ zu den Domains in Ihrem Konto hinzu.

Fügen Sie „localhost“ [hier als Domain hinzu](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Das Formular zum Hinzufügen einer Domain in den Kontoeinstellungen mit localhost im Feld für Domainnamen'; title='localhost hinzufügen'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Schritt 2: Wählen Sie einen API-Schlüssel

Wir werden eine Webhook-Konfiguration für Ihre Domain hinzufügen, daher benötigen wir einen API-Schlüssel. [Sie können das hier tun.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Neues API-Geheimnis-Formular mit der zugehörigen Domain auf localhost gesetzt und dem Schlüsselnamen Testing'; title='Testing API-Schlüssel hinzufügen'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Unter „Associate with domain“ – wählen Sie Ihre „localhost“-Domain aus.

**HINWEIS: Alternativ können Sie ein API-Geheimnis für alle Testaktivitäten und Staging-Umgebungen verwenden. Fügen Sie einfach ein API-Geheimnis für „All Domains“ hinzu und geben Sie ihm einen Namen wie „test“.**

Stellen Sie sicher, dass Sie ein API-Geheimnis für Ihre Produktionsdomain(s) definiert haben. Ereignisse für alle anderen Domains verwenden das Wildcard-(Test‑)Geheimnis.

### Schritt 3: Fügen Sie Ihren Webhook hinzu

Während Sie ngrok oder ein ähnliches Tool ausführen, setzen Sie den Wert für „localhost“ [hier](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhook-Admin mit ausgewählter localhost-Domain und einer ngrok-URL, die in den Endpunkt für erstellte Kommentare eingetragen ist'; title='Testing Webhook hinzufügen'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Wenn Sie `Send Test Payload` anklicken, senden wir zwei Testereignisse, um zu prüfen, ob Sie den API-Schlüssel validieren.

Sobald es validiert ist, klicken Sie auf `Save`.

### Schritt 4: Einen Kommentar hinzufügen

Jetzt können Sie Kommentare hinzufügen, bearbeiten oder löschen und sollten sehen, dass wir Ihre lokale Entwicklungsmaschine mit den Ereignissen aufrufen, wobei Sie Ihren Test‑API‑Schlüssel verwenden. Es kann bis zu 30 Sekunden dauern, bis die Ereignisse Ihre Maschine erreichen.

---