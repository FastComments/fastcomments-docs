### Prozentbasierte Positionierung

Image Chat verwendet prozentbasierte Koordinaten statt Pixelkoordinaten, um Chat-Markierungen auf Bildern zu positionieren. Wenn ein Benutzer auf ein Bild klickt, wandelt das Widget die Pixelkoordinaten des Klicks in Prozentsätze der Bildbreite und -höhe um. Dieser Ansatz stellt sicher, dass die Markierungen an der richtigen Stelle bleiben, unabhängig davon, wie das Bild angezeigt wird.

Zum Beispiel: Wenn ein Benutzer 250 Pixel vom linken Rand eines 1000px breiten Bildes klickt, speichert das Widget dies als 25% vom linken Rand. Wenn ein anderer Benutzer dasselbe Bild auf einem Mobilgerät mit 500px Breite betrachtet, erscheint die Markierung 125 Pixel vom linken Rand entfernt, was immer noch 25% der Breite entspricht.

### Vorteile für responsive Layouts

Dieses Prozent-System ermöglicht, dass Image Chat auf allen Gerätegrößen und Ausrichtungen nahtlos funktioniert. Ihre Bilder können je nach Bildschirmbreite, CSS-Regeln oder Containerbeschränkungen in unterschiedlichen Größen dargestellt werden, aber die Markierungen richten sich immer korrekt an den vorgesehenen Positionen aus.

Benutzer an Desktop-Computern mit großen Monitoren sehen Markierungen an denselben relativen Positionen wie Benutzer auf Smartphones mit kleinen Bildschirmen. Die Markierungen skalieren proportional mit dem Bild selbst.

### Bildskalierung und Seitenverhältnis

Solange Ihr Bild sein Seitenverhältnis beim Skalieren beibehält (was das Standardverhalten des Browsers ist), bleiben die prozentbasierten Markierungen perfekt ausgerichtet. Das Widget geht davon aus, dass Bilder proportional skaliert werden, und berechnet die Positionen basierend auf dieser Annahme.

Wenn Sie CSS anwenden, das das Seitenverhältnis des Bildes verzerrt (wie die Verwendung von `object-fit: cover` mit spezifischen Abmessungen), können die Markierungspositionen möglicherweise nicht korrekt ausgerichtet sein. Für beste Ergebnisse lassen Sie Bilder natürlich skalieren oder verwenden Sie `object-fit: contain`, um das Seitenverhältnis beizubehalten.

### Größe der Chat-Quadrate

Die visuelle Größe der Chat-Markierungen ist ebenfalls prozentbasiert. Die Konfigurationsoption `chatSquarePercentage` hat standardmäßig 5% eingestellt, was bedeutet, dass jedes Quadrat 5% der Bildbreite ausmacht. Dies sorgt für ein gleichmäßiges visuelles Gewicht bei unterschiedlichen Bildgrößen.

Bei einem 1000px breiten Bild mit der Standard-Einstellung von 5% sind die Markierungen 50px quadratisch. Bei einem 500px breiten Bild sind dieselben Markierungen 25px quadratisch. Sie bleiben proportional zur Bildgröße.

### Verhalten auf Mobilgeräten

Bei Bildschirmen unter 768px Breite wechselt Image Chat zu einem mobil-optimierten Layout. Chat-Fenster öffnen sich im Vollbildmodus statt neben der Markierung zu schweben. Dies bietet eine bessere Benutzerfreundlichkeit auf kleinen Bildschirmen, wo schwebende Fenster zu viel vom Bild verdecken würden.

Die Markierungen selbst bleiben sichtbar und an ihren prozentbasierten Positionen anklickbar. Benutzer können weiterhin sehen, wo sich alle Diskussionen befinden, und Markierungen antippen, um die Vollbild-Chat-Oberfläche zu öffnen.

### Dynamisches Laden von Bildern

Das prozentbasierte System funktioniert korrekt, selbst wenn Bilder asynchron geladen werden oder sich nach dem Laden der Seite in der Größe ändern. Das Widget überwacht das Bildelement und berechnet die Markierungspositionen neu, wann immer sich die Bildabmessungen ändern.

Wenn Sie Bilder verzögert laden (lazy-loading) oder responsive Bilder mit unterschiedlichen Größen bei verschiedenen Breakpoints implementieren, passen sich die Markierungen automatisch an, wenn sich die Bildgröße ändert.

### Konsistenz über Geräte hinweg

Da Koordinaten in der Datenbank als Prozentwerte gespeichert werden, erscheint eine Diskussion, die auf einem Desktop-Computer erstellt wurde, an exakt derselben relativen Position, wenn sie auf einem Tablet oder Telefon angezeigt wird. Benutzer können geräteübergreifend zusammenarbeiten, ohne Positionsinkonsistenzen.

Dies funktioniert bidirektional. Eine Diskussion, die durch Antippen einer bestimmten Stelle auf einem Mobilgerät erstellt wurde, erscheint an derselben relativen Position, wenn sie auf einem großen Desktop-Monitor betrachtet wird.

### Betrachtung des Viewports

Das Widget berechnet die Prozentsätze relativ zum Bildelement selbst, nicht zum Viewport. Das bedeutet, dass Scrollen auf der Seite oder das Ändern der Browserfenstergröße die Markierungspositionen nicht beeinflusst. Markierungen bleiben unabhängig von Viewport-Änderungen an ihren Positionen auf dem Bild verankert.

### Zukunftssicherheit von Inhalten

Der prozentbasierte Ansatz macht Ihre Bilddiskussionen widerstandsfähig gegenüber Änderungen im Layout, Design oder Gerätespektrum. Mit dem Aufkommen neuer Bildschirmgrößen und Geräte werden die vorhandenen Diskussionen weiterhin korrekt angezeigt, ohne dass Aktualisierungen oder Migrationen erforderlich sind.