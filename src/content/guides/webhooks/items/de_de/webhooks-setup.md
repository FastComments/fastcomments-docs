Führen Sie dieselben Schritte für `localhost` aus wie in der Produktionsumgebung. Stellen Sie sicher, dass Sie Produktionsdomains und API Secrets eingerichtet haben.

Zuerst navigieren Sie zum [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dies ist über Verwalte Daten -> Webhooks erreichbar.

Die Konfigurationsseite sieht wie folgt aus:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

Auf dieser Seite können Sie Endpoints für jeden Typ von Kommentarereignis angeben.

Für jeden Ereignistyp sollten Sie unbedingt auf Send Test Payload klicken, um sicherzustellen, dass Ihre Integration korrekt eingerichtet ist. Siehe den nächsten Abschnitt „Testen“ für Details.

---