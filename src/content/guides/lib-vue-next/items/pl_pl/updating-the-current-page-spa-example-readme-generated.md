W FastComments identyfikator artykułu lub strona, do której przypięte są komentarze, nazywany jest URL ID, ponieważ może to być adres URL lub identyfikator.
Zdefiniuj URL ID w następujący sposób. Komponent obserwuje zmiany w obiekcie config i przeładuje się, więc możesz zaktualizować URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Region konta (UWAGA: Klienci z UE)

Jeśli Twoje konto znajduje się w UE, ustaw `region = 'eu'` w konfiguracji widżetu, na przykład:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

W przeciwnym razie nie musisz definiować `region`.