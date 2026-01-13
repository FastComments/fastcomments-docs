---
W FastComments identyfikator artykułu lub strony, do której przypisywane są komentarze, nazywamy URL ID, ponieważ może to być zarówno adres URL, jak i identyfikator.
Zdefiniuj URL ID w następujący sposób. Komponent nasłuchuje zmian w obiekcie config i przeładuje się, więc możesz zaktualizować URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

Jeśli Twoje konto znajduje się w UE, ustaw `region = 'eu'` w konfiguracji widgeta, na przykład:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

W przeciwnym razie nie musisz definiować `region`.
---