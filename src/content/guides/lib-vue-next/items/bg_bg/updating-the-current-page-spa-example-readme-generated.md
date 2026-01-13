---
В FastComments наричаме идентификатора на статията или страницата, към която са прикрепени коментарите, URL ID, тъй като може да бъде URL или ID.
Определете URL ID по следния начин. Компонентът наблюдава промените в обекта config и ще презареди, така че можете да актуализирате URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Регион на акаунта (ВНИМАНИЕ: клиенти от ЕС)

Ако вашият акаунт се намира в ЕС, задайте `region = 'eu'` в конфигурацията на уиджета, например:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

В противен случай не е нужно да дефинирате `region`.
---