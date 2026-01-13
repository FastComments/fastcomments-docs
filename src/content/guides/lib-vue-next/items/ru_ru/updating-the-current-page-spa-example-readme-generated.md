В FastComments идентификатор статьи или страницы, к которой привязаны комментарии, называется URL ID, так как это может быть URL или ID.
Определите URL ID следующим образом. Компонент отслеживает изменения объекта config и будет перезагружать содержимое, поэтому вы можете обновлять URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Регион аккаунта (ВНИМАНИЕ: клиенты из ЕС)

Если ваш аккаунт находится в ЕС, установите `region = 'eu'` в конфигурации виджета, например:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

В противном случае вам не нужно задавать `region`.