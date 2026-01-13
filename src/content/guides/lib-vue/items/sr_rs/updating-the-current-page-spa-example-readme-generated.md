У FastComments-у називамо идентификатор чланка, односно странице на коју су коментари везани, URL ID јер може бити URL или ID.
Дефинишите URL ID на следећи начин. Компонента прати измене у config објекту и поново ће се учитати, тако да можете једноставно ажурирати подешавања "url" и "urlId".

Погледајте цео радни пример [овде](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Покрените пример пагинације помоћу:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регион налога (ПАЖЊА: клијенти из ЕУ)

Ако је ваш налог у ЕУ, подесите `region = 'eu'` у конфигурацији видгета, на пример:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

У супротном, није потребно да дефинишете `region`.