[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Por defecto, el widget de FastComments se redimensionará verticalmente para ajustarse a todos los comentarios visibles. La paginación se consigue mediante un botón "Ver siguiente"
al final de la página actual, ya que hemos comprobado que esta interacción resulta más agradable para la mayoría de los usuarios.

Sin embargo, hay algunos casos en los que se prefiere el desplazamiento infinito. Por ejemplo, usamos esta función en nuestro producto Stream Chat.

Podemos ocultar los botones "Ver siguiente" y cambiar a desplazamiento infinito estableciendo la bandera **enableInfiniteScrolling** a true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Esto también requiere la adición de CSS personalizado. Añada CSS personalizado para el selector `.comments` para habilitar el desplazamiento, por ejemplo:

[inline-code-attrs-start title = 'Habilitar desplazamiento infinito'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Un ejemplo completo y funcional sería:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

En el ejemplo anterior usamos la propiedad `customCSS`; sin embargo, se sugiere usar la interfaz de configuración del Widget en su lugar por razones de rendimiento. [Consulte la documentación de CSS personalizado.](/guide-customizations-and-configuration.html#custom-css)