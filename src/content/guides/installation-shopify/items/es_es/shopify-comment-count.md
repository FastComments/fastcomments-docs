El bloque **FastComments - Comment Count** muestra un pequeño contador de comentarios para una sola página. Úsalo en listas de entradas de blog, tarjetas de producto o cualquier plantilla que enlace a una página con comentarios, para que los visitantes puedan ver la actividad de cada hilo antes de hacer clic.

### Añadir el bloque

1. Abre el editor de temas de Shopify.
2. Abre la plantilla donde quieres que aparezca el contador. Por ejemplo, la plantilla **Blog** (la lista de entradas) o una sección de listado de productos.
3. Haz clic en **Add block** en la sección que muestra cada elemento.
4. En **Apps**, selecciona **FastComments - Comment Count**.
5. Haz clic en **Save**.

### Ajustes

| Ajuste | Qué hace | Valor por defecto |
|---|---|---|
| Tenant ID (opcional) | Sobrescribe de qué tenant de FastComments lee el contador. Déjalo en blanco para usar el tenant configurado automáticamente de la tienda. | (en blanco) |
| Custom URL ID | Sobrescribe el identificador de página que busca el contador. Úsalo cuando el contador esté en una página diferente al bloque de FastComments que rastrea. | (detectado automáticamente) |

### Cómo el contador coincide con el hilo de comentarios

El bloque Comment Count usa la misma lógica de detección automática que el bloque **FastComments**:

- Plantilla de entrada de blog: `shopify-article-{article.id}`
- Plantilla de producto: `shopify-product-{product.id}`
- Otras plantillas: la ruta de la solicitud

Si estableces un **Custom URL ID** en el bloque **FastComments** de una página, establece el mismo Custom URL ID en el bloque Comment Count para que apunten al mismo hilo.

### Consejos

- Los contadores de todos los elementos de la página se obtienen en una sola solicitud, por lo que añadir el bloque a cada elemento de una lista larga no tiene coste adicional de ida y vuelta.
- Un bloque Comment Count por artículo o producto en un listado es el uso esperado; el bloque puede añadirse tantas veces como necesites.