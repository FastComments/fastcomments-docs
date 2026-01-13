[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Para los sitios que permiten alternar el modo oscuro después de la carga inicial de la página, esto es un poco más complejo.

Primero, todas las versiones actuales de la biblioteca del widget de comentarios (React, Vue) tienen ejemplos de alternancia del modo oscuro en sus respectivos repositorios.

Para el widget VanillaJS, necesitaremos hacer un poco más de trabajo. Primero, FastCommentsUI devuelve un objeto con las funciones "destroy" y "update".

Podemos simplemente llamar a la función update cada vez que queramos actualizar la configuración del widget de comentarios, como sigue. Aquí hay un ejemplo completamente funcional de alternancia del modo oscuro con el widget VanillaJS.

[inline-code-attrs-start title = 'Ejemplo completo de alternancia del modo oscuro'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---