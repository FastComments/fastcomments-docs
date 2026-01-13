W FastComments identyfikator artykułu lub strony, z którą powiązane są komentarze, nazywamy URL ID, ponieważ może to być url lub ID.
Zdefiniuj URL ID w następujący sposób. Komponent nasłuchuje zmian w obiekcie config i przeładuje się, więc możesz po prostu zaktualizować ustawienia "url" i "urlId".

Zobacz pełny działający przykład [tutaj](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Uruchom przykład paginacji za pomocą:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Region konta (UWAGA: klienci z UE)

Jeśli Twoje konto znajduje się w UE, ustaw `region = 'eu'` w konfiguracji widżetu, na przykład:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

W przeciwnym razie nie musisz definiować `region`.