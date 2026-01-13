---
Führen Sie für `localhost` dieselben Schritte aus wie für die Produktionsumgebung. Stellen Sie sicher, dass Sie Produktionsdomains und API Secrets eingerichtet haben.

Zuerst navigieren Sie zum [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dies ist über Manage Data -> Webhooks zugänglich.

Die Konfigurationsseite sieht wie folgt aus:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

Auf dieser Seite können Sie Endpunkte für jede Art von Kommentarereignis angeben.

Für jede Ereignisart klicken Sie unbedingt auf Send Test Payload, um sicherzustellen, dass Sie Ihre Integration korrekt eingerichtet haben. Siehe den nächsten Abschnitt, "Testing", für Details.

---