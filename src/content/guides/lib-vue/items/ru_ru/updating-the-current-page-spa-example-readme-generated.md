В FastComments идентификатор статьи или страницы, к которой привязаны комментарии, называется URL ID, поскольку это может быть URL или идентификатор.
Определите URL ID следующим образом. Компонент отслеживает изменения в объекте конфигурации и будет перезагружаться, поэтому вы можете просто обновить настройки "url" и "urlId".

См. полный рабочий пример [здесь](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Запустите пример пагинации командой:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регион аккаунта (ВНИМАНИЕ: клиенты ЕС)

Если ваш аккаунт находится в ЕС, установите `region = 'eu'` в конфигурации виджета, например:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

В противном случае вам не нужно задавать `region`.