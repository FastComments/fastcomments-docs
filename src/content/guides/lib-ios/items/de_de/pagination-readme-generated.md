### Seitengröße

```swift
// Kommentare: Standard 30
sdk.pageSize = 50

// Feed: Standard 10
feedSDK.pageSize = 20
```

### Weitere Kommentare laden

Die Benutzeroberfläche zeigt die Paginierungssteuerungen automatisch an. Sie können die Paginierung auch programmgesteuert auslösen:

```swift
// Nächste Seite laden
try await sdk.loadMore()

// Alle verbleibenden laden (deaktiviert, wenn >2000 Kommentare aus Leistungsgründen)
try await sdk.loadAll()

// Zustand prüfen
sdk.hasMore            // Ob weitere Seiten vorhanden sind
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginierung von Antworten

Verschachtelte Antworten werden lazy geladen. Wenn ein Benutzer einen Thread erweitert, werden die ersten 5 Antworten geladen. Eine Steuerung "Weitere Antworten laden" erscheint, wenn mehr vorhanden sind. Dies wird automatisch von der Benutzeroberfläche gehandhabt.