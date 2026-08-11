---
Siga los mismos pasos para `localhost` como lo haría en producción. Asegúrese de que tiene dominios de producción y secretos de API configurados.

Primero, navegue a la [administración de Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Esto es accesible a través de Administrar datos -> Webhooks.

La página de configuración aparece de la siguiente manera:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Página de administración de Webhooks con un selector de dominio y un campo de URL de endpoint por evento de comentario, más Enviar carga de prueba'; title='Configuración de Webhooks'; cacheBuster = 'v3' app-screenshot-end]

En esta página puede especificar endpoints para cada tipo de evento de comentario.

Para cada tipo de evento, asegúrese de hacer clic en Enviar carga de prueba para garantizar que ha configurado su integración correctamente. Consulte la siguiente sección, "Testing", para obtener más detalles.

---