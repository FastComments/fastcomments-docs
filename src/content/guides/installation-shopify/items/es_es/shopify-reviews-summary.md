El bloque **FastComments - Reviews Summary** muestra una calificación agregada por estrellas y un desglose de reseñas para una página. Combínalo con el bloque **FastComments** en las plantillas de producto para el diseño estándar de reseñas: resumen arriba, formulario de reseñas y reseñas abajo.

### Prerequisite: set up Ratings & Reviews

El bloque Reviews Summary muestra las preguntas de valoración que configuraste para tu tienda. Configúralas primero:

1. Abre la app FastComments en tu administrador de Shopify.
2. Haz clic en el mosaico **Ratings & Reviews Helper** (o abre [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) directamente).
3. Añade las preguntas que quieras que responda cada reseñador (calificación general por estrellas, "cómo te quedó", etc.).

Sin preguntas configuradas, el bloque de resumen no tiene nada que agregar.

### Add the block

1. Abre el editor de temas de Shopify.
2. Abre la plantilla **Product** (o la plantilla de página donde quieras el resumen).
3. Haz clic en **Add block** cerca de la parte superior de la sección de la página, por encima de donde irá el bloque **FastComments**.
4. Bajo **Apps**, selecciona **FastComments - Reviews Summary**.
5. Añade un bloque **FastComments** más abajo en la misma página si aún no lo has hecho, para que los visitantes puedan dejar reseñas.
6. Haz clic en **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Anula de qué tenant de FastComments lee el resumen. Deja en blanco para usar el tenant configurado automáticamente de la tienda. | (en blanco) |
| Custom URL ID | Anula el identificador de página contra el que se agrega el resumen. Úsalo cuando el resumen esté en una página diferente del bloque FastComments al que hace referencia. | (detectado automáticamente) |

### How the summary matches the reviews

El bloque Reviews Summary usa la misma lógica de detección automática que el bloque **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

En una página de producto normal, el resumen y el hilo de comentarios comparten un ID de URL automáticamente, sin necesidad de configuración.

### Tips

- El resumen es de solo lectura. Para recopilar reseñas, necesitas un bloque **FastComments** en la misma página.
- Si cambias las preguntas de valoración en Ratings & Reviews Helper después de haber recopilado reseñas, el resumen se recalcula con el nuevo conjunto de preguntas.