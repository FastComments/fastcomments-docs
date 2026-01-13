FastComments unterstützt einen automatischen Wartungsmodus. Wenn die Datenbank ausfällt, kann es weiterhin beliebte Kommentar-Threads ausliefern.

Zusätzlich werden im Wartungsmodus alle Kommentare in `BACKUP_DIR` gespeichert. Sie werden verarbeitet (auf Spam geprüft usw.) und gespeichert, sobald das System wieder online ist.

Das geschieht, indem es jede Stunde die 100 beliebtesten Kommentar-Threads bestimmt und deren Inhalte auf der Festplatte zwischenspeichert. Die Bestimmung der 100 beliebtesten Threads
wird bereits aus einem vorab berechneten Zustand vorgenommen, daher ist es kein aufwändiger periodischer Job.

Das ist völlig optional und wird nur aktiviert, wenn `CACHE_DIR` und `BACKUP_DIR` gesetzt sind. Dadurch werden die Anwendungs-Knoten natürlich zustandsbehaftet, jedoch handelt es sich um einen Zustand, der
jederzeit verloren gehen kann, ohne dass die Anwendung Fehlverhalten zeigt.

Beachte, dass im Wartungsmodus eine ordnungsgemäße Authentifizierung von Kommentar-Threads nicht möglich ist, daher werden nur Threads, die sicher als öffentlich gelten, periodisch gesichert.

Im Wartungsmodus sind viele Funktionen nicht verfügbar.