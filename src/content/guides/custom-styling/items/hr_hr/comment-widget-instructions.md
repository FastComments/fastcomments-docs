## Kako prilagoditi stilove widgeta komentara

Stilove widgeta za komentare možete prilagoditi na dva načina:

### Opcija 1: Putem parametra customCSS

Proslijedite svoj prilagođeni CSS kao string u parametar `customCSS` pri inicijalizaciji widgeta:

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

### Opcija 2: Putem administratorske nadzorne ploče

1. Idite na [stranicu za prilagodbu widgeta](https://fastcomments.com/auth/my-account/customize-widget) u svojoj administratorskoj nadzornoj ploči
2. Pomaknite se do odjeljka "Custom CSS" pod "Advanced"
3. Unesite svoj prilagođeni CSS
4. Kliknite "Save"

Vaš prilagođeni CSS primijenit će se na sve widgete za komentare na vašoj web-lokaciji.

## Savjeti

- Koristite `!important` za nadjačavanje zadanim stilovima ako je potrebno
- Ciljajte specifične selektore kako biste izbjegli utjecaj na druge dijelove vaše web-lokacije
- Testirajte svoj CSS u različitim preglednicima radi kompatibilnosti
- Widget koristi standardni CSS - nisu potrebni posebni preprocessori