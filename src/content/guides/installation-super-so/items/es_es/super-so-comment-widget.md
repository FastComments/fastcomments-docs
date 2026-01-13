## Añadiendo un Widget de Comentarios en Vivo a tus artículos de Notion en Super.so

Además de Collab Chat, puedes añadir un widget de comentarios tradicional al final de tus artículos de Notion. Esto permite que los lectores dejen comentarios y mantengan discusiones sobre todo el artículo.

### Pasos de instalación

Copia el siguiente código y pégalo en la sección `Body` de los ajustes de tu sitio en `Super.so`:

[inline-code-attrs-start title = 'Widget de Comentarios en Vivo de FastComments para Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Limpiar instancia existente
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Crear nuevo elemento objetivo
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inicializar FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Actualizar la ruta actual
            currentPathname = window.location.pathname;
        }

        // Carga inicial
        load();

        // Comprobar cambios cada 500 ms
        setInterval(() => {
            // Volver a cargar si la ruta cambió
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Volver a cargar si el widget fue eliminado
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Volver a cargar si el contenedor se vació
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Notas importantes

- El widget de comentarios aparecerá al final de tus artículos de Notion
- Cada página obtiene su propio hilo de comentarios único basado en la ruta de la URL
- Asegúrate de reemplazar `"demo"` con tu ID de tenant real de tu cuenta de FastComments
- El widget gestiona automáticamente la carga dinámica de páginas de Super.so

---