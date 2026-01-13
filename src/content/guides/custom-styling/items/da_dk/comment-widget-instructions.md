## Sådan tilpasser du kommentar-widget-stil

Du kan tilpasse kommentar-widget-stilen på to måder:

### Valgmulighed 1: Via customCSS-parameteren

Send dit tilpassede CSS som en streng til `customCSS`-parameteren, når du initialiserer widget'en:

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

### Valgmulighed 2: Via administrationsdashboard

1. Gå til [Widget Customization-siden](https://fastcomments.com/auth/my-account/customize-widget) i dit admin-dashboard
2. Rul ned til afsnittet "Tilpasset CSS" under "Avanceret"
3. Indtast dit tilpassede CSS
4. Klik på "Gem"

Dit tilpassede CSS bliver anvendt på alle kommentar-widgets på dit site.

## Tips

- Brug `!important` for at tilsidesætte standardstilarter, hvis nødvendigt
- Mål specifikke selektorer for at undgå at påvirke andre dele af dit site
- Test dit CSS i forskellige browsere for kompatibilitet
- Widget'en bruger standard CSS - ingen særlige preprocessorer er nødvendige