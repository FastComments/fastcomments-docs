## Kako prilagoditi stilove widgeta za komentare

Stilove widgeta za komentare možete prilagoditi na dva načina:

### Opcija 1: Pomoću parametra customCSS

Prosledite svoj prilagođeni CSS kao string parametru `customCSS` prilikom inicijalizacije widgeta:

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

### Opcija 2: Putem administratorskog panela

1. Idite na stranicu [Stranica za prilagođavanje widgeta](https://fastcomments.com/auth/my-account/customize-widget) u svom administratorskom panelu
2. Skrolujte do odeljka "Prilagođeni CSS" pod "Napredno"
3. Unesite svoj prilagođeni CSS
4. Kliknite "Sačuvaj"

Vaš prilagođeni CSS će biti primenjen na sve widgete za komentare na vašem sajtu.

## Saveti

- Koristite `!important` da po potrebi nadjačate podrazumevane stilove
- Ciljajte konkretne selektore da biste izbegli uticaj na druge delove sajta
- Testirajte svoj CSS u različitim pregledačima radi kompatibilnosti
- Widget koristi standardni CSS - nisu potrebni posebni preprocesori