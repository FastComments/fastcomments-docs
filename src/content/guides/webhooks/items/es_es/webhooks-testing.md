---
En la administración de Webhooks hay botones `Send Test Payload` para cada tipo de evento (Create, Update, Delete). Los eventos Create y Update envían un objeto WebhookComment de prueba, mientras que al probar Delete se enviará un cuerpo de petición de prueba con solo un ID.

La prueba realizará dos llamadas para verificar el código de respuesta para los escenarios "happy" (clave API correcta) y "sad" (clave API inválida).

Cuando la prueba envía una clave API inválida deberías devolver un código de estado 401 para que la prueba pase por completo. Si no compruebas correctamente el valor del token, verás un error.

Esto es para garantizar que autentiques correctamente la solicitud.
---