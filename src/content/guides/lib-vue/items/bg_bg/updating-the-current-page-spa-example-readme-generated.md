В FastComments наричаме идентификатора на статията или страницата, към която са свързани коментарите, URL ID, тъй като той може да бъде URL или ID.
Дефинирайте URL ID по следния начин. Компонентът наблюдава промените в обекта config и ще презареди, така че просто можете да актуализирате настройките "url" и "urlId".

Вижте пълния работещ пример [тук](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Стартирайте примера за странициране чрез:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регион на акаунта (ВНИМАНИЕ: клиенти от ЕС)

Ако акаунтът ви се намира в ЕС, задайте `region = 'eu'` в конфигурацията на джаджата, например:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

В противен случай не е необходимо да дефинирате `region`.