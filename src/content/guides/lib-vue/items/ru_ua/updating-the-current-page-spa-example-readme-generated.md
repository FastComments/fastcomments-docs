В FastComments идентификатор статьи или страницы, к которой привязаны комментарии, называется URL ID, так как это может быть url или идентификатор.
Определяйте URL ID следующим образом. Компонент отслеживает изменения в объекте config и перезагрузится, поэтому вы можете просто обновить настройки "url" и "urlId".

См. полный рабочий пример [здесь](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Запустите пример пагинации командой:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регион аккаунта (ВНИМАНИЕ: клиенты из ЕС)

Если ваш аккаунт расположен в ЕС, установите `region = 'eu'` в конфигурации виджета, например:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

В противном случае задавать `region` не обязательно.