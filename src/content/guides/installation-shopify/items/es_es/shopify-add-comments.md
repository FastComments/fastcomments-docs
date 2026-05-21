---
El **FastComments** block es el widget principal de comentarios. Agrégalo a las plantillas de entradas del blog, plantillas de producto o cualquier otra página donde quieras un hilo de discusión o un chat en vivo.

### Agregar el bloque

1. Abre el editor de temas de Shopify (**Tienda online > Temas > Personalizar**).
2. Elige la plantilla en la que quieres que aparezcan los comentarios: **Entrada de blog**, **Producto**, o cualquier otra plantilla de página o sección.
3. En la sección donde quieres que aparezcan los comentarios, haz clic en **Agregar bloque**.
4. Bajo **Apps**, selecciona **FastComments**.
5. Haz clic en **Guardar**.

El bloque aparece de inmediato. No hay un Tenant ID que introducir; el tenant de tu tienda se configura automáticamente cuando instalas la aplicación.

### Configuración

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Anula con qué tenant de FastComments procesa el bloque. Déjalo en blanco para usar el tenant configurado automáticamente de la tienda. Encuentra un manual tenant ID en fastcomments.com/auth/my-account/api-secret. | (vacío) |
| SSO | Inicia sesión automáticamente al visitante como su cuenta de cliente de Shopify antes de comentar. Ver [Inicio de sesión automático de clientes de Shopify](/guide-installation-shopify.html#shopify-sso). | Activado |
| Commenting Style | **Threaded** para respuestas anidadas y votos, o **Streaming** para un feed de chat en tiempo real. | Threaded |
| Custom URL ID | Anula el identificador de página detectado automáticamente. Úsalo cuando quieras que dos URL compartan un mismo hilo de comentarios. | (detectado automáticamente) |

### Cómo se elige el identificador de la página

Cada hilo de comentarios está identificado por un URL ID. El bloque selecciona uno automáticamente:

- **Plantilla de entrada de blog:** `shopify-article-{article.id}`, que es estable frente a cambios en el slug y el título.
- **Plantilla de producto:** `shopify-product-{product.id}`, que es estable frente a cambios en el slug y el título.
- **Otras plantillas:** la ruta de la petición.

Si configuras **Custom URL ID**, se usará ese valor en su lugar. Usa el mismo Custom URL ID en varios bloques (por ejemplo, en una variante localizada de una página de producto) para compartir un solo hilo de comentarios.

### Threaded vs Streaming

**Threaded** es el valor predeterminado. Los visitantes se responden entre sí, votan y las herramientas de moderación funcionan como se espera. Es ideal para entradas de blog y reseñas de productos.

**Streaming** elimina el threading y muestra los comentarios nuevos en tiempo real a medida que se publican, como un feed de chat. Es ideal para lanzamientos de producto, eventos en vivo y páginas comunitarias.

### Múltiples bloques en la misma página

El bloque se puede añadir más de una vez a la misma plantilla. Por ejemplo, un resumen de reseñas en la parte superior de una página de producto y un bloque FastComments en la parte inferior. Los bloques comparten un URL ID, por lo que el resumen refleja los comentarios de abajo.

### Consejos

- El bloque se oculta en la vista previa del editor de temas con un aviso amarillo si no encuentra un tenant. Si eso aparece en tu tienda en vivo, reinstala la aplicación FastComments.
- Para una página de producto, el bloque FastComments funciona también como tu widget de reseñas de producto. Combínalo con **FastComments - Reviews Summary** para un resumen de calificaciones con estrellas en la parte superior de la página.

---