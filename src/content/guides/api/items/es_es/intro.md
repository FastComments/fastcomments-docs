### La API de FastComments

FastComments proporciona una API para interactuar con muchos recursos. Crea integraciones con nuestra plataforma, ¡o incluso desarrolla tus propios clientes!

En esta documentación encontrarás todos los recursos compatibles por la API documentados con sus tipos de solicitud y respuesta.

Para clientes Enterprise, todo el acceso a la API queda registrado en el Registro de Auditoría.

### SDKs Generados

FastComments ahora genera una [Especificación de la API](https://fastcomments.com/js/swagger.json) a partir de nuestro código (esto aún no está completo, pero incluye muchas APIs).

También ahora tenemos SDKs para lenguajes populares:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Autenticación

La API se autentica pasando tu [clave API](https://fastcomments.com/auth/my-account/api-secret) ya sea como un encabezado `X-API-KEY` o como un parámetro de consulta `API_KEY`. También necesitarás tu `tenantId` para realizar llamadas a la API. Esto puede obtenerse desde la misma página que tu clave API.

### Nota de seguridad

Estas rutas están pensadas para ser llamadas desde un **servidor**. __NO__ las llames desde un navegador. Hacerlo expondrá tu clave API - esto proporcionará acceso total a tu cuenta a cualquiera que pueda ver el código fuente de una página!

#### Opción de autenticación uno - Encabezados

- Encabezado: `X-API-KEY`
- Encabezado: `X-TENANT-ID`

#### Opción de autenticación dos - Parámetros de consulta

- Parámetro de consulta: `API_KEY`
- Parámetro de consulta: `tenantId`

---