Ahora que has añadido un bloque HTML personalizado, podemos agregar el código del widget de FastComments.

**Utiliza el siguiente código para Godaddy, no el código de otros tutoriales. Este código es específico para Godaddy.**

Copia el siguiente código:

[inline-code-attrs-start title = 'Fragmento de código de comentarios de Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Este fragmento de código está diseñado para ser compatible con Godaddy, y además solo se mostrará en tus publicaciones del blog, no en la página principal.

Ahora pega el código en el área `Custom Code` mencionada en `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copiar y pegar el código</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copiar y pegar el código" />
</div>

Haz clic en 'Done' en la esquina superior derecha:

<div class="screenshot white-bg">
    <div class="title">Haz clic en 'Done'</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Haz clic en 'Done'" />
</div>

¡Eso es todo para el paso dos!