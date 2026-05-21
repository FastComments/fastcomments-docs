Si no puedes instalar la [aplicación de Shopify App Store](https://apps.shopify.com/fastcomments), aún puedes añadir FastComments editando tu tema. Esta vía es útil cuando quieres conectar un tenant de FastComments que ya posees, o cuando estás insertando en una tienda de Shopify donde la aplicación no es una opción.

La instalación mediante la app es la vía recomendada para la mayoría de tiendas. Recurre a esto solo si la app no encaja.

### Paso 1: Desactivar los comentarios nativos de Shopify

En tu administrador de Shopify, ve a **Entradas del blog > Administrar blogs**, abre cada blog y configura **Los comentarios están desactivados** en el panel derecho. Guardar.

Esto evita que el sistema de comentarios integrado de Shopify se muestre junto a FastComments.

### Paso 2: Abre la plantilla del tema del blog

En tu administrador de Shopify:

1. Ve a **Tienda online > Temas**.
2. Bajo tu tema actual, haz clic en **Acciones > Editar código**.
3. En el explorador de archivos a la izquierda, abre **Sections** y haz clic en `main-article.liquid`.

Esta es la plantilla que Shopify renderiza para un solo artículo del blog.

### Paso 3: Pega el fragmento de FastComments

Desplázate hasta aproximadamente la línea 100 de `main-article.liquid`, justo después del cierre `</div>` del cuerpo del artículo. Pega el siguiente fragmento:

[inline-code-attrs-start title = 'Fragmento de FastComments para Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Reemplaza `"demo"` con tu propio Tenant ID desde [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Haz clic en **Guardar**.

### Paso 4: Autoriza el dominio de tu tienda

Abre una entrada del blog en tu tienda en vivo. Si ves un error de autorización en lugar del widget de comentarios, FastComments necesita saber que tu tienda tiene permiso para usar este tenant. Consulta [Errores de dominio](/guide-installation-shopify.html#shopify-domain-errors).

### Añadir FastComments a otras páginas

El mismo fragmento funciona en cualquier plantilla Liquid, incluidas las páginas de producto, páginas personalizadas y la página de inicio. Pégalo donde quieras que aparezcan los comentarios y ajusta `urlId` si quieres un identificador estable por página (por ejemplo, `urlId: "{{ product.id }}"` en una plantilla de producto).