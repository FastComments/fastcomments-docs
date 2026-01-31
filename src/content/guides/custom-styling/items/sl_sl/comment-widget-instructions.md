## Kako prilagoditi sloge pripomočka za komentarje

Slog pripomočka za komentarje lahko prilagodite na dva načina:

### Možnost 1: prek parametra customCSS

Posredujte svojo prilagojeno CSS kot niz parametru `customCSS` med inicializacijo pripomočka:

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

### Možnost 2: prek skrbniške nadzorne plošče

1. Pojdite na stran [Prilagajanje pripomočka](https://fastcomments.com/auth/my-account/customize-widget) v vaši skrbniški nadzorni plošči
2. Pomaknite se do razdelka "Prilagojen CSS" pod "Napredno"
3. Vnesite svojo prilagojeno CSS
4. Kliknite "Shrani"

Vaša prilagojena CSS bo uporabljena za vse pripomočke za komentarje na vaši spletni strani.

## Nasveti

- Uporabite `!important` za preglasitev privzetih stilov, če je potrebno
- Ciljajte na določene selektorje, da ne boste vplivali na druge dele vaše strani
- Preizkusite svojo CSS v različnih brskalnikih za združljivost
- Pripomoček uporablja standardni CSS - niso potrebni posebni predprocesorji