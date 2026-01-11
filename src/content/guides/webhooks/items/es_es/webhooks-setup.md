---
Sigue los mismos pasos para `localhost` que harías en producción. Asegúrate de tener configurados los dominios de producción y los secretos de API.

Primero, navega a la [administración de Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Esto es accesible vía Manage Data -> Webhooks.

La página de configuración aparece como sigue:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

En esta página puedes especificar endpoints para cada tipo de evento de comentario.

Para cada tipo de evento, asegúrate de hacer clic en Send Test Payload para verificar que has configurado correctamente tu integración. Consulta la siguiente sección, "Testing", para más detalles.

---