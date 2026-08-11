FastComments SSO (<a href="#sso">detalles aquí</a>) proporciona a sus usuarios una forma de comentar sin tener que iniciar sesión en otra plataforma.

Sin embargo, esto por sí solo no asegura sus hilos de comentarios, ya que por defecto los datos de los comentarios son información disponible públicamente: cualquiera que pueda ver la página puede ver los comentarios.

Al cambiar una configuración, podemos restringir que los comentarios se obtengan a menos que lo haga un administrador o un usuario SSO válido.

#### No-Code Setup

Podemos evitar la visualización e interacción con nuestros hilos de comentarios, cuando SSO está configurado, creando una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regla de personalización</a>.

Al hacerlo, busque SSO, y encontrará esta opción:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opción de requerir SSO para ver comentarios habilitada en una regla de personalización, con la elección del nivel de seguridad'; title='Requerir SSO para ver comentarios' app-screenshot-end]

Actívela y guarde la regla de personalización.

#### Only Protect a Certain Domain or Page

Para proteger solo un dominio o página específicos, simplemente configuraremos la regla de personalización para hacerlo.

En la parte superior de la interfaz de personalización, encontraremos dos campos de entrada, Dominio y ID de URL.

Para proteger solo un dominio en particular, introduzca el dominio en cuestión en el campo "domain".

Para proteger una página en particular, introduzca la URL de la página en el campo "URL ID". Si tiene una integración personalizada con FastComments, puede introducir aquí un tipo de ID en lugar de una URL.

#### Security Levels

Al requerir SSO, querrá decidir si requiere SSO Simple o SSO Seguro. Si requiere SSO Simple, entonces ambos están permitidos, pero si requiere SSO Seguro, el contenido debe obtenerse con una carga útil de SSO Seguro hashada con su clave API para poder ser visualizado.

La opción de nivel de seguridad aparecerá cuando seleccione "Require SSO To View Comments".

#### Protection Beyond Reading

Activar esta opción protegerá la página o dominio de ser comentado a menos que el usuario haya iniciado sesión mediante SSO.

#### Gotchas

Cualquier usuario que haya creado comentarios antes de su integración SSO no podrá verlos, a menos que inicie sesión mediante su integración SSO.