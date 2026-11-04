FastComments SSO (<a href="#sso">detalles aquí</a>) proporciona a sus usuarios una forma de comentar sin tener que iniciar sesión en otra plataforma.

Sin embargo, esto por sí solo no asegura sus hilos de comentarios, ya que por defecto los datos de los comentarios son información disponible públicamente: cualquiera que pueda ver la página puede ver los comentarios.

Al cambiar una configuración, podemos restringir que los comentarios se obtengan a menos que sea por un administrador o un usuario SSO válido.

#### Configuración sin código

Podemos evitar la visualización e interacción con nuestros hilos de comentarios, cuando SSO está configurado, creando una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regla de personalización</a>.

Al hacerlo, busque SSO, y encontrará esta opción:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opción Requerir SSO para ver comentarios habilitada en una regla de personalización, con la elección del nivel de seguridad'; title='Requerir SSO para ver comentarios' app-screenshot-end]

Actívela y guarde la regla de personalización.

#### Proteger solo un dominio o página específicos

Para proteger solo un dominio o página específicos, simplemente configuraremos la regla de personalización para hacerlo.

En la parte superior de la interfaz de personalización, encontraremos dos campos de entrada, Dominio y ID de URL.

Para proteger solo un dominio en particular, introduzca el dominio en cuestión en el campo "domain".

Para proteger una página en particular, introduzca la URL de la página en el campo "URL ID". Si tiene una integración personalizada con FastComments, puede introducir aquí un tipo de ID en lugar de una URL.

#### Niveles de seguridad

Al requerir SSO, querrá decidir si requiere Simple SSO o Secure SSO. Si requiere Simple SSO, entonces ambos están permitidos, pero si requiere Secure SSO, el contenido debe obtenerse con una carga útil Secure SSO hashada con su clave API para poder ser visualizado.

La opción de nivel de seguridad aparecerá cuando seleccione "Require SSO To View Comments", y solo tendrá efecto mientras esa opción esté activada.

Los ajustes <a href="#sso-simple">Verificar automáticamente los comentarios mediante Simple SSO y permitir administradores Simple SSO</a> son independientes, y se aplican tanto si esta opción está activada como si no.

#### Protección más allá de la lectura

Activar esta opción protegerá la página o dominio de ser comentada a menos que el usuario haya iniciado sesión mediante SSO.

#### Advertencias

Cualquier usuario que haya creado comentarios antes de su integración SSO no podrá verlos, a menos que inicie sesión mediante su integración SSO.