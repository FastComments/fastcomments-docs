---
Al moderar y ver hilos de comentarios, es deseable poder saltar directamente a un hilo para obtener contexto durante la moderación.

Esto significa que el flujo del usuario comienza en la página de Moderación de Comentarios, y luego tendría que ir de un comentario individual a
la página que contiene ese comentario, esperar a que esa página se cargue, esperar a que se carguen los comentarios y luego desplazarse hasta ese comentario.

Sin embargo, FastComments ofrece una forma más rápida. En la página de Moderación de Comentarios, junto a cada comentario, hay un botón "Ver comentario" en la esquina inferior derecha.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Si este comentario tiene respuestas, el texto del botón mostrará en su lugar el número de respuestas, pero al hacer clic en él realiza la misma acción.

Este botón te llevará al **Visor de Hilos de Comentarios**.

El Visor de Hilos de Comentarios es una aplicación pequeña y de carga rápida alojada por FastComments que renderiza el hilo de comentarios de la página en la que
está el comentario, y se desplaza hasta ese comentario.

Esto permite a los moderadores reunir rápidamente el contexto que necesitan, sin tener que esperar a que se cargue otra página.
---