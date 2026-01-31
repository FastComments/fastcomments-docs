## Hoe de stijlen van de commentaarwidget aan te passen

Je kunt de opmaak van de commentaarwidget op twee manieren aanpassen:

### Optie 1: Via customCSS-parameter

Geef je aangepaste CSS als een string door aan de `customCSS`-parameter bij het initialiseren van de widget:

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

### Optie 2: Via Admin-dashboard

1. Ga naar de [Pagina 'Widget aanpassen'](https://fastcomments.com/auth/my-account/customize-widget) in je admin-dashboard
2. Scrol naar de "Aangepaste CSS"-sectie onder "Geavanceerd"
3. Voer je aangepaste CSS in
4. Klik op "Opslaan"

Je aangepaste CSS wordt toegepast op alle commentaarwidgets op je site.

## Tips

- Gebruik `!important` om standaardstijlen te overschrijven indien nodig
- Richt je op specifieke selectors om te voorkomen dat andere delen van je site worden beïnvloed
- Test je CSS in verschillende browsers voor compatibiliteit
- De widget gebruikt standaard CSS - geen speciale preprocessors vereist