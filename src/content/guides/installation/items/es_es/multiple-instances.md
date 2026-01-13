Cada instancia del widget de comentarios está aislada. Por esta razón, FastComments admite de forma nativa más de una instancia por página, o múltiples instancias apuntando al mismo hilo de chat.

En el caso de la biblioteca VanillaJS, por ejemplo, simplemente tiene que vincular el widget de comentarios a diferentes nodos DOM. Si desea simplemente actualizar el hilo actual en la página, consulte [Cambiar hilos de comentarios sin recargar la página](guide-customizations-and-configuration.html#switching-comment-threads);

### Sincronización del estado de autenticación entre múltiples instancias

Veamos el ejemplo de una aplicación de página única personalizada que es una lista de preguntas frecuentes con su propio hilo de comentarios.

En este caso, tenemos múltiples instancias de FastComments en el DOM al mismo tiempo.

Esto está bien, pero plantea algunos desafíos para la experiencia del usuario.

Considere este flujo:

1. El usuario visita la página con una lista de preguntas, cada una con su propio widget de comentarios.
2. El usuario ingresa su nombre de usuario y correo electrónico y deja una pregunta en uno de los hilos.
3. Ve otro elemento de FAQ sobre el que tiene una pregunta.
4. Va a comentar de nuevo. ¿Tiene que ingresar su correo electrónico y nombre de usuario nuevamente?

En este caso, FastComments maneja la sincronización del estado de autenticación entre instancias del widget por usted. En el paso cuatro, el usuario ya estará autenticado temporalmente ya que ingresó su nombre de usuario y correo electrónico en la misma página.
