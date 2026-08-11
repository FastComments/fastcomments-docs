---
Por defecto, FastComments mostrará el nombre del usuario tal como lo ingresó, o como se le pasó a través de SSO.

Sin embargo, puede ser deseable enmascarar o mostrar el nombre del usuario de una manera diferente. Por ejemplo, si el nombre del usuario es Allen Rex, quizás quieras mostrar solo "Allen R.".

Esto se puede hacer sin código en la interfaz de personalización del widget, bajo la configuración llamada `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Desplegable de Formato de Nombre del Comentador abierto con opciones como Capitalizar, Inicial del Apellido y Todas las Iniciales'; title='Cambiar Formato de Nombre' app-screenshot-end]

Los formatos disponibles son:

- Capitalizar (mostrar al usuario de ejemplo como Example User)
- Inicial del Apellido (mostrar Example User como Example U.)
- Todas las Iniciales (mostrar Example User como E. U.)
- Mostrar "Anonymous"

El efecto de cambiar esto es inmediato. Los usuarios seguirán viendo su nombre de usuario completo en la parte superior del área de comentarios, para ellos mismos, pero sus comentarios mostrarán el nombre de usuario modificado.

Los nombres de usuario se enmascaran del lado del servidor para proteger a los usuarios.

---