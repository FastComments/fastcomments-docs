A continuación, necesitamos identificar dónde añadir el código del widget de FastComments.com.

Si estás usando el tema predeterminado `casper`, verás una sección como esta en la línea `82`:

<div class="screenshot white-bg">
    <div class="title">Sección de comentarios deshabilitada</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Sección de comentarios deshabilitada" />
</div>

Si estás usando otros temas, no verás esto, y necesitarás añadir este código después del último `</section>`:

[inline-code-attrs-start title = 'Ejemplo de sección'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Deberías tener algo así listo:

<div class="screenshot white-bg">
    <div class="title">Plantilla lista para el código de comentarios</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Plantilla lista para el código de comentarios" />
</div>

Una vez listo, copia el código del widget de FastComments.com:

[inline-code-attrs-start title = 'Fragmento de código de comentario de FastComments.com para Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...y debería verse así:

<div class="screenshot white-bg">
    <div class="title">Añadir código de comentarios de FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Añadir código de comentarios de FastComments.com" />
</div>

Código listo. ¡Ahora solo tenemos que volver a importar nuestro tema!