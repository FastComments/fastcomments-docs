En FastComments llamamos al article id, o a la página a la que se vinculan los comentarios, el URL ID ya que puede ser una url o un ID.
Defina el URL ID de la siguiente manera. El componente observa los cambios en el objeto config y se recargará, por lo que puede actualizar el URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Región de la cuenta (ATENCIÓN: clientes de la UE)

Si su cuenta está ubicada en la UE, establezca `region = 'eu'` en la configuración del widget, por ejemplo:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

De lo contrario, no tiene que definir `region`.