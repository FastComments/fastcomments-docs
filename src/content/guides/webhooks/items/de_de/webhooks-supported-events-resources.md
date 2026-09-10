FastComments unterstützt Webhooks nur für die Kommentar‑Ressource.

Wir unterstützen Webhooks für das Erstellen, Entfernen und Aktualisieren von Kommentaren.

Jeder dieser Vorgänge wird in unserem System als separates Ereignis betrachtet und hat daher unterschiedliche Semantiken
und Strukturen für die Webhook‑Ereignisse.

Eine beliebige Anzahl von Endpunkten kann dasselbe Ereignis abonnieren: ein Webhook pro Domain kann im
Dashboard konfiguriert werden, und weitere Abonnements können über die API erstellt werden (siehe Verwalten von Webhooks über die API).