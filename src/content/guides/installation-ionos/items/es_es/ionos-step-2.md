A continuación vamos a añadir el código del widget FastComments a su sitio. Este código buscará todos los formularios con el título `FastComments Goes Here` y
los reemplazará por FastComments.

Así que vamos a `Settings` en la esquina inferior izquierda del editor del sitio:

<div class="screenshot white-bg">
    <div class="title">Abrir Configuración</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Abrir Configuración" />
</div>

Abra la sección `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Abrir Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Abrir Custom Head Code" />
</div>

Para Ionos necesitamos una versión **especial** del código del widget FastComments. Los fragmentos de código de **otros tutoriales no funcionarán.**

Ahora copie el siguiente código:

[inline-code-attrs-start title = 'Fragmento de FastComments para Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // obtenemos el elemento que no ocupa todo el ancho
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...y pégalo como se muestra:

<div class="screenshot white-bg">
    <div class="title">Pegar y guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Pegar y guardar" />
</div>