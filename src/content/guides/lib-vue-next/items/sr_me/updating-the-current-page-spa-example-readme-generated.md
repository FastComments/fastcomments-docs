---
У FastComments-у називамо идентификатор чланка, односно странице на коју су коментари везани, URL ID, јер то може бити URL или ID.
Дефинишите URL ID на следећи начин. Компонента прати промене у објекту config и поново ће учитати, тако да можете ажурирати URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Регион налога (ПАЖЊА: корисници из ЕУ)

Ако се ваш налог налази у ЕУ, поставите `region = 'eu'` у конфигурацији видгета, на пример:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

У супротном, не морате дефинисати `region`.
---