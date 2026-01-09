FastComments SSO (<a href="#sso">detalles aquí</a>) proporciona a tus usuarios una forma de comentar sin tener que iniciar sesión en otra plataforma.

Sin embargo, esto por sí solo no protege tus hilos de comentarios, ya que por defecto los datos de los comentarios son información pública: cualquiera que pueda ver la página puede ver los comentarios.

Al cambiar una configuración, podemos restringir la obtención de comentarios a menos que sea por un administrador o un usuario SSO válido.

#### No-Code Setup

Podemos evitar que se vean e interactúe con nuestros hilos de comentarios, cuando SSO está configurado, creando una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regla de personalización</a>.

Al hacerlo, busca SSO y encontrarás esta opción:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Actívala y guarda la regla de personalización.

#### Solo proteger un dominio o una página determinada

Para proteger únicamente un dominio o una página concreta, simplemente configuraremos la regla de personalización para hacerlo.

En la parte superior de la interfaz de personalización, encontraremos dos campos, Domain y URL ID.

Para proteger solo un dominio en particular, introduce el dominio en cuestión en el campo "domain".

Para proteger una página concreta, introduce la URL de la página en el campo "URL ID". Si tienes una integración personalizada con FastComments, puedes introducir aquí un tipo de ID en lugar de una URL.

#### Niveles de seguridad

Al exigir SSO, querrás decidir si requieres Simple SSO o Secure SSO. Si requieres Simple SSO, entonces se permiten ambos, pero si requieres Secure SSO entonces
el contenido debe recuperarse con una carga útil (payload) de Secure SSO hasheada con tu API key para poder ser visualizado.

La opción de nivel de seguridad aparecerá cuando selecciones "Requerir SSO para ver comentarios".

#### Protección más allá de la lectura

Habilitar esta opción protegerá la página o el dominio para que no se pueda comentar a menos que el usuario haya iniciado sesión vía SSO.

#### Advertencias

Cualquier usuario que haya creado comentarios antes de tu integración SSO no podrá verlos, a menos que inicie sesión mediante tu integración SSO.