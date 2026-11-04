El bloque **FastComments - Reviews Summary** muestra una valoración media de estrellas y un desglose de reseñas para una página. Combínalo con el bloque **FastComments** en las plantillas de producto para el diseño estándar de reseñas: resumen arriba, formulario de reseñas y reseñas abajo.

### Requisito previo: configurar Ratings & Reviews

El bloque Reviews Summary muestra las preguntas de valoración que configuraste para tu tienda. Configúralas primero:

1. Abre la aplicación FastComments en tu administrador de Shopify.
2. Haz clic en el mosaico **Ratings & Reviews Helper** (o abre directamente [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Añade las preguntas que quieres que responda cada reseñador (valoración general de estrellas, "cómo fue el ajuste", etc.).

Sin preguntas configuradas, el bloque de resumen no tiene nada que agregar.

### Añadir el bloque

1. Abre el editor de temas de Shopify.
2. Abre la plantilla **Product** (o la plantilla de página donde quieras el resumen).
3. Haz clic en **Add block** cerca de la parte superior de la sección de página, por encima de donde estará el bloque **FastComments**.
4. En **Apps**, selecciona **FastComments - Reviews Summary**.
5. Añade un bloque **FastComments** más abajo en la misma página si aún no lo has hecho, para que los visitantes puedan dejar reseñas.
6. Haz clic en **Save**.

### Ajustes

| Ajuste | Qué hace | Valor predeterminado |
|---|---|---|
| Tenant ID (opcional) | Sobrescribe de qué tenant de FastComments lee el resumen. Déjalo en blanco para usar el tenant configurado automáticamente de la tienda. | (en blanco) |
| Custom URL ID | Sobrescribe el identificador de página contra el que agrega el resumen. Úsalo cuando el resumen esté en una página diferente al bloque FastComments que refleja. | (detectado automáticamente) |

### Cómo el resumen coincide con las reseñas

El bloque Reviews Summary usa la misma lógica de detección automática que el bloque **FastComments**:

- Plantilla de producto: `shopify-product-{product.id}`
- Plantilla de entrada de blog: `shopify-article-{article.id}`
- Otras plantillas: la ruta de la solicitud

En una página de producto normal, el resumen y el hilo de comentarios comparten automáticamente un URL ID, sin necesidad de configuración.

### Consejos

- El resumen es de solo lectura. Para recopilar reseñas, necesitas un bloque **FastComments** en la misma página.
- Si cambias las preguntas de valoración en Ratings & Reviews Helper después de recopilar reseñas, el resumen se recalcula con el nuevo conjunto de preguntas.