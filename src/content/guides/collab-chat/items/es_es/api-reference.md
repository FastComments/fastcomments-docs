### Descripción general de la API

Collab Chat proporciona tres endpoints REST API para gestionar conversaciones de texto de forma programática. Estos endpoints le permiten recuperar, crear y eliminar anotaciones sin usar el widget del navegador.

Estos son endpoints públicos que autentican a los usuarios mediante cookies del navegador. No utilizan claves de API. Los usuarios deben haber iniciado sesión en FastComments en su navegador para acceder a estos endpoints.

### URL base

Todos los endpoints de la API de Collab Chat se encuentran en:

[inline-code-attrs-start title = 'URL base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autenticación

Estos endpoints autentican a los usuarios mediante cookies del navegador. No utilizan claves de API. Los usuarios deben haber iniciado sesión en FastComments en su navegador para acceder a estos endpoints.

### Obtener todas las conversaciones

Recupera todas las conversaciones de texto para una página específica.

#### Punto final

[inline-code-attrs-start title = 'Punto final GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parámetros

`tenantId` (parámetro de ruta, requerido) es su Tenant ID de FastComments.

`urlId` (parámetro de consulta, requerido) es el identificador de la página para la que desea recuperar las conversaciones.

#### Respuesta

La respuesta incluye el estado de la API, la información de la sesión del usuario actual si está autenticado, un array de conversaciones con sus IDs, URLs y rangos de texto, un identificador de URL limpiado, una bandera que indica si el usuario actual es administrador del sitio o moderador, y detalles de conexión WebSocket para la sincronización en vivo incluyendo `tenantIdWS`, `urlIdWS`, y `userIdWS`.

#### Ejemplo de solicitud

[inline-code-attrs-start title = 'Ejemplo de solicitud GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Ejemplo de respuesta

[inline-code-attrs-start title = 'Ejemplo de respuesta GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Crear conversación

Crea una nueva conversación de texto para una selección de texto específica.

#### Punto final

[inline-code-attrs-start title = 'Punto final POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parámetros

`tenantId` (parámetro de ruta, requerido) es su Tenant ID de FastComments.

El cuerpo de la solicitud debe ser JSON e incluir estos campos obligatorios.

`urlId` (string, requerido) es el identificador base de la página.

`urlIdWithRange` (string, requerido) es la URL combinada con el rango de texto, por ejemplo `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, requerido) es el título de la página.

`selector` (string, requerido) es la ruta DOM al elemento que contiene el texto seleccionado.

`range` (string, requerido) es el rango de texto serializado en el formato `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, requerido) es el ID del comentario que inició este chat.

`broadcastId` (string, requerido) es un UUID para la sincronización en vivo para prevenir efectos de eco.

#### Respuesta

La respuesta incluye el estado de la API y el ID de la conversación recién creada.

#### Ejemplo de solicitud

[inline-code-attrs-start title = 'Ejemplo de solicitud POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Ejemplo de respuesta

[inline-code-attrs-start title = 'Ejemplo de respuesta POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Eliminar conversación

Elimina una conversación de texto existente. Este endpoint requiere permisos de administrador o moderador, o la conversación debe haber sido creada por el usuario autenticado.

#### Punto final

[inline-code-attrs-start title = 'Punto final DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parámetros

`tenantId` (parámetro de ruta, requerido) es su Tenant ID de FastComments.

`chatId` (parámetro de ruta, requerido) es el ID de la conversación a eliminar.

`broadcastId` (cuerpo de la solicitud, requerido) es un UUID para la sincronización en vivo.

#### Ejemplo de solicitud

[inline-code-attrs-start title = 'Ejemplo de solicitud DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Ejemplo de respuesta

[inline-code-attrs-start title = 'Ejemplo de respuesta DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Limitación de tasa

Estos endpoints están sujetos a la limitación de tasa estándar de la API de FastComments. El middleware de limitación de tasa se aplica por tenant para prevenir el abuso a la vez que permite patrones de uso normales.

### Respuestas de error

Todos los endpoints devuelven códigos de estado HTTP estándar. Una respuesta 400 indica parámetros de solicitud inválidos. Una respuesta 401 significa que la autenticación falló. Una respuesta 403 indica permisos insuficientes. Una respuesta 404 significa que la conversación no se encontró. Una respuesta 429 indica que se excedió el límite de tasa.

Las respuestas de error incluyen un cuerpo JSON con detalles:

[inline-code-attrs-start title = 'Ejemplo de respuesta de error'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Seguimiento de uso

La creación de conversaciones incrementa su métrica de uso `conversationCreateCount`. Toda la actividad de sincronización por WebSocket incrementa `pubSubMessageCount` y `pubSubBandwidth`. Puede monitorizar estas métricas en su panel de FastComments bajo analytics de uso.

---