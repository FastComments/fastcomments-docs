## Kako prilagoditi stilove widgeta za komentare

Stilove widgeta za komentare možete prilagoditi na dva načina:

### Opcija 1: Putem parametra customCSS

Proslijedite svoj prilagođeni CSS kao string parametru `customCSS` prilikom inicijalizacije widgeta:

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

### Opcija 2: Putem administratorske nadzorne ploče

1. Idite na [Stranica za prilagodbu widgeta](https://fastcomments.com/auth/my-account/customize-widget) u svojoj administratorskoj nadzornoj ploči
2. Pomaknite se do odjeljka "Prilagođeni CSS" pod "Napredno"
3. Unesite svoj prilagođeni CSS
4. Kliknite "Spremi"

Vaš prilagođeni CSS će se primijeniti na sve widgete za komentare na vašoj stranici.

## Savjeti

- Koristite `!important` za nadjačavanje zadanih stilova po potrebi
- Ciljajte određene selektore kako biste izbjegli utjecaj na druge dijelove vaše stranice
- Testirajte svoj CSS u različitim preglednicima radi kompatibilnosti
- Widget koristi standardni CSS - nisu potrebni posebni preprocesori