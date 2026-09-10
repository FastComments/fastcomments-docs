---
FastComments unterstützt Webhooks nur für die Comment‑Ressource.

Wir unterstützen Webhooks für das Erstellen, Entfernen und Aktualisieren von Kommentaren.

Jedes dieser Ereignisse wird in unserem System als separates Event betrachtet und hat daher unterschiedliche Semantiken
und Strukturen für die Webhook‑Events.

Eine beliebige Anzahl von Endpunkten kann dasselbe Event abonnieren, über das Dashboard oder über die API
(siehe Verwalten von Webhooks über die API). Jeder Webhook wird unabhängig zugestellt.

---