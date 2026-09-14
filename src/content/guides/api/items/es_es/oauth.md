FastComments es un servidor de autorización OAuth 2.1. Una aplicación puede obtener un token que está vinculado a una cuenta de FastComments y usarlo en cada endpoint de esta guía en lugar de una clave API. Así es como la aplicación Zapier, el servidor MCP y otras integraciones de terceros se conectan.

Los tokens se emiten mediante el flujo de código de autorización con PKCE. No existe concesión de credenciales de cliente ni implícita.

### Descubrimiento

Las ubicaciones de los endpoints, los grants compatibles y los métodos de autenticación se publican en la URL estándar de metadatos:

[inline-code-attrs-start title = 'Metadatos del Servidor de Autorización'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Los endpoints que describe:

[inline-code-attrs-start title = 'Endpoints OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Las cuentas en la región EU usan `https://eu.fastcomments.com` como emisor, con las mismas rutas.

### Registro de un cliente

Un cliente necesita un `client_id` y un `redirect_uri` registrado antes de poder iniciar el flujo. Hay dos formas de obtenerlo:

- **Registro Dinámico de Cliente.** `POST /oauth/register` con un cuerpo JSON según RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). La respuesta incluye el `client_id` y, para clientes confidenciales, el `client_secret`. El registro no requiere autenticación y está limitado por tasa por IP.
- **Documento de Metadatos de ID de Cliente.** El cliente usa una URL `https` que controla como su `client_id`. FastComments recupera esa URL y lee los mismos campos de metadatos. No se necesita una llamada de registro.

Las aplicaciones asociadas listadas en el panel de FastComments, como Zapier, son registradas directamente por FastComments. Contacte al soporte si está creando una lista de marketplace y necesita un cliente de primera parte.

### Ámbitos

[inline-code-attrs-start title = 'Ámbitos'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Una solicitud que no pide ningún ámbito recibe ambos. El usuario ve los ámbitos solicitados en la página de consentimiento. Una solicitud de un ámbito distinto a estos dos falla con `invalid_scope`.

### Paso 1 - Solicitud de autorización

Envíe el navegador del usuario al endpoint de autorización. PKCE con el método `S256` es obligatorio para cada cliente.

[inline-code-attrs-start title = 'Solicitud de autorización'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Parámetros de la solicitud de autorización'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

El usuario inicia sesión en FastComments si es necesario y ve una página de consentimiento que nombra su aplicación, la cuenta a la que se conectará y los ámbitos solicitados. El usuario debe poseer el permiso **API Admin** en esa cuenta; cualquier otro verá un error de permiso en lugar del formulario de consentimiento. Aprobar redirige el navegador a su `redirect_uri` con `code` y `state`. Denegar redirige con `error=access_denied`.

El código de autorización es válido durante 10 minutos y puede intercambiarse una sola vez. Un segundo intercambio del mismo código revoca todos los tokens que produjo el primer intercambio.

### Paso 2 - Solicitud de token

Intercambie el código por tokens. El cuerpo está codificado como formulario. Los clientes confidenciales se autentican con `client_secret_basic` (HTTP Basic) o `client_secret_post` (secreto en el cuerpo). Los clientes públicos envían solo `client_id`.

[inline-code-attrs-start title = 'Ejemplo de solicitud de token cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Cuerpo de la solicitud de token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta del token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

Los errores siguen RFC 6749: un cuerpo JSON con `error` y `error_description`, HTTP 400 para `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` y `unsupported_grant_type`, HTTP 401 para `invalid_client`, HTTP 429 cuando se supera la tasa.

### Paso 3 - Llamada a la API

Envíe el token de acceso como token bearer. El tenant está implícito en el token, por lo que `tenantId` es opcional. Cuando se proporciona, debe coincidir con el token o la solicitud fallará.

[inline-code-attrs-start title = 'Ejemplo de token Bearer cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` devuelve el tenant, el usuario que autoriza y los ámbitos concedidos, lo que lo convierte en la llamada adecuada para una prueba de conexión. Una solicitud con un token expirado o revocado recibe HTTP 401. Una solicitud cuyo método necesita un ámbito que el token no posee recibe HTTP 403.

### Paso 4 - Actualización

[inline-code-attrs-start title = 'Ejemplo de solicitud de refresco cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Cuerpo de la solicitud de token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

La respuesta tiene la misma forma que el intercambio de código. Los tokens de refresco rotan: cada refresco devuelve un nuevo `refresh_token` y revoca el anterior después de una ventana de gracia de 30 segundos para solicitudes concurrentes. Presentar un token de refresco que se rotó hace más de 30 segundos se trata como repetición y revoca todo el grant. Las aplicaciones asociadas registradas por FastComments están exentas de rotación y reciben el mismo token de refresco con su expiración extendida otros 30 días.

Un refresco también vuelve a comprobar que el usuario que autoriza sigue teniendo el permiso API Admin en la cuenta. Si no, el grant se revoca y la respuesta es `invalid_grant`.

### Revocación

[inline-code-attrs-start title = 'Ejemplo de solicitud de revocación cURL'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Revocar un token de refresco revoca todos los tokens de acceso emitidos desde el mismo grant. Revocar un token de acceso revoca solo ese token. El endpoint devuelve HTTP 200 con un objeto JSON vacío tanto si el token se encontró como si no, según RFC 7009.

Los usuarios también pueden revocar una conexión desde **Aplicaciones Conectadas** en el panel de FastComments. Cada token para esa aplicación deja de funcionar inmediatamente.