[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments está diseñado para ser personalizado. El widget de comentarios en sí se ejecuta dentro de un iframe por razones de seguridad, por lo que para aplicar estilos personalizados debes seguir uno de dos enfoques.

El primero, el enfoque más fácil y el que preferimos, es usar la [página de personalización del widget](https://fastcomments.com/auth/my-account/customize-widget).

En la página de personalización del widget, ve la sección "Mostrar opciones avanzadas", bajo la cual hay un área etiquetada como "CSS personalizado":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Editor de CSS personalizado bajo Mostrar opciones avanzadas en la página de personalización del widget'; title='Área de entrada de CSS personalizado' app-screenshot-end]

Este enfoque tiene algunos beneficios:
1. El CSS ingresado se minifica antes de enviarse al usuario, y el formato se mantiene consistente en la interfaz de edición.
2. Obtienes todos los beneficios de la interfaz de personalización del widget, por ejemplo, personalizar fácilmente el widget de comentarios de forma diferente para distintos sitios.
3. Cuando realicemos cambios en el widget de comentarios, tu estilo personalizado será probado como parte de nuestro proceso de lanzamiento.

El segundo enfoque es especificar el parámetro **customCSS** en la configuración del widget, de la siguiente manera:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Pasando CSS personalizado'; code-example-end]

Sin embargo, esto tiene *limitaciones*:
1. Existe un límite a la cantidad de CSS personalizado que se puede pasar antes de que nuestros servidores rechacen la solicitud, debido al tamaño de los encabezados.
2. Debes gestionar el CSS personalizado en tu infraestructura y sistema de compilación. Esto también puede ser una ventaja más que una desventaja.
3. Hay una sobrecarga adicional al enviar el CSS personalizado a través de la red **dos veces** en este caso, ya que debe enviarse a nuestros servidores y luego devolverse en el contenido del iframe. Sin embargo, para la mayoría de los tamaños de carga, esto no es perceptible.
4. Una optimización común es minificar el CSS para reducir su tamaño en la red, sin embargo con este enfoque tendrás que manejarlo tú mismo.
5. Tu CSS personalizado no será probado cuando realicemos cambios.

### Archivos CSS externos

¡Puedes indicarle al widget que obtenga un archivo externo usando `@import`!

Se recomienda colocar el `@import` en una regla de personalización. De esta manera, si alguna vez necesitamos hacer un cambio en el widget de comentarios, podemos usar nuestras herramientas de automatización para verificar tu configuración. Por ejemplo, crearías una regla de personalización en la interfaz de personalización del widget, harías clic en `Avanzado` y escribirías en `CSS personalizado`:

    @import url(https://example.com/styles.css);

#### En código - No recomendado

También puedes cargar un archivo CSS externo mediante la propiedad `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Archivo CSS externo'; code-example-end]

Sin embargo, recuerda que tu CSS no podrá ser probado por nosotros si haces esto.

### Estilizado del modal de perfil de usuario

Los modales de perfil de usuario también pueden estilizarse con CSS personalizado. Sin embargo, para garantizar que el estilo personalizado se aplique a los perfiles de usuario, todos los selectores CSS deben llevar el prefijo `.user-profile`. Sin este prefijo, el estilo personalizado será ignorado para los modales de perfil de usuario.

Por ejemplo:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS de perfil de usuario'; code-example-end]

### Compatibilidad hacia atrás

En FastComments, sabemos que nuestros clientes personalizan el widget de comentarios. Eso es intencional: lo último que queremos es que nuestro producto cause inconsistencias de diseño en tu producto.

Dado que esto es una parte importante de nuestro producto, contamos con una canalización de compilación que nos permite revisar los cambios al widget de comentarios, por cliente, en cada lanzamiento.

Si encontramos problemas menores, actualizaremos tu cuenta para garantizar que nuestro lanzamiento se realice sin problemas. Si vemos cambios importantes que rompan la funcionalidad, esto nos permite detener el lanzamiento.

---