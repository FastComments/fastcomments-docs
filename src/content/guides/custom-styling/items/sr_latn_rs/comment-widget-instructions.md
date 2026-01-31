## Kako prilagoditi stilove widgeta za komentare

Stilove widgeta za komentare možete prilagoditi na dva načina:

### Opcija 1: Putem parametra customCSS

Prosledite vaš prilagođeni CSS kao string u parametar `customCSS` prilikom inicijalizacije widgeta:

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

### Opcija 2: Putem administratorskog panela

1. Idite na [stranicu za prilagođavanje widgeta](https://fastcomments.com/auth/my-account/customize-widget) u vašem administratorskom panelu
2. Skrolujte do odeljka "Prilagođeni CSS" pod "Napredno"
3. Unesite vaš prilagođeni CSS
4. Kliknite "Sačuvaj"

Vaš prilagođeni CSS biće primenjen na sve widgete za komentare na vašem sajtu.

## Saveti

- Koristite `!important` da biste po potrebi nadjačali podrazumevane stilove
- Ciljajte specifične selektore kako ne biste uticali na druge delove sajta
- Testirajte vaš CSS u različitim pregledačima radi kompatibilnosti
- Widget koristi standardni CSS - nisu potrebni posebni preprocesori