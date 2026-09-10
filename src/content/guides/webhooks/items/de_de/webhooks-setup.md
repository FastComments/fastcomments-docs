---
Folgen Sie den gleichen Schritten für `localhost` wie für die Produktion. Stellen Sie sicher, dass Sie Produktionsdomains und API Secrets eingerichtet haben.

Navigieren Sie zuerst zum [Webhooks‑Admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dieser ist über Manage Data → Webhooks erreichbar.

Die Seite listet alle Webhooks in Ihrem Konto auf:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks-Admin-Seite, die jeden Webhook mit seiner URL, dem Ereignis, der Domain, der Methode, dem Status und der Anzahl der in der Warteschlange befindlichen Ereignisse auflistet'; title='Webhooks-Liste'; cacheBuster = 'v4' app-screenshot-end]

Klicken Sie auf **New Webhook**, um einen hinzuzufügen. Jeder Webhook hat eine URL, ein Kommentarereignis (erstellt, aktualisiert oder gelöscht), eine Domain und eine HTTP‑Methode:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Neues Webhook-Formular mit Feldern für URL, Ereignis, Domain und HTTP‑Methode sowie „Send Test Payload“'; title='Neuer Webhook'; cacheBuster = 'v4' app-screenshot-end]

Jeder Webhook wird unabhängig ausgeliefert. Sie können dasselbe Ereignis an mehrere Endpunkte senden, und ein auf **All Domains** beschränkter Webhook erhält Kommentare von jeder Domain, selbst wenn ein domänenspezifischer Webhook für dasselbe Ereignis existiert. Die gleiche URL, das gleiche Ereignis und die gleiche Domain können nicht zweimal hinzugefügt werden.

Klicken Sie vor dem Speichern auf **Send Test Payload**, um zu prüfen, ob der Endpunkt eine signierte Anfrage akzeptiert. Siehe den nächsten Abschnitt "Testing" für Details.

Aus der Liste können Sie einen Webhook bearbeiten, deaktivieren, wieder aktivieren oder löschen. Das Deaktivieren behält wartende Ereignisse bei, bis der Webhook wieder aktiviert wird; das Löschen verwirft sie.

Webhooks können auch über die API erstellt werden, zum Beispiel durch Zapier. Diese erscheinen in derselben Liste mit der Quelle **API**. Siehe Managing Webhooks via the API.
---