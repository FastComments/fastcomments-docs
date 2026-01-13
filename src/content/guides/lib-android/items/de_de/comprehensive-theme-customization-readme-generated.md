Alle Schaltflächen und UI-Elemente im FastComments SDK sind themenfähig. Verwenden Sie `FastCommentsTheme.Builder` für vollständige Kontrolle über das Branding Ihrer App.

### Programmgesteuerte Themenanpassung (empfohlen)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Aktionsschaltflächen: Senden, Abstimmen, Menü, Like/Teilen-Schaltflächen
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Antwort-Schaltflächen: Kommentar-Antwort-Schaltflächen  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Umschalt-Schaltflächen: Schaltflächen zum Anzeigen/Verbergen von Antworten
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // "Load more"-Schaltflächen: Paginierungs-Schaltflächen
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Theme anwenden
sdk.setTheme(theme)
```

### Schnelles Überschreiben von Farben

Override von Farbressourcen in Ihrer `colors.xml` für einfaches Branding:

```xml
<!-- In Ihrer App: res/values/colors.xml -->
<resources>
    <!-- Alle primären UI-Elemente ändern -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Oder spezifische Schaltflächentypen anpassen -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Abdeckung thematisierter Schaltflächen

**Jede Schaltfläche im SDK unterstützt Themes:**
- Sende-Schaltflächen, Abstimm-Schaltflächen, Menü-Schaltflächen, Antwort-Schaltflächen
- Schaltflächen zum Anzeigen/Verbergen von Antworten, "Load more"-Schaltflächen  
- Feed-Aktionsschaltflächen (Gefällt mir, Kommentar, Teilen)
- Dialogschaltflächen (Absenden, Abbrechen, Speichern)
- Dynamische Aufgaben-Schaltflächen in Feed-Beiträgen

Für ausführliche Dokumentation zur Themenanpassung siehe [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).