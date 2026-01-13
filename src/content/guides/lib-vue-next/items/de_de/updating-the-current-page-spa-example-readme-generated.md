Bei FastComments nennen wir die Artikel-ID oder die Seite, an die die Kommentare gebunden sind, die URL-ID, da es sich um eine url oder eine ID handeln kann.
Definieren Sie die URL-ID wie folgt. Die Komponente beobachtet Änderungen am config-Objekt und lädt neu, sodass Sie die URL-ID aktualisieren können.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Kontoregion (ACHTUNG: EU-Kunden)

Wenn sich Ihr Konto in der EU befindet, setzen Sie `region = 'eu'` in der Widget-Konfiguration, zum Beispiel:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Andernfalls müssen Sie `region` nicht definieren.