---
Befolgen Sie dieselben Schritte für `localhost` wie für die Produktion. Stellen Sie sicher, dass Sie Produktionsdomains und API‑Secrets eingerichtet haben.

Navigieren Sie zunächst zum [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dieser ist über Daten verwalten → Webhooks erreichbar.

Die Konfigurationsseite erscheint wie folgt:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks‑Admin‑Seite mit einem Domänenselektor und einem Endpunkt‑URL‑Feld pro Kommentarereignis, plus Send Test Payload'; title='Webhooks‑Konfiguration'; cacheBuster = 'v3' app-screenshot-end]

Auf dieser Seite können Sie Endpunkte für jede Art von Kommentarereignis festlegen.

Für jede Art von Ereignis sollten Sie unbedingt auf Send Test Payload klicken, um sicherzustellen, dass Sie Ihre Integration korrekt eingerichtet haben. Siehe den nächsten Abschnitt "Testing" für Details.