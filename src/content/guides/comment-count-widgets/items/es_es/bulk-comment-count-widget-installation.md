El Widget de Conteo Masivo de Comentarios esta disenado para mostrar eficientemente los conteos de comentarios de multiples paginas en la misma pagina. En lugar de realizar llamadas API individuales para cada conteo de comentarios, este widget agrupa las solicitudes para un rendimiento optimo.

## Instalacion Basica

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Como Funciona

El widget masivo funciona mediante:

1. Escanear la pagina en busca de elementos con la clase `fast-comments-count`
2. Leer el atributo `data-fast-comments-url-id` de cada elemento
3. Agrupar solicitudes API para obtener multiples conteos de comentarios eficientemente
4. Actualizar cada elemento con el conteo de comentarios apropiado

## Opciones de Configuracion

La funcion `FastCommentsCommentCountBulk` acepta las siguientes opciones de configuracion:

- **tenantId** (requerido): Tu ID de tenant de FastComments
- **apiHost** (opcional): Host API personalizado si estas usando una instancia auto-alojada

## Ejemplo del Mundo Real

Aqui hay un ejemplo practico que muestra como podrias usar el widget masivo en un listado de publicaciones de blog:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Consideraciones de Rendimiento

El widget masivo optimiza automaticamente el rendimiento mediante:

- **Agrupacion de solicitudes**: Multiples conteos de comentarios se obtienen en una sola llamada API
- **Limites de tamano de solicitud**: Las solicitudes se dividen automaticamente si la lista de URLs se vuelve demasiado grande (mas de 1,000 caracteres)
- **Deduplicacion**: Multiples elementos con el mismo `data-fast-comments-url-id` comparten el mismo conteo

## Multiples Elementos con el Mismo URL ID

Puedes tener multiples elementos en la pagina con el mismo `data-fast-comments-url-id`. Todos seran actualizados con el mismo conteo:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Localizacion

El widget masivo formatea automaticamente los conteos de comentarios segun tu configuracion de idioma de FastComments. Proporciona texto apropiado para:

- Cero comentarios
- Un comentario
- Multiples comentarios

## Cuando Usar el Widget Masivo vs Individual

**Usa el Widget Masivo cuando:**
- Tienes multiples conteos de comentarios en la misma pagina
- Estas mostrando una lista de publicaciones/articulos con conteos de comentarios
- El rendimiento es importante (reduce las llamadas API)

**Usa el Widget Individual cuando:**
- Solo necesitas un conteo de comentarios en la pagina
- Necesitas actualizaciones en vivo (el widget individual soporta actualizaciones en tiempo real)
- Quieres mas control sobre el comportamiento individual del widget
