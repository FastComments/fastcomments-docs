I FastComments kalder vi artikel-id'et, eller den side som kommentarerne knyttes til, for URL ID, da det kan være en url eller et ID.
Definér URL ID på følgende måde. Komponenten overvåger ændringer i config object, og vil genindlæse, så du kan blot opdatere indstillingerne "url" og "urlId".

Se et fuldt fungerende eksempel [her](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Kør pagination-eksemplet via:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Kontoområde (OPMÆRKSOMHED: EU-kunder)

Hvis din konto er placeret i EU, sæt `region = 'eu'` i widget-konfigurationen, for eksempel:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Ellers behøver du ikke at definere `region`.