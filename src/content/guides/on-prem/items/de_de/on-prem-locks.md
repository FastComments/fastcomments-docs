---
Wie jedes verteilte System benötigt FastComments eine Möglichkeit, Ressourcen und Abläufe zu sperren. Diese Sperren können über den Endpunkt `/locks-in-progress` überwacht werden.

[Zum Beispiel ist hier der Endpunkt auf unserem US-Shard](https://fastcomments.com/locks-in-progress).

Dies kann nützlich sein, um zu sehen, warum das System festhängt oder unter Last steht. Wenn beispielsweise ein SRE herausfinden möchte, warum das System eine hohe CPU-Auslastung hat, könnte man diesen Endpunkt abfragen, um den Namen des fehlverhaltenden Cron-Jobs zu erhalten.

---