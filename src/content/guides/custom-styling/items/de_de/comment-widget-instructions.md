## So passen Sie das Erscheinungsbild des Kommentar-Widgets an

Sie können das Styling des Kommentar-Widgets auf zwei Arten anpassen:

### Option 1: Über den `customCSS`-Parameter

Übergeben Sie Ihr benutzerdefiniertes CSS als Zeichenfolge an den `customCSS`-Parameter, wenn Sie das Widget initialisieren:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Option 2: Über das Admin-Dashboard

1. Gehen Sie zur [Widget-Anpassungsseite](https://fastcomments.com/auth/my-account/customize-widget) in Ihrem Admin-Dashboard
2. Scrollen Sie zum Abschnitt "Custom CSS" unter "Advanced"
3. Geben Sie Ihr benutzerdefiniertes CSS ein
4. Klicken Sie auf "Speichern"

Ihr benutzerdefiniertes CSS wird auf alle Kommentar-Widgets Ihrer Website angewendet.

## Tipps

- Verwenden Sie `!important`, um bei Bedarf Standardstile zu überschreiben
- Wählen Sie gezielte Selektoren aus, um andere Bereiche Ihrer Website nicht zu beeinflussen
- Testen Sie Ihr CSS in verschiedenen Browsern auf Kompatibilität
- Das Widget verwendet Standard-CSS - es sind keine speziellen Präprozessoren erforderlich