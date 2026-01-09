## Hoe de stijl van de commentaarwidget aan te passen

U kunt de styling van de commentaarwidget op twee manieren aanpassen:

### Optie 1: Via de customCSS-parameter

Geef uw aangepaste CSS als een string door aan de `customCSS`-parameter wanneer u de widget initialiseert:

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

### Optie 2: Via het Admin-dashboard

1. Ga naar de [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget) in uw beheerdersdashboard
2. Scrol naar de sectie "Custom CSS" onder "Advanced"
3. Voer uw aangepaste CSS in
4. Klik op "Save"

Uw aangepaste CSS wordt toegepast op alle commentaarwidgets op uw site.

## Tips

- Gebruik `!important` om standaardstijlen indien nodig te overschrijven
- Richt u op specifieke selectoren om te voorkomen dat andere delen van uw site worden beïnvloed
- Test uw CSS in verschillende browsers voor compatibiliteit
- De widget gebruikt standaard CSS - geen speciale preprocessors nodig