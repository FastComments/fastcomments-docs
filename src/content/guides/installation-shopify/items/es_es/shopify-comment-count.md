El bloque **FastComments - Recuento de comentarios** muestra un pequeño recuento de comentarios para una sola página. Úsalo en listas de entradas de blog, fichas de producto o cualquier plantilla que enlace a una página con comentarios, para que los visitantes puedan ver qué tan activo está cada hilo antes de hacer clic.

### Añadir el bloque

1. Abre el editor de temas de Shopify.
2. Abre la plantilla donde quieras que aparezca el recuento. Por ejemplo, la plantilla **Blog** (la lista de entradas) o una sección de listado de productos.
3. Haz clic en **Añadir bloque** en la sección que muestra cada elemento.
4. En **Apps**, selecciona **FastComments - Recuento de comentarios**.
5. Haz clic en **Guardar**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Anula de qué tenant de FastComments se lee el recuento. Déjalo en blanco para usar el tenant configurado automáticamente para la tienda. | (vacío) |
| Custom URL ID | Anula el identificador de página que busca el recuento. Úsalo cuando el recuento esté en una página diferente del bloque FastComments que rastrea. | (detectado automáticamente) |

### How the count matches the comment thread

El bloque Recuento de comentarios usa la misma lógica de detección automática que el bloque **FastComments**:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Si estableces un **Custom URL ID** en el bloque **FastComments** en una página, configura el mismo Custom URL ID en el bloque Recuento de comentarios para que apunten al mismo hilo.

### Tips

- Los recuentos de todos los elementos de la página se obtienen en una sola petición, por lo que agregar el bloque a cada elemento de una lista larga no añade coste adicional de ida y vuelta.
- El uso esperado es un bloque Recuento de comentarios por artículo o producto en un listado; el bloque puede añadirse tantas veces como necesites.