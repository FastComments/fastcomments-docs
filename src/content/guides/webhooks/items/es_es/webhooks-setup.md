---
Siga los mismos pasos para `localhost` que seguiría para producción. Asegúrese de tener configurados los dominios de producción y los API Secrets.

Primero, diríjase a la [Administración de Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Esto es accesible a través de Administrar datos -> Webhooks.

La página de configuración aparece como sigue:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

En esta página puede especificar endpoints para cada tipo de evento de comentario.

Para cada tipo de evento, asegúrese de hacer clic en Send Test Payload para garantizar que ha configurado su integración correctamente. Consulte la siguiente sección, "Pruebas", para más detalles.

---