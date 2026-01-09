### Echtzeit-Aktualisierungen

Collab Chat verwendet WebSocket-Verbindungen, um alle Unterhaltungen in Echtzeit über alle verbundenen Benutzer hinweg zu synchronisieren. Wenn jemand eine neue Annotation erstellt, einen Kommentar hinzufügt oder eine Diskussion löscht, sehen alle anderen Benutzer, die dieselbe Seite betrachten, die Aktualisierung sofort, ohne die Seite neu zu laden.

### Wie die WebSocket-Synchronisierung funktioniert

Wenn Sie Collab Chat initialisieren, stellt das Widget eine WebSocket-Verbindung zu den FastComments-Servern her. Diese Verbindung bleibt für die Dauer der Sitzung des Benutzers offen und lauscht auf Updates, die sich auf die aktuelle Seite beziehen.

Das WebSocket-System verwendet drei Arten von Broadcast-Nachrichten für Collab Chat. Das `new-text-chat`-Ereignis wird ausgelöst, wenn jemand eine neue Annotation auf der Seite erstellt. Das `updated-text-chat`-Ereignis wird ausgelöst, wenn jemand eine bestehende Unterhaltung aktualisiert. Das `deleted-text-chat`-Ereignis wird ausgelöst, wenn jemand eine Annotation löscht.

### Broadcast-ID-System

Um Echo-Effekte zu verhindern, bei denen Benutzer ihre eigenen Aktionen zurückgesendet sehen, enthält jede Aktualisierung eine eindeutige `broadcastId`. Wenn ein Benutzer eine Annotation erstellt oder aktualisiert, generiert sein Client für diese Operation eine UUID. Wenn der WebSocket die Aktualisierung an alle Clients zurücksendet, ignoriert der ursächliche Client die Aktualisierung, weil sie mit seiner eigenen `broadcastId` übereinstimmt.

Das stellt eine reibungslose Interaktion sicher, bei der Benutzer ihre Änderungen sofort in der UI sehen, ohne auf den Round-Trip über den Server warten zu müssen, während gleichzeitig gewährleistet ist, dass alle anderen Benutzer die Aktualisierung erhalten.

### Live-Benutzeranzahl

Die obere Leiste zeigt die Anzahl der Benutzer an, die die Seite derzeit betrachten. Diese Zahl aktualisiert sich in Echtzeit, wenn Benutzer beitreten oder die Seite verlassen. Die Benutzeranzahl wird über dieselbe WebSocket-Verbindung bereitgestellt und erhöht bzw. verringert sich automatisch basierend auf Verbindungs- und Trennungsereignissen.

### Verbindungsresilienz

Fällt die WebSocket-Verbindung aufgrund von Netzwerkproblemen oder Serverwartung aus, versucht das Widget automatisch, die Verbindung wiederherzustellen. Während der Wiederverbindungsphase können Benutzer weiterhin mit bestehenden Annotationen interagieren, sehen jedoch keine Echtzeit-Aktualisierungen von anderen Benutzern, bis die Verbindung wiederhergestellt ist.

Sobald die Verbindung wiederhergestellt ist, synchronisiert das Widget erneut, um sicherzustellen, dass keine Aktualisierungen verpasst wurden. Dies geschieht transparent, ohne dass eine Aktion des Benutzers erforderlich ist.

### Bandbreitenüberlegungen

WebSocket-Nachrichten sind leichtgewichtig und enthalten nur die wesentlichen Informationen, die zur Zustands-Synchronisation benötigt werden. Das Erstellen einer neuen Annotation verbraucht typischerweise weniger als 1 KB Bandbreite. Das System beinhaltet außerdem intelligente Bündelung, um die Nachrichtenfrequenz in Phasen hoher Aktivität zu reduzieren.

Ihre Nutzungsmetriken im FastComments-Dashboard verfolgen `pubSubMessageCount` und `pubSubBandwidth`, sodass Sie die Echtzeit-Sync-Aktivität über Ihre Websites überwachen können.

### Cross-Tab-Synchronisierung

Wenn ein Benutzer dieselbe Seite in mehreren Browser-Tabs geöffnet hat, erscheinen Aktualisierungen in einem Tab sofort in den anderen Tabs. Dies funktioniert über denselben WebSocket-Sync-Mechanismus und erfordert keine zusätzliche Konfiguration.

### Sicherheit

WebSocket-Nachrichten werden über sichere Verbindungen (WSS) übertragen und beinhalten eine Mandantenvalidierung, um sicherzustellen, dass Benutzer nur Aktualisierungen für Unterhaltungen erhalten, zu deren Ansicht sie berechtigt sind. Der Server validiert alle Operationen, bevor er sie broadcastet, um unbefugten Zugriff oder Manipulationen zu verhindern.