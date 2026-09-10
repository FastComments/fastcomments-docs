---
Siga los mismos pasos para `localhost` que haría en producción. Asegúrese de que tiene configurados los dominios de producción y los secretos de API.

Primero, navegue a la [administración de Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Esto es accesible a través de Administrar datos -> Webhooks.

La página muestra cada webhook en su cuenta:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Página de administración de Webhooks que muestra cada webhook con su URL, evento, dominio, método, estado y recuento de eventos en cola'; title='Lista de Webhooks'; cacheBuster = 'v4' app-screenshot-end]

Haga clic en **Nuevo Webhook** para agregar uno. Cada webhook tiene una URL, un evento de comentario (creado, actualizado o eliminado), un dominio y un método HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Formulario de nuevo webhook con campos de URL, evento, dominio y método HTTP más Enviar carga de prueba'; title='Nuevo Webhook'; cacheBuster = 'v4' app-screenshot-end]

Cada webhook se entrega de forma independiente. Puede enviar el mismo evento a varios puntos finales, y un webhook con alcance a **Todos los dominios** recibe comentarios de todos los dominios incluso cuando existe un webhook específico de dominio para el mismo evento. No se puede agregar la misma URL, evento y dominio dos veces.

Antes de guardar, haga clic en **Enviar carga de prueba** para verificar que el punto final acepta una solicitud firmada. Consulte la siguiente sección, "Pruebas", para obtener más detalles.

Desde la lista puede editar, desactivar, volver a activar o eliminar un webhook. Desactivar mantiene los eventos en cola hasta que el webhook se vuelva a activar; eliminar los descarta.

Los webhooks también pueden crearse a través de la API, por ejemplo mediante Zapier. Estos aparecen en la misma lista con la fuente **API**. Consulte la gestión de webhooks a través de la API.

---