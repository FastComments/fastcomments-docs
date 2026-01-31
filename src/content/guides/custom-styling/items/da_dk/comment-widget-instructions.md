## Sådan tilpasser du kommentar-widgetens stil

Du kan tilpasse kommentarboksens styling på to måder:

### Mulighed 1: Via customCSS-parameteren

Send dit brugerdefinerede CSS som en streng til `customCSS`-parameteren, når du initialiserer widgeten:

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

### Mulighed 2: Via Admin-dashboard

1. Gå til [Widget-tilpasningssiden](https://fastcomments.com/auth/my-account/customize-widget) i dit admin-dashboard
2. Rul ned til sektionen "Brugerdefineret CSS" under "Avanceret"
3. Indtast dit brugerdefinerede CSS
4. Klik på "Gem"

Dit brugerdefinerede CSS vil blive anvendt på alle kommentar-widgets på dit websted.

## Tips

- Brug `!important` for at tilsidesætte standardstilarter, hvis det er nødvendigt
- Målret specifikke selektorer for at undgå at påvirke andre dele af dit websted
- Test dit CSS i forskellige browsere for kompatibilitet
- Widgeten bruger standard CSS - ingen særlige preprocessorer er nødvendige