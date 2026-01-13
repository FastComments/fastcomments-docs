Por defecto, FastComments mostrará el nombre del usuario tal como lo introdujo, o tal como se nos pasó a través de SSO.

Sin embargo, puede ser deseable enmascarar o mostrar el nombre del usuario de una manera distinta. Por ejemplo, si el nombre del usuario es Allen Rex, quizá quieras mostrar solo "Allen R.".

Esto se puede hacer sin código en la interfaz de personalización del widget, en la opción llamada `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Los formatos disponibles son:

- Capitalizar (display example user as Example User)
- Inicial del apellido (display Example User as Example U.)
- Todas las iniciales (display Example User as E. U.)
- Mostrar "Anonymous"

El efecto de cambiar esto es inmediato. Los usuarios seguirán viendo su nombre completo en la parte superior del área de comentarios, para ellos mismos, pero sus comentarios mostrarán
el nombre modificado.

Los nombres de usuario se enmascaran del lado del servidor para proteger a los usuarios.