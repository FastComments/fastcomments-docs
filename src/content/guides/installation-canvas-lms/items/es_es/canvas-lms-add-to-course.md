#### Cómo aparecen los comentarios en sus cursos

Una vez que la integración LTI está habilitada y la Aplicación externa está instalada, FastComments funciona automáticamente según las ubicaciones que configuró:

#### Vista de la tarea

Si la colocación **Assignment View** está habilitada, los comentarios aparecen automáticamente debajo de cada tarea del curso. Estudiantes e instructores ven una sección de comentarios en hilos cuando visualizan una tarea: no se necesita configuración extra por tarea.

Cada tarea obtiene su propio hilo de comentarios separado.

#### Botón del Editor de contenido enriquecido

Si la colocación **Editor Button** está habilitada, los instructores pueden incrustar FastComments en cualquier contenido que use el Editor de contenido enriquecido:

1. Edite una **Página**, un **Cuestionario** o un **Anuncio**.
2. En la barra de herramientas del Editor de contenido enriquecido, haga clic en el botón **FastComments**.
3. FastComments se incrusta automáticamente en el contenido.
4. Guarde la página.

Cuando los estudiantes ven la página, el widget de FastComments incrustado carga un hilo de comentarios único para esa página.

#### SSO automático

En ambas colocaciones, los estudiantes inician sesión automáticamente a través de su cuenta de Canvas. Los nombres, correos electrónicos y avatares se sincronizan mediante el lanzamiento LTI; no se necesita un inicio de sesión separado.

#### Restringir el acceso público (recomendado)

Por defecto, los datos de comentarios de FastComments son legibles públicamente. Cualquiera que pueda adivinar la URL de un hilo o el endpoint de la API puede ver sus comentarios, incluso fuera de Canvas. Para las discusiones del curso, casi con toda seguridad querrá restringir la visualización solo a los estudiantes inscritos.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse a través del lanzamiento LTI firmado.

Consulte [Cómo proteger los hilos de comentarios con inicio de sesión único](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para el recorrido completo, incluyendo cómo delimitar la regla a un único dominio o página.