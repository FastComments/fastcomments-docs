Für die lokale Entwicklung verwenden Sie ein Tool wie [ngrok](https://ngrok.com/).

Um die Systemsicherheit zu vereinfachen, folgt die lokale Entwicklung dem gleichen Prozess wie das Einrichten und Absichern anderer Umgebungen. 

### Schritt 1: Fügen Sie "localhost" zu den Domains in Ihrem Konto hinzu.

Fügen Sie "localhost" [als Domain hier](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Schritt 2: Wählen Sie einen API-Schlüssel

Wir werden die Webhook-Konfiguration für Ihre Domain hinzufügen, daher benötigen wir einen API-Schlüssel. [Das können Sie hier tun.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Unter "Associate with domain" - wählen Sie Ihre "localhost" domain.

**HINWEIS: Alternativ können Sie ein API-Secret für alle Testaktivitäten und Staging-Umgebungen verwenden. Fügen Sie einfach ein API-Secret für "All Domains" hinzu und geben Sie ihm einen Namen wie "test".**

Stellen Sie sicher, dass Sie ein API-Secret für Ihre Produktionsdomain(s) definiert haben. Ereignisse für alle anderen Domains verwenden das Wildcard-(Test-)Secret.

### Schritt 3: Fügen Sie Ihren Webhook hinzu

Während Sie ngrok oder ein ähnliches Tool ausführen, setzen Sie den Wert für "localhost" [hier](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Sobald es validiert ist, klicken Sie auf `Save`.

### Schritt 4: Fügen Sie einen Kommentar hinzu

Jetzt können Sie Kommentare hinzufügen, bearbeiten oder löschen und sollten sehen, dass wir Ihre lokale Entwicklungsmaschine mit den Ereignissen aufrufen, wobei Ihr Test-API-Schlüssel verwendet wird. Es kann bis zu 30 Sekunden Verzögerung geben
bis die Ereignisse Ihre Maschine erreichen.

---