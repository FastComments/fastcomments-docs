En la sección **Pie de página** de la pestaña Código personalizado, pegue el siguiente código:

[inline-code-attrs-start title = 'Fragmento de código de comentarios en vivo de Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Pegue el código en la sección Pie de página</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Pegue el código de FastComments en la sección Pie de página" />
</div>

Después de pegar el código, haga clic en el botón **Guardar** para aplicar sus cambios.

Nota: Este código incluye lógica para colocar dinámicamente el widget de FastComments en la ubicación óptima de las entradas de su blog Typeflo.io. Otros fragmentos de código no funcionarán correctamente con la maquetación de Typeflo.io.

Recuerde reemplazar 'demo' con su ID de tenant de FastComments real después de registrarse. Si ha iniciado sesión en FastComments.com, ya debería estar reemplazado.