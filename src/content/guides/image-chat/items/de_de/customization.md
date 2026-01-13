### Dark-Mode-Unterstützung

Image Chat enthält eingebaute Unterstützung für den Dark-Mode. Wenn Sie `hasDarkBackground: true` in Ihrer Konfiguration setzen, passen sich die Chat-Fenster und UI-Elemente automatisch an dunkle Hintergründe an.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Die Dark-Mode-Stylingregeln gelten für Chat-Fenster, Marker-Quadrate und alle interaktiven Elemente. Wenn Ihre Seite einen Dark-Mode-Schalter hat, können Sie das Widget beim Moduswechsel neu initialisieren oder die unten beschriebene Body-Class-Methode verwenden.

### Dynamischer Dark-Mode

Wenn der Dark-Mode Ihrer Seite durch Hinzufügen einer `.dark`-Klasse zum Body-Element gesteuert wird, respektiert die Image Chat-Oberfläche dies automatisch ohne erneute Initialisierung. Die Styles des Widgets sind darauf ausgelegt, auf das Vorhandensein dieser Klasse zu reagieren.

```css
/* Ihr Dark-Mode-CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Benutzerdefiniertes Styling mit CSS

Sie können das Aussehen von Markern, Chat-Fenstern und anderen Elementen mit CSS anpassen. Das Widget fügt bestimmte Klassen hinzu, die Sie in Ihrem Stylesheet ansprechen können.

Die Chat-Quadrate und -Fenster verwenden das FastComments-Comment-Bubble-Styling-System, sodass alle Anpassungen, die Sie am Standard-Kommentar-Widget vorgenommen haben, auch Image Chat beeinflussen.

### Größe der Chat-Quadrate

Die Option `chatSquarePercentage` steuert die Größe der klickbaren Marker. Der Standardwert ist 5% der Bildbreite:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Größere, besser sichtbare Quadrate
});
```

Kleinere Werte erzeugen subtilere Marker, die sich ins Bild einfügen. Größere Werte machen die Marker prominenter und leichter anklickbar, besonders auf Mobilgeräten oder aus Gründen der Barrierefreiheit.

### Mobilverhalten

Auf Bildschirmen unter 768px Breite schaltet Image Chat automatisch auf ein mobiloptimiertes Layout um. Chat-Fenster erscheinen im Vollbild statt neben den Markern zu schweben, was die Bedienbarkeit auf kleinen Bildschirmen verbessert.

Die Marker bleiben an ihren responsiven Positionen im Bild sichtbar. Benutzer können einen Marker antippen, um die Vollbild-Chat-Oberfläche zu öffnen. Dieses Verhalten ist eingebaut und erfordert keine Konfiguration.

### Aussehen des Chat-Fensters

Chat-Fenster sind auf Desktop 300px breit und haben einen 16px-Pfeil, der auf den Marker zeigt. Die Fenster positionieren sich automatisch basierend auf dem verfügbaren Viewport-Platz und verwenden Positionierungsklassen wie `to-right`, `to-left`, `to-top` und `to-bottom`.

Sie können benutzerliches CSS hinzufügen, um Farben, Schriftarten, Abstände oder andere visuelle Eigenschaften dieser Fenster anzupassen. Die Chat-Fenster verwenden dieselbe Komponentenstruktur wie das Standard-FastComments-Widget, daher erben sie alle globalen Anpassungen, die Sie vorgenommen haben.

### Lazy-Initialisierung

Chat-Fenster initialisieren sich bei Desktop-Benutzern beim Hover oder sofort, wenn sie erstellt werden. Dadurch wird die anfängliche Ladebelastung reduziert, da die Chat-Oberfläche nur gerendert wird, wenn Benutzer tatsächlich mit einem Marker interagieren.

Die Lazy-Initialisierung geschieht transparent. Benutzer bemerken keine Verzögerung, aber der Browser muss nicht Dutzende von versteckten Chat-Fenstern rendern, wenn Sie viele Marker auf einem Bild haben.

### Lokalisierung

Image Chat unterstützt alle dieselben Lokalisierungsoptionen wie das Standard-FastComments-Widget. Setzen Sie die Option `locale`, um UI-Text in verschiedenen Sprachen anzuzeigen:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Französisch
});
```

FastComments unterstützt Dutzende von Sprachen. Die Locale-Einstellung beeinflusst alle UI-Texte einschließlich Eingabeaufforderungen, Buttons und Platzhaltertexte.

### Vererbte Anpassungsoptionen

Da Image Chat das Standard-Kommentar-Widget erweitert, erbt es alle Anpassungsoptionen des Basis-Widgets. Dies umfasst benutzerdefinierte CSS-Klassen, benutzerdefinierte Übersetzungen, Avatar-Anpassungen, Datumsformatierung und vieles mehr.

Siehe die Hauptdokumentation zu FastComments für die vollständige Liste der verfügbaren Anpassungsoptionen.

### Arbeiten mit benutzerdefinierten Schriftarten

Wenn Ihre Seite benutzerdefinierte Schriftarten verwendet, übernimmt die Image Chat-Oberfläche diese Schriftarten aus dem CSS Ihrer Seite. Die Chat-Fenster werden innerhalb des DOM Ihrer Seite gerendert und respektieren Ihre bestehende Typografie-Einstellungen.

Für beste Ergebnisse stellen Sie sicher, dass Ihre benutzerdefinierten Schriftarten geladen sind, bevor Sie Image Chat initialisieren, oder akzeptieren Sie, dass es einen kurzen Flash ungstilisierter Schriftarten geben kann, während die Schriftarten geladen werden.

### Visuelles Design der Marker

Die Quadrat-Markierungen haben ein dezentes visuelles Design, das sie bemerkbar macht, ohne das Bild zu überlagern. Sie können deren Aussehen mit CSS anpassen, wenn Sie eine andere visuelle Behandlung wünschen.

Die Marker enthalten Hover-Zustände, die Feedback geben, wenn Benutzer mit der Maus darüber fahren. Auf Touch-Geräten liefert die Tippen-Interaktion sofortiges Feedback, indem das Chat-Fenster geöffnet wird.