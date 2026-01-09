### Visión general de la API

Image Chat proporciona tres puntos finales REST para gestionar conversaciones de imagen de forma programática. Estos puntos finales le permiten recuperar, crear y eliminar marcadores sin usar el widget del navegador.

Todos los puntos finales de la API requieren autenticación y siguen los patrones estándar de la API de FastComments. Son puntos finales públicos que autentican mediante cookies del navegador, no con claves de API.

### URL base

Todos los endpoints de la API Image Chat están bajo:

```
https://fastcomments.com/comment-image-chats
```

### Autenticación

Estos endpoints autentican a los usuarios mediante cookies del navegador. No usan claves de API. Los usuarios deben haber iniciado sesión en FastComments en su navegador para acceder a estos endpoints.

### Obtener todas las conversaciones

Recupera todas las conversaciones de imagen para una imagen específica.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parámetros

`tenantId` (parámetro de ruta, obligatorio) es su ID de Tenant de FastComments.

`urlId` (parámetro de consulta, obligatorio) es el identificador de la imagen para la que desea recuperar conversaciones.

#### Respuesta

La respuesta incluye el estado de la API, la información de la sesión del usuario actual si está autenticado, un array de conversaciones con sus IDs, URLs y coordenadas X/Y, un identificador de URL limpio, una bandera que indica si el usuario actual es administrador o moderador del sitio, y detalles de conexión WebSocket para sincronización en vivo incluyendo `tenantIdWS`, `urlIdWS` y `userIdWS`.

#### Ejemplo de solicitud

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Ejemplo de respuesta

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Crear conversación

Crear una nueva conversación de imagen para una ubicación específica en una imagen.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parámetros

`tenantId` (parámetro de ruta, obligatorio) es su ID de Tenant de FastComments.

El cuerpo de la solicitud debe ser JSON e incluir estos campos obligatorios.

`urlId` (string, obligatorio) es el identificador base de la página.

`windowUrlId` (string, obligatorio) es la URL combinada con la fuente de la imagen y las coordenadas, por ejemplo `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, obligatorio) es el título de la página.

`src` (string, obligatorio) es la URL de origen de la imagen.

`x` (number, obligatorio) es la coordenada X como porcentaje (0-100).

`y` (number, obligatorio) es la coordenada Y como porcentaje (0-100).

`createdFromCommentId` (string, obligatorio) es el ID del comentario que inició este chat.

`broadcastId` (string, obligatorio) es un UUID para la sincronización en vivo para prevenir efectos de eco.

#### Respuesta

La respuesta incluye el estado de la API y el ID de la conversación recién creada.

#### Ejemplo de solicitud

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Ejemplo de respuesta

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Eliminar conversación

Eliminar una conversación de imagen existente. Este endpoint requiere permisos de administrador o moderador, o que la conversación haya sido creada por el usuario autenticado.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parámetros

`tenantId` (parámetro de ruta, obligatorio) es su ID de Tenant de FastComments.

`chatId` (parámetro de ruta, obligatorio) es el ID de la conversación que se va a eliminar.

`broadcastId` (cuerpo de la solicitud, obligatorio) es un UUID para la sincronización en vivo.

#### Ejemplo de solicitud

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Ejemplo de respuesta

```json
{
  "status": "success"
}
```

### Sistema de coordenadas

Las coordenadas X y Y se almacenan como porcentajes de las dimensiones de la imagen. X representa la posición horizontal desde el borde izquierdo (0 = borde izquierdo, 100 = borde derecho). Y representa la posición vertical desde el borde superior (0 = borde superior, 100 = borde inferior).

Estos valores porcentuales pueden incluir decimales para mayor precisión. Por ejemplo, x: 25.5 significa 25.5% desde el borde izquierdo de la imagen.

### Limitación de tasa

Estos endpoints están sujetos a la limitación de tasa estándar de la API de FastComments. El middleware de limitación de tasa se aplica por tenant para prevenir abusos permitiendo al mismo tiempo patrones de uso normales.

### Respuestas de error

Todos los endpoints devuelven códigos de estado HTTP estándar. Una respuesta 400 indica parámetros de solicitud inválidos. Una respuesta 401 significa que la autenticación falló. Una respuesta 403 indica permisos insuficientes. Una respuesta 404 significa que la conversación no se encontró. Una respuesta 429 indica que se excedió el límite de tasa.

Las respuestas de error incluyen un cuerpo JSON con detalles:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Seguimiento de uso

La creación de conversaciones incrementa su métrica de uso `conversationCreateCount`. Toda la actividad de sincronización por WebSocket incrementa `pubSubMessageCount` y `pubSubBandwidth`. Puede supervisar estas métricas en el panel de control de FastComments, en la sección de análisis de uso.