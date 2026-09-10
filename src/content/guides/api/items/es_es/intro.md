### La API de FastComments

FastComments ofrece una API para interactuar con muchos recursos. ¡Crea integraciones con nuestra plataforma, o incluso crea tus propios clientes!

En esta documentación, encontrarás todos los recursos compatibles con la API documentados con sus tipos de solicitud y respuesta.

Para clientes Enterprise, todo el acceso a la API se registra en el Registro de Auditoría.

### SDKs Generados

FastComments ahora genera una [API Spec](https://fastcomments.com/js/swagger.json) a partir de nuestro código (esto aún no está completo, pero incluye muchas APIs).

También ahora disponemos de SDKs para lenguajes populares:

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

La API se autentica pasando su [api key](https://fastcomments.com/auth/my-account/api-secret) ya sea como encabezado `X-API-KEY` o como parámetro de consulta `API_KEY`. También necesitará su `tenantId` para realizar llamadas a la API. Esto se puede obtener en la misma página que su clave API.

### Nota de Seguridad

Estas rutas están diseñadas para ser llamadas desde un **servidor**. __NO__ las llame desde un navegador. Hacerlo expondrá su clave API, lo que otorgará acceso completo a su cuenta a cualquiera que pueda ver el código fuente de una página!

#### Opción de Autenticación Uno - Encabezados

- Encabezado: `X-API-KEY`
- Encabezado: `X-TENANT-ID`

#### Opción de Autenticación Dos - Parámetros de Consulta

- Parámetro de Consulta: `API_KEY`
- Parámetro de Consulta: `tenantId`

#### Opción de Autenticación Tres - Token Portador OAuth

- Encabezado: `Authorization: Bearer fcat_...`

Las aplicaciones que se conectan a través del [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) obtienen un token mediante OAuth en lugar de una clave API. Ese token funciona en todos los puntos finales aquí. El inquilino está implícito en el token, por lo que `tenantId` es opcional, pero debe coincidir con el token cuando se proporciona. Las solicitudes `GET` necesitan el alcance `read` y cualquier otro método necesita el alcance `write`. El descubrimiento comienza en `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lectura de sus propias escrituras

FastComments ofrece disponibilidad Active-Active. Las solicitudes desde su centro de datos se enrutan al [punto de presencia más cercano](https://sophon.fastcomments.com/) al suyo. Esto es automático, y normalmente puede observar la semántica de lectura después de escritura. Si desea asegurarse de leer sus propias escrituras, puede fijar sus solicitudes a una región específica usando esa región como host de la API (aunque normalmente no es necesario para la mayoría de las integraciones):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Tenga en cuenta que si hace esto, es posible que desee definir una alternativa, ya que hemos desaprobado nodos de punto de entrada en el pasado y usamos nuevos nombres para el cambio.