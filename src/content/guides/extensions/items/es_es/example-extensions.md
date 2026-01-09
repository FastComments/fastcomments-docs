En FastComments, escribimos nuestras propias extensiones, usando la misma API. Puedes ver el código sin minimizar
para estas extensiones en los siguientes endpoints:

#### Modo Oscuro

La extensión Modo Oscuro se carga de forma condicional cuando se detecta una página "oscura". Esta extensión simplemente añade
algo de CSS al widget de comentarios. De esta manera no tenemos que cargar el CSS del modo oscuro cuando no es necesario.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderación

Cuando el usuario actual es moderador, usamos la extensión de moderación.

Este es un buen ejemplo para añadir escuchadores de eventos basados en clic, realizar solicitudes a la API, y añadir elementos al menú de comentarios y a las áreas de comentarios.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Chat en Vivo

La extensión Chat en Vivo (en combinación con otra configuración y estilos) convierte el widget de comentarios en un componente de chat en vivo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js