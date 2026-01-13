Ahora que estamos en el editor de plantillas, debemos decidir dónde queremos mostrar los comentarios o el chat en directo.

En este ejemplo lo añadiremos directamente debajo del vídeo. Pasa el cursor sobre el elemento para añadir el widget al final, y haz clic en `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Añadir elemento</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Añadir elemento" />
</div>

Select `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Seleccionar CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Seleccionar CUSTOM JS/HTML" />
</div>

Ahora abramos el editor de código donde pegaremos nuestro código.

ClickFunnels es un poco confuso en el siguiente paso.

Es importante que *NO* selecciones `Code` cuando pases el cursor sobre el nuevo elemento. En su lugar, selecciona `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Seleccionar SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Seleccionar SETTINGS" />
</div>

Ahora, en el lado derecho podemos hacer clic en `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Haz clic en Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Haz clic en Open Code Editor" />
</div>

Verás que se abre un gran recuadro. Aquí es donde podemos pegar nuestro código. Copia el siguiente fragmento (usa el botón de copiar en la esquina superior derecha):

[inline-code-attrs-start title = 'Fragmento de código de chat en streaming de ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // algunos proveedores cambian el fragmento de código para que sea asíncrono
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Este fragmento de código es para nuestro producto Streaming Chat, que va bien con vídeos. Si quieres el fragmento de código del widget Live Commenting en su lugar, que funciona mejor con páginas normales o entradas de blog, está al final de este tutorial.

Cuando peguemos el fragmento de código en la ventana, debería verse así:

<div class="screenshot white-bg">
    <div class="title">Pegar código</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Pegar código" />
</div>

Ahora solo tenemos que cerrar el cuadro:

<div class="screenshot white-bg">
    <div class="title">Cerrar</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Cerrar" />
</div>

¡Ahora puedes previsualizar tus cambios! Siéntete libre de mover el widget y ver dónde te gusta más.

<div class="screenshot white-bg">
    <div class="title">Previsualizar</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Previsualizar" />
</div>

¡Éxito! ¡No olvides probar en móvil!

<div class="screenshot white-bg">
    <div class="title">¡Éxito!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="¡Éxito!" />
</div>