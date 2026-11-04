En FastComments llamamos al id del artículo, o a la página a la que se vinculan los comentarios, el URL ID, ya que puede ser una url o un ID.
Define el URL ID de la siguiente manera. El componente observa los cambios en el objeto config y se recargará, por lo que puedes simplemente actualizar las opciones "url" y "urlId".

Consulta un ejemplo completo y funcional [aquí](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Ejecuta el ejemplo de paginación mediante:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Región de la cuenta (ATENCIÓN: Clientes de la UE)

Si tu cuenta está ubicada en la UE, establece `region = 'eu'` en la configuración del widget, por ejemplo:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

De lo contrario, no tienes que definir `region`.