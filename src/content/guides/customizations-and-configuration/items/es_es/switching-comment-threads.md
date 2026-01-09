[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Hemos explicado cómo `urlId` es el identificador de la página o artículo al que están vinculados los comentarios.

Además, para recapitular, si no se define, `urlId` tomará por defecto la URL de la página actual.

¿Qué ocurre con las SPA, o Single-Page-Applications, donde la página o el contenido al que están vinculados los comentarios cambia
dinámicamente sin recargar la página?

#### Angular, React, Vue, etc

Con nuestras librerías como Angular y React, simplemente actualizar la propiedad `urlId` que se pasa al widget
hará que el widget de comentarios se actualice. Puedes ver esto en acción para la aplicación React, por ejemplo, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">aquí</a>.

#### VanillaJS

Si estás usando la librería VanillaJS es un poco más complicado ya que no hay un framework como Angular o React
para gestionar el enlace de datos o la propagación del estado.

Cuando instancias el widget de VanillaJS, devuelve algunas funciones que se pueden llamar para actualizarlo.

Aquí hay un ejemplo funcional donde cambiamos el hash de la página y actualizamos el widget de comentarios:

[inline-code-attrs-start title = 'Ejemplo de cambio del hash de la página'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // También actualizamos url, para que las notificaciones puedan enlazar de nuevo a la página correcta
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---