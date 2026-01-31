---
## Jak dostosować style widżetu komentarzy

Możesz dostosować styl widżetu komentarzy na dwa sposoby:

### Opcja 1: Za pomocą parametru `customCSS`

Przekaż swój niestandardowy CSS jako łańcuch do parametru `customCSS` podczas inicjalizacji widżetu:

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

### Opcja 2: Za pomocą panelu administracyjnego

1. Przejdź do [strony personalizacji widżetu](https://fastcomments.com/auth/my-account/customize-widget) w panelu administracyjnym
2. Przewiń do sekcji "Niestandardowy CSS" w sekcji "Zaawansowane"
3. Wprowadź swój niestandardowy CSS
4. Kliknij "Zapisz"

Twój niestandardowy CSS zostanie zastosowany do wszystkich widżetów komentarzy na Twojej stronie.

## Wskazówki

- Użyj `!important`, aby w razie potrzeby nadpisać domyślne style
- Celuj w konkretne selektory, aby nie wpływać na inne części witryny
- Testuj swój CSS w różnych przeglądarkach pod kątem zgodności
- Widżet używa standardowego CSS - nie są wymagane żadne specjalne preprocesory

---