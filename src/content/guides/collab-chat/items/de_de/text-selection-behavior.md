### Wie die Textauswahl funktioniert

Wenn Benutzer Text innerhalb des Collab Chat Containers auswählen, erfasst das Widget diese Auswahl und ermöglicht es ihnen, eine Diskussion zu starten. Die Auswahl kann so klein wie ein einzelnes Wort oder so groß wie mehrere Absätze sein, die sich über verschiedene Elemente erstrecken.

### Verzögerung bei der Auswahl

Auf Desktop-Geräten gibt es eine Verzögerung von 3,5 Sekunden zwischen dem Zeitpunkt, an dem ein Benutzer Text auswählt, und dem Erscheinen der Diskussionsaufforderung. Dies verhindert ein Flackern der UI, wenn Benutzer Text nur zum Kopieren oder Lesen markieren. Auf mobilen Geräten erscheint die Aufforderung sofort, da die Textauswahl auf Touchscreens gezielter erfolgt.


### Eindeutige Konversations-IDs

Jede Konversation erhält eine eindeutige `urlId`, die die Seiten-URL, den DOM-Elementpfad und den serialisierten Bereich des Textes kombiniert. Dadurch wird sichergestellt, dass jede Textauswahl eine eigene Konversation erzeugt, die später wiedergefunden werden kann.

Wenn Sie eine benutzerdefinierte `urlId` in Ihrer Konfiguration angeben, wird diese mit dem Elementpfad und dem Textbereich kombiniert, um den endgültigen Bezeichner zu erstellen.

### Visuelle Hervorhebungen

Wenn für eine bestimmte Textauswahl eine Diskussion existiert, wird dieser Text visuell hervorgehoben. Die Hervorhebung wird mittels Hintergrundfarben umgesetzt und erscheint beim Überfahren mit der Maus (hover) oder wenn das zugehörige Chatfenster geöffnet ist.

Das Hervorhebungssystem funktioniert, indem der ausgewählte Text in ein spezielles Element eingeschlossen wird, das gestylt werden kann. Dieser Ansatz stellt sicher, dass Hervorhebungen genau bleiben, selbst wenn die zugrunde liegende HTML-Struktur komplex ist.

### Positionierung des Chatfensters

Wenn ein Benutzer auf eine Hervorhebung klickt oder eine neue Anmerkung erstellt, erscheint in der Nähe des ausgewählten Textes ein Chatfenster. Das Widget berechnet automatisch die beste Position für dieses Fenster basierend auf dem verfügbaren Viewport-Bereich.

Das Positionierungssystem verwendet CSS-Klassen wie `to-right`, `to-left`, `to-top` und `to-bottom`, um anzugeben, in welche Richtung sich das Chatfenster von der Hervorhebung aus erstrecken soll. Auf mobilen Geräten (Bildschirme schmaler als 768px) erscheint das Chatfenster aus Gründen der besseren Bedienbarkeit immer im Vollbild.

### Dimensionen des Chatfensters

Chatfenster sind auf dem Desktop 410px breit, mit 20px Abstand und einem 16px großen visuellen Pfeil, der auf den hervorgehobenen Text zeigt. Diese Abmessungen sind festgelegt, um ein konsistentes Verhalten zu gewährleisten, können aber mit CSS an das Erscheinungsbild angepasst werden.

### Auswahl über mehrere Elemente

Benutzer können Text auswählen, der sich über mehrere HTML-Elemente erstreckt, etwa vom Mitte eines Absatzes bis zum Anfang eines anderen. Das System zur Serialisierung des Bereichs verarbeitet dies korrekt und hebt den gesamten ausgewählten Text hervor, sogar über Elementgrenzen hinweg.

### Browser-Kompatibilität

Das Textauswahlsystem verwendet die standardmäßige API `window.getSelection()`, die in allen modernen Browsern unterstützt wird. Für ältere Versionen des Internet Explorers greift es zur Kompatibilität auf `document.selection` zurück.

### Persistenz der Auswahl

Sobald für eine Textauswahl eine Konversation erstellt wurde, bleibt diese Anmerkung erhalten, selbst wenn die Seite neu geladen wird. Der serialisierte Bereich und der DOM-Pfad ermöglichen es dem Widget, Hervorhebungen an genau derselben Stelle wiederherzustellen, wenn Benutzer zur Seite zurückkehren.

Dies funktioniert zuverlässig, solange Ihre Seiteninhalte stabil bleiben. Wenn Sie den Textinhalt ändern oder Ihre HTML-Struktur umgestalten, stimmen bestehende Anmerkungen möglicherweise nicht mehr korrekt mit dem Text überein. Aus diesem Grund ist es am besten, größere Inhaltsänderungen auf Seiten mit aktiven Anmerkungen zu vermeiden oder in Betracht zu ziehen, Anmerkungen zu migrieren, wenn Inhaltsänderungen notwendig sind.