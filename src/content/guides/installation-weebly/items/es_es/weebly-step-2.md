Para que la integración entre Weebly y FastComments funcione correctamente, debemos agregar **dos** pequeños fragmentos de código.

El primer fragmento sirve para ocultar el mensaje de Weebly "Comments are Closed", y el segundo es para cargar FastComments.

Primero, copia este pequeño fragmento de código:

[inline-code-attrs-start title = 'Fragmento de código de encabezado de FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Luego, en la misma página de configuración de `Step One`, haz clic en el `+` junto a `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Abrir código de encabezado de la entrada</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Abrir código de encabezado de la entrada" />
</div>

Deberías ver que se abre un cuadro de texto así:

<div class="screenshot white-bg">
    <div class="title">Código del encabezado de la entrada abierto</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Código del encabezado de la entrada abierto" />
</div>

Ahora vamos a pegar nuestro fragmento de código:

<div class="screenshot white-bg">
    <div class="title">Fragmento de código de encabezado pegado</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Fragmento de código de encabezado pegado" />
</div>

A continuación está el código de pie de página para habilitar FastComments. Haz clic en el signo más junto a `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Abrir código de pie de página de la entrada</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Abrir código de pie de página de la entrada" />
</div>

Copia este fragmento de código que está diseñado **específicamente para Weebly**:

[inline-code-attrs-start title = 'Fragmento de código de pie de página de FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // eliminar el botón de mostrar comentarios
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Ahora pega nuestro código de pie de página:

<div class="screenshot white-bg">
    <div class="title">Código de pie de página de la entrada agregado</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Código de pie de página de la entrada agregado" />
</div>

¡Eso es todo!