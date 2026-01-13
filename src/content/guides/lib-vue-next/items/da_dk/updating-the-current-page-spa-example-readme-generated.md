I FastComments kalder vi artikel-id'et, altså den side som kommentarerne er knyttet til, for URL ID, eftersom det kan være en URL eller et ID.
Definér URL ID på følgende måde. Komponenten overvåger ændringer i config-objektet og vil genindlæse, så du kan opdatere URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Konto-region (OPMÆRKSOMHED: EU-kunder)

Hvis din konto er placeret i EU, sæt `region = 'eu'` i widget-konfigurationen, for eksempel:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Ellers behøver du ikke at definere `region`.