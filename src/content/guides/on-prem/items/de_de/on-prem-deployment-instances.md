### Erforderliche Komponenten

Für On-Prem besteht FastComments lediglich aus einem Anwendungsserver und einer Datenbank. Wir haben die Bereitstellung vereinfacht, sodass
die Anwendung den gesamten Datenverkehr direkt bedienen kann, ohne dass weitere Komponenten hinzugefügt werden müssen.

Der Anwendungsserver wird als Docker-Image bereitgestellt und kann mit jeder Container-Management-Lösung deployt werden.

Die Datenbank, MongoDB, kann selbst betrieben oder von einem anderen Anbieter wie AWS DocumentDB oder MongoDB Atlas gehostet werden.

FastComments wird derzeit mit MongoDB 7 getestet, wir streben jedoch DocumentDB-Kompatibilität an, um die Bereitstellung zu erleichtern.

### Instanzgrößen

Sie werden feststellen, dass FastComments recht gut optimiert ist und für die Anwendung selbst keine großen Maschinen benötigt, um niedrige P99-Werte zu halten.

Alle Batch- und Cron-Jobs verwenden Streaming, um den gesamten Speicherverbrauch zu begrenzen.

Die folgenden Tabellen für den Anwendungsserver und die Datenbank können bei der Größenbestimmung helfen.

### Anwendungsserver-Instanzen


| Gleichzeitige Benutzer | CPUs des gesamten Clusters | Gesamtspeicher des Clusters |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Zum Beispiel verwendet ein einzelner Kern, der etwa 100 Kommentar-Threads pro Sekunde bedient, normalerweise nie mehr als 250mb RSS.

### Datenbankserver-Instanzen

Die Größenbestimmung der Datenbank hängt von der Größe des Working Sets ab, also der Datenmenge, auf die zu einem bestimmten Zeitpunkt zugegriffen wird, sowie von gleichzeitigen Anfragen.

FastComments geht schonend mit Mongo um, da es für die häufigen Abfragen Index-Hinweise und Streaming-Cursors verwendet und in verschiedenen Bereichen Concurrency-Limits hat,
um eine Überlastung nachgelagerter Systeme zu verhindern.

Das Folgende ist eine allgemeine Richtlinie für die Größen von Datenbankinstanzen. **Beachten Sie, dass dies __pro Instanz__ gilt, nicht die Gesamtressourcen im Cluster**.

| Gleichzeitige Benutzer | Gespeicherte Kommentare | CPUs pro Instanz | Speicher pro Instanz |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Die obigen Tabellen sind konservative Schätzungen. Die tatsächlichen Anforderungen können je nach Ihrer spezifischen Konfiguration abweichen (Seitengrößen, Kommentaraufkommen usw.).