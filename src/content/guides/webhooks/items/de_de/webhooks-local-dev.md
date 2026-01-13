Für die lokale Entwicklung verwenden Sie ein Tool wie [ngrok](https://ngrok.com/).

Um die Sicherheit des Systems zu vereinfachen, folgt die lokale Entwicklung demselben Prozess wie das Einrichten und Sichern anderer Umgebungen. 

### Step 1: Add "localhost" to domains in your account.

Fügen Sie "localhost" [als Domain hier hinzu](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Wir werden die Webhook-Konfiguration für Ihre Domain hinzufügen, daher benötigen wir einen API Key. [Das können Sie hier tun.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Unter "Associate with domain" - wählen Sie Ihre "localhost"-Domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Stellen Sie sicher, dass Sie ein API Secret für Ihre Produktionsdomain(s) definiert haben. Ereignisse für alle anderen Domains werden das Wildcard-(Testing-)Secret verwenden.

### Step 3: Add Your Webhook

Während ngrok oder ein ähnliches Tool läuft, setzen Sie den Wert für "localhost" [hier](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Beim Klicken auf `Send Test Payload` senden wir zwei Testereignisse, um zu überprüfen, dass Sie den API Key validieren.

Sobald dies validiert ist, klicken Sie auf `Save`.

### Step 4: Add A Comment

Jetzt können Sie Kommentare hinzufügen, bearbeiten oder löschen und sollten sehen, wie wir Ihre lokale Entwicklungsmaschine mit den Ereignissen aufrufen, wobei Ihr Test-API Key verwendet wird. Es kann eine Verzögerung von bis zu 30 Sekunden geben, bis die Ereignisse Ihre Maschine erreichen.