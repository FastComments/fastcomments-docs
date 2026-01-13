[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments está diseñado para ser personalizado. El widget de comentarios en sí se ejecuta dentro de un iframe por razones de seguridad, por lo que para aplicar estilos personalizados debes seguir una de dos aproximaciones.

La primera, la más sencilla y la que preferimos, es usar la [página de personalización del widget](https://fastcomments.com/auth/my-account/customize-widget).

En la página de personalización del widget, consulta la sección "Show Advanced Options" (Mostrar opciones avanzadas), bajo la cual hay un área etiquetada "Custom CSS" (CSS personalizado):

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Este enfoque tiene algunas ventajas:
1. El CSS introducido se minifica antes de enviarse al usuario, y el formato se mantiene consistente en la interfaz de edición.
2. Obtienes todos los beneficios de la interfaz de personalización del widget, por ejemplo personalizar fácilmente el widget de comentarios de forma diferente para distintos sitios.
3. Cuando hacemos cambios en el widget de comentarios, tus estilos personalizados se probarán como parte de nuestro proceso de lanzamiento.

La segunda aproximación es especificar el parámetro **customCSS** en la configuración del widget, como sigue:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Sin embargo, esto tiene *limitaciones*:
1. Hay un límite de cuánto CSS personalizado se puede pasar antes de que nuestros servidores rechacen la solicitud, debido al tamaño de los encabezados.
2. Debes gestionar el CSS personalizado en tu infraestructura y sistema de compilación. Esto también puede ser una ventaja en lugar de un inconveniente.
3. Hay una sobrecarga adicional de enviar el CSS personalizado por la red **dos veces** en este caso de uso, ya que tiene que enviarse a nuestros servidores y luego devolverse en el contenido del iframe. Sin embargo, para la mayoría de tamaños de carga útil, esto no se percibe.
4. Una optimización común es minificar el CSS para reducir su tamaño en la red; sin embargo, con este enfoque tendrás que encargarte de eso.
5. Tu CSS personalizado no será probado cuando hagamos cambios.

### External CSS Files

Puedes indicar al widget que obtenga un archivo externo usando `@import`!

Se recomienda poner el `@import` en una regla de personalización. De este modo, si alguna vez necesitamos hacer un cambio en el widget de comentarios, podemos usar nuestras herramientas de automatización para verificar tu configuración. Por ejemplo, crearías una regla de personalización en la interfaz de personalización del widget, harías clic en `Advanced` y escribirías en `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

También puedes cargar un archivo CSS externo mediante la propiedad `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Sin embargo, recuerda que tu CSS no podrá ser probado por nosotros si haces esto. 

### User Profile Modal Styling

Los modales de perfil de usuario también pueden estilizarse con CSS personalizado. Sin embargo, para asegurarte de que el estilo personalizado se aplique a los perfiles de usuario, todos los selectores CSS deben llevar el prefijo `.user-profile`. Sin este prefijo, el estilo personalizado será ignorado en los modales de perfil de usuario.

Por ejemplo:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

En FastComments, sabemos que nuestros clientes personalizan el widget de comentarios. Eso es por diseño: lo último que queremos es que nuestro producto provoque inconsistencias de diseño en tu producto.

Dado que esto es una parte importante de nuestro producto, tenemos una canalización de compilación que nos permite revisar los cambios del widget de comentarios por cliente en cada versión.

Si encontramos problemas menores, actualizaremos tu cuenta para asegurar que nuestro lanzamiento se realice sin contratiempos. Si vemos cambios mayores que rompan la funcionalidad, esto nos permite detener el lanzamiento.

---