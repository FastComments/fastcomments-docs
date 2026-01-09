## Kako prilagoditi stilove widgeta za komentare

Možete prilagoditi stilove widgeta za komentare na dva načina:

### Opcija 1: Putem parametra customCSS

Prosledite vaš prilagođeni CSS kao string u parametar `customCSS` prilikom inicijalizacije widgeta:

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

1. Idite na [Stranicu za prilagođavanje widgeta](https://fastcomments.com/auth/my-account/customize-widget) u vašem administratorskom panelu
2. Pomaknite se do odjeljka "Prilagođeni CSS" pod "Napredno"
3. Unesite vaš prilagođeni CSS
4. Kliknite "Sačuvaj"

Vaš prilagođeni CSS će se primijeniti na sve widgete za komentare na vašoj stranici.

## Savjeti

- Koristite `!important` da nadjačate zadane stilove po potrebi
- Ciljajte određene selektore kako biste izbegli uticaj na druge delove vaše stranice
- Testirajte vaš CSS u različitim preglednicima radi kompatibilnosti
- Widget koristi standardni CSS - nisu potrebni posebni preprocesori