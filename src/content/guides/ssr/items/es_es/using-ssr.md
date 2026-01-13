Para usar FastComments SSR, el cliente puede obtener HTML desde el endpoint `https://fastcomments.com/ssr/comments`.

Esto se puede hacer de varias maneras.

### Con WordPress

SSR está habilitado por defecto para los usuarios sin JS habilitado como alternativa en el plugin de WordPress desde la versión `3.10.2`.

### En una página web

Con una aplicación ya existente, SSR se puede agregar con el [siguiente ejemplo](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), suponiendo que el lenguaje usado sea PHP:

[inline-code-attrs-start title = 'Ejemplo de SSR basado en PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

También podemos mostrar la interfaz SSR solo cuando el usuario tiene JS deshabilitado:

[inline-code-attrs-start title = 'Ejemplo de fallback SSR basado en PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Para un ejemplo que usa SSO, [vea aquí](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Con contenido pre-renderizado

Nuestro blog se genera en tiempo de compilación y ofrece un [buen ejemplo de SSR con Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Los parámetros básicos

Los parámetros básicos que necesita pasar son:
- `tenantId` - Esto lo identifica como cliente.
- `urlId` - Esto identifica la página o artículo para cargar comentarios y define dónde se guardan.
- `url` - Esto se usa para notificaciones y funciones relacionadas para enlazar de vuelta al hilo de comentarios.

### Estilos personalizados

La versión SSR del widget de comentarios usa la misma estructura y motor de renderizado que la versión de JavaScript.

Por lo tanto, todos los estilos personalizados que funcionan para el widget de comentarios de JavaScript funcionan para SSR. 

### Notas

Con SSR, no hay JavaScript para controlar la altura del contenedor renderizado. En los navegadores, puede aparecer una barra de desplazamiento vertical para discusiones largas.

Por lo tanto, debe ajustarlo según lo desee.