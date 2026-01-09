En el siguiente paso necesitas copiar el código del widget predefinido que aparece a continuación.

Mientras hayas iniciado sesión en FastComments.com, el fragmento de código de abajo ya contendrá la información de tu cuenta. Vamos a copiarlo:

[inline-code-attrs-start title = 'Código de Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Limpiar la instancia existente
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Eliminar la barra superior existente si existe
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Crear nueva barra superior
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Inicializar FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Actualizar el pathname actual
            currentPathname = window.location.pathname;
        }

        // Carga inicial
        load();

        // Comprobar cambios cada 500 ms
        setInterval(() => {
            // Recargar si cambió el pathname
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Recargar si se eliminó el widget
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Recargar si el contenedor quedó vacío
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">Código pegado</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Código pegado" />
</div>

If you see a "this is a demo message" after pasting the code:

- Asegúrate de haber iniciado sesión en tu cuenta de fastcomments.com.
- Asegúrate de tener habilitadas las cookies de terceros.
- Luego actualiza esta página y copia el fragmento de código de nuevo. Debería tener `tenantId` rellenado con el identificador de tu tenant.