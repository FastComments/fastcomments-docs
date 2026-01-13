---
У FastComments-у називамо идентификатор чланка, или странице којој су коментари везани, URL ID јер то може бити URL или ID.
Дефинишите URL ID на следећи начин. Компонента прати промене у config објекту и поново ће учитати, тако да можете да ажурирате URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Регион налога (ПАЖЊА: корисници из ЕУ)

Ако је ваш налог смештен у ЕУ, подесите `region = 'eu'` у конфигурацији видгета, на пример:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

У супротном, не морате дефинисати `region`.
---