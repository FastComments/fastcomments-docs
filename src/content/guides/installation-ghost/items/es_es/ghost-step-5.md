A continuación necesitamos identificar dónde añadir el código del widget de FastComments.com.

Si estás usando el tema por defecto `casper`, verás una sección como esta en la línea `82`:

<div class="screenshot white-bg">
    <div class="title">Disabled Comment Section</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Disabled Comment Section" />
</div>

Si usas otros temas, no verás esto, y necesitarás añadir este código después del último `</section>`:

[inline-code-attrs-start title = 'Ejemplo de sección'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Deberías tener algo como esto listo:

<div class="screenshot white-bg">
    <div class="title">Template Ready For Comment Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Template Ready For Comment Code" />
</div>

Una vez listo, copia el código del widget de FastComments.com:

[inline-code-attrs-start title = 'Fragmento de código de comentarios de Ghost para FastComments.com'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
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

        window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        }];
    })();
</script>
[inline-code-end]

...y debería verse así:

<div class="screenshot white-bg">
    <div class="title">Add FastComments.com Comment Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Add FastComments.com Comment Code" />
</div>

Código hecho. ¡Ahora solo tenemos que volver a importar nuestro tema!