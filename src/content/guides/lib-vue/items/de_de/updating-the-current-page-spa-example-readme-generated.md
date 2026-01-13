Bei FastComments bezeichnen wir die Artikel-ID bzw. die Seite, an die Kommentare gebunden werden, als URL-ID, da es sich dabei um eine URL oder eine ID handeln kann.
Definieren Sie die URL-ID auf folgende Weise. Die Komponente überwacht Änderungen im config-Objekt und lädt neu, sodass Sie einfach die Einstellungen "url" und "urlId" aktualisieren können.

Ein vollständiges, funktionierendes Beispiel finden Sie [hier](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Starten Sie das Paginierungsbeispiel mit:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Kontoregion (ACHTUNG: EU-Kunden)

Wenn sich Ihr Konto in der EU befindet, setzen Sie `region = 'eu'` in der Widget-Konfiguration, zum Beispiel:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Andernfalls müssen Sie `region` nicht definieren.