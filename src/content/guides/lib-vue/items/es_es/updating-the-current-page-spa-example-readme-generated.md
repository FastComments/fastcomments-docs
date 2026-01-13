En FastComments llamamos al id del artículo, o a la página a la que se vinculan los comentarios, el URL ID ya que puede ser una url o un ID.
Define el URL ID de la siguiente manera. El componente vigila los cambios en el objeto config, y se recargará, así que puedes simplemente actualizar los ajustes "url" y "urlId".

See a full working example [here](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Run the pagination example via:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Región de la cuenta (ATENCIÓN: clientes de la UE)

If your account is located in the EU, set `region = 'eu'` in the widget configuration, for example:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Otherwise, you do not have to define `region`.