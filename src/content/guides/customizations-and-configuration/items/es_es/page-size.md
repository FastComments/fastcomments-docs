Por defecto, el tamaño de página de FastComments es `30`. Esto incluye respuestas en los hilos.

El tamaño de página se puede personalizar en la [Widget Configuration UI](https://fastcomments.com/auth/my-account/customize-widget) en tamaños variables que van desde `10` hasta `200`.

Tenga en cuenta que cambiar el tamaño de página requiere recalcular todos los hilos de comentarios en su cuenta. Esto puede tardar un par de minutos.

Esto no se puede configurar en el widget del lado del cliente ya que las páginas se calculan del lado del servidor.

A continuación se muestra un ejemplo de configuración:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; alt='Selector de tamaño de página en la página de personalización del widget, donde se puede elegir un valor de 10 a 200'; title='Tamaños de página personalizados' app-screenshot-end]

Los tamaños de página pueden personalizarse globalmente, o por dominio, o por página, creando diferentes reglas de personalización.

Esto afectará a todos los clientes, integraciones y frameworks que pueda estar utilizando para mostrar comentarios a través de nuestra plataforma.