### Required Components

Für On-Prem besteht FastComments nur aus einem Anwendungsserver und einer Datenbank. Wir haben die Bereitstellung vereinfacht, sodass
der Anwendungsserver den gesamten Traffic direkt bedienen kann, ohne weitere Komponenten hinzuzufügen.

Der Anwendungsserver wird in einem Docker-Image bereitgestellt und kann mit jeder Container-Management-Lösung bereitgestellt werden.

Die Datenbank, MongoDB, kann selbst betrieben oder von einem anderen Anbieter wie AWS DocumentDB oder MongoDB Atlas gehostet werden.

FastComments wird derzeit mit MongoDB 7 getestet, wir streben jedoch Kompatibilität mit DocumentDB an, um die Bereitstellung zu erleichtern.

### Instance Sizes

Sie werden feststellen, dass FastComments ziemlich gut optimiert ist und für den Anwendungsserver selbst keine großen Maschinen benötigt, um niedrige P99s zu erreichen.

Alle Batch- und Cron-Jobs verwenden Streaming, um den gesamten Speicherverbrauch zu begrenzen.

Die untenstehenden Tabellen für den Anwendungsserver und die Datenbank können bei der Größenplanung helfen.

### Application Server Instances


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Zum Beispiel verwendet ein einzelner Kern, der etwa 100 Kommentar-Threads pro Sekunde bedient, normalerweise selten mehr als 250mb RSS.

### Database Server Instances

Die Größe der Datenbank hängt von der Working-Set-Größe ab, also der Menge an Daten, auf die Sie zu einem bestimmten Zeitpunkt zugreifen, sowie von gleichzeitigen Anfragen.

FastComments ist gegenüber Mongo recht freundlich, da es für die heißen Abfragen Index-Hints, Streaming-Cursor verwendet und an verschiedenen Stellen Concurrency-Limits hat,
um eine Überlastung nachgelagerter Systeme zu verhindern.

Die folgende Tabelle ist eine allgemeine Richtlinie für die Größe von Datenbankinstanzen. **Beachten Sie, dass dies __pro Instanz__ gilt, nicht die Gesamtressourcen im Cluster**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Die obigen Tabellen sind konservative Schätzungen. Sie werden feststellen, dass die tatsächlichen Anforderungen je nach Ihrer spezifischen Konfiguration (Seitengrößen, Kommentarvolumen usw.) abweichen können.