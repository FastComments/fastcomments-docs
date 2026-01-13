## Wie Sie das Styling des Kommentar-Widgets anpassen

Sie können das Styling des Kommentar-Widgets auf zwei Arten anpassen:

### Option 1: Über den customCSS-Parameter

Geben Sie Ihr benutzerdefiniertes CSS als String an den `customCSS`-Parameter bei der Initialisierung des Widgets:

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### Option 2: Über das Admin-Dashboard

1. Gehen Sie auf die [Seite zur Anpassung des Widgets](https://fastcomments.com/auth/my-account/customize-widget) in Ihrem Admin-Dashboard  
2. Scrollen Sie zum Abschnitt "Benutzerdefiniertes CSS" unter "Erweitert"  
3. Geben Sie Ihr benutzerdefiniertes CSS ein  
4. Klicken Sie auf "Speichern"

Ihr benutzerdefiniertes CSS wird auf alle Kommentar-Widgets auf Ihrer Website angewendet.

## Tipps

- Verwenden Sie `!important`, um bei Bedarf Standardstile zu überschreiben  
- Zielen Sie auf spezifische Selektoren, um zu vermeiden, dass andere Teile Ihrer Website betroffen sind  
- Testen Sie Ihr CSS in verschiedenen Browsern auf Kompatibilität  
- Das Widget verwendet Standard-CSS - es sind keine speziellen Präprozessoren erforderlich