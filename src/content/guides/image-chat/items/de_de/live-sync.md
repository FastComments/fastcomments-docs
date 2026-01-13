### Echtzeit-Aktualisierungen

Image Chat verwendet WebSocket-Verbindungen, um alle Unterhaltungen in Echtzeit über alle verbundenen Nutzer hinweg zu synchronisieren. Wenn jemand einen neuen Marker erstellt, einen Kommentar hinzufügt oder eine Diskussion löscht, sehen alle anderen Nutzer, die dasselbe Bild anzeigen, die Aktualisierung sofort, ohne die Seite neu zu laden.

### Wie die WebSocket-Synchronisation funktioniert

Wenn Sie Image Chat initialisieren, stellt das Widget eine WebSocket-Verbindung zu den FastComments-Servern her. Diese Verbindung bleibt für die Dauer der Sitzung des Nutzers offen und hört auf Updates, die das aktuelle Bild betreffen.

Das WebSocket-System verwendet drei Arten von Broadcast-Nachrichten für Image Chat. Das Ereignis `new-image-chat` wird ausgelöst, wenn jemand einen neuen Marker auf dem Bild erstellt. Das Ereignis `image-chat-updated` wird ausgelöst, wenn jemand eine bestehende Konversation aktualisiert. Das Ereignis `deleted-image-chat` wird ausgelöst, wenn jemand einen Marker löscht.

### Broadcast-ID-System

Um Echo-Effekte zu verhindern, bei denen Nutzer ihre eigenen Aktionen als Broadcast zurücksehen, enthält jedes Update eine eindeutige `broadcastId`. Wenn ein Nutzer einen Marker erstellt oder aktualisiert, generiert sein Client eine UUID für diesen Vorgang. Wenn das WebSocket-Update an alle Clients zurückgesendet wird, ignoriert der ursächliche Client das Update, weil es mit seiner eigenen `broadcastId` übereinstimmt.

Das stellt sicher, dass Nutzer ihre Änderungen sofort in der Benutzeroberfläche sehen, ohne auf den Hin- und Rückweg über den Server warten zu müssen, während gleichzeitig alle anderen Nutzer das Update erhalten.

### Verbindungsresilienz

Falls die WebSocket-Verbindung aufgrund von Netzwerkproblemen oder Serverwartung unterbrochen wird, versucht das Widget automatisch, die Verbindung wiederherzustellen. Während der Wiederverbindungsphase können Nutzer weiterhin mit bestehenden Markern interagieren, sehen jedoch keine Echtzeit-Updates von anderen Nutzern, bis die Verbindung wiederhergestellt ist.

Sobald die Verbindung wieder besteht, synchronisiert das Widget erneut, um sicherzustellen, dass keine Updates verpasst wurden. Dies geschieht transparent, ohne dass der Nutzer eingreifen muss.

### Bandbreitenüberlegungen

WebSocket-Nachrichten sind schlank und enthalten nur die erforderlichen Informationen zur Synchronisation des Zustands. Das Erstellen eines neuen Markers benötigt typischerweise weniger als 1 KB Bandbreite. Das System beinhaltet zudem intelligente Bündelung, um die Nachrichtenfrequenz in Zeiten hoher Aktivität zu reduzieren.

Ihre Nutzungsmetriken im FastComments-Dashboard verfolgen `pubSubMessageCount` und `pubSubBandwidth`, damit Sie die Echtzeit-Synchronisationsaktivität auf Ihren Seiten überwachen können.

### Synchronisation über mehrere Tabs

Wenn ein Nutzer dieselbe Seite in mehreren Browser-Tabs geöffnet hat, erscheinen Aktualisierungen in einem Tab sofort in den anderen Tabs. Dies funktioniert über denselben WebSocket-Synchronisationsmechanismus und erfordert keine zusätzliche Konfiguration.

Nutzer können Ihre Seite gleichzeitig auf mehreren Geräten geöffnet haben, und alle bleiben synchron. Ein auf einem Desktop-Computer erstellter Marker erscheint sofort auf dem Tablet des Nutzers, wenn beide Geräte dasselbe Bild anzeigen.

### Sicherheit

WebSocket-Nachrichten werden über sichere Verbindungen (WSS) übertragen und enthalten Tenant-Validierung, um sicherzustellen, dass Nutzer nur Updates für Konversationen erhalten, die sie sehen dürfen. Der Server validiert alle Vorgänge, bevor er sie broadcastet, um unbefugten Zugriff oder Manipulation zu verhindern.

### Offline-Verhalten

Wenn Nutzer komplett offline sind, können sie weiterhin bestehende Marker ansehen, aber keine neuen erstellen oder Updates von anderen sehen. Das Widget erkennt den Offline-Status und zeigt entsprechende Hinweise an.

Wenn ein Nutzer versucht, einen Marker offline zu erstellen und dann wieder online kommt, schlägt die Operation fehl, anstatt in die Warteschlange gestellt zu werden, um Datenkonsistenz zu gewährleisten. Nutzer sollten die Aktion erneut versuchen, sobald ihre Verbindung wiederhergestellt ist.

### Auswirkungen auf die Leistung

Die WebSocket-Verbindung hat nur minimale Auswirkungen auf die Leistung. Die Verbindung bleibt inaktiv, wenn keine Updates stattfinden, und verarbeitet nur Nachrichten, wenn Aktivität auftritt. Bei einem typischen Bild mit moderater Marker-Aktivität verwendet das WebSocket weniger CPU als das Rendern des Bildes selbst.

Bei Seiten mit Hunderten gleichzeitiger Nutzer und hoher Marker-Erstellungsaktivität skaliert das System horizontal, um die Leistung aufrechtzuerhalten, ohne einzelne Client-Verbindungen zu beeinträchtigen.

### Kollaborative Anwendungsfälle

Die Echtzeit-Synchronisation macht Image Chat besonders leistungsfähig für kollaborative Arbeitsabläufe. Design-Teams können Mockups gemeinsam überprüfen, wobei alle Beteiligten die Markerplatzierungen in Echtzeit sehen. Kundensupport-Teams können gemeinsam Screenshots annotieren, um Probleme zu identifizieren. Lerngruppen können Diagramme diskutieren, wobei alle Teilnehmer die Marker anderer sehen, sobald sie erstellt werden.

Das unmittelbare Feedback schafft ein ansprechenderes und produktiveres Zusammenarbeitserlebnis im Vergleich zu herkömmlichen Kommentarsystemen, bei denen Nutzer aktualisieren müssen, um Änderungen zu sehen.