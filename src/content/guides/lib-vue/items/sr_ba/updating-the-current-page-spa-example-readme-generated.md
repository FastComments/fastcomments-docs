У FastComments-у зовемо идентификатор чланка, односно странице за коју се коментари везују, URL ID јер може бити url или ID.
Дефинишите URL ID на следећи начин. Компонента прати промјене у config објекту, и поново ће учитати, тако да можете једноставно ажурирати "url" и "urlId" поставке.

Погледајте потпуни примјер који ради [овдје](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Покрените примјер пагинације помоћу:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регион налога (ПАЖЊА: купци из ЕУ)

Ако се ваш налог налази у ЕУ, подесите `region = 'eu'` у конфигурацији видгета, на примјер:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

У супротном, не морате дефинисати `region`.