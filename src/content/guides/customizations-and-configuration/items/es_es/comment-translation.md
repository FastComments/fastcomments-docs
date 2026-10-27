---
Por defecto, FastComments traduce la propia interfaz del widget (botones, etiquetas, mensajes de error) al idioma del visitante, pero los comentarios se muestran exactamente como fueron escritos.

La Traducción de Comentarios agrega la traducción de los propios comentarios, de modo que una comunidad que escribe en más de un idioma pueda leer cada comentario en su propio idioma.

Hay dos modos:

- **Automático**: los comentarios escritos en un idioma distinto al del widget se muestran traducidos. Aparece una línea "Traducido del español" bajo el comentario con un enlace **Mostrar original** que alterna entre la traducción y el texto sin traducir.
- **Manual**: cada comentario obtiene un enlace **Traducir**. Al hacer clic, se traduce ese comentario al idioma del visitante, con el mismo interruptor **Mostrar original** después. Los comentarios que ya están en el idioma del visitante lo indican en su lugar.

El idioma de destino es el idioma del widget: la configuración [Configuración de idioma / región](#locale) en la misma regla de personalización si está establecida, de lo contrario el idioma del navegador del visitante. No se requiere nada del visitante, ni cuenta, extensión o software adicional, y funciona igual en el widget incrustado y en páginas de comentarios independientes.

Esto se activa por <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regla de personalización</a>, por lo que puede aplicarse a todo su sitio, a un dominio o a una sola página. Está desactivado por defecto.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Desplegable de Traducción de Comentarios en la página de personalización del widget con opciones Desactivado, Automático y Manual'; title='Traducción de Comentarios' app-screenshot-end]

La traducción es una configuración del lado del servidor. Pasar `commentTranslation` en la configuración JavaScript del widget no lo habilita; la regla sí lo hace.

Cada comentario se traduce como máximo una vez por idioma de destino y el resultado se almacena con el comentario, de modo que un comentario leído por mil hablantes de español se traduce una sola vez. Si el autor o un moderador edita el comentario, la traducción almacenada se descarta y el comentario se vuelve a traducir en la siguiente visualización. El texto traducido pasa por la misma sanitización y reglas de enlaces que el comentario original. Los comentarios que son demasiado largos, o cuyo idioma no puede detectarse, se muestran sin traducir.

El uso se factura como tokens LLM al mismo precio que los [Agentes de IA](/guide-ai-agents.html#plans-and-eligibility) y aparece en el mismo lugar en los detalles de facturación.
---