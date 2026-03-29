### The FastComments API

FastComments proporciona una API para interactuar con muchos recursos. Cree integraciones con nuestra plataforma, o incluso cree sus propios clientes!

En esta documentación, encontrará todos los recursos compatibles por la API documentados con sus tipos de solicitud y respuesta.

Para clientes Enterprise, todo el acceso a la API se registra en el Registro de Auditoría.

### Generated SDKs

FastComments ahora genera una [Especificación de la API](https://fastcomments.com/js/swagger.json) a partir de nuestro código (aún no está completa, pero incluye muchas API).

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

### Authentication

La API se autentica pasando su [clave de la API](https://fastcomments.com/auth/my-account/api-secret) ya sea como un encabezado `X-API-KEY` o como un parámetro de consulta `API_KEY`. También necesitará su `tenantId` para realizar llamadas a la API. Esto se puede obtener en la misma página que su clave de la API.

### Security Note

Estas rutas están pensadas para ser llamadas desde un **servidor**. __DO NOT__ llamarlas desde un navegador. Hacer esto expondrá su clave de la API: ¡esto proporcionará acceso completo a su cuenta a cualquiera que pueda ver el código fuente de una página!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Reading Your Own Writes

FastComments ofrece disponibilidad activa-activa. Las solicitudes desde su centro de datos se enrutan al [punto de presencia más cercano](https://sophon.fastcomments.com/) al suyo. Esto es automático, y normalmente podrá observar la semántica de leer tras escribir. Si desea asegurarse de leer sus propias escrituras, puede fijar sus solicitudes a una región determinada usando esa región como host de la API (sin embargo, esto normalmente no es necesario para la mayoría de las integraciones):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Tenga en cuenta que si hace esto quizá desee definir una alternativa, ya que en el pasado hemos dejado obsoletos los nodos de entrada y hemos utilizado nuevos nombres para el cambio.

---