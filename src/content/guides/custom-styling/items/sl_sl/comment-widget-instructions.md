## Kako prilagoditi slog komentarnega vtičnika

Slog komentarnega vtičnika lahko prilagodite na dva načina:

### Možnost 1: Preko parametra customCSS

Svoj prilagojen CSS posredujte kot niz v parameter `customCSS` med inicializacijo vtičnika:

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

### Možnost 2: Preko nadzorne plošče

1. Pojdite na [Stran za prilagoditev vtičnika](https://fastcomments.com/auth/my-account/customize-widget) v vaši nadzorni plošči
2. Pomaknite se do razdelka "Prilagojen CSS" pod "Napredno"
3. Vnesite svoj prilagojen CSS
4. Kliknite "Shrani"

Vaš prilagojeni CSS bo uporabljen za vse vtičnike za komentarje na vaši spletni strani.

## Nasveti

- Uporabite `!important`, da po potrebi preglasite privzete sloge
- Ciljajte na specifične selektorje, da ne vplivate na druge dele vaše strani
- Preizkusite svoj CSS v različnih brskalnikih zaradi združljivosti
- Vtičnik uporablja standardni CSS - posebni predprocesorji niso potrebni