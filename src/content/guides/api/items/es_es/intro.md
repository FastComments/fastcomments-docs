### La API de FastComments

FastComments ofrece una API para interactuar con muchos recursos. ¡Crea integraciones con nuestra plataforma, o incluso crea tus propios clientes!

En esta documentación, encontrarás todos los recursos compatibles con la API documentados con sus tipos de solicitud y respuesta.

Para clientes Enterprise, todo el acceso a la API se registra en el Registro de Auditoría.

### SDKs generados

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

La API se autentica pasando su [api key](https://fastcomments.com/auth/my-account/api-secret) como encabezado `X-API-KEY` o como parámetro de consulta `API_KEY`. También necesitará su `tenantId` para realizar llamadas a la API. Esto se puede obtener en la misma página que su clave API.

### Nota de seguridad

Estas rutas están diseñadas para ser llamadas desde un **servidor**. __NO__ las llame desde un navegador. Hacerlo expondrá su clave API, lo que otorgará acceso total a su cuenta a cualquiera que pueda ver el código fuente de una página!

#### Opción de autenticación uno - Encabezados

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opción de autenticación dos - Parámetros de consulta

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opción de autenticación tres - Token Bearer OAuth

- Header: `Authorization: Bearer fcat_...`

Aplicaciones de terceros como Zapier y clientes del [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) obtienen un token mediante OAuth en lugar de una clave API. Ese token funciona en todos los puntos finales aquí. El inquilino (tenant) está implícito en el token, por lo que `tenantId` es opcional, pero debe coincidir con el token cuando se proporciona. Las solicitudes `GET` necesitan el alcance `read` y cualquier otro método necesita el alcance `write`. El flujo completo, incluida la registro de cliente, PKCE, refresco y revocación, está documentado bajo [OAuth Authorization](#oauth). El descubrimiento comienza en `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lectura de sus propias escrituras

FastComments ofrece disponibilidad Active-Active. Las solicitudes desde su centro de datos se enrutan al [punto de presencia más cercano](https://sophon.fastcomments.com/) al suyo. Esto es automático, y normalmente puede observar la semántica de lectura después de escritura. Si desea asegurarse de leer sus propias escrituras, puede fijar sus solicitudes a una región específica usando esa región como host de la API (aunque normalmente no es necesario para la mayoría de integraciones):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Tenga en cuenta que si hace esto, puede que desee definir una alternativa, ya que hemos desaprobado nodos de punto de entrada en el pasado y usamos nuevos nombres para el cambio.